/// 🛑️ Transfer cancel and document replacement laws: cancel around every transfer, scan and shift units, adjacent transfer rollback, the persistent owner phases and the split publication and replacement boundaries.

#[test]
fn retained_vcs_cancel_around_every_transfer_has_exact_resource_fingerprints() {
    for completed_polls in 0..3 {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 29 + completed_polls, 1, 0);
        let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
        for _ in 0..completed_polls {
            session.poll(handle, retained_grant()).expect("pre-cancel transfer");
        }
        let before = session.resource_fingerprint();
        session.cancel(handle, retained_grant()).expect("cancel before publication");
        assert_eq!(session.resource_fingerprint(), before);
        while !matches!(session.close_operation_step(handle, retained_grant()).expect("cancel close"),RetainedCloneStep::Complete(_)) {}
        close_to_terminal(&mut session);
    }

    let mut session = FlowRetainedVcs::new(retained_fixture(), 37, 1, 0);
    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    drive_to_preview(&mut session, handle);
    let before_page = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), before_page);

    let page = session.take_page(handle).expect("take");
    let after_take = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), after_take);
    session.resume_page(handle, page.sequence).expect("resume");
    let after_resume = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), after_resume);
    session.retry_page(handle, page.sequence).expect("retry");
    let after_retry = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), after_retry);
    session.acknowledge_page(handle, page.sequence).expect("ack");
    let after_ack = session.resource_fingerprint();
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
    assert_eq!(session.resource_fingerprint(), after_ack);

    close_to_terminal(&mut session);
}

#[test]
fn retained_vcs_scan_and_shift_advance_only_one_semantic_unit_per_grant() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 41, 1, 0);
    let mut source = FlowVcsSource::new(Widget::InputNote { id: "cursor-item".into(), text: "bounded".into() });
    let handle = session.begin_add_widget(session.authority(), 0, &mut source).expect("cursor admission");
    session.poll(handle, retained_grant()).expect("progress");
    session.poll(handle, retained_grant()).expect("checkpoint");
    let slot = usize::from(handle.slot);
    assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.scan, 0);
    session.poll(handle, retained_grant()).expect("one scan");
    assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.scan, 1);
    session.poll(handle, retained_grant()).expect("second scan");
    assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.scan, 2);
    drive_to_preview(&mut session, handle);
    close_to_terminal(&mut session);
}

#[test]
fn retained_vcs_cancel_during_adjacent_transfer_rolls_back_exact_document() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 43, 1, 0);
    let before = session.resource_fingerprint();
    let before_digest = flow_vcs_host_snapshot_scalar_digest(session.document.as_ref().expect("document").host_snapshot());
    let before_ids: Vec<String> = session.document.as_ref().expect("document").host_snapshot().widgets.iter().map(|widget| widget_id_for(widget).to_owned()).collect();
    let mut source = FlowVcsSource::new(Widget::InputNote { id: "rollback-item".into(), text: "owned".into() });
    let handle = session.begin_add_widget(session.authority(), 0, &mut source).expect("cursor admission");
    session.poll(handle, retained_grant()).expect("progress");
    session.poll(handle, retained_grant()).expect("checkpoint");
    while !session.operations[usize::from(handle.slot)].as_ref().expect("operation").cursor.mutated {
        session.poll(handle, retained_grant()).expect("reach first transfer");
    }
    session.poll(handle, retained_grant()).expect("one adjacent swap");
    session.cancel(handle, retained_grant()).expect("cancel between transfers");
    while !matches!(session.close_operation_step(handle, retained_grant()).expect("incremental rollback close"),RetainedCloneStep::Complete(_)) {}
    while !matches!(session.close_retired_step(retained_grant()).expect("retire cancelled source"),RetainedCloneStep::Complete(_)) {}
    assert_eq!(flow_vcs_host_snapshot_scalar_digest(session.document.as_ref().expect("document").host_snapshot()), before_digest);
    let after_ids: Vec<String> = session.document.as_ref().expect("document").host_snapshot().widgets.iter().map(|widget| widget_id_for(widget).to_owned()).collect();
    assert_eq!(after_ids, before_ids);
    let after = session.resource_fingerprint();
    assert_eq!(after, before);
    close_to_terminal(&mut session);
}

#[test]
fn retained_vcs_replace_document_uses_persistent_owner_transfer_phases() {
    let mut replacement = retained_fixture();
    replacement.widgets.push(Widget::InputNote { id: "replacement-tail".into(), text: "tail".into() });
    let expected_widgets = replacement.widgets.len();
    let mut session = FlowRetainedVcs::new(retained_fixture(), 47, 1, 0);
    let mut source = FlowVcsSource::new(replacement);
    let handle = session.begin_replace_document(session.authority(), &mut source).expect("replace admission");
    session.poll(handle, retained_grant()).expect("progress");
    session.poll(handle, retained_grant()).expect("checkpoint");
    let slot = usize::from(handle.slot);
    session.poll(handle, retained_grant()).expect("reserve empty version");
    assert_eq!(session.document.as_ref().expect("document").versions.len(), 2);
    assert_eq!(session.document.as_ref().expect("document").versions.get(1).expect("candidate").widgets.len(), 0);
    session.poll(handle, retained_grant()).expect("transfer schema only");
    assert_eq!(session.document.as_ref().expect("document").versions.get(1).expect("candidate").widgets.len(), 0);
    drive_to_preview(&mut session, handle);
    assert_eq!(session.document.as_ref().expect("document").host_snapshot().widgets.len(), expected_widgets);
    assert_eq!(session.operations[slot].as_ref().expect("operation").stage, FlowVcsStage::PageReady);
    assert_eq!(session.undo.len(), 0);
    assert_eq!(session.redo.len(), 0);
    assert_eq!(session.document.as_ref().expect("document").active, 1);
    close_to_terminal(&mut session);
    retire_snapshot_source(&mut source);
}

#[test]
fn retained_vcs_replace_document_is_a_load_that_writes_no_history_row_and_refuses_a_live_history() {
    let mut loaded = FlowRetainedVcs::new(retained_fixture(), 48, 1, 0);
    let mut source = FlowVcsSource::new(retained_fixture());
    let handle = loaded.begin_replace_document(loaded.authority(), &mut source).expect("load admission");
    drive_to_preview(&mut loaded, handle);
    assert_eq!(loaded.undo.len(), 0);
    assert_eq!(loaded.redo.len(), 0);
    close_to_terminal(&mut loaded);
    retire_snapshot_source(&mut source);

    let mut live = FlowRetainedVcs::new(retained_fixture(), 50, 1, 0);
    live.undo.push(FlowVcsAction::Checkpoint).expect("fixed undo owner");
    let mut refused = FlowVcsSource::new(retained_fixture());
    assert_eq!(live.begin_replace_document(live.authority(), &mut refused), Err(FlowVcsFault::InvalidMutation));
    close_to_terminal(&mut live);
    retire_snapshot_source(&mut refused);
}

#[test]
fn retained_vcs_cancel_restores_every_partially_retired_redo_owner() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 49, 1, 0);
    for _ in 0..4 {
        session.redo.push(FlowVcsAction::Checkpoint).expect("fixed redo owner");
    }
    let before = session.resource_fingerprint();
    let mut source = FlowVcsSource::new(FlowLayoutEntry { id: "source".into(), layout: Some(WidgetLayout { x: 22.0, y: 23.0 }) });
    let handle = session.begin_set_layout(session.authority(), &mut source).expect("new branch");
    let slot = usize::from(handle.slot);
    while session.operations[slot].as_ref().expect("operation").cursor.phase != FlowVcsCursorPhase::RetireRedo {
        session.poll(handle, retained_grant()).expect("reach redo retirement");
    }
    for expected in 1..=3 {
        session.poll(handle, retained_grant()).expect("retire one redo owner");
        assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.redo_retired, expected);
    }
    session.cancel(handle, retained_grant()).expect("cancel after redo transfer");
    while !matches!(session.close_operation_step(handle, retained_grant()).expect("restore redo and semantic owner"),RetainedCloneStep::Complete(_)) {}
    while !matches!(session.close_retired_step(retained_grant()).expect("retire cancelled request"),RetainedCloneStep::Complete(_)) {}
    assert_eq!(session.resource_fingerprint(), before);
    assert_eq!(session.document.as_ref().expect("document").host_snapshot().layout.get("source"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    close_to_terminal(&mut session);
}

#[test]
fn retained_vcs_cancel_restores_each_split_publication_boundary() {
    for target in [FlowVcsCursorPhase::TransferSurface, FlowVcsCursorPhase::PublishVisibility, FlowVcsCursorPhase::PublishPage] {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 51, 1, 0);
        session.bind_surface(101, 202, 303).expect("surface owner");
        let before = session.resource_fingerprint();
        let mut source = FlowVcsSource::new(FlowLayoutEntry { id: "source".into(), layout: Some(WidgetLayout { x: 31.0, y: 32.0 }) });
        let handle = session.begin_set_layout(session.authority(), &mut source).expect("publication cursor");
        let slot = usize::from(handle.slot);
        while session.operations[slot].as_ref().expect("operation").cursor.phase != target {
            session.poll(handle, retained_grant()).expect("reach publication boundary");
        }
        if target == FlowVcsCursorPhase::PublishVisibility {
            session.fault(handle, retained_grant()).expect("fault at surface boundary");
        } else {
            session.cancel(handle, retained_grant()).expect("cancel at publication boundary");
        }
        loop {
            let rejected = session.resource_fingerprint();
            assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
            assert_eq!(session.resource_fingerprint(), rejected);
            if matches!(session.close_operation_step(handle, retained_grant()).expect("publication rollback"),RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        while !matches!(session.close_retired_step(retained_grant()).expect("retire cancelled request"),RetainedCloneStep::Complete(_)) {}
        assert_eq!(session.resource_fingerprint(), before);
        assert_eq!(session.document.as_ref().expect("document").host_snapshot().layout.get("source"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
        close_to_terminal(&mut session);
    }
}

#[test]
fn retained_vcs_cancel_restores_each_document_replacement_boundary() {
    let targets = [
        FlowVcsCursorPhase::ReserveReplacement,
        FlowVcsCursorPhase::ReplaceSchema,
        FlowVcsCursorPhase::ReplaceCameraX,
        FlowVcsCursorPhase::ReplaceCameraY,
        FlowVcsCursorPhase::ReplaceCameraZoom,
        FlowVcsCursorPhase::ReplaceWidgets,
        FlowVcsCursorPhase::ReverseWidgets,
        FlowVcsCursorPhase::ReplaceSynapses,
        FlowVcsCursorPhase::ReverseSynapses,
        FlowVcsCursorPhase::ReplaceLayout,
        FlowVcsCursorPhase::TransferHistory,
    ];
    for (generation, target) in targets.into_iter().enumerate() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), u32::try_from(61 + generation).expect("generation"), 1, 0);
        let before = session.resource_fingerprint();
        let mut replacement = retained_fixture();
        replacement.widgets.push(Widget::InputNote { id: "replacement-boundary".into(), text: "owner".into() });
        let mut source = FlowVcsSource::new(replacement);
        let handle = session.begin_replace_document(session.authority(), &mut source).expect("replacement cursor");
        let slot = usize::from(handle.slot);
        while session.operations[slot].as_ref().expect("operation").cursor.phase != target {
            session.poll(handle, retained_grant()).expect("reach replacement boundary");
        }
        session.cancel(handle, retained_grant()).expect("cancel replacement boundary");
        loop {
            let rejected = session.resource_fingerprint();
            assert_eq!(session.fault(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
            assert_eq!(session.resource_fingerprint(), rejected);
            if matches!(session.close_operation_step(handle, retained_grant()).expect("replacement rollback"),RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        while !matches!(session.close_retired_step(retained_grant()).expect("replacement retirement"),RetainedCloneStep::Complete(_)) {}
        assert_eq!(session.resource_fingerprint(), before);
        close_to_terminal(&mut session);
        retire_snapshot_source(&mut source);
    }
}
