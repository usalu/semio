//! 🧾️ Mounted session types and registry struct: the mounted slots and reaper fallback, the session admission, the mounted owner, the wake authority and custody, the mounted session, its stage, kind and lifecycle enums, the output, terminal, close and slot and the registry struct. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
pub(super) const GUEST_RELAY_MOUNTED_SLOTS: usize = 16;

pub(super) const GUEST_RELAY_REAPER_FALLBACK_MS: u64 = 8;

/// 🚪️ See `README.md`.
pub(super) struct GuestRelayAdmission<J: semio_framework_job::InteractiveJob + 'static> {
    job: Option<J>,
    params: Option<semio_framework_job_worker_authority::BatchJobParams>,
    alias: Option<semio_framework_async::CancelToken>,
    alias_return: Option<semio_framework_async::CancelTokenRetirement>,
    config: semio_framework_job_worker_authority::BatchDriveConfig,
    admission: semio_framework_job_worker::WorkerJobSessionAdmission<J, semio_framework_job_worker::WorkerJobSession<J>>,
    session: Option<semio_framework_job_worker::WorkerJobSession<J>>,
}

impl<J: semio_framework_job::InteractiveJob + 'static> GuestRelayAdmission<J> {
    pub(super) fn new(job: J, config: semio_framework_job_worker_authority::BatchDriveConfig) -> Self {
        Self { job: Some(job), params: None, alias: None, alias_return: None, config, admission: semio_framework_job_worker::WorkerJobSessionAdmission::new(), session: None }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.params.is_none() && self.alias.is_none() && self.alias_return.is_none() && self.admission.terminal_is_empty() && self.session.is_none()
    }

    pub(super) fn begin_close(&mut self) {
        if let Some(job) = self.job.as_mut() {
            job.begin_close();
        }
        if let Some(session) = self.session.as_ref() {
            let _ = session.begin_close();
        }
    }

    /// 🌱️ See `README.md`.
    pub(super) fn advance(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.session.is_some() {
            return Ok(true);
        }
        if self.params.is_none() && self.job.is_some() {
            if self.alias.is_none() {
                cx.admit_original_cancel_alias(&mut self.alias)?;
                return Ok(false);
            }
            let cancel = self.alias.take().ok_or_else(|| semio_framework_value::ValueError::invariant("relay admission alias absent"))?;
            self.params = Some(semio_framework_job_worker_authority::BatchJobParams { operation: cx.operation(), generation: cx.generation(), cancel, config: self.config, now_us: cx.clock_source() });
        }
        self.admission.advance(&mut self.job, &mut self.params, &mut self.session, cx)
    }

    /// 🤝️ See `README.md`.
    pub(super) fn take_born(&mut self) -> Option<semio_framework_job_worker::WorkerJobSession<J>> {
        if self.job.is_none() && self.params.is_none() && self.alias.is_none() && self.alias_return.is_none() && self.admission.terminal_is_empty() {
            self.session.take()
        } else {
            None
        }
    }

    pub(super) fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_job_worker_authority::WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::InteractiveJobCloseStep as I;
        use semio_framework_job_worker_authority::WorkerJobCloseStep as W;
        use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome};
        let grant = cx.retained_grant();
        if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(W::Pending { progress: Default::default() });
        }
        self.begin_close();
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let metadata = |cx: &mut semio_framework_job::StepContext<'_>| semio_framework_job::receive_original_retained_work(cx, RetainedWorkOutcome::<_, semio_framework_value::ValueError> { result: Ok(RetainedCloneStep::Progress(item)), work: 1 });
        if !self.admission.terminal_is_empty() {
            return Ok(match self.admission.close_step(cx)? {
                I::Pending { progress } | I::Complete { progress } => W::Pending { progress },
                I::Blocked => W::Blocked,
            });
        }
        if let Some(session) = self.session.as_mut() {
            if !session.terminal_is_empty() {
                return close_relay_session(session, cx);
            }
            drop(self.session.take());
            return Ok(W::Pending { progress: metadata(cx)?.progress() });
        }
        if self.alias_return.is_none() && (self.alias.is_some() || self.params.is_some()) {
            let cancel = match (self.alias.take(), self.params.take()) {
                (Some(cancel), _) => cancel,
                (None, Some(params)) => params.cancel,
                (None, None) => return Err(semio_framework_value::ValueError::invariant("relay admission alias absent").into()),
            };
            self.alias_return = Some(semio_framework_async::CancelTokenRetirement::from_token(cancel));
            return Ok(W::Pending { progress: metadata(cx)?.progress() });
        }
        if let Some(retirement) = self.alias_return.as_mut() {
            let turn = cx.with_original_effect(grant, |cx, effect| retirement.return_alias_step(cx.original_cancel_token(), effect))?.map_or(RetainedWorkOutcome { result: Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())), work: 0 }, |(turn, _)| turn);
            let result = turn.result.map_err(|error| match error {
                semio_framework_async::CancelTokenRetirementError::Refused(error) => error,
                semio_framework_async::CancelTokenRetirementError::Blocked(_) => semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "relay admission cancellation alias remains blocked"),
            });
            let step = semio_framework_job::receive_original_retained_work(cx, RetainedWorkOutcome { result, work: turn.work })?;
            if retirement.terminal_is_empty() {
                drop(self.alias_return.take());
            }
            return Ok(W::Pending { progress: step.progress() });
        }
        if let Some(job) = self.job.as_mut() {
            if job.terminal_is_empty() {
                drop(self.job.take());
                return Ok(W::Pending { progress: metadata(cx)?.progress() });
            }
            return Ok(match job.close_step(cx)? {
                I::Pending { progress } | I::Complete { progress } => W::Pending { progress },
                I::Blocked => W::Blocked,
            });
        }
        Ok(W::Complete { progress: Default::default() })
    }
}

/// 🪢️ See `README.md`.
pub(super) fn close_relay_session<J: semio_framework_job::InteractiveJob + 'static>(session: &mut semio_framework_job_worker::WorkerJobSession<J>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_job_worker_authority::WorkerJobCloseStep, semio_framework_job::OriginalTurnFailure> {
    use semio_framework_job_worker::WorkerJobDemandError;
    use semio_framework_job_worker_authority::WorkerJobCloseStep as W;
    use semio_framework_value::{RetainedCloneStep, RetainedWorkOutcome};
    let grant = cx.retained_grant();
    if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
        return Ok(W::Pending { progress: Default::default() });
    }
    match session.has_original_cancel_alias_witness(cx.original_cancel_token()) {
        Err(WorkerJobDemandError::Refused(error)) => Err(semio_framework_job::OriginalTurnFailure::debit_original(error, cx)),
        Err(WorkerJobDemandError::Contention(_)) => Ok(W::Blocked),
        Ok(false) => session.close_step(cx),
        Ok(true) => {
            let turn = cx.with_original_effect(grant,|cx,effect|session.return_original_cancel_alias_step(cx.original_cancel_token(),effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},|(turn,_)|turn);
            let returned = matches!(&turn.result, Ok(Some(_)));
            let result = match turn.result {
                Ok(Some(step)) => Ok(step),
                Ok(None) | Err(WorkerJobDemandError::Contention(_)) => Ok(RetainedCloneStep::Progress(Default::default())),
                Err(WorkerJobDemandError::Refused(error)) => Err(error),
            };
            let step = semio_framework_job::receive_original_retained_work(cx, RetainedWorkOutcome { result, work: turn.work })?;
            Ok(if returned { W::Pending { progress: step.progress() } } else { W::Blocked })
        }
    }
}

#[expect(clippy::large_enum_variant, reason = "The fixed registry retains every session birth and born session in place without allocating on rejection.")]
pub(super) enum GuestRelayMountedOwner {
    Admitting(GuestRelayAdmission<GuestColdRelayJob>),
    Session(semio_framework_job_worker::WorkerJobSession<GuestColdRelayJob>),
    AdmittingProbe(GuestRelayAdmission<GuestRelayLifecycleProbeJob>),
    LifecycleProbe(semio_framework_job_worker::WorkerJobSession<GuestRelayLifecycleProbeJob>),
    Empty,
}

/// 🚦️ See `README.md`.
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

/// 🚦️ See `README.md`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GuestRelayOutcomeStage {
    Idle,
    Reading,
    Acknowledging,
}

/// 🏁️ See `README.md`.
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
