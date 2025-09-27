use std::collections::HashMap;
use std::io::Read;

use crate::bail;
use itertools::Itertools;

use tudelft_dsmr_output_generator::current_over_time::{CurrentData, CurrentOverTime};
use tudelft_dsmr_output_generator::energy_over_time::{EnergyData, EnergyOverTime};
use tudelft_dsmr_output_generator::gas_over_time::{GasData, GasOverTime};
use tudelft_dsmr_output_generator::voltage_over_time::VoltageData;
use tudelft_dsmr_output_generator::{GraphBuilder, Graphs, UnixTimeStamp};

use crate::error::MainError;
use crate::telegram::*;

pub fn process_voltages(telegrams: &[Telegram]) -> Vec<VoltageData> {
    let mut processed_map: HashMap<UnixTimeStamp, (f64, f64, f64)> = HashMap::new();
    telegrams
        .iter()
        .filter_map(|t| {
            let timestamp = match &t.base.date.value {
                Some(Value::Date(date)) => date.timestamp,
                _ => bail!("Invalid timestamp"),
            };
            match &t.data {
                TelegramData::Electricity { voltages, .. } => match voltages {
                    [TelegramContent {
                        value: Some(Value::Float(p1)),
                        ..
                    }, TelegramContent {
                        value: Some(Value::Float(p2)),
                        ..
                    }, TelegramContent {
                        value: Some(Value::Float(p3)),
                        ..
                    }] => {
                        if !processed_map.contains_key(&timestamp) {
                            processed_map.insert(timestamp, (*p1, *p2, *p3));
                            Some(VoltageData {
                                timestamp: timestamp,
                                phase_1: *p1,
                                phase_2: *p2,
                                phase_3: *p3,
                            })
                        } else {
                            let mut new_value = processed_map
                                .get(&timestamp)
                                .unwrap_or_else(|| bail!("Could not unpack old_value"))
                                .clone();
                            if *p1 > new_value.0 {
                                new_value.0 = *p1;
                            }
                            if *p2 > new_value.1 {
                                new_value.1 = *p2;
                            }
                            if *p3 > new_value.2 {
                                new_value.2 = *p3;
                            }
                            processed_map.insert(timestamp, new_value);
                            Some(VoltageData {
                                timestamp: timestamp,
                                phase_1: new_value.0,
                                phase_2: new_value.1,
                                phase_3: new_value.2,
                            })
                        }
                    }
                    _ => Option::None,
                },
                _ => Option::None,
            }
        })
        .collect()
}

pub fn process_currents(telegrams: &[Telegram]) -> CurrentOverTime {
    let mut processed_map: HashMap<UnixTimeStamp, (f64, f64, f64)> = HashMap::new();
    let currents: Vec<CurrentData> = telegrams
        .iter()
        .filter_map(|t| {
            let timestamp = match &t.base.date.value {
                Some(Value::Date(date)) => date.timestamp,
                _ => bail!("Invalid timestamp"),
            };
            match &t.data {
                TelegramData::Electricity { currents, .. } => match currents {
                    [TelegramContent {
                        value: Some(Value::Float(p1)),
                        ..
                    }, TelegramContent {
                        value: Some(Value::Float(p2)),
                        ..
                    }, TelegramContent {
                        value: Some(Value::Float(p3)),
                        ..
                    }] => {
                        if !processed_map.contains_key(&timestamp) {
                            processed_map.insert(timestamp, (*p1, *p2, *p3));
                            Some(CurrentData {
                                timestamp: timestamp,
                                phase_1: *p1,
                                phase_2: *p2,
                                phase_3: *p3,
                            })
                        } else {
                            let mut new_value = processed_map
                                .get(&timestamp)
                                .unwrap_or_else(|| bail!("Could not unpack old_value"))
                                .clone();
                            if *p1 > new_value.0 {
                                new_value.0 = *p1;
                            }
                            if *p2 > new_value.1 {
                                new_value.1 = *p2;
                            }
                            if *p3 > new_value.2 {
                                new_value.2 = *p3;
                            }
                            processed_map.insert(timestamp, new_value);
                            Some(CurrentData {
                                timestamp: timestamp,
                                phase_1: new_value.0,
                                phase_2: new_value.1,
                                phase_3: new_value.2,
                            })
                        }
                    }
                    _ => Option::None,
                },
                _ => Option::None,
            }
        })
        .collect();

    let mut current_over_time = CurrentOverTime::new();
    for c in currents {
        current_over_time.add(c);
    }
    current_over_time
}

pub fn process_gas_data(telegrams: &[Telegram]) -> GasOverTime {
    let mut processed_map: HashMap<UnixTimeStamp, f64> = HashMap::new();
    let gas_pairs: Vec<(UnixTimeStamp, f64)> = telegrams
        .iter()
        .filter_map(|t| {
            let timestamp = match &t.base.date.value {
                Some(Value::Date(date)) => date.timestamp,
                _ => bail!("Invalid timestamp"),
            };
            match &t.data {
                TelegramData::Gas {
                    total_gas_delivered,
                } => match total_gas_delivered {
                    TelegramContent {
                        value: Some(Value::Float(gas)),
                        ..
                    } => {
                        if !processed_map.contains_key(&timestamp) {
                            processed_map.insert(timestamp, *gas);
                            Some((timestamp, *gas))
                        } else {
                            let new_value = processed_map
                                .get(&timestamp)
                                .unwrap_or_else(|| bail!("Could not unpack old_value"))
                                .clone();
                            if new_value > *gas {
                                processed_map.insert(timestamp, new_value);
                                Some((timestamp, new_value))
                            } else {
                                processed_map.insert(timestamp, *gas);
                                Some((timestamp, *gas))
                            }
                        }
                    }
                    _ => Option::None,
                },
                _ => Option::None,
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
            gas_delta: *current_gas - gas_pairs[idx - 1].1,
        });
    }
    gas_delta_over_time
}

pub fn process_energy_data(telegrams: &[Telegram]) -> EnergyOverTime {
    let mut processed_map: HashMap<UnixTimeStamp, (f64, f64)> = HashMap::new();
    telegrams.iter().for_each(|t| {
        let timestamp = match &t.base.date.value {
            Some(Value::Date(date)) => date.timestamp,
            _ => bail!("Invalid timestamp"),
        };
        match &t.data {
            TelegramData::Electricity {
                total_consumed,
                total_produced,
                ..
            } => match (total_consumed, total_produced) {
                (
                    TelegramContent {
                        value: Some(Value::Float(consumed)),
                        ..
                    },
                    TelegramContent {
                        value: Some(Value::Float(produced)),
                        ..
                    },
                ) => {
                    if !processed_map.contains_key(&timestamp) {
                        processed_map.insert(timestamp, (*consumed, *produced));
                    } else {
                        let old_value = processed_map
                            .get(&timestamp)
                            .unwrap_or_else(|| bail!("Could not unpack old_value"))
                            .clone();
                        processed_map.insert(
                            timestamp,
                            (old_value.0 + *consumed, old_value.1 + *produced),
                        );
                    }
                }
                _ => (),
            },
            _ => (),
        };
    });

    let mut energy_pair_delta_over_time: EnergyOverTime = EnergyOverTime::new();
    let mut energy_pair_vector: Vec<EnergyData> = Vec::new();
    let total_pairs: Vec<(UnixTimeStamp, (f64, f64))> =
        processed_map.into_iter().sorted_by_key(|p| p.0).collect();
    for (idx, (timestamp, (consumed, produced))) in total_pairs.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        energy_pair_vector.push(EnergyData {
            timestamp: *timestamp,
            consumed: *consumed - total_pairs[idx - 1].1 .0,
            produced: *produced - total_pairs[idx - 1].1 .1,
        });
    }
    for _ in 0..12 {
        if let Some(e) = energy_pair_vector.pop() {
            energy_pair_delta_over_time.add(e);
        } else {
            break;
        }
    }
    energy_pair_delta_over_time
}

pub fn process_event_logs(telegrams: &[Telegram], result: &mut Graphs) {
    telegrams.iter().for_each(|t| {
        for (id, _) in &t.base.eventlog_dates {
            let (id, severity) = &t
                .base
                .eventlog_severities
                .iter()
                .find(|x| x.0 == *id)
                .unwrap_or_else(|| bail!("Eventlog misses date"));
            let (_, message) = &t
                .base
                .eventlog_messages
                .iter()
                .find(|x| x.0 == *id)
                .unwrap_or_else(|| bail!("Eventlog misses message"));
            match message {
                TelegramContent {
                    value: Some(Value::String(message)),
                    ..
                } => {
                    match severity {
                        TelegramContent {
                            value: Some(Value::String(severity)),
                            ..
                        } => {
                            if *severity == "H".to_string() {
                                result.add_high_severity_event_log_message(decode_message(message));
                            } else if *severity == "L".to_string() {
                                result.add_low_severity_event_log_message(decode_message(message));
                            } else {
                                bail!("Unknown severity value")
                            }
                        }
                        _ => bail!("Invalid severity found"),
                    };
                }
                _ => bail!("Invalid message found"),
            }
        }
    });
}

pub fn read_from_stdin() -> Result<String, MainError> {
    let mut input = Vec::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut input)?;
    Ok(String::from_utf8_lossy(&input).to_string())
}

pub fn decode_message(message: &String) -> String {
    let mut result = "".to_string();
    let mut chars = message.chars();
    while let Some(ch) = chars.next() {
        let x = ch
            .to_digit(16)
            .unwrap_or_else(|| bail!("Invalid character in message"));
        if let Some(y) = chars.next() {
            let y = y
                .to_digit(16)
                .unwrap_or_else(|| bail!("Invalid character in message"));
            let ascii = char::from_u32(16 * x + y).unwrap_or_else(|| bail!("Invalid ASCII value"));
            result.push(ascii);
        } else {
            bail!("Unaligned block found")
        }
    }

    result
}
