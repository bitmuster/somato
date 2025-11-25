/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine
https://docs.rs/calamine/latest/calamine/

*/
use anyhow::{Result, anyhow};
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
use chrono::NaiveDate;
use std::collections;
use std::fmt;

#[derive(Debug)]
enum Location {
    Perouse,
    Gerlingen,
    Renningen,
    WeilDerStadt,
    Leonberg,
    Neuhausen,
    NotParsed,
}

#[derive(Debug)]
struct Joker {
    date: NaiveDate,
    surname: String,
    forename: String,
    warning: u32,
    location: Location,
    big: u32,
    small: u32,
    line: u32,
}

impl Joker {
    fn new(
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
        let location = match location_str.as_str()
            // .as_string()
            // .expect(format!("Cannot parse location {:?}", location).as_str())
            // .as_str()
        {
            "Gerlingen" => Location::Gerlingen,
            "Perouse" => Location::Perouse,
            "Weil der Stadt" => Location::WeilDerStadt,
            "Renningen" => Location::Renningen,
            "Leonberg"=> Location::Leonberg,
            "Neuhausen"=> Location::Neuhausen,
            "Error NA" => Location::NotParsed,
            _ => {return Err(anyhow!("Cannot parse Location {location}"))}
        };
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

#[derive(Debug)]
struct Member {
    contract_no: String,
    member_no: u32,
    surname: String,
    forename: String,
    big: u32,
    small: u32,
}

impl Member {
    fn new(
        contract_no: &Data,
        member_no: &Data,
        surname: &Data,
        forename: &Data,
        big: &Data,
        small: &Data,
    ) -> Self {
        // Its text not a number
        // let member_no = member_no.get_int().unwrap_or(8888) as u32;
        let member_no = member_no
            .as_string()
            .unwrap()
            .parse::<u32>()
            .expect("Cannot parse member no");
        let member = Member {
            contract_no: contract_no.as_string().unwrap(),
            member_no,
            surname: surname
                .as_string()
                .expect(&String::from(format!("cannot parse \"{}\"", surname))),
            forename: forename.as_string().unwrap(),
            big: big
                .as_string()
                .unwrap()
                .parse::<u32>()
                .expect("Cannot parse big"),
            small: small
                .as_string()
                .unwrap()
                .parse::<u32>()
                .expect("Cannot parse small"),
        };
        // println!("{}", member);
        member
    }
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Member: {} {} {} {} {} {}",
            self.contract_no,
            self.member_no,
            self.surname,
            self.forename,
            self.big,
            self.small
        )
    }
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
    println!("Overll Joker warnings {}", joker_warnings);
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
