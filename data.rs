use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use tempfile::NamedTempFile;

// Define a struct to represent each record in the dataset
pub struct Record {
    pub data_as_of: String,
    pub start_date: String,
    pub end_date: String,
    pub group: String,
    pub state: String,
    pub condition_group: String,
    pub condition: String,
    pub icd10_codes: String,
    pub age_group: String,
    pub covid_deaths: u32,
    pub mentions: u32,
}

// Define the number of fields expected in each line of the CSV data
const NUM_FIELDS: usize = 11;

// Function to load data from a CSV file into a vector of Record structs
pub fn load_data(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() != NUM_FIELDS {
            println!("Warning: Skipping line {}: Invalid number of fields", index + 1);
            continue;
            }

        let record = Record {
            data_as_of: fields[0].to_string(),
            start_date: fields[1].to_string(),
            end_date: fields[2].to_string(),
            group: fields[3].to_string(),
            state: fields[4].to_string(),
            condition_group: fields[5].to_string(),
            condition: fields[6].to_string(),
            icd10_codes: fields[7].to_string(),
            age_group: fields[8].to_string(),
            covid_deaths: fields[9].parse().unwrap_or(0),
            mentions: fields[10].parse().unwrap_or(0),
        };

        records.push(record);
    }
    Ok(records)
}
