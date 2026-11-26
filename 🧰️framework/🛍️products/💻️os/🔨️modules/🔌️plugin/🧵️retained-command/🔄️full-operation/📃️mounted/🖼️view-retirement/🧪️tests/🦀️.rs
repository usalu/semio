use super::*;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

#[test]
fn plain_action_view_returns_its_exact_original_allocation_in_one_item() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let view: ViewModel = serde_json::from_value(law["view"].clone()).unwrap();
    assert_eq!(serde_json::to_value(&view).unwrap(), law["view"]);
    let mut owner = Some(Arc::new(view));
    let alias = owner.clone();
    assert!(compact_demands(&owner).is_none());
    drop(alias);
    let (demand, work) = compact_demands(&owner).unwrap();
    assert_eq!(work, law["expected"]["work"].as_u64().unwrap());
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 64 };
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth, ..Default::default() };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(44851), Generation(1), stock, || Some(1), u64::MAX, 65_536);
    let denied = RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant };
    let (step, heap) = observe(|| caller.retirement_turn(denied, |cx| close_original_effect_step(cx, demand, |effect| {
        let result = compact_close(&mut owner, demand, work, effect).map(RetainedCloneStep::Progress);
        RetainedWorkPoll::Complete(RetainedWorkOutcome { work: effect.performed_work(), result })
    })).unwrap());
    assert_eq!(step.progress(), Default::default());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert!(owner.is_some());
    let (step, heap) = observe(|| caller.retirement_turn(grant, |cx| close_original_effect_step(cx, demand, |effect| {
        let result = compact_close(&mut owner, demand, work, effect).map(RetainedCloneStep::Progress);
        RetainedWorkPoll::Complete(RetainedWorkOutcome { work: effect.performed_work(), result })
    })).unwrap());
    assert!(owner.is_none());
    let progress = step.progress();
    assert_eq!(progress.copied_items as u64, law["expected"]["items"].as_u64().unwrap());
    assert_eq!(heap.requested_bytes as u64, law["expected"]["allocationBytes"].as_u64().unwrap());
    assert_eq!(progress.released_bytes, heap.released_bytes);
    assert_eq!(progress.released_bytes, demand.release_bytes);
    caller.retire(stock);
}
