use std::fmt::Display;

pub mod reqwest;

#[derive(Debug)]
pub enum RequesterError {
    BadURL,
    NotOKStatusCode(u16, serde_json::Value),
    Other(String),
}

impl Display for RequesterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[async_trait::async_trait]
pub trait Requester {
    async fn get(&self, url: &str) -> Result<serde_json::Value, RequesterError>;
}