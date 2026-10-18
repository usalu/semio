/// 🔔️ Obsolete feedback, acks and reservation cap: the obsolete reconcile feedback owner, the first publication ack refusals, the exact tree owner at the cap plus one and the resident aggregate helpers.

#[test]
fn issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use semio_framework_plugin_reactor_lifecycle::PendingPatchAuthority;
    use semio_framework::kernel::{ActorInstanceLifetime, ActorUiPatchReceipt};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📨️pending/🧪️tests/🧫️fixtures/🩹️receipt.json")).unwrap();
    let surface = fixture["issued"]["surface"].as_str().unwrap();
    for rejection in [false, true] {
        let tracker = PatchTracker::new();
        let mut pending = PendingPatchAuthority::new();
        let first = tracker.begin(surface.into(), leaf("root", "first")).unwrap();
        let mut ready = None;
        for _ in 0..4096 {
            tracker.drive_one(cx).expect("original patch fixture semantic step");
            if let Some(owner) = tracker.take_ready_patch() {
                ready = Some(owner);
                break;
            }
        }
        pending.push_reconcile(ready.expect("real reconcile publication")).unwrap_or_else(|_| panic!("empty pending slot"));
        let patch = pending.take_one(65536).unwrap().unwrap();
        let issued = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 41, instance_id: 7, guest_lifetime: 3 }, patch_sequence: 8 };
        pending.stage_emission(issued, std::iter::once(&patch)).unwrap();
        pending.commit_emission();
        let revision = patch.revision.0;
        close_test_patch(patch);
        assert!(!tracker.mark_rejected(surface, first + 1), "future feedback cannot reset a current generation");
        for turn in 0..4096 {
            tracker.drive_one(cx).expect("original patch fixture semantic step");
            tracker.close_step(cx).expect("original patch fixture close step");
            if tracker.state.borrow().slots.iter().flatten().any(|slot| slot.surface.as_ref() == surface && slot.job.is_none() && slot.producer.is_none() && slot.output_index.is_none() && slot.reconciler.is_some()) {
                break;
            }
            assert!(turn < 4095);
        }
        let second = tracker.begin(surface.into(), leaf("root", "second")).expect("old published owner permits a later real render");
        assert!(second > first);
        let before = tracker.revision(surface);
        if rejection {
            assert!(pending.apply_issued_rejection(issued, surface, revision, |generation| tracker.mark_rejected(surface, generation)));
        } else {
            assert!(pending.apply_issued_ack(issued, surface, revision, 65536, |ack| tracker.mark_published_ack(ack)).unwrap());
        }
        assert_eq!(tracker.revision(surface), before);
        assert_eq!(tracker.state.borrow().slots.iter().flatten().find(|slot| slot.surface.as_ref() == surface).unwrap().generation, second);
        for turn in 0..65536 {
            if pending.close_step(1, 4096).unwrap() {
                break;
            }
            assert!(turn < 65535);
        }
        assert!(!pending.receipt_is_issued(issued));
        assert!(!pending.has_unpublished());
        assert!(pending.has_capacity());
        close_instance_to_empty(&tracker, 7,cx);
        eprintln!("obsolete real reconcile feedback rejection={} old={} current={} exact old owner retired", rejection, first, second);
    }

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn published_owner_first_ack_rejects_early_stale_duplicate_wrong_instance_and_aba_without_authority_loss() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let mut published = Some(published(&tracker, "71:ack",cx));
    let revision = published.as_ref().unwrap().revision().0;
    let mut ack = None;
    let admitted = semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::required_acknowledge_bytes();
    assert!(!semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::acknowledge_into(&mut published, &mut ack, "72:wrong", revision, admitted).unwrap());
    assert!(published.as_ref().unwrap().matches("71:ack", revision));
    assert!(semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::acknowledge_into(&mut published, &mut ack, "71:ack", revision, admitted).unwrap());
    {
        let mut state = tracker.state.borrow_mut();
        let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface.as_ref() == "71:ack").expect("published surface");
        slot.generation += 1;
    }
    assert!(tracker.mark_published_ack(ack.as_ref().unwrap()).unwrap(), "obsolete exact published owner can retire without mutating current generation");
    assert_eq!(ack.as_ref().unwrap().surface().unwrap().0.as_str(), "71:ack", "ABA refusal preserves the identical structural authority");
    {
        let mut state = tracker.state.borrow_mut();
        let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface.as_ref() == "71:ack").expect("published surface");
        slot.generation = ack.as_ref().unwrap().generation();
        slot.acknowledged_revision = ui_contract::UiRevision(revision);
    }
    assert!(!tracker.mark_published_ack(ack.as_ref().unwrap()).unwrap(), "duplicate ACK is inert");
    assert_eq!(ack.as_ref().unwrap().surface().unwrap().0.as_str(), "71:ack", "duplicate refusal preserves exact authority");
    close_ack(ack.take().unwrap());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

#[test]
fn cap_plus_one_returns_the_exact_tree_owner() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    {
        let mut state = tracker.state.borrow_mut();
        for index in 0..SURFACE_RECONCILE_ADMISSION_SLOTS {
            let surface = format!("{index}:main");
            state.slots[index] = Some(SurfaceSlot {
                key: NativeCloseKey::fixture(index as u32, 1),
                output_index: None,
                surface: ui_contract::SurfaceId::try_from(surface).expect("bounded surface fixture"),
                generation: index as u64 + 1,
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancelled:false,pending_render:false,pending_render_source:None,
                reconciler: Some(SurfaceReconciler::new(ui_contract::SurfaceId::try_from(format!("{index}:main")).expect("bounded surface fixture"))),
                producer: None,
                job: None,
            });
        }
    }
    let tree = tree_with_owned_child("exact");
    let pointer = tree.root.children.get(0).unwrap().key.as_ptr();
    let (_, returned) = tracker.begin("65:main".into(), tree).expect_err("cap + 1");
    assert_eq!(returned.root.children.get(0).unwrap().key.as_ptr(), pointer);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

/// 🎟️ Holds process-wide resident credit until the aggregate has less than ONE reconcile floor price left,
/// so a law can reach the refusal a reconcile reservation answers without mounting one surface per 8 MiB —
/// which is no longer possible now that a reservation is priced by its own body (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B58).
fn fill_resident_aggregate() -> Vec<ui_contract::UiResidentPermit> {
    let floor = semio_framework_ui_runtime::SURFACE_RECONCILE_FLOOR_BYTES;
    let mut remaining = ui_contract::UI_RESIDENT_AGGREGATE_BYTES - ui_contract::UiResidentPermit::snapshot().expect("readable ledger").bytes;
    let mut held = Vec::new();
    while remaining >= floor {
        let bytes = ui_contract::UI_RESIDENT_SURFACE_BYTES.min(remaining - floor + 1);
        let mut permit = None;
        assert_eq!(
            ui_contract::UiResidentPermit::try_reserve(ui_contract::UiResidentLimits { items: 1, bytes }, &mut permit, ui_contract::UiResidentPermit::required_reservation_bytes()),
            Ok(true),
            "an aggregate filler of {bytes} bytes must be admitted with {remaining} free"
        );
        held.push(permit.expect("admitted aggregate filler"));
        remaining -= bytes;
    }
    held
}

fn release_resident_aggregate(held: Vec<ui_contract::UiResidentPermit>) {
    for mut permit in held {
        assert!(permit.close_step(1).expect("exact filler return").complete);
    }
}

#[test]
fn mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["residentCapacity"];
    let aggregate = semio_framework_ui_runtime::SURFACE_RECONCILE_AGGREGATE_BYTES;
    let limits = semio_framework_ui_runtime::SurfaceReconcileLimits::default();
    let floor = semio_framework_ui_runtime::SURFACE_RECONCILE_FLOOR_BYTES;
    assert_eq!(aggregate, fixture["aggregateCeilingBytes"].as_u64().unwrap() as usize);
    assert_eq!(limits.max_bytes, law["reservationBytes"].as_u64().unwrap() as usize);
    for row in law["cases"].as_array().unwrap() {
        let available = aggregate.checked_sub(usize::try_from(row["fixedBytes"].as_u64().unwrap()).unwrap()).unwrap();
        assert_eq!(available / limits.max_bytes, usize::try_from(row["capacity"].as_u64().unwrap()).unwrap());
    }
    let tracker = PatchTracker::new();
    let grant = reserve(&tracker, ui_contract::SurfaceId::try_from("4:mounted").expect("bounded surface")).expect("fixed mounted reservation");
    let generation = grant.generation;
    assert!(tracker.state.borrow().unadmitted.iter().flatten().any(|owner| owner.generation == generation));
    let tree = leaf("root", "reserved");
    grant.commit(tree);
    assert!(tracker.state.borrow().unadmitted.iter().all(Option::is_none));
    assert!(tracker.state.borrow().slots.iter().flatten().any(|slot| slot.generation == generation && slot.job.is_some()));

    let fixed_bytes = ui_contract::UiResidentPermit::fixed_backing_bytes().unwrap();
    let capacity = aggregate.checked_sub(fixed_bytes).unwrap() / limits.max_bytes;
    assert!(capacity > 0 && capacity < SURFACE_RECONCILE_ADMISSION_SLOTS, "the CEILING-sized capacity is a handful of surfaces: {capacity}");
    assert!(floor * SURFACE_RECONCILE_ADMISSION_SLOTS < aggregate - fixed_bytes, "the floor price must admit every admission slot at once: {floor} against {}", aggregate - fixed_bytes);
    let first = ui_contract::UiResidentPermit::snapshot().unwrap();
    assert_eq!(
        first,
        ui_contract::UiResidentSnapshot { bytes: fixed_bytes + floor, items: semio_framework_ui_runtime::SURFACE_RECONCILE_FLOOR_ITEMS, used_slots: 1 },
        "one mounted reservation is priced at the reconcile FLOOR, never at the per-surface ceiling {}",
        limits.max_bytes
    );
    let held = fill_resident_aggregate();
    let full = ui_contract::UiResidentPermit::snapshot().unwrap();
    assert!(full.bytes <= aggregate && full.bytes.checked_add(floor).unwrap() > aggregate, "the aggregate must have less than one floor price left: {full:?}");
    let overflow = tree_with_owned_child("overflow");
    let overflow_pointer = overflow.root.children.get(0).unwrap().key.as_ptr();
    let (_, returned) = tracker.retain_unadmitted("66:queued".into(), overflow).expect_err("cap + 1 returns the exact tree");
    assert_eq!(returned.root.children.get(0).unwrap().key.as_ptr() == overflow_pointer, law["refusalPreservesTree"].as_bool().unwrap());
    assert!(reserve(&tracker, ui_contract::SurfaceId::try_from("67:mounted").expect("bounded surface")).is_err(), "render cannot materialize before a fixed slot exists");
    assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), full);
    release_resident_aggregate(held);
    let keys = [NativeCloseKey::fixture(4, 1)];
    for key in &keys {
        tracker.reserve_close_instance(*key).unwrap();
        tracker.activate_close_instance(*key).unwrap();
    }
    for _ in 0..65_536 {
        tracker.close_step(cx).expect("original patch fixture close step");
        if keys.iter().all(|key| tracker.close_instance_complete(*key).unwrap()) {
            break;
        }
    }
    for key in &keys {
        assert!(tracker.close_instance_complete(*key).unwrap());
        tracker.release_close_instance(*key).unwrap();
    }
    assert_eq!(tracker.terminal_is_empty(), law["terminal"].as_bool().unwrap());
    assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), ui_contract::UiResidentSnapshot { bytes: fixed_bytes, items: 0, used_slots: 0 });
    eprintln!("mounted-resident-capacity fixed={fixed_bytes} floor={floor} ceiling={} ceiling-sized-accepted={capacity} full={} cap-plus-one=false exact-refusal=true restored={fixed_bytes}", limits.max_bytes, full.bytes);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}
