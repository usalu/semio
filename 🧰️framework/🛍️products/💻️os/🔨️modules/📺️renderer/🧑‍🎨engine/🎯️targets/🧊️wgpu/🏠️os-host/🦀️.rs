//! 🏠️ Composition and retirement of the renderer runtime, presenter, scheduler and deadline sources.
//! AppMailbox owns worker dispatch; the host presents completed frames and schedules wakeups.

use semio_framework_os_renderer_wgpu_host_plumbing::deadlines::HotSwapPoll;
use semio_framework_os_renderer_wgpu_host_plumbing::render_snapshot::{RenderSnapshot, RenderSnapshotSink};
use semio_framework_os_renderer_wgpu_app_runtime::AppMailbox;
use semio_framework_os_renderer_wgpu_frame_presentation::AppPresenter;
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
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        Self { origin: std::time::Instant::now() }
    }

    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        Self { origin_ms: performance_now_ms() }
    }

    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(not(target_arch = "wasm32"))]
    pub fn now_seconds(&self) -> f64 {
        self.origin.elapsed().as_secs_f64()
    }

    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
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

/// 🏠️ Owns presentation, scheduling and host lifecycle around the worker-owned runtime.
pub struct OsHost {
    retained: semio_framework_job::RetainedCloneGrant,
    pub animation_clock: semio_framework_os_renderer_wgpu_host_plumbing::deadlines::AcceptedAnimationClock,
    pub runtime: AppMailbox,
    pub presenter: AppPresenter,
    pub scheduler: FrameScheduler,
    pub clock: OsClock,
    pub hot_swap: HotSwapPoll,
    /// ⏱️ P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): monotonically incremented
    /// once per `redraw()` call — the `Generation` `winit_app.rs` stamps its
    /// `semio_framework_trace::Watchdog` with, so two consecutive frame-callback overruns are
    /// distinguishable events in `Watchdog::violations()`, not one indistinguishable repeat.
    pub frame_generation: u64,
    pub frame_ready: bool,
    pub cursor_wake_requested: Option<infinite_world::world::WorldCursorWakeToken>,
    pub platform_fullscreen: Option<bool>,
    pub present_fault: Option<String>,
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
    pub original_snapshot: Option<RenderSnapshot>,
    /// 🧵️ P3b (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): the non-blocking poll/resubmit
    /// handle for the deadline-scan job plus worker-owned `AppRuntime::frame` transaction.
    pub frame_build: semio_framework_os_renderer_wgpu_frame_job::frame_job::FrameBuildHandle,
    pub surface_resize: semio_framework_os_renderer_wgpu_host_plumbing::surface_lane::SurfaceResizeAuthority,
    component_surface_close: Option<ComponentSurfaceCloseOwner>,
    asset_decode_owner: Option<AssetDecodeOwner>,
}

/// 🪙️ The declared per-turn authority of the asset decode owner: one bounded decoder step and its paid receipts.
const ASSET_DECODE_TURN_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 4 * 1024 * 1024, maximum_capacity_bytes: 4 * 1024 * 1024, maximum_release_bytes: 16 * 1024 * 1024, maximum_depth: 64 };

/// 🪪️ The root cancellation and payload authority every asset decode session of this host runs under; both retire in paid turns before the host does.
struct AssetDecodeOwner {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    cancel: Option<semio_framework_job::CancelToken>,
    authority: Option<semio_framework_job::JobPayloadAuthority>,
    cancel_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_job::CancelToken>>,
    sequence: u64,
    closing: bool,
}

impl AssetDecodeOwner {
    fn new() -> Self {
        Self { operation: semio_framework_trace::allocate_operation_id(), generation: semio_framework_job::Generation(1), cancel: None, authority: None, cancel_close: None, sequence: 0, closing: false }
    }

    fn terminal_is_empty(&self) -> bool {
        self.cancel.is_none() && self.authority.is_none() && self.cancel_close.is_none()
    }
}

#[cfg(target_arch = "wasm32")]
pub struct BrowserRedrawOutcome {
    pub cursor_css: &'static str,
    pub media_slots:semio_framework_os_renderer_wgpu_host_plumbing::render_snapshot::RenderSnapshotOutputLoan,
}

struct OsHostRetirementState {
    retained: semio_framework_job::RetainedCloneGrant,
    camera_progress: semio_framework_job::RetainedCloneProgress,
    runtime: Option<AppMailbox>,
    presenter: Option<AppPresenter>,
    scheduler: Option<FrameScheduler>,
    clock: Option<OsClock>,
    hot_swap: Option<HotSwapPoll>,
    events: Option<ui_host::EventQueue>,
    ui_token: Option<ui_host::UiThreadToken>,
    snapshot_sink: Option<RenderSnapshotSink>,
    original_snapshot:Option<RenderSnapshot>,
    original_snapshot_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<RenderSnapshot>>,
    frame_build: Option<semio_framework_os_renderer_wgpu_frame_job::frame_job::FrameBuildHandle>,
    surface_resize: Option<semio_framework_os_renderer_wgpu_host_plumbing::surface_lane::SurfaceResizeAuthority>,
    engine_surfaces: PairedEngineSurfaceClose,
    raster_uploads: Option<semio_framework_os_renderer_wgpu_scene_state::PendingRasterAuthorityClose>,
    cursor_wake_requested: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    kernel_progress_close: Option<semio_framework_os_renderer_wgpu_kernel_runtime::KernelCloseHandle>,
}

pub struct OsHostRetirement {
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

pub use semio_framework_os_renderer_wgpu_scene_state::{
    acknowledge_component_surface_close, component_surface_close_occupied, component_surface_close_pending, component_surface_close_terminal, publish_component_surface_close_terminal, request_component_surface_close, take_component_surface_close_request, ComponentSurfaceCloseRequest,
    ComponentSurfaceCloseToken,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ComponentSurfaceClosePhase {
    Asset,
    World,
    Inspect,
    BeginCpu,
    BeginRaster,
    BeginGpu,
    Cpu,
    Gpu,
    Raster,
    Witness,
    Terminal,
}

struct ComponentSurfaceCloseOwner {
    request: ComponentSurfaceCloseRequest,
    operation: semio_framework_trace::OperationId,
    sequence: u64,
    cpu_present: bool,
    gpu_present: bool,
    phase: ComponentSurfaceClosePhase,
    faulted: bool,
    world_fault: Option<infinite_world::world::WorldDynamicFault>,
    world_progress: Option<semio_framework_value::retained_clone::RetainedCloneStep>,
}

impl ComponentSurfaceCloseOwner {
    fn new(request: ComponentSurfaceCloseRequest) -> Self {
        Self { request, operation: semio_framework_trace::allocate_operation_id(), sequence: 0, cpu_present: false, gpu_present: false, phase: ComponentSurfaceClosePhase::Asset, faulted: false, world_fault:None, world_progress:None }
    }

    fn close_asset_world_step(&mut self, runtime: &AppMailbox) {
        if self.faulted {
            return;
        }
        match self.phase {
            ComponentSurfaceClosePhase::Asset => {
                if runtime.close_component_asset_step(&self.request.owner) {
                    self.phase = ComponentSurfaceClosePhase::World;
                }
            }
            ComponentSurfaceClosePhase::World => {
                let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64};
                let Some(budget)=semio_framework_job::default_now_us().and_then(|now|semio_framework_job::StepBudget::from_duration(1,now,semio_framework_job::MAINTENANCE_LANE_WALL_US,grant)) else {
                    self.world_fault=Some(infinite_world::world::WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::WorkLimit));
                    self.faulted=true;return;
                };
                let mut recipient=semio_framework_value::retained_clone::RetainedCloneProgress::default();
                let mut context=semio_framework_job::StepContext::new(self.operation,semio_framework_job::Generation(1),budget,semio_framework_job::root_cancel_token(),semio_framework_job::default_now_us,&mut self.sequence,&mut recipient);
                match runtime.close_component_world_step(&self.request.owner,grant,&mut context) {
                    Ok(step) if step.progress().fits(grant)=>{
                        self.world_progress=Some(step);
                        if matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {self.phase=if self.request.engine_token.is_some(){ComponentSurfaceClosePhase::Inspect}else{ComponentSurfaceClosePhase::Terminal};}
                    },
                    Ok(_)=>{self.world_fault=Some(infinite_world::world::WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));self.faulted=true;},
                    Err(fault)=>{self.world_fault=Some(fault);self.faulted=true;},
                }
            },
            _ => {}
        }
    }

    fn close_step(&mut self, runtime: &AppMailbox, presenter: &mut AppPresenter, retained: semio_framework_job::RetainedCloneGrant) -> bool {
        if self.faulted {
            return false;
        }
        if matches!(self.phase, ComponentSurfaceClosePhase::Asset | ComponentSurfaceClosePhase::World) {
            self.close_asset_world_step(runtime);
            return self.phase == ComponentSurfaceClosePhase::Terminal;
        }
        let Some(engine_token) = self.request.engine_token else { return self.phase == ComponentSurfaceClosePhase::Terminal };
        match self.phase {
            ComponentSurfaceClosePhase::Asset | ComponentSurfaceClosePhase::World => {}
            ComponentSurfaceClosePhase::Inspect => {
                let cpu = semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::engine_surface_token_at(usize::from(engine_token.slot)).ok().flatten();
                let gpu = presenter.engine_surface_token_at(usize::from(engine_token.slot));
                if cpu.is_some_and(|token| token != engine_token) || gpu.is_some_and(|token| token != engine_token) {
                    self.faulted = true;
                    return false;
                }
                self.cpu_present = cpu == Some(engine_token);
                self.gpu_present = gpu == Some(engine_token);
                self.phase = ComponentSurfaceClosePhase::BeginCpu;
            }
            ComponentSurfaceClosePhase::BeginCpu => {
                if self.cpu_present {
                    match runtime.begin_engine_surface_close(engine_token) {
                        Ok(true) => {}
                        Ok(false) => {
                            self.faulted = true;
                            return false;
                        }
                        Err(()) => return false,
                    }
                }
                self.phase = ComponentSurfaceClosePhase::BeginRaster;
            }
            ComponentSurfaceClosePhase::BeginRaster => match presenter.begin_engine_raster_close(&self.request.owner.host_id) {
                Ok(true) => self.phase = ComponentSurfaceClosePhase::BeginGpu,
                Ok(false) => {}
                Err(_) => self.faulted = true,
            },
            ComponentSurfaceClosePhase::BeginGpu => {
                if self.gpu_present && !presenter.begin_engine_surface_close(engine_token) {
                    self.faulted = true;
                    return false;
                }
                self.phase = ComponentSurfaceClosePhase::Cpu;
            }
            ComponentSurfaceClosePhase::Cpu => {
                if self.cpu_present && !runtime.close_engine_surface_step(engine_token, self.operation, &mut self.sequence, retained) {
                    return false;
                }
                self.phase = ComponentSurfaceClosePhase::Gpu;
            }
            ComponentSurfaceClosePhase::Gpu => {
                if self.gpu_present {
                    match presenter.close_engine_surface_step(engine_token) {
                        Ok(true) => {}
                        Ok(false) => return false,
                        Err(_) => {
                            self.faulted = true;
                            return false;
                        }
                    }
                }
                self.phase = ComponentSurfaceClosePhase::Raster;
            }
            ComponentSurfaceClosePhase::Raster => match presenter.close_engine_raster_step(&self.request.owner.host_id) {
                Ok(true) => self.phase = ComponentSurfaceClosePhase::Witness,
                Ok(false) => {}
                Err(_) => self.faulted = true,
            },
            ComponentSurfaceClosePhase::Witness => {
                if self.cpu_present && runtime.engine_surface_terminal_is_empty(engine_token) != Ok(true) {
                    self.faulted = true;
                    return false;
                }
                if self.gpu_present && !presenter.engine_surface_terminal_is_empty(engine_token) {
                    self.faulted = true;
                    return false;
                }
                if !presenter.engine_raster_terminal_is_empty(&self.request.owner.host_id) {
                    self.faulted = true;
                    return false;
                }
                self.phase = ComponentSurfaceClosePhase::Terminal;
            }
            ComponentSurfaceClosePhase::Terminal => return true,
        }
        false
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == ComponentSurfaceClosePhase::Terminal && !self.faulted
    }
}

struct PairedEngineSurfaceClose {
    operation: semio_framework_trace::OperationId,
    sequence: u64,
    scan: usize,
    token: Option<semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken>,
    cpu_present: bool,
    gpu_present: bool,
    phase: PairedEngineSurfaceClosePhase,
    faulted: bool,
}

impl PairedEngineSurfaceClose {
    fn new() -> Self {
        Self { operation: semio_framework_trace::allocate_operation_id(), sequence: 0, scan: 0, token: None, cpu_present: false, gpu_present: false, phase: PairedEngineSurfaceClosePhase::Scan, faulted: false }
    }

    fn close_step(&mut self, runtime: &AppMailbox, presenter: &mut AppPresenter, retained: semio_framework_job::RetainedCloneGrant) -> bool {
        if self.faulted {
            return false;
        }
        match self.phase {
            PairedEngineSurfaceClosePhase::Scan => {
                if self.scan == semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::ENGINE_SURFACE_CAPACITY {
                    self.phase = PairedEngineSurfaceClosePhase::Terminal;
                    return false;
                }
                let Ok(cpu) = semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::engine_surface_token_at(self.scan) else {
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
                if self.cpu_present && !runtime.close_engine_surface_step(token, self.operation, &mut self.sequence, retained) {
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
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(runtime: AppMailbox, presenter: AppPresenter, retained: semio_framework_job::RetainedCloneGrant) -> Self {
        semio_framework_os_renderer_wgpu_scene_state::install_scene_camera_authority(retained).expect("host retains the same camera bootstrap authority");
        Self {
            retained,
            animation_clock: semio_framework_os_renderer_wgpu_host_plumbing::deadlines::AcceptedAnimationClock::default(),
            runtime,
            presenter,
            scheduler: FrameScheduler::new(),
            clock: OsClock::new(),
            hot_swap: HotSwapPoll::new(),
            frame_generation: 0,
            frame_ready: false,
            cursor_wake_requested: None,
            platform_fullscreen: None,
            present_fault: None,
            events: ui_host::EventQueue::new(),
            ui_token: ui_host::UiThreadToken::mint_for_host(),
            original_snapshot:None,
            snapshot_sink: RenderSnapshotSink::new(RenderSnapshot::new(0, CursorRequest::Default, semio_framework_ui_wgpu_hosting::cursor::SemioCursor::Default, false, None)),
            frame_build: semio_framework_os_renderer_wgpu_frame_job::frame_job::FrameBuildHandle::new(),
            surface_resize: semio_framework_os_renderer_wgpu_host_plumbing::surface_lane::SurfaceResizeAuthority::new(semio_framework_trace::allocate_operation_id()),
            component_surface_close: None,
            asset_decode_owner: None,
        }
    }

    /// 🌱️ Runs one asset decode turn under this host's own root cancellation and payload authority, born on first use and retired by [`Self::close_asset_decode_step`].
    pub fn with_asset_decode_context<R>(&mut self, run: impl FnOnce(&AppMailbox, &mut semio_framework_job::StepContext<'_>) -> Result<R, semio_framework_job::OriginalTurnFailure>) -> Result<Option<R>, semio_framework_job::OriginalTurnFailure> {
        let owner = self.asset_decode_owner.get_or_insert_with(AssetDecodeOwner::new);
        if owner.closing {
            return Ok(None);
        }
        if owner.cancel.is_none() {
            let Some((cancel, _)) = semio_framework_job::CancelToken::admit_root(ASSET_DECODE_TURN_GRANT)? else { return Ok(None) };
            owner.cancel = Some(cancel);
        }
        if owner.authority.is_none() {
            let Some((authority, _)) = semio_framework_job::JobPayloadAuthority::admit(owner.operation, owner.generation, ASSET_DECODE_TURN_GRANT)? else { return Ok(None) };
            owner.authority = Some(authority);
        }
        self.run_asset_decode_turn(run)
    }

    fn run_asset_decode_turn<R>(&mut self, run: impl FnOnce(&AppMailbox, &mut semio_framework_job::StepContext<'_>) -> Result<R, semio_framework_job::OriginalTurnFailure>) -> Result<Option<R>, semio_framework_job::OriginalTurnFailure> {
        let Some(owner) = self.asset_decode_owner.as_mut() else { return Ok(None) };
        let (Some(cancel), Some(authority)) = (owner.cancel.as_ref(), owner.authority.as_ref()) else { return Ok(None) };
        let Some(now) = semio_framework_job::default_now_us() else { return Ok(None) };
        let Some(budget) = semio_framework_job::StepBudget::from_duration(semio_framework_job::MAINTENANCE_LANE_FUEL, now, semio_framework_job::MAINTENANCE_LANE_WALL_US, ASSET_DECODE_TURN_GRANT) else { return Ok(None) };
        let mut progress = semio_framework_value::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::with_payload_authority(owner.operation, owner.generation, budget, cancel, semio_framework_job::default_now_us, &mut owner.sequence, &mut progress, authority)?;
        run(&self.runtime, &mut cx).map(Some)
    }

    /// ♻️ Closes the asset decode session under its own root, then retires that root in paid turns of the caller; true once nothing remains.
    pub fn close_asset_decode_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let Some(owner) = self.asset_decode_owner.as_mut() else { return Ok(true) };
        owner.closing = true;
        if owner.terminal_is_empty() {
            return Ok(true);
        }
        if owner.cancel.is_some() && owner.authority.is_some() && !matches!(self.run_asset_decode_turn(|runtime, asset| runtime.close_asset_decode_session_step(asset))?, Some(true)) {
            return Ok(false);
        }
        let grant = cx.retained_grant();
        let funded = !cx.retirement_should_yield() && grant.maximum_items != 0 && grant.maximum_depth != 0;
        let paid = semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let Some(owner) = self.asset_decode_owner.as_mut() else { return Ok(true) };
        if let Some(authority) = owner.authority.as_mut() {
            if !authority.terminal_is_empty() {
                let demand = authority.retirement_demands();
                let _ = semio_framework_job::close_original_retained_step(cx, demand, |step| authority.close_step(step))?;
                return Ok(false);
            }
            if !funded {
                return Ok(false);
            }
            cx.consume_retained(paid)?;
            cx.consume_fuel(1);
            owner.authority = None;
            return Ok(false);
        }
        if let Some(cancel) = owner.cancel.take() {
            if !funded {
                owner.cancel = Some(cancel);
                return Ok(false);
            }
            match semio_framework_value::retirement::controlled::ControlledRetirement::new(cancel) {
                Ok(close) => owner.cancel_close = Some(close),
                Err((error, cancel)) => {
                    owner.cancel = Some(cancel);
                    return Err(error.into());
                }
            }
            cx.consume_retained(paid)?;
            cx.consume_fuel(1);
            return Ok(false);
        }
        if let Some(close) = owner.cancel_close.as_mut() {
            if !close.terminal_is_empty() {
                let demand = close.selected_demand(grant.maximum_copy_bytes)?;
                let _ = semio_framework_job::close_original_retained_step(cx, demand, |step| close.step(step))?;
                return Ok(false);
            }
            if !funded {
                return Ok(false);
            }
            cx.consume_retained(paid)?;
            cx.consume_fuel(1);
            owner.cancel_close = None;
            return Ok(false);
        }
        Ok(owner.terminal_is_empty())
    }

    /// ⏱️ Convenience — every `winit_app.rs` callback needs "now" in this host's clock at least once.
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn now_seconds(&self) -> f64 {
        self.clock.now_seconds()
    }

    /// 🎒️ Transfers the original host only after its caller funds retirement storage.
    pub fn try_into_retirement(owner:&mut Option<Self>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<OsHostRetirement>,semio_framework_job::OriginalTurnFailure> {
        let Some(host)=owner.as_mut()else{return Ok(None)};
        if !host.close_asset_decode_step(cx)?{return Ok(None)}
        let Some(host)=owner.as_ref()else{return Ok(None)};
        let grant=cx.retained_grant();let capacity=std::mem::size_of::<OsHostRetirementState>();
        if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<capacity||host.component_surface_close.is_some()||component_surface_close_occupied(){
            return Ok(None);
        }
        let Some(abandonment) = reserve_os_host_retirement_abandonment() else {
            return Ok(None);
        };
        cx.consume_fuel(1);
        if let Err(error)=cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}){
            if !release_os_host_retirement_abandonment(abandonment){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original retirement storage admission lost its reservation").into())}
            return Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx));
        }
        let Self {
            retained,
            animation_clock: _,
            runtime,
            presenter,
            scheduler,
            clock,
            hot_swap,
            frame_generation: _,
            frame_ready: _,
            cursor_wake_requested,
            platform_fullscreen: _,
            present_fault: _,
            events,
            ui_token,
            snapshot_sink,
            original_snapshot,
            frame_build,
            surface_resize,
            component_surface_close: _,
            asset_decode_owner: _,
        } = owner.take().expect("original retirement host admitted");
        let state = OsHostRetirementState {
            retained,
            camera_progress: Default::default(),
            runtime: Some(runtime),
            presenter: Some(presenter),
            scheduler: Some(scheduler),
            clock: Some(clock),
            hot_swap: Some(hot_swap),
            events: Some(events),
            ui_token: Some(ui_token),
            snapshot_sink: Some(snapshot_sink),
            original_snapshot,
            original_snapshot_close:None,
            frame_build: Some(frame_build),
            surface_resize: Some(surface_resize),
            engine_surfaces: PairedEngineSurfaceClose::new(),
            raster_uploads: Some(semio_framework_os_renderer_wgpu_scene_state::begin_pending_raster_authority_close()),
            cursor_wake_requested,
            #[cfg(not(target_arch = "wasm32"))]
            kernel_progress_close: semio_framework_os_renderer_wgpu_kernel_runtime::KernelClient::get().begin_close_realm().ok(),
        };
        Ok(Some(OsHostRetirement { state: Some(Box::new(state)), abandonment: Some(abandonment) }))
    }

    pub fn retain_cursor_wake_directive(&mut self, token: infinite_world::world::WorldCursorWakeToken) {
        if self.cursor_wake_requested.as_ref().is_none_or(|pending| token.generation() > pending.generation()) {
            self.cursor_wake_requested = Some(token);
        }
    }

    pub fn take_cursor_wake_directive(&mut self) -> Option<infinite_world::world::WorldCursorWakeToken> {
        self.cursor_wake_requested.take()
    }

    pub fn advance_component_surface_close(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.component_surface_close.is_none() {
            if self.presenter.has_pending_presentation() {
                return Ok(false);
            }
            if component_surface_close_pending() && self.frame_build.has_live_session() {
                let _ = self.frame_build.retire_for_component_surface_close_step(cx)?;
                return Ok(true);
            }
            self.component_surface_close = take_component_surface_close_request().map(ComponentSurfaceCloseOwner::new);
        }
        let Some(owner) = self.component_surface_close.as_mut() else { return Ok(false) };
        if !owner.close_step(&self.runtime, &mut self.presenter, self.retained) {
            return Ok(true);
        }
        if !owner.terminal_is_empty() {
            return Ok(true);
        }
        let token = owner.request.token;
        if !publish_component_surface_close_terminal(token) {
            return Ok(true);
        }
        self.component_surface_close = None;
        Ok(false)
    }
}

impl OsHostRetirementState {
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure> {
        if let Some(owner)=self.original_snapshot_close.as_mut(){if !owner.terminal_is_empty(){let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;let _=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}self.original_snapshot_close=None;cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
        if self.original_snapshot.is_some(){let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}let source=self.original_snapshot.take().unwrap();match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>self.original_snapshot_close=Some(owner),Err((original,source))=>{self.original_snapshot=Some(source);return Err(original.into())}}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
        if let Some(presenter)=self.presenter.as_mut(){if !presenter.close_original_media_step(cx)?{return Ok(false)}}
        if !semio_framework_os_renderer_wgpu_host_plumbing::surface_lane::MountedSurfaceResizeLane::close_abandoned_step(cx)? {
            return Ok(false);
        }
        if let Some(surface_resize) = self.surface_resize.as_mut() {
            surface_resize.begin_close();
            if !surface_resize.close_step(cx)? || !surface_resize.terminal_is_empty() {
                return Ok(false);
            }
            self.surface_resize = None;
            return Ok(false);
        }
        if let Some(presenter) = self.presenter.as_mut() {
            if !presenter.close_surface_resize_step() {
                return Ok(false);
            }
        }
        if let Some(frame_build) = self.frame_build.as_mut() {
            if !frame_build.close_original_frame_step(cx)? {
                return Ok(false);
            }
            if !frame_build.terminal_is_empty() {
                return Ok(false);
            }
            self.frame_build = None;
            return Ok(false);
        }
        if !semio_framework_os_renderer_wgpu_scene_state::scene_camera_storage_is_empty() {
            let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
            let step = semio_framework_os_renderer_wgpu_scene_state::close_scene_camera_storage(grant).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
            let receipt = step.progress();
            assert!(receipt.fits(grant));cx.consume_fuel(receipt.copied_items as u64);cx.consume_retained(receipt)?;
            self.camera_progress.copied_items = self.camera_progress.copied_items.checked_add(receipt.copied_items).expect("host camera item receipt overflow");
            self.camera_progress.copied_bytes = self.camera_progress.copied_bytes.checked_add(receipt.copied_bytes).expect("host camera copy receipt overflow");
            self.camera_progress.retained_capacity_bytes = self.camera_progress.retained_capacity_bytes.checked_add(receipt.retained_capacity_bytes).expect("host camera birth receipt overflow");
            self.camera_progress.released_bytes = self.camera_progress.released_bytes.checked_add(receipt.released_bytes).expect("host camera release receipt overflow");
            return Ok(false);
        }
        if let Some(raster_uploads) = self.raster_uploads.as_mut() {
            if !raster_uploads.close_step() {
                return Ok(false);
            }
            if !raster_uploads.terminal_is_empty() {
                return Ok(false);
            }
            self.raster_uploads = None;
            return Ok(false);
        }
        if let Some(events) = self.events.as_mut() {
            if !events.close_step() {
                return Ok(false);
            }
            self.events = None;
            return Ok(false);
        }
        if self.cursor_wake_requested.take().is_some() {
            return Ok(false);
        }
        if let Some(presenter) = self.presenter.as_mut() {
            if !presenter.close_cursor_wake_step() {
                return Ok(false);
            }
            if !matches!(presenter.close_frame_owners_step(), Ok(true)) {
                return Ok(false);
            }
        }
        if !self.engine_surfaces.terminal_is_empty() {
            let (Some(runtime), Some(presenter)) = (self.runtime.as_ref(), self.presenter.as_mut()) else {
                return Ok(false);
            };
            if !self.engine_surfaces.close_step(runtime, presenter, cx.retained_grant()) {
                return Ok(false);
            }
            if !self.engine_surfaces.terminal_is_empty() {
                return Ok(false);
            }
        }
        if let Some(runtime) = self.runtime.as_ref() {
            if !runtime.close_input_step(cx)? {
                return Ok(false);
            }
        }
        if let Some(presenter) = self.presenter.as_mut() {
            if !matches!(presenter.close_world_owners_step(), Ok(true)) {
                return Ok(false);
            }
            if !presenter.world_owners_terminal_is_empty() || !presenter.engine_surfaces_terminal_is_empty() {
                return Ok(false);
            }
        }
        if let Some(runtime) = self.runtime.as_ref() {
            let Some(presenter)=self.presenter.as_mut()else{return Ok(false)};
            if !presenter.close_world3d_dynamic_step(runtime,cx)? {
                return Ok(false);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(close) = self.kernel_progress_close.as_ref() {
            match close.poll() {
                semio_framework_os_renderer_wgpu_kernel_runtime::KernelCloseStatus::Complete => {
                    self.kernel_progress_close = None;
                    return Ok(false);
                }
                semio_framework_os_renderer_wgpu_kernel_runtime::KernelCloseStatus::Pending | semio_framework_os_renderer_wgpu_kernel_runtime::KernelCloseStatus::AdmissionBlocked => return Ok(false),
                semio_framework_os_renderer_wgpu_kernel_runtime::KernelCloseStatus::Fault => return Ok(false),
            }
        }
        for owner in [&mut self.snapshot_sink as &mut dyn RetirementOwner, &mut self.ui_token, &mut self.hot_swap, &mut self.clock, &mut self.scheduler, &mut self.runtime, &mut self.presenter] {
            if owner.retire() {
                return Ok(false);
            }
        }
        Ok(self.terminal_is_empty())
    }

    fn terminal_is_empty(&self) -> bool {
        self.runtime.is_none()
            && self.presenter.is_none()
            && self.scheduler.is_none()
            && self.clock.is_none()
            && self.hot_swap.is_none()
            && self.events.is_none()
            && self.ui_token.is_none()
            && self.original_snapshot.is_none()
            && self.original_snapshot_close.is_none()
            && self.snapshot_sink.is_none()
            && self.frame_build.is_none()
            && self.surface_resize.is_none()
            && self.raster_uploads.is_none()
            && semio_framework_os_renderer_wgpu_scene_state::scene_camera_storage_is_empty()
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
        debug_assert!(self.terminal_is_empty() || std::thread::panicking(), "OsHostRetirementState must reach terminal-empty before release");
    }
}

impl OsHostRetirement {
    pub fn close_abandoned_step(_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.load(Ordering::Acquire)==0{return Ok(true)}
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"abandoned host requires its original supplied lifecycle context").into())
    }

    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        if let Some(state)=self.state.as_mut(){
            if !state.close_step(cx)?||!state.terminal_is_empty(){return Ok(false)}
            let grant=cx.retained_grant();let release=std::mem::size_of::<OsHostRetirementState>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<release{return Ok(false)}
            drop(self.state.take());cx.consume_fuel(1);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()})?;return Ok(false)
        }
        let Some(token)=self.abandonment else{return Ok(true)};
        if !release_os_host_retirement_abandonment(token){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original host retirement abandonment token changed before release").into())}
        self.abandonment=None;cx.consume_fuel(1);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.is_none() && self.abandonment.is_none()
    }
}

impl Drop for OsHostRetirement {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else {
            if let Some(token) = self.abandonment.take() {
                if !release_os_host_retirement_abandonment(token) && !std::thread::panicking() {
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


//#endregion 🏠️OsHost

//#endregion 🔖️OsHost

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️fixtures/🦀️.rs"]
mod fixtures;
#[cfg(any(test, feature = "testkit"))]
pub use fixtures::{finish_cpu_only_fixture_component_close, finish_retired_cpu_only_map_fixture_close, finish_world_fixture_component_close, runtime_with_shell};

#[cfg(test)]
#[path = "../../../🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs"]
mod standalone_tests;
