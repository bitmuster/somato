/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine
https://docs.rs/calamine/latest/calamine/

*/
use anyhow::Result;
use calamine::{DataType, Reader, Xlsx, open_workbook};
use std::collections;
mod joker;
mod location;
mod member;
use joker::Joker;
use location::Location;
use member::Member;

fn read_jokers() -> Result<Vec<Joker>> {
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

fn read_members() -> Result<Vec<Member>> {
    let members_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        08_Mitgliederliste/\
        2023-03-20_Mitgliederliste-Solawi-Heckengaeu_v3_Test_neu_fixed.xlsx";
    let mut excel: Xlsx<_> = open_workbook(members_file).unwrap();

    let mut members = Vec::new();
    if let Ok(r) = excel.worksheet_range("Ernteverträge") {
        for row in r.rows().skip(1).take(251) {
            // println!("row={:?}, row[0]={:?}", row, row[0]);
            // println!(
            //     "{} {} {} {} {} {} {} {}",
            //     row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]
            // );

            if (0..3).map(|x| row[x].is_empty()).any(|x| x == true) {
                // println!("Continuing");
                continue;
            };
            let contract_no = &row[0];
            let member_no = &row[1];
            let surname = &row[2];
            let forename = &row[3];
            let big = &row[5];
            let small = &row[6];
            let member = Member::new(
                contract_no,
                member_no,
                surname,
                forename,
                big,
                small,
            );
            members.push(member);
        }
    };
    Ok(members)
}

fn check_member_list(members: &Vec<Member>) {
    let mut surname_set = collections::HashSet::new();
    for member in members.iter() {
        if surname_set.insert(&member.surname) {
        } else {
            println!(
                "Duplicated surname: {} {} {}",
                member.surname, member.contract_no, member.member_no
            );
        }
    }
    let mut member_no_set = collections::HashSet::new();
    for member in members.iter() {
        if member_no_set.insert(&member.member_no) {
        } else {
            println!(
                "Duplicated member number: {} {} {}",
                member.surname, member.contract_no, member.member_no
            );
        }
    }
    let mut contract_no_set = collections::HashSet::new();
    for member in members.iter() {
        if contract_no_set.insert(&member.contract_no) {
        } else {
            println!(
                "Duplicated contract number: {} {} {}",
                member.surname, member.contract_no, member.member_no
            );
        }
    }
}

fn check_joker_list(members: &Vec<Member>, jokers: &Vec<Joker>) {
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

fn main() -> Result<()> {
    println!("Hello, world!");
    let jokers = read_jokers()?;
    let members = read_members()?;

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

    check_member_list(&members);
    check_joker_list(&members, &jokers);
    Ok(())
}
