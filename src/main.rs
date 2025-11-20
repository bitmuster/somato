/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine
https://docs.rs/calamine/latest/calamine/

*/
use anyhow::{Result, anyhow};
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
use chrono::NaiveDate;
use std::fmt;

#[derive(Debug)]
enum Location {
    Perouse,
    Gerlingen,
    Renningen,
    WeilDerStadt,
}

#[derive(Debug)]
struct Joker {
    date: NaiveDate,
    name: String,
    forename: String,
    warning: u32,
    location: Location,
    big: u32,
    small: u32,
}

impl Joker {
    fn new(date: &Data, name: &Data, forename: &Data) -> Result<Joker> {
        let ndate = match date {
            Data::DateTime(date) => {
                NaiveDate::from(date.as_datetime().unwrap())
            }
            _ => return Err(anyhow!("Cannot parse date: {:?}", date)),
        };
        // println!("{:?}", forename);
        let joker = Self {
            date: ndate,
            name: name.as_string().unwrap(),
            forename: forename
                .as_string()
                .unwrap_or(format!("I errow while parsing \"{:?}\"", forename)),
            warning: 0,
            location: Location::Gerlingen,
            big: 0,
            small: 0,
        };
        // println!("{}", joker);
        Ok(joker)
    }
}
impl fmt::Display for Joker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.date, self.name, self.forename, self.warning
        )
    }
}

fn read_jokers() -> Result<Vec<Joker>> {
    let joker_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/Joker_Solawi-Heckengaeu.xlsx";
    let mut excel: Xlsx<_> = open_workbook(joker_file).unwrap();

    let mut jokers = Vec::new();
    if let Ok(r) = excel.worksheet_range("Eingabe") {
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
            let joker = Joker::new(&date, &name, &forename)?;
            jokers.push(joker);
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
}

impl Member {
    fn new(
        contract_no: &Data,
        member_no: &Data,
        surname: &Data,
        forename: &Data,
    ) -> Self {
        // Its text not a number
        // let member_no = member_no.get_int().unwrap_or(8888) as u32;
        let member_no = member_no
            .as_string()
            .unwrap()
            .parse::<u32>()
            .expect("Cannot parse");
        let member = Member {
            contract_no: contract_no.as_string().unwrap(),
            member_no,
            surname: surname
                .as_string()
                .expect(&String::from(format!("cannot parse \"{}\"", surname))),
            forename: forename.as_string().unwrap(),
        };
        // println!("{}", member);
        member
    }
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Member: {} {} {} {}",
            self.contract_no, self.member_no, self.surname, self.forename
        )
    }
}

fn read_members() -> Result<()> {
    let members_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/08_Mitgliederliste/2023-03-20_Mitgliederliste-Solawi-Heckengaeu_v3_Test_neu_fixed.xlsx";
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
            let surename = &row[2];
            let forename = &row[3];
            let member =
                Member::new(contract_no, member_no, surename, forename);
            members.push(member);
        }
    };
    Ok(())
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let jokers = read_jokers()?;
    let members = read_members();

    for joker in jokers {
        println!("{:?}", joker);
    }
    Ok(())
}
