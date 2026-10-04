//! 📥️ Product-neutral bounded paged-command ownership, admission and acknowledged release.
use serde::Deserialize;
use std::mem::size_of;

//#region 🔖️PagedCommandIngress
pub const COMMAND_PAGE_MAXIMUM_BYTES: usize = 4_096;

/// 📥️ Largest ASSEMBLED command the host may deliver into a guest.
///
/// 🧊️ A command is a host answer like any other, so it is bound by the one declared budget for an
/// assembled host answer rather than by a transport constant of its own. Nothing here chooses a
/// ceiling: [`semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES`] already says what a guest may
/// be handed for ONE outstanding request before it answers with a typed fault instead of allocating.
pub const COMMAND_MAXIMUM_BYTES: usize = semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES;

/// 📄️ Pages that budget occupies — the assembled ceiling over the page extent, not a chosen number.
///
/// 🧊️ A page authority's spine is `COMMAND_MAXIMUM_PAGES * size_of::<FixedCommandPage>()`, and a
/// page slot is a POINTER to its own 4 KiB block (see [`FixedCommandPage`]), never the block itself
/// — so the spine stays inside [`semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`]
/// while the command it assembles is free to be as long as the host-answer budget allows. Storing
/// the blocks INLINE is what forced a 64-page ceiling: 64 inline pages are 262 272 contiguous bytes,
/// four times what a fragmented guest can be relied on to serve, and the ceiling that bought that
/// reservation also refused every command past 262 144 bytes — a 272 089-char contributions pack
/// among them (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const COMMAND_MAXIMUM_PAGES: usize = COMMAND_MAXIMUM_BYTES / COMMAND_PAGE_MAXIMUM_BYTES;
pub const COMMAND_BATCH_MAXIMUM_ITEMS: usize = 64;
pub const INVOCATION_RESULT_PACK_MAXIMUM_BYTES: usize = COMMAND_MAXIMUM_BYTES;

/// 🧱️ One command page's own 4 KiB block, reserved fallibly so an exhausted guest heap answers with
/// a `Fault` the host can display rather than `handle_alloc_error` → `unreachable`.
fn try_reserve_command_page_block() -> Result<Box<[u8; COMMAND_PAGE_MAXIMUM_BYTES]>, semio_framework_diagnostic::Fault> {
    let mut block = Vec::new();
    block
        .try_reserve_exact(COMMAND_PAGE_MAXIMUM_BYTES)
        .map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-allocation"), "a command page could not reserve its exact 4096-byte block"))?;
    block.resize(COMMAND_PAGE_MAXIMUM_BYTES, 0);
    block
        .into_boxed_slice()
        .try_into()
        .map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-allocation"), "a command page block is not its exact 4096-byte extent"))
}

/// 📄️ One 4 KiB command page, holding its block BEHIND a pointer.
///
/// 🧊️ The indirection is the whole reason a command has no page ceiling: every collection of pages
/// on this path (`CommandPageSet`, `PagedCommand`, `CommandEnvelopeSet`, `CommandBatch`) reserves a
/// spine of `size_of::<FixedCommandPage>()`-byte slots, so assembling a 272 KB command asks the
/// guest allocator for 67 separate 4 KiB blocks — each one a routine request — instead of one
/// quarter-megabyte contiguous block it is the first to refuse.
#[derive(Clone, Debug, PartialEq)]
pub struct FixedCommandPage {
    bytes: Box<[u8; COMMAND_PAGE_MAXIMUM_BYTES]>,
    len: u16,
}

impl FixedCommandPage {
    pub fn try_from_array(bytes: [u8; COMMAND_PAGE_MAXIMUM_BYTES], len: u32) -> Result<Self, semio_framework_diagnostic::Fault> {
        let len = usize::try_from(len).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-length"), "command page length is not representable"))?;
        if len > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-length"), "command page length exceeds its fixed 4096-byte authority"));
        }
        if bytes[len..].iter().any(|byte| *byte != 0) {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-padding"), "command page carries nonzero bytes outside its declared authority"));
        }
        let mut block = try_reserve_command_page_block()?;
        block.copy_from_slice(&bytes);
        Ok(Self { bytes: block, len: len as u16 })
    }

    pub fn try_copy_from(bytes: &[u8]) -> Result<Self, semio_framework_diagnostic::Fault> {
        if bytes.len() > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-length"), "command page length exceeds its fixed 4096-byte authority"));
        }
        let mut block = try_reserve_command_page_block()?;
        block[..bytes.len()].copy_from_slice(bytes);
        Ok(Self { bytes: block, len: bytes.len() as u16 })
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// 🌉️ Hand-written, not derived: `bytes` is a fixed `[u8; COMMAND_PAGE_MAXIMUM_BYTES]` (4096 slots,
/// most unused past `len`) — deriving would walk all 4096 via the blanket `[T; N]` impl instead of
/// just the live `len` prefix the old hand-rolled `serde::Serialize` tuple encoding took care to
/// emit only. Wire shape: a plain `DslValue::Array` of the `len` live bytes (no separate length
/// field — the array's own length IS the count, simpler than the old length-prefixed tuple).
impl semio_framework_value::ToValue for FixedCommandPage {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Array(self.as_slice().iter().map(semio_framework_value::ToValue::to_value).collect())
    }
}

impl semio_framework_value::FromValue for FixedCommandPage {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Array(items) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected an array for FixedCommandPage, found {value:?}")));
        };
        if items.len() > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"fixed command page exceeds 4096 bytes".to_string()));
        }
        let mut bytes = Vec::with_capacity(items.len());
        for (index, item) in items.into_iter().enumerate() {
            bytes.push(<u8 as semio_framework_value::FromValue>::from_value(item).map_err(|error| error.under(index))?);
        }
        FixedCommandPage::try_copy_from(&bytes).map_err(|fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,fault.message))
    }
}

/// 🪢️ `serde` kept alongside the hand-written `ToValue`/`FromValue` above for the same
/// wire-sharing reason as `CommandPageCursor`/`CommandIngressStatus`: mirrors the `DslValue::Array`
/// of just the `len` live bytes, no length prefix, no derive over the fixed 4096-slot backing array.
impl serde::Serialize for FixedCommandPage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.as_slice().iter().copied())
    }
}

impl<'de> serde::Deserialize<'de> for FixedCommandPage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        FixedCommandPage::try_copy_from(&bytes).map_err(|fault| serde::de::Error::custom(fault.message))
    }
}

/// 📄️ One command's page authority, reserved ONCE for exactly the pages that command DECLARES.
///
/// 🧊️ The reservation is `declared * size_of::<FixedCommandPage>()` contiguous bytes and it is taken
/// on the guest's own fixed linear memory, once per command, on the reactor's command-ingress
/// prologue. Reserving the page ceiling regardless of the declared count asked for 262 272 B for
/// a one-page command — the single largest routine allocation on a 4 Hz command stream, and the
/// first request a fragmented or exhausted guest heap refuses (`plugin.command-page-allocation`,
/// ticket 26/09/02 build #29). The declared count is validated `1..=COMMAND_MAXIMUM_PAGES` by the
/// caller's cursor and is identical for every page of one command, so an exact reservation still
/// admits every page without a second allocation.
///
/// 📐️ A slot holds a POINTER to its page's own 4 KiB block, so the whole ceiling's spine is
/// `COMMAND_MAXIMUM_PAGES * size_of::<FixedCommandPage>()` — inside
/// [`semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`], which is the law that decides
/// how many pages a command may declare at all.
#[derive(Debug, PartialEq)]
pub struct CommandPageSet {
    pages: std::collections::VecDeque<FixedCommandPage>,
    declared: usize,
    byte_len: usize,
    generic_shape_valid: bool,
    all_nonempty: bool,
}

impl CommandPageSet {
    pub fn try_new(declared: usize) -> Result<Self, semio_framework_diagnostic::Fault> {
        if declared == 0 || declared > COMMAND_MAXIMUM_PAGES {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-count"), "a command page authority is declared for 1..=COMMAND_MAXIMUM_PAGES pages"));
        }
        let mut pages = std::collections::VecDeque::new();
        pages.try_reserve_exact(declared).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-allocation"), "declared command page authority could not reserve its exact page slots"))?;
        Ok(Self { pages, declared, byte_len: 0, generic_shape_valid: true, all_nonempty: true })
    }

    /// 📏️ Contiguous bytes a `declared`-page authority reserves — what the reactor's footprint law
    /// compares against `semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`.
    pub const fn reservation_bytes(declared: usize) -> usize {
        declared * size_of::<FixedCommandPage>()
    }

    pub fn declared(&self) -> usize {
        self.declared
    }

    pub fn try_push(&mut self, page: FixedCommandPage) -> Result<(), (semio_framework_diagnostic::Fault, FixedCommandPage)> {
        if self.pages.len() == self.declared {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-count"), "command page authority is saturated"), page));
        }
        let Some(byte_len) = self.byte_len.checked_add(page.len()).filter(|total| *total <= COMMAND_MAXIMUM_BYTES) else {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-byte-cap"), "command exceeds the assembled host-answer authority a guest may be handed"), page));
        };
        if page.is_empty() {
            self.generic_shape_valid = false;
            self.all_nonempty = false;
        }
        if self.pages.back().is_some_and(|previous| previous.len() != COMMAND_PAGE_MAXIMUM_BYTES) {
            self.generic_shape_valid = false;
        }
        self.pages.push_back(page);
        self.byte_len = byte_len;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(length) = self.pages.front().map(FixedCommandPage::len) else {
            return (true, 0);
        };
        if length > maximum_bytes {
            return (false, 0);
        }
        let page = self.pages.pop_front().expect("fixed command page was present");
        let released = page.len();
        self.byte_len -= released;
        drop(page);
        (self.pages.is_empty(), released)
    }
}

#[derive(Debug, PartialEq)]
pub struct PagedCommand {
    pages: std::collections::VecDeque<FixedCommandPage>,
    byte_len: usize,
    kind: u8,
    metadata: u32,
    item_count: u32,
}

impl PagedCommand {
    pub fn try_from_pages(pages: CommandPageSet) -> Result<Self, (semio_framework_diagnostic::Fault, CommandPageSet)> {
        if pages.is_empty() || pages.len() > COMMAND_MAXIMUM_PAGES {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-count"), "command requires 1..=COMMAND_MAXIMUM_PAGES admitted pages"), pages));
        }
        if !pages.generic_shape_valid {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-shape"), "command pages must be nonempty, at most 4096 bytes, and every nonterminal page must be full"), pages));
        }
        let Some(kind) = pages.pages.front().and_then(|page| page.as_slice().first()).copied() else {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-empty"), "command has no kind byte"), pages));
        };
        Ok(Self { pages: pages.pages, byte_len: pages.byte_len, kind, metadata: 0, item_count: 0 })
    }

    pub fn try_from_presence_pages(own_color: Option<u8>, pages: CommandPageSet, item_count: usize) -> Result<Self, (semio_framework_diagnostic::Fault, CommandPageSet)> {
        if item_count > COMMAND_BATCH_MAXIMUM_ITEMS || pages.len() != item_count.max(1) {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-presence-item-cap"), "Presence command requires one exact page per peer and at most 64 peers"), pages));
        }
        if (item_count == 0 && pages.byte_len != 0) || (item_count != 0 && !pages.all_nonempty) {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-presence-page-shape"), "each Presence peer page must be nonempty and at most 4096 bytes"), pages));
        }
        let metadata = own_color.map_or(0, |color| (1u32 << 8) | u32::from(color));
        Ok(Self { pages: pages.pages, byte_len: pages.byte_len, kind: 28, metadata, item_count: item_count as u32 })
    }

    pub fn byte_len(&self) -> usize {
        self.byte_len
    }

    pub fn page_len(&self) -> usize {
        self.pages.len()
    }

    pub fn front_page(&self) -> Option<&FixedCommandPage> {
        self.pages.front()
    }

    pub fn release_front_page(&mut self, maximum_bytes: usize) -> Option<(bool, usize)> {
        let page_len = self.pages.front().map(FixedCommandPage::len)?;
        if page_len > maximum_bytes {
            return None;
        }
        let page = self.pages.pop_front().expect("front page was present");
        self.byte_len -= page.len();
        let released = page.len();
        drop(page);
        Some((self.pages.is_empty(), released))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.is_empty() && self.byte_len == 0
    }

    pub fn kind(&self) -> u8 {
        self.kind
    }

    pub fn metadata(&self) -> u32 {
        self.metadata
    }

    pub fn item_count(&self) -> u32 {
        self.item_count
    }
}

#[derive(Debug)]
pub struct PagedCommandReader {
    command: PagedCommand,
    offset: usize,
}

impl PagedCommandReader {
    pub fn new(command: PagedCommand) -> Self {
        Self { command, offset: 0 }
    }

    pub fn kind(&self) -> u8 {
        self.command.kind()
    }

    pub fn read_byte(&mut self) -> Result<u8, semio_framework_diagnostic::Fault> {
        let byte = self
            .command
            .front_page()
            .and_then(|page| page.as_slice().get(self.offset))
            .copied()
            .ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-decode-truncated"), "paged command ended inside a field"))?;
        self.offset += 1;
        if self.command.front_page().is_some_and(|page| self.offset == page.len()) {
            let _ = self.command.release_front_page(COMMAND_PAGE_MAXIMUM_BYTES).expect("fully consumed fixed page is releasable");
            self.offset = 0;
        }
        Ok(byte)
    }

    pub fn read_varint(&mut self) -> Result<u64, semio_framework_diagnostic::Fault> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.read_byte()?;
            if shift == 63 && byte > 1 {
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-decode-varint"), "paged command varint overflowed u64"));
            }
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-decode-varint"), "paged command varint exceeds ten bytes"))
    }

    pub fn read_bounded_bytes(&mut self, maximum: usize) -> Result<Vec<u8>, semio_framework_diagnostic::Fault> {
        let length = usize::try_from(self.read_varint()?).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-length"), "paged command field length is not representable"))?;
        if length > maximum {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-cap"), "paged command field exceeds its exact bounded decode authority"));
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(length).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-allocation"), "paged command field could not reserve its exact bounded authority"))?;
        for _ in 0..length {
            bytes.push(self.read_byte()?);
        }
        Ok(bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.offset == 0 && self.command.terminal_is_empty()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(page_len) = self.command.front_page().map(FixedCommandPage::len) else {
            return (self.offset == 0, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let released = self.command.release_front_page(maximum_bytes).expect("front fixed page was grant-admitted").1;
        self.offset = 0;
        (self.command.terminal_is_empty(), released)
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandEnvelope {
    pub instance: u32,
    pub seq: u64,
    pub command: PagedCommand,
}

#[derive(Debug, PartialEq)]
pub struct CommandBatch {
    pub generation: u64,
    commands: std::collections::VecDeque<CommandBatchEntry>,
    pages: std::collections::VecDeque<FixedCommandPage>,
    bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CommandBatchEntry {
    instance: u32,
    seq: u64,
    kind: u8,
    metadata: u32,
    item_count: u32,
    page_count: u32,
    remaining_pages: u32,
}

#[derive(Debug, PartialEq)]
pub struct CommandEnvelopeSet {
    commands: std::collections::VecDeque<CommandBatchEntry>,
    page_storage: std::collections::VecDeque<FixedCommandPage>,
    pages: usize,
    bytes: usize,
}

impl CommandEnvelopeSet {
    pub fn try_new() -> Result<Self, semio_framework_diagnostic::Fault> {
        let mut commands = std::collections::VecDeque::new();
        commands
            .try_reserve_exact(COMMAND_BATCH_MAXIMUM_ITEMS)
            .map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-allocation"), "fixed command batch authority could not reserve its exact 64 slots"))?;
        Ok(Self { commands, page_storage: std::collections::VecDeque::new(), pages: 0, bytes: 0 })
    }

    pub fn try_push(&mut self, command: CommandEnvelope) -> Result<(), (semio_framework_diagnostic::Fault, CommandEnvelope)> {
        if self.commands.len() == COMMAND_BATCH_MAXIMUM_ITEMS {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-cap"), "command batch exceeds its exact 64-item authority"), command));
        }
        let pages = match self.pages.checked_add(command.command.page_len()) {
            Some(pages) if pages <= COMMAND_MAXIMUM_PAGES => pages,
            _ => return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-page-cap"), "command batch exceeds its aggregate page authority"), command)),
        };
        let bytes = match self.bytes.checked_add(command.command.byte_len()) {
            Some(bytes) if bytes <= COMMAND_MAXIMUM_BYTES => bytes,
            _ => return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-byte-cap"), "command batch exceeds its aggregate assembled-byte authority"), command)),
        };
        if self.page_storage.try_reserve_exact(command.command.page_len()).is_err() {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-page-allocation"), "command batch could not reserve the exact page slots this command declares"), command));
        }
        let CommandEnvelope { instance, seq, command } = command;
        let PagedCommand { pages: mut command_pages, kind, metadata, item_count, .. } = command;
        let page_count = u32::try_from(command_pages.len()).expect("admitted command page count is u32-bounded");
        self.commands.push_back(CommandBatchEntry { instance, seq, kind, metadata, item_count, page_count, remaining_pages: page_count });
        while let Some(page) = command_pages.pop_front() {
            self.page_storage.push_back(page);
        }
        self.pages = pages;
        self.bytes = bytes;
        Ok(())
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(command) = self.commands.front_mut() else {
            return (self.page_storage.is_empty(), 0);
        };
        if command.remaining_pages == 0 {
            let _terminal = self.commands.pop_front().expect("empty command-build shell was present");
            return (self.commands.is_empty() && self.page_storage.is_empty(), 0);
        }
        let Some(page_len) = self.page_storage.front().map(FixedCommandPage::len) else {
            return (false, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let page = self.page_storage.pop_front().expect("command-build page was present");
        let released = page.len();
        self.pages -= 1;
        self.bytes -= released;
        command.remaining_pages -= 1;
        drop(page);
        if command.remaining_pages == 0 {
            let _terminal = self.commands.pop_front().expect("empty command-build shell was present");
        }
        (self.commands.is_empty() && self.page_storage.is_empty(), released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.commands.is_empty() && self.page_storage.is_empty() && self.pages == 0 && self.bytes == 0
    }
}

#[derive(Debug)]
pub struct RejectedCommandBuild {
    rejected: Option<CommandEnvelope>,
    admitted: CommandEnvelopeSet,
}

impl RejectedCommandBuild {
    pub fn new(admitted: CommandEnvelopeSet, rejected: CommandEnvelope) -> Self {
        Self { rejected: Some(rejected), admitted }
    }

    pub fn from_admitted(admitted: CommandEnvelopeSet) -> Self {
        Self { rejected: None, admitted }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        if let Some(rejected) = self.rejected.as_mut() {
            let Some((empty, released)) = rejected.command.release_front_page(maximum_bytes) else {
                return (false, 0);
            };
            if empty {
                let _terminal = self.rejected.take().expect("rejected command reached terminal empty");
            }
            return (self.terminal_is_empty(), released);
        }
        self.admitted.close_step(maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.rejected.is_none() && self.admitted.terminal_is_empty()
    }

    pub fn remaining_pages(&self) -> usize {
        self.rejected.as_ref().map_or(0, |rejected| rejected.command.page_len()) + self.admitted.pages
    }

    pub fn remaining_bytes(&self) -> usize {
        self.rejected.as_ref().map_or(0, |rejected| rejected.command.byte_len()) + self.admitted.bytes
    }
}

#[derive(Debug)]
pub struct RejectedCommandBuildRegistry<const CAPACITY: usize> {
    slots: [Option<RejectedCommandBuild>; CAPACITY],
    close_index: usize,
    occupied: usize,
}

impl<const CAPACITY: usize> Default for RejectedCommandBuildRegistry<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize> RejectedCommandBuildRegistry<CAPACITY> {
    pub fn new() -> Self {
        assert!(CAPACITY > 0);
        Self { slots: std::array::from_fn(|_| None), close_index: 0, occupied: 0 }
    }

    pub fn can_insert(&self, key: u64) -> bool {
        self.slots[key as usize % CAPACITY].is_none()
    }

    pub fn try_insert(&mut self, key: u64, owner: RejectedCommandBuild) -> Result<(), (semio_framework_diagnostic::Fault, RejectedCommandBuild)> {
        let index = key as usize % CAPACITY;
        if self.slots[index].is_some() {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-build-close-capacity"), "fixed rejected command-build close registry is occupied or collided"), owner));
        }
        self.slots[index] = Some(owner);
        self.occupied += 1;
        Ok(())
    }

    pub fn insert_admitted(&mut self, key: u64, owner: RejectedCommandBuild) {
        let index = key as usize % CAPACITY;
        assert!(self.slots[index].is_none(), "fixed rejected command-build admission changed before insert");
        self.slots[index] = Some(owner);
        self.occupied += 1;
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if self.occupied == 0 {
            return (true, 0, 0);
        }
        for _ in 0..CAPACITY {
            let index = self.close_index;
            self.close_index = (self.close_index + 1) % CAPACITY;
            let Some(owner) = self.slots[index].as_mut() else {
                continue;
            };
            let (terminal, released) = owner.close_step(maximum_bytes);
            if terminal {
                let terminal = self.slots[index].take().expect("terminal rejected command build was present");
                self.occupied -= 1;
                assert!(terminal.terminal_is_empty(), "rejected command build terminal witness changed before removal");
            }
            return (self.occupied == 0, 1, released);
        }
        (false, 0, 0)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.occupied == 0
    }
}

impl CommandBatch {
    pub fn try_new(generation: u64, commands: CommandEnvelopeSet) -> Result<Self, (semio_framework_diagnostic::Fault, CommandEnvelopeSet)> {
        if commands.commands.is_empty() {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-cap"), "command batch requires at least one exact command owner"), commands));
        }
        Ok(Self { generation, commands: commands.commands, pages: commands.page_storage, bytes: commands.bytes })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn remaining_pages(&self) -> usize {
        self.pages.len()
    }

    pub fn remaining_bytes(&self) -> usize {
        self.bytes
    }

    fn terminal_is_empty(&self) -> bool {
        self.commands.is_empty() && self.pages.is_empty() && self.bytes == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandBatchProgress {
    PageReady,
    Waiting,
    Complete,
    Faulted,
}

#[derive(Clone, Copy)]
enum CursorShape {
    Page,
    Terminal,
    Fault,
}

#[derive(Debug)]
pub struct CommandBatchDriver {
    owner: u64,
    batch: CommandBatch,
    command_index: u32,
    page_index: u32,
    admitted_page_count: u32,
    admitted_kind: u8,
    faulted: bool,
    waiting: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CommandDriverRetentionState {
    Active,
    Suspended,
    Closing,
}

#[derive(Debug)]
struct CommandDriverRetentionSlot {
    key: u64,
    generation: u64,
    driver: CommandBatchDriver,
    state: CommandDriverRetentionState,
    close_previous: Option<u16>,
    close_next: Option<u16>,
}

#[derive(Debug)]
pub struct CommandDriverRegistry<const CAPACITY: usize> {
    slots: [Option<CommandDriverRetentionSlot>; CAPACITY],
    close_head: Option<u16>,
    close_tail: Option<u16>,
    occupied: usize,
}

impl<const CAPACITY: usize> Default for CommandDriverRegistry<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize> CommandDriverRegistry<CAPACITY> {
    pub fn new() -> Self {
        assert!(CAPACITY > 0 && CAPACITY <= usize::from(u16::MAX));
        Self { slots: std::array::from_fn(|_| None), close_head: None, close_tail: None, occupied: 0 }
    }

    pub fn try_insert(&mut self, key: u64, generation: u64, driver: CommandBatchDriver) -> Result<(), (semio_framework_diagnostic::Fault, CommandBatchDriver)> {
        if !self.can_insert(key) {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-capacity"), "fixed retained command-driver slot is occupied or collided"), driver));
        }
        self.insert_admitted(key, generation, driver);
        Ok(())
    }

    pub fn can_insert(&self, key: u64) -> bool {
        self.slots[key as usize % CAPACITY].is_none()
    }

    pub fn insert_admitted(&mut self, key: u64, generation: u64, driver: CommandBatchDriver) {
        let index = key as usize % CAPACITY;
        assert!(self.slots[index].is_none(), "fixed retained command-driver admission changed before insert");
        self.slots[index] = Some(CommandDriverRetentionSlot { key, generation, driver, state: CommandDriverRetentionState::Active, close_previous: None, close_next: None });
        self.occupied += 1;
    }

    pub fn with_driver_mut<R>(&mut self, key: u64, generation: u64, f: impl FnOnce(&mut CommandBatchDriver) -> R) -> Result<R, semio_framework_diagnostic::Fault> {
        let slot = self.slot_mut(key, generation)?;
        if slot.state != CommandDriverRetentionState::Active {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-not-active"), "retained command driver is suspended or closing"));
        }
        Ok(f(&mut slot.driver))
    }

    pub fn prepare_suspend(&mut self, key: u64, generation: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Active {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-suspend-state"), "retained command driver is not active before suspension"));
        }
        self.link_close(index);
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Suspended;
        Ok(())
    }

    pub fn resume(&mut self, key: u64, generation: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Suspended {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-resume-state"), "retained command driver is not suspended before resume"));
        }
        self.unlink_close(index);
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Active;
        Ok(())
    }

    pub fn begin_close(&mut self, key: u64, generation: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        let state = self.slots[index].as_ref().expect("retained command slot exists").state;
        if state == CommandDriverRetentionState::Active {
            self.link_close(index);
        }
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Closing;
        Ok(())
    }

    pub fn begin_close_key(&mut self, key: u64) -> Result<u64, semio_framework_diagnostic::Fault> {
        let index = key as usize % CAPACITY;
        let generation = match self.slots[index].as_ref() {
            Some(slot) if slot.key == key => slot.generation,
            _ => return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-stale"), "retained command driver key is stale")),
        };
        self.begin_close(key, generation)?;
        Ok(generation)
    }

    pub fn remove_terminal(&mut self, key: u64, generation: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        if !self.slots[index].as_ref().expect("retained command slot exists").driver.terminal_is_empty() {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-nonterminal-remove"), "retained command driver cannot be removed before terminal empty"));
        }
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Active {
            self.unlink_close(index);
        }
        let terminal = self.slots[index].take().expect("retained command slot exists");
        self.occupied -= 1;
        drop(terminal);
        Ok(())
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let Some(index) = self.close_head.map(usize::from) else {
            return (self.occupied == 0, 0, 0);
        };
        let (terminal, released) = {
            let slot = self.slots[index].as_mut().expect("close-list command slot exists");
            slot.driver.close_step(maximum_bytes)
        };
        if terminal {
            self.unlink_close(index);
            let terminal = self.slots[index].take().expect("terminal command slot exists");
            self.occupied -= 1;
            drop(terminal);
        }
        (self.occupied == 0, 1, released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.occupied == 0 && self.close_head.is_none() && self.close_tail.is_none()
    }

    pub fn has_close_work(&self) -> bool {
        self.close_head.is_some()
    }

    pub fn contains(&self, key: u64, generation: u64) -> bool {
        self.index_of(key, generation).is_ok()
    }

    pub fn is_active(&self, key: u64, generation: u64) -> bool {
        self.index_of(key, generation).ok().and_then(|index| self.slots[index].as_ref()).is_some_and(|slot| slot.state == CommandDriverRetentionState::Active)
    }

    fn slot_mut(&mut self, key: u64, generation: u64) -> Result<&mut CommandDriverRetentionSlot, semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        Ok(self.slots[index].as_mut().expect("retained command slot exists"))
    }

    fn index_of(&self, key: u64, generation: u64) -> Result<usize, semio_framework_diagnostic::Fault> {
        let index = key as usize % CAPACITY;
        match self.slots[index].as_ref() {
            Some(slot) if slot.key == key && slot.generation == generation => Ok(index),
            _ => Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-stale"), "retained command driver identity or generation is stale")),
        }
    }

    fn link_close(&mut self, index: usize) {
        let previous = self.close_tail;
        let index_u16 = u16::try_from(index).expect("command registry capacity is u16-bounded");
        {
            let slot = self.slots[index].as_mut().expect("retained command slot exists");
            slot.close_previous = previous;
            slot.close_next = None;
        }
        if let Some(previous) = previous {
            self.slots[usize::from(previous)].as_mut().expect("previous close slot exists").close_next = Some(index_u16);
        } else {
            self.close_head = Some(index_u16);
        }
        self.close_tail = Some(index_u16);
    }

    fn unlink_close(&mut self, index: usize) {
        let (previous, next) = {
            let slot = self.slots[index].as_ref().expect("retained command slot exists");
            (slot.close_previous, slot.close_next)
        };
        if let Some(previous) = previous {
            self.slots[usize::from(previous)].as_mut().expect("previous close slot exists").close_next = next;
        } else {
            self.close_head = next;
        }
        if let Some(next) = next {
            self.slots[usize::from(next)].as_mut().expect("next close slot exists").close_previous = previous;
        } else {
            self.close_tail = previous;
        }
        let slot = self.slots[index].as_mut().expect("retained command slot exists");
        slot.close_previous = None;
        slot.close_next = None;
    }
}

impl CommandBatchDriver {
    pub fn new(owner: u64, batch: CommandBatch) -> Self {
        Self { owner, batch, command_index: 0, page_index: 0, admitted_page_count: 0, admitted_kind: 0, faulted: false, waiting: false }
    }

    pub fn next_page(&mut self) -> Result<Option<(CommandPageCursor, FixedCommandPage)>, semio_framework_diagnostic::Fault> {
        if self.faulted || self.waiting {
            return Ok(None);
        }
        let Some(command) = self.batch.commands.front() else {
            return Ok(None);
        };
        if self.admitted_page_count == 0 {
            self.admitted_page_count = command.page_count;
            self.admitted_kind = command.kind;
        }
        let bytes = self.batch.pages.front().ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-empty"), "nonterminal command owner has no page"))?.clone();
        let cursor = CommandPageCursor {
            owner: self.owner,
            generation: self.batch.generation,
            command_index: self.command_index,
            command_count: u32::try_from(self.batch.commands.len()).unwrap_or(u32::MAX).saturating_add(self.command_index),
            instance: command.instance,
            seq: command.seq,
            kind: self.admitted_kind,
            page_index: self.page_index,
            page_count: self.admitted_page_count,
            item_count: command.item_count,
            metadata: command.metadata,
        };
        Ok(Some((cursor, bytes)))
    }

    pub fn observe(&mut self, status: &CommandIngressStatus, maximum_release_bytes: usize) -> Result<CommandBatchProgress, semio_framework_diagnostic::Fault> {
        let (cursor, shape) = match status {
            CommandIngressStatus::Idle => {
                return Ok(if self.batch.commands.is_empty() {
                    CommandBatchProgress::Complete
                } else if self.faulted {
                    CommandBatchProgress::Faulted
                } else if self.waiting {
                    CommandBatchProgress::Waiting
                } else {
                    CommandBatchProgress::PageReady
                });
            }
            CommandIngressStatus::PageAccepted(cursor) | CommandIngressStatus::Backpressure(cursor) => (cursor, CursorShape::Page),
            CommandIngressStatus::CommandPending(cursor) | CommandIngressStatus::CommandComplete(cursor) => (cursor, CursorShape::Terminal),
            CommandIngressStatus::Fault { cursor, .. } => (cursor, CursorShape::Fault),
        };
        self.validate_cursor(cursor, shape)?;
        match status {
            CommandIngressStatus::Backpressure(_) => Ok(CommandBatchProgress::PageReady),
            CommandIngressStatus::PageAccepted(_) => {
                let Some(command) = self.batch.commands.front_mut() else {
                    return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "accepted page has no exact host owner"));
                };
                let page_len =
                    self.batch.pages.front().map(FixedCommandPage::len).ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "accepted page has no exact retained batch page"))?;
                if page_len > maximum_release_bytes {
                    return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-release-budget"), "accepted page exceeds its exact release grant"));
                }
                let page = self.batch.pages.pop_front().expect("accepted retained batch page was present");
                let released = page.len();
                self.batch.bytes -= released;
                drop(page);
                command.remaining_pages = command
                    .remaining_pages
                    .checked_sub(1)
                    .ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-underflow"), "accepted page arrived after the retained command page owner was empty"))?;
                let empty = command.remaining_pages == 0;
                self.page_index = self.page_index.saturating_add(1);
                if empty {
                    self.waiting = true;
                    Ok(CommandBatchProgress::Waiting)
                } else {
                    Ok(CommandBatchProgress::PageReady)
                }
            }
            CommandIngressStatus::CommandPending(_) => {
                self.waiting = true;
                Ok(CommandBatchProgress::Waiting)
            }
            CommandIngressStatus::CommandComplete(_) => {
                let Some(command) = self.batch.commands.front() else {
                    return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "terminal command has no exact host owner"));
                };
                if command.remaining_pages > 1 {
                    return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-terminal-pages"), "terminal acknowledgement arrived before every exact page was released"));
                }
                if command.remaining_pages == 1 {
                    let page_len = self
                        .batch
                        .pages
                        .front()
                        .map(FixedCommandPage::len)
                        .ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "terminal command has no exact retained batch page"))?;
                    if page_len > maximum_release_bytes {
                        return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-release-budget"), "terminal page exceeds its exact release grant"));
                    }
                    let page = self.batch.pages.pop_front().expect("terminal retained batch page was present");
                    self.batch.bytes -= page.len();
                    drop(page);
                }
                let _terminal = self.batch.commands.pop_front().expect("terminal command was present");
                self.command_index = self.command_index.saturating_add(1);
                self.page_index = 0;
                self.admitted_page_count = 0;
                self.admitted_kind = 0;
                self.waiting = false;
                Ok(if self.batch.commands.is_empty() { CommandBatchProgress::Complete } else { CommandBatchProgress::PageReady })
            }
            CommandIngressStatus::Fault { .. } => {
                self.faulted = true;
                Ok(CommandBatchProgress::Faulted)
            }
            CommandIngressStatus::Idle => unreachable!(),
        }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(command) = self.batch.commands.front_mut() else {
            return (self.batch.pages.is_empty(), 0);
        };
        if command.remaining_pages == 0 {
            let _terminal = self.batch.commands.pop_front().expect("empty retained command shell was present");
            self.command_index = self.command_index.saturating_add(1);
            self.page_index = 0;
            self.admitted_page_count = 0;
            self.admitted_kind = 0;
            self.waiting = false;
            return (self.batch.terminal_is_empty(), 0);
        }
        let Some(page_len) = self.batch.pages.front().map(FixedCommandPage::len) else {
            return (false, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let page = self.batch.pages.pop_front().expect("retained batch close page was present");
        let released = page.len();
        self.batch.bytes -= released;
        drop(page);
        command.remaining_pages -= 1;
        let empty = command.remaining_pages == 0;
        if empty {
            let _terminal = self.batch.commands.pop_front().expect("empty command was present");
            self.command_index = self.command_index.saturating_add(1);
            self.page_index = 0;
            self.admitted_page_count = 0;
            self.admitted_kind = 0;
            self.waiting = false;
        }
        (self.batch.terminal_is_empty(), released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.batch.terminal_is_empty()
    }

    pub fn generation(&self) -> u64 {
        self.batch.generation
    }

    pub fn remaining_pages(&self) -> usize {
        self.batch.pages.len()
    }

    pub fn remaining_bytes(&self) -> usize {
        self.batch.bytes
    }

    /// 🎯️ A terminal status carries `last_page_index + 1`, and a guest that consumed a command's LAST
    /// page and ran it to completion inside ONE turn publishes only that terminal — the per-page
    /// `PageAccepted` the host would otherwise have stepped through never exists. (`⚛️reactor/🔄️turn`'s
    /// single-page fast path does exactly this for every `page_count == 1` command, i.e. every ordinary
    /// mutation.) So a terminal cursor is validated against `page_index + remaining_pages`, which is
    /// the same number in both shapes, and a fault's cursor is accepted at either end of that range —
    /// a fault must attribute itself to its owner, never be replaced by a cursor complaint that hides
    /// the guest's own message (ticket 26/09/18 slice A2).
    fn validate_cursor(&self, cursor: &CommandPageCursor, shape: CursorShape) -> Result<(), semio_framework_diagnostic::Fault> {
        let Some(command) = self.batch.commands.front() else {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "ingress status has no exact host owner"));
        };
        let terminal_page_index = self.page_index.saturating_add(command.remaining_pages);
        let page_index_matches = match shape {
            CursorShape::Page => cursor.page_index == self.page_index,
            CursorShape::Terminal => cursor.page_index == terminal_page_index,
            CursorShape::Fault => cursor.page_index == self.page_index || cursor.page_index == terminal_page_index,
        };
        let mismatch = if cursor.owner != self.owner {
            Some(("owner", u64::from(cursor.owner), self.owner))
        } else if cursor.generation != self.batch.generation {
            Some(("generation", cursor.generation, self.batch.generation))
        } else if cursor.command_index != self.command_index {
            Some(("commandIndex", u64::from(cursor.command_index), u64::from(self.command_index)))
        } else if cursor.instance != command.instance {
            Some(("instance", u64::from(cursor.instance), u64::from(command.instance)))
        } else if cursor.seq != command.seq {
            Some(("seq", cursor.seq, command.seq))
        } else if cursor.kind != self.admitted_kind {
            Some(("kind", u64::from(cursor.kind), u64::from(self.admitted_kind)))
        } else if !page_index_matches {
            Some(("pageIndex", u64::from(cursor.page_index), u64::from(if matches!(shape, CursorShape::Page) { self.page_index } else { terminal_page_index })))
        } else if cursor.page_count != self.admitted_page_count {
            Some(("pageCount", u64::from(cursor.page_count), u64::from(self.admitted_page_count)))
        } else if cursor.item_count != command.item_count {
            Some(("itemCount", u64::from(cursor.item_count), u64::from(command.item_count)))
        } else if cursor.metadata != command.metadata {
            Some(("metadata", u64::from(cursor.metadata), u64::from(command.metadata)))
        } else {
            None
        };
        if let Some((field, reported, expected)) = mismatch {
            return Err(semio_framework_diagnostic::Fault::new(
                semio_framework_diagnostic::FaultOrigin::Framework,
                semio_framework_diagnostic::FaultCode::new("plugin.command-cursor-mismatch"),
                format!("ingress status does not identify the exact retained host owner: {field} is {reported}, the retained owner's is {expected}"),
            ));
        }
        Ok(())
    }
}

/// 🪢️ `serde` kept alongside `ToValue`/`FromValue`: `kernel::Event`/`TurnResult` carry this type
/// across the plugin-host `serde_json` wire (`🔌️plugin/🖥️host/🧵️shard/🦀️.rs`'s
/// `serde_json::to_vec(&result.command_ingress)`) and stay on that encoding, not `DslValue` — both
/// derives must produce the same shape, so `#[serde(rename_all = "camelCase")]` mirrors `#[value(…)]`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandPageCursor {
    pub owner: u64,
    pub generation: u64,
    pub command_index: u32,
    pub command_count: u32,
    pub instance: u32,
    pub seq: u64,
    pub kind: u8,
    pub page_index: u32,
    pub page_count: u32,
    pub item_count: u32,
    pub metadata: u32,
}

/// 🪢️ `serde` kept alongside `ToValue`/`FromValue` — same wire-sharing reason as
/// `CommandPageCursor` above. No `#[serde(tag = …)]`: `#[value(rename_all = "camelCase")]` with no
/// `tag` derives externally-tagged (`✨️derive/🦀️.rs`'s documented default for a tag-less,
/// mixed-variant enum), which is also serde's own default enum representation — the two already
/// agree without a matching attribute.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum CommandIngressStatus {
    Idle,
    PageAccepted(CommandPageCursor),
    Backpressure(CommandPageCursor),
    CommandPending(CommandPageCursor),
    CommandComplete(CommandPageCursor),
    Fault { cursor: CommandPageCursor, fault: Vec<u8> },
}
//#endregion 🔖️PagedCommandIngress

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
