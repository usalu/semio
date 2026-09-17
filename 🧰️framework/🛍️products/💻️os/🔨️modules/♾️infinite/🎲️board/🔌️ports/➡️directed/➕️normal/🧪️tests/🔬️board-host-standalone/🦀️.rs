#[cfg(test)]
    fn board_property_retirement_credits(properties: &graph::PropertyBag) -> Result<(usize, usize), BoardEventFault> {
        let mut stack: Box<[Option<(&graph::manifest::PropertyValue, u16)>; BOARD_POINTER_ITEM_CAPACITY]> = Box::new(std::array::from_fn(|_| None));
        let mut stack_len = 0usize;
        let mut nodes = 0usize;
        let mut bytes = 0usize;
        for (key, value) in properties {
            bytes = bytes.checked_add(key.len()).ok_or(BoardEventFault::ByteCredits)?;
            if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            stack[stack_len] = Some((value, 1));
            stack_len += 1;
        }
        while stack_len > 0 {
            stack_len -= 1;
            let (value, depth) = stack[stack_len].take().expect("property pre-admission stack owner");
            nodes += 1;
            if nodes > BOARD_POINTER_ITEM_CAPACITY || usize::from(depth) > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            match value {
                graph::manifest::PropertyValue::String(value) => {
                    bytes = bytes.checked_add(value.len()).ok_or(BoardEventFault::ByteCredits)?;
                }
                graph::manifest::PropertyValue::Array(values) => {
                    for value in values {
                        if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                            return Err(BoardEventFault::ItemCredits);
                        }
                        stack[stack_len] = Some((value, depth + 1));
                        stack_len += 1;
                    }
                }
                graph::manifest::PropertyValue::Object(values) => {
                    for (key, value) in values {
                        bytes = bytes.checked_add(key.len()).ok_or(BoardEventFault::ByteCredits)?;
                        if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                            return Err(BoardEventFault::ItemCredits);
                        }
                        stack[stack_len] = Some((value, depth + 1));
                        stack_len += 1;
                    }
                }
                graph::manifest::PropertyValue::Null | graph::manifest::PropertyValue::Bool(_) | graph::manifest::PropertyValue::Number(_) => {}
            }
            if bytes > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardEventFault::ByteCredits);
            }
        }
        Ok((nodes, bytes))
    }

#[cfg(test)]
    #[test]
    fn board_fill_descriptor_backings_are_exact_heap_owners_with_bounded_transfer_frames() {
        type SourcePages = BoardFillFixedPages<BoardFillSource, { (BOARD_FILL_SOURCE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS }>;
        type VirtualHandlePages = BoardFillFixedPages<BoardFillVirtualHandle, { (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS }>;
        let sources = SourcePages::try_new().expect("source descriptor owner");
        let virtual_handles = VirtualHandlePages::try_new().expect("virtual-handle descriptor owner");
        assert_eq!(sources.pages.len(), (BOARD_FILL_SOURCE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS);
        assert_eq!(virtual_handles.pages.len(), (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS);
        assert_eq!(size_of::<SourcePages>(), 4 * size_of::<usize>());
        assert_eq!(size_of::<VirtualHandlePages>(), 4 * size_of::<usize>());
        assert!(size_of::<BoardFillSnapshot>() <= 1_024);
        assert!(size_of::<BoardFillSnapshotCapture>() <= 32 * 1_024);
        assert!(size_of::<BoardFillSnapshotIngress>() <= 32 * 1_024);
        assert!(size_of::<BoardFillPlacement>() <= 32 * 1_024);
        assert!(size_of::<BoardFillCommitPlacement>() <= 32 * 1_024);
        assert!(size_of::<BoardFillCommitCandidate>() <= 32 * 1_024);
        assert!(size_of::<BoardFillCommitEncoder>() <= 32 * 1_024);
        assert!(size_of::<BoardFillJobState>() <= 32 * 1_024);
        assert!(size_of::<BoardFillCheckpoint>() <= 32 * 1_024);
        assert!(size_of::<BoardFillJob>() <= 32 * 1_024);
        assert!(size_of::<Result<BoardFillJob, BoardFillCheckpoint>>() <= 32 * 1_024);
        assert!(size_of::<semio_framework_job::WorkerJobOutcome<BoardFillJob>>() <= 32 * 1_024);
        assert!(size_of::<semio_framework_job::MountedWorkerJobSession<BoardFillJob>>() <= 32 * 1_024);
        assert!(size_of::<semio_framework_job::WorkerJobSessionAdmissionRejected<BoardFillJob>>() <= 32 * 1_024);
        assert!(size_of::<Result<semio_framework_job::MountedWorkerJobSession<BoardFillJob>, semio_framework_job::WorkerJobSessionAdmissionRejected<BoardFillJob>>>() <= 32 * 1_024);
        assert!(sources.terminal_is_empty());
        assert!(virtual_handles.terminal_is_empty());
        assert!(BoardFillFixedPages::<u8, { usize::MAX }>::try_new().is_err());
    }

#[cfg(test)]
    fn with_board_step_context<T>(fuel: u64, cancel: semio_framework_job::CancelToken, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
        let mut sequence = 0;
        let mut context =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX), cancel, semio_framework_job::default_now_us, &mut sequence);
        step(&mut context)
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
        direct.set_size(800, 600, 1.0);
        planned.set_size(800, 600, 1.0);
        direct.wheel_screen(320.0, 240.0, -12.0);
        let plan = planned.plan_wheel(320.0, 240.0, -12.0);
        assert!(planned.commit_wheel(&plan));
        assert_eq!([direct.camera.x, direct.camera.y, direct.camera.zoom], [planned.camera.x, planned.camera.y, planned.camera.zoom]);

        let stale = planned.plan_wheel(320.0, 240.0, -12.0);
        planned.pointer_down_screen(10.0, 10.0, 1, false, false);
        let replacement = [planned.camera.x, planned.camera.y, planned.camera.zoom];
        assert!(!planned.commit_wheel(&stale));
        assert_eq!([planned.camera.x, planned.camera.y, planned.camera.zoom], replacement);
    }

#[cfg(test)]
    #[test]
    fn pointer_pan_plan_matches_direct_and_rejects_stale_interaction() {
        let mut direct = BoardHost::default();
        let mut planned = BoardHost::default();
        direct.set_size(800, 600, 1.0);
        planned.set_size(800, 600, 1.0);
        direct.pointer_down_screen(100.0, 120.0, 1, false, false);
        planned.pointer_down_screen(100.0, 120.0, 1, false, false);
        direct.pointer_move_screen(140.0, 150.0, false, false, false);
        let plan = planned.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 140.0, y: 150.0, shift: false, ctrl_or_meta: false, alt: false }).expect("pan plan");
        planned.begin_pointer_commit(plan).expect("retained pan plan");
        drive_pointer_commit(&mut planned);
        assert_eq!([direct.camera.x, direct.camera.y, direct.camera.zoom], [planned.camera.x, planned.camera.y, planned.camera.zoom]);

        let stale = planned.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 160.0, y: 170.0, shift: false, ctrl_or_meta: false, alt: false }).expect("stale pan plan");
        planned.set_size(801, 600, 1.0);
        let camera = [planned.camera.x, planned.camera.y, planned.camera.zoom];
        assert!(planned.begin_pointer_commit(stale).is_err());
        assert_eq!([planned.camera.x, planned.camera.y, planned.camera.zoom], camera);
    }

#[cfg(test)]
    #[test]
    fn pointer_plan_rejects_drag_item_overflow_and_retires_one_delta_per_step() {
        let mut host = BoardHost::default();
        let start_positions = (0..=BOARD_POINTER_ITEM_CAPACITY).map(|index| (format!("node-{index}"), (index as f64, 0.0))).collect();
        host.interaction = Interaction::DragNodes { primary_id: "node-0".into(), offset: Vec2::ZERO, start_positions, proximity_pair: None };
        assert_eq!(host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 1.0, y: 1.0, shift: false, ctrl_or_meta: false, alt: false }).unwrap_err(), BoardPointerPlanFault::ItemCredits);

        let mut plan = BoardPointerPlan::empty(0, BoardPointerPlanKind::DragMove);
        plan.push_delta("a", 1.0, 2.0).unwrap();
        plan.push_delta("b", 3.0, 4.0).unwrap();
        let mut retirement = BoardPointerPlanRetirement::new(plan);
        assert!(!retirement.close_step());
        assert!(!retirement.close_step());
        assert!(retirement.close_step());
        assert!(retirement.terminal_is_empty());

        let mut escaped = BoardPointerPlan::empty(0, BoardPointerPlanKind::FinishDrag);
        escaped.push_delta("a\"\\\n", 1.0, 2.0).unwrap();
        let mut json = String::new();
        escaped.write_events_json(&mut json).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value[0]["payload"]["moves"][0]["id"], "a\"\\\n");

        let mut oversized = BoardPointerPlan::empty(0, BoardPointerPlanKind::FinishDrag);
        oversized.push_delta(&"\n".repeat(BOARD_POINTER_BYTE_CAPACITY / 2), 1.0, 2.0).unwrap();
        assert_eq!(oversized.write_events_json(&mut json), Err(BoardPointerPlanFault::ByteCredits));
    }

#[cfg(test)]
    #[test]
    fn drag_commit_obeys_zero_budget_one_delta_turn_cancel_and_publication_witness() {
        let mut host = deletion_fixture("node-a");
        host.interaction = Interaction::DragNodes { primary_id: "node-a".into(), offset: Vec2::ZERO, start_positions: [("node-a".to_string(), (0.0, 0.0)), ("node-b".to_string(), (20.0, 0.0))].into_iter().collect(), proximity_pair: None };
        let plan = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Up, x: 10.0, y: 5.0, shift: false, ctrl_or_meta: false, alt: false }).expect("finish drag plan");
        host.begin_pointer_commit(plan).expect("retained drag commit");
        let before = host.nodes.get("node-a").map(|node| (node.x, node.y));
        let zero = semio_framework_job::root_cancel_token();
        assert_eq!(with_board_step_context(0, zero, |context| host.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(host.nodes.get("node-a").map(|node| (node.x, node.y)), before);

        let live = semio_framework_job::root_cancel_token();
        assert_eq!(with_board_step_context(1, live.clone(), |context| host.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_ne!(host.nodes.get("node-a").map(|node| (node.x, node.y)), before, "the FIRST live turn applies the delta");
        // 🪜️ The tail is cooperative and bounded, never a fixed ladder length: every commit phase a
        // gesture gains (the proximity pair, …) adds a turn, and pinning the exact count made this law
        // red for every such addition while proving nothing the `Pending`-until-`Complete` shape does not.
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
        let publication = host.take_pointer_publication().expect("complete drag publication");
        assert!(publication.events_json().contains("nodeDragEnd"));
        assert!(host.pointer_authority_terminal_is_empty());

        let mut cancelled = deletion_fixture("node-a");
        cancelled.interaction = Interaction::DragNodes { primary_id: "node-a".into(), offset: Vec2::ZERO, start_positions: [("node-a".to_string(), (0.0, 0.0))].into_iter().collect(), proximity_pair: None };
        let plan = cancelled.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 8.0, y: 3.0, shift: false, ctrl_or_meta: false, alt: false }).expect("cancelled drag plan");
        cancelled.begin_pointer_commit(plan).expect("cancelled retained commit");
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        assert_eq!(with_board_step_context(1, cancel.clone(), |context| cancelled.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, cancel, |context| cancelled.step_pointer_commit(context)), BoardAuthorityStep::Cancelled);
        assert_eq!(cancelled.nodes.get("node-a").map(|node| (node.x, node.y)), Some((0.0, 0.0)));
        assert!(cancelled.pointer_authority_terminal_is_empty());
    }

#[cfg(test)]
    #[test]
    fn selection_move_and_up_are_revisioned_fixed_page_plans() {
        let mut host = BoardHost::default();
        host.set_size(800, 600, 1.0);
        host.pointer_down_screen(100.0, 100.0, 0, false, false);
        assert!(matches!(host.interaction, Interaction::SelectionPending { .. }));
        let preview = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 180.0, y: 160.0, shift: false, ctrl_or_meta: false, alt: false }).expect("selection preview plan");
        assert_eq!(preview.event_count(), 1);
        assert!(preview.events_json().contains("preselect"));
        host.begin_pointer_commit(preview).expect("retained selection preview");
        drive_pointer_commit(&mut host);
        let mut publication = host.take_pointer_publication().expect("preview publication");
        assert!(publication.close_step());
        assert!(matches!(host.interaction, Interaction::Selection { .. }));

        let stale = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 200.0, y: 180.0, shift: false, ctrl_or_meta: false, alt: false }).expect("stale selection plan");
        host.interaction_revision = host.interaction_revision.wrapping_add(1);
        assert!(host.begin_pointer_commit(stale).is_err());

        let commit = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Up, x: 200.0, y: 180.0, shift: false, ctrl_or_meta: false, alt: false }).expect("selection commit plan");
        assert_eq!(commit.event_count(), 1);
        assert!(commit.events_json().contains("select"));
        host.begin_pointer_commit(commit).expect("retained selection commit");
        drive_pointer_commit(&mut host);
        assert!(matches!(host.interaction, Interaction::None));
    }

#[cfg(test)]
    #[test]
    fn non_drag_pointer_commit_yields_between_selection_link_and_brush_items() {
        let live = semio_framework_job::root_cancel_token();
        let mut selection = deletion_fixture("node-a");
        selection.interaction = Interaction::SelectionPending { initial_ids: ["node-a".to_owned()].into_iter().collect(), start: Point::new(0.0, 0.0), start_screen: Point::new(0.0, 0.0) };
        let mut selection_plan = BoardPointerPlan::empty(selection.interaction_revision, BoardPointerPlanKind::SelectionCommit);
        selection_plan.push_delta("node-a", 0.0, 0.0).unwrap();
        selection_plan.seal_events().unwrap();
        selection.begin_pointer_commit(selection_plan).unwrap();
        assert_eq!(with_board_step_context(0, live.clone(), |context| selection.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert!(matches!(selection.interaction, Interaction::SelectionPending { .. }));
        assert_eq!(with_board_step_context(1, live.clone(), |context| selection.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert!(matches!(selection.interaction, Interaction::None));
        drive_pointer_commit(&mut selection);
        assert!(selection.selection.contains("node-a"));

        let mut link = BoardHost::default();
        link.interaction = Interaction::LinkAtSourceHandle { source_id: "source".into(), start_screen: Point::new(0.0, 0.0) };
        let mut link_plan = BoardPointerPlan::empty(link.interaction_revision, BoardPointerPlanKind::Idle);
        let source = link_plan.push_id("source").unwrap();
        let compat_key = link_plan.push_id("source|").unwrap();
        let ring_key = link_plan.push_id("source||").unwrap();
        link_plan.kind = BoardPointerPlanKind::LinkMove { source, target: None, hover: None, compat_key, ring_key, end_world: Point::new(2.0, 3.0), activated: true, start_screen: Point::new(0.0, 0.0) };
        link.begin_pointer_commit(link_plan).unwrap();
        assert_eq!(with_board_step_context(1, live.clone(), |context| link.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert!(link.link_compat_nodes_emit_key.is_none());
        assert_eq!(with_board_step_context(1, live.clone(), |context| link.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(link.link_compat_nodes_emit_key.as_deref(), Some("source|"));
        drive_pointer_commit(&mut link);

        let mut brush = BoardHost::default();
        brush.active_utility = ActiveUtility::Brush;
        let mut brush_plan = BoardPointerPlan::empty(brush.interaction_revision, BoardPointerPlanKind::Idle);
        brush_plan.push_delta("kind-a", 0.0, 0.0).unwrap();
        brush_plan.push_delta("kind-b", 1.0, 0.0).unwrap();
        brush_plan.kind = BoardPointerPlanKind::Brush { source: None, hover: None, alt: false, commit_old: false };
        brush.begin_pointer_commit(brush_plan).unwrap();
        assert_eq!(with_board_step_context(1, live.clone(), |context| brush.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(brush.brush_candidates.len(), 0);
        assert_eq!(with_board_step_context(1, live, |context| brush.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(brush.brush_candidates.len(), 0);
        drive_pointer_commit(&mut brush);
        assert_eq!(brush.brush_candidates.len(), 2);
    }

#[cfg(test)]
    #[test]
    fn typed_event_queue_preserves_fifo_saturation_and_one_event_close_progress() {
        let mut queue = BoardEventQueue::default();
        for index in 0..BOARD_EVENT_ITEM_CAPACITY {
            let payload = format!(r#"{{"id":"node-{index}"}}"#);
            queue.push(BoardOwnedEvent::from_payload(BoardEventKind::NodeMove, &payload, Some("node")).unwrap()).unwrap();
        }
        let overflow = BoardOwnedEvent::from_payload(BoardEventKind::Select, r#"{"ids":[]}"#, None).unwrap();
        assert!(queue.push(overflow).is_err());
        assert_eq!(queue.len(), BOARD_EVENT_ITEM_CAPACITY);
        for index in 0..BOARD_EVENT_ITEM_CAPACITY {
            let event = queue.pop().expect("fifo event");
            assert_eq!(event.kind(), BoardEventKind::NodeMove);
            assert!(event.payload_json().contains(&format!("node-{index}")));
        }
        assert!(queue.terminal_is_empty());

        queue.push(BoardOwnedEvent::from_payload(BoardEventKind::Select, r#"{"ids":["a"]}"#, None).unwrap()).unwrap();
        queue.push(BoardOwnedEvent::from_payload(BoardEventKind::Hover, r#"{"id":"a"}"#, None).unwrap()).unwrap();
        assert!(!queue.close_step());
        assert!(!queue.close_step());
        assert!(queue.close_step());
        assert!(queue.terminal_is_empty());
    }

#[cfg(test)]
    #[test]
    fn selection_event_reservation_is_flat_exact_and_precedes_mutation() {
        let mut host = BoardHost::default();
        host.set_selection_ids(&["a\"\\\n".into(), "b".into()]);
        assert_eq!(host.selection.iter().cloned().collect::<Vec<_>>(), vec!["a\"\\\n".to_string(), "b".to_string()]);
        let event = host.pop_owned_event().expect("flat select event");
        assert_eq!(event.kind(), BoardEventKind::Select);
        let payload: serde_json::Value = serde_json::from_str(event.payload_json()).unwrap();
        assert_eq!(payload["ids"], serde_json::json!(["a\"\\\n", "b"]));

        let mut saturated = BoardHost::default();
        for _ in 0..BOARD_EVENT_ITEM_CAPACITY {
            saturated.events.push(BoardOwnedEvent::from_payload(BoardEventKind::Hover, r#"{"id":null}"#, None).unwrap()).unwrap();
        }
        saturated.set_selection_ids(&["retained".into()]);
        assert!(saturated.selection.is_empty());
        assert!(saturated.event_overflow.is_some());
        assert_eq!(saturated.events.len(), BOARD_EVENT_ITEM_CAPACITY);
    }

#[cfg(test)]
    #[test]
    fn board_fixture_json_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️board-ingress.json")).unwrap();
        for fixture in vectors.as_array().unwrap() {
            let mut host = BoardHost::default();
            assert!(host.parse_fixture_json(&fixture.to_string()));
            assert_eq!(host.nodes.len(), fixture["nodes"].as_array().unwrap().len());
            for node in fixture["nodes"].as_array().unwrap() {
                let actual = &host.nodes[node["id"].as_str().unwrap()];
                assert_eq!(actual.x, node["x"].as_f64().unwrap());
                assert_eq!(actual.y, node["y"].as_f64().unwrap());
            }
            let count = host.nodes.len();
            assert!(!host.parse_fixture_json("{"));
            assert_eq!(host.nodes.len(), count);
        }
    }

#[cfg(test)]
    fn deletion_fixture(node_id: &str) -> BoardHost {
        let mut host = BoardHost::default();
        let fixture = serde_json::json!({
            "schema": "reasoning.mindmap.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": node_id, "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0 },
                { "id": "node-b", "x": 20.0, "y": 0.0, "shape": "circle", "radius": 10.0 }
            ],
            "edges": [{ "id": "edge-a-b", "source": node_id, "target": "node-b" }]
        });
        assert!(host.parse_fixture_json(&fixture.to_string()));
        while host.pop_owned_event().is_some() {}
        host.set_selection_ids_silent(&[node_id.to_string()]);
        host
    }

#[cfg(test)]
    #[test]
    fn delete_plan_retains_fifo_until_exact_credits_and_rejects_stale_or_oversized() {
        let mut saturated = deletion_fixture("node-a");
        for _ in 0..BOARD_EVENT_ITEM_CAPACITY {
            saturated.events.push(BoardOwnedEvent::hover(None, None).unwrap()).unwrap();
        }
        saturated.delete_selection();
        assert!(saturated.nodes.contains_key("node-a"));
        assert!(saturated.pending_delete_planning.is_some());
        let live = semio_framework_job::root_cancel_token();
        let mut turns = 0usize;
        while saturated.pending_delete_planning.is_some() || saturated.pending_delete_operation.is_some() {
            turns += 1;
            assert!(turns <= 768);
            let _ = with_board_step_context(1, live.clone(), |context| saturated.step_event_authority(context));
            if let Some(event) = saturated.pop_owned_event() {
                assert_eq!(event.kind(), BoardEventKind::Hover);
            }
        }
        assert!(turns > 4);
        assert!(saturated.pending_delete_operation.is_none());
        assert!(!saturated.nodes.contains_key("node-a"));
        while saturated.pop_owned_event().is_some() {}

        let mut stale = deletion_fixture("node-a");
        for _ in 0..BOARD_EVENT_ITEM_CAPACITY {
            stale.events.push(BoardOwnedEvent::hover(None, None).unwrap()).unwrap();
        }
        stale.delete_selection();
        stale.interaction_revision = stale.interaction_revision.wrapping_add(1);
        assert_eq!(with_board_step_context(1, live.clone(), |context| stale.step_event_authority(context)), BoardAuthorityStep::Fault);
        assert!(stale.pending_delete_operation.is_none());
        assert!(stale.nodes.contains_key("node-a"));
        assert!(stale.event_terminal_faulted());

        let oversized_id = "x".repeat(BOARD_POINTER_BYTE_CAPACITY + 1);
        let mut oversized = deletion_fixture("node-a");
        let mut oversized_node = oversized.nodes.remove("node-a").unwrap();
        oversized_node.id.clone_from(&oversized_id);
        oversized.nodes.insert(oversized_id.clone(), oversized_node);
        oversized.selection.clear();
        oversized.selection.insert(oversized_id.clone());
        oversized.delete_selection();
        assert_eq!(with_board_step_context(1, live.clone(), |context| oversized.step_event_authority(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, live.clone(), |context| oversized.step_event_authority(context)), BoardAuthorityStep::Fault);
        assert!(oversized.nodes.contains_key(&oversized_id));
        assert!(oversized.pending_delete_planning.is_none());
        assert!(oversized.pending_delete_operation.is_none());
        assert!(oversized.event_terminal_faulted());

        let mut interrupted = deletion_fixture("node-a");
        interrupted.delete_selection();
        assert!(interrupted.pending_delete_planning.is_some());
        let cancel = semio_framework_job::root_cancel_token();
        assert_eq!(with_board_step_context(1, cancel.clone(), |context| interrupted.step_event_authority(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, cancel.clone(), |context| interrupted.step_event_authority(context)), BoardAuthorityStep::Pending);
        assert!(!with_board_step_context(1, cancel.clone(), |context| interrupted.close_event_authority_step(context)));
        // 🪜️ Cooperative and bounded, never an exact ladder length: the first turn above is already
        // proven not to close, and how many more a cancelled delete needs is a property of the planner's
        // pending state, not of this law (it was pinned at `> 4` and went red the moment that shrank).
        let mut close_turns = 1usize;
        loop {
            close_turns += 1;
            assert!(close_turns <= 32, "the cancelled delete never closed");
            if with_board_step_context(1, cancel.clone(), |context| interrupted.close_event_authority_step(context)) {
                break;
            }
        }
        assert!(close_turns > 1, "the close must be cooperative, not a single-shot teardown");
        assert!(interrupted.event_authority_terminal_is_empty());
    }

#[cfg(test)]
    #[test]
    fn delete_property_pre_admission_rejects_hostile_nodes_without_transfer_or_mutation() {
        let mut host = deletion_fixture("node-a");
        host.nodes.get_mut("node-a").unwrap().properties.insert("hostile".into(), graph::manifest::PropertyValue::Array((0..=BOARD_POINTER_ITEM_CAPACITY).map(|_| graph::manifest::PropertyValue::Null).collect()));
        host.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        // 🧮️ One audited property node per turn: the hostile array is `BOARD_POINTER_ITEM_CAPACITY + 1`
        // entries, so the fault lands after that many turns — the budget follows the constant.
        for _ in 0..BOARD_POINTER_ITEM_CAPACITY * 4 {
            if matches!(with_board_step_context(1, live.clone(), |context| host.step_event_authority(context)), BoardAuthorityStep::Fault) {
                break;
            }
        }
        assert!(host.pending_delete_planning.is_none());
        assert!(host.pending_delete_operation.is_none());
        assert!(host.nodes.contains_key("node-a"));
        assert!(host.event_terminal_faulted());

        let mut node = host.nodes.remove("node-a").unwrap();
        let Some(graph::manifest::PropertyValue::Array(mut values)) = node.properties.remove("hostile") else { panic!("hostile property remains retained") };
        while values.pop().is_some() {}
        drop(values);
        drop(node);
    }

#[cfg(test)]
    #[test]
    fn delete_property_derivation_is_one_node_per_turn_and_cancel_restores_exact_owner() {
        let mut host = deletion_fixture("node-a");
        host.nodes.get_mut("node-a").unwrap().properties.insert("nested".into(), graph::manifest::PropertyValue::Array((0..32).map(|index| graph::manifest::PropertyValue::String(format!("value-{index}"))).collect()));
        host.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        let mut observed_audit = false;
        let mut previous_nodes = 0usize;
        for _ in 0..128 {
            let _ = with_board_step_context(1, live.clone(), |context| host.step_event_authority(context));
            if let Some(audit) = host.pending_delete_planning.as_ref().and_then(|planning| planning.property_audit.as_ref()) {
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
        assert!(host.nodes.contains_key("node-a"));
        let Some(graph::manifest::PropertyValue::Array(values)) = host.nodes.get("node-a").unwrap().properties.get("nested") else { panic!("cancelled property audit restored the original root") };
        assert_eq!(values.len(), 32);
        assert!(host.event_authority_terminal_is_empty());
    }

#[cfg(test)]
    #[test]
    fn retained_delete_plan_preserves_legacy_fifo_and_retires_mid_audit_stale_generation() {
        let mut legacy = deletion_fixture("node-a");
        legacy.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
        let legacy_plan = legacy.plan_delete_selection().unwrap();
        let legacy_order: Vec<_> = legacy_plan.entries[..usize::from(legacy_plan.len)].iter().flatten().map(|entry| (entry.kind, legacy_plan.id(entry.id).to_owned())).collect();

        let mut retained = deletion_fixture("node-a");
        retained.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
        retained.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        for _ in 0..512 {
            if retained.pending_delete_operation.is_some() {
                break;
            }
            assert_eq!(with_board_step_context(1, live.clone(), |context| retained.step_event_authority(context)), BoardAuthorityStep::Pending);
        }
        let retained_plan = &retained.pending_delete_operation.as_ref().expect("retained plan completed").plan;
        let retained_order: Vec<_> = retained_plan.entries[..usize::from(retained_plan.len)].iter().flatten().map(|entry| (entry.kind, retained_plan.id(entry.id).to_owned())).collect();
        assert_eq!(retained_order, legacy_order);

        let mut stale = deletion_fixture("node-a");
        stale.nodes.get_mut("node-a").unwrap().properties.insert("nested".into(), graph::manifest::PropertyValue::Array(vec![graph::manifest::PropertyValue::String("retained".into())]));
        stale.delete_selection();
        for _ in 0..128 {
            let _ = with_board_step_context(1, live.clone(), |context| stale.step_event_authority(context));
            if stale.pending_delete_planning.as_ref().is_some_and(|planning| planning.property_audit.is_some()) {
                break;
            }
        }
        stale.interaction_revision = stale.interaction_revision.wrapping_add(1);
        for _ in 0..128 {
            if matches!(with_board_step_context(1, live.clone(), |context| stale.step_event_authority(context)), BoardAuthorityStep::Fault) {
                break;
            }
        }
        assert!(stale.nodes.contains_key("node-a"));
        assert!(stale.nodes.get("node-a").unwrap().properties.contains_key("nested"));
        assert!(stale.pending_delete_planning.is_none());
        assert!(stale.event_terminal_faulted());
    }

#[cfg(test)]
    #[test]
    fn delete_property_key_overflow_faults_after_bounded_restore_without_mutation() {
        let mut host = deletion_fixture("node-a");
        let hostile = "k".repeat(BOARD_POINTER_BYTE_CAPACITY + 1);
        host.nodes.get_mut("node-a").unwrap().properties.insert(hostile.clone(), graph::manifest::PropertyValue::Null);
        host.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        for _ in 0..128 {
            if matches!(with_board_step_context(1, live.clone(), |context| host.step_event_authority(context)), BoardAuthorityStep::Fault) {
                break;
            }
        }
        assert!(host.event_terminal_faulted());
        assert!(host.nodes.contains_key("node-a"));
        assert!(host.nodes.get("node-a").unwrap().properties.contains_key(&hostile));
        assert!(host.pending_delete_planning.is_none());
    }

#[cfg(test)]
    #[test]
    fn removed_entity_retirement_witness_survives_interruption_and_releases_one_owner_per_turn() {
        let mut host = deletion_fixture("node-a");
        let node = host.nodes.remove("node-a").unwrap();
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
        let mut path = crate::BezPath::new();
        path.move_to((0.0, 0.0));
        for point in 0..1600 {
            path.line_to((f64::from(point), f64::from(point)));
        }
        scene.fill(FillRule::NonZero, Affine::IDENTITY, Color::from_rgba8(0, 0, 0, 255), None, &path);
        *host.world_content_cache.borrow_mut() = Some((host.content_scene_generation, BoardDrawLod::Detail, scene));
        assert!(!host.quarantine_world_content_step());
        assert!(host.opaque_scene_retirement.get().is_some());
        let mut turns = 1usize;
        while !host.quarantine_world_content_step() {
            turns += 1;
            assert!(turns < 4_096, "retained world scene cursor reaches exact terminal release");
        }
        assert!(turns > 1_600);
        assert!(host.world_content_cache.borrow().is_none());
        assert!(host.opaque_scene_retirement.get().is_none());
        assert!(!host.opaque_scene_faulted());
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
    fn fill_full_commit_candidate_contract(source: &str) -> bool {
        let Some(start) = source.find("pub struct BoardFillResult") else { return false };
        let Some(end) = source[start..].find("/// 🧵️ Persistent worker-owned fill search") else { return false };
        let codec = &source[start..start + end];
        let Some(job_start) = source.find("fn complete(&mut self, context: &mut semio_framework_job::StepContext") else { return false };
        let Some(job_end) = source[job_start..].find("fn fault_outcome") else { return false };
        let job = &source[job_start..job_start + job_end];
        codec.contains("pub struct BoardFillCommitPlacement")
            && codec.contains("pub edge_kind: BoardFillText")
            && codec.contains("pub handles: [Option<BoardFillCommitHandle>; BOARD_FILL_KIND_HANDLE_CAPACITY]")
            && codec.contains("pub placement: Option<BoardFillCommitPlacement>")
            && codec.contains("struct BoardFillCommitPayload")
            && codec.contains("BoardFillCommitPayload::new(&candidate.output)")
            && codec.contains("payload.page_count() == BOARD_FILL_COMMIT_PAGE_COUNT")
            && codec.contains("payload.page(page_index)")
            && codec.contains("text_byte: usize")
            && codec.contains("fn step_text")
            && codec.contains("self.bytes[offset + 2 + self.text_byte] = text.bytes[self.text_byte]")
            && !codec.contains("String")
            && !codec.contains("Vec<")
            && !codec.contains("BTreeMap")
            && !codec.contains("single_page()")
            && !codec.contains("const BOARD_FILL_COMMIT_BYTES: usize = 13")
            && !codec.contains("result.encode")
            && job.contains("start.saturating_add(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)")
            && job.contains("if end < BOARD_FILL_COMMIT_BYTES")
            && job.contains("encoder.output_cursor = end")
    }

#[cfg(test)]
    #[test]
    fn fill_full_commit_candidate_mutations_are_rejected() {
        let source = include_str!("../../🦀️.rs");
        assert_eq!(BOARD_FILL_COMMIT_BYTES, 10_406);
        assert_eq!(BOARD_FILL_COMMIT_PAGE_COUNT, 1);
        assert!(BOARD_FILL_COMMIT_BYTES <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES * BOARD_FILL_COMMIT_PAGE_COUNT);
        assert!(fill_full_commit_candidate_contract(source));
        let dynamic = source.replacen("pub struct BoardFillCommitPlacement {\n        pub node_id: BoardFillText,", "pub struct BoardFillCommitPlacement {\n        pub dynamic: String,\n        pub node_id: BoardFillText,", 1);
        assert!(!fill_full_commit_candidate_contract(&dynamic));
        let summary = source.replacen("const BOARD_FILL_COMMIT_BYTES: usize = BOARD_FILL_COMMIT_HANDLE_OFFSET + BOARD_FILL_KIND_HANDLE_CAPACITY * BOARD_FILL_COMMIT_HANDLE_BYTES;", "const BOARD_FILL_COMMIT_BYTES: usize = 13;", 1);
        assert!(!fill_full_commit_candidate_contract(&summary));
    }

#[cfg(test)]
    #[test]
    fn fill_commit_candidate_granularity_mutations_are_rejected() {
        let source = include_str!("../../🦀️.rs");
        assert!(fill_full_commit_candidate_contract(source));
        let text = source.replacen("self.bytes[offset + 2 + self.text_byte] = text.bytes[self.text_byte];", "self.bytes[offset + 2..offset + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES].copy_from_slice(&text.bytes);", 1);
        assert!(!fill_full_commit_candidate_contract(&text));
    }

#[cfg(test)]
    fn fill_ingress_granularity_contract(source: &str) -> bool {
        let Some(start) = source.find("impl BoardFillSnapshotIngress {") else { return false };
        let Some(end) = source[start..].find("impl Drop for BoardFillSnapshotIngress") else { return false };
        let ingress = &source[start..start + end];
        [
            "pub fn begin_node(&mut self)",
            "pub fn push_node_id_byte(&mut self, byte: u8)",
            "pub fn set_node_bound(&mut self, index: usize, value: f64)",
            "pub fn begin_handle(&mut self)",
            "pub fn push_handle_text_byte(&mut self, field: BoardFillIngressHandleText, byte: u8)",
            "pub fn begin_kind(&mut self)",
            "pub fn push_kind_text_byte(&mut self, field: BoardFillIngressKindText, byte: u8)",
            "pub fn push_kind_handle_text_byte(&mut self, field: BoardFillIngressTemplateText, byte: u8)",
            "pub fn begin_rule(&mut self)",
            "pub fn push_rule_text_byte(&mut self, field: BoardFillIngressRuleText, byte: u8)",
        ]
        .iter()
        .all(|marker| ingress.contains(marker))
            && ingress.matches("try_push_byte(byte)").count() == 13
            && !ingress.contains("pub fn push_node(")
            && !ingress.contains("pub fn push_handle(")
            && !ingress.contains("pub fn push_kind(")
            && !ingress.contains("pub fn push_rule(")
            && !ingress.contains("for byte")
    }

#[cfg(test)]
    #[test]
    fn fill_ingress_granularity_mutations_are_rejected() {
        let source = include_str!("../../🦀️.rs");
        assert!(fill_ingress_granularity_contract(source));
        let fields = source.replacen("pub fn set_node_bound(&mut self, index: usize, value: f64)", "pub fn push_node(&mut self, bounds: [f64; 4])", 1);
        assert!(!fill_ingress_granularity_contract(&fields));
        let text = source.replacen("pub fn push_node_id_byte(&mut self, byte: u8)", "pub fn push_node_id(&mut self, bytes: &[u8])", 1);
        assert!(!fill_ingress_granularity_contract(&text));
    }

#[cfg(test)]
    fn fill_owned_page_handback_contract(source: &str) -> bool {
        let Some(start) = source.find("impl BoardFillSnapshotIngress {") else { return false };
        let Some(end) = source[start..].find("impl Drop for BoardFillJob") else { return false };
        let retained = &source[start..start + end];
        let mut insertions = 0usize;
        for line in retained.lines().filter(|line| line.contains("try_push_owned")) {
            insertions += 1;
            if !line.contains("if let Err(") {
                return false;
            }
        }
        insertions == 11 && !retained.contains("try_push_owned(candidate).is_err()")
    }

#[cfg(test)]
    #[test]
    fn fill_owned_page_handback_mutation_is_rejected() {
        let source = include_str!("../../🦀️.rs");
        assert!(fill_owned_page_handback_contract(source));
        let marker = "if let Err(candidate) = state.candidates.try_push_owned(candidate) {";
        let production_end = source.find("impl Drop for BoardFillJob").expect("live fill job drop");
        let index = source[..production_end].rfind(marker).expect("live scan-compatibility handback");
        let mut mutant = String::with_capacity(source.len());
        mutant.push_str(&source[..index]);
        mutant.push_str("if state.candidates.try_push_owned(candidate).is_err() {");
        mutant.push_str(&source[index + marker.len()..]);
        assert!(!fill_owned_page_handback_contract(&mutant));
    }

#[cfg(test)]
    /// 🧮️ The descriptor census is bounded by `BOARD_DESCRIPTOR_ITEM_CAPACITY`, not by the pointer payload
    /// credits: a hundred-placement puzzle 2d fill of an eleven-handle kind (1 200 entities, past the 1 024
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
            serde_json::json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": nodes, "edges": [] }).to_string()
        };
        let mut host = BoardHost::default();
        assert!(host.parse_fixture_json(&board(100, 11)), "1 200 entities must parse past the {BOARD_POINTER_ITEM_CAPACITY} pointer credits");
        let refused = BOARD_DESCRIPTOR_ITEM_CAPACITY / 12 + 1;
        assert!(!host.parse_fixture_json(&board(refused, 11)), "a descriptor past {BOARD_DESCRIPTOR_ITEM_CAPACITY} entities must be refused");
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
            serde_json::json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": rows, "edges": edges }).to_string()
        };
        let mut host = BoardHost::default();
        host.set_size(800, 600, 1.0);
        assert!(host.parse_fixture_json(&board(4)), "the opening parse must be admitted");
        host.interaction = Interaction::DragNodes { primary_id: "node-0".into(), offset: Vec2::ZERO, start_positions: [("node-0".to_string(), (0.0, 0.0))].into_iter().collect(), proximity_pair: None };
        let plan = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Up, x: 10.0, y: 5.0, shift: false, ctrl_or_meta: false, alt: false }).expect("finish drag plan");
        host.begin_pointer_commit(plan).expect("retained drag commit");
        drive_pointer_commit(&mut host);
        let mut publication = host.take_pointer_publication().expect("drag publication");
        for _ in 0..4096 {
            if publication.close_step() {
                break;
            }
        }
        assert!(host.parse_fixture_json(&board(4)), "the re-parse after a drag must be admitted");
        assert!(host.parse_fixture_json(&board(104)), "the re-parse after a hundred-placement fill must be admitted");
        host.set_selection_ids_silent(&["node-3".to_string()]);
        host.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        let mut turns = 0usize;
        while host.pending_delete_planning.is_some() || host.pending_delete_operation.is_some() {
            turns += 1;
            assert!(turns <= 1 << 18, "the delete never reached its terminal step");
            let step = with_board_step_context(1, live.clone(), |context| host.step_event_authority(context));
            assert_ne!(step, BoardAuthorityStep::Fault, "the delete must not fault on a fill-sized board");
            let _ = host.pop_owned_event();
        }
        assert!(!host.nodes.contains_key("node-3"), "the delete must remove its node");
        assert!(host.parse_fixture_json(&board(103)), "the re-parse after a delete must be admitted");
        assert!(host.parse_fixture_json(&board(103)), "a session may re-parse its board any number of times");
    }

#[cfg(test)]
    /// 🎥️ A document whose camera is SESSION state carries no `camera` key at all (puzzle 2d since its
    /// `setCamera` became a View-kind verb). Requiring one refused every shipped 2d example outright —
    /// `parse_fixture_json` returned false before it read a single node and all three panes stayed blank.
    /// The parse now keeps the camera the host is looking through and paints the document.
    #[test]
    fn a_fixture_without_a_camera_parses_and_keeps_the_session_camera() {
        let board = |camera: Option<serde_json::Value>| {
            let mut fixture = serde_json::json!({
                "schema": "puzzle.2d.fixture",
                "nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }],
                "edges": []
            });
            if let Some(camera) = camera {
                fixture["camera"] = camera;
            }
            fixture.to_string()
        };
        let mut host = BoardHost::default();
        host.set_size(800, 600, 1.0);
        assert!(host.parse_fixture_json(&board(Some(serde_json::json!({ "x": 12.0, "y": -3.0, "zoom": 2.0 })))), "a fixture that names its camera still parses");
        let framed = (host.camera.x, host.camera.y, host.camera.zoom);
        assert_eq!(framed, (12.0, -3.0, 2.0), "a named camera is adopted");
        assert!(host.parse_fixture_json(&board(None)), "a document with no camera key must parse, not refuse");
        assert!(host.nodes.contains_key("node-a"), "the cameraless document paints its nodes");
        assert_eq!((host.camera.x, host.camera.y, host.camera.zoom), framed, "the session keeps the camera it was looking through");
    }

#[cfg(test)]
    /// 🧱️ A REFUSED parse leaves the board exactly as it was. `parse_fixture_json` used to clear the
    /// scene before validating, so a malformed row (or a descriptor past its cap) emptied every pane and
    /// the refusal and "the board went blank" were the same event (2026-09-17 battery).
    #[test]
    fn a_refused_fixture_parse_leaves_the_painted_board_untouched() {
        let good = serde_json::json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }],
            "edges": []
        })
        .to_string();
        let mut host = BoardHost::default();
        assert!(host.parse_fixture_json(&good), "the opening parse must be admitted");
        for refused in [
            serde_json::json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-z", "x": 1.0, "y": 1.0, "shape": "circle" }], "edges": [] }),
            serde_json::json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-z", "x": 1.0, "y": 1.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-z:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0, "color": "not-a-color" }] }], "edges": [] }),
            serde_json::json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-z", "x": 1.0, "y": 1.0, "shape": "rectangle", "width": 0.0, "height": 4.0 }], "edges": [] }),
        ] {
            assert!(!host.parse_fixture_json(&refused.to_string()), "this fixture must be refused: {refused}");
            assert!(host.nodes.contains_key("node-a"), "a refused parse must not empty the board: {refused}");
            assert!(!host.nodes.contains_key("node-z"), "a refused parse must not half-commit its own rows: {refused}");
        }
        assert!(host.parse_fixture_json(&good), "the next real parse still lands");
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
        let board = serde_json::json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": nodes, "edges": edge_rows }).to_string();
        let mut host = BoardHost::default();
        assert!(host.parse_fixture_json(&board), "the edged board must parse into a fresh session");
        let events = host.drain_events_json();
        assert!(!events.contains("edgeCreate"), "a fixture parse must not announce the document's edges as creations: {events}");
        for round in 0..4 {
            assert!(host.parse_fixture_json(&board), "re-parse #{round} of the same edged board must parse in the same session without draining");
        }
        assert!(host.sync_descriptor(&SceneDescriptorJson::default()).is_ok(), "an authoring sync still runs after the parses");
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
        "schema": "puzzle.2d.fixture",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [node("node-a", -40.0, false), node("node-b", 40.0, false), node("node-locked", 0.0, true)],
        "edges": []
    })
    .to_string();
    let mut host = BoardHost::default();
    host.set_size(800, 600, 1.0);
    host.set_camera_silent(0.0, 0.0, 1.0);
    assert!(host.parse_fixture_json(&fixture), "the gumball fixture must parse");
    host
}

#[cfg(test)]
/// 🎯️ Arms the ring and returns the screen point `degrees` around it.
fn transform_ring_screen_at(host: &BoardHost, degrees: f64) -> Point {
    let (pivot, radius) = host.transform_gumball_geometry().expect("the rotate ring must be armed");
    let radians = degrees.to_radians();
    host.world_to_screen(Point::new(pivot.x + radius * radians.cos(), pivot.y + radius * radians.sin()))
}

#[cfg(test)]
fn board_event_names(json: &str) -> Vec<String> {
    serde_json::from_str::<Vec<serde_json::Value>>(json).expect("events parse").into_iter().filter_map(|row| row.get("name").and_then(serde_json::Value::as_str).map(str::to_string)).collect()
}

#[cfg(test)]
/// 🔄️ The whole rotate gesture is ONE document edit: the drag streams only TRANSIENT `transformPreview`
/// frames (the peer-pane mirror's food, dropped before dispatch) and the release publishes exactly one
/// `nodeRotate` carrying the absolute delta, the ids and the pivot. Streaming `nodeMove` rows here would
/// spend one of the store's 64 applied edits per frame.
#[test]
fn a_rotate_ring_drag_commits_exactly_one_node_rotate_event() {
    let mut host = transform_gumball_host();
    host.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
    let _ = host.drain_events_json();
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    assert!(host.transform_drag.is_some(), "a press on the ring must begin the gumball gesture");
    let quarter = transform_ring_screen_at(&host, 90.0);
    host.pointer_move_screen(quarter.x, quarter.y, false, false, false);
    let during = board_event_names(&host.drain_events_json());
    assert_eq!(during, vec!["transformPreview".to_string()], "a drag frame announces only its transient preview");
    host.pointer_up_screen(quarter.x, quarter.y, false, false, false);
    assert!(host.transform_drag.is_none(), "the release must end the gesture");
    let released = host.drain_events_json();
    assert_eq!(board_event_names(&released), vec!["nodeRotate".to_string()], "the release is one row: {released}");
    let row: Vec<serde_json::Value> = serde_json::from_str(&released).expect("events parse");
    let payload = row[0].get("payload").expect("payload");
    let radians = payload.get("radians").and_then(serde_json::Value::as_f64).expect("radians");
    assert!((radians - std::f64::consts::FRAC_PI_2).abs() < 1e-6, "the commit carries the absolute quarter turn, got {radians}");
    let ids: Vec<&str> = payload.get("ids").and_then(serde_json::Value::as_array).expect("ids").iter().filter_map(serde_json::Value::as_str).collect();
    assert_eq!(ids, vec!["node-a", "node-b"], "the commit names exactly the rotated members");
    let pivot = payload.get("pivot").expect("pivot");
    assert!(pivot.get("x").and_then(serde_json::Value::as_f64).expect("pivot x").abs() < 1e-9 && pivot.get("y").and_then(serde_json::Value::as_f64).expect("pivot y").abs() < 1e-9, "the pivot is the selection centroid");
}

#[cfg(test)]
/// 👁️ The live preview turns node CENTRES and handle ANGLES together, so edges keep their geometry
/// through the drag exactly as the guest reducer will recompute them on commit.
#[test]
fn the_rotate_preview_turns_positions_and_handle_angles_together() {
    let mut host = transform_gumball_host();
    host.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
    let angle_before = host.handles.get("node-a:v0").expect("handle").angle;
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let quarter = transform_ring_screen_at(&host, 90.0);
    host.pointer_move_screen(quarter.x, quarter.y, false, false, false);
    let a = host.nodes.get("node-a").expect("node-a");
    assert!(a.x.abs() < 1e-6 && (a.y + 40.0).abs() < 1e-6, "(-40,0) turned a quarter is (0,-40), got ({}, {})", a.x, a.y);
    let b = host.nodes.get("node-b").expect("node-b");
    assert!(b.x.abs() < 1e-6 && (b.y - 40.0).abs() < 1e-6, "(40,0) turned a quarter is (0,40), got ({}, {})", b.x, b.y);
    let angle_after = host.handles.get("node-a:v0").expect("handle").angle;
    assert!((angle_after - angle_before - std::f64::consts::FRAC_PI_2).abs() < 1e-6, "every handle angle turns with its node, got {angle_before} -> {angle_after}");
    let half = transform_ring_screen_at(&host, 0.0);
    host.pointer_move_screen(half.x, half.y, false, false, false);
    let a = host.nodes.get("node-a").expect("node-a");
    assert!((a.x + 40.0).abs() < 1e-6 && a.y.abs() < 1e-6, "the preview re-derives from the grab snapshot, it never accumulates: ({}, {})", a.x, a.y);
}

#[cfg(test)]
/// 🧲️ The grid-snap modifier quantizes the live rotation AND the committed delta to the same step.
#[test]
fn the_rotate_ring_snaps_under_the_grid_snap_modifier() {
    let mut host = transform_gumball_host();
    host.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
    host.set_grid_snap_enabled(true);
    let _ = host.drain_events_json();
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let nudged = transform_ring_screen_at(&host, 20.0);
    host.pointer_move_screen(nudged.x, nudged.y, false, false, false);
    let live = host.transform_drag.as_ref().expect("the gesture is live").radians;
    assert!((live - 15.0_f64.to_radians()).abs() < 1e-6, "20° snaps to the 15° step, got {}", live.to_degrees());
    host.pointer_up_screen(nudged.x, nudged.y, false, false, false);
    let released = host.drain_events_json();
    let rows: Vec<serde_json::Value> = serde_json::from_str(&released).expect("events parse");
    let radians = rows.iter().find(|row| row.get("name").and_then(serde_json::Value::as_str) == Some("nodeRotate")).and_then(|row| row.get("payload")).and_then(|payload| payload.get("radians")).and_then(serde_json::Value::as_f64).expect("nodeRotate radians");
    assert!((radians - 15.0_f64.to_radians()).abs() < 1e-6, "the commit carries the snapped delta, got {}", radians.to_degrees());
}

#[cfg(test)]
/// 🔒️ A locked member still counts toward the pivot (so the preview and the guest reducer agree on the
/// centroid) but never moves and never appears in the commit's id list.
#[test]
fn locked_members_hold_the_pivot_but_never_turn() {
    let mut host = transform_gumball_host();
    host.set_selection_ids_silent(&["node-a".into(), "node-b".into(), "node-locked".into()]);
    let _ = host.drain_events_json();
    let locked_before = (host.nodes.get("node-locked").expect("locked node").x, host.nodes.get("node-locked").expect("locked node").y);
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let quarter = transform_ring_screen_at(&host, 90.0);
    host.pointer_move_screen(quarter.x, quarter.y, false, false, false);
    let locked_after = (host.nodes.get("node-locked").expect("locked node").x, host.nodes.get("node-locked").expect("locked node").y);
    assert_eq!(locked_before, locked_after, "a locked node must not move with the ring");
    host.pointer_up_screen(quarter.x, quarter.y, false, false, false);
    let released = host.drain_events_json();
    let rows: Vec<serde_json::Value> = serde_json::from_str(&released).expect("events parse");
    let payload = rows.iter().find(|row| row.get("name").and_then(serde_json::Value::as_str) == Some("nodeRotate")).and_then(|row| row.get("payload")).expect("nodeRotate payload");
    let ids: Vec<&str> = payload.get("ids").and_then(serde_json::Value::as_array).expect("ids").iter().filter_map(serde_json::Value::as_str).collect();
    assert_eq!(ids, vec!["node-a", "node-b"], "the locked member stays out of the commit");
    let pivot_y = payload.get("pivot").and_then(|pivot| pivot.get("y")).and_then(serde_json::Value::as_f64).expect("pivot y");
    assert!(pivot_y.abs() < 1e-9, "all three centres average to y=0, so the locked member still holds the pivot, got {pivot_y}");
}

#[cfg(test)]
/// 🎯️ Hit-test priority: the ring outranks whatever node, handle or edge happens to sit under it. A
/// press on the band starts the rotation and leaves the selection exactly as it was.
#[test]
fn the_rotate_ring_outranks_the_nodes_and_handles_under_it() {
    let mut host = transform_gumball_host();
    host.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
    let (pivot, radius) = host.transform_gumball_geometry().expect("the ring is armed");
    let under_ring = serde_json::json!({
        "schema": "puzzle.2d.fixture",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [
            { "id": "node-a", "x": -40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },
            { "id": "node-b", "x": 40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-b:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },
            { "id": "node-under", "x": pivot.x + radius, "y": pivot.y, "shape": "circle", "radius": 24.0, "handles": [{ "id": "node-under:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }
        ],
        "edges": []
    })
    .to_string();
    assert!(host.parse_fixture_json(&under_ring), "the overlapping board must parse");
    host.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
    let _ = host.drain_events_json();
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    assert!(host.transform_drag.is_some(), "the ring wins the press");
    assert!(matches!(host.interaction, Interaction::None), "no node drag may start under the ring");
    assert_eq!(host.selection.iter().cloned().collect::<Vec<_>>(), vec!["node-a".to_string(), "node-b".to_string()], "grabbing the ring must not re-pick the node beneath it");
    let names = board_event_names(&host.drain_events_json());
    assert!(!names.iter().any(|name| name == "select"), "and it announces no selection change: {names:?}");
}

#[cfg(test)]
/// 🧾️ A whole gesture leaves the event terminal exactly as it found it: every preview frame is either
/// published or dropped outright, never left holding a claim, and nothing faults.
#[test]
fn a_rotate_gesture_leaves_the_event_credits_untouched() {
    let mut host = transform_gumball_host();
    host.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
    let _ = host.drain_events_json();
    assert_eq!((host.events.claimed_items, host.events.claimed_bytes), (0, 0), "the queue starts unclaimed");
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    for step in 1..=64 {
        let point = transform_ring_screen_at(&host, f64::from(step));
        host.pointer_move_screen(point.x, point.y, false, false, false);
    }
    let last = transform_ring_screen_at(&host, 64.0);
    host.pointer_up_screen(last.x, last.y, false, false, false);
    assert_eq!((host.events.claimed_items, host.events.claimed_bytes), (0, 0), "no frame may strand a claim");
    assert!(!host.event_terminal_faulted(), "a fast drag must never fault the event terminal");
    let _ = host.drain_events_json();
    assert!(host.events.terminal_is_empty(), "the drained queue is terminal-empty");
}

#[cfg(test)]
/// ↩️ Escape abandons the ring and restores the exact pre-gesture geometry without touching the document.
#[test]
fn escape_cancels_the_rotate_ring_and_restores_the_geometry() {
    let mut host = transform_gumball_host();
    host.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
    let _ = host.drain_events_json();
    let before = (host.nodes.get("node-a").expect("node-a").x, host.nodes.get("node-a").expect("node-a").y, host.handles.get("node-a:v0").expect("handle").angle);
    let grab = transform_ring_screen_at(&host, 0.0);
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    let quarter = transform_ring_screen_at(&host, 90.0);
    host.pointer_move_screen(quarter.x, quarter.y, false, false, false);
    assert!(host.cancel_area_select(), "escape must claim the live ring gesture");
    assert!(host.transform_drag.is_none(), "and end it");
    let after = (host.nodes.get("node-a").expect("node-a").x, host.nodes.get("node-a").expect("node-a").y, host.handles.get("node-a:v0").expect("handle").angle);
    assert_eq!(before, after, "a cancel restores position AND handle angle");
    let names = board_event_names(&host.drain_events_json());
    assert!(!names.iter().any(|name| name == "nodeRotate"), "a cancelled gesture commits nothing: {names:?}");
}

#[cfg(test)]
/// 🕹️ The ring is drawn and armed ONLY while the select utility composes `rotate` and the selection
/// holds a node — the brush utility and a rotate-off gumball leave the press to the ordinary hit test.
#[test]
fn the_rotate_ring_is_armed_only_when_the_flags_allow_it() {
    let mut host = transform_gumball_host();
    host.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
    let grab = transform_ring_screen_at(&host, 0.0);
    host.set_transform_flags(true, false);
    assert!(host.transform_gumball_geometry().is_none(), "rotate off disarms the ring");
    assert!(host.transform_gumball_json().contains("\"ringVisible\":false"), "and the vitals say so: {}", host.transform_gumball_json());
    host.pointer_down_screen(grab.x, grab.y, 0, false, false);
    assert!(host.transform_drag.is_none(), "a disarmed ring must not swallow the press");
    host.pointer_up_screen(grab.x, grab.y, false, false, false);
    host.set_transform_flags(true, true);
    host.set_active_utility("brush");
    assert!(host.transform_gumball_geometry().is_none(), "the brush utility owns the pointer, not the gumball");
    host.set_active_utility("select");
    host.set_selection_ids_silent(&[]);
    assert!(host.transform_gumball_geometry().is_none(), "an empty selection has nothing to turn");
    host.set_selection_ids_silent(&["node-a".into()]);
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
    let json = host.handle_positions_json();
    assert!(json.contains("\"total\":3"), "the document's whole handle count is always named: {json}");
    assert!(json.contains("\"onScreen\":3") && json.contains("\"published\":3") && json.contains("\"capped\":false"), "three handles fit the 800×600 viewport uncapped: {json}");
    for id in ["node-a:v0", "node-b:v0", "node-locked:v0"] {
        assert!(json.contains(&format!("[\"{id}\",")), "{id} must be named with its world position: {json}");
    }
    assert_eq!(json.matches("\"b-l\",true").count(), 3, "an edgeless document leaves every handle open: {json}");
    let row = host.handle_positions_json();
    let node_a = host.nodes.get("node-a").expect("node-a");
    assert!(row.contains(&format!("[\"node-a:v0\",{},", node_a.x + 10.0)), "the position is the handle's own world point, not its node's: {row}");
    host.set_camera_silent(100_000.0, 100_000.0, 1.0);
    let away = host.handle_positions_json();
    assert!(away.contains("\"total\":3") && away.contains("\"onScreen\":0") && away.contains("\"rows\":[]"), "a camera that left the document publishes no aimable handle: {away}");
}
//#endregion 🩺️HandleVitals

//#region 🎯️TargetRegions
#[cfg(test)]
/// 🎯️ One node at the origin sitting INSIDE a free region, plus a locked and a hidden region well
/// clear of it. 800×600 at zoom 1 and camera (0,0), so `world_to_screen` is a plain centre offset.
fn region_host() -> BoardHost {
    let fixture = serde_json::json!({
        "schema": "puzzle.2d.fixture",
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
    host.set_size(800, 600, 1.0);
    host.set_camera_silent(0.0, 0.0, 1.0);
    assert!(host.parse_fixture_json(&fixture), "the region fixture must parse");
    host
}

#[cfg(test)]
fn region_press(host: &mut BoardHost, x: f64, y: f64) {
    let screen = host.world_to_screen(Point::new(x, y));
    host.pointer_down_screen(screen.x, screen.y, 0, false, false);
}

#[cfg(test)]
fn region_move_to(host: &mut BoardHost, x: f64, y: f64) {
    let screen = host.world_to_screen(Point::new(x, y));
    host.pointer_move_screen(screen.x, screen.y, false, false, false);
}

#[cfg(test)]
fn region_release_at(host: &mut BoardHost, x: f64, y: f64) {
    let screen = host.world_to_screen(Point::new(x, y));
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
    assert_eq!(host.regions.len(), 3, "all three document rows reach the engine");
    let published = host.target_regions_json();
    assert!(published.contains("\"id\":\"region-a\"") && published.contains("\"x\":-60") && published.contains("\"width\":120"), "the probe vital carries normalized bounds: {published}");
    region_press(&mut host, 0.0, 0.0);
    assert!(host.region_drag.is_none(), "the node under the region wins the press");
    assert!(matches!(host.interaction, Interaction::DragNodes { .. }), "and it starts an ordinary node drag");
    let mut host = region_host();
    region_press(&mut host, -50.0, 0.0);
    assert!(host.region_drag.is_some(), "a press inside the region but clear of the node grabs the region");
    assert_eq!(host.selection.iter().cloned().collect::<Vec<_>>(), vec!["region-a".to_string()], "and selects it at `region` granularity");
    assert!(matches!(host.interaction, Interaction::None), "no marquee may start under a grabbed region");
}

#[cfg(test)]
/// 🖍️ A click-drag with the area brush armed commits exactly ONE `regionCreate`, on release — the
/// drag itself publishes nothing, so painting a region is one applied edit of the store's 64.
#[test]
fn an_area_brush_drag_commits_exactly_one_region_create() {
    let mut host = region_host();
    host.set_active_utility("areaBrush");
    let _ = host.drain_events_json();
    region_press(&mut host, -200.0, -150.0);
    assert!(host.region_paint.is_some(), "the press anchors the brush rectangle");
    region_move_to(&mut host, -140.0, -70.0);
    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a paint frame announces nothing");
    region_release_at(&mut host, -140.0, -70.0);
    assert!(host.region_paint.is_none(), "the release ends the gesture");
    let released = host.drain_events_json();
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
    host.set_area_brush_extent(30.0, 20.0);
    let _ = host.drain_events_json();
    region_press(&mut host, 100.0, 100.0);
    region_release_at(&mut host, 100.0, 100.0);
    let payload = region_event_payloads(&host.drain_events_json(), "regionCreate").remove(0);
    assert_eq!(payload.get("width").and_then(serde_json::Value::as_f64), Some(30.0), "a click paints the brush extent: {payload}");
    assert_eq!(payload.get("height").and_then(serde_json::Value::as_f64), Some(20.0));
    host.set_area_brush_extent(f64::NAN, -5.0);
    assert_eq!(host.area_brush_extent(), (30.0, 20.0), "a non-finite or non-positive axis never collapses the brush");
    region_press(&mut host, 0.0, 200.0);
    region_move_to(&mut host, 0.5, 200.5);
    region_release_at(&mut host, 0.5, 200.5);
    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a rectangle under the extent floor commits nothing");
}

#[cfg(test)]
/// 🚚️ A body drag is ONE `regionMove` carrying the new minimum corner; a grip drag is ONE
/// `regionResize` carrying corner AND extent, because a west/north grip moves both.
#[test]
fn region_body_and_grip_drags_commit_exactly_one_event_each() {
    let mut host = region_host();
    let _ = host.drain_events_json();
    region_press(&mut host, -50.0, 0.0);
    let _ = host.drain_events_json();
    region_move_to(&mut host, -30.0, 10.0);
    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a region drag frame announces nothing");
    region_release_at(&mut host, -30.0, 10.0);
    let moved = host.drain_events_json();
    assert_eq!(board_event_names(&moved), vec!["regionMove".to_string()], "one row: {moved}");
    let payload = region_event_payloads(&moved, "regionMove").remove(0);
    assert_eq!(payload.get("x").and_then(serde_json::Value::as_f64), Some(-40.0), "the body drag translated by (+20,+10): {payload}");
    assert_eq!(payload.get("y").and_then(serde_json::Value::as_f64), Some(-50.0));
    assert!(payload.get("width").is_none(), "a move never re-states an extent it did not touch: {payload}");

    let mut host = region_host();
    let _ = host.drain_events_json();
    region_press(&mut host, 60.0, 60.0);
    assert!(host.region_drag.as_ref().is_some_and(|drag| drag.grip == RegionGrip::SouthEast), "the bottom-right corner is a resize grip");
    let _ = host.drain_events_json();
    region_move_to(&mut host, 80.0, 80.0);
    region_release_at(&mut host, 80.0, 80.0);
    let resized = host.drain_events_json();
    assert_eq!(board_event_names(&resized), vec!["regionResize".to_string()], "one row: {resized}");
    let payload = region_event_payloads(&resized, "regionResize").remove(0);
    assert_eq!(payload.get("x").and_then(serde_json::Value::as_f64), Some(-60.0), "the far corner never moved: {payload}");
    assert_eq!(payload.get("width").and_then(serde_json::Value::as_f64), Some(140.0), "and the extent grew by the drag: {payload}");
    assert_eq!(payload.get("height").and_then(serde_json::Value::as_f64), Some(140.0));

    let mut host = region_host();
    let _ = host.drain_events_json();
    region_press(&mut host, -50.0, 0.0);
    let _ = host.drain_events_json();
    region_release_at(&mut host, -50.0, 0.0);
    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a release that moved nothing commits nothing");
}

#[cfg(test)]
/// 🧲️ Region gestures quantize to the same visible grid step the node drag does, and only while the
/// grid-snap modifier is on.
#[test]
fn region_gestures_snap_only_under_the_grid_snap_modifier() {
    let mut host = region_host();
    host.set_grid_snap_enabled(true);
    let step = host.region_snap_step().expect("the normal LOD offers a snap step");
    host.set_active_utility("areaBrush");
    let _ = host.drain_events_json();
    region_press(&mut host, -203.0, -147.0);
    region_move_to(&mut host, -100.0, -20.0);
    region_release_at(&mut host, -100.0, -20.0);
    let payload = region_event_payloads(&host.drain_events_json(), "regionCreate").remove(0);
    let x = payload.get("x").and_then(serde_json::Value::as_f64).expect("x");
    let y = payload.get("y").and_then(serde_json::Value::as_f64).expect("y");
    assert!((x / step - (x / step).round()).abs() < 1e-9 && (y / step - (y / step).round()).abs() < 1e-9, "a snapped paint lands on the grid, got ({x}, {y}) against step {step}");

    let mut host = region_host();
    host.set_active_utility("areaBrush");
    let _ = host.drain_events_json();
    region_press(&mut host, -203.0, -147.0);
    region_move_to(&mut host, -100.0, -20.0);
    region_release_at(&mut host, -100.0, -20.0);
    let payload = region_event_payloads(&host.drain_events_json(), "regionCreate").remove(0);
    assert_eq!(payload.get("x").and_then(serde_json::Value::as_f64), Some(-203.0), "without the modifier the exact pointer rectangle is committed: {payload}");
}

#[cfg(test)]
/// 🔏️ A locked region still paints and still selects — it simply refuses every drag, so neither a
/// `regionMove` nor a `regionResize` can ever name it.
#[test]
fn a_locked_region_refuses_every_drag() {
    let mut host = region_host();
    let _ = host.drain_events_json();
    region_press(&mut host, 240.0, 0.0);
    assert!(host.region_drag.is_none(), "a locked region is never grabbed");
    assert_eq!(host.selection.iter().cloned().collect::<Vec<_>>(), vec!["region-locked".to_string()], "but it still selects, so the inspector can unlock it");
    let _ = host.drain_events_json();
    region_move_to(&mut host, 300.0, 60.0);
    region_release_at(&mut host, 300.0, 60.0);
    let names = board_event_names(&host.drain_events_json());
    assert!(!names.iter().any(|name| name.starts_with("region")), "a locked region publishes no geometry row: {names:?}");
    let region = host.regions.get("region-locked").expect("region-locked");
    assert!((region.x - 200.0).abs() < 1e-9 && (region.y + 40.0).abs() < 1e-9, "and its rectangle never moved");
}

#[cfg(test)]
/// 🙈️ A hidden region is not paint, so it is not a pick target either: a press inside its rectangle
/// falls straight through to the background marquee.
#[test]
fn a_hidden_region_is_neither_hit_tested_nor_pickable() {
    let mut host = region_host();
    region_press(&mut host, -260.0, 0.0);
    assert!(host.region_drag.is_none(), "a hidden region refuses the grab");
    assert!(matches!(host.interaction, Interaction::SelectionPending { .. }), "the press reaches the background instead");
    let targets = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-260.0, 0.0)).x, host.world_to_screen(Point::new(-260.0, 0.0)).y);
    assert!(!targets.contains("region-hidden"), "and it is not offered as a pick target: {targets}");
    let visible = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-50.0, 0.0)).x, host.world_to_screen(Point::new(-50.0, 0.0)).y);
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
    let overflowing = serde_json::json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 20.0, "handles": [] }], "edges": [], "targetRegions": rows }).to_string();
    assert!(!host.parse_fixture_json(&overflowing), "one node plus a full census of regions overruns the descriptor ceiling");
    assert_eq!(host.regions.len(), 3, "and the refused parse leaves the live board exactly as it was");

    let mut host = region_host();
    host.set_active_utility("areaBrush");
    let _ = host.drain_events_json();
    region_press(&mut host, -200.0, -150.0);
    region_move_to(&mut host, -140.0, -70.0);
    region_release_at(&mut host, -140.0, -70.0);
    let _ = host.drain_events_json();
    assert!(host.event_authority_terminal_is_empty(), "a whole paint gesture leaves the event terminal as it found it");
    assert!(!host.event_schema_fault, "and nothing faulted");
}
//#endregion 🎯️TargetRegions
