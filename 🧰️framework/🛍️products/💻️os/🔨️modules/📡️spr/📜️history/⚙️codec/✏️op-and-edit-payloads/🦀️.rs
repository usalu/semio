//! ✏️ Operation and edit payloads of the history codec: the operation payload and metadata readers and writers and the edit record codec. See [`super`].

use super::*;

//#region 🔖️Payloads
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
//#endregion 🔖️Payloads
