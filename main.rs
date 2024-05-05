mod data; // Ensure `data` module is correctly referenced
mod aggregate; // Include the new aggregate module
mod graph; // Include the graph module for generating charts
mod data_test; // Testing module, make sure tests are updated if necessary

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/ankitakumar/graph_analysis/Conditions_Contributing_to_COVID-19_Deaths__by_State_and_Age__Provisional_2020-2023.csv";
    match data::load_data(file_path) {
        Ok(records) => {
            println!("Loaded {} records", records.len());

            // Aggregate data
            let aggregated_data = aggregate::aggregate_data(records);
            aggregate::display_aggregated_data(&aggregated_data);

            // Ensure the output directory exists
            std::fs::create_dir_all("output")?;

            // Generate and save the graph
            let graph_result = graph::generate_bar_chart(&aggregated_data, "output/bar_chart.png");
            match graph_result {
                Ok(_) => println!("Graph generated successfully."),
                Err(e) => println!("Failed to generate graph: {}", e),
            }

            Ok(())
        },
        Err(err) => {
            eprintln!("Error loading data: {}", err);
            Err(err)
        }
    }
}