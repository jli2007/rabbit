use dotenvy::from_filename;
use std::env::var;

pub struct AppConfig {
    pub rabbitmq_exchange: String,
}

pub fn load() -> AppConfig {
    from_filename(".env.local").ok();

    AppConfig {
        rabbitmq_exchange: var("RABBITMQ_EXCHANGE").expect("RABBITMQ_EXCHANGE must be set"),
    }
}
