/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine
https://docs.rs/calamine/latest/calamine/

*/

use anyhow::Result;
use somato::somato;
use std::path;

fn main() -> Result<()> {
    println!("{}", "*".repeat(80));
    let members_file = "tests/test_data/members_synthetic.xlsx";
    let joker_file = "tests/test_data/jokers_synthetic.xlsx";
    let tickoff_file = "tests/test_data/tickoff_synthetic.xlsx";

    somato::somato_runner(members_file, joker_file, tickoff_file)?;

    println!("{}", "*".repeat(80));
    let base_folder = path::Path::new(
        "/home/micha/Repos/SolawiKommisionierSpielplatz/Daten_Stand_20251217",
    );
    let members_file = base_folder
        .join("2023-12-17_Mitgliederliste-Solawi-Heckengaeu_v3_Test2.xlsx");
    let joker_file = base_folder.join("Joker_Solawi-Heckengaeu.xlsx");
    let tickoff_file = base_folder.join("2024-10-28_Abhaklisten.xlsx");

    somato::somato_runner(
        members_file.to_str().unwrap(),
        joker_file.to_str().unwrap(),
        tickoff_file.to_str().unwrap(),
    )?;

    println!("{}", "*".repeat(80));
    Ok(())
}
