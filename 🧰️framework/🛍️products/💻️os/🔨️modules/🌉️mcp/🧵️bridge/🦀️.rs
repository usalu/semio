//! 🧵️ `ShellBridge` frame codec (`📋️master.md` §2.2 verbatim variant list, `📌️sol-P1b-packet.md`
//! §2.4) — hand-rolled binary framing (`tag: u8` + fields in declaration order), `BRIDGE_VERSION = 1`.
//! This file is the Rust SSOT; `🟦️.ts` is the TS twin, and `🧫️fixtures/📨️frames.json` is the
//! anti-drift mechanism proving both codecs agree byte-for-byte (`mod quick`'s
//! `every_fixture_round_trips_through_the_rust_codec` test; the TS side of the same fixtures is
//! exercised by a foreground `bun run` script, not wired into this crate's own test run).
//!
//! Field types are hand-picked for a self-contained wire format: this facet deliberately does NOT
//! depend on the peer `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket's `os_spr::wire` primitives
//! (`📡️spr/🧵️channel/🦀️.rs` is their exclusive, mid-rewrite territory per
//! `📌️important.md`'s collision matrix) — see this packet's report §7 for the full rationale. Opaque
//! payloads (`ShellState.state`, `ShellStatePatch.patch`, `ShellCommand.command`,
//! `AppCommand.command`, each `AppFrames.frames[i]`) are carried as length-prefixed byte blobs; what
//! goes inside them (the real packed `ShellState`) is `💻️os/🔨️modules/🖥️shell`'s concern (P9), not
//! this codec's.
//!
//! The live bridge is owned by the retained HTTP transport through `🔖️BridgeHandle`; the Axum
//! `🔖️BridgeServer` remains a `cfg(test)` differential oracle only.

use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::Deserialize;
use serde::Serialize;
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};

//#region 🔖️BridgeVersion

pub const BRIDGE_VERSION: u16 = 1;

//#endregion 🔖️BridgeVersion

//#region 🔖️Wire

/// 🧵️ Minimal length-prefixed little-endian primitives shared by both frame enums' hand-rolled
/// `encode`/`decode` — private to this file; the TS twin (`🟦️.ts`) mirrors the exact same
/// byte layout independently (that agreement is what the fixtures prove).
mod wire {
    use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};

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

        fn need(&self, count: usize) -> Result<(), semio_framework_os_mcp_foundation::errors::GatewayError> {
            if self.pos + count > self.data.len() {
                Err(semio_framework_os_mcp_foundation::errors::GatewayError::new(semio_framework_os_mcp_foundation::errors::GatewayErrorCode::InputInvalid, "bridge frame: unexpected end of buffer"))
            } else {
                Ok(())
            }
        }

        pub fn read_u8(&mut self) -> Result<u8, semio_framework_os_mcp_foundation::errors::GatewayError> {
            self.need(1)?;
            let value = self.data[self.pos];
            self.pos += 1;
            Ok(value)
        }
        pub fn read_u16(&mut self) -> Result<u16, semio_framework_os_mcp_foundation::errors::GatewayError> {
            self.need(2)?;
            let value = u16::from_le_bytes(self.data[self.pos..self.pos + 2].try_into().expect("checked length"));
            self.pos += 2;
            Ok(value)
        }
        pub fn read_u32(&mut self) -> Result<u32, semio_framework_os_mcp_foundation::errors::GatewayError> {
            self.need(4)?;
            let value = u32::from_le_bytes(self.data[self.pos..self.pos + 4].try_into().expect("checked length"));
            self.pos += 4;
            Ok(value)
        }
        pub fn read_u64(&mut self) -> Result<u64, semio_framework_os_mcp_foundation::errors::GatewayError> {
            self.need(8)?;
            let value = u64::from_le_bytes(self.data[self.pos..self.pos + 8].try_into().expect("checked length"));
            self.pos += 8;
            Ok(value)
        }
        pub fn read_bool(&mut self) -> Result<bool, semio_framework_os_mcp_foundation::errors::GatewayError> {
            Ok(self.read_u8()? != 0)
        }
        pub fn read_bytes(&mut self) -> Result<Vec<u8>, semio_framework_os_mcp_foundation::errors::GatewayError> {
            let len = self.read_u32()? as usize;
            self.need(len)?;
            let value = self.data[self.pos..self.pos + len].to_vec();
            self.pos += len;
            Ok(value)
        }
        pub fn read_string(&mut self) -> Result<String, semio_framework_os_mcp_foundation::errors::GatewayError> {
            let bytes = self.read_bytes()?;
            String::from_utf8(bytes).map_err(|error| semio_framework_os_mcp_foundation::errors::GatewayError::new(semio_framework_os_mcp_foundation::errors::GatewayErrorCode::InputInvalid, format!("bridge frame: invalid utf8: {error}")))
        }
        pub fn read_option_string(&mut self) -> Result<Option<String>, semio_framework_os_mcp_foundation::errors::GatewayError> {
            if self.read_bool()? {
                Ok(Some(self.read_string()?))
            } else {
                Ok(None)
            }
        }
        pub fn read_string_vec(&mut self) -> Result<Vec<String>, semio_framework_os_mcp_foundation::errors::GatewayError> {
            let len = self.read_u32()? as usize;
            (0..len).map(|_| self.read_string()).collect()
        }
        pub fn read_bytes_vec(&mut self) -> Result<Vec<Vec<u8>>, semio_framework_os_mcp_foundation::errors::GatewayError> {
            let len = self.read_u32()? as usize;
            (0..len).map(|_| self.read_bytes()).collect()
        }
        pub fn finish(self) -> Result<(), semio_framework_os_mcp_foundation::errors::GatewayError> {
            if self.pos == self.data.len() {
                Ok(())
            } else {
                Err(semio_framework_os_mcp_foundation::errors::GatewayError::new(semio_framework_os_mcp_foundation::errors::GatewayErrorCode::InputInvalid, format!("bridge frame: {} trailing byte(s) after decode", self.data.len() - self.pos)))
            }
        }
    }
}

//#endregion 🔖️Wire

//#region 🔖️SharedTypes

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum ShellKind {
    React,
    WgpuWeb,
    WgpuNative,
}

impl ShellKind {
    pub fn to_tag(self) -> u8 {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum ApprovalDecision {
    Deny,
    Once,
    Session,
}

impl ApprovalDecision {
    pub fn to_tag(self) -> u8 {
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

/// 🪦️ Why the gateway took an approval request back before any human decided it. The shell retires
/// the affordance and says why; the gateway no longer waits for a decision on it.
/// - `Cancelled`: the agent's call that parked the request was cancelled.
/// - `TimedOut`: nobody decided within the request's own countdown.
/// - `Superseded`: the request moved to a newer shell connection, which now carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum ApprovalWithdrawal {
    Cancelled,
    TimedOut,
    Superseded,
}

impl ApprovalWithdrawal {
    pub fn to_tag(self) -> u8 {
        match self {
            ApprovalWithdrawal::Cancelled => 0,
            ApprovalWithdrawal::TimedOut => 1,
            ApprovalWithdrawal::Superseded => 2,
        }
    }
    fn from_tag(tag: u8) -> Result<Self, GatewayError> {
        match tag {
            0 => Ok(ApprovalWithdrawal::Cancelled),
            1 => Ok(ApprovalWithdrawal::TimedOut),
            2 => Ok(ApprovalWithdrawal::Superseded),
            other => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ApprovalWithdrawal tag {other}"))),
        }
    }
}

/// 🚫️ Why a gateway refused a shell's connection instead of welcoming it. A shell treats a refusal as
/// a state of that offer — one localized notice, no redial of the same offer — never as a fault.
/// - `Version`: the shell's `Hello.bridge_version` is not the gateway's [`BRIDGE_VERSION`].
/// - `Capacity`: the gateway already serves as many connections as it admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum BridgeRefusal {
    Version,
    Capacity,
}

impl BridgeRefusal {
    pub fn to_tag(self) -> u8 {
        match self {
            BridgeRefusal::Version => 0,
            BridgeRefusal::Capacity => 1,
        }
    }
    fn from_tag(tag: u8) -> Result<Self, GatewayError> {
        match tag {
            0 => Ok(BridgeRefusal::Version),
            1 => Ok(BridgeRefusal::Capacity),
            other => Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown BridgeRefusal tag {other}"))),
        }
    }
}

/// 📇️ One entry of `Instances{entries}` — `BridgeInstanceRef{plugin_id, app_id, instance_id,
/// artifact_ref, window_ids}` verbatim from `📋️master.md` §2.2.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct BridgeInstanceRef {
    pub plugin_id: String,
    pub app_id: String,
    pub instance_id: String,
    pub artifact_ref: String,
    pub window_ids: Vec<String>,
}

impl BridgeInstanceRef {
    pub fn encode(&self, buf: &mut Vec<u8>) {
        wire::write_string(buf, &self.plugin_id);
        wire::write_string(buf, &self.app_id);
        wire::write_string(buf, &self.instance_id);
        wire::write_string(buf, &self.artifact_ref);
        wire::write_string_vec(buf, &self.window_ids);
    }
    pub fn decode(reader: &mut wire::Reader<'_>) -> Result<Self, GatewayError> {
        Ok(Self { plugin_id: reader.read_string()?, app_id: reader.read_string()?, instance_id: reader.read_string()?, artifact_ref: reader.read_string()?, window_ids: reader.read_string_vec()? })
    }
}

//#endregion 🔖️SharedTypes

//#region 🔖️ShellToGateway

/// 📨️ Shell→Gateway frames, tag 0..10 in this exact declaration order (`📋️master.md` §2.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
    /// 💬️ One human turn typed into the shell's own agent chat panel, addressed to the connected
    /// MCP agent. `message_id` is minted by the shell and is what the gateway's
    /// `semio://ui/agent-messages` resource pages from, so the agent reads each turn exactly once
    /// and never has to guess whether it already saw one.
    AgentMessage { message_id: String, text: String },
    /// 🛑️ The human pressed cancel on a tool call still shown as running in the shell's agent panel.
    /// `invocation_id` is the exact id `AgentConversation::begin_tool_call` published, which is also
    /// the id that call's `crate::ui::job_registry()` entry is keyed by — so this frame reaches the
    /// SAME cooperative cancellation the `job_cancel` MCP tool flips, never a second mechanism.
    AgentCancel { invocation_id: String },
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
            ShellToGateway::AgentMessage { message_id, text } => {
                wire::write_u8(&mut buf, 9);
                wire::write_string(&mut buf, message_id);
                wire::write_string(&mut buf, text);
            }
            ShellToGateway::AgentCancel { invocation_id } => {
                wire::write_u8(&mut buf, 10);
                wire::write_string(&mut buf, invocation_id);
            }
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
            9 => ShellToGateway::AgentMessage { message_id: reader.read_string()?, text: reader.read_string()? },
            10 => ShellToGateway::AgentCancel { invocation_id: reader.read_string()? },
            other => return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown ShellToGateway tag {other}"))),
        };
        reader.finish()?;
        Ok(frame)
    }
}

//#endregion 🔖️ShellToGateway

//#region 🔖️BoundedShellDecode

pub const BRIDGE_INBOUND_PAGE_BYTES: usize = 16_384;

const BRIDGE_INBOUND_MAX_BYTES: usize = 1_048_576;

const BRIDGE_INBOUND_MAX_PAGES: usize = BRIDGE_INBOUND_MAX_BYTES.div_ceil(BRIDGE_INBOUND_PAGE_BYTES);

const BRIDGE_INBOUND_MAX_ITEMS: usize = 256;

const BRIDGE_INBOUND_MAX_RANGES: usize = 1_280;

pub const BRIDGE_INBOUND_MAX_FIELD_BYTES: usize = BRIDGE_INBOUND_MAX_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellFrameKind {
    Hello,
    ShellState,
    ShellStatePatch,
    Instances,
    AppFrames,
    ShellCommandResult,
    Approval,
    Ping,
    Bye,
    AgentMessage,
    AgentCancel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellDecodeFault {
    Malformed,
    Capacity,
}

pub enum ShellDecodeStep {
    Pending,
    Complete(ValidatedShellFrame),
    Fault(ShellDecodeFault),
}

#[derive(Clone, Copy)]
enum ShellRangeKind {
    Bytes,
    String,
}

#[derive(Clone, Copy)]
struct ShellRange {
    pub start: usize,
    pub len: usize,
    pub kind: ShellRangeKind,
}

#[derive(Clone, Copy)]
enum ShellDecodePhase {
    Tag,
    HelloVersion,
    HelloKind,
    HelloSession,
    HelloPrincipal,
    HelloFlags,
    StateRevision,
    StateBytes,
    PatchRevision,
    PatchBaseRevision,
    PatchBytes,
    InstancesCount,
    InstancePlugin { instances: usize },
    InstanceApp { instances: usize },
    InstanceId { instances: usize },
    InstanceArtifact { instances: usize },
    InstanceWindowsCount { instances: usize },
    InstanceWindow { instances: usize, windows: usize },
    AppReply,
    AppInstance,
    AppFramesCount,
    AppFrame { frames: usize },
    CommandReply,
    CommandOk,
    CommandFaultFlag,
    CommandFaultString,
    ApprovalId,
    ApprovalDecision,
    ApprovalNoteFlag,
    ApprovalNoteString,
    AgentMessageId,
    AgentMessageText,
    AgentCancelInvocation,
    Finish,
    ValidateStrings { range: usize, offset: usize, utf8: Utf8Cursor },
    Copy { offset: usize },
}

#[derive(Clone, Copy)]
struct Utf8Cursor {
    needed: u8,
    pub min: u8,
    pub max: u8,
}

impl Utf8Cursor {
    fn new() -> Self {
        Self { needed: 0, min: 0x80, max: 0xbf }
    }

    pub fn push(&mut self, byte: u8) -> bool {
        if self.needed != 0 {
            if byte < self.min || byte > self.max {
                return false;
            }
            self.needed -= 1;
            self.min = 0x80;
            self.max = 0xbf;
            return true;
        }
        match byte {
            0x00..=0x7f => true,
            0xc2..=0xdf => {
                self.needed = 1;
                true
            }
            0xe0 => {
                self.needed = 2;
                self.min = 0xa0;
                true
            }
            0xe1..=0xec | 0xee..=0xef => {
                self.needed = 2;
                true
            }
            0xed => {
                self.needed = 2;
                self.max = 0x9f;
                true
            }
            0xf0 => {
                self.needed = 3;
                self.min = 0x90;
                true
            }
            0xf1..=0xf3 => {
                self.needed = 3;
                true
            }
            0xf4 => {
                self.needed = 3;
                self.max = 0x8f;
                true
            }
            _ => false,
        }
    }

    pub fn complete(self) -> bool {
        self.needed == 0
    }
}

pub struct ShellToGatewayDecodeCursor {
    pub phase: ShellDecodePhase,
    pub payload_len: usize,
    pub cursor: usize,
    pub kind: Option<ShellFrameKind>,
    ranges: [Option<ShellRange>; BRIDGE_INBOUND_MAX_RANGES],
    range_count: usize,
    pub items: usize,
    owned_bytes: usize,
    pending_range: Option<usize>,
    pub output: Option<ValidatedShellFrame>,
}

impl ShellToGatewayDecodeCursor {
    pub fn new(payload_len: usize) -> Self {
        Self { phase: ShellDecodePhase::Tag, payload_len, cursor: 0, kind: None, ranges: std::array::from_fn(|_| None), range_count: 0, items: 0, owned_bytes: 0, pending_range: None, output: None }
    }

    pub fn step<F>(&mut self, mut byte_at: F) -> ShellDecodeStep
    where
        F: FnMut(usize) -> Option<u8>,
    {
        if self.payload_len > BRIDGE_INBOUND_MAX_BYTES {
            return ShellDecodeStep::Fault(ShellDecodeFault::Capacity);
        }
        let result = match self.phase {
            ShellDecodePhase::Tag => self.read_tag(&mut byte_at),
            ShellDecodePhase::HelloVersion => self.read_u16(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::HelloKind)),
            ShellDecodePhase::HelloKind => self.read_u8(&mut byte_at).and_then(|kind| if kind <= 2 { self.next(ShellDecodePhase::HelloSession) } else { Err(ShellDecodeFault::Malformed) }),
            ShellDecodePhase::HelloSession => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::HelloPrincipal)),
            ShellDecodePhase::HelloPrincipal => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::HelloFlags)),
            ShellDecodePhase::HelloFlags => self.read_u8(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::Finish)),
            ShellDecodePhase::StateRevision => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::StateBytes)),
            ShellDecodePhase::StateBytes => self.read_range(&mut byte_at, ShellRangeKind::Bytes).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::Finish)),
            ShellDecodePhase::PatchRevision => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::PatchBaseRevision)),
            ShellDecodePhase::PatchBaseRevision => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::PatchBytes)),
            ShellDecodePhase::PatchBytes => self.read_range(&mut byte_at, ShellRangeKind::Bytes).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::Finish)),
            ShellDecodePhase::InstancesCount => self.read_count(&mut byte_at, 20).and_then(|count| self.next(if count == 0 { ShellDecodePhase::Finish } else { ShellDecodePhase::InstancePlugin { instances: count } })),
            ShellDecodePhase::InstancePlugin { instances } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::InstanceApp { instances })),
            ShellDecodePhase::InstanceApp { instances } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::InstanceId { instances })),
            ShellDecodePhase::InstanceId { instances } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::InstanceArtifact { instances })),
            ShellDecodePhase::InstanceArtifact { instances } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::InstanceWindowsCount { instances })),
            ShellDecodePhase::InstanceWindowsCount { instances } => self.read_count(&mut byte_at, 4).and_then(|windows| {
                self.next(if windows == 0 {
                    if instances == 1 {
                        ShellDecodePhase::Finish
                    } else {
                        ShellDecodePhase::InstancePlugin { instances: instances - 1 }
                    }
                } else {
                    ShellDecodePhase::InstanceWindow { instances, windows }
                })
            }),
            ShellDecodePhase::InstanceWindow { instances, windows } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| {
                self.next_after_range(granted, if windows > 1 {
                    ShellDecodePhase::InstanceWindow { instances, windows: windows - 1 }
                } else if instances > 1 {
                    ShellDecodePhase::InstancePlugin { instances: instances - 1 }
                } else {
                    ShellDecodePhase::Finish
                })
            }),
            ShellDecodePhase::AppReply => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::AppInstance)),
            ShellDecodePhase::AppInstance => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::AppFramesCount)),
            ShellDecodePhase::AppFramesCount => self.read_count(&mut byte_at, 4).and_then(|count| self.next(if count == 0 { ShellDecodePhase::Finish } else { ShellDecodePhase::AppFrame { frames: count } })),
            ShellDecodePhase::AppFrame { frames } => self.read_range(&mut byte_at, ShellRangeKind::Bytes).and_then(|granted| self.next_after_range(granted, if frames == 1 { ShellDecodePhase::Finish } else { ShellDecodePhase::AppFrame { frames: frames - 1 } })),
            ShellDecodePhase::CommandReply => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::CommandOk)),
            ShellDecodePhase::CommandOk => self.read_bool(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::CommandFaultFlag)),
            ShellDecodePhase::CommandFaultFlag => self.read_bool(&mut byte_at).and_then(|present| self.next(if present { ShellDecodePhase::CommandFaultString } else { ShellDecodePhase::Finish })),
            ShellDecodePhase::CommandFaultString => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::Finish)),
            ShellDecodePhase::ApprovalId => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::ApprovalDecision)),
            ShellDecodePhase::ApprovalDecision => self.read_u8(&mut byte_at).and_then(|decision| if decision <= 2 { self.next(ShellDecodePhase::ApprovalNoteFlag) } else { Err(ShellDecodeFault::Malformed) }),
            ShellDecodePhase::ApprovalNoteFlag => self.read_bool(&mut byte_at).and_then(|present| self.next(if present { ShellDecodePhase::ApprovalNoteString } else { ShellDecodePhase::Finish })),
            ShellDecodePhase::ApprovalNoteString => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::Finish)),
            ShellDecodePhase::AgentMessageId => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::AgentMessageText)),
            ShellDecodePhase::AgentMessageText => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::Finish)),
            ShellDecodePhase::AgentCancelInvocation => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|granted| self.next_after_range(granted, ShellDecodePhase::Finish)),
            ShellDecodePhase::Finish => self.finish_preflight(),
            ShellDecodePhase::ValidateStrings { range, offset, utf8 } => self.validate_string(range, offset, utf8, &mut byte_at),
            ShellDecodePhase::Copy { offset } => return self.copy_page(offset, &mut byte_at),
        };
        match result {
            Ok(()) => ShellDecodeStep::Pending,
            Err(fault) => ShellDecodeStep::Fault(fault),
        }
    }

    fn read_tag<F>(&mut self, byte_at: &mut F) -> Result<(), ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let tag = self.read_u8(byte_at)?;
        let (kind, phase) = match tag {
            0 => (ShellFrameKind::Hello, ShellDecodePhase::HelloVersion),
            1 => (ShellFrameKind::ShellState, ShellDecodePhase::StateRevision),
            2 => (ShellFrameKind::ShellStatePatch, ShellDecodePhase::PatchRevision),
            3 => (ShellFrameKind::Instances, ShellDecodePhase::InstancesCount),
            4 => (ShellFrameKind::AppFrames, ShellDecodePhase::AppReply),
            5 => (ShellFrameKind::ShellCommandResult, ShellDecodePhase::CommandReply),
            6 => (ShellFrameKind::Approval, ShellDecodePhase::ApprovalId),
            7 => (ShellFrameKind::Ping, ShellDecodePhase::Finish),
            8 => (ShellFrameKind::Bye, ShellDecodePhase::Finish),
            9 => (ShellFrameKind::AgentMessage, ShellDecodePhase::AgentMessageId),
            10 => (ShellFrameKind::AgentCancel, ShellDecodePhase::AgentCancelInvocation),
            _ => return Err(ShellDecodeFault::Malformed),
        };
        self.kind = Some(kind);
        self.phase = phase;
        Ok(())
    }

    fn read_u8<F>(&mut self, byte_at: &mut F) -> Result<u8, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let value = byte_at(self.cursor).ok_or(ShellDecodeFault::Malformed)?;
        self.cursor += 1;
        Ok(value)
    }

    fn read_u16<F>(&mut self, byte_at: &mut F) -> Result<u16, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let mut bytes = [0; 2];
        self.read_fixed(byte_at, &mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn read_u32<F>(&mut self, byte_at: &mut F) -> Result<u32, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let mut bytes = [0; 4];
        self.read_fixed(byte_at, &mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_u64<F>(&mut self, byte_at: &mut F) -> Result<u64, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let mut bytes = [0; 8];
        self.read_fixed(byte_at, &mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn read_fixed<F>(&mut self, byte_at: &mut F, output: &mut [u8]) -> Result<(), ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let end = self.cursor.checked_add(output.len()).ok_or(ShellDecodeFault::Capacity)?;
        if end > self.payload_len {
            return Err(ShellDecodeFault::Malformed);
        }
        for (offset, target) in output.iter_mut().enumerate() {
            *target = byte_at(self.cursor + offset).ok_or(ShellDecodeFault::Malformed)?;
        }
        self.cursor = end;
        Ok(())
    }

    fn read_bool<F>(&mut self, byte_at: &mut F) -> Result<bool, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        match self.read_u8(byte_at)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ShellDecodeFault::Malformed),
        }
    }

    fn read_count<F>(&mut self, byte_at: &mut F, minimum_item_bytes: usize) -> Result<usize, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let count = self.read_u32(byte_at)? as usize;
        if count > BRIDGE_INBOUND_MAX_ITEMS.saturating_sub(self.items) {
            return Err(ShellDecodeFault::Capacity);
        }
        let minimum = count.checked_mul(minimum_item_bytes).ok_or(ShellDecodeFault::Capacity)?;
        if minimum > self.payload_len.saturating_sub(self.cursor) {
            return Err(ShellDecodeFault::Malformed);
        }
        self.items += count;
        Ok(count)
    }

    /// 📏️ One length-prefixed field, admitted over TWO steps: the first reads and bounds-checks the
    /// `u32` length token alone, the second grants the range and skips the cursor past its body.
    /// Splitting them is the retained-turn law — a single step may consume one scalar token, never a
    /// whole megabyte-capped field — and `Ok(false)` means "grant still pending, stay in this phase".
    fn read_range<F>(&mut self, byte_at: &mut F, kind: ShellRangeKind) -> Result<bool, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        if let Some(len) = self.pending_range.take() {
            self.ranges[self.range_count] = Some(ShellRange { start: self.cursor, len, kind });
            self.range_count += 1;
            self.cursor += len;
            return Ok(true);
        }
        let len = self.read_u32(byte_at)? as usize;
        if len > BRIDGE_INBOUND_MAX_FIELD_BYTES || self.range_count == BRIDGE_INBOUND_MAX_RANGES {
            return Err(ShellDecodeFault::Capacity);
        }
        let end = self.cursor.checked_add(len).ok_or(ShellDecodeFault::Capacity)?;
        if end > self.payload_len {
            return Err(ShellDecodeFault::Malformed);
        }
        let owned_bytes = self.owned_bytes.checked_add(len).ok_or(ShellDecodeFault::Capacity)?;
        if owned_bytes > BRIDGE_INBOUND_MAX_BYTES {
            return Err(ShellDecodeFault::Capacity);
        }
        self.owned_bytes = owned_bytes;
        self.pending_range = Some(len);
        Ok(false)
    }

    pub fn next(&mut self, phase: ShellDecodePhase) -> Result<(), ShellDecodeFault> {
        self.phase = phase;
        Ok(())
    }

    /// ➡️ Advances only once [`Self::read_range`] actually granted the range; a pending grant keeps
    /// the current phase so the very next step performs it.
    fn next_after_range(&mut self, granted: bool, phase: ShellDecodePhase) -> Result<(), ShellDecodeFault> {
        if granted {
            self.phase = phase;
        }
        Ok(())
    }

    fn finish_preflight(&mut self) -> Result<(), ShellDecodeFault> {
        if self.cursor != self.payload_len {
            return Err(ShellDecodeFault::Malformed);
        }
        self.phase = if self.range_count == 0 { ShellDecodePhase::Copy { offset: 0 } } else { ShellDecodePhase::ValidateStrings { range: 0, offset: 0, utf8: Utf8Cursor::new() } };
        Ok(())
    }

    fn validate_string<F>(&mut self, range_index: usize, offset: usize, mut utf8: Utf8Cursor, byte_at: &mut F) -> Result<(), ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let range = self.ranges[range_index].ok_or(ShellDecodeFault::Malformed)?;
        if matches!(range.kind, ShellRangeKind::Bytes) || offset == range.len {
            if matches!(range.kind, ShellRangeKind::String) && !utf8.complete() {
                return Err(ShellDecodeFault::Malformed);
            }
            self.phase = if range_index + 1 == self.range_count { ShellDecodePhase::Copy { offset: 0 } } else { ShellDecodePhase::ValidateStrings { range: range_index + 1, offset: 0, utf8: Utf8Cursor::new() } };
            return Ok(());
        }
        let byte = byte_at(range.start + offset).ok_or(ShellDecodeFault::Malformed)?;
        if !utf8.push(byte) {
            return Err(ShellDecodeFault::Malformed);
        }
        self.phase = ShellDecodePhase::ValidateStrings { range: range_index, offset: offset + 1, utf8 };
        Ok(())
    }

    pub fn copy_page<F>(&mut self, offset: usize, byte_at: &mut F) -> ShellDecodeStep
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let output = self.output.get_or_insert_with(|| ValidatedShellFrame::new(self.kind.expect("validated bridge tag missing"), self.payload_len));
        let bytes = BRIDGE_INBOUND_PAGE_BYTES.min(self.payload_len.saturating_sub(offset));
        if bytes == 0 {
            return ShellDecodeStep::Complete(self.output.take().expect("validated bridge output missing"));
        }
        let mut page = Box::new([0; BRIDGE_INBOUND_PAGE_BYTES]);
        for (index, target) in page[..bytes].iter_mut().enumerate() {
            let Some(value) = byte_at(offset + index) else { return ShellDecodeStep::Fault(ShellDecodeFault::Malformed) };
            *target = value;
        }
        output.pages[offset / BRIDGE_INBOUND_PAGE_BYTES] = Some(page);
        let next = offset + bytes;
        if next == self.payload_len {
            ShellDecodeStep::Complete(self.output.take().expect("validated bridge output missing"))
        } else {
            self.phase = ShellDecodePhase::Copy { offset: next };
            ShellDecodeStep::Pending
        }
    }
}

pub(crate) struct ValidatedShellFrame {
    pub kind: ShellFrameKind,
    pub len: usize,
    pub pages: [Option<Box<[u8; BRIDGE_INBOUND_PAGE_BYTES]>>; BRIDGE_INBOUND_MAX_PAGES],
}

impl ValidatedShellFrame {
    fn new(kind: ShellFrameKind, len: usize) -> Self {
        Self { kind, len, pages: std::array::from_fn(|_| None) }
    }

    pub fn kind(&self) -> ShellFrameKind {
        self.kind
    }

    pub fn byte(&self, index: usize) -> Result<u8, ShellDecodeFault> {
        if index >= self.len {
            return Err(ShellDecodeFault::Malformed);
        }
        let page = self.pages[index / BRIDGE_INBOUND_PAGE_BYTES].as_ref().ok_or(ShellDecodeFault::Malformed)?;
        Ok(page[index % BRIDGE_INBOUND_PAGE_BYTES])
    }

    pub fn copy_into(&self, offset: usize, output: &mut [u8]) -> Result<(), ShellDecodeFault> {
        if offset.checked_add(output.len()).ok_or(ShellDecodeFault::Capacity)? > self.len {
            return Err(ShellDecodeFault::Malformed);
        }
        for (index, target) in output.iter_mut().enumerate() {
            *target = self.byte(offset + index)?;
        }
        Ok(())
    }
}

pub enum ShellMaterializeStep {
    Pending,
    Complete(ShellToGateway),
    Fault(ShellDecodeFault),
}

enum OwnedRange {
    Bytes { len: usize, copied: usize, value: Vec<u8> },
    String { end: usize, value: String },
}

impl OwnedRange {
    pub fn bytes(len: usize) -> Result<Self, ShellDecodeFault> {
        let mut value = Vec::new();
        value.try_reserve_exact(len).map_err(|_| ShellDecodeFault::Capacity)?;
        Ok(Self::Bytes { len, copied: 0, value })
    }

    fn string(start: usize, len: usize) -> Result<Self, ShellDecodeFault> {
        let mut value = String::new();
        value.try_reserve_exact(len).map_err(|_| ShellDecodeFault::Capacity)?;
        Ok(Self::String { end: start.checked_add(len).ok_or(ShellDecodeFault::Capacity)?, value })
    }

    pub fn step(&mut self, frame: &ValidatedShellFrame, position: &mut usize) -> Result<bool, ShellDecodeFault> {
        match self {
            Self::Bytes { len, copied, value } => {
                let bytes = BRIDGE_INBOUND_PAGE_BYTES.min(len.saturating_sub(*copied));
                if bytes == 0 {
                    return Ok(true);
                }
                let start = value.len();
                value.resize(start + bytes, 0);
                frame.copy_into(*position, &mut value[start..])?;
                *position += bytes;
                *copied += bytes;
                Ok(*copied == *len)
            }
            Self::String { end, value } => {
                if *position == *end {
                    return Ok(true);
                }
                let first = frame.byte(*position)?;
                let width = match first {
                    0x00..=0x7f => 1,
                    0xc2..=0xdf => 2,
                    0xe0..=0xef => 3,
                    0xf0..=0xf4 => 4,
                    _ => return Err(ShellDecodeFault::Malformed),
                };
                if position.checked_add(width).ok_or(ShellDecodeFault::Capacity)? > *end {
                    return Err(ShellDecodeFault::Malformed);
                }
                let mut bytes = [0; 4];
                frame.copy_into(*position, &mut bytes[..width])?;
                let text = std::str::from_utf8(&bytes[..width]).map_err(|_| ShellDecodeFault::Malformed)?;
                value.push_str(text);
                *position += width;
                Ok(*position == *end)
            }
        }
    }

    fn take_bytes(self) -> Result<Vec<u8>, ShellDecodeFault> {
        match self {
            Self::Bytes { value, .. } => Ok(value),
            Self::String { .. } => Err(ShellDecodeFault::Malformed),
        }
    }

    fn take_string(self) -> Result<String, ShellDecodeFault> {
        match self {
            Self::String { value, .. } => Ok(value),
            Self::Bytes { .. } => Err(ShellDecodeFault::Malformed),
        }
    }
}

#[derive(Clone, Copy)]
enum ShellMaterializePhase {
    Tag,
    HelloVersion,
    HelloKind,
    HelloSession,
    HelloPrincipal,
    HelloFlags,
    StateRevision,
    StateBytes,
    PatchRevision,
    PatchBaseRevision,
    PatchBytes,
    InstancesCount,
    InstancePlugin { remaining: usize },
    InstanceApp { remaining: usize },
    InstanceId { remaining: usize },
    InstanceArtifact { remaining: usize },
    InstanceWindowsCount { remaining: usize },
    InstanceWindow { remaining: usize, windows: usize },
    InstanceCommit { remaining: usize },
    AppReply,
    AppInstance,
    AppFramesCount,
    AppFrame { remaining: usize },
    AppFrameCommit { remaining: usize },
    CommandReply,
    CommandOk,
    CommandFaultFlag,
    CommandFault,
    ApprovalId,
    ApprovalDecision,
    ApprovalNoteFlag,
    ApprovalNote,
    AgentMessageId,
    AgentMessageText,
    AgentCancelInvocation,
    Finish,
}

pub struct ShellToGatewayMaterializeCursor {
    pub frame: ValidatedShellFrame,
    pub phase: ShellMaterializePhase,
    pub position: usize,
    range: Option<OwnedRange>,
    u64_a: u64,
    u64_b: u64,
    bool_a: bool,
    u16_a: u16,
    u8_a: u8,
    text_a: Option<String>,
    text_b: Option<String>,
    text_c: Option<String>,
    text_d: Option<String>,
    bytes_a: Option<Vec<u8>>,
    pub entries: Vec<BridgeInstanceRef>,
    pub windows: Vec<String>,
    pub frames: Vec<Vec<u8>>,
}

impl ShellToGatewayMaterializeCursor {
    pub fn new(frame: ValidatedShellFrame) -> Self {
        Self {
            frame,
            phase: ShellMaterializePhase::Tag,
            position: 0,
            range: None,
            u64_a: 0,
            u64_b: 0,
            bool_a: false,
            u16_a: 0,
            u8_a: 0,
            text_a: None,
            text_b: None,
            text_c: None,
            text_d: None,
            bytes_a: None,
            entries: Vec::new(),
            windows: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn kind(&self) -> ShellFrameKind {
        self.frame.kind()
    }

    pub fn step(&mut self) -> ShellMaterializeStep {
        let result = self.step_inner();
        match result {
            Ok(Some(frame)) => ShellMaterializeStep::Complete(frame),
            Ok(None) => ShellMaterializeStep::Pending,
            Err(fault) => ShellMaterializeStep::Fault(fault),
        }
    }

    fn step_inner(&mut self) -> Result<Option<ShellToGateway>, ShellDecodeFault> {
        match self.phase {
            ShellMaterializePhase::Tag => {
                let tag = self.read_u8()?;
                self.phase = match tag {
                    0 => ShellMaterializePhase::HelloVersion,
                    1 => ShellMaterializePhase::StateRevision,
                    2 => ShellMaterializePhase::PatchRevision,
                    3 => ShellMaterializePhase::InstancesCount,
                    4 => ShellMaterializePhase::AppReply,
                    5 => ShellMaterializePhase::CommandReply,
                    6 => ShellMaterializePhase::ApprovalId,
                    7 | 8 => ShellMaterializePhase::Finish,
                    9 => ShellMaterializePhase::AgentMessageId,
                    10 => ShellMaterializePhase::AgentCancelInvocation,
                    _ => return Err(ShellDecodeFault::Malformed),
                };
            }
            ShellMaterializePhase::HelloVersion => {
                self.u16_a = self.read_u16()?;
                self.phase = ShellMaterializePhase::HelloKind;
            }
            ShellMaterializePhase::HelloKind => {
                self.u8_a = self.read_u8()?;
                self.phase = ShellMaterializePhase::HelloSession;
            }
            ShellMaterializePhase::HelloSession => {
                if let Some(value) = self.step_string()? {
                    self.text_a = Some(value);
                    self.phase = ShellMaterializePhase::HelloPrincipal;
                }
            }
            ShellMaterializePhase::HelloPrincipal => {
                if let Some(value) = self.step_string()? {
                    self.text_b = Some(value);
                    self.phase = ShellMaterializePhase::HelloFlags;
                }
            }
            ShellMaterializePhase::HelloFlags => {
                self.bool_a = false;
                self.u64_a = self.read_u8()? as u64;
                self.phase = ShellMaterializePhase::Finish;
            }
            ShellMaterializePhase::StateRevision => {
                self.u64_a = self.read_u64()?;
                self.phase = ShellMaterializePhase::StateBytes;
            }
            ShellMaterializePhase::StateBytes => {
                if let Some(value) = self.step_bytes()? {
                    self.bytes_a = Some(value);
                    self.phase = ShellMaterializePhase::Finish;
                }
            }
            ShellMaterializePhase::PatchRevision => {
                self.u64_a = self.read_u64()?;
                self.phase = ShellMaterializePhase::PatchBaseRevision;
            }
            ShellMaterializePhase::PatchBaseRevision => {
                self.u64_b = self.read_u64()?;
                self.phase = ShellMaterializePhase::PatchBytes;
            }
            ShellMaterializePhase::PatchBytes => {
                if let Some(value) = self.step_bytes()? {
                    self.bytes_a = Some(value);
                    self.phase = ShellMaterializePhase::Finish;
                }
            }
            ShellMaterializePhase::InstancesCount => {
                let count = self.read_u32()? as usize;
                self.entries.try_reserve_exact(count).map_err(|_| ShellDecodeFault::Capacity)?;
                self.phase = if count == 0 { ShellMaterializePhase::Finish } else { ShellMaterializePhase::InstancePlugin { remaining: count } };
            }
            ShellMaterializePhase::InstancePlugin { remaining } => {
                if let Some(value) = self.step_string()? {
                    self.text_a = Some(value);
                    self.phase = ShellMaterializePhase::InstanceApp { remaining };
                }
            }
            ShellMaterializePhase::InstanceApp { remaining } => {
                if let Some(value) = self.step_string()? {
                    self.text_b = Some(value);
                    self.phase = ShellMaterializePhase::InstanceId { remaining };
                }
            }
            ShellMaterializePhase::InstanceId { remaining } => {
                if let Some(value) = self.step_string()? {
                    self.text_c = Some(value);
                    self.phase = ShellMaterializePhase::InstanceArtifact { remaining };
                }
            }
            ShellMaterializePhase::InstanceArtifact { remaining } => {
                if let Some(value) = self.step_string()? {
                    self.text_d = Some(value);
                    self.phase = ShellMaterializePhase::InstanceWindowsCount { remaining };
                }
            }
            ShellMaterializePhase::InstanceWindowsCount { remaining } => {
                let windows = self.read_u32()? as usize;
                self.windows.try_reserve_exact(windows).map_err(|_| ShellDecodeFault::Capacity)?;
                if windows == 0 {
                    self.phase = ShellMaterializePhase::InstanceCommit { remaining };
                } else {
                    self.phase = ShellMaterializePhase::InstanceWindow { remaining, windows };
                }
            }
            ShellMaterializePhase::InstanceWindow { remaining, windows } => {
                if let Some(value) = self.step_string()? {
                    self.windows.push(value);
                    if windows == 1 {
                        self.phase = ShellMaterializePhase::InstanceCommit { remaining };
                    } else {
                        self.phase = ShellMaterializePhase::InstanceWindow { remaining, windows: windows - 1 };
                    }
                }
            }
            ShellMaterializePhase::InstanceCommit { remaining } => self.finish_instance(remaining)?,
            ShellMaterializePhase::AppReply => {
                self.u64_a = self.read_u64()?;
                self.phase = ShellMaterializePhase::AppInstance;
            }
            ShellMaterializePhase::AppInstance => {
                if let Some(value) = self.step_string()? {
                    self.text_a = Some(value);
                    self.phase = ShellMaterializePhase::AppFramesCount;
                }
            }
            ShellMaterializePhase::AppFramesCount => {
                let count = self.read_u32()? as usize;
                self.frames.try_reserve_exact(count).map_err(|_| ShellDecodeFault::Capacity)?;
                self.phase = if count == 0 { ShellMaterializePhase::Finish } else { ShellMaterializePhase::AppFrame { remaining: count } };
            }
            ShellMaterializePhase::AppFrame { remaining } => {
                if let Some(value) = self.step_bytes()? {
                    self.bytes_a = Some(value);
                    self.phase = ShellMaterializePhase::AppFrameCommit { remaining };
                }
            }
            ShellMaterializePhase::AppFrameCommit { remaining } => {
                self.frames.push(self.bytes_a.take().ok_or(ShellDecodeFault::Malformed)?);
                self.phase = if remaining == 1 { ShellMaterializePhase::Finish } else { ShellMaterializePhase::AppFrame { remaining: remaining - 1 } };
            }
            ShellMaterializePhase::CommandReply => {
                self.u64_a = self.read_u64()?;
                self.phase = ShellMaterializePhase::CommandOk;
            }
            ShellMaterializePhase::CommandOk => {
                self.bool_a = self.read_bool()?;
                self.phase = ShellMaterializePhase::CommandFaultFlag;
            }
            ShellMaterializePhase::CommandFaultFlag => {
                self.phase = if self.read_bool()? { ShellMaterializePhase::CommandFault } else { ShellMaterializePhase::Finish };
            }
            ShellMaterializePhase::CommandFault => {
                if let Some(value) = self.step_string()? {
                    self.text_a = Some(value);
                    self.phase = ShellMaterializePhase::Finish;
                }
            }
            ShellMaterializePhase::ApprovalId => {
                if let Some(value) = self.step_string()? {
                    self.text_a = Some(value);
                    self.phase = ShellMaterializePhase::ApprovalDecision;
                }
            }
            ShellMaterializePhase::ApprovalDecision => {
                self.u8_a = self.read_u8()?;
                self.phase = ShellMaterializePhase::ApprovalNoteFlag;
            }
            ShellMaterializePhase::ApprovalNoteFlag => {
                self.phase = if self.read_bool()? { ShellMaterializePhase::ApprovalNote } else { ShellMaterializePhase::Finish };
            }
            ShellMaterializePhase::ApprovalNote => {
                if let Some(value) = self.step_string()? {
                    self.text_b = Some(value);
                    self.phase = ShellMaterializePhase::Finish;
                }
            }
            ShellMaterializePhase::AgentMessageId => {
                if let Some(value) = self.step_string()? {
                    self.text_a = Some(value);
                    self.phase = ShellMaterializePhase::AgentMessageText;
                }
            }
            ShellMaterializePhase::AgentMessageText => {
                if let Some(value) = self.step_string()? {
                    self.text_b = Some(value);
                    self.phase = ShellMaterializePhase::Finish;
                }
            }
            ShellMaterializePhase::AgentCancelInvocation => {
                if let Some(value) = self.step_string()? {
                    self.text_a = Some(value);
                    self.phase = ShellMaterializePhase::Finish;
                }
            }
            ShellMaterializePhase::Finish => return self.finish().map(Some),
        }
        Ok(None)
    }

    fn finish_instance(&mut self, remaining: usize) -> Result<(), ShellDecodeFault> {
        self.entries.push(BridgeInstanceRef {
            plugin_id: self.text_a.take().ok_or(ShellDecodeFault::Malformed)?,
            app_id: self.text_b.take().ok_or(ShellDecodeFault::Malformed)?,
            instance_id: self.text_c.take().ok_or(ShellDecodeFault::Malformed)?,
            artifact_ref: self.text_d.take().ok_or(ShellDecodeFault::Malformed)?,
            window_ids: std::mem::take(&mut self.windows),
        });
        self.phase = if remaining == 1 { ShellMaterializePhase::Finish } else { ShellMaterializePhase::InstancePlugin { remaining: remaining - 1 } };
        Ok(())
    }

    fn step_string(&mut self) -> Result<Option<String>, ShellDecodeFault> {
        if self.range.is_none() {
            let len = self.read_u32()? as usize;
            self.range = Some(OwnedRange::string(self.position, len)?);
            return Ok(None);
        }
        if !self.range.as_mut().expect("bridge range missing").step(&self.frame, &mut self.position)? {
            return Ok(None);
        }
        self.range.take().expect("bridge range missing").take_string().map(Some)
    }

    fn step_bytes(&mut self) -> Result<Option<Vec<u8>>, ShellDecodeFault> {
        if self.range.is_none() {
            let len = self.read_u32()? as usize;
            self.range = Some(OwnedRange::bytes(len)?);
            return Ok(None);
        }
        if !self.range.as_mut().expect("bridge range missing").step(&self.frame, &mut self.position)? {
            return Ok(None);
        }
        self.range.take().expect("bridge range missing").take_bytes().map(Some)
    }

    fn read_u8(&mut self) -> Result<u8, ShellDecodeFault> {
        let value = self.frame.byte(self.position)?;
        self.position += 1;
        Ok(value)
    }

    fn read_bool(&mut self) -> Result<bool, ShellDecodeFault> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ShellDecodeFault::Malformed),
        }
    }

    fn read_u16(&mut self) -> Result<u16, ShellDecodeFault> {
        let mut bytes = [0; 2];
        self.frame.copy_into(self.position, &mut bytes)?;
        self.position += bytes.len();
        Ok(u16::from_le_bytes(bytes))
    }

    fn read_u32(&mut self) -> Result<u32, ShellDecodeFault> {
        let mut bytes = [0; 4];
        self.frame.copy_into(self.position, &mut bytes)?;
        self.position += bytes.len();
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_u64(&mut self) -> Result<u64, ShellDecodeFault> {
        let mut bytes = [0; 8];
        self.frame.copy_into(self.position, &mut bytes)?;
        self.position += bytes.len();
        Ok(u64::from_le_bytes(bytes))
    }

    pub fn finish(&mut self) -> Result<ShellToGateway, ShellDecodeFault> {
        if self.position != self.frame.len {
            return Err(ShellDecodeFault::Malformed);
        }
        Ok(match self.frame.kind() {
            ShellFrameKind::Hello => ShellToGateway::Hello {
                bridge_version: self.u16_a,
                shell_kind: match self.u8_a {
                    0 => ShellKind::React,
                    1 => ShellKind::WgpuWeb,
                    2 => ShellKind::WgpuNative,
                    _ => return Err(ShellDecodeFault::Malformed),
                },
                shell_session_id: self.text_a.take().ok_or(ShellDecodeFault::Malformed)?,
                principal_actor: self.text_b.take().ok_or(ShellDecodeFault::Malformed)?,
                flags: BridgeFlags::from_bits(self.u64_a as u8),
            },
            ShellFrameKind::ShellState => ShellToGateway::ShellState { revision: self.u64_a, state: self.bytes_a.take().ok_or(ShellDecodeFault::Malformed)? },
            ShellFrameKind::ShellStatePatch => ShellToGateway::ShellStatePatch { revision: self.u64_a, base_revision: self.u64_b, patch: self.bytes_a.take().ok_or(ShellDecodeFault::Malformed)? },
            ShellFrameKind::Instances => ShellToGateway::Instances { entries: std::mem::take(&mut self.entries) },
            ShellFrameKind::AppFrames => ShellToGateway::AppFrames { in_reply_to: self.u64_a, instance_id: self.text_a.take().ok_or(ShellDecodeFault::Malformed)?, frames: std::mem::take(&mut self.frames) },
            ShellFrameKind::ShellCommandResult => ShellToGateway::ShellCommandResult { in_reply_to: self.u64_a, ok: self.bool_a, fault: self.text_a.take() },
            ShellFrameKind::Approval => ShellToGateway::Approval {
                approval_id: self.text_a.take().ok_or(ShellDecodeFault::Malformed)?,
                decision: match self.u8_a {
                    0 => ApprovalDecision::Deny,
                    1 => ApprovalDecision::Once,
                    2 => ApprovalDecision::Session,
                    _ => return Err(ShellDecodeFault::Malformed),
                },
                note: self.text_b.take(),
            },
            ShellFrameKind::Ping => ShellToGateway::Ping,
            ShellFrameKind::Bye => ShellToGateway::Bye,
            ShellFrameKind::AgentMessage => ShellToGateway::AgentMessage { message_id: self.text_a.take().ok_or(ShellDecodeFault::Malformed)?, text: self.text_b.take().ok_or(ShellDecodeFault::Malformed)? },
            ShellFrameKind::AgentCancel => ShellToGateway::AgentCancel { invocation_id: self.text_a.take().ok_or(ShellDecodeFault::Malformed)? },
        })
    }
}

//#endregion 🔖️BoundedShellDecode

//#region 🔖️GatewayToShell

/// 📤️ Gateway→Shell frames, tag 0..12 in this exact declaration order (`📋️master.md` §2.2); a new
/// variant only ever goes last. Wire fixture: `🧫️fixtures/📨️frames.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "variant", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GatewayToShell {
    Welcome { bridge_version: u16, connection: String, principal: String },
    ShellCommand { seq: u64, command: Vec<u8> },
    AppCommand { seq: u64, instance_id: String, command: Vec<u8> },
    ApprovalRequested { approval_id: String, summary: String },
    ApprovalResolved { approval_id: String, decision: ApprovalDecision },
    AgentPresence { active: bool, label: String, invocation_id: Option<String> },
    Pong,
    Bye { reason: String },
    /// 🛠️ One tool the connected agent is invoking RIGHT NOW, emitted from the gateway's own
    /// `tools/call` dispatch before the handler runs. `arguments` is the call's arguments rendered
    /// as JSON and truncated to [`AGENT_CONVERSATION_MAX_TEXT`]; the shell renders it, never
    /// re-executes it.
    AgentToolCall { invocation_id: String, tool_name: String, arguments: String },
    /// 🧾️ The terminal outcome of the `AgentToolCall` with the same `invocation_id` — `ok` is the
    /// inverse of the result's own `isError`, `summary` its first text block (or the gateway error
    /// message), truncated the same way.
    AgentToolResult { invocation_id: String, tool_name: String, ok: bool, summary: String },
    /// 💬️ One chunk of the connected agent's OWN free-text turn — the channel that lets an agent
    /// explain, ask a clarifying question, or answer at all, rather than only act. `reply_id`
    /// identifies the turn (every chunk of one turn repeats it, so a shell appends instead of
    /// stacking rows); `in_reply_to` is the `message_id` of the `ShellToGateway::AgentMessage` this
    /// answers, or `None` for a turn the agent opened itself; `complete` marks the last chunk, so a
    /// shell can show a turn as still arriving without inventing a heuristic. The text carries no
    /// locale: it is the agent's own words, and the only translated strings on the surface are the
    /// role nouns around them.
    AgentReply { reply_id: String, in_reply_to: Option<String>, text: String, complete: bool },
    /// 🪦️ The approval request `approval_id` is withdrawn — its call was cancelled, it timed out, or it
    /// moved to a newer shell connection — so the shell retires its affordance and says why.
    ApprovalWithdrawn { approval_id: String, reason: ApprovalWithdrawal },
    /// 🚫️ The gateway will not serve this connection — see [`BridgeRefusal`]; `gateway_version` is the
    /// gateway's own [`BRIDGE_VERSION`], so a shell can say which side is older. The gateway closes the
    /// socket right after it.
    Refused { reason: BridgeRefusal, gateway_version: u16 },
}

/// 💬️ Tools whose OWN result is already a conversation frame, so `tools/call` must not also publish
/// an `AgentToolCall`/`AgentToolResult` pair for them: `conversation_reply`'s arguments ARE the
/// prose the panel is about to render, and a tool-call row would print every sentence the agent says
/// twice. The audit sink still records the call — this list governs the shell projection only.
pub const SELF_PUBLISHING_CONVERSATION_TOOLS: &[&str] = &["conversation_reply"];

/// ✂️ The hard cap on every human-readable string the agent-conversation frames carry. The bridge
/// outbox admits a bounded number of bounded frames, so an agent calling a tool with a megabyte of
/// arguments must not be able to starve it — the text is truncated with a visible marker rather
/// than silently dropped or allowed through.
pub const AGENT_CONVERSATION_MAX_TEXT: usize = 2_048;

/// ✂️ [`AGENT_CONVERSATION_MAX_TEXT`], applied on a char boundary so the result is always UTF-8.
pub fn truncate_conversation_text(text: &str) -> String {
    if text.len() <= AGENT_CONVERSATION_MAX_TEXT {
        return text.to_string();
    }
    let mut end = AGENT_CONVERSATION_MAX_TEXT;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &text[..end])
}

impl GatewayToShell {
    pub fn encoded_len(&self) -> Option<usize> {
        match self {
            GatewayToShell::Welcome { connection, principal, .. } => 3usize.checked_add(bridge_wire_field_len(connection.len())?)?.checked_add(bridge_wire_field_len(principal.len())?),
            GatewayToShell::ShellCommand { command, .. } => 9usize.checked_add(bridge_wire_field_len(command.len())?),
            GatewayToShell::AppCommand { instance_id, command, .. } => 9usize.checked_add(bridge_wire_field_len(instance_id.len())?)?.checked_add(bridge_wire_field_len(command.len())?),
            GatewayToShell::ApprovalRequested { approval_id, summary } => 1usize.checked_add(bridge_wire_field_len(approval_id.len())?)?.checked_add(bridge_wire_field_len(summary.len())?),
            GatewayToShell::ApprovalResolved { approval_id, .. } => 2usize.checked_add(bridge_wire_field_len(approval_id.len())?),
            GatewayToShell::AgentPresence { label, invocation_id, .. } => {
                let base = 3usize.checked_add(bridge_wire_field_len(label.len())?)?;
                invocation_id.as_ref().map_or(Some(base), |value| base.checked_add(bridge_wire_field_len(value.len())?))
            }
            GatewayToShell::Pong => Some(1),
            GatewayToShell::Bye { reason } => 1usize.checked_add(bridge_wire_field_len(reason.len())?),
            GatewayToShell::AgentToolCall { invocation_id, tool_name, arguments } => {
                1usize.checked_add(bridge_wire_field_len(invocation_id.len())?)?.checked_add(bridge_wire_field_len(tool_name.len())?)?.checked_add(bridge_wire_field_len(arguments.len())?)
            }
            GatewayToShell::AgentToolResult { invocation_id, tool_name, summary, .. } => {
                2usize.checked_add(bridge_wire_field_len(invocation_id.len())?)?.checked_add(bridge_wire_field_len(tool_name.len())?)?.checked_add(bridge_wire_field_len(summary.len())?)
            }
            GatewayToShell::AgentReply { reply_id, in_reply_to, text, .. } => {
                let base = 3usize.checked_add(bridge_wire_field_len(reply_id.len())?)?.checked_add(bridge_wire_field_len(text.len())?)?;
                in_reply_to.as_ref().map_or(Some(base), |value| base.checked_add(bridge_wire_field_len(value.len())?))
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, .. } => 2usize.checked_add(bridge_wire_field_len(approval_id.len())?),
            GatewayToShell::Refused { .. } => Some(4),
        }
    }

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
            GatewayToShell::AgentToolCall { invocation_id, tool_name, arguments } => {
                wire::write_u8(&mut buf, 8);
                wire::write_string(&mut buf, invocation_id);
                wire::write_string(&mut buf, tool_name);
                wire::write_string(&mut buf, arguments);
            }
            GatewayToShell::AgentToolResult { invocation_id, tool_name, ok, summary } => {
                wire::write_u8(&mut buf, 9);
                wire::write_string(&mut buf, invocation_id);
                wire::write_string(&mut buf, tool_name);
                wire::write_bool(&mut buf, *ok);
                wire::write_string(&mut buf, summary);
            }
            GatewayToShell::AgentReply { reply_id, in_reply_to, text, complete } => {
                wire::write_u8(&mut buf, 10);
                wire::write_string(&mut buf, reply_id);
                wire::write_option_string(&mut buf, in_reply_to);
                wire::write_string(&mut buf, text);
                wire::write_bool(&mut buf, *complete);
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, reason } => {
                wire::write_u8(&mut buf, 11);
                wire::write_string(&mut buf, approval_id);
                wire::write_u8(&mut buf, reason.to_tag());
            }
            GatewayToShell::Refused { reason, gateway_version } => {
                wire::write_u8(&mut buf, 12);
                wire::write_u8(&mut buf, reason.to_tag());
                wire::write_u16(&mut buf, *gateway_version);
            }
        }
        buf
    }

    pub fn copy_encoded_page(&self, offset: usize, output: &mut [u8]) -> usize {
        let mut writer = BridgeEncodedPageWriter::new(offset, output);
        match self {
            Self::Welcome { bridge_version, connection, principal } => {
                writer.push(&[0]);
                writer.push(&bridge_version.to_le_bytes());
                writer.field(connection.as_bytes());
                writer.field(principal.as_bytes());
            }
            Self::ShellCommand { seq, command } => {
                writer.push(&[1]);
                writer.push(&seq.to_le_bytes());
                writer.field(command);
            }
            Self::AppCommand { seq, instance_id, command } => {
                writer.push(&[2]);
                writer.push(&seq.to_le_bytes());
                writer.field(instance_id.as_bytes());
                writer.field(command);
            }
            Self::ApprovalRequested { approval_id, summary } => {
                writer.push(&[3]);
                writer.field(approval_id.as_bytes());
                writer.field(summary.as_bytes());
            }
            Self::ApprovalResolved { approval_id, decision } => {
                writer.push(&[4]);
                writer.field(approval_id.as_bytes());
                writer.push(&[decision.to_tag()]);
            }
            Self::AgentPresence { active, label, invocation_id } => {
                writer.push(&[5, *active as u8]);
                writer.field(label.as_bytes());
                writer.push(&[invocation_id.is_some() as u8]);
                if let Some(invocation_id) = invocation_id {
                    writer.field(invocation_id.as_bytes());
                }
            }
            Self::Pong => writer.push(&[6]),
            Self::Bye { reason } => {
                writer.push(&[7]);
                writer.field(reason.as_bytes());
            }
            Self::AgentToolCall { invocation_id, tool_name, arguments } => {
                writer.push(&[8]);
                writer.field(invocation_id.as_bytes());
                writer.field(tool_name.as_bytes());
                writer.field(arguments.as_bytes());
            }
            Self::AgentToolResult { invocation_id, tool_name, ok, summary } => {
                writer.push(&[9]);
                writer.field(invocation_id.as_bytes());
                writer.field(tool_name.as_bytes());
                writer.push(&[*ok as u8]);
                writer.field(summary.as_bytes());
            }
            Self::AgentReply { reply_id, in_reply_to, text, complete } => {
                writer.push(&[10]);
                writer.field(reply_id.as_bytes());
                writer.push(&[in_reply_to.is_some() as u8]);
                if let Some(in_reply_to) = in_reply_to {
                    writer.field(in_reply_to.as_bytes());
                }
                writer.field(text.as_bytes());
                writer.push(&[*complete as u8]);
            }
            Self::ApprovalWithdrawn { approval_id, reason } => {
                writer.push(&[11]);
                writer.field(approval_id.as_bytes());
                writer.push(&[reason.to_tag()]);
            }
            Self::Refused { reason, gateway_version } => {
                writer.push(&[12, reason.to_tag()]);
                writer.push(&gateway_version.to_le_bytes());
            }
        }
        writer.written
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
            8 => GatewayToShell::AgentToolCall { invocation_id: reader.read_string()?, tool_name: reader.read_string()?, arguments: reader.read_string()? },
            9 => GatewayToShell::AgentToolResult { invocation_id: reader.read_string()?, tool_name: reader.read_string()?, ok: reader.read_bool()?, summary: reader.read_string()? },
            10 => GatewayToShell::AgentReply { reply_id: reader.read_string()?, in_reply_to: reader.read_option_string()?, text: reader.read_string()?, complete: reader.read_bool()? },
            11 => GatewayToShell::ApprovalWithdrawn { approval_id: reader.read_string()?, reason: ApprovalWithdrawal::from_tag(reader.read_u8()?)? },
            12 => GatewayToShell::Refused { reason: BridgeRefusal::from_tag(reader.read_u8()?)?, gateway_version: reader.read_u16()? },
            other => return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown GatewayToShell tag {other}"))),
        };
        reader.finish()?;
        Ok(frame)
    }
}

struct BridgeEncodedPageWriter<'a> {
    pub offset: usize,
    pub position: usize,
    pub output: &'a mut [u8],
    pub written: usize,
}

impl<'a> BridgeEncodedPageWriter<'a> {
    fn new(offset: usize, output: &'a mut [u8]) -> Self {
        Self { offset, position: 0, output, written: 0 }
    }

    pub fn field(&mut self, bytes: &[u8]) {
        self.push(&(bytes.len() as u32).to_le_bytes());
        self.push(bytes);
    }

    pub fn push(&mut self, bytes: &[u8]) {
        let start = self.position;
        self.position += bytes.len();
        let overlap_start = self.offset.max(start);
        let overlap_end = (self.offset + self.output.len()).min(self.position);
        if overlap_start >= overlap_end {
            return;
        }
        let source = overlap_start - start;
        let count = overlap_end - overlap_start;
        let target = overlap_start - self.offset;
        self.output[target..target + count].copy_from_slice(&bytes[source..source + count]);
        self.written = self.written.max(target + count);
    }
}

pub fn bridge_wire_field_len(bytes: usize) -> Option<usize> {
    u32::try_from(bytes).ok()?;
    4usize.checked_add(bytes)
}

//#endregion 🔖️GatewayToShell

//#region 🔖️BridgeServer
//#endregion 🔖️BridgeServer

//#region 🧪️Tests
//#endregion 🧪️Tests
