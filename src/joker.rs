use crate::location::Location;
use anyhow::{Result, anyhow};
use calamine::{Data, DataType};
use chrono::NaiveDate;
use std::fmt;

#[derive(Debug)]
pub struct Joker {
    pub date: NaiveDate,
    pub surname: String,
    pub forename: String,
    pub warning: u32,
    pub location: Location,
    pub big: u32,
    pub small: u32,
    pub line: u32,
}

impl Joker {
    pub fn new(
        date: &Data,
        surname: &Data,
        forename: &Data,
        warning: &Data,
        location: &Data,
        big: &Data,
        small: &Data,
        line: u32,
    ) -> Result<Joker> {
        let ndate = match date {
            Data::DateTime(date) => {
                NaiveDate::from(date.as_datetime().unwrap())
            }
            _ => return Err(anyhow!("Cannot parse date: {:?}", date)),
        };
        let location_str =
            location.as_string().unwrap_or("Error NA".to_string());
        let location = Location::parse(&location_str)?;
        // println!("{:?}", forename);
        let joker = Self {
            date: ndate,
            surname: surname.as_string().expect("Cannot parse surname"),
            forename: forename
                .as_string()
                .unwrap_or(format!("Error while parsing \"{:?}\"", forename)),
            warning: warning.as_i64().expect("Cannot parse warning") as u32,
            location: location,
            // big: big.as_i64().expect("Cannot parse big") as u32,
            // small: small.as_i64().expect("Cannot parse small") as u32,
            big: big.as_i64().unwrap_or(88) as u32,
            small: small.as_i64().unwrap_or(88) as u32,
            line: line,
        };
        // println!("{}", joker);
        Ok(joker)
    }
}

impl fmt::Display for Joker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Joker: {} {} {} {} {:?} {} {}",
            self.date,
            self.surname,
            self.forename,
            self.warning,
            self.location,
            self.small,
            self.big
        )
    }
}
