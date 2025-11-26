use crate::location::Location;
use crate::member::Member;
use anyhow::{Result, anyhow};
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
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

pub fn check_joker_list(members: &Vec<Member>, jokers: &Vec<Joker>) {
    println!("Checking Joker List");
    let mut joker_warnings = 0;
    let warn_limit = 5;
    'outer: for j in jokers.iter() {
        for m in members.iter() {
            if j.surname.to_lowercase() == m.surname.to_lowercase()
                && j.forename.to_lowercase() == m.forename.to_lowercase()
            {
                // println!("Found {}", j.surname);
                continue 'outer;
            }
        }
        if joker_warnings < warn_limit {
            println!(
                "Cannot find Joker line {} name:  {} {}",
                j.line, j.surname, j.forename
            );
        }

        joker_warnings += 1;
    }
    println!("Overall Joker warnings {}", joker_warnings);
}

pub fn read_jokers() -> Result<Vec<Joker>> {
    let joker_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        Joker_Solawi-Heckengaeu.xlsx";
    let mut excel: Xlsx<_> = open_workbook(joker_file).unwrap();

    let mut jokers = Vec::new();
    if let Ok(r) = excel.worksheet_range("Eingabe") {
        let mut line = 2;
        for row in r.rows().skip(1) {
            // println!("row={:?}, row[0]={:?}", row, row[0]);
            // println!(
            //     "{} {} {} {} {} {} {} {}",
            //     row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]
            // );
            // if let Data::DateTime(date) = row[0] {
            //     println!("{}", NaiveDate::from(date.as_datetime().unwrap()));
            // }
            let date = &row[0];
            let name = &row[1];
            let forename = &row[2];
            let warning = &row[3];
            let location = &row[4];
            let big = &row[5];
            let small = &row[6];
            let joker = Joker::new(
                &date, &name, &forename, warning, &location, big, small, line,
            )?;
            jokers.push(joker);
            line += 1;
        }
    }
    Ok(jokers)
}
