use crate::traits::validatable::Validatable;
use crate::bail;

use crate::error::{ MainError, parse_error };
use crate::telegram::*;

#[derive(Clone)]
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

pub fn parse_header(line: &str) -> Result<ParserConfig, MainError> {
    if line.contains(' ') {
        return Err(parse_error("Invalid header format"))
    }
    if !line.starts_with('/') {
        return Err(parse_error("Invalid header format"))
    }

    let parts: Vec<&str> = line.split(&['/', '\\'][..]).collect();
    if parts.len() != 3 {
        return Err(parse_error("Invalid header format"))
    }

    let version = match parts[1] {
        "v10" => if !line.contains('+') {
            (1, 0)
        } else {
            return Err(parse_error("Invalid header format"))
        },
        "v12" => (1, 2),
        _ => {
            return Err(parse_error("Invalid header format"))
        }
    };

    let parts: Vec<&str> = line.split('+').collect();
    if parts.len() != 2 {
        // No extensions
        return ParserConfig::new(version, false, false)
    }

    let (is_gas, is_recursive) = match parts[1] {
        "g" => (true, false),
        "r" => (false, true),
        "gr" | "rg" => (true, true),
        _ => {
            return Err(parse_error("Invalid header format"))
        }
    };

    ParserConfig::new(version, is_gas, is_recursive)
}

pub fn parse(input: &str) -> Result<Vec<Telegram>, MainError> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err(parse_error("Empty input"))
    }
    let mut config: Option<ParserConfig> = Option::None;
    let mut temporary_stack: Vec<Vec<TelegramContent>> = Vec::new();
    let mut completed_stack: Vec<Telegram> = Vec::new();

    // Parse all lines into telegram contents
    for (index, line) in lines.into_iter().enumerate() {
        // Parse header
        if index == 0 {
            config = Some(parse_header(line)?);
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        match parse_line(line) {
            Ok(content) => {
                match content.telegram_content_type {
                    TelegramContentType::Start => {
                        // Make a new one in temp
                        temporary_stack.push(Vec::new());
                        if let Some(last) = temporary_stack.last_mut() {
                            last.push(content);
                        }
                    },
                    TelegramContentType::End => {
                        if let Some(mut last_telegram) = temporary_stack.pop() {
                            last_telegram.push(content);
                            completed_stack.push(build_telegram(last_telegram)?);
                        }
                        if !config.clone().unwrap_or_else(|| bail!("Missing required field")).is_recursive && temporary_stack.len() > 0 {
                            return Err(parse_error("Recursive telegrams are not supported"))
                        }
                    },
                    ref tct => {
                        if !config.clone().unwrap_or_else(|| bail!("Missing required field")).is_gas && matches!(tct, TelegramContentType::GasTotalDelivered) {
                            return Err(parse_error("Gas data is not supported"))
                        }
                        if matches!(tct, TelegramContentType::InformationType) {
                            if !config.clone().unwrap_or_else(|| bail!("Missing required field")).is_gas {
                                if let Some(Value::String(ref information_type)) = content.value {
                                    if *information_type == "G".to_string() {
                                        return Err(parse_error("Gas data is not supported"))
                                    }
                                }
                            }
                        }
                        if let Some(last) = temporary_stack.last_mut() {
                            last.push(content);
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("error: failed to parse line {}: {:?}", index, e);
                return Err(parse_error("Failed to parse line"));
            }
        }
    }

    completed_stack.reverse();
    Ok(completed_stack)
}

pub fn parse_line(line: &str) -> Result<TelegramContent, MainError> {
    if !line.contains('(') || !line.contains(')') {
        return Err(parse_error("Invalid line format"))
    }

    let parts: Vec<&str> = line.split('#').collect();
    if parts.len() != 2 {
        return Err(parse_error("Invalid line format"))
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
            return Err(parse_error("Invalid value*unit format"))
        }
        (value_parts[0], Some(parse_unit(value_parts[1])?))
    } else {
        (value_part, None)
    };

    // Check value format
    let value_len = value_str.to_string().len();
    match content_type {
        TelegramContentType::Start => if value_str != "START".to_string() {
            return Err(parse_error("Invalid start block"))
        },
        TelegramContentType::EventlogSeverity => if value_str != "H".to_string() && value_str != "L".to_string() {
            return Err(parse_error("Invalid eventlog severity"))
        },
        TelegramContentType::EventlogMessage => if value_len > 1024 {
            return Err(parse_error("Invalid eventlog message"))
        },
        TelegramContentType::InformationType => if value_str != "E".to_string() && value_str != "G".to_string() {
            return Err(parse_error("Invalid information type"))
        },
        TelegramContentType::End => if value_str != "END".to_string() {
            return Err(parse_error("Invalid end block"))
        },
        TelegramContentType::Date |
        TelegramContentType::EventlogDate => if parse_date(value_str).is_err() {
            return Err(parse_error("Invalid date block"))
        },
        TelegramContentType::Voltage => {
            if !value_str.contains(".") {
                return Err(parse_error("Invalid voltage value"))
            }
            let parts: Vec<&str> = value_str.split(".").collect();
            if (parts[1].len() != 1 && parts[1].len() != 2) || value_len != 6 {
                return Err(parse_error("Invalid voltage value"))
            }
        },
        TelegramContentType::Current => {
            if !value_str.contains(".") {
                if value_len != 2 {
                    return Err(parse_error("Invalid current value"))
                }
            }
            else {
                let parts: Vec<&str> = value_str.split(".").collect();
                if (parts[1].len() != 0 && parts[1].len() != 1) || value_len != 3 {
                    return Err(parse_error("Invalid voltage value"))
                }
            }
        },
        TelegramContentType::Power => {
            let ext_size = if !value_str.starts_with("+") && !value_str.starts_with("-") { 0 } else { 1 };
            let parts: Vec<&str> = value_str.split(".").collect();
            if parts[1].len() > 3 || value_len != 6 + ext_size {
                return Err(parse_error("Invalid power value"))
            }
        },
        TelegramContentType::TotalConsumed |
        TelegramContentType::TotalProduced => {
            if !value_str.contains(".") {
                if value_len != 10 {
                    return Err(parse_error("Invalid cumulative power value"))
                }
            }
            else {
                let parts: Vec<&str> = value_str.split(".").collect();
                if parts[1].len() > 10 || value_len != 11 {
                    return Err(parse_error("Invalid cumulative power value"))
                }
            }
        },
        TelegramContentType::GasTotalDelivered => {
            if !value_str.contains(".") {
                return Err(parse_error("Invalid gas value"))
            }
            let parts: Vec<&str> = value_str.split(".").collect();
            if parts[1].len() != 3 || value_len != 9 {
                return Err(parse_error("Invalid gas value"))
            }
        }
    }

    // Create appropriate TelegramContent based on type
    let telegram_content = match content_type {
        TelegramContentType::Start |
        TelegramContentType::EventlogSeverity |
        TelegramContentType::EventlogMessage |
        TelegramContentType::InformationType |
        TelegramContentType::End => {
            Ok::<TelegramContent, MainError>(TelegramContent::new_value(content_type, id, Value::String(value_str.to_string()), unit))
        },

        TelegramContentType::Date |
        TelegramContentType::EventlogDate => {
            let date = parse_date(value_str)?;
            Ok::<TelegramContent, MainError>(TelegramContent::new_value(content_type, id, Value::Date(date), unit))
        },

        TelegramContentType::Voltage |
        TelegramContentType::Current |
        TelegramContentType::Power |
        TelegramContentType::TotalConsumed |
        TelegramContentType::TotalProduced |
        TelegramContentType::GasTotalDelivered => {
            let float_value = value_str.parse::<f64>()
                .map_err(|_| parse_error("Invalid float value"))?;
            Ok::<TelegramContent, MainError>(TelegramContent::new_value(content_type, id, Value::Float(float_value), unit))
        }
    }?;

    if telegram_content.validate() {
        Ok(telegram_content)
    } else {
        return Err(parse_error("Invalid final telegram content"))
    }
}

pub fn parse_id(id_str: &str) -> Result<(u32, u32, Option<u32>), MainError> {
    let digits: Vec<&str> = id_str.split('.').collect();
    if digits.len() == 0 || digits.len() > 3 {
        return Err(parse_error("Invalid ID format"))
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
        (2, 1, None)                => Ok(TelegramContentType::Date),
        (3, 1, _)                   => Ok(TelegramContentType::EventlogSeverity),
        (3, 2, _)                   => Ok(TelegramContentType::EventlogMessage),
        (3, 3, _)                   => Ok(TelegramContentType::EventlogDate),
        (4, 1, None)                => Ok(TelegramContentType::InformationType),
        (5, 2, None)                => Ok(TelegramContentType::GasTotalDelivered),
        (7, 1, _)                   => Ok(TelegramContentType::Voltage),
        (7, 2, _)                   => Ok(TelegramContentType::Current),
        (7, 3, _)                   => Ok(TelegramContentType::Power),
        (7, 4, Some(1))             => Ok(TelegramContentType::TotalConsumed),
        (7, 4, Some(2))             => Ok(TelegramContentType::TotalProduced),
        _                           => Err(parse_error(&format!("Unknown ID: {:?}", id))),
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
        return Err(parse_error("Invalid date format"))
    }

    // Parse date part (YY-MMM-dd)
    let date_parts: Vec<&str> = parts[0].split('-').collect();
    if date_parts.len() != 3 {
        return Err(parse_error("Invalid date part format"))
    }

    let year = format!("20{}", date_parts[0]).parse::<u16>()
        .map_err(|_| parse_error("Invalid year"))?;

    let month = match date_parts[1] {
        "Jan" => 1, "Feb" => 2, "Mar" => 3, "Apr" => 4,
        "May" => 5, "Jun" => 6, "Jul" => 7, "Aug" => 8,
        "Sep" => 9, "Oct" => 10, "Nov" => 11, "Dec" => 12,
        _ => return Err(parse_error("Invalid month name"))
    };

    let day = date_parts[2].parse::<u8>()
        .map_err(|_| parse_error("Invalid day"))?;

    // Parse time part (hh:mm:ss)
    let time_parts: Vec<&str> = parts[1].split(':').collect();
    if time_parts.len() != 3 {
        return Err(parse_error("Invalid time format"))
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
        _ => return Err(parse_error("Invalid DST flag"))
    };

    Ok(Date::new(year, month, day, hour, minute, seconds, dst))
}

pub fn build_telegram(contents: Vec<TelegramContent>) -> Result<Telegram, MainError> {
    let mut start = None;
    let mut date = None;
    let mut eventlog_severity = Vec::new();
    let mut eventlog_message = Vec::new();
    let mut eventlog_date = Vec::new();
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
            TelegramContentType::Start              => start                = Some(content),
            TelegramContentType::Date               => date                 = Some(content),
            TelegramContentType::InformationType    => information_type     = Some(content),
            TelegramContentType::End                => end                  = Some(content),
            TelegramContentType::TotalConsumed      => total_consumed       = Some(content),
            TelegramContentType::TotalProduced      => total_produced       = Some(content),
            TelegramContentType::GasTotalDelivered  => total_gas_delivered  = Some(content),
            TelegramContentType::EventlogSeverity   => eventlog_severity.push((content.id.2.unwrap_or_else(|| bail!("Missing required field")), content)),
            TelegramContentType::EventlogMessage    => eventlog_message.push((content.id.2.unwrap_or_else(|| bail!("Missing required field")), content)),
            TelegramContentType::EventlogDate       => eventlog_date.push((content.id.2.unwrap_or_else(|| bail!("Missing required field")), content)),
            TelegramContentType::Voltage            => voltages.push(content),
            TelegramContentType::Current            => currents.push(content),
            TelegramContentType::Power              => powers.push(content)
        }
    }

    // Build TelegramBase
    let base = TelegramBase::new(
        start.ok_or_else(|| parse_error("Missing start field"))?,
        date.ok_or_else(|| parse_error("Missing date field"))?,
        eventlog_severity,
        eventlog_message,
        eventlog_date,
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
                voltages.clone().into_iter().nth(0).unwrap_or_else(|| bail!("Missing required field")),
                voltages.clone().into_iter().nth(1).unwrap_or_else(|| bail!("Missing required field")),
                voltages.clone().into_iter().nth(2).unwrap_or_else(|| bail!("Missing required field")),
            ];
            let current_array: [TelegramContent; 3] = [
                currents.clone().into_iter().nth(0).unwrap_or_else(|| bail!("Missing required field")),
                currents.clone().into_iter().nth(1).unwrap_or_else(|| bail!("Missing required field")),
                currents.clone().into_iter().nth(2).unwrap_or_else(|| bail!("Missing required field")),
            ];
            let power_array: [TelegramContent; 3] = [
                powers.clone().into_iter().nth(0).unwrap_or_else(|| bail!("Missing required field")),
                powers.clone().into_iter().nth(1).unwrap_or_else(|| bail!("Missing required field")),
                powers.clone().into_iter().nth(2).unwrap_or_else(|| bail!("Missing required field")),
            ];

            TelegramData::Electricity {
                voltages: voltage_array,
                currents: current_array,
                powers: power_array,
                total_consumed: total_consumed.unwrap_or_else(|| bail!("Missing required field")),
                total_produced: total_produced.unwrap_or_else(|| bail!("Missing required field")),
            }
        } else {
            return Err(parse_error("Insufficient data for telegram"))
        };

    Ok(Telegram::new(base, data))
}
