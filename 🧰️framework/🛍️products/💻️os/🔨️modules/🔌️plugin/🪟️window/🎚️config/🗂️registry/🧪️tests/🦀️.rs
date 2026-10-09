//! 🧪️ Original registry pages refuse independently and release their physical backing exactly.

use super::*;

#[test]
fn window_config_paged_registry_actual_window_partition_custody() {
    use super::super::{WindowConfigOwner, WindowConfigOwnerRegistry, PluginLifecycleStep, pack_identity_tests::IdentityWindowOwner};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let copies = fixture["owningCopies"].as_u64().unwrap() as usize;
    let repeat = fixture["addressRepeat"].as_u64().unwrap() as usize;
    let (mut registry, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_async::block_on(async {
        let mut registry = WindowConfigOwnerRegistry::new(crate::protocol::ActorId("original-registry-actor/λ".to_owned().into()));
        registry.register::<IdentityWindowOwner>().unwrap();
        for address in fixture["addresses"].as_array().unwrap() {
            for ordinal in 0..copies {
                let address = format!("{}/{ordinal:06}", address.as_str().unwrap().repeat(repeat));
                drop(registry.owners.get_mut(IdentityWindowOwner::WINDOW_KIND_ID).unwrap().capture(&address).await.unwrap());
            }
        }
        registry
    }));
    let mut allocated = birth.requested_bytes;
    let mut released = birth.released_bytes;
    let mut turns = 0;
    while !registry.terminal_is_empty() {
        let body = fixture["maximumPageBytes"].as_u64().unwrap() as usize;
        let demand = registry.retirement_demands(body).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let mut denied = vec![RetainedCloneGrant { maximum_items: 0, ..grant }];
        if demand.copy_bytes > 0 { denied.push(RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant }); }
        if demand.capacity_bytes > 0 { denied.push(RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }); }
        if demand.release_bytes > 0 { denied.push(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }); }
        if demand.depth > 0 { denied.push(RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }); }
        for denied in denied {
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(denied));
            let progress = result.unwrap().progress().unwrap_or_default();
            assert_eq!(progress, RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(grant));
        let result = result.unwrap();
        let progress = result.progress().expect("actual original window close reports its independent currencies");
        if matches!(result, PluginLifecycleStep::Complete(_)) { assert!(registry.terminal_is_empty()); }
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        allocated += heap.requested_bytes;
        released += heap.released_bytes;
        turns += 1;
        assert!(turns < 2_000_000);
    }
    assert_eq!(allocated, released);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(registry));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, fixture["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] actual original window partitions={} physical birth={allocated} release={released} turns={turns} terminalDrop0", copies * 4);
}

#[test]
fn window_config_paged_registry_original_order_and_physical_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let copies = fixture["copies"].as_u64().unwrap() as usize;
    let repeat = fixture["addressRepeat"].as_u64().unwrap() as usize;
    let mut registry = WindowRegistry::<String, usize>::new();
    let (_, born) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        for (address_index, address) in fixture["addresses"].as_array().unwrap().iter().enumerate() {
            for ordinal in 0..copies {
                registry.insert(format!("{}/{ordinal:06}", address.as_str().unwrap().repeat(repeat)), address_index * copies + ordinal);
            }
        }
    });
    let expected = fixture["ordered"].as_array().unwrap().iter().flat_map(|address| (0..copies).map(move |ordinal| format!("{}/{ordinal:06}", address.as_str().unwrap().repeat(repeat)))).collect::<Vec<_>>();
    assert_eq!(registry.iter().map(|(key, _)| key).collect::<Vec<_>>(), expected.iter().collect::<Vec<_>>());
    let mut allocated = born.requested_bytes;
    let mut released = born.released_bytes;
    let mut released_pages = 0;
    while !registry.is_empty() {
        let demand = registry.pop_demand().unwrap();
        let original_pointer = registry.last().unwrap().0.as_ptr();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant }, RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }] {
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.pop_original(denied));
            assert!(result.unwrap().is_none());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (((key, value), progress), heap) = {
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.pop_original(grant));
            (result.unwrap().unwrap(), heap)
        };
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(progress.fits(grant));
        assert_eq!(format!("{}/{:06}", fixture["addresses"][value / copies].as_str().unwrap().repeat(repeat), value % copies), key);
        assert_eq!(key.as_ptr(), original_pointer);
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::retirement::controlled::ControlledRetirement::new(key));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut owner = result.unwrap_or_else(|_| panic!("original registry address refuses controlled ownership"));
        while !owner.terminal_is_empty() {
            let copy = owner.next_copy_byte_demand().unwrap().max(fixture["maximumPageBytes"].as_u64().unwrap() as usize);
            let demand = RetirementDemand { copy_bytes: owner.next_copy_byte_demand().unwrap(), capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(), release_bytes: owner.next_release_byte_demand().unwrap(), depth: owner.next_depth_demand().unwrap() };
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let mut denied = vec![RetainedCloneGrant { maximum_items: 0, ..grant }];
            if demand.capacity_bytes > 0 { denied.push(RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }); }
            if demand.release_bytes > 0 { denied.push(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }); }
            if demand.depth > 0 { denied.push(RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }); }
            for denied in denied {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.step(denied));
                let progress = match step {
                    Ok(step) => step.progress(),
                    Err(error) => { assert_eq!(error.kind, ValueRefusalKind::DepthLimit); assert!(denied.maximum_depth < demand.depth); RetainedCloneProgress::default() },
                };
                assert_eq!(progress, RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            if demand.copy_bytes > 0 {
                let short_copy = demand.copy_bytes - 1;
                let narrow = RetainedCloneGrant { maximum_copy_bytes: short_copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(short_copy).unwrap(), ..grant };
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.step(narrow));
                let progress = step.unwrap().progress();
                assert!(progress.fits(narrow));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
                allocated += heap.requested_bytes;
                released += heap.released_bytes;
            }
            let demand = RetirementDemand { copy_bytes: owner.next_copy_byte_demand().unwrap(), capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(), release_bytes: owner.next_release_byte_demand().unwrap(), depth: owner.next_depth_demand().unwrap() };
            let grant = RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth, ..grant };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.step(grant));
            let progress = step.unwrap().progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            allocated += heap.requested_bytes;
            released += heap.released_bytes;
        }
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
    while !registry.terminal_is_empty() {
        let demand = registry.backing_demand().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth, ..Default::default() };
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }, RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }] {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_backing_step(denied));
            assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_backing_step(grant));
        let progress = step.unwrap().progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, progress.released_bytes));
        released += heap.released_bytes;
        released_pages += 1;
    }
    assert!(released_pages > 2);
    assert_eq!(released, allocated);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(registry));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, fixture["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] original window registry lexical order entries={} pages={released_pages} physical birth={allocated} release={released} terminalDrop0", copies * 4);
}
