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
//! `ArtifactAuthority::run_query_blocking(query, consistency)` (two arguments, `db_query::
//! Consistency`-aware). This revision matches that shape exactly: `security`/`emit` are real
//! `ArtifactEngineConfig` fields, `version_graph` is required (`NullVersionGraph` is the
//! "no vcs" default rather than `Option::None`), `submit`'s conflict step is a genuine
//! `db_conflict::ConflictDetector` fed by retained recent-commit `TouchedSet` history (not a local
//! last-writer stand-in), and `query`/`RunQuery`/`run_query_blocking` take a `db_query::
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
/// @emoji 🏗️ A document's materialized state: a flat `db_state::PMap` from path to raw value
/// bytes, plus a per-path last-writer map for `submit`'s local, path-granular conflict detection
/// (see `🔖️Conflict`'s doc on why this stays local rather than `db_conflict`-backed). `values` uses
/// `PMap` (not a mutable `HashMap`) specifically so `content_hash` — the `Frontier.chain_hash`
/// source — is a real content-addressed digest of the whole state, not an incidental byte count,
/// and so `PMap::iter` gives `snapshot_now`/`query` a cheap, complete enumeration.
struct DocumentState {
    values: db_state::PMap<String, Vec<u8>>,
    last_writer: db_state::PMap<String, protocol::MutationId>,
}

impl DocumentState {
    // 🚫️async: E1 pure constructor, `db_state::PMap::new` is sync — see R9
    fn new() -> DocumentState {
        DocumentState { values: db_state::PMap::new(), last_writer: db_state::PMap::new() }
    }

    async fn get(&self, path: &str) -> Option<Vec<u8>> {
        self.values.get(&path.to_string()).cloned()
    }

    async fn content_hash(&self) -> ContentHash {
        self.values.content_hash()
    }

    /// @emoji ✍️ Applies one envelope's flattened path-value entries, returning the new state, the
    /// `TouchedSet` it wrote, and any conflicts (a path whose last writer is neither `mutation_id`
    /// itself nor a declared `dependencies` member).
    async fn apply_entries(&self, mutation_id: &protocol::MutationId, dependencies: &[protocol::MutationId], entries: &[(String, Option<DslValue>)]) -> Result<(DocumentState, db_state::TouchedSet, Vec<ConflictRecord>), DbError> {
        let mut values = self.values.clone();
        let mut last_writer = self.last_writer.clone();
        let mut touched = db_state::TouchedSet::new();
        let mut conflicts = Vec::new();
        for (path, value) in entries {
            if let Some(previous_writer) = self.last_writer.get(path) {
                if previous_writer != mutation_id && !dependencies.contains(previous_writer) {
                    conflicts.push(ConflictRecord { command_id: mutation_id.clone(), conflicting_with: previous_writer.clone(), path: path.clone() });
                }
            }
            match value {
                Some(dsl_value) => {
                    let bytes = store::pack_rt::encode_wire_value(dsl_value);
                    values = values.insert(path.clone(), bytes);
                }
                None => values = values.remove(path),
            }
            touched.record(db_state::TouchedRegion::write(path.clone()));
            last_writer = last_writer.insert(path.clone(), mutation_id.clone());
        }
        Ok((DocumentState { values, last_writer }, touched, conflicts))
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
    /// @emoji 🌱️ Creates a brand-new document: a genesis WAL (segment 0) and an empty state.
    /// Errors `AlreadyExists` if `document` already has WAL segments in `storage`.
    // 🚫️async: E5 executor bridge — an authority pool turn is a synchronous job, so the
    // engine drives its async storage calls to completion inside that finite turn.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig<A, V>, now_ms: u64) -> Result<ArtifactEngine<A, V>, DbError> {
        let core_id = db_actor::block_on(to_core_document_id(&document));
        let wal = db_actor::block_on(async { db_wal::ArtifactWal::create(&storage.wal().await, core_id.clone(), db_wal::GroupCommitPolicy::default(), now_ms).await })?;
        Ok(db_actor::block_on(ArtifactEngine::assemble(document, core_id, storage, wal, None, config)))
    }

    /// @emoji 🚑️ Materializes a document as initial ⊕ latest `db_snapshot` generation ⊕ WAL suffix
    /// (this revision adds the snapshot half — the prior revision was WAL-suffix-only): loads the
    /// latest snapshot's `DocumentState` (if any) as the starting point, opens/recovers the WAL
    /// (per `db_wal::ArtifactWal::open`), then replays only the `WAL_COMMAND` records committed
    /// AFTER the snapshot's own `head_seq` (a full-from-genesis replay when there is no snapshot
    /// yet).
    // 🚫️async: E5 executor bridge — see `create`'s doc; same finite-turn bridge.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open(document: protocol::ArtifactId, storage: &Arc<db_storage::DbBackend>, config: ArtifactEngineConfig<A, V>, now_ms: u64) -> Result<(ArtifactEngine<A, V>, MaterializeReport), DbError> {
        let core_id = db_actor::block_on(to_core_document_id(&document));
        let mut report = MaterializeReport::default();

        let mut state = DocumentState::new();
        let mut applied_head_seq = 0u64;
        let mut vcs_head = None;
        let snapshot_facet = db_actor::block_on(storage.snapshot());
        let snapshot_manager = db_actor::block_on(db_snapshot::SnapshotManager::new(&snapshot_facet));
        if let Some((generation, descriptor)) = db_actor::block_on(snapshot_manager.load_latest(&core_id))? {
            report.from_snapshot = true;
            report.snapshot_generation = Some(generation);
            let combined = db_actor::block_on(snapshot_manager.materialize_chain(&core_id, generation))?;
            let handle = db_actor::block_on(db_snapshot::open_latest(&combined))?;
            for hash in &descriptor.roots {
                let page_bytes = db_actor::block_on(db_snapshot::read_page(&combined, &handle, *hash))?;
                for (path, value) in db_actor::block_on(decode_state_page(&page_bytes))? {
                    state.values = match value {
                        Some(bytes) => state.values.insert(path, bytes),
                        None => state.values.remove(&path),
                    };
                }
            }
            applied_head_seq = descriptor.head_seq;
            vcs_head = descriptor.vcs_head;
        }
        drop(snapshot_manager);
        drop(snapshot_facet);

        let (wal, wal_recovery) = db_actor::block_on(async { db_wal::ArtifactWal::open(&storage.wal().await, core_id.clone(), db_wal::GroupCommitPolicy::default(), now_ms).await })?;
        report.torn_tail_bytes = wal_recovery.torn_tail_bytes;
        let mut engine = db_actor::block_on(ArtifactEngine::assemble(document, core_id.clone(), storage.clone(), wal, vcs_head, config));
        engine.state = state;
        engine.frontier.head_seq = applied_head_seq;

        let records = db_actor::block_on(async { db_wal::replay_document(&storage.wal().await, &core_id).await })?;
        let mut batch_ids: HashSet<String> = HashSet::new();
        let mut seen: u64 = 0;
        for record in records {
            match record {
                db_wal::WalRecord::TxBegin { .. } => batch_ids.clear(),
                db_wal::WalRecord::Command(bytes) => {
                    let mut pos = 0usize;
                    let envelope = protocol::decode_envelope(&bytes, &mut pos).map_err(|err| DbError::Corrupt(format!("wal command record is not a valid operation envelope: {err}")))?;
                    seen += 1;
                    batch_ids.insert(envelope.mutation_id.0.clone());
                    if seen <= applied_head_seq {
                        // Already folded into the loaded snapshot — replay the causal bookkeeping
                        // (`applied`) but not the state mutation itself.
                        engine.applied.insert(envelope.mutation_id.0.clone(), envelope);
                        continue;
                    }
                    let (touched, _conflicts, _) = db_actor::block_on(engine.apply_one(&envelope, &batch_ids))?;
                    let touch = command_touch(&envelope, &touched);
                    if engine.recent_touches.len() >= MAX_RECENT_TOUCHES {
                        engine.recent_touches.pop_front();
                    }
                    engine.recent_touches.push_back(touch);
                    report.commands_replayed += 1;
                }
                // 🩹️ The authoritative post-commit frontier was written verbatim at commit time
                // (see `submit`'s final record) — replaying it directly avoids recomputing
                // `head_seq`/`commit_seq` from scratch and guarantees the reopened engine agrees
                // exactly with what was durable, even if this crate's own bookkeeping ever changes.
                db_wal::WalRecord::Frontier(frontier) => engine.frontier = frontier,
                _ => {}
            }
        }
        Ok((engine, report))
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
        let (new_state, touched, conflicts) = self.state.apply_entries(&envelope.mutation_id, &envelope.dependencies, &entries).await?;
        self.state = new_state;
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
        let mut records: Vec<db_wal::WalRecord> = Vec::new();
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
            records.push(db_wal::WalRecord::Command(envelope_bytes.clone()));
            records.push(db_wal::WalRecord::Outbox(envelope_bytes.clone()));
            newly_applied.push((envelope.clone(), touched, envelope_bytes));
        }

        if newly_applied.is_empty() {
            // Every envelope in this (re-)submitted batch was already durable individually — a
            // full no-op commit, per-envelope half of the dedupe law (see `apply_one`'s doc).
            let receipt = CommandReceipt { command_id, frontier: self.frontier.clone(), durability: options.durability, conflicts: Vec::new(), state_hash: Some(self.state.content_hash().await), messages: Vec::new() };
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
            Frontier { document: self.document.clone(), head_seq: self.frontier.head_seq + newly_applied.len() as u64, commit_seq: self.frontier.commit_seq + 1, chain_hash: self.state.content_hash().await.0, epoch: self.frontier.epoch };
        records.push(db_wal::WalRecord::Frontier(new_frontier.clone()));

        // WAL append + durability (ArtifactWal::submit wraps `records` in its own TxBegin/TxCommit)
        let wal_facet = self.storage.wal().await;
        self.wal.submit(&wal_facet, &records, options.durability, now_ms).await?;
        drop(wal_facet);
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
                    ChangeRecord { parent: None, content_hash: self.state.content_hash().await, author: to_core_actor_id(&envelope.actor).await, message: format!("operation {}", envelope.mutation_id.0), timestamp_ms: now_ms },
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
        let receipt = CommandReceipt { command_id, frontier: new_frontier, durability: options.durability, conflicts: conflicts_all, state_hash: Some(self.state.content_hash().await), messages };
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

    pub async fn get(&self, path: &str) -> Option<Vec<u8>> {
        self.state.get(path).await
    }

    pub async fn frontier(&self) -> Frontier {
        self.frontier.clone()
    }

    pub async fn commit_log(&self) -> &[CommitNotification] {
        &self.commit_log
    }

    /// @emoji 📤️ Hands out (and clears) every effect queued since the last drain.
    pub async fn drain_outbox(&mut self) -> Vec<OutboxEntry> {
        std::mem::take(&mut self.outbox)
    }

    //#region 🔖️Snapshot
    /// @emoji 📸️ Publishes a new `db_snapshot` generation of the whole current `DocumentState` —
    /// new this revision; the counterpart `open` reads back to accelerate materialization.
    pub async fn snapshot_now(&self, now_ms: u64) -> Result<u64, DbError> {
        let entries: Vec<(String, Option<Vec<u8>>)> = self.state.values.iter().map(|(path, bytes)| (path.clone(), Some(bytes.clone()))).collect();
        let page = db_state::Page::new(encode_state_page(&entries).await);
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
        db_query::execute(&query, &source, None::<&db_query::NoFullTextLookup>, &db_query::QueryLimits::default()).await
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
            if let Ok(diff) = live_query.refresh(&source, None::<&db_query::NoFullTextLookup>, &limits).await {
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
    pub async fn preview_get(&self, id: &db_preview::PreviewId, path: &str) -> Result<Option<Vec<u8>>, DbError> {
        let preview = self.previews.get(id).ok_or_else(|| DbError::NotFound(format!("preview {id} not found")))?;
        for (entry_path, value) in diff_entries(&preview.envelope.diff).await? {
            if entry_path == path {
                return match value {
                    Some(dsl_value) => Ok(Some(store::pack_rt::encode_wire_value(&dsl_value))),
                    None => Ok(None),
                };
            }
        }
        Ok(self.state.get(path).await)
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
struct StateQuerySource<'a>(&'a db_state::PMap<String, Vec<u8>>);

// 🚫️async: E1 pure accessor consumed by a sync Iterator::map (QuerySource::scan's row builder) — see R9
fn path_row_value(path: &str, bytes: &[u8]) -> db_query::Value {
    let mut map = std::collections::BTreeMap::new();
    map.insert("path".to_string(), db_query::Value::Text(path.to_string()));
    match std::str::from_utf8(bytes) {
        Ok(text) => {
            map.insert("value".to_string(), db_query::Value::Text(text.to_string()));
        }
        Err(_) => {
            map.insert("value".to_string(), db_query::Value::Bytes(bytes.to_vec()));
        }
    }
    db_query::Value::Map(map)
}

impl<'a> db_query::QuerySource for StateQuerySource<'a> {
    async fn scan(&self) -> Box<dyn Iterator<Item = (db_query::RowId, db_query::Value)> + '_> {
        Box::new(self.0.iter().enumerate().map(|(index, (path, bytes))| (db_query::RowId(index as u64), path_row_value(path, bytes))))
    }
}
//#endregion 🔖️QuerySource

//#region 🔖️Snapshot
/// @emoji 📸️ This crate's own snapshot page convention (opaque to `db_snapshot`/`db_storage`): the
/// whole `DocumentState` as of the snapshot's frontier, one entry per stored path.
async fn encode_state_page(entries: &[(String, Option<Vec<u8>>)]) -> Vec<u8> {
    let mut writer = pack::ByteWriter::new();
    writer.write_varint_u64(entries.len() as u64);
    for (path, value) in entries {
        writer.write_varint_u64(path.len() as u64);
        writer.write_bytes(path.as_bytes());
        match value {
            Some(bytes) => {
                writer.write_u8(1);
                writer.write_varint_u64(bytes.len() as u64);
                writer.write_bytes(bytes);
            }
            None => writer.write_u8(0),
        }
    }
    writer.into_bytes()
}

const MAX_STATE_PAGE_ENTRIES: u64 = 10_000_000;
const MAX_STATE_PAGE_PATH_BYTES: u64 = 4_096;
const MAX_STATE_PAGE_VALUE_BYTES: u64 = 256 * 1024 * 1024;

/// @emoji 🗺️ One decoded state page: `(path, value_bytes)` pairs, `None` marking a tombstoned path.
type StatePageEntries = Vec<(String, Option<Vec<u8>>)>;

async fn decode_state_page(bytes: &[u8]) -> Result<StatePageEntries, DbError> {
    let mut reader = pack::ByteReader::new(bytes);
    let count = reader.read_varint_u64()?;
    check_len(count, MAX_STATE_PAGE_ENTRIES, "db_artifact::snapshot_page_entries")?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let path_len = reader.read_varint_u64()?;
        check_len(path_len, MAX_STATE_PAGE_PATH_BYTES, "db_artifact::snapshot_page_path")?;
        let path_bytes = reader.read_bytes(path_len as usize)?.to_vec();
        let path = String::from_utf8(path_bytes).map_err(|_| DbError::Corrupt("snapshot page path is not valid utf-8".to_string()))?;
        let value = if reader.read_u8()? == 1 {
            let len = reader.read_varint_u64()?;
            check_len(len, MAX_STATE_PAGE_VALUE_BYTES, "db_artifact::snapshot_page_value")?;
            Some(reader.read_bytes(len as usize)?.to_vec())
        } else {
            None
        };
        entries.push((path, value));
    }
    Ok(entries)
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
        reply: db_actor::ReplySender<Option<Vec<u8>>>,
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
}

/// @emoji 🎭️ A live handle to one document's authority actor. Each admitted mailbox message wakes
/// one finite `WorkerPool` turn; no job waits for the next message and no authority owns an OS
/// thread. `db_state::PMap` uses `Arc`-shared HAMT nodes, making the engine movable between turns
/// without sharing mutable actor state or weakening its single-consumer mailbox semantics.
#[cfg(not(target_arch = "wasm32"))]
pub struct ArtifactAuthority {
    address: db_actor::Address<ArtifactMessage>,
    cancel: Arc<dyn Fn() + Send + Sync>,
    done: std::sync::Mutex<Option<db_actor::ReplyReceiver<()>>>,
}

#[cfg(not(target_arch = "wasm32"))]
struct ArtifactRunner<A: AuthzHook + 'static, V: VersionGraph + 'static> {
    pool: Arc<semio_framework_async::WorkerPool>,
    address: db_actor::Address<ArtifactMessage>,
    receiver: db_actor::Receiver<ArtifactMessage>,
    builder: std::sync::Mutex<Option<Box<dyn FnOnce() -> Result<ArtifactEngine<A, V>, DbError> + Send>>>,
    engine: std::sync::Mutex<Option<ArtifactEngine<A, V>>>,
    ready: std::sync::Mutex<Option<db_actor::ReplySender<Result<(), DbError>>>>,
    done: std::sync::Mutex<Option<db_actor::ReplySender<()>>>,
    scheduled: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    terminal: std::sync::atomic::AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl<A: AuthzHook + 'static, V: VersionGraph + 'static> ArtifactRunner<A, V> {
    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.terminal.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let runner = self.clone();
        self.pool.submit(semio_framework_async::Lane::UserVisible, Box::new(move || runner.run_turn()));
    }

    fn cancel(self: &Arc<Self>) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }

    fn finish(&self) {
        if !self.terminal.swap(true, std::sync::atomic::Ordering::AcqRel) {
            self.engine.lock().unwrap().take();
            if let Some(done) = self.done.lock().unwrap().take() {
                done.send(());
            }
        }
    }

    fn run_turn(self: Arc<Self>) {
        use std::panic::AssertUnwindSafe;
        use std::sync::atomic::Ordering;

        if let Some(build) = self.builder.lock().unwrap().take() {
            match std::panic::catch_unwind(AssertUnwindSafe(build)) {
                Ok(Ok(engine)) => {
                    *self.engine.lock().unwrap() = Some(engine);
                    if let Some(ready) = self.ready.lock().unwrap().take() {
                        ready.send(Ok(()));
                    }
                }
                Ok(Err(error)) => {
                    if let Some(ready) = self.ready.lock().unwrap().take() {
                        ready.send(Err(error));
                    }
                    self.finish();
                    return;
                }
                Err(_) => {
                    if let Some(ready) = self.ready.lock().unwrap().take() {
                        ready.send(Err(DbError::Internal("document authority construction panicked".to_string())));
                    }
                    self.finish();
                    return;
                }
            }
        }
        if self.cancelled.load(Ordering::Acquire) {
            self.finish();
            return;
        }
        let envelope = self.receiver.try_recv();
        if let Some(envelope) = envelope {
            let handled = {
                let mut engine = self.engine.lock().unwrap();
                let engine = engine.as_mut().expect("ArtifactRunner is scheduled only after construction");
                std::panic::catch_unwind(AssertUnwindSafe(|| match envelope.payload {
                    ArtifactMessage::Submit { batch, options, now_ms, reply } => reply.send(db_actor::block_on(engine.submit(batch, options, now_ms))),
                    ArtifactMessage::Query { path, reply } => reply.send(db_actor::block_on(engine.get(&path))),
                    ArtifactMessage::Frontier { reply } => reply.send(db_actor::block_on(engine.frontier())),
                    ArtifactMessage::RunQuery { query, consistency, reply } => reply.send(db_actor::block_on(engine.query(query, consistency))),
                    ArtifactMessage::SnapshotNow { now_ms, reply } => reply.send(db_actor::block_on(engine.snapshot_now(now_ms))),
                    ArtifactMessage::DrainOutbox { reply } => reply.send(db_actor::block_on(engine.drain_outbox())),
                }))
            };
            if handled.is_err() {
                self.address.close();
                self.finish();
                return;
            }
        }
        if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
            self.finish();
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if self.address.has_messages() {
            self.schedule();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactAuthority {
    /// @emoji 🚀️ Builds the engine on the injected pool and resolves only after construction, so
    /// a caller never receives an authority whose engine failed to open.
    pub async fn spawn<A: AuthzHook + 'static, V: VersionGraph + 'static>(
        pool: Arc<semio_framework_async::WorkerPool>,
        build: impl FnOnce() -> Result<ArtifactEngine<A, V>, DbError> + Send + 'static,
        capacities: MailboxCapacities,
    ) -> Result<ArtifactAuthority, DbError> {
        let (address, receiver) = db_actor::mailbox::<ArtifactMessage>(capacities);
        let (ready_tx, ready_rx) = db_actor::oneshot::<Result<(), DbError>>();
        let (done_tx, done_rx) = db_actor::oneshot();
        let runner = Arc::new(ArtifactRunner {
            pool,
            address: address.clone(),
            receiver,
            builder: std::sync::Mutex::new(Some(Box::new(build))),
            engine: std::sync::Mutex::new(None),
            ready: std::sync::Mutex::new(Some(ready_tx)),
            done: std::sync::Mutex::new(Some(done_tx)),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            terminal: std::sync::atomic::AtomicBool::new(false),
        });
        let weak = Arc::downgrade(&runner);
        address.set_consumer_wake(Arc::new(move || {
            if let Some(runner) = weak.upgrade() {
                runner.schedule();
            }
        }));
        let runner_for_cancel = runner.clone();
        let cancel: Arc<dyn Fn() + Send + Sync> = Arc::new(move || runner_for_cancel.cancel());
        runner.schedule();

        match ready_rx.await {
            Ok(Ok(())) => Ok(ArtifactAuthority { address, cancel, done: std::sync::Mutex::new(Some(done_rx)) }),
            Ok(Err(err)) => Err(err),
            Err(_) => Err(DbError::Closed),
        }
    }

    pub async fn submit_blocking(&self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> Result<CommandReceipt, DbError> {
        self.address.ask_blocking(Priority::Command, |reply| ArtifactMessage::Submit { batch, options, now_ms, reply })?
    }

    pub async fn query_blocking(&self, path: &str) -> Result<Option<Vec<u8>>, DbError> {
        let path = path.to_string();
        self.address.ask_blocking(Priority::Query, |reply| ArtifactMessage::Query { path, reply })
    }

    pub async fn frontier_blocking(&self) -> Result<Frontier, DbError> {
        self.address.ask_blocking(Priority::Query, |reply| ArtifactMessage::Frontier { reply })
    }

    pub async fn run_query_blocking(&self, query: db_query::Query, consistency: db_query::Consistency) -> Result<db_query::QueryResult, DbError> {
        self.address.ask_blocking(Priority::Query, |reply| ArtifactMessage::RunQuery { query, consistency, reply })?
    }

    pub async fn snapshot_now_blocking(&self, now_ms: u64) -> Result<u64, DbError> {
        self.address.ask_blocking(Priority::Command, |reply| ArtifactMessage::SnapshotNow { now_ms, reply })?
    }

    pub async fn drain_outbox_blocking(&self) -> Result<Vec<OutboxEntry>, DbError> {
        self.address.ask_blocking(Priority::Query, |reply| ArtifactMessage::DrainOutbox { reply })
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
//#endregion 🔖️Actor

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc as StdArc;

    async fn storage() -> StdArc<db_storage::DbBackend> {
        StdArc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new().await))
    }

    async fn document_id() -> protocol::ArtifactId {
        protocol::ArtifactId("doc-1".to_string())
    }

    async fn stored_json(bytes: &[u8]) -> serde_json::Value {
        decode_pathmap_json(bytes).await.expect("stored json value")
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

        let stored = engine.get("name").await.unwrap();
        let value: serde_json::Value = stored_json(&stored).await;
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

        let name: serde_json::Value = stored_json(&reopened.get("name").await.unwrap()).await;
        assert_eq!(name, serde_json::json!("hello"));
        let count: serde_json::Value = stored_json(&reopened.get("count").await.unwrap()).await;
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
            let value: serde_json::Value = stored_json(&reopened.get(&format!("path-{i}")).await.unwrap()).await;
            assert_eq!(value, serde_json::json!(format!("value-{i}")));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn deletion_via_json_null_tombstones_a_path() {
        let storage = storage().await;
        let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
        assert!(engine.get("x").await.is_some());
        engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("x", serde_json::Value::Null)]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
        assert!(engine.get("x").await.is_none());
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
        assert!(engine.get("x").await.is_none(), "a rejected batch must not have partially applied");
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
        let x: serde_json::Value = stored_json(&engine.get("x").await.unwrap()).await;
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
        assert!(engine.get("x").await.is_some());

        let receipt = engine.undo(&protocol::MutationId("op-1".to_string()), protocol::MutationId("op-1-undo".to_string()), protocol::ActorId("alice".to_string()), 1).await.unwrap();
        assert_eq!(receipt.frontier.head_seq, 2);
        assert!(engine.get("x").await.is_none(), "undo must have applied the recorded inverse (delete x)");
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
        let preview_value: serde_json::Value = stored_json(&engine.preview_get(&preview_id, "y").await.unwrap().unwrap()).await;
        assert_eq!(preview_value, serde_json::json!("preview-value"));
        assert!(engine.get("y").await.is_none(), "a preview must never be visible in committed state");

        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("y", serde_json::json!("committed-value"))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
        assert_eq!(engine.preview_status(&preview_id).await.unwrap(), db_preview::PreviewState::Superseded, "an intersecting real commit must supersede the preview");

        let committed: serde_json::Value = stored_json(&engine.get("y").await.unwrap()).await;
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
        assert!(engine.get("x").await.is_none());
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
        let authority = ArtifactAuthority::spawn(pool.clone(), move || ArtifactEngine::create(document, storage, ArtifactEngineConfig::default(), 0), MailboxCapacities::uniform(16)).await.unwrap();

        let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hi"))]).await]).await.unwrap();
        let receipt = authority.submit_blocking(batch, SubmitOptions::default(), 0).await.unwrap();
        assert_eq!(receipt.frontier.head_seq, 1);

        let queried: serde_json::Value = stored_json(&authority.query_blocking("name").await.unwrap().unwrap()).await;
        assert_eq!(queried, serde_json::json!("hi"));

        let frontier = authority.frontier_blocking().await.unwrap();
        assert_eq!(frontier.head_seq, 1);

        let generation = authority.snapshot_now_blocking(1).await.unwrap();
        assert_eq!(generation, 0);

        authority.shutdown().await;
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn document_authority_spawn_propagates_a_build_failure_synchronously() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
        let result = ArtifactAuthority::spawn(pool.clone(), || -> Result<ArtifactEngine<AllowAll, NullVersionGraph>, DbError> { Err(DbError::InvalidArgument("boom".to_string())) }, MailboxCapacities::uniform(4));
        assert!(matches!(result.await, Err(DbError::InvalidArgument(_))));
        pool.shutdown();
    }
    //#endregion 🔖️Actor
}
//#endregion 🧪️Tests
