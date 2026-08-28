use crate::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixture.json")).unwrap() }

#[test]
fn instance_lifetime_ui_patch_pending_cancel_never_allocates_a_payload_page() {
    let fixture = fixture();
    for grant in [1, 64, 4096] {
        let mut owner = UiPendingPatchOp::default();
        *owner.source_mut().unwrap() = Some(serde_json::from_value(serde_json::json!({"type":"setComponent","id":7,"component":{"type":"text","value":"é".repeat(256)}})).unwrap());
        assert_eq!(owner.allocated_bytes(), fixture["unplaced"]["allocationBytes"].as_u64().unwrap() as usize);
        assert!(!owner.close_step(0, grant).unwrap().progressed);
        assert!(owner.get().is_some());
        let mut bytes = 0;
        for _ in 0..2000 {
            let step = owner.close_step(1, grant).unwrap();
            bytes += step.released_bytes;
            assert!(step.released_items <= 1 && step.released_bytes <= grant);
            assert_eq!(owner.get().is_some(), fixture["unplaced"]["readsAfterClose"].as_bool().unwrap());
            assert_eq!(owner.source_mut().is_ok(), fixture["unplaced"]["sourceReplacementAfterClose"].as_bool().unwrap());
            assert_eq!(owner.allocated_bytes(), 0);
            if step.complete { break; }
        }
        assert!(owner.terminal_is_empty());
        assert_eq!(bytes, fixture["unplaced"]["semanticBytes"].as_u64().unwrap() as usize);
    }
}

#[test]
fn instance_lifetime_ui_patch_pending_placement_requires_full_inline_grant() {
    let mut owner = UiPendingPatchOp::default();
    *owner.source_mut().unwrap() = Some(UiPatchOp::SetRoot { id: UiNodeId(7) });
    let mut target = UiPatchOps::default();
    assert_eq!(owner.place_into(&mut target, 32768), Ok(0));
    while !target.has_reserved_slot() { target.try_reserve_one(target.next_allocation_bytes().unwrap()).unwrap(); }
    assert_eq!(owner.place_into(&mut target, 4096), Ok(0));
    assert!(owner.get().is_some());
    assert_eq!(owner.place_into(&mut target, std::mem::size_of::<UiPatchOp>()), Ok(std::mem::size_of::<UiPatchOp>()));
    assert!(owner.terminal_is_empty());
    assert!(owner.close_step(1, 1).unwrap().complete);
    for _ in 0..100 { if target.close_step(1, 1).unwrap().complete { break; } }
    assert!(target.terminal_is_empty());
}

#[test]
fn instance_lifetime_ui_patch_storage_first_payload_does_not_reserve_logical_capacity() {
    let fixture = fixture();
    let mut operations = UiPatchOps::default();
    operations.try_push(UiPatchOp::SetRoot { id: UiNodeId(7) }).unwrap();
    let allocated = operations.allocated_bytes();
    let maximum = UI_DOCUMENT_PATCH_OPS * std::mem::size_of::<Vec<UiPatchOp>>() + std::mem::size_of::<UiPatchOp>();
    if cfg!(target_pointer_width = "64") {
        assert_eq!(std::mem::size_of::<UiPatchOp>(), fixture["native64"]["operationBytes"].as_u64().unwrap() as usize);
        assert_eq!(maximum, fixture["native64"]["firstBackingBytes"].as_u64().unwrap() as usize);
    }
    assert!(allocated <= maximum, "first initialized patch must own only directory plus one payload page: allocated={allocated}, maximum={maximum}");
}

#[test]
fn instance_lifetime_ui_patch_storage_reservation_placement_and_cancel_preserve_exact_owners() {
    let fixture = fixture();
    for grant in fixture["placementGrants"].as_array().unwrap() {
        let grant = grant.as_u64().unwrap() as usize;
        let mut operations = UiPatchOps::default();
        let directory = operations.next_allocation_bytes().unwrap();
        assert_eq!(operations.try_reserve_one(directory - 1), Ok(0));
        assert_eq!(operations.allocated_bytes(), 0);
        assert_eq!(operations.try_reserve_one(directory), Ok(directory));
        assert!(!operations.has_reserved_slot());
        let page = operations.next_allocation_bytes().unwrap();
        assert_eq!(operations.try_reserve_one(page - 1), Ok(0));
        assert_eq!(operations.allocated_bytes(), directory);
        assert_eq!(operations.try_reserve_one(page), Ok(page));
        assert_eq!(operations.allocated_bytes(), directory + page);
        assert!(operations.is_empty());
        assert!(!operations.terminal_is_empty());
        let mut source = Some(UiPatchOp::SetRoot { id: UiNodeId(7) });
        let placed = operations.try_push_reserved(&mut source, grant).unwrap();
        assert_eq!(placed, if grant >= std::mem::size_of::<UiPatchOp>() { std::mem::size_of::<UiPatchOp>() } else { 0 });
        assert_eq!(source.is_some(), placed == 0);
        if placed == 0 { assert_eq!(operations.try_push_reserved(&mut source, page), Ok(page)); }
        let payload = operations.get(0).unwrap() as *const UiPatchOp;
        let before = operations.allocated_bytes();
        let mut moved = operations.take_all();
        assert!(operations.terminal_is_empty());
        assert_eq!(operations.allocated_bytes(), fixture["sourceAfterHandoffBytes"].as_u64().unwrap() as usize);
        assert_eq!(moved.get(0).unwrap() as *const UiPatchOp, payload);
        assert_eq!(moved.allocated_bytes(), before);
        let unchanged = moved.allocated_bytes();
        assert!(!moved.close_step(0, 4096).unwrap().progressed);
        assert!(!moved.close_step(1, 0).unwrap().progressed);
        assert_eq!(moved.allocated_bytes(), unchanged);
        for _ in 0..100 { if moved.close_step(1, 1).unwrap().complete { break; } }
        assert!(moved.terminal_is_empty());
        assert_eq!(moved.allocated_bytes(), 0);
    }
}

#[test]
fn instance_lifetime_ui_patch_storage_wire_oracle_and_full_capacity_are_unchanged() {
    let fixture = fixture();
    let mut operations: UiPatchOps = serde_json::from_value(fixture["operations"].clone()).unwrap();
    assert_eq!(serde_json::to_value(&operations).unwrap(), fixture["operations"]);
    let mut order = Vec::new();
    while let Some(operation) = operations.pop() {
        order.push(match operation { UiPatchOp::SetRoot { id } | UiPatchOp::Remove { id } => id.0, _ => panic!("unexpected oracle operation") });
    }
    operations.release_empty_allocation().unwrap();
    assert_eq!(serde_json::to_value(order).unwrap(), fixture["retirementOrder"]);
    for index in 0..UI_DOCUMENT_PATCH_OPS { operations.try_push(UiPatchOp::SetRoot { id: UiNodeId(index as u64) }).unwrap(); }
    let retained = operations.allocated_bytes();
    let mut rejected = Some(UiPatchOp::SetRoot { id: UiNodeId(u64::MAX) });
    assert!(operations.next_allocation_bytes().is_err());
    assert!(operations.try_push_reserved(&mut rejected, 32768).is_err());
    assert!(rejected.is_some());
    assert_eq!(operations.allocated_bytes(), retained);
    for _ in 0..UI_DOCUMENT_PATCH_OPS * 8 { if operations.close_step(1, 1).unwrap().complete { break; } }
    assert!(operations.terminal_is_empty());
}

#[test]
fn instance_lifetime_ui_patch_storage_typed_unicode_close_is_in_place_and_resumable() {
    let fixture = fixture();
    for grant in fixture["semanticGrants"].as_array().unwrap().iter().skip(1) {
        let grant = grant.as_u64().unwrap() as usize;
        let mut operations: UiPatchOps = serde_json::from_value(serde_json::json!([{
            "type": "setComponent", "id": 7,
            "component": {"type": "text", "value": "é".repeat(256)}
        }])).unwrap();
        let pointer = operations.get(0).unwrap() as *const UiPatchOp;
        let mut total = 0;
        for turn in 0..2000 {
            let step = operations.close_step(1, grant).unwrap();
            assert!(step.released_items <= 1 && step.released_bytes <= grant);
            total += step.released_bytes;
            assert!(operations.get(0).is_none());
            if turn == 2 {
                let mut moved = operations.take_all();
                assert!(operations.terminal_is_empty());
                operations = moved.take_all();
                assert!(moved.terminal_is_empty());
            }
            if step.complete { break; }
        }
        assert_eq!(total, 512);
        assert!(operations.terminal_is_empty());
        assert_ne!(pointer as usize, 0);
    }
}
