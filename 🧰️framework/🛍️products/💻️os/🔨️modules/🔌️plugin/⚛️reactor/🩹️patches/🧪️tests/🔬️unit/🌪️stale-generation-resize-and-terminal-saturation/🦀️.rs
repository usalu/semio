/// 🌪️ Stale generation, resize and terminal saturation: the retrievable stale generation fault, the resize storm, publication order, actor close retirement and the terminal-full conversions.

#[test]
fn stale_generation_fault_is_publicly_retrievable() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let generation = tracker.begin("main".into(), leaf("root", "a")).expect("admitted");
    reject_current(&tracker, "main");
    let mut terminal = tracker.take_terminal(generation).expect("terminal owner");
    for _ in 0..32 {
        if terminal.close_step() && terminal.terminal_is_empty() {
            break;
        }
    }
    assert!(terminal.terminal_is_empty());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn resize_storm_coalesces_to_one_deferred_surface_owner() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let generation = tracker.begin("7:main".into(), leaf("root", "a")).expect("admitted");
    for _ in 0..128 {
        assert!(tracker.defer(ui_contract::SurfaceId::try_from("7:main").expect("bounded surface")).is_ok());
    }
    assert!(tracker.take_deferred_ready().is_none());
    reject_current(&tracker, "7:main");
    let mut terminal = tracker.take_terminal(generation).expect("cancelled owner");
    for _ in 0..32 {
        if terminal.close_step() && terminal.terminal_is_empty() {
            break;
        }
    }
    assert_eq!(tracker.take_deferred_ready().as_ref().map(AsRef::as_ref), Some("7:main"));
    assert!(tracker.take_deferred_ready().is_none());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn effects_publish_in_admission_order_even_when_later_tree_finishes_first() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    tracker.begin("1:first".into(), leaf("root", "a")).expect("first");
    tracker.begin("1:second".into(), leaf("root", "b")).expect("second");
    let first = finish(&tracker,cx).expect("first ready");
    let second = finish(&tracker,cx).expect("second ready");
    assert_eq!(first.surface.0.as_str(), "1:first");
    assert_eq!(second.surface.0.as_str(), "1:second");

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn actor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let old = tracker.begin("9:first".into(), leaf("root", "a")).expect("old generation");
    reject_current(&tracker, "9:first");
    let terminal = tracker.take_terminal(old).expect("old terminal");
    tracker.begin("9:first".into(), leaf("root", "b")).expect("reopened generation");
    let terminal = tracker.resume_terminal(terminal).expect_err("old generation cannot mutate reopened slot");
    let mut terminal = terminal;
    for _ in 0..32 {
        if terminal.close_step() && terminal.terminal_is_empty() {
            break;
        }
    }
    close_instance_to_empty(&tracker, 9,cx);
    assert!(tracker.terminal_is_empty());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn close_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️surface-close.json")).unwrap();
    let instance = fixture["instance"].as_u64().unwrap() as u32;
    let surface = |key: &str| fixture["surfaces"][key].as_str().unwrap();
    let tracker = PatchTracker::new();
    tracker.begin(surface("ready").into(), leaf("root", "ready")).expect("ready source");
    for _ in 0..128 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        if tracker.state.borrow().ready.iter().any(Option::is_some) {
            break;
        }
    }
    assert!(tracker.state.borrow().ready.iter().any(Option::is_some));
    assert!(tracker.defer(ui_contract::SurfaceId::try_from(surface("deferred")).expect("bounded surface")).is_ok());
    tracker.retain_unadmitted(surface("queued").into(), leaf("root", "queued")).expect("unadmitted");
    tracker.begin(surface("active").into(), leaf("root", "active")).expect("active");
    {
        let mut state = tracker.state.borrow_mut();
        let target = state.terminals.iter_mut().find(|slot| slot.is_none()).expect("terminal capacity");
        let surface = ui_contract::SurfaceId::try_from(surface("terminal")).expect("bounded surface fixture");
        *target =
            Some(TerminalSlot { key: NativeCloseKey::fixture(instance, 1), instance: Some(instance), authority: SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(surface), 90_012).expect("fixed terminal admission"), close: true });
    }
    let key = NativeCloseKey::fixture(instance, 1);
    tracker.reserve_close_instance(key).expect("exact close reservation");
    tracker.activate_close_instance(key).expect("activate retained close");
    assert_eq!(tracker.take_ready_patch().is_some(), fixture["stalePatch"].as_bool().unwrap());
    for _ in 0..16_384 {
        tracker.close_step(cx).expect("original patch fixture close step");
        if tracker.close_instance_complete(key).expect("exact close receipt") {
            break;
        }
    }
    assert!(!tracker.terminal_is_empty(), "completed close receipt remains owned until exact ACK");
    tracker.release_close_instance(key).expect("final ACK releases close slot");
    assert_eq!(tracker.terminal_is_empty(), fixture["terminalEmpty"].as_bool().unwrap());
    assert_eq!(tracker.take_ready_patch().is_some(), fixture["stalePatch"].as_bool().unwrap());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    {
        let mut state = tracker.state.borrow_mut();
        for (index, terminal) in state.terminals.iter_mut().enumerate() {
            *terminal = Some(TerminalSlot {
                key: NativeCloseKey::fixture(44, 1),
                instance: Some(44),
                authority: SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(ui_contract::SurfaceId::try_from(format!("44:terminal-{index}")).expect("bounded surface fixture")), 100_000 + index as u64)
                    .expect("fixed terminal admission"),
                close: false,
            });
        }
    }
    let generation = tracker.begin("44:active".into(), leaf("root", "active")).expect("active");
    reject_current(&tracker, "44:active");
    assert!(tracker.state.borrow().slots.iter().flatten().find(|slot| slot.surface.as_ref() == "44:active").is_some_and(|slot| slot.job.is_some()), "saturation retains the exact job locally");
    let mut released = tracker.take_terminal(100_000).expect("free one terminal grant");
    for _ in 0..32 {
        if released.close_step() && released.terminal_is_empty() {
            break;
        }
    }
    assert!(released.terminal_is_empty());
    reject_current(&tracker, "44:active");
    assert!(tracker.take_terminal(generation).is_some(), "freed capacity receives the original generation");

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    saturate_terminals(&tracker, 51, 510_000);
    tracker.retain_unadmitted("51:queued".into(), leaf("root", "queued")).expect("pre-admitted owner");
    close_instance_to_empty(&tracker, 51,cx);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn terminal_full_plus_matching_rejected_advances_capacity_before_conversion() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let identifier_bytes = semio_framework_ui_runtime::SurfaceReconcileLimits::default().max_identifier_bytes;
    tracker.begin(format!("52:{}", "x".repeat(identifier_bytes)), leaf("root", "rejected")).expect("fixed surface slot");
    assert!(tracker.state.borrow().rejected.iter().any(Option::is_some), "identifier cap produces an exact rejected owner");
    saturate_terminals(&tracker, 52, 520_000);
    close_instance_to_empty(&tracker, 52,cx);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn terminal_full_plus_matching_surface_advances_capacity_before_conversion() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    {
        let mut state = tracker.state.borrow_mut();
        state.slots[0] = Some(SurfaceSlot {
            key: NativeCloseKey::fixture(53, 1),
            output_index: None,
            surface: ui_contract::SurfaceId::try_from("53:idle").expect("bounded surface fixture"),
            generation: 530_000,
            acknowledged_revision: ui_contract::UiRevision::default(),
            cancelled:false,pending_render:false,pending_render_source:None,
            reconciler: Some(SurfaceReconciler::new(ui_contract::SurfaceId::try_from("53:idle").expect("bounded surface fixture"))),
            producer: None,
            job: None,
        });
    }
    saturate_terminals(&tracker, 53, 531_000);
    close_instance_to_empty(&tracker, 53,cx);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}
