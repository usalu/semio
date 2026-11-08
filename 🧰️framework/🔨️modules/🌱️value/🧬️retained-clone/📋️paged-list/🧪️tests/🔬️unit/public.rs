const MAXIMUM_CLONE_WORK:u64=4096;
pub use semio_framework_value_retained_clone::*;
pub use semio_framework_value_derive::{RetainedClone,RetireOwned};
use semio_framework_value_retained_clone::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value_paged_list::PagedList;
use semio_framework_value_testing_allocator::observe_retirement_allocations;
#[path="../../../🧬️contract/🧪️tests/⏱️effect/🦀️.rs"]
mod effect;
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
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_close_start_demand(self.close_started()))}
    fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneProgress,crate::ValueError>{let mut started=self.0[1]!=0;let turn=crate::start_retained_close(&mut started,effect);self.0[1]=u8::from(started);turn}
    fn close_work_demand(&self,_body:usize)->Result<u64,crate::ValueError>{Ok(u64::from(!self.terminal_is_empty()))}

    fn close_demands(&self,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
    fn output_ready(&self) -> bool { self.0[1] == 0 && self.0[0] != 0 }
    fn close_started(&self) -> bool { self.0[1] != 0 }

    fn advance_demands(&self,_source:RetainedCloneRef<'_,ReleaseProbe>,_body:usize,maximum_work_units:u64)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand {copy_bytes:usize::from(!self.output_ready()),depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, ReleaseProbe>, body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>) -> crate::OriginalRetirementOutcome {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !effect.performed(1)?{return Err(crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"release probe advance exceeded its original work").into())}work=1;self.0[0] = 1;
        self.0[2] = source.get().0;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: 1, ..Default::default() }))
    })();crate::RetainedWorkOutcome{result,work}}
    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{crate::retained_clone::retained_clone_take_demand::<ReleaseProbe>()}
    fn take<'reply>(&mut self,output:&mut Option<ReleaseProbe>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,crate::ValueError>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::ValueError>,crate::ValueError>{if !self.output_ready()||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),self.take_demand()?){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"release probe take exceeded its original work"))}self.0[0]=0;*output=Some(ReleaseProbe(std::mem::take(&mut self.0[2])));Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:self.take_demand()?.copy_bytes,..Default::default()}))))}
    
    fn close_step(&mut self, _: &mut crate::RetainedAllocationEffect<'_>) -> crate::OriginalRetirementPoll {crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
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
    let grant=RetainedCloneGrant { maximum_items:2, maximum_copy_bytes:128.max(source.borrow().binding_copy_bytes()), maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:64 };
    let mut born=None;
    for _ in 0..8 {
        let (turn,allocation)=observe_retirement_allocations(||effect::advance(&mut cursor,source.borrow(),128,MAXIMUM_CLONE_WORK,grant).result.unwrap());
                if allocation.0!=0 { born=Some((turn,allocation)); break; }
    }
    let (birth,allocation)=born.expect("the probe cursor birth allocates");
    assert_eq!(allocation,(128,0));assert_eq!(birth.progress().retained_capacity_bytes,128);
    while !cursor.output_ready() { effect::advance(&mut cursor,source.borrow(),128,MAXIMUM_CLONE_WORK,grant).result.unwrap(); }
    let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=effect::take(&mut cursor,g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,d.copy_bytes);assert_eq!(value.0,17);
    let (before,allocation)=observe_retirement_allocations(||(cursor.next_close_capacity_byte_demand(0).unwrap(),cursor.next_close_release_byte_demand().unwrap()));
    assert_eq!(allocation,(0,0));assert_eq!(before,(0,0));
    effect::begin_close(&mut cursor);
    let started=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:cursor.next_close_copy_byte_demand().unwrap(),maximum_depth:1,..Default::default()};
    let (metadata,allocation)=observe_retirement_allocations(||effect::close_step(&mut cursor,started).result.unwrap());
    assert_eq!(metadata.progress().copied_items,1);assert_eq!(allocation,(0,0));
    for _ in 0..8 {
        if cursor.next_close_release_byte_demand().unwrap()!=0 { break; }
        let step=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:cursor.next_close_copy_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand().unwrap().max(1),..Default::default()};
        let (_,allocation)=observe_retirement_allocations(||effect::close_step(&mut cursor,step).result.unwrap());
        assert_eq!(allocation,(0,0));
    }
    let (demand,allocation)=observe_retirement_allocations(||(cursor.next_close_capacity_byte_demand(0).unwrap(),cursor.next_close_release_byte_demand().unwrap()));
    assert_eq!(allocation,(0,0));assert_eq!(demand,(0,fixture["physicalCursorBytes"].as_u64().unwrap() as usize));
    let (below,allocation)=observe_retirement_allocations(||effect::close_step(&mut cursor,RetainedCloneGrant {maximum_release_bytes:127,..grant}).result.unwrap());
    assert_eq!(below.progress(),Default::default());assert_eq!(allocation,(0,0));
    let depth=cursor.next_close_depth_demand().unwrap();assert_eq!(depth,fixture["physicalScaffoldDepth"].as_u64().unwrap()as usize);
    let (denied_depth,allocation)=observe_retirement_allocations(||effect::close_step(&mut cursor,RetainedCloneGrant {maximum_items:1,maximum_release_bytes:128,..Default::default()}));
    assert_eq!(allocation,(0,0));assert_eq!(denied_depth.result.unwrap().progress(),Default::default());assert_eq!(cursor.next_close_release_byte_demand().unwrap(),128);
    let (exact,allocation)=observe_retirement_allocations(||effect::close_step(&mut cursor,RetainedCloneGrant {maximum_items:1,maximum_release_bytes:128,maximum_depth:depth,..Default::default()}).result.unwrap());
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
    let grant=RetainedCloneGrant { maximum_items:2, maximum_copy_bytes:128.max(source.borrow().binding_copy_bytes()), maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:64 };
    assert_eq!(size_of::<ReleaseProbeCursor>(),fixture["physicalCursorBytes"].as_u64().unwrap() as usize);
    RELEASE_PROBE_DROPS.store(0,Ordering::SeqCst);
    let mut birth=RetainedCloneProgress::default();
    for _ in 0..8 { birth=effect::advance(&mut cursor,source.borrow(),128,MAXIMUM_CLONE_WORK,grant).result.unwrap().progress(); if birth.retained_capacity_bytes!=0 { break; } }
    assert_eq!(birth.retained_capacity_bytes,128);
    let mut copy=RetainedCloneProgress::default();
    for _ in 0..8 { copy=effect::advance(&mut cursor,source.borrow(),128,MAXIMUM_CLONE_WORK,grant).result.unwrap().progress(); if copy.copied_bytes!=0 { break; } }
    assert_eq!(copy.copied_bytes,1);
    let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=effect::take(&mut cursor,g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,d.copy_bytes);assert_eq!(value.0,17);
    effect::begin_close(&mut cursor);
    assert_eq!(effect::close_step(&mut cursor,grant).result.unwrap().progress().copied_items,1);
    for _ in 0..8 { if cursor.next_close_release_byte_demand().unwrap()!=0 { break; } effect::close_step(&mut cursor,grant).result.unwrap(); }
    let below=RetainedCloneGrant { maximum_release_bytes:fixture["oneBelowReleaseBytes"].as_u64().unwrap() as usize, ..grant };
    assert_eq!(effect::close_step(&mut cursor,below).result.unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(RELEASE_PROBE_DROPS.load(Ordering::SeqCst),0);
    assert!(!cursor.terminal_is_empty());
    let released=effect::close_step(&mut cursor,RetainedCloneGrant { maximum_release_bytes:128, ..grant }).result.unwrap().progress();
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
    type RetirementCursor=crate::retirement::Leaf<Self>;
    fn retirement_seed(self)->Self::RetirementCursor{crate::retirement::leaf(self)}
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
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_close_start_demand(self.closing))}
    fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneProgress,crate::ValueError>{crate::start_retained_close(&mut self.closing,effect)}
    fn close_work_demand(&self,_body:usize)->Result<u64,crate::ValueError>{Ok(u64::from(!self.terminal_is_empty()))}

    fn output_ready(&self) -> bool { !self.closing && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{assert!(self.closing,"fixture close must begin before its quote");let mut demand=if self.output.is_some(){crate::RetirementDemand{depth:1,..Default::default()}}else{RetainedCloneBinding::close_demand(&self.source,body)?};demand.release_bytes=demand.release_bytes.max(128);Ok(demand)}

    fn advance_demands(&self,source:RetainedCloneRef<'_,InsufficientScaffoldChild>,_body:usize,maximum_work_units:u64)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand {copy_bytes:if !source.check_binding(&self.source)? {source.binding_copy_bytes()}else{size_of::<InsufficientScaffoldChild>()},depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, InsufficientScaffoldChild>, body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>) -> crate::OriginalRetirementOutcome {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let turn=source.bind(&mut self.source,effect);work=turn.work;if let Some(progress)=turn.result? { return Ok(RetainedCloneStep::Progress(progress)); }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if !effect.performed(1)?{return Err(crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"scaffold child advance exceeded its original work").into())}work=1;self.output = Some(*source.get());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    })();crate::RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{crate::retained_clone::retained_clone_take_demand::<InsufficientScaffoldChild>()}
    fn take<'reply>(&mut self,output:&mut Option<InsufficientScaffoldChild>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,crate::ValueError>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::ValueError>,crate::ValueError>{if !self.output_ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}let turn=crate::retained_clone::take_retained_clone_output(&mut self.output,output,effect);Ok(reply.complete(turn.result.map(|progress|if output.is_some(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})))}

    

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing{Ok(0)}else{RetainedCloneBinding::depth_demand(&self.source)}}
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(&self.source)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(&self.source,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(128.max(RetainedCloneBinding::release_demand(&self.source)?))}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.source.is_none()
    }

    fn close_step(&mut self, effect: &mut crate::RetainedAllocationEffect<'_>) -> crate::OriginalRetirementPoll {let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::RetirementPollExit>{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_release_bytes < 128 {
            return Err((crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "fixture scaffold cannot progress under its declared 128-byte release grant")).into());
        }
        if self.output.is_some(){work=1;self.output.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        {let turn=crate::RetirementPollExit::settled(crate::close_retained_binding(&mut self.source,effect))?;work=turn.work;turn.result.map_err(Into::into)}
    })();crate::RetirementPollExit::finish(result,work)}
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
        let step = effect::advance(&mut cursor,retained.borrow(),fixture.grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK,grant).result.expect("paged retained copy");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=effect::take(&mut cursor,g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,d.copy_bytes);break value;
        }
        assert!(turns < 100_000, "paged retained copy terminates");
    };
    assert!(turns > 1);
    assert_eq!(serde_json::to_vec(&copied.iter().cloned().collect::<Vec<_>>()).expect("copied serde"), oracle);
    assert_eq!(effect::begin_close(&mut cursor),1);
    let mut close_turns = 0usize;
    let mut close_cpu=0;let(mut close_born,mut close_freed)=(0,0);
    let mut bounded_close_turns=0;
    while !cursor.terminal_is_empty() {
        bounded_close_turns+=1;assert!(bounded_close_turns<100000,"retained fixture closure must terminate");
        close_turns += 1;
        let d=cursor.close_demands(fixture.grant.maximum_capacity_bytes).unwrap();
        let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let(turn,heap)=crate::observe_retirement_allocations(||effect::close_step(&mut cursor,g));let work=turn.work;assert_eq!(work,1);close_cpu+=work;let step=turn.result.expect("spent cursor close");
        assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));close_born+=heap.0;close_freed+=heap.1;
        if let RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress {copied_items:released_items,released_bytes,..}) = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= fixture.grant.maximum_capacity_bytes);
        }
        assert!(close_turns < 100_000, "spent cursor close terminates");
    }
    assert_eq!(close_turns,fixture.expected.spent_close_turns,"a spent cursor funds every original alias, native frame, frontier page and final binding header phase");assert_eq!(close_cpu,u128::from(fixture.expected.spent_close_cpu));assert_eq!(close_born,close_freed);
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
        assert!(effect::progressed(&effect::run(effect::STOCK,|e|values.reserve_one(e)).result.expect("oversized owner reserve")));
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
            assert_eq!(effect::advance(&mut cursor,source.borrow(),64,MAXIMUM_CLONE_WORK,denied).result.unwrap().progress(),Default::default());
            assert_eq!(cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap(),quote);
        }
        match effect::advance(&mut cursor,source.borrow(),64,MAXIMUM_CLONE_WORK,funded).result.expect("oversized owner retained copy") {
            RetainedCloneStep::Progress(progress) => assert_ne!(progress, RetainedCloneProgress::default()),
            RetainedCloneStep::Complete(_) => {let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let taken=effect::take(&mut cursor,g);assert_eq!(taken.work,1);let(value,p)=taken.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!(p.copied_bytes,d.copy_bytes);break value},
        }
    };
    assert_eq!(copied.get(0), Some(&OversizedOwner { first: "alpha".into(), second: "beta".into(), third: "gamma".into() }));
    assert_eq!(effect::begin_close(&mut cursor),1);
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
        assert!(effect::progressed(&effect::run(effect::STOCK,|e|values.reserve_one(e)).result.expect("insufficient scaffold reserve")));
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
        let turn=effect::advance(&mut cursor,source.borrow(),64,MAXIMUM_CLONE_WORK,grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(progress)) if progress == Default::default() => break cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap(),
            Ok(RetainedCloneStep::Progress(_)) => {},
            Ok(RetainedCloneStep::Complete(_)) => panic!("insufficient scaffold release must not complete"),
            Err(error) => panic!("{error:?}"),
        }
    };
    assert_eq!(withheld.release_bytes,128);
    assert_eq!(effect::advance(&mut cursor,source.borrow(),64,MAXIMUM_CLONE_WORK,grant).result.unwrap().progress(),Default::default());
    assert_eq!(effect::begin_close(&mut cursor),1);
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
        let turn=effect::advance(&mut cursor,source.borrow(),64,MAXIMUM_CLONE_WORK,grant);let work=turn.work;if let Ok(step)=&turn.result{assert!(step.progress()==Default::default()||work>0)}match turn.result {
            Ok(RetainedCloneStep::Progress(progress)) if progress == Default::default() => break cursor.advance_demands(source.borrow(),grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK).unwrap(),
            Ok(RetainedCloneStep::Progress(_)) => {},
            Ok(RetainedCloneStep::Complete(_)) => panic!("derived insufficient scaffold release must not complete"),
            Err(error) => panic!("{error:?}"),
        }
    };
    assert_eq!(withheld.release_bytes,128);
    assert_eq!(effect::advance(&mut cursor,source.borrow(),64,MAXIMUM_CLONE_WORK,grant).result.unwrap().progress(),Default::default());
    assert_eq!(effect::begin_close(&mut cursor),1);
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
        if matches!(effect::advance(&mut cursor,retained.borrow(),fixture.grant.maximum_copy_bytes,MAXIMUM_CLONE_WORK,grant).result.expect("paged retained prefix"), RetainedCloneStep::Complete(_)) {
            panic!("fixture must cancel a partial paged copy");
        }
    }
    assert_eq!(effect::begin_close(&mut cursor),1);
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

