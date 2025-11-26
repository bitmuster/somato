use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
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
}
