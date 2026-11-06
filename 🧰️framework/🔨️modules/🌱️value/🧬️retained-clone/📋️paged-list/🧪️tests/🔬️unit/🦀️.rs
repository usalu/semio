use semio_framework_test_fixtures::fixture_text;
use crate::retained_clone::FixtureSourceBirth;
use super::*;
use serde::Deserialize;
use std::{
    mem::{ManuallyDrop, size_of},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

fn close_fixture<T: RetainedClone>(cursor: &mut impl RetainedCloneCursor<T>, items: usize, work: usize) -> Result<RetainedCloneStep, crate::ValueError> {
    let d=cursor.close_demands(work)?;
    let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(cursor.close_demands(grant.maximum_copy_bytes)?,d);
    let turn=cursor.close_step(grant);assert!(turn.work<=1);let step=turn.result?;
    assert!(step.progress().fits(grant));
    Ok(step)
}

fn retire_fixture<T: RetireOwned>(owner: &mut crate::retirement::controlled::ControlledRetirement<T>, items: usize, work: usize) -> Result<RetainedCloneStep, crate::ValueError> {
    let d=owner.selected_demand(work)?;let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(owner.selected_demand(grant.maximum_copy_bytes)?,d);
    let result=owner.step(grant);assert!(owner.last_turn_work()<=1);let step=result?;
    assert!(step.progress().fits(grant));
    Ok(step)
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
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        if self.released {
            return RetirementStep::Complete(0);
        }
        if self.value.as_ref().is_some_and(|value|Arc::strong_count(&value.drops)<=1){return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"fixture drop counter requires its original external observer"),0);}
        drop(self.value.take());
        self.released = true;
        RetirementStep::Advanced
    }

    fn terminal_is_empty(&self) -> bool {
        self.released
    }
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
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(DropProbeRetirement { value: ManuallyDrop::new(Some(self)), released: false })
    }
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
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if !self.closing{return Ok(Default::default());}self.close.next_owner_demands::<DropProbe>(self.output.is_some(),&self.source,body)}
    fn output_ready(&self) -> bool { !self.closing && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }

    fn advance_demands(&self,source:RetainedCloneRef<'_,DropProbe>,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand {copy_bytes:if !source.check_binding(&self.source)? {source.binding_copy_bytes()}else{size_of::<DropProbe>()},depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, DropProbe>, body:usize,grant: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        let turn=source.bind(&mut self.source,grant);work=turn.work;if let Some(progress)=turn.result? { return Ok(RetainedCloneStep::Progress(progress)); }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<DropProbe>(), retained_capacity_bytes: 0, released_bytes: 0 };
        if !progress.fits(grant) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        work=1;self.output = Some(DropProbe { drops: Arc::clone(&source.get().drops) });
        Ok(RetainedCloneStep::Complete(progress))
    })();crate::RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_clone_take_demand())}
    fn take(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(DropProbe,RetainedCloneProgress)>>{if !self.output_ready(){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}crate::retained_clone::take_retained_clone_output(&mut self.output,g)}

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{self.close.next_owner_depth_with_binding(self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(&self.source)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(&self.source,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(&self.source)}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.source.is_none() && self.close.is_empty()
    }

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if !self.closing { return Err((crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "drop probe close must begin")).into()); }
        if !self.close.is_empty() { let turn=self.close.step_granted(grant);work=turn.work;return turn.result; }
        let turn=self.close.begin_granted(&mut self.output,grant);work=turn.work;if let Some(step)=turn.result? { return Ok(step); }
        {let turn=super::super::close_retained_binding(&mut self.source,grant);work=turn.work;turn.result}
    })();crate::RetainedWorkOutcome{result,work}}
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
    fn retirement(self)->Box<dyn RetirementCursor>{crate::retirement::leaf(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(crate::retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}

struct NonconformingChildCursor {
    source: Option<RetainedCloneBinding>,
    closing: bool,
}

impl RetainedCloneCursor<NonconformingChild> for NonconformingChildCursor {
    fn output_ready(&self) -> bool { false }
    fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{assert!(self.closing,"fixture close must begin before its quote");RetainedCloneBinding::close_demand(&self.source,body)}

    fn advance_demands(&self,source:RetainedCloneRef<'_,NonconformingChild>,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand {copy_bytes:if !source.check_binding(&self.source)? {source.binding_copy_bytes()}else{size_of::<NonconformingChild>()},depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, NonconformingChild>, body:usize,grant: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        let turn=source.bind(&mut self.source,grant);work=turn.work;if let Some(progress)=turn.result? { return Ok(RetainedCloneStep::Progress(progress)); }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: grant.maximum_items.saturating_add(1), copied_bytes: grant.maximum_copy_bytes.saturating_add(1), retained_capacity_bytes: 0, released_bytes: 0 }))
    })();crate::RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
    fn take(&mut self,_:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(NonconformingChild,RetainedCloneProgress)>>{crate::RetainedWorkOutcome{result:Ok(None),work:0}}

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::depth_demand(&self.source)}
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(&self.source)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(&self.source,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(&self.source)}
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let grant=effect.grant();super::super::close_retained_binding(&mut self.source,grant)}
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
    fn retirement(self)->Box<dyn RetirementCursor>{crate::retirement::leaf(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(crate::retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}

struct NonconformingRetirementChildCursor {
    output: Option<NonconformingRetirementChild>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
    reported: bool,
}

impl RetainedCloneCursor<NonconformingRetirementChild> for NonconformingRetirementChildCursor {
    fn output_ready(&self) -> bool { !self.closing && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{assert!(self.closing,"fixture close must begin before its quote");if !self.reported{Ok(crate::RetirementDemand{depth:1,..Default::default()})}else{RetainedCloneBinding::close_demand(&self.source,body)}}

    fn advance_demands(&self,source:RetainedCloneRef<'_,NonconformingRetirementChild>,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand {copy_bytes:if !source.check_binding(&self.source)? {source.binding_copy_bytes()}else{size_of::<NonconformingRetirementChild>()},depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, NonconformingRetirementChild>, body:usize,grant: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        let turn=source.bind(&mut self.source,grant);work=turn.work;if let Some(progress)=turn.result? { return Ok(RetainedCloneStep::Progress(progress)); }
        work=1;self.output = Some(NonconformingRetirementChild);
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    })();crate::RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_clone_take_demand())}
    fn take(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(NonconformingRetirementChild,RetainedCloneProgress)>>{if !self.output_ready(){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}crate::retained_clone::take_retained_clone_output(&mut self.output,g)}

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::depth_demand(&self.source)}
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(&self.source)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(&self.source,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(&self.source)}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.reported && self.source.is_none() && self.output.is_none()
    }

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if !self.reported {
            work=1;self.reported = true;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: grant.maximum_items.saturating_add(1), copied_bytes: 0, retained_capacity_bytes: grant.maximum_capacity_bytes.saturating_add(1), released_bytes: grant.maximum_release_bytes.saturating_add(1) }));
        }
        {let turn=super::super::close_retained_binding(&mut self.source,grant);work=turn.work;turn.result}
    })();crate::RetainedWorkOutcome{result,work}}
}

impl RetainedClone for NonconformingRetirementChild {
    type Cursor = NonconformingRetirementChildCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        NonconformingRetirementChildCursor { output: None, source: None, closing: false, reported: false }
    }
}

#[derive(Debug, PartialEq, crate::RetainedClone, crate::RetireOwned)]
#[value(crate="crate")]
struct OversizedOwner {
    first: String,
    second: String,
    third: String,
}

#[derive(crate::RetireOwned)]
#[value(crate="crate")]
struct ReleaseProbe(u8);

struct ReleaseProbeCursor([u8; 128]);
static RELEASE_PROBE_DROPS: AtomicUsize = AtomicUsize::new(0);
static RELEASE_PROBE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

impl Drop for ReleaseProbeCursor {
    fn drop(&mut self) { RELEASE_PROBE_DROPS.fetch_add(1, Ordering::SeqCst); }
}

impl RetainedClone for ReleaseProbe {
    type Cursor = ReleaseProbeCursor;
    fn retained_clone_cursor() -> Self::Cursor { ReleaseProbeCursor([0;128]) }
}

impl RetainedCloneCursor<ReleaseProbe> for ReleaseProbeCursor {
    fn close_demands(&self,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
    fn output_ready(&self) -> bool { self.0[1] == 0 && self.0[0] != 0 }
    fn close_started(&self) -> bool { self.0[1] != 0 }

    fn advance_demands(&self,_source:RetainedCloneRef<'_,ReleaseProbe>,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand {copy_bytes:usize::from(!self.output_ready()),depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, ReleaseProbe>, body:usize,grant: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        work=1;self.0[0] = 1;
        self.0[2] = source.get().0;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: 1, ..Default::default() }))
    })();crate::RetainedWorkOutcome{result,work}}
    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_clone_take_demand())}
    fn take(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(ReleaseProbe,RetainedCloneProgress)>>{if !self.output_ready()||!crate::retained_clone::retained_clone_grant_admits(g,self.take_demand().unwrap()){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}self.0[0]=0;let value=ReleaseProbe(std::mem::take(&mut self.0[2]));crate::RetainedWorkOutcome{result:Ok(Some((value,RetainedCloneProgress{copied_items:1,..Default::default()}))),work:1}}
    fn begin_close(&mut self) -> bool { let started=self.0[1]==0; self.0[1]=1; started }
    
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome {let _=effect.grant();crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, crate::ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> { Ok(0) }
    fn terminal_is_empty(&self) -> bool { self.0[1]==1 && self.0[0]==0 }
}

#[test]
fn paged_native_typed_field_close_exposes_exact_scaffold_demand() {
    use crate::observe_retirement_allocations;
    let _guard=RELEASE_PROBE_LOCK.lock().unwrap();
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/📏️release-authority/🔣️.json")).unwrap();
    let source=super::super::RetainedCloneSource::from_owner(ReleaseProbe(fixture["inputByte"].as_u64().unwrap() as u8));
    let (mut cursor,allocation)=observe_retirement_allocations(super::super::RetainedFieldCursor::<ReleaseProbe>::default);
    assert_eq!(allocation,(0,0));
    let grant=RetainedCloneGrant { maximum_items:2, maximum_copy_bytes:128, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:64 };
    cursor.advance(source.borrow(), 128, grant).result.unwrap();
    let (birth,allocation)=observe_retirement_allocations(||cursor.advance(source.borrow(), 128, grant).result.unwrap());
    assert_eq!(allocation,(128,0));assert_eq!(birth.progress().retained_capacity_bytes,128);
    cursor.advance(source.borrow(), 128, grant).result.unwrap();let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=cursor.take(g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,0);assert_eq!(value.0,17);
    let (before,allocation)=observe_retirement_allocations(||(cursor.next_close_capacity_byte_demand(0).unwrap(),cursor.next_close_release_byte_demand().unwrap()));
    assert_eq!(allocation,(0,0));assert_eq!(before,(0,0));
    cursor.begin_close();
    let (metadata,allocation)=observe_retirement_allocations(||cursor.close_step(RetainedCloneGrant {maximum_items:1,maximum_depth:1,..Default::default()}).result.unwrap());
    assert_eq!(metadata.progress().copied_items,1);assert_eq!(allocation,(0,0));
    let (demand,allocation)=observe_retirement_allocations(||(cursor.next_close_capacity_byte_demand(0).unwrap(),cursor.next_close_release_byte_demand().unwrap()));
    assert_eq!(allocation,(0,0));assert_eq!(demand,(0,fixture["physicalCursorBytes"].as_u64().unwrap() as usize));
    let (below,allocation)=observe_retirement_allocations(||cursor.close_step(RetainedCloneGrant {maximum_release_bytes:127,..grant}).result.unwrap());
    assert_eq!(below.progress(),Default::default());assert_eq!(allocation,(0,0));
    let depth=cursor.next_close_depth_demand().unwrap();assert_eq!(depth,fixture["physicalScaffoldDepth"].as_u64().unwrap()as usize);
    let (denied_depth,allocation)=observe_retirement_allocations(||cursor.close_step(RetainedCloneGrant {maximum_items:1,maximum_release_bytes:128,..Default::default()}));
    assert_eq!(allocation,(0,0));assert_eq!(denied_depth.result.unwrap().progress(),Default::default());assert_eq!(cursor.next_close_release_byte_demand().unwrap(),128);
    let (exact,allocation)=observe_retirement_allocations(||cursor.close_step(RetainedCloneGrant {maximum_items:1,maximum_release_bytes:128,maximum_depth:depth,..Default::default()}).result.unwrap());
    assert_eq!(allocation,(0,128));assert_eq!(exact.progress().released_bytes,128);assert_eq!(exact.progress().copied_bytes,0);
    for _ in 0..16 { if cursor.terminal_is_empty() { break; } close_fixture(&mut cursor, 1, 4096).unwrap(); }assert!(cursor.terminal_is_empty());
    assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));
}

#[test]
fn retained_paged_list_release_authority_is_distinct_from_copy_and_admits_exact_physical_extent() {
    let _guard=RELEASE_PROBE_LOCK.lock().unwrap();
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/📏️release-authority/🔣️.json")).unwrap();
    let source=super::super::RetainedCloneSource::from_owner(ReleaseProbe(fixture["inputByte"].as_u64().unwrap() as u8));
    let mut cursor=super::super::RetainedFieldCursor::<ReleaseProbe>::default();
    let grant=RetainedCloneGrant { maximum_items:2, maximum_copy_bytes:128, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:64 };
    assert_eq!(size_of::<ReleaseProbeCursor>(),fixture["physicalCursorBytes"].as_u64().unwrap() as usize);
    RELEASE_PROBE_DROPS.store(0,Ordering::SeqCst);
    cursor.advance(source.borrow(), 128, grant).result.unwrap();
    let birth=cursor.advance(source.borrow(), 128, grant).result.unwrap().progress();
    assert_eq!(birth.retained_capacity_bytes,128);
    let copy=cursor.advance(source.borrow(), 128, grant).result.unwrap().progress();
    assert_eq!(copy.copied_bytes,1);
    let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=cursor.take(g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,0);assert_eq!(value.0,17);
    cursor.begin_close();
    assert_eq!(cursor.close_step(grant).result.unwrap().progress().copied_items,1);
    let below=RetainedCloneGrant { maximum_release_bytes:fixture["oneBelowReleaseBytes"].as_u64().unwrap() as usize, ..grant };
    assert_eq!(cursor.close_step(below).result.unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(RELEASE_PROBE_DROPS.load(Ordering::SeqCst),0);
    assert!(!cursor.terminal_is_empty());
    let released=cursor.close_step(RetainedCloneGrant { maximum_release_bytes:128, ..grant }).result.unwrap().progress();
    assert_eq!(released.released_bytes,128);
    assert_eq!(released.copied_bytes,0);
    assert_eq!(released.retained_capacity_bytes,0);
    assert_eq!(RELEASE_PROBE_DROPS.load(Ordering::SeqCst),1);
    for _ in 0..16 { if cursor.terminal_is_empty() { break; } close_fixture(&mut cursor, 1, 4096).unwrap(); }
    assert!(cursor.terminal_is_empty());
}

#[derive(Clone, Copy, Debug)]
struct InsufficientScaffoldChild;

impl RetireOwned for InsufficientScaffoldChild {
    fn retirement(self)->Box<dyn RetirementCursor>{crate::retirement::leaf(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(crate::retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}

struct InsufficientScaffoldChildCursor {
    output: Option<InsufficientScaffoldChild>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
}

impl RetainedCloneCursor<InsufficientScaffoldChild> for InsufficientScaffoldChildCursor {
    fn output_ready(&self) -> bool { !self.closing && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{assert!(self.closing,"fixture close must begin before its quote");let mut demand=if self.output.is_some(){crate::RetirementDemand{depth:1,..Default::default()}}else{RetainedCloneBinding::close_demand(&self.source,body)?};demand.release_bytes=demand.release_bytes.max(128);Ok(demand)}

    fn advance_demands(&self,source:RetainedCloneRef<'_,InsufficientScaffoldChild>,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand {copy_bytes:if !source.check_binding(&self.source)? {source.binding_copy_bytes()}else{size_of::<InsufficientScaffoldChild>()},depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, InsufficientScaffoldChild>, body:usize,grant: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        let turn=source.bind(&mut self.source,grant);work=turn.work;if let Some(progress)=turn.result? { return Ok(RetainedCloneStep::Progress(progress)); }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        work=1;self.output = Some(*source.get());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    })();crate::RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_clone_take_demand())}
    fn take(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(InsufficientScaffoldChild,RetainedCloneProgress)>>{if !self.output_ready(){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}crate::retained_clone::take_retained_clone_output(&mut self.output,g)}

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing{Ok(0)}else{RetainedCloneBinding::depth_demand(&self.source)}}
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(&self.source)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(&self.source,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(128.max(RetainedCloneBinding::release_demand(&self.source)?))}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.source.is_none()
    }

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_release_bytes < 128 {
            return Err((crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "fixture scaffold cannot progress under its declared 128-byte release grant")).into());
        }
        if self.output.is_some(){work=1;self.output.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        {let turn=super::super::close_retained_binding(&mut self.source,grant);work=turn.work;turn.result}
    })();crate::RetainedWorkOutcome{result,work}}
}

impl RetainedClone for InsufficientScaffoldChild {
    type Cursor = InsufficientScaffoldChildCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        InsufficientScaffoldChildCursor { output: None, source: None, closing: false }
    }
}

#[derive(crate::RetainedClone, crate::RetireOwned)]
#[value(crate="crate")]
struct DerivedScaffoldOwner {
    child: InsufficientScaffoldChild,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    maximum_entries: usize,
    entry_count: usize,
    value_prefix: String,
    grant: GrantFixture,
    cancellation_after_entries: usize,
    expected: CloseExpectation,
}
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct CloseExpectation{spent_close_turns:usize,spent_close_cpu:u64}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GrantFixture {
    maximum_items: usize,
    maximum_copy_bytes: usize,
    maximum_capacity_bytes: usize,
    maximum_release_bytes: usize,
    maximum_depth: usize,
}

fn fixture() -> Fixture {
    serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/📦️copy/🔣️.json")).expect("paged list retained fixture")
}

fn source(fixture: &Fixture) -> PagedList<String, 1024> {
    assert_eq!(fixture.maximum_entries, 1024);
    let mut source = PagedList::default();
    for ordinal in 0..fixture.entry_count {
        while !source.has_reserved_slot() {
            let required = source.next_allocation_bytes().expect("fixture capacity");
            assert!(source.reserve_one(required).expect("fixture reserve").progressed);
        }
        source.push_reserved(format!("{}{}", fixture.value_prefix, ordinal)).expect("fixture placement");
    }
    source
}

fn grant(fixture: &Fixture) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: fixture.grant.maximum_items, maximum_copy_bytes: fixture.grant.maximum_copy_bytes, maximum_capacity_bytes: fixture.grant.maximum_capacity_bytes, maximum_depth: fixture.grant.maximum_depth, maximum_release_bytes: fixture.grant.maximum_release_bytes }
}

#[test]
fn retained_paged_list_copy_matches_vec_serde_and_closes_page_by_page() {
    let fixture = fixture();
    let source = source(&fixture);
    let expected: Vec<_> = source.iter().cloned().collect();
    let oracle = serde_json::to_vec(&expected).expect("serde oracle");
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<String, 1024>::retained_clone_cursor();
    let grant = grant(&fixture);
    let mut turns = 0usize;
    let copied = loop {
        turns += 1;
        let demand=cursor.advance_demands(retained.borrow(),grant.maximum_copy_bytes).unwrap();
        let grant=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),..grant};
        let step = cursor.advance(retained.borrow(), fixture.grant.maximum_copy_bytes, grant).result.expect("paged retained copy");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=cursor.take(g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,0);break value;
        }
        assert!(turns < 100_000, "paged retained copy terminates");
    };
    assert!(turns > 1);
    assert_eq!(serde_json::to_vec(&copied.iter().cloned().collect::<Vec<_>>()).expect("copied serde"), oracle);
    assert!(cursor.begin_close());
    let mut close_turns = 0usize;
    let mut close_cpu=0;let(mut close_born,mut close_freed)=(0,0);
    let mut bounded_close_turns=0;
    while !cursor.terminal_is_empty() {
        bounded_close_turns+=1;assert!(bounded_close_turns<100000,"retained fixture closure must terminate");
        close_turns += 1;
        let d=cursor.close_demands(fixture.grant.maximum_capacity_bytes).unwrap();
        let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let(turn,heap)=crate::observe_retirement_allocations(||cursor.close_step(g));let work=turn.work;assert_eq!(work,1);close_cpu+=work;let step=turn.result.expect("spent cursor close");
        assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));close_born+=heap.0;close_freed+=heap.1;
        if let RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress {copied_items:released_items,released_bytes,..}) = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= fixture.grant.maximum_capacity_bytes);
        }
        assert!(close_turns < 100_000, "spent cursor close terminates");
    }
    assert_eq!(close_turns,fixture.expected.spent_close_turns,"a spent cursor funds every original alias, native frame, frontier page and final binding header phase");assert_eq!(close_cpu,fixture.expected.spent_close_cpu);assert_eq!(close_born,close_freed);
    let mut retirement = crate::retirement::controlled::ControlledRetirement::new(copied).map_err(|(error,_)|error).unwrap();
    let mut retirement_turns = 0usize;
    while !retirement.terminal_is_empty() {
        retirement_turns += 1;
        match retire_fixture(&mut retirement, 1, fixture.grant.maximum_capacity_bytes).expect("production paged retirement scheduler") {
            RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:released_items,released_bytes,..}) => {
                assert!(released_items <= 1);
                assert!(released_bytes <= fixture.grant.maximum_capacity_bytes);
            }
            RetainedCloneStep::Complete(progress) => {assert_eq!(progress,Default::default());assert!(retirement.terminal_is_empty());},
        }
        assert!(retirement_turns < 100_000, "paged retirement terminates");
    }
    assert!(retirement_turns > 1);
}

#[test]
fn retained_paged_list_funds_completed_owner_and_cursor_movement_before_adoption() {
    let expected = OversizedOwner { first: "alpha".into(), second: "beta".into(), third: "gamma".into() };
    assert!(size_of::<OversizedOwner>() > 64);
    let mut values = PagedList::<OversizedOwner, 1>::default();
    while !values.has_reserved_slot() {
        let required = values.next_allocation_bytes().expect("oversized owner capacity");
        assert!(values.reserve_one(required).expect("oversized owner reserve").progressed);
    }
    values.push_reserved(expected).expect("oversized owner source placement");
    let source = super::super::RetainedCloneSource::from_owner(values);
    let mut cursor = PagedList::<OversizedOwner, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    let copied = loop {
        let demand=cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes).unwrap();
        let funded=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.max(demand.capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.max(demand.release_bytes),maximum_depth:grant.maximum_depth.max(demand.depth),..grant};
        if demand.copy_bytes>0 {
            let denied=RetainedCloneGrant {maximum_copy_bytes:demand.copy_bytes-1,..funded};
            let quote=cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes).unwrap();
            assert_eq!(cursor.advance(source.borrow(), 64, denied).result.unwrap().progress(),Default::default());
            assert_eq!(cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes).unwrap(),quote);
        }
        match cursor.advance(source.borrow(), 64, funded).result.expect("oversized owner retained copy") {
            RetainedCloneStep::Progress(progress) => assert_ne!(progress, RetainedCloneProgress::default()),
            RetainedCloneStep::Complete(_) => {let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=cursor.take(g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,0);break value},
        }
    };
    assert_eq!(copied.get(0), Some(&OversizedOwner { first: "alpha".into(), second: "beta".into(), third: "gamma".into() }));
    assert!(cursor.begin_close());
    let mut bounded_close_turns=0;
    while !cursor.terminal_is_empty() {
        bounded_close_turns+=1;assert!(bounded_close_turns<100000,"retained fixture closure must terminate");
        close_fixture(&mut cursor, 8, 4096).expect("oversized owner cursor close");
    }
    let mut retirement = crate::retirement::controlled::ControlledRetirement::new(copied).map_err(|(error,_)|error).unwrap();
    while !retirement.terminal_is_empty() {
        retire_fixture(&mut retirement, 8, 4096).expect("oversized owner retirement");
    }
    drop(source);
}

#[test]
fn retained_paged_list_refuses_scaffold_grant_that_cannot_release_owner() {
    let mut values = PagedList::<InsufficientScaffoldChild, 1>::default();
    while !values.has_reserved_slot() {
        let required = values.next_allocation_bytes().expect("insufficient scaffold capacity");
        assert!(values.reserve_one(required).expect("insufficient scaffold reserve").progressed);
    }
    values.push_reserved(InsufficientScaffoldChild).expect("insufficient scaffold source placement");
    let source = super::super::RetainedCloneSource::from_owner(values);
    let mut cursor = PagedList::<InsufficientScaffoldChild, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 64 };
    let mut error_turns=0;
    let withheld = loop {
        error_turns+=1;assert!(error_turns<100000,"adversarial retained fixture must terminate");
        let demand=cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes).unwrap();
        let grant=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),..grant};
        let turn=cursor.advance(source.borrow(), 64, grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(progress)) if progress == Default::default() => break cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes).unwrap(),
            Ok(RetainedCloneStep::Progress(_)) => {},
            Ok(RetainedCloneStep::Complete(_)) => panic!("insufficient scaffold release must not complete"),
            Err(error) => panic!("{error:?}"),
        }
    };
    assert_eq!(withheld.release_bytes,128);
    assert_eq!(cursor.advance(source.borrow(), 64, grant).result.unwrap().progress(),Default::default());
    assert!(cursor.begin_close());
    let mut bounded_close_turns=0;
    while !cursor.terminal_is_empty() {
        bounded_close_turns+=1;assert!(bounded_close_turns<100000,"retained fixture closure must terminate");
        let step=close_fixture(&mut cursor, 8, 4096).expect("insufficient scaffold cursor close");
    }
    drop(source);
}

#[test]
fn retained_derive_refuses_scaffold_grant_that_cannot_release_field() {
    let source = super::super::RetainedCloneSource::from_owner(DerivedScaffoldOwner { child: InsufficientScaffoldChild });
    let mut cursor = DerivedScaffoldOwner::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 64 };
    let mut error_turns=0;
    let withheld = loop {
        error_turns+=1;assert!(error_turns<100000,"adversarial retained fixture must terminate");
        let demand=cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes).unwrap();
        let grant=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),..grant};
        let turn=cursor.advance(source.borrow(), 64, grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(progress)) if progress == Default::default() => break cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes).unwrap(),
            Ok(RetainedCloneStep::Progress(_)) => {},
            Ok(RetainedCloneStep::Complete(_)) => panic!("derived insufficient scaffold release must not complete"),
            Err(error) => panic!("{error:?}"),
        }
    };
    assert_eq!(withheld.release_bytes,128);
    assert_eq!(cursor.advance(source.borrow(), 64, grant).result.unwrap().progress(),Default::default());
    assert!(cursor.begin_close());
    let mut bounded_close_turns=0;
    while !cursor.terminal_is_empty() {
        bounded_close_turns+=1;assert!(bounded_close_turns<100000,"retained fixture closure must terminate");
        let step=close_fixture(&mut cursor, 8, 4096).expect("derived insufficient scaffold cursor close");
    }
    drop(source);
}

#[test]
fn retained_paged_list_cancellation_preserves_source_and_retires_partial_pages() {
    let fixture = fixture();
    let source = source(&fixture);
    let expected: Vec<_> = source.iter().cloned().collect();
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<String, 1024>::retained_clone_cursor();
    let grant = grant(&fixture);
    let mut turns = 0usize;
    while turns < fixture.cancellation_after_entries {
        turns += 1;
        let demand=cursor.advance_demands(retained.borrow(),grant.maximum_copy_bytes).unwrap();
        let grant=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),..grant};
        if matches!(cursor.advance(retained.borrow(), fixture.grant.maximum_copy_bytes, grant).result.expect("paged retained prefix"), RetainedCloneStep::Complete(_)) {
            panic!("fixture must cancel a partial paged copy");
        }
    }
    assert!(cursor.begin_close());
    drop(retained);
    let mut close_turns = 0usize;
    let mut bounded_close_turns=0;
    while !cursor.terminal_is_empty() {
        bounded_close_turns+=1;assert!(bounded_close_turns<100000,"retained fixture closure must terminate");
        close_turns += 1;
        let step = close_fixture(&mut cursor, 1, fixture.grant.maximum_capacity_bytes).expect("partial paged close");
        if let RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress {copied_items:released_items,released_bytes,..}) = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= fixture.grant.maximum_capacity_bytes);
        }
        assert!(close_turns < 100_000, "partial paged close terminates");
    }
    assert!(close_turns > 1);
    assert_eq!(expected.len(), fixture.entry_count);
}

#[test]
fn retained_paged_list_refuses_over_budget_child_before_owner_placement() {
    let mut source = PagedList::<NonconformingChild, 1>::default();
    while !source.has_reserved_slot() {
        let required = source.next_allocation_bytes().expect("nonconforming fixture capacity");
        assert!(source.reserve_one(required).expect("nonconforming fixture reserve").progressed);
    }
    assert!(source.push_reserved(NonconformingChild).is_ok());
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<NonconformingChild, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RetainedCloneBinding::alias_copy_bytes().max(size_of::<NonconformingChildCursor>()), maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    let mut error_turns=0;
    let error = loop {
        error_turns+=1;assert!(error_turns<100000,"adversarial retained fixture must terminate");
        let turn=cursor.advance(retained.borrow(), RetainedCloneBinding::alias_copy_bytes().max(size_of::<NonconformingChildCursor>()), grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(_)) => {}
            Ok(RetainedCloneStep::Complete(_)) => panic!("over-budget child must not complete"),
            Err(error) => break error,
        }
    };
    assert_eq!(error.kind,crate::ValueRefusalKind::InvariantViolated);
    assert_eq!(error.message.as_ref(),"original retained turn exceeded its item, copy, capacity, or release grant");
    assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));
    assert_eq!(cursor.index, 0);
    assert_eq!(cursor.values.len(), 0);
    assert!(cursor.child_value.is_none());
    assert!(cursor.begin_close());
    let mut bounded_close_turns=0;
    while !cursor.terminal_is_empty() {
        bounded_close_turns+=1;assert!(bounded_close_turns<100000,"retained fixture closure must terminate");
        let step = close_fixture(&mut cursor, 1, 4096).expect("nonconforming child close");
        if let RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress {copied_items:released_items,released_bytes,..}) = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= 4096);
        }
    }
}

#[test]
fn retained_paged_list_refuses_over_budget_child_retirement_before_owner_placement() {
    let mut source = PagedList::<NonconformingRetirementChild, 1>::default();
    while !source.has_reserved_slot() {
        let required = source.next_allocation_bytes().expect("nonconforming retirement fixture capacity");
        assert!(source.reserve_one(required).expect("nonconforming retirement fixture reserve").progressed);
    }
    assert!(source.push_reserved(NonconformingRetirementChild).is_ok());
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<NonconformingRetirementChild, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RetainedCloneBinding::alias_copy_bytes().max(size_of::<NonconformingRetirementChildCursor>()), maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    let mut error_turns=0;
    loop {
        error_turns+=1;assert!(error_turns<100000,"adversarial retained fixture must terminate");
        let turn=cursor.advance(retained.borrow(), RetainedCloneBinding::alias_copy_bytes().max(size_of::<NonconformingRetirementChildCursor>()), grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(_)) => {}
            Ok(RetainedCloneStep::Complete(_)) => panic!("over-budget child retirement must be refused before completion"),
            Err(error) => {
                assert_eq!(error.kind,crate::ValueRefusalKind::InvariantViolated);
                assert_eq!(error.message.as_ref(),"original retained turn exceeded its item, copy, capacity, or release grant");
                assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));
                break;
            }
        }
    }
    assert_eq!(cursor.index, 0);
    assert_eq!(cursor.values.len(), 0);
    assert!(cursor.child_value.is_some());
    assert!(cursor.begin_close());
    let mut bounded_close_turns=0;
    while !cursor.terminal_is_empty() {
        bounded_close_turns+=1;assert!(bounded_close_turns<100000,"retained fixture closure must terminate");
        let step = close_fixture(&mut cursor, 1, 4096).expect("nonconforming retirement child closes after its refused report");
        if let RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress {copied_items:released_items,released_bytes,..}) = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= 4096);
        }
    }
}

#[test]
fn retained_paged_list_abandonment_panics_without_running_payload_destructors() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut source = PagedList::<DropProbe, 64>::default();
    for _ in 0..64 {
        while !source.has_reserved_slot() {
            let required = source.next_allocation_bytes().expect("drop probe fixture capacity");
            assert!(source.reserve_one(required).expect("drop probe fixture reserve").progressed);
        }
        assert!(source.push_reserved(DropProbe { drops: Arc::clone(&drops) }).is_ok());
    }
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<DropProbe, 64>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 5, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    while cursor.values.len() < 8 {
        assert!(matches!(cursor.advance(retained.borrow(), 4096, grant).result.expect("drop probe prefix"), RetainedCloneStep::Progress(_)));
    }
    let abandoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(cursor)));
    assert!(abandoned.is_err(), "active cursor abandonment fails fast");
    assert_eq!(drops.load(Ordering::SeqCst), 0, "abandonment retains every partial page and payload owner");
    drop(retained);
}
