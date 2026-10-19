/// 🗑️ Snapshot JSON, delete plans and retirement: the board snapshot JSON vectors, the delete plan FIFO and credit laws, property pre-admission and derivation, removed entity retirement, scene retirement, the non-opaque close and descriptor admission.

#[cfg(test)]
    #[test]
    fn board_snapshot_json_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🔣️board-ingress.json")).unwrap();
        for fixture in vectors.as_array().unwrap() {
            let mut host = BoardHost::default();
            assert!({let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&fixture.to_string(),&mut control)});
            assert_eq!(host.scene.nodes.len(), fixture["nodes"].as_array().unwrap().len());
            for node in fixture["nodes"].as_array().unwrap() {
                let actual = &host.scene.nodes[node["id"].as_str().unwrap()];
                assert_eq!(actual.x, node["x"].as_f64().unwrap());
                assert_eq!(actual.y, node["y"].as_f64().unwrap());
            }
            let count = host.scene.nodes.len();
            assert!(!{let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,"{",&mut control)});
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
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&fixture.to_string(),&mut control)});
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
        assert!({let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(100, 11),&mut control)}, "1 200 entities must parse past the {BOARD_POINTER_ITEM_CAPACITY} pointer credits");
        let refused = BOARD_DESCRIPTOR_ITEM_CAPACITY / 12 + 1;
        assert!(!{let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board(refused, 11),&mut control)}, "a descriptor past {BOARD_DESCRIPTOR_ITEM_CAPACITY} entities must be refused");
    }
