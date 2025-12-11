use crate::joker;
use crate::location::Location;
use anyhow::{Result, anyhow};
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
use colorama::Colored;
use std::collections;
use std::fmt;

#[derive(Debug, Clone)]
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

pub type MemberList = Vec<Member>;

impl Member {
    pub fn new(
        contract_no: &Data,
        member_no: &Data,
        surname: &Data,
        forename: &Data,
        big: &Data,
        small: &Data,
        location: &Data,
        active: &Data,
    ) -> Result<Self> {
        // Its text not a number
        // let member_no = member_no.get_int().unwrap_or(8888) as u32;
        let member_no =
            member_no.as_string().unwrap().parse::<u32>().map_err(|e| {
                anyhow!(format!(
                    "Cannot parse member no {:?} reason: {}",
                    member_no, e
                ))
            })?;
        let location_str = location.as_string().unwrap();
        let active_bool = match active.as_string().unwrap().as_str() {
            "aktiv" => true,
            "inaktiv" => false,
            _ => {
                return Err(anyhow!(format!(
                    "Error while parsing activity {}",
                    active
                )));
            }
        };
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
            location: Location::parse(&location_str).unwrap(),
            active: active_bool,
        };
        // println!("{}", member);
        Ok(member)
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

pub fn read_members(members_file: &str) -> Result<Vec<Member>> {
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
            let location = &row[15];
            let active = &row[19];
            let member = Member::new(
                contract_no,
                member_no,
                surname,
                forename,
                big,
                small,
                location,
                active,
            )?;
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

pub fn filter_active_members(members: MemberList) -> MemberList {
    let result: MemberList = members.into_iter().filter(|m| m.active).collect();
    println!("Found {} active members", result.len());
    result
}

pub fn filter_jokers(
    members: &MemberList,
    jokers: &joker::JokerList,
) -> MemberList {
    let mut result: MemberList = Vec::new();

    'outer: for m in members.iter() {
        for j in jokers {
            if j.surname.to_lowercase() == m.surname.to_lowercase()
                && j.forename.to_lowercase() == m.forename.to_lowercase()
            {
                println!(
                    "    Joker set: {} {} from {:?}",
                    j.surname.to_string().color("blue"),
                    j.forename,
                    m.location
                );
                continue 'outer;
            }
        }
        result.push(m.clone());
    }

    println!("Found {} active members with no jokers", result.len());
    result
}

pub fn filter_members_by_location(
    members: &MemberList,
    location: Location,
) -> MemberList {
    let result: MemberList = members
        .clone()
        .into_iter()
        .filter(|m| m.location == location)
        .collect();
    println!("  Found {} members in {:?}", result.len(), location);
    // println!("{:?}", result);
    result
}

pub fn filter_members_by_small(members: &MemberList) -> MemberList {
    let result: MemberList = members
        .clone()
        .into_iter()
        .filter(|m| m.small >= 1)
        .collect();
    println!("  Found {} members with size small", result.len());
    // println!("{:?}", result);
    result
}

pub fn filter_members_by_big(members: &MemberList) -> MemberList {
    let result: MemberList =
        members.clone().into_iter().filter(|m| m.big >= 1).collect();
    println!("  Found {} members with size big", result.len());
    // println!("{:?}", result);
    result
}

#[allow(dead_code)]
pub fn print_members(members: &MemberList) {
    for m in members {
        println!("Member: {} {} {} {}", m.surname, m.forename, m.big, m.small);
    }
}

#[cfg(test)]
mod member_tests {

    use super::Member;
    use calamine::Data;

    #[test]
    fn new_member() {
        let contract_no = Data::String("EV".to_string());
        let member_no = Data::Int(87);
        let surname = Data::String("John".to_string());
        let forename = Data::String("Smith".to_string());
        let big = Data::Int(88);
        let small = Data::Int(89);
        let location = Data::String("Perouse".to_string());
        let active = Data::String("aktiv".to_string());
        let _ = Member::new(
            &contract_no,
            &member_no,
            &surname,
            &forename,
            &big,
            &small,
            &location,
            &active,
        );
    }
    #[test]
    fn new_member_aktiv() {
        let _m: Member = Member::new(
            &Data::String("EV".to_string()),
            &Data::Int(87),
            &Data::String("John".to_string()),
            &Data::String("Smith".to_string()),
            &Data::Int(88),
            &Data::Int(89),
            &Data::String("Perouse".to_string()),
            &Data::String("aktiv".to_string()),
        )
        .unwrap();
    }
    #[test]
    fn new_member_inaktiv() {
        let _m: Member = Member::new(
            &Data::String("EV".to_string()),
            &Data::Int(87),
            &Data::String("John".to_string()),
            &Data::String("Smith".to_string()),
            &Data::Int(88),
            &Data::Int(89),
            &Data::String("Perouse".to_string()),
            &Data::String("inaktiv".to_string()),
        )
        .unwrap();
    }
    #[test]
    fn new_member_active_error() {
        let m = Member::new(
            &Data::String("EV".to_string()),
            &Data::Int(87),
            &Data::String("John".to_string()),
            &Data::String("Smith".to_string()),
            &Data::Int(88),
            &Data::Int(89),
            &Data::String("Perouse".to_string()),
            &Data::String("defect".to_string()),
        );
        assert!(m.is_err(), "Failed to parse activity");
    }
    #[test]
    fn new_big_amount_fail() {
        let m = Member::new(
            &Data::String("EV".to_string()),
            &Data::Int(88),
            &Data::String("John".to_string()),
            &Data::String("Smith".to_string()),
            &Data::String("Fail".to_string()),
            &Data::Int(89),
            &Data::String("Perouse".to_string()),
            &Data::String("defect".to_string()),
        );
        assert!(m.is_err(), "Failed to parse contract number {:?}", m);
    }
    #[test]
    fn new_small_amount_fail() {
        let m = Member::new(
            &Data::String("EV".to_string()),
            &Data::String("Fail".to_string()),
            &Data::String("John".to_string()),
            &Data::String("Smith".to_string()),
            &Data::Int(88),
            &Data::String("Fail".to_string()),
            &Data::String("Perouse".to_string()),
            &Data::String("defect".to_string()),
        );
        assert!(m.is_err(), "Failed to parse contract number {:?}", m);
    }
    #[test]
    fn new_member_number() {
        let m = Member::new(
            &Data::String("EV".to_string()),
            &Data::String("Fail".to_string()),
            &Data::String("John".to_string()),
            &Data::String("Smith".to_string()),
            &Data::Int(88),
            &Data::Int(89),
            &Data::String("Perouse".to_string()),
            &Data::String("defect".to_string()),
        );
        assert!(m.is_err(), "Failed to parse contract number {:?}", m);
    }
}
