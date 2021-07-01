use crate::weatherapi::api::Endpoint;

#[derive(Clone)]
pub struct AWEndpoint {
    client_id: String,
    client_secret: String,
}

impl AWEndpoint {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }
}

impl Endpoint for AWEndpoint {
    fn get_url(
        &self,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
        city: String,
    ) -> String {
        format!("https://api.aerisapi.com/forecasts/{city}?from={start_date}&to={end_date}&client_id={id}&client_secret={secret}", city=city, start_date=start_date, end_date=end_date, id=self.client_id, secret=self.client_secret)
    }
}
