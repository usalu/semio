//! 🌱️ `artifact_creation` — space artifact creation over HTTP.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use directory::os_directory::io::text::access_policy::directory_access_permits;
use axum::body::Bytes;
use axum::extract::{OriginalUri, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use directory::os_directory::schema::space_artifact_creation::{SpaceArtifactCreateV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationStatusV1, SPACE_ARTIFACT_CREATION_MAX_BYTES};
use futures::StreamExt;
use semio_framework_trace::record::{TraceOutcome, TraceRecord, Tracer};
use semio_hub_artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub_artifact_creation::{ArtifactCreationActorV1, ArtifactCreationCommitAuthorityV1, ArtifactCreationCommitFutureV1, ArtifactCreationCommitLeaseV1};
use semio_hub_artifact_creation_service::ArtifactCreationServiceV1;
#[cfg(test)]
use semio_hub_artifact_authority::ImmutableArtifactBlobStore;
use semio_hub_artifact_authority::{AuthorityLimits, AuthorityOperationControl, AuthorityProgress, OperationContext};
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use semio_hub_directory_contract::error::DirectoryError;
use semio_hub_directory_contract::model::SocketSessionBindingStatus;
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_directory::HubDirectory;
use semio_hub_directory_contract::LocalBootstrapTransport;
use semio_hub_inference_runtime_contract::GisMapApprovalIngressAuthorityV1;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use semio_hub_runtime::support::{DirectoryJson, now_ms};
use semio_hub_ledgers::socket_grants::{SocketBindingKeyV1, SocketBindingGuardV1, SocketBindingGatesV1};
use semio_hub_ledgers::guard::session_access_roles;
use db::storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 🔖️Directory
struct HubArtifactCreationCommitLeaseV1 {
    _guards: Vec<SocketBindingGuardV1>,
}

impl ArtifactCreationCommitLeaseV1 for HubArtifactCreationCommitLeaseV1 {}

pub struct HubArtifactCreationCommitAuthorityV1 {
    pub directory: Arc<HubDirectories>,
    pub gates: Arc<SocketBindingGatesV1>,
}

impl ArtifactCreationCommitAuthorityV1 for HubArtifactCreationCommitAuthorityV1 {
    fn acquire<'a>(&'a self, actor: &'a ArtifactCreationActorV1, space_id: &'a str) -> ArtifactCreationCommitFutureV1<'a> {
        Box::pin(async move {
            let guards = tokio::time::timeout(
                std::time::Duration::from_secs(2),
                self.gates.share_bindings(vec![
                    SocketBindingKeyV1::User(actor.user_id.clone()),
                    SocketBindingKeyV1::Session(actor.session_id.clone()),
                    SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_string() },
                    SocketBindingKeyV1::Membership { user_id: actor.user_id.clone(), space_id: space_id.to_string() },
                ]),
            )
            .await
            .map_err(|_| DirectoryError::Backend("artifact creation final authority unavailable".into()))?;
            let space_kind = self.directory.get_space(space_id).await.ok().flatten().map(|space| space.kind);
            match tokio::time::timeout(std::time::Duration::from_secs(2), self.directory.socket_session_binding(&actor.session_id, &actor.user_id, actor.authorization_generation, Some(space_id), now_ms())).await {
                Ok(Ok(SocketSessionBindingStatus::Active { role: Some(role), session_kind, .. })) if space_kind.as_deref().is_some_and(|kind| directory_access_permits(&session_access_roles(session_kind, Some(role)), DirectoryAccessActionV1::ArtifactCreate, Some(kind))) => {
                    Ok(Box::new(HubArtifactCreationCommitLeaseV1 { _guards: guards }) as Box<dyn ArtifactCreationCommitLeaseV1>)
                }
                Ok(Ok(SocketSessionBindingStatus::Unavailable)) | Ok(Err(_)) | Err(_) => Err(DirectoryError::Backend("artifact creation final authority unavailable".into())),
                _ => Err(DirectoryError::Unauthorized),
            }
        })
    }
}

pub fn artifact_creation_space_id_v1(space_id: &str) -> bool {
    !space_id.is_empty() && space_id.len() <= 256 && space_id.as_bytes()[0].is_ascii_alphanumeric() && space_id.bytes().all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
}

const ARTIFACT_CREATION_HTTP_CAPACITY: usize = 8;

const ARTIFACT_CREATION_SHUTDOWN_DEADLINE: std::time::Duration = std::time::Duration::from_secs(32);

pub struct ArtifactCreationHttpControlV1 {
    pub cancelled: std::sync::atomic::AtomicBool,
    shutdown_cancelled: Option<Arc<std::sync::atomic::AtomicBool>>,
    fault: Mutex<Option<String>>,
    pub progress: Mutex<Option<directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationProgressV1>>,
}

impl ArtifactCreationHttpControlV1 {
    /// 🛑️ Cancellation only. The control used to carry a wall-clock instant of its own and report
    /// `is_cancelled` past it, which made every creation a race against the calendar twice over —
    /// once in the control and once in the `OperationContext` — and turned an honest overrun into
    /// `Cancelled`, the one terminal phase that says the AUTHOR stopped the work. The bound lives
    /// in the context alone now (`ARTIFACT_CREATION_STALL_BOUND_MS`), which is the single place
    /// ticket 26/09/18 slice HT16 put operation bounds.
    pub fn new() -> Self {
        Self { cancelled: std::sync::atomic::AtomicBool::new(false), shutdown_cancelled: None, fault: Mutex::new(None), progress: Mutex::new(None) }
    }

    fn recovery(shutdown_cancelled: Arc<std::sync::atomic::AtomicBool>) -> Self {
        Self { shutdown_cancelled: Some(shutdown_cancelled), ..Self::new() }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    /// 🧯️ Takes the sentence the operation left behind, so the host reports it exactly once.
    pub fn take_fault(&self) -> Option<String> {
        self.fault.lock().ok().and_then(|mut held| held.take())
    }

    /// 📈️ Where this running creation is: `queued` until its operation reports anything.
    pub fn progress(&self) -> directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationProgressV1 {
        use directory::os_directory::schema::space_artifact_creation::{SpaceArtifactCreationProgressV1, SpaceArtifactCreationStageV1};
        self.progress.lock().ok().and_then(|held| *held).unwrap_or(SpaceArtifactCreationProgressV1 { stage: SpaceArtifactCreationStageV1::Queued, completed_units: 0, total_units: 1 })
    }
}

/// 🧭️ The creation stage an authority progress stage belongs to; a stage that says nothing about
/// where the creation is keeps the stage already reported.
fn artifact_creation_stage(stage: semio_hub_artifact_authority::AuthorityProgressStage) -> Option<directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationStageV1> {
    use semio_hub_artifact_authority::AuthorityProgressStage;
    use directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationStageV1;
    match stage {
        AuthorityProgressStage::GuestCompiling => Some(SpaceArtifactCreationStageV1::CompilingGuest),
        AuthorityProgressStage::GuestCodecExecuting => Some(SpaceArtifactCreationStageV1::Genesis),
        AuthorityProgressStage::CasChunkStored
        | AuthorityProgressStage::CasChunkVerified
        | AuthorityProgressStage::CasManifestStored
        | AuthorityProgressStage::CasManifestVerified
        | AuthorityProgressStage::PackStaged
        | AuthorityProgressStage::SprStaged
        | AuthorityProgressStage::PackVerified
        | AuthorityProgressStage::SprVerified
        | AuthorityProgressStage::Published => Some(SpaceArtifactCreationStageV1::Publishing),
        AuthorityProgressStage::Preflight
        | AuthorityProgressStage::CatalogLoading
        | AuthorityProgressStage::CatalogResolved
        | AuthorityProgressStage::InputValidated
        | AuthorityProgressStage::ApplyingOperations
        | AuthorityProgressStage::OutputValidated
        | AuthorityProgressStage::Derived
        | AuthorityProgressStage::CasSweep => None,
    }
}

impl AuthorityOperationControl for ArtifactCreationHttpControlV1 {
    fn now_ms(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.shutdown_cancelled.as_ref().is_some_and(|cancelled| cancelled.load(std::sync::atomic::Ordering::Acquire))
    }

    fn report(&self, progress: AuthorityProgress) {
        let Some(stage) = artifact_creation_stage(progress.stage) else { return };
        let total_units = progress.total_units.max(1);
        let reported = directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationProgressV1 { stage, completed_units: progress.completed_units.min(total_units), total_units };
        if let Ok(mut held) = self.progress.lock() {
            *held = Some(reported);
        }
    }

    fn fault(&self, detail: &str) {
        if let Ok(mut held) = self.fault.lock() {
            held.get_or_insert_with(|| detail.to_owned());
        }
    }
}

pub(crate) struct ArtifactCreationHttpTaskV1 {
    pub(crate) control: Arc<ArtifactCreationHttpControlV1>,
    pub(crate) pending: Arc<ArtifactCreationHttpPendingV1>,
    pub(crate) task: tokio::task::JoinHandle<()>,
}

pub(crate) struct ArtifactCreationHttpPendingV1 {
    pub(crate) control: Arc<ArtifactCreationHttpControlV1>,
    pub(crate) disposition: std::sync::atomic::AtomicU8,
    pub(crate) changed: tokio::sync::Notify,
}

pub(crate) struct ArtifactCreationHttpTaskOwnerStateV1 {
    pub(crate) closing: bool,
    reservations: BTreeMap<String, Arc<ArtifactCreationHttpPendingV1>>,
    pub(crate) tasks: BTreeMap<String, ArtifactCreationHttpTaskV1>,
    recovery: Option<tokio::task::JoinHandle<()>>,
}

pub struct ArtifactCreationHttpTaskOwnerV1 {
    pub state: Mutex<ArtifactCreationHttpTaskOwnerStateV1>,
    pub(crate) changed: Arc<tokio::sync::Notify>,
    recovery_cancelled: Arc<std::sync::atomic::AtomicBool>,
}

pub(crate) struct ArtifactCreationHttpReservationV1 {
    pub(crate) owner: Arc<ArtifactCreationHttpTaskOwnerV1>,
    pub(crate) key: String,
    pub(crate) pending: Arc<ArtifactCreationHttpPendingV1>,
    activated: bool,
}

pub enum ArtifactCreationHttpAdmissionV1 {
    Owner(ArtifactCreationHttpReservationV1),
    Join(Arc<ArtifactCreationHttpPendingV1>),
    Unavailable,
}

impl ArtifactCreationHttpTaskOwnerV1 {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(ArtifactCreationHttpTaskOwnerStateV1 { closing: false, reservations: BTreeMap::new(), tasks: BTreeMap::new(), recovery: None }),
            changed: Arc::new(tokio::sync::Notify::new()),
            recovery_cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub fn reserve(self: &Arc<Self>, key: String, control: Arc<ArtifactCreationHttpControlV1>) -> ArtifactCreationHttpAdmissionV1 {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.tasks.retain(|_, task| !task.task.is_finished());
        if state.closing {
            return ArtifactCreationHttpAdmissionV1::Unavailable;
        }
        if let Some(pending) = state.reservations.get(&key).or_else(|| state.tasks.get(&key).map(|task| &task.pending)) {
            return ArtifactCreationHttpAdmissionV1::Join(pending.clone());
        }
        if state.tasks.len().saturating_add(state.reservations.len()) >= ARTIFACT_CREATION_HTTP_CAPACITY {
            return ArtifactCreationHttpAdmissionV1::Unavailable;
        }
        let pending = Arc::new(ArtifactCreationHttpPendingV1 { control, disposition: std::sync::atomic::AtomicU8::new(0), changed: tokio::sync::Notify::new() });
        state.reservations.insert(key.clone(), pending.clone());
        ArtifactCreationHttpAdmissionV1::Owner(ArtifactCreationHttpReservationV1 { owner: self.clone(), key, pending, activated: false })
    }

    /// 🔑️ Whether a live execution in THIS process owns that creation key right now — it holds a
    /// reservation, or its detached task has not finished. Recovery must ask before it closes an
    /// uncommitted key: the sweep runs every second and reads only durable facts, so a creation
    /// that is legitimately still working looks exactly like one whose executor died, and closing
    /// it takes the key out from under an execution that is about to append `Prepared`.
    pub fn owns_live_execution(&self, key: &str) -> bool {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.tasks.retain(|_, task| !task.task.is_finished());
        state.reservations.contains_key(key) || state.tasks.contains_key(key)
    }

    /// 📈️ The progress of the live execution this process runs for `key`, if it runs one.
    fn live_progress(&self, key: &str) -> Option<directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationProgressV1> {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.tasks.retain(|_, task| !task.task.is_finished());
        state.tasks.get(key).map(|task| task.control.progress()).or_else(|| state.reservations.get(key).map(|pending| pending.control.progress()))
    }

    pub fn start_recovery(self: &Arc<Self>, service: Arc<ArtifactCreationServiceV1>, authority: Arc<HubArtifactCreationCommitAuthorityV1>, tracer: Tracer) {
        let cancelled = self.recovery_cancelled.clone();
        let wake = self.changed.clone();
        let owner = Arc::downgrade(self);
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {}
                    _ = wake.notified() => {}
                }
                if cancelled.load(std::sync::atomic::Ordering::Acquire) {
                    break;
                }
                let scan_now = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX));
                let intents = match service.recovery_candidates(scan_now, 32).await {
                    Ok(intents) => intents,
                    Err(error) => {
                        let mut record = TraceRecord::new("server.artifact.maintenance", TraceOutcome::Failed);
                        record.detail = Some(format!("creation-recovery-scan-unavailable:{error}"));
                        tracer.emit(record);
                        continue;
                    }
                };
                for intent in intents {
                    if cancelled.load(std::sync::atomic::Ordering::Acquire) {
                        break;
                    }
                    let Some(owner) = owner.upgrade() else { break };
                    if owner.owns_live_execution(&artifact_creation_task_key_v1(&intent.actor.user_id, &intent.scope.space_id, &intent.request.request_id)) {
                        continue;
                    }
                    drop(owner);
                    let control = ArtifactCreationHttpControlV1::recovery(cancelled.clone());
                    let Ok(context) = OperationContext::stall_bounded(semio_hub_artifact_creation::ARTIFACT_CREATION_STALL_BOUND_MS, AuthorityLimits::maximum(), &control) else { break };
                    if let Err(error) = service.recover(intent, authority.as_ref(), &context).await {
                        let mut record = TraceRecord::new("server.artifact.maintenance", TraceOutcome::Failed);
                        record.detail = Some(format!("creation-recovery-attempt-unavailable:{error}"));
                        tracer.emit(record);
                    }
                    if let Some(detail) = control.take_fault() {
                        let mut record = TraceRecord::new("server.artifact.creation", TraceOutcome::Failed);
                        record.detail = Some(detail);
                        tracer.emit(record);
                    }
                }
            }
        });
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.closing || state.recovery.is_some() {
            task.abort();
        } else {
            state.recovery = Some(task);
        }
    }

    pub fn cancel(&self, key: &str) {
        let state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(pending) = state.reservations.get(key) {
            pending.control.cancel();
        }
        if let Some(task) = state.tasks.get(key) {
            task.control.cancel();
        }
    }

    pub async fn shutdown(&self) {
        self.shutdown_with_deadline(ARTIFACT_CREATION_SHUTDOWN_DEADLINE).await;
    }

    pub async fn shutdown_with_deadline(&self, shutdown_deadline: std::time::Duration) {
        self.recovery_cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.changed.notify_waiters();
        let deadline = tokio::time::Instant::now() + shutdown_deadline;
        loop {
            let drained = {
                let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                state.closing = true;
                for pending in state.reservations.values() {
                    pending.control.cancel();
                }
                for task in state.tasks.values() {
                    task.control.cancel();
                }
                state.reservations.is_empty()
            };
            if drained {
                break;
            }
            if tokio::time::timeout_at(deadline, self.changed.notified()).await.is_err() {
                break;
            }
        }
        let (mut tasks, recovery, reservations) = {
            let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let tasks = std::mem::take(&mut state.tasks).into_values().map(|task| task.task).collect::<Vec<_>>();
            (tasks, state.recovery.take(), std::mem::take(&mut state.reservations).into_values().collect::<Vec<_>>())
        };
        for pending in reservations {
            pending.control.cancel();
            let _ = pending.disposition.compare_exchange(0, 2, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
            pending.changed.notify_waiters();
        }
        self.changed.notify_waiters();
        if let Some(recovery) = recovery {
            tasks.push(recovery);
        }
        if tokio::time::timeout_at(deadline, futures::future::join_all(tasks.iter_mut())).await.is_err() {
            for task in &tasks {
                task.abort();
            }
            for task in tasks {
                let _ = task.await;
            }
        }
    }

    #[cfg(test)]
    pub fn task_count(&self) -> usize {
        let state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.reservations.len().saturating_add(state.tasks.values().filter(|task| !task.task.is_finished()).count())
    }
}

impl ArtifactCreationHttpReservationV1 {
    pub(crate) fn activate<F>(mut self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let mut state = self.owner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owns_reservation = state.reservations.get(&self.key).is_some_and(|pending| Arc::ptr_eq(pending, &self.pending));
        if !state.closing && owns_reservation {
            state.reservations.remove(&self.key);
            state.tasks.insert(self.key.clone(), ArtifactCreationHttpTaskV1 { control: self.pending.control.clone(), pending: self.pending.clone(), task: tokio::spawn(future) });
            self.activated = true;
        }
        drop(state);
        let disposition = if self.activated { 1 } else { 2 };
        let _ = self.pending.disposition.compare_exchange(0, disposition, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
        self.pending.changed.notify_waiters();
        self.owner.changed.notify_waiters();
    }

    pub(crate) fn finish(mut self) {
        let mut state = self.owner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owns_reservation = state.reservations.get(&self.key).is_some_and(|pending| Arc::ptr_eq(pending, &self.pending));
        if owns_reservation {
            state.reservations.remove(&self.key);
        }
        let disposition = if !state.closing && owns_reservation { 1 } else { 2 };
        drop(state);
        self.activated = true;
        let _ = self.pending.disposition.compare_exchange(0, disposition, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
        self.pending.changed.notify_waiters();
        self.owner.changed.notify_waiters();
    }
}

impl Drop for ArtifactCreationHttpReservationV1 {
    fn drop(&mut self) {
        if self.activated {
            return;
        }
        let mut state = self.owner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.reservations.get(&self.key).is_some_and(|pending| Arc::ptr_eq(pending, &self.pending)) {
            state.reservations.remove(&self.key);
        }
        drop(state);
        let _ = self.pending.disposition.compare_exchange(0, 2, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
        self.pending.changed.notify_waiters();
        self.owner.changed.notify_waiters();
    }
}

pub async fn await_artifact_creation_http_admission_v1(pending: &ArtifactCreationHttpPendingV1) -> Option<u8> {
    let disposition = pending.disposition.load(std::sync::atomic::Ordering::Acquire);
    if disposition != 0 {
        return Some(disposition);
    }
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        loop {
            let changed = pending.changed.notified();
            let disposition = pending.disposition.load(std::sync::atomic::Ordering::Acquire);
            if disposition != 0 {
                return disposition;
            }
            changed.await;
        }
    })
    .await
    .ok()
}

pub fn artifact_creation_task_key_v1(user_id: &str, space_id: &str, request_id: &str) -> String {
    format!("{user_id}\0{space_id}\0{request_id}")
}

pub fn artifact_creation_request_id_v1(request_id: &str) -> bool {
    request_id.len() == 32 && request_id.bytes().any(|byte| byte != b'0') && request_id.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

pub fn artifact_creation_error_status(error: DirectoryError) -> StatusCode {
    match error {
        DirectoryError::NotFound(_) => StatusCode::NOT_FOUND,
        DirectoryError::Conflict(_) => StatusCode::CONFLICT,
        DirectoryError::Unauthorized => StatusCode::UNAUTHORIZED,
        DirectoryError::Backend(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

pub fn artifact_creation_status_response(status: SpaceArtifactCreationStatusV1) -> Response {
    let code = match status.phase {
        SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing => StatusCode::ACCEPTED,
        SpaceArtifactCreationPhaseV1::Ready | SpaceArtifactCreationPhaseV1::Indeterminate | SpaceArtifactCreationPhaseV1::Failed | SpaceArtifactCreationPhaseV1::Cancelled => StatusCode::OK,
    };
    let mut response = (code, DirectoryJson(status)).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
    response
}

//#endregion 🔖️Directory
