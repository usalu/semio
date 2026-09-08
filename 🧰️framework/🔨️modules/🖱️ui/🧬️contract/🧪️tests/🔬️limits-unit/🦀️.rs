use super::*;

fn text(value: &str) -> crate::UiText {
    crate::UiText::try_from_str(value).expect("bounded fixture")
}

fn surface() -> crate::SurfaceId {
    crate::SurfaceId::try_from("surface").expect("bounded fixture")
}

fn leaf(id: u64, key: &str) -> crate::UiNodeRecord {
    crate::UiNodeRecord {
        id: crate::UiNodeId(id),
        key: text(key),
        component: crate::Component::Separator(crate::SeparatorProps {}),
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: crate::UiNodeBindings::default(),
        menu: None,
        children: crate::UiNodeChildren::default(),
    }
}

fn state_with_root() -> crate::UiSnapshotState {
    let mut state = crate::UiSnapshotState::new(surface());
    state.root = Some(crate::UiNodeId(0));
    state.nodes.try_insert(leaf(0, "root")).expect("fixed root");
    state
}

fn patch_with(base: crate::UiRevision, revision: crate::UiRevision, ops: crate::UiPatchOps) -> crate::UiPatch {
    crate::UiPatch { surface: surface(), base_revision: base, revision, ops }
}

fn drive_ready(mut producer: UiPatchApplyProducer, generation: u64) -> UiPatchApplyOutcome {
    loop {
        match producer.drive_one(generation, false, false) {
            UiPatchApplyStep::MoreWork => {}
            UiPatchApplyStep::Ready => return producer.take_ready().unwrap_or_else(|_| panic!("ready producer")),
            UiPatchApplyStep::Rejected => panic!("unexpected rejection: {:?}", producer.rejection()),
        }
    }
}

#[test]
fn retained_patch_applies_one_census_node_op_and_validation_owner_per_opportunity() {
    let state = state_with_root();
    let mut ops = crate::UiPatchOps::default();
    ops.try_push(crate::UiPatchOp::SetActivity { id: crate::UiNodeId(0), activity: crate::Activity::Loading, disabled: true }).expect("one op");
    let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), ops);
    let mut producer = UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), 11).expect("retained producer");
    assert_eq!(producer.drive_one(11, false, true), UiPatchApplyStep::MoreWork, "expired deadline leaves the first census owner untouched");
    let mut outcome = drive_ready(producer, 11);
    assert_eq!(outcome.state().map(crate::UiSnapshotState::revision), Some(crate::UiRevision(1)));
    assert!(outcome.state().and_then(|state| state.get(crate::UiNodeId(0))).is_some_and(|record| record.disabled));
    while !outcome.close_step() {}
    let state = outcome.take_state().expect("closed outcome returns exact candidate");
    assert_eq!(state.revision, crate::UiRevision(1));
}

#[test]
fn retained_patch_max_plus_one_returns_exact_state_and_patch() {
    let state = state_with_root();
    let mut ops = crate::UiPatchOps::default();
    ops.try_push(crate::UiPatchOp::SetRoot { id: crate::UiNodeId(0) }).expect("one op");
    let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), ops);
    let limits = UiDocumentLimits { max_patch_ops: 0, ..UiDocumentLimits::default() };
    let mut rejected = UiPatchApplyProducer::try_new(state, patch, limits, 12).expect_err("maximum plus one rejects before cloning");
    assert!(matches!(rejected.rejection(), PatchRejection::QuotaExceeded { quota: QuotaKind::PatchOps, actual: 1, max: 0 }));
    assert_eq!(rejected.state().map(crate::UiSnapshotState::revision), Some(crate::UiRevision(0)));
    while !rejected.close_step() {}
    assert_eq!(rejected.take_state().expect("exact original state").revision, crate::UiRevision(0));
}

#[test]
fn retained_patch_cancel_stale_and_deadline_preserve_owner() {
    for (actual_generation, cancelled) in [(14, false), (13, true)] {
        let state = state_with_root();
        let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), crate::UiPatchOps::default());
        let mut producer = UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), 13).expect("retained producer");
        assert_eq!(producer.drive_one(actual_generation, cancelled, false), UiPatchApplyStep::Rejected);
        let mut rejected = producer.take_rejected().unwrap_or_else(|_| panic!("rejected owner"));
        while !rejected.close_step() {}
        assert_eq!(rejected.take_state().expect("exact original state").revision, crate::UiRevision(0));
    }
}

#[test]
fn retained_patch_remove_advances_one_node_or_child_per_opportunity() {
    let mut state = state_with_root();
    let mut root_children = crate::UiNodeChildren::default();
    root_children.try_push(crate::UiNodeId(1)).expect("one child");
    state.nodes.get_mut(&crate::UiNodeId(0)).expect("root").children = root_children;
    let mut branch = leaf(1, "branch");
    branch.component = crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None });
    branch.children.try_push(crate::UiNodeId(2)).expect("one grandchild");
    state.nodes.try_insert(branch).expect("branch");
    state.nodes.try_insert(leaf(2, "leaf")).expect("leaf");
    let mut ops = crate::UiPatchOps::default();
    ops.try_push(crate::UiPatchOp::Remove { id: crate::UiNodeId(1) }).expect("remove");
    ops.try_push(crate::UiPatchOp::SetChildren { id: crate::UiNodeId(0), children: crate::UiNodeChildren::default() }).expect("detach");
    let mut outcome = drive_ready(UiPatchApplyProducer::try_new(state, patch_with(crate::UiRevision(0), crate::UiRevision(1), ops), UiDocumentLimits::default(), 15).expect("producer"), 15);
    assert_eq!(outcome.state().map(|state| state.nodes.len()), Some(1));
    while !outcome.close_step() {}
    let _ = outcome.take_state().expect("exact candidate");
}

#[test]
fn retained_patch_duplicate_and_deep_validation_are_cursorized() {
    let mut state = state_with_root();
    let mut children = crate::UiNodeChildren::default();
    children.try_push(crate::UiNodeId(1)).expect("first");
    children.try_push(crate::UiNodeId(2)).expect("second");
    state.nodes.get_mut(&crate::UiNodeId(0)).expect("root").children = children;
    state.nodes.try_insert(leaf(1, "duplicate")).expect("first leaf");
    state.nodes.try_insert(leaf(2, "duplicate")).expect("second leaf");
    let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), crate::UiPatchOps::default());
    let mut producer = UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), 16).expect("producer");
    loop {
        match producer.drive_one(16, false, false) {
            UiPatchApplyStep::MoreWork => {}
            UiPatchApplyStep::Rejected => break,
            UiPatchApplyStep::Ready => panic!("duplicate sibling key must reject"),
        }
    }
    assert!(matches!(producer.rejection(), Some(PatchRejection::InvariantViolated { .. })));
    drop(producer);
    while !close_ui_patch_owner_one() {}
}

#[test]
fn patch_handback_arena_max_plus_one_refuses_without_reusing_a_live_generation() {
    let mut arena = UiPatchApplyArena::default();
    let mut handles = [None; UI_PATCH_APPLY_SLOTS];
    for (index, handle) in handles.iter_mut().enumerate() {
        *handle = arena.reserve(index as u64 + 1);
        assert!(handle.is_some());
    }
    assert!(arena.reserve(99).is_none());
    for handle in handles.into_iter().flatten() {
        arena.release(handle);
    }
    let replacement = arena.reserve(100).expect("released slot");
    assert_eq!(replacement.generation, 100);
    assert!(handles.into_iter().flatten().all(|old| old.epoch != replacement.epoch || old.slot != replacement.slot));
}

#[test]
fn abandoned_patch_owner_moves_to_incremental_handback_and_reopens_capacity() {
    while !close_ui_patch_owner_one() {}
    let state = state_with_root();
    let mut ops = crate::UiPatchOps::default();
    ops.try_push(crate::UiPatchOp::SetRoot { id: crate::UiNodeId(0) }).expect("one op");
    let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), ops);
    let mut producer = UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), 17).expect("reserved producer");
    assert_eq!(producer.drive_one(17, false, false), UiPatchApplyStep::MoreWork);
    drop(producer);
    assert!(!close_ui_patch_owner_one());
    while !close_ui_patch_owner_one() {}
    let replacement = UiPatchApplyProducer::try_new(state_with_root(), patch_with(crate::UiRevision(0), crate::UiRevision(1), crate::UiPatchOps::default()), UiDocumentLimits::default(), 18).expect("handback released capacity");
    drop(replacement);
    while !close_ui_patch_owner_one() {}
}
