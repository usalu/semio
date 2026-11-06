//! 🧪️ Original normal-work funding matches neutral arithmetic and actual allocator events.
use semio_framework_job::*;
use semio_framework_value::RetirementDemand;
#[global_allocator]
static ORIGINAL_TEST_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
fn grant(v: &serde_json::Value) -> RetainedCloneGrant {
    let n = |key| v[key].as_u64().unwrap() as usize;
    RetainedCloneGrant { maximum_items: n("maximumItems"), maximum_copy_bytes: n("maximumCopyBytes"), maximum_capacity_bytes: n("maximumCapacityBytes"), maximum_release_bytes: n("maximumReleaseBytes"), maximum_depth: n("maximumDepth") }
}
#[test]
fn original_step_context_forwards_independent_work_funding() {
    use semio_framework_job::{CancelToken, Generation, OperationId, StepBudget, StepContext};
    use semio_framework_job_payload::JobPayloadAuthority;
    fn now() -> Option<u64> {
        Some(0)
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let owner_grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 65536, maximum_release_bytes: 262144, maximum_depth: 64 };
    let ((cancel, birth), heap) = observe(|| CancelToken::admit_root(owner_grant).unwrap().unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (birth.retained_capacity_bytes, birth.released_bytes));
    let mut born = heap.requested_bytes;
    let ((mut authority, birth), heap) = observe(|| JobPayloadAuthority::admit(OperationId(91020), Generation(1), owner_grant).unwrap().unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (birth.retained_capacity_bytes, birth.released_bytes));
    born += heap.requested_bytes;
    for row in fixture["contexts"].as_array().unwrap() {
        let original = grant(&row["grant"]);
        let mut preview = 0;
        let mut receipt = Default::default();
        let mut cx = StepContext::with_payload_authority(OperationId(91020), Generation(1), StepBudget::new(row["fuel"].as_u64().unwrap(), 1000, original), &cancel, now, &mut preview, &mut receipt, &authority).unwrap();
        let quote = RetirementDemand { copy_bytes: 17, capacity_bytes: 17, release_bytes: 0, depth: 1 };
        let mut bytes = Vec::<u8>::new();
        let (accepted, heap) = observe(|| cx.retained_work_can_enter(quote));
        assert_eq!(accepted, row["accepted"].as_bool().unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(cx.retained_grant(), original);
        cx.consume_fuel(u64::MAX);
        assert_eq!(cx.retained_grant(), original);
        if accepted {
            let (_, heap) = observe(|| {
                bytes.try_reserve_exact(17).unwrap();
                bytes.extend_from_slice(&[71; 17]);
            });
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: 17, retained_capacity_bytes: heap.requested_bytes, released_bytes: 0 }).unwrap();
            assert_eq!(heap.requested_bytes, 17);
            assert!(cx.retained_work_can_enter(RetirementDemand { release_bytes: 17, depth: 1, ..Default::default() }));
            let (_, heap) = observe(|| drop(bytes));
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, released_bytes: heap.released_bytes, ..Default::default() }).unwrap();
            assert_eq!(cx.retained_progress().released_bytes, 17);
            assert_eq!(cx.retained_grant().maximum_capacity_bytes, 0);
        } else {
            assert!(bytes.is_empty());
        }
        assert!(cx.retained_progress().fits(original));
    }
    let (step, heap) = observe(|| authority.close_step(owner_grant).unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    let mut released = heap.released_bytes;
    let mut cancellation = semio_framework_async::CancelTokenRetirement::from_token(cancel);
    while !cancellation.terminal_is_empty() {
        let (step, heap) = observe(|| cancellation.close_step(owner_grant).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        released += heap.released_bytes;
    }
    assert_eq!(born, released);
}
