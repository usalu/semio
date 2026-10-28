//! 🔗️ Binary causal envelope, frontier and bounded batch representations.
use crate::{ArtifactDiff, FrontierSummary, InverseMutation, MutationEnvelope};
use semio_framework_value::ByteReadSource;

//#region 🔖️EnvelopeCodec
/// 🎞️ Binary record codec for `MutationEnvelope`/`FrontierSummary`, built on
/// `crate::wire::🔖️WireCodec`'s primitives — the storage/wire form `protocol_wire`'s frames
/// embed and `db_sync`'s WAL uses directly (see the amendment's "storage AND communication both
/// binary" requirement). Field declaration order, no tags — the same convention `os_dsl::op_rt` and
/// `crate::wire::WireCodec` both use.
fn encode_hlc(out: &mut Vec<u8>, hlt: &semio_framework_replication_ids::HybridLogicalTimestamp) {
    semio_framework_replication_wire_core::write_varint_u64(out, hlt.actor);
    semio_framework_replication_wire_core::write_varint_u64(out, hlt.physical_ms);
    semio_framework_replication_wire_core::write_varint_u64(out, hlt.logical);
}

pub fn decode_hlc(bytes: &[u8], pos: &mut usize) -> Result<semio_framework_replication_ids::HybridLogicalTimestamp, semio_framework_replication_error::ProtocolError> {
    let actor = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let physical_ms = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let logical = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    Ok(semio_framework_replication_ids::HybridLogicalTimestamp { actor, physical_ms, logical })
}

/// 🎯️ `mutation_id str | document_id str | actor str | dependencies vec<str> |
/// observed (0 | 1 str) | target vec<str> | diff.schema str | diff.payload bytes | inverse.schema str |
/// inverse.payload bytes | hlc | trailing flags varint (bit 0 transaction, bit 1 verb, bit 2 line) |
/// [transaction id str tool str] | [verb str] | [line str]` — an envelope with none of them keeps the single `0` flag byte.
pub fn encode_envelope(envelope: &MutationEnvelope, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &envelope.mutation_id.0);
    semio_framework_replication_wire_core::write_str(out, &envelope.document_id.0);
    semio_framework_replication_wire_core::write_str(out, &envelope.actor.0);
    semio_framework_replication_wire_core::write_varint_u64(out, envelope.dependencies.len() as u64);
    for dependency in &envelope.dependencies {
        semio_framework_replication_wire_core::write_str(out, &dependency.0);
    }
    match &envelope.observed {
        Some(observed) => {
            semio_framework_replication_wire_core::write_varint_u64(out, 1);
            semio_framework_replication_wire_core::write_str(out, &observed.0);
        }
        None => semio_framework_replication_wire_core::write_varint_u64(out, 0),
    }
    semio_framework_replication_wire_core::write_varint_u64(out, envelope.target.len() as u64);
    for segment in &envelope.target {
        semio_framework_replication_wire_core::write_str(out, segment);
    }
    semio_framework_replication_wire_core::write_str(out, &envelope.diff.schema.0);
    semio_framework_replication_wire_core::write_bytes(out, &envelope.diff.payload);
    semio_framework_replication_wire_core::write_str(out, &envelope.inverse.schema.0);
    semio_framework_replication_wire_core::write_bytes(out, &envelope.inverse.payload);
    encode_hlc(out, &envelope.timestamp);
    semio_framework_replication_wire_core::write_varint_u64(out, u64::from(envelope.transaction.is_some()) | u64::from(envelope.verb.is_some()) << 1 | u64::from(envelope.line.is_some()) << 2);
    if let Some(transaction) = &envelope.transaction {
        semio_framework_replication_wire_core::write_str(out, &transaction.id);
        semio_framework_replication_wire_core::write_str(out, &transaction.tool);
    }
    if let Some(verb) = &envelope.verb {
        semio_framework_replication_wire_core::write_str(out, verb);
    }
    if let Some(line) = &envelope.line {
        semio_framework_replication_wire_core::write_str(out, line);
    }
}

/// 📩️ Inverse of [`encode_envelope`].
pub fn decode_envelope(bytes: &[u8], pos: &mut usize) -> Result<MutationEnvelope, semio_framework_replication_error::ProtocolError> {
    let mutation_id = semio_framework_replication_ids::MutationId(semio_framework_replication_wire_core::read_str(bytes, pos)?);
    let document_id = semio_framework_replication_ids::ArtifactId(semio_framework_replication_wire_core::read_str(bytes, pos)?);
    let actor = semio_framework_replication_ids::ActorId(semio_framework_replication_wire_core::read_str(bytes, pos)?.into());
    let dependency_count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut dependencies = Vec::with_capacity((dependency_count as usize).min(bytes.len()));
    for _ in 0..dependency_count {
        dependencies.push(semio_framework_replication_ids::MutationId(semio_framework_replication_wire_core::read_str(bytes, pos)?));
    }
    let observed = match semio_framework_replication_wire_core::read_varint_u64(bytes, pos)? {
        0 => None,
        1 => Some(semio_framework_replication_ids::MutationId(semio_framework_replication_wire_core::read_str(bytes, pos)?)),
        flag => return Err(semio_framework_replication_error::ProtocolError::Malformed { what: "mutation envelope", offset: *pos as u64, detail: format!("observed flag {flag}") }),
    };
    let target_count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut target = Vec::with_capacity((target_count as usize).min(bytes.len()));
    for _ in 0..target_count {
        target.push(semio_framework_replication_wire_core::read_str(bytes, pos)?);
    }
    let diff_schema = semio_framework_replication_ids::SchemaId(semio_framework_replication_wire_core::read_str(bytes, pos)?);
    let diff_payload = semio_framework_replication_wire_core::read_bytes(bytes, pos)?;
    let inverse_schema = semio_framework_replication_ids::SchemaId(semio_framework_replication_wire_core::read_str(bytes, pos)?);
    let inverse_payload = semio_framework_replication_wire_core::read_bytes(bytes, pos)?;
    let timestamp = decode_hlc(bytes, pos)?;
    let flags = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    if flags > 0b111 {
        return Err(semio_framework_replication_error::ProtocolError::Malformed { what: "mutation envelope", offset: *pos as u64, detail: format!("trailing flags {flags}") });
    }
    let transaction = if flags & 0b01 != 0 { Some(semio_framework_replication_edit::TransactionRef { id: semio_framework_replication_wire_core::read_str(bytes, pos)?, tool: semio_framework_replication_wire_core::read_str(bytes, pos)? }) } else { None };
    let verb = if flags & 0b10 != 0 { Some(semio_framework_replication_wire_core::read_str(bytes, pos)?) } else { None };
    let line = if flags & 0b100 != 0 { Some(semio_framework_replication_wire_core::read_str(bytes, pos)?) } else { None };
    Ok(MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff: ArtifactDiff { schema: diff_schema, payload:crate::CausalPayload::try_from_slice(&diff_payload).map_err(|_|semio_framework_replication_error::ProtocolError::LimitExceeded("causal diff payload"))? }, inverse: InverseMutation { schema: inverse_schema, payload:crate::CausalPayload::try_from_slice(&inverse_payload).map_err(|_|semio_framework_replication_error::ProtocolError::LimitExceeded("causal inverse payload"))? }, timestamp, transaction, verb, line })
}

/// 📄️ `document_id str | head_edit_ordinal varint | head_edit_id str | last_commit_seq
/// varint | chain_hash 32`.
pub fn encode_frontier(f: &FrontierSummary, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &f.document_id.0);
    semio_framework_replication_wire_core::write_varint_u64(out, f.head_edit_ordinal);
    semio_framework_replication_wire_core::write_str(out, &f.head_edit_id);
    semio_framework_replication_wire_core::write_varint_u64(out, f.last_commit_seq);
    semio_framework_replication_wire_core::write_hash32(out, &f.chain_hash);
}

/// ↩️ Inverse of [`encode_frontier`].
pub fn decode_frontier(bytes: &[u8], pos: &mut usize) -> Result<FrontierSummary, semio_framework_replication_error::ProtocolError> {
    let document_id = semio_framework_replication_ids::ArtifactId(semio_framework_replication_wire_core::read_str(bytes, pos)?);
    let head_edit_ordinal = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let head_edit_id = semio_framework_replication_wire_core::read_str(bytes, pos)?;
    let last_commit_seq = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let chain_hash = semio_framework_replication_wire_core::read_hash32(bytes, pos)?;
    Ok(FrontierSummary { document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash })
}

/// 🔢️ `count varint | encode_envelope each` — for boundaries that move a whole batch of
/// envelopes as one opaque byte blob (the WIT ABI, worker frames) instead of one wire frame per
/// envelope (`ClientFrame::Commands`, which already carries `Vec<MutationEnvelope>` typed).
pub fn encode_envelopes(envelopes: &[MutationEnvelope]) -> Vec<u8> {
    let mut out = Vec::new();
    semio_framework_replication_wire_core::write_varint_u64(&mut out, envelopes.len() as u64);
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

fn document_backbone_batch_limit(limit: &'static str) -> semio_framework_replication_error::ProtocolError {
    semio_framework_replication_error::ProtocolError::LimitExceeded(limit)
}

fn document_backbone_batch_malformed(offset: usize, detail: impl Into<String>) -> semio_framework_replication_error::ProtocolError {
    semio_framework_replication_error::ProtocolError::Malformed { what: "document backbone batch", offset: offset as u64, detail: detail.into() }
}

fn read_document_backbone_u64(bytes:&(impl ByteReadSource+?Sized),position:&mut usize)->Result<u64,semio_framework_replication_error::ProtocolError>{
    let start=*position;let mut value=0u64;
    for shift in (0..70).step_by(7){let byte=bytes.byte_at(*position).ok_or_else(||document_backbone_batch_malformed(*position,"truncated"))?;*position+=1;if shift==63&&byte>1{return Err(document_backbone_batch_malformed(start,"varint-overflow"))}value|=u64::from(byte&127)<<shift;if byte&128==0{if shift!=0&&byte==0{return Err(document_backbone_batch_malformed(start,"nonminimal-varint"))}return Ok(value)}}
    Err(document_backbone_batch_malformed(start,"varint-overflow"))
}

fn read_document_backbone_count(bytes:&(impl ByteReadSource+?Sized),position:&mut usize,maximum:usize,limit:&'static str)->Result<usize,semio_framework_replication_error::ProtocolError>{let value=read_document_backbone_u64(bytes,position)?;if value>maximum as u64{return Err(document_backbone_batch_limit(limit))}Ok(value as usize)}

fn read_document_backbone_extent(bytes:&(impl ByteReadSource+?Sized),position:&mut usize,maximum:usize,limit:&'static str)->Result<usize,semio_framework_replication_error::ProtocolError>{let length=read_document_backbone_count(bytes,position,maximum,limit)?;let end=position.checked_add(length).ok_or_else(||document_backbone_batch_malformed(*position,"length-overflow"))?;if end>bytes.byte_len(){return Err(document_backbone_batch_malformed(*position,"truncated"))}Ok(end)}

fn read_document_backbone_bytes(bytes:&(impl ByteReadSource+?Sized),position:&mut usize,maximum:usize,limit:&'static str)->Result<Vec<u8>,semio_framework_replication_error::ProtocolError>{let end=read_document_backbone_extent(bytes,position,maximum,limit)?;let mut value=vec![0;end-*position];if bytes.read_bytes(*position,&mut value)!=value.len(){return Err(document_backbone_batch_malformed(*position,"truncated"))}*position=end;Ok(value)}

fn read_document_backbone_payload(bytes:&(impl ByteReadSource+?Sized),position:&mut usize,maximum:usize)->Result<crate::CausalPayload,semio_framework_replication_error::ProtocolError>{
    let end=read_document_backbone_extent(bytes,position,maximum,"payload-bytes")?;let mut output=crate::CausalPayload::default();let mut scratch=[0;256];
    while *position<end{let count=(end-*position).min(scratch.len());if bytes.read_bytes(*position,&mut scratch[..count])!=count{return Err(document_backbone_batch_malformed(*position,"truncated"))}for byte in &scratch[..count]{output.retained_bytes_mut().try_push(*byte).map_err(|_|document_backbone_batch_limit("payload-bytes"))?;}*position+=count;}
    Ok(output)
}

fn read_document_backbone_text(bytes:&(impl ByteReadSource+?Sized),position:&mut usize,maximum:usize,limit:&'static str)->Result<String,semio_framework_replication_error::ProtocolError>{let start=*position;String::from_utf8(read_document_backbone_bytes(bytes,position,maximum,limit)?).map_err(|_|document_backbone_batch_malformed(start,"utf8"))}

/// 🪢️ Decodes one terminal canonical causal batch under the shared hot-port limits,
/// checking every count and length before allocating its retained owner.
pub fn decode_document_backbone_envelopes_exact_with_limits(bytes: &(impl ByteReadSource+?Sized), limits: DocumentBackboneBatchLimitsV1) -> Result<Vec<MutationEnvelope>, semio_framework_replication_error::ProtocolError> {
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
    if bytes.byte_len() > limits.maximum_bytes {
        return Err(document_backbone_batch_limit("batch-bytes"));
    }
    let mut position = 0usize;
    let count = read_document_backbone_count(bytes, &mut position, limits.maximum_envelopes, "envelopes")?;
    let mut envelopes = Vec::with_capacity(count);
    let mut total_dependencies = 0usize;
    let mut total_target_segments = 0usize;
    let mut total_payload_bytes = 0usize;
    for _ in 0..count {
        let mutation_id = semio_framework_replication_ids::MutationId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?);
        let document_id = semio_framework_replication_ids::ArtifactId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?);
        let actor = semio_framework_replication_ids::ActorId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?.into());
        let dependency_count = read_document_backbone_count(bytes, &mut position, limits.maximum_dependencies_per_envelope, "dependencies")?;
        total_dependencies = total_dependencies.checked_add(dependency_count).ok_or_else(|| document_backbone_batch_limit("dependencies"))?;
        if total_dependencies > limits.maximum_total_dependencies {
            return Err(document_backbone_batch_limit("dependencies"));
        }
        let mut dependencies = Vec::with_capacity(dependency_count);
        for _ in 0..dependency_count {
            dependencies.push(semio_framework_replication_ids::MutationId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?));
        }
        let observed_at = position;
        let observed = match read_document_backbone_u64(bytes, &mut position)? {
            0 => None,
            1 => Some(semio_framework_replication_ids::MutationId(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?)),
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
        let diff_schema = semio_framework_replication_ids::SchemaId(read_document_backbone_text(bytes, &mut position, limits.maximum_schema_bytes, "schema-bytes")?);
        let diff_payload = read_document_backbone_payload(bytes, &mut position, limits.maximum_payload_bytes.saturating_sub(total_payload_bytes))?;
        total_payload_bytes += diff_payload.len();
        let inverse_schema = semio_framework_replication_ids::SchemaId(read_document_backbone_text(bytes, &mut position, limits.maximum_schema_bytes, "schema-bytes")?);
        let inverse_payload = read_document_backbone_payload(bytes, &mut position, limits.maximum_payload_bytes.saturating_sub(total_payload_bytes))?;
        total_payload_bytes += inverse_payload.len();
        let timestamp = semio_framework_replication_ids::HybridLogicalTimestamp { actor: read_document_backbone_u64(bytes, &mut position)?, physical_ms: read_document_backbone_u64(bytes, &mut position)?, logical: read_document_backbone_u64(bytes, &mut position)? };
        let flags_at = position;
        let flags = read_document_backbone_u64(bytes, &mut position)?;
        if flags > 0b111 {
            return Err(document_backbone_batch_malformed(flags_at, "trailing-flags"));
        }
        let transaction = if flags & 0b01 != 0 {
            Some(semio_framework_replication_edit::TransactionRef {
                id: read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?,
                tool: read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?,
            })
        } else {
            None
        };
        let verb = if flags & 0b10 != 0 { Some(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?) } else { None };
        let line = if flags & 0b100 != 0 { Some(read_document_backbone_text(bytes, &mut position, limits.maximum_identifier_bytes, "identifier-bytes")?) } else { None };
        envelopes.push(MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff: ArtifactDiff { schema: diff_schema, payload:diff_payload }, inverse: InverseMutation { schema: inverse_schema, payload:inverse_payload }, timestamp, transaction, verb, line });
    }
    if position != bytes.byte_len() {
        return Err(document_backbone_batch_malformed(position, "trailing-bytes"));
    }
    Ok(envelopes)
}

/// 🔐️ Shared production-limit decoder for one hot document-backbone mutation batch.
pub fn decode_document_backbone_envelopes_exact(bytes: &(impl ByteReadSource+?Sized)) -> Result<Vec<MutationEnvelope>, semio_framework_replication_error::ProtocolError> {
    decode_document_backbone_envelopes_exact_with_limits(bytes, DocumentBackboneBatchLimitsV1::default())
}

/// 🎯️ Inverse of [`encode_envelopes`].
pub fn decode_envelopes(bytes:&(impl ByteReadSource+?Sized))->Result<Vec<MutationEnvelope>,semio_framework_replication_error::ProtocolError>{decode_document_backbone_envelopes_exact(bytes)}

/// 📦️ `count varint | (len varint | bytes) each` — a binary vec-of-op-payloads framing,
/// replacing the `serde_json::json!({"inverse": [...]})` convention for `InverseMutation`
/// payloads that carry more than one composed op (e.g. framework/plugin's `result_from_last_edit`).
pub fn encode_ops_vec(ops: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    semio_framework_replication_wire_core::write_varint_u64(&mut out, ops.len() as u64);
    for op in ops {
        semio_framework_replication_wire_core::write_bytes(&mut out, op);
    }
    out
}

/// 🔙️ Inverse of [`encode_ops_vec`].
pub fn decode_ops_vec(bytes: &[u8]) -> Result<Vec<Vec<u8>>, semio_framework_replication_error::ProtocolError> {
    let mut pos = 0usize;
    let count = semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?;
    let mut ops = Vec::with_capacity(count as usize);
    for _ in 0..count {
        ops.push(semio_framework_replication_wire_core::read_bytes(bytes, &mut pos)?);
    }
    Ok(ops)
}
//#endregion 🔖️EnvelopeCodec

#[path="🛫️native/🦀️.rs"]
pub mod native_encoding;
