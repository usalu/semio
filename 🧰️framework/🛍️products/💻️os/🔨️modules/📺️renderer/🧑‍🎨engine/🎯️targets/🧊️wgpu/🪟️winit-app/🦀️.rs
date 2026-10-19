//! 🪟️ The event loop that does nothing when nothing changed — ticket
//! `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, packet `os-host`, master plan §5.
//! `WinitApp` is the `ApplicationHandler` **replacing `SemioApp`**: `ControlFlow::Poll`, the
//! `RedrawRequested` re-arm and `start_frame_loop` are all deleted (see this crate's
//! `📓️terra-os-host-report.md` redraw audit for the exact old line numbers and what replaced them).
//!
//! **Why this is a hand-written `ApplicationHandler` rather than `ui_host::window::NativeHost<OsHost>`
//! — a deliberate, evidence-based deviation, not an oversight.** `NativeHost::new(delegate: D)` takes
//! an already-fully-constructed delegate; its own `resumed` creates the window *itself*, after the
//! event loop is already running. This crate's boot order is the opposite: `GpuContext::from_window`
//! (this file's `AppRuntime`'s device init) needs a **live** `winit::window::Window` to construct, and
//! `OsHost` cannot exist before `AppRuntime` does — so the window must be created first, `AppRuntime`
//! booted async against it second, and only THEN can `OsHost::new` run. `ui_host::WindowDelegate`'s
//! own trait — `scheduler_mut`, `handle_event`, `handle_metrics`, `redraw`, `close_requested` — is
//! still exactly what [`OsHost`] implements below (see `impl WindowDelegate for OsHost`); this file
//! only re-implements `NativeHost`'s specific control-flow *orchestration* (`control_flow_for`/
//! `should_request_redraw`, both reused verbatim from `ui_host` where cfg allows — see inline calls)
//! around the two-phase boot handshake `SemioApp` already solved correctly. Reported as a real gap in
//! `ui_host::WindowDelegate` worth closing upstream: a `WindowDelegate` construction hook that receives
//! the just-created `Window` (or an explicit two-phase `NativeHost::new_pending()` API) would let a
//! future revision drop this file's hand-rolled `ApplicationHandler` entirely.

use crate::os_host::OsHost;
use semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState;
use semio_framework_os_renderer_wgpu_host_plumbing::input_wire::{event_modifiers_to_pointer, key_action_from_dispatch, pointer_button_to_i16};
use ui_host::{RedrawOutcome, WindowDelegate, WindowMetrics};
use ui_render::{CursorRequest, DispatchEvent, EventModifiers, ImeEvent, InvalidationReason, PointerButton, PointerInfo, TextEditTarget};

#[path = "🎞️frame-generation-and-os-host/🦀️.rs"]
mod frame_generation_and_os_host;
use frame_generation_and_os_host::{
    advance_frame_generation, collect_scheduler_redraw, invalidate_scheduler_redraw,
};
#[cfg(test)]
use frame_generation_and_os_host::{enqueue_host_event, enqueue_host_metrics, FrameGenerationHold};
#[cfg(not(target_arch="wasm32"))]
use frame_generation_and_os_host::ime_event_from_winit;
#[cfg(not(target_arch="wasm32"))]
pub use frame_generation_and_os_host::run_native;

#[path = "🚀️native-winit-app/🦀️.rs"]
mod native_winit_app;
#[cfg(not(target_arch="wasm32"))]
use native_winit_app::native;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{HostUserEvent, WinitApp};

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs"]
mod p3c_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-winit-app-callback-latency/🦀️.rs"]
mod callback_latency_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🔢️frame-generation-hold/🦀️.rs"]
mod frame_generation_hold_tests;
