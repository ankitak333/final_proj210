
#[cfg(test)]
mod tests {
    use super::super::data::load_data;
    use tempfile::NamedTempFile;
    use std::io::Write;


    #[test]
    fn test_load_data_valid_file() {
        // Create a temporary CSV file with valid data for testing
        let csv_data = "\
            Data As Of,Start Date,End Date,Group,State,Condition Group,Condition,ICD10_codes,Age Group,COVID-19 Deaths,Number of Mentions\n\
            2022-05-01,2022-04-25,2022-05-01,Month,California,Respiratory diseases,Influenza,ICD-10 code 1,18-29 years,10,20\n\
            2022-05-01,2022-04-25,2022-05-01,Month,California,Respiratory diseases,Pneumonia,ICD-10 code 2,30-39 years,15,25\n";

        // Write the CSV data to a temporary file
        let temp_dir = tempdir::TempDir::new("test_data").expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("test.csv");
        std::fs::write(&file_path, csv_data).expect("Failed to write temp file");

        // Call load_data function to load data from the temporary file
        let result = load_data(&file_path.to_string_lossy());
        
        // Check if the result is Ok
        assert!(result.is_ok());

        // Check if the loaded records match the expected records
        let records = result.unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].state, "California");
        assert_eq!(records[0].condition, "Influenza");
        assert_eq!(records[0].covid_deaths, 10);
        assert_eq!(records[1].age_group, "30-39 years");
        assert_eq!(records[1].mentions, 25);
    }

    #[test]
    fn test_load_data_empty_file() {
        let mut file = NamedTempFile::new().unwrap();
        // Write an empty string to the tempfile
        write!(file, "").unwrap();
        let file_path = file.path().to_str().unwrap();

        let result = load_data(file_path);

        assert!(result.is_ok());
        let records = result.unwrap();
        assert!(records.is_empty());
    }
}