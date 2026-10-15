//! 🪞️ `pair_blobs` — canonical checkpoint pair, document status and blob routes.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::body::Bytes;
use axum::extract::{OriginalUri, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use futures::StreamExt;
use semio_hub::artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub::artifact_authority::creation::ArtifactCreationCommitAuthorityV1;
#[cfg(test)]
use semio_hub::artifact_authority::ImmutableArtifactBlobStore;
use semio_hub::artifact_authority::AuthorityOperationControl;
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use semio_hub::directory::model::DocumentScope;
use semio_hub::directory::{HubDirectory, ProjectionRebuildControl, HubCapability, IdentityVerificationControl, AUTH_TEXT_MAX_BYTES};
use semio_hub::lag_rebootstrap::{append_canonical_pair_data, append_canonical_pair_header, append_canonical_pair_terminal, canonical_pair_etag, CanonicalPairTerminal, RebootstrapContext, RebootstrapError, RebootstrapProgress, RebootstrapProgressStage, RebootstrapTransferControl, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE, REBOOTSTRAP_DEADLINE_MS};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use crate::support::{now_ms, bearer, db_error_status, parse_content_hash, directory_error_status};
use crate::presence::db_artifact_id;
use crate::state::HubState;
use crate::guard::{authorized, authorized_for_blob, authorized_for_canonical_pair};
use db::db_storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 🔖️Rest
#[derive(Serialize)]
pub(crate) struct DocumentStatusResponse {
    pub(crate) document_id: String,
    pub(crate) head_seq: u64,
    pub(crate) commit_seq: u64,
    pub(crate) epoch: u64,
}

#[derive(Serialize)]
pub(crate) struct BlobRecord {
    pub(crate) hash: String,
    media_type: String,
    pub(crate) size: i64,
}

fn canonical_pair_bearer(headers: &HeaderMap) -> Result<String, StatusCode> {
    let values = headers.get_all(axum::http::header::AUTHORIZATION);
    if values.iter().count() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let encoded = values.iter().next().and_then(|value| value.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    if encoded.len() > AUTH_TEXT_MAX_BYTES {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let capability = encoded.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    if capability.is_empty() || capability.len() > AUTH_TEXT_MAX_BYTES || !matches!(HubCapability::parse(capability), Ok(HubCapability::Session(_)) | Ok(HubCapability::Share(_))) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(capability.to_string())
}

pub(crate) fn canonical_pair_request_admission(uri: &axum::http::Uri, headers: &HeaderMap) -> Result<String, StatusCode> {
    if uri.query().is_some() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if headers.contains_key(axum::http::header::RANGE) {
        return Err(StatusCode::RANGE_NOT_SATISFIABLE);
    }
    if headers.get_all(axum::http::header::ACCEPT).iter().count() != 1 || headers.get(axum::http::header::ACCEPT).and_then(|value| value.to_str().ok()) != Some(CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE) {
        return Err(StatusCode::NOT_ACCEPTABLE);
    }
    canonical_pair_bearer(headers)
}

pub(crate) const CANONICAL_PAIR_PROGRESS_STAGES: usize = 8;

pub(crate) fn canonical_pair_progress_index(stage: RebootstrapProgressStage) -> usize {
    match stage {
        RebootstrapProgressStage::Authorize => 0,
        RebootstrapProgressStage::Metadata => 1,
        RebootstrapProgressStage::VerifyPack => 2,
        RebootstrapProgressStage::VerifySpr => 3,
        RebootstrapProgressStage::StreamPack => 4,
        RebootstrapProgressStage::StreamSpr => 5,
        RebootstrapProgressStage::Ready => 6,
        RebootstrapProgressStage::Chunk => 7,
    }
}

pub(crate) struct CanonicalPairHttpControl {
    pub(crate) cancelled: std::sync::atomic::AtomicBool,
    pub(crate) active: std::sync::atomic::AtomicBool,
    pub(crate) progress: Mutex<[Option<RebootstrapProgress>; CANONICAL_PAIR_PROGRESS_STAGES]>,
}

impl CanonicalPairHttpControl {
    pub(crate) fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false), active: std::sync::atomic::AtomicBool::new(true), progress: Mutex::new([None; CANONICAL_PAIR_PROGRESS_STAGES]) }
    }

    pub(crate) fn request_cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    pub(crate) fn release(&self) {
        self.active.store(false, std::sync::atomic::Ordering::Release);
    }

    #[cfg(test)]
    pub(crate) fn is_active(&self) -> bool {
        self.active.load(std::sync::atomic::Ordering::Acquire)
    }

    #[cfg(test)]
    pub(crate) fn progress_snapshot(&self) -> [Option<RebootstrapProgress>; CANONICAL_PAIR_PROGRESS_STAGES] {
        *self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

struct CanonicalPairHttpRequest {
    control: Arc<CanonicalPairHttpControl>,
    response_owned: bool,
}

impl CanonicalPairHttpRequest {
    pub(crate) fn new(control: Arc<CanonicalPairHttpControl>) -> Self {
        Self { control, response_owned: false }
    }

    pub(crate) fn finish_response_owned(&mut self) {
        self.response_owned = true;
        self.control.release();
    }
}

impl Drop for CanonicalPairHttpRequest {
    fn drop(&mut self) {
        if !self.response_owned {
            self.control.request_cancel();
            self.control.release();
        }
    }
}

#[cfg(test)]
pub(crate) struct TestCanonicalPairRequestGate {
    pub(crate) entered: tokio::sync::Semaphore,
    pub(crate) release: tokio::sync::Semaphore,
    pub(crate) control: Mutex<Option<Arc<CanonicalPairHttpControl>>>,
}

#[cfg(test)]
impl TestCanonicalPairRequestGate {
    pub(crate) fn new(release_permits: usize) -> Self {
        Self { entered: tokio::sync::Semaphore::new(0), release: tokio::sync::Semaphore::new(release_permits), control: Mutex::new(None) }
    }

    async fn enter(&self, control: Arc<CanonicalPairHttpControl>) {
        *self.control.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(control);
        self.entered.add_permits(1);
        let _permit = self.release.acquire().await.expect("canonical pair request gate open");
    }

    pub(crate) fn control(&self) -> Arc<CanonicalPairHttpControl> {
        self.control.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone().expect("canonical pair request control captured")
    }
}

impl RebootstrapTransferControl for CanonicalPairHttpControl {
    fn now_ms(&self) -> u64 {
        u64::try_from(now_ms()).unwrap_or(u64::MAX)
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
    }

    fn report(&self, progress: RebootstrapProgress) {
        if progress.total_units == 0 || progress.completed_units > progress.total_units {
            return;
        }
        let mut stages = self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = &mut stages[canonical_pair_progress_index(progress.stage)];
        if slot.is_none_or(|prior| prior.total_units == progress.total_units && prior.completed_units <= progress.completed_units) {
            *slot = Some(progress);
        }
    }
}

fn canonical_pair_error_status(error: RebootstrapError) -> StatusCode {
    match error {
        RebootstrapError::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
        RebootstrapError::Unavailable => StatusCode::NOT_FOUND,
        RebootstrapError::ResourceLimit => StatusCode::PAYLOAD_TOO_LARGE,
        RebootstrapError::Cancelled => StatusCode::SERVICE_UNAVAILABLE,
        RebootstrapError::AuthorityIdentityChanged | RebootstrapError::Integrity => StatusCode::CONFLICT,
    }
}

/// 🧭️ Exact authenticated, path-only public projection of one active canonical checkpoint pair.
pub(crate) async fn get_active_checkpoint_pair(Path((space_id, document_id)): Path<(String, String)>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Response {
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
pub(crate) async fn get_document_status(Path((space_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<DocumentStatusResponse>, StatusCode> {
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

//#region Blobs
const HUB_BLOB_MAX_BYTES: usize = db::db_storage::DB_IO_PAGE_BYTES * db::db_storage::DB_IO_OPERATION_PAGES;

async fn db_io_pages_into_http_bytes(mut pages: db::db_storage::DbIoPages) -> Result<Bytes, db::DbError> {
    if pages.len() > HUB_BLOB_MAX_BYTES {
        while !pages.terminal_is_empty() {
            pages.close_step()?;
            semio_framework_async::yield_once().await;
        }
        return Err(db::DbError::LimitExceeded("hub blob response bytes"));
    }
    let mut body = Vec::with_capacity(pages.len());
    for fragment in pages.fragments() {
        body.extend_from_slice(fragment);
    }
    while !pages.terminal_is_empty() {
        pages.close_step()?;
        semio_framework_async::yield_once().await;
    }
    Ok(Bytes::from(body))
}

/// 🔏️ The path hash is client-supplied (content-addressed URL); a mismatch against the
/// storage-computed hash means the client sent the wrong bytes for that address — a bad
/// request, distinct from a document CAS conflict.
pub(crate) async fn put_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<BlobRecord>, StatusCode> {
    if !authorized_for_blob(&state, &space_id, &hash, bearer(&headers).as_deref(), DirectoryAccessActionV1::BlobWrite).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let media_type = headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()).unwrap_or("application/octet-stream").to_string();
    let size = body.len();
    let pages = db::db_storage::db_io_copy_pages(body.as_ref()).map_err(|error| db_error_status(&error))?.await.map_err(|error| db_error_status(&error))?;
    let computed = state.db.storage().await.payload().await.put(pages).await.map_err(|error| db_error_status(&error))?;
    let computed_hex = computed.to_string();
    if computed_hex != hash {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(BlobRecord { hash: computed_hex, media_type, size: size as i64 }))
}

pub(crate) async fn get_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<impl IntoResponse, StatusCode> {
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

pub(crate) async fn head_blob(Path((space_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> StatusCode {
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
