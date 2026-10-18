/// 🎞️ Fixture reparse and transform gumball: the whole editing session reparse, camera-less and refused fixtures, announced edges and the rotate ring gumball laws.

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
