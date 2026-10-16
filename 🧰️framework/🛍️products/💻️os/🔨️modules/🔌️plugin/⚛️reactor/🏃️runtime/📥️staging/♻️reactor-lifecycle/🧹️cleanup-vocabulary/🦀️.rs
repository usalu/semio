//! 🧹️ Cleanup fault, maintenance status and close status vocabulary of the runtime lifecycle.

use super::*;
/// 🩺️ The single cause taxonomy shared by both supervisory cleanup loops (live maintenance and
/// close), replacing the bare magic `u8` fault constants: every store site names its own cause,
/// so the one decode site per loop can emit a distinct `plugin.internal.*` wire fault code
/// instead of collapsing thirteen unrelated conditions into one opaque string. The language
/// neutral examples are `🩺️runtime-fault-vectors.json`; native tests and the React window-fault
/// classifier compare their observed outcomes against those examples.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub(super) enum RuntimeCleanupFault {
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
    pub(super) slug: &'static str,
    pub(super) code: &'static str,
    pub(super) detail: &'static str,
}
pub(super) const RUNTIME_CLEANUP_FAULTS: [RuntimeCleanupFault; 19] = [
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
    pub(super) const fn vector(self) -> &'static RuntimeCleanupFaultVector {
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
pub(super) enum RuntimeMaintenanceStatus {
    Ready,
    Queued,
    Running,
    Fault(RuntimeCleanupFault),
}
const RUNTIME_MAINTENANCE_FAULT_BASE: u8 = 3;
impl RuntimeMaintenanceStatus {
    pub(super) const fn repr(self) -> u8 {
        match self {
            Self::Ready => 0,
            Self::Queued => 1,
            Self::Running => 2,
            Self::Fault(cause) => RUNTIME_MAINTENANCE_FAULT_BASE + cause as u8,
        }
    }
    pub(super) fn from_repr(repr: u8) -> Self {
        match repr {
            0 => Self::Ready,
            1 => Self::Queued,
            2 => Self::Running,
            other => Self::Fault(RuntimeCleanupFault::from_index(other - RUNTIME_MAINTENANCE_FAULT_BASE).expect("maintenance status repr is written only by this module")),
        }
    }
    #[cfg(test)]
    pub(super) const fn cause(self) -> Option<RuntimeCleanupFault> {
        match self {
            Self::Fault(cause) => Some(cause),
            _ => None,
        }
    }
}
/// 🩺️ Close-worker status stored in `RuntimeCloseWorkerState::status`, laid out like
/// {@link RuntimeMaintenanceStatus} with two extra terminal states before the fault base.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum RuntimeCloseStatus {
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
    pub(super) const fn repr(self) -> u8 {
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
    pub(super) fn from_repr(repr: u8) -> Self {
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
    pub(super) const fn cause(self) -> Option<RuntimeCleanupFault> {
        match self {
            Self::Fault(cause) => Some(cause),
            _ => None,
        }
    }
}
/// 🩺️ The one decoded wire fault both cleanup loops emit — `origin`/`code`/`severity`/`message`
/// keep their shape, the discrimination lives entirely in `code`, and the message always names
/// the instance, the variant slug, and the measured turn cost against the interactive ceiling.
pub(super) fn runtime_cleanup_fault(scope: &str, cause: RuntimeCleanupFault, instance_id: u32, elapsed_us: u64) -> Fault {
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
pub(super) const RUNTIME_CLEANUP_UNMEASURED_US: u64 = u64::MAX;
pub(super) const RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT: u32 = 256;
