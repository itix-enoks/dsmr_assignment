use crate::telegram::TelegramContentType;

pub trait Wrapper<U> {
    fn wrap(value: U, telegram_content_type: TelegramContentType) -> Self;
}
