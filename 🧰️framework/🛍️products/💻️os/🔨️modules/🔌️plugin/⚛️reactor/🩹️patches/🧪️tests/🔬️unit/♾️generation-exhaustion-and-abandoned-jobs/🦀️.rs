/// ♾️ Generation exhaustion and abandoned jobs: the maximum generation issued once, terminal saturation before the maximum generation, abandoned reconcile jobs, closing terminals and deferred surfaces awaiting the host acknowledgement.

#[test]
fn generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    {
        let mut state = tracker.state.borrow_mut();
        state.next_generation = u64::MAX - 2;
    }
    assert_eq!(tracker.begin("61:first".into(), leaf("root", "first")).expect("near maximum"), u64::MAX - 1);
    assert_eq!(tracker.retain_unadmitted("61:maximum".into(), leaf("root", "maximum")).expect("maximum once"), u64::MAX);
    let refused = tree_with_owned_child("post-maximum");
    let refused_pointer = refused.root.children.get(0).unwrap().key.as_ptr();
    let (_, refused) = tracker.begin("61:refused".into(), refused).expect_err("first post-maximum refuses");
    assert_eq!(refused.root.children.get(0).unwrap().key.as_ptr(), refused_pointer);
    let repeated = tree_with_owned_child("repeated");
    let repeated_pointer = repeated.root.children.get(0).unwrap().key.as_ptr();
    let (_, repeated) = tracker.retain_unadmitted("61:repeated".into(), repeated).expect_err("repeated refusal is stable");
    assert_eq!(repeated.root.children.get(0).unwrap().key.as_ptr(), repeated_pointer);
    let state = tracker.state.borrow();
    assert_eq!(state.next_generation, u64::MAX);
    assert!(state.generation_exhausted);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    {
        let mut state = tracker.state.borrow_mut();
        state.next_generation = u64::MAX - 1;
        state.slots[0] = Some(SurfaceSlot {
            key: NativeCloseKey::fixture(62, 1),
            output_index: None,
            surface: ui_contract::SurfaceId::try_from("62:idle").expect("bounded surface fixture"),
            generation: u64::MAX - 1,
            acknowledged_revision: ui_contract::UiRevision::default(),
            cancelled:false,pending_render:false,pending_render_source:None,
            reconciler: Some(SurfaceReconciler::new(ui_contract::SurfaceId::try_from("62:idle").expect("bounded surface fixture"))),
            producer: None,
            job: None,
        });
    }
    saturate_terminals(&tracker, 62, 620_000);
    reject_current(&tracker, "62:idle");
    {
        let state = tracker.state.borrow();
        assert_eq!(state.next_generation, u64::MAX - 1);
        assert!(!state.generation_exhausted);
        assert!(state.slots[0].as_ref().is_some_and(|slot| slot.reconciler.is_some()));
    }
    let terminal = tracker.state.borrow_mut().terminals[0].take().expect("free one exact terminal reservation");
    drop(terminal);
    reject_current(&tracker, "62:idle");
    let state = tracker.state.borrow();
    assert_eq!(state.next_generation, u64::MAX);
    assert!(state.generation_exhausted);
    assert!(state.terminals.iter().flatten().any(|terminal| terminal.authority.generation() == u64::MAX));

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn abandoned_reconcile_job_closes_its_ready_output_so_the_tracker_can_idle() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    tracker.begin("5:main".into(), tree_with_owned_child("a")).expect("admitted");
    for _ in 0..4_096 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        if tracker.state.borrow().slots.iter().flatten().any(|slot| slot.job.is_some()) {
            break;
        }
    }
    {
        let mut state = tracker.state.borrow_mut();
        let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface.as_ref() == "5:main").expect("mounted test surface");
        assert!(slot.job.is_some(), "the fixture must reach the reconcile job before a newer root arrives");
        slot.reconciler = Some(SurfaceReconciler::new(slot.surface.clone()));
    }
    for _ in 0..4_096 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        tracker.close_step(cx).expect("original patch fixture close step");
        let _ = tracker.take_deferred_ready();
        if !tracker.has_work() {
            break;
        }
    }
    assert!(!tracker.has_work(), "an abandoned job must not strand its reserved output: {}", tracker.debug_state());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn a_closing_terminal_does_not_wait_behind_sixty_three_empty_slots_per_unit() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let generation = tracker.begin("6:main".into(), tree_with_owned_child("a")).expect("admitted");
    let patch = finish(&tracker,cx).expect("first publication");
    close_test_patch(patch);
    assert!(tracker.mark_rejected("6:main", generation), "the published generation must be rejectable into a terminal");
    let mut steps = 0;
    while tracker.has_work() {
        tracker.close_step(cx).expect("original patch fixture close step");
        let _ = tracker.take_deferred_ready();
        steps += 1;
        assert!(steps < 1_024, "a single closing terminal must drain one unit per close step, not one per 64: {}", tracker.debug_state());
    }

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn a_deferred_surface_awaiting_the_hosts_acknowledgement_does_not_hold_more_work() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    tracker.begin("9:main".into(), leaf("root", "a")).expect("admitted");
    for _ in 0..4_096 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        tracker.close_step(cx).expect("original patch fixture close step");
        if !tracker.state.borrow().slots.iter().flatten().any(|slot| slot.producer.is_some() || slot.job.is_some()) {
            break;
        }
    }
    {
        let mut state = tracker.state.borrow_mut();
        for ready in state.ready.iter_mut().flatten() {
            ready.published = true;
            ready.closing = true;
        }
        let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface.as_ref() == "9:main").expect("mounted test surface");
        assert!(slot.producer.is_none() && slot.job.is_none(), "the fixture must publish before the host falls behind: {}", tracker.debug_state());
        let revision = slot.reconciler.as_ref().expect("published canonical root").revision();
        slot.acknowledged_revision = ui_contract::UiRevision(revision.0.saturating_sub(1));
    }
    assert!(tracker.defer(ui_contract::SurfaceId::try_from("9:main").expect("bounded surface")).is_ok());
    assert!(tracker.take_deferred_ready().is_none(), "an unacknowledged revision keeps the surface deferred");
    assert!(tracker.has_work(), "the deferred surface is still tracked work: {}", tracker.debug_state());
    assert!(!tracker.has_publishable_work(), "a host-blocked deferred surface must not hold the actor in more-work: {}", tracker.debug_state());
    {
        let mut state = tracker.state.borrow_mut();
        let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface.as_ref() == "9:main").expect("mounted test surface");
        slot.acknowledged_revision = slot.reconciler.as_ref().expect("published canonical root").revision();
    }
    assert!(tracker.has_publishable_work(), "an acknowledged deferred surface is re-admittable work");
    assert_eq!(tracker.take_deferred_ready().as_ref().map(AsRef::as_ref), Some("9:main"));

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}
