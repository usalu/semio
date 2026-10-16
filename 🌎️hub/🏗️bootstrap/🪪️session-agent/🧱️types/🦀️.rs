//! 🧱️ The `session_agent` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use semio_hub_auth::agent::{AgentErrorCodeV1, AgentErrorV1};
use semio_hub_auth::{AuthErrorCodeV1, AuthErrorV1};
use serde::Deserialize;
use axum::Json;
use semio_hub_auth::rate_limit::RateLimitDecisionV1;
use axum::response::Response;
use axum::http::StatusCode;

//#region 🔖️Directory
/// 🛑️ The one shape every credential sign-in refusal is served in.
/// 🤖️ Every agent-delegation refusal is served exactly like a credential refusal: the code's
/// own status, `no-store`, and a `retry-after` header whenever the limiter is the reason.
pub fn agent_error_response(error: AgentErrorCodeV1, retry_after_ms: Option<u64>) -> Response {
    let status = StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut body = AgentErrorV1::new(error);
    body.retry_after_ms = retry_after_ms;
    let mut response = (status, Json(body)).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
    if let Some(retry_after_ms) = retry_after_ms {
        if let Ok(value) = axum::http::HeaderValue::from_str(&RateLimitDecisionV1::Refused { retry_after_ms }.retry_after_secs().to_string()) {
            response.headers_mut().insert(axum::http::header::RETRY_AFTER, value);
        }
    }
    response
}

/// 🔎️ `?space=<id>` — the one query parameter the delegation listing takes.
#[derive(Deserialize)]
pub struct AgentDelegationQueryV1 {
    pub space: String,
}

pub fn auth_error_response(error: AuthErrorCodeV1, retry_after_ms: Option<u64>) -> Response {
    let status = StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let body = match retry_after_ms {
        Some(retry_after_ms) => AuthErrorV1::rate_limited(retry_after_ms),
        None => AuthErrorV1::new(error),
    };
    let mut response = (status, Json(body)).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
    if let Some(retry_after_ms) = retry_after_ms {
        if let Ok(value) = axum::http::HeaderValue::from_str(&RateLimitDecisionV1::Refused { retry_after_ms }.retry_after_secs().to_string()) {
            response.headers_mut().insert(axum::http::header::RETRY_AFTER, value);
        }
    }
    response
}
//#endregion 🔖️Directory
