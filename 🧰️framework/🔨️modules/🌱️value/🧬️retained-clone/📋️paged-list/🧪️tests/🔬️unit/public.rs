const MAXIMUM_CLONE_WORK:u64=4096;
pub use semio_framework_value_retained_clone::*;
pub use semio_framework_value_derive::{RetainedClone,RetireOwned};
use semio_framework_value_retained_clone::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value_paged_list::PagedList;
use semio_framework_value_testing_allocator::observe_retirement_allocations;
include!("common.rs");
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
    fn advance_work_demand(&self,_source:RetainedCloneRef<'_,ReleaseProbe>,_body:usize,maximum_work_units:u64)->Result<u64,crate::ValueError>{Ok(u64::from(maximum_work_units!=0&&!self.output_ready()))}
    fn take_work_demand(&self)->Result<u64,crate::ValueError>{Ok(u64::from(self.output_ready()))}
    fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(u64::from(!self.close_started()))}
    fn begin_close_paid(&mut self)->crate::RetainedWorkOutcome<(),crate::ValueError>{let work=u64::from(self.begin_close());crate::RetainedWorkOutcome{result:Ok(()),work}}
    fn close_work_demand(&self,_body:usize)->Result<u64,crate::ValueError>{Ok(u64::from(!self.terminal_is_empty()))}

    fn close_demands(&self,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
    fn output_ready(&self) -> bool { self.0[1] == 0 && self.0[0] != 0 }
    fn close_started(&self) -> bool { self.0[1] != 0 }

    fn advance_demands(&self,_source:RetainedCloneRef<'_,ReleaseProbe>,_body:usize,maximum_work_units:u64)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand {copy_bytes:usize::from(!self.output_ready()),depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, ReleaseProbe>, body:usize,maximum_work_units:u64,grant: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        work=1;self.0[0] = 1;
        self.0[2] = source.get().0;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: 1, ..Default::default() }))
    })();crate::RetainedWorkOutcome{result,work}}
    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_clone_take_demand())}
    fn take(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(ReleaseProbe,RetainedCloneProgress)>>{if !self.output_ready()||!crate::retained_clone::retained_clone_grant_admits(g,self.take_demand().unwrap()){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}self.0[0]=0;let value=ReleaseProbe(std::mem::take(&mut self.0[2]));crate::RetainedWorkOutcome{result:Ok(Some((value,RetainedCloneProgress{copied_items:1,..Default::default()}))),work:1}}
    fn begin_close(&mut self) -> bool { let started=self.0[1]==0; self.0[1]=1; started }
    
    fn close_step(&mut self, _: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, crate::ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> { Ok(0) }
    fn terminal_is_empty(&self) -> bool { self.0[1]==1 && self.0[0]==0 }
}

#[test]
fn paged_native_typed_field_close_exposes_exact_scaffold_demand() {
    use crate::observe_retirement_allocations;
    let _guard=RELEASE_PROBE_LOCK.lock().unwrap();
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/📏️release-authority/🔣️.json")).unwrap();
    let source=crate::RetainedCloneSource::from_owner(ReleaseProbe(fixture["inputByte"].as_u64().unwrap() as u8));
    let (mut cursor,allocation)=observe_retirement_allocations(crate::RetainedFieldCursor::<ReleaseProbe>::default);
    assert_eq!(allocation,(0,0));
    let grant=RetainedCloneGrant { maximum_items:2, maximum_copy_bytes:128, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:64 };
    cursor.advance(source.borrow(), 128,MAXIMUM_CLONE_WORK, grant).result.unwrap();
    let (birth,allocation)=observe_retirement_allocations(||cursor.advance(source.borrow(), 128,MAXIMUM_CLONE_WORK, grant).result.unwrap());
    assert_eq!(allocation,(128,0));assert_eq!(birth.progress().retained_capacity_bytes,128);
    cursor.advance(source.borrow(), 128,MAXIMUM_CLONE_WORK, grant).result.unwrap();let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=cursor.take(g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,0);assert_eq!(value.0,17);
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
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/📏️release-authority/🔣️.json")).unwrap();
    let source=crate::RetainedCloneSource::from_owner(ReleaseProbe(fixture["inputByte"].as_u64().unwrap() as u8));
    let mut cursor=crate::RetainedFieldCursor::<ReleaseProbe>::default();
    let grant=RetainedCloneGrant { maximum_items:2, maximum_copy_bytes:128, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:64 };
    assert_eq!(size_of::<ReleaseProbeCursor>(),fixture["physicalCursorBytes"].as_u64().unwrap() as usize);
    RELEASE_PROBE_DROPS.store(0,Ordering::SeqCst);
    cursor.advance(source.borrow(), 128,MAXIMUM_CLONE_WORK, grant).result.unwrap();
    let birth=cursor.advance(source.borrow(), 128,MAXIMUM_CLONE_WORK, grant).result.unwrap().progress();
    assert_eq!(birth.retained_capacity_bytes,128);
    let copy=cursor.advance(source.borrow(), 128,MAXIMUM_CLONE_WORK, grant).result.unwrap().progress();
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
    fn advance_work_demand(&self,_source:RetainedCloneRef<'_,InsufficientScaffoldChild>,_body:usize,maximum_work_units:u64)->Result<u64,crate::ValueError>{Ok(u64::from(maximum_work_units!=0&&!self.output_ready()))}
    fn take_work_demand(&self)->Result<u64,crate::ValueError>{Ok(u64::from(self.output_ready()))}
    fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(u64::from(!self.close_started()))}
    fn begin_close_paid(&mut self)->crate::RetainedWorkOutcome<(),crate::ValueError>{let work=u64::from(self.begin_close());crate::RetainedWorkOutcome{result:Ok(()),work}}
    fn close_work_demand(&self,_body:usize)->Result<u64,crate::ValueError>{Ok(u64::from(!self.terminal_is_empty()))}

    fn output_ready(&self) -> bool { !self.closing && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{assert!(self.closing,"fixture close must begin before its quote");let mut demand=if self.output.is_some(){crate::RetirementDemand{depth:1,..Default::default()}}else{RetainedCloneBinding::close_demand(&self.source,body)?};demand.release_bytes=demand.release_bytes.max(128);Ok(demand)}

    fn advance_demands(&self,source:RetainedCloneRef<'_,InsufficientScaffoldChild>,_body:usize,maximum_work_units:u64)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand {copy_bytes:if !source.check_binding(&self.source)? {source.binding_copy_bytes()}else{size_of::<InsufficientScaffoldChild>()},depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, InsufficientScaffoldChild>, body:usize,maximum_work_units:u64,grant: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
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

    fn close_step(&mut self, grant: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_release_bytes < 128 {
            return Err((crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "fixture scaffold cannot progress under its declared 128-byte release grant")).into());
        }
        if self.output.is_some(){work=1;self.output.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        {let turn=crate::close_retained_binding(&mut self.source,grant);work=turn.work;turn.result}
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

#[test]
fn retained_paged_list_copy_matches_vec_serde_and_closes_page_by_page() {
    let fixture = fixture();
    let source = source(&fixture);
    let expected: Vec<_> = source.iter().cloned().collect();
    let oracle = serde_json::to_vec(&expected).expect("serde oracle");
    let retained = crate::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<String, 1024>::retained_clone_cursor();
    let grant = grant(&fixture);
    let mut turns = 0usize;
    let copied = loop {
        turns += 1;
        let demand=cursor.advance_demands(retained.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap();
        let grant=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),..grant};
        let step = cursor.advance(retained.borrow(), fixture.grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK, grant).result.expect("paged retained copy");
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
    let source = crate::RetainedCloneSource::from_owner(values);
    let mut cursor = PagedList::<OversizedOwner, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    let copied = loop {
        let demand=cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap();
        let funded=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.max(demand.capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.max(demand.release_bytes),maximum_depth:grant.maximum_depth.max(demand.depth),..grant};
        if demand.copy_bytes>0 {
            let denied=RetainedCloneGrant {maximum_copy_bytes:demand.copy_bytes-1,..funded};
            let quote=cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap();
            assert_eq!(cursor.advance(source.borrow(), 64,MAXIMUM_CLONE_WORK, denied).result.unwrap().progress(),Default::default());
            assert_eq!(cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap(),quote);
        }
        match cursor.advance(source.borrow(), 64,MAXIMUM_CLONE_WORK, funded).result.expect("oversized owner retained copy") {
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
    let source = crate::RetainedCloneSource::from_owner(values);
    let mut cursor = PagedList::<InsufficientScaffoldChild, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 64 };
    let mut error_turns=0;
    let withheld = loop {
        error_turns+=1;assert!(error_turns<100000,"adversarial retained fixture must terminate");
        let demand=cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap();
        let grant=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),..grant};
        let turn=cursor.advance(source.borrow(), 64,MAXIMUM_CLONE_WORK, grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(progress)) if progress == Default::default() => break cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap(),
            Ok(RetainedCloneStep::Progress(_)) => {},
            Ok(RetainedCloneStep::Complete(_)) => panic!("insufficient scaffold release must not complete"),
            Err(error) => panic!("{error:?}"),
        }
    };
    assert_eq!(withheld.release_bytes,128);
    assert_eq!(cursor.advance(source.borrow(), 64,MAXIMUM_CLONE_WORK, grant).result.unwrap().progress(),Default::default());
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
    let source = crate::RetainedCloneSource::from_owner(DerivedScaffoldOwner { child: InsufficientScaffoldChild });
    let mut cursor = DerivedScaffoldOwner::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 64 };
    let mut error_turns=0;
    let withheld = loop {
        error_turns+=1;assert!(error_turns<100000,"adversarial retained fixture must terminate");
        let demand=cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap();
        let grant=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),..grant};
        let turn=cursor.advance(source.borrow(), 64,MAXIMUM_CLONE_WORK, grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(progress)) if progress == Default::default() => break cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap(),
            Ok(RetainedCloneStep::Progress(_)) => {},
            Ok(RetainedCloneStep::Complete(_)) => panic!("derived insufficient scaffold release must not complete"),
            Err(error) => panic!("{error:?}"),
        }
    };
    assert_eq!(withheld.release_bytes,128);
    assert_eq!(cursor.advance(source.borrow(), 64,MAXIMUM_CLONE_WORK, grant).result.unwrap().progress(),Default::default());
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
    let retained = crate::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<String, 1024>::retained_clone_cursor();
    let grant = grant(&fixture);
    let mut turns = 0usize;
    while turns < fixture.cancellation_after_entries {
        turns += 1;
        let demand=cursor.advance_demands(retained.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap();
        let grant=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes.max(demand.copy_bytes),..grant};
        if matches!(cursor.advance(retained.borrow(), fixture.grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK, grant).result.expect("paged retained prefix"), RetainedCloneStep::Complete(_)) {
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

