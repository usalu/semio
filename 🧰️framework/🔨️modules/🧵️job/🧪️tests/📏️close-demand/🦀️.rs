use super::*;
use std::sync::Mutex;

struct PhysicalDemandJob {
    backing: Option<Vec<u8>>,
    calls: Arc<Mutex<Vec<RetainedCloneGrant>>>,
    closing: bool,
}

impl InteractiveJob for PhysicalDemandJob {
    fn step(&mut self, _cx: &mut StepContext<'_>) -> StepOutcome { StepOutcome::Yield }
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
        let calls = Arc::new(Mutex::new(Vec::new()));
        let job = PhysicalDemandJob { backing: Some(backing), calls: Arc::clone(&calls), closing: false };
        let params = BatchJobParams { operation: OperationId(98_100), generation: Generation(11), cancel: root_cancel_token(), config: BatchDriveConfig { site: "test.close-demand", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 1_000 }, now_us: default_now_us };
        let session = WorkerJobSession::try_new(job, params).unwrap_or_else(|_| panic!("close demand session admission"));
        session.begin_close();
        for _ in 0..16 {
            if session.close_phase() == WorkerJobClosePhase::Job { break; }
            session.close_step(RetainedCloneGrant::one_release_turn(admission,64));
        }
        assert_eq!(session.close_phase(), WorkerJobClosePhase::Job);
        let first_demand = session.next_close_demands(3).unwrap();
        let second_demand = session.next_close_demands(3).unwrap();
        let calls_before = calls.lock().unwrap().len();
        let grant = row["callerBytes"].as_u64().unwrap() as usize;
        let caller=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:fixture["grant"]["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:fixture["grant"]["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:grant,maximum_depth:fixture["grant"]["maximumDepth"].as_u64().unwrap()as usize};
        let observed = session.close_step(caller);
        let demand_after = session.next_close_demands(3).unwrap();
        let actual_calls = calls.lock().unwrap().clone();
        for _ in 0..32 {
            if session.terminal_is_empty() { break; }
            session.close_step(RetainedCloneGrant::one_release_turn(admission,64));
        }
        assert!(session.terminal_is_empty());
        assert_eq!(first_demand.maximum_release_bytes,extent);assert_eq!(first_demand.maximum_copy_bytes,0);assert_eq!(first_demand.maximum_capacity_bytes,0);assert_eq!(first_demand.maximum_depth,1);
        assert_eq!(second_demand,first_demand);
        assert_eq!(calls_before, 0, "querying demand performs no close work");
        assert_eq!(actual_calls,[caller],"worker forwards all original independent caller grants");
        let released = row["releasedBytes"].as_u64().unwrap() as usize;
        assert_eq!(observed,WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:usize::from(released>0),released_bytes:released,..RetainedCloneProgress::default()}});
        assert!(observed_progress(observed).fits(caller));
        assert_eq!(demand_after.maximum_release_bytes,if released>0{0}else{extent});
        eprintln!("[DEBUG] worker close demand {}: extent={extent} caller={grant} released={released}", row["name"]);
    }
}

fn observed_progress(step:WorkerJobCloseStep)->RetainedCloneProgress{match step{WorkerJobCloseStep::Pending{progress}|WorkerJobCloseStep::Complete{progress}=>progress,_=>panic!("original physical law requires a receipt")}}
