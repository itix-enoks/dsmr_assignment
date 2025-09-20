use dsmr_assignment::traits::Validatable;

use dsmr_assignment::telegram::{Telegram, TelegramBase, TelegramData, TelegramContent, TelegramContentType, TelegramContentUnit, Date, Value};

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
            Some(TelegramContent::new_value(
                TelegramContentType::EventlogSeverity,
                (3, 1, Some(1)),
                Value::String("H".to_string()),
                None,
            )),
            Some(TelegramContent::new_value(
                TelegramContentType::EventlogMessage,
                (3, 2, Some(1)),
                Value::String("Power outage detected".to_string()),
                None,
            )),
            Some(TelegramContent::new_value(
                TelegramContentType::EventlogDate,
                (3, 3, Some(1)),
                Value::Date(Date::new(2002, 2, 14, 14, 30, 0, true)),
                None,
            )),
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
