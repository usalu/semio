//! 🔗️ Binary causal envelope, frontier and bounded batch representations.
use crate::causal::{ArtifactDiff, FrontierSummary, InverseMutation, MutationEnvelope};

//#region 🔖️EnvelopeCodec
/// 🎞️ Binary record codec for `MutationEnvelope`/`FrontierSummary`, built on
/// `crate::wire::🔖️WireCodec`'s primitives — the storage/wire form `protocol_wire`'s frames
/// embed and `db_sync`'s WAL uses directly (see the amendment's "storage AND communication both
/// binary" requirement). Field declaration order, no tags — the same convention `os_dsl::op_rt` and
/// `crate::wire::WireCodec` both use.
fn encode_hlc(out: &mut Vec<u8>, hlt: &crate::ids::HybridLogicalTimestamp) {
    crate::wire::write_varint_u64(out, hlt.actor);
    crate::wire::write_varint_u64(out, hlt.physical_ms);
    crate::wire::write_varint_u64(out, hlt.logical);
}

pub(crate) fn decode_hlc(bytes: &[u8], pos: &mut usize) -> Result<crate::ids::HybridLogicalTimestamp, crate::ProtocolError> {
    let actor = crate::wire::read_varint_u64(bytes, pos)?;
    let physical_ms = crate::wire::read_varint_u64(bytes, pos)?;
    let logical = crate::wire::read_varint_u64(bytes, pos)?;
    Ok(crate::ids::HybridLogicalTimestamp { actor, physical_ms, logical })
}

/// 🎯️ `mutation_id str | document_id str | actor str | dependencies vec<str> |
/// observed (0 | 1 str) | target vec<str> | diff.schema str | diff.payload bytes | inverse.schema str |
/// inverse.payload bytes | hlc | trailing flags varint (bit 0 transaction, bit 1 verb, bit 2 line) |
/// [transaction id str tool str] | [verb str] | [line str]` — an envelope with none of them keeps the single `0` flag byte.
pub fn encode_envelope(envelope: &MutationEnvelope, out: &mut Vec<u8>) {
    crate::write_str(out, &envelope.mutation_id.0);
    crate::write_str(out, &envelope.document_id.0);
    crate::write_str(out, &envelope.actor.0);
    crate::wire::write_varint_u64(out, envelope.dependencies.len() as u64);
    for dependency in &envelope.dependencies {
        crate::write_str(out, &dependency.0);
    }
    match &envelope.observed {
        Some(observed) => {
            crate::wire::write_varint_u64(out, 1);
            crate::write_str(out, &observed.0);
        }
        None => crate::wire::write_varint_u64(out, 0),
    }
    crate::wire::write_varint_u64(out, envelope.target.len() as u64);
    for segment in &envelope.target {
        crate::write_str(out, segment);
    }
    crate::write_str(out, &envelope.diff.schema.0);
    crate::write_bytes(out, &envelope.diff.payload);
    crate::write_str(out, &envelope.inverse.schema.0);
    crate::write_bytes(out, &envelope.inverse.payload);
    encode_hlc(out, &envelope.timestamp);
    crate::wire::write_varint_u64(out, u64::from(envelope.transaction.is_some()) | u64::from(envelope.verb.is_some()) << 1 | u64::from(envelope.line.is_some()) << 2);
    if let Some(transaction) = &envelope.transaction {
        crate::write_str(out, &transaction.id);
        crate::write_str(out, &transaction.tool);
    }
    if let Some(verb) = &envelope.verb {
        crate::write_str(out, verb);
    }
    if let Some(line) = &envelope.line {
        crate::write_str(out, line);
    }
}

/// 🎯️ Inverse of [`encode_envelope`].
pub fn decode_envelope(bytes: &[u8], pos: &mut usize) -> Result<MutationEnvelope, crate::ProtocolError> {
    let mutation_id = crate::ids::MutationId(crate::read_str(bytes, pos)?);
    let document_id = crate::ids::ArtifactId(crate::read_str(bytes, pos)?);
    let actor = crate::ids::ActorId(crate::read_str(bytes, pos)?.into());
    let dependency_count = crate::wire::read_varint_u64(bytes, pos)?;
    let mut dependencies = Vec::with_capacity((dependency_count as usize).min(bytes.len()));
    for _ in 0..dependency_count {
        dependencies.push(crate::ids::MutationId(crate::read_str(bytes, pos)?));
    }
    let observed = match crate::wire::read_varint_u64(bytes, pos)? {
        0 => None,
        1 => Some(crate::ids::MutationId(crate::read_str(bytes, pos)?)),
        flag => return Err(crate::ProtocolError::Malformed { what: "mutation envelope", offset: *pos as u64, detail: format!("observed flag {flag}") }),
    };
    let target_count = crate::wire::read_varint_u64(bytes, pos)?;
    let mut target = Vec::with_capacity((target_count as usize).min(bytes.len()));
    for _ in 0..target_count {
        target.push(crate::read_str(bytes, pos)?);
    }
    let diff_schema = crate::ids::SchemaId(crate::read_str(bytes, pos)?);
    let diff_payload = crate::read_bytes(bytes, pos)?;
    let inverse_schema = crate::ids::SchemaId(crate::read_str(bytes, pos)?);
    let inverse_payload = crate::read_bytes(bytes, pos)?;
    let timestamp = decode_hlc(bytes, pos)?;
    let flags = crate::wire::read_varint_u64(bytes, pos)?;
    if flags > 0b111 {
        return Err(crate::ProtocolError::Malformed { what: "mutation envelope", offset: *pos as u64, detail: format!("trailing flags {flags}") });
    }
    let transaction = if flags & 0b01 != 0 { Some(crate::mutation::TransactionRef { id: crate::read_str(bytes, pos)?, tool: crate::read_str(bytes, pos)? }) } else { None };
    let verb = if flags & 0b10 != 0 { Some(crate::read_str(bytes, pos)?) } else { None };
    let line = if flags & 0b100 != 0 { Some(crate::read_str(bytes, pos)?) } else { None };
    Ok(MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff: ArtifactDiff { schema: diff_schema, payload: diff_payload }, inverse: InverseMutation { schema: inverse_schema, payload: inverse_payload }, timestamp, transaction, verb, line })
}

/// 🎯️ `document_id str | head_edit_ordinal varint | head_edit_id str | last_commit_seq
/// varint | chain_hash 32`.
pub fn encode_frontier(f: &FrontierSummary, out: &mut Vec<u8>) {
    crate::write_str(out, &f.document_id.0);
    crate::wire::write_varint_u64(out, f.head_edit_ordinal);
    crate::write_str(out, &f.head_edit_id);
    crate::wire::write_varint_u64(out, f.last_commit_seq);
    crate::write_hash32(out, &f.chain_hash);
}

/// 🎯️ Inverse of [`encode_frontier`].
pub fn decode_frontier(bytes: &[u8], pos: &mut usize) -> Result<FrontierSummary, crate::ProtocolError> {
    let document_id = crate::ids::ArtifactId(crate::read_str(bytes, pos)?);
    let head_edit_ordinal = crate::wire::read_varint_u64(bytes, pos)?;
    let head_edit_id = crate::read_str(bytes, pos)?;
    let last_commit_seq = crate::wire::read_varint_u64(bytes, pos)?;
    let chain_hash = crate::read_hash32(bytes, pos)?;
    Ok(FrontierSummary { document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash })
}

/// 🎯️ `count varint | encode_envelope each` — for boundaries that move a whole batch of
/// envelopes as one opaque byte blob (the WIT ABI, worker frames) instead of one wire frame per
/// envelope (`ClientFrame::Commands`, which already carries `Vec<MutationEnvelope>` typed).
pub fn encode_envelopes(envelopes: &[MutationEnvelope]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::wire::write_varint_u64(&mut out, envelopes.len() as u64);
    for envelope in envelopes {
        encode_envelope(envelope, &mut out);
    }
    out
}

pub const DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES: usize = 262_144;
pub const DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES: usize = 8_192;
pub const DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES: usize = 8_192;
pub const DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS: usize = 8_192;
pub const DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES: usize = 256;
pub const DOCUMENT_BACKBONE_BATCH_MAXIMUM_SCHEMA_BYTES: usize = 256;
pub const DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES: usize = 262_144;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DocumentBackboneBatchLimitsV1 {
    pub maximum_bytes: usize,
    pub maximum_envelopes: usize,
    pub maximum_dependencies_per_envelope: usize,
    pub maximum_total_dependencies: usize,
    pub maximum_target_segments_per_envelope: usize,
    pub maximum_total_target_segments: usize,
    pub maximum_identifier_bytes: usize,
    pub maximum_schema_bytes: usize,
    pub maximum_payload_bytes: usize,
}

impl Default for DocumentBackboneBatchLimitsV1 {
    fn default() -> Self {
        Self {
            maximum_bytes: DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES,
            maximum_envelopes: DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES,
            maximum_dependencies_per_envelope: DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES,
            maximum_total_dependencies: DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES,
            maximum_target_segments_per_envelope: DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS,
            maximum_total_target_segments: DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS,
            maximum_identifier_bytes: DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES,
            maximum_schema_bytes: DOCUMENT_BACKBONE_BATCH_MAXIMUM_SCHEMA_BYTES,
            maximum_payload_bytes: DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES,
        }
    }
}

fn document_backbone_batch_limit(limit: &'static str) -> crate::ProtocolError {
    crate::ProtocolError::LimitExceeded(limit)
}

fn document_backbone_batch_malformed(offset: usize, detail: impl Into<String>) -> crate::ProtocolError {
    crate::ProtocolError::Malformed { what: "document backbone batch", offset: offset as u64, detail: detail.into() }
}

fn read_document_backbone_u64(bytes: &[u8], position: &mut usize) -> Result<u64, crate::ProtocolError> {
    let start = *position;
    let value = crate::wire::read_varint_u64(bytes, position)?;
    let mut canonical = Vec::new();
    crate::wire::write_varint_u64(&mut canonical, value);
    if bytes.get(start..*position) != Some(canonical.as_slice()) {
        return Err(document_backbone_batch_malformed(start, "nonminimal-varint"));
    }
    Ok(value)
}

fn read_document_backbone_count(bytes: &[u8], position: &mut usize, maximum: usize, limit: &'static str) -> Result<usize, crate::ProtocolError> {
    let value = read_document_backbone_u64(bytes, position)?;
    if value > maximum as u64 {
        return Err(document_backbone_batch_limit(limit));
    }
    Ok(value as usize)
}

fn read_document_backbone_bytes(bytes: &[u8], position: &mut usize, maximum: usize, limit: &'static str) -> Result<Vec<u8>, crate::ProtocolError> {
    let length = read_document_backbone_count(bytes, position, maximum, limit)?;
    let end = (*position).checked_add(length).ok_or_else(|| document_backbone_batch_malformed(*position, "length-overflow"))?;
    let value = bytes.get(*position..end).ok_or_else(|| document_backbone_batch_malformed(*position, "truncated"))?.to_vec();
    *position = end;
    Ok(value)
}

fn read_document_backbone_text(bytes: &[u8], position: &mut usize, maximum: usize, limit: &'static str) -> Result<String, crate::ProtocolError> {
    let start = *position;
    String::from_utf8(read_document_backbone_bytes(bytes, position, maximum, limit)?).map_err(|_| document_backbone_batch_malformed(start, "utf8"))
}

/// 🪢️ Decodes one terminal canonical causal batch under the shared hot-port limits,
/// checking every count and length before allocating its retained owner.
pub fn decode_document_backbone_envelopes_exact_with_limits(bytes: &[u8], limits: DocumentBackboneBatchLimitsV1) -> Result<Vec<MutationEnvelope>, crate::ProtocolError> {
    let ceiling = DocumentBackboneBatchLimitsV1::default();
    if limits.maximum_bytes > ceiling.maximum_bytes
        || limits.maximum_envelopes > ceiling.maximum_envelopes
        || limits.maximum_dependencies_per_envelope > ceiling.maximum_dependencies_per_envelope
        || limits.maximum_total_dependencies > ceiling.maximum_total_dependencies
        || limits.maximum_target_segments_per_envelope > ceiling.maximum_target_segments_per_envelope
        || limits.maximum_total_target_segments > ceiling.maximum_total_target_segments
        || limits.maximum_identifier_bytes > ceiling.maximum_identifier_bytes
        || limits.maximum_schema_bytes > ceiling.maximum_schema_bytes
        || limits.maximum_payload_bytes > ceiling.maximum_payload_bytes
    {
        return Err(document_backbone_batch_limit("invalid-limits"));
    }
    if bytes.len() > limits.maximum_bytes {
        return Err(document_backbone_batch_limit("batch-bytes"));
    }
    let mut position = 0usize;
    let count = read_document_backbone_count(bytes, &mut position, limits.maximum_envelopes, "envelopes")?;
    let mut envelopes = Vec::with_capacity(count);
    let mut total_dependencies = 0usize;
    let mut total_target_segments = 0usize;
    let mut total_payload_bytes = 0usize;
    for _ in 0..count {
        let mutation_id = crate::ids::MutationId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?);
        let document_id = crate::ids::ArtifactId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?);
        let actor = crate::ids::ActorId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?.into());
        let dependency_count = read_document_backbone_count(bytes, &mut position, limits.maximum_dependencies_per_envelope, "dependencies")?;
        total_dependencies = total_dependencies.checked_add(dependency_count).ok_or_else(|| document_backbone_batch_limit("dependencies"))?;
        if total_dependencies > limits.maximum_total_dependencies {
            return Err(document_backbone_batch_limit("dependencies"));
        }
        let mut dependencies = Vec::with_capacity(dependency_count);
        for _ in 0..dependency_count {
            dependencies.push(crate::ids::MutationId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?));
        }
        let observed_at = position;
        let observed = match read_document_backbone_u64(bytes, &mut position)? {
            0 => None,
            1 => Some(crate::ids::MutationId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?)),
            _ => return Err(document_backbone_batch_malformed(observed_at, "observed-flag")),
        };
        let target_count = read_document_backbone_count(bytes, &mut position, limits.maximum_target_segments_per_envelope, "target-segments")?;
        total_target_segments = total_target_segments.checked_add(target_count).ok_or_else(|| document_backbone_batch_limit("target-segments"))?;
        if total_target_segments > limits.maximum_total_target_segments {
            return Err(document_backbone_batch_limit("target-segments"));
        }
        let mut target = Vec::with_capacity(target_count);
        for _ in 0..target_count {
            target.push(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?);
        }
        let diff_schema = crate::ids::SchemaId(read_document_backbone_text(bytes, &mut position, limits.maximum_schema_bytes, "schema-bytes")?);
        let diff_payload = read_document_backbone_bytes(bytes, &mut position, limits.maximum_payload_bytes.saturating_sub(total_payload_bytes), "payload-bytes")?;
        total_payload_bytes += diff_payload.len();
        let inverse_schema = crate::ids::SchemaId(read_document_backbone_text(bytes, &mut position, limits.maximum_schema_bytes, "schema-bytes")?);
        let inverse_payload = read_document_backbone_bytes(bytes, &mut position, limits.maximum_payload_bytes.saturating_sub(total_payload_bytes), "payload-bytes")?;
        total_payload_bytes += inverse_payload.len();
        let timestamp = crate::ids::HybridLogicalTimestamp { actor: read_document_backbone_u64(bytes, &mut position)?, physical_ms: read_document_backbone_u64(bytes, &mut position)?, logical: read_document_backbone_u64(bytes, &mut position)? };
        let flags_at = position;
        let flags = read_document_backbone_u64(bytes, &mut position)?;
        if flags > 0b111 {
            return Err(document_backbone_batch_malformed(flags_at, "trailing-flags"));
        }
        let transaction = if flags & 0b01 != 0 {
            Some(crate::mutation::TransactionRef {
                id: read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?,
                tool: read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?,
            })
        } else {
            None
        };
        let verb = if flags & 0b10 != 0 { Some(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?) } else { None };
        let line = if flags & 0b100 != 0 { Some(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?) } else { None };
        envelopes.push(MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff: ArtifactDiff { schema: diff_schema, payload: diff_payload }, inverse: InverseMutation { schema: inverse_schema, payload: inverse_payload }, timestamp, transaction, verb, line });
    }
    if position != bytes.len() {
        return Err(document_backbone_batch_malformed(position, "trailing-bytes"));
    }
    if encode_envelopes(&envelopes) != bytes {
        return Err(document_backbone_batch_malformed(0, "noncanonical"));
    }
    Ok(envelopes)
}

/// 🔐️ Shared production-limit decoder for one hot document-backbone mutation batch.
pub fn decode_document_backbone_envelopes_exact(bytes: &[u8]) -> Result<Vec<MutationEnvelope>, crate::ProtocolError> {
    decode_document_backbone_envelopes_exact_with_limits(bytes, DocumentBackboneBatchLimitsV1::default())
}

/// 🎯️ Inverse of [`encode_envelopes`].
pub fn decode_envelopes(bytes: &[u8]) -> Result<Vec<MutationEnvelope>, crate::ProtocolError> {
    let mut pos = 0usize;
    let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
    let mut envelopes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        envelopes.push(decode_envelope(bytes, &mut pos)?);
    }
    Ok(envelopes)
}

/// 🎯️ `count varint | (len varint | bytes) each` — a binary vec-of-op-payloads framing,
/// replacing the `serde_json::json!({"inverse": [...]})` convention for `InverseMutation`
/// payloads that carry more than one composed op (e.g. framework/plugin's `result_from_last_edit`).
pub fn encode_ops_vec(ops: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::wire::write_varint_u64(&mut out, ops.len() as u64);
    for op in ops {
        crate::write_bytes(&mut out, op);
    }
    out
}

/// 🎯️ Inverse of [`encode_ops_vec`].
pub fn decode_ops_vec(bytes: &[u8]) -> Result<Vec<Vec<u8>>, crate::ProtocolError> {
    let mut pos = 0usize;
    let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
    let mut ops = Vec::with_capacity(count as usize);
    for _ in 0..count {
        ops.push(crate::read_bytes(bytes, &mut pos)?);
    }
    Ok(ops)
}
//#endregion 🔖️EnvelopeCodec
