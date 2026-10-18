/// 🚪️ Mounted output admission laws: cancel and drop keep the close generation, the shared output pool refusal, partial producer step unwind, incomplete producer sources, the direct receiver lifetime, the close wait and concurrent trackers over one fixed pool.

#[test]
fn mounted_output_admission_cancel_and_drop_keep_the_original_close_generation() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let fixture = &fixture["cancelledAdmission"];
    for drop_grant in [false, true] {
        let tracker = PatchTracker::new();
        let surface = ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap();
        let grant = reserve(&tracker, surface.clone()).unwrap();
        let generation = grant.generation;
        if drop_grant {
            drop(grant);
        } else {
            grant.cancel();
        }
        let preserved = tracker.state.borrow().slots.iter().flatten().find(|slot| slot.surface == surface).is_some_and(|slot| slot.generation == generation && generation != 0);
        let revision = tracker.revision(surface.as_ref()).0;
        close_instance_to_empty(&tracker, 74,cx);
        assert_eq!(preserved, fixture["generationPreserved"].as_bool().unwrap());
        assert_eq!(revision, fixture["revision"].as_u64().unwrap());
        assert_eq!(tracker.terminal_is_empty(), fixture["terminal"].as_bool().unwrap());
    }

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut outputs = SurfaceReconcileOutputs::default();
    let mut reservations = Vec::new();
    for generation in 1..=fixture["entrySlots"].as_u64().unwrap() {
        reservations.push(outputs.try_reserve(generation, fixture["physicalGrant"].as_u64().unwrap() as usize).unwrap().unwrap());
    }
    let tracker = PatchTracker::new();
    let admitted = match reserve(&tracker, ui_contract::SurfaceId::try_from("71:output-admission").unwrap()) {
        Ok(grant) => {
            grant.cancel();
            true
        }
        Err(surface) => {
            assert_eq!(surface.as_ref(), "71:output-admission");
            false
        }
    };
    for owner in &mut reservations {
        while !owner.close_step(1).unwrap().complete {}
    }
    while !outputs.close_step(1, 4096).unwrap().complete {}
    close_instance_to_empty(&tracker, 71,cx);
    assert_eq!(admitted, fixture["extraInvocation"].as_bool().unwrap());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let tracker = PatchTracker::new();
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from("72:producer-unwind").unwrap()).unwrap();let mut source=Some(leaf("root", "owned-é"));assert!(grant.commit_source(&mut source,cx).unwrap(), "actual original source admitted");assert!(source.is_none());}
    let (index, rejected_index, generation, pointer) = {
        let state = tracker.state.borrow();
        let index = state.slots.iter().position(Option::is_some).unwrap();
        let slot = state.slots[index].as_ref().unwrap();
        let producer = slot.producer.as_ref().unwrap();
        (index, producer.rejected_index, slot.generation, producer.authority.as_ref() as *const _)
    };
    PANIC_AFTER_PRODUCER_STEP.with(|pending| pending.set(true));
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tracker.drive_one(cx).expect("original patch fixture semantic step")));
    assert!(caught.is_err());
    let retained = tracker.state.borrow().slots[index]
        .as_ref()
        .and_then(|slot| slot.producer.as_ref())
        .is_some_and(|producer| producer.authority.as_ref() as *const _ == pointer && producer.reservation.as_ref().is_some_and(|owner| owner.generation() == generation));
    if !retained {
        let mut state = tracker.state.borrow_mut();
        assert_eq!(state.rejected_reserved[rejected_index], Some(generation));
        state.rejected_reserved[rejected_index] = None;
    }
    close_instance_to_empty(&tracker, 72,cx);
    assert_eq!(retained, fixture["partialProducerUnwindRetainsOwner"].as_bool().unwrap(), "actual producer step must not remove the structural slot before invoking the child");

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_output_admission_incomplete_producer_sources_preserve_remaining_owners() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let tracker = PatchTracker::new();
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from("73:producer-source").unwrap()).unwrap();let mut source=Some(leaf("owned-root", "é"));assert!(grant.commit_source(&mut source,cx).unwrap(), "actual original source admitted");assert!(source.is_none());}
    let (index, generation, pointer, reconciler) = {
        let mut state = tracker.state.borrow_mut();
        let index = state.slots.iter().position(Option::is_some).unwrap();
        let slot = state.slots[index].as_mut().unwrap();
        let producer = slot.producer.as_mut().unwrap();
        for turn in 0..64 {
            if producer.authority.step(slot.generation, false, false) == ComponentTreeProducerStep::Complete {
                break;
            }
            assert!(turn < 63);
        }
        (index, slot.generation, producer.authority.as_ref() as *const _, producer.reconciler.take().unwrap())
    };
    tracker.drive_one(cx).expect("original patch fixture semantic step");
    tracker.drive_one(cx).expect("original patch fixture semantic step");
    let (reservation_retained, tree_retained) = {
        let mut state = tracker.state.borrow_mut();
        let result = if let Some(producer) = state.slots[index].as_mut().and_then(|slot| slot.producer.as_mut()) {
            assert_eq!(producer.authority.as_ref() as *const _, pointer);
            (producer.reservation.as_ref().is_some_and(|owner| owner.generation() == generation), producer.authority.take_complete().is_some_and(|tree| tree.root.key.as_str() == "owned-root"))
        } else if let Some(terminal) = state.producer_terminals.iter_mut().flatten().find(|terminal| terminal.authority.as_ref().is_some_and(|owner| owner.as_ref() as *const _ == pointer)) {
            (terminal.reservation.as_ref().is_some_and(|owner| owner.generation() == generation), terminal.authority.as_mut().unwrap().take_complete().is_some_and(|tree| tree.root.key.as_str() == "owned-root"))
        } else {
            (false, false)
        };
        state.slots[index].as_mut().unwrap().reconciler = Some(reconciler);
        result
    };
    close_instance_to_empty(&tracker, 73,cx);
    assert_eq!(reservation_retained && tree_retained, fixture["incompleteProducerSourcesPreserveRemainingOwners"].as_bool().unwrap(), "one missing source must not consume other completed owners");

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["capturedLifetime"];
    let instance = law["instance"].as_u64().unwrap() as u32;
    let key = NativeCloseKey::fixture(instance, law["original"].as_u64().unwrap());
    let foreign = NativeCloseKey::fixture(instance, law["reused"].as_u64().unwrap());
    let tracker = PatchTracker::new();
    let mut grant = tracker.reserve_mounted(ui_contract::SurfaceId::try_from(format!("{instance}:direct")).unwrap(), key).unwrap();
    let generation = grant.generation;
    let mut source=Some(leaf("owned-root", "界-é"));assert!(grant.commit_source(&mut source,cx).unwrap());assert!(source.is_none());
    PANIC_AFTER_OUTPUT_TRANSFER.with(|pending| pending.set(true));
    let mut caught = false;
    for _ in 0..65_536 {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tracker.drive_one(cx).expect("original patch fixture semantic step"))).is_err() {
            caught = true;
            break;
        }
    }
    assert!(caught, "actual live job-to-pool transfer callback ran");
    let exact_roots = {
        let state = tracker.state.borrow();
        let surface = state.slots.iter().flatten().find(|slot| slot.key == key).unwrap();
        let output = state.ready[surface.output_index.unwrap()].as_ref().unwrap();
        surface.job.is_some() && surface.reconciler.is_some() && output.published && output.reservation.is_none() && output.key == key && output.generation == generation
    };
    assert!(exact_roots);
    assert_eq!(tracker.reserve_close_instance(foreign).is_ok(), law["foreignCloseAccepted"].as_bool().unwrap());
    let mut target = None;
    assert!(tracker.take_ready_patch_into(foreign, generation, &mut target, 32768).is_err());
    assert!(tracker.take_ready_patch_into(key, generation + 1, &mut target, 32768).is_err());
    assert!(!tracker.take_ready_patch_into(key, generation, &mut target, 0).unwrap());
    let guard = tracker.state.borrow_mut();
    assert!(tracker.take_ready_patch_into(key, generation, &mut target, 32768).is_err());
    drop(guard);
    assert!(target.is_none());
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(tracker.take_ready_patch_into(key, generation, &mut target, 32768).unwrap());
        panic!("actual Pending receiver callback retains exact Ready");
    }));
    assert!(caught.is_err());
    assert_eq!(target.as_ref().unwrap().generation(), generation);
    assert_eq!(target.as_ref().unwrap().surface().unwrap().as_ref(), format!("{instance}:direct"));
    while !target.as_mut().unwrap().close_step_with_grant(1, 4096).unwrap().complete {}
    close_instance_to_empty(&tracker, instance,cx);
    assert_eq!(tracker.terminal_is_empty(), law["terminal"].as_bool().unwrap());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_output_admission_close_waits_for_the_original_uncommitted_grant() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let key = NativeCloseKey::fixture(76, 1);
    let mut grant = tracker.reserve_mounted(ui_contract::SurfaceId::try_from("76:grant").unwrap(), key).unwrap();
    tracker.reserve_close_instance(key).unwrap();
    tracker.activate_close_instance(key).unwrap();
    for _ in 0..8 {
        tracker.close_step(cx).expect("original patch fixture close step");
        assert!(!tracker.close_instance_complete(key).unwrap());
    }
    let mut source=Some(tree_with_owned_child("returned"));
    let pointer=source.as_ref().unwrap().root.children.get(0).unwrap().key.as_ptr();
    assert!(!grant.commit_source(&mut source,cx).expect("closing lifetime preserves source and grant"));
    assert_eq!(source.as_ref().unwrap().root.children.get(0).unwrap().key.as_ptr(),pointer);
    grant.cancel();
    let returned=source.take().unwrap();assert!(returned.native_resources.is_none());
    let mut returned=ui_contract::BuiltTreeRetirement::new(returned.root);
    while !returned.close_step(1,4096).unwrap().complete {}
    close_instance_to_empty(&tracker, 76,cx);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn mounted_output_admission_concurrent_trackers_share_one_fixed_pool_without_overadmission() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["concurrentAdmission"];
    let mut occupied = SurfaceReconcileOutputs::default();
    let mut reservations = Vec::new();
    for generation in 1..=law["occupied"].as_u64().unwrap() {
        reservations.push(occupied.try_reserve(generation, 32768).unwrap().unwrap());
    }
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
    let (tx, rx) = std::sync::mpsc::channel();
    let mut workers = Vec::new();
    for instance in [81, 82] {
        let barrier = barrier.clone();
        let tx = tx.clone();
        workers.push(std::thread::spawn(move || {
            let mut worker_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored concurrent instance caller");let mut worker_context=worker_caller.context();let cx=&mut worker_context;
            let tracker = PatchTracker::new();
            let mut surface = ui_contract::SurfaceId::try_from(format!("{instance}:concurrent")).unwrap();
            let key = NativeCloseKey::fixture(instance, 1);
            let mut admitted = None;
            barrier.wait();
            for _ in 0..64 {
                match tracker.reserve_mounted(surface, key) {
                    Ok(grant) => {
                        admitted = Some(grant);
                        break;
                    }
                    Err(returned) => {
                        assert_eq!(returned.as_ref(), format!("{instance}:concurrent"));
                        surface = returned;
                        std::thread::yield_now();
                    }
                }
            }
            tx.send(admitted.is_some()).unwrap();
            barrier.wait();
            if let Some(grant) = admitted {
                grant.cancel();
            }
            close_instance_to_empty(&tracker, instance,cx);
            let fuel=worker_context.fuel_remaining();drop(worker_context);worker_caller.retire(fuel).expect("concurrent instance caller paid source close");
        }));
    }
    barrier.wait();
    let accepted = usize::from(rx.recv().unwrap()) + usize::from(rx.recv().unwrap());
    barrier.wait();
    for worker in workers {
        worker.join().unwrap();
    }
    for reservation in &mut reservations {
        while !reservation.close_step(1).unwrap().complete {}
    }
    while !occupied.close_step(1, 4096).unwrap().complete {}
    assert_eq!(accepted, law["accepted"].as_u64().unwrap() as usize);
    let mut reused = SurfaceReconcileOutputs::default();
    let mut restored = Vec::new();
    for generation in 1..=64 {
        restored.push(reused.try_reserve(generation, 32768).unwrap().unwrap());
    }
    assert!(reused.try_reserve(65, 32768).unwrap().is_none());
    for reservation in &mut restored {
        while !reservation.close_step(1).unwrap().complete {}
    }
    while !reused.close_step(1, 4096).unwrap().complete {}

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}
