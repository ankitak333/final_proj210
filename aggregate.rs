use std::collections::HashMap;
use crate::data::Record;

/// Aggregates COVID-19 deaths by State and Condition.
pub fn aggregate_data(records: Vec<Record>) -> HashMap<(String, String), u32> {
    let mut aggregation = HashMap::new();

    for record in records {
        let key = (record.state, record.condition);
        let entry = aggregation.entry(key).or_insert(0);
        *entry += record.covid_deaths;
    }

    aggregation
}

/// Example function to display aggregated data (could be used for debugging or output purposes).
pub fn display_aggregated_data(aggregated_data: &HashMap<(String, String), u32>) {
    for ((state, condition), deaths) in aggregated_data {
        println!("State: {}, Condition: {}, COVID-19 Deaths: {}", state, condition, deaths);
    }
}
