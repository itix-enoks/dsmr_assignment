use dsmr_assignment::traits::Validatable;

use dsmr_assignment::telegram::*;
use dsmr_assignment::parser::*;

// 1. Check whether the simplified TelegramContent compiles and validates correctly
#[test]
fn test_telegram_content_string() {
    let t = TelegramContent::new_value(
        TelegramContentType::Start,
        (1, 1, Some(0)),
        Value::String(String::from("START")),
        Option::None
    );
    assert_eq!(true, t.validate());
}

#[test]
fn test_telegram_content_tdate() {
    let t = TelegramContent::new_value(
        TelegramContentType::EventlogDate,
        (3, 3, Some(1)),
        Value::Date(Date::new(1, 1, 1, 1, 1, 1, false)),
        None
    );
    assert_eq!(true, t.validate());
}

#[test]
fn test_telegram_content_f32() {
    let t = TelegramContent::new_value(
        TelegramContentType::Power,
        (7, 3, Some(1)),
        Value::Float(1.0),
        Some(TelegramContentUnit::KW)
    );
    assert_eq!(true, t.validate());
}
// 1. End

// 2. Check whether a Telegram can be constructed
#[test]
fn test_telegram_constructor() {
    let _t = Telegram::new(
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
                Value::Date(Date::new(2002, 2, 14, 0, 0, 0, true)),
                None,
            ),
            vec![(1, TelegramContent::new_value(
                TelegramContentType::EventlogSeverity,
                (3, 1, Some(1)),
                Value::String("H".to_string()),
                None,
            ))],
            vec![(1, TelegramContent::new_value(
                TelegramContentType::EventlogMessage,
                (3, 2, Some(1)),
                Value::String("Power outage detected".to_string()),
                None,
            ))],
            vec![(1, TelegramContent::new_value(
                TelegramContentType::EventlogDate,
                (3, 3, Some(1)),
                Value::Date(Date::new(2002, 2, 14, 14, 30, 0, true)),
                None,
            ))],
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
                TelegramContent::new_value(TelegramContentType::Voltage, (7, 1, Some(1)), Value::Float(230.5), None),
                TelegramContent::new_value(TelegramContentType::Voltage, (7, 1, Some(2)), Value::Float(231.2), None),
                TelegramContent::new_value(TelegramContentType::Voltage, (7, 1, Some(3)), Value::Float(229.8), None),
            ],
            currents: [
                TelegramContent::new_value(TelegramContentType::Current, (7, 2, Some(1)), Value::Float(15.3), None),
                TelegramContent::new_value(TelegramContentType::Current, (7, 2, Some(2)), Value::Float(12.7), None),
                TelegramContent::new_value(TelegramContentType::Current, (7, 2, Some(3)), Value::Float(14.1), None),
            ],
            powers: [
                TelegramContent::new_value(TelegramContentType::Power, (7, 3, Some(1)), Value::Float(3.524), None),
                TelegramContent::new_value(TelegramContentType::Power, (7, 3, Some(2)), Value::Float(2.937), None),
                TelegramContent::new_value(TelegramContentType::Power, (7, 3, Some(3)), Value::Float(3.240), None),
            ],
            total_consumed: TelegramContent::new_value(TelegramContentType::TotalConsumed, (7, 4, Some(1)), Value::Float(12345.67), None),
            total_produced: TelegramContent::new_value(TelegramContentType::TotalProduced, (7, 4, Some(2)), Value::Float(0.0), None),
        },
    );
}
// 2. End

// 3. Additional tests
#[test]
fn test_parse_header_v10() {
    let result = parse_header("/v10\\").unwrap();
    assert_eq!(result.version, (1, 0));
    assert_eq!(result.is_gas, false);
    assert_eq!(result.is_recursive, false);
}

#[test]
fn test_parse_header_v12_with_gas() {
    let result = parse_header("/v12\\+g").unwrap();
    assert_eq!(result.version, (1, 2));
    assert_eq!(result.is_gas, true);
    assert_eq!(result.is_recursive, false);
}

#[test]
fn test_parse_header_v12_with_recursive() {
    let result = parse_header("/v12\\+r").unwrap();
    assert_eq!(result.version, (1, 2));
    assert_eq!(result.is_gas, false);
    assert_eq!(result.is_recursive, true);
}

#[test]
fn test_parse_header_v12_with_gas_and_recursive() {
    let result = parse_header("/v12\\+gr").unwrap();
    assert_eq!(result.version, (1, 2));
    assert_eq!(result.is_gas, true);
    assert_eq!(result.is_recursive, true);
}

#[test]
fn test_parse_header_invalid_version() {
    let result = parse_header("/v11\\");
    assert!(result.is_err());
}

#[test]
fn test_parse_header_invalid_extension_for_v10() {
    let result = parse_header("/v10\\+g");
    assert!(result.is_err());
}

#[test]
fn test_parse_id_two_parts() {
    let result = parse_id("2.1").unwrap();
    assert_eq!(result, (2, 1, None));
}

#[test]
fn test_parse_id_three_parts() {
    let result = parse_id("7.1.3").unwrap();
    assert_eq!(result, (7, 1, Some(3)));
}

#[test]
fn test_parse_id_invalid_format() {
    let result = parse_id("invalid");
    assert!(result.is_err());
}

#[test]
fn test_parse_unit_voltage() {
    let result = parse_unit("V").unwrap();
    assert_eq!(result, TelegramContentUnit::V);
}

#[test]
fn test_parse_unit_case_insensitive() {
    let result = parse_unit("kwh").unwrap();
    assert_eq!(result, TelegramContentUnit::KWH);
}

#[test]
fn test_parse_unit_invalid() {
    let result = parse_unit("INVALID");
    assert!(result.is_err());
}

#[test]
fn test_determine_content_type_voltage() {
    let result = determine_content_type(&(7, 1, Some(1))).unwrap();
    assert_eq!(result, TelegramContentType::Voltage);
}

#[test]
fn test_determine_content_type_gas() {
    let result = determine_content_type(&(5, 2, None)).unwrap();
    assert_eq!(result, TelegramContentType::GasTotalDelivered);
}

#[test]
fn test_determine_content_type_unknown() {
    let result = determine_content_type(&(99, 99, None));
    assert!(result.is_err());
}

#[test]
fn test_parse_date_summer_time() {
    let result = parse_date("23-Jul-05 15:26:41 (S)").unwrap();
    assert_eq!(result.year, 2023);
    assert_eq!(result.month, 7);
    assert_eq!(result.day, 5);
    assert_eq!(result.hour, 15);
    assert_eq!(result.minute, 26);
    assert_eq!(result.seconds, 41);
    assert_eq!(result.dst, true);
}

#[test]
fn test_parse_date_winter_time() {
    let result = parse_date("23-Dec-15 08:30:00 (W)").unwrap();
    assert_eq!(result.year, 2023);
    assert_eq!(result.month, 12);
    assert_eq!(result.day, 15);
    assert_eq!(result.hour, 8);
    assert_eq!(result.minute, 30);
    assert_eq!(result.seconds, 0);
    assert_eq!(result.dst, false);
}

#[test]
fn test_parse_date_invalid_format() {
    let result = parse_date("invalid-date");
    assert!(result.is_err());
}

#[test]
fn test_parse_line_start() {
    let result = parse_line("1.1.0#(START)").unwrap();
    assert_eq!(result.telegram_content_type, TelegramContentType::Start);
    if let Some(Value::String(s)) = result.value {
        assert_eq!(s, "START");
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn test_parse_line_invalid_format() {
    let result = parse_line("invalid_line");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_input() {
    let result = parse("");
    assert!(result.is_err());
}

#[test]
fn test_parser_config_v10_with_extensions_error() {
    let result = ParserConfig::new((1, 0), true, false);
    assert!(result.is_err());
}

#[test]
fn test_parser_config_v12_valid() {
    let result = ParserConfig::new((1, 2), true, true).unwrap();
    assert_eq!(result.version, (1, 2));
    assert_eq!(result.is_gas, true);
    assert_eq!(result.is_recursive, true);
}
// 3. End
