//! 🏠️ The composition root — ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, packet
//! `os-host`, master plan §5. It composes; it does not implement half an operating system. Product
//! behaviour (dock, tutorial playback, world3d/node-graph/board input, chrome painting) stays exactly
//! where it already lives — `AppRuntime` (`🦀️.rs`) and the `🧱️elements/` co-location dirs — this
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
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicUsize, Ordering};
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
    use wasm_bindgen::JsCast;
    js_sys::Reflect::get(&js_sys::global(), &wasm_bindgen::JsValue::from_str("performance")).ok().and_then(|value| value.dyn_into::<web_sys::Performance>().ok()).map(|performance| performance.now()).unwrap_or(0.0)
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
    /// ⏱️ P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): monotonically incremented
    /// once per `redraw()` call — the `Generation` `winit_app.rs` stamps its
    /// `semio_framework_trace::Watchdog` with, so two consecutive frame-callback overruns are
    /// distinguishable events in `Watchdog::violations()`, not one indistinguishable repeat.
    pub frame_generation: u64,
    pub frame_ready: bool,
    pub(crate) cursor_wake_requested: Option<crate::infinite_world::world::WorldCursorWakeToken>,
    pub(crate) platform_fullscreen: Option<bool>,
    pub(crate) present_fault: Option<String>,
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
    pub(crate) surface_resize: crate::surface_lane::SurfaceResizeAuthority,
}

struct OsHostRetirementState {
    runtime: Option<RuntimeMailbox>,
    presenter: Option<AppPresenter>,
    scheduler: Option<FrameScheduler>,
    kernel: Option<AppKernelSeam>,
    clock: Option<OsClock>,
    caret: Option<CaretBlink>,
    hot_swap: Option<HotSwapPoll>,
    events: Option<ui_host::EventQueue>,
    ui_token: Option<ui_host::UiThreadToken>,
    snapshot_sink: Option<RenderSnapshotSink>,
    frame_build: Option<crate::frame_job::FrameBuildHandle>,
    surface_resize: Option<crate::surface_lane::SurfaceResizeAuthority>,
    engine_surfaces: PairedEngineSurfaceClose,
    raster_uploads: Option<crate::scenes::PendingRasterAuthorityClose>,
    cursor_wake_requested: Option<crate::infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    kernel_progress_close: Option<crate::kernel_runtime::KernelCloseHandle>,
}

pub(crate) struct OsHostRetirement {
    state: Option<Box<OsHostRetirementState>>,
    abandonment: Option<OsHostRetirementAbandonment>,
}

const OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY: usize = 64;

#[derive(Clone, Copy)]
struct OsHostRetirementAbandonment {
    slot: u8,
    generation: u64,
}

struct OsHostRetirementAbandonmentSlot {
    generation: AtomicU64,
    exhausted: AtomicBool,
    owner: AtomicPtr<OsHostRetirementState>,
}

impl OsHostRetirementAbandonmentSlot {
    const fn new() -> Self {
        Self { generation: AtomicU64::new(0), exhausted: AtomicBool::new(false), owner: AtomicPtr::new(std::ptr::null_mut()) }
    }
}

static OS_HOST_RETIREMENT_ABANDONMENTS: [OsHostRetirementAbandonmentSlot; OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY] = [const { OsHostRetirementAbandonmentSlot::new() }; OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY];
static OS_HOST_RETIREMENT_ABANDONMENT_SCAN: AtomicUsize = AtomicUsize::new(0);
static OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED: AtomicUsize = AtomicUsize::new(0);

fn os_host_retirement_reservation_marker() -> *mut OsHostRetirementState {
    std::ptr::NonNull::<OsHostRetirementState>::dangling().as_ptr()
}

fn reserve_os_host_retirement_abandonment() -> Option<OsHostRetirementAbandonment> {
    let marker = os_host_retirement_reservation_marker();
    for (index, slot) in OS_HOST_RETIREMENT_ABANDONMENTS.iter().enumerate() {
        if slot.exhausted.load(Ordering::Acquire) || slot.owner.compare_exchange(std::ptr::null_mut(), marker, Ordering::AcqRel, Ordering::Acquire).is_err() {
            continue;
        }
        let current = slot.generation.load(Ordering::Acquire);
        let Some(generation) = current.checked_add(1).filter(|generation| *generation != 0) else {
            slot.exhausted.store(true, Ordering::Release);
            slot.owner.store(std::ptr::null_mut(), Ordering::Release);
            continue;
        };
        if slot.generation.compare_exchange(current, generation, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            return Some(OsHostRetirementAbandonment { slot: index as u8, generation });
        }
        slot.owner.store(std::ptr::null_mut(), Ordering::Release);
    }
    None
}

fn release_os_host_retirement_abandonment(token: OsHostRetirementAbandonment) -> bool {
    let Some(slot) = OS_HOST_RETIREMENT_ABANDONMENTS.get(usize::from(token.slot)) else {
        return false;
    };
    slot.generation.load(Ordering::Acquire) == token.generation && slot.owner.compare_exchange(os_host_retirement_reservation_marker(), std::ptr::null_mut(), Ordering::AcqRel, Ordering::Acquire).is_ok()
}

fn publish_os_host_retirement_abandonment(token: OsHostRetirementAbandonment, state: Box<OsHostRetirementState>) -> Result<(), Box<OsHostRetirementState>> {
    let Some(slot) = OS_HOST_RETIREMENT_ABANDONMENTS.get(usize::from(token.slot)) else {
        return Err(state);
    };
    if slot.generation.load(Ordering::Acquire) != token.generation {
        return Err(state);
    }
    let pointer = Box::into_raw(state);
    match slot.owner.compare_exchange(os_host_retirement_reservation_marker(), pointer, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => {
            OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.fetch_add(1, Ordering::AcqRel);
            Ok(())
        }
        Err(_) => Err(unsafe { Box::from_raw(pointer) }),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PairedEngineSurfaceClosePhase {
    Scan,
    BeginCpu,
    BeginGpu,
    Cpu,
    Gpu,
    Witness,
    Advance,
    Terminal,
}

struct PairedEngineSurfaceClose {
    operation: semio_framework_trace::OperationId,
    sequence: u64,
    scan: usize,
    token: Option<crate::engine_canvas::EngineSurfaceToken>,
    cpu_present: bool,
    gpu_present: bool,
    phase: PairedEngineSurfaceClosePhase,
    faulted: bool,
}

impl PairedEngineSurfaceClose {
    fn new() -> Self {
        Self { operation: semio_framework_trace::allocate_operation_id(), sequence: 0, scan: 0, token: None, cpu_present: false, gpu_present: false, phase: PairedEngineSurfaceClosePhase::Scan, faulted: false }
    }

    fn close_step(&mut self, runtime: &RuntimeMailbox, presenter: &mut AppPresenter) -> bool {
        if self.faulted {
            return false;
        }
        match self.phase {
            PairedEngineSurfaceClosePhase::Scan => {
                if self.scan == crate::engine_canvas::ENGINE_SURFACE_CAPACITY {
                    self.phase = PairedEngineSurfaceClosePhase::Terminal;
                    return false;
                }
                let Ok(cpu) = crate::engine_canvas::engine_surface_token_at(self.scan) else {
                    return false;
                };
                let gpu = presenter.engine_surface_token_at(self.scan);
                if cpu.is_some() && gpu.is_some() && cpu != gpu {
                    self.faulted = true;
                    return false;
                }
                self.token = cpu.or(gpu);
                self.cpu_present = cpu.is_some();
                self.gpu_present = gpu.is_some();
                self.phase = if self.token.is_some() { PairedEngineSurfaceClosePhase::BeginCpu } else { PairedEngineSurfaceClosePhase::Advance };
            }
            PairedEngineSurfaceClosePhase::BeginCpu => {
                let Some(token) = self.token else {
                    self.faulted = true;
                    return false;
                };
                if self.cpu_present {
                    match runtime.begin_engine_surface_close(token) {
                        Ok(true) => {}
                        Ok(false) => {
                            self.faulted = true;
                            return false;
                        }
                        Err(()) => return false,
                    }
                }
                self.phase = PairedEngineSurfaceClosePhase::BeginGpu;
            }
            PairedEngineSurfaceClosePhase::BeginGpu => {
                let Some(token) = self.token else {
                    self.faulted = true;
                    return false;
                };
                if self.gpu_present && !presenter.begin_engine_surface_close(token) {
                    self.faulted = true;
                    return false;
                }
                self.phase = PairedEngineSurfaceClosePhase::Cpu;
            }
            PairedEngineSurfaceClosePhase::Cpu => {
                let Some(token) = self.token else {
                    self.faulted = true;
                    return false;
                };
                if self.cpu_present && !runtime.close_engine_surface_step(token, self.operation, &mut self.sequence) {
                    return false;
                }
                self.phase = PairedEngineSurfaceClosePhase::Gpu;
            }
            PairedEngineSurfaceClosePhase::Gpu => {
                let Some(token) = self.token else {
                    self.faulted = true;
                    return false;
                };
                if self.gpu_present {
                    match presenter.close_engine_surface_step(token) {
                        Ok(true) => {}
                        Ok(false) => return false,
                        Err(_) => {
                            self.faulted = true;
                            return false;
                        }
                    }
                }
                self.phase = PairedEngineSurfaceClosePhase::Witness;
            }
            PairedEngineSurfaceClosePhase::Witness => {
                let Some(token) = self.token else {
                    self.faulted = true;
                    return false;
                };
                if self.cpu_present {
                    match runtime.engine_surface_terminal_is_empty(token) {
                        Ok(true) => {}
                        Ok(false) => {
                            self.faulted = true;
                            return false;
                        }
                        Err(()) => return false,
                    }
                }
                if self.gpu_present && !presenter.engine_surface_terminal_is_empty(token) {
                    self.faulted = true;
                    return false;
                }
                self.phase = PairedEngineSurfaceClosePhase::Advance;
            }
            PairedEngineSurfaceClosePhase::Advance => {
                let Some(next) = self.scan.checked_add(1) else {
                    self.faulted = true;
                    return false;
                };
                self.scan = next;
                self.token = None;
                self.cpu_present = false;
                self.gpu_present = false;
                self.phase = PairedEngineSurfaceClosePhase::Scan;
            }
            PairedEngineSurfaceClosePhase::Terminal => return true,
        }
        false
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == PairedEngineSurfaceClosePhase::Terminal && self.token.is_none() && !self.cpu_present && !self.gpu_present && !self.faulted
    }
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
            frame_generation: 0,
            frame_ready: false,
            cursor_wake_requested: None,
            platform_fullscreen: None,
            present_fault: None,
            events: ui_host::EventQueue::new(),
            ui_token: ui_host::UiThreadToken::mint_for_host(),
            snapshot_sink: RenderSnapshotSink::new(RenderSnapshot::new(0, semio_framework_trace::Generation(0), 0, CursorRequest::Default, None)),
            frame_build: crate::frame_job::FrameBuildHandle::new(),
            surface_resize: crate::surface_lane::SurfaceResizeAuthority::new(semio_framework_trace::allocate_operation_id()),
        }
    }

    /// ⏱️ Convenience — every `winit_app.rs` callback needs "now" in this host's clock at least once.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn now_seconds(&self) -> f64 {
        self.clock.now_seconds()
    }

    pub(crate) fn try_into_retirement(self) -> Result<OsHostRetirement, Self> {
        let Some(abandonment) = reserve_os_host_retirement_abandonment() else {
            return Err(self);
        };
        let Self { runtime, presenter, scheduler, kernel, clock, caret, hot_swap, frame_generation: _, frame_ready: _, cursor_wake_requested, platform_fullscreen: _, present_fault: _, events, ui_token, snapshot_sink, frame_build, surface_resize } =
            self;
        let state = OsHostRetirementState {
            runtime: Some(runtime),
            presenter: Some(presenter),
            scheduler: Some(scheduler),
            kernel: Some(kernel),
            clock: Some(clock),
            caret: Some(caret),
            hot_swap: Some(hot_swap),
            events: Some(events),
            ui_token: Some(ui_token),
            snapshot_sink: Some(snapshot_sink),
            frame_build: Some(frame_build),
            surface_resize: Some(surface_resize),
            engine_surfaces: PairedEngineSurfaceClose::new(),
            raster_uploads: Some(crate::scenes::begin_pending_raster_authority_close()),
            cursor_wake_requested,
            #[cfg(not(target_arch = "wasm32"))]
            kernel_progress_close: crate::kernel_runtime::KernelClient::get().begin_close_realm().ok(),
        };
        Ok(OsHostRetirement { state: Some(Box::new(state)), abandonment: Some(abandonment) })
    }

    pub(crate) fn retain_cursor_wake_directive(&mut self, token: crate::infinite_world::world::WorldCursorWakeToken) {
        if self.cursor_wake_requested.as_ref().is_none_or(|pending| token.generation() > pending.generation()) {
            self.cursor_wake_requested = Some(token);
        }
    }

    pub(crate) fn take_cursor_wake_directive(&mut self) -> Option<crate::infinite_world::world::WorldCursorWakeToken> {
        self.cursor_wake_requested.take()
    }
}

impl OsHostRetirementState {
    fn close_step(&mut self) -> bool {
        if !crate::surface_lane::MountedSurfaceResizeLane::close_abandoned_step() {
            return false;
        }
        if let Some(surface_resize) = self.surface_resize.as_mut() {
            surface_resize.begin_close();
            if !surface_resize.close_step() || !surface_resize.terminal_is_empty() {
                return false;
            }
            self.surface_resize = None;
            return false;
        }
        if let Some(presenter) = self.presenter.as_mut() {
            if !presenter.close_surface_resize_step() {
                return false;
            }
        }
        if let Some(frame_build) = self.frame_build.as_mut() {
            if !frame_build.close_step() {
                return false;
            }
            if !frame_build.terminal_is_empty() {
                return false;
            }
            self.frame_build = None;
            return false;
        }
        if let Some(raster_uploads) = self.raster_uploads.as_mut() {
            if !raster_uploads.close_step() {
                return false;
            }
            if !raster_uploads.terminal_is_empty() {
                return false;
            }
            self.raster_uploads = None;
            return false;
        }
        if let Some(events) = self.events.as_mut() {
            if !events.close_step() {
                return false;
            }
            self.events = None;
            return false;
        }
        if self.cursor_wake_requested.take().is_some() {
            return false;
        }
        if let Some(presenter) = self.presenter.as_mut() {
            if !presenter.close_cursor_wake_step() {
                return false;
            }
            if !matches!(presenter.close_frame_owners_step(), Ok(true)) {
                return false;
            }
        }
        if !self.engine_surfaces.terminal_is_empty() {
            let (Some(runtime), Some(presenter)) = (self.runtime.as_ref(), self.presenter.as_mut()) else {
                return false;
            };
            if !self.engine_surfaces.close_step(runtime, presenter) {
                return false;
            }
            if !self.engine_surfaces.terminal_is_empty() {
                return false;
            }
        }
        if let Some(runtime) = self.runtime.as_ref() {
            if !runtime.close_input_step() {
                return false;
            }
        }
        if let Some(presenter) = self.presenter.as_mut() {
            if !matches!(presenter.close_world_owners_step(), Ok(true)) {
                return false;
            }
            if !presenter.world_owners_terminal_is_empty() || !presenter.engine_surfaces_terminal_is_empty() {
                return false;
            }
        }
        if let Some(runtime) = self.runtime.as_ref() {
            if !runtime.close_world3d_dynamic_step() {
                return false;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(close) = self.kernel_progress_close.as_ref() {
            match close.poll() {
                crate::kernel_runtime::KernelCloseStatus::Complete => {
                    self.kernel_progress_close = None;
                    return false;
                }
                crate::kernel_runtime::KernelCloseStatus::Pending | crate::kernel_runtime::KernelCloseStatus::AdmissionBlocked => return false,
                crate::kernel_runtime::KernelCloseStatus::Fault => return false,
            }
        }
        for owner in [&mut self.snapshot_sink as &mut dyn RetirementOwner, &mut self.ui_token, &mut self.hot_swap, &mut self.caret, &mut self.clock, &mut self.kernel, &mut self.scheduler, &mut self.runtime, &mut self.presenter] {
            if owner.retire() {
                return false;
            }
        }
        self.terminal_is_empty()
    }

    fn terminal_is_empty(&self) -> bool {
        self.runtime.is_none()
            && self.presenter.is_none()
            && self.scheduler.is_none()
            && self.kernel.is_none()
            && self.clock.is_none()
            && self.caret.is_none()
            && self.hot_swap.is_none()
            && self.events.is_none()
            && self.ui_token.is_none()
            && self.snapshot_sink.is_none()
            && self.frame_build.is_none()
            && self.surface_resize.is_none()
            && self.raster_uploads.is_none()
            && self.engine_surfaces.terminal_is_empty()
            && self.cursor_wake_requested.is_none()
            && {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    self.kernel_progress_close.is_none()
                }
                #[cfg(target_arch = "wasm32")]
                {
                    true
                }
            }
    }
}

impl Drop for OsHostRetirementState {
    fn drop(&mut self) {
        debug_assert!(self.terminal_is_empty(), "OsHostRetirementState must reach terminal-empty before release");
    }
}

impl OsHostRetirement {
    pub(crate) fn close_abandoned_step() -> bool {
        if OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.load(Ordering::Acquire) == 0 {
            return true;
        }
        let index = match OS_HOST_RETIREMENT_ABANDONMENT_SCAN.fetch_update(Ordering::AcqRel, Ordering::Acquire, |index| Some(index.checked_add(1).map_or(0, |next| next % OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY))) {
            Ok(index) | Err(index) => index % OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY,
        };
        let slot = &OS_HOST_RETIREMENT_ABANDONMENTS[index];
        let pointer = slot.owner.load(Ordering::Acquire);
        if pointer.is_null() || pointer == os_host_retirement_reservation_marker() {
            return false;
        }
        if slot.owner.compare_exchange(pointer, os_host_retirement_reservation_marker(), Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        let mut state = unsafe { Box::from_raw(pointer) };
        if state.close_step() && state.terminal_is_empty() {
            drop(state);
            slot.owner.store(std::ptr::null_mut(), Ordering::Release);
            OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.fetch_sub(1, Ordering::AcqRel);
        } else {
            slot.owner.store(Box::into_raw(state), Ordering::Release);
        }
        false
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if !Self::close_abandoned_step() {
            return false;
        }
        let Some(state) = self.state.as_mut() else {
            return self.terminal_is_empty();
        };
        if !state.close_step() || !state.terminal_is_empty() {
            return false;
        }
        let state = self.state.take();
        drop(state);
        let Some(token) = self.abandonment.take() else {
            return false;
        };
        if !release_os_host_retirement_abandonment(token) {
            std::process::abort();
        }
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.state.is_none() && self.abandonment.is_none()
    }
}

impl Drop for OsHostRetirement {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else {
            if let Some(token) = self.abandonment.take() {
                if !release_os_host_retirement_abandonment(token) {
                    std::process::abort();
                }
            }
            return;
        };
        let Some(token) = self.abandonment.take() else {
            let _state = std::mem::ManuallyDrop::new(state);
            std::process::abort();
        };
        if let Err(state) = publish_os_host_retirement_abandonment(token, state) {
            let _state = std::mem::ManuallyDrop::new(state);
            std::process::abort();
        }
    }
}

trait RetirementOwner {
    fn retire(&mut self) -> bool;
}

impl<T> RetirementOwner for Option<T> {
    fn retire(&mut self) -> bool {
        self.take().is_some()
    }
}

#[cfg(test)]
fn terminal_os_host_retirement_state() -> OsHostRetirementState {
    OsHostRetirementState {
        runtime: None,
        presenter: None,
        scheduler: None,
        kernel: None,
        clock: None,
        caret: None,
        hot_swap: None,
        events: None,
        ui_token: None,
        snapshot_sink: None,
        frame_build: None,
        surface_resize: None,
        engine_surfaces: PairedEngineSurfaceClose {
            operation: semio_framework_trace::allocate_operation_id(),
            sequence: 0,
            scan: crate::engine_canvas::ENGINE_SURFACE_CAPACITY,
            token: None,
            cpu_present: false,
            gpu_present: false,
            phase: PairedEngineSurfaceClosePhase::Terminal,
            faulted: false,
        },
        raster_uploads: None,
        cursor_wake_requested: None,
        #[cfg(not(target_arch = "wasm32"))]
        kernel_progress_close: None,
    }
}

#[cfg(test)]
#[test]
fn interrupted_host_retirement_is_rediscovered_and_fixed_registry_refuses_max_plus_one() {
    let token = reserve_os_host_retirement_abandonment().expect("fixed host retirement reservation");
    let stale = OsHostRetirementAbandonment { slot: token.slot, generation: token.generation.checked_add(1).expect("test generation") };
    assert!(!release_os_host_retirement_abandonment(stale));
    drop(OsHostRetirement { state: Some(Box::new(terminal_os_host_retirement_state())), abandonment: Some(token) });
    assert_eq!(OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.load(Ordering::Acquire), 1);
    let mut turns = 0usize;
    while !OsHostRetirement::close_abandoned_step() {
        turns += 1;
        assert!(turns <= OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY);
    }
    assert_eq!(OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.load(Ordering::Acquire), 0);
    let mut reservations = [None; OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY];
    for reservation in &mut reservations {
        *reservation = reserve_os_host_retirement_abandonment();
        assert!(reservation.is_some());
    }
    assert!(reserve_os_host_retirement_abandonment().is_none());
    for reservation in reservations.into_iter().flatten() {
        assert!(release_os_host_retirement_abandonment(reservation));
    }
}

//#endregion 🏠️OsHost

//#endregion 🔖️OsHost
