//! 🔌️ Frame codec: the client and server frame encoders and decoders and the snapshot chunk readers. See [`super`].

use super::*;

//#region 🔖️Codec
/// 📤️ Encodes one `ClientFrame` on the given `Lane`: `lane u8 | tag u8 | fields`.
pub async fn encode_client_frame(frame: &ClientFrame, lane: Lane) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(lane.to_byte().await);
    match frame {
        ClientFrame::SocketHelloV1 { wire_version, protocol_version, schema, pack_schema_hash, resume_token, frontier } => {
            out.push(7);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *wire_version as u64);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *protocol_version as u64);
            semio_framework_replication_wire_core::write_str(&mut out, schema);
            semio_framework_replication_wire_core::write_hash32(&mut out, pack_schema_hash);
            write_opt_str(&mut out, resume_token).await;
            write_opt_frontier(&mut out, frontier).await;
        }
        ClientFrame::Commands { batch_id, envelopes } => {
            out.push(1);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *batch_id);
            write_vec_envelope(&mut out, envelopes).await;
        }
        ClientFrame::FrontierAdvertise { frontier } => {
            out.push(2);
            semio_framework_replication_causal::binary::encode_frontier(frontier, &mut out);
        }
        ClientFrame::PreviewPublish { key, seq, payload } => {
            out.push(3);
            semio_framework_replication_wire_core::write_str(&mut out, key);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *seq);
            semio_framework_replication_wire_core::write_bytes(&mut out, payload);
        }
        ClientFrame::Presence { peer } => {
            out.push(4);
            semio_framework_replication_wire_core::write_bytes(&mut out, peer);
        }
        ClientFrame::CreditGrant { n } => {
            out.push(5);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *n as u64);
        }
        ClientFrame::Bye => out.push(6),
    }
    out
}

/// 📥️ Decodes one `ClientFrame`, returning the `Lane` it was tagged with.
pub async fn decode_client_frame(bytes: &[u8]) -> Result<(Lane, ClientFrame), semio_framework_replication_error::ProtocolError> {
    let lane_byte = match bytes.first() {
        Some(b) => *b,
        None => return Err(malformed("wire frame", 0, "empty frame").await),
    };
    let lane = match Lane::from_byte(lane_byte).await {
        Some(l) => l,
        None => return Err(malformed("wire frame lane byte", 0, &format!("unknown lane {lane_byte:#x}")).await),
    };
    let mut pos = 1usize;
    let tag = match bytes.get(pos) {
        Some(b) => *b,
        None => return Err(malformed("wire client-frame tag", pos as u64, "truncated").await),
    };
    pos += 1;
    let frame = match tag {
        1 => ClientFrame::Commands { batch_id: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos).await? },
        2 => ClientFrame::FrontierAdvertise { frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)? },
        3 => ClientFrame::PreviewPublish { key: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?, seq: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?, payload: semio_framework_replication_wire_core::read_bytes(bytes, &mut pos)? },
        4 => ClientFrame::Presence { peer: semio_framework_replication_wire_core::read_bytes(bytes, &mut pos)? },
        5 => ClientFrame::CreditGrant { n: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32 },
        6 => ClientFrame::Bye,
        7 => ClientFrame::SocketHelloV1 {
            wire_version: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32,
            protocol_version: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32,
            schema: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?,
            pack_schema_hash: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?,
            resume_token: read_opt_str(bytes, &mut pos).await?,
            frontier: read_opt_frontier(bytes, &mut pos).await?,
        },
        other => return Err(malformed("wire client-frame tag", pos as u64, &format!("unknown tag {other:#x}")).await),
    };
    Ok((lane, frame))
}

/// 📤️ Encodes one `ServerFrame` on the given `Lane`: `lane u8 | tag u8 | fields`.
pub async fn encode_server_frame(frame: &ServerFrame, lane: Lane) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(lane.to_byte().await);
    match frame {
        ServerFrame::Welcome { session_id, resume_token, server_frontier, bootstrap } => {
            out.push(0);
            semio_framework_replication_wire_core::write_str(&mut out, session_id);
            semio_framework_replication_wire_core::write_str(&mut out, resume_token);
            semio_framework_replication_causal::binary::encode_frontier(server_frontier, &mut out);
            encode_bootstrap(bootstrap, &mut out).await;
        }
        ServerFrame::SnapshotChunk { seq, bytes } => {
            out.push(1);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *seq as u64);
            semio_framework_replication_wire_core::write_bytes(&mut out, bytes.as_slice());
        }
        ServerFrame::SnapshotDone { seq_count } => {
            out.push(2);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *seq_count as u64);
        }
        ServerFrame::Commands { envelopes, origin, frontier } => {
            out.push(3);
            write_vec_envelope(&mut out, envelopes).await;
            semio_framework_replication_wire_core::write_str(&mut out, &origin.0);
            semio_framework_replication_causal::binary::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Ack { batch_id, stages, frontier } => {
            out.push(4);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *batch_id);
            write_vec_ack_stage(&mut out, stages).await;
            semio_framework_replication_causal::binary::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Preview { actor, key, seq, payload } => {
            out.push(5);
            semio_framework_replication_wire_core::write_str(&mut out, &actor.0);
            semio_framework_replication_wire_core::write_str(&mut out, key);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *seq);
            semio_framework_replication_wire_core::write_bytes(&mut out, payload);
        }
        ServerFrame::Presence { peers } => {
            out.push(6);
            write_vec_bytes(&mut out, peers).await;
        }
        ServerFrame::CreditGrant { n } => {
            out.push(7);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *n as u64);
        }
        ServerFrame::Error { code, message } => {
            out.push(8);
            semio_framework_replication_wire_core::write_str(&mut out, code);
            semio_framework_replication_wire_core::write_str(&mut out, message);
        }
        ServerFrame::Session { actor, color } => {
            out.push(9);
            semio_framework_replication_wire_core::write_str(&mut out, actor);
            out.push(*color);
        }
        ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index, bytes } => {
            out.push(10);
            semio_framework_replication_wire_core::write_hash32(&mut out, descriptor_hash);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *index as u64);
            semio_framework_replication_wire_core::write_bytes(&mut out, bytes.as_slice());
        }
        ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count } => {
            out.push(11);
            semio_framework_replication_wire_core::write_hash32(&mut out, descriptor_hash);
            semio_framework_replication_wire_core::write_varint_u64(&mut out, *chunk_count as u64);
        }
        ServerFrame::RebootstrapRequired { control } => {
            out.push(12);
            semio_framework_replication_wire_core::write_str(&mut out, &control.space_id);
            semio_framework_replication_wire_core::write_str(&mut out, &control.document_id);
            semio_framework_replication_wire_core::write_hash32(&mut out, &control.checkpoint_id);
            semio_framework_replication_wire_core::write_hash32(&mut out, &control.descriptor_hash);
            semio_framework_replication_causal::binary::encode_frontier(&control.baseline_frontier, &mut out);
        }
    }
    out
}

fn read_snapshot_chunk_bytes(bytes: &[u8], pos: &mut usize) -> Result<SnapshotChunkBytes, semio_framework_replication_error::ProtocolError> {
    let len = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    if len > SNAPSHOT_CHUNK_BACKING_BYTES as u64 {
        return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("snapshot chunk fixed backing"));
    }
    let len = len as usize;
    let end = pos.checked_add(len).ok_or(semio_framework_replication_error::ProtocolError::Malformed { what: "wire snapshot chunk", offset: *pos as u64, detail: "length overflow".to_string() })?;
    let source = bytes.get(*pos..end).ok_or(semio_framework_replication_error::ProtocolError::Malformed { what: "wire snapshot chunk", offset: *pos as u64, detail: "truncated".to_string() })?;
    let owner = SnapshotChunkBytes::try_from_slice(source).ok_or(semio_framework_replication_error::ProtocolError::LimitExceeded("snapshot chunk fixed backing"))?;
    *pos = end;
    Ok(owner)
}

fn read_artifact_bootstrap_chunk_bytes(bytes: &[u8], pos: &mut usize) -> Result<ArtifactBootstrapChunkBytes, semio_framework_replication_error::ProtocolError> {
    let bytes = read_snapshot_chunk_bytes(bytes, pos)?;
    if bytes.is_empty() {
        return Err(artifact_bootstrap_error("chunk bytes must be nonempty"));
    }
    Ok(bytes)
}

/// 📥️ Decodes one `ServerFrame`, returning the `Lane` it was tagged with.
pub async fn decode_server_frame(bytes: &[u8]) -> Result<(Lane, ServerFrame), semio_framework_replication_error::ProtocolError> {
    let lane_byte = match bytes.first() {
        Some(b) => *b,
        None => return Err(malformed("wire frame", 0, "empty frame").await),
    };
    let lane = match Lane::from_byte(lane_byte).await {
        Some(l) => l,
        None => return Err(malformed("wire frame lane byte", 0, &format!("unknown lane {lane_byte:#x}")).await),
    };
    let mut pos = 1usize;
    let tag = match bytes.get(pos) {
        Some(b) => *b,
        None => return Err(malformed("wire server-frame tag", pos as u64, "truncated").await),
    };
    pos += 1;
    let frame = match tag {
        0 => ServerFrame::Welcome {
            session_id: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?,
            resume_token: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?,
            server_frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)?,
            bootstrap: decode_bootstrap(bytes, &mut pos).await?,
        },
        1 => ServerFrame::SnapshotChunk { seq: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32, bytes: read_snapshot_chunk_bytes(bytes, &mut pos)? },
        2 => ServerFrame::SnapshotDone { seq_count: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32 },
        3 => ServerFrame::Commands { envelopes: read_vec_envelope(bytes, &mut pos).await?, origin: semio_framework_replication_ids::ActorId(semio_framework_replication_wire_core::read_str(bytes, &mut pos)?.into()), frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)? },
        4 => ServerFrame::Ack { batch_id: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?, stages: read_vec_ack_stage(bytes, &mut pos).await?, frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)? },
        5 => ServerFrame::Preview { actor: semio_framework_replication_ids::ActorId(semio_framework_replication_wire_core::read_str(bytes, &mut pos)?.into()), key: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?, seq: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?, payload: semio_framework_replication_wire_core::read_bytes(bytes, &mut pos)? },
        6 => ServerFrame::Presence { peers: read_vec_bytes(bytes, &mut pos).await? },
        7 => ServerFrame::CreditGrant { n: semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)? as u32 },
        8 => ServerFrame::Error { code: semio_framework_replication_wire_core::read_str(bytes, &mut pos)?, message: semio_framework_replication_wire_core::read_str(bytes, &mut pos)? },
        9 => {
            let actor = semio_framework_replication_wire_core::read_str(bytes, &mut pos)?;
            let color = semio_framework_replication_wire_core::read_u8(bytes, &mut pos)?;
            ServerFrame::Session { actor, color }
        }
        10 => ServerFrame::ArtifactBootstrapChunk { descriptor_hash: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?, index: u32::try_from(semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?).map_err(|_| artifact_bootstrap_error("chunk index exceeds u32"))?, bytes: read_artifact_bootstrap_chunk_bytes(bytes, &mut pos)? },
        11 => ServerFrame::ArtifactBootstrapDone { descriptor_hash: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?, chunk_count: u32::try_from(semio_framework_replication_wire_core::read_varint_u64(bytes, &mut pos)?).map_err(|_| artifact_bootstrap_error("done chunk count exceeds u32"))? },
        12 => {
            let control = RebootstrapRequired {
                space_id: read_artifact_bootstrap_string(bytes, &mut pos, "rebootstrap space")?,
                document_id: read_artifact_bootstrap_string(bytes, &mut pos, "rebootstrap document")?,
                checkpoint_id: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?,
                descriptor_hash: semio_framework_replication_wire_core::read_hash32(bytes, &mut pos)?,
                baseline_frontier: semio_framework_replication_causal::binary::decode_frontier(bytes, &mut pos)?,
            };
            validate_rebootstrap_required(&control)?;
            ServerFrame::RebootstrapRequired { control }
        }
        other => return Err(malformed("wire server-frame tag", pos as u64, &format!("unknown tag {other:#x}")).await),
    };
    Ok((lane, frame))
}
//#endregion 🔖️Codec
