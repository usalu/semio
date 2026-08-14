//! 🎞️ Protocol semio_hub wire frames: the lane-tagged `ClientFrame`/`ServerFrame` envelopes a
//! browser/native sync client exchanges with the collaboration semio_hub, plus their binary codec. Frozen
//! contract: `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment`
//! §`protocol_wire`.
//!
//! 🎯️ W5: the byte encoding is now a fully hand-rolled binary layout — `lane: u8` followed by
//! `frame tag: u8` (the frame enum's variant declaration order) and its fields in declaration
//! order, with no body-length prefix (one frame per WS message) and no per-field tags. This
//! matches `crate::os_spr::wire::🔖️WireCodec`'s convention (also used by `crate::os_spr::causal::🔖️EnvelopeCodec`
//! and `crate::os_dsl::op_rt`). `ArtifactDiff`/`InverseMutation` payloads are opaque `Vec<u8>` (never
//! `serde_json::Value`). `ClientFrame::Presence`/`ServerFrame::Presence` carry opaque presence
//! payload bytes (`peer: Vec<u8>` / `peers: Vec<Vec<u8>>`) — this crate has no dependency on
//! `framework_core` (where the concrete `PresencePeer` type and its binary codec live), so the
//! frame only ever moves the already-encoded blob a caller supplies. `protocol_core` supplies the
//! primitive codec (`write_varint_u64`/`write_str`/`write_bytes`/`write_hash32`/`write_bool` and
//! their `read_*` twins); this crate adds only the option/vec combinators and the frame/nested-enum
//! tag dispatch below.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
//#region 🔖️Lane
/// @emoji 🛣️ Which logical channel a wire frame travels on: `Command` for causally-ordered,
/// durable operation batches; `Preview` for ephemeral, best-effort UI-state broadcast.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    Command = 0,
    Preview = 1,
}

impl Lane {
    fn to_byte(self) -> u8 {
        self as u8
    }

    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Lane::Command),
            1 => Some(Lane::Preview),
            _ => None,
        }
    }
}
//#endregion 🔖️Lane

//#region 🔖️ClientFrame
/// @emoji 📨️ One frame a client sends to the semio_hub.
#[derive(Clone, Debug, PartialEq)]
pub enum ClientFrame {
    Hello { wire_version: u32, protocol_version: u32, schema: String, pack_schema_hash: [u8; 32], actor: crate::os_spr::ids::ActorId, token: Option<String>, resume_token: Option<String>, frontier: Option<crate::os_spr::causal::FrontierSummary> },
    Commands { batch_id: u64, envelopes: Vec<crate::os_spr::causal::MutationEnvelope> },
    FrontierAdvertise { frontier: crate::os_spr::causal::FrontierSummary },
    PreviewPublish { key: String, seq: u64, payload: Vec<u8> },
    Presence { peer: Vec<u8> },
    CreditGrant { n: u32 },
    Bye,
}
//#endregion 🔖️ClientFrame

//#region 🔖️ServerFrame
/// @emoji 🚀️ How a `ServerFrame::Welcome` seeds a freshly (re)connected client's local state.
#[derive(Clone, Debug, PartialEq)]
pub enum Bootstrap {
    None,
    Snapshot { pack_hash: [u8; 32], inline: Option<Vec<u8>> },
    Tail,
}

/// @emoji ⚖️ How the semio_hub resolved one submitted operation against concurrent history.
#[derive(Clone, Debug, PartialEq)]
pub enum ApplyOutcome {
    Accepted,
    // 🔒️ Boxed: MutationEnvelope is far larger than the other variants, and clippy's
    // large_enum_variant lint (a real per-instance cost, not just style) applies at -D warnings.
    Transformed { envelope: Box<crate::os_spr::causal::MutationEnvelope> },
    Rejected { reason: String },
}

/// @emoji 🪜️ One stage of a submitted batch's lifecycle, from `Received` to `Applied`.
#[derive(Clone, Debug, PartialEq)]
pub enum AckStage {
    Received,
    Persisted,
    // 🔒️ Boxed for the same reason as ApplyOutcome::Transformed above.
    Applied { outcome: Box<ApplyOutcome> },
}

/// @emoji 📬️ One frame the semio_hub sends to a client.
#[derive(Clone, Debug, PartialEq)]
pub enum ServerFrame {
    Welcome { session_id: String, resume_token: String, server_frontier: crate::os_spr::causal::FrontierSummary, bootstrap: Bootstrap },
    SnapshotChunk { seq: u32, bytes: Vec<u8> },
    SnapshotDone { seq_count: u32 },
    Commands { envelopes: Vec<crate::os_spr::causal::MutationEnvelope>, origin: crate::os_spr::ids::ActorId, frontier: crate::os_spr::causal::FrontierSummary },
    Ack { batch_id: u64, stages: Vec<AckStage>, frontier: crate::os_spr::causal::FrontierSummary },
    Preview { actor: crate::os_spr::ids::ActorId, key: String, seq: u64, payload: Vec<u8> },
    Presence { peers: Vec<Vec<u8>> },
    CreditGrant { n: u32 },
    Error { code: String, message: String },
}
//#endregion 🔖️ServerFrame

//#region 🔖️Codec
// Hand-rolled binary frame encode/decode: `lane: u8 | frame tag: u8 | fields...` — see the
// module-level docstring. `crate::os_spr::wire::🔖️WireCodec` supplies the primitives; this region adds
// the option/vec combinators the frame shapes need plus the tag-dispatch match arms.

fn malformed(what: &'static str, offset: u64, detail: &str) -> crate::os_spr::ProtocolError {
    crate::os_spr::ProtocolError::Malformed { what, offset, detail: detail.to_string() }
}

//#region 🔖️Combinators
fn write_opt_str(out: &mut Vec<u8>, value: &Option<String>) {
    crate::os_spr::write_bool(out, value.is_some());
    if let Some(s) = value {
        crate::os_spr::write_str(out, s);
    }
}

fn read_opt_str(bytes: &[u8], pos: &mut usize) -> Result<Option<String>, crate::os_spr::ProtocolError> {
    if crate::os_spr::read_bool(bytes, pos)? {
        Ok(Some(crate::os_spr::read_str(bytes, pos)?))
    } else {
        Ok(None)
    }
}

fn write_opt_bytes(out: &mut Vec<u8>, value: &Option<Vec<u8>>) {
    crate::os_spr::write_bool(out, value.is_some());
    if let Some(b) = value {
        crate::os_spr::write_bytes(out, b);
    }
}

fn read_opt_bytes(bytes: &[u8], pos: &mut usize) -> Result<Option<Vec<u8>>, crate::os_spr::ProtocolError> {
    if crate::os_spr::read_bool(bytes, pos)? {
        Ok(Some(crate::os_spr::read_bytes(bytes, pos)?))
    } else {
        Ok(None)
    }
}

fn write_opt_frontier(out: &mut Vec<u8>, value: &Option<crate::os_spr::causal::FrontierSummary>) {
    crate::os_spr::write_bool(out, value.is_some());
    if let Some(f) = value {
        crate::os_spr::causal::encode_frontier(f, out);
    }
}

fn read_opt_frontier(bytes: &[u8], pos: &mut usize) -> Result<Option<crate::os_spr::causal::FrontierSummary>, crate::os_spr::ProtocolError> {
    if crate::os_spr::read_bool(bytes, pos)? {
        Ok(Some(crate::os_spr::causal::decode_frontier(bytes, pos)?))
    } else {
        Ok(None)
    }
}

fn write_vec_bytes(out: &mut Vec<u8>, values: &[Vec<u8>]) {
    crate::os_spr::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::os_spr::write_bytes(out, value);
    }
}

fn read_vec_bytes(bytes: &[u8], pos: &mut usize) -> Result<Vec<Vec<u8>>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| crate::os_spr::read_bytes(bytes, pos)).collect()
}

fn write_vec_envelope(out: &mut Vec<u8>, values: &[crate::os_spr::causal::MutationEnvelope]) {
    crate::os_spr::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::os_spr::causal::encode_envelope(value, out);
    }
}

fn read_vec_envelope(bytes: &[u8], pos: &mut usize) -> Result<Vec<crate::os_spr::causal::MutationEnvelope>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| crate::os_spr::causal::decode_envelope(bytes, pos)).collect()
}
//#endregion 🔖️Combinators

//#region 🔖️NestedEnums
fn encode_bootstrap(bootstrap: &Bootstrap, out: &mut Vec<u8>) {
    match bootstrap {
        Bootstrap::None => out.push(0),
        Bootstrap::Snapshot { pack_hash, inline } => {
            out.push(1);
            crate::os_spr::write_hash32(out, pack_hash);
            write_opt_bytes(out, inline);
        }
        Bootstrap::Tail => out.push(2),
    }
}

fn decode_bootstrap(bytes: &[u8], pos: &mut usize) -> Result<Bootstrap, crate::os_spr::ProtocolError> {
    let tag = *bytes.get(*pos).ok_or_else(|| malformed("wire bootstrap tag", *pos as u64, "truncated"))?;
    *pos += 1;
    match tag {
        0 => Ok(Bootstrap::None),
        1 => {
            let pack_hash = crate::os_spr::read_hash32(bytes, pos)?;
            let inline = read_opt_bytes(bytes, pos)?;
            Ok(Bootstrap::Snapshot { pack_hash, inline })
        }
        2 => Ok(Bootstrap::Tail),
        other => Err(malformed("wire bootstrap tag", *pos as u64, &format!("unknown tag {other:#x}"))),
    }
}

fn encode_apply_outcome(outcome: &ApplyOutcome, out: &mut Vec<u8>) {
    match outcome {
        ApplyOutcome::Accepted => out.push(0),
        ApplyOutcome::Transformed { envelope } => {
            out.push(1);
            crate::os_spr::causal::encode_envelope(envelope, out);
        }
        ApplyOutcome::Rejected { reason } => {
            out.push(2);
            crate::os_spr::write_str(out, reason);
        }
    }
}

fn decode_apply_outcome(bytes: &[u8], pos: &mut usize) -> Result<ApplyOutcome, crate::os_spr::ProtocolError> {
    let tag = *bytes.get(*pos).ok_or_else(|| malformed("wire apply-outcome tag", *pos as u64, "truncated"))?;
    *pos += 1;
    match tag {
        0 => Ok(ApplyOutcome::Accepted),
        1 => Ok(ApplyOutcome::Transformed { envelope: Box::new(crate::os_spr::causal::decode_envelope(bytes, pos)?) }),
        2 => Ok(ApplyOutcome::Rejected { reason: crate::os_spr::read_str(bytes, pos)? }),
        other => Err(malformed("wire apply-outcome tag", *pos as u64, &format!("unknown tag {other:#x}"))),
    }
}

fn encode_ack_stage(stage: &AckStage, out: &mut Vec<u8>) {
    match stage {
        AckStage::Received => out.push(0),
        AckStage::Persisted => out.push(1),
        AckStage::Applied { outcome } => {
            out.push(2);
            encode_apply_outcome(outcome, out);
        }
    }
}

fn decode_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<AckStage, crate::os_spr::ProtocolError> {
    let tag = *bytes.get(*pos).ok_or_else(|| malformed("wire ack-stage tag", *pos as u64, "truncated"))?;
    *pos += 1;
    match tag {
        0 => Ok(AckStage::Received),
        1 => Ok(AckStage::Persisted),
        2 => Ok(AckStage::Applied { outcome: Box::new(decode_apply_outcome(bytes, pos)?) }),
        other => Err(malformed("wire ack-stage tag", *pos as u64, &format!("unknown tag {other:#x}"))),
    }
}

fn write_vec_ack_stage(out: &mut Vec<u8>, values: &[AckStage]) {
    crate::os_spr::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_ack_stage(value, out);
    }
}

fn read_vec_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<Vec<AckStage>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| decode_ack_stage(bytes, pos)).collect()
}
//#endregion 🔖️NestedEnums

/// @emoji 📤️ Encodes one `ClientFrame` on the given `Lane`: `lane u8 | tag u8 | fields`.
pub fn encode_client_frame(frame: &ClientFrame, lane: Lane) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(lane.to_byte());
    match frame {
        ClientFrame::Hello { wire_version, protocol_version, schema, pack_schema_hash, actor, token, resume_token, frontier } => {
            out.push(0);
            crate::os_spr::write_varint_u64(&mut out, *wire_version as u64);
            crate::os_spr::write_varint_u64(&mut out, *protocol_version as u64);
            crate::os_spr::write_str(&mut out, schema);
            crate::os_spr::write_hash32(&mut out, pack_schema_hash);
            crate::os_spr::write_str(&mut out, &actor.0);
            write_opt_str(&mut out, token);
            write_opt_str(&mut out, resume_token);
            write_opt_frontier(&mut out, frontier);
        }
        ClientFrame::Commands { batch_id, envelopes } => {
            out.push(1);
            crate::os_spr::write_varint_u64(&mut out, *batch_id);
            write_vec_envelope(&mut out, envelopes);
        }
        ClientFrame::FrontierAdvertise { frontier } => {
            out.push(2);
            crate::os_spr::causal::encode_frontier(frontier, &mut out);
        }
        ClientFrame::PreviewPublish { key, seq, payload } => {
            out.push(3);
            crate::os_spr::write_str(&mut out, key);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, payload);
        }
        ClientFrame::Presence { peer } => {
            out.push(4);
            crate::os_spr::write_bytes(&mut out, peer);
        }
        ClientFrame::CreditGrant { n } => {
            out.push(5);
            crate::os_spr::write_varint_u64(&mut out, *n as u64);
        }
        ClientFrame::Bye => out.push(6),
    }
    out
}

/// @emoji 📥️ Decodes one `ClientFrame`, returning the `Lane` it was tagged with.
pub fn decode_client_frame(bytes: &[u8]) -> Result<(Lane, ClientFrame), crate::os_spr::ProtocolError> {
    let lane_byte = *bytes.first().ok_or_else(|| malformed("wire frame", 0, "empty frame"))?;
    let lane = Lane::from_byte(lane_byte).ok_or_else(|| malformed("wire frame lane byte", 0, &format!("unknown lane {lane_byte:#x}")))?;
    let mut pos = 1usize;
    let tag = *bytes.get(pos).ok_or_else(|| malformed("wire client-frame tag", pos as u64, "truncated"))?;
    pos += 1;
    let frame = match tag {
        0 => ClientFrame::Hello {
            wire_version: crate::os_spr::read_varint_u64(bytes, &mut pos)? as u32,
            protocol_version: crate::os_spr::read_varint_u64(bytes, &mut pos)? as u32,
            schema: crate::os_spr::read_str(bytes, &mut pos)?,
            pack_schema_hash: crate::os_spr::read_hash32(bytes, &mut pos)?,
            actor: crate::os_spr::ids::ActorId(crate::os_spr::read_str(bytes, &mut pos)?),
            token: read_opt_str(bytes, &mut pos)?,
            resume_token: read_opt_str(bytes, &mut pos)?,
            frontier: read_opt_frontier(bytes, &mut pos)?,
        },
        1 => ClientFrame::Commands { batch_id: crate::os_spr::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos)? },
        2 => ClientFrame::FrontierAdvertise { frontier: crate::os_spr::causal::decode_frontier(bytes, &mut pos)? },
        3 => ClientFrame::PreviewPublish { key: crate::os_spr::read_str(bytes, &mut pos)?, seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, payload: crate::os_spr::read_bytes(bytes, &mut pos)? },
        4 => ClientFrame::Presence { peer: crate::os_spr::read_bytes(bytes, &mut pos)? },
        5 => ClientFrame::CreditGrant { n: crate::os_spr::read_varint_u64(bytes, &mut pos)? as u32 },
        6 => ClientFrame::Bye,
        other => return Err(malformed("wire client-frame tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok((lane, frame))
}

/// @emoji 📤️ Encodes one `ServerFrame` on the given `Lane`: `lane u8 | tag u8 | fields`.
pub fn encode_server_frame(frame: &ServerFrame, lane: Lane) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(lane.to_byte());
    match frame {
        ServerFrame::Welcome { session_id, resume_token, server_frontier, bootstrap } => {
            out.push(0);
            crate::os_spr::write_str(&mut out, session_id);
            crate::os_spr::write_str(&mut out, resume_token);
            crate::os_spr::causal::encode_frontier(server_frontier, &mut out);
            encode_bootstrap(bootstrap, &mut out);
        }
        ServerFrame::SnapshotChunk { seq, bytes } => {
            out.push(1);
            crate::os_spr::write_varint_u64(&mut out, *seq as u64);
            crate::os_spr::write_bytes(&mut out, bytes);
        }
        ServerFrame::SnapshotDone { seq_count } => {
            out.push(2);
            crate::os_spr::write_varint_u64(&mut out, *seq_count as u64);
        }
        ServerFrame::Commands { envelopes, origin, frontier } => {
            out.push(3);
            write_vec_envelope(&mut out, envelopes);
            crate::os_spr::write_str(&mut out, &origin.0);
            crate::os_spr::causal::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Ack { batch_id, stages, frontier } => {
            out.push(4);
            crate::os_spr::write_varint_u64(&mut out, *batch_id);
            write_vec_ack_stage(&mut out, stages);
            crate::os_spr::causal::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Preview { actor, key, seq, payload } => {
            out.push(5);
            crate::os_spr::write_str(&mut out, &actor.0);
            crate::os_spr::write_str(&mut out, key);
            crate::os_spr::write_varint_u64(&mut out, *seq);
            crate::os_spr::write_bytes(&mut out, payload);
        }
        ServerFrame::Presence { peers } => {
            out.push(6);
            write_vec_bytes(&mut out, peers);
        }
        ServerFrame::CreditGrant { n } => {
            out.push(7);
            crate::os_spr::write_varint_u64(&mut out, *n as u64);
        }
        ServerFrame::Error { code, message } => {
            out.push(8);
            crate::os_spr::write_str(&mut out, code);
            crate::os_spr::write_str(&mut out, message);
        }
    }
    out
}

/// @emoji 📥️ Decodes one `ServerFrame`, returning the `Lane` it was tagged with.
pub fn decode_server_frame(bytes: &[u8]) -> Result<(Lane, ServerFrame), crate::os_spr::ProtocolError> {
    let lane_byte = *bytes.first().ok_or_else(|| malformed("wire frame", 0, "empty frame"))?;
    let lane = Lane::from_byte(lane_byte).ok_or_else(|| malformed("wire frame lane byte", 0, &format!("unknown lane {lane_byte:#x}")))?;
    let mut pos = 1usize;
    let tag = *bytes.get(pos).ok_or_else(|| malformed("wire server-frame tag", pos as u64, "truncated"))?;
    pos += 1;
    let frame = match tag {
        0 => ServerFrame::Welcome {
            session_id: crate::os_spr::read_str(bytes, &mut pos)?,
            resume_token: crate::os_spr::read_str(bytes, &mut pos)?,
            server_frontier: crate::os_spr::causal::decode_frontier(bytes, &mut pos)?,
            bootstrap: decode_bootstrap(bytes, &mut pos)?,
        },
        1 => ServerFrame::SnapshotChunk { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? as u32, bytes: crate::os_spr::read_bytes(bytes, &mut pos)? },
        2 => ServerFrame::SnapshotDone { seq_count: crate::os_spr::read_varint_u64(bytes, &mut pos)? as u32 },
        3 => ServerFrame::Commands { envelopes: read_vec_envelope(bytes, &mut pos)?, origin: crate::os_spr::ids::ActorId(crate::os_spr::read_str(bytes, &mut pos)?), frontier: crate::os_spr::causal::decode_frontier(bytes, &mut pos)? },
        4 => ServerFrame::Ack { batch_id: crate::os_spr::read_varint_u64(bytes, &mut pos)?, stages: read_vec_ack_stage(bytes, &mut pos)?, frontier: crate::os_spr::causal::decode_frontier(bytes, &mut pos)? },
        5 => ServerFrame::Preview {
            actor: crate::os_spr::ids::ActorId(crate::os_spr::read_str(bytes, &mut pos)?),
            key: crate::os_spr::read_str(bytes, &mut pos)?,
            seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            payload: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        6 => ServerFrame::Presence { peers: read_vec_bytes(bytes, &mut pos)? },
        7 => ServerFrame::CreditGrant { n: crate::os_spr::read_varint_u64(bytes, &mut pos)? as u32 },
        8 => ServerFrame::Error { code: crate::os_spr::read_str(bytes, &mut pos)?, message: crate::os_spr::read_str(bytes, &mut pos)? },
        other => return Err(malformed("wire server-frame tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok((lane, frame))
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    fn sample_envelope(id: &str) -> crate::os_spr::causal::MutationEnvelope {
        crate::os_spr::causal::MutationEnvelope {
            mutation_id: crate::os_spr::ids::MutationId(id.to_string()),
            document_id: crate::os_spr::ids::ArtifactId("document-1".to_string()),
            actor: crate::os_spr::ids::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: crate::os_spr::causal::ArtifactDiff { schema: crate::os_spr::ids::SchemaId("diff.v1".to_string()), payload: format!("value:{id}").into_bytes() },
            inverse: crate::os_spr::causal::InverseMutation { schema: crate::os_spr::ids::SchemaId("diff.v1".to_string()), payload: Vec::new() },
            timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 0),
        }
    }

    fn sample_frontier() -> crate::os_spr::causal::FrontierSummary {
        crate::os_spr::causal::FrontierSummary { document_id: crate::os_spr::ids::ArtifactId("document-1".to_string()), head_edit_ordinal: 5, head_edit_id: "edit-5".to_string(), last_commit_seq: 2, chain_hash: [7u8; 32] }
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️Lane
    #[test]
    fn lane_byte_round_trips() {
        assert_eq!(Lane::from_byte(Lane::Command.to_byte()), Some(Lane::Command));
        assert_eq!(Lane::from_byte(Lane::Preview.to_byte()), Some(Lane::Preview));
        assert_eq!(Lane::from_byte(2), None);
    }
    //#endregion 🔖️Lane

    //#region 🔖️ClientFrame
    fn assert_client_round_trips(frame: &ClientFrame, lane: Lane) {
        let bytes = encode_client_frame(frame, lane);
        let (decoded_lane, decoded_frame) = decode_client_frame(&bytes).expect("decode must succeed");
        assert_eq!(decoded_lane, lane);
        assert_eq!(&decoded_frame, frame);
    }

    #[test]
    fn client_frame_hello_round_trips() {
        assert_client_round_trips(
            &ClientFrame::Hello {
                wire_version: 1,
                protocol_version: 1,
                schema: "schema.v1".to_string(),
                pack_schema_hash: [1u8; 32],
                actor: crate::os_spr::ids::ActorId("actor-1".to_string()),
                token: Some("token".to_string()),
                resume_token: None,
                frontier: Some(sample_frontier()),
            },
            Lane::Command,
        );
    }

    #[test]
    fn client_frame_hello_with_no_optionals_round_trips() {
        assert_client_round_trips(
            &ClientFrame::Hello { wire_version: 1, protocol_version: 1, schema: "schema.v1".to_string(), pack_schema_hash: [0u8; 32], actor: crate::os_spr::ids::ActorId("actor-2".to_string()), token: None, resume_token: None, frontier: None },
            Lane::Command,
        );
    }

    #[test]
    fn client_frame_commands_round_trips() {
        assert_client_round_trips(&ClientFrame::Commands { batch_id: 42, envelopes: vec![sample_envelope("op-1"), sample_envelope("op-2")] }, Lane::Command);
    }

    #[test]
    fn client_frame_frontier_advertise_round_trips() {
        assert_client_round_trips(&ClientFrame::FrontierAdvertise { frontier: sample_frontier() }, Lane::Command);
    }

    #[test]
    fn client_frame_preview_publish_round_trips() {
        assert_client_round_trips(&ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview);
    }

    #[test]
    fn client_frame_presence_round_trips() {
        assert_client_round_trips(&ClientFrame::Presence { peer: b"{\"cursor\":[1,2]}".to_vec() }, Lane::Preview);
    }

    #[test]
    fn client_frame_credit_grant_round_trips() {
        assert_client_round_trips(&ClientFrame::CreditGrant { n: 16 }, Lane::Command);
    }

    #[test]
    fn client_frame_bye_round_trips() {
        assert_client_round_trips(&ClientFrame::Bye, Lane::Command);
    }
    //#endregion 🔖️ClientFrame

    //#region 🔖️ServerFrame
    fn assert_server_round_trips(frame: &ServerFrame, lane: Lane) {
        let bytes = encode_server_frame(frame, lane);
        let (decoded_lane, decoded_frame) = decode_server_frame(&bytes).expect("decode must succeed");
        assert_eq!(decoded_lane, lane);
        assert_eq!(&decoded_frame, frame);
    }

    #[test]
    fn server_frame_welcome_round_trips_for_every_bootstrap_variant() {
        for bootstrap in [Bootstrap::None, Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: Some(vec![9, 9]) }, Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: None }, Bootstrap::Tail] {
            assert_server_round_trips(&ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: sample_frontier(), bootstrap }, Lane::Command);
        }
    }

    #[test]
    fn server_frame_snapshot_chunk_round_trips() {
        assert_server_round_trips(&ServerFrame::SnapshotChunk { seq: 0, bytes: vec![1, 2, 3, 4] }, Lane::Command);
    }

    #[test]
    fn server_frame_snapshot_done_round_trips() {
        assert_server_round_trips(&ServerFrame::SnapshotDone { seq_count: 4 }, Lane::Command);
    }

    #[test]
    fn server_frame_commands_round_trips() {
        assert_server_round_trips(&ServerFrame::Commands { envelopes: vec![sample_envelope("op-1")], origin: crate::os_spr::ids::ActorId("actor-1".to_string()), frontier: sample_frontier() }, Lane::Command);
    }

    #[test]
    fn server_frame_ack_round_trips_for_every_stage_and_apply_outcome_variant() {
        for outcome in [ApplyOutcome::Accepted, ApplyOutcome::Transformed { envelope: Box::new(sample_envelope("op-1")) }, ApplyOutcome::Rejected { reason: "conflict".to_string() }] {
            assert_server_round_trips(&ServerFrame::Ack { batch_id: 7, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(outcome) }], frontier: sample_frontier() }, Lane::Command);
        }
    }

    #[test]
    fn server_frame_preview_round_trips() {
        assert_server_round_trips(&ServerFrame::Preview { actor: crate::os_spr::ids::ActorId("actor-1".to_string()), key: "cursor".to_string(), seq: 3, payload: vec![5, 6] }, Lane::Preview);
    }

    #[test]
    fn server_frame_presence_round_trips() {
        assert_server_round_trips(&ServerFrame::Presence { peers: vec![b"{\"id\":\"a\"}".to_vec(), b"{\"id\":\"b\"}".to_vec()] }, Lane::Preview);
    }

    #[test]
    fn server_frame_credit_grant_round_trips() {
        assert_server_round_trips(&ServerFrame::CreditGrant { n: 32 }, Lane::Command);
    }

    #[test]
    fn server_frame_error_round_trips() {
        assert_server_round_trips(&ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command);
    }
    //#endregion 🔖️ServerFrame

    //#region 🔖️Codec
    #[test]
    fn decode_client_frame_rejects_empty_bytes() {
        let err = decode_client_frame(&[]).unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "wire frame", .. }));
    }

    #[test]
    fn decode_client_frame_rejects_unknown_lane_byte() {
        let err = decode_client_frame(&[2u8, 0]).unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "wire frame lane byte", .. }));
    }

    #[test]
    fn decode_client_frame_rejects_unknown_tag() {
        let bytes = vec![Lane::Command.to_byte(), 0xFF];
        let err = decode_client_frame(&bytes).unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "wire client-frame tag", .. }));
    }

    #[test]
    fn decode_server_frame_rejects_unknown_tag() {
        let bytes = vec![Lane::Command.to_byte(), 0xFF];
        let err = decode_server_frame(&bytes).unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "wire server-frame tag", .. }));
    }

    #[test]
    fn decode_client_frame_rejects_truncated_field() {
        let bytes = encode_client_frame(&ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview);
        let truncated = &bytes[..bytes.len() - 2];
        assert!(decode_client_frame(truncated).is_err());
    }

    #[test]
    fn decode_server_frame_rejects_truncated_field() {
        let bytes = encode_server_frame(&ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command);
        let truncated = &bytes[..bytes.len() - 3];
        assert!(decode_server_frame(truncated).is_err());
    }

    #[test]
    fn decode_client_frame_rejects_empty_body_after_lane() {
        let err = decode_client_frame(&[Lane::Command.to_byte()]).unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "wire client-frame tag", .. }));
    }

    #[test]
    fn different_lanes_produce_different_leading_bytes_but_same_body() {
        let command_bytes = encode_client_frame(&ClientFrame::Bye, Lane::Command);
        let preview_bytes = encode_client_frame(&ClientFrame::Bye, Lane::Preview);
        assert_eq!(command_bytes[0], 0);
        assert_eq!(preview_bytes[0], 1);
        assert_eq!(command_bytes[1..], preview_bytes[1..]);
    }
    //#endregion 🔖️Codec
}
//#endregion 🧪️Tests

//#region 🔖️Presence
// 🎯️ W6 kernel unification: `PayloadHash`/`MutationEnvelope`/`MutationDagError`/`MutationDag`/`InsertResult`
// (the local causal-sync types) and `HubClientFrame`/`HubServerFrame` (the local semio_hub wire frames)
// are DELETED — `store`/`store_sync` (their only consumers outside this crate) now speak
// `protocol::{MutationEnvelope, MutationDag, MutationDagError, InsertResult}`/`protocol::{ClientFrame,
// ServerFrame}` directly (W5 already made these real binary types; this wave just stops
// duplicating them here). `PresencePoint`/`PresenceViewport`/`PresencePeer` below are NOT
// duplicates of anything in `protocol` — no equivalent exists there — so they stay, kept in their
// own region since the `🔖️HubProtocol` name they used to share with the now-deleted frame enums no
// longer fits.
/// @emoji 📍️ A live cursor position in document space, broadcast as part of a peer's presence frame.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresencePoint {
    pub x: f64,
    pub y: f64,
}

/// @emoji 🖼️ A peer's visible canvas rectangle (pan + zoom), so remote cursors/ghosts can be rendered
/// scaled relative to what each peer is actually looking at.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceViewport {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

//#region 🔖️PresencePackSerde
/// 🔐️ Base64 (std) codec for `PresencePeer.presence_pack` so `presence_peers_json` emits `presencePack`
/// as a string rather than a JSON byte array.
mod presence_pack_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error> {
        match value {
            None => serializer.serialize_none(),
            Some(bytes) => {
                let encoded = ::base64::Engine::encode(&::base64::engine::general_purpose::STANDARD, bytes);
                serializer.serialize_some(&encoded)
            }
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error> {
        let encoded: Option<String> = Option::deserialize(deserializer)?;
        match encoded {
            None => Ok(None),
            Some(s) => {
                let bytes = ::base64::Engine::decode(&::base64::engine::general_purpose::STANDARD, s.as_bytes()).map_err(serde::de::Error::custom)?;
                Ok(Some(bytes))
            }
        }
    }
}
//#endregion 🔖️PresencePackSerde

/// @emoji 📡️ Presence roster entry broadcast to every peer connected to a document.
///
/// `presence_pack` carries the app's typed `ArtifactApp::Presence` encoded through `ArtifactPack`.
/// When serialised for `ViewModel.presence_peers_json`, that pack is base64-encoded under the
/// camelCase key `presencePack` (this layer has no app-specific `ArtifactPack` decoder, so the
/// renderer JSON contract keeps the opaque pack rather than a decoded `presence` object).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresencePeer {
    pub actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// @emoji 👥️ App-typed presence encoded as `ArtifactPack` bytes (flag bit 1 on the wire).
    #[serde(default, skip_serializing_if = "Option::is_none", with = "presence_pack_serde")]
    pub presence_pack: Option<Vec<u8>>,
    pub connected_at_ms: i64,
    /// @emoji 🪪️ Authenticated semio_hub user id, when this peer connected with an `AuthSession` rather than an anonymous share token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// @emoji 🎚️ The peer's resolved studio role (`"owner"`/`"member"`/`"viewer"`), present alongside `user_id`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// @emoji 🖱️ Live cursor position, when the peer's client streams pointer telemetry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<PresencePoint>,
    /// @emoji 🔭️ The peer's current pan/zoom, for scaling remote cursors/ghosts relative to their view.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport: Option<PresenceViewport>,
    /// @emoji 👻️ Serialized preview of an in-flight drag (opaque JSON, schema owned by the dragging app).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_ghost_json: Option<String>,
    /// @emoji 🕹️ This peer's live selection+hover roster, mirrored from local `InteractionState` —
    /// see `assemble_presence_interaction` in `store_sync` for how it gets built (flag bit 7 on the
    /// wire). `None` for peers on apps that declare no interaction domains.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction: Option<PresenceInteraction>,
}

/// @emoji 🎯️ Binary `PresencePeer` codec: `actor str | presence bitmask u8 | connected_at_ms
/// varint | fields present per bitmask`. `protocol_wire::ClientFrame::Presence`/`ServerFrame::
/// Presence` carry the resulting bytes opaquely (that crate has no dependency on this one) —
/// this is the encode/decode pair store_sync calls on either side of the wire.
/// `presence_pack` is length-prefixed bytes in flag bit 1 (formerly `selection_json`);
/// `drag_ghost_json` stays opaque app-owned text (never re-parsed as JSON here,
/// same as `ArtifactDiff.payload` staying opaque bytes). Bit 7 carries `interaction` via
/// `encode_presence_interaction`/`decode_presence_interaction` (self-delimiting varint-counted
/// fields — see the `🔖️PresenceInteraction` region below), the same convention every other
/// variable-length field on this struct already follows.
pub fn encode_presence_peer(peer: &PresencePeer) -> Vec<u8> {
    let mut out = Vec::new();
    crate::os_spr::write_str(&mut out, &peer.actor);
    let mut presence = 0u8;
    if peer.label.is_some() {
        presence |= 1 << 0;
    }
    if peer.presence_pack.is_some() {
        presence |= 1 << 1;
    }
    if peer.user_id.is_some() {
        presence |= 1 << 2;
    }
    if peer.role.is_some() {
        presence |= 1 << 3;
    }
    if peer.cursor.is_some() {
        presence |= 1 << 4;
    }
    if peer.viewport.is_some() {
        presence |= 1 << 5;
    }
    if peer.drag_ghost_json.is_some() {
        presence |= 1 << 6;
    }
    if peer.interaction.is_some() {
        presence |= 1 << 7;
    }
    out.push(presence);
    crate::os_spr::write_varint_u64(&mut out, peer.connected_at_ms as u64);
    if let Some(label) = &peer.label {
        crate::os_spr::write_str(&mut out, label);
    }
    if let Some(presence_pack) = &peer.presence_pack {
        crate::os_spr::write_bytes(&mut out, presence_pack);
    }
    if let Some(user_id) = &peer.user_id {
        crate::os_spr::write_str(&mut out, user_id);
    }
    if let Some(role) = &peer.role {
        crate::os_spr::write_str(&mut out, role);
    }
    if let Some(cursor) = &peer.cursor {
        crate::os_spr::write_f64(&mut out, cursor.x);
        crate::os_spr::write_f64(&mut out, cursor.y);
    }
    if let Some(viewport) = &peer.viewport {
        crate::os_spr::write_f64(&mut out, viewport.x);
        crate::os_spr::write_f64(&mut out, viewport.y);
        crate::os_spr::write_f64(&mut out, viewport.zoom);
    }
    if let Some(drag_ghost_json) = &peer.drag_ghost_json {
        crate::os_spr::write_str(&mut out, drag_ghost_json);
    }
    if let Some(interaction) = &peer.interaction {
        encode_presence_interaction(interaction, &mut out);
    }
    out
}

/// @emoji 🎯️ Inverse of [`encode_presence_peer`].
pub fn decode_presence_peer(bytes: &[u8]) -> Result<PresencePeer, crate::os_spr::ProtocolError> {
    let mut pos = 0usize;
    let actor = crate::os_spr::read_str(bytes, &mut pos)?;
    let presence = *bytes.get(pos).ok_or(crate::os_spr::ProtocolError::Malformed { what: "presence peer", offset: pos as u64, detail: "truncated".to_string() })?;
    pos += 1;
    let connected_at_ms = crate::os_spr::read_varint_u64(bytes, &mut pos)? as i64;
    let label = if presence & (1 << 0) != 0 { Some(crate::os_spr::read_str(bytes, &mut pos)?) } else { None };
    let presence_pack = if presence & (1 << 1) != 0 { Some(crate::os_spr::read_bytes(bytes, &mut pos)?) } else { None };
    let user_id = if presence & (1 << 2) != 0 { Some(crate::os_spr::read_str(bytes, &mut pos)?) } else { None };
    let role = if presence & (1 << 3) != 0 { Some(crate::os_spr::read_str(bytes, &mut pos)?) } else { None };
    let cursor = if presence & (1 << 4) != 0 {
        let x = crate::os_spr::read_f64(bytes, &mut pos)?;
        let y = crate::os_spr::read_f64(bytes, &mut pos)?;
        Some(PresencePoint { x, y })
    } else {
        None
    };
    let viewport = if presence & (1 << 5) != 0 {
        let x = crate::os_spr::read_f64(bytes, &mut pos)?;
        let y = crate::os_spr::read_f64(bytes, &mut pos)?;
        let zoom = crate::os_spr::read_f64(bytes, &mut pos)?;
        Some(PresenceViewport { x, y, zoom })
    } else {
        None
    };
    let drag_ghost_json = if presence & (1 << 6) != 0 { Some(crate::os_spr::read_str(bytes, &mut pos)?) } else { None };
    let interaction = if presence & (1 << 7) != 0 { Some(decode_presence_interaction(bytes, &mut pos)?) } else { None };
    Ok(PresencePeer { actor, label, presence_pack, connected_at_ms, user_id, role, cursor, viewport, drag_ghost_json, interaction })
}

#[cfg(test)]
mod presence_codec_tests {
    use super::{decode_presence_peer, encode_presence_peer, PresenceDomain, PresenceInteraction, PresencePeer, PresencePoint, PresenceViewport};

    #[test]
    fn presence_peer_binary_round_trips_with_every_field_absent() {
        let peer = PresencePeer { actor: "peer-1".into(), label: None, presence_pack: None, connected_at_ms: 1000, user_id: None, role: None, cursor: None, viewport: None, drag_ghost_json: None, interaction: None };
        let bytes = encode_presence_peer(&peer);
        assert_eq!(decode_presence_peer(&bytes).unwrap(), peer);
    }

    #[test]
    fn presence_peer_binary_round_trips_with_every_field_present() {
        let peer = PresencePeer {
            actor: "peer-2".into(),
            label: Some("Ada".into()),
            presence_pack: Some(b"{\"ids\":[1,2]}".to_vec()),
            connected_at_ms: 1_700_000_000_000,
            user_id: Some("user-9".into()),
            role: Some("owner".into()),
            cursor: Some(PresencePoint { x: 1.5, y: -2.25 }),
            viewport: Some(PresenceViewport { x: 0.0, y: 10.0, zoom: 1.75 }),
            drag_ghost_json: Some("{\"kind\":\"move\"}".into()),
            interaction: Some(PresenceInteraction {
                app_id: "draw".into(),
                domains: vec![PresenceDomain { domain: "graph".into(), granularity: "node".into(), selected: vec!["n1".into()], hovered: vec!["n2".into()] }],
            }),
        };
        let bytes = encode_presence_peer(&peer);
        assert_eq!(decode_presence_peer(&bytes).unwrap(), peer);
    }

    //#region 🔖️InteractionBit
    fn peer_with_interaction(interaction: Option<PresenceInteraction>) -> PresencePeer {
        PresencePeer { actor: "peer-3".into(), label: None, presence_pack: None, connected_at_ms: 1000, user_id: None, role: None, cursor: None, viewport: None, drag_ghost_json: None, interaction }
    }

    /// 🔎️ Presence byte index: `actor str`'s own varint-length prefix (1 byte for `peer_with_interaction`'s
    /// short actor id) plus the actor bytes themselves.
    fn presence_flag_byte(peer: &PresencePeer, bytes: &[u8]) -> u8 {
        bytes[1 + peer.actor.len()]
    }

    #[test]
    fn presence_peer_bit_7_round_trips_with_interaction_present() {
        let peer = peer_with_interaction(Some(PresenceInteraction { app_id: "draw".into(), domains: vec![PresenceDomain { domain: "graph".into(), granularity: "node".into(), selected: vec!["n1".into(), "n2".into()], hovered: vec![] }] }));
        let bytes = encode_presence_peer(&peer);
        assert_eq!(presence_flag_byte(&peer, &bytes) & (1 << 7), 1 << 7, "bit 7 set when interaction present");
        assert_eq!(decode_presence_peer(&bytes).unwrap(), peer);
    }

    #[test]
    fn presence_peer_bit_7_round_trips_with_interaction_absent() {
        let peer = peer_with_interaction(None);
        let bytes = encode_presence_peer(&peer);
        assert_eq!(presence_flag_byte(&peer, &bytes) & (1 << 7), 0, "bit 7 clear when interaction absent");
        assert_eq!(decode_presence_peer(&bytes).unwrap(), peer);
    }

    #[test]
    fn presence_peer_interaction_round_trips_with_multiple_domains() {
        let peer = peer_with_interaction(Some(PresenceInteraction {
            app_id: "space".into(),
            domains: vec![
                PresenceDomain { domain: "outline".into(), granularity: "task".into(), selected: vec!["t1".into(), "t2".into()], hovered: vec!["t3".into()] },
                PresenceDomain { domain: "board".into(), granularity: "card".into(), selected: vec![], hovered: vec!["c1".into(), "c2".into(), "c3".into()] },
                PresenceDomain { domain: "canvas".into(), granularity: "node".into(), selected: vec!["n9".into()], hovered: vec![] },
            ],
        }));
        let bytes = encode_presence_peer(&peer);
        let decoded = decode_presence_peer(&bytes).unwrap();
        assert_eq!(decoded, peer);
        assert_eq!(decoded.interaction.unwrap().domains.len(), 3);
    }
    //#endregion 🔖️InteractionBit
}

//#endregion 🔖️Presence

//#region 🔖️Interaction
// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: relocated here (from
// `semio-framework`'s `🔨️modules/🕹️interaction/{🦀️component.rs,🧬️schema/🦀️component.rs}`) to
// unblock naming `InteractionState`/`PresenceInteraction` from `store`/`sync` — `semio-framework`
// depends on this crate, never the reverse, so nothing under `os_spr` could previously name a
// framework-defined type. `InteractionDefinition`/`GranularityDefinition`/`InteractionRef` stay in
// `semio-framework` (their `label`/`icon_id` fields pull in `ui_wgpu::LocalizedLabel`/`IconName`,
// which this wasm-safe kernel crate does not and must not depend on) and now `pub use` everything
// below instead of redefining it — see that module's own header comment. Every wave-0 test covering
// this code moved with it, verbatim except `validate_state`'s fixtures (see `InteractionOutline`).
/// 🐁️ One domain's hover behavior — see `semio_framework::InteractionDefinition::hover`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct HoverSpec {
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// 🌳️ Whether hovering a target expands to its descendant closure (root first) — requires
    /// `hierarchy != HierarchyProvider::Flat`.
    #[serde(default)]
    pub transitive: bool,
    /// 📡️ Named hover channels this domain accepts (e.g. `["pointer"]`); the shared cursor throttle
    /// keys off the same channel names.
    #[serde(default = "default_pointer_channels")]
    pub channels: Vec<String>,
    /// 📣️ Whether this domain's own hover mirrors into `PresenceInteraction` for peers.
    #[serde(default = "default_true")]
    pub broadcast: bool,
}

impl Default for HoverSpec {
    fn default() -> Self {
        Self { enabled: true, transitive: false, channels: default_pointer_channels(), broadcast: true }
    }
}

/// 🖱️ One domain's selection behavior — see `semio_framework::InteractionDefinition::selection`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct SelectionSpec {
    /// 🪜️ Non-empty; the first entry is the domain's default mode.
    pub modes: Vec<SelectionMode>,
    pub methods: Vec<SelectionMethod>,
    pub merges: Vec<MergeMode>,
    /// 🌳️ Whether selecting a target expands to its descendant closure — requires
    /// `hierarchy != HierarchyProvider::Flat`.
    #[serde(default)]
    pub transitive: bool,
    /// 📣️ Whether this domain's own selection mirrors into `PresenceInteraction` for peers.
    #[serde(default = "default_true")]
    pub broadcast: bool,
}

/// 🌳️ Where a domain's target ids come from, and thus what `DomainTopology` (if any) is available for
/// range selection and transitive hover/select closures.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum HierarchyProvider {
    /// 🪨️ No parent/child structure — range and transitive closures degrade to the single target.
    Flat,
    /// 🕸️ App-supplied topology (`ArtifactApp::interaction_topology`), e.g. a DAG or scene graph.
    Topology,
    /// 🌲️ Derived from the rendered `UiTree` shape for this domain.
    UiTree,
    /// 🧵️ Derived from splitting each target id on `delimiter` (e.g. `♾️infinite`'s `"surfaceId/id"`).
    PathDelimited { delimiter: String },
}

/// 🔢️ How many targets may be selected at once within a domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum SelectionMode {
    Single,
    Multiple,
}

/// 🎯️ How a surface gathers targets for one `interactionSelect` dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum SelectionMethod {
    Pick,
    Rectangle,
    Lasso,
}

/// 🧮️ Set algebra applied when merging new targets into the current selection — see `next_selection`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum MergeMode {
    Replace,
    Additive,
    Subtractive,
    Invertive,
    Range,
}

fn default_true() -> bool {
    true
}

fn default_pointer_channels() -> Vec<String> {
    vec!["pointer".to_string()]
}

//#region 🔖️Runtime
/// 🎯️ One addressed target: a granularity id plus the target's own id (u32 domain ids are stringified
/// at the app boundary before reaching this module).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct InteractionTarget {
    pub granularity: String,
    pub id: String,
}

/// 🖱️ One domain's current selection: the active granularity, the selected ids, and the anchor id
/// range selection pivots from.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DomainSelection {
    pub granularity: String,
    pub ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub anchor_id: Option<String>,
}

/// 🐁️ One domain's current hover on one channel: the transitive closure (root first) when
/// `HoverSpec::transitive`, otherwise just the raw hovered ids.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DomainHover {
    pub channel: String,
    pub ids: Vec<String>,
}

/// 🗺️ Own persisted-local selection (`Interaction` history lane) + ephemeral-local hover, keyed by
/// domain id — the framework-owned counterpart to what every per-app config used to hand-roll.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct InteractionState {
    pub selection: BTreeMap<String, DomainSelection>,
    pub hover: BTreeMap<String, DomainHover>,
    pub active_mode: BTreeMap<String, SelectionMode>,
    pub active_granularity: BTreeMap<String, String>,
}
//#endregion 🔖️Runtime

//#region 🔖️Topology
/// 🌳️ One node of a domain's topology: its own granularity and its parent id (`None` = a root).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct TopologyNode {
    pub id: String,
    pub granularity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub parent: Option<String>,
}

/// 🌲️ One domain's topology, pre-order: `ordered`'s sequence IS the range-selection order, and every
/// node's descendants form a contiguous run immediately following it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DomainTopology {
    pub ordered: Vec<TopologyNode>,
}

impl DomainTopology {
    /// 🔎️ The pre-order index of `id`, or `None` when absent.
    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.ordered.iter().position(|node| node.id == id)
    }

    /// ✅️ Whether `id` is a known node in this topology.
    pub fn contains(&self, id: &str) -> bool {
        self.index_of(id).is_some()
    }

    fn children_by_parent(&self) -> BTreeMap<String, Vec<String>> {
        let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for node in &self.ordered {
            if let Some(parent) = &node.parent {
                children.entry(parent.clone()).or_default().push(node.id.clone());
            }
        }
        children
    }

    /// 🌳️ `root_id` plus every descendant, pre-order (root first) — empty when `root_id` is absent.
    pub fn descendant_closure(&self, root_id: &str) -> Vec<String> {
        if !self.contains(root_id) {
            return Vec::new();
        }
        let children = self.children_by_parent();
        let mut out = Vec::new();
        visit_descendants(root_id, &children, &mut out);
        out
    }

    /// 🪜️ `id`'s ancestor chain, nearest parent first, root last.
    pub fn ancestors(&self, id: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut current = self.ordered.iter().find(|node| node.id == id).and_then(|node| node.parent.clone());
        while let Some(parent_id) = current {
            current = self.ordered.iter().find(|node| node.id == parent_id).and_then(|node| node.parent.clone());
            out.push(parent_id);
        }
        out
    }
}

fn visit_descendants(id: &str, children: &BTreeMap<String, Vec<String>>, out: &mut Vec<String>) {
    out.push(id.to_string());
    if let Some(kids) = children.get(id) {
        for kid in kids {
            visit_descendants(kid, children, out);
        }
    }
}

/// 🗺️ Every domain's topology for one app instance, keyed by domain id — `ArtifactApp::interaction_topology`
/// returns this (wave 3).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct InteractionTopology {
    pub domains: BTreeMap<String, DomainTopology>,
}
//#endregion 🔖️Topology

//#region 🔖️SelectionMachine
/// 🖱️ One `next_selection` call's input: the batch of targets (a single pick or a marquee gather),
/// the merge mode to apply, and the currently active selection mode.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct SelectionInput {
    pub targets: Vec<InteractionTarget>,
    pub merge: MergeMode,
    pub mode: SelectionMode,
}

/// 🖱️ Computes the next `DomainSelection` for one domain — the generalization of Tree's
/// `getTreeNextSelectionState` (`🖱️ui/🧱️elements/🪵️Tree/🟦️component.tsx:946-968`), preserving its
/// exact single/range/toggle semantics while adding batch targets, `Additive`/`Subtractive` as
/// distinct merges, and transitive descendant-closure expansion.
///
/// - `Single` mode ignores `merge` entirely and clamps to the batch's last target (mirrors Tree
///   returning `{selectedIds:[targetId]}` unconditionally in single mode).
/// - `Range` replaces the selection with the topology-order slice between the anchor (falling back to
///   `current.anchor_id`, then `current.ids.last()`, then the target itself — mirrors Tree's
///   `fallbackAnchorId`) and the batch's last target, ascending index order; the anchor does not move.
/// - `Replace`/`Additive`/`Subtractive`/`Invertive` apply ordinary set algebra over the batch's targets
///   (each expanded to its descendant closure first when `spec.transitive`), and update the anchor to
///   the batch's last target.
///
/// Empty `input.targets` is a no-op (returns `current` unchanged).
pub fn next_selection(spec: &SelectionSpec, current: &DomainSelection, topo: &DomainTopology, input: &SelectionInput) -> DomainSelection {
    let Some(last_target) = input.targets.last() else {
        return current.clone();
    };
    let granularity = last_target.granularity.clone();
    let target_ids: Vec<String> = input.targets.iter().map(|target| target.id.clone()).collect();
    let last_target_id = last_target.id.clone();

    if input.mode == SelectionMode::Single {
        return DomainSelection { granularity, ids: vec![last_target_id.clone()], anchor_id: Some(last_target_id) };
    }

    if input.merge == MergeMode::Range {
        let fallback_anchor = current.anchor_id.clone().or_else(|| current.ids.last().cloned()).unwrap_or_else(|| last_target_id.clone());
        if let (Some(anchor_index), Some(target_index)) = (topo.index_of(&fallback_anchor), topo.index_of(&last_target_id)) {
            let (start, end) = if anchor_index <= target_index { (anchor_index, target_index) } else { (target_index, anchor_index) };
            let ids = topo.ordered[start..=end].iter().map(|node| node.id.clone()).collect();
            return DomainSelection { granularity, ids, anchor_id: Some(fallback_anchor) };
        }
        return DomainSelection { granularity, ids: vec![last_target_id.clone()], anchor_id: Some(last_target_id) };
    }

    let expanded: Vec<String> = target_ids
        .iter()
        .flat_map(|id| {
            if spec.transitive {
                let closure = topo.descendant_closure(id);
                if closure.is_empty() {
                    vec![id.clone()]
                } else {
                    closure
                }
            } else {
                vec![id.clone()]
            }
        })
        .collect();

    let mut ids = match input.merge {
        MergeMode::Replace => dedup_preserving_order(expanded),
        MergeMode::Additive => {
            let mut ids = current.ids.clone();
            for id in expanded {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
            ids
        }
        MergeMode::Subtractive => current.ids.iter().filter(|id| !expanded.contains(id)).cloned().collect(),
        MergeMode::Invertive => {
            let mut ids = current.ids.clone();
            for id in expanded {
                match ids.iter().position(|existing| *existing == id) {
                    Some(index) => {
                        ids.remove(index);
                    }
                    None => ids.push(id),
                }
            }
            ids
        }
        MergeMode::Range => unreachable!("Range handled above"),
    };
    ids = dedup_preserving_order(ids);
    DomainSelection { granularity, ids, anchor_id: Some(last_target_id) }
}

fn dedup_preserving_order(ids: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(ids.len());
    for id in ids {
        if !out.contains(&id) {
            out.push(id);
        }
    }
    out
}
//#endregion 🔖️SelectionMachine

//#region 🔖️HoverMachine
/// 🐁️ One `next_hover` call's input: the channel and the batch of hovered targets (empty = clear).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct HoverInput {
    pub channel: String,
    pub targets: Vec<InteractionTarget>,
}

/// 🐁️ Computes the next `DomainHover` for one channel: always REPLACES the channel's id list (hover
/// has no merge algebra). When `spec.transitive`, each target expands to its descendant closure with
/// the hovered root first; multiple targets concatenate in input order, deduplicated. Disabled specs
/// and empty target batches both clear the channel.
pub fn next_hover(spec: &HoverSpec, topo: &DomainTopology, input: &HoverInput) -> DomainHover {
    if !spec.enabled || input.targets.is_empty() {
        return DomainHover { channel: input.channel.clone(), ids: Vec::new() };
    }
    let mut ids: Vec<String> = Vec::new();
    for target in &input.targets {
        let expanded = if spec.transitive {
            let closure = topo.descendant_closure(&target.id);
            if closure.is_empty() {
                vec![target.id.clone()]
            } else {
                closure
            }
        } else {
            vec![target.id.clone()]
        };
        for id in expanded {
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
    }
    DomainHover { channel: input.channel.clone(), ids }
}
//#endregion 🔖️HoverMachine

//#region 🔖️Validation
/// 🪞️ The label/icon-free projection of a domain's `InteractionDefinition` that `validate_state`
/// needs — `semio-framework`'s `InteractionDefinition::outline()` builds one of these per call since
/// this crate cannot name `InteractionDefinition` itself (see this region's header comment).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct InteractionOutline {
    pub id: String,
    /// 🪜️ Non-empty; the first entry is the domain's default granularity.
    pub granularity_ids: Vec<String>,
    pub selection: SelectionSpec,
}

/// 🧹️ Re-derives a consistent `InteractionState` from declared `defs` + current `topo`: drops any
/// domain absent from `defs` (renamed/removed interaction declaration), prunes selection/hover ids no
/// longer present in that domain's topology (deleted document nodes — called after every artifact
/// dispatch), resets `active_granularity`/`active_mode` to a declared value (falling back to the
/// domain's default, its first declared entry) when the stored one is no longer declared, and clamps
/// `Single`-mode selections down to their first id (mirrors `normalizeTreeSelectedIds`'s external-update
/// normalization, not `next_selection`'s recency-preferring clamp).
pub fn validate_state(defs: &[InteractionOutline], topo: &InteractionTopology, state: &InteractionState) -> InteractionState {
    let mut result = InteractionState::default();

    for def in defs {
        let domain_topo = topo.domains.get(&def.id);
        let declared_granularities: Vec<&str> = def.granularity_ids.iter().map(String::as_str).collect();
        let default_granularity = def.granularity_ids.first().cloned().unwrap_or_default();
        let default_mode = def.selection.modes.first().copied().unwrap_or(SelectionMode::Single);

        let mode = state.active_mode.get(&def.id).copied().filter(|mode| def.selection.modes.contains(mode)).unwrap_or(default_mode);
        result.active_mode.insert(def.id.clone(), mode);

        let granularity = state.active_granularity.get(&def.id).cloned().filter(|granularity| declared_granularities.contains(&granularity.as_str())).unwrap_or_else(|| default_granularity.clone());
        result.active_granularity.insert(def.id.clone(), granularity);

        if let Some(selection) = state.selection.get(&def.id) {
            let selection_granularity = if declared_granularities.contains(&selection.granularity.as_str()) { selection.granularity.clone() } else { default_granularity.clone() };
            let mut ids: Vec<String> = selection.ids.iter().filter(|id| domain_topo.is_none_or(|topo| topo.contains(id))).cloned().collect();
            if mode == SelectionMode::Single && ids.len() > 1 {
                ids.truncate(1);
            }
            let anchor_id = selection.anchor_id.clone().filter(|anchor| ids.contains(anchor));
            result.selection.insert(def.id.clone(), DomainSelection { granularity: selection_granularity, ids, anchor_id });
        }

        if let Some(hover) = state.hover.get(&def.id) {
            let ids: Vec<String> = hover.ids.iter().filter(|id| domain_topo.is_none_or(|topo| topo.contains(id))).cloned().collect();
            result.hover.insert(def.id.clone(), DomainHover { channel: hover.channel.clone(), ids });
        }
    }

    result
}
//#endregion 🔖️Validation

//#region 🔖️PresenceInteraction
/// 📡️ One peer's interaction roster for one app instance, mirrored onto `PresencePeer.interaction`
/// (bit 7) on the heartbeat — typed (not app-opaque `presence_pack`) so the Shell renders every peer's
/// selection/hover generically. Only explicit ids broadcast; receivers expand transitive closures via
/// their own topology.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PresenceInteraction {
    pub app_id: String,
    pub domains: Vec<PresenceDomain>,
}

/// 📡️ One domain's broadcast slice of `PresenceInteraction` — the peer-facing mirror of a domain's
/// `DomainSelection`/`DomainHover`, flattened to raw explicit ids (no transitive expansion on the wire).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PresenceDomain {
    pub domain: String,
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
}

//#region 🔖️PresenceInteractionCodec
// 🎯️ Binary codec for the two structs above — what `encode_presence_peer`/`decode_presence_peer`
// (in the `🔖️Presence` region) call for bit 7. Self-delimiting throughout (varint counts, exactly
// `write_vec_bytes`/`write_vec_envelope`'s own convention in `🔖️Combinators` up top), so no outer
// length prefix is needed around the whole payload.
fn write_vec_str(out: &mut Vec<u8>, values: &[String]) {
    crate::os_spr::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::os_spr::write_str(out, value);
    }
}

fn read_vec_str(bytes: &[u8], pos: &mut usize) -> Result<Vec<String>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| crate::os_spr::read_str(bytes, pos)).collect()
}

fn encode_presence_domain(domain: &PresenceDomain, out: &mut Vec<u8>) {
    crate::os_spr::write_str(out, &domain.domain);
    crate::os_spr::write_str(out, &domain.granularity);
    write_vec_str(out, &domain.selected);
    write_vec_str(out, &domain.hovered);
}

fn decode_presence_domain(bytes: &[u8], pos: &mut usize) -> Result<PresenceDomain, crate::os_spr::ProtocolError> {
    Ok(PresenceDomain { domain: crate::os_spr::read_str(bytes, pos)?, granularity: crate::os_spr::read_str(bytes, pos)?, selected: read_vec_str(bytes, pos)?, hovered: read_vec_str(bytes, pos)? })
}

fn encode_presence_interaction(interaction: &PresenceInteraction, out: &mut Vec<u8>) {
    crate::os_spr::write_str(out, &interaction.app_id);
    crate::os_spr::write_varint_u64(out, interaction.domains.len() as u64);
    for domain in &interaction.domains {
        encode_presence_domain(domain, out);
    }
}

fn decode_presence_interaction(bytes: &[u8], pos: &mut usize) -> Result<PresenceInteraction, crate::os_spr::ProtocolError> {
    let app_id = crate::os_spr::read_str(bytes, pos)?;
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    let domains = (0..count).map(|_| decode_presence_domain(bytes, pos)).collect::<Result<Vec<_>, _>>()?;
    Ok(PresenceInteraction { app_id, domains })
}
//#endregion 🔖️PresenceInteractionCodec
//#endregion 🔖️PresenceInteraction

//#region 🔖️InteractionStorePack
/// 📦️ `ArtifactPack` for `InteractionState` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
/// W3b: lets `semio-framework-plugin`'s `VcsArtifactApp` own a real `store::ConfigStore<InteractionState,
/// _>` (persisting selection + active mode/granularity per app instance through the `HistoryLane::Interaction`
/// mechanism above) exactly like any other `ArtifactStore<P, _>`. `InteractionState` has no `RecordSpec`/
/// `dsl_derive` record lowering of its own (a small, framework-internal `BTreeMap`-keyed value, not an app
/// document), so this bridges through the same schema-less `serde_json::Value` pack codec `crate::os_store`'s
/// own `impl ArtifactPack for DslValue` ("Compose-only pack bridge") already uses, rather than hand-rolling a
/// codec. MUST live here, not in `semio-framework-plugin`: the orphan rule requires an impl of a foreign
/// trait for a foreign type to sit in the crate owning one of the two, and both `ArtifactPack` (`os_store`)
/// and `InteractionState` (this region) are this crate's own — `semio-framework-plugin` only sees both
/// through its `store`/`protocol` aliases.
impl crate::os_store::ArtifactPack for InteractionState {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let value = serde_json::to_value(self).map_err(|error| crate::os_store::PackError::Schema(error.to_string()))?;
        crate::os_store::ArtifactPack::encode_pack_with(&value, options)
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let value = <serde_json::Value as crate::os_store::ArtifactPack>::decode_pack_with(bytes, options)?;
        serde_json::from_value(value).map_err(|error| crate::os_store::PackError::Schema(error.to_string()))
    }
}
//#endregion 🔖️InteractionStorePack

#[cfg(test)]
mod interaction_tests {
    use super::*;

    //#region 🔖️Fixtures
    /// 🌲️ root → {a → {a1, a2}, b → {b1}}, pre-order: root, a, a1, a2, b, b1.
    fn sample_topology() -> DomainTopology {
        let node = |id: &str, parent: Option<&str>| TopologyNode { id: id.into(), granularity: "node".into(), parent: parent.map(Into::into) };
        DomainTopology { ordered: vec![node("root", None), node("a", Some("root")), node("a1", Some("a")), node("a2", Some("a")), node("b", Some("root")), node("b1", Some("b"))] }
    }

    fn target(id: &str) -> InteractionTarget {
        InteractionTarget { granularity: "node".into(), id: id.into() }
    }

    fn selection(ids: &[&str], anchor: Option<&str>) -> DomainSelection {
        DomainSelection { granularity: "node".into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: anchor.map(Into::into) }
    }

    fn spec(transitive: bool, merges: &[MergeMode]) -> SelectionSpec {
        SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: merges.to_vec(), transitive, broadcast: true }
    }

    fn multiple_input(ids: &[&str], merge: MergeMode) -> SelectionInput {
        SelectionInput { targets: ids.iter().map(|id| target(id)).collect(), merge, mode: SelectionMode::Multiple }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MergeModes
    #[test]
    fn replace_sets_selection_to_batch_targets() {
        let current = selection(&["a1"], Some("a1"));
        let next = next_selection(&spec(false, &[MergeMode::Replace]), &current, &sample_topology(), &multiple_input(&["b", "b1"], MergeMode::Replace));
        assert_eq!(next.ids, vec!["b".to_string(), "b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("b1"));
    }

    #[test]
    fn additive_unions_batch_into_current_selection() {
        let current = selection(&["a1"], Some("a1"));
        let next = next_selection(&spec(false, &[MergeMode::Additive]), &current, &sample_topology(), &multiple_input(&["a2"], MergeMode::Additive));
        assert_eq!(next.ids, vec!["a1".to_string(), "a2".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a2"));
    }

    #[test]
    fn subtractive_removes_batch_from_current_selection() {
        let current = selection(&["a1", "a2", "b1"], Some("b1"));
        let next = next_selection(&spec(false, &[MergeMode::Subtractive]), &current, &sample_topology(), &multiple_input(&["a2"], MergeMode::Subtractive));
        assert_eq!(next.ids, vec!["a1".to_string(), "b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a2"), "anchor tracks the last acted-on target, even on removal");
    }

    #[test]
    fn invertive_toggles_each_batch_target_independently() {
        let current = selection(&["a1", "a2"], Some("a2"));
        let next = next_selection(&spec(false, &[MergeMode::Invertive]), &current, &sample_topology(), &multiple_input(&["a2", "b1"], MergeMode::Invertive));
        assert_eq!(next.ids, vec!["a1".to_string(), "b1".to_string()], "a2 was present so it toggles off, b1 was absent so it toggles on");
    }
    //#endregion 🔖️MergeModes

    //#region 🔖️Range
    #[test]
    fn range_slices_topology_order_between_anchor_and_target() {
        let current = selection(&["a"], Some("a"));
        let next = next_selection(&spec(false, &[MergeMode::Range]), &current, &sample_topology(), &multiple_input(&["b1"], MergeMode::Range));
        assert_eq!(next.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string(), "b".to_string(), "b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a"), "range never moves the anchor");
    }

    #[test]
    fn range_falls_back_to_last_selected_id_when_no_anchor_recorded() {
        let current = selection(&["a1", "a2"], None);
        let next = next_selection(&spec(false, &[MergeMode::Range]), &current, &sample_topology(), &multiple_input(&["b"], MergeMode::Range));
        assert_eq!(next.ids, vec!["a2".to_string(), "b".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a2"));
    }

    #[test]
    fn range_handles_target_before_anchor_in_topology_order() {
        let current = selection(&["b"], Some("b"));
        let next = next_selection(&spec(false, &[MergeMode::Range]), &current, &sample_topology(), &multiple_input(&["a1"], MergeMode::Range));
        assert_eq!(next.ids, vec!["a1".to_string(), "a2".to_string(), "b".to_string()]);
    }
    //#endregion 🔖️Range

    //#region 🔖️SingleClamp
    #[test]
    fn single_mode_clamps_to_last_target_regardless_of_merge() {
        let current = selection(&["a1", "a2"], Some("a1"));
        let input = SelectionInput { targets: vec![target("b"), target("b1")], merge: MergeMode::Additive, mode: SelectionMode::Single };
        let next = next_selection(&spec(false, &[MergeMode::Additive]), &current, &sample_topology(), &input);
        assert_eq!(next.ids, vec!["b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("b1"));
    }
    //#endregion 🔖️SingleClamp

    //#region 🔖️Transitive
    #[test]
    fn transitive_select_expands_target_to_descendant_closure() {
        let current = DomainSelection::default();
        let next = next_selection(&spec(true, &[MergeMode::Replace]), &current, &sample_topology(), &multiple_input(&["a"], MergeMode::Replace));
        assert_eq!(next.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string()]);
    }

    #[test]
    fn transitive_hover_expands_with_root_first() {
        let hover_spec = HoverSpec { enabled: true, transitive: true, channels: default_pointer_channels(), broadcast: true };
        let input = HoverInput { channel: "pointer".into(), targets: vec![target("a")] };
        let hover = next_hover(&hover_spec, &sample_topology(), &input);
        assert_eq!(hover.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string()]);
        assert_eq!(hover.ids.first().map(String::as_str), Some("a"), "hovered root sorts first");
    }

    #[test]
    fn non_transitive_hover_replaces_with_raw_targets_only() {
        let hover_spec = HoverSpec { enabled: true, transitive: false, channels: default_pointer_channels(), broadcast: true };
        let input = HoverInput { channel: "pointer".into(), targets: vec![target("a")] };
        let hover = next_hover(&hover_spec, &sample_topology(), &input);
        assert_eq!(hover.ids, vec!["a".to_string()]);
    }

    #[test]
    fn empty_hover_targets_clears_the_channel() {
        let hover_spec = HoverSpec::default();
        let hover = next_hover(&hover_spec, &sample_topology(), &HoverInput { channel: "pointer".into(), targets: Vec::new() });
        assert!(hover.ids.is_empty());
    }
    //#endregion 🔖️Transitive

    //#region 🔖️ValidateState
    fn sample_outline() -> InteractionOutline {
        InteractionOutline { id: "graph".into(), granularity_ids: vec!["node".into(), "edge".into()], selection: spec(false, &[MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range]) }
    }

    #[test]
    fn validate_state_prunes_ids_absent_from_topology() {
        let def = sample_outline();
        let mut topo = InteractionTopology::default();
        topo.domains.insert("graph".into(), sample_topology());

        let mut state = InteractionState::default();
        state.selection.insert("graph".into(), selection(&["a1", "deleted-node", "b1"], Some("deleted-node")));
        state.hover.insert("graph".into(), DomainHover { channel: "pointer".into(), ids: vec!["a1".into(), "gone".into()] });
        state.active_mode.insert("graph".into(), SelectionMode::Multiple);
        state.active_granularity.insert("graph".into(), "node".into());

        let validated = validate_state(&[def], &topo, &state);
        let graph_selection = validated.selection.get("graph").expect("graph domain kept");
        assert_eq!(graph_selection.ids, vec!["a1".to_string(), "b1".to_string()], "deleted-node pruned");
        assert_eq!(graph_selection.anchor_id, None, "stale anchor pruned along with its id");
        assert_eq!(validated.hover.get("graph").unwrap().ids, vec!["a1".to_string()], "gone pruned");
    }

    #[test]
    fn validate_state_drops_undeclared_domains_and_granularities() {
        let def = sample_outline();
        let topo = InteractionTopology::default();

        let mut state = InteractionState::default();
        state.selection.insert("mesh".into(), selection(&["x"], None));
        state.active_granularity.insert("graph".into(), "face".into());

        let validated = validate_state(&[def], &topo, &state);
        assert!(validated.selection.get("mesh").is_none(), "undeclared domain dropped");
        assert_eq!(validated.active_granularity.get("graph").map(String::as_str), Some("node"), "undeclared granularity resets to the default");
    }

    #[test]
    fn validate_state_clamps_single_mode_selection_to_first_id() {
        let def = sample_outline();
        let mut topo = InteractionTopology::default();
        topo.domains.insert("graph".into(), sample_topology());

        let mut state = InteractionState::default();
        state.selection.insert("graph".into(), selection(&["a1", "a2", "b1"], None));
        state.active_mode.insert("graph".into(), SelectionMode::Single);

        let validated = validate_state(&[def], &topo, &state);
        assert_eq!(validated.selection.get("graph").unwrap().ids, vec!["a1".to_string()]);
    }
    //#endregion 🔖️ValidateState

    //#region 🔖️Serde
    #[test]
    fn hierarchy_provider_serializes_internally_tagged_variants() {
        let path_delimited = HierarchyProvider::PathDelimited { delimiter: "/".into() };
        let json = serde_json::to_string(&path_delimited).unwrap();
        assert_eq!(json, "{\"kind\":\"pathDelimited\",\"delimiter\":\"/\"}");
        assert_eq!(serde_json::from_str::<HierarchyProvider>(&json).unwrap(), path_delimited);

        let flat_json = serde_json::to_string(&HierarchyProvider::Flat).unwrap();
        assert_eq!(flat_json, "{\"kind\":\"flat\"}");
    }
    //#endregion 🔖️Serde
}
//#endregion 🔖️Interaction
