//! 🔀️ Transition, composition, conflict and viewer payloads of the history codec, with the dictionary record applier. See [`super`].

use super::*;

//#region 🔖️Payloads
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
//#endregion 🔖️Index
