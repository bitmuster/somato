// use crate::location::Location;
use anyhow::{Result, anyhow};
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
// use chrono::NaiveDate;
use crate::location::Location;
use crate::member;
use chrono;
use colorama::Colored;

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
            .ok_or_else(|| return anyhow!("Cannot parse name \"{}\"", name));
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
        // println!(
        //     "Creating new entry with {:?} {:?} {:?}",
        //     item.name, item.big, item.small
        // );
        // println!("{item:?}");
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
        println!(
            "{}",
            format!("Cannot find member \"{}\" in tickoff list", member)
                .color("red")
        );
    }
    Ok(())
}

pub type TickOffList = Vec<TickOffItem>;

pub fn tick_off_list(
    tickoff_file: &str,
    location: &Location,
) -> Result<TickOffList> {
    let mut excel: Xlsx<_> = open_workbook(tickoff_file).map_err(|e| {
        anyhow!(format!(
            "Error {e} while loading tickoff file {tickoff_file}"
        ))
    })?;
    let mut tick_off_list = vec![];

    // Historic reasos
    let offset = match location {
        Location::Perouse => 0,
        _ => 1,
    };
    let mut sum_big: u32 = 0;
    let mut sum_small: u32 = 0;
    if let Ok(r) = excel.worksheet_range(location.to_short()) {
        for row in r.rows().skip(7).take(100) {
            // println!(
            //     "Big: \"{}\" {}\" Small: {}\" {}\"",
            //     row[0],
            //     row[1],
            //     row[5 + offset],
            //     row[6 + offset],
            // );
            if let Data::DateTime(date) = row[0] {
                println!(
                    "{}",
                    chrono::NaiveDate::from(date.as_datetime().unwrap())
                );
            }
            let (mut big_done, mut small_done) = (false, false);
            let name_big = &row[0];
            let amount_big = &row[1];

            let name_small = &row[4 + offset];
            let amount_small = &row[5 + offset];
            let item_big = TickOffItem::new(name_big, Some(amount_big), None);
            let item_small =
                TickOffItem::new(name_small, None, Some(amount_small));

            if let Ok(item) = item_big {
                tick_off_list.push(item);
            } else {
                match amount_big.as_i64() {
                    Some(x) => sum_big = x as u32,
                    None => {}
                };
                // println!("Error while parsing big: {item_big:?}");
                big_done = true;
            }
            if let Ok(item) = item_small {
                tick_off_list.push(item);
            } else {
                match amount_small.as_i64() {
                    Some(x) => sum_small = x as u32,
                    None => {}
                };
                // println!("Error while parsing small: {item_small:?}");
                small_done = true;
            }
            if big_done && small_done {
                // println!("Items exhausted");
                // println!("Parsed {amount_big:?} big amount");
                // println!("Parsed {amount_small:?} small amount");
                break;
            }
            // let joker = Joker::new(
            //     &date, &name, &forename, warning, &location, big, small, line,
            // )?;
            // jokers.push(joker);
            // line += 1;
        }
    }
    let all_big: u32 = tick_off_list
        .iter()
        .filter(|x| x.big > 0)
        .map(|x| x.big)
        .sum();
    let all_small: u32 = tick_off_list
        .iter()
        .filter(|x| x.small > 0)
        .map(|x| x.small)
        .sum();
    println!("Parsed {sum_big} big amount");
    println!("Parsed {sum_small} small amount");
    assert_eq!(
        sum_big, all_big,
        "Amount for big in tickoff list does not match"
    );
    assert_eq!(
        sum_small, all_small,
        "Amount for small in tickoff list does not match"
    );
    Ok(tick_off_list)
}
