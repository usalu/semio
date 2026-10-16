use super::*;

//#region 🧪️CanonicalRootLaws
fn document_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}
/// 📏️ The most a SINGLE lease-close turn may release. `close_document_owner` widens its own grant to
/// the retired document's node-page allocation
/// (`retire_exact`: `grant = maximum_bytes.max(slot.nodes.entries.allocated_bytes())`), so a close
/// turn is never a partial slice borrowed from the child's budget — it is at least one whole
/// [`SURFACE_COMPONENT_COPY_WORK_BYTES`] page, and at most the one page a `PageBytes` fault admits.
///
/// 🐛️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING packet P5: this law used to pin the frozen
/// literal `4096`. `TreeItemProps` gaining `window`/`granularity` (§3) grew
/// `size_of::<ui_contract::Component>()` 2568 → 3096 and with it
/// `size_of::<ui_contract::UiNodeRecord>()` to 6416, which pushed one node page past the 4 KiB work
/// grant — the measured close turns became `7568, 7568, 7568, 7568, 2496` (32768 B of surface payload)
/// and the frozen equality could never hold again. The law is therefore a RANGE derived from the two
/// constants the implementation itself uses, so it holds for any `Component` size.
fn document_close_turn_ceiling() -> usize {
    SURFACE_RECONCILE_PAGE_BYTES
}
#[test]
fn surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant() {
    let _guard = registry_guard();
    let fixture = document_fixture();
    assert!(fixture["completionTransferUsesSeparateGrant"].as_bool().unwrap());
    let component = |last| {
        let mut bytes = vec![17u8; 32768];
        *bytes.last_mut().unwrap() = last;
        serde_json::from_value::<ui_contract::Component>(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":bytes},"bindings":[]})).unwrap()
    };
    let id = ui_contract::UiNodeId(1);
    let mut current = SurfaceShadow::from_text("completion-grant");
    current.install_fixture_record(build_record_owned(id, crate::TreeNode::try_new("surface", component(18)).unwrap(), Default::default(), None));
    let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
    cursor.stage = SurfaceReconcileStage::DiffRecords;
    cursor.record_diff = Some(RecordDiffCursor { id, record: build_record_owned(id, crate::TreeNode::try_new("surface", component(19)).unwrap(), Default::default(), None).into(), field: 0, fresh: None, owned_copy: None });
    let incoming = &cursor.record_diff.as_ref().unwrap().record.component as *const _;
    let mut comparison_completed = false;
    let mut copy_completed = false;
    let mut source_returned = false;
    let mut candidate_returned = false;
    let mut old_retired = false;
    let mut full_close_grant = false;
    for _ in 0..100_000 {
        let diff = cursor.record_diff.as_ref().unwrap();
        let comparing = matches!(&diff.owned_copy, Some(RecordOwnedCopy::Comparison(owner)) if owner.changed.is_none() && owner.lease.is_some() && owner.cursor.result().is_none());
        let closing = matches!(&diff.owned_copy, Some(RecordOwnedCopy::Comparison(owner)) if owner.changed.is_some() && owner.lease.is_some());
        if closing && !old_retired {
            while !current.retire_one_detached() {}
            old_retired = true;
        }
        let copying = matches!(&diff.owned_copy, Some(RecordOwnedCopy::Component(owner)) if owner.candidate().is_none() && owner.source().is_some());
        let returning_source = matches!(&diff.owned_copy, Some(RecordOwnedCopy::Component(owner)) if owner.candidate().is_some() && owner.source().is_some());
        let returning_candidate = matches!(&diff.owned_copy, Some(RecordOwnedCopy::Component(owner)) if owner.candidate().is_some() && owner.source().is_none());
        assert!(matches!(cursor.advance_existing_component_with_grant(&current, 0), SurfaceReconcileStep::Yield { bytes: 0, .. }));
        let SurfaceReconcileStep::Yield { bytes, .. } = cursor.advance_existing_component(&current) else {
            panic!("retained completion fixture fault");
        };
        assert!(bytes <= fixture["physicalBytes"].as_u64().unwrap() as usize);
        let diff = cursor.record_diff.as_ref().unwrap();
        if comparing {
            assert_eq!(&diff.record.component as *const _, incoming);
            let Some(RecordOwnedCopy::Comparison(owner)) = &diff.owned_copy else {
                panic!("child must retain the comparison owner");
            };
            assert!(owner.changed.is_none() && owner.lease.is_some());
            if owner.cursor.result().is_some() {
                assert_eq!(bytes, SURFACE_COMPONENT_COPY_WORK_BYTES);
                comparison_completed = true;
            }
        }
        if closing && bytes >= SURFACE_COMPONENT_COPY_WORK_BYTES {
            assert!(bytes <= document_close_turn_ceiling(), "one lease-close turn must stay inside the page a `PageBytes` fault enforces: {bytes} of {}B", document_close_turn_ceiling());
            full_close_grant = true;
            assert!(matches!(&diff.owned_copy, Some(RecordOwnedCopy::Comparison(_))));
        }
        if copying && matches!(&diff.owned_copy, Some(RecordOwnedCopy::Component(owner)) if owner.candidate().is_some()) {
            assert!(bytes <= SURFACE_COMPONENT_COPY_WORK_BYTES && cursor.pending_op.get().is_none());
            copy_completed = true;
        }
        if returning_source {
            assert_eq!(bytes, size_of::<ui_contract::Component>());
            assert!(cursor.pending_op.get().is_none());
            source_returned = true;
        }
        if returning_candidate {
            assert_eq!(bytes, size_of::<ui_contract::UiPatchOp>());
            assert!(cursor.pending_op.get().is_some());
            candidate_returned = true;
        }
        if diff.field != 0 {
            break;
        }
    }
    while !cursor.retire_one() {}
    while !current.retire_one_detached() {}
    assert!(comparison_completed && copy_completed && source_returned && candidate_returned && full_close_grant);
    assert!(size_of::<ExistingComponentComparison>() <= SURFACE_COMPONENT_COPY_WORK_BYTES);
    eprintln!(
        "parent-child-grants compare-final={} lease-close=[{}, {}] node-record={} comparison-owner={} source-return={} candidate-physical={} separate-turns=true",
        SURFACE_COMPONENT_COPY_WORK_BYTES,
        SURFACE_COMPONENT_COPY_WORK_BYTES,
        document_close_turn_ceiling(),
        size_of::<ui_contract::UiNodeRecord>(),
        size_of::<ExistingComponentComparison>(),
        size_of::<ui_contract::Component>(),
        size_of::<ui_contract::UiPatchOp>()
    );
}
#[test]
fn surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind() {
    let _guard = registry_guard();
    for phase in document_fixture()["unwindPhases"].as_array().unwrap() {
        let phase = phase.as_str().unwrap();
        let component = |value| serde_json::from_value::<ui_contract::Component>(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":vec![value;32768]},"bindings":[]})).unwrap();
        let id = ui_contract::UiNodeId(1);
        let mut current = SurfaceShadow::from_text("retained-existing-unwind");
        current.install_fixture_record(build_record_owned(id, crate::TreeNode::try_new("surface", component(1u8)).unwrap(), Default::default(), None));
        let before = serde_json::to_value(&current.read_record(id).unwrap().unwrap().component).unwrap();
        let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
        cursor.stage = SurfaceReconcileStage::DiffRecords;
        cursor.record_diff = Some(RecordDiffCursor { id, record: build_record_owned(id, crate::TreeNode::try_new("surface", component(2u8)).unwrap(), Default::default(), None).into(), field: 0, fresh: None, owned_copy: None });
        let mut reached = false;
        for _ in 0..100_000 {
            let diff = cursor.record_diff.as_ref().unwrap();
            reached = match (phase, diff.owned_copy.as_ref()) {
                ("comparison", Some(RecordOwnedCopy::Comparison(owner))) => owner.lease.is_some() && owner.changed.is_none(),
                ("copy", Some(RecordOwnedCopy::Component(owner))) => owner.source().is_some() && owner.candidate().is_none(),
                ("source-returned", Some(RecordOwnedCopy::Component(owner))) => owner.source().is_none() && owner.candidate().is_some(),
                ("candidate-returned", Some(RecordOwnedCopy::Component(owner))) => owner.terminal_is_empty(),
                _ => false,
            };
            if reached {
                break;
            }
            assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
        }
        assert!(reached);
        cursor.fail_component_step = true;
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cursor.step(&current)));
        cursor.fail_component_step = false;
        assert!(failed.is_err());
        assert_eq!(serde_json::to_value(&current.read_record(id).unwrap().unwrap().component).unwrap(), before);
        while !cursor.retire_one() {}
        while !current.retire_one_detached() {}
        eprintln!("existing-pair-unwind phase={phase} exact-current-unchanged=true retained-close=true");
    }
}
#[test]
fn surface_canonical_document_fresh_children_retain_completed_roots_for_a_separate_turn() {
    let _guard = registry_guard();
    let mut outcomes = Vec::new();
    for field in document_fixture()["freshCompletionFields"].as_array().unwrap() {
        let field = field.as_str().unwrap();
        let component: ui_contract::Component = serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":vec![17u8;32768]},"bindings":[]})).unwrap();
        let mut node = crate::TreeNode::try_new("source", component).unwrap();
        for index in 0..32 {
            node = with_binding(node, "fixture", &format!("action-{index}"));
        }
        let mut current = SurfaceShadow::from_text("fresh-child-grant");
        let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
        cursor.stage = SurfaceReconcileStage::DiffRecords;
        let start = if field == "component" { 1 } else { 5 };
        cursor.record_diff =
            Some(RecordDiffCursor { id: ui_contract::UiNodeId(1), record: build_record_owned(ui_contract::UiNodeId(1), node, Default::default(), None).into(), field: start, fresh: Some(FreshRecordClone::default()), owned_copy: None });
        let mut separate = false;
        for _ in 0..100_000 {
            let step = cursor.step(&current);
            assert!(matches!(step, SurfaceReconcileStep::Yield { bytes, .. } if bytes <= 32768));
            let diff = cursor.record_diff.as_ref().unwrap();
            separate = match diff.owned_copy.as_ref() {
                Some(RecordOwnedCopy::Component(owner)) => owner.candidate().is_some() && owner.source().is_some(),
                Some(RecordOwnedCopy::Bindings(owner)) => owner.candidate().is_some_and(|candidate| candidate.len() == 32) && owner.source_allocated_bytes() != 0,
                _ => false,
            };
            if separate || diff.field != start {
                break;
            }
        }
        while !cursor.retire_one() {}
        while !current.retire_one_detached() {}
        outcomes.push((field.to_string(), separate));
    }
    assert!(outcomes.iter().all(|(_, separate)| *separate));
}
//#endregion 🧪️CanonicalRootLaws
