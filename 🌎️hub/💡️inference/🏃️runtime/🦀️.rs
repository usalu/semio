//! 🏃️ Owner-private GIS Map proposal runtime: frozen binding, ledger, per-document gate, typed approval.

use std::collections::HashMap;
use std::future::Future;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::{oneshot, OwnedSemaphorePermit, Semaphore};
use tokio::task::JoinHandle;

use directory::os_directory::{ArtifactFrontier, DocumentScope};

use super::catalog::VerifiedGisMapArtifactBindingV1;
use super::command::{encode_server_stamped_command_v1, CanonicalInferenceCommandPartsV1, CanonicalInferenceCommandV1};
use super::schema::{InferenceIdentityV1, GIS_DOCUMENT_SCHEMA, PROPOSAL_MAX_BYTES, RESULT_MAX_BYTES};
use super::sqlite::{InferenceJobLedgerV1, InferenceReaderV1};
use super::wal::{CommittedInferenceWalWitnessV1, InferenceDocumentFenceV1, InferenceWalTargetV1, InferenceWalVerifierV1};
use super::{sha256, InferenceErrorV1, InferenceOperationControlV1, InferencePrivateBytesV1};
use crate::directory::HubDirectory;

/// 🔢️ Fixed bounds every inference route enforces before it touches storage or the GIS executor.
pub const OPERATION_CAPACITY: usize = 32;
pub const DOCUMENT_GATE_CAPACITY: usize = 64;
pub const WORK_UNIT_LIMIT: u64 = 4096;
pub const ALLOCATION_BYTES: u64 = 1 << 20;
pub const RECURSION_DEPTH: u32 = 32;
pub const APPROVAL_MAX_RECORDS: u64 = 8;
pub const GIS_MAP_COMMITTER_CAPACITY: usize = 64;

/// 🚦️ The one stable failure vocabulary the four authenticated inference routes may publish.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceRouteErrorV1 {
    Unavailable,
    Denied,
    NotFound,
    Invalid,
    Bounds,
    Conflict,
    Capacity,
    Expired,
    Cancelled,
    CommitUnavailable,
    Storage,
}

impl InferenceRouteErrorV1 {
    /// 🏷️ Returns the exact wire code; a caller never learns which private object was missing.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "inference.unavailable",
            Self::Denied => "inference.denied",
            Self::NotFound => "inference.not-found",
            Self::Invalid => "inference.invalid",
            Self::Bounds => "inference.bounds",
            Self::Conflict => "inference.conflict",
            Self::Capacity => "inference.capacity",
            Self::Expired => "inference.expired",
            Self::Cancelled => "inference.cancelled",
            Self::CommitUnavailable => "approval.commit-unavailable",
            Self::Storage => "inference.storage",
        }
    }

    /// 🔢️ Returns the exact HTTP status this stable code publishes.
    pub const fn status(self) -> u16 {
        match self {
            Self::Unavailable | Self::CommitUnavailable | Self::Storage => 503,
            Self::Denied => 403,
            Self::NotFound => 404,
            Self::Invalid => 400,
            Self::Bounds => 413,
            Self::Conflict | Self::Cancelled => 409,
            Self::Capacity => 429,
            Self::Expired => 410,
        }
    }
}

impl From<InferenceErrorV1> for InferenceRouteErrorV1 {
    fn from(error: InferenceErrorV1) -> Self {
        match error {
            InferenceErrorV1::Invalid => Self::Invalid,
            InferenceErrorV1::Bounds => Self::Bounds,
            InferenceErrorV1::Denied => Self::Denied,
            InferenceErrorV1::Conflict => Self::Conflict,
            InferenceErrorV1::Capacity => Self::Capacity,
            InferenceErrorV1::Expired => Self::Expired,
            InferenceErrorV1::Cancelled => Self::Cancelled,
            InferenceErrorV1::Storage => Self::Storage,
        }
    }
}

/// 🧾️ Everything the atomic parent+existing-child composition transaction needs, all server-derived.
pub struct GisMapApprovalCommitRequestV1<'a> {
    /// 🕸️ The composed child members the frozen base Map already owns, in stable-member order.
    ///
    /// A `CreateRegion` is never parent-only when these are present: `create_region_group_work`
    /// pairs it with a `gismap-drawing` `CreateNode` and a `gismap-value` `insertListItem`, and the
    /// Map's own apply function does not keep them in sync. The retained committer accepts only the
    /// exact ordered drawing/value pair and refuses every parent-only or substituted membership.
    pub composed_children: &'a [String],
    pub scope: &'a DocumentScope,
    pub actor: &'a str,
    pub mutation_id: &'a str,
    pub command_hash: &'a str,
    pub job_id: &'a str,
    pub proposal_hash: &'a str,
    pub command: &'a [u8],
    pub base: &'a InferenceMapBaseV1,
    pub base_frontier: &'a ArtifactFrontier,
    pub deadline_ms: u64,
    pub now_ms: u64,
    pub document_write: Arc<tokio::sync::Mutex<()>>,
    pub ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
}

/// ↩️ Server-owned durable inverse request; the client contributes no mutation bytes.
pub struct GisMapApprovalUndoCommitRequestV1<'a> {
    pub composed_children: &'a [String],
    pub scope: &'a DocumentScope,
    pub actor: &'a str,
    pub target: &'a super::sqlite::GisMapApprovalUndoTargetV1,
    pub idempotency_key: &'a str,
    pub mutation_id: &'a str,
    pub command_hash: &'a str,
    pub operation_id: &'a str,
    pub proposal_hash: &'a str,
    pub command: &'a [u8],
    pub base: &'a InferenceMapBaseV1,
    pub deadline_ms: u64,
    pub now_ms: u64,
    pub document_write: Arc<tokio::sync::Mutex<()>>,
    pub ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
}

/// 🎫️ Hub-owned admission retained across the exact privileged approval operation.
pub trait GisMapApprovalIngressAuthorityV1: Send + Sync {
    fn scope(&self) -> &DocumentScope;
    fn user_id(&self) -> &str;
    fn session_id(&self) -> &str;
    fn authorization_generation(&self) -> u64;
}

/// 🔒 One exact Hub document-write exclusion held from actor recheck through durable verification.
pub struct GisMapDocumentWriteAuthorityV1 {
    gate: Arc<tokio::sync::Mutex<()>>,
    request: Arc<GisMapApprovalRequestTokenV1>,
    _guard: tokio::sync::OwnedMutexGuard<()>,
}

struct GisMapApprovalRequestTokenV1;

impl GisMapDocumentWriteAuthorityV1 {
    async fn acquire(gate: Arc<tokio::sync::Mutex<()>>, request: Arc<GisMapApprovalRequestTokenV1>) -> Self {
        let guard = gate.clone().lock_owned().await;
        Self { gate, request, _guard: guard }
    }
}

type GisMapDocumentWriteLeaseV1 = Arc<GisMapDocumentWriteAuthorityV1>;

/// ⛔️ Why a typed composition publication refused; never a partial or optimistic outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GisMapApprovalCommitErrorV1 {
    Unavailable,
    Rejected,
    Conflict,
    Capacity,
    Storage,
}

/// 🧾️ A committed publication receipt: only a real committed-WAL witness may reconcile the outbox.
pub struct GisMapApprovalReceiptV1 {
    pub witness: CommittedInferenceWalWitnessV1,
    pub document_generation: u64,
    pub applied: bool,
    pub undo: directory::os_directory::GisMapApprovalUndoHandleV1,
    pub frontier: directory::os_directory::CheckpointPublicationFrontierV1,
}

/// 🎁️ Public approval result stripped of the private WAL witness.
pub struct GisMapApprovalOutcomeV1 {
    pub applied: bool,
    pub undo: directory::os_directory::GisMapApprovalUndoHandleV1,
}

struct GisMapPreparedUndoCommandV1 {
    operation_id: String,
    proposal_hash: String,
    mutation_id: String,
    command_hash: String,
    command: InferencePrivateBytesV1,
}

/// ↩️ Internal durable inverse outcome with its second committed-WAL witness.
pub struct GisMapApprovalUndoCommitReceiptV1 {
    pub witness: CommittedInferenceWalWitnessV1,
    pub document_generation: u64,
    pub applied: bool,
    pub frontier: directory::os_directory::CheckpointPublicationFrontierV1,
}

/// 📸️ Exact post-decision actor/store state offered to the process-owned checkpoint publisher.
pub struct GisMapApprovalCheckpointRequestV1 {
    pub scope: DocumentScope,
    pub descriptor_digest: String,
    pub base_frontier: ArtifactFrontier,
    pub handle: db::ArtifactHandle,
    pub actor_snapshot: db::CheckpointPublicationSnapshot,
    pub pair: crate::artifact_authority::ArtifactPair,
    pub journal_receipt: directory::os_store::durable_group::DurableOwnedGroupJournalReceiptV1,
}

/// 📣️ Process boundary that must durably publish the exact post-decision pair before approval.
pub trait GisMapApprovalCheckpointPublisherV1: Send + Sync {
    fn publish<'a>(
        &'a self,
        request: GisMapApprovalCheckpointRequestV1,
        document_write: Arc<GisMapDocumentWriteAuthorityV1>,
        attempt_lifetime_ms: u64,
        decision_now_ms: u64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<directory::os_directory::PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1>> + Send + 'a>>;

    /// 📡️ Announces an already-published and ledger-applied checkpoint without yielding or releasing DocumentWrite.
    fn checkpoint_applied(&self, checkpoint: &directory::os_directory::PublishedArtifactCheckpoint) -> Result<(), GisMapApprovalCommitErrorV1>;
}

/// 🔌️ The private port an atomic parent+existing-child composition transaction implements.
///
/// Its contract is deliberately narrow: it receives one already-prepared server-stamped envelope and
/// must either refuse, or publish that exact envelope in a single visibility flip and return the
/// committed-WAL witness minted by [`super::wal::InferenceWalVerifierV1`]. Because the witness has
/// no public constructor, an implementation cannot fabricate durability, and the ledger outbox is
/// reconciled only against that real proof. It must never use `ArtifactHandle::submit` or the
/// generic `db.pathmap.v1` receiver, and it must never apply anything without explicit approval.
pub trait GisMapApprovalCommitterV1: Send + Sync {
    fn commit<'a>(&'a self, request: GisMapApprovalCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>>;
    fn undo<'a>(&'a self, request: GisMapApprovalUndoCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GisMapApprovalUndoCommitReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>>;
    fn close<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), GisMapApprovalCommitErrorV1>> + Send + 'a>>;
}

/// 🚧️ Fail-closed committer for every deployment where no composition transaction is registered.
///
/// A first single-document committer is deliberately NOT registered here. `bounds_proposal` produces
/// the parent `CreateRegion` alone, while `create_region_group_work` shows the semantically complete
/// approval is a fixed three-member group (parent + `gismap-drawing` `CreateNode` + `gismap-value`
/// `insertListItem`); the Map's own apply function does not keep the two children in sync. Publishing
/// the parent alone would therefore durably corrupt any Map that owns children, so
/// `commit_prepared_approval` refuses a child-bearing Map before a committer is ever consulted, and
/// the remaining zero-children case waits for the typed composition transaction rather than being
/// shortcut through the generic document receiver.
pub struct UnavailableGisMapApprovalCommitterV1;

impl GisMapApprovalCommitterV1 for UnavailableGisMapApprovalCommitterV1 {
    fn commit<'a>(&'a self, _request: GisMapApprovalCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async { Err(GisMapApprovalCommitErrorV1::Unavailable) })
    }

    fn undo<'a>(&'a self, _request: GisMapApprovalUndoCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GisMapApprovalUndoCommitReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async { Err(GisMapApprovalCommitErrorV1::Unavailable) })
    }

    fn close<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async { Ok(()) })
    }
}

type GisMapParentSnapshotV1 = semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot;
type GisMapParentMutationV1 = semio_s_plugin_gis::artifacts::gismap::mutations::GisMapMutation;
type GisMapDrawingSnapshotV1 = semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
type GisMapDrawingMutationV1 = semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
type GisMapValueSnapshotV1 = semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
type GisMapValueMutationV1 = semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::mutations::SemioValueMutation;
type GisMapParentStoreV1 = directory::os_store::ArtifactStore<GisMapParentSnapshotV1, GisMapParentMutationV1>;
type GisMapDrawingStoreV1 = directory::os_store::ArtifactStore<GisMapDrawingSnapshotV1, GisMapDrawingMutationV1>;
type GisMapValueStoreV1 = directory::os_store::ArtifactStore<GisMapValueSnapshotV1, GisMapValueMutationV1>;
type GisMapAssemblyV1 = directory::os_store::durable_group::DurableOwnedThreeStoreMapAssemblyV1<GisMapParentSnapshotV1, GisMapParentMutationV1, GisMapDrawingSnapshotV1, GisMapDrawingMutationV1, GisMapValueSnapshotV1, GisMapValueMutationV1>;
type GisMapCommitHostV1 = directory::os_store::durable_group::DurableOwnedMapCommitHostV1<GisMapParentSnapshotV1, GisMapParentMutationV1, GisMapDrawingSnapshotV1, GisMapDrawingMutationV1, GisMapValueSnapshotV1, GisMapValueMutationV1>;
type GisMapRecoveryOwnerV1 = db::document::ArtifactDurableGroupRecoveryOwnerV1<GisMapParentSnapshotV1, GisMapParentMutationV1, GisMapDrawingSnapshotV1, GisMapDrawingMutationV1, GisMapValueSnapshotV1, GisMapValueMutationV1>;
type GisMapJournalReceiptV1 = directory::os_store::durable_group::DurableOwnedGroupJournalReceiptV1;

struct GisMapDocumentStoresV1 {
    parent: Option<GisMapParentStoreV1>,
    drawing: Option<GisMapDrawingStoreV1>,
    value: Option<GisMapValueStoreV1>,
    handle: db::ArtifactHandle,
    scope: DocumentScope,
    generation: u64,
    document_write: Arc<tokio::sync::Mutex<()>>,
    fence: Arc<InferenceDocumentFenceV1>,
}

#[derive(Clone)]
struct GisMapCommitIdentityV1 {
    scope: DocumentScope,
    actor: String,
    mutation_id: String,
    command_hash: String,
    job_id: String,
    proposal_hash: String,
    base_frontier: ArtifactFrontier,
    descriptor_digest: String,
    base_digest: String,
    timestamp: protocol::HybridLogicalTimestamp,
    journal_now_ms: u64,
    document_write: Arc<tokio::sync::Mutex<()>>,
    ingress: Option<Arc<dyn GisMapApprovalIngressAuthorityV1>>,
    operation: GisMapCommitOperationV1,
}

#[derive(Clone)]
enum GisMapCommitOperationV1 {
    Approval,
    Undo { target_id: String, idempotency_key: String, original_job_id: String, original_command: Arc<InferencePrivateBytesV1> },
}

struct GisMapClosingStoresV1 {
    owners: GisMapDocumentStoresV1,
    phase: u8,
    document_write: Option<GisMapDocumentWriteLeaseV1>,
}

pub enum RetainedGisMapDocumentStateV1 {
    Recovery {
        owner: GisMapRecoveryOwnerV1,
        handle: db::ArtifactHandle,
        scope: DocumentScope,
        generation: u64,
        document_write: GisMapDocumentWriteLeaseV1,
        fence: Arc<InferenceDocumentFenceV1>,
    },
    Recovered {
        owners: GisMapDocumentStoresV1,
        checkpoint: db::document::ArtifactDurableGroupRecoveredCheckpointV1,
        document_write: GisMapDocumentWriteLeaseV1,
    },
    Ready {
        owners: GisMapDocumentStoresV1,
        pending: Option<GisMapCommitIdentityV1>,
        document_write: Option<GisMapDocumentWriteLeaseV1>,
    },
    Assembly {
        owner: GisMapAssemblyV1,
        handle: db::ArtifactHandle,
        scope: DocumentScope,
        generation: u64,
        identity: GisMapCommitIdentityV1,
        document_write: GisMapDocumentWriteLeaseV1,
        fence: Arc<InferenceDocumentFenceV1>,
    },
    Journal {
        owner: GisMapCommitHostV1,
        handle: db::ArtifactHandle,
        scope: DocumentScope,
        generation: u64,
        identity: GisMapCommitIdentityV1,
        receipt: Option<GisMapJournalReceiptV1>,
        document_write: GisMapDocumentWriteLeaseV1,
        fence: Arc<InferenceDocumentFenceV1>,
    },
    Verification {
        owners: GisMapDocumentStoresV1,
        identity: GisMapCommitIdentityV1,
        receipt: GisMapJournalReceiptV1,
        document_write: GisMapDocumentWriteLeaseV1,
    },
    Publishing {
        owners: GisMapDocumentStoresV1,
        identity: GisMapCommitIdentityV1,
        receipt: GisMapJournalReceiptV1,
        document_write: GisMapDocumentWriteLeaseV1,
    },
    Published {
        owners: GisMapDocumentStoresV1,
        identity: GisMapCommitIdentityV1,
        receipt: GisMapJournalReceiptV1,
        document_write: Option<GisMapDocumentWriteLeaseV1>,
    },
    Closing(GisMapClosingStoresV1),
}

enum GisMapCommitTurnV1 {
    Continue,
    Committed,
    Preflight { handle: db::ArtifactHandle, generation: u64 },
    Verify { generation: u64, receipt: GisMapJournalReceiptV1 },
    Rejected(GisMapApprovalCommitErrorV1),
}

enum GisMapAbandonedTurnV1 {
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
struct GisMapPreparedApprovalV1 {
    scope: DocumentScope,
    job_id: String,
    mutation_id: String,
    command_hash: String,
    proposal_hash: String,
    ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
}

#[derive(Clone)]
struct GisMapAbandonedRequestV1 {
    key: String,
    identity: GisMapCommitIdentityV1,
    request: Arc<GisMapApprovalRequestTokenV1>,
}

struct GisMapApprovalRequestOwnerV1 {
    committer: RetainedGisMapApprovalCommitterV1,
    key: String,
    identity: Option<GisMapCommitIdentityV1>,
    prepared: Option<GisMapPreparedApprovalV1>,
    request: Arc<GisMapApprovalRequestTokenV1>,
}

impl GisMapApprovalRequestOwnerV1 {
    fn admit(&mut self, identity: GisMapCommitIdentityV1) {
        self.prepared = None;
        self.identity = Some(identity);
    }

    fn complete(&mut self) {
        let job_id = self.identity.as_ref().map(|identity| identity.job_id.as_str()).or_else(|| self.prepared.as_ref().map(|prepared| prepared.job_id.as_str())).map(str::to_owned);
        self.prepared = None;
        self.identity = None;
        if let Some(job_id) = job_id {
            self.committer.release_cleanup_job(&job_id);
        }
    }
}

impl Drop for GisMapApprovalRequestOwnerV1 {
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
#[derive(Clone)]
pub struct RetainedGisMapApprovalCommitterV1 {
    database: Arc<db::Database>,
    verifier: Arc<InferenceWalVerifierV1>,
    ledger: Arc<InferenceJobLedgerV1>,
    publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1>,
    documents: Arc<tokio::sync::Mutex<HashMap<String, RetainedGisMapDocumentStateV1>>>,
    next_operation: Arc<AtomicU64>,
    maintenance: Arc<std::sync::Mutex<std::collections::HashSet<String>>>,
    cleanup_jobs: Arc<std::sync::Mutex<HashMap<String, u16>>>,
    closing: Arc<AtomicBool>,
    parked_prepared: Arc<std::sync::Mutex<HashMap<String, GisMapPreparedApprovalV1>>>,
    parked_requests: Arc<std::sync::Mutex<HashMap<String, GisMapAbandonedRequestV1>>>,
    state_epoch: Arc<tokio::sync::watch::Sender<u64>>,
}

struct GisMapAbandonedRequestTaskV1 {
    committer: RetainedGisMapApprovalCommitterV1,
    maintenance_key: String,
    owner: Option<GisMapAbandonedRequestV1>,
}

impl GisMapAbandonedRequestTaskV1 {
    fn complete(mut self) {
        let job_id = self.owner.as_ref().map(|owner| owner.identity.job_id.clone());
        self.owner = None;
        self.committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&self.maintenance_key);
        if let Some(job_id) = job_id {
            self.committer.release_cleanup_job(&job_id);
        }
        self.committer.announce_state_change();
    }
}

impl Drop for GisMapAbandonedRequestTaskV1 {
    fn drop(&mut self) {
        let Some(owner) = self.owner.take() else { return };
        self.committer.park_abandoned_request(owner);
        self.committer.announce_state_change();
    }
}

struct GisMapPreparedApprovalTaskV1 {
    committer: RetainedGisMapApprovalCommitterV1,
    maintenance_key: String,
    owner: Option<GisMapPreparedApprovalV1>,
}

impl GisMapPreparedApprovalTaskV1 {
    fn complete(mut self) {
        let job_id = self.owner.as_ref().map(|owner| owner.job_id.clone());
        self.owner = None;
        self.committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&self.maintenance_key);
        if let Some(job_id) = job_id {
            self.committer.release_cleanup_job(&job_id);
        }
        self.committer.announce_state_change();
    }
}

impl Drop for GisMapPreparedApprovalTaskV1 {
    fn drop(&mut self) {
        let Some(owner) = self.owner.take() else { return };
        self.committer.park_prepared_approval(owner);
        self.committer.announce_state_change();
    }
}

impl RetainedGisMapApprovalCommitterV1 {
    pub fn new(database: Arc<db::Database>, storage: Arc<db::storage::DbBackend>, ledger: Arc<InferenceJobLedgerV1>, publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1>) -> Self {
        Self {
            database,
            verifier: Arc::new(InferenceWalVerifierV1::new(storage)),
            ledger,
            publisher,
            documents: Arc::new(tokio::sync::Mutex::new(HashMap::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
            next_operation: Arc::new(AtomicU64::new(1)),
            maintenance: Arc::new(std::sync::Mutex::new(std::collections::HashSet::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
            cleanup_jobs: Arc::new(std::sync::Mutex::new(HashMap::with_capacity(super::schema::JOB_CAPACITY))),
            closing: Arc::new(AtomicBool::new(false)),
            parked_prepared: Arc::new(std::sync::Mutex::new(HashMap::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
            parked_requests: Arc::new(std::sync::Mutex::new(HashMap::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
            state_epoch: Arc::new(tokio::sync::watch::channel(0).0),
        }
    }

    fn announce_state_change(&self) {
        self.state_epoch.send_modify(|epoch| *epoch = epoch.wrapping_add(1));
    }

    fn reserve_cleanup_job(&self, job_id: &str) -> Result<(), GisMapApprovalCommitErrorV1> {
        let mut jobs = self.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.closing.load(Ordering::Acquire) {
            return Err(GisMapApprovalCommitErrorV1::Unavailable);
        }
        if let Some(owners) = jobs.get_mut(job_id) {
            let Some(next) = owners.checked_add(1) else { return Err(GisMapApprovalCommitErrorV1::Capacity) };
            *owners = next;
            return Ok(());
        }
        if jobs.len() >= super::schema::JOB_CAPACITY {
            return Err(GisMapApprovalCommitErrorV1::Capacity);
        }
        jobs.insert(job_id.to_owned(), 1);
        Ok(())
    }

    fn validate_prepared_request(&self, request: &GisMapApprovalCommitRequestV1<'_>) -> Result<(), GisMapApprovalCommitErrorV1> {
        let (outbox, identity, _) = self.ledger.approval_recovery_by_mutation(request.mutation_id).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?.ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
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

    fn release_cleanup_job(&self, job_id: &str) {
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

    fn abandoned_request_maintenance_key(owner: &GisMapAbandonedRequestV1) -> String {
        format!("request:{}:{}:{}", owner.key.len(), owner.key, owner.identity.job_id)
    }

    fn maintenance_owns_document(&self, key: &str) -> bool {
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

    fn resume_parked_requests(&self) {
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

    fn resume_parked_prepared(&self) {
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

    fn parent_store(id: &str, snapshot: GisMapParentSnapshotV1) -> GisMapParentStoreV1 {
        use directory::ArtifactPack;
        let mut envelope = directory::os_store::create_document_envelope::<GisMapParentSnapshotV1, GisMapParentMutationV1>(GIS_DOCUMENT_SCHEMA, id, snapshot.clone(), None);
        envelope.dialect = Some(directory::os_io::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() });
        let digest = *semio_framework_hash::hash(&snapshot.encode_pack()).as_bytes();
        let runtime = directory::os_store::ArtifactStoreInitializationRuntime::new(id, GIS_DOCUMENT_SCHEMA, snapshot, digest);
        directory::os_store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, semio_s_plugin_gis::artifacts::gismap::spr::gis_map_document_store_owners())
    }

    fn drawing_store(id: &str, parent: directory::os_io::ArtifactRef, snapshot: GisMapDrawingSnapshotV1) -> GisMapDrawingStoreV1 {
        use directory::{os_store::MemberStoreOwner, ArtifactPack};
        let schema = semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA;
        let mut envelope = directory::os_store::create_document_envelope::<GisMapDrawingSnapshotV1, GisMapDrawingMutationV1>(schema, id, snapshot.clone(), None);
        envelope.dialect = Some(directory::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() });
        envelope.owner = Some(directory::os_store::OwnerRef { parent, slot: "drawing".into(), child_id: id.into() });
        let digest = *semio_framework_hash::hash(&snapshot.encode_pack()).as_bytes();
        let runtime = directory::os_store::ArtifactStoreInitializationRuntime::new(id, schema, snapshot, digest);
        directory::os_store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, <GisMapDrawingSnapshotV1 as MemberStoreOwner<GisMapDrawingMutationV1>>::member_store_owners())
    }

    fn value_store(id: &str, parent: directory::os_io::ArtifactRef, snapshot: GisMapValueSnapshotV1) -> GisMapValueStoreV1 {
        use directory::{os_store::MemberStoreOwner, ArtifactPack};
        let schema = semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
        let mut envelope = directory::os_store::create_document_envelope::<GisMapValueSnapshotV1, GisMapValueMutationV1>(schema, id, snapshot.clone(), None);
        envelope.dialect = Some(directory::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "value".into() });
        envelope.owner = Some(directory::os_store::OwnerRef { parent, slot: "value".into(), child_id: id.into() });
        let digest = *semio_framework_hash::hash(&snapshot.encode_pack()).as_bytes();
        let runtime = directory::os_store::ArtifactStoreInitializationRuntime::new(id, schema, snapshot, digest);
        directory::os_store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, <GisMapValueSnapshotV1 as MemberStoreOwner<GisMapValueMutationV1>>::member_store_owners())
    }

    fn derived_children(snapshot: &GisMapParentSnapshotV1) -> (GisMapDrawingSnapshotV1, GisMapValueSnapshotV1) {
        use semio_s_plugin_gis::artifacts::gismap::schema::{gis_map_descriptor_json, gis_map_snapshot_to_drawing};
        (gis_map_snapshot_to_drawing(snapshot), semio_s_plugin_gis::artifacts::gismap::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(snapshot)))
    }

    fn stores_match(owners: &GisMapDocumentStoresV1, snapshot: &GisMapParentSnapshotV1) -> bool {
        let (drawing, value) = Self::derived_children(snapshot);
        owners.parent.as_ref().is_some_and(|store| store.snapshot_ref() == snapshot) && owners.drawing.as_ref().is_some_and(|store| store.snapshot_ref() == &drawing) && owners.value.as_ref().is_some_and(|store| store.snapshot_ref() == &value)
    }

    fn checkpoint_matches_frontier(snapshot: &db::CheckpointPublicationSnapshot, generation: u64, frontier: &ArtifactFrontier, scope: &DocumentScope) -> bool {
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

    fn preflight(request: &GisMapApprovalCommitRequestV1<'_>) -> Result<(GisMapCommitIdentityV1, GisMapParentSnapshotV1), GisMapApprovalCommitErrorV1> {
        let (snapshot, inference) = deterministic_map_inference(request.base, request.job_id).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let work = inference.create_region_group_work(&snapshot, request.job_id).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        if request.composed_children.len() != 2 || request.composed_children[0] != work.drawing_child.child_id || request.composed_children[1] != work.value_child.child_id {
            return Err(GisMapApprovalCommitErrorV1::Rejected);
        }
        let proposal = directory::os_pack::json::to_json_string(&work.parent).into_bytes();
        let inverse = directory::os_pack::json::to_json_string(&work.parent_inverse).into_bytes();
        let command = CanonicalInferenceCommandV1::decode(request.command).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let actor = request.actor.strip_prefix("user:").and_then(|value| value.split_once("#session:"));
        let admitted_actor = actor.is_some_and(|(user, session)| request.ingress.scope() == request.scope && request.ingress.user_id() == user && request.ingress.session_id() == session && request.ingress.authorization_generation() != 0);
        if request.proposal_hash != sha256(&proposal)
            || request.mutation_id != approval_mutation_id(request.job_id, request.proposal_hash)
            || request.command_hash != sha256(request.command)
            || request.base_frontier != &request.base.frontier
            || request.deadline_ms <= request.now_ms
            || request.deadline_ms - request.now_ms > super::schema::JOB_MAX_LIFETIME_MS
            || !actor.is_some_and(|(user, session)| super::schema::server_id(user) && super::schema::server_id(session))
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

    fn preflight_undo(request: &GisMapApprovalUndoCommitRequestV1<'_>) -> Result<(GisMapCommitIdentityV1, GisMapParentSnapshotV1), GisMapApprovalCommitErrorV1> {
        use directory::Inference as _;
        use semio_s_plugin_gis::artifacts::gismap::mutations::{apply_gis_map_mutation, GisMapMutation};
        use semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;
        let snapshot = <GisMapParentSnapshotV1 as directory::ArtifactPack>::decode_pack(request.base.pack.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let actor = request.actor.strip_prefix("user:").and_then(|value| value.split_once("#session:"));
        let target = request.target;
        let expected_frontier = Self::publication_wire_frontier(&request.base.frontier);
        let command = CanonicalInferenceCommandV1::decode(request.command).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let original = CanonicalInferenceCommandV1::decode(target.original_command.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let inverse_text = std::str::from_utf8(original.inverse_payload()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let inverses = directory::os_pack::json::from_json_str::<Vec<GisMapMutation>>(inverse_text).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        if inverses.len() != 1 {
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        let mut before = snapshot.clone();
        apply_gis_map_mutation(&mut before, &inverses[0]).map_err(|_| GisMapApprovalCommitErrorV1::Conflict)?;
        let work = GisMapInference::infer(&before).create_region_group_work(&before, &target.original_job_id).map_err(|_| GisMapApprovalCommitErrorV1::Conflict)?;
        let undo_forward = directory::os_pack::json::to_json_string(&inverses[0]).into_bytes();
        let undo_inverse = directory::os_pack::json::to_json_string(&vec![work.parent]).into_bytes();
        if target.scope != *request.scope
            || target.user_id != request.ingress.user_id()
            || target.session_id != request.ingress.session_id()
            || target.authorization_generation != request.ingress.authorization_generation()
            || request.ingress.scope() != request.scope
            || target.after_frontier != expected_frontier
            || request.command_hash != sha256(request.command)
            || request.proposal_hash != sha256(&undo_forward)
            || request.mutation_id != approval_mutation_id(request.operation_id, request.proposal_hash)
            || request.composed_children != ["gismap-drawing", "gismap-value"]
            || request.deadline_ms <= request.now_ms
            || request.deadline_ms - request.now_ms > super::schema::JOB_MAX_LIFETIME_MS
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
                    original_command: Arc::new(InferencePrivateBytesV1::new(target.original_command.as_slice().to_vec(), super::command::COMMAND_MAX_BYTES).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?),
                },
            },
            snapshot,
        ))
    }

    fn identity_matches(identity: &GisMapCommitIdentityV1, candidate: &GisMapCommitIdentityV1) -> bool {
        identity.scope == candidate.scope
            && identity.actor == candidate.actor
            && identity.mutation_id == candidate.mutation_id
            && identity.command_hash == candidate.command_hash
            && identity.job_id == candidate.job_id
            && identity.proposal_hash == candidate.proposal_hash
            && identity.base_frontier == candidate.base_frontier
            && identity.descriptor_digest == candidate.descriptor_digest
            && identity.base_digest == candidate.base_digest
            && Arc::ptr_eq(&identity.document_write, &candidate.document_write)
            && match (&identity.operation, &candidate.operation) {
                (GisMapCommitOperationV1::Approval, GisMapCommitOperationV1::Approval) => true,
                (
                    GisMapCommitOperationV1::Undo { target_id: left_target, idempotency_key: left_key, original_job_id: left_job, original_command: left_command },
                    GisMapCommitOperationV1::Undo { target_id: right_target, idempotency_key: right_key, original_job_id: right_job, original_command: right_command },
                ) => left_target == right_target && left_key == right_key && left_job == right_job && left_command.as_slice() == right_command.as_slice(),
                _ => false,
            }
            && match (&identity.ingress, &candidate.ingress) {
                (Some(left), Some(right)) => Arc::ptr_eq(left, right),
                (None, Some(_)) => true,
                (None, None) => true,
                _ => false,
            }
    }

    fn reserve_operations(&self) -> Result<[semio_framework_job::OperationId; 3], GisMapApprovalCommitErrorV1> {
        let start = self.next_operation.fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| value.checked_add(3)).map_err(|_| GisMapApprovalCommitErrorV1::Capacity)?;
        Ok([semio_framework_job::OperationId(start), semio_framework_job::OperationId(start + 1), semio_framework_job::OperationId(start + 2)])
    }

    /// 📍 Installs one exact document actor and an actor-owned committed-Event recovery before any
    /// approval can enter.
    async fn mount_document(&self, scope: DocumentScope, handle: db::ArtifactHandle, base: &InferenceMapBaseV1, document_write: GisMapDocumentWriteLeaseV1) -> Result<bool, GisMapApprovalCommitErrorV1> {
        let snapshot = <GisMapParentSnapshotV1 as directory::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
        let observed = handle.checkpoint_publication_snapshot().await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
        if !Self::checkpoint_matches(&observed, observed.authority_generation, base, &scope) {
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        let key = document_key(&scope);
        let mut documents = self.documents.lock().await;
        if let Some(state) = documents.get(&key) {
            let matches = match state {
                RetainedGisMapDocumentStateV1::Ready { owners, .. } | RetainedGisMapDocumentStateV1::Published { owners, .. } => {
                    owners.scope == scope && owners.generation == observed.authority_generation && Arc::ptr_eq(&owners.document_write, &document_write.gate) && Self::stores_match(owners, &snapshot)
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
        let (drawing, value) = Self::derived_children(&snapshot);
        let parent_id = key.clone();
        let drawing_id = snapshot.drawing.child_id.clone();
        let value_id = snapshot.value.child_id.clone();
        let parent = Self::parent_store(&parent_id, snapshot);
        let parent_reference = directory::os_io::ArtifactRef { artifact_id: parent_id, dialect: directory::os_io::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() } };
        let admission = directory::os_store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(parent, Self::drawing_store(&drawing_id, parent_reference.clone(), drawing), Self::value_store(&value_id, parent_reference, value));
        let recovery = handle.durable_group_recovery_retained(admission);
        documents.insert(key, RetainedGisMapDocumentStateV1::Recovery { owner: recovery, handle, scope, generation: observed.authority_generation, document_write, fence });
        Ok(true)
    }

    async fn finish_document_recovery(&self, key: &str) -> Result<(), GisMapApprovalCommitErrorV1> {
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        let RetainedGisMapDocumentStateV1::Recovery { mut owner, handle, scope, generation, document_write, fence } = state else {
            documents.insert(key.to_owned(), state);
            return Ok(());
        };
        if !owner.scan_is_active() {
            if let Err(_) = handle.resume_durable_group_recovery(&mut owner) {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                return Err(GisMapApprovalCommitErrorV1::Storage);
            }
        }
        match owner.advance().await {
            Ok(true) => {
                let recovered_checkpoint = owner.recovered_checkpoint();
                let Some(owners) = owner.take_terminal_owners() else {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                    return Err(GisMapApprovalCommitErrorV1::Storage);
                };
                drop(owner);
                let owners = Self::restored_stores(owners.parent, owners.drawing, owners.value, handle, scope, generation, document_write.gate.clone(), fence);
                if let Some(checkpoint) = recovered_checkpoint {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write });
                } else {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: Some(document_write) });
                }
                Ok(())
            }
            Ok(false) | Err(_) => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                Err(GisMapApprovalCommitErrorV1::Storage)
            }
        }
    }

    async fn resume_recovered_checkpoint(&self, key: &str) -> Result<(GisMapCommitIdentityV1, u64, GisMapJournalReceiptV1), GisMapApprovalCommitErrorV1> {
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        let RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } = state else {
            documents.insert(key.to_owned(), state);
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        };
        let recovered = (|| {
            let mutation_id = checkpoint.head_edit_id().0.as_str();
            if let Some((outbox, accepted, ledger_applied)) = self.ledger.approval_recovery_by_mutation(mutation_id).map_err(|_| GisMapApprovalCommitErrorV1::Storage)? {
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
            let undo = self.ledger.gis_map_approval_undo_recovery_by_mutation(mutation_id).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?.ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
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

    async fn prepare_retained_document(&self, scope: &DocumentScope, base: &InferenceMapBaseV1, gate: Arc<tokio::sync::Mutex<()>>, request: Arc<GisMapApprovalRequestTokenV1>) -> Result<(), GisMapApprovalCommitErrorV1> {
        let key = document_key(scope);
        let mut state_changes = self.state_epoch.subscribe();
        loop {
            enum PrepareTurn {
                Recover,
                PublishRecovered,
                Acquire,
                Ready,
                Wait,
            }
            let turn = {
                let documents = self.documents.lock().await;
                match documents.get(&key) {
                    Some(RetainedGisMapDocumentStateV1::Recovery { scope: stored, document_write, .. }) => {
                        if stored != scope || !Arc::ptr_eq(&document_write.gate, &gate) {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if Self::request_matches(document_write, &request) {
                            PrepareTurn::Recover
                        } else {
                            PrepareTurn::Wait
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Ready { owners, document_write, .. }) => {
                        if owners.scope != *scope
                            || !Arc::ptr_eq(&owners.document_write, &gate)
                            || !Self::stores_match(owners, &<GisMapParentSnapshotV1 as directory::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?)
                        {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        match document_write {
                            Some(document_write) if Self::request_matches(document_write, &request) => PrepareTurn::Ready,
                            Some(_) => PrepareTurn::Wait,
                            None => PrepareTurn::Acquire,
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Published { owners, document_write, .. }) => {
                        if owners.scope != *scope
                            || !Arc::ptr_eq(&owners.document_write, &gate)
                            || !Self::stores_match(owners, &<GisMapParentSnapshotV1 as directory::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?)
                        {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        match document_write {
                            Some(document_write) if Self::request_matches(document_write, &request) => PrepareTurn::Ready,
                            Some(_) => PrepareTurn::Wait,
                            None => PrepareTurn::Acquire,
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Recovered { owners, document_write, .. }) => {
                        if owners.scope != *scope || !Arc::ptr_eq(&owners.document_write, &gate) || !Arc::ptr_eq(&document_write.gate, &gate) {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if Self::request_matches(document_write, &request) {
                            PrepareTurn::PublishRecovered
                        } else {
                            PrepareTurn::Wait
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Verification { owners, identity, document_write, .. }) | Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity, document_write, .. }) => {
                        if owners.scope != *scope || !Arc::ptr_eq(&owners.document_write, &gate) || !Arc::ptr_eq(&document_write.gate, &gate) || identity.base_frontier != base.frontier || identity.base_digest != base.digest() {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if Self::request_matches(document_write, &request) {
                            PrepareTurn::Ready
                        } else {
                            PrepareTurn::Wait
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Assembly { scope: stored, identity, document_write, .. }) | Some(RetainedGisMapDocumentStateV1::Journal { scope: stored, identity, document_write, .. }) => {
                        if stored != scope || !Arc::ptr_eq(&identity.document_write, &gate) || identity.base_frontier != base.frontier || identity.base_digest != base.digest() {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if Self::request_matches(document_write, &request) {
                            PrepareTurn::Ready
                        } else {
                            PrepareTurn::Wait
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Closing(_)) => PrepareTurn::Wait,
                    None => PrepareTurn::Acquire,
                }
            };
            match turn {
                PrepareTurn::Ready => return Ok(()),
                PrepareTurn::Wait => {
                    state_changes.changed().await.map_err(|_| GisMapApprovalCommitErrorV1::Unavailable)?;
                    continue;
                }
                PrepareTurn::Recover => {
                    self.finish_document_recovery(&key).await?;
                    self.announce_state_change();
                    continue;
                }
                PrepareTurn::PublishRecovered => {
                    self.resume_recovered_checkpoint(&key).await?;
                    self.announce_state_change();
                    return Ok(());
                }
                PrepareTurn::Acquire => {}
            }
            let document_write = Arc::new(GisMapDocumentWriteAuthorityV1::acquire(gate.clone(), request.clone()).await);
            let existing = {
                let mut documents = self.documents.lock().await;
                match documents.remove(&key) {
                    Some(RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write: None }) => {
                        documents.insert(key.clone(), RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write: Some(document_write) });
                        true
                    }
                    Some(RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write: None }) => {
                        documents.insert(key.clone(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write: Some(document_write) });
                        true
                    }
                    Some(state) => {
                        documents.insert(key.clone(), state);
                        drop(document_write);
                        false
                    }
                    None => {
                        drop(documents);
                        let handle = self.database.document(&protocol::ArtifactId(key.clone())).await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
                        self.mount_document(scope.clone(), handle, base, document_write).await?;
                        self.finish_document_recovery(&key).await?;
                        return Ok(());
                    }
                }
            };
            if existing {
                return Ok(());
            }
        }
    }

    fn build_assembly(
        &self,
        mut owners: GisMapDocumentStoresV1,
        identity: GisMapCommitIdentityV1,
        snapshot: &GisMapParentSnapshotV1,
        document_write: GisMapDocumentWriteLeaseV1,
    ) -> Result<RetainedGisMapDocumentStateV1, (GisMapApprovalCommitErrorV1, GisMapDocumentStoresV1)> {
        use directory::os_store::durable_group::{DurableOwnedMapMemberAdmissionV1, DurableOwnedThreeStoreMapAssemblyV1};
        use directory::Inference as _;
        use semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;
        use semio_s_plugin_gis::editor::gis2d::{gis_map_drawing_stamped_one_item_preparation_factory, gis_map_parent_stamped_one_item_preparation_factory, gis_map_value_stamped_one_item_preparation_factory, GisMapOneItemStampV1};
        let work = match &identity.operation {
            GisMapCommitOperationV1::Approval => GisMapInference::infer(snapshot).create_region_group_work(snapshot, &identity.job_id),
            GisMapCommitOperationV1::Undo { original_job_id, original_command, .. } => {
                use semio_s_plugin_gis::artifacts::gismap::mutations::{apply_gis_map_mutation, GisMapMutation};
                let command = match CanonicalInferenceCommandV1::decode(original_command.as_slice()) {
                    Ok(command) => command,
                    Err(_) => return Err((GisMapApprovalCommitErrorV1::Rejected, owners)),
                };
                let inverse_text = match std::str::from_utf8(command.inverse_payload()) {
                    Ok(value) => value,
                    Err(_) => return Err((GisMapApprovalCommitErrorV1::Rejected, owners)),
                };
                let inverses = match directory::os_pack::json::from_json_str::<Vec<GisMapMutation>>(inverse_text) {
                    Ok(value) if value.len() == 1 => value,
                    _ => return Err((GisMapApprovalCommitErrorV1::Rejected, owners)),
                };
                let mut before = snapshot.clone();
                if apply_gis_map_mutation(&mut before, &inverses[0]).is_err() {
                    return Err((GisMapApprovalCommitErrorV1::Conflict, owners));
                }
                let work = match GisMapInference::infer(&before).create_region_group_work(&before, original_job_id) {
                    Ok(work) => work,
                    Err(_) => return Err((GisMapApprovalCommitErrorV1::Conflict, owners)),
                };
                let expected_inverse = directory::os_pack::json::to_json_string(&work.parent_inverse).into_bytes();
                let expected_forward = directory::os_pack::json::to_json_string(&work.parent).into_bytes();
                if !command.matches_fixed_three_parent(&approval_mutation_id(original_job_id, &sha256(&expected_forward)), &document_key(&identity.scope), &identity.actor, &expected_forward, &expected_inverse) {
                    return Err((GisMapApprovalCommitErrorV1::Conflict, owners));
                }
                Ok(work)
            }
        };
        let work = match work {
            Ok(work) => work,
            Err(_) => return Err((GisMapApprovalCommitErrorV1::Rejected, owners)),
        };
        let operations = match self.reserve_operations() {
            Ok(operations) => operations,
            Err(error) => return Err((error, owners)),
        };
        let parent = owners.parent.take().expect("mounted Map owner retains parent Store");
        let drawing = owners.drawing.take().expect("mounted Map owner retains drawing Store");
        let value = owners.value.take().expect("mounted Map owner retains value Store");
        let (parent_mutation, drawing_mutation, value_mutation) = match &identity.operation {
            GisMapCommitOperationV1::Approval => (work.parent, work.drawing, work.value),
            GisMapCommitOperationV1::Undo { .. } => {
                if work.parent_inverse.len() != 1 || work.drawing_inverse.len() != 1 || work.value_inverse.len() != 1 {
                    return Err((GisMapApprovalCommitErrorV1::Rejected, owners));
                }
                (work.parent_inverse.into_iter().next().expect("one parent inverse"), work.drawing_inverse.into_iter().next().expect("one drawing inverse"), work.value_inverse.into_iter().next().expect("one value inverse"))
            }
        };
        let parent_admission = DurableOwnedMapMemberAdmissionV1::new(operations[0], parent.generation_now(), parent.content_revision_now(), identity.actor.clone(), parent_mutation, Some(format!("inference:{}", identity.job_id)));
        let drawing_admission = DurableOwnedMapMemberAdmissionV1::new(operations[1], drawing.generation_now(), drawing.content_revision_now(), identity.actor.clone(), drawing_mutation, Some(format!("inference:{}:drawing", identity.job_id)));
        let value_admission = DurableOwnedMapMemberAdmissionV1::new(operations[2], value.generation_now(), value.content_revision_now(), identity.actor.clone(), value_mutation, Some(format!("inference:{}:value", identity.job_id)));
        let stamp = |mutation_id: String| GisMapOneItemStampV1 { mutation_id: protocol::MutationId(mutation_id), timestamp: identity.timestamp };
        let sink = owners.handle.durable_group_journal_sink(identity.journal_now_ms);
        let handle = owners.handle;
        let scope = owners.scope;
        let generation = owners.generation;
        let fence = owners.fence;
        Ok(RetainedGisMapDocumentStateV1::Assembly {
            owner: DurableOwnedThreeStoreMapAssemblyV1::new(
                parent,
                drawing,
                value,
                parent_admission,
                drawing_admission,
                value_admission,
                gis_map_parent_stamped_one_item_preparation_factory(stamp(identity.mutation_id.clone())),
                gis_map_drawing_stamped_one_item_preparation_factory(stamp(format!("{}:drawing", identity.mutation_id))),
                gis_map_value_stamped_one_item_preparation_factory(stamp(format!("{}:value", identity.mutation_id))),
                sink,
            ),
            handle,
            scope,
            generation,
            identity,
            document_write,
            fence,
        })
    }

    fn restored_stores(
        parent: GisMapParentStoreV1,
        drawing: GisMapDrawingStoreV1,
        value: GisMapValueStoreV1,
        handle: db::ArtifactHandle,
        scope: DocumentScope,
        generation: u64,
        document_write: Arc<tokio::sync::Mutex<()>>,
        fence: Arc<InferenceDocumentFenceV1>,
    ) -> GisMapDocumentStoresV1 {
        GisMapDocumentStoresV1 { parent: Some(parent), drawing: Some(drawing), value: Some(value), handle, scope, generation, document_write, fence }
    }

    async fn drive_turn(&self, key: &str, candidate: &GisMapCommitIdentityV1, snapshot: &GisMapParentSnapshotV1) -> GisMapCommitTurnV1 {
        use directory::os_store::durable_group::{DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1};
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Unavailable) };
        match state {
            RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Unavailable)
            }
            RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } => {
                let generation = owners.generation;
                let receipt = checkpoint.receipt().clone();
                let mut identity = candidate.clone();
                identity.ingress = None;
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write });
                GisMapCommitTurnV1::Verify { generation, receipt }
            }
            RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write } => {
                if !Self::stores_match(&owners, snapshot) || pending.as_ref().is_some_and(|identity| !Self::identity_matches(identity, candidate)) {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write });
                    return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict);
                }
                let handle = owners.handle.clone();
                let generation = owners.generation;
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(candidate.clone()), document_write });
                GisMapCommitTurnV1::Preflight { handle, generation }
            }
            RetainedGisMapDocumentStateV1::Assembly { mut owner, handle, scope, generation, identity, document_write, fence } => {
                if !Self::identity_matches(&identity, candidate) {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                    return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict);
                }
                let result = owner.advance(directory::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES });
                match result {
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted) => {
                        let host = owner.take_mounted_host().expect("mounted assembly returns its exact host");
                        drop(owner);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner: host, handle, scope, generation, identity, receipt: None, document_write, fence });
                        GisMapCommitTurnV1::Continue
                    }
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal) => {
                        let terminal = owner.take_terminal_owners().expect("terminal assembly returns every owner");
                        #[cfg(test)]
                        eprintln!("[DEBUG] GIS fixed-three assembly terminal failure={:?}", terminal.failure);
                        drop(owner);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                        GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Rejected)
                    }
                    Ok(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        GisMapCommitTurnV1::Continue
                    }
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Storage)
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Journal { mut owner, handle, scope, generation, mut identity, mut receipt, document_write, fence } => {
                if !Self::identity_matches(&identity, candidate) {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                    return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict);
                }
                match owner.advance(directory::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(exact_receipt)) => {
                        if receipt.as_ref().is_some_and(|stored| stored != &exact_receipt) {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Storage);
                        }
                        receipt = Some(exact_receipt);
                        identity.ingress = None;
                        if !owner.acknowledge() {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Storage);
                        }
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapCommitTurnV1::Committed
                    }
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => {
                        let Some(receipt) = receipt else {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt: None, document_write, fence });
                            return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Storage);
                        };
                        let terminal = owner.take_terminal_owners().expect("acknowledged journal host returns exact Stores and sink");
                        drop(owner);
                        drop(terminal.sink);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write });
                        GisMapCommitTurnV1::Verify { generation, receipt }
                    }
                    Ok(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapCommitTurnV1::Continue
                    }
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Storage)
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write } => {
                let matches = Self::identity_matches(&identity, candidate);
                let generation = owners.generation;
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write });
                if matches {
                    GisMapCommitTurnV1::Verify { generation, receipt }
                } else {
                    GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict)
                }
            }
            RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write } => {
                let matches = Self::identity_matches(&identity, candidate);
                let generation = owners.generation;
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt: receipt.clone(), document_write });
                if matches {
                    GisMapCommitTurnV1::Verify { generation, receipt }
                } else {
                    GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict)
                }
            }
            RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write } => {
                if Self::identity_matches(&identity, candidate) {
                    let generation = owners.generation;
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt: receipt.clone(), document_write });
                    GisMapCommitTurnV1::Verify { generation, receipt }
                } else if Self::stores_match(&owners, snapshot) && document_write.is_some() {
                    let handle = owners.handle.clone();
                    let generation = owners.generation;
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(candidate.clone()), document_write });
                    GisMapCommitTurnV1::Preflight { handle, generation }
                } else {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write });
                    GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict)
                }
            }
            closing @ RetainedGisMapDocumentStateV1::Closing(_) => {
                documents.insert(key.to_owned(), closing);
                GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Unavailable)
            }
        }
    }

    fn request_matches(document_write: &GisMapDocumentWriteLeaseV1, request: &Arc<GisMapApprovalRequestTokenV1>) -> bool {
        Arc::ptr_eq(&document_write.request, request)
    }

    fn state_has_active_request(state: &RetainedGisMapDocumentStateV1) -> bool {
        let document_write = match state {
            RetainedGisMapDocumentStateV1::Recovery { document_write, .. }
            | RetainedGisMapDocumentStateV1::Recovered { document_write, .. }
            | RetainedGisMapDocumentStateV1::Assembly { document_write, .. }
            | RetainedGisMapDocumentStateV1::Journal { document_write, .. }
            | RetainedGisMapDocumentStateV1::Verification { document_write, .. }
            | RetainedGisMapDocumentStateV1::Publishing { document_write, .. } => Some(document_write),
            RetainedGisMapDocumentStateV1::Ready { document_write, .. } | RetainedGisMapDocumentStateV1::Published { document_write, .. } => document_write.as_ref(),
            RetainedGisMapDocumentStateV1::Closing(closing) => closing.document_write.as_ref(),
        };
        document_write.is_some_and(|document_write| Arc::strong_count(&document_write.request) > 1)
    }

    async fn drive_abandoned_turn(&self, key: &str, candidate: &GisMapCommitIdentityV1, request: &Arc<GisMapApprovalRequestTokenV1>) -> GisMapAbandonedTurnV1 {
        use directory::os_store::durable_group::{DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1};
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return GisMapAbandonedTurnV1::Complete };
        match state {
            RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence } => {
                let matches = Self::request_matches(&document_write, request);
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                if matches {
                    GisMapAbandonedTurnV1::Recover
                } else {
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } => {
                let matches = Self::request_matches(&document_write, request);
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write });
                if matches {
                    GisMapAbandonedTurnV1::Recovered
                } else {
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write } => {
                let matches = document_write.as_ref().is_some_and(|lease| Self::request_matches(lease, request)) && pending.as_ref().is_none_or(|identity| Self::identity_matches(identity, candidate));
                if matches {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                    drop(document_write);
                } else {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write });
                }
                if matches {
                    GisMapAbandonedTurnV1::Aborted
                } else {
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Assembly { mut owner, handle, scope, generation, identity, document_write, fence } => {
                if !Self::request_matches(&document_write, request) || !Self::identity_matches(&identity, candidate) {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                    return GisMapAbandonedTurnV1::Complete;
                }
                owner.cancel();
                match owner.advance(directory::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted) => {
                        let host = owner.take_mounted_host().expect("mounted abandoned assembly returns its exact host");
                        drop(owner);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner: host, handle, scope, generation, identity, receipt: None, document_write, fence });
                        GisMapAbandonedTurnV1::Continue
                    }
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal) => {
                        let terminal = owner.take_terminal_owners().expect("abandoned assembly returns every owner");
                        drop(owner);
                        drop(terminal.sink);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                        drop(document_write);
                        GisMapAbandonedTurnV1::Aborted
                    }
                    Ok(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        GisMapAbandonedTurnV1::Continue
                    }
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        GisMapAbandonedTurnV1::Retry
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Journal { mut owner, handle, scope, generation, mut identity, mut receipt, document_write, fence } => {
                if !Self::request_matches(&document_write, request) || !Self::identity_matches(&identity, candidate) {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                    return GisMapAbandonedTurnV1::Complete;
                }
                if receipt.is_none() {
                    owner.cancel();
                }
                match owner.advance(directory::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(exact_receipt)) => {
                        if receipt.as_ref().is_some_and(|stored| stored != &exact_receipt) {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return GisMapAbandonedTurnV1::Retry;
                        }
                        receipt = Some(exact_receipt);
                        identity.ingress = None;
                        if !owner.acknowledge() {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return GisMapAbandonedTurnV1::Retry;
                        }
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapAbandonedTurnV1::Committed
                    }
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => {
                        let terminal = owner.take_terminal_owners().expect("abandoned journal returns every owner");
                        drop(owner);
                        drop(terminal.sink);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        if let Some(receipt) = receipt {
                            let verify_identity = identity.clone();
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write });
                            GisMapAbandonedTurnV1::Verify { identity: verify_identity, generation, receipt }
                        } else {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                            drop(document_write);
                            GisMapAbandonedTurnV1::Aborted
                        }
                    }
                    Ok(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapAbandonedTurnV1::Continue
                    }
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapAbandonedTurnV1::Retry
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write } => {
                let matches = Self::request_matches(&document_write, request) && Self::identity_matches(&identity, candidate);
                let generation = owners.generation;
                let verify_identity = identity.clone();
                let verify_receipt = receipt.clone();
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write });
                if matches {
                    GisMapAbandonedTurnV1::Verify { identity: verify_identity, generation, receipt: verify_receipt }
                } else {
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write } => {
                let matches = Self::request_matches(&document_write, request) && Self::identity_matches(&identity, candidate);
                let generation = owners.generation;
                let verify_identity = identity.clone();
                let verify_receipt = receipt.clone();
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write });
                if matches {
                    GisMapAbandonedTurnV1::Verify { identity: verify_identity, generation, receipt: verify_receipt }
                } else {
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write } => {
                let matches = document_write.as_ref().is_some_and(|lease| Self::request_matches(lease, request)) && Self::identity_matches(&identity, candidate);
                if matches {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write: None });
                    drop(document_write);
                } else {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write });
                }
                GisMapAbandonedTurnV1::Complete
            }
            closing @ RetainedGisMapDocumentStateV1::Closing(_) => {
                documents.insert(key.to_owned(), closing);
                GisMapAbandonedTurnV1::Complete
            }
        }
    }

    async fn drive_abandoned_request(&self, key: &str, mut identity: GisMapCommitIdentityV1, request: Arc<GisMapApprovalRequestTokenV1>) {
        let mut retry_delay = std::time::Duration::from_millis(1);
        loop {
            let turn = self.drive_abandoned_turn(key, &identity, &request).await;
            self.announce_state_change();
            let retry = match turn {
                GisMapAbandonedTurnV1::Complete | GisMapAbandonedTurnV1::Aborted => {
                    if matches!(&identity.operation, GisMapCommitOperationV1::Undo { .. }) {
                        return;
                    }
                    let Some(ingress) = identity.ingress.as_ref() else { return };
                    let reader =
                        InferenceReaderV1 { user_id: ingress.user_id(), session_id: ingress.session_id(), authorization_generation: ingress.authorization_generation(), space_id: &identity.scope.space_id, document_id: &identity.scope.document_id };
                    match self.ledger.abandon_prepared_approval(&reader, &identity.job_id, &identity.mutation_id, &identity.command_hash, &identity.proposal_hash) {
                        Ok(_) => return,
                        Err(_) => true,
                    }
                }
                GisMapAbandonedTurnV1::Continue => false,
                GisMapAbandonedTurnV1::Committed => {
                    identity.ingress = None;
                    false
                }
                GisMapAbandonedTurnV1::Recover => self.finish_document_recovery(key).await.is_err(),
                GisMapAbandonedTurnV1::Recovered => match self.resume_recovered_checkpoint(key).await {
                    Ok((identity, generation, receipt)) => self.verify(key, &identity, generation, receipt, super::schema::JOB_MAX_LIFETIME_MS, identity.journal_now_ms).await.is_err(),
                    Err(_) => true,
                },
                GisMapAbandonedTurnV1::Verify { identity, generation, receipt } => self.verify(key, &identity, generation, receipt, super::schema::JOB_MAX_LIFETIME_MS, identity.journal_now_ms).await.is_err(),
                GisMapAbandonedTurnV1::Retry => true,
            };
            if retry {
                tokio::time::sleep(retry_delay).await;
                retry_delay = retry_delay.saturating_mul(2).min(std::time::Duration::from_secs(1));
            } else {
                retry_delay = std::time::Duration::from_millis(1);
                tokio::task::yield_now().await;
            }
        }
    }

    async fn finish_preflight(&self, key: &str, candidate: &GisMapCommitIdentityV1, snapshot: &GisMapParentSnapshotV1, observed: &db::CheckpointPublicationSnapshot) -> Result<(), GisMapApprovalCommitErrorV1> {
        if !Self::checkpoint_matches_frontier(observed, observed.authority_generation, &candidate.base_frontier, &candidate.scope) {
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        let RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(identity), document_write } = state else {
            documents.insert(key.to_owned(), state);
            return Ok(());
        };
        if !Self::identity_matches(&identity, candidate) || owners.generation != observed.authority_generation || !Self::stores_match(&owners, snapshot) {
            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(identity), document_write });
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        let Some(document_write) = document_write else {
            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(identity), document_write: None });
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        };
        match self.build_assembly(owners, identity, snapshot, document_write) {
            Ok(state) => {
                documents.insert(key.to_owned(), state);
                Ok(())
            }
            Err((error, owners)) => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                Err(error)
            }
        }
    }

    fn publication_frontier(scope: &DocumentScope, snapshot: &db::CheckpointPublicationSnapshot) -> Option<ArtifactFrontier> {
        Some(ArtifactFrontier {
            document_id: scope.document_id.clone(),
            head_edit_ordinal: snapshot.frontier.head_seq,
            head_edit_id: snapshot.head_edit_id.as_ref()?.0.clone(),
            last_commit_seq: snapshot.frontier.commit_seq,
            chain_hash: directory::os_directory::ArtifactHash(snapshot.frontier.chain_hash),
        })
    }

    fn publication_wire_frontier(frontier: &ArtifactFrontier) -> directory::os_directory::CheckpointPublicationFrontierV1 {
        directory::os_directory::CheckpointPublicationFrontierV1 {
            document_id: frontier.document_id.clone(),
            head_edit_ordinal: frontier.head_edit_ordinal,
            head_edit_id: frontier.head_edit_id.clone(),
            last_commit_seq: frontier.last_commit_seq,
            chain_sha256: frontier.chain_hash.hex(),
        }
    }

    async fn publish_checkpoint(
        &self,
        key: &str,
        identity: &GisMapCommitIdentityV1,
        generation: u64,
        receipt: &GisMapJournalReceiptV1,
        attempt_lifetime_ms: u64,
        decision_now_ms: u64,
    ) -> Result<directory::os_directory::PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1> {
        let (handle, scope) = {
            let documents = self.documents.lock().await;
            let Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write }) = documents.get(key) else {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            };
            if !Self::identity_matches(stored, identity) || stored_receipt != receipt || owners.generation != generation {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            (owners.handle.clone(), owners.scope.clone())
        };
        let actor_snapshot = handle.checkpoint_publication_snapshot().await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
        let (request, document_write) = {
            let documents = self.documents.lock().await;
            let Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write }) = documents.get(key) else {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            };
            if !Self::identity_matches(stored, identity) || stored_receipt != receipt || owners.generation != generation || owners.scope != scope {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let projected = Self::publication_frontier(&owners.scope, &actor_snapshot).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
            if actor_snapshot.authority_generation != generation
                || projected.head_edit_ordinal != identity.base_frontier.head_edit_ordinal.checked_add(1).ok_or(GisMapApprovalCommitErrorV1::Capacity)?
                || projected.last_commit_seq != identity.base_frontier.last_commit_seq.checked_add(1).ok_or(GisMapApprovalCommitErrorV1::Capacity)?
                || projected.head_edit_id != identity.mutation_id
            {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let files = semio_framework::io::resolve_ready(owners.parent.as_ref().ok_or(GisMapApprovalCommitErrorV1::Storage)?.snapshot_pack()).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            if files.pack.is_empty() || files.spr.is_empty() {
                return Err(GisMapApprovalCommitErrorV1::Storage);
            }
            (
                GisMapApprovalCheckpointRequestV1 {
                    scope: owners.scope.clone(),
                    descriptor_digest: identity.descriptor_digest.clone(),
                    base_frontier: identity.base_frontier.clone(),
                    handle: owners.handle.clone(),
                    actor_snapshot,
                    pair: crate::artifact_authority::ArtifactPair { pack: files.pack, spr: files.spr },
                    journal_receipt: receipt.clone(),
                },
                document_write.clone(),
            )
        };
        let expected_scope = request.scope.clone();
        let expected_descriptor = identity.descriptor_digest.clone();
        let expected_frontier = Self::publication_frontier(&expected_scope, &request.actor_snapshot).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
        let expected_pack_hash = sha256(&request.pair.pack);
        let expected_spr_hash = sha256(&request.pair.spr);
        let published = self.publisher.publish(request, document_write, attempt_lifetime_ms, decision_now_ms).await?;
        if published.scope != expected_scope
            || published.descriptor_digest_v1.hex() != expected_descriptor
            || published.baseline_frontier != expected_frontier
            || published.pack.sha256.hex() != expected_pack_hash
            || published.spr.sha256.hex() != expected_spr_hash
        {
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        }
        Ok(published)
    }

    async fn verify(&self, key: &str, identity: &GisMapCommitIdentityV1, generation: u64, receipt: GisMapJournalReceiptV1, attempt_lifetime_ms: u64, decision_now_ms: u64) -> Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1> {
        let control = Arc::new(InferenceOperationControlV1::new(attempt_lifetime_ms, 65_536).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?);
        let fence = {
            let documents = self.documents.lock().await;
            match documents.get(key) {
                Some(RetainedGisMapDocumentStateV1::Verification { owners, identity: stored, receipt: stored_receipt, .. })
                | Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, .. })
                | Some(RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, .. })
                    if owners.generation == generation && Self::identity_matches(stored, identity) && stored_receipt == &receipt =>
                {
                    owners.fence.clone()
                }
                _ => return Err(GisMapApprovalCommitErrorV1::Conflict),
            }
        };
        let target = InferenceWalTargetV1 {
            scope: identity.scope.clone(),
            generation,
            job_id: identity.job_id.clone(),
            proposal_hash: identity.proposal_hash.clone(),
            mutation_id: identity.mutation_id.clone(),
            command_hash: identity.command_hash.clone(),
            actor: identity.actor.clone(),
            maximum_records: APPROVAL_MAX_RECORDS,
            receipt: receipt.clone(),
        };
        let witness = self
            .verifier
            .verify(target, fence, control)
            .await
            .map_err(|error| match error {
                InferenceErrorV1::Conflict | InferenceErrorV1::Invalid | InferenceErrorV1::Denied => GisMapApprovalCommitErrorV1::Conflict,
                _ => GisMapApprovalCommitErrorV1::Storage,
            })?
            .ok_or(GisMapApprovalCommitErrorV1::Storage)?;
        let already_published = {
            let mut documents = self.documents.lock().await;
            let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
            match state {
                RetainedGisMapDocumentStateV1::Verification { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write });
                    false
                }
                RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write });
                    false
                }
                RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write });
                    true
                }
                state => {
                    documents.insert(key.to_owned(), state);
                    return Err(GisMapApprovalCommitErrorV1::Conflict);
                }
            }
        };
        let published = if already_published { None } else { Some(self.publish_checkpoint(key, identity, generation, &receipt, attempt_lifetime_ms, decision_now_ms).await?) };
        let (after_frontier, after_base_digest) = if let Some(checkpoint) = published.as_ref() {
            (Self::publication_wire_frontier(&checkpoint.baseline_frontier), checkpoint.pack.sha256.hex())
        } else {
            let (handle, after_base_digest) = {
                let documents = self.documents.lock().await;
                match documents.get(key) {
                    Some(RetainedGisMapDocumentStateV1::Published { owners, .. }) => {
                        let files = semio_framework::io::resolve_ready(owners.parent.as_ref().ok_or(GisMapApprovalCommitErrorV1::Storage)?.snapshot_pack()).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
                        (owners.handle.clone(), sha256(&files.pack))
                    }
                    _ => return Err(GisMapApprovalCommitErrorV1::Conflict),
                }
            };
            let snapshot = handle.checkpoint_publication_snapshot().await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            (Self::publication_wire_frontier(&Self::publication_frontier(&identity.scope, &snapshot).ok_or(GisMapApprovalCommitErrorV1::Conflict)?), after_base_digest)
        };
        let (applied, undo) = match &identity.operation {
            GisMapCommitOperationV1::Approval => {
                let reconciliation = self.ledger.reconcile_committed_approval(&identity.job_id, &witness, generation, &after_frontier, &identity.descriptor_digest, &after_base_digest, identity.journal_now_ms).map_err(|error| match error {
                    InferenceErrorV1::Conflict | InferenceErrorV1::Invalid | InferenceErrorV1::Denied => GisMapApprovalCommitErrorV1::Conflict,
                    _ => GisMapApprovalCommitErrorV1::Storage,
                })?;
                (reconciliation.applied, reconciliation.undo)
            }
            GisMapCommitOperationV1::Undo { target_id, .. } => {
                let applied = self.ledger.reconcile_committed_gis_map_approval_undo(target_id, &witness, generation, &after_frontier).map_err(|error| match error {
                    InferenceErrorV1::Conflict | InferenceErrorV1::Invalid | InferenceErrorV1::Denied => GisMapApprovalCommitErrorV1::Conflict,
                    _ => GisMapApprovalCommitErrorV1::Storage,
                })?;
                (applied, directory::os_directory::GisMapApprovalUndoHandleV1 { target_id: target_id.clone(), expected_current: after_frontier.clone() })
            }
        };
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        match state {
            RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                let notification = published.as_ref().map_or(Err(GisMapApprovalCommitErrorV1::Conflict), |checkpoint| match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.publisher.checkpoint_applied(checkpoint))) {
                    Ok(result) => result,
                    Err(_) => Err(GisMapApprovalCommitErrorV1::Storage),
                });
                if let Err(error) = notification {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write });
                    return Err(error);
                }
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write: None });
                drop(document_write);
            }
            RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write: None });
                drop(document_write);
            }
            state => {
                documents.insert(key.to_owned(), state);
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
        }
        Ok(GisMapApprovalReceiptV1 { witness, document_generation: generation, applied, undo, frontier: after_frontier })
    }

    async fn commit_retained(&self, request: GisMapApprovalCommitRequestV1<'_>, mut owner: GisMapApprovalRequestOwnerV1, preflight: (GisMapCommitIdentityV1, GisMapParentSnapshotV1)) -> Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1> {
        self.resume_parked_requests();
        self.resume_parked_prepared();
        let (mut identity, snapshot) = preflight;
        owner.admit(identity.clone());
        let base = request.base;
        let deadline_ms = request.deadline_ms;
        let now_ms = request.now_ms;
        let document_write = request.document_write;
        let mut live_ingress = Some(request.ingress);
        let key = document_key(&identity.scope);
        let request_token = owner.request.clone();
        self.prepare_retained_document(&identity.scope, base, document_write, request_token).await?;
        let result = loop {
            let turn = self.drive_turn(&key, &identity, &snapshot).await;
            self.announce_state_change();
            match turn {
                GisMapCommitTurnV1::Continue => tokio::task::yield_now().await,
                GisMapCommitTurnV1::Committed => {
                    identity.ingress = None;
                    owner.identity.as_mut().expect("active approval request owner").ingress = None;
                    drop(live_ingress.take());
                    tokio::task::yield_now().await;
                }
                GisMapCommitTurnV1::Preflight { handle, generation } => {
                    let observed = handle.checkpoint_publication_snapshot().await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
                    if observed.authority_generation != generation {
                        break Err(GisMapApprovalCommitErrorV1::Conflict);
                    }
                    self.finish_preflight(&key, &identity, &snapshot, &observed).await?;
                }
                GisMapCommitTurnV1::Verify { generation, receipt } => {
                    let attempt_lifetime_ms = deadline_ms.checked_sub(now_ms).filter(|value| *value != 0).ok_or(GisMapApprovalCommitErrorV1::Rejected)?;
                    let verified = self.verify(&key, &identity, generation, receipt, attempt_lifetime_ms, now_ms).await;
                    self.announce_state_change();
                    break verified;
                }
                GisMapCommitTurnV1::Rejected(error) => break Err(error),
            }
        };
        if result.is_ok() {
            owner.complete();
        }
        result
    }

    async fn commit_undo_retained(
        &self,
        request: GisMapApprovalUndoCommitRequestV1<'_>,
        mut owner: GisMapApprovalRequestOwnerV1,
        preflight: (GisMapCommitIdentityV1, GisMapParentSnapshotV1),
    ) -> Result<GisMapApprovalUndoCommitReceiptV1, GisMapApprovalCommitErrorV1> {
        self.resume_parked_requests();
        self.resume_parked_prepared();
        let (mut identity, snapshot) = preflight;
        owner.admit(identity.clone());
        let key = document_key(&identity.scope);
        let request_token = owner.request.clone();
        let mut live_ingress = Some(request.ingress);
        self.prepare_retained_document(&identity.scope, request.base, request.document_write, request_token).await?;
        let result = loop {
            let turn = self.drive_turn(&key, &identity, &snapshot).await;
            self.announce_state_change();
            match turn {
                GisMapCommitTurnV1::Continue => tokio::task::yield_now().await,
                GisMapCommitTurnV1::Committed => {
                    identity.ingress = None;
                    owner.identity.as_mut().expect("active undo request owner").ingress = None;
                    drop(live_ingress.take());
                    tokio::task::yield_now().await;
                }
                GisMapCommitTurnV1::Preflight { handle, generation } => {
                    let observed = handle.checkpoint_publication_snapshot().await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
                    if observed.authority_generation != generation {
                        break Err(GisMapApprovalCommitErrorV1::Conflict);
                    }
                    self.finish_preflight(&key, &identity, &snapshot, &observed).await?;
                }
                GisMapCommitTurnV1::Verify { generation, receipt } => {
                    let attempt_lifetime_ms = request.deadline_ms.checked_sub(request.now_ms).filter(|value| *value != 0).ok_or(GisMapApprovalCommitErrorV1::Rejected)?;
                    break self.verify(&key, &identity, generation, receipt, attempt_lifetime_ms, request.now_ms).await.map(|receipt| GisMapApprovalUndoCommitReceiptV1 {
                        witness: receipt.witness,
                        document_generation: receipt.document_generation,
                        applied: receipt.applied,
                        frontier: receipt.frontier,
                    });
                }
                GisMapCommitTurnV1::Rejected(error) => break Err(error),
            }
        };
        if result.is_ok() {
            owner.complete();
        }
        result
    }

    fn closing(owners: GisMapDocumentStoresV1, document_write: Option<GisMapDocumentWriteLeaseV1>) -> RetainedGisMapDocumentStateV1 {
        owners.fence.invalidate();
        RetainedGisMapDocumentStateV1::Closing(GisMapClosingStoresV1 { owners, phase: 0, document_write })
    }

    fn close_store<P, M>(store: &mut Option<directory::os_store::ArtifactStore<P, M>>) -> Result<bool, GisMapApprovalCommitErrorV1>
    where
        P: directory::ArtifactPack + Clone + directory::ToValue + directory::FromValue + Send + Sync + 'static,
        M: directory::Mutation<P> + directory::os_spr::OpBinary + directory::os_spr::OpText + Clone + directory::ToValue + directory::FromValue + Send + 'static,
    {
        use directory::os_store::{SnapshotRetirementStep, SpaceMember};
        let Some(owner) = store.as_mut() else { return Ok(true) };
        match owner.close_owned_step(1, directory::os_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).map_err(|_| GisMapApprovalCommitErrorV1::Storage)? {
            SnapshotRetirementStep::Complete => {
                if !owner.close_owned_terminal_is_empty() {
                    return Err(GisMapApprovalCommitErrorV1::Storage);
                }
                drop(store.take());
                Ok(true)
            }
            SnapshotRetirementStep::Pending { .. } | SnapshotRetirementStep::Blocked => Ok(false),
        }
    }

    async fn close_turn(&self, key: &str) -> Result<bool, GisMapApprovalCommitErrorV1> {
        use directory::os_store::durable_group::{DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1};
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Ok(true) };
        let state = match state {
            RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                return Err(GisMapApprovalCommitErrorV1::Storage);
            }
            RetainedGisMapDocumentStateV1::Ready { owners, document_write, .. } => Self::closing(owners, document_write),
            RetainedGisMapDocumentStateV1::Published { owners, document_write, .. } => Self::closing(owners, document_write),
            RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write });
                return Err(GisMapApprovalCommitErrorV1::Storage);
            }
            RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write });
                return Err(GisMapApprovalCommitErrorV1::Storage);
            }
            RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write });
                return Err(GisMapApprovalCommitErrorV1::Storage);
            }
            RetainedGisMapDocumentStateV1::Assembly { mut owner, handle, scope, generation, identity, document_write, fence } => {
                owner.cancel();
                match owner.advance(directory::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal) => {
                        let terminal = owner.take_terminal_owners().expect("closed assembly returns exact owners");
                        drop(owner);
                        drop(terminal.sink);
                        Self::closing(Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence), Some(document_write))
                    }
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted) => {
                        let host = owner.take_mounted_host().expect("mounted closing assembly returns its exact host");
                        drop(owner);
                        RetainedGisMapDocumentStateV1::Journal { owner: host, handle, scope, generation, identity, receipt: None, document_write, fence }
                    }
                    Ok(_) => RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence },
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        return Err(GisMapApprovalCommitErrorV1::Storage);
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Journal { mut owner, handle, scope, generation, identity, mut receipt, document_write, fence } => {
                if receipt.is_none() {
                    owner.cancel();
                }
                match owner.advance(directory::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(exact_receipt)) => {
                        if receipt.as_ref().is_some_and(|stored| stored != &exact_receipt) {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return Err(GisMapApprovalCommitErrorV1::Storage);
                        }
                        receipt = Some(exact_receipt);
                        if !owner.acknowledge() {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return Err(GisMapApprovalCommitErrorV1::Storage);
                        }
                        RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence }
                    }
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => {
                        let terminal = owner.take_terminal_owners().expect("closed journal host returns exact owners");
                        drop(owner);
                        drop(terminal.sink);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        match receipt {
                            Some(receipt) => RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write },
                            None => Self::closing(owners, Some(document_write)),
                        }
                    }
                    Ok(_) => RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence },
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        return Err(GisMapApprovalCommitErrorV1::Storage);
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Closing(mut closing) => {
                let closed = match closing.phase {
                    0 => Self::close_store(&mut closing.owners.value),
                    1 => Self::close_store(&mut closing.owners.drawing),
                    2 => Self::close_store(&mut closing.owners.parent),
                    3 => Ok(true),
                    _ => Err(GisMapApprovalCommitErrorV1::Storage),
                };
                let closed = match closed {
                    Ok(closed) => closed,
                    Err(error) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Closing(closing));
                        return Err(error);
                    }
                };
                let complete = match closing.phase {
                    0 if closed => {
                        closing.phase = 1;
                        false
                    }
                    1 if closed => {
                        closing.phase = 2;
                        false
                    }
                    2 if closed => {
                        closing.phase = 3;
                        false
                    }
                    3 => true,
                    _ => false,
                };
                if complete {
                    return Ok(true);
                }
                RetainedGisMapDocumentStateV1::Closing(closing)
            }
        };
        documents.insert(key.to_owned(), state);
        Ok(false)
    }

    /// 🧹 Drives every retained Store, assembly and journal owner to one explicit terminal handoff.
    pub async fn close(&self) -> Result<(), GisMapApprovalCommitErrorV1> {
        {
            let _jobs = self.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            self.closing.store(true, Ordering::Release);
        }
        self.announce_state_change();
        let mut state_changes = self.state_epoch.subscribe();
        loop {
            self.resume_parked_requests();
            self.resume_parked_prepared();
            let active_request = {
                let documents = self.documents.lock().await;
                documents.values().any(Self::state_has_active_request)
            };
            if active_request {
                state_changes.changed().await.map_err(|_| GisMapApprovalCommitErrorV1::Unavailable)?;
                continue;
            }
            let next = {
                let documents = self.documents.lock().await;
                documents
                    .iter()
                    .next()
                    .map(|(key, state)| {
                        let gate = match state {
                            RetainedGisMapDocumentStateV1::Ready { owners, document_write, .. } => {
                                if document_write.is_some() {
                                    return Some((key.clone(), None));
                                }
                                owners.document_write.clone()
                            }
                            RetainedGisMapDocumentStateV1::Published { owners, document_write, .. } => {
                                if document_write.is_some() {
                                    return Some((key.clone(), None));
                                }
                                owners.document_write.clone()
                            }
                            RetainedGisMapDocumentStateV1::Recovery { .. } | RetainedGisMapDocumentStateV1::Recovered { .. } => return Some((key.clone(), None)),
                            RetainedGisMapDocumentStateV1::Assembly { .. } | RetainedGisMapDocumentStateV1::Journal { .. } | RetainedGisMapDocumentStateV1::Verification { .. } | RetainedGisMapDocumentStateV1::Publishing { .. } => {
                                return Some((key.clone(), None));
                            }
                            RetainedGisMapDocumentStateV1::Closing(closing) => {
                                if closing.document_write.is_some() {
                                    return Some((key.clone(), None));
                                }
                                closing.owners.document_write.clone()
                            }
                        };
                        Some((key.clone(), Some(gate)))
                    })
                    .flatten()
            };
            let Some((key, gate)) = next else {
                let maintenance_is_empty = self.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty();
                let cleanup_is_empty = self.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty();
                if maintenance_is_empty && cleanup_is_empty {
                    return Ok(());
                }
                state_changes.changed().await.map_err(|_| GisMapApprovalCommitErrorV1::Unavailable)?;
                continue;
            };
            let maintenance_owns_document = self.maintenance_owns_document(&key);
            if maintenance_owns_document {
                state_changes.changed().await.map_err(|_| GisMapApprovalCommitErrorV1::Unavailable)?;
                continue;
            }
            let _document_write = match gate {
                Some(gate) => Some(Arc::new(GisMapDocumentWriteAuthorityV1::acquire(gate, Arc::new(GisMapApprovalRequestTokenV1)).await)),
                None => None,
            };
            let recovering = {
                let documents = self.documents.lock().await;
                matches!(documents.get(&key), Some(RetainedGisMapDocumentStateV1::Recovery { .. }))
            };
            if recovering {
                self.finish_document_recovery(&key).await?;
                self.announce_state_change();
                continue;
            }
            let recovered = {
                let documents = self.documents.lock().await;
                matches!(documents.get(&key), Some(RetainedGisMapDocumentStateV1::Recovered { .. }))
            };
            if recovered {
                self.resume_recovered_checkpoint(&key).await?;
                self.announce_state_change();
                continue;
            }
            let verification = {
                let documents = self.documents.lock().await;
                match documents.get(&key) {
                    Some(RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, .. }) | Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, .. }) => {
                        Some((identity.clone(), owners.generation, receipt.clone()))
                    }
                    _ => None,
                }
            };
            if let Some((identity, generation, receipt)) = verification {
                self.verify(&key, &identity, generation, receipt, super::schema::JOB_MAX_LIFETIME_MS, identity.journal_now_ms).await?;
                self.announce_state_change();
                continue;
            }
            let closed = self.close_turn(&key).await?;
            self.announce_state_change();
            if closed {
                continue;
            }
            tokio::task::yield_now().await;
        }
    }
}

impl GisMapApprovalCommitterV1 for RetainedGisMapApprovalCommitterV1 {
    fn commit<'a>(&'a self, request: GisMapApprovalCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
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

    fn undo<'a>(&'a self, request: GisMapApprovalUndoCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GisMapApprovalUndoCommitReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
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

    fn close<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(RetainedGisMapApprovalCommitterV1::close(self))
    }
}

/// 🗺️ The server-materialized Map base one job is frozen against.
pub struct InferenceMapBaseV1 {
    pub frontier: ArtifactFrontier,
    pub descriptor_digest: String,
    pub pack: InferencePrivateBytesV1,
}

impl InferenceMapBaseV1 {
    /// 🔐️ Returns the exact base-pack digest the identity froze and every recheck compares.
    pub fn digest(&self) -> String {
        sha256(self.pack.as_slice())
    }

    /// 🕸️ Returns the composed child members this Map owns, in stable-member order.
    pub fn composed_children(&self) -> Result<Vec<String>, InferenceRouteErrorV1> {
        let snapshot = <semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(self.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let mut members = Vec::with_capacity(3);
        if !snapshot.drawing.child_id.is_empty() {
            members.push(snapshot.drawing.child_id.clone());
        }
        if let Some(image) = snapshot.image.as_ref() {
            members.push(image.child_id.clone());
        }
        if !snapshot.value.child_id.is_empty() {
            members.push(snapshot.value.child_id.clone());
        }
        Ok(members)
    }
}

/// 🧮️ One completed deterministic Map inference plus its sole typed proposal.
pub struct InferenceProposalV1 {
    pub result: InferencePrivateBytesV1,
    pub proposal: InferencePrivateBytesV1,
    pub proposal_hash: String,
}

/// 🪪️ The exact durable execution claim one retained worker owns.
#[derive(Clone, Debug, PartialEq, Eq)]
struct InferenceRunKeyV1 {
    job_id: String,
    identity_digest: String,
    run_epoch: u64,
}

impl InferenceRunKeyV1 {
    fn same_run(&self, job_id: &str, identity_digest: &str) -> bool {
        self.job_id == job_id && self.identity_digest == identity_digest && self.run_epoch != 0
    }
}

/// 🧵️ One installed worker whose blocking child is joined by its outer task.
struct RetainedInferenceRunV1 {
    key: InferenceRunKeyV1,
    control: Arc<InferenceOperationControlV1>,
    task: JoinHandle<()>,
}

/// 🛡️ The process-owned admission, cancellation and join authority for GIS inference.
struct RetainedInferenceRunSupervisorV1 {
    closing: AtomicBool,
    capacity: Arc<Semaphore>,
    operations: Mutex<HashMap<String, RetainedInferenceRunV1>>,
}

impl RetainedInferenceRunSupervisorV1 {
    fn new() -> Self {
        Self { closing: AtomicBool::new(false), capacity: Arc::new(Semaphore::new(OPERATION_CAPACITY)), operations: Mutex::new(HashMap::new()) }
    }

    async fn reap_finished_operations(&self) -> Result<(), InferenceRouteErrorV1> {
        let finished = {
            let mut operations = self.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
            let job_ids = operations.iter().filter_map(|(job_id, operation)| operation.task.is_finished().then(|| job_id.clone())).collect::<Vec<_>>();
            job_ids.into_iter().filter_map(|job_id| operations.remove(&job_id)).collect::<Vec<_>>()
        };
        for operation in finished {
            operation.task.await.map_err(|_| InferenceRouteErrorV1::Storage)?;
        }
        Ok(())
    }

    fn interrupt_operation(&self, job_id: &str, identity_digest: &str) -> Result<(), InferenceRouteErrorV1> {
        let operations = self.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if let Some(operation) = operations.get(job_id) {
            if !operation.key.same_run(job_id, identity_digest) {
                return Err(InferenceRouteErrorV1::Denied);
            }
            operation.control.cancel();
        }
        Ok(())
    }

    async fn signal_and_join_operations(&self) -> Result<(), InferenceRouteErrorV1> {
        self.closing.store(true, Ordering::Release);
        let operations = {
            let mut operations = self.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
            std::mem::take(&mut *operations).into_values().collect::<Vec<_>>()
        };
        for operation in &operations {
            operation.control.cancel();
        }
        for operation in operations {
            operation.task.await.map_err(|_| InferenceRouteErrorV1::Storage)?;
        }
        Ok(())
    }
}

enum InferenceOperationAdmissionV1 {
    Existing,
    Installed(oneshot::Sender<()>),
    Unclaimed,
}

/// 🔒️ The private input and authorities retained until the blocking worker is joined.
struct RetainedInferenceRunOwnerV1 {
    runtime: Arc<HubInferenceRuntimeV1>,
    directory: Arc<crate::directory::HubDirectories>,
    rebootstrap: Arc<crate::lag_rebootstrap::VerifiedRebootstrapSource>,
    gate: Arc<tokio::sync::Mutex<()>>,
    scope: DocumentScope,
    identity: InferenceIdentityV1,
    job_id: String,
    expires_at_ms: u64,
    claim: super::sqlite::InferenceRunClaimV1,
    base: InferenceMapBaseV1,
    control: Arc<InferenceOperationControlV1>,
    _permit: OwnedSemaphorePermit,
}

/// 🧪️ A bounded test-only pause placed inside the real GIS codec checkpoint callback.
#[cfg(feature = "test-support")]
pub struct InferenceCheckpointTestGateV1 {
    entered: AtomicBool,
    entered_notify: tokio::sync::Notify,
    released: Mutex<bool>,
    release_notify: std::sync::Condvar,
    inherited: Option<Mutex<std::fs::File>>,
}

#[cfg(feature = "test-support")]
impl InferenceCheckpointTestGateV1 {
    pub fn new() -> Self {
        Self { entered: AtomicBool::new(false), entered_notify: tokio::sync::Notify::new(), released: Mutex::new(false), release_notify: std::sync::Condvar::new(), inherited: None }
    }

    /// 🔌️ Opens the process runner's fixed inherited checkpoint-control endpoint.
    pub fn open_inherited() -> Result<Self, InferenceRouteErrorV1> {
        Ok(Self { entered: AtomicBool::new(false), entered_notify: tokio::sync::Notify::new(), released: Mutex::new(false), release_notify: std::sync::Condvar::new(), inherited: Some(Mutex::new(inherited_inference_checkpoint_file()?)) })
    }

    pub async fn entered(&self) {
        loop {
            let entered = self.entered_notify.notified();
            tokio::pin!(entered);
            entered.as_mut().enable();
            if self.entered.load(Ordering::Acquire) {
                return;
            }
            entered.await;
        }
    }

    pub fn release(&self) {
        if let Ok(mut released) = self.released.lock() {
            *released = true;
            self.release_notify.notify_all();
        }
    }

    fn checkpoint(&self, control: &InferenceOperationControlV1, job_id: &str) -> Result<(), InferenceRouteErrorV1> {
        self.entered.store(true, Ordering::Release);
        self.entered_notify.notify_waiters();
        let mut released = self.released.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if *released {
            return control.checkpoint(control.progress().0).map_err(Into::into);
        }
        if let Some(inherited) = &self.inherited {
            let mut endpoint = inherited.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
            write_inference_checkpoint_control_frame(&mut endpoint, &InferenceCheckpointControlFrameV1::entered(job_id))?;
            let frame = read_inference_checkpoint_control_frame(&mut endpoint)?;
            if frame != InferenceCheckpointControlFrameV1::release(job_id) {
                return Err(InferenceRouteErrorV1::Denied);
            }
            *released = true;
            return control.checkpoint(control.progress().0).map_err(Into::into);
        }
        while !*released {
            control.checkpoint(control.progress().0)?;
            let waited = self.release_notify.wait_timeout(released, std::time::Duration::from_millis(10)).map_err(|_| InferenceRouteErrorV1::Storage)?;
            released = waited.0;
        }
        Ok(())
    }
}

#[cfg(feature = "test-support")]
const INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES: usize = 256;
#[cfg(feature = "test-support")]
const INHERITED_INFERENCE_CHECKPOINT_DESCRIPTOR: i32 = 4;

#[cfg(feature = "test-support")]
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct InferenceCheckpointControlFrameV1 {
    schema: String,
    version: u8,
    sequence: u8,
    kind: String,
    job_id: String,
}

#[cfg(feature = "test-support")]
impl InferenceCheckpointControlFrameV1 {
    fn entered(job_id: &str) -> Self {
        Self { schema: "semio.hub.gis-inference-checkpoint-control/v1".into(), version: 1, sequence: 1, kind: "entered".into(), job_id: job_id.into() }
    }

    fn release(job_id: &str) -> Self {
        Self { schema: "semio.hub.gis-inference-checkpoint-control/v1".into(), version: 1, sequence: 2, kind: "release".into(), job_id: job_id.into() }
    }
}

#[cfg(feature = "test-support")]
fn write_inference_checkpoint_control_frame(endpoint: &mut std::fs::File, frame: &InferenceCheckpointControlFrameV1) -> Result<(), InferenceRouteErrorV1> {
    use std::io::Write as _;
    let bytes = serde_json::to_vec(frame).map_err(|_| InferenceRouteErrorV1::Storage)?;
    if bytes.is_empty() || bytes.len() > INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let length = u32::try_from(bytes.len()).map_err(|_| InferenceRouteErrorV1::Bounds)?.to_be_bytes();
    endpoint.write_all(&length).and_then(|_| endpoint.write_all(&bytes)).and_then(|_| endpoint.flush()).map_err(|_| InferenceRouteErrorV1::Storage)
}

#[cfg(feature = "test-support")]
fn read_inference_checkpoint_control_frame(endpoint: &mut std::fs::File) -> Result<InferenceCheckpointControlFrameV1, InferenceRouteErrorV1> {
    use std::io::Read as _;
    let mut length = [0_u8; 4];
    endpoint.read_exact(&mut length).map_err(|_| InferenceRouteErrorV1::Storage)?;
    let length = usize::try_from(u32::from_be_bytes(length)).map_err(|_| InferenceRouteErrorV1::Bounds)?;
    if length == 0 || length > INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let mut bytes = vec![0_u8; length];
    endpoint.read_exact(&mut bytes).map_err(|_| InferenceRouteErrorV1::Storage)?;
    let frame = serde_json::from_slice::<InferenceCheckpointControlFrameV1>(&bytes).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let canonical = serde_json::to_vec(&frame).map_err(|_| InferenceRouteErrorV1::Storage)?;
    let exact = canonical == bytes;
    bytes.fill(0);
    if !exact {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    Ok(frame)
}

#[cfg(all(feature = "test-support", unix))]
fn inherited_inference_checkpoint_file() -> Result<std::fs::File, InferenceRouteErrorV1> {
    use std::os::fd::FromRawFd as _;
    unsafe extern "C" {
        fn fcntl(fd: i32, command: i32, ...) -> i32;
    }
    if unsafe { fcntl(INHERITED_INFERENCE_CHECKPOINT_DESCRIPTOR, 1) } < 0 {
        return Err(InferenceRouteErrorV1::Unavailable);
    }
    Ok(unsafe { std::fs::File::from_raw_fd(INHERITED_INFERENCE_CHECKPOINT_DESCRIPTOR) })
}

#[cfg(all(feature = "test-support", windows))]
fn inherited_inference_checkpoint_file() -> Result<std::fs::File, InferenceRouteErrorV1> {
    use std::os::windows::io::FromRawHandle as _;
    unsafe extern "C" {
        fn _get_osfhandle(fd: i32) -> isize;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
        fn DuplicateHandle(source_process: *mut std::ffi::c_void, source_handle: *mut std::ffi::c_void, target_process: *mut std::ffi::c_void, target_handle: *mut *mut std::ffi::c_void, desired_access: u32, inherit_handle: i32, options: u32) -> i32;
    }
    let handle = unsafe { _get_osfhandle(INHERITED_INFERENCE_CHECKPOINT_DESCRIPTOR) };
    if handle == -1 {
        return Err(InferenceRouteErrorV1::Unavailable);
    }
    let process = unsafe { GetCurrentProcess() };
    let mut duplicate = std::ptr::null_mut();
    if unsafe { DuplicateHandle(process, handle as *mut std::ffi::c_void, process, &mut duplicate, 0, 0, 0x0000_0002) } == 0 || duplicate.is_null() {
        return Err(InferenceRouteErrorV1::Unavailable);
    }
    Ok(unsafe { std::fs::File::from_raw_handle(duplicate) })
}

/// 🏃️ The hub's only inference authority: frozen binding, ledger, gates, retained cancellation.
pub struct HubInferenceRuntimeV1 {
    binding: Arc<VerifiedGisMapArtifactBindingV1>,
    ledger: Arc<InferenceJobLedgerV1>,
    supervisor: RetainedInferenceRunSupervisorV1,
    document_gates: Mutex<HashMap<String, Arc<tokio::sync::Mutex<()>>>>,
    committer: Arc<dyn GisMapApprovalCommitterV1>,
    #[cfg(feature = "test-support")]
    checkpoint_test_gate: Mutex<Option<Arc<InferenceCheckpointTestGateV1>>>,
}

impl HubInferenceRuntimeV1 {
    /// 🧊️ Binds one process-lifetime runtime to an already-verified GIS Map selection.
    pub fn new(binding: Arc<VerifiedGisMapArtifactBindingV1>, ledger: Arc<InferenceJobLedgerV1>, committer: Arc<dyn GisMapApprovalCommitterV1>) -> Self {
        Self {
            binding,
            ledger,
            supervisor: RetainedInferenceRunSupervisorV1::new(),
            document_gates: Mutex::new(HashMap::new()),
            committer,
            #[cfg(feature = "test-support")]
            checkpoint_test_gate: Mutex::new(None),
        }
    }

    pub fn binding(&self) -> &Arc<VerifiedGisMapArtifactBindingV1> {
        &self.binding
    }

    pub fn ledger(&self) -> &Arc<InferenceJobLedgerV1> {
        &self.ledger
    }

    pub fn committer(&self) -> &Arc<dyn GisMapApprovalCommitterV1> {
        &self.committer
    }

    /// 🧪️ Installs one exact pause at the real native codec checkpoint for physical race laws.
    #[cfg(feature = "test-support")]
    pub fn install_checkpoint_test_gate(&self, gate: Arc<InferenceCheckpointTestGateV1>) -> Result<(), InferenceRouteErrorV1> {
        let mut selected = self.checkpoint_test_gate.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if selected.is_some() {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        *selected = Some(gate);
        Ok(())
    }

    /// 🧪️ Reaps finished workers and returns the exact retained task count.
    #[cfg(feature = "test-support")]
    pub async fn retained_operation_count_for_test(&self) -> Result<usize, InferenceRouteErrorV1> {
        self.reap_finished_operations().await?;
        self.supervisor.operations.lock().map(|operations| operations.len()).map_err(|_| InferenceRouteErrorV1::Storage)
    }

    /// 🧹️ Stops admission, signals and joins every retained worker, then closes approval owners.
    pub async fn close(&self) -> Result<(), GisMapApprovalCommitErrorV1> {
        self.supervisor.signal_and_join_operations().await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
        self.committer.close().await
    }

    /// 🚧️ Returns the one per-`DocumentScope` async gate every checked phase serializes on.
    pub fn document_gate(&self, scope: &DocumentScope) -> Result<Arc<tokio::sync::Mutex<()>>, InferenceRouteErrorV1> {
        let key = document_key(scope);
        let mut gates = self.document_gates.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if let Some(gate) = gates.get(&key) {
            return Ok(gate.clone());
        }
        if gates.len() >= DOCUMENT_GATE_CAPACITY {
            gates.retain(|_, gate| Arc::strong_count(gate) > 1);
        }
        if gates.len() >= DOCUMENT_GATE_CAPACITY {
            return Err(InferenceRouteErrorV1::Capacity);
        }
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        gates.insert(key, gate.clone());
        Ok(gate)
    }

    async fn reap_finished_operations(&self) -> Result<(), InferenceRouteErrorV1> {
        self.supervisor.reap_finished_operations().await
    }

    fn install_operation<F, Fut>(&self, receipt: &super::sqlite::InferenceJobReceiptV1, identity: &InferenceIdentityV1, now_ms: u64, control: Arc<InferenceOperationControlV1>, task: F) -> Result<InferenceOperationAdmissionV1, InferenceRouteErrorV1>
    where
        F: FnOnce(super::sqlite::InferenceRunClaimV1, oneshot::Receiver<()>, OwnedSemaphorePermit) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut operations = self.supervisor.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if let Some(operation) = operations.get(&receipt.job_id) {
            return if operation.key.same_run(&receipt.job_id, &receipt.identity_digest) { Ok(InferenceOperationAdmissionV1::Existing) } else { Err(InferenceRouteErrorV1::Conflict) };
        }
        if self.supervisor.closing.load(Ordering::Acquire) {
            return Err(InferenceRouteErrorV1::Unavailable);
        }
        let permit = self.supervisor.capacity.clone().try_acquire_owned().map_err(|_| InferenceRouteErrorV1::Capacity)?;
        let Some(claim) = self.ledger.start(&receipt.job_id, identity, now_ms)? else {
            return Ok(InferenceOperationAdmissionV1::Unclaimed);
        };
        let key = InferenceRunKeyV1 { job_id: receipt.job_id.clone(), identity_digest: receipt.identity_digest.clone(), run_epoch: claim.run_epoch };
        let (start, ready) = oneshot::channel();
        let handle = tokio::spawn(task(claim, ready, permit));
        operations.insert(receipt.job_id.clone(), RetainedInferenceRunV1 { key, control, task: handle });
        Ok(InferenceOperationAdmissionV1::Installed(start))
    }

    /// 🛑️ Interrupts only the exact retained claim already authenticated by its private identity.
    fn interrupt_operation(&self, job_id: &str, identity_digest: &str) -> Result<(), InferenceRouteErrorV1> {
        self.supervisor.interrupt_operation(job_id, identity_digest)
    }

    fn fresh_now_ms(&self) -> Result<u64, InferenceRouteErrorV1> {
        fresh_now_ms()
    }

    /// 🔍️ Rejects any drift between the frozen job identity and the current server-materialized base.
    pub fn compare_frozen(&self, identity: &InferenceIdentityV1, scope: &DocumentScope, base: &InferenceMapBaseV1) -> Result<(), InferenceRouteErrorV1> {
        compare_frozen_identity(&self.binding.identity(), identity, scope, base)
    }

    /// 🧮️ Runs the frozen native GIS service under the claim's control and derives its sole proposal.
    pub fn infer(
        &self,
        identity: &InferenceIdentityV1,
        job_id: &str,
        base: &InferenceMapBaseV1,
        control: &InferenceOperationControlV1,
        checkpoint: &mut dyn FnMut(u64, u64) -> Result<(), InferenceRouteErrorV1>,
    ) -> Result<InferenceProposalV1, InferenceRouteErrorV1> {
        use directory::FromValue;
        use semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;
        let _ = identity;
        let budgets = semio_framework_plugin::WireArtifactInferenceBudget { allocation_bytes: ALLOCATION_BYTES, work_units: WORK_UNIT_LIMIT, recursion_depth: RECURSION_DEPTH };
        let request = semio_framework_plugin::ArtifactInferenceExecutionRequest {
            policy: b"gis-map-v1",
            budgets: &budgets,
            cancellation_id: job_id,
            previous_state: None,
            requested_cache_mode: semio_framework_plugin::WireArtifactInferenceCacheMode::Cold,
            canonical_payload: base.pack.as_slice(),
            dependencies: &[],
        };
        let mut failure: Option<InferenceRouteErrorV1> = None;
        let execution = semio_s_plugin_gis::artifacts::gismap::infer_gis_map_controlled(&request, &mut |completed| {
            if let Err(error) = control.checkpoint(completed) {
                failure = Some(error.into());
                return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("hub.inference.interrupted", "bounded inference interrupted"));
            }
            #[cfg(feature = "test-support")]
            if let Some(gate) = self.checkpoint_test_gate.lock().ok().and_then(|selected| selected.clone()) {
                if let Err(error) = gate.checkpoint(control, job_id) {
                    failure = Some(error);
                    return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("hub.inference.interrupted", "bounded inference interrupted"));
                }
            }
            if let Err(error) = checkpoint(completed, WORK_UNIT_LIMIT) {
                failure = Some(error);
                return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("hub.inference.interrupted", "bounded inference interrupted"));
            }
            Ok(())
        });
        let execution = match execution {
            Ok(execution) => execution,
            Err(_) => return Err(failure.unwrap_or(InferenceRouteErrorV1::Invalid)),
        };
        let snapshot = <semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let inference = GisMapInference::from_value(directory::pack_rt::decode_wire_value(&execution.canonical_payload).map_err(|_| InferenceRouteErrorV1::Invalid)?).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let mutation = inference.bounds_proposal(&snapshot, job_id).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let proposal_bytes = directory::os_pack::json::to_json_string(&mutation).into_bytes();
        let proposal = InferencePrivateBytesV1::new(proposal_bytes, PROPOSAL_MAX_BYTES)?;
        let proposal_hash = sha256(proposal.as_slice());
        Ok(InferenceProposalV1 { result: InferencePrivateBytesV1::new(execution.canonical_payload, RESULT_MAX_BYTES)?, proposal, proposal_hash })
    }

    /// ✍️ Rebuilds the sole `CreateRegion` and its inverse server-side and stamps one canonical envelope.
    pub fn server_stamped_command(&self, identity: &InferenceIdentityV1, job_id: &str, base: &InferenceMapBaseV1, proposal_hash: &str, now_ms: u64) -> Result<InferencePrivateBytesV1, InferenceRouteErrorV1> {
        use semio_s_plugin_gis::artifacts::gismap::mutations::inverse_gis_map_mutation;
        let (snapshot, inference) = deterministic_map_inference(base, job_id)?;
        let mutation = inference.bounds_proposal(&snapshot, job_id).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let proposal_bytes = directory::os_pack::json::to_json_string(&mutation).into_bytes();
        if sha256(&proposal_bytes) != proposal_hash {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let inverse = inverse_gis_map_mutation(&snapshot, &mutation);
        let inverse_bytes = directory::os_pack::json::to_json_string(&inverse).into_bytes();
        let mutation_id = approval_mutation_id(job_id, proposal_hash);
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let bytes = encode_server_stamped_command_v1(&CanonicalInferenceCommandPartsV1 {
            mutation_id: &mutation_id,
            document_id: &document_key(&scope),
            actor: &approval_actor(identity),
            diff_schema: GIS_DOCUMENT_SCHEMA,
            diff_payload: &proposal_bytes,
            inverse_schema: GIS_DOCUMENT_SCHEMA,
            inverse_payload: &inverse_bytes,
            timestamp: protocol::HybridLogicalTimestamp { actor: 1, physical_ms: now_ms, logical: 0 },
        })?;
        Ok(InferencePrivateBytesV1::new(bytes, super::command::COMMAND_MAX_BYTES)?)
    }

    /// ↩️ Rebuilds the original fixed-three work and stamps only its exact typed inverse.
    fn server_stamped_undo_command(&self, target: &super::sqlite::GisMapApprovalUndoTargetV1, idempotency_key: &str, base: &InferenceMapBaseV1, now_ms: u64) -> Result<GisMapPreparedUndoCommandV1, InferenceRouteErrorV1> {
        use directory::Inference as _;
        use semio_s_plugin_gis::artifacts::gismap::mutations::{apply_gis_map_mutation, GisMapMutation};
        use semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;

        if sha256(target.original_command.as_slice()) != target.original_command_hash {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let original = CanonicalInferenceCommandV1::decode(target.original_command.as_slice()).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let inverse_text = std::str::from_utf8(original.inverse_payload()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let inverses = directory::os_pack::json::from_json_str::<Vec<GisMapMutation>>(inverse_text).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        if inverses.len() != 1 {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let current = <semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let mut before = current.clone();
        apply_gis_map_mutation(&mut before, &inverses[0]).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let work = GisMapInference::infer(&before).create_region_group_work(&before, &target.original_job_id).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let original_forward = directory::os_pack::json::to_json_string(&work.parent).into_bytes();
        let original_inverse = directory::os_pack::json::to_json_string(&work.parent_inverse).into_bytes();
        let actor = format!("user:{}#session:{}", target.user_id, target.session_id);
        if target.original_mutation_id != approval_mutation_id(&target.original_job_id, &sha256(&original_forward))
            || !original.matches_fixed_three_parent(&target.original_mutation_id, &document_key(&target.scope), &actor, &original_forward, &original_inverse)
        {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let diff = directory::os_pack::json::to_json_string(&inverses[0]).into_bytes();
        let inverse = directory::os_pack::json::to_json_string(&vec![work.parent]).into_bytes();
        let operation_id = sha256(format!("semio.hub.gis-map-approval-undo-operation/v1\0{}\0{idempotency_key}", target.target_id).as_bytes())[..32].to_owned();
        let proposal_hash = sha256(&diff);
        let mutation_id = approval_mutation_id(&operation_id, &proposal_hash);
        let bytes = encode_server_stamped_command_v1(&CanonicalInferenceCommandPartsV1 {
            mutation_id: &mutation_id,
            document_id: &document_key(&target.scope),
            actor: &actor,
            diff_schema: GIS_DOCUMENT_SCHEMA,
            diff_payload: &diff,
            inverse_schema: GIS_DOCUMENT_SCHEMA,
            inverse_payload: &inverse,
            timestamp: protocol::HybridLogicalTimestamp { actor: 1, physical_ms: now_ms, logical: 0 },
        })?;
        let command_hash = sha256(&bytes);
        Ok(GisMapPreparedUndoCommandV1 { operation_id, proposal_hash, mutation_id, command_hash, command: InferencePrivateBytesV1::new(bytes, super::command::COMMAND_MAX_BYTES)? })
    }

    /// 🧾️ Hands the prepared envelope to the composition transaction and reconciles only its witness.
    pub async fn commit_approval(
        &self,
        identity: &InferenceIdentityV1,
        job_id: &str,
        proposal_hash: &str,
        command: &InferencePrivateBytesV1,
        base: &InferenceMapBaseV1,
        deadline_ms: u64,
        now_ms: u64,
        document_write: Arc<tokio::sync::Mutex<()>>,
        ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
    ) -> Result<GisMapApprovalOutcomeV1, InferenceRouteErrorV1> {
        commit_prepared_approval(&self.committer, identity, job_id, proposal_hash, command, base, deadline_ms, now_ms, document_write, ingress).await
    }

    async fn commit_undo(
        &self,
        target: &super::sqlite::GisMapApprovalUndoTargetV1,
        idempotency_key: &str,
        prepared: &GisMapPreparedUndoCommandV1,
        base: &InferenceMapBaseV1,
        deadline_ms: u64,
        now_ms: u64,
        document_write: Arc<tokio::sync::Mutex<()>>,
        ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
    ) -> Result<GisMapApprovalUndoCommitReceiptV1, InferenceRouteErrorV1> {
        let composed_children = base.composed_children()?;
        let actor = format!("user:{}#session:{}", target.user_id, target.session_id);
        self.committer
            .undo(GisMapApprovalUndoCommitRequestV1 {
                composed_children: &composed_children,
                scope: &target.scope,
                actor: &actor,
                target,
                idempotency_key,
                mutation_id: &prepared.mutation_id,
                command_hash: &prepared.command_hash,
                operation_id: &prepared.operation_id,
                proposal_hash: &prepared.proposal_hash,
                command: prepared.command.as_slice(),
                base,
                deadline_ms,
                now_ms,
                document_write,
                ingress,
            })
            .await
            .map_err(|error| match error {
                GisMapApprovalCommitErrorV1::Unavailable => InferenceRouteErrorV1::CommitUnavailable,
                GisMapApprovalCommitErrorV1::Rejected => InferenceRouteErrorV1::Denied,
                GisMapApprovalCommitErrorV1::Conflict => InferenceRouteErrorV1::Conflict,
                GisMapApprovalCommitErrorV1::Capacity => InferenceRouteErrorV1::Capacity,
                GisMapApprovalCommitErrorV1::Storage => InferenceRouteErrorV1::Storage,
            })
    }
}

/// 🧮️ Runs the frozen native GIS executable once and decodes exactly its own canonical result.
fn deterministic_map_inference(
    base: &InferenceMapBaseV1,
    job_id: &str,
) -> Result<(semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot, semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference), InferenceRouteErrorV1> {
    use directory::FromValue;
    use semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;
    let budgets = semio_framework_plugin::WireArtifactInferenceBudget { allocation_bytes: ALLOCATION_BYTES, work_units: WORK_UNIT_LIMIT, recursion_depth: RECURSION_DEPTH };
    let execution = semio_s_plugin_gis::artifacts::gismap::infer_gis_map_controlled(
        &semio_framework_plugin::ArtifactInferenceExecutionRequest {
            policy: b"gis-map-v1",
            budgets: &budgets,
            cancellation_id: job_id,
            previous_state: None,
            requested_cache_mode: semio_framework_plugin::WireArtifactInferenceCacheMode::Cold,
            canonical_payload: base.pack.as_slice(),
            dependencies: &[],
        },
        &mut |_| Ok(()),
    )
    .map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let snapshot = <semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let inference = GisMapInference::from_value(directory::pack_rt::decode_wire_value(&execution.canonical_payload).map_err(|_| InferenceRouteErrorV1::Invalid)?).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    Ok((snapshot, inference))
}

/// 🔍️ Compares one frozen binding plus scope, document, frontier and base-pack digest, exactly.
pub fn compare_frozen_identity(frozen: &super::schema::InferenceBindingIdentityV1, identity: &InferenceIdentityV1, scope: &DocumentScope, base: &InferenceMapBaseV1) -> Result<(), InferenceRouteErrorV1> {
    if identity.binding != *frozen {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    identity.validate()?;
    if identity.space_id != scope.space_id
        || identity.document_id != scope.document_id
        || identity.descriptor_digest != base.descriptor_digest
        || identity.head_ordinal != base.frontier.head_edit_ordinal
        || identity.head_edit_id != base.frontier.head_edit_id
        || identity.last_commit_seq != base.frontier.last_commit_seq
        || identity.input_hash != base.digest()
    {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    Ok(())
}

/// 🧾️ Publishes one prepared envelope and reconciles the outbox only against its real WAL witness.
#[allow(clippy::too_many_arguments)]
pub async fn commit_prepared_approval(
    committer: &Arc<dyn GisMapApprovalCommitterV1>,
    identity: &InferenceIdentityV1,
    job_id: &str,
    proposal_hash: &str,
    command: &InferencePrivateBytesV1,
    base: &InferenceMapBaseV1,
    deadline_ms: u64,
    now_ms: u64,
    document_write: Arc<tokio::sync::Mutex<()>>,
    ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
) -> Result<GisMapApprovalOutcomeV1, InferenceRouteErrorV1> {
    let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
    if ingress.scope() != &scope || ingress.user_id() != identity.user_id || ingress.session_id() != identity.session_id || ingress.authorization_generation() != identity.authorization_generation {
        return Err(InferenceRouteErrorV1::Denied);
    }
    let composed_children = base.composed_children()?;
    if composed_children.as_slice() != ["gismap-drawing", "gismap-value"] {
        return Err(InferenceRouteErrorV1::CommitUnavailable);
    }
    let receipt = committer
        .commit(GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &approval_actor(identity),
            mutation_id: &approval_mutation_id(job_id, proposal_hash),
            command_hash: &sha256(command.as_slice()),
            job_id,
            proposal_hash,
            command: command.as_slice(),
            base,
            base_frontier: &base.frontier,
            deadline_ms,
            now_ms,
            document_write,
            ingress,
        })
        .await
        .map_err(|error| match error {
            GisMapApprovalCommitErrorV1::Unavailable => InferenceRouteErrorV1::CommitUnavailable,
            GisMapApprovalCommitErrorV1::Rejected => InferenceRouteErrorV1::Denied,
            GisMapApprovalCommitErrorV1::Conflict => InferenceRouteErrorV1::Conflict,
            GisMapApprovalCommitErrorV1::Capacity => InferenceRouteErrorV1::Capacity,
            GisMapApprovalCommitErrorV1::Storage => InferenceRouteErrorV1::Storage,
        })?;
    Ok(GisMapApprovalOutcomeV1 { applied: receipt.applied, undo: receipt.undo })
}

//#region 🛣️Routes
/// 🛣️ Everything one authenticated inference route call may read; no ambient authority exists.
pub struct InferenceRouteContextV1<'a> {
    pub runtime: &'a Arc<HubInferenceRuntimeV1>,
    pub directory: &'a Arc<crate::directory::HubDirectories>,
    pub rebootstrap: &'a Arc<crate::lag_rebootstrap::VerifiedRebootstrapSource>,
    pub scope: DocumentScope,
    pub token: Option<&'a str>,
    pub now_ms: u64,
    pub document_write: Arc<tokio::sync::Mutex<()>>,
}

/// 🛡️ Approval-only route context retaining Hub admission before document-write acquisition.
pub struct InferenceApprovalRouteContextV1<'a> {
    pub route: InferenceRouteContextV1<'a>,
    pub ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
}

/// 🧾️ The closed receipt a submitted job returns; it never carries private result or base bytes.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceJobReceiptDtoV1 {
    pub schema: &'static str,
    pub job_id: String,
    pub state: super::schema::InferenceJobStateV1,
    pub proposal_state: super::schema::InferenceProposalStateV1,
    pub proposal_hash: Option<String>,
    pub cursor: u64,
    pub expires_at_ms: u64,
}

/// 📈️ One owner-private progress row rendered on the wire.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceProgressDtoV1 {
    pub cursor: u64,
    pub run_epoch: u64,
    pub completed: u64,
    pub total: u64,
    pub at_ms: u64,
}

/// 🗓️ One owner-private lifecycle event rendered on the wire.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceEventDtoV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
}

/// 📃️ The owner-private bounded page a single `events` read returns.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceEventPageDtoV1 {
    pub schema: &'static str,
    pub job_id: String,
    pub state: super::schema::InferenceJobStateV1,
    pub proposal_state: super::schema::InferenceProposalStateV1,
    pub cancel_requested: bool,
    pub stale: bool,
    pub proposal_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<GisMapInferencePreviewDtoV1>,
    pub events: Vec<InferenceEventDtoV1>,
    pub progress: Vec<InferenceProgressDtoV1>,
    pub next_cursor: u64,
}

/// 🗺️ The bounded, host-only geometry an owner may inspect before approving a proposal.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapInferencePreviewDtoV1 {
    pub schema: &'static str,
    pub job_id: String,
    pub proposal_hash: String,
    pub region_id: String,
    pub ring: [[f64; 2]; 5],
}

fn gis_map_inference_preview(job_id: &str, proposal_hash: &str, proposal: &[u8]) -> Result<GisMapInferencePreviewDtoV1, InferenceRouteErrorV1> {
    use semio_s_plugin_gis::artifacts::gismap::mutations::GisMapMutation;

    if sha256(proposal) != proposal_hash {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let text = std::str::from_utf8(proposal).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let mutation = directory::os_pack::json::from_json_str::<GisMapMutation>(text).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let GisMapMutation::CreateRegion(created) = mutation else {
        return Err(InferenceRouteErrorV1::Conflict);
    };
    let region_id = format!("inference-{job_id}");
    let entries = created.item.data.as_object().ok_or(InferenceRouteErrorV1::Invalid)?;
    if created.item.id != region_id || entries.len() != 3 || created.item.data.get("id").and_then(|value| value.as_str()) != Some(region_id.as_str()) || created.item.data.get("kind").and_then(|value| value.as_str()) != Some("inference-bounds") {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let points = created.item.data.get("ring").and_then(|value| value.as_array()).ok_or(InferenceRouteErrorV1::Invalid)?;
    if points.len() != 5 {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let mut ring = [[0.0_f64; 2]; 5];
    for (index, point) in points.iter().enumerate() {
        let coordinates = point.as_array().ok_or(InferenceRouteErrorV1::Invalid)?;
        if coordinates.len() != 2 {
            return Err(InferenceRouteErrorV1::Bounds);
        }
        ring[index] = [coordinates[0].as_f64().ok_or(InferenceRouteErrorV1::Invalid)?, coordinates[1].as_f64().ok_or(InferenceRouteErrorV1::Invalid)?];
    }
    let [lon_min, lat_min] = ring[0];
    let [lon_max, lat_max] = ring[2];
    if !ring.iter().flatten().all(|value| value.is_finite())
        || !(-180.0..=180.0).contains(&lon_min)
        || !(-180.0..=180.0).contains(&lon_max)
        || !(-90.0..=90.0).contains(&lat_min)
        || !(-90.0..=90.0).contains(&lat_max)
        || lon_min > lon_max
        || lat_min > lat_max
        || ring != [[lon_min, lat_min], [lon_max, lat_min], [lon_max, lat_max], [lon_min, lat_max], [lon_min, lat_min]]
    {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    Ok(GisMapInferencePreviewDtoV1 { schema: "semio.hub.gis-map-inference-preview/v1", job_id: job_id.to_owned(), proposal_hash: proposal_hash.to_owned(), region_id, ring })
}

/// ✅️ The closed approval outcome; `applied` is true only after a real committed-WAL witness.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceApprovalReceiptDtoV1 {
    pub schema: &'static str,
    pub job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub proposal_hash: String,
    pub applied: bool,
    pub undo: directory::os_directory::GisMapApprovalUndoHandleV1,
}

async fn authenticated_session(directory: &Arc<crate::directory::HubDirectories>, token: Option<&str>) -> Result<crate::directory::model::AuthSessionRecord, InferenceRouteErrorV1> {
    let Some(crate::directory::HubCapability::Session(capability)) = token.and_then(|value| crate::directory::HubCapability::parse(value).ok()) else {
        return Err(InferenceRouteErrorV1::Denied);
    };
    directory.authenticate_session(&capability).await.map_err(|_| InferenceRouteErrorV1::Storage)?.ok_or(InferenceRouteErrorV1::Denied)
}

/// 🗺️ Materializes the current Map base through the verified active checkpoint pair reader alone.
///
/// The retained `MemberFactory::Open` opener is the intended long-term materializer; it is not
/// native-accepted yet, so this boundary reads the authority-verified pack of the active checkpoint
/// and never accepts a client-supplied Map pack.
pub async fn map_base(rebootstrap: &Arc<crate::lag_rebootstrap::VerifiedRebootstrapSource>, scope: &DocumentScope, deadline_ms: u64, control: &InferenceMapBaseControlV1) -> Result<InferenceMapBaseV1, InferenceRouteErrorV1> {
    let context = crate::lag_rebootstrap::RebootstrapContext::new(deadline_ms, control);
    let pair = rebootstrap.active_pair(scope, &context).await.map_err(|_| InferenceRouteErrorV1::Unavailable)?;
    let selection = &pair.selection;
    Ok(InferenceMapBaseV1 {
        frontier: selection.baseline_frontier.clone(),
        descriptor_digest: directory::os_directory::hex_lower(&selection.descriptor_digest_v1.0),
        pack: InferencePrivateBytesV1::new(pair.pair().pack.clone(), super::schema::INPUT_MAX_BYTES)?,
    })
}

/// ⏱️ Bounded transfer control for one base materialization; it never reports outside its deadline.
pub struct InferenceMapBaseControlV1 {
    pub deadline_ms: u64,
    pub now_ms: u64,
    pub control: Arc<InferenceOperationControlV1>,
}

impl crate::lag_rebootstrap::RebootstrapTransferControl for InferenceMapBaseControlV1 {
    fn now_ms(&self) -> u64 {
        self.now_ms
    }

    fn is_cancelled(&self) -> bool {
        self.control.checkpoint(self.control.progress().0).is_err()
    }

    fn report(&self, _progress: crate::lag_rebootstrap::RebootstrapProgress) {}
}

async fn finish_retained_gis_map_job(owner: RetainedInferenceRunOwnerV1, outcome: Result<InferenceProposalV1, InferenceRouteErrorV1>) {
    let Ok(now_ms) = owner.runtime.fresh_now_ms() else {
        return;
    };
    let _guard = owner.gate.lock().await;
    match outcome {
        Ok(proposal) => {
            let checked = super::authorization::check_live_inference_author(&owner.directory, &owner.identity, &owner.scope, || i64::try_from(now_ms).unwrap_or(i64::MAX), &owner.control).await.map_err(InferenceRouteErrorV1::from);
            let checked = match checked {
                Ok(()) => {
                    let base_control = InferenceMapBaseControlV1 { deadline_ms: owner.expires_at_ms, now_ms, control: owner.control.clone() };
                    map_base(&owner.rebootstrap, &owner.scope, base_control.deadline_ms, &base_control).await.and_then(|current| owner.runtime.compare_frozen(&owner.identity, &owner.scope, &current))
                }
                Err(error) => Err(error),
            };
            if checked.is_ok() {
                let _ = owner.runtime.ledger().succeed(&owner.job_id, &owner.identity, owner.claim.run_epoch, &proposal.result, &proposal.proposal, now_ms);
            } else {
                let _ = owner.runtime.ledger().cancel_run(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, now_ms);
            }
        }
        Err(InferenceRouteErrorV1::Cancelled | InferenceRouteErrorV1::Expired) => {
            let _ = owner.runtime.ledger().cancel_run(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, now_ms);
        }
        Err(_) => {
            let _ = owner.runtime.ledger().fail_run(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, now_ms);
        }
    }
}

async fn drive_retained_gis_map_job(owner: RetainedInferenceRunOwnerV1, ready: oneshot::Receiver<()>) {
    if ready.await.is_err() {
        return;
    }
    let terminal_runtime = owner.runtime.clone();
    let terminal_identity = owner.identity.clone();
    let terminal_job_id = owner.job_id.clone();
    let terminal_claim = owner.claim;
    let terminal_control = owner.control.clone();
    let terminal_gate = owner.gate.clone();
    let computed = tokio::task::spawn_blocking(move || {
        let mut appended = 0_u64;
        let mut last = 0_u64;
        let outcome = owner.runtime.infer(&owner.identity, &owner.job_id, &owner.base, &owner.control, &mut |completed, total| {
            let now_ms = owner.runtime.fresh_now_ms()?;
            if completed > last && appended < super::schema::PROGRESS_MAX_CURSOR {
                owner.runtime.ledger().heartbeat(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, completed, total, now_ms)?;
                appended += 1;
                last = completed;
            } else {
                owner.runtime.ledger().renew_claim(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, now_ms)?;
            }
            Ok(())
        });
        (owner, outcome)
    })
    .await;
    match computed {
        Ok((owner, outcome)) => finish_retained_gis_map_job(owner, outcome).await,
        Err(_) => {
            let Ok(now_ms) = terminal_runtime.fresh_now_ms() else {
                return;
            };
            let _guard = terminal_gate.lock().await;
            if terminal_control.checkpoint(terminal_control.progress().0).is_err() {
                let _ = terminal_runtime.ledger().cancel_run(&terminal_job_id, &reader(&terminal_identity), terminal_claim.run_epoch, now_ms);
            } else {
                let _ = terminal_runtime.ledger().fail_run(&terminal_job_id, &reader(&terminal_identity), terminal_claim.run_epoch, now_ms);
            }
        }
    }
}

/// 📥️ Accepts one closed client intent, claims it, runs the frozen service, and offers one proposal.
pub async fn submit_gis_map_job(context: InferenceRouteContextV1<'_>, body: &[u8]) -> Result<InferenceJobReceiptDtoV1, InferenceRouteErrorV1> {
    let request = super::schema::InferenceRequestV1::decode(body)?;
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    runtime.reap_finished_operations().await?;
    let gate = runtime.document_gate(&context.scope)?;
    let control = Arc::new(InferenceOperationControlV1::new(request.lifetime_ms, WORK_UNIT_LIMIT)?);
    let base_control = InferenceMapBaseControlV1 { deadline_ms: context.now_ms.saturating_add(super::schema::JOB_MAX_LIFETIME_MS), now_ms: context.now_ms, control: control.clone() };
    let (receipt, identity, admission) = {
        let _guard = gate.lock().await;
        let descriptor = context.directory.get_document_descriptor(&context.scope).await.map_err(|_| InferenceRouteErrorV1::Storage)?.ok_or(InferenceRouteErrorV1::NotFound)?;
        let base = map_base(context.rebootstrap, &context.scope, base_control.deadline_ms, &base_control).await?;
        let identity = super::catalog::identity_from_frozen_binding(
            runtime.binding(),
            super::catalog::InferenceIdentitySourceV1 {
                request,
                scope: &context.scope,
                descriptor: &descriptor,
                session: &session,
                frontier: &base.frontier,
                materialized_input: &base.pack,
                now_ms: i64::try_from(context.now_ms).map_err(|_| InferenceRouteErrorV1::Bounds)?,
            },
            &control,
        )
        .await?;
        runtime.compare_frozen(&identity, &context.scope, &base)?;
        super::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
        let receipt = runtime.ledger().accept(&identity, &base.pack, context.now_ms)?;
        let runtime_owner = runtime.clone();
        let directory = context.directory.clone();
        let rebootstrap = context.rebootstrap.clone();
        let owner_gate = gate.clone();
        let scope = context.scope.clone();
        let job_id = receipt.job_id.clone();
        let expires_at_ms = receipt.expires_at_ms;
        let owner_identity = identity.clone();
        let owner_control = control.clone();
        let admission = runtime.install_operation(&receipt, &identity, context.now_ms, control.clone(), move |claim, ready, permit| {
            let owner = RetainedInferenceRunOwnerV1 { runtime: runtime_owner, directory, rebootstrap, gate: owner_gate, scope, identity: owner_identity, job_id, expires_at_ms, claim, base, control: owner_control, _permit: permit };
            async move { drive_retained_gis_map_job(owner, ready).await }
        });
        let admission = match admission {
            Ok(admission) => admission,
            Err(error) => {
                let now_ms = runtime.fresh_now_ms().unwrap_or(context.now_ms);
                let _ = runtime.ledger().fail(&receipt.job_id, &reader(&identity), now_ms);
                return Err(error);
            }
        };
        (receipt, identity, admission)
    };
    let now_ms = runtime.fresh_now_ms()?;
    let response = owner_page_receipt(runtime, &receipt.job_id, &identity, receipt.expires_at_ms, now_ms);
    match admission {
        InferenceOperationAdmissionV1::Installed(start) => {
            if response.is_err() {
                control.cancel();
            }
            let _ = start.send(());
        }
        InferenceOperationAdmissionV1::Existing => {}
        InferenceOperationAdmissionV1::Unclaimed => {
            if response.as_ref().is_ok_and(|receipt| receipt.state == super::schema::InferenceJobStateV1::Running) {
                return Err(InferenceRouteErrorV1::Unavailable);
            }
        }
    }
    response
}

fn owner_page_receipt(runtime: &Arc<HubInferenceRuntimeV1>, job_id: &str, identity: &InferenceIdentityV1, expires_at_ms: u64, now_ms: u64) -> Result<InferenceJobReceiptDtoV1, InferenceRouteErrorV1> {
    let page = runtime.ledger().events(job_id, &reader(identity), 0, now_ms)?;
    Ok(InferenceJobReceiptDtoV1 { schema: "semio.hub.inference-job-receipt/v1", job_id: job_id.to_owned(), state: page.state, proposal_state: page.proposal_state, proposal_hash: page.proposal_hash, cursor: page.next_cursor, expires_at_ms })
}

/// 📤️ Returns the owner-private bounded event page and marks a drifted offer stale to its owner.
pub async fn read_gis_map_job_events(context: InferenceRouteContextV1<'_>, job_id: &str, after: u64) -> Result<InferenceEventPageDtoV1, InferenceRouteErrorV1> {
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    runtime.reap_finished_operations().await?;
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let control = Arc::new(InferenceOperationControlV1::new(super::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    let identity = runtime.ledger().identity_of(job_id, &session_reader(&session, &context.scope))?;
    super::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    let page = runtime.ledger().events(job_id, &reader(&identity), after, context.now_ms)?;
    let base_control = InferenceMapBaseControlV1 { deadline_ms: context.now_ms.saturating_add(super::schema::JOB_MAX_LIFETIME_MS), now_ms: context.now_ms, control };
    let current = map_base(context.rebootstrap, &context.scope, base_control.deadline_ms, &base_control).await?;
    let stale = runtime.compare_frozen(&identity, &context.scope, &current).is_err();
    let preview = if !stale && !page.cancel_requested && page.state == super::schema::InferenceJobStateV1::Succeeded && page.proposal_state == super::schema::InferenceProposalStateV1::Offered {
        let proposal_hash = page.proposal_hash.as_deref().ok_or(InferenceRouteErrorV1::Conflict)?;
        let view = runtime.ledger().read(job_id, &reader(&identity), context.now_ms)?;
        Some(gis_map_inference_preview(job_id, proposal_hash, view.proposal.as_slice())?)
    } else {
        None
    };
    Ok(InferenceEventPageDtoV1 {
        schema: "semio.hub.inference-job-events/v1",
        job_id: job_id.to_owned(),
        state: page.state,
        proposal_state: page.proposal_state,
        cancel_requested: page.cancel_requested,
        stale,
        proposal_hash: page.proposal_hash,
        preview,
        events: page.events.into_iter().map(|row| InferenceEventDtoV1 { ordinal: row.ordinal, kind: row.kind, at_ms: row.at_ms }).collect(),
        progress: page.progress.into_iter().map(|row| InferenceProgressDtoV1 { cursor: row.cursor, run_epoch: row.run_epoch, completed: row.completed, total: row.total, at_ms: row.at_ms }).collect(),
        next_cursor: page.next_cursor,
    })
}

/// 🛑️ Records the owner's durable cancel request and interrupts its retained bounded work.
pub async fn cancel_gis_map_job(context: InferenceRouteContextV1<'_>, job_id: &str) -> Result<InferenceEventPageDtoV1, InferenceRouteErrorV1> {
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    runtime.reap_finished_operations().await?;
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let control = Arc::new(InferenceOperationControlV1::new(super::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    let identity = runtime.ledger().identity_of(job_id, &session_reader(&session, &context.scope))?;
    super::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    runtime.interrupt_operation(job_id, &identity.digest()?)?;
    runtime.ledger().request_cancel(job_id, &reader(&identity), context.now_ms)?;
    let page = runtime.ledger().events(job_id, &reader(&identity), 0, context.now_ms)?;
    Ok(InferenceEventPageDtoV1 {
        schema: "semio.hub.inference-job-events/v1",
        job_id: job_id.to_owned(),
        state: page.state,
        proposal_state: page.proposal_state,
        cancel_requested: page.cancel_requested,
        stale: false,
        proposal_hash: page.proposal_hash,
        preview: None,
        events: page.events.into_iter().map(|row| InferenceEventDtoV1 { ordinal: row.ordinal, kind: row.kind, at_ms: row.at_ms }).collect(),
        progress: page.progress.into_iter().map(|row| InferenceProgressDtoV1 { cursor: row.cursor, run_epoch: row.run_epoch, completed: row.completed, total: row.total, at_ms: row.at_ms }).collect(),
        next_cursor: page.next_cursor,
    })
}

/// ✅️ Explicit approval: rebuilds the typed effect server-side and hands it to the composition port.
pub async fn approve_gis_map_job(context: InferenceApprovalRouteContextV1<'_>, job_id: &str, body: &[u8]) -> Result<InferenceApprovalReceiptDtoV1, InferenceRouteErrorV1> {
    let InferenceApprovalRouteContextV1 { route: context, ingress } = context;
    let approval = super::schema::InferenceApprovalRequestV1::decode(body)?;
    if approval.job_id != job_id {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    runtime.reap_finished_operations().await?;
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let control = Arc::new(InferenceOperationControlV1::new(super::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    let identity = runtime.ledger().identity_of(&approval.job_id, &session_reader(&session, &context.scope))?;
    super::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    let view = runtime.ledger().read(&approval.job_id, &reader(&identity), context.now_ms)?;
    if view.proposal.as_slice().is_empty() || sha256(view.proposal.as_slice()) != approval.proposal_hash {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let base_control = InferenceMapBaseControlV1 { deadline_ms: context.now_ms.saturating_add(super::schema::JOB_MAX_LIFETIME_MS), now_ms: context.now_ms, control };
    let base = map_base(context.rebootstrap, &context.scope, base_control.deadline_ms, &base_control).await?;
    runtime.compare_frozen(&identity, &context.scope, &base)?;
    let command = runtime.server_stamped_command(&identity, &approval.job_id, &base, &approval.proposal_hash, context.now_ms)?;
    let prepared = runtime.ledger().prepare_approval(&approval.job_id, &identity, &approval.proposal_hash, &command, context.now_ms)?;
    let committed = runtime.commit_approval(&identity, &approval.job_id, &approval.proposal_hash, &command, &base, base_control.deadline_ms, context.now_ms, context.document_write, ingress).await?;
    Ok(InferenceApprovalReceiptDtoV1 {
        schema: "semio.hub.inference-approval-receipt/v1",
        job_id: approval.job_id,
        mutation_id: prepared.mutation_id,
        command_hash: prepared.command_hash,
        proposal_hash: prepared.proposal_hash,
        applied: committed.applied,
        undo: committed.undo,
    })
}

/// ↩️ Applies the server-retained fixed-three inverse against the exact current durable tail.
pub async fn undo_gis_map_approval(context: InferenceApprovalRouteContextV1<'_>, body: &[u8]) -> Result<directory::os_directory::GisMapApprovalUndoReceiptV1, InferenceRouteErrorV1> {
    let InferenceApprovalRouteContextV1 { route: context, ingress } = context;
    if body.is_empty() || body.len() > super::schema::REQUEST_MAX_BYTES {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let request: directory::os_directory::GisMapApprovalUndoRequestV1 = serde_json::from_slice(body).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    if !request.validate() || request.expected_current.document_id != context.scope.document_id {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let reader = session_reader(&session, &context.scope);
    let original_job_id = runtime.ledger().gis_map_approval_undo_job_id(&request.target_id, &reader)?;
    let identity = runtime.ledger().identity_of(&original_job_id, &reader)?;
    let control = Arc::new(InferenceOperationControlV1::new(super::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    super::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    if let Some(receipt) = runtime.ledger().replayed_gis_map_approval_undo(&request.target_id, &request.idempotency_key, &reader)? {
        return Ok(receipt);
    }
    let target = runtime.ledger().gis_map_approval_undo_target(&request.target_id, &reader)?;
    if target.after_frontier != request.expected_current || identity.descriptor_digest.is_empty() || target.descriptor_digest != identity.descriptor_digest {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let base_control = InferenceMapBaseControlV1 { deadline_ms: context.now_ms.saturating_add(super::schema::JOB_MAX_LIFETIME_MS), now_ms: context.now_ms, control };
    let base = map_base(context.rebootstrap, &context.scope, base_control.deadline_ms, &base_control).await?;
    let current = directory::os_directory::CheckpointPublicationFrontierV1 {
        document_id: base.frontier.document_id.clone(),
        head_edit_ordinal: base.frontier.head_edit_ordinal,
        head_edit_id: base.frontier.head_edit_id.clone(),
        last_commit_seq: base.frontier.last_commit_seq,
        chain_sha256: base.frontier.chain_hash.hex(),
    };
    if current != request.expected_current || base.descriptor_digest != identity.descriptor_digest || base.digest() != target.after_base_digest {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    super::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &base_control.control).await?;
    let prepared = runtime.server_stamped_undo_command(&target, &request.idempotency_key, &base, context.now_ms)?;
    match runtime.ledger().prepare_gis_map_approval_undo(&target, &request.idempotency_key, &prepared.operation_id, &prepared.proposal_hash, &prepared.mutation_id, &prepared.command_hash, &prepared.command)? {
        super::sqlite::GisMapApprovalUndoAdmissionV1::Replayed(receipt) => return Ok(receipt),
        super::sqlite::GisMapApprovalUndoAdmissionV1::Prepared => {}
    }
    let committed = runtime.commit_undo(&target, &request.idempotency_key, &prepared, &base, base_control.deadline_ms, context.now_ms, context.document_write, ingress).await?;
    Ok(directory::os_directory::GisMapApprovalUndoReceiptV1 {
        schema: "semio.hub.gis-map-approval-undo-receipt/v1".into(),
        target_id: target.target_id,
        original_job_id: target.original_job_id,
        mutation_id: prepared.mutation_id,
        command_hash: prepared.command_hash,
        applied: committed.applied,
        replayed: false,
        frontier: committed.frontier,
    })
}

fn session_reader<'a>(session: &'a crate::directory::model::AuthSessionRecord, scope: &'a DocumentScope) -> InferenceReaderV1<'a> {
    InferenceReaderV1 { user_id: &session.user_id, session_id: &session.id, authorization_generation: session.authorization_generation, space_id: &scope.space_id, document_id: &scope.document_id }
}
//#endregion 🛣️Routes

/// 🔑️ Renders the exact full document key every envelope and witness compares byte for byte.
pub fn document_key(scope: &DocumentScope) -> String {
    format!("v1:{}:{}:{}{}", scope.space_id.len(), scope.document_id.len(), scope.space_id, scope.document_id)
}

/// 🕰️ Reads the process clock for retained work after its request has already returned.
fn fresh_now_ms() -> Result<u64, InferenceRouteErrorV1> {
    let elapsed = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| InferenceRouteErrorV1::Storage)?;
    u64::try_from(elapsed.as_millis()).ok().filter(|value| *value <= super::schema::SAFE_INTEGER_MAX).ok_or(InferenceRouteErrorV1::Bounds)
}

/// 🧊️ Decodes one canonical actor-frontier chain, including the genesis zero sentinel.
fn parse_frontier_chain_hash(value: &str) -> Option<directory::os_directory::ArtifactHash> {
    if value.len() != 64 || value.bytes().any(|byte| !byte.is_ascii_digit() && !matches!(byte, b'a'..=b'f')) {
        return None;
    }
    let mut bytes = [0_u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).ok()?;
    }
    Some(directory::os_directory::ArtifactHash::new(bytes))
}

/// 🎭️ Renders the exact server-derived actor; a client never supplies or influences it.
pub fn approval_actor(identity: &InferenceIdentityV1) -> String {
    format!("user:{}#session:{}", identity.user_id, identity.session_id)
}

/// 🆔️ Derives the deterministic approval mutation identity from job and proposal alone.
pub fn approval_mutation_id(job_id: &str, proposal_hash: &str) -> String {
    sha256(format!("semio.hub.inference-approval-mutation/v1\0{job_id}\0{proposal_hash}").as_bytes())[..32].to_string()
}

/// 👤️ Renders the owner-private reader every read, cancel, and approval revalidates against.
pub fn reader<'a>(identity: &'a InferenceIdentityV1) -> InferenceReaderV1<'a> {
    InferenceReaderV1 { user_id: &identity.user_id, session_id: &identity.session_id, authorization_generation: identity.authorization_generation, space_id: &identity.space_id, document_id: &identity.document_id }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inference::schema::{InferenceIdentityV1 as Identity, INPUT_MAX_BYTES, PROGRESS_MAX_CURSOR};
    use std::future::Future;

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture")
    }

    fn ledger_fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json")).expect("ledger fixture")
    }

    fn identity() -> Identity {
        serde_json::from_value(ledger_fixture()["identity"].clone()).expect("accepted identity")
    }

    fn ledger() -> Arc<InferenceJobLedgerV1> {
        let path = std::env::temp_dir().join(format!("semio-gis-map-proposal-{}.sqlite", directory::os_identity::time_ordered_id()));
        Arc::new(InferenceJobLedgerV1::open(&path).expect("bounded private ledger"))
    }

    fn input(identity: &Identity) -> InferencePrivateBytesV1 {
        let bytes = ledger_fixture()["input"].as_str().expect("literal base").as_bytes().to_vec();
        assert_eq!(sha256(&bytes), identity.input_hash, "the literal base is exactly the frozen input");
        InferencePrivateBytesV1::new(bytes, INPUT_MAX_BYTES).expect("bounded base")
    }

    fn canonical_map_pack() -> InferencePrivateBytesV1 {
        use directory::ArtifactPack as _;
        let descriptor = ledger_fixture()["input"].as_str().expect("literal Map descriptor").to_owned();
        let snapshot = semio_s_plugin_gis::artifacts::gismap::schema::gis_map_document_from_descriptor_json(&descriptor);
        InferencePrivateBytesV1::new(snapshot.encode_pack(), INPUT_MAX_BYTES).expect("bounded canonical Map pack")
    }

    fn canonical_map_base(identity: &Identity) -> InferenceMapBaseV1 {
        InferenceMapBaseV1 {
            frontier: directory::os_directory::ArtifactFrontier {
                document_id: identity.document_id.clone(),
                head_edit_ordinal: identity.head_ordinal,
                head_edit_id: identity.head_edit_id.clone(),
                last_commit_seq: identity.last_commit_seq,
                chain_hash: directory::os_directory::ArtifactHash([0; 32]),
            },
            descriptor_digest: identity.descriptor_digest.clone(),
            pack: canonical_map_pack(),
        }
    }

    fn canonical_identity_and_base() -> (Identity, InferenceMapBaseV1) {
        let mut identity = identity();
        let base = canonical_map_base(&identity);
        identity.input_hash = base.digest();
        (identity, base)
    }

    fn canonical_approval(identity: &Identity, job_id: &str, base: &InferenceMapBaseV1, now_ms: u64) -> (InferencePrivateBytesV1, InferencePrivateBytesV1) {
        let (snapshot, inference) = deterministic_map_inference(base, job_id).expect("real GIS Map inference");
        let work = inference.create_region_group_work(&snapshot, job_id).expect("server-derived fixed-three CreateRegion work");
        let proposal_bytes = directory::os_pack::json::to_json_string(&work.parent).into_bytes();
        let proposal = InferencePrivateBytesV1::new(proposal_bytes, PROPOSAL_MAX_BYTES).expect("bounded canonical proposal");
        let proposal_hash = sha256(proposal.as_slice());
        let inverse = directory::os_pack::json::to_json_string(&work.parent_inverse).into_bytes();
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let mutation_id = approval_mutation_id(job_id, &proposal_hash);
        let document_id = document_key(&scope);
        let actor = approval_actor(identity);
        let command = encode_server_stamped_command_v1(&CanonicalInferenceCommandPartsV1 {
            mutation_id: &mutation_id,
            document_id: &document_id,
            actor: &actor,
            diff_schema: GIS_DOCUMENT_SCHEMA,
            diff_payload: proposal.as_slice(),
            inverse_schema: GIS_DOCUMENT_SCHEMA,
            inverse_payload: &inverse,
            timestamp: protocol::HybridLogicalTimestamp { actor: 1, physical_ms: now_ms, logical: 0 },
        })
        .expect("server-stamped approval command");
        (proposal, InferencePrivateBytesV1::new(command, super::super::command::COMMAND_MAX_BYTES).expect("bounded approval command"))
    }

    struct TestApprovalIngressAuthorityV1 {
        scope: DocumentScope,
        user_id: String,
        session_id: String,
        authorization_generation: u64,
        released: Option<Arc<std::sync::atomic::AtomicBool>>,
    }

    impl GisMapApprovalIngressAuthorityV1 for TestApprovalIngressAuthorityV1 {
        fn scope(&self) -> &DocumentScope {
            &self.scope
        }

        fn user_id(&self) -> &str {
            &self.user_id
        }

        fn session_id(&self) -> &str {
            &self.session_id
        }

        fn authorization_generation(&self) -> u64 {
            self.authorization_generation
        }
    }

    impl Drop for TestApprovalIngressAuthorityV1 {
        fn drop(&mut self) {
            if let Some(released) = &self.released {
                released.store(true, Ordering::Release);
            }
        }
    }

    fn approval_ingress(identity: &Identity) -> Arc<dyn GisMapApprovalIngressAuthorityV1> {
        tracked_approval_ingress(identity, None)
    }

    fn tracked_approval_ingress(identity: &Identity, released: Option<Arc<std::sync::atomic::AtomicBool>>) -> Arc<dyn GisMapApprovalIngressAuthorityV1> {
        Arc::new(TestApprovalIngressAuthorityV1 {
            scope: DocumentScope::new(identity.space_id.clone(), identity.document_id.clone()),
            user_id: identity.user_id.clone(),
            session_id: identity.session_id.clone(),
            authorization_generation: identity.authorization_generation,
            released,
        })
    }

    struct OrderedApprovalCheckpointPublisherV1 {
        ledger: Arc<InferenceJobLedgerV1>,
        identity: Identity,
        job_id: String,
        attempts: std::sync::atomic::AtomicUsize,
        order: Arc<std::sync::Mutex<Vec<&'static str>>>,
        pause: Option<(Arc<tokio::sync::Notify>, Arc<tokio::sync::Notify>)>,
    }

    impl GisMapApprovalCheckpointPublisherV1 for OrderedApprovalCheckpointPublisherV1 {
        fn publish<'a>(
            &'a self,
            request: GisMapApprovalCheckpointRequestV1,
            document_write: Arc<GisMapDocumentWriteAuthorityV1>,
            _attempt_lifetime_ms: u64,
            decision_now_ms: u64,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<directory::os_directory::PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
            Box::pin(async move {
                let snapshot = <semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(&request.pair.pack).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
                let expected_region = format!("inference-{}", self.job_id);
                let attempt = self.attempts.fetch_add(1, Ordering::AcqRel);
                let has_expected_region = snapshot
                    .regions
                    .iter()
                    .any(|region| region.id == expected_region && region.data.get("id").and_then(|value| value.as_str()) == Some(expected_region.as_str()) && region.data.get("kind").and_then(|value| value.as_str()) == Some("inference-bounds"));
                if has_expected_region != (attempt < 2) {
                    return Err(GisMapApprovalCommitErrorV1::Conflict);
                }
                let view = self.ledger.read(&self.job_id, &reader(&self.identity), decision_now_ms).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
                let expected_proposal_state = if attempt < 2 { super::super::schema::InferenceProposalStateV1::Offered } else { super::super::schema::InferenceProposalStateV1::Approved };
                if view.proposal_state != expected_proposal_state || document_write.gate.try_lock().is_ok() {
                    return Err(GisMapApprovalCommitErrorV1::Conflict);
                }
                self.order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("public-checkpoint-attempt");
                if attempt == 0 {
                    return Err(GisMapApprovalCommitErrorV1::Storage);
                }
                if attempt == 1 {
                    if let Some((entered, release)) = &self.pause {
                        entered.notify_one();
                        release.notified().await;
                    }
                }
                let frontier = RetainedGisMapApprovalCommitterV1::publication_frontier(&request.scope, &request.actor_snapshot).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
                let pack_hash = directory::os_directory::ArtifactHash::parse_hex(&sha256(&request.pair.pack)).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
                let spr_hash = directory::os_directory::ArtifactHash::parse_hex(&sha256(&request.pair.spr)).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
                self.order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("public-checkpoint-ack");
                Ok(directory::os_directory::PublishedArtifactCheckpoint {
                    scope: request.scope,
                    checkpoint_id: directory::os_directory::ArtifactHash([0x45; 32]),
                    parent_checkpoint_id: None,
                    descriptor_digest_v1: directory::os_directory::ArtifactHash::parse_hex(&request.descriptor_digest).ok_or(GisMapApprovalCommitErrorV1::Storage)?,
                    baseline_frontier: frontier,
                    pack: directory::os_directory::PublishedArtifactBlob { sha256: pack_hash, byte_length: u64::try_from(request.pair.pack.len()).map_err(|_| GisMapApprovalCommitErrorV1::Capacity)? },
                    spr: directory::os_directory::PublishedArtifactBlob { sha256: spr_hash, byte_length: u64::try_from(request.pair.spr.len()).map_err(|_| GisMapApprovalCommitErrorV1::Capacity)? },
                    aggregate_sha256: directory::os_directory::ArtifactHash([0x46; 32]),
                    published_at_ms: decision_now_ms,
                })
            })
        }

        fn checkpoint_applied(&self, checkpoint: &directory::os_directory::PublishedArtifactCheckpoint) -> Result<(), GisMapApprovalCommitErrorV1> {
            let view = self.ledger.read(&self.job_id, &reader(&self.identity), checkpoint.published_at_ms).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            if view.proposal_state != super::super::schema::InferenceProposalStateV1::Approved {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            self.order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("peer-rebootstrap");
            Ok(())
        }
    }

    enum AbandonedApprovalPhaseV1 {
        Preflight,
        Assembly,
        Journal,
        Committed,
    }

    struct ApprovalPollWakeV1 {
        ready: std::sync::atomic::AtomicBool,
    }

    impl std::task::Wake for ApprovalPollWakeV1 {
        fn wake(self: Arc<Self>) {
            self.ready.store(true, Ordering::Release);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.ready.store(true, Ordering::Release);
        }
    }

    async fn poll_approval_to_phase<F>(future: &mut std::pin::Pin<Box<F>>, committer: &RetainedGisMapApprovalCommitterV1, key: &str, phase: AbandonedApprovalPhaseV1)
    where
        F: std::future::Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + ?Sized,
    {
        let wake = Arc::new(ApprovalPollWakeV1 { ready: std::sync::atomic::AtomicBool::new(true) });
        let waker = std::task::Waker::from(wake.clone());
        for _ in 0..4_096 {
            while !wake.ready.swap(false, Ordering::AcqRel) {
                tokio::task::yield_now().await;
            }
            let pending = {
                let mut context = std::task::Context::from_waker(&waker);
                matches!(future.as_mut().poll(&mut context), std::task::Poll::Pending)
            };
            assert!(pending, "approval must remain retained before its cancellation phase");
            let reached = {
                let documents = committer.documents.lock().await;
                match (documents.get(key), &phase) {
                    (Some(RetainedGisMapDocumentStateV1::Ready { pending: Some(_), document_write: Some(_), .. }), AbandonedApprovalPhaseV1::Preflight)
                    | (Some(RetainedGisMapDocumentStateV1::Assembly { .. }), AbandonedApprovalPhaseV1::Assembly)
                    | (Some(RetainedGisMapDocumentStateV1::Journal { receipt: None, .. }), AbandonedApprovalPhaseV1::Journal)
                    | (Some(RetainedGisMapDocumentStateV1::Journal { receipt: Some(_), .. }), AbandonedApprovalPhaseV1::Committed) => true,
                    _ => false,
                }
            };
            if reached {
                return;
            }
        }
        panic!("approval did not reach its exact retained cancellation phase");
    }

    async fn wait_for_abandoned_approval_handoff(committer: &RetainedGisMapApprovalCommitterV1, key: &str, gate: &Arc<tokio::sync::Mutex<()>>, ingress_released: &std::sync::atomic::AtomicBool) {
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                let ready = {
                    let documents = committer.documents.lock().await;
                    matches!(documents.get(key), Some(RetainedGisMapDocumentStateV1::Ready { pending: None, document_write: None, .. }))
                };
                let maintenance_idle = !committer.maintenance_owns_document(key);
                if ready && maintenance_idle && gate.try_lock().is_ok() && ingress_released.load(Ordering::Acquire) {
                    return;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("abandoned approval returns every Store, ingress and document writer");
    }

    async fn wait_for_abandoned_prepared_handoff(
        committer: &RetainedGisMapApprovalCommitterV1,
        ledger: &InferenceJobLedgerV1,
        identity: &Identity,
        job_id: &str,
        key: &str,
        gate: &Arc<tokio::sync::Mutex<()>>,
        ingress_released: &std::sync::atomic::AtomicBool,
    ) {
        let maintenance_key = format!("prepared:{job_id}");
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                let document_absent = !committer.documents.lock().await.contains_key(key);
                let maintenance_idle = !committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains(&maintenance_key);
                let outbox_abandoned = ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).is_ok_and(|page| page.rows.is_empty());
                if document_absent && maintenance_idle && outbox_abandoned && gate.try_lock().is_ok() && ingress_released.load(Ordering::Acquire) {
                    return;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("unpolled or rejected preflight returns its exact ingress and prepared outbox");
    }

    async fn resume_parked_cleanup(committer: &RetainedGisMapApprovalCommitterV1) {
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                committer.resume_parked_requests();
                committer.resume_parked_prepared();
                let maintenance_is_empty = committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty();
                let cleanup_is_empty = committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty();
                if maintenance_is_empty && cleanup_is_empty {
                    return;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("fresh runtime drains every parked cleanup owner");
    }

    #[test]
    fn gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation() {
        let fixture = fixture();
        let identity = identity();
        let ledger = ledger();
        let owner = reader(&identity);
        let receipt = ledger.accept(&identity, &input(&identity), 1_000).expect("accepted job");
        assert_eq!(ledger.accept(&identity, &input(&identity), 1_001).expect("scoped idempotency"), receipt);
        let claim = ledger.start(&receipt.job_id, &identity, 1_002).expect("claim").expect("first owned epoch");
        assert_eq!(claim.run_epoch, 1);
        assert_eq!(ledger.start(&receipt.job_id, &identity, 1_003).expect("second claim"), None, "a live lease is never stolen");
        for step in 1..=4_u64 {
            let now_ms = if step == 4 { 30_000 } else { 1_003 + step };
            assert_eq!(ledger.progress(&receipt.job_id, &owner, claim.run_epoch, step, WORK_UNIT_LIMIT, now_ms).expect("progress heartbeat"), step);
        }
        assert_eq!(ledger.start(&receipt.job_id, &identity, 45_000).expect("heartbeat-protected claim"), None, "a fresh checkpoint lease cannot be stolen after its original lease elapsed");
        assert_eq!(ledger.progress(&receipt.job_id, &owner, claim.run_epoch + 1, 5, WORK_UNIT_LIMIT, 45_001), Err(InferenceErrorV1::Conflict), "a foreign epoch cannot append progress");
        assert_eq!(ledger.progress(&receipt.job_id, &owner, claim.run_epoch, 3, WORK_UNIT_LIMIT, 45_002), Err(InferenceErrorV1::Conflict), "progress never regresses");
        let page = ledger.events(&receipt.job_id, &owner, 0, 45_003).expect("owner page");
        assert_eq!(page.progress.iter().map(|row| row.cursor).collect::<Vec<_>>(), vec![1, 2, 3, 4]);
        assert_eq!(page.next_cursor, 4);
        assert!(page.progress.len() <= super::super::schema::EVENT_PAGE_MAX_ITEMS && page.events.len() <= super::super::schema::EVENT_PAGE_MAX_ITEMS);
        assert_eq!(ledger.events(&receipt.job_id, &owner, 4, 45_004).expect("tail page").progress.len(), 0);
        assert_eq!(ledger.events(&receipt.job_id, &owner, PROGRESS_MAX_CURSOR + 1, 45_004).err(), Some(InferenceErrorV1::Bounds));
        assert!(ledger.request_cancel(&receipt.job_id, &owner, 45_005).expect("durable cancel request"));
        let cancelled = ledger.events(&receipt.job_id, &owner, 0, 45_006).expect("cancelled page");
        assert!(cancelled.cancel_requested);
        assert_eq!(
            cancelled.events.iter().map(|row| (row.ordinal, row.kind.as_str())).collect::<Vec<_>>(),
            fixture["cancelLifecycle"].as_array().expect("cancel trace").iter().map(|row| (row["ordinal"].as_u64().expect("ordinal"), row["kind"].as_str().expect("kind"))).collect::<Vec<_>>()
        );
        let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
        let proposal = InferencePrivateBytesV1::new(b"bounded-proposal".to_vec(), PROPOSAL_MAX_BYTES).expect("bounded proposal");
        assert_eq!(ledger.succeed(&receipt.job_id, &identity, claim.run_epoch, &result, &proposal, 45_007), Ok(false), "a retired job never publishes a late offer");
        assert!(ledger.request_cancel(&receipt.job_id, &owner, 45_008).is_ok(), "cancellation is idempotent");
    }

    #[test]
    fn gis_map_proposal_is_private_to_its_original_author_owner() {
        let identity = identity();
        let ledger = ledger();
        let owner = reader(&identity);
        let receipt = ledger.accept(&identity, &input(&identity), 1_000).expect("accepted job");
        ledger.start(&receipt.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
        let peer_user = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string();
        let foreign = [
            ("peer-author-same-space", InferenceReaderV1 { user_id: peer_user.as_str(), ..reader(&identity) }),
            ("cross-space-author", InferenceReaderV1 { space_id: peer_user.as_str(), ..reader(&identity) }),
            ("wrong-document", InferenceReaderV1 { document_id: peer_user.as_str(), ..reader(&identity) }),
            ("stale-session", InferenceReaderV1 { session_id: peer_user.as_str(), ..reader(&identity) }),
            ("stale-authorization-generation", InferenceReaderV1 { authorization_generation: identity.authorization_generation + 1, ..reader(&identity) }),
        ];
        for (role, candidate) in &foreign {
            assert_eq!(ledger.identity_of(&receipt.job_id, candidate).err(), Some(InferenceErrorV1::Denied), "{role} read the frozen identity");
            assert_eq!(ledger.events(&receipt.job_id, candidate, 0, 1_002).err(), Some(InferenceErrorV1::Denied), "{role} read the private stream");
            assert_eq!(ledger.read(&receipt.job_id, candidate, 1_002).err(), Some(InferenceErrorV1::Denied), "{role} read the private proposal");
            assert_eq!(ledger.request_cancel(&receipt.job_id, candidate, 1_002), Err(InferenceErrorV1::Denied), "{role} cancelled another owner's job");
            assert_eq!(ledger.progress(&receipt.job_id, candidate, 1, 1, WORK_UNIT_LIMIT, 1_002), Err(InferenceErrorV1::Denied), "{role} appended progress");
        }
        assert!(ledger.events(&receipt.job_id, &owner, 0, 1_003).is_ok(), "the original owner still reads its own stream");
    }

    #[tokio::test]
    async fn gis_map_approval_fails_closed_without_a_composition_transaction_and_never_auto_applies() {
        let fixture = fixture();
        let identity = identity();
        let ledger = ledger();
        let owner = reader(&identity);
        let receipt = ledger.accept(&identity, &input(&identity), 1_000).expect("accepted job");
        let claim = ledger.start(&receipt.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
        let ledger_corpus = ledger_fixture();
        let outbox = &ledger_corpus["outbox"];
        let proposal = InferencePrivateBytesV1::new(outbox["proposal"].as_str().expect("literal proposal").as_bytes().to_vec(), PROPOSAL_MAX_BYTES).expect("bounded proposal");
        let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
        assert!(ledger.succeed(&receipt.job_id, &identity, claim.run_epoch, &result, &proposal, 1_002).expect("offer"));
        let hex = outbox["commandHex"].as_str().expect("literal command");
        let command = InferencePrivateBytesV1::new((0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex byte")).collect(), super::super::command::COMMAND_MAX_BYTES).expect("bounded command");
        let proposal_hash = sha256(proposal.as_slice());
        let prepared = ledger.prepare_approval(&receipt.job_id, &identity, &proposal_hash, &command, 1_003).expect("prepared outbox");
        assert_eq!(prepared.mutation_id, approval_mutation_id(&receipt.job_id, &proposal_hash));
        assert_eq!(prepared.mutation_id, outbox["mutationId"].as_str().expect("literal mutation"));
        let again = ledger.prepare_approval(&receipt.job_id, &identity, &proposal_hash, &command, 1_004).expect("duplicate approval");
        assert_eq!((again.mutation_id, again.command_hash, again.prepared_at_ms), (prepared.mutation_id.clone(), prepared.command_hash.clone(), prepared.prepared_at_ms), "a duplicate approval reconciles to exactly one prepared envelope");
        let base = canonical_map_base(&identity);
        let committer: Arc<dyn GisMapApprovalCommitterV1> = Arc::new(UnavailableGisMapApprovalCommitterV1);
        for attempt in 0..2 {
            assert!(
                matches!(
                    commit_prepared_approval(&committer, &identity, &receipt.job_id, &proposal_hash, &command, &base, 60_000, 1_005 + attempt, Arc::new(tokio::sync::Mutex::new(())), approval_ingress(&identity),).await,
                    Err(InferenceRouteErrorV1::CommitUnavailable)
                ),
                "no composition transaction is registered, so approval must fail closed"
            );
        }
        assert_eq!(InferenceRouteErrorV1::CommitUnavailable.code(), "approval.commit-unavailable");
        assert_eq!(InferenceRouteErrorV1::CommitUnavailable.status(), 503);
        let view = ledger.read(&receipt.job_id, &owner, 1_007).expect("owner view");
        assert_eq!(serde_json::to_value(view.proposal_state).expect("proposal state"), "offered", "a refused publication never marks the proposal approved");
        let page = ledger.events(&receipt.job_id, &owner, 0, 1_008).expect("owner page");
        assert_eq!(
            page.events.iter().map(|row| (row.ordinal, row.kind.as_str())).collect::<Vec<_>>(),
            fixture["lifecycle"].as_array().expect("lifecycle").iter().take(4).map(|row| (row["ordinal"].as_u64().expect("ordinal"), row["kind"].as_str().expect("kind"))).collect::<Vec<_>>()
        );
        assert!(!page.events.iter().any(|row| row.kind == "approved"), "no witness, no approved event");
    }

    #[tokio::test]
    async fn gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply() {
        let (identity, base) = canonical_identity_and_base();
        let undo_fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/↩️gis-map-approval-undo-v1/🔣️.json")).expect("durable undo fixture");
        let undo_contract = &undo_fixture["genesisFirstUndo"];
        let genesis_snapshot = <semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(base.pack.as_slice()).expect("exact initial Map snapshot");
        let owner = reader(&identity);
        let ledger = ledger();
        let accepted = ledger.accept(&identity, &base.pack, 1_000).expect("accepted job");
        let claim = ledger.start(&accepted.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
        let (proposal, command) = canonical_approval(&identity, &accepted.job_id, &base, 1_004);
        let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
        assert!(ledger.succeed(&accepted.job_id, &identity, claim.run_epoch, &result, &proposal, 1_002).expect("offer"));
        let proposal_hash = sha256(proposal.as_slice());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_003).expect("prepared outbox");

        let pool = Arc::new(db::semio_framework_async::WorkerPool::new(db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let memory = db::storage::MemoryStorage::new(pool.clone()).await.expect("memory storage");
        let backend = Arc::new(db::storage::DbBackend::Memory(memory));
        let database = Arc::new(db::Database::open(pool.clone(), db::DbConfig::for_profile(db::Profile::Test), backend.clone()).await.expect("database"));
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        assert_eq!(undo_contract["initialShape"], "scope-bound-zero-frontier");
        assert!(base.frontier.is_genesis_for(&scope), "the first approval begins at the exact scope-bound zero frontier");
        assert_eq!(undo_contract["syntheticGenesisEdit"], false, "the fixture never invents a bootstrap edit");
        let document = protocol::ArtifactId(document_key(&scope));
        let handle = database.ensure_document(&document).await.expect("document actor");
        let initial = handle.checkpoint_publication_snapshot().await.expect("initial actor frontier");
        assert_eq!((initial.authority_generation, initial.frontier.head_seq, initial.frontier.commit_seq, initial.head_edit_id), (0, 0, 0, None));
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));
        let publisher_entered = Arc::new(tokio::sync::Notify::new());
        let publisher_release = Arc::new(tokio::sync::Notify::new());
        let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> = Arc::new(OrderedApprovalCheckpointPublisherV1 {
            ledger: ledger.clone(),
            identity: identity.clone(),
            job_id: accepted.job_id.clone(),
            attempts: std::sync::atomic::AtomicUsize::new(0),
            order: order.clone(),
            pause: Some((publisher_entered.clone(), publisher_release.clone())),
        });
        let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
        let committer_port: Arc<dyn GisMapApprovalCommitterV1> = committer.clone();
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        let composed_children = base.composed_children().expect("canonical fixed-three children");
        let expected_mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
        let expected_command_hash = sha256(command.as_slice());
        let approval_actor = approval_actor(&identity);
        let pure_ingress = approval_ingress(&identity);
        let pure_request = GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &approval_actor,
            mutation_id: &expected_mutation_id,
            command_hash: &expected_command_hash,
            job_id: &accepted.job_id,
            proposal_hash: &proposal_hash,
            command: command.as_slice(),
            base: &base,
            base_frontier: &base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_004,
            document_write: gate.clone(),
            ingress: pure_ingress,
        };
        assert!(committer.validate_prepared_request(&pure_request).is_ok(), "the exact ledger/base tuple passes immutable admission before any owner reservation");
        assert!(RetainedGisMapApprovalCommitterV1::preflight(&pure_request).is_ok(), "the exact fixed-three command passes pure GIS preflight before Store assembly");
        let wrong_ingress: Arc<dyn GisMapApprovalIngressAuthorityV1> =
            Arc::new(TestApprovalIngressAuthorityV1 { scope: scope.clone(), user_id: identity.user_id.clone(), session_id: identity.session_id.clone(), authorization_generation: identity.authorization_generation + 1, released: None });
        assert!(
            matches!(commit_prepared_approval(&committer_port, &identity, &accepted.job_id, &proposal_hash, &command, &base, 60_000, 1_004, gate.clone(), wrong_ingress).await, Err(InferenceRouteErrorV1::Denied)),
            "a substituted Hub ingress generation is rejected before document-write acquisition",
        );
        assert!(gate.try_lock().is_ok(), "rejected ingress never acquires the document writer");
        let first_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let first_ingress = tracked_approval_ingress(&identity, Some(first_ingress_released.clone()));
        let first = commit_prepared_approval(&committer_port, &identity, &accepted.job_id, &proposal_hash, &command, &base, 60_000, 1_004, gate.clone(), first_ingress).await;
        assert!(matches!(first, Err(InferenceRouteErrorV1::Storage)), "public checkpoint refusal remains a retained nonterminal publication");
        assert!(first_ingress_released.load(Ordering::Acquire), "a failed request releases its live Hub ingress authority after the durable decision is retained");
        assert!(gate.try_lock().is_err(), "the exact document write authority remains held after publication refusal");
        assert_eq!(ledger.read(&accepted.job_id, &owner, 1_005).expect("retained proposal").proposal_state, super::super::schema::InferenceProposalStateV1::Offered);
        let actor = handle.checkpoint_publication_snapshot().await.expect("committed Event actor frontier");
        assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq), (1, 1));
        let expected_mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
        assert_eq!(actor.head_edit_id.as_ref().map(|value| value.0.as_str()), Some(expected_mutation_id.as_str()));
        let (parent_revision, parent_reference, drawing_reference, drawing_owner, value_reference, value_owner) = {
            use directory::os_store::SpaceMember;
            let documents = committer.documents.lock().await;
            match documents.get(&document_key(&scope)) {
                Some(RetainedGisMapDocumentStateV1::Publishing { owners, .. }) => {
                    let parent = owners.parent.as_ref().expect("retained parent Store");
                    let drawing = owners.drawing.as_ref().expect("retained drawing Store");
                    let value = owners.value.as_ref().expect("retained value Store");
                    (
                        parent.content_revision_now(),
                        parent.artifact_ref().expect("parent Store identity"),
                        drawing.artifact_ref().expect("drawing Store identity"),
                        drawing.owner_ref().expect("drawing Store owner"),
                        value.artifact_ref().expect("value Store identity"),
                        value.owner_ref().expect("value Store owner"),
                    )
                }
                _ => panic!("publisher refusal retains the verified publication owner"),
            }
        };
        let expected_parent = directory::os_io::ArtifactRef { artifact_id: document_key(&scope), dialect: directory::os_io::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() } };
        assert_eq!(parent_reference, expected_parent);
        assert_eq!(drawing_reference.to_uri(), "gismap-drawing!s.stdio.semio@v1/drawing");
        assert_eq!(drawing_owner, directory::os_store::OwnerRef { parent: expected_parent.clone(), slot: "drawing".into(), child_id: "gismap-drawing".into() });
        assert_eq!(value_reference.to_uri(), "gismap-value!s.stdio.semio@v1/value");
        assert_eq!(value_owner, directory::os_store::OwnerRef { parent: expected_parent, slot: "value".into(), child_id: "gismap-value".into() });
        assert_ne!(actor.frontier.chain_hash, parent_revision, "the actor WAL chain and Store content revision are intentionally distinct hash domains");

        let retry_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let retry_ingress = tracked_approval_ingress(&identity, Some(retry_ingress_released.clone()));
        let mut retry = committer.commit(GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &approval_actor,
            mutation_id: &expected_mutation_id,
            command_hash: &expected_command_hash,
            job_id: &accepted.job_id,
            proposal_hash: &proposal_hash,
            command: command.as_slice(),
            base: &base,
            base_frontier: &base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_006,
            document_write: gate.clone(),
            ingress: retry_ingress,
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            tokio::select! {
                _ = publisher_entered.notified() => {}
                _ = &mut retry => panic!("fresh retry completed before the retained publisher pause"),
            }
        })
        .await
        .expect("the sole retained driver reaches the paused publisher");
        let mut close = Box::pin(committer.close());
        let close_wake = Arc::new(ApprovalPollWakeV1 { ready: std::sync::atomic::AtomicBool::new(true) });
        let close_waker = std::task::Waker::from(close_wake);
        let mut close_context = std::task::Context::from_waker(&close_waker);
        assert!(matches!(close.as_mut().poll(&mut close_context), std::task::Poll::Pending), "close joins the active post-witness driver instead of starting another verifier or publisher");
        assert_eq!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(), ["public-checkpoint-attempt", "public-checkpoint-attempt"]);
        drop(close);
        publisher_release.notify_one();
        let terminal = tokio::time::timeout(std::time::Duration::from_secs(5), &mut retry).await.expect("fresh retry observes retained publication").expect("fresh request joins the one publication");
        assert!(!terminal.applied, "the joining request observes the already-applied terminal state");
        assert_eq!(terminal.document_generation, 0, "the committed witness retains the live initial actor generation");
        assert_eq!(terminal.frontier.head_edit_ordinal, base.frontier.head_edit_ordinal + undo_contract["firstCommandOrdinalDelta"].as_u64().expect("first command delta"), "the first real command is the first history edit",);
        drop(retry);
        let approved = ledger.read(&accepted.job_id, &owner, 1_007).expect("approved proposal");
        assert_eq!(approved.proposal_state, super::super::schema::InferenceProposalStateV1::Approved);
        assert_eq!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(), ["public-checkpoint-attempt", "public-checkpoint-attempt", "public-checkpoint-ack", "peer-rebootstrap"],);
        assert!(gate.try_lock().is_ok(), "the document write authority releases only after public ACK and ledger apply");
        assert!(retry_ingress_released.load(Ordering::Acquire), "the fresh Hub ingress authority releases after public ACK and ledger apply");

        let after_pack = {
            use directory::os_store::SpaceMember as _;
            let documents = committer.documents.lock().await;
            let owners = match documents.get(&document_key(&scope)) {
                Some(RetainedGisMapDocumentStateV1::Published { owners, .. }) => owners,
                _ => panic!("the approved publication retains its exact three Store owners"),
            };
            semio_framework::io::resolve_ready(owners.parent.as_ref().expect("retained parent Store").snapshot_pack()).expect("approved Map snapshot").pack
        };
        let after_base = InferenceMapBaseV1 {
            frontier: ArtifactFrontier {
                document_id: terminal.frontier.document_id.clone(),
                head_edit_ordinal: terminal.frontier.head_edit_ordinal,
                head_edit_id: terminal.frontier.head_edit_id.clone(),
                last_commit_seq: terminal.frontier.last_commit_seq,
                chain_hash: directory::os_directory::ArtifactHash::parse_hex(&terminal.frontier.chain_sha256).expect("approved chain hash"),
            },
            descriptor_digest: identity.descriptor_digest.clone(),
            pack: InferencePrivateBytesV1::new(after_pack, INPUT_MAX_BYTES).expect("bounded approved Map pack"),
        };
        let target = ledger.gis_map_approval_undo_target(&terminal.undo.target_id, &owner).expect("owner-retained durable undo target");
        assert_eq!(target.after_frontier, terminal.frontier);
        assert_eq!(target.after_base_digest, after_base.digest());
        let original = CanonicalInferenceCommandV1::decode(target.original_command.as_slice()).expect("retained original command");
        let inverses =
            directory::os_pack::json::from_json_str::<Vec<semio_s_plugin_gis::artifacts::gismap::mutations::GisMapMutation>>(std::str::from_utf8(original.inverse_payload()).expect("canonical inverse text")).expect("canonical inverse mutations");
        assert_eq!(inverses.len(), 1, "the retained approval owns one exact parent inverse");
        let current = <semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(after_base.pack.as_slice()).expect("approved Map snapshot");
        let mut before = current.clone();
        semio_s_plugin_gis::artifacts::gismap::mutations::apply_gis_map_mutation(&mut before, &inverses[0]).expect("server inverse applies to the exact current Map");
        use directory::Inference as _;
        let work = semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&before)
            .create_region_group_work(&before, &target.original_job_id)
            .expect("server reconstructs the original fixed-three work");
        let undo_diff = directory::os_pack::json::to_json_string(&inverses[0]).into_bytes();
        let undo_inverse = directory::os_pack::json::to_json_string(&vec![work.parent]).into_bytes();
        let undo_idempotency_key = "55".repeat(16);
        let undo_operation_id = sha256(format!("semio.hub.gis-map-approval-undo-operation/v1\0{}\0{}", target.target_id, undo_idempotency_key).as_bytes())[..32].to_owned();
        let undo_proposal_hash = sha256(&undo_diff);
        let undo_mutation_id = approval_mutation_id(&undo_operation_id, &undo_proposal_hash);
        let undo_command = InferencePrivateBytesV1::new(
            encode_server_stamped_command_v1(&CanonicalInferenceCommandPartsV1 {
                mutation_id: &undo_mutation_id,
                document_id: &document_key(&scope),
                actor: &approval_actor,
                diff_schema: GIS_DOCUMENT_SCHEMA,
                diff_payload: &undo_diff,
                inverse_schema: GIS_DOCUMENT_SCHEMA,
                inverse_payload: &undo_inverse,
                timestamp: protocol::HybridLogicalTimestamp { actor: 1, physical_ms: 1_010, logical: 0 },
            })
            .expect("server-stamped undo command"),
            super::super::command::COMMAND_MAX_BYTES,
        )
        .expect("bounded undo command");
        let undo_command_hash = sha256(undo_command.as_slice());
        assert!(matches!(
            ledger.prepare_gis_map_approval_undo(&target, &undo_idempotency_key, &undo_operation_id, &undo_proposal_hash, &undo_mutation_id, &undo_command_hash, &undo_command,),
            Ok(super::super::sqlite::GisMapApprovalUndoAdmissionV1::Prepared)
        ));
        let undo_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let undo_ingress = tracked_approval_ingress(&identity, Some(undo_ingress_released.clone()));
        let undo_receipt = committer_port
            .undo(GisMapApprovalUndoCommitRequestV1 {
                composed_children: &composed_children,
                scope: &scope,
                actor: &approval_actor,
                target: &target,
                idempotency_key: &undo_idempotency_key,
                mutation_id: &undo_mutation_id,
                command_hash: &undo_command_hash,
                operation_id: &undo_operation_id,
                proposal_hash: &undo_proposal_hash,
                command: undo_command.as_slice(),
                base: &after_base,
                deadline_ms: 60_000,
                now_ms: 1_010,
                document_write: gate.clone(),
                ingress: undo_ingress,
            })
            .await
            .expect("retained durable undo");
        assert!(undo_receipt.applied, "the first exact undo applies one second durable decision");
        assert_eq!(undo_receipt.frontier.head_edit_id, undo_mutation_id);
        assert_eq!(undo_receipt.frontier.head_edit_ordinal, terminal.frontier.head_edit_ordinal + undo_contract["undoCommandOrdinalDelta"].as_u64().expect("undo command delta"),);
        assert_eq!(undo_receipt.frontier.last_commit_seq, terminal.frontier.last_commit_seq + 1);
        assert!(!undo_contract["restoresGenesisFrontier"].as_bool().expect("frontier contract"));
        assert!(undo_receipt.frontier.head_edit_ordinal > 0 && undo_receipt.frontier.last_commit_seq > 0, "undo restores content through a real second command, never by resetting lineage to genesis");
        assert!(undo_ingress_released.load(Ordering::Acquire), "durable undo releases its retained Hub ingress only after publication ACK");
        let reverted = {
            use directory::os_store::SpaceMember as _;
            let documents = committer.documents.lock().await;
            let owners = match documents.get(&document_key(&scope)) {
                Some(RetainedGisMapDocumentStateV1::Published { owners, .. }) => owners,
                _ => panic!("the undo publication retains its exact three Store owners"),
            };
            let pack = semio_framework::io::resolve_ready(owners.parent.as_ref().expect("retained reverted parent Store").snapshot_pack()).expect("reverted Map snapshot").pack;
            <semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot as directory::ArtifactPack>::decode_pack(&pack).expect("reverted Map pack")
        };
        assert!(undo_contract["restoresExactInitialSnapshot"].as_bool().expect("snapshot contract"));
        assert_eq!(reverted, genesis_snapshot, "the second durable publication restores the exact package-owned initial Map snapshot");
        assert_eq!(
            order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(),
            ["public-checkpoint-attempt", "public-checkpoint-attempt", "public-checkpoint-ack", "peer-rebootstrap", "public-checkpoint-attempt", "public-checkpoint-ack", "peer-rebootstrap"]
        );
        let replay = ledger.replayed_gis_map_approval_undo(&target.target_id, &undo_idempotency_key, &owner).expect("exact undo replay lookup").expect("committed undo replay receipt");
        assert!(replay.applied && replay.replayed);
        assert_eq!((&replay.mutation_id, &replay.command_hash, &replay.frontier), (&undo_mutation_id, &undo_command_hash, &undo_receipt.frontier));
        assert!(matches!(
            ledger.prepare_gis_map_approval_undo(
                &target,
                &undo_idempotency_key,
                &undo_operation_id,
                &undo_proposal_hash,
                &replay.mutation_id,
                &replay.command_hash,
                &undo_command,
            ),
            Ok(super::super::sqlite::GisMapApprovalUndoAdmissionV1::Replayed(receipt)) if receipt.frontier == undo_receipt.frontier
        ));

        committer.close().await.expect("committer close");
        assert_eq!(
            ledger.reconcile_committed_approval(&accepted.job_id, &terminal.witness, terminal.document_generation, &terminal.frontier, &identity.descriptor_digest, &"11".repeat(32), 1_008,).map(|value| value.applied),
            Err(InferenceErrorV1::Conflict),
            "an invalidated initial-generation witness cannot reconcile again",
        );
        let approved_events = ledger.events(&accepted.job_id, &owner, 0, 1_009).expect("terminal owner events").events.into_iter().filter(|event| event.kind == "approved").count();
        assert_eq!(approved_events, 1, "initial-generation reconciliation emits one approved event");
        drop(committer_port);
        drop(committer);
        drop(handle);
        let mut database = Arc::try_unwrap(database).ok().expect("sole database owner");
        database.shutdown(&db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.expect("database shutdown");
        drop(database);
        pool.shutdown().expect("worker pool shutdown");
    }

    #[tokio::test]
    async fn gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer() {
        let (identity, base) = canonical_identity_and_base();
        let ledger = ledger();
        let accepted = ledger.accept(&identity, &base.pack, 1_000).expect("accepted job");
        let claim = ledger.start(&accepted.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
        let (proposal, command) = canonical_approval(&identity, &accepted.job_id, &base, 1_004);
        let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
        assert!(ledger.succeed(&accepted.job_id, &identity, claim.run_epoch, &result, &proposal, 1_002).expect("offer"));
        let proposal_hash = sha256(proposal.as_slice());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_003).expect("prepared outbox");
        let pool = Arc::new(db::semio_framework_async::WorkerPool::new(db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let memory = db::storage::MemoryStorage::new(pool.clone()).await.expect("memory storage");
        let backend = Arc::new(db::storage::DbBackend::Memory(memory));
        let database = Arc::new(db::Database::open(pool.clone(), db::DbConfig::for_profile(db::Profile::Test), backend.clone()).await.expect("database"));
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let document = protocol::ArtifactId(document_key(&scope));
        let handle = database.ensure_document(&document).await.expect("document actor");
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));
        let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> =
            Arc::new(OrderedApprovalCheckpointPublisherV1 { ledger: ledger.clone(), identity: identity.clone(), job_id: accepted.job_id.clone(), attempts: std::sync::atomic::AtomicUsize::new(0), order: order.clone(), pause: None });
        let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
        let key = document_key(&scope);
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        let composed_children = base.composed_children().expect("exact composed children");
        let actor = approval_actor(&identity);
        let mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
        let command_hash = sha256(command.as_slice());
        let invalid_children = vec!["gismap-drawing".to_owned(), "substituted-value".to_owned()];
        let foreign_scope_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let foreign_scope_ingress: Arc<dyn GisMapApprovalIngressAuthorityV1> = Arc::new(TestApprovalIngressAuthorityV1 {
            scope: DocumentScope::new("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", identity.document_id.clone()),
            user_id: identity.user_id.clone(),
            session_id: identity.session_id.clone(),
            authorization_generation: identity.authorization_generation,
            released: Some(foreign_scope_ingress_released.clone()),
        });
        assert!(matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &composed_children,
                    scope: &scope,
                    actor: &actor,
                    mutation_id: &mutation_id,
                    command_hash: &command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &base.frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_004,
                    document_write: gate.clone(),
                    ingress: foreign_scope_ingress,
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Conflict)
        ));
        assert!(foreign_scope_ingress_released.load(Ordering::Acquire));
        let mut substituted_bases = Vec::new();
        let mut descriptor = canonical_map_base(&identity);
        descriptor.descriptor_digest = "9".repeat(64);
        substituted_bases.push(("descriptor", descriptor));
        let mut pack = canonical_map_base(&identity);
        pack.pack = InferencePrivateBytesV1::new(b"substituted-pack".to_vec(), INPUT_MAX_BYTES).expect("bounded substituted pack");
        substituted_bases.push(("pack", pack));
        let mut head_ordinal = canonical_map_base(&identity);
        head_ordinal.frontier.head_edit_ordinal += 1;
        substituted_bases.push(("head-ordinal", head_ordinal));
        let mut head_edit = canonical_map_base(&identity);
        head_edit.frontier.head_edit_id = "substituted-edit".to_owned();
        substituted_bases.push(("head-edit", head_edit));
        let mut commit = canonical_map_base(&identity);
        commit.frontier.last_commit_seq += 1;
        substituted_bases.push(("commit", commit));
        let mut chain = canonical_map_base(&identity);
        chain.frontier.chain_hash = directory::os_directory::ArtifactHash([1; 32]);
        substituted_bases.push(("chain", chain));
        for (name, candidate) in &substituted_bases {
            let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
            assert!(
                matches!(
                    committer
                        .commit(GisMapApprovalCommitRequestV1 {
                            composed_children: &composed_children,
                            scope: &scope,
                            actor: &actor,
                            mutation_id: &mutation_id,
                            command_hash: &command_hash,
                            job_id: &accepted.job_id,
                            proposal_hash: &proposal_hash,
                            command: command.as_slice(),
                            base: candidate,
                            base_frontier: &candidate.frontier,
                            deadline_ms: 60_000,
                            now_ms: 1_004,
                            document_write: gate.clone(),
                            ingress: tracked_approval_ingress(&identity, Some(ingress_released.clone())),
                        })
                        .await,
                    Err(GisMapApprovalCommitErrorV1::Conflict)
                ),
                "{name} substitution reached cleanup admission",
            );
            assert!(ingress_released.load(Ordering::Acquire), "{name} substitution retained ingress");
        }
        let mut substituted_frontier = base.frontier.clone();
        substituted_frontier.head_edit_ordinal += 1;
        let frontier_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &composed_children,
                    scope: &scope,
                    actor: &actor,
                    mutation_id: &mutation_id,
                    command_hash: &command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &substituted_frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_004,
                    document_write: gate.clone(),
                    ingress: tracked_approval_ingress(&identity, Some(frontier_ingress_released.clone())),
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Conflict)
        ));
        assert!(frontier_ingress_released.load(Ordering::Acquire));
        let substituted_actor = format!("user:{}#session:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", identity.user_id);
        let actor_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &composed_children,
                    scope: &scope,
                    actor: &substituted_actor,
                    mutation_id: &mutation_id,
                    command_hash: &command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &base.frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_004,
                    document_write: gate.clone(),
                    ingress: tracked_approval_ingress(&identity, Some(actor_ingress_released.clone())),
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Conflict)
        ));
        assert!(actor_ingress_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(ledger.approval_recovery_by_mutation(&mutation_id).expect("pure admission refusal lookup").is_some(), "pure admission refusals preserve the prepared outbox");
        let substituted_command_hash = sha256(b"substituted-command");
        let substituted_proposal_hash = sha256(b"substituted-proposal");
        let substituted_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(
            matches!(
                committer
                    .commit(GisMapApprovalCommitRequestV1 {
                        composed_children: &composed_children,
                        scope: &scope,
                        actor: &actor,
                        mutation_id: &mutation_id,
                        command_hash: &substituted_command_hash,
                        job_id: &accepted.job_id,
                        proposal_hash: &substituted_proposal_hash,
                        command: command.as_slice(),
                        base: &base,
                        base_frontier: &base.frontier,
                        deadline_ms: 60_000,
                        now_ms: 1_004,
                        document_write: gate.clone(),
                        ingress: tracked_approval_ingress(&identity, Some(substituted_ingress_released.clone())),
                    })
                    .await,
                Err(GisMapApprovalCommitErrorV1::Conflict)
            ),
            "the public committer refuses a same-job substituted command/proposal tuple before cleanup admission",
        );
        assert!(substituted_ingress_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        let prepared_after_substitution = ledger.approval_recovery_by_mutation(&mutation_id).expect("substitution outbox lookup").expect("exact prepared outbox remains");
        assert_eq!(
            (prepared_after_substitution.0.command_hash.as_str(), prepared_after_substitution.0.proposal_hash.as_str(), prepared_after_substitution.1.authorization_generation,),
            (command_hash.as_str(), proposal_hash.as_str(), identity.authorization_generation),
        );
        let cleanup_reservations = (0..super::super::schema::JOB_CAPACITY).map(|index| format!("{index:064x}")).collect::<Vec<_>>();
        for job_id in &cleanup_reservations {
            committer.reserve_cleanup_job(job_id).expect("bounded cleanup reservation");
        }
        let capacity_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let capacity_refused = committer
            .commit(GisMapApprovalCommitRequestV1 {
                composed_children: &composed_children,
                scope: &scope,
                actor: &actor,
                mutation_id: &mutation_id,
                command_hash: &command_hash,
                job_id: &accepted.job_id,
                proposal_hash: &proposal_hash,
                command: command.as_slice(),
                base: &base,
                base_frontier: &base.frontier,
                deadline_ms: 60_000,
                now_ms: 1_004,
                document_write: gate.clone(),
                ingress: tracked_approval_ingress(&identity, Some(capacity_ingress_released.clone())),
            })
            .await;
        assert!(matches!(capacity_refused, Err(GisMapApprovalCommitErrorV1::Capacity)));
        assert!(capacity_ingress_released.load(Ordering::Acquire));
        assert_eq!(
            ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("capacity outbox query").rows.len(),
            1,
            "cleanup-capacity refusal preserves the caller's exact durable prepared owner",
        );
        for job_id in &cleanup_reservations {
            committer.release_cleanup_job(job_id);
        }
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        let prepared_runtime_shutdown_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        committer.reserve_cleanup_job(&accepted.job_id).expect("prepared cleanup reservation");
        let prepared_runtime_shutdown_owner = GisMapPreparedApprovalV1 {
            scope: scope.clone(),
            job_id: accepted.job_id.clone(),
            mutation_id: mutation_id.clone(),
            command_hash: command_hash.clone(),
            proposal_hash: proposal_hash.clone(),
            ingress: tracked_approval_ingress(&identity, Some(prepared_runtime_shutdown_released.clone())),
        };
        let prepared_runtime_shutdown_key = format!("prepared:{}", accepted.job_id);
        assert!(committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(prepared_runtime_shutdown_key.clone()));
        let prepared_runtime_shutdown_task = GisMapPreparedApprovalTaskV1 { committer: committer.as_ref().clone(), maintenance_key: prepared_runtime_shutdown_key, owner: Some(prepared_runtime_shutdown_owner) };
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread().build().expect("temporary prepared-cleanup runtime");
            runtime.spawn(async move {
                let _retained = prepared_runtime_shutdown_task;
                std::future::pending::<()>().await;
            });
            runtime.block_on(tokio::task::yield_now());
        })
        .join()
        .expect("temporary prepared-cleanup runtime shutdown");
        assert!(committer.parked_prepared.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
        assert!(!prepared_runtime_shutdown_released.load(Ordering::Acquire), "runtime shutdown reinserts the exact prepared ingress cursor instead of dropping it");
        resume_parked_cleanup(&committer).await;
        assert!(prepared_runtime_shutdown_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("prepared runtime-shutdown query").rows.is_empty());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after prepared cleanup runtime shutdown");
        let runtime_shutdown_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        committer.reserve_cleanup_job(&accepted.job_id).expect("request cleanup reservation");
        let runtime_shutdown_ingress = tracked_approval_ingress(&identity, Some(runtime_shutdown_ingress_released.clone()));
        let runtime_shutdown_request = Arc::new(GisMapApprovalRequestTokenV1);
        let (runtime_shutdown_identity, _) = RetainedGisMapApprovalCommitterV1::preflight(&GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &actor,
            mutation_id: &mutation_id,
            command_hash: &command_hash,
            job_id: &accepted.job_id,
            proposal_hash: &proposal_hash,
            command: command.as_slice(),
            base: &base,
            base_frontier: &base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_004,
            document_write: gate.clone(),
            ingress: runtime_shutdown_ingress,
        })
        .expect("runtime-shutdown cursor preflight");
        let runtime_shutdown_owner = GisMapAbandonedRequestV1 { key: key.clone(), identity: runtime_shutdown_identity, request: runtime_shutdown_request };
        let runtime_shutdown_maintenance_key = RetainedGisMapApprovalCommitterV1::abandoned_request_maintenance_key(&runtime_shutdown_owner);
        assert!(committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(runtime_shutdown_maintenance_key.clone()));
        let runtime_shutdown_task = GisMapAbandonedRequestTaskV1 { committer: committer.as_ref().clone(), maintenance_key: runtime_shutdown_maintenance_key, owner: Some(runtime_shutdown_owner) };
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread().build().expect("temporary cleanup runtime");
            runtime.spawn(async move {
                let _retained = runtime_shutdown_task;
                std::future::pending::<()>().await;
            });
            runtime.block_on(tokio::task::yield_now());
        })
        .join()
        .expect("temporary cleanup runtime shutdown");
        assert!(committer.parked_requests.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
        assert!(!runtime_shutdown_ingress_released.load(Ordering::Acquire), "runtime shutdown reinserts the exact ingress cursor instead of dropping it");
        resume_parked_cleanup(&committer).await;
        assert!(runtime_shutdown_ingress_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("runtime-shutdown outbox query").rows.is_empty());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after temporary cleanup runtime shutdown");
        let thread_committer = committer.clone();
        let thread_identity = identity.clone();
        let thread_scope = scope.clone();
        let thread_frontier = base.frontier.clone();
        let thread_descriptor = base.descriptor_digest.clone();
        let thread_pack = base.pack.as_slice().to_vec();
        let thread_command = command.as_slice().to_vec();
        let thread_children = composed_children.clone();
        let thread_actor = actor.clone();
        let thread_mutation_id = mutation_id.clone();
        let thread_command_hash = command_hash.clone();
        let thread_job_id = accepted.job_id.clone();
        let thread_proposal_hash = proposal_hash.clone();
        let thread_gate = gate.clone();
        let thread_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let thread_released = thread_ingress_released.clone();
        std::thread::spawn(move || {
            let thread_base = InferenceMapBaseV1 { frontier: thread_frontier, descriptor_digest: thread_descriptor, pack: InferencePrivateBytesV1::new(thread_pack, INPUT_MAX_BYTES).expect("bounded thread base") };
            let thread_command = InferencePrivateBytesV1::new(thread_command, super::super::command::COMMAND_MAX_BYTES).expect("bounded thread command");
            let future = thread_committer.commit(GisMapApprovalCommitRequestV1 {
                composed_children: &thread_children,
                scope: &thread_scope,
                actor: &thread_actor,
                mutation_id: &thread_mutation_id,
                command_hash: &thread_command_hash,
                job_id: &thread_job_id,
                proposal_hash: &thread_proposal_hash,
                command: thread_command.as_slice(),
                base: &thread_base,
                base_frontier: &thread_base.frontier,
                deadline_ms: 60_000,
                now_ms: 1_004,
                document_write: thread_gate,
                ingress: tracked_approval_ingress(&thread_identity, Some(thread_released)),
            });
            drop(future);
        })
        .join()
        .expect("no-runtime unpolled owner thread");
        assert!(thread_ingress_released.load(Ordering::Acquire), "no-runtime future Drop synchronously returns its ingress after exact outbox abandonment");
        assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("no-runtime outbox query").rows.is_empty());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after no-runtime Drop");
        let unpolled_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let unpolled = committer.commit(GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &actor,
            mutation_id: &mutation_id,
            command_hash: &command_hash,
            job_id: &accepted.job_id,
            proposal_hash: &proposal_hash,
            command: command.as_slice(),
            base: &base,
            base_frontier: &base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_004,
            document_write: gate.clone(),
            ingress: tracked_approval_ingress(&identity, Some(unpolled_ingress_released.clone())),
        });
        drop(unpolled);
        wait_for_abandoned_prepared_handoff(&committer, &ledger, &identity, &accepted.job_id, &key, &gate, unpolled_ingress_released.as_ref()).await;
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after an unpolled future");
        let rejected_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &invalid_children,
                    scope: &scope,
                    actor: &actor,
                    mutation_id: &mutation_id,
                    command_hash: &command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &base.frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_006,
                    document_write: gate.clone(),
                    ingress: tracked_approval_ingress(&identity, Some(rejected_ingress_released.clone())),
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Rejected)
        ),);
        assert!(rejected_ingress_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(ledger.approval_recovery_by_mutation(&mutation_id).expect("rejected preflight lookup").is_some(), "a pure preflight rejection leaves the prepared outbox unchanged");
        for (phase_index, phase) in [AbandonedApprovalPhaseV1::Preflight, AbandonedApprovalPhaseV1::Assembly, AbandonedApprovalPhaseV1::Journal].into_iter().enumerate() {
            let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let ingress = tracked_approval_ingress(&identity, Some(ingress_released.clone()));
            let mut future = committer.commit(GisMapApprovalCommitRequestV1 {
                composed_children: &composed_children,
                scope: &scope,
                actor: &actor,
                mutation_id: &mutation_id,
                command_hash: &command_hash,
                job_id: &accepted.job_id,
                proposal_hash: &proposal_hash,
                command: command.as_slice(),
                base: &base,
                base_frontier: &base.frontier,
                deadline_ms: 60_000,
                now_ms: 1_004,
                document_write: gate.clone(),
                ingress,
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), poll_approval_to_phase(&mut future, &committer, &key, phase)).await.expect("approval reaches its retained cancellation phase");
            drop(future);
            assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
            wait_for_abandoned_approval_handoff(&committer, &key, &gate, ingress_released.as_ref()).await;
            let actor = handle.checkpoint_publication_snapshot().await.expect("unchanged actor frontier");
            assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq, actor.head_edit_id), (0, 0, None));
            let pending = ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("abandoned outbox query");
            assert!(pending.rows.is_empty(), "the autonomous cancellation owner clears its exact prepared outbox row before releasing Hub ingress");
            let revived = ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005 + u64::try_from(phase_index).expect("bounded phase")).expect("same owner revives its exact abandoned outbox row");
            assert_eq!((revived.mutation_id.as_str(), revived.command_hash.as_str(), revived.proposal_hash.as_str()), (mutation_id.as_str(), command_hash.as_str(), proposal_hash.as_str()));
        }
        let prepared_events = ledger.events(&accepted.job_id, &reader(&identity), 0, 1_008).expect("owner event page").events.into_iter().filter(|event| event.kind == "approval-prepared").count();
        assert_eq!(prepared_events, 1, "three exact cancellation/revival cycles preserve one immutable preparation event");
        assert!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty(), "pre-witness cancellation emits neither a public checkpoint nor a peer rebootstrap signal");
        assert_eq!(ledger.read(&accepted.job_id, &reader(&identity), 1_005).expect("offered proposal").proposal_state, super::super::schema::InferenceProposalStateV1::Offered);
        let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let ingress = tracked_approval_ingress(&identity, Some(ingress_released.clone()));
        let mut future = committer.commit(GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &actor,
            mutation_id: &mutation_id,
            command_hash: &command_hash,
            job_id: &accepted.job_id,
            proposal_hash: &proposal_hash,
            command: command.as_slice(),
            base: &base,
            base_frontier: &base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_006,
            document_write: gate.clone(),
            ingress,
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), poll_approval_to_phase(&mut future, &committer, &key, AbandonedApprovalPhaseV1::Committed)).await.expect("approval reaches its committed receipt cutover");
        drop(future);
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            loop {
                let approved = ledger.read(&accepted.job_id, &reader(&identity), 1_007).is_ok_and(|view| view.proposal_state == super::super::schema::InferenceProposalStateV1::Approved);
                let maintenance_idle = !committer.maintenance_owns_document(&key);
                if approved && maintenance_idle && gate.try_lock().is_ok() && ingress_released.load(Ordering::Acquire) {
                    return;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("receipt cutover autonomously reaches public checkpoint and ledger apply");
        assert_eq!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(), ["public-checkpoint-attempt", "public-checkpoint-attempt", "public-checkpoint-ack", "peer-rebootstrap"],);
        let actor = handle.checkpoint_publication_snapshot().await.expect("committed actor frontier");
        assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq, actor.head_edit_id.as_ref().map(|id| id.0.as_str())), (1, 1, Some(mutation_id.as_str())));
        committer.close().await.expect("committer close");
        drop(committer);
        drop(handle);
        let mut database = Arc::try_unwrap(database).ok().expect("sole database owner");
        database.shutdown(&db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.expect("database shutdown");
        drop(database);
        pool.shutdown().expect("worker pool shutdown");
    }

    #[tokio::test]
    async fn gis_map_terminal_close_waits_for_unpolled_cleanup_and_fences_new_admission() {
        let (identity, base) = canonical_identity_and_base();
        let ledger = ledger();
        let accepted = ledger.accept(&identity, &base.pack, 1_000).expect("accepted job");
        let claim = ledger.start(&accepted.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
        let (proposal, command) = canonical_approval(&identity, &accepted.job_id, &base, 1_004);
        let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
        assert!(ledger.succeed(&accepted.job_id, &identity, claim.run_epoch, &result, &proposal, 1_002).expect("offer"));
        let proposal_hash = sha256(proposal.as_slice());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_003).expect("prepared outbox");
        let pool = Arc::new(db::semio_framework_async::WorkerPool::new(db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let memory = db::storage::MemoryStorage::new(pool.clone()).await.expect("memory storage");
        let backend = Arc::new(db::storage::DbBackend::Memory(memory));
        let database = Arc::new(db::Database::open(pool.clone(), db::DbConfig::for_profile(db::Profile::Test), backend.clone()).await.expect("database"));
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> = Arc::new(OrderedApprovalCheckpointPublisherV1 {
            ledger: ledger.clone(),
            identity: identity.clone(),
            job_id: accepted.job_id.clone(),
            attempts: std::sync::atomic::AtomicUsize::new(0),
            order: Arc::new(std::sync::Mutex::new(Vec::new())),
            pause: None,
        });
        let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        let children = base.composed_children().expect("exact composed children");
        let actor = approval_actor(&identity);
        let mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
        let command_hash = sha256(command.as_slice());
        let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let unpolled = committer.commit(GisMapApprovalCommitRequestV1 {
            composed_children: &children,
            scope: &scope,
            actor: &actor,
            mutation_id: &mutation_id,
            command_hash: &command_hash,
            job_id: &accepted.job_id,
            proposal_hash: &proposal_hash,
            command: command.as_slice(),
            base: &base,
            base_frontier: &base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_004,
            document_write: gate.clone(),
            ingress: tracked_approval_ingress(&identity, Some(ingress_released.clone())),
        });
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
        let mut close = Box::pin(committer.close());
        let close_wake = Arc::new(ApprovalPollWakeV1 { ready: std::sync::atomic::AtomicBool::new(true) });
        let close_waker = std::task::Waker::from(close_wake);
        let mut close_context = std::task::Context::from_waker(&close_waker);
        assert!(matches!(close.as_mut().poll(&mut close_context), std::task::Poll::Pending), "terminal close waits for the validated unpolled cleanup owner");
        let refused_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(
            matches!(
                committer
                    .commit(GisMapApprovalCommitRequestV1 {
                        composed_children: &children,
                        scope: &scope,
                        actor: &actor,
                        mutation_id: &mutation_id,
                        command_hash: &command_hash,
                        job_id: &accepted.job_id,
                        proposal_hash: &proposal_hash,
                        command: command.as_slice(),
                        base: &base,
                        base_frontier: &base.frontier,
                        deadline_ms: 60_000,
                        now_ms: 1_005,
                        document_write: gate.clone(),
                        ingress: tracked_approval_ingress(&identity, Some(refused_ingress_released.clone())),
                    })
                    .await,
                Err(GisMapApprovalCommitErrorV1::Unavailable)
            ),
            "terminal close atomically fences later admission",
        );
        assert!(refused_ingress_released.load(Ordering::Acquire));
        drop(unpolled);
        tokio::time::timeout(std::time::Duration::from_secs(5), &mut close).await.expect("cleanup decrement wakes terminal close").expect("terminal close");
        drop(close);
        assert!(ingress_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("terminal outbox query").rows.is_empty());
        assert!(gate.try_lock().is_ok());
        drop(committer);
        let mut database = Arc::try_unwrap(database).ok().expect("sole database owner");
        database.shutdown(&db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.expect("database shutdown");
        drop(database);
        pool.shutdown().expect("worker pool shutdown");
    }

    #[test]
    fn gis_map_proposal_fixture_pins_the_exact_frozen_comparison_limits_and_error_vocabulary() {
        let fixture = fixture();
        let preview = gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), fixture["proposalHash"].as_str().expect("proposal hash"), fixture["proposalCanonical"].as_str().expect("canonical proposal").as_bytes())
            .expect("typed owner preview");
        assert_eq!(serde_json::to_value(preview).expect("preview wire"), fixture["preview"]);
        assert_eq!(gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), &"0".repeat(64), fixture["proposalCanonical"].as_str().expect("proposal").as_bytes()).err(), Some(InferenceRouteErrorV1::Conflict));
        let substituted = b"{\"DeleteRegion\":{\"id\":\"inference-11111111111111111111111111111111\"}}";
        assert_eq!(gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), &sha256(substituted), substituted).err(), Some(InferenceRouteErrorV1::Conflict));
        let proposal = || serde_json::from_str::<serde_json::Value>(fixture["proposalCanonical"].as_str().expect("proposal")).expect("proposal value");
        let rejected = [
            ("wrong-region-id", serde_json::json!("substituted"), "/CreateRegion/item/id", InferenceRouteErrorV1::Conflict),
            ("wrong-kind", serde_json::json!("route"), "/CreateRegion/item/data/kind", InferenceRouteErrorV1::Conflict),
            ("short-ring", serde_json::json!([[7, 46], [9, 46], [9, 48], [7, 46]]), "/CreateRegion/item/data/ring", InferenceRouteErrorV1::Bounds),
            ("reordered-ring", serde_json::json!([[7, 46], [7, 48], [9, 48], [9, 46], [7, 46]]), "/CreateRegion/item/data/ring", InferenceRouteErrorV1::Conflict),
            ("out-of-range-ring", serde_json::json!([[-181, 46], [9, 46], [9, 48], [-181, 48], [-181, 46]]), "/CreateRegion/item/data/ring", InferenceRouteErrorV1::Conflict),
        ];
        for (name, value, pointer, expected) in rejected {
            let mut candidate = proposal();
            *candidate.pointer_mut(pointer).unwrap_or_else(|| panic!("{name} pointer")) = value;
            let bytes = serde_json::to_vec(&candidate).expect("candidate bytes");
            assert_eq!(gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), &sha256(&bytes), &bytes).err(), Some(expected), "{name}");
        }
        for (name, bytes) in [("malformed", b"{".as_slice()), ("non-finite", br#"{"CreateRegion":{"index":0,"item":{"id":"inference-11111111111111111111111111111111","data":{"id":"inference-11111111111111111111111111111111","kind":"inference-bounds","ring":[[1e999,46],[9,46],[9,48],[1e999,48],[1e999,46]]}}}}"#.as_slice())] {
            assert_eq!(gis_map_inference_preview(fixture["sampleJobId"].as_str().expect("sample job"), &sha256(bytes), bytes).err(), Some(InferenceRouteErrorV1::Invalid), "{name}");
        }
        let identity = identity();
        let frozen: super::super::schema::InferenceBindingIdentityV1 = serde_json::from_value(fixture["binding"].clone()).expect("frozen binding identity");
        assert_eq!(frozen, identity.binding);
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let base = InferenceMapBaseV1 {
            frontier: directory::os_directory::ArtifactFrontier {
                document_id: identity.document_id.clone(),
                head_edit_ordinal: identity.head_ordinal,
                head_edit_id: identity.head_edit_id.clone(),
                last_commit_seq: identity.last_commit_seq,
                chain_hash: directory::os_directory::ArtifactHash([0; 32]),
            },
            descriptor_digest: identity.descriptor_digest.clone(),
            pack: input(&identity),
        };
        assert_eq!(compare_frozen_identity(&frozen, &identity, &scope, &base), Ok(()));
        let drift: Vec<(&str, Box<dyn Fn(&mut Identity)>)> = vec![
            ("changed-frontier", Box::new(|value: &mut Identity| value.last_commit_seq += 1)),
            ("changed-base-pack", Box::new(|value: &mut Identity| value.input_hash = "9".repeat(64))),
            ("changed-binding-digest", Box::new(|value: &mut Identity| value.binding.digest = "9".repeat(64))),
            ("changed-catalog-generation", Box::new(|value: &mut Identity| value.binding.catalog_generation_id = "9".repeat(64))),
            ("changed-parent-dialect", Box::new(|value: &mut Identity| value.binding.parent_dialect.subset = "lite".into())),
            ("changed-surface", Box::new(|value: &mut Identity| value.binding.surface_id = "s.gis.gismap@1/*#viewer".into())),
            ("changed-granted-mode", Box::new(|value: &mut Identity| value.binding.granted_mode = "read-observe".into())),
        ];
        for (name, mutate) in &drift {
            let mut candidate = identity.clone();
            mutate(&mut candidate);
            assert_eq!(compare_frozen_identity(&frozen, &candidate, &scope, &base), Err(InferenceRouteErrorV1::Conflict), "{name} was admitted against the frozen binding");
            assert!(fixture["approvalRejections"].as_array().expect("rejections").iter().any(|row| row["name"] == *name && row["code"] == "inference.conflict"), "{name} is not pinned by the neutral corpus");
        }
        let mut cross_space = identity.clone();
        cross_space.space_id = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".into();
        assert!(compare_frozen_identity(&frozen, &cross_space, &scope, &base).is_err());
        let limits = &fixture["limits"];
        assert_eq!(limits["requestMaxBytes"], super::super::schema::REQUEST_MAX_BYTES as u64);
        assert_eq!(limits["inputMaxBytes"], INPUT_MAX_BYTES as u64);
        assert_eq!(limits["resultMaxBytes"], RESULT_MAX_BYTES as u64);
        assert_eq!(limits["proposalMaxBytes"], PROPOSAL_MAX_BYTES as u64);
        assert_eq!(limits["commandMaxBytes"], super::super::command::COMMAND_MAX_BYTES as u64);
        assert_eq!(limits["identityJsonMaxBytes"], super::super::schema::IDENTITY_JSON_MAX_BYTES as u64);
        assert_eq!(limits["jobCapacity"], super::super::schema::JOB_CAPACITY as u64);
        assert_eq!(limits["operationCapacity"], OPERATION_CAPACITY as u64);
        assert_eq!(limits["documentGateCapacity"], DOCUMENT_GATE_CAPACITY as u64);
        assert_eq!(limits["progressMaxCursor"], PROGRESS_MAX_CURSOR);
        assert_eq!(limits["eventPageMaxItems"], super::super::schema::EVENT_PAGE_MAX_ITEMS as u64);
        assert_eq!(limits["claimLeaseMaxMs"], super::super::schema::CLAIM_LEASE_MAX_MS);
        assert_eq!(limits["jobMaxLifetimeMs"], super::super::schema::JOB_MAX_LIFETIME_MS);
        assert_eq!(limits["workUnitLimit"], WORK_UNIT_LIMIT);
        assert_eq!(limits["recursionDepth"], u64::from(RECURSION_DEPTH));
        assert_eq!(limits["allocationBytes"], ALLOCATION_BYTES);
        assert_eq!(limits["approvalMaxRecords"], APPROVAL_MAX_RECORDS);
        for row in fixture["errors"].as_array().expect("error vocabulary") {
            let code = row["code"].as_str().expect("code");
            let status = row["status"].as_u64().expect("status");
            let published = [
                InferenceRouteErrorV1::Unavailable,
                InferenceRouteErrorV1::Denied,
                InferenceRouteErrorV1::NotFound,
                InferenceRouteErrorV1::Invalid,
                InferenceRouteErrorV1::Bounds,
                InferenceRouteErrorV1::Conflict,
                InferenceRouteErrorV1::Capacity,
                InferenceRouteErrorV1::Expired,
                InferenceRouteErrorV1::Cancelled,
                InferenceRouteErrorV1::CommitUnavailable,
                InferenceRouteErrorV1::Storage,
            ]
            .into_iter()
            .find(|candidate| candidate.code() == code)
            .unwrap_or_else(|| panic!("{code} is not a published inference route code"));
            assert_eq!(u64::from(published.status()), status, "{code}");
        }
        assert_eq!(document_key(&scope), format!("v1:{}:{}:{}{}", identity.space_id.len(), identity.document_id.len(), identity.space_id, identity.document_id));
        assert_eq!(approval_actor(&identity), format!("user:{}#session:{}", identity.user_id, identity.session_id));
    }
}
