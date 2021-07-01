use std::{error::Error, fmt::Display};

use serde::Serialize;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;

use crate::weatherapi::api::WeatherApiError;

#[derive(Debug, Serialize)]
pub enum ServerError {
    ParamNotSpecified(String),
    WeatherApiError(WeatherApiError),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ServerError {}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        let msg = json!({"error": self});
        HttpResponse::build(code).json(msg)
    }

}
