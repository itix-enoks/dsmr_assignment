use tudelft_dsmr_output_generator::Graphs;
use dsmr_assignment::error::MainError;
use tudelft_dsmr_output_generator::voltage_over_time::{create_voltage_over_time_graph, VoltageData};
use tudelft_dsmr_output_generator::current_over_time::{CurrentOverTime, CurrentData};
use dsmr_assignment::telegram::{TelegramContent, TelegramData, Value};
use std::io::Read;
// use tudelft_dsmr_output_generator::voltage_over_time::{
//     create_voltage_over_time_graph, VoltageData,
// };
// use tudelft_dsmr_output_generator::Graphs;

/// Contains `MainError`, and code to convert `PlotError` and `io::Error` into a `MainError`
// Package declaration moved to lib.rs
// // mod error;

// Moved my parser to different module for own sake.
use dsmr_assignment::parser::parse;

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

use tudelft_dsmr_output_generator::GraphBuilder;
fn main() -> Result<(), MainError> {
    let input = read_from_stdin()?;

    let telegrams = parse(&input)?;

    let voltages: Vec<VoltageData> =
        telegrams
        .iter()
        .filter_map(|t| {
            let timestamp = match &t.base.date.value {
                Some(Value::Date(date)) => date.timestamp,
                _ => panic!("Invalid timestamp")
            };
            match &t.data {
                TelegramData::Electricity { voltages, .. }  =>
                    match voltages {
                        [TelegramContent { value: Some(Value::Float(p1)), .. },
                         TelegramContent { value: Some(Value::Float(p2)), .. },
                         TelegramContent { value: Some(Value::Float(p3)), .. }] =>
                            Some(VoltageData {
                                timestamp: timestamp,
                                phase_1: *p1,
                                phase_2: *p2,
                                phase_3: *p3
                            }),
                        _ => Option::None
                    },
                _ => Option::None
            }
        })
        .collect();

    let currents: Vec<CurrentData> =
        telegrams
        .iter()
        .filter_map(|t| {
            let timestamp = match &t.base.date.value {
                Some(Value::Date(date)) => date.timestamp,
                _ => panic!("Invalid timestamp")
            };
            match &t.data {
                TelegramData::Electricity { currents, .. }  =>
                    match currents {
                        [TelegramContent { value: Some(Value::Float(p1)), .. },
                         TelegramContent { value: Some(Value::Float(p2)), .. },
                         TelegramContent { value: Some(Value::Float(p3)), .. }] =>
                            Some(CurrentData {
                                timestamp: timestamp,
                                phase_1: *p1,
                                phase_2: *p2,
                                phase_3: *p3
                            }),
                        _ => Option::None
                    },
                _ => Option::None
            }
        })
        .collect();

    let mut current_over_time = CurrentOverTime::new();
    for c in currents {
        current_over_time.add(c);
    }

    let mut result = Graphs::new()?;

    result.add_graph(create_voltage_over_time_graph(voltages))?;
    result.add_graph(current_over_time)?;

    let _ = result.generate();

    Ok(())
}
