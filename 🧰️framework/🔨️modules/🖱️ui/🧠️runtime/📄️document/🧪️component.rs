use super::*;

//#region 🧪️CanonicalRootLaws
fn document_fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixture.json")).unwrap() }

fn complete_document_job(current: SurfaceReconciler, name: &str, generation: u64) -> SurfaceReconciler {
    let mut job = SurfaceReconcileJob::try_new(current, tree(leaf(name)), generation).expect("real reconciliation admission");
    let mut sequence = 0;
    for _ in 0..100_000 {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
        match job.drive_one(&mut cx) {
            SurfaceReconcileJobStep::MoreWork => {}
            SurfaceReconcileJobStep::Ready => {
                let (current, output) = job.take_ready().unwrap_or_else(|_| panic!("completed job retains its exact ready output"));
                if let Some(mut output) = output { while !output.close_step() {} }
                return current;
            }
            SurfaceReconcileJobStep::Fault => panic!("real reconciliation fault: {:?}", job.fault()),
        }
    }
    panic!("real reconciliation did not finish")
}

#[test]
fn surface_canonical_document_nine_live_reconcilers_share_the_original_root_with_readers() {
    let fixture = document_fixture();
    let mut owners = Vec::new();
    let mut readers = Vec::new();
    for index in 0..fixture["surfaces"].as_u64().unwrap() {
        let owner = complete_document_job(SurfaceReconciler::new(format!("canonical-{index}")), "root", 910_000 + index);
        let mut reader = None;
        assert!(owner.capture_document(&mut reader, fixture["physicalBytes"].as_u64().unwrap() as usize).unwrap());
        let reader = reader.unwrap();
        assert!(owner.document.as_ref().unwrap().same_root(&reader));
        assert_eq!(reader.try_read().unwrap().len(), 1);
        owners.push(owner); readers.push(reader);
    }
    assert_eq!(owners.len(), 9);
    for owner in &mut owners { while !owner.retire_one() {} }
    for reader in &readers { assert_eq!(reader.try_read().unwrap().len(), 1); }
    for reader in &mut readers { while !reader.close_read_step_with_grant(1, 64).unwrap().complete {} }
    eprintln!("[DEBUG] canonical-reconcilers actual-surfaces=9 exact-root-readers=9 roots-after-owner-close=9 typed-reader-close=true");
}

#[test]
fn surface_canonical_document_old_reader_keeps_original_credit_during_replacement() {
    for grant in document_fixture()["readerReleaseGrants"].as_array().unwrap() {
        let grant = grant.as_u64().unwrap() as usize;
        let current = complete_document_job(SurfaceReconciler::new("canonical-reader-pressure"), "before", 920_000 + grant as u64);
        let mut reader = None;
        assert!(current.capture_document(&mut reader, 32768).unwrap());
        let mut reader = reader.unwrap();
        let before = serde_json::to_value(reader.try_read().unwrap().node_at(0).unwrap()).unwrap();
        let mut replacement = complete_document_job(current, "after", 930_000 + grant as u64);
        assert!(!replacement.document.as_ref().unwrap().same_root(&reader));
        assert_eq!(serde_json::to_value(reader.try_read().unwrap().node_at(0).unwrap()).unwrap(), before);
        assert!(reader.try_read().unwrap().resident_limits().bytes > 0);
        while !replacement.retire_one() {}
        assert_eq!(serde_json::to_value(reader.try_read().unwrap().node_at(0).unwrap()).unwrap(), before);
        while !reader.close_read_step_with_grant(1, grant).unwrap().complete {}
        eprintln!("[DEBUG] canonical-reader-replacement grant={grant} original-root-unchanged=true original-credit-retained=true typed-terminal=true");
    }
}

#[test]
fn surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant() {
    let fixture = document_fixture();
    assert!(fixture["completionTransferUsesSeparateGrant"].as_bool().unwrap());
    let component = |last| {
        let mut bytes = vec![17u8; 32768];
        *bytes.last_mut().unwrap() = last;
        serde_json::from_value::<ui_contract::Component>(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":bytes},"bindings":[]})).unwrap()
    };
    let id = ui_contract::UiNodeId(1);
    let mut current = SurfaceReconciler::new("completion-grant");
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
        if closing && !old_retired { while !current.retire_one() {} old_retired = true; }
        let copying = matches!(&diff.owned_copy, Some(RecordOwnedCopy::Component(owner)) if owner.candidate().is_none() && owner.source().is_some());
        let returning_source = matches!(&diff.owned_copy, Some(RecordOwnedCopy::Component(owner)) if owner.candidate().is_some() && owner.source().is_some());
        let returning_candidate = matches!(&diff.owned_copy, Some(RecordOwnedCopy::Component(owner)) if owner.candidate().is_some() && owner.source().is_none());
        assert!(matches!(cursor.advance_existing_component_with_grant(&current, 0), SurfaceReconcileStep::Yield { bytes: 0, .. }));
        let SurfaceReconcileStep::Yield { bytes, .. } = cursor.advance_existing_component(&current) else { panic!("retained completion fixture fault"); };
        assert!(bytes <= fixture["physicalBytes"].as_u64().unwrap() as usize);
        let diff = cursor.record_diff.as_ref().unwrap();
        if comparing {
            assert_eq!(&diff.record.component as *const _, incoming);
            let Some(RecordOwnedCopy::Comparison(owner)) = &diff.owned_copy else { panic!("child must retain the comparison owner"); };
            assert!(owner.changed.is_none() && owner.lease.is_some());
            if owner.cursor.result().is_some() { assert_eq!(bytes, 4096); comparison_completed = true; }
        }
        if closing && bytes == 4096 { full_close_grant = true; assert!(matches!(&diff.owned_copy, Some(RecordOwnedCopy::Comparison(_)))); }
        if copying && matches!(&diff.owned_copy, Some(RecordOwnedCopy::Component(owner)) if owner.candidate().is_some()) {
            assert!(bytes <= 4096 && cursor.pending_op.get().is_none());
            copy_completed = true;
        }
        if returning_source { assert_eq!(bytes, size_of::<ui_contract::Component>()); assert!(cursor.pending_op.get().is_none()); source_returned = true; }
        if returning_candidate { assert_eq!(bytes, size_of::<ui_contract::UiPatchOp>()); assert!(cursor.pending_op.get().is_some()); candidate_returned = true; }
        if diff.field != 0 { break; }
    }
    while !cursor.retire_one() {}
    while !current.retire_one() {}
    assert!(comparison_completed && copy_completed && source_returned && candidate_returned && full_close_grant);
    assert!(size_of::<ExistingComponentComparison>() <= 4096);
    eprintln!("[DEBUG] parent-child-grants compare-final=4096 lease-close=4096 comparison-owner={} source-return={} candidate-physical={} separate-turns=true", size_of::<ExistingComponentComparison>(), size_of::<ui_contract::Component>(), size_of::<ui_contract::UiPatchOp>());
}

#[test]
fn surface_canonical_document_existing_pair_stays_structurally_owned_across_unwind() {
    for phase in document_fixture()["unwindPhases"].as_array().unwrap() {
        let phase = phase.as_str().unwrap();
        let component = |value| serde_json::from_value::<ui_contract::Component>(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":vec![value;32768]},"bindings":[]})).unwrap();
        let id = ui_contract::UiNodeId(1);
        let mut current = SurfaceReconciler::new("retained-existing-unwind");
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
            if reached { break; }
            assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
        }
        assert!(reached);
        cursor.fail_component_step = true;
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cursor.step(&current)));
        cursor.fail_component_step = false;
        assert!(failed.is_err());
        assert_eq!(serde_json::to_value(&current.read_record(id).unwrap().unwrap().component).unwrap(), before);
        while !cursor.retire_one() {}
        while !current.retire_one() {}
        eprintln!("[DEBUG] existing-pair-unwind phase={phase} exact-current-unchanged=true retained-close=true");
    }
}

#[test]
fn surface_canonical_document_fresh_children_retain_completed_roots_for_a_separate_turn() {
    let mut outcomes = Vec::new();
    for field in document_fixture()["freshCompletionFields"].as_array().unwrap() {
        let field = field.as_str().unwrap();
        let component: ui_contract::Component = serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":vec![17u8;32768]},"bindings":[]})).unwrap();
        let mut node = crate::TreeNode::try_new("source", component).unwrap();
        for index in 0..32 { node = with_binding(node, "fixture", &format!("action-{index}")); }
        let mut current = SurfaceReconciler::new("fresh-child-grant");
        let mut cursor = SurfaceReconcileCursor::new(tree(leaf("unused")), &current);
        cursor.stage = SurfaceReconcileStage::DiffRecords;
        let start = if field == "component" { 1 } else { 5 };
        cursor.record_diff = Some(RecordDiffCursor { id: ui_contract::UiNodeId(1), record: build_record_owned(ui_contract::UiNodeId(1), node, Default::default(), None).into(), field: start, fresh: Some(FreshRecordClone::default()), owned_copy: None });
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
            if separate || diff.field != start { break; }
        }
        while !cursor.retire_one() {}
        while !current.retire_one() {}
        outcomes.push((field.to_string(), separate));
    }
    eprintln!("[DEBUG] fresh-child-completion {outcomes:?}");
    assert!(outcomes.iter().all(|(_, separate)| *separate));
}
//#endregion 🧪️CanonicalRootLaws
