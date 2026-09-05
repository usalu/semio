//! 🏃️ Owner-private GIS Map proposal runtime: frozen binding, ledger, per-document gate, typed approval.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use directory::os_directory::{ArtifactFrontier, DocumentScope};

use super::catalog::VerifiedGisMapArtifactBindingV1;
use super::command::{encode_server_stamped_command_v1, CanonicalInferenceCommandPartsV1};
use super::schema::{InferenceIdentityV1, GIS_DOCUMENT_SCHEMA, PROPOSAL_MAX_BYTES, RESULT_MAX_BYTES};
use super::sqlite::{InferenceJobLedgerV1, InferenceReaderV1};
use super::wal::CommittedInferenceWalWitnessV1;
use super::{sha256, InferenceErrorV1, InferenceOperationControlV1, InferencePrivateBytesV1};
use crate::directory::HubDirectory;

/// 🔢️ Fixed bounds every inference route enforces before it touches storage or the GIS executor.
pub const OPERATION_CAPACITY: usize = 32;
pub const DOCUMENT_GATE_CAPACITY: usize = 64;
pub const WORK_UNIT_LIMIT: u64 = 4096;
pub const ALLOCATION_BYTES: u64 = 1 << 20;
pub const RECURSION_DEPTH: u32 = 32;
pub const APPROVAL_MAX_RECORDS: u64 = 8;

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
    pub scope: &'a DocumentScope,
    pub actor: &'a str,
    pub mutation_id: &'a str,
    pub command_hash: &'a str,
    pub job_id: &'a str,
    pub proposal_hash: &'a str,
    pub command: &'a [u8],
    pub base_frontier: &'a ArtifactFrontier,
    pub deadline_ms: u64,
}

/// ⛔️ Why a typed composition publication refused; never a partial or optimistic outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GisMapApprovalCommitErrorV1 {
    Unavailable,
    Rejected,
    Conflict,
    Storage,
}

/// 🧾️ A committed publication receipt: only a real committed-WAL witness may reconcile the outbox.
pub struct GisMapApprovalReceiptV1 {
    pub witness: CommittedInferenceWalWitnessV1,
    pub document_generation: u64,
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
}

/// 🚧️ Fail-closed committer for every deployment where no composition transaction is registered.
pub struct UnavailableGisMapApprovalCommitterV1;

impl GisMapApprovalCommitterV1 for UnavailableGisMapApprovalCommitterV1 {
    fn commit<'a>(&'a self, _request: GisMapApprovalCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async { Err(GisMapApprovalCommitErrorV1::Unavailable) })
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
    pub async fn commit_approval(&self, identity: &InferenceIdentityV1, job_id: &str, proposal_hash: &str, command: &InferencePrivateBytesV1, base: &InferenceMapBaseV1, deadline_ms: u64, now_ms: u64) -> Result<bool, InferenceRouteErrorV1> {
        commit_prepared_approval(&self.committer, &self.ledger, identity, job_id, proposal_hash, command, base, deadline_ms, now_ms).await
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
    ledger: &Arc<InferenceJobLedgerV1>,
    identity: &InferenceIdentityV1,
    job_id: &str,
    proposal_hash: &str,
    command: &InferencePrivateBytesV1,
    base: &InferenceMapBaseV1,
    deadline_ms: u64,
    now_ms: u64,
) -> Result<bool, InferenceRouteErrorV1> {
    let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
    let receipt = committer
        .commit(GisMapApprovalCommitRequestV1 {
            scope: &scope,
            actor: &approval_actor(identity),
            mutation_id: &approval_mutation_id(job_id, proposal_hash),
            command_hash: &sha256(command.as_slice()),
            job_id,
            proposal_hash,
            command: command.as_slice(),
            base_frontier: &base.frontier,
            deadline_ms,
        })
        .await
        .map_err(|error| match error {
            GisMapApprovalCommitErrorV1::Unavailable => InferenceRouteErrorV1::CommitUnavailable,
            GisMapApprovalCommitErrorV1::Rejected => InferenceRouteErrorV1::Denied,
            GisMapApprovalCommitErrorV1::Conflict => InferenceRouteErrorV1::Conflict,
            GisMapApprovalCommitErrorV1::Storage => InferenceRouteErrorV1::Storage,
        })?;
    Ok(ledger.reconcile_committed_approval(job_id, &receipt.witness, receipt.document_generation, now_ms)?)
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
pub async fn approve_gis_map_job(context: InferenceRouteContextV1<'_>, job_id: &str, body: &[u8]) -> Result<InferenceApprovalReceiptDtoV1, InferenceRouteErrorV1> {
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
    let applied = runtime.commit_approval(&identity, &approval.job_id, &approval.proposal_hash, &command, &base, base_control.deadline_ms, context.now_ms).await?;
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
    use crate::inference::schema::{InferenceIdentityV1 as Identity, INPUT_MAX_BYTES, PROGRESS_MAX_CURSOR};

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
                commit_prepared_approval(&committer, &ledger, &identity, &receipt.job_id, &proposal_hash, &command, &base, 60_000, 1_005 + attempt).await,
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
