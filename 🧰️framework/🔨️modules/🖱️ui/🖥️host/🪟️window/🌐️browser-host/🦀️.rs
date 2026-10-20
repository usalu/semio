//! 🌐️ Browser host module: the host port and types, the canvas host lifecycle, step, staging and close and the helpers with the linear memory port, with the shared imports of the former inline module. See [`super`].

use super::*;
use crate::event::{decode_browser_host_event, BrowserHostEvent, CanvasId, ListenerId};

#[path = "🔌️browser-host-port-and-types/🦀️.rs"]
mod browser_host_port_and_types;
pub use browser_host_port_and_types::{
    BrowserHostError, BrowserHostPort, BrowserHostProgress, BrowserHostStep, BrowserHostUnavailable,
    BrowserWindowDelegate, ClipboardOperation, ClipboardRequest, FrameSchedulerEnvelope,
    BROWSER_HOST_EVENT_ENVELOPE_BYTES, BROWSER_HOST_INITIAL_POLL_BYTES,
    BROWSER_HOST_MAX_ENCODED_EVENT_BYTES, BROWSER_HOST_MAX_EVENT_BODY_BYTES,
    BROWSER_HOST_MAX_PAGE_BODY_BYTES, BROWSER_HOST_OPERATION_ATTACH,
    BROWSER_HOST_OPERATION_CLIPBOARD_READ, BROWSER_HOST_OPERATION_CLIPBOARD_WRITE,
    BROWSER_HOST_OPERATION_CURSOR, BROWSER_HOST_OPERATION_DETACH, BROWSER_HOST_OPERATION_FRAME,
    BROWSER_HOST_PAGE_ENVELOPE_BYTES,
};
use browser_host_port_and_types::{PendingEvent, PendingPage, PendingReply};

#[path = "🎞️canvas-host-lifecycle/🦀️.rs"]
mod canvas_host_lifecycle;
pub use canvas_host_lifecycle::CanvasHost;

#[path = "⏩️canvas-host-step-and-events/🦀️.rs"]
mod canvas_host_step_and_events;

#[path = "🔚️canvas-host-staging-and-close/🦀️.rs"]
mod canvas_host_staging_and_close;

#[path = "🧰️browser-host-helpers-and-linear-memory-port/🦀️.rs"]
mod browser_host_helpers_and_linear_memory_port;
use browser_host_helpers_and_linear_memory_port::{
    canvas_payload, cursor_tag, message_body_bytes, one_credit, parse_listener, progress, request,
    unavailable_from_reply, validate_budget, validate_listener_handle,
};
pub use browser_host_helpers_and_linear_memory_port::LinearMemoryBrowserHostPort;
#[cfg(feature = "testkit")]
pub use browser_host_helpers_and_linear_memory_port::linear_memory_test_import;
