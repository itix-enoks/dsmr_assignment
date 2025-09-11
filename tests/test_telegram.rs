use dsmr_assignment::traits::Validatable;

use dsmr_assignment::telegram::{Telegram, TelegramBase, TelegramData, TelegramContent, TelegramContentType, TelegramContentUnit, Date};

// 1. Check whether the simplified TelegramContent compiles and validates correctly
#[test]
fn test_telegram_content_string() {
    let t = TelegramContent::new_string(
        TelegramContentType::Start,
        (1, 1, Some(0)),
        String::from("START"),
        Option::None
    );
    assert_eq!(true, t.validate());
}

#[test]
fn test_telegram_content_tdate() {
    let t = TelegramContent::new_date(
        TelegramContentType::EventlogDate,
        (3, 3, Some(1)),
        Date::new(1, 1, 1, 1, 1, 1, false),
        None
    );
    assert_eq!(true, t.validate());
}

#[test]
fn test_telegram_content_f32() {
    let t = TelegramContent::new_float(
        TelegramContentType::Power,
        (7, 3, Some(1)),
        1.0,
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
            TelegramContent::new_string(
                TelegramContentType::Start,
                (1, 1, Some(0)),
                "START".to_string(),
                None,
            ),
            TelegramContent::new_date(
                TelegramContentType::Date,
                (2, 1, None),
                Date::new(2002, 2, 14, 0, 0, 0, true),
                None,
            ),
            TelegramContent::new_string(
                TelegramContentType::EventlogSeverity,
                (3, 1, Some(1)),
                "H".to_string(),
                None,
            ),
            TelegramContent::new_string(
                TelegramContentType::EventlogMessage,
                (3, 2, Some(1)),
                "Power outage detected".to_string(),
                None,
            ),
            TelegramContent::new_date(
                TelegramContentType::EventlogDate,
                (3, 3, Some(1)),
                Date::new(2002, 2, 14, 14, 30, 0, true),
                None,
            ),
            TelegramContent::new_string(
                TelegramContentType::InformationType,
                (4, 1, None),
                "E".to_string(),
                None,
            ),
            Some(TelegramContent::new_string(
                TelegramContentType::StartChild,
                (1, 1, Some(1)),
                "START".to_string(),
                None,
            )),
            Some(TelegramContent::new_string(
                TelegramContentType::EndChild,
                (1, 2, Some(1)),
                "END".to_string(),
                None,
            )),
            TelegramContent::new_string(
                TelegramContentType::End,
                (1, 2, Some(0)),
                "END".to_string(),
                None,
            ),
        ),
        TelegramData::Electricity {
            voltages: [
                TelegramContent::new_float(TelegramContentType::Voltage, (7, 1, Some(1)), 230.5, None),
                TelegramContent::new_float(TelegramContentType::Voltage, (7, 1, Some(2)), 231.2, None),
                TelegramContent::new_float(TelegramContentType::Voltage, (7, 1, Some(3)), 229.8, None),
            ],
            currents: [
                TelegramContent::new_float(TelegramContentType::Current, (7, 2, Some(1)), 15.3, None),
                TelegramContent::new_float(TelegramContentType::Current, (7, 2, Some(2)), 12.7, None),
                TelegramContent::new_float(TelegramContentType::Current, (7, 2, Some(3)), 14.1, None),
            ],
            powers: [
                TelegramContent::new_float(TelegramContentType::Power, (7, 3, Some(1)), 3.524, None),
                TelegramContent::new_float(TelegramContentType::Power, (7, 3, Some(2)), 2.937, None),
                TelegramContent::new_float(TelegramContentType::Power, (7, 3, Some(3)), 3.240, None),
            ],
            total_consumed: TelegramContent::new_float(TelegramContentType::TotalConsumed, (7, 4, Some(1)), 12345.67, None),
            total_produced: TelegramContent::new_float(TelegramContentType::TotalProduced, (7, 4, Some(2)), 0.0, None),
        },
    );
}
// 2. End
