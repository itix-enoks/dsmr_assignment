use crate::traits::Validatable;
use crate::traits::Wrapper;

use crate::telegram::TelegramContentType;

pub struct TString {
    telegram_content_type: TelegramContentType,
    value: String
}

impl Validatable for TString {
    fn validate(&self) -> bool {
        true // todo!()
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

pub struct TDate {
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
        true // todo!()
    }
}

impl Wrapper<TDate> for TDate {
    fn wrap(value: TDate, _telegram_content_type: TelegramContentType) -> Self {
        value
    }
}

impl TDate {
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        seconds: u8,
        dst: bool
    ) -> Self {
        TDate {
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

pub struct TFloat {
    telegram_content_type: TelegramContentType,
    value: f32
}

impl Validatable for TFloat {
    fn validate(&self) -> bool {
        true // todo!()
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
