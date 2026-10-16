use super::*;

//#region 📐️Fixtures
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}
//#endregion 📐️Fixtures
//#region 🧪️Laws
#[test]
fn surface_ownership_native_backing_inventory_preserves_capacity() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    for case in fixture()["backingCases"].as_array().unwrap() {
        let capacity = case["capacity"].as_u64().unwrap() as usize;
        let element_bytes = case["elementBytes"].as_u64().unwrap() as usize;
        let allocation = std::alloc::Layout::array::<u8>(capacity * element_bytes).unwrap();
        assert_eq!(allocation.size(), case["ownedBytes"].as_u64().unwrap() as usize);
        assert!(case["initialized"].as_u64().unwrap() as usize <= capacity);
    }
    let sizes = [
        ("tree-node", size_of::<crate::TreeNode>()),
        ("record", size_of::<ui_contract::UiNodeRecord>()),
        ("patch-op", size_of::<ui_contract::UiPatchOp>()),
        ("action-binding", size_of::<ui_contract::ActionBinding>()),
        ("row-action", size_of::<ui_contract::RowAction>()),
        ("flat-backing", SURFACE_RECONCILE_FIXED_NODES * size_of::<Option<FlatPresentedNode>>()),
        ("retained-backing", SURFACE_RECONCILE_FIXED_NODES * size_of::<Option<(ui_contract::UiNodeId, ui_contract::UiNodeRecord)>>()),
        ("key-index-backing", SURFACE_RECONCILE_FIXED_NODES * size_of::<Option<(NodeIdentity, ui_contract::UiNodeId)>>()),
        ("traversal-backing", SURFACE_RECONCILE_VALUE_DEPTH * size_of::<Option<PresentationFrame>>()),
        ("postorder-backing", SURFACE_RECONCILE_FIXED_NODES * size_of::<Option<usize>>()),
        ("seen-backing", SURFACE_RECONCILE_FIXED_NODES * size_of::<Option<(Option<usize>, ui_contract::UiText)>>()),
        ("ids-backing", SURFACE_RECONCILE_FIXED_NODES * size_of::<Option<ui_contract::UiNodeId>>()),
        ("removal-backing", SURFACE_RECONCILE_FIXED_NODES * size_of::<Option<RemovalFrame>>()),
        ("semantic-value-stack", SURFACE_RECONCILE_VALUE_DEPTH * size_of::<Option<SurfaceSemanticValueFrame>>()),
        ("lazy-tree-retirement-stack", ui_contract::UI_BUILT_CHILD_RETIRE_SLOTS * size_of::<Option<ui_contract::BuiltChildrenIntoIter>>()),
        ("hypothetical-inline-patch-op-backing", SURFACE_RECONCILE_FIXED_OPS * size_of::<ui_contract::UiPatchOp>()),
        ("patch-directory-backing", SURFACE_RECONCILE_FIXED_OPS * size_of::<Vec<ui_contract::UiPatchOp>>()),
        ("patch-first-payload-backing", size_of::<ui_contract::UiPatchOp>()),
        ("generic-list-owner", size_of::<ui_contract::UiNodeBindings>()),
        ("binding-copy-owner", size_of::<ui_contract::UiBindingsCopy>()),
        ("pending-patch-owner", size_of::<ui_contract::UiPendingPatchOp>()),
        ("cursor", size_of::<SurfaceReconcileCursor>()),
        ("retained-job-allocation", size_of::<SurfaceReconcileRetained>()),
        ("reconciler", size_of::<SurfaceReconciler>()),
    ];
    for (owner, bytes) in sizes {
        eprintln!("surface-physical-owner owner={owner} bytes={bytes}");
    }
}
#[test]
fn surface_ownership_resident_reservation_uses_one_shared_aggregate_ledger() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let data: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️contract/🎟️resident/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    assert!(register_surface_reconcile_backing(32768).unwrap());
    let before = ui_contract::UiResidentPermit::snapshot().unwrap();
    let bytes = data["smallBytes"].as_u64().unwrap() as usize;
    let limits = SurfaceReconcileLimits { max_bytes: bytes, ..Default::default() };
    let credit = reserve_surface_reconcile(limits).expect("one small runtime reservation");
    let during = ui_contract::UiResidentPermit::snapshot().unwrap();
    release_surface_reconcile(credit);
    let after = ui_contract::UiResidentPermit::snapshot().unwrap();
    assert_eq!(after, before);
    assert_eq!(during.bytes - before.bytes, bytes, "the runtime must not retain an independent second aggregate ledger");
    assert_eq!(during.used_slots - before.used_slots, 1);
}
#[test]
#[ignore = "re-entrant resident observe plus handback drain deadlocks in-suite; run via runExactCargoLaws"]
fn surface_ownership_resident_return_maintenance_preserves_contended_credit() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    assert!(register_surface_reconcile_backing(32768).unwrap());
    let before = ui_contract::UiResidentPermit::snapshot().unwrap();
    let credit = reserve_surface_reconcile(SurfaceReconcileLimits { max_bytes: 65536, ..Default::default() }).unwrap();
    let observed = ui_contract::UiResidentPermit::try_observe().unwrap();
    assert!(observed.owns(&credit));
    release_surface_reconcile(credit);
    assert!(ui_contract::UiResidentPermit::has_pending_returns());
    for _ in 0..128 {
        assert!(!close_surface_reconcile_handback_one().unwrap());
    }
    assert_eq!(observed.snapshot().bytes, before.bytes + 65536);
    drop(observed);
    let mut complete = false;
    for _ in 0..256 {
        if close_surface_reconcile_handback_one().unwrap() {
            complete = true;
            break;
        }
    }
    assert!(complete);
    assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), before);
}
//#endregion 🧪️Laws
