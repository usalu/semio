use super::{close_tool_erased_retirement, tool_erased_retirement_demand};
use semio_framework_job::{CancelToken, Generation, OperationId, StepBudget, StepContext};
use semio_framework_value::{factory::ArcRootRetirement, ArtifactOwnedValueRetirementFactory, OwnedValueRetirementFactory, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedWorkPoll, RetirementDemand};
use std::sync::Arc;

fn with_effect<R>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> R {
    let mut sequence = 0;
    let mut progress = RetainedCloneProgress::default();
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(65536, u64::MAX, grant), CancelToken::root_now(), || Some(1), &mut sequence, &mut progress);
    cx.with_original_effect(grant, |_, effect| run(effect)).unwrap().unwrap().0
}

fn grant(demand: RetirementDemand, items: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: demand.copy_bytes.max(4096), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

#[test]
fn tool_run_closes_owned_and_aliased_store_roots_through_the_resident_reply() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["contract"], "plugin.tool-run-retained-snapshot-close/v1");
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let payload = Arc::new(String::from(id));
        let weak = Arc::downgrade(&payload);
        let retained = case["keepAlias"].as_bool().unwrap().then(|| Arc::clone(&payload));
        let mut original = Some(payload);
        let mut factory: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<String>>> = Some(Arc::new(OwnedValueRetirementFactory::<String>::default()));
        let mut owner = None;
        let admission = grant(ArcRootRetirement::<String>::admission_demand(), 1);
        let poll = with_effect(admission, |effect| ArcRootRetirement::admit_original(&mut original, &mut factory, &mut owner, effect));
        assert!(matches!(poll, RetainedWorkPoll::Complete(turn) if turn.result.is_ok()));
        assert!(original.is_none() && owner.is_some());
        let demand = tool_erased_retirement_demand(&owner, 4096).unwrap();
        let denied = grant(demand, case["deniedItems"].as_u64().unwrap() as usize);
        assert_eq!(with_effect(denied, |effect| close_tool_erased_retirement(&mut owner, effect)).unwrap().progress(), RetainedCloneProgress::default());
        for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
            if owner.is_none() { break; }
            let funded = grant(tool_erased_retirement_demand(&owner, 4096).unwrap(), 1);
            let progress = with_effect(funded, |effect| close_tool_erased_retirement(&mut owner, effect)).unwrap().progress();
            assert!(progress.fits(funded));
        }
        assert_eq!(owner.is_none(), case["terminalEmpty"].as_bool().unwrap(), "{id}");
        assert_eq!(weak.upgrade().is_some(), case["payloadAlive"].as_bool().unwrap(), "{id}");
        drop(retained);
    }
}
