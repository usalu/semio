//! 🎞️ Protocol semio_hub wire frames: the lane-tagged `ClientFrame`/`ServerFrame` envelopes a
//! browser/native sync client exchanges with the collaboration semio_hub, plus their binary codec. Frozen
//! contract: `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment`
//! §`protocol_wire`.
//!
//! 🎯️ W5: the byte encoding is now a fully hand-rolled binary layout — `lane: u8` followed by
//! `frame tag: u8` (the frame enum's variant declaration order) and its fields in declaration
//! order, with no body-length prefix (one frame per WS message) and no per-field tags. This
//! matches `crate::wire::🔖️WireCodec`'s convention (also used by `crate::causal::🔖️EnvelopeCodec`
//! and `os_dsl::op_rt`). `ArtifactDiff`/`InverseMutation` payloads are opaque `Vec<u8>` (never
//! `serde_json::Value`). `ClientFrame::Presence`/`ServerFrame::Presence` carry opaque presence
//! payload bytes (`peer: Vec<u8>` / `peers: Vec<Vec<u8>>`) — this crate has no dependency on
//! `framework_core` (where the concrete `PresencePeer` type and its binary codec live), so the
//! frame only ever moves the already-encoded blob a caller supplies. `protocol_core` supplies the
//! primitive codec (`write_varint_u64`/`write_str`/`write_bytes`/`write_hash32`/`write_bool` and
//! their `read_*` twins); this crate adds only the option/vec combinators and the frame/nested-enum
//! tag dispatch below.

use std::collections::BTreeMap;

//#region 🔖️Lane
/// @emoji 🛣️ Which logical channel a wire frame travels on: `Command` for causally-ordered,
/// durable operation batches; `Preview` for ephemeral, best-effort UI-state broadcast.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    Command = 0,
    Preview = 1,
}

impl Lane {
    async fn to_byte(self) -> u8 {
        self as u8
    }

    async fn from_byte(byte: u8) -> Option<Self> {
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
    Hello { wire_version: u32, protocol_version: u32, schema: String, pack_schema_hash: [u8; 32], actor: crate::ids::ActorId, token: Option<String>, resume_token: Option<String>, frontier: Option<crate::causal::FrontierSummary> },
    Commands { batch_id: u64, envelopes: Vec<crate::causal::MutationEnvelope> },
    FrontierAdvertise { frontier: crate::causal::FrontierSummary },
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
    Transformed {
        envelope: Box<crate::causal::MutationEnvelope>,
    },
    /// 🧾 `messages` (trailing addition, tag unchanged) is one packed `Vec<MutationMessage>` blob —
    /// opaque here (this crate stays decoupled from `os_spr::command`'s concrete type, matching
    /// `ArtifactDiff`/`InverseMutation`'s opaque-bytes convention above), packed by the caller with
    /// `pack::encode_record_body` before construction. See contract-freeze.md §C8 of
    /// `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
    Rejected {
        reason: String,
        messages: Vec<u8>,
    },
}

/// @emoji 🪜️ One stage of a submitted batch's lifecycle, from `Received` to `Applied`.
#[derive(Clone, Debug, PartialEq)]
pub enum AckStage {
    Received,
    Persisted,
    // 🔒️ Boxed for the same reason as ApplyOutcome::Transformed above.
    Applied { outcome: Box<ApplyOutcome> },
}

pub const SNAPSHOT_CHUNK_BACKING_BYTES: usize = 4 * 1024;

/// @emoji 🧱️ One fixed-capacity snapshot frame backing whose owned allocation can never exceed one wire page.
#[derive(Clone)]
pub struct SnapshotChunkBytes {
    backing: Option<Box<[u8; SNAPSHOT_CHUNK_BACKING_BYTES]>>,
    len: u16,
}

impl SnapshotChunkBytes {
    /// @emoji 🆕️ Allocates exactly one fixed snapshot backing.
    pub fn allocate_fixed() -> Self {
        Self { backing: Some(Box::new([0; SNAPSHOT_CHUNK_BACKING_BYTES])), len: 0 }
    }

    /// @emoji 📥️ Copies one bounded source into a fixed snapshot backing.
    pub fn try_from_slice(source: &[u8]) -> Option<Self> {
        if source.len() > SNAPSHOT_CHUNK_BACKING_BYTES {
            return None;
        }
        let mut owner = Self::allocate_fixed();
        owner.try_extend_from_slice(source).then_some(owner)
    }

    /// @emoji ➕️ Appends bytes only while the fixed backing can retain the complete source.
    pub fn try_extend_from_slice(&mut self, source: &[u8]) -> bool {
        let start = usize::from(self.len);
        let Some(end) = start.checked_add(source.len()).filter(|end| *end <= SNAPSHOT_CHUNK_BACKING_BYTES) else { return false };
        let Some(backing) = self.backing.as_mut() else { return false };
        backing[start..end].copy_from_slice(source);
        self.len = end as u16;
        true
    }

    /// @emoji 📏️ Returns the initialized snapshot byte count.
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    /// @emoji 🩹️ Reports whether the initialized snapshot range is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// @emoji 🧩️ Borrows the initialized snapshot byte range.
    pub fn as_slice(&self) -> &[u8] {
        self.backing.as_ref().map_or(&[], |backing| &backing[..usize::from(self.len)])
    }

    /// @emoji 🧮️ Returns the exact retained fixed backing debit.
    pub fn backing_bytes(&self) -> usize {
        self.backing.as_ref().map_or(0, |backing| std::mem::size_of_val(backing.as_ref()))
    }

    /// @emoji 🧹️ Retires the single fixed backing in one explicit close opportunity.
    pub fn close_one(&mut self) -> bool {
        if self.backing.take().is_none() {
            return false;
        }
        self.len = 0;
        true
    }

    /// @emoji 🏁️ Reports whether the fixed backing owner has been retired.
    pub fn terminal_is_empty(&self) -> bool {
        self.backing.is_none()
    }
}

impl std::fmt::Debug for SnapshotChunkBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple("SnapshotChunkBytes").field(&self.as_slice()).finish()
    }
}

impl PartialEq for SnapshotChunkBytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for SnapshotChunkBytes {}

/// @emoji 📬️ One frame the semio_hub sends to a client.
#[derive(Clone, Debug, PartialEq)]
pub enum ServerFrame {
    Welcome {
        session_id: String,
        resume_token: String,
        server_frontier: crate::causal::FrontierSummary,
        bootstrap: Bootstrap,
    },
    SnapshotChunk {
        seq: u32,
        bytes: SnapshotChunkBytes,
    },
    SnapshotDone {
        seq_count: u32,
    },
    Commands {
        envelopes: Vec<crate::causal::MutationEnvelope>,
        origin: crate::ids::ActorId,
        frontier: crate::causal::FrontierSummary,
    },
    Ack {
        batch_id: u64,
        stages: Vec<AckStage>,
        frontier: crate::causal::FrontierSummary,
    },
    Preview {
        actor: crate::ids::ActorId,
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
    },
    /// @emoji 🎨️ The hub's one-time session assignment for this connection: `color` is the
    /// hub-assigned palette index (`HubState.session_colors`, §C7.3), leased per `(space, actor)` and
    /// stamped by the client actor onto every outbound `PresencePeer` (never filled by a shell). Sent
    /// exactly once per connection, after `Welcome` (and its follow-up bootstrap frames) and before
    /// any `Presence` frame.
    Session {
        actor: String,
        color: u8,
    },
}
//#endregion 🔖️ServerFrame

//#region 🔖️Codec
// Hand-rolled binary frame encode/decode: `lane: u8 | frame tag: u8 | fields...` — see the
// module-level docstring. `crate::wire::🔖️WireCodec` supplies the primitives; this region adds
// the option/vec combinators the frame shapes need plus the tag-dispatch match arms.

async fn malformed(what: &'static str, offset: u64, detail: &str) -> crate::ProtocolError {
    crate::ProtocolError::Malformed { what, offset, detail: detail.to_string() }
}

//#region 🔖️Combinators
async fn write_opt_str(out: &mut Vec<u8>, value: &Option<String>) {
    crate::write_bool(out, value.is_some());
    if let Some(s) = value {
        crate::write_str(out, s);
    }
}

async fn read_opt_str(bytes: &[u8], pos: &mut usize) -> Result<Option<String>, crate::ProtocolError> {
    if crate::read_bool(bytes, pos)? {
        Ok(Some(crate::read_str(bytes, pos)?))
    } else {
        Ok(None)
    }
}

async fn write_opt_bytes(out: &mut Vec<u8>, value: &Option<Vec<u8>>) {
    crate::write_bool(out, value.is_some());
    if let Some(b) = value {
        crate::write_bytes(out, b);
    }
}

async fn read_opt_bytes(bytes: &[u8], pos: &mut usize) -> Result<Option<Vec<u8>>, crate::ProtocolError> {
    if crate::read_bool(bytes, pos)? {
        Ok(Some(crate::read_bytes(bytes, pos)?))
    } else {
        Ok(None)
    }
}

async fn write_opt_frontier(out: &mut Vec<u8>, value: &Option<crate::causal::FrontierSummary>) {
    crate::write_bool(out, value.is_some());
    if let Some(f) = value {
        crate::causal::encode_frontier(f, out);
    }
}

async fn read_opt_frontier(bytes: &[u8], pos: &mut usize) -> Result<Option<crate::causal::FrontierSummary>, crate::ProtocolError> {
    if crate::read_bool(bytes, pos)? {
        Ok(Some(crate::causal::decode_frontier(bytes, pos)?))
    } else {
        Ok(None)
    }
}

async fn write_vec_bytes(out: &mut Vec<u8>, values: &[Vec<u8>]) {
    crate::wire::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::write_bytes(out, value);
    }
}

async fn read_vec_bytes(bytes: &[u8], pos: &mut usize) -> Result<Vec<Vec<u8>>, crate::ProtocolError> {
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(crate::read_bytes(bytes, pos)?);
    }
    Ok(out)
}

async fn write_vec_envelope(out: &mut Vec<u8>, values: &[crate::causal::MutationEnvelope]) {
    crate::wire::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::causal::encode_envelope(value, out);
    }
}

async fn read_vec_envelope(bytes: &[u8], pos: &mut usize) -> Result<Vec<crate::causal::MutationEnvelope>, crate::ProtocolError> {
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(crate::causal::decode_envelope(bytes, pos)?);
    }
    Ok(out)
}
//#endregion 🔖️Combinators

//#region 🔖️NestedEnums
async fn encode_bootstrap(bootstrap: &Bootstrap, out: &mut Vec<u8>) {
    match bootstrap {
        Bootstrap::None => out.push(0),
        Bootstrap::Snapshot { pack_hash, inline } => {
            out.push(1);
            crate::write_hash32(out, pack_hash);
            write_opt_bytes(out, inline).await;
        }
        Bootstrap::Tail => out.push(2),
    }
}

async fn decode_bootstrap(bytes: &[u8], pos: &mut usize) -> Result<Bootstrap, crate::ProtocolError> {
    let tag = match bytes.get(*pos) {
        Some(b) => *b,
        None => return Err(malformed("wire bootstrap tag", *pos as u64, "truncated").await),
    };
    *pos += 1;
    match tag {
        0 => Ok(Bootstrap::None),
        1 => {
            let pack_hash = crate::read_hash32(bytes, pos)?;
            let inline = read_opt_bytes(bytes, pos).await?;
            Ok(Bootstrap::Snapshot { pack_hash, inline })
        }
        2 => Ok(Bootstrap::Tail),
        other => Err(malformed("wire bootstrap tag", *pos as u64, &format!("unknown tag {other:#x}")).await),
    }
}

async fn encode_apply_outcome(outcome: &ApplyOutcome, out: &mut Vec<u8>) {
    match outcome {
        ApplyOutcome::Accepted => out.push(0),
        ApplyOutcome::Transformed { envelope } => {
            out.push(1);
            crate::causal::encode_envelope(envelope, out);
        }
        ApplyOutcome::Rejected { reason, messages } => {
            out.push(2);
            crate::write_str(out, reason);
            crate::write_bytes(out, messages);
        }
    }
}

async fn decode_apply_outcome(bytes: &[u8], pos: &mut usize) -> Result<ApplyOutcome, crate::ProtocolError> {
    let tag = match bytes.get(*pos) {
        Some(b) => *b,
        None => return Err(malformed("wire apply-outcome tag", *pos as u64, "truncated").await),
    };
    *pos += 1;
    match tag {
        0 => Ok(ApplyOutcome::Accepted),
        1 => Ok(ApplyOutcome::Transformed { envelope: Box::new(crate::causal::decode_envelope(bytes, pos)?) }),
        2 => Ok(ApplyOutcome::Rejected { reason: crate::read_str(bytes, pos)?, messages: crate::read_bytes(bytes, pos)? }),
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

async fn decode_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<AckStage, crate::ProtocolError> {
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

async fn write_vec_ack_stage(out: &mut Vec<u8>, values: &[AckStage]) {
    crate::wire::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_ack_stage(value, out).await;
    }
}

async fn read_vec_ack_stage(bytes: &[u8], pos: &mut usize) -> Result<Vec<AckStage>, crate::ProtocolError> {
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(decode_ack_stage(bytes, pos).await?);
    }
    Ok(out)
}
//#endregion 🔖️NestedEnums

/// @emoji 📤️ Encodes one `ClientFrame` on the given `Lane`: `lane u8 | tag u8 | fields`.
pub async fn encode_client_frame(frame: &ClientFrame, lane: Lane) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(lane.to_byte().await);
    match frame {
        ClientFrame::Hello { wire_version, protocol_version, schema, pack_schema_hash, actor, token, resume_token, frontier } => {
            out.push(0);
            crate::wire::write_varint_u64(&mut out, *wire_version as u64);
            crate::wire::write_varint_u64(&mut out, *protocol_version as u64);
            crate::write_str(&mut out, schema);
            crate::write_hash32(&mut out, pack_schema_hash);
            crate::write_str(&mut out, &actor.0);
            write_opt_str(&mut out, token).await;
            write_opt_str(&mut out, resume_token).await;
            write_opt_frontier(&mut out, frontier).await;
        }
        ClientFrame::Commands { batch_id, envelopes } => {
            out.push(1);
            crate::wire::write_varint_u64(&mut out, *batch_id);
            write_vec_envelope(&mut out, envelopes).await;
        }
        ClientFrame::FrontierAdvertise { frontier } => {
            out.push(2);
            crate::causal::encode_frontier(frontier, &mut out);
        }
        ClientFrame::PreviewPublish { key, seq, payload } => {
            out.push(3);
            crate::write_str(&mut out, key);
            crate::wire::write_varint_u64(&mut out, *seq);
            crate::write_bytes(&mut out, payload);
        }
        ClientFrame::Presence { peer } => {
            out.push(4);
            crate::write_bytes(&mut out, peer);
        }
        ClientFrame::CreditGrant { n } => {
            out.push(5);
            crate::wire::write_varint_u64(&mut out, *n as u64);
        }
        ClientFrame::Bye => out.push(6),
    }
    out
}

/// @emoji 📥️ Decodes one `ClientFrame`, returning the `Lane` it was tagged with.
pub async fn decode_client_frame(bytes: &[u8]) -> Result<(Lane, ClientFrame), crate::ProtocolError> {
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
        0 => ClientFrame::Hello {
            wire_version: crate::wire::read_varint_u64(bytes, &mut pos)? as u32,
            protocol_version: crate::wire::read_varint_u64(bytes, &mut pos)? as u32,
            schema: crate::read_str(bytes, &mut pos)?,
            pack_schema_hash: crate::read_hash32(bytes, &mut pos)?,
            actor: crate::ids::ActorId(crate::read_str(bytes, &mut pos)?),
            token: read_opt_str(bytes, &mut pos).await?,
            resume_token: read_opt_str(bytes, &mut pos).await?,
            frontier: read_opt_frontier(bytes, &mut pos).await?,
        },
        1 => ClientFrame::Commands { batch_id: crate::wire::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos).await? },
        2 => ClientFrame::FrontierAdvertise { frontier: crate::causal::decode_frontier(bytes, &mut pos)? },
        3 => ClientFrame::PreviewPublish { key: crate::read_str(bytes, &mut pos)?, seq: crate::wire::read_varint_u64(bytes, &mut pos)?, payload: crate::read_bytes(bytes, &mut pos)? },
        4 => ClientFrame::Presence { peer: crate::read_bytes(bytes, &mut pos)? },
        5 => ClientFrame::CreditGrant { n: crate::wire::read_varint_u64(bytes, &mut pos)? as u32 },
        6 => ClientFrame::Bye,
        other => return Err(malformed("wire client-frame tag", pos as u64, &format!("unknown tag {other:#x}")).await),
    };
    Ok((lane, frame))
}

/// @emoji 📤️ Encodes one `ServerFrame` on the given `Lane`: `lane u8 | tag u8 | fields`.
pub async fn encode_server_frame(frame: &ServerFrame, lane: Lane) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(lane.to_byte().await);
    match frame {
        ServerFrame::Welcome { session_id, resume_token, server_frontier, bootstrap } => {
            out.push(0);
            crate::write_str(&mut out, session_id);
            crate::write_str(&mut out, resume_token);
            crate::causal::encode_frontier(server_frontier, &mut out);
            encode_bootstrap(bootstrap, &mut out).await;
        }
        ServerFrame::SnapshotChunk { seq, bytes } => {
            out.push(1);
            crate::wire::write_varint_u64(&mut out, *seq as u64);
            crate::write_bytes(&mut out, bytes.as_slice());
        }
        ServerFrame::SnapshotDone { seq_count } => {
            out.push(2);
            crate::wire::write_varint_u64(&mut out, *seq_count as u64);
        }
        ServerFrame::Commands { envelopes, origin, frontier } => {
            out.push(3);
            write_vec_envelope(&mut out, envelopes).await;
            crate::write_str(&mut out, &origin.0);
            crate::causal::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Ack { batch_id, stages, frontier } => {
            out.push(4);
            crate::wire::write_varint_u64(&mut out, *batch_id);
            write_vec_ack_stage(&mut out, stages).await;
            crate::causal::encode_frontier(frontier, &mut out);
        }
        ServerFrame::Preview { actor, key, seq, payload } => {
            out.push(5);
            crate::write_str(&mut out, &actor.0);
            crate::write_str(&mut out, key);
            crate::wire::write_varint_u64(&mut out, *seq);
            crate::write_bytes(&mut out, payload);
        }
        ServerFrame::Presence { peers } => {
            out.push(6);
            write_vec_bytes(&mut out, peers).await;
        }
        ServerFrame::CreditGrant { n } => {
            out.push(7);
            crate::wire::write_varint_u64(&mut out, *n as u64);
        }
        ServerFrame::Error { code, message } => {
            out.push(8);
            crate::write_str(&mut out, code);
            crate::write_str(&mut out, message);
        }
        ServerFrame::Session { actor, color } => {
            out.push(9);
            crate::write_str(&mut out, actor);
            out.push(*color);
        }
    }
    out
}

fn read_snapshot_chunk_bytes(bytes: &[u8], pos: &mut usize) -> Result<SnapshotChunkBytes, crate::ProtocolError> {
    let len = crate::wire::read_varint_u64(bytes, pos)?;
    if len > SNAPSHOT_CHUNK_BACKING_BYTES as u64 {
        return Err(crate::ProtocolError::LimitExceeded("snapshot chunk fixed backing"));
    }
    let len = len as usize;
    let end = pos.checked_add(len).ok_or(crate::ProtocolError::Malformed { what: "wire snapshot chunk", offset: *pos as u64, detail: "length overflow".to_string() })?;
    let source = bytes.get(*pos..end).ok_or(crate::ProtocolError::Malformed { what: "wire snapshot chunk", offset: *pos as u64, detail: "truncated".to_string() })?;
    let owner = SnapshotChunkBytes::try_from_slice(source).ok_or(crate::ProtocolError::LimitExceeded("snapshot chunk fixed backing"))?;
    *pos = end;
    Ok(owner)
}

/// @emoji 📥️ Decodes one `ServerFrame`, returning the `Lane` it was tagged with.
pub async fn decode_server_frame(bytes: &[u8]) -> Result<(Lane, ServerFrame), crate::ProtocolError> {
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
            session_id: crate::read_str(bytes, &mut pos)?,
            resume_token: crate::read_str(bytes, &mut pos)?,
            server_frontier: crate::causal::decode_frontier(bytes, &mut pos)?,
            bootstrap: decode_bootstrap(bytes, &mut pos).await?,
        },
        1 => ServerFrame::SnapshotChunk { seq: crate::wire::read_varint_u64(bytes, &mut pos)? as u32, bytes: read_snapshot_chunk_bytes(bytes, &mut pos)? },
        2 => ServerFrame::SnapshotDone { seq_count: crate::wire::read_varint_u64(bytes, &mut pos)? as u32 },
        3 => ServerFrame::Commands { envelopes: read_vec_envelope(bytes, &mut pos).await?, origin: crate::ids::ActorId(crate::read_str(bytes, &mut pos)?), frontier: crate::causal::decode_frontier(bytes, &mut pos)? },
        4 => ServerFrame::Ack { batch_id: crate::wire::read_varint_u64(bytes, &mut pos)?, stages: read_vec_ack_stage(bytes, &mut pos).await?, frontier: crate::causal::decode_frontier(bytes, &mut pos)? },
        5 => ServerFrame::Preview { actor: crate::ids::ActorId(crate::read_str(bytes, &mut pos)?), key: crate::read_str(bytes, &mut pos)?, seq: crate::wire::read_varint_u64(bytes, &mut pos)?, payload: crate::read_bytes(bytes, &mut pos)? },
        6 => ServerFrame::Presence { peers: read_vec_bytes(bytes, &mut pos).await? },
        7 => ServerFrame::CreditGrant { n: crate::wire::read_varint_u64(bytes, &mut pos)? as u32 },
        8 => ServerFrame::Error { code: crate::read_str(bytes, &mut pos)?, message: crate::read_str(bytes, &mut pos)? },
        9 => {
            let actor = crate::read_str(bytes, &mut pos)?;
            let color = crate::read_u8(bytes, &mut pos)?;
            ServerFrame::Session { actor, color }
        }
        other => return Err(malformed("wire server-frame tag", pos as u64, &format!("unknown tag {other:#x}")).await),
    };
    Ok((lane, frame))
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    async fn sample_envelope(id: &str) -> crate::causal::MutationEnvelope {
        crate::causal::MutationEnvelope {
            mutation_id: crate::ids::MutationId(id.to_string()),
            document_id: crate::ids::ArtifactId("document-1".to_string()),
            actor: crate::ids::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: crate::causal::ArtifactDiff { schema: crate::ids::SchemaId("diff.v1".to_string()), payload: format!("value:{id}").into_bytes() },
            inverse: crate::causal::InverseMutation { schema: crate::ids::SchemaId("diff.v1".to_string()), payload: Vec::new() },
            timestamp: crate::ids::HybridLogicalTimestamp::new(1, 0),
        }
    }

    async fn sample_frontier() -> crate::causal::FrontierSummary {
        crate::causal::FrontierSummary { document_id: crate::ids::ArtifactId("document-1".to_string()), head_edit_ordinal: 5, head_edit_id: "edit-5".to_string(), last_commit_seq: 2, chain_hash: [7u8; 32] }
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️Lane
    #[semio_framework_async_macros::async_test]
    async fn lane_byte_round_trips() {
        assert_eq!(Lane::from_byte(Lane::Command.to_byte().await).await, Some(Lane::Command));
        assert_eq!(Lane::from_byte(Lane::Preview.to_byte().await).await, Some(Lane::Preview));
        assert_eq!(Lane::from_byte(2).await, None);
    }
    //#endregion 🔖️Lane

    //#region 🔖️ClientFrame
    async fn assert_client_round_trips(frame: &ClientFrame, lane: Lane) {
        let bytes = encode_client_frame(frame, lane).await;
        let (decoded_lane, decoded_frame) = decode_client_frame(&bytes).await.expect("decode must succeed");
        assert_eq!(decoded_lane, lane);
        assert_eq!(&decoded_frame, frame);
    }

    #[semio_framework_async_macros::async_test]
    async fn client_frame_hello_round_trips() {
        assert_client_round_trips(
            &ClientFrame::Hello {
                wire_version: 1,
                protocol_version: 1,
                schema: "schema.v1".to_string(),
                pack_schema_hash: [1u8; 32],
                actor: crate::ids::ActorId("actor-1".to_string()),
                token: Some("token".to_string()),
                resume_token: None,
                frontier: Some(sample_frontier().await),
            },
            Lane::Command,
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn client_frame_hello_with_no_optionals_round_trips() {
        assert_client_round_trips(
            &ClientFrame::Hello { wire_version: 1, protocol_version: 1, schema: "schema.v1".to_string(), pack_schema_hash: [0u8; 32], actor: crate::ids::ActorId("actor-2".to_string()), token: None, resume_token: None, frontier: None },
            Lane::Command,
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn client_frame_commands_round_trips() {
        assert_client_round_trips(&ClientFrame::Commands { batch_id: 42, envelopes: vec![sample_envelope("op-1").await, sample_envelope("op-2").await] }, Lane::Command).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn client_frame_frontier_advertise_round_trips() {
        assert_client_round_trips(&ClientFrame::FrontierAdvertise { frontier: sample_frontier().await }, Lane::Command).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn client_frame_preview_publish_round_trips() {
        assert_client_round_trips(&ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn client_frame_presence_round_trips() {
        assert_client_round_trips(&ClientFrame::Presence { peer: b"{\"cursor\":[1,2]}".to_vec() }, Lane::Preview).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn client_frame_credit_grant_round_trips() {
        assert_client_round_trips(&ClientFrame::CreditGrant { n: 16 }, Lane::Command).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn client_frame_bye_round_trips() {
        assert_client_round_trips(&ClientFrame::Bye, Lane::Command).await;
    }
    //#endregion 🔖️ClientFrame

    //#region 🔖️ServerFrame
    async fn assert_server_round_trips(frame: &ServerFrame, lane: Lane) {
        let bytes = encode_server_frame(frame, lane).await;
        let (decoded_lane, decoded_frame) = decode_server_frame(&bytes).await.expect("decode must succeed");
        assert_eq!(decoded_lane, lane);
        assert_eq!(&decoded_frame, frame);
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_welcome_round_trips_for_every_bootstrap_variant() {
        for bootstrap in [Bootstrap::None, Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: Some(vec![9, 9]) }, Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: None }, Bootstrap::Tail] {
            assert_server_round_trips(&ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: sample_frontier().await, bootstrap }, Lane::Command).await;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_snapshot_chunk_round_trips() {
        assert_server_round_trips(&ServerFrame::SnapshotChunk { seq: 0, bytes: SnapshotChunkBytes::try_from_slice(&[1, 2, 3, 4]).unwrap() }, Lane::Command).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_snapshot_done_round_trips() {
        assert_server_round_trips(&ServerFrame::SnapshotDone { seq_count: 4 }, Lane::Command).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_commands_round_trips() {
        assert_server_round_trips(&ServerFrame::Commands { envelopes: vec![sample_envelope("op-1").await], origin: crate::ids::ActorId("actor-1".to_string()), frontier: sample_frontier().await }, Lane::Command).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_ack_round_trips_for_every_stage_and_apply_outcome_variant() {
        for outcome in [ApplyOutcome::Accepted, ApplyOutcome::Transformed { envelope: Box::new(sample_envelope("op-1").await) }, ApplyOutcome::Rejected { reason: "conflict".to_string(), messages: vec![1, 2] }] {
            assert_server_round_trips(&ServerFrame::Ack { batch_id: 7, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(outcome) }], frontier: sample_frontier().await }, Lane::Command).await;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_preview_round_trips() {
        assert_server_round_trips(&ServerFrame::Preview { actor: crate::ids::ActorId("actor-1".to_string()), key: "cursor".to_string(), seq: 3, payload: vec![5, 6] }, Lane::Preview).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_presence_round_trips() {
        assert_server_round_trips(&ServerFrame::Presence { peers: vec![b"{\"id\":\"a\"}".to_vec(), b"{\"id\":\"b\"}".to_vec()] }, Lane::Preview).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_credit_grant_round_trips() {
        assert_server_round_trips(&ServerFrame::CreditGrant { n: 32 }, Lane::Command).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_error_round_trips() {
        assert_server_round_trips(&ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn server_frame_session_round_trips() {
        assert_server_round_trips(&ServerFrame::Session { actor: "actor-1".to_string(), color: 7 }, Lane::Command).await;
    }
    //#endregion 🔖️ServerFrame

    //#region 🔖️Codec
    #[semio_framework_async_macros::async_test]
    async fn decode_client_frame_rejects_empty_bytes() {
        let err = decode_client_frame(&[]).await.unwrap_err();
        assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire frame", .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_client_frame_rejects_unknown_lane_byte() {
        let err = decode_client_frame(&[2u8, 0]).await.unwrap_err();
        assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire frame lane byte", .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_client_frame_rejects_unknown_tag() {
        let bytes = vec![Lane::Command.to_byte().await, 0xFF];
        let err = decode_client_frame(&bytes).await.unwrap_err();
        assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire client-frame tag", .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_server_frame_rejects_unknown_tag() {
        let bytes = vec![Lane::Command.to_byte().await, 0xFF];
        let err = decode_server_frame(&bytes).await.unwrap_err();
        assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire server-frame tag", .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_client_frame_rejects_truncated_field() {
        let bytes = encode_client_frame(&ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview).await;
        let truncated = &bytes[..bytes.len() - 2];
        assert!(decode_client_frame(truncated).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_server_frame_rejects_truncated_field() {
        let bytes = encode_server_frame(&ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command).await;
        let truncated = &bytes[..bytes.len() - 3];
        assert!(decode_server_frame(truncated).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_client_frame_rejects_empty_body_after_lane() {
        let err = decode_client_frame(&[Lane::Command.to_byte().await]).await.unwrap_err();
        assert!(matches!(err, crate::ProtocolError::Malformed { what: "wire client-frame tag", .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn different_lanes_produce_different_leading_bytes_but_same_body() {
        let command_bytes = encode_client_frame(&ClientFrame::Bye, Lane::Command).await;
        let preview_bytes = encode_client_frame(&ClientFrame::Bye, Lane::Preview).await;
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
// longer fits. 🎯️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION
// C7.1: `PresencePoint`/`PresenceViewport` and `PresencePeer.cursor`/`.viewport` are DELETED —
// replaced by `views: Vec<PresenceWindowView>` (one entry per open window/surface, artifact-scope,
// matched by `space`) plus `ui: Option<PresenceUi>` (app-scope `data-ui-path` hover/focus/press).
//#region 🔖️PresenceView
/// @emoji 🪟️ One open window/surface's live view for a document — camera/pan-zoom plus in-view
/// pointer, broadcast so peers can render each other's viewport rectangles / camera frustums /
/// cursor markers. `window_id` disambiguates multiple windows viewing the same space; `space` is
/// the coordinate-space id the surface host reports (`"world"`/`"canvas"`/`"geo"`, or an app-declared
/// finer id) — an overlay renders a peer view only in local surfaces with the same `space`.
#[derive(Clone, Debug, PartialEq)]
pub struct PresenceWindowView {
    pub window_id: String,
    pub space: String,
    pub kind: PresenceViewKind,
    /// @emoji 📐️ The reporting surface's pixel size — needed to draw a peer's viewport rectangle.
    pub size: [f64; 2],
    /// @emoji 📍️ In view coordinates: world point (Orbit), `[x, y, 0]` canvas point (Canvas),
    /// `[lng, lat, 0]` (Geo).
    pub pointer: Option<[f64; 3]>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. The real wire path is
/// `🔖️PresenceViewCodec` below (a hand-rolled binary codec); this exists for capability parity.
impl crate::value::ToValue for PresenceWindowView {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![
            ("windowId".to_string(), crate::value::ToValue::to_value(&self.window_id)),
            ("space".to_string(), crate::value::ToValue::to_value(&self.space)),
            ("kind".to_string(), crate::value::ToValue::to_value(&self.kind)),
            ("size".to_string(), crate::value::ToValue::to_value(&self.size)),
        ];
        if self.pointer.is_some() {
            entries.push(("pointer".to_string(), crate::value::ToValue::to_value(&self.pointer)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for PresenceWindowView {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for PresenceWindowView, found {value:?}")));
        };
        let mut window_id = None;
        let mut space = None;
        let mut kind = None;
        let mut size = None;
        let mut pointer = None;
        for (key, entry) in fields {
            match key.as_str() {
                "windowId" => window_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("windowId"))?),
                "space" => space = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("space"))?),
                "kind" => kind = Some(<PresenceViewKind as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("kind"))?),
                "size" => size = Some(<[f64; 2] as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("size"))?),
                "pointer" => pointer = <Option<[f64; 3]> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("pointer"))?,
                _ => {}
            }
        }
        Ok(PresenceWindowView {
            window_id: window_id.ok_or_else(|| crate::value::ValueError::new("PresenceWindowView missing windowId"))?,
            space: space.ok_or_else(|| crate::value::ValueError::new("PresenceWindowView missing space"))?,
            kind: kind.ok_or_else(|| crate::value::ValueError::new("PresenceWindowView missing kind"))?,
            size: size.ok_or_else(|| crate::value::ValueError::new("PresenceWindowView missing size"))?,
            pointer,
        })
    }
}

/// @emoji 🎥️ A peer's live camera/pan-zoom, tagged by surface family.
#[derive(Clone, Debug, PartialEq)]
pub enum PresenceViewKind {
    Canvas { x: f64, y: f64, zoom: f64 },
    Orbit { position: [f64; 3], target: [f64; 3], up: [f64; 3], fov: f64 },
    Geo { lng: f64, lat: f64, zoom: f64, bearing: f64, pitch: f64 },
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. Internally tagged on
/// `"kind"`, mirroring `#[serde(tag = "kind", rename_all = "camelCase")]` byte-for-byte.
impl crate::value::ToValue for PresenceViewKind {
    fn to_value(&self) -> crate::value::DslValue {
        match self {
            PresenceViewKind::Canvas { x, y, zoom } => crate::value::DslValue::object(vec![
                ("kind".to_string(), crate::value::DslValue::String("canvas".to_string())),
                ("x".to_string(), crate::value::ToValue::to_value(x)),
                ("y".to_string(), crate::value::ToValue::to_value(y)),
                ("zoom".to_string(), crate::value::ToValue::to_value(zoom)),
            ]),
            PresenceViewKind::Orbit { position, target, up, fov } => crate::value::DslValue::object(vec![
                ("kind".to_string(), crate::value::DslValue::String("orbit".to_string())),
                ("position".to_string(), crate::value::ToValue::to_value(position)),
                ("target".to_string(), crate::value::ToValue::to_value(target)),
                ("up".to_string(), crate::value::ToValue::to_value(up)),
                ("fov".to_string(), crate::value::ToValue::to_value(fov)),
            ]),
            PresenceViewKind::Geo { lng, lat, zoom, bearing, pitch } => crate::value::DslValue::object(vec![
                ("kind".to_string(), crate::value::DslValue::String("geo".to_string())),
                ("lng".to_string(), crate::value::ToValue::to_value(lng)),
                ("lat".to_string(), crate::value::ToValue::to_value(lat)),
                ("zoom".to_string(), crate::value::ToValue::to_value(zoom)),
                ("bearing".to_string(), crate::value::ToValue::to_value(bearing)),
                ("pitch".to_string(), crate::value::ToValue::to_value(pitch)),
            ]),
        }
    }
}
impl crate::value::FromValue for PresenceViewKind {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for PresenceViewKind, found {value:?}")));
        };
        let get = |key: &str| fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let kind = match get("kind") {
            Some(crate::value::DslValue::String(s)) => s,
            _ => return Err(crate::value::ValueError::new("PresenceViewKind missing kind")),
        };
        match kind.as_str() {
            "canvas" => Ok(PresenceViewKind::Canvas {
                x: <f64 as crate::value::FromValue>::from_value(get("x").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.canvas missing x"))?).map_err(|e| e.under("x"))?,
                y: <f64 as crate::value::FromValue>::from_value(get("y").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.canvas missing y"))?).map_err(|e| e.under("y"))?,
                zoom: <f64 as crate::value::FromValue>::from_value(get("zoom").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.canvas missing zoom"))?).map_err(|e| e.under("zoom"))?,
            }),
            "orbit" => Ok(PresenceViewKind::Orbit {
                position: <[f64; 3] as crate::value::FromValue>::from_value(get("position").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.orbit missing position"))?).map_err(|e| e.under("position"))?,
                target: <[f64; 3] as crate::value::FromValue>::from_value(get("target").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.orbit missing target"))?).map_err(|e| e.under("target"))?,
                up: <[f64; 3] as crate::value::FromValue>::from_value(get("up").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.orbit missing up"))?).map_err(|e| e.under("up"))?,
                fov: <f64 as crate::value::FromValue>::from_value(get("fov").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.orbit missing fov"))?).map_err(|e| e.under("fov"))?,
            }),
            "geo" => Ok(PresenceViewKind::Geo {
                lng: <f64 as crate::value::FromValue>::from_value(get("lng").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.geo missing lng"))?).map_err(|e| e.under("lng"))?,
                lat: <f64 as crate::value::FromValue>::from_value(get("lat").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.geo missing lat"))?).map_err(|e| e.under("lat"))?,
                zoom: <f64 as crate::value::FromValue>::from_value(get("zoom").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.geo missing zoom"))?).map_err(|e| e.under("zoom"))?,
                bearing: <f64 as crate::value::FromValue>::from_value(get("bearing").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.geo missing bearing"))?).map_err(|e| e.under("bearing"))?,
                pitch: <f64 as crate::value::FromValue>::from_value(get("pitch").ok_or_else(|| crate::value::ValueError::new("PresenceViewKind.geo missing pitch"))?).map_err(|e| e.under("pitch"))?,
            }),
            other => Err(crate::value::ValueError::new(format!("unknown PresenceViewKind kind `{other}`"))),
        }
    }
}

/// @emoji 🖱️ A peer's live `data-ui-path` hover/focus/press state (APP scope) — the grammar
/// `type[idx]#id/...`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PresenceUi {
    pub hovered_path: Option<String>,
    pub focused_path: Option<String>,
    pub pressed_path: Option<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for PresenceUi {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = Vec::new();
        if self.hovered_path.is_some() {
            entries.push(("hoveredPath".to_string(), crate::value::ToValue::to_value(&self.hovered_path)));
        }
        if self.focused_path.is_some() {
            entries.push(("focusedPath".to_string(), crate::value::ToValue::to_value(&self.focused_path)));
        }
        if self.pressed_path.is_some() {
            entries.push(("pressedPath".to_string(), crate::value::ToValue::to_value(&self.pressed_path)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for PresenceUi {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for PresenceUi, found {value:?}")));
        };
        let mut out = PresenceUi::default();
        for (key, entry) in fields {
            match key.as_str() {
                "hoveredPath" => out.hovered_path = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("hoveredPath"))?,
                "focusedPath" => out.focused_path = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("focusedPath"))?,
                "pressedPath" => out.pressed_path = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("pressedPath"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

//#region 🔖️PresenceViewCodec
async fn encode_presence_view_kind(kind: &PresenceViewKind, out: &mut Vec<u8>) {
    match kind {
        PresenceViewKind::Canvas { x, y, zoom } => {
            out.push(0);
            crate::write_f64(out, *x);
            crate::write_f64(out, *y);
            crate::write_f64(out, *zoom);
        }
        PresenceViewKind::Orbit { position, target, up, fov } => {
            out.push(1);
            for value in position.iter().chain(target.iter()).chain(up.iter()) {
                crate::write_f64(out, *value);
            }
            crate::write_f64(out, *fov);
        }
        PresenceViewKind::Geo { lng, lat, zoom, bearing, pitch } => {
            out.push(2);
            crate::write_f64(out, *lng);
            crate::write_f64(out, *lat);
            crate::write_f64(out, *zoom);
            crate::write_f64(out, *bearing);
            crate::write_f64(out, *pitch);
        }
    }
}

async fn decode_presence_view_kind(bytes: &[u8], pos: &mut usize) -> Result<PresenceViewKind, crate::ProtocolError> {
    let tag = match bytes.get(*pos) {
        Some(b) => *b,
        None => return Err(malformed("presence view kind tag", *pos as u64, "truncated").await),
    };
    *pos += 1;
    match tag {
        0 => Ok(PresenceViewKind::Canvas { x: crate::read_f64(bytes, pos)?, y: crate::read_f64(bytes, pos)?, zoom: crate::read_f64(bytes, pos)? }),
        1 => {
            async fn read3(bytes: &[u8], pos: &mut usize) -> Result<[f64; 3], crate::ProtocolError> {
                Ok([crate::read_f64(bytes, pos)?, crate::read_f64(bytes, pos)?, crate::read_f64(bytes, pos)?])
            }
            let position = read3(bytes, pos).await?;
            let target = read3(bytes, pos).await?;
            let up = read3(bytes, pos).await?;
            let fov = crate::read_f64(bytes, pos)?;
            Ok(PresenceViewKind::Orbit { position, target, up, fov })
        }
        2 => Ok(PresenceViewKind::Geo { lng: crate::read_f64(bytes, pos)?, lat: crate::read_f64(bytes, pos)?, zoom: crate::read_f64(bytes, pos)?, bearing: crate::read_f64(bytes, pos)?, pitch: crate::read_f64(bytes, pos)? }),
        other => Err(malformed("presence view kind tag", *pos as u64, &format!("unknown tag {other:#x}")).await),
    }
}

async fn encode_presence_window_view(view: &PresenceWindowView, out: &mut Vec<u8>) {
    crate::write_str(out, &view.window_id);
    crate::write_str(out, &view.space);
    encode_presence_view_kind(&view.kind, out).await;
    crate::write_f64(out, view.size[0]);
    crate::write_f64(out, view.size[1]);
    crate::write_bool(out, view.pointer.is_some());
    if let Some(pointer) = view.pointer {
        for value in pointer {
            crate::write_f64(out, value);
        }
    }
}

async fn decode_presence_window_view(bytes: &[u8], pos: &mut usize) -> Result<PresenceWindowView, crate::ProtocolError> {
    let window_id = crate::read_str(bytes, pos)?;
    let space = crate::read_str(bytes, pos)?;
    let kind = decode_presence_view_kind(bytes, pos).await?;
    let size = [crate::read_f64(bytes, pos)?, crate::read_f64(bytes, pos)?];
    let pointer = if crate::read_bool(bytes, pos)? { Some([crate::read_f64(bytes, pos)?, crate::read_f64(bytes, pos)?, crate::read_f64(bytes, pos)?]) } else { None };
    Ok(PresenceWindowView { window_id, space, kind, size, pointer })
}

async fn write_vec_presence_window_view(out: &mut Vec<u8>, values: &[PresenceWindowView]) {
    crate::wire::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_presence_window_view(value, out).await;
    }
}

async fn read_vec_presence_window_view(bytes: &[u8], pos: &mut usize) -> Result<Vec<PresenceWindowView>, crate::ProtocolError> {
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(decode_presence_window_view(bytes, pos).await?);
    }
    Ok(out)
}

async fn encode_presence_ui(ui: &PresenceUi, out: &mut Vec<u8>) {
    write_opt_str(out, &ui.hovered_path).await;
    write_opt_str(out, &ui.focused_path).await;
    write_opt_str(out, &ui.pressed_path).await;
}

async fn decode_presence_ui(bytes: &[u8], pos: &mut usize) -> Result<PresenceUi, crate::ProtocolError> {
    Ok(PresenceUi { hovered_path: read_opt_str(bytes, pos).await?, focused_path: read_opt_str(bytes, pos).await?, pressed_path: read_opt_str(bytes, pos).await? })
}
//#endregion 🔖️PresenceViewCodec
//#endregion 🔖️PresenceView

// 🌱️ `presence_pack_serde` (the former `#[serde(with = "…")]` base64 shim for `PresencePeer.
// presence_pack`) is deleted: `PresencePeer::to_value`/`from_value` above inline the identical
// base64-string encoding directly, and nothing else referenced this module.

/// @emoji 📡️ Presence roster entry broadcast to every peer connected to a document.
///
/// `presence_pack` carries the app's typed `ArtifactApp::Presence` encoded through `ArtifactPack`.
/// When serialised for `ViewModel.presence_peers_json`, that pack is base64-encoded under the
/// camelCase key `presencePack` (this layer has no app-specific `ArtifactPack` decoder, so the
/// renderer JSON contract keeps the opaque pack rather than a decoded `presence` object).
#[derive(Clone, Debug, PartialEq)]
pub struct PresencePeer {
    pub actor: String,
    pub connected_at_ms: i64,
    pub label: Option<String>,
    /// @emoji 👥️ App-typed presence encoded as `ArtifactPack` bytes (flag bit 1 on the wire, APP scope).
    pub presence_pack: Option<Vec<u8>>,
    /// @emoji 🪪️ Authenticated hub user id, when this peer connected with an `AuthSession` rather than an anonymous share token.
    pub user_id: Option<String>,
    /// @emoji 🎚️ The peer's resolved studio role (`"owner"`/`"member"`/`"viewer"`), present alongside `user_id`.
    pub role: Option<String>,
    /// @emoji 👻️ Serialized preview of an in-flight drag (opaque JSON, schema owned by the dragging app, APP scope).
    pub drag_ghost_json: Option<String>,
    /// @emoji 🕹️ This peer's live selection+hover roster (ARTIFACT scope), mirrored from local
    /// `InteractionState` — see `assemble_presence_interaction` below. `None` for peers on apps that
    /// declare no interaction domains.
    pub interaction: Option<PresenceInteraction>,
    /// @emoji 🎨️ Hub-assigned palette index, stamped by the client actor — never filled by a shell.
    pub color: Option<u8>,
    /// @emoji 🪟️ Canonical surface id, stamped by the client actor — never filled by a shell.
    pub surface: Option<String>,
    /// @emoji 🪟️ Every open window/surface's live camera + in-view pointer (ARTIFACT scope), matched
    /// by `space`. Empty when the peer has no open windows for this document.
    pub views: Vec<PresenceWindowView>,
    /// @emoji 🖱️ Live `data-ui-path` hover/focus/press state (APP scope).
    pub ui: Option<PresenceUi>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. `presence_pack` mirrors
/// the pre-existing `#[serde(with = "presence_pack_serde")]` base64-string wire shape byte-for-byte
/// (rather than the default `Vec<u8>` numeric-array shape) via this crate's own
/// `semio-framework-io-base64` dependency. The real wire path is `encode_presence_peer`/
/// `decode_presence_peer` below (a hand-rolled binary codec); this exists for capability parity.
impl crate::value::ToValue for PresencePeer {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![("actor".to_string(), crate::value::ToValue::to_value(&self.actor)), ("connectedAtMs".to_string(), crate::value::ToValue::to_value(&self.connected_at_ms))];
        if let Some(label) = &self.label {
            entries.push(("label".to_string(), crate::value::ToValue::to_value(label)));
        }
        if let Some(pack) = &self.presence_pack {
            entries.push(("presencePack".to_string(), crate::value::DslValue::String(crate::base64_standard_encode(pack))));
        }
        if let Some(user_id) = &self.user_id {
            entries.push(("userId".to_string(), crate::value::ToValue::to_value(user_id)));
        }
        if let Some(role) = &self.role {
            entries.push(("role".to_string(), crate::value::ToValue::to_value(role)));
        }
        if let Some(drag_ghost_json) = &self.drag_ghost_json {
            entries.push(("dragGhostJson".to_string(), crate::value::ToValue::to_value(drag_ghost_json)));
        }
        if let Some(interaction) = &self.interaction {
            entries.push(("interaction".to_string(), crate::value::ToValue::to_value(interaction)));
        }
        if let Some(color) = &self.color {
            entries.push(("color".to_string(), crate::value::ToValue::to_value(color)));
        }
        if let Some(surface) = &self.surface {
            entries.push(("surface".to_string(), crate::value::ToValue::to_value(surface)));
        }
        if !self.views.is_empty() {
            entries.push(("views".to_string(), crate::value::ToValue::to_value(&self.views)));
        }
        if let Some(ui) = &self.ui {
            entries.push(("ui".to_string(), crate::value::ToValue::to_value(ui)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for PresencePeer {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for PresencePeer, found {value:?}")));
        };
        let mut actor = None;
        let mut connected_at_ms = None;
        let mut label = None;
        let mut presence_pack = None;
        let mut user_id = None;
        let mut role = None;
        let mut drag_ghost_json = None;
        let mut interaction = None;
        let mut color = None;
        let mut surface = None;
        let mut views = Vec::new();
        let mut ui = None;
        for (key, entry) in fields {
            match key.as_str() {
                "actor" => actor = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("actor"))?),
                "connectedAtMs" => connected_at_ms = Some(<i64 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("connectedAtMs"))?),
                "label" => label = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("label"))?,
                "presencePack" => {
                    let crate::value::DslValue::String(encoded) = entry else {
                        return Err(crate::value::ValueError::new("PresencePeer.presencePack must be a string").under("presencePack"));
                    };
                    presence_pack = Some(crate::base64_standard_decode(encoded.as_bytes()).map_err(|error| crate::value::ValueError::new(error.to_string()).under("presencePack"))?);
                }
                "userId" => user_id = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("userId"))?,
                "role" => role = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("role"))?,
                "dragGhostJson" => drag_ghost_json = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("dragGhostJson"))?,
                "interaction" => interaction = <Option<PresenceInteraction> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("interaction"))?,
                "color" => color = <Option<u8> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("color"))?,
                "surface" => surface = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("surface"))?,
                "views" => views = <Vec<PresenceWindowView> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("views"))?,
                "ui" => ui = <Option<PresenceUi> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("ui"))?,
                _ => {}
            }
        }
        Ok(PresencePeer {
            actor: actor.ok_or_else(|| crate::value::ValueError::new("PresencePeer missing actor"))?,
            connected_at_ms: connected_at_ms.ok_or_else(|| crate::value::ValueError::new("PresencePeer missing connectedAtMs"))?,
            label,
            presence_pack,
            user_id,
            role,
            drag_ghost_json,
            interaction,
            color,
            surface,
            views,
            ui,
        })
    }
}

/// @emoji 🎯️ Binary `PresencePeer` codec: `actor str | flags varint_u64 | connected_at_ms varint |
/// fields present per bitmask, strictly in bit order`. `protocol_wire::ClientFrame::Presence`/
/// `ServerFrame::Presence` carry the resulting bytes opaquely (that crate has no dependency on this
/// one) — this is the encode/decode pair store_sync calls on either side of the wire.
/// `presence_pack` is length-prefixed bytes in flag bit 1; `drag_ghost_json` stays opaque app-owned
/// text (never re-parsed as JSON here, same as `ArtifactDiff.payload` staying opaque bytes). Bit 5
/// carries `interaction` via `encode_presence_interaction`/`decode_presence_interaction`
/// (self-delimiting varint-counted fields — see the `🔖️PresenceInteraction` region below); bit 8
/// (`views`) is set iff non-empty; bit 9 (`ui`) carries three `opt_str` fields unconditionally once
/// present. `flags` widened from a single `u8` to a varint (ticket 26/08/17/SHARED-PRESENCE-SESSION-
/// COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.1) now that bit 9 exceeds a byte's range.
pub async fn encode_presence_peer(peer: &PresencePeer) -> Vec<u8> {
    let mut out = Vec::new();
    crate::write_str(&mut out, &peer.actor);
    let mut flags = 0u64;
    if peer.label.is_some() {
        flags |= 1 << 0;
    }
    if peer.presence_pack.is_some() {
        flags |= 1 << 1;
    }
    if peer.user_id.is_some() {
        flags |= 1 << 2;
    }
    if peer.role.is_some() {
        flags |= 1 << 3;
    }
    if peer.drag_ghost_json.is_some() {
        flags |= 1 << 4;
    }
    if peer.interaction.is_some() {
        flags |= 1 << 5;
    }
    if peer.color.is_some() {
        flags |= 1 << 6;
    }
    if peer.surface.is_some() {
        flags |= 1 << 7;
    }
    if !peer.views.is_empty() {
        flags |= 1 << 8;
    }
    if peer.ui.is_some() {
        flags |= 1 << 9;
    }
    crate::wire::write_varint_u64(&mut out, flags);
    crate::wire::write_varint_u64(&mut out, peer.connected_at_ms as u64);
    if let Some(label) = &peer.label {
        crate::write_str(&mut out, label);
    }
    if let Some(presence_pack) = &peer.presence_pack {
        crate::write_bytes(&mut out, presence_pack);
    }
    if let Some(user_id) = &peer.user_id {
        crate::write_str(&mut out, user_id);
    }
    if let Some(role) = &peer.role {
        crate::write_str(&mut out, role);
    }
    if let Some(drag_ghost_json) = &peer.drag_ghost_json {
        crate::write_str(&mut out, drag_ghost_json);
    }
    if let Some(interaction) = &peer.interaction {
        encode_presence_interaction(interaction, &mut out).await;
    }
    if let Some(color) = peer.color {
        out.push(color);
    }
    if let Some(surface) = &peer.surface {
        crate::write_str(&mut out, surface);
    }
    if !peer.views.is_empty() {
        write_vec_presence_window_view(&mut out, &peer.views).await;
    }
    if let Some(ui) = &peer.ui {
        encode_presence_ui(ui, &mut out).await;
    }
    out
}

/// @emoji 🎯️ Inverse of [`encode_presence_peer`]. Any flag bit ≥ 10 set is a drift guard failure
/// (`ProtocolError::Malformed { what: "presence peer flags", .. }`) — no silent forward compatibility.
pub async fn decode_presence_peer(bytes: &[u8]) -> Result<PresencePeer, crate::ProtocolError> {
    let mut pos = 0usize;
    let actor = crate::read_str(bytes, &mut pos)?;
    let flags = crate::wire::read_varint_u64(bytes, &mut pos)?;
    if flags >> 10 != 0 {
        return Err(crate::ProtocolError::Malformed { what: "presence peer flags", offset: pos as u64, detail: format!("unknown flag bits set: {flags:#x}") });
    }
    let connected_at_ms = crate::wire::read_varint_u64(bytes, &mut pos)? as i64;
    let label = if flags & (1 << 0) != 0 { Some(crate::read_str(bytes, &mut pos)?) } else { None };
    let presence_pack = if flags & (1 << 1) != 0 { Some(crate::read_bytes(bytes, &mut pos)?) } else { None };
    let user_id = if flags & (1 << 2) != 0 { Some(crate::read_str(bytes, &mut pos)?) } else { None };
    let role = if flags & (1 << 3) != 0 { Some(crate::read_str(bytes, &mut pos)?) } else { None };
    let drag_ghost_json = if flags & (1 << 4) != 0 { Some(crate::read_str(bytes, &mut pos)?) } else { None };
    let interaction = if flags & (1 << 5) != 0 { Some(decode_presence_interaction(bytes, &mut pos).await?) } else { None };
    let color = if flags & (1 << 6) != 0 {
        let byte = *bytes.get(pos).ok_or(crate::ProtocolError::Malformed { what: "presence peer color", offset: pos as u64, detail: "truncated".to_string() })?;
        pos += 1;
        Some(byte)
    } else {
        None
    };
    let surface = if flags & (1 << 7) != 0 { Some(crate::read_str(bytes, &mut pos)?) } else { None };
    let views = if flags & (1 << 8) != 0 { read_vec_presence_window_view(bytes, &mut pos).await? } else { Vec::new() };
    let ui = if flags & (1 << 9) != 0 { Some(decode_presence_ui(bytes, &mut pos).await?) } else { None };
    Ok(PresencePeer { actor, connected_at_ms, label, presence_pack, user_id, role, drag_ghost_json, interaction, color, surface, views, ui })
}

#[cfg(test)]
mod presence_codec_tests {
    use super::{decode_presence_peer, encode_presence_peer, PresenceDomain, PresenceInteraction, PresencePeer, PresenceUi, PresenceViewKind, PresenceWindowView};

    #[semio_framework_async_macros::async_test]
    async fn presence_peer_binary_round_trips_with_every_field_absent() {
        let peer = PresencePeer { actor: "peer-1".into(), connected_at_ms: 1000, label: None, presence_pack: None, user_id: None, role: None, drag_ghost_json: None, interaction: None, color: None, surface: None, views: Vec::new(), ui: None };
        let bytes = encode_presence_peer(&peer).await;
        assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
    }

    #[semio_framework_async_macros::async_test]
    async fn presence_peer_binary_round_trips_with_every_field_present() {
        let peer = PresencePeer {
            actor: "peer-2".into(),
            connected_at_ms: 1_700_000_000_000,
            label: Some("Ada".into()),
            presence_pack: Some(b"{\"ids\":[1,2]}".to_vec()),
            user_id: Some("user-9".into()),
            role: Some("owner".into()),
            drag_ghost_json: Some("{\"kind\":\"move\"}".into()),
            interaction: Some(PresenceInteraction { app_id: "draw".into(), domains: vec![PresenceDomain { domain: "graph".into(), granularity: "node".into(), selected: vec!["n1".into()], hovered: vec!["n2".into()] }] }),
            color: Some(3),
            surface: Some("s.space.home@1/*#editor".into()),
            views: vec![
                PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: PresenceViewKind::Canvas { x: 1.0, y: 2.0, zoom: 1.5 }, size: [800.0, 600.0], pointer: Some([10.0, 20.0, 0.0]) },
                PresenceWindowView { window_id: "w2".into(), space: "world".into(), kind: PresenceViewKind::Orbit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], up: [0.0, 1.0, 0.0], fov: 45.0 }, size: [1024.0, 768.0], pointer: None },
            ],
            ui: Some(PresenceUi { hovered_path: Some("row[0]#a".into()), focused_path: None, pressed_path: Some("btn[1]#save".into()) }),
        };
        let bytes = encode_presence_peer(&peer).await;
        assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
    }

    #[semio_framework_async_macros::async_test]
    async fn presence_peer_round_trips_views_ui_color_surface() {
        let peer = PresencePeer {
            actor: "peer-4".into(),
            connected_at_ms: 5000,
            label: None,
            presence_pack: None,
            user_id: None,
            role: None,
            drag_ghost_json: None,
            interaction: None,
            color: Some(11),
            surface: Some("s.space.home@1/*#viewer".into()),
            views: vec![PresenceWindowView { window_id: "w1".into(), space: "geo".into(), kind: PresenceViewKind::Geo { lng: 8.5, lat: 47.4, zoom: 12.0, bearing: 0.0, pitch: 0.0 }, size: [500.0, 400.0], pointer: Some([8.5, 47.4, 0.0]) }],
            ui: Some(PresenceUi { hovered_path: None, focused_path: Some("panel[0]#tools".into()), pressed_path: None }),
        };
        let bytes = encode_presence_peer(&peer).await;
        let decoded = decode_presence_peer(&bytes).await.unwrap();
        assert_eq!(decoded, peer);
        assert_eq!(decoded.color, Some(11));
        assert_eq!(decoded.views.len(), 1);
        assert!(decoded.ui.is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn presence_peer_rejects_unknown_flag_bits() {
        // 🔎️ Hand-built rather than mutating an `encode_presence_peer` output: flags is a
        // varint_u64, so flipping a bit in the encoded byte stream doesn't map 1:1 onto a logical
        // flag bit. Bit 10 is one past the frozen 0..=9 range — no field on this struct sets it.
        let mut bytes = Vec::new();
        crate::write_str(&mut bytes, "peer-5");
        crate::wire::write_varint_u64(&mut bytes, 1 << 10);
        crate::wire::write_varint_u64(&mut bytes, 1000);
        let err = decode_presence_peer(&bytes).await.unwrap_err();
        assert!(matches!(err, crate::ProtocolError::Malformed { what: "presence peer flags", .. }));
    }

    //#region 🔖️InteractionBit
    async fn peer_with_interaction(interaction: Option<PresenceInteraction>) -> PresencePeer {
        PresencePeer { actor: "peer-3".into(), connected_at_ms: 1000, label: None, presence_pack: None, user_id: None, role: None, drag_ghost_json: None, interaction, color: None, surface: None, views: Vec::new(), ui: None }
    }

    /// 🔎️ Presence byte index: `actor str`'s own varint-length prefix (1 byte for `peer_with_interaction`'s
    /// short actor id) plus the actor bytes themselves.
    async fn presence_flag_byte(peer: &PresencePeer, bytes: &[u8]) -> u8 {
        bytes[1 + peer.actor.len()]
    }

    #[semio_framework_async_macros::async_test]
    async fn presence_peer_bit_5_round_trips_with_interaction_present() {
        let peer = peer_with_interaction(Some(PresenceInteraction { app_id: "draw".into(), domains: vec![PresenceDomain { domain: "graph".into(), granularity: "node".into(), selected: vec!["n1".into(), "n2".into()], hovered: vec![] }] })).await;
        let bytes = encode_presence_peer(&peer).await;
        assert_eq!(presence_flag_byte(&peer, &bytes).await & (1 << 5), 1 << 5, "bit 5 set when interaction present");
        assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
    }

    #[semio_framework_async_macros::async_test]
    async fn presence_peer_bit_5_round_trips_with_interaction_absent() {
        let peer = peer_with_interaction(None).await;
        let bytes = encode_presence_peer(&peer).await;
        assert_eq!(presence_flag_byte(&peer, &bytes).await & (1 << 5), 0, "bit 5 clear when interaction absent");
        assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
    }

    #[semio_framework_async_macros::async_test]
    async fn presence_peer_interaction_round_trips_with_multiple_domains() {
        let peer = peer_with_interaction(Some(PresenceInteraction {
            app_id: "space".into(),
            domains: vec![
                PresenceDomain { domain: "outline".into(), granularity: "task".into(), selected: vec!["t1".into(), "t2".into()], hovered: vec!["t3".into()] },
                PresenceDomain { domain: "board".into(), granularity: "card".into(), selected: vec![], hovered: vec!["c1".into(), "c2".into(), "c3".into()] },
                PresenceDomain { domain: "canvas".into(), granularity: "node".into(), selected: vec!["n9".into()], hovered: vec![] },
            ],
        }))
        .await;
        let bytes = encode_presence_peer(&peer).await;
        let decoded = decode_presence_peer(&bytes).await.unwrap();
        assert_eq!(decoded, peer);
        assert_eq!(decoded.interaction.unwrap().domains.len(), 3);
    }
    //#endregion 🔖️InteractionBit
}

//#endregion 🔖️Presence

//#region 🔖️Interaction
// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: relocated here (from
// `semio-framework`'s `🔨️modules/🕹️interaction/{🦀️.rs,🧬️schema/🦀️component.rs}`) to
// unblock naming `InteractionState`/`PresenceInteraction` from `store`/`sync` — `semio-framework`
// depends on this crate, never the reverse, so nothing under `os_spr` could previously name a
// framework-defined type. `InteractionDefinition`/`GranularityDefinition`/`InteractionRef` stay in
// `semio-framework` (their `label`/`icon_id` fields pull in `ui_wgpu::LocalizedLabel`/`IconName`,
// which this wasm-safe kernel crate does not and must not depend on) and now `pub use` everything
// below instead of redefining it — see that module's own header comment. Every wave-0 test covering
// this code moved with it, verbatim except `validate_state`'s fixtures (see `InteractionOutline`).
/// 🐁️ One domain's hover behavior — see `semio_framework::InteractionDefinition::hover`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
    // 🚫️async: E1 impl of externally-declared `Default` trait
    fn default() -> Self {
        Self { enabled: true, transitive: false, channels: default_pointer_channels(), broadcast: true }
    }
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. Mirrors the
/// pre-existing `#[serde(rename_all = "camelCase", default = "…")]` wire shape byte-for-byte:
/// every field is sparse-optional on read (missing key falls back to `default()`), always emitted
/// on write.
impl crate::value::ToValue for HoverSpec {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("enabled".to_string(), crate::value::ToValue::to_value(&self.enabled)),
            ("transitive".to_string(), crate::value::ToValue::to_value(&self.transitive)),
            ("channels".to_string(), crate::value::ToValue::to_value(&self.channels)),
            ("broadcast".to_string(), crate::value::ToValue::to_value(&self.broadcast)),
        ])
    }
}
impl crate::value::FromValue for HoverSpec {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for HoverSpec, found {value:?}")));
        };
        let mut out = HoverSpec::default();
        for (key, entry) in fields {
            match key.as_str() {
                "enabled" => out.enabled = <bool as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("enabled"))?,
                "transitive" => out.transitive = <bool as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("transitive"))?,
                "channels" => out.channels = <Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("channels"))?,
                "broadcast" => out.broadcast = <bool as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("broadcast"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

// 🚫️async: E1 — called by name from `#[serde(default = "...")]`, whose generated call site is sync.
fn default_true() -> bool {
    true
}

/// 🖱️ One domain's selection behavior — see `semio_framework::InteractionDefinition::selection`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for SelectionSpec {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("modes".to_string(), crate::value::ToValue::to_value(&self.modes)),
            ("methods".to_string(), crate::value::ToValue::to_value(&self.methods)),
            ("merges".to_string(), crate::value::ToValue::to_value(&self.merges)),
            ("transitive".to_string(), crate::value::ToValue::to_value(&self.transitive)),
            ("broadcast".to_string(), crate::value::ToValue::to_value(&self.broadcast)),
        ])
    }
}
impl crate::value::FromValue for SelectionSpec {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for SelectionSpec, found {value:?}")));
        };
        let mut modes = None;
        let mut methods = None;
        let mut merges = None;
        let mut transitive = false;
        let mut broadcast = true;
        for (key, entry) in fields {
            match key.as_str() {
                "modes" => modes = Some(<Vec<SelectionMode> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("modes"))?),
                "methods" => methods = Some(<Vec<SelectionMethod> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("methods"))?),
                "merges" => merges = Some(<Vec<MergeMode> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("merges"))?),
                "transitive" => transitive = <bool as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("transitive"))?,
                "broadcast" => broadcast = <bool as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("broadcast"))?,
                _ => {}
            }
        }
        Ok(SelectionSpec {
            modes: modes.ok_or_else(|| crate::value::ValueError::new("SelectionSpec missing modes"))?,
            methods: methods.ok_or_else(|| crate::value::ValueError::new("SelectionSpec missing methods"))?,
            merges: merges.ok_or_else(|| crate::value::ValueError::new("SelectionSpec missing merges"))?,
            transitive,
            broadcast,
        })
    }
}

/// 🌳️ Where a domain's target ids come from, and thus what `DomainTopology` (if any) is available for
/// range selection and transitive hover/select closures.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. Internally tagged on
/// `"kind"`, mirroring `#[serde(tag = "kind", rename_all = "camelCase")]` byte-for-byte
/// (`rename_all_fields` dropped per the derive's own doc: redundant when it matches `rename_all`).
impl crate::value::ToValue for HierarchyProvider {
    fn to_value(&self) -> crate::value::DslValue {
        match self {
            HierarchyProvider::Flat => crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("flat".to_string()))]),
            HierarchyProvider::Topology => crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("topology".to_string()))]),
            HierarchyProvider::UiTree => crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("uiTree".to_string()))]),
            HierarchyProvider::PathDelimited { delimiter } => {
                crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("pathDelimited".to_string())), ("delimiter".to_string(), crate::value::ToValue::to_value(delimiter))])
            }
        }
    }
}
impl crate::value::FromValue for HierarchyProvider {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for HierarchyProvider, found {value:?}")));
        };
        let kind = fields.iter().find(|(k, _)| k == "kind").map(|(_, v)| v.clone()).ok_or_else(|| crate::value::ValueError::new("HierarchyProvider missing kind"))?;
        let crate::value::DslValue::String(kind) = kind else {
            return Err(crate::value::ValueError::new("HierarchyProvider kind must be a string"));
        };
        match kind.as_str() {
            "flat" => Ok(HierarchyProvider::Flat),
            "topology" => Ok(HierarchyProvider::Topology),
            "uiTree" => Ok(HierarchyProvider::UiTree),
            "pathDelimited" => {
                let delimiter = fields.into_iter().find(|(k, _)| k == "delimiter").map(|(_, v)| v).ok_or_else(|| crate::value::ValueError::new("HierarchyProvider.pathDelimited missing delimiter"))?;
                Ok(HierarchyProvider::PathDelimited { delimiter: <String as crate::value::FromValue>::from_value(delimiter).map_err(|e| e.under("delimiter"))? })
            }
            other => Err(crate::value::ValueError::new(format!("unknown HierarchyProvider kind `{other}`"))),
        }
    }
}

/// 🔢️ How many targets may be selected at once within a domain.
/// 🌱️ serde's derives are carried ALONGSIDE the hand-written `ToValue`/`FromValue` twin below —
/// the transitional state the serde-fanout playbook prescribes ("add alongside, do not blind-swap").
/// Attributes are restored verbatim from 67fb4216b2 so the serde wire shape stays byte-identical to
/// the twin. Drop them once every consumer in `🔗️causal`/`📡️wire`/`⚔️conflict` moves to `ToValue`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SelectionMode {
    Single,
    Multiple,
}

/// 🌱️ RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01): hand-written, not
/// derived — this crate physically owns `ToValue`/`FromValue`/`DslValue` (`crate::value`), and the
/// `#[derive(ToValue, FromValue)]` macro's generated code is rooted at
/// `::semio_framework_os_kernel::…`, which this crate cannot depend on (os-kernel depends on
/// `protocol`, not the reverse). A bare-string representation matching `serde`'s own default for a
/// data-less enum (`"single"`/`"multiple"`, not `{"tag":"single"}`).
impl crate::value::ToValue for SelectionMode {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(
            match self {
                SelectionMode::Single => "single",
                SelectionMode::Multiple => "multiple",
            }
            .to_string(),
        )
    }
}
impl crate::value::FromValue for SelectionMode {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        match value {
            crate::value::DslValue::String(s) => match s.as_str() {
                "single" => Ok(SelectionMode::Single),
                "multiple" => Ok(SelectionMode::Multiple),
                other => Err(crate::value::ValueError::new(format!("unknown SelectionMode variant `{other}`"))),
            },
            other => Err(crate::value::ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🎯️ How a surface gathers targets for one `interactionSelect` dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SelectionMethod {
    Pick,
    Rectangle,
    Lasso,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for SelectionMethod {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(match self { SelectionMethod::Pick => "pick", SelectionMethod::Rectangle => "rectangle", SelectionMethod::Lasso => "lasso" }.to_string())
    }
}
impl crate::value::FromValue for SelectionMethod {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        match value {
            crate::value::DslValue::String(s) => match s.as_str() {
                "pick" => Ok(SelectionMethod::Pick),
                "rectangle" => Ok(SelectionMethod::Rectangle),
                "lasso" => Ok(SelectionMethod::Lasso),
                other => Err(crate::value::ValueError::new(format!("unknown SelectionMethod variant `{other}`"))),
            },
            other => Err(crate::value::ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🧮️ Set algebra applied when merging new targets into the current selection — see `next_selection`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MergeMode {
    Replace,
    Additive,
    Subtractive,
    Invertive,
    Range,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for MergeMode {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(match self { MergeMode::Replace => "replace", MergeMode::Additive => "additive", MergeMode::Subtractive => "subtractive", MergeMode::Invertive => "invertive", MergeMode::Range => "range" }.to_string())
    }
}
impl crate::value::FromValue for MergeMode {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        match value {
            crate::value::DslValue::String(s) => match s.as_str() {
                "replace" => Ok(MergeMode::Replace),
                "additive" => Ok(MergeMode::Additive),
                "subtractive" => Ok(MergeMode::Subtractive),
                "invertive" => Ok(MergeMode::Invertive),
                "range" => Ok(MergeMode::Range),
                other => Err(crate::value::ValueError::new(format!("unknown MergeMode variant `{other}`"))),
            },
            other => Err(crate::value::ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

// 🚫️async: E1 — called from a plain `Default` impl body, which is sync.
fn default_pointer_channels() -> Vec<String> {
    vec!["pointer".to_string()]
}

//#region 🔖️Runtime
/// 🎯️ One addressed target: a granularity id plus the target's own id (u32 domain ids are stringified
/// at the app boundary before reaching this module).await.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionTarget {
    pub granularity: String,
    pub id: String,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for InteractionTarget {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![("granularity".to_string(), crate::value::ToValue::to_value(&self.granularity)), ("id".to_string(), crate::value::ToValue::to_value(&self.id))])
    }
}
impl crate::value::FromValue for InteractionTarget {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for InteractionTarget, found {value:?}")));
        };
        let mut granularity = None;
        let mut id = None;
        for (key, entry) in fields {
            match key.as_str() {
                "granularity" => granularity = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("granularity"))?),
                "id" => id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                _ => {}
            }
        }
        Ok(InteractionTarget {
            granularity: granularity.ok_or_else(|| crate::value::ValueError::new("InteractionTarget missing granularity"))?,
            id: id.ok_or_else(|| crate::value::ValueError::new("InteractionTarget missing id"))?,
        })
    }
}

/// 🖱️ One domain's current selection: the active granularity, the selected ids, and the anchor id
/// range selection pivots from.
/// 🌱️ Carries serde's derives ALONGSIDE the hand-written `ToValue`/`FromValue` below, which is the
/// transitional state the serde-fanout playbook prescribes ("add alongside — do not blind-swap").
/// Its consumers in `⚔️conflict`, `📡️wire/🏠️local-interaction` and `🔗️causal` still serialize it
/// through serde; the three fields are `String`/`Vec<String>`/`Option<String>`, so the derives add no
/// bound fan-out. Drop them once those call sites move to `ToValue`.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DomainSelection {
    pub granularity: String,
    pub ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_id: Option<String>,
}

/// 🌱️ Hand-written twin of the `SelectionMode` note above — same reason (this crate cannot depend
/// on the derive macro's target crate).
impl crate::value::ToValue for DomainSelection {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries: Vec<(String, crate::value::DslValue)> = vec![
            ("granularity".to_string(), crate::value::ToValue::to_value(&self.granularity)),
            ("ids".to_string(), crate::value::ToValue::to_value(&self.ids)),
        ];
        if self.anchor_id.is_some() {
            entries.push(("anchorId".to_string(), crate::value::ToValue::to_value(&self.anchor_id)));
        }
        crate::value::DslValue::Object(entries)
    }
}
impl crate::value::FromValue for DomainSelection {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let entries = value.into_object()?;
        let granularity = match entries.iter().find(|(k, _)| k == "granularity") {
            Some((_, v)) => crate::value::FromValue::from_value(v.clone()).map_err(|error: crate::value::ValueError| error.under("granularity"))?,
            None => return Err(crate::value::ValueError::new("missing field `granularity`")),
        };
        let ids = match entries.iter().find(|(k, _)| k == "ids") {
            Some((_, v)) => crate::value::FromValue::from_value(v.clone()).map_err(|error: crate::value::ValueError| error.under("ids"))?,
            None => return Err(crate::value::ValueError::new("missing field `ids`")),
        };
        let anchor_id = match entries.iter().find(|(k, _)| k == "anchorId") {
            Some((_, v)) => crate::value::FromValue::from_value(v.clone()).map_err(|error: crate::value::ValueError| error.under("anchorId"))?,
            None => ::std::default::Default::default(),
        };
        Ok(Self { granularity, ids, anchor_id })
    }
}

/// 🐁️ One domain's current hover on one channel: the transitive closure (root first) when
/// `HoverSpec::transitive`, otherwise just the raw hovered ids.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainHover {
    pub channel: String,
    pub ids: Vec<String>,
}

/// 🌱️ Hand-written twin of the `SelectionMode` note above.
impl crate::value::ToValue for DomainHover {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object([
            ("channel".to_string(), crate::value::ToValue::to_value(&self.channel)),
            ("ids".to_string(), crate::value::ToValue::to_value(&self.ids)),
        ])
    }
}
impl crate::value::FromValue for DomainHover {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let entries = value.into_object()?;
        let channel = match entries.iter().find(|(k, _)| k == "channel") {
            Some((_, v)) => crate::value::FromValue::from_value(v.clone()).map_err(|error: crate::value::ValueError| error.under("channel"))?,
            None => return Err(crate::value::ValueError::new("missing field `channel`")),
        };
        let ids = match entries.iter().find(|(k, _)| k == "ids") {
            Some((_, v)) => crate::value::FromValue::from_value(v.clone()).map_err(|error: crate::value::ValueError| error.under("ids"))?,
            None => return Err(crate::value::ValueError::new("missing field `ids`")),
        };
        Ok(Self { channel, ids })
    }
}

/// 🗺️ Own persisted-local selection (`Interaction` history lane).await + ephemeral-local hover, keyed by
/// domain id — the framework-owned counterpart to what every per-app config used to hand-roll.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionState {
    pub selection: BTreeMap<String, DomainSelection>,
    pub hover: BTreeMap<String, DomainHover>,
    pub active_mode: BTreeMap<String, SelectionMode>,
    pub active_granularity: BTreeMap<String, String>,
}

/// 🌱️ Hand-written twin of the `SelectionMode` note above — this is the type
/// `crate::app::InteractionConfigMutation` (`🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`)
/// composes through `SetInteractionState`, so it must satisfy `Mutation`/`MutationDiff`'s
/// `ToValue + FromValue` supertrait bound like every other mutation payload.
impl crate::value::ToValue for InteractionState {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object([
            ("selection".to_string(), crate::value::ToValue::to_value(&self.selection)),
            ("hover".to_string(), crate::value::ToValue::to_value(&self.hover)),
            ("activeMode".to_string(), crate::value::ToValue::to_value(&self.active_mode)),
            ("activeGranularity".to_string(), crate::value::ToValue::to_value(&self.active_granularity)),
        ])
    }
}
impl crate::value::FromValue for InteractionState {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let entries = value.into_object()?;
        let selection = match entries.iter().find(|(k, _)| k == "selection") {
            Some((_, v)) => crate::value::FromValue::from_value(v.clone()).map_err(|error: crate::value::ValueError| error.under("selection"))?,
            None => return Err(crate::value::ValueError::new("missing field `selection`")),
        };
        let hover = match entries.iter().find(|(k, _)| k == "hover") {
            Some((_, v)) => crate::value::FromValue::from_value(v.clone()).map_err(|error: crate::value::ValueError| error.under("hover"))?,
            None => return Err(crate::value::ValueError::new("missing field `hover`")),
        };
        let active_mode = match entries.iter().find(|(k, _)| k == "activeMode") {
            Some((_, v)) => crate::value::FromValue::from_value(v.clone()).map_err(|error: crate::value::ValueError| error.under("activeMode"))?,
            None => return Err(crate::value::ValueError::new("missing field `activeMode`")),
        };
        let active_granularity = match entries.iter().find(|(k, _)| k == "activeGranularity") {
            Some((_, v)) => crate::value::FromValue::from_value(v.clone()).map_err(|error: crate::value::ValueError| error.under("activeGranularity"))?,
            None => return Err(crate::value::ValueError::new("missing field `activeGranularity`")),
        };
        Ok(Self { selection, hover, active_mode, active_granularity })
    }
}
//#endregion 🔖️Runtime

//#region 🔖️Topology
/// 🌳️ One node of a domain's topology: its own granularity and its parent id (`None` = a root).await.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopologyNode {
    pub id: String,
    pub granularity: String,
    pub parent: Option<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for TopologyNode {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![("id".to_string(), crate::value::ToValue::to_value(&self.id)), ("granularity".to_string(), crate::value::ToValue::to_value(&self.granularity))];
        if self.parent.is_some() {
            entries.push(("parent".to_string(), crate::value::ToValue::to_value(&self.parent)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for TopologyNode {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for TopologyNode, found {value:?}")));
        };
        let mut id = None;
        let mut granularity = None;
        let mut parent = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "granularity" => granularity = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("granularity"))?),
                "parent" => parent = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("parent"))?,
                _ => {}
            }
        }
        Ok(TopologyNode {
            id: id.ok_or_else(|| crate::value::ValueError::new("TopologyNode missing id"))?,
            granularity: granularity.ok_or_else(|| crate::value::ValueError::new("TopologyNode missing granularity"))?,
            parent,
        })
    }
}

/// 🌲️ One domain's topology, pre-order: `ordered`'s sequence IS the range-selection order, and every
/// node's descendants form a contiguous run immediately following it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DomainTopology {
    pub ordered: Vec<TopologyNode>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for DomainTopology {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![("ordered".to_string(), crate::value::ToValue::to_value(&self.ordered))])
    }
}
impl crate::value::FromValue for DomainTopology {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for DomainTopology, found {value:?}")));
        };
        let mut ordered = Vec::new();
        for (key, entry) in fields {
            if key == "ordered" {
                ordered = <Vec<TopologyNode> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("ordered"))?;
            }
        }
        Ok(DomainTopology { ordered })
    }
}

impl DomainTopology {
    /// 🔎️ The pre-order index of `id`, or `None` when absent.
    pub async fn index_of(&self, id: &str) -> Option<usize> {
        self.ordered.iter().position(|node| node.id == id)
    }

    /// ✅️ Whether `id` is a known node in this topology.
    pub async fn contains(&self, id: &str) -> bool {
        self.index_of(id).await.is_some()
    }

    async fn children_by_parent(&self) -> BTreeMap<String, Vec<String>> {
        let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for node in &self.ordered {
            if let Some(parent) = &node.parent {
                children.entry(parent.clone()).or_default().push(node.id.clone());
            }
        }
        children
    }

    /// 🌳️ `root_id` plus every descendant, pre-order (root first) — empty when `root_id` is absent.
    pub async fn descendant_closure(&self, root_id: &str) -> Vec<String> {
        if !self.contains(root_id).await {
            return Vec::new();
        }
        let children = self.children_by_parent().await;
        let mut out = Vec::new();
        visit_descendants(root_id, &children, &mut out).await;
        out
    }

    /// 🪜️ `id`'s ancestor chain, nearest parent first, root last.
    pub async fn ancestors(&self, id: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut current = self.ordered.iter().find(|node| node.id == id).and_then(|node| node.parent.clone());
        while let Some(parent_id) = current {
            current = self.ordered.iter().find(|node| node.id == parent_id).and_then(|node| node.parent.clone());
            out.push(parent_id);
        }
        out
    }
}

async fn visit_descendants(id: &str, children: &BTreeMap<String, Vec<String>>, out: &mut Vec<String>) {
    out.push(id.to_string());
    if let Some(kids) = children.get(id) {
        for kid in kids {
            Box::pin(visit_descendants(kid, children, out)).await;
        }
    }
}

/// 🗺️ Every domain's topology for one app instance, keyed by domain id — `ArtifactApp::interaction_topology`
/// returns this (wave 3).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InteractionTopology {
    pub domains: BTreeMap<String, DomainTopology>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for InteractionTopology {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![("domains".to_string(), crate::value::ToValue::to_value(&self.domains))])
    }
}
impl crate::value::FromValue for InteractionTopology {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for InteractionTopology, found {value:?}")));
        };
        let mut domains = BTreeMap::new();
        for (key, entry) in fields {
            if key == "domains" {
                domains = <BTreeMap<String, DomainTopology> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("domains"))?;
            }
        }
        Ok(InteractionTopology { domains })
    }
}
//#endregion 🔖️Topology

//#region 🔖️SelectionMachine
/// 🖱️ One `next_selection` call's input: the batch of targets (a single pick or a marquee gather),
/// the merge mode to apply, and the currently active selection mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionInput {
    pub targets: Vec<InteractionTarget>,
    pub merge: MergeMode,
    pub mode: SelectionMode,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for SelectionInput {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("targets".to_string(), crate::value::ToValue::to_value(&self.targets)),
            ("merge".to_string(), crate::value::ToValue::to_value(&self.merge)),
            ("mode".to_string(), crate::value::ToValue::to_value(&self.mode)),
        ])
    }
}
impl crate::value::FromValue for SelectionInput {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for SelectionInput, found {value:?}")));
        };
        let mut targets = None;
        let mut merge = None;
        let mut mode = None;
        for (key, entry) in fields {
            match key.as_str() {
                "targets" => targets = Some(<Vec<InteractionTarget> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("targets"))?),
                "merge" => merge = Some(<MergeMode as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("merge"))?),
                "mode" => mode = Some(<SelectionMode as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("mode"))?),
                _ => {}
            }
        }
        Ok(SelectionInput {
            targets: targets.ok_or_else(|| crate::value::ValueError::new("SelectionInput missing targets"))?,
            merge: merge.ok_or_else(|| crate::value::ValueError::new("SelectionInput missing merge"))?,
            mode: mode.ok_or_else(|| crate::value::ValueError::new("SelectionInput missing mode"))?,
        })
    }
}

/// 🖱️ Computes the next `DomainSelection` for one domain — the generalization of Tree's
/// `getTreeNextSelectionState` (`🖱️ui/🧱️elements/🪵️Tree/🟦️.tsx:946-968`), preserving its
/// exact single/range/toggle semantics while adding batch targets, `Additive`/`Subtractive` as
/// distinct merges, and transitive descendant-closure expansion.
///
/// - `Single` mode ignores `merge` entirely and clamps to the batch's last target (mirrors Tree
///   returning `{selectedIds:[targetId]}` unconditionally in single mode).await.
/// - `Range` replaces the selection with the topology-order slice between the anchor (falling back to
///   `current.anchor_id`, then `current.ids.last()`, then the target itself — mirrors Tree's
///   `fallbackAnchorId`) and the batch's last target, ascending index order; the anchor does not move.
/// - `Replace`/`Additive`/`Subtractive`/`Invertive` apply ordinary set algebra over the batch's targets
///   (each expanded to its descendant closure first when `spec.transitive`), and update the anchor to
///   the batch's last target.
///
/// Empty `input.targets` is a no-op (returns `current` unchanged).
pub async fn next_selection(spec: &SelectionSpec, current: &DomainSelection, topo: &DomainTopology, input: &SelectionInput) -> DomainSelection {
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
        if let (Some(anchor_index), Some(target_index)) = (topo.index_of(&fallback_anchor).await, topo.index_of(&last_target_id).await) {
            let (start, end) = if anchor_index <= target_index { (anchor_index, target_index) } else { (target_index, anchor_index) };
            let ids = topo.ordered[start..=end].iter().map(|node| node.id.clone()).collect();
            return DomainSelection { granularity, ids, anchor_id: Some(fallback_anchor) };
        }
        return DomainSelection { granularity, ids: vec![last_target_id.clone()], anchor_id: Some(last_target_id) };
    }

    let mut expanded: Vec<String> = Vec::new();
    for id in &target_ids {
        if spec.transitive {
            let closure = topo.descendant_closure(id).await;
            if closure.is_empty() {
                expanded.push(id.clone());
            } else {
                expanded.extend(closure);
            }
        } else {
            expanded.push(id.clone());
        }
    }

    let mut ids = match input.merge {
        MergeMode::Replace => dedup_preserving_order(expanded).await,
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
    ids = dedup_preserving_order(ids).await;
    DomainSelection { granularity, ids, anchor_id: Some(last_target_id) }
}

async fn dedup_preserving_order(ids: Vec<String>) -> Vec<String> {
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverInput {
    pub channel: String,
    pub targets: Vec<InteractionTarget>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for HoverInput {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![("channel".to_string(), crate::value::ToValue::to_value(&self.channel)), ("targets".to_string(), crate::value::ToValue::to_value(&self.targets))])
    }
}
impl crate::value::FromValue for HoverInput {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for HoverInput, found {value:?}")));
        };
        let mut channel = None;
        let mut targets = None;
        for (key, entry) in fields {
            match key.as_str() {
                "channel" => channel = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("channel"))?),
                "targets" => targets = Some(<Vec<InteractionTarget> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("targets"))?),
                _ => {}
            }
        }
        Ok(HoverInput {
            channel: channel.ok_or_else(|| crate::value::ValueError::new("HoverInput missing channel"))?,
            targets: targets.ok_or_else(|| crate::value::ValueError::new("HoverInput missing targets"))?,
        })
    }
}

/// 🐁️ Computes the next `DomainHover` for one channel: always REPLACES the channel's id list (hover
/// has no merge algebra). When `spec.transitive`, each target expands to its descendant closure with
/// the hovered root first; multiple targets concatenate in input order, deduplicated. Disabled specs
/// and empty target batches both clear the channel.
pub async fn next_hover(spec: &HoverSpec, topo: &DomainTopology, input: &HoverInput) -> DomainHover {
    if !spec.enabled || input.targets.is_empty() {
        return DomainHover { channel: input.channel.clone(), ids: Vec::new() };
    }
    let mut ids: Vec<String> = Vec::new();
    for target in &input.targets {
        let expanded = if spec.transitive {
            let closure = topo.descendant_closure(&target.id).await;
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
#[derive(Clone, Debug, PartialEq)]
pub struct InteractionOutline {
    pub id: String,
    /// 🪜️ Non-empty; the first entry is the domain's default granularity.
    pub granularity_ids: Vec<String>,
    pub selection: SelectionSpec,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for InteractionOutline {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("id".to_string(), crate::value::ToValue::to_value(&self.id)),
            ("granularityIds".to_string(), crate::value::ToValue::to_value(&self.granularity_ids)),
            ("selection".to_string(), crate::value::ToValue::to_value(&self.selection)),
        ])
    }
}
impl crate::value::FromValue for InteractionOutline {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for InteractionOutline, found {value:?}")));
        };
        let mut id = None;
        let mut granularity_ids = None;
        let mut selection = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "granularityIds" => granularity_ids = Some(<Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("granularityIds"))?),
                "selection" => selection = Some(<SelectionSpec as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("selection"))?),
                _ => {}
            }
        }
        Ok(InteractionOutline {
            id: id.ok_or_else(|| crate::value::ValueError::new("InteractionOutline missing id"))?,
            granularity_ids: granularity_ids.ok_or_else(|| crate::value::ValueError::new("InteractionOutline missing granularityIds"))?,
            selection: selection.ok_or_else(|| crate::value::ValueError::new("InteractionOutline missing selection"))?,
        })
    }
}

/// 🧹️ Re-derives a consistent `InteractionState` from declared `defs` + current `topo`: drops any
/// domain absent from `defs` (renamed/removed interaction declaration), prunes selection/hover ids no
/// longer present in that domain's topology (deleted document nodes — called after every artifact
/// dispatch), resets `active_granularity`/`active_mode` to a declared value (falling back to the
/// domain's default, its first declared entry) when the stored one is no longer declared, and clamps
/// `Single`-mode selections down to their first id (mirrors `normalizeTreeSelectedIds`'s external-update
/// normalization, not `next_selection`'s recency-preferring clamp).await.
pub async fn validate_state(defs: &[InteractionOutline], topo: &InteractionTopology, state: &InteractionState) -> InteractionState {
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
            let mut ids: Vec<String> = Vec::new();
            for id in &selection.ids {
                let keep = match domain_topo {
                    Some(topo) => topo.contains(id).await,
                    None => true,
                };
                if keep {
                    ids.push(id.clone());
                }
            }
            if mode == SelectionMode::Single && ids.len() > 1 {
                ids.truncate(1);
            }
            let anchor_id = selection.anchor_id.clone().filter(|anchor| ids.contains(anchor));
            result.selection.insert(def.id.clone(), DomainSelection { granularity: selection_granularity, ids, anchor_id });
        }

        if let Some(hover) = state.hover.get(&def.id) {
            let mut ids: Vec<String> = Vec::new();
            for id in &hover.ids {
                let keep = match domain_topo {
                    Some(topo) => topo.contains(id).await,
                    None => true,
                };
                if keep {
                    ids.push(id.clone());
                }
            }
            result.hover.insert(def.id.clone(), DomainHover { channel: hover.channel.clone(), ids });
        }
    }

    result
}
//#endregion 🔖️Validation

//#region 🔖️PresenceInteraction
/// 📡️ One peer's interaction roster for one app instance, mirrored onto `PresencePeer.interaction`
/// (bit 5) on the heartbeat — typed (not app-opaque `presence_pack`) so the Shell renders every peer's
/// selection/hover generically. Only explicit ids broadcast; receivers expand transitive closures via
/// their own topology.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PresenceInteraction {
    pub app_id: String,
    pub domains: Vec<PresenceDomain>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. This crate's own
/// hand-rolled binary codec (`🔖️PresenceInteractionCodec` below) is the real wire path; this impl
/// exists for capability parity with the type's former `Serialize`/`Deserialize`.
impl crate::value::ToValue for PresenceInteraction {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![("appId".to_string(), crate::value::ToValue::to_value(&self.app_id)), ("domains".to_string(), crate::value::ToValue::to_value(&self.domains))])
    }
}
impl crate::value::FromValue for PresenceInteraction {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for PresenceInteraction, found {value:?}")));
        };
        let mut out = PresenceInteraction::default();
        for (key, entry) in fields {
            match key.as_str() {
                "appId" => out.app_id = <String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("appId"))?,
                "domains" => out.domains = <Vec<PresenceDomain> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("domains"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

/// 📡️ One domain's broadcast slice of `PresenceInteraction` — the peer-facing mirror of a domain's
/// `DomainSelection`/`DomainHover`, flattened to raw explicit ids (no transitive expansion on the wire).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PresenceDomain {
    pub domain: String,
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl crate::value::ToValue for PresenceDomain {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("domain".to_string(), crate::value::ToValue::to_value(&self.domain)),
            ("granularity".to_string(), crate::value::ToValue::to_value(&self.granularity)),
            ("selected".to_string(), crate::value::ToValue::to_value(&self.selected)),
            ("hovered".to_string(), crate::value::ToValue::to_value(&self.hovered)),
        ])
    }
}
impl crate::value::FromValue for PresenceDomain {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for PresenceDomain, found {value:?}")));
        };
        let mut out = PresenceDomain::default();
        for (key, entry) in fields {
            match key.as_str() {
                "domain" => out.domain = <String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("domain"))?,
                "granularity" => out.granularity = <String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("granularity"))?,
                "selected" => out.selected = <Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("selected"))?,
                "hovered" => out.hovered = <Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("hovered"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

//#region 🔖️PresenceInteractionCodec
// 🎯️ Binary codec for the two structs above — what `encode_presence_peer`/`decode_presence_peer`
// (in the `🔖️Presence` region) call for bit 5. Self-delimiting throughout (varint counts, exactly
// `write_vec_bytes`/`write_vec_envelope`'s own convention in `🔖️Combinators` up top), so no outer
// length prefix is needed around the whole payload.
async fn write_vec_str(out: &mut Vec<u8>, values: &[String]) {
    crate::wire::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::write_str(out, value);
    }
}

async fn read_vec_str(bytes: &[u8], pos: &mut usize) -> Result<Vec<String>, crate::ProtocolError> {
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(crate::read_str(bytes, pos)?);
    }
    Ok(out)
}

async fn encode_presence_domain(domain: &PresenceDomain, out: &mut Vec<u8>) {
    crate::write_str(out, &domain.domain);
    crate::write_str(out, &domain.granularity);
    write_vec_str(out, &domain.selected).await;
    write_vec_str(out, &domain.hovered).await;
}

async fn decode_presence_domain(bytes: &[u8], pos: &mut usize) -> Result<PresenceDomain, crate::ProtocolError> {
    Ok(PresenceDomain { domain: crate::read_str(bytes, pos)?, granularity: crate::read_str(bytes, pos)?, selected: read_vec_str(bytes, pos).await?, hovered: read_vec_str(bytes, pos).await? })
}

/// @emoji 🎯️ Encodes one `PresenceInteraction` — `pub` (ticket 26/08/17/SHARED-PRESENCE-SESSION-
/// COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4): guests never enable the kernel's `sync` feature, and
/// `VcsArtifactApp` (the plugin ABI's presence adoption path) must be able to call this directly.
pub async fn encode_presence_interaction(interaction: &PresenceInteraction, out: &mut Vec<u8>) {
    crate::write_str(out, &interaction.app_id);
    crate::wire::write_varint_u64(out, interaction.domains.len() as u64);
    for domain in &interaction.domains {
        encode_presence_domain(domain, out).await;
    }
}

/// @emoji 🎯️ Inverse of [`encode_presence_interaction`] — see its doc for why this is `pub`.
pub async fn decode_presence_interaction(bytes: &[u8], pos: &mut usize) -> Result<PresenceInteraction, crate::ProtocolError> {
    let app_id = crate::read_str(bytes, pos)?;
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    let mut domains = Vec::with_capacity(count as usize);
    for _ in 0..count {
        domains.push(decode_presence_domain(bytes, pos).await?);
    }
    Ok(PresenceInteraction { app_id, domains })
}
//#endregion 🔖️PresenceInteractionCodec

//#region 🔖️Assemble
/// @emoji 📡️ Assembles `PresencePeer.interaction` from local `InteractionState` plus each domain's
/// declared hover/selection behavior — the ONE place this logic lives, so every app broadcasts
/// selection+hover with ZERO app-side code (call this wherever a `PresenceHeartbeat`'s `peer` is
/// built, right before `presence_to_bytes`/`encode_presence_peer`, at the same cadence cursor updates
/// already get throttled at — this fn is pure/stateless, so it invents no throttle of its own).
/// Forwards each domain's already-computed `ids` verbatim (never re-derives a closure itself) —
/// EXPLICIT ids only cross the wire; a receiver re-expands any transitive closure via its own
/// `DomainTopology`. Only the `"pointer"` hover channel ever broadcasts (any other channel, e.g. a
/// drag-only one, stays local); a domain whose `HoverSpec::broadcast`/`SelectionSpec::broadcast` is
/// `false` contributes nothing for that half. A domain that ends up with both halves empty is omitted
/// from `domains` entirely.
///
/// 🎯️ Moved here (ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION
/// C7.4) from `store_sync` — guests never enable the kernel's `sync` feature, and `VcsArtifactApp`
/// (the plugin ABI's presence-adoption path, §C7.6) must be able to call this pure fn without pulling
/// in the whole actor layer.
pub async fn assemble_presence_interaction(app_id: &str, state: &InteractionState, hover_specs: &BTreeMap<String, HoverSpec>, selection_specs: &BTreeMap<String, SelectionSpec>) -> PresenceInteraction {
    let mut domain_ids: std::collections::BTreeSet<&String> = std::collections::BTreeSet::new();
    domain_ids.extend(state.selection.keys());
    domain_ids.extend(state.hover.keys());

    let mut domains = Vec::new();
    for domain_id in domain_ids {
        let selected = if selection_specs.get(domain_id).is_some_and(|spec| spec.broadcast) { state.selection.get(domain_id).map(|selection| selection.ids.clone()).unwrap_or_default() } else { Vec::new() };
        let hovered = if hover_specs.get(domain_id).is_some_and(|spec| spec.broadcast) { state.hover.get(domain_id).filter(|hover| hover.channel == "pointer").map(|hover| hover.ids.clone()).unwrap_or_default() } else { Vec::new() };
        if selected.is_empty() && hovered.is_empty() {
            continue;
        }
        let granularity = state.active_granularity.get(domain_id).cloned().unwrap_or_default();
        domains.push(PresenceDomain { domain: domain_id.clone(), granularity, selected, hovered });
    }

    PresenceInteraction { app_id: app_id.to_string(), domains }
}

#[cfg(test)]
mod assemble_presence_interaction_tests {
    use super::*;

    async fn selection(ids: &[&str]) -> DomainSelection {
        DomainSelection { granularity: "node".into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: None }
    }

    async fn hover(channel: &str, ids: &[&str]) -> DomainHover {
        DomainHover { channel: channel.into(), ids: ids.iter().map(|id| id.to_string()).collect() }
    }

    async fn broadcasting_hover_spec() -> HoverSpec {
        HoverSpec { enabled: true, transitive: false, channels: vec!["pointer".into()], broadcast: true }
    }

    async fn broadcasting_selection_spec() -> SelectionSpec {
        SelectionSpec { modes: vec![SelectionMode::Multiple], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true }
    }

    #[semio_framework_async_macros::async_test]
    async fn assemble_presence_interaction_includes_broadcasting_domains() {
        let mut state = InteractionState::default();
        state.selection.insert("graph".into(), selection(&["n1", "n2"]).await);
        state.hover.insert("graph".into(), hover("pointer", &["n3"]).await);
        state.active_granularity.insert("graph".into(), "node".into());

        let hover_specs = BTreeMap::from([("graph".to_string(), broadcasting_hover_spec().await)]);
        let selection_specs = BTreeMap::from([("graph".to_string(), broadcasting_selection_spec().await)]);

        let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs).await;
        assert_eq!(interaction.app_id, "draw");
        assert_eq!(interaction.domains.len(), 1);
        let domain = &interaction.domains[0];
        assert_eq!(domain.domain, "graph");
        assert_eq!(domain.granularity, "node");
        assert_eq!(domain.selected, vec!["n1".to_string(), "n2".to_string()]);
        assert_eq!(domain.hovered, vec!["n3".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn assemble_presence_interaction_omits_domains_with_broadcast_disabled() {
        let mut state = InteractionState::default();
        state.selection.insert("private".into(), selection(&["secret"]).await);
        state.hover.insert("private".into(), hover("pointer", &["secret"]).await);

        let hover_specs = BTreeMap::from([("private".to_string(), HoverSpec { broadcast: false, ..broadcasting_hover_spec().await })]);
        let selection_specs = BTreeMap::from([("private".to_string(), SelectionSpec { broadcast: false, ..broadcasting_selection_spec().await })]);

        let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs).await;
        assert!(interaction.domains.is_empty(), "broadcast:false on both halves drops the domain entirely");
    }

    #[semio_framework_async_macros::async_test]
    async fn assemble_presence_interaction_only_broadcasts_the_pointer_hover_channel() {
        let mut state = InteractionState::default();
        state.hover.insert("graph".into(), hover("drag-preview", &["n1"]).await);

        let hover_specs = BTreeMap::from([("graph".to_string(), broadcasting_hover_spec().await)]);
        let selection_specs = BTreeMap::new();

        let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs).await;
        assert!(interaction.domains.is_empty(), "a non-pointer hover channel never broadcasts");
    }

    #[semio_framework_async_macros::async_test]
    async fn assemble_presence_interaction_respects_each_half_independently() {
        let mut state = InteractionState::default();
        state.selection.insert("graph".into(), selection(&["n1"]).await);
        state.hover.insert("graph".into(), hover("pointer", &["n2"]).await);

        let hover_specs = BTreeMap::from([("graph".to_string(), HoverSpec { broadcast: false, ..broadcasting_hover_spec().await })]);
        let selection_specs = BTreeMap::from([("graph".to_string(), broadcasting_selection_spec().await)]);

        let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs).await;
        assert_eq!(interaction.domains.len(), 1);
        assert_eq!(interaction.domains[0].selected, vec!["n1".to_string()], "selection still broadcasts");
        assert!(interaction.domains[0].hovered.is_empty(), "hover suppressed by its own broadcast:false");
    }
}
//#endregion 🔖️Assemble
//#endregion 🔖️PresenceInteraction

//#region 🔖️InteractionStorePack
//#endregion 🔖️InteractionStorePack

#[cfg(test)]
mod interaction_tests {
    use super::*;

    //#region 🔖️Fixtures
    /// 🌲️ root → {a → {a1, a2}, b → {b1}}, pre-order: root, a, a1, a2, b, b1.
    async fn sample_topology() -> DomainTopology {
        let node = |id: &str, parent: Option<&str>| TopologyNode { id: id.into(), granularity: "node".into(), parent: parent.map(Into::into) };
        DomainTopology { ordered: vec![node("root", None), node("a", Some("root")), node("a1", Some("a")), node("a2", Some("a")), node("b", Some("root")), node("b1", Some("b"))] }
    }

    async fn target(id: &str) -> InteractionTarget {
        InteractionTarget { granularity: "node".into(), id: id.into() }
    }

    async fn selection(ids: &[&str], anchor: Option<&str>) -> DomainSelection {
        DomainSelection { granularity: "node".into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: anchor.map(Into::into) }
    }

    async fn spec(transitive: bool, merges: &[MergeMode]) -> SelectionSpec {
        SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: merges.to_vec(), transitive, broadcast: true }
    }

    async fn multiple_input(ids: &[&str], merge: MergeMode) -> SelectionInput {
        let mut targets = Vec::with_capacity(ids.len());
        for id in ids {
            targets.push(target(id).await);
        }
        SelectionInput { targets, merge, mode: SelectionMode::Multiple }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MergeModes
    #[semio_framework_async_macros::async_test]
    async fn replace_sets_selection_to_batch_targets() {
        let current = selection(&["a1"], Some("a1")).await;
        let next = next_selection(&spec(false, &[MergeMode::Replace]).await, &current, &sample_topology().await, &multiple_input(&["b", "b1"], MergeMode::Replace).await).await;
        assert_eq!(next.ids, vec!["b".to_string(), "b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("b1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn additive_unions_batch_into_current_selection() {
        let current = selection(&["a1"], Some("a1")).await;
        let next = next_selection(&spec(false, &[MergeMode::Additive]).await, &current, &sample_topology().await, &multiple_input(&["a2"], MergeMode::Additive).await).await;
        assert_eq!(next.ids, vec!["a1".to_string(), "a2".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a2"));
    }

    #[semio_framework_async_macros::async_test]
    async fn subtractive_removes_batch_from_current_selection() {
        let current = selection(&["a1", "a2", "b1"], Some("b1")).await;
        let next = next_selection(&spec(false, &[MergeMode::Subtractive]).await, &current, &sample_topology().await, &multiple_input(&["a2"], MergeMode::Subtractive).await).await;
        assert_eq!(next.ids, vec!["a1".to_string(), "b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a2"), "anchor tracks the last acted-on target, even on removal");
    }

    #[semio_framework_async_macros::async_test]
    async fn invertive_toggles_each_batch_target_independently() {
        let current = selection(&["a1", "a2"], Some("a2")).await;
        let next = next_selection(&spec(false, &[MergeMode::Invertive]).await, &current, &sample_topology().await, &multiple_input(&["a2", "b1"], MergeMode::Invertive).await).await;
        assert_eq!(next.ids, vec!["a1".to_string(), "b1".to_string()], "a2 was present so it toggles off, b1 was absent so it toggles on");
    }
    //#endregion 🔖️MergeModes

    //#region 🔖️Range
    #[semio_framework_async_macros::async_test]
    async fn range_slices_topology_order_between_anchor_and_target() {
        let current = selection(&["a"], Some("a")).await;
        let next = next_selection(&spec(false, &[MergeMode::Range]).await, &current, &sample_topology().await, &multiple_input(&["b1"], MergeMode::Range).await).await;
        assert_eq!(next.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string(), "b".to_string(), "b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a"), "range never moves the anchor");
    }

    #[semio_framework_async_macros::async_test]
    async fn range_falls_back_to_last_selected_id_when_no_anchor_recorded() {
        let current = selection(&["a1", "a2"], None).await;
        let next = next_selection(&spec(false, &[MergeMode::Range]).await, &current, &sample_topology().await, &multiple_input(&["b"], MergeMode::Range).await).await;
        assert_eq!(next.ids, vec!["a2".to_string(), "b".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("a2"));
    }

    #[semio_framework_async_macros::async_test]
    async fn range_handles_target_before_anchor_in_topology_order() {
        let current = selection(&["b"], Some("b")).await;
        let next = next_selection(&spec(false, &[MergeMode::Range]).await, &current, &sample_topology().await, &multiple_input(&["a1"], MergeMode::Range).await).await;
        assert_eq!(next.ids, vec!["a1".to_string(), "a2".to_string(), "b".to_string()]);
    }
    //#endregion 🔖️Range

    //#region 🔖️SingleClamp
    #[semio_framework_async_macros::async_test]
    async fn single_mode_clamps_to_last_target_regardless_of_merge() {
        let current = selection(&["a1", "a2"], Some("a1")).await;
        let input = SelectionInput { targets: vec![target("b").await, target("b1").await], merge: MergeMode::Additive, mode: SelectionMode::Single };
        let next = next_selection(&spec(false, &[MergeMode::Additive]).await, &current, &sample_topology().await, &input).await;
        assert_eq!(next.ids, vec!["b1".to_string()]);
        assert_eq!(next.anchor_id.as_deref(), Some("b1"));
    }
    //#endregion 🔖️SingleClamp

    //#region 🔖️Transitive
    #[semio_framework_async_macros::async_test]
    async fn transitive_select_expands_target_to_descendant_closure() {
        let current = DomainSelection::default();
        let next = next_selection(&spec(true, &[MergeMode::Replace]).await, &current, &sample_topology().await, &multiple_input(&["a"], MergeMode::Replace).await).await;
        assert_eq!(next.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn transitive_hover_expands_with_root_first() {
        let hover_spec = HoverSpec { enabled: true, transitive: true, channels: default_pointer_channels(), broadcast: true };
        let input = HoverInput { channel: "pointer".into(), targets: vec![target("a").await] };
        let hover = next_hover(&hover_spec, &sample_topology().await, &input).await;
        assert_eq!(hover.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string()]);
        assert_eq!(hover.ids.first().map(String::as_str), Some("a"), "hovered root sorts first");
    }

    #[semio_framework_async_macros::async_test]
    async fn non_transitive_hover_replaces_with_raw_targets_only() {
        let hover_spec = HoverSpec { enabled: true, transitive: false, channels: default_pointer_channels(), broadcast: true };
        let input = HoverInput { channel: "pointer".into(), targets: vec![target("a").await] };
        let hover = next_hover(&hover_spec, &sample_topology().await, &input).await;
        assert_eq!(hover.ids, vec!["a".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_hover_targets_clears_the_channel() {
        let hover_spec = HoverSpec::default();
        let hover = next_hover(&hover_spec, &sample_topology().await, &HoverInput { channel: "pointer".into(), targets: Vec::new() }).await;
        assert!(hover.ids.is_empty());
    }
    //#endregion 🔖️Transitive

    //#region 🔖️ValidateState
    async fn sample_outline() -> InteractionOutline {
        InteractionOutline { id: "graph".into(), granularity_ids: vec!["node".into(), "edge".into()], selection: spec(false, &[MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range]).await }
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_state_prunes_ids_absent_from_topology() {
        let def = sample_outline().await;
        let mut topo = InteractionTopology::default();
        topo.domains.insert("graph".into(), sample_topology().await);

        let mut state = InteractionState::default();
        state.selection.insert("graph".into(), selection(&["a1", "deleted-node", "b1"], Some("deleted-node")).await);
        state.hover.insert("graph".into(), DomainHover { channel: "pointer".into(), ids: vec!["a1".into(), "gone".into()] });
        state.active_mode.insert("graph".into(), SelectionMode::Multiple);
        state.active_granularity.insert("graph".into(), "node".into());

        let validated = validate_state(&[def], &topo, &state).await;
        let graph_selection = validated.selection.get("graph").expect("graph domain kept");
        assert_eq!(graph_selection.ids, vec!["a1".to_string(), "b1".to_string()], "deleted-node pruned");
        assert_eq!(graph_selection.anchor_id, None, "stale anchor pruned along with its id");
        assert_eq!(validated.hover.get("graph").unwrap().ids, vec!["a1".to_string()], "gone pruned");
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_state_drops_undeclared_domains_and_granularities() {
        let def = sample_outline().await;
        let topo = InteractionTopology::default();

        let mut state = InteractionState::default();
        state.selection.insert("mesh".into(), selection(&["x"], None).await);
        state.active_granularity.insert("graph".into(), "face".into());

        let validated = validate_state(&[def], &topo, &state).await;
        assert!(!validated.selection.contains_key("mesh"), "undeclared domain dropped");
        assert_eq!(validated.active_granularity.get("graph").map(String::as_str), Some("node"), "undeclared granularity resets to the default");
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_state_clamps_single_mode_selection_to_first_id() {
        let def = sample_outline().await;
        let mut topo = InteractionTopology::default();
        topo.domains.insert("graph".into(), sample_topology().await);

        let mut state = InteractionState::default();
        state.selection.insert("graph".into(), selection(&["a1", "a2", "b1"], None).await);
        state.active_mode.insert("graph".into(), SelectionMode::Single);

        let validated = validate_state(&[def], &topo, &state).await;
        assert_eq!(validated.selection.get("graph").unwrap().ids, vec!["a1".to_string()]);
    }
    //#endregion 🔖️ValidateState

    //#region 🔖️Serde
    /// 🌱️ Rewritten off `serde_json` (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
    /// 26/09/01): asserts the same internally-tagged shape directly on `DslValue` instead of a JSON
    /// string — this crate cannot depend on `pack::json` (it sits below `pack` in the DAG), and the
    /// `DslValue` tree IS the wire shape `ToValue`/`FromValue` produce/consume.
    #[semio_framework_async_macros::async_test]
    async fn hierarchy_provider_to_value_is_internally_tagged() {
        let path_delimited = HierarchyProvider::PathDelimited { delimiter: "/".into() };
        let value = crate::value::ToValue::to_value(&path_delimited);
        assert_eq!(
            value,
            crate::value::DslValue::object(vec![
                ("kind".to_string(), crate::value::DslValue::String("pathDelimited".to_string())),
                ("delimiter".to_string(), crate::value::DslValue::String("/".to_string())),
            ])
        );
        assert_eq!(<HierarchyProvider as crate::value::FromValue>::from_value(value).unwrap(), path_delimited);

        let flat_value = crate::value::ToValue::to_value(&HierarchyProvider::Flat);
        assert_eq!(flat_value, crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("flat".to_string()))]));
    }
    //#endregion 🔖️Serde
}
//#endregion 🔖️Interaction
