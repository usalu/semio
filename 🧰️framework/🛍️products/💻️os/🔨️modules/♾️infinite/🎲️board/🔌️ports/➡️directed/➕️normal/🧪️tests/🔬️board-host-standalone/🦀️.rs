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
        assert_ne!(host.nodes.get("node-a").map(|node| (node.x, node.y)), before);
        assert_eq!(with_board_step_context(1, live.clone(), |context| host.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, live, |context| host.step_pointer_commit(context)), BoardAuthorityStep::Complete);
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
        let mut close_turns = 1usize;
        while !with_board_step_context(1, cancel.clone(), |context| interrupted.close_event_authority_step(context)) {
            close_turns += 1;
            assert!(close_turns <= 32);
        }
        assert!(close_turns > 4);
        assert!(interrupted.event_authority_terminal_is_empty());
    }

#[cfg(test)]
    #[test]
    fn delete_property_pre_admission_rejects_hostile_nodes_without_transfer_or_mutation() {
        let mut host = deletion_fixture("node-a");
        host.nodes.get_mut("node-a").unwrap().properties.insert("hostile".into(), graph::manifest::PropertyValue::Array((0..=BOARD_POINTER_ITEM_CAPACITY).map(|_| graph::manifest::PropertyValue::Null).collect()));
        host.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        for _ in 0..1024 {
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
