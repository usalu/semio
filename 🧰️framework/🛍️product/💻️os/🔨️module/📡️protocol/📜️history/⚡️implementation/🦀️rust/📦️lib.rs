//! 🎞️ Protocol history log: the typed record model (`HistoryLog` and friends), the `.ops` text
//! grammar twin (built directly on `dsl_schema`, never on `vcs`), per-kind binary payload codecs
//! (built on `protocol_core::scalar` + `protocol_format`'s frame writer/reader), the whole-file
//! codec, a streaming append API, and a lazy forward/reverse scan API. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol_history`).
//!
//! Op payloads are opaque validated bytes to this crate — it stores, hashes, frames, and indexes
//! them, but never interprets operation semantics (that is `protocol_command`'s concern, a sibling
//! crate this one does not depend on).

use dsl_schema::{FieldSpec, FieldValue, JoinMode, ParseOptions, RecordLayout, RecordSpec, RecordValue, Shape};
use pack_core::{ByteReader, ByteWriter, CodecId, PackSink};
use protocol_core::{DictBuilder, DictReader, ProtocolError, ProtocolLimits, RecordHasher};
use protocol_format::{Blake3Hasher, FrameCursor, HEADER_SIZE, RecoveryMode, ReverseFrameCursor, SprWriter, VerificationLevel, WriteOptions};
use std::collections::HashMap;

//#region 🔖️Model
// Every field of store::OpsHeaderLine (Doc/Edit/Change/Checkpoint/Alternative/Active) has exactly
// one slot below. Op lines are opaque exact `print_op` strings (one per line, no '\n' inside).
// Derived data (backwards, sequence_number, unless explicitly captured via `meta`) is excluded.
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
    /// @emoji 🔙️ The edit's inverse operations, in apply order (mirrors `protocol_command::Edit
    /// ::backwards`). Empty for text-compiled/imported logs — a decoder recomputing them from a
    /// fresh replay never touches this field; when non-empty, `write_backwards_section` persisted
    /// them explicitly (only the `.spr` binary path ever sets this — the `.ops` text mirror stays
    /// forwards-only, see `store::print_document_spr`/`parse_document_spr`).
    pub backwards: Vec<OpPayload>,
    /// @emoji 🧮️ Present iff the caller supplied it; absent for text-compiled/imported logs. Not
    /// required for round-trip — a decoder recomputing backwards/meta from a fresh replay never
    /// touches this field.
    pub meta: Option<Vec<HistoryOpMeta>>,
}

/// @emoji 🧾️ `binary` carries the `protocol_command::OpBinary` encoding of this op when the
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
/// `DocumentStore::current_checkpoint_id`; the active alternative stays on the existing
/// `HistoryLog::active_alternative_id` (unrelated lifecycle — churns far less often).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryCursor {
    pub applied_edit_ids: Vec<String>,
    pub redo_edit_ids: Vec<String>,
    pub checkpoint_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryOpMeta {
    pub op_id: Option<String>,
    pub dependencies: Vec<String>,
    pub base_version: u64,
    pub author_id: Option<String>,
    pub hlt: Option<(u64, i64, u64)>,
    pub undo_policy: u8,
    pub payload_hash: Option<[u8; 32]>,
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
// Own twin of store::OpsHeaderLine's grammar, built directly against `dsl_schema` (never `vcs`,
// never `dsl_derive` — this crate has no path dep on either). Field declaration order below
// mirrors vcs's struct field order exactly: `dsl_schema::print_record` reorders keyed fields
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
fn author_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(F_AUTHOR_ID, "", Shape::Text).positional(0), FieldSpec::new(F_AUTHOR_NAME, "", Shape::Text).positional(1)])
}

fn doc_spec() -> RecordSpec {
    RecordSpec::new(Some("doc"), RecordLayout::Inline, vec![FieldSpec::new(F_DOC_ID, "", Shape::Text).positional(0), FieldSpec::new(F_DOC_SCHEMA, "schema", Shape::Text)])
}

fn edit_spec() -> RecordSpec {
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

fn change_spec() -> RecordSpec {
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

fn checkpoint_spec() -> RecordSpec {
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

fn alternative_spec() -> RecordSpec {
    RecordSpec::new(
        Some("alternative"),
        RecordLayout::Inline,
        vec![
            FieldSpec::new(F_ALTERNATIVE_ID, "", Shape::Text).positional(0),
            FieldSpec::new(F_ALTERNATIVE_NAME, "name", Shape::Text),
            FieldSpec::new(F_ALTERNATIVE_CHECKPOINTS, "checkpoints", Shape::List(Box::new(Shape::Text))),
        ],
    )
}

fn active_spec() -> RecordSpec {
    RecordSpec::new(Some("active"), RecordLayout::Inline, vec![FieldSpec::new(F_ACTIVE_ID, "", Shape::Text).positional(0)])
}

/// @emoji 🎯️ `cursor applied=[...] redo=[...] checkpoint=<id>` — carries the FULL applied/redo
/// edit-id lists (see `HistoryCursor`'s doc for why a single marker id is insufficient).
fn cursor_spec() -> RecordSpec {
    RecordSpec::new(
        Some("cursor"),
        RecordLayout::Inline,
        vec![
            FieldSpec::new(F_CURSOR_APPLIED, "applied", Shape::List(Box::new(Shape::Text))),
            FieldSpec::new(F_CURSOR_REDO, "redo", Shape::List(Box::new(Shape::Text))),
            FieldSpec::new(F_CURSOR_CHECKPOINT, "checkpoint", Shape::Text).optional(),
        ],
    )
}

fn record_with(fields: Vec<(u16, FieldValue)>) -> RecordValue {
    RecordValue { fields: fields.into_iter().collect() }
}

fn field_text(record: &RecordValue, id: u16) -> Option<String> {
    match record.get(id) {
        Some(FieldValue::Text(s)) => Some(s.clone()),
        _ => None,
    }
}

fn required_text(record: &RecordValue, id: u16, what: &'static str) -> Result<String, ProtocolError> {
    field_text(record, id).ok_or_else(|| ProtocolError::Malformed { what, offset: 0, detail: "missing required field in ops text".to_string() })
}

fn field_text_list(record: &RecordValue, id: u16) -> Vec<String> {
    match record.get(id) {
        Some(FieldValue::List(items)) => items.iter().filter_map(|v| if let FieldValue::Text(s) = v { Some(s.clone()) } else { None }).collect(),
        _ => Vec::new(),
    }
}

fn field_authors(record: &RecordValue, id: u16) -> Vec<HistoryAuthor> {
    match record.get(id) {
        Some(FieldValue::List(items)) => items
            .iter()
            .filter_map(|v| {
                let FieldValue::Record(rec) = v else { return None };
                Some(HistoryAuthor { id: field_text(rec, F_AUTHOR_ID)?, name: field_text(rec, F_AUTHOR_NAME)? })
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn text_error_to_protocol(err: dsl_core::TextError) -> ProtocolError {
    ProtocolError::Malformed { what: "ops text", offset: err.span.line as u64, detail: err.message }
}

/// @emoji 📥️ Parses the full `.ops` text into a `HistoryLog`. Blank lines and `#`-comments
/// normalize away; a two-space-indented line under a pending `edit` header is an opaque forward
/// op line (never interpreted). Unlike `store::replay_ops`, this never replays operation semantics
/// (ops are opaque here) — `HistoryEdit::meta`/backwards are simply never populated from text.
pub fn parse_ops_text(ops: &str) -> Result<HistoryLog, ProtocolError> {
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

    fn flush(pending: &mut Option<PendingEdit>, forwards: &mut Vec<OpPayload>, edits: &mut Vec<HistoryEdit>) {
        if let Some(header) = pending.take() {
            edits.push(HistoryEdit {
                id: header.id,
                actor: header.actor,
                started_at: header.started_at,
                finished_at: header.finished_at,
                coalesce_key: header.coalesce_key,
                description: header.description,
                ops: std::mem::take(forwards),
                backwards: Vec::new(),
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
        flush(&mut pending, &mut forwards, &mut log.edits);

        let opts = ParseOptions::default();
        let keyword = trimmed.split_whitespace().next().unwrap_or("");
        match keyword {
            "doc" => {
                let record = dsl_schema::parse(trimmed, &doc_spec(), &opts).map_err(text_error_to_protocol)?;
                log.doc_id = required_text(&record, F_DOC_ID, "doc id")?;
                log.schema = required_text(&record, F_DOC_SCHEMA, "doc schema")?;
            }
            "edit" => {
                let record = dsl_schema::parse(trimmed, &edit_spec(), &opts).map_err(text_error_to_protocol)?;
                pending = Some(PendingEdit {
                    id: required_text(&record, F_EDIT_ID, "edit id")?,
                    started_at: required_text(&record, F_EDIT_STARTED, "edit started")?,
                    actor: field_text(&record, F_EDIT_ACTOR),
                    finished_at: field_text(&record, F_EDIT_FINISHED),
                    coalesce_key: field_text(&record, F_EDIT_KEY),
                    description: field_text(&record, F_EDIT_DESCRIPTION),
                });
                forwards = Vec::new();
            }
            "change" => {
                let record = dsl_schema::parse(trimmed, &change_spec(), &opts).map_err(text_error_to_protocol)?;
                log.changes.push(HistoryChange {
                    id: required_text(&record, F_CHANGE_ID, "change id")?,
                    saved_at: required_text(&record, F_CHANGE_SAVED, "change saved")?,
                    edit_ids: field_text_list(&record, F_CHANGE_EDITS),
                    description: field_text(&record, F_CHANGE_DESCRIPTION),
                });
            }
            "checkpoint" => {
                let record = dsl_schema::parse(trimmed, &checkpoint_spec(), &opts).map_err(text_error_to_protocol)?;
                log.checkpoints.push(HistoryCheckpoint {
                    id: required_text(&record, F_CHECKPOINT_ID, "checkpoint id")?,
                    timestamp: required_text(&record, F_CHECKPOINT_AT, "checkpoint at")?,
                    change_ids: field_text_list(&record, F_CHECKPOINT_CHANGES),
                    parent_id: field_text(&record, F_CHECKPOINT_PARENT),
                    authors: field_authors(&record, F_CHECKPOINT_BY),
                    message: field_text(&record, F_CHECKPOINT_MESSAGE),
                });
            }
            "alternative" => {
                let record = dsl_schema::parse(trimmed, &alternative_spec(), &opts).map_err(text_error_to_protocol)?;
                log.alternatives.push(HistoryAlternative {
                    id: required_text(&record, F_ALTERNATIVE_ID, "alternative id")?,
                    name: required_text(&record, F_ALTERNATIVE_NAME, "alternative name")?,
                    checkpoint_ids: field_text_list(&record, F_ALTERNATIVE_CHECKPOINTS),
                });
            }
            "active" => {
                let record = dsl_schema::parse(trimmed, &active_spec(), &opts).map_err(text_error_to_protocol)?;
                log.active_alternative_id = Some(required_text(&record, F_ACTIVE_ID, "active id")?);
            }
            "cursor" => {
                let record = dsl_schema::parse(trimmed, &cursor_spec(), &opts).map_err(text_error_to_protocol)?;
                log.cursor = Some(HistoryCursor {
                    applied_edit_ids: field_text_list(&record, F_CURSOR_APPLIED),
                    redo_edit_ids: field_text_list(&record, F_CURSOR_REDO),
                    checkpoint_id: field_text(&record, F_CURSOR_CHECKPOINT),
                });
            }
            other => return Err(ProtocolError::Malformed { what: "ops text line", offset: 0, detail: format!("unknown line keyword '{other}'") }),
        }
    }
    flush(&mut pending, &mut forwards, &mut log.edits);
    Ok(log)
}

/// @emoji 📤️ Prints a `HistoryLog` back to `.ops` text: `doc`, every edit (header + two-space
/// indented forward op lines), then `change`/`checkpoint`/`alternative`/`active` records — the
/// same section order `store::print_ops_log` uses. Errors if any op payload carries no text
/// (the binary-only `.spr` convention): this crate is schema-agnostic and cannot recover text
/// from an opaque binary payload — printing `.ops` for a real app document goes through the
/// concrete `Operation::print_op` path instead (`store::print_document_pack`'s `.ops` mirror).
pub fn print_ops_text(log: &HistoryLog) -> Result<String, ProtocolError> {
    let mut out = String::new();

    let doc_record = record_with(vec![(F_DOC_ID, FieldValue::Text(log.doc_id.clone())), (F_DOC_SCHEMA, FieldValue::Text(log.schema.clone()))]);
    out.push_str(&dsl_schema::print(&doc_record, &doc_spec(), JoinMode::Inline));
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
        out.push_str(&dsl_schema::print(&record_with(fields), &edit_spec(), JoinMode::Inline));
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
        let mut fields = vec![
            (F_CHANGE_ID, FieldValue::Text(change.id.clone())),
            (F_CHANGE_SAVED, FieldValue::Text(change.saved_at.clone())),
            (F_CHANGE_EDITS, FieldValue::List(change.edit_ids.iter().map(|s| FieldValue::Text(s.clone())).collect())),
        ];
        if let Some(description) = &change.description {
            fields.push((F_CHANGE_DESCRIPTION, FieldValue::Text(description.clone())));
        }
        out.push_str(&dsl_schema::print(&record_with(fields), &change_spec(), JoinMode::Inline));
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
        fields.push((
            F_CHECKPOINT_BY,
            FieldValue::List(
                checkpoint
                    .authors
                    .iter()
                    .map(|a| FieldValue::Record(record_with(vec![(F_AUTHOR_ID, FieldValue::Text(a.id.clone())), (F_AUTHOR_NAME, FieldValue::Text(a.name.clone()))])))
                    .collect(),
            ),
        ));
        if let Some(message) = &checkpoint.message {
            fields.push((F_CHECKPOINT_MESSAGE, FieldValue::Text(message.clone())));
        }
        out.push_str(&dsl_schema::print(&record_with(fields), &checkpoint_spec(), JoinMode::Inline));
        out.push('\n');
    }

    for alternative in &log.alternatives {
        let fields = vec![
            (F_ALTERNATIVE_ID, FieldValue::Text(alternative.id.clone())),
            (F_ALTERNATIVE_NAME, FieldValue::Text(alternative.name.clone())),
            (F_ALTERNATIVE_CHECKPOINTS, FieldValue::List(alternative.checkpoint_ids.iter().map(|s| FieldValue::Text(s.clone())).collect())),
        ];
        out.push_str(&dsl_schema::print(&record_with(fields), &alternative_spec(), JoinMode::Inline));
        out.push('\n');
    }

    if let Some(active_id) = &log.active_alternative_id {
        out.push_str(&dsl_schema::print(&record_with(vec![(F_ACTIVE_ID, FieldValue::Text(active_id.clone()))]), &active_spec(), JoinMode::Inline));
        out.push('\n');
    }

    if let Some(cursor) = &log.cursor {
        let mut fields = vec![
            (F_CURSOR_APPLIED, FieldValue::List(cursor.applied_edit_ids.iter().map(|s| FieldValue::Text(s.clone())).collect())),
            (F_CURSOR_REDO, FieldValue::List(cursor.redo_edit_ids.iter().map(|s| FieldValue::Text(s.clone())).collect())),
        ];
        if let Some(checkpoint_id) = &cursor.checkpoint_id {
            fields.push((F_CURSOR_CHECKPOINT, FieldValue::Text(checkpoint_id.clone())));
        }
        out.push_str(&dsl_schema::print(&record_with(fields), &cursor_spec(), JoinMode::Inline));
        out.push('\n');
    }

    Ok(out)
}
//#endregion 🔖️TextGrammar

//#region 🔖️Payloads
// Binary codec for each record kind, using protocol_core::scalar + protocol_format's frame
// writer/reader. Every payload starts `format: u8` (=1); trailing bytes are ignored on read
// (additive-evolution slot) except a critical record demands `format <= known` (all kinds here
// are critical per protocol_core::is_critical_kind, so every decode_* rejects format > 1).
//
// 🎯️ Design choices (contract leaves these to the implementer, documented once here):
// - Every encode_*/decode_* pair takes a SINGLE `DictBuilder`/`DictReader` (matching the frozen
//   encode_doc/encode_edit signatures, which only expose one `dict` parameter each) — this crate
//   backs `REC_STR_DICT` only; `REC_ACTOR_DICT` stays defined in `protocol_core` but is never
//   emitted by this crate's writer (a no-op skip on read, for forward compatibility).
// - `encode_change`'s `edit_ordinal_of` is the only place besides `encode_edit` that genuinely
//   references edit ids (`HistoryChange::edit_ids`); `encode_checkpoint`/`encode_alternative`/
//   `encode_active` take no `edit_ordinal_of` since they never reference an edit.
// - `encode_edit` itself is data-driven: it writes presence bit5 + the backwards section iff
//   `edit.backwards` is non-empty — real op payloads, using the same op-payload wire shape as
//   `edit.ops` (op_tag bit1 flags a binary payload; both tags are per-payload, not per-edit, so
//   text-only and binary-carrying ops may mix freely within one edit). `EncodeOptions::
//   write_backwards_section` is the batch-level policy switch `encode_history` applies on top
//   (stripping `edit.backwards` before encoding when false, even if the caller's `HistoryLog`
//   has it populated) — `encode_edit`/`HistoryAppender::append_edit` have no such switch; a
//   streaming caller controls persistence per edit via the data it hands in. A decoder never
//   assumes backwards are present and always recomputes them via replay when the section (or the
//   whole `HistoryLog`) is absent.
// - Every `Option<T>` field not already covered by a record-level presence bitmask (i.e. every
//   field inside one `HistoryOpMeta` entry) gets its own bitmask byte, described per-function.

fn malformed_fmt(what: &'static str, format: u8) -> ProtocolError {
    ProtocolError::Malformed { what, offset: 0, detail: format!("unsupported format {format}") }
}

fn write_str_field(out: &mut ByteWriter, s: &str) {
    out.write_varint_u64(s.len() as u64);
    out.write_bytes(s.as_bytes());
}

fn read_str_field(input: &mut ByteReader<'_>) -> Result<String, ProtocolError> {
    let len = input.read_varint_u64()? as usize;
    let bytes = input.read_bytes(len)?;
    std::str::from_utf8(bytes).map(str::to_string).map_err(|_| ProtocolError::Malformed { what: "utf8", offset: 0, detail: "invalid utf-8".to_string() })
}

fn write_id_field(out: &mut ByteWriter, id: &str, dict: &mut DictBuilder, edit_ordinal_of: &dyn Fn(&str) -> Option<u64>) -> Result<(), ProtocolError> {
    protocol_core::scalar::write_id(out, id, |s| dict.intern(s), edit_ordinal_of).map_err(ProtocolError::from)
}

fn read_id_field<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<String, ProtocolError> {
    protocol_core::scalar::read_id(
        input,
        |idx: u32| dict.resolve(idx).map_err(|_| pack_core::PackError::Malformed { what: "dict index", offset: idx as u64, detail: "out of range".to_string() }),
        |ord: u64| ordinal_to_id(ord).map_err(|_| pack_core::PackError::Malformed { what: "edit ordinal", offset: ord, detail: "unresolvable".to_string() }),
    )
    .map_err(ProtocolError::from)
}

//#region 🔖️Doc
pub fn encode_doc(doc_id: &str, schema: &str, dict: &mut DictBuilder) -> Vec<u8> {
    let mut out = ByteWriter::new();
    out.write_u8(1);
    write_id_field(&mut out, doc_id, dict, &|_: &str| None).expect("write_id never fails for an in-memory ByteWriter");
    write_id_field(&mut out, schema, dict, &|_: &str| None).expect("write_id never fails for an in-memory ByteWriter");
    out.into_bytes()
}

pub fn decode_doc(payload: &[u8], dict: &DictReader) -> Result<(String, String), ProtocolError> {
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("doc", format));
    }
    let doc_id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    let schema = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    Ok((doc_id, schema))
}
//#endregion 🔖️Doc

//#region 🔖️Edit
// REC_EDIT layout: format u8, presence u8 (bit0 actor, bit1 finished, bit2 key, bit3 description,
// bit4 explicit_meta, bit5 has_backwards_section), id, started(ts), [actor(dictref)],
// [finished(ts)], [key(str)], [description(str)], op_count varint, op_count x op-payload (see
// write_op_payload: op_tag u8 [bit0 has_text=1 required in v1, bit1 has_binary] + text_len varint
// + utf8 + [binary_len varint + bytes iff bit1]), [iff bit5: back_count varint + back_count x
// op-payload (backwards, in apply order)], [explicit_meta iff bit4: op_count x op-meta entry (see
// write_op_meta) — always keyed by op_count, never back_count, since meta describes the forward
// ops only].

/// @emoji 🎯️ Writes one op payload: `op_tag u8 [bit0 has_text=1 required in v1, bit1 has_binary]
/// + text_len varint + utf8 + [binary_len varint + bytes iff bit1]`. Used for both `edit.ops`
/// and `edit.backwards` — the two sections share this exact wire shape.
fn write_op_payload(out: &mut ByteWriter, op: &OpPayload) -> Result<(), ProtocolError> {
    if op.text.is_none() && op.binary.is_none() {
        return Err(ProtocolError::Malformed { what: "op payload", offset: 0, detail: "requires text or binary".to_string() });
    }
    let tag = (op.text.is_some() as u8) | ((op.binary.is_some() as u8) << 1);
    out.write_u8(tag);
    if let Some(text) = &op.text {
        write_str_field(out, text);
    }
    if let Some(binary) = &op.binary {
        out.write_varint_u64(binary.len() as u64);
        out.write_bytes(binary);
    }
    Ok(())
}

/// @emoji 🎯️ Inverse of [`write_op_payload`].
fn read_op_payload(input: &mut ByteReader<'_>) -> Result<OpPayload, ProtocolError> {
    let op_tag = input.read_u8()?;
    if op_tag & 0b11 == 0 {
        return Err(ProtocolError::Malformed { what: "op payload", offset: 0, detail: "requires text or binary bit set".to_string() });
    }
    let text = if op_tag & 0b01 != 0 { Some(read_str_field(input)?) } else { None };
    let binary = if op_tag & 0b10 != 0 {
        let len = input.read_varint_u64()? as usize;
        Some(input.read_bytes(len)?.to_vec())
    } else {
        None
    };
    Ok(OpPayload { text, binary })
}

fn write_op_meta(out: &mut ByteWriter, meta: &HistoryOpMeta, dict: &mut DictBuilder, edit_ordinal_of: &dyn Fn(&str) -> Option<u64>) -> Result<(), ProtocolError> {
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
    out.write_u8(presence);
    if let Some(op_id) = &meta.op_id {
        write_id_field(out, op_id, dict, edit_ordinal_of)?;
    }
    out.write_varint_u64(meta.dependencies.len() as u64);
    for dep in &meta.dependencies {
        write_id_field(out, dep, dict, edit_ordinal_of)?;
    }
    out.write_varint_u64(meta.base_version);
    if let Some(author) = &meta.author_id {
        write_id_field(out, author, dict, edit_ordinal_of)?;
    }
    if let Some((actor, physical_ms, logical)) = &meta.hlt {
        out.write_varint_u64(*actor);
        out.write_varint_i64(*physical_ms);
        out.write_varint_u64(*logical);
    }
    out.write_u8(meta.undo_policy);
    if let Some(hash) = &meta.payload_hash {
        out.write_bytes(hash);
    }
    Ok(())
}

fn read_op_meta<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryOpMeta, ProtocolError> {
    let presence = input.read_u8()?;
    let op_id = if presence & (1 << 0) != 0 { Some(read_id_field(input, dict, ordinal_to_id)?) } else { None };
    let dep_count = input.read_varint_u64()?;
    let mut dependencies = Vec::with_capacity(dep_count as usize);
    for _ in 0..dep_count {
        dependencies.push(read_id_field(input, dict, ordinal_to_id)?);
    }
    let base_version = input.read_varint_u64()?;
    let author_id = if presence & (1 << 1) != 0 { Some(read_id_field(input, dict, ordinal_to_id)?) } else { None };
    let hlt = if presence & (1 << 2) != 0 {
        let actor = input.read_varint_u64()?;
        let physical_ms = input.read_varint_i64()?;
        let logical = input.read_varint_u64()?;
        Some((actor, physical_ms, logical))
    } else {
        None
    };
    let undo_policy = input.read_u8()?;
    let payload_hash = if presence & (1 << 3) != 0 { Some(input.read_array32()?) } else { None };
    Ok(HistoryOpMeta { op_id, dependencies, base_version, author_id, hlt, undo_policy, payload_hash })
}

pub fn encode_edit(edit: &HistoryEdit, dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &dyn Fn(&str) -> Option<u64> = &edit_ordinal_of;
    let mut out = ByteWriter::new();
    out.write_u8(1);
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
    if !edit.backwards.is_empty() {
        presence |= 1 << 5;
    }
    out.write_u8(presence);
    write_id_field(&mut out, &edit.id, dict, &|_: &str| None)?;
    let mut prev_epoch_ms = protocol_core::scalar::write_timestamp(&mut out, &edit.started_at, None);
    if let Some(actor) = &edit.actor {
        write_id_field(&mut out, actor, dict, edit_ordinal_of)?;
    }
    if let Some(finished) = &edit.finished_at {
        prev_epoch_ms = protocol_core::scalar::write_timestamp(&mut out, finished, prev_epoch_ms);
    }
    let _ = prev_epoch_ms;
    if let Some(key) = &edit.coalesce_key {
        write_str_field(&mut out, key);
    }
    if let Some(description) = &edit.description {
        write_str_field(&mut out, description);
    }
    if edit.ops.len() as u64 > ProtocolLimits::default().max_op_count_per_edit as u64 {
        return Err(ProtocolError::LimitExceeded("edit op count exceeds ProtocolLimits::max_op_count_per_edit"));
    }
    out.write_varint_u64(edit.ops.len() as u64);
    for op in &edit.ops {
        write_op_payload(&mut out, op)?;
    }
    if !edit.backwards.is_empty() {
        out.write_varint_u64(edit.backwards.len() as u64);
        for op in &edit.backwards {
            write_op_payload(&mut out, op)?;
        }
    }
    if let Some(metas) = &edit.meta {
        if metas.len() != edit.ops.len() {
            return Err(ProtocolError::Malformed { what: "edit meta", offset: 0, detail: "explicit meta length must match op count".to_string() });
        }
        for meta in metas {
            write_op_meta(&mut out, meta, dict, edit_ordinal_of)?;
        }
    }
    Ok(out.into_bytes())
}

pub fn decode_edit<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryEdit, ProtocolError> {
    let ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError> = &ordinal_to_id;
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("edit", format));
    }
    let presence = input.read_u8()?;
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    let (started_at, mut prev_epoch_ms) = protocol_core::scalar::read_timestamp(&mut input, None)?;
    let actor = if presence & (1 << 0) != 0 { Some(read_id_field(&mut input, dict, ordinal_to_id)?) } else { None };
    let finished_at = if presence & (1 << 1) != 0 {
        let (s, p) = protocol_core::scalar::read_timestamp(&mut input, prev_epoch_ms)?;
        prev_epoch_ms = p;
        Some(s)
    } else {
        None
    };
    let _ = prev_epoch_ms;
    let coalesce_key = if presence & (1 << 2) != 0 { Some(read_str_field(&mut input)?) } else { None };
    let description = if presence & (1 << 3) != 0 { Some(read_str_field(&mut input)?) } else { None };
    let op_count = input.read_varint_u64()?;
    let max_ops = ProtocolLimits::default().max_op_count_per_edit as u64;
    if op_count > max_ops {
        return Err(ProtocolError::LimitExceeded("edit op count exceeds ProtocolLimits::max_op_count_per_edit"));
    }
    let mut ops = Vec::with_capacity(op_count as usize);
    for _ in 0..op_count {
        ops.push(read_op_payload(&mut input)?);
    }
    let backwards = if presence & (1 << 5) != 0 {
        let back_count = input.read_varint_u64()?;
        if back_count > max_ops {
            return Err(ProtocolError::LimitExceeded("edit backwards op count exceeds ProtocolLimits::max_op_count_per_edit"));
        }
        let mut backs = Vec::with_capacity(back_count as usize);
        for _ in 0..back_count {
            backs.push(read_op_payload(&mut input)?);
        }
        backs
    } else {
        Vec::new()
    };
    let meta = if presence & (1 << 4) != 0 {
        let mut metas = Vec::with_capacity(op_count as usize);
        for _ in 0..op_count {
            metas.push(read_op_meta(&mut input, dict, ordinal_to_id)?);
        }
        Some(metas)
    } else {
        None
    };
    Ok(HistoryEdit { id, actor, started_at, finished_at, coalesce_key, description, ops, backwards, meta })
}
//#endregion 🔖️Edit

//#region 🔖️Change
pub fn encode_change(change: &HistoryChange, dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &dyn Fn(&str) -> Option<u64> = &edit_ordinal_of;
    let mut out = ByteWriter::new();
    out.write_u8(1);
    let mut presence = 0u8;
    if change.description.is_some() {
        presence |= 1 << 0;
    }
    out.write_u8(presence);
    write_id_field(&mut out, &change.id, dict, &|_: &str| None)?;
    protocol_core::scalar::write_timestamp(&mut out, &change.saved_at, None);
    out.write_varint_u64(change.edit_ids.len() as u64);
    for edit_id in &change.edit_ids {
        write_id_field(&mut out, edit_id, dict, edit_ordinal_of)?;
    }
    if let Some(description) = &change.description {
        write_str_field(&mut out, description);
    }
    Ok(out.into_bytes())
}

pub fn decode_change<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryChange, ProtocolError> {
    let ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError> = &ordinal_to_id;
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("change", format));
    }
    let presence = input.read_u8()?;
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    let (saved_at, _) = protocol_core::scalar::read_timestamp(&mut input, None)?;
    let edit_count = input.read_varint_u64()?;
    let mut edit_ids = Vec::with_capacity(edit_count as usize);
    for _ in 0..edit_count {
        edit_ids.push(read_id_field(&mut input, dict, ordinal_to_id)?);
    }
    let description = if presence & (1 << 0) != 0 { Some(read_str_field(&mut input)?) } else { None };
    Ok(HistoryChange { id, saved_at, edit_ids, description })
}
//#endregion 🔖️Change

//#region 🔖️Checkpoint
pub fn encode_checkpoint(checkpoint: &HistoryCheckpoint, dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let mut out = ByteWriter::new();
    out.write_u8(1);
    let mut presence = 0u8;
    if checkpoint.parent_id.is_some() {
        presence |= 1 << 0;
    }
    if checkpoint.message.is_some() {
        presence |= 1 << 1;
    }
    out.write_u8(presence);
    write_id_field(&mut out, &checkpoint.id, dict, &|_: &str| None)?;
    protocol_core::scalar::write_timestamp(&mut out, &checkpoint.timestamp, None);
    out.write_varint_u64(checkpoint.change_ids.len() as u64);
    for change_id in &checkpoint.change_ids {
        write_id_field(&mut out, change_id, dict, &|_: &str| None)?;
    }
    if let Some(parent) = &checkpoint.parent_id {
        write_id_field(&mut out, parent, dict, &|_: &str| None)?;
    }
    out.write_varint_u64(checkpoint.authors.len() as u64);
    for author in &checkpoint.authors {
        write_id_field(&mut out, &author.id, dict, &|_: &str| None)?;
        write_str_field(&mut out, &author.name);
    }
    if let Some(message) = &checkpoint.message {
        write_str_field(&mut out, message);
    }
    Ok(out.into_bytes())
}

pub fn decode_checkpoint(payload: &[u8], dict: &DictReader) -> Result<HistoryCheckpoint, ProtocolError> {
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("checkpoint", format));
    }
    let presence = input.read_u8()?;
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    let (timestamp, _) = protocol_core::scalar::read_timestamp(&mut input, None)?;
    let change_count = input.read_varint_u64()?;
    let mut change_ids = Vec::with_capacity(change_count as usize);
    for _ in 0..change_count {
        change_ids.push(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?);
    }
    let parent_id = if presence & (1 << 0) != 0 { Some(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?) } else { None };
    let author_count = input.read_varint_u64()?;
    let mut authors = Vec::with_capacity(author_count as usize);
    for _ in 0..author_count {
        let author_id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
        let name = read_str_field(&mut input)?;
        authors.push(HistoryAuthor { id: author_id, name });
    }
    let message = if presence & (1 << 1) != 0 { Some(read_str_field(&mut input)?) } else { None };
    Ok(HistoryCheckpoint { id, timestamp, change_ids, parent_id, authors, message })
}
//#endregion 🔖️Checkpoint

//#region 🔖️Alternative
pub fn encode_alternative(alternative: &HistoryAlternative, dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let mut out = ByteWriter::new();
    out.write_u8(1);
    write_id_field(&mut out, &alternative.id, dict, &|_: &str| None)?;
    write_str_field(&mut out, &alternative.name);
    out.write_varint_u64(alternative.checkpoint_ids.len() as u64);
    for checkpoint_id in &alternative.checkpoint_ids {
        write_id_field(&mut out, checkpoint_id, dict, &|_: &str| None)?;
    }
    Ok(out.into_bytes())
}

pub fn decode_alternative(payload: &[u8], dict: &DictReader) -> Result<HistoryAlternative, ProtocolError> {
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("alternative", format));
    }
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    let name = read_str_field(&mut input)?;
    let checkpoint_count = input.read_varint_u64()?;
    let mut checkpoint_ids = Vec::with_capacity(checkpoint_count as usize);
    for _ in 0..checkpoint_count {
        checkpoint_ids.push(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?);
    }
    Ok(HistoryAlternative { id, name, checkpoint_ids })
}
//#endregion 🔖️Alternative

//#region 🔖️Active
pub fn encode_active(alternative_id: Option<&str>, dict: &mut DictBuilder) -> Vec<u8> {
    let mut out = ByteWriter::new();
    out.write_u8(1);
    match alternative_id {
        Some(id) => {
            out.write_u8(1);
            write_id_field(&mut out, id, dict, &|_: &str| None).expect("write_id never fails for an in-memory ByteWriter");
        }
        None => out.write_u8(0),
    }
    out.into_bytes()
}

pub fn decode_active(payload: &[u8], dict: &DictReader) -> Result<Option<String>, ProtocolError> {
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("active", format));
    }
    let presence = input.read_u8()?;
    if presence & 1 != 0 { Ok(Some(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?)) } else { Ok(None) }
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
pub fn encode_cursor(cursor: &HistoryCursor, dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &dyn Fn(&str) -> Option<u64> = &edit_ordinal_of;
    let mut out = ByteWriter::new();
    out.write_u8(1);
    out.write_u8(if cursor.checkpoint_id.is_some() { 1 } else { 0 });
    out.write_varint_u64(cursor.applied_edit_ids.len() as u64);
    for id in &cursor.applied_edit_ids {
        write_id_field(&mut out, id, dict, edit_ordinal_of)?;
    }
    out.write_varint_u64(cursor.redo_edit_ids.len() as u64);
    for id in &cursor.redo_edit_ids {
        write_id_field(&mut out, id, dict, edit_ordinal_of)?;
    }
    if let Some(checkpoint_id) = &cursor.checkpoint_id {
        write_id_field(&mut out, checkpoint_id, dict, &|_: &str| None)?;
    }
    Ok(out.into_bytes())
}

/// @emoji 🎯️ Inverse of [`encode_cursor`].
pub fn decode_cursor<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryCursor, ProtocolError> {
    let ordinal_to_id: &dyn Fn(u64) -> Result<&'d str, ProtocolError> = &ordinal_to_id;
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("cursor", format));
    }
    let presence = input.read_u8()?;
    let applied_count = input.read_varint_u64()?;
    let mut applied_edit_ids = Vec::with_capacity(applied_count as usize);
    for _ in 0..applied_count {
        applied_edit_ids.push(read_id_field(&mut input, dict, ordinal_to_id)?);
    }
    let redo_count = input.read_varint_u64()?;
    let mut redo_edit_ids = Vec::with_capacity(redo_count as usize);
    for _ in 0..redo_count {
        redo_edit_ids.push(read_id_field(&mut input, dict, ordinal_to_id)?);
    }
    let checkpoint_id = if presence & 1 != 0 { Some(read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?) } else { None };
    Ok(HistoryCursor { applied_edit_ids, redo_edit_ids, checkpoint_id })
}
//#endregion 🔖️Cursor
//#endregion 🔖️Payloads

//#region 🔖️Codec
// Whole-file compile: HistoryLog <-> .spr bytes, using protocol_format::SprWriter/FrameCursor.
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

fn flush_dict_delta<S: PackSink>(writer: &mut SprWriter<S>, dict: &DictBuilder, base: &mut u32) -> Result<(), ProtocolError> {
    let len = dict.len();
    if len > *base {
        let entries = dict.entries_since(*base);
        let mut payload = ByteWriter::new();
        payload.write_u8(1);
        payload.write_varint_u64(*base as u64);
        payload.write_varint_u64(entries.len() as u64);
        for entry in entries {
            payload.write_varint_u64(entry.len() as u64);
            payload.write_bytes(entry.as_bytes());
        }
        writer.write_record(protocol_core::REC_STR_DICT, true, &payload.into_bytes(), CodecId(0))?;
        *base = len;
    }
    Ok(())
}

fn apply_dict_record(dict: &mut DictReader, payload: &[u8]) -> Result<(), ProtocolError> {
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("dict", format));
    }
    let base_count = input.read_varint_u64()? as u32;
    let count = input.read_varint_u64()?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(read_str_field(&mut input)?);
    }
    dict.extend(base_count, entries)
}

/// 🎞️ `(commit_seq, chain_hash)` out of a `REC_COMMIT` frame's payload — thin wrapper over
/// `protocol_format::parse_commit_payload` (public since that crate's own follow-up review pass)
/// so `VerificationLevel::Full`'s chain recompute doesn't need this crate's own byte-offset copy.
fn parse_commit_fields(payload: &[u8]) -> Result<(u64, [u8; 32]), ProtocolError> {
    let commit = protocol_format::parse_commit_payload(payload)?;
    Ok((commit.commit_seq, commit.chain_hash))
}

pub fn encode_history(log: &HistoryLog, options: &EncodeOptions) -> Result<Vec<u8>, ProtocolError> {
    if log.edits.len() as u64 > options.limits.max_record_count {
        return Err(ProtocolError::LimitExceeded("edit count exceeds ProtocolLimits::max_record_count"));
    }
    let write_options = WriteOptions { required_flags: protocol_core::REQUIRED_HASH_CHAIN, optional_flags: if options.canonical { protocol_core::OPTIONAL_CANONICAL } else { 0 } };
    let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options)?;
    let mut dict = DictBuilder::new();
    let mut dict_base = 0u32;

    let doc_payload = encode_doc(&log.doc_id, &log.schema, &mut dict);
    flush_dict_delta(&mut writer, &dict, &mut dict_base)?;
    writer.write_record(protocol_core::REC_DOC, true, &doc_payload, CodecId(0))?;

    // 🎯️ Built incrementally (an edit's own id is inserted only AFTER it is encoded), matching
    // `HistoryAppender::append_edit`'s streaming semantics and the decoder's causal resolution
    // (`EditIter`/`prescan_full` only ever know edits already decoded). A one-shot, whole-list
    // `ordinals` map would let an edit's own `operation_meta[i].operation_id` — legitimately equal
    // to `edit.id` for a single-op edit — resolve to a self-referencing ordinal at encode time,
    // which the decoder can never resolve (it hasn't registered the current edit's id yet).
    let mut ordinals: HashMap<&str, u64> = HashMap::new();
    for (index, edit) in log.edits.iter().enumerate() {
        // 🎯️ `write_backwards_section` is a batch-level policy switch: even when `edit.backwards`
        // is populated (e.g. by a live store that always computes it), a caller can opt out of
        // persisting it here. `HistoryAppender::append_edit` has no such switch — its streaming,
        // one-edit-at-a-time API gives the caller direct per-edit control via the data itself.
        let payload = if options.write_backwards_section {
            encode_edit(edit, &mut dict, |id| ordinals.get(id).copied())?
        } else {
            let stripped = HistoryEdit { backwards: Vec::new(), ..edit.clone() };
            encode_edit(&stripped, &mut dict, |id| ordinals.get(id).copied())?
        };
        flush_dict_delta(&mut writer, &dict, &mut dict_base)?;
        writer.write_record(protocol_core::REC_EDIT, true, &payload, CodecId(0))?;
        ordinals.insert(edit.id.as_str(), index as u64);
    }
    for change in &log.changes {
        let payload = encode_change(change, &mut dict, |id| ordinals.get(id).copied())?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base)?;
        writer.write_record(protocol_core::REC_CHANGE, true, &payload, CodecId(0))?;
    }
    for checkpoint in &log.checkpoints {
        let payload = encode_checkpoint(checkpoint, &mut dict)?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base)?;
        writer.write_record(protocol_core::REC_CHECKPOINT, true, &payload, CodecId(0))?;
    }
    for alternative in &log.alternatives {
        let payload = encode_alternative(alternative, &mut dict)?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base)?;
        writer.write_record(protocol_core::REC_ALTERNATIVE, true, &payload, CodecId(0))?;
    }
    let active_payload = encode_active(log.active_alternative_id.as_deref(), &mut dict);
    flush_dict_delta(&mut writer, &dict, &mut dict_base)?;
    writer.write_record(protocol_core::REC_ACTIVE, true, &active_payload, CodecId(0))?;

    if let Some(cursor) = &log.cursor {
        let cursor_payload = encode_cursor(cursor, &mut dict, |id| ordinals.get(id).copied())?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base)?;
        writer.write_record(REC_CURSOR, false, &cursor_payload, CodecId(0))?;
    }

    writer.commit()?;
    Ok(writer.into_sink())
}

fn decode_history_from(trusted: &[u8], options: &DecodeOptions) -> Result<HistoryLog, ProtocolError> {
    let mut dict = DictReader::new();
    let mut edit_ids: Vec<String> = Vec::new();
    let mut log = HistoryLog::default();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64);
    let hasher = Blake3Hasher;
    let full = options.verification == VerificationLevel::Full;
    let mut running_chain = if full { hasher.hash(&trusted[..HEADER_SIZE]) } else { [0u8; 32] };
    let mut pending_digests: Vec<[u8; 32]> = Vec::new();

    while let Some(frame) = cursor.next_frame()? {
        if full && frame.kind != protocol_core::REC_COMMIT {
            let frame_bytes = &trusted[frame.offset as usize..(frame.offset + frame.frame_len()) as usize];
            pending_digests.push(hasher.hash(frame_bytes));
        }
        match frame.kind {
            protocol_core::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload())?,
            protocol_core::REC_ACTOR_DICT => {} // v1 never splits an actor dictionary — see 🔖️Payloads note
            protocol_core::REC_DOC => {
                let (doc_id, schema) = decode_doc(frame.payload(), &dict)?;
                log.doc_id = doc_id;
                log.schema = schema;
            }
            protocol_core::REC_EDIT => {
                let edit_ids_ref = &edit_ids;
                let edit = decode_edit(frame.payload(), &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32)))?;
                edit_ids.push(edit.id.clone());
                log.edits.push(edit);
            }
            protocol_core::REC_CHANGE => {
                let edit_ids_ref = &edit_ids;
                let change = decode_change(frame.payload(), &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32)))?;
                log.changes.push(change);
            }
            protocol_core::REC_CHECKPOINT => log.checkpoints.push(decode_checkpoint(frame.payload(), &dict)?),
            protocol_core::REC_ALTERNATIVE => log.alternatives.push(decode_alternative(frame.payload(), &dict)?),
            protocol_core::REC_ACTIVE => log.active_alternative_id = decode_active(frame.payload(), &dict)?,
            REC_CURSOR => {
                let edit_ids_ref = &edit_ids;
                log.cursor = Some(decode_cursor(frame.payload(), &dict, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32)))?);
            }
            protocol_core::REC_COMMIT if full => {
                let (commit_seq, chain_hash) = parse_commit_fields(frame.payload())?;
                let mut concat = running_chain.to_vec();
                for digest in &pending_digests {
                    concat.extend_from_slice(digest);
                }
                let recomputed = hasher.hash(&concat);
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

pub fn decode_history(bytes: &[u8], options: &DecodeOptions) -> Result<HistoryLog, ProtocolError> {
    HistoryReader::open(bytes, options)?.log()
}
//#endregion 🔖️Codec

//#region 🔖️Append
// Streaming append API over protocol_format::SprWriter — the hot path. One edit -> one REC_EDIT
// frame, O(new edit) allocation.
pub struct HistoryAppender<S: PackSink> {
    writer: SprWriter<S>,
    dict: DictBuilder,
    dict_base: u32,
    edit_ordinals: HashMap<String, u64>,
    next_edit_ordinal: u64,
}

impl<S: PackSink> HistoryAppender<S> {
    pub fn begin(sink: S, doc_id: &str, schema: &str, options: &WriteOptions) -> Result<Self, ProtocolError> {
        let mut writer = SprWriter::begin(sink, options)?;
        let mut dict = DictBuilder::new();
        let mut dict_base = 0u32;
        let payload = encode_doc(doc_id, schema, &mut dict);
        flush_dict_delta(&mut writer, &dict, &mut dict_base)?;
        writer.write_record(protocol_core::REC_DOC, true, &payload, CodecId(0))?;
        Ok(Self { writer, dict, dict_base, edit_ordinals: HashMap::new(), next_edit_ordinal: 0 })
    }

    pub fn append_edit(&mut self, edit: &HistoryEdit) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_edit(edit, &mut self.dict, |id| ordinals.get(id).copied())?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base)?;
        let offset = self.writer.write_record(protocol_core::REC_EDIT, true, &payload, CodecId(0))?;
        self.edit_ordinals.insert(edit.id.clone(), self.next_edit_ordinal);
        self.next_edit_ordinal += 1;
        Ok(offset)
    }

    pub fn append_change(&mut self, change: &HistoryChange) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_change(change, &mut self.dict, |id| ordinals.get(id).copied())?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base)?;
        self.writer.write_record(protocol_core::REC_CHANGE, true, &payload, CodecId(0))
    }

    pub fn append_checkpoint(&mut self, checkpoint: &HistoryCheckpoint) -> Result<u64, ProtocolError> {
        let payload = encode_checkpoint(checkpoint, &mut self.dict)?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base)?;
        self.writer.write_record(protocol_core::REC_CHECKPOINT, true, &payload, CodecId(0))
    }

    pub fn append_alternative(&mut self, alternative: &HistoryAlternative) -> Result<u64, ProtocolError> {
        let payload = encode_alternative(alternative, &mut self.dict)?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base)?;
        self.writer.write_record(protocol_core::REC_ALTERNATIVE, true, &payload, CodecId(0))
    }

    pub fn set_active(&mut self, alternative_id: Option<&str>) -> Result<u64, ProtocolError> {
        let payload = encode_active(alternative_id, &mut self.dict);
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base)?;
        self.writer.write_record(protocol_core::REC_ACTIVE, true, &payload, CodecId(0))
    }

    pub fn append_cursor(&mut self, cursor: &HistoryCursor) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_cursor(cursor, &mut self.dict, |id| ordinals.get(id).copied())?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base)?;
        self.writer.write_record(REC_CURSOR, false, &payload, CodecId(0))
    }

    pub fn commit(&mut self) -> Result<u64, ProtocolError> {
        self.writer.commit()
    }

    pub fn into_sink(self) -> S {
        self.writer.into_sink()
    }
}
//#endregion 🔖️Append

//#region 🔖️Scan
// Read-side over a byte buffer, via protocol_format cursors. `open` establishes a trusted byte
// range via `protocol_format::recover` (RecoveryMode::LastCommit) once; every subsequent
// operation stays within that range, so a torn tail can never surface a partially-written record.
pub struct HistoryReader<'a> {
    trusted: &'a [u8],
    options: DecodeOptions,
}

impl<'a> HistoryReader<'a> {
    pub fn open(bytes: &'a [u8], options: &DecodeOptions) -> Result<Self, ProtocolError> {
        let recovery = protocol_format::recover(&bytes, &options.limits, RecoveryMode::LastCommit)?;
        let trusted = &bytes[..recovery.bytes_recovered as usize];
        Ok(Self { trusted, options: options.clone() })
    }

    pub fn log(&self) -> Result<HistoryLog, ProtocolError> {
        decode_history_from(self.trusted, &self.options)
    }

    pub fn edits(&self) -> EditIter<'a> {
        EditIter { cursor: FrameCursor::new(self.trusted, HEADER_SIZE as u64), dict: DictReader::new(), edit_ids: Vec::new() }
    }

    pub fn edits_rev(&self, limit: usize) -> RevEditIter<'a> {
        match prescan_full(self.trusted) {
            Ok((dict, edit_ids)) => {
                RevEditIter { state: Ok(RevEditIterReady { cursor: ReverseFrameCursor::at_end(&self.trusted[HEADER_SIZE..]), dict, edit_ids, remaining: limit }) }
            }
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

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.cursor.next_frame() {
                Ok(Some(frame)) => match frame.kind {
                    protocol_core::REC_STR_DICT => {
                        if let Err(e) = apply_dict_record(&mut self.dict, frame.payload()) {
                            return Some(Err(e));
                        }
                    }
                    protocol_core::REC_EDIT => {
                        let edit_ids_ref = &self.edit_ids;
                        let dict_ref = &self.dict;
                        let result = decode_edit(frame.payload(), dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32)));
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

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            Err(pending) => pending.take().map(Err),
            Ok(ready) => {
                if ready.remaining == 0 {
                    return None;
                }
                loop {
                    match ready.cursor.prev_frame() {
                        Ok(Some(frame)) => {
                            if frame.kind == protocol_core::REC_EDIT {
                                let edit_ids_ref = &ready.edit_ids;
                                let dict_ref = &ready.dict;
                                let result = decode_edit(frame.payload(), dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32)));
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
fn prescan_full(trusted: &[u8]) -> Result<(DictReader, Vec<String>), ProtocolError> {
    let mut dict = DictReader::new();
    let mut edit_ids = Vec::new();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64);
    while let Some(frame) = cursor.next_frame()? {
        match frame.kind {
            protocol_core::REC_STR_DICT => apply_dict_record(&mut dict, frame.payload())?,
            protocol_core::REC_EDIT => {
                let edit_ids_ref = &edit_ids;
                let dict_ref = &dict;
                let edit = decode_edit(frame.payload(), dict_ref, |ord| edit_ids_ref.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32)))?;
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
pub fn frontier_delta(local: &FrontierSummary, remote: &FrontierSummary) -> FrontierComparison {
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
pub const SEC_PROJECTION_OFFSETS: u8 = 0x04;
pub const SEC_SEALED_OFFSETS: u8 = 0x05;

fn write_pair_section(out: &mut ByteWriter, kind: u8, entries: &[(u64, u64)]) {
    out.write_u8(kind);
    out.write_varint_u64(entries.len() as u64);
    for (a, b) in entries {
        out.write_varint_u64(*a);
        out.write_varint_u64(*b);
    }
}

fn write_offsets_section(out: &mut ByteWriter, kind: u8, offsets: &[u64]) {
    out.write_u8(kind);
    out.write_varint_u64(offsets.len() as u64);
    for offset in offsets {
        out.write_varint_u64(*offset);
    }
}

#[derive(Clone, Debug, Default)]
pub struct IndexBuilder {
    edits: Vec<(u64, u64)>,
    checkpoints: Vec<(String, u64, u64)>,
    dict_offsets: Vec<u64>,
    projections: Vec<(u64, u64)>,
    sealed: Vec<u64>,
}

impl IndexBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_edit(&mut self, ordinal: u64, offset: u64) {
        self.edits.push((ordinal, offset));
    }

    pub fn record_checkpoint(&mut self, id: &str, offset: u64, edit_ordinal: u64) {
        self.checkpoints.push((id.to_string(), offset, edit_ordinal));
    }

    pub fn record_dict(&mut self, offset: u64) {
        self.dict_offsets.push(offset);
    }

    pub fn record_projection(&mut self, edit_ordinal: u64, offset: u64) {
        self.projections.push((edit_ordinal, offset));
    }

    pub fn record_sealed(&mut self, offset: u64) {
        self.sealed.push(offset);
    }

    pub fn build(&self) -> Vec<u8> {
        let mut out = ByteWriter::new();
        out.write_u8(1);
        write_pair_section(&mut out, SEC_EDIT_OFFSETS, &self.edits);
        out.write_u8(SEC_CHECKPOINT_OFFSETS);
        out.write_varint_u64(self.checkpoints.len() as u64);
        for (id, offset, edit_ordinal) in &self.checkpoints {
            write_str_field(&mut out, id);
            out.write_varint_u64(*offset);
            out.write_varint_u64(*edit_ordinal);
        }
        write_offsets_section(&mut out, SEC_DICT_OFFSETS, &self.dict_offsets);
        write_pair_section(&mut out, SEC_PROJECTION_OFFSETS, &self.projections);
        write_offsets_section(&mut out, SEC_SEALED_OFFSETS, &self.sealed);
        out.into_bytes()
    }
}

pub struct IndexReader<'a> {
    edits: Vec<(u64, u64)>,
    checkpoints: Vec<(&'a str, u64, u64)>,
    projections: Vec<(u64, u64)>,
}

impl<'a> IndexReader<'a> {
    pub fn open(payload: &'a [u8]) -> Result<Self, ProtocolError> {
        let mut input = ByteReader::new(payload);
        let format = input.read_u8()?;
        if format > 1 {
            return Err(malformed_fmt("index", format));
        }
        let mut edits = Vec::new();
        let mut checkpoints = Vec::new();
        let mut projections = Vec::new();
        while input.remaining() > 0 {
            let kind = input.read_u8()?;
            let count = input.read_varint_u64()?;
            match kind {
                SEC_EDIT_OFFSETS => {
                    for _ in 0..count {
                        let ordinal = input.read_varint_u64()?;
                        let offset = input.read_varint_u64()?;
                        edits.push((ordinal, offset));
                    }
                }
                SEC_CHECKPOINT_OFFSETS => {
                    for _ in 0..count {
                        let len = input.read_varint_u64()? as usize;
                        let bytes = input.read_bytes(len)?;
                        let id = std::str::from_utf8(bytes).map_err(|_| ProtocolError::Malformed { what: "index checkpoint id utf8", offset: 0, detail: "invalid utf-8".to_string() })?;
                        let offset = input.read_varint_u64()?;
                        let edit_ordinal = input.read_varint_u64()?;
                        checkpoints.push((id, offset, edit_ordinal));
                    }
                }
                SEC_DICT_OFFSETS => {
                    for _ in 0..count {
                        input.read_varint_u64()?;
                    }
                }
                SEC_PROJECTION_OFFSETS => {
                    for _ in 0..count {
                        let ordinal = input.read_varint_u64()?;
                        let offset = input.read_varint_u64()?;
                        projections.push((ordinal, offset));
                    }
                }
                SEC_SEALED_OFFSETS => {
                    for _ in 0..count {
                        input.read_varint_u64()?;
                    }
                }
                other => return Err(ProtocolError::Malformed { what: "index section kind", offset: 0, detail: format!("unknown section {other:#x}") }),
            }
        }
        Ok(Self { edits, checkpoints, projections })
    }

    pub fn edit_offset_at_or_before(&self, ordinal: u64) -> Option<u64> {
        self.edits.iter().filter(|(o, _)| *o <= ordinal).max_by_key(|(o, _)| *o).map(|(_, offset)| *offset)
    }

    pub fn checkpoint_offset(&self, checkpoint_id: &str) -> Option<(u64, u64)> {
        self.checkpoints.iter().find(|(id, _, _)| *id == checkpoint_id).map(|(_, offset, edit_ordinal)| (*offset, *edit_ordinal))
    }

    pub fn latest_projection_offset_at_or_before(&self, ordinal: u64) -> Option<u64> {
        self.projections.iter().filter(|(o, _)| *o <= ordinal).max_by_key(|(o, _)| *o).map(|(_, offset)| *offset)
    }
}
//#endregion 🔖️Index

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_log() -> HistoryLog {
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
                    backwards: Vec::new(),
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
                    backwards: Vec::new(),
                    meta: Some(vec![HistoryOpMeta {
                        op_id: Some("op-1".to_string()),
                        dependencies: vec!["edit-1".to_string()],
                        base_version: 7,
                        author_id: Some("alice".to_string()),
                        hlt: Some((1, 1_700_000_000_000, 3)),
                        undo_policy: 2,
                        payload_hash: Some([9u8; 32]),
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
        }
    }

    //#region 🔖️TextGrammar
    #[test]
    fn ops_text_round_trips_a_full_log() {
        // `HistoryEdit::meta` is derived data the text grammar never carries (see the Model
        // region note) — parse_ops_text always yields `meta: None`, so the expectation strips it
        // before comparing rather than asserting full structural equality including meta.
        let mut log = sample_log();
        for edit in &mut log.edits {
            edit.meta = None;
        }
        let text = print_ops_text(&log).unwrap();
        let parsed = parse_ops_text(&text).unwrap();
        assert_eq!(parsed, log);
    }

    #[test]
    fn ops_text_is_a_fixpoint_under_reprint() {
        let log = sample_log();
        let text = print_ops_text(&log).unwrap();
        let reparsed = parse_ops_text(&text).unwrap();
        assert_eq!(print_ops_text(&reparsed).unwrap(), text);
    }

    #[test]
    fn ops_text_skips_comments_and_blank_lines() {
        let text = "doc doc-1 schema=s1\n\n# a comment\nactive alt-1\n";
        let log = parse_ops_text(text).unwrap();
        assert_eq!(log.doc_id, "doc-1");
        assert_eq!(log.active_alternative_id.as_deref(), Some("alt-1"));
    }

    #[test]
    fn ops_text_rejects_unknown_line_keyword() {
        let err = parse_ops_text("doc doc-1 schema=s1\nbogus x\n").unwrap_err();
        assert!(matches!(err, ProtocolError::Malformed { .. }));
    }

    #[test]
    fn ops_text_edit_without_active_line_leaves_none() {
        let log = HistoryLog { doc_id: "d".into(), schema: "s".into(), active_alternative_id: None, ..Default::default() };
        let text = print_ops_text(&log).unwrap();
        assert!(!text.contains("active"));
        assert_eq!(parse_ops_text(&text).unwrap().active_alternative_id, None);
    }

    #[test]
    fn ops_text_round_trips_a_cursor_line_with_undo_then_apply_interleaving() {
        let mut log = sample_log();
        for edit in &mut log.edits {
            edit.meta = None;
        }
        // A single tail-edit marker cannot represent this: edit-1 undone (moved to redo), then a
        // later apply produced edit-2 — edit-1 precedes edit-2 in file order but is NOT applied.
        log.cursor = Some(HistoryCursor { applied_edit_ids: vec!["edit-2".to_string()], redo_edit_ids: vec!["edit-1".to_string()], checkpoint_id: Some("ck-1".to_string()) });
        let text = print_ops_text(&log).unwrap();
        assert!(text.contains("cursor"));
        let parsed = parse_ops_text(&text).unwrap();
        assert_eq!(parsed, log);
    }

    #[test]
    fn ops_text_without_a_cursor_line_leaves_cursor_none() {
        let log = HistoryLog { doc_id: "d".into(), schema: "s".into(), ..Default::default() };
        let text = print_ops_text(&log).unwrap();
        assert!(!text.contains("cursor"));
        assert_eq!(parse_ops_text(&text).unwrap().cursor, None);
    }
    //#endregion 🔖️TextGrammar

    //#region 🔖️Payloads
    #[test]
    fn doc_payload_round_trips() {
        let mut dict = DictBuilder::new();
        let payload = encode_doc("doc-1", "org.semio.demo.v1", &mut dict);
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let (id, schema) = decode_doc(&payload, &reader).unwrap();
        assert_eq!(id, "doc-1");
        assert_eq!(schema, "org.semio.demo.v1");
    }

    #[test]
    fn edit_payload_round_trips_with_all_optionals_and_meta() {
        let log = sample_log();
        let edit = &log.edits[1];
        let mut dict = DictBuilder::new();
        let ordinals: HashMap<&str, u64> = [("edit-1", 0u64)].into_iter().collect();
        let payload = encode_edit(edit, &mut dict, |id| ordinals.get(id).copied()).unwrap();
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let edit_ids = ["edit-1".to_string()];
        let decoded = decode_edit(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).unwrap();
        assert_eq!(decoded, *edit);
    }

    #[test]
    fn edit_payload_round_trips_minimal_edit() {
        let edit = HistoryEdit {
            id: "edit-x".to_string(),
            actor: None,
            started_at: "2024-01-01T00:00:00Z".to_string(),
            finished_at: None,
            coalesce_key: None,
            description: None,
            ops: Vec::new(),
            backwards: Vec::new(),
            meta: None,
        };
        let mut dict = DictBuilder::new();
        let payload = encode_edit(&edit, &mut dict, |_| None).unwrap();
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).unwrap();
        assert_eq!(decoded, edit);
    }

    #[test]
    fn change_payload_round_trips_and_references_edit_ordinals() {
        let change = HistoryChange { id: "change-1".to_string(), saved_at: "2024-01-01T00:00:00Z".to_string(), edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], description: Some("d".to_string()) };
        let mut dict = DictBuilder::new();
        let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
        let payload = encode_change(&change, &mut dict, |id| ordinals.get(id).copied()).unwrap();
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
        let decoded = decode_change(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).unwrap();
        assert_eq!(decoded, change);
    }

    #[test]
    fn checkpoint_payload_round_trips_with_authors() {
        let checkpoint = sample_log().checkpoints.remove(0);
        let mut dict = DictBuilder::new();
        let payload = encode_checkpoint(&checkpoint, &mut dict).unwrap();
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let decoded = decode_checkpoint(&payload, &reader).unwrap();
        assert_eq!(decoded, checkpoint);
    }

    #[test]
    fn alternative_payload_round_trips() {
        let alternative = sample_log().alternatives.remove(0);
        let mut dict = DictBuilder::new();
        let payload = encode_alternative(&alternative, &mut dict).unwrap();
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let decoded = decode_alternative(&payload, &reader).unwrap();
        assert_eq!(decoded, alternative);
    }

    #[test]
    fn active_payload_round_trips_some_and_none() {
        let mut dict = DictBuilder::new();
        let payload_some = encode_active(Some("alt-1"), &mut dict);
        let payload_none = encode_active(None, &mut dict);
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        assert_eq!(decode_active(&payload_some, &reader).unwrap(), Some("alt-1".to_string()));
        assert_eq!(decode_active(&payload_none, &reader).unwrap(), None);
    }

    #[test]
    fn edit_payload_round_trips_a_backwards_section_mixing_text_and_binary_payloads() {
        let edit = HistoryEdit {
            id: "edit-y".to_string(),
            actor: Some("bob".to_string()),
            started_at: "2024-02-01T00:00:00Z".to_string(),
            finished_at: Some("2024-02-01T00:00:01Z".to_string()),
            coalesce_key: None,
            description: None,
            ops: vec![OpPayload { text: Some("set n=1".to_string()), binary: Some(vec![1, 2, 3]) }, OpPayload { text: Some("set n=2".to_string()), binary: None }],
            backwards: vec![OpPayload { text: Some("set n=0".to_string()), binary: Some(vec![0]) }, OpPayload { text: Some("set n=1".to_string()), binary: None }],
            meta: None,
        };
        let mut dict = DictBuilder::new();
        let payload = encode_edit(&edit, &mut dict, |_| None).unwrap();
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).unwrap();
        assert_eq!(decoded, edit);
        assert_eq!(decoded.ops[0].binary, Some(vec![1, 2, 3]));
        assert_eq!(decoded.backwards[0].binary, Some(vec![0]));
    }

    #[test]
    fn edit_payload_with_empty_backwards_omits_the_section_and_decodes_empty() {
        let edit = HistoryEdit {
            id: "edit-z".to_string(),
            actor: None,
            started_at: "2024-02-01T00:00:00Z".to_string(),
            finished_at: None,
            coalesce_key: None,
            description: None,
            ops: vec![OpPayload { text: Some("noop".to_string()), binary: None }],
            backwards: Vec::new(),
            meta: None,
        };
        let mut dict = DictBuilder::new();
        let payload = encode_edit(&edit, &mut dict, |_| None).unwrap();
        // presence byte is the 2nd byte (offset 1); bit5 (0x20) must be unset when backwards is empty.
        assert_eq!(payload[1] & 0b0010_0000, 0, "bit5 must be unset for empty backwards");
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).unwrap();
        assert_eq!(decoded.backwards, Vec::new());
    }

    #[test]
    fn cursor_payload_round_trips_with_dict_and_ordinal_refs() {
        let cursor = HistoryCursor { applied_edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], redo_edit_ids: vec!["edit-3".to_string()], checkpoint_id: Some("ck-1".to_string()) };
        let mut dict = DictBuilder::new();
        let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
        let payload = encode_cursor(&cursor, &mut dict, |id| ordinals.get(id).copied()).unwrap();
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
        let decoded = decode_cursor(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).unwrap();
        assert_eq!(decoded, cursor);
    }

    #[test]
    fn cursor_payload_round_trips_without_a_checkpoint() {
        let cursor = HistoryCursor { applied_edit_ids: Vec::new(), redo_edit_ids: Vec::new(), checkpoint_id: None };
        let mut dict = DictBuilder::new();
        let payload = encode_cursor(&cursor, &mut dict, |_| None).unwrap();
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        let decoded = decode_cursor(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).unwrap();
        assert_eq!(decoded, cursor);
    }

    #[test]
    fn decode_rejects_unsupported_format() {
        let payload = vec![2u8, 0, 0];
        let dict = DictReader::new();
        let err = decode_doc(&payload, &dict).unwrap_err();
        assert!(matches!(err, ProtocolError::Malformed { .. }));
    }
    //#endregion 🔖️Payloads

    //#region 🔖️Codec
    #[test]
    fn history_encode_decode_identity_standard() {
        let log = sample_log();
        let bytes = encode_history(&log, &EncodeOptions::default()).unwrap();
        let decoded = decode_history(&bytes, &DecodeOptions::default()).unwrap();
        assert_eq!(decoded, log);
    }

    #[test]
    fn history_encode_decode_identity_full_verification() {
        let log = sample_log();
        let bytes = encode_history(&log, &EncodeOptions::default()).unwrap();
        let options = DecodeOptions { verification: VerificationLevel::Full, limits: ProtocolLimits::default() };
        let decoded = decode_history(&bytes, &options).unwrap();
        assert_eq!(decoded, log);
    }

    #[test]
    fn history_encode_is_canonically_stable() {
        let log = sample_log();
        let a = encode_history(&log, &EncodeOptions::default()).unwrap();
        let b = encode_history(&log, &EncodeOptions::default()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn history_full_verification_detects_tampering() {
        let log = sample_log();
        let mut bytes = encode_history(&log, &EncodeOptions::default()).unwrap();
        let last = bytes.len();
        bytes[last - 10] ^= 0xFF;
        let options = DecodeOptions { verification: VerificationLevel::Full, limits: ProtocolLimits::default() };
        // A single-byte flip breaks that frame's own CRC-32C, so `protocol_format::recover`
        // (RecoveryMode::LastCommit, run by `HistoryReader::open`) truncates the trusted range to
        // before the corrupted frame — here that means before the file's only commit, so the
        // decoded log comes back empty rather than as an `Err`. Either outcome is an acceptable
        // "tamper never goes unnoticed": the result must never silently equal the original log.
        let result = decode_history(&bytes, &options);
        assert!(result.is_err() || result.unwrap() != log);
    }

    #[test]
    fn history_round_trips_backwards_and_binary_payloads_and_cursor_when_write_backwards_section_is_set() {
        let mut log = sample_log();
        log.edits[0].backwards = vec![OpPayload { text: Some("unset foo".to_string()), binary: Some(vec![9, 9]) }, OpPayload { text: Some("unset bar".to_string()), binary: None }];
        log.edits[1].ops[0].binary = Some(vec![7]);
        log.cursor = Some(HistoryCursor { applied_edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], redo_edit_ids: Vec::new(), checkpoint_id: Some("ck-1".to_string()) });
        let options = EncodeOptions { write_backwards_section: true, ..EncodeOptions::default() };
        let bytes = encode_history(&log, &options).unwrap();
        let decoded = decode_history(&bytes, &DecodeOptions::default()).unwrap();
        assert_eq!(decoded, log);
        assert_eq!(decoded.edits[0].backwards[0].binary, Some(vec![9, 9]));
        assert_eq!(decoded.edits[1].ops[0].binary, Some(vec![7]));
        assert_eq!(decoded.cursor, log.cursor);
    }

    #[test]
    fn history_strips_backwards_when_write_backwards_section_is_unset_even_if_populated() {
        let mut log = sample_log();
        log.edits[0].backwards = vec![OpPayload { text: Some("unset foo".to_string()), binary: None }];
        let bytes = encode_history(&log, &EncodeOptions::default()).unwrap();
        let decoded = decode_history(&bytes, &DecodeOptions::default()).unwrap();
        assert_eq!(decoded.edits[0].backwards, Vec::new(), "write_backwards_section defaults false and must strip populated backwards");
    }
    //#endregion 🔖️Codec

    //#region 🔖️Append
    #[test]
    fn streamed_append_equals_buffered_encode() {
        let log = sample_log();
        let options = WriteOptions { required_flags: protocol_core::REQUIRED_HASH_CHAIN, optional_flags: protocol_core::OPTIONAL_CANONICAL };
        let mut appender = HistoryAppender::begin(Vec::<u8>::new(), &log.doc_id, &log.schema, &options).unwrap();
        for edit in &log.edits {
            appender.append_edit(edit).unwrap();
        }
        for change in &log.changes {
            appender.append_change(change).unwrap();
        }
        for checkpoint in &log.checkpoints {
            appender.append_checkpoint(checkpoint).unwrap();
        }
        for alternative in &log.alternatives {
            appender.append_alternative(alternative).unwrap();
        }
        appender.set_active(log.active_alternative_id.as_deref()).unwrap();
        appender.commit().unwrap();
        let streamed_bytes = appender.into_sink();

        let decoded = decode_history(&streamed_bytes, &DecodeOptions::default()).unwrap();
        assert_eq!(decoded, log);
    }

    #[test]
    fn append_cursor_then_decode_recovers_it() {
        let mut log = sample_log();
        for edit in &mut log.edits {
            edit.meta = None;
        }
        let cursor = HistoryCursor { applied_edit_ids: vec!["edit-1".to_string()], redo_edit_ids: vec!["edit-2".to_string()], checkpoint_id: Some("ck-1".to_string()) };
        let options = WriteOptions { required_flags: protocol_core::REQUIRED_HASH_CHAIN, optional_flags: protocol_core::OPTIONAL_CANONICAL };
        let mut appender = HistoryAppender::begin(Vec::<u8>::new(), &log.doc_id, &log.schema, &options).unwrap();
        for edit in &log.edits {
            appender.append_edit(edit).unwrap();
        }
        appender.append_cursor(&cursor).unwrap();
        appender.commit().unwrap();
        let bytes = appender.into_sink();

        let decoded = decode_history(&bytes, &DecodeOptions::default()).unwrap();
        assert_eq!(decoded.cursor, Some(cursor));
    }
    //#endregion 🔖️Append

    //#region 🔖️Scan
    #[test]
    fn reader_edits_forward_matches_log() {
        let log = sample_log();
        let bytes = encode_history(&log, &EncodeOptions::default()).unwrap();
        let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).unwrap();
        let edits: Vec<HistoryEdit> = reader.edits().map(|r| r.unwrap()).collect();
        assert_eq!(edits, log.edits);
    }

    #[test]
    fn reader_edits_rev_matches_tail_in_reverse() {
        let log = sample_log();
        let bytes = encode_history(&log, &EncodeOptions::default()).unwrap();
        let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).unwrap();
        let rev: Vec<HistoryEdit> = reader.edits_rev(1).map(|r| r.unwrap()).collect();
        assert_eq!(rev.len(), 1);
        assert_eq!(rev[0], log.edits[1]);
    }

    #[test]
    fn reader_edits_rev_full_matches_reversed_forward() {
        let log = sample_log();
        let bytes = encode_history(&log, &EncodeOptions::default()).unwrap();
        let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).unwrap();
        let rev: Vec<HistoryEdit> = reader.edits_rev(usize::MAX).map(|r| r.unwrap()).collect();
        let mut expected = log.edits;
        expected.reverse();
        assert_eq!(rev, expected);
    }
    //#endregion 🔖️Scan

    //#region 🔖️Frontier
    fn frontier(head_edit_ordinal: u64, head_edit_id: &str, chain_hash: [u8; 32]) -> FrontierSummary {
        FrontierSummary { document_id: "doc-1".to_string(), head_edit_ordinal, head_edit_id: head_edit_id.to_string(), alternatives: Vec::new(), last_commit_seq: 1, chain_hash }
    }

    #[test]
    fn frontier_delta_reports_equal_ahead_behind_diverged() {
        let a = frontier(5, "edit-5", [1u8; 32]);
        let b = frontier(5, "edit-5", [1u8; 32]);
        assert_eq!(frontier_delta(&a, &b), FrontierComparison::Equal);

        let ahead = frontier(6, "edit-6", [2u8; 32]);
        assert_eq!(frontier_delta(&ahead, &a), FrontierComparison::Ahead);
        assert_eq!(frontier_delta(&a, &ahead), FrontierComparison::Behind);

        let diverged = frontier(5, "edit-5-alt", [3u8; 32]);
        assert_eq!(frontier_delta(&a, &diverged), FrontierComparison::Diverged { common_edit_count: 5 });
    }
    //#endregion 🔖️Frontier

    //#region 🔖️Index
    #[test]
    fn index_round_trips_edits_checkpoints_and_projections() {
        let mut builder = IndexBuilder::new();
        builder.record_edit(0, 100);
        builder.record_edit(5, 300);
        builder.record_edit(10, 500);
        builder.record_checkpoint("ck-1", 700, 10);
        builder.record_projection(5, 250);
        builder.record_sealed(50);
        let payload = builder.build();

        let reader = IndexReader::open(&payload).unwrap();
        assert_eq!(reader.edit_offset_at_or_before(7), Some(300));
        assert_eq!(reader.edit_offset_at_or_before(0), Some(100));
        assert_eq!(reader.edit_offset_at_or_before(10), Some(500));
        assert_eq!(reader.checkpoint_offset("ck-1"), Some((700, 10)));
        assert_eq!(reader.checkpoint_offset("missing"), None);
        assert_eq!(reader.latest_projection_offset_at_or_before(9), Some(250));
    }
    //#endregion 🔖️Index
}
//#endregion 🧪️Tests
