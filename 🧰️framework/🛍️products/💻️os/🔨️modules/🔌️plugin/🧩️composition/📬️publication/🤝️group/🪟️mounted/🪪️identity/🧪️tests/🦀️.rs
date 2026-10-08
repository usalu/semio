#[test]
fn mounted_child_group_identity_preserves_original_inputs_and_physical_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let grant = |items, capacity, release| RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: 64, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 };
    for row in fixture["cases"].as_array().unwrap() {
        let revision: [u8; 32] = std::array::from_fn(|index| u8::from_str_radix(&row["revision"].as_str().unwrap()[index * 2..index * 2 + 2], 16).unwrap());
        let source = MountedChildGroupIdentitySource { parent: row["parent"].as_str().unwrap(), actor: row["actor"].as_str().unwrap(), revision, instance: row["instance"].as_u64().unwrap() as u32, operation: row["operation"].as_u64().unwrap() };
        let mut owner = MountedChildGroupIdentityIssuer::new();
        for _ in 0..32 {
            if owner.next_capacity_byte_demand() == 70 { break; }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, grant(1, 0, 0)).unwrap());
            assert!(step.progress().copied_items <= 1 && step.progress().copied_bytes <= 64);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        assert_eq!(owner.next_capacity_byte_demand(), 70);
        for denied in [grant(0, 70, 0), grant(1, 0, 0), grant(1, 69, 0)] {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, denied).unwrap());
            assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(owner.next_capacity_byte_demand(), 70);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, grant(1, 70, 0)).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (70, 0));
        assert_eq!(step.progress().retained_capacity_bytes, 70);
        for _ in 0..2 { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, grant(1, 0, 0)).unwrap()); assert!(step.progress().copied_bytes <= 64); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); }
        assert!(owner.ready());
        let pointer = owner.output.as_ref().unwrap().as_ptr();
        assert!(owner.take_ready(grant(0, 0, 0)).is_none());
        let (identity, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take_ready(grant(1, 0, 0)).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(identity.as_ptr(), pointer);
        assert_eq!(identity, row["expectedId"].as_str().unwrap());
        assert!(owner.terminal_is_empty()); drop(identity);
        for stop in [0, 1, 4, 8, 9, 10, 11] {
            let mut cancelled = MountedChildGroupIdentityIssuer::new();
            for _ in 0..stop { let capacity = cancelled.next_capacity_byte_demand(); cancelled.advance(source, grant(1, capacity, 0)).unwrap(); }
            for _ in 0..3 {
                if cancelled.terminal_is_empty() { break; }
                let bytes = cancelled.next_close_byte_demand();
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cancelled.close_granted(grant(0, 0, bytes)));
                assert_eq!(step, RetainedCloneStep::Progress(Default::default())); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                if bytes != 0 { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cancelled.close_granted(grant(1, 0, bytes - 1))); assert_eq!(step, RetainedCloneStep::Progress(Default::default())); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); }
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cancelled.close_granted(grant(1, 0, bytes)));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes));
            }
            assert!(cancelled.terminal_is_empty());
        }
        println!("[DEBUG] original mounted group ID case={} Node/Python SHA256 fixture matched; whole70 birth, copy64, original transfer, seven cancelled frontiers exact native heap parity", row["id"]);
    }
}
