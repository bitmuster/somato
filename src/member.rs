use crate::location::Location;
use anyhow::Result;
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
use std::collections;
use std::fmt;

#[derive(Debug)]
pub struct Member {
    pub contract_no: String,
    pub member_no: u32,
    pub surname: String,
    pub forename: String,
    pub big: u32,
    pub small: u32,
    pub location: Location,
    pub active: bool,
}

impl Member {
    pub fn new(
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
            location: Location::Gerlingen,
            active: true,
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

pub fn read_members() -> Result<Vec<Member>> {
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

pub fn check_member_list(members: &Vec<Member>) {
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
