//! Used in tests and benchmarks;

/// Loads price data from trades from a csv file.
///
/// # Panics:
/// When the data is invalid.
/// SHOULD ONLY BE USED IN A TEST CONTEXT.
pub fn load_prices_from_csv(filename: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    use std::fs::File;

    let f = File::open(filename)?;

    let mut r = csv::Reader::from_reader(f);

    Ok(Vec::from_iter(r.records().map(|record| {
        let row = record.expect("The record is valid; qed");
        row[1].parse::<f32>().expect("Must be able to parse price")
    })))
}
