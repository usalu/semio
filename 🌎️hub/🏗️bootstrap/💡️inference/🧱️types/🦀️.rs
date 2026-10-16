//! 🧱️ The `inference` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use crate::directory_commands::AuthedUser;
use serde::{Deserialize, Serialize};
use directory::os_directory::DocumentScope;
use semio_hub_inference_runtime_contract::{GisMapApprovalIngressAuthorityV1, InferenceRouteErrorV1};
use axum::Json;
use axum::response::Response;
use semio_hub_ledgers::socket_grants::SocketBindingGuardV1;
use axum::http::StatusCode;

//#region 💡️Inference
/// 🧾️ The one closed error body every inference route publishes; it never names a private object.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InferenceErrorBodyV1 {
    schema: &'static str,
    code: &'static str,
}

pub fn inference_error_response(error: InferenceRouteErrorV1) -> Response {
    let status = StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(InferenceErrorBodyV1 { schema: "semio.hub.inference-error/v1", code: error.code() })).into_response()
}

/// 🎟️ One private Hub approval admission retaining the exact sorted identity and space guards.
pub struct HubGisMapApprovalIngressAuthorityV1 {
    pub scope: DocumentScope,
    pub caller: AuthedUser,
    pub _guards: Vec<SocketBindingGuardV1>,
}

impl GisMapApprovalIngressAuthorityV1 for HubGisMapApprovalIngressAuthorityV1 {
    fn scope(&self) -> &DocumentScope {
        &self.scope
    }

    fn user_id(&self) -> &str {
        &self.caller.user_id
    }

    fn session_id(&self) -> &str {
        &self.caller.session_id
    }

    fn authorization_generation(&self) -> u64 {
        self.caller.authorization_generation
    }
}

/// 🔖️ The only inference query parameter: an exact bounded progress cursor.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InferenceEventQueryV1 {
    pub after: Option<u64>,
}

pub const INFERENCE_REQUEST_MAX_BYTES: usize = 1024;
//#endregion 💡️Inference
