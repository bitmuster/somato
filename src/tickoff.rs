// use crate::location::Location;
use crate::location::Location;
use crate::member;
use anyhow::{Result, anyhow};
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};
use chrono;
use colored::Colorize;
use lazy_regex;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    /// Try to generate a new Tick Off Item from parsed data.
    pub fn try_new(
        name: &Data,
        big: Option<&Data>,
        small: Option<&Data>,
    ) -> Result<Self> {
        // println!("Creating new entry with {:?} {:?} {:?}", name, big, small);
        let name = name
            .as_string()
            .ok_or_else(|| anyhow!("Cannot parse name \"{}\"", name));
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

/// Check if the given name is well formated
/// Allowed : "Surname, N."
/// TODO: This will fail for chars with all kinds of apostophs and forenames that
/// start with umlauts.
pub fn check_name_with_initial(name: &str) -> bool {
    let re = lazy_regex::regex!(r"^[A-ZÄÖÜa-zäöü\- ]*, [[:alpha:]].$");
    let rematch = re.is_match(name);
    if !rematch {
        println!("{}", format!("    Malformed name \"{}\"", name).red());
    };
    rematch
}

/// Split the given name into surname and first part of forename
/// TODO: This will fail for chars with all kinds of apostophs and forenames that
/// start with umlauts.
pub fn split_name(name: &str) -> Option<(&str, &str)> {
    let re = lazy_regex::regex!(r"^([A-ZÄÖÜa-zäöü\- ]*), ([[:alpha:]]).$");
    let (_, [surname, initial]) =
        re.captures(name).map(|caps| caps.extract())?;

    Some((surname, initial))
}

/// Check names given as surname, forename for equality with the initial.
/// E.g. "Surname, N."
/// Warning this check is not exhaustive as there could be multiple forenames
/// that begin with the same character.
pub fn check_name_equality(
    surname: &str,
    forename: &str,
    name_with_initial: &str,
) -> bool {
    if !check_name_with_initial(name_with_initial) {
        return false;
    }
    let Some((name, initial)) = split_name(name_with_initial) else {
        return false;
    };
    if surname.to_lowercase() == name.to_lowercase()
        && forename
            .chars()
            .next()
            .unwrap()
            .eq_ignore_ascii_case(&initial.chars().next().unwrap())
    {
        return true;
    }
    false
}

/// Checks if all members are mentioned in the tickoff list.
/// If they are not this is an idication of a joker.
/// Number of warnings is returned with an Option<usize>
pub fn check_for_members_in_tickoff_list(
    members: &member::MemberList,
    tickoff: &TickOffList,
) -> Result<Option<usize>> {
    println!("Checking tickoff list for missig members.");
    println!(
        "  Got {} members and {} tickoff to check",
        members.len(),
        tickoff.len()
    );
    let mut warnings = None;
    let tickoffset: HashSet<TickOffItem> = HashSet::from_iter(tickoff.clone());

    'outer: for member in members.iter() {
        // println!("Checking member {member}");

        for tick in tickoffset.iter() {
            if !check_name_with_initial(&tick.name) {
                warnings = match warnings {
                    Some(w) => Some(w + 1),
                    None => Some(1),
                };
                continue;
            }
            if check_name_equality(
                &member.surname,
                &member.forename,
                &tick.name,
            ) {
                // println!("Found {}", member.surname);
                continue 'outer;
            }
        }
        println!(
            "{}",
            format!(
                "    Cannot find member \"{}\" in tickoff list. Joker?",
                member
            )
            .blue()
        );
        warnings = match warnings {
            Some(w) => Some(w + 1),
            None => Some(1),
        };
    }
    Ok(warnings)
}

/// Checks if all members are mentioned in the tickoff list.
/// Number of warnings is returned with an Option<usize>
pub fn check_tickoff_list_against_members(
    members: &member::MemberList,
    tickoff: &TickOffList,
) -> Result<Option<usize>> {
    println!("Checking members for missig enries in tickoff list.");
    println!(
        "  Got {} members and {} tickoff to check",
        members.len(),
        tickoff.len()
    );
    let mut warnings = None;

    'outer: for tick in tickoff.iter() {
        for member in members.iter() {
            // println!("Checking member {member}");

            if !check_name_with_initial(&tick.name) {
                warnings = match warnings {
                    Some(w) => Some(w + 1),
                    None => Some(1),
                };
                // Name is malformed - skip furhter analysis
                continue;
            }
            if check_name_equality(
                &member.surname,
                &member.forename,
                &tick.name,
            ) {
                // println!("Found {}", member.surname);
                if member.big != tick.big {
                    warnings = match warnings {
                        Some(w) => Some(w + 1),
                        None => Some(1),
                    };
                    println!(
                        "{}",
                        format!(
                            "    Tickoff size for big {} does not match: {} {}",
                            member.surname, member.big, tick.big
                        )
                        .red()
                    );
                    // warnings += 1;
                }
                if member.small != tick.small {
                    warnings = match warnings {
                        Some(w) => Some(w + 1),
                        None => Some(1),
                    };
                    println!(
                        "{}",
                        format!(
                            "    Tickoff size for small {} does not match: {} {}",
                            member.surname, member.big, tick.small
                        )
                        .red()
                    );
                    // warnings += 1;
                }
                continue 'outer;
            }
        }
        println!(
            "{}",
            format!("    Cannot find item \"{}\" in member list", tick.name)
                .red()
        );
        warnings = match warnings {
            Some(w) => Some(w + 1),
            None => Some(1),
        };
    }
    Ok(warnings)
}

/// Container type for the TickOffList
pub type TickOffList = Vec<TickOffItem>;

/// Parse tickoff list from filename and location
pub fn tick_off_list(
    tickoff_file: &str,
    location: &Location,
) -> Result<TickOffList> {
    println!("Parsing tickoff list");
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

            // Try to parse_items
            let item_big =
                TickOffItem::try_new(name_big, Some(amount_big), None);
            let item_small =
                TickOffItem::try_new(name_small, None, Some(amount_small));

            // Check if we reached the end of the list
            if let Ok(item) = item_big {
                tick_off_list.push(item);
            } else {
                if let Some(s) = amount_big.as_i64() {
                    sum_big = s as u32
                };
                // println!("Error while parsing big: {item_big:?}");
                big_done = true;
            }

            // Check if we reached the end of the list
            if let Ok(item) = item_small {
                tick_off_list.push(item);
            } else {
                if let Some(s) = amount_small.as_i64() {
                    sum_small = s as u32
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
    let all_big = get_amount_big(&tick_off_list);
    let all_small = get_amount_small(&tick_off_list);
    println!("  Parsed {sum_big} big amount");
    println!("  Parsed {sum_small} small amount");
    // assert_eq!(
    //     sum_big, all_big,
    //     "Amount for big in tickoff list does not match"
    // );
    // assert_eq!(
    //     sum_small, all_small,
    //     "Amount for small in tickoff list does not match"
    // );
    if sum_big != all_big {
        println!(
            "{}",
            format!(
                "    Amount for big in tickoff list does not match {} vs. {}",
                sum_big, all_big,
            )
            .red()
        );
    }
    if sum_small != all_small {
        println!(
            "{}",
            format!(
                "    Amount for small in tickoff list does not match {} vs. {}",
                sum_small, all_small,
            )
            .red()
        );
    }
    Ok(tick_off_list)
}

/// Helper function to get the amout of big collectors
pub fn get_amount_big(tick_off_list: &[TickOffItem]) -> u32 {
    tick_off_list
        .iter()
        .filter(|x| x.big > 0)
        .map(|x| x.big)
        .sum::<u32>()
}

/// Helper function to get the amout of big collectors
pub fn get_amount_small(tick_off_list: &[TickOffItem]) -> u32 {
    tick_off_list
        .iter()
        .filter(|x| x.small > 0)
        .map(|x| x.small)
        .sum::<u32>()
}

#[cfg(test)]
mod tickoff_tests {

    use super::*;
    use crate::test_common::test_common::*;

    #[test]
    fn test_new() {
        let _ = TickOffItem::try_new(
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
    fn test_get_big_amount() {
        let toi = gen_toi_ok();
        assert_eq!(get_amount_big(&toi), 5);
    }

    #[test]
    fn test_get_small_amount() {
        let toi = gen_toi_ok();
        assert_eq!(get_amount_small(&toi), 5);
    }

    #[test]
    fn test_check_for_members_in_tickoff_list() {
        let [a, a_small, b, c] = gen_toi_fail();
        let [m, n, _o] = gen_members();

        let r = check_for_members_in_tickoff_list(
            &vec![m.clone(), n.clone()],
            &vec![a.clone(), b.clone()],
        );
        assert!(r.is_ok(), "Base test");
        assert_eq!(r.unwrap(), None);

        let r = check_for_members_in_tickoff_list(
            &vec![m.clone(), n.clone()],
            &vec![a_small.clone(), b.clone()],
        );
        assert!(r.is_ok(), "Error in small caps");
        assert_eq!(r.unwrap(), None);

        // One entry missing
        let r = check_for_members_in_tickoff_list(
            &vec![m.clone(), n.clone()],
            &vec![a.clone()],
        );
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), Some(1));

        // Second entry missing
        let r = check_for_members_in_tickoff_list(
            &vec![m.clone(), n.clone()],
            &vec![b.clone()],
        );
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), Some(1));

        // Empty tickoff
        let r = check_for_members_in_tickoff_list(
            &vec![m.clone(), n.clone()],
            &vec![],
        );
        assert_eq!(r.unwrap(), Some(2));

        // Invalid entry
        let r = check_for_members_in_tickoff_list(
            &vec![m.clone(), n.clone()],
            &vec![c.clone()],
        );
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), Some(4));

        // Additional tickoffs cannot be detected for now
        // let r = check_for_members_in_tickoff_list(
        //     &vec![m.clone(), n.clone()],
        //     &vec![a.clone(), b.clone(), c.clone()],
        // );
        // assert!(r.is_err());
    }
    #[test]
    fn test_check_tickoff_list_against_members() {
        let [a, _a_small, b, _c] = gen_toi_fail();
        let [m, n, _c] = gen_members();

        let r = check_tickoff_list_against_members(
            &vec![m.clone(), n.clone()],
            &vec![a.clone(), b.clone()],
        );
        assert!(r.is_ok(), "Base test");
        assert_eq!(r.unwrap(), None);

        let r = check_tickoff_list_against_members(
            &vec![m.clone()],
            &vec![a.clone(), b.clone()],
        );
        assert!(r.is_ok(), "One missing");
        assert_eq!(r.unwrap(), Some(1));

        let r = check_tickoff_list_against_members(
            &vec![],
            &vec![a.clone(), b.clone()],
        );
        assert!(r.is_ok(), "Two missing");
        assert_eq!(r.unwrap(), Some(2));
    }

    #[test]
    fn test_check_tickoff_list_wrong_size() {
        let [mut a, mut b, _c] = gen_toi_ok();
        let [m, n, _c] = gen_members();
        a.big = 999;
        let r = check_tickoff_list_against_members(
            &vec![m.clone(), n.clone()],
            &vec![a.clone(), b.clone()],
        );
        assert!(r.is_ok(), "Base test");
        assert_eq!(r.unwrap(), Some(1));

        a.big = 99;
        a.small = 32;
        b.big = 42;
        b.small = 11;
        let r = check_tickoff_list_against_members(
            &vec![m.clone(), n.clone()],
            &vec![a.clone(), b.clone()],
        );
        assert!(r.is_ok(), "Base test");
        assert_eq!(r.unwrap(), Some(4));
    }

    #[test]
    fn test_check_initial() {
        assert!(check_name_with_initial("Smith, J."));
        assert!(check_name_with_initial("van der Smith, J."));
        assert!(check_name_with_initial("van der smith, j."));
        // assert!(check_name_with_initial("ÄÖÜäöü, ä."));
        assert!(check_name_with_initial("ÄÖÜäöü, t."));

        assert!(!check_name_with_initial("van der Smith, J.."));
        assert!(!check_name_with_initial("van der Smith, J"));
        assert!(!check_name_with_initial("Smith"));
    }

    #[test]
    fn test_split_name() {
        assert_eq!(split_name("Smith, J."), Some(("Smith", "J")));

        assert_eq!(split_name("Smith, t."), Some(("Smith", "t")));
        assert_eq!(split_name("Smith"), None);
        assert_eq!(split_name("Smith J."), None);
        assert_eq!(split_name("Smith,, J"), None);
    }

    #[test]
    fn test_name_equality() {
        assert!(check_name_equality("Smith", "John", "Smith, J."));
        assert!(check_name_equality("Smith", "Bob", "Smith, B."));
        assert!(check_name_equality("Über", "Börkan", "Über, B."));
        assert!(check_name_equality(
            "von Über-Flieger",
            "Börkan",
            "von Über-Flieger, B."
        ));

        assert!(!check_name_equality("Smith", "John", "Smith"));
        assert!(!check_name_equality("Smith", "Bob", "Smith, J."));
        assert!(!check_name_equality("Smith", "Bob", "Smith J."));
    }
}
