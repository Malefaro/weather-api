use std::str::FromStr;

use crate::weatherapi::{
    api::{Forecast, WeatherApiError},
    parser::{Parser, ParserError},
};

#[derive(Clone)]
pub struct VCParser {}

impl VCParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parser for VCParser {
    fn parse(&self, json: serde_json::Value) -> Result<Vec<Forecast>, ParserError> {
        if json.is_null() {
            return Err(ParserError::EmptyData);
        }
        let mut result: Vec<Forecast> = Vec::new();
        for data in json["days"].as_array().unwrap() {
            let raw_temp = &data["temp"];
            let temp: f64 = raw_temp.as_f64().ok_or(ParserError::Other(format!(
                "Cannot convert value to float64: {:?}",
                raw_temp
            )))?;
            let raw_date = data["datetime"].as_str().ok_or(ParserError::Other(format!(
                "Cannot convert value to u64 {:?}",
                data["dt"]
            )))?;
            let date = chrono::NaiveDate::from_str(raw_date).map_err(|err| {
                ParserError::Other(format!(
                    "Cannot parse date({}) {}",
                    raw_date,
                    err.to_string()
                ))
            })?;
            result.push(Forecast::new(temp, date));
        }
        Ok(result)
    }

    fn parse_error(&self, json: serde_json::Value) -> Result<WeatherApiError, ParserError> {
        // TODO: parse platform errors (not enougn docs)
        if json.is_null() {
            return Err(ParserError::EmptyData);
        }
        Err(ParserError::Other(json.to_string()))
    }
}
