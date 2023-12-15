use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
// Define a struct to hold CSV record data
pub struct Record {
    pub source: i32,
    pub target: i32,
    pub rating: i32,
}

// Function to read CSV data from a file and parse it into a vector of `Record` structs
pub fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(file_path)?;
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}
