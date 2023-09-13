use chrono::{Timelike, Utc};
use regex::Regex;
use std::error::Error;

use crate::time::{format_time_with_timezone, parse_time, parse_tz};

pub fn process_command(text: &str) -> String {
    match text {
        "/start" => start(),

        _ => match text.split_once(' ') {
            Some((command, rest)) => match command {
                "/now" => now(rest),
                "/convert" => convert(rest).unwrap_or_else(|e| e.to_string()),
                _ => "Invalid command".to_string(),
            },
            None => "Invalid command".to_string(),
        },
    }
}

fn start() -> String {
    "Welcome!".to_string()
}

fn now(timezone: &str) -> String {
    let tz = match parse_tz(timezone) {
        Ok(tz) => tz,
        _ => return format!("Invalid timezone: {timezone}").to_string(),
    };
    let now = Utc::now().with_timezone(&tz);
    format_time_with_timezone(now, timezone)
}

fn convert(input: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"(\d{1,2}:?\d{0,2}) (\w*) (\w*)")?;

    // Check if the input string matches the pattern
    if let Some(captures) = re.captures(input) {
        let source_time = captures.get(1).unwrap().as_str();
        let source_timezone = captures.get(2).unwrap().as_str();
        let target_timezone = captures.get(3).unwrap().as_str();

        let time = parse_time(source_time)?;
        let source_tz = parse_tz(source_timezone)?;
        let target_tz = parse_tz(target_timezone)?;

        let source_time = Utc::now()
            .with_timezone(&source_tz)
            .with_hour(time.hour())
            .unwrap()
            .with_minute(time.minute())
            .unwrap()
            .with_second(0)
            .unwrap();
        let target_time = source_time.with_timezone(&target_tz);
        Ok(format_time_with_timezone(target_time, target_timezone))
    } else {
        Err(Box::try_from("Invalid format".to_string()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_time_brt_cet() {
        let result = convert("12:00 BRT CET");
        assert_eq!(result.ok(), Some("17:00 CET".to_string()));
    }
    #[test]
    fn test_convert_time_utc_brl() {
        let result = convert("12:00 UTC BRT");
        assert_eq!(result.ok(), Some("09:00 BRT".to_string()));
    }
    #[test]
    fn test_convert_time_one_digit() {
        let result = convert("1:00 BRT CET");
        assert_eq!(result.ok(), Some("06:00 CET".to_string()));
    }
    #[test]
    fn test_convert_time_minimal() {
        let result = convert("1 BRT CET");
        assert_eq!(result.ok(), Some("06:00 CET".to_string()));
    }
    #[test]
    fn test_convert_time_missing_target_tz() {
        let result = convert("12:00 UTC");
        assert!(result.is_err());
    }
    #[test]
    fn test_process_command_start() {
        let result = process_command("/start");
        assert_eq!(result, start());
    }
    #[test]
    fn test_process_command_now() {
        let result = process_command("/now utc");
        assert_eq!(result, now("utc"));
    }
    #[test]
    fn test_process_command_convert() {
        let result = process_command("/convert 12:00 UTC BRT");
        assert_eq!(result, convert("12:00 UTC BRT").unwrap());
    }
    #[test]
    fn test_process_command_invalid() {
        let result = process_command("invalid");
        assert_eq!(result, "Invalid command");
    }
}
