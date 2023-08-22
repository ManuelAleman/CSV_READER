use std::error::Error;
use std::f32::consts::E;
use std::io;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(e) => eprintln!("Error reading record: {}", e),
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./customers.csv") {
        eprintln!("{}", e);
    }
}
