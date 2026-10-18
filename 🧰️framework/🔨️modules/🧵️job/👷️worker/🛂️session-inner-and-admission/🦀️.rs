//! 🛂️ Session inner and admission: the shared session inner custody, the session handle, the admission rejection and the submission records. See [`super`].

use super::*;

//#region 🏭️RetainedSessions
pub(crate) struct WorkerJobSessionInner<J> {
    pub(super) generation: Generation,
    pub(super) phase: AtomicU8,
    pub(super) authority: ManuallyDrop<std::cell::UnsafeCell<Option<OriginalWorkerJobAuthority<J>>>>,
    pub(super) rejection_kind: AtomicU8,
    pub(super) close_requested: AtomicBool,
    pub(super) terminal_intent: AtomicU8,
    pub(super) wake_pending: AtomicBool,
    pub(super) wake_sequence: AtomicU64,
    pub(super) wake_exhausted: AtomicBool,
    pub(super) wake_guard: AtomicBool,
    pub(super) waker: ManuallyDrop<std::cell::UnsafeCell<Option<Waker>>>,
    pub(super) retained_wake:ManuallyDrop<std::cell::UnsafeCell<Option<Box<dyn RetainedWorkerWake>>>>,
    pub(super) last_step_end_us: AtomicU64,
}

unsafe impl<J: Send> Send for WorkerJobSessionInner<J> {}

unsafe impl<J: Send> Sync for WorkerJobSessionInner<J> {}

impl<J> WorkerJobSessionInner<J> {
    pub(super) fn phase(&self) -> u8 {
        self.phase.load(Ordering::Acquire)
    }

    pub(super) unsafe fn take_authority(&self) -> OriginalWorkerJobAuthority<J> {
        unsafe { (&mut *self.authority.get()).take().expect("session phase owns exact authority") }
    }

    pub(super) unsafe fn put_authority(&self, authority: OriginalWorkerJobAuthority<J>, phase: u8) {
        unsafe {
            let storage = &mut *self.authority.get();
            assert!(storage.is_none(), "session transition cannot overwrite an authority");
            *storage = Some(authority);
        }
        self.phase.store(phase, Ordering::Release);
        self.raise_wake();
    }

    pub(super) unsafe fn put_authority_quiet(&self, authority: OriginalWorkerJobAuthority<J>, phase: u8) {
        unsafe {
            let storage = &mut *self.authority.get();
            assert!(storage.is_none(), "session transition cannot overwrite an authority");
            *storage = Some(authority);
        }
        self.phase.store(phase, Ordering::Release);
    }

    pub(super) fn raise_wake(&self) {
        if self.wake_pending.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        if self.wake_sequence.try_update(Ordering::AcqRel, Ordering::Acquire, |sequence| sequence.checked_add(1)).is_err() {
            self.wake_exhausted.store(true, Ordering::Release);
        }
        if self.wake_guard.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            if let Some(original)=unsafe{(&*self.retained_wake.get()).as_ref()}{original.wake_by_ref();}
            let waker = unsafe { (&mut *self.waker.get()).take() };
            self.wake_guard.store(false, Ordering::Release);
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }

    pub(super) fn register_waker(&self, waker: &Waker) -> Result<(), WorkerJobContention> {
        if self.wake_exhausted.load(Ordering::Acquire) {
            return Err(WorkerJobContention::WakeExhausted(self.generation));
        }
        if self.wake_guard.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(WorkerJobContention::CheckedOut(self.generation));
        }
        if unsafe{(&*self.retained_wake.get()).is_some()}{self.wake_guard.store(false,Ordering::Release);return Err(WorkerJobContention::CheckedOut(self.generation))}
        let wake_now = unsafe {
            *self.waker.get() = Some(waker.clone());
            if self.wake_pending.load(Ordering::Acquire) { (&mut *self.waker.get()).take() } else { None }
        };
        self.wake_guard.store(false, Ordering::Release);
        if let Some(waker) = wake_now {
            waker.wake();
        }
        Ok(())
    }
}

impl<J> Drop for WorkerJobSessionInner<J> {
    fn drop(&mut self) {
        if self.phase.load(Ordering::Acquire) == SESSION_EMPTY {
            unsafe {
                ManuallyDrop::drop(&mut self.authority);
                ManuallyDrop::drop(&mut self.waker);
                ManuallyDrop::drop(&mut self.retained_wake);
            }
        }
    }
}

pub(super) struct WorkerJobSessionArc<J>(pub(super) Option<session_return::SessionHandle<J>>);

impl<J> std::ops::Deref for WorkerJobSessionArc<J>{type Target=session_return::SessionHandle<J>;fn deref(&self)->&Self::Target{self.0.as_ref().expect("live session retains its original Arc")}}

pub struct WorkerJobSession<J: InteractiveJob + 'static> {
    pub(super) inner: WorkerJobSessionArc<J>,
    pub(super) generation: Generation,
    pub(super) terminal_step_end_us: Option<u64>,
    pub(super) retirement: std::cell::UnsafeCell<Option<Box<WorkerJobRetirementNode<J>>>>,
    pub(super) retirement_state: AtomicU8,
}

unsafe impl<J: InteractiveJob + 'static> Send for WorkerJobSession<J> {}

unsafe impl<J: InteractiveJob + 'static> Sync for WorkerJobSession<J> {}

pub struct WorkerJobSessionAdmissionRejected<J> {
    pub(super) job: ManuallyDrop<Option<J>>,
    pub(super) params: ManuallyDrop<Option<BatchJobParams>>,
    fault_source: ManuallyDrop<Option<JobPayloadPageSource>>,
    pub(super) closing: bool,
    close_stage: u8,
}

impl<J> WorkerJobSessionAdmissionRejected<J> {
    pub fn job(&self) -> &J {
        self.job.as_ref().expect("rejected worker session admission owns exact job")
    }

    pub fn fault_backing_identity(&self) -> Option<*const MaybeUninit<u8>> {
        self.fault_source.as_ref().map(JobPayloadPageSource::backing_identity)
    }
}

impl<J: InteractiveJob> WorkerJobSessionAdmissionRejected<J> {
    /// 🪙️ Publish the physical demand of the next retained rejection owner.
    pub fn retirement_demands(&self,maximum_copy_bytes:usize) -> Result<RetirementDemand,ValueError> {
        if self.close_stage==0{
            if let Some(job)=self.job.as_ref(){if !job.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(maximum_copy_bytes)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?})}}
        }
        Ok(RetirementDemand{release_bytes:if self.close_stage>1&&self.params.is_none(){self.fault_source.as_ref().map_or(0,JobPayloadPageSource::allocated_capacity_bytes)}else{0},depth:1,..RetirementDemand::default()})
    }

    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(job) = self.job.as_mut() {
            job.begin_close();
        }
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}let grant=cx.retained_grant();self.begin_close();
        if self.close_stage==0{if let Some(job)=self.job.as_mut(){let step=job.close_step(cx)?;let step=step.admit(grant,job.terminal_is_empty())?;return Ok(match step{InteractiveJobCloseStep::Complete{progress}=>{self.close_stage=1;InteractiveJobCloseStep::Pending{progress}},step=>step})}self.close_stage=2;return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let progress=if self.close_stage==1{drop(self.job.take());self.close_stage=2;RetainedCloneProgress{copied_items:1,..Default::default()}}else if self.params.is_some(){drop(self.params.take());self.close_stage=3;RetainedCloneProgress{copied_items:1,..Default::default()}}else if let Some(source)=self.fault_source.as_ref(){let extent=source.allocated_capacity_bytes();if grant.maximum_release_bytes<extent{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}drop(self.fault_source.take());self.close_stage=4;RetainedCloneProgress{copied_items:1,released_bytes:extent,..Default::default()}}else{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})};cx.consume_fuel(1);cx.consume_retained(progress)?;Ok(InteractiveJobCloseStep::Pending{progress})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.params.is_none() && self.fault_source.is_none()
    }
}

impl<J> Drop for WorkerJobSessionAdmissionRejected<J> {
    fn drop(&mut self) {
        debug_assert!(self.job.is_none() && self.params.is_none() && self.fault_source.is_none(), "rejected worker session admission requires exact incremental close");
    }
}

pub(super) struct WorkerJobSubmission<J> {
    pub(super) inner: session_return::SessionHandle<J>,
    pub(super) authority: Option<OriginalWorkerJobAuthority<J>>,
    pub(super) ran: bool,
}

impl<J: InteractiveJob + 'static> WorkerJobSubmission<J> {
    pub(super) fn run(mut self) {
        self.ran = true;
        let mut authority = self.authority.take().expect("submitted worker closure owns exact job authority");
        let terminal = authority.run_staged_callback();
        self.inner.last_step_end_us.store(authority.last_step_end_us().unwrap_or(u64::MAX), Ordering::Release);
        if self.inner.close_requested.load(Ordering::Acquire) {
            self.inner.terminal_intent.store(1, Ordering::Release);
        }
        unsafe { self.inner.put_authority(authority, if terminal { SESSION_TERMINAL } else { SESSION_OUTCOME }) };
    }
}

impl<J> Drop for WorkerJobSubmission<J> {
    fn drop(&mut self) {
        let Some(authority) = self.authority.take() else { return };
        let rejected = self.inner.rejection_kind.load(Ordering::Acquire) != u8::MAX;
        unsafe { self.inner.put_authority(authority, if rejected { SESSION_REJECTED } else { SESSION_CLOSE }) };
    }
}
//#endregion 🏭️RetainedSessions
