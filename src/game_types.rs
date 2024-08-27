use asr::settings::{gui::Title, Gui};

#[derive(Gui)]
pub struct Settings {
    /// General Settings
    _general_settings: Title,
    /// Use chapter timer (as opposed to file timer)
    pub level_time: bool,

    /// General Splits
    _general_splits: Title,
    /// Manual Split (Not Automatic)
    pub manual: bool,
    /// Any Chapter (Complete)
    pub chapter: bool,
    /// Level (On Enter)
    pub level_enter: bool,
    /// Level (On Exit)
    pub level_exit: bool,

    /// Chapter Splits
    _chapter_splits: Title,
    /// Prologue (Complete)
    pub prologue: bool,
    /// Chapter 1 - Forsaken City A/B/C (Complete)
    pub chapter1: bool,
    /// Chapter 2 - Old Site A/B/C (Complete)
    pub chapter2: bool,
    /// Chapter 3 - Celestial Resort A/B/C (Complete)
    pub chapter3: bool,
    /// Chapter 4 - Golden Ridge A/B/C (Complete)
    pub chapter4: bool,
    /// Chapter 5 - Mirror Temple A/B/C (Complete)
    pub chapter5: bool,
    /// Chapter 6 - Reflection A/B/C (Complete)
    pub chapter6: bool,
    /// Chapter 7 - The Summit A/B/C (Complete)
    pub chapter7: bool,
    /// Epilogue (Complete)
    pub epilogue: bool,
    /// Chapter 8 - Core A/B/C (Complete)
    pub chapter8: bool,

    /// Checkpoint Splits
    _checkpoint_splits: Title,
    /// Chapter 1 - Crossing (A) / Contraption (B) (CP 1)
    pub chapter1_checkpoint1: bool,
    /// Chapter 1 - Chasm (A) / Scrap Pit (B) (CP 2)
    pub chapter1_checkpoint2: bool,
    /// Chapter 2 - Intervention (A) / Combination Lock (B) (CP 1)
    pub chapter2_checkpoint1: bool,
    /// Chapter 2 - Awake (A) / Dream Altar (B) (CP 2)
    pub chapter2_checkpoint2: bool,
    /// Chapter 3 - Huge Mess (A) / Staff Quarters (B) (CP 1)
    pub chapter3_checkpoint1: bool,
    /// Chapter 3 - Elevator Shaft (A) / Library (B) (CP 2)
    pub chapter3_checkpoint2: bool,
    /// Chapter 3 - Presidential Suite (A) / Rooftop (B) (CP 3)
    pub chapter3_checkpoint3: bool,
    /// Chapter 4 - Shrine (A) / Stepping Stones (B) (CP 1)
    pub chapter4_checkpoint1: bool,
    /// Chapter 4 - Old Trail (A) / Gusty Canyon (B) (CP 2)
    pub chapter4_checkpoint2: bool,
    /// Chapter 4 - Cliff Face (A) / Eye Of The Storm (B) (CP 3)
    pub chapter4_checkpoint3: bool,
    /// Chapter 5 - Depths (A) / Central Chamber (B) (CP 1)
    pub chapter5_checkpoint1: bool,
    /// Chapter 5 - Unravelling (A) / Through The Mirror (B) (CP 2)
    pub chapter5_checkpoint2: bool,
    /// Chapter 5 - Search (A) / Mix Master (B) (CP 3)
    pub chapter5_checkpoint3: bool,
    /// Chapter 5 - Rescue (A) (CP 4)
    pub chapter5_checkpoint4: bool,
    /// Chapter 6 - Lake (A) / Reflection (B) (CP 1)
    pub chapter6_checkpoint1: bool,
    /// Chapter 6 - Hollows (A) / Rock Bottom (B) (CP 2)
    pub chapter6_checkpoint2: bool,
    /// Chapter 6 - Reflection (A) / Reprieve (B) (CP 3)
    pub chapter6_checkpoint3: bool,
    /// Chapter 6 - Rock Bottom (A) (CP 4)
    pub chapter6_checkpoint4: bool,
    /// Chapter 6 - Resolution (A) (CP 5)
    pub chapter6_checkpoint5: bool,
    /// Chapter 7 - 500M (A) / 500M (B) (CP 1)
    pub chapter7_checkpoint1: bool,
    /// Chapter 7 - 1000M (A) / 1000M (B) (CP 2)
    pub chapter7_checkpoint2: bool,
    /// Chapter 7 - 1500M (A) / 1500M (B) (CP 3)
    pub chapter7_checkpoint3: bool,
    /// Chapter 7 - 2000M (A) / 2000M (B) (CP 4)
    pub chapter7_checkpoint4: bool,
    /// Chapter 7 - 2500M (A) / 2500M (B) (CP 5)
    pub chapter7_checkpoint5: bool,
    /// Chapter 7 - 3000M (A) / 3000M (B) (CP 6)
    pub chapter7_checkpoint6: bool,
    /// Chapter 8 - Into The Core (A) / Into The Core (B) (CP 1)
    pub chapter8_checkpoint1: bool,
    /// Chapter 8 - Hot And Cold (A) / Burning Or Freezing (B) (CP 2)
    pub chapter8_checkpoint2: bool,
    /// Chapter 8 - Heart Of The Mountain (A) / Heartbeat (B) (CP 3)
    pub chapter8_checkpoint3: bool,

    /// Collectable Splits
    _collectable_splits: Title,
    /// Chapter 1 - Cassette (Pickup)
    pub chapter1_cassette: bool,
    /// Chapter 1 - Heart Gem A/B/C (Pickup)
    pub chapter1_heartgem: bool,
    /// Chapter 2 - Cassette (Pickup)
    pub chapter2_cassette: bool,
    /// Chapter 2 - Heart Gem A/B/C (Pickup)
    pub chapter2_heartgem: bool,
    /// Chapter 3 - Cassette (Pickup)
    pub chapter3_cassette: bool,
    /// Chapter 3 - Heart Gem A/B/C (Pickup)
    pub chapter3_heartgem: bool,
    /// Chapter 4 - Cassette (Pickup)
    pub chapter4_cassette: bool,
    /// Chapter 4 - Heart Gem A/B/C (Pickup)
    pub chapter4_heartgem: bool,
    /// Chapter 5 - Cassette (Pickup)
    pub chapter5_cassette: bool,
    /// Chapter 5 - Heart Gem A/B/C (Pickup)
    pub chapter5_heartgem: bool,
    /// Chapter 6 - Cassette (Pickup)
    pub chapter6_cassette: bool,
    /// Chapter 6 - Heart Gem A/B/C (Pickup)
    pub chapter6_heartgem: bool,
    /// Chapter 7 - Cassette (Pickup)
    pub chapter7_cassette: bool,
    /// Chapter 7 - Heart Gem A/B/C (Pickup)
    pub chapter7_heartgem: bool,
    /// Chapter 8 - Cassette (Pickup)
    pub chapter8_cassette: bool,
    /// Chapter 8 - Heart Gem A/B/C (Pickup)
    pub chapter8_heartgem: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum Area {
    Menu = -1,
    Prologue = 0,
    ForsakenCity = 1,
    OldSite = 2,
    CelestialResort = 3,
    GoldenRidge = 4,
    MirrorTemple = 5,
    Reflection = 6,
    TheSummit = 7,
    Epilogue = 8,
    Core = 9,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AreaMode {
    ASide,
    BSide,
    CSide,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum Menu {
    InGame = 0,
    Intro = 14,
    FileSelect = 60,
    MainMenu = 64,
    ChapterSelect = 80,
    ChapterPanel = 168,
    FileRename = 180,
}
