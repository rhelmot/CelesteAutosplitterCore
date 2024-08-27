mod game_types;

use {
    crate::game_types::{Area, AreaMode, Settings}, asr::{print_limited, settings::Gui, string::ArrayCString, time::Duration, timer::{pause_game_time, reset, set_game_time, set_variable, split, start}, Error, Process}, bytemuck::Pod, static_locks::{MappedMutexGuard, Mutex, MutexGuard}
};

static STATE: Mutex<Option<Celeste>> = Mutex::new(None);

struct Celeste {
    process: Process,
    settings: Settings,
    asi_base: u64,
    last_completed: bool,
    exiting_chapter: bool,
    last_level: String,
}

impl Celeste {
    fn read<T: Pod>(&self, address: u64) -> T {
        self.process.read(address).unwrap()
    }

    fn chapter_completed(&self) -> bool {
        self.read::<u8>(self.asi_base + 0x12) != 0
    }

    fn level_name(&self) -> String {
        let level_ptr = self.read::<u64>(self.asi_base);
        if level_ptr == 0 {
            return "".to_owned();
        }
        let size = self.read::<u32>(level_ptr + 0x10);
        let mut buffer = vec![0u16; size as usize];
        self.process.read_into_slice(level_ptr + 0x14, &mut buffer).unwrap();
        return String::from_utf16(&buffer).unwrap();
    }

    fn area_id_fallible(&self) -> Result<i32, Error> {
        self.process.read::<i32>(self.asi_base + 0x8)
    }

    fn area_id(&self) -> i32 {
        self.area_id_fallible().unwrap()
    }

    fn area_difficulty(&self) -> i32 {
        self.read(self.asi_base + 0xc)
    }

    fn chapter_started(&self) -> bool {
        self.read::<u8>(self.asi_base + 0x11) != 0
    }

    fn game_time(&self) -> Duration {
        Duration::milliseconds(self.read::<i64>(self.asi_base + 0x28) / 10000)
    }

    fn level_time(&self) -> Duration {
        Duration::milliseconds(self.read::<i64>(self.asi_base + 0x18) / 10000)
    }

    fn file_strawberries(&self) -> i32 {
        self.read(self.asi_base + 0x30)
    }

    fn chapter_strawberries(&self) -> i32 {
        self.read(self.asi_base + 0x20)
    }

    fn file_cassettes(&self) -> i32 {
        self.read(self.asi_base + 0x34)
    }

    fn file_hearts(&self) -> i32 {
        self.read(self.asi_base + 0x38)
    }

    fn chapter_cassette(&self) -> bool {
        self.read::<u8>(self.asi_base + 0x24) != 0
    }

    fn chapter_heart(&self) -> bool {
        self.read::<u8>(self.asi_base + 0x25) != 0
    }

    fn chapter_split(
        &mut self,
        area_id: i32,
        chapter_area: Area,
        level: &str,
        completed: bool,
        il_splits: bool,
    ) -> bool {
        if !self.exiting_chapter {
            let not_in_credits = if chapter_area == Area::TheSummit {
                !level.starts_with("credits") // TODO: May need to be case insensitive
            } else {
                true
            };
            self.exiting_chapter = area_id == chapter_area as i32
                && completed
                && !self.last_completed
                && not_in_credits;
            self.exiting_chapter && il_splits
        } else {
            !completed && self.last_completed
        }
    }
}

fn class_field_offset(process: &Process, klass: u64, name: &str) -> u64 {
    let class_kind = process.read::<u8>(klass + 0x24).unwrap() & 7;
    if class_kind == 3 {
        return class_field_offset(process, process.read::<u64>(process.read::<u64>(klass + 0xe0).unwrap()).unwrap(), name);
    }
    if class_kind != 1 && class_kind != 2 {
        panic!();
    }

    let num_fields = process.read::<i32>(klass + 0xf0).unwrap();
    let fields_ptr = process.read::<u64>(klass + 0x90).unwrap();

    let mut fields_buf = vec![0u64; num_fields as usize * 4];
    process.read_into_slice(fields_ptr, &mut fields_buf).unwrap();

    for arr in fields_buf.chunks(4) {
        let field_name_ptr = arr[1];
        let field_offset = arr[3] & 0xffff_ffff;
        if process.read::<ArrayCString::<256>>(field_name_ptr).unwrap().matches(name) {
            return field_offset;
        }
    }
    unreachable!();
}


fn lookup_class(process: &Process, class_cache: u64, name: &str) -> u64 {
    let celeste_class_cache_table = process.read::<u32>(class_cache + 0x20).unwrap();
    let hash_table_size = process.read::<u32>(class_cache + 0x18).unwrap();
    for bucket in 0..hash_table_size {
        let mut klass = process.read::<u64>(celeste_class_cache_table + 8*bucket).unwrap();
        while klass != 0 {
            let current_name_ptr = process.read::<u64>(klass + 0x40).unwrap();
            let name_arr = process.read::<ArrayCString<128>>(current_name_ptr).unwrap();
            if name_arr.matches(name) {
                return klass;
            }
            klass = process.read::<u64>(klass + 0xf8).unwrap();
        }
    }
    unreachable!();
}

fn class_static_fields(process: &Process, klass: u64) -> u64 {
    let runtime_info = process.read::<u64>(klass + 0xc8).unwrap();
    let celeste_vtable = process.read::<u64>(runtime_info + 8).unwrap();
    let vtable_size = process.read::<u32>(klass + 0x54).unwrap() as u64;
    process.read(celeste_vtable + 64 + vtable_size * 8).unwrap()
}

fn instance_class(process: &Process, instance: u64) -> u64 {
    process.read(process.read::<u64>(instance).unwrap() & 0xffff_ffff_ffff_fffe).unwrap()
}

fn field<T: Pod>(process: &Process, instance: u64, name: &str) -> T {
    let klass = instance_class(process, instance);
    let offset = class_field_offset(process, klass, name);
    process.read(offset + instance).unwrap()
}

fn static_field<T: Pod>(process: &Process, klass: u64, name: &str) -> T {
    let offset = class_field_offset(process, klass, name);
    let static_ptr = class_static_fields(process, klass);
    process.read(offset + static_ptr).unwrap()
}

fn find_base() -> Option<Celeste> {
    let process = Process::attach("Celeste.bin.x86_64")?;
    //let mono_addr = process.get_module_address("Celeste.bin.x86_64").ok()?;
    //let mono_root_domain = mono_addr + 0xA17650;
    let domains_list = process.read::<u64>(0xA17698u64).ok()?;

    let first_domain = process.read::<u64>(domains_list).ok()?;
    let second_domain = process.read::<u64>(domains_list + 8).ok()?;
    let first_domain_name = if first_domain == 0 { None } else { Some(process.read::<ArrayCString<128>>(process.read::<u64>(first_domain + 0xd8).ok()?).ok()?) };
    let second_domain_name = if second_domain == 0 { None } else { Some(process.read::<ArrayCString<128>>(process.read::<u64>(second_domain + 0xd8).ok()?).ok()?) };

    if first_domain_name.map(|s| s.matches("Celeste.exe")) != Some(true) {
        return None;
    }

    let celeste_domain = if second_domain != 0 {
        print_limited::<128>(&format_args!("Connected to {} (2)", second_domain_name.unwrap().validate_utf8().unwrap()));
        second_domain
    } else {
        print_limited::<128>(&format_args!("Connected to {} (1)", first_domain_name.unwrap().validate_utf8().unwrap()));
        first_domain
    };

    let celeste_assembly = process.read::<u64>(celeste_domain + 0xd0).unwrap();
    let celeste_image = process.read::<u64>(celeste_assembly + 0x60).unwrap();
    let class_cache = celeste_image + 1216;

    let celeste_class = lookup_class(&process, class_cache, "Celeste");
    let celeste_obj = static_field::<u64>(&process, celeste_class, "Instance");
    let autosplitter_obj = field::<u64>(&process, celeste_obj, "AutoSplitterInfo");

    let settings = Settings::register();
    pause_game_time();
    return Some(Celeste {
        process,
        settings,
        asi_base: autosplitter_obj + 0x10,
        last_completed: false,
        exiting_chapter: false,
        last_level: "".to_owned(),
    });
}

fn state() -> Option<MappedMutexGuard<'static, Celeste>> {
    let mut state = STATE.lock();
    if state.is_none() {
        *state = find_base();
    }
    MutexGuard::try_map(state, |x| x.as_mut()).ok()
}

#[no_mangle]
pub extern "C" fn update() {
    if update_inner() {
        *STATE.lock() = None;
    }
}

fn update_inner() -> bool {
    if let Some(mut state) = state() {
        let Ok(area_id) = state.area_id_fallible() else {
            return true;
        };
        state.settings.update();
        let time = game_time(&mut state);
        set_game_time(time);
        if time <= Duration::milliseconds(17) && area_id != -1 {
            reset();
            start();
        }
        if should_split(&mut state) {
            split()
        }
    }
    false
}

fn should_split(state: &mut Celeste) -> bool {
    let completed = state.chapter_completed();
    let area_id = state.area_id();
    // TODO: addAmount
    let level_name = state.level_name();
    set_variable("Level", &level_name);
    let level_name = if level_name == state.last_level {
        "".to_owned()
    } else {
        state.last_level = level_name.clone();
        level_name
    };

    let mut should_split = false;
    let lt = state.settings.level_time;

    should_split |= state.settings.chapter && state.chapter_split(
            Area::Prologue as i32,
            Area::Prologue,
            &level_name,
            completed,
            lt,
        );
    should_split |= state.settings.prologue && state.chapter_split(area_id, Area::Prologue, &level_name, completed, lt);
    should_split |= state.settings.chapter1 && state.chapter_split(area_id, Area::ForsakenCity, &level_name, completed, lt);
    should_split |= state.settings.chapter2 && state.chapter_split(area_id, Area::OldSite, &level_name, completed, lt);
    should_split |= state.settings.chapter3 && state.chapter_split(area_id, Area::CelestialResort, &level_name, completed, lt);
    should_split |= state.settings.chapter4 && state.chapter_split(area_id, Area::GoldenRidge, &level_name, completed, lt);
    should_split |= state.settings.chapter5 && state.chapter_split(area_id, Area::MirrorTemple, &level_name, completed, lt);
    should_split |= state.settings.chapter6 && state.chapter_split(area_id, Area::Reflection, &level_name, completed, lt);
    should_split |= state.settings.chapter7 && state.chapter_split(area_id, Area::TheSummit, &level_name, completed, lt);
    should_split |= state.settings.epilogue && state.chapter_split(area_id, Area::Epilogue, &level_name, completed, lt);
    should_split |= state.settings.chapter8 && state.chapter_split(area_id, Area::Core, &level_name, completed, lt);
    should_split |= state.settings.chapter1_checkpoint1 && area_id == Area::ForsakenCity as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"6"[..]
                    } else {
                        &"04"[..]
                    };
    should_split |= state.settings.chapter1_checkpoint2 && area_id == Area::ForsakenCity as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"9b"[..]
                    } else {
                        &"08"[..]
                    };
    should_split |= state.settings.chapter2_checkpoint1 && area_id == Area::OldSite as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"3"[..]
                    } else {
                        &"03"[..]
                    };
    should_split |= state.settings.chapter2_checkpoint2 && area_id == Area::OldSite as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"end_3"[..]
                    } else {
                        &"08b"[..]
                    };
    should_split |= state.settings.chapter3_checkpoint1 && area_id == Area::CelestialResort as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"08-a"[..]
                    } else {
                        &"06"[..]
                    };
    should_split |= state.settings.chapter3_checkpoint2 && area_id == Area::CelestialResort as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"09-d"[..]
                    } else {
                        &"11"[..]
                    };
    should_split |= state.settings.chapter3_checkpoint3 && area_id == Area::CelestialResort as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"00-d"[..]
                    } else {
                        &"16"[..]
                    };
    should_split |= state.settings.chapter4_checkpoint1 && area_id == Area::GoldenRidge as i32 && level_name == "b-00";
    should_split |= state.settings.chapter4_checkpoint2 && area_id == Area::GoldenRidge as i32 && level_name == "c-00";
    should_split |= state.settings.chapter4_checkpoint3 && area_id == Area::GoldenRidge as i32 && level_name == "d-00";
    should_split |= state.settings.chapter5_checkpoint1 && area_id == Area::MirrorTemple as i32 && level_name == "b-00";
    should_split |= state.settings.chapter5_checkpoint2 && area_id == Area::MirrorTemple as i32 && level_name == "c-00";
    should_split |= state.settings.chapter5_checkpoint3 && area_id == Area::MirrorTemple as i32 && level_name == "d-00";
    should_split |= state.settings.chapter5_checkpoint4 && area_id == Area::MirrorTemple as i32 && level_name == "e-00";
    should_split |= state.settings.chapter6_checkpoint1 && area_id == Area::Reflection as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"00"[..]
                    } else {
                        &"b-00"[..]
                    };
    should_split |= state.settings.chapter6_checkpoint2 && area_id == Area::Reflection as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"04"[..]
                    } else {
                        &"c-00"[..]
                    };
    should_split |= state.settings.chapter6_checkpoint3 && area_id == Area::Reflection as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"b-00"[..]
                    } else {
                        &"d-00"[..]
                    };
    should_split |= state.settings.chapter6_checkpoint4 && area_id == Area::Reflection as i32 && level_name == "boss-00";
    should_split |= state.settings.chapter6_checkpoint5 && area_id == Area::Reflection as i32 && level_name == "after-00";
    should_split |= state.settings.chapter7_checkpoint1 && area_id == Area::TheSummit as i32 && level_name == "b-00";
    should_split |= state.settings.chapter7_checkpoint2 && area_id == Area::TheSummit as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"c-00"[..]
                    } else {
                        &"c-01"[..]
                    };
    should_split |= state.settings.chapter7_checkpoint3 && area_id == Area::TheSummit as i32 && level_name == "d-00";
    should_split |= state.settings.chapter7_checkpoint4 && area_id == Area::TheSummit as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"e-00b"[..]
                    } else {
                        &"e-00"[..]
                    };
    should_split |= state.settings.chapter7_checkpoint5 && area_id == Area::TheSummit as i32 && level_name == "f-00";
    should_split |= state.settings.chapter7_checkpoint6 && area_id == Area::TheSummit as i32 && level_name == "g-00";
    should_split |= state.settings.chapter8_checkpoint1 && area_id == Area::Core as i32 && level_name == "a-00";
    should_split |= state.settings.chapter8_checkpoint2 && area_id == Area::Core as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"c-00"[..]
                    } else {
                        &"b-00"[..]
                    };
    should_split |= state.settings.chapter8_checkpoint3 && area_id == Area::Core as i32
                && level_name
                    == if state.area_difficulty() == AreaMode::ASide as i32 {
                        &"d-00"[..]
                    } else {
                        &"c-01"[..]
                    };

    state.last_completed = completed;

    // state.last_elapsed = elapsed;

    if should_split {
        state.exiting_chapter = false;
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn is_loading() -> bool {
    true
}

fn game_time(state: &mut Celeste) -> Duration {
    set_variable("Strawberries", &state.file_strawberries().to_string());
    set_variable("Level Timer", &format!("{:.2}s", state.level_time()));
    let elapsed = if state.settings.level_time {
        state.level_time()
    } else {
        state.game_time()
    };
    elapsed
}
