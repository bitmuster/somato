// use crate::location::Location;
use anyhow::{Result, anyhow};
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
// use chrono::NaiveDate;
use crate::member;
#[derive(Debug, Clone)]
pub struct TickOffItem {
    // pub date: NaiveDate,
    // pub surname: String,
    // pub forename: String,
    pub name: String,
    // pub location: Location,
    pub big: u32,
    pub small: u32,
    // pub line: u32,
}

impl TickOffItem {
    pub fn new(
        name: &Data,
        big: Option<&Data>,
        small: Option<&Data>,
    ) -> Result<Self> {
        // println!("Creating new entry with {:?} {:?} {:?}", name, big, small);
        let name = name
            .as_string()
            .ok_or_else(|| return anyhow!("Cannot parse \"{}\"", name));
        // .unwrap_or(format!("Error while parsing \"{:?}\"", name));
        let big = match big {
            Some(i) => i.as_i64().unwrap_or(88) as u32,
            None => 0,
        };
        let small = match small {
            Some(i) => i.as_i64().unwrap_or(88) as u32,
            None => 0,
        };
        let item = TickOffItem {
            name: name?,
            big,
            small,
        };
        println!(
            "Creating new entry with {:?} {:?} {:?}",
            item.name, item.big, item.small
        );
        Ok(item)
    }
}

pub fn check_lists(
    members: &member::MemberList,
    tickoff: &TickOffList,
) -> Result<()> {
    println!("Got {} members to check", members.len());
    println!("Got {} tickoff to check", tickoff.len());
    'outer: for member in members.iter() {
        // println!("Checking member {member}");

        for tick in tickoff.iter() {
            let name = tick.name.split(",").next().unwrap();
            // println!("Name {}", name);
            if member.surname == name {
                // println!("Found {}", member.surname);
                continue 'outer;
            }
        }
        println!("Cannot find member {} in tickoff list", member);
    }
    Ok(())
}

pub type TickOffList = Vec<TickOffItem>;

pub fn tick_off_list(tickoff_file: &str) -> Result<TickOffList> {
    let mut excel: Xlsx<_> = open_workbook(tickoff_file).unwrap();
    let mut tick_off_list = vec![];
    // let mut jokers = Vec::new();
    if let Ok(r) = excel.worksheet_range("PER") {
        for row in r.rows().skip(7).take(100) {
            // println!("Big: {} {} Small: {} {}", row[0], row[1], row[5], row[6],);
            // if let Data::DateTime(date) = row[0] {
            //     println!("{}", NaiveDate::from(date.as_datetime().unwrap()));
            // }
            let name_big = &row[0];
            let amount_big = &row[1];
            // Gerlingen
            // let name_small = &row[5];
            // let amount_small = &row[6];
            // Perouse
            let name_small = &row[4];
            let amount_small = &row[5];
            let item_big = TickOffItem::new(name_big, Some(amount_big), None);
            let item_small =
                TickOffItem::new(name_small, None, Some(amount_small));
            if let Err(_) = item_big {
                if let Err(_) = item_small {
                    println!("Items exhausted");
                    break;
                }
            }
            if let Ok(item) = item_big {
                tick_off_list.push(item);
            }
            if let Ok(item) = item_small {
                tick_off_list.push(item);
            }
            // let joker = Joker::new(
            //     &date, &name, &forename, warning, &location, big, small, line,
            // )?;
            // jokers.push(joker);
            // line += 1;
        }
    }
    Ok(tick_off_list)
}
