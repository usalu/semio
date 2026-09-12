use super::*;
use semio_framework_ui_runtime::TreeNode;

thread_local! { static PANIC_AFTER_OUTPUT_TRANSFER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
thread_local! { static PANIC_AFTER_PRODUCER_STEP: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }

pub(super) fn after_output_transfer() {
    PANIC_AFTER_OUTPUT_TRANSFER.with(|pending| {
        if pending.replace(false) {
            panic!("[DEBUG] actual mounted direct-output transfer unwind");
        }
    });
}

pub(super) fn after_producer_step() {
    if PANIC_AFTER_PRODUCER_STEP.with(|pending| pending.replace(false)) {
        panic!("[DEBUG] actual mounted producer partial-step unwind");
    }
}

fn reject_current(tracker: &PatchTracker, surface: &str) {
    let generation = tracker.state.borrow().slots.iter().flatten().find(|slot| slot.surface.as_ref() == surface).expect("current test surface").generation;
    tracker.mark_rejected(surface, generation);
}

fn reserve(tracker: &PatchTracker, surface: ui_contract::SurfaceId) -> Result<MountedReconcileGrant, ui_contract::SurfaceId> {
    let key = NativeCloseKey::fixture(surface_instance(surface.as_ref()).expect("numeric test surface"), 1);
    tracker.reserve_mounted(surface, key)
}

fn leaf(key: &str, text: &str) -> ComponentTree {
    let value = ui_contract::UiText::try_from_str(text).expect("bounded test text");
    let root = TreeNode::try_new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(value), emphasize: None, data_attributes: None })).unwrap_or_else(|_| panic!("bounded test tree"));
    ComponentTree { root }
}

fn tree_with_owned_child(text: &str) -> ComponentTree {
    let mut tree = leaf("root", text);
    tree.root.children.try_push(leaf("owned-child", text).root).expect("bounded owned child");
    tree
}

fn publish_test(mut ready: SurfaceReconcileReadyPatch) -> (ui_contract::UiPatch, semio_framework_ui_runtime::SurfaceReconcilePublishedPatch) {
    let mut payload = ui_contract::UiPendingPatch::default();
    let mut published = None;
    assert!(ready.publish_into(&mut payload, &mut published, SurfaceReconcileReadyPatch::required_publish_bytes()).unwrap() > 0);
    assert!(ready.terminal_is_empty());
    (payload.source_mut().unwrap().take().unwrap(), published.unwrap())
}

fn close_published(mut published: semio_framework_ui_runtime::SurfaceReconcilePublishedPatch) {
    for turn in 0..65_536 {
        let step = published.close_step_with_grant(1, 4096).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= 4096);
        if step.complete && published.terminal_is_empty() {
            return;
        }
        assert!(turn < 65_535);
    }
}

fn close_ack(mut ack: SurfaceReconcilePublishedAck) {
    for turn in 0..65_536 {
        let step = ack.close_step_with_grant(1, 4096).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= 4096);
        if step.complete && ack.terminal_is_empty() {
            return;
        }
        assert!(turn < 65_535);
    }
}

fn close_test_patch(patch: ui_contract::UiPatch) {
    let mut owner = ui_contract::UiPendingPatch::default();
    *owner.source_mut().unwrap() = Some(patch);
    for turn in 0..65_536 {
        owner.close_step(1, 4096).unwrap();
        if owner.terminal_is_empty() {
            return;
        }
        assert!(turn < 65_535);
    }
}

fn finish(tracker: &PatchTracker) -> Option<ui_contract::UiPatch> {
    for _ in 0..4_096 {
        tracker.drive_one();
        if let Some(owner) = tracker.take_ready_patch() {
            let (patch, published) = publish_test(owner);
            close_published(published);
            return Some(patch);
        }
        if !tracker.has_work() {
            return None;
        }
    }
    None
}

fn published(tracker: &PatchTracker, surface: &str) -> semio_framework_ui_runtime::SurfaceReconcilePublishedPatch {
    tracker.begin(surface.to_owned(), leaf("root", "published")).expect("admitted publication");
    for _ in 0..4_096 {
        tracker.drive_one();
        if let Some(owner) = tracker.take_ready_patch() {
            let (patch, published) = publish_test(owner);
            close_test_patch(patch);
            return published;
        }
    }
    panic!("publication did not become ready")
}

fn saturate_terminals(tracker: &PatchTracker, instance: u32, generation: u64) {
    let mut state = tracker.state.borrow_mut();
    for (index, terminal) in state.terminals.iter_mut().enumerate() {
        *terminal = Some(TerminalSlot {
            key: NativeCloseKey::fixture(instance, 1),
            instance: Some(instance),
            authority: SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(ui_contract::SurfaceId::try_from(format!("{instance}:terminal-{index}")).expect("bounded surface fixture")), generation + index as u64)
                .expect("fixed terminal admission"),
            close: true,
        });
    }
}

fn close_instance_to_empty(tracker: &PatchTracker, instance: u32) {
    let key = NativeCloseKey::fixture(instance, 1);
    tracker.reserve_close_instance(key).expect("exact close reservation");
    tracker.activate_close_instance(key).expect("activate retained close");
    for _ in 0..65_536 {
        tracker.close_step(1, 4096);
        if tracker.close_instance_complete(key).expect("exact close receipt") {
            tracker.release_close_instance(key).expect("final ACK releases close slot");
            assert!(tracker.terminal_is_empty());
            return;
        }
    }
    panic!("instance {instance} did not reach terminal empty: {}", tracker.debug_state());
}

#[test]
fn mounted_output_admission_cancel_and_drop_keep_the_original_close_generation() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixtures/🔣️.json")).unwrap();
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
        close_instance_to_empty(&tracker, 74);
        assert_eq!(preserved, fixture["generationPreserved"].as_bool().unwrap());
        assert_eq!(revision, fixture["revision"].as_u64().unwrap());
        assert_eq!(tracker.terminal_is_empty(), fixture["terminal"].as_bool().unwrap());
        eprintln!("[DEBUG] mounted-uncommitted-close drop={drop_grant} generation={generation} preserved={preserved} revision={revision} terminal=true");
    }
}

#[test]
fn mounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixtures/🔣️.json")).unwrap();
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
    close_instance_to_empty(&tracker, 71);
    assert_eq!(admitted, fixture["extraInvocation"].as_bool().unwrap());
    eprintln!("[DEBUG] mounted-output-admission accepted={admitted} tree-constructed=false shared-entries=64");
}

#[test]
fn mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixtures/🔣️.json")).unwrap();
    let tracker = PatchTracker::new();
    reserve(&tracker, ui_contract::SurfaceId::try_from("72:producer-unwind").unwrap()).unwrap().commit_source(leaf("root", "owned-é").root).unwrap();
    let (index, rejected_index, generation, pointer) = {
        let state = tracker.state.borrow();
        let index = state.slots.iter().position(Option::is_some).unwrap();
        let slot = state.slots[index].as_ref().unwrap();
        let producer = slot.producer.as_ref().unwrap();
        (index, producer.rejected_index, slot.generation, producer.authority.as_ref() as *const _)
    };
    PANIC_AFTER_PRODUCER_STEP.with(|pending| pending.set(true));
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tracker.drive_one()));
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
    close_instance_to_empty(&tracker, 72);
    assert_eq!(retained, fixture["partialProducerUnwindRetainsOwner"].as_bool().unwrap(), "actual producer step must not remove the structural slot before invoking the child");
    eprintln!("[DEBUG] mounted-producer-unwind exact-slot-and-box-retained={retained}");
}

#[test]
fn mounted_output_admission_incomplete_producer_sources_preserve_remaining_owners() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixtures/🔣️.json")).unwrap();
    let tracker = PatchTracker::new();
    reserve(&tracker, ui_contract::SurfaceId::try_from("73:producer-source").unwrap()).unwrap().commit_source(leaf("owned-root", "é").root).unwrap();
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
    tracker.drive_one();
    tracker.drive_one();
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
    close_instance_to_empty(&tracker, 73);
    assert_eq!(reservation_retained && tree_retained, fixture["incompleteProducerSourcesPreserveRemainingOwners"].as_bool().unwrap(), "one missing source must not consume other completed owners");
    eprintln!("[DEBUG] mounted-producer-incomplete reservation-retained={reservation_retained} tree-retained={tree_retained}");
}

#[test]
fn mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["capturedLifetime"];
    let instance = law["instance"].as_u64().unwrap() as u32;
    let key = NativeCloseKey::fixture(instance, law["original"].as_u64().unwrap());
    let foreign = NativeCloseKey::fixture(instance, law["reused"].as_u64().unwrap());
    let tracker = PatchTracker::new();
    let grant = tracker.reserve_mounted(ui_contract::SurfaceId::try_from(format!("{instance}:direct")).unwrap(), key).unwrap();
    let generation = grant.generation;
    grant.commit_source(leaf("owned-root", "界-é").root).unwrap();
    PANIC_AFTER_OUTPUT_TRANSFER.with(|pending| pending.set(true));
    let mut caught = false;
    for _ in 0..65_536 {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tracker.drive_one())).is_err() {
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
        panic!("[DEBUG] actual Pending receiver callback retains exact Ready");
    }));
    assert!(caught.is_err());
    assert_eq!(target.as_ref().unwrap().generation(), generation);
    assert_eq!(target.as_ref().unwrap().surface().unwrap().as_ref(), format!("{instance}:direct"));
    while !target.as_mut().unwrap().close_step_with_grant(1, 4096).unwrap().complete {}
    close_instance_to_empty(&tracker, instance);
    assert_eq!(tracker.terminal_is_empty(), law["terminal"].as_bool().unwrap());
    eprintln!("[DEBUG] live-output exact-lifetime=true admission-generation={generation} producer-callback-roots={exact_roots} occupied-busy-zero-refusal=true pending-callback-retained=true terminal=true");
}

#[test]
fn mounted_output_admission_close_waits_for_the_original_uncommitted_grant() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let key = NativeCloseKey::fixture(76, 1);
    let grant = tracker.reserve_mounted(ui_contract::SurfaceId::try_from("76:grant").unwrap(), key).unwrap();
    tracker.reserve_close_instance(key).unwrap();
    tracker.activate_close_instance(key).unwrap();
    for _ in 0..8 {
        tracker.close_step(1, 4096);
        assert!(!tracker.close_instance_complete(key).unwrap());
    }
    let root = tree_with_owned_child("returned");
    let pointer = root.root.children.get(0).unwrap().key.as_ptr();
    let returned = grant.commit_source(root.root).expect_err("closing lifetime rejects producer invocation");
    assert_eq!(returned.children.get(0).unwrap().key.as_ptr(), pointer);
    close_instance_to_empty(&tracker, 76);
    eprintln!("[DEBUG] live-output close-waits-for-original-grant=true rejected-tree-pointer-preserved=true");
}

#[test]
fn mounted_output_admission_concurrent_trackers_share_one_fixed_pool_without_overadmission() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixtures/🔣️.json")).unwrap();
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
            close_instance_to_empty(&tracker, instance);
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
    eprintln!("[DEBUG] live-output same-process-workers=2 preoccupied=63 accepted={accepted} exact-refusal=true full64-restored=true");
}

#[test]
fn tracker_initialization_fits_the_component_stack_budget() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let bytes = size_of::<PatchTrackerState>();
    assert!(bytes <= 256, "PatchTrackerState requires {bytes} bytes");
    let state = PatchTrackerState::default();
    assert_eq!(state.slots.len(), SURFACE_RECONCILE_ADMISSION_SLOTS);
    assert_eq!(state.unadmitted.len(), SURFACE_RECONCILE_ADMISSION_SLOTS + 1);
    assert_eq!(state.ready.len(), READY_PATCH_CAPACITY);
}

#[test]
fn mounted_path_advances_one_reconcile_opportunity_per_grant() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    tracker.begin("main".into(), leaf("root", "a")).expect("admitted");
    assert!(tracker.take_ready_patch().is_none());
    assert!(tracker.drive_one());
    assert!(tracker.take_ready_patch().is_none());
    let patch = finish(&tracker).expect("eventual patch");
    assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
}

#[test]
fn mounted_document_tree_publishes_nested_interactive_rows() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use ui_contract::{Buildable, HasBase, HasChildren};
    fn row(value: &serde_json::Value) -> ui_contract::BuiltNode {
        let id = value["id"].as_str().unwrap();
        let mut builder = ui_contract::tree_item(ui_contract::Label(ui_contract::UiText::try_from_str(id).unwrap())).try_id(id).ok().unwrap();
        let action = |name: &str| serde_json::from_value(serde_json::json!({ "scope": "fixture", "name": name, "version": 1 })).unwrap();
        let args = || serde_json::from_value(serde_json::json!({ "domainId": "fixture", "merge": "replace", "method": "pick", "targets": id })).unwrap();
        builder = builder.try_on_with(ui_contract::Trigger::Activate, action("select"), args()).ok().unwrap();
        for name in value["rowActions"].as_array().into_iter().flatten() {
            let name = name.as_str().unwrap();
            builder = builder
                .try_row_action(ui_contract::RowAction {
                    icon: ui_contract::UiText::try_from_str(name).unwrap(),
                    label: Some(ui_contract::Label(ui_contract::UiText::try_from_str(name).unwrap())),
                    action: ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: action(name), args: Some(args()), capability: None },
                    placement: ui_contract::RowActionPlacement::Row,
                })
                .ok()
                .unwrap();
        }
        builder.try_children(value["children"].as_array().into_iter().flatten().map(row)).ok().unwrap().try_build().unwrap()
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📃️document-surface.json")).unwrap();
    assert_eq!(ui_contract::UI_NODE_BINDINGS as u64, fixture["limits"]["nodeBindings"].as_u64().unwrap());
    assert_eq!(semio_framework_ui_runtime::SurfaceReconcileLimits::default().max_bytes as u64, fixture["limits"]["surfaceBytes"].as_u64().unwrap());
    assert_eq!(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES as u64, fixture["limits"]["pageBytes"].as_u64().unwrap());
    assert_eq!(semio_framework_ui_runtime::SURFACE_RECONCILE_AGGREGATE_BYTES as u64, fixture["limits"]["aggregateBytes"].as_u64().unwrap());
    let sections = fixture["sections"].as_array().unwrap().iter().map(|section| {
        ui_contract::tree_section(ui_contract::Label(ui_contract::UiText::try_from_str(section["id"].as_str().unwrap()).unwrap()))
            .try_id(section["id"].as_str().unwrap())
            .ok()
            .unwrap()
            .try_children(section["rows"].as_array().unwrap().iter().map(row))
            .ok()
            .unwrap()
            .try_build()
            .unwrap()
    });
    let root = ui_contract::tree().try_id("document").ok().unwrap().try_children(sections).ok().unwrap().try_build().unwrap();
    let tracker = PatchTracker::new();
    reserve(&tracker, ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
    let patch = finish(&tracker).unwrap_or_else(|| panic!("document never published: {:?}", tracker.state.borrow().terminals.iter().flatten().map(|slot| slot.authority.fault()).collect::<Vec<_>>()));
    let nodes = patch.ops.iter().filter_map(|op| if let ui_contract::UiPatchOp::Upsert(node) = op { Some(node) } else { None }).collect::<Vec<_>>();
    assert_eq!(nodes.len(), fixture["nodes"].as_u64().unwrap() as usize);
    assert_eq!(nodes.iter().filter(|node| !node.bindings.is_empty()).count(), fixture["interactiveRows"].as_u64().unwrap() as usize);
}

#[test]
fn mounted_settings_controls_publish_with_authored_fields() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use ui_contract::{Buildable, HasBase, HasChildren};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎚️settings-surface.json")).expect("language-neutral settings");
    let fields = fixture["fields"].as_array().unwrap();
    let children = fields.iter().map(|field| {
        let id = field["id"].as_str().unwrap();
        let action = serde_json::from_value(serde_json::json!({ "scope": "fixture", "name": field["action"], "version": 1 })).unwrap();
        let mut control =
            ui_contract::BuiltNode::try_new(format!("{id}.control"), ui_contract::Component::NumberStepper(ui_contract::NumberStepperProps { value: field["value"].as_f64().unwrap(), step: field["step"].as_f64().unwrap(), uniform: false }))
                .ok()
                .unwrap();
        control.bindings.try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Change, action, args: None, capability: None }).ok().unwrap();
        ui_contract::field(ui_contract::Label(ui_contract::UiText::try_from_str(field["label"].as_str().unwrap()).unwrap())).try_id(id).ok().unwrap().try_child(control).ok().unwrap().try_build().unwrap()
    });
    let root = ui_contract::section(ui_contract::Label(ui_contract::UiText::try_from_str(fixture["label"].as_str().unwrap()).unwrap())).try_id("settings").ok().unwrap().default_open(true).try_children(children).ok().unwrap().try_build().unwrap();
    let tracker = PatchTracker::new();
    reserve(&tracker, ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
    let patch = finish(&tracker).unwrap_or_else(|| panic!("settings never published: {:?}", tracker.state.borrow().slots.iter().flatten().map(|slot| &slot.job).collect::<Vec<_>>()));
    let controls = patch
        .ops
        .iter()
        .filter_map(|op| match op {
            ui_contract::UiPatchOp::Upsert(node) => match &node.component {
                ui_contract::Component::NumberStepper(props) => Some(serde_json::json!({ "value": props.value, "step": props.step, "action": node.bindings[0].action.name.as_str() })),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(controls, fields.iter().map(|field| serde_json::json!({ "value": field["value"], "step": field["step"], "action": field["action"] })).collect::<Vec<_>>());
}

#[test]
fn mounted_catalogue_publishes_every_section_beyond_thirty_two_nodes() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use ui_contract::{Buildable, HasBase, HasChildren};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🗂️catalogue-surface.json")).unwrap();
    let rows = fixture["rowsPerSection"].as_u64().unwrap();
    let mut expected = std::collections::BTreeSet::new();
    let sections = fixture["sections"].as_array().unwrap().iter().map(|section| {
        let section = section.as_str().unwrap();
        let children = (0..rows).map(|row| {
            let key = format!("{section}.{row}");
            expected.insert(key.clone());
            leaf(&key, &key).root
        });
        ui_contract::tree_section(ui_contract::Label(ui_contract::UiText::try_from_str(section).unwrap())).try_id(section).ok().unwrap().try_children(children).ok().unwrap().try_build().unwrap()
    });
    let root = ui_contract::tree().try_id("catalogue").ok().unwrap().try_children(sections).ok().unwrap().try_build().unwrap();
    assert_eq!(expected.len() as u64, fixture["expectedRows"].as_u64().unwrap());
    let tracker = PatchTracker::new();
    reserve(&tracker, ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
    let mut published = None;
    for _ in 0..65_536 {
        tracker.drive_one();
        if let Some(owner) = tracker.take_ready_patch() {
            let (patch, authority) = publish_test(owner);
            let mut authority = Some(authority);
            let mut acknowledgement = None;
            assert!(semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::acknowledge_into(
                &mut authority,
                &mut acknowledgement,
                fixture["surface"].as_str().unwrap(),
                patch.revision.0,
                semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::required_acknowledge_bytes()
            )
            .unwrap());
            assert!(tracker.mark_published_ack(acknowledgement.as_ref().unwrap()).unwrap());
            close_ack(acknowledgement.take().unwrap());
            published = Some(patch);
            break;
        }
        if !tracker.has_work() {
            break;
        }
    }
    let fault = format!("{:?}", tracker.state.borrow().terminals.iter().flatten().map(|slot| slot.authority.fault()).collect::<Vec<_>>());
    close_instance_to_empty(&tracker, 1);
    let patch = published.unwrap_or_else(|| panic!("catalogue did not publish: {fault}"));
    let nodes = patch.ops.iter().filter_map(|op| if let ui_contract::UiPatchOp::Upsert(node) = op { Some(node) } else { None }).collect::<Vec<_>>();
    assert_eq!(nodes.len() as u64, fixture["expectedNodes"].as_u64().unwrap());
    assert_eq!(nodes.iter().filter(|node| matches!(node.component, ui_contract::Component::Text(_))).map(|node| node.key.to_string()).collect::<std::collections::BTreeSet<_>>(), expected);
}

#[test]
fn mounted_catalogue_reports_producer_failure_once_before_cleanup() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🗂️catalogue-surface.json")).unwrap();
    let failure = &fixture["failure"];
    let key = failure["key"].as_str().unwrap();
    let mut root = leaf("catalogue", "Catalogue").root;
    root.children.try_push(leaf(key, key).root).unwrap();
    root.children.try_push(leaf(key, key).root).unwrap();
    let tracker = PatchTracker::new();
    reserve(&tracker, ui_contract::SurfaceId::try_from(failure["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
    let mut reported = None;
    for _ in 0..4096 {
        tracker.drive_one();
        if let Some(fault) = tracker.take_render_fault() {
            reported = Some(fault);
            break;
        }
    }
    assert!(tracker.take_ready_patch().is_none());
    assert!(tracker.take_render_fault().is_none());
    close_instance_to_empty(&tracker, failure["instance"].as_u64().unwrap() as u32);
    let (instance, reason) = reported.expect("producer fault must not disappear during cleanup");
    assert_eq!(instance as u64, failure["instance"].as_u64().unwrap());
    assert!(reason.contains(failure["surface"].as_str().unwrap()));
    assert!(reason.contains(failure["reason"].as_str().unwrap()));
}

#[test]
fn mounted_catalogue_reports_reconcile_capacity_without_leaking_owners() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🗂️catalogue-surface.json")).unwrap();
    let failure = &fixture["capacityFailure"];
    assert_eq!(ui_contract::UI_DOCUMENT_NODES as u64, failure["nodeLimit"].as_u64().unwrap());
    let mut root = leaf("catalogue", "Catalogue").root;
    for section in 0..failure["sections"].as_u64().unwrap() {
        let mut child = leaf(&section.to_string(), "Section").root;
        for row in 0..failure["rowsPerSection"].as_u64().unwrap() {
            child.children.try_push(leaf(&row.to_string(), "Row").root).unwrap();
        }
        root.children.try_push(child).unwrap();
    }
    let tracker = PatchTracker::new();
    reserve(&tracker, ui_contract::SurfaceId::try_from(failure["surface"].as_str().unwrap()).unwrap()).unwrap().commit_source(root).unwrap();
    let mut reported = None;
    for _ in 0..65_536 {
        tracker.drive_one();
        if let Some(fault) = tracker.take_render_fault() {
            reported = Some(fault);
            break;
        }
    }
    assert!(tracker.take_ready_patch().is_none());
    assert!(tracker.take_render_fault().is_none());
    close_instance_to_empty(&tracker, failure["instance"].as_u64().unwrap() as u32);
    let (instance, reason) = reported.expect("reconcile failure must not become an idle empty surface");
    assert_eq!(instance as u64, failure["instance"].as_u64().unwrap());
    assert!(reason.contains(failure["surface"].as_str().unwrap()));
    assert!(reason.contains(failure["reason"].as_str().unwrap()));
}

#[test]
fn mounted_sources_publish_every_window_and_panel_tree() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use ui_contract::{Buildable, HasBase, HasChildren};
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️mounted-surfaces.json")).expect("language-neutral surface fixtures");
    let fixtures = fixtures.as_array().expect("surface list");
    let tracker = PatchTracker::new();
    for fixture in fixtures {
        let surface = fixture["surface"].as_str().expect("surface key");
        let children = fixture["children"].as_array().expect("child keys").iter().map(|key| {
            let mut node = leaf(key.as_str().expect("child key"), "content").root;
            let action = serde_json::from_value(fixture["action"].clone()).expect("language-neutral action");
            node.bindings.try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action, args: None, capability: None }).expect("bounded binding");
            node
        });
        let root = ui_contract::column().try_id(surface).ok().expect("bounded key").try_children(children).ok().expect("bounded children").try_build().expect("bounded root");
        reserve(&tracker, ui_contract::SurfaceId::try_from(surface).expect("bounded surface")).expect("mounted reservation").commit_source(root).expect("mounted producer");
        let patch = finish(&tracker).unwrap_or_else(|| {
            let state = tracker.state.borrow();
            panic!(
                "surface {} never published: {:?}; terminals={}, rejected={}",
                fixture["surface"],
                state.slots.iter().flatten().map(|slot| (&slot.surface, slot.producer.as_ref().map(|producer| producer.authority.fault()), &slot.job, slot.reconciler.is_some())).collect::<Vec<_>>(),
                state.terminals.iter().flatten().count(),
                state.rejected.iter().flatten().count()
            );
        });
        assert_eq!(patch.surface.0.as_str(), fixture["surface"].as_str().unwrap());
        assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(..))).count(), 1 + fixture["children"].as_array().unwrap().len());
        for op in &patch.ops {
            if let ui_contract::UiPatchOp::Upsert(record) = op {
                if record.key.as_str() != surface {
                    assert_eq!(serde_json::to_value(&record.bindings[0].action).expect("action wire oracle"), fixture["action"]);
                }
            }
        }
    }
}

#[test]
fn one_active_surface_does_not_wait_behind_sixty_three_empty_slots_between_steps() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let steps = [0, SURFACE_RECONCILE_ADMISSION_SLOTS - 1].map(|index| {
        let tracker = PatchTracker::new();
        tracker.begin("main".into(), leaf("root", "a")).expect("admitted");
        tracker.state.borrow_mut().slots.swap(0, index);
        for step in 1..=4_096 {
            tracker.drive_one();
            if let Some(owner) = tracker.take_ready_patch() {
                let (patch, published) = publish_test(owner);
                close_test_patch(patch);
                close_published(published);
                return step;
            }
        }
        panic!("active surface never published");
    });
    assert_eq!(steps[0], steps[1], "empty slots consume no reconcile opportunities");
}

#[test]
fn issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    use super::super::pending::PendingPatchAuthority;
    use semio_framework::kernel::{ActorInstanceLifetime, ActorUiPatchReceipt};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📨️pending/🧫️fixtures/🩹️receipt.json")).unwrap();
    let surface = fixture["issued"]["surface"].as_str().unwrap();
    for rejection in [false, true] {
        let tracker = PatchTracker::new();
        let mut pending = PendingPatchAuthority::new();
        let first = tracker.begin(surface.into(), leaf("root", "first")).unwrap();
        let mut ready = None;
        for _ in 0..4096 {
            tracker.drive_one();
            if let Some(owner) = tracker.take_ready_patch() {
                ready = Some(owner);
                break;
            }
        }
        pending.push_reconcile(ready.expect("real reconcile publication")).unwrap_or_else(|_| panic!("empty pending slot"));
        assert!(pending.take_one(65536).unwrap().is_none());
        let patch = pending.take_one(65536).unwrap().unwrap();
        let issued = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 41, instance_id: 7, guest_lifetime: 3 }, patch_sequence: 8 };
        pending.stage_emission(issued, &patch).unwrap();
        pending.commit_emission();
        let revision = patch.revision.0;
        close_test_patch(patch);
        assert!(!tracker.mark_rejected(surface, first + 1), "future feedback cannot reset a current generation");
        for turn in 0..4096 {
            tracker.drive_one();
            tracker.close_step(1, 4096);
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
        close_instance_to_empty(&tracker, 7);
        eprintln!("[DEBUG] obsolete real reconcile feedback rejection={} old={} current={} exact old owner retired", rejection, first, second);
    }
}

#[test]
fn published_owner_first_ack_rejects_early_stale_duplicate_wrong_instance_and_aba_without_authority_loss() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let mut published = Some(published(&tracker, "71:ack"));
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
}

#[test]
fn cap_plus_one_returns_the_exact_tree_owner() {
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
                operation: semio_framework_job::allocate_operation_id(),
                preview_sequence: 0,
                acknowledged_revision: ui_contract::UiRevision::default(),
                cancel: semio_framework_job::root_cancel_token(),
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
}

#[test]
fn mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧠️runtime/📤️output/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["residentCapacity"];
    let aggregate = semio_framework_ui_runtime::SURFACE_RECONCILE_AGGREGATE_BYTES;
    let limits = semio_framework_ui_runtime::SurfaceReconcileLimits::default();
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
    assert!(capacity > 0 && capacity < SURFACE_RECONCILE_ADMISSION_SLOTS);
    let first = ui_contract::UiResidentPermit::snapshot().unwrap();
    assert_eq!(first, ui_contract::UiResidentSnapshot { bytes: fixed_bytes + limits.max_bytes, items: limits.max_items, used_slots: 1 });
    for index in 0..capacity - 1 {
        tracker.retain_unadmitted(format!("{index}:queued"), leaf("root", "queued")).expect("fixed unadmitted slot");
        let admitted = index + 2;
        assert_eq!(
            ui_contract::UiResidentPermit::snapshot().unwrap(),
            ui_contract::UiResidentSnapshot { bytes: fixed_bytes.checked_add(admitted.checked_mul(limits.max_bytes).unwrap()).unwrap(), items: admitted.checked_mul(limits.max_items).unwrap(), used_slots: admitted }
        );
    }
    let full = ui_contract::UiResidentPermit::snapshot().unwrap();
    assert!(full.bytes <= aggregate && full.bytes.checked_add(limits.max_bytes).unwrap() > aggregate);
    let overflow = tree_with_owned_child("overflow");
    let overflow_pointer = overflow.root.children.get(0).unwrap().key.as_ptr();
    let (_, returned) = tracker.retain_unadmitted("66:queued".into(), overflow).expect_err("cap + 1 returns the exact tree");
    assert_eq!(returned.root.children.get(0).unwrap().key.as_ptr() == overflow_pointer, law["refusalPreservesTree"].as_bool().unwrap());
    assert!(reserve(&tracker, ui_contract::SurfaceId::try_from("67:mounted").expect("bounded surface")).is_err(), "render cannot materialize before a fixed slot exists");
    assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), full);
    let keys: Vec<_> = std::iter::once(4).chain((0..capacity - 1).map(|index| u32::try_from(index).unwrap())).map(|instance| NativeCloseKey::fixture(instance, 1)).collect();
    for key in &keys {
        tracker.reserve_close_instance(*key).unwrap();
        tracker.activate_close_instance(*key).unwrap();
    }
    for _ in 0..65_536 {
        tracker.close_step(1, 4096);
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
    eprintln!("[DEBUG] mounted-resident-capacity fixed={fixed_bytes} per={} accepted={capacity} full={} cap-plus-one=false exact-refusal=true restored={fixed_bytes}", limits.max_bytes, full.bytes);
}

#[test]
fn stale_generation_fault_is_publicly_retrievable() {
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
}

#[test]
fn resize_storm_coalesces_to_one_deferred_surface_owner() {
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
}

#[test]
fn effects_publish_in_admission_order_even_when_later_tree_finishes_first() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    tracker.begin("1:first".into(), leaf("root", "a")).expect("first");
    tracker.begin("1:second".into(), leaf("root", "b")).expect("second");
    let first = finish(&tracker).expect("first ready");
    let second = finish(&tracker).expect("second ready");
    assert_eq!(first.surface.0.as_str(), "1:first");
    assert_eq!(second.surface.0.as_str(), "1:second");
}

#[test]
fn actor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot() {
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
    close_instance_to_empty(&tracker, 9);
    assert!(tracker.terminal_is_empty());
}

#[test]
fn close_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️surface-close.json")).unwrap();
    let instance = fixture["instance"].as_u64().unwrap() as u32;
    let surface = |key: &str| fixture["surfaces"][key].as_str().unwrap();
    let tracker = PatchTracker::new();
    tracker.begin(surface("ready").into(), leaf("root", "ready")).expect("ready source");
    for _ in 0..128 {
        tracker.drive_one();
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
        tracker.close_step(1, 4096);
        if tracker.close_instance_complete(key).expect("exact close receipt") {
            break;
        }
    }
    assert!(!tracker.terminal_is_empty(), "completed close receipt remains owned until exact ACK");
    tracker.release_close_instance(key).expect("final ACK releases close slot");
    assert_eq!(tracker.terminal_is_empty(), fixture["terminalEmpty"].as_bool().unwrap());
    assert_eq!(tracker.take_ready_patch().is_some(), fixture["stalePatch"].as_bool().unwrap());
}

#[test]
fn terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed() {
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
}

#[test]
fn terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    saturate_terminals(&tracker, 51, 510_000);
    tracker.retain_unadmitted("51:queued".into(), leaf("root", "queued")).expect("pre-admitted owner");
    close_instance_to_empty(&tracker, 51);
}

#[test]
fn terminal_full_plus_matching_rejected_advances_capacity_before_conversion() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let identifier_bytes = semio_framework_ui_runtime::SurfaceReconcileLimits::default().max_identifier_bytes;
    tracker.begin(format!("52:{}", "x".repeat(identifier_bytes)), leaf("root", "rejected")).expect("fixed surface slot");
    assert!(tracker.state.borrow().rejected.iter().any(Option::is_some), "identifier cap produces an exact rejected owner");
    saturate_terminals(&tracker, 52, 520_000);
    close_instance_to_empty(&tracker, 52);
}

#[test]
fn terminal_full_plus_matching_surface_advances_capacity_before_conversion() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    {
        let mut state = tracker.state.borrow_mut();
        state.slots[0] = Some(SurfaceSlot {
            key: NativeCloseKey::fixture(53, 1),
            output_index: None,
            surface: ui_contract::SurfaceId::try_from("53:idle").expect("bounded surface fixture"),
            generation: 530_000,
            operation: semio_framework_job::allocate_operation_id(),
            preview_sequence: 0,
            acknowledged_revision: ui_contract::UiRevision::default(),
            cancel: semio_framework_job::root_cancel_token(),
            reconciler: Some(SurfaceReconciler::new(ui_contract::SurfaceId::try_from("53:idle").expect("bounded surface fixture"))),
            producer: None,
            job: None,
        });
    }
    saturate_terminals(&tracker, 53, 531_000);
    close_instance_to_empty(&tracker, 53);
}

#[test]
fn generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation() {
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
}

#[test]
fn terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation() {
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
            operation: semio_framework_job::allocate_operation_id(),
            preview_sequence: 0,
            acknowledged_revision: ui_contract::UiRevision::default(),
            cancel: semio_framework_job::root_cancel_token(),
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
}

#[test]
fn abandoned_reconcile_job_closes_its_ready_output_so_the_tracker_can_idle() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    tracker.begin("5:main".into(), tree_with_owned_child("a")).expect("admitted");
    for _ in 0..4_096 {
        tracker.drive_one();
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
        tracker.drive_one();
        tracker.close_step(1, 4096);
        let _ = tracker.take_deferred_ready();
        if !tracker.has_work() {
            break;
        }
    }
    assert!(!tracker.has_work(), "an abandoned job must not strand its reserved output: {}", tracker.debug_state());
}

#[test]
fn a_closing_terminal_does_not_wait_behind_sixty_three_empty_slots_per_unit() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let generation = tracker.begin("6:main".into(), tree_with_owned_child("a")).expect("admitted");
    let patch = finish(&tracker).expect("first publication");
    close_test_patch(patch);
    assert!(tracker.mark_rejected("6:main", generation), "the published generation must be rejectable into a terminal");
    let mut steps = 0;
    while tracker.has_work() {
        tracker.close_step(1, 4096);
        let _ = tracker.take_deferred_ready();
        steps += 1;
        assert!(steps < 1_024, "a single closing terminal must drain one unit per close step, not one per 64: {}", tracker.debug_state());
    }
}

#[test]
fn a_deferred_surface_awaiting_the_hosts_acknowledgement_does_not_hold_more_work() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    tracker.begin("9:main".into(), leaf("root", "a")).expect("admitted");
    for _ in 0..4_096 {
        tracker.drive_one();
        tracker.close_step(1, 4096);
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
}

/// 🕹️ Wave B48 LAW: a render reservation that ends WITHOUT `cancel` releases exactly what `cancel`
/// releases — so the surface it held is reservable again, and its reserved output is not leaked.
///
/// 🐛️ `MountedReconcileGrant`'s `Drop` used to release only the reconciler and the reservation, while
/// `cancel` also cleared `slot.output_index` and closed the reserved `ready` output. Every
/// `commit_source` failure exit takes the `Drop` path (it returns `Err(root)` without disarming), and
/// `reserve_mounted_owned` refuses any slot whose `output_index.is_some()` — so ONE refused commit made
/// that surface permanently un-reservable: every later dirty render was refused, only `defer`red,
/// answered `deferred_surface_ready` (no producer, no job, reconciler restored, revision acknowledged),
/// re-dirtied, and refused again, forever. The surface stays frozen at its last published revision while
/// the actor answers `more-work` and publishes nothing — measured live on `:6013` at wasm #58 as a pick
/// whose refresh asked for all three world bodies, dropped none, and got `changed:[]` with every
/// retained surface still at revision 1 (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B46 §5/§8.1, wave
/// B48 §3). The loop below runs past [`READY_PATCH_CAPACITY`] because each such exit also leaked one
/// output slot, and once those are gone `reserve_mounted` refuses EVERY surface in the shell.
#[test]
fn a_dropped_render_reservation_releases_its_surface_slot_and_its_output() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let key = NativeCloseKey::fixture(5, 1);
    let surface = ui_contract::SurfaceId::try_from("5:window").expect("bounded surface");
    for attempt in 0..(READY_PATCH_CAPACITY + 2) {
        let grant = match tracker.reserve_mounted(surface.clone(), key) {
            Ok(grant) => grant,
            Err(_refused) => panic!("attempt {attempt} must still be reservable after {attempt} dropped reservations: {}", tracker.debug_state()),
        };
        drop(grant);
        // 🧹️ A released output is CLOSED, not yet retired — retirement is incremental and a turn drives
        // it, so the law drives it too. Without this the loop would only prove the surface slot is free;
        // with it, it also proves the released output returns to the fixed `ready` pool.
        for _ in 0..1_024 {
            tracker.drive_one();
            if tracker.close_step(1, 4_096) {
                break;
            }
        }
    }
    let grant = tracker.reserve_mounted(surface.clone(), key).expect("a released slot reserves");
    grant.cancel();
    assert!(tracker.reserve_mounted(surface, key).is_ok(), "`cancel` and `Drop` must leave the slot in the same reservable state: {}", tracker.debug_state());
}
