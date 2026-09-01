use super::*;

//#region 🧪️WholePatchRetirement
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture.json")).unwrap() }
fn make(ops: UiPatchOps) -> UiPendingPatch {
    let fixture = fixture();
    let mut owner = UiPendingPatch::default();
    *owner.source_mut().unwrap() = Some(UiPatch { surface: SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap(), base_revision: UiRevision(7), revision: UiRevision(8), ops });
    owner
}
fn close(owner: &mut UiPendingPatch, grant: usize) -> usize {
    let mut bytes = 0;
    for _ in 0..500_000 {
        let before = owner.retained_operation_bytes();
        let step = owner.close_step(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        assert!(owner.retained_operation_bytes() <= before);
        bytes += step.released_bytes;
        if step.complete { assert!(owner.terminal_is_empty()); assert_eq!(owner.retained_operation_bytes(), 0); return bytes; }
    }
    panic!("whole patch did not retire");
}

#[test]
fn retained_pending_patch_closes_all_native_component_payloads_and_exact_surface_bytes() {
    let fixture = fixture();
    let components: serde_json::Value = serde_json::from_str(include_str!("../../../../🌳️typed/🧪️components.json")).unwrap();
    for row in components["cases"].as_array().unwrap() {
        for grant in fixture["grants"].as_array().unwrap() {
            let mut ops = UiPatchOps::default();
            ops.try_push(UiPatchOp::SetComponent { id: UiNodeId(41), component: serde_json::from_value(row["component"].clone()).unwrap() }).unwrap();
            let mut owner = make(ops);
            let before = serde_json::to_value(owner.get().unwrap()).unwrap();
            assert_eq!(before["surface"], fixture["surface"]);
            assert!(!owner.close_step(0, 4096).unwrap().progressed);
            assert!(!owner.close_step(1, 0).unwrap().progressed);
            assert_eq!(serde_json::to_value(owner.get().unwrap()).unwrap(), before);
            assert_eq!(close(&mut owner, grant.as_u64().unwrap() as usize), fixture["surfaceBytes"].as_u64().unwrap() as usize + row["bytes"].as_u64().unwrap() as usize);
            assert!(owner.get().is_none() && owner.source_mut().is_err());
        }
    }
    eprintln!("[DEBUG] pending-whole-patch components=18 close-grants=1,64,4096 exact-surface-bytes=4 no-close-allocation=true");
}

#[test]
fn retained_pending_patch_keeps_empty_backing_and_partial_owner_through_unwind() {
    let fixture = fixture();
    for frontier in fixture["frontiers"].as_array().unwrap() {
        let mut ops = UiPatchOps::default();
        while !ops.has_reserved_slot() { let request = ops.next_allocation_bytes().unwrap(); ops.try_reserve_one(request).unwrap(); }
        let backing = ops.allocated_bytes();
        assert!(backing > 0 && ops.is_empty());
        let mut owner = make(ops);
        assert_eq!(owner.retained_operation_bytes(), backing);
        let mut retired = 0;
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for _ in 0..frontier.as_u64().unwrap() { retired += owner.close_step(1, 1).unwrap().released_bytes; }
            panic!("fixture producer failure with owner retained outside callback");
        }));
        assert!(panic.is_err());
        retired += close(&mut owner, 1);
        assert_eq!(retired, 4);
    }
    let mut owner = make(UiPatchOps::default());
    let exact = owner.source_mut().unwrap().take().unwrap();
    assert!(owner.terminal_is_empty());
    *owner.source_mut().unwrap() = Some(exact);
    assert_eq!(close(&mut owner, 64), 4);
    eprintln!("[DEBUG] pending-whole-patch unwind-frontiers=6 empty-reserved-backing-retired=true exact-handoff=true owner-bytes={}", std::mem::size_of::<UiPendingPatch>());
}
//#endregion 🧪️WholePatchRetirement
