//! 🧬️ Db artifact commands, diffs, conflict records and durable-group receipts.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as store;

use db_artifact_history::{ArtifactHistoryView, HistoryReplayReservation};
use db_durability::{DurabilityClass, Frontier};
use db_ids::{*, ActorId, ArtifactId, DbError};
use db_io_core::WalStorage;
use pack::ContentHash;
use protocol::MutationDiff as _;
use semio_framework_value::DslValue;
use std::{collections::VecDeque, pin::Pin, sync::Arc};

//#region 🔖️Ids
/// 🌉️ `protocol::ArtifactId` → `ArtifactId`, the lossless single-`String` bridge
/// `db_core`'s module doc promises (see `ArtifactId`'s own doc for the rationale: this
/// crate is the first one in the family that depends on both `db_core` and `protocol` and so is
/// where the bridge is actually exercised).
pub async fn to_core_document_id(id: &protocol::ArtifactId) -> ArtifactId {
    ArtifactId(id.0.clone())
}

/// 🌉️ `protocol::ActorId` → `ActorId`, same bridge as `to_core_document_id`.
pub async fn to_core_actor_id(id: &protocol::ActorId) -> ActorId {
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
/// 📦️ One atomically-submitted group of causally-related operations against a single
/// document — the unit `ArtifactEngine::submit` accepts. Every envelope must target the same
/// `document_id` (checked at construction, and again against the engine's own document at submit
/// time); the batch's `command_id` (for dedupe/the returned `CommandReceipt`) is its LAST
/// envelope's `mutation_id`, since `MutationEnvelope` has no separate batch-level id of its own.
pub struct CommandBatch {
    pub envelopes: Vec<protocol::MutationEnvelope>,
}

impl CommandBatch {
    /// 🏗️ Builds a batch, rejecting an empty one or one whose envelopes disagree on
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

/// 🎚️ Per-submit durability override — see `DurabilityClass`'s doc for the
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
/// 🧬️ The schema tag `db_artifact` reserves for its own generic path-value diff
/// convention (see module doc) — the only `ArtifactDiff`/`InverseMutation` shape this crate
/// knows how to interpret. 🎯️ W5: `diff`/`inverse` payloads are opaque `Vec<u8>` on the wire now;
/// `db_artifact` still only understands ITS OWN JSON-object-of-paths convention, tagged with this
/// schema so `diff_entries`/`inverse_entries` can distinguish "our own pathmap bytes" from a
/// typed op's binary payload it has no business decoding (foreign schema -> empty `TouchedSet`,
/// not an error — the envelope is still persisted/relayed, just not interpreted at this layer;
/// that's `db_artifact`-and-above's future typed path).
pub const DB_PATHMAP_SCHEMA: &str = "db.pathmap.v1";

/// 🎯️ `DslValue` object pathmap -> `store::pack_rt::encode_wire_value` bytes.
pub async fn encode_pathmap(value: &DslValue) -> Vec<u8> {
    store::pack_rt::encode_wire_value(value)
}

/// 🎯️ Inverse of `encode_pathmap`.
async fn decode_pathmap(bytes: &[u8]) -> Result<DslValue, DbError> {
    store::pack_rt::decode_wire_value(bytes).map_err(wire_err)
}

/// 🧰️ Public convenience for every crate above this one that hand-builds a `DB_PATHMAP_SCHEMA`
/// `MutationEnvelope` (test fixtures, `db_cli`'s `profile`/`migrate` commands, `db_fault_testing`'s
/// workload generators) rather than going through `envelope_from_operation`: encodes a
/// `serde_json::Value::Object` the same way `decode_pathmap`/`apply_one` decode it. Centralizing
/// this is the single source of truth for `DB_PATHMAP_SCHEMA`'s actual wire bytes — a caller that
/// instead hand-rolls `serde_json::to_vec` produces bytes `decode_pathmap` cannot read (it expects
/// `store::pack_rt`'s binary encoding, not raw JSON text), which is exactly the "wire error:
/// truncated" bug this function exists to make structurally impossible to repeat.
pub async fn encode_pathmap_json(value: &serde_json::Value) -> Result<Vec<u8>, DbError> {
    Ok(encode_pathmap(&semio_framework_value::DslValue::from(value)).await)
}

/// 🧰️ Inverse of `encode_pathmap_json` — also the general "read back one stored/queried
/// value's bytes as JSON" decode every caller above this crate needs: `ArtifactEngine::get`/
/// `preview_get` and `db_engine::ArtifactHandle::query` all hand back these same `store::pack_rt`
/// wire bytes (single value OR whole pathmap object, both are just a `DslValue` tree to this
/// codec), never raw JSON text — a caller reaching for `serde_json::from_slice` on them directly
/// hits exactly the "expected value" parse error this function exists to make impossible.
pub async fn decode_pathmap_json(bytes: &[u8]) -> Result<serde_json::Value, DbError> {
    Ok(serde_json::Value::from(decode_pathmap(bytes).await?))
}

/// 🧮️ Flattens a diff/inverse pathmap object into `(path, Some(value) | None)` pairs per this
/// module's generic path-value convention (see module doc). Errors if `value` is not an object —
/// this crate's own schema-erased documents have no other shape it can interpret.
async fn entries_from_value(value: &DslValue) -> Result<Vec<(String, Option<DslValue>)>, DbError> {
    let object = value.as_object().ok_or_else(|| DbError::InvalidArgument("diff/inverse payload must be a pathmap object".to_string()))?;
    Ok(object.iter().map(|(path, entry)| (path.clone(), if entry.is_null() { None } else { Some(entry.clone()) })).collect())
}

/// ➡️ Entries for an envelope's forward diff — empty (not an error) for any schema other
/// than `DB_PATHMAP_SCHEMA`, see its doc.
pub async fn diff_entries(diff: &protocol::ArtifactDiff) -> Result<Vec<(String, Option<DslValue>)>, DbError> {
    if diff.schema.0 != DB_PATHMAP_SCHEMA {
        return Ok(Vec::new());
    }
    entries_from_value(&decode_pathmap(&diff.payload).await?).await
}

/// ↩️ Entries for an envelope's inverse diff (the `undo` pipeline's source) — same
/// foreign-schema handling as `diff_entries`.
pub async fn inverse_entries(inverse: &protocol::InverseMutation) -> Result<Vec<(String, Option<DslValue>)>, DbError> {
    if inverse.schema.0 != DB_PATHMAP_SCHEMA {
        return Ok(Vec::new());
    }
    entries_from_value(&decode_pathmap(&inverse.payload).await?).await
}

/// 🧮️ The inverse of `entries_from_value` — rebuilds a JSON object from path-value pairs,
/// `None` becoming an explicit `null` tombstone. Used by `undo` to construct a compensating
/// envelope's diff/inverse payloads.
// 🚫️async: E1 pure accessor, always used as `&entries_to_value(...)` inline into another call — see R9
pub fn entries_to_value(entries: &[(String, Option<DslValue>)]) -> DslValue {
    semio_framework_value::DslValue::Object(entries.iter().map(|(path, value)| (path.clone(), value.clone().unwrap_or(semio_framework_value::DslValue::Null))).collect())
}

/// 👣️ The `TouchedSet` a set of entries would write — shared by `DocumentState::
/// apply_entries` and preview publishing.
// 🚫️async: E1 pure accessor, `db_state::TouchedSet`'s own methods are sync — see R9
pub fn entries_touched(entries: &[(String, Option<DslValue>)]) -> db_state::TouchedSet {
    let mut touched = db_state::TouchedSet::new();
    for (path, _) in entries {
        touched.record(db_state::TouchedRegion::write(path.clone()));
    }
    touched
}
//#endregion 🔖️Diff
//#region 🔖️Bridge
/// 🌉️ The generic ingestion boundary: builds an `MutationEnvelope` (in this crate's own
/// path-value diff convention) from a typed `protocol::Mutation<P>` against a serializable
/// projection `P`. The operation's diff is applied once through [`protocol::apply_diff`]; the envelope then persists only what
/// that diff changed — one `path/field` entry per top-level field of `P` whose value differs (a removed field is a tombstone),
/// the inverse carrying the base value of exactly those fields — never the whole post- or base projection. A projection that is
/// not a record is one entity: its single entry sits at `path`. Genuinely exercises `Mutation`/`MutationDiff`'s trait methods
/// (`diff`/`mutation_id`/`dependencies`/`author_id`/`timestamp`) — see the module doc's design-choice note on why this crate is
/// allowed to.
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
    P: semio_framework_value::ToValue,
    Op: protocol::Mutation<P>,
{
    let diff = op.diff(base);
    let post = protocol::apply_diff(diff.diff(), base).map_err(|error| DbError::InvalidArgument(error.to_string()))?;
    let (forward, backward) = changed_entries(path, &semio_framework_value::ToValue::to_value(base), &semio_framework_value::ToValue::to_value(&post));
    let schema = protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string());
    Ok(protocol::MutationEnvelope {
        mutation_id: op.mutation_id().unwrap_or(default_mutation_id),
        document_id: document,
        actor: op.author_id().unwrap_or(default_actor),
        dependencies: op.dependencies(),
        observed: None,
        target: op.conflict_target(),
        diff: protocol::ArtifactDiff { schema: schema.clone(), payload: encode_pathmap(&forward).await },
        inverse: protocol::InverseMutation { schema, payload: encode_pathmap(&backward).await },
        timestamp: op.timestamp().unwrap_or(default_timestamp),
        transaction: None, verb: None, line: None,
    })
}

/// 🔺️ The pathmap entries a base→post change writes, as `(forward, inverse)`: per differing top-level field of two record
/// values `path/field` (absent after = `Null` tombstone, absent before = `Null` in the inverse), else the one entity at `path`.
fn changed_entries(path: &str, base: &DslValue, post: &DslValue) -> (DslValue, DslValue) {
    let (DslValue::Object(before), DslValue::Object(after)) = (base, post) else {
        return (DslValue::Object(vec![(path.to_string(), post.clone())]), DslValue::Object(vec![(path.to_string(), base.clone())]));
    };
    let mut forward = Vec::new();
    let mut backward = Vec::new();
    for (key, value) in after {
        let prior = before.iter().find(|(name, _)| name == key).map(|(_, prior)| prior);
        if prior != Some(value) {
            forward.push((format!("{path}/{key}"), value.clone()));
            backward.push((format!("{path}/{key}"), prior.cloned().unwrap_or(DslValue::Null)));
        }
    }
    for (key, prior) in before {
        if !after.iter().any(|(name, _)| name == key) {
            forward.push((format!("{path}/{key}"), DslValue::Null));
            backward.push((format!("{path}/{key}"), prior.clone()));
        }
    }
    (DslValue::Object(forward), DslValue::Object(backward))
}
//#endregion 🔖️Bridge
//#region 🔖️Conflict
/// ⚔️ One detected write/write intersection between the currently-executing operation and
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

/// ⚔️ Builds the `db_conflict::CommandTouch` `envelope` would produce, without applying it —
/// shared by `submit`'s outcome-step gate and `preview_conflicts`'s advisory `db_conflict::
/// ConflictDetector` use. No per-operation conflict-declaration tag anymore (C10 deleted the CRDT-era
/// vocabulary): a `CommandTouch` carries only what `db_conflict::ConflictDetector` actually needs to detect —
/// identity, kind, timestamp, and the regions it touched.
// 🚫️async: E1 pure accessor consumed by a sync Iterator::map closure — see R9
pub fn command_touch(envelope: &protocol::MutationEnvelope, touched: &db_state::TouchedSet) -> db_conflict::CommandTouch {
    let kind = if supersedes_inputs(envelope) { SUPERSEDE_TOUCH_KIND } else { envelope.diff.schema.0.as_str() };
    let touch = db_conflict::CommandTouch::new(envelope.mutation_id.clone(), envelope.actor.clone(), db_conflict::CommandKind::from(kind), envelope.timestamp);
    touched.regions.iter().fold(touch, |touch, region| touch.touch(region.clone()))
}

/// ✏️ Whether `envelope` is a `Supersede` — the one history transition that writes document fields: it replaces the
/// effective input of committed operations, so it is graded by its declared `target` like any other write
/// (📋️ `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §2, §9.1).
fn supersedes_inputs(envelope: &protocol::MutationEnvelope) -> bool {
    matches!(protocol::history_transition_from_envelope(envelope), Ok(Some(protocol::HistoryTransition::Supersede(_))))
}

/// ⚖️ Grades one `db_conflict::ConflictRecord` into the `protocol::MutationMessage` the
/// outcome step judges against `options.policy` (contract §C9: "region intersection = `Warning`;
/// constraint violation = `Fatal`") — `db_conflict` deliberately never grades its own findings (see
/// its module doc), so `db_artifact`, the first crate below it that actually decides what to DO
/// about a conflict, is where that grading belongs. `TouchedRegion` still lands (last-writer-wins,
/// see `🔖️Conflict`'s doc) so it reads as an adjusted-but-applied write (`mutation.clamped`); a
/// violated `Constraint` reads as a broken structural invariant (`mutation.invariant`) — the two
/// `Warning`/`Fatal` codes from the frozen nine that fit each shape.
async fn grade_conflict_record(record: &db_conflict::ConflictRecord) -> protocol::MutationMessage {
    match &record.kind {
        db_conflict::ConflictKind::TouchedRegion(regions) => {
            let target: Vec<String> = regions.iter().map(|region| region.path.clone()).collect();
            protocol::MutationMessage::warning("mutation.clamped", format!("command {} touches region(s) also touched by concurrent command {}", record.command_id.0, record.conflicting_with.0)).at(target)
        }
        db_conflict::ConflictKind::Constraint(description) => {
            protocol::MutationMessage::fatal("mutation.invariant", format!("command {} violates constraint '{description}' held by concurrent command {}", record.command_id.0, record.conflicting_with.0)).at([description.clone()])
        }
    }
}

/// 🏷️ `CommandTouch` kind of a durable group decision's marker in the recent commit window: it names
/// the decision's edit so a later write can say it observed it, and it is never itself graded.
const DURABLE_GROUP_TOUCH_KIND: &str = "db.durable-group";

/// 🏷️ `CommandTouch` kind of a committed `Supersede` ([`supersedes_inputs`]): unlike every other history transition it
/// writes the fields its declared target names, so later writes are graded against it and it against them.
const SUPERSEDE_TOUCH_KIND: &str = "db.supersede";

/// 🪪️ The code of the refusal the event log answers a `Revert`/`Reinstate` naming an operation another actor authored
/// (`ArtifactEngine::submit`); a client localizes the refusal by this code, never by the message prose.
pub const FOREIGN_HISTORY_TRANSITION_CODE: &str = "history.foreign-transition";

/// 🗣️ The message of that refusal (developer detail; the UI text is the localized label of its code).
pub const FOREIGN_HISTORY_TRANSITION_MESSAGE: &str = "an undo or redo may only name operations its own author wrote";

/// 🧩️ The code of the refusal a history transition gets when its payload does not decode, or when a `Supersede` does
/// not declare exactly its targets as its dependencies: every replica would refuse it, so the log never holds it.
pub const MALFORMED_HISTORY_TRANSITION_CODE: &str = "history.malformed-transition";

/// 🗣️ The message of that refusal.
pub const MALFORMED_HISTORY_TRANSITION_MESSAGE: &str = "a history transition must decode and a supersession must depend on exactly its targets";

/// 👻️ The code of the refusal a `Supersede` gets when it names an operation neither committed, nor earlier in its batch,
/// nor folded by a durable group decision — the event log's twin of the fold's unknown-target error.
pub const UNKNOWN_SUPERSEDE_TARGET_CODE: &str = "history.unknown-target";

/// 🗣️ The message of that refusal.
pub const UNKNOWN_SUPERSEDE_TARGET_MESSAGE: &str = "a supersession may only name operations the document's log holds";

/// 🧷️ The recent-window marker of one committed durable group decision (see [`DURABLE_GROUP_TOUCH_KIND`]).
pub fn durable_group_touch(edit_id: &str) -> db_conflict::CommandTouch {
    db_conflict::CommandTouch::new(protocol::MutationId(edit_id.to_string()), protocol::ActorId(String::new()), db_conflict::CommandKind::from(DURABLE_GROUP_TOUCH_KIND), protocol::HybridLogicalTimestamp::new(0, 0))
}

/// 🪟️ Appends one committed touch to the bounded recent commit window, oldest out first.
pub fn remember_recent_touch(window: &mut VecDeque<db_conflict::CommandTouch>, touch: db_conflict::CommandTouch) {
    if window.len() >= MAX_RECENT_TOUCHES {
        window.pop_front();
    }
    window.push_back(touch);
}

/// 👁️ Whether `observed` names `touch`: its command itself, or — for a durable group decision's marker —
/// one of the operations its edit folded (`<edit>#<position>`), which reach a replica inside that edit.
fn touch_is_named(touch: &db_conflict::CommandTouch, observed: &str) -> bool {
    observed == touch.command_id.0 || (touch.kind.0 == DURABLE_GROUP_TOUCH_KIND && observed.strip_prefix(touch.command_id.0.as_str()).is_some_and(|position| position.starts_with('#')))
}

/// ✍️ Whether `touch` wrote document fields: every operation and every `Supersede` did; the other history
/// transitions (undo, redo, checkpoints) and durable group markers never did.
fn touch_writes_fields(touch: &db_conflict::CommandTouch) -> bool {
    touch.kind.0 != DURABLE_GROUP_TOUCH_KIND && touch.kind.0 != protocol::HISTORY_TRANSITION_SCHEMA
}

/// 🕰️ The writes `envelope` (whose own touch is `written`) was authored without seeing: in commit order (the recent
/// commit window, then the batch's earlier envelopes), every write after the one it names as `observed` — all of them
/// when it observed nothing in the window — by another actor. A replica stamps each operation it authors with the
/// newest foreign operation it had applied, and replicas receive commits in commit order, so everything up to that one
/// was seen; the author's own writes are its own history. Only a field write is graded: a `Supersede` is, the other
/// history transitions never are.
pub fn unseen_concurrent_writes<'a>(recent: &'a VecDeque<db_conflict::CommandTouch>, batch: &'a [db_conflict::CommandTouch], written: &db_conflict::CommandTouch, envelope: &protocol::MutationEnvelope) -> Vec<&'a db_conflict::CommandTouch> {
    if !touch_writes_fields(written) {
        return Vec::new();
    }
    let window: Vec<&db_conflict::CommandTouch> = recent.iter().chain(batch.iter()).collect();
    let seen = envelope.observed.as_ref().and_then(|observed| window.iter().rposition(|touch| touch_is_named(touch, &observed.0))).map_or(0, |index| index + 1);
    window.into_iter().skip(seen).filter(|touch| touch.actor != envelope.actor && touch_writes_fields(touch)).collect()
}

/// 🎯️ Whether two declared targets (outermost segment first) can address the same part: an empty target is
/// the whole artifact; otherwise they overlap when they share a segment — conservative for path-shaped targets
/// (`[collection, id]`) and exact for id-set targets.
fn targets_overlap(a: &[String], b: &[String]) -> bool {
    a.is_empty() || b.is_empty() || a.iter().any(|segment| b.contains(segment))
}

/// ⚖️ Grades `written` — authored without seeing the concurrent write `unseen` — into `mutation.clamped`
/// (contract §C9: region intersection = `Warning`): fields both touch when the database can read both diffs, else
/// their declared targets overlapping. Disjoint parts are no conflict. `Normal` accepts a clamped write with its
/// message, `Vigilant` refuses it (ticket 26/09/23 C10: a vigilant hub accepted a same-field write authored
/// during a 12 s cut, because no plugin diff is readable here and no client said what it had seen).
pub async fn grade_concurrent_write(written: &db_conflict::CommandTouch, written_target: &[String], unseen: &db_conflict::CommandTouch, unseen_target: &[String]) -> Vec<protocol::MutationMessage> {
    if !written.touched.regions.is_empty() && !unseen.touched.regions.is_empty() {
        let mut messages = Vec::new();
        for record in db_conflict::ConflictDetector::new().detect(&[unseen.clone(), written.clone()]) {
            messages.push(grade_conflict_record(&record).await);
        }
        return messages;
    }
    if !targets_overlap(written_target, unseen_target) {
        return Vec::new();
    }
    vec![protocol::MutationMessage::warning("mutation.clamped", format!("command {} was authored without seeing concurrent command {}, which writes the same part of the document", written.command_id.0, unseen.command_id.0)).at(written_target.to_vec())]
}
//#endregion 🔖️Conflict
//#region 🔖️Receipt
/// 🧾️ What `ArtifactEngine::submit` returns: the committed batch's identity, the document's
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
    /// 📨️ Every `protocol::MutationMessage` the outcome step graded this batch's
    /// `db_conflict::ConflictRecord`s into (contract §C9) — present even on an accepted-but-degraded
    /// commit (`options.policy` let a `Warning`-or-below worst level through), empty on a clean one.
    pub messages: Vec<protocol::MutationMessage>,
}

/// 📍️ One actor-serialized checkpoint-publication observation, including the exact
/// committed tip identity omitted from the generic database frontier.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckpointPublicationSnapshot {
    pub frontier: Frontier,
    pub head_edit_id: Option<protocol::MutationId>,
}

/// 🧾️ Actor-owned outcome for one Store-admitted durable fixed-three decision event.
pub enum ArtifactDurableGroupJournalAppendV1 {
    Absent,
    Rejected(DbError),
    Committed(store::durable_group::DurableOwnedGroupJournalReceiptV1),
}

/// 🔏️ One fixed-three decision proven to occupy exactly one committed WAL transaction body.
pub struct ArtifactCommittedDurableGroupDecisionV1 {
    pub document: ArtifactId,
    pub transaction_id: u64,
    pub segment_index: u64,
    pub record: store::durable_group::DurableOwnedGroupJournalRecordV1,
}

/// 📍️ Read-only actor projection recovered from one sole committed fixed-three Event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactDurableGroupRecoveredCheckpointV1 {
    pub receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
    pub head_edit_id: protocol::MutationId,
    pub chain_hash: [u8; 32],
    pub record: store::durable_group::DurableOwnedGroupJournalRecordV1,
    already_applied: bool,
}

impl ArtifactDurableGroupRecoveredCheckpointV1 {
    pub fn receipt(&self) -> &store::durable_group::DurableOwnedGroupJournalReceiptV1 {
        &self.receipt
    }

    pub fn head_edit_id(&self) -> &protocol::MutationId {
        &self.head_edit_id
    }

    pub fn chain_hash(&self) -> [u8; 32] {
        self.chain_hash
    }

    pub fn already_applied(&self) -> bool {
        self.already_applied
    }

    /// 🔎️ Projects immutable typed edits only after revalidating the DB-consumed committed record.
    pub fn verify_fixed_three_edits<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
    ) -> Result<store::durable_group::DurableOwnedGroupVerifiedThreeEditsV1<ParentMutation, DrawingMutation, ValueMutation>, store::durable_group::DurableOwnedGroupDecisionError>
    where
        ParentP: pack::snapshot::ArtifactPack,
        ParentMutation: semio_framework_value::ToValue + semio_framework_value::FromValue,
        DrawingP: pack::snapshot::ArtifactPack,
        DrawingMutation: semio_framework_value::ToValue + semio_framework_value::FromValue,
        ValueP: pack::snapshot::ArtifactPack,
        ValueMutation: semio_framework_value::ToValue + semio_framework_value::FromValue,
    {
        self.record.verify_fixed_three_edits::<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>()
    }
}

impl ArtifactCommittedDurableGroupDecisionV1 {
    pub fn document(&self) -> &ArtifactId {
        &self.document
    }

    pub fn transaction_id(&self) -> u64 {
        self.transaction_id
    }

    pub fn segment_index(&self) -> u64 {
        self.segment_index
    }

    pub fn receipt(&self) -> store::durable_group::DurableOwnedGroupJournalReceiptV1 {
        store::durable_group::DurableOwnedGroupJournalReceiptV1 {
            anchor_sha256: self.record.anchor_sha256().to_string(),
            decision_sha256: self.record.decision_sha256().to_string(),
            transaction_id: self.transaction_id,
            segment_index: self.segment_index,
        }
    }

    fn recovered_checkpoint(&self) -> ArtifactDurableGroupRecoveredCheckpointV1 {
        ArtifactDurableGroupRecoveredCheckpointV1 {
            receipt: self.receipt(),
            head_edit_id: protocol::MutationId(self.record.parent_edit_id().to_string()),
            chain_hash: self.record.parent_post_revision(),
            record: self.record.clone(),
            already_applied: false,
        }
    }

    pub fn into_store_owned_recovery<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        self,
        admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        ArtifactCommittedDurableGroupRecoveryV1 { state: std::mem::ManuallyDrop::new(Some(ArtifactCommittedDurableGroupRecoveryStateV1::Admitting { witness: self, admission, last_error: None })) }
    }
}

enum ArtifactCommittedDurableGroupRecoveryStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    Admitting {
        witness: ArtifactCommittedDurableGroupDecisionV1,
        admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
        last_error: Option<store::durable_group::DurableOwnedGroupDecisionError>,
    },
    Recovering {
        document: ArtifactId,
        receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
        host: store::durable_group::DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    },
    Complete {
        document: ArtifactId,
        receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
        already_applied: bool,
        owners: store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    },
}

/// 🔁️ One non-cancellable exact-three-Store restoration driven only by a consumed committed WAL witness.
#[must_use = "committed recovery must reach one terminal three-Store owner handoff"]
pub(crate) struct ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    pub state: std::mem::ManuallyDrop<Option<ArtifactCommittedDurableGroupRecoveryStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactCommittedDurableGroupRecoveryAdvanceV1 {
    Progress(store::durable_group::DurableOwnedThreeStoreCommitPhaseV1),
    Blocked,
    Fault(store::durable_group::DurableOwnedGroupDecisionError),
    Complete,
}

pub(crate) struct ArtifactCommittedDurableGroupRecoveryTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    pub document: ArtifactId,
    pub receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
    pub(crate) already_applied: bool,
    pub owners: store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
}

pub(crate) struct ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    pub document: ArtifactId,
    pub receipt: store::durable_group::DurableOwnedGroupJournalReceiptV1,
    pub error: store::durable_group::DurableOwnedGroupDecisionError,
    pub owners: store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
{
    pub fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> ArtifactCommittedDurableGroupRecoveryAdvanceV1 {
        let Some(state) = self.state.take() else {
            return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidOutcome);
        };
        match state {
            ArtifactCommittedDurableGroupRecoveryStateV1::Admitting { witness, admission, .. } => {
                let document = witness.document.clone();
                let receipt = witness.receipt();
                match admission.restore_untrusted_committed_record(&witness.record, receipt.clone()) {
                    Ok(store::durable_group::DurableOwnedMapRecoveryStartV1::Apply(host)) => {
                        let Some(phase) = host.phase() else {
                            *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                            return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidOutcome);
                        };
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                        ArtifactCommittedDurableGroupRecoveryAdvanceV1::Progress(phase)
                    }
                    Ok(store::durable_group::DurableOwnedMapRecoveryStartV1::AlreadyApplied(owners)) => {
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Complete { document, receipt, already_applied: true, owners });
                        ArtifactCommittedDurableGroupRecoveryAdvanceV1::Complete
                    }
                    Err(rejected) => {
                        let error = rejected.error.clone();
                        let admission = store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(rejected.parent, rejected.drawing, rejected.value);
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Admitting { witness, admission, last_error: Some(error.clone()) });
                        ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(error)
                    }
                }
            }
            ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, mut host } => match host.advance(grant) {
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::Progress(phase) => {
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Progress(phase)
                }
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::Blocked => {
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Blocked
                }
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::Fault(error) => {
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(error)
                }
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::AwaitingAcknowledgement => {
                    if !host.acknowledge_restoration(&receipt) {
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                        return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidHash);
                    }
                    let Some(phase) = host.phase() else {
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                        return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidOutcome);
                    };
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Progress(phase)
                }
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::Complete => {
                    let Some(owners) = host.take_terminal_owners() else {
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                        return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidOutcome);
                    };
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Complete { document, receipt, already_applied: false, owners });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Complete
                }
            },
            complete @ ArtifactCommittedDurableGroupRecoveryStateV1::Complete { .. } => {
                *self.state = Some(complete);
                ArtifactCommittedDurableGroupRecoveryAdvanceV1::Complete
            }
        }
    }

    pub(crate) fn take_terminal(&mut self) -> Option<ArtifactCommittedDurableGroupRecoveryTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        let state = self.state.take()?;
        match state {
            ArtifactCommittedDurableGroupRecoveryStateV1::Complete { document, receipt, already_applied, owners } => Some(ArtifactCommittedDurableGroupRecoveryTerminalV1 { document, receipt, already_applied, owners }),
            state => {
                *self.state = Some(state);
                None
            }
        }
    }

    pub fn take_rejected_terminal(&mut self) -> Option<ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        let state = self.state.take()?;
        match state {
            ArtifactCommittedDurableGroupRecoveryStateV1::Admitting { witness, admission, last_error: Some(error) } => {
                let document = witness.document.clone();
                let receipt = witness.receipt();
                drop(witness);
                Some(ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1 { document, receipt, error, owners: admission.into_owners() })
            }
            state => {
                *self.state = Some(state);
                None
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    fn drop(&mut self) {
        assert!(self.state.is_none(), "committed WAL recovery reached Drop before exact three-Store terminal handoff");
    }
}

pub async fn committed_durable_group_decision_from_transaction<S: WalStorage>(document: &ArtifactId, mut transaction: db_wal_log::WalCommittedTransaction<'_, '_, S>) -> Result<Option<ArtifactCommittedDurableGroupDecisionV1>, DbError> {
    let transaction_id = transaction.transaction_id();
    let segment_index = transaction.segment_index();
    let record_count = transaction.record_count();
    let mut event = None;
    let mut mixed = false;
    loop {
        match transaction.next_record_step()? {
            db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Event(bytes)) => {
                if event.is_some() {
                    mixed = true;
                } else {
                    let mut canonical_pack = Vec::with_capacity(bytes.len());
                    for fragment in bytes.fragments() {
                        canonical_pack.extend_from_slice(fragment);
                    }
                    event = Some(canonical_pack);
                }
            }
            db_wal_log::WalCommittedRecordStep::Record(_) => mixed = event.is_some() || record_count > 1,
            db_wal_log::WalCommittedRecordStep::Yield => {
                transaction.replenish(std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
                semio_framework_async::yield_once().await;
                continue;
            }
            db_wal_log::WalCommittedRecordStep::Done => break,
        }
        while transaction.close_record_step()? {
            transaction.replenish(std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
            semio_framework_async::yield_once().await;
        }
    }
    transaction.finish()?;
    let Some(canonical_pack) = event else { return Ok(None) };
    if mixed || record_count != 1 {
        return Err(DbError::Corrupt("durable group decision must be the sole committed transaction body".to_string()));
    }
    let record = store::durable_group::DurableOwnedGroupJournalRecordV1::admit_canonical(canonical_pack).map_err(|error| DbError::Corrupt(format!("committed durable group decision is invalid: {error}")))?;
    if record.document().artifact_id != document.0 {
        return Err(DbError::Corrupt("committed durable group decision targets a different replay document".to_string()));
    }
    Ok(Some(ArtifactCommittedDurableGroupDecisionV1 { document: document.clone(), transaction_id, segment_index, record }))
}

enum ArtifactDurableGroupRecoveryTargetStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    Admitting(store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
    Recovering { owner: ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, checkpoint: ArtifactDurableGroupRecoveredCheckpointV1 },
    Complete(store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
    Empty,
}

struct ArtifactDurableGroupRecoveryTargetSharedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    pub document: ArtifactId,
    pub state: std::sync::Mutex<ArtifactDurableGroupRecoveryTargetStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>,
    pub scan_complete: std::sync::atomic::AtomicBool,
    recovered_checkpoint: std::sync::Mutex<Option<ArtifactDurableGroupRecoveredCheckpointV1>>,
}

pub trait ArtifactDurableGroupRecoveryDriverCoreV1: Send {
    fn document(&self) -> &ArtifactId;
    fn drive_recovery_step(&mut self) -> Result<bool, DbError>;
    fn accept(&mut self, witness: ArtifactCommittedDurableGroupDecisionV1) -> Result<(), DbError>;
    fn finish_scan(&mut self) -> Result<(), DbError>;
}

/// 🧭 Opaque actor-mailbox owner for one exact-three Store recovery scan.
pub struct ArtifactDurableGroupRecoveryDriverV1 {
    pub owner: Box<dyn ArtifactDurableGroupRecoveryDriverCoreV1>,
}

struct ArtifactDurableGroupRecoveryTargetDriverV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    pub shared: Arc<ArtifactDurableGroupRecoveryTargetSharedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>,
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> ArtifactDurableGroupRecoveryDriverCoreV1 for ArtifactDurableGroupRecoveryTargetDriverV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
{
    fn document(&self) -> &ArtifactId {
        &self.shared.document
    }

    fn drive_recovery_step(&mut self) -> Result<bool, DbError> {
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        let ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: mut recovery, checkpoint } = current else {
            *state = current;
            return Ok(true);
        };
        match recovery.advance(store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
            ArtifactCommittedDurableGroupRecoveryAdvanceV1::Progress(_) | ArtifactCommittedDurableGroupRecoveryAdvanceV1::Blocked => {
                *state = ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: recovery, checkpoint };
                Ok(false)
            }
            ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(error) => {
                *state = ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: recovery, checkpoint };
                Err(DbError::Unavailable(format!("committed durable group recovery retained a fault: {error:?}")))
            }
            ArtifactCommittedDurableGroupRecoveryAdvanceV1::Complete => {
                let Some(terminal) = recovery.take_terminal() else {
                    *state = ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: recovery, checkpoint };
                    return Err(DbError::Internal("committed durable group recovery completed without exact Store owners".to_string()));
                };
                drop(recovery);
                if terminal.document != self.shared.document {
                    *state = ArtifactDurableGroupRecoveryTargetStateV1::Admitting(store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(terminal.owners.parent, terminal.owners.drawing, terminal.owners.value));
                    return Err(DbError::Corrupt("committed durable group recovery returned a different document".to_string()));
                }
                if terminal.receipt != *checkpoint.receipt() {
                    *state = ArtifactDurableGroupRecoveryTargetStateV1::Admitting(store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(terminal.owners.parent, terminal.owners.drawing, terminal.owners.value));
                    return Err(DbError::Corrupt("committed durable group recovery returned a different receipt".to_string()));
                }
                let mut checkpoint = checkpoint;
                checkpoint.already_applied = terminal.already_applied;
                *self.shared.recovered_checkpoint.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(checkpoint);
                *state = ArtifactDurableGroupRecoveryTargetStateV1::Admitting(store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(terminal.owners.parent, terminal.owners.drawing, terminal.owners.value));
                Ok(true)
            }
        }
    }

    fn accept(&mut self, witness: ArtifactCommittedDurableGroupDecisionV1) -> Result<(), DbError> {
        if witness.document() != &self.shared.document {
            return Err(DbError::Corrupt("committed durable group recovery witness targets a different document".to_string()));
        }
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        let ArtifactDurableGroupRecoveryTargetStateV1::Admitting(admission) = current else {
            *state = current;
            return Err(DbError::Conflict("committed durable group recovery target is not ready for another witness".to_string()));
        };
        let checkpoint = witness.recovered_checkpoint();
        *state = ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: witness.into_store_owned_recovery(admission), checkpoint };
        Ok(())
    }

    fn finish_scan(&mut self) -> Result<(), DbError> {
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        match current {
            ArtifactDurableGroupRecoveryTargetStateV1::Admitting(admission) => {
                *state = ArtifactDurableGroupRecoveryTargetStateV1::Complete(admission.into_owners());
                self.shared.scan_complete.store(true, std::sync::atomic::Ordering::Release);
                Ok(())
            }
            current @ ArtifactDurableGroupRecoveryTargetStateV1::Complete(_) => {
                *state = current;
                Ok(())
            }
            current => {
                *state = current;
                Err(DbError::Conflict("committed durable group recovery scan ended with an active witness".to_string()))
            }
        }
    }
}

/// 🧰 Retains one actor scan and the exact three Stores across caller cancellation and retry.
#[must_use = "durable group recovery must return its exact three Store owners"]
pub struct ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    pub shared: Arc<ArtifactDurableGroupRecoveryTargetSharedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>,
    pub request: Option<Pin<Box<db_actor::AskFuture<ArtifactMessage, Result<(), DbError>>>>>,
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
{
    pub fn new(document: ArtifactId, admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>) -> (Self, ArtifactDurableGroupRecoveryDriverV1) {
        let shared = Arc::new(ArtifactDurableGroupRecoveryTargetSharedV1 {
            document,
            state: std::sync::Mutex::new(ArtifactDurableGroupRecoveryTargetStateV1::Admitting(admission)),
            scan_complete: std::sync::atomic::AtomicBool::new(false),
            recovered_checkpoint: std::sync::Mutex::new(None),
        });
        let driver = ArtifactDurableGroupRecoveryDriverV1 { owner: Box::new(ArtifactDurableGroupRecoveryTargetDriverV1 { shared: shared.clone() }) };
        (Self { shared, request: None }, driver)
    }

    pub fn driver(&self) -> ArtifactDurableGroupRecoveryDriverV1 {
        ArtifactDurableGroupRecoveryDriverV1 { owner: Box::new(ArtifactDurableGroupRecoveryTargetDriverV1 { shared: self.shared.clone() }) }
    }

    pub async fn advance(&mut self) -> Result<bool, DbError> {
        let Some(request) = self.request.as_mut() else {
            return Ok(self.shared.scan_complete.load(std::sync::atomic::Ordering::Acquire));
        };
        let result = request.as_mut().await;
        self.request = None;
        result??;
        Ok(self.shared.scan_complete.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn take_terminal_owners(&mut self) -> Option<store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if self.request.is_some() || !self.shared.scan_complete.load(std::sync::atomic::Ordering::Acquire) {
            return None;
        }
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        match current {
            ArtifactDurableGroupRecoveryTargetStateV1::Complete(owners) => Some(owners),
            current => {
                *state = current;
                None
            }
        }
    }

    /// 🧹 Returns exact Stores only when a failed scan has no accepted committed witness in
    /// flight. Once a witness is accepted, recovery is non-cancellable and must be resumed.
    pub fn take_idle_owners(&mut self) -> Option<store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if self.request.is_some() {
            return None;
        }
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        match current {
            ArtifactDurableGroupRecoveryTargetStateV1::Admitting(admission) => Some(admission.into_owners()),
            current => {
                *state = current;
                None
            }
        }
    }

    pub fn scan_is_active(&self) -> bool {
        self.request.is_some()
    }

    /// 📍️ Returns the last fully restored committed Event projection without exposing its WAL witness.
    pub fn recovered_checkpoint(&self) -> Option<ArtifactDurableGroupRecoveredCheckpointV1> {
        self.shared.recovered_checkpoint.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(&*self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner), ArtifactDurableGroupRecoveryTargetStateV1::Empty)
    }
}
//#endregion 🔖️Receipt
//#region 🔖️Engine
pub const MAX_RECENT_TOUCHES: usize = 256;
//#endregion 🔖️Engine
//#region 🔖️Actor
/// 📨️ A `Send` message crossing `ArtifactAuthority`'s bounded mailbox.
pub enum ArtifactMessage {
    AppendDurableGroupDecision {
        record: store::durable_group::DurableOwnedGroupJournalRecordV1,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        now_ms: u64,
        reply: db_actor::ReplySender<Result<ArtifactDurableGroupJournalAppendV1, DbError>>,
    },
    RecoverDurableGroupDecisions {
        driver: ArtifactDurableGroupRecoveryDriverV1,
        reply: db_actor::ReplySender<Result<(), DbError>>,
    },
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
    CheckpointPublicationSnapshot {
        reply: db_actor::ReplySender<CheckpointPublicationSnapshot>,
    },
    /// 🔎️ Additive this revision — `db_engine`'s current `ArtifactHandle::query` goes
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
    History {
        operation_generation: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        reservation: HistoryReplayReservation,
        reply: db_actor::ReplySender<Result<ArtifactHistoryView, DbError>>,
    },
    Compact {
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        budget: db_compact::CompactionBudget,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        reply: db_actor::ReplySender<Result<db_compact::CompactionReport, DbError>>,
    },
}
//#endregion 🔖️Actor
//#endregion 🔖️Ids

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
