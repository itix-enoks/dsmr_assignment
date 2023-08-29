use error::MainError;
use std::io::Read;
use tudelft_dsmr_output_generator::voltage_over_time::{
    create_voltage_over_time_graph, VoltageData,
};
use tudelft_dsmr_output_generator::Graphs;

/// Contains `MainError`, and code to convert `PlotError` and `io::Error` into a `MainError`
mod error;

fn parse(_input: &str) -> Result<(), MainError> {
    // Note that you can use this function:
    // tudelft_dsmr_output_generator::date_to_timestamp()

    todo!()
}

/// Reads the DSMR file from the terminal.
/// You do not need to change this nor understand this.
/// You can use
/// ```
/// cargo run < examples/good/simple_gas.dsmr
/// ```
/// to quickly test an example dsmr file with your submission.
/// We also use this at the end to assist with grading your submission!
fn read_from_stdin() -> Result<String, MainError> {
    let mut input = Vec::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut input)?;
    Ok(String::from_utf8_lossy(&input).to_string())
}

fn main() -> Result<(), MainError> {
    let input = read_from_stdin()?;

    let _parsed = parse(&input)?;

    let mut result = Graphs::new()?;

    result.add_graph(create_voltage_over_time_graph(vec![
        VoltageData {
            phase_1: 100.0,
            phase_2: 200.0,
            phase_3: 300.0,
            timestamp: 100,
        },
        VoltageData {
            phase_1: 200.0,
            phase_2: 300.0,
            phase_3: 250.0,
            timestamp: 10000,
        },
    ]))?;

    Ok(())
}
