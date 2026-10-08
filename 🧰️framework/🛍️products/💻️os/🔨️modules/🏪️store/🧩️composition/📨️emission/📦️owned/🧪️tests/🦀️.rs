use super::*;

#[test]
fn member_owned_batch_copy_query_preserves_zero_work_and_exact_physical_receipts() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let original: Vec<i32> = serde_json::from_value(fixture["values"].clone()).unwrap();
    for row in fixture["copyDemand"]["rowCounts"].as_array().unwrap() {
        let count = row.as_u64().unwrap() as usize;
        let values: Vec<i32> = (0..count).map(|index| original[index % original.len()]).collect();
        assert_eq!(serde_json::to_value(&values).unwrap(), serde_json::Value::Array((0..count).map(|index| fixture["values"][index % original.len()].clone()).collect()));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
        let (mut owner, _) = MemberStoreOwnedBatch::try_new(values, grant).unwrap_or_else(|_| panic!("exact scalar source scaffold admitted"));
        let mut work = 0;
        let mut released = 0;
        for _ in 0..count * 8 + 32 {
            let copy = owner.next_copy_byte_demand().unwrap();
            let demand = owner.next_demands(copy).unwrap();
            let (capacity, release) = (demand.capacity_bytes, demand.release_bytes);
            assert!(copy <= 64 && capacity <= 4096 && release <= 4096);
            if copy != 0 {
                assert_eq!(copy, fixture["copyDemand"]["minimumCopyBytes"].as_u64().unwrap() as usize);
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_granted(RetainedCloneGrant { maximum_copy_bytes: 0, maximum_capacity_bytes: capacity, maximum_release_bytes: release, ..grant }).unwrap());
                assert_eq!(step.progress(), RetainedCloneProgress::default());
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(owner.next_copy_byte_demand().unwrap(), copy);
                assert_eq!(owner.next_demands(copy).unwrap(), demand);
            }
            let exact = RetainedCloneGrant { maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, ..grant };
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_granted(exact).unwrap());
            let progress = step.progress();
            assert!(progress.fits(exact));
            assert_eq!((events.requested_bytes, events.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            work += progress.copied_bytes;
            released += progress.released_bytes;
            if owner.terminal_is_empty() { break; }
        }
        assert!(owner.terminal_is_empty());
        assert_eq!(work, count * std::mem::size_of::<i32>());
        println!("[DEBUG] owned typed scalar source count={count} work={work} exact physical releases={released}; original count*8+32 bound and zero-work allocator parity preserved");
    }
}

#[test]
fn member_owned_batch_borrowed_admission_preserves_original_parent_source() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let source:Vec<i32>=serde_json::from_value(fixture["values"].clone()).unwrap();
    let pointer=source.as_ptr();let capacity=source.capacity();let mut source=Some(source);
    let birth=MemberStoreOwnedBatch::scaffold_byte_demand::<i32>();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:birth,maximum_release_bytes:4096,maximum_depth:64};
    for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:birth-1,..grant}]{
        assert!(MemberStoreOwnedBatch::admit(&mut source,denied).unwrap().is_none());
        assert_eq!(source.as_ref().unwrap().as_ptr(),pointer);assert_eq!(source.as_ref().unwrap().capacity(),capacity);
        assert_eq!(serde_json::to_value(source.as_ref().unwrap()).unwrap(),fixture["values"]);
    }
    let(mut batch,progress)=MemberStoreOwnedBatch::admit(&mut source,grant).unwrap().unwrap();
    assert!(source.is_none()&&progress.fits(grant));
    assert_eq!(batch.mutations::<i32>().unwrap().as_ptr(),pointer);
    assert_eq!(serde_json::to_value(batch.mutations::<i32>().unwrap()).unwrap(),fixture["values"]);
    let close=RetainedCloneGrant{maximum_capacity_bytes:4096,..grant};
    for _ in 0..1024{if batch.terminal_is_empty(){break;}assert!(batch.close_granted(close).unwrap().progress().fits(close));}
    assert!(batch.terminal_is_empty());
    println!("[DEBUG] borrowed parent admission preserved original pointer/capacity/ordered serde source on zero and one-below grants before funded handoff");
}

#[test]
fn member_owned_batch_phase_queries_fund_actual_scalar_retirement_births() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let values: Vec<i32> = serde_json::from_value(fixture["values"].clone()).unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
    let (mut batch, _) = MemberStoreOwnedBatch::try_new(values, grant).unwrap_or_else(|_| panic!("funded scalar owner"));
    for _ in 0..1024 {
        if batch.terminal_is_empty() { break; }
        let demand = batch.next_demands(grant.maximum_copy_bytes).unwrap();
        let (capacity, release) = (demand.capacity_bytes, demand.release_bytes);
        let exact = RetainedCloneGrant { maximum_capacity_bytes: capacity, maximum_release_bytes: release.max(1), ..grant };
        let progress = batch.close_granted(exact).unwrap().progress();
        assert!(progress.fits(exact));
        assert!(progress.copied_items > 0 || progress.retained_capacity_bytes > 0 || progress.released_bytes > 0, "queried scalar phase must fund its real birth or release");
    }
    assert!(batch.terminal_is_empty());
    println!("[DEBUG] owned scalar source exact phase queries funded every real constructor and physical release");
}

#[test]
fn member_owned_batch_retains_exact_type_source_and_physical_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let source: Vec<i32> = serde_json::from_value(fixture["values"].clone()).unwrap();
    let pointer = source.as_ptr();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
    let (error, source) = match MemberStoreOwnedBatch::try_new(source, RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }) {
        Err(refused) => refused,
        Ok(_) => panic!("zero capacity cannot allocate the exact typed owner scaffold"),
    };
    assert_eq!(error.kind, ValueRefusalKind::AllocationFailed);
    assert_eq!(source.as_ptr(), pointer);
    let (mut batch, birth) = MemberStoreOwnedBatch::try_new(source, grant).unwrap_or_else(|_| panic!("explicit typed scalar retirement authority"));
    assert!(birth.fits(grant));
    assert!(batch.take_mutations::<u32>().is_none());
    assert_eq!(batch.mutations::<i32>().unwrap().as_ptr(), pointer);
    assert_eq!(serde_json::to_value(batch.mutations::<i32>().unwrap()).unwrap(), fixture["values"]);
    let source = batch.take_mutations::<i32>().unwrap();
    assert_eq!(source.as_ptr(), pointer);
    batch.restore_mutations(source);
    assert_eq!(batch.mutations::<i32>().unwrap().as_ptr(), pointer);
    let demand = batch.next_demands(grant.maximum_copy_bytes).unwrap();
        let (capacity, release) = (demand.capacity_bytes, demand.release_bytes);
    assert!(capacity > 0 && release == 0);
    assert_eq!(batch.close_granted(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap().progress(), RetainedCloneProgress::default());
    assert_eq!(batch.mutations::<i32>().unwrap().as_ptr(), pointer);
    let mut released = 0;
    for _ in 0..1000 {
        if batch.terminal_is_empty() { break; }
        let demand = batch.next_demands(grant.maximum_copy_bytes).unwrap();
        let (capacity, release) = (demand.capacity_bytes, demand.release_bytes);
        assert!(capacity <= grant.maximum_capacity_bytes && release <= grant.maximum_release_bytes);
        if batch.owner.as_ref().is_some_and(|owner| owner.terminal_is_empty()) {
            assert!(release > 0);
            let zero = batch.close_granted(RetainedCloneGrant { maximum_release_bytes: 0, ..grant }).unwrap();
            assert_eq!(zero.progress(), RetainedCloneProgress::default());
            let refused = batch.close_granted(RetainedCloneGrant { maximum_release_bytes: release - 1, ..grant }).unwrap();
            assert_eq!(refused.progress(), RetainedCloneProgress::default());
            assert_eq!(batch.next_demands(grant.maximum_copy_bytes).unwrap(), demand);
        }
        let step = batch.close_granted(grant).unwrap();
        assert!(step.progress().fits(grant));
        released += step.progress().released_bytes;
    }
    assert!(batch.terminal_is_empty());
    assert!(released >= birth.retained_capacity_bytes);
    assert_eq!(batch.next_demands(0).unwrap(), semio_framework_value::RetirementDemand::default());
    println!("[DEBUG] Owned member batch exact pointer/type/order retained across zero capacity, wrong type, restore, zero work and one-below physical release; born={} released={released}", birth.retained_capacity_bytes);
}

#[test]
fn member_owned_batch_constructor_refusals_preserve_exact_original_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["constructorRefusals"].as_array().unwrap() {
        let source: Vec<i32> = serde_json::from_value(fixture["values"].clone()).unwrap();
        let pointer = source.as_ptr();
        let capacity = source.capacity();
        let mut grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
        let value = row["value"].as_u64().unwrap() as usize;
        match row["axis"].as_str().unwrap() { "maximumDepth" => grant.maximum_depth = value, "maximumItems" => grant.maximum_items = value, "maximumCapacityBytes" => grant.maximum_capacity_bytes = value, _ => panic!("authored grant axis") }
        let (refusal, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| MemberStoreOwnedBatch::try_new(source, grant));
        let (error, source) = match refusal { Err(refusal) => refusal, Ok(_) => panic!("{}: denied constructor allocated an owner", row["id"]) };
        assert_eq!(format!("{:?}", error.kind), row["kind"].as_str().unwrap());
        assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
        assert_eq!(source.as_ptr(), pointer);
        assert_eq!(source.capacity(), capacity);
        assert_eq!(serde_json::to_value(&source).unwrap(), fixture["values"]);
    }
}

#[test]
fn member_owned_batch_child_depth_refusal_keeps_original_before_allocating_retirement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let values: Vec<i32> = serde_json::from_value(fixture["values"].clone()).unwrap();
    let pointer = values.as_ptr();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
    let (mut batch, _) = MemberStoreOwnedBatch::try_new(values, grant).unwrap_or_else(|_| panic!("admitted original batch"));
    let demand = batch.next_demands(grant.maximum_copy_bytes).unwrap();
    assert_eq!(demand.depth, 2);
    let (refusal, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| batch.close_granted(RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }));
    assert_eq!(refusal.unwrap_err().kind, ValueRefusalKind::DepthLimit);
    assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
    assert_eq!(batch.mutations::<i32>().unwrap().as_ptr(), pointer);
    assert_eq!(serde_json::to_value(batch.mutations::<i32>().unwrap()).unwrap(), fixture["values"]);
    for _ in 0..1024 {
        if batch.terminal_is_empty() { break; }
        let demand = batch.next_demands(grant.maximum_copy_bytes).unwrap();
        let exact = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth, ..grant };
        let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| batch.close_granted(exact).unwrap());
        assert!(step.progress().fits(exact));
        assert_eq!((events.requested_bytes, events.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    assert!(batch.terminal_is_empty());
}
