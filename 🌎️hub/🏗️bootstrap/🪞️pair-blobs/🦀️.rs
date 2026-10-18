//! 🪞️ `pair_blobs` — canonical checkpoint pair, document status and blob routes.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::body::Bytes;
use axum::extract::{OriginalUri, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use futures::StreamExt;
use semio_hub_artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub_artifact_creation::ArtifactCreationCommitAuthorityV1;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_artifact_authority::ImmutableArtifactBlobStore;
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use directory::os_directory::DocumentScope;
use semio_hub_directory_contract::{HubCapability, AUTH_TEXT_MAX_BYTES};
use semio_hub_lag_rebootstrap::{RebootstrapError, RebootstrapProgress, RebootstrapProgressStage, RebootstrapTransferControl, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use semio_hub_runtime::support::now_ms;
use db::storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 🔖️Rest
#[derive(Serialize)]
pub struct DocumentStatusResponse {
    pub document_id: String,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
}

#[derive(Serialize)]
pub struct BlobRecord {
    pub hash: String,
    pub media_type: String,
    pub size: i64,
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

pub fn canonical_pair_request_admission(uri: &axum::http::Uri, headers: &HeaderMap) -> Result<String, StatusCode> {
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

pub const CANONICAL_PAIR_PROGRESS_STAGES: usize = 8;

pub fn canonical_pair_progress_index(stage: RebootstrapProgressStage) -> usize {
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

pub struct CanonicalPairHttpControl {
    pub cancelled: std::sync::atomic::AtomicBool,
    pub(crate) active: std::sync::atomic::AtomicBool,
    pub progress: Mutex<[Option<RebootstrapProgress>; CANONICAL_PAIR_PROGRESS_STAGES]>,
}

impl CanonicalPairHttpControl {
    pub fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false), active: std::sync::atomic::AtomicBool::new(true), progress: Mutex::new([None; CANONICAL_PAIR_PROGRESS_STAGES]) }
    }

    pub fn request_cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn release(&self) {
        self.active.store(false, std::sync::atomic::Ordering::Release);
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn is_active(&self) -> bool {
        self.active.load(std::sync::atomic::Ordering::Acquire)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn progress_snapshot(&self) -> [Option<RebootstrapProgress>; CANONICAL_PAIR_PROGRESS_STAGES] {
        *self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

pub struct CanonicalPairHttpRequest {
    pub control: Arc<CanonicalPairHttpControl>,
    response_owned: bool,
}

impl CanonicalPairHttpRequest {
    pub fn new(control: Arc<CanonicalPairHttpControl>) -> Self {
        Self { control, response_owned: false }
    }

    pub fn finish_response_owned(&mut self) {
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

#[cfg(any(test, feature = "testkit"))]
pub struct TestCanonicalPairRequestGate {
    pub entered: tokio::sync::Semaphore,
    pub release: tokio::sync::Semaphore,
    pub control: Mutex<Option<Arc<CanonicalPairHttpControl>>>,
}

#[cfg(any(test, feature = "testkit"))]
impl TestCanonicalPairRequestGate {
    pub fn new(release_permits: usize) -> Self {
        Self { entered: tokio::sync::Semaphore::new(0), release: tokio::sync::Semaphore::new(release_permits), control: Mutex::new(None) }
    }

    async fn enter(&self, control: Arc<CanonicalPairHttpControl>) {
        *self.control.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(control);
        self.entered.add_permits(1);
        let _permit = self.release.acquire().await.expect("canonical pair request gate open");
    }

    pub fn control(&self) -> Arc<CanonicalPairHttpControl> {
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

pub fn canonical_pair_error_status(error: RebootstrapError) -> StatusCode {
    match error {
        RebootstrapError::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
        RebootstrapError::Unavailable => StatusCode::NOT_FOUND,
        RebootstrapError::ResourceLimit => StatusCode::PAYLOAD_TOO_LARGE,
        RebootstrapError::Cancelled => StatusCode::SERVICE_UNAVAILABLE,
        RebootstrapError::AuthorityIdentityChanged | RebootstrapError::Integrity => StatusCode::CONFLICT,
    }
}

//#region Blobs
const HUB_BLOB_MAX_BYTES: usize = db::storage::DB_IO_PAGE_BYTES * db::storage::DB_IO_OPERATION_PAGES;

pub async fn db_io_pages_into_http_bytes(mut pages: db::storage::DbIoPages) -> Result<Bytes, db::DbError> {
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

//#endregion Blobs
//#endregion 🔖️Rest
