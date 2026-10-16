//! 🛣️ The `transport` handlers that act over `HubState`.

use semio_hub_auth::AuthErrorCodeV1;
use semio_hub_state::HubState;
use semio_hub_auth::rate_limit::{RateLimitClassV1, RateLimitDecisionV1, RateLimitRefusalV1, RateLimitSubjectV1};
use axum::response::Response;
use semio_hub_directory_contract::SessionCapability;
use std::net::SocketAddr;
use axum::extract::State;
use semio_framework_trace::record::TraceOutcome;
use semio_hub_route_models::session_agent::auth_error_response;
use semio_hub_runtime::support::bearer;
use semio_hub_runtime::transport::{rate_limit_class, rate_limit_refusal_response};

//#region 🔖️Directory
/// 🚦️ Charges every rate-limited route family before its handler runs: credential sign-in
/// and sign-out, directory commands, invite redemption and socket-grant issuance. Each request is
/// charged to its remote address and, when it carries one, to its session capability's public
/// selector; the handler for `POST /auth/sessions` adds the claimed identity's own bucket, which
/// only exists after the body is decoded. Every other route passes through untouched.
pub async fn rate_limit_middleware(State(state): State<HubState>, request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let Some(class) = rate_limit_class(request.method(), request.uri().path()) else { return next.run(request).await };
    let mut subjects = Vec::with_capacity(2);
    match request.extensions().get::<axum::extract::ConnectInfo<SocketAddr>>() {
        Some(axum::extract::ConnectInfo(peer)) => subjects.push(RateLimitSubjectV1::remote_address(&peer.ip())),
        None => subjects.push(RateLimitSubjectV1::principal("connect-info-absent")),
    }
    if let Some(selector) = bearer(request.headers()).and_then(|token| SessionCapability::parse(&token).ok()).map(|capability| capability.selector().to_string()) {
        subjects.push(RateLimitSubjectV1::principal(&selector));
    }
    match state.rate_limits.admit(class, &subjects) {
        RateLimitDecisionV1::Admitted => next.run(request).await,
        RateLimitDecisionV1::Refused { retry_after_ms } => match class {
            RateLimitClassV1::Auth => {
                state.note("server.rate-limit", TraceOutcome::Refused, "auth");
                auth_error_response(AuthErrorCodeV1::RateLimited, Some(retry_after_ms))
            }
            _ => {
                state.note("server.rate-limit", TraceOutcome::Refused, class.as_str());
                rate_limit_refusal_response(RateLimitRefusalV1::new(class, retry_after_ms))
            }
        },
    }
}
//#endregion 🔖️Directory
