use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Region {
    NorthAmerica,       // U.S., Canada, Mexico
    LatinAmerica,       // Central and South America
    EuropeanUnion,      // EU countries
    EasternEurope,      // Non-EU Eastern Europe
    SubSaharanAfrica,
    NorthAfrica,
    MENA,               // Middle East and North Africa
    ChinaEastAsia,      // China + adjacent
    SouthAsia,          // India, Pakistan, Bangladesh
    SoutheastAsia,      // ASEAN
    CentralAsia,
    Oceania,            // Australia, NZ, Pacific Islands
    RussiaCIS,          // Russia + Commonwealth of Independent States
    Global,             // Fallback or aggregated value
}
