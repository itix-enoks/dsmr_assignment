use std::io::Read;
use std::collections::HashSet;

use tudelft_dsmr_output_generator::{Graphs, GraphBuilder, UnixTimeStamp};
use tudelft_dsmr_output_generator::voltage_over_time::{create_voltage_over_time_graph, VoltageData};
use tudelft_dsmr_output_generator::current_over_time::{CurrentOverTime, CurrentData};
use tudelft_dsmr_output_generator::energy_over_time::{EnergyOverTime, EnergyData};
use tudelft_dsmr_output_generator::gas_over_time::{GasOverTime, GasData};

use dsmr_assignment::telegram::{TelegramContent, TelegramData, Value};
use dsmr_assignment::error::MainError;
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

fn main() -> Result<(), MainError> {
    let input = read_from_stdin()?;

    let mut telegrams = parse(&input)?;
    telegrams.sort_by_key(|t|
                          match &t.base.date.value {
                              Some(Value::Date(date)) => date.timestamp,
                              _ => panic!("Invalid timestamp")
                          }
    );

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

    let gas_pairs: Vec<(UnixTimeStamp, f64)> =
        telegrams
        .iter()
        .filter_map(|t| {
            let timestamp = match &t.base.date.value {
                Some(Value::Date(date)) => date.timestamp,
                _ => panic!("Invalid timestamp")
            };
            match &t.data {
                TelegramData::Gas { total_gas_delivered }  =>
                    match total_gas_delivered {
                        TelegramContent { value: Some(Value::Float(gas)), .. } =>
                            Some((timestamp, *gas)),
                        _ => Option::None
                    },
                _ => Option::None
            }
        })
        .collect();
    let mut gas_delta_over_time: GasOverTime = GasOverTime::new();
    for (idx, (timestamp, current_gas)) in gas_pairs.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        gas_delta_over_time.add(GasData {
            timestamp: *timestamp,
            gas_delta: *current_gas - gas_pairs[idx - 1].1 // Total gas stashed decreases
        });
    }

    let total_pairs: Vec<(UnixTimeStamp, (f64, f64))> =
        telegrams
        .iter()
        .filter_map(|t| {
            let timestamp = match &t.base.date.value {
                Some(Value::Date(date)) => date.timestamp,
                _ => panic!("Invalid timestamp")
            };
            match &t.data {
                TelegramData::Electricity { total_consumed, total_produced, .. }  =>
                    match (total_consumed, total_produced) {
                        (TelegramContent { value: Some(Value::Float(consumed)), .. },
                         TelegramContent { value: Some(Value::Float(produced)), .. }) =>
                            Some((timestamp, (*consumed, *produced))),
                        _ => Option::None
                    },
                _ => Option::None
            }
        })
        .collect();

    /// Duplicate timestamps in dsmr file messes up the energy_over_time plot, it does not seem to
    ///  mess up the other plots though (visually at least: but I may need to check them later).
    let mut processed_timestamps: HashSet<UnixTimeStamp> = HashSet::new();
    let mut energy_pair_delta_over_time: EnergyOverTime = EnergyOverTime::new();
    let mut energy_pair_vector: Vec<EnergyData> = Vec::new();
    for (idx, (timestamp, (consumed, produced))) in total_pairs.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        if processed_timestamps.contains(timestamp) {
            continue;
        }
        energy_pair_vector.push(EnergyData {
            timestamp: *timestamp,
            consumed: *consumed - total_pairs[idx - 1].1.0,
            produced: *produced - total_pairs[idx - 1].1.1
        });
        processed_timestamps.insert(*timestamp);
    }
    for _ in 0..12 {
        if let Some(e) = energy_pair_vector.pop() {
            // The order of the data points is reversed here, but in the plot module this is sorted
            // correctly by timestamp again
            energy_pair_delta_over_time.add(e);
        } else {
            break;
        }
    }

    let mut result = Graphs::new()?;
    result.add_graph(create_voltage_over_time_graph(voltages))?;
    result.add_graph(current_over_time)?;
    result.add_graph(gas_delta_over_time)?;
    result.add_graph(energy_pair_delta_over_time)?;
    let _ = result.generate();

    Ok(())
}
