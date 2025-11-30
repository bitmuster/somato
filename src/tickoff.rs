// use crate::location::Location;
use anyhow::Result;
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
// use chrono::NaiveDate;

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
    pub fn new(name: &Data, big: Option<&Data>, small: Option<&Data>) -> Self {
        // println!("Creating new entry with {:?} {:?} {:?}", name, big, small);
        let name = name
            .as_string()
            .unwrap_or(format!("Error while parsing \"{:?}\"", name));
        let big = match big {
            Some(i) => i.as_i64().unwrap_or(88) as u32,
            None => 0,
        };
        let small = match small {
            Some(i) => i.as_i64().unwrap_or(88) as u32,
            None => 0,
        };
        let item = TickOffItem { name, big, small };
        println!(
            "Creating new entry with {:?} {:?} {:?}",
            item.name, item.big, item.small
        );
        item
    }
}

// pub type TickOffList = Vec<TickOffItem>;

pub fn tick_off_list(tickoff_file: &str) -> Result<()> {
    let mut excel: Xlsx<_> = open_workbook(tickoff_file).unwrap();

    // let mut jokers = Vec::new();
    if let Ok(r) = excel.worksheet_range("GER") {
        // let mut line = 2;
        for row in r.rows().skip(7).take(13) {
            // println!("Big: {} {} Small: {} {}", row[0], row[1], row[5], row[6],);
            // if let Data::DateTime(date) = row[0] {
            //     println!("{}", NaiveDate::from(date.as_datetime().unwrap()));
            // }
            let name_big = &row[0];
            let amount_big = &row[1];
            let name_small = &row[5];
            let amount_small = &row[6];
            let _item = TickOffItem::new(name_big, Some(amount_big), None);
            let _item = TickOffItem::new(name_small, None, Some(amount_small));
            // let joker = Joker::new(
            //     &date, &name, &forename, warning, &location, big, small, line,
            // )?;
            // jokers.push(joker);
            // line += 1;
        }
    }
    Ok(())
}
