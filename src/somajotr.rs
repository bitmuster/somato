use anyhow::Result;

use crate::joker;
use crate::location::Location;
use crate::member;
use crate::tickoff;

pub fn somajotr() -> Result<()> {
    println!("Hello, world!");
    let members_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        08_Mitgliederliste/\
        2023-03-20_Mitgliederliste-Solawi-Heckengaeu_v3_Test_neu_fixed.xlsx";
    // let members_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
    //     Entenhausen/\
    //     members_entenhausen.xlsx";
    // let joker_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
    //     Joker_Solawi-Heckengaeu.xlsx";
    let joker_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        Daten_Stand_2025.11.27/\
        Joker_Solawi-Heckengaeu.xlsx";
    let tickoff_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        Daten_Stand_2025.11.27/\
        2024-10-28_Abhaklisten.xlsx";

    let jokers = joker::read_jokers(&joker_file)?;
    let members = member::read_members(&members_file)?;

    println!("Some Members:");
    for member in members.iter().take(5) {
        println!("{}", member);
    }

    println!("Some Jokers:");
    for joker in jokers.iter().take(5) {
        println!("{}", joker);
    }

    println!("Found {} members", members.len());
    println!("Found {} jokers", jokers.len());

    member::check_member_list(&members);
    joker::check_joker_list(&members, &jokers);
    let active_members = member::filter_active_members(members.clone());
    let _gerlingen = member::filter_members_by_location(
        &active_members,
        Location::Gerlingen,
    );
    let date = chrono::naive::NaiveDate::from_ymd_opt(2025, 11, 21).unwrap();
    let weekly_jokers = joker::filter_jokers_by_date(jokers.clone(), date);
    println!("Weekly jokers {} at {}", weekly_jokers.len(), date);
    let weekly_jokers_gerlingen = joker::filter_jokers_by_location(
        weekly_jokers.clone(),
        Location::Gerlingen,
    );
    println!(
        "Weekly jokers {} at {} in Gerlingen",
        weekly_jokers_gerlingen.len(),
        date
    );
    let filtered_members =
        member::filter_jokers(&active_members, &weekly_jokers);
    let _ = member::filter_members_by_big(&members);
    let _ = member::filter_members_by_small(&members);
    println!("Analysis for Gerlingen:");
    let gerlingen = member::filter_members_by_location(
        &filtered_members,
        Location::Gerlingen,
    );
    let mb = member::filter_members_by_big(&gerlingen);
    let ms = member::filter_members_by_small(&gerlingen);
    member::print_members(&mb);
    member::print_members(&ms);
    tickoff::tick_off_list(tickoff_file)?;
    Ok(())
}
