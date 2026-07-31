//! 🎞️ Protocol semio_hub wire frames: the lane-tagged `ClientFrame`/`ServerFrame` envelopes a
//! browser/native sync client exchanges with the collaboration semio_hub, plus their binary codec. Frozen
//! contract: `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment`
//! §`protocol_wire`.
//!
//! 🎯️ W5: the byte encoding is now a fully hand-rolled binary layout — `lane: u8` followed by
//! `frame tag: u8` (the frame enum's variant declaration order) and its fields in declaration
//! order, with no body-length prefix (one frame per WS message) and no per-field tags. This
//! matches `protocol_core::🔖️WireCodec`'s convention (also used by `protocol_causal::🔖️EnvelopeCodec`
//! and `dsl::op_rt`). `DocumentDiff`/`InverseOperation` payloads are opaque `Vec<u8>` (never
//! `serde_json::Value`). `ClientFrame::Presence`/`ServerFrame::Presence` carry opaque presence
//! payload bytes (`peer: Vec<u8>` / `peers: Vec<Vec<u8>>`) — this crate has no dependency on
//! `framework_core` (where the concrete `PresencePeer` type and its binary codec live), so the
//! frame only ever moves the already-encoded blob a caller supplies. `protocol_core` supplies the
//! primitive codec (`write_varint_u64`/`write_str`/`write_bytes`/`write_hash32`/`write_bool` and
//! their `read_*` twins); this crate adds only the option/vec combinators and the frame/nested-enum
//! tag dispatch below.

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
    Hello {
        wire_version: u32,
        protocol_version: u32,
        schema: String,
        pack_schema_hash: [u8; 32],
        actor: protocol_core::ActorId,
        token: Option<String>,
        resume_token: Option<String>,
        frontier: Option<protocol_causal::FrontierSummary>,
    },
    Commands {
        batch_id: u64,
        envelopes: Vec<protocol_causal::OperationEnvelope>,
    },
    FrontierAdvertise {
        frontier: protocol_causal::FrontierSummary,
    },
    PreviewPublish {
        key: String,
        seq: u64,
        payload: Vec<u8>,
    },
    Presence {
        peer: Vec<u8>,
    },
    CreditGrant {
        n: u32,
    },
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
    // 🔒️ Boxed: OperationEnvelope is far larger than the other variants, and clippy's
    // large_enum_variant lint (a real per-instance cost, not just style) applies at -D warnings.
    Transformed { envelope: Box<protocol_causal::OperationEnvelope> },
    Rejected { reason: String }
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
    Welcome {
        session_id: String,
        resume_token: String,
        server_frontier: protocol_causal::FrontierSummary,
        bootstrap: Bootstrap,
    },
    SnapshotChunk {
        seq: u32,
        bytes: Vec<u8>,
    },
    SnapshotDone {
        seq_count: u32,
    },
    Commands {
        envelopes: Vec<protocol_causal::OperationEnvelope>,
        origin: protocol_core::ActorId,
        frontier: protocol_causal::FrontierSummary,
    },
    Ack {
        batch_id: u64,
        stages: Vec<AckStage>,
        frontier: protocol_causal::FrontierSummary,
    },
    Preview {
        actor: protocol_core::ActorId,
        key: String,
        seq: u64,
        payload: Vec<u8>,
    },
    Presence {
        peers: Vec<Vec<u8>>,
    },
    CreditGrant {
        n: u32,
    },
    Error {
        code: String,
        message: String,
    }
}
//#endregion 🔖️ServerFrame

//#region 🔖️Codec
// Hand-rolled binary frame encode/decode: `lane: u8 | frame tag: u8 | fields...` — see the
// module-level docstring. `protocol_core::🔖️WireCodec` supplies the primitives; this region adds
// the option/vec combinators the frame shapes need plus the tag-dispatch match arms.

fn malformed(what: &'static str, offset: u64, detail: &str) -> protocol_core::ProtocolError {
    protocol_core::ProtocolError::Malformed { what, offset, detail: detail.to_string() }
}

//#region 🔖️Combinators
fn write_opt_str(out: &mut Vec<u8>, value: &Option<String>) {
    protocol_core::write_bool(out, value.is_some());
    if let Some(s) = value {
        protocol_core::write_str(out, s);
    }
}

fn read_opt_str(bytes: &[u8], pos: &mut usize) -> Result<Option<String>, protocol_core::ProtocolError> {
    if protocol_core::read_bool(bytes, pos)? { Ok(Some(protocol_core::read_str(bytes, pos)?)) } else { Ok(None) }
}

fn write_opt_bytes(out: &mut Vec<u8>, value: &Option<Vec<u8>>) {
    protocol_core::write_bool(out, value.is_some());
    if let Some(b) = value {
        protocol_core::write_bytes(out, b);
    }
}

fn read_opt_bytes(bytes: &[u8], pos: &mut usize) -> Result<Option<Vec<u8>>, protocol_core::ProtocolError> {
    if protocol_core::read_bool(bytes, pos)? { Ok(Some(protocol_core::read_bytes(bytes, pos)?)) } else { Ok(None) }
}

fn write_opt_frontier(out: &mut Vec<u8>, value: &Option<protocol_causal::FrontierSummary>) {
    protocol_core::write_bool(out, value.is_some());
    if let Some(f) = value {
        protocol_causal::encode_frontier(f, out);
    }
}

fn read_opt_frontier(bytes: &[u8], pos: &mut usize) -> Result<Option<protocol_causal::FrontierSummary>, protocol_core::ProtocolError> {
    if protocol_core::read_bool(bytes, pos)? { Ok(Some(protocol_causal::decode_frontier(bytes, pos)?)) } else { Ok(None) }
}

fn write_vec_bytes(out: &mut Vec<u8>, values: &[Vec<u8>]) {
    protocol_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        protocol_core::write_bytes(out, value);
    }
}

fn read_vec_bytes(bytes: &[u8], pos: &mut usize) -> Result<Vec<Vec<u8>>, protocol_core::ProtocolError> {
    let count = protocol_core::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| protocol_core::read_bytes(bytes, pos)).collect()
}

fn write_vec_envelope(out: &mut Vec<u8>, values: &[protocol_causal::OperationEnvelope]) {
    protocol_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        protocol_causal::encode_envelope(value, out);
    }
}

fn read_vec_envelope(bytes: &[u8], pos: &mut usize) -> Result<Vec<protocol_causal::OperationEnvelope>, protocol_core::ProtocolError> {
    let count = protocol_core::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| protocol_causal::decode_envelope(bytes, pos)).collect()
}
//#endregion 🔖️Combinators

//#region 🔖️NestedEnums
fn encode_bootstrap(bootstrap: &Bootstrap, out: &mut Vec<u8>) {
    match bootstrap {
        Bootstrap::None => out.push(0),
        Bootstrap::Snapshot { pack_hash, inline } => {
            out.push(1);
            protocol_core::write_hash32(out, pack_hash);
            write_opt_bytes(out, inline);
        }
        Bootstrap::Tail => out.push(2),
    }
}

fn decode_bootstrap(bytes: &[u8], pos: &mut usize) -> Result<Bootstrap, protocol_core::ProtocolError> {
    let tag = *bytes.get(*pos).ok_or_else(|| malformed("wire bootstrap tag", *pos as u64, "truncated"))?;
    *pos += 1;
    match tag {
        0 => Ok(Bootstrap::None),
        1 => {
            let pack_hash = protocol_core::read_hash32(bytes, pos)?;
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
            protocol_causal::encode_envelope(envelope, out);
        }
        ApplyOutcome::Rejected { reason } => {
            out.push(2);
            protocol_core::write_str(out, reason);
        }
    }
}

fn decode_apply_outcome(bytes: &[u8], pos: &mut usize) -> Result<ApplyOutcome, protocol_core::ProtocolError> {
    let tag = *bytes.get(*pos).ok_or_else(|| malformed("wire apply-outcome tag", *pos as u64, "truncated"))?;
    *pos += 1;
    match tag {
        0 => Ok(ApplyOutcome::Accepted),
        1 => Ok(ApplyOutcome::Transformed { envelope: Box::new(protocol_causal::decode_envelope(bytes, pos)?) }),
        2 => Ok(ApplyOutcome::Rejected { reason: protocol_core::read_str(bytes, pos)? }),
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

fn decode_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<AckStage, protocol_core::ProtocolError> {
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
    protocol_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_ack_stage(value, out);
    }
}

fn read_vec_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<Vec<AckStage>, protocol_core::ProtocolError> {
    let count = protocol_core::read_varint_u64(bytes, pos)?;
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
            protocol_core::write_varint_u64(&mut out, *wire_version as u64);
            protocol_core::write_varint_u64(&mut out, *protocol_version as u64);
            protocol_core::write_str(&mut out, schema);
            protocol_core::write_hash32(&mut out, pack_schema_hash);
            protocol_core::write_str(&mut out, &actor.0);
            write_opt_str(&mut out, token);
            write_opt_str(&mut out, resume_token);
            write_opt_frontier(&mut out, frontier);
        }
        ClientFrame::Commands { batch_id, envelopes } => {
            out.push(1);
            protocol_core::write_varint_u64(&mut out, *batch_id);
            write_vec_envelope(&mut out, envelopes);
        }
        ClientFrame::FrontierAdvertise { frontier } => {
            out.push(2);
            protocol_causal::encode_frontier(frontier, &mut out);
        }
        ClientFrame::PreviewPublish { key, seq, payload } => {
            out.push(3);
            protocol_core::write_str(&mut out, key);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_bytes(&mut out, payload);
        }
        ClientFrame::Presence { peer } => {
            out.push(4);
            protocol_core::write_bytes(&mut out, peer);
        }
        ClientFrame::CreditGrant { n } => {
            out.push(5);
            protocol_core::write_varint_u64(&mut out, *n as u64);
        }
        ClientFrame::Bye => out.push(6),
    }
    out
}

/// @emoji 📥️ Decodes one `ClientFrame`, returning the `Lane` it was tagged with.
pub fn decode_client_frame(bytes: &[u8]) -> Result<(Lane, ClientFrame), protocol_core::ProtocolError> {
    let lane_byte = *bytes.first().ok_or_else(|| malformed("wire frame", 0, "empty frame"))?;
    let lane = Lane::from_byte(lane_byte).ok_or_else(|| malformed("wire frame lane byte", 0, &format!("unknown lane {lane_byte:#x}")))?;
    let mut pos = 1usize;
    let tag = *bytes.get(pos).ok_or_else(|| malformed("wire client-frame tag", pos as u64, "truncated"))?;
    pos += 1;
    let frame = match tag {
        0 => ClientFrame::Hello {
            wire_version: protocol_core::read_varint_u64(bytes, &mut pos)? as u32,
            protocol_version: protocol_core::read_varint_u64(bytes, &mut pos)? as u32,
            schema: protocol_core::read_str(bytes, &mut pos)?,
            pack_schema_hash: protocol_core::read_hash32(bytes, &mut pos)?,
            actor: protocol_core::ActorId(protocol_core::read_str(bytes, &mut pos)?),
            token: read_opt_str(bytes, &mut pos)?,
            resume_token: read_opt_str(bytes, &mut pos)?,
            frontier: read_opt_frontier(bytes, &mut pos)?,
        },
        1 => ClientFrame::Commands { batch_id: protocol_core::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos)? },
        2 => ClientFrame::FrontierAdvertise { frontier: protocol_causal::decode_frontier(bytes, &mut pos)? },
        3 => ClientFrame::PreviewPublish { key: protocol_core::read_str(bytes, &mut pos)?, seq: protocol_core::read_varint_u64(bytes, &mut pos)?, payload: protocol_core::read_bytes(bytes, &mut pos)? },
        4 => ClientFrame::Presence { peer: protocol_core::read_bytes(bytes, &mut pos)? },
        5 => ClientFrame::CreditGrant { n: protocol_core::read_varint_u64(bytes, &mut pos)? as u32 },
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
            protocol_core::write_str(&mut out, session_id);
            protocol_core::write_str(&mut out, resume_token);
            protocol_causal::encode_frontier(server_frontier, &mut out);
            encode_bootstrap(bootstrap, &mut out);
        }
        ServerFrame::SnapshotChunk { seq, bytes } => {
            out.push(1);
            protocol_core::write_varint_u64(&mut out, *seq as u64);
            protocol_core::write_bytes(&mut out, bytes);
        }
        ServerFrame::SnapshotDone { seq_count } => {
            out.push(2);
            protocol_core::write_varint_u64(&mut out, *seq_count as u64);
        }
        ServerFrame::Commands { envelopes, origin, frontier } => {
            out.push(3);
            write_vec_envelope(&mut out, envelopes);
            protocol_core::write_str(&mut out, &origin.0);
            protocol_causal::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Ack { batch_id, stages, frontier } => {
            out.push(4);
            protocol_core::write_varint_u64(&mut out, *batch_id);
            write_vec_ack_stage(&mut out, stages);
            protocol_causal::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Preview { actor, key, seq, payload } => {
            out.push(5);
            protocol_core::write_str(&mut out, &actor.0);
            protocol_core::write_str(&mut out, key);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_bytes(&mut out, payload);
        }
        ServerFrame::Presence { peers } => {
            out.push(6);
            write_vec_bytes(&mut out, peers);
        }
        ServerFrame::CreditGrant { n } => {
            out.push(7);
            protocol_core::write_varint_u64(&mut out, *n as u64);
        }
        ServerFrame::Error { code, message } => {
            out.push(8);
            protocol_core::write_str(&mut out, code);
            protocol_core::write_str(&mut out, message);
        }
    }
    out
}

/// @emoji 📥️ Decodes one `ServerFrame`, returning the `Lane` it was tagged with.
pub fn decode_server_frame(bytes: &[u8]) -> Result<(Lane, ServerFrame), protocol_core::ProtocolError> {
    let lane_byte = *bytes.first().ok_or_else(|| malformed("wire frame", 0, "empty frame"))?;
    let lane = Lane::from_byte(lane_byte).ok_or_else(|| malformed("wire frame lane byte", 0, &format!("unknown lane {lane_byte:#x}")))?;
    let mut pos = 1usize;
    let tag = *bytes.get(pos).ok_or_else(|| malformed("wire server-frame tag", pos as u64, "truncated"))?;
    pos += 1;
    let frame = match tag {
        0 => ServerFrame::Welcome {
            session_id: protocol_core::read_str(bytes, &mut pos)?,
            resume_token: protocol_core::read_str(bytes, &mut pos)?,
            server_frontier: protocol_causal::decode_frontier(bytes, &mut pos)?,
            bootstrap: decode_bootstrap(bytes, &mut pos)?,
        },
        1 => ServerFrame::SnapshotChunk { seq: protocol_core::read_varint_u64(bytes, &mut pos)? as u32, bytes: protocol_core::read_bytes(bytes, &mut pos)? },
        2 => ServerFrame::SnapshotDone { seq_count: protocol_core::read_varint_u64(bytes, &mut pos)? as u32 },
        3 => ServerFrame::Commands { envelopes: read_vec_envelope(bytes, &mut pos)?, origin: protocol_core::ActorId(protocol_core::read_str(bytes, &mut pos)?), frontier: protocol_causal::decode_frontier(bytes, &mut pos)? },
        4 => ServerFrame::Ack { batch_id: protocol_core::read_varint_u64(bytes, &mut pos)?, stages: read_vec_ack_stage(bytes, &mut pos)?, frontier: protocol_causal::decode_frontier(bytes, &mut pos)? },
        5 => ServerFrame::Preview { actor: protocol_core::ActorId(protocol_core::read_str(bytes, &mut pos)?), key: protocol_core::read_str(bytes, &mut pos)?, seq: protocol_core::read_varint_u64(bytes, &mut pos)?, payload: protocol_core::read_bytes(bytes, &mut pos)? },
        6 => ServerFrame::Presence { peers: read_vec_bytes(bytes, &mut pos)? },
        7 => ServerFrame::CreditGrant { n: protocol_core::read_varint_u64(bytes, &mut pos)? as u32 },
        8 => ServerFrame::Error { code: protocol_core::read_str(bytes, &mut pos)?, message: protocol_core::read_str(bytes, &mut pos)? },
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
    fn sample_envelope(id: &str) -> protocol_causal::OperationEnvelope {
        protocol_causal::OperationEnvelope {
            operation_id: protocol_core::OperationId(id.to_string()),
            document_id: protocol_core::DocumentId("document-1".to_string()),
            actor: protocol_core::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol_causal::DocumentDiff { schema: protocol_core::SchemaId("diff.v1".to_string()), payload: format!("value:{id}").into_bytes() },
            inverse: protocol_causal::InverseOperation { schema: protocol_core::SchemaId("diff.v1".to_string()), payload: Vec::new() },
            timestamp: protocol_core::HybridLogicalTimestamp::new(1, 0),
        }
    }

    fn sample_frontier() -> protocol_causal::FrontierSummary {
        protocol_causal::FrontierSummary {
            document_id: protocol_core::DocumentId("document-1".to_string()),
            head_edit_ordinal: 5,
            head_edit_id: "edit-5".to_string(),
            last_commit_seq: 2,
            chain_hash: [7u8; 32],
        }
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
                actor: protocol_core::ActorId("actor-1".to_string()),
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
            &ClientFrame::Hello {
                wire_version: 1,
                protocol_version: 1,
                schema: "schema.v1".to_string(),
                pack_schema_hash: [0u8; 32],
                actor: protocol_core::ActorId("actor-2".to_string()),
                token: None,
                resume_token: None,
                frontier: None,
            },
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
            assert_server_round_trips(
                &ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: sample_frontier(), bootstrap },
                Lane::Command,
            );
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
        assert_server_round_trips(
            &ServerFrame::Commands { envelopes: vec![sample_envelope("op-1")], origin: protocol_core::ActorId("actor-1".to_string()), frontier: sample_frontier() },
            Lane::Command,
        );
    }

    #[test]
    fn server_frame_ack_round_trips_for_every_stage_and_apply_outcome_variant() {
        for outcome in [ApplyOutcome::Accepted, ApplyOutcome::Transformed { envelope: Box::new(sample_envelope("op-1")) }, ApplyOutcome::Rejected { reason: "conflict".to_string() }] {
            assert_server_round_trips(
                &ServerFrame::Ack { batch_id: 7, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(outcome) }], frontier: sample_frontier() },
                Lane::Command,
            );
        }
    }

    #[test]
    fn server_frame_preview_round_trips() {
        assert_server_round_trips(&ServerFrame::Preview { actor: protocol_core::ActorId("actor-1".to_string()), key: "cursor".to_string(), seq: 3, payload: vec![5, 6] }, Lane::Preview);
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
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "wire frame", .. }));
    }

    #[test]
    fn decode_client_frame_rejects_unknown_lane_byte() {
        let err = decode_client_frame(&[2u8, 0]).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "wire frame lane byte", .. }));
    }

    #[test]
    fn decode_client_frame_rejects_unknown_tag() {
        let bytes = vec![Lane::Command.to_byte(), 0xFF];
        let err = decode_client_frame(&bytes).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "wire client-frame tag", .. }));
    }

    #[test]
    fn decode_server_frame_rejects_unknown_tag() {
        let bytes = vec![Lane::Command.to_byte(), 0xFF];
        let err = decode_server_frame(&bytes).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "wire server-frame tag", .. }));
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
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "wire client-frame tag", .. }));
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
