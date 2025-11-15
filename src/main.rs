/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine

*/
use calamine::{Reader, Xlsx, open_workbook};

fn read() {
    let joker_file = "/home/micha/Repos/SolawiKommisionierSpielplatz/Joker_Solawi-Heckengaeu.xlsx";
    let mut excel: Xlsx<_> = open_workbook(joker_file).unwrap();

    if let Ok(r) = excel.worksheet_range("Eingabe") {
        for row in r.rows() {
            // println!("row={:?}, row[0]={:?}", row, row[0]);
            println!(
                "{} {} {} {} {} {} {} {}",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]
            );
        }
    }
}

fn main() {
    println!("Hello, world!");
    read();
}
