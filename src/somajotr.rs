use anyhow::Result;

use crate::joker;
use crate::location::Location;
use crate::member;
use crate::tickoff;
use colorama::Colored;
use strum::IntoEnumIterator;

pub fn somajotr(
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
                format!("Difference in amount {}", diff)
                    .to_string()
                    .color("red")
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
