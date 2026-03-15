use axum::{Json, Router, routing::{get, post}};
use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties, ExchangeKind};
use serde::Deserialize;
use std::sync::Arc;

const EXCHANGE_NAME: &str = "test_event";
const ROUTING_KEY: &str = "test_key";

#[derive(Deserialize)]
struct WebhookPayload {
    body: String,
}

async fn health() -> &'static str {
    "OK"
}

// publishes message to rabbit
async fn webhook(
    axum::extract::State(channel):
    axum::extract::State<Arc<lapin::Channel>>, 
    Json(payload): Json<WebhookPayload>,
) -> &'static str {
    channel
        .basic_publish(
            EXCHANGE_NAME.into(),
            ROUTING_KEY.into(),
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
    let conn = Connection::connect("amqp://guest:guest@localhost:5672", ConnectionProperties::default())
          .await
          .unwrap();
    
    let channel = conn.create_channel().await.unwrap();

    channel
        .exchange_declare(
            EXCHANGE_NAME.into(),
            ExchangeKind::Topic,
            ExchangeDeclareOptions { durable: true, ..Default::default() },
            FieldTable::default(),
        )
        .await
        .unwrap();

    let channel = Arc::new(channel);

    let app = Router::new()
        .route("/health", get(health))
        .route("/webhook", post(webhook))
        .with_state(channel);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
