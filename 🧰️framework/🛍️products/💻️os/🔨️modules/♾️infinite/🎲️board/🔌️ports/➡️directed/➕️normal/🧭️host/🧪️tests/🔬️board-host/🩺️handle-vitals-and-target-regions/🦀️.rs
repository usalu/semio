// 🩺️ Handle vitals and target regions: the on-screen handle vitals and the target region ingest, area brush, region drag, snap, lock, hidden and census laws.

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
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&fixture,&mut control)}, "the region fixture must parse");
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
    assert!(!{let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&overflowing,&mut control)}, "one node plus a full census of regions overruns the descriptor ceiling");
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
