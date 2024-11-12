// Weighted probability of being picked?
// Need positive, negative, neutral traits associated with actions
// Villagers should be able to get a few traits, not just one. 1-3?
// traits should play off one another / have interactions in determining outcomes
// more traits is more fun but may not be practical
// some traits exclusive of one another, but keep to a minimum, people are complicated
// villagers may or may not know about each others' personality traits - maybe pick some to be "secret" and decrease probability?
// Mark whether considered a flaw or not, flaw makes people less willing to share things affected by flaw
// flaws can also be things villagers are trying to overcome and have opposite / unexpected effect most of the time, but expected effect part of the time
pub enum PersonalityTrait {
    // should be connected to probability of theft or spite actions
    Jealous,
    // connected to violent actions, also sharp boost to negative actions immediately following anger trigger that trails off quickly
    HotHead,
    // carries grudges, boost to negative action vs villager / family who triggered anger that stays over time
    Grudge,
    // less likely to anger others
    Polite,
    // more likely to anger others
    Condescending,
    // less likely to anger others, but different interactions than polite
    Charming,
    // more likely to remember events that happened nearby
    GoodMemory,
    // less likely to remember events
    PoorMemory,
    // more likely to notice events
    Observant,
    // less likely
    Unobservant,
    // very unlikely to lie
    Honest,
    // more likely than usual to lie, especially about Secrets or flaws, may make things up if they also have poor memory
    Dishonest,
    // unwilling to share information with stranger
    Distrustful,
    // very willing to share info
    Open,
    // willing to share wealth - unlikely to be stolen from by most people
    Generous,
    // willing to steal from generous people
    Taker,
    // unwilling to share wealth - easy to anger people with poor wealth especially
    Stingy,
    // increased chance of retaliatory crime vs suspected perpetrators, especially crime toward self, family, friends
    Vengeful,
    // increases determination to resolve crime vs family, or may commit retaliation on those caught in act
    Protector,
    // shares too much but not necessarily useful stuff, requires good memory
    OverlyGoodMemory,
    // Decreases chance villager will be noticed by others all the time
    Sneaky,
    // increases chance villager will be noticed
    Loud,
    // reduces connections to other villagers, replace with connection to forest/animals
    NatureLover,
    // Doubles effects of other modifiers on event chances, lowers chances of understanding, remembering, and correctly answering questions
    Simple,
    // increases connection/positive modifiers on not suspecting characters with certain flaws, such as Simple
    Empathetic,
    // suspects everybody if a crime occurs. high modifier to suspicion checks
    Suspicious,
    // suspects nobody and is hard to convince that anybody did anything bad, but very high negative modifier if they become convinced
    Trusting,
    // increases the chance and scale of random changes in wealth, especially negative ones
    GamblingHabit,
    // complete effects TBD but increases chance that villager will commit crime and not remember, also decreases chance of remembering things in general especially when drunk
    Alcoholic,
    // Introduces themselves to the player by a false name
    FalseName,
    // Increases chance of wandering randomly around town
    Wanderer,
}
