/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine
https://docs.rs/calamine/latest/calamine/

*/
use anyhow::Result;

mod joker;
mod location;
mod member;

fn main() -> Result<()> {
    println!("Hello, world!");
    let members_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        08_Mitgliederliste/\
        2023-03-20_Mitgliederliste-Solawi-Heckengaeu_v3_Test_neu_fixed.xlsx";
    let joker_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        Joker_Solawi-Heckengaeu.xlsx";
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
        location::Location::Gerlingen,
    );
    let date = chrono::naive::NaiveDate::from_ymd_opt(2025, 8, 8).unwrap();
    let weekly_jokers = joker::filter_jokers(jokers.clone(), date);
    let filtered_members =
        member::filter_jokers(&active_members, &weekly_jokers);
    let gerlingen = member::filter_members_by_location(
        &filtered_members,
        location::Location::Gerlingen,
    );
    let _ = member::filter_members_by_big(&members);
    let _ = member::filter_members_by_small(&members);
    let mb = member::filter_members_by_big(&gerlingen);
    let ms = member::filter_members_by_small(&gerlingen);
    member::print_members(&mb);
    member::print_members(&ms);
    Ok(())
}
