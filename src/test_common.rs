#[cfg(test)]
pub mod test_common {
    use crate::joker::Joker;
    use crate::location::Location;
    use crate::member::Member;
    use crate::tickoff::TickOffItem;
    use chrono::NaiveDate;

    pub fn gen_toi_ok() -> [TickOffItem; 3] {
        let a = TickOffItem {
            name: "Testerin, A.".to_string(),
            big: 2,
            small: 0,
        };
        let b = TickOffItem {
            name: "Tester, B.".to_string(),
            big: 0,
            small: 2,
        };
        let c = TickOffItem {
            name: "Testeress, C.".to_string(),
            big: 3,
            small: 3,
        };
        [a, b, c]
    }

    pub fn gen_toi_fail() -> [TickOffItem; 4] {
        let a = TickOffItem {
            name: "Testerin, A.".to_string(),
            big: 2,
            small: 0,
        };
        let a_small = TickOffItem {
            name: "testerin, a.".to_string(),
            big: 2,
            small: 0,
        };
        let b = TickOffItem {
            name: "Tester, B.".to_string(),
            big: 0,
            small: 2,
        };
        let c = TickOffItem {
            name: "Fail".to_string(),
            big: 2,
            small: 4,
        };
        [a, a_small, b, c]
    }

    pub fn gen_members() -> [Member; 3] {
        let a = Member::new_from_values(
            "EV-1",
            1,
            "Testerin",
            "Alice",
            2,
            0,
            Location::Perouse,
            false,
        );
        let b = Member::new_from_values(
            "EV-2",
            2,
            "Tester",
            "Bob",
            0,
            2,
            Location::Perouse,
            false,
        );
        let c = Member::new_from_values(
            "EV-3",
            3,
            "Testeress",
            "Cloe",
            3,
            3,
            Location::Perouse,
            false,
        );
        [a, b, c]
    }

    pub fn gen_joker_a() -> Joker {
        let j = Joker {
            date: NaiveDate::from_ymd_opt(1, 1, 1).unwrap(),
            surname: "Testerin".to_string(),
            forename: "Alice".to_string(),
            warning: 0,
            location: Location::Perouse,
            big: 2,
            small: 0,
            line: 88,
        };
        j
    }
    pub fn gen_joker_b() -> Joker {
        let j = Joker {
            date: NaiveDate::from_ymd_opt(1, 1, 1).unwrap(),
            surname: "Tester".to_string(),
            forename: "Bob".to_string(),
            warning: 0,
            location: Location::Perouse,
            big: 0,
            small: 2,
            line: 88,
        };
        j
    }
    pub fn gen_joker_c() -> Joker {
        let j = Joker {
            date: NaiveDate::from_ymd_opt(1, 1, 1).unwrap(),
            surname: "Testeress".to_string(),
            forename: "Cloe".to_string(),
            warning: 0,
            location: Location::Perouse,
            big: 3,
            small: 3,
            line: 88,
        };
        j
    }
}
