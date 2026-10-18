//! 📨️ Submit admission and state: the submit limits, the admission slots and the submit state machine. See [`super`].

use super::*;

//#region 🔖️ArtifactHandle
const ARTIFACT_SUBMIT_OPERATION_ITEMS: usize = 64;

pub(super) const ARTIFACT_SUBMIT_PAGE_BYTES: u64 = 16 * 1024;

/// 📐️ One submit holds any batch the document backbone declares legal (`protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_*`): a
/// writer's whole post-cut outbox arrives as one batch and must commit, so the envelope, nested-item and byte credits are the
/// declaration's own maxima — never a smaller engine-side cap a declared batch could hit.
pub(super) const ARTIFACT_SUBMIT_BATCH_ITEMS: usize = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES;

const ARTIFACT_SUBMIT_NESTED_ITEMS: usize = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS;

/// 📐️ Owner bytes of the largest declared batch: the credit page, its fixed owners (envelopes, dependency ids, target
/// segments), every declared byte, and one declared identifier per envelope for the document id the hub re-keys.
const ARTIFACT_SUBMIT_DECLARED_BATCH_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES
    + (protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES * size_of::<protocol::MutationEnvelope>()
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES * size_of::<protocol::MutationId>()
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS * size_of::<String>()
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES
        + protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES * protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES) as u64;

const ARTIFACT_SUBMIT_OPERATION_PAGES: u64 = ARTIFACT_SUBMIT_DECLARED_BATCH_BYTES.div_ceil(ARTIFACT_SUBMIT_PAGE_BYTES);

pub(super) const ARTIFACT_SUBMIT_OPERATION_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_OPERATION_PAGES;

/// 📐️ In-flight submit owners across every document: four declared-maximal batches at once; a fifth waits (`Unavailable`).
const ARTIFACT_SUBMIT_TOTAL_PAGES: u64 = ARTIFACT_SUBMIT_OPERATION_PAGES * 4;

const ARTIFACT_SUBMIT_TOTAL_BYTES: u64 = ARTIFACT_SUBMIT_PAGE_BYTES * ARTIFACT_SUBMIT_TOTAL_PAGES;

pub(super) const ARTIFACT_SUBMIT_RETRY_MS: u64 = 1;

pub(super) const ARTIFACT_SUBMIT_RETRY_LIMIT: u8 = 8;

#[derive(Clone, Copy)]
struct ArtifactSubmitAdmissionSlot {
    pub generation: u64,
    pub bytes: u64,
    pub items: usize,
    pub occupied: bool,
}

const EMPTY_ARTIFACT_SUBMIT_SLOT: ArtifactSubmitAdmissionSlot = ArtifactSubmitAdmissionSlot { generation: 0, bytes: 0, items: 0, occupied: false };

struct ArtifactSubmitAdmissionState {
    pub slots: [ArtifactSubmitAdmissionSlot; ARTIFACT_SUBMIT_OPERATION_ITEMS],
    pub bytes: u64,
    pub next_generation: u64,
}

static ARTIFACT_SUBMIT_ADMISSION: Mutex<ArtifactSubmitAdmissionState> = Mutex::new(ArtifactSubmitAdmissionState { slots: [EMPTY_ARTIFACT_SUBMIT_SLOT; ARTIFACT_SUBMIT_OPERATION_ITEMS], bytes: 0, next_generation: 1 });

pub(super) struct ArtifactSubmitAdmission {
    pub slot: usize,
    pub generation: u64,
    pub bytes: u64,
    pub items: usize,
}

impl ArtifactSubmitAdmission {
    pub fn try_claim(items: usize, bytes: u64) -> Result<Self, DbError> {
        if items == 0 || items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit item credit"));
        }
        if bytes == 0 || bytes > ARTIFACT_SUBMIT_OPERATION_BYTES {
            return Err(DbError::LimitExceeded("artifact submit operation byte credit"));
        }
        let mut state = ARTIFACT_SUBMIT_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::Unavailable("artifact submit item capacity exhausted".to_string()));
        };
        if state.bytes.checked_add(bytes).is_none_or(|next| next > ARTIFACT_SUBMIT_TOTAL_BYTES) {
            return Err(DbError::Unavailable("artifact submit byte capacity exhausted".to_string()));
        }
        let generation = state.next_generation;
        state.next_generation = state.next_generation.checked_add(1).ok_or(DbError::LimitExceeded("artifact submit generation"))?;
        state.slots[slot] = ArtifactSubmitAdmissionSlot { generation, bytes, items, occupied: true };
        state.bytes += bytes;
        Ok(Self { slot, generation, bytes, items })
    }
}

impl Drop for ArtifactSubmitAdmission {
    fn drop(&mut self) {
        let mut state = ARTIFACT_SUBMIT_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let entry = &mut state.slots[self.slot];
        if !entry.occupied || entry.generation != self.generation || entry.bytes != self.bytes || entry.items != self.items {
            return;
        }
        *entry = EMPTY_ARTIFACT_SUBMIT_SLOT;
        state.bytes = state.bytes.checked_sub(self.bytes).expect("artifact submit byte credit underflow");
    }
}

pub(super) fn artifact_submit_credit(batch: &db_artifact_contract::CommandBatch) -> Result<(usize, u64), DbError> {
    if batch.envelopes.is_empty() || batch.envelopes.len() > ARTIFACT_SUBMIT_BATCH_ITEMS {
        return Err(DbError::LimitExceeded("artifact submit batch item credit"));
    }
    let mut items = batch.envelopes.len();
    let mut bytes = ARTIFACT_SUBMIT_PAGE_BYTES;
    let envelope_owner_bytes = batch.envelopes.capacity().checked_mul(size_of::<protocol::MutationEnvelope>()).ok_or(DbError::LimitExceeded("artifact submit envelope owner bytes"))?;
    bytes = bytes.checked_add(envelope_owner_bytes as u64).ok_or(DbError::LimitExceeded("artifact submit envelope owner bytes"))?;
    for envelope in &batch.envelopes {
        items = items.checked_add(envelope.dependencies.len()).ok_or(DbError::LimitExceeded("artifact submit nested items"))?;
        if items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit nested item credit"));
        }
        let dependency_owner_bytes = envelope.dependencies.capacity().checked_mul(size_of::<protocol::MutationId>()).ok_or(DbError::LimitExceeded("artifact submit dependency owner bytes"))?;
        bytes = bytes
            .checked_add(envelope.mutation_id.0.capacity() as u64)
            .and_then(|value| value.checked_add(envelope.document_id.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.actor.0.capacity() as u64))
            .and_then(|value| value.checked_add(dependency_owner_bytes as u64))
            .and_then(|value| value.checked_add(envelope.diff.schema.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.diff.payload.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.inverse.schema.0.capacity() as u64))
            .and_then(|value| value.checked_add(envelope.inverse.payload.capacity() as u64))
            .ok_or(DbError::LimitExceeded("artifact submit nested byte credit"))?;
        for dependency in &envelope.dependencies {
            bytes = bytes.checked_add(dependency.0.capacity() as u64).ok_or(DbError::LimitExceeded("artifact submit dependency byte credit"))?;
        }
        items = items.checked_add(envelope.target.len()).ok_or(DbError::LimitExceeded("artifact submit nested items"))?;
        if items > ARTIFACT_SUBMIT_NESTED_ITEMS {
            return Err(DbError::LimitExceeded("artifact submit nested item credit"));
        }
        let target_owner_bytes = envelope.target.capacity().checked_mul(size_of::<String>()).ok_or(DbError::LimitExceeded("artifact submit target owner bytes"))?;
        bytes = bytes
            .checked_add(target_owner_bytes as u64)
            .and_then(|value| value.checked_add(envelope.observed.as_ref().map_or(0, |observed| observed.0.capacity() as u64)))
            .ok_or(DbError::LimitExceeded("artifact submit target byte credit"))?;
        for segment in &envelope.target {
            bytes = bytes.checked_add(segment.capacity() as u64).ok_or(DbError::LimitExceeded("artifact submit target byte credit"))?;
        }
    }
    let pages = bytes.checked_add(ARTIFACT_SUBMIT_PAGE_BYTES - 1).ok_or(DbError::LimitExceeded("artifact submit page rounding"))? / ARTIFACT_SUBMIT_PAGE_BYTES;
    let admitted = pages.checked_mul(ARTIFACT_SUBMIT_PAGE_BYTES).ok_or(DbError::LimitExceeded("artifact submit page credit"))?;
    if admitted > ARTIFACT_SUBMIT_OPERATION_BYTES {
        return Err(DbError::LimitExceeded("artifact submit operation byte credit"));
    }
    Ok((items, admitted))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubmitProgress {
    Admitted,
    Scheduled,
    Waiting,
    Completed,
    Cancelled,
    Fault,
}

type ArtifactActorSubmitFuture = db_actor::AskFuture<db_artifact_contract::ArtifactMessage, Result<db_artifact_contract::CommandReceipt, DbError>>;

pub type ArtifactSubmitOutcome = Result<Result<CommandReceipt, DbError>, DbError>;

pub(super) enum ArtifactSubmitWorkOwner {
    Request { batch: db_artifact_contract::CommandBatch, options: db_artifact_contract::SubmitOptions, submitted_at_ms: u64 },
    Actor(ArtifactActorSubmitFuture),
}

pub(super) struct ArtifactSubmitState {
    pub pool: WorkerPool,
    pub authority: Mutex<Option<Arc<db_artifact::ArtifactAuthority>>>,
    pub document: protocol::ArtifactId,
    pub generation: u64,
    pub authority_generation: GenerationId,
    pub admission: Mutex<Option<ArtifactSubmitAdmission>>,
    pub work: Mutex<Option<ArtifactSubmitWorkOwner>>,
    pub completion: Mutex<Option<ArtifactSubmitOutcome>>,
    pub terminal_work: Mutex<Option<ArtifactSubmitWorkOwner>>,
    pub terminal_result: Mutex<Option<ArtifactSubmitOutcome>>,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    pub waker: Mutex<Option<std::task::Waker>>,
    pub retry_armed: std::sync::atomic::AtomicBool,
    pub retry_generation: std::sync::atomic::AtomicU64,
    pub scheduled: std::sync::atomic::AtomicBool,
    pub cancelled: std::sync::atomic::AtomicBool,
    pub abandoned: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    pub progress: std::sync::atomic::AtomicU8,
}

pub struct SubmitFuture {
    pub state: Arc<ArtifactSubmitState>,
    pub resolved: bool,
}

pub struct ArtifactSubmitTerminalJob {
    pub state: Arc<ArtifactSubmitState>,
    pub owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

pub struct ArtifactSubmitTerminalWork {
    pub state: Arc<ArtifactSubmitState>,
    pub owner: Option<ArtifactSubmitWorkOwner>,
}

struct ArtifactSubmitWake {
    pub state: std::sync::Weak<ArtifactSubmitState>,
    pub generation: u64,
}

impl std::task::Wake for ArtifactSubmitWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            if self.generation == state.generation {
                state.schedule();
            }
        }
    }
}

impl ArtifactSubmitState {
    pub fn set_progress(&self, progress: SubmitProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn wake_waiter(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    pub fn finish(&self) {
        if !self.finished.swap(true, std::sync::atomic::Ordering::AcqRel) {
            self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.authority.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        }
    }

    pub fn authority(&self) -> Option<Arc<db_artifact::ArtifactAuthority>> {
        self.authority.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    pub(super) fn finish_if_terminal_empty(&self) {
        if self.terminal_is_empty() && !self.scheduled.load(std::sync::atomic::Ordering::Acquire) && !self.retry_armed.load(std::sync::atomic::Ordering::Acquire) {
            self.finish();
        }
    }

    pub fn complete(&self, result: ArtifactSubmitOutcome, progress: SubmitProgress) {
        self.set_progress(progress);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            self.wake_waiter();
        }
    }

    pub(super) fn terminalize_work(&self, result: ArtifactSubmitOutcome, progress: SubmitProgress) {
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
        }
        self.complete(result, progress);
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.set_progress(SubmitProgress::Scheduled);
        let state = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || state.drive_one(generation)), 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match worker_submit_retry_attempt(error.kind(), attempt, ARTIFACT_SUBMIT_RETRY_LIMIT) {
                Some(next) => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                    self.arm_retry();
                }
                None => {
                    let kind = error.kind();
                    let job = error.into_job();
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                    }
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                    self.complete(Err(DbError::Unavailable(format!("artifact submit WorkerPool submission failed: {kind:?}"))), SubmitProgress::Fault);
                }
            },
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = match self.retry_generation.try_update(Ordering::AcqRel, Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
            Ok(previous) => match previous.checked_add(1) {
                Some(generation) => generation,
                None => {
                    self.terminalize_retry_authority("artifact submit retry generation exhausted");
                    return;
                }
            },
            Err(_) => {
                self.terminalize_retry_authority("artifact submit retry generation exhausted");
                return;
            }
        };
        let Some(deadline) = self.pool.now_ms().checked_add(ARTIFACT_SUBMIT_RETRY_MS) else {
            self.terminalize_retry_authority("artifact submit retry deadline exhausted");
            return;
        };
        let state = self.clone();
        self.pool.callback_at(deadline, move || {
            if generation != state.retry_generation.load(Ordering::Acquire) {
                return;
            }
            state.retry_armed.store(false, Ordering::Release);
            let retry = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if state.cancelled.load(Ordering::Acquire) {
                    drop(job);
                    state.scheduled.store(false, Ordering::Release);
                    state.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
                } else {
                    state.submit_exact(job, attempt);
                }
            }
        });
    }

    pub(super) fn terminalize_retry_authority(&self, detail: &'static str) {
        self.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        let retained = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some((job, attempt)) = retained {
            let mut terminal = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal.is_none() {
                *terminal = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
            } else {
                drop(terminal);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
        }
        self.terminalize_work(Err(DbError::Unavailable(detail.to_string())), SubmitProgress::Fault);
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::future::Future as _;
        use std::sync::atomic::Ordering;

        if generation != self.generation {
            return;
        }
        let Some(authority) = self.authority() else {
            self.scheduled.store(false, Ordering::Release);
            self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
            return;
        };
        if authority.generation() != self.authority_generation {
            self.scheduled.store(false, Ordering::Release);
            self.terminalize_work(Err(DbError::StaleGeneration { expected: authority.generation(), actual: self.authority_generation }), SubmitProgress::Fault);
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if self.cancelled.load(Ordering::Acquire) {
            self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled);
            return;
        }

        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if matches!(work.as_ref(), Some(ArtifactSubmitWorkOwner::Request { .. })) {
            let Some(ArtifactSubmitWorkOwner::Request { batch, options, submitted_at_ms }) = work.take() else {
                return;
            };
            *work = Some(ArtifactSubmitWorkOwner::Actor(authority.submit_retained(batch, options, submitted_at_ms)));
            drop(work);
            self.schedule();
            return;
        }

        let Some(ArtifactSubmitWorkOwner::Actor(future)) = work.as_mut() else {
            return;
        };
        let waker = std::task::Waker::from(Arc::new(ArtifactSubmitWake { state: Arc::downgrade(&self), generation }));
        let mut context = std::task::Context::from_waker(&waker);
        match std::pin::Pin::new(future).poll(&mut context) {
            std::task::Poll::Pending => {
                self.set_progress(SubmitProgress::Waiting);
            }
            std::task::Poll::Ready(result) => {
                work.take();
                drop(work);
                let result = result.map(|inner| inner.map(|receipt| to_engine_receipt(receipt, self.document.clone())));
                if self.cancelled.load(Ordering::Acquire) {
                    *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                    self.complete(Err(DbError::Closed), SubmitProgress::Cancelled);
                } else {
                    self.complete(result, SubmitProgress::Completed);
                }
            }
        }
    }

    pub fn close_one(&self) -> bool {
        if let Some((_, job)) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return true;
        }
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return true;
        }
        if let Some(work) = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(work);
            return true;
        }
        if let Some(result) = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return true;
        }
        false
    }
}
//#endregion 🔖️ArtifactHandle
