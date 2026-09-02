//! 🎞️ Protocol app-engine channel: the `AppCommand`/`AppFrame` binary frame taxonomy every app,
//! once turned into a headless engine driven by bidirectional streaming of typed binary commands,
//! exchanges with its client (a UI or a headless runner) — every UI interaction becomes a
//! forwarded `AppCommand`, every engine reaction a returned `AppFrame`. Ticket:
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️01/HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS/`.
//!
//! 🎯️ Mirrors `protocol_wire`'s W5 hand-rolled binary layout exactly: `tag: u8` (assigned
//! sequentially in match-arm declaration order below, NOT the enum's own discriminant) followed by
//! its fields in declaration order, no per-field tags, no body-length prefix — one frame per
//! channel message. `crate::os_spr::wire::🔖️WireCodec` supplies the primitive codec
//! (`write_varint_u64`/`write_str`/`write_bytes`/`write_bool` and their `read_*` twins); this crate
//! adds only the option/vec combinators and the two enums' tag dispatch below. Unlike
//! `crate::os_spr::wire::ClientFrame`/`ServerFrame`, `AppCommand`/`AppFrame` carry no `Lane` byte —
//! the app-engine channel is a single logical stream, not split into causally-ordered vs.
//! best-effort lanes.

//#region 🔖️Version
/// @emoji 🔢️ The channel wire format's own version, pinned against the shared cross-language
/// fixture `channel-version.json` so a half-done bump fails a test instead of drifting silently.
/// Channel v12 (`📓️design-abi.md` §2 "`exchange` collapse") retires the `AppCommand::Hello` /
/// `AppFrame::Welcome` handshake entirely — lifecycle now arrives through the reactor ABI's
/// `Event::InstanceOpen`/`InstanceClose`, so this constant is no longer carried on the wire by any
/// frame; it exists purely as the drift guard the tests below assert against.
pub const CHANNEL_VERSION: u32 = 13;
//#endregion 🔖️Version

//#region 🔖️ChildPackEntry
/// @emoji 🧸️ One owned child's whole persisted envelope, as it travels between host and guest.
/// Composed children are their OWN envelopes with their own `ArtifactVcs` history, so a composing
/// document's `LoadDocument`/`Document` pair is not sufficient to save or restore it — its children
/// would exist only until the process ended. `AppCommand::LoadChildren`/`AppFrame::Children` carry
/// exactly these, keyed the way the parent's `ArtifactChild` handles name them.
#[derive(Clone, Debug, PartialEq)]
pub struct ChildPackEntry {
    pub slot: String,
    pub child_id: String,
    /// 🎯️ `ArtifactDialect` as its `<kind>@<standard>/<subset>` wire string — the guest needs it to
    /// pick the right `ChildStoreFactory`, and it is not recoverable from the pack bytes alone.
    pub dialect: String,
    /// 📦️ The child's full envelope pack (`encode_document_pack_bytes` framing: pack + spr).
    pub envelope_pack: Vec<u8>,
}
//#endregion 🔖️ChildPackEntry

//#region 🔖️PagedCommandIngress
pub const COMMAND_PAGE_MAXIMUM_BYTES: usize = 4_096;
pub const COMMAND_MAXIMUM_PAGES: usize = 64;
pub const COMMAND_MAXIMUM_BYTES: usize = COMMAND_PAGE_MAXIMUM_BYTES * COMMAND_MAXIMUM_PAGES;
pub const COMMAND_BATCH_MAXIMUM_ITEMS: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct FixedCommandPage {
    bytes: [u8; COMMAND_PAGE_MAXIMUM_BYTES],
    len: u16,
}

impl FixedCommandPage {
    pub fn try_from_array(bytes: [u8; COMMAND_PAGE_MAXIMUM_BYTES], len: u32) -> Result<Self, crate::Fault> {
        let len = usize::try_from(len).map_err(|_| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-length"), "command page length is not representable"))?;
        if len > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-length"), "command page length exceeds its fixed 4096-byte authority"));
        }
        if bytes[len..].iter().any(|byte| *byte != 0) {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-padding"), "command page carries nonzero bytes outside its declared authority"));
        }
        Ok(Self { bytes, len: len as u16 })
    }

    pub fn try_copy_from(bytes: &[u8]) -> Result<Self, crate::Fault> {
        if bytes.len() > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-length"), "command page length exceeds its fixed 4096-byte authority"));
        }
        let mut fixed = [0; COMMAND_PAGE_MAXIMUM_BYTES];
        fixed[..bytes.len()].copy_from_slice(bytes);
        Ok(Self { bytes: fixed, len: bytes.len() as u16 })
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
impl protocol::value::ToValue for FixedCommandPage {
    fn to_value(&self) -> protocol::value::DslValue {
        protocol::value::DslValue::Array(self.as_slice().iter().map(protocol::value::ToValue::to_value).collect())
    }
}

impl protocol::value::FromValue for FixedCommandPage {
    fn from_value(value: protocol::value::DslValue) -> Result<Self, protocol::value::ValueError> {
        let protocol::value::DslValue::Array(items) = value else {
            return Err(protocol::value::ValueError::new(format!("expected an array for FixedCommandPage, found {value:?}")));
        };
        if items.len() > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(protocol::value::ValueError::new("fixed command page exceeds 4096 bytes".to_string()));
        }
        let mut bytes = Vec::with_capacity(items.len());
        for (index, item) in items.into_iter().enumerate() {
            bytes.push(<u8 as protocol::value::FromValue>::from_value(item).map_err(|error| error.under(index))?);
        }
        FixedCommandPage::try_copy_from(&bytes).map_err(|fault| protocol::value::ValueError::new(fault.message))
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

#[derive(Debug, PartialEq)]
pub struct CommandPageSet {
    pages: std::collections::VecDeque<FixedCommandPage>,
    byte_len: usize,
    generic_shape_valid: bool,
    all_nonempty: bool,
}

impl CommandPageSet {
    pub fn try_new() -> Result<Self, crate::Fault> {
        let mut pages = std::collections::VecDeque::new();
        pages.try_reserve_exact(COMMAND_MAXIMUM_PAGES).map_err(|_| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-allocation"), "fixed command page authority could not reserve its exact 64 slots"))?;
        Ok(Self { pages, byte_len: 0, generic_shape_valid: true, all_nonempty: true })
    }

    pub fn try_push(&mut self, page: FixedCommandPage) -> Result<(), (crate::Fault, FixedCommandPage)> {
        if self.pages.len() == COMMAND_MAXIMUM_PAGES {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-count"), "command page authority is saturated"), page));
        }
        let Some(byte_len) = self.byte_len.checked_add(page.len()).filter(|total| *total <= COMMAND_MAXIMUM_BYTES) else {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-byte-cap"), "command exceeds its fixed 262144-byte authority"), page));
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
    pub fn try_from_pages(pages: CommandPageSet) -> Result<Self, (crate::Fault, CommandPageSet)> {
        if pages.is_empty() || pages.len() > COMMAND_MAXIMUM_PAGES {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-count"), "command requires 1..=64 admitted pages"), pages));
        }
        if !pages.generic_shape_valid {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-shape"), "command pages must be nonempty, at most 4096 bytes, and every nonterminal page must be full"), pages));
        }
        let Some(kind) = pages.pages.front().and_then(|page| page.as_slice().first()).copied() else {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-empty"), "command has no kind byte"), pages));
        };
        Ok(Self { pages: pages.pages, byte_len: pages.byte_len, kind, metadata: 0, item_count: 0 })
    }

    pub fn try_from_presence_pages(own_color: Option<u8>, pages: CommandPageSet, item_count: usize) -> Result<Self, (crate::Fault, CommandPageSet)> {
        if item_count > COMMAND_BATCH_MAXIMUM_ITEMS || pages.len() != item_count.max(1) {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-presence-item-cap"), "Presence command requires one exact page per peer and at most 64 peers"), pages));
        }
        if (item_count == 0 && pages.byte_len != 0) || (item_count != 0 && !pages.all_nonempty) {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-presence-page-shape"), "each Presence peer page must be nonempty and at most 4096 bytes"), pages));
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
        let Some(page_len) = self.pages.front().map(FixedCommandPage::len) else {
            return None;
        };
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

    pub fn read_byte(&mut self) -> Result<u8, crate::Fault> {
        let byte = self
            .command
            .front_page()
            .and_then(|page| page.as_slice().get(self.offset))
            .copied()
            .ok_or_else(|| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-decode-truncated"), "paged command ended inside a field"))?;
        self.offset += 1;
        if self.command.front_page().is_some_and(|page| self.offset == page.len()) {
            let _ = self.command.release_front_page(COMMAND_PAGE_MAXIMUM_BYTES).expect("fully consumed fixed page is releasable");
            self.offset = 0;
        }
        Ok(byte)
    }

    pub fn read_varint(&mut self) -> Result<u64, crate::Fault> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.read_byte()?;
            if shift == 63 && byte > 1 {
                return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-decode-varint"), "paged command varint overflowed u64"));
            }
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-decode-varint"), "paged command varint exceeds ten bytes"))
    }

    pub fn read_bounded_bytes(&mut self, maximum: usize) -> Result<Vec<u8>, crate::Fault> {
        let length = usize::try_from(self.read_varint()?).map_err(|_| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-field-length"), "paged command field length is not representable"))?;
        if length > maximum {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-field-cap"), "paged command field exceeds its exact bounded decode authority"));
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(length).map_err(|_| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-field-allocation"), "paged command field could not reserve its exact bounded authority"))?;
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
    pub fn try_new() -> Result<Self, crate::Fault> {
        let mut commands = std::collections::VecDeque::new();
        commands
            .try_reserve_exact(COMMAND_BATCH_MAXIMUM_ITEMS)
            .map_err(|_| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-batch-allocation"), "fixed command batch authority could not reserve its exact 64 slots"))?;
        let mut page_storage = std::collections::VecDeque::new();
        page_storage
            .try_reserve_exact(COMMAND_MAXIMUM_PAGES)
            .map_err(|_| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-batch-page-allocation"), "fixed command batch authority could not reserve its exact 64 page slots"))?;
        Ok(Self { commands, page_storage, pages: 0, bytes: 0 })
    }

    pub fn try_push(&mut self, command: CommandEnvelope) -> Result<(), (crate::Fault, CommandEnvelope)> {
        if self.commands.len() == COMMAND_BATCH_MAXIMUM_ITEMS {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-batch-cap"), "command batch exceeds its exact 64-item authority"), command));
        }
        let pages = match self.pages.checked_add(command.command.page_len()) {
            Some(pages) if pages <= COMMAND_MAXIMUM_PAGES => pages,
            _ => return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-batch-page-cap"), "command batch exceeds its aggregate 64-page authority"), command)),
        };
        let bytes = match self.bytes.checked_add(command.command.byte_len()) {
            Some(bytes) if bytes <= COMMAND_MAXIMUM_BYTES => bytes,
            _ => return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-batch-byte-cap"), "command batch exceeds its aggregate 262144-byte authority"), command)),
        };
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

impl<const CAPACITY: usize> RejectedCommandBuildRegistry<CAPACITY> {
    pub fn new() -> Self {
        assert!(CAPACITY > 0);
        Self { slots: std::array::from_fn(|_| None), close_index: 0, occupied: 0 }
    }

    pub fn can_insert(&self, key: u64) -> bool {
        self.slots[key as usize % CAPACITY].is_none()
    }

    pub fn try_insert(&mut self, key: u64, owner: RejectedCommandBuild) -> Result<(), (crate::Fault, RejectedCommandBuild)> {
        let index = key as usize % CAPACITY;
        if self.slots[index].is_some() {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-build-close-capacity"), "fixed rejected command-build close registry is occupied or collided"), owner));
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
    pub fn try_new(generation: u64, commands: CommandEnvelopeSet) -> Result<Self, (crate::Fault, CommandEnvelopeSet)> {
        if commands.commands.is_empty() {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-batch-cap"), "command batch requires at least one exact command owner"), commands));
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

impl<const CAPACITY: usize> CommandDriverRegistry<CAPACITY> {
    pub fn new() -> Self {
        assert!(CAPACITY > 0 && CAPACITY <= usize::from(u16::MAX));
        Self { slots: std::array::from_fn(|_| None), close_head: None, close_tail: None, occupied: 0 }
    }

    pub fn try_insert(&mut self, key: u64, generation: u64, driver: CommandBatchDriver) -> Result<(), (crate::Fault, CommandBatchDriver)> {
        if !self.can_insert(key) {
            return Err((crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-driver-capacity"), "fixed retained command-driver slot is occupied or collided"), driver));
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

    pub fn with_driver_mut<R>(&mut self, key: u64, generation: u64, f: impl FnOnce(&mut CommandBatchDriver) -> R) -> Result<R, crate::Fault> {
        let slot = self.slot_mut(key, generation)?;
        if slot.state != CommandDriverRetentionState::Active {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-driver-not-active"), "retained command driver is suspended or closing"));
        }
        Ok(f(&mut slot.driver))
    }

    pub fn prepare_suspend(&mut self, key: u64, generation: u64) -> Result<(), crate::Fault> {
        let index = self.index_of(key, generation)?;
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Active {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-driver-suspend-state"), "retained command driver is not active before suspension"));
        }
        self.link_close(index);
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Suspended;
        Ok(())
    }

    pub fn resume(&mut self, key: u64, generation: u64) -> Result<(), crate::Fault> {
        let index = self.index_of(key, generation)?;
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Suspended {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-driver-resume-state"), "retained command driver is not suspended before resume"));
        }
        self.unlink_close(index);
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Active;
        Ok(())
    }

    pub fn begin_close(&mut self, key: u64, generation: u64) -> Result<(), crate::Fault> {
        let index = self.index_of(key, generation)?;
        let state = self.slots[index].as_ref().expect("retained command slot exists").state;
        if state == CommandDriverRetentionState::Active {
            self.link_close(index);
        }
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Closing;
        Ok(())
    }

    pub fn begin_close_key(&mut self, key: u64) -> Result<u64, crate::Fault> {
        let index = key as usize % CAPACITY;
        let generation = match self.slots[index].as_ref() {
            Some(slot) if slot.key == key => slot.generation,
            _ => return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-driver-stale"), "retained command driver key is stale")),
        };
        self.begin_close(key, generation)?;
        Ok(generation)
    }

    pub fn remove_terminal(&mut self, key: u64, generation: u64) -> Result<(), crate::Fault> {
        let index = self.index_of(key, generation)?;
        if !self.slots[index].as_ref().expect("retained command slot exists").driver.terminal_is_empty() {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-driver-nonterminal-remove"), "retained command driver cannot be removed before terminal empty"));
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

    fn slot_mut(&mut self, key: u64, generation: u64) -> Result<&mut CommandDriverRetentionSlot, crate::Fault> {
        let index = self.index_of(key, generation)?;
        Ok(self.slots[index].as_mut().expect("retained command slot exists"))
    }

    fn index_of(&self, key: u64, generation: u64) -> Result<usize, crate::Fault> {
        let index = key as usize % CAPACITY;
        match self.slots[index].as_ref() {
            Some(slot) if slot.key == key && slot.generation == generation => Ok(index),
            _ => Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-driver-stale"), "retained command driver identity or generation is stale")),
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

    pub fn next_page(&mut self) -> Result<Option<(CommandPageCursor, FixedCommandPage)>, crate::Fault> {
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
        let bytes = self.batch.pages.front().ok_or_else(|| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-owner-empty"), "nonterminal command owner has no page"))?.clone();
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

    pub fn observe(&mut self, status: &CommandIngressStatus, maximum_release_bytes: usize) -> Result<CommandBatchProgress, crate::Fault> {
        let cursor = match status {
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
            CommandIngressStatus::PageAccepted(cursor) | CommandIngressStatus::Backpressure(cursor) | CommandIngressStatus::CommandPending(cursor) | CommandIngressStatus::CommandComplete(cursor) => cursor,
            CommandIngressStatus::Fault { cursor, .. } => cursor,
        };
        self.validate_cursor(cursor)?;
        match status {
            CommandIngressStatus::Backpressure(_) => Ok(CommandBatchProgress::PageReady),
            CommandIngressStatus::PageAccepted(_) => {
                let Some(command) = self.batch.commands.front_mut() else {
                    return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-owner-missing"), "accepted page has no exact host owner"));
                };
                let page_len =
                    self.batch.pages.front().map(FixedCommandPage::len).ok_or_else(|| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-owner-missing"), "accepted page has no exact retained batch page"))?;
                if page_len > maximum_release_bytes {
                    return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-release-budget"), "accepted page exceeds its exact release grant"));
                }
                let page = self.batch.pages.pop_front().expect("accepted retained batch page was present");
                let released = page.len();
                self.batch.bytes -= released;
                drop(page);
                command.remaining_pages = command
                    .remaining_pages
                    .checked_sub(1)
                    .ok_or_else(|| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-page-underflow"), "accepted page arrived after the retained command page owner was empty"))?;
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
                    return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-owner-missing"), "terminal command has no exact host owner"));
                };
                if command.remaining_pages != 0 {
                    return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-terminal-pages"), "terminal acknowledgement arrived before every exact page was released"));
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

    fn validate_cursor(&self, cursor: &CommandPageCursor) -> Result<(), crate::Fault> {
        let Some(command) = self.batch.commands.front() else {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-owner-missing"), "ingress status has no exact host owner"));
        };
        if cursor.owner != self.owner
            || cursor.generation != self.batch.generation
            || cursor.command_index != self.command_index
            || cursor.instance != command.instance
            || cursor.seq != command.seq
            || cursor.kind != self.admitted_kind
            || cursor.page_index != self.page_index
            || cursor.page_count != self.admitted_page_count
            || cursor.item_count != command.item_count
            || cursor.metadata != command.metadata
        {
            return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-cursor-mismatch"), "ingress status does not identify the exact retained host owner"));
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

//#region 🔖️PresenceRosterWire
pub const PRESENCE_ROSTER_MAXIMUM_ITEMS: usize = 64;
pub const PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES: usize = 4_096;
pub const PRESENCE_ROSTER_MAXIMUM_BYTES: usize = PRESENCE_ROSTER_MAXIMUM_ITEMS * PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES;

#[derive(Debug, PartialEq)]
pub struct PresenceRosterWire {
    entries: [Option<Box<[u8]>>; PRESENCE_ROSTER_MAXIMUM_ITEMS],
    len: usize,
    bytes: usize,
}

#[derive(Debug)]
pub struct PresenceRosterPushRejected {
    pub entry: Vec<u8>,
    pub reason: &'static str,
}

impl PresenceRosterWire {
    pub fn empty() -> Self {
        Self { entries: std::array::from_fn(|_| None), len: 0, bytes: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn byte_len(&self) -> usize {
        self.bytes
    }

    pub fn try_push(&mut self, entry: Vec<u8>) -> Result<(), PresenceRosterPushRejected> {
        if self.len == PRESENCE_ROSTER_MAXIMUM_ITEMS {
            return Err(PresenceRosterPushRejected { entry, reason: "presence roster exceeds its fixed item authority" });
        }
        if entry.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES || self.bytes.checked_add(entry.len()).is_none_or(|bytes| bytes > PRESENCE_ROSTER_MAXIMUM_BYTES) {
            return Err(PresenceRosterPushRejected { entry, reason: "presence roster exceeds its fixed byte authority" });
        }
        let bytes = entry.len();
        self.entries[self.len] = Some(entry.into_boxed_slice());
        self.len += 1;
        self.bytes += bytes;
        Ok(())
    }

    pub fn pop_back(&mut self) -> Option<Box<[u8]>> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let entry = self.entries[self.len].take().expect("occupied presence roster slot");
        self.bytes -= entry.len();
        Some(entry)
    }

    pub fn iter(&self) -> impl Iterator<Item = &[u8]> {
        self.entries[..self.len].iter().filter_map(|entry| entry.as_deref())
    }

    pub fn last(&self) -> Option<&[u8]> {
        self.len.checked_sub(1).and_then(|index| self.entries[index].as_deref())
    }
}

impl Default for PresenceRosterWire {
    fn default() -> Self {
        Self::empty()
    }
}
//#endregion 🔖️PresenceRosterWire

//#region 🔖️PresenceCommandCursor
pub const PRESENCE_COMMAND_MAXIMUM_BYTES: usize = PRESENCE_ROSTER_MAXIMUM_BYTES + PRESENCE_ROSTER_MAXIMUM_ITEMS * 10 + 32;

pub struct PresenceCommandCursor {
    page: Option<FixedCommandPage>,
    remaining: usize,
    seq: u64,
    own_color: Option<u8>,
    next_page: u32,
}

impl PresenceCommandCursor {
    pub fn retain_rejected(page: FixedCommandPage, seq: u64) -> Self {
        Self { page: Some(page), remaining: 0, seq, own_color: None, next_page: 1 }
    }

    pub fn admit_page(seq: u64, own_color: Option<u8>, item_count: u32, page: FixedCommandPage) -> Result<Self, (crate::os_spr::ProtocolError, FixedCommandPage)> {
        if item_count as usize > PRESENCE_ROSTER_MAXIMUM_ITEMS || page.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES || (item_count != 0 && page.is_empty()) || (item_count == 0 && !page.is_empty()) {
            return Err((malformed("channel presence page", 0, "Presence page violates its exact item or 4096-byte authority"), page));
        }
        Ok(Self { page: Some(page), remaining: item_count as usize, seq, own_color, next_page: 1 })
    }

    pub fn push_page(&mut self, page_index: u32, page: FixedCommandPage) -> Result<(), (crate::os_spr::ProtocolError, FixedCommandPage)> {
        if self.page.is_some() || self.remaining == 0 || page_index != self.next_page || page.is_empty() || page.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES {
            return Err((malformed("channel presence page", page_index as u64, "page is out of order, saturated, empty, or oversized"), page));
        }
        self.page = Some(page);
        self.next_page = self.next_page.saturating_add(1);
        Ok(())
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    pub fn own_color(&self) -> Option<u8> {
        self.own_color
    }

    pub fn take_next(&mut self) -> Result<Option<FixedCommandPage>, crate::os_spr::ProtocolError> {
        if self.remaining == 0 {
            return Ok(None);
        }
        let entry = self.page.take().ok_or_else(|| malformed("channel presence command owner", self.next_page as u64, "next page has not been admitted"))?;
        self.remaining -= 1;
        Ok(Some(entry))
    }

    pub fn next_len(&self) -> Result<Option<usize>, crate::os_spr::ProtocolError> {
        if self.remaining == 0 {
            return Ok(None);
        }
        Ok(self.page.as_ref().map(FixedCommandPage::len))
    }

    pub fn close_release(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(page_len) = self.page.as_ref().map(FixedCommandPage::len) else {
            return (self.remaining == 0, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let page = self.page.take().expect("presence page was present");
        let released = page.len();
        drop(page);
        self.remaining = 0;
        (true, released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.page.is_none() && self.remaining == 0
    }

    pub fn waiting_for_page(&self) -> bool {
        self.page.is_none() && self.remaining != 0
    }
}
//#endregion 🔖️PresenceCommandCursor

//#region 🔖️PagedAppCommandDecode
const APP_COMMAND_FIELD_MAXIMUM_BYTES: usize = COMMAND_MAXIMUM_BYTES;

#[derive(Debug)]
enum PagedAppCommandDecodeState {
    Header,
    ConfigCommand { seq: u64 },
    CommandPayload { seq: u64 },
    CommandView { seq: u64, command: Option<Vec<u8>> },
    CommandText { seq: u64 },
    ContextMenu { seq: u64 },
    ArtifactCommand { seq: u64 },
    LoadDocumentPack { seq: u64 },
    LoadDocumentSpr { seq: u64, pack: Option<Vec<u8>> },
    ReadDocument { seq: u64 },
    LoadConfigPack { seq: u64 },
    LoadConfigSpr { seq: u64, pack: Option<Vec<u8>> },
    ReadConfig { seq: u64 },
    ReadChildren { seq: u64 },
    ReadHistory { seq: u64 },
    ReadConflicts { seq: u64 },
    LocalInteractionQuery { seq: u64 },
    RejectedFields { first: Option<Vec<u8>>, second: Option<Vec<u8>> },
    Terminal,
    Faulted,
}

#[derive(Debug)]
pub struct PagedAppCommandDecodeCursor {
    reader: PagedCommandReader,
    state: PagedAppCommandDecodeState,
}

#[derive(Debug)]
pub struct DecodedAppCommandOwner {
    command: Option<AppCommand>,
    close_stage: u8,
}

impl DecodedAppCommandOwner {
    pub fn new(command: AppCommand) -> Self {
        Self { command: Some(command), close_stage: 0 }
    }

    pub fn take_for_dispatch(&mut self) -> Option<AppCommand> {
        self.command.take()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let Some(command) = self.command.as_mut() else {
            return (true, 0, 0);
        };
        let field = match (self.close_stage, command) {
            (0, AppCommand::ConfigCommand { command, .. }) | (0, AppCommand::ContextMenu { request: command, .. }) | (0, AppCommand::ArtifactCommand { command, .. }) | (0, AppCommand::Command { command, .. }) => Some(std::mem::take(command)),
            (0, AppCommand::CommandText { line, .. }) => Some(std::mem::take(line).into_bytes()),
            (0, AppCommand::LoadDocument { pack, .. }) | (0, AppCommand::LoadConfig { pack, .. }) => Some(std::mem::take(pack)),
            (1, AppCommand::Command { view_state, .. }) => Some(std::mem::take(view_state)),
            (1, AppCommand::LoadDocument { spr, .. }) | (1, AppCommand::LoadConfig { spr, .. }) => Some(std::mem::take(spr)),
            _ => None,
        };
        if let Some(field) = field {
            if field.len() > maximum_bytes {
                match (self.close_stage, self.command.as_mut().expect("decoded command owner was present")) {
                    (0, AppCommand::ConfigCommand { command, .. }) | (0, AppCommand::ContextMenu { request: command, .. }) | (0, AppCommand::ArtifactCommand { command, .. }) | (0, AppCommand::Command { command, .. }) => *command = field,
                    (0, AppCommand::CommandText { line, .. }) => *line = String::from_utf8(field).expect("decoded command text remains valid UTF-8"),
                    (0, AppCommand::LoadDocument { pack, .. }) | (0, AppCommand::LoadConfig { pack, .. }) => *pack = field,
                    (1, AppCommand::Command { view_state, .. }) => *view_state = field,
                    (1, AppCommand::LoadDocument { spr, .. }) | (1, AppCommand::LoadConfig { spr, .. }) => *spr = field,
                    _ => unreachable!("decoded command close field has an exact restoration target"),
                }
                return (false, 0, 0);
            }
            let released = field.len();
            drop(field);
            self.close_stage = self.close_stage.saturating_add(1);
            return (false, 1, released);
        }
        let command = self.command.take().expect("decoded command shell was present");
        drop(command);
        (true, 1, 0)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.command.is_none()
    }
}

impl PagedAppCommandDecodeCursor {
    pub fn new(command: PagedCommand) -> Self {
        Self { reader: PagedCommandReader::new(command), state: PagedAppCommandDecodeState::Header }
    }

    pub fn kind(&self) -> u8 {
        self.reader.kind()
    }

    pub fn step(&mut self) -> Result<Option<AppCommand>, crate::Fault> {
        let state = std::mem::replace(&mut self.state, PagedAppCommandDecodeState::Faulted);
        let outcome = match state {
            PagedAppCommandDecodeState::Header => {
                let tag = self.reader.read_byte()?;
                let seq = self.reader.read_varint()?;
                self.state = match tag {
                    0 => PagedAppCommandDecodeState::ConfigCommand { seq },
                    1 => PagedAppCommandDecodeState::CommandPayload { seq },
                    2 => PagedAppCommandDecodeState::CommandText { seq },
                    3 => PagedAppCommandDecodeState::ContextMenu { seq },
                    4 => PagedAppCommandDecodeState::ArtifactCommand { seq },
                    6 => PagedAppCommandDecodeState::LoadDocumentPack { seq },
                    7 => PagedAppCommandDecodeState::ReadDocument { seq },
                    8 => PagedAppCommandDecodeState::LoadConfigPack { seq },
                    9 => PagedAppCommandDecodeState::ReadConfig { seq },
                    15 => PagedAppCommandDecodeState::ReadChildren { seq },
                    16 => PagedAppCommandDecodeState::ReadHistory { seq },
                    27 => PagedAppCommandDecodeState::ReadConflicts { seq },
                    29 => PagedAppCommandDecodeState::LocalInteractionQuery { seq },
                    _ => {
                        return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-route-state-machine-required"), "this AppCommand kind requires its route-specific retained decoder before admission"));
                    }
                };
                None
            }
            PagedAppCommandDecodeState::ConfigCommand { seq } => {
                let command = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                Some(AppCommand::ConfigCommand { seq, command })
            }
            PagedAppCommandDecodeState::CommandPayload { seq } => {
                let command = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                self.state = PagedAppCommandDecodeState::CommandView { seq, command: Some(command) };
                None
            }
            PagedAppCommandDecodeState::CommandView { seq, mut command } => {
                let view_state = match self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES) {
                    Ok(view_state) => view_state,
                    Err(fault) => {
                        self.state = PagedAppCommandDecodeState::CommandView { seq, command };
                        return Err(fault);
                    }
                };
                Some(AppCommand::Command { seq, command: command.take().expect("retained command payload"), view_state })
            }
            PagedAppCommandDecodeState::CommandText { seq } => {
                let bytes = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                let line = match String::from_utf8(bytes) {
                    Ok(line) => line,
                    Err(error) => {
                        self.state = PagedAppCommandDecodeState::RejectedFields { first: Some(error.into_bytes()), second: None };
                        return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-field-utf8"), "paged command text is not valid UTF-8"));
                    }
                };
                Some(AppCommand::CommandText { seq, line })
            }
            PagedAppCommandDecodeState::ContextMenu { seq } => {
                let request = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                Some(AppCommand::ContextMenu { seq, request })
            }
            PagedAppCommandDecodeState::ArtifactCommand { seq } => {
                let command = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                Some(AppCommand::ArtifactCommand { seq, command })
            }
            PagedAppCommandDecodeState::LoadDocumentPack { seq } => {
                let pack = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                self.state = PagedAppCommandDecodeState::LoadDocumentSpr { seq, pack: Some(pack) };
                None
            }
            PagedAppCommandDecodeState::LoadDocumentSpr { seq, mut pack } => {
                let spr = match self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES) {
                    Ok(spr) => spr,
                    Err(fault) => {
                        self.state = PagedAppCommandDecodeState::LoadDocumentSpr { seq, pack };
                        return Err(fault);
                    }
                };
                Some(AppCommand::LoadDocument { seq, pack: pack.take().expect("retained document pack"), spr })
            }
            PagedAppCommandDecodeState::ReadDocument { seq } => Some(AppCommand::ReadDocument { seq }),
            PagedAppCommandDecodeState::LoadConfigPack { seq } => {
                let pack = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                self.state = PagedAppCommandDecodeState::LoadConfigSpr { seq, pack: Some(pack) };
                None
            }
            PagedAppCommandDecodeState::LoadConfigSpr { seq, mut pack } => {
                let spr = match self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES) {
                    Ok(spr) => spr,
                    Err(fault) => {
                        self.state = PagedAppCommandDecodeState::LoadConfigSpr { seq, pack };
                        return Err(fault);
                    }
                };
                Some(AppCommand::LoadConfig { seq, pack: pack.take().expect("retained config pack"), spr })
            }
            PagedAppCommandDecodeState::ReadConfig { seq } => Some(AppCommand::ReadConfig { seq }),
            PagedAppCommandDecodeState::ReadChildren { seq } => Some(AppCommand::ReadChildren { seq }),
            PagedAppCommandDecodeState::ReadHistory { seq } => Some(AppCommand::ReadHistory { seq }),
            PagedAppCommandDecodeState::ReadConflicts { seq } => Some(AppCommand::ReadConflicts { seq }),
            PagedAppCommandDecodeState::LocalInteractionQuery { seq } => {
                let bytes = self.reader.read_bounded_bytes(142)?;
                let command = protocol::decode_local_interaction_query_command(&bytes).map_err(|reason| crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("local-interaction.command-wire"), reason))?;
                Some(AppCommand::LocalInteractionQuery { seq, command })
            },
            PagedAppCommandDecodeState::RejectedFields { first, second } => {
                self.state = PagedAppCommandDecodeState::RejectedFields { first, second };
                return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-decode-closing"), "rejected paged command must be closed before it can be stepped again"));
            }
            PagedAppCommandDecodeState::Terminal | PagedAppCommandDecodeState::Faulted => {
                return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-decode-terminal"), "paged command decoder was stepped after terminal"));
            }
        };
        if let Some(command) = outcome {
            if !self.reader.terminal_is_empty() {
                let (first, second) = match command {
                    AppCommand::ConfigCommand { command, .. } | AppCommand::ContextMenu { request: command, .. } | AppCommand::ArtifactCommand { command, .. } => (Some(command), None),
                    AppCommand::Command { command, view_state, .. } => (Some(command), Some(view_state)),
                    AppCommand::CommandText { line, .. } => (Some(line.into_bytes()), None),
                    AppCommand::LoadDocument { pack, spr, .. } | AppCommand::LoadConfig { pack, spr, .. } => (Some(pack), Some(spr)),
                    AppCommand::ReadDocument { .. } | AppCommand::ReadConfig { .. } | AppCommand::ReadChildren { .. } | AppCommand::ReadHistory { .. } | AppCommand::ReadConflicts { .. } | AppCommand::LocalInteractionQuery { .. } => (None, None),
                    AppCommand::Presence { .. } => unreachable!("Presence is never decoded by the generic paged cursor"),
                    _ => unreachable!("route-specific AppCommand is never decoded by the generic paged cursor"),
                };
                self.state = PagedAppCommandDecodeState::RejectedFields { first, second };
                return Err(crate::Fault::new(crate::FaultOrigin::Framework, crate::FaultCode::new("plugin.command-decode-trailing"), "paged command carries trailing bytes after its terminal field"));
            }
            self.state = PagedAppCommandDecodeState::Terminal;
            Ok(Some(command))
        } else {
            Ok(None)
        }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        if let PagedAppCommandDecodeState::CommandView { command, .. } = &mut self.state {
            if let Some(bytes) = command.as_ref() {
                if bytes.len() > maximum_bytes {
                    return (false, 0);
                }
                let bytes = command.take().expect("retained command payload");
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        if let Some(retained) = match &mut self.state {
            PagedAppCommandDecodeState::LoadDocumentSpr { pack, .. } | PagedAppCommandDecodeState::LoadConfigSpr { pack, .. } => Some(pack),
            _ => None,
        } {
            if let Some(bytes) = retained.as_ref() {
                if bytes.len() > maximum_bytes {
                    return (false, 0);
                }
                let bytes = retained.take().expect("retained pack was present");
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        if let PagedAppCommandDecodeState::RejectedFields { first, second } = &mut self.state {
            if let Some(bytes) = first.as_ref() {
                if bytes.len() > maximum_bytes {
                    return (false, 0);
                }
                let bytes = first.take().expect("first rejected field was present");
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
            if let Some(bytes) = second.as_ref() {
                if bytes.len() > maximum_bytes {
                    return (false, 0);
                }
                let bytes = second.take().expect("second rejected field was present");
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        let (empty, released) = self.reader.close_step(maximum_bytes);
        if empty {
            self.state = PagedAppCommandDecodeState::Terminal;
        }
        (empty, released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(self.state, PagedAppCommandDecodeState::Terminal) && self.reader.terminal_is_empty()
    }
}
//#endregion 🔖️PagedAppCommandDecode

//#region 🔖️AppCommand
/// @emoji 📨️ One frame a client (UI or headless runner) sends to the app engine.
#[derive(Debug, PartialEq)]
pub enum AppCommand {
    ConfigCommand {
        seq: u64,
        command: Vec<u8>,
    },
    Command {
        seq: u64,
        command: Vec<u8>,
        /// 🗣️ Packed `ViewModel` (see `crate::os_store::pack_rt`) the client wants this command evaluated against.
        view_state: Vec<u8>,
    },
    CommandText {
        seq: u64,
        line: String,
    },
    ContextMenu {
        seq: u64,
        request: Vec<u8>,
    },
    ArtifactCommand {
        seq: u64,
        command: Vec<u8>,
    },
    ApplyEnvelopes {
        seq: u64,
        envelopes: Vec<crate::os_spr::causal::MutationEnvelope>,
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
    /// 🧾 Host-authoritative command: document/config/draft packs travel with the command; guest
    /// returns `AppFrame::Emit` ops only (host applies). CHANNEL_VERSION 5 wire addition.
    PureCommand {
        seq: u64,
        command: Vec<u8>,
        document: Vec<u8>,
        document_spr: Vec<u8>,
        config: Vec<u8>,
        config_spr: Vec<u8>,
        draft: Vec<u8>,
        draft_spr: Vec<u8>,
    },
    /// 🧸️ Restores a composing document's owned children into the engine, each as its own live
    /// store. Sent after `LoadDocument` (the parent must exist before its children can be adopted).
    /// CHANNEL_VERSION 6 wire addition.
    LoadChildren {
        seq: u64,
        entries: Vec<ChildPackEntry>,
    },
    /// 🧸️ Asks the engine for every owned child's current envelope, for persistence — the child-side
    /// counterpart of `ReadDocument`. CHANNEL_VERSION 6 wire addition.
    ReadChildren {
        seq: u64,
    },
    /// 🧾️ Reads a complete history projection after initial connection or cursor resynchronization.
    ReadHistory {
        seq: u64,
    },
    /// 🤝️ Phase-1 prepare for one transaction member — flat fields carry EITHER the owner-mutation
    /// form (`mutation_id`+`payload` set, `prepared_ops` empty) OR the pre-planned form
    /// (`prepared_ops`+`label`+`origin` set, `mutation_id` empty); see contract-freeze.md §2 of
    /// `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/`.
    /// CHANNEL_VERSION 9 wire addition.
    TransactionPrepare {
        seq: u64,
        txn_id: String,
        mutation_id: String,
        payload: Vec<u8>,
        prepared_ops: Vec<Vec<u8>>,
        label: String,
        origin: Vec<u8>,
    },
    /// ✅️ Phase-2 commit for one transaction member. CHANNEL_VERSION 9 wire addition.
    TransactionCommit {
        seq: u64,
        txn_id: String,
    },
    /// ↩️ Aborts a not-yet-committed transaction member. CHANNEL_VERSION 9 wire addition.
    TransactionRollback {
        seq: u64,
        txn_id: String,
    },
    /// ⏪️ Fans a group undo out to one already-committed transaction member. CHANNEL_VERSION 9 wire addition.
    TransactionUndo {
        seq: u64,
        group_id: String,
    },
    /// ⏩️ Fans a group redo out to one already-committed transaction member. CHANNEL_VERSION 9 wire addition.
    TransactionRedo {
        seq: u64,
        group_id: String,
    },
    /// 📂️ Opens an artifact in its resolved (or explicitly named) viewer/editor surface — see
    /// contract-freeze.md §3 of `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`.
    /// Empty `plugin_id`/`app_id` means "resolve via `OpeningResolver`". CHANNEL_VERSION 10 wire addition.
    OpenArtifact {
        seq: u64,
        artifact_ref: String,
        role: u8,
        plugin_id: String,
        app_id: String,
    },
    /// 🎚️ Pins a viewer/editor default for one `(artifact_kind, standard, subset, role)` coordinate,
    /// persisted event-sourced in the OS `🎚️config` opening-preferences facet. CHANNEL_VERSION 10 wire addition.
    SetDefaultApp {
        seq: u64,
        artifact_kind: String,
        standard: String,
        subset: String,
        role: u8,
        plugin_id: String,
        app_id: String,
    },
    /// 🎚️ Clears a previously pinned default, falling back to the `OpeningResolver`'s owner/router
    /// order. CHANNEL_VERSION 10 wire addition.
    ClearDefaultApp {
        seq: u64,
        artifact_kind: String,
        standard: String,
        subset: String,
        role: u8,
    },
    /// ⚖️ Pins this connection's local/authority `MergePolicy` (`0`=`LaissezFaire`, `1`=`Normal`,
    /// `2`=`Vigilant`) — never carried on a `MutationEnvelope`/`BackboneMessage`, see
    /// contract-freeze.md §C3/C8 of `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
    /// CHANNEL_VERSION 11 wire addition.
    SetMergePolicy {
        seq: u64,
        policy: u8,
    },
    /// ⚔️ Resolves one open `Conflict` (`0`=`Accept`, `1`=`Discard`) — see contract-freeze.md §C5/C6.
    /// CHANNEL_VERSION 11 wire addition.
    ResolveConflict {
        seq: u64,
        conflict_id: String,
        resolution: u8,
    },
    /// ⚔️ Reads every open `Conflict` for the current artifact. CHANNEL_VERSION 11 wire addition.
    ReadConflicts {
        seq: u64,
    },
    /// 👥️ Pushes the document-wide presence roster into this app instance — the ONLY plugin ingress
    /// for peers (contract-freeze §C7.6 of ticket
    /// `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`).
    /// `own_color` is this actor's hub-assigned palette index (`None` for a folder-only session with
    /// no hub); `peers` are `encode_presence_peer` blobs, the whole roster with the wrapper's own
    /// actor already dropped. Reply is a plain `AppFrame::Done`. CHANNEL_VERSION 12 wire addition.
    Presence {
        seq: u64,
        own_color: Option<u8>,
        peers: PresenceRosterWire,
    },
    /// 🏠️ Fixed request/token commands; page data travels only on the reply lane.
    LocalInteractionQuery { seq: u64, command: protocol::LocalInteractionQueryCommand },
}
//#endregion 🔖️AppCommand

//#region 🔖️AppFrame
/// @emoji 📬️ One frame the app engine sends to its client.
#[derive(Clone, Debug, PartialEq)]
pub enum AppFrame {
    Done {
        in_reply_to: u64,
    },
    /// 🧾 `messages` (CHANNEL_VERSION 11 trailing addition) is one packed `DispatchReport` for this
    /// dispatch — see contract-freeze.md §C8.
    Invocation {
        in_reply_to: u64,
        output: Vec<u8>,
        diagnostics: Vec<u8>,
        ui_scope: Vec<u8>,
        history_patch: Vec<u8>,
        messages: Vec<u8>,
    },
    DocumentChanged {
        envelopes: Vec<crate::os_spr::causal::MutationEnvelope>,
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
        envelopes: Vec<crate::os_spr::causal::MutationEnvelope>,
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
    /// 🧾 `report` (CHANNEL_VERSION 11 trailing addition) is one packed `DispatchReport` of the
    /// rejected dispatch, accompanying a `Fault.code == "mutation.rejected"` — see
    /// contract-freeze.md §C8/C9.
    Error {
        in_reply_to: Option<u64>,
        fault: Vec<u8>,
        report: Vec<u8>,
    },
    /// 📤️ Guest Emit bytes for host-applied store authority (document/config/draft op packs).
    Emit {
        in_reply_to: u64,
        document_ops: Vec<u8>,
        config_ops: Vec<u8>,
        draft_ops: Vec<u8>,
        output: Vec<u8>,
        diagnostics: Vec<u8>,
    },
    /// 📝️ Draft-lane pack snapshot (volatile; never enters a Change/Checkpoint).
    Draft {
        in_reply_to: u64,
        pack: Vec<u8>,
        spr: Vec<u8>,
        ops: String,
    },
    /// 🧸️ Every owned child's current envelope — the reply to `ReadChildren`, and also emitted
    /// unsolicited (`in_reply_to` of the originating command) after a composite gesture creates new
    /// children, so the host learns about a genesis child without having to poll for it.
    /// CHANNEL_VERSION 6 wire addition.
    Children {
        in_reply_to: u64,
        entries: Vec<ChildPackEntry>,
    },
    /// 👥️ Typed guest ephemeral-lane snapshot. Presence is the app-defined `ArtifactPack` payload;
    /// generations let hosts skip unchanged renderer work while transient remains local-only.
    /// `interaction` (trailing field, CHANNEL_VERSION 12 wire addition, contract-freeze §C7.6) is the
    /// output of `encode_presence_interaction` over the app's own declared broadcast domains — empty
    /// bytes when the app declares no interaction domains or nothing is selected/hovered right now.
    Ephemeral {
        presence: Vec<u8>,
        presence_generation: u64,
        transient_generation: u64,
        interaction: Vec<u8>,
    },
    /// 🧾️ Full history patch for initial host projection and gap recovery.
    HistorySnapshot {
        in_reply_to: u64,
        history_patch: Vec<u8>,
    },
    /// 📣️ A guest's dispatch touched a foreign artifact — the host mints `txn_id`, resolves each
    /// opaque `ForeignStep` in `foreign` (one `store::pack_rt::encode_wire_value`-encoded serde
    /// form per element; not decoded at this layer), and drives the transaction protocol (contract
    /// freeze §5). CHANNEL_VERSION 9 wire addition.
    TransactionProposal {
        in_reply_to: u64,
        proposal_id: String,
        local_ops: Vec<Vec<u8>>,
        description: String,
        coalesce_key: String,
        foreign: Vec<Vec<u8>>,
    },
    /// 🤝️ Phase-1 reply — empty `rejection` means the member is prepared. CHANNEL_VERSION 9 wire addition.
    TransactionPrepared {
        txn_id: String,
        foreign: Vec<Vec<u8>>,
        rejection: Vec<u8>,
    },
    /// ✅️ Phase-2 commit succeeded for a member. CHANNEL_VERSION 9 wire addition.
    TransactionCommitted {
        txn_id: String,
        edit_id: String,
    },
    /// ↩️ A member rolled back its not-yet-committed transaction. CHANNEL_VERSION 9 wire addition.
    TransactionRolledBack {
        txn_id: String,
    },
    /// ⚔️ Pushed unsolicited (next to `DocumentChanged`) after every ingest: one packed `MergeReport`
    /// describing how the batch was resolved. CHANNEL_VERSION 11 wire addition.
    MergeReport {
        in_reply_to: Option<u64>,
        report: Vec<u8>,
    },
    /// ⚔️ Pushed unsolicited (next to `DocumentChanged`) after every ingest, and the reply to
    /// `AppCommand::ReadConflicts`: one packed `Vec<Conflict>`. CHANNEL_VERSION 11 wire addition.
    Conflicts {
        in_reply_to: Option<u64>,
        conflicts: Vec<u8>,
    },
    /// 🎨️ Revisioned UI patch batch for one surface — replaces `UiSection`'s cache-probe push. The
    /// reactor ABI's guest returns `turn-result.ui-patches` (`semio_framework::kernel::UiPatch`,
    /// `📓️design-abi.md` §2 "`exchange` collapse"); the host re-frames each one as this channel
    /// frame to reach a UI client. `surface`/`kind`/`revision`/`base_revision` mirror
    /// `kernel::UiPatch` field-for-field; `ops` is `kernel::PatchOp` — reused from
    /// `semio_framework::kernel`, never redefined here — pack-encoded via
    /// `store::pack_rt::encode_wire_value` (`Vec<PatchOp>`), same "nested payload stays opaque
    /// bytes" convention every other structured field in this file already uses.
    /// `base_revision` lets the client detect a stale diff and fall back to a full body instead of
    /// reconciling a diff it can't trust. `in_reply_to` is `None` for an unsolicited push (the
    /// common case — surfaces render lazily off `surface-visible`/timers, not off a command reply).
    /// CHANNEL_VERSION 12 wire addition.
    UiPatch {
        in_reply_to: Option<u64>,
        surface: String,
        kind: String,
        revision: u64,
        base_revision: u64,
        ops: Vec<u8>,
    },
    /// 🏁️ Marks the end of one surface's initial full-body snapshot burst, so a client that just
    /// subscribed (`surface-visible`) knows when it has seen a complete tree and can start applying
    /// incremental `UiPatch` frames instead of buffering. CHANNEL_VERSION 12 wire addition.
    UiSnapshotEnd {
        revision: u64,
    },
    /// 📃️ ACK-owned local-only pages are independent of ordinary command outcomes.
    LocalInteractionQuery { reply: protocol::LocalInteractionQueryReply },
}
//#endregion 🔖️AppFrame

//#region 🔖️Codec
// Hand-rolled binary frame encode/decode: `tag: u8 | fields...` — see the module-level docstring.
// `crate::os_spr::wire::🔖️WireCodec` supplies the primitives; this crate adds only the option/vec
// combinators the frame shapes need plus the tag-dispatch match arms below.

// 🚫️async: R9 pure accessor — most call sites are inside `.ok_or_else`'s sync closure
// (Option::ok_or_else requires sync FnOnce); no suspension point exists in the body either.
fn malformed(what: &'static str, offset: u64, detail: &str) -> crate::os_spr::ProtocolError {
    crate::os_spr::ProtocolError::Malformed { what, offset, detail: detail.to_string() }
}

//#region 🔖️Combinators
async fn write_opt_u64(out: &mut Vec<u8>, value: &Option<u64>) {
    crate::os_spr::write_bool(out, value.is_some());
    if let Some(v) = value {
        crate::os_spr::write_varint_u64(out, *v);
    }
}

async fn read_opt_u64(bytes: &[u8], pos: &mut usize) -> Result<Option<u64>, crate::os_spr::ProtocolError> {
    if crate::os_spr::read_bool(bytes, pos)? {
        Ok(Some(crate::os_spr::read_varint_u64(bytes, pos)?))
    } else {
        Ok(None)
    }
}

/// 🎞️ `presence u8 | byte` — an `Option<u8>` (`AppCommand::Presence.own_color`), the same
/// presence-byte convention as {@link write_opt_u64} above.
async fn write_opt_u8(out: &mut Vec<u8>, value: &Option<u8>) {
    crate::os_spr::write_bool(out, value.is_some());
    if let Some(v) = value {
        out.push(*v);
    }
}

async fn read_opt_u8(bytes: &[u8], pos: &mut usize) -> Result<Option<u8>, crate::os_spr::ProtocolError> {
    if crate::os_spr::read_bool(bytes, pos)? {
        let byte = *bytes.get(*pos).ok_or_else(|| malformed("channel app-command opt-u8", *pos as u64, "truncated"))?;
        *pos += 1;
        Ok(Some(byte))
    } else {
        Ok(None)
    }
}

async fn write_vec_bytes(out: &mut Vec<u8>, values: &[Vec<u8>]) {
    crate::os_spr::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::os_spr::write_bytes(out, value);
    }
}

async fn read_vec_bytes(bytes: &[u8], pos: &mut usize) -> Result<Vec<Vec<u8>>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    // 🚫️async: R10 shape 2 — `read_bytes` is async but `Iterator::map`'s closure is sync; hoisted
    // into a plain loop so each element can be awaited.
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(crate::os_spr::read_bytes(bytes, pos)?);
    }
    Ok(out)
}

async fn write_presence_roster(out: &mut Vec<u8>, roster: &PresenceRosterWire) {
    crate::os_spr::write_varint_u64(out, roster.len() as u64);
    for entry in roster.iter() {
        crate::os_spr::write_bytes(out, entry);
    }
}

async fn write_vec_envelope(out: &mut Vec<u8>, values: &[crate::os_spr::causal::MutationEnvelope]) {
    crate::os_spr::write_varint_u64(out, values.len() as u64);
    for value in values {
        crate::os_spr::causal::encode_envelope(value, out);
    }
}

async fn read_vec_envelope(bytes: &[u8], pos: &mut usize) -> Result<Vec<crate::os_spr::causal::MutationEnvelope>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    // 🚫️async: R10 shape 2 — `decode_envelope` is async but `Iterator::map`'s closure is sync;
    // hoisted into a plain loop so each element can be awaited.
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(crate::os_spr::causal::decode_envelope(bytes, pos)?);
    }
    Ok(out)
}
//#endregion 🔖️Combinators

/// @emoji 📤️ Encodes one `AppCommand`: `tag u8 | fields`.

struct CommandPageWriter {
    pages: CommandPageSet,
    current: [u8; COMMAND_PAGE_MAXIMUM_BYTES],
    current_len: usize,
    bytes: usize,
}

impl CommandPageWriter {
    fn try_new() -> Result<Self, crate::Fault> {
        Ok(Self { pages: CommandPageSet::try_new()?, current: [0; COMMAND_PAGE_MAXIMUM_BYTES], current_len: 0, bytes: 0 })
    }

    fn flush(&mut self) -> Result<(), crate::Fault> {
        let current = std::mem::replace(&mut self.current, [0; COMMAND_PAGE_MAXIMUM_BYTES]);
        let page = FixedCommandPage::try_from_array(current, self.current_len as u32)?;
        self.current_len = 0;
        self.pages.try_push(page).map_err(|(fault, _page)| fault)
    }

    fn write(&mut self, mut bytes: &[u8]) -> Result<(), crate::Fault> {
        while !bytes.is_empty() {
            if self.current_len == COMMAND_PAGE_MAXIMUM_BYTES {
                self.flush()?;
            }
            let take = bytes.len().min(COMMAND_PAGE_MAXIMUM_BYTES - self.current_len);
            self.current[self.current_len..self.current_len + take].copy_from_slice(&bytes[..take]);
            self.current_len += take;
            bytes = &bytes[take..];
            self.bytes += take;
        }
        Ok(())
    }

    fn byte(&mut self, byte: u8) -> Result<(), crate::Fault> {
        self.write(&[byte])
    }

    fn varint(&mut self, mut value: u64) -> Result<(), crate::Fault> {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.byte(byte)?;
            if value == 0 {
                return Ok(());
            }
        }
    }

    fn bytes(&mut self, bytes: &[u8]) -> Result<(), crate::Fault> {
        self.varint(bytes.len() as u64)?;
        self.write(bytes)
    }

    fn string(&mut self, value: &str) -> Result<(), crate::Fault> {
        self.bytes(value.as_bytes())
    }

    fn envelope(&mut self, value: &crate::os_spr::causal::MutationEnvelope) -> Result<(), crate::Fault> {
        self.string(&value.mutation_id.0)?;
        self.string(&value.document_id.0)?;
        self.string(&value.actor.0)?;
        self.varint(value.dependencies.len() as u64)?;
        for dependency in &value.dependencies {
            self.string(&dependency.0)?;
        }
        self.string(&value.diff.schema.0)?;
        self.bytes(&value.diff.payload)?;
        self.string(&value.inverse.schema.0)?;
        self.bytes(&value.inverse.payload)?;
        self.varint(value.timestamp.actor)?;
        self.varint(value.timestamp.physical_ms)?;
        self.varint(value.timestamp.logical)
    }

    fn finish(mut self) -> Result<PagedCommand, crate::Fault> {
        if self.current_len != 0 {
            self.flush()?;
        }
        PagedCommand::try_from_pages(self.pages).map_err(|(fault, _pages)| fault)
    }
}

/// 📄️ Produces the exact pre-admitted page owner consumed by reactor command batches.
pub async fn encode_app_command(command: &AppCommand) -> Result<PagedCommand, crate::Fault> {
    if let AppCommand::Presence { own_color, peers, .. } = command {
        let mut pages = CommandPageSet::try_new()?;
        for peer in peers.iter() {
            pages.try_push(FixedCommandPage::try_copy_from(peer)?).map_err(|(fault, _page)| fault)?;
        }
        if pages.is_empty() {
            pages.try_push(FixedCommandPage::try_copy_from(&[])?).map_err(|(fault, _page)| fault)?;
        }
        return PagedCommand::try_from_presence_pages(*own_color, pages, peers.len()).map_err(|(fault, _pages)| fault);
    }
    let mut out = CommandPageWriter::try_new()?;
    match command {
        AppCommand::ConfigCommand { seq, command } => {
            out.byte(0)?;
            out.varint(*seq)?;
            out.bytes(command)?;
        }
        AppCommand::Command { seq, command, view_state } => {
            out.byte(1)?;
            out.varint(*seq)?;
            out.bytes(command)?;
            out.bytes(view_state)?;
        }
        AppCommand::CommandText { seq, line } => {
            out.byte(2)?;
            out.varint(*seq)?;
            out.string(line)?;
        }
        AppCommand::ContextMenu { seq, request } => {
            out.byte(3)?;
            out.varint(*seq)?;
            out.bytes(request)?;
        }
        AppCommand::ArtifactCommand { seq, command } => {
            out.byte(4)?;
            out.varint(*seq)?;
            out.bytes(command)?;
        }
        AppCommand::ApplyEnvelopes { seq, envelopes } => {
            out.byte(5)?;
            out.varint(*seq)?;
            out.varint(envelopes.len() as u64)?;
            for envelope in envelopes {
                out.envelope(envelope)?;
            }
        }
        AppCommand::LoadDocument { seq, pack, spr } => {
            out.byte(6)?;
            out.varint(*seq)?;
            out.bytes(pack)?;
            out.bytes(spr)?;
        }
        AppCommand::ReadDocument { seq } => {
            out.byte(7)?;
            out.varint(*seq)?;
        }
        AppCommand::LoadConfig { seq, pack, spr } => {
            out.byte(8)?;
            out.varint(*seq)?;
            out.bytes(pack)?;
            out.bytes(spr)?;
        }
        AppCommand::ReadConfig { seq } => {
            out.byte(9)?;
            out.varint(*seq)?;
        }
        AppCommand::MediaIn { seq, port, descriptor, data } => {
            out.byte(10)?;
            out.varint(*seq)?;
            out.string(port)?;
            out.bytes(descriptor)?;
            out.bytes(data)?;
        }
        AppCommand::MediaOut { seq, port, request } => {
            out.byte(11)?;
            out.varint(*seq)?;
            out.string(port)?;
            out.bytes(request)?;
        }
        AppCommand::MediaFingerprint { seq, port } => {
            out.byte(12)?;
            out.varint(*seq)?;
            out.string(port)?;
        }
        AppCommand::PureCommand { seq, command, document, document_spr, config, config_spr, draft, draft_spr } => {
            out.byte(13)?;
            out.varint(*seq)?;
            out.bytes(command)?;
            out.bytes(document)?;
            out.bytes(document_spr)?;
            out.bytes(config)?;
            out.bytes(config_spr)?;
            out.bytes(draft)?;
            out.bytes(draft_spr)?;
        }
        AppCommand::LoadChildren { seq, entries } => {
            out.byte(14)?;
            out.varint(*seq)?;
            out.varint(entries.len() as u64)?;
            for entry in entries {
                out.string(&entry.slot)?;
                out.string(&entry.child_id)?;
                out.string(&entry.dialect)?;
                out.bytes(&entry.envelope_pack)?;
            }
        }
        AppCommand::ReadChildren { seq } => {
            out.byte(15)?;
            out.varint(*seq)?;
        }
        AppCommand::ReadHistory { seq } => {
            out.byte(16)?;
            out.varint(*seq)?;
        }
        AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, label, origin } => {
            out.byte(17)?;
            out.varint(*seq)?;
            out.string(txn_id)?;
            out.string(mutation_id)?;
            out.bytes(payload)?;
            out.varint(prepared_ops.len() as u64)?;
            for op in prepared_ops {
                out.bytes(op)?;
            }
            out.string(label)?;
            out.bytes(origin)?;
        }
        AppCommand::TransactionCommit { seq, txn_id } => {
            out.byte(18)?;
            out.varint(*seq)?;
            out.string(txn_id)?;
        }
        AppCommand::TransactionRollback { seq, txn_id } => {
            out.byte(19)?;
            out.varint(*seq)?;
            out.string(txn_id)?;
        }
        AppCommand::TransactionUndo { seq, group_id } => {
            out.byte(20)?;
            out.varint(*seq)?;
            out.string(group_id)?;
        }
        AppCommand::TransactionRedo { seq, group_id } => {
            out.byte(21)?;
            out.varint(*seq)?;
            out.string(group_id)?;
        }
        AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } => {
            out.byte(22)?;
            out.varint(*seq)?;
            out.string(artifact_ref)?;
            out.byte(*role)?;
            out.string(plugin_id)?;
            out.string(app_id)?;
        }
        AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } => {
            out.byte(23)?;
            out.varint(*seq)?;
            out.string(artifact_kind)?;
            out.string(standard)?;
            out.string(subset)?;
            out.byte(*role)?;
            out.string(plugin_id)?;
            out.string(app_id)?;
        }
        AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } => {
            out.byte(24)?;
            out.varint(*seq)?;
            out.string(artifact_kind)?;
            out.string(standard)?;
            out.string(subset)?;
            out.byte(*role)?;
        }
        AppCommand::SetMergePolicy { seq, policy } => {
            out.byte(25)?;
            out.varint(*seq)?;
            out.byte(*policy)?;
        }
        AppCommand::ResolveConflict { seq, conflict_id, resolution } => {
            out.byte(26)?;
            out.varint(*seq)?;
            out.string(conflict_id)?;
            out.byte(*resolution)?;
        }
        AppCommand::ReadConflicts { seq } => {
            out.byte(27)?;
            out.varint(*seq)?;
        }
        AppCommand::LocalInteractionQuery { seq, command } => {
            out.byte(29)?;
            out.varint(*seq)?;
            out.bytes(&protocol::encode_local_interaction_query_command(command))?;
        },
        AppCommand::Presence { .. } => unreachable!(),
    }
    out.finish()
}

/// @emoji 🧸️ `count varint | (slot, child_id, dialect, envelope_pack)*` — the shared list codec for
/// both `AppCommand::LoadChildren` and `AppFrame::Children`.
async fn write_vec_child_pack(out: &mut Vec<u8>, entries: &[ChildPackEntry]) {
    crate::os_spr::write_varint_u64(out, entries.len() as u64);
    for entry in entries {
        crate::os_spr::write_str(out, &entry.slot);
        crate::os_spr::write_str(out, &entry.child_id);
        crate::os_spr::write_str(out, &entry.dialect);
        crate::os_spr::write_bytes(out, &entry.envelope_pack);
    }
}

/// @emoji 🧸️ Inverse of [`write_vec_child_pack`].
async fn read_vec_child_pack(bytes: &[u8], pos: &mut usize) -> Result<Vec<ChildPackEntry>, crate::os_spr::ProtocolError> {
    let count = crate::os_spr::read_varint_u64(bytes, pos)?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(ChildPackEntry { slot: crate::os_spr::read_str(bytes, pos)?, child_id: crate::os_spr::read_str(bytes, pos)?, dialect: crate::os_spr::read_str(bytes, pos)?, envelope_pack: crate::os_spr::read_bytes(bytes, pos)? });
    }
    Ok(entries)
}

/// @emoji 📥️ Decodes one `AppCommand`, the inverse of [`encode_app_command`].
pub(super) async fn decode_app_command(bytes: &[u8]) -> Result<AppCommand, crate::os_spr::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-command tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let command = match tag {
        0 => AppCommand::ConfigCommand { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)? },
        1 => AppCommand::Command { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)?, view_state: crate::os_spr::read_bytes(bytes, &mut pos)? },
        2 => AppCommand::CommandText { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, line: crate::os_spr::read_str(bytes, &mut pos)? },
        3 => AppCommand::ContextMenu { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, request: crate::os_spr::read_bytes(bytes, &mut pos)? },
        4 => AppCommand::ArtifactCommand { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, command: crate::os_spr::read_bytes(bytes, &mut pos)? },
        5 => AppCommand::ApplyEnvelopes { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos).await? },
        6 => AppCommand::LoadDocument { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)? },
        7 => AppCommand::ReadDocument { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        8 => AppCommand::LoadConfig { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)? },
        9 => AppCommand::ReadConfig { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        10 => AppCommand::MediaIn { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, descriptor: crate::os_spr::read_bytes(bytes, &mut pos)?, data: crate::os_spr::read_bytes(bytes, &mut pos)? },
        11 => AppCommand::MediaOut { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, request: crate::os_spr::read_bytes(bytes, &mut pos)? },
        12 => AppCommand::MediaFingerprint { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)? },
        13 => AppCommand::PureCommand {
            seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            command: crate::os_spr::read_bytes(bytes, &mut pos)?,
            document: crate::os_spr::read_bytes(bytes, &mut pos)?,
            document_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft_spr: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        14 => AppCommand::LoadChildren { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos).await? },
        15 => AppCommand::ReadChildren { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        16 => AppCommand::ReadHistory { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        17 => AppCommand::TransactionPrepare {
            seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            txn_id: crate::os_spr::read_str(bytes, &mut pos)?,
            mutation_id: crate::os_spr::read_str(bytes, &mut pos)?,
            payload: crate::os_spr::read_bytes(bytes, &mut pos)?,
            prepared_ops: read_vec_bytes(bytes, &mut pos).await?,
            label: crate::os_spr::read_str(bytes, &mut pos)?,
            origin: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        18 => AppCommand::TransactionCommit { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, txn_id: crate::os_spr::read_str(bytes, &mut pos)? },
        19 => AppCommand::TransactionRollback { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, txn_id: crate::os_spr::read_str(bytes, &mut pos)? },
        20 => AppCommand::TransactionUndo { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, group_id: crate::os_spr::read_str(bytes, &mut pos)? },
        21 => AppCommand::TransactionRedo { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)?, group_id: crate::os_spr::read_str(bytes, &mut pos)? },
        22 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let artifact_ref = crate::os_spr::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command OpenArtifact.role", pos as u64, "truncated"))?;
            pos += 1;
            let plugin_id = crate::os_spr::read_str(bytes, &mut pos)?;
            let app_id = crate::os_spr::read_str(bytes, &mut pos)?;
            AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id }
        }
        23 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let artifact_kind = crate::os_spr::read_str(bytes, &mut pos)?;
            let standard = crate::os_spr::read_str(bytes, &mut pos)?;
            let subset = crate::os_spr::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command SetDefaultApp.role", pos as u64, "truncated"))?;
            pos += 1;
            let plugin_id = crate::os_spr::read_str(bytes, &mut pos)?;
            let app_id = crate::os_spr::read_str(bytes, &mut pos)?;
            AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id }
        }
        24 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let artifact_kind = crate::os_spr::read_str(bytes, &mut pos)?;
            let standard = crate::os_spr::read_str(bytes, &mut pos)?;
            let subset = crate::os_spr::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command ClearDefaultApp.role", pos as u64, "truncated"))?;
            AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role }
        }
        25 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let policy = *bytes.get(pos).ok_or_else(|| malformed("channel app-command SetMergePolicy.policy", pos as u64, "truncated"))?;
            AppCommand::SetMergePolicy { seq, policy }
        }
        26 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let conflict_id = crate::os_spr::read_str(bytes, &mut pos)?;
            let resolution = *bytes.get(pos).ok_or_else(|| malformed("channel app-command ResolveConflict.resolution", pos as u64, "truncated"))?;
            AppCommand::ResolveConflict { seq, conflict_id, resolution }
        }
        27 => AppCommand::ReadConflicts { seq: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        28 => return Err(malformed("channel presence command", pos as u64, "Presence requires reserve-before-decode PresenceCommandCursor admission")),
        29 => {
            let seq = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let length = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            if length > 142 || length as usize != bytes.len().saturating_sub(pos) { return Err(malformed("local interaction command", pos as u64, "invalid exact bounded envelope")); }
            let command = protocol::decode_local_interaction_query_command(&bytes[pos..]).map_err(|reason| malformed("local interaction command", pos as u64, reason))?;
            AppCommand::LocalInteractionQuery { seq, command }
        },
        other => return Err(malformed("channel app-command tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok(command)
}

/// 🪪 Reads only the fixed Presence tag and sequence so the app can reserve its exact
/// item/byte owner before the roster decoder allocates or copies any entry.
pub fn presence_command_sequence(bytes: &[u8]) -> Result<Option<u64>, crate::os_spr::ProtocolError> {
    let Some(tag) = bytes.first().copied() else {
        return Err(malformed("channel app-command tag", 0, "empty frame"));
    };
    if tag != 28 {
        return Ok(None);
    }
    let mut pos = 1usize;
    crate::os_spr::read_varint_u64(bytes, &mut pos).map(Some)
}

/// @emoji 📤️ Encodes one `AppFrame`: `tag u8 | fields`.
pub async fn encode_app_frame(frame: &AppFrame) -> Vec<u8> {
    let mut out = Vec::new();
    match frame {
        AppFrame::LocalInteractionQuery { reply } => {
            out.reserve_exact(4260);
            encode_local_interaction_query_frame_into(reply, &mut out).expect("typed local query frame fits admitted wire extent");
        },
        AppFrame::Done { in_reply_to } => {
            out.push(0);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
        }
        AppFrame::Invocation { in_reply_to, output, diagnostics, ui_scope, history_patch, messages } => {
            out.push(1);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, output);
            crate::os_spr::write_bytes(&mut out, diagnostics);
            crate::os_spr::write_bytes(&mut out, ui_scope);
            crate::os_spr::write_bytes(&mut out, history_patch);
            crate::os_spr::write_bytes(&mut out, messages);
        }
        AppFrame::DocumentChanged { envelopes, origin } => {
            out.push(2);
            write_vec_envelope(&mut out, envelopes).await;
            crate::os_spr::write_str(&mut out, origin);
        }
        AppFrame::Document { in_reply_to, pack, spr, ops } => {
            out.push(3);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
            crate::os_spr::write_str(&mut out, ops);
        }
        AppFrame::Config { in_reply_to, pack, spr, ops } => {
            out.push(4);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
            crate::os_spr::write_str(&mut out, ops);
        }
        AppFrame::ConfigChanged { envelopes, origin } => {
            out.push(5);
            write_vec_envelope(&mut out, envelopes).await;
            crate::os_spr::write_str(&mut out, origin);
        }
        AppFrame::ContextMenu { in_reply_to, items } => {
            out.push(6);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, items);
        }
        AppFrame::Media { in_reply_to, port, descriptor, data } => {
            out.push(7);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, descriptor);
            crate::os_spr::write_bytes(&mut out, data);
        }
        AppFrame::MediaFingerprint { in_reply_to, port, fingerprint } => {
            out.push(8);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_str(&mut out, port);
            crate::os_spr::write_bytes(&mut out, fingerprint);
        }
        AppFrame::Error { in_reply_to, fault, report } => {
            out.push(9);
            write_opt_u64(&mut out, in_reply_to).await;
            crate::os_spr::write_bytes(&mut out, fault);
            crate::os_spr::write_bytes(&mut out, report);
        }
        AppFrame::Emit { in_reply_to, document_ops, config_ops, draft_ops, output, diagnostics } => {
            out.push(10);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, document_ops);
            crate::os_spr::write_bytes(&mut out, config_ops);
            crate::os_spr::write_bytes(&mut out, draft_ops);
            crate::os_spr::write_bytes(&mut out, output);
            crate::os_spr::write_bytes(&mut out, diagnostics);
        }
        AppFrame::Draft { in_reply_to, pack, spr, ops } => {
            out.push(11);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, pack);
            crate::os_spr::write_bytes(&mut out, spr);
            crate::os_spr::write_str(&mut out, ops);
        }
        AppFrame::Children { in_reply_to, entries } => {
            out.push(12);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            write_vec_child_pack(&mut out, entries).await;
        }
        AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction } => {
            out.push(13);
            crate::os_spr::write_bytes(&mut out, presence);
            crate::os_spr::write_varint_u64(&mut out, *presence_generation);
            crate::os_spr::write_varint_u64(&mut out, *transient_generation);
            crate::os_spr::write_bytes(&mut out, interaction);
        }
        AppFrame::HistorySnapshot { in_reply_to, history_patch } => {
            out.push(14);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, history_patch);
        }
        AppFrame::TransactionProposal { in_reply_to, proposal_id, local_ops, description, coalesce_key, foreign } => {
            out.push(15);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_str(&mut out, proposal_id);
            write_vec_bytes(&mut out, local_ops).await;
            crate::os_spr::write_str(&mut out, description);
            crate::os_spr::write_str(&mut out, coalesce_key);
            write_vec_bytes(&mut out, foreign).await;
        }
        AppFrame::TransactionPrepared { txn_id, foreign, rejection } => {
            out.push(16);
            crate::os_spr::write_str(&mut out, txn_id);
            write_vec_bytes(&mut out, foreign).await;
            crate::os_spr::write_bytes(&mut out, rejection);
        }
        AppFrame::TransactionCommitted { txn_id, edit_id } => {
            out.push(17);
            crate::os_spr::write_str(&mut out, txn_id);
            crate::os_spr::write_str(&mut out, edit_id);
        }
        AppFrame::TransactionRolledBack { txn_id } => {
            out.push(18);
            crate::os_spr::write_str(&mut out, txn_id);
        }
        AppFrame::MergeReport { in_reply_to, report } => {
            out.push(19);
            write_opt_u64(&mut out, in_reply_to).await;
            crate::os_spr::write_bytes(&mut out, report);
        }
        AppFrame::Conflicts { in_reply_to, conflicts } => {
            out.push(20);
            write_opt_u64(&mut out, in_reply_to).await;
            crate::os_spr::write_bytes(&mut out, conflicts);
        }
        AppFrame::UiPatch { in_reply_to, surface, kind, revision, base_revision, ops } => {
            out.push(21);
            write_opt_u64(&mut out, in_reply_to).await;
            crate::os_spr::write_str(&mut out, surface);
            crate::os_spr::write_str(&mut out, kind);
            crate::os_spr::write_varint_u64(&mut out, *revision);
            crate::os_spr::write_varint_u64(&mut out, *base_revision);
            crate::os_spr::write_bytes(&mut out, ops);
        }
        AppFrame::UiSnapshotEnd { revision } => {
            out.push(22);
            crate::os_spr::write_varint_u64(&mut out, *revision);
        }
    }
    out
}

/// @emoji 📥️ Decodes one `AppFrame`, the inverse of [`encode_app_frame`].
pub async fn decode_app_frame(bytes: &[u8]) -> Result<AppFrame, crate::os_spr::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-frame tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let frame = match tag {
        0 => AppFrame::Done { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        1 => AppFrame::Invocation {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            output: crate::os_spr::read_bytes(bytes, &mut pos)?,
            diagnostics: crate::os_spr::read_bytes(bytes, &mut pos)?,
            ui_scope: crate::os_spr::read_bytes(bytes, &mut pos)?,
            history_patch: crate::os_spr::read_bytes(bytes, &mut pos)?,
            messages: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        2 => AppFrame::DocumentChanged { envelopes: read_vec_envelope(bytes, &mut pos).await?, origin: crate::os_spr::read_str(bytes, &mut pos)? },
        3 => AppFrame::Document { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)?, ops: crate::os_spr::read_str(bytes, &mut pos)? },
        4 => AppFrame::Config { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)?, ops: crate::os_spr::read_str(bytes, &mut pos)? },
        5 => AppFrame::ConfigChanged { envelopes: read_vec_envelope(bytes, &mut pos).await?, origin: crate::os_spr::read_str(bytes, &mut pos)? },
        6 => AppFrame::ContextMenu { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, items: crate::os_spr::read_bytes(bytes, &mut pos)? },
        7 => {
            AppFrame::Media { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, descriptor: crate::os_spr::read_bytes(bytes, &mut pos)?, data: crate::os_spr::read_bytes(bytes, &mut pos)? }
        }
        8 => AppFrame::MediaFingerprint { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, port: crate::os_spr::read_str(bytes, &mut pos)?, fingerprint: crate::os_spr::read_bytes(bytes, &mut pos)? },
        9 => AppFrame::Error { in_reply_to: read_opt_u64(bytes, &mut pos).await?, fault: crate::os_spr::read_bytes(bytes, &mut pos)?, report: crate::os_spr::read_bytes(bytes, &mut pos)? },
        10 => AppFrame::Emit {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            document_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
            config_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
            draft_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
            output: crate::os_spr::read_bytes(bytes, &mut pos)?,
            diagnostics: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        11 => AppFrame::Draft { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, pack: crate::os_spr::read_bytes(bytes, &mut pos)?, spr: crate::os_spr::read_bytes(bytes, &mut pos)?, ops: crate::os_spr::read_str(bytes, &mut pos)? },
        12 => AppFrame::Children { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos).await? },
        13 => AppFrame::Ephemeral {
            presence: crate::os_spr::read_bytes(bytes, &mut pos)?,
            presence_generation: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            transient_generation: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            interaction: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        14 => AppFrame::HistorySnapshot { in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?, history_patch: crate::os_spr::read_bytes(bytes, &mut pos)? },
        15 => AppFrame::TransactionProposal {
            in_reply_to: crate::os_spr::read_varint_u64(bytes, &mut pos)?,
            proposal_id: crate::os_spr::read_str(bytes, &mut pos)?,
            local_ops: read_vec_bytes(bytes, &mut pos).await?,
            description: crate::os_spr::read_str(bytes, &mut pos)?,
            coalesce_key: crate::os_spr::read_str(bytes, &mut pos)?,
            foreign: read_vec_bytes(bytes, &mut pos).await?,
        },
        16 => AppFrame::TransactionPrepared { txn_id: crate::os_spr::read_str(bytes, &mut pos)?, foreign: read_vec_bytes(bytes, &mut pos).await?, rejection: crate::os_spr::read_bytes(bytes, &mut pos)? },
        17 => AppFrame::TransactionCommitted { txn_id: crate::os_spr::read_str(bytes, &mut pos)?, edit_id: crate::os_spr::read_str(bytes, &mut pos)? },
        18 => AppFrame::TransactionRolledBack { txn_id: crate::os_spr::read_str(bytes, &mut pos)? },
        19 => AppFrame::MergeReport { in_reply_to: read_opt_u64(bytes, &mut pos).await?, report: crate::os_spr::read_bytes(bytes, &mut pos)? },
        20 => AppFrame::Conflicts { in_reply_to: read_opt_u64(bytes, &mut pos).await?, conflicts: crate::os_spr::read_bytes(bytes, &mut pos)? },
        21 => {
            let in_reply_to = read_opt_u64(bytes, &mut pos).await?;
            let surface = crate::os_spr::read_str(bytes, &mut pos)?;
            let kind = crate::os_spr::read_str(bytes, &mut pos)?;
            let revision = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let base_revision = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            let ops = crate::os_spr::read_bytes(bytes, &mut pos)?;
            AppFrame::UiPatch { in_reply_to, surface, kind, revision, base_revision, ops }
        }
        22 => AppFrame::UiSnapshotEnd { revision: crate::os_spr::read_varint_u64(bytes, &mut pos)? },
        23 => {
            let length = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
            if length > 4256 || length as usize != bytes.len().saturating_sub(pos) { return Err(malformed("local interaction reply", pos as u64, "invalid exact bounded envelope")); }
            AppFrame::LocalInteractionQuery { reply: protocol::decode_local_interaction_query_reply(&bytes[pos..]).map_err(|reason| malformed("local interaction reply", pos as u64, reason))? }
        },
        other => return Err(malformed("channel app-frame tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok(frame)
}
//#endregion 🔖️Codec

/// 📤️ Atomic bounded frame encoding into caller-admitted storage; failure never modifies output.
pub fn encode_local_interaction_query_frame_into(reply: &protocol::LocalInteractionQueryReply, out: &mut Vec<u8>) -> Result<(), &'static str> {
    let length = protocol::local_interaction_query_reply_encoded_len(reply)?;
    let prefix = 1 + ((usize::BITS - length.leading_zeros()).max(1) as usize + 6) / 7;
    if out.capacity() - out.len() < prefix + length { return Err("local-interaction.frame-not-admitted"); }
    out.push(23);
    crate::os_spr::write_varint_u64(out, length as u64);
    protocol::encode_local_interaction_query_reply_into(reply, out)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_batch_descriptor_has_no_destructor() {
        assert!(!std::mem::needs_drop::<CommandBatchEntry>());
    }

    //#region 🧸️Fixtures
    async fn sample_envelope(id: &str) -> crate::os_spr::causal::MutationEnvelope {
        crate::os_spr::causal::MutationEnvelope {
            mutation_id: crate::os_spr::ids::MutationId(id.to_string()),
            document_id: crate::os_spr::ids::ArtifactId("document-1".to_string()),
            actor: crate::os_spr::ids::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: crate::os_spr::causal::ArtifactDiff { schema: crate::os_spr::ids::SchemaId("diff.v1".to_string()), payload: format!("value:{id}").into_bytes() },
            inverse: crate::os_spr::causal::InverseMutation { schema: crate::os_spr::ids::SchemaId("diff.v1".to_string()), payload: Vec::new() },
            timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 0),
        }
    }

    /// @emoji #️⃣ Tiny hand-rolled `&[u8] -> String` hex encoder for this crate's own fixture-corpus
    /// tests — mirrors `db_engine`'s `write!("{byte:02x}")` idiom (no `hex` crate dependency exists
    /// anywhere in `framework/product/os`, so this crate does not introduce one either).
    async fn hex_encode(bytes: &[u8]) -> String {
        use std::fmt::Write;
        let mut out = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            let _ = write!(out, "{byte:02x}");
        }
        out
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️AppCommand
    async fn assert_command_round_trips(command: &AppCommand) {
        let encoded = encode_app_command(command).await.expect("encode must succeed");
        assert_eq!(encoded.page_len(), 1, "round-trip fixture is one page");
        let bytes = encoded.front_page().expect("round-trip fixture has one page").as_slice();
        let decoded = decode_app_command(bytes).await.expect("decode must succeed");
        assert_eq!(&decoded, command);
    }

    async fn encode_fixture_command(command: &AppCommand) -> Vec<u8> {
        let encoded = encode_app_command(command).await.expect("fixture encode must succeed");
        assert_eq!(encoded.page_len(), 1, "fixture command is one page");
        encoded.front_page().expect("fixture command has one page").as_slice().to_vec()
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_config_command_round_trips() {
        assert_command_round_trips(&AppCommand::ConfigCommand { seq: 1, command: vec![9, 9] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_command_round_trips() {
        assert_command_round_trips(&AppCommand::Command { seq: 2, command: vec![1, 2], view_state: vec![] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_command_text_round_trips() {
        assert_command_round_trips(&AppCommand::CommandText { seq: 3, line: "set foo = 1".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_context_menu_round_trips() {
        assert_command_round_trips(&AppCommand::ContextMenu { seq: 5, request: vec![7] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_document_command_round_trips() {
        assert_command_round_trips(&AppCommand::ArtifactCommand { seq: 6, command: vec![8, 8] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_apply_envelopes_round_trips() {
        assert_command_round_trips(&AppCommand::ApplyEnvelopes { seq: 7, envelopes: vec![sample_envelope("op-1").await, sample_envelope("op-2").await] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_load_document_round_trips() {
        assert_command_round_trips(&AppCommand::LoadDocument { seq: 8, pack: vec![1], spr: vec![2] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_read_artifact_round_trips() {
        assert_command_round_trips(&AppCommand::ReadDocument { seq: 9 }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_load_config_round_trips() {
        assert_command_round_trips(&AppCommand::LoadConfig { seq: 10, pack: vec![1], spr: vec![2] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_read_config_round_trips() {
        assert_command_round_trips(&AppCommand::ReadConfig { seq: 11 }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_media_in_round_trips() {
        assert_command_round_trips(&AppCommand::MediaIn { seq: 14, port: "camera".to_string(), descriptor: vec![1], data: vec![2, 3] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_media_out_round_trips() {
        assert_command_round_trips(&AppCommand::MediaOut { seq: 15, port: "speaker".to_string(), request: vec![4] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_media_fingerprint_round_trips() {
        assert_command_round_trips(&AppCommand::MediaFingerprint { seq: 16, port: "camera".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_pure_command_round_trips() {
        assert_command_round_trips(&AppCommand::PureCommand { seq: 18, command: vec![1], document: vec![2], document_spr: vec![3], config: vec![4], config_spr: vec![5], draft: vec![6], draft_spr: vec![7] }).await;
    }

    //#region 🔖️Transaction
    #[semio_framework_async_macros::async_test]
    async fn app_command_transaction_prepare_round_trips_owner_and_preplanned_forms() {
        assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 1, txn_id: "t".to_string(), mutation_id: "m".to_string(), payload: vec![9], prepared_ops: Vec::new(), label: String::new(), origin: Vec::new() }).await;
        assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], label: "l".to_string(), origin: vec![9] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_transaction_commit_round_trips() {
        assert_command_round_trips(&AppCommand::TransactionCommit { seq: 3, txn_id: "t".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_transaction_rollback_round_trips() {
        assert_command_round_trips(&AppCommand::TransactionRollback { seq: 4, txn_id: "t".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_transaction_undo_round_trips() {
        assert_command_round_trips(&AppCommand::TransactionUndo { seq: 5, group_id: "g".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_transaction_redo_round_trips() {
        assert_command_round_trips(&AppCommand::TransactionRedo { seq: 6, group_id: "g".to_string() }).await;
    }
    //#endregion 🔖️Transaction

    //#region 🔖️Opening
    #[semio_framework_async_macros::async_test]
    async fn app_command_open_artifact_round_trips_resolved_and_explicit_forms() {
        assert_command_round_trips(&AppCommand::OpenArtifact { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer".to_string(), role: 0, plugin_id: String::new(), app_id: String::new() }).await;
        assert_command_round_trips(&AppCommand::OpenArtifact { seq: 2, artifact_ref: "s.cad.cad@1/*#editor".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_set_default_app_round_trips() {
        assert_command_round_trips(&AppCommand::SetDefaultApp { seq: 3, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() })
            .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_clear_default_app_round_trips() {
        assert_command_round_trips(&AppCommand::ClearDefaultApp { seq: 4, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 0 }).await;
    }
    //#endregion 🔖️Opening

    //#region 🔖️Merge
    #[semio_framework_async_macros::async_test]
    async fn app_command_set_merge_policy_round_trips() {
        assert_command_round_trips(&AppCommand::SetMergePolicy { seq: 5, policy: 0 }).await;
        assert_command_round_trips(&AppCommand::SetMergePolicy { seq: 6, policy: 2 }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_resolve_conflict_round_trips() {
        assert_command_round_trips(&AppCommand::ResolveConflict { seq: 7, conflict_id: "c-1".to_string(), resolution: 0 }).await;
        assert_command_round_trips(&AppCommand::ResolveConflict { seq: 8, conflict_id: "c-1".to_string(), resolution: 1 }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_read_conflicts_round_trips() {
        assert_command_round_trips(&AppCommand::ReadConflicts { seq: 9 }).await;
    }
    //#endregion 🔖️Merge
    //#endregion 🔖️AppCommand

    //#region 🔖️AppFrame
    async fn assert_frame_round_trips(frame: &AppFrame) {
        let bytes = encode_app_frame(frame);
        let decoded = decode_app_frame(&bytes.await).await.expect("decode must succeed");
        assert_eq!(&decoded, frame);
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_done_round_trips() {
        assert_frame_round_trips(&AppFrame::Done { in_reply_to: 1 }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_invocation_round_trips() {
        assert_frame_round_trips(&AppFrame::Invocation { in_reply_to: 2, output: vec![1], diagnostics: vec![2], ui_scope: vec![3], history_patch: vec![4], messages: vec![5] }).await;
        assert_frame_round_trips(&AppFrame::Invocation { in_reply_to: 2, output: vec![1], diagnostics: vec![2], ui_scope: vec![3], history_patch: vec![4], messages: Vec::new() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_document_changed_round_trips() {
        assert_frame_round_trips(&AppFrame::DocumentChanged { envelopes: vec![sample_envelope("op-1").await], origin: "peer-1".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_document_round_trips() {
        assert_frame_round_trips(&AppFrame::Document { in_reply_to: 5, pack: vec![1], spr: vec![2], ops: "set foo = 1".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_config_round_trips() {
        assert_frame_round_trips(&AppFrame::Config { in_reply_to: 5, pack: vec![1], spr: vec![2], ops: "set cam = 1".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_config_changed_round_trips() {
        assert_frame_round_trips(&AppFrame::ConfigChanged { envelopes: vec![sample_envelope("cfg-1").await], origin: "peer-1".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_context_menu_round_trips() {
        assert_frame_round_trips(&AppFrame::ContextMenu { in_reply_to: 6, items: vec![1, 2, 3] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_media_round_trips() {
        assert_frame_round_trips(&AppFrame::Media { in_reply_to: 7, port: "camera".to_string(), descriptor: vec![1], data: vec![2] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_media_fingerprint_round_trips() {
        assert_frame_round_trips(&AppFrame::MediaFingerprint { in_reply_to: 8, port: "camera".to_string(), fingerprint: vec![1, 2] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_error_round_trips() {
        assert_frame_round_trips(&AppFrame::Error { in_reply_to: Some(9), fault: b"rejected:bad command".to_vec(), report: vec![1, 2] }).await;
        assert_frame_round_trips(&AppFrame::Error { in_reply_to: None, fault: b"rejected:bad command".to_vec(), report: Vec::new() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_emit_round_trips() {
        assert_frame_round_trips(&AppFrame::Emit { in_reply_to: 14, document_ops: vec![1], config_ops: vec![2], draft_ops: vec![3], output: vec![4], diagnostics: vec![5] }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_draft_round_trips() {
        assert_frame_round_trips(&AppFrame::Draft { in_reply_to: 15, pack: vec![1], spr: vec![2], ops: "d".to_string() }).await;
        assert_frame_round_trips(&AppFrame::Children { in_reply_to: 16, entries: sample_child_entries().await }).await;
        assert_frame_round_trips(&AppFrame::Children { in_reply_to: 17, entries: Vec::new() }).await;
        assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![9, 9] }).await;
        assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: Vec::new() }).await;
    }

    //#region 🔖️Children
    /// 🧸️ Two children in different slots, one of them with an empty pack (a genesis child whose
    /// envelope has not been printed yet), so the list codec is exercised at both extremes.
    async fn sample_child_entries() -> Vec<ChildPackEntry> {
        vec![
            ChildPackEntry { slot: "mesh".to_string(), child_id: "child-1".to_string(), dialect: "s.stdio.mesh@1/*".to_string(), envelope_pack: vec![7, 8, 9] },
            ChildPackEntry { slot: "brep".to_string(), child_id: "child-2".to_string(), dialect: "s.stdio.brep@1/*".to_string(), envelope_pack: Vec::new() },
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn child_pack_commands_round_trip() {
        assert_command_round_trips(&AppCommand::LoadChildren { seq: 19, entries: sample_child_entries().await }).await;
        assert_command_round_trips(&AppCommand::LoadChildren { seq: 20, entries: Vec::new() }).await;
        assert_command_round_trips(&AppCommand::ReadChildren { seq: 21 }).await;
        assert_command_round_trips(&AppCommand::ReadHistory { seq: 22 }).await;
    }
    //#endregion 🔖️Children

    //#region 🔖️Transaction
    #[semio_framework_async_macros::async_test]
    async fn app_frame_transaction_proposal_round_trips() {
        assert_frame_round_trips(&AppFrame::TransactionProposal { in_reply_to: 1, proposal_id: "p".to_string(), local_ops: vec![vec![1]], description: "d".to_string(), coalesce_key: "k".to_string(), foreign: Vec::new() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_transaction_prepared_round_trips_with_and_without_rejection() {
        assert_frame_round_trips(&AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: vec![vec![1]], rejection: Vec::new() }).await;
        assert_frame_round_trips(&AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: Vec::new(), rejection: b"rejected".to_vec() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_transaction_committed_round_trips() {
        assert_frame_round_trips(&AppFrame::TransactionCommitted { txn_id: "t".to_string(), edit_id: "e".to_string() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_transaction_rolled_back_round_trips() {
        assert_frame_round_trips(&AppFrame::TransactionRolledBack { txn_id: "t".to_string() }).await;
    }
    //#endregion 🔖️Transaction

    //#region 🔖️Merge
    #[semio_framework_async_macros::async_test]
    async fn app_frame_merge_report_round_trips() {
        assert_frame_round_trips(&AppFrame::MergeReport { in_reply_to: Some(1), report: vec![1, 2, 3] }).await;
        assert_frame_round_trips(&AppFrame::MergeReport { in_reply_to: None, report: Vec::new() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_conflicts_round_trips() {
        assert_frame_round_trips(&AppFrame::Conflicts { in_reply_to: Some(2), conflicts: vec![4, 5] }).await;
        assert_frame_round_trips(&AppFrame::Conflicts { in_reply_to: None, conflicts: Vec::new() }).await;
    }
    //#endregion 🔖️Merge

    //#region 🔖️UiPatch
    #[semio_framework_async_macros::async_test]
    async fn app_frame_ui_patch_round_trips_with_and_without_in_reply_to() {
        assert_frame_round_trips(&AppFrame::UiPatch { in_reply_to: Some(3), surface: "1:body".to_string(), kind: "window".to_string(), revision: 5, base_revision: 4, ops: vec![1, 2, 3] }).await;
        assert_frame_round_trips(&AppFrame::UiPatch { in_reply_to: None, surface: "1:body".to_string(), kind: "window".to_string(), revision: 1, base_revision: 0, ops: Vec::new() }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_ui_snapshot_end_round_trips() {
        assert_frame_round_trips(&AppFrame::UiSnapshotEnd { revision: 7 }).await;
    }
    //#endregion 🔖️UiPatch
    //#endregion 🔖️AppFrame

    //#region 🔖️Codec
    #[semio_framework_async_macros::async_test]
    async fn encoding_is_deterministic() {
        let command = AppCommand::ContextMenu { seq: 1, request: vec![1, 2, 3] };
        assert_eq!(encode_app_command(&command).await, encode_app_command(&command).await);

        let frame = AppFrame::Error { in_reply_to: Some(1), fault: b"e:m".to_vec(), report: vec![9] };
        assert_eq!(encode_app_frame(&frame).await, encode_app_frame(&frame).await);
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_app_command_rejects_empty_bytes() {
        let err = decode_app_command(&[]).await.unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-command tag", .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_app_frame_rejects_empty_bytes() {
        let err = decode_app_frame(&[]).await.unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_app_command_rejects_unknown_tag() {
        let err = decode_app_command(&[0xFF]).await.unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-command tag", .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_app_frame_rejects_unknown_tag() {
        let err = decode_app_frame(&[0xFF]).await.unwrap_err();
        assert!(matches!(err, crate::os_spr::ProtocolError::Malformed { what: "channel app-frame tag", .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_app_command_rejects_truncated_field() {
        let encoded = encode_app_command(&AppCommand::CommandText { seq: 1, line: "hello".to_string() }).await.unwrap();
        assert_eq!(encoded.page_len(), 1);
        let bytes = encoded.front_page().unwrap().as_slice();
        let truncated = &bytes[..bytes.len() - 2];
        assert!(decode_app_command(truncated).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn presence_roster_fixed_maximum_plus_one_returns_the_exact_rejected_owner() {
        let mut roster = PresenceRosterWire::empty();
        for index in 0..PRESENCE_ROSTER_MAXIMUM_ITEMS {
            roster.try_push(vec![index as u8]).expect("fixed admitted roster slot");
        }
        let rejected = vec![0xA5; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES];
        let error = roster.try_push(rejected.clone()).expect_err("maximum plus one must fail before growth");
        assert_eq!(error.entry, rejected);
        assert_eq!(roster.len(), PRESENCE_ROSTER_MAXIMUM_ITEMS);
    }

    #[semio_framework_async_macros::async_test]
    async fn presence_cursor_preserves_fifo_and_releases_at_most_one_grant() {
        let mut roster = PresenceRosterWire::empty();
        roster.try_push(vec![1; 17]).unwrap();
        roster.try_push(vec![2; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES]).unwrap();
        let mut cursor = PresenceCommandCursor::admit_page(77, Some(4), 2, FixedCommandPage::try_copy_from(&[1; 17]).unwrap()).map_err(|(error, _)| error).unwrap();
        assert_eq!(cursor.take_next().unwrap().unwrap().as_slice(), &[1; 17]);
        cursor.push_page(1, FixedCommandPage::try_copy_from(&[2; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES]).unwrap()).map_err(|(error, _)| error).unwrap();
        assert_eq!(cursor.take_next().unwrap().unwrap().as_slice(), &[2; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES]);
        assert!(cursor.take_next().unwrap().is_none());
        assert!(cursor.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn generic_decoder_rejects_presence_before_whole_roster_materialization() {
        let mut roster = PresenceRosterWire::empty();
        roster.try_push(vec![7]).unwrap();
        let encoded = encode_app_command(&AppCommand::Presence { seq: 3, own_color: None, peers: roster }).await.unwrap();
        assert_eq!(encoded.kind(), 28);
        let mut cursor = PresenceCommandCursor::admit_page(3, None, 1, FixedCommandPage::try_copy_from(&[7]).unwrap()).map_err(|(error, _)| error).unwrap();
        while !cursor.close_release(PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES).0 {}
    }

    #[semio_framework_async_macros::async_test]
    async fn paged_generic_decoder_crosses_a_two_page_field_boundary_without_concatenation() {
        let expected = AppCommand::Command { seq: 1, command: vec![0xA5; COMMAND_PAGE_MAXIMUM_BYTES + 1], view_state: vec![7, 8] };
        let encoded = encode_app_command(&expected).await.unwrap();
        assert_eq!(encoded.page_len(), 2);
        let mut cursor = PagedAppCommandDecodeCursor::new(encoded);
        assert!(cursor.step().unwrap().is_none());
        assert!(cursor.step().unwrap().is_none());
        assert_eq!(cursor.step().unwrap(), Some(expected));
        assert!(cursor.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn paged_generic_decoder_admits_document_config_and_projection_commands_used_during_browser_boot() {
        let commands = [
            AppCommand::LoadDocument { seq: 1, pack: vec![1, 2], spr: vec![3] },
            AppCommand::ReadDocument { seq: 2 },
            AppCommand::LoadConfig { seq: 3, pack: vec![4], spr: vec![5, 6] },
            AppCommand::ReadConfig { seq: 4 },
            AppCommand::ReadChildren { seq: 5 },
            AppCommand::ReadHistory { seq: 6 },
            AppCommand::ReadConflicts { seq: 7 },
        ];
        for expected in commands {
            let encoded = encode_app_command(&expected).await.unwrap();
            let mut cursor = PagedAppCommandDecodeCursor::new(encoded);
            let mut decoded = None;
            for _ in 0..4 {
                decoded = cursor.step().unwrap();
                if decoded.is_some() {
                    break;
                }
            }
            assert_eq!(decoded, Some(expected));
            assert!(cursor.terminal_is_empty());
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn paged_generic_decoder_faults_hostile_field_length_and_closes_exact_owner() {
        let page = FixedCommandPage::try_copy_from(&[0, 1, 0x81, 0x80, 0x10]).unwrap();
        let mut pages = CommandPageSet::try_new().unwrap();
        pages.try_push(page).unwrap();
        let mut cursor = PagedAppCommandDecodeCursor::new(PagedCommand::try_from_pages(pages).unwrap());
        assert!(cursor.step().unwrap().is_none());
        let fault = cursor.step().unwrap_err();
        assert_eq!(fault.code.0, "plugin.command-field-cap");
        assert_eq!(cursor.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 0));
        assert!(cursor.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn paged_generic_decoder_retains_first_field_when_middle_field_is_truncated() {
        let page = FixedCommandPage::try_copy_from(&[1, 1, 1, 9, 5, 7]).unwrap();
        let mut pages = CommandPageSet::try_new().unwrap();
        pages.try_push(page).unwrap();
        let mut cursor = PagedAppCommandDecodeCursor::new(PagedCommand::try_from_pages(pages).unwrap());
        assert!(cursor.step().unwrap().is_none());
        assert!(cursor.step().unwrap().is_none());
        assert_eq!(cursor.step().unwrap_err().code.0, "plugin.command-decode-truncated");
        assert_eq!(cursor.close_step(1), (false, 1));
        assert_eq!(cursor.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 0));
        assert!(cursor.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn paged_generic_decoder_retains_terminal_fields_when_trailing_bytes_fault() {
        let page = FixedCommandPage::try_copy_from(&[3, 1, 1, 9, 0xFF]).unwrap();
        let mut pages = CommandPageSet::try_new().unwrap();
        pages.try_push(page).unwrap();
        let mut cursor = PagedAppCommandDecodeCursor::new(PagedCommand::try_from_pages(pages).unwrap());
        assert!(cursor.step().unwrap().is_none());
        assert_eq!(cursor.step().unwrap_err().code.0, "plugin.command-decode-trailing");
        assert_eq!(cursor.close_step(1), (false, 1));
        assert_eq!(cursor.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 5));
        assert!(cursor.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn decoded_generic_owner_closes_two_fields_one_grant_at_a_time() {
        let mut owner = DecodedAppCommandOwner::new(AppCommand::Command { seq: 7, command: vec![1; 4_096], view_state: vec![2; 4_096] });
        assert_eq!(owner.close_step(4_096), (false, 1, 4_096));
        assert_eq!(owner.close_step(4_096), (false, 1, 4_096));
        assert_eq!(owner.close_step(4_096), (true, 1, 0));
        assert!(owner.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_app_frame_rejects_truncated_field() {
        let bytes = encode_app_frame(&AppFrame::Error { in_reply_to: Some(1), fault: b"e:message".to_vec(), report: Vec::new() }).await;
        let truncated = &bytes[..bytes.len() - 2];
        assert!(decode_app_frame(truncated).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_app_command_never_panics_on_arbitrary_short_buffers() {
        for len in 0..8 {
            let buf = vec![0u8; len];
            let _ = decode_app_command(&buf);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_app_frame_never_panics_on_arbitrary_short_buffers() {
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
    async fn channel_command_fixture_corpus() -> Vec<(&'static str, AppCommand)> {
        let mut presence_roster = PresenceRosterWire::empty();
        presence_roster.try_push(vec![1, 2]).expect("bounded presence fixture entry");
        presence_roster.try_push(vec![9]).expect("bounded presence fixture entry");
        vec![
            ("ConfigCommand", AppCommand::ConfigCommand { seq: 1, command: vec![9] }),
            ("Command", AppCommand::Command { seq: 1, command: vec![1], view_state: vec![] }),
            ("CommandText", AppCommand::CommandText { seq: 1, line: "go".to_string() }),
            ("ContextMenu", AppCommand::ContextMenu { seq: 1, request: vec![1] }),
            ("ArtifactCommand", AppCommand::ArtifactCommand { seq: 1, command: vec![1] }),
            ("ApplyEnvelopes", AppCommand::ApplyEnvelopes { seq: 1, envelopes: Vec::new() }),
            ("LoadDocument", AppCommand::LoadDocument { seq: 1, pack: vec![1], spr: vec![2] }),
            ("ReadDocument", AppCommand::ReadDocument { seq: 1 }),
            ("LoadConfig", AppCommand::LoadConfig { seq: 1, pack: vec![1], spr: vec![2] }),
            ("ReadConfig", AppCommand::ReadConfig { seq: 1 }),
            ("MediaIn", AppCommand::MediaIn { seq: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
            ("MediaOut", AppCommand::MediaOut { seq: 1, port: "p".to_string(), request: vec![1] }),
            ("MediaFingerprint", AppCommand::MediaFingerprint { seq: 1, port: "p".to_string() }),
            ("PureCommand", AppCommand::PureCommand { seq: 1, command: vec![1], document: vec![2], document_spr: vec![3], config: vec![4], config_spr: vec![5], draft: vec![6], draft_spr: vec![7] }),
            ("LoadChildren", AppCommand::LoadChildren { seq: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1] }] }),
            ("ReadChildren", AppCommand::ReadChildren { seq: 1 }),
            ("ReadHistory", AppCommand::ReadHistory { seq: 1 }),
            ("TransactionPrepareOwner", AppCommand::TransactionPrepare { seq: 1, txn_id: "t".to_string(), mutation_id: "m".to_string(), payload: vec![9], prepared_ops: Vec::new(), label: String::new(), origin: Vec::new() }),
            ("TransactionPreparePrePlanned", AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], label: "l".to_string(), origin: vec![9] }),
            ("TransactionCommit", AppCommand::TransactionCommit { seq: 3, txn_id: "t".to_string() }),
            ("TransactionRollback", AppCommand::TransactionRollback { seq: 4, txn_id: "t".to_string() }),
            ("TransactionUndo", AppCommand::TransactionUndo { seq: 5, group_id: "g".to_string() }),
            ("TransactionRedo", AppCommand::TransactionRedo { seq: 6, group_id: "g".to_string() }),
            ("OpenArtifactResolve", AppCommand::OpenArtifact { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer".to_string(), role: 0, plugin_id: String::new(), app_id: String::new() }),
            ("OpenArtifactExplicit", AppCommand::OpenArtifact { seq: 2, artifact_ref: "s.cad.cad@1/*#editor".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() }),
            ("SetDefaultApp", AppCommand::SetDefaultApp { seq: 3, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 1, plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() }),
            ("ClearDefaultApp", AppCommand::ClearDefaultApp { seq: 4, artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string(), role: 0 }),
            ("SetMergePolicy", AppCommand::SetMergePolicy { seq: 5, policy: 1 }),
            ("ResolveConflict", AppCommand::ResolveConflict { seq: 6, conflict_id: "conflict-1".to_string(), resolution: 0 }),
            ("ReadConflicts", AppCommand::ReadConflicts { seq: 7 }),
            ("Presence", AppCommand::Presence { seq: 8, own_color: Some(3), peers: presence_roster }),
        ]
    }

    /// @emoji 🧾️ Named `AppFrame` fixture corpus, one entry per variant.
    async fn channel_frame_fixture_corpus() -> Vec<(&'static str, AppFrame)> {
        vec![
            ("Done", AppFrame::Done { in_reply_to: 1 }),
            ("Invocation", AppFrame::Invocation { in_reply_to: 1, output: vec![1], diagnostics: vec![], ui_scope: vec![], history_patch: vec![], messages: vec![9] }),
            ("DocumentChanged", AppFrame::DocumentChanged { envelopes: vec![], origin: "o".to_string() }),
            ("Document", AppFrame::Document { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "o".to_string() }),
            ("Config", AppFrame::Config { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "c".to_string() }),
            ("ConfigChanged", AppFrame::ConfigChanged { envelopes: vec![], origin: "o".to_string() }),
            ("ContextMenu", AppFrame::ContextMenu { in_reply_to: 1, items: vec![1] }),
            ("Media", AppFrame::Media { in_reply_to: 1, port: "p".to_string(), descriptor: vec![1], data: vec![2] }),
            ("MediaFingerprint", AppFrame::MediaFingerprint { in_reply_to: 1, port: "p".to_string(), fingerprint: vec![1] }),
            ("Error", AppFrame::Error { in_reply_to: None, fault: vec![99], report: vec![7] }),
            ("Emit", AppFrame::Emit { in_reply_to: 1, document_ops: vec![1], config_ops: vec![], draft_ops: vec![], output: vec![2], diagnostics: vec![] }),
            ("Draft", AppFrame::Draft { in_reply_to: 1, pack: vec![1], spr: vec![2], ops: "d".to_string() }),
            ("Children", AppFrame::Children { in_reply_to: 1, entries: vec![ChildPackEntry { slot: "s".to_string(), child_id: "c".to_string(), dialect: "d".to_string(), envelope_pack: vec![1] }] }),
            ("Ephemeral", AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![7] }),
            ("HistorySnapshot", AppFrame::HistorySnapshot { in_reply_to: 1, history_patch: vec![1] }),
            ("TransactionProposal", AppFrame::TransactionProposal { in_reply_to: 1, proposal_id: "p".to_string(), local_ops: vec![vec![1]], description: "d".to_string(), coalesce_key: "k".to_string(), foreign: Vec::new() }),
            ("TransactionPrepared", AppFrame::TransactionPrepared { txn_id: "t".to_string(), foreign: vec![vec![1]], rejection: Vec::new() }),
            ("TransactionCommitted", AppFrame::TransactionCommitted { txn_id: "t".to_string(), edit_id: "e".to_string() }),
            ("TransactionRolledBack", AppFrame::TransactionRolledBack { txn_id: "t".to_string() }),
            ("MergeReport", AppFrame::MergeReport { in_reply_to: Some(1), report: vec![1] }),
            ("Conflicts", AppFrame::Conflicts { in_reply_to: None, conflicts: vec![2] }),
            ("UiPatch", AppFrame::UiPatch { in_reply_to: Some(1), surface: "1:body".to_string(), kind: "window".to_string(), revision: 2, base_revision: 1, ops: vec![3] }),
            ("UiSnapshotEnd", AppFrame::UiSnapshotEnd { revision: 4 }),
        ]
    }

    /// @emoji 🔒️ Golden hex per `AppCommand` fixture-corpus label — sourced by actually running
    /// `encode_app_command` over `channel_command_fixture_corpus()` (never hand-computed), then
    /// committed here as the drift guard: any future codec change that shifts these bytes fails
    /// this test, forcing a deliberate update of both this table and the TS-side twin (WP-0B).
    async fn channel_command_fixture_hex(label: &str) -> &'static str {
        match label {
            "ConfigCommand" => "00010109",
            "Command" => "0101010100",
            "CommandText" => "020102676f",
            "ContextMenu" => "03010101",
            "ArtifactCommand" => "04010101",
            "ApplyEnvelopes" => "050100",
            "LoadDocument" => "060101010102",
            "ReadDocument" => "0701",
            "LoadConfig" => "080101010102",
            "ReadConfig" => "0901",
            "MediaIn" => "0a01017001010102",
            "MediaOut" => "0b0101700101",
            "MediaFingerprint" => "0c010170",
            "PureCommand" => "0d010101010201030104010501060107",
            "LoadChildren" => "0e01010173016301640101",
            "ReadChildren" => "0f01",
            "ReadHistory" => "1001",
            "TransactionPrepareOwner" => "11010174016d0109000000",
            "TransactionPreparePrePlanned" => "110201740000020101020202016c0109",
            "TransactionCommit" => "12030174",
            "TransactionRollback" => "13040174",
            "TransactionUndo" => "14050167",
            "TransactionRedo" => "15060167",
            "OpenArtifactResolve" => "160114732e6361642e63616440312f2a23766965776572000000",
            "OpenArtifactExplicit" => "160214732e6361642e63616440312f2a23656469746f72010363616414732e6361642e63616440312f2a23656469746f72",
            "SetDefaultApp" => "170309732e6361642e6361640131012a010363616414732e6361642e63616440312f2a23656469746f72",
            "ClearDefaultApp" => "180409732e6361642e6361640131012a00",
            "SetMergePolicy" => "190501",
            "ResolveConflict" => "1a060a636f6e666c6963742d3100",
            "ReadConflicts" => "1b07",
            "Presence" => "1c080103020201020109",
            other => panic!("channel_command_fixture_hex: no golden hex registered for label {other:?}"),
        }
    }

    /// @emoji 🔒️ Golden hex per `AppFrame` fixture-corpus label — see
    /// `channel_command_fixture_hex`'s docstring for provenance/drift-guard rationale.
    async fn channel_frame_fixture_hex(label: &str) -> &'static str {
        match label {
            "Done" => "0001",
            "Invocation" => "010101010000000109",
            "DocumentChanged" => "0200016f",
            "Document" => "030101010102016f",
            "Config" => "0401010101020163",
            "ConfigChanged" => "0500016f",
            "ContextMenu" => "06010101",
            "Media" => "0701017001010102",
            "MediaFingerprint" => "080101700101",
            "Error" => "090001630107",
            "Emit" => "0a0101010000010200",
            "Draft" => "0b01010101020164",
            "Children" => "0c01010173016301640101",
            "Ephemeral" => "0d02010203040107",
            "HistorySnapshot" => "0e010101",
            "TransactionProposal" => "0f0101700101010164016b00",
            "TransactionPrepared" => "10017401010100",
            "TransactionCommitted" => "1101740165",
            "TransactionRolledBack" => "120174",
            "MergeReport" => "1301010101",
            "Conflicts" => "14000102",
            "UiPatch" => "15010106313a626f64790677696e646f7702010103",
            "UiSnapshotEnd" => "1604",
            other => panic!("channel_frame_fixture_hex: no golden hex registered for label {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn app_command_fixture_corpus_matches_golden_hex_and_round_trips() {
        for (label, value) in channel_command_fixture_corpus().await {
            if let AppCommand::Presence { seq, own_color, peers } = &value {
                let first = peers.iter().next().map(<[u8]>::to_vec).unwrap_or_default();
                let mut cursor = PresenceCommandCursor::admit_page(*seq, *own_color, peers.len() as u32, FixedCommandPage::try_copy_from(&first).expect("encoded Presence page is fixed-authority"))
                    .map_err(|(error, _)| error)
                    .expect("Presence uses retained cursor admission");
                assert_eq!(cursor.seq(), *seq);
                assert_eq!(cursor.own_color(), *own_color);
                let mut entries = Vec::new();
                for (index, peer) in peers.iter().enumerate() {
                    if index != 0 {
                        cursor.push_page(index as u32, FixedCommandPage::try_copy_from(peer).expect("encoded Presence page is fixed-authority")).map_err(|(error, _)| error).expect("ordered peer page");
                    }
                    if let Some(entry) = cursor.take_next().expect("bounded Presence entry") {
                        entries.push(entry);
                    }
                }
                if peers.is_empty() {
                    while !cursor.close_release(PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES).0 {}
                }
                assert!(entries.iter().map(FixedCommandPage::as_slice).eq(peers.iter()), "Presence retained cursor must preserve exact entry order");
                while !cursor.close_release(PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES).0 {}
                assert!(cursor.terminal_is_empty());
                continue;
            }
            let encoded = encode_app_command(&value).await.unwrap();
            assert_eq!(encoded.page_len(), 1, "fixture is one page");
            let bytes = encoded.front_page().expect("fixture has one page").as_slice();
            let actual = hex_encode(bytes).await;
            assert_eq!(actual, channel_command_fixture_hex(label).await, "{label}'s encoding drifted from its committed golden hex");
            let decoded = decode_app_command(bytes).await.unwrap();
            assert_eq!(decoded, value, "{label} must round-trip");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn app_frame_fixture_corpus_matches_golden_hex_and_round_trips() {
        for (label, value) in channel_frame_fixture_corpus().await {
            let encoded = encode_app_frame(&value).await;
            let actual = hex_encode(&encoded).await;
            assert_eq!(actual, channel_frame_fixture_hex(label).await, "{label}'s encoding drifted from its committed golden hex");
            let decoded = decode_app_frame(&encode_app_frame(&value).await).await.unwrap();
            assert_eq!(decoded, value, "{label} must round-trip");
        }
    }

    /// @emoji 📡️ The wire version is owned by `🧫️fixtures/📡️channel/channel-version.json`, not by
    /// either language's constant, so a bump that updates only one host fails here instead of at
    /// runtime — the drift this guard was added for was a live `APP_CHANNEL_VERSION = 8` in
    /// TypeScript against `CHANNEL_VERSION = 10` in Rust. The TS twin asserts the same file.
    #[semio_framework_async_macros::async_test]
    async fn channel_version_matches_the_shared_cross_language_pin() {
        let json = include_str!("../../../🧫️fixtures/📡️channel/channel-version.json");
        let pin: serde_json::Value = serde_json::from_str(json).expect("channel-version.json must parse");
        let pinned = pin.get("channelVersion").and_then(serde_json::Value::as_u64).expect("channel-version.json must carry channelVersion");
        assert_eq!(u64::from(CHANNEL_VERSION), pinned, "CHANNEL_VERSION and the shared cross-language pin disagree — bump both, plus APP_CHANNEL_VERSION in 🟦️.ts");
    }

    /// @emoji 🔗️ Cross-language drift guard for the M2 transaction variants (tags 17-21/15-18): the
    /// two JSON files under `🧫️fixtures/📡️channel/` are the single source of truth this codec's TS
    /// twin (`🟦️.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too —
    /// a change to either side's encode/decode that shifts these bytes fails on exactly one side.
    #[semio_framework_async_macros::async_test]
    async fn channel_transaction_fixtures_match_shared_cross_language_json_vectors() {
        let command_json = include_str!("../../../🧫️fixtures/📡️channel/app-command-transaction.json");
        let frame_json = include_str!("../../../🧫️fixtures/📡️channel/app-frame-transaction.json");
        let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("app-command-transaction.json must parse");
        let frame_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(frame_json).expect("app-frame-transaction.json must parse");
        assert_eq!(command_vectors.len(), 6, "app-command-transaction.json vector count changed");
        assert_eq!(frame_vectors.len(), 4, "app-frame-transaction.json vector count changed");

        for (label, value) in channel_command_fixture_corpus().await {
            if let Some(expected) = command_vectors.get(label) {
                let actual = hex_encode(&encode_fixture_command(&value).await).await;
                assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
            }
        }
        for (label, value) in channel_frame_fixture_corpus().await {
            if let Some(expected) = frame_vectors.get(label) {
                let actual = hex_encode(&encode_app_frame(&value).await).await;
                assert_eq!(&actual, expected, "AppFrame::{label} drifted from the shared cross-language fixture");
            }
        }
    }

    /// @emoji 🔗️ Cross-language drift guard for the C3 opening variants (tags 22-24): the JSON file
    /// under `🧫️fixtures/📡️channel/` is the single source of truth this codec's TS twin
    /// (`🟦️.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too — no
    /// `AppFrame` variants were added for opening, so only the command-side vector file exists.
    #[semio_framework_async_macros::async_test]
    async fn channel_opening_fixtures_match_shared_cross_language_json_vectors() {
        let command_json = include_str!("../../../🧫️fixtures/📡️channel/app-command-opening.json");
        let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("app-command-opening.json must parse");
        assert_eq!(command_vectors.len(), 4, "app-command-opening.json vector count changed");

        for (label, value) in channel_command_fixture_corpus().await {
            if let Some(expected) = command_vectors.get(label) {
                let actual = hex_encode(&encode_fixture_command(&value).await).await;
                assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
            }
        }
    }

    /// @emoji 🔗️ Cross-language drift guard for the C8 merge-policy/conflict variants (tags 25-27,
    /// 19-20) plus the extended `Invocation`/`Error` frames: the two JSON files under
    /// `🧫️fixtures/📡️channel/` are the single source of truth this codec's TS twin
    /// (`🟦️.ts`'s `AppChannelCodec` `🧪️Tests` region) loads and asserts against too — see
    /// contract-freeze.md §C8 of
    /// `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
    #[semio_framework_async_macros::async_test]
    async fn channel_merge_fixtures_match_shared_cross_language_json_vectors() {
        let command_json = include_str!("../../../🧫️fixtures/📡️channel/app-command-merge.json");
        let frame_json = include_str!("../../../🧫️fixtures/📡️channel/app-frame-merge.json");
        let command_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(command_json).expect("app-command-merge.json must parse");
        let frame_vectors: std::collections::BTreeMap<String, String> = serde_json::from_str(frame_json).expect("app-frame-merge.json must parse");
        assert_eq!(command_vectors.len(), 3, "app-command-merge.json vector count changed");
        assert_eq!(frame_vectors.len(), 4, "app-frame-merge.json vector count changed");

        for (label, value) in channel_command_fixture_corpus().await {
            if let Some(expected) = command_vectors.get(label) {
                let actual = hex_encode(&encode_fixture_command(&value).await).await;
                assert_eq!(&actual, expected, "AppCommand::{label} drifted from the shared cross-language fixture");
            }
        }
        for (label, value) in channel_frame_fixture_corpus().await {
            if let Some(expected) = frame_vectors.get(label) {
                let actual = hex_encode(&encode_app_frame(&value).await).await;
                assert_eq!(&actual, expected, "AppFrame::{label} drifted from the shared cross-language fixture");
            }
        }
    }
    //#endregion 🔖️Corpus
}
//#endregion 🧪️Tests
