use std::{error::Error, fmt::Display};

use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Forecast {
    pub temperature: f64,
    pub date: NaiveDate,
}
impl Forecast {
    pub fn new(temperature: f64, date: NaiveDate) -> Self {
        Self { temperature, date }
    }
}

#[derive(Debug, Serialize)]
pub enum WeatherApiError {
    ApiNotAvailable,
    BadRequest(String),
    RateLimit,
    UnavailableDate,
    CityNotFound,
    Other(String),
}

impl Display for WeatherApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for WeatherApiError {}

pub type WeatherApiResult<T> = Result<T, WeatherApiError>;

#[async_trait::async_trait]
pub trait WeatherApi: WeatherApiClone {
    // pub trait WeatherApi{
    async fn forecast(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        city: String,
    ) -> WeatherApiResult<Vec<Forecast>>;
    async fn daily_forecast(&self, date: NaiveDate, city: String) -> WeatherApiResult<Forecast>;
}

pub trait WeatherApiClone {
    fn clone_box(&self) -> Box<dyn WeatherApi + Send + Sync>;
}

impl<T: 'static + WeatherApi + Send + Sync + Clone> WeatherApiClone for T {
    fn clone_box(&self) -> Box<dyn WeatherApi + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn WeatherApi + Send + Sync> {
    fn clone(&self) -> Box<dyn WeatherApi + Send + Sync> {
        self.clone_box()
    }
}

pub trait Endpoint {
    fn get_url(&self, start_date: NaiveDate, end_date: NaiveDate, city: String) -> String;
}
