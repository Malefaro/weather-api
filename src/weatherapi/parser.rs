use std::{error::Error, fmt::Display};

use super::api::{Forecast, WeatherApiError};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    EmptyData,
    Other(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParserError {}
pub trait Parser {
    fn parse(&self, json: serde_json::Value) -> Result<Vec<Forecast>, ParserError>;
    fn parse_error(&self, json: serde_json::Value) -> Result<WeatherApiError, ParserError>;
}
