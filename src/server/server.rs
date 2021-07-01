use crate::weatherapi::{
    api::{Forecast, WeatherApi},
};

use actix_web::{
    guard,
    web::{self, Query},
    App, HttpResponse, HttpServer, Result as ActixResult,
};
use chrono::NaiveDate;
use serde::Deserialize;

use super::error::ServerError;

type API = Box<dyn WeatherApi + 'static + Send + Sync>;
pub struct Server {
    port: u16,
    apis: Vec<API>,
}

#[derive(Deserialize, Debug)]
struct Info {
    date: Option<NaiveDate>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    city: Option<String>,
}

async fn one(
    web::Query(info): Query<Info>,
    apis: web::Data<Vec<API>>,
) -> ActixResult<HttpResponse> {
    let date = info
        .date
        .ok_or(ServerError::ParamNotSpecified("date".to_string()))?;
    let city = info
        .city
        .ok_or(ServerError::ParamNotSpecified("city".to_string()))?;
    let mut forecasts = vec![];
    for api in apis.iter() {
        let f = api
            .daily_forecast(date, city.clone())
            .await
            .map_err(|err| ServerError::WeatherApiError(err))?;
        forecasts.push(f);
    }
    let result = forecasts.iter().map(|f| f.temperature).sum::<f64>() / forecasts.len() as f64;
    Ok(HttpResponse::Ok().json(Forecast::new(result, date)))
}

async fn many(
    web::Query(info): Query<Info>,
    apis: web::Data<Vec<API>>,
) -> ActixResult<HttpResponse> {
    let start_date = info
        .start_date
        .ok_or(ServerError::ParamNotSpecified("start_date".to_string()))?;
    let end_date = info
        .end_date
        .ok_or(ServerError::ParamNotSpecified("end_date".to_string()))?;
    let city = info
        .city
        .as_ref()
        .ok_or(ServerError::ParamNotSpecified("city".to_string()))?;

    let mut forecasts = vec![];
    for api in apis.iter() {
        let f = api
            .forecast(start_date, end_date, city.clone())
            .await
            .map_err(|err| ServerError::WeatherApiError(err))?;
        forecasts.push(f);
    }

    // transpose vector from [[1,2,3], [4,5,6]] -> [[1,4], [2,5], [3,6]]
    // yes. Its not effective due to cloning, but it looks good :) (and we have maximum 14 forecasts)
    let forecasts : Vec<_> = (0..forecasts[0].len())
        .map(|i| {
            forecasts
                .iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<Forecast>>()
        })
        .collect();

    let mut result = vec![];
    for vec_fc in forecasts.iter() {
        let temp = vec_fc.iter().map(|f| f.temperature).sum::<f64>() / vec_fc.len() as f64;
        let fc_opt = vec_fc.first();
        if fc_opt.is_none() {
            continue;
        }
        let date = fc_opt.unwrap().date;
        result.push(Forecast::new(temp, date))
    }
    Ok(HttpResponse::Ok().json(result))
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self { port, apis: vec![] }
    }
    pub fn add_api(&mut self, api: API) -> &mut Self {
        self.apis.push(api);
        self
    }
    pub async fn run(&mut self) -> std::io::Result<()> {
        println!("starting server at port {}", self.port);
        let apis = self.apis.clone();
        HttpServer::new(move || {
            App::new()
                .data(apis.clone())
                .service(web::resource("/single_date").guard(guard::Get()).to(one))
                .service(web::resource("/date_range").guard(guard::Get()).to(many))
        })
        .bind(format!("0.0.0.0:{}", self.port))?
        .run()
        .await
    }
}
