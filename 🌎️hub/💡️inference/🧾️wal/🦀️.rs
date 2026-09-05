//! 🧾️ Exact committed-WAL command witnesses with fenced scope and retained cancellation cleanup.

use super::{
    schema::{hex, server_id, SAFE_INTEGER_MAX},
    InferenceErrorV1, InferenceOperationControlV1,
};
use db::wal::{WalCursorControl, WalRecord, WalReplayCursor, WalReplayStep};
use directory::os_directory::DocumentScope;
use semio_framework_hash::Sha256;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

const VERIFIER_CAPACITY: usize = 4;
const CLOSE_MAX_STEPS: usize = 8192;

pub struct InferenceDocumentFenceV1 {
    scope: DocumentScope,
    generation: AtomicU64,
}

impl InferenceDocumentFenceV1 {
    pub fn new(scope: DocumentScope, generation: u64) -> Result<Self, InferenceErrorV1> {
        if !server_id(&scope.space_id) || !server_id(&scope.document_id) || generation == 0 || generation > SAFE_INTEGER_MAX {
            return Err(InferenceErrorV1::Invalid);
        }
        Ok(Self { scope, generation: AtomicU64::new(generation) })
    }

    pub fn invalidate(&self) {
        self.generation.store(0, Ordering::Release);
    }
}

pub struct InferenceWalTargetV1 {
    pub scope: DocumentScope,
    pub generation: u64,
    pub job_id: String,
    pub proposal_hash: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub actor: String,
    pub maximum_records: u64,
}

impl InferenceWalTargetV1 {
    fn validate(&self, fence: &InferenceDocumentFenceV1) -> Result<(), InferenceErrorV1> {
        if !hex(&self.job_id, 32) || !hex(&self.proposal_hash, 64) || !hex(&self.mutation_id, 32) || !hex(&self.command_hash, 64) || self.maximum_records == 0 || self.maximum_records > 65_536 {
            return Err(InferenceErrorV1::Bounds);
        }
        let (user, session) = self.actor.strip_prefix("user:").and_then(|value| value.split_once("#session:")).ok_or(InferenceErrorV1::Invalid)?;
        if !server_id(user) || !server_id(session) {
            return Err(InferenceErrorV1::Invalid);
        }
        if super::sha256(format!("semio.hub.inference-approval-mutation/v1\0{}\0{}", self.job_id, self.proposal_hash).as_bytes())[..32] != self.mutation_id {
            return Err(InferenceErrorV1::Conflict);
        }
        if self.scope != fence.scope || self.generation == 0 || fence.generation.load(Ordering::Acquire) != self.generation {
            return Err(InferenceErrorV1::Conflict);
        }
        Ok(())
    }

    fn document_key(&self) -> db::ArtifactId {
        db::ArtifactId(format!("v1:{}:{}:{}{}", self.scope.space_id.len(), self.scope.document_id.len(), self.scope.space_id, self.scope.document_id))
    }
}

pub struct CommittedInferenceWalWitnessV1 {
    fence: Arc<InferenceDocumentFenceV1>,
    scope: DocumentScope,
    generation: u64,
    job_id: String,
    proposal_hash: String,
    mutation_id: String,
    command_hash: String,
    transaction_id: u64,
}

impl CommittedInferenceWalWitnessV1 {
    pub(super) fn matches(&self, scope: &DocumentScope, generation: u64, job_id: &str, proposal_hash: &str, mutation_id: &str, command_hash: &str) -> bool {
        self.scope == *scope
            && self.generation == generation
            && self.fence.generation.load(Ordering::Acquire) == generation
            && self.job_id == job_id
            && self.proposal_hash == proposal_hash
            && self.mutation_id == mutation_id
            && self.command_hash == command_hash
            && self.transaction_id != 0
    }
}

struct VerifierState {
    storage: Arc<db::storage::DbBackend>,
    slots: Arc<tokio::sync::Semaphore>,
    active: AtomicUsize,
    close_steps: AtomicU64,
    #[cfg(test)]
    replay_gate: Option<Arc<tokio::sync::Semaphore>>,
    #[cfg(test)]
    hashing_gate: Option<Arc<tokio::sync::Semaphore>>,
    #[cfg(test)]
    hashing_steps: AtomicU64,
}

pub struct InferenceWalVerifierV1 {
    state: Arc<VerifierState>,
}

struct VerifierAdmission {
    state: Arc<VerifierState>,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl Drop for VerifierAdmission {
    fn drop(&mut self) {
        self.state.active.fetch_sub(1, Ordering::AcqRel);
    }
}

struct CallerCancellation {
    control: Arc<InferenceOperationControlV1>,
    disarmed: bool,
}

impl Drop for CallerCancellation {
    fn drop(&mut self) {
        if !self.disarmed {
            self.control.cancel();
        }
    }
}

impl InferenceWalVerifierV1 {
    pub fn new(storage: Arc<db::storage::DbBackend>) -> Self {
        Self {
            state: Arc::new(VerifierState {
                storage,
                slots: Arc::new(tokio::sync::Semaphore::new(VERIFIER_CAPACITY)),
                active: AtomicUsize::new(0),
                close_steps: AtomicU64::new(0),
                #[cfg(test)]
                replay_gate: None,
                #[cfg(test)]
                hashing_gate: None,
                #[cfg(test)]
                hashing_steps: AtomicU64::new(0),
            }),
        }
    }

    pub fn active(&self) -> usize {
        self.state.active.load(Ordering::Acquire)
    }
    pub fn close_steps(&self) -> u64 {
        self.state.close_steps.load(Ordering::Acquire)
    }

    pub async fn verify(&self, target: InferenceWalTargetV1, fence: Arc<InferenceDocumentFenceV1>, control: Arc<InferenceOperationControlV1>) -> Result<Option<CommittedInferenceWalWitnessV1>, InferenceErrorV1> {
        target.validate(&fence)?;
        control.checkpoint(0)?;
        let permit = self.state.slots.clone().try_acquire_owned().map_err(|_| InferenceErrorV1::Capacity)?;
        self.state.active.fetch_add(1, Ordering::AcqRel);
        let admission = VerifierAdmission { state: self.state.clone(), _permit: permit };
        let mut cancellation = CallerCancellation { control: control.clone(), disarmed: false };
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let retained_control = control.clone();
        tokio::spawn(async move {
            let _admission = admission;
            let outcome = verify_retained(&_admission.state, &target, &fence, &retained_control).await;
            drop(_admission);
            let _ = sender.send(outcome);
        });
        let outcome = tokio::select! {
            biased;
            error = control.interruption() => Err(error),
            outcome = receiver => outcome.map_err(|_| InferenceErrorV1::Storage)?,
        };
        cancellation.disarmed = true;
        outcome
    }
}

struct Transaction {
    id: u64,
    records: u32,
    matched: bool,
}

async fn verify_retained(state: &VerifierState, target: &InferenceWalTargetV1, fence: &Arc<InferenceDocumentFenceV1>, control: &InferenceOperationControlV1) -> Result<Option<CommittedInferenceWalWitnessV1>, InferenceErrorV1> {
    let document = target.document_key();
    let storage = state.storage.wal().await;
    let retained_control = WalCursorControl::new(Arc::new(AtomicBool::new(false)), Instant::now() + Duration::from_secs(2), 1_000_000).map_err(|_| InferenceErrorV1::Storage)?;
    control.checkpoint(0)?;
    let mut replay = WalReplayCursor::open_genesis(&storage, &document, retained_control).await.map_err(|_| InferenceErrorV1::Storage)?;
    let outcome = scan(state, &mut replay, target, fence, control).await;
    for _ in 0..CLOSE_MAX_STEPS {
        replay.replenish(Instant::now() + Duration::from_secs(2), 1024).map_err(|_| InferenceErrorV1::Storage)?;
        if !replay.close_owner_step().map_err(|_| InferenceErrorV1::Storage)? {
            if !replay.terminal_is_empty() {
                return Err(InferenceErrorV1::Storage);
            }
            if outcome.is_err() {
                return outcome;
            }
            control.checkpoint(control.progress().0)?;
            target.validate(fence)?;
            return outcome;
        }
        state.close_steps.fetch_add(1, Ordering::AcqRel);
        tokio::task::yield_now().await;
    }
    Err(InferenceErrorV1::Bounds)
}

async fn scan(
    _state: &VerifierState,
    replay: &mut WalReplayCursor<'_, db::storage::WalRef<'_>>,
    target: &InferenceWalTargetV1,
    fence: &Arc<InferenceDocumentFenceV1>,
    control: &InferenceOperationControlV1,
) -> Result<Option<CommittedInferenceWalWitnessV1>, InferenceErrorV1> {
    let document = target.document_key();
    let mut active: Option<Transaction> = None;
    let mut matched_transaction = None;
    let mut last_transaction = 0;
    let mut next_segment = 0;
    let mut records = 0;
    loop {
        control.checkpoint(records)?;
        target.validate(fence)?;
        replay.replenish(Instant::now() + Duration::from_secs(2), 1_000_000).map_err(|_| InferenceErrorV1::Storage)?;
        let step = replay.next_step().await.map_err(|_| InferenceErrorV1::Storage)?;
        let mut record = match step {
            WalReplayStep::Done => break,
            WalReplayStep::Yield => {
                #[cfg(test)]
                if _state.hashing_steps.fetch_add(1, Ordering::AcqRel) == 0 {
                    if let Some(gate) = &_state.hashing_gate {
                        gate.acquire().await.map_err(|_| InferenceErrorV1::Storage)?.forget();
                    }
                }
                tokio::task::yield_now().await;
                continue;
            }
            WalReplayStep::Record(record) => record,
        };
        let checked = (|| {
            control.checkpoint(records)?;
            target.validate(fence)?;
            if let WalRecord::SegmentHeader { document: stored, segment_index, prev_chain_hash } = &record {
                if stored != &document || *segment_index != next_segment || active.is_some() || (*segment_index == 0) != prev_chain_hash.is_none() {
                    return Err(InferenceErrorV1::Invalid);
                }
                next_segment += 1;
                return Ok(());
            }
            if next_segment == 0 || records >= target.maximum_records {
                return Err(InferenceErrorV1::Bounds);
            }
            records += 1;
            match &record {
                WalRecord::TxBegin { tx_id } => {
                    if active.is_some() || *tx_id <= last_transaction {
                        return Err(InferenceErrorV1::Invalid);
                    }
                    active = Some(Transaction { id: *tx_id, records: 0, matched: false });
                }
                WalRecord::TxCommit { tx_id, .. } | WalRecord::TxAbort { tx_id } => {
                    let transaction = active.take().ok_or(InferenceErrorV1::Invalid)?;
                    if transaction.id != *tx_id {
                        return Err(InferenceErrorV1::Invalid);
                    }
                    if let WalRecord::TxCommit { record_count, .. } = &record {
                        if transaction.records != *record_count {
                            return Err(InferenceErrorV1::Invalid);
                        }
                        if transaction.matched && matched_transaction.replace(*tx_id).is_some() {
                            return Err(InferenceErrorV1::Invalid);
                        }
                    }
                    last_transaction = *tx_id;
                }
                _ => {
                    let transaction = active.as_mut().ok_or(InferenceErrorV1::Invalid)?;
                    transaction.records = transaction.records.checked_add(1).ok_or(InferenceErrorV1::Bounds)?;
                    if let WalRecord::Command(bytes) = &record {
                        let mut hash = Sha256::new();
                        for fragment in bytes.fragments() {
                            control.checkpoint(records)?;
                            hash.update(fragment);
                        }
                        let digest: String = hash.finalize().iter().map(|byte| format!("{byte:02x}")).collect();
                        if digest == target.command_hash {
                            if bytes.len() > super::command::COMMAND_MAX_BYTES || transaction.matched {
                                return Err(InferenceErrorV1::Invalid);
                            }
                            let mut exact = super::InferencePrivateBytesV1::new(Vec::with_capacity(bytes.len()), super::command::COMMAND_MAX_BYTES)?;
                            for fragment in bytes.fragments() {
                                control.checkpoint(records)?;
                                exact.0.extend_from_slice(fragment);
                            }
                            let decoded = super::command::CanonicalInferenceCommandV1::decode(exact.as_slice())?;
                            if !decoded.matches_identity(&target.mutation_id, &document.0, &target.actor) {
                                return Err(InferenceErrorV1::Invalid);
                            }
                            transaction.matched = true;
                        }
                    }
                }
            }
            Ok(())
        })();
        for _ in 0..CLOSE_MAX_STEPS {
            if !record.close_step().map_err(|_| InferenceErrorV1::Storage)? {
                break;
            }
            tokio::task::yield_now().await;
        }
        if !record.terminal_is_empty() {
            return Err(InferenceErrorV1::Storage);
        }
        checked?;
        control.checkpoint(records)?;
        #[cfg(test)]
        if records == 1 {
            if let Some(gate) = &_state.replay_gate {
                gate.acquire().await.map_err(|_| InferenceErrorV1::Storage)?.forget();
            }
        }
        tokio::task::yield_now().await;
    }
    if active.is_some() {
        return Err(InferenceErrorV1::Invalid);
    }
    control.checkpoint(records)?;
    target.validate(fence)?;
    Ok(matched_transaction.map(|transaction_id| CommittedInferenceWalWitnessV1 {
        fence: fence.clone(),
        scope: target.scope.clone(),
        generation: target.generation,
        job_id: target.job_id.clone(),
        proposal_hash: target.proposal_hash.clone(),
        mutation_id: target.mutation_id.clone(),
        command_hash: target.command_hash.clone(),
        transaction_id,
    }))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(super) mod tests;
