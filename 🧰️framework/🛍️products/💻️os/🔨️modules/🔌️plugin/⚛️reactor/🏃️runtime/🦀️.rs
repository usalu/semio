//! 📤️ WASM component export glue for plugin bundles.
#[cfg(test)] use semio_framework_plugin_ownership::bounded_ownership::runtime_lifecycle_step;
use crate::app_emission::ActionMeta;
use crate::plugin_program::{AppInstance, Plugin, PluginProgram};
use crate::media_contract::ArtifactMediaExportHandle;
use crate::media_contract::ArtifactMediaExportPoll;
use crate::media_contract::ArtifactMediaExportResult;
use crate::tool_jobs::{EmitWire, TransactionProposalDraft, TypedOperationLeftover, TypedOperationResultPage, TypedOperationResultToken};
use crate::app_contract::{EphemeralSnapshot, MediaArtifact, MediaArtifactDescriptor, MediaArtifactError, MediaConsumption};
use crate::plugin_app::PluginApp;
use semio_framework::MediaError;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use crate::peer_roster::PresenceRosterAdmission;
use crate::{ArtifactApp, WindowConfigPack, MountedOwnerPolicyV1, MountedOwnerTurnV1, MountedOwnerIdentityV1, MountedOwnerPhaseV1};
use semio_framework_value::{DslValue,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework::manifest::{ActionInvocation as ManifestActionInvocation, CommandAddress as ManifestCommandAddress, CommandInvocation as ManifestCommandInvocation, CommandOwnerAddress as ManifestCommandOwnerAddress};
use semio_framework::{
    kernel::{ActivationEvent, CapabilityRequest, CapabilityRequirement, Effect, InvocationResult, MessageEndpoint, QuotaSchema},
    AssetDeclaration, ExecutionMode, ExtensionPointDeclaration, Fault, FaultCode, FaultFrom, FaultOrigin, PluginManifest, TopicContribution, ViewModel,
};
/// 🎯️ M1/M2 (ticket 26/08/17 `design-unified.md`): `UiIntent`/`PresenceUpdate` for
/// `plugin_dispatch_intents`/`plugin_render_surface` — this module (a sibling of `pub mod app`, not
/// nested inside it) has no glob import of the contract crate the way `mod app` does.
use semio_framework_ui_contract as ui_contract;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-helpers`): `sdk-flip` flipped `plugin_render`/
/// `plugin_render_with_document` below to return `ComponentTree` but this module (a sibling of
/// `pub mod app`, not nested inside it) never gained an import for the name — added here.
use semio_framework_ui_runtime::ComponentTree;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::{Cell, RefCell};
use std::mem::{size_of_val};
#[cfg(test)] use std::mem::{size_of};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};
use ui_wgpu::wgpu::{ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextMenuSurfaceTarget, UiMenuRef};
/// 🛎️ Why a lane did not get [`RuntimeAppCell::instance`]. `Contended` is a hold that releases
/// on its own — another lane is mid-step; `Poisoned` is a hold that never will.
#[derive(Debug)]
enum RuntimeInstanceRefusal {
    Contended,
    Poisoned,
}
/// 🛎️ The liveness backstop on a host crossing's wait for a background lane, counted in
/// scheduler yields. One background turn is priced at [`RUNTIME_CLOSE_INNER_GRANT_US`] and
/// every yield hands the core to whoever holds the app, so a well-behaved lane releases within
/// a handful of them; this ceiling exists only so a lane that breaks its own step budget turns
/// into the refusal the host already knows how to report rather than an unbounded host stall.
const RUNTIME_HOST_CONTENTION_YIELDS: u32 = 4_096;
/// 🛎️ A host crossing's exclusive hold on one app, which keeps every background lane off the
/// app for as long as it lives. It is deliberately held across `.await`: a host crossing's
/// local future retains exclusive app ownership while suspended (a submitted media export
/// spans several awaits), which is what makes the crossing atomic against the maintenance and
/// close lanes; those lanes never wait on it — they read [`RuntimeAppCell::host_crossings`] and
/// yield their whole turn.
struct HostInstanceGuard<'a, PA: PluginApp> {
    cell: &'a RuntimeAppCell<PA>,
    instance: Option<std::sync::MutexGuard<'a, AppInstance<PA>>>,
}
impl<PA: PluginApp> std::ops::Deref for HostInstanceGuard<'_, PA> {
    type Target = AppInstance<PA>;
    fn deref(&self) -> &AppInstance<PA> {
        self.instance.as_ref().expect("a live host crossing holds its app")
    }
}
impl<PA: PluginApp> std::ops::DerefMut for HostInstanceGuard<'_, PA> {
    fn deref_mut(&mut self) -> &mut AppInstance<PA> {
        self.instance.as_mut().expect("a live host crossing holds its app")
    }
}
/// 🛎️ The app is released BEFORE the crossing is unregistered, so no lane can ever read a free
/// counter while the mutex is still held — the reading a waiting lane makes its decision on is
/// never stale in the direction that would refuse a crossing.
impl<PA: PluginApp> Drop for HostInstanceGuard<'_, PA> {
    fn drop(&mut self) {
        drop(self.instance.take());
        self.cell.host_crossings.fetch_sub(1, Ordering::SeqCst);
    }
}
/// 🔧️ A background maintenance or close turn's hold on one app, counted while it lives so a
/// host crossing can tell a transient hold from one it must refuse.
struct BackgroundInstanceGuard<'a, PA: PluginApp> {
    cell: &'a RuntimeAppCell<PA>,
    instance: Option<std::sync::MutexGuard<'a, AppInstance<PA>>>,
}
impl<PA: PluginApp> std::ops::Deref for BackgroundInstanceGuard<'_, PA> {
    type Target = AppInstance<PA>;
    fn deref(&self) -> &AppInstance<PA> {
        self.instance.as_ref().expect("a live background turn holds its app")
    }
}
impl<PA: PluginApp> std::ops::DerefMut for BackgroundInstanceGuard<'_, PA> {
    fn deref_mut(&mut self) -> &mut AppInstance<PA> {
        self.instance.as_mut().expect("a live background turn holds its app")
    }
}
/// 🔧️ The app is released BEFORE the hold is unregistered. The other order has a window in
/// which a waiting host crossing reads zero background holds while the mutex is still taken,
/// and refuses the very turn it was waiting out.
impl<PA: PluginApp> Drop for BackgroundInstanceGuard<'_, PA> {
    fn drop(&mut self) {
        drop(self.instance.take());
        self.cell.background_holds.fetch_sub(1, Ordering::SeqCst);
    }
}
struct RuntimeAppCell<PA: PluginApp> {
    id: u32,
    mounted_policy: MountedOwnerPolicyV1,
    instance: std::sync::Mutex<AppInstance<PA>>,
    maintenance_pump: std::sync::Mutex<RuntimeLiveCleanupPump<PA>>,
    maintenance_status: AtomicU8,
    maintenance_fault_us: AtomicU64,
    maintenance_generation: AtomicU64,
    maintenance_stalled_steps: AtomicU32,
    /// 🛎️ How many HOST crossings currently want or hold [`Self::instance`]. A background lane
    /// reads it before it touches the mutex and yields its whole turn while it is non-zero, so
    /// the host lane — which has nothing to yield to — is never made to wait on maintenance it
    /// did not ask for.
    host_crossings: AtomicU32,
    /// 🔧️ How many BACKGROUND lane turns currently want or hold [`Self::instance`]. It is
    /// raised before the try-lock and lowered on release, so a non-zero reading is the host's
    /// proof that a refused try-lock is a transient maintenance or close step (which is priced
    /// at [`RUNTIME_CLOSE_INNER_GRANT_US`] and will release) rather than an app the host lane
    /// itself already owns.
    background_holds: AtomicU32,
    /// 🌡️ [`PluginApp::maintenance_under_pressure`] as of the last maintenance step — read by the
    /// reactor turn to decide whether this instance gets a burst of steps instead of one.
    maintenance_pressure: AtomicBool,
    #[cfg(target_arch = "wasm32")]
    maintenance_probe_turns: AtomicU64,
    #[cfg(any(test, target_arch = "wasm32"))]
    maintenance_probe_entries: AtomicU64,
    #[cfg(test)]
    maintenance_probe_input_waits: AtomicU64,
}
impl<PA: PluginApp> RuntimeAppCell<PA> {
    fn new(instance: AppInstance<PA>, mounted_policy: MountedOwnerPolicyV1) -> Self {
        Self {
            id: instance.id,
            mounted_policy,
            instance: std::sync::Mutex::new(instance),
            maintenance_pump: std::sync::Mutex::new(RuntimeLiveCleanupPump::new()),
            maintenance_status: AtomicU8::new(RuntimeMaintenanceStatus::Ready.repr()),
            maintenance_fault_us: AtomicU64::new(0),
            maintenance_generation: AtomicU64::new(0),
            maintenance_stalled_steps: AtomicU32::new(0),
            host_crossings: AtomicU32::new(0),
            background_holds: AtomicU32::new(0),
            maintenance_pressure: AtomicBool::new(false),
            #[cfg(target_arch = "wasm32")]
            maintenance_probe_turns: AtomicU64::new(0),
            #[cfg(any(test, target_arch = "wasm32"))]
            maintenance_probe_entries: AtomicU64::new(0),
            #[cfg(test)]
            maintenance_probe_input_waits: AtomicU64::new(0),
        }
    }
    /// 🛎️ Takes the app for a HOST crossing, which outranks every background lane. Registering
    /// the crossing BEFORE the try-lock stops a background lane from starting a new turn on
    /// this app at all; a turn that was already running is waited out, because it is priced at
    /// [`RUNTIME_CLOSE_INNER_GRANT_US`] and releases on its own, and refusing the host over it
    /// would turn a scheduling accident into a `plugin.internal` fault the caller has no way to
    /// retry. `None` keeps the one meaning the bare try-lock always had at a host site: the app
    /// is owned by something that will NOT release for this crossing — the host lane itself (a
    /// reentrant crossing, or a submitted media export holding the app across its await) or a
    /// poisoned mutex.
    fn host_instance(&self) -> Option<HostInstanceGuard<'_, PA>> {
        self.host_crossings.fetch_add(1, Ordering::SeqCst);
        for _ in 0..RUNTIME_HOST_CONTENTION_YIELDS {
            match self.instance.try_lock() {
                Ok(instance) => return Some(HostInstanceGuard { cell: self, instance: Some(instance) }),
                Err(std::sync::TryLockError::Poisoned(_)) => break,
                Err(std::sync::TryLockError::WouldBlock) => {
                    if self.background_holds.load(Ordering::SeqCst) == 0 {
                        break;
                    }
                    std::thread::yield_now();
                }
            }
        }
        self.host_crossings.fetch_sub(1, Ordering::SeqCst);
        None
    }
    /// 🔧️ Takes the app for a BACKGROUND maintenance or close turn. It never waits: a host
    /// crossing in flight, or a lock already taken, is a contended turn, and the lane's own
    /// `Yield` is exactly what a contended turn costs.
    fn background_instance(&self) -> Result<BackgroundInstanceGuard<'_, PA>, RuntimeInstanceRefusal> {
        if self.host_crossings.load(Ordering::SeqCst) > 0 {
            return Err(RuntimeInstanceRefusal::Contended);
        }
        self.background_holds.fetch_add(1, Ordering::SeqCst);
        match self.instance.try_lock() {
            Ok(instance) => Ok(BackgroundInstanceGuard { cell: self, instance: Some(instance) }),
            Err(refusal) => {
                self.background_holds.fetch_sub(1, Ordering::SeqCst);
                Err(match refusal {
                    std::sync::TryLockError::Poisoned(_) => RuntimeInstanceRefusal::Poisoned,
                    std::sync::TryLockError::WouldBlock => RuntimeInstanceRefusal::Contended,
                })
            }
        }
    }
}
/// 🩺️ The single cause taxonomy shared by both supervisory cleanup loops (live maintenance and
/// close), replacing the bare magic `u8` fault constants: every store site names its own cause,
/// so the one decode site per loop can emit a distinct `plugin.internal.*` wire fault code
/// instead of collapsing thirteen unrelated conditions into one opaque string. The language
/// neutral examples are `🩺️runtime-fault-vectors.json`; native tests and the React window-fault
/// classifier compare their observed outcomes against those examples.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum RuntimeCleanupFault {
    Clock = 0,
    ClockRegression = 1,
    InteractiveCeiling = 2,
    ZeroProgress = 3,
    AlreadyClosing = 4,
    AdmissionRejected = 5,
    Resume = 6,
    PriorOutcome = 7,
    MissingSession = 8,
    AbiMismatch = 9,
    Checkout = 10,
    CheckedOutJob = 11,
    TakeOutcome = 12,
    CooperativeClock = 13,
    OwnerPoisoned = 14,
    InstancePoisoned = 15,
    InstanceNotDrained = 16,
    MaintenancePoisoned = 17,
    MaintenanceNotDrained = 18,
}
/// 🩺️ Wire projection of one {@link RuntimeCleanupFault}: the fixture key, the emitted
/// `Fault::code`, and the human sentence spliced into `Fault::message`.
struct RuntimeCleanupFaultVector {
    slug: &'static str,
    code: &'static str,
    detail: &'static str,
}
const RUNTIME_CLEANUP_FAULTS: [RuntimeCleanupFault; 19] = [
    RuntimeCleanupFault::Clock,
    RuntimeCleanupFault::ClockRegression,
    RuntimeCleanupFault::InteractiveCeiling,
    RuntimeCleanupFault::ZeroProgress,
    RuntimeCleanupFault::AlreadyClosing,
    RuntimeCleanupFault::AdmissionRejected,
    RuntimeCleanupFault::Resume,
    RuntimeCleanupFault::PriorOutcome,
    RuntimeCleanupFault::MissingSession,
    RuntimeCleanupFault::AbiMismatch,
    RuntimeCleanupFault::Checkout,
    RuntimeCleanupFault::CheckedOutJob,
    RuntimeCleanupFault::TakeOutcome,
    RuntimeCleanupFault::CooperativeClock,
    RuntimeCleanupFault::OwnerPoisoned,
    RuntimeCleanupFault::InstancePoisoned,
    RuntimeCleanupFault::InstanceNotDrained,
    RuntimeCleanupFault::MaintenancePoisoned,
    RuntimeCleanupFault::MaintenanceNotDrained,
];
const RUNTIME_CLEANUP_FAULT_VECTORS: [RuntimeCleanupFaultVector; 19] = [
    RuntimeCleanupFaultVector { slug: "clock", code: "plugin.internal.clock", detail: "no monotonic clock reading at turn start" },
    RuntimeCleanupFaultVector { slug: "clock-regression", code: "plugin.internal.clock-regression", detail: "the monotonic clock ran backwards across the turn" },
    RuntimeCleanupFaultVector { slug: "interactive-ceiling", code: "plugin.internal.interactive-ceiling", detail: "the turn overran the interactive step ceiling" },
    RuntimeCleanupFaultVector { slug: "zero-progress", code: "plugin.internal.zero-progress", detail: "the cleanup pump made no progress for its whole stall credit" },
    RuntimeCleanupFaultVector { slug: "already-closing", code: "plugin.internal.already-closing", detail: "a close is already in flight for this instance" },
    RuntimeCleanupFaultVector { slug: "admission-rejected", code: "plugin.internal.admission-rejected", detail: "the cleanup job session was refused admission and has now drained" },
    RuntimeCleanupFaultVector { slug: "resume", code: "plugin.internal.resume", detail: "the cleanup job session refused to resume after its outcome drained" },
    RuntimeCleanupFaultVector { slug: "prior-outcome", code: "plugin.internal.prior-outcome", detail: "a prior step outcome faulted or was cancelled" },
    RuntimeCleanupFaultVector { slug: "missing-session", code: "plugin.internal.missing-session", detail: "the cleanup pump lost the job session it was driving" },
    RuntimeCleanupFaultVector { slug: "abi-mismatch", code: "plugin.internal.abi-mismatch", detail: "the component step trapped, so its exports no longer match the host abi" },
    RuntimeCleanupFaultVector { slug: "checkout", code: "plugin.internal.checkout", detail: "the cleanup job session produced no checkout after a step" },
    RuntimeCleanupFaultVector { slug: "checked-out-job", code: "plugin.internal.checked-out-job", detail: "the checked out cleanup job body was unreachable" },
    RuntimeCleanupFaultVector { slug: "take-outcome", code: "plugin.internal.take-outcome", detail: "the cleanup job session produced no step outcome" },
    RuntimeCleanupFaultVector { slug: "clock-cooperative", code: "plugin.internal.clock-cooperative", detail: "cooperative maintenance requires an installed monotonic clock" },
    RuntimeCleanupFaultVector { slug: "owner-poisoned", code: "plugin.internal.owner-poisoned", detail: "the instance owner lock was poisoned by a panicking turn" },
    RuntimeCleanupFaultVector { slug: "instance-poisoned", code: "plugin.internal.instance-poisoned", detail: "the app instance lock was poisoned by a panicking turn" },
    RuntimeCleanupFaultVector { slug: "instance-not-drained", code: "plugin.internal.instance-not-drained", detail: "the app instance still held terminal state at retirement" },
    RuntimeCleanupFaultVector { slug: "maintenance-poisoned", code: "plugin.internal.maintenance-poisoned", detail: "the maintenance pump lock was poisoned by a panicking turn" },
    RuntimeCleanupFaultVector { slug: "maintenance-not-drained", code: "plugin.internal.maintenance-not-drained", detail: "the maintenance pump still held terminal state at retirement" },
];
impl RuntimeCleanupFault {
    const fn vector(self) -> &'static RuntimeCleanupFaultVector {
        &RUNTIME_CLEANUP_FAULT_VECTORS[self as usize]
    }
    fn from_index(index: u8) -> Option<Self> {
        RUNTIME_CLEANUP_FAULTS.get(index as usize).copied()
    }
}
/// 🩺️ Live maintenance-loop status stored in `RuntimeAppCell::maintenance_status`. The three
/// non-fault states keep their historic `0..2` encoding; every fault is `3 + cause`, so the
/// atomic carries the discriminator the decode site needs without a second word.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RuntimeMaintenanceStatus {
    Ready,
    Queued,
    Running,
    Fault(RuntimeCleanupFault),
}
const RUNTIME_MAINTENANCE_FAULT_BASE: u8 = 3;
impl RuntimeMaintenanceStatus {
    const fn repr(self) -> u8 {
        match self {
            Self::Ready => 0,
            Self::Queued => 1,
            Self::Running => 2,
            Self::Fault(cause) => RUNTIME_MAINTENANCE_FAULT_BASE + cause as u8,
        }
    }
    fn from_repr(repr: u8) -> Self {
        match repr {
            0 => Self::Ready,
            1 => Self::Queued,
            2 => Self::Running,
            other => Self::Fault(RuntimeCleanupFault::from_index(other - RUNTIME_MAINTENANCE_FAULT_BASE).expect("maintenance status repr is written only by this module")),
        }
    }
    #[cfg(test)]
    const fn cause(self) -> Option<RuntimeCleanupFault> {
        match self {
            Self::Fault(cause) => Some(cause),
            _ => None,
        }
    }
}
/// 🩺️ Close-worker status stored in `RuntimeCloseWorkerState::status`, laid out like
/// {@link RuntimeMaintenanceStatus} with two extra terminal states before the fault base.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RuntimeCloseStatus {
    Ready,
    Queued,
    Running,
    Complete,
    ExternalWait,
    DeadlineYield,
    Fault(RuntimeCleanupFault),
}
const RUNTIME_CLOSE_FAULT_BASE: u8 = 6;
impl RuntimeCloseStatus {
    const fn repr(self) -> u8 {
        match self {
            Self::Ready => 0,
            Self::Queued => 1,
            Self::Running => 2,
            Self::Complete => 3,
            Self::ExternalWait => 4,
            Self::DeadlineYield => 5,
            Self::Fault(cause) => RUNTIME_CLOSE_FAULT_BASE + cause as u8,
        }
    }
    fn from_repr(repr: u8) -> Self {
        match repr {
            0 => Self::Ready,
            1 => Self::Queued,
            2 => Self::Running,
            3 => Self::Complete,
            4 => Self::ExternalWait,
            5 => Self::DeadlineYield,
            other => Self::Fault(RuntimeCleanupFault::from_index(other - RUNTIME_CLOSE_FAULT_BASE).expect("close status repr is written only by this module")),
        }
    }
    const fn cause(self) -> Option<RuntimeCleanupFault> {
        match self {
            Self::Fault(cause) => Some(cause),
            _ => None,
        }
    }
}
/// 🩺️ The one decoded wire fault both cleanup loops emit — `origin`/`code`/`severity`/`message`
/// keep their shape, the discrimination lives entirely in `code`, and the message always names
/// the instance, the variant slug, and the measured turn cost against the interactive ceiling.
fn runtime_cleanup_fault(scope: &str, cause: RuntimeCleanupFault, instance_id: u32, elapsed_us: u64) -> Fault {
    let vector = cause.vector();
    let elapsed = if elapsed_us == RUNTIME_CLEANUP_UNMEASURED_US { "unmeasured".to_string() } else { format!("{elapsed_us}us") };
    Fault::new(
        FaultOrigin::Plugin,
        FaultCode::new(vector.code),
        format!("runtime {scope} cleanup faulted for instance {instance_id}: {} [{}] (elapsed {elapsed}, ceiling {}us)", vector.detail, vector.slug, semio_framework_trace::INTERACTIVE_STEP_CEILING_US),
    )
}
/// 🚪️ Whether a wire fault code is one of the cleanup taxonomy's own verdicts — i.e. whether it
/// names ONE instance's cleanup ladder rather than the reactor turn that drives every instance of
/// this actor. The reactor turn asks before deciding whether a close-cleanup failure may abort it:
/// an instance-scoped verdict is contained (its lifetime is already retired and already out of
/// quarantine), anything else — a busy runtime authority, a poisoned borrow — still ends the turn.
pub fn runtime_cleanup_fault_is_instance_scoped(code: &str) -> bool {
    RUNTIME_CLEANUP_FAULT_VECTORS.iter().any(|vector| vector.code == code)
}
/// ⏱️ Sentinel the two cleanup loops already store when the clock itself failed, so the decoded
/// message says `unmeasured` instead of printing `u64::MAX` microseconds as if it were a reading.
const RUNTIME_CLEANUP_UNMEASURED_US: u64 = u64::MAX;
const RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT: u32 = 256;
struct RuntimeCloseWorkerState<PA: PluginApp> {
    instance_id: u32,
    pub(crate) generation: semio_framework_job::Generation,
    mounted_policy: MountedOwnerPolicyV1,
    cell: std::sync::Mutex<std::mem::ManuallyDrop<Option<std::sync::Arc<RuntimeAppCell<PA>>>>>,
    pump: std::sync::Mutex<RuntimeCloseCleanupPump<PA>>,
    status: AtomicU8,
    deadline_resume: AtomicU8,
    deadline_elapsed_us: AtomicU64,
    stalled_steps: AtomicU8,
    /// ⏱️ Monotonic instant the current zero-progress run began (`0` = the ladder is moving).
    stall_since_us: AtomicU64,
    /// ⏱️ Stall credit actually spent when a `ZeroProgress` verdict was stored, so the emitted
    /// fault quotes the measured stall instead of the last turn's unrelated cost.
    stall_credit_spent_us: AtomicU64,
    last_callback_elapsed_us: AtomicU64,
    #[cfg(test)]
    last_fault: std::sync::Mutex<[u8; 256]>,
    #[cfg(test)]
    last_fault_origin: AtomicU8,
    #[cfg(test)]
    physical_close_calls: AtomicU64,
    #[cfg(test)]
    callback_phase_started_us: AtomicU64,
    #[cfg(test)]
    callback_phase_us: [AtomicU64; 24],
}
struct RuntimeCloseEntry<PA: PluginApp> {
    state: std::sync::Arc<RuntimeCloseWorkerState<PA>>,
}
mod instance_lifetime {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🚪️lifetime/🦀️.rs"));
}
pub use instance_lifetime::{plugin_capture_instance_close, PluginInstanceCloseLease};
const PLUGIN_RUNTIME_INSTANCE_SLOTS: usize = 1_024;
const PLUGIN_RUNTIME_INSTANCE_WORDS: usize = PLUGIN_RUNTIME_INSTANCE_SLOTS / u64::BITS as usize;
const PLUGIN_RUNTIME_ACTOR_BYTES: usize = 4_096;
struct RuntimeActorAuthority {
    len: u16,
    bytes: [u8; PLUGIN_RUNTIME_ACTOR_BYTES],
}
impl RuntimeActorAuthority {
    fn new(actor: &str) -> Result<Self, Fault> {
        if actor.is_empty() || actor.len() > PLUGIN_RUNTIME_ACTOR_BYTES {
            return Err(plugin_internal_fault("runtime actor identity is empty or exceeds its fixed byte cap"));
        }
        let mut bytes = [0; PLUGIN_RUNTIME_ACTOR_BYTES];
        bytes[..actor.len()].copy_from_slice(actor.as_bytes());
        Ok(Self { len: actor.len() as u16, bytes })
    }
}
impl std::fmt::Display for RuntimeActorAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("admitted actor authority remains utf-8"))
    }
}
struct RuntimeInstanceRegistry<T> {
    slots: Box<[std::mem::MaybeUninit<(u32, T)>]>,
    occupied: [u64; PLUGIN_RUNTIME_INSTANCE_WORDS],
    pub(crate) allocation_admitted: bool,
}
impl<T> RuntimeInstanceRegistry<T> {
    /// 🈳️ Constructs the registry EMPTY — no slot backing at all. Allocator success is
    /// not an admission: a runtime carries four of these, and the actor one alone is
    /// `PLUGIN_RUNTIME_INSTANCE_SLOTS` × `RuntimeActorAuthority` ≈ 4 MiB, which an actor that
    /// never opens an instance would otherwise reserve contiguously up front — the exact shape
    /// a guest's `dlmalloc` never gives back. Backing is taken by {@link admit_backing}, on the
    /// first admission that actually needs a slot.
    pub(crate) fn new() -> Self {
        Self { slots: Vec::new().into_boxed_slice(), occupied: [0; PLUGIN_RUNTIME_INSTANCE_WORDS], allocation_admitted: false }
    }
    /// 🛂️ Takes the one fixed slot allocation, once, and reports whether this registry
    /// now has backing. Idempotent; a refused reservation leaves the registry exactly as empty
    /// as it was, so every admission gate answers "cannot insert" instead of panicking.
    fn admit_backing(&mut self) -> bool {
        if self.allocation_admitted {
            return true;
        }
        let mut slots = Vec::new();
        if slots.try_reserve_exact(PLUGIN_RUNTIME_INSTANCE_SLOTS).is_err() {
            return false;
        }
        slots.resize_with(PLUGIN_RUNTIME_INSTANCE_SLOTS, std::mem::MaybeUninit::uninit);
        self.slots = slots.into_boxed_slice();
        self.allocation_admitted = true;
        true
    }
    fn index(instance_id: u32) -> usize {
        instance_id as usize % PLUGIN_RUNTIME_INSTANCE_SLOTS
    }
    fn occupied(&self, index: usize) -> bool {
        self.occupied[index / u64::BITS as usize] & (1u64 << (index % u64::BITS as usize)) != 0
    }
    fn set_occupied(&mut self, index: usize, occupied: bool) {
        let word = &mut self.occupied[index / u64::BITS as usize];
        let mask = 1u64 << (index % u64::BITS as usize);
        if occupied {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }
    /// 📎️ SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
    fn entry(&self, index: usize) -> Option<&(u32, T)> {
        if !self.occupied(index) {
            return None;
        }
        self.slots.get(index).map(|slot| unsafe { slot.assume_init_ref() })
    }
    #[cfg(test)]
    fn insert(&mut self, instance_id: u32, value: T) -> Result<(), T> {
        if !self.admit_backing() {
            return Err(value);
        }
        let index = Self::index(instance_id);
        if self.occupied(index) {
            return Err(value);
        }
        self.slots[index].write((instance_id, value));
        self.set_occupied(index, true);
        Ok(())
    }
    fn insert_admitted(&mut self, instance_id: u32, value: T) {
        let index = Self::index(instance_id);
        let admitted = self.admit_backing();
        debug_assert!(admitted && !self.occupied(index));
        self.slots[index].write((instance_id, value));
        self.set_occupied(index, true);
    }
    fn get(&self, instance_id: u32) -> Option<&T> {
        self.entry(Self::index(instance_id)).and_then(|(candidate, value)| (*candidate == instance_id).then_some(value))
    }
    fn get_mut(&mut self, instance_id: u32) -> Option<&mut T> {
        let index = Self::index(instance_id);
        if self.entry(index).is_none_or(|(candidate, _)| *candidate != instance_id) {
            return None;
        }
        Some(unsafe { &mut self.slots[index].assume_init_mut().1 })
    }
    /// 🚦️ The admission gate. It takes the backing itself (see {@link admit_backing}) so
    /// a preflight that answers `true` is a promise the following {@link insert_admitted} can
    /// keep, and a refused reservation is an ordinary "cannot insert" instead of a panic.
    fn can_insert(&mut self, instance_id: u32) -> bool {
        self.admit_backing() && !self.occupied(Self::index(instance_id))
    }
    /// 🫴️ SAFETY: exact identity and occupancy were checked, and occupancy is now cleared.
    fn take(&mut self, instance_id: u32) -> Option<T> {
        let index = Self::index(instance_id);
        if self.entry(index).is_none_or(|(candidate, _)| *candidate != instance_id) {
            return None;
        }
        self.set_occupied(index, false);
        Some(unsafe { self.slots[index].assume_init_read().1 })
    }
    fn entry_at(&self, index: usize) -> Option<(u32, &T)> {
        self.entry(index).map(|(instance_id, value)| (*instance_id, value))
    }
    fn next_entry_from(&self, start: usize) -> Option<(usize, u32, &T)> {
        let start = start % PLUGIN_RUNTIME_INSTANCE_SLOTS;
        let word = start / u64::BITS as usize;
        let bit = start % u64::BITS as usize;
        for offset in 0..=PLUGIN_RUNTIME_INSTANCE_WORDS {
            let index = (word + offset) % PLUGIN_RUNTIME_INSTANCE_WORDS;
            let mask = if offset == 0 {
                u64::MAX << bit
            } else if offset == PLUGIN_RUNTIME_INSTANCE_WORDS {
                (1u64 << bit) - 1
            } else {
                u64::MAX
            };
            let occupied = self.occupied[index] & mask;
            if occupied == 0 {
                continue;
            }
            let slot = index * u64::BITS as usize + occupied.trailing_zeros() as usize;
            return self.entry_at(slot).map(|(id, value)| (slot, id, value));
        }
        None
    }
    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.occupied.iter().all(|word| *word == 0)
    }
}
impl<T> Drop for RuntimeInstanceRegistry<T> {
    fn drop(&mut self) {}
}
struct RuntimeDocumentBackboneBindingV1 {
    pub(crate) generation: u64,
    uri: Option<String>,
    channel: Option<store::os_store::backbone::ActorBackboneChannelOwner>,
}
impl RuntimeDocumentBackboneBindingV1 {
    fn state(&self) -> crate::document_backbone_binding::DocumentBackboneBindingStateV1 {
        crate::document_backbone_binding::DocumentBackboneBindingStateV1 { generation: self.generation, uri: self.uri.clone() }
    }
}
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-native-aggregate-backing/🦀️.rs");
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-runtime-instance-registry/🦀️.rs");
/// 🧩️ Typed state owned by each embedding plugin's `plugin_exports!` expansion.
pub struct PluginRuntime<PA: PluginApp> {
    mounted_policy: MountedOwnerPolicyV1,
    pub(crate) guest_lifetimes: RefCell<crate::reactor::instance_lifetime::NativeLifecycleRegistry<PA>>,
    plugin: RefCell<Option<Plugin<PA>>>,
    plugin_assembly_error: RefCell<Option<Fault>>,
    instances: RefCell<RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>>,
    close_quarantine: RefCell<RuntimeInstanceRegistry<RuntimeCloseEntry<PA>>>,
    close_cleanup_cursor: Cell<usize>,
    live_cleanup_cursor: Cell<usize>,
    /// 🌡️ Whether the instance the last [`plugin_step_live_cleanup`] stepped reported maintenance pressure.
    live_cleanup_pressure: Cell<bool>,
    typed_continuation_cursor: Cell<usize>,
    close_generation: Cell<u64>,
    local_interaction_query_generation: crate::local_interaction::live::LocalInteractionQueryGeneration,
    instance_actors: RefCell<RuntimeInstanceRegistry<RuntimeActorAuthority>>,
    document_backbones: RefCell<RuntimeInstanceRegistry<RuntimeDocumentBackboneBindingV1>>,
}
impl<PA: PluginApp> PluginRuntime<PA> {
    /// 🏇️ Borrows the embedding owner's unchanged phase authority for each real reactor turn.
    pub fn mounted_owner_policy(&self) -> MountedOwnerPolicyV1 { self.mounted_policy }
    /// 🧩️ Creates an empty typed runtime without erasing the plugin-owned app enum.
    pub fn new(mounted_policy: MountedOwnerPolicyV1) -> Result<Self, semio_framework_value::ValueError> {
        let mounted_policy = mounted_policy.validate()?;
        Ok(Self {
            mounted_policy,
            guest_lifetimes: RefCell::new(crate::reactor::instance_lifetime::NativeLifecycleRegistry::new()),
            plugin: RefCell::new(None),
            plugin_assembly_error: RefCell::new(None),
            instances: RefCell::new(RuntimeInstanceRegistry::new()),
            close_quarantine: RefCell::new(RuntimeInstanceRegistry::new()),
            close_cleanup_cursor: Cell::new(0),
            live_cleanup_cursor: Cell::new(0),
            live_cleanup_pressure: Cell::new(false),
            typed_continuation_cursor: Cell::new(0),
            close_generation: Cell::new(0),
            local_interaction_query_generation: Default::default(),
            instance_actors: RefCell::new(RuntimeInstanceRegistry::new()),
            document_backbones: RefCell::new(RuntimeInstanceRegistry::new()),
        })
    }
}
/// 🌉️ Wire encode over `T: ToValue` directly — no `serde_json` detour. Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: every caller's `T`
/// (`Fault`, `Effect`, `DispatchReport`, …) now carries a first-party `ToValue` impl.
fn encode_wire_serialized<T: ToValue>(value: &T) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&key_ordered_wire_value(value.to_value()))
}
/// 🔡️ `value` with the members of every object in key-byte order, through arrays and objects alike — the one order a host
/// reader re-derives from what it decoded. The pack encoder keeps the order a value arrives in and a derived `to_value`
/// arrives in declaration order, which a reader that re-encodes what it read (the history patch of a publication, the
/// document-port control grammar) refuses as non-canonical (live fault F9 on build B1).
fn key_ordered_wire_value(value: DslValue) -> DslValue {
    use semio_framework_value::DslValue;
    match value {
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(key_ordered_wire_value).collect()),
        DslValue::Object(members) => {
            let mut members: Vec<(String, DslValue)> = members.into_iter().map(|(key, member)| (key, key_ordered_wire_value(member))).collect();
            members.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            DslValue::Object(members)
        }
        scalar => scalar,
    }
}
async fn push_app_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, fault: Fault) {
    frames.push(protocol::AppFrame::Error { in_reply_to, fault: encode_wire_serialized(&fault), report: Vec::new() });
}
async fn push_dispatch_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: u64, fault: Fault, report: &protocol::DispatchReport) {
    frames.push(protocol::AppFrame::Error { in_reply_to: Some(in_reply_to), fault: encode_wire_serialized(&fault), report: encode_wire_serialized(report) });
}
async fn push_os_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, code: &str, message: String) {
    push_app_fault(frames, in_reply_to, Fault::new(FaultOrigin::Os, FaultCode::new(code), message)).await;
}
// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
fn plugin_internal_fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), message)
}
pub(crate) fn debug_runtime_line(line: impl std::fmt::Display) {
    if semio_framework_trace::runtime_diagnostics_enabled() {
        eprintln!("{line}");
    }
}
/// 🌉️ Twin of `encode_wire_serialized` above — wire decode over `T: FromValue` directly.
pub(crate) async fn decode_wire_serialized<T: FromValue>(bytes: &[u8]) -> Result<T, Fault> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| plugin_internal_fault(error.to_string()))?;
    T::from_value(value).map_err(|error| plugin_internal_fault(error.to_string()))
}
async fn decode_wire_serialized_or<T: FromValue>(bytes: &[u8], default: T) -> T {
    decode_wire_serialized(bytes).await.unwrap_or(default)
}
/// ⚖️ Executes the channel's local policy command and returns its canonical acknowledgment frame.
/// 🗃️ sdk-dedyn (O1/§1.5): generic `<PA: PluginApp>` replaces `&mut dyn PluginApp`.
async fn set_merge_policy_frames<PA: PluginApp>(app: &mut PA, seq: u64, policy: u8) -> Result<Vec<protocol::AppFrame>, Fault> {
    let policy = protocol::MergePolicy::from_u8(policy).ok_or_else(|| Fault::new(FaultOrigin::Os, FaultCode::new("merge.invalid-policy"), format!("unknown merge policy ordinal {policy}")))?;
    app.set_merge_policy(policy).await;
    Ok(vec![protocol::AppFrame::Done { in_reply_to: seq }])
}
/// ⚔️ Executes the channel's conflict-resolution command and returns the authoritative report and open projection.
async fn resolve_conflict_frames<PA: PluginApp>(app: &mut PA, seq: u64, conflict_id: String, resolution: u8) -> Result<Vec<protocol::AppFrame>, Fault> {
    let resolution = match resolution {
        0 => protocol::ConflictResolution::Accept,
        1 => protocol::ConflictResolution::Discard,
        _ => return Err(Fault::new(FaultOrigin::Os, FaultCode::new("merge.invalid-resolution"), format!("unknown conflict resolution ordinal {resolution}"))),
    };
    let report = app.resolve_conflict(&conflict_id, resolution).await?;
    let conflicts = app.open_conflicts().await;
    Ok(vec![protocol::AppFrame::MergeReport { in_reply_to: Some(seq), report: encode_wire_serialized(&report) }, protocol::AppFrame::Conflicts { in_reply_to: Some(seq), conflicts: encode_wire_serialized(&conflicts) }])
}
/// ⚔️ Executes the channel's open-conflict read command and returns its canonical projection frame.
async fn read_conflicts_frames<PA: PluginApp>(app: &PA, seq: u64) -> Vec<protocol::AppFrame> {
    vec![protocol::AppFrame::Conflicts { in_reply_to: Some(seq), conflicts: encode_wire_serialized(&app.open_conflicts().await) }]
}
/// 🪪️ Reads the actor retained by exact instance-open admission.
pub(crate) async fn instance_actor<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<String, Fault> {
    runtime.instance_actors.try_borrow().map_err(|_| plugin_internal_fault("runtime actor authority is busy"))?.get(instance_id).map(RuntimeActorAuthority::to_string).ok_or_else(|| plugin_internal_fault("instance has no admitted actor authority"))
}
/// 🧬️ The local async mutex keeps the instance in-place across a suspending app call. Dropping an
/// in-flight future releases the RAII guard and wakes the next caller without losing the instance.
async fn with_instances_mut<PA: PluginApp, R, F: FnOnce(&mut RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>) -> Result<R, Fault>>(runtime: &PluginRuntime<PA>, f: F) -> Result<R, Fault> {
    let mut instances = runtime.instances.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?;
    f(&mut instances)
}
/// 🎟️ Keeps an action source's complete original failure under the actual runtime registry loan.
async fn with_instances_turn_mut<PA:PluginApp,R>(runtime:&PluginRuntime<PA>,f:impl FnOnce(&mut RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<R,semio_framework_job::OriginalTurnFailure>{let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority is busy"))?;f(&mut instances)}
/// 🪢️ Returns complete lifecycle causes through the same original registry authority.
async fn with_instances_original_mut<PA:PluginApp,R>(runtime:&PluginRuntime<PA>,f:impl FnOnce(&mut RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>)->Result<R,crate::PluginLifecycleFailure>)->Result<R,crate::PluginLifecycleFailure>{let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority is busy"))?;f(&mut instances)}
/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: registers `instance` directly into `INSTANCES`,
/// bypassing `plugin_create_app_with_id`'s `PLUGIN`/`PluginProgram` requirement — a real
/// `AsyncTask` resume test only needs A `dyn PluginApp` at a known id, not a fully assembled
/// plugin bundle. `#[cfg(test)]`-gated, never part of the real API surface.
#[cfg(test)]
async fn test_push_instance<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance: AppInstance<PA>) {
    with_instances_mut(runtime, |list| list.insert(instance.id, std::sync::Arc::new(RuntimeAppCell::new(instance, runtime.mounted_policy))).map_err(|_| plugin_internal_fault("fixed runtime instance authority is saturated or collided")))
        .await
        .expect("test_push_instance must not fail");
}
/// 🪪️ Admits the actor that an instance pushed straight into a test runtime would otherwise lack: the exact authority instance-open admission retains for every later exchange, resume and restore.
#[cfg(test)]
fn test_admit_instance_actor<PA: PluginApp>(runtime: &PluginRuntime<PA>, id: u32, actor: &str) {
    let authority = RuntimeActorAuthority::new(actor).expect("test actor authority");
    runtime.instance_actors.try_borrow_mut().expect("test actor authority is free").insert_admitted(id, authority);
}
static NEXT_INSTANCE_ID: AtomicU32 = AtomicU32::new(1);
pub fn install_plugin_bundle<PA: PluginApp>(runtime: &PluginRuntime<PA>, bundle: Plugin<PA>) {
    *runtime.plugin.borrow_mut() = Some(bundle);
    *runtime.plugin_assembly_error.borrow_mut() = None;
}
/// 🧩️ Installs a successful plugin bundle or retains its typed assembly fault for WIT startup calls.
pub fn install_plugin_bundle_result<PA: PluginApp>(runtime: &PluginRuntime<PA>, bundle: Result<Plugin<PA>, PluginAssemblyError>) {
    match bundle {
        Ok(bundle) => install_plugin_bundle(runtime, bundle),
        Err(error) => {
            *runtime.plugin_assembly_error.borrow_mut() = Some(Fault::new(FaultOrigin::Plugin, FaultCode::new(error.code), error.message));
        }
    }
}
/// 🩹️ Reports every guest panic through the WASI stderr stream before the target's abort trap ends
/// the instance. The embedding crate's runtime ensure (`plugin_exports!`/`extension_exports!`)
/// calls it once, before its bundle is assembled, so a panic during assembly is reported too.
// 🚫️async: E1 called from the sync `std::sync::Once` closure of the macro-generated runtime ensure.
pub fn install_guest_panic_report() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|panic| eprintln!("[semio-plugin panic] {panic}")));
}
pub async fn plugin_manifest<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> PluginManifest {
    if let Some(fault) = runtime.plugin_assembly_error.borrow().clone() {
        return PluginManifest {
            plugin_id: "assembly-failed".into(),
            label: fault.message,
            version: "0.0.0".into(),
            apps: vec![],
            examples: vec![],
            capabilities: vec![],
            topic_contributions: vec![],
            commands: vec![],
            artifact_kinds: vec![],
            hosted_artifact_kinds: Vec::new(),
            dependencies: vec![],
            contributions: vec![],
        };
    }
    runtime.plugin.borrow().as_ref().map_or_else(
        || PluginManifest {
            plugin_id: "empty".into(),
            label: "Empty".into(),
            version: "0.0.0".into(),
            apps: vec![],
            examples: vec![],
            capabilities: vec![],
            topic_contributions: vec![],
            commands: vec![],
            artifact_kinds: vec![],
            hosted_artifact_kinds: Vec::new(),
            dependencies: vec![],
            contributions: vec![],
        },
        Plugin::manifest,
    )
}
/// 🛂️ Builder-declared descriptor fields owned by the same Plugin as its manifest.
#[derive(Clone, Default)]
pub struct PluginDescriptorExtras {
    pub package_id: String,
    pub activation_events: Vec<ActivationEvent>,
    pub capability_requests: Vec<CapabilityRequest>,
    pub extension_points: Vec<ExtensionPointDeclaration>,
    pub execution: ExecutionMode,
    pub quotas: QuotaSchema,
    pub assets: Vec<AssetDeclaration>,
}
/// 📖️ Reads only the descriptor authority installed in this exact runtime.
pub async fn plugin_descriptor_extras<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> PluginDescriptorExtras {
    runtime
        .plugin
        .borrow()
        .as_ref()
        .map(|plugin| {
            let mut extras = plugin.descriptor_extras().clone();
            for declaration in plugin.deferred_example_assets() {
                match extras.assets.iter().position(|asset| asset.name == declaration.name) {
                    Some(existing) => extras.assets[existing] = declaration.clone(),
                    None => extras.assets.push(declaration.clone()),
                }
            }
            extras
        })
        .unwrap_or_default()
}
/// 🛂️ Re-encodes a packed SEMANTIC `PackageDescriptor` canonically — shared by `plugin_exports!`/
/// `extension_exports!`'s `descriptor_is_fresh()` tests. A native `describe_plugin()`/`describe_extension()`
/// and the committed `🛂️.descriptor.semio` are both semantic (no `hashes` binding: byte identity lives in the
/// generated `ComponentReceiptV1`, ticket 26/09/23 wave A2), so freshness is a plain byte comparison of the
/// two canonical encodings. `None` on any decode failure — a descriptor carrying any byte binding among them,
/// since the descriptor schema has no such member (caller treats that as "cannot compare", same as a missing file).
/// 🌉️ Uses the descriptor's first-party structural codec so freshness observes exactly the
/// same wire contract as guest `describe()` and never introduces a serde-shaped adapter.
pub fn canonical_semantic_descriptor_bytes(bytes: &[u8]) -> Option<Vec<u8>> {
    let value = store::pack_rt::decode_wire_value(bytes).ok()?;
    let descriptor: semio_framework::PackageDescriptor = FromValue::from_value(value).ok()?;
    Some(store::pack_rt::encode_wire_value(semio_framework::CanonicalDescriptorValue::new(ToValue::to_value(&descriptor)).value()))
}
/// 💡️ Lists only inference services frozen into the installed plugin assembly.
///
/// 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready` (no real suspension
/// point: the found-plugin branch resolves a frozen in-memory assembly).
pub async fn plugin_wire_list_artifact_inference_services<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> Result<Vec<u8>, crate::artifact_inference::ArtifactInferenceExecutionError> {
    {
        let plugin = runtime.plugin.borrow();
        let plugin = plugin.as_ref().ok_or_else(|| crate::artifact_inference::ArtifactInferenceExecutionError::new("artifact-inference.not-registered", "no plugin bundle is installed"))?;
        Ok(plugin.wire_list_artifact_inference_services())
    }
}
/// 💡️ Executes only an inference service frozen into the installed plugin assembly.
///
/// 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready` (no real suspension
/// point: the found-plugin branch resolves a frozen in-memory assembly).
pub async fn plugin_wire_artifact_infer_cold<PA: PluginApp>(runtime: &PluginRuntime<PA>, request: &[u8],operation:u64,generation:u64,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<u8>, crate::artifact_inference::ArtifactInferenceExecutionError> {
    {
        let plugin = runtime.plugin.borrow();
        let plugin = plugin.as_ref().ok_or_else(|| crate::artifact_inference::ArtifactInferenceExecutionError::new("artifact-inference.not-registered", "no plugin bundle is installed"))?;
        plugin.wire_artifact_infer_cold(request,operation,generation,cx)
    }
}
/// 🎯️ Lists only mutation rows frozen into the installed plugin assembly.
///
/// 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready` for both the found-plugin
/// and no-plugin-yet branches (neither has a real suspension point).
pub async fn plugin_wire_list_artifact_mutations<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> Vec<u8> {
    match runtime.plugin.borrow().as_ref() {
        Some(plugin) => Plugin::wire_list_artifact_mutations(plugin),
        None => encode_wire_serialized(&Vec::<crate::artifact_registration::WireMutationRosterEntry>::new()),
    }
}
/// 🎯️ Plans only a mutation service frozen into the installed plugin assembly.
///
/// 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready` (no real suspension
/// point: wire encode/decode plus a frozen in-memory mutation-plan lookup).
pub async fn plugin_wire_artifact_mutation_plan<PA: PluginApp>(runtime: &PluginRuntime<PA>, request: &[u8]) -> Result<Vec<u8>, Fault> {
    runtime.plugin.borrow().as_ref().ok_or_else(|| plugin_internal_fault("no plugin bundle is installed"))?.wire_artifact_mutation_plan(request)
}
/// 📖️ This plugin's deterministic `contributor.list-artifact-mutations` roster: every owner
/// document app's `SemanticMutation::kinds()` (`"<document-schema>#<kebab-kind>"`, committed by
/// `PluginBuilder::document_app`/`::try_build`) plus every resolved `ArtifactContribution`'s
/// mutations (`"<target-document-schema>#<plugin-id>:<kebab-kind>"`, committed by
/// `PluginBuilder::try_build`/`ExtensionBundle::contributes`) — `crate::app::mutation_roster_entries`
/// sorts by `mutation_id`, so repeated calls here never reorder or duplicate rows.
pub async fn wire_list_artifact_mutations() -> Vec<u8> {
    encode_wire_serialized(&crate::artifact_registration::mutation_roster_entries().await)
}
/// 🎯️ `contributor.artifact-mutation-plan`: decodes the target snapshot pack + contributor
/// payload, runs the registered contribution's `CompositeMutationKind::plan` through a fresh
/// `protocol::Planner`, and echoes `artifact_kind`/`mutation_id`/`revision`/`generation` back
/// alongside the resulting `owner_ops`/`label`/`foreign` — mirrors `wire_artifact_infer`'s decode →
/// validate-echo → execute → encode shape (`validate_wire_request_metadata`'s discipline, applied
/// to `artifact_kind`/`mutation_id` instead of the inference metadata tuple: `crate::
/// app::contributed_mutation_plan` rejects a request whose `artifact_kind` does not match the
/// mutation's own registered target before ever touching the snapshot/payload bytes).
pub async fn wire_artifact_mutation_plan(request: &[u8]) -> Result<Vec<u8>, Fault> {
    let request: crate::artifact_registration::WireArtifactMutationPlanRequest = decode_wire_serialized(request).await?;
    if request.mutation_id.trim().is_empty() {
        return Err(plugin_internal_fault("artifact-mutation-plan requires a non-empty mutation_id"));
    }
    let output = crate::artifact_registration::contributed_mutation_plan(&request.mutation_id, &request.artifact_kind, &request.snapshot_pack, &request.payload).await.map_err(|error| plugin_internal_fault(error.to_string()))?;
    let result = crate::artifact_registration::WireArtifactMutationPlanResult {
        artifact_kind: request.artifact_kind,
        mutation_id: request.mutation_id,
        revision: request.revision,
        generation: request.generation,
        owner_ops: output.owner_ops,
        label: output.label,
        foreign: output.foreign,
    };
    Ok(encode_wire_serialized(&result))
}
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-contributed-mutation-wire/🦀️.rs");
/// 🧳️ Keeps the exact opening inputs and partial app through every original caller admission turn.
pub(crate) struct PendingNativeCreation<PA:PluginApp>{request:semio_framework::kernel::ActorInstanceOpenRequest,source:std::mem::ManuallyDrop<Option<(String,String)>>,app:std::mem::ManuallyDrop<Option<PA>>,loan:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,identity:(semio_framework_job::OperationId,semio_framework_job::Generation),parent:usize,returning_loan:bool,authority:Option<RuntimeActorAuthority>,issuer_ready:bool,closing:bool,failure:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalTurnFailure>>,metadata_source:std::mem::ManuallyDrop<Option<String>>,metadata_closing:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>>,source_closing:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<(String,String)>>>}
impl<PA:PluginApp> PendingNativeCreation<PA>{
 fn validate(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{if self.identity!=(cx.operation(),cx.generation())||self.parent!=cx.original_source_parent()?.source_identity()?{return Err(plugin_internal_fault("original opening continuation requires its same actor creation source").into())}if !self.returning_loan{if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)?}}Ok(())}
}
/// 🪪️ Retains original request inputs before factory work; a denied issuer never erases the partial app.
pub(crate) async fn plugin_open_actor_instance<PA:PluginApp>(runtime:&PluginRuntime<PA>,request:semio_framework::kernel::ActorInstanceOpenRequest,app_id:String,actor:String,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 let mut lifetimes=runtime.guest_lifetimes.try_borrow_mut().map_err(|_|plugin_internal_fault("lifecycle authority busy"))?;let slot=lifetimes.get_mut(request.instance_id).filter(|slot|slot.cell.matches_open(request)).ok_or_else(||plugin_internal_fault("exact opening lifecycle missing"))?;
 if slot.native_created||slot.cell.owner().is_some(){return Err(plugin_internal_fault("native opening already captured").into())}
 if slot.pending_creation.is_some(){return Err(plugin_internal_fault("original opening inputs already remain owned").into())}
 *slot.pending_creation=Some(PendingNativeCreation{request,source:std::mem::ManuallyDrop::new(Some((app_id,actor))),app:std::mem::ManuallyDrop::new(None),loan:std::mem::ManuallyDrop::new(None),identity:(cx.operation(),cx.generation()),parent:cx.original_source_parent()?.source_identity()?,returning_loan:false,authority:None,issuer_ready:false,closing:false,failure:std::mem::ManuallyDrop::new(None),metadata_source:std::mem::ManuallyDrop::new(None),metadata_closing:std::mem::ManuallyDrop::new(None),source_closing:std::mem::ManuallyDrop::new(None)});drop(lifetimes);
 plugin_resume_actor_opening(runtime,request.instance_id,identity,cx).await
}
async fn plugin_resume_actor_opening<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 match plugin_advance_actor_opening(runtime,instance_id,identity,cx).await{Ok(ready)=>Ok(ready),Err(failure)=>{let mut slots=runtime.guest_lifetimes.try_borrow_mut().map_err(|_|plugin_internal_fault("opening refusal authority busy"))?;let Some(opening)=slots.get_mut(instance_id).and_then(|slot|slot.pending_creation.as_mut())else{return Err(failure)};if opening.closing{return Err(failure)}opening.closing=true;*opening.failure=Some(failure);Ok(false)}}
}
async fn plugin_advance_actor_opening<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 let mut lifetimes=runtime.guest_lifetimes.try_borrow_mut().map_err(|_|plugin_internal_fault("lifecycle authority busy"))?;let slot=lifetimes.get_mut(instance_id).ok_or_else(||plugin_internal_fault("original opening lifecycle missing"))?;let Some(opening)=slot.pending_creation.as_mut()else{return Ok(slot.native_created)};opening.validate(cx)?;
 if cx.should_yield(){return Ok(false)}
 if cx.is_cancelled()&&!opening.closing{opening.closing=true;*opening.failure=Some(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original actor creation was canceled before publication").into());}
 if opening.loan.is_none()&&!opening.returning_loan{let parent=cx.original_source_parent()?;let Some(loan)=parent.admit_loan(cx)?else{return Ok(false)};*opening.loan=Some(loan);return Ok(false)}
 if opening.closing{
  if let Some(app)=opening.app.as_mut(){
   if !app.close_terminal_is_empty(){let grant=runtime_lifecycle_grant();app.close_step(cx,grant)?;return Ok(false)}
   if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}opening.app.take();opening.authority=None;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
  }
 }else{
 let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority busy"))?;let mut quarantine=runtime.close_quarantine.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime quarantine authority busy"))?;let mut actors=runtime.instance_actors.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime actor authority busy"))?;let mut document_backbones=runtime.document_backbones.try_borrow_mut().map_err(|_|plugin_internal_fault("document backbone binding authority busy"))?;
 if !instances.can_insert(instance_id)||!quarantine.can_insert(instance_id)||!actors.can_insert(instance_id)||!document_backbones.can_insert(instance_id){return Err(plugin_internal_fault("native opening slot collided").into())}
 if opening.app.is_none(){
  if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
  let source=opening.source.as_ref().ok_or_else(||plugin_internal_fault("original opening source missing"))?;let actor=RuntimeActorAuthority::new(&source.1)?;
  if let Some(fault)=runtime.plugin_assembly_error.try_borrow().map_err(|_|plugin_internal_fault("plugin assembly authority busy"))?.clone(){return Err(fault.into())}
  let program=runtime.plugin.try_borrow().map_err(|_|plugin_internal_fault("plugin factory authority busy"))?;let program=program.as_ref().ok_or_else(||plugin_internal_fault("plugin not initialized"))?;
  let mut app=program.create_app(&source.0,protocol::ActorId(actor.to_string().into()),runtime.mounted_policy,identity).ok_or_else(||plugin_internal_fault("unknown app"))?;
  ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(instance_id));::semio_framework_async::poll::resolve_ready(app.bind_actor(&source.1));opening.authority=Some(actor);*opening.app=Some(app);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
 }
 if !opening.issuer_ready{if !opening.app.as_mut().unwrap().admit_cancellation_issuer(cx)?{return Ok(false)}opening.issuer_ready=true;return Ok(false)}
 }
 if let Some(source)=opening.source_closing.as_mut(){
  if !source.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,source.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|source.step(grant))?;return Ok(false)}
  if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}opening.source_closing.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
 }
 if opening.source.is_some(){
  if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}let source=opening.source.take().unwrap();match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(source)=>*opening.source_closing=Some(source),Err((failure,source))=>{*opening.source=Some(source);return Err(failure.into())}}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
 }
 if opening.closing{
  if let Some(owner)=opening.metadata_closing.as_mut(){if !owner.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.step(grant))?;return Ok(false)}if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}opening.metadata_closing.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  if opening.metadata_source.is_some(){
   if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
   let source=opening.metadata_source.take().unwrap();
   match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>*opening.metadata_closing=Some(owner),Err((failure,source))=>{*opening.metadata_source=Some(source);return Err(failure.into())}}
   cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
  }
  if crate::reactor::take_original_instance_metadata_app_id(instance_id,&mut opening.metadata_source,cx)?!=Some(true){return Ok(false)}
 }
 if let Some(loan)=opening.loan.as_mut(){opening.returning_loan=true;let parent=cx.original_source_parent()?;loan.close_step(parent,cx)?;if loan.terminal_is_empty(){opening.loan.take();}return Ok(false)}
 if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
 if opening.closing{let failure=opening.failure.take();slot.pending_creation.take();lifetimes.remove_uncreated(instance_id).map_err(plugin_internal_fault)?;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return match failure{Some(failure)=>Err(failure),None=>Ok(false)}}
 let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority busy"))?;let mut actors=runtime.instance_actors.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime actor authority busy"))?;
 let actor=opening.authority.take().ok_or_else(||plugin_internal_fault("original opening actor authority missing"))?;let app=opening.app.take().unwrap();let cell=std::sync::Arc::new(RuntimeAppCell::new(AppInstance{id:instance_id,app,surface_contexts:Default::default()},runtime.mounted_policy));let lease=PluginInstanceCloseLease::from_cell(instance_id,&cell);let owner=crate::reactor::instance_lifetime::NativeLifetimeOwner::from_lease(slot.cell.lifetime(),lease).expect("preflighted exact native lifetime");slot.cell.install_owner(owner).unwrap_or_else(|_|panic!("preflighted opening owner slot"));instances.insert_admitted(instance_id,cell);actors.insert_admitted(instance_id,actor);slot.native_created=true;slot.pending_creation.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}
/// 🔁️ Resumes the original allocation-bound opening once per supplied caller turn without running its factory again.
pub(crate) async fn plugin_resume_actor_openings<PA:PluginApp>(runtime:&PluginRuntime<PA>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 let instance={let lifetimes=runtime.guest_lifetimes.try_borrow().map_err(|_|plugin_internal_fault("lifecycle authority busy"))?;lifetimes.pending_opening()};if let Some(instance)=instance{plugin_resume_actor_opening(runtime,instance,identity,cx).await?;}Ok(())
}
pub async fn plugin_create_app<PA: PluginApp>(runtime: &PluginRuntime<PA>, app_id: &str, actor: protocol::ActorId, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<u32, Fault> {
    plugin_create_app_with_id(runtime, NEXT_INSTANCE_ID.fetch_add(1, Ordering::SeqCst), app_id, actor, identity).await
}
/// 🧬️ Constructs every store under the explicit instance id and opened actor before publishing the app.
pub async fn plugin_create_app_with_id<PA: PluginApp>(runtime: &PluginRuntime<PA>, id: u32, app_id: &str, actor: protocol::ActorId, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<u32, Fault> {
    if let Some(fault) = runtime.plugin_assembly_error.borrow().clone() {
        return Err(fault);
    }
    if !runtime.instances.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?.can_insert(id)
        || !runtime.close_quarantine.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime close quarantine is busy"))?.can_insert(id)
        || !runtime.instance_actors.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime actor authority is busy"))?.can_insert(id)
    {
        return Err(plugin_internal_fault(format!("fixed instance authority is saturated, collided, or quarantined: {id}")));
    }
    {
        let program = runtime.plugin.borrow();
        let program = program.as_ref().ok_or_else(|| plugin_internal_fault("plugin not initialized"))?;
        let authority = RuntimeActorAuthority::new(&actor.0)?;
        let mut app = program.create_app(app_id, actor, runtime.mounted_policy, identity).ok_or_else(|| plugin_internal_fault(format!("unknown app: {app_id}")))?;
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(id));
        runtime.instance_actors.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime actor authority is busy"))?.insert_admitted(id, authority);
        ::semio_framework_async::poll::resolve_ready(with_instances_mut(runtime, |list| {
            list.insert_admitted(id, std::sync::Arc::new(RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy)));
            Ok(())
        }))?;
        Ok(id)
    }
}
/// 🚪️ Bytes one app close/maintenance step may retire — one PAGE, the same unit the reactor turn's
/// own retirement ladder is priced in (`PATCH_RETIREMENT_BYTES_PER_UNIT`). At 4 096 it was exactly
/// `ARTIFACT_OUTPUT_CHUNK_BYTES`, so a retained item larger than one output chunk — a tessellated
/// preview mesh in a displaced window transient — could not be released by ANY number of steps and
/// the ladder answered `Pending { 0, 0 }` until the structural accountant killed the close with
/// `plugin.internal.zero-progress` (reproduced by the close-cost fixture's eight-document session,
/// ticket 26/09/09).
const RUNTIME_CLOSE_BYTES_PER_STEP: usize = 32 * 1_024;
/// 🎟️ Runtime admission bounds price logical copy, constructor capacity and physical release independently.
pub(crate) fn runtime_lifecycle_grant()->RetainedCloneGrant {
    RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RUNTIME_CLOSE_BYTES_PER_STEP,maximum_capacity_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES,maximum_release_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES,maximum_depth:4096}
}
const RUNTIME_CLOSE_INNER_GRANT_US: u64 = 2_000;
const RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT: u8 = 8;
/// ⏱️ Real time a close ladder must spend making no progress before the structural accountant is
/// allowed to call it stalled — the SECOND half of the zero-progress verdict, next to the step
/// count above.
///
/// 🐛️ The count alone was the whole rule, and a count is not a duration: eight consecutive steps
/// that each observe an empty ladder cost microseconds, so a close that had nothing left to retire
/// faulted with `plugin.internal.zero-progress` reporting `elapsed 0us` against an `8000us`
/// ceiling — a stall credit judged exhausted before any time had passed (six-pane demonstrator
/// boot, ticket 26/08/28, `📓️fix-2026-09-16-concurrent-boot-timeout-and-close-fault.md`). A ladder
/// that is genuinely wedged still burns this credit within a few reactor turns; one that is merely
/// finishing never does.
const RUNTIME_CLOSE_STALL_CREDIT_US: u64 = semio_framework_trace::INTERACTIVE_STEP_CEILING_US;
struct RuntimeLiveCleanupPump<PA: PluginApp> {
    session: Option<semio_framework_job::BatchJobSession<RuntimeLiveCleanupJob<PA>>>,
    rejected: Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<RuntimeLiveCleanupJob<PA>>>,
    pub(crate) outcome_pending: bool,
    terminal: bool,
    pending_status: RuntimeMaintenanceStatus,
    pub(crate) closing: bool,
    faulted: bool,
    pub(crate) resume_pending: bool,
}
impl<PA: PluginApp> RuntimeLiveCleanupPump<PA> {
    pub(crate) fn new() -> Self {
        Self { session: None, rejected: None, outcome_pending: false, terminal: false, pending_status: RuntimeMaintenanceStatus::Ready, closing: false, faulted: false, resume_pending: false }
    }
    fn retirement_demands(&self,body:usize)->Result<RetirementDemand,semio_framework_value::ValueError>{
        let metadata=|bytes|RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};
        let quote=if let Some(rejected)=self.rejected.as_ref(){if rejected.terminal_is_empty(){return Ok(metadata(size_of_val(rejected)));}rejected.retirement_demands(body)?}
        else if let Some(session)=self.session.as_ref(){if session.terminal_is_empty(){return Ok(metadata(size_of_val(session)));}session.retirement_demands(body).map_err(|error|match error{semio_framework_job::WorkerJobDemandError::Refused(error)=>error,semio_framework_job::WorkerJobDemandError::Contention(_)=>semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"runtime cleanup original session remains checked out")})?}
        else{return Ok(Default::default());};
        Ok(RetirementDemand{depth:quote.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"runtime cleanup session depth overflow"))?,..quote})
    }
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep as Step,WorkerJobCloseStep as Worker};
        let grant=cx.retained_grant();if cx.should_yield(){return Ok(Step::Pending{progress:Default::default()})}
        let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Step::Pending{progress:Default::default()});}
        self.closing=true;
        let metadata=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        if let Some(rejected)=self.rejected.as_mut(){
            if rejected.terminal_is_empty(){cx.consume_retained(metadata)?;drop(self.rejected.take());cx.consume_fuel(1);return Ok(Step::Pending{progress:metadata});}
            let step=rejected.close_step(cx)?.admit(grant,rejected.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
            return Ok(match step{Step::Complete{progress}=>Step::Pending{progress},step=>step});
        }
        if let Some(session)=self.session.as_mut(){
            if session.terminal_is_empty(){cx.consume_retained(metadata)?;drop(self.session.take());self.outcome_pending=false;self.resume_pending=false;cx.consume_fuel(1);return Ok(Step::Pending{progress:metadata});}
            session.begin_close();
            let step=match session.close_step(cx)?{Worker::Pending{progress}|Worker::Complete{progress}=>Step::Pending{progress},Worker::Blocked=>Step::Blocked};
            return step.admit(grant,false).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        Ok(Step::Complete{progress:Default::default()})
    }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.closing && self.session.is_none() && self.rejected.is_none() && !self.outcome_pending
    }
}
struct RuntimeCloseCleanupPump<PA: PluginApp> {
    session: Option<semio_framework_job::BatchJobSession<RuntimeCloseCleanupJob<PA>>>,
    rejected: Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<RuntimeCloseCleanupJob<PA>>>,
    pub(crate) outcome_pending: bool,
    terminal: bool,
    complete: bool,
    blocked: bool,
    pending_status: RuntimeCloseStatus,
    faulted: bool,
    pub(crate) resume_pending: bool,
}
impl<PA: PluginApp> RuntimeCloseCleanupPump<PA> {
    pub(crate) fn new() -> Self {
        Self { session: None, rejected: None, outcome_pending: false, terminal: false, complete: false, blocked: false, pending_status: RuntimeCloseStatus::Ready, faulted: false, resume_pending: false }
    }
}
use crate::reserved_job_outcomes::CleanupTurn;
struct RuntimeLiveCleanupJob<PA: PluginApp> {
    instance_id: u32,
    pub(crate) generation: semio_framework_job::Generation,
    cell: Option<std::sync::Weak<RuntimeAppCell<PA>>>,
    progress: Option<PluginLifecycleStep>,
    contended: bool,
    publication: crate::reserved_job_outcomes::CleanupPublication,
    pub(crate) closing: bool,
}
impl<PA: PluginApp> crate::reserved_job_outcomes::CleanupJob for RuntimeLiveCleanupJob<PA> {
    fn advance(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> CleanupTurn {
        RuntimeLiveCleanupJob::advance(self, cx)
    }
    fn instance_id(&self) -> u32 {
        self.instance_id
    }
    fn publication_mut(&mut self) -> &mut crate::reserved_job_outcomes::CleanupPublication {
        &mut self.publication
    }
}
impl<PA: PluginApp> RuntimeLiveCleanupJob<PA> {
    fn advance(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->CleanupTurn{
        let Self{instance_id,cell,progress,contended,..}=self;let Some(original)=cell.as_ref()else{return CleanupTurn::Cancelled};
        match cx.with_original_native_weak_source(original,|cell,cx|Ok(Self::advance_source(cell,*instance_id,progress,contended,cx))){Ok(turn)=>turn,Err(original)=>CleanupTurn::OriginalFailure(original)}
    }
    fn advance_source(cell:&RuntimeAppCell<PA>,instance_id:u32,outcome_progress:&mut Option<PluginLifecycleStep>,contended:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->CleanupTurn{
        cx.set_stage("plugin-runtime-live-cleanup");
        *outcome_progress = None;
        *contended = false;
        if cx.is_cancelled() {
            return CleanupTurn::Cancelled;
        }
        if cx.deadline_exceeded() || cx.fuel_exhausted() { return CleanupTurn::Yield; }
        let mut instance = match cell.background_instance() {
            Ok(instance) => instance,
            Err(RuntimeInstanceRefusal::Contended) => {
                *contended = true;
                return CleanupTurn::Yield;
            }
            Err(RuntimeInstanceRefusal::Poisoned) => {
                return CleanupTurn::Fault(b"plugin live cleanup authority is poisoned");
            }
        };
        #[cfg(any(test, target_arch = "wasm32"))]
        cell.maintenance_probe_entries.fetch_add(1, Ordering::Relaxed);
        let traced = semio_framework_trace::runtime_diagnostics_enabled();
        let maintenance_started_us = traced.then(semio_framework_job::default_now_us).flatten();
        let maintenance = (|| -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
            let phase = instance.app.mounted_owner_maintenance_phase()?;
            let identity = MountedOwnerIdentityV1 { instance_id: instance_id, operation: cx.operation(), generation: cx.generation() };
            let Some(mut turn) = MountedOwnerTurnV1::admit(instance_id, identity, phase, cell.mounted_policy, cx)? else { return Ok(PluginLifecycleStep::AwaitingInput { reason: "mounted maintenance retains its original scheduling authority" }); };
            let step = instance.app.mounted_owner_maintenance_step(&mut turn)?;
            if step.progress().unwrap_or_default() != turn.progress() || matches!(step, PluginLifecycleStep::Complete(_)) && !instance.app.maintenance_terminal_is_empty() { return Err(Fault::from("mounted maintenance receipt or terminal witness disagrees").into()); }
            Ok(step)
        })();
        cell.maintenance_pressure.store(instance.app.maintenance_under_pressure(), Ordering::Relaxed);
        if let (Some(started_us), Some(finished_us)) = (maintenance_started_us, maintenance_started_us.and_then(|_| semio_framework_job::default_now_us())) {
            if finished_us.saturating_sub(started_us) >= 2_000 {
                eprintln!("[TRACE] maintenance stage={} elapsed_us={} outcome={:?}", LAST_MAINTENANCE_STAGE.load(Ordering::Relaxed), finished_us - started_us, maintenance.as_ref().map(|_| ()).map_err(|failure| &failure.primary));
            }
        }
        match maintenance {
            Ok(progress @ PluginLifecycleStep::Progress(receipt)) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Checkpoint(receipt.copied_items as u64)
            }
            Ok(progress @ PluginLifecycleStep::Blocked { .. }) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Yield
            }
            Ok(progress @ PluginLifecycleStep::AwaitingInput { .. }) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Yield
            }
            Ok(progress @ PluginLifecycleStep::Complete(_)) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Complete
            }
            Err(error) => CleanupTurn::OriginalFailure(error.into()),
        }
    }
}
impl<PA: PluginApp> semio_framework_job::InteractiveJob for RuntimeLiveCleanupJob<PA> {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        crate::reserved_job_outcomes::drive_turn(self, cx)
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.publication.borrow_outcome(descriptor)
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.closing = true;
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(cx);
        }
        if self.cell.is_some(){return Ok(match cx.return_original_native_weak(&mut self.cell)?{Some(progress)=>semio_framework_job::InteractiveJobCloseStep::Complete{progress},None=>semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()}})}
        Ok(semio_framework_job::InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        let demand = self.publication.close_demands()?;
        Ok(if demand == RetirementDemand::default() { usize::from(self.cell.is_some()) } else { demand.depth })
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.cell.is_none() && self.publication.terminal_is_empty()
    }
}
fn runtime_live_cleanup_nonterminal_status(contended: bool, progress: Option<PluginLifecycleStep>, stalled_steps: &AtomicU32) -> RuntimeMaintenanceStatus {
    if contended {
        return RuntimeMaintenanceStatus::Ready;
    }
    if matches!(progress, Some(PluginLifecycleStep::AwaitingInput { .. })) {
        stalled_steps.store(0, Ordering::SeqCst);
        return RuntimeMaintenanceStatus::Ready;
    }
    let zero_progress = matches!(progress,Some(PluginLifecycleStep::Progress(receipt)) if receipt==RetainedCloneProgress::default())||matches!(progress,Some(PluginLifecycleStep::Blocked{..}));
    if !zero_progress {
        stalled_steps.store(0, Ordering::SeqCst);
        return RuntimeMaintenanceStatus::Ready;
    }
    let stalled = stalled_steps.fetch_add(1, Ordering::SeqCst).saturating_add(1);
    if stalled >= RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT {
        RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::ZeroProgress)
    } else {
        RuntimeMaintenanceStatus::Ready
    }
}
/// ⏱️ Publishes one maintenance turn: the measured wall cost is recorded next to the status so
/// the decode site can name the overrun in microseconds, and a clock failure or a ceiling
/// overrun outranks whatever the pump itself concluded.
fn runtime_live_cleanup_publish_turn<PA: PluginApp>(cell: &RuntimeAppCell<PA>, status: RuntimeMaintenanceStatus, elapsed: Result<u64, RuntimeCleanupFault>) {
    let verdict = match elapsed {
        Err(cause) => RuntimeMaintenanceStatus::Fault(cause),
        Ok(elapsed_us) if semio_framework_trace::interactive_step_contract_violated(elapsed_us) => {
            if semio_framework_trace::runtime_diagnostics_enabled() {
                eprintln!("[TRACE] cooperative maintenance callback overran the interactive ceiling for instance {} (elapsed {elapsed_us}us) — recorded, not fatal: the callback clock measures wall time including descheduling and the pumped foreign job step; the job-level step contract quarantines slow steps", cell.id);
            }
            status
        }
        Ok(_) => status,
    };
    cell.maintenance_fault_us.store(elapsed.unwrap_or(RUNTIME_CLEANUP_UNMEASURED_US), Ordering::SeqCst);
    cell.maintenance_status.store(verdict.repr(), Ordering::SeqCst);
}
fn run_runtime_live_cleanup_turn<PA:PluginApp+'static>(cell:&std::sync::Arc<RuntimeAppCell<PA>>,generation:semio_framework_job::Generation,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
    if cx.should_yield(){return Ok(())}
    let mut clock_source=cx.clock_source();let clock=runtime_callback_clock_begin(&mut clock_source);
    let status=cx.with_original_native_weak_parent(cell,|cx|match cell.maintenance_pump.try_lock(){Ok(mut pump)=>runtime_live_cleanup_pump_one(cell,generation,&mut pump,cx),Err(_)=>Ok(RuntimeMaintenanceStatus::Ready)})?;
    let elapsed=clock.and_then(|clock|runtime_callback_clock_elapsed(&mut clock_source,clock));
    runtime_live_cleanup_publish_turn(cell,status,elapsed);Ok(())
}
fn runtime_live_cleanup_pump_one<PA: PluginApp + 'static>(cell: &std::sync::Arc<RuntimeAppCell<PA>>, generation: semio_framework_job::Generation, pump: &mut RuntimeLiveCleanupPump<PA>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<RuntimeMaintenanceStatus,semio_framework_job::OriginalTurnFailure> {
    let grant=cx.retained_grant();
    if pump.closing {let _=pump.close_step(cx)?;return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::AlreadyClosing));}
    if let Some(rejected)=pump.rejected.as_mut().filter(|pending|!pending.admission_pending()||cx.is_cancelled()){
        if rejected.terminal_is_empty(){if grant.maximum_copy_bytes<size_of_val(rejected){return Ok(RuntimeMaintenanceStatus::Ready);}drop(pump.rejected.take());return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::AdmissionRejected));}
        rejected.close_step(cx)?;return Ok(RuntimeMaintenanceStatus::Ready)
    }
    if pump.outcome_pending{
        let Some(session)=pump.session.as_mut()else{return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::MissingSession))};
        let acknowledgement=session.acknowledge_outcome(grant);cx.consume_retained(acknowledgement.progress())?;if !matches!(acknowledgement,RetainedCloneStep::Complete(_)){return Ok(RuntimeMaintenanceStatus::Ready);}
        pump.outcome_pending=false;
        let Some(job) = session.checked_out_job_mut() else { return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::CheckedOutJob)) };
        pump.pending_status = runtime_live_cleanup_nonterminal_status(job.contended, job.progress, &cell.maintenance_stalled_steps);
        #[cfg(test)]
        if matches!(job.progress, Some(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput { .. })) {
            cell.maintenance_probe_input_waits.fetch_add(1, Ordering::SeqCst);
        }
        pump.resume_pending=true;
        return Ok(RuntimeMaintenanceStatus::Ready);
    }
    if pump.resume_pending{
        pump.resume_pending=false;
        if pump.terminal{if let Some(session)=pump.session.as_mut(){session.begin_close();}return Ok(RuntimeMaintenanceStatus::Ready);}
        return Ok(if pump.session.as_mut().is_some_and(|session|session.resume().is_err()){RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::Resume)}else{pump.pending_status});
    }
    if pump.terminal{
        let Some(session)=pump.session.as_mut()else{pump.terminal=false;return Ok(pump.pending_status);};
        if session.terminal_is_empty(){if grant.maximum_copy_bytes<size_of_val(session){return Ok(RuntimeMaintenanceStatus::Ready);}drop(pump.session.take());pump.terminal=false;return Ok(if pump.faulted{RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::PriorOutcome)}else{pump.pending_status});}
        session.close_step(cx)?;return Ok(RuntimeMaintenanceStatus::Ready)
    }
    if pump.session.is_none()&&pump.rejected.is_none(){let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RuntimeMaintenanceStatus::Ready)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);pump.rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(RuntimeLiveCleanupJob { instance_id: cell.id, generation, cell: Some(std::sync::Arc::downgrade(cell)), progress: None, contended: false, publication: crate::reserved_job_outcomes::CleanupPublication::new(), closing: false },(cx.operation(),cx.generation())));}
    if let Some(pending)=pump.rejected.as_mut(){
        let config=semio_framework_job::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_runtime_live_cleanup",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:RUNTIME_CLOSE_INNER_GRANT_US};
        match pending.admit_pending_batch(config,cx)?{Some(session)=>{pump.session=Some(session);drop(pump.rejected.take());},None=>return Ok(RuntimeMaintenanceStatus::Ready)}
        return Ok(RuntimeMaintenanceStatus::Ready);
    }
    let Some(session) = pump.session.as_mut() else { return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::MissingSession)) };
    match session.step_on_original_caller(cx) {
        Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) => {}
        Ok(_) => return Ok(RuntimeMaintenanceStatus::Ready),
        Err(failure) => return Err(failure),
    }
    if !session.checkout_outcome() {
        return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::Checkout));
    }
    if !session.take_checked_out_retained_step_receipt().is_some_and(|(issued, progress)| progress.fits(issued)) { return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::TakeOutcome)); }
    let (terminal, faulted) = match session.checked_out_outcome() {
        Ok(Some(view)) => (view.is_terminal(), matches!(view, semio_framework_job::JobOutcomeView::Fault { .. } | semio_framework_job::JobOutcomeView::Cancelled { .. })),
        Ok(None) => (false, false),
        Err(_) => return Ok(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::TakeOutcome)),
    };
    pump.terminal = terminal;
    pump.faulted = faulted;
    pump.outcome_pending = true;
    Ok(RuntimeMaintenanceStatus::Ready)
}
struct RuntimeCloseCleanupJob<PA: PluginApp> {
    state: Option<std::sync::Weak<RuntimeCloseWorkerState<PA>>>,
    instance_id: u32,
    progress: Option<PluginLifecycleStep>,
    contended: bool,
    publication: crate::reserved_job_outcomes::CleanupPublication,
    pub(crate) closing: bool,
}
impl<PA: PluginApp> crate::reserved_job_outcomes::CleanupJob for RuntimeCloseCleanupJob<PA> {
    fn advance(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> CleanupTurn {
        RuntimeCloseCleanupJob::advance(self, cx)
    }
    fn instance_id(&self) -> u32 {
        self.instance_id
    }
    fn publication_mut(&mut self) -> &mut crate::reserved_job_outcomes::CleanupPublication {
        &mut self.publication
    }
}
impl<PA: PluginApp> RuntimeCloseCleanupJob<PA> {
    fn advance(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->CleanupTurn{
        let Self{state,progress,contended,..}=self;let Some(original)=state.as_ref()else{return CleanupTurn::Cancelled};
        match cx.with_original_native_weak_source(original,|state,cx|Ok(Self::advance_source(state,progress,contended,cx))){Ok(turn)=>turn,Err(original)=>CleanupTurn::OriginalFailure(original)}
    }
    fn advance_source(state:&RuntimeCloseWorkerState<PA>,outcome_progress:&mut Option<PluginLifecycleStep>,contended:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->CleanupTurn{
        runtime_close_phase(&state, 8);
        cx.set_stage("plugin-runtime-close-cleanup");
        runtime_close_phase(&state, 9);
        *outcome_progress = None;
        *contended = false;
        if cx.is_cancelled() {
            return CleanupTurn::Cancelled;
        }
        if cx.deadline_exceeded() || cx.fuel_exhausted() { return CleanupTurn::Yield; }
        let cell_owner = match state.cell.try_lock() {
                Ok(cell) => cell,
                Err(std::sync::TryLockError::WouldBlock) => {
                    *contended = true;
                    return CleanupTurn::Yield;
                }
                Err(std::sync::TryLockError::Poisoned(_)) => {
                    return CleanupTurn::Fault(b"plugin close cell authority is poisoned");
                }
            };
        let Some(cell) = cell_owner.as_ref() else {
                *outcome_progress = Some(PluginLifecycleStep::Complete(Default::default()));
                return CleanupTurn::Complete;
            };
        {

            let mut maintenance = match cell.maintenance_pump.try_lock() {
                Ok(maintenance) => maintenance,
                Err(std::sync::TryLockError::WouldBlock) => {
                    *contended = true;
                    return CleanupTurn::Yield;
                }
                Err(std::sync::TryLockError::Poisoned(_)) => {
                    return CleanupTurn::Fault(b"plugin maintenance session authority is poisoned");
                }
            };
            runtime_close_phase(&state, 10);
            let grant=state.mounted_policy.close;
            let progress=match cx.with_original_native_weak_parent(cell,|cx|maintenance.close_step(cx)){Ok(step)=>match step.admit(grant,maintenance.terminal_is_empty()){Ok(step)=>step,Err(error)=>return CleanupTurn::OriginalFailure(semio_framework_job::OriginalTurnFailure::from(error))},Err(failure)=>return CleanupTurn::OriginalFailure(failure)};
            runtime_close_phase(&state, 11);
            match progress {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress }|semio_framework_job::InteractiveJobCloseStep::Complete { progress } => {
                    *outcome_progress = Some(PluginLifecycleStep::Progress(progress));
                    if progress!=RetainedCloneProgress::default(){return CleanupTurn::Yield;}
                }
                semio_framework_job::InteractiveJobCloseStep::Blocked => *contended = true,
            }
            if !maintenance.terminal_is_empty(){return CleanupTurn::Yield;}
        }
        let mut instance = match cell.background_instance() {
            Ok(instance) => instance,
            Err(RuntimeInstanceRefusal::Contended) => {
                *contended = true;
                return CleanupTurn::Yield;
            }
            Err(RuntimeInstanceRefusal::Poisoned) => {
                return CleanupTurn::Fault(b"plugin app close authority is poisoned");
            }
        };
        runtime_close_phase(&state, 12);
        #[cfg(test)]
        state.physical_close_calls.fetch_add(1, Ordering::SeqCst);
        let progress = (|| -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
            let identity = MountedOwnerIdentityV1 { instance_id: state.instance_id, operation: cx.operation(), generation: cx.generation() };
            let Some(mut turn) = MountedOwnerTurnV1::admit(state.instance_id, identity, MountedOwnerPhaseV1::Close, state.mounted_policy, cx)? else { return Ok(PluginLifecycleStep::AwaitingInput { reason: "mounted close retains its original scheduling authority" }); };
            let step = instance.app.mounted_owner_close_step(&mut turn)?;
            if step.progress().unwrap_or_default() != turn.progress() || matches!(step, PluginLifecycleStep::Complete(_)) && !instance.app.close_terminal_is_empty() { return Err(Fault::from("mounted close receipt or terminal witness disagrees").into()); }
            Ok(step)
        })();
        runtime_close_phase(&state, 13);
        let outcome = match progress {
            Ok(progress @ PluginLifecycleStep::Progress(receipt)) => {
                *outcome_progress = Some(progress);
                CleanupTurn::Checkpoint(receipt.copied_items as u64)
            }
            Ok(progress @ PluginLifecycleStep::Blocked { reason }) => {
                runtime_close_pending_authority(reason);
                *outcome_progress = Some(progress);
                CleanupTurn::Yield
            }
            Ok(progress @ PluginLifecycleStep::AwaitingInput { reason }) => {
                runtime_close_pending_authority(reason);
                *outcome_progress = Some(progress);
                CleanupTurn::Yield
            }
            Ok(progress @ PluginLifecycleStep::Complete(_)) => {
                if !instance.app.close_terminal_is_empty() {
                    return CleanupTurn::Fault(b"plugin close reported Complete without a terminal-empty app witness");
                }
                *outcome_progress = Some(progress);
                CleanupTurn::Complete
            }
            Err(error) => CleanupTurn::OriginalFailure(error.into()),
        };
        runtime_close_phase(&state, 14);
        outcome
    }
}
impl<PA: PluginApp> semio_framework_job::InteractiveJob for RuntimeCloseCleanupJob<PA> {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        crate::reserved_job_outcomes::drive_turn(self, cx)
    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.publication.borrow_outcome(descriptor)
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.closing = true;
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(cx);
        }
        if self.state.is_some(){return Ok(match cx.return_original_native_weak(&mut self.state)?{Some(progress)=>semio_framework_job::InteractiveJobCloseStep::Complete{progress},None=>semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()}})}
        Ok(semio_framework_job::InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.publication.close_demands()?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        let demand = self.publication.close_demands()?;
        Ok(if demand == RetirementDemand::default() { usize::from(self.state.is_some()) } else { demand.depth })
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.state.is_none() && self.publication.terminal_is_empty()
    }
}
fn runtime_close_pool() -> semio_framework_async::WorkerPool {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores))
}
/// 🩺️ The structural accountant's verdict on ONE close-ladder step, priced in BOTH currencies it
/// claims to spend: consecutive fruitless steps AND the real microseconds they burned.
///
/// `progress` is what the ladder READ this step. `None` means the step recorded no reading at all —
/// it was cancelled, or it never reached the ladder — which is an absence of evidence, not evidence
/// of a stall, so it buys no stall debt; a cancelled or faulted outcome is terminal through
/// `RuntimeCleanupFault::PriorOutcome` instead. Only an observed `Pending { 0, 0 }` — the ladder
/// answered, and answered that it released nothing — is a stalled step.
///
/// The fault needs both: `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT` consecutive stalled steps, and
/// `RUNTIME_CLOSE_STALL_CREDIT_US` of real elapsed time since the ladder last moved. `stall_since_us`
/// carries that instant (`0` = not stalling); the returned reading is the credit actually spent, so
/// the emitted message quotes a measured stall rather than the unrelated cost of its last turn.
fn runtime_close_stall_verdict(contended: bool, progress: Option<PluginLifecycleStep>, stalled_steps: &AtomicU8, stall_since_us: &AtomicU64, now_us: Option<u64>) -> (RuntimeCloseStatus, u64) {
    let clear = || {
        stalled_steps.swap(0, Ordering::SeqCst);
        stall_since_us.store(0, Ordering::SeqCst);
    };
    if contended {
        return (RuntimeCloseStatus::Ready, 0);
    }
    if matches!(progress, Some(PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. })) {
        clear();
        return (RuntimeCloseStatus::ExternalWait, 0);
    }
    if !matches!(progress,Some(PluginLifecycleStep::Progress(receipt)) if receipt==RetainedCloneProgress::default()) {
        clear();
        return (RuntimeCloseStatus::Ready, 0);
    }
    let stalled = stalled_steps.try_update(Ordering::SeqCst, Ordering::SeqCst, |seen| Some(seen.saturating_add(1))).unwrap_or(u8::MAX).saturating_add(1);
    let Some(now_us) = now_us else { return (RuntimeCloseStatus::Ready, 0) };
    let since_us = match stall_since_us.compare_exchange(0, now_us.max(1), Ordering::SeqCst, Ordering::SeqCst) {
        Ok(_) => now_us.max(1),
        Err(previous) => previous,
    };
    let credit_us = now_us.saturating_sub(since_us);
    if stalled >= RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT && credit_us >= RUNTIME_CLOSE_STALL_CREDIT_US {
        (RuntimeCloseStatus::Fault(RuntimeCleanupFault::ZeroProgress), credit_us)
    } else {
        (RuntimeCloseStatus::Ready, credit_us)
    }
}
fn run_runtime_close_turn_inner<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<RuntimeCloseStatus>,semio_framework_job::OriginalTurnFailure> {
    if state.status.compare_exchange(RuntimeCloseStatus::Queued.repr(), RuntimeCloseStatus::Running.repr(), Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Ok(None);
    }
    let retained = state.deadline_resume.load(Ordering::SeqCst);
    if retained != u8::MAX {
        return Ok(Some(RuntimeCloseStatus::from_repr(retained)));
    }
    #[cfg(test)]
    {
        state.callback_phase_started_us.store(semio_framework_job::default_now_us().unwrap_or(0), Ordering::SeqCst);
        for elapsed in &state.callback_phase_us {
            elapsed.store(0, Ordering::SeqCst);
        }
    }
    let status = match state.pump.try_lock() {
        Ok(mut pump) => {
            let status = cx.with_original_native_weak_parent(state,|cx|runtime_close_cleanup_pump_one(state,&mut pump,cx))?;
            runtime_close_phase(state, 20);
            drop(pump);
            runtime_close_phase(state, 21);
            status
        }
        Err(_) => RuntimeCloseStatus::Ready,
    };
    runtime_close_phase(state, 6);
    let candidate = if status == RuntimeCloseStatus::Complete { runtime_close_retire_cell(state) } else { status };
    runtime_close_phase(state, 7);
    Ok(Some(candidate))
}
fn runtime_close_retire_cell<PA: PluginApp>(state: &RuntimeCloseWorkerState<PA>) -> RuntimeCloseStatus {
    let mut owner = match state.cell.try_lock() {
        Ok(owner) => owner,
        Err(std::sync::TryLockError::WouldBlock) => return RuntimeCloseStatus::Ready,
        Err(std::sync::TryLockError::Poisoned(_)) => return runtime_close_fault(state, RuntimeCleanupFault::OwnerPoisoned),
    };
    let Some(cell) = owner.as_ref() else { return RuntimeCloseStatus::Complete };
    {
        let instance = match cell.background_instance() {
            Ok(instance) => instance,
            Err(RuntimeInstanceRefusal::Contended) => return RuntimeCloseStatus::Ready,
            Err(RuntimeInstanceRefusal::Poisoned) => return runtime_close_fault(state, RuntimeCleanupFault::InstancePoisoned),
        };
        if !instance.app.close_terminal_is_empty() {
            return runtime_close_fault(state, RuntimeCleanupFault::InstanceNotDrained);
        }
        let maintenance = match cell.maintenance_pump.try_lock() {
            Ok(maintenance) => maintenance,
            Err(std::sync::TryLockError::WouldBlock) => return RuntimeCloseStatus::Ready,
            Err(std::sync::TryLockError::Poisoned(_)) => return runtime_close_fault(state, RuntimeCleanupFault::MaintenancePoisoned),
        };
        if !maintenance.terminal_is_empty() {
            return runtime_close_fault(state, RuntimeCleanupFault::MaintenanceNotDrained);
        }
    }
    let detached = owner.take().expect("captured owner remains under the same guard");
    match std::sync::Arc::try_unwrap(detached) {
        Ok(cell) => {
            drop(cell);
            RuntimeCloseStatus::Complete
        }
        Err(detached) => {
            **owner = Some(detached);
            RuntimeCloseStatus::ExternalWait
        }
    }
}
/// ⏳️ A close authority that is not yet releasable is a WAIT, never a fault: the ladder keeps
/// stepping until its external owner lets go (mirrors the live cleanup job, which has always
/// yielded on `Blocked`). Reported on a doubling cadence so a genuinely stuck owner is visible
/// without flooding a close that clears after a few turns.
fn runtime_close_pending_authority(reason: &'static str) {
    let seen = RUNTIME_CLOSE_PENDING_AUTHORITY_TURNS.fetch_add(1, Ordering::Relaxed).saturating_add(1);
    if seen.is_power_of_two() {
        debug_runtime_line(format_args!("[TRACE] runtime close pending authority turn={seen} reason={reason}"));
    }
}
static RUNTIME_CLOSE_PENDING_AUTHORITY_TURNS: AtomicU64 = AtomicU64::new(0);
fn runtime_close_fault<PA: PluginApp>(state: &RuntimeCloseWorkerState<PA>, cause: RuntimeCleanupFault) -> RuntimeCloseStatus {
    #[cfg(test)]
    state.last_fault_origin.store(cause as u8, Ordering::SeqCst);
    #[cfg(not(test))]
    let _ = state;
    RuntimeCloseStatus::Fault(cause)
}
fn runtime_close_phase<PA: PluginApp>(state: &RuntimeCloseWorkerState<PA>, phase: usize) {
    #[cfg(test)]
    if let Some(now) = semio_framework_job::default_now_us() {
        state.callback_phase_us[phase].store(now.saturating_sub(state.callback_phase_started_us.load(Ordering::SeqCst)), Ordering::SeqCst);
    }
    #[cfg(not(test))]
    let _ = (state, phase);
}
fn runtime_close_cleanup_pump_one<PA: PluginApp + 'static>(state: &std::sync::Arc<RuntimeCloseWorkerState<PA>>, pump: &mut RuntimeCloseCleanupPump<PA>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<RuntimeCloseStatus,semio_framework_job::OriginalTurnFailure> {
    runtime_close_phase(state, 0);
    let grant=cx.retained_grant();
    if let Some(rejected)=pump.rejected.as_mut().filter(|pending|!pending.admission_pending()||cx.is_cancelled()){
        if rejected.terminal_is_empty(){if grant.maximum_copy_bytes<size_of_val(rejected){return Ok(RuntimeCloseStatus::Ready);}drop(pump.rejected.take());return Ok(runtime_close_fault(state,RuntimeCleanupFault::AdmissionRejected));}
        rejected.close_step(cx)?;return Ok(RuntimeCloseStatus::Ready);
    }
    if pump.outcome_pending{
        let Some(session)=pump.session.as_mut()else{return Ok(runtime_close_fault(state,RuntimeCleanupFault::MissingSession));};
        let acknowledgement=session.acknowledge_outcome(grant);cx.consume_retained(acknowledgement.progress())?;if !matches!(acknowledgement,RetainedCloneStep::Complete(_)){return Ok(RuntimeCloseStatus::Ready);}
        pump.outcome_pending=false;
        let Some(job) = session.checked_out_job_mut() else { return Ok(runtime_close_fault(state, RuntimeCleanupFault::CheckedOutJob)) };
        pump.complete = matches!(job.progress,Some(PluginLifecycleStep::Complete(_)));
        pump.blocked = matches!(job.progress, Some(PluginLifecycleStep::Blocked { .. }));
        let (status, credit_us) = runtime_close_stall_verdict(job.contended, job.progress, &state.stalled_steps, &state.stall_since_us, semio_framework_job::default_now_us());
        pump.pending_status = status;
        if let Some(cause) = pump.pending_status.cause() {
            state.stall_credit_spent_us.store(credit_us, Ordering::SeqCst);
            runtime_close_fault(state, cause);
        }
        pump.resume_pending=true;
        return Ok(RuntimeCloseStatus::Ready);
    }
    if pump.resume_pending{
        pump.resume_pending=false;
        if pump.terminal{if let Some(session)=pump.session.as_mut(){session.begin_close();}return Ok(RuntimeCloseStatus::Ready);}
        return Ok(if pump.session.as_mut().is_some_and(|session|session.resume().is_err()){runtime_close_fault(state,RuntimeCleanupFault::Resume)}else{pump.pending_status});
    }
    if pump.terminal{
        let Some(session)=pump.session.as_mut()else{return Ok(runtime_close_fault(state,RuntimeCleanupFault::MissingSession));};
        if !session.terminal_is_empty(){session.close_step(cx)?;return Ok(RuntimeCloseStatus::Ready);}
        if grant.maximum_copy_bytes<size_of_val(session){return Ok(RuntimeCloseStatus::Ready);}
        drop(pump.session.take());pump.terminal=false;
        return Ok(if pump.faulted{runtime_close_fault(state,RuntimeCleanupFault::PriorOutcome)}else if pump.blocked{RuntimeCloseStatus::ExternalWait}else if pump.complete{RuntimeCloseStatus::Complete}else{pump.pending_status});
    }
    if pump.session.is_none()&&pump.rejected.is_none(){let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RuntimeCloseStatus::Ready)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);pump.rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(RuntimeCloseCleanupJob { state: Some(std::sync::Arc::downgrade(state)), instance_id: state.instance_id, progress: None, contended: false, publication: crate::reserved_job_outcomes::CleanupPublication::new(), closing: false },(cx.operation(),cx.generation())));}
    if let Some(pending)=pump.rejected.as_mut(){
        let config=semio_framework_job::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_runtime_close_cleanup",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:RUNTIME_CLOSE_INNER_GRANT_US};
        match pending.admit_pending_batch(config,cx)?{Some(session)=>{pump.session=Some(session);drop(pump.rejected.take());},None=>return Ok(RuntimeCloseStatus::Ready)}
        return Ok(RuntimeCloseStatus::Ready);
    }
    let Some(session) = pump.session.as_mut() else { return Ok(runtime_close_fault(state, RuntimeCleanupFault::MissingSession)) };
    runtime_close_phase(state, 22);
    let stepped = session.step_on_original_caller(cx);
    runtime_close_phase(state, 23);
    match stepped {
        Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) => {}
        Ok(_) => return Ok(RuntimeCloseStatus::Ready),
        Err(failure) => return Err(failure),
    }
    runtime_close_phase(state, 3);
    if !session.checkout_outcome() {
        return Ok(runtime_close_fault(state, RuntimeCleanupFault::Checkout));
    }
    if !session.take_checked_out_retained_step_receipt().is_some_and(|(issued, progress)| progress.fits(issued)) { return Ok(runtime_close_fault(state, RuntimeCleanupFault::TakeOutcome)); }
    let (terminal, faulted) = match session.checked_out_outcome() {
        Ok(Some(view)) => {
            #[cfg(test)]
            if let semio_framework_job::JobOutcomeView::Fault { detail, .. } = &view {
                if let Some(bytes) = detail.single_page() {
                    let mut last_fault = state.last_fault.lock().unwrap();
                    let count = bytes.len().min(last_fault.len());
                    last_fault[..count].copy_from_slice(&bytes[..count]);
                }
            }
            (view.is_terminal(), matches!(view, semio_framework_job::JobOutcomeView::Fault { .. } | semio_framework_job::JobOutcomeView::Cancelled { .. }))
        }
        Ok(None) => (false, false),
        Err(_) => return Ok(runtime_close_fault(state, RuntimeCleanupFault::TakeOutcome)),
    };
    runtime_close_phase(state, 4);
    pump.terminal = terminal;
    pump.faulted = faulted;
    pump.outcome_pending = true;
    Ok(RuntimeCloseStatus::Ready)
}
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs");
/// 🩺️ Language-neutral decode conformance: every declared cleanup-fault variant must round-trip
/// through its status atom and produce exactly the fixture's wire code and message shape, so the
/// React shell's classifier and this module can never drift apart silently.
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-runtime-cleanup-fault-vector/🦀️.rs");
fn run_runtime_close_turn<PA:PluginApp+'static>(state:&std::sync::Arc<RuntimeCloseWorkerState<PA>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
    if cx.should_yield(){return Ok(())}
    state.status.store(RuntimeCloseStatus::Queued.repr(),Ordering::SeqCst);
    let mut clock=cx.clock_source();let measured=runtime_callback_clock_begin(&mut clock);
    let Some(status)=run_runtime_close_turn_inner(state,cx)?else{return Ok(())};
    match measured.and_then(|start|runtime_callback_clock_elapsed(&mut clock,start)){Ok(elapsed)=>runtime_close_publish_turn(state,status,elapsed),Err(cause)=>{state.last_callback_elapsed_us.store(u64::MAX,Ordering::SeqCst);state.status.store(runtime_close_fault(state,cause).repr(),Ordering::SeqCst);}}Ok(())
}
fn runtime_callback_clock_begin(now_us: &mut impl FnMut() -> Option<u64>) -> Result<(u64, u64), RuntimeCleanupFault> {
    let start = now_us().ok_or(RuntimeCleanupFault::Clock)?;
    let preflight = now_us().ok_or(RuntimeCleanupFault::Clock)?;
    if preflight < start {
        return Err(RuntimeCleanupFault::ClockRegression);
    }
    Ok((start, preflight))
}
fn runtime_callback_clock_elapsed(now_us: &mut impl FnMut() -> Option<u64>, (start, preflight): (u64, u64)) -> Result<u64, RuntimeCleanupFault> {
    let finished = now_us().ok_or(RuntimeCleanupFault::Clock)?;
    if finished < preflight {
        return Err(RuntimeCleanupFault::ClockRegression);
    }
    Ok(finished - start)
}

fn runtime_close_publish_turn<PA: PluginApp>(state: &RuntimeCloseWorkerState<PA>, status: RuntimeCloseStatus, elapsed_us: u64) {
    state.last_callback_elapsed_us.store(elapsed_us, Ordering::SeqCst);
    let verdict = if !matches!(status, RuntimeCloseStatus::Fault(_)) && semio_framework_trace::interactive_step_contract_violated(elapsed_us) {
        #[cfg(test)]
        debug_runtime_line(format_args!("[TRACE] native close deadline instance={} generation={} candidate={status:?} elapsed_us={elapsed_us}", state.instance_id, state.generation.0));
        state.deadline_resume.store(status.repr(), Ordering::SeqCst);
        let _ = state.deadline_elapsed_us.compare_exchange(0, elapsed_us, Ordering::SeqCst, Ordering::SeqCst);
        RuntimeCloseStatus::DeadlineYield
    } else {
        state.deadline_resume.store(u8::MAX, Ordering::SeqCst);
        status
    };
    state.status.store(verdict.repr(), Ordering::SeqCst);
}

fn checked_runtime_close_generation(current: u64) -> Result<u64, Fault> {
    current.checked_add(1).ok_or_else(|| plugin_internal_fault("runtime close generation exhausted"))
}
pub async fn plugin_destroy_app<PA: PluginApp + 'static>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<(), Fault> {
    plugin_begin_instance_close(runtime, instance_id, None).map(|_| ())
}
#[cfg(test)]
std::thread_local! {
    static RUNTIME_CLOSE_CONSTRUCTION_LIVE: Cell<Option<bool>> = const { Cell::new(None) };
    static RUNTIME_CLOSE_CONSTRUCTION_FAIL: Cell<bool> = const { Cell::new(false) };
}
fn plugin_begin_instance_close<PA: PluginApp + 'static>(runtime: &PluginRuntime<PA>, instance_id: u32, expected: Option<&std::sync::Weak<RuntimeAppCell<PA>>>) -> Result<std::sync::Arc<RuntimeCloseWorkerState<PA>>, Fault> {
    let mut instances = runtime.instances.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?;
    let mut quarantine = runtime.close_quarantine.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime close quarantine is busy"))?;
    let mut actors = runtime.instance_actors.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime actor authority is busy"))?;
    let mut document_backbones = runtime.document_backbones.try_borrow_mut().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
    if let Some(expected) = expected {
        let current = instances.get(instance_id).ok_or_else(|| plugin_internal_fault("captured app lifetime is no longer live"))?;
        if !std::sync::Weak::ptr_eq(expected, &std::sync::Arc::downgrade(current)) {
            return Err(plugin_internal_fault("captured app lifetime does not match the current instance"));
        }
    }
    if !quarantine.can_insert(instance_id) {
        return Err(plugin_internal_fault(format!("runtime close quarantine is saturated or collided: {instance_id}")));
    }
    let generation = checked_runtime_close_generation(runtime.close_generation.get())?;
    #[cfg(test)]
    let original_allocation = instances.get(instance_id).map(std::sync::Arc::as_ptr);
    let cell = instances.get(instance_id).cloned().ok_or_else(|| plugin_internal_fault(format!("unknown instance: {instance_id}")))?;
    #[cfg(test)]
    RUNTIME_CLOSE_CONSTRUCTION_FAIL.with(|fail| {
        if fail.replace(false) {
            panic!("injected before close worker allocation");
        }
    });
    let state = std::sync::Arc::new(RuntimeCloseWorkerState {
        instance_id,
        generation: semio_framework_job::Generation(generation),
        mounted_policy: cell.mounted_policy,
        cell: std::sync::Mutex::new(std::mem::ManuallyDrop::new(Some(cell))),
        pump: std::sync::Mutex::new(RuntimeCloseCleanupPump::new()),
        status: AtomicU8::new(RuntimeCloseStatus::Ready.repr()),
        deadline_resume: AtomicU8::new(u8::MAX),
        deadline_elapsed_us: AtomicU64::new(0),
        stalled_steps: AtomicU8::new(0),
        stall_since_us: AtomicU64::new(0),
        stall_credit_spent_us: AtomicU64::new(0),
        last_callback_elapsed_us: AtomicU64::new(0),
        #[cfg(test)]
        last_fault: std::sync::Mutex::new([0; 256]),
        #[cfg(test)]
        last_fault_origin: AtomicU8::new(0),
        #[cfg(test)]
        physical_close_calls: AtomicU64::new(0),
        #[cfg(test)]
        callback_phase_started_us: AtomicU64::new(0),
        #[cfg(test)]
        callback_phase_us: std::array::from_fn(|_| AtomicU64::new(0)),
    });
    #[cfg(test)]
    RUNTIME_CLOSE_CONSTRUCTION_LIVE.with(|probe| probe.set(Some(original_allocation.is_some() && instances.get(instance_id).map(std::sync::Arc::as_ptr) == original_allocation)));
    quarantine.insert_admitted(instance_id, RuntimeCloseEntry { state: state.clone() });
    drop(instances.take(instance_id));
    runtime.close_generation.set(generation);
    let _ = actors.take(instance_id);
    if let Some(binding) = document_backbones.take(instance_id) {
        if let Some(owner) = binding.channel {
            owner.begin_retire().map_err(|error| plugin_internal_fault(error.to_string()))?;
        }
    }
    drop(document_backbones);
    drop(actors);
    drop(quarantine);
    drop(instances);
    Ok(state)
}
/// 🚪️ A faulted close is TERMINAL for this instance: the entry leaves quarantine with the
/// verdict, so the same dead lifetime cannot re-raise it on every later cursor pass (which,
/// now that the reactor turn survives a close fault, would be an unbounded fault storm).
/// `ZeroProgress` quotes the stall credit it actually spent; every other cause quotes the
/// turn cost it was measured against.
pub fn plugin_step_close_cleanup<PA: PluginApp + 'static>(runtime:&PluginRuntime<PA>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let quarantine=runtime.close_quarantine.try_borrow().map_err(|_|plugin_internal_fault("runtime close quarantine is busy"))?;
    let Some((index,instance_id,entry))=quarantine.next_entry_from(runtime.close_cleanup_cursor.get())else{return Ok(false)};
    let state=&entry.state;runtime.close_cleanup_cursor.set((index+1)%PLUGIN_RUNTIME_INSTANCE_SLOTS);
    match RuntimeCloseStatus::from_repr(state.status.load(Ordering::SeqCst)){
        RuntimeCloseStatus::Complete=>{drop(quarantine);let removed=runtime.close_quarantine.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime close quarantine is busy"))?.take(instance_id);drop(removed);Ok(true)}
        RuntimeCloseStatus::Fault(cause)=>{let elapsed_us=if cause==RuntimeCleanupFault::ZeroProgress{state.stall_credit_spent_us.load(Ordering::SeqCst)}else{state.last_callback_elapsed_us.load(Ordering::SeqCst)};drop(quarantine);let removed=runtime.close_quarantine.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime close quarantine is busy"))?.take(instance_id);drop(removed);Err(runtime_cleanup_fault("close",cause,instance_id,elapsed_us).into())}
        _=>{if cx.should_yield(){return Ok(false)}run_runtime_close_turn(state,cx)?;Ok(true)}
    }
}

#[cfg(target_arch = "wasm32")]
fn pump_runtime_live_cooperative_turn<PA: PluginApp>(cell: &RuntimeAppCell<PA>) -> Result<(), Fault> {
    let before = cell.maintenance_status.load(Ordering::SeqCst);
    let turn = cell.maintenance_probe_turns.load(Ordering::Relaxed).saturating_add(1);
    cell.maintenance_probe_turns.store(turn, Ordering::Relaxed);
    let pool = runtime_close_pool();
    let now_ms = semio_framework_job::default_now_ms();
    if let Some(now_ms) = now_ms {
        pool.pump(now_ms);
    } else {
        cell.maintenance_fault_us.store(RUNTIME_CLEANUP_UNMEASURED_US, Ordering::SeqCst);
        cell.maintenance_status.store(RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::CooperativeClock).repr(), Ordering::SeqCst);
    }
    if turn <= 4096 && turn.is_power_of_two() && semio_framework_trace::runtime_diagnostics_enabled() {
        let phase = cell
            .maintenance_pump
            .try_lock()
            .ok()
            .map(|pump| u8::from(pump.session.is_some()) | (u8::from(pump.outcome_pending) << 1) | (u8::from(pump.rejected.is_some()) << 2) | (u8::from(pump.closing) << 3) | (u8::from(pump.faulted) << 4) | (u8::from(pump.terminal) << 5));
        eprintln!(
            "[TRACE] cooperative-maintenance instance={} turn={} generation={} status={}->{} entries={} phase={:?} clock={} pool={:?}",
            cell.id,
            turn,
            cell.maintenance_generation.load(Ordering::SeqCst),
            before,
            cell.maintenance_status.load(Ordering::SeqCst),
            cell.maintenance_probe_entries.load(Ordering::Relaxed),
            phase,
            now_ms.is_some(),
            pool.try_cooperative_snapshot()
        );
    }
    now_ms.map(|_| ()).ok_or_else(|| runtime_cleanup_fault("live", RuntimeCleanupFault::CooperativeClock, cell.id, RUNTIME_CLEANUP_UNMEASURED_US))
}
/// 🌡️ Whether the instance the last [`plugin_step_live_cleanup`] stepped still reports
/// maintenance pressure — the reactor turn keeps stepping while this holds, bounded by
/// [`LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN`] and its retirement deadline.
pub fn plugin_live_cleanup_under_pressure<PA: PluginApp + 'static>(runtime: &PluginRuntime<PA>) -> bool {
    runtime.live_cleanup_pressure.get()
}
/// 🌡️ Maintenance steps one reactor turn may spend on pressured instances. A live commit cadence
/// displaces three or more owners per commit and at most one commit lands per turn, so this
/// burst drains faster than any cadence fills while the fair single step stays the idle cost.
pub const LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN: usize = 32;
pub fn plugin_step_live_cleanup<PA: PluginApp + 'static>(runtime:&PluginRuntime<PA>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let instances=runtime.instances.try_borrow().map_err(|_|plugin_internal_fault("runtime instance authority is busy"))?;
    let Some((index,_,cell))=instances.next_entry_from(runtime.live_cleanup_cursor.get())else{runtime.live_cleanup_pressure.set(false);return Ok(false)};
    runtime.live_cleanup_cursor.set((index+1)%PLUGIN_RUNTIME_INSTANCE_SLOTS);let stepped=plugin_step_live_cleanup_cell(cell,cx);runtime.live_cleanup_pressure.set(cell.maintenance_pressure.load(Ordering::Relaxed));stepped
}

fn plugin_step_live_cleanup_cell<PA:PluginApp+'static>(cell:&std::sync::Arc<RuntimeAppCell<PA>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    if cx.should_yield(){return Ok(false)}
    if let RuntimeMaintenanceStatus::Fault(cause)=RuntimeMaintenanceStatus::from_repr(cell.maintenance_status.load(Ordering::SeqCst)){return Err(runtime_cleanup_fault("live",cause,cell.id,cell.maintenance_fault_us.load(Ordering::SeqCst)).into())}
    let generation=semio_framework_job::Generation(cell.maintenance_generation.load(Ordering::SeqCst));
    run_runtime_live_cleanup_turn(cell,generation,cx)?;Ok(true)
}
// 🚫️async: E1-adjacent pure lookup consumed by `with_instances_mut`'s sync callback
// (`FnOnce(&mut Vec<AppInstance>) -> Result<R, Fault>`, itself required sync by the
// single-threaded `RefCell::borrow_mut` reentrancy guard) — see R9.
fn find_instance<'a, PA: PluginApp>(list: &'a RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>, instance_id: u32) -> Result<HostInstanceGuard<'a, PA>, Fault> {
    let cell = list.get(instance_id).ok_or_else(|| plugin_internal_fault(format!("unknown instance: {instance_id}")))?;
    cell.host_instance().ok_or_else(|| plugin_internal_fault(format!("instance busy or poisoned: {instance_id}")))
}
fn runtime_instance_cell<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<std::sync::Arc<RuntimeAppCell<PA>>, Fault> {
    runtime.instances.try_borrow().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?.get(instance_id).cloned().ok_or_else(|| plugin_internal_fault(format!("unknown instance: {instance_id}")))
}
/// 📏️ The ONE public invocation envelope, read off the language-neutral contract
/// (`🛂️manifest/🎛️public-invocation/🧬️schema/🔣️.json`). `MAX_PUBLIC_ACTION_STRING_BYTES` is
/// checked BEFORE the addressed tool's own `max_raw_wire_bytes`, so no tool contract can widen
/// it. `setContributions` is the host pack crossing: it keeps the body cap and skips the per-string
/// cap so one scoped contributions pack can land without 4 KiB paging
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const MAX_PUBLIC_ACTION_BODY_BYTES: usize = semio_framework::PUBLIC_INVOCATION_BODY_BYTES;
const MAX_PUBLIC_ACTION_STRING_BYTES: usize = semio_framework::PUBLIC_INVOCATION_STRING_BYTES;
const MAX_PUBLIC_ACTION_DEPTH: usize = semio_framework::PUBLIC_INVOCATION_DEPTH;
const HOST_CONTRIBUTIONS_PACK_COMMAND_ID: &str = "setContributions";
fn dff_interactive_wire_limit(owner: crate::tool_contract::ToolOwnerWitness, runtime_controller_id: &str, addressed_controller_id: &str, command_id: &str, app_contracts: &[crate::tool_contract::ArtifactToolPublicContract]) -> Result<Option<usize>, Fault> {
    let registration = app_contracts.iter().find(|registration| registration.controller_id == addressed_controller_id && registration.tool_id == command_id);
    match registration {
        Some(registration) if runtime_controller_id == addressed_controller_id && registration.owner == owner && !registration.schema_id.is_empty() => Ok(Some(registration.max_raw_wire_bytes)),
        Some(_) => Err(plugin_internal_fault("addressed app-owned tool is not owned by the live app type/controller")),
        None => Ok(None),
    }
}
fn dff_invocation_wire_limit(body: &str, identifier_key: &str, label: &str, owner: crate::tool_contract::ToolOwnerWitness, runtime_controller_id: &str, app_contracts: &[crate::tool_contract::ArtifactToolPublicContract]) -> Result<Option<usize>, Fault> {
    let bytes = body.as_bytes();
    let mut cursor = 0usize;
    let mut depth = 0usize;
    let mut address_depth = None;
    let mut pending_address_start = None;
    let mut identifier_seen = false;
    let mut controller_seen = false;
    let mut selected_identifier = None;
    let mut selected_controller = None;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'{' => {
                depth = depth.saturating_add(1);
                if pending_address_start == Some(cursor) {
                    address_depth = Some(depth);
                    pending_address_start = None;
                }
                cursor += 1;
            }
            b'}' => {
                if address_depth == Some(depth) {
                    address_depth = None;
                }
                depth = depth.saturating_sub(1);
                cursor += 1;
            }
            b'"' => {
                let key_start = cursor + 1;
                cursor = key_start;
                let mut escaped = false;
                while cursor < bytes.len() && bytes[cursor] != b'"' {
                    if bytes[cursor] == b'\\' {
                        escaped = true;
                        cursor = cursor.saturating_add(1);
                    }
                    cursor = cursor.saturating_add(1);
                }
                if cursor >= bytes.len() {
                    return Err(plugin_internal_fault(format!("{label} contains an unterminated string")));
                }
                let key_end = cursor;
                cursor += 1;
                let mut value_cursor = cursor;
                while value_cursor < bytes.len() && bytes[value_cursor].is_ascii_whitespace() {
                    value_cursor += 1;
                }
                if bytes.get(value_cursor) != Some(&b':') {
                    continue;
                }
                if escaped && (depth == 1 || address_depth.is_some_and(|start| depth >= start)) {
                    return Err(plugin_internal_fault(format!("{label} address keys must be plain bounded identifiers")));
                }
                value_cursor += 1;
                while value_cursor < bytes.len() && bytes[value_cursor].is_ascii_whitespace() {
                    value_cursor += 1;
                }
                let key = &body[key_start..key_end];
                if depth == 1 && key == "address" {
                    if bytes.get(value_cursor) != Some(&b'{') {
                        return Err(plugin_internal_fault(format!("{label} address must be an object")));
                    }
                    pending_address_start = Some(value_cursor);
                } else if address_depth == Some(depth) && key == identifier_key {
                    if identifier_seen {
                        return Err(plugin_internal_fault(format!("{label} address contains a duplicate {identifier_key}")));
                    }
                    identifier_seen = true;
                    if bytes.get(value_cursor) != Some(&b'"') {
                        return Err(plugin_internal_fault(format!("{identifier_key} must be a plain bounded string")));
                    }
                    let value_start = value_cursor + 1;
                    value_cursor = value_start;
                    let mut value_escaped = false;
                    while value_cursor < bytes.len() && bytes[value_cursor] != b'"' {
                        if bytes[value_cursor] == b'\\' {
                            value_escaped = true;
                            value_cursor = value_cursor.saturating_add(1);
                        }
                        value_cursor = value_cursor.saturating_add(1);
                    }
                    if value_cursor >= bytes.len() || value_escaped {
                        return Err(plugin_internal_fault(format!("{identifier_key} must be an unescaped bounded identifier")));
                    }
                    selected_identifier = Some(&body[value_start..value_cursor]);
                } else if address_depth.is_some_and(|start| depth >= start) && key == "appId" {
                    if controller_seen {
                        return Err(plugin_internal_fault(format!("{label} address contains a duplicate appId")));
                    }
                    controller_seen = true;
                    if bytes.get(value_cursor) != Some(&b'"') {
                        return Err(plugin_internal_fault("appId must be a plain bounded string"));
                    }
                    let value_start = value_cursor + 1;
                    value_cursor = value_start;
                    let mut value_escaped = false;
                    while value_cursor < bytes.len() && bytes[value_cursor] != b'"' {
                        if bytes[value_cursor] == b'\\' {
                            value_escaped = true;
                            value_cursor = value_cursor.saturating_add(1);
                        }
                        value_cursor = value_cursor.saturating_add(1);
                    }
                    if value_cursor >= bytes.len() || value_escaped {
                        return Err(plugin_internal_fault("appId must be an unescaped bounded identifier"));
                    }
                    selected_controller = Some(&body[value_start..value_cursor]);
                }
            }
            _ => cursor += 1,
        }
    }
    match selected_controller.zip(selected_identifier) {
        Some((addressed_controller_id, command_id)) => dff_interactive_wire_limit(owner, runtime_controller_id, addressed_controller_id, command_id, app_contracts),
        None => Ok(None),
    }
}
fn public_address_command_id(body: &str) -> Option<String> {
    serde_json::from_str::<Value>(body).ok()?.get("address")?.get("commandId")?.as_str().map(str::to_owned)
}
fn validate_public_json_envelope(body: &str, label: &str) -> Result<(), Fault> {
    validate_public_json_envelope_with(body, label, true)
}
fn validate_public_command_json(body: &str) -> Result<(), Fault> {
    let cap_strings = public_address_command_id(body).as_deref() != Some(HOST_CONTRIBUTIONS_PACK_COMMAND_ID);
    validate_public_json_envelope_with(body, "command", cap_strings)
}
fn validate_public_json_envelope_with(body: &str, label: &str, cap_strings: bool) -> Result<(), Fault> {
    if body.len() > MAX_PUBLIC_ACTION_BODY_BYTES {
        return Err(plugin_internal_fault(format!("{label} exceeds the bounded public action body")));
    }
    let mut depth = 0usize;
    let mut string_bytes = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for byte in body.bytes() {
        if in_string {
            if escaped {
                escaped = false;
                string_bytes = string_bytes.saturating_add(1);
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            } else {
                string_bytes = string_bytes.saturating_add(1);
            }
            if cap_strings && string_bytes > MAX_PUBLIC_ACTION_STRING_BYTES {
                return Err(plugin_internal_fault(format!("{label} contains an oversized string")));
            }
            continue;
        }
        match byte {
            b'"' => {
                in_string = true;
                string_bytes = 0;
            }
            b'{' | b'[' => {
                depth = depth.saturating_add(1);
                if depth > MAX_PUBLIC_ACTION_DEPTH {
                    return Err(plugin_internal_fault(format!("{label} exceeds the bounded nesting depth")));
                }
            }
            b'}' | b']' if depth == 0 => return Err(plugin_internal_fault(format!("{label} closes outside its envelope"))),
            b'}' | b']' => depth -= 1,
            _ => {}
        }
    }
    if in_string || escaped || depth != 0 {
        return Err(plugin_internal_fault(format!("{label} is structurally incomplete")));
    }
    Ok(())
}
fn validate_public_action_envelope(body: &str, owner: crate::tool_contract::ToolOwnerWitness, runtime_controller_id: &str, app_contracts: &[crate::tool_contract::ArtifactToolPublicContract]) -> Result<(), Fault> {
    validate_public_json_envelope(body, "action")?;
    if let Some(limit) = dff_invocation_wire_limit(body, "actionId", "action", owner, runtime_controller_id, app_contracts)? {
        if body.len() > limit {
            return Err(plugin_internal_fault("interactive Draw/Flow/Forms action exceeds its command-specific wire bound"));
        }
    }
    Ok(())
}
fn validate_public_command_envelope(body: &str, owner: crate::tool_contract::ToolOwnerWitness, runtime_controller_id: &str, app_contracts: &[crate::tool_contract::ArtifactToolPublicContract]) -> Result<(), Fault> {
    validate_public_command_json(body)?;
    if let Some(limit) = dff_invocation_wire_limit(body, "commandId", "command", owner, runtime_controller_id, app_contracts)? {
        if body.len() > limit {
            return Err(plugin_internal_fault("interactive Draw/Flow/Forms command exceeds its command-specific wire bound"));
        }
    }
    Ok(())
}
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-dff-public-action-admission/🦀️.rs");
fn addressed_action_view(view: &ViewModel, address: &semio_framework::manifest::ActionAddress) -> Result<ViewModel, Fault> {
    let projected = view.for_window_instance(&address.window_instance_id).ok_or_else(|| plugin_internal_fault(format!("unknown action window instance {}", address.window_instance_id)))?;
    if projected.active_window_kind_id.as_deref() != Some(address.window_kind_id.as_str()) {
        return Err(plugin_internal_fault(format!("action window instance {} has a different kind than {}", address.window_instance_id, address.window_kind_id)));
    }
    Ok(projected)
}
fn debug_action_dispatch_branch(action_id: &str) -> &'static str {
    if !crate::reserved_jobs::is_framework_reserved_action_id(action_id) {
        "catalog"
    } else if matches!(action_id, "copy" | "cut" | "paste") {
        "clipboard-instack"
    } else if crate::tool_jobs::is_tool_run_action_id(action_id) || crate::app_contract::is_time_travel_action_id(action_id) || action_id == crate::app_views::CANCEL_TYPED_OPERATION_ACTION_ID || action_id == semio_framework::HOST_EVENT_ACTION_ID {
        "host-driven"
    } else {
        "spawn-admit"
    }
}
/// 🎯️ Window projection for a catalog action, or the unprojected view for a framework-reserved verb.
/// Retained actor ingress (`AppCommand::Command`) uses this rule so undo/redo
/// reach `dispatch_framework_reserved_action` when the chrome addresses a window that does not
/// declare the verb (or is absent from `window_instances`).
pub(crate) fn admit_addressed_action_view(view: &ViewModel, address: &semio_framework::manifest::ActionAddress) -> Result<ViewModel, Fault> {
    match addressed_action_view(view, address) {
        Ok(projected) => Ok(projected),
        Err(_) if crate::reserved_jobs::is_framework_reserved_action_id(&address.action_id) => Ok(view.clone()),
        Err(fault) => Err(fault),
    }
}
/// 📥️ Ingests binary-encoded remote `MutationEnvelope`s (`protocol::decode_envelopes`) into
/// the instance's document store (idempotent — duplicate mutation ids are dropped by the causal
/// DAG / edit-id dedupe), returning one `protocol::MergeReport` per envelope for the caller
/// (`ApplyEnvelopes`'s handler) to frame as unsolicited `AppFrame::MergeReport` frames.
pub async fn plugin_ingest_operations<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.ingest_operations(mutations, identity, cx))
    })
    .await
}
/// 📜️ Text-DSL counterpart of {@link plugin_ingest_operations}: one already-authored operation
/// per non-blank op-text line instead of a binary `MutationEnvelope` array.
pub async fn plugin_ingest_operations_text<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operations_text: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.ingest_operations_text(operations_text, identity))
    })
    .await
}
/// 📜️ Text-DSL counterpart of {@link plugin_document_pack}.
pub async fn plugin_document_text<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<store::os_store::text_format::ArtifactTextFiles, Fault> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.document_text())
    })
    .await
}
/// 🧬️ Resolves the `codec` answers of the ONE app of the installed bundle that owns `artifact_schema`. This is
/// the guest half of `world actor`'s `codec` interface: a host that links no Rust codec for this package selects
/// a document kind by the same `schema` string `store::os_store::serialization::ArtifactCodec` is keyed by, and the bundle answers from its
/// own registered apps. An editor is preferred over a viewer because only an editor's snapshot is the kind's
/// creation authority; an ambiguous schema is refused rather than resolved by order. Nothing is constructed: the
/// answers are the functions of the owner's type its registration recorded ([`crate::app::ArtifactCodecTableV1`]).
fn plugin_artifact_codec<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str) -> Result<crate::surface_contract::ArtifactCodecTableV1, Fault> {
    if artifact_schema.is_empty() || artifact_schema.len() > 256 || artifact_schema.chars().any(char::is_control) {
        return Err(plugin_internal_fault("artifact codec schema identity is empty, oversized or control-bearing"));
    }
    if let Some(fault) = runtime.plugin_assembly_error.try_borrow().map_err(|_| plugin_internal_fault("plugin assembly authority busy"))?.clone() {
        return Err(fault);
    }
    let program = runtime.plugin.try_borrow().map_err(|_| plugin_internal_fault("plugin factory authority busy"))?;
    let program = program.as_ref().ok_or_else(|| plugin_internal_fault("plugin not initialized"))?;
    let definition = artifact_codec_owner(program, artifact_schema)?;
    program.app_codec(&definition.id).ok_or_else(|| plugin_internal_fault("artifact codec owner has no registered codec"))
}
/// 🪪️ The apps of `program` that own `artifact_schema`: those whose registered type opens that document
/// schema — the primary key `store::os_store::serialization::ArtifactCodec` and the hub's trusted catalog use — and those whose
/// dialect names it as an ARTIFACT KIND, the identity build tooling with nothing but a compiled
/// descriptor asks by (it reads the schema back out of `codec.genesis` and asks by schema after that; a
/// kind `s.<plugin>.<artifact>` and a `DOCUMENT_SCHEMA` never collide). Pure: nothing is constructed.
pub(crate) fn artifact_codec_candidates<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> impl Iterator<Item = &'a semio_framework::AppDefinition> + 'a {
    program.manifest.apps.iter().filter(move |definition| program.app_document_schema(&definition.id) == Some(artifact_schema) || definition.dialect.artifact_kind == artifact_schema)
}
/// 🎯️ The ONE app whose codec answers `artifact_schema`: the owning editor — only an editor's snapshot is the
/// kind's creation authority — else the owning viewer. Two owners of the same role are refused rather than
/// resolved by order, and no owner is refused; all of it decided on the declarations.
pub(crate) fn artifact_codec_owner<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> Result<&'a semio_framework::AppDefinition, Fault> {
    let (mut editor, mut viewer) = (None, None);
    for definition in artifact_codec_candidates(program, artifact_schema) {
        let slot = if definition.role == semio_framework::AppRole::Editor { &mut editor } else { &mut viewer };
        if slot.replace(definition).is_some() {
            return Err(plugin_internal_fault("artifact codec schema resolves more than one app of the same role"));
        }
    }
    editor.or(viewer).ok_or_else(|| plugin_internal_fault("artifact codec schema is owned by no app of this bundle"))
}
/// 🧬️ `codec.pack-schema-hash` — the kind's own 32-byte snapshot-record fingerprint.
pub async fn plugin_artifact_pack_schema_hash<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str) -> Result<[u8; 32], Fault> {
    let codec = plugin_artifact_codec(runtime, artifact_schema)?;
    (codec.pack_schema_hash)().ok_or_else(|| plugin_internal_fault("artifact codec schema has no structural record specification"))
}
/// 🏛️ `codec.sqlite-schema` names a registered dialect's handwritten relational declaration.
pub fn plugin_snapshot_sqlite_schema(dialect: &str) -> Result<String, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    let dialect = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(dialect).map_err(|error| plugin_internal_fault(&error))?;
    semio_framework_os_kernel::io::io_mechanism::native_snapshot_sqlite_schema(&dialect).map_err(|error| plugin_internal_fault(&error.to_string()))
}
/// 📤️ `codec.sqlite-export` executes the exact declared native snapshot provider.
pub async fn plugin_snapshot_sqlite_export(owner: &mut crate::sqlite_wire::SnapshotInputOwner, native: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>, snapshot: &mut semio_framework::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<crate::sqlite_wire::SnapshotFileResult, Fault> {
    use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework::io_schema::{IoPayload,SQLITE_SNAPSHOT}};
    let input=owner.wire.as_mut().ok_or_else(||plugin_internal_fault("snapshot original request is absent"))?;
    let limits=input.limits.native().map_err(plugin_internal_fault)?;
    snapshot.restrict_limits(limits).map_err(|error|plugin_internal_fault(error.into_message()))?;
    owner.dialect=Some(ArtifactDialect::parse_coordinate(&input.dialect).map_err(plugin_internal_fault)?);
    let encoding=semio_framework::sqlite_snapshot::SnapshotEncoding::parse(&input.encoding).map_err(|error|plugin_internal_fault(error.into_message()))?;
    if matches!(encoding,semio_framework::sqlite_snapshot::SnapshotEncoding::Text)&&std::str::from_utf8(&input.payload).is_err(){return Err(plugin_internal_fault("snapshot original text is invalid UTF8"))}
    owner.payload=Some(match encoding{semio_framework::sqlite_snapshot::SnapshotEncoding::Binary=>IoPayload::Binary(std::mem::take(&mut input.payload)),semio_framework::sqlite_snapshot::SnapshotEncoding::Text=>IoPayload::Text(String::from_utf8(std::mem::take(&mut input.payload)).unwrap())});
    owner.route=Some(semio_framework_os_kernel::io::io_mechanism::io_route(owner.dialect.as_ref().unwrap(),&ArtifactDialect::from(SQLITE_SNAPSHOT),1).await);
    let route=match owner.route.as_mut().unwrap(){Ok(route)=>&route.value,Err(error)=>{let kind=error.cause.kind;return Err(plugin_internal_fault(std::mem::replace(&mut error.cause,semio_framework_value::ValueError::literal(kind,"snapshot route refused")).into_message()))}};
    owner.hop=Some(semio_framework_os_kernel::io::io_mechanism::io_run_with_snapshot_control(route,&mut owner.payload,native,snapshot).await);
    match owner.hop.as_mut().unwrap(){
        Err(error)=>{let kind=error.cause.kind;let diagnostics=crate::sqlite_wire::encode_diagnostics(&error.diagnostics);let message=std::mem::replace(&mut error.cause,semio_framework_value::ValueError::literal(kind,"snapshot hop refused")).into_message();Ok(crate::sqlite_wire::SnapshotFileResult::Rejected(crate::sqlite_wire::SnapshotRejection{kind,message,diagnostics}))},
        Ok(output)=>match &mut output.value{IoPayload::Binary(bytes)=>Ok(crate::sqlite_wire::SnapshotFileResult::Done(crate::sqlite_wire::SnapshotFile{bytes:std::mem::take(bytes),diagnostics:crate::sqlite_wire::encode_diagnostics(&output.diagnostics)})),IoPayload::Text(_)=>Err(plugin_internal_fault("semantic SQLite export must produce binary"))},
    }
}
/// 📥️ `codec.sqlite-import` validates exact dialect and DDL before reconstructing native values.
pub async fn plugin_snapshot_sqlite_import(owner: &mut crate::sqlite_wire::SnapshotInputOwner, native: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>, snapshot: &mut semio_framework::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<crate::sqlite_wire::SnapshotPayloadResult, Fault> {
    use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework::io_schema::{IoPayload,SQLITE_SNAPSHOT}};
    let input=owner.wire.as_mut().ok_or_else(||plugin_internal_fault("snapshot original request is absent"))?;
    let limits=input.limits.native().map_err(plugin_internal_fault)?;
    snapshot.restrict_limits(limits).map_err(|error|plugin_internal_fault(error.into_message()))?;
    owner.dialect=Some(ArtifactDialect::parse_coordinate(&input.dialect).map_err(plugin_internal_fault)?);
    owner.payload=Some(IoPayload::Binary(std::mem::take(&mut input.payload)));
    owner.route=Some(semio_framework_os_kernel::io::io_mechanism::io_route(&ArtifactDialect::from(SQLITE_SNAPSHOT),owner.dialect.as_ref().unwrap(),1).await);
    let route=match owner.route.as_mut().unwrap(){Ok(route)=>&route.value,Err(error)=>{let kind=error.cause.kind;return Err(plugin_internal_fault(std::mem::replace(&mut error.cause,semio_framework_value::ValueError::literal(kind,"snapshot route refused")).into_message()))}};
    owner.hop=Some(semio_framework_os_kernel::io::io_mechanism::io_run_with_snapshot_control(route,&mut owner.payload,native,snapshot).await);
    match owner.hop.as_mut().unwrap(){
        Err(error)=>{let kind=error.cause.kind;let diagnostics=crate::sqlite_wire::encode_diagnostics(&error.diagnostics);let message=std::mem::replace(&mut error.cause,semio_framework_value::ValueError::literal(kind,"snapshot hop refused")).into_message();Ok(crate::sqlite_wire::SnapshotPayloadResult::Rejected(crate::sqlite_wire::SnapshotRejection{kind,message,diagnostics}))},
        Ok(output)=>{let diagnostics=crate::sqlite_wire::encode_diagnostics(&output.diagnostics);Ok(crate::sqlite_wire::SnapshotPayloadResult::Done(match &mut output.value{IoPayload::Binary(bytes)=>crate::sqlite_wire::SnapshotPayload{encoding:crate::sqlite_wire::SnapshotEncoding::Binary,bytes:std::mem::take(bytes),diagnostics},IoPayload::Text(text)=>crate::sqlite_wire::SnapshotPayload{encoding:crate::sqlite_wire::SnapshotEncoding::Text,bytes:std::mem::take(text).into_bytes(),diagnostics}}))},
    }
}
/// 🌱️ `codec.genesis` — the canonical empty document of `artifact_schema` at `document_id`.
pub async fn plugin_artifact_genesis<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str, document_id: &str) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
    (plugin_artifact_codec(runtime, artifact_schema)?.genesis)(document_id).await
}
/// 📥️ `codec.print-mirror` — the host's pair-validation fence for an unlinked package.
pub async fn plugin_artifact_print_mirror<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str, pack: &[u8], spr: &[u8]) -> Result<store::os_store::text_format::ArtifactTextFiles, Fault> {
    (plugin_artifact_codec(runtime, artifact_schema)?.print_mirror)(pack, spr).await
}
/// 🧩️ `codec.apply-ops` — the host-authoritative edit apply for an unlinked package.
pub async fn plugin_artifact_apply_ops<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
    (plugin_artifact_codec(runtime, artifact_schema)?.apply_ops)(pack, spr, ops, identity).await
}
/// 📜️ `codec.replay-envelopes` — the hub's Check In fold for an unlinked package.
pub async fn plugin_artifact_replay_envelopes<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str, pack: &[u8], spr: &[u8], envelopes: &[u8]) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
    (plugin_artifact_codec(runtime, artifact_schema)?.replay_envelopes)(pack, spr, envelopes).await
}
/// 📦️ Serializes the instance's full persistent document as pack+spr bytes
/// ({@link store::os_store::serialization::ArtifactPackFiles}) via `store::os_store::text_format::print_document_pack`.
pub async fn plugin_document_pack<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.document_pack())
    })
    .await
}
/// 🏗️ Whether instance `instance_id` still owes store constructor turns the reactor drives before any command.
pub async fn plugin_store_constructor_pending<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<bool, Fault> {
    with_instances_mut(runtime, |list| Ok(find_instance(list, instance_id)?.app.store_constructor_is_pending())).await
}
/// 🪪️ The archive a whole-document load of `files` into instance `instance_id` admits, stamped with that instance's store
/// identity ([`PluginApp::document_load_archive`]).
pub async fn plugin_document_load_archive<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<protocol::DocumentArchivePack, Fault> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        instance.app.document_load_archive(files)
    })
    .await
}
/// 🛬️ Admits one whole-document archive load under `operation` on instance `instance_id` (`📓️api-stepped-document-load.md`)
/// — the in-guest twin of `AppCommand::LoadDocumentArchive`.
pub async fn plugin_begin_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64, archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.begin_document_archive_load(operation, archive)
    })
    .await
}
/// 📊️ Advances one whole-document archive load for at most one poll wall and answers its status — the in-guest twin of
/// `AppCommand::PollDocumentArchiveLoad`.
pub async fn plugin_poll_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        drive_self_waking_ready(instance.app.poll_document_archive_load(operation, identity, cx))
    })
    .await
}
/// 🛑️ Requests the cancellation of one running whole-document archive load — the in-guest twin of
/// `AppCommand::CancelDocumentArchiveLoad`; the load reaches `Cancelled` over later polls.
pub async fn plugin_cancel_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.cancel_document_archive_load(operation)
    })
    .await
}
/// 📨️ Releases one terminal whole-document archive load — the in-guest twin of `AppCommand::AcknowledgeDocumentArchiveLoad`.
pub async fn plugin_acknowledge_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.acknowledge_document_archive_load(operation)
    })
    .await
}
/// 📡️ One inbound backbone delivery's turn output. `document_changed` is true when the delivery
/// merged remote edits into the document, so every view of it must be re-projected.
pub struct DocumentBackboneTurnOutputV1 {
    pub instance_id: u32,
    pub document_changed: bool,
    pub frames: Vec<Vec<u8>>,
    pub effects: Vec<Effect>,
}
fn document_backbone_effects(owner: &store::os_store::backbone::ActorBackboneChannelOwner, uri: &str) -> Result<Vec<Effect>, Fault> {
    let mut effects = Vec::new();
    for _ in 0..store::os_store::backbone::BACKBONE_CHANNEL_MAXIMUM_MESSAGES {
        let Some(payload) = owner.take_outbound().map_err(|error| plugin_internal_fault(error.to_string()))? else { break };
        effects.push(Effect::SendMessage { target: MessageEndpoint::Backbone { uri: uri.into() }, payload });
    }
    Ok(effects)
}
fn live_document_backbone_for_uri<PA: PluginApp>(runtime: &PluginRuntime<PA>, uri: &str) -> Result<Option<(u32, u64, store::os_store::backbone::ActorBackboneChannelOwner)>, Fault> {
    let bindings = runtime.document_backbones.try_borrow().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
    for index in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS {
        let Some((instance_id, binding)) = bindings.entry_at(index) else { continue };
        if binding.uri.as_deref() == Some(uri) {
            if let Some(channel) = &binding.channel {
                return Ok(Some((instance_id, binding.generation, channel.clone())));
            }
        }
    }
    Ok(None)
}
fn document_backbone_receipt_effect(receipt: &crate::document_backbone_binding::DocumentBackboneBindingReceiptV1) -> Effect {
    Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(receipt.instance_id.to_string()) }, payload: receipt.encode() }
}
pub async fn plugin_handle_document_backbone_binding<PA: PluginApp>(runtime: &PluginRuntime<PA>, source_instance: &str, payload: &[u8]) -> Result<Option<Vec<Effect>>, Fault> {
    use crate::document_backbone_binding::{decide_document_backbone_binding_v1, decode_document_backbone_binding_command_v1, DocumentBackboneBindingDecisionV1};
    let Some(command) = decode_document_backbone_binding_command_v1(payload).map_err(plugin_internal_fault)? else { return Ok(None) };
    let source_matches = source_instance.parse::<u32>().ok() == Some(command.instance_id);
    let instance_live = runtime.guest_lifetimes.try_borrow().map_err(|_| plugin_internal_fault("lifecycle authority busy"))?.get(command.instance_id).is_some_and(|slot| slot.cell.is_live());
    if !source_matches || !instance_live {
        return Ok(Some(vec![document_backbone_receipt_effect(&crate::document_backbone_binding::DocumentBackboneBindingReceiptV1::refused(&command, "plugin.document-backbone.instance-refused"))]));
    }
    let state = runtime.document_backbones.try_borrow().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?.get(command.instance_id).map(RuntimeDocumentBackboneBindingV1::state).unwrap_or_default();
    match decide_document_backbone_binding_v1(&state, &command) {
        DocumentBackboneBindingDecisionV1::Replay(receipt) | DocumentBackboneBindingDecisionV1::Refuse(receipt) => Ok(Some(vec![document_backbone_receipt_effect(&receipt)])),
        DocumentBackboneBindingDecisionV1::Bind(receipt) => {
            if live_document_backbone_for_uri(runtime, &command.uri)?.is_some() {
                return Ok(Some(vec![document_backbone_receipt_effect(&crate::document_backbone_binding::DocumentBackboneBindingReceiptV1::refused(&command, "plugin.document-backbone.binding-collision"))]));
            }
            let (channel, owner) = store::os_store::backbone::ActorBackboneChannelOwner::pair(&command.uri);
            let backbone = store::os_store::backbone::Backbones::Port(store::os_store::backbone::PortBackbone::with_channel(&command.uri, channel).await);
            let attached = with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, command.instance_id)?;
                ::semio_framework_async::poll::resolve_ready(instance.app.attach_hot_backbone(backbone))
            })
            .await;
            if attached.is_err() {
                owner.begin_retire().map_err(|error| plugin_internal_fault(error.to_string()))?;
                return Ok(Some(vec![document_backbone_receipt_effect(&crate::document_backbone_binding::DocumentBackboneBindingReceiptV1::refused(&command, "plugin.document-backbone.attach-refused"))]));
            }
            let rejected = {
                let mut bindings = runtime.document_backbones.try_borrow_mut().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
                if let Some(binding) = bindings.get_mut(command.instance_id) {
                    if binding.state() != state {
                        Some(("plugin.document-backbone.binding-collision", owner))
                    } else {
                        binding.generation = command.binding_generation;
                        binding.uri = Some(command.uri.clone());
                        binding.channel = Some(owner);
                        None
                    }
                } else if !bindings.can_insert(command.instance_id) {
                    Some(("plugin.document-backbone.binding-capacity", owner))
                } else {
                    bindings.insert_admitted(command.instance_id, RuntimeDocumentBackboneBindingV1 { generation: command.binding_generation, uri: Some(command.uri.clone()), channel: Some(owner) });
                    None
                }
            };
            if let Some((code, owner)) = rejected {
                let _ = plugin_detach_backbone(runtime, command.instance_id).await;
                owner.begin_retire().map_err(|error| plugin_internal_fault(error.to_string()))?;
                return Ok(Some(vec![document_backbone_receipt_effect(&crate::document_backbone_binding::DocumentBackboneBindingReceiptV1::refused(&command, code))]));
            }
            Ok(Some(vec![document_backbone_receipt_effect(&receipt)]))
        }
        DocumentBackboneBindingDecisionV1::Retire(receipt) => {
            let owner = runtime
                .document_backbones
                .try_borrow()
                .map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?
                .get(command.instance_id)
                .and_then(|binding| binding.channel.clone())
                .ok_or_else(|| plugin_internal_fault("live document backbone channel is missing"))?;
            owner.begin_retire().map_err(|error| plugin_internal_fault(error.to_string()))?;
            plugin_detach_backbone(runtime, command.instance_id).await?;
            if !owner.terminal_is_empty().map_err(|error| plugin_internal_fault(error.to_string()))? {
                return Err(plugin_internal_fault("retired document backbone channel did not drain"));
            }
            let mut bindings = runtime.document_backbones.try_borrow_mut().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
            let binding = bindings.get_mut(command.instance_id).ok_or_else(|| plugin_internal_fault("document backbone binding disappeared during retirement"))?;
            binding.uri = None;
            binding.channel = None;
            Ok(Some(vec![document_backbone_receipt_effect(&receipt)]))
        }
    }
}
pub async fn plugin_receive_document_backbone<PA: PluginApp>(runtime: &PluginRuntime<PA>, uri: &str, payload: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<DocumentBackboneTurnOutputV1, Fault> {
    let (instance_id, generation, owner) = live_document_backbone_for_uri(runtime, uri)?.ok_or_else(|| plugin_internal_fault("backbone message has no exact live document binding"))?;
    owner.push_inbound(uri, payload).map_err(|error| plugin_internal_fault(error.to_string()))?;
    let (reports, conflicts) = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        let reports = ::semio_framework_async::poll::resolve_ready(instance.app.tick_backbone(identity, cx))?;
        let conflicts = if reports.is_empty() { Vec::new() } else { ::semio_framework_async::poll::resolve_ready(instance.app.open_conflicts()) };
        Ok((reports, conflicts))
    })
    .await?;
    let current = live_document_backbone_for_uri(runtime, uri)?;
    if !current.as_ref().is_some_and(|(current_instance, current_generation, current_owner)| *current_instance == instance_id && *current_generation == generation && current_owner.admits_same_channel(&owner)) {
        return Err(plugin_internal_fault("document backbone authority changed during inbound delivery"));
    }
    let mut frames = Vec::new();
    for report in &reports {
        frames.push(protocol::encode_app_frame(&protocol::AppFrame::MergeReport { in_reply_to: None, report: encode_wire_serialized(report) }).await);
    }
    if !reports.is_empty() {
        frames.push(protocol::encode_app_frame(&protocol::AppFrame::Conflicts { in_reply_to: None, conflicts: encode_wire_serialized(&conflicts) }).await);
    }
    Ok(DocumentBackboneTurnOutputV1 { instance_id, document_changed: !reports.is_empty(), frames, effects: document_backbone_effects(&owner, uri)? })
}
pub fn plugin_drain_document_backbones<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> Result<Vec<Effect>, Fault> {
    let bindings = runtime.document_backbones.try_borrow().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
    let mut effects = Vec::new();
    for index in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS {
        let Some((_, binding)) = bindings.entry_at(index) else { continue };
        let (Some(uri), Some(owner)) = (binding.uri.as_deref(), binding.channel.as_ref()) else { continue };
        effects.extend(document_backbone_effects(owner, uri)?);
    }
    Ok(effects)
}
/// 🗂️ Registers `A::Snapshot`'s pack↔dsl codec under `schema` in the process-wide
/// `store::os_store::serialization::ArtifactCodec` registry — the one-liner every app's own native registration fn
/// (`register_<app>_exports()`-style) calls once per document kind so `framework/sync`'s
/// `FolderEndpoint` (and any other schema-string-keyed caller) can print/parse that kind without
/// depending on its concrete `Snapshot`/`Mutation` types.
pub async fn register_document_codec_for_app<A: ArtifactApp>(schema: impl Into<String>) -> Result<(), PluginAssemblyError>
where
    A::Mutation: Sync,
{
    let codec = store::bare_artifact_codec::<A::Snapshot, A::Mutation>(schema);
    let app = A::default();
    let owner = app.instance_id().await.to_string();
    let document_bindings = vec![semio_framework_os_kernel::io::ArtifactCodecBinding::from_codec(semio_framework_os_kernel::io::ArtifactCodecBindingChannel::DirectDocumentNative, owner.clone(), Some(A::DIALECT.artifact_kind.to_string()), vec![owner], Some(A::DIALECT.into()), None, &codec)];
    let plan = semio_framework_os_kernel::io::ArtifactAssemblyRegistryPlan {
        document_codecs: vec![codec.clone()],
        document_bindings,
        native_snapshots: semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration::from_capability(A::DIALECT.into(), codec).into_iter().collect(),
        ..Default::default()
    };
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|error| PluginAssemblyError::new("plugin-assembly.unavailable", error.to_string()))?;
    semio_framework_os_kernel::io::commit_artifact_assembly_registry_plan(&assembly, plan).map_err(|error| PluginAssemblyError::new("plugin-assembly.snapshot", error.to_string()))
}
/// 🔗️ Attaches a backbone channel by URI. The URI is resolved to a `store::os_store::backbone::PortBackbone`
/// (a pure queue relayed across the wasm sandbox to the host); the host owns the real IO endpoint.
pub async fn plugin_attach_backbone<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, uri: &str) -> Result<(), Fault> {
    let backbone: store::os_store::backbone::Backbones = store::os_store::backbone::Backbones::Port(store::os_store::backbone::PortBackbone::new(uri).await);
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.attach_backbone(backbone))
    })
    .await
}
/// ✂️ Detaches the instance's backbone channel; the document graph stays in memory.
pub async fn plugin_detach_backbone<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.detach_backbone())
    })
    .await
}
/// 🎞️ WIT `consume-media` glue — decodes the incoming `media-artifact` (`descriptor-json` + `data`)
/// and dispatches to `PluginApp::consume_media`. A whole document is admitted as a stepped archive load under `operation`
/// (the host sequence of the import, `📓️api-stepped-document-load.md` §4) and answers its pending status for the host to
/// poll; an applied import answers `None`.
pub async fn plugin_consume_media<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64, port_id: &str, descriptor_json: &str, data: Vec<u8>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<protocol::DocumentArchiveLoadStatus>, Fault> {
    let descriptor: MediaArtifactDescriptor = semio_framework_pack_json::from_json_str(descriptor_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| plugin_internal_fault(error.to_string()))?;
    let artifact = MediaArtifact { descriptor, data };
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        match ::semio_framework_async::poll::resolve_ready(instance.app.consume_media(port_id, artifact, cx)).map_err(crate::app_contract::media_artifact_fault)? {
            MediaConsumption::Applied => Ok(None),
            MediaConsumption::DocumentLoad(archive) => {
                instance.app.begin_document_archive_load(operation, archive)?;
                Ok(Some(protocol::DocumentArchiveLoadStatus { operation, state: protocol::DocumentArchiveLoadState::Pending, completed: 0, total: 0, ahead: 0, fault: Vec::new() }))
            }
        }
    })
    .await
}
/// 🎬️ Interactive workflow entry: submits without waiting for the exporter to finish.
pub async fn plugin_submit_media_export<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, port_id: &str, expected_parent_document_id: &str, expected_base_revision: u64,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let cell = runtime_instance_cell(runtime, instance_id)?;
    let mut instance = cell.host_instance().ok_or_else(|| plugin_internal_fault("media export original instance is busy or poisoned"))?;
    instance.app.submit_media_export_checked(port_id, expected_parent_document_id, expected_base_revision,receiver,cx).await
}
/// 📡️ Interactive workflow entry receives the actual initiating context and its complete failure.
pub async fn plugin_poll_media_export<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, handle: crate::media_contract::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<ArtifactMediaExportPoll>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let cell = runtime_instance_cell(runtime, instance_id)?;
    let mut instance = cell.host_instance().ok_or_else(|| plugin_internal_fault("media export original instance is busy or poisoned"))?;
    instance.app.poll_media_export(handle,receiver,cx).await
}
/// 🛑️ Interactive workflow entry: cancels the exact live operation named by the handle.
pub async fn plugin_cancel_media_export<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
    let cell = runtime_instance_cell(runtime, instance_id).map_err(|error| MediaError::Payload(handle.parent_document_id.clone(), error.message))?;
    let mut instance = cell.host_instance().ok_or_else(|| MediaError::Payload(handle.parent_document_id.clone(), format!("instance busy or poisoned: {instance_id}")))?;
    instance.app.cancel_media_export(handle).await
}
pub async fn plugin_retain_media_export_result<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,handle:crate::media_contract::ArtifactMediaExportHandleLoan<'_>,source:&mut Option<ArtifactMediaExportPoll>,status:&mut Option<crate::media_contract::ArtifactMediaExportPublishedStatus>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let cell=runtime_instance_cell(runtime,instance_id)?;let mut instance=cell.host_instance().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"download publication requires its original instance loan"))?;instance.app.retain_media_export_result(handle,source,status,cx).await
}
pub async fn plugin_take_media_export_chunk<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,handle:crate::media_contract::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<Vec<u8>>,terminal:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let cell=runtime_instance_cell(runtime,instance_id)?;let mut instance=cell.host_instance().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"media chunk requires its original instance loan"))?;instance.app.take_media_export_chunk(handle,receiver,terminal,cx).await
}
/// 🧵️ Takes one already-produced bounded action-download chunk without blocking another instance.
pub async fn plugin_take_segmented_download_chunk<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation_id: u64) -> Result<Option<Vec<u8>>, Fault> {
    let cell = runtime_instance_cell(runtime, instance_id)?;
    let mut instance = cell.host_instance().ok_or_else(|| plugin_internal_fault(format!("instance busy or poisoned: {instance_id}")))?;
    instance.app.take_segmented_download_chunk(operation_id).await
}
/// 🎞️ WIT `produce-media` glue — dispatches to `PluginApp::produce_media` and encodes the result back
/// into `(descriptor-json, data)` for the `media-artifact` WIT record. `_request_json` is reserved for
/// future parameterized requests; unused by the SDK default (see `PluginApp::produce_media`).
pub async fn plugin_produce_media<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, port_id: &str, _request_json: &str, owner: &mut crate::io::control::NativeSnapshotEncodeOwner<'_,'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<MediaArtifact>,crate::app_contract::MediaArtifactOperationFailure> {
    let cell = runtime_instance_cell(runtime, instance_id).map_err(MediaArtifactError::Import)?;
    let mut instance = cell.host_instance().ok_or_else(|| MediaArtifactError::Value(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original media instance is busy or poisoned")))?;
    instance.app.produce_media(port_id, owner,cx).await
}
// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): return type flipped from
// `UiNode` to `ui_runtime::ComponentTree`, matching `PluginApp::render`/`ArtifactApp::render` —
// this is the exact boundary `⚛️reactor/🦀️.rs`'s `poll()` calls into before handing the
// tree to `PatchTracker::diff`.
/// 🧩️ Canonical JSON of one reserved refresh section — key-ordered through a `BTreeMap` because the
/// accessors return `HashMap`s whose iteration order would otherwise change the retained text
/// leaves (and therefore the surface hash) on every render without the section itself changing.
// 🚫️async: E1 pure serialization consumed synchronously by `plugin_render_section` — see R9.
fn canonical_section_json<T: Serialize>(section: &HashMap<String, T>) -> String {
    let ordered: std::collections::BTreeMap<&str, &T> = section.iter().map(|(key, value)| (key.as_str(), value)).collect();
    serde_json::to_string(&ordered).unwrap_or_else(|_| String::from("{}"))
}
/// 🧩️ Renders one reserved section surface. A section body key never reaches `PluginApp::render`:
/// it names the object-safe accessor (`window_engagements`/`window_measures`/`tool_measures`/
/// `app_catalogue_json`) whose
/// canonical JSON `crate::app::section_component_tree` carries as retained text leaves, so the
/// shell's `SET_WINDOW_MEASURES_BY_WINDOW_ID`/`SET_TOOL_MEASURES_BY_TOOL_ID`/
/// `SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID` reducers receive the same shapes the deleted JSON
/// `refreshUi` wire produced.
// 🚫️async: E5 executor bridge — the accessors stay genuinely `async fn`; resolved synchronously
// here for the same reason `plugin_render_surface`'s own `instance.app.render` call is.
/// 🛍️ App-STATIC and view-independent: the catalogue is the app's own registered operator set,
/// already canonical JSON authored by the app, so it needs neither `view_state` nor the
/// `HashMap` key-ordering pass the three window-scoped sections above go through.
pub(crate) fn original_render_section<PA: PluginApp>(app: &mut PA, section: semio_framework::UiRefreshSection, view_state: &ViewModel) -> Result<ComponentTree, Fault> {
    let payload = match section {
        semio_framework::UiRefreshSection::Engagements => canonical_section_json(&::semio_framework_async::poll::resolve_ready(app.window_engagements(view_state))),
        semio_framework::UiRefreshSection::Measures => canonical_section_json(&::semio_framework_async::poll::resolve_ready(app.window_measures(view_state))),
        semio_framework::UiRefreshSection::Tools => canonical_section_json(&::semio_framework_async::poll::resolve_ready(app.tool_measures(view_state))),
        semio_framework::UiRefreshSection::Catalogue => ::semio_framework_async::poll::resolve_ready(app.app_catalogue_json()),
    };
    crate::window_surface::section_component_tree(section, &payload).map_err(|error| plugin_internal_fault(error.to_string()))
}
/// 📏️ Both bounds are DERIVED from the one language-neutral view-context contract
/// (`🛂️manifest/🪟️view-context/🧬️schema/🔣️.json`), not borrowed from the DFF public-action
/// admission caps this used to reuse: `MAX_PUBLIC_ACTION_BODY_BYTES` (256 KiB) is smaller
/// than the two 64 Ki-character long strings that schema alone permits, so a schema-valid
/// context was rejected and every window body became this fault card
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) async fn plugin_mount_surface<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, surface: String, body_key: String, view_state: &[u8]) -> Result<(), Fault> {
    if view_state.len() > semio_framework::MAX_SURFACE_VIEW_CONTEXT_BYTES || body_key.len() > semio_framework::MAX_SURFACE_BODY_KEY_BYTES {
        return Err(plugin_internal_fault(format!(
            "surface context exceeds its wire bound: view state {} B of {} B, body key {} B of {} B",
            view_state.len(),
            semio_framework::MAX_SURFACE_VIEW_CONTEXT_BYTES,
            body_key.len(),
            semio_framework::MAX_SURFACE_BODY_KEY_BYTES
        )));
    }
    let view = decode_wire_serialized::<ViewModel>(view_state).await?;
    let view = match (semio_framework::UiRefreshSection::from_body_key(&body_key), view.window_id.as_deref()) {
        (None, Some(window)) => view.for_window_instance(window).ok_or_else(|| plugin_internal_fault(format!("unknown surface window instance {window}")))?,
        _ => view,
    };
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.surface_contexts.insert(surface, body_key, view).map_err(plugin_internal_fault)
    })
    .await
}
pub(crate) async fn plugin_hide_surface<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, surface: &str) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.surface_contexts.remove(surface);
        Ok(())
    })
    .await
}
/// 🪟️ The surfaces background work for this instance must dirty — every mounted window
/// instance, or its app-level panels when it has no window (see
/// `SurfaceContexts::background_surfaces`). Answers empty for an instance with nothing mounted, so a
/// job that completes before the first refresh dirties nothing instead of minting a synthetic surface
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
pub(crate) async fn plugin_instance_document_surfaces<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Vec<String> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(instance.surface_contexts.document_surfaces())
    })
    .await
    .unwrap_or_default()
}
pub(crate) async fn plugin_instance_background_surfaces<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Vec<String> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(instance.surface_contexts.background_surfaces())
    })
    .await
    .unwrap_or_default()
}
pub(crate) async fn plugin_render_surface<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,surface:&str,receive:&mut dyn FnMut(&mut Option<ComponentTree>,&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Vec<ui_contract::PresenceUpdate>>,semio_framework_job::OriginalTurnFailure>{
    with_instances_turn_mut(runtime,|list|{
        let mut instance=find_instance(list,instance_id)?;
        let context=instance.surface_contexts.get(surface).ok_or_else(||plugin_internal_fault(format!("surface {surface} has no host context")))?;
        let original_render_result=::semio_framework_async::poll::resolve_ready(instance.app.render(&context.body_key,None,&context.view_state,cx));
        let Some(())=original_render_result? else{return Ok(None)};
        if !receive(instance.app.original_render_tree_source(cx)?,cx)?{return Ok(None)};
        let mut presence=::semio_framework_async::poll::resolve_ready(instance.app.take_pending_presence());
        let target=ui_contract::SurfaceId::try_from(surface).map_err(|_|plugin_internal_fault("presence surface exceeds fixed capacity"))?;
        for update in &mut presence{update.surface=target.clone();}
        Ok(Some(presence))
    }).await
}
pub async fn plugin_render<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,body_key:&str,view_state_json:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<()>,semio_framework_job::OriginalTurnFailure>{
    plugin_render_with_document(runtime,instance_id,body_key,None,view_state_json,cx).await
}
pub async fn plugin_render_with_document<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,body_key:&str,snapshot_override_json:Option<&str>,view_state_json:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<()>,semio_framework_job::OriginalTurnFailure>{
    #[derive(Deserialize)]
    #[serde(rename_all="camelCase")]
    struct WindowRenderInput{
        #[serde(default)]
        body_key:String,
        view_state:ViewModel,
        #[serde(default)]
        document_json:Option<String>,
    }
    let (resolved_body_key,view_state,override_snapshot)=if body_key.is_empty(){
        let input:WindowRenderInput=serde_json::from_str(view_state_json).map_err(|error|plugin_internal_fault(error.to_string()))?;
        (input.body_key,input.view_state,input.document_json)
    }else if let Ok(input)=serde_json::from_str::<WindowRenderInput>(view_state_json){
        let key=if input.body_key.is_empty(){body_key.to_string()}else{input.body_key};
        (key,input.view_state,input.document_json.or_else(||snapshot_override_json.map(str::to_string)))
    }else{
        let view_state:ViewModel=serde_json::from_str(view_state_json).map_err(|error|plugin_internal_fault(error.to_string()))?;
        (body_key.to_string(),view_state,snapshot_override_json.map(str::to_string))
    };
    with_instances_turn_mut(runtime,|list|{
        let mut instance=find_instance(list,instance_id)?;
        instance.surface_contexts.update_view(&view_state);
        ::semio_framework_async::poll::resolve_ready(instance.app.render(&resolved_body_key,override_snapshot.as_deref(),&view_state,cx))
    }).await
}
/// 🐢️ The raw fnv1a-64 core — a tiny non-cryptographic hash for cheap "did this section's content
/// change" checks, not a security boundary, just change detection, so speed over collision-resistance
/// is the right tradeoff (mirrors the identical pattern already used for `cached_snapshot_json` in
/// puzzle's plugin). Used by `ui_refresh_fnv1a_hash` below (`plugin_refresh_ui`'s legacy JSON
/// `refreshUi` wire, unrelated to channel v12's binary `AppCommand`/`AppFrame`).
// 🚫️async: E1 pure computation consumed by `format!`'s sync formatting argument — see R9.
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
/// 🐢️ A tiny non-cryptographic hash (fnv1a-64) for cheap "did this section's content change" checks —
/// not a security boundary, just change detection, so speed over collision-resistance is the right
/// tradeoff (mirrors the identical pattern already used for `cached_snapshot_json` in puzzle's plugin).
async fn ui_refresh_fnv1a_hash(bytes: &[u8]) -> String {
    format!("{:016x}", fnv1a64(bytes))
}
/// 🐢️ Hashes `value`'s canonical JSON serialization and returns `(hash, Some(value))` when it differs
/// from `known_hash`, or `(hash, None)` when unchanged — the response omits the payload either way the
/// caller doesn't need it, keeping the wire payload proportional to what actually changed.
async fn ui_refresh_section<T: Serialize>(value: &T, known_hash: Option<&str>) -> (String, Option<Value>) {
    let wire = serde_json::to_string(value).unwrap_or_default();
    let hash = ui_refresh_fnv1a_hash(wire.as_bytes()).await;
    if known_hash == Some(hash.as_str()) {
        (hash, None)
    } else {
        let payload = serde_json::from_str(&wire).unwrap_or(Value::Null);
        (hash, Some(payload))
    }
}
/// 🐢️ Batched, hash-conditional UI refresh: replaces the individual
/// `render`/`windowEngagements`/`windowMeasures`/`appLabels` WASM round trips a full `refreshUi` used
/// to make with **one** call. `request_json` lists every section the host wants (windows/panels by
/// `{key, bodyKey, hash}`, engagements/measures/labels each `{hash}`); the response includes a payload
/// only for sections whose hash differs from what the host already holds. Utility bars are no longer a
/// plugin section — the renderer derives them from the utility registry via `derive_utility_nodes`.
///
/// 🌉️ `serde_json::Value` is foreign and has no `ToValue` impl (orphan rule forbids adding
/// one here) — bridges the field through the existing `DslValue <-> serde_json::Value` `From`
/// conversion instead of the derive's default `ToValue::to_value` call.
///
/// ⏱️ Arm/advance background work BEFORE rendering below, not after — e.g. a `flowEvalTick`
/// chain's `computing_json` must be fresh by the time this same pass renders the graph, or a
/// cold-start load would render one full refresh cycle behind (nothing flagged as computing
/// until the *next* refresh).
///
/// 🪟️ Stamp this window's instance id and its own active utility into the view state before
/// rendering, so a `ArtifactApp` can key per-window options and utility-driven scene state off
/// `view_state.window_id` / `view_state.active_utility_id` and never off the focused window alone.
///
/// 🚧️ `utilities` intentionally unhandled here: `PluginApp`/`ArtifactApp` currently expose no
/// object-safe `utilities()` accessor (mid-refactor elsewhere toward a declarative window-kind
/// utility builder — unrelated to this ticket; see `tools`/`tool_measures` above for the analogous
/// mode-level mechanism that IS wired up). No puzzle2d scope ever requests `utilities: true` (it
/// uses static window-kind-scoped utilities only), so `request.utilities` is always empty in
/// practice; wire this up once the utilities API refactor lands.
///
/// 🗣️ Labels no longer need a runtime overlay round-trip: the manifest itself now carries a full
/// `LocalizedLabel` matrix per field, resolved shell-side from the active locale/terminology —
/// see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND. A
/// `labels` refresh request now always comes back empty; kept accepting it (rather than erroring)
/// so an unupdated shell doesn't break.
pub async fn plugin_refresh_ui<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, request_json: &str,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<String>,semio_framework_job::OriginalTurnFailure> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SectionRequest {
        key: String,
        #[serde(default)]
        body_key: String,
        #[serde(default)]
        hash: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SingleRequest {
        #[serde(default)]
        hash: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct RefreshRequest {
        view_state: ViewModel,
        #[serde(default)]
        windows: Vec<SectionRequest>,
        #[serde(default)]
        panels: Vec<SectionRequest>,
        #[serde(default)]
        utilities: Vec<SectionRequest>,
        #[serde(default)]
        engagements: Option<SingleRequest>,
        #[serde(default)]
        measures: Option<SingleRequest>,
        #[serde(default)]
        tools: Option<SingleRequest>,
        #[serde(default)]
        labels: Option<SingleRequest>,
    }
    fn section_value_to_value(value: &Option<Value>) -> DslValue {
        match value {
            Some(v) => DslValue::from(v),
            None => DslValue::Null,
        }
    }
    #[derive(Serialize, semio_framework_value::ToValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    struct SectionResponse {
        key: String,
        hash: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none", serialize_with = "section_value_to_value")]
        value: Option<Value>,
    }
    #[derive(Serialize, semio_framework_value::ToValue, Default)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    struct RefreshResponse {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[value(skip_serializing_if = "Vec::is_empty")]
        windows: Vec<SectionResponse>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[value(skip_serializing_if = "Vec::is_empty")]
        panels: Vec<SectionResponse>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[value(skip_serializing_if = "Vec::is_empty")]
        utilities: Vec<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        engagements: Option<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        measures: Option<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        tools: Option<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        labels: Option<SectionResponse>,
        /// ⏱️ See `ArtifactApp::pending_effects` — e.g. a `flowEvalTick` chain resuming after this refresh.
        /// `Effect` derives `ToValue` directly (`🎠️kernel/🦀️.rs`) — no bridge needed, the
        /// derive's default per-element `ToValue::to_value` call handles this `Vec<Effect>` field.
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[value(skip_serializing_if = "Vec::is_empty")]
        requested_effects: Vec<Effect>,
    }
    let request: RefreshRequest = serde_json::from_str(request_json).map_err(|error|plugin_internal_fault(error.to_string()))?;
    with_instances_turn_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.surface_contexts.update_view(&request.view_state);
        let mut response = RefreshResponse { requested_effects: ::semio_framework_async::poll::resolve_ready(instance.app.pending_effects(Some(&request.view_state))), ..RefreshResponse::default() };
        for entry in &request.windows {
            let window_view_state = request.view_state.for_window_instance(&entry.key).ok_or_else(|| plugin_internal_fault(format!("unknown render window instance {}", entry.key)))?;
            let Some(()) = ::semio_framework_async::poll::resolve_ready(instance.app.render(&entry.body_key,None,&window_view_state,cx))? else{return Ok(None)};
            let node=instance.app.original_render_tree_source(cx)?.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"ready window source has no original tree"))?;
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&node.root, entry.hash.as_deref()));
            response.windows.push(SectionResponse { key: entry.key.clone(), hash, value });
        }
        let panel_view_state = request.view_state.for_panel();
        for entry in &request.panels {
            let Some(()) = ::semio_framework_async::poll::resolve_ready(instance.app.render(&entry.body_key,None,&panel_view_state,cx))? else{return Ok(None)};
            let node=instance.app.original_render_tree_source(cx)?.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"ready panel source has no original tree"))?;
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&node.root, entry.hash.as_deref()));
            response.panels.push(SectionResponse { key: entry.key.clone(), hash, value });
        }
        let _ = &request.utilities;
        if let Some(requested) = &request.engagements {
            let engagements = ::semio_framework_async::poll::resolve_ready(instance.app.window_engagements(&request.view_state));
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&engagements, requested.hash.as_deref()));
            response.engagements = Some(SectionResponse { key: "engagements".into(), hash, value });
        }
        if let Some(requested) = &request.measures {
            let measures = ::semio_framework_async::poll::resolve_ready(instance.app.window_measures(&request.view_state));
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&measures, requested.hash.as_deref()));
            response.measures = Some(SectionResponse { key: "measures".into(), hash, value });
        }
        if let Some(requested) = &request.tools {
            let tool_measures = ::semio_framework_async::poll::resolve_ready(instance.app.tool_measures(&request.view_state));
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&tool_measures, requested.hash.as_deref()));
            response.tools = Some(SectionResponse { key: "tools".into(), hash, value });
        }
        let _ = &request.labels;
        Ok(Some(serde_json::to_string(&response).map_err(|error|plugin_internal_fault(error.to_string()))?))
    })
    .await
}
/// 🖱️ Wire shape for an on-demand context-menu request. Module-scoped (not nested in
/// `plugin_context_menu`) so `plugin_exchange`'s `AppCommand::ContextMenu` arm below can decode the
/// same typed shape directly off the binary wire instead of round-tripping through JSON strings.
#[derive(Deserialize, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct ContextMenuWireRequest {
    menu: UiMenuRef,
    pub(crate) view_state: ViewModel,
    #[serde(default)]
    #[value(default)]
    surface: Option<ContextMenuSurfaceTarget>,
    #[serde(default)]
    #[value(default)]
    window_instance_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    point: Option<ContextMenuPoint>,
}
impl ContextMenuWireRequest {
    fn into_parts(self) -> Result<(ContextMenuRequest, ViewModel), Fault> {
        let view = match self.window_instance_id.as_deref() {
            Some(window) => self.view_state.for_window_instance(window).ok_or_else(|| plugin_internal_fault(format!("unknown context-menu window instance {window}")))?,
            None => self.view_state.for_panel(),
        };
        let request = ContextMenuRequest { menu: self.menu, surface: self.surface, window_instance_id: self.window_instance_id, point: self.point };
        Ok((request, view))
    }
}
/// 🖱️ On-demand context-menu computation (WIT `context-menu` export's SDK entry point) — never
/// cached, never part of `refresh_ui`. String-in/string-out JSON entry point for the WIT boundary.
pub async fn plugin_context_menu<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, request_json: &str) -> Result<String, String> {
    let wire: ContextMenuWireRequest = serde_json::from_str(request_json).map_err(|error| error.to_string())?;
    let (request, view_state) = wire.into_parts().map_err(|fault| fault.message)?;
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        let items = ::semio_framework_async::poll::resolve_ready(instance.app.context_menu(&request, &view_state));
        Ok(serde_json::to_string(&ContextMenuResponse { items }).unwrap_or_else(|_| r#"{"items":[]}"#.into()))
    })
    .await
    .map_err(|fault| fault.message)
}
/// 🎯️ `AppCommand::ArtifactCommand`'s Wave 1 mapping — the same magic action-name strings
/// `VcsArtifactApp::dispatch_action` already intercepts for `store::os_store::store_schema::ArtifactCommand` verbs (duplicated
/// here rather than imported since `app`'s `HISTORY_ACTION_IDS` const is private — see
/// `plugin_exchange`'s doc for why this frame is scoped to history verbs only in Wave 1).
const DOCUMENT_COMMAND_ACTION_IDS: [&str; 6] = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"];
/// 📤️ Collects `result`'s side-channel effects/events as wire-encoded bytes — channel v12 (A4)
/// dropped `AppFrame::Effects`/`AppFrame::Events`, since effects/events now travel straight into
/// `TurnResult` (design-abi.md §2/§4) instead of being wrapped as a frame; `PluginExchangeOutput`
/// carries them alongside `frames`. Shared by `AppCommand::Command`'s and
/// `AppCommand::ArtifactCommand`'s handling below.
async fn push_invocation_side_frames(effects: &mut Vec<Vec<u8>>, events: &mut Vec<Vec<u8>>, result: &InvocationResult) {
    for effect in result.requested_effects.iter() {
        effects.push(encode_wire_serialized(effect));
    }
    for event in result.events.iter() {
        events.push(encode_wire_serialized(event));
    }
}
async fn opening_role(role: u8) -> Result<semio_framework::AppRole, Fault> {
    match role {
        0 => Ok(semio_framework::AppRole::Viewer),
        1 => Ok(semio_framework::AppRole::Editor),
        value => Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-role"), format!("opening role {value} must be 0 (viewer) or 1 (editor)"))),
    }
}
async fn opening_dialect(artifact_kind: String, standard: String, subset: String) -> Result<semio_framework_artifact_reference::ArtifactDialect, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    for (field, value) in [("artifact_kind", &artifact_kind), ("standard", &standard), ("subset", &subset)] {
        if value.trim().is_empty() {
            return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-dialect"), format!("opening {field} must be non-empty")));
        }
    }
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind, standard, subset };
    semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&dialect.to_coordinate()).map_err(|error| Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-dialect"), error))?;
    Ok(dialect)
}
async fn relay_opening_command(action_id: &'static str, arguments: Value) -> Result<Effect, Fault> {
    Ok(Effect::ReplayShellCommand { action_id: action_id.into(), args: Some(DslValue::from(&arguments)) })
}
async fn relay_open_artifact(artifact_ref: String, role_wire: u8, plugin_id: String, app_id: String) -> Result<Effect, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    let role = opening_role(role_wire).await?;
    let (dialect, artifact_role) = semio_framework::parse_surface_app_id(&artifact_ref).map_err(|error| Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-artifact-ref"), error))?;
    if role != artifact_role {
        return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.role-mismatch"), format!("artifact ref {artifact_ref} declares {} but command declares {}", artifact_role.as_str(), role.as_str())));
    }
    match (plugin_id.trim().is_empty(), app_id.trim().is_empty()) {
        (true, true) => {}
        (false, false) => {
            let (app_dialect, app_role) = semio_framework::parse_surface_app_id(&app_id).map_err(|error| Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-app-ref"), error))?;
            if app_dialect != dialect || app_role != role {
                return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.app-mismatch"), format!("explicit app {app_id} does not serve {}#{}", dialect.to_coordinate(), role.as_str())));
            }
        }
        _ => return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.partial-app-ref"), "plugin_id and app_id must either both be empty or both be set")),
    }
    relay_opening_command("os.open-artifact", serde_json::json!({ "artifactRef": artifact_ref, "role": role_wire, "pluginId": plugin_id, "appId": app_id })).await
}
async fn relay_set_default_app(artifact_kind: String, standard: String, subset: String, role_wire: u8, plugin_id: String, app_id: String) -> Result<Effect, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    let dialect = opening_dialect(artifact_kind.clone(), standard.clone(), subset.clone()).await?;
    let role = opening_role(role_wire).await?;
    if plugin_id.trim().is_empty() {
        return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-app-ref"), "plugin_id must be non-empty when setting a default app"));
    }
    let (app_dialect, app_role) = semio_framework::parse_surface_app_id(&app_id).map_err(|error| Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-app-ref"), error))?;
    if app_dialect != dialect || app_role != role {
        return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.app-mismatch"), format!("default app {app_id} does not serve {}#{}", dialect.to_coordinate(), role.as_str())));
    }
    relay_opening_command("os.set-default-app", serde_json::json!({ "artifactKind": artifact_kind, "standard": standard, "subset": subset, "role": role_wire, "pluginId": plugin_id, "appId": app_id })).await
}
async fn relay_clear_default_app(artifact_kind: String, standard: String, subset: String, role_wire: u8) -> Result<Effect, Fault> {
    opening_dialect(artifact_kind.clone(), standard.clone(), subset.clone()).await?;
    opening_role(role_wire).await?;
    relay_opening_command("os.clear-default-app", serde_json::json!({ "artifactKind": artifact_kind, "standard": standard, "subset": subset, "role": role_wire })).await
}
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-opening-command-relay/🦀️.rs");
/// 🔀️ Frames a `TransactionProposalDraft` (contract §5.1) as `AppFrame::TransactionProposal`,
/// wire-encoding each foreign step via `encode_wire_serialized` (contract §2's per-element
/// `ForeignStep` encoding note) and minting a proposal id scoped to this dispatch
/// (`"<instance_id>:<seq>"`, unique enough for one instance's own in-flight proposals — all a
/// single-command dispatch ever needs).
async fn transaction_proposal_frame(instance_id: u32, seq: u64, proposal: TransactionProposalDraft) -> protocol::AppFrame {
    let mut foreign = Vec::with_capacity(proposal.foreign.len());
    for step in proposal.foreign.iter() {
        foreign.push(encode_wire_serialized(step));
    }
    protocol::AppFrame::TransactionProposal { in_reply_to: seq, proposal_id: format!("{instance_id}:{seq}"), local_ops: proposal.local_ops, foreign }
}
async fn encode_exchange_frames(frames: &[protocol::AppFrame]) -> Vec<Vec<u8>> {
    let mut encoded = Vec::with_capacity(frames.len());
    for frame in frames {
        encoded.push(protocol::encode_app_frame(frame).await);
    }
    encoded
}
/// 🔀️ `PluginExchangeOutput` — `plugin_exchange`'s return shape. `frames` still carries the
/// surviving `protocol::AppFrame` wire encodings (routed through `route_app_frame` by the
/// reactor's `poll`); `effects`/`events` are wire-encoded `kernel::Effect`/`kernel::AppEvent`
/// bytes collected directly — channel v12 (A4) removed `AppFrame::Effects`/`AppFrame::Events`,
/// since design-abi.md §2/§4 has effects/events travel straight into `TurnResult`, never wrapped
/// as a frame.

#[path="🎞️command/📥️custody/🛂️manifest/🦀️.rs"]
mod original_manifest_receiving;
pub use original_manifest_receiving::OriginalDecodedPluginCommand;
use original_manifest_receiving::OriginalManifestCommandReceiving;
#[cfg(feature="original-actor-context-testing")]
/// 🧪️ Qualifies the canonical receiving body without mounting unrelated app fixture suites.
pub fn original_manifest_receiving_law(){original_manifest_receiving::tests::original_manifest_fields_keep_paid_source_and_denied_transfer()}

pub enum PluginCommandIngress {
    Encoded(semio_framework::kernel::PagedCommand),
    Decoding(protocol::PagedAppCommandDecodeCursor),
    Decoded(OriginalDecodedPluginCommand),
    Closing { cursor: protocol::PagedAppCommandDecodeCursor, fault: Fault },
    ClosingDecoded { owner: OriginalDecodedPluginCommand, fault: Fault },
    FailedDecoded{owner:OriginalDecodedPluginCommand,failure:Option<semio_framework_job::OriginalTurnFailure>,identity:(semio_framework_job::OperationId,semio_framework_job::Generation),close_failure:Option<semio_framework_job::OriginalTurnFailure>,compound:Option<semio_framework_job::OriginalClosureFailure>,raw_media:Option<MediaArtifactError>},
}
pub(crate) enum OriginalCommandIngressRetirement{Pending(PluginCommandIngress),Failed(semio_framework_job::OriginalTurnFailure)}
enum PluginCommandIngressStep {
    Pending(PluginCommandIngress),
    Ready(OriginalDecodedPluginCommand),
    TerminalFault(Fault),
    TerminalOriginalFailure(semio_framework_job::OriginalTurnFailure),
}
/// 📥️ Ingress moves one turn may spend driving a command from its assembled pages to dispatch.
///
/// Every state [`PluginCommandIngress::step`] reaches is a bounded move: a decode state reads one
/// `read_bounded_bytes` field, a closing state releases exactly one page. Answering `Pending`
/// after each of them cost the HOST a whole turn round trip per move — measured 4.006 turns for a
/// ONE-page `AppCommand::CommandText` (`a_long_command_stream_never_pins_the_retained_ingress_
/// authority`: 320 commands, 1 282 turns), five for a one-page `AppCommand::Command`, and one more
/// for every further page. The grant is the whole close ladder — `COMMAND_MAXIMUM_PAGES` page
/// releases plus the three decoded field stages plus the header — so a command assembles, decodes
/// and dispatches inside the turn that carried its LAST page, and a cancelled one retires inside a
/// single turn too. Ticket 26/09/02 wave B24.
const COMMAND_INGRESS_MOVES_PER_TURN: usize = semio_framework::kernel::COMMAND_MAXIMUM_PAGES + 8;
impl PluginCommandIngress {
    pub(crate) fn retire_failed_step_on_original(self,cx:&mut semio_framework_job::StepContext<'_>)->OriginalCommandIngressRetirement{
        if !matches!(self,Self::FailedDecoded{..}){return OriginalCommandIngressRetirement::Pending(self)}
        match self.advance_original(1,cx){PluginCommandIngressStep::Pending(owner)=>OriginalCommandIngressRetirement::Pending(owner),PluginCommandIngressStep::TerminalOriginalFailure(failure)=>OriginalCommandIngressRetirement::Failed(failure),_=>unreachable!("failed original source only closes and returns its full original cause")}
    }

    fn advance_original(self,moves:usize,cx:&mut semio_framework_job::StepContext<'_>)->PluginCommandIngressStep{
        if let Self::FailedDecoded{mut owner,mut failure,identity,mut close_failure,mut compound,mut raw_media}=self{
            if identity!=(cx.operation(),cx.generation())||cx.should_yield(){return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})}
            if raw_media.is_some(){
                match semio_framework_job::admit_original_domain_failure(&mut raw_media,cx){Ok(Some(original))=>failure=Some(original),Ok(None)=>{},Err(refusal)=>close_failure=Some(refusal)}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            if compound.is_some(){
                match semio_framework_job::admit_original_domain_failure(&mut compound,cx){Ok(Some(original))=>failure=Some(original),Ok(None)=>{},Err(refusal)=>close_failure=Some(refusal)}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            let grant=cx.retained_grant();
            if close_failure.is_some(){
                if grant.maximum_items>0&&grant.maximum_depth>0{compound=Some(semio_framework_job::OriginalClosureFailure{initial:failure.take().expect("failed decoded initial source remains original"),closure:close_failure.take().expect("original closure cause remains original")});cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).expect("original closure donation was preflighted");cx.consume_fuel(1)}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            if !owner.terminal_is_empty(){
                match owner.close_original_step(cx){Ok(_)=>{},Err(refusal)=>close_failure=Some(refusal)}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            if grant.maximum_items>0&&grant.maximum_depth>0{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).expect("original failure donation was preflighted");cx.consume_fuel(1);return PluginCommandIngressStep::TerminalOriginalFailure(failure.take().expect("failed decoded initial source remains original"))}
            PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
        }else{self.advance(moves)}
    }
    /// 📥️ Drives the owner until it is dispatchable, terminal, or the turn's move grant is spent.
    fn advance(self, moves: usize) -> PluginCommandIngressStep {
        let mut owner = self;
        for _ in 0..moves {
            match owner.step() {
                PluginCommandIngressStep::Pending(next) => owner = next,
                terminal => return terminal,
            }
        }
        PluginCommandIngressStep::Pending(owner)
    }
    pub(crate) fn retire_step(self) -> Option<Self> {
        if matches!(self,Self::FailedDecoded{..}){return Some(self)}
        match self.cancel(plugin_internal_fault("command lifetime is closing")).step() {
            PluginCommandIngressStep::Pending(owner) => Some(owner),
            PluginCommandIngressStep::TerminalFault(_) => None,
            PluginCommandIngressStep::TerminalOriginalFailure(_)=>unreachable!("whole failed decoded sources require their original retirement caller"),
            PluginCommandIngressStep::Ready(_) => unreachable!("cancelled ingress cannot dispatch"),
        }
    }
    pub fn cancel(self, fault: Fault) -> Self {
        match self {
            Self::Encoded(command) => Self::Closing { cursor: protocol::PagedAppCommandDecodeCursor::new(command), fault },
            Self::Decoding(cursor) | Self::Closing { cursor, .. } => Self::Closing { cursor, fault },
            Self::Decoded(owner) | Self::ClosingDecoded { owner, .. } => match owner.receiving.as_ref(){Some(receiving)=>Self::FailedDecoded{identity:receiving.identity(),owner,failure:Some(fault.into()),close_failure:None,compound:None,raw_media:None},None=>Self::ClosingDecoded { owner, fault }},
            original @ Self::FailedDecoded{..}=>original,
        }
    }
    fn step(self) -> PluginCommandIngressStep {
        match self {
            Self::Encoded(encoded) => {
                let cursor = protocol::PagedAppCommandDecodeCursor::new(encoded);
                if cursor.kind() == 28 {
                    PluginCommandIngressStep::Pending(Self::Closing { cursor, fault: plugin_internal_fault("Presence commands require the reserved presence ingress authority") })
                } else {
                    PluginCommandIngressStep::Pending(Self::Decoding(cursor))
                }
            }
            Self::Decoding(mut cursor) => match cursor.step() {
                Ok(Some(command)) => PluginCommandIngressStep::Pending(Self::Decoded(OriginalDecodedPluginCommand::new(command))),
                Ok(None) => PluginCommandIngressStep::Pending(Self::Decoding(cursor)),
                Err(fault) => PluginCommandIngressStep::Pending(Self::Closing { cursor, fault }),
            },
            Self::Decoded(owner) => PluginCommandIngressStep::Ready(owner),
            original @ Self::FailedDecoded{..}=>PluginCommandIngressStep::Pending(original),
            Self::Closing { mut cursor, fault } => {
                let (complete, _) = cursor.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if complete && cursor.terminal_is_empty() {
                    PluginCommandIngressStep::TerminalFault(fault)
                } else {
                    PluginCommandIngressStep::Pending(Self::Closing { cursor, fault })
                }
            }
            Self::ClosingDecoded { mut owner, fault } => {
                let (complete, _, _) = owner.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if complete && owner.terminal_is_empty() {
                    PluginCommandIngressStep::TerminalFault(fault)
                } else {
                    PluginCommandIngressStep::Pending(Self::ClosingDecoded { owner, fault })
                }
            }
        }
    }
}
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-paged-command-ingress/🦀️.rs");
#[derive(Default)]
pub struct PluginExchangeOutput {
    pub original_receiving:Option<OriginalPublicationReceiving>,
    pub constructor_receipt: Option<(crate::document_ingress::ArtifactStoreConstructorKind, crate::document_ingress::ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)>,
    pub preparation_receipt: Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)>,
    pub retirement_receipt: Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)>,
    pub history_command_receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    pub tool_run_receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    pub frames: Vec<Vec<u8>>,
    pub effects: Vec<Vec<u8>>,
    pub events: Vec<Vec<u8>>,
    pub retry_command: Option<(u64, PluginCommandIngress)>,
    pub command_terminal_fault: Option<Vec<u8>>,
    pub presence_pending: Option<u64>,
    pub presence_terminal: Option<u64>,
    pub presence_terminal_fault: Option<Vec<u8>>,
    pub typed_operation_results: Vec<TypedOperationResultPage>,
}
/// 🏷️ Names the one actual producer whose full physical receipt was posted in the initiating context.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalPublicationFamily{Constructor,Preparation,Retirement,History,Tool}
/// 🧾️ Captures actual already-posted effects, exact initiating grant and recipient before another loan.
#[derive(Clone,Copy)]
pub struct OriginalPublicationReceiving{pub family:Option<OriginalPublicationFamily>,pub before:RetainedCloneProgress,pub posted:RetainedCloneProgress,pub grant:RetainedCloneGrant,pub recipient:usize}
/// 🧮️ Intersects two original authorities independently without creating new currency.
pub(crate) fn original_plugin_turn_grant(policy:RetainedCloneGrant,remaining:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:policy.maximum_items.min(remaining.maximum_items),maximum_copy_bytes:policy.maximum_copy_bytes.min(remaining.maximum_copy_bytes),maximum_capacity_bytes:policy.maximum_capacity_bytes.min(remaining.maximum_capacity_bytes),maximum_release_bytes:policy.maximum_release_bytes.min(remaining.maximum_release_bytes),maximum_depth:policy.maximum_depth.min(remaining.maximum_depth)}}
/// 🧾️ What one host turn still owns after its units spent `spent`: every consumed axis shrinks, nesting depth is not consumed.
pub(crate) fn remaining_plugin_turn_grant(turn:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:turn.maximum_items.saturating_sub(spent.copied_items),maximum_copy_bytes:turn.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:turn.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:turn.maximum_release_bytes.saturating_sub(spent.released_bytes),maximum_depth:turn.maximum_depth}}
impl PluginExchangeOutput{
    /// 🧾️ Keeps original ownership receipts ahead of the next semantic producer.
    fn ownership_receipt_pending(&self)->bool{self.original_receiving.is_some()||self.constructor_receipt.is_some()||self.preparation_receipt.is_some()||self.retirement_receipt.is_some()||self.history_command_receipt.is_some()||self.tool_run_receipt.is_some()}
    fn collect_original_receipts<PA:PluginApp>(&mut self,app:&mut PA){self.constructor_receipt=app.take_constructor_receipt();self.preparation_receipt=app.take_typed_operation_preparation_receipt();self.retirement_receipt=app.take_typed_operation_retirement_receipt();self.history_command_receipt=app.take_history_command_receipt();self.tool_run_receipt=app.take_tool_run_receipt();}
    fn original_receipt_family(&self)->Option<OriginalPublicationFamily>{let mut families=[self.constructor_receipt.is_some().then_some(OriginalPublicationFamily::Constructor),self.preparation_receipt.is_some().then_some(OriginalPublicationFamily::Preparation),self.retirement_receipt.is_some().then_some(OriginalPublicationFamily::Retirement),self.history_command_receipt.is_some().then_some(OriginalPublicationFamily::History),self.tool_run_receipt.is_some().then_some(OriginalPublicationFamily::Tool)].into_iter().flatten();let first=families.next()?;families.next().is_none().then_some(first)}
}
/// 🎰️ Highest typed-operation slot occupancy any instance of this guest has reached, and whether that
/// peak still owes its drain witness — the two values that bound the diagnostic below to at most two
/// console lines per distinct peak, i.e. at most `2 × ARTIFACT_LIVE_OUTPUT_SLOTS` for a whole session.
static PEAK_TYPED_OPERATION_SLOTS: AtomicU64 = AtomicU64::new(0);
static TYPED_OPERATION_SLOT_DRAIN_PENDING: AtomicBool = AtomicBool::new(false);
/// 🎰️ Reports the guest's live typed-operation slot occupancy on the console channel the rest of this
/// runtime's `[TRACE]` diagnostics already use — no wire, no protocol frame, and nothing the host may
/// branch on. A browser probe reads `live=0` as the proof every admitted slot reached its release path;
/// a `peak` that rises and never drains is the retirement leak that refused
/// `engagementAbort`/`setCamera`/`openVortexSuggestions` 39 times in the 2026-09-11 puzzle3d battery
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B8). Every new peak and its first full drain print
/// unconditionally because that pair IS the leak signal and is self-limiting; every intermediate
/// transition prints only under [`semio_framework_trace::runtime_diagnostics_enabled`].
fn trace_typed_operation_slot_occupancy<PA: PluginApp>(app: &PA, instance: u32) {
    let live = app.live_typed_operation_slots() as u64;
    let previous = PEAK_TYPED_OPERATION_SLOTS.fetch_max(live, Ordering::Relaxed);
    let peak = previous.max(live);
    let raised = live > previous;
    if raised {
        TYPED_OPERATION_SLOT_DRAIN_PENDING.store(true, Ordering::Relaxed);
    }
    let drained = live == 0 && TYPED_OPERATION_SLOT_DRAIN_PENDING.swap(false, Ordering::Relaxed);
    if !raised && !drained && !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    eprintln!("[TRACE] typed-operation slots instance={instance} live={live}/{} peak={peak}", semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS);
}
fn advance_typed_operation_output_with_leftover<PA: PluginApp>(app: &mut PA, instance: u32, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(PluginExchangeOutput, Option<TypedOperationLeftover>), crate::PluginLifecycleFailure> {
    let mut output=PluginExchangeOutput::default();output.collect_original_receipts(app);if output.ownership_receipt_pending(){return Ok((output,None))}
    let before=cx.retained_progress();let original=cx.retained_grant();let recipient=cx.original_recipient_identity();
    ::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity,cx))?;
    if let Some((grant,progress))=app.take_local_interaction_query_receipt(){
        let remaining=cx.retained_grant();let covered=grant.maximum_items<=remaining.maximum_items&&grant.maximum_copy_bytes<=remaining.maximum_copy_bytes&&grant.maximum_capacity_bytes<=remaining.maximum_capacity_bytes&&grant.maximum_release_bytes<=remaining.maximum_release_bytes&&grant.maximum_depth<=remaining.maximum_depth&&progress.fits(grant);cx.consume_retained(progress)?;
        if !covered{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original interaction receipt exceeds its caller grant").with_retained_progress(progress).into())}
        return Ok((PluginExchangeOutput::default(),None));
    }
    trace_typed_operation_slot_occupancy(app, instance);
    output.collect_original_receipts(app);let after=cx.retained_progress();let subtract=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original publication recipient moved backwards"));let posted=RetainedCloneProgress{copied_items:subtract(after.copied_items,before.copied_items)?,copied_bytes:subtract(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:subtract(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:subtract(after.released_bytes,before.released_bytes)?};if posted!=RetainedCloneProgress::default(){output.original_receiving=Some(OriginalPublicationReceiving{family:output.original_receipt_family(),before,posted,grant:original,recipient});}
    if output.ownership_receipt_pending(){return Ok((output,None));}
    match app.take_reserved_commit_outcome() {
        Some(Ok(result)) => {
            output.frames.push(::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&protocol::AppFrame::Invocation {
                in_reply_to: 0,
                output: encode_wire_serialized(&result.output),
                diagnostics: encode_wire_serialized(&result.diagnostics),
                ui_scope: encode_wire_serialized(&result.ui_scope),
                history_patch: encode_wire_serialized(&result.history_patch),
                messages: Vec::new(),
                mutations: encode_wire_serialized(&result.mutations),
                inverse_group: encode_wire_serialized(&result.inverse_group),
            })));
            ::semio_framework_async::poll::resolve_ready(push_invocation_side_frames(&mut output.effects, &mut output.events, &result));
        }
        Some(Err(fault)) => {
            let mut frames = Vec::new();
            ::semio_framework_async::poll::resolve_ready(push_app_fault(&mut frames, None, fault));
            for frame in frames.iter() {
                output.frames.push(::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(frame)));
            }
        }
        None => {}
    }
    while output.typed_operation_results.len() < TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM {
        let Some(page) = app.take_typed_operation_result_page(instance) else { break };
        output.typed_operation_results.push(page);
    }
    if let Some(effect) = app.take_typed_operation_effect() {
        output.effects.push(encode_wire_serialized(&effect));
    }
    if let Some(event) = app.take_typed_operation_event() {
        output.events.push(encode_wire_serialized(&event));
    }
    if let Some(completion) = ::semio_framework_async::poll::resolve_ready(app.take_typed_operation_completion())? {
        output.frames.push(::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&protocol::AppFrame::OperationCompleted {
            operation: completion.operation,
            revision: completion.revision,
            ui_scope: encode_wire_serialized(&completion.ui_scope),
            history_patch: completion.history_patch.as_ref().map(encode_wire_serialized).unwrap_or_default(),
        })));
    }
    let mut leftover = None;
    if let Some(progress) = app.take_typed_operation_ui_progress() {
        output.frames.push(::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&protocol::AppFrame::Invocation {
            in_reply_to: 0,
            output: progress.leftover.as_ref().map(|leftover| encode_wire_serialized(&leftover.view)).unwrap_or_default(),
            diagnostics: Vec::new(),
            ui_scope: encode_wire_serialized(&progress.ui_scope),
            history_patch: progress.history_patch.as_ref().map(encode_wire_serialized).unwrap_or_default(),
            messages: Vec::new(),
            mutations: Vec::new(),
            inverse_group: Vec::new(),
        })));
        leftover = progress.leftover;
    }
    app.publish_local_interaction_query_reply(&mut output.frames, 4);
    Ok((output, leftover))
}
fn advance_typed_operation_output<PA: PluginApp>(app: &mut PA, instance: u32, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginExchangeOutput, crate::PluginLifecycleFailure> {
    advance_typed_operation_output_with_leftover(app, instance, identity,cx).map(|(output, _)| output)
}
/// 🕹️ How many extra typed-operation units `plugin_exchange` drives for the operation a command
/// of that exchange started, so the operation's inline interaction fold (and its leftover
/// `InteractionView`) lands inside the command's own reply. A pick with no durable lane folds
/// within a handful of units (worker step, install, fold); one with a document lane presents a
/// result page first and stops this drive on it.
const TYPED_OPERATION_REPLY_LEFTOVER_PUMPS: usize = 16;
/// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — merges a typed operation's inline-folded
/// leftover `InteractionView` into the `Invocation` reply of the command that STARTED that
/// operation, when both are in the same exchange: the reply's `output` is the operation's
/// `{ operationId, generation }` witness, and it becomes `{ operationId, generation,
/// interactionView }` — exactly the envelope `interactionViewFromLeftoverOutput` (`🛠️ShellHelpers`)
/// peels from `response.output` in `applyLeftoverInteractionView`, so the host's leftover
/// selection overlay, Inspection tab and `INTERACTION_STATE_OBSERVED` land for a folded verb the
/// same way they landed when the verb took a host round trip. A reply that answers a different
/// operation, or no typed operation at all, is left untouched.
fn merge_typed_operation_leftover_into_reply(frames: &mut [protocol::AppFrame], leftover: &TypedOperationLeftover) -> bool {
    let operation = leftover.operation.to_string();
    for frame in frames.iter_mut() {
        let protocol::AppFrame::Invocation { in_reply_to, output, .. } = frame else { continue };
        if *in_reply_to == 0 || output.is_empty() {
            continue;
        }
        let Ok(DslValue::Object(mut entries)) = store::pack_rt::decode_wire_value(output) else { continue };
        if !entries.iter().any(|(key, value)| key == "operationId" && value.as_str() == Some(operation.as_str())) {
            continue;
        }
        let DslValue::Object(envelope) = &leftover.view else { return false };
        entries.retain(|(key, _)| !envelope.iter().any(|(leftover_key, _)| leftover_key == key));
        entries.extend(envelope.iter().cloned());
        *output = store::pack_rt::encode_wire_value(&DslValue::Object(entries));
        return true;
    }
    false
}
/// 🔁️ What a typed-operation scan found: a runnable instance, or only a busy one.
///
/// A busy instance is NOT runnable work — it is another owner holding the same lock — and the two
/// used to fold into one `bool` that the reactor turn reported as `MoreWork` either way. An idle
/// app that answers `MoreWork` because a mounted worker session held the lock for a microsecond
/// costs the host a whole turn round trip for nothing, so the turn now weighs them separately.
/// See `📓️idle-turns-2026-09-10.md`.
#[derive(Clone, Copy, Default)]
pub struct TypedOperationScan {
    pub runnable: bool,
    pub contended: bool,
}
/// ⏱️ Wall milliseconds one turn may spend driving typed-operation units — the reactor executor's
/// own slice, so the continuation phase and the executor phase cost an interactive turn the same.
pub const TYPED_OPERATION_SLICE_MS: u64 = 8;
/// 🔁️ Units one turn may drive before it hands the host its turn back regardless of the clock.
/// A guest whose host clock is missing still terminates; 256 covers every staged work this
/// runtime admits (the heaviest measured is a 5.7 KB `registerBrushMesh` at ~84).
pub const TYPED_OPERATION_UNITS_PER_TURN: u32 = 256;
/// 📄️ Result pages ONE turn may hand the host, and therefore the widest acknowledgement batch the
/// shell posts back in one crossing.
///
/// 🐛️ A turn used to carry EXACTLY ONE page: [`advance_typed_operation_output`] took a single page
/// and [`TypedOperationGrant::spent`] ended the drive the instant it existed, so every
/// acknowledgeable publication unit of every live operation cost its own host round trip. Measured
/// on the React door of 🧊️generation3d (`📓️shard-message-crossings-2026-09-15.md` §2): **200 ack
/// crossings over 69 operations, every single one carrying exactly ONE ack** — 10.0 of a
/// `flowEvalTick` hop's 36.2 worker crossings, each a full structured-clone round trip to retire 53
/// bytes. A page's ACK is per-page by protocol; the CROSSING that carries it never had to be.
/// `Event::Message` is a list on the wire and the reactor's poll already acknowledges every entry of
/// it, so N presentable pages retire in ONE crossing — the same cardinality change
/// `📓️ui-turn-patch-batching-2026-09-15.md` made for `patch-ack`.
///
/// Each operation still retains exactly one unacknowledged page (`queue_page`), so this bound is
/// over DISTINCT live operations, never over one operation's own sequence.
pub const TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM: usize = 8;
/// 🔁️ What one turn may spend continuing typed operations, and what stops it early.
///
/// [`plugin_continue_typed_operations`] advanced EXACTLY ONE publication unit per call and the
/// reactor calls it once per turn, so every unit of a staged operation cost the host a whole turn
/// round trip. A trivial `compositeEdit` reached its terminal result in 48 turns
/// (`retained_operation_continues_after_command_admission_until_publication_and_retirement`) and a
/// 5.7 KB `registerBrushMesh` in ~84 (`📓️2026-09-12-wave-B22-brush-mesh-upload.md` §1.3). A turn
/// now drives units until one of four bounds, and every one of them is the protocol's own rather
/// than a pacing constant: a result page the host must acknowledge before the operation may
/// continue, the turn's declared frame and effect capacity, [`Self::deadline`], and [`Self::units`].
/// Ticket 26/09/02 wave B24.
#[derive(Clone, Copy)]
pub struct TypedOperationGrant {
    pub units: u32,
    pub max_frames: usize,
    pub max_effects: usize,
    pub deadline_us: Option<u64>,
    pub clock:fn()->Option<u64>,
}
impl TypedOperationGrant {
    /// 🔁️ One unit and nothing more — for a law that steps the continuation deliberately.
    #[cfg(test)]
    pub const UNIT: Self = Self { units: 1, max_frames: usize::MAX, max_effects: usize::MAX, deadline_us: None,clock:semio_framework_job::logical_now_us };
    /// 🔁️ The grant a turn's own [`semio_framework::kernel::Budget`] declares.
    pub fn turn(budget: semio_framework::kernel::Budget,cx:&semio_framework_job::StepContext<'_>) -> Self {
        Self {
            units: TYPED_OPERATION_UNITS_PER_TURN.min(u32::try_from(cx.fuel_remaining()).unwrap_or(u32::MAX)),
            max_frames: budget.max_frames as usize,
            max_effects: budget.max_effects as usize,
            deadline_us:Some(cx.deadline_us()),
            clock:cx.clock_source(),
        }
    }
    /// 🛑️ Whether the collected output has reached what one turn may carry. An EMPTY collection
    /// never counts as reached, so a budget declaring zero frames still drives units rather than
    /// silently restoring the one-unit-per-turn pacing this grant exists to remove.
    fn spent(&self, output: &PluginExchangeOutput) -> bool {
        output.original_receiving.is_some() || output.constructor_receipt.is_some() || output.preparation_receipt.is_some() || output.retirement_receipt.is_some() || output.history_command_receipt.is_some() || output.tool_run_receipt.is_some() || output.typed_operation_results.len() >= TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM || (!output.frames.is_empty() && output.frames.len() >= self.max_frames) || (!output.effects.is_empty() && output.effects.len() >= self.max_effects)
    }
    fn expired(&self) -> bool {
        self.deadline_us.is_some_and(|deadline| (self.clock)().is_none_or(|now|now>=deadline))
    }
}
fn pending_typed_operation_instance<PA: PluginApp>(runtime: &PluginRuntime<PA>, start: usize) -> Result<(Option<(usize, u32)>, TypedOperationScan), Fault> {
    let instances = runtime.instances.try_borrow().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?;
    let mut cursor = start;
    let mut first = None;
    let mut contended = false;
    for _ in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS {
        let Some((index, id, cell)) = instances.next_entry_from(cursor) else { break };
        if first == Some(index) {
            break;
        }
        first.get_or_insert(index);
        cursor = (index + 1) % PLUGIN_RUNTIME_INSTANCE_SLOTS;
        if runtime.guest_lifetimes.try_borrow().map_err(|_| plugin_internal_fault("lifecycle authority busy"))?.get(id).is_some_and(|slot| !slot.cell.is_live()) {
            continue;
        }
        match cell.instance.try_lock() {
            Ok(instance) if instance.app.has_runnable_typed_operations() => return Ok((Some((index, id)), TypedOperationScan { runnable: true, contended })),
            Ok(_) => {}
            Err(std::sync::TryLockError::WouldBlock) => contended = true,
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(plugin_internal_fault(format!("runtime operation authority is poisoned for instance {id}"))),
        }
    }
    Ok((None, TypedOperationScan { runnable: false, contended }))
}
/// 📮️ Publishes admitted operations without requiring another user command — as many units of ONE
/// instance as [`TypedOperationGrant`] allows, so a staged work costs the host one round trip per
/// acknowledgeable result page instead of one per unit. Units of a SECOND instance are left to the
/// next turn: the round-robin cursor is the fairness authority and one turn's output carries one
/// receiver.
pub async fn plugin_continue_typed_operations<PA: PluginApp>(runtime: &PluginRuntime<PA>, grant: TypedOperationGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(Option<(u32, PluginExchangeOutput)>, TypedOperationScan), crate::PluginLifecycleFailure> {
    let mut driven: Option<(u32, PluginExchangeOutput)> = None;
    let mut units = 0;
    loop {
        let (next, scan) = pending_typed_operation_instance(runtime, runtime.typed_continuation_cursor.get())?;
        let Some((index, instance)) = next else { return Ok((driven, scan)) };
        if units >= grant.units || driven.as_ref().is_some_and(|(owner, _)| *owner != instance) || (units > 0 && grant.expired()) {
            return Ok((driven, scan));
        }
        runtime.typed_continuation_cursor.set((index + 1) % PLUGIN_RUNTIME_INSTANCE_SLOTS);
        let cell = runtime_instance_cell(runtime, instance)?;
        let output = match cell.instance.try_lock() {
            Ok(mut active) => advance_typed_operation_output(&mut active.app, instance, identity,cx)?,
            Err(std::sync::TryLockError::WouldBlock) => return Ok((driven, TypedOperationScan { runnable: false, contended: true })),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(plugin_internal_fault(format!("runtime operation authority is poisoned for instance {instance}")).into()),
        };
        units += 1;
        let collected = driven.get_or_insert((instance, PluginExchangeOutput::default()));
        assert!(collected.1.preparation_receipt.is_none()&&collected.1.retirement_receipt.is_none()&&collected.1.history_command_receipt.is_none()&&collected.1.tool_run_receipt.is_none()&&collected.1.constructor_receipt.is_none(),"an original receipt stops before another publication unit");
        collected.1.original_receiving=output.original_receiving;
        collected.1.constructor_receipt=output.constructor_receipt;
        collected.1.preparation_receipt=output.preparation_receipt;
        collected.1.retirement_receipt=output.retirement_receipt;
        collected.1.history_command_receipt=output.history_command_receipt;
        collected.1.tool_run_receipt=output.tool_run_receipt;
        collected.1.frames.extend(output.frames);
        collected.1.effects.extend(output.effects);
        collected.1.events.extend(output.events);
        collected.1.typed_operation_results.extend(output.typed_operation_results);
        if grant.spent(&collected.1) {
            let (_, scan) = pending_typed_operation_instance(runtime, runtime.typed_continuation_cursor.get())?;
            return Ok((driven, scan));
        }
    }
}
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): erased `AsyncTask` resolution
/// input for `plugin_resume_task` — mirrors `⚛️reactor`'s own (private) `TaskResumeOutcome`
/// minus its `Fault` case (a task that itself resolved `Err` never reaches `plugin_runtime` at
/// all — `⚛️reactor::drain_task_resumes` frames that straight to the shell). Kept as a SEPARATE
/// type, not a shared one, so the dependency direction stays what it already is everywhere else
/// in this file: `plugin_runtime` never imports `reactor`; `reactor` imports `plugin_runtime`.
pub enum TaskResumeInput {
    Command(Vec<u8>),
    Emit { artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8> },
}
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumes ONE `AsyncTask`
/// follow-up against `instance_id`'s live `dyn PluginApp` — `Command` re-enters through
/// `PluginApp::resume_task_command` (the SAME APPLYING typed-command path a fresh command uses,
/// against CURRENT state — Elm's Msg-from-Cmd), `Emit` through `PluginApp::resume_task_emit`
/// (decode + `dispatch_emit` directly, no re-entry into `handle`). Frames the result EXACTLY
/// like `AppCommand::PureCommand` does (`AppFrame::Emit`, carrying the SAME
/// `take_last_emit_wire()` triple `dispatch_emit` always captures) rather than
/// `AppFrame::Invocation` — a task follow-up is a guest-applied store-authority mutation, not a
/// live client's reply; `in_reply_to: 0` marks it unsolicited (no seq is awaiting it). An
/// app-level `Err` becomes an `Error` frame, exactly like every other dispatch path in this
/// module — this function itself never fails. Returns the SAME `(frames, effects, events)`
/// shape `PluginExchangeOutput` carries so the caller (`⚛️reactor::drain_task_resumes`) feeds
/// `frames` through the SAME `route_app_frame` a live turn's command frames go through.
///
/// 🌉️ Rewritten from a `.map(protocol::encode_app_frame).collect()` into an explicit loop
/// (sync — `Iterator::map` cannot take an async closure, and `encode_app_frame` is genuinely async).
pub async fn plugin_resume_task<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, meta: &ActionMeta, input: TaskResumeInput, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> PluginExchangeOutput {
    let mut frames: Vec<protocol::AppFrame> = Vec::new();
    let mut effect_bytes: Vec<Vec<u8>> = Vec::new();
    let mut event_bytes: Vec<Vec<u8>> = Vec::new();
    let dispatched = match input {
        TaskResumeInput::Command(bytes) => {
            with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                ::semio_framework_async::poll::resolve_ready(instance.app.resume_task_command(bytes, meta, identity, cx))
            })
            .await
        }
        TaskResumeInput::Emit { artifact_ops, config_ops, draft_ops } => {
            with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                ::semio_framework_async::poll::resolve_ready(instance.app.resume_task_emit(artifact_ops, config_ops, draft_ops, meta, identity, cx))
            })
            .await
        }
    };
    match dispatched {
        Ok(result) => {
            let emit_wire = with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                Ok(::semio_framework_async::poll::resolve_ready(instance.app.take_last_emit_wire()).unwrap_or_default())
            })
            .await
            .unwrap_or_default();
            let EmitWire { document: document_ops, config: config_ops, draft: draft_ops, children: child_ops } = emit_wire;
            frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics), child_ops });
            push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
        }
        Err(fault) => push_app_fault(&mut frames, None, fault).await,
    }
    let mut frame_bytes = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        frame_bytes.push(protocol::encode_app_frame(frame).await);
    }
    PluginExchangeOutput {
        original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
        frames: frame_bytes,
        effects: effect_bytes,
        events: event_bytes,
        retry_command: None,
        command_terminal_fault: None,
        presence_pending: None,
        presence_terminal: None,
        presence_terminal_fault: None,
        typed_operation_results: Vec::new(),
    }
}
/// 🔁️ Dispatches one extension invocation's OUTCOME back into the instance that asked for it —
/// `⚛️reactor`'s `Event::Completed` continuation branch is this function's only caller. `action`
/// is the `response_action` the app declared on its `Emit::extension_invocations` entry and is
/// routed through the SAME owner-qualified `PluginApp::handle_command` path a live client's
/// command frame takes (kind discipline, tool proofs, the command log and undo grouping all
/// apply unchanged) — there is no second dispatch path to keep in sync. The address is
/// reconstructed rather than carried: `plugin_id` from the loaded program, `app_id` from the
/// live instance itself, so an app can never name a foreign owner through a response action.
/// Frames the result EXACTLY like `plugin_resume_task` (`AppFrame::Emit`/`Error`, `in_reply_to:
/// 0`): a continuation is guest-applied, unsolicited by any client `seq`.
///
/// 🪟️ A continuation carries the instance's LAST HOST VIEW, exactly like `pending_effects`
/// does. A response action is a window-addressed route whenever the request it answers was one
/// — `reactor::extension_response_args` echoes the request's own `windowId`/`windowKindId` back
/// onto it — and `ArtifactApp::retained_window_transient_target` validates that address against
/// the trusted roster. Dispatching it with no view made every `flowEvalResolve` fault
/// `targeted window transient capture requires an exact ViewModel roster`, so no extension
/// answer ever folded back and the preview never converged
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub async fn plugin_dispatch_response_action<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<PluginExchangeOutput>,semio_framework_job::OriginalTurnFailure> {
    let mut frames: Vec<protocol::AppFrame> = Vec::new();
    let mut effect_bytes: Vec<Vec<u8>> = Vec::new();
    let mut event_bytes: Vec<Vec<u8>> = Vec::new();
    let view_state = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(instance.surface_contexts.view().cloned())
    })
    .await
    .ok()
    .flatten();
    let actor = match instance_actor(runtime, instance_id).await {
        Ok(actor) => actor.into(),
        Err(fault) => {
            push_app_fault(&mut frames, None, fault).await;
            let mut output = PluginExchangeOutput::default();
            for frame in &frames { output.frames.push(protocol::encode_app_frame(frame).await); }
            return Ok(Some(output));
        }
    };
    let meta = ActionMeta { actor, instance_id, view_state };
    let dispatched = with_instances_turn_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.handle_response_action(action, args, original_fault, &meta, identity, cx))
    })
    .await;
    match dispatched {
        Ok(None)=>return Ok(None),
        Ok(Some(result)) => {
            let emit_wire = with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                Ok(::semio_framework_async::poll::resolve_ready(instance.app.take_last_emit_wire()).unwrap_or_default())
            })
            .await
            .unwrap_or_default();
            let EmitWire { document: document_ops, config: config_ops, draft: draft_ops, children: child_ops } = emit_wire;
            frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics), child_ops });
            push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
        }
        Err(failure) => return Err(failure),
    }
    let mut frame_bytes = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        frame_bytes.push(protocol::encode_app_frame(frame).await);
    }
    Ok(Some(PluginExchangeOutput {
        original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
        frames: frame_bytes,
        effects: effect_bytes,
        events: event_bytes,
        retry_command: None,
        command_terminal_fault: None,
        presence_pending: None,
        presence_terminal: None,
        presence_terminal_fault: None,
        typed_operation_results: Vec::new(),
    }))
}
/// 🕰️ Host-driven completion for one framework-reserved spawn-job: the job's output joins the app's
/// reserved-commit queue and one typed-operation continuation unit runs at once. A commit that
/// needs more units (composed children, revert walks) stays runnable and the reactor's
/// continuation finishes it on this and later turns. A job the app never admitted is a no-op so
/// fill and other isolated jobs keep their existing JobCompleted path.
pub async fn plugin_complete_reserved_spawned_job<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, job: u64, output: Result<Vec<u8>, Fault>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginExchangeOutput,crate::PluginLifecycleFailure> {
    let admitted = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.admit_reserved_spawned_job(job, output)
    })
    .await;
    match admitted {
        Ok(false) => Ok(PluginExchangeOutput::default()),
        Ok(true) => {
            with_instances_original_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                advance_typed_operation_output(&mut instance.app, instance_id, identity,cx)
            })
            .await
        }
        Err(fault) => Err(fault.into()),
    }
}
/// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): dispatches every `intents` entry for
/// `instance_id` against the app's live `dyn PluginApp` via `PluginApp::handle_intent_frame` —
/// the ONLY dispatch path this takes, which resolves a typed `Command` through
/// `ArtifactApp::command_from_intent` and routes it through the SAME internal
/// `dispatch_typed_command_inner` every other command path uses (kind discipline, command log,
/// undo grouping, `AsyncTask` spawning all apply automatically). `intents` is ALREADY
/// revision-guarded by the caller (`⚛️reactor::poll`'s `PatchTracker::revision` +
/// `ui_runtime::is_stale_intent`, at the reactor because it owns the reconciler that owns the
/// revision) — this function never re-checks staleness, matching `plugin_exchange`'s own division
/// of labor (host-level routing vs. app-level dispatch).
///
/// Frames the result EXACTLY like `plugin_resume_task` (`AppFrame::Emit`/`Error`, `in_reply_to:
/// 0`): an intent is guest-applied, not a live client's `Command`-frame reply awaiting a specific
/// `seq` — the reply IS the next `UiPatch` revision bump the caller's own `dirty_render` pass
/// produces this same turn, already acknowledged by the existing `PatchAck` event; `intent.seq`
/// is the dedupe/log key a caller may record, not an `in_reply_to`. A per-intent `Fault` (e.g. an
/// `ActionId.version` mismatch, or a `View`/`🐚️Shell`-kind action returning operations) rides
/// `AppFrame::Error` — never silently dropped, never a hard stop for the remaining intents in the
/// same batch.
pub async fn plugin_dispatch_intents<PA: PluginApp, const N: usize>(runtime: &PluginRuntime<PA>, instance_id: u32, intents: &ui_contract::UiFixedList<ui_contract::UiIntent, N>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<PluginExchangeOutput>,semio_framework_job::OriginalTurnFailure> {
    let mut frames: Vec<protocol::AppFrame> = Vec::new();
    let mut effect_bytes: Vec<Vec<u8>> = Vec::new();
    let mut event_bytes: Vec<Vec<u8>> = Vec::new();
    let actor: semio_framework_value::SharedUtf8 = instance_actor(runtime, instance_id).await?.into();
    for intent in intents {
        let dispatched = with_instances_turn_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            let context = instance.surface_contexts.get(intent.surface.as_ref()).ok_or_else(|| plugin_internal_fault("intent surface has no host context"))?;
            let meta = ActionMeta { actor: actor.clone(), instance_id, view_state: Some(context.view_state) };
            ::semio_framework_async::poll::resolve_ready(instance.app.handle_intent_frame(intent, &meta, identity, cx))
        })
        .await;
        match dispatched {
            Ok(None)=>return Ok(None),
            Ok(Some(result)) => {
                let emit_wire = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    Ok(::semio_framework_async::poll::resolve_ready(instance.app.take_last_emit_wire()).unwrap_or_default())
                })
                .await
                .unwrap_or_default();
                let EmitWire { document: document_ops, config: config_ops, draft: draft_ops, children: child_ops } = emit_wire;
                frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics), child_ops });
                push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
            }
            Err(failure) => return Err(failure),
        }
    }
    let mut frame_bytes = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        frame_bytes.push(protocol::encode_app_frame(frame).await);
    }
    Ok(Some(PluginExchangeOutput {
        original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
        frames: frame_bytes,
        effects: effect_bytes,
        events: event_bytes,
        retry_command: None,
        command_terminal_fault: None,
        presence_pending: None,
        presence_terminal: None,
        presence_terminal_fault: None,
        typed_operation_results: Vec::new(),
    }))
}
pub async fn plugin_reserve_presence_ingress<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, seq: u64) -> Result<PresenceRosterAdmission, Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.reserve_presence_ingress(seq)
    })
    .await
}
#[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
pub async fn plugin_admit_reserved_presence<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    admission: PresenceRosterAdmission,
    own_color: Option<u8>,
    item_count: u32,
    page: semio_framework::kernel::FixedCommandPage,
    now_ms: i64,
) -> Result<u64, (Fault, PresenceRosterAdmission, semio_framework::kernel::FixedCommandPage)> {
    let instances = match runtime.instances.try_borrow_mut() {
        Ok(instances) => instances,
        Err(_) => return Err((plugin_internal_fault("runtime instance authority is busy"), admission, page)),
    };
    let mut instance = match find_instance(&instances, instance_id) {
        Ok(instance) => instance,
        Err(fault) => return Err((fault, admission, page)),
    };
    let generation = admission.generation();
    let cursor = match protocol::PresenceCommandCursor::admit_page(admission.sequence(), own_color, item_count, page) {
        Ok(cursor) => cursor,
        Err((error, rejected)) => {
            instance.app.reject_presence_ingress(admission, rejected, error.into_fault());
            return Ok(generation);
        }
    };
    instance.app.admit_presence_ingress(admission, cursor, now_ms);
    Ok(generation)
}
#[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
pub async fn plugin_push_reserved_presence_page<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    generation: u64,
    page_index: u32,
    page: semio_framework::kernel::FixedCommandPage,
) -> Result<(), (Fault, semio_framework::kernel::FixedCommandPage)> {
    let instances = match runtime.instances.try_borrow_mut() {
        Ok(instances) => instances,
        Err(_) => return Err((plugin_internal_fault("runtime instance authority is busy"), page)),
    };
    let mut instance = match find_instance(&instances, instance_id) {
        Ok(instance) => instance,
        Err(fault) => return Err((fault, page)),
    };
    instance.app.push_presence_ingress(generation, page_index, page)
}
#[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
pub async fn plugin_reject_reserved_presence<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    admission: PresenceRosterAdmission,
    command: semio_framework::kernel::FixedCommandPage,
    fault: Fault,
) -> Result<(), (Fault, semio_framework::kernel::FixedCommandPage)> {
    let instances = match runtime.instances.try_borrow_mut() {
        Ok(instances) => instances,
        Err(_) => return Err((plugin_internal_fault("runtime instance authority is busy"), command)),
    };
    let mut instance = match find_instance(&instances, instance_id) {
        Ok(instance) => instance,
        Err(error) => return Err((error, command)),
    };
    instance.app.reject_presence_ingress(admission, command, fault);
    Ok(())
}
/// 🔀️ The single bidirectional entry point behind WIT `exchange` (see `📜️wit/📜️world.wit`'s
/// `interface plugin` doc) — decodes each `protocol::AppCommand` in `commands`, dispatches it
/// against `instance_id`, and returns every produced `protocol::AppFrame`/effect/event.
///
/// 🚧️ Wave 1 scope — documented here rather than silently: `AppCommand::CommandText` is stubbed
/// (`Error{code:"unsupported"}`, a later wave wires real headless op-text scripts);
/// `AppCommand::ArtifactCommand` only accepts the six history verbs (`DOCUMENT_COMMAND_ACTION_IDS`),
/// mapped onto the existing magic action-name interception rather than a real typed
/// `store::os_store::store_schema::ArtifactCommand` wire codec; the unsolicited outbox (backbone-driven
/// `AppFrame::DocumentChanged`, a persistent per-instance frame queue surviving across calls) is
/// NOT implemented — only `pending_effects` is drained, once, at the end of the batch, whenever a
/// dispatched command mutated the document, mirroring exactly where `plugin_refresh_ui` already
/// calls it.
///
/// 🧬️ Channel v12 (A4) retired `AppCommand::{Hello, RefreshUi, AttachBackbone, DetachBackbone,
/// Bye}` — lifecycle is now `Event::InstanceOpen`/`InstanceClose` at the reactor level, surface
/// rendering is driven by `surface-visible`/`hidden` instead of `RefreshUi` probes, and the
/// backbone attach/detach commands are gone with no replacement command in this wave. Those arms
/// are deleted here rather than adapted (design-abi.md §4).
pub async fn plugin_acknowledge_typed_operation_result<PA: PluginApp>(runtime: &PluginRuntime<PA>, token: TypedOperationResultToken) -> Result<bool, Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, token.receiver)?;
        instance.app.acknowledge_typed_operation_result(token)
    })
    .await
}
/// 🏃️ Drives a self-waking guest future to completion inside one wasm poll. `resolve_ready`
/// samples once; framework-reserved routes yield through [`plugin_job_yield_once`] before their
/// commit, so a single poll would drop the job and the host would see `command-complete` without
/// [`commit_framework_history_route`].
pub fn drive_self_waking_ready<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, Waker};
    let waker = Waker::noop();
    let mut cx = Context::from_waker(&waker);
    let mut fut = std::pin::pin!(fut);
    for _ in 0..1_048_576 {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => continue,
        }
    }
    panic!("drive_self_waking_ready: exceeded the self-wake bound");
}
fn media_export_handle_from_wire(handle: protocol::MediaExportHandleWire) -> ArtifactMediaExportHandle {
    ArtifactMediaExportHandle {
        app_instance_id: handle.app_instance_id,
        parent_document_id: handle.parent_document_id,
        operation_id: semio_framework_job::OperationId(handle.operation_id),
        base_revision: semio_framework_job::RevisionId(handle.base_revision),
        generation: semio_framework_job::Generation(handle.generation),
    }
}
fn media_export_handle_into_wire(handle:ArtifactMediaExportHandle)->protocol::MediaExportHandleWire{protocol::MediaExportHandleWire{app_instance_id:handle.app_instance_id,parent_document_id:handle.parent_document_id,operation_id:handle.operation_id.0,base_revision:handle.base_revision.0,generation:handle.generation.0}}
/// 🕹️ The typed operation an `AppCommand::Command` of this exchange STARTED (its reply's
/// `output.operationId`) — the operation whose inline interaction fold this exchange still
/// tries to land before the reply is sealed (`TYPED_OPERATION_REPLY_LEFTOVER_PUMPS`).
///
/// 🧮️ B1: `command` is a binary-encoded `store::os_store::store_schema::ArtifactCommand<A::ConfigMutation>` —
/// real dispatch against the config store (replaces the deleted `apply_config_bytes`
/// whole-record-replace legacy path); undo/redo/checkpoint all work on config now.
///
/// 🔀️ Contract §5.1: a gesture whose mutations carried foreign steps
/// applied nothing (`dispatch_emit` stashed a proposal instead) — frame
/// `AppFrame::TransactionProposal` in place of the ordinary
/// `Invocation` frame whenever one is pending.
///
/// 🚧️ Wave 1 stub — see this function's doc comment.
///
/// 🗂️ Decodes straight into the typed wire shape and encodes the typed response straight
/// back out — no intermediate `Value`/JSON-string hop through `plugin_context_menu`
/// (which stays as the separate string-in/string-out entry point the WIT boundary needs).
///
/// 🔀️ Same proposal check as `AppCommand::Command` above — the six
/// history verbs never carry foreign steps in practice, but
/// `dispatch_emit` is the shared choke point, so this arm is covered too.
///
/// ⚔️ Pushed unsolicited after every ingest, next to `DocumentChanged`
/// (contract §C8): one `MergeReport` per ingested envelope, then the
/// authoritative open-conflict projection once the whole batch has landed.
///
/// 🧸️ Composed children are their own envelopes with their own histories, so they
/// need their own load/read pair — a parent's own pack+spr (`AppFrame::Document`) carries none
/// of them, and before this existed a genesis child lived only until the process
/// ended.
///
/// 🌉️ `with_instances_mut`'s callback is deliberately sync (single-threaded
/// `RefCell::borrow_mut` reentrancy guard, see its own doc comment) —
/// bridged via `resolve_ready` exactly like `media_fingerprint` above.
///
/// 🧮️ B1: real work — loads straight into the config `ArtifactStore` (was routed
/// through the deleted `apply_config_bytes` whole-record-replace legacy path).
///
/// 🧮️ B1: real work — the config store's current pack+spr+ops (was a stub always
/// returning empty bytes).
///
/// 🔀️ Member state machine (contract §5.3-§5.10) — see `📓️w1-b-report.md` for the
/// full frame/rejection-code table.
///
/// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the operation this command started owes
/// its inline interaction fold's leftover `InteractionView` to THIS reply (`output` is the only
/// lane `applyLeftoverInteractionView` reads for a typed command), so the ladder is driven a
/// few more bounded units here — exactly the units the reactor's own continuation would drive
/// next — until that fold lands, a result page is presented (the host must ACK it first),
/// nothing is runnable, or the bound is spent. A fold that lands later still rides the `Ui`
/// progress frame's `output`.
///
/// 🌉️ Rewritten from a `.map(protocol::encode_app_frame).collect()` into an explicit loop
/// (sync — `Iterator::map` cannot take an async closure, and `encode_app_frame` is genuinely async).
pub async fn plugin_exchange<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, command: Option<(u64, PluginCommandIngress)>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginExchangeOutput, crate::PluginLifecycleFailure> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    debug_runtime_line(format_args!("[TRACE] plugin_exchange entry instance={instance_id} command={}", command.is_some()));
    let constructing = with_instances_mut(runtime, |list| { let instance = find_instance(list, instance_id)?; Ok(instance.app.store_constructor_is_pending()) }).await?;
    if constructing {
        let mut output = with_instances_original_mut(runtime, |list| { let mut instance = find_instance(list, instance_id)?; advance_typed_operation_output(&mut instance.app, instance_id, identity,cx) }).await?;
        output.retry_command = command;
        return Ok(output);
    }
    let mut frames: Vec<protocol::AppFrame> = Vec::new();
    let mut effect_bytes: Vec<Vec<u8>> = Vec::new();
    let mut event_bytes: Vec<Vec<u8>> = Vec::new();
    let retry_command = None;
    let mut command_terminal_fault = None;
    let presence_pending = None;
    let mut presence_terminal = None;
    let mut presence_terminal_fault = None;
    let mut mutated = false;
    let mut started_typed_operation: Option<u64> = None;
    let command = match command {
        Some((envelope_seq, ingress)) => match ingress.advance_original(COMMAND_INGRESS_MOVES_PER_TURN,cx) {
            PluginCommandIngressStep::Pending(ingress) => {
                return Ok(PluginExchangeOutput {
                    original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
                    frames: Vec::new(),
                    effects: effect_bytes,
                    events: event_bytes,
                    retry_command: Some((envelope_seq, ingress)),
                    command_terminal_fault,
                    presence_pending,
                    presence_terminal,
                    presence_terminal_fault,
                    typed_operation_results: Vec::new(),
                });
            }
            PluginCommandIngressStep::Ready(owner) => Some((envelope_seq, owner)),
            PluginCommandIngressStep::TerminalOriginalFailure(failure)=>return Err(failure.into()),
            PluginCommandIngressStep::TerminalFault(fault) => {
                command_terminal_fault = Some(semio_framework_diagnostic::encode_fault_bytes(&fault));
                push_app_fault(&mut frames, Some(envelope_seq), fault).await;
                return Ok(PluginExchangeOutput {
                    original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
                    frames: encode_exchange_frames(&frames).await,
                    effects: effect_bytes,
                    events: event_bytes,
                    retry_command,
                    command_terminal_fault,
                    presence_pending,
                    presence_terminal,
                    presence_terminal_fault,
                    typed_operation_results: Vec::new(),
                });
            }
        },
        None => None,
    };
    let presence_outcome = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        Ok(instance.app.take_presence_outcome())
    })
    .await;
    let presence_outcome = match presence_outcome {
        Ok(outcome) => outcome,
        Err(fault) => {
            if let Some((envelope_seq, owner)) = command {
                let ingress = PluginCommandIngress::Decoded(owner);
                let ingress = if fault.message == "runtime instance authority is busy" { ingress } else { ingress.cancel(fault) };
                return Ok(PluginExchangeOutput {
                    original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
                    frames: Vec::new(),
                    effects: effect_bytes,
                    events: event_bytes,
                    retry_command: Some((envelope_seq, ingress)),
                    command_terminal_fault,
                    presence_pending,
                    presence_terminal,
                    presence_terminal_fault,
                    typed_operation_results: Vec::new(),
                });
            }
            return Err(fault.into());
        }
    };
    if let Some(outcome) = presence_outcome {
        presence_terminal = Some(outcome.seq);
        match outcome.fault {
            Some(fault) => {
                presence_terminal_fault = Some(semio_framework_diagnostic::encode_fault_bytes(&fault));
                push_app_fault(&mut frames, Some(outcome.seq), fault).await;
            }
            None => frames.push(protocol::AppFrame::Done { in_reply_to: outcome.seq }),
        }
    }
    if let Some((envelope_seq, mut owner)) = command {
        let manifest_result=if owner.command_seq.is_some(){
            match owner.advance_manifest_step(runtime,instance_id,identity,cx).await{
                Ok(Some(result))=>Some(result),
                result=>{let ingress=match result{Ok(None)=>PluginCommandIngress::Decoded(owner),Err(failure)=>{let identity=owner.receiving.as_ref().map_or((cx.operation(),cx.generation()),|owner|owner.identity());PluginCommandIngress::FailedDecoded{owner,failure:Some(failure),identity,close_failure:None,compound:None,raw_media:None}},Ok(Some(_))=>unreachable!()};return Ok(PluginExchangeOutput{frames:encode_exchange_frames(&frames).await,effects:effect_bytes,events:event_bytes,retry_command:Some((envelope_seq,ingress)),command_terminal_fault,presence_pending,presence_terminal,presence_terminal_fault,..Default::default()})}
            }
        }else{None};
        if let Some((seq,result))=manifest_result{
                        let (proposal, report) = with_instances_mut(runtime, |list| {
                            let mut instance = find_instance(list, instance_id)?;
                            Ok((::semio_framework_async::poll::resolve_ready(instance.app.take_pending_transaction_proposal()), ::semio_framework_async::poll::resolve_ready(instance.app.dispatch_report()).clone()))
                        })
                        .await?;
                        if let Some(proposal) = proposal {
                            frames.push(transaction_proposal_frame(instance_id, seq, proposal).await);
                            push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
                        } else {
                            let output = encode_wire_serialized(&result.output);
                            let diagnostics = encode_wire_serialized(&result.diagnostics);
                            let mutations = encode_wire_serialized(&result.mutations);
                            let inverse_group = encode_wire_serialized(&result.inverse_group);
                            if mutations.len() > ::replication::wire::command_ingress::INVOCATION_RESULT_PACK_MAXIMUM_BYTES || inverse_group.len() > ::replication::wire::command_ingress::INVOCATION_RESULT_PACK_MAXIMUM_BYTES {
                                push_dispatch_fault(&mut frames, seq, Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.invocation-result-capacity"), "packed mutations or inverse group exceed command transport authority"), &report).await;
                            } else {
                                mutated = true;
                                started_typed_operation = result.output.get("operationId").and_then(DslValue::as_str).and_then(|operation| operation.parse::<u64>().ok());
                                frames.push(protocol::AppFrame::Invocation {
                                    in_reply_to: seq,
                                    output,
                                    diagnostics,
                                    ui_scope: encode_wire_serialized(&result.ui_scope),
                                    history_patch: encode_wire_serialized(&result.history_patch),
                                    messages: encode_wire_serialized(&report),
                                    mutations,
                                    inverse_group,
                                });
                                push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
                            }
                        }
        }else{
        let mut original_media_submission_ready=false;
        let mut original_media_poll_ready=false;
        let mut original_media_failure:Option<semio_framework_job::OriginalTurnFailure>=None;
        let mut original_media_domain=None;
        let mut original_media_artifact=None;
        let mut original_preview=None;
        let mut original_configuration=None;
        let media_pending=match owner.original_command(){
            Some(protocol::AppCommand::MediaIn{..})=>{
                let result=with_instances_original_mut(runtime,|list|{let mut instance=find_instance(list,instance_id)?;::semio_framework_async::poll::resolve_ready(instance.app.prepare_media_import(cx)).map_err(Into::into)}).await;
                match result{Ok(ready)=>!ready,Err(failure)=>{original_media_failure=Some(failure.into());true}}
            },
            Some(protocol::AppCommand::SubmitMediaExport{..})=>{
                let(port,parent,revision,receiver)=owner.media_submission_parts()?;
                match plugin_submit_media_export(runtime,instance_id,port,parent,revision,receiver,cx).await{Ok(ready)=>original_media_submission_ready=ready,Err(failure)=>original_media_failure=Some(failure.into())};
                !original_media_submission_ready
            },
            Some(protocol::AppCommand::PollMediaExport{..})=>{
                let(loan,poll,status)=owner.media_poll_parts()?;
                if poll.is_none()&&status.is_none(){match plugin_poll_media_export(runtime,instance_id,loan,poll,cx).await{Ok(ready)=>original_media_poll_ready=ready,Err(failure)=>original_media_failure=Some(failure)}}else{original_media_poll_ready=true}
                if original_media_failure.is_none()&&original_media_poll_ready&&(status.is_some()||matches!(poll.as_ref(),Some(ArtifactMediaExportPoll::Complete(_)))){match plugin_retain_media_export_result(runtime,instance_id,loan,poll,status,cx).await{Ok(ready)=>original_media_poll_ready=ready,Err(failure)=>{original_media_failure=Some(failure);original_media_poll_ready=false}}}
                !original_media_poll_ready
            },
            Some(protocol::AppCommand::TakeMediaExportChunk{..})=>{
                let(loan,receiver,terminal)=owner.media_chunk_parts()?;match plugin_take_media_export_chunk(runtime,instance_id,loan,receiver,terminal,cx).await{Ok(ready)=>!ready,Err(failure)=>{original_media_failure=Some(failure);true}}
            },
            Some(protocol::AppCommand::MediaOut{port,..})=>{
                let grant=cx.retained_grant();let artifact=identity.encode(|native|{let mut encoder=crate::io::control::NativeSnapshotEncodeOwner::new(native,grant);let result=::semio_framework_async::poll::resolve_ready(plugin_produce_media(runtime,instance_id,port,"",&mut encoder,cx));cx.consume_retained(encoder.progress()).map_err(crate::app_contract::MediaArtifactOperationFailure::from)?;result});
                match artifact{Ok(result)=>original_media_artifact=result,Err(crate::app_contract::MediaArtifactOperationFailure::Original(failure))=>original_media_failure=Some(failure.into()),Err(crate::app_contract::MediaArtifactOperationFailure::Domain(source))=>original_media_domain=Some(source)};
                original_media_artifact.is_none()
            },
            Some(protocol::AppCommand::ConfigCommand{..})=>{
                let meta=ActionMeta{actor:instance_actor(runtime,instance_id).await?.into(),instance_id,view_state:None};let command=owner.configuration_bytes_mut()?;
                let result=with_instances_original_mut(runtime,|list|{let mut instance=find_instance(list,instance_id)?;::semio_framework_async::poll::resolve_ready(instance.app.dispatch_config_command(command,&meta,identity,cx)).map_err(Into::into)}).await;
                match result{Ok(result)=>original_configuration=result,Err(failure)=>original_media_failure=Some(failure.into())}original_configuration.is_none()
            },
            Some(protocol::AppCommand::PureCommand{command,head,..})=>{
                let meta=ActionMeta{actor:instance_actor(runtime,instance_id).await?.into(),instance_id,view_state:None};
                let result=with_instances_original_mut(runtime,|list|{let mut instance=find_instance(list,instance_id)?;if !instance.app.original_command_frame_pending()&&!::semio_framework_async::poll::resolve_ready(instance.app.hydrate_pure_head(head,cx))?{return Ok(None)}let result=::semio_framework_async::poll::resolve_ready(instance.app.handle_command_frame(command,&meta,identity,cx))?;Ok(result.map(|result|(result,::semio_framework_async::poll::resolve_ready(instance.app.take_last_emit_wire()).unwrap_or_default())))}).await;
                match result{Ok(result)=>original_preview=result,Err(failure)=>original_media_failure=Some(failure.into())};original_preview.is_none()
            },
            _=>false,
        };
        if media_pending{
            let ingress=match original_media_failure{Some(failure)=>PluginCommandIngress::FailedDecoded{owner,failure:Some(failure),identity:(cx.operation(),cx.generation()),close_failure:None,compound:None,raw_media:original_media_domain},None=>if original_media_domain.is_some(){PluginCommandIngress::FailedDecoded{owner,failure:None,identity:(cx.operation(),cx.generation()),close_failure:None,compound:None,raw_media:original_media_domain}}else{PluginCommandIngress::Decoded(owner)}};
            return Ok(PluginExchangeOutput{frames:encode_exchange_frames(&frames).await,effects:effect_bytes,events:event_bytes,retry_command:Some((envelope_seq,ingress)),command_terminal_fault,presence_pending,presence_terminal,presence_terminal_fault,..Default::default()});
        }
        let command = owner.take_for_dispatch().ok_or_else(|| plugin_internal_fault("decoded command owner is terminal before dispatch"))?;
        match command {
            protocol::AppCommand::Presence { .. } => unreachable!("Presence commands are admitted from their fixed header before whole-command decoding"),
            protocol::AppCommand::LocalInteractionQuery { seq, command } => {
                frames.try_reserve_exact(2).map_err(|_| plugin_internal_fault("local interaction receipt and rejection slots could not be admitted"))?;
                let reply = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    Ok(match command {
                        protocol::LocalInteractionQueryCommand::Read { request_id } => match runtime.local_interaction_query_generation.next() {
                            Some(generation) => instance.app.begin_local_interaction_query(request_id, generation),
                            None => Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::GenerationExhausted }),
                        },
                        protocol::LocalInteractionQueryCommand::Acknowledge { token } => {
                            instance.app.acknowledge_local_interaction_query(&token);
                            None
                        }
                        protocol::LocalInteractionQueryCommand::Cancel { token } => {
                            instance.app.cancel_local_interaction_query(&token);
                            None
                        }
                    })
                })
                .await?;
                if let Some(reply) = reply {
                    frames.push(protocol::AppFrame::LocalInteractionQuery { reply });
                }
                frames.push(protocol::AppFrame::Done { in_reply_to: seq });
            }
            protocol::AppCommand::ConfigCommand{seq,command}=>{
                assert_eq!(command.capacity(),0,"configuration native job must own the exact original raw allocation before dispatch donation");let result=original_configuration.take().expect("original configuration completes before decoded command donation");mutated=true;frames.push(protocol::AppFrame::Done{in_reply_to:seq});push_invocation_side_frames(&mut effect_bytes,&mut event_bytes,&result).await;
            }
            protocol::AppCommand::Command { .. } => unreachable!("original manifest receiver dispatches before source consumption"),
            protocol::AppCommand::CommandText { seq, line: _ } => {
                push_os_fault(&mut frames, Some(seq), "unsupported", "CommandText not yet wired".into()).await;
            }
            protocol::AppCommand::ContextMenu { seq, request } => match decode_wire_serialized::<ContextMenuWireRequest>(&request).await.and_then(ContextMenuWireRequest::into_parts) {
                Ok((request, view_state)) => {
                    let outcome = with_instances_mut(runtime, |list| {
                        let mut instance = find_instance(list, instance_id)?;
                        Ok(::semio_framework_async::poll::resolve_ready(instance.app.context_menu(&request, &view_state)))
                    });
                    match outcome.await {
                        Ok(items) => frames.push(protocol::AppFrame::ContextMenu { in_reply_to: seq, items: encode_wire_serialized(&items) }),
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                    }
                }
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::ArtifactCommand { .. } => unreachable!("original artifact receiver dispatches before source consumption"),
            protocol::AppCommand::ApplyEnvelopes { seq, envelopes } => {
                let encoded = protocol::encode_envelopes(&envelopes);
                match plugin_ingest_operations(runtime, instance_id, &encoded, identity, cx).await {
                    Ok(reports) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                        for report in &reports {
                            frames.push(protocol::AppFrame::MergeReport { in_reply_to: None, report: encode_wire_serialized(report) });
                        }
                        if !reports.is_empty() {
                            let open: Vec<protocol::Conflict> = with_instances_mut(runtime, |list| {
                                let instance = find_instance(list, instance_id)?;
                                Ok(::semio_framework_async::poll::resolve_ready(instance.app.open_conflicts()))
                            })
                            .await?;
                            frames.push(protocol::AppFrame::Conflicts { in_reply_to: None, conflicts: encode_wire_serialized(&open) });
                        }
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadDocument { seq } => match plugin_document_pack(runtime, instance_id).await {
                Ok(files) => frames.push(protocol::AppFrame::Document { in_reply_to: seq, pack: files.pack, spr: files.spr, ops: files.ops }),
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::LoadChildren { seq, entries } => {
                let loaded = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    for entry in &entries {
                        if !entry.owner.is_empty() {
                            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), format!("child pack {}/{} belongs to member {}: a member's own children load with the document archive", entry.slot, entry.child_id, entry.owner)));
                        }
                        let dialect = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&entry.dialect).map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), error))?;
                        ::semio_framework_async::poll::resolve_ready(instance.app.load_child_pack(&entry.slot, &entry.child_id, dialect, &entry.envelope_pack, identity))?;
                    }
                    Ok(())
                });
                match loaded.await {
                    Ok(()) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadChildren { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.child_packs())
                });
                match read.await {
                    Ok(entries) => frames.push(protocol::AppFrame::Children { in_reply_to: seq, entries }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadChildHeads { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.child_head_packs())
                });
                match read.await {
                    Ok(entries) => frames.push(protocol::AppFrame::ChildHeads { in_reply_to: seq, entries }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::LoadDocumentArchive { seq, archive } => {
                let admitted = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    instance.app.begin_document_archive_load(seq, archive)
                });
                match admitted.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::MergeDocumentArchive { seq, archive } => {
                let admitted = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    instance.app.begin_document_archive_merge(seq, archive)
                });
                match admitted.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::PollDocumentArchiveLoad { seq, operation } => {
                let status = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    drive_self_waking_ready(instance.app.poll_document_archive_load(operation, identity, cx))
                });
                match status.await {
                    Ok(status) => {
                        mutated |= status.state == protocol::DocumentArchiveLoadState::Ready;
                        frames.push(protocol::AppFrame::DocumentArchiveLoad { in_reply_to: seq, status });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::CancelDocumentArchiveLoad { seq, operation } => {
                let cancelled = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    instance.app.cancel_document_archive_load(operation)
                });
                match cancelled.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation } => {
                let acknowledged = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    instance.app.acknowledge_document_archive_load(operation)
                });
                match acknowledged.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadDocumentIdentity { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    Ok(protocol::AppDocumentIdentity { app_instance_id: instance_id, parent_document_id: instance.app.document_identity() })
                });
                match read.await {
                    Ok(identity) => frames.push(protocol::AppFrame::DocumentIdentity { in_reply_to: seq, identity }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadDocumentArchive { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.document_archive())
                });
                match read.await {
                    Ok(archive) => frames.push(protocol::AppFrame::DocumentArchive { in_reply_to: seq, archive }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadHistory { seq } => {
                let snapshot = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.history_snapshot())
                });
                match snapshot.await {
                    Ok(history_patch) => frames.push(protocol::AppFrame::HistorySnapshot { in_reply_to: seq, history_patch: encode_wire_serialized(&history_patch) }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::LoadConfig { seq, pack, spr } => {
                let applied = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.load_config_pack(&store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }))
                });
                match applied.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadConfig { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.config_pack())
                });
                match read.await {
                    Ok(files) => frames.push(protocol::AppFrame::Config { in_reply_to: seq, pack: files.pack, spr: files.spr, ops: files.ops }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::LoadWindowConfig { seq, entry } => match store::os_store::serialization::decode_document_pack_bytes(&entry.envelope_pack).await {
                Ok((pack, spr)) => {
                    let loaded = with_instances_mut(runtime, |list| {
                        let mut instance = find_instance(list, instance_id)?;
                        ::semio_framework_async::poll::resolve_ready(instance.app.load_window_config_pack(WindowConfigPack {
                            window_id: entry.window_id,
                            window_kind_id: entry.window_kind_id,
                            files: store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() },
                        }))
                    });
                    match loaded.await {
                        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                    }
                }
                Err(error) => push_app_fault(&mut frames, Some(seq), error.into_fault()).await,
            },
            protocol::AppCommand::ReadWindowConfigs { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.window_config_packs())
                });
                match read.await {
                    Ok(packs) => {
                        let mut entries = Vec::with_capacity(packs.len());
                        for pack in packs {
                            entries.push(protocol::WindowConfigPackEntry { window_id: pack.window_id, window_kind_id: pack.window_kind_id, envelope_pack: store::os_store::serialization::encode_document_pack_bytes(&pack.files.pack, &pack.files.spr).await });
                        }
                        frames.push(protocol::AppFrame::WindowConfigs { in_reply_to: seq, entries });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::MediaIn { seq, port, descriptor, data } => {
                let descriptor_value: DslValue = decode_wire_serialized_or(&descriptor, DslValue::Null).await;
                let descriptor_json = serde_json::to_string(&descriptor_value).unwrap_or_else(|_| "{}".into());
                match plugin_consume_media(runtime, instance_id, seq, &port, &descriptor_json, data, cx).await {
                    Ok(None) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Ok(Some(status)) => frames.push(protocol::AppFrame::DocumentArchiveLoad { in_reply_to: seq, status }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::MediaOut{seq,port,..}=>{let artifact=original_media_artifact.take().expect("original media encoding completes before command donation");frames.push(protocol::AppFrame::Media{in_reply_to:seq,port,descriptor:encode_wire_serialized(&artifact.descriptor),data:artifact.data});},
            protocol::AppCommand::SubmitMediaExport { seq, .. } => {
                let handle=owner.take_media_submission().expect("original media submission is complete before command donation");
                frames.push(protocol::AppFrame::MediaExportSubmitted{in_reply_to:seq,handle:media_export_handle_into_wire(handle)});
            },
            protocol::AppCommand::PollMediaExport { seq, handle } => {
                let handle = media_export_handle_from_wire(handle);
                if let Some(status)=owner.take_media_status(){frames.push(protocol::AppFrame::MediaExportStatus{in_reply_to:seq,handle:media_export_handle_into_wire(handle),state:protocol::MediaExportStateWire::Complete,applied_progress:status.total_bytes,checkpoint_available:false,mime_type:status.mime_type,total_bytes:status.total_bytes,detail:String::new()});}else{
                match owner.take_media_poll().expect("original media poll is retained before command donation") {
                    ArtifactMediaExportPoll::Running { applied_progress, checkpoint_available } => frames.push(protocol::AppFrame::MediaExportStatus {
                        in_reply_to: seq,
                        handle: media_export_handle_into_wire(handle),
                        state: protocol::MediaExportStateWire::Running,
                        applied_progress,
                        checkpoint_available,
                        mime_type: String::new(),
                        total_bytes: 0,
                        detail: String::new(),
                    }),
                    ArtifactMediaExportPoll::Complete(_) => unreachable!("terminal result donates only to its original download registry before decoded command consumption"),
                    ArtifactMediaExportPoll::Cancelled => frames.push(protocol::AppFrame::MediaExportStatus {
                        in_reply_to: seq,
                        handle: media_export_handle_into_wire(handle),
                        state: protocol::MediaExportStateWire::Cancelled,
                        applied_progress: 0,
                        checkpoint_available: false,
                        mime_type: String::new(),
                        total_bytes: 0,
                        detail: String::new(),
                    }),
                    ArtifactMediaExportPoll::Failed(detail) => frames.push(protocol::AppFrame::MediaExportStatus {
                        in_reply_to: seq,
                        handle: media_export_handle_into_wire(handle),
                        state: protocol::MediaExportStateWire::Failed,
                        applied_progress: 0,
                        checkpoint_available: false,
                        mime_type: String::new(),
                        total_bytes: 0,
                        detail,
                    }),
                }
            }}
            protocol::AppCommand::CancelMediaExport { seq, handle } => {
                let handle = media_export_handle_from_wire(handle);
                match plugin_cancel_media_export(runtime, instance_id, &handle).await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(error) => push_app_fault(&mut frames, Some(seq), plugin_internal_fault(error.to_string())).await,
                }
            }
            protocol::AppCommand::TakeMediaExportChunk{seq,handle}=>{let(data,terminal)=owner.take_media_chunk();frames.push(protocol::AppFrame::MediaExportChunk{in_reply_to:seq,handle,data:data.unwrap_or_default(),terminal});},
            protocol::AppCommand::MediaFingerprint { seq, port } => {
                let fingerprint = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.media_fingerprint(&port)).map_err(|error| plugin_internal_fault(error.to_string()))
                });
                match fingerprint.await {
                    Ok(fingerprint) => {
                        let value = DslValue::String(fingerprint.0);
                        frames.push(protocol::AppFrame::MediaFingerprint { in_reply_to: seq, port, fingerprint: encode_wire_serialized(&value) });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::PureCommand{seq,..}=>{let(result,EmitWire{document:document_ops,config:config_ops,draft:draft_ops,children:child_ops})=original_preview.take().expect("original preview completes before decoded command donation");mutated=true;frames.push(protocol::AppFrame::Emit{in_reply_to:seq,document_ops,config_ops,draft_ops,output:encode_wire_serialized(&result.output),diagnostics:encode_wire_serialized(&result.diagnostics),child_ops});push_invocation_side_frames(&mut effect_bytes,&mut event_bytes,&result).await;},
            protocol::AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, origin, prepared_child_ops } => {
                let decoded_origin = if origin.is_empty() { Ok(None) } else { decode_wire_serialized::<protocol::MutationOrigin>(&origin).await.map(Some) };
                match decoded_origin {
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                    Ok(decoded_origin) => {
                        let outcome = with_instances_mut(runtime, |list| {
                            let mut instance = find_instance(list, instance_id)?;
                            Ok(::semio_framework_async::poll::resolve_ready(instance.app.transaction_prepare(&txn_id, &mutation_id, &payload, &prepared_ops, &prepared_child_ops, decoded_origin)))
                        });
                        match outcome.await {
                            Ok(outcome) => {
                                let mut foreign = Vec::with_capacity(outcome.foreign.len());
                                for step in outcome.foreign.iter() {
                                    foreign.push(encode_wire_serialized(step));
                                }
                                let rejection = match outcome.rejection {
                                    Some(fault) => encode_wire_serialized(&fault),
                                    None => Vec::new(),
                                };
                                frames.push(protocol::AppFrame::TransactionPrepared { txn_id, foreign, rejection });
                                frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                            }
                            Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                        }
                    }
                }
            }
            protocol::AppCommand::TransactionCommit { seq, txn_id } => {
                let meta = ActionMeta { actor: instance_actor(runtime, instance_id).await?.into(), instance_id, view_state: None };
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.transaction_commit(&txn_id, &meta, identity))
                });
                match outcome.await {
                    Ok(edit_id) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::TransactionCommitted { txn_id, edit_id });
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::TransactionRollback { seq, txn_id } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.transaction_rollback(&txn_id))
                });
                match outcome.await {
                    Ok(()) => {
                        frames.push(protocol::AppFrame::TransactionRolledBack { txn_id });
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::TransactionUndo { seq, group_id } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.transaction_undo(&group_id, identity))
                });
                match outcome.await {
                    Ok(()) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::TransactionRedo { seq, group_id } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.transaction_redo(&group_id, identity))
                });
                match outcome.await {
                    Ok(()) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } => match relay_open_artifact(artifact_ref, role, plugin_id, app_id).await {
                Ok(effect) => {
                    frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    effect_bytes.push(encode_wire_serialized(&effect));
                }
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } => match relay_set_default_app(artifact_kind, standard, subset, role, plugin_id, app_id).await {
                Ok(effect) => {
                    frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    effect_bytes.push(encode_wire_serialized(&effect));
                }
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } => match relay_clear_default_app(artifact_kind, standard, subset, role).await {
                Ok(effect) => {
                    frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    effect_bytes.push(encode_wire_serialized(&effect));
                }
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::SetMergePolicy { seq, policy } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(set_merge_policy_frames(&mut instance.app, seq, policy))
                });
                match outcome.await {
                    Ok(outcome) => frames.extend(outcome),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ResolveConflict { seq, conflict_id, resolution } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(resolve_conflict_frames(&mut instance.app, seq, conflict_id, resolution))
                });
                match outcome.await {
                    Ok(outcome) => frames.extend(outcome),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadConflicts { seq } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    Ok(::semio_framework_async::poll::resolve_ready(read_conflicts_frames(&instance.app, seq)))
                });
                match outcome.await {
                    Ok(outcome) => frames.extend(outcome),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
        }
        }
    }
    if mutated {
        let effects = with_instances_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            let view = instance.surface_contexts.view().cloned();
            Ok(::semio_framework_async::poll::resolve_ready(instance.app.pending_effects(view.as_ref())))
        })
        .await
        .unwrap_or_default();
        for effect in effects.iter() {
            effect_bytes.push(encode_wire_serialized(effect));
        }
    }
    let (mut typed_output, mut typed_leftover) = with_instances_original_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        advance_typed_operation_output_with_leftover(&mut instance.app, instance_id, identity,cx)
    })
    .await?;
    if let Some(operation) = started_typed_operation {
        let mut pumps = 0;
        while pumps < TYPED_OPERATION_REPLY_LEFTOVER_PUMPS && typed_leftover.as_ref().is_none_or(|leftover| leftover.operation != operation) && typed_output.typed_operation_results.is_empty() && !typed_output.ownership_receipt_pending() {
            let (more, leftover) = with_instances_original_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                if !instance.app.has_runnable_typed_operations() {
                    return Ok((None, None));
                }
                advance_typed_operation_output_with_leftover(&mut instance.app, instance_id, identity,cx).map(|(output, leftover)| (Some(output), leftover))
            })
            .await?;
            let Some(more) = more else { break };
            typed_output.original_receiving=more.original_receiving;
            typed_output.constructor_receipt=more.constructor_receipt;
            typed_output.preparation_receipt=more.preparation_receipt;
            typed_output.retirement_receipt=more.retirement_receipt;
            typed_output.history_command_receipt=more.history_command_receipt;
            typed_output.tool_run_receipt=more.tool_run_receipt;
            typed_output.frames.extend(more.frames);
            typed_output.effects.extend(more.effects);
            typed_output.events.extend(more.events);
            typed_output.typed_operation_results.extend(more.typed_operation_results);
            if leftover.is_some() {
                typed_leftover = leftover;
            }
            pumps += 1;
        }
    }
    effect_bytes.extend(typed_output.effects);
    event_bytes.extend(typed_output.events);
    if let Some(leftover) = typed_leftover.as_ref() {
        let merged = merge_typed_operation_leftover_into_reply(&mut frames, leftover);
        debug_runtime_line(format_args!("[TRACE] typed-operation inline leftover operation={} merged-into-reply={merged}", leftover.operation));
    }
    let ephemeral = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(::semio_framework_async::poll::resolve_ready(instance.app.ephemeral_snapshot()))
    });
    if let Ok(EphemeralSnapshot { presence, presence_generation, transient_generation, interaction, tool_run, history_edit }) = ephemeral.await {
        let tool_run = tool_run.map_or_else(Vec::new, |tool_run| {
            let mut bytes = Vec::new();
            protocol::encode_presence_tool_run(&tool_run, &mut bytes);
            bytes
        });
        let history_edit = history_edit.map_or_else(Vec::new, |history_edit| {
            let mut bytes = Vec::new();
            protocol::encode_presence_history_edit(&history_edit, &mut bytes);
            bytes
        });
        frames.push(protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run, history_edit });
    }
    if let Some(outcome) = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        Ok(instance.app.take_presence_outcome())
    })
    .await?
    {
        if let Some(fault) = outcome.fault {
            push_app_fault(&mut frames, Some(outcome.seq), fault).await;
        }
    }
    let mut frame_bytes = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        frame_bytes.push(protocol::encode_app_frame(frame).await);
    }
    frame_bytes.extend(typed_output.frames);
    Ok(PluginExchangeOutput {
        original_receiving: typed_output.original_receiving,
        constructor_receipt: typed_output.constructor_receipt,
        preparation_receipt: typed_output.preparation_receipt,
        retirement_receipt: typed_output.retirement_receipt,
        history_command_receipt: typed_output.history_command_receipt,
        tool_run_receipt: typed_output.tool_run_receipt,
        frames: frame_bytes,
        effects: effect_bytes,
        events: event_bytes,
        retry_command,
        command_terminal_fault,
        presence_pending,
        presence_terminal,
        presence_terminal_fault,
        typed_operation_results: typed_output.typed_operation_results,
    })
}
/// 🧩️ One actor-world export implementation shared by plugin and extension component owners.
#[doc(hidden)]
#[macro_export]
macro_rules! __semio_actor_exports {
    ($guest:ident, $runtime:ident, $ensure:ident, $describe:ident) => {
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        struct $guest;
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::reactor::Guest for $guest {
            async fn stage_command_page(cursor: $crate::component::wasip2::exports::semio::framework::reactor::CommandPageCursor, bytes: Vec<u8>) -> Result<(), $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $crate::__async::poll::resolve_ready($crate::reactor::stage_command_page(cursor, bytes)).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn stage_cold_pair_page(page: $crate::component::wasip2::exports::semio::framework::reactor::ColdDocumentPairPage) -> Result<(), $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $crate::__async::poll::resolve_ready($crate::reactor::stage_cold_pair_page(page)).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn admit_actor_context(input: $crate::component::wasip2::exports::semio::framework::reactor::RetainedTurnInput,retained_policy:$crate::component::wasip2::exports::semio::framework::reactor::RetainedGrant) -> $crate::component::wasip2::exports::semio::framework::reactor::ActorAdmissionCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::admit_wit(input,retained_policy)
            }
            async fn close_actor_context(input: $crate::component::wasip2::exports::semio::framework::reactor::RetainedTurnInput,retained_policy:$crate::component::wasip2::exports::semio::framework::reactor::RetainedGrant) -> $crate::component::wasip2::exports::semio::framework::reactor::ActorContextCloseCall {
                $crate::reactor::original_actor_context::exports::close_wit(input,retained_policy)
            }
            async fn reserve_poll_input(extent:u64,budget:$crate::component::wasip2::exports::semio::framework::reactor::Budget)->Result<$crate::component::wasip2::exports::semio::framework::reactor::ActorPollInputCall,$crate::component::wasip2::exports::semio::framework::reactor::ActorCallRefusal>{
                $ensure();
                $crate::reactor::original_actor_context::exports::reserve_poll_input_wit(extent,budget)
            }
            async fn write_poll_input(page:$crate::component::wasip2::exports::semio::framework::reactor::PollInputPage,budget:$crate::component::wasip2::exports::semio::framework::reactor::Budget)->Result<$crate::component::wasip2::exports::semio::framework::reactor::ActorPollInputCall,$crate::component::wasip2::exports::semio::framework::reactor::ActorCallRefusal>{
                $crate::reactor::original_actor_context::exports::write_poll_input_wit(page,budget)
            }
            async fn close_poll_input(budget:$crate::component::wasip2::exports::semio::framework::reactor::Budget)->Result<$crate::component::wasip2::exports::semio::framework::reactor::ActorPollInputCall,$crate::component::wasip2::exports::semio::framework::reactor::ActorCallRefusal>{
                $crate::reactor::original_actor_context::exports::close_poll_input_wit(budget)
            }
            async fn poll(
                events: Vec<$crate::component::wasip2::exports::semio::framework::reactor::Event>,
                budget: $crate::component::wasip2::exports::semio::framework::reactor::Budget,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::reactor::TurnResult, $crate::component::wasip2::exports::semio::framework::reactor::ActorCallRefusal> {
                $ensure();
                $runtime.with(|runtime| $crate::reactor::original_actor_context::exports::poll_wit(runtime, events, budget))
            }
            async fn channel_version() -> u32 {
                $crate::component::wasip2::channel_version()
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::jobs::Guest for $guest {
            async fn start_job(source: Option<$crate::component::wasip2::exports::semio::framework::jobs::JobSource>, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobStartCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::start_job_wit(source, budget)
            }
            async fn step_job(job: u64, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobStepCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::step_job_wit(job, budget)
            }
            async fn read_job_output(job: u64, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobOutputCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::read_job_output_wit(job, budget)
            }
            async fn acknowledge_job_output(job: u64, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobOutputAckCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::acknowledge_job_output_wit(job, budget)
            }
            async fn cancel_job(job: u64, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobCancelCall {
                $crate::reactor::original_actor_context::exports::cancel_job_wit(job, budget)
            }
            async fn close_jobs(budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobCancelCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::close_jobs_wit(budget)
            }

            async fn take_segmented_download_chunk(instance_id: u32, operation_id: u64) -> Result<Option<Vec<u8>>, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_take_segmented_download_chunk(runtime, instance_id, operation_id))).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::checkpoint::Guest for $guest {
            async fn checkpoint() -> Result<Vec<u8>, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::reactor::checkpoint_now(runtime))).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn restore(state: Vec<u8>) -> Result<(), $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::reactor::restore_now(runtime, &state))).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::describe::Guest for $guest {
            async fn describe() -> Vec<u8> {
                $ensure();
                $describe()
            }
        }
        // 🌱️ OS-HUB-COLLABORATION-AI-END-TO-END (TC3b): `world actor`'s `codec` export. Every
        // function resolves the owning app from the installed bundle by `artifact-schema` and
        // runs on a throwaway instance, so none of them observe or disturb live actor state.
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::codec::Guest for $guest {
            async fn sqlite_schema(dialect: String) -> Result<String, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $crate::plugin_runtime::plugin_snapshot_sqlite_schema(&dialect).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_export(
                dialect: String,
                encoding: $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding,
                snapshot: Vec<u8>,
                limits: $crate::component::wasip2::exports::semio::framework::codec::SnapshotLimits,
                native: $crate::component::wasip2::semio::framework::types::RetainedCloneGrant,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                let encoding = match encoding {
                    $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding::Binary => "binary",
                    $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding::Text => "text",
                }
                .to_string();
                let mut input = Some($crate::sqlite_wire::SnapshotInput { dialect, encoding, payload: snapshot, limits: $crate::component::wasip2::sqlite_limits(limits), native: $crate::component::wasip2::sqlite_grant(native) });
                $crate::operation_authority::with_snapshot_authority(&mut input,$crate::io::io_mechanism::IoNativeDirection::Decode,true,|input,native,snapshot|$crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_snapshot_sqlite_export(input,native,snapshot)))
                    .map_err(|error| $crate::component::wasip2::semio::framework::types::PluginError::OperationRefusal($crate::operation_authority::refusal_code(error.kind)))?
                    .map(|output| match output {
                        $crate::sqlite_wire::SnapshotFileResult::Done(output) => {
                            $crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult::Done($crate::component::wasip2::exports::semio::framework::codec::SnapshotFile { bytes: output.bytes, diagnostics: output.diagnostics })
                        }
                        $crate::sqlite_wire::SnapshotFileResult::Rejected(output) => {
                            $crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult::Rejected($crate::component::wasip2::exports::semio::framework::codec::SnapshotRejection {
                                kind: $crate::component::wasip2::sqlite_refusal_kind(output.kind),
                                message: output.message,
                                diagnostics: output.diagnostics,
                            })
                        }
                        $crate::sqlite_wire::SnapshotFileResult::Pending(owner) => $crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult::Pending($crate::component::wasip2::sqlite_retirement(owner)),
                    })
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_import(
                dialect: String,
                file: Vec<u8>,
                limits: $crate::component::wasip2::exports::semio::framework::codec::SnapshotLimits,
                native: $crate::component::wasip2::semio::framework::types::RetainedCloneGrant,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                let mut input = Some($crate::sqlite_wire::SnapshotInput { dialect, encoding: String::new(), payload: file, limits: $crate::component::wasip2::sqlite_limits(limits), native: $crate::component::wasip2::sqlite_grant(native) });
                $crate::operation_authority::with_snapshot_authority(&mut input,$crate::io::io_mechanism::IoNativeDirection::Encode,true,|input,native,snapshot|$crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_snapshot_sqlite_import(input,native,snapshot)))
                    .map_err(|error| $crate::component::wasip2::semio::framework::types::PluginError::OperationRefusal($crate::operation_authority::refusal_code(error.kind)))?
                    .map(|output| match output {
                        $crate::sqlite_wire::SnapshotPayloadResult::Done(output) => {
                            let encoding = match output.encoding.as_str() {
                                "binary" => $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding::Binary,
                                _ => $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding::Text,
                            };
                            $crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult::Done($crate::component::wasip2::exports::semio::framework::codec::SnapshotPayload {
                                encoding,
                                bytes: output.bytes,
                                diagnostics: output.diagnostics,
                            })
                        }
                        $crate::sqlite_wire::SnapshotPayloadResult::Rejected(output) => {
                            $crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult::Rejected($crate::component::wasip2::exports::semio::framework::codec::SnapshotRejection {
                                kind: $crate::component::wasip2::sqlite_refusal_kind(output.kind),
                                message: output.message,
                                diagnostics: output.diagnostics,
                            })
                        }
                        $crate::sqlite_wire::SnapshotPayloadResult::Pending(owner) => $crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult::Pending($crate::component::wasip2::sqlite_retirement(owner)),
                    })
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_take_file(ticket:u64)->Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult,$crate::component::wasip2::semio::framework::types::PluginError>{
                $crate::operation_authority::snapshot_take::<$crate::sqlite_wire::SnapshotFileResult>(ticket,true).map_err(|error|$crate::component::wasip2::semio::framework::types::PluginError::OperationRefusal($crate::operation_authority::refusal_code(error.kind)))?.map($crate::component::wasip2::sqlite_file).map_err(|fault|$crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_take_payload(ticket:u64)->Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult,$crate::component::wasip2::semio::framework::types::PluginError>{
                $crate::operation_authority::snapshot_take::<$crate::sqlite_wire::SnapshotPayloadResult>(ticket,true).map_err(|error|$crate::component::wasip2::semio::framework::types::PluginError::OperationRefusal($crate::operation_authority::refusal_code(error.kind)))?.map($crate::component::wasip2::sqlite_payload).map_err(|fault|$crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_retirement(copy_bytes:u64) -> Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotRetirement,u32> {
                $crate::operation_authority::snapshot_retirement(usize::try_from(copy_bytes).map_err(|_|2u32)?).map($crate::component::wasip2::sqlite_retirement).map_err(|error| $crate::operation_authority::refusal_code(error.kind))
            }
            async fn sqlite_close(ticket:u64,native:$crate::component::wasip2::semio::framework::types::RetainedCloneGrant) -> Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotCloseReceipt,u32> {
                let grant=$crate::component::wasip2::sqlite_grant(native).native().map_err(|error| $crate::operation_authority::refusal_code(error.kind))?;
                $crate::operation_authority::snapshot_close(ticket,grant).map(|(retirement,progress,refusal)|$crate::component::wasip2::exports::semio::framework::codec::SnapshotCloseReceipt {
                    retirement:$crate::component::wasip2::sqlite_retirement(retirement),
                    progress:$crate::component::wasip2::semio::framework::types::RetainedCloneProgress{copied_items:progress.copied_items as u64,copied_bytes:progress.copied_bytes as u64,retained_capacity_bytes:progress.retained_capacity_bytes as u64,released_bytes:progress.released_bytes as u64},refusal,
                }).map_err(|error| $crate::operation_authority::refusal_code(error.kind))
            }
            async fn pack_schema_hash(artifact_schema: String) -> Result<Vec<u8>, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_pack_schema_hash(runtime, &artifact_schema)))
                    .map(|hash| hash.to_vec())
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn genesis(artifact_schema: String, document_id: String) -> Result<$crate::component::wasip2::exports::semio::framework::codec::DocumentPair, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_genesis(runtime, &artifact_schema, &document_id)))
                    .map(|files| $crate::component::wasip2::exports::semio::framework::codec::DocumentPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn print_mirror(artifact_schema: String, pair: $crate::component::wasip2::exports::semio::framework::codec::DocumentPair) -> Result<(String, String), $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_print_mirror(runtime, &artifact_schema, &pair.pack, &pair.spr)))
                    .map(|files| (files.dsl, files.ops))
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn apply_ops(
                artifact_schema: String,
                pair: $crate::component::wasip2::exports::semio::framework::codec::DocumentPair,
                ops: Vec<u8>,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::codec::DocumentPair, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::operation_authority::with_wit_operation_authority(|identity| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_apply_ops(runtime, &artifact_schema, &pair.pack, &pair.spr, &ops , identity)).map(|files| $crate::component::wasip2::exports::semio::framework::codec::DocumentPair { pack: files.pack, spr: files.spr }),$crate::operation_authority::return_walk::walk_document_pair))
            }
            async fn replay_envelopes(
                artifact_schema: String,
                pair: $crate::component::wasip2::exports::semio::framework::codec::DocumentPair,
                envelopes: Vec<u8>,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::codec::DocumentPair, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_replay_envelopes(runtime, &artifact_schema, &pair.pack, &pair.spr, &envelopes)))
                    .map(|files| $crate::component::wasip2::exports::semio::framework::codec::DocumentPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        $crate::__semio_export_component_guest!($guest);
    };
}
/// 🧬️ Defines the single owned core ABI used by plugin and extension component owners.
/// Every export must match the interpreter's authored name and core type.
/// Each owner supplies its persistent runtime slot, install hook and descriptor encoder.
#[doc(hidden)]
#[macro_export]
macro_rules! __semio_owned_core_exports {
    ($runtime:ident, $ensure:ident, $describe_bytes:ident) => {
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_alloc_v1(length: u32) -> u32 {
            $crate::owned_abi::allocate(length)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_dealloc_v1(pointer: u32, length: u32) {
            unsafe { $crate::owned_abi::deallocate(pointer, length) };
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_admit_actor_context_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_actor_admission_input(pointer, length) };
            $ensure();
            match input {
                Ok((input,policy)) => $crate::reactor::original_actor_context::exports::admit_owned(input,policy),
                Err(kind) => $crate::operation_authority::owned_result::encode_refusal(kind),
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_close_actor_context_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_actor_admission_input(pointer, length) };
            match input {
                Ok((input,policy)) => $crate::reactor::original_actor_context::exports::close_owned(input,policy),
                Err(kind) => $crate::operation_authority::owned_result::encode_refusal(kind),
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_poll_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_actor_poll_input(pointer, length) };
            $ensure();
            match input {
                Ok(input) => $runtime.with(|runtime| $crate::reactor::original_actor_context::exports::poll_owned(runtime, input)),
                Err(kind) => $crate::operation_authority::owned_result::encode_refusal(kind),
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_start_job_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::StartJobInput>(pointer, length) };
            $ensure();
            let result: Result<(), Vec<u8>> = match input {
                Ok(input) => $runtime.with(|runtime| $crate::reactor::jobs::abi::start(runtime, input.job, input.kind, input.input)).map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_step_job_v1(job:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::step_job_owned(job,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_read_job_output_v1(job:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::read_job_output_owned(job,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_acknowledge_job_output_v1(job:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::acknowledge_job_output_owned(job,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_cancel_job_v1(job:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::cancel_job_owned(job,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_close_jobs_v1(operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::close_jobs_owned(budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_borrow_job_output_v1(job:u64,operation:u64,generation:u64,offset:u64,length:u64) -> u64 {$crate::reactor::original_actor_context::exports::borrow_job_output_owned(job,operation,generation,offset,length)}
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_acknowledge_job_output_chunk_v1(job:u64,offset:u64,length:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::acknowledge_job_output_chunk_owned(job,offset,length,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}}
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_checkpoint_v1() -> u64 {
            $ensure();
            let result = $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::reactor::checkpoint_now(runtime))).map_err(|fault| $crate::encode_fault_bytes(&fault));
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_restore_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::RestoreInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::reactor::restore_now(runtime, &input.state))).map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_describe_v1() -> u64 {
            $ensure();
            $crate::owned_abi::return_bytes($describe_bytes())
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_channel_version_v1() -> u64 {
            $crate::owned_abi::return_json(&$crate::component::wasip2::channel_version())
        }
        // 🌱️ OS-HUB-COLLABORATION-AI-END-TO-END (TC3b): the owned-ABI twin of `world actor`'s
        // `codec` export, so the repository-owned interpreter (the production `describe` path,
        // and the hub's only guest execution path) reaches the same four functions wasmtime and
        // jco reach through the WIT.
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_schema_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::sqlite_wire::SnapshotInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $crate::plugin_runtime::plugin_snapshot_sqlite_schema(&input.dialect).map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_export_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::sqlite_wire::SnapshotInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => {let mut input=Some(input);match $crate::operation_authority::with_snapshot_authority(&mut input,$crate::io::io_mechanism::IoNativeDirection::Decode,false,|input,native,snapshot|$crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_snapshot_sqlite_export(input,native,snapshot))){Ok(result)=>result.map_err(|fault| $crate::encode_fault_bytes(&fault)),Err(error)=>return $crate::operation_authority::owned_result::encode_refusal(error.kind)}},
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_import_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::sqlite_wire::SnapshotInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => {let mut input=Some(input);match $crate::operation_authority::with_snapshot_authority(&mut input,$crate::io::io_mechanism::IoNativeDirection::Encode,false,|input,native,snapshot|$crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_snapshot_sqlite_import(input,native,snapshot))){Ok(result)=>result.map_err(|fault| $crate::encode_fault_bytes(&fault)),Err(error)=>return $crate::operation_authority::owned_result::encode_refusal(error.kind)}},
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch="wasm32",target_env="p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_retirement_v1(pointer:u32,length:u32)->u64 {
            let body=match unsafe{$crate::owned_abi::take_json::<usize>(pointer,length)}{Ok(body)=>body,Err(error)=>return $crate::owned_abi::return_json(&Err::<$crate::sqlite_wire::SnapshotRetirement,_>(error))};
            match $crate::operation_authority::snapshot_retirement(body){Ok(value)=>$crate::owned_abi::return_json(&Ok::<_,Vec<u8>>(value)),Err(error)=>$crate::operation_authority::owned_result::encode_refusal(error.kind)}
        }
        #[cfg(all(target_arch="wasm32",target_env="p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_close_v1(pointer:u32,length:u32)->u64 {
            let input=unsafe{$crate::owned_abi::take_json::<$crate::sqlite_wire::SnapshotCloseInput>(pointer,length)};
            let input=match input{Ok(input)=>input,Err(error)=>return $crate::owned_abi::return_json(&Err::<$crate::sqlite_wire::SnapshotCloseReceipt,_>(error))};
            let grant=match input.native.native(){Ok(grant)=>grant,Err(error)=>return $crate::operation_authority::owned_result::encode_refusal(error.kind)};
            match $crate::operation_authority::snapshot_close(input.ticket,grant){Ok((retirement,progress,refusal))=>$crate::owned_abi::return_json(&Ok::<_,Vec<u8>>($crate::sqlite_wire::SnapshotCloseReceipt{retirement,progress:progress.into(),refusal})),Err(error)=>$crate::operation_authority::owned_result::encode_refusal(error.kind)}
        }
        #[cfg(all(target_arch="wasm32",target_env="p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_take_file_v1(pointer:u32,length:u32)->u64 {
            let ticket=match unsafe{$crate::owned_abi::take_json::<u64>(pointer,length)}{Ok(ticket)=>ticket,Err(error)=>return $crate::owned_abi::return_json(&Err::<$crate::sqlite_wire::SnapshotFileResult,_>(error))};
            match $crate::operation_authority::snapshot_take::<$crate::sqlite_wire::SnapshotFileResult>(ticket,false){Ok(result)=>$crate::owned_abi::return_json(&result.map_err(|fault|$crate::encode_fault_bytes(&fault))),Err(error)=>$crate::operation_authority::owned_result::encode_refusal(error.kind)}
        }
        #[cfg(all(target_arch="wasm32",target_env="p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_take_payload_v1(pointer:u32,length:u32)->u64 {
            let ticket=match unsafe{$crate::owned_abi::take_json::<u64>(pointer,length)}{Ok(ticket)=>ticket,Err(error)=>return $crate::owned_abi::return_json(&Err::<$crate::sqlite_wire::SnapshotPayloadResult,_>(error))};
            match $crate::operation_authority::snapshot_take::<$crate::sqlite_wire::SnapshotPayloadResult>(ticket,false){Ok(result)=>$crate::owned_abi::return_json(&result.map_err(|fault|$crate::encode_fault_bytes(&fault))),Err(error)=>$crate::operation_authority::owned_result::encode_refusal(error.kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_pack_schema_hash_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_pack_schema_hash(runtime, &input.artifact_schema)))
                    .map(|hash| hash.to_vec())
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_genesis_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_genesis(runtime, &input.artifact_schema, &input.document_id)))
                    .map(|files| $crate::owned_abi::CodecPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_print_mirror_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_print_mirror(runtime, &input.artifact_schema, &input.pack, &input.spr)))
                    .map(|files| $crate::owned_abi::CodecMirror { dsl: files.dsl, ops: files.ops })
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_apply_ops_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => match $runtime
                    .with(|runtime| $crate::operation_authority::with_owned_operation_authority(|identity| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_apply_ops(runtime, &input.artifact_schema, &input.pack, &input.spr, &input.ops , identity))))
                    {Ok(result)=>result,Err(error)=>return $crate::operation_authority::owned_result::encode_refusal(error.kind)}
                    .map(|files| $crate::owned_abi::CodecPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_replay_envelopes_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_replay_envelopes(runtime, &input.artifact_schema, &input.pack, &input.spr, &input.ops)))
                    .map(|files| $crate::owned_abi::CodecPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
    };
}
/// 🔌️ Exports one typed actor runtime for a plugin bundle.
#[macro_export]
macro_rules! plugin_exports {
    ($mounted_policy:expr, $bundle_fn:expr, $app:ty) => {
        $crate::__semio_plugin_actor_exports!($mounted_policy, $bundle_fn, $app, $crate::describe::describe_plugin, || {});
    };
}
#[macro_export]
macro_rules! __semio_plugin_actor_exports {
    ($mounted_policy:expr, $bundle_fn:expr, $app:ty, $describe:path, $refresh:expr) => {
        $crate::component_persistent_local! {
            static __SEMIO_PLUGIN_RUNTIME: $crate::plugin_runtime::PluginRuntime<$app> = $crate::plugin_runtime::PluginRuntime::new($mounted_policy).expect("explicit embedding mounted policy");
        }
        fn __semio_ensure_plugin_runtime() {
            static ONCE: std::sync::Once = std::sync::Once::new();
            ONCE.call_once(|| {
                $crate::plugin_runtime::install_guest_panic_report();
                __semio_install_plugin_bundle();
            });
            ($refresh)();
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        fn __semio_describe_component() -> Vec<u8> {
            __SEMIO_PLUGIN_RUNTIME.with(|runtime| $crate::__async::poll::resolve_ready($describe(runtime)))
        }
        $crate::__semio_owned_core_exports!(__SEMIO_PLUGIN_RUNTIME, __semio_ensure_plugin_runtime, __semio_describe_component);
        $crate::__semio_actor_exports!(__SemioComponentGuest, __SEMIO_PLUGIN_RUNTIME, __semio_ensure_plugin_runtime, __semio_describe_component);
        // 🚫️async: E1 called from the sync `std::sync::Once` closure of `__semio_ensure_plugin_runtime`.
        fn __semio_install_plugin_bundle() {
            __SEMIO_PLUGIN_RUNTIME.with(|runtime| {
                $crate::plugin_runtime::install_plugin_bundle_result(runtime, ($bundle_fn)());
            });
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[used]
        static _SEMIO_PLUGIN_COMPONENT_LINK: fn() = $crate::component_export_anchor;
        // 🛂️ E1-describe (`📓️design-abi.md` §3), path corrected by E2-builder-descriptor: freshness
        // without wasm in the loop — natively installs the bundle and byte-compares
        // `describe::describe_plugin()`'s packed
        // output against the crate's checked-in `<owner>/🛂️.descriptor.semio`. Registrar ruling
        // (`📌️important.md`, this ticket): descriptors live at the plugin/extension OWNER ROOT,
        // sibling of the tracked `🛂️manifest.json` — NOT under `🤖️generated/`, which is globally
        // gitignored (`.gitignore` 87-88) and would mean this checked-in file could never survive
        // a commit; `📓️design-abi.md` §3's `🤖️generated` path is superseded.
        //
        // 🚦️ D0-descriptor-plumbing ratchet: a missing committed descriptor is a HARD failure only
        // for a plugin explicitly named in `DESCRIPTOR_MIGRATED_PLUGINS` below — this is the
        // "explicit opt-in, visible per plugin" gate `important.md` calls for, so the fleet does not
        // go red before each plugin's own emission packet lands. Extend this list from a plugin's
        // D-packet report the moment its `🛂️.descriptor.semio` + `🔣️.json` are committed —
        // never remove a name once added (that would be silently re-hiding a regression). The
        // repo-wide ratchet metric this ladder climbs against is `📇️registry:check`'s own
        // "descriptor gate: N/<total> crates have a 🔣️.json" census line — that count is
        // the thing that should trend toward `<total>` as this list grows, never this test alone.
        $crate::__semio_plugin_descriptor_fresh_test!($describe);
    };
}
/// 📨️ Encoded extension request handler owned by its guest bundle.
enum ExtensionRequestHandler {
    Plain(fn(&[u8],&mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault>),
    Owned,
}
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
#[path = "../../🧩️extension/🚪️retirement/🦀️.rs"]
mod extension_retirement;
pub use extension_retirement::{ExtensionResourceOwner,ExtensionInvokeStep};
use extension_retirement::{ExtensionBundleRegistry, MetadataRetirement};
/// 🧩️ Extension guest bundle — no apps; contributes + invoke handlers.
pub struct ExtensionBundle {
    pub manifest: std::mem::ManuallyDrop<ExtensionManifest>,
    handlers: std::mem::ManuallyDrop<Vec<(String, ExtensionRequestHandler)>>,
    resource_owner: std::mem::ManuallyDrop<Option<Box<dyn ExtensionResourceOwner>>>,
    pub(crate) closing: bool,
    close_cancelled: bool,
    manifest_retired: bool,
    metadata_retirement: std::mem::ManuallyDrop<MetadataRetirement>,
    inference_gateway:std::mem::ManuallyDrop<std::sync::Mutex<Option<crate::ArtifactInferenceGateway>>>,
}
/// 🌉️ `CapabilityRequirement`/`PluginDependency`/`ArtifactContributionDescriptor`/`ExecutionMode`
/// are foreign (owned by the `semio-framework` crate's kernel/manifest vocabulary) and only
/// implement `Serialize`/`Deserialize`, not `ToValue`/`FromValue` — orphan rule forbids
/// implementing either trait for them here. `ExtensionManifest` below bridges each through
/// `serde_json` via `#[value(serialize_with = …, deserialize_with = …)]` instead of deriving;
/// generic over `T` so one pair covers both the bare `ExecutionMode` field and the `Vec<_>`
/// fields. `CapabilityRequest` now derives `ToValue`/`FromValue` directly upstream (kernel), so
/// `capability_requests` below uses the default derive instead of this bridge.
mod foreign_kernel_value_bridge {
    pub fn to_value<T: serde::Serialize>(value: &T) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::from(&serde_json::to_value(value).expect("value always serializes"))
    }
    pub fn from_value<T: serde::de::DeserializeOwned>(value: semio_framework_value::DslValue) -> Result<T, semio_framework_value::ValueError> {
        serde_json::from_value(serde_json::Value::from(value)).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
}
/// 📦️ Manifest for a runtime-installable extension (WIT `extension::manifest` payload).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ExtensionManifest {
    pub package_id: String,
    pub extension_id: String,
    pub label: String,
    pub version: String,
    pub extends: String,
    #[value(default, skip_serializing_if = "Vec::is_empty", serialize_with = "foreign_kernel_value_bridge::to_value", deserialize_with = "foreign_kernel_value_bridge::from_value")]
    pub capabilities: Vec<CapabilityRequirement>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_contributions: Vec<TopicContribution>,
    /// 🔗️ Direct plugin dependencies this extension requires — contract freeze §3/§4: `extends`
    /// MUST equal `dependencies[0].plugin_id`; `.extends`/`.depends_on` both assert this on every
    /// call that could violate it. Real `semio_framework::PluginDependency` (not a local mirror —
    /// `semio-framework-plugin` already depends on `semio-framework`, unlike the `.sxt` package
    /// format's own `ExtensionPackageManifest`).
    #[value(default, skip_serializing_if = "Vec::is_empty", serialize_with = "foreign_kernel_value_bridge::to_value", deserialize_with = "foreign_kernel_value_bridge::from_value")]
    pub dependencies: Vec<semio_framework::PluginDependency>,
    /// 🗂️ Artifact-kind contributions this extension contributes onto artifact kinds it depends on.
    #[value(default, skip_serializing_if = "Vec::is_empty", serialize_with = "foreign_kernel_value_bridge::to_value", deserialize_with = "foreign_kernel_value_bridge::from_value")]
    pub contributions: Vec<semio_framework::ArtifactContributionDescriptor>,
    /// 🚦️ `📓️design-abi.md` §5 — how this extension's actor runs relative to its host plugin.
    /// Default `Isolated` (`ExecutionMode::default()`), set via `ExtensionBundle::mode(..)`.
    #[value(default, serialize_with = "foreign_kernel_value_bridge::to_value", deserialize_with = "foreign_kernel_value_bridge::from_value")]
    pub execution: ExecutionMode,
    /// 🙏️ Capability asks the broker resolves at install/link/runtime, set via
    /// `ExtensionBundle::requests(..)` — the NEW broker-scoped `kernel::CapabilityRequest`, not
    /// the older kernel-level `CapabilityRequirement` `capabilities` above carries (see that
    /// field's own doc for why both exist).
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_requests: Vec<CapabilityRequest>,
}
impl ExtensionBundle {
    /// 🧩️ Starts an extension bundle with identity + version.
    /// 🚫️async: E1 — pure struct literal, zero suspension points; reverted per R9 alongside
    /// `depends_on`/`extends`/`assert_extends_matches_primary_dependency` below (their only
    /// sync-closure `catch_unwind` test consumers are language-barred from async). `contributes`
    /// stays genuinely async — real registry I/O, not touched.
    pub fn new(extension_id: impl Into<String>, label: impl Into<String>, version: impl Into<String>) -> Self {
        let extension_id = extension_id.into();
        Self {
            manifest: std::mem::ManuallyDrop::new(ExtensionManifest {
                package_id: format!("semio:{extension_id}"),
                extension_id,
                label: label.into(),
                version: version.into(),
                extends: String::new(),
                capabilities: Vec::new(),
                topic_contributions: Vec::new(),
                dependencies: Vec::new(),
                contributions: Vec::new(),
                execution: ExecutionMode::default(),
                capability_requests: Vec::new(),
            }),
            handlers: std::mem::ManuallyDrop::new(Vec::new()),
            resource_owner: std::mem::ManuallyDrop::new(None),
            closing: false,
            close_cancelled: false,
            manifest_retired: false,
            metadata_retirement: std::mem::ManuallyDrop::new(MetadataRetirement::default()),
            inference_gateway:std::mem::ManuallyDrop::new(std::sync::Mutex::new(None)),
        }
    }
    /// 🔗 Declares the host app/plugin this extension extends.
    pub fn extends(mut self, extends: impl Into<String>) -> Self {
        self.manifest.extends = extends.into();
        self.assert_extends_matches_primary_dependency();
        self
    }
    /// 📦️ Declares the canonical component package identity emitted by `describe()`.
    pub fn package_id(mut self, package_id: impl Into<String>) -> Self {
        self.manifest.package_id = package_id.into();
        self
    }
    /// 🔗️ Declares a direct plugin dependency, pinned exactly (`semio_framework::tree_pin!()`) — contract freeze §3/§4:
    /// the FIRST dependency declared (in call order) must be the same plugin `.extends(...)` names.
    pub fn depends_on(mut self, plugin_id: impl Into<String>, version: semio_framework::VersionPin) -> Self {
        self.manifest.dependencies.push(semio_framework::PluginDependency::new(plugin_id, version));
        self.assert_extends_matches_primary_dependency();
        self
    }
    /// 🚨️ Contract freeze §3/§4: `extends == dependencies[0].plugin_id`. Panics on violation — the
    /// same misuse-reporting idiom `Plugin::plugin_command`'s `assert!`s already use elsewhere in
    /// this file, since every `ExtensionBundle` builder method is infallible (`-> Self`, never
    /// `Result`). Called from both `.extends`/`.depends_on` so the check holds regardless of call
    /// order; a no-op until both fields are non-empty.
    fn assert_extends_matches_primary_dependency(&self) {
        if self.manifest.extends.is_empty() {
            return;
        }
        if let Some(primary) = self.manifest.dependencies.first() {
            assert_eq!(
                primary.plugin_id, self.manifest.extends,
                "extension {:?} declares extends={:?} but its first dependency is {:?} — contract freeze §3/§4 requires extends == dependencies[0].plugin_id",
                self.manifest.extension_id, self.manifest.extends, primary.plugin_id
            );
        }
    }
    /// 🔒️ Declares a capability requirement for the extension.
    pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
        if !self.manifest.capabilities.contains(&capability) {
            self.manifest.capabilities.push(capability);
        }
        self
    }
    /// 🗂️ Adds an open topic contribution to the extension manifest; `topic` reuses this crate's
    /// own `contributes`/`consumes` metadata vocabulary (e.g. `"cad.computer"`).
    pub fn contributes_topic(mut self, topic: impl Into<String>, payload: DslValue) -> Self {
        self.manifest.topic_contributions.push(TopicContribution::new(topic, payload));
        self
    }
    /// 🗂️ Declares one contribution of mutations/inferences onto an artifact kind owned by a
    /// dependency. Resolved IMMEDIATELY (unlike `PluginBuilder::contributes`, an extension's own id
    /// is already known at `ExtensionBundle::new()`): gate-checked via
    /// `crate::app::register_contributions` and committed to the same process-global runtime
    /// registries `PluginBuilder::try_build` commits to. Panics on a gate violation — same idiom as
    /// `.extends`/`.depends_on` above.
    pub async fn contributes(mut self, contribution: crate::artifact_registration::ArtifactContribution) -> Self {
        let contributor = self.manifest.extension_id.clone();
        let (descriptor, inference_services, mutation_runtime) = contribution.resolve(&contributor);
        crate::artifact_registration::register_contributions(&contributor, &self.manifest.dependencies, std::slice::from_ref(&descriptor)).unwrap_or_else(|error| panic!("extension {contributor:?} contribution rejected: {error}"));
        crate::artifact_inference::preflight_artifact_inference_services(&inference_services).unwrap_or_else(|error| panic!("extension {contributor:?} contributed inference service conflict: {error}"));
        crate::artifact_inference::register_artifact_inference_services(inference_services).unwrap_or_else(|error| panic!("extension {contributor:?} contributed inference service registration failed: {error}"));
        crate::artifact_registration::commit_contributed_mutation_services(mutation_runtime).await.unwrap_or_else(|error| panic!("extension {contributor:?} contributed mutation registration failed: {error}"));
        self.manifest.contributions.push(descriptor);
        self
    }
    /// 🔀️ Registers a capability handler invoked via WIT `extension::invoke`.
    pub fn handler(mut self, capability: impl Into<String>, handler: fn(&[u8],&mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault>) -> Self {
        let capability = capability.into();
        assert!(!matches!(capability.as_str(), "extension.close.cancel" | "extension.close.resume"), "extension close control capability is reserved");
        self.insert_handler(capability, ExtensionRequestHandler::Plain(handler));
        self
    }
    /// 🚦️ Sets how this extension's actor runs relative to its host plugin — default `Isolated`
    /// (`📓️design-abi.md` §5).
    pub fn mode(mut self, mode: ExecutionMode) -> Self {
        self.manifest.execution = mode;
        self
    }
    /// 🙏️ Declares one capability ask the broker resolves at install/link/runtime
    /// (`📓️design-abi.md` §5). Repeatable; idempotent for a byte-identical ask.
    pub fn requests(mut self, request: CapabilityRequest) -> Self {
        if !self.manifest.capability_requests.contains(&request) {
            self.manifest.capability_requests.push(request);
        }
        self
    }
}
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-extension-bundle-dependency/🦀️.rs");
#[cfg(test)]
include!("../../🧪️tests/🔬️extension-retirement/🦀️.rs");
crate::component_persistent_local! {
    static EXTENSION_BUNDLE: RefCell<ExtensionBundleRegistry> = RefCell::new(ExtensionBundleRegistry::new());
}
/// 📤️ Admits one replacement while retaining the current owner; refusal leaves the candidate untouched.
pub async fn install_extension_bundle(candidate: &mut Option<ExtensionBundle>) -> Result<bool, Fault> {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow_mut().map_err(|_| plugin_internal_fault("extension install authority is busy"))?.install(candidate))
}
/// 📦️ Returns the installed extension manifest (empty defaults when unset).
pub async fn extension_manifest() -> ExtensionManifest {
    EXTENSION_BUNDLE.with(|slot| slot.borrow().current().map_or_else(ExtensionManifest::empty, |bundle| (*bundle.manifest).clone()))
}
/// 🚨️ Requests activation after any admitted replacement has retired its predecessor.
pub async fn extension_activate() -> Result<(), Fault> {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow_mut().map_err(|_| plugin_internal_fault("extension activation authority is busy"))?.activate())
}
/// 🔒️ Seals invocation and retains all installed resource owners for budgeted retirement.
pub async fn extension_deactivate() {
    EXTENSION_BUNDLE.with(|slot| slot.borrow_mut().begin_close());
}
/// ❄️ Explicitly disposes the installed registry for cold inspection and test callers.
pub fn extension_dispose_cold(cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    EXTENSION_BUNDLE.with(|slot| {
        let mut registry = slot.try_borrow_mut().map_err(|_| plugin_internal_fault("extension cold disposal authority is busy"))?;
        registry.begin_close();
        registry.resume_close();
        loop {
            let demand = registry.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
            match cx.with_retained_loan(extension_retirement::cold_grant(demand),|same|registry.close_step(same))? {
                PluginLifecycleStep::Complete(_) => return Ok(()),
                PluginLifecycleStep::Progress(progress) if progress != RetainedCloneProgress::default() => {}
                _ => return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"cold extension disposal requires its remaining original caller authority").into()),
            }
        }
    })
}
/// ♻️ Advances the installed registry under the caller's complete grant; a grant below any demanded axis yields an empty receipt.
pub fn extension_close_step(cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow_mut().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension close authority is busy"))?.close_step(cx))
}
pub fn extension_terminal_is_empty() -> bool {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow().is_ok_and(|registry| registry.terminal_is_empty()))
}
/// 📏️ Returns the minimal grant on every independent axis that the next close turn needs, without advancing its cursor.
pub fn extension_retirement_demands(maximum_body_bytes: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension close demand authority is busy"))?.retirement_demands(maximum_body_bytes))
}
pub fn extension_cancel_close() {
    EXTENSION_BUNDLE.with(|slot| slot.borrow_mut().cancel_close());
}
pub fn extension_resume_close() {
    EXTENSION_BUNDLE.with(|slot| slot.borrow_mut().resume_close());
}
pub(crate) fn extension_retirement_turn(cx:&mut semio_framework_job::StepContext<'_>) -> Result<(bool, PluginLifecycleStep), semio_framework_job::OriginalTurnFailure> {
    EXTENSION_BUNDLE.with(|slot| {
        let mut registry = slot.try_borrow_mut().map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension retirement authority is busy"))?;
        if !registry.has_retirement() {
            return Ok((false, PluginLifecycleStep::AwaitingInput { reason: "extension retirement awaits an explicit close request" }));
        }
        let step = registry.close_step(cx)?;
        let more_work = registry.has_retirement() && !matches!(step, PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. });
        Ok((more_work, step))
    })
}
/// 🔀️ Dispatches through the current unsealed owner's registered capability handler.
pub async fn extension_invoke(capability: &str, request: &[u8], cx:&mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault> {
    let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:None})}
    if matches!(capability, "extension.close.cancel" | "extension.close.resume") {
        if !request.is_empty() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("extension.close-control-payload"), "extension close controls require an empty request"));
        }
        if capability == "extension.close.cancel" {
            extension_cancel_close();
        } else {
            extension_resume_close();
        }
        let retained_progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(retained_progress).map_err(|error|FaultFrom::to_fault(&error))?;
        return Ok(ExtensionInvokeStep{payload:Some(Vec::new()),retained_progress,refusal:None});
    }
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow_mut().map_err(|_| plugin_internal_fault("extension invocation authority is busy"))?.invoke(capability, request, cx))
}
/// 🧷️ Supplies the current extension's retained inference context to the standard gateway.
pub(crate) fn cancel_extension_inference(cancellation_id: &str) -> bool {
    EXTENSION_BUNDLE.with(|slot| slot.try_borrow().ok().and_then(|state| state.current().and_then(|bundle| bundle.resource_owner.as_ref()).map(|owner| owner.cancel_inference(cancellation_id))).unwrap_or(false))
}
pub(crate) fn with_original_extension_inference_context<T>(run:impl FnOnce(Option<&dyn std::any::Any>)->Result<T,semio_framework_value::ValueError>)->Result<T,semio_framework_value::ValueError>{
    EXTENSION_BUNDLE.with(|slot|{
        let registry=slot.try_borrow().map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension inference owner is busy"))?;
        let bundle=registry.current();
        if bundle.is_some_and(|bundle|bundle.closing){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"extension inference owner is closing"));}
        run(bundle.and_then(|bundle|bundle.resource_owner.as_ref()).and_then(|owner|owner.inference_context()))
    })
}
pub(crate) fn with_extension_inference_context<T>(run: impl FnOnce(Option<&dyn std::any::Any>) -> Result<T, crate::artifact_inference::ArtifactInferenceExecutionError>) -> Result<T, crate::artifact_inference::ArtifactInferenceExecutionError> {
    EXTENSION_BUNDLE.with(|slot| {
        let registry = slot.try_borrow().map_err(|_| crate::artifact_inference::ArtifactInferenceExecutionError::new("artifact-inference.context-busy", "extension inference owner is busy"))?;
        let bundle = registry.current();
        if bundle.is_some_and(|bundle| bundle.closing) {
            return Err(crate::artifact_inference::ArtifactInferenceExecutionError::new("artifact-inference.context-closing", "extension inference owner is closing"));
        }
        run(bundle.and_then(|bundle| bundle.resource_owner.as_ref()).and_then(|owner| owner.inference_context()))
    })
}
/// 🚧️ PLACEHOLDER for W1-A. PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS
/// (§6): `extension-world` now exports `contributor` alongside `extension`, but no extension has
/// ever wired inference services — this is a deterministic EMPTY roster
/// (`store::pack_rt::encode_wire_value` of an empty list) until W1-A adds real extension-side
/// inference wiring (mirroring `crate::app::wire_list_artifact_inference_services`, plugin-side).
pub async fn extension_wire_list_artifact_inferences() -> Vec<u8> {
    store::pack_rt::encode_wire_value(&DslValue::Array(Vec::new()))
}
/// 🚧️ PLACEHOLDER for W1-A. Companion to `extension_wire_list_artifact_inferences` — since the
/// roster above is always empty, this can never legitimately be called; it exists only so
/// `contributor.artifact-infer` type-checks on `extension-world`. Always returns a typed `Fault`.
pub async fn extension_wire_artifact_infer_cold() -> Fault {
    plugin_internal_fault("artifact-infer not implemented for extensions (W1-A placeholder)")
}
/// 🧩️ Installs an extension crate's bundle builder into TLS for WIT guest exports.
#[macro_export]
macro_rules! extension_exports {
    ($mounted_policy:expr, $bundle_fn:expr, $plugin_fn:expr, $app:ty) => {
        $crate::component_persistent_local! {
            static __SEMIO_PENDING_EXTENSION: std::cell::RefCell<Option<$crate::ExtensionBundle>> = std::cell::RefCell::new(None);
        }
        $crate::__semio_plugin_actor_exports!(
            $mounted_policy,
            || {
                __SEMIO_PENDING_EXTENSION
                    .with(|pending| {
                        let mut pending = pending.borrow_mut();
                        if pending.is_none() {
                            *pending = Some(($bundle_fn)());
                        }
                        $crate::__async::poll::resolve_ready($crate::plugin_runtime::install_extension_bundle(&mut pending))
                    })
                    .map_err(|fault| $crate::PluginAssemblyError::new("extension.install", format!("{fault:?}")))?;
                $crate::__async::poll::resolve_ready($crate::plugin_runtime::extension_activate()).map_err(|fault| $crate::PluginAssemblyError::new("extension.activate", format!("{fault:?}")))?;
                ($plugin_fn)()
            },
            $app,
            $crate::describe::describe_extension_with_apps,
            || __SEMIO_PENDING_EXTENSION.with(|pending| {
                let mut pending = pending.borrow_mut();
                if pending.is_some() && $crate::__async::poll::resolve_ready($crate::plugin_runtime::install_extension_bundle(&mut pending)).expect("retained extension admission") {
                    $crate::__async::poll::resolve_ready($crate::plugin_runtime::extension_activate()).expect("admitted extension activation");
                }
            })
        );
    };
    ($mounted_policy:expr, $bundle_fn:expr) => {
        $crate::component_persistent_local! {
            static __SEMIO_EXTENSION_RUNTIME: $crate::plugin_runtime::PluginRuntime<$crate::app::NoPluginApp> = $crate::plugin_runtime::PluginRuntime::new($mounted_policy).expect("explicit embedding mounted policy");
            static __SEMIO_PENDING_EXTENSION: std::cell::RefCell<Option<$crate::ExtensionBundle>> = std::cell::RefCell::new(None);
        }
        fn __semio_ensure_extension_runtime() {
            static ONCE: std::sync::Once = std::sync::Once::new();
            ONCE.call_once(|| {
                $crate::plugin_runtime::install_guest_panic_report();
                __semio_install_extension_bundle();
                $crate::__async::poll::resolve_ready($crate::plugin_runtime::extension_activate()).expect("installed extension activation");
            });
            __SEMIO_PENDING_EXTENSION.with(|pending| {
                let mut pending = pending.borrow_mut();
                if pending.is_some() && $crate::__async::poll::resolve_ready($crate::plugin_runtime::install_extension_bundle(&mut pending)).expect("retained extension admission") {
                    $crate::__async::poll::resolve_ready($crate::plugin_runtime::extension_activate()).expect("admitted extension activation");
                }
            });
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        fn __semio_describe_component() -> Vec<u8> {
            $crate::__async::poll::resolve_ready($crate::describe::describe_extension())
        }
        $crate::__semio_owned_core_exports!(__SEMIO_EXTENSION_RUNTIME, __semio_ensure_extension_runtime, __semio_describe_component);
        $crate::__semio_actor_exports!(__SemioExtensionGuest, __SEMIO_EXTENSION_RUNTIME, __semio_ensure_extension_runtime, __semio_describe_component);
        // 🚫️async: E1 called from the sync `std::sync::Once` closure of `__semio_ensure_extension_runtime`.
        fn __semio_install_extension_bundle() {
            __SEMIO_PENDING_EXTENSION.with(|pending| {
                let mut pending = pending.borrow_mut();
                if pending.is_none() {
                    *pending = Some(($bundle_fn)());
                }
                $crate::__async::poll::resolve_ready($crate::plugin_runtime::install_extension_bundle(&mut pending)).expect("retained extension admission");
            });
        }
        // 🧬️ A2 (design-abi.md §1): one `world actor` for both roles now — extensions link the
        // SAME `component_export_anchor` a plugin does (the old separate `extension-world`
        // component/anchor is deleted, see `important.md`'s "Replace, never wrap" list).
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[used]
        static _SEMIO_EXTENSION_COMPONENT_LINK: fn() = $crate::component_export_anchor;
        // 🛂️ E1-describe (`📓️design-abi.md` §3) — see `plugin_exports!`'s identical test for the
        // full rationale (native install, no wasm) and for E2-builder-descriptor's path correction
        // (owner root, not `🤖️generated/` — that's gitignored, see the registrar ruling quoted
        // there). `describe::describe_extension()` is this macro's own counterpart of
        // `describe_plugin()`.
        //
        // 🚦️ D0-descriptor-plumbing ratchet — see `plugin_exports!`'s identical test for the full
        // rationale. `DESCRIPTOR_MIGRATED_EXTENSIONS` starts empty: no extension has committed a
        // descriptor yet as of this packet; extend it, never shrink it, as each extension's own
        // emission packet lands.
        $crate::__semio_extension_descriptor_fresh_test!();
    };
}
/// 🏗️ Plugin registration lives in each owner's root `🦀️.rs` via
/// [`Plugin::builder`](crate::Plugin::builder) + [`plugin_exports!`](crate::plugin_exports).
/// The retired `semio_plugin!` macro is gone — typestate on the builder makes missing identity fields a compile error.
#[cfg(test)]
include!("../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs");

/// 🐞️ `[TRACE]` last maintenance stage entered — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.
pub static LAST_MAINTENANCE_STAGE: AtomicU64 = AtomicU64::new(0);

#[cfg(test)]
#[path="🎞️command/📥️custody/🧪️tests/🦀️.rs"]
mod original_media_command_custody_tests;
