//! 🧵️ Canonical 👶️scope implementation under its own native owner.
use semio_framework_job::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::sync::atomic::{AtomicBool,AtomicPtr,AtomicU8,AtomicU32,AtomicU64,AtomicUsize,Ordering};
use std::mem::ManuallyDrop;
use std::task::Waker;
//#region 👶️JobScope

pub const JOB_CHILD_SLOTS: usize = 64;

const CHILD_VACANT: u8 = 0;
const CHILD_LIVE: u8 = 1;
const CHILD_CLOSE_INTENT: u8 = 2;
const CHILD_EXHAUSTED: u8 = 3;
const CHILD_CHECKED_OUT: u8 = 4;

struct JobChildSlot {
    generation: AtomicU64,
    state: AtomicU8,
    node: AtomicPtr<JobChildNodeHeader>,
}

impl JobChildSlot {
    fn vacant() -> Self {
        Self { generation: AtomicU64::new(0), state: AtomicU8::new(CHILD_VACANT), node: AtomicPtr::new(std::ptr::null_mut()) }
    }
}

#[repr(C)]
struct JobChildNodeHeader {
    pump: unsafe fn(*mut JobChildNodeHeader, &mut StepContext<'_>) -> Result<InteractiveJobCloseStep,OriginalTurnFailure>,
    demands:unsafe fn(*mut JobChildNodeHeader,usize)->Result<RetirementDemand,ValueError>,
    release_bytes:usize,
    destroy: unsafe fn(*mut JobChildNodeHeader),
}

unsafe fn job_child_node_close_demands<J:InteractiveJob>(pointer:*mut JobChildNodeHeader,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
    let node=unsafe{&*pointer.cast::<JobChildNode<J>>()};
    if node.close_stage==1{
        let child=node.child.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"closing child node lacks original child"))?;
        return Ok(RetirementDemand{copy_bytes:child.next_close_copy_byte_demand()?,capacity_bytes:child.next_close_capacity_byte_demand(maximum_copy_bytes)?,release_bytes:child.next_close_release_byte_demand()?,depth:child.next_close_depth_demand()?})
    }
    Ok(RetirementDemand{copy_bytes:0,release_bytes:if node.close_stage==3{node.header.release_bytes}else{0},depth:1,..Default::default()})
}

#[repr(C)]
struct JobChildNode<J> {
    header: JobChildNodeHeader,
    child: Option<J>,
    close_stage: u8,
}

unsafe fn pump_job_child_node<J: InteractiveJob>(pointer:*mut JobChildNodeHeader,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
    let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
    let node=unsafe{&mut *pointer.cast::<JobChildNode<J>>()};
    if node.close_stage==0{node.child.as_mut().expect("live original child").begin_close();node.close_stage=1;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_fuel(1);cx.consume_retained(progress)?;return Ok(InteractiveJobCloseStep::Pending{progress})}
    if node.close_stage==1{
        let child=node.child.as_mut().expect("closing original child");
        if child.terminal_is_empty(){node.close_stage=2;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_fuel(1);cx.consume_retained(progress)?;return Ok(InteractiveJobCloseStep::Pending{progress})}
        let step=child.close_step(cx)?.admit(grant,child.terminal_is_empty())?;return Ok(match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})
    }
    if node.close_stage==2{drop(node.child.take());node.close_stage=3;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_fuel(1);cx.consume_retained(progress)?;return Ok(InteractiveJobCloseStep::Pending{progress})}
    Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
}

unsafe fn destroy_job_child_node<J>(pointer: *mut JobChildNodeHeader) {
    drop(unsafe { Box::from_raw(pointer.cast::<JobChildNode<J>>()) });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobChildToken {
    pub parent_operation: OperationId,
    pub parent_generation: Generation,
    pub slot: u16,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobChildAdmissionFault {
    Capacity,
    Exhausted,
    Closing,
}

pub struct JobChildAdmissionRejected<J> {
    pub fault: JobChildAdmissionFault,
    child: ManuallyDrop<Option<J>>,
    closing: bool,
    close_stage: u8,
}

impl<J> JobChildAdmissionRejected<J> {
    pub fn child(&self) -> &J {
        self.child.as_ref().expect("rejected structured child owner remains exact")
    }

    pub fn into_child(mut self) -> J {
        self.child.take().expect("rejected structured child owner remains exact")
    }
}

impl<J: InteractiveJob> JobChildAdmissionRejected<J> {
    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(child) = self.child.as_mut() {
            child.begin_close();
        }
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}self.begin_close();
        if self.close_stage==0{
            if let Some(child)=self.child.as_mut(){if !child.terminal_is_empty(){let step=child.close_step(cx)?.admit(grant,child.terminal_is_empty())?;return Ok(match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})}}
            self.close_stage=1;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_fuel(1);cx.consume_retained(progress)?;return Ok(InteractiveJobCloseStep::Pending{progress})
        }
        if self.close_stage==1{drop(self.child.take());self.close_stage=2;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_fuel(1);cx.consume_retained(progress)?;return Ok(InteractiveJobCloseStep::Pending{progress})}
        Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.child.is_none()
    }
}

impl<J> Drop for JobChildAdmissionRejected<J> {
    fn drop(&mut self) {
        debug_assert!(self.child.is_none(), "rejected structured child requires exact handback");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobChildCompletionFault {
    LiveChildren,
    Stale,
    Duplicate,
}

pub struct JobScope {
    cancel: CancelToken,
    parent_operation: OperationId,
    parent_generation: Generation,
    slots: [JobChildSlot; JOB_CHILD_SLOTS],
    live_children: AtomicU32,
    closing: AtomicBool,
    wake_pending: AtomicBool,
}

impl JobScope {
    pub fn root() -> JobScope {
        JobScope::for_operation(&root_cancel_token(), OperationId(0), Generation(0))
    }

    pub fn child_of(parent: &CancelToken) -> JobScope {
        JobScope::for_operation(parent, OperationId(0), Generation(0))
    }

    pub fn for_operation(parent: &CancelToken, parent_operation: OperationId, parent_generation: Generation) -> JobScope {
        JobScope {
            cancel: child_cancel_token(parent),
            parent_operation,
            parent_generation,
            slots: std::array::from_fn(|_| JobChildSlot::vacant()),
            live_children: AtomicU32::new(0),
            closing: AtomicBool::new(false),
            wake_pending: AtomicBool::new(false),
        }
    }

    pub fn cancel_token(&self) -> CancelToken {
        self.cancel.clone()
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel.is_cancelled_now()
    }

    pub fn spawn_child<J: InteractiveJob + 'static>(&self, child: J) -> Result<ChildJobGuard<'_, J>, JobChildAdmissionRejected<J>> {
        if self.closing.load(Ordering::Acquire) || self.is_cancelled() {
            return Err(JobChildAdmissionRejected { fault: JobChildAdmissionFault::Closing, child: ManuallyDrop::new(Some(child)), closing: false, close_stage: 0 });
        }
        let mut child = Some(child);
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.state.load(Ordering::Acquire) == CHILD_EXHAUSTED {
                continue;
            }
            if slot.state.compare_exchange(CHILD_VACANT, CHILD_LIVE, Ordering::AcqRel, Ordering::Acquire).is_err() {
                continue;
            }
            let previous = slot.generation.load(Ordering::Acquire);
            let Some(generation) = previous.checked_add(1) else {
                slot.state.store(CHILD_EXHAUSTED, Ordering::Release);
                continue;
            };
            let node = Box::new(JobChildNode { header: JobChildNodeHeader { pump: pump_job_child_node::<J>, demands:job_child_node_close_demands::<J>,release_bytes:size_of::<JobChildNode<J>>(), destroy: destroy_job_child_node::<J> }, child: child.take(), close_stage: 0 });
            slot.generation.store(generation, Ordering::Release);
            slot.node.store(Box::into_raw(node).cast::<JobChildNodeHeader>(), Ordering::Release);
            self.live_children.fetch_add(1, Ordering::AcqRel);
            if self.closing.load(Ordering::Acquire) || self.is_cancelled() {
                let _ = slot.state.compare_exchange(CHILD_LIVE, CHILD_CLOSE_INTENT, Ordering::AcqRel, Ordering::Acquire);
                self.raise_wake();
            }
            let token = JobChildToken { parent_operation: self.parent_operation, parent_generation: self.parent_generation, slot: index as u16, generation };
            return Ok(ChildJobGuard { scope: self, token: Some(token), marker: std::marker::PhantomData });
        }
        let exhausted = self.slots.iter().all(|slot| slot.state.load(Ordering::Acquire) == CHILD_EXHAUSTED);
        Err(JobChildAdmissionRejected { fault: if exhausted { JobChildAdmissionFault::Exhausted } else { JobChildAdmissionFault::Capacity }, child: ManuallyDrop::new(child), closing: false, close_stage: 0 })
    }

    pub fn live_child_count(&self) -> u32 {
        self.live_children.load(Ordering::SeqCst)
    }

    pub fn has_live_children(&self) -> bool {
        self.live_child_count() > 0 || self.slots.iter().any(|slot| matches!(slot.state.load(Ordering::Acquire), CHILD_LIVE | CHILD_CLOSE_INTENT | CHILD_CHECKED_OUT))
    }

    pub fn assert_completable(&self) -> Result<(), JobChildCompletionFault> {
        if self.has_live_children() { Err(JobChildCompletionFault::LiveChildren) } else { Ok(()) }
    }

    pub fn begin_close(&self) {
        self.closing.store(true, Ordering::Release);
        self.cancel.cancel_now();
        for slot in &self.slots {
            let _ = slot.state.compare_exchange(CHILD_LIVE, CHILD_CLOSE_INTENT, Ordering::AcqRel, Ordering::Acquire);
        }
        self.raise_wake();
    }

    /// 🔐️ Reads all original child demands under the same exclusive node admission as its close.
    pub fn child_retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
        for slot in &self.slots{
            if slot.state.compare_exchange(CHILD_CLOSE_INTENT,CHILD_CHECKED_OUT,Ordering::AcqRel,Ordering::Acquire).is_err(){continue}
            let pointer=slot.node.load(Ordering::Acquire);
            let demand=if pointer.is_null(){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closing scope child lacks original node"))}else{unsafe{((*pointer).demands)(pointer,maximum_copy_bytes)}};
            slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return demand;
        }
        Ok(RetirementDemand::default())
    }

    pub fn pump_child_close(&self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        for slot in &self.slots{
            if slot.state.compare_exchange(CHILD_CLOSE_INTENT,CHILD_CHECKED_OUT,Ordering::AcqRel,Ordering::Acquire).is_err(){continue}
            let pointer=slot.node.load(Ordering::Acquire);if pointer.is_null(){slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return Ok(InteractiveJobCloseStep::Blocked)}
            let demand=match unsafe{((*pointer).demands)(pointer,grant.maximum_copy_bytes)}{Ok(demand)=>demand,Err(error)=>{slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return Err(error.into())}};
            if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
            let step=match unsafe{((*pointer).pump)(pointer,cx)}{Ok(step)=>step,Err(failure)=>{slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return Err(failure)}};
            if matches!(step,InteractiveJobCloseStep::Complete{..}){
                let release=unsafe{(*pointer).release_bytes};let remaining=cx.retained_grant();if step.progress()!=RetainedCloneProgress::default()||remaining.maximum_items==0||remaining.maximum_release_bytes<release||remaining.maximum_depth==0||cx.should_yield(){slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}
                let pointer=slot.node.swap(std::ptr::null_mut(),Ordering::AcqRel);unsafe{((*pointer).destroy)(pointer)};self.live_children.fetch_sub(1,Ordering::AcqRel);slot.state.store(if slot.generation.load(Ordering::Acquire)==u64::MAX{CHILD_EXHAUSTED}else{CHILD_VACANT},Ordering::Release);self.raise_wake();let progress=RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()};cx.consume_fuel(1);cx.consume_retained(progress)?;return Ok(InteractiveJobCloseStep::Pending{progress})
            }
            slot.state.store(CHILD_CLOSE_INTENT,Ordering::Release);return Ok(step)
        }
        Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }

    pub fn take_wake(&self) -> bool {
        self.wake_pending.swap(false, Ordering::AcqRel)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.live_child_count() == 0 && self.slots.iter().all(|slot| matches!(slot.state.load(Ordering::Acquire), CHILD_VACANT | CHILD_EXHAUSTED) && slot.node.load(Ordering::Acquire).is_null())
    }

    fn complete_child(&self, token: JobChildToken) -> Result<(), JobChildCompletionFault> {
        if token.parent_operation != self.parent_operation || token.parent_generation != self.parent_generation {
            return Err(JobChildCompletionFault::Stale);
        }
        let Some(slot) = self.slots.get(token.slot as usize) else { return Err(JobChildCompletionFault::Stale) };
        if slot.generation.load(Ordering::Acquire) != token.generation {
            return Err(JobChildCompletionFault::Stale);
        }
        let state = slot.state.load(Ordering::Acquire);
        if state == CHILD_CLOSE_INTENT {
            return Err(JobChildCompletionFault::Duplicate);
        }
        if state != CHILD_LIVE {
            return Err(JobChildCompletionFault::Duplicate);
        }
        slot.state.compare_exchange(CHILD_LIVE, CHILD_CLOSE_INTENT, Ordering::AcqRel, Ordering::Acquire).map_err(|_| JobChildCompletionFault::Duplicate)?;
        self.raise_wake();
        Ok(())
    }

    fn raise_wake(&self) {
        self.wake_pending.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).ok();
    }
}

pub struct ChildJobGuard<'a, J: InteractiveJob + 'static> {
    scope: &'a JobScope,
    token: Option<JobChildToken>,
    marker: std::marker::PhantomData<J>,
}

impl<J: InteractiveJob + 'static> ChildJobGuard<'_, J> {
    pub fn token(&self) -> JobChildToken {
        self.token.expect("live child guard owns token")
    }

    pub fn complete(mut self) -> Result<(), JobChildCompletionFault> {
        let token = self.token.take().expect("live child guard owns token");
        self.scope.complete_child(token)
    }

    pub fn with_child_mut<R>(&mut self, use_child: impl FnOnce(&mut J) -> R) -> Result<R, JobChildCompletionFault> {
        let token = self.token.expect("live structured child guard owns token");
        if token.parent_operation != self.scope.parent_operation || token.parent_generation != self.scope.parent_generation {
            return Err(JobChildCompletionFault::Stale);
        }
        let slot = self.scope.slots.get(token.slot as usize).ok_or(JobChildCompletionFault::Stale)?;
        if slot.generation.load(Ordering::Acquire) != token.generation {
            return Err(JobChildCompletionFault::Stale);
        }
        slot.state.compare_exchange(CHILD_LIVE, CHILD_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).map_err(|_| JobChildCompletionFault::Duplicate)?;
        struct Handback<'a> {
            scope: &'a JobScope,
            slot: &'a JobChildSlot,
        }
        impl Drop for Handback<'_> {
            fn drop(&mut self) {
                self.slot.state.store(if self.scope.closing.load(Ordering::Acquire) { CHILD_CLOSE_INTENT } else { CHILD_LIVE }, Ordering::Release);
                self.scope.raise_wake();
            }
        }
        let handback = Handback { scope: self.scope, slot };
        let pointer = slot.node.load(Ordering::Acquire);
        if pointer.is_null() {
            return Err(JobChildCompletionFault::Stale);
        }
        let child = unsafe { (&mut *pointer.cast::<JobChildNode<J>>()).child.as_mut().expect("checked-out structured child node owns exact child") };
        let result = use_child(child);
        drop(handback);
        Ok(result)
    }
}

impl<J: InteractiveJob + 'static> Drop for ChildJobGuard<'_, J> {
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            let _ = self.scope.complete_child(token);
        }
    }
}
//#endregion 👶️JobScope

#[cfg(test)]
const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};
#[cfg(test)]
#[global_allocator]
static JOB_OWNER_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[cfg(test)]
#[path="🧪️tests/👶️ownership/🦀️.rs"]
mod retained_ownership_tests;
