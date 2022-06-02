use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub discord_token: String,
    pub port: u16
}

pub struct State {
    pub config: Config
}

#[derive(Deserialize)]
pub struct StripeWebhookPayload {
    
}