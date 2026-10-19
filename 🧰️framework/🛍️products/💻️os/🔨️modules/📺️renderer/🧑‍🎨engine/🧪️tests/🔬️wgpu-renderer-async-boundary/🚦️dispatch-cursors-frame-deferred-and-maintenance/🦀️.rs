/// 🚦️ Dispatch cursors, frame deferred and maintenance: the native entrypoint driver, the manifest edges, the dispatch cursors, the frame deferred cursor and cancel, the catalogue terminal pair, the maintenance authority, refusal cell and test owner.

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn presenter_restores_only_the_checked_out_interaction_and_retains_its_deferred_cursor() {
    let runtime = native_reference_runtime();
    let interaction = frame_maintenance_test_owner(121, 0).interaction.take().expect("interaction fixture");
    {
        let mut owner = runtime.try_lock().expect("runtime owner");
        owner.interaction = Some(interaction);
        let checked_out = owner.check_out_interaction("presenter-return-law").expect("exact checkout owner");
        let cursor = FrameDeferredCursor::new(FrameActionOwners::default(), false, false, false, false, 121, semio_framework_job::root_cancel_token());
        let mut queue = runtime.0.completions.lock().expect("runtime completion mailbox lock");
        assert!(queue.reserve_interaction());
        queue.finish(returned_completion(121, RuntimeApply::ResumeFrameDeferred { interaction: Some(checked_out), cursor: Some(cursor) }));
    }

    let mut original = OriginalRendererFixture::new();
    assert_eq!(original.turn(|cx| runtime.restore_presenter_interaction_step(cx)).unwrap(), PresenterInteractionStep::Restored);
    assert!(runtime.try_lock().expect("restored runtime").interaction_available(), "the presenter-only step returns the exact checked-out state");
    let mut completion = runtime.0.completions.lock().expect("runtime completion mailbox lock").take_at(0).expect("deferred cursor remains queued");
    assert!(!completion.restores_interaction);
    let RuntimeApply::ResumeFrameDeferred { interaction, cursor } = &mut completion.apply else { panic!("exact deferred continuation") };
    assert!(interaction.is_none(), "the returned state cannot be applied twice");
    let mut cursor = cursor.take().expect("the deferred cursor remains owned for post-presentation apply");
    cursor.begin_close();
    while !original.turn(|cx| cursor.close_step(cx)).unwrap() {}
    assert!(cursor.terminal_is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn original_mailbox_apply_and_removal_progress_with_one_unit_loans() {
    let runtime = native_reference_runtime();
    assert!(runtime.0.completions.lock().unwrap().enqueue(RuntimeCompletion { key: Some("window-metrics"), revision: 121, requires_interaction: false, restores_interaction: false, apply: RuntimeApply::Resize { width: 640.0, height: 480.0, dpr: 1.0 } }));
    let mut original = OriginalRendererFixture::new();
    for turn in 0..2 {
        assert!(original.turn(|cx| { let deadline = cx.deadline_us(); cx.with_original_work_loan(1, deadline, |cx| runtime.apply_pending_step(cx)) }).unwrap());
        let queue = runtime.0.completions.lock().unwrap();
        assert_eq!(queue.len(), usize::from(turn == 0));
        if turn == 0 { assert!(matches!(queue.ready.front().unwrap().apply, RuntimeApply::Applied)); }
    }
    assert_eq!(runtime.0.applied_window_metrics_revision.load(Ordering::Acquire), 121);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn original_recovery_returns_the_same_interaction_and_cursor_before_unique_cell_release() {
    let runtime = native_reference_runtime();
    let owner = frame_maintenance_test_owner(241, 1);
    let identity = owner.interaction.as_ref().unwrap().shell.plugin_filter.as_ptr();
    let cell = Arc::new(FrameExecutionOwnerCell::new(owner));
    let witness = cell.clone();
    *runtime.0.original_recovery.lock().unwrap() = Some(FrameOriginalRecovery::Maintenance(FrameMaintenanceRefusal { owner: cell, reservation_live: false }));
    let mut original = OriginalRendererFixture::new();
    let before = original.receipt();
    assert!(!original.limited(semio_framework_value::RetainedCloneGrant { maximum_items: 0, ..renderer_law_stock() }, |cx| { let mut owner = runtime.try_lock().unwrap(); runtime.recover_original_frame_step(&mut owner, cx) }).unwrap());
    assert_eq!(original.receipt(), before);
    assert!(original.turn(|cx| { let mut owner = runtime.try_lock().unwrap(); runtime.recover_original_frame_step(&mut owner, cx) }).unwrap());
    assert_eq!(runtime.try_lock().unwrap().interaction.as_ref().unwrap().shell.plugin_filter.as_ptr(), identity);
    assert!(original.turn(|cx| { let mut owner = runtime.try_lock().unwrap(); runtime.recover_original_frame_step(&mut owner, cx) }).unwrap());
    assert!(runtime.try_lock().unwrap().pending_frame_deferred.is_some());
    let (_, denied_heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| original.turn(|cx| { let mut owner = runtime.try_lock().unwrap(); runtime.recover_original_frame_step(&mut owner, cx) }).unwrap());
    assert_eq!((denied_heap.requested_bytes, denied_heap.released_bytes), (0, 0));
    assert!(runtime.0.original_recovery.lock().unwrap().is_some());
    drop(witness);
    let before = original.receipt();
    let (released, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| original.turn(|cx| { let mut owner = runtime.try_lock().unwrap(); runtime.recover_original_frame_step(&mut owner, cx) }).unwrap());
    assert!(released);
    let after = original.receipt();
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, semio_framework_value::shared_retirement_allocation_bytes::<FrameExecutionOwnerCell<FrameMaintenanceOwner>>()));
    assert_eq!(after.released_bytes - before.released_bytes, heap.released_bytes);
    assert!(runtime.0.original_recovery.lock().unwrap().is_none());
    for _ in 0..32768 { if original.turn(|cx| runtime.close_input_step(cx)).unwrap() { return; } }
    panic!("returned original cursor did not finish bounded close");
}

#[test]
fn native_binary_owns_exactly_one_entrypoint_driver() {
    assert_eq!(BINARY_SOURCE().matches(concat!("block", "_on(")).count(), 1);
    assert_eq!(BINARY_SOURCE().matches("drive_native_entrypoint(").count(), 3);
}

#[test]
fn manifest_has_no_retired_direct_edges() {
    assert!(!MANIFEST_SOURCE().contains(concat!("poll", "ster =")));
    assert!(!MANIFEST_SOURCE().contains(concat!("wasm-bindgen", "-test")));
    assert!(!MANIFEST_SOURCE().lines().any(|line| line.trim_start().starts_with("naga =")));
    assert!(!MANIFEST_SOURCE().lines().any(|line| line.trim_start().starts_with("rfd =")));
    assert!(!MANIFEST_SOURCE().lines().any(|line| line.trim_start().starts_with("ureq =")));
}

#[test]
fn runtime_dispatch_cursor_merges_retained_pointer_by_input_generation() {
    let pointer = ui_host::PointerMoveSample {
        pointer: ui_render::PointerInfo { id: ui_render::PointerId(1), kind: ui_render::PointerKind::Mouse, pressure: None, tilt: None },
        x: 1.0,
        y: 2.0,
        modifiers: ui_render::EventModifiers { shift: true, ..Default::default() },
        generation: ui_host::InputGeneration(2),
    };
    let scroll = ui_host::DiscreteEvent { event: ui_render::DispatchEvent::Scroll { x: 3.0, y: 4.0, delta_x: 5.0, delta_y: 6.0, modifiers: ui_render::EventModifiers { ctrl: true, ..Default::default() } }, generation: ui_host::InputGeneration(1) };
    let key = ui_host::DiscreteEvent { event: ui_render::DispatchEvent::KeyDown { key: "A".to_string(), modifiers: ui_render::EventModifiers::default() }, generation: ui_host::InputGeneration(3) };
    let mut events = ui_host::DrainedEvents { pointer_move: Some(pointer), ..Default::default() };
    events.discrete[0] = Some(scroll);
    events.discrete[1] = Some(key);
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);let original=caller.turn(super::RuntimeDispatchSource::admit).unwrap().unwrap();
    let mut cursor = RuntimeDispatchCursor::new(events,original);

    assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::Scroll { .. })));
    assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::PointerMove { .. })));
    assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::KeyDown { .. })));
    assert!(cursor.take_next().is_none());
    assert!(caller.turn(|cx|cursor.close_step(cx)).unwrap());
    assert!(cursor.terminal_is_empty());caller.finish();
}

#[test]
fn runtime_dispatch_cursor_retires_one_owned_event_per_step() {
    let mut events = ui_host::DrainedEvents::default();
    for (index, slot) in events.discrete.iter_mut().enumerate() {
        *slot = Some(ui_host::DiscreteEvent { event: ui_render::DispatchEvent::Paste { text: "x".repeat(4096) }, generation: ui_host::InputGeneration(index as u64) });
    }
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);let original=caller.turn(super::RuntimeDispatchSource::admit).unwrap().unwrap();
    let mut cursor = RuntimeDispatchCursor::new(events,original);
    for _ in 0..cursor.events.discrete.len() {
        assert!(!caller.turn(|cx|cursor.close_step(cx)).unwrap());
    }
    assert!(caller.turn(|cx|cursor.close_step(cx)).unwrap());
    assert!(cursor.terminal_is_empty());caller.finish();
}

#[test]
fn frame_deferred_cursor_advances_one_owned_operation_in_order() {
    let mut caller = crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let mut actions = FrameActionOwners::default();
    assert!(actions.try_push(ActionDescriptor { controller_id: "a".to_string(), action: "one".to_string(), args: None }).is_ok());
    assert!(actions.try_push(ActionDescriptor { controller_id: "b".to_string(), action: "two".to_string(), args: None }).is_ok());
    let mut cursor = FrameDeferredCursor::new(actions, true, true, true, true, 1, caller.alias());
    assert_eq!(cursor.checkout_site(), "frame-deferred-shell-maintenance");
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::ShellMaintenance)));
    assert_eq!(cursor.checkout_site(), "frame-deferred-pump-sync");
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::PumpSync)));
    assert_eq!(cursor.checkout_site(), "frame-deferred-action");
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::Action(action)) if action.descriptor.action == "one"));
    assert_eq!(cursor.checkout_site(), "frame-deferred-action");
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::Action(action)) if action.descriptor.action == "two"));
    assert_eq!(cursor.checkout_site(), "frame-deferred-tutorial-flush");
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::FlushTutorial)));
    assert_eq!(cursor.checkout_site(), "frame-deferred-settle");
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::Settle)));
    assert_eq!(cursor.checkout_site(), "frame-deferred-empty");
    assert!(cursor.take_next().is_none());
    while !caller.turn(|cx|cursor.close_step(cx)).unwrap() {}
    assert!(cursor.terminal_is_empty());caller.finish();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn renderer_input_retirement_drains_ready_handback_resident_cursor_and_staged_actions() {
    let runtime = native_reference_runtime();
    let mut interaction = frame_maintenance_test_owner(141, 0).interaction.take().unwrap();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🧾️correlated-action-receipts/🔣️.json")).unwrap();
    let rows = fixture["receipts"].as_array().unwrap();
    let mut groups = Vec::new();
    for pair in rows.chunks(2) {
        let mut batch = interaction.input.reserve_actions(pair.len(), pair.len() * 128).unwrap();
        for row in pair {
            batch
                .action("writer", row["action"].as_str().unwrap(), 128, |builder| {
                    builder.set_receipt(semio_framework_ui_action::ActionQueueReceipt { source: semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor,
                        token: std::num::NonZeroU64::new(row["token"].as_u64().unwrap()).unwrap(),
                        member: row["member"].as_u64().unwrap() as u8,
                        abort_correlation_on_error: row["abort"].as_bool().unwrap(),
                    })
                })
                .unwrap();
        }
        batch.publish().unwrap();
        let mut actions = FrameActionOwners::default();
        for _ in pair {
            assert!(matches!(transfer_frame_input_action(&mut interaction.input, &mut actions), Ok(FrameInputActionStep::Pending | FrameInputActionStep::Transferred)));
        }
        groups.push(actions);
    }
    let resident = groups.pop().unwrap();
    let handback = groups.pop().unwrap();
    let mut batch = interaction.input.reserve_actions(2, 256).unwrap();
    for member in 0..2 {
        batch
            .action("writer", "textEdit", 128, |builder| builder.set_receipt(semio_framework_ui_action::ActionQueueReceipt { source: semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor, token: std::num::NonZeroU64::new(fixture["stagedToken"].as_u64().unwrap()).unwrap(), member, abort_correlation_on_error: member == 0 }))
            .unwrap();
    }
    batch.publish().unwrap();
    let mut staged = FrameActionOwners::default();
    assert_eq!(transfer_frame_input_action(&mut interaction.input, &mut staged), Ok(FrameInputActionStep::Pending));
    {
        let mut owner = runtime.try_lock().unwrap();
        owner.interaction = Some(interaction);
        let returned = owner.check_out_interaction("receipt-retirement").unwrap();
        owner.frame_actions = staged;
        owner.pending_frame_deferred = Some(FrameDeferredCursor::new(resident, false, false, false, false, 141, semio_framework_job::root_cancel_token()));
        let mut queue = runtime.0.completions.lock().unwrap();
        assert!(queue.reserve_interaction());
        queue.finish(returned_completion(141, RuntimeApply::ResumeFrameDeferred { interaction: Some(returned), cursor: Some(FrameDeferredCursor::new(handback, false, false, false, false, 141, semio_framework_job::root_cancel_token())) }));
    }
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    assert!(!caller.turn(|cx|runtime.close_input_step(cx)).unwrap());
    assert!(runtime.try_lock().unwrap().interaction_available());
    for step in 0..64 {
        if caller.turn(|cx|runtime.close_input_step(cx)).unwrap() {
            assert!(step > 3);
            break;
        }
        assert!(step < 63, "retirement must finish within its finite owner count");
    }
    let owner = runtime.try_lock().unwrap();
    assert!(owner.pending_frame_deferred.is_none());
    assert!(owner.frame_actions.is_empty());
    assert!(owner.interaction.as_ref().unwrap().input.terminal_is_empty());
    assert_eq!(runtime.0.completions.lock().unwrap().len(), 0);
    caller.finish();
}

#[test]
fn frame_deferred_cancel_retires_one_action_per_step() {
    let mut caller = crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let mut actions = FrameActionOwners::default();
    for index in 0..WORLD3D_DEADLINE_CAPACITY {
        assert!(actions.try_push(ActionDescriptor { controller_id: index.to_string(), action: "cancel".to_string(), args: None }).is_ok());
    }
    let mut cursor = FrameDeferredCursor::new(actions, false, true, false, false, 1, caller.alias());
    let mut turns = 0;
    while !cursor.terminal_is_empty() {
        let before = cursor.actions.len;
        caller.turn(|cx| cursor.close_step(cx)).unwrap();
        assert!(before - cursor.actions.len <= 1, "one original callback cannot discard two admitted actions");
        turns += 1;
        assert!(turns < 32768, "all descriptor scaffolds and original fields must reach paid close");
    }
    assert!(cursor.terminal_is_empty());
    caller.finish();
}

#[test]
fn catalogue_terminal_pair_never_partially_enters_the_frame_action_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🛒️canvas-catalogue-terminal/🔣️.json")).unwrap();
    let frame = &fixture["frameOwner"];
    assert_eq!(WORLD3D_DEADLINE_CAPACITY, frame["capacity"].as_u64().unwrap() as usize);
    let mut actions = FrameActionOwners::default();
    for index in 0..frame["occupiedBeforePair"].as_u64().unwrap() as usize {
        actions.try_push(ActionDescriptor { controller_id: "fixture".into(), action: format!("occupied-{index}"), args: None }).unwrap();
    }
    let mut input = InputState::<ActionDescriptor>::default();
    let mut batch = input.reserve_actions(2, 512).unwrap();
    batch
        .action("layout-play", "canvasDragLeave", 256, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), "canvas.catalogue.terminal")?;
            builder.end_container()
        })
        .unwrap();
    batch
        .action("layout-play", "canvasDrop", 256, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), "canvas.catalogue.terminal")?;
            builder.string(Some("dragData"), "{\"kind\":\"rect\"}")?;
            builder.end_container()
        })
        .unwrap();
    batch.publish().unwrap();
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Deferred), "expected capacity pressure defers the whole source batch without faulting the frame");
    assert_eq!(input.take_action_batch_len_step(), Ok(Some(2)), "refusal retains the complete source batch at its original owner");
    assert_eq!(actions.pop_front().unwrap().descriptor.action, "occupied-0", "the previously published FIFO remains independently drainable");
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Pending), "one opportunity materializes only the first member into the staged owner");
    assert_eq!(input.take_action_batch_len_step(), Ok(Some(1)), "the unpublished drop remains source-owned between opportunities");
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Transferred));
    assert_eq!(input.take_action_batch_len_step(), Ok(None));
    let mut published = Vec::new();
    while let Some(action) = actions.pop_front() {
        published.push(action.descriptor.action);
    }
    assert_eq!(published.first().map(String::as_str), Some("occupied-1"));
    assert_eq!(published[published.len() - 2..].iter().map(String::as_str).collect::<Vec<_>>(), ["canvasDragLeave", "canvasDrop"]);
    assert_eq!(frame["expectedPairAdmission"], "refusedWhole");
}

#[test]
fn a_fault_after_staging_leave_retires_the_source_drop_before_a_later_single_dispatches() {
    let mut input = InputState::<ActionDescriptor>::default();
    let mut terminal = input.reserve_actions(2, 64).unwrap();
    terminal.action("fixture", "canvasDragLeave", 32, |_| Ok(())).unwrap();
    terminal.action("fixture", "canvasDrop", 32, |_| Ok(())).unwrap();
    terminal.publish().unwrap();
    input.reserve_action("fixture", "later", 16).unwrap().publish().unwrap();
    let mut actions = FrameActionOwners::default();
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Pending));
    input.record_action_fault(semio_framework_ui_action::BoundedActionFault::ByteCredits);
    let mut faulted = false;
    for _ in 0..ui_wgpu::wgpu::action::ACTION_BATCH_ITEM_CAPACITY * 2 + 2 {
        match transfer_frame_input_action(&mut input, &mut actions) {
            Err("bounded frame input action batch source faulted") => {
                faulted = true;
                break;
            }
            Ok(FrameInputActionStep::Pending) => {}
            other => panic!("fault retirement published or deferred a terminal member: {other:?}"),
        }
    }
    assert!(faulted, "the batch fault becomes observable after bounded staged/source retirement");
    assert!(actions.is_empty(), "no terminal prefix entered the dispatch ledger");
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Transferred));
    assert_eq!(actions.pop_front().unwrap().descriptor.action, "later", "the independent successor remains the next dispatchable owner");
    assert_eq!(input.take_action_batch_len_step(), Ok(None));
}

#[test]
fn frame_deferred_cancel_token_closes_one_exact_owner_per_grant() {
    let mut caller = crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let mut actions = FrameActionOwners::default();
    assert!(actions.try_push(ActionDescriptor { controller_id: "cancel-owner".to_string(), action: "one".to_string(), args: None }).is_ok());
    let mut cursor = FrameDeferredCursor::new(actions, true, true, true, false, 17, caller.alias());
    caller.turn(|cx| { cx.original_cancel_token().cancel_now(); Ok(()) }).unwrap();
    assert!(cursor.cancel.as_ref().is_some_and(|cancel| cancel.is_cancelled_now()));
    cursor.begin_close();
    let mut turns = 0;
    while !cursor.terminal_is_empty() { caller.turn(|cx| cursor.close_step(cx)).unwrap(); turns += 1; assert!(turns < 1024); }
    assert!(turns > 4, "descriptor fields, cancellation return and scalar removal have distinct original frontiers");
    assert!(cursor.terminal_is_empty());
    caller.finish();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn frame_maintenance_authority_refuses_aba_and_releases_the_exact_generation() {
    let authority = FrameMaintenanceAuthority::new();
    assert!(authority.try_reserve(41));
    assert!(authority.is_live(41));
    assert!(!authority.try_reserve(42));
    assert!(!authority.release(42));
    assert!(authority.is_live(41));
    assert!(authority.release(41));
    assert!(!authority.is_live(41));
    assert!(authority.try_reserve(42));
    assert!(authority.release(42));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn frame_maintenance_refusal_cell_recovers_exact_identity_without_blocking() {
    let owner = Box::new(73u64);
    let identity = owner.as_ref() as *const u64;
    let cell = FrameExecutionOwnerCell::new(owner);
    let Some(owner) = cell.try_take() else { panic!("exact owner admission") };
    assert_eq!(owner.as_ref() as *const u64, identity);
    assert!(cell.try_take().is_none());
    assert!(cell.try_restore(owner).is_ok());
    let Some(owner) = cell.try_take() else { panic!("exact owner handback") };
    assert_eq!(owner.as_ref() as *const u64, identity);
    assert!(cell.try_take().is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn frame_maintenance_cancel_and_stale_each_close_one_populated_owner_per_grant() {
    let mut caller = crate::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let mut cancelled_actions = FrameActionOwners::default();
    assert!(cancelled_actions.try_push(ActionDescriptor { controller_id: "cancelled".to_string(), action: "one".to_string(), args: None }).is_ok());
    let mut cancelled = FrameDeferredCursor::new(cancelled_actions, false, false, false, false, 81, caller.alias());
    caller.turn(|cx| { cx.original_cancel_token().cancel_now(); Ok(()) }).unwrap();
    assert_eq!(frame_maintenance_terminal_fault(cancelled.cancel.as_ref().is_some_and(|cancel| cancel.is_cancelled_now()), false, false), Some("frame maintenance cancelled"));
    cancelled.begin_close();
    let before = cancelled.actions.len;
    assert!(!caller.turn(|cx| cancelled.close_step(cx)).unwrap());
    assert_eq!(before - cancelled.actions.len, 1);
    while !caller.turn(|cx| cancelled.close_step(cx)).unwrap() {}
    assert!(cancelled.terminal_is_empty());

    let authority = FrameMaintenanceAuthority::new();
    assert!(authority.try_reserve(82));
    assert!(authority.release(82));
    let mut stale_actions = FrameActionOwners::default();
    assert!(stale_actions.try_push(ActionDescriptor { controller_id: "stale".to_string(), action: "one".to_string(), args: None }).is_ok());
    let mut stale = FrameDeferredCursor::new(stale_actions, false, false, false, false, 82, caller.alias());
    assert_eq!(frame_maintenance_terminal_fault(false, !authority.is_live(stale.generation), false), Some("frame maintenance generation became stale"));
    stale.begin_close();
    let before = stale.actions.len;
    assert!(!caller.turn(|cx| stale.close_step(cx)).unwrap());
    assert_eq!(before - stale.actions.len, 1);
    while !caller.turn(|cx| stale.close_step(cx)).unwrap() {}
    assert!(stale.terminal_is_empty());
    caller.finish();
}

#[cfg(not(target_arch = "wasm32"))]
fn frame_maintenance_test_owner(generation: u64, actions: usize) -> FrameMaintenanceOwner {
    let mut action_owners = FrameActionOwners::default();
    for index in 0..actions {
        assert!(action_owners.try_push(ActionDescriptor { controller_id: format!("owner-{index}"), action: "close".to_string(), args: None }).is_ok());
    }
    let interaction = AppInteractionState {
        shell: ShellState::new(Vec::new(), "maintenance-owner".to_string(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),
        input: InputState::default(),
        theme: Theme::default(),
        theme_dark: false,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        pointer_capture: semio_framework_os_renderer_wgpu_shell_input::PointerCapture::default(),
        modifiers: PointerModifiers::default(),
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        text_streams: std::array::from_fn(|_| None),
        text_fault: None,
        frame_fault: None,
        text_cancel_pending: false,
        last_sync_pump_ms: 0.0,
    };
    FrameMaintenanceOwner { interaction: Some(interaction), cursor: Some(FrameDeferredCursor::new(action_owners, true, true, true, false, generation, semio_framework_job::root_cancel_token())), deadline_ms: Some(u64::MAX), restart: true }
}
