use crate::location::Location;
use crate::member::Member;
use anyhow::{Result, anyhow};
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
use chrono::NaiveDate;
use colored::Colorize;
use std::fmt;

#[derive(Debug, Clone)]
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

pub type JokerList = Vec<Joker>;

impl Joker {
    #[allow(clippy::too_many_arguments)]
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
        // println!("{date:?}");
        let ndate = match date {
            Data::DateTime(date) => {
                NaiveDate::from(date.as_datetime().unwrap())
            }
            _ => {
                return Err(anyhow!(
                    "Cannot parse date \"{}\" on line {}",
                    date,
                    line
                ));
            }
        };
        let location_str =
            location.as_string().unwrap_or("Error NA".to_string());
        let location_str = location_str.trim();
        let location = Location::parse(location_str)?;

        // This can be Error(NA) when the contract is inactive
        match forename {
            Data::String(_s) => {
                // println!("Forename Found String {s}")
            }
            Data::Error(calamine::CellErrorType::NA) => {
                // N/A is the only error type that we allow here due to
                // inactive entries and defective queries
                // println!("Forename: Found Error N/A")
            }
            s => {
                println!("Found inacceptable data in forename {s:?}");
                return Err(anyhow!(
                    "Found inacceptable data type in forename: {}",
                    s
                ));
            }
        }
        let forename = forename
            .as_string()
            .unwrap_or(format!("Error while parsing \"{:?}\"", forename))
            .trim()
            .to_string();

        let surname = surname
            .as_string()
            .unwrap_or(format!("Error while parsing \"{:?}\"", surname))
            .trim()
            .to_string();

        let joker = Self {
            date: ndate,
            surname,
            forename,
            warning: warning.as_i64().expect("Cannot parse warning") as u32,
            location,
            // big: big.as_i64().expect("Cannot parse big") as u32,
            // small: small.as_i64().expect("Cannot parse small") as u32,
            big: big.as_i64().unwrap_or(88) as u32,
            small: small.as_i64().unwrap_or(88) as u32,
            line,
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

pub fn check_joker_names(members: &[Member], jokers: &[Joker]) -> Result<u32> {
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
            if j.surname.to_lowercase() == m.surname.to_lowercase() && !m.active
            {
                // println!("  Ignoring inactive entry {}", j.surname);
                continue 'outer;
            }
        }
        if joker_warnings < warn_limit {
            println!(
                "{}",
                format!(
                    "  Cannot find Joker line {} in member list name: \"{}\" forename: \"{}\"",
                    j.line, j.surname, j.forename
                )
                .red()
            );
        }

        joker_warnings += 1;
    }
    Ok(joker_warnings)
}

pub fn check_joker_list(members: &[Member], jokers: &[Joker]) -> Result<u32> {
    println!("Checking Joker List");
    let mut joker_warnings = 0;
    let warn_limit = 5;
    joker_warnings += check_joker_names(members, jokers)?;
    println!(
        "{}",
        format!("  Overall Joker warnings {}", joker_warnings).red()
    );
    Ok(joker_warnings)
}

pub fn read_jokers(joker_file: &str) -> Result<Vec<Joker>> {
    let mut excel: Xlsx<_> = open_workbook(joker_file).map_err(|e| {
        anyhow!(format!("Error {e} while loading joker file {joker_file}"))
    })?;

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
                date, name, forename, warning, location, big, small, line,
            )?;
            jokers.push(joker);
            line += 1;
        }
    }
    Ok(jokers)
}

pub fn filter_jokers_by_date(
    jokers: &[Joker],
    date: &chrono::NaiveDate,
) -> JokerList {
    let result: JokerList = jokers
        .to_owned()
        .into_iter()
        .filter(|j| j.date == *date)
        .collect();
    println!("  Filtered {} jokers at {}", result.len(), date);
    result
}

pub fn filter_jokers_by_location(
    jokers: JokerList,
    location: &Location,
) -> JokerList {
    let result: JokerList = jokers
        .into_iter()
        .filter(|j| j.location == *location)
        .collect();
    println!("  Filtered {} jokers at {:?}", result.len(), location);
    result
}

#[cfg(test)]
mod joker_tests {

    use super::*;
    use crate::test_common::test_common;
    use injectorpp::interface::injector::*;

    #[test]
    fn test_new_wrong_date() {
        let j = Joker::new(
            &Data::String("wrongDate".to_string()),
            &Data::String("Smith".to_string()),
            &Data::String("John".to_string()),
            &Data::Int(88),
            &Data::String(" Perouse ".to_string()),
            &Data::Int(80),
            &Data::Int(81),
            88,
        );
        assert!(j.is_err());
        // println!("{:?}", j);
        if let Err(e) = j {
            assert_eq!(
                e.to_string(),
                "Cannot parse date \"wrongDate\" on line 88"
            );
        }
    }
    #[test]
    fn test_new() {
        let j = Joker::new(
            &Data::DateTime(calamine::ExcelDateTime::new(
                45658.0,
                calamine::ExcelDateTimeType::DateTime,
                false,
            )),
            &Data::String("Smith".to_string()),
            &Data::String("John".to_string()),
            &Data::Int(88),
            &Data::String("Perouse".to_string()),
            &Data::Int(80),
            &Data::Int(81),
            88,
        );
        println!("{:?}", j);
        assert!(j.is_ok());
        println!("{:?}", j);
        let j = j.unwrap();
        assert_eq!(j.date, NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        assert_eq!(j.surname, "Smith");
        assert_eq!(j.forename, "John");
        assert_eq!(j.warning, 88);
        assert_eq!(j.location, Location::Perouse);
        assert_eq!(j.big, 80);
        assert_eq!(j.small, 81);
        assert_eq!(j.line, 88);
    }
    #[test]
    fn test_new_whitespaces() {
        let j = Joker::new(
            &Data::DateTime(calamine::ExcelDateTime::new(
                45658.0,
                calamine::ExcelDateTimeType::DateTime,
                false,
            )),
            &Data::String(" Smith ".to_string()),
            &Data::String("  John ".to_string()),
            &Data::Int(88),
            &Data::String("  Perouse ".to_string()),
            &Data::Int(80),
            &Data::Int(81),
            88,
        );
        println!("{:?}", j);
        assert!(j.is_ok());
        println!("{:?}", j);
        let j = j.unwrap();
        assert_eq!(j.date, NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        assert_eq!(j.surname, "Smith");
        assert_eq!(j.forename, "John");
        assert_eq!(j.warning, 88);
        assert_eq!(j.location, Location::Perouse);
        assert_eq!(j.big, 80);
        assert_eq!(j.small, 81);
        assert_eq!(j.line, 88);
    }

    #[test]
    fn test_check_joker_list() {
        let members = test_common::gen_members();
        let jokers = test_common::gen_jokers();

        let result = check_joker_list(&members, &jokers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_check_joker_list_fail() {
        let members = test_common::gen_members();
        let jokers = test_common::gen_jokers();
        let mut injector = InjectorPP::new();
        injector
            .when_called(
                injectorpp::func!(fn (check_joker_names)( &[Member], &[Joker]) -> Result<u32>),
            )
            .will_execute(injectorpp::fake!(
                func_type: fn(_m:&[Member], _j:&[Joker]) -> Result<u32>,
                returns: Ok(22),
                times: 1
            ));

        let result = check_joker_list(&members, &jokers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 22);
    }

    #[test]
    fn test_check_joker_names_extra_joker() {
        let members = test_common::gen_members();
        let mut jokers = test_common::gen_jokers().to_vec();
        let j = Joker {
            date: NaiveDate::from_ymd_opt(1, 1, 1).unwrap(),
            surname: "Nobody".to_string(),
            forename: "Nono".to_string(),
            warning: 0,
            location: Location::Perouse,
            big: 0,
            small: 2,
            line: 88,
        };
        jokers.push(j);
        let result = check_joker_names(&members, &jokers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
}
