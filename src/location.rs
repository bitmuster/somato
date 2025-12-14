use anyhow::{Result, anyhow};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum Location {
    Perouse,
    Gerlingen,
    Renningen,
    WeilDerStadt,
    Leonberg,
    Neuhausen,
    NotParsed,
}

impl Location {
    pub fn parse(location: &str) -> Result<Location> {
        let loc = match location {
            "Gerlingen" => Location::Gerlingen,
            "Perouse" => Location::Perouse,
            "Weil der Stadt" => Location::WeilDerStadt,
            "Renningen" => Location::Renningen,
            "Leonberg" => Location::Leonberg,
            "Neuhausen" => Location::Neuhausen,
            "Error NA" => Location::NotParsed,
            _ => return Err(anyhow!("Cannot parse Location {location}")),
        };
        Ok(loc)
    }
    pub fn to_short(location: &Self) -> &'static str {
        match location {
            Self::Perouse => "PER",
            Self::Gerlingen => "GER",
            Self::Renningen => "REN",
            Self::WeilDerStadt => "WDS",
            Self::Leonberg => "LEO",
            Self::Neuhausen => "NEU",
            Self::NotParsed => "NOT",
        }
    }
}
