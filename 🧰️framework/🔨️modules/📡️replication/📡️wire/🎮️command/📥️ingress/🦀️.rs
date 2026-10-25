//! 📥️ Product-neutral bounded paged-command ownership, admission and acknowledged release.
use serde::Deserialize;
use std::mem::size_of;

//#region 🔖️PagedCommandIngress
pub const COMMAND_PAGE_MAXIMUM_BYTES: usize = 4_096;

/// 🚚️ Largest ASSEMBLED command the host may deliver into a guest.
///
/// 🧊️ A command is a host answer like any other, so it is bound by the one declared budget for an
/// assembled host answer rather than by a transport constant of its own. Nothing here chooses a
/// ceiling: [`semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES`] already says what a guest may
/// be handed for ONE outstanding request before it answers with a typed fault instead of allocating.
pub const COMMAND_MAXIMUM_BYTES: usize = semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES;

/// 📃️ Pages that budget occupies — the assembled ceiling over the page extent, not a chosen number.
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

#[path = "📄️command-page-behavior/🦀️.rs"]
mod command_page_behavior;

#[path = "🗃️page-set-and-reader-behavior/🦀️.rs"]
mod page_set_and_reader_behavior;

#[path = "📨️envelope-and-rejection-behavior/🦀️.rs"]
mod envelope_and_rejection_behavior;

#[path = "🧰️driver-registry-behavior/🦀️.rs"]
mod driver_registry_behavior;

#[path = "🚚️batch-driver-behavior/🦀️.rs"]
mod batch_driver_behavior;

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

struct FixedCommandPageRetirement{bytes:std::mem::ManuallyDrop<Option<Box<[u8;COMMAND_PAGE_MAXIMUM_BYTES]>>>}

/// 🔂️ One command's page authority, reserved ONCE for exactly the pages that command DECLARES.
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

#[derive(Debug, PartialEq)]
pub struct PagedCommand {
    pages: std::collections::VecDeque<FixedCommandPage>,
    byte_len: usize,
    kind: u8,
    metadata: u32,
    item_count: u32,
}

#[derive(Debug)]
pub struct PagedCommandReader {
    command: PagedCommand,
    offset: usize,
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

#[derive(Debug)]
pub struct RejectedCommandBuild {
    rejected: Option<CommandEnvelope>,
    admitted: CommandEnvelopeSet,
}

#[derive(Debug)]
pub struct RejectedCommandBuildRegistry<const CAPACITY: usize> {
    slots: [Option<RejectedCommandBuild>; CAPACITY],
    close_index: usize,
    occupied: usize,
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

/// 🪢️ `serde` kept alongside `ToValue`/`FromValue`: `kernel::Event`/`TurnResult` carry this type
/// across the plugin-host `serde_json` wire (`🔌️plugin/🖥️host/🧵️shard/🦀️.rs`'s
/// `serde_json::to_vec(&result.command_ingress)`) and stay on that encoding, not `DslValue` — both
/// derives must produce the same shape, so `#[serde(rename_all = "camelCase")]` mirrors `#[value(…)]`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
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

/// 🚥️ `serde` kept alongside `ToValue`/`FromValue` — same wire-sharing reason as
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
