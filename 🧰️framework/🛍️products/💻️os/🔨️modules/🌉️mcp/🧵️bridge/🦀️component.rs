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
//! The live bridge is owned by the retained HTTP transport through `🔖️BridgeHandle`; the Axum
//! `🔖️BridgeServer` remains a `cfg(test)` differential oracle only.

use crate::errors::{GatewayError, GatewayErrorCode};
use semio_framework_async::{Job, Lane, ProcessKind, WorkerPool, WorkerPoolConfig, WorkerSubmitErrorKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
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

//#region 🔖️BoundedShellDecode
pub(crate) const BRIDGE_INBOUND_PAGE_BYTES: usize = 16_384;
const BRIDGE_INBOUND_MAX_BYTES: usize = 1_048_576;
const BRIDGE_INBOUND_MAX_PAGES: usize = BRIDGE_INBOUND_MAX_BYTES.div_ceil(BRIDGE_INBOUND_PAGE_BYTES);
const BRIDGE_INBOUND_MAX_ITEMS: usize = 256;
const BRIDGE_INBOUND_MAX_RANGES: usize = 1_280;
const BRIDGE_INBOUND_MAX_FIELD_BYTES: usize = BRIDGE_INBOUND_MAX_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellFrameKind {
    Hello,
    ShellState,
    ShellStatePatch,
    Instances,
    AppFrames,
    ShellCommandResult,
    Approval,
    Ping,
    Bye,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellDecodeFault {
    Malformed,
    Capacity,
}

pub(crate) enum ShellDecodeStep {
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
    start: usize,
    len: usize,
    kind: ShellRangeKind,
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
    Finish,
    ValidateStrings { range: usize, offset: usize, utf8: Utf8Cursor },
    Copy { offset: usize },
}

#[derive(Clone, Copy)]
struct Utf8Cursor {
    needed: u8,
    min: u8,
    max: u8,
}

impl Utf8Cursor {
    fn new() -> Self {
        Self { needed: 0, min: 0x80, max: 0xbf }
    }

    fn push(&mut self, byte: u8) -> bool {
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

    fn complete(self) -> bool {
        self.needed == 0
    }
}

pub(crate) struct ShellToGatewayDecodeCursor {
    phase: ShellDecodePhase,
    payload_len: usize,
    cursor: usize,
    kind: Option<ShellFrameKind>,
    ranges: [Option<ShellRange>; BRIDGE_INBOUND_MAX_RANGES],
    range_count: usize,
    items: usize,
    owned_bytes: usize,
    output: Option<ValidatedShellFrame>,
}

impl ShellToGatewayDecodeCursor {
    pub(crate) fn new(payload_len: usize) -> Self {
        Self { phase: ShellDecodePhase::Tag, payload_len, cursor: 0, kind: None, ranges: std::array::from_fn(|_| None), range_count: 0, items: 0, owned_bytes: 0, output: None }
    }

    pub(crate) fn step<F>(&mut self, mut byte_at: F) -> ShellDecodeStep
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
            ShellDecodePhase::HelloSession => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::HelloPrincipal)),
            ShellDecodePhase::HelloPrincipal => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::HelloFlags)),
            ShellDecodePhase::HelloFlags => self.read_u8(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::Finish)),
            ShellDecodePhase::StateRevision => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::StateBytes)),
            ShellDecodePhase::StateBytes => self.read_range(&mut byte_at, ShellRangeKind::Bytes).and_then(|_| self.next(ShellDecodePhase::Finish)),
            ShellDecodePhase::PatchRevision => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::PatchBaseRevision)),
            ShellDecodePhase::PatchBaseRevision => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::PatchBytes)),
            ShellDecodePhase::PatchBytes => self.read_range(&mut byte_at, ShellRangeKind::Bytes).and_then(|_| self.next(ShellDecodePhase::Finish)),
            ShellDecodePhase::InstancesCount => self.read_count(&mut byte_at, 20).and_then(|count| self.next(if count == 0 { ShellDecodePhase::Finish } else { ShellDecodePhase::InstancePlugin { instances: count } })),
            ShellDecodePhase::InstancePlugin { instances } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::InstanceApp { instances })),
            ShellDecodePhase::InstanceApp { instances } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::InstanceId { instances })),
            ShellDecodePhase::InstanceId { instances } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::InstanceArtifact { instances })),
            ShellDecodePhase::InstanceArtifact { instances } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::InstanceWindowsCount { instances })),
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
            ShellDecodePhase::InstanceWindow { instances, windows } => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| {
                self.next(if windows > 1 {
                    ShellDecodePhase::InstanceWindow { instances, windows: windows - 1 }
                } else if instances > 1 {
                    ShellDecodePhase::InstancePlugin { instances: instances - 1 }
                } else {
                    ShellDecodePhase::Finish
                })
            }),
            ShellDecodePhase::AppReply => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::AppInstance)),
            ShellDecodePhase::AppInstance => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::AppFramesCount)),
            ShellDecodePhase::AppFramesCount => self.read_count(&mut byte_at, 4).and_then(|count| self.next(if count == 0 { ShellDecodePhase::Finish } else { ShellDecodePhase::AppFrame { frames: count } })),
            ShellDecodePhase::AppFrame { frames } => self.read_range(&mut byte_at, ShellRangeKind::Bytes).and_then(|_| self.next(if frames == 1 { ShellDecodePhase::Finish } else { ShellDecodePhase::AppFrame { frames: frames - 1 } })),
            ShellDecodePhase::CommandReply => self.read_u64(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::CommandOk)),
            ShellDecodePhase::CommandOk => self.read_bool(&mut byte_at).and_then(|_| self.next(ShellDecodePhase::CommandFaultFlag)),
            ShellDecodePhase::CommandFaultFlag => self.read_bool(&mut byte_at).and_then(|present| self.next(if present { ShellDecodePhase::CommandFaultString } else { ShellDecodePhase::Finish })),
            ShellDecodePhase::CommandFaultString => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::Finish)),
            ShellDecodePhase::ApprovalId => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::ApprovalDecision)),
            ShellDecodePhase::ApprovalDecision => self.read_u8(&mut byte_at).and_then(|decision| if decision <= 2 { self.next(ShellDecodePhase::ApprovalNoteFlag) } else { Err(ShellDecodeFault::Malformed) }),
            ShellDecodePhase::ApprovalNoteFlag => self.read_bool(&mut byte_at).and_then(|present| self.next(if present { ShellDecodePhase::ApprovalNoteString } else { ShellDecodePhase::Finish })),
            ShellDecodePhase::ApprovalNoteString => self.read_range(&mut byte_at, ShellRangeKind::String).and_then(|_| self.next(ShellDecodePhase::Finish)),
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

    fn read_range<F>(&mut self, byte_at: &mut F, kind: ShellRangeKind) -> Result<(), ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
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
        self.ranges[self.range_count] = Some(ShellRange { start: self.cursor, len, kind });
        self.range_count += 1;
        self.owned_bytes = owned_bytes;
        self.cursor = end;
        Ok(())
    }

    fn next(&mut self, phase: ShellDecodePhase) -> Result<(), ShellDecodeFault> {
        self.phase = phase;
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

    fn copy_page<F>(&mut self, offset: usize, byte_at: &mut F) -> ShellDecodeStep
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
    kind: ShellFrameKind,
    len: usize,
    pages: [Option<Box<[u8; BRIDGE_INBOUND_PAGE_BYTES]>>; BRIDGE_INBOUND_MAX_PAGES],
}

impl ValidatedShellFrame {
    fn new(kind: ShellFrameKind, len: usize) -> Self {
        Self { kind, len, pages: std::array::from_fn(|_| None) }
    }

    pub(crate) fn kind(&self) -> ShellFrameKind {
        self.kind
    }

    fn byte(&self, index: usize) -> Result<u8, ShellDecodeFault> {
        if index >= self.len {
            return Err(ShellDecodeFault::Malformed);
        }
        let page = self.pages[index / BRIDGE_INBOUND_PAGE_BYTES].as_ref().ok_or(ShellDecodeFault::Malformed)?;
        Ok(page[index % BRIDGE_INBOUND_PAGE_BYTES])
    }

    fn copy_into(&self, offset: usize, output: &mut [u8]) -> Result<(), ShellDecodeFault> {
        if offset.checked_add(output.len()).ok_or(ShellDecodeFault::Capacity)? > self.len {
            return Err(ShellDecodeFault::Malformed);
        }
        for (index, target) in output.iter_mut().enumerate() {
            *target = self.byte(offset + index)?;
        }
        Ok(())
    }
}

pub(crate) enum ShellMaterializeStep {
    Pending,
    Complete(ShellToGateway),
    Fault(ShellDecodeFault),
}

enum OwnedRange {
    Bytes { len: usize, copied: usize, value: Vec<u8> },
    String { end: usize, value: String },
}

impl OwnedRange {
    fn bytes(len: usize) -> Result<Self, ShellDecodeFault> {
        let mut value = Vec::new();
        value.try_reserve_exact(len).map_err(|_| ShellDecodeFault::Capacity)?;
        Ok(Self::Bytes { len, copied: 0, value })
    }

    fn string(start: usize, len: usize) -> Result<Self, ShellDecodeFault> {
        let mut value = String::new();
        value.try_reserve_exact(len).map_err(|_| ShellDecodeFault::Capacity)?;
        Ok(Self::String { end: start.checked_add(len).ok_or(ShellDecodeFault::Capacity)?, value })
    }

    fn step(&mut self, frame: &ValidatedShellFrame, position: &mut usize) -> Result<bool, ShellDecodeFault> {
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
    Finish,
}

pub(crate) struct ShellToGatewayMaterializeCursor {
    frame: ValidatedShellFrame,
    phase: ShellMaterializePhase,
    position: usize,
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
    entries: Vec<BridgeInstanceRef>,
    windows: Vec<String>,
    frames: Vec<Vec<u8>>,
}

impl ShellToGatewayMaterializeCursor {
    pub(crate) fn new(frame: ValidatedShellFrame) -> Self {
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

    pub(crate) fn kind(&self) -> ShellFrameKind {
        self.frame.kind()
    }

    pub(crate) fn step(&mut self) -> ShellMaterializeStep {
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

    fn finish(&mut self) -> Result<ShellToGateway, ShellDecodeFault> {
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
        })
    }
}
//#endregion 🔖️BoundedShellDecode

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
        }
        buf
    }

    fn copy_encoded_page(&self, offset: usize, output: &mut [u8]) -> usize {
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
            other => return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("bridge frame: unknown GatewayToShell tag {other}"))),
        };
        reader.finish()?;
        Ok(frame)
    }
}

struct BridgeEncodedPageWriter<'a> {
    offset: usize,
    position: usize,
    output: &'a mut [u8],
    written: usize,
}

impl<'a> BridgeEncodedPageWriter<'a> {
    fn new(offset: usize, output: &'a mut [u8]) -> Self {
        Self { offset, position: 0, output, written: 0 }
    }

    fn field(&mut self, bytes: &[u8]) {
        self.push(&(bytes.len() as u32).to_le_bytes());
        self.push(bytes);
    }

    fn push(&mut self, bytes: &[u8]) {
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

fn bridge_wire_field_len(bytes: usize) -> Option<usize> {
    u32::try_from(bytes).ok()?;
    4usize.checked_add(bytes)
}
//#endregion 🔖️GatewayToShell

//#region 🔖️BridgeHandle
/// 🆔️ One live `/bridge` connection's id — `Copy`/`Eq`/`Hash` so it keys a map and passes around
/// freely; `Display` is the same string the connection's own `Welcome.connection` field carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShellConnectionId(u64);

impl std::fmt::Display for ShellConnectionId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "conn_{}", self.0)
    }
}

struct ConnectionEntry {
    generation: u64,
    outbox: Arc<BridgeOutbox>,
    last_shell_state: Option<ShellToGateway>,
    last_instances: Option<Vec<BridgeInstanceRef>>,
    last_command_result: Option<(u64, bool, Option<String>)>,
    last_approval: Option<(String, ApprovalDecision, Option<String>)>,
}

const BRIDGE_OUTBOX_MAX_ITEMS: usize = 64;
const BRIDGE_OUTBOX_MAX_BYTES: usize = 1_048_576;
const BRIDGE_BROADCAST_MAX_RECIPIENTS: usize = 64;
const BRIDGE_BROADCAST_MAX_BYTES: usize = BRIDGE_OUTBOX_MAX_BYTES * BRIDGE_BROADCAST_MAX_RECIPIENTS;
pub(crate) const BRIDGE_OUTBOX_PAGE_BYTES: usize = 16_384;
const BRIDGE_OUTBOX_MAX_PAGES: usize = BRIDGE_OUTBOX_MAX_BYTES.div_ceil(BRIDGE_OUTBOX_PAGE_BYTES);
const BRIDGE_BROADCAST_MAX_PENDING: usize = 64;
const BRIDGE_RETIREMENT_MAX_PENDING: usize = 256;
const BRIDGE_ASYNC_RETRY_MS: u64 = 1;

#[derive(Clone, Copy)]
struct BridgeRetirementGrant {
    generation: u64,
}

struct BridgeRetirementCursor {
    pages: [Option<Box<[u8; BRIDGE_OUTBOX_PAGE_BYTES]>>; BRIDGE_OUTBOX_MAX_PAGES],
    page: usize,
}

enum BridgeRecipientState {
    Claimed { id: ShellConnectionId, outbox: Arc<BridgeOutbox>, grant: BridgeOutboxGrant },
    Published { id: ShellConnectionId },
    RecipientClosed { id: ShellConnectionId, grant: BridgeOutboxGrant },
}

struct BridgeBroadcastCursor {
    frame: Option<GatewayToShell>,
    expected: usize,
    offset: usize,
    encoded: BridgeEncodedFrame,
    shared: Option<Arc<BridgeEncodedFrame>>,
    recipients_state: [Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS],
    recipients: usize,
    recipient_cursor: usize,
    delivered: usize,
    recipient_closed: usize,
}

enum BridgeBroadcastStep {
    Pending(BridgeBroadcastCursor),
    Complete(BridgeBroadcastCompletion),
}

#[derive(Debug, PartialEq)]
pub enum BridgeBroadcastCompletion {
    Delivered { delivered: usize, recipient_closed: usize },
    Undelivered { frame: GatewayToShell, recipient_closed: usize },
}

impl BridgeBroadcastCursor {
    fn step(mut self) -> BridgeBroadcastStep {
        if self.offset != self.expected {
            let bytes = BRIDGE_OUTBOX_PAGE_BYTES.min(self.expected - self.offset);
            let mut page = Box::new([0; BRIDGE_OUTBOX_PAGE_BYTES]);
            let written = self.frame.as_ref().expect("bridge broadcast frame disappeared").copy_encoded_page(self.offset, &mut page[..bytes]);
            assert_eq!(written, bytes, "bridge broadcast page preflight changed");
            self.encoded.pages[self.offset / BRIDGE_OUTBOX_PAGE_BYTES] = Some(page);
            self.offset += bytes;
            return BridgeBroadcastStep::Pending(self);
        }
        if self.shared.is_none() {
            self.shared = Some(Arc::new(std::mem::replace(&mut self.encoded, BridgeEncodedFrame::empty(0, None))));
            return BridgeBroadcastStep::Pending(self);
        }
        if self.recipient_cursor < self.recipients {
            let index = self.recipient_cursor;
            let state = self.recipients_state[index].take().expect("bridge recipient state disappeared");
            match state {
                BridgeRecipientState::Claimed { id, outbox, grant } => match outbox.publish(grant, Arc::clone(self.shared.as_ref().expect("shared bridge frame disappeared"))) {
                    Ok(()) => {
                        self.recipients_state[index] = Some(BridgeRecipientState::Published { id });
                        self.delivered += 1;
                    }
                    Err(rejected) => {
                        self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant: rejected.grant });
                        self.recipient_closed += 1;
                        drop(rejected.encoded);
                    }
                },
                BridgeRecipientState::Published { id } => self.recipients_state[index] = Some(BridgeRecipientState::Published { id }),
                BridgeRecipientState::RecipientClosed { id, grant } => self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant }),
            }
            self.recipient_cursor += 1;
            return BridgeBroadcastStep::Pending(self);
        }
        let completion = if self.delivered == 0 {
            BridgeBroadcastCompletion::Undelivered { frame: self.frame.take().expect("undelivered bridge frame disappeared"), recipient_closed: self.recipient_closed }
        } else {
            self.frame.take();
            BridgeBroadcastCompletion::Delivered { delivered: self.delivered, recipient_closed: self.recipient_closed }
        };
        BridgeBroadcastStep::Complete(completion)
    }

    fn close_one_claim(mut self) -> BridgeBroadcastStep {
        while self.recipient_cursor < self.recipients {
            let index = self.recipient_cursor;
            self.recipient_cursor += 1;
            let state = self.recipients_state[index].take().expect("bridge recipient state disappeared");
            match state {
                BridgeRecipientState::Claimed { id, outbox, grant } => {
                    outbox.cancel(grant);
                    self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant });
                    self.recipient_closed += 1;
                    return BridgeBroadcastStep::Pending(self);
                }
                BridgeRecipientState::Published { id } => self.recipients_state[index] = Some(BridgeRecipientState::Published { id }),
                BridgeRecipientState::RecipientClosed { id, grant } => self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant }),
            }
        }
        let completion = if self.delivered == 0 {
            BridgeBroadcastCompletion::Undelivered { frame: self.frame.take().expect("terminal bridge frame disappeared"), recipient_closed: self.recipient_closed }
        } else {
            self.frame.take();
            BridgeBroadcastCompletion::Delivered { delivered: self.delivered, recipient_closed: self.recipient_closed }
        };
        BridgeBroadcastStep::Complete(completion)
    }
}

/// 🏗️ Heap-allocates `len` `None` slots without ever materializing them as one contiguous stack
/// value first — unlike `Box::new(std::array::from_fn(...))`, an unoptimized build gives no such
/// guarantee for a large fixed-size array literal, and `BridgeAsyncState`'s three ring buffers below
/// are hundreds of KiB combined (proven: `size_of::<BridgeAsyncState>()` measured 337,688 bytes, the
/// stack overflow this replaced only ever tripped on `BridgeHandle::new`, and vanished once these
/// fields moved off the stack).
fn boxed_slot_ring<T>(len: usize) -> Box<[Option<T>]> {
    (0..len).map(|_| None).collect()
}

struct BridgeAsyncState {
    broadcasts: Box<[Option<BridgeBroadcastCursor>]>,
    broadcast_head: usize,
    broadcast_len: usize,
    broadcast_reserved: usize,
    broadcast_driving: bool,
    completions: Box<[Option<BridgeBroadcastCompletion>]>,
    completion_head: usize,
    completion_len: usize,
    completion_reserved: usize,
    retirements: Box<[Option<BridgeRetirementCursor>]>,
    retirement_head: usize,
    retirement_len: usize,
    retirement_reserved: usize,
    retirement_generation: u64,
}

impl BridgeAsyncState {
    fn new() -> Self {
        Self {
            broadcasts: boxed_slot_ring(BRIDGE_BROADCAST_MAX_PENDING),
            broadcast_head: 0,
            broadcast_len: 0,
            broadcast_reserved: 0,
            broadcast_driving: false,
            completions: boxed_slot_ring(BRIDGE_BROADCAST_MAX_PENDING),
            completion_head: 0,
            completion_len: 0,
            completion_reserved: 0,
            retirements: boxed_slot_ring(BRIDGE_RETIREMENT_MAX_PENDING),
            retirement_head: 0,
            retirement_len: 0,
            retirement_reserved: 0,
            retirement_generation: 1,
        }
    }

    fn has_work(&self) -> bool {
        self.broadcast_len != 0 || self.retirement_len != 0
    }
}

struct BridgeAsyncAuthority {
    pool: WorkerPool,
    state: Mutex<BridgeAsyncState>,
    scheduled: AtomicBool,
    retry_armed: AtomicBool,
    retry_generation: AtomicU64,
    retry_job: Mutex<Option<Job>>,
    terminal_job: Mutex<Option<(WorkerSubmitErrorKind, Job)>>,
    terminal: AtomicBool,
}

impl BridgeAsyncAuthority {
    fn new(pool: WorkerPool) -> Arc<Self> {
        Arc::new(Self {
            pool,
            state: Mutex::new(BridgeAsyncState::new()),
            scheduled: AtomicBool::new(false),
            retry_armed: AtomicBool::new(false),
            retry_generation: AtomicU64::new(0),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            terminal: AtomicBool::new(false),
        })
    }

    fn reserve_broadcast(&self) -> Result<(), ()> {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if self.terminal.load(Ordering::Acquire) || state.broadcast_len.saturating_add(state.broadcast_reserved) == BRIDGE_BROADCAST_MAX_PENDING || state.completion_len.saturating_add(state.completion_reserved) == BRIDGE_BROADCAST_MAX_PENDING {
            return Err(());
        }
        state.broadcast_reserved += 1;
        state.completion_reserved += 1;
        Ok(())
    }

    fn cancel_broadcast_reservation(&self) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        state.broadcast_reserved = state.broadcast_reserved.saturating_sub(1);
        state.completion_reserved = state.completion_reserved.saturating_sub(1);
    }

    fn reserve_retirement(&self) -> Result<BridgeRetirementGrant, ()> {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if self.terminal.load(Ordering::Acquire) || state.retirement_len.saturating_add(state.retirement_reserved) == BRIDGE_RETIREMENT_MAX_PENDING {
            return Err(());
        }
        state.retirement_reserved += 1;
        Ok(BridgeRetirementGrant { generation: state.retirement_generation })
    }

    fn cancel_retirement(&self, grant: BridgeRetirementGrant) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if grant.generation == state.retirement_generation {
            state.retirement_reserved = state.retirement_reserved.saturating_sub(1);
        }
    }

    fn enqueue_broadcast(self: &Arc<Self>, cursor: BridgeBroadcastCursor) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        state.broadcast_reserved = state.broadcast_reserved.saturating_sub(1);
        let index = (state.broadcast_head + state.broadcast_len) % BRIDGE_BROADCAST_MAX_PENDING;
        state.broadcasts[index] = Some(cursor);
        state.broadcast_len += 1;
        drop(state);
        self.request_schedule();
    }

    fn retain_completion(state: &mut BridgeAsyncState, completion: BridgeBroadcastCompletion) {
        state.completion_reserved = state.completion_reserved.saturating_sub(1);
        let index = (state.completion_head + state.completion_len) % BRIDGE_BROADCAST_MAX_PENDING;
        state.completions[index] = Some(completion);
        state.completion_len += 1;
    }

    fn publish_retirement(self: &Arc<Self>, grant: BridgeRetirementGrant, pages: [Option<Box<[u8; BRIDGE_OUTBOX_PAGE_BYTES]>>; BRIDGE_OUTBOX_MAX_PAGES]) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if grant.generation != state.retirement_generation || state.retirement_reserved == 0 {
            drop(state);
            drop(pages);
            return;
        }
        state.retirement_reserved -= 1;
        let index = (state.retirement_head + state.retirement_len) % BRIDGE_RETIREMENT_MAX_PENDING;
        state.retirements[index] = Some(BridgeRetirementCursor { pages, page: 0 });
        state.retirement_len += 1;
        drop(state);
        self.request_schedule();
    }

    fn request_schedule(self: &Arc<Self>) {
        if self.terminal.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let authority = Arc::clone(self);
        self.submit_exact(Box::new(move || authority.drive_one()));
    }

    fn submit_exact(self: &Arc<Self>, job: Job) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated => {
                    *self.retry_job.lock().expect("bridge retry lock poisoned") = Some(error.into_job());
                    self.arm_retry();
                }
                kind @ (WorkerSubmitErrorKind::Shutdown | WorkerSubmitErrorKind::Poisoned) => {
                    self.terminal.store(true, Ordering::Release);
                    self.retry_generation.fetch_add(1, Ordering::AcqRel);
                    *self.terminal_job.lock().expect("bridge terminal lock poisoned") = Some((kind, error.into_job()));
                }
            },
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = self.retry_generation.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        let authority = Arc::clone(self);
        self.pool.callback_at(self.pool.now_ms().saturating_add(BRIDGE_ASYNC_RETRY_MS), move || {
            if generation != authority.retry_generation.load(Ordering::Acquire) {
                return;
            }
            authority.retry_armed.store(false, Ordering::Release);
            let job = authority.retry_job.lock().expect("bridge retry lock poisoned").take();
            if let Some(job) = job {
                authority.submit_exact(job);
            }
        });
    }

    fn drive_one(self: Arc<Self>) {
        if self.terminal.load(Ordering::Acquire) {
            self.scheduled.store(false, Ordering::Release);
            return;
        }
        let broadcast = {
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            if state.broadcast_len == 0 || state.broadcast_driving {
                None
            } else {
                let head = state.broadcast_head;
                state.broadcast_driving = true;
                state.broadcasts[head].take()
            }
        };
        if let Some(cursor) = broadcast {
            let step = cursor.step();
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            state.broadcast_driving = false;
            let head = state.broadcast_head;
            match step {
                BridgeBroadcastStep::Pending(cursor) => state.broadcasts[head] = Some(cursor),
                BridgeBroadcastStep::Complete(completion) => {
                    state.broadcast_head = (head + 1) % BRIDGE_BROADCAST_MAX_PENDING;
                    state.broadcast_len -= 1;
                    Self::retain_completion(&mut state, completion);
                }
            }
        } else {
            let page = {
                let mut state = self.state.lock().expect("bridge async lock poisoned");
                if state.retirement_len == 0 {
                    None
                } else {
                    let head = state.retirement_head;
                    let cursor = state.retirements[head].as_mut().expect("bridge retirement cursor disappeared");
                    while cursor.page < BRIDGE_OUTBOX_MAX_PAGES && cursor.pages[cursor.page].is_none() {
                        cursor.page += 1;
                    }
                    if cursor.page == BRIDGE_OUTBOX_MAX_PAGES {
                        state.retirements[head] = None;
                        state.retirement_head = (head + 1) % BRIDGE_RETIREMENT_MAX_PENDING;
                        state.retirement_len -= 1;
                        None
                    } else {
                        let page = cursor.pages[cursor.page].take();
                        cursor.page += 1;
                        page
                    }
                }
            };
            drop(page);
        }
        self.scheduled.store(false, Ordering::Release);
        if self.state.lock().expect("bridge async lock poisoned").has_work() {
            self.request_schedule();
        }
    }

    fn take_terminal_job(&self) -> Option<(WorkerSubmitErrorKind, Job)> {
        self.terminal_job.lock().expect("bridge terminal lock poisoned").take()
    }

    fn cancel(&self) {
        self.terminal.store(true, Ordering::Release);
        self.retry_generation.fetch_add(1, Ordering::AcqRel);
    }

    fn take_broadcast_completion(&self) -> Option<BridgeBroadcastCompletion> {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if state.completion_len == 0 {
            return None;
        }
        let head = state.completion_head;
        let completion = state.completions[head].take();
        state.completion_head = (head + 1) % BRIDGE_BROADCAST_MAX_PENDING;
        state.completion_len -= 1;
        completion
    }

    fn close_one_terminal_broadcast_claim(&self) -> bool {
        if !self.terminal.load(Ordering::Acquire) {
            return false;
        }
        let cursor = {
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            if state.broadcast_len == 0 || state.broadcast_driving {
                return false;
            }
            let head = state.broadcast_head;
            state.broadcast_driving = true;
            state.broadcasts[head].take().expect("terminal bridge broadcast cursor disappeared")
        };
        let step = cursor.close_one_claim();
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        state.broadcast_driving = false;
        let head = state.broadcast_head;
        match step {
            BridgeBroadcastStep::Pending(cursor) => state.broadcasts[head] = Some(cursor),
            BridgeBroadcastStep::Complete(completion) => {
                state.broadcast_head = (head + 1) % BRIDGE_BROADCAST_MAX_PENDING;
                state.broadcast_len -= 1;
                Self::retain_completion(&mut state, completion);
            }
        }
        true
    }

    fn close_one_terminal_retired_page(&self) -> bool {
        if !self.terminal.load(Ordering::Acquire) {
            return false;
        }
        let page = {
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            if state.retirement_len == 0 {
                return false;
            }
            let head = state.retirement_head;
            let cursor = state.retirements[head].as_mut().expect("bridge retirement cursor disappeared");
            while cursor.page < BRIDGE_OUTBOX_MAX_PAGES && cursor.pages[cursor.page].is_none() {
                cursor.page += 1;
            }
            let page = if cursor.page < BRIDGE_OUTBOX_MAX_PAGES {
                let page = cursor.pages[cursor.page].take();
                cursor.page += 1;
                page
            } else {
                None
            };
            if cursor.page == BRIDGE_OUTBOX_MAX_PAGES || cursor.pages[cursor.page..].iter().all(Option::is_none) {
                state.retirements[head] = None;
                state.retirement_head = (head + 1) % BRIDGE_RETIREMENT_MAX_PENDING;
                state.retirement_len -= 1;
            }
            page
        };
        drop(page);
        true
    }
}

pub(crate) struct BridgeEncodedFrame {
    pages: [Option<Box<[u8; BRIDGE_OUTBOX_PAGE_BYTES]>>; BRIDGE_OUTBOX_MAX_PAGES],
    len: usize,
    retirement: Option<(Arc<BridgeAsyncAuthority>, BridgeRetirementGrant)>,
}

impl BridgeEncodedFrame {
    fn empty(len: usize, retirement: Option<(Arc<BridgeAsyncAuthority>, BridgeRetirementGrant)>) -> Self {
        Self { pages: std::array::from_fn(|_| None), len, retirement }
    }

    fn encode(frame: &GatewayToShell, expected: usize) -> Self {
        let mut encoded = Self::empty(0, None);
        match frame {
            GatewayToShell::Welcome { bridge_version, connection, principal } => {
                encoded.write_u8(0);
                encoded.write_bytes(&bridge_version.to_le_bytes());
                encoded.write_field(connection.as_bytes());
                encoded.write_field(principal.as_bytes());
            }
            GatewayToShell::ShellCommand { seq, command } => {
                encoded.write_u8(1);
                encoded.write_bytes(&seq.to_le_bytes());
                encoded.write_field(command);
            }
            GatewayToShell::AppCommand { seq, instance_id, command } => {
                encoded.write_u8(2);
                encoded.write_bytes(&seq.to_le_bytes());
                encoded.write_field(instance_id.as_bytes());
                encoded.write_field(command);
            }
            GatewayToShell::ApprovalRequested { approval_id, summary } => {
                encoded.write_u8(3);
                encoded.write_field(approval_id.as_bytes());
                encoded.write_field(summary.as_bytes());
            }
            GatewayToShell::ApprovalResolved { approval_id, decision } => {
                encoded.write_u8(4);
                encoded.write_field(approval_id.as_bytes());
                encoded.write_u8(decision.to_tag());
            }
            GatewayToShell::AgentPresence { active, label, invocation_id } => {
                encoded.write_u8(5);
                encoded.write_u8(*active as u8);
                encoded.write_field(label.as_bytes());
                encoded.write_u8(invocation_id.is_some() as u8);
                if let Some(invocation_id) = invocation_id {
                    encoded.write_field(invocation_id.as_bytes());
                }
            }
            GatewayToShell::Pong => encoded.write_u8(6),
            GatewayToShell::Bye { reason } => {
                encoded.write_u8(7);
                encoded.write_field(reason.as_bytes());
            }
        }
        assert_eq!(encoded.len, expected, "preflighted bridge frame length changed during encode");
        encoded
    }

    fn write_u8(&mut self, value: u8) {
        self.write_bytes(&[value]);
    }

    fn write_field(&mut self, value: &[u8]) {
        self.write_bytes(&(value.len() as u32).to_le_bytes());
        self.write_bytes(value);
    }

    fn write_bytes(&mut self, mut value: &[u8]) {
        while !value.is_empty() {
            let page_index = self.len / BRIDGE_OUTBOX_PAGE_BYTES;
            let page_offset = self.len % BRIDGE_OUTBOX_PAGE_BYTES;
            if self.pages[page_index].is_none() {
                self.pages[page_index] = Some(Box::new([0; BRIDGE_OUTBOX_PAGE_BYTES]));
            }
            let bytes = value.len().min(BRIDGE_OUTBOX_PAGE_BYTES - page_offset);
            self.pages[page_index].as_mut().expect("bridge encode page disappeared")[page_offset..page_offset + bytes].copy_from_slice(&value[..bytes]);
            self.len += bytes;
            value = &value[bytes..];
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn copy_into(&self, offset: usize, output: &mut [u8]) -> usize {
        if offset >= self.len {
            return 0;
        }
        let mut source = offset;
        let mut written = 0;
        while written < output.len() && source < self.len {
            let page_index = source / BRIDGE_OUTBOX_PAGE_BYTES;
            let page_offset = source % BRIDGE_OUTBOX_PAGE_BYTES;
            let bytes = (output.len() - written).min(BRIDGE_OUTBOX_PAGE_BYTES - page_offset).min(self.len - source);
            output[written..written + bytes].copy_from_slice(&self.pages[page_index].as_ref().expect("bridge encoded page disappeared")[page_offset..page_offset + bytes]);
            written += bytes;
            source += bytes;
        }
        written
    }

    #[cfg(test)]
    fn page_count(&self) -> usize {
        self.pages.iter().filter(|page| page.is_some()).count()
    }

    #[cfg(test)]
    fn decode(&self) -> GatewayToShell {
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(self.len).expect("bridge test decode reservation failed");
        for offset in (0..self.len).step_by(BRIDGE_OUTBOX_PAGE_BYTES) {
            let bytes_in_page = BRIDGE_OUTBOX_PAGE_BYTES.min(self.len - offset);
            let start = bytes.len();
            bytes.resize(start + bytes_in_page, 0);
            assert_eq!(self.copy_into(offset, &mut bytes[start..]), bytes_in_page);
        }
        GatewayToShell::decode(&bytes).expect("bridge fixed-page encode did not decode")
    }
}

impl Drop for BridgeEncodedFrame {
    fn drop(&mut self) {
        let Some((authority, grant)) = self.retirement.take() else { return };
        let mut pages = std::array::from_fn(|_| None);
        for (target, source) in pages.iter_mut().zip(self.pages.iter_mut()) {
            *target = source.take();
        }
        authority.publish_retirement(grant, pages);
    }
}

#[derive(Clone)]
pub(crate) struct BridgeEncodedLease {
    generation: u64,
    encoded: Arc<BridgeEncodedFrame>,
}

impl BridgeEncodedLease {
    pub(crate) fn len(&self) -> usize {
        self.encoded.len()
    }

    pub(crate) fn copy_into(&self, offset: usize, output: &mut [u8]) -> usize {
        self.encoded.copy_into(offset, output)
    }
}

struct BridgeOutboxItem {
    lease: BridgeEncodedLease,
    bytes: usize,
}

#[derive(Clone, Copy)]
struct BridgeOutboxGrant {
    generation: u64,
    bytes: usize,
}

struct BridgeRejectedPublish {
    grant: BridgeOutboxGrant,
    encoded: Arc<BridgeEncodedFrame>,
}

struct BridgeOutboxState {
    slots: [Option<BridgeOutboxItem>; BRIDGE_OUTBOX_MAX_ITEMS],
    head: usize,
    len: usize,
    bytes: usize,
    generation: u64,
    reserved_items: usize,
    reserved_bytes: usize,
    closed: bool,
    #[cfg(test)]
    encode_count: usize,
    #[cfg(test)]
    waker: Option<std::task::Waker>,
}

impl BridgeOutboxState {
    fn new(generation: u64) -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
            bytes: 0,
            generation,
            reserved_items: 0,
            reserved_bytes: 0,
            closed: false,
            #[cfg(test)]
            encode_count: 0,
            #[cfg(test)]
            waker: None,
        }
    }

    fn claim(&mut self, generation: u64, bytes: usize) -> Result<BridgeOutboxGrant, ()> {
        if self.closed || generation != self.generation || self.len.saturating_add(self.reserved_items) == BRIDGE_OUTBOX_MAX_ITEMS || bytes > BRIDGE_OUTBOX_MAX_BYTES.saturating_sub(self.bytes).saturating_sub(self.reserved_bytes) {
            return Err(());
        }
        self.reserved_items += 1;
        self.reserved_bytes += bytes;
        Ok(BridgeOutboxGrant { generation, bytes })
    }

    fn cancel(&mut self, grant: BridgeOutboxGrant) {
        if grant.generation == self.generation {
            self.reserved_items = self.reserved_items.saturating_sub(1);
            self.reserved_bytes = self.reserved_bytes.saturating_sub(grant.bytes);
        }
    }

    fn publish(&mut self, grant: BridgeOutboxGrant, encoded: Arc<BridgeEncodedFrame>) -> Result<(), BridgeRejectedPublish> {
        if self.closed || grant.generation != self.generation || self.reserved_items == 0 || grant.bytes > self.reserved_bytes {
            return Err(BridgeRejectedPublish { grant, encoded });
        }
        self.reserved_items -= 1;
        self.reserved_bytes -= grant.bytes;
        let index = (self.head + self.len) % BRIDGE_OUTBOX_MAX_ITEMS;
        self.len += 1;
        self.bytes += grant.bytes;
        #[cfg(test)]
        {
            self.encode_count += 1;
        }
        self.slots[index] = Some(BridgeOutboxItem { lease: BridgeEncodedLease { generation: grant.generation, encoded }, bytes: grant.bytes });
        #[cfg(test)]
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
        Ok(())
    }

    fn pop_front(&mut self) -> Option<BridgeOutboxItem> {
        if self.len == 0 {
            return None;
        }
        let item = self.slots[self.head].take().expect("bridge outbox FIFO slot disappeared");
        self.head = (self.head + 1) % BRIDGE_OUTBOX_MAX_ITEMS;
        self.len -= 1;
        self.bytes = self.bytes.saturating_sub(item.bytes);
        Some(item)
    }
}

struct BridgeOutbox {
    state: Mutex<BridgeOutboxState>,
}

impl BridgeOutbox {
    fn new(generation: u64) -> Self {
        Self { state: Mutex::new(BridgeOutboxState::new(generation)) }
    }

    fn try_send(&self, frame: GatewayToShell) -> Result<(), GatewayToShell> {
        let Some(bytes) = frame.encoded_len() else { return Err(frame) };
        let grant = match self.claim(bytes) {
            Ok(grant) => grant,
            Err(()) => return Err(frame),
        };
        let encoded = Arc::new(BridgeEncodedFrame::encode(&frame, bytes));
        if self.publish(grant, encoded).is_err() {
            return Err(frame);
        }
        Ok(())
    }

    fn claim(&self, bytes: usize) -> Result<BridgeOutboxGrant, ()> {
        let mut state = self.state.lock().expect("bridge outbox lock poisoned");
        let generation = state.generation;
        state.claim(generation, bytes)
    }

    fn publish(&self, grant: BridgeOutboxGrant, encoded: Arc<BridgeEncodedFrame>) -> Result<(), BridgeRejectedPublish> {
        self.state.lock().expect("bridge outbox lock poisoned").publish(grant, encoded)
    }

    fn cancel(&self, grant: BridgeOutboxGrant) {
        self.state.lock().expect("bridge outbox lock poisoned").cancel(grant);
    }

    fn try_recv(&self) -> Option<BridgeOutboxItem> {
        let mut state = self.state.lock().expect("bridge outbox lock poisoned");
        let item = state.pop_front()?;
        if item.lease.generation != state.generation {
            return None;
        }
        Some(item)
    }

    fn close(&self) {
        let mut state = self.state.lock().expect("bridge outbox lock poisoned");
        state.closed = true;
        state.generation = state.generation.wrapping_add(1);
        state.reserved_items = 0;
        state.reserved_bytes = 0;
        #[cfg(test)]
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

pub(crate) struct BridgeOutboxReceiver(Arc<BridgeOutbox>);

impl BridgeOutboxReceiver {
    #[cfg(test)]
    pub(crate) async fn recv(&mut self) -> Option<GatewayToShell> {
        std::future::poll_fn(|context| {
            let mut state = self.0.state.lock().expect("bridge outbox lock poisoned");
            if let Some(item) = state.pop_front() {
                if item.lease.generation == state.generation {
                    return std::task::Poll::Ready(Some(item.lease.encoded.decode()));
                }
                return std::task::Poll::Ready(None);
            }
            if state.closed {
                return std::task::Poll::Ready(None);
            }
            state.waker = Some(context.waker().clone());
            std::task::Poll::Pending
        })
        .await
    }

    #[cfg(test)]
    pub(crate) fn try_recv(&mut self) -> Option<GatewayToShell> {
        self.0.try_recv().map(|item| item.lease.encoded.decode())
    }

    pub(crate) fn try_recv_encoded(&mut self) -> Option<BridgeEncodedLease> {
        self.0.try_recv().map(|item| item.lease)
    }
}

struct BridgeInner {
    next_id: AtomicU64,
    connections: Mutex<HashMap<ShellConnectionId, ConnectionEntry>>,
    asynchronous: Arc<BridgeAsyncAuthority>,
}

/// 🖇️ The seam a later packet reaches live `/bridge` connections through WITHOUT this facet
/// depending on theirs — P6's policy engine routes a parked approval to a connected shell via
/// [`send_to`](Self::send_to)/[`broadcast`](Self::broadcast); a future `ui.*` tool pushes a
/// `ShellCommand` the same way; `semio://ui/shell`/`context_resolve` read
/// [`last_shell_state`](Self::last_shell_state). None of that wiring happens in THIS file
/// (`📓️sol-P1c-packet.md` §3: "do not wire it into P6's files … just publish the API") — obtain a
/// `BridgeHandle` from the live [`crate::HttpTransportRun::bridge`] owner; the Axum router is a
/// test-only differential oracle.
#[derive(Clone)]
pub struct BridgeHandle {
    inner: Arc<BridgeInner>,
}

impl Default for BridgeHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl BridgeHandle {
    pub fn new() -> Self {
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        Self::with_pool(semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores)))
    }

    pub(crate) fn with_pool(pool: WorkerPool) -> Self {
        Self { inner: Arc::new(BridgeInner { next_id: AtomicU64::new(0), connections: Mutex::new(HashMap::new()), asynchronous: BridgeAsyncAuthority::new(pool) }) }
    }

    pub(crate) fn register(&self) -> (ShellConnectionId, BridgeOutboxReceiver) {
        let id = ShellConnectionId(self.inner.next_id.fetch_add(1, Ordering::Relaxed));
        let outbox = Arc::new(BridgeOutbox::new(id.0));
        self.inner
            .connections
            .lock()
            .expect("bridge connections lock poisoned")
            .insert(id, ConnectionEntry { generation: id.0, outbox: Arc::clone(&outbox), last_shell_state: None, last_instances: None, last_command_result: None, last_approval: None });
        (id, BridgeOutboxReceiver(outbox))
    }

    pub(crate) fn unregister(&self, id: ShellConnectionId) {
        if let Some(entry) = self.inner.connections.lock().expect("bridge connections lock poisoned").remove(&id) {
            entry.outbox.close();
        }
    }

    /// 📝️ Records the effect of one received [`ShellToGateway`] frame against its connection —
    /// `Hello`/`Ping`/`Bye` never reach here (the read loop handles all three inline).
    pub(crate) fn record(&self, id: ShellConnectionId, frame: ShellToGateway) {
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
        self.try_send_to(id, frame).is_ok()
    }

    /// 📦️ Fixed-credit bridge admission with exact rejected-frame handback.
    pub fn try_send_to(&self, id: ShellConnectionId, frame: GatewayToShell) -> Result<(), GatewayToShell> {
        let connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        match connections.get(&id) {
            Some(entry) => try_send_bridge_frame(entry, frame),
            None => Err(frame),
        }
    }

    /// 📢️ Atomically admits one shared encoded frame to every live connection.
    pub fn broadcast(&self, frame: GatewayToShell) -> Result<usize, GatewayToShell> {
        let Some(bytes) = frame.encoded_len() else { return Err(frame) };
        let connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        let recipients = connections.len();
        if recipients > BRIDGE_BROADCAST_MAX_RECIPIENTS || bytes.checked_mul(recipients).map_or(true, |total| total > BRIDGE_BROADCAST_MAX_BYTES) {
            return Err(frame);
        }
        if recipients == 0 {
            return Ok(0);
        }
        if self.inner.asynchronous.reserve_broadcast().is_err() {
            return Err(frame);
        }
        let retirement = match self.inner.asynchronous.reserve_retirement() {
            Ok(grant) => grant,
            Err(()) => {
                self.inner.asynchronous.cancel_broadcast_reservation();
                return Err(frame);
            }
        };
        let mut recipient_ids: [Option<ShellConnectionId>; BRIDGE_BROADCAST_MAX_RECIPIENTS] = [None; BRIDGE_BROADCAST_MAX_RECIPIENTS];
        for (index, id) in connections.keys().copied().enumerate() {
            recipient_ids[index] = Some(id);
        }
        recipient_ids[..recipients].sort_unstable();
        let mut recipients_state: [Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS] = std::array::from_fn(|_| None);
        let mut admitted = 0;
        for id in recipient_ids[..recipients].iter().flatten().copied() {
            let entry = connections.get(&id).expect("stable bridge recipient disappeared under admission lock");
            let grant = match entry.outbox.claim(bytes) {
                Ok(grant) if grant.generation == entry.generation => grant,
                Ok(grant) => {
                    entry.outbox.cancel(grant);
                    for state in recipients_state[..admitted].iter_mut().filter_map(Option::take) {
                        if let BridgeRecipientState::Claimed { outbox, grant, .. } = state {
                            outbox.cancel(grant);
                        }
                    }
                    self.inner.asynchronous.cancel_retirement(retirement);
                    self.inner.asynchronous.cancel_broadcast_reservation();
                    return Err(frame);
                }
                Err(()) => {
                    for state in recipients_state[..admitted].iter_mut().filter_map(Option::take) {
                        if let BridgeRecipientState::Claimed { outbox, grant, .. } = state {
                            outbox.cancel(grant);
                        }
                    }
                    self.inner.asynchronous.cancel_retirement(retirement);
                    self.inner.asynchronous.cancel_broadcast_reservation();
                    return Err(frame);
                }
            };
            recipients_state[admitted] = Some(BridgeRecipientState::Claimed { id, outbox: Arc::clone(&entry.outbox), grant });
            admitted += 1;
        }
        let cursor = BridgeBroadcastCursor {
            frame: Some(frame),
            expected: bytes,
            offset: 0,
            encoded: BridgeEncodedFrame::empty(bytes, Some((Arc::clone(&self.inner.asynchronous), retirement))),
            shared: None,
            recipients_state,
            recipients,
            recipient_cursor: 0,
            delivered: 0,
            recipient_closed: 0,
        };
        drop(connections);
        self.inner.asynchronous.enqueue_broadcast(cursor);
        Ok(recipients)
    }

    pub fn take_broadcast_completion(&self) -> Option<BridgeBroadcastCompletion> {
        self.inner.asynchronous.take_broadcast_completion()
    }

    pub fn close_one_terminal_broadcast_claim(&self) -> bool {
        self.inner.asynchronous.close_one_terminal_broadcast_claim()
    }

    pub fn cancel_broadcasts(&self) {
        self.inner.asynchronous.cancel();
    }

    pub(crate) fn take_terminal_broadcast_job(&self) -> Option<(WorkerSubmitErrorKind, Job)> {
        self.inner.asynchronous.take_terminal_job()
    }

    pub fn close_one_terminal_retired_page(&self) -> bool {
        self.inner.asynchronous.close_one_terminal_retired_page()
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

fn try_send_bridge_frame(entry: &ConnectionEntry, frame: GatewayToShell) -> Result<(), GatewayToShell> {
    entry.outbox.try_send(frame)
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
#[cfg(test)]
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

    fn bounded_shell_decode(bytes: &[u8]) -> Result<ShellToGateway, ShellDecodeFault> {
        let mut decoder = ShellToGatewayDecodeCursor::new(bytes.len());
        let validated = loop {
            match decoder.step(|index| bytes.get(index).copied()) {
                ShellDecodeStep::Pending => {}
                ShellDecodeStep::Complete(frame) => break frame,
                ShellDecodeStep::Fault(fault) => return Err(fault),
            }
        };
        let mut materializer = ShellToGatewayMaterializeCursor::new(validated);
        loop {
            match materializer.step() {
                ShellMaterializeStep::Pending => {}
                ShellMaterializeStep::Complete(frame) => return Ok(frame),
                ShellMaterializeStep::Fault(fault) => return Err(fault),
            }
        }
    }

    fn retained_broadcast_cursor(handle: &BridgeHandle, ids: &[ShellConnectionId], frame: GatewayToShell) -> BridgeBroadcastCursor {
        let expected = frame.encoded_len().unwrap();
        handle.inner.asynchronous.reserve_broadcast().unwrap();
        let retirement = handle.inner.asynchronous.reserve_retirement().unwrap();
        let connections = handle.inner.connections.lock().unwrap();
        let mut recipients_state = std::array::from_fn(|_| None);
        for (index, id) in ids.iter().copied().enumerate() {
            let outbox = Arc::clone(&connections.get(&id).unwrap().outbox);
            let grant = outbox.claim(expected).unwrap();
            recipients_state[index] = Some(BridgeRecipientState::Claimed { id, outbox, grant });
        }
        BridgeBroadcastCursor {
            frame: Some(frame),
            expected,
            offset: 0,
            encoded: BridgeEncodedFrame::empty(expected, Some((Arc::clone(&handle.inner.asynchronous), retirement))),
            shared: None,
            recipients_state,
            recipients: ids.len(),
            recipient_cursor: 0,
            delivered: 0,
            recipient_closed: 0,
        }
    }

    fn pending_broadcast(step: BridgeBroadcastStep) -> BridgeBroadcastCursor {
        match step {
            BridgeBroadcastStep::Pending(cursor) => cursor,
            BridgeBroadcastStep::Complete(_) => panic!("broadcast completed before requested fixture boundary"),
        }
    }

    fn encoded_broadcast(mut cursor: BridgeBroadcastCursor) -> BridgeBroadcastCursor {
        while cursor.offset != cursor.expected || cursor.shared.is_none() {
            cursor = pending_broadcast(cursor.step());
        }
        cursor
    }

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

    #[test]
    fn bounded_shell_decoder_rejects_ffffffff_counts_and_truncated_ranges_before_owner_allocation() {
        let mut instances = vec![3];
        instances.extend_from_slice(&u32::MAX.to_le_bytes());
        assert!(matches!(bounded_shell_decode(&instances), Err(ShellDecodeFault::Capacity)));

        let mut windows = vec![3];
        windows.extend_from_slice(&1u32.to_le_bytes());
        for _ in 0..4 {
            windows.extend_from_slice(&0u32.to_le_bytes());
        }
        windows.extend_from_slice(&u32::MAX.to_le_bytes());
        assert!(matches!(bounded_shell_decode(&windows), Err(ShellDecodeFault::Capacity)));

        let mut frames = vec![4];
        frames.extend_from_slice(&7u64.to_le_bytes());
        frames.extend_from_slice(&0u32.to_le_bytes());
        frames.extend_from_slice(&u32::MAX.to_le_bytes());
        assert!(matches!(bounded_shell_decode(&frames), Err(ShellDecodeFault::Capacity)));

        assert!(matches!(bounded_shell_decode(&[3, 1, 0]), Err(ShellDecodeFault::Malformed)));
        let mut truncated_range = vec![1];
        truncated_range.extend_from_slice(&1u64.to_le_bytes());
        truncated_range.extend_from_slice(&4u32.to_le_bytes());
        truncated_range.extend_from_slice(&[1, 2, 3]);
        assert!(matches!(bounded_shell_decode(&truncated_range), Err(ShellDecodeFault::Malformed)));
    }

    #[test]
    fn bounded_shell_decoder_cap_plus_one_and_every_variant_match_the_canonical_fixture() {
        let mut cap_plus_one = vec![1];
        cap_plus_one.extend_from_slice(&1u64.to_le_bytes());
        cap_plus_one.extend_from_slice(&((BRIDGE_INBOUND_MAX_FIELD_BYTES + 1) as u32).to_le_bytes());
        assert!(matches!(bounded_shell_decode(&cap_plus_one), Err(ShellDecodeFault::Capacity)));
        for frame in sample_shell_frames() {
            assert_eq!(bounded_shell_decode(&frame.encode()), Ok(frame));
        }
    }

    #[test]
    fn bounded_shell_decoder_and_materializer_advance_incrementally() {
        let bytes = ShellToGateway::ShellState { revision: 9, state: vec![7; BRIDGE_INBOUND_PAGE_BYTES + 1] }.encode();
        let mut decoder = ShellToGatewayDecodeCursor::new(bytes.len());
        assert!(matches!(decoder.step(|index| bytes.get(index).copied()), ShellDecodeStep::Pending));
        assert!(matches!(decoder.step(|index| bytes.get(index).copied()), ShellDecodeStep::Pending));
        assert!(matches!(decoder.step(|index| bytes.get(index).copied()), ShellDecodeStep::Pending));
        assert_eq!(decoder.cursor, 9, "one preflight grant may consume only one scalar token");
    }

    #[test]
    fn bridge_outbox_item_cap_plus_one_returns_the_exact_frame_and_rearms_after_one_receive() {
        let handle = BridgeHandle::new();
        let (id, mut outbox) = handle.register();
        for _ in 0..BRIDGE_OUTBOX_MAX_ITEMS {
            assert!(handle.try_send_to(id, GatewayToShell::Pong).is_ok());
        }
        let rejected = GatewayToShell::Bye { reason: "cap-plus-one".into() };
        assert_eq!(handle.try_send_to(id, rejected.clone()), Err(rejected));
        assert_eq!(outbox.try_recv(), Some(GatewayToShell::Pong));
        assert!(handle.try_send_to(id, GatewayToShell::Pong).is_ok());
    }

    #[test]
    fn bridge_outbox_byte_cap_plus_one_returns_the_exact_frame_before_queue_mutation() {
        let outbox = BridgeOutbox::new(7);
        let accepted = GatewayToShell::ShellCommand { seq: 7, command: vec![9; BRIDGE_OUTBOX_MAX_BYTES - 13] };
        assert_eq!(accepted.encoded_len(), Some(BRIDGE_OUTBOX_MAX_BYTES));
        assert_eq!(outbox.try_send(accepted), Ok(()));
        let encoded = outbox.try_recv().unwrap().lease.encoded;
        assert_eq!(encoded.len(), BRIDGE_OUTBOX_MAX_BYTES);
        assert_eq!(encoded.page_count(), BRIDGE_OUTBOX_MAX_PAGES);

        let rejected = GatewayToShell::ShellCommand { seq: 8, command: vec![7; BRIDGE_OUTBOX_MAX_BYTES - 12] };
        let rejected_copy = rejected.clone();
        let encodes_before = outbox.state.lock().unwrap().encode_count;
        assert_eq!(outbox.try_send(rejected), Err(rejected_copy));
        let state = outbox.state.lock().unwrap();
        assert_eq!(state.encode_count, encodes_before, "cap+1 must reject before page allocation/encode");
        assert_eq!(state.len, 0);
        assert_eq!(state.bytes, 0);
        assert!(bridge_wire_field_len(usize::MAX).is_none(), "checked wire-size overflow must fail before admission");
    }

    #[test]
    fn bridge_outbox_page_boundary_matches_the_canonical_encoder() {
        for reason_bytes in [BRIDGE_OUTBOX_PAGE_BYTES - 5, BRIDGE_OUTBOX_PAGE_BYTES - 4] {
            let frame = GatewayToShell::Bye { reason: "x".repeat(reason_bytes) };
            let expected = frame.encode();
            let outbox = BridgeOutbox::new(9);
            outbox.try_send(frame).unwrap();
            let encoded = outbox.try_recv().unwrap().lease.encoded;
            let mut actual = vec![0; encoded.len()];
            assert_eq!(encoded.copy_into(0, &mut actual), actual.len());
            assert_eq!(actual, expected);
            assert_eq!(encoded.page_count(), if reason_bytes == BRIDGE_OUTBOX_PAGE_BYTES - 5 { 1 } else { 2 });
        }
    }

    #[test]
    fn bridge_outbox_terminal_close_rejects_the_exact_late_frame() {
        let handle = BridgeHandle::new();
        let (id, mut outbox) = handle.register();
        handle.unregister(id);
        let rejected = GatewayToShell::Bye { reason: "late-after-close".into() };
        assert_eq!(handle.try_send_to(id, rejected.clone()), Err(rejected));
        assert!(outbox.try_recv().is_none());
    }

    #[test]
    fn broadcast_partial_saturation_rolls_back_every_claim_and_returns_the_exact_uncloned_message() {
        let handle = BridgeHandle::new();
        let (first, _first_receiver) = handle.register();
        let (_second, mut second_receiver) = handle.register();
        for _ in 0..BRIDGE_OUTBOX_MAX_ITEMS {
            assert!(handle.try_send_to(first, GatewayToShell::Pong).is_ok());
        }
        let original = GatewayToShell::Bye { reason: "partial-saturation".into() };
        assert_eq!(handle.broadcast(original.clone()), Err(original));
        assert!(second_receiver.try_recv().is_none());
        assert_eq!(handle.inner.asynchronous.state.lock().unwrap().broadcast_len, 0);
    }

    #[test]
    fn broadcast_many_recipient_and_oversize_preflight_reject_before_encode() {
        let handle = BridgeHandle::new();
        for _ in 0..=BRIDGE_BROADCAST_MAX_RECIPIENTS {
            handle.register();
        }
        let many = GatewayToShell::Pong;
        assert_eq!(handle.broadcast(many.clone()), Err(many));

        let other = BridgeHandle::new();
        other.register();
        let oversized = GatewayToShell::ShellCommand { seq: 1, command: vec![0; BRIDGE_OUTBOX_MAX_BYTES] };
        assert_eq!(other.broadcast(oversized.clone()), Err(oversized));
        assert_eq!(other.inner.asynchronous.state.lock().unwrap().broadcast_len, 0);
    }

    #[test]
    fn shared_broadcast_leases_are_generation_keyed_and_close_rejects_aba_publish() {
        let first = Arc::new(BridgeOutbox::new(11));
        let second = Arc::new(BridgeOutbox::new(12));
        let frame = GatewayToShell::Bye { reason: "shared".into() };
        let bytes = frame.encoded_len().unwrap();
        let first_grant = first.claim(bytes).unwrap();
        let second_grant = second.claim(bytes).unwrap();
        let encoded = Arc::new(BridgeEncodedFrame::encode(&frame, bytes));
        assert!(first.publish(first_grant, Arc::clone(&encoded)).is_ok());
        assert!(second.publish(second_grant, Arc::clone(&encoded)).is_ok());
        let first_lease = first.try_recv().unwrap().lease;
        let second_lease = second.try_recv().unwrap().lease;
        assert!(Arc::ptr_eq(&first_lease.encoded, &second_lease.encoded));

        let stale = first.claim(bytes).unwrap();
        first.close();
        assert!(first.publish(stale, Arc::clone(&encoded)).is_err());
        assert!(first.try_recv().is_none(), "closed generation must not yield an ABA lease");
    }

    #[test]
    fn broadcast_close_before_first_publish_delivers_survivors_in_stable_admitted_order() {
        let handle = BridgeHandle::new();
        let (first, mut first_receiver) = handle.register();
        let (second, mut second_receiver) = handle.register();
        let original = GatewayToShell::Bye { reason: "x".repeat(BRIDGE_OUTBOX_PAGE_BYTES + 1) };
        let mut cursor = encoded_broadcast(retained_broadcast_cursor(&handle, &[first, second], original));
        handle.unregister(first);
        cursor = pending_broadcast(cursor.step());
        assert!(matches!(cursor.recipients_state[0].as_ref(), Some(BridgeRecipientState::RecipientClosed { id, .. }) if *id == first));
        cursor = pending_broadcast(cursor.step());
        assert!(matches!(cursor.recipients_state[1].as_ref(), Some(BridgeRecipientState::Published { id }) if *id == second));
        assert!(matches!(cursor.step(), BridgeBroadcastStep::Complete(BridgeBroadcastCompletion::Delivered { delivered: 1, recipient_closed: 1 })));
        assert!(first_receiver.try_recv().is_none());
        assert!(second_receiver.try_recv().is_some());
    }

    #[test]
    fn broadcast_close_mid_recipient_list_reports_partial_counts_and_fifo_delivery() {
        let handle = BridgeHandle::new();
        let (first, mut first_receiver) = handle.register();
        let (second, mut second_receiver) = handle.register();
        let (third, mut third_receiver) = handle.register();
        let mut cursor = encoded_broadcast(retained_broadcast_cursor(&handle, &[first, second, third], GatewayToShell::Pong));
        cursor = pending_broadcast(cursor.step());
        handle.unregister(second);
        cursor = pending_broadcast(cursor.step());
        cursor = pending_broadcast(cursor.step());
        assert!(matches!(cursor.recipients_state[0].as_ref(), Some(BridgeRecipientState::Published { id }) if *id == first));
        assert!(matches!(cursor.recipients_state[1].as_ref(), Some(BridgeRecipientState::RecipientClosed { id, .. }) if *id == second));
        assert!(matches!(cursor.recipients_state[2].as_ref(), Some(BridgeRecipientState::Published { id }) if *id == third));
        assert!(matches!(cursor.step(), BridgeBroadcastStep::Complete(BridgeBroadcastCompletion::Delivered { delivered: 2, recipient_closed: 1 })));
        assert_eq!(first_receiver.try_recv(), Some(GatewayToShell::Pong));
        assert!(second_receiver.try_recv().is_none());
        assert_eq!(third_receiver.try_recv(), Some(GatewayToShell::Pong));
    }

    #[test]
    fn broadcast_all_close_returns_the_exact_original_completion_after_every_claim() {
        let handle = BridgeHandle::new();
        let (first, _first_receiver) = handle.register();
        let (second, _second_receiver) = handle.register();
        let original = GatewayToShell::Bye { reason: "all-closed".into() };
        let mut cursor = encoded_broadcast(retained_broadcast_cursor(&handle, &[first, second], original.clone()));
        handle.unregister(first);
        handle.unregister(second);
        cursor = pending_broadcast(cursor.step());
        cursor = pending_broadcast(cursor.step());
        assert_eq!(
            match cursor.step() {
                BridgeBroadcastStep::Complete(completion) => completion,
                BridgeBroadcastStep::Pending(_) => panic!("all-close completion remained pending"),
            },
            BridgeBroadcastCompletion::Undelivered { frame: original, recipient_closed: 2 }
        );
    }

    #[test]
    fn broadcast_reopen_same_slot_aba_cannot_consume_the_stale_recipient_claim() {
        let old = Arc::new(BridgeOutbox::new(41));
        let frame = GatewayToShell::Pong;
        let bytes = frame.encoded_len().unwrap();
        let stale = old.claim(bytes).unwrap();
        old.close();
        let reopened = Arc::new(BridgeOutbox::new(42));
        let encoded = Arc::new(BridgeEncodedFrame::encode(&frame, bytes));
        let rejected = old.publish(stale, Arc::clone(&encoded)).unwrap_err();
        assert_eq!(rejected.grant.generation, 41);
        assert_eq!(reopened.state.lock().unwrap().len, 0);
        assert_eq!(reopened.state.lock().unwrap().reserved_items, 0);
    }

    #[test]
    fn broadcast_shutdown_cancel_poison_closes_each_remaining_claim_one_grant_then_reports_partial_delivery() {
        let handle = BridgeHandle::new();
        let (first, mut first_receiver) = handle.register();
        let (second, _second_receiver) = handle.register();
        let (third, _third_receiver) = handle.register();
        let cursor = encoded_broadcast(retained_broadcast_cursor(&handle, &[first, second, third], GatewayToShell::Pong));
        let cursor = pending_broadcast(cursor.step());
        {
            let mut state = handle.inner.asynchronous.state.lock().unwrap();
            state.broadcast_reserved -= 1;
            state.broadcasts[0] = Some(cursor);
            state.broadcast_len = 1;
        }
        handle.cancel_broadcasts();
        assert!(handle.close_one_terminal_broadcast_claim());
        assert!(handle.close_one_terminal_broadcast_claim());
        assert!(handle.close_one_terminal_broadcast_claim());
        assert_eq!(handle.take_broadcast_completion(), Some(BridgeBroadcastCompletion::Delivered { delivered: 1, recipient_closed: 2 }));
        assert_eq!(first_receiver.try_recv(), Some(GatewayToShell::Pong));
    }

    #[test]
    fn last_shared_lease_transfers_pages_to_one_page_terminal_retirement_grants() {
        let handle = BridgeHandle::new();
        let retirement = handle.inner.asynchronous.reserve_retirement().unwrap();
        handle.inner.asynchronous.terminal.store(true, Ordering::Release);
        let mut encoded = BridgeEncodedFrame::empty(BRIDGE_OUTBOX_PAGE_BYTES + 1, Some((Arc::clone(&handle.inner.asynchronous), retirement)));
        encoded.pages[0] = Some(Box::new([1; BRIDGE_OUTBOX_PAGE_BYTES]));
        encoded.pages[1] = Some(Box::new([2; BRIDGE_OUTBOX_PAGE_BYTES]));
        drop(encoded);
        assert_eq!(handle.inner.asynchronous.state.lock().unwrap().retirement_len, 1);
        assert!(handle.close_one_terminal_retired_page());
        assert!(handle.close_one_terminal_retired_page());
        assert_eq!(handle.inner.asynchronous.state.lock().unwrap().retirement_len, 0);
    }

    #[test]
    fn terminal_broadcast_close_returns_one_exact_original_and_cancels_recipient_credit() {
        let handle = BridgeHandle::new();
        let (id, _receiver) = handle.register();
        let original = GatewayToShell::Bye { reason: "terminal-broadcast".into() };
        let bytes = original.encoded_len().unwrap();
        handle.inner.asynchronous.reserve_broadcast().unwrap();
        let retirement = handle.inner.asynchronous.reserve_retirement().unwrap();
        let outbox = Arc::clone(&handle.inner.connections.lock().unwrap().get(&id).unwrap().outbox);
        let grant = outbox.claim(bytes).unwrap();
        let mut recipients_state: [Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS] = std::array::from_fn(|_| None);
        recipients_state[0] = Some(BridgeRecipientState::Claimed { id, outbox: Arc::clone(&outbox), grant });
        let cursor = BridgeBroadcastCursor {
            frame: Some(original.clone()),
            expected: bytes,
            offset: 0,
            encoded: BridgeEncodedFrame::empty(bytes, Some((Arc::clone(&handle.inner.asynchronous), retirement))),
            shared: None,
            recipients_state,
            recipients: 1,
            recipient_cursor: 0,
            delivered: 0,
            recipient_closed: 0,
        };
        {
            let mut state = handle.inner.asynchronous.state.lock().unwrap();
            state.broadcast_reserved -= 1;
            state.broadcasts[0] = Some(cursor);
            state.broadcast_len = 1;
        }
        handle.inner.asynchronous.terminal.store(true, Ordering::Release);
        assert!(handle.close_one_terminal_broadcast_claim());
        assert!(handle.close_one_terminal_broadcast_claim());
        assert_eq!(handle.take_broadcast_completion(), Some(BridgeBroadcastCompletion::Undelivered { frame: original, recipient_closed: 1 }));
        let state = outbox.state.lock().unwrap();
        assert_eq!(state.reserved_items, 0);
        assert_eq!(state.reserved_bytes, 0);
        drop(state);
        assert!(handle.close_one_terminal_retired_page());
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

        assert_eq!(handle.broadcast(GatewayToShell::Pong), Ok(1), "broadcast must reach exactly the one live connection");
        assert_eq!(recv_frame(&mut socket).await, GatewayToShell::Pong);

        drop(socket);
        server_task.abort();
    }

    #[test]
    fn send_to_an_unknown_connection_returns_false() {
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
