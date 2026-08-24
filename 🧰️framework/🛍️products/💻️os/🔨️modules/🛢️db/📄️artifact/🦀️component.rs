//! 🗄️ `db_artifact` — the document authority actor and its command pipeline: admit → dedupe →
//! base-resolve → authz → deps → validate → conflict → execute → WAL append → durability →
//! publish → project → vcs → preview-reconcile → receipt. Composes `db_state` (materialized
//! overlay), `db_wal` (durability), `db_storage` (the pluggable substrate), and `protocol`
//! (`MutationEnvelope`/`MutationDiff`) into `ArtifactEngine`, the crate's central type, plus a
//! thin `db_actor`-mailbox wrapper (`ArtifactAuthority`) around it. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_artifact` row).
//!
//! 🎯️ Design choice (compatibility surface — re-checked against `db_engine`'s live state, which
//! was itself being revised concurrently while this file was authored): `db_engine`'s
//! `document_engine_config` now builds `ArtifactEngineConfig{limits, security, version_graph,
//! emit, ..ArtifactEngineConfig::default()}` (a real `db_security::SecurityGate` baked directly
//! in, `version_graph: Arc<dyn VersionGraph>` non-optional, defaulted `..` spread for the rest),
//! `CommandReceipt.conflicts: Vec<db_conflict::ConflictRecord>` (the real type), and
//! `ArtifactAuthority::run_query(query, consistency)` (two arguments, `db_query::
//! Consistency`-aware). This revision matches that shape exactly: `security`/`emit` are real
//! `ArtifactEngineConfig` fields, `version_graph` is required (`NullVersionGraph` is the
//! "no vcs" default rather than `Option::None`), `submit`'s conflict step is a genuine
//! `db_conflict::ConflictDetector` fed by retained recent-commit `TouchedSet` history (not a local
//! last-writer stand-in), and `query`/`RunQuery`/`run_query` take a `db_query::
//! Consistency` and resolve it via `db_query::resolve_consistency` + `db_index::
//! IndexConsistencyResolver`. `AuthzHook`/`AllowAll` are kept defined (unused in the hot `submit`
//! path now that `security` supersedes them) purely because they are still a public, documented
//! extension seam and cost nothing to keep — a caller may still hand-roll one. Because
//! `ArtifactEngineConfig`'s new fields are absorbed via `..Default::default()` at every call site
//! observed, `db_projection` registration is also wired in as a `projections` factory field (see
//! `🔖️Engine`'s doc for why a factory, not a stored engine).
//!
//! 🎯️ Design choice (diff convention, unchanged): `protocol::MutationEnvelope`'s `diff`/`inverse`
//! payloads are schema-erased `dsl::DslValue` pathmaps encoded with `store::pack_rt::encode_wire_value`
//! — `db_artifact` has no compile-time knowledge of
//! any concrete document schema, so it adopts one generic convention for BOTH: a JSON *object*
//! whose keys are `db_state`-style `/`-segmented paths and whose values are either the new JSON
//! value to set at that path, or JSON `null` as an explicit tombstone (path deleted). This is a
//! real, documented limitation, not a workaround: a legitimate application-level `null` value is
//! indistinguishable from a delete under this convention. `envelope_from_operation` (new this
//! revision) is the generic ingestion boundary that actually exercises `protocol::Mutation`/
//! `MutationDiff` (`diff`/`apply`/`mutation_id`/`dependencies`/`author_id`/`timestamp`) to build
//! an envelope in this same convention from a typed operation — the one place in the `db` family
//! below `db_artifact` allowed to interpret operation semantics at all (per the contract's hard
//! dependency rule).

use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::db_durability::Frontier;
use crate::db_ids::*;
use crate::*;
use protocol::MutationDiff as _;

use dsl::DslValue;

//#region 🔖️Ids
/// @emoji 🌉️ `protocol::ArtifactId` → `ArtifactId`, the lossless single-`String` bridge
/// `db_core`'s module doc promises (see `ArtifactId`'s own doc for the rationale: this
/// crate is the first one in the family that depends on both `db_core` and `protocol` and so is
/// where the bridge is actually exercised).
async fn to_core_document_id(id: &protocol::ArtifactId) -> ArtifactId {
    ArtifactId(id.0.clone())
}

/// @emoji 🌉️ `protocol::ActorId` → `ActorId`, same bridge as `to_core_document_id`.
async fn to_core_actor_id(id: &protocol::ActorId) -> ActorId {
    ActorId(id.0.clone())
}

// 🔒️ Used as a bare fn-pointer error mapper (`.map_err(json_err)`) throughout this file —
// `Result::map_err`'s `FnOnce(E) -> F2` bound always calls the mapper with an owned `E`, so a
// by-reference signature would not type-check at any of those call sites despite the function
// body itself only borrowing `err` to format it.
// 🚫️async: E4 fn-pointer slot
fn wire_err(err: store::PackError) -> DbError {
    DbError::InvalidArgument(format!("db_artifact wire error: {err}"))
}

// 🚫️async: E4 fn-pointer slot
fn dsl_err(err: String) -> DbError {
    DbError::InvalidArgument(format!("db_artifact dsl error: {err}"))
}

//#region 🔖️Command
/// @emoji 📦️ One atomically-submitted group of causally-related operations against a single
/// document — the unit `ArtifactEngine::submit` accepts. Every envelope must target the same
/// `document_id` (checked at construction, and again against the engine's own document at submit
/// time); the batch's `command_id` (for dedupe/the returned `CommandReceipt`) is its LAST
/// envelope's `mutation_id`, since `MutationEnvelope` has no separate batch-level id of its own.
pub struct CommandBatch {
    pub envelopes: Vec<protocol::MutationEnvelope>,
}

impl CommandBatch {
    /// @emoji 🏗️ Builds a batch, rejecting an empty one or one whose envelopes disagree on
    /// `document_id`.
    pub async fn new(envelopes: Vec<protocol::MutationEnvelope>) -> Result<CommandBatch, DbError> {
        let first = envelopes.first().ok_or_else(|| DbError::InvalidArgument("command batch must contain at least one operation".to_string()))?;
        let document_id = first.document_id.clone();
        if envelopes.iter().any(|envelope| envelope.document_id != document_id) {
            return Err(DbError::InvalidArgument("every envelope in a command batch must target the same document".to_string()));
        }
        Ok(CommandBatch { envelopes })
    }
}

/// @emoji 🎚️ Per-submit durability override — see `DurabilityClass`'s doc for the
/// strength ordering `ArtifactWal::submit` honors. `policy` is the authority-local `protocol::
/// MergePolicy` `submit`'s outcome step judges this batch's worst graded conflict/message level
/// against (contract §C9) — never carried on the wire, never part of shared history (see
/// `protocol::MergePolicy`'s own doc).
#[derive(Clone, Copy, Debug)]
pub struct SubmitOptions {
    pub durability: DurabilityClass,
    pub policy: protocol::MergePolicy,
}

impl Default for SubmitOptions {
    fn default() -> Self {
        SubmitOptions { durability: DurabilityClass::Memory, policy: protocol::MergePolicy::default() }
    }
}
//#endregion 🔖️Command

//#region 🔖️Diff
/// @emoji 🧬️ The schema tag `db_artifact` reserves for its own generic path-value diff
/// convention (see module doc) — the only `ArtifactDiff`/`InverseMutation` shape this crate
/// knows how to interpret. 🎯️ W5: `diff`/`inverse` payloads are opaque `Vec<u8>` on the wire now;
/// `db_artifact` still only understands ITS OWN JSON-object-of-paths convention, tagged with this
/// schema so `diff_entries`/`inverse_entries` can distinguish "our own pathmap bytes" from a
/// typed op's binary payload it has no business decoding (foreign schema -> empty `TouchedSet`,
/// not an error — the envelope is still persisted/relayed, just not interpreted at this layer;
/// that's `db_artifact`-and-above's future typed path).
pub const DB_PATHMAP_SCHEMA: &str = "db.pathmap.v1";

/// @emoji 🎯️ `DslValue` object pathmap -> `store::pack_rt::encode_wire_value` bytes.
async fn encode_pathmap(value: &DslValue) -> Vec<u8> {
    store::pack_rt::encode_wire_value(value)
}

/// @emoji 🎯️ Inverse of `encode_pathmap`.
async fn decode_pathmap(bytes: &[u8]) -> Result<DslValue, DbError> {
    store::pack_rt::decode_wire_value(bytes).map_err(wire_err)
}

/// @emoji 🧰️ Public convenience for every crate above this one that hand-builds a `DB_PATHMAP_SCHEMA`
/// `MutationEnvelope` (test fixtures, `db_cli`'s `profile`/`migrate` commands, `db_testkit`'s
/// workload generators) rather than going through `envelope_from_operation`: encodes a
/// `serde_json::Value::Object` the same way `decode_pathmap`/`apply_one` decode it. Centralizing
/// this is the single source of truth for `DB_PATHMAP_SCHEMA`'s actual wire bytes — a caller that
/// instead hand-rolls `serde_json::to_vec` produces bytes `decode_pathmap` cannot read (it expects
/// `store::pack_rt`'s binary encoding, not raw JSON text), which is exactly the "wire error:
/// truncated" bug this function exists to make structurally impossible to repeat.
pub async fn encode_pathmap_json(value: &serde_json::Value) -> Result<Vec<u8>, DbError> {
    Ok(encode_pathmap(&dsl::to_dsl_value(value).map_err(dsl_err)?).await)
}

/// @emoji 🧰️ Inverse of `encode_pathmap_json` — also the general "read back one stored/queried
/// value's bytes as JSON" decode every caller above this crate needs: `ArtifactEngine::get`/
/// `preview_get` and `db_engine::ArtifactHandle::query` all hand back these same `store::pack_rt`
/// wire bytes (single value OR whole pathmap object, both are just a `DslValue` tree to this
/// codec), never raw JSON text — a caller reaching for `serde_json::from_slice` on them directly
/// hits exactly the "expected value" parse error this function exists to make impossible.
pub async fn decode_pathmap_json(bytes: &[u8]) -> Result<serde_json::Value, DbError> {
    dsl::from_dsl_value(decode_pathmap(bytes).await?).map_err(dsl_err)
}

/// @emoji 🧮️ Flattens a diff/inverse pathmap object into `(path, Some(value) | None)` pairs per this
/// module's generic path-value convention (see module doc). Errors if `value` is not an object —
/// this crate's own schema-erased documents have no other shape it can interpret.
async fn entries_from_value(value: &DslValue) -> Result<Vec<(String, Option<DslValue>)>, DbError> {
    let object = value.as_object().ok_or_else(|| DbError::InvalidArgument("diff/inverse payload must be a pathmap object".to_string()))?;
    Ok(object.iter().map(|(path, entry)| (path.clone(), if entry.is_null() { None } else { Some(entry.clone()) })).collect())
}

/// @emoji ➡️ Entries for an envelope's forward diff — empty (not an error) for any schema other
/// than `DB_PATHMAP_SCHEMA`, see its doc.
async fn diff_entries(diff: &protocol::ArtifactDiff) -> Result<Vec<(String, Option<DslValue>)>, DbError> {
    if diff.schema.0 != DB_PATHMAP_SCHEMA {
        return Ok(Vec::new());
    }
    entries_from_value(&decode_pathmap(&diff.payload).await?).await
}

/// @emoji ↩️ Entries for an envelope's inverse diff (the `undo` pipeline's source) — same
/// foreign-schema handling as `diff_entries`.
async fn inverse_entries(inverse: &protocol::InverseMutation) -> Result<Vec<(String, Option<DslValue>)>, DbError> {
    if inverse.schema.0 != DB_PATHMAP_SCHEMA {
        return Ok(Vec::new());
    }
    entries_from_value(&decode_pathmap(&inverse.payload).await?).await
}

/// @emoji 🧮️ The inverse of `entries_from_value` — rebuilds a JSON object from path-value pairs,
/// `None` becoming an explicit `null` tombstone. Used by `undo` to construct a compensating
/// envelope's diff/inverse payloads.
// 🚫️async: E1 pure accessor, always used as `&entries_to_value(...)` inline into another call — see R9
fn entries_to_value(entries: &[(String, Option<DslValue>)]) -> DslValue {
    DslValue::Object(entries.iter().map(|(path, value)| (path.clone(), value.clone().unwrap_or(DslValue::Null))).collect())
}

/// @emoji 👣️ The `TouchedSet` a set of entries would write — shared by `DocumentState::
/// apply_entries` and preview publishing.
// 🚫️async: E1 pure accessor, `db_state::TouchedSet`'s own methods are sync — see R9
fn entries_touched(entries: &[(String, Option<DslValue>)]) -> db_state::TouchedSet {
    let mut touched = db_state::TouchedSet::new();
    for (path, _) in entries {
        touched.record(db_state::TouchedRegion::write(path.clone()));
    }
    touched
}
//#endregion 🔖️Diff

//#region 🔖️Bridge
/// @emoji 🌉️ The generic ingestion boundary: builds an `MutationEnvelope` (in this crate's own
/// path-value diff convention) from a typed `protocol::Mutation<P>` against a serializable
/// projection `P`, writing the whole post-state at `path`. Genuinely exercises `Mutation`/
/// `MutationDiff`'s trait methods (`diff`/`apply`/`mutation_id`/`dependencies`/`author_id`/
/// `timestamp`) — see the module doc's design-choice note on why this crate is allowed to. A
/// caller wanting per-sub-path granularity builds the JSON object directly via
/// `CommandBatch::new`/`entries_to_value` instead of this whole-projection convenience.
pub async fn envelope_from_operation<P, Op>(
    document: protocol::ArtifactId,
    path: &str,
    op: &Op,
    base: &P,
    default_actor: protocol::ActorId,
    default_mutation_id: protocol::MutationId,
    default_timestamp: protocol::HybridLogicalTimestamp,
) -> Result<protocol::MutationEnvelope, DbError>
where
    P: serde::Serialize,
    Op: protocol::Mutation<P>,
{
    let diff = op.diff(base);
    let post = diff.diff().apply(base).map_err(|error| DbError::InvalidArgument(error.to_string()))?;
    let forward = DslValue::Object(vec![(path.to_string(), dsl::to_dsl_value(&post).map_err(dsl_err)?)]);
    let backward = DslValue::Object(vec![(path.to_string(), dsl::to_dsl_value(base).map_err(dsl_err)?)]);
    let schema = protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string());
    Ok(protocol::MutationEnvelope {
        mutation_id: op.mutation_id().unwrap_or(default_mutation_id),
        document_id: document,
        actor: op.author_id().unwrap_or(default_actor),
        dependencies: op.dependencies(),
        diff: protocol::ArtifactDiff { schema: schema.clone(), payload: encode_pathmap(&forward).await },
        inverse: protocol::InverseMutation { schema, payload: encode_pathmap(&backward).await },
        timestamp: op.timestamp().unwrap_or(default_timestamp),
    })
}
//#endregion 🔖️Bridge

//#region 🔖️Conflict
/// @emoji ⚔️ One detected write/write intersection between the currently-executing operation and
/// an earlier operation it did not declare as a `dependency`. Detected directly off
/// `db_state::TouchedRegion::path_intersects` (see `DocumentState::apply_entries`). Resolution
/// policy is last-writer-wins (the conflicting write still applies) — recorded for the caller's
/// visibility only, never rejected, since every commit in a single-document-actor pipeline is
/// already fully serialized by the time this runs. Kept a local, path-granular shape (rather than
/// `db_conflict::ConflictRecord`, which reports per conflicting COMMAND PAIR) because it is
/// `db_engine`'s own frozen `CommandReceipt.conflicts` element type — see module doc.
/// `preview_conflicts` (below) is the real, additive `db_conflict::ConflictDetector` integration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConflictRecord {
    pub command_id: protocol::MutationId,
    pub conflicting_with: protocol::MutationId,
    pub path: String,
}

/// @emoji ⚔️ Builds the `db_conflict::CommandTouch` `envelope` would produce, without applying it —
/// shared by `submit`'s outcome-step gate and `preview_conflicts`'s advisory `db_conflict::
/// ConflictDetector` use. No per-operation conflict-declaration tag anymore (C10 deleted the CRDT-era
/// vocabulary): a `CommandTouch` carries only what `db_conflict::ConflictDetector` actually needs to detect —
/// identity, kind, timestamp, and the regions it touched.
// 🚫️async: E1 pure accessor consumed by a sync Iterator::map closure — see R9
fn command_touch(envelope: &protocol::MutationEnvelope, touched: &db_state::TouchedSet) -> db_conflict::CommandTouch {
    let touch = db_conflict::CommandTouch::new(envelope.mutation_id.clone(), envelope.actor.clone(), db_conflict::CommandKind::from(envelope.diff.schema.0.as_str()), envelope.timestamp);
    touched.regions.iter().fold(touch, |touch, region| touch.touch(region.clone()))
}

/// @emoji ⚖️ Grades one `db_conflict::ConflictRecord` into the `protocol::MutationMessage` the
/// outcome step judges against `options.policy` (contract §C9: "region intersection = `Warning`;
/// constraint violation = `Fatal`") — `db_conflict` deliberately never grades its own findings (see
/// its module doc), so `db_artifact`, the first crate below it that actually decides what to DO
/// about a conflict, is where that grading belongs. `TouchedRegion` still lands (last-writer-wins,
/// see `🔖️Conflict`'s doc) so it reads as an adjusted-but-applied write (`mutation.clamped`); a
/// violated `Constraint` reads as a broken structural invariant (`mutation.invariant`) — the two
/// `Warning`/`Fatal` codes from the frozen 7 that fit each shape.
async fn grade_conflict_record(record: &db_conflict::ConflictRecord) -> protocol::MutationMessage {
    match &record.kind {
        db_conflict::ConflictKind::TouchedRegion(regions) => {
            let target: Vec<String> = regions.iter().map(|region| region.path.clone()).collect();
            protocol::MutationMessage::warn("mutation.clamped", format!("command {} touches region(s) also touched by concurrent command {}", record.command_id.0, record.conflicting_with.0)).at(target)
        }
        db_conflict::ConflictKind::Constraint(description) => {
            protocol::MutationMessage::fatal("mutation.invariant", format!("command {} violates constraint '{description}' held by concurrent command {}", record.command_id.0, record.conflicting_with.0)).at([description.clone()])
        }
    }
}
//#endregion 🔖️Conflict

//#region 🔖️Receipt
/// @emoji 🧾️ What `ArtifactEngine::submit` returns: the committed batch's identity, the document's
/// new `Frontier`, the durability actually requested, any detected conflicts, and the post-commit
/// state's content hash. Mirrors the `db` facade's frozen `CommandReceipt` shape, except `frontier`
/// is `Frontier` (this crate's own internal currency) rather than the facade's
/// `protocol::ArtifactId`-keyed twin — the facade converts via `to_core_document_id`'s inverse at
/// its own boundary (see module doc's bridge note).
#[derive(Clone, Debug, PartialEq)]
pub struct CommandReceipt {
    pub command_id: protocol::MutationId,
    pub frontier: Frontier,
    pub durability: DurabilityClass,
    pub conflicts: Vec<ConflictRecord>,
    pub state_hash: Option<ContentHash>,
    /// @emoji 📨️ Every `protocol::MutationMessage` the outcome step graded this batch's
    /// `db_conflict::ConflictRecord`s into (contract §C9) — present even on an accepted-but-degraded
    /// commit (`options.policy` let a `Warning`-or-below worst level through), empty on a clean one.
    pub messages: Vec<protocol::MutationMessage>,
}

/// @emoji 📤️ One committed operation's opaque effect bytes, queued for downstream
/// replication/notification (`db_sync`/`db_engine`'s concern to actually drain and ship — this
/// crate only accumulates and hands them out via `ArtifactEngine::drain_outbox`).
#[derive(Clone, Debug)]
pub struct OutboxEntry {
    pub mutation_id: protocol::MutationId,
    pub bytes: Vec<u8>,
}

/// @emoji 📣️ One commit's live-query-relevant summary — `ArtifactEngine::commit_log` accumulates
/// these so a poll-based subscriber can diff its last-seen index against the log to discover what
/// changed, without this crate needing an actual push/subscribe transport of its own. `db_query`'s
/// `LiveQuery` (see `🔖️Query`, new this revision) is the push-shaped sibling of this same signal.
#[derive(Clone, Debug)]
pub struct CommitNotification {
    pub frontier: Frontier,
    pub operation_ids: Vec<protocol::MutationId>,
    pub touched: db_state::TouchedSet,
}
//#endregion 🔖️Receipt

//#region 🔖️State
async fn admit_wal_bytes(source: Vec<u8>, maximum: u64, control: &mut db_wal::WalCursorControl) -> Result<db_wal::WalBytes, DbError> {
    match db_wal::WalBytes::try_admit(source, maximum, control).await {
        Ok(bytes) => Ok(bytes),
        Err(mut rejected) => {
            control.grant()?;
            let _ = rejected.close_step()?;
            Err(rejected.into_error())
        }
    }
}

async fn push_wal_record(records: &mut db_wal::WalRecordBatch, record: db_wal::WalRecord, control: &mut db_wal::WalCursorControl) -> Result<(), DbError> {
    match records.push(record) {
        Ok(()) => Ok(()),
        Err(mut record) => {
            control.grant()?;
            let _ = record.close_step()?;
            drop(record);
            Err(DbError::LimitExceeded("db_artifact fixed wal record batch"))
        }
    }
}

fn decode_retained_envelope(bytes: &db_wal::WalBytes, control: &mut db_wal::WalCursorControl) -> Result<protocol::MutationEnvelope, DbError> {
    let mut cursor = bytes.cursor();
    let mutation_id = protocol::MutationId(cursor.text(4_096, control)?);
    let document_id = protocol::ArtifactId(cursor.text(4_096, control)?);
    let actor = protocol::ActorId(cursor.text(4_096, control)?);
    let count = cursor.varint(control)?;
    check_len(count, 65_536, "artifact wal envelope dependencies")?;
    let mut dependencies = Vec::with_capacity(count as usize);
    for _ in 0..count {
        dependencies.push(protocol::MutationId(cursor.text(4_096, control)?));
    }
    let diff_schema = protocol::SchemaId(cursor.text(4_096, control)?);
    let diff_payload = decode_protocol_field(&mut cursor, 256 * 1024 * 1024, control)?;
    let inverse_schema = protocol::SchemaId(cursor.text(4_096, control)?);
    let inverse_payload = decode_protocol_field(&mut cursor, 256 * 1024 * 1024, control)?;
    let timestamp = protocol::HybridLogicalTimestamp { actor: cursor.varint(control)?, physical_ms: cursor.varint(control)?, logical: cursor.varint(control)? };
    if cursor.remaining() != 0 {
        return Err(DbError::Corrupt("wal command envelope has trailing bytes".to_string()));
    }
    Ok(protocol::MutationEnvelope {
        mutation_id,
        document_id,
        actor,
        dependencies,
        diff: protocol::ArtifactDiff { schema: diff_schema, payload: diff_payload },
        inverse: protocol::InverseMutation { schema: inverse_schema, payload: inverse_payload },
        timestamp,
    })
}

fn decode_protocol_field(cursor: &mut db_wal::WalBytesCursor<'_>, maximum: u64, control: &mut db_wal::WalCursorControl) -> Result<Vec<u8>, DbError> {
    let mut remaining = cursor.begin_field(maximum, control)?;
    let mut output = Vec::with_capacity(remaining);
    let mut fragment = [0u8; 4096];
    while remaining != 0 {
        let copied = cursor.read_field_fragment(&mut remaining, &mut fragment, control)?;
        output.extend_from_slice(&fragment[..copied]);
    }
    Ok(output)
}

//#region 🔖️StateRetirement
const ARTIFACT_STATE_RETIREMENT_SLOTS: usize = 64;

/// @emoji 🧹️ Persists one rejected staging graph and advances exactly one refusal,
/// source, slot, page, or text owner for each maintenance grant.
struct ArtifactStateRetirementCursor {
    rejected: Option<db_state::StateEntryRejected>,
    staged: [Option<db_state::StateEntry>; 64],
    phase: u8,
    slot: u8,
}

impl ArtifactStateRetirementCursor {
    fn rejected(rejected: db_state::StateEntryRejected, staged: [Option<db_state::StateEntry>; 64]) -> Self {
        Self { rejected: Some(rejected), staged, phase: 0, slot: 0 }
    }

    fn entry(entry: db_state::StateEntry) -> Self {
        let mut staged = std::array::from_fn(|_| None);
        staged[0] = Some(entry);
        Self { rejected: None, staged, phase: 2, slot: 0 }
    }

    fn empty() -> Self {
        Self { rejected: None, staged: std::array::from_fn(|_| None), phase: 3, slot: 64 }
    }

    fn close_step(&mut self) -> Result<bool, DbError> {
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1)?;
        control.grant()?;
        match self.phase {
            0 => {
                let rejected = self.rejected.as_mut().ok_or_else(|| DbError::Internal("artifact retirement lost refusal owner".to_string()))?;
                if !rejected.close_step()? {
                    self.phase = 1;
                }
                Ok(true)
            }
            1 => {
                let source = self.rejected.take().and_then(db_state::StateEntryRejected::into_source);
                drop(source);
                self.phase = 2;
                Ok(true)
            }
            2 => {
                let Some(entry) = self.staged.get_mut(self.slot as usize) else {
                    self.phase = 3;
                    return Ok(true);
                };
                if let Some(entry) = entry.as_mut() {
                    if entry.close_step()? {
                        return Ok(true);
                    }
                }
                *entry = None;
                self.slot += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == 3 && self.rejected.is_none() && self.staged.iter().all(Option::is_none)
    }
}

static ARTIFACT_STATE_RETIREMENT: std::sync::Mutex<[Option<ArtifactStateRetirementCursor>; ARTIFACT_STATE_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; ARTIFACT_STATE_RETIREMENT_SLOTS]);
static ARTIFACT_STATE_RETIREMENT_OVERFLOW: std::sync::Mutex<[Option<ArtifactStateRetirementCursor>; ARTIFACT_STATE_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; ARTIFACT_STATE_RETIREMENT_SLOTS]);
static ARTIFACT_STATE_RETIREMENT_QUARANTINE: std::sync::Mutex<[Option<ArtifactStateRetirementCursor>; ARTIFACT_STATE_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; ARTIFACT_STATE_RETIREMENT_SLOTS]);
static ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn retire_artifact_state_owner(owner: ArtifactStateRetirementCursor) -> Result<(), ArtifactStateRetirementCursor> {
    let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(slot) = retired.iter_mut().find(|slot| slot.is_none()) {
        *slot = Some(owner);
        Ok(())
    } else {
        drop(retired);
        ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        let mut overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(slot) = overflow.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(owner);
            return Ok(());
        }
        drop(overflow);
        let mut quarantine = ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = quarantine.iter_mut().find(|slot| slot.is_none()) else { return Err(owner) };
        *slot = Some(owner);
        Ok(())
    }
}

fn artifact_state_return_to_leaf_authorities(mut owner: ArtifactStateRetirementCursor) {
    let rejected = owner.rejected.take();
    let staged = std::mem::replace(&mut owner.staged, std::array::from_fn(|_| None));
    owner.phase = 3;
    drop(owner);
    drop(rejected);
    drop(staged);
}

fn retire_artifact_state_owner_or_recover(owner: ArtifactStateRetirementCursor) {
    if let Err(owner) = retire_artifact_state_owner(owner) {
        artifact_state_return_to_leaf_authorities(owner);
    }
}

pub fn artifact_state_retirement_maintenance_step() -> Result<bool, DbError> {
    let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = retired.iter_mut().find(|slot| slot.is_some()) else {
        drop(retired);
        let mut overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let owner = quarantine[index].take().ok_or_else(|| DbError::Internal("artifact quarantine retirement changed owner".to_string()))?;
            drop(quarantine);
            let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Internal("artifact retirement primary refilled before quarantine recovery".to_string()))?;
            *slot = Some(owner);
            return Ok(true);
        };
        let owner = overflow[index].take().ok_or_else(|| DbError::Internal("artifact overflow retirement changed owner".to_string()))?;
        drop(overflow);
        let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = retired.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| DbError::Internal("artifact retirement primary refilled before overflow recovery".to_string()))?;
        *slot = Some(owner);
        return Ok(true);
    };
    let owner = slot.as_mut().ok_or_else(|| DbError::Internal("artifact retirement changed owner".to_string()))?;
    if owner.close_step()? {
        return Ok(true);
    }
    *slot = None;
    Ok(true)
}

impl Drop for ArtifactStateRetirementCursor {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            retire_artifact_state_owner_or_recover(std::mem::replace(self, Self::empty()));
        }
    }
}
//#endregion 🔖️StateRetirement

/// @emoji 🏗️ A document's materialized state: a flat `db_state::PMap` from path to raw value
/// bytes, plus a per-path last-writer map for `submit`'s local, path-granular conflict detection
/// (see `🔖️Conflict`'s doc on why this stays local rather than `db_conflict`-backed). `values` uses
/// `PMap` (not a mutable `HashMap`) specifically so `content_hash` — the `Frontier.chain_hash`
/// source — is a real content-addressed digest of the whole state, not an incidental byte count,
/// and so `PMap::iter` gives `snapshot_now`/`query` a cheap, complete enumeration.
struct DocumentState {
    values: db_state::RetainedStateMap,
    last_writer: db_state::PMap<String, protocol::MutationId>,
}

impl DocumentState {
    // 🚫️async: E1 pure constructor, `db_state::PMap::new` is sync — see R9
    fn new() -> DocumentState {
        DocumentState { values: db_state::RetainedStateMap::new(), last_writer: db_state::PMap::new() }
    }

    fn get(&self, path: &str) -> Option<&db_storage::DbIoPages> {
        self.values.get(path)
    }

    async fn content_hash(&self) -> Result<ContentHash, DbError> {
        let mut control = db_state::StateCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        self.values.content_hash(&mut control).await
    }

    /// @emoji ✍️ Applies one envelope's flattened path-value entries, returning the new state, the
    /// `TouchedSet` it wrote, and any conflicts (a path whose last writer is neither `mutation_id`
    /// itself nor a declared `dependencies` member).
    async fn apply_entries(&mut self, mutation_id: &protocol::MutationId, dependencies: &[protocol::MutationId], entries: &[(String, Option<DslValue>)]) -> Result<(db_state::TouchedSet, Vec<ConflictRecord>), DbError> {
        check_len(entries.len() as u64, 64, "db_artifact::retained_state_mutations")?;
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let mut staged: [Option<db_state::StateEntry>; 64] = std::array::from_fn(|_| None);
        for (index, (path, value)) in entries.iter().enumerate() {
            let Some(dsl_value) = value else { continue };
            let bytes = store::pack_rt::encode_wire_value(dsl_value);
            match db_state::StateEntry::try_admit(path, bytes, MAX_STATE_PAGE_VALUE_BYTES, &mut control).await {
                Ok(entry) => staged[index] = Some(entry),
                Err(rejected) => {
                    let error = format!("retained state admission failed: {}", rejected.error());
                    retire_artifact_state_owner_or_recover(ArtifactStateRetirementCursor::rejected(rejected, staged));
                    let _ = artifact_state_retirement_maintenance_step()?;
                    return Err(DbError::InvalidArgument(error));
                }
            }
        }
        let mut touched = db_state::TouchedSet::new();
        let mut conflicts = Vec::new();
        for (index, (path, value)) in entries.iter().enumerate() {
            if let Some(previous_writer) = self.last_writer.get(path) {
                if previous_writer != mutation_id && !dependencies.contains(previous_writer) {
                    conflicts.push(ConflictRecord { command_id: mutation_id.clone(), conflicting_with: previous_writer.clone(), path: path.clone() });
                }
            }
            match value {
                Some(_) => {
                    let entry = staged[index].take().ok_or_else(|| DbError::Internal("retained state staging lost entry".to_string()))?;
                    let replaced = match self.values.insert(entry) {
                        Ok(replaced) => replaced,
                        Err(rejected) => {
                            retire_artifact_state_owner_or_recover(ArtifactStateRetirementCursor::entry(rejected));
                            let _ = artifact_state_retirement_maintenance_step()?;
                            return Err(DbError::LimitExceeded("retained state entries"));
                        }
                    };
                    if let Some(replaced) = replaced {
                        retire_artifact_state_owner_or_recover(ArtifactStateRetirementCursor::entry(replaced));
                        let _ = artifact_state_retirement_maintenance_step()?;
                    }
                }
                None => {
                    if let Some(removed) = self.values.remove(path) {
                        retire_artifact_state_owner_or_recover(ArtifactStateRetirementCursor::entry(removed));
                        let _ = artifact_state_retirement_maintenance_step()?;
                    }
                }
            }
            touched.record(db_state::TouchedRegion::write(path.clone()));
            self.last_writer = self.last_writer.insert(path.clone(), mutation_id.clone());
        }
        Ok((touched, conflicts))
    }
}
//#endregion 🔖️State

//#region 🔖️Hooks
/// @emoji 🛂️ The authorization seam `ArtifactEngine::submit` calls once per envelope, before
/// executing it — kept as its own narrow trait (rather than a direct `db_security` dependency) so a
/// real deployment supplies whatever backend it wants at `ArtifactEngineConfig` construction time.
/// `db_engine`'s `SecurityAuthzHook` is the real `db_security::SecurityGate`-backed implementation.
pub trait AuthzHook: Send + Sync {
    async fn authorize(&self, actor: &protocol::ActorId, envelope: &protocol::MutationEnvelope) -> Result<(), DbError>;
}

/// @emoji 🟢️ The default `AuthzHook`: authorizes everything. Correct for a single-tenant/test
/// deployment with no authorization policy configured; a real multi-tenant deployment must supply
/// its own hook.
#[derive(Clone, Copy, Default, Debug)]
pub struct AllowAll;

impl AuthzHook for AllowAll {
    async fn authorize(&self, _actor: &protocol::ActorId, _envelope: &protocol::MutationEnvelope) -> Result<(), DbError> {
        Ok(())
    }
}
//#endregion 🔖️Hooks

//#region 🔖️Engine
/// @emoji ⚙️ Construction-time configuration for one `ArtifactEngine`. Field shape is FROZEN for
/// this wave (see module doc): `db_engine` constructs this as a 4-field struct literal with no
/// `..Default::default()` spread, so a new required field here would be a breaking change to a
/// sibling crate this session does not own.
// 🔀️ `A` is the pluggable `AuthzHook` implementation (open extension point per the module doc: "a
// caller may still hand-roll one") — dedyn-fw-os-misc, R11(a): a stored, caller-supplied
// implementation is trivially generic, so `Arc<dyn AuthzHook>` becomes `Arc<A>` with `AllowAll` as
// the default so every existing `ArtifactEngineConfig`/`::default()` call site keeps compiling
// unparameterized.
// 🔀️ `V` is the pluggable `VersionGraph` backend, generic for the same reason as `A` (R11a). Unlike
// `AuthzHook`, `VersionGraph`'s own closed 2-implementor set (`NullVersionGraph` here, the
// `vcs`-feature-gated `VcsVersionGraph`) is closed with `dyn_enum_close!` into `db_engine`'s
// `VersionGraphs` enum instead — but that enum lives in `db_engine`, one layer above this crate, and
// the hard dependency rule ("only `db_engine` may depend on `vcs`") means `db_artifact` must stay
// ignorant of it. Staying generic here (rather than naming `VersionGraphs` directly) preserves
// exactly the erasure `Arc<dyn VersionGraph>` used to give this crate; `db_engine` is the one layer
// that instantiates `V = VersionGraphs` concretely (see its `Database::document_engine_config`).
pub struct ArtifactEngineConfig<A: AuthzHook + 'static = AllowAll, V: VersionGraph + 'static = NullVersionGraph> {
    pub limits: DbLimits,
    /// @emoji 🛂️ Deprecated-in-spirit extension seam, kept defined (see module doc): `submit` now
    /// authorizes through `security` instead. A caller with an existing `AuthzHook` impl can still
    /// call it manually; `ArtifactEngine` itself no longer does.
    pub authz: Arc<A>,
    /// @emoji 🔐️ The real authz/dedupe/DoS-budget gate `submit` calls once per envelope — see
    /// `db_security::SecurityGate::admit_command`'s doc. Keyed per-envelope by a `Principal`
    /// synthesized from that envelope's own `actor` (a permissive `"member"` role, `"default"`
    /// tenant) — `SubmitOptions` stays durability-only (see its doc) so this crate's dedupe/authz
    /// story does not require a caller to separately authenticate every submit call.
    pub security: db_security::SecurityGate,
    /// @emoji 🌿️ The `vcs` seam (see `VersionGraph`'s doc) — `NullVersionGraph`
    /// (the default) answers every call `Unimplemented` rather than requiring an `Option` layer;
    /// only `db_engine` behind the `vcs` feature wires a real implementation in.
    pub version_graph: Arc<V>,
    // 🔀️ dedyn-emit-runtime, O1/R11(c): every real call site (this crate's own `default()`,
    // `db_engine::document_engine_config`'s `other_defaults.emit` spread) constructs `NullEmit` and
    // nothing else — the field is stored but never actually called (`grep '.emit(' this crate: zero
    // hits). Unlike `db_security::SecurityGate` (which genuinely needs `E: Emit` generic so its own
    // tests can inject a `RecordingEmit`), there is no second implementor anywhere in this crate's
    // call graph, so O1 takes the "exactly one impl" branch: concrete `NullEmit`, no `dyn`, no
    // generic param added to this already-two-deep (`A`, `V`) config type.
    pub emit: Arc<NullEmit>,
    pub preview_ttl_ms: u64,
    /// @emoji 🧬️ Projection factory: `submit`'s project step registers a fresh
    /// `db_projection::ProjectionEngine` from this on every call it needs one (see `🔖️Engine`'s doc
    /// for why a factory rather than a stored, already-built engine — `db_projection::
    /// ProjectionEngine::new`'s borrowed-`IndexStorage` + owned-`Vec<E>` shape does not
    /// semio_compose_rs with `ArtifactEngine` owning its storage as `Arc<dyn DbStorage>` without
    /// becoming self-referential). Defaults to no projections registered — `db_projection::
    /// NoProjections` (dedyn-fw-os-guestruntime, O1/R1: no first-party `dyn ErasedProjection`
    /// trait object) is uninhabited, so this factory can never actually return anything today; not
    /// one call site repo-wide overrides it with anything else. The day a real caller wants to
    /// register a projection here, it swaps `NoProjections` for its own closed `ErasedProjection`
    /// enum (R11) — this field's own `dyn Fn` closure stays (`dyn Fn` is R1-legal, a std trait).
    pub projections: Arc<dyn Fn() -> Vec<db_projection::NoProjections> + Send + Sync>,
}

impl Default for ArtifactEngineConfig<AllowAll, NullVersionGraph> {
    fn default() -> Self {
        let limits = DbLimits::default();
        let policy = db_security::RoleBasedPolicy::new().with_grant(db_security::Grant::allow("member", &["**"], &[db_security::Action::Read, db_security::Action::Write]));
        ArtifactEngineConfig {
            preview_ttl_ms: limits.max_preview_ttl_ms,
            limits,
            authz: Arc::new(AllowAll),
            security: db_security::SecurityGate::new(policy, db_security::ReplayGuard::new(60_000, 4_096), db_security::BudgetRegistry::new(100_000, 100_000), Arc::new(NullEmit)),
            version_graph: Arc::new(NullVersionGraph),
            emit: Arc::new(NullEmit),
            projections: Arc::new(Vec::new),
        }
    }
}

/// @emoji 🎭️ The document authority's real, synchronous pipeline: one open document's WAL,
/// materialized state, causal dependency bookkeeping, previews, and outbox — everything
/// `ArtifactAuthority` (the `db_actor`-mailbox wrapper below) drives in finite process-pool turns.
/// The engine is moved only between serialized turns; callers can also use it directly wherever a
/// mailbox is unnecessary (for example this crate's own tests).
pub struct ArtifactEngine<A: AuthzHook + 'static = AllowAll, V: VersionGraph + 'static = NullVersionGraph> {
    document: ArtifactId,
    protocol_document: protocol::ArtifactId,
    storage: Arc<db_storage::DbBackend>,
    wal: db_wal::ArtifactWal,
    state: DocumentState,
    vcs_head: Option<String>,
    applied: HashMap<String, protocol::MutationEnvelope>,
    applied_receipts: HashMap<String, CommandReceipt>,
    actor_seq: HashMap<String, u64>,
    frontier: Frontier,
    outbox: Vec<OutboxEntry>,
    commit_log: Vec<CommitNotification>,
    previews: db_preview::PreviewStore,
    recent_touches: VecDeque<db_conflict::CommandTouch>,
    live_queries: HashMap<u64, db_query::LiveQuery>,
    next_live_query_id: u64,
    config: ArtifactEngineConfig<A, V>,
}

const MAX_RECENT_TOUCHES: usize = 256;

impl<A: AuthzHook + 'static, V: VersionGraph + 'static> ArtifactEngine<A, V> {
    /// @emoji 🌱️ Retained constructor used by the document authority. Every storage wait remains
    /// represented by this future so a pool worker only polls it once before yielding.
    pub async fn create_retained(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig<A, V>, now_ms: u64) -> Result<ArtifactEngine<A, V>, DbError> {
        let core_id = to_core_document_id(&document).await;
        let wal = db_wal::ArtifactWal::create(&storage.wal().await, core_id.clone(), db_wal::GroupCommitPolicy::default(), now_ms).await?;
        Ok(ArtifactEngine::assemble(document, core_id, storage, wal, None, config).await)
    }

    /// @emoji 🌱️ Creates a brand-new document: a genesis WAL (segment 0) and an empty state.
    /// Errors `AlreadyExists` if `document` already has WAL segments in `storage`.
    /// Process/test entry-point convenience; live document authorities use `create_retained`.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn create(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig<A, V>, now_ms: u64) -> Result<ArtifactEngine<A, V>, DbError> {
        db_actor::block_on(Self::create_retained(document, storage, config, now_ms))
    }

    /// @emoji 🚑️ Retained materialization as initial ⊕ snapshot ⊕ WAL suffix.
    pub async fn open_retained(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig<A, V>, now_ms: u64) -> Result<(ArtifactEngine<A, V>, MaterializeReport), DbError> {
        let core_id = to_core_document_id(&document).await;
        let mut report = MaterializeReport::default();

        let mut state = DocumentState::new();
        let mut applied_head_seq = 0u64;
        let mut vcs_head = None;
        let snapshot_facet = storage.snapshot().await;
        let snapshot_manager = db_snapshot::SnapshotManager::new(&snapshot_facet).await;
        if let Some((generation, descriptor)) = snapshot_manager.load_latest(&core_id).await? {
            report.from_snapshot = true;
            report.snapshot_generation = Some(generation);
            let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
            let mut snapshot_cursor = snapshot_manager.chain_cursor(&core_id, generation, control);
            for hash in &descriptor.roots {
                let mut page_bytes = snapshot_cursor.read_page(*hash).await?;
                let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
                let mut decoder = StatePageDecodeCursor::new(&page_bytes, cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
                while let Some(entry) = decoder.next().await? {
                    let replaced = match state.values.insert(entry) {
                        Ok(replaced) => replaced,
                        Err(mut rejected) => {
                            let _ = rejected.close_step()?;
                            return Err(DbError::LimitExceeded("retained state entries"));
                        }
                    };
                    if let Some(mut replaced) = replaced {
                        let _ = replaced.close_step()?;
                        drop(replaced);
                    }
                }
                let _ = decoder.close_step()?;
                drop(decoder);
                let _ = page_bytes.close_step()?;
                drop(page_bytes);
            }
            let _ = snapshot_cursor.close_step()?;
            applied_head_seq = descriptor.head_seq;
            vcs_head = descriptor.vcs_head;
        }
        drop(snapshot_manager);
        drop(snapshot_facet);

        let (wal, wal_recovery) = db_wal::ArtifactWal::open(&storage.wal().await, core_id.clone(), db_wal::GroupCommitPolicy::default(), now_ms).await?;
        report.torn_tail_bytes = wal_recovery.torn_tail_bytes;
        let mut engine = ArtifactEngine::assemble(document, core_id.clone(), storage.clone(), wal, vcs_head, config).await;
        engine.state = state;
        engine.frontier.head_seq = applied_head_seq;

        let wal_facet = storage.wal().await;
        let replay_cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let replay_control = db_wal::WalCursorControl::new(replay_cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
        let mut records = db_wal::replay_document(&wal_facet, &core_id, replay_control).await?;
        let mut batch_ids: HashSet<String> = HashSet::new();
        let mut seen: u64 = 0;
        loop {
            let mut record = match records.next_step().await? {
                db_wal::WalReplayStep::Record(record) => record,
                db_wal::WalReplayStep::Yield => continue,
                db_wal::WalReplayStep::Done => break,
            };
            match &mut record {
                db_wal::WalRecord::TxBegin { .. } => batch_ids.clear(),
                db_wal::WalRecord::Command(bytes) => {
                    let mut control = db_wal::WalCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
                    let envelope = decode_retained_envelope(bytes, &mut control)?;
                    seen += 1;
                    batch_ids.insert(envelope.mutation_id.0.clone());
                    if seen <= applied_head_seq {
                        engine.applied.insert(envelope.mutation_id.0.clone(), envelope);
                        continue;
                    }
                    let (touched, _conflicts, _) = engine.apply_one(&envelope, &batch_ids).await?;
                    let touch = command_touch(&envelope, &touched);
                    if engine.recent_touches.len() >= MAX_RECENT_TOUCHES {
                        engine.recent_touches.pop_front();
                    }
                    engine.recent_touches.push_back(touch);
                    report.commands_replayed += 1;
                }
                db_wal::WalRecord::Frontier(frontier) => engine.frontier = frontier.clone(),
                _ => {}
            }
            let _ = record.close_step()?;
            drop(record);
        }
        let _ = records.close_step().await?;
        drop(records);
        drop(wal_facet);
        Ok((engine, report))
    }

    /// @emoji 🚑️ Process/test entry-point convenience; live authorities use `open_retained`.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open(document: protocol::ArtifactId, storage: &Arc<db_storage::DbBackend>, config: ArtifactEngineConfig<A, V>, now_ms: u64) -> Result<(ArtifactEngine<A, V>, MaterializeReport), DbError> {
        db_actor::block_on(Self::open_retained(document, storage.clone(), config, now_ms))
    }

    async fn assemble(protocol_document: protocol::ArtifactId, core_id: ArtifactId, storage: Arc<db_storage::DbBackend>, wal: db_wal::ArtifactWal, vcs_head: Option<String>, config: ArtifactEngineConfig<A, V>) -> ArtifactEngine<A, V> {
        let preview_budgets = db_preview::PreviewBudgets { default_ttl_ms: config.preview_ttl_ms, max_ttl_ms: config.preview_ttl_ms, ..db_preview::PreviewBudgets::default() };
        ArtifactEngine {
            document: core_id.clone(),
            protocol_document,
            storage,
            wal,
            state: DocumentState::new(),
            vcs_head,
            applied: HashMap::new(),
            applied_receipts: HashMap::new(),
            actor_seq: HashMap::new(),
            frontier: Frontier::genesis(core_id.clone()),
            outbox: Vec::new(),
            commit_log: Vec::new(),
            previews: db_preview::PreviewStore::new(core_id, preview_budgets),
            recent_touches: VecDeque::new(),
            live_queries: HashMap::new(),
            next_live_query_id: 0,
            config,
        }
    }

    /// @emoji ✅️🚫️ Dependency + dedupe + execute for one envelope, shared by `submit` (before the
    /// WAL write) and `open`'s replay (after it). `batch_ids` is the set of operation ids already
    /// seen earlier in the SAME transaction (a multi-envelope batch may reference its own earlier
    /// members as dependencies). Returns `(touched, conflicts, applied_now)`; `applied_now` is
    /// `false` (with empty touched/conflicts) if `envelope.mutation_id` was already applied in an
    /// earlier commit — the per-envelope half of this crate's dedupe law.
    async fn apply_one(&mut self, envelope: &protocol::MutationEnvelope, batch_ids: &HashSet<String>) -> Result<(db_state::TouchedSet, Vec<ConflictRecord>, bool), DbError> {
        if self.applied.contains_key(&envelope.mutation_id.0) {
            return Ok((db_state::TouchedSet::new(), Vec::new(), false));
        }
        for dependency in &envelope.dependencies {
            if !self.applied.contains_key(&dependency.0) && !batch_ids.contains(&dependency.0) {
                return Err(DbError::InvalidArgument(format!("operation {} depends on unseen operation {}", envelope.mutation_id.0, dependency.0)));
            }
        }
        let entries = diff_entries(&envelope.diff).await?;
        check_len(entries.len() as u64, self.config.limits.max_batch_commands as u64, "db_artifact::diff_entries")?;
        let (touched, conflicts) = self.state.apply_entries(&envelope.mutation_id, &envelope.dependencies, &entries).await?;
        self.applied.insert(envelope.mutation_id.0.clone(), envelope.clone());
        let actor_seq = self.actor_seq.entry(envelope.actor.0.clone()).or_insert(0);
        *actor_seq += 1;
        Ok((touched, conflicts, true))
    }

    /// @emoji 🚦️ The full command pipeline: admit → dedupe → base-resolve/deps → authz → validate →
    /// conflict → execute → WAL append → durability → publish → project → vcs →
    /// preview-reconcile → live-query notify → receipt.
    // 🔒️ `batch` is taken by value deliberately, not just because the current body happens not to
    // move it: submitting a batch is the one place in this API where the caller's copy is
    // logically spent (it becomes part of durable history), matching `ArtifactMessage::Submit`'s
    // owned payload on the actor-mailbox side one level up. Keeping ownership here leaves room for
    // a future zero-copy WAL append that moves `batch.envelopes` directly into the write path.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn submit(&mut self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> Result<CommandReceipt, DbError> {
        // admit
        check_len(batch.envelopes.len() as u64, self.config.limits.max_batch_commands as u64, "db_artifact::batch_commands")?;
        for envelope in &batch.envelopes {
            if envelope.document_id != self.protocol_document {
                return Err(DbError::InvalidArgument(format!("envelope targets document {:?} but this actor owns {:?}", envelope.document_id, self.protocol_document)));
            }
        }
        let command_id = batch.envelopes.last().expect("CommandBatch::new guarantees at least one envelope").mutation_id.clone();

        // dedupe (whole-batch, keyed by the batch's designated command_id)
        if let Some(cached) = self.applied_receipts.get(&command_id.0) {
            return Ok(cached.clone());
        }

        let mut batch_ids: HashSet<String> = HashSet::new();
        let mut records = db_wal::WalRecordBatch::new();
        let wal_cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut wal_control = db_wal::WalCursorControl::new(wal_cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
        let mut touched_all = db_state::TouchedSet::new();
        let mut conflicts_all: Vec<ConflictRecord> = Vec::new();
        // 🎯️ Third tuple element (`Vec<u8>`, the envelope's own encoded bytes) is kept alongside so
        // the outbox push below the outcome-step gate doesn't have to re-encode.
        let mut newly_applied: Vec<(protocol::MutationEnvelope, db_state::TouchedSet, Vec<u8>)> = Vec::new();

        for envelope in &batch.envelopes {
            // authz: the `AuthzHook` seam (defaults to `AllowAll`; `db_engine`'s `SecurityAuthzHook`
            // wraps a real `db_security::SecurityGate` here).
            self.config.authz.authorize(&envelope.actor, envelope).await?;

            // authz (defense in depth): the newer, real `db_security::SecurityGate` gate, keyed by a
            // permissive principal synthesized from the envelope's own actor (see
            // `ArtifactEngineConfig::security`'s doc) — additive, does not replace `authz` above.
            let principal = db_security::Principal::new(envelope.actor.clone(), db_security::TenantId::from("default"), vec!["member".to_string()]);
            self.config.security.admit_command(&principal, &db_security::TenantId::from("default"), &envelope.document_id, &envelope.diff.schema.0, &envelope.actor, &envelope.mutation_id, now_ms).await?;

            // 🎯️ W5: `WalRecord::Command`'s bytes are `protocol::encode_envelope`'s binary record now
            // (M-C's "storage AND communication both binary") — `db_sync::replay_sync_state` reads
            // these same WAL records via `decode_command_envelope` (the same binary codec) to serve
            // semio_hub bootstrap/catch-up, so this crate's own write-and-read-back convention below must
            // agree with it byte-for-byte.
            let mut envelope_bytes = Vec::new();
            protocol::encode_envelope(envelope, &mut envelope_bytes);
            check_len(envelope_bytes.len() as u64, self.config.limits.max_command_bytes, "db_artifact::envelope_bytes")?;

            // base-resolve/deps + execute
            let (touched, conflicts, applied_now) = self.apply_one(envelope, &batch_ids).await?;
            batch_ids.insert(envelope.mutation_id.0.clone());
            if !applied_now {
                continue;
            }

            for region in &touched.regions {
                touched_all.record(region.clone());
            }
            conflicts_all.extend(conflicts);

            // 🎯️ B4: `WalRecord::Diff`/`Inverse` (JSON `serde_json::to_vec` of the same fields
            // `Command` already carries in real binary, via `protocol::encode_envelope`) deleted —
            // never read anywhere in recovery/replay (confirmed: `db_artifact`/`db_engine` only
            // ever reconstruct state from `WalRecord::Command`), a pure redundant JSON duplicate.
            let command_bytes = admit_wal_bytes(envelope_bytes, self.config.limits.max_command_bytes, &mut wal_control).await?;
            let mut outbox_bytes = Vec::new();
            protocol::encode_envelope(envelope, &mut outbox_bytes);
            let outbox_bytes = admit_wal_bytes(outbox_bytes, self.config.limits.max_command_bytes, &mut wal_control).await?;
            push_wal_record(&mut records, db_wal::WalRecord::Command(command_bytes), &mut wal_control).await?;
            push_wal_record(&mut records, db_wal::WalRecord::Outbox(outbox_bytes), &mut wal_control).await?;
            let mut envelope_bytes = Vec::new();
            protocol::encode_envelope(envelope, &mut envelope_bytes);
            newly_applied.push((envelope.clone(), touched, envelope_bytes));
        }

        if newly_applied.is_empty() {
            // Every envelope in this (re-)submitted batch was already durable individually — a
            // full no-op commit, per-envelope half of the dedupe law (see `apply_one`'s doc).
            let receipt = CommandReceipt { command_id, frontier: self.frontier.clone(), durability: options.durability, conflicts: Vec::new(), state_hash: Some(self.state.content_hash().await?), messages: Vec::new() };
            self.applied_receipts.insert(receipt.command_id.0.clone(), receipt.clone());
            return Ok(receipt);
        }

        // outcome step (contract §C9): union this batch's own `db_conflict::ConflictDetector`
        // findings (probed against recent commit history) into graded `protocol::MutationMessage`s,
        // then let `options.policy` decide before touching `self.recent_touches`/`self.outbox`/the
        // WAL at all — a rejected batch must leave every one of those untouched.
        let new_ids: HashSet<&str> = newly_applied.iter().map(|(envelope, _, _)| envelope.mutation_id.0.as_str()).collect();
        let batch_touches: Vec<db_conflict::CommandTouch> = newly_applied.iter().map(|(envelope, touched, _)| command_touch(envelope, touched)).collect();
        let probe: Vec<db_conflict::CommandTouch> = self.recent_touches.iter().cloned().chain(batch_touches.iter().cloned()).collect();
        // 🔀️ `grade_conflict_record` genuinely awaits (`protocol::MutationMessage::warn`/`fatal`),
        // so this can't stay an `Iterator::map` chain (R10 residue shape 1: `.await` inside a sync
        // closure) — hoisted into an explicit async loop instead.
        let mut messages: Vec<protocol::MutationMessage> = Vec::new();
        for record in db_conflict::ConflictDetector::new().detect(&probe).iter().filter(|record| new_ids.contains(record.command_id.0.as_str()) || new_ids.contains(record.conflicting_with.0.as_str())) {
            messages.push(grade_conflict_record(record).await);
        }
        if let Some(worst) = protocol::worst_level(&messages) {
            if options.policy.rejects(worst) {
                return Err(DbError::Rejected { policy: options.policy, worst, messages });
            }
        }

        // Bookkeeping for `preview_conflicts`'s real, additive `db_conflict::ConflictDetector`
        // integration (see its own doc) — `submit`'s own returned `ConflictRecord`s stay this
        // crate's original path-granular last-writer detection above (see `🔖️Conflict`'s doc). Only
        // reached once the outcome step above has accepted the batch.
        for touch in batch_touches {
            if self.recent_touches.len() >= MAX_RECENT_TOUCHES {
                self.recent_touches.pop_front();
            }
            self.recent_touches.push_back(touch);
        }
        for (envelope, _, bytes) in &newly_applied {
            self.outbox.push(OutboxEntry { mutation_id: envelope.mutation_id.clone(), bytes: bytes.clone() });
        }

        // publish: compute + WAL-append the new frontier in the same transaction as its commands
        let new_frontier =
            Frontier { document: self.document.clone(), head_seq: self.frontier.head_seq + newly_applied.len() as u64, commit_seq: self.frontier.commit_seq + 1, chain_hash: self.state.content_hash().await?.0, epoch: self.frontier.epoch };
        push_wal_record(&mut records, db_wal::WalRecord::Frontier(new_frontier.clone()), &mut wal_control).await?;

        // WAL append + durability (ArtifactWal::submit wraps `records` in its own TxBegin/TxCommit)
        let wal_facet = self.storage.wal().await;
        self.wal.submit(&wal_facet, &records, options.durability, now_ms).await?;
        drop(wal_facet);
        wal_control.grant()?;
        let _ = records.close_step()?;
        drop(records);
        self.frontier = new_frontier.clone();

        // publish: durable indices
        let index_facet = self.storage.index().await;
        let command_index = db_index::CommandIndex::new(&index_facet, self.document.clone()).await;
        let inverse_index = db_index::InverseIndex::new(&index_facet, self.document.clone()).await;
        let actor_seq_index = db_index::ActorSeqIndex::new(&index_facet, self.document.clone()).await;
        db_index::FrontierIndex::new(&index_facet, self.document.clone()).await.record(&new_frontier).await?;
        let base_seq = self.frontier.head_seq - newly_applied.len() as u64;
        for (offset, (envelope, _, _)) in newly_applied.iter().enumerate() {
            let seq = base_seq + offset as u64 + 1;
            let location = db_index::RecordLocation { segment: self.wal.active_segment_index().await, offset: seq, len: 1 };
            command_index.record(seq, location).await?;
            inverse_index.record(seq, location).await?;
            let core_actor = to_core_actor_id(&envelope.actor).await;
            let actor_seq = *self.actor_seq.get(&envelope.actor.0).unwrap_or(&0);
            actor_seq_index.record(&core_actor, actor_seq, seq).await?;
        }

        // project: run every registered projection over each newly-applied envelope
        let projection_classes = (self.config.projections)();
        if !projection_classes.is_empty() {
            let engine = db_projection::ProjectionEngine::new(&index_facet, self.document.clone(), projection_classes).await?;
            for (offset, (envelope, touched, _)) in newly_applied.iter().enumerate() {
                engine.apply_envelope(base_seq + offset as u64 + 1, envelope, touched).await?;
            }
        }
        drop(index_facet);

        // preview-reconcile
        self.previews.reconcile_with(&db_preview::LandedCommand { frontier: new_frontier.clone(), touched: touched_all.clone() }, &db_preview::DbConflictOracle::default());
        self.commit_log.push(CommitNotification { frontier: new_frontier.clone(), operation_ids: newly_applied.iter().map(|(envelope, _, _)| envelope.mutation_id.clone()).collect(), touched: touched_all });

        // vcs (best-effort: this crate never blocks a commit on the vcs seam's outcome; a disabled
        // vcs feature supplies `NullVersionGraph`, whose `Unimplemented` is tolerated here)
        for (envelope, _, _) in &newly_applied {
            match self
                .config
                .version_graph
                .record_change(
                    &self.document,
                    ChangeRecord { parent: None, content_hash: self.state.content_hash().await?, author: to_core_actor_id(&envelope.actor).await, message: format!("operation {}", envelope.mutation_id.0), timestamp_ms: now_ms },
                )
                .await
            {
                Ok(_) | Err(DbError::Unimplemented(_)) => {}
                Err(other) => return Err(other),
            }
        }

        // live-query notify
        // 🩹️ R13: was `let _ = self.refresh_live_queries();` — silently dropped the future, so
        // this notify step never actually ran. `.await`ed now; the `Vec<(u64, QueryDiff)>` result
        // is genuinely unneeded here (callers observe diffs via their own subscription poll), so
        // `let _ =` on the awaited, resolved value is the correct fire-and-forget shape.
        let _ = self.refresh_live_queries().await;

        // receipt
        let receipt = CommandReceipt { command_id, frontier: new_frontier, durability: options.durability, conflicts: conflicts_all, state_hash: Some(self.state.content_hash().await?), messages };
        self.applied_receipts.insert(receipt.command_id.0.clone(), receipt.clone());
        Ok(receipt)
    }

    /// @emoji ↩️ The inverse-undo pipeline: looks up `target`'s already-applied envelope, flips its
    /// `inverse` into a compensating envelope's `diff` (and vice versa, so the compensating
    /// envelope's OWN inverse can re-undo the undo), and submits it as a fresh, ordinary command
    /// depending on `target` — undo is just another commit, not a WAL rewrite. This is the crate's
    /// "inverse undo, using protocol's inverse-operation machinery".
    pub async fn undo(&mut self, target: &protocol::MutationId, undo_mutation_id: protocol::MutationId, actor: protocol::ActorId, now_ms: u64) -> Result<CommandReceipt, DbError> {
        let original = self.applied.get(&target.0).cloned().ok_or_else(|| DbError::NotFound(format!("operation {} not found for undo", target.0)))?;
        let undo_diff_entries = inverse_entries(&original.inverse).await?;
        let redo_inverse_entries = diff_entries(&original.diff).await?;
        let compensating = protocol::MutationEnvelope {
            mutation_id: undo_mutation_id,
            document_id: self.protocol_document.clone(),
            actor,
            dependencies: vec![target.clone()],
            diff: protocol::ArtifactDiff { schema: original.inverse.schema.clone(), payload: encode_pathmap(&entries_to_value(&undo_diff_entries)).await },
            inverse: protocol::InverseMutation { schema: original.diff.schema, payload: encode_pathmap(&entries_to_value(&redo_inverse_entries)).await },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms),
        };
        self.submit(CommandBatch::new(vec![compensating]).await?, SubmitOptions::default(), now_ms).await
    }

    pub async fn get(&self, path: &str) -> Result<Option<db_query::QueryBytes>, DbError> {
        let Some(value) = self.state.get(path) else { return Ok(None) };
        let mut control = db_query::QueryCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        Ok(Some(db_query::QueryBytes::copy_from_pages(value, &mut control).await?))
    }

    pub async fn frontier(&self) -> Frontier {
        self.frontier.clone()
    }

    pub async fn commit_log(&self) -> &[CommitNotification] {
        &self.commit_log
    }

    pub fn history_replay(&self, operation_generation: u64, cancelled: Arc<std::sync::atomic::AtomicBool>, reservation: HistoryReplayReservation) -> HistoryReplayFuture {
        HistoryReplayFuture::new(self.storage.clone(), self.document.clone(), operation_generation, cancelled, reservation)
    }

    /// @emoji 📤️ Hands out (and clears) every effect queued since the last drain.
    pub async fn drain_outbox(&mut self) -> Vec<OutboxEntry> {
        std::mem::take(&mut self.outbox)
    }

    //#region 🔖️Snapshot
    /// @emoji 📸️ Publishes a new `db_snapshot` generation of the whole current `DocumentState` —
    /// new this revision; the counterpart `open` reads back to accelerate materialization.
    pub async fn snapshot_now(&self, now_ms: u64) -> Result<u64, DbError> {
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let encoder = StatePageEncodeCursor::try_new(&self.state.values, cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let page = db_state::Page::try_from_pages(encoder.finish().await?).await?;
        let snapshot_facet = self.storage.snapshot().await;
        let snapshot_manager = db_snapshot::SnapshotManager::new(&snapshot_facet).await;
        let origin = if snapshot_manager.load_latest(&self.document).await?.is_some() { db_snapshot::SnapshotOrigin::Incremental } else { db_snapshot::SnapshotOrigin::FullBaseline };
        let body = db_snapshot::SnapshotBody {
            head_seq: self.frontier.head_seq,
            commit_seq: self.frontier.commit_seq,
            epoch: self.frontier.epoch,
            chain_hash: self.frontier.chain_hash,
            protocol_version: 1,
            vcs_head: self.vcs_head.clone(),
            base_pack_hash: None,
            roots: vec![page.hash],
            created_at_ms: now_ms,
        };
        snapshot_manager.publish(&self.document, origin, &[page], body).await
    }
    //#endregion 🔖️Snapshot

    //#region 🔖️Query
    /// @emoji 🔎️ One-shot query over the document's current materialized state, resolved under
    /// `consistency` via `db_index`'s `CommitIndex`/`FrontierIndex`. `StateQuerySource` always reads
    /// the CURRENT canonical state regardless of what `consistency` resolved to — a true
    /// point-in-time replay is `db_engine`'s documented deferred extension (see its own module doc).
    // 🔒️ Mirrors `submit`'s ownership rationale above: `ArtifactMessage::RunQuery` already owns
    // `query`/`consistency` one level up on the actor-mailbox side, and query descriptors are
    // small, cheap-to-move value types — taking them by reference here would only push the
    // ownership question onto every caller for no benefit.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn query(&self, query: db_query::Query, consistency: db_query::Consistency) -> Result<db_query::QueryResult, DbError> {
        let index_facet = self.storage.index().await;
        let resolver = db_query::IndexConsistencyResolver { commits: db_index::CommitIndex::new(&index_facet, self.document.clone()).await, frontiers: db_index::FrontierIndex::new(&index_facet, self.document.clone()).await };
        // A fresh document has no recorded frontier yet; canonical reads still succeed via the
        // in-memory frontier, so only consult the resolver for modes that truly need the index.
        if !matches!(consistency, db_query::Consistency::Canonical) {
            db_query::resolve_consistency(&consistency, &resolver).await?;
        }
        let source = StateQuerySource(&self.state.values);
        let mut control = db_query::QueryCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        db_query::execute(&query, &source, None::<&db_query::NoFullTextLookup>, &db_query::QueryLimits::default(), &mut control).await
    }

    /// @emoji 📡️ Registers a live query, returning its subscription id — new this revision.
    pub async fn subscribe(&mut self, spec: db_query::LiveQuerySpec) -> u64 {
        let id = self.next_live_query_id;
        self.next_live_query_id += 1;
        self.live_queries.insert(id, db_query::LiveQuery::new(spec).await);
        id
    }

    pub async fn unsubscribe(&mut self, id: u64) {
        self.live_queries.remove(&id);
    }

    /// @emoji 📡️ Live-query notify: re-evaluates every registered live query and returns what
    /// changed. Called automatically at the end of `submit`; also callable directly.
    pub async fn refresh_live_queries(&mut self) -> Vec<(u64, db_query::QueryDiff)> {
        let source = StateQuerySource(&self.state.values);
        let limits = db_query::QueryLimits::default();
        let mut diffs = Vec::new();
        for (id, live_query) in self.live_queries.iter_mut() {
            let mut control = match db_query::QueryCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536) {
                Ok(control) => control,
                Err(_) => continue,
            };
            if let Ok(diff) = live_query.refresh(&source, None::<&db_query::NoFullTextLookup>, &limits, &mut control).await {
                if !diff.added.is_empty() || !diff.removed.is_empty() || !diff.updated.is_empty() {
                    diffs.push((*id, diff));
                }
            }
        }
        diffs
    }
    //#endregion 🔖️Query

    //#region 🔖️Advisory
    /// @emoji 🔮️ Advisory-only, real `db_conflict::ConflictDetector` integration: runs `batch`'s
    /// envelopes' touched regions against recent commit history WITHOUT executing anything, using
    /// the family's real bloom-filter/kind-matrix machinery — a caller (e.g. a UI) can call this
    /// before `submit` to preview likely conflicts. `submit`'s own returned `ConflictRecord`s stay
    /// this crate's original path-granular last-writer detection (see `🔖️Conflict`'s doc on why
    /// `db_conflict`'s per-command-pair shape can't replace it without breaking `db_engine`'s frozen
    /// `CommandReceipt.conflicts` element type).
    pub async fn preview_conflicts(&self, batch: &CommandBatch) -> Result<Vec<db_conflict::ConflictRecord>, DbError> {
        let mut probe: Vec<db_conflict::CommandTouch> = self.recent_touches.iter().cloned().collect();
        for envelope in &batch.envelopes {
            let entries = diff_entries(&envelope.diff).await?;
            let touched = entries_touched(&entries);
            probe.push(command_touch(envelope, &touched));
        }
        Ok(db_conflict::ConflictDetector::new().detect(&probe))
    }
    //#endregion 🔖️Advisory

    //#region 🔖️Preview
    /// @emoji 🌫️ Publishes a new preview overlaying the CURRENT committed state — never durable
    /// (never touches the WAL), per the contract's preview law. Backed by a real
    /// `db_preview::PreviewStore` this revision (previously a local, minimal stand-in).
    pub async fn publish_preview(&mut self, entries: &[(String, Option<serde_json::Value>)], now_ms: u64) -> Result<db_preview::PreviewId, DbError> {
        let dsl_entries: Vec<(String, Option<DslValue>)> = entries.iter().map(|(path, value)| Ok((path.clone(), value.as_ref().map(|json| dsl::to_dsl_value(json).map_err(dsl_err)).transpose()?))).collect::<Result<Vec<_>, DbError>>()?;
        let touched = entries_touched(&dsl_entries);
        let envelope = protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(format!("preview-{}", entries.len())),
            document_id: self.protocol_document.clone(),
            actor: protocol::ActorId("preview".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&entries_to_value(&dsl_entries)).await },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(vec![])).await },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms),
        };
        self.previews.publish(db_preview::PublishPreviewRequest { document: self.document.clone(), actor: ActorId("preview".to_string()), key: format!("preview-{now_ms}"), base: self.frontier.clone(), envelope, touched, ttl_ms: None, now_ms })
    }

    /// @emoji 🌫️ The value a preview would show at `path`: the preview's own diff if it touches
    /// `path`, else falling through to the committed state.
    pub async fn preview_get(&self, id: &db_preview::PreviewId, path: &str) -> Result<Option<db_query::QueryBytes>, DbError> {
        let preview = self.previews.get(id).ok_or_else(|| DbError::NotFound(format!("preview {id} not found")))?;
        for (entry_path, value) in diff_entries(&preview.envelope.diff).await? {
            if entry_path == path {
                return match value {
                    Some(dsl_value) => {
                        let bytes = store::pack_rt::encode_wire_value(&dsl_value);
                        let mut writer = db_storage::DbIoPageWriter::try_reserve(bytes.capacity().div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
                        let mut offset = 0;
                        while offset < bytes.len() {
                            offset += writer.write_fragment(&bytes[offset..])?;
                        }
                        let pages = writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)?;
                        Ok(Some(db_query::QueryBytes::from_pages(pages).map_err(|(error, mut pages)| {
                            let _ = pages.close_step();
                            drop(pages);
                            error
                        })?))
                    }
                    None => Ok(None),
                };
            }
        }
        self.get(path).await
    }

    pub async fn preview_status(&self, id: &db_preview::PreviewId) -> Result<db_preview::PreviewState, DbError> {
        Ok(self.previews.get(id).ok_or_else(|| DbError::NotFound(format!("preview {id} not found")))?.state)
    }

    pub async fn withdraw_preview(&mut self, id: &db_preview::PreviewId) -> Result<(), DbError> {
        self.previews.withdraw(id)
    }

    pub async fn reject_preview(&mut self, id: &db_preview::PreviewId) -> Result<(), DbError> {
        self.previews.reject(id)
    }

    pub async fn expire_previews(&mut self, now_ms: u64) -> Vec<db_preview::PreviewId> {
        self.previews.sweep_expired(now_ms)
    }
    //#endregion 🔖️Preview
}
//#endregion 🔖️Engine

//#region 🔖️QuerySource
/// @emoji 🚰️ The `db_query::QuerySource` this crate supplies over its own `DocumentState`: one row
/// per stored path, `{"path": <path>, "value": <text-or-bytes>}`.
struct StateQuerySource<'a>(&'a db_state::RetainedStateMap);

// 🚫️async: E1 pure accessor consumed by a sync Iterator::map (QuerySource::scan's row builder) — see R9
async fn path_row_value(path: &str, bytes: &db_storage::DbIoPages, control: &mut db_query::QueryCursorControl) -> Result<db_query::Value, DbError> {
    let mut map = std::collections::BTreeMap::new();
    map.insert("path".to_string(), db_query::Value::Text(path.to_string()));
    map.insert("value".to_string(), db_query::Value::Bytes(db_query::QueryBytes::copy_from_pages(bytes, control).await?));
    Ok(db_query::Value::Map(map))
}

impl<'a> db_query::QuerySource for StateQuerySource<'a> {
    async fn scan(&self, control: &mut db_query::QueryCursorControl) -> Result<db_query::QueryRows, DbError> {
        let mut rows = db_query::QueryRows::new();
        for (index, entry) in self.0.iter().enumerate() {
            control.grant()?;
            let row = db_query::QueryRow::new(db_query::RowId(index as u64), path_row_value(entry.key(), entry.value(), control).await?);
            rows.push(row).map_err(|_| DbError::LimitExceeded("artifact query row slots"))?;
        }
        Ok(rows)
    }
}
//#endregion 🔖️QuerySource

//#region 🔖️Snapshot
const MAX_STATE_PAGE_ENTRIES: u64 = 10_000_000;
const MAX_STATE_PAGE_PATH_BYTES: u64 = db_storage::DbIoText::maximum_capacity() as u64;
const MAX_STATE_PAGE_VALUE_BYTES: u64 = 256 * 1024 * 1024;

fn state_page_varint_len(mut value: u64) -> usize {
    let mut len = 1;
    while value >= 0x80 {
        value >>= 7;
        len += 1;
    }
    len
}

fn state_page_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
    let mut len = 0;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output[len] = byte;
        len += 1;
        if value == 0 {
            return &output[..len];
        }
    }
}

struct StatePageGrant {
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    deadline: std::time::Instant,
    fuel: usize,
}

impl StatePageGrant {
    fn consume(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("state page cursor cancelled".to_string()));
        }
        if std::time::Instant::now() >= self.deadline {
            return Err(DbError::Unavailable("state page cursor deadline reached".to_string()));
        }
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("state page cursor fuel"))?;
        Ok(())
    }
}

struct StatePageEncodeCursor<'state> {
    values: &'state db_state::RetainedStateMap,
    writer: db_storage::DbIoPageWriter,
    grant: StatePageGrant,
}

impl<'state> StatePageEncodeCursor<'state> {
    fn try_new(values: &'state db_state::RetainedStateMap, cancelled: Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        check_len(values.len() as u64, MAX_STATE_PAGE_ENTRIES, "db_artifact::snapshot_page_entries")?;
        let mut len = state_page_varint_len(values.len() as u64);
        for entry in values.iter() {
            let path = entry.key();
            let value = entry.value();
            check_len(path.len() as u64, MAX_STATE_PAGE_PATH_BYTES, "db_artifact::snapshot_page_path")?;
            check_len(value.len() as u64, MAX_STATE_PAGE_VALUE_BYTES, "db_artifact::snapshot_page_value")?;
            len = len.checked_add(state_page_varint_len(path.len() as u64) + path.len() + 1 + state_page_varint_len(value.len() as u64) + value.len()).ok_or(DbError::LimitExceeded("state page encoded bytes"))?;
        }
        let pages = len.div_ceil(db_storage::DB_IO_PAGE_BYTES);
        if pages > db_storage::DB_IO_OPERATION_PAGES {
            return Err(DbError::LimitExceeded("state page reservation"));
        }
        let writer = db_storage::DbIoPageWriter::try_reserve(pages).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        Ok(Self { values, writer, grant: StatePageGrant { cancelled, deadline, fuel } })
    }

    async fn write(&mut self, bytes: &[u8]) -> Result<(), DbError> {
        let mut offset = 0;
        while offset < bytes.len() {
            self.grant.consume()?;
            offset += self.writer.write_fragment(&bytes[offset..])?;
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }

    async fn finish(mut self) -> Result<db_storage::DbIoPages, DbError> {
        let mut varint = [0u8; 10];
        self.write(state_page_varint(self.values.len() as u64, &mut varint)).await?;
        for entry in self.values.iter() {
            let path = entry.key();
            let value = entry.value();
            self.write(state_page_varint(path.len() as u64, &mut varint)).await?;
            self.write(path.as_bytes()).await?;
            self.write(&[1]).await?;
            self.write(state_page_varint(value.len() as u64, &mut varint)).await?;
            for fragment in value.fragments() {
                self.write(fragment).await?;
            }
        }
        self.writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)
    }
}

struct StatePageDecodeCursor<'pages> {
    pages: &'pages db_storage::DbIoPages,
    page: u8,
    offset: usize,
    remaining: Option<u64>,
    grant: StatePageGrant,
    closed: bool,
}

impl<'pages> StatePageDecodeCursor<'pages> {
    fn new(pages: &'pages db_storage::DbIoPages, cancelled: Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if pages.operation() == 0 || fuel == 0 {
            return Err(DbError::InvalidArgument("state page decode authority".to_string()));
        }
        Ok(Self { pages, page: 0, offset: 0, remaining: None, grant: StatePageGrant { cancelled, deadline, fuel }, closed: false })
    }

    fn byte(&mut self) -> Result<u8, DbError> {
        self.grant.consume()?;
        loop {
            let fragment = self.pages.page(self.page).ok_or_else(|| DbError::Corrupt("state page ended early".to_string()))?;
            if let Some(byte) = fragment.get(self.offset) {
                self.offset += 1;
                return Ok(*byte);
            }
            self.page = self.page.checked_add(1).ok_or(DbError::LimitExceeded("state page fragment cursor"))?;
            self.offset = 0;
        }
    }

    fn varint(&mut self) -> Result<u64, DbError> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.byte()?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(DbError::Corrupt("state page varint overflow".to_string()))
    }

    fn fixed_field<const N: usize>(&mut self, len: usize) -> Result<[u8; N], DbError> {
        if len > N {
            return Err(DbError::LimitExceeded("state page fixed field"));
        }
        let mut output = [0u8; N];
        let mut written = 0;
        while written < len {
            self.grant.consume()?;
            let fragment = self.pages.page(self.page).ok_or_else(|| DbError::Corrupt("state page field ended early".to_string()))?;
            if self.offset == fragment.len() {
                self.page = self.page.checked_add(1).ok_or(DbError::LimitExceeded("state page fragment cursor"))?;
                self.offset = 0;
                continue;
            }
            let copied = (len - written).min(fragment.len() - self.offset);
            output[written..written + copied].copy_from_slice(&fragment[self.offset..self.offset + copied]);
            written += copied;
            self.offset += copied;
        }
        Ok(output)
    }

    async fn retained_field(&mut self, len: usize) -> Result<db_storage::DbIoPages, DbError> {
        let pages = len.div_ceil(db_storage::DB_IO_PAGE_BYTES);
        let mut writer = db_storage::DbIoPageWriter::try_reserve_for_operation(self.pages.operation(), pages).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        let mut written = 0;
        while written < len {
            self.grant.consume()?;
            let fragment = self.pages.page(self.page).ok_or_else(|| DbError::Corrupt("state page value ended early".to_string()))?;
            if self.offset == fragment.len() {
                self.page = self.page.checked_add(1).ok_or(DbError::LimitExceeded("state page fragment cursor"))?;
                self.offset = 0;
                continue;
            }
            let copied = (len - written).min(fragment.len() - self.offset);
            let accepted = writer.write_fragment(&fragment[self.offset..self.offset + copied])?;
            if accepted != copied {
                return Err(DbError::Internal("state page value writer made partial progress".to_string()));
            }
            self.offset += copied;
            written += copied;
            semio_framework_async::yield_once().await;
        }
        writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)
    }

    async fn next(&mut self) -> Result<Option<db_state::StateEntry>, DbError> {
        let remaining = match self.remaining {
            Some(remaining) => remaining,
            None => {
                let count = self.varint()?;
                check_len(count, MAX_STATE_PAGE_ENTRIES, "db_artifact::snapshot_page_entries")?;
                self.remaining = Some(count);
                count
            }
        };
        if remaining == 0 {
            return Ok(None);
        }
        let path_len = self.varint()?;
        check_len(path_len, MAX_STATE_PAGE_PATH_BYTES, "db_artifact::snapshot_page_path")?;
        let path = self.fixed_field::<1024>(path_len as usize)?;
        let path = std::str::from_utf8(&path[..path_len as usize]).map_err(|_| DbError::Corrupt("snapshot page path is not valid utf-8".to_string()))?;
        let value = match self.byte()? {
            1 => {
                let len = self.varint()?;
                check_len(len, MAX_STATE_PAGE_VALUE_BYTES, "db_artifact::snapshot_page_value")?;
                self.retained_field(len as usize).await?
            }
            _ => return Err(DbError::Corrupt("state page value tag".to_string())),
        };
        let entry = db_state::StateEntry::try_new(path, value).map_err(|(error, mut value)| {
            let _ = value.close_step();
            drop(value);
            error
        })?;
        self.remaining = Some(remaining - 1);
        semio_framework_async::yield_once().await;
        Ok(Some(entry))
    }

    fn close_step(&mut self) -> Result<bool, DbError> {
        self.grant.consume()?;
        if self.closed {
            return Ok(false);
        }
        self.remaining = Some(0);
        self.closed = true;
        Ok(true)
    }
}

/// @emoji 📋️ What `ArtifactEngine::open` did to materialize state — "initial ⊕ snapshot ⊕ WAL
/// suffix" made observable. New this revision (was `db_wal::WalRecoveryReport` alone before).
#[derive(Clone, Debug, Default)]
pub struct MaterializeReport {
    pub from_snapshot: bool,
    pub snapshot_generation: Option<u64>,
    pub torn_tail_bytes: u64,
    pub commands_replayed: u64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HistoryReplay
const HISTORY_REPLAY_PAGE_BYTES: u64 = 16 * 1024;
const HISTORY_REPLAY_SEGMENT_PAGES: u64 = 1_024;
const HISTORY_REPLAY_SEGMENT_BYTES: u64 = HISTORY_REPLAY_PAGE_BYTES * HISTORY_REPLAY_SEGMENT_PAGES;
const HISTORY_REPLAY_RESULT_BYTES: u64 = 15 * 1024 * 1024;
const HISTORY_REPLAY_OPERATION_BYTES: u64 = 32 * 1024 * 1024;
const HISTORY_REPLAY_MAX_FRAME_BYTES: u64 = 1024 * 1024;
const HISTORY_REPLAY_MAX_FIELD_BYTES: usize = HISTORY_REPLAY_PAGE_BYTES as usize;
const HISTORY_REPLAY_MAX_ENTRIES: usize = 4_096;
const HISTORY_REPLAY_MAX_OPERATION_IDS: usize = 8_192;
const HISTORY_REPLAY_RESULT_PAGES: usize = (HISTORY_REPLAY_RESULT_BYTES / HISTORY_REPLAY_PAGE_BYTES) as usize;
const HISTORY_REPLAY_CONSTRUCTION_SLOTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct HistoryTextRange {
    start: u32,
    len: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactHistoryEntry {
    pub operation_start: u16,
    pub operation_count: u16,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
    pub epoch: u64,
}

pub struct ArtifactHistoryView {
    pub entries: Vec<ArtifactHistoryEntry>,
    operation_ids: Vec<HistoryTextRange>,
    result_pages: Vec<Option<Vec<u8>>>,
    result_len: u64,
    pub retained_bytes: u64,
    pub retained_items: usize,
}

impl ArtifactHistoryView {
    pub fn operation_id_eq(&self, entry: usize, operation: usize, expected: &str) -> bool {
        let Some(entry) = self.entries.get(entry) else {
            return false;
        };
        if operation >= entry.operation_count as usize {
            return false;
        }
        let Some(index) = (entry.operation_start as usize).checked_add(operation) else {
            return false;
        };
        let Some(range) = self.operation_ids.get(index).copied() else {
            return false;
        };
        let expected = expected.as_bytes();
        if expected.len() != range.len as usize {
            return false;
        }
        let offset = range.start as usize;
        let page = offset / HISTORY_REPLAY_PAGE_BYTES as usize;
        let within = offset % HISTORY_REPLAY_PAGE_BYTES as usize;
        let Some(first) = self.result_pages.get(page).and_then(Option::as_ref) else {
            return false;
        };
        let first_len = expected.len().min(first.len().saturating_sub(within));
        if first_len == 0 || expected.get(..first_len) != first.get(within..within + first_len) {
            return false;
        }
        if first_len == expected.len() {
            return true;
        }
        self.result_pages.get(page + 1).and_then(Option::as_ref).and_then(|second| second.get(..expected.len() - first_len)) == expected.get(first_len..)
    }

    pub fn close_step(&mut self) -> bool {
        if self.operation_ids.pop().is_some() {
            return true;
        }
        if self.entries.pop().is_some() {
            return true;
        }
        if let Some(owner) = self.result_pages.pop() {
            drop(owner);
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.operation_ids.is_empty() && self.entries.is_empty() && self.result_pages.is_empty()
    }
}

pub struct HistoryReplayReservation {
    source_pages: Vec<Option<Vec<u8>>>,
    source_page_count: usize,
    result_pages: Vec<Option<Vec<u8>>>,
    operation_ids: Vec<HistoryTextRange>,
    entries: Vec<ArtifactHistoryEntry>,
    scratch: Option<Vec<u8>>,
    retained_operation_bytes: u64,
    retained_result_bytes: u64,
}

impl HistoryReplayReservation {
    pub(crate) fn try_new() -> Result<Self, HistoryReplayReservationConstructionFault> {
        Self::try_new_retained(None)
    }

    #[cfg(test)]
    fn try_new_with_result_page_failure(failure_after: usize) -> Result<Self, HistoryReplayReservationConstructionFault> {
        Self::try_new_retained(Some(failure_after))
    }

    fn try_new_retained(failure_after: Option<usize>) -> Result<Self, HistoryReplayReservationConstructionFault> {
        let mut builder = match HistoryReplayReservationConstructionBuilder::new() {
            Ok(builder) => builder,
            Err(fault) => return Err(fault),
        };
        if !builder.edit_cursor(|cursor| cursor.source_pages.as_mut().is_some_and(|owners| owners.try_reserve_exact(HISTORY_REPLAY_SEGMENT_PAGES as usize).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history source owner-slot reservation failed".to_string())));
        }
        if !builder
            .edit_cursor(|cursor| {
                let Some(owners) = cursor.source_pages.as_mut() else {
                    return false;
                };
                owners.resize_with(HISTORY_REPLAY_SEGMENT_PAGES as usize, || None);
                true
            })
            .unwrap_or(false)
        {
            return Err(builder.fault(|| DbError::Unavailable("history source owner-slot publication failed".to_string())));
        }
        if !builder.edit_cursor(|cursor| cursor.result_pages.as_mut().is_some_and(|owners| owners.try_reserve_exact(HISTORY_REPLAY_RESULT_PAGES).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history result page-slot reservation failed".to_string())));
        }
        let mut result_owner_bytes = 0u64;
        for page_index in 0..HISTORY_REPLAY_RESULT_PAGES {
            if failure_after == Some(page_index) {
                return Err(builder.fault(|| DbError::Unavailable("history injected result page reservation failed".to_string())));
            }
            let mut page = Vec::new();
            if page.try_reserve_exact(HISTORY_REPLAY_PAGE_BYTES as usize).is_err() {
                return Err(builder.fault(|| DbError::Unavailable("history result page reservation failed".to_string())));
            }
            page.resize(HISTORY_REPLAY_PAGE_BYTES as usize, 0);
            let Some(next_result_owner_bytes) = result_owner_bytes.checked_add(page.capacity() as u64) else {
                let published = builder.edit_cursor(|cursor| {
                    cursor.result_pages.as_mut().is_some_and(|owners| {
                        owners.push(Some(page));
                        true
                    })
                });
                if !published.unwrap_or(false) {
                    return Err(builder.fault(|| DbError::Unavailable("history result page publication failed".to_string())));
                }
                return Err(builder.fault(|| DbError::LimitExceeded("history result reservation bytes")));
            };
            result_owner_bytes = next_result_owner_bytes;
            if !builder
                .edit_cursor(|cursor| {
                    cursor.result_pages.as_mut().is_some_and(|owners| {
                        owners.push(Some(page));
                        true
                    })
                })
                .unwrap_or(false)
            {
                return Err(builder.fault(|| DbError::Unavailable("history result page publication failed".to_string())));
            }
        }
        if failure_after == Some(HISTORY_REPLAY_RESULT_PAGES) {
            return Err(builder.fault(|| DbError::Unavailable("history injected result page boundary failed".to_string())));
        }
        if !builder.edit_cursor(|cursor| cursor.operation_ids.as_mut().is_some_and(|owners| owners.try_reserve_exact(HISTORY_REPLAY_MAX_OPERATION_IDS).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history operation owner-slot reservation failed".to_string())));
        }
        if !builder.edit_cursor(|cursor| cursor.entries.as_mut().is_some_and(|owners| owners.try_reserve_exact(HISTORY_REPLAY_MAX_ENTRIES).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history entry owner-slot reservation failed".to_string())));
        }
        let Some((result_slot_bytes, operation_slot_bytes, entry_slot_bytes, source_slot_bytes)) = builder
            .read_cursor(|cursor| {
                let result = cursor.result_pages.as_ref().map(Vec::capacity);
                let operations = cursor.operation_ids.as_ref().map(Vec::capacity);
                let entries = cursor.entries.as_ref().map(Vec::capacity);
                let source = cursor.source_pages.as_ref().map(Vec::capacity);
                match (result, operations, entries, source) {
                    (Some(result), Some(operations), Some(entries), Some(source)) => Some((
                        (result * std::mem::size_of::<Option<Vec<u8>>>()) as u64,
                        (operations * std::mem::size_of::<HistoryTextRange>()) as u64,
                        (entries * std::mem::size_of::<ArtifactHistoryEntry>()) as u64,
                        (source * std::mem::size_of::<Option<Vec<u8>>>()) as u64,
                    )),
                    _ => None,
                }
            })
            .flatten()
        else {
            return Err(builder.fault(|| DbError::Unavailable("history construction owner accounting was unavailable".to_string())));
        };
        let Some(retained_result_bytes) = result_owner_bytes.checked_add(result_slot_bytes).and_then(|bytes| bytes.checked_add(operation_slot_bytes)).and_then(|bytes| bytes.checked_add(entry_slot_bytes)) else {
            return Err(builder.fault(|| DbError::LimitExceeded("history retained result bytes")));
        };
        let Some(retained_operation_bytes) = retained_result_bytes.checked_add(source_slot_bytes).and_then(|bytes| bytes.checked_add(HISTORY_REPLAY_MAX_FIELD_BYTES as u64)) else {
            return Err(builder.fault(|| DbError::LimitExceeded("history retained operation bytes")));
        };
        if !builder.edit_cursor(|cursor| cursor.scratch.as_mut().is_some_and(|scratch| scratch.try_reserve_exact(HISTORY_REPLAY_MAX_FIELD_BYTES).is_ok())).unwrap_or(false) {
            return Err(builder.fault(|| DbError::Unavailable("history scratch page reservation failed".to_string())));
        }
        if !builder
            .edit_cursor(|cursor| {
                cursor.scratch.as_mut().is_some_and(|scratch| {
                    scratch.resize(HISTORY_REPLAY_MAX_FIELD_BYTES, 0);
                    true
                })
            })
            .unwrap_or(false)
        {
            return Err(builder.fault(|| DbError::Unavailable("history scratch page publication failed".to_string())));
        }
        builder.finish(retained_operation_bytes, retained_result_bytes)
    }

    fn retained_bytes(&self) -> Option<u64> {
        Some(self.retained_operation_bytes)
    }

    fn preflight_result_range(&self, result_len: u64, len: u64) -> Result<u64, DbError> {
        if self.operation_ids.len() >= HISTORY_REPLAY_MAX_OPERATION_IDS {
            return Err(DbError::LimitExceeded("history operation item credit"));
        }
        match result_len.checked_add(len) {
            Some(end) if end <= HISTORY_REPLAY_RESULT_BYTES => Ok(end),
            _ => Err(DbError::LimitExceeded("history result byte credit")),
        }
    }

    #[cfg(test)]
    fn retain_source_page(&mut self, page: Vec<u8>) -> Result<(), Vec<u8>> {
        let Some(slot) = self.source_pages.get_mut(self.source_page_count) else {
            return Err(page);
        };
        if slot.is_some() {
            return Err(page);
        }
        *slot = Some(page);
        self.source_page_count += 1;
        Ok(())
    }
}

pub struct HistoryReplayReservationCloseCursor {
    source_pages: Option<Vec<Option<Vec<u8>>>>,
    source_page_count: usize,
    result_pages: Option<Vec<Option<Vec<u8>>>>,
    operation_ids: Option<Vec<HistoryTextRange>>,
    entries: Option<Vec<ArtifactHistoryEntry>>,
    scratch: Option<Vec<u8>>,
    retained_operation_bytes: u64,
    retained_result_bytes: u64,
    started: bool,
}

impl HistoryReplayReservationCloseCursor {
    pub fn new(reservation: HistoryReplayReservation) -> Self {
        let HistoryReplayReservation { source_pages, source_page_count, result_pages, operation_ids, entries, scratch, retained_operation_bytes, retained_result_bytes } = reservation;
        Self { source_pages: Some(source_pages), source_page_count, result_pages: Some(result_pages), operation_ids: Some(operation_ids), entries: Some(entries), scratch, retained_operation_bytes, retained_result_bytes, started: false }
    }

    pub fn close_step(&mut self) -> bool {
        self.started = true;
        if self.source_page_count != 0 {
            self.source_page_count -= 1;
            if let Some(owner) = self.source_pages.as_mut().and_then(|pages| pages.get_mut(self.source_page_count)).and_then(Option::take) {
                drop(owner);
            }
            return true;
        }
        if self.operation_ids.as_mut().is_some_and(|owners| owners.pop().is_some()) {
            return true;
        }
        if self.entries.as_mut().is_some_and(|owners| owners.pop().is_some()) {
            return true;
        }
        if let Some(owner) = self.result_pages.as_mut().and_then(Vec::pop) {
            drop(owner);
            return true;
        }
        if self.scratch.take().is_some() {
            return true;
        }
        if self.source_pages.take().is_some() {
            return true;
        }
        if self.operation_ids.take().is_some() {
            return true;
        }
        if self.entries.take().is_some() {
            return true;
        }
        if self.result_pages.take().is_some() {
            return true;
        }
        false
    }

    pub fn resume(&mut self) -> Option<HistoryReplayReservation> {
        if self.started {
            return None;
        }
        Some(HistoryReplayReservation {
            source_pages: self.source_pages.take()?,
            source_page_count: self.source_page_count,
            result_pages: self.result_pages.take()?,
            operation_ids: self.operation_ids.take()?,
            entries: self.entries.take()?,
            scratch: self.scratch.take(),
            retained_operation_bytes: self.retained_operation_bytes,
            retained_result_bytes: self.retained_result_bytes,
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.source_pages.is_none() && self.result_pages.is_none() && self.operation_ids.is_none() && self.entries.is_none() && self.scratch.is_none()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HistoryReplayReservationConstructionToken {
    slot: usize,
    generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HistoryReplayReservationConstructionHandbackRejection {
    slot: usize,
    generation: u64,
}

struct HistoryReplayReservationConstructionSlot {
    generation: u64,
    occupied: bool,
    checked_out: bool,
    error: Option<DbError>,
    cursor: Option<HistoryReplayReservationCloseCursor>,
}

impl Default for HistoryReplayReservationConstructionSlot {
    fn default() -> Self {
        Self { generation: 0, occupied: false, checked_out: false, error: None, cursor: None }
    }
}

struct HistoryReplayReservationConstructionRegistry {
    slots: [HistoryReplayReservationConstructionSlot; HISTORY_REPLAY_CONSTRUCTION_SLOTS],
    next_generation: u64,
}

fn history_replay_reservation_construction_registry() -> &'static std::sync::Mutex<HistoryReplayReservationConstructionRegistry> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<HistoryReplayReservationConstructionRegistry>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(HistoryReplayReservationConstructionRegistry { slots: std::array::from_fn(|_| HistoryReplayReservationConstructionSlot::default()), next_generation: 1 }))
}

fn claim_history_replay_reservation_construction() -> Result<HistoryReplayReservationConstructionToken, DbError> {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = registry.slots.iter().position(|slot| !slot.occupied) else {
        return Err(DbError::Unavailable("history construction registry capacity exhausted".to_string()));
    };
    let generation = registry.next_generation;
    let Some(next_generation) = generation.checked_add(1) else {
        return Err(DbError::LimitExceeded("history construction generation"));
    };
    registry.next_generation = next_generation;
    registry.slots[slot] = HistoryReplayReservationConstructionSlot {
        generation,
        occupied: true,
        checked_out: true,
        error: None,
        cursor: Some(HistoryReplayReservationCloseCursor {
            source_pages: Some(Vec::new()),
            source_page_count: 0,
            result_pages: Some(Vec::new()),
            operation_ids: Some(Vec::new()),
            entries: Some(Vec::new()),
            scratch: Some(Vec::new()),
            retained_operation_bytes: 0,
            retained_result_bytes: 0,
            started: false,
        }),
    };
    Ok(HistoryReplayReservationConstructionToken { slot, generation })
}

fn handback_history_replay_reservation_construction(token: &HistoryReplayReservationConstructionToken) -> Result<(), HistoryReplayReservationConstructionHandbackRejection> {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = registry.slots.get_mut(token.slot) else {
        return Err(HistoryReplayReservationConstructionHandbackRejection { slot: token.slot, generation: token.generation });
    };
    if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.cursor.is_none() {
        return Err(HistoryReplayReservationConstructionHandbackRejection { slot: token.slot, generation: token.generation });
    }
    slot.checked_out = false;
    Ok(())
}

fn release_history_replay_reservation_construction(token: &HistoryReplayReservationConstructionToken) -> bool {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = registry.slots.get_mut(token.slot) else {
        return false;
    };
    if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() || slot.cursor.is_some() {
        return false;
    }
    slot.generation = 0;
    slot.occupied = false;
    slot.checked_out = false;
    true
}

pub(crate) fn take_history_replay_reservation_construction_fault(generation: u64) -> Option<HistoryReplayReservationConstructionFault> {
    let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let slot_index = registry.slots.iter().position(|slot| slot.occupied && !slot.checked_out && slot.generation == generation)?;
    let slot = &mut registry.slots[slot_index];
    slot.checked_out = true;
    Some(HistoryReplayReservationConstructionFault { token: Some(HistoryReplayReservationConstructionToken { slot: slot_index, generation }), unregistered_error: None })
}

struct HistoryReplayReservationConstructionBuilder {
    token: Option<HistoryReplayReservationConstructionToken>,
}

impl HistoryReplayReservationConstructionBuilder {
    fn new() -> Result<Self, HistoryReplayReservationConstructionFault> {
        let token = match claim_history_replay_reservation_construction() {
            Ok(token) => token,
            Err(error) => return Err(HistoryReplayReservationConstructionFault::unregistered(error)),
        };
        Ok(Self { token: Some(token) })
    }

    fn edit_cursor<T>(&mut self, edit: impl FnOnce(&mut HistoryReplayReservationCloseCursor) -> T) -> Option<T> {
        let token = self.token.as_ref()?;
        let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = registry.slots.get_mut(token.slot)?;
        if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() {
            return None;
        }
        slot.cursor.as_mut().map(edit)
    }

    fn read_cursor<T>(&self, read: impl FnOnce(&HistoryReplayReservationCloseCursor) -> T) -> Option<T> {
        let token = self.token.as_ref()?;
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = registry.slots.get(token.slot)?;
        if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() {
            return None;
        }
        slot.cursor.as_ref().map(read)
    }

    fn fault(mut self, error: impl FnOnce() -> DbError) -> HistoryReplayReservationConstructionFault {
        let Some(token) = self.token.take() else {
            return HistoryReplayReservationConstructionFault::unregistered(DbError::Unavailable("history construction token was unavailable".to_string()));
        };
        {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = registry.slots.get_mut(token.slot) else {
                return HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None };
            };
            if slot.occupied && slot.checked_out && slot.generation == token.generation && slot.error.is_none() && slot.cursor.is_some() {
                slot.error = Some(error());
            }
        }
        HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None }
    }

    fn finish(mut self, retained_operation_bytes: u64, retained_result_bytes: u64) -> Result<HistoryReplayReservation, HistoryReplayReservationConstructionFault> {
        let Some(token) = self.token.take() else {
            return Err(HistoryReplayReservationConstructionFault::unregistered(DbError::Unavailable("history construction token was unavailable".to_string())));
        };
        let reservation = {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = registry.slots.get_mut(token.slot) else {
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            };
            if !slot.occupied || !slot.checked_out || slot.generation != token.generation || slot.error.is_some() {
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            }
            let Some(mut cursor) = slot.cursor.take() else {
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            };
            cursor.retained_operation_bytes = retained_operation_bytes;
            cursor.retained_result_bytes = retained_result_bytes;
            let Some(reservation) = cursor.resume() else {
                slot.cursor = Some(cursor);
                return Err(HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None });
            };
            slot.generation = 0;
            slot.occupied = false;
            slot.checked_out = false;
            reservation
        };
        Ok(reservation)
    }
}

impl Drop for HistoryReplayReservationConstructionBuilder {
    fn drop(&mut self) {
        if let Some(token) = self.token.as_ref() {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(slot) = registry.slots.get_mut(token.slot).filter(|slot| slot.occupied && slot.checked_out && slot.generation == token.generation && slot.error.is_none() && slot.cursor.is_some()) {
                slot.error = Some(DbError::Unavailable("history reservation construction unwound".to_string()));
            }
        }
        if let Some(token) = self.token.take() {
            let _ = handback_history_replay_reservation_construction(&token);
        }
    }
}

pub(crate) struct HistoryReplayReservationConstructionFault {
    token: Option<HistoryReplayReservationConstructionToken>,
    unregistered_error: Option<DbError>,
}

impl HistoryReplayReservationConstructionFault {
    fn unregistered(error: DbError) -> Self {
        Self { token: None, unregistered_error: Some(error) }
    }

    pub(crate) fn generation(&self) -> Option<u64> {
        self.token.as_ref().map(|token| token.generation)
    }

    pub(crate) fn take_error(&mut self) -> Option<DbError> {
        if self.unregistered_error.is_some() {
            return self.unregistered_error.take();
        }
        let token = self.token.as_ref()?;
        let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = registry.slots.get_mut(token.slot)?;
        if !slot.occupied || !slot.checked_out || slot.generation != token.generation {
            return None;
        }
        slot.error.take()
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if self.unregistered_error.take().is_some() {
            return true;
        }
        let Some(token) = self.token.as_ref() else {
            return false;
        };
        {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(slot) = registry.slots.get_mut(token.slot) else {
                return false;
            };
            if !slot.occupied || !slot.checked_out || slot.generation != token.generation {
                return false;
            }
            if let Some(cursor) = slot.cursor.as_mut() {
                if cursor.close_step() {
                    return true;
                }
                if cursor.terminal_is_empty() {
                    slot.cursor = None;
                    return true;
                }
            }
            if slot.error.take().is_some() {
                return true;
            }
        }
        if release_history_replay_reservation_construction(token) {
            self.token = None;
            return true;
        }
        false
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.token.is_none() && self.unregistered_error.is_none()
    }

    #[cfg(test)]
    fn retained_result_page_count(&self) -> usize {
        let Some(token) = self.token.as_ref() else {
            return 0;
        };
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        registry.slots.get(token.slot).filter(|slot| slot.occupied && slot.generation == token.generation).and_then(|slot| slot.cursor.as_ref()).and_then(|cursor| cursor.result_pages.as_ref()).map_or(0, Vec::len)
    }

    #[cfg(test)]
    fn retained_result_page_pointer(&self, index: usize) -> Option<*const u8> {
        let token = self.token.as_ref()?;
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        registry
            .slots
            .get(token.slot)
            .filter(|slot| slot.occupied && slot.generation == token.generation)
            .and_then(|slot| slot.cursor.as_ref())
            .and_then(|cursor| cursor.result_pages.as_ref())
            .and_then(|owners| owners.get(index))
            .and_then(Option::as_ref)
            .map(Vec::as_ptr)
    }

    #[cfg(test)]
    fn retained_error_pointer(&self) -> Option<*const u8> {
        if let Some(DbError::Unavailable(message)) = self.unregistered_error.as_ref() {
            return Some(message.as_ptr());
        }
        let token = self.token.as_ref()?;
        let registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match registry.slots.get(token.slot).filter(|slot| slot.occupied && slot.generation == token.generation)?.error.as_ref()? {
            DbError::Unavailable(message) => Some(message.as_ptr()),
            _ => None,
        }
    }
}

impl std::fmt::Debug for HistoryReplayReservationConstructionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("HistoryReplayReservationConstructionFault").field("generation", &self.generation()).field("terminal_is_empty", &self.terminal_is_empty()).finish()
    }
}

impl Drop for HistoryReplayReservationConstructionFault {
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            let _ = handback_history_replay_reservation_construction(&token);
        }
    }
}

impl Drop for HistoryReplayReservationCloseCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "history replay reservation reached Drop before retained retirement or exact resume");
    }
}

struct HistoryPageSet {
    pages: Vec<Option<Vec<u8>>>,
    len: u64,
}

impl HistoryPageSet {
    fn byte(&self, offset: u64) -> Result<u8, DbError> {
        if offset >= self.len {
            return Err(DbError::Corrupt("history page cursor exceeded the segment".to_string()));
        }
        let page = usize::try_from(offset / HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page index"))?;
        let within = usize::try_from(offset % HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page offset"))?;
        self.pages.get(page).and_then(Option::as_ref).and_then(|bytes| bytes.get(within)).copied().ok_or_else(|| DbError::Corrupt("history page owner is missing admitted bytes".to_string()))
    }

    fn page_slice(&self, offset: u64, maximum: u64) -> Result<&[u8], DbError> {
        if offset >= self.len {
            return Ok(&[]);
        }
        let page = usize::try_from(offset / HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page index"))?;
        let within = usize::try_from(offset % HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history page offset"))?;
        let bytes = self.pages.get(page).and_then(Option::as_ref).ok_or_else(|| DbError::Corrupt("history page owner is missing".to_string()))?;
        let available = bytes.len().checked_sub(within).ok_or_else(|| DbError::Corrupt("history page offset exceeds owner".to_string()))?;
        let take = available.min(usize::try_from(maximum).unwrap_or(usize::MAX));
        Ok(&bytes[within..within + take])
    }

    fn read_varint(&self, pos: &mut u64, end: u64) -> Result<u64, DbError> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            if *pos >= end {
                return Err(DbError::Corrupt("history varint exceeds its admitted range".to_string()));
            }
            let byte = self.byte(*pos)?;
            *pos = pos.checked_add(1).ok_or(DbError::LimitExceeded("history varint offset"))?;
            value |= u64::from(byte & 0x7f).checked_shl(shift).ok_or(DbError::LimitExceeded("history varint"))?;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(DbError::Corrupt("history varint exceeds ten bytes".to_string()))
    }

    fn read_range(&self, pos: &mut u64, end: u64, maximum: u64) -> Result<std::ops::Range<u64>, DbError> {
        let len = self.read_varint(pos, end)?;
        if len > maximum {
            return Err(DbError::LimitExceeded("history envelope field bytes"));
        }
        let start = *pos;
        let next = start.checked_add(len).ok_or(DbError::LimitExceeded("history envelope field range"))?;
        if next > end {
            return Err(DbError::Corrupt("history envelope field exceeds its frame payload".to_string()));
        }
        *pos = next;
        Ok(start..next)
    }

    fn copy_small<'a>(&self, range: std::ops::Range<u64>, scratch: &'a mut [u8]) -> Result<&'a [u8], DbError> {
        let len = range.end.checked_sub(range.start).ok_or(DbError::LimitExceeded("history copied range"))?;
        if len > HISTORY_REPLAY_PAGE_BYTES {
            return Err(DbError::LimitExceeded("history scratch page bytes"));
        }
        let first = self.page_slice(range.start, len)?;
        scratch[..first.len()].copy_from_slice(first);
        let remaining = usize::try_from(len).map_err(|_| DbError::LimitExceeded("history scratch page bytes"))?.checked_sub(first.len()).ok_or(DbError::LimitExceeded("history scratch page bytes"))?;
        if remaining != 0 {
            let offset = range.start.checked_add(first.len() as u64).ok_or(DbError::LimitExceeded("history scratch page offset"))?;
            let second = self.page_slice(offset, remaining as u64)?;
            if second.len() != remaining {
                return Err(DbError::Corrupt("history scratch page is truncated".to_string()));
            }
            scratch[first.len()..first.len() + remaining].copy_from_slice(second);
        }
        Ok(&scratch[..len as usize])
    }

    fn read_array<const N: usize>(&self, pos: &mut u64, end: u64, scratch: &mut [u8]) -> Result<[u8; N], DbError> {
        let next = pos.checked_add(N as u64).ok_or(DbError::LimitExceeded("history scalar range"))?;
        if next > end {
            return Err(DbError::Corrupt("history scalar exceeds its frame payload".to_string()));
        }
        let owner = self.copy_small(*pos..next, scratch)?;
        *pos = next;
        owner.try_into().map_err(|_| DbError::Corrupt("history scalar length mismatch".to_string()))
    }
}

enum HistoryFrameToken {
    End,
    TxBegin,
    Command { offset: u64, len: u64 },
    Frontier { offset: u64, len: u64 },
    Other,
}

enum HistoryFrameStage {
    BodyLen,
    Kind,
    Flags,
    Payload,
    StoredCrc,
    BackLen,
    Finish,
}

struct HistoryFrameCursor {
    frame_start: u64,
    pos: u64,
    body_len: u64,
    varint_shift: u32,
    kind: u8,
    payload_start: u64,
    payload_remaining: u64,
    crc: protocol::codec::Crc32cCursor,
    stored_crc: [u8; 4],
    stored_crc_pos: usize,
    back_len: [u8; 4],
    back_len_pos: usize,
    stage: HistoryFrameStage,
}

impl HistoryFrameCursor {
    fn new(offset: u64) -> Self {
        Self {
            frame_start: offset,
            pos: offset,
            body_len: 0,
            varint_shift: 0,
            kind: 0,
            payload_start: 0,
            payload_remaining: 0,
            crc: protocol::codec::Crc32cCursor::new(),
            stored_crc: [0; 4],
            stored_crc_pos: 0,
            back_len: [0; 4],
            back_len_pos: 0,
            stage: HistoryFrameStage::BodyLen,
        }
    }

    fn step(&mut self, pages: &HistoryPageSet) -> Result<Option<HistoryFrameToken>, DbError> {
        if self.frame_start == pages.len {
            return Ok(Some(HistoryFrameToken::End));
        }
        match self.stage {
            HistoryFrameStage::BodyLen => {
                let byte = pages.byte(self.pos)?;
                self.pos = self.pos.checked_add(1).ok_or(DbError::LimitExceeded("history frame offset"))?;
                self.body_len |= u64::from(byte & 0x7f).checked_shl(self.varint_shift).ok_or(DbError::LimitExceeded("history frame body length"))?;
                if byte & 0x80 == 0 {
                    if self.body_len < 2 || self.body_len > HISTORY_REPLAY_MAX_FRAME_BYTES {
                        return Err(DbError::LimitExceeded("history frame body bytes"));
                    }
                    self.stage = HistoryFrameStage::Kind;
                } else {
                    self.varint_shift = self.varint_shift.checked_add(7).ok_or(DbError::LimitExceeded("history frame body varint"))?;
                    if self.varint_shift >= 70 {
                        return Err(DbError::Corrupt("history frame body varint exceeds ten bytes".to_string()));
                    }
                }
            }
            HistoryFrameStage::Kind => {
                self.kind = pages.byte(self.pos)?;
                self.crc.update_page(&[self.kind]);
                self.pos = self.pos.checked_add(1).ok_or(DbError::LimitExceeded("history frame offset"))?;
                self.stage = HistoryFrameStage::Flags;
            }
            HistoryFrameStage::Flags => {
                let flags = pages.byte(self.pos)?;
                if flags & protocol::wire::FRAME_FLAG_COMPRESSED != 0 {
                    return Err(DbError::Corrupt("compressed WAL history frame is unsupported".to_string()));
                }
                self.crc.update_page(&[flags]);
                self.pos = self.pos.checked_add(1).ok_or(DbError::LimitExceeded("history frame offset"))?;
                self.payload_start = self.pos;
                self.payload_remaining = self.body_len - 2;
                self.stage = if self.payload_remaining == 0 { HistoryFrameStage::StoredCrc } else { HistoryFrameStage::Payload };
            }
            HistoryFrameStage::Payload => {
                let maximum = self.payload_remaining.min(HISTORY_REPLAY_PAGE_BYTES);
                let page = pages.page_slice(self.pos, maximum)?;
                if page.is_empty() {
                    return Err(DbError::Corrupt("history frame payload is truncated".to_string()));
                }
                self.crc.update_page(page);
                self.pos = self.pos.checked_add(page.len() as u64).ok_or(DbError::LimitExceeded("history frame payload offset"))?;
                self.payload_remaining = self.payload_remaining.checked_sub(page.len() as u64).ok_or(DbError::LimitExceeded("history frame payload bytes"))?;
                if self.payload_remaining == 0 {
                    self.stage = HistoryFrameStage::StoredCrc;
                }
            }
            HistoryFrameStage::StoredCrc => {
                self.stored_crc[self.stored_crc_pos] = pages.byte(self.pos)?;
                self.pos = self.pos.checked_add(1).ok_or(DbError::LimitExceeded("history frame CRC offset"))?;
                self.stored_crc_pos += 1;
                if self.stored_crc_pos == self.stored_crc.len() {
                    self.stage = HistoryFrameStage::BackLen;
                }
            }
            HistoryFrameStage::BackLen => {
                self.back_len[self.back_len_pos] = pages.byte(self.pos)?;
                self.pos = self.pos.checked_add(1).ok_or(DbError::LimitExceeded("history frame trailer offset"))?;
                self.back_len_pos += 1;
                if self.back_len_pos == self.back_len.len() {
                    self.stage = HistoryFrameStage::Finish;
                }
            }
            HistoryFrameStage::Finish => {
                let stored_crc = u32::from_le_bytes(self.stored_crc);
                if stored_crc != self.crc.finish() {
                    return Err(DbError::Corrupt("history frame CRC mismatch".to_string()));
                }
                let frame_len = self.pos.checked_sub(self.frame_start).ok_or(DbError::LimitExceeded("history frame length"))?;
                if u64::from(u32::from_le_bytes(self.back_len)) != frame_len {
                    return Err(DbError::Corrupt("history frame back length mismatch".to_string()));
                }
                let payload_len = self.body_len - 2;
                let token = match self.kind {
                    db_wal::WAL_TX_BEGIN => HistoryFrameToken::TxBegin,
                    db_wal::WAL_COMMAND => HistoryFrameToken::Command { offset: self.payload_start, len: payload_len },
                    db_wal::WAL_FRONTIER => HistoryFrameToken::Frontier { offset: self.payload_start, len: payload_len },
                    kind if (db_wal::WAL_SEGMENT_HEADER..=db_wal::WAL_MIGRATION).contains(&kind) || kind == protocol::wire::REC_COMMIT => HistoryFrameToken::Other,
                    kind => return Err(DbError::Corrupt(format!("unexpected frame kind {kind:#x} in history replay"))),
                };
                return Ok(Some(token));
            }
        }
        Ok(None)
    }
}

enum HistoryEnvelopeField {
    MutationId,
    Document,
    Actor,
    DependencyCount,
    Dependency,
    DiffSchema,
    DiffPayload,
    InverseSchema,
    InversePayload,
    ClockActor,
    ClockPhysical,
    ClockLogical,
    Done,
}

struct HistoryEnvelopeCursor {
    pos: u64,
    end: u64,
    field: HistoryEnvelopeField,
    dependencies: u64,
    mutation_id: Option<std::ops::Range<u64>>,
}

impl HistoryEnvelopeCursor {
    fn new(pages: &HistoryPageSet, offset: u64, len: u64) -> Result<Self, DbError> {
        let end = offset.checked_add(len).ok_or(DbError::LimitExceeded("history command range"))?;
        if end > pages.len {
            return Err(DbError::Corrupt("history command range exceeds retained pages".to_string()));
        }
        Ok(Self { pos: offset, end, field: HistoryEnvelopeField::MutationId, dependencies: 0, mutation_id: None })
    }

    fn skip_text(&mut self, pages: &HistoryPageSet, scratch: &mut [u8]) -> Result<(), DbError> {
        let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
        let scratch = pages.copy_small(range, scratch)?;
        std::str::from_utf8(&scratch).map_err(|_| DbError::Corrupt("history envelope text is not valid utf-8".to_string()))?;
        Ok(())
    }

    fn step(&mut self, pages: &HistoryPageSet, scratch: &mut [u8]) -> Result<Option<std::ops::Range<u64>>, DbError> {
        match self.field {
            HistoryEnvelopeField::MutationId => {
                let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
                let value = pages.copy_small(range.clone(), scratch)?;
                std::str::from_utf8(value).map_err(|_| DbError::Corrupt("history operation id is not valid utf-8".to_string()))?;
                self.mutation_id = Some(range);
                self.field = HistoryEnvelopeField::Document;
            }
            HistoryEnvelopeField::Document => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::Actor;
            }
            HistoryEnvelopeField::Actor => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::DependencyCount;
            }
            HistoryEnvelopeField::DependencyCount => {
                self.dependencies = pages.read_varint(&mut self.pos, self.end)?;
                if self.dependencies > HISTORY_REPLAY_MAX_OPERATION_IDS as u64 {
                    return Err(DbError::LimitExceeded("history dependency item credit"));
                }
                self.field = if self.dependencies == 0 { HistoryEnvelopeField::DiffSchema } else { HistoryEnvelopeField::Dependency };
            }
            HistoryEnvelopeField::Dependency => {
                self.skip_text(pages, scratch)?;
                self.dependencies -= 1;
                if self.dependencies == 0 {
                    self.field = HistoryEnvelopeField::DiffSchema;
                }
            }
            HistoryEnvelopeField::DiffSchema => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::DiffPayload;
            }
            HistoryEnvelopeField::DiffPayload => {
                pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FRAME_BYTES)?;
                self.field = HistoryEnvelopeField::InverseSchema;
            }
            HistoryEnvelopeField::InverseSchema => {
                self.skip_text(pages, scratch)?;
                self.field = HistoryEnvelopeField::InversePayload;
            }
            HistoryEnvelopeField::InversePayload => {
                pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FRAME_BYTES)?;
                self.field = HistoryEnvelopeField::ClockActor;
            }
            HistoryEnvelopeField::ClockActor => {
                pages.read_varint(&mut self.pos, self.end)?;
                self.field = HistoryEnvelopeField::ClockPhysical;
            }
            HistoryEnvelopeField::ClockPhysical => {
                pages.read_varint(&mut self.pos, self.end)?;
                self.field = HistoryEnvelopeField::ClockLogical;
            }
            HistoryEnvelopeField::ClockLogical => {
                pages.read_varint(&mut self.pos, self.end)?;
                if self.pos != self.end {
                    return Err(DbError::Corrupt("history envelope has trailing bytes".to_string()));
                }
                self.field = HistoryEnvelopeField::Done;
            }
            HistoryEnvelopeField::Done => return Ok(self.mutation_id.take()),
        }
        Ok(None)
    }
}

enum HistoryFrontierField {
    Document,
    Head,
    Commit,
    Chain,
    Epoch,
}

struct HistoryFrontierCursor {
    pos: u64,
    end: u64,
    field: HistoryFrontierField,
    head_seq: u64,
    commit_seq: u64,
    chain_hash: [u8; 32],
}

impl HistoryFrontierCursor {
    fn new(pages: &HistoryPageSet, offset: u64, len: u64) -> Result<Self, DbError> {
        let end = offset.checked_add(len).ok_or(DbError::LimitExceeded("history frontier range"))?;
        if end > pages.len {
            return Err(DbError::Corrupt("history frontier range exceeds retained pages".to_string()));
        }
        Ok(Self { pos: offset, end, field: HistoryFrontierField::Document, head_seq: 0, commit_seq: 0, chain_hash: [0; 32] })
    }

    fn step(&mut self, pages: &HistoryPageSet, scratch: &mut [u8]) -> Result<Option<(u64, u64, [u8; 32], u64)>, DbError> {
        match self.field {
            HistoryFrontierField::Document => {
                let range = pages.read_range(&mut self.pos, self.end, HISTORY_REPLAY_MAX_FIELD_BYTES as u64)?;
                let scratch = pages.copy_small(range, scratch)?;
                std::str::from_utf8(&scratch).map_err(|_| DbError::Corrupt("history frontier document is not valid utf-8".to_string()))?;
                self.field = HistoryFrontierField::Head;
            }
            HistoryFrontierField::Head => {
                self.head_seq = u64::from_le_bytes(pages.read_array(&mut self.pos, self.end, scratch)?);
                self.field = HistoryFrontierField::Commit;
            }
            HistoryFrontierField::Commit => {
                self.commit_seq = u64::from_le_bytes(pages.read_array(&mut self.pos, self.end, scratch)?);
                self.field = HistoryFrontierField::Chain;
            }
            HistoryFrontierField::Chain => {
                self.chain_hash = pages.read_array(&mut self.pos, self.end, scratch)?;
                self.field = HistoryFrontierField::Epoch;
            }
            HistoryFrontierField::Epoch => {
                let epoch = u64::from_le_bytes(pages.read_array(&mut self.pos, self.end, scratch)?);
                if self.pos != self.end {
                    return Err(DbError::Corrupt("history frontier has trailing bytes".to_string()));
                }
                return Ok(Some((self.head_seq, self.commit_seq, self.chain_hash, epoch)));
            }
        }
        Ok(None)
    }
}

type HistorySegmentLenFuture = Pin<Box<dyn Future<Output = Result<u64, DbError>> + Send + 'static>>;
type HistoryPageReadFuture = Pin<Box<dyn Future<Output = Result<Vec<u8>, DbError>> + Send + 'static>>;

enum HistoryReplayPhase {
    Probe { index: u64 },
    SegmentLen { index: u64, future: HistorySegmentLenFuture },
    PageStart { index: u64, len: u64, offset: u64 },
    PageRead { index: u64, len: u64, offset: u64, requested: u64, future: HistoryPageReadFuture },
    Frame { index: u64, cursor: HistoryFrameCursor },
    Envelope { index: u64, next_offset: u64, cursor: HistoryEnvelopeCursor },
    CopyMutation { index: u64, next_offset: u64, range: std::ops::Range<u64>, copied: u64, result_start: u64 },
    Frontier { index: u64, next_offset: u64, cursor: HistoryFrontierCursor },
    ClearPending { index: u64, next_offset: u64 },
    Publish { index: u64, next_offset: u64, head_seq: u64, commit_seq: u64, chain_hash: [u8; 32], epoch: u64 },
    Retire { next_index: u64 },
    FinalizeSuccess,
}

enum HistoryReplayTransition {
    InProgress,
    FaultRetire,
    Complete,
}

pub struct HistoryReplayFuture {
    storage: Arc<db_storage::DbBackend>,
    document: Arc<ArtifactId>,
    phase: Option<HistoryReplayPhase>,
    transition: HistoryReplayTransition,
    pages: HistoryPageSet,
    page_count: usize,
    reservation: Option<HistoryReplayReservation>,
    reservation_close: Option<HistoryReplayReservationCloseCursor>,
    terminal_page: Option<Vec<u8>>,
    terminal_error: Option<DbError>,
    result_len: u64,
    pending_start: usize,
    operation_generation: u64,
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    panic_before_transition_commit: bool,
}

impl HistoryReplayFuture {
    fn new(storage: Arc<db_storage::DbBackend>, document: ArtifactId, operation_generation: u64, cancelled: Arc<std::sync::atomic::AtomicBool>, mut reservation: HistoryReplayReservation) -> Self {
        let source_pages = std::mem::take(&mut reservation.source_pages);
        reservation.source_page_count = 0;
        Self {
            storage,
            document: Arc::new(document),
            phase: Some(HistoryReplayPhase::Probe { index: 0 }),
            transition: HistoryReplayTransition::InProgress,
            pages: HistoryPageSet { pages: source_pages, len: 0 },
            page_count: 0,
            reservation: Some(reservation),
            reservation_close: None,
            terminal_page: None,
            terminal_error: None,
            result_len: 0,
            pending_start: 0,
            operation_generation,
            cancelled,
            panic_before_transition_commit: false,
        }
    }

    pub fn request_close(&mut self, error: DbError) {
        if self.terminal_error.is_none() {
            self.terminal_error = Some(error);
        }
        if !matches!(self.transition, HistoryReplayTransition::Complete) {
            self.transition = HistoryReplayTransition::FaultRetire;
        }
    }

    pub fn close_step(&mut self) -> bool {
        if self.terminal_page.take().is_some() {
            return true;
        }
        if self.page_count != 0 {
            self.page_count -= 1;
            drop(self.pages.pages[self.page_count].take());
            return true;
        }
        if !self.pages.pages.is_empty() {
            drop(std::mem::take(&mut self.pages.pages));
            return true;
        }
        if self.reservation_close.is_none() {
            if let Some(reservation) = self.reservation.take() {
                self.reservation_close = Some(HistoryReplayReservationCloseCursor::new(reservation));
                return true;
            }
        }
        if self.reservation_close.as_mut().is_some_and(HistoryReplayReservationCloseCursor::close_step) {
            return true;
        }
        if self.reservation_close.as_ref().is_some_and(HistoryReplayReservationCloseCursor::terminal_is_empty) {
            self.reservation_close = None;
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal_page.is_none() && self.page_count == 0 && self.pages.pages.is_empty() && self.reservation.is_none() && self.reservation_close.is_none() && self.phase.is_none() && matches!(self.transition, HistoryReplayTransition::Complete)
    }

    fn begin_fault(&mut self, error: DbError) {
        self.request_close(error);
    }

    fn fail_pending(&mut self, error: DbError) {
        self.begin_fault(error);
    }

    fn copy_mutation_fragment(pages: &HistoryPageSet, reservation: &mut Option<HistoryReplayReservation>, range: &std::ops::Range<u64>, copied: u64, result_start: u64) -> Result<u64, DbError> {
        let remaining = range.end.checked_sub(range.start).and_then(|len| len.checked_sub(copied)).ok_or(DbError::LimitExceeded("history result copy range"))?;
        if remaining == 0 {
            return Ok(0);
        }
        let source_offset = range.start.checked_add(copied).ok_or(DbError::LimitExceeded("history result source offset"))?;
        let destination_offset = result_start.checked_add(copied).ok_or(DbError::LimitExceeded("history result destination offset"))?;
        let source = pages.page_slice(source_offset, remaining.min(HISTORY_REPLAY_PAGE_BYTES))?;
        let page_index = usize::try_from(destination_offset / HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history result page index"))?;
        let within = usize::try_from(destination_offset % HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history result page offset"))?;
        let reservation = reservation.as_mut().ok_or(DbError::Closed)?;
        let destination = reservation.result_pages.get_mut(page_index).and_then(Option::as_mut).ok_or_else(|| DbError::Corrupt("history result page owner is missing".to_string()))?;
        let take = source.len().min(destination.len().saturating_sub(within));
        if take == 0 {
            return Err(DbError::Corrupt("history result copy made no progress".to_string()));
        }
        destination[within..within + take].copy_from_slice(&source[..take]);
        Ok(take as u64)
    }

    fn finish_view(&mut self) -> Result<ArtifactHistoryView, DbError> {
        let Some(reservation) = self.reservation.as_ref() else {
            return Err(DbError::Closed);
        };
        let retained_bytes = Ok(reservation.retained_result_bytes);
        let retained_items = reservation
            .result_pages
            .len()
            .checked_add(reservation.operation_ids.len())
            .and_then(|items| items.checked_add(reservation.entries.len()))
            .and_then(|items| items.checked_add(1))
            .ok_or(DbError::LimitExceeded("history retained result items"));
        let (Ok(retained_bytes), Ok(retained_items)) = (retained_bytes, retained_items) else {
            return Err(DbError::LimitExceeded("history retained result credit"));
        };
        let Some(mut reservation) = self.reservation.take() else {
            return Err(DbError::Closed);
        };
        if reservation.scratch.is_some() || !reservation.source_pages.is_empty() {
            self.reservation = Some(reservation);
            return Err(DbError::Internal("history final view retained non-result owners".to_string()));
        }
        Ok(ArtifactHistoryView { entries: reservation.entries, operation_ids: reservation.operation_ids, result_pages: reservation.result_pages, result_len: self.result_len, retained_bytes, retained_items })
    }

    fn next(self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Result<ArtifactHistoryView, DbError>> {
        let this = self.get_mut();
        if this.operation_generation == 0 {
            this.request_close(DbError::Closed);
        }
        if this.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            this.request_close(DbError::Closed);
        }
        if matches!(this.transition, HistoryReplayTransition::Complete) {
            return std::task::Poll::Ready(Err(this.terminal_error.take().unwrap_or(DbError::Closed)));
        }
        if matches!(this.transition, HistoryReplayTransition::FaultRetire) {
            if this.phase.take().is_some() {
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            if this.close_step() {
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            this.transition = HistoryReplayTransition::Complete;
            return std::task::Poll::Ready(Err(this.terminal_error.take().unwrap_or(DbError::Closed)));
        }
        let mut next = None;
        let mut fault = None;
        let mut finalize = false;
        let Some(active) = this.phase.as_mut() else {
            this.request_close(DbError::Internal("history replay transition authority was missing".to_string()));
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        };
        match active {
            HistoryReplayPhase::Probe { index } => {
                if let Some(error) = this.terminal_error.take() {
                    fault = Some(error);
                } else {
                    let index = *index;
                    let storage = this.storage.clone();
                    let document = this.document.clone();
                    next = Some(HistoryReplayPhase::SegmentLen { index, future: Box::pin(async move { storage.wal().await.segment_len(&document, index).await }) });
                }
            }
            HistoryReplayPhase::SegmentLen { index, future } => match future.as_mut().poll(context) {
                std::task::Poll::Pending => {}
                std::task::Poll::Ready(Err(DbError::NotFound(_))) => {
                    if let Some(error) = this.terminal_error.take() {
                        fault = Some(error);
                    } else {
                        next = Some(HistoryReplayPhase::FinalizeSuccess);
                    }
                }
                std::task::Poll::Ready(Err(error)) => fault = Some(this.terminal_error.take().unwrap_or(error)),
                std::task::Poll::Ready(Ok(len)) => {
                    if let Some(error) = this.terminal_error.take() {
                        fault = Some(error);
                    } else if let Some(page_count) = len.checked_add(HISTORY_REPLAY_PAGE_BYTES - 1).map(|bytes| bytes / HISTORY_REPLAY_PAGE_BYTES) {
                        let simultaneous = this.reservation.as_ref().and_then(HistoryReplayReservation::retained_bytes).and_then(|bytes| bytes.checked_add(len));
                        if len < protocol::format::HEADER_SIZE as u64 || page_count > HISTORY_REPLAY_SEGMENT_PAGES || simultaneous.is_none_or(|bytes| bytes > HISTORY_REPLAY_OPERATION_BYTES) {
                            fault = Some(DbError::LimitExceeded("history replay page/source byte credit"));
                        } else {
                            this.pages.len = len;
                            next = Some(HistoryReplayPhase::PageStart { index: *index, len, offset: 0 });
                        }
                    } else {
                        fault = Some(DbError::LimitExceeded("history segment page rounding"));
                    }
                }
            },
            HistoryReplayPhase::PageStart { index, len, offset } => {
                if offset == len {
                    next = Some(HistoryReplayPhase::Frame { index: *index, cursor: HistoryFrameCursor::new(protocol::format::HEADER_SIZE as u64) });
                } else {
                    match len.checked_sub(*offset) {
                        Some(remaining) => {
                            let requested = remaining.min(HISTORY_REPLAY_PAGE_BYTES);
                            let storage = this.storage.clone();
                            let document = this.document.clone();
                            let (index, len, offset) = (*index, *len, *offset);
                            next = Some(HistoryReplayPhase::PageRead { index, len, offset, requested, future: Box::pin(async move { storage.wal().await.read(&document, index, pack::ByteRange { offset, len: requested }).await }) });
                        }
                        None => fault = Some(DbError::LimitExceeded("history page remaining bytes")),
                    }
                }
            }
            HistoryReplayPhase::PageRead { index, len, offset, requested, future } => match future.as_mut().poll(context) {
                std::task::Poll::Pending => {}
                std::task::Poll::Ready(Err(error)) => fault = Some(this.terminal_error.take().unwrap_or(error)),
                std::task::Poll::Ready(Ok(page)) => {
                    if let Some(error) = this.terminal_error.take() {
                        this.terminal_page = Some(page);
                        fault = Some(error);
                    } else if page.len() as u64 != *requested || page.capacity() as u64 > HISTORY_REPLAY_PAGE_BYTES || this.page_count >= HISTORY_REPLAY_SEGMENT_PAGES as usize {
                        this.terminal_page = Some(page);
                        fault = Some(DbError::LimitExceeded("history backend page ownership"));
                    } else if let Some(next_offset) = offset.checked_add(*requested) {
                        this.pages.pages[this.page_count] = Some(page);
                        this.page_count += 1;
                        next = Some(HistoryReplayPhase::PageStart { index: *index, len: *len, offset: next_offset });
                    } else {
                        this.terminal_page = Some(page);
                        fault = Some(DbError::LimitExceeded("history page offset"));
                    }
                }
            },
            HistoryReplayPhase::Frame { index, cursor } => match cursor.step(&this.pages) {
                Err(error) => fault = Some(error),
                Ok(None) => {}
                Ok(Some(HistoryFrameToken::End)) => match index.checked_add(1) {
                    Some(next_index) => next = Some(HistoryReplayPhase::Retire { next_index }),
                    None => fault = Some(DbError::LimitExceeded("history segment index")),
                },
                Ok(Some(HistoryFrameToken::TxBegin)) => next = Some(HistoryReplayPhase::ClearPending { index: *index, next_offset: cursor.pos }),
                Ok(Some(HistoryFrameToken::Command { offset, len })) => match HistoryEnvelopeCursor::new(&this.pages, offset, len) {
                    Ok(envelope) => next = Some(HistoryReplayPhase::Envelope { index: *index, next_offset: cursor.pos, cursor: envelope }),
                    Err(error) => fault = Some(error),
                },
                Ok(Some(HistoryFrameToken::Frontier { offset, len })) => match HistoryFrontierCursor::new(&this.pages, offset, len) {
                    Ok(frontier) => next = Some(HistoryReplayPhase::Frontier { index: *index, next_offset: cursor.pos, cursor: frontier }),
                    Err(error) => fault = Some(error),
                },
                Ok(Some(HistoryFrameToken::Other)) => next = Some(HistoryReplayPhase::Frame { index: *index, cursor: HistoryFrameCursor::new(cursor.pos) }),
            },
            HistoryReplayPhase::Envelope { index, next_offset, cursor } => {
                let step = match this.reservation.as_mut().and_then(|owner| owner.scratch.as_mut()) {
                    Some(scratch) => cursor.step(&this.pages, scratch),
                    None => Err(DbError::Closed),
                };
                match step {
                    Err(error) => fault = Some(error),
                    Ok(Some(range)) => {
                        let len = range.end.saturating_sub(range.start);
                        match this.reservation.as_ref().map_or(Err(DbError::Closed), |owner| owner.preflight_result_range(this.result_len, len)) {
                            Ok(_) => next = Some(HistoryReplayPhase::CopyMutation { index: *index, next_offset: *next_offset, range, copied: 0, result_start: this.result_len }),
                            Err(error) => fault = Some(error),
                        }
                    }
                    Ok(None) => {}
                }
            }
            HistoryReplayPhase::CopyMutation { index, next_offset, range, copied, result_start } => match Self::copy_mutation_fragment(&this.pages, &mut this.reservation, range, *copied, *result_start) {
                Err(error) => fault = Some(error),
                Ok(0) => {
                    if let Some(len) = range.end.checked_sub(range.start).and_then(|value| u16::try_from(value).ok()) {
                        if let Some(reservation) = this.reservation.as_mut() {
                            reservation.operation_ids.push(HistoryTextRange { start: *result_start as u32, len });
                            this.result_len = *result_start + u64::from(len);
                            next = Some(HistoryReplayPhase::Frame { index: *index, cursor: HistoryFrameCursor::new(*next_offset) });
                        } else {
                            fault = Some(DbError::Closed);
                        }
                    } else {
                        fault = Some(DbError::LimitExceeded("history operation id range"));
                    }
                }
                Ok(count) => *copied += count,
            },
            HistoryReplayPhase::Frontier { index, next_offset, cursor } => {
                let step = match this.reservation.as_mut().and_then(|owner| owner.scratch.as_mut()) {
                    Some(scratch) => cursor.step(&this.pages, scratch),
                    None => Err(DbError::Closed),
                };
                match step {
                    Err(error) => fault = Some(error),
                    Ok(Some((head_seq, commit_seq, chain_hash, epoch))) => next = Some(HistoryReplayPhase::Publish { index: *index, next_offset: *next_offset, head_seq, commit_seq, chain_hash, epoch }),
                    Ok(None) => {}
                }
            }
            HistoryReplayPhase::ClearPending { index, next_offset } => {
                let reservation = this.reservation.as_mut();
                if reservation.as_ref().is_some_and(|owner| owner.operation_ids.len() > this.pending_start) {
                    if let Some(range) = reservation.and_then(|owner| owner.operation_ids.pop()) {
                        this.result_len = range.start as u64;
                    }
                } else {
                    next = Some(HistoryReplayPhase::Frame { index: *index, cursor: HistoryFrameCursor::new(*next_offset) });
                }
            }
            HistoryReplayPhase::Publish { index, next_offset, head_seq, commit_seq, chain_hash, epoch } => {
                let operation_end = this.reservation.as_ref().map_or(this.pending_start, |owner| owner.operation_ids.len());
                if operation_end == this.pending_start {
                    next = Some(HistoryReplayPhase::Frame { index: *index, cursor: HistoryFrameCursor::new(*next_offset) });
                } else if this.reservation.as_ref().is_none_or(|owner| owner.entries.len() >= HISTORY_REPLAY_MAX_ENTRIES) {
                    fault = Some(DbError::LimitExceeded("history entry item credit"));
                } else {
                    let count = operation_end - this.pending_start;
                    let start = u16::try_from(this.pending_start);
                    let count = u16::try_from(count);
                    match (start, count, this.reservation.as_mut()) {
                        (Ok(operation_start), Ok(operation_count), Some(reservation)) => {
                            reservation.entries.push(ArtifactHistoryEntry { operation_start, operation_count, head_seq: *head_seq, commit_seq: *commit_seq, chain_hash: *chain_hash, epoch: *epoch });
                            this.pending_start = operation_end;
                            next = Some(HistoryReplayPhase::Frame { index: *index, cursor: HistoryFrameCursor::new(*next_offset) });
                        }
                        _ => fault = Some(DbError::LimitExceeded("history entry range")),
                    }
                }
            }
            HistoryReplayPhase::Retire { next_index } => {
                if this.page_count != 0 {
                    this.page_count -= 1;
                    drop(this.pages.pages[this.page_count].take());
                } else {
                    this.pages.len = 0;
                    next = Some(HistoryReplayPhase::Probe { index: *next_index });
                }
            }
            HistoryReplayPhase::FinalizeSuccess => {
                if this.reservation.as_mut().and_then(|owner| owner.scratch.take()).is_some() {
                } else if !this.pages.pages.is_empty() {
                    drop(std::mem::take(&mut this.pages.pages));
                } else {
                    finalize = true;
                }
            }
        }
        if std::mem::take(&mut this.panic_before_transition_commit) {
            panic!("history replay transition panic fixture");
        }
        if let Some(error) = fault {
            if this.terminal_error.is_none() {
                this.terminal_error = Some(error);
            }
            this.transition = HistoryReplayTransition::FaultRetire;
        } else if finalize {
            match this.finish_view() {
                Ok(view) => {
                    this.phase = None;
                    this.transition = HistoryReplayTransition::Complete;
                    return std::task::Poll::Ready(Ok(view));
                }
                Err(error) => {
                    this.terminal_error.get_or_insert(error);
                    this.transition = HistoryReplayTransition::FaultRetire;
                }
            }
        } else if let Some(next) = next {
            this.phase = Some(next);
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for HistoryReplayFuture {
    type Output = Result<ArtifactHistoryView, DbError>;

    fn poll(self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.next(context)
    }
}

impl Drop for HistoryReplayFuture {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "history replay reached Drop before retained source/result/page retirement");
    }
}
//#endregion 🔖️HistoryReplay

//#region 🔖️Actor
/// @emoji 📨️ A `Send` message crossing `ArtifactAuthority`'s bounded mailbox.
pub enum ArtifactMessage {
    Submit {
        batch: CommandBatch,
        options: SubmitOptions,
        now_ms: u64,
        reply: db_actor::ReplySender<Result<CommandReceipt, DbError>>,
    },
    Query {
        path: String,
        reply: db_actor::ReplySender<Result<Option<db_query::QueryBytes>, DbError>>,
    },
    Frontier {
        reply: db_actor::ReplySender<Frontier>,
    },
    /// @emoji 🔎️ Additive this revision — `db_engine`'s current `ArtifactHandle::query` goes
    /// through `Query { path, .. }` above and never constructs this variant, so adding it is safe.
    RunQuery {
        query: db_query::Query,
        consistency: db_query::Consistency,
        reply: db_actor::ReplySender<Result<db_query::QueryResult, DbError>>,
    },
    SnapshotNow {
        now_ms: u64,
        reply: db_actor::ReplySender<Result<u64, DbError>>,
    },
    DrainOutbox {
        reply: db_actor::ReplySender<Vec<OutboxEntry>>,
    },
    History {
        operation_generation: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        reservation: HistoryReplayReservation,
        reply: db_actor::ReplySender<Result<ArtifactHistoryView, DbError>>,
    },
}

/// @emoji 🎭️ A live handle to one document's authority actor. Each admitted mailbox message wakes
/// one finite `WorkerPool` turn; no job waits for the next message and no authority owns an OS
/// thread. `db_state::PMap` uses `Arc`-shared HAMT nodes, making the engine movable between turns
/// without sharing mutable actor state or weakening its single-consumer mailbox semantics.
#[cfg(not(target_arch = "wasm32"))]
pub struct ArtifactAuthority {
    address: db_actor::Address<ArtifactMessage>,
    cancel: Arc<dyn Fn() + Send + Sync>,
    handoff: Arc<ArtifactRunnerHandoff>,
    done: std::sync::Mutex<Option<db_actor::ReplyReceiver<()>>>,
}

#[cfg(not(target_arch = "wasm32"))]
type ArtifactBuildFuture<A, V> = Pin<Box<dyn Future<Output = Result<ArtifactEngine<A, V>, DbError>> + Send + 'static>>;

#[cfg(not(target_arch = "wasm32"))]
type ArtifactTurnFuture<A, V> = Pin<Box<dyn Future<Output = ArtifactEngine<A, V>> + Send + 'static>>;

#[cfg(not(target_arch = "wasm32"))]
enum ArtifactTurn<A: AuthzHook + 'static, V: VersionGraph + 'static> {
    Future(ArtifactTurnFuture<A, V>),
    History { engine: Option<ArtifactEngine<A, V>>, replay: HistoryReplayFuture, reply: Option<db_actor::ReplySender<Result<ArtifactHistoryView, DbError>>> },
}

#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_RUNNER_RETRY_MS: u64 = 1;
#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_RUNNER_RETRY_LIMIT: u8 = 8;

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerHandoff {
    pool: Arc<semio_framework_async::WorkerPool>,
    terminal_job: std::sync::Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    close_runner: std::sync::Mutex<Option<Arc<dyn Fn() -> bool + Send + Sync>>>,
    active_history: std::sync::atomic::AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct ArtifactRunnerTerminalJob {
    handoff: Arc<ArtifactRunnerHandoff>,
    owner: Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunner<A: AuthzHook + 'static, V: VersionGraph + 'static> {
    pool: Arc<semio_framework_async::WorkerPool>,
    address: db_actor::Address<ArtifactMessage>,
    receiver: db_actor::Receiver<ArtifactMessage>,
    generation: u64,
    builder: std::sync::Mutex<Option<ArtifactBuildFuture<A, V>>>,
    engine: std::sync::Mutex<Option<ArtifactEngine<A, V>>>,
    turn: std::sync::Mutex<Option<ArtifactTurn<A, V>>>,
    ready: std::sync::Mutex<Option<db_actor::ReplySender<Result<(), DbError>>>>,
    done: std::sync::Mutex<Option<db_actor::ReplySender<()>>>,
    handoff: Arc<ArtifactRunnerHandoff>,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    retry_armed: std::sync::atomic::AtomicBool,
    retry_generation: std::sync::atomic::AtomicU64,
    scheduled: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    close_driving: std::sync::atomic::AtomicBool,
    terminal: std::sync::atomic::AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunnerWake<A: AuthzHook + 'static, V: VersionGraph + 'static> {
    runner: std::sync::Weak<ArtifactRunner<A, V>>,
    generation: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl<A: AuthzHook + 'static, V: VersionGraph + 'static> std::task::Wake for ArtifactRunnerWake<A, V> {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(runner) = self.runner.upgrade() {
            if self.generation == runner.generation && !runner.close_driving.load(std::sync::atomic::Ordering::Acquire) {
                runner.schedule();
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<A: AuthzHook + 'static, V: VersionGraph + 'static> ArtifactRunner<A, V> {
    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.terminal.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let runner = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || runner.run_turn(generation)), 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(semio_framework_async::Lane::UserVisible, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated if attempt < ARTIFACT_RUNNER_RETRY_LIMIT => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt + 1));
                    self.arm_retry();
                }
                kind => {
                    let job = error.into_job();
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        drop(job);
                        ready.send(Err(DbError::Unavailable(format!("artifact authority WorkerPool submission failed: {kind:?}"))));
                        self.finish();
                    } else {
                        *self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                    }
                }
            },
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = match self.retry_generation.fetch_update(Ordering::AcqRel, Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
            Ok(previous) => match previous.checked_add(1) {
                Some(generation) => generation,
                None => {
                    self.terminalize_retry_authority("artifact runner retry generation exhausted");
                    return;
                }
            },
            Err(_) => {
                self.terminalize_retry_authority("artifact runner retry generation exhausted");
                return;
            }
        };
        let Some(deadline) = self.pool.now_ms().checked_add(ARTIFACT_RUNNER_RETRY_MS) else {
            self.terminalize_retry_authority("artifact runner retry deadline exhausted");
            return;
        };
        let runner = self.clone();
        self.pool.callback_at(deadline, move || {
            if generation != runner.retry_generation.load(Ordering::Acquire) {
                return;
            }
            runner.retry_armed.store(false, Ordering::Release);
            let retry = runner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if runner.cancelled.load(Ordering::Acquire) {
                    runner.scheduled.store(false, Ordering::Release);
                    *runner.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
                } else {
                    runner.submit_exact(job, attempt);
                }
            }
        });
    }

    fn terminalize_retry_authority(&self, detail: &'static str) {
        self.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            let mut terminal = self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal.is_none() {
                *terminal = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
            } else {
                drop(terminal);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
        }
        if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            ready.send(Err(DbError::Unavailable(detail.to_string())));
            self.finish();
        }
    }

    fn cancel(self: &Arc<Self>) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }

    fn finish(&self) {
        if self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|turn| matches!(turn, ArtifactTurn::History { replay, .. } if !replay.terminal_is_empty())) {
            return;
        }
        if !self.terminal.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let builder = self.builder.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if builder.is_none() {
                let turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                if turn.is_none() {
                    self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                }
            }
            if let Some(done) = self.done.lock().unwrap().take() {
                done.send(());
            }
        }
    }

    fn start_turn(engine: ArtifactEngine<A, V>, message: ArtifactMessage) -> ArtifactTurn<A, V> {
        match message {
            ArtifactMessage::History { operation_generation, cancelled, reservation, reply } => {
                let replay = engine.history_replay(operation_generation, cancelled, reservation);
                ArtifactTurn::History { engine: Some(engine), replay, reply: Some(reply) }
            }
            message => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                match message {
                    ArtifactMessage::Submit { batch, options, now_ms, reply } => reply.send(engine.submit(batch, options, now_ms).await),
                    ArtifactMessage::Query { path, reply } => reply.send(engine.get(&path).await),
                    ArtifactMessage::Frontier { reply } => reply.send(engine.frontier().await),
                    ArtifactMessage::RunQuery { query, consistency, reply } => reply.send(engine.query(query, consistency).await),
                    ArtifactMessage::SnapshotNow { now_ms, reply } => reply.send(engine.snapshot_now(now_ms).await),
                    ArtifactMessage::DrainOutbox { reply } => reply.send(engine.drain_outbox().await),
                    ArtifactMessage::History { reply, .. } => reply.send(Err(DbError::Internal("history turn bypassed retained runner cursor".to_string()))),
                }
                engine
            })),
        }
    }

    fn run_turn(self: Arc<Self>, generation: u64) {
        use std::panic::AssertUnwindSafe;
        use std::sync::atomic::Ordering;

        if generation != self.generation || self.terminal.load(Ordering::Acquire) {
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if self.cancelled.load(Ordering::Acquire) {
            let mut turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(ArtifactTurn::History { replay, .. }) = turn.as_mut() {
                replay.request_close(DbError::Closed);
            } else {
                drop(turn);
                self.finish();
                return;
            }
        }

        let waker = std::task::Waker::from(Arc::new(ArtifactRunnerWake { runner: Arc::downgrade(&self), generation }));
        let mut context = std::task::Context::from_waker(&waker);

        let mut builder = self.builder.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(future) = builder.as_mut() {
            match std::panic::catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context))) {
                Ok(std::task::Poll::Pending) => return,
                Ok(std::task::Poll::Ready(Ok(engine))) => {
                    builder.take();
                    drop(builder);
                    *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        ready.send(Ok(()));
                    }
                    if self.address.has_messages() {
                        self.schedule();
                    }
                    return;
                }
                Ok(std::task::Poll::Ready(Err(error))) => {
                    builder.take();
                    drop(builder);
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        ready.send(Err(error));
                    }
                    self.finish();
                    return;
                }
                Err(_) => {
                    builder.take();
                    drop(builder);
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        ready.send(Err(DbError::Internal("document authority construction panicked".to_string())));
                    }
                    self.finish();
                    return;
                }
            }
        }
        drop(builder);

        let mut turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(active) = turn.as_mut() {
            match active {
                ArtifactTurn::Future(future) => match std::panic::catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context))) {
                    Ok(std::task::Poll::Pending) => return,
                    Ok(std::task::Poll::Ready(engine)) => {
                        turn.take();
                        drop(turn);
                        *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                        if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
                            self.finish();
                        } else if self.address.has_messages() {
                            self.schedule();
                        }
                        return;
                    }
                    Err(_) => {
                        turn.take();
                        drop(turn);
                        self.address.close();
                        self.finish();
                        return;
                    }
                },
                ArtifactTurn::History { engine, replay, reply } => {
                    if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
                        replay.request_close(DbError::Closed);
                    }
                    match std::panic::catch_unwind(AssertUnwindSafe(|| Pin::new(&mut *replay).poll(&mut context))) {
                        Ok(std::task::Poll::Pending) => return,
                        Ok(std::task::Poll::Ready(result)) => {
                            let engine = engine.take();
                            if let Some(reply) = reply.take() {
                                reply.send(result);
                            }
                            let replay_empty = replay.terminal_is_empty();
                            if replay_empty {
                                self.handoff.active_history.store(false, Ordering::Release);
                                turn.take();
                            }
                            drop(turn);
                            if let Some(engine) = engine {
                                *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                            }
                            if !replay_empty {
                                self.schedule();
                            } else if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
                                self.finish();
                            } else if self.address.has_messages() {
                                self.schedule();
                            }
                            return;
                        }
                        Err(_) => {
                            replay.request_close(DbError::Internal("history replay cursor panicked".to_string()));
                            drop(turn);
                            self.address.close();
                            self.schedule();
                            return;
                        }
                    }
                }
            }
        }
        drop(turn);

        if let Some(envelope) = self.receiver.try_recv() {
            let engine = self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some(engine) = engine {
                let turn = Self::start_turn(engine, envelope.payload);
                self.handoff.active_history.store(matches!(&turn, ArtifactTurn::History { .. }), Ordering::Release);
                *self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(turn);
                self.schedule();
            } else {
                self.address.close();
                self.finish();
            }
            return;
        }
        if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
            self.finish();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactAuthority {
    /// @emoji 🚀️ Builds the engine on the injected pool and resolves only after construction, so
    /// a caller never receives an authority whose engine failed to open.
    pub async fn spawn<A: AuthzHook + 'static, V: VersionGraph + 'static, F: Future<Output = Result<ArtifactEngine<A, V>, DbError>> + Send + 'static>(
        pool: Arc<semio_framework_async::WorkerPool>,
        build: impl FnOnce() -> F + Send + 'static,
        capacities: MailboxCapacities,
    ) -> Result<ArtifactAuthority, DbError> {
        let (address, receiver) = db_actor::mailbox::<ArtifactMessage>(capacities);
        let (ready_tx, ready_rx) = db_actor::oneshot::<Result<(), DbError>>();
        let (done_tx, done_rx) = db_actor::oneshot();
        let handoff = Arc::new(ArtifactRunnerHandoff { pool: pool.clone(), terminal_job: std::sync::Mutex::new(None), close_runner: std::sync::Mutex::new(None), active_history: std::sync::atomic::AtomicBool::new(false) });
        let runner = Arc::new(ArtifactRunner {
            pool,
            address: address.clone(),
            receiver,
            generation: address.generation().0,
            builder: std::sync::Mutex::new(Some(Box::pin(build()))),
            engine: std::sync::Mutex::new(None),
            turn: std::sync::Mutex::new(None),
            ready: std::sync::Mutex::new(Some(ready_tx)),
            done: std::sync::Mutex::new(Some(done_tx)),
            handoff: handoff.clone(),
            retry_job: std::sync::Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            close_driving: std::sync::atomic::AtomicBool::new(false),
            terminal: std::sync::atomic::AtomicBool::new(false),
        });
        let weak = Arc::downgrade(&runner);
        address.set_consumer_wake(Arc::new(move || {
            if let Some(runner) = weak.upgrade() {
                runner.schedule();
            }
        }));
        let weak = Arc::downgrade(&runner);
        *handoff.close_runner.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move || -> bool {
            if let Some(runner) = weak.upgrade() {
                runner.cancelled.store(true, std::sync::atomic::Ordering::Release);
                runner.close_driving.store(true, std::sync::atomic::Ordering::Release);
                runner.scheduled.store(true, std::sync::atomic::Ordering::Release);
                let generation = runner.generation;
                let witness = runner.clone();
                runner.run_turn(generation);
                let closed = witness.terminal.load(std::sync::atomic::Ordering::Acquire);
                witness.close_driving.store(false, std::sync::atomic::Ordering::Release);
                closed
            } else {
                true
            }
        }));
        let runner_for_cancel = runner.clone();
        let cancel: Arc<dyn Fn() + Send + Sync> = Arc::new(move || runner_for_cancel.cancel());
        runner.schedule();

        match ready_rx.await {
            Ok(Ok(())) => Ok(ArtifactAuthority { address, cancel, handoff, done: std::sync::Mutex::new(Some(done_rx)) }),
            Ok(Err(err)) => Err(err),
            Err(_) => Err(DbError::Closed),
        }
    }

    /// @emoji 📨️ Nonblocking retained submit cursor used by `db_engine::SubmitFuture`.
    pub fn submit_retained(&self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> db_actor::AskFuture<ArtifactMessage, Result<CommandReceipt, DbError>> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::Submit { batch, options, now_ms, reply })
    }

    pub fn history_retained(&self, operation_generation: u64, cancelled: Arc<std::sync::atomic::AtomicBool>, reservation: HistoryReplayReservation) -> db_actor::AskFuture<ArtifactMessage, Result<ArtifactHistoryView, DbError>> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::History { operation_generation, cancelled, reservation, reply })
    }

    pub fn generation(&self) -> GenerationId {
        self.address.generation()
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactRunnerTerminalJob> {
        self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactRunnerTerminalJob { handoff: self.handoff.clone(), owner: Some(owner) })
    }

    pub fn close_step(&self) -> bool {
        let active = self.handoff.active_history.load(std::sync::atomic::Ordering::Acquire);
        if !active && self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            return false;
        }
        let closed = self.handoff.close_runner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_none_or(|close| close());
        let mut terminal = self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_none() {
            return active;
        }
        if closed {
            let owner = terminal.take();
            drop(owner);
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && !self.handoff.active_history.load(std::sync::atomic::Ordering::Acquire)
    }

    pub async fn submit(&self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> Result<CommandReceipt, DbError> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::Submit { batch, options, now_ms, reply }).await?
    }

    pub async fn query(&self, path: &str) -> Result<Option<db_query::QueryBytes>, DbError> {
        let path = path.to_string();
        self.address.ask(Priority::Query, |reply| ArtifactMessage::Query { path, reply }).await?
    }

    pub async fn frontier(&self) -> Result<Frontier, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::Frontier { reply }).await
    }

    pub async fn run_query(&self, query: db_query::Query, consistency: db_query::Consistency) -> Result<db_query::QueryResult, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::RunQuery { query, consistency, reply }).await?
    }

    pub async fn snapshot_now(&self, now_ms: u64) -> Result<u64, DbError> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::SnapshotNow { now_ms, reply }).await?
    }

    pub async fn drain_outbox(&self) -> Result<Vec<OutboxEntry>, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::DrainOutbox { reply }).await
    }

    /// @emoji 🚪️ Closes the mailbox, cancels any future turn, and awaits the finite in-flight turn.
    pub async fn shutdown(self) {
        self.address.close();
        (self.cancel)();
        let done = self.done.lock().unwrap().take();
        if let Some(done) = done {
            let _ = done.await;
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactRunnerTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact runner job already resolved").0
    }

    pub fn resume(mut self) {
        let owner = self.owner.take().expect("terminal artifact runner job already resolved");
        match self.handoff.pool.try_submit(semio_framework_async::Lane::UserVisible, owner.1) {
            Ok(()) => {}
            Err(error) => {
                *self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.kind(), error.into_job()));
            }
        }
    }

    pub fn close(mut self) {
        let owner = self.owner.take().expect("terminal artifact runner job already resolved");
        let closed = self.handoff.close_runner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_none_or(|close| close());
        if !closed {
            *self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}
//#endregion 🔖️Actor

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc as StdArc;

    fn history_construction_test_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_staging_retirement_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_are_lossless() {
        while artifact_state_retirement_maintenance_step().unwrap() {}
        let accepted_cancel = StdArc::new(std::sync::atomic::AtomicBool::new(false));
        let mut accepted_control = db_state::StateCursorControl::new(accepted_cancel, std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
        let mut accepted = db_state::StateEntry::try_admit("accepted", vec![0x41], 1, &mut accepted_control).await.unwrap();
        assert!(accepted.close_step().unwrap());
        assert!(accepted.close_step().unwrap());
        assert!(accepted.close_step().unwrap());
        assert!(!accepted.close_step().unwrap());
        assert!(accepted.terminal_is_empty());

        let cancelled = StdArc::new(std::sync::atomic::AtomicBool::new(true));
        let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
        let rejected = db_state::StateEntry::try_admit("path", vec![0x42], 1, &mut control).await.unwrap_err();
        assert!(matches!(rejected.error(), DbError::Unavailable(message) if message == "state cursor cancelled"));
        let source = rejected.source().expect("exact refused source").as_ptr();
        let mut second_control = db_state::StateCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(true)), std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
        let second_rejected = db_state::StateEntry::try_admit("second-path", vec![0x44], 1, &mut second_control).await.unwrap_err();
        let second_source = second_rejected.source().expect("second exact refused source").as_ptr();
        let mut cursor = ArtifactStateRetirementCursor::rejected(rejected, std::array::from_fn(|_| None));
        assert!(cursor.close_step().unwrap());
        assert_eq!(cursor.rejected.as_ref().and_then(db_state::StateEntryRejected::source).map(Vec::as_ptr), Some(source));
        {
            let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in retired.iter_mut() {
                *slot = Some(ArtifactStateRetirementCursor::empty());
            }
        }
        ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
        assert!(retire_artifact_state_owner(cursor).is_ok());
        assert!(retire_artifact_state_owner(ArtifactStateRetirementCursor::rejected(second_rejected, std::array::from_fn(|_| None))).is_ok());
        assert!(ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
        {
            let overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert_eq!(overflow.iter().flatten().find_map(|owner| owner.rejected.as_ref()?.source().map(Vec::as_ptr)), Some(source));
            assert!(overflow.iter().flatten().any(|owner| owner.rejected.as_ref().and_then(db_state::StateEntryRejected::source).map(Vec::as_ptr) == Some(second_source)));
        }
        {
            let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in retired.iter_mut() {
                *slot = None;
            }
        }
        assert!(artifact_state_retirement_maintenance_step().unwrap());
        {
            let retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert_eq!(retired.iter().flatten().find_map(|owner| owner.rejected.as_ref()?.source().map(Vec::as_ptr)), Some(source));
        }
        while artifact_state_retirement_maintenance_step().unwrap() {}

        let mut deadline_control = db_state::StateCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now(), 1).unwrap();
        let deadline = db_state::StateEntry::try_admit("deadline", vec![0x43], 1, &mut deadline_control).await.unwrap_err();
        assert!(matches!(deadline.error(), DbError::Unavailable(message) if message == "state cursor deadline reached"));
        assert!(retire_artifact_state_owner(ArtifactStateRetirementCursor::rejected(deadline, std::array::from_fn(|_| None))).is_ok());
        while artifact_state_retirement_maintenance_step().unwrap() {}
    }

    struct HistoryReplayTestWake;

    impl std::task::Wake for HistoryReplayTestWake {
        fn wake(self: StdArc<Self>) {}
    }

    async fn storage() -> StdArc<db_storage::DbBackend> {
        StdArc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()))
    }

    async fn document_id() -> protocol::ArtifactId {
        protocol::ArtifactId("doc-1".to_string())
    }

    async fn stored_json(mut bytes: db_query::QueryBytes) -> serde_json::Value {
        let mut raw = Vec::with_capacity(bytes.len());
        for fragment in bytes.fragments() {
            raw.extend_from_slice(fragment);
        }
        while bytes.close_step().unwrap().is_some() {}
        decode_pathmap_json(&raw).await.expect("stored json value")
    }

    async fn envelope(id: &str, deps: &[&str], actor: &str, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
        let object: Vec<(String, DslValue)> = entries.iter().map(|(path, value)| (path.to_string(), dsl::to_dsl_value(value).expect("test envelope dsl"))).collect();
        protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(id.to_string()),
            document_id: document_id().await,
            actor: protocol::ActorId(actor.to_string()),
            dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(object)).await },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(vec![])).await },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }

    //#region 🔖️Command
    #[semio_framework_async_macros::async_test]
    async fn command_batch_rejects_empty_and_mixed_documents() {
        assert!(CommandBatch::new(Vec::new()).await.is_err());
        let mut mismatched = envelope("op-2", &[], "alice", &[("x", serde_json::json!(1))]).await;
        mismatched.document_id = protocol::ArtifactId("other-doc".to_string());
        let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await, mismatched]).await;
        assert!(batch.is_err());
    }
    //#endregion 🔖️Command

    //#region 🔖️Bridge
    mod bridge {
        use super::*;

        #[derive(Clone, serde::Serialize, serde::Deserialize)]
        struct Counter(i64);

        #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
        struct AddDiff(i64);

        impl protocol::MutationDiff<Counter> for AddDiff {
            fn apply(&self, base: &Counter) -> protocol::MutationApplyResult<Counter> {
                Ok(Counter(base.0 + self.0))
            }
            fn absorb(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        #[derive(Clone, serde::Serialize, serde::Deserialize)]
        struct Add(i64);

        impl protocol::Mutation<Counter> for Add {
            type Diff = AddDiff;
            fn diff(&self, _base: &Counter) -> protocol::MutationOutcome<AddDiff> {
                protocol::MutationOutcome::new(AddDiff(self.0))
            }
            fn inverse(&self, _base: &Counter) -> Vec<Self> {
                vec![Add(-self.0)]
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn envelope_from_operation_uses_operation_and_diff_traits() {
            let base = Counter(10);
            let op = Add(5);
            let envelope = envelope_from_operation(document_id().await, "counter", &op, &base, protocol::ActorId("alice".to_string()), protocol::MutationId("op-add-1".to_string()), protocol::HybridLogicalTimestamp::new(1, 0)).await.unwrap();
            let entries = diff_entries(&envelope.diff).await.unwrap();
            assert_eq!(entries.len(), 1);
            let (path, value) = &entries[0];
            assert_eq!(path, "counter");
            let new_value: Counter = dsl::from_dsl_value(value.clone().unwrap()).unwrap();
            assert_eq!(new_value.0, 15);
        }
    }
    //#endregion 🔖️Bridge

    //#region 🔖️Engine submit + materialize + WAL replay
    #[semio_framework_async_macros::async_test]
    async fn submit_persists_to_wal_and_updates_materialized_state_and_frontier() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

        let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
        let receipt = engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();

        assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
        assert_eq!(receipt.frontier.head_seq, 1);
        assert_eq!(receipt.frontier.commit_seq, 1);
        assert!(receipt.conflicts.is_empty());
        assert!(receipt.state_hash.is_some());

        let stored = engine.get("name").await.unwrap().unwrap();
        let value: serde_json::Value = stored_json(stored).await;
        assert_eq!(value, serde_json::json!("hello"));
        assert_eq!(engine.frontier().await.head_seq, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn open_replays_the_wal_and_reconstructs_state_and_frontier_identically() {
        let storage = storage().await;
        {
            let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
            let batch1 = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
            engine.submit(batch1, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();
            let batch2 = CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("count", serde_json::json!(2))]).await]).await.unwrap();
            engine.submit(batch2, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 2).await.unwrap();
        }

        let (reopened, report) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 3).unwrap();
        assert_eq!(report.torn_tail_bytes, 0);
        assert_eq!(reopened.frontier().await.head_seq, 2);
        assert_eq!(reopened.frontier().await.commit_seq, 2);

        let name: serde_json::Value = stored_json(reopened.get("name").await.unwrap().unwrap()).await;
        assert_eq!(name, serde_json::json!("hello"));
        let count: serde_json::Value = stored_json(reopened.get("count").await.unwrap().unwrap()).await;
        assert_eq!(count, serde_json::json!(2));
    }

    #[semio_framework_async_macros::async_test]
    async fn materialize_from_snapshot_plus_wal_suffix_matches_full_replay() {
        let storage = storage().await;
        {
            let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
            for i in 0..3 {
                let key = format!("path-{i}");
                let value = format!("value-{i}");
                let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))]).await]).await.unwrap();
                engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i).await.unwrap();
            }
            engine.snapshot_now(10).await.unwrap();
            for i in 3..6 {
                let key = format!("path-{i}");
                let value = format!("value-{i}");
                let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))]).await]).await.unwrap();
                engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i).await.unwrap();
            }
        }

        let (reopened, report) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 20).unwrap();
        assert!(report.from_snapshot);
        assert_eq!(report.commands_replayed, 3);
        assert_eq!(reopened.frontier().await.head_seq, 6);
        for i in 0..6 {
            let value: serde_json::Value = stored_json(reopened.get(&format!("path-{i}")).await.unwrap().unwrap()).await;
            assert_eq!(value, serde_json::json!(format!("value-{i}")));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn deletion_via_json_null_tombstones_a_path() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
        assert!(engine.get("x").await.unwrap().is_some());
        engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("x", serde_json::Value::Null)]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
        assert!(engine.get("x").await.unwrap().is_none());
    }
    //#endregion 🔖️Engine submit + materialize + WAL replay

    //#region 🔖️Deps + Dedupe
    #[semio_framework_async_macros::async_test]
    async fn submit_rejects_an_envelope_whose_dependency_was_never_applied() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        let batch = CommandBatch::new(vec![envelope("op-2", &["op-1-never-applied"], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap();
        let result = engine.submit(batch, SubmitOptions::default(), 0);
        assert!(matches!(result.await, Err(DbError::InvalidArgument(_))));
        assert!(engine.get("x").await.unwrap().is_none(), "a rejected batch must not have partially applied");
    }

    #[semio_framework_async_macros::async_test]
    async fn resubmitting_the_same_batch_returns_the_cached_receipt_without_advancing_the_frontier() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        let batch = || db_actor::block_on(CommandBatch::new(vec![db_actor::block_on(envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]))])).unwrap();

        let first = engine.submit(batch(), SubmitOptions::default(), 0).await.unwrap();
        let frontier_after_first = engine.frontier().await;
        let second = engine.submit(batch(), SubmitOptions::default(), 1).await.unwrap();

        assert_eq!(first, second);
        assert_eq!(engine.frontier().await, frontier_after_first, "a deduped resubmit must not move the frontier");
    }
    //#endregion 🔖️Deps + Dedupe

    //#region 🔖️Conflict
    #[semio_framework_async_macros::async_test]
    async fn concurrent_write_to_the_same_path_without_a_dependency_is_recorded_as_a_conflict() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

        // op-2 writes the same path but does NOT declare op-1 as a dependency: a real concurrent
        // write from `op-2`'s author's point of view.
        let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
        assert_eq!(receipt.conflicts.len(), 1);
        assert_eq!(receipt.conflicts[0].conflicting_with, protocol::MutationId("op-1".to_string()));
        assert_eq!(receipt.conflicts[0].path, "x");
        // Last-writer-wins: the conflicting write still applies.
        let x: serde_json::Value = stored_json(engine.get("x").await.unwrap().unwrap()).await;
        assert_eq!(x, serde_json::json!(2));
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_the_prior_writer_as_a_dependency_avoids_the_conflict() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
        let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
        assert!(receipt.conflicts.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_conflicts_uses_real_db_conflict_detector_against_recent_history() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

        let probe = CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap();
        let conflicts = engine.preview_conflicts(&probe).await.unwrap();
        assert!(!conflicts.is_empty(), "db_conflict must detect the same-path intersection against recent history");
    }
    //#endregion 🔖️Conflict

    //#region 🔖️Undo
    #[semio_framework_async_macros::async_test]
    async fn undo_applies_the_recorded_inverse_and_produces_a_fresh_commit() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        let original = protocol::MutationEnvelope {
            mutation_id: protocol::MutationId("op-1".to_string()),
            document_id: document_id().await,
            actor: protocol::ActorId("alice".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&dsl::to_dsl_value(&serde_json::json!({ "x": 1 })).expect("dsl")).await },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&dsl::to_dsl_value(&serde_json::json!({ "x": null })).expect("dsl")).await },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        };
        engine.submit(CommandBatch::new(vec![original]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
        assert!(engine.get("x").await.unwrap().is_some());

        let receipt = engine.undo(&protocol::MutationId("op-1".to_string()), protocol::MutationId("op-1-undo".to_string()), protocol::ActorId("alice".to_string()), 1).await.unwrap();
        assert_eq!(receipt.frontier.head_seq, 2);
        assert!(engine.get("x").await.unwrap().is_none(), "undo must have applied the recorded inverse (delete x)");
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_of_an_unknown_operation_errs_not_found() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        let never_applied = protocol::MutationId("never-applied".to_string());
        let result = engine.undo(&never_applied, protocol::MutationId("undo-1".to_string()), protocol::ActorId("alice".to_string()), 0);
        assert!(matches!(result.await, Err(DbError::NotFound(_))));
    }
    //#endregion 🔖️Undo

    //#region 🔖️Preview
    #[semio_framework_async_macros::async_test]
    async fn preview_is_never_durable_and_a_conflicting_commit_supersedes_it() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

        let preview_id = engine.publish_preview(&[("y".to_string(), Some(serde_json::json!("preview-value")))], 0).await.unwrap();
        assert_eq!(engine.preview_status(&preview_id).await.unwrap(), db_preview::PreviewState::Active);
        let preview_value: serde_json::Value = stored_json(engine.preview_get(&preview_id, "y").await.unwrap().unwrap()).await;
        assert_eq!(preview_value, serde_json::json!("preview-value"));
        assert!(engine.get("y").await.unwrap().is_none(), "a preview must never be visible in committed state");

        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("y", serde_json::json!("committed-value"))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
        assert_eq!(engine.preview_status(&preview_id).await.unwrap(), db_preview::PreviewState::Superseded, "an intersecting real commit must supersede the preview");

        let committed: serde_json::Value = stored_json(engine.get("y").await.unwrap().unwrap()).await;
        assert_eq!(committed, serde_json::json!("committed-value"));
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_withdraw_and_expire_transitions() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

        let withdrawn_id = engine.publish_preview(&[("a".to_string(), Some(serde_json::json!(1)))], 0).await.unwrap();
        engine.withdraw_preview(&withdrawn_id).await.unwrap();
        assert_eq!(engine.preview_status(&withdrawn_id).await.unwrap(), db_preview::PreviewState::Withdrawn);

        let dummy_id = db_preview::PreviewId("does-not-exist".to_string());
        assert!(matches!(engine.preview_status(&dummy_id).await, Err(DbError::NotFound(_))));
    }
    //#endregion 🔖️Preview

    //#region 🔖️Security
    #[semio_framework_async_macros::async_test]
    async fn security_gate_rejects_a_principal_denied_by_its_policy() {
        // An empty `RoleBasedPolicy` (no grants at all) denies every action, per its own doc — a
        // default-deny policy, matching `db_engine`'s own equivalent test of the same gate.
        let security = db_security::SecurityGate::new(db_security::RoleBasedPolicy::new(), db_security::ReplayGuard::new(60_000, 1_024), db_security::BudgetRegistry::new(100_000, 100_000), Arc::new(NullEmit));
        let storage = storage().await;
        let config = ArtifactEngineConfig { security, ..ArtifactEngineConfig::default() };
        let mut engine = ArtifactEngine::create(document_id().await, storage, config, 0).unwrap();
        let result = engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0);
        assert!(matches!(result.await, Err(DbError::Unauthorized(_))));
        assert!(engine.get("x").await.unwrap().is_none());
    }
    //#endregion 🔖️Security

    //#region 🔖️Query
    #[semio_framework_async_macros::async_test]
    async fn query_finds_a_committed_row_by_path() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("greeting", serde_json::json!("hello"))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

        let query = db_query::Query::new().filter(db_query::Predicate::Eq(db_query::Path::empty().push_field("path"), db_query::Value::Text("greeting".to_string())));
        let result = engine.query(query, db_query::Consistency::Canonical).await.unwrap();
        assert_eq!(result.rows.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn live_query_refresh_reports_no_further_diff_right_after_submit_already_refreshed_it() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        let id = engine.subscribe(db_query::LiveQuerySpec { query: db_query::Query::new(), consistency: db_query::Consistency::Canonical }).await;
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
        let diffs = engine.refresh_live_queries().await;
        assert!(diffs.is_empty());
        engine.unsubscribe(id).await;
    }
    //#endregion 🔖️Query

    //#region 🔖️Outbox + CommitLog
    #[semio_framework_async_macros::async_test]
    async fn outbox_and_commit_log_accumulate_and_outbox_drains() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

        assert_eq!(engine.commit_log().await.len(), 1);
        assert_eq!(engine.commit_log().await[0].operation_ids, vec![protocol::MutationId("op-1".to_string())]);

        let drained = engine.drain_outbox().await;
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].mutation_id, protocol::MutationId("op-1".to_string()));
        assert!(engine.drain_outbox().await.is_empty(), "drain must clear the outbox");
    }
    //#endregion 🔖️Outbox + CommitLog

    //#region 🔖️Actor
    #[semio_framework_async_macros::async_test]
    async fn document_authority_submits_and_queries_over_finite_pool_turns() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
        let storage = storage().await;
        let document = document_id().await;
        let authority = ArtifactAuthority::spawn(pool.clone(), move || ArtifactEngine::create_retained(document, storage, ArtifactEngineConfig::default(), 0), MailboxCapacities::uniform(16)).await.unwrap();

        let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hi"))]).await]).await.unwrap();
        let receipt = authority.submit(batch, SubmitOptions::default(), 0).await.unwrap();
        assert_eq!(receipt.frontier.head_seq, 1);

        let queried: serde_json::Value = stored_json(authority.query("name").await.unwrap().unwrap()).await;
        assert_eq!(queried, serde_json::json!("hi"));

        let frontier = authority.frontier().await.unwrap();
        assert_eq!(frontier.head_seq, 1);

        let generation = authority.snapshot_now(1).await.unwrap();
        assert_eq!(generation, 0);

        authority.shutdown().await;
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn document_authority_spawn_propagates_a_build_failure_synchronously() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
        let result = ArtifactAuthority::spawn(pool.clone(), || async { Err::<ArtifactEngine<AllowAll, NullVersionGraph>, DbError>(DbError::InvalidArgument("boom".to_string())) }, MailboxCapacities::uniform(4));
        assert!(matches!(result.await, Err(DbError::InvalidArgument(_))));
        pool.shutdown();
    }

    #[test]
    fn artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each() {
        let mut reservation = HistoryReplayReservation::try_new().unwrap();
        for _ in 0..HISTORY_REPLAY_SEGMENT_PAGES {
            reservation.retain_source_page(vec![0]).unwrap();
        }
        let mut cursor = HistoryReplayReservationCloseCursor::new(reservation);
        for remaining in (0..HISTORY_REPLAY_SEGMENT_PAGES as usize).rev() {
            assert!(cursor.close_step());
            assert_eq!(cursor.source_page_count, remaining);
        }
        assert!(cursor.close_step(), "fault retirement continues with one result page/range/scalar owner");
        for _ in 0..HISTORY_REPLAY_RESULT_PAGES + 8 {
            if cursor.terminal_is_empty() {
                break;
            }
            assert!(cursor.close_step());
        }
        assert!(cursor.terminal_is_empty());
        let replay = include_str!("🦀️component.rs");
        for fault in ["history backend page ownership", "history frame CRC mismatch", "history envelope has trailing bytes"] {
            assert!(replay.contains(fault), "missing retained fault source {fault}");
        }
        assert!(replay.contains("HistoryReplayTransition::FaultRetire"));
    }

    #[test]
    fn artifact_history_scratch_result_boundary_plus_one_preserves_exact_owner() {
        let reservation = HistoryReplayReservation::try_new().unwrap();
        let first_result_owner = reservation.result_pages[0].as_ref().unwrap().as_ptr();
        assert_eq!(reservation.source_pages.len(), HISTORY_REPLAY_SEGMENT_PAGES as usize);
        assert_eq!(reservation.result_pages.len(), HISTORY_REPLAY_RESULT_PAGES);
        assert_eq!(reservation.operation_ids.capacity(), HISTORY_REPLAY_MAX_OPERATION_IDS);
        assert_eq!(reservation.entries.capacity(), HISTORY_REPLAY_MAX_ENTRIES);
        assert_eq!(reservation.scratch.as_ref().unwrap().len(), HISTORY_REPLAY_MAX_FIELD_BYTES);
        assert_eq!(reservation.result_pages[0].as_ref().unwrap().as_ptr(), first_result_owner);
        assert_eq!(reservation.preflight_result_range(HISTORY_REPLAY_RESULT_BYTES - 1, 1).unwrap(), HISTORY_REPLAY_RESULT_BYTES);
        assert!(matches!(reservation.preflight_result_range(HISTORY_REPLAY_RESULT_BYTES, 1), Err(DbError::LimitExceeded("history result byte credit"))));
        assert_eq!(reservation.result_pages[0].as_ref().unwrap().as_ptr(), first_result_owner, "cap+1 rejection must return the exact preadmitted page owner");
        let mut cursor = HistoryReplayReservationCloseCursor::new(reservation);
        for _ in 0..HISTORY_REPLAY_RESULT_PAGES + 8 {
            if cursor.terminal_is_empty() {
                break;
            }
            assert!(cursor.close_step());
        }
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn artifact_history_reservation_construction_fault_cap_plus_one_and_each_page_retire_one_owner() {
        let _guard = history_construction_test_lock();
        for failure_after in [0, 1, HISTORY_REPLAY_RESULT_PAGES / 2, HISTORY_REPLAY_RESULT_PAGES - 1, HISTORY_REPLAY_RESULT_PAGES] {
            let mut fault = HistoryReplayReservation::try_new_with_result_page_failure(failure_after).unwrap_err();
            let error = fault.take_error().expect("exact construction error");
            assert!(matches!(error, DbError::Unavailable(_)));
            let retained_pages = fault.retained_result_page_count();
            assert_eq!(retained_pages, failure_after, "failure must retain every page allocated before its exact boundary");
            let mut previous_pages = retained_pages;
            while !fault.terminal_is_empty() {
                assert!(fault.close_step());
                let current_pages = fault.retained_result_page_count();
                assert!(previous_pages.saturating_sub(current_pages) <= 1, "one construction-fault grant may retire at most one result page");
                previous_pages = current_pages;
            }
        }

        let reservation = HistoryReplayReservation::try_new_with_result_page_failure(HISTORY_REPLAY_RESULT_PAGES + 1).expect("a failure boundary beyond the fixed page cap cannot fabricate an extra owner");
        assert_eq!(reservation.result_pages.len(), HISTORY_REPLAY_RESULT_PAGES);
        let mut cursor = HistoryReplayReservationCloseCursor::new(reservation);
        while !cursor.terminal_is_empty() {
            assert!(cursor.close_step());
        }

        for page_count in 0..=HISTORY_REPLAY_RESULT_PAGES {
            let result_pages = (0..page_count).map(|_| Some(Vec::new())).collect();
            let token = claim_history_replay_reservation_construction().expect("fixed construction slot");
            let close = HistoryReplayReservationCloseCursor {
                source_pages: Some(Vec::new()),
                source_page_count: 0,
                result_pages: Some(result_pages),
                operation_ids: Some(Vec::new()),
                entries: Some(Vec::new()),
                scratch: None,
                retained_operation_bytes: 0,
                retained_result_bytes: 0,
                started: true,
            };
            {
                let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let slot = &mut registry.slots[token.slot];
                slot.cursor = Some(close);
            }
            let mut fault = HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None };
            let mut retired_pages = 0;
            while fault.retained_result_page_count() != 0 {
                let before = fault.retained_result_page_count();
                assert!(fault.close_step());
                let after = fault.retained_result_page_count();
                assert_eq!(before - after, 1, "failure after page {page_count} must retire exactly one allocated page per grant");
                retired_pages += 1;
            }
            assert_eq!(retired_pages, page_count);
            while !fault.terminal_is_empty() {
                assert!(fault.close_step());
            }
        }
    }

    #[test]
    fn artifact_history_unchecked_construction_error_and_checked_out_drop_hand_back_exact_pages() {
        let _guard = history_construction_test_lock();
        let generation = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).next_generation;
        let dropped = std::panic::catch_unwind(|| {
            let _ = HistoryReplayReservation::try_new_with_result_page_failure(3);
        });
        assert!(dropped.is_ok(), "unchecked construction error Drop must not panic");
        let fault = take_history_replay_reservation_construction_fault(generation).expect("unchecked error registered exact partial owner");
        assert_eq!(fault.retained_result_page_count(), 3);
        drop(fault);
        let mut resumed = take_history_replay_reservation_construction_fault(generation).expect("checked-out Drop returned exact partial owner");
        assert!(matches!(resumed.take_error(), Some(DbError::Unavailable(_))));
        let mut previous_pages = 3;
        while !resumed.terminal_is_empty() {
            assert!(resumed.close_step());
            let pages = resumed.retained_result_page_count();
            assert!(previous_pages.saturating_sub(pages) <= 1);
            previous_pages = pages;
        }
        assert!(take_history_replay_reservation_construction_fault(generation).is_none());
    }

    #[test]
    fn artifact_history_construction_unwind_hands_partial_owner_to_registry_without_bulk_drop() {
        let _guard = history_construction_test_lock();
        let generation = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).next_generation;
        let unwind = std::panic::catch_unwind(|| {
            let mut builder = HistoryReplayReservationConstructionBuilder::new().expect("fixed construction authority");
            let mut page = Vec::new();
            page.try_reserve_exact(HISTORY_REPLAY_PAGE_BYTES as usize).expect("fixture page");
            page.resize(HISTORY_REPLAY_PAGE_BYTES as usize, 0);
            assert!(builder
                .edit_cursor(|cursor| cursor.result_pages.as_mut().is_some_and(|owners| {
                    owners.push(Some(page));
                    true
                }))
                .unwrap_or(false));
            panic!("injected construction unwind");
        });
        assert!(unwind.is_err());
        let mut fault = take_history_replay_reservation_construction_fault(generation).expect("builder Drop registered partial owner");
        assert_eq!(fault.retained_result_page_count(), 1);
        assert!(matches!(fault.take_error(), Some(DbError::Unavailable(_))));
        let before = fault.retained_result_page_count();
        assert!(fault.close_step());
        let after = fault.retained_result_page_count();
        assert_eq!(before - after, 1);
        while !fault.terminal_is_empty() {
            assert!(fault.close_step());
        }
    }

    #[test]
    fn artifact_history_construction_registry_saturation_rejects_before_partial_owner_and_reuses_with_fresh_generation() {
        let _guard = history_construction_test_lock();
        let mut faults = Vec::with_capacity(HISTORY_REPLAY_CONSTRUCTION_SLOTS);
        for _ in 0..HISTORY_REPLAY_CONSTRUCTION_SLOTS {
            faults.push(HistoryReplayReservation::try_new_with_result_page_failure(0).unwrap_err());
        }
        let mut rejected = HistoryReplayReservation::try_new_with_result_page_failure(0).unwrap_err();
        assert!(rejected.generation().is_none());
        assert!(matches!(rejected.take_error(), Some(DbError::Unavailable(_))));
        let released_generation = faults[0].generation().expect("registered generation");
        let mut released = faults.remove(0);
        while !released.terminal_is_empty() {
            assert!(released.close_step());
        }
        let replacement = HistoryReplayReservation::try_new_with_result_page_failure(0).unwrap_err();
        assert_ne!(replacement.generation(), Some(released_generation));
        faults.push(replacement);
        for mut fault in faults {
            while !fault.terminal_is_empty() {
                assert!(fault.close_step());
            }
        }
    }

    #[test]
    fn artifact_history_construction_handback_rejects_stale_duplicate_and_aba_without_owner_overwrite() {
        let _guard = history_construction_test_lock();
        let mut first = HistoryReplayReservation::try_new_with_result_page_failure(1).unwrap_err();
        let first_token = first.token.as_ref().map(|token| (token.slot, token.generation)).expect("first linear construction token");
        while !first.terminal_is_empty() {
            assert!(first.close_step());
        }

        let replacement = HistoryReplayReservation::try_new_with_result_page_failure(1).unwrap_err();
        let replacement_token = replacement.token.as_ref().map(|token| (token.slot, token.generation)).expect("replacement linear construction token");
        assert_eq!(replacement_token.0, first_token.0);
        assert_ne!(replacement_token.1, first_token.1);
        let page_pointer = replacement.retained_result_page_pointer(0).expect("replacement page owner");
        let error_pointer = replacement.retained_error_pointer().expect("replacement error owner");

        let stale = HistoryReplayReservationConstructionToken { slot: first_token.0, generation: first_token.1 };
        assert_eq!(handback_history_replay_reservation_construction(&stale), Err(HistoryReplayReservationConstructionHandbackRejection { slot: first_token.0, generation: first_token.1 }));
        assert_eq!(replacement.retained_result_page_pointer(0), Some(page_pointer));
        assert_eq!(replacement.retained_error_pointer(), Some(error_pointer));

        let out_of_bounds = HistoryReplayReservationConstructionToken { slot: HISTORY_REPLAY_CONSTRUCTION_SLOTS, generation: replacement_token.1 };
        assert_eq!(handback_history_replay_reservation_construction(&out_of_bounds), Err(HistoryReplayReservationConstructionHandbackRejection { slot: HISTORY_REPLAY_CONSTRUCTION_SLOTS, generation: replacement_token.1 }));
        assert_eq!(replacement.retained_result_page_pointer(0), Some(page_pointer));
        assert_eq!(replacement.retained_error_pointer(), Some(error_pointer));

        drop(replacement);
        let duplicate = HistoryReplayReservationConstructionToken { slot: replacement_token.0, generation: replacement_token.1 };
        assert_eq!(handback_history_replay_reservation_construction(&duplicate), Err(HistoryReplayReservationConstructionHandbackRejection { slot: replacement_token.0, generation: replacement_token.1 }));
        let mut resumed = take_history_replay_reservation_construction_fault(replacement_token.1).expect("current generation remained resumable");
        assert_eq!(resumed.retained_result_page_pointer(0), Some(page_pointer));
        assert_eq!(resumed.retained_error_pointer(), Some(error_pointer));
        while !resumed.terminal_is_empty() {
            assert!(resumed.close_step());
        }
    }

    #[test]
    fn artifact_history_fixed_owner_accounting_has_no_capacity_scan() {
        let source = include_str!("🦀️component.rs");
        let replay = &source[source.find("//#region 🔖️HistoryReplay").unwrap()..source.find("//#endregion 🔖️HistoryReplay").unwrap()];
        for forbidden in [".rposition(", "result_pages.iter()", "source_pages.iter().all", "pages.iter().all", "pages.iter().filter"] {
            assert!(!replay.contains(forbidden), "retained history accounting scanned fixed capacity through {forbidden}");
        }
        for retained in ["source_page_count", "retained_operation_bytes", "retained_result_bytes"] {
            assert!(replay.contains(retained), "retained history accounting omitted {retained}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_history_panic_at_each_phase_transition_retains_then_fault_retires() {
        let engine = ArtifactEngine::create_retained(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).await.unwrap();
        let phases = Vec::from([
            HistoryReplayPhase::Probe { index: 0 },
            HistoryReplayPhase::SegmentLen { index: 0, future: Box::pin(async { Err(DbError::NotFound("phase fixture".to_string())) }) },
            HistoryReplayPhase::PageStart { index: 0, len: 1, offset: 0 },
            HistoryReplayPhase::PageRead { index: 0, len: 1, offset: 0, requested: 1, future: Box::pin(async { Ok(vec![0]) }) },
            HistoryReplayPhase::Frame { index: 0, cursor: HistoryFrameCursor::new(0) },
            HistoryReplayPhase::Envelope { index: 0, next_offset: 0, cursor: HistoryEnvelopeCursor { pos: 0, end: 0, field: HistoryEnvelopeField::MutationId, dependencies: 0, mutation_id: None } },
            HistoryReplayPhase::CopyMutation { index: 0, next_offset: 0, range: 0..0, copied: 0, result_start: 0 },
            HistoryReplayPhase::Frontier { index: 0, next_offset: 0, cursor: HistoryFrontierCursor { pos: 0, end: 0, field: HistoryFrontierField::Document, head_seq: 0, commit_seq: 0, chain_hash: [0; 32] } },
            HistoryReplayPhase::ClearPending { index: 0, next_offset: 0 },
            HistoryReplayPhase::Publish { index: 0, next_offset: 0, head_seq: 0, commit_seq: 0, chain_hash: [0; 32], epoch: 0 },
            HistoryReplayPhase::Retire { next_index: 1 },
            HistoryReplayPhase::FinalizeSuccess,
        ]);
        let waker = std::task::Waker::from(StdArc::new(HistoryReplayTestWake));
        let mut context = std::task::Context::from_waker(&waker);
        for phase in phases {
            let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
            replay.phase = Some(phase);
            replay.transition = HistoryReplayTransition::InProgress;
            replay.panic_before_transition_commit = true;
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Pin::new(&mut replay).poll(&mut context))).is_err());
            assert!(replay.phase.is_some(), "panic must retain the exact active phase owner");
            assert!(matches!(replay.transition, HistoryReplayTransition::InProgress));
            replay.request_close(DbError::Internal("phase panic fixture".to_string()));
            let mut terminal = false;
            for _ in 0..50_000 {
                if Pin::new(&mut replay).poll(&mut context).is_ready() {
                    terminal = true;
                    break;
                }
            }
            assert!(terminal);
            assert!(replay.terminal_is_empty());
        }
    }
    //#endregion 🔖️Actor
}
//#endregion 🧪️Tests
