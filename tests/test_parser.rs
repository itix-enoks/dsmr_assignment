use dsmr_assignment::parser::*;
use dsmr_assignment::telegram::*;
use std::fs;

#[test]
fn test_parse_reorder() {
    let input =
        fs::read_to_string("examples/good/reorder.dsmr").expect("Failed to read reorder.dsmr file");

    let expected = vec![Telegram::new(
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
            vec![(
                1,
                TelegramContent::new_value(
                    TelegramContentType::EventlogSeverity,
                    (3, 1, Some(1)),
                    Value::String("H".to_string()),
                    None,
                ),
            )],
            vec![(
                1,
                TelegramContent::new_value(
                    TelegramContentType::EventlogMessage,
                    (3, 2, Some(1)),
                    Value::String("506f776572204661696c757265".to_string()),
                    None,
                ),
            )],
            vec![(
                1,
                TelegramContent::new_value(
                    TelegramContentType::EventlogDate,
                    (3, 3, Some(1)),
                    Value::Date(Date::new(2023, 7, 2, 13, 12, 0, true)),
                    None,
                ),
            )],
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
                TelegramContent::new_value(
                    TelegramContentType::Voltage,
                    (7, 1, Some(1)),
                    Value::Float(241.7),
                    Some(TelegramContentUnit::V),
                ),
                TelegramContent::new_value(
                    TelegramContentType::Voltage,
                    (7, 1, Some(2)),
                    Value::Float(240.6),
                    Some(TelegramContentUnit::V),
                ),
                TelegramContent::new_value(
                    TelegramContentType::Voltage,
                    (7, 1, Some(3)),
                    Value::Float(241.92),
                    Some(TelegramContentUnit::V),
                ),
            ],
            currents: [
                TelegramContent::new_value(
                    TelegramContentType::Current,
                    (7, 2, Some(1)),
                    Value::Float(1.0),
                    Some(TelegramContentUnit::A),
                ),
                TelegramContent::new_value(
                    TelegramContentType::Current,
                    (7, 2, Some(2)),
                    Value::Float(10.0),
                    Some(TelegramContentUnit::A),
                ),
                TelegramContent::new_value(
                    TelegramContentType::Current,
                    (7, 2, Some(3)),
                    Value::Float(0.5),
                    Some(TelegramContentUnit::A),
                ),
            ],
            powers: [
                TelegramContent::new_value(
                    TelegramContentType::Power,
                    (7, 3, Some(1)),
                    Value::Float(1.00),
                    Some(TelegramContentUnit::KW),
                ),
                TelegramContent::new_value(
                    TelegramContentType::Power,
                    (7, 3, Some(2)),
                    Value::Float(-5.010),
                    Some(TelegramContentUnit::KW),
                ),
                TelegramContent::new_value(
                    TelegramContentType::Power,
                    (7, 3, Some(3)),
                    Value::Float(2.500),
                    Some(TelegramContentUnit::KW),
                ),
            ],
            total_consumed: TelegramContent::new_value(
                TelegramContentType::TotalConsumed,
                (7, 4, Some(1)),
                Value::Float(11454892.0),
                Some(TelegramContentUnit::KWH),
            ),
            total_produced: TelegramContent::new_value(
                TelegramContentType::TotalProduced,
                (7, 4, Some(2)),
                Value::Float(1245.0),
                Some(TelegramContentUnit::KWH),
            ),
        },
    )];

    let parsed = parse(&input).expect("Failed to parse reorder example");
    assert_eq!(parsed.len(), expected.len());

    assert_eq!(
        parsed[0].base.start.telegram_content_type,
        expected[0].base.start.telegram_content_type
    );
    match (&parsed[0].data, &expected[0].data) {
        (TelegramData::Electricity { .. }, TelegramData::Electricity { .. }) => {
            // Electricity data matched, the '..' rest operator ignores all the fields
        }
        _ => panic!("Data type mismatch"),
    }
}

#[test]
fn test_parse_simple_gas() {
    let input = fs::read_to_string("examples/good/simple_gas.dsmr")
        .expect("Failed to read simple_gas.dsmr file");

    let expected = vec![
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
                vec![(
                    1,
                    TelegramContent::new_value(
                        TelegramContentType::EventlogSeverity,
                        (3, 1, Some(1)),
                        Value::String("H".to_string()),
                        None,
                    ),
                )],
                vec![(
                    1,
                    TelegramContent::new_value(
                        TelegramContentType::EventlogMessage,
                        (3, 2, Some(1)),
                        Value::String("506f776572204661696c757265".to_string()),
                        None,
                    ),
                )],
                vec![(
                    1,
                    TelegramContent::new_value(
                        TelegramContentType::EventlogDate,
                        (3, 3, Some(1)),
                        Value::Date(Date::new(2023, 7, 2, 13, 12, 0, true)),
                        None,
                    ),
                )],
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
                    TelegramContent::new_value(
                        TelegramContentType::Voltage,
                        (7, 1, Some(1)),
                        Value::Float(241.7),
                        Some(TelegramContentUnit::V),
                    ),
                    TelegramContent::new_value(
                        TelegramContentType::Voltage,
                        (7, 1, Some(2)),
                        Value::Float(240.6),
                        Some(TelegramContentUnit::V),
                    ),
                    TelegramContent::new_value(
                        TelegramContentType::Voltage,
                        (7, 1, Some(3)),
                        Value::Float(241.92),
                        Some(TelegramContentUnit::V),
                    ),
                ],
                currents: [
                    TelegramContent::new_value(
                        TelegramContentType::Current,
                        (7, 2, Some(1)),
                        Value::Float(1.0),
                        Some(TelegramContentUnit::A),
                    ),
                    TelegramContent::new_value(
                        TelegramContentType::Current,
                        (7, 2, Some(2)),
                        Value::Float(10.0),
                        Some(TelegramContentUnit::A),
                    ),
                    TelegramContent::new_value(
                        TelegramContentType::Current,
                        (7, 2, Some(3)),
                        Value::Float(0.5),
                        Some(TelegramContentUnit::A),
                    ),
                ],
                powers: [
                    TelegramContent::new_value(
                        TelegramContentType::Power,
                        (7, 3, Some(1)),
                        Value::Float(1.00),
                        Some(TelegramContentUnit::KW),
                    ),
                    TelegramContent::new_value(
                        TelegramContentType::Power,
                        (7, 3, Some(2)),
                        Value::Float(-5.010),
                        Some(TelegramContentUnit::KW),
                    ),
                    TelegramContent::new_value(
                        TelegramContentType::Power,
                        (7, 3, Some(3)),
                        Value::Float(2.500),
                        Some(TelegramContentUnit::KW),
                    ),
                ],
                total_consumed: TelegramContent::new_value(
                    TelegramContentType::TotalConsumed,
                    (7, 4, Some(1)),
                    Value::Float(11454892.0),
                    Some(TelegramContentUnit::KWH),
                ),
                total_produced: TelegramContent::new_value(
                    TelegramContentType::TotalProduced,
                    (7, 4, Some(2)),
                    Value::Float(1245.0),
                    Some(TelegramContentUnit::KWH),
                ),
            },
        ),
        // Gas recursive inner telegram
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
                vec![], // empty vector instead of None
                vec![], // empty vector instead of None
                vec![], // empty vector instead of None
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
                    Value::Float(12345.123),
                    Some(TelegramContentUnit::M3),
                ),
            },
        ),
    ];

    let parsed = parse(&input).expect("Failed to parse simple_gas example");
    assert_eq!(parsed.len(), expected.len());

    // Check that we got both gas and electricity telegrams
    let has_electricity = matches!(parsed[0].data, TelegramData::Electricity { .. });
    let has_gas = matches!(parsed[1].data, TelegramData::Gas { .. });

    assert!(has_gas, "Should contain a gas telegram");
    assert!(has_electricity, "Should contain an electricity telegram");
}

#[test]
fn test_parse_two_packets() {
    let input = fs::read_to_string("examples/good/two_packets.dsmr")
        .expect("Failed to read two_packets.dsmr file");

    let parsed = parse(&input).expect("Failed to parse two_packets example");
    assert_eq!(parsed.len(), 2, "Should parse exactly two telegrams");

    // Both should be electricity telegrams
    for telegram in &parsed {
        match &telegram.data {
            TelegramData::Electricity { .. } => {}
            _ => panic!("Expected electricity telegram"),
        }
    }

    // Simple voltage test
    if let (
        TelegramData::Electricity { voltages: v1, .. },
        TelegramData::Electricity { voltages: v2, .. },
    ) = (&parsed[0].data, &parsed[1].data)
    {
        if let (Some(Value::Float(first_voltage)), Some(Value::Float(second_voltage))) =
            (&v1[0].value, &v2[0].value)
        {
            assert!(
                (first_voltage - 242.7).abs() < 0.001,
                "First telegram voltage should be 242.7"
            );
            assert!(
                (second_voltage - 241.7).abs() < 0.001,
                "Second telegram voltage should be 241.7"
            );
        }
    }
}

#[test]
fn test_parse_good_sequences() {
    use std::path::Path;

    let sequences_dir = Path::new("examples/good_sequences");
    if !sequences_dir.exists() {
        panic!("Directory examples/good_sequences does not exist");
    }

    let entries = fs::read_dir(sequences_dir).expect("Failed to read good_sequences directory");

    for entry in entries {
        let entry = entry.expect("Failed to get directory entry");
        let path = entry.path();

        let filename = path.file_name().unwrap().to_str().unwrap();
        // println!("Testing file: {}", filename);

        let input = fs::read_to_string(&path).expect(&format!("Failed to read file: {}", filename));

        let result = parse(&input);
        // println!("{:?}", &result);
        assert!(
            result.is_ok(),
            "Failed to parse file: {} - Error: {:?}",
            filename,
            result.err()
        );

        let telegrams = result.unwrap();
        assert!(
            !telegrams.is_empty(),
            "File {} produced no telegrams",
            filename
        );

        println!(
            "Successfully parsed {} telegrams from {}",
            telegrams.len(),
            filename
        );
    }
}
