//! 🔍️ The bounded shell frame decode: limits, frame kinds, faults, the UTF-8 cursor and the validating decode cursor.

use super::shell_materialize_cursor::ValidatedShellFrame;

//#region 🔖️BoundedShellDecode

pub const BRIDGE_INBOUND_PAGE_BYTES: usize = 16_384;

const BRIDGE_INBOUND_MAX_BYTES: usize = 1_048_576;

pub(super) const BRIDGE_INBOUND_MAX_PAGES: usize = BRIDGE_INBOUND_MAX_BYTES.div_ceil(BRIDGE_INBOUND_PAGE_BYTES);

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
    pub(super) fn new() -> Self {
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

    pub(super) fn read_u8<F>(&mut self, byte_at: &mut F) -> Result<u8, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let value = byte_at(self.cursor).ok_or(ShellDecodeFault::Malformed)?;
        self.cursor += 1;
        Ok(value)
    }

    pub(super) fn read_u16<F>(&mut self, byte_at: &mut F) -> Result<u16, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let mut bytes = [0; 2];
        self.read_fixed(byte_at, &mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    pub(super) fn read_u32<F>(&mut self, byte_at: &mut F) -> Result<u32, ShellDecodeFault>
    where
        F: FnMut(usize) -> Option<u8>,
    {
        let mut bytes = [0; 4];
        self.read_fixed(byte_at, &mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub(super) fn read_u64<F>(&mut self, byte_at: &mut F) -> Result<u64, ShellDecodeFault>
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

    pub(super) fn read_bool<F>(&mut self, byte_at: &mut F) -> Result<bool, ShellDecodeFault>
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
//#endregion 🔖️BoundedShellDecode
