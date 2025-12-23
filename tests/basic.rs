use somato::somato;

#[test]
fn basic_load_synth() -> Result<(), anyhow::Error> {
    let config = somato::Config {
        members: "tests/test_data/members_synthetic.xlsx".to_string(),
        jokers: "tests/test_data/jokers_synthetic.xlsx".to_string(),
        tickoff: "tests/test_data/tickoff_synthetic.xlsx".to_string(),
        date: "2025-12-19".to_string(),
    };
    somato::somato_runner(&config)
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
