//! 🧵️ `ShellBridge` frame codec (`📋️master.md` §2.2 verbatim variant list, `📌️sol-P1b-packet.md`
//! §2.4) — hand-rolled binary framing (`tag: u8` + fields in declaration order), `BRIDGE_VERSION = 1`.
//! This file is the Rust SSOT; `🟦️component.ts` is the TS twin, and `🧫️fixtures/frames.json` is the
//! anti-drift mechanism proving both codecs agree byte-for-byte (`mod quick`'s
//! `every_fixture_round_trips_through_the_rust_codec` test; the TS side of the same fixtures is
//! exercised by a foreground `bun run` script, not wired into this crate's own test run).
//!
//! Field types are hand-picked for a self-contained wire format: this facet deliberately does NOT
//! depend on the peer `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket's `os_spr::wire` primitives
//! (`📡️spr/🧵️channel/🦀️component.rs` is their exclusive, mid-rewrite territory per
//! `📌️important.md`'s collision matrix) — see this packet's report §7 for the full rationale. Opaque
//! payloads (`ShellState.state`, `ShellStatePatch.patch`, `ShellCommand.command`,
//! `AppCommand.command`, each `AppFrames.frames[i]`) are carried as length-prefixed byte blobs; what
//! goes inside them (the real packed `ShellState`) is `💻️os/🔨️modules/🖥️shell`'s concern (P9), not
//! this codec's.

use crate::errors::{GatewayError, GatewayErrorCode};
use serde::{Deserialize, Serialize};

//#region 🔖️BridgeVersion
pub const BRIDGE_VERSION: u16 = 1;
//#endregion 🔖️BridgeVersion

//#region 🔖️Wire
/// 🧵️ Minimal length-prefixed little-endian primitives shared by both frame enums' hand-rolled
/// `encode`/`decode` — private to this file; the TS twin (`🟦️component.ts`) mirrors the exact same
/// byte layout independently (that agreement is what the fixtures prove).
mod wire {
    use crate::errors::{GatewayError, GatewayErrorCode};

    pub fn write_u8(buf: &mut Vec<u8>, value: u8) {
        buf.push(value);
    }
    pub fn write_u16(buf: &mut Vec<u8>, value: u16) {
        buf.extend_from_slice(&value.to_le_bytes());
    }
    pub fn write_u32(buf: &mut Vec<u8>, value: u32) {
        buf.extend_from_slice(&value.to_le_bytes());
    }
    pub fn write_u64(buf: &mut Vec<u8>, value: u64) {
        buf.extend_from_slice(&value.to_le_bytes());
    }
    pub fn write_bool(buf: &mut Vec<u8>, value: bool) {
        buf.push(value as u8);
    }
    pub fn write_bytes(buf: &mut Vec<u8>, value: &[u8]) {
        write_u32(buf, value.len() as u32);
        buf.extend_from_slice(value);
    }
    pub fn write_string(buf: &mut Vec<u8>, value: &str) {
        write_bytes(buf, value.as_bytes());
    }
    pub fn write_option_string(buf: &mut Vec<u8>, value: &Option<String>) {
        match value {
            Some(inner) => {
                write_bool(buf, true);
                write_string(buf, inner);
            }
            None => write_bool(buf, false),
        }
    }
    pub fn write_string_vec(buf: &mut Vec<u8>, value: &[String]) {
        write_u32(buf, value.len() as u32);
        for item in value {
            write_string(buf, item);
        }
    }
    pub fn write_bytes_vec(buf: &mut Vec<u8>, value: &[Vec<u8>]) {
        write_u32(buf, value.len() as u32);
        for item in value {
            write_bytes(buf, item);
        }
    }

    /// 📖️ A forward-only cursor over a decode buffer — every read is bounds-checked, `finish()`
    /// rejects trailing bytes so a truncated OR over-long frame is always a decode error, never a
    /// silent partial parse.
    pub struct Reader<'a> {
        data: &'a [u8],
        pos: usize,
    }

    impl<'a> Reader<'a> {
        pub fn new(data: &'a [u8]) -> Self {
            Self { data, pos: 0 }
        }

        fn need(&self, count: usize) -> Result<(), GatewayError> {
            if self.pos + count > self.data.len() {
                Err(GatewayError::new(GatewayErrorCode::InputInvalid, "bridge frame: unexpected end of buffer"))
            } else {
                Ok(())
            }
        }

        pub fn read_u8(&mut self) -> Result<u8, GatewayError> {
            self.need(1)?;
            let value = self.data[self.pos];
            self.pos += 1;
            Ok(value)
        }
        pub fn read_u16(&mut self) -> Result<u16, GatewayError> {
            self.need(2)?;
            let value = u16::from_le_bytes(self.data[self.pos..self.pos + 2].try_into().expect("checked length"));
            self.pos += 2;
            Ok(value)
        }
        pub fn read_u32(&mut self) -> Result<u32, GatewayError> {
            self.need(4)?;
            let value = u32::from_le_bytes(self.data[self.pos..self.pos + 4].try_into().expect("checked length"));
            self.pos += 4;
            Ok(value)
        }
        pub fn read_u64(&mut self) -> Result<u64, GatewayError> {
            self.need(8)?;
            let value = u64::from_le_bytes(self.data[self.pos..self.pos + 8].try_into().expect("checked length"));
            self.pos += 8;
            Ok(value)
        }
        pub fn read_bool(&mut self) -> Result<bool, GatewayError> {
            Ok(self.read_u8()? != 0)
        }
        pub fn read_bytes(&mut self) -> Result<Vec<u8>, GatewayError> {
            let len = self.read_u32()? as usize;
            self.need(len)?;
            let value = self.data[self.pos..self.pos + len].to_vec();
            self.pos += len;
            Ok(value)
        }
        pub fn read_string(&mut self) -> Result<String, GatewayError> {
            let bytes = self.read_bytes()?;
            String::from_utf8(bytes).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: invalid utf8: {error}")))
        }
        pub fn read_option_string(&mut self) -> Result<Option<String>, GatewayError> {
            if self.read_bool()? {
                Ok(Some(self.read_string()?))
            } else {
                Ok(None)
            }
        }
        pub fn read_string_vec(&mut self) -> Result<Vec<String>, GatewayError> {
            let len = self.read_u32()? as usize;
            (0..len).map(|_| self.read_string()).collect()
        }
        pub fn read_bytes_vec(&mut self) -> Result<Vec<Vec<u8>>, GatewayError> {
            let len = self.read_u32()? as usize;
            (0..len).map(|_| self.read_bytes()).collect()
        }
        pub fn finish(self) -> Result<(), GatewayError> {
            if self.pos == self.data.len() {
                Ok(())
            } else {
                Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: {} trailing byte(s) after decode", self.data.len() - self.pos)))
            }
        }
    }
}
//#endregion 🔖️Wire

//#region 🔖️SharedTypes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShellKind {
    React,
    WgpuWeb,
    WgpuNative,
}

impl ShellKind {
    fn to_tag(self) -> u8 {
        match self {
            ShellKind::React => 0,
            ShellKind::WgpuWeb => 1,
            ShellKind::WgpuNative => 2,
        }
    }
    fn from_tag(tag: u8) -> Result<Self, GatewayError> {
        match tag {
            0 => Ok(ShellKind::React),
            1 => Ok(ShellKind::WgpuWeb),
            2 => Ok(ShellKind::WgpuNative),
            other => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ShellKind tag {other}"))),
        }
    }
}

/// 🚩️ `RelayAppCommands|SharedBackbone|Elicit` — a bitmask on the wire (`to_bits`/`from_bits`), a
/// small struct in Rust/JSON so fixtures and call sites read as named booleans, not magic numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeFlags {
    pub relay_app_commands: bool,
    pub shared_backbone: bool,
    pub elicit: bool,
}

impl BridgeFlags {
    pub const NONE: Self = Self { relay_app_commands: false, shared_backbone: false, elicit: false };

    fn to_bits(self) -> u8 {
        (self.relay_app_commands as u8) | ((self.shared_backbone as u8) << 1) | ((self.elicit as u8) << 2)
    }
    fn from_bits(bits: u8) -> Self {
        Self { relay_app_commands: bits & 0b001 != 0, shared_backbone: bits & 0b010 != 0, elicit: bits & 0b100 != 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecision {
    Deny,
    Once,
    Session,
}

impl ApprovalDecision {
    fn to_tag(self) -> u8 {
        match self {
            ApprovalDecision::Deny => 0,
            ApprovalDecision::Once => 1,
            ApprovalDecision::Session => 2,
        }
    }
    fn from_tag(tag: u8) -> Result<Self, GatewayError> {
        match tag {
            0 => Ok(ApprovalDecision::Deny),
            1 => Ok(ApprovalDecision::Once),
            2 => Ok(ApprovalDecision::Session),
            other => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ApprovalDecision tag {other}"))),
        }
    }
}

/// 📇️ One entry of `Instances{entries}` — `BridgeInstanceRef{plugin_id, app_id, instance_id,
/// artifact_ref, window_ids}` verbatim from `📋️master.md` §2.2.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeInstanceRef {
    pub plugin_id: String,
    pub app_id: String,
    pub instance_id: String,
    pub artifact_ref: String,
    pub window_ids: Vec<String>,
}

impl BridgeInstanceRef {
    fn encode(&self, buf: &mut Vec<u8>) {
        wire::write_string(buf, &self.plugin_id);
        wire::write_string(buf, &self.app_id);
        wire::write_string(buf, &self.instance_id);
        wire::write_string(buf, &self.artifact_ref);
        wire::write_string_vec(buf, &self.window_ids);
    }
    fn decode(reader: &mut wire::Reader<'_>) -> Result<Self, GatewayError> {
        Ok(Self { plugin_id: reader.read_string()?, app_id: reader.read_string()?, instance_id: reader.read_string()?, artifact_ref: reader.read_string()?, window_ids: reader.read_string_vec()? })
    }
}
//#endregion 🔖️SharedTypes

//#region 🔖️ShellToGateway
/// 📨️ Shell→Gateway frames, tag 0..8 in this exact declaration order (`📋️master.md` §2.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellToGateway {
    Hello { bridge_version: u16, shell_kind: ShellKind, shell_session_id: String, principal_actor: String, flags: BridgeFlags },
    ShellState { revision: u64, state: Vec<u8> },
    ShellStatePatch { revision: u64, base_revision: u64, patch: Vec<u8> },
    Instances { entries: Vec<BridgeInstanceRef> },
    AppFrames { in_reply_to: u64, instance_id: String, frames: Vec<Vec<u8>> },
    ShellCommandResult { in_reply_to: u64, ok: bool, fault: Option<String> },
    Approval { approval_id: String, decision: ApprovalDecision, note: Option<String> },
    Ping,
    Bye,
}

impl ShellToGateway {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            ShellToGateway::Hello { bridge_version, shell_kind, shell_session_id, principal_actor, flags } => {
                wire::write_u8(&mut buf, 0);
                wire::write_u16(&mut buf, *bridge_version);
                wire::write_u8(&mut buf, shell_kind.to_tag());
                wire::write_string(&mut buf, shell_session_id);
                wire::write_string(&mut buf, principal_actor);
                wire::write_u8(&mut buf, flags.to_bits());
            }
            ShellToGateway::ShellState { revision, state } => {
                wire::write_u8(&mut buf, 1);
                wire::write_u64(&mut buf, *revision);
                wire::write_bytes(&mut buf, state);
            }
            ShellToGateway::ShellStatePatch { revision, base_revision, patch } => {
                wire::write_u8(&mut buf, 2);
                wire::write_u64(&mut buf, *revision);
                wire::write_u64(&mut buf, *base_revision);
                wire::write_bytes(&mut buf, patch);
            }
            ShellToGateway::Instances { entries } => {
                wire::write_u8(&mut buf, 3);
                wire::write_u32(&mut buf, entries.len() as u32);
                for entry in entries {
                    entry.encode(&mut buf);
                }
            }
            ShellToGateway::AppFrames { in_reply_to, instance_id, frames } => {
                wire::write_u8(&mut buf, 4);
                wire::write_u64(&mut buf, *in_reply_to);
                wire::write_string(&mut buf, instance_id);
                wire::write_bytes_vec(&mut buf, frames);
            }
            ShellToGateway::ShellCommandResult { in_reply_to, ok, fault } => {
                wire::write_u8(&mut buf, 5);
                wire::write_u64(&mut buf, *in_reply_to);
                wire::write_bool(&mut buf, *ok);
                wire::write_option_string(&mut buf, fault);
            }
            ShellToGateway::Approval { approval_id, decision, note } => {
                wire::write_u8(&mut buf, 6);
                wire::write_string(&mut buf, approval_id);
                wire::write_u8(&mut buf, decision.to_tag());
                wire::write_option_string(&mut buf, note);
            }
            ShellToGateway::Ping => wire::write_u8(&mut buf, 7),
            ShellToGateway::Bye => wire::write_u8(&mut buf, 8),
        }
        buf
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, GatewayError> {
        let mut reader = wire::Reader::new(bytes);
        let tag = reader.read_u8()?;
        let frame = match tag {
            0 => ShellToGateway::Hello {
                bridge_version: reader.read_u16()?,
                shell_kind: ShellKind::from_tag(reader.read_u8()?)?,
                shell_session_id: reader.read_string()?,
                principal_actor: reader.read_string()?,
                flags: BridgeFlags::from_bits(reader.read_u8()?),
            },
            1 => ShellToGateway::ShellState { revision: reader.read_u64()?, state: reader.read_bytes()? },
            2 => ShellToGateway::ShellStatePatch { revision: reader.read_u64()?, base_revision: reader.read_u64()?, patch: reader.read_bytes()? },
            3 => {
                let len = reader.read_u32()? as usize;
                let mut entries = Vec::with_capacity(len);
                for _ in 0..len {
                    entries.push(BridgeInstanceRef::decode(&mut reader)?);
                }
                ShellToGateway::Instances { entries }
            }
            4 => ShellToGateway::AppFrames { in_reply_to: reader.read_u64()?, instance_id: reader.read_string()?, frames: reader.read_bytes_vec()? },
            5 => ShellToGateway::ShellCommandResult { in_reply_to: reader.read_u64()?, ok: reader.read_bool()?, fault: reader.read_option_string()? },
            6 => ShellToGateway::Approval { approval_id: reader.read_string()?, decision: ApprovalDecision::from_tag(reader.read_u8()?)?, note: reader.read_option_string()? },
            7 => ShellToGateway::Ping,
            8 => ShellToGateway::Bye,
            other => return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ShellToGateway tag {other}"))),
        };
        reader.finish()?;
        Ok(frame)
    }
}
//#endregion 🔖️ShellToGateway

//#region 🔖️GatewayToShell
/// 📤️ Gateway→Shell frames, tag 0..7 in this exact declaration order (`📋️master.md` §2.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GatewayToShell {
    Welcome { bridge_version: u16, connection: String, principal: String },
    ShellCommand { seq: u64, command: Vec<u8> },
    AppCommand { seq: u64, instance_id: String, command: Vec<u8> },
    ApprovalRequested { approval_id: String, summary: String },
    ApprovalResolved { approval_id: String, decision: ApprovalDecision },
    AgentPresence { active: bool, label: String, invocation_id: Option<String> },
    Pong,
    Bye { reason: String },
}

impl GatewayToShell {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            GatewayToShell::Welcome { bridge_version, connection, principal } => {
                wire::write_u8(&mut buf, 0);
                wire::write_u16(&mut buf, *bridge_version);
                wire::write_string(&mut buf, connection);
                wire::write_string(&mut buf, principal);
            }
            GatewayToShell::ShellCommand { seq, command } => {
                wire::write_u8(&mut buf, 1);
                wire::write_u64(&mut buf, *seq);
                wire::write_bytes(&mut buf, command);
            }
            GatewayToShell::AppCommand { seq, instance_id, command } => {
                wire::write_u8(&mut buf, 2);
                wire::write_u64(&mut buf, *seq);
                wire::write_string(&mut buf, instance_id);
                wire::write_bytes(&mut buf, command);
            }
            GatewayToShell::ApprovalRequested { approval_id, summary } => {
                wire::write_u8(&mut buf, 3);
                wire::write_string(&mut buf, approval_id);
                wire::write_string(&mut buf, summary);
            }
            GatewayToShell::ApprovalResolved { approval_id, decision } => {
                wire::write_u8(&mut buf, 4);
                wire::write_string(&mut buf, approval_id);
                wire::write_u8(&mut buf, decision.to_tag());
            }
            GatewayToShell::AgentPresence { active, label, invocation_id } => {
                wire::write_u8(&mut buf, 5);
                wire::write_bool(&mut buf, *active);
                wire::write_string(&mut buf, label);
                wire::write_option_string(&mut buf, invocation_id);
            }
            GatewayToShell::Pong => wire::write_u8(&mut buf, 6),
            GatewayToShell::Bye { reason } => {
                wire::write_u8(&mut buf, 7);
                wire::write_string(&mut buf, reason);
            }
        }
        buf
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, GatewayError> {
        let mut reader = wire::Reader::new(bytes);
        let tag = reader.read_u8()?;
        let frame = match tag {
            0 => GatewayToShell::Welcome { bridge_version: reader.read_u16()?, connection: reader.read_string()?, principal: reader.read_string()? },
            1 => GatewayToShell::ShellCommand { seq: reader.read_u64()?, command: reader.read_bytes()? },
            2 => GatewayToShell::AppCommand { seq: reader.read_u64()?, instance_id: reader.read_string()?, command: reader.read_bytes()? },
            3 => GatewayToShell::ApprovalRequested { approval_id: reader.read_string()?, summary: reader.read_string()? },
            4 => GatewayToShell::ApprovalResolved { approval_id: reader.read_string()?, decision: ApprovalDecision::from_tag(reader.read_u8()?)? },
            5 => GatewayToShell::AgentPresence { active: reader.read_bool()?, label: reader.read_string()?, invocation_id: reader.read_option_string()? },
            6 => GatewayToShell::Pong,
            7 => GatewayToShell::Bye { reason: reader.read_string()? },
            other => return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown GatewayToShell tag {other}"))),
        };
        reader.finish()?;
        Ok(frame)
    }
}
//#endregion 🔖️GatewayToShell

//#region 🔖️BridgeServer
/// 🌐️ WebSocket skeleton over axum's `ws` — echoes `Hello` → `Welcome` and nothing else yet.
/// Wiring this into the real gateway process (dispatch of every other frame kind, `ShellState`
/// broadcast, approval routing) is explicitly out of this packet's scope (`📌️sol-P1b-packet.md` §2.4:
/// "Do not wire the WebSocket server yet if that forces you into the shell's territory").
pub mod server {
    use super::{BridgeFlags, GatewayToShell, ShellToGateway, BRIDGE_VERSION};
    use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
    use axum::response::Response;
    use axum::routing::get;
    use axum::Router;

    pub fn bridge_router() -> Router {
        Router::new().route("/bridge", get(upgrade))
    }

    async fn upgrade(ws: WebSocketUpgrade) -> Response {
        ws.on_upgrade(handle_socket)
    }

    /// 🤝️ Reads exactly one frame; if it's a `Hello`, replies `Welcome`; anything else (or a second
    /// frame) is left to a later packet's real dispatch loop.
    async fn handle_socket(mut socket: WebSocket) {
        let Some(Ok(Message::Binary(bytes))) = socket.recv().await else { return };
        let Ok(ShellToGateway::Hello { shell_session_id, .. }) = ShellToGateway::decode(&bytes) else { return };
        let _ = BridgeFlags::NONE; // keeps the import meaningful if the Hello arm above is ever trimmed
        let welcome = GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: format!("conn_{shell_session_id}"), principal: "agent:local".to_string() };
        let _ = socket.send(Message::Binary(welcome.encode().into())).await;
    }
}
//#endregion 🔖️BridgeServer

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    fn sample_shell_frames() -> Vec<ShellToGateway> {
        vec![
            ShellToGateway::Hello { bridge_version: BRIDGE_VERSION, shell_kind: ShellKind::React, shell_session_id: "shell-1".into(), principal_actor: "agent:local".into(), flags: BridgeFlags { relay_app_commands: true, shared_backbone: false, elicit: true } },
            ShellToGateway::ShellState { revision: 7, state: vec![1, 2, 3, 4] },
            ShellToGateway::ShellStatePatch { revision: 8, base_revision: 7, patch: vec![9, 9] },
            ShellToGateway::Instances { entries: vec![BridgeInstanceRef { plugin_id: "cad".into(), app_id: "viewport".into(), instance_id: "inst-1".into(), artifact_ref: "cad-1".into(), window_ids: vec!["win-1".into(), "win-2".into()] }] },
            ShellToGateway::AppFrames { in_reply_to: 3, instance_id: "inst-1".into(), frames: vec![vec![1], vec![2, 3]] },
            ShellToGateway::ShellCommandResult { in_reply_to: 5, ok: true, fault: None },
            ShellToGateway::ShellCommandResult { in_reply_to: 6, ok: false, fault: Some("instance-busy".into()) },
            ShellToGateway::Approval { approval_id: "appr_1".into(), decision: ApprovalDecision::Once, note: Some("looks fine".into()) },
            ShellToGateway::Approval { approval_id: "appr_2".into(), decision: ApprovalDecision::Deny, note: None },
            ShellToGateway::Ping,
            ShellToGateway::Bye,
        ]
    }

    fn sample_gateway_frames() -> Vec<GatewayToShell> {
        vec![
            GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: "conn_1".into(), principal: "agent:local".into() },
            GatewayToShell::ShellCommand { seq: 1, command: vec![1, 2] },
            GatewayToShell::AppCommand { seq: 2, instance_id: "inst-1".into(), command: vec![3] },
            GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "translate selection by (1,0,0)".into() },
            GatewayToShell::ApprovalResolved { approval_id: "appr_1".into(), decision: ApprovalDecision::Session },
            GatewayToShell::AgentPresence { active: true, label: "claude-code".into(), invocation_id: Some("inv-1".into()) },
            GatewayToShell::AgentPresence { active: false, label: "".into(), invocation_id: None },
            GatewayToShell::Pong,
            GatewayToShell::Bye { reason: "shutdown".into() },
        ]
    }

    //#region 🔖️RoundTrip
    #[test]
    fn every_shell_to_gateway_variant_round_trips_through_encode_decode() {
        for frame in sample_shell_frames() {
            let bytes = frame.encode();
            let decoded = ShellToGateway::decode(&bytes).unwrap_or_else(|error| panic!("decode failed for {frame:?}: {error}"));
            assert_eq!(decoded, frame);
        }
    }

    #[test]
    fn every_gateway_to_shell_variant_round_trips_through_encode_decode() {
        for frame in sample_gateway_frames() {
            let bytes = frame.encode();
            let decoded = GatewayToShell::decode(&bytes).unwrap_or_else(|error| panic!("decode failed for {frame:?}: {error}"));
            assert_eq!(decoded, frame);
        }
    }

    #[test]
    fn decode_rejects_truncated_buffers() {
        // Tag 0 = Hello, which expects far more bytes than just the tag byte itself.
        let hello_tag_only = vec![0u8];
        assert!(ShellToGateway::decode(&hello_tag_only).is_err());
    }

    #[test]
    fn decode_rejects_trailing_bytes() {
        let mut bytes = ShellToGateway::Ping.encode();
        bytes.push(0xFF);
        let error = ShellToGateway::decode(&bytes).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::InputInvalid);
    }

    #[test]
    fn decode_rejects_an_unknown_tag() {
        let error = ShellToGateway::decode(&[99]).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::InputInvalid);
    }
    //#endregion 🔖️RoundTrip

    //#region 🔖️FixtureParity
    /// 🧬️ The anti-drift mechanism: `🧫️fixtures/frames.json` holds `{direction, variant, frame, hex}`
    /// rows. For each row this test (a) deserializes `frame` via serde into the real enum, (b) hand-
    /// encodes it and compares to `hex`, and (c) hex-decodes `hex` and compares the result back to the
    /// serde-deserialized frame — proving the fixture's `frame` JSON and `hex` bytes agree with THIS
    /// codec. The TS twin runs the mirror-image check against the SAME file.
    #[test]
    fn every_fixture_round_trips_through_the_rust_codec() {
        let raw = include_str!("🧫️fixtures/frames.json");
        let rows: Vec<serde_json::Value> = serde_json::from_str(raw).expect("fixtures/frames.json must parse");
        assert!(!rows.is_empty(), "fixture file must not be empty");
        let mut shell_to_gateway_count = 0;
        let mut gateway_to_shell_count = 0;
        for row in &rows {
            let direction = row["direction"].as_str().expect("row.direction");
            let hex = row["hex"].as_str().expect("row.hex");
            let frame_json = row["frame"].clone();
            let expected_bytes = decode_hex(hex);
            match direction {
                "shell_to_gateway" => {
                    shell_to_gateway_count += 1;
                    let frame: ShellToGateway = serde_json::from_value(frame_json).expect("frame must deserialize as ShellToGateway");
                    assert_eq!(encode_hex(&frame.encode()), hex, "encode mismatch for {frame:?}");
                    let decoded = ShellToGateway::decode(&expected_bytes).unwrap();
                    assert_eq!(decoded, frame, "decode mismatch for hex {hex}");
                }
                "gateway_to_shell" => {
                    gateway_to_shell_count += 1;
                    let frame: GatewayToShell = serde_json::from_value(frame_json).expect("frame must deserialize as GatewayToShell");
                    assert_eq!(encode_hex(&frame.encode()), hex, "encode mismatch for {frame:?}");
                    let decoded = GatewayToShell::decode(&expected_bytes).unwrap();
                    assert_eq!(decoded, frame, "decode mismatch for hex {hex}");
                }
                other => panic!("unknown fixture direction: {other}"),
            }
        }
        assert_eq!(shell_to_gateway_count, 11, "fixtures must cover every ShellToGateway variant instance");
        assert_eq!(gateway_to_shell_count, 9, "fixtures must cover every GatewayToShell variant instance");
    }

    fn encode_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn decode_hex(hex: &str) -> Vec<u8> {
        (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("valid hex")).collect()
    }
    //#endregion 🔖️FixtureParity
}

/// 🌐️ Exercises the real axum `ws` upgrade end-to-end over a bound loopback socket — foreground,
/// finishes within one `#[tokio::test]`, no background process left running.
#[cfg(test)]
mod long {
    use super::server::bridge_router;
    use super::*;
    use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

    #[tokio::test]
    async fn bridge_websocket_replies_welcome_to_hello() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server_task = tokio::spawn(async move {
            axum::serve(listener, bridge_router()).await.unwrap();
        });

        let (mut socket, _response) = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge")).await.expect("client connects");
        let hello = ShellToGateway::Hello { bridge_version: BRIDGE_VERSION, shell_kind: ShellKind::WgpuNative, shell_session_id: "shell-42".into(), principal_actor: "agent:local".into(), flags: BridgeFlags::NONE };
        use futures::{SinkExt, StreamExt};
        socket.send(TungsteniteMessage::Binary(hello.encode().into())).await.unwrap();

        let response = socket.next().await.expect("a response frame").expect("a valid ws message");
        let TungsteniteMessage::Binary(bytes) = response else { panic!("expected a binary frame") };
        let decoded = GatewayToShell::decode(&bytes).unwrap();
        assert_eq!(decoded, GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: "conn_shell-42".into(), principal: "agent:local".into() });

        server_task.abort();
    }
}
//#endregion 🧪️Tests
