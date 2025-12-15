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

/// Checks if all members are mentioned in the tickoff list
/// TODO: Check name format "Name, N." with regex
pub fn check_lists(
    members: &member::MemberList,
    tickoff: &TickOffList,
) -> Result<()> {
    println!("Got {} members to check", members.len());
    println!("Got {} tickoff to check", tickoff.len());
    'outer: for member in members.iter() {
        // println!("Checking member {member}");

        for tick in tickoff.iter() {
            let mut split = tick.name.split(",");
            let name = split.next().unwrap();
            let initials = split.next().unwrap(); // should be " T."
            // println!("{name} {initials}");
            // println!("Name {}", name);
            if member.surname == name {
                if member.forename.chars().next().unwrap()
                    == initials.chars().skip(1).next().unwrap()
                {
                    // println!("Found {}", member.surname);
                    continue 'outer;
                }
            }
        }
        println!(
            "{}",
            format!("Cannot find member \"{}\" in tickoff list", member)
                .color("red")
        );
        return Err(anyhow!(format!(
            "Cannot find member \"{}\" in tickoff list",
            member
        )));
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

#[cfg(test)]
mod tickoff_tests {

    use super::*;
    use crate::member::Member;

    #[test]
    fn test_new() {
        let _ = TickOffItem::new(
            &Data::String("Test".to_string()),
            Some(&Data::Int(5)),
            Some(&Data::Int(6)),
        );
        let _t = TickOffItem {
            name: "Test".to_string(),
            big: 2,
            small: 3,
        };
    }
    #[test]
    fn test_check_lists() {
        let a = TickOffItem {
            name: "Test, A.".to_string(),
            big: 2,
            small: 3,
        };
        let b = TickOffItem {
            name: "Test, B.".to_string(),
            big: 2,
            small: 3,
        };
        let c = TickOffItem {
            name: "Fail".to_string(),
            big: 2,
            small: 3,
        };
        let m = Member::new_from_values(
            "EV-1",
            1,
            "Test",
            "Alice",
            1,
            1,
            Location::Perouse,
            false,
        );
        let n = Member::new_from_values(
            "EV-2",
            1,
            "Test",
            "Bob",
            1,
            1,
            Location::Perouse,
            false,
        );

        let r = check_lists(
            &vec![m.clone(), n.clone()],
            &vec![a.clone(), b.clone()],
        );
        assert!(r.is_ok());

        // One entry missing
        let r = check_lists(&vec![m.clone(), n.clone()], &vec![a.clone()]);
        assert!(r.is_err());

        // Second entry missing
        let r = check_lists(&vec![m.clone(), n.clone()], &vec![b.clone()]);
        assert!(r.is_err());

        // Empty tickoff
        let r = check_lists(&vec![m.clone(), n.clone()], &vec![]);
        assert!(r.is_err());

        // Additional tickoffs cannot be detected for now
        // let r = check_lists(
        //     &vec![m.clone(), n.clone()],
        //     &vec![a.clone(), b.clone(), c.clone()],
        // );
        // assert!(r.is_err());
    }
}
