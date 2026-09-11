use super::*;
use super::turn::{PATCH_CLOSE_UNITS_PER_TURN, PATCH_RETIREMENT_BYTES_PER_UNIT, PATCH_RETIREMENT_ITEMS_PER_UNIT};

#[test]
fn patch_frame_limit_does_not_limit_internal_reconciliation_steps() {
    assert_eq!(reconcile_step_opportunities(512), 512);
    assert_eq!(reconcile_step_opportunities(15_640), RECONCILE_STEP_OPPORTUNITY_LIMIT as usize);
    assert_eq!(reconcile_step_opportunities(0), 1);
}

#[test]
fn reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps() {
    let instance = 991u32;
    let key = instance_lifetime::NativeCloseKey::fixture(instance, 1);
    reserve_reactor_close(key).expect("fixed reactor close admission");
    activate_reactor_close(key).expect("all reservations admitted before activation");
    let mut steps = 0usize;
    loop {
        REACTOR_CLOSE_CURSOR.with(|cursor| cursor.set(ReactorCloseRegistry::index(instance)));
        assert!(step_reactor_close().expect("one bounded reactor close opportunity"));
        steps += 1;
        if reactor_close_complete(key).expect("exact retained receipt") {
            break;
        }
        assert!(steps < 8_192, "fixed close cursor must terminate within its structural capacities");
    }
    assert!(steps > REACTOR_TASK_SLOTS + REACTOR_TIMER_SLOTS, "request, resume, task, timer, and metadata owners must retire across distinct opportunities");
    // ✅️ The task stage terminates on `cancel_instance_tasks_step`'s OWN witness, never on its
    // cursor: the production executor answers `Complete` without advancing the cursor once its
    // sweep has nothing left to visit, so a cursor-bound condition looped forever and no native
    // close ever reached `Retired` (26/09/09/PROCEDURAL-3D-END-TO-END). Production coverage lives
    // in `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs`, which links a non-test framework.
    assert!(
        REACTOR_CLOSES.with(|closes| closes.borrow().slots.get(ReactorCloseRegistry::index(instance)).is_some_and(|state| state.tasks_complete)),
        "the terminal reactor close must carry the task sweep's own completion witness"
    );
    assert!(reserve_reactor_close(instance_lifetime::NativeCloseKey::fixture(instance, 2)).is_err(), "terminal receipt holds its exact slot until ACK");
    release_reactor_close(key).expect("final exact receipt release");
    assert!(reactor_close_complete(key).is_err(), "absence is not a terminal receipt");
}

/// 🌍️ A world-3d scene the size the puzzle 3d editor publishes for the Nakagin Capsule Tower
/// (measured 2026-09-10: 11 lanes, 56 794 payload bytes, the `instances` lane 55 154 of them).
fn nakagin_scale_world_scene() -> semio_framework_ui_scene::World3dScene {
    let instances = (0..180)
        .map(|index| {
            format!(
                r#"{{"disabled":false,"id":"0189{index:04}-66f2-4544-98f0-b6f0c0615492","label":"Capsule With Balcony J · cs_sl{index}_d0_t_f{index}_b_c{index}","meshId":"mesh:capsule_J","objectKind":"Capsule With Balcony J","position":[{index}.45,-12.55,39.58],"rotation":[0,0,0.7071067811865475,0.7071067811865475],"scale":[1,1,1]}}"#
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let meshes = (0..14).map(|index| format!(r#"{{"id":"mesh:capsule_{index}","url":"/mesh/capsule_{index}.glb"}}"#)).collect::<Vec<_>>().join(",");
    let mut scene = semio_framework_ui_scene::World3dScene::base("{}".into(), format!("[{meshes}]"), format!("[{instances}]"), r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#.into());
    scene.vortices_json = Some("[]".into());
    scene.attractions_json = Some("[]".into());
    scene.target_volumes_json = Some("[]".into());
    scene.references_json = Some("[]".into());
    scene.interaction_json = Some(r#"{"hoveredId":null,"gridFactor":1,"granularity":"object","domain":"puzzle3d","selectionMode":"object"}"#.into());
    scene.lod_json = Some(r#"{"gridFactor":1,"automaticLod":true,"depthVariableLod":false,"manualLod":1}"#.into());
    scene.chunking_json = Some(r#"{"chunkSize":64,"radius":8000}"#.into());
    scene.environment_json = Some(r#"{"ambient":{"intensity":1.15},"sun":{"azimuth":0.5,"altitude":0.9}}"#.into());
    scene
}

/// 🩹️ Publishes one Nakagin-scale world surface through a FRESH tracker — its own, not the reactor's
/// thread-local one, so the law can publish the same document twice and price its retirement two
/// ways — and hands back the patch the turn would emit plus the reconcile turns it took at the
/// production per-turn opportunity budget.
fn publish_nakagin_world_patch(surface: &str) -> (ui_contract::UiPatch, usize, usize) {
    let node = crate::app::scene_surface("puzzle3d-main-perspective", semio_framework_ui_contract::SurfaceKind::World3d, &nakagin_scale_world_scene()).expect("nakagin world surface");
    let tree = crate::app::built_to_component_tree(node);
    let opportunities = reconcile_step_opportunities(50_000_000);
    let patches = patches::PatchTracker::new();
    patches.begin(surface.to_string(), tree).expect("mounted admission");
    let (mut turns, mut steps, mut owner) = (0usize, 0usize, None);
    'outer: loop {
        turns += 1;
        assert!(turns < 10_000, "reconcile never published after {steps} steps");
        for _ in 0..opportunities {
            if !patches.has_publishable_work() {
                break;
            }
            patches.drive_one();
            steps += 1;
            if let Some(ready) = patches.take_ready_patch() {
                owner = Some(ready);
                break 'outer;
            }
        }
    }
    let mut ready = owner.expect("published patch");
    let mut payload = ui_contract::UiPendingPatch::default();
    let mut published = None;
    ready.publish_into(&mut payload, &mut published, semio_framework_ui_runtime::SurfaceReconcileReadyPatch::required_publish_bytes()).expect("publication grant");
    let patch = payload.source_mut().expect("payload").take().expect("published patch");
    while !ready.close_step_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("ready close").complete {}
    if let Some(mut published) = published {
        while !published.close_step_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("published close").complete {}
    }
    for _ in 0..100_000 {
        if patches.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT) {
            break;
        }
    }
    (patch, turns, steps)
}

/// 🧹️ Retires one emitted patch at a given per-turn pacing and answers `(turns, units)` — turns being
/// the number of host round trips that pacing costs, because a turn the reactor still has retirement
/// work in is answered as `MoreWork`.
fn retirement_turns(patch: ui_contract::UiPatch, items_per_unit: usize, bytes_per_unit: usize, units_per_turn: usize) -> (usize, usize) {
    let mut owner = ui_contract::UiPendingPatch::default();
    *owner.source_mut().expect("writable payload") = Some(patch);
    let (mut turns, mut units) = (0usize, 0usize);
    loop {
        turns += 1;
        assert!(turns < 100_000, "retirement never completed");
        for _ in 0..units_per_turn {
            units += 1;
            if owner.close_step(items_per_unit, bytes_per_unit).expect("patch retirement").complete {
                return (turns, units);
            }
        }
    }
}

/// ⏱️ Wave W-S2 (ticket 26/09/02): the world-3d publication a 180-object document swap produces must
/// reach the host — and be RETIRED — in a bounded handful of reactor turns, because a turn the
/// reactor still has retirement work in is answered as `MoreWork` and costs the host one more round
/// trip. Measured in the browser 2026-09-10 before this wave: 24.3 s and 8 799 worker messages
/// between the Nakagin example click and the world repaint, with only 5.4 s of main-thread work in
/// it. Measured here: the reconcile is 3 turns, while retiring the very same patch takes over a
/// thousand units — and the reactor granted ONE item per unit, 8 units per turn.
///
/// Both clauses matter: the first is the bound the wave bought (and it reads the PRODUCTION
/// constants, so reverting either half of the pacing fails here instead of in a browser), the second
/// pins the pacing that made it necessary as still visibly expensive.
#[test]
fn a_nakagin_scale_world_publication_reconciles_and_retires_within_a_handful_of_reactor_turns() {
    let (patch, reconcile_turns, reconcile_steps) = publish_nakagin_world_patch("1:puzzle3d-main-perspective");
    assert!(reconcile_turns <= 8, "the reconcile itself is cheap: {reconcile_turns} turns for {reconcile_steps} steps");
    let (granted_turns, granted_units) = retirement_turns(patch, PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT, PATCH_CLOSE_UNITS_PER_TURN);
    let (dripped_turns, dripped_units) = retirement_turns(publish_nakagin_world_patch("2:puzzle3d-main-perspective").0, 1, 4_096, 8);
    assert!(dripped_units > 512, "this patch must be document-scaled for the bound below to mean anything; observed {dripped_units} retirement units");
    assert!(dripped_turns > 64, "the pre-W-S2 pacing is what this law guards against; it must stay visibly expensive, observed {dripped_turns} turns");
    assert!(granted_turns <= 8, "one world-3d publication must retire inside a handful of reactor turns; observed {granted_turns} turns over {granted_units} units against {dripped_turns} turns at one item per unit and 8 units per turn");
    eprintln!("[DEBUG] nakagin publication reconciled in {reconcile_turns} turns ({reconcile_steps} steps) and retires in {granted_turns} turns / {granted_units} units, against {dripped_turns} turns / {dripped_units} units at the pre-W-S2 pacing");
}

/// 📊️ An outliner-scale retained TABLE surface — the second surface the same Nakagin
/// `interactionSelect` turn republishes beside the world — queued for retirement exactly the way its
/// window queues it: the view is dropped, which hands its rows to the process-wide table-rows retire
/// arena. Answers the retirement units it queued, so the bound below can prove it is document-scaled.
fn queue_outliner_scale_table_surface() -> usize {
    let mut view = crate::app::TableRowsView::new(ui_contract::UiText::try_from_str("").expect("bounded actions header"));
    for column in ["Object", "Kind", "Mesh"] {
        view.try_push_column(ui_contract::UiText::try_from_str(column).expect("bounded column")).expect("outliner column capacity");
    }
    for index in 0..crate::app::TABLE_WINDOW_ROWS {
        let mut row = crate::app::TableRow::new(ui_contract::UiText::try_from_string(format!("0189{index:04}-66f2-4544-98f0-b6f0c0615492")).expect("bounded row id"));
        for cell in 0..crate::app::TABLE_WINDOW_CELLS {
            row.try_push_cell(ui_contract::UiText::try_from_string(format!("Capsule With Balcony J · cs_sl{index}_d0_t_f{cell}")).expect("bounded cell")).expect("outliner cell capacity");
        }
        view.try_push_row(row).expect("outliner row capacity");
    }
    drop(view);
    crate::app::TABLE_WINDOW_ROWS * (crate::app::TABLE_WINDOW_CELLS + 1) + 3 + 1
}

/// 🌍️ Mounts one Nakagin-scale world surface on its OWN tracker and publishes it, leaving the
/// tracker mounted so the caller can retire the surface owner through the terminal ladder — the
/// retained half W-S2 measured only through the emitted patch.
fn mount_and_publish_nakagin_world(surface: &str) -> (patches::PatchTracker, ui_contract::UiPatch) {
    let node = crate::app::scene_surface("puzzle3d-main-perspective", semio_framework_ui_contract::SurfaceKind::World3d, &nakagin_scale_world_scene()).expect("nakagin world surface");
    let tree = crate::app::built_to_component_tree(node);
    let tracker = patches::PatchTracker::new();
    tracker.begin(surface.to_string(), tree).expect("mounted admission");
    let mut owner = None;
    for _ in 0..5_000_000 {
        if !tracker.has_publishable_work() {
            break;
        }
        tracker.drive_one();
        if let Some(ready) = tracker.take_ready_patch() {
            owner = Some(ready);
            break;
        }
    }
    let mut ready = owner.expect("published patch");
    let mut payload = ui_contract::UiPendingPatch::default();
    let mut published = None;
    ready.publish_into(&mut payload, &mut published, semio_framework_ui_runtime::SurfaceReconcileReadyPatch::required_publish_bytes()).expect("publication grant");
    let patch = payload.source_mut().expect("payload").take().expect("published patch");
    while !ready.close_step_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("ready close").complete {}
    if let Some(mut published) = published {
        while !published.close_step_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("published close").complete {}
    }
    (tracker, patch)
}

/// ⏱️ Wave W-B2 (ticket 26/09/02): a MIXED-surface turn — the world-3d surface AND a retained table
/// surface (the outliner), which is what one Nakagin `interactionSelect` actually touches — must
/// retire every one of its ladders inside a SINGLE-DIGIT number of reactor turns.
///
/// W-S2 priced only the world-3d patch ladder per page and recorded in its own §7 that the terminal
/// ladders (`SurfaceReconcileTerminal`/`MountedTreeTerminal`) and the `…_one()` ladders
/// (`close_table_rows_view_one`, `close_ui_turn_patch_transport_one`) still retire ONE owner per
/// unit. A turn holding ANY of them answers `MoreWork`, the host counts that as one continuation
/// toward the SAME `PLUGIN_UI_CONTINUATION_LIMIT` regardless of which surface is stalling it, and
/// `interactionSelect` on Nakagin still throws `did not publish its requested UI surfaces within
/// 4096 continuations` (ticket evidence `🗑generated/w-ab-41-nakagin-brush4.txt`, 2026-09-10).
#[test]
fn a_nakagin_scale_mixed_surface_turn_retires_every_ladder_within_a_handful_of_reactor_turns() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let instance = 3u32;
    let (tracker, patch) = mount_and_publish_nakagin_world(&format!("{instance}:puzzle3d-main-perspective"));
    let mut world = ui_contract::UiPendingPatch::default();
    *world.source_mut().expect("writable payload") = Some(patch);
    let table_units = queue_outliner_scale_table_surface();
    let key = instance_lifetime::NativeCloseKey::fixture(instance, 1);
    tracker.reserve_close_instance(key).expect("exact close reservation");
    tracker.activate_close_instance(key).expect("activate retained close");
    let (mut turns, mut world_done, mut table_done, mut surface_done) = (0usize, false, false, false);
    while !world_done || !table_done || !surface_done {
        turns += 1;
        assert!(turns < 100_000, "mixed-surface retirement never completed after {turns} turns");
        for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
            if world_done {
                break;
            }
            world_done = world.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("world patch retirement").complete;
        }
        for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
            if tracker.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT) {
                break;
            }
        }
        for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
            if semio_framework_ui_runtime::close_surface_reconcile_handback_one().expect("surface handback retirement") {
                break;
            }
        }
        surface_done = tracker.close_instance_complete(key).expect("exact close receipt");
        table_done = false;
        for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
            if !crate::app::close_table_rows_view_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT) {
                table_done = true;
                break;
            }
        }
    }
    tracker.release_close_instance(key).expect("final ACK releases close slot");
    let dripped_units = {
        queue_outliner_scale_table_surface();
        let mut units = 0usize;
        while crate::app::close_table_rows_view_one() {
            units += 1;
            assert!(units < 1_000_000, "the pre-W-B2 table ladder never terminated");
        }
        units
    };
    assert!(dripped_units > 512, "the retained table must be document-scaled for the bound below to mean anything; observed {dripped_units} retirement units against {table_units} queued");
    assert!(turns < 10, "a mixed world-3d + retained-table turn must retire every ladder inside a single-digit number of reactor turns; observed {turns} turns against the {dripped_units} turns the same table costs at one owner per turn");
    eprintln!("[DEBUG] mixed-surface retirement completed in {turns} turns, against {dripped_units} turns for the retained table alone at the pre-W-B2 one-owner-per-turn pacing ({table_units} units queued)");
}
