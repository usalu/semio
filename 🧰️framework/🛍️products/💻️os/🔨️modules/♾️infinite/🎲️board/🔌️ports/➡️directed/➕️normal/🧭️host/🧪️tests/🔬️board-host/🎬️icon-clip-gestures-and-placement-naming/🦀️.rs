/// 🎬️ Icon clip, gestures and placement naming: the icon clip centres, board edge cap start, node and group drag gesture records, draft references and the brush and wire placement naming.

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
        match semio_framework_job::close_with_self_funded_context(exact, |cx| job.close_step(cx)).expect("original fixture close turn") {
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
