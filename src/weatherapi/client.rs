use chrono::NaiveDate;

use crate::weatherapi::api::{Forecast, WeatherApiError, WeatherApiResult, WeatherApi};
use crate::weatherapi::parser::Parser;
use crate::weatherapi::requester::{Requester, RequesterError};

use crate::weatherapi::api::Endpoint;

#[derive(Clone)]
pub struct Client<R, P, E> {
    requester: R,
    parser: P,
    endpoint: E,
}

impl<R, P, E> Client<R, P, E> {
    pub fn new(requester: R, parser: P, endpoint: E) -> Self {
        Self {
            requester,
            parser,
            endpoint,
        }
    }
}

#[async_trait::async_trait]
impl<R, P, E> WeatherApi for Client<R, P, E>
where
    R: Requester + Sync + Send + Clone + 'static,
    P: Parser + Sync + Send + Clone + 'static,
    E: Endpoint + Sync + Send + Clone + 'static,
{
    async fn forecast(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        city: String,
    ) -> WeatherApiResult<Vec<Forecast>> {
        let convert_error_closure = |code, err: serde_json::Value| -> WeatherApiError {
            match code {
                500..=600 => WeatherApiError::ApiNotAvailable,
                _ => {
                    let msg = err.to_string();
                    self.parser
                        .parse_error(err)
                        .unwrap_or(WeatherApiError::Other(msg))
                }
            }
        };
        let url = self.endpoint.get_url(start_date, end_date, city);
        let result = self.requester.get(&url).await.map_err(|err| match err {
            RequesterError::NotOKStatusCode(code, err) => convert_error_closure(code, err),
            _ => WeatherApiError::BadRequest(err.to_string()),
        })?;
        let parsed = self
            .parser
            .parse(result)
            .map_err(|err| WeatherApiError::Other(err.to_string()))?;
        Ok(parsed)
    }

    async fn daily_forecast(&self, date: NaiveDate, city: String) -> WeatherApiResult<Forecast> {
        self.forecast(date, date, city)
            .await?
            .into_iter()
            .next()
            .ok_or(WeatherApiError::UnavailableDate)
    }

}
