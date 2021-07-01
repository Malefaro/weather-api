use weatherapi::requester::reqwest::ReqwestRequester;

use crate::{
    server::server::Server,
    weatherapi::{
        aeris_weather::{endpoint::AWEndpoint, parser::AWParser},
        client::Client,
        visual_crossing::{endpoint::VCEndpoint, parser::VCParser},
    },
};

mod server;
mod weatherapi;

const VC_TOKEN: &'static str = "VC_TOKEN";
const AW_CLIENT_ID: &'static str = "AW_CLIENT_ID";
const AW_CLIENT_SECRET: &'static str = "AW_CLIENT_SECRET";
const PORT: &'static str = "PORT";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let vc_token = std::env::var(VC_TOKEN).expect(&format!("{} not set", VC_TOKEN));
    let aw_client_id = std::env::var(AW_CLIENT_ID).expect(&format!("{} not set", AW_CLIENT_ID));
    let aw_client_secret =
        std::env::var(AW_CLIENT_SECRET).expect(&format!("{} not set", AW_CLIENT_SECRET));
    let server_port: u16 = std::env::var(PORT)
        .expect(&format!("{} not set", PORT))
        .parse()
        .expect("Cannot parse string");
    let mut server = Server::new(server_port);
    let vc = Client::new(
        ReqwestRequester::new(),
        VCParser::new(),
        VCEndpoint::new(vc_token),
    );
    let aw = Client::new(
        ReqwestRequester::new(),
        AWParser::new(),
        AWEndpoint::new(aw_client_id, aw_client_secret),
    );
    server.add_api(Box::new(vc)).add_api(Box::new(aw));
    server.run().await
}
