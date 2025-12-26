/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine
https://docs.rs/calamine/latest/calamine/

https://docs.rs/anyhow/latest/anyhow/
https://docs.rs/injectorpp/latest/injectorpp/

*/

pub use crate::joker;
pub use crate::location::Location;
pub use crate::member;
pub use crate::test_common;
pub use crate::tickoff;
use anyhow::Result;
use anyhow::anyhow;
use chrono::Datelike;
use chrono::naive;
use colored::Colorize;
use serde::Deserialize;
use std::fs;
use strum::IntoEnumIterator;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    pub members: String,
    pub jokers: String,
    pub tickoff: String,
    pub date: String,
}

/// Read and return the base config.
/// Separated into a function with simpler interface.
pub fn read_config(file: &str) -> Result<Config> {
    let config: Config = toml::from_str(&fs::read_to_string(file)?)?;
    Ok(config)
}

/// Main entry point for somato
pub fn somato_main() -> Result<()> {
    println!("{}", "*".repeat(80));

    let config = read_config("config_synth.toml")?;

    somato_runner(&config)?;

    println!("{}", "*".repeat(80));
    Ok(())
}
pub fn parse_date(date: &str) -> Result<naive::NaiveDate> {
    let mut date_split = date.split("-");
    let year = date_split
        .next()
        .ok_or(anyhow!("Cannot parse year"))?
        .parse::<i32>()?;
    let month = date_split
        .next()
        .ok_or(anyhow!("Cannot parse month"))?
        .parse::<u32>()?;
    let day = date_split
        .next()
        .ok_or(anyhow!("Cannot parse day"))?
        .parse::<u32>()?;
    if !date_split.next().is_none() {
        return Err(anyhow!("Found extra tokens in date"));
    }

    let date = chrono::naive::NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(anyhow!("Cannot process date"))?;
    println!("Parsed Date {:?} {}", date, date.weekday());
    if date.weekday() != chrono::Weekday::Fri {
        return Err(anyhow!("Distribution should be on Fridays"));
    }
    Ok(date)
}

/// Analyses the current state of Jokers
/// Returns the amount of active collectors, collectors for big and small.
pub fn analyze_jokers(
    active_members: &[member::Member],
    jokers: &[joker::Joker],
    date: &chrono::NaiveDate,
) -> (usize, usize, usize) {
    let weekly_jokers = joker::filter_jokers_by_date(jokers, date);
    println!("Weekly jokers {} at {}", weekly_jokers.len(), date);

    for location in Location::iter() {
        let weekly_jokers_loc =
            joker::filter_jokers_by_location(weekly_jokers.clone(), &location);

        println!(
            "    Weekly jokers {} at {} in {:?}",
            weekly_jokers_loc.len(),
            date,
            &location
        );
    }

    let active_collectors =
        member::filter_jokers(active_members, &weekly_jokers);
    let members_jokers_big = member::filter_members_by_big(&active_collectors);
    let members_jokers_small =
        member::filter_members_by_small(&active_collectors);
    println!(
        "Active collectors: all {}, big: {}, small: {}",
        active_collectors.len(),
        members_jokers_big.len(),
        members_jokers_small.len()
    );
    (
        active_collectors.len(),
        members_jokers_big.len(),
        members_jokers_small.len(),
    )
}

/// Run analytics based on given configuration.
pub fn somato_runner(config: &Config) -> Result<()> {
    let members = member::read_members(&config.members)?;
    let jokers = joker::read_jokers(&config.jokers)?;
    let mut warnings = 0;

    println!("  Parsed {} members", members.len());
    println!("  Parsed {} jokers", jokers.len());

    member::check_member_list(&members);
    joker::check_joker_list(&members, &jokers);

    let active_members = member::filter_active_members(members.clone());
    let date = parse_date(&config.date)?;
    analyze_jokers(&active_members, &jokers, &date);

    for location in Location::iter() {
        println!("{}", "*".repeat(80));
        println!("* Analysis for: {location:?}");
        println!("{}", "*".repeat(80));
        let loc =
            member::filter_members_by_location(&active_members, &location);
        let mb = member::filter_members_by_big(&loc);
        let ms = member::filter_members_by_small(&loc);
        let all = loc.len();
        let big = mb.len();
        let small = ms.len();
        println!("  Found {all}, big {big}, small {small}");
        let diff: i32 = all as i32 - big as i32 - small as i32;

        if diff != 0 {
            println!(
                "  {}",
                format!("Difference in member/portion amount {}", diff)
                    .to_string()
                    .red()
            );
        }
        // member::print_members(&mb);
        // member::print_members(&ms);

        let tick_off = tickoff::tick_off_list(&config.tickoff, &location)?;
        if let Some(warn) =
            tickoff::check_for_members_in_tickoff_list(&loc, &tick_off).unwrap()
        {
            warnings += warn;
        };
        if let Some(warn) =
            tickoff::check_tickoff_list_against_members(&loc, &tick_off)
                .unwrap()
        {
            warnings += warn;
        }
    }

    println!("Accumulated {warnings} warnings");
    Ok(())
}

#[cfg(test)]
mod test_somato {
    use crate::test_common::test_common::*;

    use super::*;
    use injectorpp::interface::injector::*;

    #[ignore] // Fails randomly, when run in multiple threads
    #[test]
    pub fn test_somato_main() {
        let mut injector = InjectorPP::new();
        injector
            .when_called(
                injectorpp::func!(fn (somato_runner)(&Config) -> Result<()>),
            )
            .will_execute(injectorpp::fake!(
                func_type: fn(_c:&Config) -> Result<()>,
                returns: Ok(()),
                times: 1
            ));
        let result = somato_main();
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[ignore] // Fails randomly, when run in multiple threads
    #[test]
    pub fn test_somato_main_2() {
        let mut injector = InjectorPP::new();
        injector
            .when_called(
                injectorpp::func!(fn (somato_runner)(&Config) -> Result<()>),
            )
            .will_execute(injectorpp::fake!(
                func_type: fn(_c:&Config) -> Result<()>,
                returns: Ok(()),
                times: 1
            ));
        let result = somato_main();
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    /// Issues
    /// 1) the injector is not cleaned up correctly
    /// 2) creating a second injector seems to hang forever
    #[ignore] // Cannot work that way
    #[test]
    pub fn test_somato_main_fake() {
        let mut injector = InjectorPP::new();
        injector
            .when_called(
                injectorpp::func!(fn (somato_runner)(&Config) -> Result<()>),
            )
            .will_execute(injectorpp::fake!(
                func_type: fn(_c:&Config) -> Result<()>,
                returns: Ok(()),
                times: 1
            ));

        let _config: Config = toml::from_str(
            r#"
        members = "tests/test_data/members_synthetic.xlsx"
        jokers = "tests/test_data/jokers_synthetic.xlsx"
        tickoff = "tests/test_data/tickoff_synthetic.xlsx"
        date = "2025-12-19"
        "#,
        )
        .unwrap();

        // injector_2
        injector
            .when_called(
                injectorpp::func!(fn (read_config)(&str) -> Result<Config>),
            )
            .will_execute(injectorpp::fake!(
            func_type: fn(_a:&str) -> Result<Config>,
            returns: Ok(Config{
                members : "tests/test_data/members_synthetic.xlsx".to_string(),
                jokers : "tests/test_data/jokers_synthetic.xlsx".to_string(),
                tickoff : "tests/test_data/tickoff_synthetic.xlsx".to_string(),
                date : "2025-12-19".to_string(),
            }),
            times: 1
            ));

        let result = somato_main();
        println!("{:?}", result);
        assert!(result.is_ok());
        // make sure the injector guard is not optimised away
        // println!("{:?}", inj.type_id());
    }
    #[test]
    fn test_read_config() {
        let config_expect = Config {
            members: "tests/test_data/members_synthetic.xlsx".to_string(),
            jokers: "tests/test_data/jokers_synthetic.xlsx".to_string(),
            tickoff: "tests/test_data/tickoff_synthetic.xlsx".to_string(),
            date: "2025-11-07".to_string(),
        };
        let config =
            read_config("config_synth.toml").expect("Failed to parse config");
        assert_eq!(config, config_expect);
    }
    #[test]
    fn test_somato_runner() {
        let config = Config {
            members: "tests/test_data/members_synthetic.xlsx".to_string(),
            jokers: "tests/test_data/jokers_synthetic.xlsx".to_string(),
            tickoff: "tests/test_data/tickoff_synthetic.xlsx".to_string(),
            date: "2025-11-07".to_string(),
        };
        let result = somato_runner(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_date() {
        assert_eq!(
            parse_date("2025-11-07").unwrap(),
            naive::NaiveDate::from_ymd_opt(2025, 11, 7).unwrap()
        );
    }

    #[test]
    fn test_parse_date_2() -> Result<(), anyhow::Error> {
        assert_eq!(
            parse_date("2025-11-07")?,
            naive::NaiveDate::from_ymd_opt(2025, 11, 7)
                .ok_or(anyhow!("Fail"))?
        );
        Ok(())
    }

    #[test]
    fn test_parse_date_fail() {
        assert!(parse_date("2025-1107").is_err());
        assert!(parse_date("202511-07").is_err());
        assert!(parse_date("2025-11-07-99").is_err());
        assert!(parse_date("20251107").is_err());
        assert!(parse_date("what").is_err());
        assert!(parse_date("0000-00-00").is_err());
    }

    #[test]
    fn test_analyze_jokers() {
        let date = naive::NaiveDate::from_ymd_opt(2025, 11, 7).unwrap();
        let members = gen_members();
        let jokers = gen_jokers();
        let (m, b, s) = analyze_jokers(&members, &jokers, &date);
        assert_eq!(m, 3);
        assert_eq!(b, 2);
        assert_eq!(s, 2);
    }
}
