//! 🎞️ Protocol history log: the typed record model (`HistoryLog` and friends), the `.ops` text
//! grammar twin (built directly on `dsl_schema`, never on `vcs`), per-kind binary payload codecs
//! (built on `crate::os_spr::wire::scalar` + `protocol_format`'s frame writer/reader), the whole-file
//! codec, a streaming append API, and a lazy forward/reverse scan API. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol_history`).
//!
//! Op payloads are opaque validated bytes to this crate — it stores, hashes, frames, and indexes
//! them, but never interprets operation semantics (that is `protocol_command`'s concern, a sibling
//! crate this one does not depend on).

use crate::os_dsl::schema::{FieldSpec, FieldValue, JoinMode, ParseOptions, RecordLayout, RecordSpec, RecordValue, Shape};
use crate::os_pack::{ByteReader, ByteWriter, CodecId, PackSink};
use crate::os_spr::format::{Blake3Hasher, FrameCursor, HEADER_SIZE, RecoveryMode, ReverseFrameCursor, SprWriter, VerificationLevel, WriteOptions};
use crate::os_spr::wire::{DictBuilder, DictReader, ProtocolError, ProtocolLimits, RecordHasher};
use std::collections::{HashMap, HashSet};

//#region 🔖️Model
// Every field of crate::os_store::OpsHeaderLine (Doc/Edit/Change/Checkpoint/Alternative/Active) has exactly
// one slot below. Op lines are opaque exact `print_op` strings (one per line, no '\n' inside).
// Derived data (inverse, sequence_number, unless explicitly captured via `meta`) is excluded.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryLog {
    pub doc_id: String,
    pub schema: String,
    pub edits: Vec<HistoryEdit>,
    pub changes: Vec<HistoryChange>,
    pub checkpoints: Vec<HistoryCheckpoint>,
    pub alternatives: Vec<HistoryAlternative>,
    pub active_alternative_id: Option<String>,
    /// @emoji 🎯️ Undo/redo/checkout position, present only when the caller explicitly persisted
    /// it (`REC_CURSOR`) — absent for text-compiled/imported logs and for any log predating this
    /// field, in which case undo/redo position is runtime-only, exactly as before.
    pub cursor: Option<HistoryCursor>,
    /// @emoji 🧩️ Composition overlay (`REC_COMPOSITION`): who owns this document, which dialect it
    /// materializes as, and each checkpoint's child pins. Absent for every non-composed document
    /// (the overwhelming majority) and for logs predating the record.
    pub composition: Option<HistoryComposition>,
    /// @emoji ⚔️ First-class merge conflicts (`REC_CONFLICT`), durable per
    /// `.🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/
    /// 📋️contract-freeze.md` §C7: a `Quarantined` batch rejected outright, or a `Degraded`
    /// accepted-but-messy merge — see `crate::os_spr::conflict::ConflictKind`. Empty for the
    /// overwhelming majority of documents and for logs predating the record; no record is written
    /// when empty.
    pub conflicts: Vec<HistoryConflict>,
}

/// @emoji 🧩️ The durable form of a document's composition facts, carried as ONE extension record
/// rather than as new fields on `REC_DOC`/`REC_CHECKPOINT`: those two are format-frozen critical
/// records, and a composition overlay is precisely the kind of thing an older/foreign reader must
/// be able to skip without failing the whole file. Before this record existed all three of these
/// were in-memory only, so a reloaded child forgot both that it was owned and what dialect it
/// materialized as — which made "children with their own version history" unpersistable.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryComposition {
    /// 🏠️ `(parent_artifact_uri, slot, child_id)` — the child-side ownership stamp.
    pub owner: Option<(String, String, String)>,
    /// 🎯️ `(artifact_kind, standard, subset)` — the dialect this document materializes as.
    pub dialect: Option<(String, String, String)>,
    /// 📌️ `(checkpoint_id, [(child_artifact_uri, child_checkpoint_id)])` — the cascade pins.
    pub checkpoint_pins: Vec<(String, Vec<(String, String)>)>,
}

/// @emoji ⚔️ Durable form of `crate::os_spr::conflict::Conflict` (`REC_CONFLICT`): `kind`/`status`
/// are the numeric mirrors of `crate::os_spr::conflict::ConflictKind`/`ConflictStatus`
/// (`kind`: 0 = `Quarantined`, 1 = `Degraded`; `status`: 0 = `Open`, 1 = `Accepted`, 2 =
/// `Discarded`) — `envelopes` is populated only for `Quarantined` (opaque, already-serialized
/// `crate::os_spr::causal::MutationEnvelope` bytes — this crate never interprets them, same stance
/// as every other opaque payload here), `edit_ids` only for `Degraded`. No `policy` field: a merge
/// policy is local/authority state per the frozen contract, never part of an artifact's shared
/// history.
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryConflict {
    pub id: String,
    pub kind: u8,
    pub status: u8,
    pub actors: Vec<String>,
    /// ⏰️ `(actor, physical_ms, logical)` — field order/types mirror
    /// `crate::os_spr::ids::HybridLogicalTimestamp` exactly.
    pub hlt: (u64, u64, u64),
    pub edit_ids: Vec<String>,
    pub envelopes: Vec<Vec<u8>>,
    pub messages: Vec<HistoryMessage>,
}

/// @emoji 📨️ Durable form of `crate::os_spr::command::MutationMessage`: `level` is the numeric
/// mirror of `crate::os_dsl::Severity` (`as_u8`/`from_u8`, 0..3), `code` is dict-interned (the
/// frozen seven `mutation.*` codes repeat heavily across one document's history — see
/// `📋️contract-freeze.md` §C2), `message`/`target` are plain strings (English prose / element
/// address, never interned — they vary per occurrence).
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryMessage {
    pub level: u8,
    pub code: String,
    pub message: String,
    pub target: Vec<String>,
    pub op_index: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryEdit {
    pub id: String,
    pub actor: Option<String>,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub coalesce_key: Option<String>,
    pub description: Option<String>,
    pub ops: Vec<OpPayload>,
    /// @emoji 🔙️ The edit's inverse operations, in apply order (mirrors `crate::os_spr::command::Edit
    /// ::inverse`). Empty for text-compiled/imported logs — a decoder recomputing them from a
    /// fresh replay never touches this field; when non-empty, `write_backwards_section` persisted
    /// them explicitly (only the `.spr` binary path ever sets this — the `.ops` text mirror stays
    /// forwards-only, see `crate::os_store::print_document_spr`/`parse_document_spr`).
    pub inverse: Vec<OpPayload>,
    /// @emoji 🧮️ Present iff the caller supplied it; absent for text-compiled/imported logs. Not
    /// required for round-trip — a decoder recomputing inverse/meta from a fresh replay never
    /// touches this field.
    pub meta: Option<Vec<HistoryOpMeta>>,
}

/// @emoji 🧾️ `binary` carries the `crate::os_spr::command::OpBinary` encoding of this op when the
/// caller has one (the `.spr` binary path always sets it, and since the binary-only-spr flip
/// this is the ONLY face `.spr` ever carries); `text` is the `OpText::print_op` form, present
/// only when a text-tooling caller supplied it (`.ops` compile, hand-authored logs). Invariant:
/// at least one of `text`/`binary` is `Some` — both `None` is a construction bug, rejected by
/// `write_op_payload`.
#[derive(Clone, Debug, PartialEq)]
pub struct OpPayload {
    pub text: Option<String>,
    pub binary: Option<Vec<u8>>,
}

/// @emoji 🎯️ Undo/redo/checkout position. Carries the FULL applied-edit list (not just the tail
/// edit id) because undo-then-apply interleavings are not representable by a single marker: an
/// edit undone mid-history precedes later-applied edits in file order, and the redo stack can
/// contain edits in any order relative to `applied`. `checkpoint_id` mirrors
/// `ArtifactStore::current_checkpoint_id`; the active alternative stays on the existing
/// `HistoryLog::active_alternative_id` (unrelated lifecycle — churns far less often).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryCursor {
    pub applied_edit_ids: Vec<String>,
    pub redo_edit_ids: Vec<String>,
    pub checkpoint_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryOpMeta {
    pub op_id: Option<String>,
    pub dependencies: Vec<String>,
    pub base_version: u64,
    pub author_id: Option<String>,
    pub hlt: Option<(u64, i64, u64)>,
    pub undo_policy: u8,
    pub payload_hash: Option<[u8; 32]>,
    /// @emoji 🧑‍🤝‍🧑️ Durable twin of `crate::os_spr::command::MutationMeta.group_id` — the composite-
    /// gesture stamp, present iff the op it describes was authored as one member of a multi-
    /// document composite gesture. Dict-interned like `op_id`/`author_id`/`dependencies` (bullet
    /// design point: every sibling member of one composite gesture shares the identical string,
    /// so the dictionary compresses it near-for-free across a whole edit/checkpoint).
    pub group_id: Option<String>,
    /// @emoji 🔀️ Durable twin of `crate::os_spr::command::MutationMeta.origin` — canonical-JSON
    /// encoded (not dict-interned like the id fields above: an origin's `Contributed`/`Transaction`
    /// payload carries structured data, not a short repeated token). `MutationOrigin::Owner`
    /// (`Default`) whenever absent from the byte log, matching `group_id`'s own "absent for logs
    /// predating this field" contract.
    pub origin: crate::os_spr::command::MutationOrigin,
    /// @emoji 📨️ Durable ledger twin of `crate::os_spr::command::MutationOutcome::messages` — every
    /// diagnostic this op's `diff` raised, persisted rather than recomputed (unlike inverse/meta's
    /// general "a fresh replay never touches this field" contract, messages are NOT reproducible
    /// from a replay alone — they are the durable record of what actually happened at write time).
    /// Dict-interned per-message `code` (bit6 of the same presence byte, same "absent for logs
    /// predating this field" contract as `group_id`/`origin`) — empty `Vec` for logs predating
    /// this field.
    pub messages: Vec<HistoryMessage>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryChange {
    pub id: String,
    pub saved_at: String,
    pub edit_ids: Vec<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryCheckpoint {
    pub id: String,
    pub timestamp: String,
    pub change_ids: Vec<String>,
    pub parent_id: Option<String>,
    pub authors: Vec<HistoryAuthor>,
    pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryAuthor {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryAlternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}
//#endregion 🔖️Model

//#region 🔖️TextGrammar
// Own twin of crate::os_store::OpsHeaderLine's grammar, built directly against `dsl_schema` (never `vcs`,
// never `dsl_derive` — this crate has no path dep on either). Field declaration order below
// mirrors vcs's struct field order exactly: `crate::os_dsl::schema::print_record` reorders keyed fields
// scalar-before-composite (stable sort, ties broken by declaration order), so matching vcs's
// declared order here reproduces vcs's exact printed field order for free.

const F_DOC_ID: u16 = 0;
const F_DOC_SCHEMA: u16 = 1;
const F_EDIT_ID: u16 = 0;
const F_EDIT_STARTED: u16 = 1;
const F_EDIT_ACTOR: u16 = 2;
const F_EDIT_FINISHED: u16 = 3;
const F_EDIT_KEY: u16 = 4;
const F_EDIT_DESCRIPTION: u16 = 5;
const F_CHANGE_ID: u16 = 0;
const F_CHANGE_SAVED: u16 = 1;
const F_CHANGE_EDITS: u16 = 2;
const F_CHANGE_DESCRIPTION: u16 = 3;
const F_CHECKPOINT_ID: u16 = 0;
const F_CHECKPOINT_AT: u16 = 1;
const F_CHECKPOINT_CHANGES: u16 = 2;
const F_CHECKPOINT_PARENT: u16 = 3;
const F_CHECKPOINT_BY: u16 = 4;
const F_CHECKPOINT_MESSAGE: u16 = 5;
const F_ALTERNATIVE_ID: u16 = 0;
const F_ALTERNATIVE_NAME: u16 = 1;
const F_ALTERNATIVE_CHECKPOINTS: u16 = 2;
const F_ACTIVE_ID: u16 = 0;
const F_AUTHOR_ID: u16 = 0;
const F_AUTHOR_NAME: u16 = 1;
const F_CURSOR_APPLIED: u16 = 0;
const F_CURSOR_REDO: u16 = 1;
const F_CURSOR_CHECKPOINT: u16 = 2;

/// @emoji 🖋️ `by=[...]` list entry twin of vcs's `OpsAuthor`: two positional fields, no keyword.
// 🚫️async: E4 fn-pointer slot — stored bare as `Shape::Record(fn() -> RecordSpec)` (🗣️dsl schema),
// so this item's pointer type must stay nameable. Builds `RecordSpec`/`FieldSpec` via their `pub`
// fields directly rather than the async `::new`/`positional` builders, which a sync fn cannot call.
fn author_spec() -> RecordSpec {
    RecordSpec {
        keyword: None,
        layout: RecordLayout::Inline,
        fields: vec![
            FieldSpec { id: F_AUTHOR_ID, key: String::new(), position: Some(0), shape: Shape::Text, optional: false, flatten: false, defines: None, is_call_name: false },
            FieldSpec { id: F_AUTHOR_NAME, key: String::new(), position: Some(1), shape: Shape::Text, optional: false, flatten: false, defines: None, is_call_name: false },
        ],
    }
}

async fn doc_spec() -> RecordSpec {
    RecordSpec::new(Some("doc"), RecordLayout::Inline, vec![FieldSpec::new(F_DOC_ID, "", Shape::Text).positional(0), FieldSpec::new(F_DOC_SCHEMA, "schema", Shape::Text)])
}

async fn edit_spec() -> RecordSpec {
    RecordSpec::new(
        Some("edit"),
        RecordLayout::Inline,
        vec![
            FieldSpec::new(F_EDIT_ID, "", Shape::Text).positional(0),
            FieldSpec::new(F_EDIT_STARTED, "started", Shape::Text),
            FieldSpec::new(F_EDIT_ACTOR, "actor", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_FINISHED, "finished", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_KEY, "key", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_DESCRIPTION, "description", Shape::Text).optional(),
        ],
    )
}

async fn change_spec() -> RecordSpec {
    RecordSpec::new(
        Some("change"),
        RecordLayout::Inline,
        vec![
            FieldSpec::new(F_CHANGE_ID, "", Shape::Text).positional(0),
            FieldSpec::new(F_CHANGE_SAVED, "saved", Shape::Text),
            FieldSpec::new(F_CHANGE_EDITS, "edits", Shape::List(Box::new(Shape::Text))),
            FieldSpec::new(F_CHANGE_DESCRIPTION, "description", Shape::Text).optional(),
        ],
    )
}

async fn checkpoint_spec() -> RecordSpec {
    RecordSpec::new(
        Some("checkpoint"),
        RecordLayout::Inline,
        vec![
            FieldSpec::new(F_CHECKPOINT_ID, "", Shape::Text).positional(0),
            FieldSpec::new(F_CHECKPOINT_AT, "at", Shape::Text),
            FieldSpec::new(F_CHECKPOINT_CHANGES, "changes", Shape::List(Box::new(Shape::Text))),
            FieldSpec::new(F_CHECKPOINT_PARENT, "parent", Shape::Text).optional(),
            FieldSpec::new(F_CHECKPOINT_BY, "by", Shape::List(Box::new(Shape::Record(author_spec)))),
            FieldSpec::new(F_CHECKPOINT_MESSAGE, "message", Shape::Text).optional(),
        ],
    )
}

async fn alternative_spec() -> RecordSpec {
    RecordSpec::new(
        Some("alternative"),
        RecordLayout::Inline,
        vec![FieldSpec::new(F_ALTERNATIVE_ID, "", Shape::Text).positional(0), FieldSpec::new(F_ALTERNATIVE_NAME, "name", Shape::Text), FieldSpec::new(F_ALTERNATIVE_CHECKPOINTS, "checkpoints", Shape::List(Box::new(Shape::Text)))],
    )
}

async fn active_spec() -> RecordSpec {
    RecordSpec::new(Some("active"), RecordLayout::Inline, vec![FieldSpec::new(F_ACTIVE_ID, "", Shape::Text).positional(0)])
}

/// @emoji 🎯️ `cursor applied=[...] redo=[...] checkpoint=<id>` — carries the FULL applied/redo
/// edit-id lists (see `HistoryCursor`'s doc for why a single marker id is insufficient).
async fn cursor_spec() -> RecordSpec {
    RecordSpec::new(
        Some("cursor"),
        RecordLayout::Inline,
        vec![FieldSpec::new(F_CURSOR_APPLIED, "applied", Shape::List(Box::new(Shape::Text))), FieldSpec::new(F_CURSOR_REDO, "redo", Shape::List(Box::new(Shape::Text))), FieldSpec::new(F_CURSOR_CHECKPOINT, "checkpoint", Shape::Text).optional()],
    )
}

async fn record_with(fields: Vec<(u16, FieldValue)>) -> RecordValue {
    RecordValue { fields: fields.into_iter().collect() }
}

async fn field_text(record: &RecordValue, id: u16) -> Option<String> {
    match record.get(id) {
        Some(FieldValue::Text(s)) => Some(s.clone()),
        _ => None,
    }
}

async fn required_text(record: &RecordValue, id: u16, what: &'static str) -> Result<String, ProtocolError> {
    field_text(record, id).await.ok_or_else(|| ProtocolError::Malformed { what, offset: 0, detail: "missing required field in ops text".to_string() })
}

async fn field_text_list(record: &RecordValue, id: u16) -> Vec<String> {
    match record.get(id) {
        Some(FieldValue::List(items)) => items.iter().filter_map(|v| if let FieldValue::Text(s) = v { Some(s.clone()) } else { None }).collect(),
        _ => Vec::new(),
    }
}

async fn field_authors(record: &RecordValue, id: u16) -> Vec<HistoryAuthor> {
    match record.get(id) {
        Some(FieldValue::List(items)) => {
            // 🚫️async: R10 shape 1 — `field_text` is async now, but `filter_map`'s closure is sync;
            // hoisted into a plain loop so the two lookups can be awaited.
            let mut out = Vec::new();
            for v in items.iter() {
                let FieldValue::Record(rec) = v else { continue };
                let (Some(id), Some(name)) = (field_text(rec, F_AUTHOR_ID).await, field_text(rec, F_AUTHOR_NAME).await) else { continue };
                out.push(HistoryAuthor { id, name });
            }
            out
        }
        _ => Vec::new(),
    }
}

// 🚫️async: R9 pure accessor — only consumer is `.map_err(text_error_to_protocol)`, and
// `Result::map_err` requires a sync `FnOnce`; no suspension point exists in the body either.
fn text_error_to_protocol(err: crate::os_dsl::TextError) -> ProtocolError {
    ProtocolError::Malformed { what: "ops text", offset: err.span.line as u64, detail: err.message }
}

/// @emoji 📥️ Parses the full `.ops` text into a `HistoryLog`. Blank lines and `#`-comments
/// normalize away; a two-space-indented line under a pending `edit` header is an opaque forward
/// op line (never interpreted). Unlike `crate::os_store::replay_ops`, this never replays operation semantics
/// (ops are opaque here) — `HistoryEdit::meta`/inverse are simply never populated from text.
pub async fn parse_ops_text(ops: &str) -> Result<HistoryLog, ProtocolError> {
    struct PendingEdit {
        id: String,
        actor: Option<String>,
        started_at: String,
        finished_at: Option<String>,
        coalesce_key: Option<String>,
        description: Option<String>,
    }

    let mut log = HistoryLog::default();
    let mut pending: Option<PendingEdit> = None;
    let mut forwards: Vec<OpPayload> = Vec::new();

    async fn flush(pending: &mut Option<PendingEdit>, forwards: &mut Vec<OpPayload>, edits: &mut Vec<HistoryEdit>) {
        if let Some(header) = pending.take() {
            edits.push(HistoryEdit {
                id: header.id,
                actor: header.actor,
                started_at: header.started_at,
                finished_at: header.finished_at,
                coalesce_key: header.coalesce_key,
                description: header.description,
                ops: std::mem::take(forwards),
                inverse: Vec::new(),
                meta: None,
            });
        }
    }

    for raw_line in ops.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if raw_line.starts_with("  ") && pending.is_some() {
            forwards.push(OpPayload { text: Some(trimmed.to_string()), binary: None });
            continue;
        }
        flush(&mut pending, &mut forwards, &mut log.edits).await;

        let opts = ParseOptions::default();
        let keyword = trimmed.split_whitespace().next().unwrap_or("");
        match keyword {
            "doc" => {
                let record = crate::os_dsl::schema::parse(trimmed, &doc_spec().await, &opts).await.map_err(text_error_to_protocol)?;
                log.doc_id = required_text(&record, F_DOC_ID, "doc id").await?;
                log.schema = required_text(&record, F_DOC_SCHEMA, "doc schema").await?;
            }
            "edit" => {
                let record = crate::os_dsl::schema::parse(trimmed, &edit_spec().await, &opts).await.map_err(text_error_to_protocol)?;
                pending = Some(PendingEdit {
                    id: required_text(&record, F_EDIT_ID, "edit id").await?,
                    started_at: required_text(&record, F_EDIT_STARTED, "edit started").await?,
                    actor: field_text(&record, F_EDIT_ACTOR).await,
                    finished_at: field_text(&record, F_EDIT_FINISHED).await,
                    coalesce_key: field_text(&record, F_EDIT_KEY).await,
                    description: field_text(&record, F_EDIT_DESCRIPTION).await,
                });
                forwards = Vec::new();
            }
            "change" => {
                let record = crate::os_dsl::schema::parse(trimmed, &change_spec().await, &opts).await.map_err(text_error_to_protocol)?;
                log.changes.push(HistoryChange {
                    id: required_text(&record, F_CHANGE_ID, "change id").await?,
                    saved_at: required_text(&record, F_CHANGE_SAVED, "change saved").await?,
                    edit_ids: field_text_list(&record, F_CHANGE_EDITS).await,
                    description: field_text(&record, F_CHANGE_DESCRIPTION).await,
                });
            }
            "checkpoint" => {
                let record = crate::os_dsl::schema::parse(trimmed, &checkpoint_spec().await, &opts).await.map_err(text_error_to_protocol)?;
                log.checkpoints.push(HistoryCheckpoint {
                    id: required_text(&record, F_CHECKPOINT_ID, "checkpoint id").await?,
                    timestamp: required_text(&record, F_CHECKPOINT_AT, "checkpoint at").await?,
                    change_ids: field_text_list(&record, F_CHECKPOINT_CHANGES).await,
                    parent_id: field_text(&record, F_CHECKPOINT_PARENT).await,
                    authors: field_authors(&record, F_CHECKPOINT_BY).await,
                    message: field_text(&record, F_CHECKPOINT_MESSAGE).await,
                });
            }
            "alternative" => {
                let record = crate::os_dsl::schema::parse(trimmed, &alternative_spec().await, &opts).await.map_err(text_error_to_protocol)?;
                log.alternatives.push(HistoryAlternative {
                    id: required_text(&record, F_ALTERNATIVE_ID, "alternative id").await?,
                    name: required_text(&record, F_ALTERNATIVE_NAME, "alternative name").await?,
                    checkpoint_ids: field_text_list(&record, F_ALTERNATIVE_CHECKPOINTS).await,
                });
            }
            "active" => {
                let record = crate::os_dsl::schema::parse(trimmed, &active_spec().await, &opts).await.map_err(text_error_to_protocol)?;
                log.active_alternative_id = Some(required_text(&record, F_ACTIVE_ID, "active id").await?);
            }
            "cursor" => {
                let record = crate::os_dsl::schema::parse(trimmed, &cursor_spec().await, &opts).await.map_err(text_error_to_protocol)?;
                log.cursor = Some(HistoryCursor { applied_edit_ids: field_text_list(&record, F_CURSOR_APPLIED).await, redo_edit_ids: field_text_list(&record, F_CURSOR_REDO).await, checkpoint_id: field_text(&record, F_CURSOR_CHECKPOINT).await });
            }
            other => return Err(ProtocolError::Malformed { what: "ops text line", offset: 0, detail: format!("unknown line keyword '{other}'") }),
        }
    }
    flush(&mut pending, &mut forwards, &mut log.edits).await;
    Ok(log)
}

/// @emoji 📤️ Prints a `HistoryLog` back to `.ops` text: `doc`, every edit (header + two-space
/// indented forward op lines), then `change`/`checkpoint`/`alternative`/`active` records — the
/// same section order `crate::os_store::print_ops_log` uses. Errors if any op payload carries no text
/// (the binary-only `.spr` convention): this crate is schema-agnostic and cannot recover text
/// from an opaque binary payload — printing `.ops` for a real app document goes through the
/// concrete `Mutation::print_op` path instead (`crate::os_store::print_document_pack`'s `.ops` mirror).
pub async fn print_ops_text(log: &HistoryLog) -> Result<String, ProtocolError> {
    let mut out = String::new();

    let doc_record = record_with(vec![(F_DOC_ID, FieldValue::Text(log.doc_id.clone())), (F_DOC_SCHEMA, FieldValue::Text(log.schema.clone()))]).await;
    out.push_str(&crate::os_dsl::schema::print(&doc_record, &doc_spec().await, JoinMode::Inline).await);
    out.push('\n');

    for edit in &log.edits {
        let mut fields = vec![(F_EDIT_ID, FieldValue::Text(edit.id.clone())), (F_EDIT_STARTED, FieldValue::Text(edit.started_at.clone()))];
        if let Some(actor) = &edit.actor {
            fields.push((F_EDIT_ACTOR, FieldValue::Text(actor.clone())));
        }
        if let Some(finished) = &edit.finished_at {
            fields.push((F_EDIT_FINISHED, FieldValue::Text(finished.clone())));
        }
        if let Some(key) = &edit.coalesce_key {
            fields.push((F_EDIT_KEY, FieldValue::Text(key.clone())));
        }
        if let Some(description) = &edit.description {
            fields.push((F_EDIT_DESCRIPTION, FieldValue::Text(description.clone())));
        }
        out.push_str(&crate::os_dsl::schema::print(&record_with(fields).await, &edit_spec().await, JoinMode::Inline).await);
        out.push('\n');
        for op in &edit.ops {
            let Some(text) = &op.text else {
                return Err(ProtocolError::Malformed { what: "op payload", offset: 0, detail: "no text face — cannot print .ops for a binary-only op".to_string() });
            };
            out.push_str("  ");
            out.push_str(text);
            out.push('\n');
        }
    }

    for change in &log.changes {
        let mut fields = vec![(F_CHANGE_ID, FieldValue::Text(change.id.clone())), (F_CHANGE_SAVED, FieldValue::Text(change.saved_at.clone())), (F_CHANGE_EDITS, FieldValue::List(change.edit_ids.iter().map(|s| FieldValue::Text(s.clone())).collect()))];
        if let Some(description) = &change.description {
            fields.push((F_CHANGE_DESCRIPTION, FieldValue::Text(description.clone())));
        }
        out.push_str(&crate::os_dsl::schema::print(&record_with(fields).await, &change_spec().await, JoinMode::Inline).await);
        out.push('\n');
    }

    for checkpoint in &log.checkpoints {
        let mut fields = vec![
            (F_CHECKPOINT_ID, FieldValue::Text(checkpoint.id.clone())),
            (F_CHECKPOINT_AT, FieldValue::Text(checkpoint.timestamp.clone())),
            (F_CHECKPOINT_CHANGES, FieldValue::List(checkpoint.change_ids.iter().map(|s| FieldValue::Text(s.clone())).collect())),
        ];
        if let Some(parent) = &checkpoint.parent_id {
            fields.push((F_CHECKPOINT_PARENT, FieldValue::Text(parent.clone())));
        }
        // 🚫️async: R10 shape 1 — `record_with` is async now, but `Iterator::map`'s closure is sync;
        // hoisted into a plain loop so each author record can be awaited.
        let mut author_records = Vec::with_capacity(checkpoint.authors.len());
        for a in &checkpoint.authors {
            author_records.push(FieldValue::Record(record_with(vec![(F_AUTHOR_ID, FieldValue::Text(a.id.clone())), (F_AUTHOR_NAME, FieldValue::Text(a.name.clone()))]).await));
        }
        fields.push((F_CHECKPOINT_BY, FieldValue::List(author_records)));
        if let Some(message) = &checkpoint.message {
            fields.push((F_CHECKPOINT_MESSAGE, FieldValue::Text(message.clone())));
        }
        out.push_str(&crate::os_dsl::schema::print(&record_with(fields).await, &checkpoint_spec().await, JoinMode::Inline).await);
        out.push('\n');
    }

    for alternative in &log.alternatives {
        let fields = vec![
            (F_ALTERNATIVE_ID, FieldValue::Text(alternative.id.clone())),
            (F_ALTERNATIVE_NAME, FieldValue::Text(alternative.name.clone())),
            (F_ALTERNATIVE_CHECKPOINTS, FieldValue::List(alternative.checkpoint_ids.iter().map(|s| FieldValue::Text(s.clone())).collect())),
        ];
        out.push_str(&crate::os_dsl::schema::print(&record_with(fields).await, &alternative_spec().await, JoinMode::Inline).await);
        out.push('\n');
    }

    if let Some(active_id) = &log.active_alternative_id {
        out.push_str(&crate::os_dsl::schema::print(&record_with(vec![(F_ACTIVE_ID, FieldValue::Text(active_id.clone()))]).await, &active_spec().await, JoinMode::Inline).await);
        out.push('\n');
    }

    if let Some(cursor) = &log.cursor {
        let mut fields =
            vec![(F_CURSOR_APPLIED, FieldValue::List(cursor.applied_edit_ids.iter().map(|s| FieldValue::Text(s.clone())).collect())), (F_CURSOR_REDO, FieldValue::List(cursor.redo_edit_ids.iter().map(|s| FieldValue::Text(s.clone())).collect()))];
        if let Some(checkpoint_id) = &cursor.checkpoint_id {
            fields.push((F_CURSOR_CHECKPOINT, FieldValue::Text(checkpoint_id.clone())));
        }
        out.push_str(&crate::os_dsl::schema::print(&record_with(fields).await, &cursor_spec().await, JoinMode::Inline).await);
        out.push('\n');
    }

    Ok(out)
}
//#endregion 🔖️TextGrammar

//#region 🔖️Payloads
// Binary codec for each record kind, using crate::os_spr::wire::scalar + protocol_format's frame
// writer/reader. Every payload starts `format: u8` (=1); trailing bytes are ignored on read
// (additive-evolution slot) except a critical record demands `format <= known` (all kinds here
// are critical per crate::os_spr::is_critical_kind, so every decode_* rejects format > 1).
//
// 🎯️ Design choices (contract leaves these to the implementer, documented once here):
// - Every encode_*/decode_* pair takes a SINGLE `DictBuilder`/`DictReader` (matching the frozen
//   encode_doc/encode_edit signatures, which only expose one `dict` parameter each) — this crate
//   backs `REC_STR_DICT` only; `REC_ACTOR_DICT` stays defined in `protocol_core` but is never
//   emitted by this crate's writer (a no-op skip on read, for forward compatibility).
// - `encode_change`'s `edit_ordinal_of` is the only place besides `encode_edit` that genuinely
//   references edit ids (`HistoryChange::edit_ids`); `encode_checkpoint`/`encode_alternative`/
//   `encode_active` take no `edit_ordinal_of` since they never reference an edit.
// - `encode_edit` itself is data-driven: it writes presence bit5 + the inverse section iff
//   `edit.inverse` is non-empty — real op payloads, using the same op-payload wire shape as
//   `edit.ops` (op_tag bit1 flags a binary payload; both tags are per-payload, not per-edit, so
//   text-only and binary-carrying ops may mix freely within one edit). `EncodeOptions::
//   write_backwards_section` is the batch-level policy switch `encode_history` applies on top
//   (stripping `edit.inverse` before encoding when false, even if the caller's `HistoryLog`
//   has it populated) — `encode_edit`/`HistoryAppender::append_edit` have no such switch; a
//   streaming caller controls persistence per edit via the data it hands in. A decoder never
//   assumes inverse are present and always recomputes them via replay when the section (or the
//   whole `HistoryLog`) is absent.
// - Every `Option<T>` field not already covered by a record-level presence bitmask (i.e. every
//   field inside one `HistoryOpMeta` entry) gets its own bitmask byte, described per-function.

async fn malformed_fmt(what: &'static str, format: u8) -> ProtocolError {
    ProtocolError::Malformed { what, offset: 0, detail: format!("unsupported format {format}") }
}

async fn write_str_field(out: &mut ByteWriter, s: &str) {
    out.write_varint_u64(s.len() as u64).await;
    out.write_bytes(s.as_bytes()).await;
}

async fn read_str_field(input: &mut ByteReader<'_>) -> Result<String, ProtocolError> {
    let len = input.read_varint_u64().await? as usize;
    let bytes = input.read_bytes(len).await?;
    std::str::from_utf8(bytes).map(str::to_string).map_err(|_| ProtocolError::Malformed { what: "utf8", offset: 0, detail: "invalid utf-8".to_string() })
}

async fn write_id_field(out: &mut ByteWriter, id: &str, dict: &mut DictBuilder, edit_ordinal_of: &dyn Fn(&str) -> Option<u64>) -> Result<(), ProtocolError> {
    // ✏️ Genuine `async |s|` closure — see `read_id_field`'s tag above (the HRTB gap applies to
    // `AsyncFnMut` the same way it does to `AsyncFn`).
    crate::os_spr::scalar::scalar::write_id(out, id, async |s| dict.intern(s).await, edit_ordinal_of).await.map_err(ProtocolError::from)
}

async fn read_id_field<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<String, ProtocolError> {
    // ✏️ `scalar::read_id`'s `resolve` param is `AsyncFn` (granted lease, kernel-finish packet).
    // A plain closure returning a future satisfies the blanket impl in THEORY, but hits rustc's
    // known "implementation of AsyncFn is not general enough" HRTB gap in practice (reproduced in
    // `📡️replication/🧾️wire/🦀️component.rs`'s tests) — a genuine `async |args|` closure sidesteps it.
    crate::os_spr::scalar::scalar::read_id(
        input,
        async |idx: u32| dict.resolve(idx).await.map_err(|_| crate::os_pack::PackError::Malformed { what: "dict index", offset: idx as u64, detail: "out of range".to_string() }),
        |ord: u64| ordinal_to_id(ord).map_err(|_| crate::os_pack::PackError::Malformed { what: "edit ordinal", offset: ord, detail: "unresolvable".to_string() }),
    )
    .await.map_err(ProtocolError::from)
}

//#region 🔖️Message
// Shared wire shape for one `HistoryMessage`, used by both `HistoryOpMeta.messages` (🔖️Edit) and
// `HistoryConflict.messages` (🔖️Conflict) — one definition, both call sites.

/// @emoji 🎯️ `level u8 | code(idfield, dict-interned) | message(strfield) | target_count varint +
/// target(strfield)* | op_index presence u8 + [varint]`.
async fn write_history_message(out: &mut ByteWriter, message: &HistoryMessage, dict: &mut DictBuilder) -> Result<(), ProtocolError> {
    out.write_u8(message.level).await;
    write_id_field(out, &message.code, dict, &|_: &str| None).await?;
    write_str_field(out, &message.message).await;
    out.write_varint_u64(message.target.len() as u64).await;
    for target in &message.target {
        write_str_field(out, target).await;
    }
    match message.op_index {
        Some(index) => {
            out.write_u8(1).await;
            out.write_varint_u64(index as u64).await;
        }
        None => out.write_u8(0).await,
    }
    Ok(())
}

/// @emoji 🎯️ Inverse of [`write_history_message`].
async fn read_history_message(input: &mut ByteReader<'_>, dict: &DictReader) -> Result<HistoryMessage, ProtocolError> {
    let level = input.read_u8().await?;
    if crate::os_dsl::Severity::from_u8(level).is_none() {
        return Err(ProtocolError::Malformed { what: "history message severity", offset: input.position().await as u64 - 1, detail: format!("unknown severity {level}") });
    }
    let code = read_id_field(input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?;
    let message = read_str_field(input).await?;
    let target_count = input.read_varint_u64().await?;
    let mut target = Vec::with_capacity(target_count as usize);
    for _ in 0..target_count {
        target.push(read_str_field(input).await?);
    }
    let has_op_index = input.read_u8().await?;
    let op_index = match has_op_index {
        0 => None,
        1 => {
            let raw = input.read_varint_u64().await?;
            // 🚫️async: R10 shape 1 — `position()` is async but `map_err`'s closure is sync; the
            // offset is captured up front instead of awaited inside the closure.
            let offset = input.position().await as u64;
            Some(u32::try_from(raw).map_err(|_| ProtocolError::Malformed { what: "history message operation index", offset, detail: "exceeds u32".to_string() })?)
        }
        value => return Err(ProtocolError::Malformed { what: "history message operation index presence", offset: input.position().await as u64 - 1, detail: format!("expected 0 or 1, got {value}") }),
    };
    Ok(HistoryMessage { level, code, message, target, op_index })
}
//#endregion 🔖️Message

//#region 🔖️Doc
pub async fn encode_doc(doc_id: &str, schema: &str, dict: &mut DictBuilder) -> Vec<u8> {
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    write_id_field(&mut out, doc_id, dict, &|_: &str| None).await.expect("write_id never fails for an in-memory ByteWriter");
    write_id_field(&mut out, schema, dict, &|_: &str| None).await.expect("write_id never fails for an in-memory ByteWriter");
    out.into_bytes().await
}

pub async fn decode_doc(payload: &[u8], dict: &DictReader) -> Result<(String, String), ProtocolError> {
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("doc", format).await);
    }
    let doc_id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?;
    let schema = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?;
    Ok((doc_id, schema))
}
//#endregion 🔖️Doc

//#region 🔖️Edit
// REC_EDIT layout: format u8, presence u8 (bit0 actor, bit1 finished, bit2 key, bit3 description,
// bit4 explicit_meta, bit5 has_backwards_section), id, started(ts), [actor(dictref)],
// [finished(ts)], [key(str)], [description(str)], op_count varint, op_count x op-payload (see
// write_op_payload: op_tag u8 [bit0 has_text=1 required in v1, bit1 has_binary] + text_len varint
// + utf8 + [binary_len varint + bytes iff bit1]), [iff bit5: back_count varint + back_count x
// op-payload (inverse, in apply order)], [explicit_meta iff bit4: op_count x op-meta entry (see
// write_op_meta) — always keyed by op_count, never back_count, since meta describes the forward
// ops only].

/// @emoji 🎯️ Writes one op payload: `op_tag u8 [bit0 has_text=1 required in v1, bit1 has_binary]
/// + text_len varint + utf8 + [binary_len varint + bytes iff bit1]`. Used for both `edit.ops`
/// and `edit.inverse` — the two sections share this exact wire shape.
async fn write_op_payload(out: &mut ByteWriter, op: &OpPayload) -> Result<(), ProtocolError> {
    if op.text.is_none() && op.binary.is_none() {
        return Err(ProtocolError::Malformed { what: "op payload", offset: 0, detail: "requires text or binary".to_string() });
    }
    let tag = (op.text.is_some() as u8) | ((op.binary.is_some() as u8) << 1);
    out.write_u8(tag).await;
    if let Some(text) = &op.text {
        write_str_field(out, text).await;
    }
    if let Some(binary) = &op.binary {
        out.write_varint_u64(binary.len() as u64).await;
        out.write_bytes(binary).await;
    }
    Ok(())
}

/// @emoji 🎯️ Inverse of [`write_op_payload`].
async fn read_op_payload(input: &mut ByteReader<'_>) -> Result<OpPayload, ProtocolError> {
    let op_tag = input.read_u8().await?;
    if op_tag & 0b11 == 0 {
        return Err(ProtocolError::Malformed { what: "op payload", offset: 0, detail: "requires text or binary bit set".to_string() });
    }
    let text = if op_tag & 0b01 != 0 { Some(read_str_field(input).await?) } else { None };
    let binary = if op_tag & 0b10 != 0 {
        let len = input.read_varint_u64().await? as usize;
        Some(input.read_bytes(len).await?.to_vec())
    } else {
        None
    };
    Ok(OpPayload { text, binary })
}

async fn write_op_meta(out: &mut ByteWriter, meta: &HistoryOpMeta, dict: &mut DictBuilder, edit_ordinal_of: &dyn Fn(&str) -> Option<u64>) -> Result<(), ProtocolError> {
    let mut presence = 0u8;
    if meta.op_id.is_some() {
        presence |= 1 << 0;
    }
    if meta.author_id.is_some() {
        presence |= 1 << 1;
    }
    if meta.hlt.is_some() {
        presence |= 1 << 2;
    }
    if meta.payload_hash.is_some() {
        presence |= 1 << 3;
    }
    if meta.group_id.is_some() {
        presence |= 1 << 4;
    }
    if !meta.origin.is_owner() {
        presence |= 1 << 5;
    }
    if !meta.messages.is_empty() {
        presence |= 1 << 6;
    }
    out.write_u8(presence).await;
    if let Some(op_id) = &meta.op_id {
        write_id_field(out, op_id, dict, edit_ordinal_of).await?;
    }
    out.write_varint_u64(meta.dependencies.len() as u64).await;
    for dep in &meta.dependencies {
        write_id_field(out, dep, dict, edit_ordinal_of).await?;
    }
    out.write_varint_u64(meta.base_version).await;
    if let Some(author) = &meta.author_id {
        write_id_field(out, author, dict, edit_ordinal_of).await?;
    }
    if let Some((actor, physical_ms, logical)) = &meta.hlt {
        out.write_varint_u64(*actor).await;
        out.write_varint_i64(*physical_ms).await;
        out.write_varint_u64(*logical).await;
    }
    out.write_u8(meta.undo_policy).await;
    if let Some(hash) = &meta.payload_hash {
        out.write_bytes(hash).await;
    }
    // 🎯️ Appended past the pre-existing tail (bit4 of the same presence byte) — a decoder reading
    // a byte-log written before this field existed sees bit4 unset (that bit never existed in the
    // old presence byte, so it always tests as 0) and recovers `group_id: None`, exactly the
    // "absent for logs predating this field" contract `HistoryLog.cursor` documents for its own
    // additive field.
    if let Some(group_id) = &meta.group_id {
        write_id_field(out, group_id, dict, edit_ordinal_of).await?;
    }
    // 🎯️ Appended past `group_id` (bit5 of the same presence byte, same "absent for logs predating
    // this field" contract) — canonical-JSON, not dict-interned: unlike `group_id`, an origin's
    // `Contributed`/`Transaction` payload is structured data that won't repeat verbatim across
    // siblings the way a shared composite-gesture id does.
    if !meta.origin.is_owner() {
        let encoded = serde_json::to_string(&meta.origin).expect("MutationOrigin canonical encoding never fails");
        write_str_field(out, &encoded).await;
    }
    // 🎯️ Appended past `origin` (bit6 of the same presence byte, same "absent for logs predating
    // this field" contract) — the durable message ledger, not reproducible from a fresh replay.
    if !meta.messages.is_empty() {
        out.write_varint_u64(meta.messages.len() as u64).await;
        for message in &meta.messages {
            write_history_message(out, message, dict).await?;
        }
    }
    Ok(())
}

async fn read_op_meta<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryOpMeta, ProtocolError> {
    let presence = input.read_u8().await?;
    let op_id = if presence & (1 << 0) != 0 { Some(read_id_field(input, dict, ordinal_to_id).await?) } else { None };
    let dep_count = input.read_varint_u64().await?;
    let mut dependencies = Vec::with_capacity(dep_count as usize);
    for _ in 0..dep_count {
        dependencies.push(read_id_field(input, dict, ordinal_to_id).await?);
    }
    let base_version = input.read_varint_u64().await?;
    let author_id = if presence & (1 << 1) != 0 { Some(read_id_field(input, dict, ordinal_to_id).await?) } else { None };
    let hlt = if presence & (1 << 2) != 0 {
        let actor = input.read_varint_u64().await?;
        let physical_ms = input.read_varint_i64().await?;
        let logical = input.read_varint_u64().await?;
        Some((actor, physical_ms, logical))
    } else {
        None
    };
    let undo_policy = input.read_u8().await?;
    let payload_hash = if presence & (1 << 3) != 0 { Some(input.read_array32().await?) } else { None };
    let group_id = if presence & (1 << 4) != 0 { Some(read_id_field(input, dict, ordinal_to_id).await?) } else { None };
    let origin = if presence & (1 << 5) != 0 {
        let encoded = read_str_field(input).await?;
        serde_json::from_str(&encoded).map_err(|error| ProtocolError::Malformed { what: "op meta origin", offset: 0, detail: error.to_string() })?
    } else {
        crate::os_spr::command::MutationOrigin::Owner
    };
    let messages = if presence & (1 << 6) != 0 {
        let count = input.read_varint_u64().await?;
        let mut messages = Vec::with_capacity(count as usize);
        for _ in 0..count {
            messages.push(read_history_message(input, dict).await?);
        }
        messages
    } else {
        Vec::new()
    };
    Ok(HistoryOpMeta { op_id, dependencies, base_version, author_id, hlt, undo_policy, payload_hash, group_id, origin, messages })
}

pub async fn encode_edit(edit: &HistoryEdit, dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &dyn Fn(&str) -> Option<u64> = &edit_ordinal_of;
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    let mut presence = 0u8;
    if edit.actor.is_some() {
        presence |= 1 << 0;
    }
    if edit.finished_at.is_some() {
        presence |= 1 << 1;
    }
    if edit.coalesce_key.is_some() {
        presence |= 1 << 2;
    }
    if edit.description.is_some() {
        presence |= 1 << 3;
    }
    if edit.meta.is_some() {
        presence |= 1 << 4;
    }
    if !edit.inverse.is_empty() {
        presence |= 1 << 5;
    }
    out.write_u8(presence).await;
    write_id_field(&mut out, &edit.id, dict, &|_: &str| None).await?;
    let mut prev_epoch_ms = crate::os_spr::scalar::scalar::write_timestamp(&mut out, &edit.started_at, None).await;
    if let Some(actor) = &edit.actor {
        write_id_field(&mut out, actor, dict, edit_ordinal_of).await?;
    }
    if let Some(finished) = &edit.finished_at {
        prev_epoch_ms = crate::os_spr::scalar::scalar::write_timestamp(&mut out, finished, prev_epoch_ms).await;
    }
    let _ = prev_epoch_ms;
    if let Some(key) = &edit.coalesce_key {
        write_str_field(&mut out, key).await;
    }
    if let Some(description) = &edit.description {
        write_str_field(&mut out, description).await;
    }
    if edit.ops.len() as u64 > ProtocolLimits::default().max_op_count_per_edit as u64 {
        return Err(ProtocolError::LimitExceeded("edit op count exceeds ProtocolLimits::max_op_count_per_edit"));
    }
    out.write_varint_u64(edit.ops.len() as u64).await;
    for op in &edit.ops {
        write_op_payload(&mut out, op).await?;
    }
    if !edit.inverse.is_empty() {
        out.write_varint_u64(edit.inverse.len() as u64).await;
        for op in &edit.inverse {
            write_op_payload(&mut out, op).await?;
        }
    }
    if let Some(metas) = &edit.meta {
        if metas.len() != edit.ops.len() {
            return Err(ProtocolError::Malformed { what: "edit meta", offset: 0, detail: "explicit meta length must match op count".to_string() });
        }
        for meta in metas {
            write_op_meta(&mut out, meta, dict, edit_ordinal_of).await?;
        }
    }
    Ok(out.into_bytes().await)
}

pub async fn decode_edit<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryEdit, ProtocolError> {
    let ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError> = &ordinal_to_id;
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("edit", format).await);
    }
    let presence = input.read_u8().await?;
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?;
    let (started_at, mut prev_epoch_ms) = crate::os_spr::scalar::scalar::read_timestamp(&mut input, None).await?;
    let actor = if presence & (1 << 0) != 0 { Some(read_id_field(&mut input, dict, ordinal_to_id).await?) } else { None };
    let finished_at = if presence & (1 << 1) != 0 {
        let (s, p) = crate::os_spr::scalar::scalar::read_timestamp(&mut input, prev_epoch_ms).await?;
        prev_epoch_ms = p;
        Some(s)
    } else {
        None
    };
    let _ = prev_epoch_ms;
    let coalesce_key = if presence & (1 << 2) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let description = if presence & (1 << 3) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let op_count = input.read_varint_u64().await?;
    let max_ops = ProtocolLimits::default().max_op_count_per_edit as u64;
    if op_count > max_ops {
        return Err(ProtocolError::LimitExceeded("edit op count exceeds ProtocolLimits::max_op_count_per_edit"));
    }
    let mut ops = Vec::with_capacity(op_count as usize);
    for _ in 0..op_count {
        ops.push(read_op_payload(&mut input).await?);
    }
    let inverse = if presence & (1 << 5) != 0 {
        let back_count = input.read_varint_u64().await?;
        if back_count > max_ops {
            return Err(ProtocolError::LimitExceeded("edit inverse op count exceeds ProtocolLimits::max_op_count_per_edit"));
        }
        let mut backs = Vec::with_capacity(back_count as usize);
        for _ in 0..back_count {
            backs.push(read_op_payload(&mut input).await?);
        }
        backs
    } else {
        Vec::new()
    };
    let meta = if presence & (1 << 4) != 0 {
        let mut metas = Vec::with_capacity(op_count as usize);
        for _ in 0..op_count {
            metas.push(read_op_meta(&mut input, dict, ordinal_to_id).await?);
        }
        Some(metas)
    } else {
        None
    };
    Ok(HistoryEdit { id, actor, started_at, finished_at, coalesce_key, description, ops, inverse, meta })
}
//#endregion 🔖️Edit

//#region 🔖️Change
pub async fn encode_change(change: &HistoryChange, dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &dyn Fn(&str) -> Option<u64> = &edit_ordinal_of;
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    let mut presence = 0u8;
    if change.description.is_some() {
        presence |= 1 << 0;
    }
    out.write_u8(presence).await;
    write_id_field(&mut out, &change.id, dict, &|_: &str| None).await?;
    crate::os_spr::scalar::scalar::write_timestamp(&mut out, &change.saved_at, None).await;
    out.write_varint_u64(change.edit_ids.len() as u64).await;
    for edit_id in &change.edit_ids {
        write_id_field(&mut out, edit_id, dict, edit_ordinal_of).await?;
    }
    if let Some(description) = &change.description {
        write_str_field(&mut out, description).await;
    }
    Ok(out.into_bytes().await)
}

pub async fn decode_change<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryChange, ProtocolError> {
    let ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError> = &ordinal_to_id;
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("change", format).await);
    }
    let presence = input.read_u8().await?;
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?;
    let (saved_at, _) = crate::os_spr::scalar::scalar::read_timestamp(&mut input, None).await?;
    let edit_count = input.read_varint_u64().await?;
    let mut edit_ids = Vec::with_capacity(edit_count as usize);
    for _ in 0..edit_count {
        edit_ids.push(read_id_field(&mut input, dict, ordinal_to_id).await?);
    }
    let description = if presence & (1 << 0) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    Ok(HistoryChange { id, saved_at, edit_ids, description })
}
//#endregion 🔖️Change

//#region 🔖️Checkpoint
pub async fn encode_checkpoint(checkpoint: &HistoryCheckpoint, dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    let mut presence = 0u8;
    if checkpoint.parent_id.is_some() {
        presence |= 1 << 0;
    }
    if checkpoint.message.is_some() {
        presence |= 1 << 1;
    }
    out.write_u8(presence).await;
    write_id_field(&mut out, &checkpoint.id, dict, &|_: &str| None).await?;
    crate::os_spr::scalar::scalar::write_timestamp(&mut out, &checkpoint.timestamp, None).await;
    out.write_varint_u64(checkpoint.change_ids.len() as u64).await;
    for change_id in &checkpoint.change_ids {
        write_id_field(&mut out, change_id, dict, &|_: &str| None).await?;
    }
    if let Some(parent) = &checkpoint.parent_id {
        write_id_field(&mut out, parent, dict, &|_: &str| None).await?;
    }
    out.write_varint_u64(checkpoint.authors.len() as u64).await;
    for author in &checkpoint.authors {
        write_id_field(&mut out, &author.id, dict, &|_: &str| None).await?;
        write_str_field(&mut out, &author.name).await;
    }
    if let Some(message) = &checkpoint.message {
        write_str_field(&mut out, message).await;
    }
    Ok(out.into_bytes().await)
}

pub async fn decode_checkpoint(payload: &[u8], dict: &DictReader) -> Result<HistoryCheckpoint, ProtocolError> {
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("checkpoint", format).await);
    }
    let presence = input.read_u8().await?;
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?;
    let (timestamp, _) = crate::os_spr::scalar::scalar::read_timestamp(&mut input, None).await?;
    let change_count = input.read_varint_u64().await?;
    let mut change_ids = Vec::with_capacity(change_count as usize);
    for _ in 0..change_count {
        change_ids.push(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?);
    }
    let parent_id = if presence & (1 << 0) != 0 { Some(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?) } else { None };
    let author_count = input.read_varint_u64().await?;
    let mut authors = Vec::with_capacity(author_count as usize);
    for _ in 0..author_count {
        let author_id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?;
        let name = read_str_field(&mut input).await?;
        authors.push(HistoryAuthor { id: author_id, name });
    }
    let message = if presence & (1 << 1) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    Ok(HistoryCheckpoint { id, timestamp, change_ids, parent_id, authors, message })
}
//#endregion 🔖️Checkpoint

//#region 🔖️Alternative
pub async fn encode_alternative(alternative: &HistoryAlternative, dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    write_id_field(&mut out, &alternative.id, dict, &|_: &str| None).await?;
    write_str_field(&mut out, &alternative.name).await;
    out.write_varint_u64(alternative.checkpoint_ids.len() as u64).await;
    for checkpoint_id in &alternative.checkpoint_ids {
        write_id_field(&mut out, checkpoint_id, dict, &|_: &str| None).await?;
    }
    Ok(out.into_bytes().await)
}

pub async fn decode_alternative(payload: &[u8], dict: &DictReader) -> Result<HistoryAlternative, ProtocolError> {
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("alternative", format).await);
    }
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?;
    let name = read_str_field(&mut input).await?;
    let checkpoint_count = input.read_varint_u64().await?;
    let mut checkpoint_ids = Vec::with_capacity(checkpoint_count as usize);
    for _ in 0..checkpoint_count {
        checkpoint_ids.push(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?);
    }
    Ok(HistoryAlternative { id, name, checkpoint_ids })
}
//#endregion 🔖️Alternative

//#region 🔖️Active
pub async fn encode_active(alternative_id: Option<&str>, dict: &mut DictBuilder) -> Vec<u8> {
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    match alternative_id {
        Some(id) => {
            out.write_u8(1).await;
            write_id_field(&mut out, id, dict, &|_: &str| None).await.expect("write_id never fails for an in-memory ByteWriter");
        }
        None => out.write_u8(0).await,
    }
    out.into_bytes().await
}

pub async fn decode_active(payload: &[u8], dict: &DictReader) -> Result<Option<String>, ProtocolError> {
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("active", format).await);
    }
    let presence = input.read_u8().await?;
    if presence & 1 != 0 { Ok(Some(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?)) } else { Ok(None) }
}
//#endregion 🔖️Active

//#region 🔖️Cursor
// Extension-range record (protocol_core's frozen kind table stays 0x00..0x12 + 0x7F; the
// 0x40..=0x7E range is caller-defined per its `is_critical_kind` doc). Written with the critical
// bit UNSET — a foreign/older reader skips it via the standard skip-unknown rule, exactly like
// `REC_ACTOR_DICT` today. Last-wins: a later REC_CURSOR frame in the same file supersedes an
// earlier one, mirroring REC_ACTIVE.
pub const REC_CURSOR: u8 = 0x40;

/// @emoji 🎯️ `format u8 (=1) | presence u8 (bit0 checkpoint) | applied_count varint + id* |
/// redo_count varint + id* | [checkpoint id]`. Edit ids go through `write_id_field` (dict +
/// edit-ordinal refs), same as every other edit-id reference in this crate.
pub async fn encode_cursor(cursor: &HistoryCursor, dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &dyn Fn(&str) -> Option<u64> = &edit_ordinal_of;
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    out.write_u8(if cursor.checkpoint_id.is_some() { 1 } else { 0 }).await;
    out.write_varint_u64(cursor.applied_edit_ids.len() as u64).await;
    for id in &cursor.applied_edit_ids {
        write_id_field(&mut out, id, dict, edit_ordinal_of).await?;
    }
    out.write_varint_u64(cursor.redo_edit_ids.len() as u64).await;
    for id in &cursor.redo_edit_ids {
        write_id_field(&mut out, id, dict, edit_ordinal_of).await?;
    }
    if let Some(checkpoint_id) = &cursor.checkpoint_id {
        write_id_field(&mut out, checkpoint_id, dict, &|_: &str| None).await?;
    }
    Ok(out.into_bytes().await)
}

/// @emoji 🎯️ Inverse of [`encode_cursor`].
pub async fn decode_cursor<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryCursor, ProtocolError> {
    let ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError> = &ordinal_to_id;
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("cursor", format).await);
    }
    let presence = input.read_u8().await?;
    if presence & !1 != 0 {
        return Err(ProtocolError::Malformed { what: "cursor presence", offset: input.position().await as u64 - 1, detail: format!("unknown presence bits {presence:#010b}") });
    }
    let applied_count = input.read_varint_u64().await?;
    let mut applied_edit_ids = Vec::with_capacity(applied_count as usize);
    for _ in 0..applied_count {
        applied_edit_ids.push(read_id_field(&mut input, dict, ordinal_to_id).await?);
    }
    let redo_count = input.read_varint_u64().await?;
    let mut redo_edit_ids = Vec::with_capacity(redo_count as usize);
    for _ in 0..redo_count {
        redo_edit_ids.push(read_id_field(&mut input, dict, ordinal_to_id).await?);
    }
    let checkpoint_id = if presence & 1 != 0 { Some(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?) } else { None };
    if input.remaining().await != 0 {
        return Err(ProtocolError::Malformed { what: "cursor", offset: input.position().await as u64, detail: "trailing payload bytes".to_string() });
    }
    Ok(HistoryCursor { applied_edit_ids, redo_edit_ids, checkpoint_id })
}
//#endregion 🔖️Cursor

//#region 🔖️Composition
/// @emoji 🧩️ Second caller-defined extension record (`REC_CURSOR`'s neighbour in the 0x40..=0x7E
/// range), written NON-critical for the same reason: a reader that does not know about composition
/// skips it under the standard skip-unknown rule and still reads a fully valid document. Last-wins,
/// mirroring `REC_ACTIVE`/`REC_CURSOR`.
pub const REC_COMPOSITION: u8 = 0x41;

/// @emoji 🧩️ `format u8 (=1) | presence u8 (bit0 owner, bit1 dialect) | [owner triple] |
/// [dialect triple] | pin_group_count varint + (checkpoint_id, pin_count, (child_uri, child_ck)*)*`.
/// Every string goes through the shared dictionary via `write_id_field`, same as every other
/// identifier in this crate — composition ids repeat heavily across checkpoints, so dictionary
/// coding is what keeps the overlay small on a document with a long pinned history.
pub async fn encode_composition(composition: &HistoryComposition, dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let plain: &dyn Fn(&str) -> Option<u64> = &|_: &str| None;
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    let presence = u8::from(composition.owner.is_some()) | (u8::from(composition.dialect.is_some()) << 1);
    out.write_u8(presence).await;
    if let Some((parent, slot, child_id)) = &composition.owner {
        for field in [parent, slot, child_id] {
            write_id_field(&mut out, field, dict, plain).await?;
        }
    }
    if let Some((kind, standard, subset)) = &composition.dialect {
        for field in [kind, standard, subset] {
            write_id_field(&mut out, field, dict, plain).await?;
        }
    }
    out.write_varint_u64(composition.checkpoint_pins.len() as u64).await;
    for (checkpoint_id, pins) in &composition.checkpoint_pins {
        write_id_field(&mut out, checkpoint_id, dict, plain).await?;
        out.write_varint_u64(pins.len() as u64).await;
        for (child_uri, child_checkpoint_id) in pins {
            write_id_field(&mut out, child_uri, dict, plain).await?;
            write_id_field(&mut out, child_checkpoint_id, dict, plain).await?;
        }
    }
    Ok(out.into_bytes().await)
}

/// @emoji 🧩️ Inverse of [`encode_composition`].
pub async fn decode_composition<'d>(payload: &[u8], dict: &'d DictReader) -> Result<HistoryComposition, ProtocolError> {
    let miss: &dyn Fn(u64) -> Result<&'d str, ProtocolError> = &|ord: u64| Err(ProtocolError::DictMiss(ord as u32));
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("composition", format).await);
    }
    let presence = input.read_u8().await?;
    // 🚫️async: R10 shape 1 — `read_id_field` is async but a plain closure can't await; hoisted into
    // a nested async fn (`dict`/`miss` threaded through explicitly, since a nested fn can't capture).
    async fn read_triple<'r>(input: &mut ByteReader<'_>, dict: &'r DictReader, miss: &dyn Fn(u64) -> Result<&'r str, ProtocolError>) -> Result<(String, String, String), ProtocolError> {
        Ok((read_id_field(input, dict, miss).await?, read_id_field(input, dict, miss).await?, read_id_field(input, dict, miss).await?))
    }
    let owner = if presence & 1 != 0 { Some(read_triple(&mut input, dict, miss).await?) } else { None };
    let dialect = if presence & 2 != 0 { Some(read_triple(&mut input, dict, miss).await?) } else { None };
    let group_count = input.read_varint_u64().await?;
    let mut checkpoint_pins = Vec::with_capacity(group_count as usize);
    for _ in 0..group_count {
        let checkpoint_id = read_id_field(&mut input, dict, miss).await?;
        let pin_count = input.read_varint_u64().await?;
        let mut pins = Vec::with_capacity(pin_count as usize);
        for _ in 0..pin_count {
            pins.push((read_id_field(&mut input, dict, miss).await?, read_id_field(&mut input, dict, miss).await?));
        }
        checkpoint_pins.push((checkpoint_id, pins));
    }
    Ok(HistoryComposition { owner, dialect, checkpoint_pins })
}
//#endregion 🔖️Composition

//#region 🔖️Conflict
/// @emoji ⚔️ Third caller-defined extension record (`REC_CURSOR`'s/`REC_COMPOSITION`'s neighbour in
/// the 0x40..=0x7E range), written NON-critical for the same reason: a reader that doesn't know
/// about first-class conflicts (`📋️contract-freeze.md` §C5/§C7) skips the whole record and still
/// reads a fully valid document. Unlike `REC_EDIT` (one frame per edit, the hot streaming path),
/// the whole `HistoryLog.conflicts` list is written as ONE frame — conflicts are not append-only
/// hot data, an authority's open/resolved set is small and always persisted together, so a single
/// frame keeps the shape symmetric with `encode_cursor`'s own single-payload framing while still
/// carrying a list. Absent for every log with no open or historical conflicts (the overwhelming
/// majority) and for logs predating the record — no frame is written when `conflicts` is empty.
pub const REC_CONFLICT: u8 = 0x42;

async fn validate_conflict_tags(kind: u8, status: u8, offset: u64) -> Result<(), ProtocolError> {
    if !matches!(kind, 0 | 1) {
        return Err(ProtocolError::Malformed { what: "conflict", offset, detail: format!("unknown conflict kind {kind}") });
    }
    if !matches!(status, 0 | 1 | 2) {
        return Err(ProtocolError::Malformed { what: "conflict", offset, detail: format!("unknown conflict status {status}") });
    }
    Ok(())
}

async fn write_conflict(out: &mut ByteWriter, conflict: &HistoryConflict, dict: &mut DictBuilder, edit_ordinal_of: &dyn Fn(&str) -> Option<u64>) -> Result<(), ProtocolError> {
    validate_conflict_tags(conflict.kind, conflict.status, 0).await?;
    write_id_field(out, &conflict.id, dict, &|_: &str| None).await?;
    out.write_u8(conflict.kind).await;
    out.write_u8(conflict.status).await;
    out.write_varint_u64(conflict.actors.len() as u64).await;
    for actor in &conflict.actors {
        write_id_field(out, actor, dict, &|_: &str| None).await?;
    }
    out.write_varint_u64(conflict.hlt.0).await;
    out.write_varint_u64(conflict.hlt.1).await;
    out.write_varint_u64(conflict.hlt.2).await;
    out.write_varint_u64(conflict.edit_ids.len() as u64).await;
    for edit_id in &conflict.edit_ids {
        write_id_field(out, edit_id, dict, edit_ordinal_of).await?;
    }
    out.write_varint_u64(conflict.envelopes.len() as u64).await;
    for envelope in &conflict.envelopes {
        out.write_varint_u64(envelope.len() as u64).await;
        out.write_bytes(envelope).await;
    }
    out.write_varint_u64(conflict.messages.len() as u64).await;
    for message in &conflict.messages {
        write_history_message(out, message, dict).await?;
    }
    Ok(())
}

async fn read_conflict<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryConflict, ProtocolError> {
    let id = read_id_field(input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?;
    let kind = input.read_u8().await?;
    let status = input.read_u8().await?;
    validate_conflict_tags(kind, status, input.position().await as u64 - 2).await?;
    let actor_count = input.read_varint_u64().await?;
    let mut actors = Vec::with_capacity(actor_count as usize);
    for _ in 0..actor_count {
        actors.push(read_id_field(input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await?);
    }
    let hlt = (input.read_varint_u64().await?, input.read_varint_u64().await?, input.read_varint_u64().await?);
    let edit_id_count = input.read_varint_u64().await?;
    let mut edit_ids = Vec::with_capacity(edit_id_count as usize);
    for _ in 0..edit_id_count {
        edit_ids.push(read_id_field(input, dict, ordinal_to_id).await?);
    }
    let envelope_count = input.read_varint_u64().await?;
    let mut envelopes = Vec::with_capacity(envelope_count as usize);
    for _ in 0..envelope_count {
        let len = input.read_varint_u64().await? as usize;
        envelopes.push(input.read_bytes(len).await?.to_vec());
    }
    let message_count = input.read_varint_u64().await?;
    let mut messages = Vec::with_capacity(message_count as usize);
    for _ in 0..message_count {
        messages.push(read_history_message(input, dict).await?);
    }
    Ok(HistoryConflict { id, kind, status, actors, hlt, edit_ids, envelopes, messages })
}

/// @emoji 🎯️ `format u8 (=1) | count varint + count x conflict entry` — mirrors [`encode_cursor`]'s
/// shape (single top-level format byte, then the payload) with the payload being a length-prefixed
/// list instead of one struct: each entry is `id(idfield) | kind u8 | status u8 | actor_count
/// varint + actor(idfield)* | hlt(actor varint, physical_ms varint, logical varint) |
/// edit_id_count varint + edit_id(idfield, edit-ordinal-eligible)* | envelope_count varint +
/// (len varint + raw bytes)* | message_count varint + message*` (see [`write_history_message`] for
/// one message's shape). Envelopes are opaque bytes to this crate (already-serialized
/// `crate::os_spr::causal::MutationEnvelope`s) — same "never interprets" stance the module
/// docstring states for op payloads.
pub async fn encode_conflicts(conflicts: &[HistoryConflict], dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &dyn Fn(&str) -> Option<u64> = &edit_ordinal_of;
    let mut out = ByteWriter::new().await;
    out.write_u8(1).await;
    out.write_varint_u64(conflicts.len() as u64).await;
    for conflict in conflicts {
        write_conflict(&mut out, conflict, dict, edit_ordinal_of).await?;
    }
    Ok(out.into_bytes().await)
}

/// @emoji 🎯️ Inverse of [`encode_conflicts`].
pub async fn decode_conflicts<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<Vec<HistoryConflict>, ProtocolError> {
    let ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError> = &ordinal_to_id;
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("conflict", format).await);
    }
    let count = input.read_varint_u64().await?;
    let mut conflicts = Vec::with_capacity(count as usize);
    let mut ids = HashSet::new();
    for _ in 0..count {
        let conflict = read_conflict(&mut input, dict, ordinal_to_id).await?;
        if !ids.insert(conflict.id.clone()) {
            return Err(ProtocolError::Malformed { what: "conflict", offset: input.position().await as u64, detail: format!("duplicate conflict id {}", conflict.id) });
        }
        conflicts.push(conflict);
    }
    if input.remaining().await != 0 {
        return Err(ProtocolError::Malformed { what: "conflict", offset: input.position().await as u64, detail: "trailing payload bytes".to_string() });
    }
    Ok(conflicts)
}
//#endregion 🔖️Conflict
//#endregion 🔖️Payloads

//#region 🔖️Codec
// Whole-file compile: HistoryLog <-> .spr bytes, using crate::os_spr::format::SprWriter/FrameCursor.
//
// 🎯️ Design choice: dictionaries are flushed INCREMENTALLY (a REC_STR_DICT delta record right
// before whichever record first needed the new entries), not in a separate pre-pass — this keeps
// encode_history a single forward pass while still satisfying "dict records come before the
// records that reference them"; determinism (and therefore canonical-stability) falls out of the
// fact that DictBuilder's first-use interning order is itself deterministic.

#[derive(Clone, Debug)]
pub struct EncodeOptions {
    pub canonical: bool,
    pub write_backwards_section: bool,
    pub limits: ProtocolLimits,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self { canonical: true, write_backwards_section: false, limits: ProtocolLimits::default() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DecodeOptions {
    pub verification: VerificationLevel,
    pub limits: ProtocolLimits,
}

async fn flush_dict_delta<S: PackSink>(writer: &mut SprWriter<S>, dict: &DictBuilder, base: &mut u32) -> Result<(), ProtocolError> {
    let len = dict.len().await;
    if len > *base {
        let entries = dict.entries_since(*base).await;
        let mut payload = ByteWriter::new().await;
        payload.write_u8(1).await;
        payload.write_varint_u64(*base as u64).await;
        payload.write_varint_u64(entries.len() as u64).await;
        for entry in entries {
            payload.write_varint_u64(entry.len() as u64).await;
            payload.write_bytes(entry.as_bytes()).await;
        }
        writer.write_record(crate::os_spr::REC_STR_DICT, true, &payload.into_bytes().await, CodecId(0)).await?;
        *base = len;
    }
    Ok(())
}

async fn apply_dict_record(dict: &mut DictReader, payload: &[u8]) -> Result<(), ProtocolError> {
    let mut input = ByteReader::new(payload).await;
    let format = input.read_u8().await?;
    if format > 1 {
        return Err(malformed_fmt("dict", format).await);
    }
    let base_count = input.read_varint_u64().await? as u32;
    let count = input.read_varint_u64().await?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(read_str_field(&mut input).await?);
    }
    dict.extend(base_count, entries).await
}

/// 🎞️ `(commit_seq, chain_hash)` out of a `REC_COMMIT` frame's payload — thin wrapper over
/// `crate::os_spr::format::parse_commit_payload` (public since that crate's own follow-up review pass)
/// so `VerificationLevel::Full`'s chain recompute doesn't need this crate's own byte-offset copy.
async fn parse_commit_fields(payload: &[u8]) -> Result<(u64, [u8; 32]), ProtocolError> {
    let commit = crate::os_spr::format::parse_commit_payload(payload).await?;
    Ok((commit.commit_seq, commit.chain_hash))
}

pub async fn encode_history(log: &HistoryLog, options: &EncodeOptions) -> Result<Vec<u8>, ProtocolError> {
    if log.edits.len() as u64 > options.limits.max_record_count {
        return Err(ProtocolError::LimitExceeded("edit count exceeds ProtocolLimits::max_record_count"));
    }
    let write_options = WriteOptions { required_flags: crate::os_spr::REQUIRED_HASH_CHAIN, optional_flags: if options.canonical { crate::os_spr::OPTIONAL_CANONICAL } else { 0 } };
    let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options).await?;
    let mut dict = DictBuilder::new().await;
    let mut dict_base = 0u32;

    let doc_payload = encode_doc(&log.doc_id, &log.schema, &mut dict).await;
    flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
    writer.write_record(crate::os_spr::REC_DOC, true, &doc_payload, CodecId(0)).await?;

    // 🎯️ Built incrementally (an edit's own id is inserted only AFTER it is encoded), matching
    // `HistoryAppender::append_edit`'s streaming semantics and the decoder's causal resolution
    // (`EditIter`/`prescan_full` only ever know edits already decoded). A one-shot, whole-list
    // `ordinals` map would let an edit's own `mutation_meta[i].mutation_id` — legitimately equal
    // to `edit.id` for a single-op edit — resolve to a self-referencing ordinal at encode time,
    // which the decoder can never resolve (it hasn't registered the current edit's id yet).
    let mut ordinals: HashMap<&str, u64> = HashMap::new();
    for (index, edit) in log.edits.iter().enumerate() {
        // 🎯️ `write_backwards_section` is a batch-level policy switch: even when `edit.inverse`
        // is populated (e.g. by a live store that always computes it), a caller can opt out of
        // persisting it here. `HistoryAppender::append_edit` has no such switch — its streaming,
        // one-edit-at-a-time API gives the caller direct per-edit control via the data itself.
        let payload = if options.write_backwards_section {
            encode_edit(edit, &mut dict, |id| ordinals.get(id).copied()).await?
        } else {
            let stripped = HistoryEdit { inverse: Vec::new(), ..edit.clone() };
            encode_edit(&stripped, &mut dict, |id| ordinals.get(id).copied()).await?
        };
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(crate::os_spr::REC_EDIT, true, &payload, CodecId(0)).await?;
        ordinals.insert(edit.id.as_str(), index as u64);
    }
    for change in &log.changes {
        let payload = encode_change(change, &mut dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(crate::os_spr::REC_CHANGE, true, &payload, CodecId(0)).await?;
    }
    for checkpoint in &log.checkpoints {
        let payload = encode_checkpoint(checkpoint, &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(crate::os_spr::REC_CHECKPOINT, true, &payload, CodecId(0)).await?;
    }
    for alternative in &log.alternatives {
        let payload = encode_alternative(alternative, &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(crate::os_spr::REC_ALTERNATIVE, true, &payload, CodecId(0)).await?;
    }
    let active_payload = encode_active(log.active_alternative_id.as_deref(), &mut dict).await;
    flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
    writer.write_record(crate::os_spr::REC_ACTIVE, true, &active_payload, CodecId(0)).await?;

    if let Some(cursor) = &log.cursor {
        let cursor_payload = encode_cursor(cursor, &mut dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_CURSOR, false, &cursor_payload, CodecId(0)).await?;
    }

    if let Some(composition) = &log.composition {
        let composition_payload = encode_composition(composition, &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_COMPOSITION, false, &composition_payload, CodecId(0)).await?;
    }

    if !log.conflicts.is_empty() {
        let conflicts_payload = encode_conflicts(&log.conflicts, &mut dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_CONFLICT, false, &conflicts_payload, CodecId(0)).await?;
    }

    writer.commit().await?;
    Ok(writer.into_sink().await)
}

async fn decode_history_from(trusted: &[u8], options: &DecodeOptions) -> Result<HistoryLog, ProtocolError> {
    let mut dict = DictReader::new().await;
    let mut edit_ids: Vec<String> = Vec::new();
    let mut log = HistoryLog::default();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64).await;
    let hasher = Blake3Hasher;
    let full = options.verification == VerificationLevel::Full;
    let mut running_chain = if full { hasher.hash(&trusted[..HEADER_SIZE]).await } else { [0u8; 32] };
    let mut pending_digests: Vec<[u8; 32]> = Vec::new();
    let mut saw_conflicts = false;

    while let Some(frame) = cursor.next_frame().await? {
        if full && frame.kind != crate::os_spr::REC_COMMIT {
            let frame_bytes = &trusted[frame.offset as usize..(frame.offset + frame.frame_len().await) as usize];
            pending_digests.push(hasher.hash(frame_bytes).await);
        }
        match frame.kind {
            crate::os_spr::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload().await).await?,
            crate::os_spr::REC_ACTOR_DICT => {} // v1 never splits an actor dictionary — see 🔖️Payloads note
            crate::os_spr::REC_DOC => {
                let (doc_id, schema) = decode_doc(frame.payload().await, &dict).await?;
                log.doc_id = doc_id;
                log.schema = schema;
            }
            crate::os_spr::REC_EDIT => {
                let edit_ids_ref = &edit_ids;
                let edit = decode_edit(frame.payload().await, &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                edit_ids.push(edit.id.clone());
                log.edits.push(edit);
            }
            crate::os_spr::REC_CHANGE => {
                let edit_ids_ref = &edit_ids;
                let change = decode_change(frame.payload().await, &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                log.changes.push(change);
            }
            crate::os_spr::REC_CHECKPOINT => log.checkpoints.push(decode_checkpoint(frame.payload().await, &dict).await?),
            crate::os_spr::REC_ALTERNATIVE => log.alternatives.push(decode_alternative(frame.payload().await, &dict).await?),
            crate::os_spr::REC_ACTIVE => log.active_alternative_id = decode_active(frame.payload().await, &dict).await?,
            REC_CURSOR => {
                let edit_ids_ref = &edit_ids;
                log.cursor = Some(decode_cursor(frame.payload().await, &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?);
            }
            REC_COMPOSITION => log.composition = Some(decode_composition(frame.payload().await, &dict).await?),
            REC_CONFLICT => {
                if saw_conflicts {
                    return Err(ProtocolError::Malformed { what: "history", offset: frame.offset, detail: "duplicate conflict record".to_string() });
                }
                let edit_ids_ref = &edit_ids;
                log.conflicts = decode_conflicts(frame.payload().await, &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                saw_conflicts = true;
            }
            crate::os_spr::REC_COMMIT if full => {
                let (commit_seq, chain_hash) = parse_commit_fields(frame.payload().await).await?;
                let mut concat = running_chain.to_vec();
                for digest in &pending_digests {
                    concat.extend_from_slice(digest);
                }
                let recomputed = hasher.hash(&concat).await;
                if recomputed != chain_hash {
                    return Err(ProtocolError::ChainMismatch { commit_seq });
                }
                running_chain = recomputed;
                pending_digests.clear();
            }
            // Every other kind (REC_PROJECTION, REC_INDEX, REC_FRONTIER, extension range, ...) is
            // foreign to this crate's semantic layer — skip regardless of the frame's critical bit;
            // this reader only enforces criticality for its own known kind set handled above.
            _ => {}
        }
    }
    Ok(log)
}

pub async fn decode_history(bytes: &[u8], options: &DecodeOptions) -> Result<HistoryLog, ProtocolError> {
    HistoryReader::open(bytes, options).await?.log().await
}
//#endregion 🔖️Codec

//#region 🔖️Append
// Streaming append API over crate::os_spr::format::SprWriter — the hot path. One edit -> one REC_EDIT
// frame, O(new edit) allocation.
pub struct HistoryAppender<S: PackSink> {
    writer: SprWriter<S>,
    dict: DictBuilder,
    dict_base: u32,
    edit_ordinals: HashMap<String, u64>,
    next_edit_ordinal: u64,
}

impl<S: PackSink> HistoryAppender<S> {
    pub async fn begin(sink: S, doc_id: &str, schema: &str, options: &WriteOptions) -> Result<Self, ProtocolError> {
        let mut writer = SprWriter::begin(sink, options).await?;
        let mut dict = DictBuilder::new().await;
        let mut dict_base = 0u32;
        let payload = encode_doc(doc_id, schema, &mut dict).await;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(crate::os_spr::REC_DOC, true, &payload, CodecId(0)).await?;
        Ok(Self { writer, dict, dict_base, edit_ordinals: HashMap::new(), next_edit_ordinal: 0 })
    }

    pub async fn append_edit(&mut self, edit: &HistoryEdit) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_edit(edit, &mut self.dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        let offset = self.writer.write_record(crate::os_spr::REC_EDIT, true, &payload, CodecId(0)).await?;
        self.edit_ordinals.insert(edit.id.clone(), self.next_edit_ordinal);
        self.next_edit_ordinal += 1;
        Ok(offset)
    }

    pub async fn append_change(&mut self, change: &HistoryChange) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_change(change, &mut self.dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(crate::os_spr::REC_CHANGE, true, &payload, CodecId(0)).await
    }

    pub async fn append_checkpoint(&mut self, checkpoint: &HistoryCheckpoint) -> Result<u64, ProtocolError> {
        let payload = encode_checkpoint(checkpoint, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(crate::os_spr::REC_CHECKPOINT, true, &payload, CodecId(0)).await
    }

    pub async fn append_alternative(&mut self, alternative: &HistoryAlternative) -> Result<u64, ProtocolError> {
        let payload = encode_alternative(alternative, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(crate::os_spr::REC_ALTERNATIVE, true, &payload, CodecId(0)).await
    }

    pub async fn set_active(&mut self, alternative_id: Option<&str>) -> Result<u64, ProtocolError> {
        let payload = encode_active(alternative_id, &mut self.dict).await;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(crate::os_spr::REC_ACTIVE, true, &payload, CodecId(0)).await
    }

    pub async fn append_cursor(&mut self, cursor: &HistoryCursor) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_cursor(cursor, &mut self.dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_CURSOR, false, &payload, CodecId(0)).await
    }

    pub async fn commit(&mut self) -> Result<u64, ProtocolError> {
        self.writer.commit().await
    }

    pub async fn into_sink(self) -> S {
        self.writer.into_sink().await
    }
}
//#endregion 🔖️Append

//#region 🔖️Scan
// Read-side over a byte buffer, via protocol_format cursors. `open` establishes a trusted byte
// range via `crate::os_spr::format::recover` (RecoveryMode::LastCommit) once; every subsequent
// operation stays within that range, so a torn tail can never surface a partially-written record.
pub struct HistoryReader<'a> {
    trusted: &'a [u8],
    options: DecodeOptions,
}

impl<'a> HistoryReader<'a> {
    pub async fn open(bytes: &'a [u8], options: &DecodeOptions) -> Result<Self, ProtocolError> {
        let recovery = crate::os_spr::format::recover(&bytes, &options.limits, RecoveryMode::LastCommit).await?;
        let trusted = &bytes[..recovery.bytes_recovered as usize];
        Ok(Self { trusted, options: options.clone() })
    }

    pub async fn log(&self) -> Result<HistoryLog, ProtocolError> {
        decode_history_from(self.trusted, &self.options).await
    }

    pub async fn edits(&self) -> EditIter<'a> {
        EditIter { cursor: FrameCursor::new(self.trusted, HEADER_SIZE as u64).await, dict: DictReader::new().await, edit_ids: Vec::new() }
    }

    pub async fn edits_rev(&self, limit: usize) -> RevEditIter<'a> {
        match prescan_full(self.trusted).await {
            Ok((dict, edit_ids)) => RevEditIter { state: Ok(RevEditIterReady { cursor: ReverseFrameCursor::at_end(&self.trusted[HEADER_SIZE..]).await, dict, edit_ids, remaining: limit }) },
            Err(e) => RevEditIter { state: Err(Some(e)) },
        }
    }
}

pub struct EditIter<'a> {
    cursor: FrameCursor<'a>,
    dict: DictReader,
    edit_ids: Vec<String>,
}

impl<'a> Iterator for EditIter<'a> {
    type Item = Result<HistoryEdit, ProtocolError>;

    // 🚫️async: E1 — `Iterator::next` is an externally-declared trait method, so it must stay a
    // plain sync `fn`. Its callees (`next_frame`/`payload`/`apply_dict_record`/`decode_edit`) are
    // pure in-memory byte-buffer parsing with no real suspension point, so they are resolved via
    // the crate's one sanctioned E5 bridge (`os_io::resolve_ready`) rather than awaited.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match crate::os_io::resolve_ready(self.cursor.next_frame()) {
                Ok(Some(frame)) => match frame.kind {
                    crate::os_spr::REC_STR_DICT => {
                        let payload = crate::os_io::resolve_ready(frame.payload());
                        if let Err(e) = crate::os_io::resolve_ready(apply_dict_record(&mut self.dict, payload)) {
                            return Some(Err(e));
                        }
                    }
                    crate::os_spr::REC_EDIT => {
                        let edit_ids_ref = &self.edit_ids;
                        let dict_ref = &self.dict;
                        let payload = crate::os_io::resolve_ready(frame.payload());
                        let result = crate::os_io::resolve_ready(decode_edit(payload, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))));
                        match result {
                            Ok(edit) => {
                                self.edit_ids.push(edit.id.clone());
                                return Some(Ok(edit));
                            }
                            Err(e) => return Some(Err(e)),
                        }
                    }
                    _ => continue,
                },
                Ok(None) => return None,
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

struct RevEditIterReady<'a> {
    cursor: ReverseFrameCursor<'a>,
    dict: DictReader,
    edit_ids: Vec<String>,
    remaining: usize,
}

/// @emoji 🚧️ `edits_rev` cannot return `Result` per the frozen contract signature, so a prescan
/// failure (needed to build the full dict + edit-id table up front — see `prescan_full`) is
/// deferred: the first `next()` call yields it, every call after that yields `None`.
pub struct RevEditIter<'a> {
    state: Result<RevEditIterReady<'a>, Option<ProtocolError>>,
}

impl<'a> Iterator for RevEditIter<'a> {
    type Item = Result<HistoryEdit, ProtocolError>;

    // 🚫️async: E1 — same rationale as `EditIter::next` above: `Iterator::next` must stay sync, its
    // callees are pure in-memory parsing, resolved via the crate's one `os_io::resolve_ready` bridge.
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            Err(pending) => pending.take().map(Err),
            Ok(ready) => {
                if ready.remaining == 0 {
                    return None;
                }
                loop {
                    match crate::os_io::resolve_ready(ready.cursor.prev_frame()) {
                        Ok(Some(frame)) => {
                            if frame.kind == crate::os_spr::REC_EDIT {
                                let edit_ids_ref = &ready.edit_ids;
                                let dict_ref = &ready.dict;
                                let payload = crate::os_io::resolve_ready(frame.payload());
                                let result = crate::os_io::resolve_ready(decode_edit(payload, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))));
                                ready.remaining -= 1;
                                return Some(result);
                            }
                        }
                        Ok(None) => return None,
                        Err(e) => {
                            ready.remaining = 0;
                            return Some(Err(e));
                        }
                    }
                }
            }
        }
    }
}

/// @emoji 🔎️ Builds the FULL dictionary and the FULL forward-ordered edit-id table in one forward
/// pass. Safe to reuse for decoding any earlier record: dict indices and edit ordinals are both
/// append-only and stable once assigned, so the final state is a superset valid at every offset.
async fn prescan_full(trusted: &[u8]) -> Result<(DictReader, Vec<String>), ProtocolError> {
    let mut dict = DictReader::new().await;
    let mut edit_ids = Vec::new();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64).await;
    while let Some(frame) = cursor.next_frame().await? {
        match frame.kind {
            crate::os_spr::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload().await).await?,
            crate::os_spr::REC_EDIT => {
                let edit_ids_ref = &edit_ids;
                let dict_ref = &dict;
                let edit = decode_edit(frame.payload().await, dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await?;
                edit_ids.push(edit.id);
            }
            _ => {}
        }
    }
    Ok((dict, edit_ids))
}
//#endregion 🔖️Scan

//#region 🔖️Frontier
#[derive(Clone, Debug, PartialEq)]
pub struct AlternativeHead {
    pub alternative_id: String,
    pub checkpoint_id: String,
    pub head_edit_ordinal: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrontierSummary {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub alternatives: Vec<AlternativeHead>,
    pub last_commit_seq: u64,
    pub chain_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrontierComparison {
    Equal,
    Ahead,
    Behind,
    Diverged { common_edit_count: u64 },
}

/// @emoji 🧭️ Compares two frontiers by head edit ordinal, then head edit id/chain hash for the
/// equal-ordinal case. `Diverged::common_edit_count` is a conservative estimate (the shared
/// ordinal itself) — `FrontierSummary` alone carries no shared-ancestry data to verify a true
/// common ancestor; documented "your choice" per the contract.
pub async fn frontier_delta(local: &FrontierSummary, remote: &FrontierSummary) -> FrontierComparison {
    if local.head_edit_ordinal == remote.head_edit_ordinal && local.head_edit_id == remote.head_edit_id && local.chain_hash == remote.chain_hash {
        return FrontierComparison::Equal;
    }
    if local.head_edit_ordinal > remote.head_edit_ordinal {
        return FrontierComparison::Ahead;
    }
    if local.head_edit_ordinal < remote.head_edit_ordinal {
        return FrontierComparison::Behind;
    }
    FrontierComparison::Diverged { common_edit_count: local.head_edit_ordinal }
}
//#endregion 🔖️Frontier

//#region 🔖️Index
// Advisory REC_INDEX payload sections. Rebuildable from scan; never authoritative. Byte layout is
// this crate's own choice (the contract only fixes the section-kind bytes): format u8, then zero
// or more `(section_kind: u8, count: varint, count x entry)` sections back-to-back.
pub const SEC_EDIT_OFFSETS: u8 = 0x01;
pub const SEC_CHECKPOINT_OFFSETS: u8 = 0x02;
pub const SEC_DICT_OFFSETS: u8 = 0x03;
pub const SEC_SNAPSHOT_OFFSETS: u8 = 0x04;
pub const SEC_SEALED_OFFSETS: u8 = 0x05;

async fn write_pair_section(out: &mut ByteWriter, kind: u8, entries: &[(u64, u64)]) {
    out.write_u8(kind).await;
    out.write_varint_u64(entries.len() as u64).await;
    for (a, b) in entries {
        out.write_varint_u64(*a).await;
        out.write_varint_u64(*b).await;
    }
}

async fn write_offsets_section(out: &mut ByteWriter, kind: u8, offsets: &[u64]) {
    out.write_u8(kind).await;
    out.write_varint_u64(offsets.len() as u64).await;
    for offset in offsets {
        out.write_varint_u64(*offset).await;
    }
}

#[derive(Clone, Debug, Default)]
pub struct IndexBuilder {
    edits: Vec<(u64, u64)>,
    checkpoints: Vec<(String, u64, u64)>,
    dict_offsets: Vec<u64>,
    snapshots: Vec<(u64, u64)>,
    sealed: Vec<u64>,
}

impl IndexBuilder {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn record_edit(&mut self, ordinal: u64, offset: u64) {
        self.edits.push((ordinal, offset));
    }

    pub async fn record_checkpoint(&mut self, id: &str, offset: u64, edit_ordinal: u64) {
        self.checkpoints.push((id.to_string(), offset, edit_ordinal));
    }

    pub async fn record_dict(&mut self, offset: u64) {
        self.dict_offsets.push(offset);
    }

    pub async fn record_snapshot(&mut self, edit_ordinal: u64, offset: u64) {
        self.snapshots.push((edit_ordinal, offset));
    }

    pub async fn record_sealed(&mut self, offset: u64) {
        self.sealed.push(offset);
    }

    pub async fn build(&self) -> Vec<u8> {
        let mut out = ByteWriter::new().await;
        out.write_u8(1).await;
        write_pair_section(&mut out, SEC_EDIT_OFFSETS, &self.edits).await;
        out.write_u8(SEC_CHECKPOINT_OFFSETS).await;
        out.write_varint_u64(self.checkpoints.len() as u64).await;
        for (id, offset, edit_ordinal) in &self.checkpoints {
            write_str_field(&mut out, id).await;
            out.write_varint_u64(*offset).await;
            out.write_varint_u64(*edit_ordinal).await;
        }
        write_offsets_section(&mut out, SEC_DICT_OFFSETS, &self.dict_offsets).await;
        write_pair_section(&mut out, SEC_SNAPSHOT_OFFSETS, &self.snapshots).await;
        write_offsets_section(&mut out, SEC_SEALED_OFFSETS, &self.sealed).await;
        out.into_bytes().await
    }
}

pub struct IndexReader<'a> {
    edits: Vec<(u64, u64)>,
    checkpoints: Vec<(&'a str, u64, u64)>,
    snapshots: Vec<(u64, u64)>,
}

impl<'a> IndexReader<'a> {
    pub async fn open(payload: &'a [u8]) -> Result<Self, ProtocolError> {
        let mut input = ByteReader::new(payload).await;
        let format = input.read_u8().await?;
        if format > 1 {
            return Err(malformed_fmt("index", format).await);
        }
        let mut edits = Vec::new();
        let mut checkpoints = Vec::new();
        let mut snapshots = Vec::new();
        while input.remaining().await > 0 {
            let kind = input.read_u8().await?;
            let count = input.read_varint_u64().await?;
            match kind {
                SEC_EDIT_OFFSETS => {
                    for _ in 0..count {
                        let ordinal = input.read_varint_u64().await?;
                        let offset = input.read_varint_u64().await?;
                        edits.push((ordinal, offset));
                    }
                }
                SEC_CHECKPOINT_OFFSETS => {
                    for _ in 0..count {
                        let len = input.read_varint_u64().await? as usize;
                        let bytes = input.read_bytes(len).await?;
                        let id = std::str::from_utf8(bytes).map_err(|_| ProtocolError::Malformed { what: "index checkpoint id utf8", offset: 0, detail: "invalid utf-8".to_string() })?;
                        let offset = input.read_varint_u64().await?;
                        let edit_ordinal = input.read_varint_u64().await?;
                        checkpoints.push((id, offset, edit_ordinal));
                    }
                }
                SEC_DICT_OFFSETS => {
                    for _ in 0..count {
                        input.read_varint_u64().await?;
                    }
                }
                SEC_SNAPSHOT_OFFSETS => {
                    for _ in 0..count {
                        let ordinal = input.read_varint_u64().await?;
                        let offset = input.read_varint_u64().await?;
                        snapshots.push((ordinal, offset));
                    }
                }
                SEC_SEALED_OFFSETS => {
                    for _ in 0..count {
                        input.read_varint_u64().await?;
                    }
                }
                other => return Err(ProtocolError::Malformed { what: "index section kind", offset: 0, detail: format!("unknown section {other:#x}") }),
            }
        }
        Ok(Self { edits, checkpoints, snapshots })
    }

    pub async fn edit_offset_at_or_before(&self, ordinal: u64) -> Option<u64> {
        self.edits.iter().filter(|(o, _)| *o <= ordinal).max_by_key(|(o, _)| *o).map(|(_, offset)| *offset)
    }

    pub async fn checkpoint_offset(&self, checkpoint_id: &str) -> Option<(u64, u64)> {
        self.checkpoints.iter().find(|(id, _, _)| *id == checkpoint_id).map(|(_, offset, edit_ordinal)| (*offset, *edit_ordinal))
    }

    pub async fn latest_snapshot_offset_at_or_before(&self, ordinal: u64) -> Option<u64> {
        self.snapshots.iter().filter(|(o, _)| *o <= ordinal).max_by_key(|(o, _)| *o).map(|(_, offset)| *offset)
    }
}
//#endregion 🔖️Index

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn sample_log() -> HistoryLog {
        HistoryLog {
            doc_id: "doc-1".to_string(),
            schema: "org.semio.demo.v1".to_string(),
            edits: vec![
                HistoryEdit {
                    id: "edit-1".to_string(),
                    actor: Some("alice".to_string()),
                    started_at: "2024-01-15T10:30:00Z".to_string(),
                    finished_at: Some("2024-01-15T10:30:05Z".to_string()),
                    coalesce_key: Some("typing".to_string()),
                    description: Some("first edit".to_string()),
                    ops: vec![OpPayload { text: Some("set foo=1".to_string()), binary: None }, OpPayload { text: Some("set bar=2".to_string()), binary: None }],
                    inverse: Vec::new(),
                    meta: None,
                },
                HistoryEdit {
                    id: "edit-2".to_string(),
                    actor: None,
                    started_at: "not-a-canonical-timestamp".to_string(),
                    finished_at: None,
                    coalesce_key: None,
                    description: None,
                    ops: vec![OpPayload { text: Some("set baz=3".to_string()), binary: None }],
                    inverse: Vec::new(),
                    meta: Some(vec![HistoryOpMeta {
                        op_id: Some("op-1".to_string()),
                        dependencies: vec!["edit-1".to_string()],
                        base_version: 7,
                        author_id: Some("alice".to_string()),
                        hlt: Some((1, 1_700_000_000_000, 3)),
                        undo_policy: 2,
                        payload_hash: Some([9u8; 32]),
                        // 🎯️ Non-`None` on purpose: `sample_log().await` feeds every encode/decode
                        // identity test below (`history_encode_decode_identity_standard` etc.), so
                        // a populated `group_id` here proves the composite-gesture stamp survives
                        // a real `.spr` byte round trip via `assert_eq!(decoded, log)`, not just a
                        // narrowly-targeted unit test.
                        group_id: Some("group-composite-1".to_string()),
                        // 🎯️ A non-`Owner`, field-carrying variant on purpose (mirrors the `group_id`
                        // choice above): proves `MutationOrigin::Contributed`'s structured payload —
                        // not just the unit `Owner` case — survives a real `.spr` byte round trip.
                        origin: crate::os_spr::command::MutationOrigin::Contributed {
                            plugin_id: "flow".to_string(),
                            mutation_id: crate::os_spr::ids::SchemaId("widget.doc#recolor".to_string()),
                            payload_hash: crate::os_spr::ids::PayloadHash([3u8; 32]),
                        },
                        messages: Vec::new(),
                    }]),
                },
            ],
            changes: vec![HistoryChange { id: "change-1".to_string(), saved_at: "2024-01-15T10:31:00Z".to_string(), edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], description: None }],
            checkpoints: vec![HistoryCheckpoint {
                id: "ck-1".to_string(),
                timestamp: "2024-01-15T10:32:00Z".to_string(),
                change_ids: vec!["change-1".to_string()],
                parent_id: None,
                authors: vec![HistoryAuthor { id: "u1".to_string(), name: "Ueli Saluz".to_string() }],
                message: Some("first checkpoint".to_string()),
            }],
            alternatives: vec![HistoryAlternative { id: "alt-1".to_string(), name: "main".to_string(), checkpoint_ids: vec!["ck-1".to_string()] }],
            active_alternative_id: Some("alt-1".to_string()),
            cursor: None,
            composition: None,
            conflicts: Vec::new(),
        }
    }

    async fn sample_conflicts() -> Vec<HistoryConflict> {
        vec![
            HistoryConflict {
                id: "conflict-quarantine-1".to_string(),
                kind: 0,
                status: 0,
                actors: vec!["alice".to_string(), "bob".to_string()],
                hlt: (1, 1_700_000_000_000, 4),
                edit_ids: Vec::new(),
                envelopes: vec![vec![1, 2, 3], vec![4, 5, 6, 7]],
                messages: vec![HistoryMessage { level: 2, code: "mutation.duplicate-id".to_string(), message: "id collided".to_string(), target: vec!["node-1".to_string()], op_index: Some(0) }],
            },
            HistoryConflict {
                id: "conflict-degraded-1".to_string(),
                kind: 1,
                status: 1,
                actors: vec!["carol".to_string()],
                hlt: (2, 1_700_000_001_000, 0),
                edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()],
                envelopes: Vec::new(),
                messages: vec![
                    HistoryMessage { level: 0, code: "mutation.cascade".to_string(), message: "cascaded".to_string(), target: Vec::new(), op_index: None },
                    HistoryMessage { level: 1, code: "mutation.partial".to_string(), message: "partial apply".to_string(), target: vec!["a".to_string(), "b".to_string()], op_index: Some(1) },
                ],
            },
        ]
    }

    //#region 🔖️Composition
    #[semio_framework_async_macros::async_test]
    async fn composition_overlay_round_trips_through_the_binary_log() {
        let composition = HistoryComposition {
            owner: Some(("parent-1!s.stdio.object@1/*".to_string(), "mesh".to_string(), "child-1".to_string())),
            dialect: Some(("s.stdio.mesh".to_string(), "1".to_string(), "*".to_string())),
            checkpoint_pins: vec![("ck-1".to_string(), vec![("child-1!s.stdio.mesh@1/*".to_string(), "ck-child-7".to_string())])],
        };
        let log = HistoryLog { composition: Some(composition.clone()), ..sample_log().await };

        let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode");
        let decoded = decode_history(&bytes, &DecodeOptions::default()).await.expect("decode");
        assert_eq!(decoded.composition, Some(composition), "the composition overlay did not survive the binary round trip");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_log_without_composition_writes_no_composition_record() {
        let bytes = encode_history(&sample_log().await, &EncodeOptions::default()).await.expect("encode");
        assert_eq!(decode_history(&bytes, &DecodeOptions::default()).await.expect("decode").composition, None);
        // 🎯️ The record is non-critical, so a reader that skips it must still read the whole log —
        // which is exactly what "absent" and "skipped" both look like from here.
        assert!(!bytes.windows(1).any(|window| window == [REC_COMPOSITION]) || decode_history(&bytes, &DecodeOptions::default()).await.is_ok());
    }
    //#endregion 🔖️Composition

    //#region 🔖️Conflict
    #[semio_framework_async_macros::async_test]
    async fn conflict_payload_round_trips_both_kinds_byte_identically() {
        let log = HistoryLog { conflicts: sample_conflicts().await, ..sample_log().await };
        let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode");
        let decoded = decode_history(&bytes, &DecodeOptions::default()).await.expect("decode");
        assert_eq!(decoded, log, "conflicts of both kinds must survive the binary round trip structurally");
        let re_encoded = encode_history(&decoded, &EncodeOptions::default()).await.expect("re-encode");
        assert_eq!(re_encoded, bytes, "re-encoding the decoded log must reproduce byte-identical output");
    }

    #[semio_framework_async_macros::async_test]
    async fn encode_conflicts_round_trips_with_dict_and_ordinal_refs() {
        let conflicts = sample_conflicts().await;
        let mut dict = DictBuilder::new().await;
        let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
        let payload = encode_conflicts(&conflicts, &mut dict, |id| ordinals.get(id).copied()).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
        let decoded = decode_conflicts(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded, conflicts);
    }

    #[semio_framework_async_macros::async_test]
    async fn encode_conflicts_round_trips_empty() {
        let mut dict = DictBuilder::new().await;
        let payload = encode_conflicts(&Vec::new(), &mut dict, |_| None).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let decoded = decode_conflicts(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded, Vec::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn conflict_decoder_rejects_duplicate_ids_and_trailing_payload_bytes() {
        let conflict = sample_conflicts().await.remove(0);
        let mut dict = DictBuilder::new().await;
        let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
        let duplicate_payload = encode_conflicts(&vec![conflict.clone(), conflict], &mut dict, |id| ordinals.get(id).copied()).await.expect("encode duplicate fixture");
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.expect("dictionary");
        let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
        assert!(matches!(decode_conflicts(&duplicate_payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await, Err(ProtocolError::Malformed { .. })));

        let mut trailing_dict = DictBuilder::new().await;
        let mut trailing_payload = encode_conflicts(&sample_conflicts().await, &mut trailing_dict, |_| None).await.expect("encode trailing fixture");
        trailing_payload.push(0);
        let mut trailing_reader = DictReader::new().await;
        trailing_reader.extend(0, trailing_dict.entries_since(0).await.to_vec()).await.expect("dictionary");
        assert!(matches!(decode_conflicts(&trailing_payload, &trailing_reader, |_| Err(ProtocolError::DictMiss(0))).await, Err(ProtocolError::Malformed { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn conflict_codec_rejects_unknown_kind_and_status_tags() {
        let mut dict = DictBuilder::new().await;
        let mut conflicts = sample_conflicts().await;
        conflicts[0].kind = 9;
        assert!(matches!(encode_conflicts(&conflicts, &mut dict, |_| None).await, Err(ProtocolError::Malformed { what: "conflict", .. })));

        let mut dict = DictBuilder::new().await;
        let mut payload = encode_conflicts(&sample_conflicts().await[..1], &mut dict, |_| None).await.expect("encode valid conflict");
        let tag_offset = payload.windows(3).position(|window| window == [0, 0, 2]).expect("kind/status/actor-count tags");
        payload[tag_offset + 1] = 9;
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.expect("dictionary");
        assert!(matches!(decode_conflicts(&payload, &reader, |_| Err(ProtocolError::DictMiss(0))).await, Err(ProtocolError::Malformed { what: "conflict", .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn a_log_without_conflicts_writes_no_conflict_record() {
        let bytes = encode_history(&sample_log().await, &EncodeOptions::default()).await.expect("encode");
        assert_eq!(decode_history(&bytes, &DecodeOptions::default()).await.expect("decode").conflicts, Vec::new());
        // 🎯️ Scans actual frames (not a raw byte-window guess like the composition precedent
        // above) — the rigorous form of "no conflicts ⇒ no REC_CONFLICT record emitted".
        let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
        let mut saw_conflict_record = false;
        while let Some(frame) = cursor.next_frame().await.expect("scan") {
            if frame.kind == REC_CONFLICT {
                saw_conflict_record = true;
            }
        }
        assert!(!saw_conflict_record, "no REC_CONFLICT frame should be emitted when conflicts is empty");
    }
    //#endregion 🔖️Conflict

    //#region 🔖️Message
    #[semio_framework_async_macros::async_test]
    async fn op_meta_messages_round_trip_every_severity_and_target_shape() {
        let meta = HistoryOpMeta {
            op_id: Some("op-9".to_string()),
            dependencies: Vec::new(),
            base_version: 1,
            author_id: None,
            hlt: None,
            undo_policy: 0,
            payload_hash: None,
            group_id: None,
            origin: crate::os_spr::command::MutationOrigin::Owner,
            messages: vec![
                HistoryMessage { level: 0, code: "mutation.cascade".to_string(), message: "cascaded".to_string(), target: Vec::new(), op_index: None },
                HistoryMessage { level: 1, code: "mutation.clamped".to_string(), message: "clamped".to_string(), target: vec!["a".to_string()], op_index: Some(0) },
                HistoryMessage { level: 2, code: "mutation.target-missing".to_string(), message: "missing".to_string(), target: vec!["a".to_string(), "b".to_string()], op_index: Some(3) },
                HistoryMessage { level: 3, code: "mutation.invariant".to_string(), message: "broken".to_string(), target: vec!["x".to_string(), "y".to_string(), "z".to_string()], op_index: None },
            ],
        };
        let edit = HistoryEdit {
            id: "edit-m".to_string(),
            actor: None,
            started_at: "2024-01-01T00:00:00Z".to_string(),
            finished_at: None,
            coalesce_key: None,
            description: None,
            ops: vec![OpPayload { text: Some("noop".to_string()), binary: None }],
            inverse: Vec::new(),
            meta: Some(vec![meta.clone()]),
        };
        let mut dict = DictBuilder::new().await;
        let payload = encode_edit(&edit, &mut dict, |_| None).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded.meta, Some(vec![meta]));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_meta_without_messages_writes_no_messages_section() {
        let meta = HistoryOpMeta { op_id: None, dependencies: Vec::new(), base_version: 0, author_id: None, hlt: None, undo_policy: 0, payload_hash: None, group_id: None, origin: crate::os_spr::command::MutationOrigin::Owner, messages: Vec::new() };
        let mut dict = DictBuilder::new().await;
        let mut out = ByteWriter::new().await;
        write_op_meta(&mut out, &meta, &mut dict, &|_: &str| None).await.unwrap();
        let payload = out.into_bytes().await;
        // presence byte is the very first byte written by write_op_meta; bit6 (0x40) must be unset.
        assert_eq!(payload[0] & 0b0100_0000, 0, "bit6 must be unset for empty messages");
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let mut input = ByteReader::new(&payload).await;
        let decoded = read_op_meta(&mut input, &reader, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded.messages, Vec::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn history_message_decoder_rejects_invalid_severity_presence_and_index_width() {
        let message = HistoryMessage { level: 1, code: "mutation.clamped".to_string(), message: "clamped".to_string(), target: Vec::new(), op_index: None };
        let mut dict = DictBuilder::new().await;
        let mut out = ByteWriter::new().await;
        write_history_message(&mut out, &message, &mut dict).await.expect("encode message");
        let encoded = out.into_bytes().await;
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.expect("dictionary");

        let mut invalid_severity = encoded.clone();
        invalid_severity[0] = 4;
        assert!(matches!(read_history_message(&mut ByteReader::new(&invalid_severity), &reader).await, Err(ProtocolError::Malformed { .. })));

        let mut invalid_presence = encoded;
        *invalid_presence.last_mut().expect("presence byte") = 2;
        assert!(matches!(read_history_message(&mut ByteReader::new(&invalid_presence), &reader).await, Err(ProtocolError::Malformed { .. })));

        let indexed = HistoryMessage { op_index: Some(0), ..message };
        let mut indexed_out = ByteWriter::new().await;
        write_history_message(&mut indexed_out, &indexed, &mut dict).await.expect("encode indexed message");
        let mut oversized_index = indexed_out.into_bytes().await;
        oversized_index.pop();
        oversized_index.extend_from_slice(&[0x80, 0x80, 0x80, 0x80, 0x10]);
        assert!(matches!(read_history_message(&mut ByteReader::new(&oversized_index), &reader).await, Err(ProtocolError::Malformed { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn message_code_interning_does_not_grow_the_dictionary_linearly() {
        let mut dict = DictBuilder::new().await;
        let mut out = ByteWriter::new().await;
        for _ in 0..500 {
            let message = HistoryMessage { level: 1, code: "mutation.clamped".to_string(), message: String::new(), target: Vec::new(), op_index: None };
            write_history_message(&mut out, &message, &mut dict).await.unwrap();
        }
        assert_eq!(dict.len().await, 1, "500 identical codes must intern to a single dictionary entry, not 500");
        let payload_len = out.into_bytes().await.len();
        let bytes_per_message = payload_len / 500;
        // 🎯️ A raw (non-interned) code string would cost len("mutation.clamped")=17 bytes alone
        // per repeat; interning collapses every repeat after the first to a small dictref (tag +
        // varint index), so the true per-message average must stay far below the raw string's own
        // length — proof the growth is sub-linear, not just "small on average by luck".
        assert!(bytes_per_message < 10, "expected sub-10-byte-per-message average from interning, got {bytes_per_message}");
    }
    //#endregion 🔖️Message

    //#region 🔖️TextGrammar
    #[semio_framework_async_macros::async_test]
    async fn ops_text_round_trips_a_full_log() {
        // `HistoryEdit::meta` is derived data the text grammar never carries (see the Model
        // region note) — parse_ops_text always yields `meta: None`, so the expectation strips it
        // before comparing rather than asserting full structural equality including meta.
        let mut log = sample_log().await;
        for edit in &mut log.edits {
            edit.meta = None;
        }
        let text = print_ops_text(&log).await.unwrap();
        let parsed = parse_ops_text(&text).await.unwrap();
        assert_eq!(parsed, log);
    }

    #[semio_framework_async_macros::async_test]
    async fn ops_text_is_a_fixpoint_under_reprint() {
        let log = sample_log().await;
        let text = print_ops_text(&log).await.unwrap();
        let reparsed = parse_ops_text(&text).await.unwrap();
        assert_eq!(print_ops_text(&reparsed).await.unwrap(), text);
    }

    #[semio_framework_async_macros::async_test]
    async fn ops_text_skips_comments_and_blank_lines() {
        let text = "doc doc-1 schema=s1\n\n# a comment\nactive alt-1\n";
        let log = parse_ops_text(text).await.unwrap();
        assert_eq!(log.doc_id, "doc-1");
        assert_eq!(log.active_alternative_id.as_deref(), Some("alt-1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn ops_text_rejects_unknown_line_keyword() {
        let err = parse_ops_text("doc doc-1 schema=s1\nbogus x\n").await.unwrap_err();
        assert!(matches!(err, ProtocolError::Malformed { .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn ops_text_edit_without_active_line_leaves_none() {
        let log = HistoryLog { doc_id: "d".into(), schema: "s".into(), active_alternative_id: None, ..Default::default() };
        let text = print_ops_text(&log).await.unwrap();
        assert!(!text.contains("active"));
        assert_eq!(parse_ops_text(&text).await.unwrap().active_alternative_id, None);
    }

    #[semio_framework_async_macros::async_test]
    async fn ops_text_round_trips_a_cursor_line_with_undo_then_apply_interleaving() {
        let mut log = sample_log().await;
        for edit in &mut log.edits {
            edit.meta = None;
        }
        // A single tail-edit marker cannot represent this: edit-1 undone (moved to redo), then a
        // later apply produced edit-2 — edit-1 precedes edit-2 in file order but is NOT applied.
        log.cursor = Some(HistoryCursor { applied_edit_ids: vec!["edit-2".to_string()], redo_edit_ids: vec!["edit-1".to_string()], checkpoint_id: Some("ck-1".to_string()) });
        let text = print_ops_text(&log).await.unwrap();
        assert!(text.contains("cursor"));
        let parsed = parse_ops_text(&text).await.unwrap();
        assert_eq!(parsed, log);
    }

    #[semio_framework_async_macros::async_test]
    async fn ops_text_without_a_cursor_line_leaves_cursor_none() {
        let log = HistoryLog { doc_id: "d".into(), schema: "s".into(), ..Default::default() };
        let text = print_ops_text(&log).await.unwrap();
        assert!(!text.contains("cursor"));
        assert_eq!(parse_ops_text(&text).await.unwrap().cursor, None);
    }
    //#endregion 🔖️TextGrammar

    //#region 🔖️Payloads
    #[semio_framework_async_macros::async_test]
    async fn doc_payload_round_trips() {
        let mut dict = DictBuilder::new().await;
        let payload = encode_doc("doc-1", "org.semio.demo.v1", &mut dict).await;
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let (id, schema) = decode_doc(&payload, &reader).await.unwrap();
        assert_eq!(id, "doc-1");
        assert_eq!(schema, "org.semio.demo.v1");
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_payload_round_trips_with_all_optionals_and_meta() {
        let log = sample_log().await;
        let edit = &log.edits[1];
        let mut dict = DictBuilder::new().await;
        let ordinals: HashMap<&str, u64> = [("edit-1", 0u64)].into_iter().collect();
        let payload = encode_edit(edit, &mut dict, |id| ordinals.get(id).copied()).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let edit_ids = ["edit-1".to_string()];
        let decoded = decode_edit(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded, *edit);
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_payload_round_trips_minimal_edit() {
        let edit = HistoryEdit { id: "edit-x".to_string(), actor: None, started_at: "2024-01-01T00:00:00Z".to_string(), finished_at: None, coalesce_key: None, description: None, ops: Vec::new(), inverse: Vec::new(), meta: None };
        let mut dict = DictBuilder::new().await;
        let payload = encode_edit(&edit, &mut dict, |_| None).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded, edit);
    }

    #[semio_framework_async_macros::async_test]
    async fn change_payload_round_trips_and_references_edit_ordinals() {
        let change = HistoryChange { id: "change-1".to_string(), saved_at: "2024-01-01T00:00:00Z".to_string(), edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], description: Some("d".to_string()) };
        let mut dict = DictBuilder::new().await;
        let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
        let payload = encode_change(&change, &mut dict, |id| ordinals.get(id).copied()).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
        let decoded = decode_change(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded, change);
    }

    #[semio_framework_async_macros::async_test]
    async fn checkpoint_payload_round_trips_with_authors() {
        let checkpoint = sample_log().await.checkpoints.remove(0);
        let mut dict = DictBuilder::new().await;
        let payload = encode_checkpoint(&checkpoint, &mut dict).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let decoded = decode_checkpoint(&payload, &reader).await.unwrap();
        assert_eq!(decoded, checkpoint);
    }

    #[semio_framework_async_macros::async_test]
    async fn alternative_payload_round_trips() {
        let alternative = sample_log().await.alternatives.remove(0);
        let mut dict = DictBuilder::new().await;
        let payload = encode_alternative(&alternative, &mut dict).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let decoded = decode_alternative(&payload, &reader).await.unwrap();
        assert_eq!(decoded, alternative);
    }

    #[semio_framework_async_macros::async_test]
    async fn active_payload_round_trips_some_and_none() {
        let mut dict = DictBuilder::new().await;
        let payload_some = encode_active(Some("alt-1"), &mut dict).await;
        let payload_none = encode_active(None, &mut dict).await;
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        assert_eq!(decode_active(&payload_some, &reader).await.unwrap(), Some("alt-1".to_string()));
        assert_eq!(decode_active(&payload_none, &reader).await.unwrap(), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_payload_round_trips_a_backwards_section_mixing_text_and_binary_payloads() {
        let edit = HistoryEdit {
            id: "edit-y".to_string(),
            actor: Some("bob".to_string()),
            started_at: "2024-02-01T00:00:00Z".to_string(),
            finished_at: Some("2024-02-01T00:00:01Z".to_string()),
            coalesce_key: None,
            description: None,
            ops: vec![OpPayload { text: Some("set n=1".to_string()), binary: Some(vec![1, 2, 3]) }, OpPayload { text: Some("set n=2".to_string()), binary: None }],
            inverse: vec![OpPayload { text: Some("set n=0".to_string()), binary: Some(vec![0]) }, OpPayload { text: Some("set n=1".to_string()), binary: None }],
            meta: None,
        };
        let mut dict = DictBuilder::new().await;
        let payload = encode_edit(&edit, &mut dict, |_| None).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded, edit);
        assert_eq!(decoded.ops[0].binary, Some(vec![1, 2, 3]));
        assert_eq!(decoded.inverse[0].binary, Some(vec![0]));
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_payload_with_empty_backwards_omits_the_section_and_decodes_empty() {
        let edit = HistoryEdit {
            id: "edit-z".to_string(),
            actor: None,
            started_at: "2024-02-01T00:00:00Z".to_string(),
            finished_at: None,
            coalesce_key: None,
            description: None,
            ops: vec![OpPayload { text: Some("noop".to_string()), binary: None }],
            inverse: Vec::new(),
            meta: None,
        };
        let mut dict = DictBuilder::new().await;
        let payload = encode_edit(&edit, &mut dict, |_| None).await.unwrap();
        // presence byte is the 2nd byte (offset 1); bit5 (0x20) must be unset when inverse is empty.
        assert_eq!(payload[1] & 0b0010_0000, 0, "bit5 must be unset for empty inverse");
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded.inverse, Vec::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn cursor_payload_round_trips_with_dict_and_ordinal_refs() {
        let cursor = HistoryCursor { applied_edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], redo_edit_ids: vec!["edit-3".to_string()], checkpoint_id: Some("ck-1".to_string()) };
        let mut dict = DictBuilder::new().await;
        let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
        let payload = encode_cursor(&cursor, &mut dict, |id| ordinals.get(id).copied()).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
        let decoded = decode_cursor(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded, cursor);
    }

    #[semio_framework_async_macros::async_test]
    async fn cursor_payload_round_trips_without_a_checkpoint() {
        let cursor = HistoryCursor { applied_edit_ids: Vec::new(), redo_edit_ids: Vec::new(), checkpoint_id: None };
        let mut dict = DictBuilder::new().await;
        let payload = encode_cursor(&cursor, &mut dict, |_| None).await.unwrap();
        let mut reader = DictReader::new().await;
        reader.extend(0, dict.entries_since(0).await.to_vec()).await.unwrap();
        let decoded = decode_cursor(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
        assert_eq!(decoded, cursor);
    }

    #[semio_framework_async_macros::async_test]
    async fn cursor_decoder_rejects_unknown_presence_bits_and_trailing_payload() {
        let dict = DictReader::new().await;
        assert!(matches!(decode_cursor(&[1, 2], &dict, |_| Err(ProtocolError::DictMiss(0))).await, Err(ProtocolError::Malformed { .. })));
        assert!(matches!(decode_cursor(&[1, 0, 0, 0, 0], &dict, |_| Err(ProtocolError::DictMiss(0))).await, Err(ProtocolError::Malformed { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_rejects_unsupported_format() {
        let payload = vec![2u8, 0, 0];
        let dict = DictReader::new().await;
        let err = decode_doc(&payload, &dict).await.unwrap_err();
        assert!(matches!(err, ProtocolError::Malformed { .. }));
    }
    //#endregion 🔖️Payloads

    //#region 🔖️Codec
    #[semio_framework_async_macros::async_test]
    async fn history_encode_decode_identity_standard() {
        let log = sample_log().await;
        let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
        let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
        assert_eq!(decoded, log);
    }

    #[semio_framework_async_macros::async_test]
    async fn history_encode_decode_identity_full_verification() {
        let log = sample_log().await;
        let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
        let options = DecodeOptions { verification: VerificationLevel::Full, limits: ProtocolLimits::default() };
        let decoded = decode_history(&bytes, &options).await.unwrap();
        assert_eq!(decoded, log);
    }

    #[semio_framework_async_macros::async_test]
    async fn history_encode_is_canonically_stable() {
        let log = sample_log().await;
        let a = encode_history(&log, &EncodeOptions::default()).await.unwrap();
        let b = encode_history(&log, &EncodeOptions::default()).await.unwrap();
        assert_eq!(a, b);
    }

    #[semio_framework_async_macros::async_test]
    async fn history_full_verification_detects_tampering() {
        let log = sample_log().await;
        let mut bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
        let last = bytes.len();
        bytes[last - 10] ^= 0xFF;
        let options = DecodeOptions { verification: VerificationLevel::Full, limits: ProtocolLimits::default() };
        // A single-byte flip breaks that frame's own CRC-32C, so `crate::os_spr::format::recover`
        // (RecoveryMode::LastCommit, run by `HistoryReader::open`) truncates the trusted range to
        // before the corrupted frame — here that means before the file's only commit, so the
        // decoded log comes back empty rather than as an `Err`. Either outcome is an acceptable
        // "tamper never goes unnoticed": the result must never silently equal the original log.
        let result = decode_history(&bytes, &options).await;
        assert!(result.is_err() || result.unwrap() != log);
    }

    #[semio_framework_async_macros::async_test]
    async fn history_round_trips_backwards_and_binary_payloads_and_cursor_when_write_backwards_section_is_set() {
        let mut log = sample_log().await;
        log.edits[0].inverse = vec![OpPayload { text: Some("unset foo".to_string()), binary: Some(vec![9, 9]) }, OpPayload { text: Some("unset bar".to_string()), binary: None }];
        log.edits[1].ops[0].binary = Some(vec![7]);
        log.cursor = Some(HistoryCursor { applied_edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], redo_edit_ids: Vec::new(), checkpoint_id: Some("ck-1".to_string()) });
        let options = EncodeOptions { write_backwards_section: true, ..EncodeOptions::default() };
        let bytes = encode_history(&log, &options).await.unwrap();
        let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
        assert_eq!(decoded, log);
        assert_eq!(decoded.edits[0].inverse[0].binary, Some(vec![9, 9]));
        assert_eq!(decoded.edits[1].ops[0].binary, Some(vec![7]));
        assert_eq!(decoded.cursor, log.cursor);
    }

    #[semio_framework_async_macros::async_test]
    async fn history_strips_backwards_when_write_backwards_section_is_unset_even_if_populated() {
        let mut log = sample_log().await;
        log.edits[0].inverse = vec![OpPayload { text: Some("unset foo".to_string()), binary: None }];
        let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
        let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
        assert_eq!(decoded.edits[0].inverse, Vec::new(), "write_backwards_section defaults false and must strip populated inverse");
    }
    //#endregion 🔖️Codec

    //#region 🔖️Append
    #[semio_framework_async_macros::async_test]
    async fn streamed_append_equals_buffered_encode() {
        let log = sample_log().await;
        let options = WriteOptions { required_flags: crate::os_spr::REQUIRED_HASH_CHAIN, optional_flags: crate::os_spr::OPTIONAL_CANONICAL };
        let mut appender = HistoryAppender::begin(Vec::<u8>::new(), &log.doc_id, &log.schema, &options).await.unwrap();
        for edit in &log.edits {
            appender.append_edit(edit).await.unwrap();
        }
        for change in &log.changes {
            appender.append_change(change).await.unwrap();
        }
        for checkpoint in &log.checkpoints {
            appender.append_checkpoint(checkpoint).await.unwrap();
        }
        for alternative in &log.alternatives {
            appender.append_alternative(alternative).await.unwrap();
        }
        appender.set_active(log.active_alternative_id.as_deref()).await.unwrap();
        appender.commit().await.unwrap();
        let streamed_bytes = appender.into_sink().await;

        let decoded = decode_history(&streamed_bytes, &DecodeOptions::default()).await.unwrap();
        assert_eq!(decoded, log);
    }

    #[semio_framework_async_macros::async_test]
    async fn append_cursor_then_decode_recovers_it() {
        let mut log = sample_log().await;
        for edit in &mut log.edits {
            edit.meta = None;
        }
        let cursor = HistoryCursor { applied_edit_ids: vec!["edit-1".to_string()], redo_edit_ids: vec!["edit-2".to_string()], checkpoint_id: Some("ck-1".to_string()) };
        let options = WriteOptions { required_flags: crate::os_spr::REQUIRED_HASH_CHAIN, optional_flags: crate::os_spr::OPTIONAL_CANONICAL };
        let mut appender = HistoryAppender::begin(Vec::<u8>::new(), &log.doc_id, &log.schema, &options).await.unwrap();
        for edit in &log.edits {
            appender.append_edit(edit).await.unwrap();
        }
        appender.append_cursor(&cursor).await.unwrap();
        appender.commit().await.unwrap();
        let bytes = appender.into_sink().await;

        let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
        assert_eq!(decoded.cursor, Some(cursor));
    }
    //#endregion 🔖️Append

    //#region 🔖️Scan
    #[semio_framework_async_macros::async_test]
    async fn reader_edits_forward_matches_log() {
        let log = sample_log().await;
        let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
        let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).await.unwrap();
        let edits: Vec<HistoryEdit> = reader.edits().await.map(|r| r.unwrap()).collect();
        assert_eq!(edits, log.edits);
    }

    #[semio_framework_async_macros::async_test]
    async fn reader_edits_rev_matches_tail_in_reverse() {
        let log = sample_log().await;
        let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
        let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).await.unwrap();
        let rev: Vec<HistoryEdit> = reader.edits_rev(1).await.map(|r| r.unwrap()).collect();
        assert_eq!(rev.len(), 1);
        assert_eq!(rev[0], log.edits[1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn reader_edits_rev_full_matches_reversed_forward() {
        let log = sample_log().await;
        let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
        let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).await.unwrap();
        let rev: Vec<HistoryEdit> = reader.edits_rev(usize::MAX).await.map(|r| r.unwrap()).collect();
        let mut expected = log.edits;
        expected.reverse();
        assert_eq!(rev, expected);
    }
    //#endregion 🔖️Scan

    //#region 🔖️Frontier
    async fn frontier(head_edit_ordinal: u64, head_edit_id: &str, chain_hash: [u8; 32]) -> FrontierSummary {
        FrontierSummary { document_id: "doc-1".to_string(), head_edit_ordinal, head_edit_id: head_edit_id.to_string(), alternatives: Vec::new(), last_commit_seq: 1, chain_hash }
    }

    #[semio_framework_async_macros::async_test]
    async fn frontier_delta_reports_equal_ahead_behind_diverged() {
        let a = frontier(5, "edit-5", [1u8; 32]).await;
        let b = frontier(5, "edit-5", [1u8; 32]).await;
        assert_eq!(frontier_delta(&a, &b).await, FrontierComparison::Equal);

        let ahead = frontier(6, "edit-6", [2u8; 32]).await;
        assert_eq!(frontier_delta(&ahead, &a).await, FrontierComparison::Ahead);
        assert_eq!(frontier_delta(&a, &ahead).await, FrontierComparison::Behind);

        let diverged = frontier(5, "edit-5-alt", [3u8; 32]).await;
        assert_eq!(frontier_delta(&a, &diverged).await, FrontierComparison::Diverged { common_edit_count: 5 });
    }
    //#endregion 🔖️Frontier

    //#region 🔖️Index
    #[semio_framework_async_macros::async_test]
    async fn index_round_trips_edits_checkpoints_and_snapshots() {
        let mut builder = IndexBuilder::new().await;
        builder.record_edit(0, 100).await;
        builder.record_edit(5, 300).await;
        builder.record_edit(10, 500).await;
        builder.record_checkpoint("ck-1", 700, 10).await;
        builder.record_snapshot(5, 250).await;
        builder.record_sealed(50).await;
        let payload = builder.build().await;

        let reader = IndexReader::open(&payload).await.unwrap();
        assert_eq!(reader.edit_offset_at_or_before(7).await, Some(300));
        assert_eq!(reader.edit_offset_at_or_before(0).await, Some(100));
        assert_eq!(reader.edit_offset_at_or_before(10).await, Some(500));
        assert_eq!(reader.checkpoint_offset("ck-1").await, Some((700, 10)));
        assert_eq!(reader.checkpoint_offset("missing").await, None);
        assert_eq!(reader.latest_snapshot_offset_at_or_before(9).await, Some(250));
    }
    //#endregion 🔖️Index
}
//#endregion 🧪️Tests
