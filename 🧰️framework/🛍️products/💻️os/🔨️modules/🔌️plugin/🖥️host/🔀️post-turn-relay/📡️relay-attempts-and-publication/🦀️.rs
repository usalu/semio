//! 📡️ Relay attempts and publication: the relay attempt, the request run, the failure recovery and the relay publication with its drop and close receive. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
#[derive(Clone)]
pub(super) struct GuestRelayAttempt {
    pub(super) runtime: Arc<GuestRuntimes>,
    pub(super) instance: Arc<Mutex<GuestInstanceSlot>>,
    pub(super) instance_gate: Arc<semio_framework_async::Semaphore>,
    pub(super) pool: WorkerPool,
    pub(super) cancel_scheduled: Arc<AtomicBool>,
    pub(super) cancel_admitted: Arc<AtomicBool>,
    pub(super) job: u64,
    pub(super) sender: GuestRelayCompletionSender,
}

pub(super) async fn run_guest_relay_request(attempt: GuestRelayAttempt, cancel: semio_framework_async::CancelToken, request: GuestRelayRequest) {
    let GuestRelayAttempt { runtime, instance, instance_gate, pool: _pool, cancel_scheduled: _cancel_scheduled, cancel_admitted, job, sender } = attempt;
    let permit = if matches!(&request, GuestRelayRequest::Cancel) {
        instance_gate.acquire_owned().await
    } else {
        match semio_framework_async::select2(instance_gate.acquire_owned(), cancel.cancelled()).await {
            semio_framework_async::Either::Left(permit) => permit,
            semio_framework_async::Either::Right(()) => {
                sender.send(GuestRelayCompletion::Cancelled);
                return;
            }
        }
    };
    let unwind_detail = match &request {
        GuestRelayRequest::Cancel => b"plugin instance quarantined after cancel-job panic".to_vec(),
        GuestRelayRequest::Start { .. } | GuestRelayRequest::Step => b"plugin instance cleanup pending after guest relay panic".to_vec(),
    };
    let mut guest = match if matches!(&request, GuestRelayRequest::Cancel) { GuestInstanceLease::acquire_cleanup(Arc::clone(&instance)) } else { GuestInstanceLease::acquire(Arc::clone(&instance), unwind_detail) } {
        Ok(guest) => guest,
        Err(error) => {
            sender.send(GuestRelayCompletion::Rejected(GuestRelayOwnedBytes::new(error)));
            return;
        }
    };
    let mut cleanup_pending = false;
    let completion = match request {
        GuestRelayRequest::Start { kind, input } => match semio_framework_async::select2(runtime.start_job(guest.guest(), job, &kind, input), cancel.cancelled()).await {
            semio_framework_async::Either::Left(Ok(())) => {
                guest.available();
                GuestRelayCompletion::Started(Ok(()))
            }
            semio_framework_async::Either::Left(Err(error)) => {
                guest.cleanup_pending(guest_relay_cleanup_pending_detail("start-job", &error));
                cleanup_pending = true;
                GuestRelayCompletion::Started(Err(error))
            }
            semio_framework_async::Either::Right(()) => {
                let result = cancel_guest_job_once(&runtime, guest.guest(), job, &cancel_admitted).await;
                foreground_cancel_completion(result, &mut guest)
            }
        },
        GuestRelayRequest::Step => match semio_framework_async::select2(runtime.step_job(guest.guest(), job, RELAY_JOB_BUDGET), cancel.cancelled()).await {
            semio_framework_async::Either::Left(Ok(step)) => {
                guest.available();
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::from_guest(step)))
            }
            semio_framework_async::Either::Left(Err(error)) => {
                guest.cleanup_pending(guest_relay_cleanup_pending_detail("step-job", &error));
                cleanup_pending = true;
                GuestRelayCompletion::Stepped(Err(error))
            }
            semio_framework_async::Either::Right(()) => {
                let result = cancel_guest_job_once(&runtime, guest.guest(), job, &cancel_admitted).await;
                foreground_cancel_completion(result, &mut guest)
            }
        },
        GuestRelayRequest::Cancel => {
            let result = cancel_guest_job_once(&runtime, guest.guest(), job, &cancel_admitted).await;
            foreground_cancel_completion(result, &mut guest)
        }
    };
    drop(guest);
    drop(permit);
    #[cfg(any(test, feature = "testkit"))]
    wait_for_scripted_guest_relay_release(&runtime, &completion).await;
    let _ = cleanup_pending;
    sender.send(completion);
}

pub(super) fn recover_guest_relay_failure(attempt: GuestRelayAttempt, request: GuestRelayRequestKind, failure: GuestRelayPoolFailure) {
    let GuestRelayAttempt { runtime, instance, instance_gate, pool, cancel_scheduled, cancel_admitted, job, sender } = attempt;
    if let GuestRelayPoolFailure::Admission(kind) = failure {
        let detail = match kind {
            semio_framework_async::WorkerSubmitErrorKind::Shutdown => b"plugin instance quarantined after relay scheduler shutdown".to_vec(),
            semio_framework_async::WorkerSubmitErrorKind::Poisoned => b"plugin instance quarantined after relay scheduler poison".to_vec(),
            semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated => b"plugin instance quarantined after non-retryable relay admission failure".to_vec(),
        };
        quarantine_guest_instance(&instance, detail.clone());
        sender.send(GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail)));
        return;
    }
    #[cfg(any(test, feature = "testkit"))]
    if let GuestRuntimes::Mock(mock) = runtime.as_ref() {
        let barrier = match request {
            GuestRelayRequestKind::Start => mock.take_relay_start_failure_release(),
            GuestRelayRequestKind::Step => mock.take_relay_step_failure_release(),
            GuestRelayRequestKind::Cancel => None,
        };
        if let Some(barrier) = barrier {
            let panic_instance = Arc::clone(&instance);
            GuestRelayPoolFuture::spawn_recoverable(
                pool.clone(),
                Lane::UserVisible,
                async move {
                    barrier.wait().await;
                    recover_guest_relay_failure(GuestRelayAttempt { runtime, instance, instance_gate, pool, cancel_scheduled, cancel_admitted, job, sender }, request, GuestRelayPoolFailure::FuturePanicked);
                },
                move |_| quarantine_guest_instance(&panic_instance, b"plugin instance quarantined after relay panic recovery panic".to_vec()),
            );
            return;
        }
    }
    let detail = if matches!(request, GuestRelayRequestKind::Cancel) || cancel_admitted.load(std::sync::atomic::Ordering::Acquire) {
        let detail = b"plugin instance quarantined after cancel-job panic";
        quarantine_guest_instance(&instance, detail.to_vec());
        detail.to_vec()
    } else {
        let _ = (runtime, instance_gate, pool, job, cancel_scheduled, cancel_admitted);
        mark_guest_cleanup_pending(&instance, b"plugin instance cleanup pending after guest relay panic".to_vec());
        b"plugin guest relay panicked after acquiring its instance".to_vec()
    };
    sender.send(if matches!(request, GuestRelayRequestKind::Cancel) { GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail)) } else { GuestRelayCompletion::Fault(GuestRelayOwnedBytes::new(detail)) });
}

#[derive(Clone, Copy)]
pub(super) enum GuestRelayPublicationKind {
    Preview,
    Commit,
    Fault,
}

pub(super) struct GuestRelayPublication {
    pub(super) kind: GuestRelayPublicationKind,
    pub(super) source: std::mem::ManuallyDrop<Vec<u8>>,
    pub(super) cursor: usize,
    pub(super) copied: bool,
    pub(super) retired: bool,
    pub(super) oversized: bool,
    pub(super) delivered: bool,
    pub(super) writer: Option<semio_framework_job_payload::RetainedPayloadBuilder>,
}

impl GuestRelayPublication {
    pub(crate) fn new(kind: GuestRelayPublicationKind, source: Vec<u8>) -> Self {
        let oversized = source.len() > semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES;
        let kind = if oversized { GuestRelayPublicationKind::Fault } else { kind };
        let stream = match kind {
            GuestRelayPublicationKind::Preview => semio_framework_job_payload::JobPayloadStream::Preview,
            GuestRelayPublicationKind::Commit => semio_framework_job_payload::JobPayloadStream::CommitOutput,
            GuestRelayPublicationKind::Fault => semio_framework_job_payload::JobPayloadStream::Fault,
        };
        Self { kind, source:std::mem::ManuallyDrop::new(source), cursor: 0, copied: false, retired: false, oversized, delivered: false, writer: Some(semio_framework_job_payload::RetainedPayloadBuilder::new(stream)) }
    }

    /// 🏁️ A commit or fault ends the relay; only a preview is retired inside the running job.
    pub(super) fn is_terminal(&self) -> bool {
        !matches!(self.kind, GuestRelayPublicationKind::Preview)
    }

    pub(crate) fn invalid(detail: &'static str) -> semio_framework_value::ValueError {
        semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, detail)
    }

    /// ✍️ Pays one frontier of the publication (bind, append, retire the source, seal) and lends the sealed pages once.
    pub(crate) fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if self.delivered {
            return Ok(None);
        }
        if !self.writer.as_ref().ok_or_else(|| Self::invalid("guest relay publication lost its payload builder"))?.is_initialized() {
            self.writer.as_mut().expect("guest relay publication owns builder").advance_initialization(context)?;
            return Ok(None);
        }
        if !self.copied {
            let bytes = if self.oversized { &b"plugin-guest-relay-output-limit"[..] } else { self.source.as_slice() };
            let writer = self.writer.as_mut().expect("guest relay publication owns builder");
            if writer.append_original(context, bytes, &mut self.cursor)? {
                self.copied = true;
            }
            context.consume_fuel(1);
            return Ok(None);
        }
        if !self.retired {
            if self.source.is_empty() && self.source.capacity() == 0 {
                let grant = context.retained_grant();
                let bytes = size_of::<bool>();
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth < 1 {
                    return Ok(None);
                }
                self.retired = true;
                context.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                context.consume_fuel(1);
                return Ok(None);
            }
            let step = self.close_original_source(context.retained_grant()).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,context))?;
            context.consume_retained(step.progress())?;
            context.consume_fuel(1);
            return Ok(None);
        }
        let writer = self.writer.as_mut().expect("guest relay publication owns builder");
        if writer.published().is_none() {
            writer.seal(context)?;
            return Ok(None);
        }
        let payload = writer.published().ok_or_else(|| Self::invalid("guest relay publication lost its sealed payload"))?;
        let outcome = match self.kind {
            GuestRelayPublicationKind::Preview => semio_framework_job::JobOutcomeBorrow::admit_preview(context, payload)?,
            GuestRelayPublicationKind::Commit => semio_framework_job::JobOutcomeBorrow::admit_complete(context, None, Some(payload))?,
            GuestRelayPublicationKind::Fault => semio_framework_job::JobOutcomeBorrow::admit_fault(context, payload)?,
        };
        self.delivered |= outcome.is_some();
        Ok(outcome)
    }

    /// 🤝️ Resolves a descriptor against the same sealed pages this publication lent.
    pub(crate) fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        let payload = self.writer.as_ref().and_then(semio_framework_job_payload::RetainedPayloadBuilder::published).filter(|_| self.delivered).ok_or_else(|| Self::invalid("guest relay descriptor requires its delivered payload"))?;
        match self.kind {
            GuestRelayPublicationKind::Preview => descriptor.preview(payload),
            GuestRelayPublicationKind::Commit => descriptor.complete(None, Some(payload)),
            GuestRelayPublicationKind::Fault => descriptor.fault(payload),
        }
    }

    /// ♻️ Retires a delivered non-terminal publication's pages inside the next running step; true once nothing is left.
    pub(crate) fn retire(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if let Some(writer) = self.writer.as_mut() {
            if !writer.terminal_is_empty() {
                let step = writer.close_step_granted(context.retained_grant()).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,context))?;
                context.consume_retained(step.progress())?;
                return Ok(false);
            }
        }
        self.writer = None;
        Ok(self.terminal_is_empty())
    }

    pub(crate) fn begin_close(&mut self) {}

    pub(super) fn original_source_demand(&self)->semio_framework_value::RetirementDemand{
        if !self.source.is_empty(){semio_framework_value::RetirementDemand{copy_bytes:1+size_of::<usize>(),depth:1,..Default::default()}}
        else if self.source.capacity()!=0{semio_framework_value::RetirementDemand{copy_bytes:size_of::<Vec<u8>>(),release_bytes:self.source.capacity(),depth:1,..Default::default()}}
        else{Default::default()}
    }

    pub(super) fn close_original_source(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetirementTurnError};
        if self.source.is_empty()&&self.source.capacity()==0{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.original_source_demand();
        semio_framework_value::advance_retirement_turn(demand,grant,|grant|{
            if !self.source.is_empty(){let count=self.source.len().min(grant.maximum_copy_bytes-size_of::<usize>());let keep=self.source.len()-count;self.source.truncate(keep);return Ok((RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:count+size_of::<usize>(),..Default::default()}),false));}
            drop(std::mem::take(&mut*self.source));
            Ok((RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}),self.source.capacity()==0))
        }).map_err(|error|match error{RetirementTurnError::Owner(error)|RetirementTurnError::Receipt(error)=>error})
    }

    pub(crate) fn retirement_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(writer)=self.writer.as_ref(){return if writer.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_job_payload::RetainedPayloadBuilder>>(),depth:1,..Default::default()})}else{let mut demand=writer.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"Host publication writer depth overflow"))?;Ok(demand)};}
        Ok(self.original_source_demand())
    }

    pub(crate) fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::InteractiveJobCloseStep as S;
        let demand=self.retirement_demands()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(S::Pending{progress:Default::default()});}
        if let Some(writer)=self.writer.as_mut(){
            if writer.terminal_is_empty(){drop(self.writer.take());return Ok(S::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}});}
            return writer.close_step_granted(semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|S::Pending{progress:step.progress()});
        }
        self.close_original_source(grant).map(|step|match step{semio_framework_value::RetainedCloneStep::Progress(progress)=>S::Pending{progress},semio_framework_value::RetainedCloneStep::Complete(progress)=>S::Complete{progress}})
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.source.is_empty() && self.source.capacity() == 0
    }
}

impl Drop for GuestRelayPublication{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Host publication abandoned original writer or source backing");if self.source.is_empty()&&self.source.capacity()==0{unsafe{std::mem::ManuallyDrop::drop(&mut self.source);}}}}

pub(super) fn receive_relay_close(result:Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
    let step=result.map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
    cx.consume_retained(step.progress())?;
    if step.progress().copied_items!=0{cx.consume_fuel(1)}
    Ok(step)
}
//#endregion 🔀️PostTurnRelay
