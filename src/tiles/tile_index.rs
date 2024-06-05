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
}

pub enum Path {
    StraightNorthSouth = 57,
    CornerSouthToEast = 58,
    TeeNorthSouthEast = 59,
    Cross = 60,
    EndFromSouth = 61,
}
