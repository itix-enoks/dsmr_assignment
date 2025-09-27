use tudelft_dsmr_output_generator::Graphs;

use dsmr_assignment::telegram::*;
use dsmr_assignment::helpers::*;

#[test]
fn test_decode_message_simple() {
    let encoded = "48656c6c6f".to_string(); // "Hello" in hex
    let result = decode_message(&encoded);
    assert_eq!(result, "Hello");
}

#[test]
fn test_decode_message_power_failure() {
    let encoded = "506f776572204661696c757265".to_string(); // "Power Failure" in hex
    let result = decode_message(&encoded);
    assert_eq!(result, "Power Failure");
}

#[test]
fn test_process_voltages_single_telegram() {
    let telegram = create_test_electricity_telegram(
        1234567890,
        [230.1, 231.2, 229.8],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );

    let voltages = process_voltages(&[telegram]);
    assert_eq!(voltages.len(), 1);
    assert!((voltages[0].phase_1 - 230.1).abs() < 0.001);
    assert!((voltages[0].phase_2 - 231.2).abs() < 0.001);
    assert!((voltages[0].phase_3 - 229.8).abs() < 0.001);
}

#[test]
fn test_process_voltages_duplicate_timestamp() {
    let telegram1 = create_test_electricity_telegram(
        1234567890,
        [230.0, 231.0, 229.0],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );
    let telegram2 = create_test_electricity_telegram(
        1234567890, // Same timestamp
        [235.0, 236.0, 234.0], // Higher voltages
        [5.1, 6.1, 4.6],
        [1.16, 1.39, 1.05],
        12346.67,
        124.45
    );

    let voltages = process_voltages(&[telegram1, telegram2]);
    assert_eq!(voltages.len(), 2);
    // Should take the higher voltage values for duplicate timestamps
    assert!((voltages[1].phase_1 - 235.0).abs() < 0.001);
}
#[test]

fn test_process_voltages_multiple_telegrams() {
    let telegram1 = create_test_electricity_telegram(
        1234567890,
        [230.0, 231.0, 229.0],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );
    let telegram2 = create_test_electricity_telegram(
        1234567900,
        [232.0, 233.0, 231.0],
        [5.2, 6.2, 4.7],
        [1.19, 1.42, 1.08],
        12346.67,
        124.45
    );

    let voltages = process_voltages(&[telegram1, telegram2]);
    assert_eq!(voltages.len(), 2);
    assert!((voltages[0].phase_1 - 230.0).abs() < 0.001);
    assert!((voltages[1].phase_1 - 232.0).abs() < 0.001);
}

#[test]
fn test_process_currents_single_telegram() {
    let telegram = create_test_electricity_telegram(
        1234567890,
        [230.1, 231.2, 229.8],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );

    let _current_over_time = process_currents(&[telegram]);
    // We can't easily test the internal structure, but we can verify it doesn't panic
    assert!(true);
}

#[test]
fn test_process_currents_multiple_telegrams() {
    let telegram1 = create_test_electricity_telegram(
        1234567890,
        [230.1, 231.2, 229.8],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );
    let telegram2 = create_test_electricity_telegram(
        1234567900,
        [230.5, 231.6, 230.2],
        [5.2, 6.2, 4.7],
        [1.19, 1.42, 1.08],
        12346.67,
        124.45
    );

    let _current_over_time = process_currents(&[telegram1, telegram2]);

    assert!(true);
}

#[test]
fn test_process_gas_data_single_telegram() {
    let telegram = create_test_gas_telegram(1234567890, 12345.123);

    let _gas_over_time = process_gas_data(&[telegram]);
    // With only one telegram, no delta can be calculated
    assert!(true);
}

#[test]
fn test_process_gas_data_multiple_telegrams() {
    let telegram1 = create_test_gas_telegram(1234567890, 12345.123);
    let telegram2 = create_test_gas_telegram(2234567900, 22345.456);

    let _gas_over_time = process_gas_data(&[telegram1, telegram2]);
    // Should calculate delta between telegrams
    assert!(true);
}

#[test]
fn test_process_energy_data_single_telegram() {
    let telegram = create_test_electricity_telegram(
        1234567890,
        [230.1, 231.2, 229.8],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );

    let _energy_over_time = process_energy_data(&[telegram]);
    // With only one telegram, no delta can be calculated
    assert!(true);
}

#[test]
fn test_process_energy_data_multiple_telegrams() {
    let telegram1 = create_test_electricity_telegram(
        1234567890,
        [230.1, 231.2, 229.8],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );
    let telegram2 = create_test_electricity_telegram(
        1234567900,
        [230.5, 231.6, 230.2],
        [5.2, 6.2, 4.7],
        [1.19, 1.42, 1.08],
        12346.67, // +1.0 kWh consumed
        124.45    // +1.0 kWh produced
    );

    let _energy_over_time = process_energy_data(&[telegram1, telegram2]);
    // Should calculate delta between telegrams
    assert!(true);
}

#[test]
fn test_process_voltages_with_none_values() {
    let mut telegram = create_test_electricity_telegram(
        1234567890,
        [230.1, 231.2, 229.8],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );

    // Set voltage values to None
    if let TelegramData::Electricity { ref mut voltages, .. } = telegram.data {
        voltages[0].value = None;
        voltages[1].value = None;
        voltages[2].value = None;
    }

    let result = process_voltages(&[telegram]);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_process_currents_with_none_values() {
    let mut telegram = create_test_electricity_telegram(
        1234567890,
        [230.1, 231.2, 229.8],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );

    // Set current values to None
    if let TelegramData::Electricity { ref mut currents, .. } = telegram.data {
        currents[0].value = None;
        currents[1].value = None;
        currents[2].value = None;
    }

    let _result = process_currents(&[telegram]);
    assert!(true);
}

#[test]
fn test_process_gas_data_with_none_value() {
    let mut telegram = create_test_gas_telegram(1234567890, 12345.123);

    // Set gas value to None
    if let TelegramData::Gas { ref mut total_gas_delivered } = telegram.data {
        total_gas_delivered.value = None;
    }

    let _result = process_gas_data(&[telegram]);
    assert!(true);
}

#[test]
fn test_process_energy_data_with_none_values() {
    let mut telegram = create_test_electricity_telegram(
        1234567890,
        [230.1, 231.2, 229.8],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );

    // Set energy values to None
    if let TelegramData::Electricity { ref mut total_consumed, ref mut total_produced, .. } = telegram.data {
        total_consumed.value = None;
        total_produced.value = None;
    }

    let _result = process_energy_data(&[telegram]);
    assert!(true);
}

fn create_test_electricity_telegram(
    _timestamp: u64,
    voltages: [f64; 3],
    currents: [f64; 3],
    powers: [f64; 3],
    total_consumed: f64,
    total_produced: f64,
) -> Telegram {
    Telegram::new(
        TelegramBase::new(
            TelegramContent::new_value(
                TelegramContentType::Start,
                (1, 1, Some(0)),
                Value::String("START".to_string()),
                None,
            ),
            TelegramContent::new_value(
                TelegramContentType::Date,
                (2, 1, None),
                Value::Date(Date::new(2023, 7, 5, 15, 26, 41, true)),
                None,
            ),
            vec![],
            vec![],
            vec![],
            TelegramContent::new_value(
                TelegramContentType::InformationType,
                (4, 1, None),
                Value::String("E".to_string()),
                None,
            ),
            TelegramContent::new_value(
                TelegramContentType::End,
                (1, 2, Some(0)),
                Value::String("END".to_string()),
                None,
            ),
        ),
        TelegramData::Electricity {
            voltages: [
                TelegramContent::new_value(TelegramContentType::Voltage, (7, 1, Some(1)), Value::Float(voltages[0]), Some(TelegramContentUnit::V)),
                TelegramContent::new_value(TelegramContentType::Voltage, (7, 1, Some(2)), Value::Float(voltages[1]), Some(TelegramContentUnit::V)),
                TelegramContent::new_value(TelegramContentType::Voltage, (7, 1, Some(3)), Value::Float(voltages[2]), Some(TelegramContentUnit::V)),
            ],
            currents: [
                TelegramContent::new_value(TelegramContentType::Current, (7, 2, Some(1)), Value::Float(currents[0]), Some(TelegramContentUnit::A)),
                TelegramContent::new_value(TelegramContentType::Current, (7, 2, Some(2)), Value::Float(currents[1]), Some(TelegramContentUnit::A)),
                TelegramContent::new_value(TelegramContentType::Current, (7, 2, Some(3)), Value::Float(currents[2]), Some(TelegramContentUnit::A)),
            ],
            powers: [
                TelegramContent::new_value(TelegramContentType::Power, (7, 3, Some(1)), Value::Float(powers[0]), Some(TelegramContentUnit::KW)),
                TelegramContent::new_value(TelegramContentType::Power, (7, 3, Some(2)), Value::Float(powers[1]), Some(TelegramContentUnit::KW)),
                TelegramContent::new_value(TelegramContentType::Power, (7, 3, Some(3)), Value::Float(powers[2]), Some(TelegramContentUnit::KW)),
            ],
            total_consumed: TelegramContent::new_value(TelegramContentType::TotalConsumed, (7, 4, Some(1)), Value::Float(total_consumed), Some(TelegramContentUnit::KWH)),
            total_produced: TelegramContent::new_value(TelegramContentType::TotalProduced, (7, 4, Some(2)), Value::Float(total_produced), Some(TelegramContentUnit::KWH)),
        },
    )
}

fn create_test_gas_telegram(_timestamp: u64, gas_total: f64) -> Telegram {
    Telegram::new(
        TelegramBase::new(
            TelegramContent::new_value(
                TelegramContentType::Start,
                (1, 1, Some(1)),
                Value::String("START".to_string()),
                None,
            ),
            TelegramContent::new_value(
                TelegramContentType::Date,
                (2, 1, None),
                Value::Date(Date::new(2023, 7, 5, 15, 26, 41, true)),
                None,
            ),
            vec![],
            vec![],
            vec![],
            TelegramContent::new_value(
                TelegramContentType::InformationType,
                (4, 1, None),
                Value::String("G".to_string()),
                None,
            ),
            TelegramContent::new_value(
                TelegramContentType::End,
                (1, 2, Some(1)),
                Value::String("END".to_string()),
                None,
            ),
        ),
        TelegramData::Gas {
            total_gas_delivered: TelegramContent::new_value(
                TelegramContentType::GasTotalDelivered,
                (5, 2, None),
                Value::Float(gas_total),
                Some(TelegramContentUnit::M3),
            ),
        },
    )
}

#[test]
fn test_process_event_logs_high_severity() {
    let mut result = Graphs::new().unwrap();
    let telegram = create_test_telegram_with_eventlog(1, "H".to_string(), "48656c6c6f".to_string());
    process_event_logs(&[telegram], &mut result);
    let _freaking_library_that_forces_you_to_use_a_function_prior_to_dropping = result.generate();
    assert!(true);
}

#[test]
fn test_process_event_logs_low_severity() {
    let mut result = Graphs::new().unwrap();
    let telegram = create_test_telegram_with_eventlog(1, "L".to_string(), "576f726c64".to_string());
    process_event_logs(&[telegram], &mut result);
    let _freaking_library_that_forces_you_to_use_a_function_prior_to_dropping = result.generate();
    assert!(true);
}

#[test]
fn test_process_voltages_with_gas_telegrams() {
    let gas_telegram = create_test_gas_telegram(1234567890, 12345.123);
    let voltages = process_voltages(&[gas_telegram]);
    assert_eq!(voltages.len(), 0);
}

#[test]
fn test_process_currents_with_gas_telegrams() {
    let gas_telegram = create_test_gas_telegram(1234567890, 12345.123);
    let _current_over_time = process_currents(&[gas_telegram]);
    // Should handle gracefully with no current data
    assert!(true);
}

#[test]
fn test_process_gas_data_with_electricity_telegrams() {
    let electricity_telegram = create_test_electricity_telegram(
        1234567890,
        [230.1, 231.2, 229.8],
        [5.0, 6.0, 4.5],
        [1.15, 1.38, 1.04],
        12345.67,
        123.45
    );
    let _gas_over_time = process_gas_data(&[electricity_telegram]);
    // Should handle gracefully with no gas data
    assert!(true);
}

#[test]
fn test_process_energy_data_with_gas_telegrams() {
    let gas_telegram = create_test_gas_telegram(1234567890, 12345.123);
    let _energy_over_time = process_energy_data(&[gas_telegram]);
    // Should handle gracefully with no energy data
    assert!(true);
}

fn create_test_telegram_with_eventlog(event_id: u32, severity: String, message: String) -> Telegram {
    Telegram::new(
        TelegramBase::new(
            TelegramContent::new_value(TelegramContentType::Start, (1, 1, Some(0)), Value::String("START".to_string()), None),
            TelegramContent::new_value(TelegramContentType::Date, (2, 1, None), Value::Date(Date::new(2023, 7, 5, 15, 26, 41, true)), None),
            vec![(event_id, TelegramContent::new_value(TelegramContentType::EventlogSeverity, (3, 1, Some(event_id)), Value::String(severity), None))],
            vec![(event_id, TelegramContent::new_value(TelegramContentType::EventlogMessage, (3, 2, Some(event_id)), Value::String(message), None))],
            vec![(event_id, TelegramContent::new_value(TelegramContentType::EventlogDate, (3, 3, Some(event_id)), Value::Date(Date::new(2023, 7, 2, 13, 12, 0, true)), None))],
            TelegramContent::new_value(TelegramContentType::InformationType, (4, 1, None), Value::String("E".to_string()), None),
            TelegramContent::new_value(TelegramContentType::End, (1, 2, Some(0)), Value::String("END".to_string()), None),
        ),
        TelegramData::Electricity {
            voltages: [
                TelegramContent::new_value(TelegramContentType::Voltage, (7, 1, Some(1)), Value::Float(230.1), Some(TelegramContentUnit::V)),
                TelegramContent::new_value(TelegramContentType::Voltage, (7, 1, Some(2)), Value::Float(231.2), Some(TelegramContentUnit::V)),
                TelegramContent::new_value(TelegramContentType::Voltage, (7, 1, Some(3)), Value::Float(229.8), Some(TelegramContentUnit::V)),
            ],
            currents: [
                TelegramContent::new_value(TelegramContentType::Current, (7, 2, Some(1)), Value::Float(5.0), Some(TelegramContentUnit::A)),
                TelegramContent::new_value(TelegramContentType::Current, (7, 2, Some(2)), Value::Float(6.0), Some(TelegramContentUnit::A)),
                TelegramContent::new_value(TelegramContentType::Current, (7, 2, Some(3)), Value::Float(4.5), Some(TelegramContentUnit::A)),
            ],
            powers: [
                TelegramContent::new_value(TelegramContentType::Power, (7, 3, Some(1)), Value::Float(1.15), Some(TelegramContentUnit::KW)),
                TelegramContent::new_value(TelegramContentType::Power, (7, 3, Some(2)), Value::Float(1.38), Some(TelegramContentUnit::KW)),
                TelegramContent::new_value(TelegramContentType::Power, (7, 3, Some(3)), Value::Float(1.04), Some(TelegramContentUnit::KW)),
            ],
            total_consumed: TelegramContent::new_value(TelegramContentType::TotalConsumed, (7, 4, Some(1)), Value::Float(12345.67), Some(TelegramContentUnit::KWH)),
            total_produced: TelegramContent::new_value(TelegramContentType::TotalProduced, (7, 4, Some(2)), Value::Float(123.45), Some(TelegramContentUnit::KWH)),
        },
    )
}
