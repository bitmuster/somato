/*

https://github.com/tafia/calamine
https://crates.io/crates/calamine
https://docs.rs/calamine/latest/calamine/

*/
use anyhow::Result;

mod joker;
mod location;
mod member;
mod somajotr;

fn main() -> Result<()> {
    somajotr::somajotr()?;
    Ok(())
}
