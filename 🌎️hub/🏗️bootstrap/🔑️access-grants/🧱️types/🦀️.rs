//! 🧱️ The `access_grants` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use semio_hub_directory_contract::AUTH_TEXT_MAX_BYTES;
use semio_hub_runtime::support::DirectoryJson;
use directory::os_directory::{DocumentOpenCheckpointV1, DocumentOpenPlanErrorCodeV1, DocumentOpenPlanErrorV1, DocumentScope, PublishedArtifactCheckpoint};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::Serialize;
use semio_framework_hash::Sha256;
use semio_hub_auth::rate_limit::StreamPermitV1;
use semio_hub_trusted_catalog_descriptor::TrustedCatalogAsset;
use semio_hub_runtime::presence::{db_artifact_id, document_scope_key_v1};
use directory::os_directory;

//#region 🔖️State
pub const DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES: usize = 8 * 1024;

pub const DOCUMENT_OPEN_PLAN_DEADLINE_MS: u64 = 10_000;

pub const DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES: usize = 8 * 1024;
//#endregion 🔖️State

//#region 🔖️Rest
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocketGrantReceiptV1 {
    pub schema: &'static str,
    pub protocol: &'static str,
    pub grant: String,
    pub actor_id: String,
    pub expires_at_ms: i64,
}

/// 📝️ Schema of [`DocumentSocketGrantReceiptV1`] — `os.directory#/$defs/DocumentSocketGrantReceiptV1`.
pub const DOCUMENT_SOCKET_GRANT_SCHEMA_V1: &str = "semio.hub.document-socket-grant/v1";

/// 📝️ The answer to a document open-plan exchange: the actor the next `semio.session.v1`
/// upgrade of the same credential is admitted as, and until when. It carries no secret — the
/// credential itself is what the upgrade presents.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSocketGrantReceiptV1 {
    pub schema: &'static str,
    pub protocol: &'static str,
    pub actor_id: String,
    pub expires_at_ms: i64,
}

pub fn socket_issue_bearer(headers: &HeaderMap) -> Result<String, StatusCode> {
    let values = headers.get_all(axum::http::header::AUTHORIZATION);
    if values.iter().count() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let encoded = values.iter().next().and_then(|value| value.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    if encoded.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let capability = encoded.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    if capability.is_empty() || capability.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(capability.to_string())
}

/// 🙋️ The hub actor id bound to one credential: a session or a share token hashes to the same
/// actor on every open-plan, socket grant and socket upgrade, so no caller ever names its actor.
pub fn socket_actor_id(material: &[u8; 32], stable_session: bool) -> String {
    let mut digest = Sha256::new();
    digest.update(b"semio/hub/socket/actor/v1\0");
    digest.update(if stable_session { b"session".as_slice() } else { b"share".as_slice() });
    digest.update(material);
    format!("hub.v1.{}", os_directory::io::binary::artifact_hash::hex_lower(&digest.finalize()))
}

pub fn socket_text_bounded(value: &str) -> bool {
    !value.is_empty() && value.len() <= AUTH_TEXT_MAX_BYTES
}

pub type DocumentOpenPlanRouteError = (StatusCode, DirectoryJson<DocumentOpenPlanErrorV1>);

pub fn document_open_plan_route_error(status: StatusCode, code: DocumentOpenPlanErrorCodeV1) -> DocumentOpenPlanRouteError {
    (status, DirectoryJson(DocumentOpenPlanErrorV1 { schema: "semio.hub.document-open-plan-error/v1".into(), code }))
}

pub fn document_open_plan_exchange_error(code: DocumentOpenPlanErrorCodeV1) -> DocumentOpenPlanRouteError {
    let status = match code {
        DocumentOpenPlanErrorCodeV1::Denied => StatusCode::UNAUTHORIZED,
        DocumentOpenPlanErrorCodeV1::NotFound => StatusCode::NOT_FOUND,
        DocumentOpenPlanErrorCodeV1::Expired => StatusCode::GONE,
        DocumentOpenPlanErrorCodeV1::Stale | DocumentOpenPlanErrorCodeV1::AlreadyConsumed => StatusCode::CONFLICT,
        DocumentOpenPlanErrorCodeV1::Cancelled => StatusCode::REQUEST_TIMEOUT,
        DocumentOpenPlanErrorCodeV1::CatalogUnavailable | DocumentOpenPlanErrorCodeV1::ComponentUnavailable | DocumentOpenPlanErrorCodeV1::DeadlineExceeded => StatusCode::SERVICE_UNAVAILABLE,
    };
    document_open_plan_route_error(status, code)
}

pub fn document_open_checkpoint(checkpoint: PublishedArtifactCheckpoint) -> DocumentOpenCheckpointV1 {
    DocumentOpenCheckpointV1 {
        checkpoint_id: os_directory::io::binary::artifact_hash::hex_lower(&checkpoint.checkpoint_id.0),
        descriptor_digest_v1: os_directory::io::binary::artifact_hash::hex_lower(&checkpoint.descriptor_digest_v1.0),
        baseline_frontier: checkpoint.baseline_frontier,
        aggregate_sha256: os_directory::io::binary::artifact_hash::hex_lower(&checkpoint.aggregate_sha256.0),
    }
}
//#endregion 🔖️Rest

//#region 🔖️Rest
//#region 🪪️ExecutionTargetLease
pub const DOCUMENT_EXECUTION_TARGET_REQUEST_MAX_BYTES: usize = 8 * 1024;

pub const DOCUMENT_EXECUTION_TARGET_DEADLINE_MS: u64 = 8_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentExecutionTargetAssetV1 {
    Manifest,
    Component,
    Descriptor,
    BrowserActor,
}

/// 🌊️ Streams one catalog-verified execution-target asset (the document's component or browser actor, tens of MB) from
/// the generation it was verified in. The route's deadline bounds the selection that authorizes it, never the transfer:
/// reading and hashing the whole asset under that deadline answered 503 for every large component once the hub was busy
/// (ticket 26/09/23 S15: catalog B2 puzzle3d, `execution-target/component` 503 after a slow creation; the plugin-module
/// twin of this defect was S12-1b). The stream withholds its last chunk until length, SHA-256 and BLAKE3 match, so a
/// tampered asset ends the body short instead of completing it. It holds `permit`, its slot of the asset stream rule, until
/// the body ends or the client drops it.
pub fn document_execution_target_stream(asset: &TrustedCatalogAsset, permit: StreamPermitV1) -> Result<Response, DocumentOpenPlanRouteError> {
    let stream = asset.stream().map_err(|_| document_open_plan_exchange_error(DocumentOpenPlanErrorCodeV1::ComponentUnavailable))?;
    let length = stream.byte_length();
    let body = axum::body::Body::from_stream(futures::stream::unfold((stream, permit), |(mut stream, permit)| async move {
        stream.next_chunk().await.map(|chunk| (chunk.map_err(|error| std::io::Error::other(error.to_string())), (stream, permit)))
    }));
    Ok((StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/octet-stream".to_string()), (axum::http::header::CONTENT_LENGTH, length.to_string()), (axum::http::header::CACHE_CONTROL, "no-store".to_string())], body).into_response())
}

pub fn document_execution_target_bytes(bytes: &[u8]) -> Response {
    (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/octet-stream".to_string()), (axum::http::header::CONTENT_LENGTH, bytes.len().to_string()), (axum::http::header::CACHE_CONTROL, "no-store".to_string())], bytes.to_vec())
        .into_response()
}
//#endregion 🪪️ExecutionTargetLease
//#endregion 🔖️Rest

//#region 🔖️Rest
#[test]
fn document_scope_key_v1_is_length_prefixed_and_never_colon_ambiguous() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🚧️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
    let vectors = fixture["documentScopeKeyV1"].as_array().expect("scope key vectors");
    let mut encoded = std::collections::HashSet::new();
    for vector in vectors {
        let scope = DocumentScope::new(vector["scope"]["spaceId"].as_str().unwrap(), vector["scope"]["documentId"].as_str().unwrap());
        let actual = document_scope_key_v1(&scope);
        assert_eq!(actual, vector["encoded"].as_str().unwrap());
        assert!(encoded.insert(actual), "scope vector aliased");
    }
    assert_ne!(db_artifact_id(&DocumentScope::new("space-a", "shared")), db_artifact_id(&DocumentScope::new("space-b", "shared")));
}
//#endregion 🔖️Rest
