//! 🧱️ Field primitives and message codecs of the history codec: the string and id field readers and writers, the history message codec and the doc header codec. See [`super`].

use super::*;

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

pub(super) async fn malformed_fmt(what: &'static str, format: u8) -> ProtocolError {
    ProtocolError::Malformed { what, offset: 0, detail: format!("unsupported format {format}") }
}

pub(super) async fn write_str_field(out: &mut ByteWriter, s: &str) {
    out.write_varint_u64(s.len() as u64);
    out.write_bytes(s.as_bytes());
}

pub(super) async fn read_str_field(input: &mut ByteReader<'_>) -> Result<String, ProtocolError> {
    let len = input.read_varint_u64()? as usize;
    let bytes = input.read_bytes(len)?;
    std::str::from_utf8(bytes).map(str::to_string).map_err(|_| ProtocolError::Malformed { what: "utf8", offset: 0, detail: "invalid utf-8".to_string() })
}

pub(super) async fn write_id_field(out: &mut ByteWriter, id: &str, dict: &mut DictBuilder, edit_ordinal_of: &(dyn Fn(&str) -> Option<u64> + Send + Sync)) -> Result<(), ProtocolError> {
    protocol::scalar::write_id(out, id, |s| dict.intern(s), edit_ordinal_of).map_err(ProtocolError::from)
}

pub(super) fn read_id_field<'d>(input: &mut ByteReader<'_>, dict: &'d DictReader, ordinal_to_id: &(dyn Fn(u64) -> Result<&'d str, ProtocolError> + Send + Sync)) -> Result<String, ProtocolError> {
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
//#endregion 🔖️Payloads
