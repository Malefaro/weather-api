use reqwest::{Client, Url};

use super::{Requester, RequesterError};

#[derive(Clone)]
pub struct ReqwestRequester {
    client: Client,
}

impl ReqwestRequester {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl Requester for ReqwestRequester {
    async fn get(&self, url: &str) -> Result<serde_json::Value, RequesterError> {
        let url = Url::parse(url).map_err(|_| RequesterError::BadURL)?;
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| RequesterError::Other(err.to_string()))?;

        let code = response.status().as_u16();

        let bytes = response
            .bytes()
            .await
            .map_err(|err| RequesterError::Other(err.to_string()))?;

        let body: serde_json::Value = match bytes.is_empty() {
            true => serde_json::Value::Null,
            false => {
                serde_json::from_slice(&bytes).unwrap_or(
                    serde_json::Value::String(String::from_utf8(bytes.into_iter().collect::<Vec<_>>()).map_err(|_| RequesterError::Other("Cannot parse string".to_string()))?)
                )
            }
        };

        match code {
            400..=599 => return Err(RequesterError::NotOKStatusCode(code, body)),
            _ => (),
        }
        Ok(body)
    }
}
