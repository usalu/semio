//! 🔐️ The retained GIS Map approval committer: state and turn types, request guards, maintenance tasks, construction and recovery.

use std::sync::{Arc, Mutex};
use directory::os_directory::{ArtifactFrontier, DocumentScope};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use semio_hub_inference::command::CanonicalInferenceCommandV1;
use semio_hub_inference_binding::{ComposedMapCommitBindingV1, MapBindingRefusalV1};
use std::future::Future;
use semio_hub_inference_runtime_contract::{GIS_MAP_COMMITTER_CAPACITY, GisMapApprovalCheckpointPublisherV1, GisMapApprovalCommitErrorV1, GisMapApprovalCommitRequestV1, GisMapApprovalCommitterV1, GisMapApprovalIngressAuthorityV1, GisMapApprovalReceiptV1, GisMapApprovalRequestTokenV1, GisMapApprovalUndoCommitReceiptV1, GisMapApprovalUndoCommitRequestV1, GisMapAssemblyV1, GisMapClosingStoresV1, GisMapCommitHostV1, GisMapCommitIdentityV1, GisMapCommitOperationV1, GisMapDocumentStoresV1, GisMapDocumentWriteLeaseV1, GisMapJournalReceiptV1, GisMapParentSnapshotV1, GisMapRecoveryOwnerV1, InferenceMapBaseV1, approval_actor, approval_mutation_id, document_key, gis_map_storage_refusal, inference_budget, parse_frontier_chain_hash, record_identity_mismatch};
use std::collections::HashMap;
use semio_hub_inference::wal::{InferenceDocumentFenceV1, InferenceWalVerifierV1};
use semio_hub_inference_ledger::{InferenceJobLedgerV1, InferenceReaderV1};
use semio_hub_inference::{InferencePrivateBytesV1, sha256};

pub enum RetainedGisMapDocumentStateV1<B: ComposedMapCommitBindingV1> {
    Recovery {
        owner: Arc<tokio::sync::Mutex<GisMapRecoveryOwnerV1<B>>>,
        handle: db::ArtifactHandle,
        scope: DocumentScope,
        generation: u64,
        document_write: GisMapDocumentWriteLeaseV1,
        fence: Arc<InferenceDocumentFenceV1>,
    },
    Recovered {
        owners: GisMapDocumentStoresV1<B>,
        checkpoint: db::document::ArtifactDurableGroupRecoveredCheckpointV1,
        document_write: GisMapDocumentWriteLeaseV1,
    },
    Ready {
        owners: GisMapDocumentStoresV1<B>,
        pending: Option<GisMapCommitIdentityV1>,
        document_write: Option<GisMapDocumentWriteLeaseV1>,
    },
    Assembly {
        owner: GisMapAssemblyV1<B>,
        handle: db::ArtifactHandle,
        scope: DocumentScope,
        generation: u64,
        identity: GisMapCommitIdentityV1,
        document_write: GisMapDocumentWriteLeaseV1,
        fence: Arc<InferenceDocumentFenceV1>,
    },
    Journal {
        owner: GisMapCommitHostV1<B>,
        handle: db::ArtifactHandle,
        scope: DocumentScope,
        generation: u64,
        identity: GisMapCommitIdentityV1,
        receipt: Option<GisMapJournalReceiptV1>,
        document_write: GisMapDocumentWriteLeaseV1,
        fence: Arc<InferenceDocumentFenceV1>,
    },
    Verification {
        owners: GisMapDocumentStoresV1<B>,
        identity: GisMapCommitIdentityV1,
        receipt: GisMapJournalReceiptV1,
        document_write: GisMapDocumentWriteLeaseV1,
    },
    Publishing {
        owners: GisMapDocumentStoresV1<B>,
        identity: GisMapCommitIdentityV1,
        receipt: GisMapJournalReceiptV1,
        document_write: GisMapDocumentWriteLeaseV1,
    },
    Published {
        owners: GisMapDocumentStoresV1<B>,
        identity: GisMapCommitIdentityV1,
        receipt: GisMapJournalReceiptV1,
        document_write: Option<GisMapDocumentWriteLeaseV1>,
    },
    Closing(GisMapClosingStoresV1<B>),
}

/// 🔁️ What one commit turn answered. Every non-terminal turn — `Preflight` included — yields to the
/// runtime once before the next one runs, so dropping the approval lands on an exact retained state
/// instead of the whole approval racing through assembly, journal and publication inside one poll.
pub(crate) enum GisMapCommitTurnV1 {
    Continue,
    Committed,
    Preflight { handle: db::ArtifactHandle, generation: u64 },
    Verify { generation: u64, receipt: GisMapJournalReceiptV1 },
    Rejected(GisMapApprovalCommitErrorV1),
}

pub(crate) enum GisMapAbandonedTurnV1 {
    Complete,
    Aborted,
    Continue,
    Committed,
    Recover,
    Recovered,
    Verify { identity: GisMapCommitIdentityV1, generation: u64, receipt: GisMapJournalReceiptV1 },
    Retry,
}

#[derive(Clone)]
pub struct GisMapPreparedApprovalV1 {
    pub scope: DocumentScope,
    pub job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub proposal_hash: String,
    pub ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
}

#[derive(Clone)]
pub struct GisMapAbandonedRequestV1 {
    pub key: String,
    pub identity: GisMapCommitIdentityV1,
    pub request: Arc<GisMapApprovalRequestTokenV1>,
}

pub(crate) struct GisMapApprovalRequestOwnerV1<B: ComposedMapCommitBindingV1> {
    pub(crate) committer: RetainedGisMapApprovalCommitterV1<B>,
    pub(crate) key: String,
    pub(crate) identity: Option<GisMapCommitIdentityV1>,
    pub(crate) prepared: Option<GisMapPreparedApprovalV1>,
    pub(crate) request: Arc<GisMapApprovalRequestTokenV1>,
}

impl<B: ComposedMapCommitBindingV1> GisMapApprovalRequestOwnerV1<B> {
    pub(crate) fn admit(&mut self, identity: GisMapCommitIdentityV1) {
        self.prepared = None;
        self.identity = Some(identity);
    }

    pub(crate) fn complete(&mut self) {
        let job_id = self.identity.as_ref().map(|identity| identity.job_id.as_str()).or_else(|| self.prepared.as_ref().map(|prepared| prepared.job_id.as_str())).map(str::to_owned);
        self.prepared = None;
        self.identity = None;
        if let Some(job_id) = job_id {
            self.committer.release_cleanup_job(&job_id);
        }
    }
}

impl<B: ComposedMapCommitBindingV1> Drop for GisMapApprovalRequestOwnerV1<B> {
    fn drop(&mut self) {
        let Some(identity) = self.identity.take() else {
            if let Some(prepared) = self.prepared.take() {
                self.committer.spawn_abandoned_prepared(prepared);
            }
            return;
        };
        self.committer.spawn_abandoned_request(GisMapAbandonedRequestV1 { key: self.key.clone(), identity, request: self.request.clone() });
    }
}

/// 🏠️ Per-document fixed-three Store owner; only a sole committed WAL event can finish verification, and production registration remains fail-closed until Hub shares its document-write fence and mounts recovery.
pub struct RetainedGisMapApprovalCommitterV1<B: ComposedMapCommitBindingV1> {
    pub database: Arc<db::Database>,
    pub(crate) verifier: Arc<InferenceWalVerifierV1>,
    pub ledger: Arc<InferenceJobLedgerV1>,
    pub(crate) publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1>,
    pub documents: Arc<tokio::sync::Mutex<HashMap<String, RetainedGisMapDocumentStateV1<B>>>>,
    next_operation: Arc<AtomicU64>,
    pub maintenance: Arc<Mutex<std::collections::HashSet<String>>>,
    pub cleanup_jobs: Arc<Mutex<HashMap<String, u16>>>,
    pub(crate) closing: Arc<AtomicBool>,
    pub parked_prepared: Arc<Mutex<HashMap<String, GisMapPreparedApprovalV1>>>,
    pub parked_requests: Arc<Mutex<HashMap<String, GisMapAbandonedRequestV1>>>,
    pub(crate) state_epoch: Arc<tokio::sync::watch::Sender<u64>>,
}

impl<B: ComposedMapCommitBindingV1> Clone for RetainedGisMapApprovalCommitterV1<B> {
    fn clone(&self) -> Self {
        Self {
            database: self.database.clone(),
            verifier: self.verifier.clone(),
            ledger: self.ledger.clone(),
            publisher: self.publisher.clone(),
            documents: self.documents.clone(),
            next_operation: self.next_operation.clone(),
            maintenance: self.maintenance.clone(),
            cleanup_jobs: self.cleanup_jobs.clone(),
            closing: self.closing.clone(),
            parked_prepared: self.parked_prepared.clone(),
            parked_requests: self.parked_requests.clone(),
            state_epoch: self.state_epoch.clone(),
        }
    }
}

/// 🚪️ The `closing` bit for the lifetime of ONE `close()` future. `close()` raises the bit in its
/// first statement and the bit refuses every later `reserve_cleanup_job`, so a close future that is
/// polled once and then DROPPED — which callers do to prove that close joins the active driver
/// rather than starting a second one — used to leave the committer permanently refusing every
/// commit and undo with `Unavailable`, with no close in flight to lift it. A cancelled or errored
/// close is retryable, so the bit falls with the future; only a close that reached its terminal
/// handoff surrenders the cursor and leaves the committer closed for good.
pub(crate) struct GisMapClosingCursorV1 {
    pub(crate) closing: Arc<AtomicBool>,
    pub(crate) state_epoch: Arc<tokio::sync::watch::Sender<u64>>,
    pub(crate) retained: bool,
}

impl GisMapClosingCursorV1 {
    pub(crate) fn surrender(&mut self) {
        self.retained = false;
    }
}

impl Drop for GisMapClosingCursorV1 {
    fn drop(&mut self) {
        if !self.retained {
            return;
        }
        self.closing.store(false, Ordering::Release);
        self.state_epoch.send_modify(|epoch| *epoch = epoch.wrapping_add(1));
    }
}

pub struct GisMapAbandonedRequestTaskV1<B: ComposedMapCommitBindingV1> {
    pub committer: RetainedGisMapApprovalCommitterV1<B>,
    pub maintenance_key: String,
    pub owner: Option<GisMapAbandonedRequestV1>,
}

struct GisMapMaintenanceCompletionV1 {
    pub(crate) maintenance: Arc<Mutex<std::collections::HashSet<String>>>,
    pub(crate) cleanup_jobs: Arc<Mutex<HashMap<String, u16>>>,
    pub(crate) state_epoch: Arc<tokio::sync::watch::Sender<u64>>,
    pub(crate) maintenance_key: String,
    pub(crate) job_id: Option<String>,
}

impl GisMapMaintenanceCompletionV1 {
    pub(crate) fn complete(self) {
        self.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&self.maintenance_key);
        if let Some(job_id) = self.job_id {
            let mut jobs = self.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(owners) = jobs.get_mut(&job_id) {
                if *owners == 1 {
                    jobs.remove(&job_id);
                } else {
                    *owners -= 1;
                }
            }
        }
        self.state_epoch.send_modify(|epoch| *epoch = epoch.wrapping_add(1));
    }
}

impl<B: ComposedMapCommitBindingV1> GisMapAbandonedRequestTaskV1<B> {
    pub(crate) fn complete(mut self) {
        let completion = GisMapMaintenanceCompletionV1 {
            maintenance: self.committer.maintenance.clone(),
            cleanup_jobs: self.committer.cleanup_jobs.clone(),
            state_epoch: self.committer.state_epoch.clone(),
            maintenance_key: self.maintenance_key.clone(),
            job_id: self.owner.as_ref().map(|owner| owner.identity.job_id.clone()),
        };
        self.owner = None;
        drop(self);
        completion.complete();
    }
}

impl<B: ComposedMapCommitBindingV1> Drop for GisMapAbandonedRequestTaskV1<B> {
    fn drop(&mut self) {
        let Some(owner) = self.owner.take() else { return };
        self.committer.park_abandoned_request(owner);
        self.committer.announce_state_change();
    }
}

pub struct GisMapPreparedApprovalTaskV1<B: ComposedMapCommitBindingV1> {
    pub committer: RetainedGisMapApprovalCommitterV1<B>,
    pub maintenance_key: String,
    pub owner: Option<GisMapPreparedApprovalV1>,
}

impl<B: ComposedMapCommitBindingV1> GisMapPreparedApprovalTaskV1<B> {
    pub(crate) fn complete(mut self) {
        let completion = GisMapMaintenanceCompletionV1 {
            maintenance: self.committer.maintenance.clone(),
            cleanup_jobs: self.committer.cleanup_jobs.clone(),
            state_epoch: self.committer.state_epoch.clone(),
            maintenance_key: self.maintenance_key.clone(),
            job_id: self.owner.as_ref().map(|owner| owner.job_id.clone()),
        };
        self.owner = None;
        drop(self);
        completion.complete();
    }
}

impl<B: ComposedMapCommitBindingV1> Drop for GisMapPreparedApprovalTaskV1<B> {
    fn drop(&mut self) {
        let Some(owner) = self.owner.take() else { return };
        self.committer.park_prepared_approval(owner);
        self.committer.announce_state_change();
    }
}

impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B> {
    pub fn new(database: Arc<db::Database>, storage: Arc<db::storage::DbBackend>, ledger: Arc<InferenceJobLedgerV1>, publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1>) -> Self {
        Self {
            database,
            verifier: Arc::new(InferenceWalVerifierV1::new::<B>(storage)),
            ledger,
            publisher,
            documents: Arc::new(tokio::sync::Mutex::new(HashMap::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
            next_operation: Arc::new(AtomicU64::new(1)),
            maintenance: Arc::new(Mutex::new(std::collections::HashSet::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
            cleanup_jobs: Arc::new(Mutex::new(HashMap::with_capacity(semio_hub_inference::schema::JOB_CAPACITY))),
            closing: Arc::new(AtomicBool::new(false)),
            parked_prepared: Arc::new(Mutex::new(HashMap::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
            parked_requests: Arc::new(Mutex::new(HashMap::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
            state_epoch: Arc::new(tokio::sync::watch::channel(0).0),
        }
    }

    pub(crate) fn announce_state_change(&self) {
        self.state_epoch.send_modify(|epoch| *epoch = epoch.wrapping_add(1));
    }

    pub fn reserve_cleanup_job(&self, job_id: &str) -> Result<(), GisMapApprovalCommitErrorV1> {
        let mut jobs = self.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.closing.load(Ordering::Acquire) {
            return Err(GisMapApprovalCommitErrorV1::Unavailable);
        }
        if let Some(owners) = jobs.get_mut(job_id) {
            let Some(next) = owners.checked_add(1) else { return Err(GisMapApprovalCommitErrorV1::Capacity) };
            *owners = next;
            return Ok(());
        }
        if jobs.len() >= semio_hub_inference::schema::JOB_CAPACITY {
            return Err(GisMapApprovalCommitErrorV1::Capacity);
        }
        jobs.insert(job_id.to_owned(), 1);
        Ok(())
    }

    pub fn validate_prepared_request(&self, request: &GisMapApprovalCommitRequestV1<'_>) -> Result<(), GisMapApprovalCommitErrorV1> {
        let (outbox, identity, _) = self
            .ledger
            .approval_recovery_by_mutation(request.mutation_id)
            .map_err(|error| gis_map_storage_refusal(&error))?
            .ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
        let chain_hash = parse_frontier_chain_hash(&identity.chain_hash).ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
        if outbox.job_id != request.job_id
            || outbox.mutation_id != request.mutation_id
            || outbox.command_hash != request.command_hash
            || outbox.proposal_hash != request.proposal_hash
            || identity.space_id != request.scope.space_id
            || identity.document_id != request.scope.document_id
            || request.ingress.scope() != request.scope
            || identity.user_id != request.ingress.user_id()
            || identity.session_id != request.ingress.session_id()
            || identity.authorization_generation != request.ingress.authorization_generation()
            || identity.descriptor_digest != request.base.descriptor_digest
            || identity.document_id != request.base.frontier.document_id
            || identity.head_ordinal != request.base.frontier.head_edit_ordinal
            || identity.head_edit_id != request.base.frontier.head_edit_id
            || identity.last_commit_seq != request.base.frontier.last_commit_seq
            || chain_hash != request.base.frontier.chain_hash
            || identity.input_hash != request.base.digest()
            || request.base_frontier != &request.base.frontier
        {
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        Ok(())
    }

    pub fn release_cleanup_job(&self, job_id: &str) {
        let mut jobs = self.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(owners) = jobs.get_mut(job_id) else { return };
        if *owners == 1 {
            jobs.remove(job_id);
        } else {
            *owners -= 1;
        }
        drop(jobs);
        self.announce_state_change();
    }

    fn park_abandoned_request(&self, owner: GisMapAbandonedRequestV1) {
        let job_id = owner.identity.job_id.clone();
        let replaced = self.parked_requests.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(job_id.clone(), owner);
        if replaced.is_some() {
            self.release_cleanup_job(&job_id);
        }
    }

    fn park_prepared_approval(&self, owner: GisMapPreparedApprovalV1) {
        let job_id = owner.job_id.clone();
        let replaced = self.parked_prepared.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(job_id.clone(), owner);
        if replaced.is_some() {
            self.release_cleanup_job(&job_id);
        }
    }

    pub fn abandoned_request_maintenance_key(owner: &GisMapAbandonedRequestV1) -> String {
        format!("request:{}:{}:{}", owner.key.len(), owner.key, owner.identity.job_id)
    }

    pub fn maintenance_owns_document(&self, key: &str) -> bool {
        let prefix = format!("request:{}:{key}:", key.len());
        self.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().any(|candidate| candidate.starts_with(&prefix))
    }

    fn spawn_abandoned_request(&self, owner: GisMapAbandonedRequestV1) {
        let maintenance_key = Self::abandoned_request_maintenance_key(&owner);
        let mut maintenance = self.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !maintenance.insert(maintenance_key.clone()) {
            self.release_cleanup_job(&owner.identity.job_id);
            return;
        }
        drop(maintenance);
        let committer = self.clone();
        match tokio::runtime::Handle::try_current() {
            Ok(runtime) => {
                let task = GisMapAbandonedRequestTaskV1 { committer, maintenance_key, owner: Some(owner) };
                runtime.spawn(async move {
                    let owner = task.owner.as_ref().expect("abandoned request task retains its exact owner").clone();
                    task.committer.drive_abandoned_request(&owner.key, owner.identity, owner.request).await;
                    task.complete();
                });
            }
            Err(_) => {
                self.park_abandoned_request(owner);
            }
        }
    }

    pub fn resume_parked_requests(&self) {
        let Ok(runtime) = tokio::runtime::Handle::try_current() else { return };
        let parked = {
            let mut owners = self.parked_requests.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            owners.drain().map(|(_, owner)| (Self::abandoned_request_maintenance_key(&owner), owner)).collect::<Vec<_>>()
        };
        for (maintenance_key, owner) in parked {
            let task = GisMapAbandonedRequestTaskV1 { committer: self.clone(), maintenance_key, owner: Some(owner) };
            runtime.spawn(async move {
                let owner = task.owner.as_ref().expect("resumed abandoned request task retains its exact owner").clone();
                task.committer.drive_abandoned_request(&owner.key, owner.identity, owner.request).await;
                task.complete();
            });
        }
    }

    fn spawn_abandoned_prepared(&self, prepared: GisMapPreparedApprovalV1) {
        let maintenance_key = format!("prepared:{}", prepared.job_id);
        let mut maintenance = self.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !maintenance.insert(maintenance_key.clone()) {
            self.release_cleanup_job(&prepared.job_id);
            return;
        }
        drop(maintenance);
        let committer = self.clone();
        match tokio::runtime::Handle::try_current() {
            Ok(runtime) => {
                let task = GisMapPreparedApprovalTaskV1 { committer, maintenance_key, owner: Some(prepared) };
                runtime.spawn(async move {
                    let prepared = task.owner.as_ref().expect("prepared approval task retains its exact owner").clone();
                    task.committer.drive_abandoned_prepared(prepared).await;
                    task.complete();
                });
            }
            Err(_) => {
                let reader = InferenceReaderV1 {
                    user_id: prepared.ingress.user_id(),
                    session_id: prepared.ingress.session_id(),
                    authorization_generation: prepared.ingress.authorization_generation(),
                    space_id: &prepared.scope.space_id,
                    document_id: &prepared.scope.document_id,
                };
                if self.ledger.abandon_prepared_approval(&reader, &prepared.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash).is_ok() {
                    self.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&maintenance_key);
                    self.release_cleanup_job(&prepared.job_id);
                    self.announce_state_change();
                } else {
                    self.park_prepared_approval(prepared);
                }
            }
        }
    }

    pub fn resume_parked_prepared(&self) {
        let Ok(runtime) = tokio::runtime::Handle::try_current() else { return };
        let parked = {
            let mut owners = self.parked_prepared.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            owners.drain().map(|(job_id, owner)| (format!("prepared:{job_id}"), owner)).collect::<Vec<_>>()
        };
        for (maintenance_key, prepared) in parked {
            let task = GisMapPreparedApprovalTaskV1 { committer: self.clone(), maintenance_key, owner: Some(prepared) };
            runtime.spawn(async move {
                let prepared = task.owner.as_ref().expect("resumed prepared approval task retains its exact owner").clone();
                task.committer.drive_abandoned_prepared(prepared).await;
                task.complete();
            });
        }
    }

    async fn drive_abandoned_prepared(&self, prepared: GisMapPreparedApprovalV1) {
        let mut retry_delay = std::time::Duration::from_millis(1);
        loop {
            let reader = InferenceReaderV1 {
                user_id: prepared.ingress.user_id(),
                session_id: prepared.ingress.session_id(),
                authorization_generation: prepared.ingress.authorization_generation(),
                space_id: &prepared.scope.space_id,
                document_id: &prepared.scope.document_id,
            };
            if self.ledger.abandon_prepared_approval(&reader, &prepared.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash).is_ok() {
                return;
            }
            tokio::time::sleep(retry_delay).await;
            retry_delay = retry_delay.saturating_mul(2).min(std::time::Duration::from_secs(1));
        }
    }

    pub(crate) fn stores_match(owners: &GisMapDocumentStoresV1<B>, snapshot: &GisMapParentSnapshotV1<B>) -> bool {
        let (drawing, value) = B::derived_members(snapshot);
        owners.parent.as_ref().is_some_and(|store| store.snapshot_ref() == snapshot) && owners.drawing.as_ref().is_some_and(|store| store.snapshot_ref() == &drawing) && owners.value.as_ref().is_some_and(|store| store.snapshot_ref() == &value)
    }

    /// 🧷️ Answers whether the retained three Stores still fold to one consistent parent-derived
    /// triple, at whatever frontier the document has reached.
    ///
    /// `stores_match` answers the same question *against a base pack*, which is only the live state
    /// before the document's first commit: the `pack` half of an `ArtifactPair` is the genesis and
    /// the live state is pack + spr folded. Past the first commit the advanced Stores can never
    /// equal the genesis pack, so every post-commit join has to read the Stores themselves.
    pub(crate) fn stores_consistent(owners: &GisMapDocumentStoresV1<B>) -> bool {
        let Some(parent) = owners.parent.as_ref() else { return false };
        let (drawing, value) = B::derived_members(parent.snapshot_ref());
        owners.drawing.as_ref().is_some_and(|store| store.snapshot_ref() == &drawing) && owners.value.as_ref().is_some_and(|store| store.snapshot_ref() == &value)
    }

    /// 🪜️ Answers whether a base names the exact frontier one retained publication produced.
    ///
    /// A publication advances its own base by exactly one commit carrying its own mutation, which is
    /// the projection `publish` already verifies against the actor checkpoint. The pack is unchanged
    /// (it is the genesis half), so the base digest and descriptor stay the publication's own.
    pub(crate) fn publication_result_base(identity: &GisMapCommitIdentityV1, frontier: &ArtifactFrontier, digest: &str, descriptor_digest: &str) -> bool {
        identity.base_digest == digest
            && identity.descriptor_digest == descriptor_digest
            && frontier.document_id == identity.base_frontier.document_id
            && frontier.head_edit_id == identity.mutation_id
            && identity.base_frontier.head_edit_ordinal.checked_add(1) == Some(frontier.head_edit_ordinal)
            && identity.base_frontier.last_commit_seq.checked_add(1) == Some(frontier.last_commit_seq)
    }

    pub(crate) fn checkpoint_matches_frontier(snapshot: &db::CheckpointPublicationSnapshot, generation: u64, frontier: &ArtifactFrontier, scope: &DocumentScope) -> bool {
        snapshot.authority_generation == generation
            && snapshot.frontier.document.0 == document_key(scope)
            && snapshot.frontier.head_seq == frontier.head_edit_ordinal
            && snapshot.frontier.commit_seq == frontier.last_commit_seq
            && snapshot.frontier.chain_hash == frontier.chain_hash.0
            && snapshot.head_edit_id.as_ref().map_or("", |identity| identity.0.as_str()) == frontier.head_edit_id
    }

    fn checkpoint_matches(snapshot: &db::CheckpointPublicationSnapshot, generation: u64, base: &InferenceMapBaseV1, scope: &DocumentScope) -> bool {
        Self::checkpoint_matches_frontier(snapshot, generation, &base.frontier, scope)
    }

    pub fn preflight(request: &GisMapApprovalCommitRequestV1<'_>) -> Result<(GisMapCommitIdentityV1, GisMapParentSnapshotV1<B>), GisMapApprovalCommitErrorV1> {
        let (snapshot, work) = B::executed_approval_work(request.base.pack.as_slice(), request.job_id, &inference_budget()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        if request.composed_children.len() != 2 || request.composed_children[0] != work.drawing_child_id || request.composed_children[1] != work.value_child_id {
            return Err(GisMapApprovalCommitErrorV1::Rejected);
        }
        let proposal = work.parent_json;
        let inverse = work.parent_inverse_json;
        let command = CanonicalInferenceCommandV1::decode(request.command).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let actor = request.actor.strip_prefix("user:").and_then(|value| value.split_once("#session:"));
        let admitted_actor = actor.is_some_and(|(user, session)| request.ingress.scope() == request.scope && request.ingress.user_id() == user && request.ingress.session_id() == session && request.ingress.authorization_generation() != 0);
        if request.proposal_hash != sha256(&proposal)
            || request.mutation_id != approval_mutation_id(request.job_id, request.proposal_hash)
            || request.command_hash != sha256(request.command)
            || request.base_frontier != &request.base.frontier
            || request.deadline_ms <= request.now_ms
            || request.deadline_ms - request.now_ms > semio_hub_inference::schema::JOB_MAX_LIFETIME_MS
            || !actor.is_some_and(|(user, session)| semio_hub_inference::schema::server_id(user) && semio_hub_inference::schema::server_id(session))
            || !admitted_actor
            || !command.matches_fixed_three_parent(request.mutation_id, &document_key(request.scope), request.actor, &proposal, &inverse)
        {
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        Ok((
            GisMapCommitIdentityV1 {
                scope: request.scope.clone(),
                actor: request.actor.to_owned(),
                mutation_id: request.mutation_id.to_owned(),
                command_hash: request.command_hash.to_owned(),
                job_id: request.job_id.to_owned(),
                proposal_hash: request.proposal_hash.to_owned(),
                base_frontier: request.base_frontier.clone(),
                descriptor_digest: request.base.descriptor_digest.clone(),
                base_digest: request.base.digest(),
                timestamp: command.timestamp(),
                journal_now_ms: request.now_ms,
                document_write: request.document_write.clone(),
                ingress: Some(request.ingress.clone()),
                operation: GisMapCommitOperationV1::Approval,
            },
            snapshot,
        ))
    }

    pub(crate) fn preflight_undo(request: &GisMapApprovalUndoCommitRequestV1<'_>) -> Result<(GisMapCommitIdentityV1, GisMapParentSnapshotV1<B>), GisMapApprovalCommitErrorV1> {
        let snapshot = B::decode_parent(request.base.pack.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let actor = request.actor.strip_prefix("user:").and_then(|value| value.split_once("#session:"));
        let target = request.target;
        let expected_frontier = Self::publication_wire_frontier(&request.base.frontier);
        let command = CanonicalInferenceCommandV1::decode(request.command).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let original = CanonicalInferenceCommandV1::decode(target.original_command.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let undo = B::undo_work(&snapshot, original.inverse_payload(), &target.original_job_id).map_err(|refusal| match refusal {
            MapBindingRefusalV1::Undecodable => GisMapApprovalCommitErrorV1::Rejected,
            _ => GisMapApprovalCommitErrorV1::Conflict,
        })?;
        let undo_forward = undo.forward_json;
        let undo_inverse = undo.inverse_json;
        if target.scope != *request.scope
            || target.user_id != request.ingress.user_id()
            || target.session_id != request.ingress.session_id()
            || target.authorization_generation != request.ingress.authorization_generation()
            || request.ingress.scope() != request.scope
            || target.after_frontier != expected_frontier
            || request.command_hash != sha256(request.command)
            || request.proposal_hash != sha256(&undo_forward)
            || request.mutation_id != approval_mutation_id(request.operation_id, request.proposal_hash)
            || request.composed_children != B::COMMIT_MEMBERS
            || request.deadline_ms <= request.now_ms
            || request.deadline_ms - request.now_ms > semio_hub_inference::schema::JOB_MAX_LIFETIME_MS
            || !actor.is_some_and(|(user, session)| user == target.user_id && session == target.session_id)
            || !command.matches_fixed_three_parent(request.mutation_id, &document_key(request.scope), request.actor, &undo_forward, &undo_inverse)
        {
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        Ok((
            GisMapCommitIdentityV1 {
                scope: request.scope.clone(),
                actor: request.actor.to_owned(),
                mutation_id: request.mutation_id.to_owned(),
                command_hash: request.command_hash.to_owned(),
                job_id: request.operation_id.to_owned(),
                proposal_hash: request.proposal_hash.to_owned(),
                base_frontier: request.base.frontier.clone(),
                descriptor_digest: request.base.descriptor_digest.clone(),
                base_digest: request.base.digest(),
                timestamp: command.timestamp(),
                journal_now_ms: request.now_ms,
                document_write: request.document_write.clone(),
                ingress: Some(request.ingress.clone()),
                operation: GisMapCommitOperationV1::Undo {
                    target_id: target.target_id.clone(),
                    idempotency_key: request.idempotency_key.to_owned(),
                    original_job_id: target.original_job_id.clone(),
                    original_command: Arc::new(InferencePrivateBytesV1::new(target.original_command.as_slice().to_vec(), semio_hub_inference::command::COMMAND_MAX_BYTES).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?),
                },
            },
            snapshot,
        ))
    }

    pub(crate) fn identity_matches(identity: &GisMapCommitIdentityV1, candidate: &GisMapCommitIdentityV1) -> bool {
        match Self::first_identity_mismatch(identity, candidate) {
            None => true,
            Some(field) => {
                record_identity_mismatch(field);
                false
            }
        }
    }

    /// 🔬️ The FIRST field on which a retained identity and a joining candidate differ, in the order
    /// the join compares them. `identity_matches` answers a bool, so a request refused as `Conflict`
    /// against a retained publication cannot say which of its twelve conjuncts was false. This names
    /// it for the recorder.
    fn first_identity_mismatch(identity: &GisMapCommitIdentityV1, candidate: &GisMapCommitIdentityV1) -> Option<&'static str> {
        if identity.scope != candidate.scope {
            return Some("scope");
        }
        if identity.actor != candidate.actor {
            return Some("actor");
        }
        if identity.mutation_id != candidate.mutation_id {
            return Some("mutation_id");
        }
        if identity.command_hash != candidate.command_hash {
            return Some("command_hash");
        }
        if identity.job_id != candidate.job_id {
            return Some("job_id");
        }
        if identity.proposal_hash != candidate.proposal_hash {
            return Some("proposal_hash");
        }
        if identity.base_frontier != candidate.base_frontier {
            return Some("base_frontier");
        }
        if identity.descriptor_digest != candidate.descriptor_digest {
            return Some("descriptor_digest");
        }
        if identity.base_digest != candidate.base_digest {
            return Some("base_digest");
        }
        if !Arc::ptr_eq(&identity.document_write, &candidate.document_write) {
            return Some("document_write");
        }
        let operation = match (&identity.operation, &candidate.operation) {
            (GisMapCommitOperationV1::Approval, GisMapCommitOperationV1::Approval) => true,
            (
                GisMapCommitOperationV1::Undo { target_id: left_target, idempotency_key: left_key, original_job_id: left_job, original_command: left_command },
                GisMapCommitOperationV1::Undo { target_id: right_target, idempotency_key: right_key, original_job_id: right_job, original_command: right_command },
            ) => left_target == right_target && left_key == right_key && left_job == right_job && left_command.as_slice() == right_command.as_slice(),
            _ => false,
        };
        if !operation {
            return Some("operation");
        }
        let ingress = match (&identity.ingress, &candidate.ingress) {
            (Some(left), Some(right)) => Arc::ptr_eq(left, right),
            (None, Some(_)) | (None, None) => true,
            _ => false,
        };
        if !ingress {
            return Some("ingress");
        }
        None
    }

    pub(crate) fn reserve_operations(&self) -> Result<[semio_framework_job::OperationId; 3], GisMapApprovalCommitErrorV1> {
        let start = self.next_operation.try_update(Ordering::AcqRel, Ordering::Acquire, |value| value.checked_add(3)).map_err(|_| GisMapApprovalCommitErrorV1::Capacity)?;
        Ok([semio_framework_job::OperationId(start), semio_framework_job::OperationId(start + 1), semio_framework_job::OperationId(start + 2)])
    }

    /// 📍 Installs one exact document actor and an actor-owned committed-Event recovery before any
    /// approval can enter.
    pub(crate) async fn mount_document(&self, scope: DocumentScope, handle: db::ArtifactHandle, base: &InferenceMapBaseV1, document_write: GisMapDocumentWriteLeaseV1, actor: &str) -> Result<bool, GisMapApprovalCommitErrorV1> {
        let snapshot = B::decode_parent(base.pack.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let observed = handle.checkpoint_publication_snapshot().await.map_err(|error| gis_map_storage_refusal(&error))?;
        if !Self::checkpoint_matches(&observed, observed.authority_generation, base, &scope) {
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        let key = document_key(&scope);
        let mut documents = self.documents.lock().await;
        if let Some(state) = documents.get(&key) {
            let matches = match state {
                RetainedGisMapDocumentStateV1::Ready { owners, .. } => {
                    owners.scope == scope && owners.generation == observed.authority_generation && Arc::ptr_eq(&owners.document_write, &document_write.gate) && Self::stores_match(owners, &snapshot)
                }
                RetainedGisMapDocumentStateV1::Published { owners, .. } => {
                    owners.scope == scope && owners.generation == observed.authority_generation && Arc::ptr_eq(&owners.document_write, &document_write.gate) && (Self::stores_match(owners, &snapshot) || Self::stores_consistent(owners))
                }
                RetainedGisMapDocumentStateV1::Recovered { owners, document_write: retained, .. } => owners.scope == scope && owners.generation == observed.authority_generation && Arc::ptr_eq(&retained.gate, &document_write.gate),
                _ => false,
            };
            return if matches { Ok(false) } else { Err(GisMapApprovalCommitErrorV1::Conflict) };
        }
        if documents.len() >= GIS_MAP_COMMITTER_CAPACITY {
            return Err(GisMapApprovalCommitErrorV1::Capacity);
        }
        let fence = Arc::new(InferenceDocumentFenceV1::new(scope.clone(), observed.authority_generation).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?);
        let members = B::member_stores(&key, snapshot, actor);
        let admission = directory::os_store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(members.parent, members.drawing, members.value);
        let recovery = handle.durable_group_recovery_retained(admission);
        documents.insert(key, RetainedGisMapDocumentStateV1::Recovery { owner: Arc::new(tokio::sync::Mutex::new(recovery)), handle, scope, generation: observed.authority_generation, document_write, fence });
        Ok(true)
    }

    pub(crate) async fn finish_document_recovery(&self, key: &str) -> Result<(), GisMapApprovalCommitErrorV1> {
        let (scan, scan_handle) = {
            let documents = self.documents.lock().await;
            let Some(state) = documents.get(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
            let RetainedGisMapDocumentStateV1::Recovery { owner, handle, .. } = state else { return Ok(()) };
            (owner.clone(), handle.clone())
        };
        let advanced = {
            let mut owner = scan.lock().await;
            if !owner.scan_is_active() {
                if let Err(error) = scan_handle.resume_durable_group_recovery(&mut owner) {
                    return Err(gis_map_storage_refusal(&error));
                }
            }
            match owner.advance().await {
                Ok(true) => Ok((owner.recovered_checkpoint(), owner.take_terminal_owners())),
                other => Err(gis_map_storage_refusal(&other)),
            }
        };
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        let RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence } = state else {
            documents.insert(key.to_owned(), state);
            return Ok(());
        };
        match advanced {
            Ok((recovered_checkpoint, Some(owners))) => {
                let owners = Self::restored_stores(owners.parent, owners.drawing, owners.value, handle, scope, generation, document_write.gate.clone(), fence);
                if let Some(checkpoint) = recovered_checkpoint {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write });
                } else {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: Some(document_write) });
                }
                Ok(())
            }
            Ok((_, None)) => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                Err(gis_map_storage_refusal(&"a completed recovery scan returned no terminal owners"))
            }
            Err(error) => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                Err(error)
            }
        }
    }

    pub(crate) async fn resume_recovered_checkpoint(&self, key: &str) -> Result<(GisMapCommitIdentityV1, u64, GisMapJournalReceiptV1), GisMapApprovalCommitErrorV1> {
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        let RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } = state else {
            documents.insert(key.to_owned(), state);
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        };
        let recovered = (|| {
            let mutation_id = checkpoint.head_edit_id().0.as_str();
            if let Some((outbox, accepted, ledger_applied)) = self.ledger.approval_recovery_by_mutation(mutation_id).map_err(|error| gis_map_storage_refusal(&error))? {
                accepted.validate().map_err(|_| GisMapApprovalCommitErrorV1::Conflict)?;
                let scope = DocumentScope::new(accepted.space_id.clone(), accepted.document_id.clone());
                let actor = approval_actor(&accepted);
                let chain_hash = parse_frontier_chain_hash(&accepted.chain_hash).ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
                let command_matches = if outbox.command.as_slice().is_empty() {
                    true
                } else {
                    let command = CanonicalInferenceCommandV1::decode(outbox.command.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Conflict)?;
                    outbox.command_hash == sha256(outbox.command.as_slice()) && command.matches_identity(&outbox.mutation_id, key, &actor)
                };
                if scope != owners.scope || outbox.mutation_id != mutation_id || outbox.mutation_id != approval_mutation_id(&outbox.job_id, &outbox.proposal_hash) || !command_matches {
                    return Err(GisMapApprovalCommitErrorV1::Conflict);
                }
                let identity = GisMapCommitIdentityV1 {
                    scope,
                    actor,
                    mutation_id: outbox.mutation_id,
                    command_hash: outbox.command_hash,
                    job_id: outbox.job_id,
                    proposal_hash: outbox.proposal_hash,
                    base_frontier: ArtifactFrontier { document_id: accepted.document_id, head_edit_ordinal: accepted.head_ordinal, head_edit_id: accepted.head_edit_id, last_commit_seq: accepted.last_commit_seq, chain_hash },
                    descriptor_digest: accepted.descriptor_digest,
                    base_digest: accepted.input_hash,
                    timestamp: protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 },
                    journal_now_ms: outbox.prepared_at_ms,
                    document_write: owners.document_write.clone(),
                    ingress: None,
                    operation: GisMapCommitOperationV1::Approval,
                };
                return Ok((identity, owners.generation, checkpoint.receipt().clone(), ledger_applied));
            }
            let undo = self.ledger.gis_map_approval_undo_recovery_by_mutation(mutation_id).map_err(|error| gis_map_storage_refusal(&error))?.ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
            let target = undo.target;
            let scope = target.scope.clone();
            let actor = format!("user:{}#session:{}", target.user_id, target.session_id);
            let chain_hash = parse_frontier_chain_hash(&target.after_frontier.chain_sha256).ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
            let timestamp = if undo.command.as_slice().is_empty() {
                protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }
            } else {
                let command = CanonicalInferenceCommandV1::decode(undo.command.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Conflict)?;
                if !command.matches_identity(&undo.mutation_id, key, &actor) {
                    return Err(GisMapApprovalCommitErrorV1::Conflict);
                }
                command.timestamp()
            };
            if scope != owners.scope || undo.mutation_id != mutation_id || undo.mutation_id != approval_mutation_id(&undo.operation_id, &undo.proposal_hash) {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let identity = GisMapCommitIdentityV1 {
                scope,
                actor,
                mutation_id: undo.mutation_id,
                command_hash: undo.command_hash,
                job_id: undo.operation_id,
                proposal_hash: undo.proposal_hash,
                base_frontier: ArtifactFrontier {
                    document_id: target.after_frontier.document_id,
                    head_edit_ordinal: target.after_frontier.head_edit_ordinal,
                    head_edit_id: target.after_frontier.head_edit_id,
                    last_commit_seq: target.after_frontier.last_commit_seq,
                    chain_hash,
                },
                descriptor_digest: target.descriptor_digest,
                base_digest: target.after_base_digest,
                timestamp,
                journal_now_ms: 0,
                document_write: owners.document_write.clone(),
                ingress: None,
                operation: GisMapCommitOperationV1::Undo { target_id: target.target_id, idempotency_key: undo.idempotency_key, original_job_id: target.original_job_id, original_command: Arc::new(target.original_command) },
            };
            Ok((identity, owners.generation, checkpoint.receipt().clone(), undo.ledger_applied))
        })();
        match recovered {
            Ok((identity, generation, receipt, ledger_applied)) => {
                let state = if ledger_applied {
                    RetainedGisMapDocumentStateV1::Published { owners, identity: identity.clone(), receipt: receipt.clone(), document_write: Some(document_write) }
                } else {
                    RetainedGisMapDocumentStateV1::Verification { owners, identity: identity.clone(), receipt: receipt.clone(), document_write }
                };
                documents.insert(key.to_owned(), state);
                Ok((identity, generation, receipt))
            }
            Err(error) => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write });
                Err(error)
            }
        }
    }
}

impl<B: ComposedMapCommitBindingV1> GisMapApprovalCommitterV1 for RetainedGisMapApprovalCommitterV1<B> {
    fn commit<'a>(&'a self, request: GisMapApprovalCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        if let Err(error) = self.validate_prepared_request(&request) {
            return Box::pin(async move { Err(error) });
        }
        let preflight = match Self::preflight(&request) {
            Ok(preflight) => preflight,
            Err(error) => return Box::pin(async move { Err(error) }),
        };
        if let Err(error) = self.reserve_cleanup_job(request.job_id) {
            return Box::pin(async move { Err(error) });
        }
        let key = document_key(request.scope);
        let request_token = Arc::new(GisMapApprovalRequestTokenV1);
        let owner = GisMapApprovalRequestOwnerV1 {
            committer: self.clone(),
            key,
            identity: None,
            prepared: Some(GisMapPreparedApprovalV1 {
                scope: request.scope.clone(),
                job_id: request.job_id.to_owned(),
                mutation_id: request.mutation_id.to_owned(),
                command_hash: request.command_hash.to_owned(),
                proposal_hash: request.proposal_hash.to_owned(),
                ingress: request.ingress.clone(),
            }),
            request: request_token,
        };
        Box::pin(self.commit_retained(request, owner, preflight))
    }

    fn undo<'a>(&'a self, request: GisMapApprovalUndoCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn Future<Output = Result<GisMapApprovalUndoCommitReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        let preflight = match Self::preflight_undo(&request) {
            Ok(preflight) => preflight,
            Err(error) => return Box::pin(async move { Err(error) }),
        };
        if let Err(error) = self.reserve_cleanup_job(request.operation_id) {
            return Box::pin(async move { Err(error) });
        }
        let owner = GisMapApprovalRequestOwnerV1 { committer: self.clone(), key: document_key(request.scope), identity: None, prepared: None, request: Arc::new(GisMapApprovalRequestTokenV1) };
        Box::pin(self.commit_undo_retained(request, owner, preflight))
    }

    fn close<'a>(&'a self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(RetainedGisMapApprovalCommitterV1::close(self))
    }

    fn document_clock<'a>(&'a self, scope: &'a DocumentScope) -> std::pin::Pin<Box<dyn Future<Output = Option<protocol::HybridLogicalTimestamp>> + Send + 'a>> {
        Box::pin(async move {
            let documents = self.documents.lock().await;
            match documents.get(&document_key(scope)) {
                Some(RetainedGisMapDocumentStateV1::Ready { owners, .. }) | Some(RetainedGisMapDocumentStateV1::Published { owners, .. }) => owners.parent.as_ref().map(directory::os_store::ArtifactStore::clock_now),
                _ => None,
            }
        })
    }
}

#[path = "../🧱️document-preparation/🦀️.rs"]
mod document_preparation;

#[path = "../🔁️commit-turns/🦀️.rs"]
mod commit_turns;

#[path = "../🔎️checkpoint-verify/🦀️.rs"]
mod checkpoint_verify;

#[path = "../📕️commit-and-close/🦀️.rs"]
mod commit_and_close;
