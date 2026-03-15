mod config;

use axum::{extract::State, routing::{get, post}, Json, Router};
use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties, ExchangeKind};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

const ROUTING_KEY: &str = "issue.classify";

#[derive(Clone)]
struct AppState {
    channel: Arc<lapin::Channel>,
    exchange_name: String,
    routing_key: String,
}

#[derive(Deserialize)]
struct WebhookPayload {
    body: String,
}

async fn health() -> &'static str {
    "OK"
}

// publishes message to rabbit
async fn webhook(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> &'static str {
    state
        .channel
        .basic_publish(
            state.exchange_name.as_str().into(),
            state.routing_key.as_str().into(),
            BasicPublishOptions::default(),
            payload.body.as_bytes(),
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();
    "published"
}

#[tokio::main]
async fn main() {
    let cfg = config::load();
    let conn = Connection::connect("amqp://guest:guest@localhost:5672", ConnectionProperties::default())
          .await
          .unwrap();
    
    let channel = conn.create_channel().await.unwrap();

    channel
        .exchange_declare(
            cfg.rabbitmq_exchange.as_str().into(),
            ExchangeKind::Topic,
            ExchangeDeclareOptions { durable: true, ..Default::default() },
            FieldTable::default(),
        )
        .await
        .unwrap();

    let state = AppState {
        channel: Arc::new(channel),
        exchange_name: cfg.rabbitmq_exchange,
        routing_key: ROUTING_KEY.to_string(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/webhook", post(webhook))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
