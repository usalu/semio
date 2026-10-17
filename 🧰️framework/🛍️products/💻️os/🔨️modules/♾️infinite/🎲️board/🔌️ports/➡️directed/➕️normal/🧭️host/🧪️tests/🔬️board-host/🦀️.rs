//! 🧪️ normal-host tests (🔬️board-host): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_schema::NodeDescriptor;
use semio_framework_os_infinite_board_engine::{ActiveUtility, BoardElementStyleKind, GestureStage, HandleDescriptor, Interaction, RegionGrip, SceneDescriptor};
use semio_framework_os_infinite_board_normal_model::{BOARD_DESCRIPTOR_ITEM_CAPACITY, BOARD_EVENT_ITEM_CAPACITY, BOARD_POINTER_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, board_pointer_offset, BoardDrawLod, BoardEventKind, BoardFillStage, BoardNaming, BoardOwnedEvent, BoardPlacementNaming, StyleChromePass, WorldContentCache};
use semio_framework_os_infinite_board_normal_scene::{board_edge_curve, BoardPointerPlan, BoardPointerPlanFault, BoardPointerPlanKind, effective_handle_radius, handle_positions_json, region_snap_step};
use semio_framework_os_infinite_board_normal_focus::{BoardPointerIntent, BoardPointerPhase, commit_wheel, interaction_overlay_entity_ids, load_board_snapshot_json, resolve_node_style_kind, set_camera_silent, set_selection_ids, set_selection_ids_silent, wheel_screen};
use semio_framework_os_infinite_board_normal_gestures::{BoardAuthorityStep, BoardEntityRetirement, BoardPointerPlanRetirement, BoardRemovedEntity, event_authority_terminal_is_empty, pointer_lane_is_direct, quarantine_world_content_step, set_transform_flags, target_regions_json};
use semio_framework_os_infinite_board_normal_fill::{BoardFillJob, BoardFillSnapshotIngress, NEUTRAL_BOARD_NAMING};
use semio_framework_os_infinite_board_normal_host::{begin_pointer_commit, board_delete_selection, BoardHost, BoardHostRetirement};
use semio_framework_test_fixtures::fixture_text;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_board_normal_host::plan_delete_selection;
use semio_framework_canvas::{Affine, Color, FillRule, Point, Scene, Vec2};
use semio_framework_value::NativeDecodeControl;
use semio_framework_graph_manifest::PropertyValue;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_pack_json::Value;
use semio_framework_graph_drawing::routing::handle_position_on_circle;
use semio_framework_value::retirement::sequence;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

#[cfg(test)]
    fn with_board_step_context<T>(fuel: u64, cancel: semio_framework_job::CancelToken, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
        let mut sequence = 0;
        let mut receipt=semio_framework_value::RetainedCloneProgress::default();
        let policy:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️board-host-standalone/🧫️descriptor.json")).unwrap();
        let axis=|name:&str|policy["contextGrant"][name].as_u64().unwrap()as usize;
        let grant=semio_framework_value::RetainedCloneGrant{maximum_items:axis("items"),maximum_copy_bytes:axis("copyBytes"),maximum_capacity_bytes:axis("capacityBytes"),maximum_release_bytes:axis("releaseBytes"),maximum_depth:axis("depth")};
        let mut context =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX,grant), cancel, semio_framework_job::default_now_us, &mut sequence,&mut receipt);
        let result=step(&mut context);
        assert!(context.retained_progress().fits(grant));
        result
    }

#[cfg(test)]
    fn drive_pointer_commit(host: &mut BoardHost) {
        let cancel = semio_framework_job::root_cancel_token();
        for _ in 0..4096 {
            match with_board_step_context(1, cancel.clone(), |context| host.step_pointer_commit(context)) {
                BoardAuthorityStep::Pending => {}
                BoardAuthorityStep::Complete => return,
                other => panic!("pointer authority failed: {other:?}"),
            }
        }
        panic!("pointer authority did not reach a terminal step");
    }

#[cfg(test)]
    #[test]
    fn wheel_plan_matches_direct_and_rejects_stale_interaction() {
        let mut direct = BoardHost::default();
        let mut planned = BoardHost::default();
        direct.scene.set_size(800, 600, 1.0);
        planned.scene.set_size(800, 600, 1.0);
        wheel_screen(&mut direct.scene, &mut direct.outbox, 320.0, 240.0, -12.0);
        let plan = planned.scene.plan_wheel(320.0, 240.0, -12.0);
        assert!(commit_wheel(&mut planned.scene, &mut planned.outbox, &plan));
        assert_eq!([direct.scene.camera.x, direct.scene.camera.y, direct.scene.camera.zoom], [planned.scene.camera.x, planned.scene.camera.y, planned.scene.camera.zoom]);

        let stale = planned.scene.plan_wheel(320.0, 240.0, -12.0);
        planned.pointer_down_screen(10.0, 10.0, 1, false, false);
        let replacement = [planned.scene.camera.x, planned.scene.camera.y, planned.scene.camera.zoom];
        assert!(!commit_wheel(&mut planned.scene, &mut planned.outbox, &stale));
        assert_eq!([planned.scene.camera.x, planned.scene.camera.y, planned.scene.camera.zoom], replacement);
    }

#[cfg(test)]
    #[test]
    fn pointer_pan_plan_matches_direct_and_rejects_stale_interaction() {
        let mut direct = BoardHost::default();
        let mut planned = BoardHost::default();
        direct.scene.set_size(800, 600, 1.0);
        planned.scene.set_size(800, 600, 1.0);
        direct.pointer_down_screen(100.0, 120.0, 1, false, false);
        planned.pointer_down_screen(100.0, 120.0, 1, false, false);
        direct.pointer_move_screen(140.0, 150.0, false, false, false);
        let plan = planned.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 140.0, y: 150.0, shift: false, ctrl_or_meta: false, alt: false }).expect("pan plan");
        begin_pointer_commit(&mut planned.scene, &mut planned.commit, plan).expect("retained pan plan");
        drive_pointer_commit(&mut planned);
        assert_eq!([direct.scene.camera.x, direct.scene.camera.y, direct.scene.camera.zoom], [planned.scene.camera.x, planned.scene.camera.y, planned.scene.camera.zoom]);

        let stale = planned.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 160.0, y: 170.0, shift: false, ctrl_or_meta: false, alt: false }).expect("stale pan plan");
        planned.scene.set_size(801, 600, 1.0);
        let camera = [planned.scene.camera.x, planned.scene.camera.y, planned.scene.camera.zoom];
        assert!(begin_pointer_commit(&mut planned.scene, &mut planned.commit, stale).is_err());
        assert_eq!([planned.scene.camera.x, planned.scene.camera.y, planned.scene.camera.zoom], camera);
    }

#[cfg(test)]
    #[test]
    fn pointer_plan_rejects_drag_item_overflow_and_retires_one_delta_per_step() {
        let mut host = BoardHost::default();
        let start_positions = (0..=BOARD_POINTER_ITEM_CAPACITY).map(|index| (format!("node-{index}"), (index as f64, 0.0))).collect();
        host.focus.interaction = Interaction::DragNodes { primary_id: "node-0".into(), offset: Vec2::ZERO, start_positions, proximity_pair: None, gesture: GestureStage::default(), delta: Vec2::ZERO };
        assert_eq!(host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 1.0, y: 1.0, shift: false, ctrl_or_meta: false, alt: false }).unwrap_err(), BoardPointerPlanFault::ItemCredits);

        let mut plan = BoardPointerPlan::empty(0, BoardPointerPlanKind::DragMove);
        plan.push_delta("a", 1.0, 2.0).unwrap();
        plan.push_delta("b", 3.0, 4.0).unwrap();
        let mut retirement = BoardPointerPlanRetirement::new(plan);
        assert!(!retirement.close_step());
        assert!(!retirement.close_step());
        assert!(retirement.close_step());
        assert!(retirement.terminal_is_empty());

        let release = |id: &str| {
            let mut host = BoardHost::default();
            host.focus.interaction = Interaction::DragNodes { primary_id: id.into(), offset: Vec2::ZERO, start_positions: [(id.to_string(), (0.0, 0.0))].into_iter().collect(), proximity_pair: None, gesture: GestureStage { id: "gesture-9".into(), select: None, restore: None }, delta: Vec2::ZERO };
            (host.scene.screen_to_world(Point::new(1.0, 2.0)), host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Up, x: 1.0, y: 2.0, shift: false, ctrl_or_meta: false, alt: false }))
        };
        let (world, escaped) = release("a\"\\\n");
        let value: serde_json::Value = serde_json::from_str(escaped.expect("escaped release plan").events_json()).unwrap();
        assert_eq!(value[0]["name"], "gesture");
        assert_eq!(value[0]["payload"]["gestureId"], "gesture-9");
        assert_eq!(value[0]["payload"]["targets"][0], "a\"\\\n");
        assert_eq!((value[0]["payload"]["dx"].as_f64(), value[0]["payload"]["dy"].as_f64()), (Some(world.x), Some(world.y)), "the record's offset is the release point's world offset");
        assert_eq!(release(&"\n".repeat(BOARD_POINTER_BYTE_CAPACITY / 2)).1.unwrap_err(), BoardPointerPlanFault::ByteCredits);
    }

/// 🪜️ The tail is cooperative and bounded, never a fixed ladder length: every commit phase a
/// gesture gains (the proximity pair, …) adds a turn, and pinning the exact count made this law
/// red for every such addition while proving nothing the `Pending`-until-`Complete` shape does not.
#[cfg(test)]
    #[test]
    fn drag_commit_obeys_zero_budget_one_delta_turn_cancel_and_publication_witness() {
        let mut host = deletion_fixture("node-a");
        host.focus.interaction = Interaction::DragNodes { primary_id: "node-a".into(), offset: Vec2::ZERO, start_positions: [("node-a".to_string(), (0.0, 0.0)), ("node-b".to_string(), (20.0, 0.0))].into_iter().collect(), proximity_pair: None, gesture: GestureStage::default(), delta: Vec2::ZERO };
        let plan = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Up, x: 10.0, y: 5.0, shift: false, ctrl_or_meta: false, alt: false }).expect("finish drag plan");
        begin_pointer_commit(&mut host.scene, &mut host.commit, plan).expect("retained drag commit");
        let before = host.scene.nodes.get("node-a").map(|node| (node.x, node.y));
        let zero = semio_framework_job::root_cancel_token();
        assert_eq!(with_board_step_context(0, zero, |context| host.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(host.scene.nodes.get("node-a").map(|node| (node.x, node.y)), before);

        let live = semio_framework_job::root_cancel_token();
        assert_eq!(with_board_step_context(1, live.clone(), |context| host.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_ne!(host.scene.nodes.get("node-a").map(|node| (node.x, node.y)), before, "the FIRST live turn applies the delta");
        let mut turns = 1usize;
        loop {
            turns += 1;
            assert!(turns <= 64, "the drag commit never completed");
            match with_board_step_context(1, live.clone(), |context| host.step_pointer_commit(context)) {
                BoardAuthorityStep::Pending => {}
                BoardAuthorityStep::Complete => break,
                other => panic!("the drag commit must only yield or complete, got {other:?}"),
            }
        }
        assert!(turns > 2, "the commit must yield at least once before completing");
        let publication = host.commit.take_pointer_publication().expect("complete drag publication");
        let rows: serde_json::Value = serde_json::from_str(publication.events_json()).expect("publication rows");
        let world = host.scene.screen_to_world(Point::new(10.0, 5.0));
        assert_eq!(rows.as_array().map(Vec::len), Some(1), "a release that staged no selection publishes the gesture record alone: {rows}");
        assert_eq!((rows[0]["name"].as_str(), rows[0]["payload"]["kind"].as_str(), rows[0]["payload"]["dx"].as_f64(), rows[0]["payload"]["dy"].as_f64()), (Some("gesture"), Some("drag"), Some(world.x), Some(world.y)));
        assert_eq!(rows[0]["payload"]["targets"], serde_json::json!(["node-a", "node-b"]));
        assert!(host.commit.pointer_authority_terminal_is_empty());

        let mut cancelled = deletion_fixture("node-a");
        cancelled.focus.interaction = Interaction::DragNodes { primary_id: "node-a".into(), offset: Vec2::ZERO, start_positions: [("node-a".to_string(), (0.0, 0.0))].into_iter().collect(), proximity_pair: None, gesture: GestureStage::default(), delta: Vec2::ZERO };
        let plan = cancelled.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 8.0, y: 3.0, shift: false, ctrl_or_meta: false, alt: false }).expect("cancelled drag plan");
        begin_pointer_commit(&mut cancelled.scene, &mut cancelled.commit, plan).expect("cancelled retained commit");
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        assert_eq!(with_board_step_context(1, cancel.clone(), |context| cancelled.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, cancel, |context| cancelled.step_pointer_commit(context)), BoardAuthorityStep::Cancelled);
        assert_eq!(cancelled.scene.nodes.get("node-a").map(|node| (node.x, node.y)), Some((0.0, 0.0)));
        assert!(cancelled.commit.pointer_authority_terminal_is_empty());
    }

#[cfg(test)]
    #[test]
    fn selection_move_and_up_are_revisioned_fixed_page_plans() {
        let mut host = BoardHost::default();
        host.scene.set_size(800, 600, 1.0);
        host.pointer_down_screen(100.0, 100.0, 0, false, false);
        assert!(matches!(host.focus.interaction, Interaction::SelectionPending { .. }));
        let preview = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 180.0, y: 160.0, shift: false, ctrl_or_meta: false, alt: false }).expect("selection preview plan");
        assert_eq!(preview.event_count(), 1);
        assert!(preview.events_json().contains("preselect"));
        begin_pointer_commit(&mut host.scene, &mut host.commit, preview).expect("retained selection preview");
        drive_pointer_commit(&mut host);
        let mut publication = host.commit.take_pointer_publication().expect("preview publication");
        assert!(publication.close_step());
        assert!(matches!(host.focus.interaction, Interaction::Selection { .. }));

        let stale = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 200.0, y: 180.0, shift: false, ctrl_or_meta: false, alt: false }).expect("stale selection plan");
        host.scene.interaction_revision = host.scene.interaction_revision.wrapping_add(1);
        assert!(begin_pointer_commit(&mut host.scene, &mut host.commit, stale).is_err());

        let commit = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Up, x: 200.0, y: 180.0, shift: false, ctrl_or_meta: false, alt: false }).expect("selection commit plan");
        assert_eq!(commit.event_count(), 1);
        assert!(commit.events_json().contains("select"));
        begin_pointer_commit(&mut host.scene, &mut host.commit, commit).expect("retained selection commit");
        drive_pointer_commit(&mut host);
        assert!(matches!(host.focus.interaction, Interaction::None));
    }

#[cfg(test)]
    #[test]
    fn non_drag_pointer_commit_yields_between_selection_link_and_brush_items() {
        let live = semio_framework_job::root_cancel_token();
        let mut selection = deletion_fixture("node-a");
        selection.focus.interaction = Interaction::SelectionPending { initial_ids: ["node-a".to_owned()].into_iter().collect(), start: Point::new(0.0, 0.0), start_screen: Point::new(0.0, 0.0) };
        let mut selection_plan = BoardPointerPlan::empty(selection.scene.interaction_revision, BoardPointerPlanKind::SelectionCommit);
        selection_plan.push_delta("node-a", 0.0, 0.0).unwrap();
        selection_plan.seal_events().unwrap();
        begin_pointer_commit(&mut selection.scene, &mut selection.commit, selection_plan).unwrap();
        assert_eq!(with_board_step_context(0, live.clone(), |context| selection.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert!(matches!(selection.focus.interaction, Interaction::SelectionPending { .. }));
        assert_eq!(with_board_step_context(1, live.clone(), |context| selection.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert!(matches!(selection.focus.interaction, Interaction::None));
        drive_pointer_commit(&mut selection);
        assert!(selection.focus.selection.contains("node-a"));

        let mut link = BoardHost::default();
        link.focus.interaction = Interaction::LinkAtSourceHandle { source_id: "source".into(), start_screen: Point::new(0.0, 0.0) };
        let mut link_plan = BoardPointerPlan::empty(link.scene.interaction_revision, BoardPointerPlanKind::Idle);
        let source = link_plan.push_id("source").unwrap();
        let compat_key = link_plan.push_id("source|").unwrap();
        let ring_key = link_plan.push_id("source||").unwrap();
        link_plan.kind = BoardPointerPlanKind::LinkMove { source, target: None, hover: None, compat_key, ring_key, end_world: Point::new(2.0, 3.0), activated: true, start_screen: Point::new(0.0, 0.0) };
        begin_pointer_commit(&mut link.scene, &mut link.commit, link_plan).unwrap();
        assert_eq!(with_board_step_context(1, live.clone(), |context| link.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert!(link.outbox.link_compat_nodes_emit_key.is_none());
        assert_eq!(with_board_step_context(1, live.clone(), |context| link.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(link.outbox.link_compat_nodes_emit_key.as_deref(), Some("source|"));
        drive_pointer_commit(&mut link);

        let mut brush = BoardHost::default();
        brush.focus.active_utility = ActiveUtility::Brush;
        let mut brush_plan = BoardPointerPlan::empty(brush.scene.interaction_revision, BoardPointerPlanKind::Idle);
        brush_plan.push_delta("kind-a", 0.0, 0.0).unwrap();
        brush_plan.push_delta("kind-b", 1.0, 0.0).unwrap();
        brush_plan.kind = BoardPointerPlanKind::Brush { source: None, hover: None, alt: false, commit_old: false };
        begin_pointer_commit(&mut brush.scene, &mut brush.commit, brush_plan).unwrap();
        assert_eq!(with_board_step_context(1, live.clone(), |context| brush.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(brush.brush.brush_candidates.len(), 0);
        assert_eq!(with_board_step_context(1, live, |context| brush.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(brush.brush.brush_candidates.len(), 0);
        drive_pointer_commit(&mut brush);
        assert_eq!(brush.brush.brush_candidates.len(), 2);
    }

#[cfg(test)]
    #[test]
    fn selection_event_reservation_is_flat_exact_and_precedes_mutation() {
        let mut host = BoardHost::default();
        set_selection_ids(&mut host.scene, &mut host.focus, &mut host.outbox, &["a\"\\\n".into(), "b".into()]);
        assert_eq!(host.focus.selection.iter().cloned().collect::<Vec<_>>(), vec!["a\"\\\n".to_string(), "b".to_string()]);
        let event = host.outbox.pop_owned_event().expect("flat select event");
        assert_eq!(event.kind(), BoardEventKind::Select);
        let payload: serde_json::Value = serde_json::from_str(event.payload_json()).unwrap();
        assert_eq!(payload["ids"], serde_json::json!(["a\"\\\n", "b"]));

        let mut saturated = BoardHost::default();
        for _ in 0..BOARD_EVENT_ITEM_CAPACITY {
            saturated.outbox.events.push(BoardOwnedEvent::from_payload(BoardEventKind::Hover, r#"{"id":null}"#, None).unwrap()).unwrap();
        }
        set_selection_ids(&mut saturated.scene, &mut saturated.focus, &mut saturated.outbox, &["retained".into()]);
        assert!(saturated.focus.selection.is_empty());
        assert!(saturated.outbox.event_overflow.is_some());
        assert_eq!(saturated.outbox.events.len(), BOARD_EVENT_ITEM_CAPACITY);
    }

#[cfg(test)]
    #[test]
    fn board_snapshot_json_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🔣️board-ingress.json")).unwrap();
        for fixture in vectors.as_array().unwrap() {
            let mut host = BoardHost::default();
            assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&fixture.to_string(),&mut control)});
            assert_eq!(host.scene.nodes.len(), fixture["nodes"].as_array().unwrap().len());
            for node in fixture["nodes"].as_array().unwrap() {
                let actual = &host.scene.nodes[node["id"].as_str().unwrap()];
                assert_eq!(actual.x, node["x"].as_f64().unwrap());
                assert_eq!(actual.y, node["y"].as_f64().unwrap());
            }
            let count = host.scene.nodes.len();
            assert!(!{let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,"{",&mut control)});
            assert_eq!(host.scene.nodes.len(), count);
        }
    }

#[cfg(test)]
    fn deletion_fixture(node_id: &str) -> BoardHost {
        let mut host = BoardHost::default();
        let fixture = serde_json::json!({
            "schema": "board.normal.undirected.v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": node_id, "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0 },
                { "id": "node-b", "x": 20.0, "y": 0.0, "shape": "circle", "radius": 10.0 }
            ],
            "edges": [{ "id": "edge-a-b", "source": node_id, "target": "node-b" }]
        });
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&fixture.to_string(),&mut control)});
        while host.outbox.pop_owned_event().is_some() {}
        set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &[node_id.to_string()]);
        host
    }

/// 🪜️ Cooperative and bounded, never an exact ladder length: the first turn above is already
/// proven not to close, and how many more a cancelled delete needs is a property of the planner's
/// pending state, not of this law (it was pinned at `> 4` and went red the moment that shrank).
#[cfg(test)]
    #[test]
    fn delete_plan_retains_fifo_until_exact_credits_and_rejects_stale_or_oversized() {
        let mut saturated = deletion_fixture("node-a");
        for _ in 0..BOARD_EVENT_ITEM_CAPACITY {
            saturated.outbox.events.push(BoardOwnedEvent::hover(None, None).unwrap()).unwrap();
        }
        board_delete_selection(&mut saturated.scene, &mut saturated.outbox, &mut saturated.commit);
        assert!(saturated.scene.nodes.contains_key("node-a"));
        assert!(saturated.commit.pending_delete_planning.is_some());
        let live = semio_framework_job::root_cancel_token();
        let mut turns = 0usize;
        while saturated.commit.pending_delete_planning.is_some() || saturated.commit.pending_delete_operation.is_some() {
            turns += 1;
            assert!(turns <= 768);
            let _ = with_board_step_context(1, live.clone(), |context| saturated.step_event_authority(context));
            if let Some(event) = saturated.outbox.pop_owned_event() {
                assert_eq!(event.kind(), BoardEventKind::Hover);
            }
        }
        assert!(turns > 4);
        assert!(saturated.commit.pending_delete_operation.is_none());
        assert!(!saturated.scene.nodes.contains_key("node-a"));
        while saturated.outbox.pop_owned_event().is_some() {}

        let mut stale = deletion_fixture("node-a");
        for _ in 0..BOARD_EVENT_ITEM_CAPACITY {
            stale.outbox.events.push(BoardOwnedEvent::hover(None, None).unwrap()).unwrap();
        }
        board_delete_selection(&mut stale.scene, &mut stale.outbox, &mut stale.commit);
        stale.scene.interaction_revision = stale.scene.interaction_revision.wrapping_add(1);
        assert_eq!(with_board_step_context(1, live.clone(), |context| stale.step_event_authority(context)), BoardAuthorityStep::Fault);
        assert!(stale.commit.pending_delete_operation.is_none());
        assert!(stale.scene.nodes.contains_key("node-a"));
        assert!(stale.outbox.event_terminal_faulted());

        let oversized_id = "x".repeat(BOARD_POINTER_BYTE_CAPACITY + 1);
        let mut oversized = deletion_fixture("node-a");
        let mut oversized_node = oversized.scene.nodes.remove("node-a").unwrap();
        oversized_node.id.clone_from(&oversized_id);
        oversized.scene.nodes.insert(oversized_id.clone(), oversized_node);
        oversized.focus.selection.clear();
        oversized.focus.selection.insert(oversized_id.clone());
        board_delete_selection(&mut oversized.scene, &mut oversized.outbox, &mut oversized.commit);
        assert_eq!(with_board_step_context(1, live.clone(), |context| oversized.step_event_authority(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, live.clone(), |context| oversized.step_event_authority(context)), BoardAuthorityStep::Fault);
        assert!(oversized.scene.nodes.contains_key(&oversized_id));
        assert!(oversized.commit.pending_delete_planning.is_none());
        assert!(oversized.commit.pending_delete_operation.is_none());
        assert!(oversized.outbox.event_terminal_faulted());

        let mut interrupted = deletion_fixture("node-a");
        board_delete_selection(&mut interrupted.scene, &mut interrupted.outbox, &mut interrupted.commit);
        assert!(interrupted.commit.pending_delete_planning.is_some());
        let cancel = semio_framework_job::root_cancel_token();
        assert_eq!(with_board_step_context(1, cancel.clone(), |context| interrupted.step_event_authority(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, cancel.clone(), |context| interrupted.step_event_authority(context)), BoardAuthorityStep::Pending);
        assert!(!with_board_step_context(1, cancel.clone(), |context| interrupted.close_event_authority_step(context)));
        let mut close_turns = 1usize;
        loop {
            close_turns += 1;
            assert!(close_turns <= 32, "the cancelled delete never closed");
            if with_board_step_context(1, cancel.clone(), |context| interrupted.close_event_authority_step(context)) {
                break;
            }
        }
        assert!(close_turns > 1, "the close must be cooperative, not a single-shot teardown");
        assert!(event_authority_terminal_is_empty(&interrupted.outbox, &interrupted.commit));
    }

/// 🧮️ One audited property node per turn: the hostile array is `BOARD_POINTER_ITEM_CAPACITY + 1`
/// entries, so the fault lands after that many turns — the budget follows the constant.
#[cfg(test)]
    #[test]
    fn delete_property_pre_admission_rejects_hostile_nodes_without_transfer_or_mutation() {
        let mut host = deletion_fixture("node-a");
        host.scene.nodes.get_mut("node-a").unwrap().properties.insert("hostile".into(), semio_framework_graph_manifest::PropertyValue::Array((0..=BOARD_POINTER_ITEM_CAPACITY).map(|_| semio_framework_graph_manifest::PropertyValue::Null).collect()));
        board_delete_selection(&mut host.scene, &mut host.outbox, &mut host.commit);
        let live = semio_framework_job::root_cancel_token();
        for _ in 0..BOARD_POINTER_ITEM_CAPACITY * 4 {
            if matches!(with_board_step_context(1, live.clone(), |context| host.step_event_authority(context)), BoardAuthorityStep::Fault) {
                break;
            }
        }
        assert!(host.commit.pending_delete_planning.is_none());
        assert!(host.commit.pending_delete_operation.is_none());
        assert!(host.scene.nodes.contains_key("node-a"));
        assert!(host.outbox.event_terminal_faulted());

        let mut node = host.scene.nodes.remove("node-a").unwrap();
        let Some(semio_framework_graph_manifest::PropertyValue::Array(mut values)) = node.properties.remove("hostile") else { panic!("hostile property remains retained") };
        while values.pop().is_some() {}
        drop(values);
        drop(node);
    }

#[cfg(test)]
    #[test]
    fn delete_property_derivation_is_one_node_per_turn_and_cancel_restores_exact_owner() {
        let mut host = deletion_fixture("node-a");
        host.scene.nodes.get_mut("node-a").unwrap().properties.insert("nested".into(), semio_framework_graph_manifest::PropertyValue::Array((0..32).map(|index| semio_framework_graph_manifest::PropertyValue::String(format!("value-{index}"))).collect()));
        board_delete_selection(&mut host.scene, &mut host.outbox, &mut host.commit);
        let live = semio_framework_job::root_cancel_token();
        let mut observed_audit = false;
        let mut previous_nodes = 0usize;
        for _ in 0..128 {
            let _ = with_board_step_context(1, live.clone(), |context| host.step_event_authority(context));
            if let Some(audit) = host.commit.pending_delete_planning.as_ref().and_then(|planning| planning.property_audit.as_ref()) {
                observed_audit = true;
                assert!(audit.nodes.saturating_sub(previous_nodes) <= 1);
                previous_nodes = audit.nodes;
                if audit.nodes >= 4 {
                    break;
                }
            }
        }
        assert!(observed_audit);
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        let mut turns = 0usize;
        while !with_board_step_context(1, cancel.clone(), |context| host.close_event_authority_step(context)) {
            turns += 1;
            assert!(turns <= 256);
        }
        assert!(host.scene.nodes.contains_key("node-a"));
        let Some(semio_framework_graph_manifest::PropertyValue::Array(values)) = host.scene.nodes.get("node-a").unwrap().properties.get("nested") else { panic!("cancelled property audit restored the original root") };
        assert_eq!(values.len(), 32);
        assert!(event_authority_terminal_is_empty(&host.outbox, &host.commit));
    }

#[cfg(test)]
    #[test]
    fn retained_delete_plan_preserves_legacy_fifo_and_retires_mid_audit_stale_generation() {
        let mut legacy = deletion_fixture("node-a");
        set_selection_ids_silent(&mut legacy.scene, &mut legacy.focus, &mut legacy.outbox, &["node-a".into(), "node-b".into()]);
        let legacy_plan = plan_delete_selection(&legacy.scene, &legacy.focus, &legacy.style).unwrap();
        let legacy_order: Vec<_> = legacy_plan.entries[..usize::from(legacy_plan.len)].iter().flatten().map(|entry| (entry.kind, legacy_plan.id(entry.id).to_owned())).collect();

        let mut retained = deletion_fixture("node-a");
        set_selection_ids_silent(&mut retained.scene, &mut retained.focus, &mut retained.outbox, &["node-a".into(), "node-b".into()]);
        board_delete_selection(&mut retained.scene, &mut retained.outbox, &mut retained.commit);
        let live = semio_framework_job::root_cancel_token();
        for _ in 0..512 {
            if retained.commit.pending_delete_operation.is_some() {
                break;
            }
            assert_eq!(with_board_step_context(1, live.clone(), |context| retained.step_event_authority(context)), BoardAuthorityStep::Pending);
        }
        let retained_plan = &retained.commit.pending_delete_operation.as_ref().expect("retained plan completed").plan;
        let retained_order: Vec<_> = retained_plan.entries[..usize::from(retained_plan.len)].iter().flatten().map(|entry| (entry.kind, retained_plan.id(entry.id).to_owned())).collect();
        assert_eq!(retained_order, legacy_order);

        let mut stale = deletion_fixture("node-a");
        stale.scene.nodes.get_mut("node-a").unwrap().properties.insert("nested".into(), semio_framework_graph_manifest::PropertyValue::Array(vec![semio_framework_graph_manifest::PropertyValue::String("retained".into())]));
        board_delete_selection(&mut stale.scene, &mut stale.outbox, &mut stale.commit);
        for _ in 0..128 {
            let _ = with_board_step_context(1, live.clone(), |context| stale.step_event_authority(context));
            if stale.commit.pending_delete_planning.as_ref().is_some_and(|planning| planning.property_audit.is_some()) {
                break;
            }
        }
        stale.scene.interaction_revision = stale.scene.interaction_revision.wrapping_add(1);
        for _ in 0..128 {
            if matches!(with_board_step_context(1, live.clone(), |context| stale.step_event_authority(context)), BoardAuthorityStep::Fault) {
                break;
            }
        }
        assert!(stale.scene.nodes.contains_key("node-a"));
        assert!(stale.scene.nodes.get("node-a").unwrap().properties.contains_key("nested"));
        assert!(stale.commit.pending_delete_planning.is_none());
        assert!(stale.outbox.event_terminal_faulted());
    }

#[cfg(test)]
    #[test]
    fn delete_property_key_overflow_faults_after_bounded_restore_without_mutation() {
        let mut host = deletion_fixture("node-a");
        let hostile = "k".repeat(BOARD_POINTER_BYTE_CAPACITY + 1);
        host.scene.nodes.get_mut("node-a").unwrap().properties.insert(hostile.clone(), semio_framework_graph_manifest::PropertyValue::Null);
        board_delete_selection(&mut host.scene, &mut host.outbox, &mut host.commit);
        let live = semio_framework_job::root_cancel_token();
        for _ in 0..128 {
            if matches!(with_board_step_context(1, live.clone(), |context| host.step_event_authority(context)), BoardAuthorityStep::Fault) {
                break;
            }
        }
        assert!(host.outbox.event_terminal_faulted());
        assert!(host.scene.nodes.contains_key("node-a"));
        assert!(host.scene.nodes.get("node-a").unwrap().properties.contains_key(&hostile));
        assert!(host.commit.pending_delete_planning.is_none());
    }

#[cfg(test)]
    #[test]
    fn removed_entity_retirement_witness_survives_interruption_and_releases_one_owner_per_turn() {
        let mut host = deletion_fixture("node-a");
        let node = host.scene.nodes.remove("node-a").unwrap();
        let mut retirement = BoardEntityRetirement::new(BoardRemovedEntity::Node(node));
        assert!(!retirement.step().unwrap());
        assert!(!retirement.terminal_is_empty());
        let mut turns = 1usize;
        while !retirement.step().unwrap() {
            turns += 1;
            assert!(turns <= 32);
        }
        assert!(turns > 4);
        assert!(retirement.terminal_is_empty());
    }

#[cfg(test)]
    #[test]
    fn board_world_scene_retirement_retains_exact_token_until_backing_is_released() {
        let mut host = deletion_fixture("node-a");
        let mut scene = Scene::new();
        for _ in 0..128 {
            scene.pop_layer();
        }
        let mut path = semio_framework_canvas::BezPath::new();
        path.move_to((0.0, 0.0));
        for point in 0..1600 {
            path.line_to((f64::from(point), f64::from(point)));
        }
        scene.fill(FillRule::NonZero, Affine::IDENTITY, Color::from_rgba8(0, 0, 0, 255), None, &path);
        *host.scene.world_content_cache.borrow_mut() = Some(WorldContentCache { generation: host.scene.content_scene_generation, lod: BoardDrawLod::Detail, scene, spans: Vec::new() });
        assert!(!quarantine_world_content_step(&mut host.scene, &mut host.closing));
        assert!(host.closing.opaque_scene_retirement.get().is_some());
        let mut turns = 1usize;
        while !quarantine_world_content_step(&mut host.scene, &mut host.closing) {
            turns += 1;
            assert!(turns < 4_096, "retained world scene cursor reaches exact terminal release");
        }
        assert!(turns > 1_600);
        assert!(host.scene.world_content_cache.borrow().is_none());
        assert!(host.closing.opaque_scene_retirement.get().is_none());
        assert!(!host.closing.opaque_scene_faulted());
    }

#[cfg(test)]
    #[test]
    fn board_host_nonopaque_close_is_interruptible_and_terminal_witnessed() {
        let host = deletion_fixture("node-a");
        let mut retirement = BoardHostRetirement::new(host);
        let live = semio_framework_job::root_cancel_token();
        assert!(!with_board_step_context(0, live.clone(), |context| retirement.close_step(context)));
        let mut turns = 0usize;
        while !with_board_step_context(1, live.clone(), |context| retirement.close_step(context)) {
            turns += 1;
            assert!(turns < 8_192, "fixed BoardHost close reaches an exact terminal witness");
        }
        assert!(turns > 16);
        assert!(retirement.terminal_nonopaque_is_empty());
    }

#[cfg(test)]
    /// 🧮️ The descriptor census is bounded by `BOARD_DESCRIPTOR_ITEM_CAPACITY`, not by the pointer payload
    /// credits: a hundred-placement fill of an eleven-handle kind (1 200 entities, past the 1 024
    /// pointer credits) must keep painting, while a descriptor past the descriptor cap is still refused.
    #[test]
    fn descriptor_admits_boards_past_the_pointer_credits_and_refuses_past_its_own_cap() {
        let board = |nodes: usize, handles_per_node: usize| {
            let nodes: Vec<serde_json::Value> = (0..nodes)
                .map(|index| {
                    let handles: Vec<serde_json::Value> = (0..handles_per_node).map(|handle| serde_json::json!({ "id": format!("node-{index}:v{handle}"), "handleKind": "b-l", "angle": handle as f64 * 0.5, "radius": 3.0 })).collect();
                    serde_json::json!({ "id": format!("node-{index}"), "x": index as f64 * 60.0, "y": 0.0, "shape": "circle", "radius": 24.0, "handles": handles })
                })
                .collect();
            serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": nodes, "edges": [] }).to_string()
        };
        let mut host = BoardHost::default();
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(100, 11),&mut control)}, "1 200 entities must parse past the {BOARD_POINTER_ITEM_CAPACITY} pointer credits");
        let refused = BOARD_DESCRIPTOR_ITEM_CAPACITY / 12 + 1;
        assert!(!{let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(refused, 11),&mut control)}, "a descriptor past {BOARD_DESCRIPTOR_ITEM_CAPACITY} entities must be refused");
    }

#[cfg(test)]
    /// 🎬️ The battery's own session, in one host: parse → drag a node → repaint the board a
    /// hundred-placement fill grew → delete → parse again. Every parse must be admitted — the panes go
    /// blank for exactly one step whenever one of them is refused mid-session (2026-09-17).
    #[test]
    fn a_whole_editing_session_reparses_the_board_after_every_gesture() {
        let board = |nodes: usize| {
            let rows: Vec<serde_json::Value> = (0..nodes)
                .map(|index| {
                    let handles: Vec<serde_json::Value> = (0..11).map(|handle| serde_json::json!({ "id": format!("node-{index}:v{handle}"), "handleKind": "b-l", "angle": handle as f64 * 0.5, "radius": 3.0 })).collect();
                    serde_json::json!({ "id": format!("node-{index}"), "x": index as f64 * 60.0, "y": 0.0, "shape": "circle", "radius": 24.0, "handles": handles })
                })
                .collect();
            let edges: Vec<serde_json::Value> = (0..nodes.saturating_sub(1)).map(|index| serde_json::json!({ "id": format!("edge-{index}"), "source": format!("node-{index}:v0"), "target": format!("node-{}:v1", index + 1) })).collect();
            serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": rows, "edges": edges }).to_string()
        };
        let mut host = BoardHost::default();
        host.scene.set_size(800, 600, 1.0);
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(4),&mut control)}, "the opening parse must be admitted");
        host.focus.interaction = Interaction::DragNodes { primary_id: "node-0".into(), offset: Vec2::ZERO, start_positions: [("node-0".to_string(), (0.0, 0.0))].into_iter().collect(), proximity_pair: None, gesture: GestureStage::default(), delta: Vec2::ZERO };
        let plan = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Up, x: 10.0, y: 5.0, shift: false, ctrl_or_meta: false, alt: false }).expect("finish drag plan");
        begin_pointer_commit(&mut host.scene, &mut host.commit, plan).expect("retained drag commit");
        drive_pointer_commit(&mut host);
        let mut publication = host.commit.take_pointer_publication().expect("drag publication");
        for _ in 0..4096 {
            if publication.close_step() {
                break;
            }
        }
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(4),&mut control)}, "the re-parse after a drag must be admitted");
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(104),&mut control)}, "the re-parse after a hundred-placement fill must be admitted");
        set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-3".to_string()]);
        board_delete_selection(&mut host.scene, &mut host.outbox, &mut host.commit);
        let live = semio_framework_job::root_cancel_token();
        let mut turns = 0usize;
        while host.commit.pending_delete_planning.is_some() || host.commit.pending_delete_operation.is_some() {
            turns += 1;
            assert!(turns <= 1 << 18, "the delete never reached its terminal step");
            let step = with_board_step_context(1, live.clone(), |context| host.step_event_authority(context));
            assert_ne!(step, BoardAuthorityStep::Fault, "the delete must not fault on a fill-sized board");
            let _ = host.outbox.pop_owned_event();
        }
        assert!(!host.scene.nodes.contains_key("node-3"), "the delete must remove its node");
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(103),&mut control)}, "the re-parse after a delete must be admitted");
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(103),&mut control)}, "a session may re-parse its board any number of times");
    }

#[cfg(test)]
    /// 🎥️ A document whose camera is SESSION state carries no `camera` key at all (a domain whose
    /// `setCamera` is a View-kind verb). Requiring one refused every shipped 2d example outright —
    /// `load_board_snapshot_json` returned false before it read a single node and all three panes stayed blank.
    /// The parse now keeps the camera the host is looking through and paints the document.
    #[test]
    fn a_fixture_without_a_camera_parses_and_keeps_the_session_camera() {
        let board = |camera: Option<serde_json::Value>| {
            let mut fixture = serde_json::json!({
                "schema": "board.ports.directed.v1",
                "nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }],
                "edges": []
            });
            if let Some(camera) = camera {
                fixture["camera"] = camera;
            }
            fixture.to_string()
        };
        let mut host = BoardHost::default();
        host.scene.set_size(800, 600, 1.0);
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(Some(serde_json::json!({ "x": 12.0, "y": -3.0, "zoom": 2.0 }))),&mut control)}, "a fixture that names its camera still parses");
        let framed = (host.scene.camera.x, host.scene.camera.y, host.scene.camera.zoom);
        assert_eq!(framed, (12.0, -3.0, 2.0), "a named camera is adopted");
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(None),&mut control)}, "a document with no camera key must parse, not refuse");
        assert!(host.scene.nodes.contains_key("node-a"), "the cameraless document paints its nodes");
        assert_eq!((host.scene.camera.x, host.scene.camera.y, host.scene.camera.zoom), framed, "the session keeps the camera it was looking through");
    }

#[cfg(test)]
    /// 🧱️ A REFUSED parse leaves the board exactly as it was. `load_board_snapshot_json` used to clear the
    /// scene before validating, so a malformed row (or a descriptor past its cap) emptied every pane and
    /// the refusal and "the board went blank" were the same event (2026-09-17 battery).
    #[test]
    fn a_refused_fixture_parse_leaves_the_painted_board_untouched() {
        let good = serde_json::json!({
            "schema": "board.ports.directed.v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }],
            "edges": []
        })
        .to_string();
        let mut host = BoardHost::default();
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&good,&mut control)}, "the opening parse must be admitted");
        for refused in [
            serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-z", "x": 1.0, "y": 1.0, "shape": "circle" }], "edges": [] }),
            serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-z", "x": 1.0, "y": 1.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-z:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0, "color": "not-a-color" }] }], "edges": [] }),
            serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-z", "x": 1.0, "y": 1.0, "shape": "rectangle", "width": 0.0, "height": 4.0 }], "edges": [] }),
        ] {
            assert!(!{let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&refused.to_string(),&mut control)}, "this fixture must be refused: {refused}");
            assert!(host.scene.nodes.contains_key("node-a"), "a refused parse must not empty the board: {refused}");
            assert!(!host.scene.nodes.contains_key("node-z"), "a refused parse must not half-commit its own rows: {refused}");
        }
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&good,&mut control)}, "the next real parse still lands");
    }

#[cfg(test)]
    /// 🚚️ A fixture parse is the document's echo, not authoring: it emits no `edgeCreate` events, and an
    /// edged document re-parses in the SAME session any number of times without touching the event
    /// credits — undrained descriptor edge events used to refuse the second parse of Nakagin (179 edges
    /// twice > `BOARD_EVENT_ITEM_CAPACITY`) and to echo every edge back to the plugin as a creation.
    #[test]
    fn fixture_parse_announces_no_edges_and_reparses_in_one_session() {
        let edges = BOARD_EVENT_ITEM_CAPACITY / 2 + 8;
        let nodes: Vec<serde_json::Value> = (0..=edges)
            .map(|index| serde_json::json!({ "id": format!("node-{index}"), "x": index as f64 * 60.0, "y": 0.0, "shape": "circle", "radius": 24.0, "handles": [{ "id": format!("node-{index}:a"), "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }, { "id": format!("node-{index}:b"), "handleKind": "b-l", "angle": 3.0, "radius": 3.0 }] }))
            .collect();
        let edge_rows: Vec<serde_json::Value> = (0..edges).map(|index| serde_json::json!({ "id": format!("edge-{index}"), "source": format!("node-{index}:b"), "target": format!("node-{}:a", index + 1) })).collect();
        let board = serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": nodes, "edges": edge_rows }).to_string();
        let mut host = BoardHost::default();
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board,&mut control)}, "the edged board must parse into a fresh session");
        let events = host.outbox.drain_events_json();
        assert!(!events.contains("edgeCreate"), "a fixture parse must not announce the document's edges as creations: {events}");
        for round in 0..4 {
            assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board,&mut control)}, "re-parse #{round} of the same edged board must parse in the same session without draining");
        }
        assert!(host.sync_descriptor(&SceneDescriptor::default()).is_ok(), "an authoring sync still runs after the parses");
    }

#[cfg(test)]
//#region 🕹️TransformGumball
/// 🕹️ Two free nodes either side of the origin plus one locked node above it, on an 800×600 pane at
/// zoom 1 so `world_to_screen` is a plain centre offset.
fn transform_gumball_host() -> BoardHost {
    let node = |id: &str, x: f64, locked: bool| {
        serde_json::json!({ "id": id, "x": x, "y": 0.0, "shape": "circle", "radius": 10.0, "locked": locked, "handles": [{ "id": format!("{id}:v0"), "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] })
    };
    let fixture = serde_json::json!({
        "schema": "board.ports.directed.v1",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [node("node-a", -40.0, false), node("node-b", 40.0, false), node("node-locked", 0.0, true)],
        "edges": []
    })
    .to_string();
    let mut host = BoardHost::default();
    host.scene.set_size(800, 600, 1.0);
    set_camera_silent(&mut host.scene, &mut host.outbox, 0.0, 0.0, 1.0);
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&fixture,&mut control)}, "the gumball fixture must parse");
    host
}

#[cfg(test)]
/// 🎯️ Arms the ring and returns the screen point `degrees` around it.
fn transform_ring_screen_at(host: &BoardHost, degrees: f64) -> Point {
    let (pivot, radius) = host.transform_gumball_geometry().expect("the rotate ring must be armed");
    let radians = degrees.to_radians();
    host.scene.world_to_screen(Point::new(pivot.x + radius * radians.cos(), pivot.y + radius * radians.sin()))
}

#[cfg(test)]
fn board_event_names(json: &str) -> Vec<String> {
    serde_json::from_str::<Vec<serde_json::Value>>(json).expect("events parse").into_iter().filter_map(|row| row.get("name").and_then(serde_json::Value::as_str).map(str::to_string)).collect()
}

#[cfg(test)]
/// 🔄️ The whole rotate gesture is ONE gesture record: the drag streams only TRANSIENT `transformPreview`
/// frames (the peer-pane mirror's food, dropped before dispatch) and the release publishes exactly one
/// `gesture` row of kind `rotate` carrying the turned ids, the pivot and the angle. Streaming `nodeMove`
/// rows here would spend one of the store's 64 applied edits per frame.
#[test]
fn a_rotate_ring_drag_commits_exactly_one_rotate_gesture_record() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    let _ = host.outbox.drain_events_json();
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    assert!(host.gestures.transform_drag.is_some(), "a press on the ring must begin the gumball gesture");
    let quarter = transform_ring_screen_at(&host, 90.0);
    host.pointer_move_screen(quarter.x, quarter.y, false, false, false);
    let during = board_event_names(&host.outbox.drain_events_json());
    assert_eq!(during, vec!["transformPreview".to_string()], "a drag frame announces only its transient preview");
    host.pointer_up_screen(quarter.x, quarter.y, false, false, false);
    assert!(host.gestures.transform_drag.is_none(), "the release must end the gesture");
    let released = host.outbox.drain_events_json();
    assert_eq!(board_event_names(&released), vec!["gesture".to_string()], "the release is one row: {released}");
    let row: Vec<serde_json::Value> = serde_json::from_str(&released).expect("events parse");
    let payload = row[0].get("payload").expect("payload");
    assert_eq!(payload["kind"], "rotate");
    assert!(payload["gestureId"].as_str().is_some_and(|id| id.starts_with("gesture-")), "the record names its gesture: {payload}");
    let angle = payload["angle"].as_f64().expect("angle");
    assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-6, "the record carries the absolute quarter turn, got {angle}");
    assert_eq!(payload["targets"], serde_json::json!(["node-a", "node-b"]), "the record names exactly the rotated members");
    assert!(payload["pivotX"].as_f64().expect("pivot x").abs() < 1e-9 && payload["pivotY"].as_f64().expect("pivot y").abs() < 1e-9, "the pivot is the selection centroid");
    assert_eq!(payload["proximity"], serde_json::json!([]));
}

#[cfg(test)]
/// 🎞️ However many frames a rotate paints — sixty-four here — the release is ONE record, so the guest yields ONE
/// transaction for the whole ring drag.
#[test]
fn a_streamed_rotate_is_one_record_however_many_frames_it_painted() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    let _ = host.outbox.drain_events_json();
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let mut frames = 0usize;
    for step in 1..=64 {
        let point = transform_ring_screen_at(&host, f64::from(step));
        host.pointer_move_screen(point.x, point.y, false, false, false);
        frames += board_event_names(&host.outbox.drain_events_json()).len();
    }
    let last = transform_ring_screen_at(&host, 64.0);
    host.pointer_up_screen(last.x, last.y, false, false, false);
    assert_eq!(frames, 64, "every frame previews once");
    assert_eq!(board_event_names(&host.outbox.drain_events_json()), vec!["gesture".to_string()], "and the whole gesture commits once");
}

#[cfg(test)]
/// 🎛️ The rotate ring and the area brush are DIRECT lanes: the retained plan refuses their moves and releases, so a host
/// (wgpu) drives them through `pointer_move_screen`/`pointer_up_screen` as React does, while a leave still plans; the ring's
/// release closes the lane.
#[test]
fn direct_pointer_lanes_refuse_the_retained_plan() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    let _ = host.outbox.drain_events_json();
    let intent = |phase: BoardPointerPhase, at: Point| BoardPointerIntent { phase, x: at.x, y: at.y, shift: false, ctrl_or_meta: false, alt: false };
    let grab = transform_ring_screen_at(&host, 0.0);
    assert!(!pointer_lane_is_direct(&host.focus, &host.gestures), "an idle select board plans every pointer");
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    assert!(pointer_lane_is_direct(&host.focus, &host.gestures), "a press on the ring opens a direct lane");
    let quarter = transform_ring_screen_at(&host, 90.0);
    for phase in [BoardPointerPhase::Move, BoardPointerPhase::Up] {
        assert_eq!(host.plan_pointer(intent(phase, quarter)).err(), Some(BoardPointerPlanFault::Unsupported), "{phase:?} on the ring is direct");
    }
    assert!(host.plan_pointer(intent(BoardPointerPhase::Leave, quarter)).is_ok(), "a leave still plans");
    host.pointer_up_screen(quarter.x, quarter.y, false, false, false);
    assert!(!pointer_lane_is_direct(&host.focus, &host.gestures), "the release closes the lane");
    host.set_active_utility("areaBrush");
    assert!(pointer_lane_is_direct(&host.focus, &host.gestures), "the area brush is direct from the first move");
    assert_eq!(host.plan_pointer(intent(BoardPointerPhase::Move, grab)).err(), Some(BoardPointerPlanFault::Unsupported));
}

#[cfg(test)]
/// 👁️ The live preview turns node CENTRES and handle ANGLES together, so edges keep their geometry
/// through the drag exactly as the guest reducer will recompute them on commit.
#[test]
fn the_rotate_preview_turns_positions_and_handle_angles_together() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    let angle_before = host.scene.handles.get("node-a:v0").expect("handle").angle;
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let quarter = transform_ring_screen_at(&host, 90.0);
    host.pointer_move_screen(quarter.x, quarter.y, false, false, false);
    let a = host.scene.nodes.get("node-a").expect("node-a");
    assert!(a.x.abs() < 1e-6 && (a.y + 40.0).abs() < 1e-6, "(-40,0) turned a quarter is (0,-40), got ({}, {})", a.x, a.y);
    let b = host.scene.nodes.get("node-b").expect("node-b");
    assert!(b.x.abs() < 1e-6 && (b.y - 40.0).abs() < 1e-6, "(40,0) turned a quarter is (0,40), got ({}, {})", b.x, b.y);
    let angle_after = host.scene.handles.get("node-a:v0").expect("handle").angle;
    assert!((angle_after - angle_before - std::f64::consts::FRAC_PI_2).abs() < 1e-6, "every handle angle turns with its node, got {angle_before} -> {angle_after}");
    let half = transform_ring_screen_at(&host, 0.0);
    host.pointer_move_screen(half.x, half.y, false, false, false);
    let a = host.scene.nodes.get("node-a").expect("node-a");
    assert!((a.x + 40.0).abs() < 1e-6 && a.y.abs() < 1e-6, "the preview re-derives from the grab snapshot, it never accumulates: ({}, {})", a.x, a.y);
}

#[cfg(test)]
/// 🧲️ The grid-snap modifier quantizes the live rotation AND the committed delta to the same step.
#[test]
fn the_rotate_ring_snaps_under_the_grid_snap_modifier() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    host.style.set_grid_snap_enabled(true);
    let _ = host.outbox.drain_events_json();
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let nudged = transform_ring_screen_at(&host, 20.0);
    host.pointer_move_screen(nudged.x, nudged.y, false, false, false);
    let live = host.gestures.transform_drag.as_ref().expect("the gesture is live").radians;
    assert!((live - 15.0_f64.to_radians()).abs() < 1e-6, "20° snaps to the 15° step, got {}", live.to_degrees());
    host.pointer_up_screen(nudged.x, nudged.y, false, false, false);
    let released = host.outbox.drain_events_json();
    let rows: Vec<serde_json::Value> = serde_json::from_str(&released).expect("events parse");
    let radians = rows.iter().find(|row| row.get("name").and_then(serde_json::Value::as_str) == Some("gesture")).and_then(|row| row.get("payload")).and_then(|payload| payload.get("angle")).and_then(serde_json::Value::as_f64).expect("rotate record angle");
    assert!((radians - 15.0_f64.to_radians()).abs() < 1e-6, "the commit carries the snapped delta, got {}", radians.to_degrees());
}

#[cfg(test)]
/// 🔒️ A locked member still counts toward the pivot (so the preview and the guest reducer agree on the
/// centroid) but never moves and never appears in the commit's id list.
#[test]
fn locked_members_hold_the_pivot_but_never_turn() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into(), "node-locked".into()]);
    let _ = host.outbox.drain_events_json();
    let locked_before = (host.scene.nodes.get("node-locked").expect("locked node").x, host.scene.nodes.get("node-locked").expect("locked node").y);
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let quarter = transform_ring_screen_at(&host, 90.0);
    host.pointer_move_screen(quarter.x, quarter.y, false, false, false);
    let locked_after = (host.scene.nodes.get("node-locked").expect("locked node").x, host.scene.nodes.get("node-locked").expect("locked node").y);
    assert_eq!(locked_before, locked_after, "a locked node must not move with the ring");
    host.pointer_up_screen(quarter.x, quarter.y, false, false, false);
    let released = host.outbox.drain_events_json();
    let rows: Vec<serde_json::Value> = serde_json::from_str(&released).expect("events parse");
    let payload = rows.iter().find(|row| row.get("name").and_then(serde_json::Value::as_str) == Some("gesture")).and_then(|row| row.get("payload")).expect("rotate record payload");
    assert_eq!(payload["targets"], serde_json::json!(["node-a", "node-b"]), "the locked member stays out of the record");
    let pivot_y = payload["pivotY"].as_f64().expect("pivot y");
    assert!(pivot_y.abs() < 1e-9, "all three centres average to y=0, so the locked member still holds the pivot, got {pivot_y}");
}

#[cfg(test)]
/// 🎯️ Hit-test priority: the ring outranks whatever node, handle or edge happens to sit under it. A
/// press on the band starts the rotation and leaves the selection exactly as it was.
#[test]
fn the_rotate_ring_outranks_the_nodes_and_handles_under_it() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    let (pivot, radius) = host.transform_gumball_geometry().expect("the ring is armed");
    let under_ring = serde_json::json!({
        "schema": "board.ports.directed.v1",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [
            { "id": "node-a", "x": -40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },
            { "id": "node-b", "x": 40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-b:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },
            { "id": "node-under", "x": pivot.x + radius, "y": pivot.y, "shape": "circle", "radius": 24.0, "handles": [{ "id": "node-under:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }
        ],
        "edges": []
    })
    .to_string();
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&under_ring,&mut control)}, "the overlapping board must parse");
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    let _ = host.outbox.drain_events_json();
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    assert!(host.gestures.transform_drag.is_some(), "the ring wins the press");
    assert!(matches!(host.focus.interaction, Interaction::None), "no node drag may start under the ring");
    assert_eq!(host.focus.selection.iter().cloned().collect::<Vec<_>>(), vec!["node-a".to_string(), "node-b".to_string()], "grabbing the ring must not re-pick the node beneath it");
    let names = board_event_names(&host.outbox.drain_events_json());
    assert!(!names.iter().any(|name| name == "select"), "and it announces no selection change: {names:?}");
}

#[cfg(test)]
/// 🧾️ A whole gesture leaves the event terminal exactly as it found it: every preview frame is either
/// published or dropped outright, never left holding a claim, and nothing faults.
#[test]
fn a_rotate_gesture_leaves_the_event_credits_untouched() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    let _ = host.outbox.drain_events_json();
    assert_eq!((host.outbox.events.claimed_items, host.outbox.events.claimed_bytes), (0, 0), "the queue starts unclaimed");
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    for step in 1..=64 {
        let point = transform_ring_screen_at(&host, f64::from(step));
        host.pointer_move_screen(point.x, point.y, false, false, false);
    }
    let last = transform_ring_screen_at(&host, 64.0);
    host.pointer_up_screen(last.x, last.y, false, false, false);
    assert_eq!((host.outbox.events.claimed_items, host.outbox.events.claimed_bytes), (0, 0), "no frame may strand a claim");
    assert!(!host.outbox.event_terminal_faulted(), "a fast drag must never fault the event terminal");
    let _ = host.outbox.drain_events_json();
    assert!(host.outbox.events.terminal_is_empty(), "the drained queue is terminal-empty");
}

#[cfg(test)]
/// ↩️ Escape abandons the ring and restores the exact pre-gesture geometry without touching the document.
#[test]
fn escape_cancels_the_rotate_ring_and_restores_the_geometry() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    let _ = host.outbox.drain_events_json();
    let before = (host.scene.nodes.get("node-a").expect("node-a").x, host.scene.nodes.get("node-a").expect("node-a").y, host.scene.handles.get("node-a:v0").expect("handle").angle);
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let quarter = transform_ring_screen_at(&host, 90.0);
    host.pointer_move_screen(quarter.x, quarter.y, false, false, false);
    assert!(host.cancel_area_select(), "escape must claim the live ring gesture");
    assert!(host.gestures.transform_drag.is_none(), "and end it");
    let after = (host.scene.nodes.get("node-a").expect("node-a").x, host.scene.nodes.get("node-a").expect("node-a").y, host.scene.handles.get("node-a:v0").expect("handle").angle);
    assert_eq!(before, after, "a cancel restores position AND handle angle");
    let names = board_event_names(&host.outbox.drain_events_json());
    assert!(!names.iter().any(|name| name == "gesture"), "a cancelled gesture commits nothing: {names:?}");
}

#[cfg(test)]
/// 🕹️ The ring is drawn and armed ONLY while the select utility composes `rotate` and the selection
/// holds a node — the brush utility and a rotate-off gumball leave the press to the ordinary hit test.
#[test]
fn the_rotate_ring_is_armed_only_when_the_flags_allow_it() {
    let mut host = transform_gumball_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into(), "node-b".into()]);
    let grab = transform_ring_screen_at(&host, 0.0);
    set_transform_flags(&mut host.scene, &mut host.gestures, true, false);
    assert!(host.transform_gumball_geometry().is_none(), "rotate off disarms the ring");
    assert!(host.transform_gumball_json().contains("\"ringVisible\":false"), "and the vitals say so: {}", host.transform_gumball_json());
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    assert!(host.gestures.transform_drag.is_none(), "a disarmed ring must not swallow the press");
    host.pointer_up_screen(grab.x, grab.y, false, false, false);
    set_transform_flags(&mut host.scene, &mut host.gestures, true, true);
    host.set_active_utility("brush");
    assert!(host.transform_gumball_geometry().is_none(), "the brush utility owns the pointer, not the gumball");
    host.set_active_utility("select");
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &[]);
    assert!(host.transform_gumball_geometry().is_none(), "an empty selection has nothing to turn");
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".into()]);
    assert!(host.transform_gumball_geometry().is_some(), "one node is enough to arm the ring");
    assert!(host.transform_gumball_json().contains("\"rotate\":true"), "the vitals name the composed handles: {}", host.transform_gumball_json());
}
//#endregion 🕹️TransformGumball

//#region 🩺️HandleVitals
#[cfg(test)]
/// 🩺️ The DOM's only channel that NAMES a handle. `data-board-positions-json` carries nodes only, so
/// a headless caller could never aim at the handle `connect`/`openHandleSuggestions`/`createEdge` all
/// take. The rows are the camera's own viewport — a handle off-screen cannot be clicked — so a pan
/// away publishes nothing while `total` keeps naming the whole document.
#[test]
fn handle_vitals_name_every_on_screen_handle_and_nothing_else() {
    let mut host = transform_gumball_host();
    let json = handle_positions_json(&host.scene, &host.style);
    assert!(json.contains("\"total\":3"), "the document's whole handle count is always named: {json}");
    assert!(json.contains("\"onScreen\":3") && json.contains("\"published\":3") && json.contains("\"capped\":false"), "three handles fit the 800×600 viewport uncapped: {json}");
    for id in ["node-a:v0", "node-b:v0", "node-locked:v0"] {
        assert!(json.contains(&format!("[\"{id}\",")), "{id} must be named with its world position: {json}");
    }
    assert_eq!(json.matches("\"b-l\",true").count(), 3, "an edgeless document leaves every handle open: {json}");
    let row = handle_positions_json(&host.scene, &host.style);
    let node_a = host.scene.nodes.get("node-a").expect("node-a");
    assert!(row.contains(&format!("[\"node-a:v0\",{},", node_a.x + 10.0)), "the position is the handle's own world point, not its node's: {row}");
    set_camera_silent(&mut host.scene, &mut host.outbox, 100_000.0, 100_000.0, 1.0);
    let away = handle_positions_json(&host.scene, &host.style);
    assert!(away.contains("\"total\":3") && away.contains("\"onScreen\":0") && away.contains("\"rows\":[]"), "a camera that left the document publishes no aimable handle: {away}");
}
//#endregion 🩺️HandleVitals

//#region 🎯️TargetRegions
#[cfg(test)]
/// 🎯️ One node at the origin sitting INSIDE a free region, plus a locked and a hidden region well
/// clear of it. 800×600 at zoom 1 and camera (0,0), so `world_to_screen` is a plain centre offset.
fn region_host() -> BoardHost {
    let fixture = serde_json::json!({
        "schema": "board.ports.directed.v1",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 20.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }],
        "edges": [],
        "targetRegions": [
            { "id": "region-a", "x": -60.0, "y": -60.0, "width": 120.0, "height": 120.0 },
            { "id": "region-locked", "x": 200.0, "y": -40.0, "width": 80.0, "height": 80.0, "locked": true },
            { "id": "region-hidden", "x": -300.0, "y": -40.0, "width": 80.0, "height": 80.0, "hidden": true }
        ]
    })
    .to_string();
    let mut host = BoardHost::default();
    host.scene.set_size(800, 600, 1.0);
    set_camera_silent(&mut host.scene, &mut host.outbox, 0.0, 0.0, 1.0);
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&fixture,&mut control)}, "the region fixture must parse");
    host
}

#[cfg(test)]
fn region_press(host: &mut BoardHost, x: f64, y: f64) {
    let screen = host.scene.world_to_screen(Point::new(x, y));
    host.pointer_down_screen(screen.x, screen.y, 0, false, false);
}

#[cfg(test)]
fn region_move_to(host: &mut BoardHost, x: f64, y: f64) {
    let screen = host.scene.world_to_screen(Point::new(x, y));
    host.pointer_move_screen(screen.x, screen.y, false, false, false);
}

#[cfg(test)]
fn region_release_at(host: &mut BoardHost, x: f64, y: f64) {
    let screen = host.scene.world_to_screen(Point::new(x, y));
    host.pointer_up_screen(screen.x, screen.y, false, false, false);
}

#[cfg(test)]
fn region_event_payloads(json: &str, name: &str) -> Vec<serde_json::Value> {
    serde_json::from_str::<Vec<serde_json::Value>>(json)
        .expect("events parse")
        .into_iter()
        .filter(|row| row.get("name").and_then(serde_json::Value::as_str) == Some(name))
        .filter_map(|row| row.get("payload").cloned())
        .collect()
}

#[cfg(test)]
/// 🎯️ The fixture's `targetRegions` reach the engine, and they are the BACKDROP: a press on the node
/// that sits inside a region starts a node drag, never a region drag. The region is only grabbed once
/// every node, handle, edge and the rotate ring have missed.
#[test]
fn target_regions_are_ingested_and_hit_tested_after_every_entity() {
    let mut host = region_host();
    assert_eq!(host.scene.regions.len(), 3, "all three document rows reach the engine");
    let published = target_regions_json(&host.scene, &host.gestures);
    assert!(published.contains("\"id\":\"region-a\"") && published.contains("\"x\":-60") && published.contains("\"width\":120"), "the probe vital carries normalized bounds: {published}");
    region_press(&mut host, 0.0, 0.0);
    assert!(host.gestures.region_drag.is_none(), "the node under the region wins the press");
    assert!(matches!(host.focus.interaction, Interaction::DragNodes { .. }), "and it starts an ordinary node drag");
    let mut host = region_host();
    region_press(&mut host, -50.0, 0.0);
    assert!(host.gestures.region_drag.is_some(), "a press inside the region but clear of the node grabs the region");
    assert_eq!(host.focus.selection.iter().cloned().collect::<Vec<_>>(), vec!["region-a".to_string()], "and selects it at `region` granularity");
    assert!(matches!(host.focus.interaction, Interaction::None), "no marquee may start under a grabbed region");
}

#[cfg(test)]
/// 🖍️ A click-drag with the area brush armed commits exactly ONE `regionCreate`, on release — the
/// drag itself publishes nothing, so painting a region is one applied edit of the store's 64.
#[test]
fn an_area_brush_drag_commits_exactly_one_region_create() {
    let mut host = region_host();
    host.set_active_utility("areaBrush");
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, -200.0, -150.0);
    assert!(host.gestures.region_paint.is_some(), "the press anchors the brush rectangle");
    region_move_to(&mut host, -140.0, -70.0);
    assert_eq!(board_event_names(&host.outbox.drain_events_json()), Vec::<String>::new(), "a paint frame announces nothing");
    region_release_at(&mut host, -140.0, -70.0);
    assert!(host.gestures.region_paint.is_none(), "the release ends the gesture");
    let released = host.outbox.drain_events_json();
    assert_eq!(board_event_names(&released), vec!["regionCreate".to_string()], "one row: {released}");
    let payload = region_event_payloads(&released, "regionCreate").remove(0);
    assert_eq!(payload.get("x").and_then(serde_json::Value::as_f64), Some(-200.0), "the minimum corner is the anchor: {payload}");
    assert_eq!(payload.get("y").and_then(serde_json::Value::as_f64), Some(-150.0));
    assert_eq!(payload.get("width").and_then(serde_json::Value::as_f64), Some(60.0), "the extent is the drag: {payload}");
    assert_eq!(payload.get("height").and_then(serde_json::Value::as_f64), Some(80.0));
}

#[cfg(test)]
/// 🖍️ A CLICK that never left its anchor paints the utility's configured extent instead of a
/// zero-area sliver; a rectangle that stays under the extent floor commits nothing at all.
#[test]
fn an_area_brush_click_paints_the_configured_extent_and_a_collapsed_one_paints_nothing() {
    let mut host = region_host();
    host.set_active_utility("areaBrush");
    host.gestures.set_area_brush_extent(30.0, 20.0);
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, 100.0, 100.0);
    region_release_at(&mut host, 100.0, 100.0);
    let payload = region_event_payloads(&host.outbox.drain_events_json(), "regionCreate").remove(0);
    assert_eq!(payload.get("width").and_then(serde_json::Value::as_f64), Some(30.0), "a click paints the brush extent: {payload}");
    assert_eq!(payload.get("height").and_then(serde_json::Value::as_f64), Some(20.0));
    host.gestures.set_area_brush_extent(f64::NAN, -5.0);
    assert_eq!(host.gestures.area_brush_extent(), (30.0, 20.0), "a non-finite or non-positive axis never collapses the brush");
    region_press(&mut host, 0.0, 200.0);
    region_move_to(&mut host, 0.5, 200.5);
    region_release_at(&mut host, 0.5, 200.5);
    assert_eq!(board_event_names(&host.outbox.drain_events_json()), Vec::<String>::new(), "a rectangle under the extent floor commits nothing");
}

#[cfg(test)]
/// 🚚️ A body drag is ONE `drag` gesture record by the snapped offset, riding one batch with the `select` its
/// press staged; a grip drag is ONE `regionResize` carrying corner AND extent, because a west/north grip moves
/// both. A release that moved nothing publishes only the selection.
#[test]
fn region_body_and_grip_drags_commit_exactly_one_event_each() {
    let mut host = region_host();
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, -50.0, 0.0);
    assert_eq!(board_event_names(&host.outbox.drain_events_json()), Vec::<String>::new(), "the press stages its selection and publishes nothing yet");
    region_move_to(&mut host, -30.0, 10.0);
    assert_eq!(board_event_names(&host.outbox.drain_events_json()), Vec::<String>::new(), "a region drag frame announces nothing");
    region_release_at(&mut host, -30.0, 10.0);
    let moved = host.outbox.drain_events_json();
    assert_eq!(board_event_names(&moved), vec!["select".to_string(), "gesture".to_string()], "one batch: {moved}");
    let select = region_event_payloads(&moved, "select").remove(0);
    let payload = region_event_payloads(&moved, "gesture").remove(0);
    assert_eq!(select["ids"], serde_json::json!(["region-a"]));
    assert_eq!(select["gestureId"], payload["gestureId"], "the select is tagged with its gesture: {moved}");
    assert_eq!((payload["kind"].as_str(), payload["dx"].as_f64(), payload["dy"].as_f64()), (Some("drag"), Some(20.0), Some(10.0)), "the body drag translated by (+20,+10): {payload}");
    assert_eq!(payload["targets"], serde_json::json!(["region-a"]));

    let mut host = region_host();
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, 60.0, 60.0);
    assert!(host.gestures.region_drag.as_ref().is_some_and(|drag| drag.grip == RegionGrip::SouthEast), "the bottom-right corner is a resize grip");
    region_move_to(&mut host, 80.0, 80.0);
    region_release_at(&mut host, 80.0, 80.0);
    let resized = host.outbox.drain_events_json();
    assert_eq!(board_event_names(&resized), vec!["select".to_string(), "regionResize".to_string()], "one batch: {resized}");
    assert!(region_event_payloads(&resized, "select").remove(0).get("gestureId").is_none(), "a resize carries no gesture record, so its select is untagged");
    let payload = region_event_payloads(&resized, "regionResize").remove(0);
    assert_eq!(payload.get("x").and_then(serde_json::Value::as_f64), Some(-60.0), "the far corner never moved: {payload}");
    assert_eq!(payload.get("width").and_then(serde_json::Value::as_f64), Some(140.0), "and the extent grew by the drag: {payload}");
    assert_eq!(payload.get("height").and_then(serde_json::Value::as_f64), Some(140.0));

    let mut host = region_host();
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, -50.0, 0.0);
    region_release_at(&mut host, -50.0, 0.0);
    assert_eq!(board_event_names(&host.outbox.drain_events_json()), vec!["select".to_string()], "a release that moved nothing publishes only the selection");

    let mut host = region_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["region-locked".to_string()]);
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, -50.0, 0.0);
    region_move_to(&mut host, -30.0, 10.0);
    assert!(host.cancel_area_select(), "escape claims the live region drag");
    assert_eq!(host.focus.selection.iter().cloned().collect::<Vec<_>>(), vec!["region-locked".to_string()], "a cancel restores the selection the press replaced");
    assert_eq!(board_event_names(&host.outbox.drain_events_json()), Vec::<String>::new(), "and publishes nothing");
}

#[cfg(test)]
/// 🧲️ Region gestures quantize to the same visible grid step the node drag does, and only while the
/// grid-snap modifier is on.
#[test]
fn region_gestures_snap_only_under_the_grid_snap_modifier() {
    let mut host = region_host();
    host.style.set_grid_snap_enabled(true);
    let step = region_snap_step(&host.scene, &host.style).expect("the normal LOD offers a snap step");
    host.set_active_utility("areaBrush");
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, -203.0, -147.0);
    region_move_to(&mut host, -100.0, -20.0);
    region_release_at(&mut host, -100.0, -20.0);
    let payload = region_event_payloads(&host.outbox.drain_events_json(), "regionCreate").remove(0);
    let x = payload.get("x").and_then(serde_json::Value::as_f64).expect("x");
    let y = payload.get("y").and_then(serde_json::Value::as_f64).expect("y");
    assert!((x / step - (x / step).round()).abs() < 1e-9 && (y / step - (y / step).round()).abs() < 1e-9, "a snapped paint lands on the grid, got ({x}, {y}) against step {step}");

    let mut host = region_host();
    host.set_active_utility("areaBrush");
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, -203.0, -147.0);
    region_move_to(&mut host, -100.0, -20.0);
    region_release_at(&mut host, -100.0, -20.0);
    let payload = region_event_payloads(&host.outbox.drain_events_json(), "regionCreate").remove(0);
    assert_eq!(payload.get("x").and_then(serde_json::Value::as_f64), Some(-203.0), "without the modifier the exact pointer rectangle is committed: {payload}");
}

#[cfg(test)]
/// 🔏️ A locked region still paints and still selects — it simply refuses every drag, so neither a `drag`
/// gesture record nor a `regionResize` can ever name it.
#[test]
fn a_locked_region_refuses_every_drag() {
    let mut host = region_host();
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, 240.0, 0.0);
    assert!(host.gestures.region_drag.is_none(), "a locked region is never grabbed");
    assert_eq!(host.focus.selection.iter().cloned().collect::<Vec<_>>(), vec!["region-locked".to_string()], "but it still selects, so the inspector can unlock it");
    let _ = host.outbox.drain_events_json();
    region_move_to(&mut host, 300.0, 60.0);
    region_release_at(&mut host, 300.0, 60.0);
    let names = board_event_names(&host.outbox.drain_events_json());
    assert!(!names.iter().any(|name| name.starts_with("region")), "a locked region publishes no geometry row: {names:?}");
    let region = host.scene.regions.get("region-locked").expect("region-locked");
    assert!((region.x - 200.0).abs() < 1e-9 && (region.y + 40.0).abs() < 1e-9, "and its rectangle never moved");
}

#[cfg(test)]
/// 🙈️ A hidden region is not paint, so it is not a pick target either: a press inside its rectangle
/// falls straight through to the background marquee.
#[test]
fn a_hidden_region_is_neither_hit_tested_nor_pickable() {
    let mut host = region_host();
    region_press(&mut host, -260.0, 0.0);
    assert!(host.gestures.region_drag.is_none(), "a hidden region refuses the grab");
    assert!(matches!(host.focus.interaction, Interaction::SelectionPending { .. }), "the press reaches the background instead");
    let targets = host.pick_targets_at_screen_json(host.scene.world_to_screen(Point::new(-260.0, 0.0)).x, host.scene.world_to_screen(Point::new(-260.0, 0.0)).y);
    assert!(!targets.contains("region-hidden"), "and it is not offered as a pick target: {targets}");
    let visible = host.pick_targets_at_screen_json(host.scene.world_to_screen(Point::new(-50.0, 0.0)).x, host.scene.world_to_screen(Point::new(-50.0, 0.0)).y);
    assert!(visible.contains("\"region\"") && visible.contains("region-a"), "a visible one is, at the least specific generality: {visible}");
}

#[cfg(test)]
/// 🧮️ Regions are entities: they count against the fixed DESCRIPTOR census, never against the pointer
/// credits a gesture spends. A document whose regions overflow the census is refused whole.
#[test]
fn regions_count_against_the_descriptor_census_and_never_the_pointer_credits() {
    let mut host = region_host();
    let region = |index: usize| serde_json::json!({ "id": format!("r{index}"), "x": 0.0, "y": 0.0, "width": 8.0, "height": 8.0 });
    let rows: Vec<serde_json::Value> = (0..BOARD_DESCRIPTOR_ITEM_CAPACITY).map(region).collect();
    let overflowing = serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 20.0, "handles": [] }], "edges": [], "targetRegions": rows }).to_string();
    assert!(!{let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&overflowing,&mut control)}, "one node plus a full census of regions overruns the descriptor ceiling");
    assert_eq!(host.scene.regions.len(), 3, "and the refused parse leaves the live board exactly as it was");

    let mut host = region_host();
    host.set_active_utility("areaBrush");
    let _ = host.outbox.drain_events_json();
    region_press(&mut host, -200.0, -150.0);
    region_move_to(&mut host, -140.0, -70.0);
    region_release_at(&mut host, -140.0, -70.0);
    let _ = host.outbox.drain_events_json();
    assert!(event_authority_terminal_is_empty(&host.outbox, &host.commit), "a whole paint gesture leaves the event terminal as it found it");
    assert!(!host.outbox.event_schema_fault, "and nothing faulted");
}
//#endregion 🎯️TargetRegions

#[cfg(test)]
fn icon_clip_centers(scene: &Scene) -> Vec<(f64, f64)> {
    let encoded: serde_json::Value = serde_json::from_str(&semio_framework_canvas::draw_list::scene_draw_list_json(scene, semio_framework_canvas::draw_list::DrawListOptions::default())).expect("icon draw list");
    encoded["commands"]
        .as_array()
        .expect("commands")
        .iter()
        .filter(|command| command[0].as_str() == Some("pc") && command[3][0].as_str() == Some("ci"))
        .map(|command| {
            let affine = command[2].as_array().expect("clip affine");
            let coeffs: Vec<f64> = affine.iter().map(|value| value.as_f64().expect("affine coefficient")).collect();
            let center = command[3].as_array().expect("clip circle");
            let x = center[1].as_f64().expect("circle x");
            let y = center[2].as_f64().expect("circle y");
            (coeffs[0] * x + coeffs[2] * y + coeffs[4], coeffs[1] * x + coeffs[3] * y + coeffs[5])
        })
        .collect()
}

#[cfg(test)]
/// 🗺️ A drag keeps each node's icon on that node while the previous icon scene is still retiring.
#[test]
fn dragged_node_icon_stays_on_the_node_while_icon_cache_retirement_is_pending() {
    let icon = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10"/></svg>"#;
    let mut host = BoardHost::default();
    let fixture = serde_json::json!({
        "schema": "board.ports.directed.v1",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [
            { "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 36.0, "iconKind": icon, "handles": [] },
            { "id": "node-b", "x": 400.0, "y": 0.0, "shape": "circle", "radius": 36.0, "iconKind": icon, "handles": [] }
        ],
        "edges": []
    });
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&fixture.to_string(),&mut control)});
    host.scene.set_size(800, 600, 1.0);
    host.style.set_automatic_lod(false);
    host.style.set_forced_draw_lod_label("detail");
    let _ = host.build_vector_scene();
    let origin = host.scene.world_to_screen(Point::new(0.0, 0.0));
    host.pointer_down_screen(origin.x, origin.y, 0, false, false);
    let first = host.scene.world_to_screen(Point::new(120.0, 80.0));
    host.pointer_move_screen(first.x, first.y, false, false, false);
    let _ = host.build_vector_scene();
    let second = host.scene.world_to_screen(Point::new(240.0, 160.0));
    host.pointer_move_screen(second.x, second.y, false, false, false);
    let painted = host.build_vector_scene();
    assert!(host.closing.opaque_scene_retirement.get().is_some(), "the previous icon scene is still retiring, which is when the icon used to freeze");
    let node_a = host.scene.nodes.get("node-a").expect("dragged node");
    let node_b = host.scene.nodes.get("node-b").expect("stationary node");
    assert!((node_a.x - 240.0).abs() < 1e-6 && (node_a.y - 160.0).abs() < 1e-6, "the drag committed the second move");
    let centers = icon_clip_centers(&painted);
    let expect = |id_screen: Point| centers.iter().any(|(x, y)| (x - id_screen.x).abs() < 1.0 && (y - id_screen.y).abs() < 1.0);
    assert!(expect(host.scene.world_to_screen(Point::new(node_a.x, node_a.y))), "dragged icon clips: {centers:?}");
    assert!(expect(host.scene.world_to_screen(Point::new(node_b.x, node_b.y))), "stationary icon clips: {centers:?}");
    let mut turns = 0usize;
    while !quarantine_world_content_step(&mut host.scene, &mut host.closing) {
        turns += 1;
        assert!(turns < 8_192, "world icon scene reaches a terminal release");
    }
    while !host.style.icon_paint_cache.close_step() {
        turns += 1;
        assert!(turns < 8_192, "icon paint cache reaches a terminal release");
    }
}

#[cfg(test)]
/// 🎯️ A board handle's authored `radius` is the cap size, and the edge leaves that cap rather than the node circle.
#[test]
fn board_edge_starts_at_the_handle_cap() {
    let board = serde_json::json!({
        "schema": "board.ports.directed.v1",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [
            { "id": "a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 20.0, "handles": [{ "id": "a:h", "handleKind": "door", "angle": 0.0, "radius": 3.0 }] },
            { "id": "b", "x": 200.0, "y": 0.0, "shape": "circle", "radius": 20.0, "handles": [{ "id": "b:h", "handleKind": "door", "angle": 3.141592653589793, "radius": 3.0 }] }
        ],
        "edges": [{ "id": "e", "source": "a:h", "target": "b:h" }]
    })
    .to_string();
    let mut host = BoardHost::default();
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board,&mut control)}, "the two-node board parses");
    let source = host.scene.handles.get("a:h").expect("source handle");
    assert!((effective_handle_radius(&host.scene, &host.style, source) - 3.0).abs() < 1e-9, "the authored cap radius is kept");
    let curve = board_edge_curve(&host.scene, &host.style, host.scene.edges.get("e").expect("edge")).expect("edge curve");
    let start = curve.p0();
    let end = curve.p3();
    assert!((start.x - 23.0).abs() < 1e-6 && start.y.abs() < 1e-6, "the edge leaves the east cap, outside the node circle: {start:?}");
    assert!((end.x - 177.0).abs() < 1e-6 && end.y.abs() < 1e-6, "the edge arrives at the west cap, outside the node circle: {end:?}");
}

//#region 🎬️Gestures
#[cfg(test)]
/// 🎬️ Two free nodes and one locked node, rotate ring disarmed, so a press on a node centre is always a node drag.
fn gesture_host() -> BoardHost {
    let node = |id: &str, x: f64, locked: bool| serde_json::json!({ "id": id, "x": x, "y": 0.0, "shape": "circle", "radius": 10.0, "locked": locked, "handles": [] });
    let fixture = serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [node("node-a", -40.0, false), node("node-b", 40.0, false), node("node-locked", 120.0, true)], "edges": [] }).to_string();
    let mut host = BoardHost::default();
    host.scene.set_size(800, 600, 1.0);
    set_camera_silent(&mut host.scene, &mut host.outbox, 0.0, 0.0, 1.0);
    set_transform_flags(&mut host.scene, &mut host.gestures, true, false);
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&fixture,&mut control)}, "the gesture fixture must parse");
    let _ = host.outbox.drain_events_json();
    host
}

#[cfg(test)]
/// 🐁️ Every drained row except `hover`, which travels on the framework's own hover lane.
fn gesture_rows(host: &mut BoardHost) -> Vec<serde_json::Value> {
    serde_json::from_str::<Vec<serde_json::Value>>(&host.outbox.drain_events_json()).expect("events parse").into_iter().filter(|row| row["name"] != "hover").collect()
}

#[cfg(test)]
/// 🎬️ One drag of an unselected node: the press stages its selection and publishes nothing, the frames are
/// transient `nodeMove` previews only, and the release is ONE batch — the `select` and the `drag` record,
/// tagged with the same gesture id, carrying the dragged ids and the one offset.
#[test]
fn a_node_drag_is_its_select_and_one_gesture_record_on_release() {
    let mut host = gesture_host();
    region_press(&mut host, -40.0, 0.0);
    assert_eq!(host.focus.selection.iter().cloned().collect::<Vec<_>>(), vec!["node-a".to_string()], "the engine selects at once");
    assert!(gesture_rows(&mut host).is_empty(), "but the press publishes nothing yet");
    region_move_to(&mut host, 0.0, 20.0);
    region_move_to(&mut host, 40.0, 40.0);
    let frames = gesture_rows(&mut host);
    assert!(frames.iter().all(|row| row["name"] == "nodeMove"), "a drag frame is a transient preview only: {frames:?}");
    region_release_at(&mut host, 40.0, 40.0);
    let released = gesture_rows(&mut host);
    assert_eq!(released.iter().map(|row| row["name"].as_str().unwrap_or_default()).collect::<Vec<_>>(), vec!["select", "gesture"], "one batch: {released:?}");
    let (select, record) = (&released[0]["payload"], &released[1]["payload"]);
    assert_eq!(select["ids"], serde_json::json!(["node-a"]));
    assert!(record["gestureId"].as_str().is_some_and(|id| id.starts_with("gesture-")) && select["gestureId"] == record["gestureId"], "both rows carry the same gesture id: {released:?}");
    assert_eq!((record["kind"].as_str(), record["dx"].as_f64(), record["dy"].as_f64()), (Some("drag"), Some(80.0), Some(40.0)));
    assert_eq!(record["targets"], serde_json::json!(["node-a"]));
    assert_eq!(record["proximity"], serde_json::json!([]));
    let node = host.scene.nodes.get("node-a").expect("node-a");
    assert_eq!((node.x, node.y), (40.0, 40.0), "the engine keeps the previewed position until the guest's commit re-syncs it");
}

#[cfg(test)]
/// 🖱️ A click on a node is a selection, not a gesture: the release publishes the untagged `select` alone.
#[test]
fn a_click_without_motion_publishes_only_its_selection() {
    let mut host = gesture_host();
    region_press(&mut host, 40.0, 0.0);
    region_release_at(&mut host, 40.0, 0.0);
    let released = gesture_rows(&mut host);
    assert_eq!(released.len(), 1, "one row: {released:?}");
    assert_eq!((released[0]["name"].as_str(), &released[0]["payload"]["ids"]), (Some("select"), &serde_json::json!(["node-b"])));
    assert!(released[0]["payload"].get("gestureId").is_none(), "no record follows, so the select is untagged");
    region_press(&mut host, 40.0, 0.0);
    region_release_at(&mut host, 40.0, 0.0);
    assert!(gesture_rows(&mut host).is_empty(), "re-clicking the selected node changes nothing and publishes nothing");
}

#[cfg(test)]
/// 👥️ A group drag moves every UNLOCKED selected member by one offset; the locked member neither previews nor
/// appears in the record, and a press on an already-selected member stages no selection change.
#[test]
fn a_group_drag_records_every_unlocked_member_and_one_offset() {
    let mut host = gesture_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-a".to_string(), "node-b".to_string(), "node-locked".to_string()]);
    region_press(&mut host, -40.0, 0.0);
    region_move_to(&mut host, -30.0, 5.0);
    region_release_at(&mut host, -30.0, 5.0);
    let released = gesture_rows(&mut host);
    let record = &released.iter().find(|row| row["name"] == "gesture").expect("gesture record")["payload"];
    assert!(!released.iter().any(|row| row["name"] == "select"), "the press kept the group, so no select rides the batch: {released:?}");
    assert_eq!(record["targets"], serde_json::json!(["node-a", "node-b"]), "the locked member stays out");
    assert_eq!((record["dx"].as_f64(), record["dy"].as_f64()), (Some(10.0), Some(5.0)));
    assert_eq!(host.scene.nodes.get("node-locked").map(|node| (node.x, node.y)), Some((120.0, 0.0)), "the locked member never previews a move");
    assert_eq!(host.scene.nodes.get("node-b").map(|node| (node.x, node.y)), Some((50.0, 5.0)), "every member previews the one offset");
}

#[cfg(test)]
/// ↩️ A cancelled drag leaves zero trace: positions AND the press's selection return, nothing dispatchable is
/// published (only transient restore frames for the sibling panes), and the next drag opens a fresh gesture.
#[test]
fn a_cancelled_node_drag_leaves_zero_trace() {
    for cancel in [BoardHost::pointer_cancel_screen as fn(&mut BoardHost) -> bool, BoardHost::cancel_area_select] {
        let mut host = gesture_host();
        set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-b".to_string()]);
        region_press(&mut host, -40.0, 0.0);
        region_move_to(&mut host, 0.0, 30.0);
        let _ = gesture_rows(&mut host);
        assert!(cancel(&mut host), "the cancel claims the live drag");
        assert!(matches!(host.focus.interaction, Interaction::None));
        assert_eq!(host.scene.nodes.get("node-a").map(|node| (node.x, node.y)), Some((-40.0, 0.0)), "the node returns");
        assert_eq!(host.focus.selection.iter().cloned().collect::<Vec<_>>(), vec!["node-b".to_string()], "the press's selection change is undone");
        let rows = gesture_rows(&mut host);
        assert!(rows.iter().all(|row| row["name"] == "nodeMove"), "only transient restore frames: {rows:?}");
        assert_eq!(rows.first().map(|row| (&row["payload"]["x"], &row["payload"]["y"])), Some((&serde_json::json!(-40), &serde_json::json!(0))), "the restore frame carries the grab-time position");
        region_press(&mut host, -40.0, 0.0);
        region_move_to(&mut host, -20.0, 0.0);
        region_release_at(&mut host, -20.0, 0.0);
        let next = gesture_rows(&mut host);
        let record = next.iter().find(|row| row["name"] == "gesture").expect("the next drag records");
        assert_eq!(record["payload"]["dx"].as_f64(), Some(20.0), "the next drag starts from the restored geometry");
    }
}

#[cfg(test)]
/// ✂️ A drag offset is recorded as the shortest decimal of its f32 value: pointer coordinates carry f32 precision,
/// so a release whose screen reading picked up f32 noise stores a short offset, never the f64 residue.
#[test]
fn a_drag_offset_is_recorded_without_f64_pointer_noise() {
    assert_eq!(board_pointer_offset(80.00003051757813), 80.00003);
    assert_eq!(board_pointer_offset(0.1 + 0.2), 0.3);
    for exact in [80.0, -12.5, 0.0, 1.0e-9, 1234.5] {
        assert_eq!(board_pointer_offset(exact), exact, "{exact} is already short");
    }
    assert!(board_pointer_offset(f64::NAN).is_nan());
    let mut host = gesture_host();
    let grab = host.scene.world_to_screen(Point::new(-40.0, 0.0));
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let (release_x, release_y) = (grab.x + 80.00003051757813, grab.y + 0.1 + 0.2);
    host.pointer_move_screen(grab.x + 40.0, grab.y, false, false, false);
    host.pointer_move_screen(release_x, release_y, false, false, false);
    host.pointer_up_screen(release_x, release_y, false, false, false);
    let released = gesture_rows(&mut host);
    let record = released.iter().find(|row| row["name"] == "gesture").map(|row| &row["payload"]).expect("the release records the drag");
    assert_eq!((record["dx"].as_f64(), record["dy"].as_f64()), (Some(80.00003), Some(0.3)), "the stored offset is short: {record}");
}
//#endregion 🎬️Gestures

//#region 🔗️DraftReferences
#[cfg(test)]
/// 🔗️ What an open time-travel draft references (`Board2dScene.highlighted_ids_json`) paints highlighted — a locked
/// entity too, never dimmed — while the selection keeps its own chrome; setting it publishes nothing, the overlay pass
/// repaints it without rebuilding the world cache, a fixture re-parse (every draft edit repaints the preview) keeps it,
/// and the empty set clears it.
#[test]
fn draft_referenced_ids_paint_highlighted_without_publishing() {
    let mut host = gesture_host();
    set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["node-b".to_string()]);
    host.focus.set_highlighted_ids(vec!["node-locked".into(), "node-a".into(), "node-b".into()]);
    assert!(gesture_rows(&mut host).is_empty(), "a highlight is never an event");
    let style = |host: &BoardHost, id: &str| resolve_node_style_kind(&host.focus, &host.style, host.scene.nodes.get(id).expect("painted node"), StyleChromePass::InteractionOverlay);
    assert_eq!(style(&host, "node-a"), BoardElementStyleKind::Highlighted);
    assert_eq!(style(&host, "node-locked"), BoardElementStyleKind::Highlighted, "a locked reference is highlighted, not dimmed");
    assert_eq!(style(&host, "node-b"), BoardElementStyleKind::Selected, "the selection keeps its chrome");
    assert!(interaction_overlay_entity_ids(&host.scene, &host.focus).contains("node-a"), "the overlay pass repaints the highlight");
    let node = |id: &str, x: f64| serde_json::json!({ "id": id, "x": x, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [] });
    let preview = serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [node("node-a", 0.0), node("node-b", 40.0)], "edges": [] }).to_string();
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&preview,&mut control)}, "the preview fixture parses");
    assert_eq!(host.focus.highlighted_ids_json().expect("highlighted ids"), r#"["node-a","node-b","node-locked"]"#, "a preview repaint keeps what the draft references");
    assert_eq!(style(&host, "node-b"), BoardElementStyleKind::Highlighted, "the re-parse cleared the selection, not the highlight");
    host.focus.set_highlighted_ids(Vec::new());
    assert_eq!((style(&host, "node-a"), host.focus.highlighted_ids_json().expect("highlighted ids")), (BoardElementStyleKind::Neutral, "[]".to_string()), "the empty set clears it");
}
//#endregion 🔗️DraftReferences

#[cfg(test)]
/// 🧷️ A consumer's installed naming is what the board mints with: brush placements carry its prefixes and a fill
/// snapshot built under it hands the job its labels, while an untouched host keeps the neutral naming.
#[test]
fn installed_naming_owns_brush_ids_and_fill_labels() {
    fn probe_stage_label(stage: BoardFillStage) -> &'static str {
        if stage == BoardFillStage::Complete { "probe-complete" } else { "probe-busy" }
    }
    static PROBE_NAMING: BoardNaming = BoardNaming {
        brush_node_prefix: "probe.node.",
        brush_edge_prefix: "probe.edge.",
        fill_node_prefix: "probe.fill.",
        fill_edge_prefix: "probe.fill.edge.",
        fill_stage_label: probe_stage_label,
        fill_stale_operation_fault: "probe-stale",
    };
    let place = |naming: Option<&'static BoardNaming>| {
        let mut host = BoardHost::new();
        if let Some(naming) = naming {
            host.style.set_naming(naming);
        }
        let events = probe_brush_placement_events(&mut host);
        assert!(std::ptr::eq(host.style.naming(), naming.unwrap_or(&NEUTRAL_BOARD_NAMING)), "the host reports the naming it mints with");
        events
    };
    let neutral = place(None);
    assert!(neutral.contains("\"board.brush.1\"") && neutral.contains("\"board.brush.edge.1\""), "the untouched host mints neutral ids: {neutral}");
    let probe = place(Some(&PROBE_NAMING));
    assert!(probe.contains("\"probe.node.1\"") && probe.contains("\"probe.edge.1\""), "the installed naming prefixes brush placements: {probe}");
    assert!(!probe.contains("board.brush."), "no neutral id leaks past an installed naming: {probe}");
    let mut ingress = BoardFillSnapshotIngress::new(0.0, &PROBE_NAMING);
    let snapshot = ingress.take_snapshot().expect("empty probe snapshot");
    assert!(std::ptr::eq(snapshot.naming, &PROBE_NAMING), "the ingress hands its naming to the snapshot");
    let mut job = BoardFillJob::with_operation(snapshot, 0, semio_framework_job::Operation::new(semio_framework_job::OperationId(1), semio_framework_job::RevisionId(1), semio_framework_job::Generation(1), 1));
    assert!(std::ptr::eq(job.naming, &PROBE_NAMING), "the job keeps the snapshot's naming");
    assert_eq!((job.naming.fill_stage_label)(job.stage()), "probe-complete");
    close_probe_fill_job(&mut job);
}

#[cfg(test)]
fn close_probe_fill_job(job: &mut BoardFillJob) {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    for turn in 0.. {
        assert!(turn < 64, "probe job close reaches terminal-empty");
        let copy = job.next_close_copy_byte_demand().expect("job copy demand");
        let exact = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: job.next_close_capacity_byte_demand(copy).expect("job capacity demand"),
            maximum_release_bytes: job.next_close_release_byte_demand().expect("job release demand"),
            maximum_depth: job.next_close_depth_demand().expect("job depth demand"),
        };
        match job.close_step(exact) {
            InteractiveJobCloseStep::Pending { .. } => {}
            InteractiveJobCloseStep::Complete { .. } => break,
            other => panic!("probe job close ended in {other:?}"),
        }
    }
    assert!(job.terminal_is_empty());
}

#[cfg(test)]
fn probe_brush_placement_events(host: &mut BoardHost) -> String {
    host.scene.set_size(800, 600, 1.0);
    host.set_active_utility("brush");
    host.set_suggestion_offset(40.0);
    host.set_brush_node_size(40.0);
    let catalogs = serde_json::json!({
        "handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],
        "nodeKinds": [{ "id": "brush.kind", "name": "Brush Kind", "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }] }]
    });
    host.style.set_board_kind_catalogs_from_json(&catalogs.to_string()).expect("probe catalogs");
    let scene = SceneDescriptor {
        nodes: vec![NodeDescriptor { id: "a".into(), x: 0.0, y: 0.0, draggable: Some(true), selected: None, style: None, text: None, icon_kind: None, node_kind: Some("a.kind".into()), user_data: None, visible: None, locked: None, root: None, shape: Some("circle".into()), radius: Some(40.0), width: None, height: None, scale: None }].into(),
        handles: vec![HandleDescriptor { id: "a:h0".into(), node_id: "a".into(), angle: 0.0, radius: None, scale: None, selected: None, visible: None, locked: None, style: None, handle_kind: Some("port".into()), color: None, icon_kind: None, user_data: None }].into(),
        edges: vec![].into(),
        wires: vec![].into(),
        regions: vec![],
        selection_exit_highlight_ids: vec![],
    };
    host.sync_descriptor(&scene).expect("probe scene syncs");
    let handle = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
    let slot = host.scene.world_to_screen(handle + (handle - Point::new(0.0, 0.0)));
    host.pointer_move_screen(slot.x, slot.y, false, false, false);
    let _ = host.outbox.drain_events_json();
    host.pointer_leave_screen(true);
    host.outbox.drain_events_json()
}

#[cfg(test)]
/// 📮️ A scene's wire naming reaches a host no domain configured: a bare host mints the scene's brush prefixes, the wire form
/// round-trips a static naming byte for byte, malformed forms are refused without touching the prefixes in force, clearing
/// falls back to the installed naming, and installing a static naming drops an earlier wire override.
#[test]
fn scene_wire_placement_naming_overrides_brush_prefixes() {
    static WIRE_PROBE: BoardNaming = BoardNaming {
        brush_node_prefix: "wire.node.",
        brush_edge_prefix: "wire.edge.",
        fill_node_prefix: "wire.fill.",
        fill_edge_prefix: "wire.fill.edge.",
        fill_stage_label: BoardFillStage::id,
        fill_stale_operation_fault: "wire-stale",
    };
    let json = WIRE_PROBE.placement_naming().to_json();
    assert_eq!(json, r#"{"brushNodePrefix":"wire.node.","brushEdgePrefix":"wire.edge."}"#);
    assert_eq!(BoardPlacementNaming::from_json(&json).expect("wire form parses"), WIRE_PROBE.placement_naming());
    let mut host = BoardHost::new();
    host.style.set_placement_naming_json(Some(&json)).expect("wire naming installs");
    assert_eq!(host.style.brush_placement_naming(), WIRE_PROBE.placement_naming());
    let events = probe_brush_placement_events(&mut host);
    assert!(events.contains("\"wire.node.1\"") && events.contains("\"wire.edge.1\""), "the wire prefixes mint the placement: {events}");
    assert!(!events.contains("board.brush."), "no neutral id leaks past a wire naming: {events}");
    assert!(host.style.set_placement_naming_json(Some(r#"{"brushNodePrefix":"","brushEdgePrefix":"x."}"#)).is_err());
    assert!(host.style.set_placement_naming_json(Some(r#"{"brushNodePrefix":"a.","brushEdgePrefix":"b.","extra":1}"#)).is_err());
    assert!(host.style.set_placement_naming_json(Some("[]")).is_err());
    assert_eq!(host.style.brush_placement_naming(), WIRE_PROBE.placement_naming(), "a refused wire form keeps the prefixes in force");
    host.style.set_placement_naming_json(None).expect("wire naming clears");
    assert_eq!(host.style.brush_placement_naming(), NEUTRAL_BOARD_NAMING.placement_naming());
    host.style.set_placement_naming_json(Some(&json)).expect("wire naming installs again");
    host.style.set_naming(&NEUTRAL_BOARD_NAMING);
    assert_eq!(host.style.brush_placement_naming(), NEUTRAL_BOARD_NAMING.placement_naming(), "a static naming replaces an earlier wire override");
}
