//! ⚙️ History text grammar and individual record payload codecs.
pub use semio_framework_os_history_schema::*;
use semio_framework_dsl_record::FieldSpec;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::JoinMode;
use semio_framework_dsl_record::ParseOptions;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::Shape;
use semio_framework_os_pack::{ByteReader, ByteWriter, CodecId, PackSink};
use protocol::format::{Blake3Hasher, FrameCursor, RecoveryMode, ReverseFrameCursor, SprWriter, VerificationLevel, WriteOptions, HEADER_SIZE};
use protocol::wire::{DictBuilder, DictReader, ProtocolError, ProtocolLimits, RecordHasher};
use std::collections::{HashMap, HashSet};


//#region 🔖️TextGrammar
// Own twin of crate::os_store::command_format::OpsHeaderLine's grammar, built directly against `dsl_schema` (never `vcs`,
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
const F_EDIT_VERB: u16 = 5;
const F_EDIT_LINE: u16 = 6;
const F_TRANSITION_ID: u16 = 0;
const F_TRANSITION_ACTOR: u16 = 1;
const F_TRANSITION_HLC: u16 = 2;
const F_TRANSITION_DEPENDENCIES: u16 = 3;
const F_TRANSITION_PAYLOAD: u16 = 4;
const F_TRANSITION_OBSERVED: u16 = 5;

fn doc_spec() -> RecordSpec {
    RecordSpec::new(Some("doc"), semio_framework_dsl_record::RecordLayout::Inline, vec![FieldSpec::new(F_DOC_ID, "", Shape::Text).positional(0), FieldSpec::new(F_DOC_SCHEMA, "schema", Shape::Text)])
}

fn edit_spec() -> RecordSpec {
    RecordSpec::new(
        Some("edit"),
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            FieldSpec::new(F_EDIT_ID, "", Shape::Text).positional(0),
            FieldSpec::new(F_EDIT_STARTED, "started", Shape::Text),
            FieldSpec::new(F_EDIT_ACTOR, "actor", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_FINISHED, "finished", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_VERB, "verb", Shape::Text).optional(),
            FieldSpec::new(F_EDIT_LINE, "line", Shape::List(Box::new(Shape::Text))),
        ],
    )
}

/// 🔀️ `transition <id> actor=<actor> hlc=<actor>,<physical_ms>,<logical> dependencies=[...]
/// payload=<base64> [observed=<id>]` — one [`HistoryTransitionRecord`].
fn transition_spec() -> RecordSpec {
    RecordSpec::new(
        Some("transition"),
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            FieldSpec::new(F_TRANSITION_ID, "", Shape::Text).positional(0),
            FieldSpec::new(F_TRANSITION_ACTOR, "actor", Shape::Text),
            FieldSpec::new(F_TRANSITION_HLC, "hlc", Shape::Tuple(Box::new(Shape::UInt), Some(3))),
            FieldSpec::new(F_TRANSITION_DEPENDENCIES, "dependencies", Shape::List(Box::new(Shape::Text))),
            FieldSpec::new(F_TRANSITION_PAYLOAD, "payload", Shape::Bytes64),
            FieldSpec::new(F_TRANSITION_OBSERVED, "observed", Shape::Text).optional(),
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
fn text_error_to_protocol(err: semio_framework_diagnostic::TextError) -> ProtocolError {
    ProtocolError::Malformed { what: "ops text", offset: err.span.line as u64, detail: err.message }
}

/// 📥️ Parses the full `.ops` text into a `HistoryLog`. Blank lines and `#`-comments
/// normalize away; a two-space-indented line under a pending `edit` header is an opaque forward
/// op line (never interpreted). Unlike `crate::os_store::text_format::replay_ops`, this never replays operation semantics
/// (ops are opaque here) — `HistoryEdit::meta`/inverse are simply never populated from text.
pub fn parse_ops_text(ops: &str) -> Result<HistoryLog, ProtocolError> {
    struct PendingEdit {
        id: String,
        actor: Option<semio_framework_value::SharedUtf8>,
        started_at: String,
        finished_at: Option<String>,
        verb: Option<String>,
        line: Option<String>,
    }

    let mut log = HistoryLog::default();
    let mut pending: Option<PendingEdit> = None;
    let mut forwards: Vec<OpPayload> = Vec::new();

    fn flush(pending: &mut Option<PendingEdit>, forwards: &mut Vec<OpPayload>, edits: &mut HistoryEdits) {
        if let Some(header) = pending.take() {
            edits.push(HistoryEdit {
                id: header.id,
                actor: header.actor,
                started_at: header.started_at,
                finished_at: header.finished_at, verb: header.verb, line: header.line,
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

        let opts = semio_framework_dsl_record::ParseOptions::default();
        let keyword = trimmed.split_whitespace().next().unwrap_or("");
        match keyword {
            "doc" => {
                let record = semio_framework_dsl_record::parse(trimmed, &doc_spec(), &opts).map_err(text_error_to_protocol)?;
                log.doc_id = required_text(&record, F_DOC_ID, "doc id")?;
                log.schema = required_text(&record, F_DOC_SCHEMA, "doc schema")?;
            }
            "edit" => {
                let record = semio_framework_dsl_record::parse(trimmed, &edit_spec(), &opts).map_err(text_error_to_protocol)?;
                pending = Some(PendingEdit {
                    id: required_text(&record, F_EDIT_ID, "edit id")?,
                    started_at: required_text(&record, F_EDIT_STARTED, "edit started")?,
                    actor: field_text(&record, F_EDIT_ACTOR).map(Into::into),
                    finished_at: field_text(&record, F_EDIT_FINISHED),
                    verb: field_text(&record, F_EDIT_VERB),
                    line: match record.get(F_EDIT_LINE) {
                        Some(FieldValue::List(values)) if values.len() <= 1 => values.first().map(|value| match value { FieldValue::Text(line) => Ok(line.clone()), _ => Err(ProtocolError::Malformed { what: "edit line", offset: 0, detail: "expected text".into() }) }).transpose()?,
                        _ => return Err(ProtocolError::Malformed { what: "edit line", offset: 0, detail: "expected explicit zero-or-one text list".into() }),
                    },
                });
                forwards = Vec::new();
            }
            "transition" => {
                let record = semio_framework_dsl_record::parse(trimmed, &transition_spec(), &opts).map_err(text_error_to_protocol)?;
                log.transitions.push(HistoryTransitionRecord {
                    id: required_text(&record, F_TRANSITION_ID, "transition id")?,
                    actor: required_text(&record, F_TRANSITION_ACTOR, "transition actor")?.into(),
                    hlt: field_hlc(&record, F_TRANSITION_HLC)?,
                    dependencies: field_text_list(&record, F_TRANSITION_DEPENDENCIES),
                    observed: field_text(&record, F_TRANSITION_OBSERVED),
                    payload: field_bytes(&record, F_TRANSITION_PAYLOAD, "transition payload")?,
                });
            }
            other => return Err(ProtocolError::Malformed { what: "ops text line", offset: 0, detail: format!("unknown line keyword '{other}'") }),
        }
    }
    flush(&mut pending, &mut forwards, &mut log.edits);
    Ok(log)
}

/// 📤️ Prints a `HistoryLog` back to `.ops` text: `doc`, every edit (header + two-space indented forward op lines), then one
/// `transition` line per [`HistoryTransitionRecord`] — the shared log only: the viewer head stays `.spr`-local
/// (`REC_VIEWER`). Errors if any op payload carries no text
/// (the binary-only `.spr` convention): this crate is schema-agnostic and cannot recover text
/// from an opaque binary payload — printing `.ops` for a real app document goes through the
/// concrete `Mutation::print_op` path instead (`crate::os_store::text_format::print_document_pack`'s `.ops` mirror).
pub fn print_ops_text(log: &HistoryLog) -> Result<String, ProtocolError> {
    let mut out = String::new();

    let doc_record = record_with(vec![(F_DOC_ID, FieldValue::Text(log.doc_id.clone())), (F_DOC_SCHEMA, FieldValue::Text(log.schema.clone()))]);
    out.push_str(&semio_framework_dsl_record::print(&doc_record, &doc_spec(), semio_framework_dsl_record::JoinMode::Inline));
    out.push('\n');

    for edit in &log.edits {
        let mut fields = vec![(F_EDIT_ID, FieldValue::Text(edit.id.clone())), (F_EDIT_STARTED, FieldValue::Text(edit.started_at.clone())), (F_EDIT_LINE, FieldValue::List(edit.line.iter().cloned().map(FieldValue::Text).collect()))];
        if let Some(actor) = &edit.actor {
            fields.push((F_EDIT_ACTOR, FieldValue::Text(actor.as_str().to_owned())));
        }
        if let Some(finished) = &edit.finished_at {
            fields.push((F_EDIT_FINISHED, FieldValue::Text(finished.clone())));
        }
        if let Some(verb) = &edit.verb {
            fields.push((F_EDIT_VERB, FieldValue::Text(verb.clone())));
        }
        out.push_str(&semio_framework_dsl_record::print(&record_with(fields), &edit_spec(), semio_framework_dsl_record::JoinMode::Inline));
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
        let mut fields = vec![
            (F_TRANSITION_ID, FieldValue::Text(transition.id.clone())),
            (F_TRANSITION_ACTOR, FieldValue::Text(transition.actor.as_str().to_owned())),
            (F_TRANSITION_HLC, FieldValue::Tuple(vec![FieldValue::UInt(transition.hlt.0), FieldValue::UInt(transition.hlt.1), FieldValue::UInt(transition.hlt.2)])),
            (F_TRANSITION_DEPENDENCIES, FieldValue::List(transition.dependencies.iter().map(|s| FieldValue::Text(s.clone())).collect())),
            (F_TRANSITION_PAYLOAD, FieldValue::Bytes64(transition.payload.clone())),
        ];
        if let Some(observed) = &transition.observed {
            fields.push((F_TRANSITION_OBSERVED, FieldValue::Text(observed.clone())));
        }
        out.push_str(&semio_framework_dsl_record::print(&record_with(fields), &transition_spec(), semio_framework_dsl_record::JoinMode::Inline));
        out.push('\n');
    }

    Ok(out)
}
//#endregion 🔖️TextGrammar

//#region 🔖️Payloads
// Binary codec for each record kind, using protocol::wire::scalar + protocol_format's frame
// writer/reader. Every payload starts `format: u8` (=1); trailing bytes are ignored on read
// (additive-evolution slot) except a critical record demands `format <= known` (all kinds here
// are critical per protocol::is_critical_kind, so every decode_* rejects format > 1).
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
    protocol::scalar::write_id(out, id, |s| dict.intern(s), edit_ordinal_of).map_err(ProtocolError::from)
}

fn read_id_field<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync)) -> Result<String, ProtocolError> {
    protocol::scalar::read_id(
        input,
        |idx: u32| dict.resolve(idx).map_err(|_| semio_framework_os_pack::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "dict index", offset: idx as u64, detail: "out of range".to_string() }),
        |ord: u64| ordinal_to_id(ord).map_err(|_| semio_framework_os_pack::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "edit ordinal", offset: ord, detail: "unresolvable".to_string() }),
    )
    .map_err(ProtocolError::from)
}

//#region 🔖️Message
// Shared wire shape for one `HistoryMessage`, used by both `HistoryOpMeta.messages` (🔖️Edit) and
// `HistoryConflict.messages` (🔖️Conflict) — one definition, both call sites.

/// 🎯️ `level u8 | code(idfield, dict-interned) | message(strfield) | target_count varint +
/// target(strfield)* | op_index presence u8 + [varint]`.
pub async fn write_history_message(out: &mut ByteWriter, message: &HistoryMessage, dict: &mut DictBuilder) -> Result<(), ProtocolError> {
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

/// 🎯️ Inverse of [`write_history_message`].
pub async fn read_history_message(input: &mut ByteReader<'_>, dict: &DictReader) -> Result<HistoryMessage, ProtocolError> {
    let level = input.read_u8()?;
    if semio_framework_diagnostic::Severity::from_u8(level).is_none() {
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
// REC_EDIT layout: format u8, presence u8 (bit0 actor, bit1 finished, bits 2-3 unassigned and refused,
// bit4 explicit_meta, bit5 has_backwards_section, bit6 lane, bit7 verb), id, started(ts), [actor(dictref)],
// [finished(ts)], [lane(str)], [verb(str)], op_count varint, op_count x op-payload (see
// write_op_payload: op_tag u8 [bit0 has_text=1 required in v1, bit1 has_binary] + text_len varint
// + utf8 + [binary_len varint + bytes iff bit1]), [iff bit5: back_count varint + back_count x
// op-payload (inverse, in apply order)], [explicit_meta iff bit4: op_count x op-meta entry (see
// write_op_meta) — always keyed by op_count, never back_count, since meta describes the forward
// ops only].

/// 🎯️ Writes one op payload: `op_tag u8 [bit0 has_text=1 required in v1, bit1 has_binary]
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

/// 🎯️ Inverse of [`write_op_payload`].
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

pub async fn write_op_meta(out: &mut ByteWriter, meta: &HistoryOpMeta, dict: &mut DictBuilder, edit_ordinal_of: &(dyn Fn(&str) -> Option<u64> + Send + Sync)) -> Result<(), ProtocolError> {
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
    if meta.transaction.is_some() {
        presence |= 1 << 7;
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
        let encoded = semio_framework_pack_json::to_json_string(&meta.origin);
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
    if let Some(transaction) = &meta.transaction {
        write_id_field(out, &transaction.id, dict, edit_ordinal_of).await?;
        write_id_field(out, &transaction.tool, dict, edit_ordinal_of).await?;
    }
    Ok(())
}

pub async fn read_op_meta<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync)) -> Result<HistoryOpMeta, ProtocolError> {
    let presence = input.read_u8()?;
    let op_id = if presence & (1 << 0) != 0 { Some(read_id_field(input, dict, ordinal_to_id)?) } else { None };
    let dep_count = input.read_varint_u64()?;
    let mut dependencies = Vec::with_capacity(dep_count as usize);
    for _ in 0..dep_count {
        dependencies.push(read_id_field(input, dict, ordinal_to_id)?);
    }
    let base_version = input.read_varint_u64()?;
    let author_id = if presence & (1 << 1) != 0 { Some(read_id_field(input, dict, ordinal_to_id)?.into()) } else { None };
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
        semio_framework_pack_json::from_json_str(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ProtocolError::Malformed { what: "op meta origin", offset: 0, detail: error.to_string() })?
    } else {
        protocol::mutation::MutationOrigin::Owner
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
    let transaction = if presence & (1 << 7) != 0 { Some(protocol::mutation::TransactionRef { id: read_id_field(input, dict, ordinal_to_id)?, tool: read_id_field(input, dict, ordinal_to_id)? }) } else { None };
    Ok(HistoryOpMeta { op_id, dependencies, base_version, author_id, hlt, undo_policy, payload_hash, group_id, origin, messages, transaction })
}

pub async fn encode_edit(edit: &HistoryEdit, dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64> + Send + Sync) -> Result<Vec<u8>, ProtocolError> {
    let edit_ordinal_of: &(dyn Fn(&str) -> Option<u64> + Send + Sync) = &edit_ordinal_of;
    let mut out = ByteWriter::new();
    out.write_u8(2);
    let mut presence = 0u8;
    if edit.actor.is_some() {
        presence |= 1 << 0;
    }
    if edit.finished_at.is_some() {
        presence |= 1 << 1;
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
    if edit.verb.is_some() {
        presence |= 1 << 7;
    }
    out.write_u8(presence);
    write_id_field(&mut out, &edit.id, dict, &|_: &str| None).await?;
    let mut prev_epoch_ms = protocol::scalar::write_timestamp(&mut out, &edit.started_at, None);
    if let Some(actor) = &edit.actor {
        write_id_field(&mut out, actor, dict, edit_ordinal_of).await?;
    }
    if let Some(finished) = &edit.finished_at {
        prev_epoch_ms = protocol::scalar::write_timestamp(&mut out, finished, prev_epoch_ms);
    }
    let _ = prev_epoch_ms;
    if let Some(lane) = &edit.lane {
        write_str_field(&mut out, lane).await;
    }
    if let Some(verb) = &edit.verb {
        write_str_field(&mut out, verb).await;
    }
    match &edit.line {
        None => out.write_u8(0),
        Some(line) => { out.write_u8(1); write_str_field(&mut out, line).await; }
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
    if format != 2 {
        return Err(malformed_fmt("edit", format).await);
    }
    let presence = input.read_u8()?;
    let id = read_id_field(&mut input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?;
    let (started_at, mut prev_epoch_ms) = protocol::scalar::read_timestamp(&mut input, None)?;
    let actor = if presence & (1 << 0) != 0 { Some(read_id_field(&mut input, dict, ordinal_to_id)?.into()) } else { None };
    let finished_at = if presence & (1 << 1) != 0 {
        let (s, p) = protocol::scalar::read_timestamp(&mut input, prev_epoch_ms)?;
        prev_epoch_ms = p;
        Some(s)
    } else {
        None
    };
    let _ = prev_epoch_ms;
    if presence & 0b1100 != 0 {
        return Err(ProtocolError::Malformed { what: "edit presence", offset: 0, detail: "bits 2 and 3 are unassigned".into() });
    }
    let lane = if presence & (1 << 6) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let verb = if presence & (1 << 7) != 0 { Some(read_str_field(&mut input).await?) } else { None };
    let line = match input.read_u8()? {
        0 => None,
        1 => Some(read_str_field(&mut input).await?),
        tag => return Err(ProtocolError::Malformed { what: "edit line", offset: 0, detail: format!("unknown line tag {tag}") }),
    };
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
    Ok(HistoryEdit { id, actor, started_at, finished_at, verb, line, ops, inverse, meta, lane })
}
//#endregion 🔖️Edit

//#region 🔖️Transition
/// 🔀️ Caller-defined extension record (the 0x40..=0x7E range, next to `REC_COMPOSITION`/
/// `REC_CONFLICT`) carrying one history transition. Written CRITICAL: a reader that skipped it
/// would fold a different history, so it must refuse the file instead. One frame per transition,
/// appended exactly like `REC_EDIT`.
pub const REC_TRANSITION: u8 = 0x43;

/// 🎯️ `format u8 | id(idfield) | actor(idfield) | hlt(actor varint, physical_ms
/// varint, logical varint) | dependency_count varint + dependency(idfield)* | payload_len varint +
/// payload | [observed(idfield)]` — format 1 without an observed operation, format 2 with one. Ids, actor, dependencies
/// and the observed operation are dict-interned like every other identifier here.
pub async fn encode_transition(transition: &HistoryTransitionRecord, dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let mut out = ByteWriter::new();
    out.write_u8(if transition.observed.is_some() { 2 } else { 1 });
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
    if let Some(observed) = &transition.observed {
        write_id_field(&mut out, observed, dict, &|_: &str| None).await?;
    }
    Ok(out.into_bytes())
}

/// 🎯️ Inverse of [`encode_transition`]; refuses trailing payload bytes.
pub async fn decode_transition(payload: &[u8], dict: &DictReader) -> Result<HistoryTransitionRecord, ProtocolError> {
    let miss = &|ord: u64| Err(ProtocolError::DictMiss(ord as u32));
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if !(1..=2).contains(&format) {
        return Err(malformed_fmt("transition", format).await);
    }
    let id = read_id_field(&mut input, dict, miss)?;
    let actor = read_id_field(&mut input, dict, miss)?.into();
    let hlt = (input.read_varint_u64()?, input.read_varint_u64()?, input.read_varint_u64()?);
    let dependency_count = input.read_varint_u64()?;
    let mut dependencies = Vec::with_capacity(dependency_count.min(input.remaining() as u64) as usize);
    for _ in 0..dependency_count {
        dependencies.push(read_id_field(&mut input, dict, miss)?);
    }
    let len = input.read_varint_u64()? as usize;
    let transition_payload = input.read_bytes(len)?.to_vec();
    let observed = if format == 2 { Some(read_id_field(&mut input, dict, miss)?) } else { None };
    if input.remaining() != 0 {
        return Err(ProtocolError::Malformed { what: "transition", offset: input.position() as u64, detail: "trailing payload bytes".to_string() });
    }
    Ok(HistoryTransitionRecord { id, actor, hlt, dependencies, observed, payload: transition_payload })
}
//#endregion 🔖️Transition

//#region 🔖️Composition
/// 🧩️ Caller-defined extension record in the 0x40..=0x7E range, written NON-critical: a
/// reader that does not know about composition skips it under the standard skip-unknown rule and
/// still reads a fully valid document. Last-wins.
pub const REC_COMPOSITION: u8 = 0x41;

/// 🧩️ `format u8 (=1) | presence u8 (bit0 owner, bit1 dialect) | [owner triple] |
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

/// 🧩️ Inverse of [`encode_composition`].
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
/// ⚔️ Caller-defined extension record (`REC_COMPOSITION`'s neighbour in the 0x40..=0x7E
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
        actors.push(read_id_field(input, dict, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32)))?.into());
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

/// 🎯️ `format u8 (=1) | count varint + count x conflict entry` — single top-level format
/// byte, then a length-prefixed list: each entry is `id(idfield) | kind u8 | status u8 | actor_count
/// varint + actor(idfield)* | hlt(actor varint, physical_ms varint, logical varint) |
/// edit_id_count varint + edit_id(idfield, edit-ordinal-eligible)* | envelope_count varint +
/// (len varint + raw bytes)* | message_count varint + message*` (see [`write_history_message`] for
/// one message's shape). Envelopes are opaque bytes to this crate (already-serialized
/// `protocol::causal::MutationEnvelope`s) — same "never interprets" stance the module
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

/// 🎯️ Inverse of [`encode_conflicts`].
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

//#region 👁ViewerHead
/// 👁 Non-critical extension: this replica's head. A hub check-in omits it, so the published
/// pair hydrates to the canonical trunk. Last-wins. Absent, the head is the trunk tip.
pub const REC_VIEWER: u8 = 0x45;

/// 👁 `format u8 (=1) | presence u8 (bit0 line, bit1 checkpoint) | [line] | [checkpoint]`.
pub async fn encode_viewer(line: Option<&str>, checkpoint: Option<&str>, dict: &mut DictBuilder) -> Result<Vec<u8>, ProtocolError> {
    let plain: &(dyn Fn(&str) -> Option<u64> + Send + Sync) = &|_: &str| None;
    let mut out = ByteWriter::new();
    out.write_u8(1);
    let presence = u8::from(line.is_some()) | (u8::from(checkpoint.is_some()) << 1);
    out.write_u8(presence);
    if let Some(line) = line {
        write_id_field(&mut out, line, dict, plain).await?;
    }
    if let Some(checkpoint) = checkpoint {
        write_id_field(&mut out, checkpoint, dict, plain).await?;
    }
    Ok(out.into_bytes())
}

/// 👁 Inverse of [`encode_viewer`].
pub async fn decode_viewer<'d>(payload: &[u8], dict: &'d DictReader) -> Result<(Option<String>, Option<String>), ProtocolError> {
    let miss: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync) = &|ord: u64| Err(ProtocolError::DictMiss(ord as u32));
    let mut input = ByteReader::new(payload);
    let format = input.read_u8()?;
    if format > 1 {
        return Err(malformed_fmt("viewer", format).await);
    }
    let presence = input.read_u8()?;
    let line = if presence & 1 != 0 { Some(read_id_field(&mut input, dict, miss)?) } else { None };
    let checkpoint = if presence & 2 != 0 { Some(read_id_field(&mut input, dict, miss)?) } else { None };
    if input.remaining() != 0 {
        return Err(ProtocolError::Malformed { what: "viewer", offset: input.position() as u64, detail: "trailing payload bytes".to_string() });
    }
    Ok((line, checkpoint))
}
//#endregion 👁ViewerHead

//#endregion 🔖️Payloads

//#region 🔖️Codec

pub async fn apply_dict_record(dict: &mut DictReader, payload: &[u8]) -> Result<(), ProtocolError> {
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
