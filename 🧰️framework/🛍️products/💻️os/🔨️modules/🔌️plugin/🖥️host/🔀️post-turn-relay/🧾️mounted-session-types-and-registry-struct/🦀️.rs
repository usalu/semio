//! 🧾️ Mounted session types and registry struct: the mounted slots and reaper fallback, the session admission, the mounted owner, the wake authority and custody, the mounted session, its stage, kind and lifecycle enums, the output, terminal, close and slot and the registry struct. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
pub(super) const GUEST_RELAY_MOUNTED_SLOTS: usize = 16;

pub(super) const GUEST_RELAY_REAPER_FALLBACK_MS: u64 = 8;

/// 🚪️ Admits one session from the registry's own drive grant; a refusal keeps the original job and parameters.
pub(super) fn admit_guest_relay_session<J: semio_framework_job::InteractiveJob + 'static>(job: J, params: semio_framework_job_worker_authority::BatchJobParams, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_job_worker::WorkerJobSession<J>, GuestRelayRefusedSource<J>> {
    let mut job = Some(job);
    let mut params = Some(params);
    let original = params.as_ref().expect("original session parameters");
    let (operation, generation, now_us) = (original.operation, original.generation, original.now_us);
    let mut recipient = semio_framework_value::RetainedCloneProgress::default();
    match semio_framework_job_worker_authority::WorkerJobAdmissionContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, grant), now_us, &mut recipient) {
        Ok(mut control) => match semio_framework_job_worker::WorkerJobSession::try_admit_owned(&mut job, &mut params, &mut control) {
            Ok(Some((session, _admission))) => Ok(session),
            Ok(None) | Err(_) => Err(GuestRelayRefusedSource { job, params, closing: false }),
        },
        Err(_) => Err(GuestRelayRefusedSource { job, params, closing: false }),
    }
}

#[expect(clippy::large_enum_variant, reason = "Failed worker admission must retain the exact job and credits for bounded retirement without allocating on rejection.")]
pub(super) enum GuestRelayMountedOwner {
    Session(semio_framework_job_worker::WorkerJobSession<GuestColdRelayJob>),
    Rejected(GuestRelayRefusedSource<GuestColdRelayJob>),
    LifecycleProbe(semio_framework_job_worker::WorkerJobSession<GuestRelayLifecycleProbeJob>),
    Empty,
}

/// 🚦️ Required first-party issuer and receipt receiver belong to the actual Host embedding.
#[derive(Clone)]
pub struct GuestRelayWakeAuthority{pub drive_policy:semio_framework_value::retained_clone::RetainedCloneGrant,pub policy:semio_framework_value::retained_clone::RetainedCloneGrant,pub issuer:semio_framework_job_worker::OriginalWorkerWakeIssuer,pub receiver:Arc<GuestRelayWakeDriver>}

pub(super) struct GuestRelayWakeCustody{pub(super) original:std::mem::ManuallyDrop<Option<Box<dyn semio_framework_job_worker::RetainedWorkerWake>>>,pub(super) receipt:Option<GuestRelayWakeReceipt>,pub(super) registered:bool,pub(super) refusal:Option<semio_framework_value::ValueRefusalKind>}

impl GuestRelayWakeCustody{pub(super) fn new()->Self{Self{original:std::mem::ManuallyDrop::new(None),receipt:None,registered:false,refusal:None}}}

impl Drop for GuestRelayWakeCustody{fn drop(&mut self){assert!(std::thread::panicking()||(self.original.is_none()&&self.receipt.is_none()),"mounted relay abandoned original wake frame or receipt");if self.original.is_none(){unsafe{std::mem::ManuallyDrop::drop(&mut self.original);}}}}

pub(super) struct GuestRelayMountedSession {
    pub(super) wake:GuestRelayWakeCustody,
    pub(super) generation: u64,
    pub(super) owner: GuestRelayMountedOwner,
    pub(super) checked_out: Option<semio_framework_job_worker::WorkerJobOutcome<GuestColdRelayJob>>,
    pub(super) lifecycle_probe_checked_out: Option<semio_framework_job_worker::WorkerJobOutcome<GuestRelayLifecycleProbeJob>>,
    pub(super) outcome_stage: GuestRelayOutcomeStage,
    pub(super) outcome_kind: GuestRelayOutcomeKind,
    pub(super) outcome_page: usize,
    pub(super) output: GuestRelayMountedOutput,
    pub(super) terminal: Option<GuestRelayMountedTerminal>,
    pub(super) lifecycle: GuestRelayMountedLifecycle,
}

/// 🚦️ Where the checked-out semantic outcome of a mounted relay is: its pages are read once, then acknowledged, then handed back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GuestRelayOutcomeStage {
    Idle,
    Reading,
    Acknowledging,
}

/// 🏁️ What the lent outcome means for the relay: it goes on, or it is the terminal result the caller drains.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GuestRelayOutcomeKind {
    Continues,
    Complete,
    Cancelled,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GuestRelayMountedLifecycle {
    Running,
    DrainingForCaller,
    DetachedForReap,
}

pub(super) struct GuestRelayMountedOutput {
    pub(super) storage: Box<[std::mem::MaybeUninit<u8>; semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES]>,
    pub(super) length: usize,
}

impl GuestRelayMountedOutput {
    pub(crate) fn new() -> Self {
        let storage = Box::<[std::mem::MaybeUninit<u8>; semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES]>::new_uninit();
        Self { storage: unsafe { storage.assume_init() }, length: 0 }
    }

    pub(super) fn write_page(&mut self, page: &[u8]) -> Result<(), PluginHostError> {
        let end = self.length.checked_add(page.len()).filter(|end| *end <= semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES).ok_or_else(|| PluginHostError::Plugin("plugin cold relay mounted output exceeds its admitted authority".into()))?;
        for (target, byte) in self.storage[self.length..end].iter_mut().zip(page.iter().copied()) {
            target.write(byte);
        }
        self.length = end;
        Ok(())
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.storage.as_ptr().cast::<u8>(), self.length) }
    }

    pub(super) fn into_vec(self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

pub(super) enum GuestRelayMountedTerminal {
    Complete,
    Cancelled,
    Fault,
    HostFault(PluginHostError),
}

pub(super) enum GuestRelayMountedClose {
    Complete,
    Pending,
    Blocked { wake_registered: bool },
}

#[expect(clippy::large_enum_variant, reason = "The fixed registry allocates all slot storage before admission, including retained rejection and terminal owners.")]
pub(super) enum GuestRelayMountedSlot {
    Empty,
    Reserved { generation: u64, output: GuestRelayMountedOutput },
    Mounted(GuestRelayMountedSession),
}

pub(super) struct GuestRelayMountedRegistry {
    pub(super) wake_authority:GuestRelayWakeAuthority,
    pub(super) slots: Box<[Mutex<GuestRelayMountedSlot>]>,
    pub(super) next_generation: std::sync::atomic::AtomicU64,
    pub(super) pool: WorkerPool,
    pub(super) reaper_cursor: std::sync::atomic::AtomicUsize,
    pub(super) reaper_epoch: std::sync::atomic::AtomicU64,
    pub(super) reaper_active: AtomicBool,
    pub(super) reaper_failed: AtomicBool,
    pub(super) reaper_timer_pending: AtomicBool,
    #[cfg(any(test, feature = "testkit"))]
    pub(super) reaper_polls: std::sync::atomic::AtomicUsize,
    #[cfg(any(test, feature = "testkit"))]
    pub(super) reaper_timers: std::sync::atomic::AtomicUsize,
    #[cfg(any(test, feature = "testkit"))]
    pub(super) reaper_wake_waits: std::sync::atomic::AtomicUsize,
}
//#endregion 🔀️PostTurnRelay
