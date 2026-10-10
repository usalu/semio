use super::*;
use std::sync::Mutex;

struct PhysicalDemandJob {
    backing: Option<Vec<u8>>,
    calls: Arc<Mutex<Vec<RetainedCloneGrant>>>,
    closing: bool,
}

impl InteractiveJob for PhysicalDemandJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,ValueError> {JobOutcomeBorrow::admit_yield(cx)}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}
    fn begin_close(&mut self) { self.closing = true; }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.backing.as_ref().map_or(0,Vec::capacity))}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.backing.is_some()))}
    fn close_step(&mut self, grant:RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.calls.lock().unwrap().push(grant);
        let Some(backing) = self.backing.as_ref() else { return InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()}; };
        let bytes = backing.capacity();
        if !self.closing||grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0 {
            return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()};
        }
        drop(self.backing.take());
        InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..RetainedCloneProgress::default()}}
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.backing.is_none() }
}

#[test]
fn worker_close_demand_queries_preserve_exact_physical_grants() {
    let _admission = worker_session_slots_shared();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut backing = Vec::new();
        backing.try_reserve_exact(extent).unwrap();
        assert_eq!(backing.capacity(), extent);
        let calls = Arc::new(Mutex::new(Vec::with_capacity(64)));
        let job = PhysicalDemandJob { backing: Some(backing), calls: Arc::clone(&calls), closing: false };
        let params = BatchJobParams { operation: OperationId(98_100), generation: Generation(11), cancel: root_cancel_token(), config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "test.close-demand", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 1_000 }, now_us: default_now_us };
        let mut session = admit_original_fixture_owner!(WorkerJobSession,job, params).unwrap_or_else(|_| panic!("close demand session admission"));
        session.begin_close();
        let scaffold_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:admission,maximum_depth:64};
        for _ in 0..16 {
            if session.close_phase() == WorkerJobClosePhase::Job { break; }
            session.close_step(scaffold_grant);
        }
        assert_eq!(session.close_phase(), WorkerJobClosePhase::Job);
        let((first_demand,second_demand),query_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(session.retirement_demands(3).unwrap(),session.retirement_demands(3).unwrap()));
        assert_eq!(query_heap.requested_bytes,0);assert_eq!(query_heap.released_bytes,0);
        let calls_before = calls.lock().unwrap().len();
        let grant = row["callerBytes"].as_u64().unwrap() as usize;
        let caller=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:fixture["grant"]["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:grant,maximum_depth:fixture["grant"]["maximumDepth"].as_u64().unwrap()as usize};
        let observed = session.close_step(caller);
        let demand_after = session.retirement_demands(3).unwrap();
        let actual_calls = calls.lock().unwrap().clone();
        for _ in 0..32 {
            if session.terminal_is_empty() { break; }
            session.close_step(scaffold_grant);
        }
        assert!(session.terminal_is_empty());
        assert_eq!(first_demand.release_bytes,extent);assert_eq!(first_demand.copy_bytes,0);assert_eq!(first_demand.capacity_bytes,0);assert_eq!(first_demand.depth,1);
        assert_eq!(second_demand,first_demand);
        assert_eq!(calls_before, 0, "querying demand performs no close work");
        if grant>=extent{assert_eq!(actual_calls,[caller],"worker forwards the original independently funded grant");}else{assert!(actual_calls.is_empty(),"undergrant preserves original owner before entering its callback");}
        let released = row["releasedBytes"].as_u64().unwrap() as usize;
        assert_eq!(observed,WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:usize::from(released>0),released_bytes:released,..RetainedCloneProgress::default()}});
        assert!(observed_progress(observed).fits(caller));
        assert_eq!(demand_after.release_bytes,if released>0{0}else{extent});
        eprintln!("[DEBUG] worker close demand {}: extent={extent} caller={grant} released={released}", row["name"]);
    }
}

fn observed_progress(step:WorkerJobCloseStep)->RetainedCloneProgress{match step{WorkerJobCloseStep::Pending{progress}|WorkerJobCloseStep::Complete{progress}=>progress,_=>panic!("original physical law requires a receipt")}}

#[test]
fn worker_close_receipt_preserves_all_original_currencies_without_heap_effects(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();let p=&law["progress"];let progress=RetainedCloneProgress{copied_items:p["copiedItems"].as_u64().unwrap()as usize,copied_bytes:p["copiedBytes"].as_u64().unwrap()as usize,retained_capacity_bytes:p["retainedCapacityBytes"].as_u64().unwrap()as usize,released_bytes:p["releasedBytes"].as_u64().unwrap()as usize};
 let g=&law["grant"];let grant=RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap()as usize};
 for step in [WorkerJobCloseStep::Pending{progress},WorkerJobCloseStep::Complete{progress}]{let(actual,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||step.progress());assert_eq!(actual,progress);assert!(actual.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));for denied in [RetainedCloneGrant{maximum_items:grant.maximum_items-1,..grant},RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant},RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant},RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes-1,..grant}]{assert!(!actual.fits(denied));}}
 assert_eq!(WorkerJobCloseStep::Blocked.progress(),RetainedCloneProgress::default());assert_eq!(WorkerJobCloseStep::Refused{kind:ValueRefusalKind::WorkLimit,progress:RetainedCloneProgress::default()}.progress(),RetainedCloneProgress::default());
 println!("[DEBUG] worker Pending/Complete full receipt preserved items1/copy3/capacity128/release65536 pure heap0, each independent undergrant refused");
}

#[test]
fn original_failed_job_close_preserves_actual_allocated_receipt_before_refusal(){
 let(mut original,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut value=Vec::<u8>::new();value.try_reserve_exact(64).unwrap();value});let pointer=original.as_ptr();assert_eq!((heap.requested_bytes,heap.released_bytes),(64,0));
 let progress=RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:heap.requested_bytes,released_bytes:heap.released_bytes};
 let error=ValueError::literal(ValueRefusalKind::InvariantViolated,"original producer failed after physical reservation").with_retained_progress(progress);
 let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||worker_payload_step(Err(error)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(step.progress(),progress);assert_eq!(original.as_ptr(),pointer);
 let denied=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};assert!(!step.progress().fits(denied));
 let refused=InteractiveJobCloseStep::Complete{progress}.admit(denied,false);assert_eq!(refused.progress(),progress);assert!(matches!(refused,InteractiveJobCloseStep::Refused{kind:ValueRefusalKind::InvariantViolated,..}));
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(std::mem::take(&mut original)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,64));
 eprintln!("[DEBUG] original Worker/Interactive Refused preserves real allocated64 receipt before failure without heap effect; same original Vec pointer retained until physical release64");
}
