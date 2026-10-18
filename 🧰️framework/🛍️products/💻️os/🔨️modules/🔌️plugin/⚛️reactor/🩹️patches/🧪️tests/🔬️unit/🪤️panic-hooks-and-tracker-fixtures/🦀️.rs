// 🪤️ Panic hooks and tracker fixtures: the panic-after-transfer and panic-after-step thread locals, the reserve, leaf, tree, publish, close, drain, acknowledge and saturate helpers over one patch tracker.

thread_local! { static PANIC_AFTER_OUTPUT_TRANSFER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
thread_local! { static PANIC_AFTER_PRODUCER_STEP: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }

pub(super) fn after_output_transfer() {
    PANIC_AFTER_OUTPUT_TRANSFER.with(|pending| {
        if pending.replace(false) {
            panic!("actual mounted direct-output transfer unwind");
        }
    });
}

pub(super) fn after_producer_step() {
    if PANIC_AFTER_PRODUCER_STEP.with(|pending| pending.replace(false)) {
        panic!("actual mounted producer partial-step unwind");
    }
}

fn reject_current(tracker: &PatchTracker, surface: &str) {
    let generation = tracker.state.borrow().slots.iter().flatten().find(|slot| slot.surface.as_ref() == surface).expect("current test surface").generation;
    tracker.mark_rejected(surface, generation);
}

#[doc(hidden)]
pub fn reserve(tracker: &PatchTracker, surface: ui_contract::SurfaceId) -> Result<MountedReconcileGrant, ui_contract::SurfaceId> {
    let key = NativeCloseKey::fixture(surface_instance(surface.as_ref()).expect("numeric test surface"), 1);
    tracker.reserve_mounted(surface, key)
}

#[doc(hidden)]
pub fn leaf(key: &str, text: &str) -> ComponentTree {
    let value = ui_contract::UiText::try_from_str(text).expect("bounded test text");
    let root = TreeNode::try_new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(value), emphasize: None, data_attributes: None })).unwrap_or_else(|_| panic!("bounded test tree"));
    ComponentTree::new(root)
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

#[doc(hidden)]
pub fn finish(tracker: &PatchTracker,cx:&mut semio_framework_job::StepContext<'_>) -> Option<ui_contract::UiPatch> {
    for _ in 0..4_096 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
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

/// 🚰️ Pumps the reconcile and close ladders until one surface's slot has handed its output slot back
/// and holds its reconciler again — the state the NEXT dirty render of that surface reserves from. A real
/// turn does this in `reconcile_step_opportunities` and `close_step`; a law that publishes the same surface
/// twice has to do it explicitly (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
fn drain_released_output(tracker: &PatchTracker, surface: &str, round: u32,cx:&mut semio_framework_job::StepContext<'_>) {
    for _ in 0..65_536 {
        let ready = tracker.state.borrow().slots.iter().flatten().find(|slot| slot.surface.as_ref() == surface).is_some_and(|slot| slot.output_index.is_none() && slot.job.is_none() && slot.reconciler.is_some());
        if ready {
            return;
        }
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        tracker.close_step(cx).expect("original patch fixture close step");
    }
    panic!("round {round} never released its output slot: {}", tracker.debug_state());
}

/// 🤝️ Drives one publication all the way through the host's own acknowledgement, so the surface's slot
/// releases its output and the NEXT publication of the same surface is admissible — what a real session
/// does every refresh, and what a repeat-publication law needs (ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B56).
fn acknowledge_one_publication(tracker: &PatchTracker, surface: &str,cx:&mut semio_framework_job::StepContext<'_>) -> Option<ui_contract::UiPatch> {
    for _ in 0..65_536 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        if let Some(owner) = tracker.take_ready_patch() {
            let (patch, authority) = publish_test(owner);
            let mut authority = Some(authority);
            let mut acknowledgement = None;
            assert!(semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::acknowledge_into(
                &mut authority,
                &mut acknowledgement,
                surface,
                patch.revision.0,
                semio_framework_ui_runtime::SurfaceReconcilePublishedPatch::required_acknowledge_bytes()
            )
            .unwrap());
            assert!(tracker.mark_published_ack(acknowledgement.as_ref().unwrap()).unwrap());
            close_ack(acknowledgement.take().unwrap());
            return Some(patch);
        }
        if !tracker.has_work() {
            return None;
        }
    }
    None
}

#[doc(hidden)]
pub fn published(tracker: &PatchTracker, surface: &str,cx:&mut semio_framework_job::StepContext<'_>) -> semio_framework_ui_runtime::SurfaceReconcilePublishedPatch {
    tracker.begin(surface.to_owned(), leaf("root", "published")).expect("admitted publication");
    for _ in 0..4_096 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
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

fn close_instance_to_empty(tracker: &PatchTracker, instance: u32,cx:&mut semio_framework_job::StepContext<'_>) {
    let key = NativeCloseKey::fixture(instance, 1);
    tracker.reserve_close_instance(key).expect("exact close reservation");
    tracker.activate_close_instance(key).expect("activate retained close");
    for _ in 0..65_536 {
        tracker.close_step(cx).expect("original patch fixture close step");
        if tracker.close_instance_complete(key).expect("exact close receipt") {
            tracker.release_close_instance(key).expect("final ACK releases close slot");
            assert!(tracker.terminal_is_empty());
            return;
        }
    }
    panic!("instance {instance} did not reach terminal empty: {}", tracker.debug_state());
}
