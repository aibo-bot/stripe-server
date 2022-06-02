use axum::extract::Json;
use std::sync::Arc;
use crate::models::{
    StripeWebhookPayload,
    State
};

pub async fn receive_webhook(Json(payload): Json<StripeWebhookPayload>, state: Arc<State>) {
    todo!()
}