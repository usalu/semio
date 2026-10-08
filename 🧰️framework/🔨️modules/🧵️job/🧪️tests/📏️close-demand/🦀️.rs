use super::*;
use std::sync::Mutex;

struct PhysicalDemandJob {
    backing: Option<Vec<u8>>,
    calls: Arc<Mutex<Vec<(usize, usize)>>>,
    closing: bool,
}

impl InteractiveJob for PhysicalDemandJob {
    fn step(&mut self, _cx: &mut StepContext<'_>) -> StepOutcome { StepOutcome::Yield }
    fn begin_close(&mut self) { self.closing = true; }
    fn next_close_byte_demand(&self) -> usize { self.backing.as_ref().map_or(0, Vec::capacity) }
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.calls.lock().unwrap().push((maximum_items, maximum_bytes));
        let Some(backing) = self.backing.as_ref() else { return InteractiveJobCloseStep::Complete; };
        let bytes = backing.capacity();
        if !self.closing || maximum_items == 0 || maximum_bytes < bytes {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        drop(self.backing.take());
        InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes }
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
            session.close_step(1, admission);
        }
        assert_eq!(session.close_phase(), WorkerJobClosePhase::Job);
        let first_demand = session.next_close_byte_demand().unwrap();
        let second_demand = session.next_close_byte_demand().unwrap();
        let calls_before = calls.lock().unwrap().len();
        let grant = row["callerBytes"].as_u64().unwrap() as usize;
        let observed = session.close_step(1, grant);
        let demand_after = session.next_close_byte_demand().unwrap();
        let actual_calls = calls.lock().unwrap().clone();
        for _ in 0..32 {
            if session.terminal_is_empty() { break; }
            session.close_step(1, admission);
        }
        assert!(session.terminal_is_empty());
        assert_eq!(first_demand, extent);
        assert_eq!(second_demand, extent);
        assert_eq!(calls_before, 0, "querying demand performs no close work");
        assert_eq!(actual_calls, [(1, grant)], "worker forwards the exact caller grant");
        let released = row["releasedBytes"].as_u64().unwrap() as usize;
        assert_eq!(observed, WorkerJobCloseStep::Pending { released_items: usize::from(released > 0), released_bytes: released });
        assert_eq!(demand_after, if released > 0 { 0 } else { extent });
        eprintln!("[DEBUG] worker close demand {}: extent={extent} caller={grant} released={released}", row["name"]);
    }
}
