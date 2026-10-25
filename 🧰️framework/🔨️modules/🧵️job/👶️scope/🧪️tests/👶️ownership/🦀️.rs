use super::*;
use std::time::Instant;
/// ⏳️ A liveness bound, not a performance one: a loaded machine may deschedule the owner for long.
const WORKER_LIVENESS_BOUND: std::time::Duration = std::time::Duration::from_secs(30);
struct StructuredChildJob {
    backing: Option<Box<u8>>,
    closing: bool,
}

impl InteractiveJob for StructuredChildJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure> {JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        self.begin_close();if self.backing.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=RetirementDemand{release_bytes:self.next_close_release_byte_demand()?,depth:1,..Default::default()};close_original_retained_step(cx,demand,|_|{drop(self.backing.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.backing.is_some()))}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.backing.is_none()
    }
}

#[test]
fn child_registry_max_plus_one_stale_duplicate_exhaustion_and_parent_completion_are_exact() {
    let source=crate::component::TEST_RETAINED_POLICY;let mut caller=OriginalJobFixtureCaller::admit(OperationId(90_003),Generation(9),source,default_now_us,u64::MAX,1);let scope = JobScope::for_operation(caller.cancel(), OperationId(90_003), Generation(9));
    let mut guards: [Option<ChildJobGuard<'_, StructuredChildJob>>; JOB_CHILD_SLOTS] =
        std::array::from_fn(|index| Some(scope.spawn_child(StructuredChildJob { backing: Some(Box::new(index as u8)), closing: false }).unwrap_or_else(|_| panic!("fixed child slot"))));
    let retained_child_pointer = guards[1].as_mut().expect("second structured child").with_child_mut(|child| child.backing.as_deref().expect("retained structured child backing") as *const u8).expect("generation-qualified child checkout");
    assert_eq!(unsafe { *retained_child_pointer }, 1);
    let plus_one_backing = Box::new(211u8);
    let plus_one_pointer = plus_one_backing.as_ref() as *const u8;
    let rejected = match scope.spawn_child(StructuredChildJob { backing: Some(plus_one_backing), closing: false }) {
        Ok(_) => panic!("child maximum plus one must be rejected"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.fault, JobChildAdmissionFault::Capacity);
    assert_eq!(rejected.child().backing.as_deref().expect("rejected structured child backing") as *const u8, plus_one_pointer);
    let mut rejected_child = rejected.into_child();
    assert_eq!(rejected_child.backing.as_deref().expect("returned structured child backing") as *const u8, plus_one_pointer);
    rejected_child.begin_close();
    assert_eq!(caller.turn(RetainedCloneGrant{maximum_items:0,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()},|cx|rejected_child.close_step(cx)).unwrap(), InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !rejected_child.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = caller.turn(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:1,maximum_depth:64,..RetainedCloneGrant::default()},|cx|rejected_child.close_step(cx)).unwrap();
    }
    assert_eq!(scope.assert_completable(), Err(JobChildCompletionFault::LiveChildren));
    let token = guards[0].as_ref().expect("first child").token();
    guards[0].take().expect("first child").complete().expect("first exact completion");
    assert_eq!(scope.complete_child(token), Err(JobChildCompletionFault::Duplicate));
    let stale = JobChildToken { generation: token.generation + 1, ..token };
    assert_eq!(scope.complete_child(stale), Err(JobChildCompletionFault::Stale));
    drop(guards);
    assert_eq!(caller.turn(RetainedCloneGrant{maximum_items:0,maximum_depth:64,..Default::default()},|cx|scope.pump_child_close(cx)).unwrap(), InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    assert_eq!(caller.turn(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:JOB_PAYLOAD_PAGE_BYTES,maximum_depth:64},|cx|scope.pump_child_close(cx)).unwrap(), InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}}, "child begin-close transfers control without claiming an owner release");
    let close_started=Instant::now();let mut close_attempts=0;
    while !scope.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = caller.turn(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:JOB_PAYLOAD_PAGE_BYTES,maximum_depth:64},|cx|scope.pump_child_close(cx)).unwrap();
    }
    assert!(scope.assert_completable().is_ok());
    for slot in &scope.slots {
        slot.generation.store(u64::MAX, Ordering::Release);
        slot.state.store(CHILD_VACANT, Ordering::Release);
    }
    let rejected_backing = Box::new(7);
    let rejected_backing_pointer = rejected_backing.as_ref() as *const u8;
    let mut rejected = match scope.spawn_child(StructuredChildJob { backing: Some(rejected_backing), closing: false }) {
        Ok(_) => panic!("exhausted child generation must reject"),
        Err(rejected) => rejected,
    };
    assert_eq!(rejected.fault, JobChildAdmissionFault::Exhausted);
    assert_eq!(rejected.child().backing.as_deref().expect("exhausted rejected backing") as *const u8, rejected_backing_pointer);
    rejected.begin_close();
    assert_eq!(caller.turn(RetainedCloneGrant{maximum_items:0,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()},|cx|rejected.close_step(cx)).unwrap(), InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !rejected.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = caller.turn(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:1,maximum_depth:64,..RetainedCloneGrant::default()},|cx|rejected.close_step(cx)).unwrap();
    }
    scope.begin_close();
    assert!(scope.terminal_is_empty());let mut scope_cancel=semio_framework_async::CancelTokenRetirement::from_token(scope.cancel);while !scope_cancel.terminal_is_empty(){scope_cancel.close_step(source).unwrap();}caller.retire(source);
}

