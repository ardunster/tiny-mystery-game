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
}

pub enum Cobble {
    NorthWestCorner = 18,
    North = 19,
    NorthEastCorner = 20,
    West,
    Center,
    East,
    SouthWestCorner,
    South,
    SouthEastCorner,
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
}

pub enum Plant {
    TreeConiferFlat = 49,
    TreeConiferRough = 50,
    TreeDeciduous = 51,
    TreeDeciduousPair = 52,
    TreeDeciduousThick = 53,
    TreeJungle = 54,
    Cactus = 55,
    CactusPair = 56,
}

pub enum Path {
    StraightNorthSouth = 57,
    CornerSouthToEast = 58,
    TeeNorthSouthEast = 59,
    Cross = 60,
    EndFromSouth = 61,
}
