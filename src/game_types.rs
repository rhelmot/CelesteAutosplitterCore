#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SplitType {
    /// Manual Split (Not Automatic)
    Manual,
    /// Any Chapter (Complete)
    ChapterA,
    /// Level (On Enter)
    LevelEnter,
    /// Level (On Exit)
    LevelExit,
    /// Prologue (Complete)
    Prologue,
    /// Chapter 1 - Crossing (A) / Contraption (B) (CP 1)
    Chapter1Checkpoint1,
    /// Chapter 1 - Chasm (A) / Scrap Pit (B) (CP 2)
    Chapter1Checkpoint2,
    /// Chapter 1 - Forsaken City A/B/C (Complete)
    Chapter1,
    /// Chapter 2 - Intervention (A) / Combination Lock (B) (CP 1)
    Chapter2Checkpoint1,
    /// Chapter 2 - Awake (A) / Dream Altar (B) (CP 2)
    Chapter2Checkpoint2,
    /// Chapter 2 - Old Site A/B/C (Complete)
    Chapter2,
    /// Chapter 3 - Huge Mess (A) / Staff Quarters (B) (CP 1)
    Chapter3Checkpoint1,
    /// Chapter 3 - Elevator Shaft (A) / Library (B) (CP 2)
    Chapter3Checkpoint2,
    /// Chapter 3 - Presidential Suite (A) / Rooftop (B) (CP 3)
    Chapter3Checkpoint3,
    /// Chapter 3 - Celestial Resort A/B/C (Complete)
    Chapter3,
    /// Chapter 4 - Shrine (A) / Stepping Stones (B) (CP 1)
    Chapter4Checkpoint1,
    /// Chapter 4 - Old Trail (A) / Gusty Canyon (B) (CP 2)
    Chapter4Checkpoint2,
    /// Chapter 4 - Cliff Face (A) / Eye Of The Storm (B) (CP 3)
    Chapter4Checkpoint3,
    /// Chapter 4 - Golden Ridge A/B/C (Complete)
    Chapter4,
    /// Chapter 5 - Depths (A) / Central Chamber (B) (CP 1)
    Chapter5Checkpoint1,
    /// Chapter 5 - Unravelling (A) / Through The Mirror (B) (CP 2)
    Chapter5Checkpoint2,
    /// Chapter 5 - Search (A) / Mix Master (B) (CP 3)
    Chapter5Checkpoint3,
    /// Chapter 5 - Rescue (A) (CP 4)
    Chapter5Checkpoint4,
    /// Chapter 5 - Mirror Temple A/B/C (Complete)
    Chapter5,
    /// Chapter 6 - Lake (A) / Reflection (B) (CP 1)
    Chapter6Checkpoint1,
    /// Chapter 6 - Hollows (A) / Rock Bottom (B) (CP 2)
    Chapter6Checkpoint2,
    /// Chapter 6 - Reflection (A) / Reprieve (B) (CP 3)
    Chapter6Checkpoint3,
    /// Chapter 6 - Rock Bottom (A) (CP 4)
    Chapter6Checkpoint4,
    /// Chapter 6 - Resolution (A) (CP 5)
    Chapter6Checkpoint5,
    /// Chapter 6 - Reflection A/B/C (Complete)
    Chapter6,
    /// Chapter 7 - 500M (A) / 500M (B) (CP 1)
    Chapter7Checkpoint1,
    /// Chapter 7 - 1000M (A) / 1000M (B) (CP 2)
    Chapter7Checkpoint2,
    /// Chapter 7 - 1500M (A) / 1500M (B) (CP 3)
    Chapter7Checkpoint3,
    /// Chapter 7 - 2000M (A) / 2000M (B) (CP 4)
    Chapter7Checkpoint4,
    /// Chapter 7 - 2500M (A) / 2500M (B) (CP 5)
    Chapter7Checkpoint5,
    /// Chapter 7 - 3000M (A) / 3000M (B) (CP 6)
    Chapter7Checkpoint6,
    /// Chapter 7 - The Summit A/B/C (Complete)
    Chapter7,
    /// Epilogue (Complete)
    Epilogue,
    /// Chapter 8 - Into The Core (A) / Into The Core (B) (CP 1)
    Chapter8Checkpoint1,
    /// Chapter 8 - Hot And Cold (A) / Burning Or Freezing (B) (CP 2)
    Chapter8Checkpoint2,
    /// Chapter 8 - Heart Of The Mountain (A) / Heartbeat (B) (CP 3)
    Chapter8Checkpoint3,
    /// Chapter 8 - Core A/B/C (Complete)
    Chapter8,
    /// Chapter 1 - Cassette (Pickup)
    Chapter1Cassette,
    /// Chapter 1 - Heart Gem A/B/C (Pickup)
    Chapter1HeartGem,
    /// Chapter 2 - Cassette (Pickup)
    Chapter2Cassette,
    /// Chapter 2 - Heart Gem A/B/C (Pickup)
    Chapter2HeartGem,
    /// Chapter 3 - Cassette (Pickup)
    Chapter3Cassette,
    /// Chapter 3 - Heart Gem A/B/C (Pickup)
    Chapter3HeartGem,
    /// Chapter 4 - Cassette (Pickup)
    Chapter4Cassette,
    /// Chapter 4 - Heart Gem A/B/C (Pickup)
    Chapter4HeartGem,
    /// Chapter 5 - Cassette (Pickup)
    Chapter5Cassette,
    /// Chapter 5 - Heart Gem A/B/C (Pickup)
    Chapter5HeartGem,
    /// Chapter 6 - Cassette (Pickup)
    Chapter6Cassette,
    /// Chapter 6 - Heart Gem A/B/C (Pickup)
    Chapter6HeartGem,
    /// Chapter 7 - Cassette (Pickup)
    Chapter7Cassette,
    /// Chapter 7 - Heart Gem A/B/C (Pickup)
    Chapter7HeartGem,
    /// Chapter 8 - Cassette (Pickup)
    Chapter8Cassette,
    /// Chapter 8 - Heart Gem A/B/C (Pickup)
    Chapter8HeartGem,
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
