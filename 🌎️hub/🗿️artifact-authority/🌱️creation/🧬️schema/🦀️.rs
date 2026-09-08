//! 📚️ Immutable creation facts bind one intent to one private pair and one public triple.

use super::ArtifactCreationActorV1;
use crate::directory::error::{DirectoryError, DirectoryResult};
use directory::{FromValue, ToValue};
use directory::os_directory::{ArtifactCheckpoint, ArtifactHash, DocumentDescriptor, DocumentOwner, DocumentScope};
use directory::os_directory::schema::space_artifact_creation::{SpaceArtifactCreateV1, SpaceArtifactCreationDialectV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationReadyV1, SpaceArtifactCreationStatusV1};
use directory::os_io::ArtifactDialect;
use semio_framework_hash::Sha256;

//#region 🔖️ScopeSchemaExports
use semio_framework_schema_registry::{register_scope_schema_exports, FacetLeaves, SchemaExport, ScopeSchemaExports};

/// 🧬️ The scope id every export of this module resolves under.
pub const SCHEMA_SCOPE: &str = "hub.artifact-authority.creation";

/// 🏷️ The leaves of an export that is only ever validated, never transported by a Rust decoder —
/// exactly the set its `"x-semio-formats"` annotation declares (contract §B).
const VALIDATED_ONLY: FacetLeaves = FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" };

/// 📚️ The module document the annotation is read from, so the law never restates it.
#[cfg(test)]
const MODULE_JSON: &str = include_str!("🔣️.json");

/// 🏷️ `$defs` of `🔣️.json`, in declaration order.
const EXPORTS: [SchemaExport; 12] = [
    SchemaExport { id: "ArtifactCreationPhaseV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationFactKindV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationOperationStateV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationTransitionV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationCancellationDecisionV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationTransactionKindV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationTransactionOutcomeV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationAcceptedRecoveryV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationHttpLimitsV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationHttpRouteV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationHttpAuthorityV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "ArtifactCreationHttpResponseV1", leaves: VALIDATED_ONLY },
];

/// 📌️ Registers `hub.artifact-authority.creation`'s named exports into the process-wide export catalog.
/// See `📋️execution-contract.md` §C and `semio_framework_schema_registry::resolve_schema_export`.
// 🚫️async: pure registration helper (no I/O)
pub fn register_scope_exports() {
    register_scope_schema_exports(ScopeSchemaExports { scope: SCHEMA_SCOPE, exports: &EXPORTS }).expect("hub.artifact-authority.creation scope schema exports");
}
/// 🔬 Proves the registration at runtime rather than by inspection: it registers, resolves every
/// export in exactly the formats its `"x-semio-formats"` annotation names and in no other, and
/// asserts the scope is visible in the process-wide catalog.
#[cfg(test)]
#[path = "🧪️tests/🔬️scope-schema-export-law-standalone/🦀️.rs"]
mod scope_schema_export_law;
//#endregion 🔖️ScopeSchemaExports

pub const ARTIFACT_CREATION_DEADLINE_MS: u64 = 30_000;
pub const ARTIFACT_CREATION_PAIR_MAX_BYTES: usize = 1024 * 1024;
pub const ARTIFACT_CREATION_FACTS_MAX: usize = 3;

/// 🪢️ A durable accepted intent captures server-selected identity before invoking any factory.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(crate = "::directory", rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactCreationIntentV1 {
    pub actor: ArtifactCreationActorV1,
    pub scope: DocumentScope,
    pub request: SpaceArtifactCreateV1,
    pub catalog_generation: String,
    pub owner: DocumentOwner,
    pub artifact_schema: String,
    pub pack_schema_hash: String,
    pub parent_dialect: ArtifactDialect,
    pub command_sha256: String,
    pub accepted_at_ms: u64,
    pub deadline_ms: u64,
}

/// 🪺️ Prepared bytes are retained durably; recovery never invokes the initial factory again.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(crate = "::directory", rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactCreationPreparedV1 {
    pub descriptor: DocumentDescriptor,
    pub checkpoint: ArtifactCheckpoint,
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
}

/// 🧾️ Completion is inseparable from the three dense public events and exact private checkpoint.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(crate = "::directory", rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactCreationReceiptV1 {
    pub ready: SpaceArtifactCreationReadyV1,
    pub checkpoint_id: ArtifactHash,
    pub descriptor_digest_v1: ArtifactHash,
    pub event_seq_first: u64,
    pub event_seq_last: u64,
    pub event_ids: Vec<String>,
}

/// 📜️ Terminal facts never reopen a creation key or replace its captured initial bytes.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(crate = "::directory", tag = "kind", rename_all = "lowercase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum ArtifactCreationFactBodyV1 {
    Accepted { intent: ArtifactCreationIntentV1 },
    Prepared { candidate: ArtifactCreationPreparedV1 },
    Committed { receipt: ArtifactCreationReceiptV1 },
    Cancelled,
    Failed,
}

/// 🔢️ A request-local immutable revision is appended under the backend writer transaction.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(crate = "::directory", rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactCreationFactV1 {
    pub actor_user_id: String,
    pub request_id: String,
    pub revision: u64,
    pub recorded_at_ms: u64,
    pub body: ArtifactCreationFactBodyV1,
}

/// 🚦️ Read-side state is derived exclusively from validated durable facts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCreationOperationV1 {
    pub intent: ArtifactCreationIntentV1,
    pub prepared: Option<ArtifactCreationPreparedV1>,
    pub receipt: Option<ArtifactCreationReceiptV1>,
    pub phase: SpaceArtifactCreationPhaseV1,
    pub revision: u64,
}

/// 📨️ Only a newly inserted accepted fact grants this process one execution attempt.
pub struct ArtifactCreationAcceptanceV1 {
    pub status: SpaceArtifactCreationStatusV1,
    pub execution: Option<ArtifactCreationExecutionV1>,
}

/// 🎟️ A non-cloneable execution grant exists only for the newly committed accepted fact.
pub struct ArtifactCreationExecutionV1 { pub(super) intent: ArtifactCreationIntentV1 }

impl ArtifactCreationExecutionV1 {
    /// 🔎️ Inspection cannot construct, clone or renew the factory execution grant.
    pub fn intent(&self) -> &ArtifactCreationIntentV1 { &self.intent }
}

/// 🆕️ Only the transaction that inserts revision one owns factory execution.
pub enum ArtifactCreationClaimV1 {
    Accepted(ArtifactCreationOperationV1),
    Existing(ArtifactCreationOperationV1),
}

/// ✍️ A private transition must match the complete accepted key and expected revision.
pub struct ArtifactCreationFactAppendV1 {
    pub actor: ArtifactCreationActorV1,
    pub space_id: String,
    pub request_id: String,
    pub command_sha256: String,
    pub expected_revision: u64,
    pub recorded_at_ms: u64,
    pub body: ArtifactCreationFactBodyV1,
}

/// ⚛️ One reserved genesis publishes exactly three events and its private completion together.
pub struct DocumentGenesisAppendV1 {
    pub intent: ArtifactCreationIntentV1,
    pub events: [crate::directory::NewDirectoryEvent; 3],
    pub checkpoint: ArtifactCheckpoint,
    pub reservation: crate::artifact_authority::chunk_cas::ArtifactCasReservation,
    pub now_ms: u64,
}

/// 📣️ Reconciliation returns the original receipt without rebroadcasting historical events.
pub enum DocumentGenesisCommitV1 {
    Committed { events: Vec<directory::os_directory::DirectoryEvent>, operation: ArtifactCreationOperationV1 },
    Existing(ArtifactCreationOperationV1),
    Indeterminate,
}

/// 🔀️ A transition appends at most one immutable fact; retries never erase or reopen prior facts.
pub(crate) fn decide_artifact_creation_fact_append_v1(facts: &[ArtifactCreationFactV1], append: &ArtifactCreationFactAppendV1, observed_now_ms: u64) -> DirectoryResult<Option<ArtifactCreationFactV1>> {
    let operation = ArtifactCreationOperationV1::fold(facts)?;
    if append.actor.user_id != operation.intent.actor.user_id || append.space_id != operation.intent.scope.space_id || append.request_id != operation.intent.request.request_id || append.command_sha256 != operation.intent.command_sha256 { return Err(rejected("artifact creation transition belongs to another intent")); }
    let cancellation = matches!(append.body, ArtifactCreationFactBodyV1::Cancelled);
    if append.recorded_at_ms > observed_now_ms || cancellation && (append.expected_revision == 0 || append.expected_revision > operation.revision) { return Err(rejected("artifact creation transition clock or observed revision is invalid")); }
    match &append.body {
        ArtifactCreationFactBodyV1::Accepted { .. } | ArtifactCreationFactBodyV1::Committed { .. } => return Err(rejected("artifact creation transition requires its dedicated transaction")),
        ArtifactCreationFactBodyV1::Prepared { candidate } => {
            if append.actor != operation.intent.actor { return Err(rejected("artifact creation preparation session differs")); }
            candidate.validate(&operation.intent)?;
            if let Some(previous) = operation.prepared.as_ref() { return if previous == candidate { Ok(None) } else { Err(rejected("artifact creation prepared bytes cannot be replaced")) }; }
        }
        ArtifactCreationFactBodyV1::Failed if append.actor != operation.intent.actor => return Err(rejected("artifact creation failure owner differs")),
        _ => {}
    }
    if matches!(operation.phase, SpaceArtifactCreationPhaseV1::Ready | SpaceArtifactCreationPhaseV1::Cancelled | SpaceArtifactCreationPhaseV1::Failed) { return Ok(None); }
    if !cancellation && append.expected_revision != operation.revision { return Err(rejected("artifact creation transition revision changed")); }
    let next = ArtifactCreationFactV1 { actor_user_id: append.actor.user_id.clone(), request_id: append.request_id.clone(), revision: operation.revision + 1, recorded_at_ms: observed_now_ms, body: append.body.clone() };
    let mut proposed = facts.to_vec(); proposed.push(next.clone()); ArtifactCreationOperationV1::fold(&proposed)?;
    Ok(Some(next))
}

fn rejected(message: &'static str) -> DirectoryError { DirectoryError::Conflict(message.into()) }

fn text(value: &str) -> bool { !value.is_empty() && value.len() <= 256 && !value.chars().any(char::is_control) }

fn hash(value: &str) -> bool { ArtifactHash::parse_hex(value).is_some_and(|digest| digest.0 != [0; 32] && digest.hex() == value) }

/// 🔏️ Scope is length-prefixed before the canonical client intent; no route alias can collide.
pub fn artifact_creation_command_digest_v1(space_id: &str, request: &SpaceArtifactCreateV1) -> DirectoryResult<String> {
    if !text(space_id) || !request.validate() { return Err(rejected("artifact creation intent is invalid")); }
    let canonical = directory::os_pack::json::to_json_string(request);
    let mut digest = Sha256::new();
    digest.update(b"semio.hub.artifact-creation-intent.v1\0");
    for bytes in [space_id.as_bytes(), canonical.as_bytes()] {
        digest.update(&(bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    Ok(directory::os_directory::hex_lower(&digest.finalize()))
}

impl ArtifactCreationIntentV1 {
    /// 🚪️ Coordinates are derived from the accepted catalog binding, not supplied as a ready reply.
    pub fn ready(&self) -> SpaceArtifactCreationReadyV1 {
        SpaceArtifactCreationReadyV1 { document_id: self.scope.document_id.clone(), kind_id: self.request.kind_id.clone(), artifact_schema: self.artifact_schema.clone(), parent_dialect: SpaceArtifactCreationDialectV1 { artifact_kind: self.parent_dialect.artifact_kind.clone(), standard: self.parent_dialect.standard.clone(), subset: self.parent_dialect.subset.clone() } }
    }

    /// 🛡️ A stored intent is bounded, exact, and names one server-minted artifact.
    pub fn validate(&self) -> DirectoryResult<()> {
        let descriptor = DocumentDescriptor { space_id: self.scope.space_id.clone(), document_id: self.scope.document_id.clone(), artifact_kind: self.request.kind_id.clone(), artifact_schema: self.artifact_schema.clone(), owner: self.owner.clone(), pack_schema_hash: self.pack_schema_hash.clone(), bootstrap_version: 1, bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 }, bootstrap_snapshot_hash: "01".repeat(32) };
        if !text(&self.actor.user_id) || !text(&self.actor.session_id) || self.actor.authorization_generation == 0 || self.actor.authorization_generation > directory::os_directory::schema::DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || !(SpaceArtifactCreationStatusV1 { schema: "semio.hub.space-artifact-creation-status/v1".into(), request_id: self.request.request_id.clone(), space_id: self.scope.space_id.clone(), phase: SpaceArtifactCreationPhaseV1::Accepted, ready: None }).validate() || !self.ready().validate() || !hash(&self.catalog_generation)
            || self.command_sha256 != artifact_creation_command_digest_v1(&self.scope.space_id, &self.request)?
            || self.accepted_at_ms.checked_add(ARTIFACT_CREATION_DEADLINE_MS) != Some(self.deadline_ms) || self.deadline_ms > directory::os_directory::schema::DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || directory::os_directory::descriptor_digest_v1(&descriptor).is_err() { return Err(rejected("artifact creation accepted identity is invalid")); }
        Ok(())
    }
}

impl ArtifactCreationPreparedV1 {
    /// 🧬️ The private pair, descriptor and exact zero checkpoint remain bound to their accepted intent.
    pub fn validate(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<()> {
        intent.validate()?;
        let d = &self.descriptor;
        let c = &self.checkpoint;
        if self.pack.is_empty() || self.spr.is_empty() || self.pack.len().checked_add(self.spr.len()).is_none_or(|length| length > ARTIFACT_CREATION_PAIR_MAX_BYTES)
            || d.space_id != intent.scope.space_id || d.document_id != intent.scope.document_id || d.artifact_kind != intent.request.kind_id || d.artifact_schema != intent.artifact_schema || d.owner != intent.owner || d.pack_schema_hash != intent.pack_schema_hash
            || d.bootstrap_version != 1 || d.bootstrap_frontier != (directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 }) || d.bootstrap_snapshot_hash != directory::os_directory::hex_lower(&Sha256::digest(&self.pack))
            || c.scope != intent.scope || !c.baseline_frontier.is_genesis_for(&intent.scope) || c.parent_checkpoint_id.is_some()
            || c.published_at_ms < intent.accepted_at_ms || c.published_at_ms >= intent.deadline_ms
            || directory::os_directory::descriptor_digest_v1(d).ok() != Some(c.descriptor_digest_v1)
            || c.pack.sha256 != ArtifactHash(Sha256::digest(&self.pack)) || c.pack.byte_length != self.pack.len() as u64
            || c.spr.sha256 != ArtifactHash(Sha256::digest(&self.spr)) || c.spr.byte_length != self.spr.len() as u64
            || c.pack.storage_key != format!("sha256/{}", c.pack.sha256.hex()) || c.spr.storage_key != format!("sha256/{}", c.spr.sha256.hex()) { return Err(rejected("artifact creation prepared pair differs from its accepted intent")); }
        let mut aggregate = Sha256::new(); aggregate.update(&self.pack); aggregate.update(&self.spr);
        if c.aggregate_sha256 != ArtifactHash(aggregate.finalize()) || super::super::checkpoint_id_encoding_v1(c).ok().map(|bytes| ArtifactHash(Sha256::digest(&bytes))) != Some(c.checkpoint_id) { return Err(rejected("artifact creation prepared integrity differs")); }
        Ok(())
    }
}

impl ArtifactCreationOperationV1 {
    /// 📖️ Validates a complete bounded history; neither an orphan completion nor a terminal retry opens a key.
    pub fn fold(facts: &[ArtifactCreationFactV1]) -> DirectoryResult<Self> {
        if facts.is_empty() || facts.len() > ARTIFACT_CREATION_FACTS_MAX { return Err(rejected("artifact creation fact count is invalid")); }
        let ArtifactCreationFactBodyV1::Accepted { intent } = &facts[0].body else { return Err(rejected("artifact creation history has no accepted intent")); };
        intent.validate()?;
        let mut result = Self { intent: intent.clone(), prepared: None, receipt: None, phase: SpaceArtifactCreationPhaseV1::Accepted, revision: 0 };
        let mut timestamp = intent.accepted_at_ms;
        for (index, fact) in facts.iter().enumerate() {
            if fact.actor_user_id != intent.actor.user_id || fact.request_id != intent.request.request_id || fact.revision != index as u64 + 1 || fact.recorded_at_ms < timestamp || fact.recorded_at_ms > directory::os_directory::schema::DOCUMENT_OPEN_MAX_SAFE_INTEGER { return Err(rejected("artifact creation fact identity or order differs")); }
            match (&fact.body, result.phase, index) {
                (ArtifactCreationFactBodyV1::Accepted { .. }, _, 0) if fact.recorded_at_ms == intent.accepted_at_ms => {}
                (ArtifactCreationFactBodyV1::Prepared { candidate }, SpaceArtifactCreationPhaseV1::Accepted, _) => {
                    candidate.validate(intent)?;
                    if fact.recorded_at_ms < candidate.checkpoint.published_at_ms || fact.recorded_at_ms >= intent.deadline_ms { return Err(rejected("artifact creation preparation expired")); }
                    result.prepared = Some(candidate.clone()); result.phase = SpaceArtifactCreationPhaseV1::Preparing;
                }
                (ArtifactCreationFactBodyV1::Committed { receipt }, SpaceArtifactCreationPhaseV1::Preparing, _) => {
                    let candidate = result.prepared.as_ref().ok_or_else(|| rejected("artifact creation completion is unprepared"))?;
                    if receipt.ready != intent.ready() || receipt.checkpoint_id != candidate.checkpoint.checkpoint_id || receipt.descriptor_digest_v1 != candidate.checkpoint.descriptor_digest_v1 || receipt.event_seq_first == 0 || receipt.event_seq_first.checked_add(2) != Some(receipt.event_seq_last) || receipt.event_seq_last > directory::os_directory::schema::DOCUMENT_OPEN_MAX_SAFE_INTEGER || receipt.event_ids.len() != 3 || receipt.event_ids.iter().any(|id| !text(id) || !id.is_ascii()) || receipt.event_ids.iter().collect::<std::collections::BTreeSet<_>>().len() != 3 { return Err(rejected("artifact creation completion differs from its prepared triple")); }
                    result.receipt = Some(receipt.clone()); result.phase = SpaceArtifactCreationPhaseV1::Ready;
                }
                (ArtifactCreationFactBodyV1::Cancelled, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing, _) => result.phase = SpaceArtifactCreationPhaseV1::Cancelled,
                (ArtifactCreationFactBodyV1::Failed, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing, _) => result.phase = SpaceArtifactCreationPhaseV1::Failed,
                _ => return Err(rejected("artifact creation fact transition is invalid")),
            }
            result.revision = fact.revision; timestamp = fact.recorded_at_ms;
        }
        Ok(result)
    }

    /// 📬️ Private bytes and authority identities never appear in a creation status response.
    pub fn status(&self) -> SpaceArtifactCreationStatusV1 {
        SpaceArtifactCreationStatusV1 { schema: "semio.hub.space-artifact-creation-status/v1".into(), request_id: self.intent.request.request_id.clone(), space_id: self.intent.scope.space_id.clone(), phase: self.phase, ready: self.receipt.as_ref().map(|receipt| receipt.ready.clone()) }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
