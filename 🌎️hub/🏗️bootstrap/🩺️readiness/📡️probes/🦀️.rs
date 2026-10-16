//! 🛣️ The `readiness` handlers that act over `HubState`.

use semio_hub_runtime::readiness::{HubClosedReadinessGateV1, hub_liveness};
use semio_hub_state::HubState;
use axum::response::IntoResponse;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

//#region 🔖️AdminPage
pub async fn get_healthz(State(state): State<HubState>) -> impl IntoResponse {
    Json(hub_liveness(&state.readiness.run_id))
}

pub async fn get_readyz(State(state): State<HubState>) -> impl IntoResponse {
    let mut readiness = (*state.readiness).clone();
    readiness.artifact_cas_sweeper.ready = state.artifact_maintenance.healthy();
    if !readiness.artifact_cas_sweeper.ready {
        readiness.status = "not-ready";
        readiness.artifact_cas_sweeper.reason = Some("artifact-cas-maintenance-supervisor-failed-closed");
        readiness.blocked_by.push(HubClosedReadinessGateV1 { gate: "artifactCasSweeper", reason: "artifact-cas-maintenance-supervisor-failed-closed" });
    }
    let status = if readiness.status == "ready" { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (status, Json(readiness))
}
//#endregion 🔖️AdminPage
