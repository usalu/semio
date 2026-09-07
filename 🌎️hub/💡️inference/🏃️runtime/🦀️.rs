//! 🏃️ Owner-private GIS Map proposal runtime: frozen binding, ledger, per-document gate, typed approval.

use std::collections::HashMap;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};

use directory::os_directory::{ArtifactFrontier, DocumentScope};

use super::catalog::VerifiedGisMapArtifactBindingV1;
use super::command::{CanonicalInferenceCommandPartsV1, CanonicalInferenceCommandV1, encode_server_stamped_command_v1};
use super::schema::{GIS_DOCUMENT_SCHEMA, InferenceIdentityV1, PROPOSAL_MAX_BYTES, RESULT_MAX_BYTES};
use super::sqlite::{InferenceJobLedgerV1, InferenceReaderV1};
use super::wal::{CommittedInferenceWalWitnessV1, InferenceDocumentFenceV1, InferenceWalTargetV1, InferenceWalVerifierV1};
use super::{InferenceErrorV1, InferenceOperationControlV1, InferencePrivateBytesV1, sha256};
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
        deadline_ms: u64,
        now_ms: u64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<directory::os_directory::PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1>> + Send + 'a>>;
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

    fn close<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async { Ok(()) })
    }
}

type GisMapParentSnapshotV1 = semio_s_plugin_gis::artifacts::gismap::GisMapSnapshot;
type GisMapParentMutationV1 = semio_s_plugin_gis::artifacts::gismap::mutations::GisMapMutation;
type GisMapDrawingSnapshotV1 = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
type GisMapDrawingMutationV1 = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
type GisMapValueSnapshotV1 = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
type GisMapValueMutationV1 = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::mutations::SemioValueMutation;
type GisMapParentStoreV1 = directory::os_store::ArtifactStore<GisMapParentSnapshotV1, GisMapParentMutationV1>;
type GisMapDrawingStoreV1 = directory::os_store::ArtifactStore<GisMapDrawingSnapshotV1, GisMapDrawingMutationV1>;
type GisMapValueStoreV1 = directory::os_store::ArtifactStore<GisMapValueSnapshotV1, GisMapValueMutationV1>;
type GisMapAssemblyV1 = directory::os_store::durable_group::DurableOwnedThreeStoreMapAssemblyV1<GisMapParentSnapshotV1, GisMapParentMutationV1, GisMapDrawingSnapshotV1, GisMapDrawingMutationV1, GisMapValueSnapshotV1, GisMapValueMutationV1>;
type GisMapCommitHostV1 = directory::os_store::durable_group::DurableOwnedMapCommitHostV1<GisMapParentSnapshotV1, GisMapParentMutationV1, GisMapDrawingSnapshotV1, GisMapDrawingMutationV1, GisMapValueSnapshotV1, GisMapValueMutationV1>;
type GisMapRecoveryOwnerV1 = db::document::ArtifactDurableGroupRecoveryOwnerV1<
    GisMapParentSnapshotV1,
    GisMapParentMutationV1,
    GisMapDrawingSnapshotV1,
    GisMapDrawingMutationV1,
    GisMapValueSnapshotV1,
    GisMapValueMutationV1,
>;
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
    Verification { owners: GisMapDocumentStoresV1, identity: GisMapCommitIdentityV1, receipt: GisMapJournalReceiptV1, document_write: GisMapDocumentWriteLeaseV1 },
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
    Continue,
    Committed,
    Recover,
    Recovered,
    Verify { identity: GisMapCommitIdentityV1, generation: u64, receipt: GisMapJournalReceiptV1 },
    Retry,
}

struct GisMapApprovalRequestOwnerV1 {
    committer: RetainedGisMapApprovalCommitterV1,
    key: String,
    identity: Option<GisMapCommitIdentityV1>,
    request: Arc<GisMapApprovalRequestTokenV1>,
}

impl GisMapApprovalRequestOwnerV1 {
    fn complete(&mut self) {
        self.identity = None;
    }
}

impl Drop for GisMapApprovalRequestOwnerV1 {
    fn drop(&mut self) {
        let Some(identity) = self.identity.take() else { return };
        let mut maintenance = self.committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !maintenance.insert(self.key.clone()) {
            return;
        }
        drop(maintenance);
        let committer = self.committer.clone();
        let key = self.key.clone();
        let request = self.request.clone();
        match tokio::runtime::Handle::try_current() {
            Ok(runtime) => {
                runtime.spawn(async move {
                    committer.drive_abandoned_request(&key, identity, request).await;
                    committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&key);
                });
            }
            Err(_) => {
                self.committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&self.key);
            }
        }
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
}

impl RetainedGisMapApprovalCommitterV1 {
    pub fn new(
        database: Arc<db::Database>,
        storage: Arc<db::storage::DbBackend>,
        ledger: Arc<InferenceJobLedgerV1>,
        publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1>,
    ) -> Self {
        Self {
            database,
            verifier: Arc::new(InferenceWalVerifierV1::new(storage)),
            ledger,
            publisher,
            documents: Arc::new(tokio::sync::Mutex::new(HashMap::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
            next_operation: Arc::new(AtomicU64::new(1)),
            maintenance: Arc::new(std::sync::Mutex::new(std::collections::HashSet::with_capacity(GIS_MAP_COMMITTER_CAPACITY))),
        }
    }

    fn parent_store(id: &str, snapshot: GisMapParentSnapshotV1) -> GisMapParentStoreV1 {
        use directory::ArtifactPack;
        let envelope = directory::os_store::create_document_envelope::<GisMapParentSnapshotV1, GisMapParentMutationV1>(GIS_DOCUMENT_SCHEMA, id, snapshot.clone(), None);
        let digest = *semio_framework_hash::hash(&snapshot.encode_pack()).as_bytes();
        let runtime = directory::os_store::ArtifactStoreInitializationRuntime::new(id, GIS_DOCUMENT_SCHEMA, snapshot, digest);
        directory::os_store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, semio_s_plugin_gis::artifacts::gismap::spr::gis_map_document_store_owners())
    }

    fn drawing_store(id: &str, snapshot: GisMapDrawingSnapshotV1) -> GisMapDrawingStoreV1 {
        use directory::{ArtifactPack, os_store::MemberStoreOwner};
        let schema = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA;
        let envelope = directory::os_store::create_document_envelope::<GisMapDrawingSnapshotV1, GisMapDrawingMutationV1>(schema, id, snapshot.clone(), None);
        let digest = *semio_framework_hash::hash(&snapshot.encode_pack()).as_bytes();
        let runtime = directory::os_store::ArtifactStoreInitializationRuntime::new(id, schema, snapshot, digest);
        directory::os_store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, <GisMapDrawingSnapshotV1 as MemberStoreOwner<GisMapDrawingMutationV1>>::member_store_owners())
    }

    fn value_store(id: &str, snapshot: GisMapValueSnapshotV1) -> GisMapValueStoreV1 {
        use directory::{ArtifactPack, os_store::MemberStoreOwner};
        let schema = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
        let envelope = directory::os_store::create_document_envelope::<GisMapValueSnapshotV1, GisMapValueMutationV1>(schema, id, snapshot.clone(), None);
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
        let admitted_actor = actor.is_some_and(|(user, session)| {
            request.ingress.scope() == request.scope
                && request.ingress.user_id() == user
                && request.ingress.session_id() == session
                && request.ingress.authorization_generation() != 0
        });
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
    async fn mount_document(
        &self,
        scope: DocumentScope,
        handle: db::ArtifactHandle,
        base: &InferenceMapBaseV1,
        document_write: GisMapDocumentWriteLeaseV1,
    ) -> Result<bool, GisMapApprovalCommitErrorV1> {
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
                    owners.scope == scope
                        && owners.generation == observed.authority_generation
                        && Arc::ptr_eq(&owners.document_write, &document_write.gate)
                        && Self::stores_match(owners, &snapshot)
                }
                RetainedGisMapDocumentStateV1::Recovered { owners, document_write: retained, .. } => {
                    owners.scope == scope && owners.generation == observed.authority_generation && Arc::ptr_eq(&retained.gate, &document_write.gate)
                }
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
        let drawing_id = format!("{key}#{}", snapshot.drawing.child_id);
        let value_id = format!("{key}#{}", snapshot.value.child_id);
        let admission = directory::os_store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(
            Self::parent_store(&parent_id, snapshot),
            Self::drawing_store(&drawing_id, drawing),
            Self::value_store(&value_id, value),
        );
        let recovery = handle.durable_group_recovery_retained(admission);
        documents.insert(
            key,
            RetainedGisMapDocumentStateV1::Recovery {
                owner: recovery,
                handle,
                scope,
                generation: observed.authority_generation,
                document_write,
                fence,
            },
        );
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

    async fn resume_recovered_checkpoint(
        &self,
        key: &str,
    ) -> Result<(GisMapCommitIdentityV1, u64, GisMapJournalReceiptV1), GisMapApprovalCommitErrorV1> {
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        let RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } = state else {
            documents.insert(key.to_owned(), state);
            return Err(GisMapApprovalCommitErrorV1::Conflict);
        };
        let recovered = (|| {
            let mutation_id = checkpoint.head_edit_id().0.as_str();
            let (outbox, accepted, ledger_applied) = self
                .ledger
                .approval_recovery_by_mutation(mutation_id)
                .map_err(|_| GisMapApprovalCommitErrorV1::Storage)?
                .ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
            accepted.validate().map_err(|_| GisMapApprovalCommitErrorV1::Conflict)?;
            let scope = DocumentScope::new(accepted.space_id.clone(), accepted.document_id.clone());
            let actor = approval_actor(&accepted);
            let chain_hash = directory::os_directory::ArtifactHash::parse_hex(&accepted.chain_hash).ok_or(GisMapApprovalCommitErrorV1::Conflict)?;
            let command_matches = if outbox.command.as_slice().is_empty() {
                true
            } else {
                let command = CanonicalInferenceCommandV1::decode(outbox.command.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Conflict)?;
                outbox.command_hash == sha256(outbox.command.as_slice()) && command.matches_identity(&outbox.mutation_id, key, &actor)
            };
            if scope != owners.scope
                || outbox.mutation_id != mutation_id
                || outbox.mutation_id != approval_mutation_id(&outbox.job_id, &outbox.proposal_hash)
                || !command_matches
            {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let identity = GisMapCommitIdentityV1 {
                scope,
                actor,
                mutation_id: outbox.mutation_id,
                command_hash: outbox.command_hash,
                job_id: outbox.job_id,
                proposal_hash: outbox.proposal_hash,
                base_frontier: ArtifactFrontier {
                    document_id: accepted.document_id,
                    head_edit_ordinal: accepted.head_ordinal,
                    head_edit_id: accepted.head_edit_id,
                    last_commit_seq: accepted.last_commit_seq,
                    chain_hash,
                },
                descriptor_digest: accepted.descriptor_digest,
                base_digest: accepted.input_hash,
                timestamp: protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 },
                journal_now_ms: outbox.prepared_at_ms,
                document_write: owners.document_write.clone(),
                ingress: None,
            };
            Ok((identity, owners.generation, checkpoint.receipt().clone(), ledger_applied))
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

    async fn prepare_retained_document(
        &self,
        scope: &DocumentScope,
        base: &InferenceMapBaseV1,
        gate: Arc<tokio::sync::Mutex<()>>,
        request: Arc<GisMapApprovalRequestTokenV1>,
    ) -> Result<(), GisMapApprovalCommitErrorV1> {
        let key = document_key(scope);
        loop {
            enum PrepareTurn {
                Recover,
                PublishRecovered,
                Acquire,
                Ready,
            }
            let turn = {
                let documents = self.documents.lock().await;
                match documents.get(&key) {
                    Some(RetainedGisMapDocumentStateV1::Recovery { scope: stored, document_write, .. }) => {
                        if stored != scope || !Arc::ptr_eq(&document_write.gate, &gate) || !Self::request_matches(document_write, &request) {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        PrepareTurn::Recover
                    }
                    Some(RetainedGisMapDocumentStateV1::Ready { owners, document_write, .. }) => {
                        if owners.scope != *scope || !Arc::ptr_eq(&owners.document_write, &gate) || !Self::stores_match(owners, &<GisMapParentSnapshotV1 as directory::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?) {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        match document_write {
                            Some(document_write) if Self::request_matches(document_write, &request) => PrepareTurn::Ready,
                            Some(_) => return Err(GisMapApprovalCommitErrorV1::Conflict),
                            None => PrepareTurn::Acquire,
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Published { owners, identity, document_write, .. }) => {
                        if owners.scope != *scope
                            || !Arc::ptr_eq(&owners.document_write, &gate)
                            || identity.base_frontier != base.frontier
                            || identity.base_digest != base.digest()
                        {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        match document_write {
                            Some(document_write) if Self::request_matches(document_write, &request) => PrepareTurn::Ready,
                            Some(_) => return Err(GisMapApprovalCommitErrorV1::Conflict),
                            None => PrepareTurn::Acquire,
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Recovered { owners, document_write, .. }) => {
                        if owners.scope != *scope || !Arc::ptr_eq(&owners.document_write, &gate) || !Arc::ptr_eq(&document_write.gate, &gate) {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        PrepareTurn::PublishRecovered
                    }
                    Some(RetainedGisMapDocumentStateV1::Verification { owners, identity, document_write, .. })
                    | Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity, document_write, .. }) => {
                        if owners.scope != *scope
                            || !Arc::ptr_eq(&owners.document_write, &gate)
                            || !Arc::ptr_eq(&document_write.gate, &gate)
                            || identity.base_frontier != base.frontier
                            || identity.base_digest != base.digest()
                        {
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        PrepareTurn::Ready
                    }
                    Some(_) => return Err(GisMapApprovalCommitErrorV1::Conflict),
                    None => PrepareTurn::Acquire,
                }
            };
            match turn {
                PrepareTurn::Ready => return Ok(()),
                PrepareTurn::Recover => {
                    self.finish_document_recovery(&key).await?;
                    continue;
                }
                PrepareTurn::PublishRecovered => {
                    self.resume_recovered_checkpoint(&key).await?;
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
                        let handle = self
                            .database
                            .document(&protocol::ArtifactId(key.clone()))
                            .await
                            .map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
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
        use semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;
        use semio_s_plugin_gis::editor::gis2d::{GisMapOneItemStampV1, gis_map_drawing_stamped_one_item_preparation_factory, gis_map_parent_stamped_one_item_preparation_factory, gis_map_value_stamped_one_item_preparation_factory};
        let work = match GisMapInference::infer(snapshot).create_region_group_work(snapshot, &identity.job_id) {
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
        let parent_admission = DurableOwnedMapMemberAdmissionV1::new(operations[0], parent.generation_now(), parent.content_revision_now(), identity.actor.clone(), work.parent, Some(format!("inference:{}", identity.job_id)));
        let drawing_admission = DurableOwnedMapMemberAdmissionV1::new(operations[1], drawing.generation_now(), drawing.content_revision_now(), identity.actor.clone(), work.drawing, Some(format!("inference:{}:drawing", identity.job_id)));
        let value_admission = DurableOwnedMapMemberAdmissionV1::new(operations[2], value.generation_now(), value.content_revision_now(), identity.actor.clone(), work.value, Some(format!("inference:{}:value", identity.job_id)));
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
                documents.insert(
                    key.to_owned(),
                    RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write },
                );
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
                if matches { GisMapCommitTurnV1::Verify { generation, receipt } } else { GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict) }
            }
            RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write } => {
                let matches = Self::identity_matches(&identity, candidate);
                let generation = owners.generation;
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt: receipt.clone(), document_write });
                if matches { GisMapCommitTurnV1::Verify { generation, receipt } } else { GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict) }
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

    async fn drive_abandoned_turn(
        &self,
        key: &str,
        candidate: &GisMapCommitIdentityV1,
        request: &Arc<GisMapApprovalRequestTokenV1>,
    ) -> GisMapAbandonedTurnV1 {
        use directory::os_store::durable_group::{DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1};
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return GisMapAbandonedTurnV1::Complete };
        match state {
            RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence } => {
                let matches = Self::request_matches(&document_write, request);
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                if matches { GisMapAbandonedTurnV1::Recover } else { GisMapAbandonedTurnV1::Complete }
            }
            RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } => {
                let matches = Self::request_matches(&document_write, request);
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write });
                if matches { GisMapAbandonedTurnV1::Recovered } else { GisMapAbandonedTurnV1::Complete }
            }
            RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write } => {
                let matches = document_write.as_ref().is_some_and(|lease| Self::request_matches(lease, request))
                    && pending.as_ref().is_none_or(|identity| Self::identity_matches(identity, candidate));
                if matches {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                    drop(document_write);
                } else {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write });
                }
                GisMapAbandonedTurnV1::Complete
            }
            RetainedGisMapDocumentStateV1::Assembly { mut owner, handle, scope, generation, identity, document_write, fence } => {
                if !Self::request_matches(&document_write, request) || !Self::identity_matches(&identity, candidate) {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                    return GisMapAbandonedTurnV1::Complete;
                }
                owner.cancel();
                match owner.advance(directory::os_store::ArtifactStoreOneItemGrant {
                    maximum_items: 1,
                    maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES,
                }) {
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
                        let owners = Self::restored_stores(
                            terminal.parent,
                            terminal.drawing,
                            terminal.value,
                            handle,
                            scope,
                            generation,
                            identity.document_write.clone(),
                            fence,
                        );
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                        drop(document_write);
                        GisMapAbandonedTurnV1::Complete
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
                match owner.advance(directory::os_store::ArtifactStoreOneItemGrant {
                    maximum_items: 1,
                    maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES,
                }) {
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
                        let owners = Self::restored_stores(
                            terminal.parent,
                            terminal.drawing,
                            terminal.value,
                            handle,
                            scope,
                            generation,
                            identity.document_write.clone(),
                            fence,
                        );
                        if let Some(receipt) = receipt {
                            let verify_identity = identity.clone();
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write });
                            GisMapAbandonedTurnV1::Verify { identity: verify_identity, generation, receipt }
                        } else {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                            drop(document_write);
                            GisMapAbandonedTurnV1::Complete
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
            let retry = match turn {
                GisMapAbandonedTurnV1::Complete => return,
                GisMapAbandonedTurnV1::Continue => false,
                GisMapAbandonedTurnV1::Committed => {
                    identity.ingress = None;
                    false
                }
                GisMapAbandonedTurnV1::Recover => self.finish_document_recovery(key).await.is_err(),
                GisMapAbandonedTurnV1::Recovered => match self.resume_recovered_checkpoint(key).await {
                    Ok((identity, generation, receipt)) => self
                        .verify(
                            key,
                            &identity,
                            generation,
                            receipt,
                            identity.journal_now_ms.saturating_add(super::schema::JOB_MAX_LIFETIME_MS),
                            identity.journal_now_ms,
                        )
                        .await
                        .is_err(),
                    Err(_) => true,
                },
                GisMapAbandonedTurnV1::Verify { identity, generation, receipt } => self
                    .verify(
                        key,
                        &identity,
                        generation,
                        receipt,
                        identity.journal_now_ms.saturating_add(super::schema::JOB_MAX_LIFETIME_MS),
                        identity.journal_now_ms,
                    )
                    .await
                    .is_err(),
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

    async fn finish_preflight(
        &self,
        key: &str,
        candidate: &GisMapCommitIdentityV1,
        snapshot: &GisMapParentSnapshotV1,
        observed: &db::CheckpointPublicationSnapshot,
    ) -> Result<(), GisMapApprovalCommitErrorV1> {
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

    async fn publish_checkpoint(
        &self,
        key: &str,
        identity: &GisMapCommitIdentityV1,
        generation: u64,
        receipt: &GisMapJournalReceiptV1,
        deadline_ms: u64,
        now_ms: u64,
    ) -> Result<directory::os_directory::PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1> {
        let (request, document_write) = {
            let documents = self.documents.lock().await;
            let Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write }) = documents.get(key) else {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            };
            if !Self::identity_matches(stored, identity) || stored_receipt != receipt || owners.generation != generation {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let actor_snapshot = owners.handle.checkpoint_publication_snapshot().await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
            let projected = Self::publication_frontier(&owners.scope, &actor_snapshot).ok_or(GisMapApprovalCommitErrorV1::Storage)?;
            if actor_snapshot.authority_generation != generation
                || projected.head_edit_ordinal != identity.base_frontier.head_edit_ordinal.checked_add(1).ok_or(GisMapApprovalCommitErrorV1::Capacity)?
                || projected.last_commit_seq != identity.base_frontier.last_commit_seq.checked_add(1).ok_or(GisMapApprovalCommitErrorV1::Capacity)?
                || projected.head_edit_id != identity.mutation_id
                || projected.chain_hash.0 != owners.parent.as_ref().ok_or(GisMapApprovalCommitErrorV1::Storage)?.content_revision_now()
            {
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
            let files = owners.parent.as_ref().ok_or(GisMapApprovalCommitErrorV1::Storage)?.snapshot_pack().await.map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
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
        let published = self.publisher.publish(request, document_write, deadline_ms, now_ms).await?;
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

    async fn verify(&self, key: &str, identity: &GisMapCommitIdentityV1, generation: u64, receipt: GisMapJournalReceiptV1, deadline_ms: u64, now_ms: u64) -> Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1> {
        let lifetime = deadline_ms.checked_sub(now_ms).filter(|value| *value != 0).ok_or(GisMapApprovalCommitErrorV1::Rejected)?;
        let control = Arc::new(InferenceOperationControlV1::new(lifetime, 65_536).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?);
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
                RetainedGisMapDocumentStateV1::Verification { owners, identity: stored, receipt: stored_receipt, document_write }
                    if Self::identity_matches(&stored, identity) && stored_receipt == receipt =>
                {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write });
                    false
                }
                RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write }
                    if Self::identity_matches(&stored, identity) && stored_receipt == receipt =>
                {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write });
                    false
                }
                RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write }
                    if Self::identity_matches(&stored, identity) && stored_receipt == receipt =>
                {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write });
                    true
                }
                state => {
                    documents.insert(key.to_owned(), state);
                    return Err(GisMapApprovalCommitErrorV1::Conflict);
                }
            }
        };
        if !already_published {
            self.publish_checkpoint(key, identity, generation, &receipt, deadline_ms, now_ms).await?;
        }
        let applied = self
            .ledger
            .reconcile_committed_approval(&identity.job_id, &witness, generation, identity.journal_now_ms)
            .map_err(|error| match error {
                InferenceErrorV1::Conflict | InferenceErrorV1::Invalid | InferenceErrorV1::Denied => GisMapApprovalCommitErrorV1::Conflict,
                _ => GisMapApprovalCommitErrorV1::Storage,
            })?;
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        match state {
            RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write }
                if Self::identity_matches(&stored, identity) && stored_receipt == receipt =>
            {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write: None });
                drop(document_write);
            }
            RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write }
                if Self::identity_matches(&stored, identity) && stored_receipt == receipt =>
            {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write: None });
                drop(document_write);
            }
            state => {
                documents.insert(key.to_owned(), state);
                return Err(GisMapApprovalCommitErrorV1::Conflict);
            }
        }
        Ok(GisMapApprovalReceiptV1 { witness, document_generation: generation, applied })
    }

    async fn commit_retained(&self, request: GisMapApprovalCommitRequestV1<'_>) -> Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1> {
        let (mut identity, snapshot) = Self::preflight(&request)?;
        let base = request.base;
        let deadline_ms = request.deadline_ms;
        let now_ms = request.now_ms;
        let document_write = request.document_write;
        let mut live_ingress = Some(request.ingress);
        let key = document_key(&identity.scope);
        let request_token = Arc::new(GisMapApprovalRequestTokenV1);
        let mut owner = GisMapApprovalRequestOwnerV1 {
            committer: self.clone(),
            key: key.clone(),
            identity: Some(identity.clone()),
            request: request_token.clone(),
        };
        self.prepare_retained_document(&identity.scope, base, document_write, request_token).await?;
        let result = loop {
            match self.drive_turn(&key, &identity, &snapshot).await {
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
                GisMapCommitTurnV1::Verify { generation, receipt } => break self.verify(&key, &identity, generation, receipt, deadline_ms, now_ms).await,
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
            RetainedGisMapDocumentStateV1::Ready { owners, document_write, .. } => {
                Self::closing(owners, document_write)
            }
            RetainedGisMapDocumentStateV1::Published { owners, document_write, .. } => {
                Self::closing(owners, document_write)
            }
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
        loop {
            let next = {
                let documents = self.documents.lock().await;
                documents.iter().next().map(|(key, state)| {
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
                        RetainedGisMapDocumentStateV1::Assembly { .. }
                        | RetainedGisMapDocumentStateV1::Journal { .. }
                        | RetainedGisMapDocumentStateV1::Verification { .. }
                        | RetainedGisMapDocumentStateV1::Publishing { .. } => return Some((key.clone(), None)),
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
            let Some((key, gate)) = next else { return Ok(()) };
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
                continue;
            }
            let verification = {
                let documents = self.documents.lock().await;
                match documents.get(&key) {
                    Some(RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, .. }) => Some((identity.clone(), owners.generation, receipt.clone())),
                    _ => None,
                }
            };
            if let Some((identity, generation, receipt)) = verification {
                self.verify(
                    &key,
                    &identity,
                    generation,
                    receipt,
                    identity.journal_now_ms.saturating_add(super::schema::JOB_MAX_LIFETIME_MS),
                    identity.journal_now_ms,
                )
                .await?;
                continue;
            }
            if self.close_turn(&key).await? {
                continue;
            }
            tokio::task::yield_now().await;
        }
    }
}

impl GisMapApprovalCommitterV1 for RetainedGisMapApprovalCommitterV1 {
    fn commit<'a>(&'a self, request: GisMapApprovalCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(self.commit_retained(request))
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

/// 🏃️ The hub's only inference authority: frozen binding, ledger, gates, retained cancellation.
pub struct HubInferenceRuntimeV1 {
    binding: Arc<VerifiedGisMapArtifactBindingV1>,
    ledger: Arc<InferenceJobLedgerV1>,
    operations: Mutex<HashMap<String, Arc<InferenceOperationControlV1>>>,
    document_gates: Mutex<HashMap<String, Arc<tokio::sync::Mutex<()>>>>,
    committer: Arc<dyn GisMapApprovalCommitterV1>,
}

impl HubInferenceRuntimeV1 {
    /// 🧊️ Binds one process-lifetime runtime to an already-verified GIS Map selection.
    pub fn new(binding: Arc<VerifiedGisMapArtifactBindingV1>, ledger: Arc<InferenceJobLedgerV1>, committer: Arc<dyn GisMapApprovalCommitterV1>) -> Self {
        Self { binding, ledger, operations: Mutex::new(HashMap::new()), document_gates: Mutex::new(HashMap::new()), committer }
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

    /// 🧹️ Drives every retained GIS approval owner to terminal process handoff.
    pub async fn close(&self) -> Result<(), GisMapApprovalCommitErrorV1> {
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

    /// 🎛️ Retains one bounded cancellation controller for the lifetime of a running job.
    pub fn retain_operation(&self, job_id: &str, control: Arc<InferenceOperationControlV1>) -> Result<(), InferenceRouteErrorV1> {
        let mut operations = self.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if operations.len() >= OPERATION_CAPACITY && !operations.contains_key(job_id) {
            return Err(InferenceRouteErrorV1::Capacity);
        }
        operations.insert(job_id.to_owned(), control);
        Ok(())
    }

    /// 🧹️ Releases a retained controller once its bounded work has terminated.
    pub fn release_operation(&self, job_id: &str) {
        if let Ok(mut operations) = self.operations.lock() {
            operations.remove(job_id);
        }
    }

    /// 🛑️ Interrupts the retained controller so running work stops at its next bounded checkpoint.
    pub fn interrupt_operation(&self, job_id: &str) {
        if let Ok(operations) = self.operations.lock() {
            if let Some(control) = operations.get(job_id) {
                control.cancel();
            }
        }
    }

    /// 🔍️ Rejects any drift between the frozen job identity and the current server-materialized base.
    pub fn compare_frozen(&self, identity: &InferenceIdentityV1, scope: &DocumentScope, base: &InferenceMapBaseV1) -> Result<(), InferenceRouteErrorV1> {
        compare_frozen_identity(&self.binding.identity(), identity, scope, base)
    }

    /// 🧮️ Runs the frozen native GIS service under the claim's control and derives its sole proposal.
    pub fn infer(&self, identity: &InferenceIdentityV1, job_id: &str, base: &InferenceMapBaseV1, control: &InferenceOperationControlV1, checkpoint: &mut dyn FnMut(u64, u64)) -> Result<InferenceProposalV1, InferenceRouteErrorV1> {
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
        let mut failure = None;
        let execution = semio_s_plugin_gis::artifacts::gismap::infer_gis_map_controlled(&request, &mut |completed| {
            if let Err(error) = control.checkpoint(completed) {
                failure = Some(error);
                return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("hub.inference.interrupted", "bounded inference interrupted"));
            }
            checkpoint(completed, WORK_UNIT_LIMIT);
            Ok(())
        });
        let execution = match execution {
            Ok(execution) => execution,
            Err(_) => return Err(failure.map_or(InferenceRouteErrorV1::Invalid, InferenceRouteErrorV1::from)),
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
    ) -> Result<bool, InferenceRouteErrorV1> {
        commit_prepared_approval(&self.committer, identity, job_id, proposal_hash, command, base, deadline_ms, now_ms, document_write, ingress).await
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
) -> Result<bool, InferenceRouteErrorV1> {
    let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
    if ingress.scope() != &scope
        || ingress.user_id() != identity.user_id
        || ingress.session_id() != identity.session_id
        || ingress.authorization_generation() != identity.authorization_generation
    {
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
    Ok(receipt.applied)
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

/// 📥️ Accepts one closed client intent, claims it, runs the frozen service, and offers one proposal.
pub async fn submit_gis_map_job(context: InferenceRouteContextV1<'_>, body: &[u8]) -> Result<InferenceJobReceiptDtoV1, InferenceRouteErrorV1> {
    let request = super::schema::InferenceRequestV1::decode(body)?;
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    let gate = runtime.document_gate(&context.scope)?;
    let control = Arc::new(InferenceOperationControlV1::new(request.lifetime_ms, WORK_UNIT_LIMIT)?);
    let base_control = InferenceMapBaseControlV1 { deadline_ms: context.now_ms.saturating_add(super::schema::JOB_MAX_LIFETIME_MS), now_ms: context.now_ms, control: control.clone() };
    let (receipt, identity, claim, base) = {
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
        runtime.retain_operation(&receipt.job_id, control.clone())?;
        let claim = runtime.ledger().start(&receipt.job_id, &identity, context.now_ms)?;
        (receipt, identity, claim, base)
    };
    let Some(claim) = claim else {
        runtime.release_operation(&receipt.job_id);
        return owner_page_receipt(runtime, &receipt.job_id, &identity, receipt.expires_at_ms, context.now_ms);
    };
    let ledger = runtime.ledger().clone();
    let job_id = receipt.job_id.clone();
    let owner = reader(&identity);
    let mut appended = 0_u64;
    let mut last = 0_u64;
    let outcome = runtime.infer(&identity, &receipt.job_id, &base, &control, &mut |completed, total| {
        if completed > last && appended < super::schema::PROGRESS_MAX_CURSOR && ledger.progress(&job_id, &owner, claim.run_epoch, completed, total, context.now_ms).is_ok() {
            appended += 1;
            last = completed;
        }
    });
    let published: Result<bool, InferenceRouteErrorV1> = match outcome {
        Ok(proposal) => {
            let _guard = gate.lock().await;
            let author = super::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await.map_err(InferenceRouteErrorV1::from);
            let checked = match author {
                Ok(()) => map_base(context.rebootstrap, &context.scope, base_control.deadline_ms, &base_control).await.and_then(|current| runtime.compare_frozen(&identity, &context.scope, &current)),
                Err(error) => Err(error),
            };
            match checked {
                Ok(()) => runtime.ledger().succeed(&receipt.job_id, &identity, claim.run_epoch, &proposal.result, &proposal.proposal, context.now_ms).map_err(InferenceRouteErrorV1::from),
                Err(error) => Err(error),
            }
        }
        Err(error) => {
            let _guard = gate.lock().await;
            let retired = if error == InferenceRouteErrorV1::Cancelled { runtime.ledger().cancel(&receipt.job_id, &reader(&identity), context.now_ms) } else { runtime.ledger().fail(&receipt.job_id, &reader(&identity), context.now_ms) };
            let _ = retired;
            Err(error)
        }
    };
    runtime.release_operation(&receipt.job_id);
    published?;
    owner_page_receipt(runtime, &receipt.job_id, &identity, receipt.expires_at_ms, context.now_ms)
}

fn owner_page_receipt(runtime: &Arc<HubInferenceRuntimeV1>, job_id: &str, identity: &InferenceIdentityV1, expires_at_ms: u64, now_ms: u64) -> Result<InferenceJobReceiptDtoV1, InferenceRouteErrorV1> {
    let page = runtime.ledger().events(job_id, &reader(identity), 0, now_ms)?;
    Ok(InferenceJobReceiptDtoV1 { schema: "semio.hub.inference-job-receipt/v1", job_id: job_id.to_owned(), state: page.state, proposal_state: page.proposal_state, proposal_hash: page.proposal_hash, cursor: page.next_cursor, expires_at_ms })
}

/// 📤️ Returns the owner-private bounded event page and marks a drifted offer stale to its owner.
pub async fn read_gis_map_job_events(context: InferenceRouteContextV1<'_>, job_id: &str, after: u64) -> Result<InferenceEventPageDtoV1, InferenceRouteErrorV1> {
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
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
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let control = Arc::new(InferenceOperationControlV1::new(super::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    let identity = runtime.ledger().identity_of(job_id, &session_reader(&session, &context.scope))?;
    super::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    runtime.interrupt_operation(job_id);
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
    let applied = runtime
        .commit_approval(
            &identity,
            &approval.job_id,
            &approval.proposal_hash,
            &command,
            &base,
            base_control.deadline_ms,
            context.now_ms,
            context.document_write,
            ingress,
        )
        .await?;
    Ok(InferenceApprovalReceiptDtoV1 { schema: "semio.hub.inference-approval-receipt/v1", job_id: approval.job_id, mutation_id: prepared.mutation_id, command_hash: prepared.command_hash, proposal_hash: prepared.proposal_hash, applied })
}

fn session_reader<'a>(session: &'a crate::directory::model::AuthSessionRecord, scope: &'a DocumentScope) -> InferenceReaderV1<'a> {
    InferenceReaderV1 { user_id: &session.user_id, session_id: &session.id, authorization_generation: session.authorization_generation, space_id: &scope.space_id, document_id: &scope.document_id }
}
//#endregion 🛣️Routes

/// 🔑️ Renders the exact full document key every envelope and witness compares byte for byte.
pub fn document_key(scope: &DocumentScope) -> String {
    format!("v1:{}:{}:{}{}", scope.space_id.len(), scope.document_id.len(), scope.space_id, scope.document_id)
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
    use crate::inference::schema::{INPUT_MAX_BYTES, InferenceIdentityV1 as Identity, PROGRESS_MAX_CURSOR};

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
    }

    impl GisMapApprovalCheckpointPublisherV1 for OrderedApprovalCheckpointPublisherV1 {
        fn publish<'a>(
            &'a self,
            request: GisMapApprovalCheckpointRequestV1,
            document_write: Arc<GisMapDocumentWriteAuthorityV1>,
            _deadline_ms: u64,
            now_ms: u64,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<directory::os_directory::PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
            Box::pin(async move {
                let view = self.ledger.read(&self.job_id, &reader(&self.identity), now_ms).map_err(|_| GisMapApprovalCommitErrorV1::Storage)?;
                if view.proposal_state == super::super::schema::InferenceProposalStateV1::Approved || document_write.gate.try_lock().is_ok() {
                    return Err(GisMapApprovalCommitErrorV1::Conflict);
                }
                self.order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("public-checkpoint-attempt");
                if self.attempts.fetch_add(1, Ordering::AcqRel) == 0 {
                    return Err(GisMapApprovalCommitErrorV1::Storage);
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
                    pack: directory::os_directory::PublishedArtifactBlob {
                        sha256: pack_hash,
                        byte_length: u64::try_from(request.pair.pack.len()).map_err(|_| GisMapApprovalCommitErrorV1::Capacity)?,
                    },
                    spr: directory::os_directory::PublishedArtifactBlob {
                        sha256: spr_hash,
                        byte_length: u64::try_from(request.pair.spr.len()).map_err(|_| GisMapApprovalCommitErrorV1::Capacity)?,
                    },
                    aggregate_sha256: directory::os_directory::ArtifactHash([0x46; 32]),
                    published_at_ms: now_ms,
                })
            })
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
                let maintenance_idle = !committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains(key);
                if ready && maintenance_idle && gate.try_lock().is_ok() && ingress_released.load(Ordering::Acquire) {
                    return;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("abandoned approval returns every Store, ingress and document writer");
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
            assert_eq!(ledger.progress(&receipt.job_id, &owner, claim.run_epoch, step, WORK_UNIT_LIMIT, 1_003 + step).expect("progress"), step);
        }
        assert_eq!(ledger.progress(&receipt.job_id, &owner, claim.run_epoch + 1, 5, WORK_UNIT_LIMIT, 1_010), Err(InferenceErrorV1::Conflict), "a foreign epoch cannot append progress");
        assert_eq!(ledger.progress(&receipt.job_id, &owner, claim.run_epoch, 3, WORK_UNIT_LIMIT, 1_011), Err(InferenceErrorV1::Conflict), "progress never regresses");
        let page = ledger.events(&receipt.job_id, &owner, 0, 1_012).expect("owner page");
        assert_eq!(page.progress.iter().map(|row| row.cursor).collect::<Vec<_>>(), vec![1, 2, 3, 4]);
        assert_eq!(page.next_cursor, 4);
        assert!(page.progress.len() <= super::super::schema::EVENT_PAGE_MAX_ITEMS && page.events.len() <= super::super::schema::EVENT_PAGE_MAX_ITEMS);
        assert_eq!(ledger.events(&receipt.job_id, &owner, 4, 1_013).expect("tail page").progress.len(), 0);
        assert_eq!(ledger.events(&receipt.job_id, &owner, PROGRESS_MAX_CURSOR + 1, 1_013).err(), Some(InferenceErrorV1::Bounds));
        assert!(ledger.request_cancel(&receipt.job_id, &owner, 1_014).expect("durable cancel request"));
        let cancelled = ledger.events(&receipt.job_id, &owner, 0, 1_015).expect("cancelled page");
        assert!(cancelled.cancel_requested);
        assert_eq!(
            cancelled.events.iter().map(|row| (row.ordinal, row.kind.as_str())).collect::<Vec<_>>(),
            fixture["cancelLifecycle"].as_array().expect("cancel trace").iter().map(|row| (row["ordinal"].as_u64().expect("ordinal"), row["kind"].as_str().expect("kind"))).collect::<Vec<_>>()
        );
        let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
        let proposal = InferencePrivateBytesV1::new(b"bounded-proposal".to_vec(), PROPOSAL_MAX_BYTES).expect("bounded proposal");
        assert_eq!(ledger.succeed(&receipt.job_id, &identity, claim.run_epoch, &result, &proposal, 1_016), Ok(false), "a retired job never publishes a late offer");
        assert!(ledger.request_cancel(&receipt.job_id, &owner, 1_017).is_ok(), "cancellation is idempotent");
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
        let committer: Arc<dyn GisMapApprovalCommitterV1> = Arc::new(UnavailableGisMapApprovalCommitterV1);
        for attempt in 0..2 {
            assert_eq!(
                commit_prepared_approval(
                    &committer,
                    &identity,
                    &receipt.job_id,
                    &proposal_hash,
                    &command,
                    &base,
                    60_000,
                    1_005 + attempt,
                    Arc::new(tokio::sync::Mutex::new(())),
                    approval_ingress(&identity),
                )
                .await,
                Err(InferenceRouteErrorV1::CommitUnavailable),
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
        let identity = identity();
        let owner = reader(&identity);
        let ledger = ledger();
        let accepted = ledger.accept(&identity, &input(&identity), 1_000).expect("accepted job");
        let claim = ledger.start(&accepted.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
        let corpus = ledger_fixture();
        let outbox = &corpus["outbox"];
        let proposal = InferencePrivateBytesV1::new(outbox["proposal"].as_str().expect("literal proposal").as_bytes().to_vec(), PROPOSAL_MAX_BYTES).expect("bounded proposal");
        let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
        assert!(ledger.succeed(&accepted.job_id, &identity, claim.run_epoch, &result, &proposal, 1_002).expect("offer"));
        let hex = outbox["commandHex"].as_str().expect("literal command");
        let command = InferencePrivateBytesV1::new(
            (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex byte")).collect(),
            super::super::command::COMMAND_MAX_BYTES,
        )
        .expect("bounded command");
        let proposal_hash = sha256(proposal.as_slice());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_003).expect("prepared outbox");

        let pool = Arc::new(db::semio_framework_async::WorkerPool::new(db::semio_framework_async::WorkerPoolConfig::new(
            db::semio_framework_async::ProcessKind::HeadlessBatch,
            2,
        )));
        let memory = db::storage::MemoryStorage::new(pool.clone()).await.expect("memory storage");
        let backend = Arc::new(db::storage::DbBackend::Memory(memory));
        let database = Arc::new(db::Database::open(pool.clone(), db::DbConfig::for_profile(db::Profile::Test), backend.clone()).await.expect("database"));
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let document = protocol::ArtifactId(document_key(&scope));
        let handle = database.ensure_document(&document).await.expect("document actor");
        let initial = handle.checkpoint_publication_snapshot().await.expect("initial actor frontier");
        assert_eq!((initial.frontier.head_seq, initial.frontier.commit_seq, initial.head_edit_id), (0, 0, None));
        let base = InferenceMapBaseV1 {
            frontier: directory::os_directory::ArtifactFrontier {
                document_id: identity.document_id.clone(),
                head_edit_ordinal: 0,
                head_edit_id: String::new(),
                last_commit_seq: 0,
                chain_hash: directory::os_directory::ArtifactHash([0; 32]),
            },
            descriptor_digest: identity.descriptor_digest.clone(),
            pack: input(&identity),
        };
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));
        let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> = Arc::new(OrderedApprovalCheckpointPublisherV1 {
            ledger: ledger.clone(),
            identity: identity.clone(),
            job_id: accepted.job_id.clone(),
            attempts: std::sync::atomic::AtomicUsize::new(0),
            order: order.clone(),
        });
        let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
        let committer_port: Arc<dyn GisMapApprovalCommitterV1> = committer.clone();
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        let wrong_ingress: Arc<dyn GisMapApprovalIngressAuthorityV1> = Arc::new(TestApprovalIngressAuthorityV1 {
            scope: scope.clone(),
            user_id: identity.user_id.clone(),
            session_id: identity.session_id.clone(),
            authorization_generation: identity.authorization_generation + 1,
            released: None,
        });
        assert_eq!(
            commit_prepared_approval(&committer_port, &identity, &accepted.job_id, &proposal_hash, &command, &base, 60_000, 1_004, gate.clone(), wrong_ingress).await,
            Err(InferenceRouteErrorV1::Denied),
            "a substituted Hub ingress generation is rejected before document-write acquisition",
        );
        assert!(gate.try_lock().is_ok(), "rejected ingress never acquires the document writer");
        let first_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let first_ingress = tracked_approval_ingress(&identity, Some(first_ingress_released.clone()));
        let first = commit_prepared_approval(&committer_port, &identity, &accepted.job_id, &proposal_hash, &command, &base, 60_000, 1_004, gate.clone(), first_ingress).await;
        assert_eq!(first, Err(InferenceRouteErrorV1::Storage), "public checkpoint refusal remains a retained nonterminal publication");
        assert!(first_ingress_released.load(Ordering::Acquire), "a failed request releases its live Hub ingress authority after the durable decision is retained");
        assert!(gate.try_lock().is_err(), "the exact document write authority remains held after publication refusal");
        assert_eq!(ledger.read(&accepted.job_id, &owner, 1_005).expect("retained proposal").proposal_state, super::super::schema::InferenceProposalStateV1::Offered);
        let actor = handle.checkpoint_publication_snapshot().await.expect("committed Event actor frontier");
        assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq), (1, 1));
        let expected_mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
        assert_eq!(actor.head_edit_id.as_ref().map(|value| value.0.as_str()), Some(expected_mutation_id.as_str()));

        let retry_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let retry_ingress = tracked_approval_ingress(&identity, Some(retry_ingress_released.clone()));
        assert!(commit_prepared_approval(&committer_port, &identity, &accepted.job_id, &proposal_hash, &command, &base, 60_000, 1_006, gate.clone(), retry_ingress).await.expect("fresh-authority publication retry"));
        let approved = ledger.read(&accepted.job_id, &owner, 1_007).expect("approved proposal");
        assert_eq!(approved.proposal_state, super::super::schema::InferenceProposalStateV1::Approved);
        assert_eq!(
            order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(),
            ["public-checkpoint-attempt", "public-checkpoint-attempt", "public-checkpoint-ack"],
        );
        assert!(gate.try_lock().is_ok(), "the document write authority releases only after public ACK and ledger apply");
        assert!(retry_ingress_released.load(Ordering::Acquire), "the fresh Hub ingress authority releases after public ACK and ledger apply");

        committer.close().await.expect("committer close");
        drop(committer_port);
        drop(committer);
        drop(handle);
        let mut database = Arc::try_unwrap(database).ok().expect("sole database owner");
        database.shutdown().await.expect("database shutdown");
        drop(database);
        pool.shutdown().expect("worker pool shutdown");
    }

    #[tokio::test]
    async fn gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer() {
        let identity = identity();
        let ledger = ledger();
        let accepted = ledger.accept(&identity, &input(&identity), 1_000).expect("accepted job");
        let claim = ledger.start(&accepted.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
        let corpus = ledger_fixture();
        let outbox = &corpus["outbox"];
        let proposal = InferencePrivateBytesV1::new(outbox["proposal"].as_str().expect("literal proposal").as_bytes().to_vec(), PROPOSAL_MAX_BYTES).expect("bounded proposal");
        let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
        assert!(ledger.succeed(&accepted.job_id, &identity, claim.run_epoch, &result, &proposal, 1_002).expect("offer"));
        let hex = outbox["commandHex"].as_str().expect("literal command");
        let command = InferencePrivateBytesV1::new(
            (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex byte")).collect(),
            super::super::command::COMMAND_MAX_BYTES,
        )
        .expect("bounded command");
        let proposal_hash = sha256(proposal.as_slice());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_003).expect("prepared outbox");
        let pool = Arc::new(db::semio_framework_async::WorkerPool::new(db::semio_framework_async::WorkerPoolConfig::new(
            db::semio_framework_async::ProcessKind::HeadlessBatch,
            2,
        )));
        let memory = db::storage::MemoryStorage::new(pool.clone()).await.expect("memory storage");
        let backend = Arc::new(db::storage::DbBackend::Memory(memory));
        let database = Arc::new(db::Database::open(pool.clone(), db::DbConfig::for_profile(db::Profile::Test), backend.clone()).await.expect("database"));
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let document = protocol::ArtifactId(document_key(&scope));
        let handle = database.ensure_document(&document).await.expect("document actor");
        let base = InferenceMapBaseV1 {
            frontier: directory::os_directory::ArtifactFrontier {
                document_id: identity.document_id.clone(),
                head_edit_ordinal: 0,
                head_edit_id: String::new(),
                last_commit_seq: 0,
                chain_hash: directory::os_directory::ArtifactHash([0; 32]),
            },
            descriptor_digest: identity.descriptor_digest.clone(),
            pack: input(&identity),
        };
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));
        let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> = Arc::new(OrderedApprovalCheckpointPublisherV1 {
            ledger: ledger.clone(),
            identity: identity.clone(),
            job_id: accepted.job_id.clone(),
            attempts: std::sync::atomic::AtomicUsize::new(0),
            order: order.clone(),
        });
        let committer = Arc::new(RetainedGisMapApprovalCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
        let key = document_key(&scope);
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        let composed_children = base.composed_children().expect("exact composed children");
        let actor = approval_actor(&identity);
        let mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
        let command_hash = sha256(command.as_slice());
        for phase in [AbandonedApprovalPhaseV1::Preflight, AbandonedApprovalPhaseV1::Assembly, AbandonedApprovalPhaseV1::Journal] {
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
            tokio::time::timeout(std::time::Duration::from_secs(5), poll_approval_to_phase(&mut future, &committer, &key, phase))
                .await
                .expect("approval reaches its retained cancellation phase");
            drop(future);
            wait_for_abandoned_approval_handoff(&committer, &key, &gate, ingress_released.as_ref()).await;
            let actor = handle.checkpoint_publication_snapshot().await.expect("unchanged actor frontier");
            assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq, actor.head_edit_id), (0, 0, None));
        }
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
        tokio::time::timeout(std::time::Duration::from_secs(5), poll_approval_to_phase(&mut future, &committer, &key, AbandonedApprovalPhaseV1::Committed))
            .await
            .expect("approval reaches its committed receipt cutover");
        drop(future);
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            loop {
                let approved = ledger.read(&accepted.job_id, &reader(&identity), 1_007).is_ok_and(|view| view.proposal_state == super::super::schema::InferenceProposalStateV1::Approved);
                let maintenance_idle = !committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains(&key);
                if approved && maintenance_idle && gate.try_lock().is_ok() && ingress_released.load(Ordering::Acquire) {
                    return;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("receipt cutover autonomously reaches public checkpoint and ledger apply");
        assert_eq!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(), ["public-checkpoint-attempt", "public-checkpoint-attempt", "public-checkpoint-ack"]);
        let actor = handle.checkpoint_publication_snapshot().await.expect("committed actor frontier");
        assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq, actor.head_edit_id.as_ref().map(|id| id.0.as_str())), (1, 1, Some(mutation_id.as_str())));
        committer.close().await.expect("committer close");
        drop(committer);
        drop(handle);
        let mut database = Arc::try_unwrap(database).ok().expect("sole database owner");
        database.shutdown().await.expect("database shutdown");
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
