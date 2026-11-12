//! 🎟️ The independent budget owner preserves neutral arithmetic and real backing-allocation receipts.
use semio_framework_job_work_grant::RetainedWorkBudget;
use semio_framework_value_retained_clone::{
    retained_clone::{RetainedCloneGrant, RetainedCloneProgress},
    RetirementDemand,
};
use std::mem::size_of;
use semio_framework_value_testing_allocator as _;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
fn grant(v: &serde_json::Value) -> RetainedCloneGrant {
    let n = |key| v[key].as_u64().unwrap() as usize;
    RetainedCloneGrant { maximum_items: n("maximumItems"), maximum_copy_bytes: n("maximumCopyBytes"), maximum_capacity_bytes: n("maximumCapacityBytes"), maximum_release_bytes: n("maximumReleaseBytes"), maximum_depth: n("maximumDepth") }
}
fn demand(v: &serde_json::Value) -> RetirementDemand {
    let n = |key| v[key].as_u64().unwrap() as usize;
    RetirementDemand { copy_bytes: n("copyBytes"), capacity_bytes: n("capacityBytes"), release_bytes: n("releaseBytes"), depth: n("depth") }
}
#[test]
fn original_retained_work_funding_preserves_axes_and_physical_receipts() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let original = grant(&row["grant"]);
        let quote = demand(&row["demand"]);
        let budget = RetainedWorkBudget::new(original);
        let (accepted, heap) = observe(|| budget.can_enter(quote));
        assert_eq!(heap.requested_bytes, 0);
        assert_eq!(heap.released_bytes, 0);
        assert_eq!(accepted, row["accepted"].as_bool().unwrap());
        assert_eq!(budget.remaining(), original);
    }
    let row = &fixture["sequence"];
    let mut budget = RetainedWorkBudget::new(grant(&row["grant"]));
    let quote = demand(&row["demand"]);
    let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: 8, retained_capacity_bytes: 16, released_bytes: 4 };
    for accepted in [true, true, false] {
        assert_eq!(budget.can_enter(quote), accepted);
        if accepted {
            budget.charge(receipt).unwrap();
        }
    }
    let original = budget.remaining();
    assert!(budget.charge(receipt).is_err());
    assert_eq!(budget.remaining(), original);
    let original = RetainedCloneGrant { maximum_items: 2, maximum_copy_bytes: size_of::<Vec<u8>>(), maximum_capacity_bytes: 17, maximum_release_bytes: 17, maximum_depth: 1 };
    let mut budget = RetainedWorkBudget::new(original);
    let quote = RetirementDemand { copy_bytes: size_of::<Vec<u8>>(), capacity_bytes: 17, release_bytes: 0, depth: 1 };
    let mut source = Vec::<u8>::new();
    for axis in 0..5 {
        let mut denied = original;
        match axis {
            0 => denied.maximum_items = 0,
            1 => denied.maximum_copy_bytes -= 1,
            2 => denied.maximum_capacity_bytes -= 1,
            3 => continue,
            _ => denied.maximum_depth = 0,
        };
        let (accepted, heap) = observe(|| RetainedWorkBudget::new(denied).can_enter(quote));
        assert!(!accepted);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(source.is_empty());
    }
    assert!(budget.can_enter(quote));
    let (_, heap) = observe(|| source.try_reserve_exact(17).unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (17, 0));
    budget.charge(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Vec<u8>>(), retained_capacity_bytes: heap.requested_bytes, released_bytes: 0 }).unwrap();
    assert!(budget.can_enter(RetirementDemand { release_bytes: 17, depth: 1, ..Default::default() }));
    let (_, heap) = observe(|| drop(source));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 17));
    budget.charge(RetainedCloneProgress { copied_items: 1, released_bytes: heap.released_bytes, ..Default::default() }).unwrap();
    assert_eq!(budget.remaining(), RetainedCloneGrant { maximum_depth: 1, ..Default::default() });
}
