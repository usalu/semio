//! 🎞️ Protocol app-engine channel: the `AppCommand`/`AppFrame` binary frame taxonomy every app,
//! once turned into a headless engine driven by bidirectional streaming of typed binary commands,
//! exchanges with its client (a UI or a headless runner) — every UI interaction becomes a
//! forwarded `AppCommand`, every engine reaction a returned `AppFrame`. Ticket:
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️01/HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS/`.
//!
//! 🎯️ Mirrors `protocol_wire`'s W5 hand-rolled binary layout exactly: `tag: u8` (the enum
//! variant's declaration order) followed by its fields in declaration order, no per-field tags, no
//! body-length prefix — one frame per channel message. `protocol_core::🔖️WireCodec` supplies the
//! primitive codec (`write_varint_u64`/`write_str`/`write_bytes`/`write_bool` and their `read_*`
//! twins); this crate adds only the option/vec/`SectionProbe` combinators and the two enums' tag
//! dispatch below. Unlike `protocol_wire::ClientFrame`/`ServerFrame`, `AppCommand`/`AppFrame` carry
//! no `Lane` byte — the app-engine channel is a single logical stream, not split into
//! causally-ordered vs. best-effort lanes.

//#region 🔖️Version
/// @emoji 🔢️ The channel wire format's own version, advertised by `AppCommand::Hello` and echoed
/// back by `AppFrame::Welcome` so either side can detect a mismatched build before exchanging any
/// other frame.
pub const CHANNEL_VERSION: u32 = 3;
//#endregion 🔖️Version

//#region 🔖️SectionProbe
/// @emoji 🔍️ One UI-section cache probe: `AppCommand::RefreshUi` sends a batch of these so the
/// engine can skip re-sending a `AppFrame::UiSection` body the client's `hash` already matches.
#[derive(Clone, Debug, PartialEq)]
pub struct SectionProbe {
    pub kind: u8,
    pub key: String,
    pub hash: Option<u64>,
}
//#endregion 🔖️SectionProbe

//#region 🔖️AppCommand
/// @emoji 📨️ One frame a client (UI or headless runner) sends to the app engine.
#[derive(Clone, Debug, PartialEq)]
pub enum AppCommand {
    Hello {
        channel_version: u32,
        app_id: String,
        actor: String,
        config: Vec<u8>,
    },
    ConfigCommand {
        seq: u64,
        command: Vec<u8>,
    },
    Command {
        seq: u64,
        command: Vec<u8>,
        /// 🗣️ Packed `ViewState` (see `store::pack_rt`) the client wants this command evaluated against.
        view_state: Vec<u8>,
    },
    CommandText {
        seq: u64,
        line: String,
    },
    RefreshUi {
        seq: u64,
        sections: Vec<SectionProbe>,
        /// 🗣️ Packed `ViewState` for this refresh — locale/terminology/active-utility must arrive before any
        /// Command, otherwise first-paint `app_labels` resolve against `ViewState::default()`.
        view_state: Vec<u8>,
    },
    ContextMenu {
        seq: u64,
        request: Vec<u8>,
    },
    DocumentCommand {
        seq: u64,
        command: Vec<u8>,
    },
    ApplyEnvelopes {
        seq: u64,
        envelopes: Vec<protocol_causal::OperationEnvelope>,
    },
    LoadDocument {
        seq: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
    },
    ReadDocument {
        seq: u64,
    },
    LoadConfig {
        seq: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
    },
    ReadConfig {
        seq: u64,
    },
    AttachBackbone {
        seq: u64,
        uri: String,
    },
    DetachBackbone {
        seq: u64,
    },
    MediaIn {
        seq: u64,
        port: String,
        descriptor: Vec<u8>,
        data: Vec<u8>,
    },
    MediaOut {
        seq: u64,
        port: String,
        request: Vec<u8>,
    },
    MediaFingerprint {
        seq: u64,
        port: String,
    },
    Bye,
}
//#endregion 🔖️AppCommand

//#region 🔖️AppFrame
/// @emoji 📬️ One frame the app engine sends to its client.
#[derive(Clone, Debug, PartialEq)]
pub enum AppFrame {
    Welcome {
        channel_version: u32,
        instance: u32,
        manifest: Vec<u8>,
    },
    Done {
        in_reply_to: u64,
    },
    Invocation {
        in_reply_to: u64,
        output: Vec<u8>,
        diagnostics: Vec<u8>,
    },
    UiSection {
        in_reply_to: Option<u64>,
        kind: u8,
        key: String,
        hash: u64,
        body: Option<Vec<u8>>,
    },
    Effects {
        in_reply_to: Option<u64>,
        effects: Vec<Vec<u8>>,
    },
    Events {
        in_reply_to: Option<u64>,
        events: Vec<Vec<u8>>,
    },
    DocumentChanged {
        envelopes: Vec<protocol_causal::OperationEnvelope>,
        origin: String,
    },
    Document {
        in_reply_to: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
        ops: String,
    },
    Config {
        in_reply_to: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
        ops: String,
    },
    ConfigChanged {
        envelopes: Vec<protocol_causal::OperationEnvelope>,
        origin: String,
    },
    ContextMenu {
        in_reply_to: u64,
        items: Vec<u8>,
    },
    Media {
        in_reply_to: u64,
        port: String,
        descriptor: Vec<u8>,
        data: Vec<u8>,
    },
    MediaFingerprint {
        in_reply_to: u64,
        port: String,
        fingerprint: Vec<u8>,
    },
    Error {
        in_reply_to: Option<u64>,
        code: String,
        message: String,
    },
}
//#endregion 🔖️AppFrame

//#region 🔖️Codec
// Hand-rolled binary frame encode/decode: `tag: u8 | fields...` — see the module-level docstring.
// `protocol_core::🔖️WireCodec` supplies the primitives; this region adds the option/vec/
// `SectionProbe` combinators the frame shapes need plus the tag-dispatch match arms.

fn malformed(what: &'static str, offset: u64, detail: &str) -> protocol_core::ProtocolError {
    protocol_core::ProtocolError::Malformed { what, offset, detail: detail.to_string() }
}

//#region 🔖️Combinators
fn write_opt_u64(out: &mut Vec<u8>, value: &Option<u64>) {
    protocol_core::write_bool(out, value.is_some());
    if let Some(v) = value {
        protocol_core::write_varint_u64(out, *v);
    }
}

fn read_opt_u64(bytes: &[u8], pos: &mut usize) -> Result<Option<u64>, protocol_core::ProtocolError> {
    if protocol_core::read_bool(bytes, pos)? { Ok(Some(protocol_core::read_varint_u64(bytes, pos)?)) } else { Ok(None) }
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

fn encode_section_probe(probe: &SectionProbe, out: &mut Vec<u8>) {
    out.push(probe.kind);
    protocol_core::write_str(out, &probe.key);
    write_opt_u64(out, &probe.hash);
}

fn decode_section_probe(bytes: &[u8], pos: &mut usize) -> Result<SectionProbe, protocol_core::ProtocolError> {
    let kind = *bytes.get(*pos).ok_or_else(|| malformed("channel section-probe kind", *pos as u64, "truncated"))?;
    *pos += 1;
    let key = protocol_core::read_str(bytes, pos)?;
    let hash = read_opt_u64(bytes, pos)?;
    Ok(SectionProbe { kind, key, hash })
}

fn write_vec_section_probe(out: &mut Vec<u8>, values: &[SectionProbe]) {
    protocol_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_section_probe(value, out);
    }
}

fn read_vec_section_probe(bytes: &[u8], pos: &mut usize) -> Result<Vec<SectionProbe>, protocol_core::ProtocolError> {
    let count = protocol_core::read_varint_u64(bytes, pos)?;
    (0..count).map(|_| decode_section_probe(bytes, pos)).collect()
}
//#endregion 🔖️Combinators

/// @emoji 📤️ Encodes one `AppCommand`: `tag u8 | fields`.
pub fn encode_app_command(command: &AppCommand) -> Vec<u8> {
    let mut out = Vec::new();
    match command {
        AppCommand::Hello { channel_version, app_id, actor, config } => {
            out.push(0);
            protocol_core::write_varint_u64(&mut out, *channel_version as u64);
            protocol_core::write_str(&mut out, app_id);
            protocol_core::write_str(&mut out, actor);
            protocol_core::write_bytes(&mut out, config);
        }
        AppCommand::ConfigCommand { seq, command } => {
            out.push(1);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_bytes(&mut out, command);
        }
        AppCommand::Command { seq, command, view_state } => {
            out.push(2);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_bytes(&mut out, command);
            protocol_core::write_bytes(&mut out, view_state);
        }
        AppCommand::CommandText { seq, line } => {
            out.push(3);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_str(&mut out, line);
        }
        AppCommand::RefreshUi { seq, sections, view_state } => {
            out.push(4);
            protocol_core::write_varint_u64(&mut out, *seq);
            write_vec_section_probe(&mut out, sections);
            protocol_core::write_bytes(&mut out, view_state);
        }
        AppCommand::ContextMenu { seq, request } => {
            out.push(5);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_bytes(&mut out, request);
        }
        AppCommand::DocumentCommand { seq, command } => {
            out.push(6);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_bytes(&mut out, command);
        }
        AppCommand::ApplyEnvelopes { seq, envelopes } => {
            out.push(7);
            protocol_core::write_varint_u64(&mut out, *seq);
            write_vec_envelope(&mut out, envelopes);
        }
        AppCommand::LoadDocument { seq, pack, spr } => {
            out.push(8);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_bytes(&mut out, pack);
            protocol_core::write_bytes(&mut out, spr);
        }
        AppCommand::ReadDocument { seq } => {
            out.push(9);
            protocol_core::write_varint_u64(&mut out, *seq);
        }
        AppCommand::LoadConfig { seq, pack, spr } => {
            out.push(10);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_bytes(&mut out, pack);
            protocol_core::write_bytes(&mut out, spr);
        }
        AppCommand::ReadConfig { seq } => {
            out.push(11);
            protocol_core::write_varint_u64(&mut out, *seq);
        }
        AppCommand::AttachBackbone { seq, uri } => {
            out.push(12);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_str(&mut out, uri);
        }
        AppCommand::DetachBackbone { seq } => {
            out.push(13);
            protocol_core::write_varint_u64(&mut out, *seq);
        }
        AppCommand::MediaIn { seq, port, descriptor, data } => {
            out.push(14);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_str(&mut out, port);
            protocol_core::write_bytes(&mut out, descriptor);
            protocol_core::write_bytes(&mut out, data);
        }
        AppCommand::MediaOut { seq, port, request } => {
            out.push(15);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_str(&mut out, port);
            protocol_core::write_bytes(&mut out, request);
        }
        AppCommand::MediaFingerprint { seq, port } => {
            out.push(16);
            protocol_core::write_varint_u64(&mut out, *seq);
            protocol_core::write_str(&mut out, port);
        }
        AppCommand::Bye => out.push(17),
    }
    out
}

/// @emoji 📥️ Decodes one `AppCommand`, the inverse of [`encode_app_command`].
pub fn decode_app_command(bytes: &[u8]) -> Result<AppCommand, protocol_core::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-command tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let command = match tag {
        0 => AppCommand::Hello {
            channel_version: protocol_core::read_varint_u64(bytes, &mut pos)? as u32,
            app_id: protocol_core::read_str(bytes, &mut pos)?,
            actor: protocol_core::read_str(bytes, &mut pos)?,
            config: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        1 => AppCommand::ConfigCommand { seq: protocol_core::read_varint_u64(bytes, &mut pos)?, command: protocol_core::read_bytes(bytes, &mut pos)? },
        2 => AppCommand::Command {
            seq: protocol_core::read_varint_u64(bytes, &mut pos)?,
            command: protocol_core::read_bytes(bytes, &mut pos)?,
            view_state: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        3 => AppCommand::CommandText { seq: protocol_core::read_varint_u64(bytes, &mut pos)?, line: protocol_core::read_str(bytes, &mut pos)? },
        4 => AppCommand::RefreshUi {
            seq: protocol_core::read_varint_u64(bytes, &mut pos)?,
            sections: read_vec_section_probe(bytes, &mut pos)?,
            view_state: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        5 => AppCommand::ContextMenu { seq: protocol_core::read_varint_u64(bytes, &mut pos)?, request: protocol_core::read_bytes(bytes, &mut pos)? },
        6 => AppCommand::DocumentCommand { seq: protocol_core::read_varint_u64(bytes, &mut pos)?, command: protocol_core::read_bytes(bytes, &mut pos)? },
        7 => AppCommand::ApplyEnvelopes { seq: protocol_core::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos)? },
        8 => AppCommand::LoadDocument {
            seq: protocol_core::read_varint_u64(bytes, &mut pos)?,
            pack: protocol_core::read_bytes(bytes, &mut pos)?,
            spr: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        9 => AppCommand::ReadDocument { seq: protocol_core::read_varint_u64(bytes, &mut pos)? },
        10 => AppCommand::LoadConfig {
            seq: protocol_core::read_varint_u64(bytes, &mut pos)?,
            pack: protocol_core::read_bytes(bytes, &mut pos)?,
            spr: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        11 => AppCommand::ReadConfig { seq: protocol_core::read_varint_u64(bytes, &mut pos)? },
        12 => AppCommand::AttachBackbone { seq: protocol_core::read_varint_u64(bytes, &mut pos)?, uri: protocol_core::read_str(bytes, &mut pos)? },
        13 => AppCommand::DetachBackbone { seq: protocol_core::read_varint_u64(bytes, &mut pos)? },
        14 => AppCommand::MediaIn {
            seq: protocol_core::read_varint_u64(bytes, &mut pos)?,
            port: protocol_core::read_str(bytes, &mut pos)?,
            descriptor: protocol_core::read_bytes(bytes, &mut pos)?,
            data: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        15 => AppCommand::MediaOut { seq: protocol_core::read_varint_u64(bytes, &mut pos)?, port: protocol_core::read_str(bytes, &mut pos)?, request: protocol_core::read_bytes(bytes, &mut pos)? },
        16 => AppCommand::MediaFingerprint { seq: protocol_core::read_varint_u64(bytes, &mut pos)?, port: protocol_core::read_str(bytes, &mut pos)? },
        17 => AppCommand::Bye,
        other => return Err(malformed("channel app-command tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok(command)
}

/// @emoji 📤️ Encodes one `AppFrame`: `tag u8 | fields`.
pub fn encode_app_frame(frame: &AppFrame) -> Vec<u8> {
    let mut out = Vec::new();
    match frame {
        AppFrame::Welcome { channel_version, instance, manifest } => {
            out.push(0);
            protocol_core::write_varint_u64(&mut out, *channel_version as u64);
            protocol_core::write_varint_u64(&mut out, *instance as u64);
            protocol_core::write_bytes(&mut out, manifest);
        }
        AppFrame::Done { in_reply_to } => {
            out.push(1);
            protocol_core::write_varint_u64(&mut out, *in_reply_to);
        }
        AppFrame::Invocation { in_reply_to, output, diagnostics } => {
            out.push(2);
            protocol_core::write_varint_u64(&mut out, *in_reply_to);
            protocol_core::write_bytes(&mut out, output);
            protocol_core::write_bytes(&mut out, diagnostics);
        }
        AppFrame::UiSection { in_reply_to, kind, key, hash, body } => {
            out.push(3);
            write_opt_u64(&mut out, in_reply_to);
            out.push(*kind);
            protocol_core::write_str(&mut out, key);
            protocol_core::write_varint_u64(&mut out, *hash);
            write_opt_bytes(&mut out, body);
        }
        AppFrame::Effects { in_reply_to, effects } => {
            out.push(4);
            write_opt_u64(&mut out, in_reply_to);
            write_vec_bytes(&mut out, effects);
        }
        AppFrame::Events { in_reply_to, events } => {
            out.push(5);
            write_opt_u64(&mut out, in_reply_to);
            write_vec_bytes(&mut out, events);
        }
        AppFrame::DocumentChanged { envelopes, origin } => {
            out.push(6);
            write_vec_envelope(&mut out, envelopes);
            protocol_core::write_str(&mut out, origin);
        }
        AppFrame::Document { in_reply_to, pack, spr, ops } => {
            out.push(7);
            protocol_core::write_varint_u64(&mut out, *in_reply_to);
            protocol_core::write_bytes(&mut out, pack);
            protocol_core::write_bytes(&mut out, spr);
            protocol_core::write_str(&mut out, ops);
        }
        AppFrame::Config { in_reply_to, pack, spr, ops } => {
            out.push(8);
            protocol_core::write_varint_u64(&mut out, *in_reply_to);
            protocol_core::write_bytes(&mut out, pack);
            protocol_core::write_bytes(&mut out, spr);
            protocol_core::write_str(&mut out, ops);
        }
        AppFrame::ConfigChanged { envelopes, origin } => {
            out.push(9);
            write_vec_envelope(&mut out, envelopes);
            protocol_core::write_str(&mut out, origin);
        }
        AppFrame::ContextMenu { in_reply_to, items } => {
            out.push(10);
            protocol_core::write_varint_u64(&mut out, *in_reply_to);
            protocol_core::write_bytes(&mut out, items);
        }
        AppFrame::Media { in_reply_to, port, descriptor, data } => {
            out.push(11);
            protocol_core::write_varint_u64(&mut out, *in_reply_to);
            protocol_core::write_str(&mut out, port);
            protocol_core::write_bytes(&mut out, descriptor);
            protocol_core::write_bytes(&mut out, data);
        }
        AppFrame::MediaFingerprint { in_reply_to, port, fingerprint } => {
            out.push(12);
            protocol_core::write_varint_u64(&mut out, *in_reply_to);
            protocol_core::write_str(&mut out, port);
            protocol_core::write_bytes(&mut out, fingerprint);
        }
        AppFrame::Error { in_reply_to, code, message } => {
            out.push(13);
            write_opt_u64(&mut out, in_reply_to);
            protocol_core::write_str(&mut out, code);
            protocol_core::write_str(&mut out, message);
        }
    }
    out
}

/// @emoji 📥️ Decodes one `AppFrame`, the inverse of [`encode_app_frame`].
pub fn decode_app_frame(bytes: &[u8]) -> Result<AppFrame, protocol_core::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-frame tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let frame = match tag {
        0 => AppFrame::Welcome {
            channel_version: protocol_core::read_varint_u64(bytes, &mut pos)? as u32,
            instance: protocol_core::read_varint_u64(bytes, &mut pos)? as u32,
            manifest: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        1 => AppFrame::Done { in_reply_to: protocol_core::read_varint_u64(bytes, &mut pos)? },
        2 => AppFrame::Invocation {
            in_reply_to: protocol_core::read_varint_u64(bytes, &mut pos)?,
            output: protocol_core::read_bytes(bytes, &mut pos)?,
            diagnostics: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        3 => {
            let in_reply_to = read_opt_u64(bytes, &mut pos)?;
            let kind = *bytes.get(pos).ok_or_else(|| malformed("channel ui-section kind", pos as u64, "truncated"))?;
            pos += 1;
            let key = protocol_core::read_str(bytes, &mut pos)?;
            let hash = protocol_core::read_varint_u64(bytes, &mut pos)?;
            let body = read_opt_bytes(bytes, &mut pos)?;
            AppFrame::UiSection { in_reply_to, kind, key, hash, body }
        }
        4 => AppFrame::Effects { in_reply_to: read_opt_u64(bytes, &mut pos)?, effects: read_vec_bytes(bytes, &mut pos)? },
        5 => AppFrame::Events { in_reply_to: read_opt_u64(bytes, &mut pos)?, events: read_vec_bytes(bytes, &mut pos)? },
        6 => AppFrame::DocumentChanged { envelopes: read_vec_envelope(bytes, &mut pos)?, origin: protocol_core::read_str(bytes, &mut pos)? },
        7 => AppFrame::Document {
            in_reply_to: protocol_core::read_varint_u64(bytes, &mut pos)?,
            pack: protocol_core::read_bytes(bytes, &mut pos)?,
            spr: protocol_core::read_bytes(bytes, &mut pos)?,
            ops: protocol_core::read_str(bytes, &mut pos)?,
        },
        8 => AppFrame::Config {
            in_reply_to: protocol_core::read_varint_u64(bytes, &mut pos)?,
            pack: protocol_core::read_bytes(bytes, &mut pos)?,
            spr: protocol_core::read_bytes(bytes, &mut pos)?,
            ops: protocol_core::read_str(bytes, &mut pos)?,
        },
        9 => AppFrame::ConfigChanged { envelopes: read_vec_envelope(bytes, &mut pos)?, origin: protocol_core::read_str(bytes, &mut pos)? },
        10 => AppFrame::ContextMenu { in_reply_to: protocol_core::read_varint_u64(bytes, &mut pos)?, items: protocol_core::read_bytes(bytes, &mut pos)? },
        11 => AppFrame::Media {
            in_reply_to: protocol_core::read_varint_u64(bytes, &mut pos)?,
            port: protocol_core::read_str(bytes, &mut pos)?,
            descriptor: protocol_core::read_bytes(bytes, &mut pos)?,
            data: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        12 => AppFrame::MediaFingerprint {
            in_reply_to: protocol_core::read_varint_u64(bytes, &mut pos)?,
            port: protocol_core::read_str(bytes, &mut pos)?,
            fingerprint: protocol_core::read_bytes(bytes, &mut pos)?,
        },
        13 => AppFrame::Error { in_reply_to: read_opt_u64(bytes, &mut pos)?, code: protocol_core::read_str(bytes, &mut pos)?, message: protocol_core::read_str(bytes, &mut pos)? },
        other => return Err(malformed("channel app-frame tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok(frame)
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

    /// @emoji #️⃣ Tiny hand-rolled `&[u8] -> String` hex encoder for this crate's own fixture-corpus
    /// tests — mirrors `db_engine`'s `write!("{byte:02x}")` idiom (no `hex` crate dependency exists
    /// anywhere in `framework/product/os`, so this crate does not introduce one either).
    fn hex_encode(bytes: &[u8]) -> String {
        use std::fmt::Write;
        let mut out = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            let _ = write!(out, "{byte:02x}");
        }
        out
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️AppCommand
    fn assert_command_round_trips(command: &AppCommand) {
        let bytes = encode_app_command(command);
        let decoded = decode_app_command(&bytes).expect("decode must succeed");
        assert_eq!(&decoded, command);
    }

    #[test]
    fn app_command_hello_round_trips() {
        assert_command_round_trips(&AppCommand::Hello { channel_version: CHANNEL_VERSION, app_id: "app-1".to_string(), actor: "actor-1".to_string(), config: vec![1, 2, 3] });
    }

    #[test]
    fn app_command_config_command_round_trips() {
        assert_command_round_trips(&AppCommand::ConfigCommand { seq: 1, command: vec![9, 9] });
    }

    #[test]
    fn app_command_command_round_trips() {
        assert_command_round_trips(&AppCommand::Command { seq: 2, command: vec![1, 2], view_state: vec![] });
    }

    #[test]
    fn app_command_command_text_round_trips() {
        assert_command_round_trips(&AppCommand::CommandText { seq: 3, line: "set foo = 1".to_string() });
    }

    #[test]
    fn app_command_refresh_ui_round_trips() {
        assert_command_round_trips(&AppCommand::RefreshUi {
            seq: 4,
            sections: vec![SectionProbe { kind: 1, key: "outline".to_string(), hash: Some(42) }, SectionProbe { kind: 2, key: "inspector".to_string(), hash: None }],
            view_state: vec![],
        });
    }

    #[test]
    fn app_command_context_menu_round_trips() {
        assert_command_round_trips(&AppCommand::ContextMenu { seq: 5, request: vec![7] });
    }

    #[test]
    fn app_command_document_command_round_trips() {
        assert_command_round_trips(&AppCommand::DocumentCommand { seq: 6, command: vec![8, 8] });
    }

    #[test]
    fn app_command_apply_envelopes_round_trips() {
        assert_command_round_trips(&AppCommand::ApplyEnvelopes { seq: 7, envelopes: vec![sample_envelope("op-1"), sample_envelope("op-2")] });
    }

    #[test]
    fn app_command_load_document_round_trips() {
        assert_command_round_trips(&AppCommand::LoadDocument { seq: 8, pack: vec![1], spr: vec![2] });
    }

    #[test]
    fn app_command_read_document_round_trips() {
        assert_command_round_trips(&AppCommand::ReadDocument { seq: 9 });
    }

    #[test]
    fn app_command_load_config_round_trips() {
        assert_command_round_trips(&AppCommand::LoadConfig { seq: 10, pack: vec![1], spr: vec![2] });
    }

    #[test]
    fn app_command_read_config_round_trips() {
        assert_command_round_trips(&AppCommand::ReadConfig { seq: 11 });
    }

    #[test]
    fn app_command_attach_backbone_round_trips() {
        assert_command_round_trips(&AppCommand::AttachBackbone { seq: 12, uri: "backbone://host/doc".to_string() });
    }

    #[test]
    fn app_command_detach_backbone_round_trips() {
        assert_command_round_trips(&AppCommand::DetachBackbone { seq: 13 });
    }

    #[test]
    fn app_command_media_in_round_trips() {
        assert_command_round_trips(&AppCommand::MediaIn { seq: 14, port: "camera".to_string(), descriptor: vec![1], data: vec![2, 3] });
    }

    #[test]
    fn app_command_media_out_round_trips() {
        assert_command_round_trips(&AppCommand::MediaOut { seq: 15, port: "speaker".to_string(), request: vec![4] });
    }

    #[test]
    fn app_command_media_fingerprint_round_trips() {
        assert_command_round_trips(&AppCommand::MediaFingerprint { seq: 16, port: "camera".to_string() });
    }

    #[test]
    fn app_command_bye_round_trips() {
        assert_command_round_trips(&AppCommand::Bye);
    }
    //#endregion 🔖️AppCommand

    //#region 🔖️AppFrame
    fn assert_frame_round_trips(frame: &AppFrame) {
        let bytes = encode_app_frame(frame);
        let decoded = decode_app_frame(&bytes).expect("decode must succeed");
        assert_eq!(&decoded, frame);
    }

    #[test]
    fn app_frame_welcome_round_trips() {
        assert_frame_round_trips(&AppFrame::Welcome { channel_version: CHANNEL_VERSION, instance: 1, manifest: vec![1, 2, 3] });
    }

    #[test]
    fn app_frame_done_round_trips() {
        assert_frame_round_trips(&AppFrame::Done { in_reply_to: 1 });
    }

    #[test]
    fn app_frame_invocation_round_trips() {
        assert_frame_round_trips(&AppFrame::Invocation { in_reply_to: 2, output: vec![1], diagnostics: vec![2] });
    }

    #[test]
    fn app_frame_ui_section_round_trips_with_and_without_optionals() {
        assert_frame_round_trips(&AppFrame::UiSection { in_reply_to: Some(3), kind: 1, key: "outline".to_string(), hash: 99, body: Some(vec![1, 2]) });
        assert_frame_round_trips(&AppFrame::UiSection { in_reply_to: None, kind: 1, key: "outline".to_string(), hash: 99, body: None });
    }

    #[test]
    fn app_frame_effects_round_trips() {
        assert_frame_round_trips(&AppFrame::Effects { in_reply_to: Some(4), effects: vec![vec![1], vec![2, 2]] });
    }

    #[test]
    fn app_frame_events_round_trips() {
        assert_frame_round_trips(&AppFrame::Events { in_reply_to: None, events: vec![vec![9]] });
    }

    #[test]
    fn app_frame_document_changed_round_trips() {
        assert_frame_round_trips(&AppFrame::DocumentChanged { envelopes: vec![sample_envelope("op-1")], origin: "peer-1".to_string() });
    }

    #[test]
    fn app_frame_document_round_trips() {
        assert_frame_round_trips(&AppFrame::Document { in_reply_to: 5, pack: vec![1], spr: vec![2], ops: "set foo = 1".to_string() });
    }

    #[test]
    fn app_frame_config_round_trips() {
        assert_frame_round_trips(&AppFrame::Config { in_reply_to: 5, pack: vec![1], spr: vec![2], ops: "set cam = 1".to_string() });
    }

    #[test]
    fn app_frame_config_changed_round_trips() {
        assert_frame_round_trips(&AppFrame::ConfigChanged { envelopes: vec![sample_envelope("cfg-1")], origin: "peer-1".to_string() });
    }

    #[test]
    fn app_frame_context_menu_round_trips() {
        assert_frame_round_trips(&AppFrame::ContextMenu { in_reply_to: 6, items: vec![1, 2, 3] });
    }

    #[test]
    fn app_frame_media_round_trips() {
        assert_frame_round_trips(&AppFrame::Media { in_reply_to: 7, port: "camera".to_string(), descriptor: vec![1], data: vec![2] });
    }

    #[test]
    fn app_frame_media_fingerprint_round_trips() {
        assert_frame_round_trips(&AppFrame::MediaFingerprint { in_reply_to: 8, port: "camera".to_string(), fingerprint: vec![1, 2] });
    }

    #[test]
    fn app_frame_error_round_trips() {
        assert_frame_round_trips(&AppFrame::Error { in_reply_to: Some(9), code: "rejected".to_string(), message: "bad command".to_string() });
        assert_frame_round_trips(&AppFrame::Error { in_reply_to: None, code: "rejected".to_string(), message: "bad command".to_string() });
    }
    //#endregion 🔖️AppFrame

    //#region 🔖️SectionProbe
    #[test]
    fn section_probe_round_trips_standalone_and_with_and_without_hash() {
        for probe in [SectionProbe { kind: 3, key: "tree".to_string(), hash: Some(7) }, SectionProbe { kind: 3, key: "tree".to_string(), hash: None }] {
            let mut out = Vec::new();
            encode_section_probe(&probe, &mut out);
            let mut pos = 0;
            assert_eq!(decode_section_probe(&out, &mut pos).unwrap(), probe);
            assert_eq!(pos, out.len());
        }
    }
    //#endregion 🔖️SectionProbe

    //#region 🔖️Codec
    #[test]
    fn encoding_is_deterministic() {
        let command = AppCommand::RefreshUi { seq: 1, sections: vec![SectionProbe { kind: 1, key: "a".to_string(), hash: Some(1) }], view_state: vec![] };
        assert_eq!(encode_app_command(&command), encode_app_command(&command));

        let frame = AppFrame::Error { in_reply_to: Some(1), code: "e".to_string(), message: "m".to_string() };
        assert_eq!(encode_app_frame(&frame), encode_app_frame(&frame));
    }

    #[test]
    fn decode_app_command_rejects_empty_bytes() {
        let err = decode_app_command(&[]).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "channel app-command tag", .. }));
    }

    #[test]
    fn decode_app_frame_rejects_empty_bytes() {
        let err = decode_app_frame(&[]).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
    }

    #[test]
    fn decode_app_command_rejects_unknown_tag() {
        let err = decode_app_command(&[0xFF]).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "channel app-command tag", .. }));
    }

    #[test]
    fn decode_app_frame_rejects_unknown_tag() {
        let err = decode_app_frame(&[0xFF]).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
    }

    #[test]
    fn decode_app_command_rejects_truncated_field() {
        let bytes = encode_app_command(&AppCommand::CommandText { seq: 1, line: "hello".to_string() });
        let truncated = &bytes[..bytes.len() - 2];
        assert!(decode_app_command(truncated).is_err());
    }

    #[test]
    fn decode_app_frame_rejects_truncated_field() {
        let bytes = encode_app_frame(&AppFrame::Error { in_reply_to: Some(1), code: "e".to_string(), message: "message".to_string() });
        let truncated = &bytes[..bytes.len() - 2];
        assert!(decode_app_frame(truncated).is_err());
    }

    #[test]
    fn decode_app_command_never_panics_on_arbitrary_short_buffers() {
        for len in 0..8 {
            let buf = vec![0u8; len];
            let _ = decode_app_command(&buf);
        }
    }

    #[test]
    fn decode_app_frame_never_panics_on_arbitrary_short_buffers() {
        for len in 0..8 {
            let buf = vec![0u8; len];
            let _ = decode_app_frame(&buf);
        }
    }
    //#endregion 🔖️Codec

    //#region 🔖️Corpus
    // Cross-language drift fixture: a sibling TypeScript work package duplicates these exact hex
    // strings in a vitest suite, so `AppCommand`/`AppFrame` and the TS-side codec they hand-port
    // stay byte-exact. Every entry is `(variant label, value)`; `channel_command_fixture_hex`/
    // `channel_frame_fixture_hex` below are this codec's own committed golden hex per label —
    // sourced from `encode_app_command`/`encode_app_frame`'s actual output, not hand-computed.

    /// @emoji 🧾️ Named `AppCommand` fixture corpus, one entry per variant.
    fn channel_command_fixture_corpus() -> Vec<(&'static str, AppCommand)> {
        vec![
            ("Hello", AppCommand::Hello { channel_version: CHANNEL_VERSION, app_id: "app".to_string(), actor: "actor".to_string(), config: vec![1, 2] }),
            ("ConfigCommand", AppCommand::ConfigCommand { seq: 1, command: vec![9] }),
            ("Command", AppCommand::Command { seq: 1, command: vec![1], view_state: vec![] }),
            ("CommandText", AppCommand::CommandText { seq: 1, line: "go".to_string() }),
            ("RefreshUi", AppCommand::RefreshUi { seq: 1, sections: vec![SectionProbe { kind: 1, key: "a".to_string(), hash: Some(1) }], view_state: vec![] }),
            ("ContextMenu", AppCommand::ContextMenu { seq: 1, request: vec![1] }),
            ("DocumentCommand", AppCommand::DocumentCommand { seq: 1, command: vec![1] }),
            ("ApplyEnvelopes", AppCommand::ApplyEnvelopes { seq: 1, envelopes: Vec::new() }),
            ("LoadDocument", AppCommand::LoadDocument { seq: 1, pack: vec![1], spr: vec![2] }),
            ("ReadDocument", AppCommand::ReadDocument { seq: 1 }),
            ("LoadConfig", AppCommand::LoadConfig { seq: 1, pack: vec![1], spr: vec![2] }),
            ("ReadConfig", AppCommand::ReadConfig { seq: 1 }),
            ("AttachBackbone", AppCommand::AttachBackbone { seq: 1, uri: "u".to_string() }),
            ("DetachBackbone", AppCommand::DetachBackbone { seq: 1 }),
            ("MediaIn", AppCommand::MediaIn { seq: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
            ("MediaOut", AppCommand::MediaOut { seq: 1, port: "p".to_string(), request: vec![1] }),
            ("MediaFingerprint", AppCommand::MediaFingerprint { seq: 1, port: "p".to_string() }),
            ("Bye", AppCommand::Bye),
        ]
    }

    /// @emoji 🧾️ Named `AppFrame` fixture corpus, one entry per variant.
    fn channel_frame_fixture_corpus() -> Vec<(&'static str, AppFrame)> {
        vec![
            ("Welcome", AppFrame::Welcome { channel_version: CHANNEL_VERSION, instance: 1, manifest: vec![1] }),
            ("Done", AppFrame::Done { in_reply_to: 1 }),
            ("Invocation", AppFrame::Invocation { in_reply_to: 1, output: vec![1], diagnostics: vec![] }),
            ("UiSection", AppFrame::UiSection { in_reply_to: Some(1), kind: 1, key: "k".to_string(), hash: 1, body: None }),
            ("Effects", AppFrame::Effects { in_reply_to: None, effects: vec![vec![1]] }),
            ("Events", AppFrame::Events { in_reply_to: None, events: vec![] }),
            ("DocumentChanged", AppFrame::DocumentChanged { envelopes: vec![], origin: "o".to_string() }),
            ("Document", AppFrame::Document { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "o".to_string() }),
            ("Config", AppFrame::Config { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "c".to_string() }),
            ("ConfigChanged", AppFrame::ConfigChanged { envelopes: vec![], origin: "o".to_string() }),
            ("ContextMenu", AppFrame::ContextMenu { in_reply_to: 1, items: vec![1] }),
            ("Media", AppFrame::Media { in_reply_to: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
            ("MediaFingerprint", AppFrame::MediaFingerprint { in_reply_to: 1, port: "p".to_string(), fingerprint: vec![1] }),
            ("Error", AppFrame::Error { in_reply_to: None, code: "c".to_string(), message: "m".to_string() }),
        ]
    }

    /// @emoji 🔒️ Golden hex per `AppCommand` fixture-corpus label — sourced by actually running
    /// `encode_app_command` over `channel_command_fixture_corpus()` (never hand-computed), then
    /// committed here as the drift guard: any future codec change that shifts these bytes fails
    /// this test, forcing a deliberate update of both this table and the TS-side twin (WP-0B).
    fn channel_command_fixture_hex(label: &str) -> &'static str {
        match label {
            "Hello" => "000303617070056163746f72020102",
            "ConfigCommand" => "01010109",
            "Command" => "0201010100",
            "CommandText" => "030102676f",
            "RefreshUi" => "040101010161010100",
            "ContextMenu" => "05010101",
            "DocumentCommand" => "06010101",
            "ApplyEnvelopes" => "070100",
            "LoadDocument" => "080101010102",
            "ReadDocument" => "0901",
            "LoadConfig" => "0a0101010102",
            "ReadConfig" => "0b01",
            "AttachBackbone" => "0c010175",
            "DetachBackbone" => "0d01",
            "MediaIn" => "0e01017001010102",
            "MediaOut" => "0f0101700101",
            "MediaFingerprint" => "10010170",
            "Bye" => "11",
            other => panic!("channel_command_fixture_hex: no golden hex registered for label {other:?}"),
        }
    }

    /// @emoji 🔒️ Golden hex per `AppFrame` fixture-corpus label — see
    /// `channel_command_fixture_hex`'s docstring for provenance/drift-guard rationale.
    fn channel_frame_fixture_hex(label: &str) -> &'static str {
        match label {
            "Welcome" => "0003010101",
            "Done" => "0101",
            "Invocation" => "0201010100",
            "UiSection" => "03010101016b0100",
            "Effects" => "0400010101",
            "Events" => "050000",
            "DocumentChanged" => "0600016f",
            "Document" => "070101010102016f",
            "Config" => "0801010101020163",
            "ConfigChanged" => "0900016f",
            "ContextMenu" => "0a010101",
            "Media" => "0b01017001010102",
            "MediaFingerprint" => "0c0101700101",
            "Error" => "0d000163016d",
            other => panic!("channel_frame_fixture_hex: no golden hex registered for label {other:?}"),
        }
    }

    #[test]
    fn app_command_fixture_corpus_matches_golden_hex_and_round_trips() {
        for (label, value) in channel_command_fixture_corpus() {
            let actual = hex_encode(&encode_app_command(&value));
            println!("[DEBUG] AppCommand::{label} = {actual}");
            assert_eq!(actual, channel_command_fixture_hex(label), "{label}'s encoding drifted from its committed golden hex");
            let decoded = decode_app_command(&encode_app_command(&value)).unwrap();
            assert_eq!(decoded, value, "{label} must round-trip");
        }
    }

    #[test]
    fn app_frame_fixture_corpus_matches_golden_hex_and_round_trips() {
        for (label, value) in channel_frame_fixture_corpus() {
            let actual = hex_encode(&encode_app_frame(&value));
            println!("[DEBUG] AppFrame::{label} = {actual}");
            assert_eq!(actual, channel_frame_fixture_hex(label), "{label}'s encoding drifted from its committed golden hex");
            let decoded = decode_app_frame(&encode_app_frame(&value)).unwrap();
            assert_eq!(decoded, value, "{label} must round-trip");
        }
    }
    //#endregion 🔖️Corpus
}
//#endregion 🧪️Tests
