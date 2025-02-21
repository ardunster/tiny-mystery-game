pub enum GroundTile {
    Empty = 0,
    GrassMixed = 1,
    PathDirt = 2,
    TileRosetteLight = 3,
    TileRosetteHeavy = 4,
    GrassFine = 5,
    GrassFlower = 6,
    GrassThick = 7,
    TileHerring = 16,
    TileTriangle = 17,
    LadderEnd = 21,
    Ladder = 70,
    Rock = 103,
}

pub enum Cobble {
    NorthWestCorner = 18,
    North = 19,
    NorthEastCorner = 20,
    West = 67,
    Center = 68,
    East = 69,
    SouthWestCorner = 116,
    South = 117,
    SouthEastCorner = 118,
    BrokenNorthWest = 165,
    BrokenNorthEast = 166,
    SlantNorthWest = 167,
    SlantNorthEast = 168,
    BrokenSouthWest = 214,
    BrokenSouthEast = 215,
    SlantSouthWest = 216,
    SlantSouthEast = 218,
}

pub enum Person {
    PriestRobe = 24,
    Person = 25,
    Spear = 26,
    SwordShield = 27,
    HelmetSwordShield = 28,
    HelmetSpearShield = 29,
    Armor = 30,
    HornedHelmetSwordShield = 31,
    WizardRobe = 73,
    Peasant = 74,
    Woman = 75,
    ButtonDownShirt = 76,
    Sash = 77,
    Tie = 78,
    Hood = 79,
    HoodWithStaff = 80,
    RobeWithBeard = 122,
    MaskWithSword = 126,
    TwoSwords = 128,
    Farmer = 129,
    Brute = 171,
    King = 175,
    Queen = 176,
    Boy = 177,
    Girl = 178,
    Miner = 223,
    Cowboy = 224,
    OldMan = 225,
    LongButtonDownShirt = 226,
    PriestHabit = 227,
    HoodAndBeard = 318,
    HoodAndForkBeard = 319,
    BruteTwo = 466,
    Gnome = 467,
    Ninja = 470,
    Ranger = 471,
}

pub enum Face {
    Boy = 514,
    Girl = 515,
    ManBeard = 516,
    ManBald = 517,
    Man = 518,
    ManMoustache = 519,
    WomanBangs = 520,
    WomanStraightBangs = 521,
}

pub enum Plant {
    ConiferFlat = 49,
    ConiferRough = 50,
    Deciduous = 51,
    DeciduousPair = 52,
    DeciduousThick = 53,
    FruitTree = 54,
    Cactus = 55,
    CactusPair = 56,
    LongGrass = 98,
    Thorns = 99,
    Vine = 100,
    ConiferPair = 101,
    Oak = 102,
    DeadThorns = 104,
    PalmTree = 105,
    GrassPatch = 119,
    Furrows = 306,
    Sprouts = 307,
    CornSprouts = 308,
    Cabbages = 309,
    FlowerSprouts = 310,
    VineSprouts = 311,
}

pub enum Path {
    StraightNorthSouth = 57,
    CornerSouthToEast = 58,
    TeeNorthSouthEast = 59,
    Cross = 60,
    EndFromSouth = 61,
    BridgeArch = 202,
    BridgeRaisedSolid = 203,
    BridgeArchLow = 251,
    BridgeRaisedPlanks = 252,
    BridgeSuspension = 260,
    BridgeFlatPlanks = 261,
    BridgeFlatBroken = 262,
}

pub enum Fence {
    PicketShort = 147,
    PicketPointed = 148,
    PicketTall = 149,
    PicketGateClosed = 150,
    PicketGateOpen = 151,
    Iron = 152,
    IronBroken = 153,
    IronGateSolidClosed = 199,
    IronGateOpen = 200,
    IronGateClosed = 201,
}

pub enum Water {
    StraightNorthSouth = 204,
    CornerSouthToEast = 205,
    TeeNorthSouthEast = 206,
    Cross = 207,
    EndFromSouth = 208,
    Solid = 253,
    West = 254,
    NorthWestCorner = 255,
    ThreeSidesSouthWestEast = 256,
    TrickleNorthSouth = 257,
    TrickleCornerSouthToEast = 258,
    PoolTrickleFromNorth = 259,
}

pub enum Critter {
    Scorpion = 269,
    Crab = 270,
    Bee = 271,
    Turtle = 272,
    Spider = 273,
    SpiderTwo = 274,
    Ant = 275,
    Duck = 368,
    Chicken = 369,
    Cow = 370,
    Horse = 371,
    Pig = 372,
    Cat = 373,
    Dog = 374,
    Bat = 418,
    Snake = 420,
    Bear = 422,
    Rat = 423,
}

pub enum Building {
    Watchtower = 508,
    HouseAFrame = 509,
    ThatchHutAFrame = 931,
    ThatchHutWindows = 932,
    StoneTower = 933,
    ThatchRoundedRoof = 980,
    ThatchTall = 981,
    Church = 983,
    WoodenHut = 984,
    WoodenAFrame = 986,
    RusticAFrame = 987,
    Pagoda = 988,
    Barn = 1029,
    HouseChimney = 1030,
    HousePillars = 1031,
}
