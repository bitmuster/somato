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
    let jokers = joker::read_jokers()?;
    let members = member::read_members()?;

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
    Ok(())
}
