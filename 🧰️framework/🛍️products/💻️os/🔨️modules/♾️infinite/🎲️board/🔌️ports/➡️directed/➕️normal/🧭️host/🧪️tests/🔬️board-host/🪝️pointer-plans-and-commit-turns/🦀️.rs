/// 🪝️ Pointer plans and commit turns: the board step context and pointer commit driver, the wheel and pan plans, drag item overflow, drag commit budgets, revisioned selection plans, non-drag commit yields and selection event reservations.

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
