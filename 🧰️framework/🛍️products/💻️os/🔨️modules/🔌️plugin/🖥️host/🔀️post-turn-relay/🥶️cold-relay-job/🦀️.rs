//! 🥶️ Cold relay job: the guest cold relay job, its interactive job implementation and its drop. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
pub(super) struct GuestColdRelayJob {
    pub(super) runtime: Arc<GuestRuntimes>,
    pub(super) instance: Arc<Mutex<GuestInstanceSlot>>,
    pub(super) instance_gate: Arc<semio_framework_async::Semaphore>,
    pub(super) pool: WorkerPool,
    pub(super) cancel: semio_framework_async::CancelToken,
    pub(super) cancel_scheduled: Arc<AtomicBool>,
    pub(super) cancel_admitted: Arc<AtomicBool>,
    pub(super) job: u64,
    start: Option<(String, Vec<u8>)>,
    pub(super) pending: Option<Arc<GuestRelayCompletionSlot>>,
    pub(super) publication: Option<GuestRelayPublication>,
    pub(super) closing: bool,
    start_submitted: bool,
    started: bool,
    cleanup_required: bool,
    terminal_delivered: bool,
}

impl GuestColdRelayJob {
    fn original_close_demand(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        let Some(publication)=self.publication.as_ref()else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Host relay runtime, Pool, completion and cancellation owners require native retirement"));};
        if publication.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<GuestRelayPublication>>(),depth:1,..Default::default()});}
        let mut demand=publication.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Host relay publication depth overflow"))?;Ok(demand)
    }

    pub(crate) fn submit(&mut self, request: GuestRelayRequest) {
        let request_kind = request.kind();
        let slot = Arc::new(GuestRelayCompletionSlot::new(self.pool.clone()));
        let attempt = GuestRelayAttempt {
            runtime: Arc::clone(&self.runtime),
            instance: Arc::clone(&self.instance),
            instance_gate: Arc::clone(&self.instance_gate),
            pool: self.pool.clone(),
            cancel_scheduled: Arc::clone(&self.cancel_scheduled),
            cancel_admitted: Arc::clone(&self.cancel_admitted),
            job: self.job,
            sender: GuestRelayCompletionSender::new(Arc::clone(&slot)),
        };
        let recovery = attempt.clone();
        slot.admit(GuestRelayInlineRequest { future: Box::pin(run_guest_relay_request(attempt, self.cancel.clone(), request)), failure: Some(Box::new(move |failure| recover_guest_relay_failure(recovery, request_kind, failure))) });
        self.pending = Some(slot);
    }

    fn submit_cleanup(&mut self) {
        self.cancel.cancel_now();
        if self.start_submitted && self.pending.is_none() && self.cancel_scheduled.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
            self.submit(GuestRelayRequest::Cancel);
        }
    }

}

impl semio_framework_job::InteractiveJob for GuestColdRelayJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::JobOutcomeBorrow;
        if self.publication.as_ref().is_some_and(|publication| publication.delivered) {
            if self.publication.as_ref().is_some_and(GuestRelayPublication::is_terminal) {
                return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            let retired = self.publication.as_mut().expect("delivered guest relay publication").retire(context)?;
            if retired {
                let bytes = size_of::<Option<GuestRelayPublication>>();
                let grant = context.retained_grant();
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth < 1 {
                    return Ok(None);
                }
                context.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                self.publication = None;
            }
            return Ok(None);
        }
        if self.publication.is_some() {
            let terminal = self.publication.as_ref().is_some_and(GuestRelayPublication::is_terminal);
            let outcome = self.publication.as_mut().expect("guest relay publication was just observed").step(context)?;
            if outcome.is_some() && terminal {
                self.terminal_delivered = true;
            }
            return Ok(outcome);
        }
        if self.terminal_delivered {
            return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if context.is_cancelled() || self.cancel.is_cancelled_now() {
            self.cancel.cancel_now();
            if self.pending.is_none() && self.cleanup_required {
                self.submit_cleanup();
                return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            if self.pending.is_none() {
                self.terminal_delivered = true;
                return JobOutcomeBorrow::admit_cancelled(context).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
        }
        if let Some(slot) = &self.pending {
            let Some(completion) = slot.try_take() else { return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from) };
            self.pending = None;
            match completion {
                GuestRelayCompletion::Started(Ok(())) => {
                    self.started = true;
                }
                GuestRelayCompletion::Started(Err(error)) | GuestRelayCompletion::Stepped(Err(error)) => {
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.to_string().into_bytes()));
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Running { progress: Some(progress) })) => {
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Preview, progress.into_source()));
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Running { progress: None })) => {}
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Done { output })) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Commit, output.into_source()));
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Failed { error })) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                }
                GuestRelayCompletion::Rejected(error) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                }
                GuestRelayCompletion::Fault(error) => {
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                }
                GuestRelayCompletion::TerminalFault(error) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                }
                GuestRelayCompletion::Cancelled => {
                    self.cleanup_required = false;
                    self.terminal_delivered = true;
                    return JobOutcomeBorrow::admit_cancelled(context).map_err(semio_framework_job::OriginalTurnFailure::from);
                }
            }
            return JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if let Some((kind, input)) = self.start.take() {
            self.start_submitted = true;
            self.cleanup_required = true;
            self.submit(GuestRelayRequest::Start { kind, input });
        } else if self.started {
            self.submit(GuestRelayRequest::Step);
        } else {
            self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, b"plugin guest relay lost its start state".to_vec()));
        }
        JobOutcomeBorrow::admit_yield(context).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ => self.publication.as_ref().ok_or_else(|| GuestRelayPublication::invalid("guest relay descriptor requires its publication"))?.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        self.cancel.cancel_now();
        if let Some(publication) = self.publication.as_mut() {
            publication.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as S;
        let grant=cx.retained_grant();
        if cx.should_yield(){return Ok(S::Pending{progress:Default::default()})}
        if !self.closing{self.begin_close();}
        if let Some(publication)=self.publication.as_mut(){
            if publication.terminal_is_empty(){
                let bytes=size_of::<Option<GuestRelayPublication>>();
                if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth<1{return Ok(S::Pending{progress:Default::default()});}
                let progress=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};cx.consume_retained(progress)?;
                drop(self.publication.take());cx.consume_fuel(1);return Ok(S::Pending{progress});
            }
            if grant.maximum_depth==0{return Ok(S::Pending{progress:Default::default()});}
            return receive_relay_close(publication.close_step(semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|match step{S::Complete{progress}=>S::Pending{progress},step=>step}),cx);
        }
        Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Host relay native retirement is undeclared"),cx))
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demand()?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demand()?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demand()?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demand()?.depth)}

    fn terminal_is_empty(&self) -> bool {
        false
    }
}

impl Drop for GuestColdRelayJob {
    fn drop(&mut self) {
        if self.cleanup_required || self.start.is_some() || self.pending.is_some() || self.publication.as_ref().is_some_and(|publication| !publication.terminal_is_empty()) {
            self.cancel.cancel_now();
            self.closing = true;
        }
        if let Some(slot) = self.pending.take() {
            slot.abandon();
        }
    }
}

impl GuestColdRelayJob {
    pub(crate) fn new(runtime: Arc<GuestRuntimes>, instance: Arc<Mutex<GuestInstanceSlot>>, instance_gate: Arc<semio_framework_async::Semaphore>, pool: WorkerPool, cancel: semio_framework_async::CancelToken, job: u64, start: (String, Vec<u8>)) -> Self {
        Self {
            runtime,
            instance,
            instance_gate,
            pool,
            cancel,
            cancel_scheduled: Arc::new(AtomicBool::new(false)),
            cancel_admitted: Arc::new(AtomicBool::new(false)),
            job,
            start: Some(start),
            pending: None,
            publication: None,
            closing: false,
            start_submitted: false,
            started: false,
            cleanup_required: false,
            terminal_delivered: false,
        }
    }
}
//#endregion 🔀️PostTurnRelay
