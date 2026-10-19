//! ⚙️ Mounted registry pump and reaper: the pump close, the pump with its outcome and the outcome finish, the relay step, the outcome page read, the mounted reaper and the mounted future. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
impl GuestRelayMountedRegistry {
    pub(super) fn pump_close(&self,session:&mut GuestRelayMountedSession,index:usize,_waker:Option<&std::task::Waker>)->Result<GuestRelayMountedClose,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
        if !self.collect_original_wake_receipt(&mut session.wake){return Ok(GuestRelayMountedClose::Blocked{wake_registered:session.wake.registered})}
        let result=self.wake_authority.receiver.dispatch_cleanup_step(index,session.generation,|cx|{
            let grant=cx.retained_grant();let bytes=if session.checked_out.is_some(){size_of::<Option<semio_framework_job_worker::WorkerJobOutcome<GuestColdRelayJob>>>()}else if session.lifecycle_probe_checked_out.is_some(){size_of::<Option<semio_framework_job_worker::WorkerJobOutcome<GuestRelayLifecycleProbeJob>>>()}else{size_of::<GuestRelayMountedOwner>()};
            let terminal=match &session.owner{GuestRelayMountedOwner::Session(owner)=>owner.terminal_is_empty(),GuestRelayMountedOwner::LifecycleProbe(owner)=>owner.terminal_is_empty(),GuestRelayMountedOwner::Rejected(owner)=>owner.terminal_is_empty(),GuestRelayMountedOwner::Empty=>true};
            if session.checked_out.is_some()||session.lifecycle_probe_checked_out.is_some()||terminal{
                if cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(GuestRelayMountedClose::Pending)}
                cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
                if let Some(owner)=session.checked_out.take(){owner.begin_close();session.outcome_stage=GuestRelayOutcomeStage::Idle;return Ok(GuestRelayMountedClose::Pending)}
                if let Some(owner)=session.lifecycle_probe_checked_out.take(){owner.begin_close();session.outcome_stage=GuestRelayOutcomeStage::Idle;return Ok(GuestRelayMountedClose::Pending)}
                session.owner=GuestRelayMountedOwner::Empty;return Ok(GuestRelayMountedClose::Complete)
            }
            let step=match &mut session.owner{
                GuestRelayMountedOwner::Session(owner)=>owner.close_step(cx)?,
                GuestRelayMountedOwner::LifecycleProbe(owner)=>owner.close_step(cx)?,
                GuestRelayMountedOwner::Rejected(owner)=>match owner.close_step(cx)?{semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{progress},semio_framework_job::InteractiveJobCloseStep::Blocked=>semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked},
                GuestRelayMountedOwner::Empty=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original mounted close lost its live owner").into()),
            };
            Ok(match step{semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{..}|semio_framework_job_worker_authority::WorkerJobCloseStep::Complete{..}=>GuestRelayMountedClose::Pending,semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked=>GuestRelayMountedClose::Blocked{wake_registered:session.wake.registered}})
        });
        if result.as_ref().is_ok_and(|value|value.is_none()){return Ok(GuestRelayMountedClose::Blocked{wake_registered:session.wake.registered})}
        let returned=self.wake_authority.receiver.finish_turn(index,session.generation);
        match(result,returned){(Ok(Some(step)),Ok(_))=>Ok(step),(Err(mut failure),Err(receipt))=>{failure.context_refusal=Some(receipt);Err(failure)},(Err(failure),Ok(_))=>Err(failure),(Ok(_),Err(receipt))=>Err(receipt.into()),(Ok(None),Ok(_))=>unreachable!()}
    }

    pub(crate) fn pump(self: &Arc<Self>, index: usize, generation: u64, waker: &std::task::Waker) -> std::task::Poll<Result<Vec<u8>, PluginHostError>> {
        let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let GuestRelayMountedSlot::Mounted(session) = &mut *slot else {
            return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay slot is not mounted".into())));
        };
        if session.generation != generation {
            return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay generation is stale".into())));
        }
        if matches!(session.lifecycle,GuestRelayMountedLifecycle::Running){
            let admitted=match &session.owner{GuestRelayMountedOwner::Session(owner)=>self.admit_original_session_wake(owner,&mut session.wake,index,generation,waker),GuestRelayMountedOwner::LifecycleProbe(owner)=>self.admit_original_session_wake(owner,&mut session.wake,index,generation,waker),GuestRelayMountedOwner::Rejected(_)|GuestRelayMountedOwner::Empty=>true};
            if !admitted{return std::task::Poll::Pending}
        }else if !self.collect_original_wake_receipt(&mut session.wake){return std::task::Poll::Pending}
        if matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap) {
            return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay ownership is detached".into())));
        }
        if matches!(session.lifecycle, GuestRelayMountedLifecycle::DrainingForCaller) {
            match self.pump_close(session,index,Some(waker)) {
                Ok(GuestRelayMountedClose::Complete) => {
                    let mounted = std::mem::replace(&mut *slot, GuestRelayMountedSlot::Empty);
                    let GuestRelayMountedSlot::Mounted(mut session) = mounted else { unreachable!("mounted relay slot remains mounted through close") };
                    let result = match session.terminal.take() {
                        Some(GuestRelayMountedTerminal::Complete) => Ok(session.output.into_vec()),
                        Some(GuestRelayMountedTerminal::Cancelled) => Err(PluginHostError::Plugin("plugin cold relay cancelled".into())),
                        Some(GuestRelayMountedTerminal::Fault) => Err(PluginHostError::Plugin(String::from_utf8_lossy(session.output.bytes()).into_owned())),
                        Some(GuestRelayMountedTerminal::HostFault(error)) => Err(error),
                        None => Err(PluginHostError::Plugin("plugin cold relay closed".into())),
                    };
                    return std::task::Poll::Ready(result);
                }
                Ok(GuestRelayMountedClose::Pending) => waker.wake_by_ref(),
                Ok(GuestRelayMountedClose::Blocked { wake_registered: false }) => self.schedule_reaper_timer(waker),
                Ok(GuestRelayMountedClose::Blocked { wake_registered: true }) => {},
                Err(failure)=>return std::task::Poll::Ready(Err(PluginHostError::OriginalTurn(failure)))
            }
            return std::task::Poll::Pending;
        }
        if session.checked_out.is_some() || session.lifecycle_probe_checked_out.is_some() {
            return self.pump_outcome(session, index, generation, waker);
        }
        let Some(context) = self.checkout_original_wake(&mut session.wake, index, generation) else {
            return std::task::Poll::Pending;
        };
        let (grant, epoch) = (context.grant, context.epoch);
        let stepped = match &session.owner {
            GuestRelayMountedOwner::LifecycleProbe(owner) => match step_guest_relay_owner(owner, grant) {
                GuestRelayStep::Checked(checked, receipt) => {
                    session.lifecycle_probe_checked_out = Some(checked);
                    receipt
                }
                GuestRelayStep::Idle(receipt) => receipt,
                GuestRelayStep::Unavailable(detail) => return std::task::Poll::Ready(Err(PluginHostError::Plugin(detail.into()))),
            },
            GuestRelayMountedOwner::Session(owner) => match step_guest_relay_owner(owner, grant) {
                GuestRelayStep::Checked(checked, receipt) => {
                    session.checked_out = Some(checked);
                    receipt
                }
                GuestRelayStep::Idle(receipt) => receipt,
                GuestRelayStep::Unavailable(detail) => return std::task::Poll::Ready(Err(PluginHostError::Plugin(detail.into()))),
            },
            GuestRelayMountedOwner::Rejected(_) | GuestRelayMountedOwner::Empty => {
                session.wake.receipt = Some(GuestRelayWakeReceipt { epoch, slot: index, generation, grant, progress: Default::default() });
                self.collect_original_wake_receipt(&mut session.wake);
                if matches!(session.owner, GuestRelayMountedOwner::Rejected(_)) {
                    let _ = self.pump_close(session, index, Some(waker));
                }
                return std::task::Poll::Pending;
            }
        };
        let progress = stepped.map_or(Default::default(), |(_, progress)| progress);
        if !progress.fits(grant) {
            session.wake.refusal = Some(semio_framework_value::ValueRefusalKind::InvariantViolated);
        }
        session.wake.receipt = Some(GuestRelayWakeReceipt { epoch, slot: index, generation, grant, progress });
        self.collect_original_wake_receipt(&mut session.wake);
        if session.checked_out.is_some() || session.lifecycle_probe_checked_out.is_some() {
            session.outcome_stage = GuestRelayOutcomeStage::Reading;
            session.outcome_kind = GuestRelayOutcomeKind::Continues;
            session.outcome_page = 0;
            waker.wake_by_ref();
        }
        std::task::Poll::Pending
    }

    /// 📖️ One paid turn of the checked-out outcome: read its pages once, acknowledge the descriptor, then resume or drain the terminal.
    fn pump_outcome(&self, session: &mut GuestRelayMountedSession, index: usize, generation: u64, waker: &std::task::Waker) -> std::task::Poll<Result<Vec<u8>, PluginHostError>> {
        use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep};
        let Some(context) = self.checkout_original_wake(&mut session.wake, index, generation) else {
            return std::task::Poll::Pending;
        };
        let grant = context.grant;
        let mut progress = RetainedCloneProgress::default();
        match session.outcome_stage {
            GuestRelayOutcomeStage::Idle | GuestRelayOutcomeStage::Reading => {
                let read = match (session.checked_out.as_ref(), session.lifecycle_probe_checked_out.as_ref()) {
                    (Some(checked), _) => read_guest_relay_outcome_page(checked, session.outcome_page),
                    (None, Some(checked)) => read_guest_relay_outcome_page(checked, session.outcome_page),
                    (None, None) => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted relay lost its checked-out outcome")),
                };
                match read {
                    Err(error) => {
                        session.wake.refusal = Some(error.kind);
                        progress = error.retained_progress();
                    }
                    Ok((Some(page), kind)) => {
                        let bytes = page.len() + 2 * size_of::<usize>();
                        if grant.maximum_items > 0 && grant.maximum_copy_bytes >= bytes && grant.maximum_depth > 0 && session.output.length.checked_add(page.len()).is_some_and(|length| length <= semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES) {
                            if matches!(kind, GuestRelayOutcomeKind::Complete | GuestRelayOutcomeKind::Fault) {
                                session.output.write_page(page).expect("original output capacity was admitted before page effects");
                            }
                            session.outcome_page += 1;
                            progress = RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() };
                        }
                    }
                    Ok((None, kind)) => {
                        if grant.maximum_items > 0 && grant.maximum_depth > 0 {
                            session.outcome_kind = kind;
                            session.outcome_stage = GuestRelayOutcomeStage::Acknowledging;
                            progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
                        }
                    }
                }
            }
            GuestRelayOutcomeStage::Acknowledging => {
                let step = match (session.checked_out.as_mut(), session.lifecycle_probe_checked_out.as_mut()) {
                    (Some(checked), _) => checked.acknowledge_outcome(grant),
                    (None, Some(checked)) => checked.acknowledge_outcome(grant),
                    (None, None) => RetainedCloneStep::Progress(Default::default()),
                };
                progress = step.progress();
                if matches!(step, RetainedCloneStep::Complete(_)) {
                    self.finish_guest_relay_outcome(session);
                }
            }
        }
        if !progress.fits(grant) {
            session.wake.refusal = Some(semio_framework_value::ValueRefusalKind::InvariantViolated);
        }
        session.wake.receipt = Some(GuestRelayWakeReceipt { epoch: context.epoch, slot: index, generation, grant, progress });
        self.collect_original_wake_receipt(&mut session.wake);
        if progress.copied_items > 0 {
            waker.wake_by_ref();
        }
        std::task::Poll::Pending
    }

    /// 🔚️ A continuing outcome resumes its worker; a terminal one becomes the result the caller drains while the session closes.
    fn finish_guest_relay_outcome(&self, session: &mut GuestRelayMountedSession) {
        match session.outcome_kind {
            GuestRelayOutcomeKind::Continues => {
                if let Some(checked) = session.checked_out.take() {
                    if let Err(checked) = checked.resume() {
                        session.checked_out = Some(checked);
                        return;
                    }
                } else if let Some(checked) = session.lifecycle_probe_checked_out.take() {
                    if let Err(checked) = checked.resume() {
                        session.lifecycle_probe_checked_out = Some(checked);
                        return;
                    }
                }
            }
            terminal => {
                session.terminal = Some(match terminal {
                    GuestRelayOutcomeKind::Complete => GuestRelayMountedTerminal::Complete,
                    GuestRelayOutcomeKind::Cancelled => GuestRelayMountedTerminal::Cancelled,
                    _ => GuestRelayMountedTerminal::Fault,
                });
                session.lifecycle = GuestRelayMountedLifecycle::DrainingForCaller;
                if let Some(checked) = session.checked_out.take() {
                    checked.begin_close();
                } else if let Some(checked) = session.lifecycle_probe_checked_out.take() {
                    checked.begin_close();
                }
            }
        }
        session.outcome_stage = GuestRelayOutcomeStage::Idle;
        session.outcome_page = 0;
    }
}

/// 🪜️ One caller-driven worker step of a mounted relay owner, with its physical receipt taken exactly once.
pub(super) enum GuestRelayStep<J> {
    Checked(semio_framework_job_worker::WorkerJobOutcome<J>, Option<(semio_framework_value::RetainedCloneGrant, semio_framework_value::RetainedCloneProgress)>),
    Idle(Option<(semio_framework_value::RetainedCloneGrant, semio_framework_value::RetainedCloneProgress)>),
    Unavailable(&'static str),
}

pub(super) fn step_guest_relay_owner<J: semio_framework_job::InteractiveJob + 'static>(owner: &semio_framework_job_worker::WorkerJobSession<J>, grant: semio_framework_value::RetainedCloneGrant) -> GuestRelayStep<J> {
    let checked = match owner.try_step_on_caller(grant) {
        Ok((ticket, semio_framework_job_worker::WorkerJobPoll::Outcome)) => match owner.take_outcome(ticket) {
            Ok(checked) => checked,
            Err(_) => return GuestRelayStep::Unavailable("plugin cold relay outcome unavailable"),
        },
        Ok((_, semio_framework_job_worker::WorkerJobPoll::Terminal)) => match owner.take_terminal() {
            Ok(checked) => checked,
            Err(_) => return GuestRelayStep::Unavailable("plugin cold relay terminal unavailable"),
        },
        Ok(_) | Err(_) => return GuestRelayStep::Idle(owner.take_retained_step_receipt().ok().flatten()),
    };
    let mut checked = checked;
    let receipt = checked.take_retained_step_receipt();
    GuestRelayStep::Checked(checked, receipt)
}

/// 📄️ Borrows one page of the checked-out outcome's payload and classifies the outcome.
pub(super) fn read_guest_relay_outcome_page<J: semio_framework_job::InteractiveJob>(checked: &semio_framework_job_worker::WorkerJobOutcome<J>, page: usize) -> Result<(Option<&[u8]>, GuestRelayOutcomeKind), semio_framework_value::ValueError> {
    use semio_framework_job::JobOutcomeView as V;
    let view = checked.outcome()?.ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted relay outcome is absent"))?;
    let (payload, kind) = match view {
        V::Yield { .. } => (None, GuestRelayOutcomeKind::Continues),
        V::PreviewReady { payload, .. } => (Some(payload), GuestRelayOutcomeKind::Continues),
        V::CheckpointReady { state, .. } => (Some(state), GuestRelayOutcomeKind::Continues),
        V::Complete { output, .. } => (output, GuestRelayOutcomeKind::Complete),
        V::Cancelled { .. } => (None, GuestRelayOutcomeKind::Cancelled),
        V::Fault { detail, .. } => (Some(detail), GuestRelayOutcomeKind::Fault),
    };
    Ok((payload.and_then(|payload| payload.page(page)), kind))
}

pub(super) enum GuestRelayMountedReap {
    Idle,
    Progress,
    Blocked { wake_registered: bool },
}

pub(super) struct GuestRelayMountedReaper {
    pub(super) registry: Arc<GuestRelayMountedRegistry>,
}

impl std::future::Future for GuestRelayMountedReaper {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        #[cfg(any(test, feature = "testkit"))]
        {
            self.registry.reaper_polls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        let observed_epoch = self.registry.reaper_epoch.load(std::sync::atomic::Ordering::Acquire);
        match self.registry.reap_one(context.waker()) {
            GuestRelayMountedReap::Progress => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            GuestRelayMountedReap::Blocked { wake_registered } => {
                self.registry.park_blocked_reaper(context.waker(), wake_registered);
                std::task::Poll::Pending
            }
            GuestRelayMountedReap::Idle => {
                self.registry.reaper_active.store(false, std::sync::atomic::Ordering::Release);
                if self.registry.reaper_epoch.load(std::sync::atomic::Ordering::Acquire) != observed_epoch || self.registry.has_detached() {
                    self.registry.ensure_reaper();
                }
                std::task::Poll::Ready(())
            }
        }
    }
}

pub(super) struct GuestRelayMountedFuture {
    pub(super) registry: Arc<GuestRelayMountedRegistry>,
    pub(super) index: usize,
    pub(super) generation: u64,
    pub(super) complete: bool,
}

impl std::future::Future for GuestRelayMountedFuture {
    type Output = Result<Vec<u8>, PluginHostError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let poll = self.registry.pump(self.index, self.generation, context.waker());
        if poll.is_ready() {
            self.complete = true;
        }
        poll
    }
}

impl Drop for GuestRelayMountedFuture {
    fn drop(&mut self) {
        if !self.complete {
            self.registry.detach(self.index, self.generation);
        }
    }
}
//#endregion 🔀️PostTurnRelay
