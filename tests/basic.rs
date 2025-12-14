use somajotr::somajotr;

#[test]
fn basic_load() {
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
