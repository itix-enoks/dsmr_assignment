use crate::error::MainError;
use crate::telegram::{
    Telegram, TelegramBase, TelegramData, TelegramContent, TelegramContentType,
    TelegramContentUnit, Date, Value
};

pub struct ParserConfig {
    pub version: (u32, u32),
    pub is_gas: bool,
    pub is_recursive: bool
}

impl ParserConfig {
    pub fn new(version: (u32, u32), is_gas: bool, is_recursive: bool) -> Result<ParserConfig, MainError> {
        if version == (1, 0) && (is_gas || is_recursive) {
            return Err(parse_error(format!("Protocal version {}.{} does not support extensions", version.0, version.1).as_str()))
        }

        Ok(Self {
            version: version,
            is_gas: is_gas,
            is_recursive: is_recursive
        })
    }
}

pub fn parse_error(msg: &str) -> MainError {
    MainError::from(msg) // Made MainError implement From<&str>
}

pub fn parse_header(line: &str) -> Result<ParserConfig, MainError> {
    if line.contains(' ') {
        return Err(parse_error("Invalid header format"));
    }
    if !line.starts_with('/') {
        return Err(parse_error("Invalid header format"));
    }

    let parts: Vec<&str> = line.split(&['/', '\\'][..]).collect();
    if parts.len() != 3 {
        return Err(parse_error("Invalid header format"));
    }

    let version = match parts[1] {
        "v10" => if !line.contains('+') {
            (1, 0)
        } else {
            return Err(parse_error("Invalid header format"));
        },
        "v12" => (1, 2),
        _ => {
            return Err(parse_error("Invalid header format"));
        }
    };

    let parts: Vec<&str> = line.split('+').collect();
    if parts.len() != 2 {
        // No extensions
        return ParserConfig::new(version, false, false);
    }

    let (is_gas, is_recursive) = match parts[1] {
        "g" => (true, false),
        "r" => (false, true),
        "gr" | "rg" => (true, true),
        _ => {
            return Err(parse_error("Invalid header format"));
        }
    };

    ParserConfig::new(version, is_gas, is_recursive)
}

pub fn parse(input: &str) -> Result<Vec<Telegram>, MainError> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err(parse_error("Empty input"));
    }
    let mut _config: Option<ParserConfig> = Option::None;
    let mut temporary_stack: Vec<Vec<TelegramContent>> = Vec::new();
    let mut completed_stack: Vec<Telegram> = Vec::new();

    // FIXME: we should raise exception when parsing extensions that MUST be disabled by the spec (per version).
    // Parse all lines into telegram contents
    for (index, line) in lines.into_iter().enumerate() {
        // Parse header
        if index == 0 {
            _config = Some(parse_header(line)?);
            continue;
        }
        println!("info: parsing line {}: {}", index, line);
        if line.trim().is_empty() {
            continue;
        }
        //
        match parse_line(line) {
            Ok(content) => {
                match content.telegram_content_type {
                    TelegramContentType::Start => {
                        // println!("info: {:?}", content);
                        // Make a new one in temp
                        temporary_stack.push(Vec::new());
                        if let Some(last) = temporary_stack.last_mut() {
                            last.push(content);
                        }
                    },
                    TelegramContentType::End => {
                        // println!("info: {:?}", content);
                        if let Some(mut last_telegram) = temporary_stack.pop() {
                            last_telegram.push(content);
                            completed_stack.push(build_telegram(last_telegram)?);
                        }
                    },
                    _ => {
                        // println!("info: {:?}", content);
                        if let Some(last) = temporary_stack.last_mut() {
                            last.push(content);
                        }
                    }
                }
            },
            Err(e) => {
                println!("warning: failed to parse line {}: {:?}", index, e);
                continue;
            }
        }
    }

    completed_stack.reverse(); println!("info: {:?}", completed_stack);
    Ok(completed_stack)
}

pub fn parse_line(line: &str) -> Result<TelegramContent, MainError> {
    if !line.contains('(') || !line.contains(')') {
        return Err(parse_error("Invalid line format"));
    }

    let parts: Vec<&str> = line.split('#').collect();
    if parts.len() != 2 {
        return Err(parse_error("Invalid line format"));
    }

    let id_part = parts[0];
    let value_part = parts[1].trim_start_matches('(').trim_end_matches(')');

    // ID
    let id = parse_id(id_part)?;

    // Determine content type from ID
    let content_type = determine_content_type(&id)?;

    // Parse value and unit
    let (value_str, unit) = if value_part.contains('*') {
        let value_parts: Vec<&str> = value_part.split('*').collect();
        if value_parts.len() != 2 {
            return Err(parse_error("Invalid value*unit format"));
        }
        (value_parts[0], Some(parse_unit(value_parts[1])?))
    } else {
        (value_part, None)
    };

    // Create appropriate TelegramContent based on type
    match content_type {
        TelegramContentType::Start |
        TelegramContentType::EventlogSeverity |
        TelegramContentType::EventlogMessage |
        TelegramContentType::InformationType |
        TelegramContentType::End => {
            Ok(TelegramContent::new_value(content_type, id, Value::String(value_str.to_string()), unit))
        },

        TelegramContentType::Date |
        TelegramContentType::EventlogDate => {
            let date = parse_date(value_str)?;
            Ok(TelegramContent::new_value(content_type, id, Value::Date(date), unit))
        },

        TelegramContentType::Voltage |
        TelegramContentType::Current |
        TelegramContentType::Power |
        TelegramContentType::TotalConsumed |
        TelegramContentType::TotalProduced |
        TelegramContentType::GasTotalDelivered => {
            let float_value = value_str.parse::<f64>()
                .map_err(|_| parse_error("Invalid float value"))?;
            Ok(TelegramContent::new_value(content_type, id, Value::Float(float_value), unit))
        }
    }
}

pub fn parse_id(id_str: &str) -> Result<(u32, u32, Option<u32>), MainError> {
    let digits: Vec<&str> = id_str.split('.').collect();
    if digits.len() == 0 || digits.len() > 3 {
        return Err(parse_error("Invalid ID format"));
    }

    let id_0 = digits[0].parse::<u32>()
        .map_err(|_| parse_error("Invalid major ID"))?;
    let id_1 = digits[1].parse::<u32>()
        .map_err(|_| parse_error("Invalid minor ID"))?;
    let id_3 = if digits.len() == 3 {
        Some(digits[2].parse::<u32>()
             .map_err(|_| parse_error("Invalid minor ID"))?)
    } else {
        None
    };

    Ok((id_0, id_1, id_3))
}

pub fn determine_content_type(id: &(u32, u32, Option<u32>)) -> Result<TelegramContentType, MainError> {
    match id {
        (1, 1, Some(0)) | (1, 1, _) => Ok(TelegramContentType::Start),
        (1, 2, Some(0)) | (1, 2, _) => Ok(TelegramContentType::End),
        (2, 1, None) => Ok(TelegramContentType::Date),
        (3, 1, _) => Ok(TelegramContentType::EventlogSeverity),
        (3, 2, _) => Ok(TelegramContentType::EventlogMessage),
        (3, 3, _) => Ok(TelegramContentType::EventlogDate),
        (4, 1, None) => Ok(TelegramContentType::InformationType),
        (5, 2, None) => Ok(TelegramContentType::GasTotalDelivered),
        (7, 1, _) => Ok(TelegramContentType::Voltage),
        (7, 2, _) => Ok(TelegramContentType::Current),
        (7, 3, _) => Ok(TelegramContentType::Power),
        (7, 4, Some(1)) => Ok(TelegramContentType::TotalConsumed),
        (7, 4, Some(2)) => Ok(TelegramContentType::TotalProduced),
        _ => Err(parse_error(&format!("Unknown ID: {:?}", id))),
    }
}

pub fn parse_unit(unit_str: &str) -> Result<TelegramContentUnit, MainError> {
    match unit_str.to_uppercase().as_str() {
        "V" => Ok(TelegramContentUnit::V),
        "A" => Ok(TelegramContentUnit::A),
        "KW" => Ok(TelegramContentUnit::KW),
        "KWH" => Ok(TelegramContentUnit::KWH),
        "M3" => Ok(TelegramContentUnit::M3),
        _ => Err(parse_error(&format!("Unknown unit: {}", unit_str))),
    }
}

pub fn parse_date(date_str: &str) -> Result<Date, MainError> {
    // Expected format: "YY-MMM-dd hh:mm:ss (X)" where MMM is month name and X is S/W for DST
    // Example: "23-Jul-05 15:26:41 (S)"

    // Remove parentheses and split by space
    let cleaned = date_str.trim_start_matches('(').trim_end_matches(')');
    let parts: Vec<&str> = cleaned.split(' ').collect();

    if parts.len() != 3 {
        return Err(parse_error("Invalid date format"));
    }

    // Parse date part (YY-MMM-dd)
    let date_parts: Vec<&str> = parts[0].split('-').collect();
    if date_parts.len() != 3 {
        return Err(parse_error("Invalid date part format"));
    }

    let year = format!("20{}", date_parts[0]).parse::<u16>()
        .map_err(|_| parse_error("Invalid year"))?;

    let month = match date_parts[1] {
        "Jan" => 1, "Feb" => 2, "Mar" => 3, "Apr" => 4,
        "May" => 5, "Jun" => 6, "Jul" => 7, "Aug" => 8,
        "Sep" => 9, "Oct" => 10, "Nov" => 11, "Dec" => 12,
        _ => return Err(parse_error("Invalid month name")),
    };

    let day = date_parts[2].parse::<u8>()
        .map_err(|_| parse_error("Invalid day"))?;

    // Parse time part (hh:mm:ss)
    let time_parts: Vec<&str> = parts[1].split(':').collect();
    if time_parts.len() != 3 {
        return Err(parse_error("Invalid time format"));
    }

    let hour = time_parts[0].parse::<u8>()
        .map_err(|_| parse_error("Invalid hour"))?;
    let minute = time_parts[1].parse::<u8>()
        .map_err(|_| parse_error("Invalid minute"))?;
    let seconds = time_parts[2].parse::<u8>()
        .map_err(|_| parse_error("Invalid seconds"))?;

    // Parse DST flag (S for summer/DST, W for winter/standard time)
    let dst_flag = parts[2].trim_start_matches('(').trim_end_matches(')');
    let dst = match dst_flag {
        "S" => true,  // Summer time (DST active)
        "W" => false, // Winter time (DST not active)
        _ => return Err(parse_error("Invalid DST flag")),
    };

    Ok(Date::new(year, month, day, hour, minute, seconds, dst))
}

pub fn build_telegram(contents: Vec<TelegramContent>) -> Result<Telegram, MainError> {
    let mut start = None;
    let mut date = None;
    let mut eventlog_severity = None;
    let mut eventlog_message = None;
    let mut eventlog_date = None;
    let mut information_type = None;
    let mut end = None;

    let mut voltages = Vec::new();
    let mut currents = Vec::new();
    let mut powers = Vec::new();
    let mut total_consumed = None;
    let mut total_produced = None;
    let mut total_gas_delivered = None;

    // Sort contents into appropriate fields
    for content in contents {
        match content.telegram_content_type {
            TelegramContentType::Start => start = Some(content),
            TelegramContentType::Date => date = Some(content),
            TelegramContentType::EventlogSeverity => eventlog_severity = Some(content),
            TelegramContentType::EventlogMessage => eventlog_message = Some(content),
            TelegramContentType::EventlogDate => eventlog_date = Some(content),
            TelegramContentType::InformationType => information_type = Some(content),
            TelegramContentType::End => end = Some(content),
            TelegramContentType::Voltage => voltages.push(content),
            TelegramContentType::Current => currents.push(content),
            TelegramContentType::Power => powers.push(content),
            TelegramContentType::TotalConsumed => total_consumed = Some(content),
            TelegramContentType::TotalProduced => total_produced = Some(content),
            TelegramContentType::GasTotalDelivered => total_gas_delivered = Some(content),
        }
    }

    // Build TelegramBase
    let base = TelegramBase::new(
        start.ok_or_else(|| parse_error("Missing start field"))?,
        date.ok_or_else(|| parse_error("Missing date field"))?,
        eventlog_severity.or(Option::None),
        eventlog_message.or(Option::None),
        eventlog_date.or(Option::None),
        information_type.ok_or_else(|| parse_error("Missing information_type field"))?,
        end.ok_or_else(|| parse_error("Missing end field"))?,
    );

    // Determine data type and build TelegramData
    let data = if let Some(gas_delivered) = total_gas_delivered {
        TelegramData::Gas {
            total_gas_delivered: gas_delivered,
        }
    } else if voltages.len() >= 3 && currents.len() >= 3 && powers.len() >= 3
        && total_consumed.is_some() && total_produced.is_some() {

            let voltage_array: [TelegramContent; 3] = [
                voltages.clone().into_iter().nth(0).unwrap(),
                voltages.clone().into_iter().nth(1).unwrap(),
                voltages.clone().into_iter().nth(2).unwrap(),
            ];
            let current_array: [TelegramContent; 3] = [
                currents.clone().into_iter().nth(0).unwrap(),
                currents.clone().into_iter().nth(1).unwrap(),
                currents.clone().into_iter().nth(2).unwrap(),
            ];
            let power_array: [TelegramContent; 3] = [
                powers.clone().into_iter().nth(0).unwrap(),
                powers.clone().into_iter().nth(1).unwrap(),
                powers.clone().into_iter().nth(2).unwrap(),
            ];

            TelegramData::Electricity {
                voltages: voltage_array,
                currents: current_array,
                powers: power_array,
                total_consumed: total_consumed.unwrap(),
                total_produced: total_produced.unwrap(),
            }
        } else {
            return Err(parse_error("Insufficient data for telegram"));
        };

    Ok(Telegram::new(base, data))
}
