//! 🏠️ The composition root — ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, packet
//! `os-host`, master plan §5. It composes; it does not implement half an operating system. Product
//! behaviour (dock, tutorial playback, world3d/node-graph/board input, chrome painting) stays exactly
//! where it already lives — `AppRuntime` (`📦️glue.rs`) and the `🧱️elements/` co-location dirs — this
//! file only OWNS what wires them to the new scheduling/kernel seams.
//!
//! **Deviation from the master plan's literal `OsHost { engine, backend, scheduler, surfaces, … }`
//! sketch — recorded here, and in `📓️terra-os-host-report.md`'s own deviations section.** That shape
//! names `ui_render::FrameEngine`/`ui_host::ActiveBackend`, i.e. the NEW `Element`-tree pipeline. This
//! crate's actual rendering is still the old immediate-mode `DrawList` pipeline living inside
//! `AppRuntime` (`self.draw`/`self.overlay`/`self.gpu`) — migrating that onto `Element`/`FrameEngine`
//! is the `render-elements`/`runtime-*` packets' job (master plan §2/§4), not this one's. Instantiating
//! `FrameEngine`/`ActiveBackend` fields here today, with nothing yet producing real `Element`s to feed
//! them, would be dead scaffolding wired to nothing — worse than the honest alternative below: `OsHost`
//! owns the scheduling/kernel seam this packet is actually chartered to build (`FrameScheduler`,
//! `KernelSeam`, the deadline sources), composed **around** the existing, still-`DrawList`-based
//! `AppRuntime`, so the headline claim — idle windows render zero frames — is true today, without
//! waiting on the Element migration to land first.

use crate::deadlines::{CaretBlink, HotSwapPoll};
use crate::kernel_seam::{default_intent_exchange, AppKernelSeam};
use crate::render_snapshot::{RenderSnapshot, RenderSnapshotSink};
use crate::{AppPresenter, RuntimeMailbox};
use std::collections::HashMap;
use ui_render::{CursorRequest, FrameScheduler};

//#region 🔖️OsHost

//#region 🕰️OsClock

/// 🕰️ `os_host`'s own wall-time source, mirroring `ui_host::window::native::MonotonicClock`/
/// `BrowserClock` primitive-for-primitive (`Instant::now()` native, `performance.now()` wasm) — see
/// `deadlines.rs`'s own module docstring for why a *separately constructed* clock with the same
/// origin epoch, rather than reading `ui_host`'s private one (there is no accessor — a real gap, see
/// the report), is an accepted approximation: both clocks are captured within microseconds of each
/// other during the same boot sequence, negligible against this file's sub-second deadlines.
/// **Not** `crate::app_now_ms()` — that function is wall-clock epoch (`SystemTime`/`Date.now()`),
/// unrelated to either clock `FrameScheduler::should_render`'s `now` must be measured against.
pub struct OsClock {
    #[cfg(not(target_arch = "wasm32"))]
    origin: std::time::Instant,
    #[cfg(target_arch = "wasm32")]
    origin_ms: f64,
}

impl OsClock {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        Self { origin: std::time::Instant::now() }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        Self { origin_ms: performance_now_ms() }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(not(target_arch = "wasm32"))]
    pub fn now_seconds(&self) -> f64 {
        self.origin.elapsed().as_secs_f64()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(target_arch = "wasm32")]
    pub fn now_seconds(&self) -> f64 {
        (performance_now_ms() - self.origin_ms) / 1000.0
    }
}

impl Default for OsClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
fn performance_now_ms() -> f64 {
    web_sys::window().and_then(|window| window.performance()).map(|performance| performance.now()).unwrap_or(0.0)
}

//#endregion 🕰️OsClock

//#region 🏠️OsHost

/// 🏠️ Owns lifecycle/composition: the product runtime, the `FrameScheduler` that ends continuous
/// redraw, the `KernelSeam` that ends the renderer owning the actor kernel, and every deadline
/// source's bookkeeping. `window`/`kernel` naming mirrors the master plan's
/// `OsHost { engine, backend, scheduler, surfaces, shell_model, kernel, theme, window, … }` sketch as
/// closely as this crate's still-`DrawList` reality allows — see this file's own module docstring for
/// the named deviation.
///
/// **Why `scheduler` lives here and not on `AppRuntime`.** `ui_host::WindowDelegate::scheduler_mut`
/// returns a plain `&mut FrameScheduler` — a real, exclusively-owned reference. `AppRuntime` is
/// addressed by worker jobs through `RuntimeMailbox`, which deliberately exposes no borrow that can
/// cross a suspension. `OsHost`, by contrast, is owned exclusively by the event loop, so
/// `&mut self.scheduler`
/// is trivially sound. This is a real, load-bearing reason the composition root is a *separate* type
/// from the product runtime, not just an organizational preference.
pub struct OsHost {
    pub(crate) runtime: RuntimeMailbox,
    pub(crate) presenter: AppPresenter,
    pub scheduler: FrameScheduler,
    pub kernel: AppKernelSeam,
    pub clock: OsClock,
    pub caret: CaretBlink,
    pub hot_swap: HotSwapPoll,
    /// 🕒️ Node-graph wheel-zoom settle tokens, keyed by surface id — `deadlines::arm`/`sweep_expired`.
    pub wheel_zoom_settle: HashMap<String, f64>,
    /// 🕒️ World3D camera-settle tokens, keyed by surface id — `deadlines::arm`/`sweep_expired`.
    pub camera_settle: HashMap<String, f64>,
    /// ⏱️ P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): monotonically incremented
    /// once per `redraw()` call — the `Generation` `winit_app.rs` stamps its
    /// `semio_framework_trace::Watchdog` with, so two consecutive frame-callback overruns are
    /// distinguishable events in `Watchdog::violations()`, not one indistinguishable repeat.
    pub frame_generation: u64,
    pub frame_ready: bool,
    /// 📬️ P3a (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): the fixed-capacity enqueue-only
    /// sink `WindowDelegate::handle_event`/`handle_metrics` write into instead of immediately spawning
    /// a heap-allocated future per event. `redraw()` drains it once per frame and dispatches the whole
    /// batch through one deferred reduction on the process worker pool.
    pub events: ui_host::EventQueue,
    /// 🎫️ Minted once, here, at `OsHost` construction — `OsHost` is owned exclusively by whatever
    /// drives this crate's hand-rolled `WinitApp` event loop (see that file's own docstring on why it
    /// cannot use `ui_host::NativeHost` directly), so this token's presence on `OsHost` is itself the
    /// proof every `WindowDelegate` callback below runs on the thread that owns the window.
    pub ui_token: ui_host::UiThreadToken,
    /// 📸️ P3a (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): the atomically-published frame
    /// artifact. Worker-built prepared packets and UI-only presentation share its generation and
    /// revision vocabulary; the retained dispatch-tree field remains an explicit follow-up.
    pub snapshot_sink: RenderSnapshotSink,
    /// 🧵️ P3b (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): the non-blocking poll/resubmit
    /// handle for the deadline-scan job plus worker-owned `AppRuntime::frame` transaction.
    pub(crate) frame_build: crate::frame_job::FrameBuildHandle,
}

impl OsHost {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn new(runtime: RuntimeMailbox, presenter: AppPresenter) -> Self {
        Self {
            runtime,
            presenter,
            scheduler: FrameScheduler::new(),
            kernel: AppKernelSeam::new(default_intent_exchange),
            clock: OsClock::new(),
            caret: CaretBlink::new(),
            hot_swap: HotSwapPoll::new(),
            wheel_zoom_settle: HashMap::new(),
            camera_settle: HashMap::new(),
            frame_generation: 0,
            frame_ready: false,
            events: ui_host::EventQueue::new(),
            ui_token: ui_host::UiThreadToken::mint_for_host(),
            snapshot_sink: RenderSnapshotSink::new(RenderSnapshot::new(0, semio_framework_trace::Generation(0), 0, CursorRequest::Default, None)),
            frame_build: crate::frame_job::FrameBuildHandle::new(),
        }
    }

    /// ⏱️ Convenience — every `winit_app.rs` callback needs "now" in this host's clock at least once.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn now_seconds(&self) -> f64 {
        self.clock.now_seconds()
    }
}

//#endregion 🏠️OsHost

//#endregion 🔖️OsHost
