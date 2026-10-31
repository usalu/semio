//! 📦️ Canonical original reserved producer custody and bounded physical return.
use super::*;

/// 🧹️ Required disposal side of an app-owned persistent job. `Complete` is an assertion
/// that dropping the concrete job immediately afterward is constant-time; no blanket/default
/// implementation exists because an erased job may own arbitrarily deep snapshots or queues.
pub trait ArtifactReservedJob: semio_framework_job::InteractiveJob + std::any::Any {}

/// 📏️ Sized ActionBus payload around one app-owned concrete resumable state machine.
pub struct ArtifactReservedToolJobState {
    pub(super) inner:ManuallyDrop<Option<Box<dyn ArtifactReservedJob+Send>>>,
    pub(super) operation:Option<semio_framework_job::Operation>,
    pub(super) lent:bool,
}

pub struct ArtifactReservedToolJob {
    pub(super) state:ManuallyDrop<Option<std::sync::Arc<semio_framework_value::retirement::controlled::RetainedOwnerGate<ArtifactReservedToolJobState>>>>,
    pub(super) held:ManuallyDrop<Option<Box<dyn ArtifactReservedJob+Send>>>,
}

/// 🧬️ A clone shares the gated state only; the concrete job is lent to at most one handle at a time and never aliased.
impl Clone for ArtifactReservedToolJob {
    fn clone(&self)->Self{Self{state:ManuallyDrop::new(self.state.as_ref().map(std::sync::Arc::clone)),held:ManuallyDrop::new(None)}}
}

impl ArtifactReservedToolJob {
    pub fn new<J:ArtifactReservedJob+Send+'static>(job:J)->Self {
        Self{state:ManuallyDrop::new(Some(std::sync::Arc::new(semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ArtifactReservedToolJobState{inner:ManuallyDrop::new(Some(Box::new(job))),operation:None,lent:false})))),held:ManuallyDrop::new(None)}
    }

    #[cfg(any(test, feature = "reserved-producer-testing"))]
    #[doc(hidden)]
    pub fn test_source_identity(&self)->Option<usize>{self.state.as_ref().map(|state|std::sync::Arc::as_ptr(state)as usize)}
    #[cfg(any(test, feature = "reserved-producer-testing"))]
    #[doc(hidden)]
    pub fn test_bound_operation(&self)->Option<semio_framework_job::Operation>{self.state.as_ref()?.try_lock().ok()?.operation}
    pub fn bind_operation(&mut self,operation:semio_framework_job::Operation)->Result<(),ValueError>{
        let source=self.state.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job state already returned"))?;
        let mut state=source.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job operation authority is busy"))?;
        if state.operation.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job operation authority was already bound"))}
        state.operation=Some(operation);Ok(())
    }

    fn close_demand(&self,kind:u8,copy:usize)->Result<usize,ValueError>{
        let Some(source)=self.state.as_ref()else{return Ok(0)};
        let state=source.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"reserved producer demand authority is borrowed"))?;
        if state.lent&&self.held.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"reserved producer job is lent to another handle"))}
        let Some(inner)=self.held.as_ref().or_else(||state.inner.as_ref())else{return Ok(if state.operation.is_some(){match kind{0=>size_of::<semio_framework_job::Operation>(),3=>1,_=>0}}else{match kind{0=>size_of::<Self>(),2=>Self::original_shared_frame_bytes(),3=>1,_=>0}})};
        if semio_framework_job::InteractiveJob::terminal_is_empty(&**inner){return Ok(match kind{2=>size_of_val(&**inner),3=>1,_=>0})}
        match kind{0=>inner.next_close_copy_byte_demand(),1=>inner.next_close_capacity_byte_demand(copy),2=>inner.next_close_release_byte_demand(),_=>inner.next_close_depth_demand()}
    }

    pub fn terminal_is_empty(&self)->bool{self.state.is_none()}

    /// 🪃️ Hands a lent job back to the gated state; false while the gate is busy.
    fn return_held(&mut self)->bool{
        let Some(inner)=self.held.take()else{return true};
        if let Some(source)=self.state.as_ref(){if let Ok(mut state)=source.try_lock(){*state.inner=Some(inner);state.lent=false;return true}}
        *self.held=Some(inner);false
    }

    pub fn begin_close(&mut self){self.return_held();if let Some(source)=self.state.as_ref(){if let Ok(mut state)=source.try_lock(){if let Some(inner)=state.inner.as_mut(){inner.begin_close()}}}}
}

impl semio_framework_job::InteractiveJob for ArtifactReservedToolJob {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,return_claim:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{
        use semio_framework_job::InteractiveJobStep as Step;
        let cancelled=|cx:&mut StepContext<'_>|Step::from_callback(JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into));
        let yielded=|cx:&mut StepContext<'_>|Step::from_callback(JobOutcomeBorrow::admit_yield(cx).map_err(Into::into));
        let Some(source)=self.state.as_ref()else{return cancelled(cx)};
        if self.held.is_none(){
            match source.try_lock(){
                Ok(mut state)=>{
                    if state.lent{return yielded(cx)}
                    match state.inner.take(){Some(inner)=>{state.lent=true;*self.held=Some(inner)},None=>return cancelled(cx)}
                },
                Err(_)=>return yielded(cx),
            }
        }
        match self.held.as_mut(){Some(inner)=>inner.step(cx,return_claim),None=>cancelled(cx)}
    }

    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{
        if let Some(inner)=self.held.as_ref(){return inner.borrow_outcome(descriptor)}
        let Some(source)=self.state.as_ref()else{return match descriptor.kind(){JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved producer state returned before outcome handback"))}};
        let state=source.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"reserved producer outcome authority is borrowed"))?;
        match descriptor.kind(){
            JobOutcomeKind::Cancelled=>descriptor.cancelled(),
            _ if state.inner.is_some()||state.lent=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"reserved producer outcome is borrowable only through the handle that stepped the job")),
            _=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved producer released before its outcome borrow")),
        }
    }

    fn begin_close(&mut self){ArtifactReservedToolJob::begin_close(self)}

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        if !self.return_held(){return Ok(Step::Blocked)}
        let Some(source)=self.state.as_ref()else{return Ok(Step::Complete{progress:Default::default()})};
        if cx.retirement_should_yield(){return Ok(Step::Pending{progress:Default::default()})}let grant=cx.retained_grant();
        let mut state=match source.try_lock(){Ok(state)=>state,Err(_)=>return Ok(Step::Blocked)};
        if let Some(inner)=state.inner.as_mut(){
            inner.begin_close();
            if semio_framework_job::InteractiveJob::terminal_is_empty(&**inner){let demand=RetirementDemand{release_bytes:size_of_val(&**inner),depth:1,..Default::default()};return semio_framework_job::close_original_retained_step(cx,demand,|_|{drop(state.inner.take());semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})),work:1}})}
            let step=semio_framework_job::InteractiveJob::close_step(&mut**inner,cx)?.admit(grant,semio_framework_job::InteractiveJob::terminal_is_empty(&**inner))?;return Ok(match step{Step::Complete{progress}=>Step::Pending{progress},step=>step})
        }
        if state.operation.is_some(){let demand=RetirementDemand{copy_bytes:size_of::<semio_framework_job::Operation>(),depth:1,..Default::default()};return semio_framework_job::close_original_retained_step(cx,demand,|_|{state.operation.take();semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}})}
        drop(state);
        let demand=RetirementDemand{copy_bytes:size_of::<Self>(),release_bytes:Self::original_shared_frame_bytes(),depth:1,..Default::default()};
        semio_framework_job::close_original_retained_step(cx,demand,|_|{let result=(||->Result<RetainedCloneStep,ValueError>{let original=std::sync::Arc::into_inner(self.state.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job state already returned"))?);let released_bytes=if let Some(original)=original{let original=original.into_inner();assert!(original.inner.is_none()&&original.operation.is_none(),"original shared producer frame still retains a body");drop(original);demand.release_bytes}else{0};Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()}))})();semio_framework_value::RetainedWorkOutcome{result,work:1}})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(0,0)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.close_demand(1,copy)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(2,0)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand(3,0)}
    fn terminal_is_empty(&self)->bool{ArtifactReservedToolJob::terminal_is_empty(self)}
}

impl Drop for ArtifactReservedToolJob {
    fn drop(&mut self){self.return_held();assert!(std::thread::panicking()||self.terminal_is_empty(),"original reserved producer requires its paid shared frame return")}
}

#[cfg(test)]
mod lending_tests {
    use super::*;

    static BEGIN_CLOSES:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
    struct Stub;
    impl semio_framework_job::InteractiveJob for Stub {
        fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(JobOutcomeBorrow::admit_yield(cx).map_err(Into::into))}
        fn borrow_outcome<'a>(&'a self,_:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"lending specimen has no output"))}
        fn begin_close(&mut self){BEGIN_CLOSES.fetch_add(1,std::sync::atomic::Ordering::SeqCst);}
        fn close_step(&mut self,_:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
        fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
        fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
        fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
        fn terminal_is_empty(&self)->bool{true}
    }
    impl ArtifactReservedJob for Stub {}

    fn retire(mut job:ArtifactReservedToolJob){
        assert!(job.return_held());
        let state=job.state.take().expect("state");
        let original=std::sync::Arc::into_inner(state).expect("sole owner").into_inner();
        drop(ManuallyDrop::into_inner(original.inner));
    }

    #[test]
    fn the_job_is_lent_to_one_handle_and_never_aliased_by_a_clone() {
        let mut stepper=ArtifactReservedToolJob::new(Stub);
        let clone=stepper.clone();
        assert!(clone.held.is_none(),"a clone never inherits the lent job");
        {
            let mut state=stepper.state.as_ref().expect("state").try_lock().expect("idle gate");
            *stepper.held=state.inner.take();
            state.lent=true;
        }
        assert!(clone.close_demand(0,0).is_err(),"a clone sees the job as lent to another handle");
        assert!(stepper.close_demand(0,0).is_ok(),"the lending handle still quotes its own job");
        assert!(stepper.return_held(),"the lent job returns to the gated state");
        assert!(stepper.held.is_none());
        assert!(clone.close_demand(0,0).is_ok(),"after the return every handle quotes the job again");
        let mut clone=clone;
        *clone.state=None;
        retire(stepper);
    }

    #[test]
    fn a_begin_close_lost_to_a_busy_gate_is_reissued_and_a_dropped_lender_hands_the_job_back() {
        let mut lender=ArtifactReservedToolJob::new(Stub);
        let mut other=lender.clone();
        {
            let mut state=lender.state.as_ref().expect("state").try_lock().expect("idle gate");
            *lender.held=state.inner.take();
            state.lent=true;
        }
        let before=BEGIN_CLOSES.load(std::sync::atomic::Ordering::SeqCst);
        {
            let gate=lender.state.as_ref().map(std::sync::Arc::clone).expect("state");
            let _busy=gate.try_lock().expect("idle gate");
            lender.begin_close();
            assert!(lender.held.is_some(),"a busy gate keeps the job with the lender");
        }
        assert!(other.close_demand(0,0).is_err(),"the job is still lent");
        *other.state=None;
        assert!(lender.return_held(),"the free gate takes the job back");
        lender.begin_close();
        assert_eq!(BEGIN_CLOSES.load(std::sync::atomic::Ordering::SeqCst),before+1,"begin_close reaches the job once the gate is free");
        retire(lender);
    }
}
