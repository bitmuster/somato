use crate::location::Location;
use calamine::{Data, DataType};
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
