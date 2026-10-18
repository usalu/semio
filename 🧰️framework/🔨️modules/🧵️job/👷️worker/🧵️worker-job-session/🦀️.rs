//! 🧵️ Worker job session: the begin close and close step helpers and the worker job session operations with their drop. See [`super`].

use super::*;

//#region 🏭️RetainedSessions
pub(super) fn worker_job_begin_close<J>(inner: &WorkerJobSessionInner<J>) -> WorkerJobCloseStep {
    inner.close_requested.store(true, Ordering::Release);
    loop {
        let phase = inner.phase();
        if phase == SESSION_SUBMITTED || phase == SESSION_TRANSITION || phase == SESSION_CHECKED_OUT {
            inner.terminal_intent.store(1, Ordering::Release);
            inner.raise_wake();
            return WorkerJobCloseStep::Blocked;
        }
        if phase == SESSION_CLOSE {
            return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
        if phase == SESSION_EMPTY {
            return WorkerJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if inner.phase.compare_exchange(phase, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            let authority = unsafe { inner.take_authority() };
            unsafe { inner.put_authority(authority, SESSION_CLOSE) };
            return WorkerJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 0, released_bytes: 0, ..RetainedCloneProgress::default() } };
        }
    }
}

pub(super) fn worker_job_close_step<J:InteractiveJob>(inner:&WorkerJobSessionInner<J>,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{if cx.should_yield(){return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}if inner.phase.compare_exchange(SESSION_CLOSE,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Ok(if inner.phase()==SESSION_EMPTY{WorkerJobCloseStep::Complete{progress:Default::default()}}else{WorkerJobCloseStep::Blocked})}let mut authority=unsafe{inner.take_authority()};let result=authority.close_step(cx);if matches!(result,Ok(WorkerJobCloseStep::Complete{..})){assert!(authority.terminal_is_empty());drop(authority);inner.phase.store(SESSION_EMPTY,Ordering::Release);inner.raise_wake()}else{unsafe{inner.put_authority(authority,SESSION_CLOSE)}}result}

impl<J: InteractiveJob + 'static> WorkerJobSession<J> {
    /// 📏️ Quotes the actual fixed storage births and initialized source/control writes before custody changes.
    pub fn admission_demand()->RetirementDemand{
        let mut demand=OriginalWorkerJobAuthority::<J>::admission_demand();
        demand.capacity_bytes+=semio_framework_value::shared_retirement_allocation_bytes::<WorkerJobSessionInner<J>>()+size_of::<WorkerJobRetirementNode<J>>();
        demand
    }
    /// 🚪️ Retains both original sources until their existing normal control admits every physical birth.
    /// 🎟️ Quotes complete atomic initialization and the original job/parameter moves before storage birth.
    pub fn owned_admission_demand()->Result<RetirementDemand,ValueError>{
        let mut demand=Self::admission_demand();let initialized=OriginalWorkerPayload::initialization_copy_bytes()+OriginalWorkerJobAuthority::<J>::admission_page_publication_copy_bytes()+size_of::<J>()+2*size_of::<Option<J>>()+2*size_of::<Option<BatchJobParams>>()+2*size_of::<bool>()+size_of::<u8>();demand.copy_bytes=demand.copy_bytes.checked_add(initialized).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original atomic session initialization extent overflow"))?;Ok(demand)
    }

    pub fn try_admit_owned(job:&mut Option<J>,params:&mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let original=params.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"session admission requires original parameters"))?;
        if job.is_none()||control.admission_identity()!=(original.operation,original.generation)||!control.admission_cancel().is_none_or(|token|token.is_same_node(&original.cancel)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session admission requires the same original job and identity"))}
        let demand=Self::owned_admission_demand()?;let grant=control.admission_grant()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth||original.cancel.is_cancelled_now()||!control.admission_is_open(){return Ok(None)}
        let Some(session)=Self::birth_storage(original.operation,original.generation,control)?else{return Ok(None)};
        let inner=session.inner.0.as_ref().expect("admitted original session Arc");let authority=unsafe{(&mut *inner.authority.get()).as_mut().expect("admitted original authority")};
        while !authority.is_initialized(){let progress=authority.advance_initialization(grant);assert!(progress.copied_items>0,"admitted original atomic initialization must advance");}
        unsafe{authority.write_owned_source_prepaid(job,params);}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:0};
        control.receive_admission(progress).expect("same exclusive caller admitted original session births");
        Ok(Some((session,progress)))
    }

    pub(super) fn birth_storage(operation:OperationId,generation:Generation,control:&mut impl WorkerJobAdmissionControl)->Result<Option<Self>,ValueError>{if !OriginalWorkerPageReservation::available(){return Ok(None)}let Some(slot)=reserve_worker_job_retirement_slot()else{return Ok(None)};let Some(reservation)=OriginalWorkerPageReservation::try_reserve()else{WORKER_JOB_RETIREMENT_SLOTS[slot].store(std::ptr::null_mut(),Ordering::Release);control.receive_admission(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(None)};let authority=match unsafe{OriginalWorkerJobAuthority::admit_pending_prepaid(operation,generation,reservation)}{Ok(authority)=>authority,Err(error)=>{WORKER_JOB_RETIREMENT_SLOTS[slot].store(std::ptr::null_mut(),Ordering::Release);control.receive_admission(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Err(error)}};let inner=session_return::SessionHandle::new(WorkerJobSessionInner{generation,phase:AtomicU8::new(SESSION_IDLE),authority:ManuallyDrop::new(std::cell::UnsafeCell::new(Some(authority))),rejection_kind:AtomicU8::new(u8::MAX),close_requested:AtomicBool::new(false),terminal_intent:AtomicU8::new(0),wake_pending:AtomicBool::new(false),wake_sequence:AtomicU64::new(0),wake_exhausted:AtomicBool::new(false),wake_guard:AtomicBool::new(false),waker:ManuallyDrop::new(std::cell::UnsafeCell::new(None)),retained_wake:ManuallyDrop::new(std::cell::UnsafeCell::new(None)),last_step_end_us:AtomicU64::new(u64::MAX)});let retirement=Box::new(WorkerJobRetirementNode{header:WorkerJobRetirementHeader{slot,phase:worker_job_retirement_node_phase::<J>,pump:pump_worker_job_retirement_node::<J>,destroy:destroy_worker_job_retirement_node::<J>},inner:None});Ok(Some(Self{inner:WorkerJobSessionArc(Some(inner)),generation,terminal_step_end_us:None,retirement:std::cell::UnsafeCell::new(Some(retirement)),retirement_state:AtomicU8::new(0)}))}

    /// 📥️ Returns an outstanding original physical receipt even when cancellation already requested close.
    pub fn take_retained_step_receipt(&self)->Result<Option<(RetainedCloneGrant,RetainedCloneProgress)>,WorkerJobContention>{if self.inner.0.is_none(){return Ok(None)}let phase=self.inner.phase();if matches!(phase,SESSION_SUBMITTED|SESSION_TRANSITION|SESSION_CHECKED_OUT)||self.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self.contention())}let mut authority=unsafe{self.inner.take_authority()};let original=authority.take_retained_step_receipt();unsafe{self.inner.put_authority_quiet(authority,phase)};Ok(original)}

    pub fn generation(&self) -> Generation {
        self.generation
    }

    pub fn poll(&self) -> WorkerJobPoll {
        if self.inner.0.is_none(){return if self.retirement_state.load(Ordering::Acquire)==3{WorkerJobPoll::TerminalEmpty}else{WorkerJobPoll::Closing}}
        match self.inner.phase() {
            SESSION_IDLE => WorkerJobPoll::Idle,
            SESSION_SUBMITTED => WorkerJobPoll::Submitted,
            SESSION_OUTCOME => WorkerJobPoll::Outcome,
            SESSION_TERMINAL => WorkerJobPoll::Terminal,
            SESSION_REJECTED => WorkerJobPoll::Rejected,
            SESSION_CHECKED_OUT | SESSION_TRANSITION => WorkerJobPoll::CheckedOut,
            SESSION_CLOSE => WorkerJobPoll::Closing,
            SESSION_EMPTY => WorkerJobPoll::TerminalEmpty,
            _ => WorkerJobPoll::Closing,
        }
    }

    fn try_step_inline(&self,retained:RetainedCloneGrant) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        if self.inner.0.is_none(){return Err(WorkerJobContention::TerminalEmpty)}
        if self.inner.phase.compare_exchange(SESSION_IDLE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(self.contention());
        }
        let mut authority = unsafe { self.inner.take_authority() };
        let issued=authority.issued_grant(retained);
        let ticket = WorkerJobTicket { generation: self.inner.generation, step_sequence: authority.sequence() };
        if issued.maximum_items==0||issued.maximum_depth==0{unsafe{self.inner.put_authority_quiet(authority,SESSION_IDLE)};return Ok((ticket,WorkerJobPoll::Idle))}
        authority.stage_callback(issued);
        let terminal = authority.run_staged_callback();
        self.inner.last_step_end_us.store(authority.last_step_end_us().unwrap_or(u64::MAX), Ordering::Release);
        if self.inner.close_requested.load(Ordering::Acquire) {
            self.inner.terminal_intent.store(1, Ordering::Release);
        }
        unsafe { self.inner.put_authority(authority, if terminal { SESSION_TERMINAL } else { SESSION_OUTCOME }) };
        Ok((ticket, if terminal { WorkerJobPoll::Terminal } else { WorkerJobPoll::Outcome }))
    }

    /// 🎟️ Borrows the initiating context for one exact original callback and retains job custody on every failure.
    pub fn try_step_on_original_caller(&self,cx:&mut StepContext<'_>)->Result<Option<(WorkerJobTicket,WorkerJobPoll)>,OriginalTurnFailure>{if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(None)}if self.inner.0.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback session is terminal empty").into())}if self.inner.phase.compare_exchange(SESSION_IDLE,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Ok(None)}let mut authority=unsafe{self.inner.take_authority()};if let Err(error)=authority.validate_original_caller(cx){unsafe{self.inner.put_authority_quiet(authority,SESSION_IDLE)};return Err(error.into())}let ticket=WorkerJobTicket{generation:self.inner.generation,step_sequence:authority.sequence()};let terminal=match authority.step_on_original_caller(cx){Ok(terminal)=>terminal,Err(failure)=>{unsafe{self.inner.put_authority(authority,SESSION_CLOSE)};return Err(failure)}};self.inner.last_step_end_us.store(authority.last_step_end_us().unwrap_or(u64::MAX),Ordering::Release);unsafe{self.inner.put_authority(authority,if terminal{SESSION_TERMINAL}else{SESSION_OUTCOME})};Ok(Some((ticket,if terminal{WorkerJobPoll::Terminal}else{WorkerJobPoll::Outcome})))}

    pub fn try_step_on_caller(&self,retained:RetainedCloneGrant) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        self.try_step_inline(retained)
    }

    /// 🕰️ The clock reading the last step of this session ended at — its driver's exit reading, so a
    /// caller deciding whether to run another step needs no read of its own.
    pub fn last_step_end_us(&self) -> Option<u64> {
        if self.inner.0.is_none(){return self.terminal_step_end_us}
        Some(self.inner.last_step_end_us.load(Ordering::Acquire)).filter(|end_us| *end_us != u64::MAX)
    }

    /// 🧵️ Executes one exact owner turn from a retained scheduler already running inside its worker.
    pub fn try_step_on_worker(&self,retained:RetainedCloneGrant) -> Result<(WorkerJobTicket, WorkerJobPoll), WorkerJobContention> {
        self.try_step_inline(retained)
    }

    /// 👁️ Borrows the original typed registration state under its existing wake gate.
    pub fn has_retained_wake(&self)->Result<bool,WorkerJobContention>{
        if self.inner.0.is_none(){return Ok(false)}
        if self.inner.wake_guard.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_err(){return Err(WorkerJobContention::CheckedOut(self.generation))}
        let retained=unsafe{(&*self.inner.retained_wake.get()).is_some()};self.inner.wake_guard.store(false,Ordering::Release);Ok(retained)
    }

    /// 🔔️ Keeps the original admitted typed wake frame until its own paid close turns finish.
    pub fn register_retained_wake(&self,original:&mut Option<Box<dyn RetainedWorkerWake>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,WorkerJobContention>{
        if original.is_none(){return Ok(None)}
        if self.inner.0.is_none()||self.inner.phase()==SESSION_EMPTY{return Err(WorkerJobContention::TerminalEmpty)}
        let copy=size_of::<Option<Box<dyn RetainedWorkerWake>>>();
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(None)}
        if self.inner.wake_guard.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_err(){return Err(WorkerJobContention::CheckedOut(self.generation))}
        let slot=unsafe{&mut *self.inner.retained_wake.get()};
        if slot.is_some()||unsafe{(&*self.inner.waker.get()).is_some()}{self.inner.wake_guard.store(false,Ordering::Release);return Err(WorkerJobContention::CheckedOut(self.generation))}
        *slot=original.take();
        if self.inner.wake_pending.load(Ordering::Acquire){slot.as_ref().unwrap().wake_by_ref();}
        self.inner.wake_guard.store(false,Ordering::Release);
        Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }

    pub fn register_wake(&self, waker: &Waker) -> Result<(), WorkerJobContention> {
        if self.inner.0.is_none(){return Err(WorkerJobContention::TerminalEmpty)}
        self.inner.register_waker(waker)
    }

    /// 🔔️ Replaces a stale session wake with the exact job-owned close wake when available.
    pub fn register_close_wake(&self, waker: &Waker) -> Result<bool, WorkerJobContention> {
        if self.inner.0.is_none(){return Ok(false)}
        loop {
            let phase = self.inner.phase();
            if phase != SESSION_CLOSE {
                self.take_wake();
                self.register_wake(waker)?;
                return Ok(true);
            }
            if self.inner.phase.compare_exchange(SESSION_CLOSE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
                continue;
            }
            self.inner.wake_pending.store(false, Ordering::Release);
            let authority = unsafe { self.inner.take_authority() };
            let registered = authority.register_close_wake(waker);
            unsafe { self.inner.put_authority_quiet(authority, SESSION_CLOSE) };
            return Ok(registered);
        }
    }

    pub fn take_wake(&self) -> bool {
        if self.inner.0.is_none(){return false}
        self.inner.wake_pending.swap(false, Ordering::AcqRel)
    }

    /// 🧵️ `J: Send` is stated here rather than on [`InteractiveJob`]: this is the one call that
    /// actually hands the job to another thread, so the requirement belongs to it. Targets with no
    /// second thread (the browser wasm build) never reach this and keep their `Rc`-owning jobs.
    pub fn try_submit_step(&self, pool: &WorkerPool, lane: Lane,retained:RetainedCloneGrant) -> Result<WorkerJobTicket, WorkerJobSubmitFault>
    where
        J: Send,
    {
        if self.inner.0.is_none(){return Err(WorkerJobSubmitFault::Contention(WorkerJobContention::TerminalEmpty))}
        if self.inner.phase.compare_exchange(SESSION_IDLE, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobSubmitFault::Contention(self.contention()));
        }
        let mut authority = unsafe { self.inner.take_authority() };
        authority.stage_callback(retained);
        if authority.sequence() == u64::MAX {
            unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
            return Err(WorkerJobSubmitFault::SequenceExhausted);
        }
        let ticket = WorkerJobTicket { generation: self.inner.generation, step_sequence: authority.sequence() };
        self.inner.rejection_kind.store(u8::MAX, Ordering::Release);
        self.inner.phase.store(SESSION_SUBMITTED, Ordering::Release);
        let submission = WorkerJobSubmission { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), ran: false };
        let closure: semio_framework_async::Job = Box::new(move || submission.run());
        match pool.try_submit(lane, closure) {
            Ok(()) => Ok(ticket),
            Err(error) => {
                let kind = error.kind();
                self.inner.rejection_kind.store(worker_rejection_code(kind), Ordering::Release);
                drop(error.into_job());
                Err(WorkerJobSubmitFault::Pool(kind))
            }
        }
    }

    pub fn take_outcome(&self, ticket: WorkerJobTicket) -> Result<WorkerJobOutcome<J>, WorkerJobTakeFault> {
        if self.inner.0.is_none(){return Err(if ticket.generation!=self.generation{WorkerJobTakeFault::Stale}else{WorkerJobTakeFault::WrongPhase})}
        if ticket.generation != self.inner.generation {
            return Err(WorkerJobTakeFault::Stale);
        }
        if self.inner.phase.compare_exchange(SESSION_OUTCOME, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(if self.inner.phase() == SESSION_SUBMITTED { WorkerJobTakeFault::Pending } else { WorkerJobTakeFault::WrongPhase });
        }
        let authority = unsafe { self.inner.take_authority() };
        if authority.sequence() != ticket.step_sequence.saturating_add(1) {
            unsafe { self.inner.put_authority(authority, SESSION_OUTCOME) };
            return Err(WorkerJobTakeFault::Stale);
        }
        Ok(WorkerJobOutcome { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), restore_phase: SESSION_OUTCOME })
    }

    pub fn take_terminal(&self) -> Result<WorkerJobOutcome<J>, WorkerJobTakeFault> {
        if self.inner.0.is_none(){return Err(WorkerJobTakeFault::WrongPhase)}
        if self.inner.phase.compare_exchange(SESSION_TERMINAL, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(if self.inner.phase() == SESSION_SUBMITTED { WorkerJobTakeFault::Pending } else { WorkerJobTakeFault::WrongPhase });
        }
        let authority = unsafe { self.inner.take_authority() };
        Ok(WorkerJobOutcome { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), restore_phase: SESSION_TERMINAL })
    }

    pub fn take_rejected(&self) -> Result<WorkerJobRejected<J>, WorkerJobTakeFault> {
        if self.inner.0.is_none(){return Err(WorkerJobTakeFault::WrongPhase)}
        if self.inner.phase.compare_exchange(SESSION_REJECTED, SESSION_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobTakeFault::WrongPhase);
        }
        let authority = unsafe { self.inner.take_authority() };
        let kind = worker_rejection_kind(self.inner.rejection_kind.load(Ordering::Acquire));
        Ok(WorkerJobRejected { inner: session_return::SessionHandle::clone(&self.inner), authority: Some(authority), kind })
    }

    pub fn begin_close(&self) -> WorkerJobCloseStep {
        if self.inner.0.is_none(){return if self.terminal_is_empty(){WorkerJobCloseStep::Complete{progress:Default::default()}}else{WorkerJobCloseStep::Pending{progress:Default::default()}}}
        worker_job_begin_close(&self.inner)
    }

    /// 🪢️ Returns only the original cancellation alias witnessed by the caller's still-live matching root.
    pub fn has_original_cancel_alias_witness(&self,witness:&CancelToken)->Result<bool,WorkerJobDemandError>{if self.inner.0.is_none(){return Ok(false)}let phase=self.inner.phase();if phase==SESSION_EMPTY{return Ok(false)}if phase!=SESSION_CLOSE||self.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(WorkerJobDemandError::Contention(self.contention()))}let authority=unsafe{self.inner.take_authority()};let original=authority.has_original_cancel_alias_witness(witness);unsafe{self.inner.put_authority_quiet(authority,phase)};Ok(original)}

    /// 🪢️ Returns only the original cancellation alias witnessed by the caller's still-live matching root.
    pub fn return_original_cancel_alias_step(&self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,WorkerJobDemandError>{if self.inner.0.is_none(){return Ok(None)}let phase=self.inner.phase();if phase==SESSION_EMPTY{return Ok(None)}if phase!=SESSION_CLOSE||self.inner.phase.compare_exchange(phase,SESSION_TRANSITION,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(WorkerJobDemandError::Contention(self.contention()))}let mut authority=unsafe{self.inner.take_authority()};let result=authority.return_original_cancel_alias_step(witness,grant).map_err(|error|match error{semio_framework_async::CancelTokenRetirementError::Refused(error)=>WorkerJobDemandError::Refused(error),semio_framework_async::CancelTokenRetirementError::Blocked(_)=>WorkerJobDemandError::Contention(WorkerJobContention::CheckedOut(self.inner.generation))});unsafe{self.inner.put_authority_quiet(authority,phase)};result}
    /// 🔐️ Inspect under the same exclusive phase admission used by the close cursor.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,WorkerJobDemandError> {
        if self.inner.0.is_none(){return Ok(if self.retirement_state.load(Ordering::Acquire)==3{Default::default()}else{RetirementDemand{copy_bytes:0,release_bytes:size_of::<WorkerJobRetirementNode<J>>(),depth:1,..Default::default()}})}
        let phase = self.inner.phase();
        if phase == SESSION_EMPTY { return self.original_session_arc_demands(maximum_copy_bytes).map_err(WorkerJobDemandError::Refused); }
        if matches!(phase, SESSION_SUBMITTED | SESSION_TRANSITION | SESSION_CHECKED_OUT) || self.inner.phase.compare_exchange(phase, SESSION_TRANSITION, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobDemandError::Contention(WorkerJobContention::CheckedOut(self.inner.generation)));
        }
        let authority = unsafe { self.inner.take_authority() };
        let demand = authority.retirement_demands(maximum_copy_bytes);
        unsafe { self.inner.put_authority_quiet(authority, phase) };
        demand.map_err(WorkerJobDemandError::Refused)
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
        if cx.should_yield(){return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}let before=cx.retained_progress();let grant=cx.retained_grant();if self.inner.0.is_none(){let step=self.release_retirement_slot(grant);return receive_original_worker_close_effects(cx,before,step)}
        let step=match worker_job_close_step(&self.inner,cx)?{WorkerJobCloseStep::Complete{progress}if progress!=Default::default()=>WorkerJobCloseStep::Pending{progress},WorkerJobCloseStep::Complete{..}=>{let grant=cx.retained_grant();let result=self.close_original_session_arc(grant).map(|step|match step{WorkerJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},step=>step});return receive_original_worker_close_result(cx,before,result)},step=>step};receive_original_worker_close_effects(cx,before,step)
    }

    pub(super) fn original_session_arc_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
        if self.inner.0.is_none(){return Ok(Default::default())}
        if let Some(demand)=worker_wake_retirement::session_retained_wake_demands(&self.inner,maximum_copy_bytes)?{return Ok(demand)}
        Ok(RetirementDemand{copy_bytes:0,release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<WorkerJobSessionInner<J>>(),depth:1,..Default::default()})
    }

    fn close_original_session_arc(&mut self,grant:RetainedCloneGrant)->Result<WorkerJobCloseStep,ValueError>{
        let demand=self.original_session_arc_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
        let original=self.inner.0.as_ref().expect("terminal session retains original Arc");
        if !original.is_unique()||unsafe{(&*original.waker.get()).is_some()}{return Ok(WorkerJobCloseStep::Blocked)}
        if let Some(step)=worker_wake_retirement::close_session_retained_wake(original,grant)?{return Ok(step)}
        self.terminal_step_end_us=self.last_step_end_us();Ok(match self.inner.0.take().unwrap().try_return(){Ok(original)=>{drop(original);WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}}},Err(original)=>{self.inner.0=Some(original);WorkerJobCloseStep::Blocked}})
    }

    pub fn terminal_is_empty(&self) -> bool {self.inner.0.is_none()&&self.retirement_state.load(Ordering::Acquire)==3}

    /// 🔭️ The named phase this session's close cursor is parked in — see [`WorkerJobClosePhase`].
    /// Reads the same fields the next `close_step` would walk, in the same order, without taking
    /// the authority or spending a turn.
    pub fn close_phase(&self) -> WorkerJobClosePhase {
        if self.inner.0.is_none(){return if self.retirement_state.load(Ordering::Acquire)==3{WorkerJobClosePhase::Empty}else{WorkerJobClosePhase::RetirementSlot}}
        match self.inner.phase() {
            SESSION_EMPTY => return WorkerJobClosePhase::SessionArc,
            SESSION_CHECKED_OUT | SESSION_TRANSITION => return WorkerJobClosePhase::CheckedOut,
            SESSION_CLOSE => {}
            _ => return WorkerJobClosePhase::Open,
        }
        let Some(authority) = (unsafe { (&*self.inner.authority.get()).as_ref() }) else { return WorkerJobClosePhase::CheckedOut };
        worker_authority_phase(authority.close_phase())
    }

    pub(super) fn contention(&self) -> WorkerJobContention {
        if self.inner.0.is_none(){return WorkerJobContention::TerminalEmpty}
        let generation = self.inner.generation;
        if self.inner.wake_exhausted.load(Ordering::Acquire) {
            return WorkerJobContention::WakeExhausted(generation);
        }
        let sequence = unsafe { (&*self.inner.authority.get()).as_ref().map_or(0, |authority| authority.sequence()) };
        let ticket = WorkerJobTicket { generation, step_sequence: sequence };
        match self.inner.phase() {
            SESSION_SUBMITTED => WorkerJobContention::Submitted(ticket),
            SESSION_OUTCOME => WorkerJobContention::Outcome(ticket),
            SESSION_TERMINAL => WorkerJobContention::Terminal(ticket),
            SESSION_REJECTED => WorkerJobContention::Rejected(generation),
            SESSION_CHECKED_OUT | SESSION_TRANSITION => WorkerJobContention::CheckedOut(generation),
            SESSION_CLOSE => WorkerJobContention::Closing(generation),
            SESSION_EMPTY => WorkerJobContention::TerminalEmpty,
            _ => WorkerJobContention::CheckedOut(generation),
        }
    }

    fn release_retirement_slot(&self, grant:RetainedCloneGrant) -> WorkerJobCloseStep {
        if self.retirement_state.load(Ordering::Acquire) == 3 {
            return WorkerJobCloseStep::Complete{progress:RetainedCloneProgress::default()};
        }
        let released_bytes=size_of::<WorkerJobRetirementNode<J>>();
        let copied_bytes=0;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_release_bytes<released_bytes||grant.maximum_depth==0||self.retirement_state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return WorkerJobCloseStep::Pending{progress:RetainedCloneProgress::default()};
        }
        let retirement = unsafe { (&mut *self.retirement.get()).take().expect("live worker session owns pre-admitted retirement node") };
        WORKER_JOB_RETIREMENT_SLOTS[retirement.header.slot].store(std::ptr::null_mut(), Ordering::Release);
        drop(retirement);
        self.retirement_state.store(3, Ordering::Release);
        WorkerJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:copied_bytes,released_bytes,..RetainedCloneProgress::default()}}
    }
}

impl<J: InteractiveJob + 'static> Drop for WorkerJobSession<J> {
    fn drop(&mut self) {
        if self.terminal_is_empty(){return}
        if let Some(original)=self.inner.0.as_ref(){original.close_requested.store(true,Ordering::Release);original.terminal_intent.store(1,Ordering::Release);original.raise_wake();}
        if self.retirement_state.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).is_err(){assert!(std::thread::panicking(),"original session Arc requires its admitted close turn");return}
        let mut retirement=unsafe{(&mut *self.retirement.get()).take().expect("live worker session owns original retirement node")};
        retirement.inner=self.inner.0.take();
        let slot=retirement.header.slot;
        let pointer=Box::into_raw(retirement).cast::<WorkerJobRetirementHeader>();
        WORKER_JOB_RETIREMENT_SLOTS[slot].store(pointer,Ordering::Release);
        self.retirement_state.store(2,Ordering::Release);
        WORKER_JOB_RETIREMENT_WAKE.store(true,Ordering::Release);
    }
}
//#endregion 🏭️RetainedSessions
