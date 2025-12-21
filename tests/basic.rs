use somato::somato;

#[test]
fn basic_load_synth() -> Result<(), anyhow::Error> {
    let members_file = "tests/test_data/members_synthetic.xlsx";
    let joker_file = "tests/test_data/jokers_synthetic.xlsx";
    let tickoff_file = "tests/test_data/tickoff_synthetic.xlsx";
    let date = "2025-12-19";
    somato::somato_runner(members_file, joker_file, tickoff_file, date)
}

#[ignore]
#[test]
fn basic_load_real() {
    let members_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        08_Mitgliederliste/\
        2023-03-20_Mitgliederliste-Solawi-Heckengaeu_v3_Test_neu_fixed.xlsx";
    let joker_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        Daten_Stand_2025.11.27/\
        Joker_Solawi-Heckengaeu.xlsx";
    let tickoff_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
        Daten_Stand_2025.11.27/\
        2024-10-28_Abhaklisten.xlsx";
    let date = "2025-12-19";

    let result =
        somato::somato_runner(members_file, joker_file, tickoff_file, date);
    println!("{:?}", result);
    assert!(result.is_ok());
}
