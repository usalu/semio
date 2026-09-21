//! 🎞️ Protocol history log: the typed record model (`HistoryLog` and friends), the `.ops` text
//! grammar twin (built directly on `dsl_schema`, never on `vcs`), per-kind binary payload codecs
//! (built on `crate::os_spr::wire::scalar` + `protocol_format`'s frame writer/reader), the whole-file
//! codec, a streaming append API, and a lazy forward/reverse scan API. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol_history`).
//!
//! Op payloads are opaque validated bytes to this crate — it stores, hashes, frames, and indexes
//! them, but never interprets operation semantics (that is `protocol_command`'s concern, a sibling
//! crate this one does not depend on).

#[path = "🛂️identity/🦀️.rs"]
pub(crate) mod identity;

use crate::os_dsl::schema::{FieldSpec, FieldValue, JoinMode, ParseOptions, RecordLayout, RecordSpec, RecordValue, Shape};
use crate::os_pack::{ByteReader, ByteWriter, CodecId, PackSink};
use crate::os_spr::format::{Blake3Hasher, FrameCursor, RecoveryMode, ReverseFrameCursor, SprWriter, VerificationLevel, WriteOptions, HEADER_SIZE};
use crate::os_spr::wire::{DictBuilder, DictReader, ProtocolError, ProtocolLimits, RecordHasher};
use std::collections::{HashMap, HashSet};

//#region 🔖️Model
// Persisted history is the semantic event log only: edits (opaque `print_op` lines / `OpBinary`
// payloads) and structural transitions. Change/checkpoint/alternative facts, the active alternative,
// checkpoint pins and the undo/redo cursor are derived by [`HistoryLog::fold`], never stored.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryLog {
    pub doc_id: String,
    pub schema: String,
    pub edits: Vec<HistoryEdit>,
    /// @emoji 🔀️ Structural history steps (`REC_TRANSITION`) — undo/redo, commits, branches,
    /// checkouts and repins — in persisted order; the fold orders them by `(hlt, id)`.
    pub transitions: Vec<HistoryTransitionRecord>,
    /// @emoji 🧩️ Composition overlay (`REC_COMPOSITION`): who owns this document and which dialect
    /// it materializes as. Absent for every non-composed document.
    pub composition: Option<HistoryComposition>,
    /// @emoji ⚔️ First-class merge conflicts (`REC_CONFLICT`), durable per
    /// `.🧬semio/🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/
    /// 📋️contract-freeze.md` §C7: a `Quarantined` batch rejected outright, or a `Degraded`
    /// accepted-but-messy merge — see `crate::os_spr::conflict::ConflictKind`. Empty for the
    /// overwhelming majority of documents; no record is written when empty.
    pub conflicts: Vec<HistoryConflict>,
}

/// @emoji 🔀️ One persisted history transition: the [`crate::os_spr::MutationEnvelope`] minus its
/// document id and schema (implied by `REC_DOC` and [`crate::os_spr::HISTORY_TRANSITION_SCHEMA`]).
/// `hlt` is `(actor, physical_ms, logical)` like [`HistoryConflict::hlt`]; `payload` is the encoded
/// [`crate::os_spr::HistoryTransition`], opaque to this codec.
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryTransitionRecord {
    pub id: String,
    pub actor: String,
    pub hlt: (u64, u64, u64),
    pub dependencies: Vec<String>,
    pub payload: Vec<u8>,
}

/// @emoji 🧩️ The durable form of a document's composition facts, carried as ONE extension record
/// rather than as new fields on the format-frozen critical `REC_DOC`: an older/foreign reader must
/// be able to skip it without failing the whole file. Checkpoint pins are not stored here — they
/// are facts of `Repin` transitions, derived by [`HistoryLog::fold`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryComposition {
    /// 🏠️ `(parent_artifact_uri, slot, child_id)` — the child-side ownership stamp.
    pub owner: Option<(String, String, String)>,
    /// 🎯️ `(artifact_kind, standard, subset)` — the dialect this document materializes as.
    pub dialect: Option<(String, String, String)>,
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
    /// @emoji 🛤️ Which undo/redo cursor this edit belongs to, as the document layer's own lane name
    /// (`crate::os_store::HistoryLane`'s camelCase word). `None` means the default DOCUMENT lane, so
    /// an ordinary edit costs nothing on the wire. It has to live here and not only on the envelope:
    /// without it a save/load cycle turned every side-lane edit back into a document edit, and the
    /// reloaded store's plain `Undo` then reverted an interaction edit that must never be undone
    /// that way (`history_lane_interaction_entries_survive_owned_document_round_trip`).
    pub lane: Option<String>,
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
//#endregion 🔖️Model

//#region 🔖️Fold
impl HistoryTransitionRecord {
    /// @emoji 📥️ Strips `envelope`'s document id and schema, keeping every other transition fact.
    pub fn from_envelope(envelope: &crate::os_spr::MutationEnvelope) -> Self {
        Self {
            id: envelope.mutation_id.0.clone(),
            actor: envelope.actor.0.clone(),
            hlt: (envelope.timestamp.actor, envelope.timestamp.physical_ms, envelope.timestamp.logical),
            dependencies: envelope.dependencies.iter().map(|dependency| dependency.0.clone()).collect(),
            payload: envelope.diff.payload.clone(),
        }
    }

    /// @emoji 📤️ The transition envelope of `document_id`: [`crate::os_spr::HISTORY_TRANSITION_SCHEMA`]
    /// diff carrying `payload`, empty inverse under the same schema.
    pub fn to_envelope(&self, document_id: &str) -> crate::os_spr::MutationEnvelope {
        let schema = crate::os_spr::SchemaId(crate::os_spr::HISTORY_TRANSITION_SCHEMA.to_string());
        crate::os_spr::MutationEnvelope {
            mutation_id: crate::os_spr::MutationId(self.id.clone()),
            document_id: crate::os_spr::ArtifactId(document_id.to_string()),
            actor: crate::os_spr::ActorId(self.actor.clone()),
            dependencies: self.dependencies.iter().cloned().map(crate::os_spr::MutationId).collect(),
            diff: crate::os_spr::ArtifactDiff { schema: schema.clone(), payload: self.payload.clone() },
            inverse: crate::os_spr::InverseMutation { schema, payload: Vec::new() },
            timestamp: crate::os_spr::HybridLogicalTimestamp { actor: self.hlt.0, physical_ms: self.hlt.1, logical: self.hlt.2 },
        }
    }
}

impl HistoryLog {
    /// @emoji 🧮️ Folds this log's edits and transitions ([`crate::os_spr::fold_history`]) into every
    /// derived history fact and position. An edit's clock is its first operation's `hlt` (zero when
    /// absent); its mutation ids are each operation's `op_id`, else `{edit.id}#{index}` (the
    /// [`crate::os_spr::mutation_ids_for_edit`] fallback). Edits owning an operation of a
    /// non-accepted quarantined conflict are excluded.
    pub fn fold(&self) -> Result<crate::os_spr::HistoryFold, ProtocolError> {
        let mut edits = Vec::with_capacity(self.edits.len());
        let mut owners: HashMap<String, &str> = HashMap::new();
        for edit in &self.edits {
            let meta = edit.meta.as_deref().unwrap_or_default();
            let timestamp = match meta.first().and_then(|meta| meta.hlt) {
                Some((actor, physical_ms, logical)) => crate::os_spr::HybridLogicalTimestamp {
                    actor,
                    physical_ms: u64::try_from(physical_ms).map_err(|_| ProtocolError::Malformed { what: "history fold", offset: 0, detail: format!("edit {} has a negative hybrid-clock time", edit.id) })?,
                    logical,
                },
                None => crate::os_spr::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 },
            };
            let mutation_ids: Vec<crate::os_spr::MutationId> = (0..edit.ops.len())
                .map(|index| crate::os_spr::MutationId(meta.get(index).and_then(|meta| meta.op_id.clone()).unwrap_or_else(|| format!("{}#{index}", edit.id))))
                .collect();
            for mutation_id in &mutation_ids {
                owners.insert(mutation_id.0.clone(), edit.id.as_str());
            }
            edits.push(crate::os_spr::FoldEdit { id: edit.id.clone(), actor: edit.actor.clone(), timestamp, mutation_ids });
        }
        let mut excluded = HashSet::new();
        for conflict in self.conflicts.iter().filter(|conflict| conflict.kind == 0 && conflict.status != 1) {
            for bytes in &conflict.envelopes {
                let mut position = 0;
                let envelope = crate::os_spr::decode_envelope(bytes, &mut position)?;
                if position != bytes.len() {
                    return Err(ProtocolError::Malformed { what: "history fold", offset: position as u64, detail: format!("quarantined conflict {} envelope has trailing bytes", conflict.id) });
                }
                if let Some(owner) = owners.get(&envelope.mutation_id.0) {
                    excluded.insert((*owner).to_string());
                }
            }
        }
        let transitions: Vec<crate::os_spr::MutationEnvelope> = self.transitions.iter().map(|transition| transition.to_envelope(&self.doc_id)).collect();
        crate::os_spr::fold_history(&edits, &transitions, &excluded)
    }
}
//#endregion 🔖️Fold

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
const F_TRANSITION_ID: u16 = 0;
const F_TRANSITION_ACTOR: u16 = 1;
const F_TRANSITION_HLC: u16 = 2;
const F_TRANSITION_DEPENDENCIES: u16 = 3;
const F_TRANSITION_PAYLOAD: u16 = 4;

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

/// @emoji 🔀️ `transition <id> actor=<actor> hlc=<actor>,<physical_ms>,<logical> dependencies=[...]
/// payload=<base64>` — one [`HistoryTransitionRecord`].
fn transition_spec() -> RecordSpec {
    RecordSpec::new(
        Some("transition"),
        RecordLayout::Inline,
        vec![
            FieldSpec::new(F_TRANSITION_ID, "", Shape::Text).positional(0),
            FieldSpec::new(F_TRANSITION_ACTOR, "actor", Shape::Text),
            FieldSpec::new(F_TRANSITION_HLC, "hlc", Shape::Tuple(Box::new(Shape::UInt), Some(3))),
            FieldSpec::new(F_TRANSITION_DEPENDENCIES, "dependencies", Shape::List(Box::new(Shape::Text))),
            FieldSpec::new(F_TRANSITION_PAYLOAD, "payload", Shape::Bytes64),
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

fn field_hlc(record: &RecordValue, id: u16) -> Result<(u64, u64, u64), ProtocolError> {
    match record.get(id) {
        Some(FieldValue::Tuple(items)) => match items.as_slice() {
            [FieldValue::UInt(actor), FieldValue::UInt(physical_ms), FieldValue::UInt(logical)] => Ok((*actor, *physical_ms, *logical)),
            _ => Err(ProtocolError::Malformed { what: "transition hlc", offset: 0, detail: "expected three unsigned integers".to_string() }),
        },
        _ => Err(ProtocolError::Malformed { what: "transition hlc", offset: 0, detail: "missing required field in ops text".to_string() }),
    }
}

fn field_bytes(record: &RecordValue, id: u16, what: &'static str) -> Result<Vec<u8>, ProtocolError> {
    match record.get(id) {
        Some(FieldValue::Bytes64(bytes)) => Ok(bytes.clone()),
        _ => Err(ProtocolError::Malformed { what, offset: 0, detail: "missing required field in ops text".to_string() }),
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
                inverse: Vec::new(),
                meta: None, lane: None,
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
                let record = crate::os_dsl::schema::parse(trimmed, &doc_spec(), &opts).map_err(text_error_to_protocol)?;
                log.doc_id = required_text(&record, F_DOC_ID, "doc id")?;
                log.schema = required_text(&record, F_DOC_SCHEMA, "doc schema")?;
            }
            "edit" => {
                let record = crate::os_dsl::schema::parse(trimmed, &edit_spec(), &opts).map_err(text_error_to_protocol)?;
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
            "transition" => {
                let record = crate::os_dsl::schema::parse(trimmed, &transition_spec(), &opts).map_err(text_error_to_protocol)?;
                log.transitions.push(HistoryTransitionRecord {
                    id: required_text(&record, F_TRANSITION_ID, "transition id")?,
                    actor: required_text(&record, F_TRANSITION_ACTOR, "transition actor")?,
                    hlt: field_hlc(&record, F_TRANSITION_HLC)?,
                    dependencies: field_text_list(&record, F_TRANSITION_DEPENDENCIES),
                    payload: field_bytes(&record, F_TRANSITION_PAYLOAD, "transition payload")?,
                });
            }
            other => return Err(ProtocolError::Malformed { what: "ops text line", offset: 0, detail: format!("unknown line keyword '{other}'") }),
        }
    }
    flush(&mut pending, &mut forwards, &mut log.edits);
    Ok(log)
}

/// @emoji 📤️ Prints a `HistoryLog` back to `.ops` text: `doc`, every edit (header + two-space
/// indented forward op lines), then one `transition` line per [`HistoryTransitionRecord`]. Errors if any op payload carries no text
/// (the binary-only `.spr` convention): this crate is schema-agnostic and cannot recover text
/// from an opaque binary payload — printing `.ops` for a real app document goes through the
/// concrete `Mutation::print_op` path instead (`crate::os_store::print_document_pack`'s `.ops` mirror).
pub fn print_ops_text(log: &HistoryLog) -> Result<String, ProtocolError> {
    let mut out = String::new();

    let doc_record = record_with(vec![(F_DOC_ID, FieldValue::Text(log.doc_id.clone())), (F_DOC_SCHEMA, FieldValue::Text(log.schema.clone()))]);
    out.push_str(&crate::os_dsl::schema::print(&doc_record, &doc_spec(), JoinMode::Inline));
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
        out.push_str(&crate::os_dsl::schema::print(&record_with(fields), &edit_spec(), JoinMode::Inline));
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

    for transition in &log.transitions {
        let fields = vec![
            (F_TRANSITION_ID, FieldValue::Text(transition.id.clone())),
            (F_TRANSITION_ACTOR, FieldValue::Text(transition.actor.clone())),
            (F_TRANSITION_HLC, FieldValue::Tuple(vec![FieldValue::UInt(transition.hlt.0), FieldValue::UInt(transition.hlt.1), FieldValue::UInt(transition.hlt.2)])),
            (F_TRANSITION_DEPENDENCIES, FieldValue::List(transition.dependencies.iter().map(|s| FieldValue::Text(s.clone())).collect())),
            (F_TRANSITION_PAYLOAD, FieldValue::Bytes64(transition.payload.clone())),
        ];
        out.push_str(&crate::os_dsl::schema::print(&record_with(fields), &transition_spec(), JoinMode::Inline));
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
// - `encode_edit`/`encode_conflicts` are the only encoders that reference edit ids (edit-ordinal
//   eligible); transitions reference operations by mutation id, never by edit ordinal.
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
    out.write_varint_u64(s.len() as u64);
    out.write_bytes(s.as_bytes());
}

async fn read_str_field(input: &mut ByteReader<'_>) -> Result<String, ProtocolError> {
    let len = input.read_varint_u64()? as usize;
    let bytes = input.read_bytes(len)?;
    std::str::from_utf8(bytes).map(str::to_string).map_err(|_| ProtocolError::Malformed { what: "utf8", offset: 0, detail: "invalid utf-8".to_string() })
}

async fn write_id_field(out: &mut ByteWriter, id: &str, dict: &mut DictBuilder, edit_ordinal_of: &(dyn Fn(&str) -> Option<u64> + Send + Sync)) -> Result<(), ProtocolError> {
    // ✏️ Genuine `|s|` closure — see `read_id_field`'s tag above (the HRTB gap applies to
    // `AsyncFnMut` the same way it does to `AsyncFn`).
    crate::os_spr::scalar::write_id(out, id, |s| dict.intern(s), edit_ordinal_of).map_err(ProtocolError::from)
}

fn read_id_field<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync)) -> Result<String, ProtocolError> {
    crate::os_spr::scalar::read_id(
        input,
        |idx: u32| dict.resolve(idx).map_err(|_| crate::os_pack::PackError::Malformed { what: "dict index", offset: idx as u64, detail: "out of range".to_string() }),
        |ord: u64| ordinal_to_id(ord).map_err(|_| crate::os_pack::PackError::Malformed { what: "edit ordinal", offset: ord, detail: "unresolvable".to_string() }),
    )
    .map_err(ProtocolError::from)
}

//#region 🔖️Message
// Shared wire shape for one `HistoryMessage`, used by both `HistoryOpMeta.messages` (🔖️Edit) and
// `HistoryConflict.messages` (🔖️Conflict) — one definition, both call sites.

/// @emoji 🎯️ `level u8 | code(idfield, dict-interned) | message(strfield) | target_count varint +
/// target(strfield)* | op_index presence u8 + [varint]`.
async fn write_history_message(out: &mut ByteWriter, message: &HistoryMessage, dict: &mut DictBuilder) -> Result<(), ProtocolError> {
    out.write_u8(message.level);
    write_id_field(out, &message.code, dict, &|_: &str| None).await?;
    write_str_field(out, &message.message).await;
    out.write_varint_u64(message.target.len() as u64);
    for target in &message.target {
        write_str_field(out, target).await;
    }
    match message.op_index {
        Some(index) => {
            out.write_u8(1);
            out.write_varint_u64(index as u64);
        }
        None => out.write_u8(0),
    }
    Ok(())
}

/// @emoji 🎯️ Inverse of [`write_history_message`].
async fn read_history_message(input: &mut ByteReader<'_>, dict: &DictReader) -> Result<HistoryMessage, ProtocolError> {
    let level = input.read_u8()?;
    if crate::os_dsl::Severity::from_u8(level).is_none() {
        return Err(ProtocolError::Malformed { what: "history message severity", offset: input.position() as u64 - 1, detail: format!("unknown severity {level}") });
    }
    let code = read_id_field(input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    let message = read_str_field(input).await?;
    let target_count = input.read_varint_u64()?;
    let mut target = Vec::with_capacity(target_count as usize);
    for _ in 0..target_count {
        target.push(read_str_field(input).await?);
    }
    let has_op_index = input.read_u8()?;
    let op_index = match has_op_index {
        0 => None,
        1 => {
            let raw = input.read_varint_u64()?;
            // 🚫️async: R10 shape 1 — `position()` is async but `map_err`'s closure is sync; the
            // offset is captured up front instead of awaited inside the closure.
            let offset = input.position() as u64;
            Some(u32::try_from(raw).map_err(|_| ProtocolError::Malformed { what: "history message operation index", offset, detail: "exceeds u32".to_string() })?)
        }
        value => return Err(ProtocolError::Malformed { what: "history message operation index presence", offset: input.position() as u64 - 1, detail: format!("expected 0 or 1, got {value}") }),
    };
    Ok(HistoryMessage { level, code, message, target, op_index })
}
//#endregion 🔖️Message

//#region 🔖️Doc
pub async fn encode_doc(doc_id: &str, schema: &str, dict: &mut DictBuilder) -> Vec<u8> {
    let mut out = ByteWriter::new();
    out.write_u8(1);
    write_id_field(&mut out, doc_id, dict, &|_: &str| None).await.expect("write_id never fails for an in-memory ByteWriter");
    write_id_field(&mut out, schema, dict, &|_: &str| None).await.expect("write_id never fails for an in-memory ByteWriter");
    out.into_bytes()
}

pub async fn decode_doc(payload: &[u8], dict: &DictReader) -> Result<(String, String), ProtocolError> {
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("doc", format).await);
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
    out.write_u8(tag);
    if let Some(text) = &op.text {
        write_str_field(out, text).await;
    }
    if let Some(binary) = &op.binary {
        out.write_varint_u64(binary.len() as u64);
        out.write_bytes(binary);
    }
    Ok(())
}

/// @emoji 🎯️ Inverse of [`write_op_payload`].
async fn read_op_payload(input: &mut ByteReader<'_>) -> Result<OpPayload, ProtocolError> {
    let op_tag = input.read_u8()?;
    if op_tag & 0b11 == 0 {
        return Err(ProtocolError::Malformed { what: "op payload", offset: 0, detail: "requires text or binary bit set".to_string() });
    }
    let text = if op_tag & 0b01 != 0 { Some(read_str_field(input).await?) } else { None };
    let binary = if op_tag & 0b10 != 0 {
        let len = input.read_varint_u64()? as usize;
        Some(input.read_bytes(len)?.to_vec())
    } else {
        None
    };
    Ok(OpPayload { text, binary })
}

async fn write_op_meta(out: &mut ByteWriter, meta: &HistoryOpMeta, dict: &mut DictBuilder, edit_ordinal_of: &(dyn Fn(&str) -> Option<u64> + Send + Sync)) -> Result<(), ProtocolError> {
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
    out.write_u8(presence);
    if let Some(op_id) = &meta.op_id {
        write_id_field(out, op_id, dict, edit_ordinal_of).await?;
    }
    out.write_varint_u64(meta.dependencies.len() as u64);
    for dep in &meta.dependencies {
        write_id_field(out, dep, dict, edit_ordinal_of).await?;
    }
    out.write_varint_u64(meta.base_version);
    if let Some(author) = &meta.author_id {
        write_id_field(out, author, dict, edit_ordinal_of).await?;
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
    // 🎯️ Appended past the pre-existing tail (bit4 of the same presence byte) — a decoder reading
    // a byte-log written before this field existed sees bit4 unset (that bit never existed in the
    // old presence byte, so it always tests as 0) and recovers `group_id: None`.
    if let Some(group_id) = &meta.group_id {
        write_id_field(out, group_id, dict, edit_ordinal_of).await?;
    }
    // 🎯️ Appended past `group_id` (bit5 of the same presence byte, same "absent for logs predating
    // this field" contract) — canonical-JSON, not dict-interned: unlike `group_id`, an origin's
    // `Contributed`/`Transaction` payload is structured data that won't repeat verbatim across
    // siblings the way a shared composite-gesture id does.
    if !meta.origin.is_owner() {
        let encoded = crate::os_pack::json::to_json_string(&meta.origin);
        write_str_field(out, &encoded).await;
    }
    // 🎯️ Appended past `origin` (bit6 of the same presence byte, same "absent for logs predating
    // this field" contract) — the durable message ledger, not reproducible from a fresh replay.
    if !meta.messages.is_empty() {
        out.write_varint_u64(meta.messages.len() as u64);
        for message in &meta.messages {
            write_history_message(out, message, dict).await?;
        }
    }
    Ok(())
}

async fn read_op_meta<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync)) -> Result<HistoryOpMeta, ProtocolError> {
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
    let group_id = if presence & (1 << 4) != 0 { Some(read_id_field(input, dict, ordinal_to_id)?) } else { None };
    let origin = if presence & (1 << 5) != 0 {
        let encoded = read_str_field(input).await?;
        crate::os_pack::json::from_json_str(&encoded).map_err(|error| ProtocolError::Malformed { what: "op meta origin", offset: 0, detail: error.to_string() })?
    } else {
        crate::os_spr::command::MutationOrigin::Owner
    };
    let messages = if presence & (1 << 6) != 0 {
        let count = input.read_varint_u64()?;
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

pub async fn encode_edit(edit: &HistoryEdit, dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64> + Send + Sync) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &(dyn Fn(&str) -> Option<u64> + Send + Sync) = &edit_ordinal_of;
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
    if !edit.inverse.is_empty() {
        presence |= 1 << 5;
    }
    if edit.lane.is_some() {
        presence |= 1 << 6;
    }
    out.write_u8(presence);
    write_id_field(&mut out, &edit.id, dict, &|_: &str| None).await?;
    let mut prev_epoch_ms = crate::os_spr::scalar::write_timestamp(&mut out, &edit.started_at, None);
    if let Some(actor) = &edit.actor {
        write_id_field(&mut out, actor, dict, edit_ordinal_of).await?;
    }
    if let Some(finished) = &edit.finished_at {
        prev_epoch_ms = crate::os_spr::scalar::write_timestamp(&mut out, finished, prev_epoch_ms);
    }
    let _ = prev_epoch_ms;
    if let Some(key) = &edit.coalesce_key {
        write_str_field(&mut out, key).await;
    }
    if let Some(description) = &edit.description {
        write_str_field(&mut out, description).await;
    }
    if let Some(lane) = &edit.lane {
        write_str_field(&mut out, lane).await;
    }
    if edit.ops.len() as u64 > ProtocolLimits::default().max_op_count_per_edit as u64 {
        return Err(ProtocolError::LimitExceeded("edit op count exceeds ProtocolLimits::max_op_count_per_edit"));
    }
    out.write_varint_u64(edit.ops.len() as u64);
    for op in &edit.ops {
        write_op_payload(&mut out, op).await?;
    }
    if !edit.inverse.is_empty() {
        out.write_varint_u64(edit.inverse.len() as u64);
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
    Ok(out.into_bytes())
}

pub async fn decode_edit<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync) -> Result<HistoryEdit, ProtocolError> {
    let ordinal_to_id: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync) = &ordinal_to_id;
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("edit", format).await);
    }
    let presence = input.read_u8()?;
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    let (started_at, mut prev_epoch_ms) = crate::os_spr::scalar::read_timestamp(&mut input, None)?;
    let actor = if presence & (1 << 0) != 0 { Some(read_id_field(&mut input, dict, ordinal_to_id)?) } else { None };
    let finished_at = if presence & (1 << 1) != 0 {
        let (s, p) = crate::os_spr::scalar::read_timestamp(&mut input, prev_epoch_ms)?;
        prev_epoch_ms = p;
        Some(s)
    } else {
        None
    };
    let _ = prev_epoch_ms;
    let coalesce_key = if presence & (1 << 2) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let description = if presence & (1 << 3) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let lane = if presence & (1 << 6) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let op_count = input.read_varint_u64()?;
    let max_ops = ProtocolLimits::default().max_op_count_per_edit as u64;
    if op_count > max_ops {
        return Err(ProtocolError::LimitExceeded("edit op count exceeds ProtocolLimits::max_op_count_per_edit"));
    }
    let mut ops = Vec::with_capacity(op_count as usize);
    for _ in 0..op_count {
        ops.push(read_op_payload(&mut input).await?);
    }
    let inverse = if presence & (1 << 5) != 0 {
        let back_count = input.read_varint_u64()?;
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
    Ok(HistoryEdit { id, actor, started_at, finished_at, coalesce_key, description, ops, inverse, meta, lane })
}
//#endregion 🔖️Edit

//#region 🔖️Transition
/// @emoji 🔀️ Caller-defined extension record (the 0x40..=0x7E range, next to `REC_COMPOSITION`/
/// `REC_CONFLICT`) carrying one history transition. Written CRITICAL: a reader that skipped it
/// would fold a different history, so it must refuse the file instead. One frame per transition,
/// appended exactly like `REC_EDIT`.
pub const REC_TRANSITION: u8 = 0x43;

/// @emoji 🎯️ `format u8 (=1) | id(idfield) | actor(idfield) | hlt(actor varint, physical_ms
/// varint, logical varint) | dependency_count varint + dependency(idfield)* | payload_len varint +
/// payload`. Ids, actor and dependencies are dict-interned like every other identifier here.
pub async fn encode_transition(transition: &HistoryTransitionRecord, dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let mut out = ByteWriter::new();
    out.write_u8(1);
    write_id_field(&mut out, &transition.id, dict, &|_: &str| None).await?;
    write_id_field(&mut out, &transition.actor, dict, &|_: &str| None).await?;
    out.write_varint_u64(transition.hlt.0);
    out.write_varint_u64(transition.hlt.1);
    out.write_varint_u64(transition.hlt.2);
    out.write_varint_u64(transition.dependencies.len() as u64);
    for dependency in &transition.dependencies {
        write_id_field(&mut out, dependency, dict, &|_: &str| None).await?;
    }
    out.write_varint_u64(transition.payload.len() as u64);
    out.write_bytes(&transition.payload);
    Ok(out.into_bytes())
}

/// @emoji 🎯️ Inverse of [`encode_transition`]; refuses trailing payload bytes.
pub async fn decode_transition(payload: &[u8], dict: &DictReader) -> Result<HistoryTransitionRecord, ProtocolError> {
    let miss = &|ord: u64| Err(ProtocolError::DictMiss(ord as u32));
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("transition", format).await);
    }
    let id = read_id_field(&mut input, dict, miss)?;
    let actor = read_id_field(&mut input, dict, miss)?;
    let hlt = (input.read_varint_u64()?, input.read_varint_u64()?, input.read_varint_u64()?);
    let dependency_count = input.read_varint_u64()?;
    let mut dependencies = Vec::with_capacity(dependency_count.min(input.remaining() as u64) as usize);
    for _ in 0..dependency_count {
        dependencies.push(read_id_field(&mut input, dict, miss)?);
    }
    let len = input.read_varint_u64()? as usize;
    let transition_payload = input.read_bytes(len)?.to_vec();
    if input.remaining() != 0 {
        return Err(ProtocolError::Malformed { what: "transition", offset: input.position() as u64, detail: "trailing payload bytes".to_string() });
    }
    Ok(HistoryTransitionRecord { id, actor, hlt, dependencies, payload: transition_payload })
}
//#endregion 🔖️Transition

//#region 🔖️Composition
/// @emoji 🧩️ Caller-defined extension record in the 0x40..=0x7E range, written NON-critical: a
/// reader that does not know about composition skips it under the standard skip-unknown rule and
/// still reads a fully valid document. Last-wins.
pub const REC_COMPOSITION: u8 = 0x41;

/// @emoji 🧩️ `format u8 (=1) | presence u8 (bit0 owner, bit1 dialect) | [owner triple] |
/// [dialect triple]`. Every string goes through the shared dictionary via `write_id_field`, same
/// as every other identifier in this crate.
pub async fn encode_composition(composition: &HistoryComposition, dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let plain: &(dyn Fn(&str) -> Option<u64> + Send + Sync) = &|_: &str| None;
    let mut out = ByteWriter::new();
    out.write_u8(1);
    let presence = u8::from(composition.owner.is_some()) | (u8::from(composition.dialect.is_some()) << 1);
    out.write_u8(presence);
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
    Ok(out.into_bytes())
}

/// @emoji 🧩️ Inverse of [`encode_composition`].
pub async fn decode_composition<'d>(payload: &[u8], dict: &'d DictReader) -> Result<HistoryComposition, ProtocolError> {
    let miss: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync) = &|ord: u64| Err(ProtocolError::DictMiss(ord as u32));
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("composition", format).await);
    }
    let presence = input.read_u8()?;
    // 🚫️async: R10 shape 1 — `read_id_field` is async but a plain closure can't await; hoisted into
    // a nested async fn (`dict`/`miss` threaded through explicitly, since a nested fn can't capture).
    async fn read_triple<'r>(input: &mut ByteReader<'_>, dict: &'r DictReader, miss: &(dyn Fn(u64) -> Result<&'r str, ProtocolError> + Send + Sync)) -> Result<(String, String, String), ProtocolError> {
        Ok((read_id_field(input, dict, miss)?, read_id_field(input, dict, miss)?, read_id_field(input, dict, miss)?))
    }
    let owner = if presence & 1 != 0 { Some(read_triple(&mut input, dict, miss).await?) } else { None };
    let dialect = if presence & 2 != 0 { Some(read_triple(&mut input, dict, miss).await?) } else { None };
    Ok(HistoryComposition { owner, dialect })
}
//#endregion 🔖️Composition

//#region 🔖️Conflict
/// @emoji ⚔️ Caller-defined extension record (`REC_COMPOSITION`'s neighbour in the 0x40..=0x7E
/// range), written NON-critical for the same reason: a reader that doesn't know
/// about first-class conflicts (`📋️contract-freeze.md` §C5/§C7) skips the whole record and still
/// reads a fully valid document. Unlike `REC_EDIT` (one frame per edit, the hot streaming path),
/// the whole `HistoryLog.conflicts` list is written as ONE frame — conflicts are not append-only
/// hot data, an authority's open/resolved set is small and always persisted together, so a single
/// frame carries the whole list. Absent for every log with no open or historical conflicts (the
/// overwhelming majority) — no frame is written when `conflicts` is empty.
pub const REC_CONFLICT: u8 = 0x42;

async fn validate_conflict_tags(kind: u8, status: u8, offset: u64) -> Result<(), ProtocolError> {
    if !matches!(kind, 0 | 1) {
        return Err(ProtocolError::Malformed { what: "conflict", offset, detail: format!("unknown conflict kind {kind}") });
    }
    if !matches!(status, 0..=2) {
        return Err(ProtocolError::Malformed { what: "conflict", offset, detail: format!("unknown conflict status {status}") });
    }
    Ok(())
}

async fn write_conflict(out: &mut ByteWriter, conflict: &HistoryConflict, dict: &mut DictBuilder, edit_ordinal_of: &(dyn Fn(&str) -> Option<u64> + Send + Sync)) -> Result<(), ProtocolError> {
    validate_conflict_tags(conflict.kind, conflict.status, 0).await?;
    write_id_field(out, &conflict.id, dict, &|_: &str| None).await?;
    out.write_u8(conflict.kind);
    out.write_u8(conflict.status);
    out.write_varint_u64(conflict.actors.len() as u64);
    for actor in &conflict.actors {
        write_id_field(out, actor, dict, &|_: &str| None).await?;
    }
    out.write_varint_u64(conflict.hlt.0);
    out.write_varint_u64(conflict.hlt.1);
    out.write_varint_u64(conflict.hlt.2);
    out.write_varint_u64(conflict.edit_ids.len() as u64);
    for edit_id in &conflict.edit_ids {
        write_id_field(out, edit_id, dict, edit_ordinal_of).await?;
    }
    out.write_varint_u64(conflict.envelopes.len() as u64);
    for envelope in &conflict.envelopes {
        out.write_varint_u64(envelope.len() as u64);
        out.write_bytes(envelope);
    }
    out.write_varint_u64(conflict.messages.len() as u64);
    for message in &conflict.messages {
        write_history_message(out, message, dict).await?;
    }
    Ok(())
}

async fn read_conflict<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync)) -> Result<HistoryConflict, ProtocolError> {
    let id = read_id_field(input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    let kind = input.read_u8()?;
    let status = input.read_u8()?;
    validate_conflict_tags(kind, status, input.position() as u64 - 2).await?;
    let actor_count = input.read_varint_u64()?;
    let mut actors = Vec::with_capacity(actor_count as usize);
    for _ in 0..actor_count {
        actors.push(read_id_field(input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?);
    }
    let hlt = (input.read_varint_u64()?, input.read_varint_u64()?, input.read_varint_u64()?);
    let edit_id_count = input.read_varint_u64()?;
    let mut edit_ids = Vec::with_capacity(edit_id_count as usize);
    for _ in 0..edit_id_count {
        edit_ids.push(read_id_field(input, dict, ordinal_to_id)?);
    }
    let envelope_count = input.read_varint_u64()?;
    let mut envelopes = Vec::with_capacity(envelope_count as usize);
    for _ in 0..envelope_count {
        let len = input.read_varint_u64()? as usize;
        envelopes.push(input.read_bytes(len)?.to_vec());
    }
    let message_count = input.read_varint_u64()?;
    let mut messages = Vec::with_capacity(message_count as usize);
    for _ in 0..message_count {
        messages.push(read_history_message(input, dict).await?);
    }
    Ok(HistoryConflict { id, kind, status, actors, hlt, edit_ids, envelopes, messages })
}

/// @emoji 🎯️ `format u8 (=1) | count varint + count x conflict entry` — single top-level format
/// byte, then a length-prefixed list: each entry is `id(idfield) | kind u8 | status u8 | actor_count
/// varint + actor(idfield)* | hlt(actor varint, physical_ms varint, logical varint) |
/// edit_id_count varint + edit_id(idfield, edit-ordinal-eligible)* | envelope_count varint +
/// (len varint + raw bytes)* | message_count varint + message*` (see [`write_history_message`] for
/// one message's shape). Envelopes are opaque bytes to this crate (already-serialized
/// `crate::os_spr::causal::MutationEnvelope`s) — same "never interprets" stance the module
/// docstring states for op payloads.
pub async fn encode_conflicts(conflicts: &[HistoryConflict], dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64> + Send + Sync) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &(dyn Fn(&str) -> Option<u64> + Send + Sync) = &edit_ordinal_of;
    let mut out = ByteWriter::new();
    out.write_u8(1);
    out.write_varint_u64(conflicts.len() as u64);
    for conflict in conflicts {
        write_conflict(&mut out, conflict, dict, edit_ordinal_of).await?;
    }
    Ok(out.into_bytes())
}

/// @emoji 🎯️ Inverse of [`encode_conflicts`].
pub async fn decode_conflicts<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync) -> Result<Vec<HistoryConflict>, ProtocolError> {
    let ordinal_to_id: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync) = &ordinal_to_id;
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("conflict", format).await);
    }
    let count = input.read_varint_u64()?;
    let mut conflicts = Vec::with_capacity(count as usize);
    let mut ids = HashSet::new();
    for _ in 0..count {
        let conflict = read_conflict(&mut input, dict, ordinal_to_id).await?;
        if !ids.insert(conflict.id.clone()) {
            return Err(ProtocolError::Malformed { what: "conflict", offset: input.position() as u64, detail: format!("duplicate conflict id {}", conflict.id) });
        }
        conflicts.push(conflict);
    }
    if input.remaining() != 0 {
        return Err(ProtocolError::Malformed { what: "conflict", offset: input.position() as u64, detail: "trailing payload bytes".to_string() });
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedHistoryDecodeStep {
    Pending { completed_bytes: u64, total_bytes: u64, decoded_records: u64 },
    Ready,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedHistoryDecodePhase {
    Verify,
    Decode,
    Validate,
    Ready,
    Fault,
}

pub struct RetainedHistoryDecode {
    verifier: Option<crate::os_spr::format::retained::RetainedSprVerification>,
    log: Option<HistoryLog>,
    dict: DictReader,
    edit_ids: Vec<String>,
    trusted_end: u64,
    offset: u64,
    decoded_records: u64,
    validation_stage: u8,
    validation_index: usize,
    saw_conflicts: bool,
    limits: crate::os_spr::format::retained::RetainedSprLimits,
    require_persisted_document: bool,
    phase: RetainedHistoryDecodePhase,
}

impl RetainedHistoryDecode {
    pub fn new(total_bytes: usize, limits: crate::os_spr::format::retained::RetainedSprLimits) -> Result<Self, String> {
        let verifier = crate::os_spr::format::retained::RetainedSprVerification::new(total_bytes as u64, limits).map_err(|error| format!("SPR history admission failed: {error:?}"))?;
        Ok(Self {
            verifier: Some(verifier),
            log: Some(HistoryLog::default()),
            dict: DictReader::new(),
            edit_ids: Vec::new(),
            trusted_end: 0,
            offset: HEADER_SIZE as u64,
            decoded_records: 0,
            validation_stage: 0,
            validation_index: 0,
            saw_conflicts: false,
            limits,
            require_persisted_document: false,
            phase: RetainedHistoryDecodePhase::Verify,
        })
    }

    pub fn new_persisted_document(total_bytes: usize, limits: crate::os_spr::format::retained::RetainedSprLimits) -> Result<Self, String> {
        let mut decoder = Self::new(total_bytes, limits)?;
        decoder.require_persisted_document = true;
        Ok(decoder)
    }

    pub fn step(&mut self, bytes: &[u8], maximum_bytes: usize, maximum_records: usize) -> Result<RetainedHistoryDecodeStep, String> {
        if self.phase == RetainedHistoryDecodePhase::Fault {
            return Err("SPR history decoder is faulted".into());
        }
        if self.phase == RetainedHistoryDecodePhase::Ready {
            return Ok(RetainedHistoryDecodeStep::Ready);
        }
        if bytes.len() as u64 > self.limits.file_bytes {
            self.phase = RetainedHistoryDecodePhase::Fault;
            return Err("SPR history exceeds retained file byte authority".into());
        }
        if self.phase == RetainedHistoryDecodePhase::Verify {
            let verifier = self.verifier.as_mut().ok_or_else(|| "SPR history verifier owner is absent".to_string())?;
            if verifier.consumed() > bytes.len() as u64 {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history input changed during retained verification".into());
            }
            let start = verifier.consumed() as usize;
            let end = start.saturating_add(maximum_bytes).min(bytes.len());
            let mut fuel = end - start;
            if fuel != 0 {
                if let Err(error) = verifier.push(&bytes[start..end], &mut fuel) {
                    self.phase = RetainedHistoryDecodePhase::Fault;
                    return Err(format!("SPR history verification failed: {error:?}"));
                }
            }
            if verifier.consumed() != bytes.len() as u64 {
                return Ok(self.progress(bytes.len()));
            }
            let span = match verifier.finish() {
                Ok(span) => span,
                Err(error) => {
                    self.phase = RetainedHistoryDecodePhase::Fault;
                    return Err(format!("SPR history verification failed: {error:?}"));
                }
            };
            if span.sequence() == 0 || span.end() != bytes.len() as u64 || span.tail() != 0 {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history requires one exact committed span without a torn or uncommitted tail".into());
            }
            self.trusted_end = span.end();
            self.verifier = None;
            self.phase = RetainedHistoryDecodePhase::Decode;
            if maximum_records == 0 {
                return Ok(self.progress(bytes.len()));
            }
        }

        if self.phase == RetainedHistoryDecodePhase::Validate {
            for _ in 0..maximum_records {
                match self.validate_one() {
                    Ok(true) => {
                        self.phase = RetainedHistoryDecodePhase::Ready;
                        return Ok(RetainedHistoryDecodeStep::Ready);
                    }
                    Ok(false) => {}
                    Err(error) => {
                        self.phase = RetainedHistoryDecodePhase::Fault;
                        return Err(error);
                    }
                }
            }
            return Ok(self.progress(bytes.len()));
        }

        let trusted_end = usize::try_from(self.trusted_end).map_err(|_| "SPR history trusted span exceeds this platform".to_string())?;
        if trusted_end > bytes.len() {
            self.phase = RetainedHistoryDecodePhase::Fault;
            return Err("SPR history input shortened after retained verification".into());
        }
        for _ in 0..maximum_records {
            if self.offset == self.trusted_end {
                self.phase = RetainedHistoryDecodePhase::Validate;
                return Ok(self.progress(bytes.len()));
            }
            if self.offset < HEADER_SIZE as u64 || self.offset > self.trusted_end {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history semantic cursor escaped its verified span".into());
            }
            let mut cursor = crate::os_io::resolve_ready(FrameCursor::new(&bytes[..trusted_end], self.offset));
            let frame = crate::os_io::resolve_ready(cursor.next_frame())
                .map_err(|error| {
                    self.phase = RetainedHistoryDecodePhase::Fault;
                    format!("SPR history record framing failed: {error}")
                })?
                .ok_or_else(|| {
                    self.phase = RetainedHistoryDecodePhase::Fault;
                    "SPR history ended before its verified span".to_string()
                })?;
            let frame_len = crate::os_io::resolve_ready(frame.frame_len());
            if frame_len > self.limits.frame_body_bytes.saturating_add(18) {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history record exceeds retained frame byte authority".into());
            }
            let next = frame.offset.checked_add(frame_len).ok_or_else(|| "SPR history record offset overflowed".to_string())?;
            if next > self.trusted_end {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history record crossed its verified span".into());
            }
            self.decode_frame(frame).map_err(|error| {
                self.phase = RetainedHistoryDecodePhase::Fault;
                error
            })?;
            self.offset = next;
            self.decoded_records = self.decoded_records.checked_add(1).ok_or_else(|| "SPR history decoded record count overflowed".to_string())?;
            if self.decoded_records > self.limits.records {
                self.phase = RetainedHistoryDecodePhase::Fault;
                return Err("SPR history exceeds retained record authority".into());
            }
        }
        Ok(self.progress(bytes.len()))
    }

    fn validate_one(&mut self) -> Result<bool, String> {
        let log = self.log.as_ref().ok_or_else(|| "SPR history log owner is absent".to_string())?;
        match self.validation_stage {
            0 if self.validation_index < log.edits.len() => {
                let index = self.validation_index;
                let edit = &log.edits[index];
                if edit.id.trim().is_empty() || log.edits[..index].iter().any(|prior| prior.id == edit.id) {
                    return Err(format!("SPR history repeats or omits authoritative edit {}", edit.id));
                }
                if self.require_persisted_document && edit.meta.is_none() {
                    return Err(format!("SPR history edit {} has no authoritative operation metadata", edit.id));
                }
                if edit.meta.as_ref().is_some_and(|meta| meta.len() != edit.ops.len()) || edit.ops.iter().chain(&edit.inverse).any(|payload| payload.binary.is_none() && payload.text.is_none()) {
                    return Err(format!("SPR history edit {} has malformed operations or metadata", edit.id));
                }
                self.validation_index += 1;
                Ok(false)
            }
            0 => {
                self.validation_stage = 1;
                self.validation_index = 0;
                Ok(false)
            }
            1 if self.validation_index < log.transitions.len() => {
                let index = self.validation_index;
                let transition = &log.transitions[index];
                if transition.id.trim().is_empty() || transition.actor.trim().is_empty() || log.transitions[..index].iter().any(|prior| prior.id == transition.id) {
                    return Err(format!("SPR history repeats or omits authoritative transition {}", transition.id));
                }
                if crate::os_spr::decode_history_transition(&transition.payload).is_err() {
                    return Err(format!("SPR history transition {} has a malformed payload", transition.id));
                }
                self.validation_index += 1;
                Ok(false)
            }
            1 => {
                self.validation_stage = 2;
                self.validation_index = 0;
                Ok(false)
            }
            _ => Ok(true),
        }
    }

    fn decode_frame(&mut self, frame: crate::os_spr::RecordFrame<'_>) -> Result<(), String> {
        let payload = crate::os_io::resolve_ready(frame.payload());
        let log = self.log.as_mut().ok_or_else(|| "SPR history log owner is absent".to_string())?;
        match frame.kind {
            crate::os_spr::REC_STR_DICT => crate::os_io::resolve_ready(apply_dict_record(&mut self.dict, payload)).map_err(|error| error.to_string())?,
            crate::os_spr::REC_ACTOR_DICT => {}
            crate::os_spr::REC_DOC => {
                let (doc_id, schema) = crate::os_io::resolve_ready(decode_doc(payload, &self.dict)).map_err(|error| error.to_string())?;
                log.doc_id = doc_id;
                log.schema = schema;
            }
            crate::os_spr::REC_EDIT => {
                let edit_ids = &self.edit_ids;
                let edit = crate::os_io::resolve_ready(decode_edit(payload, &self.dict, |ordinal| edit_ids.get(ordinal as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ordinal as u32))))
                    .map_err(|error| error.to_string())?;
                self.edit_ids.push(edit.id.clone());
                log.edits.push(edit);
            }
            REC_TRANSITION => log.transitions.push(crate::os_io::resolve_ready(decode_transition(payload, &self.dict)).map_err(|error| error.to_string())?),
            REC_COMPOSITION => log.composition = Some(crate::os_io::resolve_ready(decode_composition(payload, &self.dict)).map_err(|error| error.to_string())?),
            REC_CONFLICT => {
                if self.saw_conflicts {
                    return Err("SPR history repeats its conflict record".into());
                }
                let edit_ids = &self.edit_ids;
                log.conflicts = crate::os_io::resolve_ready(decode_conflicts(payload, &self.dict, |ordinal| edit_ids.get(ordinal as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ordinal as u32))))
                    .map_err(|error| error.to_string())?;
                self.saw_conflicts = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn progress(&self, total_bytes: usize) -> RetainedHistoryDecodeStep {
        let completed_bytes = self.verifier.as_ref().map_or(self.offset, crate::os_spr::format::retained::RetainedSprVerification::consumed);
        RetainedHistoryDecodeStep::Pending { completed_bytes, total_bytes: total_bytes as u64, decoded_records: self.decoded_records }
    }

    pub fn take_ready(&mut self) -> Option<HistoryLog> {
        if self.phase != RetainedHistoryDecodePhase::Ready {
            return None;
        }
        self.log.take()
    }

    pub fn take_partial(&mut self) -> Option<HistoryLog> {
        self.phase = RetainedHistoryDecodePhase::Fault;
        self.verifier = None;
        self.log.take()
    }

    pub fn take_auxiliary_owners(&mut self) -> (Vec<String>, Vec<String>) {
        (self.dict.take_entries(), std::mem::take(&mut self.edit_ids))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.verifier.is_none() && self.log.is_none() && self.dict.is_empty() && self.edit_ids.is_empty()
    }
}

impl Drop for RetainedHistoryDecode {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained history decoder reached Drop before every parsed and auxiliary owner was transferred");
    }
}

async fn flush_dict_delta<S: PackSink>(writer: &mut SprWriter<S>, dict: &DictBuilder, base: &mut u32) -> Result<(), ProtocolError> {
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
        writer.write_record(crate::os_spr::REC_STR_DICT, true, &payload.into_bytes(), CodecId(0)).await?;
        *base = len;
    }
    Ok(())
}

async fn apply_dict_record(dict: &mut DictReader, payload: &[u8]) -> Result<(), ProtocolError> {
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("dict", format).await);
    }
    let base_count = input.read_varint_u64()? as u32;
    let count = input.read_varint_u64()?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(read_str_field(&mut input).await?);
    }
    dict.extend(base_count, entries)
}

/// 🎞️ `(commit_seq, chain_hash)` out of a `REC_COMMIT` frame's payload — thin wrapper over
/// `crate::os_spr::format::parse_commit_payload` (public since that crate's own follow-up review pass)
/// so `VerificationLevel::Full`'s chain recompute doesn't need this crate's own byte-offset copy.
async fn parse_commit_fields(payload: &[u8]) -> Result<(u64, [u8; 32]), ProtocolError> {
    let commit = crate::os_spr::format::parse_commit_payload(payload)?;
    Ok((commit.commit_seq, commit.chain_hash))
}

pub async fn encode_history(log: &HistoryLog, options: &EncodeOptions) -> Result<Vec<u8>, ProtocolError> {
    if log.edits.len() as u64 > options.limits.max_record_count {
        return Err(ProtocolError::LimitExceeded("edit count exceeds ProtocolLimits::max_record_count"));
    }
    let write_options = WriteOptions { required_flags: crate::os_spr::REQUIRED_HASH_CHAIN, optional_flags: if options.canonical { crate::os_spr::OPTIONAL_CANONICAL } else { 0 } };
    let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options).await?;
    let mut dict = DictBuilder::new();
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
    for transition in &log.transitions {
        let payload = encode_transition(transition, &mut dict).await?;
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await?;
        writer.write_record(REC_TRANSITION, true, &payload, CodecId(0)).await?;
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
    let mut dict = DictReader::new();
    let mut edit_ids: Vec<String> = Vec::new();
    let mut log = HistoryLog::default();
    let mut cursor = FrameCursor::new(trusted, HEADER_SIZE as u64).await;
    let hasher = Blake3Hasher;
    let full = options.verification == VerificationLevel::Full;
    let mut running_chain = if full { hasher.hash(&trusted[..HEADER_SIZE]) } else { [0u8; 32] };
    let mut pending_digests: Vec<[u8; 32]> = Vec::new();
    let mut saw_conflicts = false;

    while let Some(frame) = cursor.next_frame().await? {
        if full && frame.kind != crate::os_spr::REC_COMMIT {
            let frame_bytes = &trusted[frame.offset as usize..(frame.offset + frame.frame_len().await) as usize];
            pending_digests.push(hasher.hash(frame_bytes));
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
            REC_TRANSITION => log.transitions.push(decode_transition(frame.payload().await, &dict).await?),
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
        let mut dict = DictBuilder::new();
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

    /// @emoji 🔀️ Appends one `REC_TRANSITION` frame (critical); returns its offset.
    pub async fn append_transition(&mut self, transition: &HistoryTransitionRecord) -> Result<u64, ProtocolError> {
        let payload = encode_transition(transition, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_TRANSITION, true, &payload, CodecId(0)).await
    }

    /// @emoji 🧩️ Appends the composition overlay record (skippable extension); returns its offset.
    pub async fn append_composition(&mut self, composition: &HistoryComposition) -> Result<u64, ProtocolError> {
        let payload = encode_composition(composition, &mut self.dict).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_COMPOSITION, false, &payload, CodecId(0)).await
    }

    /// @emoji ⚔️ Appends the log's one conflict record, resolving `Degraded` edit references against
    /// the edits appended so far; a file carries at most one such record.
    pub async fn append_conflicts(&mut self, conflicts: &[HistoryConflict]) -> Result<u64, ProtocolError> {
        let ordinals = &self.edit_ordinals;
        let payload = encode_conflicts(conflicts, &mut self.dict, |id| ordinals.get(id).copied()).await?;
        flush_dict_delta(&mut self.writer, &self.dict, &mut self.dict_base).await?;
        self.writer.write_record(REC_CONFLICT, false, &payload, CodecId(0)).await
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
        EditIter { cursor: FrameCursor::new(self.trusted, HEADER_SIZE as u64).await, dict: DictReader::new(), edit_ids: Vec::new() }
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
    let mut dict = DictReader::new();
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
    out.write_u8(kind);
    out.write_varint_u64(entries.len() as u64);
    for (a, b) in entries {
        out.write_varint_u64(*a);
        out.write_varint_u64(*b);
    }
}

async fn write_offsets_section(out: &mut ByteWriter, kind: u8, offsets: &[u64]) {
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
        let mut out = ByteWriter::new();
        out.write_u8(1);
        write_pair_section(&mut out, SEC_EDIT_OFFSETS, &self.edits).await;
        out.write_u8(SEC_CHECKPOINT_OFFSETS);
        out.write_varint_u64(self.checkpoints.len() as u64);
        for (id, offset, edit_ordinal) in &self.checkpoints {
            write_str_field(&mut out, id).await;
            out.write_varint_u64(*offset);
            out.write_varint_u64(*edit_ordinal);
        }
        write_offsets_section(&mut out, SEC_DICT_OFFSETS, &self.dict_offsets).await;
        write_pair_section(&mut out, SEC_SNAPSHOT_OFFSETS, &self.snapshots).await;
        write_offsets_section(&mut out, SEC_SEALED_OFFSETS, &self.sealed).await;
        out.into_bytes()
    }
}

pub struct IndexReader<'a> {
    edits: Vec<(u64, u64)>,
    checkpoints: Vec<(&'a str, u64, u64)>,
    snapshots: Vec<(u64, u64)>,
}

impl<'a> IndexReader<'a> {
    pub async fn open(payload: &'a [u8]) -> Result<Self, ProtocolError> {
        let mut input = ByteReader::new(payload);
        let format = input.read_u8()?;
        if format > 1 {
            return Err(malformed_fmt("index", format).await);
        }
        let mut edits = Vec::new();
        let mut checkpoints = Vec::new();
        let mut snapshots = Vec::new();
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
                SEC_SNAPSHOT_OFFSETS => {
                    for _ in 0..count {
                        let ordinal = input.read_varint_u64()?;
                        let offset = input.read_varint_u64()?;
                        snapshots.push((ordinal, offset));
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
