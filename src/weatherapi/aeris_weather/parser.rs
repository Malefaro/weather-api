use crate::weatherapi::{
    api::{Forecast, WeatherApiError},
    parser::{Parser, ParserError},
};

#[derive(Clone)]
pub struct AWParser {}

impl AWParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parser for AWParser {
    fn parse(&self, json: serde_json::Value) -> Result<Vec<Forecast>, ParserError> {
        if json.is_null() {
            return Err(ParserError::EmptyData);
        }
        if !json["success"].as_bool().unwrap() {
            return Err(ParserError::Other(json["error"].to_string()));
        }
        let mut result: Vec<Forecast> = Vec::new();
        for data in json["response"].as_array().unwrap().iter().next().unwrap()["periods"]
            .as_array()
            .unwrap()
        {
            let temp_opt = &data["tempC"].as_f64();
            let temp: f64;
            if temp_opt.is_none() {
                let min_temp = data["minTempC"].as_f64().ok_or(ParserError::Other(format!(
                    "Cannot convert minTempC to int64 {:?}",
                    data["minTempC"]
                )))?;
                let max_temp = data["maxTempC"].as_f64().ok_or(ParserError::Other(format!(
                    "Cannot convert maxTempC to int64 {:?}",
                    data["maxTempC"]
                )))?;
                temp = (min_temp + max_temp) as f64 / 2.0;
            } else {
                temp = temp_opt.unwrap() as f64;
            }
            let raw_date = data["timestamp"]
                .as_i64()
                .ok_or(ParserError::Other(format!(
                    "Cannot convert value to i64 {:?}",
                    data["dt"]
                )))?;
            let date = chrono::NaiveDateTime::from_timestamp(raw_date, 0).date();
            result.push(Forecast::new(temp, date));
        }
        Ok(result)
    }

    fn parse_error(&self, json: serde_json::Value) -> Result<WeatherApiError, ParserError> {
        if json.is_null() {
            return Err(ParserError::EmptyData);
        }
        Err(ParserError::Other(json.to_string()))
    }
}
