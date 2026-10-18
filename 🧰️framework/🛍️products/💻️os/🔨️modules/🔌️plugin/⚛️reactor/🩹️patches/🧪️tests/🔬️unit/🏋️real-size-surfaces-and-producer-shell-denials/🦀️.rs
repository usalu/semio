/// 🏋️ Real-size surfaces and producer shell denials: the sized body helper, twelve mounted surfaces at their real sizes and the original producer shell denials.

/// 📐️ A body of roughly `bytes` of authored text, spread over as many bounded `UiText` rows as the fixed
/// document node ceiling admits — the shape of a real pane body rather than a one-node fixture.
fn sized_body(key: &str, bytes: usize) -> TreeNode {
    let filler = "n".repeat(480);
    let rows = bytes.div_ceil(480).min(120);
    let mut root = leaf(key, "body").root;
    for group in 0..rows.div_ceil(30) {
        let mut branch = leaf(&format!("{key}-group-{group}"), "group").root;
        for row in (group * 30)..((group + 1) * 30).min(rows) {
            branch.children.try_push(leaf(&format!("{key}-{row}"), &filler).root).expect("bounded fixture row");
        }
        root.children.try_push(branch).expect("bounded fixture group");
    }
    root
}

/// 🎫️ WAVE B58 LAW: every surface a session mounts must hold a reconcile reservation AT ONCE, priced by
/// the body it is actually reconciling.
///
/// 🐛️ One reservation asked for `SurfaceReconcileLimits::default().max_bytes` — the per-surface CEILING
/// `UI_RESIDENT_SURFACE_BYTES`, 8 MiB — of a 32 MiB `UI_RESIDENT_AGGREGATE_BYTES`, so exactly THREE
/// surfaces could reconcile process-wide while a puzzle3d session mounts THIRTEEN (three panes carrying a
/// ~54 KB world body, ten panels of a few KB). Every fourth surface was refused
/// `registry-resident-credit-exhausted`, `reserve_mounted` handed it back, the turn deferred it,
/// `redirty_acknowledged_deferred_surfaces` re-dirtied it, and the reactor answered `more-work` with
/// `effects=0` for ever — so a mutation's world, document and panel bodies never all landed and
/// `deleteSelection` measured 135 s with the object census unchanged at 180 (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B54 §8.2, wave B56 §1.1 and §6). The ceiling is a per-surface
/// MAXIMUM, never a price, and this law fixes the aggregate to the session's real occupancy.
#[test]
fn twelve_mounted_surfaces_at_their_real_sizes_all_hold_a_reconcile_reservation_at_once() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let baseline = semio_framework_ui_runtime::surface_reconcile_registry_census();
    let tracker = PatchTracker::new();
    let panes = ["puzzle3d-main", "puzzle3d-main-top", "puzzle3d-main-perspective"];
    let panels = [
        "framework.panel.artifact",
        "framework.panel.catalogue",
        "framework.panel.inspection",
        "puzzle3d.panel.settings",
        "framework.panel.history",
        "framework.section.engagements",
        "framework.section.measures",
        "framework.section.tools",
        "framework.section.catalogue",
    ];
    let mut mounted = Vec::new();
    for (index, body) in panes.iter().map(|name| (*name, 54 * 1024)).chain(panels.iter().enumerate().map(|(index, name)| (*name, 1024 + index * 768))).enumerate() {
        let (name, bytes) = body;
        let surface = format!("58:{name}");
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from(surface.clone()).expect("bounded surface"))
            .unwrap_or_else(|_| panic!("surface {index} of a twelve-surface session must hold a reservation priced by its own {bytes}-byte body: {}", tracker.debug_state()));let mut source=Some(ComponentTree::new(sized_body(name, bytes)));assert!(grant.commit_source(&mut source,cx)
            .unwrap_or_else(|_| panic!("surface {surface} commits its rendered body: {}", tracker.debug_state())), "actual original source admitted");assert!(source.is_none());}
        mounted.push(surface);
    }
    let state = tracker.debug_state();
    assert!(state.contains("reserve_refusal=none"), "no surface of a mounted session may be refused its reconcile reservation: {state}");
    let census = semio_framework_ui_runtime::surface_reconcile_registry_census();
    assert_eq!(census.resident_slots, baseline.resident_slots + mounted.len(), "every mounted surface must hold its own reservation at once: {census:?} against {baseline:?} — {state}");
    assert!(
        census.resident_bytes - baseline.resident_bytes < mounted.len() * ui_contract::UI_RESIDENT_SURFACE_BYTES,
        "twelve reservations priced at the per-surface ceiling cannot fit the aggregate, so they must be priced by their bodies: {census:?} against {baseline:?}"
    );
    let mut peak = census.resident_bytes;
    let mut published = 0usize;
    for _ in 0..1_048_576 {
        tracker.drive_one(cx).expect("original patch fixture semantic step");
        peak = peak.max(semio_framework_ui_runtime::surface_reconcile_registry_census().resident_bytes);
        if let Some(owner) = tracker.take_ready_patch() {
            let (patch, authority) = publish_test(owner);
            close_published(authority);
            close_test_patch(patch);
            published += 1;
            if published == mounted.len() {
                break;
            }
        }
    }
    assert_eq!(published, mounted.len(), "every mounted surface must publish its body within one turn set: {}", tracker.debug_state());
    assert!(
        peak - baseline.resident_bytes < mounted.len() * ui_contract::UI_RESIDENT_SURFACE_BYTES,
        "twelve concurrent reconciles must never hold twelve per-surface ceilings: peak {peak} against baseline {} — {}",
        baseline.resident_bytes,
        tracker.debug_state()
    );
    eprintln!(
        "{} surfaces admitted at floor={}B each; reserved={}B for {} slots (mean {}B); peak while reconciling and publishing all of them={}B (mean {}B per surface) of {}B aggregate against a {}B per-surface ceiling; baseline={}B",
        mounted.len(),
        semio_framework_ui_runtime::SURFACE_RECONCILE_FLOOR_BYTES,
        census.resident_bytes - baseline.resident_bytes,
        census.resident_slots - baseline.resident_slots,
        (census.resident_bytes - baseline.resident_bytes) / mounted.len(),
        peak - baseline.resident_bytes,
        (peak - baseline.resident_bytes) / mounted.len(),
        ui_contract::UI_RESIDENT_AGGREGATE_BYTES,
        ui_contract::UI_RESIDENT_SURFACE_BYTES,
        baseline.resident_bytes
    );
    close_instance_to_empty(&tracker, 58,cx);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}


#[test]
fn mounted_original_producer_shell_denials_unchanged_child_census_and_paid_terminal_release() {
    use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller};
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
    fn clock()->Option<u64>{Some(1000)}
    fn observe<T>(run:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,heap.requested_bytes,heap.released_bytes)}
    let _guard=semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/🧊️wgpu/🎬️media-slots/🎟️collect/🔣️.json")).unwrap();
    let stock=RetainedCloneGrant{maximum_items:1000000,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:256};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(17),stock,clock,1000000,1000000);
    let tracker=PatchTracker::new();let key=NativeCloseKey::fixture(941,1);
    let mut grant=tracker.reserve_mounted(ui_contract::SurfaceId::try_from("941:original-producer").unwrap(),key).unwrap();
    let(mut source,source_birth,source_release)=observe(||Some(tree_with_owned_child("original-é")));assert_eq!(source_release,0);
    let child=source.as_ref().unwrap().root.children.get(0).unwrap()as*const TreeNode as usize;
    let generation=grant.generation;let capacity=ComponentTreeProducer::required_capacity_bytes();
    for denied in fixture["originalTreeProducer"]["capacityDenials"].as_array().unwrap(){
        let denial=RetainedCloneGrant{maximum_capacity_bytes:denied.as_u64().unwrap()as usize,..stock};
        let(admitted,born,released)=observe(||caller.turn(denial,|cx|grant.commit_source(&mut source,cx)).unwrap());
        assert!(!admitted);assert_eq!((born,released),(0,0));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert!(grant.active);assert!(matches!(grant.owner,MountedReconcileOwner::Live{..}));assert_eq!(source.as_ref().unwrap().root.children.get(0).unwrap()as*const TreeNode as usize,child);
    }
    let(admitted,born,released)=observe(||caller.turn(RetainedCloneGrant{maximum_copy_bytes:0,maximum_capacity_bytes:capacity,..stock},|cx|grant.commit_source(&mut source,cx)).unwrap());assert!(admitted);assert!(source.is_none());assert_eq!((born,released),(capacity,0));assert_eq!(caller.receipt().retained_capacity_bytes,capacity);assert_eq!(caller.receipt().copied_bytes,0);drop(grant);
    let mut producer=tracker.state.borrow_mut().slots.iter_mut().flatten().find(|slot|slot.key==key).unwrap().producer.take().unwrap();
    for _ in 0..256{
        assert_eq!(producer.authority.original_source().unwrap().root.children.get(0).unwrap()as*const TreeNode as usize,child);
        let(step,born,released)=observe(||caller.turn(RetainedCloneGrant{maximum_copy_bytes:0,..stock},|cx|producer.authority.step_on_original(generation,false,cx)).unwrap());assert_eq!((born,released),(0,0));assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),(0,0));if step==ComponentTreeProducerStep::Complete{break}
    }
    assert!(producer.authority.has_complete());
    let tree=producer.authority.take_complete().unwrap();assert_eq!(tree.root.children.get(0).unwrap()as*const TreeNode as usize,child);
    let mut tree=ui_contract::OriginalBuiltTreeRetirement::new(tree.root);let mut freed=0;let mut close_birth=0;
    for _ in 0..4096{if tree.terminal_is_empty(){break}let(_,born,released)=observe(||caller.turn(stock,|cx|{let demand=tree.next_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|loan|tree.step(loan))?;Ok(())}).unwrap());assert_eq!((born,released),(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));freed+=released;close_birth+=born;}
    assert!(tree.terminal_is_empty());assert_eq!(source_birth+close_birth,freed);assert_eq!(observe(||drop(tree)).1,0);
    let mut terminal=MountedTreeTerminal{key,instance:Some(941),surface_index:None,surface:ui_contract::SurfaceId::try_from("941:original-producer").unwrap(),reconciler:None,reservation:None,authority:Some(producer.authority),close:true};
    let(done,born,released)=observe(||caller.turn(RetainedCloneGrant{maximum_release_bytes:capacity-1,..stock},|cx|terminal.close_step(cx)).unwrap());assert!(!done);assert_eq!((born,released),(0,0));assert!(terminal.authority.is_some());assert_eq!(caller.receipt(),Default::default());
    let(done,born,released)=observe(||caller.turn(RetainedCloneGrant{maximum_release_bytes:capacity,..stock},|cx|terminal.close_step(cx)).unwrap());assert!(!done);assert_eq!((born,released),(0,capacity));assert_eq!(caller.receipt().released_bytes,capacity);assert!(terminal.terminal_is_empty());assert_eq!(observe(||drop(terminal)).1,0);
    {let mut state=tracker.state.borrow_mut();let slot=state.slots.iter_mut().flatten().find(|slot|slot.key==key).unwrap();slot.reconciler=producer.reconciler.take();}drop(producer.reservation.take());
    caller.turn(stock,|cx|{close_instance_to_empty(&tracker,941,cx);Ok(())}).unwrap();caller.retire(stock);
}
