/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine
https://docs.rs/calamine/latest/calamine/

https://docs.rs/anyhow/latest/anyhow/
https://docs.rs/injectorpp/latest/injectorpp/

*/

use crate::joker;
use crate::location::Location;
use crate::member;
use crate::tickoff;
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;
use std::fs;
use std::path;
use strum::IntoEnumIterator;
use toml::value::Datetime;

#[derive(Deserialize)]
pub struct Config {
    members: String,
    jokers: String,
    tickoff: String,
    date: String,
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
    // let members_file = "tests/test_data/members_synthetic.xlsx";
    // let joker_file = "tests/test_data/jokers_synthetic.xlsx";
    // let tickoff_file = "tests/test_data/tickoff_synthetic.xlsx";

    // somato_runner(members_file, joker_file, tickoff_file)?;

    // let config: Config = toml::from_str(&fs::read_to_string("config.toml")?)?;
    let config = read_config("config.toml")?;

    somato_runner(
        &config.members,
        &config.jokers,
        &config.tickoff,
        &config.date,
    )?;

    println!("{}", "*".repeat(80));
    Ok(())
}

/// Run analytics based on given configuration.
pub fn somato_runner(
    members_file: &str,
    joker_file: &str,
    tickoff_file: &str,
    date: &str,
) -> Result<()> {
    let members = member::read_members(&members_file)?;
    let jokers = joker::read_jokers(&joker_file)?;
    let mut warnings = 0;
    println!("Some exemplary members:");
    for member in members.iter().take(5) {
        println!("    {}", member);
    }

    println!("Some exemplary jokers:");
    for joker in jokers.iter().take(5) {
        println!("    {}", joker);
    }

    println!("  Parsed {} members", members.len());
    println!("  Parsed {} jokers", jokers.len());

    member::check_member_list(&members);
    joker::check_joker_list(&members, &jokers);

    let active_members = member::filter_active_members(members.clone());

    let date = chrono::naive::NaiveDate::from_ymd_opt(2025, 12, 19).unwrap();
    let weekly_jokers = joker::filter_jokers_by_date(jokers.clone(), date);
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
        member::filter_jokers(&active_members, &weekly_jokers);
    let jokers_big = member::filter_members_by_big(&active_collectors);
    let jokers_small = member::filter_members_by_small(&active_collectors);
    println!(
        "Active collectors: all {}, big: {}, small: {}",
        active_members.len(),
        jokers_big.len(),
        jokers_small.len()
    );
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

        let tick_off = tickoff::tick_off_list(tickoff_file, &location)?;
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
    use super::*;
    use core::any::Any;
    use injectorpp::interface::injector::*;

    fn get_injector_ok() -> InjectorPP {
        let mut injector = InjectorPP::new();
        injector
            .when_called(injectorpp::func!(fn (somato_runner)(&str,&str,&str,&str) -> Result<()>))
            .will_execute(injectorpp::fake!(
                func_type: fn(_a:&str,_b:&str,_c:&str,_d:&str) -> Result<()>,
                returns: Ok(()),
                times: 1
            ));
        return injector;
    }

    #[test]
    pub fn test_somato_main() {
        let _inj = get_injector_ok(); // With the name it is not optimised away
        let result = somato_main();
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    /// Issues
    /// 1) the injector is not cleaned up correctly
    /// 2) creating a second injector fails as well
    #[ignore]
    #[test]
    pub fn test_somato_main_fake() {
        let mut injector = get_injector_ok(); // With the name it is not optimised away

        let _config: Config = toml::from_str(
            r#"
        members = "tests/test_data/members_synthetic.xlsx"
        jokers = "tests/test_data/jokers_synthetic.xlsx"
        tickoff = "tests/test_data/tickoff_synthetic.xlsx"
        date = "2025-12-19"
        "#,
        )
        .unwrap();

        // For some reason complains about lifetimes
        // injector
        //     .when_called(
        //         injectorpp::func!(fn (toml::from_str)(&str) -> std::result::Result<Config, toml::de::Error>),
        //     );
        // .will_execute(injectorpp::fake!(
        //     func_type: fn(_a:&str) -> Result<Config>,
        //     returns: ||{config},
        //     times: 1
        // ));

        // let mut injector_2 = InjectorPP::new();
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
}
