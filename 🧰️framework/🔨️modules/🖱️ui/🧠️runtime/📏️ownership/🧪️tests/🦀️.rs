use super::*;

//#region 📐️Fixtures
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap()
}

fn census(node: &crate::TreeNode) -> SurfaceSemanticUsage {
    let mut cursor = SurfaceSemanticCensusCursor::default();
    let mut result = SurfaceSemanticUsage::default();
    for _ in 0..10_000 {
        match cursor.step(node) {
            SurfaceSemanticCensusStep::Progress(progress) => {
                result.items = result.items.checked_add(progress.items).unwrap();
                result.bytes = result.bytes.checked_add(progress.bytes).unwrap();
            }
            SurfaceSemanticCensusStep::Complete => return result,
            SurfaceSemanticCensusStep::Fault(fault) => panic!("exact footprint fixture fault: {fault:?}"),
        }
    }
    panic!("bounded footprint fixture did not finish")
}

fn icon_node(icon: bool) -> crate::TreeNode {
    crate::TreeNode::try_new("machine", ui_contract::Component::TreeItem(ui_contract::TreeItemProps {
        label: ui_contract::Label::try_from("machine").unwrap(),
        description: None,
        icon: icon.then(|| ui_text("machine")),
        default_open: None,
        draggable: None,
        drag_data: None,
        dimmed: None,
        row_actions: Default::default(),
    })).unwrap()
}
//#endregion 📐️Fixtures

//#region 🧪️Laws
#[test]
fn surface_ownership_existing_component_refuses_before_cloning_unadmitted_payload() {
    let component: ui_contract::Component = serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":vec![17u8;32768]},"bindings":[]})).unwrap();
    let expected = serde_json::to_value(&component).unwrap();
    let node = crate::TreeNode::try_new("surface", component).unwrap();
    let id = ui_contract::UiNodeId(1);
    let mut current = SurfaceReconciler::new("existing-component-refusal");
    current.install_fixture_record(build_record_owned(id, leaf("surface"), Default::default(), None));
    let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
    cursor.stage = SurfaceReconcileStage::DiffRecords;
    cursor.record_diff = Some(RecordDiffCursor { id, record: build_record_owned(id, node, Default::default(), None).into(), field: 0, fresh: None, owned_copy: None });
    cursor.limits.max_bytes = 0;
    let step = cursor.step(&current);
    let rejected = matches!(step, SurfaceReconcileStep::Fault(SurfaceReconcileFault::Credits { .. }));
    let allocation = match cursor.pending_op.get() { Some(ui_contract::UiPatchOp::SetComponent { component: ui_contract::Component::Surface(props), .. }) => props.doc.bytes.as_slice().len(), _ => 0 };
    let source_unchanged = serde_json::to_value(&cursor.record_diff.as_ref().unwrap().record.component).unwrap() == expected;
    while !cursor.retire_one() {}
    while !current.retire_one() {}
    eprintln!("[DEBUG] existing-component-refusal rejected={rejected} allocation-before-admission={allocation} source-unchanged={source_unchanged}");
    assert!(rejected && source_unchanged);
    assert_eq!(allocation, fixture()["existingComponent"]["rejectionAllocationBytes"].as_u64().unwrap() as usize);
}

#[test]
fn surface_ownership_existing_component_retains_comparison_and_copy_between_turns() {
    let data = fixture()["existingComponent"].clone();
    let component = |last: u8| -> ui_contract::Component {
        let mut bytes = vec![17u8; data["payloadBytes"].as_u64().unwrap() as usize];
        *bytes.last_mut().unwrap() = last;
        serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":bytes},"bindings":[]})).unwrap()
    };
    let id = ui_contract::UiNodeId(1);
    let mut current = SurfaceReconciler::new("existing-component-copy");
    current.install_fixture_record(build_record_owned(id, crate::TreeNode::try_new("surface", component(18)).unwrap(), Default::default(), None));
    let before = serde_json::to_value(&current.read_record(id).unwrap().unwrap().component).unwrap();
    let expected = serde_json::to_value(component(19)).unwrap();
    let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
    cursor.stage = SurfaceReconcileStage::DiffRecords;
    cursor.record_diff = Some(RecordDiffCursor { id, record: build_record_owned(id, crate::TreeNode::try_new("surface", component(19)).unwrap(), Default::default(), None).into(), field: 0, fresh: None, owned_copy: None });
    let initial = cursor.usage.bytes;
    let mut turns = 0;
    for _ in 0..100_000 {
        let step = cursor.step(&current);
        assert!(matches!(step, SurfaceReconcileStep::Yield { .. }), "existing retained field failed: {step:?}");
        turns += 1;
        if cursor.record_diff.as_ref().unwrap().field != 0 { break; }
    }
    let allocation = cursor.usage.bytes - initial;
    let candidate = match cursor.pending_op.get().unwrap() { ui_contract::UiPatchOp::SetComponent { component, .. } => serde_json::to_value(component).unwrap(), _ => panic!("changed component patch missing") };
    let old_unchanged = serde_json::to_value(&current.read_record(id).unwrap().unwrap().component).unwrap() == before;
    while !cursor.retire_one() {}
    while !current.retire_one() {}
    assert_eq!(candidate, expected);
    assert!(old_unchanged);
    eprintln!("[DEBUG] existing-component-copy turns={turns} allocation-ledger={allocation} old-unchanged={old_unchanged}");
    assert!(turns > data["minimumTurns"].as_u64().unwrap());
    assert!(allocation >= data["payloadBytes"].as_u64().unwrap() as usize);
}

#[test]
fn surface_ownership_component_copy_charges_actual_surface_backing_before_publication() {
    let fixture = fixture();
    let data = &fixture["componentCopy"];
    let bytes = vec![17u8; data["payloadBytes"].as_u64().unwrap() as usize];
    let component: ui_contract::Component = serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":bytes},"bindings":[]})).unwrap();
    let expected = serde_json::to_value(&component).unwrap();
    let node = crate::TreeNode::try_new("surface", component).unwrap();
    let mut current = SurfaceReconciler::new("component-copy-fixture");
    let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
    cursor.stage = SurfaceReconcileStage::DiffRecords;
    cursor.record_diff = Some(RecordDiffCursor { id: ui_contract::UiNodeId(1), record: build_record_owned(ui_contract::UiNodeId(1), node, Default::default(), None).into(), field: 1, fresh: Some(FreshRecordClone::default()), owned_copy: None });
    let before = cursor.usage.bytes;
    let mut turns = 0;
    let mut reported = 0;
    for _ in 0..256 {
        let step = cursor.step(&current);
        let SurfaceReconcileStep::Yield { bytes, .. } = step else { panic!("component copy fault: {step:?}"); };
        assert!(bytes <= data["allocationGrant"].as_u64().unwrap() as usize);
        reported += bytes;
        turns += 1;
        if cursor.record_diff.as_ref().unwrap().field != 1 { break; }
    }
    let after = cursor.usage.bytes;
    let fresh = cursor.record_diff.as_ref().unwrap().fresh.as_ref().unwrap().component.as_ref().unwrap();
    assert_eq!(serde_json::to_value(fresh).unwrap(), expected);
    while !cursor.retire_one() {}
    while !current.retire_one() {}
    eprintln!("[DEBUG] surface-component-copy turns={turns} reported={reported} ledger-allocation={} actual-allocation=32768", after - before);
    assert_eq!(after - before, data["payloadBytes"].as_u64().unwrap() as usize);
    assert!(reported >= data["payloadBytes"].as_u64().unwrap() as usize);
    assert!(turns > 8);
}

#[test]
fn surface_ownership_binding_clone_requires_bounded_backing_and_copy() {
    let data = fixture();
    let data = &data["bindingClone"];
    let mut node = leaf("root");
    for index in 0..data["items"].as_u64().unwrap() { node = with_binding(node, "fixture", &format!("action-{index}")); }
    let mut current = SurfaceReconciler::new("allocation-fixture");
    let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
    cursor.stage = SurfaceReconcileStage::DiffRecords;
    cursor.record_diff = Some(RecordDiffCursor { id: ui_contract::UiNodeId(1), record: build_record_owned(ui_contract::UiNodeId(1), node, Default::default(), None).into(), field: 5, fresh: Some(FreshRecordClone::default()), owned_copy: None });
    let expected = serde_json::to_value(&cursor.record_diff.as_ref().unwrap().record.bindings).unwrap();
    let mut previous_allocated = 0;
    let mut previous_initialized = 0;
    let mut largest_allocation = 0;
    let mut largest_initialization = 0;
    let mut turns = 0;
    for _ in 0..1000 {
        let step = cursor.step(&current);
        let SurfaceReconcileStep::Yield { bytes, .. } = step else { panic!("binding copy fault: {step:?}"); };
        let diff = cursor.record_diff.as_ref().unwrap();
        let fresh = diff.fresh.as_ref().unwrap();
        let candidate = fresh.bindings.as_ref().or_else(|| diff.owned_copy.as_ref().and_then(RecordOwnedCopy::bindings).and_then(ui_contract::UiBindingsCopy::candidate));
        let allocated = candidate.map_or(0, ui_contract::UiNodeBindings::allocated_bytes);
        let initialized = candidate.map_or(0, |bindings| bindings.len() * size_of::<ui_contract::ActionBinding>());
        let allocation = allocated - previous_allocated;
        let initialization = initialized - previous_initialized;
        assert!(bytes >= allocation && bytes >= initialization, "actual backing and payload placement must be reported");
        assert!(bytes <= data["maximumBytesPerTurn"].as_u64().unwrap() as usize);
        largest_allocation = largest_allocation.max(allocation);
        largest_initialization = largest_initialization.max(initialization);
        previous_allocated = allocated;
        previous_initialized = initialized;
        turns += 1;
        if diff.field != 5 { break; }
    }
    let diff = cursor.record_diff.as_ref().unwrap();
    assert_eq!(diff.field, 6);
    assert_eq!(serde_json::to_value(diff.fresh.as_ref().unwrap().bindings.as_ref().unwrap()).unwrap(), expected);
    assert_eq!(serde_json::to_value(&diff.record.bindings).unwrap(), expected);
    while !cursor.retire_one() {}
    while !current.retire_one() {}
    eprintln!("[DEBUG] surface-binding-clone turns={turns} allocated={previous_allocated} initialized={previous_initialized} maximum-allocation={largest_allocation} maximum-placement={largest_initialization}");
    assert!(largest_allocation <= data["maximumBytesPerTurn"].as_u64().unwrap() as usize, "a retained field clone must pre-admit bounded backing");
    assert!(largest_initialization <= data["maximumBytesPerTurn"].as_u64().unwrap() as usize, "a field cannot copy all bindings as one bounded item");
    assert!(turns > 32);
}

#[test]
fn surface_ownership_component_copy_unwind_and_credit_refusal_keep_exact_source() {
    let data = fixture()["componentCopy"].clone();
    for frontier in data["cancelFrontiers"].as_array().unwrap() {
        let component: ui_contract::Component = serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":vec![17u8;32768]},"bindings":[]})).unwrap();
        let node = crate::TreeNode::try_new("surface", component).unwrap();
        let mut current = SurfaceReconciler::new("component-copy-fixture");
        let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
        cursor.stage = SurfaceReconcileStage::DiffRecords;
        cursor.record_diff = Some(RecordDiffCursor { id: ui_contract::UiNodeId(1), record: build_record_owned(ui_contract::UiNodeId(1), node, Default::default(), None).into(), field: 1, fresh: Some(FreshRecordClone::default()), owned_copy: None });
        for _ in 0..frontier.as_u64().unwrap() {
            if cursor.record_diff.as_ref().unwrap().field != 1 { break; }
            assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
        }
        if frontier.as_u64().unwrap() <= 12 {
            cursor.fail_component_step = true;
            let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cursor.step(&current))).is_err();
            cursor.fail_component_step = false;
            assert!(failed);
            let owner = cursor.record_diff.as_ref().unwrap().owned_copy.as_ref().and_then(RecordOwnedCopy::component).expect("callback keeps exact component owner");
            let ui_contract::Component::Surface(source) = owner.source().unwrap() else { panic!("retained source changed variant"); };
            assert_eq!(source.doc.bytes.as_slice(), vec![17u8;32768]);
        }
        let mut terminal = false;
        for _ in 0..100_000 { if cursor.retire_one() { terminal = true; break; } }
        assert!(terminal && cursor.record_diff.is_none());
        while !current.retire_one() {}
        eprintln!("[DEBUG] surface-component-unwind frontier={frontier} retained-outside-callback=true terminal-close=true");
    }
    let component: ui_contract::Component = serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":[1,2,3]},"bindings":[]})).unwrap();
    let node = crate::TreeNode::try_new("surface", component).unwrap();
    let mut current = SurfaceReconciler::new("component-refusal-fixture");
    let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
    cursor.stage = SurfaceReconcileStage::DiffRecords;
    cursor.record_diff = Some(RecordDiffCursor { id: ui_contract::UiNodeId(1), record: build_record_owned(ui_contract::UiNodeId(1), node, Default::default(), None).into(), field: 1, fresh: Some(FreshRecordClone::default()), owned_copy: None });
    for _ in 0..32 {
        if cursor.record_diff.as_ref().unwrap().owned_copy.as_ref().and_then(RecordOwnedCopy::component).is_some_and(|copy| copy.next_allocation_bytes() == Ok(32768)) { break; }
        cursor.step(&current);
    }
    cursor.limits.max_bytes = 0;
    assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Fault(SurfaceReconcileFault::Credits { .. })));
    assert_eq!(cursor.usage.bytes, 0);
    assert!(cursor.record_diff.as_ref().unwrap().owned_copy.as_ref().and_then(RecordOwnedCopy::component).unwrap().source().is_some());
    while !cursor.retire_one() {}
    while !current.retire_one() {}
    eprintln!("[DEBUG] surface-component-refusal actual-allocation=0 source-retained=true terminal=true");
}

#[test]
fn surface_ownership_patch_backing_is_admitted_in_separate_turns() {
    let mut current = SurfaceReconciler::new("allocation-fixture");
    let mut cursor = SurfaceReconcileCursor::new(tree(leaf("root")), &current);
    cursor.stage = SurfaceReconcileStage::Finalize;
    cursor.ids.try_push(ui_contract::UiNodeId(1)).unwrap();
    let mut largest = 0;
    let mut allocation_turns = 0;
    let mut complete = None;
    for _ in 0..100 {
        let before = cursor.ops.allocated_bytes();
        let outcome = cursor.step(&current);
        let after = match &outcome { SurfaceReconcileStep::Complete { patch: Some(patch), .. } => patch.ops.allocated_bytes(), _ => cursor.ops.allocated_bytes() };
        let allocated = after.saturating_sub(before);
        largest = largest.max(allocated);
        allocation_turns += usize::from(allocated != 0);
        match outcome {
            SurfaceReconcileStep::Yield { .. } => {}
            SurfaceReconcileStep::Complete { reconciler, patch } => { complete = Some((reconciler, patch)); break; }
            SurfaceReconcileStep::Fault(fault) => panic!("patch allocation fixture fault: {fault:?}"),
        }
    }
    let (mut candidate, mut patch) = complete.expect("bounded patch publication");
    assert_eq!(patch.as_ref().unwrap().ops.len(), 1);
    while !cursor.retire_one() {}
    while !candidate.retire_one() {}
    while !current.retire_one() {}
    if let Some(patch) = patch.as_mut() { while !patch.ops.terminal_is_empty() { patch.ops.close_step(1, 4096).unwrap(); } }
    eprintln!("[DEBUG] surface-patch-allocation turns={allocation_turns} largest={largest} operation-bytes={}", size_of::<ui_contract::UiPatchOp>());
    assert!(largest <= fixture()["patchAllocation"]["maximumBytesPerTurn"].as_u64().unwrap() as usize, "directory and payload allocations require distinct admitted opportunities");
    assert_eq!(allocation_turns, 2);
}

#[test]
fn surface_ownership_binding_copy_cancel_keeps_all_original_and_partial_backings() {
    for frontier in fixture()["bindingClone"]["cancelFrontiers"].as_array().unwrap() {
        let frontier = frontier.as_u64().unwrap();
        let mut node = leaf("root");
        for index in 0..32 { node = with_binding(node, "fixture", &format!("action-{index}")); }
        let mut current = SurfaceReconciler::new("allocation-fixture");
        let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
        cursor.stage = SurfaceReconcileStage::DiffRecords;
        cursor.record_diff = Some(RecordDiffCursor { id: ui_contract::UiNodeId(1), record: build_record_owned(ui_contract::UiNodeId(1), node, Default::default(), None).into(), field: 5, fresh: Some(FreshRecordClone::default()), owned_copy: None });
        let bytes = |cursor: &SurfaceReconcileCursor| cursor.record_diff.as_ref().map_or(0, |diff| {
            diff.record.bindings.allocated_bytes()
                + diff.fresh.as_ref().and_then(|fresh| fresh.bindings.as_ref()).map_or(0, ui_contract::UiNodeBindings::allocated_bytes)
                + diff.owned_copy.as_ref().and_then(RecordOwnedCopy::bindings).map_or(0, |copy| copy.source_allocated_bytes() + copy.candidate_allocated_bytes())
        });
        for _ in 0..frontier { assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. })); }
        let retained = bytes(&cursor);
        let mut previous = retained;
        let mut terminal = false;
        for _ in 0..100_000 {
            terminal = cursor.retire_one();
            let current = bytes(&cursor);
            assert!(current <= previous, "cancel cannot replace a source or candidate with fresh backing");
            assert!(previous - current <= SURFACE_RECONCILE_PAGE_BYTES, "one close turn must release one admitted page: frontier={frontier} before={previous} after={current}");
            previous = current;
            if terminal { break; }
        }
        assert!(terminal && cursor.record_diff.is_none());
        while !current.retire_one() {}
        eprintln!("[DEBUG] surface-binding-cancel frontier={frontier} retained={retained} terminal=true allocation-during-close=0");
    }
}

#[test]
fn surface_ownership_binding_copy_unwind_keeps_owners_outside_callback() {
    for frontier in 1..=8 {
        let mut node = leaf("root");
        for index in 0..32 { node = with_binding(node, "fixture", &format!("action-{index}")); }
        let mut current = SurfaceReconciler::new("allocation-fixture");
        let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
        cursor.stage = SurfaceReconcileStage::DiffRecords;
        cursor.record_diff = Some(RecordDiffCursor { id: ui_contract::UiNodeId(1), record: build_record_owned(ui_contract::UiNodeId(1), node, Default::default(), None).into(), field: 5, fresh: Some(FreshRecordClone::default()), owned_copy: None });
        for _ in 0..frontier { cursor.step(&current); }
        let before = cursor.record_diff.as_ref().unwrap().owned_copy.as_ref().and_then(RecordOwnedCopy::bindings).unwrap() as *const ui_contract::UiBindingsCopy;
        cursor.fail_binding_step = true;
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cursor.step(&current))).is_err();
        cursor.fail_binding_step = false;
        assert!(failed);
        assert_eq!(cursor.record_diff.is_some(), fixture()["bindingClone"]["unwindRetainsRoots"].as_bool().unwrap());
        let owner = cursor.record_diff.as_ref().unwrap().owned_copy.as_ref().and_then(RecordOwnedCopy::bindings).unwrap();
        assert_eq!(owner as *const ui_contract::UiBindingsCopy, before);
        assert!(owner.source_allocated_bytes() != 0);
        while !cursor.retire_one() {}
        while !current.retire_one() {}
        eprintln!("[DEBUG] surface-binding-unwind frontier={frontier} same-owner=true terminal-close=true");
    }
}

#[test]
fn surface_ownership_transfer_preserves_backing_without_allocating_replacement() {
    let fixture = fixture();
    let case = &fixture["transfer"];
    let mut source = SurfaceFixedVec::<u64, 4>::default();
    for value in case["values"].as_array().unwrap() { source.try_push(value.as_u64().unwrap()).unwrap(); }
    let original = source.entries.as_ptr();
    let original_bytes = std::mem::size_of_val(source.entries.as_ref());
    let mut moved = source.take_all();
    assert_eq!(moved.entries.as_ptr(), original);
    assert_eq!(std::mem::size_of_val(moved.entries.as_ref()), original_bytes);
    assert_eq!(serde_json::to_value(moved.iter().copied().collect::<Vec<_>>()).unwrap(), case["values"]);
    assert_eq!(source.len(), case["sourceItems"].as_u64().unwrap() as usize);
    let source_bytes = std::mem::size_of_val(source.entries.as_ref());
    while moved.pop().is_some() {}
    eprintln!("[DEBUG] surface-backing-transfer source-bytes={source_bytes} moved-bytes={original_bytes}");
    assert_eq!(source_bytes, case["sourceBackingBytes"].as_u64().unwrap() as usize, "a moved-from owner must not allocate a replacement full backing");
    let mut source = SurfaceFixedVec::<String, 4>::default();
    let moved = source.take_all();
    let payload = case["rejectedPayload"].as_str().unwrap().to_owned();
    let pointer = payload.as_ptr();
    let capacity = payload.capacity();
    let returned = source.try_push(payload).expect_err("moved source has no admitted backing");
    assert_eq!(returned, case["rejectedPayload"].as_str().unwrap());
    assert_eq!(returned.as_ptr(), pointer);
    assert_eq!(returned.capacity(), capacity);
    assert_eq!(std::mem::size_of_val(source.entries.as_ref()), 0);
    assert!(source.is_empty());
    assert!(moved.is_empty());
    eprintln!("[DEBUG] surface-moved-source rejected-exact-payload=true payload-capacity={capacity} replacement-bytes=0");
}

#[test]
fn surface_ownership_patch_refusal_and_cancel_keep_exact_unallocated_owner() {
    for turns in 1..=4 {
        let mut current = SurfaceReconciler::new("allocation-fixture");
        let mut cursor = SurfaceReconcileCursor::new(tree(leaf("root")), &current);
        cursor.stage = SurfaceReconcileStage::Finalize;
        cursor.ids.try_push(ui_contract::UiNodeId(1)).unwrap();
        for _ in 0..turns { assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. })); }
        let allocated = cursor.ops.allocated_bytes();
        if turns == 1 {
            cursor.limits.max_bytes = 0;
            assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Fault(SurfaceReconcileFault::Credits { .. })));
            assert_eq!(cursor.ops.allocated_bytes(), 0);
            assert!(matches!(cursor.pending_op.get(), Some(ui_contract::UiPatchOp::SetRoot { id: ui_contract::UiNodeId(1) })));
        }
        let mut closed = false;
        for _ in 0..10_000 {
            if cursor.retire_one() { closed = true; break; }
            assert!(cursor.ops.allocated_bytes() <= allocated, "cancellation cannot allocate a retirement page");
        }
        assert!(closed);
        assert!(cursor.pending_op.terminal_is_empty());
        assert!(cursor.ops.terminal_is_empty());
        while !current.retire_one() {}
        eprintln!("[DEBUG] surface-patch-cancel stage={turns} retained-before={allocated} terminal=true allocation-during-close=0");
    }
}

#[test]
fn surface_ownership_finalize_transfers_exact_record_and_index_allocations() {
    let mut current = SurfaceReconciler::new("allocation-fixture");
    let mut cursor = SurfaceReconcileCursor::new(tree(leaf("root")), &current);
    let mut records = None;
    let indexes = cursor.new_key_index.entries.entries.as_ptr();
    let mut complete = None;
    for _ in 0..10_000 {
        if cursor.stage == SurfaceReconcileStage::Finalize { records = cursor.assembly.root_identity(); }
        match cursor.step(&current) {
            SurfaceReconcileStep::Yield { .. } => {}
            SurfaceReconcileStep::Complete { reconciler, patch } => { complete = Some((reconciler, patch)); break; }
            SurfaceReconcileStep::Fault(fault) => panic!("exact transfer fixture fault: {fault:?}"),
        }
    }
    let (mut reconciler, mut patch) = complete.expect("bounded finalize");
    assert!(records.is_some());
    assert_eq!(reconciler.assembly.root_identity(), records);
    assert_eq!(reconciler.key_index.entries.entries.as_ptr(), indexes);
    assert!(cursor.assembly.terminal_is_empty());
    assert_eq!(std::mem::size_of_val(cursor.new_key_index.entries.entries.as_ref()), 0);
    let mut complete = false;
    for _ in 0..10_000 { if cursor.retire_one() { complete = true; break; } }
    assert!(complete);
    let mut complete = false;
    for _ in 0..10_000 { if reconciler.retire_one() { complete = true; break; } }
    assert!(complete);
    let mut complete = false;
    for _ in 0..10_000 { if current.retire_one() { complete = true; break; } }
    assert!(complete);
    if let Some(patch) = patch.as_mut() { while patch.ops.pop().is_some() {} patch.ops.release_empty_allocation().unwrap(); }
    eprintln!("[DEBUG] surface-finalize-transfer exact-records=true exact-indexes=true replacement-bytes=0 closed=true");
}

#[test]
fn surface_ownership_inline_fields_do_not_allocate_a_second_owner() {
    let fixture = fixture();
    let mut differences = Vec::new();
    let mut expected = Vec::new();
    for case in fixture["inlineCases"].as_array().unwrap() {
        let (before, after) = match case["name"].as_str().unwrap() {
            "tree-item-icon" => (icon_node(false), icon_node(true)),
            "reserved-binding" => (with_binding(leaf("machine"), "fixture", "first"), with_binding(with_binding(leaf("machine"), "fixture", "first"), "fixture", "second")),
            _ => unreachable!(),
        };
        assert_eq!(before.bindings.capacity(), after.bindings.capacity());
        assert_eq!(before.bindings.capacity(), case["backingCapacity"].as_u64().unwrap() as usize);
        let before_usage = census(&before);
        let after_usage = census(&after);
        let difference = after_usage.bytes.checked_sub(before_usage.bytes).unwrap();
        eprintln!("[DEBUG] surface-inline-footprint name={} before={} after={} delta={} items-before={} items-after={}", case["name"], before_usage.bytes, after_usage.bytes, difference, before_usage.items, after_usage.items);
        differences.push(difference);
        expected.push(case["additionalOwnedBytes"].as_u64().unwrap() as usize);
        assert!(after_usage.items >= before_usage.items, "additional traversal work remains accounted independently");
    }
    assert_eq!(differences, expected, "inline bytes already belong to the enclosing allocated payload");
}

#[test]
fn surface_ownership_native_backing_inventory_preserves_capacity() {
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
    for (owner, bytes) in sizes { eprintln!("[DEBUG] surface-physical-owner owner={owner} bytes={bytes}"); }
}
#[test]
fn surface_ownership_resident_reservation_uses_one_shared_aggregate_ledger() {
    let data: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️contract/🎟️resident/🧫️fixture/🔣️.json")).unwrap();
    assert!(register_surface_reconcile_backing(32768).unwrap());
    let before = ui_contract::UiResidentPermit::snapshot().unwrap();
    let bytes = data["smallBytes"].as_u64().unwrap() as usize;
    let limits = SurfaceReconcileLimits { max_bytes: bytes, ..Default::default() };
    let credit = reserve_surface_reconcile(limits).expect("one small runtime reservation");
    let during = ui_contract::UiResidentPermit::snapshot().unwrap();
    release_surface_reconcile(credit);
    let after = ui_contract::UiResidentPermit::snapshot().unwrap();
    assert_eq!(after, before);
    eprintln!("[DEBUG] runtime-resident-join expected-bytes={bytes} observed-bytes={} expected-slots=1 observed-slots={}", during.bytes - before.bytes, during.used_slots - before.used_slots);
    assert_eq!(during.bytes - before.bytes, bytes, "the runtime must not retain an independent second aggregate ledger");
    assert_eq!(during.used_slots - before.used_slots, 1);
}

#[test]
fn surface_ownership_resident_return_maintenance_preserves_contended_credit() {
    assert!(register_surface_reconcile_backing(32768).unwrap());
    let before = ui_contract::UiResidentPermit::snapshot().unwrap();
    let credit = reserve_surface_reconcile(SurfaceReconcileLimits { max_bytes: 65536, ..Default::default() }).unwrap();
    let observed = ui_contract::UiResidentPermit::try_observe().unwrap();
    assert!(observed.owns(&credit));
    release_surface_reconcile(credit);
    assert!(ui_contract::UiResidentPermit::has_pending_returns());
    for _ in 0..128 { assert!(!close_surface_reconcile_handback_one().unwrap()); }
    assert_eq!(observed.snapshot().bytes, before.bytes + 65536);
    drop(observed);
    let mut complete = false;
    for _ in 0..256 { if close_surface_reconcile_handback_one().unwrap() { complete = true; break; } }
    assert!(complete);
    assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), before);
    eprintln!("[DEBUG] runtime-resident-return mutex-busy-keeps-credit=true maintenance-resumes=true exact-return=65536");
}
//#endregion 🧪️Laws
