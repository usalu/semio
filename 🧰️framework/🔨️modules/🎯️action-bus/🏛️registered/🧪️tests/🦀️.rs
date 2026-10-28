//! 🧪️ Native aliases name the same process-owned factory registry and return only their own custody.
use super::RegisteredActionBusSource;
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneStep};
const TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 4096 };
#[test]
fn registered_bus_alias_keeps_the_original_process_registry() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let source = RegisteredActionBusSource::production();
    let identity = source.identity();
    assert_ne!(identity, 0);
    let mut aliases = Vec::new();
    for _ in 0..law["mountedAliases"].as_u64().unwrap() {
        let d = source.alias_demands();
        for axis in 0..3 {
            let mut denied = TURN;
            match axis {
                0 => denied.maximum_items = 0,
                1 => denied.maximum_copy_bytes = d.copy_bytes - 1,
                _ => denied.maximum_depth = 0,
            }
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| source.admit_alias(denied).unwrap());
            assert!(result.is_none());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let ((alias, p), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| source.admit_alias(TURN).unwrap().unwrap());
        assert!(p.fits(TURN));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (p.retained_capacity_bytes, p.released_bytes));
        assert_eq!(p.retained_capacity_bytes, law["nativeAliasAllocationBytes"].as_u64().unwrap() as usize);
        assert_eq!(alias.identity(), Some(identity));
        aliases.push(alias);
    }
    for alias in &mut aliases {
        let d = alias.close_demands();
        for axis in 0..3 {
            let mut denied = TURN;
            match axis {
                0 => denied.maximum_items = 0,
                1 => denied.maximum_copy_bytes = d.copy_bytes - 1,
                _ => denied.maximum_depth = 0,
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| alias.close_step(denied));
            assert_eq!(step.progress(), Default::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(alias.identity(), Some(identity));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| alias.close_step(TURN));
        assert!(matches!(step, RetainedCloneStep::Complete(_)));
        assert!(alias.terminal_is_empty());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, law["nativeAliasReleaseBytes"].as_u64().unwrap() as usize));
    }
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(aliases));
    assert_eq!(heap.requested_bytes, 0);
    let mut original = Some(source);
    let mut close = RetainedCloneClose::default();
    for turn in 0..4096 {
        if original.is_none() && close.is_empty() {
            break;
        }
        let d = close.next_owner_demands::<RegisteredActionBusSource>(original.is_some(), &None, 4096).unwrap();
        assert!(d.copy_bytes <= TURN.maximum_copy_bytes && d.capacity_bytes <= TURN.maximum_capacity_bytes && d.release_bytes <= TURN.maximum_release_bytes && d.depth <= TURN.maximum_depth);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| if let Some(step) = close.begin_granted(&mut original, TURN).unwrap() { step } else { close.step_granted(TURN).unwrap() });
        assert!(step.progress().fits(TURN));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        assert!(turn < 4095)
    }
    let (new, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(RegisteredActionBusSource::production);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(new.identity(), identity);
}
