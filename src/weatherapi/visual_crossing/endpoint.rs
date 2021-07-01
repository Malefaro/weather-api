use crate::weatherapi::api::Endpoint;

#[derive(Clone)]
pub struct VCEndpoint {
    token: String,
}

impl VCEndpoint {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Endpoint for VCEndpoint {
    fn get_url(
        &self,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
        city: String,
    ) -> String {
        format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/{city}/{start_date}/{end_date}?key={token}&unitGroup=metric", city=city, start_date=start_date, end_date=end_date, token=self.token)
    }
}
