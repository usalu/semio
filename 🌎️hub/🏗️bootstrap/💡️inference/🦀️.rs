//! 💡️ `inference` — GIS map inference job routes.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use semio_hub::directory::model::DocumentScope;
use semio_hub::directory::HubDirectory;
use semio_hub::inference::runtime::{GisMapApprovalIngressAuthorityV1, InferenceApprovalRouteContextV1, InferenceRouteErrorV1};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::support::{now_ms, bearer};
use crate::state::HubState;
use crate::socket_grants::{SocketBindingKeyV1, SocketBindingGuardV1};
use crate::guard::{session_access_roles, access_permits_in_space};
use crate::directory_commands::{AuthedUser, resolve_bearer_user, revalidate_directory_caller};

//#region 💡️Inference
/// 🧾️ The one closed error body every inference route publishes; it never names a private object.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InferenceErrorBodyV1 {
    schema: &'static str,
    code: &'static str,
}

fn inference_error_response(error: InferenceRouteErrorV1) -> Response {
    let status = StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(InferenceErrorBodyV1 { schema: "semio.hub.inference-error/v1", code: error.code() })).into_response()
}

/// 🎟️ One private Hub approval admission retaining the exact sorted identity and space guards.
pub(crate) struct HubGisMapApprovalIngressAuthorityV1 {
    pub(crate) scope: DocumentScope,
    pub(crate) caller: AuthedUser,
    pub(crate) _guards: Vec<SocketBindingGuardV1>,
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

pub(crate) async fn revalidate_gis_map_approval_authority(state: &HubState, authority: &HubGisMapApprovalIngressAuthorityV1) -> Result<(), InferenceRouteErrorV1> {
    revalidate_directory_caller(state, &authority.caller).await.map_err(|status| if status == StatusCode::SERVICE_UNAVAILABLE { InferenceRouteErrorV1::Unavailable } else { InferenceRouteErrorV1::Denied })?;
    let role =
        tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.principal_role(&authority.scope.space_id, authority.caller.principal())).await.map_err(|_| InferenceRouteErrorV1::Unavailable)?.map_err(|_| InferenceRouteErrorV1::Unavailable)?;
    let roles = session_access_roles(authority.caller.session_kind, role);
    if !access_permits_in_space(state, &roles, DirectoryAccessActionV1::DocumentWrite, &authority.scope.space_id).await {
        return Err(InferenceRouteErrorV1::Denied);
    }
    Ok(())
}

pub(crate) async fn acquire_gis_map_approval_ingress(state: &HubState, scope: DocumentScope, token: Option<&str>) -> Result<Arc<HubGisMapApprovalIngressAuthorityV1>, InferenceRouteErrorV1> {
    let caller = resolve_bearer_user(state, token).await.ok_or(InferenceRouteErrorV1::Denied)?;
    let guards = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.socket_binding_gates.share_bindings(vec![
            SocketBindingKeyV1::User(caller.user_id.clone()),
            SocketBindingKeyV1::Session(caller.session_id.clone()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: scope.space_id.clone() },
            SocketBindingKeyV1::Membership { user_id: caller.user_id.clone(), space_id: scope.space_id.clone() },
        ]),
    )
    .await
    .map_err(|_| InferenceRouteErrorV1::Unavailable)?;
    let authority = Arc::new(HubGisMapApprovalIngressAuthorityV1 { scope, caller, _guards: guards });
    revalidate_gis_map_approval_authority(state, &authority).await?;
    Ok(authority)
}

fn inference_context<'a>(state: &'a HubState, space_id: &str, document_id: &str, token: &'a Option<String>) -> Result<semio_hub::inference::runtime::InferenceRouteContextV1<'a>, Response> {
    let runtime = state.inference_runtime.as_ref().ok_or_else(|| inference_error_response(InferenceRouteErrorV1::Unavailable))?;
    let scope = DocumentScope::new(space_id, document_id);
    Ok(semio_hub::inference::runtime::InferenceRouteContextV1 {
        runtime,
        directory: &state.directory,
        rebootstrap: &state.rebootstrap,
        document_write: state.socket_binding_gates.document_write(&scope),
        scope,
        token: token.as_deref(),
        now_ms: u64::try_from(now_ms()).unwrap_or(0),
    })
}

pub(crate) async fn post_inference_gis_map_job(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let token = bearer(&headers);
    let context = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match semio_hub::inference::runtime::submit_gis_map_job(context, &body).await {
        Ok(receipt) => Json(receipt).into_response(),
        Err(error) => inference_error_response(error),
    }
}

pub(crate) async fn post_inference_gis_map_job_reconcile(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let token = bearer(&headers);
    let context = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match semio_hub::inference::runtime::reconcile_gis_map_job(context, &body).await {
        Ok(result) => Json(result).into_response(),
        Err(error) => inference_error_response(error),
    }
}

pub(crate) async fn get_inference_gis_map_job_events(Path((space_id, document_id, job_id)): Path<(String, String, String)>, Query(query): Query<InferenceEventQueryV1>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let token = bearer(&headers);
    let context = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match semio_hub::inference::runtime::read_gis_map_job_events(context, &job_id, query.after.unwrap_or(0)).await {
        Ok(page) => Json(page).into_response(),
        Err(error) => inference_error_response(error),
    }
}

/// 🔖️ The only inference query parameter: an exact bounded progress cursor.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InferenceEventQueryV1 {
    pub(crate) after: Option<u64>,
}

pub(crate) async fn post_inference_gis_map_job_cancel(Path((space_id, document_id, job_id)): Path<(String, String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let token = bearer(&headers);
    let context = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match semio_hub::inference::runtime::cancel_gis_map_job(context, &job_id).await {
        Ok(page) => Json(page).into_response(),
        Err(error) => inference_error_response(error),
    }
}

/// 🗳️ Approves one offered proposal. The caller's authority is validated once at ingress and its bindings stay
/// held (shared) until the answer is sent, so no revocation can interleave with the commit; once the commit is
/// durable the answer is its receipt. A second directory check after the commit could only fail for availability
/// (a 2 s directory read on a loaded hub) and answered `503 inference.unavailable` for a committed approval — the
/// document advanced while the caller was told nothing happened (ticket 26/09/23 G10, 7800 quartet runs 3/5/6).
pub(crate) async fn post_inference_gis_map_job_approval(Path((space_id, document_id, job_id)): Path<(String, String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let token = bearer(&headers);
    let route = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let ingress = match acquire_gis_map_approval_ingress(&state, route.scope.clone(), token.as_deref()).await {
        Ok(ingress) => ingress,
        Err(error) => return inference_error_response(error),
    };
    let context = InferenceApprovalRouteContextV1 { route, ingress };
    match semio_hub::inference::runtime::approve_gis_map_job(context, &job_id, &body).await {
        Ok(receipt) => Json(receipt).into_response(),
        Err(error) => inference_error_response(error),
    }
}

/// ↩️ Undoes one approval under the same ingress law as [`post_inference_gis_map_job_approval`]: a durable
/// undo is answered with its receipt.
pub(crate) async fn post_inference_gis_map_approval_undo(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Response {
    let token = bearer(&headers);
    let route = match inference_context(&state, &space_id, &document_id, &token) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let ingress = match acquire_gis_map_approval_ingress(&state, route.scope.clone(), token.as_deref()).await {
        Ok(ingress) => ingress,
        Err(error) => return inference_error_response(error),
    };
    let context = InferenceApprovalRouteContextV1 { route, ingress };
    match semio_hub::inference::runtime::undo_gis_map_approval(context, &body).await {
        Ok(receipt) => Json(receipt).into_response(),
        Err(error) => inference_error_response(error),
    }
}

pub(crate) const INFERENCE_REQUEST_MAX_BYTES: usize = 1024;
//#endregion 💡️Inference
