//! 🧰️ Mounted and batch sessions: the mounted worker job session and the batch job session operations over the interactive job. See [`super`].

use super::*;

//#region 🏭️RetainedSessions
impl<J: InteractiveJob + 'static> MountedWorkerJobSession<J> {
    pub fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        WorkerJobSession::try_admit_owned(job,params,control).map(|admitted|admitted.map(|(session,progress)|(Self{session,ticket:None,checked_out:None},progress)))
    }

    pub fn generation(&self) -> Generation {
        self.session.generation()
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.checked_out.is_some() { WorkerJobPoll::CheckedOut } else { self.session.poll() }
    }

    /// ⛽️ `J: Send` for the same reason as [`WorkerJobSession::try_submit_step`] — this is the
    /// pool-submitting half of the mounted session, not the single-threaded drive path.
    pub fn pump_one(&mut self, pool: &WorkerPool, lane: Lane,retained:RetainedCloneGrant) -> Result<WorkerJobPoll, MountedWorkerJobPumpFault>
    where
        J: Send,
    {
        if self.checked_out.is_some() {
            return Err(MountedWorkerJobPumpFault::CheckedOut);
        }
        match self.session.poll() {
            WorkerJobPoll::Idle => {
                #[cfg(not(target_arch = "wasm32"))]
                if lane == Lane::Interactive {
                    let (ticket, poll) = self
                        .session
                        .try_step_on_caller(retained)
                        .map_err(|contention| MountedWorkerJobPumpFault::Submit(WorkerJobSubmitFault::Contention(contention)))?;
                    self.ticket = Some(ticket);
                    return match poll {
                        WorkerJobPoll::Outcome | WorkerJobPoll::Terminal => self.pump_one(pool, lane,retained),
                        other => Ok(other),
                    };
                }
                let ticket = self.session.try_submit_step(pool, lane,retained).map_err(MountedWorkerJobPumpFault::Submit)?;
                self.ticket = Some(ticket);
                Ok(WorkerJobPoll::Submitted)
            }
            WorkerJobPoll::Submitted => Ok(WorkerJobPoll::Submitted),
            WorkerJobPoll::Outcome => {
                let ticket = self.ticket.take().ok_or(MountedWorkerJobPumpFault::MissingTicket)?;
                let owner = self.session.take_outcome(ticket).map_err(MountedWorkerJobPumpFault::Take)?;
                self.checked_out = Some(owner);
                Ok(WorkerJobPoll::Outcome)
            }
            WorkerJobPoll::Terminal => {
                let owner = self.session.take_terminal().map_err(MountedWorkerJobPumpFault::Take)?;
                self.checked_out = Some(owner);
                Ok(WorkerJobPoll::Terminal)
            }
            WorkerJobPoll::Rejected => {
                let owner = self.session.take_rejected().map_err(MountedWorkerJobPumpFault::Take)?;
                owner.resume();
                Ok(WorkerJobPoll::Rejected)
            }
            poll => Ok(poll),
        }
    }

    /// 🧵️ Runs the session's next step on the calling thread and checks its outcome out. For a caller
    /// that already owns a bounded slice of its own — a guest `semio.infer` `step-job` crossing —
    /// where handing the step to a pool and waiting for it only adds a round trip per step.
    pub fn step_on_caller(&mut self,retained:RetainedCloneGrant) -> Result<WorkerJobPoll, MountedWorkerJobPumpFault> {
        if self.checked_out.is_some() {
            return Err(MountedWorkerJobPumpFault::CheckedOut);
        }
        let (ticket, poll) = self.session.try_step_on_caller(retained).map_err(|contention| MountedWorkerJobPumpFault::Submit(WorkerJobSubmitFault::Contention(contention)))?;
        if poll==WorkerJobPoll::Idle{return Ok(poll)}
        let owner = if poll == WorkerJobPoll::Terminal { self.session.take_terminal() } else { self.session.take_outcome(ticket) };
        self.checked_out = Some(owner.map_err(MountedWorkerJobPumpFault::Take)?);
        Ok(poll)
    }

    /// 🕰️ See [`WorkerJobSession::last_step_end_us`].
    pub fn last_step_end_us(&self) -> Option<u64> {
        self.session.last_step_end_us()
    }

    /// 🏃️ Executes the admitted business producer on the same original caller without renewing any budget.
    pub fn pump_one_for_interactive_turn(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobPoll,OriginalTurnFailure>{
        if self.checked_out.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker outcome must be returned before another callback").into())}
        let before=cx.retained_progress();
        let granted=cx.retained_grant();
        let Some((ticket,poll))=self.session.try_step_on_original_caller(cx)?else{return Ok(WorkerJobPoll::Idle)};
        if poll==WorkerJobPoll::Idle{return Ok(poll)}
        let owner=if poll==WorkerJobPoll::Terminal{self.session.take_terminal()}else{self.session.take_outcome(ticket)};
        self.checked_out=Some(owner.map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback lost its exact checked-out producer"))?);
        let (issued,progress)=self.checked_out_retained_step_receipt().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback omitted its physical receipt"))?;
        if *issued!=granted||!progress.fits(granted)||before.checked_add(*progress)?!=cx.retained_progress(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback receipt differs from its issued authority or posted effects").into())}
        Ok(poll)
    }

    /// 📥️ Returns the original effective grant and actual physical receipt once before semantic custody advances.
    /// 🔎️ Borrows the original issued authority and pending physical receipt without handing it back.
    pub fn checked_out_retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.checked_out.as_ref().and_then(WorkerJobOutcome::retained_step_receipt)}

    pub fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.checked_out.as_mut().and_then(WorkerJobOutcome::take_retained_step_receipt)}

    /// 🧾️ Borrows the actual same-owner worker receipt before transferring its outcome.
    pub fn checked_out_retained_step_progress(&self)->Option<RetainedCloneProgress>{self.checked_out.as_ref().map(WorkerJobOutcome::retained_step_progress)}

    pub fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{match self.checked_out.as_ref(){Some(original)=>original.outcome(),None=>Ok(None)}}

    /// 🎫️ Observes the same paid descriptor through its separate acknowledgement and removal phases.
    pub fn checked_out_outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{self.checked_out.as_ref().and_then(WorkerJobOutcome::outcome_descriptor)}

    /// 🛑️ Keeps the worker's original refusal inside its checked-out authority.
    pub fn checked_out_failure(&self)->Option<&OriginalTurnFailure>{self.checked_out.as_ref().and_then(WorkerJobOutcome::retained_failure)}

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::callback_verdict)
    }

    pub fn overrun_ledger(&self) -> Option<&semio_framework_trace::StepOverrunLedger> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::overrun_ledger)
    }

    pub fn acknowledge_checked_out_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.checked_out.as_mut().map_or(RetainedCloneStep::Complete(Default::default()),|original|original.acknowledge_outcome(grant))}

    pub fn checked_out_job_mut(&mut self) -> Option<&mut J> {
        self.checked_out.as_mut().and_then(|original|original.job_mut().ok())
    }
    pub fn checked_out_job(&self)->Option<&J>{self.checked_out.as_ref().map(WorkerJobOutcome::job)}

    /// 🧮️ Inspect the exact retained owner without advancing or enlarging its close grant.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError> {
        match self.checked_out.as_ref().and_then(|owner| owner.authority.as_ref()) {
            Some(authority)=>authority.retirement_demands(maximum_copy_bytes).map_err(WorkerJobDemandError::Refused),
            None=>self.session.retirement_demands(maximum_copy_bytes),
        }
    }

    pub fn resume(&mut self) -> Result<(), WorkerJobContention> {
        let owner = self.checked_out.take().ok_or_else(|| self.session.contention())?;
        owner.resume().map_err(|owner| {
            self.checked_out = Some(owner);
            self.session.contention()
        })
    }

    pub fn begin_close(&mut self) {
        if let Some(owner) = self.checked_out.take() {
            owner.begin_close();
        } else {
            let _ = self.session.begin_close();
        }
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
        if let Some(mut owner)=self.checked_out.take(){
            if let Some(demand)=owner.authority.as_ref().and_then(OriginalWorkerJobAuthority::outcome_demands){
                let result=close_original_retained_step(cx,demand,|grant|Ok(owner.acknowledge_outcome(grant)));self.checked_out=Some(owner);return result.map(|step|match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>WorkerJobCloseStep::Blocked})
            }
            let progress=RetainedCloneProgress{copied_items:1,..Default::default()};if let Err(error)=cx.consume_retained(progress){self.checked_out=Some(owner);return Err(error.into())}cx.consume_fuel(1);owner.begin_close();return Ok(WorkerJobCloseStep::Pending{progress})
        }
        self.session.close_step(cx)
    }

    /// 🪢️ Preserves checkout ownership while forwarding the caller's borrowed original cancellation root.
    pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{
        if self.checked_out.is_some(){return Ok(false);}
        self.session.has_original_cancel_alias_witness(witness)
    }

    /// ✖️ Preserves checkout ownership while forwarding the caller's borrowed original cancellation root.
    pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{
        if self.checked_out.is_some(){return Ok(None);}
        self.session.return_original_cancel_alias_step(witness,grant)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.checked_out.is_none() && self.session.terminal_is_empty()
    }

    /// 🔭️ The named phase this mounted session's close cursor is parked in — see
    /// [`WorkerJobClosePhase`]. A retained checked-out outcome is the mounted half's own phase and
    /// outranks whatever the inner session reports.
    pub fn close_phase(&self) -> WorkerJobClosePhase {
        if self.checked_out.is_some() { WorkerJobClosePhase::CheckedOut } else { self.session.close_phase() }
    }
}

impl<J: InteractiveJob + 'static> BatchJobSession<J> {
    /// 🪪️ Borrows the original parameters under the same native custody gate used by close quotes.
    pub fn original_identity(&self)->Result<Option<(OperationId,Generation)>,WorkerJobContention>{
        if let Some(owner)=self.checked_out.as_ref(){return Ok(owner.authority.as_ref().and_then(OriginalWorkerJobAuthority::source_identity))}
        if self.session.inner.0.is_none(){return Ok(None)}
        let phase=self.session.inner.phase();
        if matches!(phase,SESSION_SUBMITTED|SESSION_TRANSITION|SESSION_CHECKED_OUT|SESSION_EMPTY)||self.session.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self.session.contention())}
        let authority=unsafe{self.session.inner.take_authority()};
        let identity=authority.source_identity();
        unsafe{self.session.inner.put_authority_quiet(authority,phase)};Ok(identity)
    }

    pub fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        WorkerJobSession::try_admit_owned(job,params,control).map(|admitted|admitted.map(|(session,progress)|(Self{session,ticket:None,checked_out:None},progress)))
    }

    /// 🤝️ Hands the same prepared native authority to Batch on a separate paid metadata turn.
    pub fn admit_prepared(original:&mut Option<WorkerJobSession<J>>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"prepared Batch admission requires the same idle original authority");
        let session=original.as_ref().ok_or_else(invalid)?;
        if session.inner.0.is_none(){return Err(invalid())}
        let grant=control.admission_grant()?;
        if !control.admission_is_open()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        if session.inner.phase.compare_exchange(SESSION_IDLE,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(invalid())}
        let authority=unsafe{session.inner.take_authority()};
        let admitted=authority.can_admit_prepared(control,session.generation)&&!session.inner.close_requested.load(Ordering::Acquire);
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        let received=if admitted{control.receive_admission(progress)}else{Err(invalid())};
        unsafe{session.inner.put_authority_quiet(authority,SESSION_IDLE)};
        received?;
        let session=original.take().ok_or_else(invalid)?;
        Ok(Some((Self{session,ticket:None,checked_out:None},progress)))
    }

    /// 🎟️ Keeps the exact original callback ticket while the caller separately receives semantic custody.
    pub fn step_on_original_caller(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobPoll,OriginalTurnFailure>{
        if self.checked_out.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original batch outcome remains checked out").into())}
        let Some((ticket,poll))=self.session.try_step_on_original_caller(cx)?else{return Ok(WorkerJobPoll::Idle)};
        if poll!=WorkerJobPoll::Idle{self.ticket=Some(ticket)}
        Ok(poll)
    }

    pub fn step(&mut self,retained:RetainedCloneGrant) -> Result<WorkerJobPoll, WorkerJobContention> {
        if self.checked_out.is_some() {
            return Err(WorkerJobContention::CheckedOut(self.session.generation()));
        }
        let (ticket, poll) = self.session.try_step_on_caller(retained)?;
        if poll!=WorkerJobPoll::Idle{self.ticket = Some(ticket);}
        Ok(poll)
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.checked_out.is_some() { WorkerJobPoll::CheckedOut } else { self.session.poll() }
    }

    /// 🔎️ Borrows the original issued authority and pending physical receipt without handing it back.
    pub fn checked_out_retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.checked_out.as_ref().and_then(WorkerJobOutcome::retained_step_receipt)}

    pub fn take_checked_out_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.checked_out.as_mut().and_then(WorkerJobOutcome::take_retained_step_receipt)}

    pub fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.checked_out.as_mut().map_or(RetainedCloneStep::Progress(Default::default()),|original|original.acknowledge_outcome(grant))}

    pub fn checkout_outcome(&mut self) -> bool {
        if self.checked_out.is_some() {
            return true;
        }
        let owner = match self.session.poll() {
            WorkerJobPoll::Outcome => match self.ticket.take().and_then(|ticket| self.session.take_outcome(ticket).ok()) {
                Some(owner) => owner,
                None => return false,
            },
            WorkerJobPoll::Terminal => match self.session.take_terminal().ok() {
                Some(owner) => owner,
                None => return false,
            },
            _ => return false,
        };
        self.checked_out = Some(owner);
        true
    }

    pub fn checked_out_outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{match self.checked_out.as_ref(){Some(original)=>original.outcome(),None=>Ok(None)}}

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::callback_verdict)
    }

    pub fn overrun_ledger(&self) -> Option<&semio_framework_trace::StepOverrunLedger> {
        self.checked_out.as_ref().and_then(WorkerJobOutcome::overrun_ledger)
    }

    pub fn checked_out_job_mut(&mut self) -> Option<&mut J> {
        WorkerJobOutcome::job_mut(self.checked_out.as_mut()?).ok()
    }

    /// 🧭️ Forward the exact owner demand while preserving checkout and worker ownership.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError> {
        match self.checked_out.as_ref().and_then(|owner| owner.authority.as_ref()) {
            Some(authority)=>authority.retirement_demands(maximum_copy_bytes).map_err(WorkerJobDemandError::Refused),
            None=>self.session.retirement_demands(maximum_copy_bytes),
        }
    }

    pub fn resume(&mut self) -> Result<(), WorkerJobContention> {
        let owner = self.checked_out.take().ok_or_else(|| self.session.contention())?;
        owner.resume().map_err(|owner| {
            self.checked_out = Some(owner);
            self.session.contention()
        })
    }

    pub fn begin_close(&mut self) {
        if let Some(owner) = self.checked_out.take() {
            owner.begin_close();
        } else {
            let _ = self.session.begin_close();
        }
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
        if let Some(owner)=self.checked_out.take(){let progress=RetainedCloneProgress{copied_items:1,..Default::default()};if let Err(error)=cx.consume_retained(progress){self.checked_out=Some(owner);return Err(error.into())}cx.consume_fuel(1);owner.begin_close();return Ok(WorkerJobCloseStep::Pending{progress})}
        self.session.close_step(cx)
    }

    /// ❌️ Preserves checkout custody before inspecting the original borrowed cancellation witness.
    pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{if self.checked_out.is_some(){return Ok(false)}self.session.has_original_cancel_alias_witness(witness)}
    /// 🪜️ Returns the original alias through its sole session after checkout custody has returned.
    pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{if self.checked_out.is_some(){return Ok(None)}self.session.return_original_cancel_alias_step(witness,grant)}

    pub fn terminal_is_empty(&self) -> bool {
        self.checked_out.is_none() && self.session.terminal_is_empty()
    }

    /// 📪️ Observe the actual batch checkout or original worker retirement phase without effects.
    pub fn close_phase(&self)->WorkerJobClosePhase{if self.checked_out.is_some(){WorkerJobClosePhase::CheckedOut}else{self.session.close_phase()}}
}
//#endregion 🏭️RetainedSessions
