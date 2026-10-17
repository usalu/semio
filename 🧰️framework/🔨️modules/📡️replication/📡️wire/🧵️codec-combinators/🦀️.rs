//! 🧵️ Frame codec parts: the frontier and envelope vector combinators, the artifact bootstrap codec, the bootstrap, apply outcome and ack stage codecs. See [`super`].

use super::*;

//#region 🔖️Codec
//#region 🔖️Combinators
pub(super) async fn write_opt_frontier(out: &mut Vec<u8>, value: &Option<semio_framework_replication_causal::FrontierSummary>) {
    semio_framework_replication_wire_core::write_bool(out, value.is_some());
    if let Some(f) = value {
        semio_framework_replication_causal::binary::encode_frontier(f, out);
    }
}

pub(super) async fn read_opt_frontier(bytes: &[u8], pos: &mut usize) -> Result<Option<semio_framework_replication_causal::FrontierSummary>, semio_framework_replication_error::ProtocolError> {
    if semio_framework_replication_wire_core::read_bool(bytes, pos)? {
        Ok(Some(semio_framework_replication_causal::binary::decode_frontier(bytes, pos)?))
    } else {
        Ok(None)
    }
}

pub(super) async fn write_vec_envelope(out: &mut Vec<u8>, values: &[semio_framework_replication_causal::MutationEnvelope]) {
    semio_framework_replication_wire_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        semio_framework_replication_causal::binary::encode_envelope(value, out);
    }
}

pub(super) async fn read_vec_envelope(bytes: &[u8], pos: &mut usize) -> Result<Vec<semio_framework_replication_causal::MutationEnvelope>, semio_framework_replication_error::ProtocolError> {
    let count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity((count as usize).min(bytes.len()));
    for _ in 0..count {
        out.push(semio_framework_replication_causal::binary::decode_envelope(bytes, pos)?);
    }
    Ok(out)
}
//#endregion 🔖️Combinators

//#region 🔖️NestedEnums
fn encode_artifact_bootstrap(bootstrap: &ArtifactBootstrap, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_varint_u64(out, bootstrap.format_version as u64);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.descriptor_hash);
    semio_framework_replication_wire_core::write_str(out, &bootstrap.artifact_schema);
    semio_framework_replication_wire_core::write_str(out, &bootstrap.artifact_kind);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.pack_schema_hash);
    semio_framework_replication_causal::binary::encode_frontier(&bootstrap.baseline_frontier, out);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.pack_hash);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.spr_hash);
    semio_framework_replication_wire_core::write_varint_u64(out, bootstrap.pack_length);
    semio_framework_replication_wire_core::write_varint_u64(out, bootstrap.spr_length);
    semio_framework_replication_wire_core::write_varint_u64(out, bootstrap.chunk_count as u64);
    semio_framework_replication_wire_core::write_hash32(out, &bootstrap.aggregate_hash);
    semio_framework_replication_causal::binary::encode_frontier(&bootstrap.required_tail_frontier, out);
    semio_framework_replication_wire_core::write_bool(out, bootstrap.inline.is_some());
    if let Some(pair) = &bootstrap.inline {
        semio_framework_replication_wire_core::write_bytes(out, &pair.pack);
        semio_framework_replication_wire_core::write_bytes(out, &pair.spr);
    }
}

pub(super) fn read_artifact_bootstrap_string(bytes: &[u8], pos: &mut usize, name: &'static str) -> Result<String, semio_framework_replication_error::ProtocolError> {
    let length = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    if length == 0 || length > 256 {
        return Err(artifact_bootstrap_error(format!("{name} length is invalid")));
    }
    let length = length as usize;
    let end = pos.checked_add(length).ok_or_else(|| artifact_bootstrap_error(format!("{name} length overflow")))?;
    let value = bytes.get(*pos..end).ok_or_else(|| artifact_bootstrap_error(format!("{name} is truncated")))?;
    let value = std::str::from_utf8(value).map_err(|_| artifact_bootstrap_error(format!("{name} is not UTF-8")))?.to_string();
    *pos = end;
    Ok(value)
}

fn read_artifact_bootstrap_bytes(bytes: &[u8], pos: &mut usize, expected: u64, name: &'static str) -> Result<Vec<u8>, semio_framework_replication_error::ProtocolError> {
    let length = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    if length != expected || length > ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES {
        return Err(artifact_bootstrap_error(format!("{name} length does not match metadata")));
    }
    let length = length as usize;
    let end = pos.checked_add(length).ok_or_else(|| artifact_bootstrap_error(format!("{name} length overflow")))?;
    let value = bytes.get(*pos..end).ok_or_else(|| artifact_bootstrap_error(format!("{name} is truncated")))?.to_vec();
    *pos = end;
    Ok(value)
}

fn decode_artifact_bootstrap(bytes: &[u8], pos: &mut usize) -> Result<ArtifactBootstrap, semio_framework_replication_error::ProtocolError> {
    let format_version = u32::try_from(semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?).map_err(|_| artifact_bootstrap_error("format version exceeds u32"))?;
    let mut bootstrap = ArtifactBootstrap {
        format_version,
        descriptor_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        artifact_schema: read_artifact_bootstrap_string(bytes, pos, "artifact schema")?,
        artifact_kind: read_artifact_bootstrap_string(bytes, pos, "artifact kind")?,
        pack_schema_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        baseline_frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, pos)?,
        pack_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        spr_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        pack_length: semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?,
        spr_length: semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?,
        chunk_count: u32::try_from(semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?).map_err(|_| artifact_bootstrap_error("chunk count exceeds u32"))?,
        aggregate_hash: semio_framework_replication_wire_core::read_hash32(bytes, pos)?,
        required_tail_frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, pos)?,
        inline: None,
    };
    let inline = semio_framework_replication_wire_core::read_bool(bytes, pos)?;
    validate_artifact_bootstrap_header(&bootstrap, inline, ArtifactBootstrapLimits::default())?;
    if inline {
        bootstrap.inline = Some(ArtifactBootstrapPair { pack: read_artifact_bootstrap_bytes(bytes, pos, bootstrap.pack_length, "inline pack")?, spr: read_artifact_bootstrap_bytes(bytes, pos, bootstrap.spr_length, "inline SPR")? });
    }
    validate_artifact_bootstrap(&bootstrap, ArtifactBootstrapLimits::default())?;
    Ok(bootstrap)
}

pub(super) async fn encode_bootstrap(bootstrap: &Bootstrap, out: &mut Vec<u8>) {
    match bootstrap {
        Bootstrap::None => out.push(0),
        Bootstrap::Snapshot { pack_hash, inline } => {
            out.push(1);
            semio_framework_replication_wire_core::write_hash32(out, pack_hash);
            write_opt_bytes(out, inline).await;
        }
        Bootstrap::Tail => out.push(2),
        Bootstrap::ArtifactBootstrap(bootstrap) => {
            out.push(3);
            encode_artifact_bootstrap(bootstrap, out);
        }
    }
}

pub(super) async fn decode_bootstrap(bytes: &[u8], pos: &mut usize) -> Result<Bootstrap, semio_framework_replication_error::ProtocolError> {
    let tag = match bytes.get(*pos) {
        Some(b) => *b,
        None => return Err(malformed("wire bootstrap tag", *pos as u64, "truncated").await),
    };
    *pos += 1;
    match tag {
        0 => Ok(Bootstrap::None),
        1 => {
            let pack_hash = semio_framework_replication_wire_core::read_hash32(bytes, pos)?;
            let inline = read_opt_bytes(bytes, pos).await?;
            Ok(Bootstrap::Snapshot { pack_hash, inline })
        }
        2 => Ok(Bootstrap::Tail),
        3 => Ok(Bootstrap::ArtifactBootstrap(Box::new(decode_artifact_bootstrap(bytes, pos)?))),
        other => Err(malformed("wire bootstrap tag", *pos as u64, &format!("unknown tag {other:#x}")).await),
    }
}

async fn encode_apply_outcome(outcome: &ApplyOutcome, out: &mut Vec<u8>) {
    match outcome {
        ApplyOutcome::Accepted => out.push(0),
        ApplyOutcome::Transformed { envelope } => {
            out.push(1);
            semio_framework_replication_causal::binary::encode_envelope(envelope, out);
        }
        ApplyOutcome::Rejected { reason, messages } => {
            out.push(2);
            semio_framework_replication_wire_core::write_str(out, reason);
            semio_framework_replication_wire_core::write_bytes(out, messages);
        }
    }
}

async fn decode_apply_outcome(bytes: &[u8], pos: &mut usize) -> Result<ApplyOutcome, semio_framework_replication_error::ProtocolError> {
    let tag = match bytes.get(*pos) {
        Some(b) => *b,
        None => return Err(malformed("wire apply-outcome tag", *pos as u64, "truncated").await),
    };
    *pos += 1;
    match tag {
        0 => Ok(ApplyOutcome::Accepted),
        1 => Ok(ApplyOutcome::Transformed { envelope: Box::new(semio_framework_replication_causal::binary::decode_envelope(bytes, pos)?) }),
        2 => Ok(ApplyOutcome::Rejected { reason: semio_framework_replication_wire_core::read_str(bytes, pos)?, messages: semio_framework_replication_wire_core::read_bytes(bytes, pos)? }),
        other => Err(malformed("wire apply-outcome tag", *pos as u64, &format!("unknown tag {other:#x}")).await),
    }
}

async fn encode_ack_stage(stage: &AckStage, out: &mut Vec<u8>) {
    match stage {
        AckStage::Received => out.push(0),
        AckStage::Persisted => out.push(1),
        AckStage::Applied { outcome } => {
            out.push(2);
            encode_apply_outcome(outcome, out).await;
        }
    }
}

async fn decode_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<AckStage, semio_framework_replication_error::ProtocolError> {
    let tag = match bytes.get(*pos) {
        Some(b) => *b,
        None => return Err(malformed("wire ack-stage tag", *pos as u64, "truncated").await),
    };
    *pos += 1;
    match tag {
        0 => Ok(AckStage::Received),
        1 => Ok(AckStage::Persisted),
        2 => Ok(AckStage::Applied { outcome: Box::new(decode_apply_outcome(bytes, pos).await?) }),
        other => Err(malformed("wire ack-stage tag", *pos as u64, &format!("unknown tag {other:#x}")).await),
    }
}

pub(super) async fn write_vec_ack_stage(out: &mut Vec<u8>, values: &[AckStage]) {
    semio_framework_replication_wire_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_ack_stage(value, out).await;
    }
}

pub(super) async fn read_vec_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<Vec<AckStage>, semio_framework_replication_error::ProtocolError> {
    let count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(decode_ack_stage(bytes, pos).await?);
    }
    Ok(out)
}
//#endregion 🔖️NestedEnums
//#endregion 🔖️Codec
