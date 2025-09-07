use crate::traits::Validatable;
use crate::traits::Wrapper;

use crate::primitives::{TString, TDate, TFloat};

#[derive(Clone)]
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

    /// V1.2+
    StartChild,
    EndChild,

    /// Gas
    GasTotalDelivered,

    End
}

#[derive(Clone)]
pub enum TelegramContentUnit {
    V,
    A,
    KW,
    KWH,
    M3
}

#[derive(Clone)]
pub struct TelegramContent<T: Validatable + Wrapper<U>, U> {
    telegram_content_type: TelegramContentType,
    id: (u32, u32, Option<u32>),
    value: T,
    unit: Option<TelegramContentUnit>,

    _phantom: std::marker::PhantomData<U> // This is needed for correct static-type checking apparently when not having a type U in any of these fields above
}

impl<T: Validatable + Wrapper<U>, U> TelegramContent<T, U> {
    pub fn new(telegram_content_type: TelegramContentType, id: (u32, u32, Option<u32>), value: U, unit: Option<TelegramContentUnit>) -> Self {
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
            TelegramContentType::Voltage =>
                match self.id {
                    (7, 1, _) => true,
                    _ => false
                },
            TelegramContentType::Current =>
                match self.id {
                    (7, 2, _) => true,
                    _ => false
                },
            TelegramContentType::Power =>
                match self.id {
                    (7, 3, _) => true,
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
}

impl<T: Validatable + Wrapper<U>, U> Validatable for TelegramContent<T, U> {
    fn validate(&self) -> bool {
        let id_check = self.is_id_correct();
        let value_check = self.value.validate();
        let unit_check = self.is_unit_correct();

        if !id_check {
            println!("[ERROR] id_check failed.");
        }
        if !value_check {
            println!("[ERROR] value_check failed.");
        }
        if !unit_check {
            println!("[ERROR] unit_check failed.");
        }

        return id_check && value_check && unit_check;
    }
}

pub struct TelegramBase {
    start: TelegramContent<TString, String>,
    date: TelegramContent<TDate, TDate>,

    eventlog_severity: TelegramContent<TString, String>,
    eventlog_message: TelegramContent<TString, String>,
    eventlog_date: TelegramContent<TDate, TDate>,

    information_type: TelegramContent<TString, String>,

    start_child: Option<TelegramContent<TString, String>>,
    end_child: Option<TelegramContent<TString, String>>,

    end: TelegramContent<TString, String>
}

impl TelegramBase {
    pub fn new(
        start: TelegramContent<TString, String>,
        date: TelegramContent<TDate, TDate>,
        eventlog_severity: TelegramContent<TString, String>,
        eventlog_message: TelegramContent<TString, String>,
        eventlog_date: TelegramContent<TDate, TDate>,
        information_type: TelegramContent<TString, String>,
        start_child: Option<TelegramContent<TString, String>>,
        end_child: Option<TelegramContent<TString, String>>,
        end: TelegramContent<TString, String>,
    ) -> Self {
        Self {
            start,
            date,
            eventlog_severity,
            eventlog_message,
            eventlog_date,
            information_type,
            start_child,
            end_child,
            end,
        }
    }
}

pub enum TelegramData {
    Electricity {
        voltages: [TelegramContent<TFloat, f32>; 3],
        currents: [TelegramContent<TFloat, f32>; 3],
        powers: [TelegramContent<TFloat, f32>; 3],
        total_consumed: TelegramContent<TFloat, f32>,
        total_produced: TelegramContent<TFloat, f32>,
    },
    Gas {
        total_gas_delivered: TelegramContent<TFloat, f32>,
    },
}

pub struct Telegram {
    base: TelegramBase,
    data: TelegramData,
}

impl Telegram {
    pub fn new(base: TelegramBase, data: TelegramData) -> Self {
        Self {
            base,
            data
        }
    }
}
