//! 🧬️ The bounded shell frame materialization: the validated frame, owned ranges and the materialize cursor.

use super::shared_types::{ApprovalDecision, BridgeFlags, BridgeInstanceRef, ShellKind};
use super::shell_decode_cursor::{BRIDGE_INBOUND_MAX_PAGES, BRIDGE_INBOUND_PAGE_BYTES, ShellDecodeFault, ShellFrameKind};
use super::shell_to_gateway::ShellToGateway;

//#region 🔖️BoundedShellDecode
pub(crate) struct ValidatedShellFrame {
    pub kind: ShellFrameKind,
    pub len: usize,
    pub pages: [Option<Box<[u8; BRIDGE_INBOUND_PAGE_BYTES]>>; BRIDGE_INBOUND_MAX_PAGES],
}

impl ValidatedShellFrame {
    pub(super) fn new(kind: ShellFrameKind, len: usize) -> Self {
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
    pub(super) range: Option<OwnedRange>,
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

    pub(super) fn read_u8(&mut self) -> Result<u8, ShellDecodeFault> {
        let value = self.frame.byte(self.position)?;
        self.position += 1;
        Ok(value)
    }

    pub(super) fn read_bool(&mut self) -> Result<bool, ShellDecodeFault> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ShellDecodeFault::Malformed),
        }
    }

    pub(super) fn read_u16(&mut self) -> Result<u16, ShellDecodeFault> {
        let mut bytes = [0; 2];
        self.frame.copy_into(self.position, &mut bytes)?;
        self.position += bytes.len();
        Ok(u16::from_le_bytes(bytes))
    }

    pub(super) fn read_u32(&mut self) -> Result<u32, ShellDecodeFault> {
        let mut bytes = [0; 4];
        self.frame.copy_into(self.position, &mut bytes)?;
        self.position += bytes.len();
        Ok(u32::from_le_bytes(bytes))
    }

    pub(super) fn read_u64(&mut self) -> Result<u64, ShellDecodeFault> {
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
