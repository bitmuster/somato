use crate::somato::member;
use ::somato::tickoff::get_amount_big;
use anyhow::Result;
use somato::Location;
use somato::joker;
use somato::somato;
use somato::tickoff;

fn get_config_synth() -> somato::Config {
    somato::Config {
        members: "tests/test_data/members_synthetic.xlsx".to_string(),
        jokers: "tests/test_data/jokers_synthetic.xlsx".to_string(),
        tickoff: "tests/test_data/tickoff_synthetic.xlsx".to_string(),
        date: "2025-12-19".to_string(),
    }
}

#[test]
fn basic_load_synth() -> Result<(), anyhow::Error> {
    let config = get_config_synth();
    somato::somato_runner(&config)
}

#[test]
fn basic_read_members() -> Result<(), anyhow::Error> {
    let members_count = 93;
    let config = get_config_synth();
    let members = member::read_members(&config.members)?;
    assert_eq!(members.len(), members_count);
    // make sure the first is there
    let m = members
        .iter()
        .filter(|m| m.forename == "Donald" && m.surname == "Duck");
    assert_eq!(m.count(), 1);
    // make sure the last is there
    let m = members
        .iter()
        .filter(|m| m.forename == "Borstinger" && m.surname == "Borstinger");
    assert_eq!(m.count(), 1);
    Ok(())
}

#[test]
fn basic_read_jokers() -> Result<(), anyhow::Error> {
    let jokers_count = 15;
    let config = get_config_synth();
    let jokers = joker::read_jokers(&config.jokers)?;
    assert_eq!(jokers.len(), jokers_count);
    Ok(())
}

#[test]
fn basic_read_tickoff_count() -> Result<(), anyhow::Error> {
    let to_count_per = 12;
    let to_count_ren = 10;
    let to_count_ger = 10;
    let to_count_leo = 12;
    let to_count_wds = 11;
    let to_count_neu = 11;
    let config = get_config_synth();
    let to = tickoff::tick_off_list(&config.tickoff, &Location::Perouse)?;
    assert_eq!(to.len(), to_count_per);
    let to = tickoff::tick_off_list(&config.tickoff, &Location::Renningen)?;
    assert_eq!(to.len(), to_count_ren);
    let to = tickoff::tick_off_list(&config.tickoff, &Location::Gerlingen)?;
    assert_eq!(to.len(), to_count_ger);
    let to = tickoff::tick_off_list(&config.tickoff, &Location::Leonberg)?;
    assert_eq!(to.len(), to_count_leo);
    let to = tickoff::tick_off_list(&config.tickoff, &Location::WeilDerStadt)?;
    assert_eq!(to.len(), to_count_wds);
    let to = tickoff::tick_off_list(&config.tickoff, &Location::Neuhausen)?;
    assert_eq!(to.len(), to_count_neu);
    Ok(())
}

#[test]
fn basic_read_tickoff_count_dense() -> Result<(), anyhow::Error> {
    let to_count = [
        (Location::Perouse, 12),
        (Location::Renningen, 10),
        (Location::Gerlingen, 10),
        (Location::Leonberg, 12),
        (Location::WeilDerStadt, 11),
        (Location::Neuhausen, 11),
    ];
    let config = get_config_synth();
    for toi in to_count.iter() {
        let to = tickoff::tick_off_list(&config.tickoff, &toi.0)?;
        assert_eq!(to.len(), toi.1);
    }
    Ok(())
}

#[test]
fn basic_read_tickoff_counts() -> Result<(), anyhow::Error> {
    let to_count = [
        (Location::Perouse, 7, 8),
        (Location::Renningen, 6, 5),
        (Location::Gerlingen, 5, 6),
        (Location::Leonberg, 6, 9),
        (Location::WeilDerStadt, 8, 5),
        (Location::Neuhausen, 6, 5),
    ];
    let config = get_config_synth();
    for toi in to_count.iter() {
        println!("{:?}", toi.0);
        let to = tickoff::tick_off_list(&config.tickoff, &toi.0)?;
        assert_eq!(tickoff::get_amount_big(&to), toi.1, "big fail");
        assert_eq!(tickoff::get_amount_small(&to), toi.2, "small fail");
    }
    Ok(())
}

#[ignore]
#[test]
fn basic_load_real() {
    let config = somato::Config {
        members: "/home/micha/Repos/SolawiKommisionierSpielplatz/\
            08_Mitgliederliste/\
            2023-03-20_Mitgliederliste-Solawi-Heckengaeu_v3_Test_neu_fixed.xlsx"
            .to_string(),
        jokers: "/home/micha/Repos/SolawiKommisionierSpielplatz/\
            Daten_Stand_2025.11.27/\
            Joker_Solawi-Heckengaeu.xlsx"
            .to_string(),
        tickoff: "/home/micha/Repos/SolawiKommisionierSpielplatz/\
            Daten_Stand_2025.11.27/\
            2024-10-28_Abhaklisten.xlsx"
            .to_string(),
        date: "2025-12-19".to_string(),
    };
    let result = somato::somato_runner(&config);
    println!("{:?}", result);
    assert!(result.is_ok());
}
