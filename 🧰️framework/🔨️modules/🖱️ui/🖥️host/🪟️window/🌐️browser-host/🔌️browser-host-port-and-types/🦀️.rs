//! 🔌️ Browser host port and types: the browser host byte limits and operations, the host port, the unavailable and error enums, the clipboard request, the scheduler envelope, the progress and step records and the browser window delegate. See [`super`].

use super::*;

pub const BROWSER_HOST_MAX_EVENT_BODY_BYTES: usize = 1_024;

pub const BROWSER_HOST_EVENT_ENVELOPE_BYTES: usize = 27;

pub const BROWSER_HOST_MAX_ENCODED_EVENT_BYTES: usize = BROWSER_HOST_MAX_EVENT_BODY_BYTES + BROWSER_HOST_EVENT_ENVELOPE_BYTES;

pub const BROWSER_HOST_PAGE_ENVELOPE_BYTES: usize = 18;

pub const BROWSER_HOST_MAX_PAGE_BODY_BYTES: usize = BROWSER_HOST_MAX_ENCODED_EVENT_BYTES - BROWSER_HOST_PAGE_ENVELOPE_BYTES;

pub const BROWSER_HOST_INITIAL_POLL_BYTES: usize = BROWSER_HOST_MAX_EVENT_BODY_BYTES;

pub const BROWSER_HOST_OPERATION_ATTACH: u16 = 1_793;

pub const BROWSER_HOST_OPERATION_FRAME: u16 = 1_794;

pub const BROWSER_HOST_OPERATION_CURSOR: u16 = 1_795;

pub const BROWSER_HOST_OPERATION_CLIPBOARD_READ: u16 = 1_796;

pub const BROWSER_HOST_OPERATION_CLIPBOARD_WRITE: u16 = 1_797;

pub const BROWSER_HOST_OPERATION_DETACH: u16 = 1_798;

/// 🔌️ A1 port specialization implemented by an owned host shim.
pub trait BrowserHostPort: AbiPort {}

impl<T: AbiPort> BrowserHostPort for T {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserHostUnavailable {
    Window,
    Document,
    Canvas,
    Clipboard,
    Listener,
    Callback,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserHostError {
    Abi(AbiErrorCode),
    Unavailable(BrowserHostUnavailable),
    MalformedHostReply,
    Closed,
    Busy,
}

impl From<AbiErrorCode> for BrowserHostError {
    fn from(value: AbiErrorCode) -> Self {
        Self::Abi(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipboardOperation {
    Read,
    Write,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClipboardRequest {
    pub request_id: AbiRequestId,
    pub generation: u32,
    pub operation: ClipboardOperation,
    pub text: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameSchedulerEnvelope {
    pub canvas: CanvasId,
    pub generation: u32,
    pub timestamp_ms: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserHostProgress {
    pub completed_units: u32,
    pub total_units: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BrowserHostStep {
    Progress(BrowserHostProgress),
    AwaitingHost,
    Event,
    Frame(FrameSchedulerEnvelope),
    Clipboard { request_id: AbiRequestId, text: Option<String> },
    Unavailable(BrowserHostUnavailable),
    Closed,
}

pub(super) struct PendingEvent {
    pub(super) event: AbiEvent,
    pub(super) inspected_bytes: usize,
}

pub(super) struct PendingReply {
    pub(super) reply: AbiReply,
    pub(super) inspected_bytes: usize,
}

pub(super) struct PendingPage {
    pub(super) page: AbiPage,
    pub(super) inspected_bytes: usize,
}

/// 🧵️ Browser delegate split preserving worker frame preparation.
pub trait BrowserWindowDelegate: WindowDelegate {
    fn enqueue_browser_frame(&mut self, reason: InvalidationReason);
    fn present_browser_frame(&mut self) -> RedrawOutcome;
}
