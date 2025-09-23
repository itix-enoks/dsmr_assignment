use tudelft_dsmr_output_generator::{ UnixTimeStamp, date_to_timestamp };

use crate::traits::Validatable;

use crate::error::parse_error;

#[derive(Clone, Debug, PartialEq)]
pub enum TelegramContentType {
    Start,
    Date,
    EventlogSeverity,
    EventlogMessage,
    EventlogDate,
    InformationType,

    /// Electricity
    Voltage,
    Current,
    Power,
    TotalConsumed,
    TotalProduced,

    /// Gas
    GasTotalDelivered,

    End
}

#[derive(Clone, Debug, PartialEq)]
pub enum TelegramContentUnit {
    V,
    A,
    KW,
    KWH,
    M3
}

#[derive(Clone, Debug, PartialEq)]
pub struct Date {
    pub timestamp: UnixTimeStamp,

    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub seconds: u8,
    pub dst: bool
}

impl Date {
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        seconds: u8,
        dst: bool
    ) -> Self {
        Date {
            timestamp: date_to_timestamp(
                year,
                month,
                day,
                hour,
                minute,
                seconds,
                dst,
            ).ok_or(std::io::ErrorKind::InvalidData).map_err(|_| {
                parse_error("Invalid date field")
            }).unwrap(),

            year,
            month,
            day,
            hour,
            minute,
            seconds,
            dst,
        }
    }
}

impl Validatable for Date {
    fn validate(&self) -> bool {
        // Add proper date validation logic here
        self.month >= 1 && self.month <= 12 &&
            self.day >= 1 && self.day <= 31 &&
            self.hour < 24 &&
            self.minute < 60 &&
            self.seconds < 60
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Date(Date),
    Float(f64)
}

#[derive(Debug, Clone, PartialEq)]
pub struct TelegramContent {
    pub telegram_content_type: TelegramContentType,
    pub id: (u32, u32, Option<u32>),
    pub value: Option<Value>,
    pub unit: Option<TelegramContentUnit>,
}

impl TelegramContent {
    pub fn new_value(
        telegram_content_type: TelegramContentType,
        id: (u32, u32, Option<u32>),
        value: Value,
        unit: Option<TelegramContentUnit>
    ) -> Self {
        Self {
            telegram_content_type,
            id,
            value: Some(value),
            unit,
        }
    }

    // Note that I realize that these types are dependent on eachotehr (so given an ID, we determine the telegram content type *based* on that)
    // These tests are just for when I still f- up the constructions of said telegram content types
    fn is_id_correct(&self) -> bool {
        match self.telegram_content_type {
            TelegramContentType::Start =>
                match self.id {
                    (1, 1, _) => true, // NOTE: childs do not have the third omitted digit, but parents do
                    _ => false
                },
            TelegramContentType::Date =>
                match self.id {
                    (2, 1, None) => true,
                    _ => false
                }
            TelegramContentType::EventlogSeverity =>
                match self.id {
                    (3, 1, Some(_)) => true,
                    _ => false
                },
            TelegramContentType::EventlogMessage =>
                match self.id {
                    (3, 2, Some(_)) => true,
                    _ => false
                },
            TelegramContentType::EventlogDate =>
                match self.id {
                    (3, 3, Some(_)) => true,
                    _ => false
                },
            TelegramContentType::InformationType =>
                match self.id {
                    (4, 1, None) => true,
                    _ => false
                },
            TelegramContentType::Voltage =>
                match self.id {
                    (7, 1, Some(_)) => true,
                    _ => false
                },
            TelegramContentType::Current =>
                match self.id {
                    (7, 2, Some(_)) => true,
                    _ => false
                },
            TelegramContentType::Power =>
                match self.id {
                    (7, 3, Some(_)) => true,
                    _ => false
                },
            TelegramContentType::TotalConsumed =>
                match self.id {
                    (7, 4, Some(1)) => true,
                    _ => false
                },
            TelegramContentType::TotalProduced =>
                match self.id {
                    (7, 4, Some(2)) => true,
                    _ => false
                },
            TelegramContentType::GasTotalDelivered =>
                match self.id {
                    (5, 2, None) => true,
                    _ => false
                },
            TelegramContentType::End =>
                match self.id {
                    (1, 2, _) => true, // NOTE: childs do not have the third omitted digit, but parents do
                    _ => false
                }
        }
    }

    fn is_unit_correct(&self) -> bool {
        match self.telegram_content_type {
            TelegramContentType::Voltage =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::V => true,
                    _ => false
                },
            TelegramContentType::Current =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::A => true,
                    _ => false
                },
            TelegramContentType::Power =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::KW => true,
                    _ => false
                },
            TelegramContentType::TotalConsumed =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::KWH => true,
                    _ => false
                },
            TelegramContentType::TotalProduced =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::KWH => true,
                    _ => false
                },
            TelegramContentType::GasTotalDelivered =>
                match self.unit.as_ref().unwrap() {
                    TelegramContentUnit::M3 => true,
                    _ => false
                },
            _ =>
                match self.unit.as_ref() {
                    None => true,
                    _ => false
                },
        }
    }

    fn is_value_correct(&self) -> bool {
        match self.telegram_content_type {
            TelegramContentType::Start |
            TelegramContentType::EventlogSeverity |
            TelegramContentType::EventlogMessage |
            TelegramContentType::InformationType |
            TelegramContentType::End => {
                if let Some(Value::String(_)) = self.value {
                    return true;
                } else {
                    return false;
                }
            },
            TelegramContentType::Date |
            TelegramContentType::EventlogDate => {
                if let Some(Value::Date(_)) = self.value {
                    return true;
                } else {
                    return false;
                }
            },
            TelegramContentType::Voltage |
            TelegramContentType::Current |
            TelegramContentType::Power |
            TelegramContentType::TotalConsumed |
            TelegramContentType::TotalProduced |
            TelegramContentType::GasTotalDelivered => {
                if let Some(Value::Float(_)) = self.value {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}

impl Validatable for TelegramContent {
    fn validate(&self) -> bool {
        let id_check = self.is_id_correct();
        let unit_check = self.is_unit_correct();
        let value_check = self.is_value_correct();

        // Additional validation for date values
        let date_validation = if let Some(Value::Date(date)) = &self.value {
            date.validate()
        } else {
            true
        };

        if !id_check {
            eprintln!("[ERROR] id_check failed.");
        }
        if !unit_check {
            eprintln!("[ERROR] unit_check failed.");
        }
        if !value_check {
            eprintln!("[ERROR] value_check failed.");
        }
        if !date_validation {
            eprintln!("[ERROR] date_validation failed.");
        }

        return id_check && unit_check && value_check && date_validation;
    }
}

#[derive(Debug)]
pub struct TelegramBase {
    pub start: TelegramContent,
    pub date: TelegramContent,
    pub eventlog_severities: Vec<(u32, TelegramContent)>,
    pub eventlog_messages: Vec<(u32, TelegramContent)>,
    pub eventlog_dates: Vec<(u32, TelegramContent)>,
    pub information_type: TelegramContent,
    pub end: TelegramContent,
}

impl TelegramBase {
    pub fn new(
        start: TelegramContent,
        date: TelegramContent,
        eventlog_severities: Vec<(u32, TelegramContent)>,
        eventlog_messages: Vec<(u32, TelegramContent)>,
        eventlog_dates: Vec<(u32, TelegramContent)>,
        information_type: TelegramContent,
        end: TelegramContent,
    ) -> Self {
        Self {
            start,
            date,
            eventlog_severities,
            eventlog_messages,
            eventlog_dates,
            information_type,
            end,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TelegramData {
    Electricity {
        voltages: [TelegramContent; 3],
        currents: [TelegramContent; 3],
        powers: [TelegramContent; 3],
        total_consumed: TelegramContent,
        total_produced: TelegramContent,
    },
    Gas {
        total_gas_delivered: TelegramContent,
    },
}

#[derive(Debug)]
pub struct Telegram {
    pub base: TelegramBase,
    pub data: TelegramData,
}

impl Telegram {
    pub fn new(base: TelegramBase, data: TelegramData) -> Self {
        Self {
            base,
            data
        }
    }
}
