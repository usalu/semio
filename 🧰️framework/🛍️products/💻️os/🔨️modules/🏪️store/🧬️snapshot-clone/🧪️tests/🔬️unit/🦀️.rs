use super::*;

#[test]
fn lifecycle_fixture_is_schema_first() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/📦️lifecycle/🔣️.json")).expect("retained clone preparation fixture");
    assert_eq!(fixture["cases"].as_array().expect("lifecycle cases").len(), 6);
    assert!(fixture["largeCapacity"]["stringByteLength"].as_u64().expect("large string byte length") > fixture["grant"]["maximumBytes"].as_u64().expect("per-turn byte grant"));
    assert_eq!(fixture["largeCapacity"]["expectedCode"], "retained-clone.step-grant-too-small");
    let maximum = fixture["grant"]["maximumBytes"].as_u64().unwrap() as usize;
    for grant in [RetainedCloneGrant::one_capacity_turn(maximum, 64), RetainedCloneGrant::one_payload_turn(maximum, 64), RetainedCloneGrant::one_release_turn(maximum, 64)] {
        assert_eq!(grant.maximum_capacity_bytes + grant.maximum_copy_bytes + grant.maximum_release_bytes, fixture["grant"]["combinedCapacityCopyAndReleaseMaximum"].as_u64().unwrap() as usize);
    }
}

#[test]
fn snapshot_clone_original_inline_custody_observes_exact_granted_birth_work_and_release() {
    fn run<T: RetireOwned>(value: T) {
        let mut original = Some(value);
        let mut owner = None;
        let initial = pending::demands(&original, &owner, 0).unwrap();
        assert_eq!(initial.copy_bytes, size_of::<T>());
        let below = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: initial.copy_bytes.saturating_sub(1), maximum_depth: initial.depth, ..Default::default() };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| pending::close(&mut original, &mut owner, below).unwrap().unwrap());
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(original.is_some());
        for _ in 0..100_000 {
            if original.is_none() && owner.is_none() {
                let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner.take()));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                return;
            }
            let (demand, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                let body = pending::demands(&original, &owner, 0).unwrap().copy_bytes;
                pending::demands(&original, &owner, body).unwrap()
            });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(demand.copy_bytes + demand.capacity_bytes + demand.release_bytes <= 4096);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (zero, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| pending::close(&mut original, &mut owner, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap().unwrap());
            assert_eq!(zero.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| pending::close(&mut original, &mut owner, grant).unwrap().unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            if grant.maximum_copy_bytes > 0 && grant.maximum_release_bytes == 0 { assert_eq!(heap.released_bytes, 0); }
        }
        panic!("snapshot clone original inline custody did not close under exact demands");
    }
    run(7u64);
    run("original\0Ä🧩".to_string());
    run((0..33).collect::<Vec<u32>>());
    println!("[DEBUG] snapshot clone original scalar/text/sequence retains undergrant and closes under exact separate work/birth/release/depth");
}
