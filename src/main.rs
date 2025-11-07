use std::io::Error;

use brute_force::data::{get_data_path, read_file};

fn main() -> Result<(), Error> {
    // Example: adjust filename in data/raw/
    let p = get_data_path("btcusdt_8h_columnar.json", "raw");
    println!("{p:?}");
    if p.exists() {
    } else {
    }
    Ok(())
}
