//! ⚙️ The engine's private-state laws: a child that reports beyond its grant is refused before owner placement, and an abandoned active cursor fails fast without running payload destructors. The public clone laws live in `public.rs`.
use crate::retained_clone::{FixtureSourceBirth,RetainedCloneProgress,RetainedCloneStep,settled,with_effect};
use crate::retirement::{RetirementCursor,RetirementStep};
use semio_framework_value_retirement::retirement_contract::RetirementPollExit;
use super::*;
use std::{
    mem::{ManuallyDrop, size_of},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

const CPU:u64=1048576;
const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1048576,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:33554432,maximum_depth:256};

#[repr(transparent)]
struct Element<T:RetainedClone>(T::Cursor);
impl<T:RetainedClone> PagedCloneElement<T> for Element<T>{fn new()->Self{Self(T::retained_clone_cursor())}
fn output_ready(&self)->bool{self.0.output_ready()}
fn advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.0.advance_demands(source,body,maximum_work_units)}
fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.0.advance_work_demand(source,body,maximum_work_units)}
fn advance(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementOutcome{self.0.advance(source,body,maximum_work_units,effect)}
fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.0.take_demand()}
fn take_work_demand(&self)->Result<u64,ValueError>{self.0.take_work_demand()}
fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{self.0.take(output,effect,reply)}
fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{self.0.begin_close_demand()}
fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.0.begin_close_work_demand()}
fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{self.0.begin_close_paid(effect)}
fn close_started(&self)->bool{self.0.close_started()}
fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.close_demands(body)}
fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.0.close_work_demand(body)}
fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();self.0.close_step(&mut effect.nested(grant))}
fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
}
type Cursor<T,const N:usize>=PagedListCloneEngine<T,Element<T>,N>;

fn close_fixture<T: RetainedClone, const N: usize>(cursor: &mut Cursor<T,N>, items: usize, work: usize) -> Result<RetainedCloneStep, OriginalRetirementFailure> {
    let d=cursor.close_demands(work).map_err(OriginalRetirementFailure::from)?;
    let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
    let mut actual=None;let turn=settled(with_effect(grant,CPU,&mut actual,|effect|cursor.close_step(effect)).0);assert!(turn.work<=1);let step=turn.result?;
    assert!(step.progress().fits(grant));
    Ok(step)
}

fn begin<T: RetainedClone, const N: usize>(cursor: &mut Cursor<T,N>) {
    let d=cursor.begin_close_demand().unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
    let mut actual=None;with_effect(grant,CPU,&mut actual,|effect|cursor.begin_close_paid(effect)).0.result.unwrap();
}

fn advance<T: RetainedClone, const N: usize>(cursor: &mut Cursor<T,N>, source: RetainedCloneRef<'_, PagedList<T,N>>, body: usize, grant: RetainedCloneGrant) -> OriginalRetirementOutcome {
    let mut actual=None;with_effect(grant,CPU,&mut actual,|effect|cursor.advance(source,body,CPU,effect)).0
}

fn fill<T, const N: usize>(source: &mut PagedList<T, N>) {
    while !source.has_reserved_slot() {
        let mut actual=None;
        let outcome=with_effect(STOCK,CPU,&mut actual,|effect|source.reserve_one(effect)).0.result.expect("fixture reserve");
        assert!(matches!(outcome,PagedListReservationOutcome::Ready(progress) if progress.progressed));
    }
}

fn binding_grant(copy: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy.max(RetainedCloneBinding::alias_copy_bytes()), maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 }
}

struct DropProbe {
    drops: Arc<AtomicUsize>,
}

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

struct DropProbeRetirement {
    value: ManuallyDrop<Option<DropProbe>>,
    released: bool,
}

impl RetirementCursor for DropProbeRetirement {
    fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(!self.released))}
    fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
    fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{
        if effect.grant().maximum_items == 0 || effect.maximum_work_units() == 0 { return RetirementStep::BudgetExhausted; }
        if self.released { return RetirementStep::Complete(0); }
        if self.value.as_ref().is_some_and(|value|Arc::strong_count(&value.drops)<=1){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"fixture drop counter requires its original external observer").into(),0);}
        let work=match retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};
        drop(self.value.take());
        self.released = true;
        RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()},work)
    }
    fn terminal_is_empty(&self) -> bool { self.released }
    fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

impl Drop for DropProbeRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.released, "drop probe retired before terminal-empty");
    }
}

impl RetireOwned for DropProbe {
    type RetirementCursor=DropProbeRetirement;
    fn retirement_seed(self)->Self::RetirementCursor{DropProbeRetirement { value: ManuallyDrop::new(Some(self)), released: false }}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<DropProbeRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

struct DropProbeCursor {
    output: Option<DropProbe>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
    close: RetainedCloneClose,
}

impl RetainedCloneCursor<DropProbe> for DropProbeCursor {
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,DropProbe>,_body:usize,maximum_work_units:u64)->Result<u64,ValueError>{if maximum_work_units==0||self.output_ready(){return Ok(0)}let work=if self.source.is_none(){source.binding_work_demand(&self.source)?}else{1};Ok(if work<=maximum_work_units{work}else{0})}
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.close_started()))}
    fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(retained_close_start_demand(self.closing))}
    fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{start_retained_close(&mut self.closing,effect)}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing||self.terminal_is_empty(){return Ok(0)}if !self.close.is_empty()||self.output.is_some(){return self.close.next_owner_work::<DropProbe>(self.output.is_some(),&self.source,body)}RetainedCloneBinding::close_work_demand(&self.source,body)}
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(Default::default());}self.close.next_owner_demands::<DropProbe>(self.output.is_some(),&self.source,body)}
    fn output_ready(&self) -> bool { !self.closing && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }

    fn advance_demands(&self,source:RetainedCloneRef<'_,DropProbe>,_body:usize,_maximum_work_units:u64)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand {copy_bytes:if !source.check_binding(&self.source)? {source.binding_copy_bytes()}else{size_of::<DropProbe>()},depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, DropProbe>, _body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>) -> OriginalRetirementOutcome {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,OriginalRetirementFailure>{
        if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let turn=source.bind(&mut self.source,effect);work=turn.work;if let Some(progress)=turn.result? { return Ok(RetainedCloneStep::Progress(progress)); }
        if self.output.is_some() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<DropProbe>(), retained_capacity_bytes: 0, released_bytes: 0 };
        if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"drop probe advance exceeded its original work").into())}
        work=1;self.output = Some(DropProbe { drops: Arc::clone(&source.get().drops) });
        Ok(RetainedCloneStep::Complete(progress))
    })();RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<RetirementDemand,ValueError>{retained_clone_take_demand::<DropProbe>()}
    fn take<'reply>(&mut self,output:&mut Option<DropProbe>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{if !self.output_ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}let turn=take_retained_clone_output(&mut self.output,output,effect);Ok(reply.complete(turn.result.map(|progress|if output.is_some(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})))}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close.next_owner_depth_with_binding(self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::copy_demand(&self.source)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{RetainedCloneBinding::capacity_demand(&self.source,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::release_demand(&self.source)}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.source.is_none() && self.close.is_empty()
    }

    fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,RetirementPollExit>{
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "drop probe close must begin").into()); }
        if !self.close.is_empty() { let turn=RetirementPollExit::settled(self.close.step_granted(effect))?;work=turn.work;return turn.result.map_err(Into::into); }
        let turn=self.close.begin_granted(&mut self.output,effect);work=turn.work;if let Some(step)=turn.result? { return Ok(step); }
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        {let turn=RetirementPollExit::settled(close_retained_binding(&mut self.source,effect))?;work+=turn.work;turn.result.map_err(Into::into)}
    })();RetirementPollExit::finish(result,work)}
}

impl RetainedClone for DropProbe {
    type Cursor = DropProbeCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        DropProbeCursor { output: None, source: None, closing: false, close: RetainedCloneClose::default() }
    }
}

#[derive(Clone, Copy)]
struct NonconformingChild;

impl RetireOwned for NonconformingChild {
    type RetirementCursor=retirement::Leaf<Self>;
    fn retirement_seed(self)->Self::RetirementCursor{retirement::leaf(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}

struct NonconformingChildCursor {
    source: Option<RetainedCloneBinding>,
    closing: bool,
}

impl RetainedCloneCursor<NonconformingChild> for NonconformingChildCursor {
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,NonconformingChild>,_body:usize,maximum_work_units:u64)->Result<u64,ValueError>{if maximum_work_units==0{return Ok(0)}let work=if self.source.is_none(){source.binding_work_demand(&self.source)?}else{1};Ok(if work<=maximum_work_units{work}else{0})}
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(0)}
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.close_started()))}
    fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(retained_close_start_demand(self.closing))}
    fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{start_retained_close(&mut self.closing,effect)}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing||self.terminal_is_empty(){return Ok(0)}RetainedCloneBinding::close_work_demand(&self.source,body)}
    fn output_ready(&self) -> bool { false }
    fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{assert!(self.closing,"fixture close must begin before its quote");RetainedCloneBinding::close_demand(&self.source,body)}

    fn advance_demands(&self,source:RetainedCloneRef<'_,NonconformingChild>,_body:usize,_maximum_work_units:u64)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand {copy_bytes:if !source.check_binding(&self.source)? {source.binding_copy_bytes()}else{size_of::<NonconformingChild>()},depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, NonconformingChild>, _body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>) -> OriginalRetirementOutcome {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,OriginalRetirementFailure>{
        if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let turn=source.bind(&mut self.source,effect);work=turn.work;if let Some(progress)=turn.result? { return Ok(RetainedCloneStep::Progress(progress)); }
        if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"nonconforming child advance exceeded its original work").into())}
        work=1;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: grant.maximum_items.saturating_add(1), copied_bytes: grant.maximum_copy_bytes.saturating_add(1), retained_capacity_bytes: 0, released_bytes: 0 }))
    })();RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
    fn take<'reply>(&mut self,_output:&mut Option<NonconformingChild>,_effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::depth_demand(&self.source)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::copy_demand(&self.source)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{RetainedCloneBinding::capacity_demand(&self.source,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::release_demand(&self.source)}
    fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{close_retained_binding(&mut self.source,effect)}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.source.is_none()
    }
}

impl RetainedClone for NonconformingChild {
    type Cursor = NonconformingChildCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        NonconformingChildCursor { source: None, closing: false }
    }
}

#[derive(Clone, Copy)]
struct NonconformingRetirementChild;

impl RetireOwned for NonconformingRetirementChild {
    type RetirementCursor=retirement::Leaf<Self>;
    fn retirement_seed(self)->Self::RetirementCursor{retirement::leaf(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}

struct NonconformingRetirementChildCursor {
    output: Option<NonconformingRetirementChild>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
    reported: bool,
}

impl RetainedCloneCursor<NonconformingRetirementChild> for NonconformingRetirementChildCursor {
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,NonconformingRetirementChild>,_body:usize,maximum_work_units:u64)->Result<u64,ValueError>{if maximum_work_units==0||self.output_ready(){return Ok(0)}let work=if self.source.is_none(){source.binding_work_demand(&self.source)?}else{1};Ok(if work<=maximum_work_units{work}else{0})}
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.close_started()))}
    fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(retained_close_start_demand(self.closing))}
    fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{start_retained_close(&mut self.closing,effect)}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing||self.terminal_is_empty(){return Ok(0)}if !self.reported{return Ok(1)}RetainedCloneBinding::close_work_demand(&self.source,body)}
    fn output_ready(&self) -> bool { !self.closing && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{assert!(self.closing,"fixture close must begin before its quote");if !self.reported{Ok(RetirementDemand{depth:1,..Default::default()})}else{RetainedCloneBinding::close_demand(&self.source,body)}}

    fn advance_demands(&self,source:RetainedCloneRef<'_,NonconformingRetirementChild>,_body:usize,_maximum_work_units:u64)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand {copy_bytes:if !source.check_binding(&self.source)? {source.binding_copy_bytes()}else{size_of::<NonconformingRetirementChild>()},depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, NonconformingRetirementChild>, _body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>) -> OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,OriginalRetirementFailure>{
        if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let turn=source.bind(&mut self.source,effect);work=turn.work;if let Some(progress)=turn.result? { return Ok(RetainedCloneStep::Progress(progress)); }
        if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"nonconforming retirement child advance exceeded its original work").into())}
        work=1;self.output = Some(NonconformingRetirementChild);
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    })();RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<RetirementDemand,ValueError>{retained_clone_take_demand::<NonconformingRetirementChild>()}
    fn take<'reply>(&mut self,output:&mut Option<NonconformingRetirementChild>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{if !self.output_ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}let turn=take_retained_clone_output(&mut self.output,output,effect);Ok(reply.complete(turn.result.map(|progress|if output.is_some(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})))}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::depth_demand(&self.source)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::copy_demand(&self.source)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{RetainedCloneBinding::capacity_demand(&self.source,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::release_demand(&self.source)}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.reported && self.source.is_none() && self.output.is_none()
    }

    fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,RetirementPollExit>{
        if !self.reported {
            if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"nonconforming retirement child close exceeded its original work").into())}
            work=1;self.reported = true;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: grant.maximum_items.saturating_add(1), copied_bytes: 0, retained_capacity_bytes: grant.maximum_capacity_bytes.saturating_add(1), released_bytes: grant.maximum_release_bytes.saturating_add(1) }));
        }
        {let turn=RetirementPollExit::settled(close_retained_binding(&mut self.source,effect))?;work+=turn.work;turn.result.map_err(Into::into)}
    })();RetirementPollExit::finish(result,work)}
}

impl RetainedClone for NonconformingRetirementChild {
    type Cursor = NonconformingRetirementChildCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        NonconformingRetirementChildCursor { output: None, source: None, closing: false, reported: false }
    }
}

const REFUSAL:&str="original retained turn exceeded its item, copy, capacity, or release grant";

fn drain<T: RetainedClone, const N: usize>(cursor: &mut Cursor<T,N>) {
    let mut bounded_close_turns=0;
    while !cursor.terminal_is_empty() {
        bounded_close_turns+=1;assert!(bounded_close_turns<100000,"retained fixture closure must terminate");
        let step = close_fixture(cursor, 1, 4096).expect("fixture child closes after its refused report");
        if let RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:released_items,released_bytes,..}) = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= 4096);
        }
    }
}

#[test]
fn retained_paged_list_refuses_over_budget_child_before_owner_placement() {
    let mut source = PagedList::<NonconformingChild, 1>::default();
    fill(&mut source);
    assert!(source.push_reserved(NonconformingChild).is_ok());
    let retained = crate::retained_clone::RetainedCloneSource::from_owner(source);
    let mut cursor = Cursor::<NonconformingChild, 1>::default();
    let body = RetainedCloneBinding::alias_copy_bytes().max(size_of::<NonconformingChildCursor>());
    let grant = binding_grant(body);
    let mut error_turns=0;
    let error = loop {
        error_turns+=1;assert!(error_turns<100000,"adversarial retained fixture must terminate");
        let turn=advance(&mut cursor,retained.borrow(),body,grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(_)) => {}
            Ok(RetainedCloneStep::Complete(_)) => panic!("over-budget child must not complete"),
            Err(error) => break error,
        }
    };
    let primary=error.primary().expect("typed refusal");
    assert_eq!(primary.kind,ValueRefusalKind::InvariantViolated);
    assert_eq!(primary.message.as_ref(),REFUSAL);
    assert!(matches!(primary.message,std::borrow::Cow::Borrowed(_)));
    assert_eq!(cursor.index, 0);
    assert_eq!(cursor.values.len(), 0);
    assert!(cursor.child_value.is_none());
    begin(&mut cursor);
    drain(&mut cursor);
}

#[test]
fn retained_paged_list_refuses_over_budget_child_retirement_before_owner_placement() {
    let mut source = PagedList::<NonconformingRetirementChild, 1>::default();
    fill(&mut source);
    assert!(source.push_reserved(NonconformingRetirementChild).is_ok());
    let retained = crate::retained_clone::RetainedCloneSource::from_owner(source);
    let mut cursor = Cursor::<NonconformingRetirementChild, 1>::default();
    let body = RetainedCloneBinding::alias_copy_bytes().max(size_of::<NonconformingRetirementChildCursor>());
    let grant = binding_grant(body);
    let mut error_turns=0;
    loop {
        error_turns+=1;assert!(error_turns<100000,"adversarial retained fixture must terminate");
        let turn=advance(&mut cursor,retained.borrow(),body,grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(_)) => {}
            Ok(RetainedCloneStep::Complete(_)) => panic!("over-budget child retirement must be refused before completion"),
            Err(error) => {
                let primary=error.primary().expect("typed refusal");
                assert_eq!(primary.kind,ValueRefusalKind::InvariantViolated);
                assert_eq!(primary.message.as_ref(),REFUSAL);
                assert!(matches!(primary.message,std::borrow::Cow::Borrowed(_)));
                break;
            }
        }
    }
    assert_eq!(cursor.index, 0);
    assert_eq!(cursor.values.len(), 0);
    assert!(cursor.child_value.is_some());
    begin(&mut cursor);
    drain(&mut cursor);
}

#[test]
fn retained_paged_list_abandonment_panics_without_running_payload_destructors() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut source = PagedList::<DropProbe, 64>::default();
    for _ in 0..64 {
        fill(&mut source);
        assert!(source.push_reserved(DropProbe { drops: Arc::clone(&drops) }).is_ok());
    }
    let retained = crate::retained_clone::RetainedCloneSource::from_owner(source);
    let mut cursor = Cursor::<DropProbe, 64>::default();
    let grant = RetainedCloneGrant { maximum_items: 5, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    while cursor.values.len() < 8 {
        assert!(matches!(advance(&mut cursor,retained.borrow(),4096,grant).result.expect("drop probe prefix"), RetainedCloneStep::Progress(_)));
    }
    let abandoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(cursor)));
    assert!(abandoned.is_err(), "active cursor abandonment fails fast");
    assert_eq!(drops.load(Ordering::SeqCst), 0, "abandonment retains every partial page and payload owner");
    drop(retained);
}
