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
//!
//! P1c (`📓️sol-P1c-packet.md`) makes the bridge LIVE: `🔖️BridgeHandle`/`🔖️BridgeToken` below plus the
//! real `mod server` (`🔖️BridgeServer`) that mounts `/bridge` alongside `/mcp` on the same axum app
//! `🚚️transport/🦀️component.rs`'s `HttpTransport::router` builds.

use crate::errors::{GatewayError, GatewayErrorCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

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

//#region 🔖️BridgeHandle
/// 🆔️ One live `/bridge` connection's id — `Copy`/`Eq`/`Hash` so it keys a map and passes around
/// freely; `Display` is the same string the connection's own `Welcome.connection` field carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShellConnectionId(u64);

impl std::fmt::Display for ShellConnectionId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "conn_{}", self.0)
    }
}

struct ConnectionEntry {
    outbox: tokio::sync::mpsc::UnboundedSender<GatewayToShell>,
    last_shell_state: Option<ShellToGateway>,
    last_instances: Option<Vec<BridgeInstanceRef>>,
    last_command_result: Option<(u64, bool, Option<String>)>,
    last_approval: Option<(String, ApprovalDecision, Option<String>)>,
}

#[derive(Default)]
struct BridgeInner {
    next_id: AtomicU64,
    connections: Mutex<HashMap<ShellConnectionId, ConnectionEntry>>,
}

/// 🖇️ The seam a later packet reaches live `/bridge` connections through WITHOUT this facet
/// depending on theirs — P6's policy engine routes a parked approval to a connected shell via
/// [`send_to`](Self::send_to)/[`broadcast`](Self::broadcast); a future `ui.*` tool pushes a
/// `ShellCommand` the same way; `semio://ui/shell`/`context_resolve` read
/// [`last_shell_state`](Self::last_shell_state). None of that wiring happens in THIS file
/// (`📓️sol-P1c-packet.md` §3: "do not wire it into P6's files … just publish the API") — obtain a
/// `BridgeHandle` from [`server::bridge_router`] or [`crate::HttpTransport::router`]'s returned tuple.
#[derive(Clone, Default)]
pub struct BridgeHandle {
    inner: Arc<BridgeInner>,
}

impl BridgeHandle {
    pub fn new() -> Self {
        Self::default()
    }

    fn register(&self) -> (ShellConnectionId, tokio::sync::mpsc::UnboundedReceiver<GatewayToShell>) {
        let id = ShellConnectionId(self.inner.next_id.fetch_add(1, Ordering::Relaxed));
        let (outbox, inbox) = tokio::sync::mpsc::unbounded_channel();
        self.inner.connections.lock().expect("bridge connections lock poisoned").insert(id, ConnectionEntry { outbox, last_shell_state: None, last_instances: None, last_command_result: None, last_approval: None });
        (id, inbox)
    }

    fn unregister(&self, id: ShellConnectionId) {
        self.inner.connections.lock().expect("bridge connections lock poisoned").remove(&id);
    }

    /// 📝️ Records the effect of one received [`ShellToGateway`] frame against its connection —
    /// `Hello`/`Ping`/`Bye` never reach here (the read loop handles all three inline).
    fn record(&self, id: ShellConnectionId, frame: ShellToGateway) {
        let mut connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        let Some(entry) = connections.get_mut(&id) else { return };
        match frame {
            ShellToGateway::ShellState { .. } | ShellToGateway::ShellStatePatch { .. } => entry.last_shell_state = Some(frame),
            ShellToGateway::Instances { entries } => entry.last_instances = Some(entries),
            ShellToGateway::ShellCommandResult { in_reply_to, ok, fault } => entry.last_command_result = Some((in_reply_to, ok, fault)),
            ShellToGateway::Approval { approval_id, decision, note } => entry.last_approval = Some((approval_id, decision, note)),
            ShellToGateway::Hello { .. } | ShellToGateway::Ping | ShellToGateway::Bye | ShellToGateway::AppFrames { .. } => {}
        }
    }

    /// 📤️ Pushes one frame to exactly one live connection — `false` if that connection no longer
    /// exists or its outbox is closed (the connection's own task will unregister it shortly after).
    pub fn send_to(&self, id: ShellConnectionId, frame: GatewayToShell) -> bool {
        let connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        match connections.get(&id) {
            Some(entry) => entry.outbox.send(frame).is_ok(),
            None => false,
        }
    }

    /// 📢️ Pushes one frame to EVERY live connection — returns how many it actually reached.
    pub fn broadcast(&self, frame: GatewayToShell) -> usize {
        let connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        connections.values().filter(|entry| entry.outbox.send(frame.clone()).is_ok()).count()
    }

    pub fn connections(&self) -> Vec<ShellConnectionId> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").keys().copied().collect()
    }

    /// 🧭️ The last `ShellState`/`ShellStatePatch` frame received on this connection. Applying a patch
    /// onto a base state is `💻️os/🔨️modules/🖥️shell`'s reducer's job, not this facet's — a caller that
    /// needs the merged/canonical state feeds whatever this returns through that reducer.
    pub fn last_shell_state(&self, id: ShellConnectionId) -> Option<ShellToGateway> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_shell_state.clone())
    }

    pub fn last_instances(&self, id: ShellConnectionId) -> Option<Vec<BridgeInstanceRef>> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_instances.clone())
    }

    pub fn last_command_result(&self, id: ShellConnectionId) -> Option<(u64, bool, Option<String>)> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_command_result.clone())
    }

    pub fn last_approval(&self, id: ShellConnectionId) -> Option<(String, ApprovalDecision, Option<String>)> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_approval.clone())
    }
}
//#endregion 🔖️BridgeHandle

//#region 🔖️BridgeToken
/// 🎲️ The `/bridge` connection secret, freshly minted per process start — a DIFFERENT secret from the
/// `/mcp` bearer token (`📋️master.md` §2.1: "token minted at start, 0600 file"). Same
/// dependency-free blake3-mixed scheme as `🎫️handles::mint_id` (no `rand`/`uuid` added for this).
pub fn mint_bridge_token() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    let now_ns = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_nanos()).unwrap_or(0);
    let entropy_marker = Box::new(counter);
    let entropy = format!("{now_ns}:{counter}:{:p}:{}", entropy_marker.as_ref(), std::process::id());
    framework_hash::hash_bytes(entropy.as_bytes())
}

fn home_dir() -> Option<PathBuf> {
    for variable in ["HOME", "USERPROFILE"] {
        if let Ok(value) = std::env::var(variable) {
            if !value.is_empty() {
                return Some(PathBuf::from(value));
            }
        }
    }
    None
}

/// 🏠️ `~/.semio/agent/bridge-token` — overridable by `semio-os-mcp http`'s `--bridge-token-file`.
pub fn default_bridge_token_path() -> PathBuf {
    home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".semio").join("agent").join("bridge-token")
}

/// 🔐️ Creates the parent directory if missing, writes `token` verbatim, and (unix only) chmods the
/// file `0600` — best-effort on non-unix targets (no POSIX mode bits there; the file still inherits
/// the parent directory's normal ACLs), documented rather than silently claimed as `0600` everywhere.
pub fn write_bridge_token_file(path: &Path, token: &str) -> Result<(), GatewayError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot create bridge token directory `{}`: {error}", parent.display())))?;
    }
    std::fs::write(path, token).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot write bridge token file `{}`: {error}", path.display())))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot chmod bridge token file `{}`: {error}", path.display())))?;
    }
    Ok(())
}
//#endregion 🔖️BridgeToken

//#region 🔖️BridgeServer
/// 🌐️ The real `/bridge` websocket endpoint (P1c) — mounted onto the SAME axum app `/mcp` lives on
/// (`🚚️transport/🦀️component.rs`'s `HttpTransport::router` calls [`bridge_router`] and `.merge()`s the
/// result). Auth: `Origin` must be loopback/`null`/allowlisted (`403` otherwise — the identical policy
/// `/mcp` uses, via `crate::transport::origin_allowed`), and `?token=` must match the minted bridge
/// token (`401` otherwise, constant-time compared via `crate::transport::constant_time_eq`) — BOTH
/// checked before the websocket upgrade completes, so a rejected client gets a plain HTTP error
/// status, never a silently-closed socket.
pub mod server {
    use super::{BridgeHandle, GatewayToShell, ShellToGateway, BRIDGE_VERSION};
    use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
    use axum::extract::{Query, State};
    use axum::http::{HeaderMap, StatusCode};
    use axum::response::{IntoResponse, Response};
    use axum::routing::get;
    use axum::Router;
    use futures::{SinkExt, StreamExt};
    use serde::Deserialize;
    use std::sync::Arc;

    #[derive(Deserialize)]
    struct BridgeQuery {
        token: Option<String>,
    }

    #[derive(Clone)]
    struct BridgeServerState {
        token: Arc<str>,
        allowed_origins: Arc<Vec<String>>,
        handle: BridgeHandle,
    }

    /// 🏗️ Builds the `/bridge` route + its [`BridgeHandle`] — never bound to a socket by itself; a
    /// caller `.merge()`s the returned [`Router`] into the app it actually serves.
    pub fn bridge_router(token: impl Into<String>, allowed_origins: Vec<String>) -> (Router, BridgeHandle) {
        let handle = BridgeHandle::new();
        let state = BridgeServerState { token: Arc::from(token.into().as_str()), allowed_origins: Arc::new(allowed_origins), handle: handle.clone() };
        let router = Router::new().route("/bridge", get(upgrade)).with_state(state);
        (router, handle)
    }

    async fn upgrade(ws: WebSocketUpgrade, Query(query): Query<BridgeQuery>, headers: HeaderMap, State(state): State<BridgeServerState>) -> Response {
        let origin = headers.get(axum::http::header::ORIGIN).and_then(|value| value.to_str().ok());
        if !crate::transport::origin_allowed(origin, &state.allowed_origins) {
            return (StatusCode::FORBIDDEN, "origin not allowed").into_response();
        }
        let provided = query.token.unwrap_or_default();
        if !crate::transport::constant_time_eq(provided.as_bytes(), state.token.as_bytes()) {
            return (StatusCode::UNAUTHORIZED, "invalid bridge token").into_response();
        }
        let handle = state.handle.clone();
        ws.on_upgrade(move |socket| handle_socket(socket, handle))
    }

    /// 🔁️ One connection's full lifecycle: the OPENING frame must decode as `Hello` (anything else,
    /// or a closed/errored socket, ends the connection immediately without ever registering it) →
    /// reply `Welcome` → loop reading client frames (`Ping`→`Pong` inline, `Bye`/close ends the loop,
    /// `ShellState`/`ShellStatePatch`/`Instances`/`ShellCommandResult`/`Approval` recorded via
    /// [`BridgeHandle::record`]; a frame that fails to decode is skipped rather than killing the
    /// connection) while concurrently draining this connection's OWN outbox
    /// ([`BridgeHandle::send_to`]/[`BridgeHandle::broadcast`] push onto it) to the socket.
    async fn handle_socket(socket: WebSocket, handle: BridgeHandle) {
        let (mut sender, mut receiver) = socket.split();
        let Some(Ok(Message::Binary(bytes))) = receiver.next().await else { return };
        let Ok(ShellToGateway::Hello { .. }) = ShellToGateway::decode(&bytes) else { return };

        let (id, mut outbox) = handle.register();
        let welcome = GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: id.to_string(), principal: "agent:local".to_string() };
        if sender.send(Message::Binary(welcome.encode().into())).await.is_err() {
            handle.unregister(id);
            return;
        }

        loop {
            tokio::select! {
                incoming = receiver.next() => {
                    match incoming {
                        Some(Ok(Message::Binary(bytes))) => match ShellToGateway::decode(&bytes) {
                            Ok(ShellToGateway::Ping) => {
                                if sender.send(Message::Binary(GatewayToShell::Pong.encode().into())).await.is_err() {
                                    break;
                                }
                            }
                            Ok(ShellToGateway::Bye) => break,
                            Ok(frame) => handle.record(id, frame),
                            Err(_malformed) => {}
                        },
                        Some(Ok(Message::Close(_))) | None => break,
                        Some(Ok(_)) => {}
                        Some(Err(_)) => break,
                    }
                }
                pushed = outbox.recv() => {
                    match pushed {
                        Some(frame) => {
                            if sender.send(Message::Binary(frame.encode().into())).await.is_err() {
                                break;
                            }
                        }
                        None => break,
                    }
                }
            }
        }
        handle.unregister(id);
    }
}
//#endregion 🔖️BridgeServer

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    fn sample_shell_frames() -> Vec<ShellToGateway> {
        vec![
            ShellToGateway::Hello {
                bridge_version: BRIDGE_VERSION,
                shell_kind: ShellKind::React,
                shell_session_id: "shell-1".into(),
                principal_actor: "agent:local".into(),
                flags: BridgeFlags { relay_app_commands: true, shared_backbone: false, elicit: true },
            },
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
    use futures::{SinkExt, StreamExt};
    use tokio_tungstenite::tungstenite::client::IntoClientRequest;
    use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

    async fn boot(token: &str, allowed_origins: Vec<String>) -> (std::net::SocketAddr, BridgeHandle, tokio::task::JoinHandle<()>) {
        let (router, handle) = bridge_router(token.to_string(), allowed_origins);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server_task = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        (addr, handle, server_task)
    }

    fn hello(session_id: &str) -> ShellToGateway {
        ShellToGateway::Hello { bridge_version: BRIDGE_VERSION, shell_kind: ShellKind::WgpuNative, shell_session_id: session_id.into(), principal_actor: "agent:local".into(), flags: BridgeFlags::NONE }
    }

    async fn recv_frame(socket: &mut (impl StreamExt<Item = Result<TungsteniteMessage, tokio_tungstenite::tungstenite::Error>> + Unpin)) -> GatewayToShell {
        let message = socket.next().await.expect("a response frame").expect("a valid ws message");
        let TungsteniteMessage::Binary(bytes) = message else { panic!("expected a binary frame, got {message:?}") };
        GatewayToShell::decode(&bytes).unwrap()
    }

    #[tokio::test]
    async fn bridge_websocket_replies_welcome_to_hello() {
        let (addr, _handle, server_task) = boot("secret", vec![]).await;
        let (mut socket, _response) = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge?token=secret")).await.expect("client connects");
        socket.send(TungsteniteMessage::Binary(hello("shell-42").encode().into())).await.unwrap();
        let welcome = recv_frame(&mut socket).await;
        assert!(matches!(welcome, GatewayToShell::Welcome { bridge_version, ref principal, .. } if bridge_version == BRIDGE_VERSION && principal == "agent:local"));
        server_task.abort();
    }

    #[tokio::test]
    async fn wrong_token_is_rejected_before_the_websocket_upgrade() {
        let (addr, _handle, server_task) = boot("correct-token", vec![]).await;
        let result = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge?token=wrong-token")).await;
        assert!(result.is_err(), "a mismatched token must never complete the websocket handshake");
        server_task.abort();
    }

    #[tokio::test]
    async fn missing_token_is_rejected() {
        let (addr, _handle, server_task) = boot("correct-token", vec![]).await;
        let result = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge")).await;
        assert!(result.is_err());
        server_task.abort();
    }

    #[tokio::test]
    async fn an_evil_origin_is_rejected_before_the_websocket_upgrade() {
        let (addr, _handle, server_task) = boot("secret", vec![]).await;
        let mut request = format!("ws://{addr}/bridge?token=secret").into_client_request().unwrap();
        request.headers_mut().insert("origin", "https://evil.example".parse().unwrap());
        let result = tokio_tungstenite::connect_async(request).await;
        assert!(result.is_err(), "a non-loopback Origin must never complete the websocket handshake");
        server_task.abort();
    }

    /// 🔁️ The full scenario `📓️sol-P1c-packet.md`'s acceptance list names in one place: `Hello`→
    /// `Welcome`, a `ShellState` publish becomes readable via [`BridgeHandle::last_shell_state`], a
    /// server-pushed `ShellCommand` reaches the client, and the client's `ShellCommandResult` becomes
    /// readable via [`BridgeHandle::last_command_result`].
    #[tokio::test]
    async fn full_bridge_lifecycle_hello_state_push_and_command_result() {
        let (addr, handle, server_task) = boot("secret", vec![]).await;
        let (mut socket, _response) = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge?token=secret")).await.expect("client connects");
        socket.send(TungsteniteMessage::Binary(hello("shell-1").encode().into())).await.unwrap();
        let _welcome = recv_frame(&mut socket).await;

        let id = handle.connections().first().copied().expect("exactly one connection registered");
        assert!(handle.last_shell_state(id).is_none());

        let state_frame = ShellToGateway::ShellState { revision: 5, state: vec![9, 9, 9] };
        socket.send(TungsteniteMessage::Binary(state_frame.clone().encode().into())).await.unwrap();
        // Ping/Pong round trip as a synchronization barrier — the connection task processes frames
        // strictly in arrival order, so by the time Pong comes back the ShellState above is recorded.
        socket.send(TungsteniteMessage::Binary(ShellToGateway::Ping.encode().into())).await.unwrap();
        assert_eq!(recv_frame(&mut socket).await, GatewayToShell::Pong);
        assert_eq!(handle.last_shell_state(id), Some(state_frame));

        let pushed = GatewayToShell::ShellCommand { seq: 7, command: vec![1, 2, 3] };
        assert!(handle.send_to(id, pushed.clone()), "send_to must reach the live connection");
        assert_eq!(recv_frame(&mut socket).await, pushed);

        let result_frame = ShellToGateway::ShellCommandResult { in_reply_to: 7, ok: true, fault: None };
        socket.send(TungsteniteMessage::Binary(result_frame.encode().into())).await.unwrap();
        socket.send(TungsteniteMessage::Binary(ShellToGateway::Ping.encode().into())).await.unwrap();
        assert_eq!(recv_frame(&mut socket).await, GatewayToShell::Pong);
        assert_eq!(handle.last_command_result(id), Some((7, true, None)));

        assert_eq!(handle.broadcast(GatewayToShell::Pong), 1, "broadcast must reach exactly the one live connection");
        assert_eq!(recv_frame(&mut socket).await, GatewayToShell::Pong);

        drop(socket);
        server_task.abort();
    }

    #[tokio::test]
    async fn send_to_an_unknown_connection_returns_false() {
        let handle = BridgeHandle::new();
        assert!(!handle.send_to(ShellConnectionId(999), GatewayToShell::Pong));
    }

    #[test]
    fn mint_bridge_token_produces_distinct_high_entropy_tokens() {
        let a = mint_bridge_token();
        let b = mint_bridge_token();
        assert_ne!(a, b);
        assert_eq!(a.len(), 64, "blake3 hex digest is 64 chars");
    }

    #[test]
    fn write_bridge_token_file_creates_parents_and_is_readable_back() {
        let dir = std::env::temp_dir().join(format!("semio-mcp-bridge-token-test-{}-{}", std::process::id(), framework_hash::hash_bytes(b"bridge-token-test")));
        let path = dir.join("bridge-token");
        write_bridge_token_file(&path, "the-token").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "the-token");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o600);
        }
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn default_bridge_token_path_ends_with_the_frozen_suffix() {
        assert!(default_bridge_token_path().ends_with(Path::new(".semio").join("agent").join("bridge-token")));
    }
}
//#endregion 🧪️Tests
