use somajotr::somajotr;

#[test]
fn basic_load_synth() {
    let members_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
    Entenhausen/\
    members_synthetic.xlsx";
    let joker_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/\
    Entenhausen/\
    jokers_synthetic.xlsx";
    let tickoff_file = "";

    let result = somajotr::somajotr(members_file, joker_file, tickoff_file);
    assert!(result.is_ok());
}

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

    let result = somajotr::somajotr(members_file, joker_file, tickoff_file);
    assert!(result.is_ok());
}
