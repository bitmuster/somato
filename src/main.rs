/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine
https://docs.rs/calamine/latest/calamine/

*/
use anyhow::{Result, anyhow};
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
use chrono::NaiveDate;

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
            _ => return Err(anyhow!("oh")),
        };
        let joker = Self {
            date: ndate,
            name: name.as_string().unwrap(),
            forename: forename.as_string().unwrap(),
            warning: 0,
            location: Location::Gerlingen,
            big: 0,
            small: 0,
        };
        println!("{:?}", joker);
        Ok(joker)
    }
}

fn read() -> Result<()> {
    let joker_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/Joker_Solawi-Heckengaeu.xlsx";
    let mut excel: Xlsx<_> = open_workbook(joker_file).unwrap();

    if let Ok(r) = excel.worksheet_range("Eingabe") {
        for row in r.rows() {
            // println!("row={:?}, row[0]={:?}", row, row[0]);
            println!(
                "{} {} {} {} {} {} {} {}",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]
            );
            if let Data::DateTime(date) = row[0] {
                println!("{}", NaiveDate::from(date.as_datetime().unwrap()));
            }
            let date = &row[0];
            let name = &row[1];
            let forename = &row[1];
            let joker = Joker::new(&date, &name, &forename);
        }
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
    read();
}
