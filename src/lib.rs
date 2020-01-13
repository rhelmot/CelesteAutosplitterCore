#[macro_use]
extern crate wstr;

mod game_types;

use {
    crate::game_types::{Area, AreaMode, Menu, SplitType},
    asl::{ASLState, Address},
    plain::Plain,
    static_locks::{MappedMutexGuard, Mutex, MutexGuard},
    std::sync::atomic::{AtomicUsize, Ordering},
};

#[derive(ASLState)]
#[Process = "Celeste.exe"]
struct EmptyState {}

static STATE: Mutex<Option<Celeste>> = Mutex::new(None);
static CURRENT_SPLIT: AtomicUsize = AtomicUsize::new(0);

const DEFAULT_SPLITS: &[SplitType] = &[
    SplitType::Prologue,
    SplitType::Chapter1Checkpoint1,
    SplitType::Chapter1Checkpoint2,
    SplitType::Chapter1,
    SplitType::Chapter2Checkpoint1,
    SplitType::Chapter2Checkpoint2,
    SplitType::Chapter2,
    SplitType::Chapter3Checkpoint1,
    SplitType::Chapter3Checkpoint2,
    SplitType::Chapter3Checkpoint3,
    SplitType::Chapter3,
    SplitType::Chapter4Checkpoint1,
    SplitType::Chapter4Checkpoint2,
    SplitType::Chapter4Checkpoint3,
    SplitType::Chapter4,
    SplitType::Chapter5Checkpoint1,
    SplitType::Chapter5Checkpoint2,
    SplitType::Chapter5Checkpoint3,
    SplitType::Chapter5Checkpoint4,
    SplitType::Chapter5,
    SplitType::Chapter6Checkpoint1,
    SplitType::Chapter6Checkpoint2,
    SplitType::Chapter6Checkpoint3,
    SplitType::Chapter6Checkpoint4,
    SplitType::Chapter6Checkpoint5,
    SplitType::Chapter6,
    SplitType::Chapter7Checkpoint1,
    SplitType::Chapter7Checkpoint2,
    SplitType::Chapter7Checkpoint3,
    SplitType::Chapter7Checkpoint4,
    SplitType::Chapter7Checkpoint5,
    SplitType::Chapter7Checkpoint6,
    SplitType::Chapter7,
];

#[derive(Copy, Clone, PartialEq)]
enum PointerVersion {
    Xna,
    OpenGl,
    Itch,
}

struct Celeste {
    version: PointerVersion,
    base: u32,
    last_show_input_ui: bool,
    last_completed: bool,
    il_splits: bool,
    exiting_chapter: bool,
}

impl Celeste {
    fn resolve_offsets(&self, offsets: &[i32]) -> Address {
        let mut address = self.base;
        let mut offsets = offsets.iter().cloned().peekable();
        while let Some(offset) = offsets.next() {
            address = (address as u32).wrapping_add(offset as u32);
            if offsets.peek().is_some() {
                address = unsafe { asl::read_val(Address(address as u64)) };
            }
        }
        Address(address as u64)
    }

    fn read<T: Plain + Sized>(&self, offsets: &[i32]) -> T {
        let address = self.resolve_offsets(offsets);
        unsafe { asl::read_val(address) }
    }

    fn chapter_completed(&self) -> bool {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0x32]
        } else {
            &[0x0, 0x8c, 0x32]
        };
        self.read::<u8>(offsets) != 0
    }

    fn level_name(&self) -> Vec<u16> {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0x14, 0x0]
        } else {
            &[0x0, 0x8c, 0x14, 0x0]
        };
        // TODO: Maybe it's null? Also maybe one more deref.
        let Address(address) = self.resolve_offsets(offsets);
        let len_ucs2 = unsafe { asl::read_val::<i32>(Address(address + 0x4)) };
        if len_ucs2 < 0 || len_ucs2 > 2048 {
            return Vec::new();
        }
        let mut buf = Vec::with_capacity(len_ucs2 as usize);
        unsafe {
            buf.set_len(len_ucs2 as usize);
        }
        let (_, u8_view, _) = unsafe { buf.align_to_mut() };
        asl::read_into_buf(Address(address + 0x8), u8_view);
        buf
    }

    fn area_id(&self) -> i32 {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0x18]
        } else {
            &[0x0, 0x8c, 0x18]
        };
        self.read(offsets)
    }

    fn area_difficulty(&self) -> i32 {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0x1c]
        } else {
            &[0x0, 0x8c, 0x1c]
        };
        self.read(offsets)
    }

    fn chapter_started(&self) -> bool {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0x31]
        } else {
            &[0x0, 0x8c, 0x31]
        };
        self.read::<u8>(offsets) != 0
    }

    fn game_time(&self) -> f64 {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0xc]
        } else {
            &[0x0, 0x8c, 0xc]
        };
        self.read::<i64>(offsets) as f64 / 10000000.0
    }

    fn level_time(&self) -> f64 {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0x4]
        } else {
            &[0x0, 0x8c, 0x4]
        };
        self.read::<i64>(offsets) as f64 / 10000000.0
    }

    fn strawberries(&self) -> i32 {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0x24]
        } else {
            &[0x0, 0x8c, 0x24]
        };
        self.read(offsets)
    }

    fn cassettes(&self) -> i32 {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0x28]
        } else {
            &[0x0, 0x8c, 0x28]
        };
        self.read(offsets)
    }

    fn heart_gems(&self) -> i32 {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0xac, 0x2c]
        } else {
            &[0x0, 0x8c, 0x2c]
        };
        self.read(offsets)
    }

    fn type_size(&self) -> i32 {
        let offsets = if self.version == PointerVersion::Xna {
            &[0x0, 0x98, 0x0, 0x4]
        } else {
            &[0x0, 0x78, 0x0, 0x4]
        };
        self.read(offsets)
    }

    fn menu_type(&self) -> i32 {
        if self.type_size() == 100 {
            let offsets = if self.version == PointerVersion::Xna {
                &[0x0, 0x98, 0x30, 0x0, 0x4]
            } else {
                &[0x0, 0x78, 0x30, 0x0, 0x4]
            };
            self.read(offsets)
        } else {
            Menu::InGame as i32
        }
    }

    fn show_input_ui(&self) -> bool {
        if self.type_size() == 100 {
            let offsets = if self.version == PointerVersion::Xna {
                &[0x0, 0x98, 0x2b]
            } else {
                &[0x0, 0x78, 0x2b]
            };
            self.read::<u8>(offsets) != 0
        } else {
            false
        }
    }

    fn chapter_split(
        &mut self,
        area_id: i32,
        chapter_area: Area,
        level: &[u16],
        completed: bool,
    ) -> bool {
        if !self.exiting_chapter {
            let not_in_credits = if chapter_area == Area::TheSummit {
                !level.starts_with(wstr!("credits")) // TODO: May need to be case insensitive
            } else {
                true
            };
            self.exiting_chapter = area_id == chapter_area as i32
                && completed
                && !self.last_completed
                && not_in_credits;
            self.exiting_chapter && self.il_splits
        } else {
            !completed && self.last_completed
        }
    }
}

fn find_base() -> Option<Celeste> {
    const POTENTIAL_SIGNATURES: [(PointerVersion, &str, i32); 3] = [
        (
            PointerVersion::Xna,
            "83C604F30F7E06660FD6078BCBFF15????????8D15",
            21,
        ),
        (
            PointerVersion::OpenGl,
            "8B55F08B45E88D5274E8????????8B45F08D15",
            19,
        ),
        (
            PointerVersion::Itch,
            "8D5674E8????????8D15????????E8????????C605",
            10,
        ),
    ];

    for &(version, signature, offset) in &POTENTIAL_SIGNATURES {
        if let Some(Address(address)) = asl::scan_signature(signature) {
            let address = (address as u32).wrapping_add(offset as u32) as u64;
            let base = unsafe { asl::read_val(Address(address)) };
            return Some(Celeste {
                version,
                base,
                last_show_input_ui: false,
                last_completed: false,
                il_splits: false,
                exiting_chapter: false,
            });
        }
    }

    None
}

fn state() -> Option<MappedMutexGuard<'static, Celeste>> {
    let mut state = STATE.lock();
    if state.is_none() {
        *state = find_base();
    }
    MutexGuard::try_map(state, |x| x.as_mut()).ok()
}

#[no_mangle]
pub extern "C" fn disconnect() {
    // TODO: Make sure this actually works.
    *STATE.lock() = None;
    CURRENT_SPLIT.store(0, Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn should_start() -> bool {
    if let Some(mut state) = state() {
        let show_input_ui = state.show_input_ui();
        let should_start = !show_input_ui
            && state.last_show_input_ui
            && state.menu_type() == Menu::FileSelect as i32;
        state.last_show_input_ui = show_input_ui;
        if should_start {
            CURRENT_SPLIT.store(0, Ordering::Relaxed);
            return true;
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn should_split() -> bool {
    if let Some(mut state) = state() {
        let completed = state.chapter_completed();
        let area_id = state.area_id();
        // TODO: addAmount
        let level_name = state.level_name();

        let current_split = CURRENT_SPLIT.load(Ordering::Relaxed);
        let split_type = DEFAULT_SPLITS[current_split];

        let should_split = match split_type {
            SplitType::ChapterA => state.chapter_split(
                Area::Prologue as i32,
                Area::Prologue,
                &level_name,
                completed,
            ),
            SplitType::Prologue => {
                state.chapter_split(area_id, Area::Prologue, &level_name, completed)
            }
            SplitType::Chapter1 => {
                state.chapter_split(area_id, Area::ForsakenCity, &level_name, completed)
            }
            SplitType::Chapter2 => {
                state.chapter_split(area_id, Area::OldSite, &level_name, completed)
            }
            SplitType::Chapter3 => {
                state.chapter_split(area_id, Area::CelestialResort, &level_name, completed)
            }
            SplitType::Chapter4 => {
                state.chapter_split(area_id, Area::GoldenRidge, &level_name, completed)
            }
            SplitType::Chapter5 => {
                state.chapter_split(area_id, Area::MirrorTemple, &level_name, completed)
            }
            SplitType::Chapter6 => {
                state.chapter_split(area_id, Area::Reflection, &level_name, completed)
            }
            SplitType::Chapter7 => {
                state.chapter_split(area_id, Area::TheSummit, &level_name, completed)
            }
            SplitType::Epilogue => {
                state.chapter_split(area_id, Area::Epilogue, &level_name, completed)
            }
            SplitType::Chapter8 => state.chapter_split(area_id, Area::Core, &level_name, completed),
            SplitType::Chapter1Checkpoint1 => {
                area_id == Area::ForsakenCity as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("6")[..]
                        } else {
                            &wstr!("04")[..]
                        }
            }
            SplitType::Chapter1Checkpoint2 => {
                area_id == Area::ForsakenCity as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("9b")[..]
                        } else {
                            &wstr!("08")[..]
                        }
            }

            SplitType::Chapter2Checkpoint1 => {
                area_id == Area::OldSite as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("3")[..]
                        } else {
                            &wstr!("03")[..]
                        }
            }
            SplitType::Chapter2Checkpoint2 => {
                area_id == Area::OldSite as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("end_3")[..]
                        } else {
                            &wstr!("08b")[..]
                        }
            }
            SplitType::Chapter3Checkpoint1 => {
                area_id == Area::CelestialResort as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("08-a")[..]
                        } else {
                            &wstr!("06")[..]
                        }
            }
            SplitType::Chapter3Checkpoint2 => {
                area_id == Area::CelestialResort as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("09-d")[..]
                        } else {
                            &wstr!("11")[..]
                        }
            }
            SplitType::Chapter3Checkpoint3 => {
                area_id == Area::CelestialResort as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("00-d")[..]
                        } else {
                            &wstr!("16")[..]
                        }
            }
            SplitType::Chapter4Checkpoint1 => {
                area_id == Area::GoldenRidge as i32 && level_name == wstr!("b-00")
            }
            SplitType::Chapter4Checkpoint2 => {
                area_id == Area::GoldenRidge as i32 && level_name == wstr!("c-00")
            }
            SplitType::Chapter4Checkpoint3 => {
                area_id == Area::GoldenRidge as i32 && level_name == wstr!("d-00")
            }
            SplitType::Chapter5Checkpoint1 => {
                area_id == Area::MirrorTemple as i32 && level_name == wstr!("b-00")
            }
            SplitType::Chapter5Checkpoint2 => {
                area_id == Area::MirrorTemple as i32 && level_name == wstr!("c-00")
            }
            SplitType::Chapter5Checkpoint3 => {
                area_id == Area::MirrorTemple as i32 && level_name == wstr!("d-00")
            }
            SplitType::Chapter5Checkpoint4 => {
                area_id == Area::MirrorTemple as i32 && level_name == wstr!("e-00")
            }
            SplitType::Chapter6Checkpoint1 => {
                area_id == Area::Reflection as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("00")[..]
                        } else {
                            &wstr!("b-00")[..]
                        }
            }
            SplitType::Chapter6Checkpoint2 => {
                area_id == Area::Reflection as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("04")[..]
                        } else {
                            &wstr!("c-00")[..]
                        }
            }
            SplitType::Chapter6Checkpoint3 => {
                area_id == Area::Reflection as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("b-00")[..]
                        } else {
                            &wstr!("d-00")[..]
                        }
            }
            SplitType::Chapter6Checkpoint4 => {
                area_id == Area::Reflection as i32 && level_name == wstr!("boss-00")
            }
            SplitType::Chapter6Checkpoint5 => {
                area_id == Area::Reflection as i32 && level_name == wstr!("after-00")
            }
            SplitType::Chapter7Checkpoint1 => {
                area_id == Area::TheSummit as i32 && level_name == wstr!("b-00")
            }
            SplitType::Chapter7Checkpoint2 => {
                area_id == Area::TheSummit as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("c-00")[..]
                        } else {
                            &wstr!("c-01")[..]
                        }
            }
            SplitType::Chapter7Checkpoint3 => {
                area_id == Area::TheSummit as i32 && level_name == wstr!("d-00")
            }
            SplitType::Chapter7Checkpoint4 => {
                area_id == Area::TheSummit as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("e-00b")[..]
                        } else {
                            &wstr!("e-00")[..]
                        }
            }
            SplitType::Chapter7Checkpoint5 => {
                area_id == Area::TheSummit as i32 && level_name == wstr!("f-00")
            }
            SplitType::Chapter7Checkpoint6 => {
                area_id == Area::TheSummit as i32 && level_name == wstr!("g-00")
            }
            SplitType::Chapter8Checkpoint1 => {
                area_id == Area::Core as i32 && level_name == wstr!("a-00")
            }
            SplitType::Chapter8Checkpoint2 => {
                area_id == Area::Core as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("c-00")[..]
                        } else {
                            &wstr!("b-00")[..]
                        }
            }
            SplitType::Chapter8Checkpoint3 => {
                area_id == Area::Core as i32
                    && level_name
                        == if state.area_difficulty() == AreaMode::ASide as i32 {
                            &wstr!("d-00")[..]
                        } else {
                            &wstr!("c-01")[..]
                        }
            }

            _ => unimplemented!(),
        };

        state.last_completed = completed;
        // state.last_level_name = level_name;

        // state.last_elapsed = elapsed;

        if should_split {
            state.exiting_chapter = false;
            CURRENT_SPLIT.fetch_add(1, Ordering::Relaxed);
            return true;
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn is_loading() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn game_time() -> f64 {
    if let Some(state) = state() {
        asl::set_variable("Strawberries", &state.strawberries().to_string());
        asl::set_variable("Level Timer", &format!("{:.2}s", state.level_time()));
        let elapsed = if state.il_splits {
            state.level_time()
        } else {
            state.game_time()
        };
        elapsed
    } else {
        std::f64::NAN
    }
}
