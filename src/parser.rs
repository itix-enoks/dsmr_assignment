use std::str::Lines;

use crate::error::MainError;

#[derive(Clone)]
enum TelegramContentType {
    Start,
    Date,
    EventlogSeverity,
    EventlogMessage,
    EventlogDate,
    InformationType,

    /// Electricity
    ElectricityVoltage,
    ElectricityCurrent,
    ElectricityPower,
    ElectricityTotalConsumed,
    ElectricityTotalProduced,

    /// V1.2+
    StartChild,
    EndChild,

    /// Gas
    GasTotalDelivered,

    End
}

enum TelegramContentUnit {
    V,
    A,
    KW,
    KWH,
    M3
}

/// FIXME: make TString have a range as needed by the specific type.
trait Validatable {
    fn validate(&self) -> bool;
}

trait Wrapper<U> {
    fn wrap(value: U, telegram_content_type: TelegramContentType) -> Self;
}

struct TString {
    telegram_content_type: TelegramContentType,
    value: String
}
impl Validatable for TString {
    fn validate(&self) -> bool {
        true
    }
}
impl Wrapper<String> for TString {
    fn wrap(value: String, telegram_content_type: TelegramContentType) -> Self {
        TString {
            telegram_content_type: telegram_content_type,
            value: value
        }
    }
}

struct TDate {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    seconds: u8,
    dst: bool
}
impl Validatable for TDate {
    fn validate(&self) -> bool {
        true
    }
}
impl Wrapper<TDate> for TDate {
    fn wrap(value: TDate, _telegram_content_type: TelegramContentType) -> Self {
        value
    }
}

struct TFloat {
    telegram_content_type: TelegramContentType,
    value: f32
}
impl Validatable for TFloat {
    fn validate(&self) -> bool {
        true
    }
}
impl Wrapper<f32> for TFloat {
    fn wrap(value: f32, telegram_content_type: TelegramContentType) -> Self {
        TFloat {
            telegram_content_type: telegram_content_type,
            value: value
        }
    }
}

struct TelegramContent<T: Validatable + Wrapper<U>, U> {
    telegram_content_type: TelegramContentType,
    id: (u32, u32, Option<u32>),
    value: T,
    unit: Option<TelegramContentUnit>,

    _phantom: std::marker::PhantomData<U> // This is needed for correct static-type checking apparently when not having a type U in any of these fields above
}

impl<T: Validatable + Wrapper<U>, U> TelegramContent<T, U> {
    fn new(telegram_content_type: TelegramContentType, id: (u32, u32, Option<u32>), value: U, unit: Option<TelegramContentUnit>) -> Self {
        Self {
            telegram_content_type: telegram_content_type.clone(),
            id: id,
            value: T::wrap(value, telegram_content_type.clone()),
            unit: unit,
            _phantom: std::marker::PhantomData
        }
    }
}

impl<T: Validatable + Wrapper<U>, U> TelegramContent<T, U> {
    fn is_id_correct(&self) -> bool {
        match self.telegram_content_type {
            TelegramContentType::Start =>
                match self.id {
                    (1, 1, Some(0)) => true,
                    _ => false
                },
            TelegramContentType::Date =>
                match self.id {
                    (2, 1, Option::None) => true,
                    _ => false
                }
            TelegramContentType::EventlogSeverity =>
                match self.id {
                    (3, 1, _) => true,
                    _ => false
                },
            TelegramContentType::EventlogMessage =>
                match self.id {
                    (3, 2, _) => true,
                    _ => false
                },
            TelegramContentType::EventlogDate =>
                match self.id {
                    (3, 3, _) => true,
                    _ => false
                },
            TelegramContentType::InformationType =>
                match self.id {
                    (4, 1, Option::None) => true,
                    _ => false
                },
            TelegramContentType::ElectricityVoltage =>
                match self.id {
                    (7, 1, _) => true,
                    _ => false
                },
            TelegramContentType::ElectricityCurrent =>
                match self.id {
                    (7, 2, _) => true,
                    _ => false
                },
            TelegramContentType::ElectricityPower =>
                match self.id {
                    (7, 3, _) => true,
                    _ => false
                },
            TelegramContentType::ElectricityTotalConsumed =>
                match self.id {
                    (7, 4, Some(1)) => true,
                    _ => false
                },
            TelegramContentType::ElectricityTotalProduced =>
                match self.id {
                    (7, 4, Some(2)) => true,
                    _ => false
                },
            TelegramContentType::StartChild =>
                match self.id {
                    (1, 1, _) => true,
                    _ => false
                },
            TelegramContentType::EndChild =>
                match self.id {
                    (1, 2, _) => true,
                    _ => false
                },
            TelegramContentType::GasTotalDelivered =>
                match self.id {
                    (5, 2, Option::None) => true,
                    _ => false
                },
            TelegramContentType::End =>
                match self.id {
                    (1, 2, Some(0)) => true,
                    _ => false
                }
        }
    }

    fn is_unit_correct(&self) -> bool {
        match self.telegram_content_type {
            TelegramContentType::ElectricityVoltage =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::V => true,
                    _ => false
                },
            TelegramContentType::ElectricityCurrent =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::A => true,
                    _ => false
                },
            TelegramContentType::ElectricityPower =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::KW => true,
                    _ => false
                },
            TelegramContentType::ElectricityTotalConsumed =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::KWH => true,
                    _ => false
                },
            TelegramContentType::ElectricityTotalProduced =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::KWH => true,
                    _ => false
                },
            TelegramContentType::GasTotalDelivered =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::M3 => true,
                    _ => false
                },
            _ => true // Absence of unit who are absent in this match is valid (otherwise they would be in this match)
        }
    }
}

impl<T: Validatable + Wrapper<U>, U> Validatable for TelegramContent<T, U> {
    fn validate(&self) -> bool {
        let id_check = self.is_id_correct();
        let value_check = self.value.validate();
        let unit_check = self.is_unit_correct();

        if !id_check {
            println!("[ERROR] id_check failed.");
            // std::process::exit(42);
        }
        if !value_check {
            println!("[ERROR] value_check failed.");
            // std::process::exit(42);
        }
        if !unit_check {
            println!("[ERROR] unit_check failed.");
            // std::process::exit(42);
        }

        return id_check && value_check && unit_check;
    }
}

struct Telegram<'a> {
    lines: Lines<'a>,
    version: u32
}

impl<'a> Telegram<'a> {

}

pub fn parse(input: &str) -> Result<(), MainError> {
    let lines = input.lines();
    for l in lines {
        println!("info: line: {}", l);
    }

    Ok(())
}

// 1. Check whether generics compile + validate TelegramContent
#[test]
fn test_telegram_content_string() {
    let t: TelegramContent<TString, String> = TelegramContent::new(
        TelegramContentType::Start,
        (1, 1, Some(0)),
        String::from("START"),
        Option::None
    );
    assert_eq!(true, t.validate());
}
#[test]
fn test_telegram_content_tdate() {
    let t: TelegramContent<TDate, TDate> = TelegramContent::new(
        TelegramContentType::EventlogDate,
        (3, 3, Some(1)),
        TDate {
            year: 1, month: 1, day: 1, hour: 1, minute: 1, seconds: 1, dst: false
        },
        Some(TelegramContentUnit::KWH)
    );
    assert_eq!(true, t.validate());
}
#[test]
fn test_telegram_content_f32() {
    let t: TelegramContent<TFloat, f32> = TelegramContent::new(
        TelegramContentType::ElectricityPower,
        (7, 3, Some(1)),
        1.0,
        Some(TelegramContentUnit::KW)
    );
    assert_eq!(true, t.validate());
}
// 1. End
