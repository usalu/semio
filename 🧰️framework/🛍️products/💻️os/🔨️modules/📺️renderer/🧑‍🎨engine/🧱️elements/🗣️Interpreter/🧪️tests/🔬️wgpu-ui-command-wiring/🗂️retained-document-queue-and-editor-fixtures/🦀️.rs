/// 🗂️ Retained document queue and editor fixtures: the unbounded step grant, the text editor outbox and focus fixtures, and the retained document close queue, page budget and wrong generation laws.

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

fn collect_text_editor_actions_accepted(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let _ = semio_framework_os_renderer_wgpu_engine_text::text_editor::drive_text_editor_outbox_step(input).expect("text editor outbox");
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority") {
        let queued = action.into_envelope().expect("action envelope");
        if let Some(receipt) = queued.receipt {
            semio_framework_os_renderer_wgpu_engine_text::text_editor::settle_text_editor_action_receipt(receipt, semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Accepted);
        }
        actions.push(queued.descriptor);
    }
    actions
}

fn install_fixture_text_editor_focus(window_id: &str, window_generation: u64, node: NodeId, host_id: &str) -> FocusedTextEditor {
    if UI_ENGINE.with(|cell| cell.borrow().presented_document_id(window_id, node).is_none()) {
        present_seeded_scene_window(window_id);
    }
    let focus = UI_ENGINE.with(|cell| text_editor_focus_in(&cell.borrow(), window_id, window_generation, node, host_id)).expect("the accepted TextEditor fixture has an exact surface and document identity");
    install_text_editor_focus(focus.clone());
    focus
}

#[test]
fn retained_document_close_queue_is_bounded_and_same_window_replacement_costs_no_credit() {
    let mut queue = UiDocumentCloseQueue::default();
    let mut engine = semio_framework_ui_wgpu_engine::Ui::new(semio_framework_ui_locale::Locale::En);
    for index in 0..semio_framework_ui_wgpu_engine::UI_LAYOUT_SURFACE_SLOTS {
        let window_id = SurfaceId::try_from(format!("close-{index}").as_str()).unwrap();
        let token = engine.try_admit_surface(window_id.as_ref()).unwrap();
        assert!(queue.try_upsert(UiDocumentCloseOwner { window_id, token, generation: 1 }));
    }
    let token = engine.surface_token("close-0").unwrap();
    assert!(!queue.try_upsert(UiDocumentCloseOwner { window_id: SurfaceId::try_from("close-overflow").unwrap(), token, generation: 1 }), "a sixty-fifth retained surface close is refused before its owner is lost");
    assert!(queue.try_upsert(UiDocumentCloseOwner { window_id: SurfaceId::try_from("close-0").unwrap(), token, generation: 2 }), "the same window replaces its stale close generation without another credit");
    assert_eq!(queue.owners.len(), semio_framework_ui_surface_identity::UI_LAYOUT_SURFACE_SLOTS);
    assert_eq!(queue.owners.front().map(|owner| owner.generation), Some(2));
}

fn ingress_opportunity_document() -> UiDocumentLease {
    let fixture: Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🌳️document-tree-reconcile/🔣️.json")).unwrap();
    let source = &fixture["document"];
    let mut builder = ui_contract::UiDocumentBuilder::try_new(
        1,
        ui_contract::SurfaceId::try_from(source["surface"].as_str().unwrap()).unwrap(),
        ui_contract::UiRevision(1),
        Some(ui_contract::UiNodeId(source["root"].as_u64().unwrap())),
        source["layoutEpoch"].as_u64().unwrap(),
    )
    .unwrap();
    for record in source["nodes"].as_array().unwrap() {
        builder.try_push(serde_json::from_value(record.clone()).unwrap()).unwrap();
    }
    builder.finish().unwrap()
}

#[test]
fn retained_document_page_budget_refusal_preserves_the_cursor_and_retries_the_same_page() {
    for (fuel, deadline, clock, cancelled) in [(0, u64::MAX, (|| Some(0)) as fn() -> Option<u64>, false), (1, 1, (|| Some(2)) as fn() -> Option<u64>, false), (1, u64::MAX, (|| Some(0)) as fn() -> Option<u64>, true)] {
        let mut document = ingress_opportunity_document();
        let header = document.header().unwrap();
        let mut engine = semio_framework_ui_wgpu_engine::Ui::new(semio_framework_ui_locale::Locale::En);
        let mut cursor = UiDocumentFrameCursor::default();
        let mut sequence = 0;
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut admitted_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut admitted = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), || Some(0), &mut sequence, &mut admitted_retained);
        engine.begin_document("budget-refusal", header, &mut admitted).unwrap();
        if cancelled {
            cancel.cancel_now();
        }
        let mut refused_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut refused = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, deadline, UNBOUNDED_STEP_GRANT), cancel.clone(), clock, &mut sequence, &mut refused_retained);
        apply_retained_document_page(&mut engine, &mut cursor, "budget-refusal", document.read_node_page(0).unwrap().unwrap(), &mut refused);
        assert!(!cursor.terminal_is_fault(), "a refused opportunity must retain the page cursor for a fresh grant");
        assert!(matches!(engine.document_status("budget-refusal", 1), semio_framework_ui_wgpu_engine_surface::UiDocumentIngressStatus::Pending { next_page: 0, .. }));
        let mut retry_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut retry =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::CancelToken::root_now(), || Some(0), &mut sequence, &mut retry_retained);
        apply_retained_document_page(&mut engine, &mut cursor, "budget-refusal", document.read_node_page(0).unwrap().unwrap(), &mut retry);
        assert!(!cursor.terminal_is_fault());
        assert!(matches!(engine.document_status("budget-refusal", 1), semio_framework_ui_wgpu_engine_surface::UiDocumentIngressStatus::Pending { next_page: 1, .. }));
        while !document.close_step() {}
    }
}

#[test]
fn retained_document_page_with_a_live_wrong_generation_remains_a_terminal_fault() {
    let mut document = ingress_opportunity_document();
    let mut engine = semio_framework_ui_wgpu_engine::Ui::new(semio_framework_ui_locale::Locale::En);
    let mut cursor = UiDocumentFrameCursor::default();
    let mut sequence = 0;
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut admitted_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut admitted = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), || Some(0), &mut sequence, &mut admitted_retained);
    engine.begin_document("wrong-generation", document.header().unwrap(), &mut admitted).unwrap();
    let mut stale_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut stale = semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(2), semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel, || Some(0), &mut sequence, &mut stale_retained);
    apply_retained_document_page(&mut engine, &mut cursor, "wrong-generation", document.read_node_page(0).unwrap().unwrap(), &mut stale);
    assert!(cursor.terminal_is_fault());
    while !document.close_step() {}
}
