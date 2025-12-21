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
use std::path;
use strum::IntoEnumIterator;

/// Main entry point for somato
pub fn somato_main() -> Result<()> {
    println!("{}", "*".repeat(80));
    let members_file = "tests/test_data/members_synthetic.xlsx";
    let joker_file = "tests/test_data/jokers_synthetic.xlsx";
    let tickoff_file = "tests/test_data/tickoff_synthetic.xlsx";

    somato_runner(members_file, joker_file, tickoff_file)?;

    println!("{}", "*".repeat(80));
    let base_folder = path::Path::new(
        "/home/micha/Repos/SolawiKommisionierSpielplatz/Daten_Stand_20251217",
    );
    let members_file = base_folder
        .join("2023-12-17_Mitgliederliste-Solawi-Heckengaeu_v3_Test2.xlsx");
    let joker_file = base_folder.join("Joker_Solawi-Heckengaeu.xlsx");
    let tickoff_file = base_folder.join("2024-10-28_Abhaklisten.xlsx");

    somato_runner(
        members_file.to_str().unwrap(),
        joker_file.to_str().unwrap(),
        tickoff_file.to_str().unwrap(),
    )?;

    println!("{}", "*".repeat(80));
    Ok(())
}

/// Run analytics based on given configuration.
pub fn somato_runner(
    members_file: &str,
    joker_file: &str,
    tickoff_file: &str,
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

    let _date = chrono::naive::NaiveDate::from_ymd_opt(2025, 11, 21).unwrap();
    let _date = chrono::naive::NaiveDate::from_ymd_opt(2025, 8, 15).unwrap();
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
    use injectorpp::interface::injector::*;

    #[test]
    pub fn test_somato_main() {
        let mut injector = InjectorPP::new();
        injector
            .when_called(injectorpp::func!(fn (somato_runner)(&str,&str,&str) -> Result<()>))
            .will_execute(injectorpp::fake!(
                func_type: fn(_a: &str,_b: &str,_c: &str) -> Result<()>,
                returns: Ok(()),
                times: 2
            ));

        assert!(somato_main().is_ok());
    }
}
