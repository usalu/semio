//! 🛣️ `routes` — the route table.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use directory::os_directory::schema::space_artifact_creation::SPACE_ARTIFACT_CREATION_MAX_BYTES;
use directory::os_directory;
use directory::os_directory::{DOCUMENT_CHECK_IN_MAX_BYTES, DIRECTORY_COMMAND_REQUEST_MAX_BYTES};
use semio_hub::artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub::directory::ADMIN_INTENT_REQUEST_MAX_BYTES;
use crate::state::HubState;
use crate::readiness::{HUB_PROCESS_START, get_healthz, get_readyz};
use crate::transport::{ForwardedTlsTrustV1, refusal_middleware, transport_security_middleware, rate_limit_middleware, CrossOriginPolicyV1, cors_middleware, route_metrics_middleware};
use crate::access_grants::{issue_document_open_plan, issue_document_execution_target_manifest, issue_document_execution_target_component, issue_document_execution_target_descriptor, issue_document_execution_target_browser_actor, issue_document_plan_socket_grant, issue_directory_socket_grant, issue_scoped_directory_socket_grant};
use crate::pair_blobs::{get_active_checkpoint_pair, get_document_status, put_blob, get_blob, head_blob};
use crate::check_in::{post_document_check_in, get_document_check_in, post_document_check_in_cancel};
use crate::document_socket::document_ws_v1;
use crate::directory_commands::{post_directory_commands, get_directory_spaces, get_directory_space, post_redeem_invite};
use crate::directory_events::{get_directory_event_page_v1, get_directory_preference_page_v1, get_directory_events, directory_ws_v1, directory_scoped_ws_v1};
use crate::artifact_creation::{get_space_artifact_creation_catalog, post_space_artifact_creation, get_space_artifact_creation_status, post_space_artifact_creation_cancel};
use crate::session_agent::{get_session_me, post_session_sign_out, post_auth_session, post_auth_credential, post_agent_delegation, get_agent_delegations, post_agent_delegation_revoke, post_agent_session};
use crate::admin::{admin_operation, cancel_admin_operation, admin_operation_audit, admin_intents, admin_overview, admin_observability, admin_spaces, admin_space, admin_users, admin_connections, admin_documents, admin_events};
use crate::assets::{list_extensions, get_extension_asset, get_trusted_plugin_module_index, get_trusted_plugin_module_manifest, get_trusted_plugin_module_file, get_admin_root, get_admin_asset};
use crate::inference::{post_inference_gis_map_job, post_inference_gis_map_job_reconcile, get_inference_gis_map_job_events, post_inference_gis_map_job_cancel, post_inference_gis_map_job_approval, post_inference_gis_map_approval_undo, INFERENCE_REQUEST_MAX_BYTES};
use db::db_storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 💡️Inference
fn inference_routes(router: Router<HubState>) -> Router<HubState> {
    router
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs", post(post_inference_gis_map_job).layer(DefaultBodyLimit::max(INFERENCE_REQUEST_MAX_BYTES)))
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/reconcile", post(post_inference_gis_map_job_reconcile).layer(DefaultBodyLimit::max(semio_framework_job::reconcile::JOB_RECONCILE_REQUEST_MAX_BYTES)))
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/{job_id}/events", get(get_inference_gis_map_job_events))
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/{job_id}/cancel", post(post_inference_gis_map_job_cancel).layer(DefaultBodyLimit::max(INFERENCE_REQUEST_MAX_BYTES)))
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs/{job_id}/approval", post(post_inference_gis_map_job_approval).layer(DefaultBodyLimit::max(INFERENCE_REQUEST_MAX_BYTES)))
        .route("/spaces/{space_id}/documents/{document_id}/inference/gis-map/approval-undos", post(post_inference_gis_map_approval_undo).layer(DefaultBodyLimit::max(INFERENCE_REQUEST_MAX_BYTES)))
}
//#endregion 💡️Inference

//#region 🔖️Main
fn artifact_creation_routes(router: Router<HubState>) -> Router<HubState> {
    router
        .route("/spaces/{space_id}/artifact-creations", get(get_space_artifact_creation_catalog).post(post_space_artifact_creation).layer(DefaultBodyLimit::max(SPACE_ARTIFACT_CREATION_MAX_BYTES)))
        .route("/spaces/{space_id}/artifact-creations/{request_id}", get(get_space_artifact_creation_status))
        .route("/spaces/{space_id}/artifact-creations/{request_id}/cancel", post(post_space_artifact_creation_cancel).layer(DefaultBodyLimit::max(0)))
}

/// 🐙️ w4-h: router-wide CORS grant — see `cors_middleware`'s doc comment (`🔖️Directory` region)
/// for why this must cover the whole router, not just `/directory/*`.
/// 🚦️ Inside the CORS layer, so a refused request never reaches a handler and its `429`
/// still leaves through `cors_middleware` with the headers a browser needs to read it.
///
/// 🛡️ Outermost: a request a trusted proxy reports as cleartext is refused before CORS, rate
/// limiting or any handler sees it — a compromised transport is not a per-route question.
///
/// 🚧️ Outermost of all: every answer, the transport refusal included, leaves typed and signed.
pub(crate) fn router(state: HubState, cross_origin: CrossOriginPolicyV1, forwarded_tls: ForwardedTlsTrustV1) -> Router {
    std::sync::LazyLock::force(&HUB_PROCESS_START);
    artifact_creation_routes(inference_routes(Router::new()))
        .route("/healthz", get(get_healthz))
        .route("/readyz", get(get_readyz))
        .route(semio_hub::auth::SESSION_MINT_ROUTE, post(post_auth_session).layer(DefaultBodyLimit::max(semio_hub::auth::SIGN_IN_REQUEST_MAX_BYTES)))
        .route(semio_hub::auth::CREDENTIAL_ROUTE, post(post_auth_credential).layer(DefaultBodyLimit::max(semio_hub::auth::CREDENTIAL_CHANGE_REQUEST_MAX_BYTES)))
        .route("/auth/sessions/me", get(get_session_me))
        .route("/auth/sessions/me/sign-out", post(post_session_sign_out).layer(DefaultBodyLimit::max(0)))
        .route(semio_hub::auth::agent::AGENT_DELEGATION_ROUTE, post(post_agent_delegation).get(get_agent_delegations).layer(DefaultBodyLimit::max(semio_hub::auth::agent::AGENT_DELEGATION_REQUEST_MAX_BYTES)))
        .route(semio_hub::auth::agent::AGENT_DELEGATION_REVOKE_ROUTE, post(post_agent_delegation_revoke).layer(DefaultBodyLimit::max(0)))
        .route(semio_hub::auth::agent::AGENT_SESSION_ROUTE, post(post_agent_session).layer(DefaultBodyLimit::max(semio_hub::auth::agent::AGENT_SESSION_REQUEST_MAX_BYTES)))
        .route("/directory/commands", post(post_directory_commands).layer(DefaultBodyLimit::max(DIRECTORY_COMMAND_REQUEST_MAX_BYTES)))
        .route("/directory/spaces", get(get_directory_spaces))
        .route("/directory/spaces/{id}", get(get_directory_space))
        .route("/directory/invites/{token}/redeem", post(post_redeem_invite))
        .route("/directory/events", get(get_directory_events))
        .route("/directory/event-page/v1", get(get_directory_event_page_v1))
        .route(os_directory::DIRECTORY_PREFERENCE_PAGE_PATH_V1, get(get_directory_preference_page_v1))
        .route("/directory/socket-grants", post(issue_directory_socket_grant).layer(DefaultBodyLimit::max(0)))
        .route("/directory/socket/v1", get(directory_ws_v1))
        .route("/directory/spaces/{space_id}/documents/{document_id}/socket-grants", post(issue_scoped_directory_socket_grant).layer(DefaultBodyLimit::max(256)))
        .route("/directory/spaces/{space_id}/documents/{document_id}/socket/v1", get(directory_scoped_ws_v1))
        .route("/admin/api/overview", get(admin_overview))
        .route("/admin/api/spaces", get(admin_spaces))
        .route("/admin/api/spaces/{id}", get(admin_space))
        .route("/admin/api/users", get(admin_users))
        .route("/admin/api/connections", get(admin_connections))
        .route("/admin/api/documents", get(admin_documents))
        .route("/admin/api/events", get(admin_events))
        .route("/admin/api/operations/{operation_id}", get(admin_operation))
        .route("/admin/api/operations/{operation_id}/cancel", post(cancel_admin_operation))
        .route("/admin/api/audit", get(admin_operation_audit))
        .route("/admin/api/observability", get(admin_observability))
        .route("/admin/api/intents", post(admin_intents).layer(DefaultBodyLimit::max(ADMIN_INTENT_REQUEST_MAX_BYTES)))
        .route("/%F0%9F%A7%A9%EF%B8%8Fextension-modules", get(list_extensions))
        .route("/%F0%9F%A7%A9%EF%B8%8Fextension-modules/{extension_id}/{*rest}", get(get_extension_asset))
        .route("/trusted-catalog/plugin-modules", get(get_trusted_plugin_module_index))
        .route("/trusted-catalog/plugin-modules/{bundle_sha256}", get(get_trusted_plugin_module_manifest))
        .route("/trusted-catalog/plugin-modules/{bundle_sha256}/{*path}", get(get_trusted_plugin_module_file))
        .route("/admin", get(get_admin_root))
        .route("/admin/", get(get_admin_root))
        .route("/admin/{*path}", get(get_admin_asset))
        .route("/spaces/{space_id}/blobs/{hash}", get(get_blob).head(head_blob).put(put_blob))
        .route("/spaces/{space_id}/documents/{id}", get(get_document_status))
        .route("/spaces/{space_id}/documents/{document_id}/active-checkpoint/pair", get(get_active_checkpoint_pair))
        .route("/spaces/{space_id}/documents/{document_id}/check-ins", post(post_document_check_in).layer(DefaultBodyLimit::max(DOCUMENT_CHECK_IN_MAX_BYTES)))
        .route("/spaces/{space_id}/documents/{document_id}/check-ins/{request_id}", get(get_document_check_in))
        .route("/spaces/{space_id}/documents/{document_id}/check-ins/{request_id}/cancel", post(post_document_check_in_cancel).layer(DefaultBodyLimit::max(0)))
        .route("/spaces/{space_id}/documents/{id}/open-plan", post(issue_document_open_plan))
        .route("/spaces/{space_id}/documents/{id}/socket-grants", post(issue_document_plan_socket_grant))
        .route("/scopes/{scope}/document/ws", get(document_ws_v1))
        .route("/spaces/{space_id}/documents/{id}/execution-target/manifest", post(issue_document_execution_target_manifest))
        .route("/spaces/{space_id}/documents/{id}/execution-target/component", post(issue_document_execution_target_component))
        .route("/spaces/{space_id}/documents/{id}/execution-target/descriptor", post(issue_document_execution_target_descriptor))
        .route("/spaces/{space_id}/documents/{id}/execution-target/browser-actor", post(issue_document_execution_target_browser_actor))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
        .route_layer(axum::middleware::from_fn_with_state(state.route_metrics.clone(), route_metrics_middleware))
        .layer(axum::middleware::from_fn_with_state(cross_origin, cors_middleware))
        .layer(axum::middleware::from_fn_with_state(forwarded_tls, transport_security_middleware))
        .layer(axum::middleware::from_fn_with_state(state.tracer.clone(), refusal_middleware))
        .with_state(state)
}
//#endregion 🔖️Main
