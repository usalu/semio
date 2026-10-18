//! 🛣️ The `pair_blobs` handlers that act over `HubState`.

use std::sync::Arc;
use semio_hub_ledgers::pair_blobs::{BlobRecord, CanonicalPairHttpControl, CanonicalPairHttpRequest, DocumentStatusResponse, canonical_pair_error_status, canonical_pair_request_admission, db_io_pages_into_http_bytes};
use axum::body::Bytes;
use semio_hub_lag_rebootstrap::{CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE, CanonicalPairTerminal, REBOOTSTRAP_DEADLINE_MS, RebootstrapContext, RebootstrapProgress, RebootstrapProgressStage, RebootstrapTransferControl, append_canonical_pair_data, append_canonical_pair_header, append_canonical_pair_terminal, canonical_pair_etag};
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use directory::os_directory::DocumentScope;
use axum::http::{HeaderMap, StatusCode};
use semio_hub_state::HubState;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::extract::{OriginalUri, Path, State};
use semio_hub_session_routes::guard::{authorized, authorized_for_blob, authorized_for_canonical_pair};
use semio_hub_runtime::support::{bearer, db_error_status, directory_error_status, parse_content_hash};
use semio_hub_runtime::presence::db_artifact_id;
use semio_hub_directory::DocumentDirectory;

//#region 🔖️Rest
/// 🧭️ Exact authenticated, path-only public projection of one active canonical checkpoint pair.
pub async fn get_active_checkpoint_pair(Path((space_id, document_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let token = match canonical_pair_request_admission(&uri, &headers) {
        Ok(token) => token,
        Err(status) => return status.into_response(),
    };
    let scope = DocumentScope::new(space_id, document_id);
    let control = Arc::new(CanonicalPairHttpControl::new());
    let mut request = CanonicalPairHttpRequest::new(control.clone());
    let deadline_ms = REBOOTSTRAP_DEADLINE_MS;
    #[cfg(test)]
    let deadline_ms = state.canonical_pair_deadline_ms.unwrap_or(deadline_ms);
    let operation = async {
        #[cfg(test)]
        if let Some(gate) = &state.canonical_pair_request_gate {
            gate.enter(control.clone()).await;
        }
        canonical_pair_response(&state, &scope, &token, control.as_ref(), deadline_ms).await
    };
    let response = match tokio::time::timeout(std::time::Duration::from_millis(deadline_ms), operation).await {
        Ok(response) => response,
        Err(_) => {
            control.request_cancel();
            StatusCode::GATEWAY_TIMEOUT.into_response()
        }
    };
    request.finish_response_owned();
    response
}

async fn canonical_pair_response(state: &HubState, scope: &DocumentScope, token: &str, control: &CanonicalPairHttpControl, deadline_ms: u64) -> Response {
    let deadline = control.now_ms().saturating_add(deadline_ms);
    let context = RebootstrapContext::new(deadline, control);
    if let Err(error) = context.checkpoint() {
        return canonical_pair_error_status(error).into_response();
    }
    if !authorized_for_canonical_pair(state, scope, token).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    control.report(RebootstrapProgress { stage: RebootstrapProgressStage::Authorize, completed_units: 1, total_units: 1 });
    if let Err(error) = context.checkpoint() {
        return canonical_pair_error_status(error).into_response();
    }
    let pair = match state.rebootstrap.active_pair(scope, &context).await {
        Ok(pair) => pair,
        Err(error) => return canonical_pair_error_status(error).into_response(),
    };
    if !authorized_for_canonical_pair(state, scope, token).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let mut body = Vec::new();
    if let Err(error) = append_canonical_pair_header(&mut body, &pair.selection) {
        return canonical_pair_error_status(error).into_response();
    }
    for ordinal in 0..pair.data_record_count() {
        if !authorized_for_canonical_pair(state, scope, token).await {
            return StatusCode::UNAUTHORIZED.into_response();
        }
        let record = match pair.data_record(ordinal, &context) {
            Ok(Some(record)) => record,
            Ok(None) => return StatusCode::CONFLICT.into_response(),
            Err(error) => return canonical_pair_error_status(error).into_response(),
        };
        if let Err(error) = append_canonical_pair_data(&mut body, &record) {
            return canonical_pair_error_status(error).into_response();
        }
    }
    if !authorized_for_canonical_pair(state, scope, token).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    if let Err(error) = append_canonical_pair_terminal(&mut body, CanonicalPairTerminal::Complete) {
        return canonical_pair_error_status(error).into_response();
    }
    let etag = match canonical_pair_etag(&pair.selection).ok().and_then(|value| value.parse().ok()) {
        Some(value) => value,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let mut response = Bytes::from(body).into_response();
    response.headers_mut().insert(axum::http::header::CONTENT_TYPE, axum::http::HeaderValue::from_static(CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE));
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("private, no-store"));
    response.headers_mut().insert(axum::http::header::VARY, axum::http::HeaderValue::from_static("Authorization"));
    response.headers_mut().insert(axum::http::header::ETAG, etag);
    control.report(RebootstrapProgress { stage: RebootstrapProgressStage::Ready, completed_units: 4, total_units: 4 });
    if let Err(error) = context.checkpoint() {
        return canonical_pair_error_status(error).into_response();
    }
    response
}

/// 🧭️ A durably announced document's current frontier.
pub async fn get_document_status(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<DocumentStatusResponse>, StatusCode> {
    if !authorized(&state, &space_id, &document_id, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let scope = DocumentScope::new(space_id, document_id);
    if state.directory.get_document_descriptor(&scope).await.map_err(directory_error_status)?.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    let handle = state.ensure_document(&db_artifact_id(&scope)).await.map_err(|e| db_error_status(&e))?;
    let frontier = handle.frontier().await.map_err(|e| db_error_status(&e))?;
    Ok(Json(DocumentStatusResponse { document_id: scope.document_id, head_seq: frontier.head_seq, commit_seq: frontier.commit_seq, epoch: frontier.epoch }))
}
//#endregion 🔖️Rest

//#region 🔖️Rest
//#region Blobs
/// 🔏️ The path hash is client-supplied (content-addressed URL); a mismatch against the
/// storage-computed hash means the client sent the wrong bytes for that address — a bad
/// request, distinct from a document CAS conflict.
pub async fn put_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<BlobRecord>, StatusCode> {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref(), DirectoryAccessActionV1::BlobWrite).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let media_type = headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()).unwrap_or("application/octet-stream").to_string();
    let size = body.len();
    let pages = db::storage::db_io_copy_pages(body.as_ref()).map_err(|error| db_error_status(&error))?.await.map_err(|error| db_error_status(&error))?;
    let computed = state.db.storage().await.payload().await.put(pages).await.map_err(|error| db_error_status(&error))?;
    let computed_hex = computed.to_string();
    if computed_hex != hash {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(BlobRecord { hash: computed_hex, media_type, size: size as i64 }))
}

pub async fn get_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<impl IntoResponse, StatusCode> {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref(), DirectoryAccessActionV1::BlobRead).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let content_hash = parse_content_hash(&hash).ok_or(StatusCode::BAD_REQUEST)?;
    match state.db.storage().await.payload().await.get(&content_hash).await {
        Ok(pages) => {
            let bytes = db_io_pages_into_http_bytes(pages).await.map_err(|error| db_error_status(&error))?;
            Ok(([(axum::http::header::CONTENT_TYPE, "application/octet-stream")], bytes))
        }
        Err(error) => Err(db_error_status(&error)),
    }
}

pub async fn head_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> StatusCode {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref(), DirectoryAccessActionV1::BlobRead).await {
        return StatusCode::UNAUTHORIZED;
    }
    let Some(content_hash) = parse_content_hash(&hash) else { return StatusCode::BAD_REQUEST };
    match state.db.storage().await.payload().await.contains(&content_hash).await {
        Ok(true) => StatusCode::OK,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
//#endregion Blobs
//#endregion 🔖️Rest
