// 🧩️ Engine parse and handle suggestion laws: the board engine parse of the painted fixtures, the deleted handle laws, the hover scene and the suggestion popup laws.

//#region 🔖️EngineParse
/// 🎲️ Every shipped example must parse in the board engine as the host paints it: a `false` from
/// `load_board_snapshot_json` is silent at runtime and leaves the panes empty (Nakagin, 2026-09-16).
#[semio_framework_async_macros::async_test]
async fn shipped_examples_parse_in_the_board_engine() {
    for (name, json) in [("concrete-forest", concrete_forest_example_json()), ("nakagin", nakagin_example_json())] {
        let snapshot: Value = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("example json");
        let mut host = BoardHost::default();
        let parsed = {let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);semio_framework_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&snapshot.to_string(),&mut control)};
        if !parsed {
            let nodes = board_snapshot_nodes(&snapshot).to_vec();
            let edges = board_snapshot_edges(&snapshot).to_vec();
            let probe = |nodes: &[Value], edges: &[Value]| {let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);semio_framework_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut BoardHost::default(),&json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0, "y": 0, "zoom": 1 }, "nodes": nodes, "edges": edges }).to_string(),&mut control)};
            let bad_node = (1..=nodes.len()).find(|count| !probe(&nodes[..*count], &[])).map(|count| nodes[count - 1].clone());
            let bad_edge = (1..=edges.len()).find(|count| !probe(&nodes, &edges[..*count])).map(|count| edges[count - 1].clone());
            panic!("{name}: the board engine refused the example snapshot; first refused node = {bad_node:?}; first refused edge = {bad_edge:?}");
        }
    }
}

/// 🎯️ What the host paints after a gesture: the overview scene's snapshot lane, re-parsed by the
/// engine, with the first refused node/edge named when it refuses (the runtime only logs a length).
fn painted_fixture_parses(app: &mut Puzzle2dApp, what: &str) {
    let body: Value = semio_framework_pack_json::from_json_str(&render_body(app, overview::BODY_KEY), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("overview body json");
    let snapshot_json = body.get("board2d").and_then(|scene| scene.get("snapshotJson")).and_then(Value::as_str).expect("painted snapshot lane").to_string();
    if {let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);semio_framework_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut BoardHost::default(),&snapshot_json,&mut control)} {
        return;
    }
    let snapshot: Value = semio_framework_pack_json::from_json_str(&snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("painted snapshot json");
    let nodes = board_snapshot_nodes(&snapshot).to_vec();
    let edges = board_snapshot_edges(&snapshot).to_vec();
    let probe = |nodes: &[Value], edges: &[Value]| {let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);semio_framework_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut BoardHost::default(),&json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0, "y": 0, "zoom": 1 }, "nodes": nodes, "edges": edges }).to_string(),&mut control)};
    let bad_node = (1..=nodes.len()).find(|count| !probe(&nodes[..*count], &[])).map(|count| nodes[count - 1].clone());
    let bad_edge = (1..=edges.len()).find(|count| !probe(&nodes, &edges[..*count])).map(|count| edges[count - 1].clone());
    let head: String = snapshot_json.chars().take(400).collect();
    panic!("{what}: the board engine refused the painted snapshot ({} chars); first refused node = {bad_node:?}; first refused edge = {bad_edge:?}; head = {head}", snapshot_json.len());
}

/// 🖱️ A node drag (one `gesture` record through `applyBoardEvents`) must leave a snapshot the engine still
/// paints — after the store round-trip, not just in the scene the command patched (2026-09-17: three
/// blank panes after every drag on Nakagin).
#[semio_framework_async_macros::async_test]
async fn dragging_a_node_keeps_the_painted_board_parseable() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    painted_fixture_parses(&mut app, "before drag");
    let node = board_snapshot_nodes(&fixture_of(&app))[0].clone();
    let id = node.get("id").and_then(Value::as_str).expect("node id").to_string();
    let x = node.get("x").and_then(Value::as_f64).expect("x") + 8.0;
    let y = node.get("y").and_then(Value::as_f64).expect("y") + 4.0;
    let events = json!([{ "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "drag", "targets": [id], "dx": 8.0, "dy": 4.0, "proximity": [] } }]).to_string();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": events })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID));
    assert!(result.is_ok(), "applyBoardEvents must not fault: {:?}", result.err());
    let moved = board_snapshot_nodes(&fixture_of(&app)).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id.as_str())).cloned().expect("moved node");
    assert_eq!(moved.get("x").and_then(Value::as_f64), Some(x), "the drag must commit the new x");
    painted_fixture_parses(&mut app, "after drag");
    close_app(&mut app);
}


/// 🕹️ A board `select` row (the engine's click echo, through `applyBoardEvents`) is the selection the
/// inspector and the painted board both show — the write lands before either body renders.
#[semio_framework_async_macros::async_test]
async fn board_select_row_reaches_the_inspector_and_the_board() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let id = first_node_id(&app);
    let events = json!([{ "name": "hover", "payload": { "id": id } }, { "name": "select", "payload": { "ids": [id], "gesture": "click" } }]).to_string();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": events })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID));
    assert!(result.is_ok(), "applyBoardEvents must not fault: {:?}", result.err());
    let board = render_body(&mut app, overview::BODY_KEY);
    let inspector = render_body(&mut app, crate::editor::puzzle2d::config::PUZZLE2D_PLAY_BODY_PROPERTIES);
    close_app(&mut app);
    assert!(board.contains(&id), "the painted board must carry the selected id: {}", &board[..board.len().min(400)]);
    assert!(inspector.contains(&id) && inspector.contains("puzzle2d-play-inspector.node.id"), "the inspector must show the selected node's fields: {}", &inspector[..inspector.len().min(600)]);
}
//#endregion 🔖️EngineParse

/// 🎲️ Deleting a handle that carries an edge must leave a board the engine still paints — a dangling
/// edge makes `load_board_snapshot_json` refuse the WHOLE document (three blank panes, 2026-09-16).
#[semio_framework_async_macros::async_test]
async fn deleting_an_edged_handle_keeps_the_board_parseable() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let before = fixture_of(&app);
    let edge = board_snapshot_edges(&before)[0].clone();
    let handle_id = edge.get("source").and_then(Value::as_str).expect("edge source").to_string();
    let edge_id = edge.get("id").and_then(Value::as_str).expect("edge id").to_string();
    select_id(&mut app, PUZZLE2D_GRANULARITY_HANDLE, &handle_id).expect("select handle");
    dispatch(&mut app, "deleteSelection", None, None).expect("delete");
    let after = fixture_of(&app);
    assert!(!board_snapshot_edges(&after).iter().any(|edge| edge.get("id").and_then(Value::as_str) == Some(edge_id.as_str())), "the edge on the deleted handle must go with it");
    let mut host = BoardHost::default();
    let parsed = {let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024*1024,&mut accepted);semio_framework_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&after.to_string(),&mut control)};
    close_app(&mut app);
    assert!(parsed, "the board engine must still parse the document after a handle delete");
}

/// 🗑️ Deleting an edged node commits through the store (its inverse is one `create-node` plus one
/// `connect-handles` per edge — the fold footprint must admit that cascade) and empties the selection.
#[semio_framework_async_macros::async_test]
async fn deleting_an_edged_node_commits_and_clears_the_selection() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let before = fixture_of(&app);
    let edge = board_snapshot_edges(&before)[0].clone();
    let handle_id = edge.get("source").and_then(Value::as_str).expect("edge source");
    let node_id = handle_id.split(':').next().expect("handle id carries its node id").to_string();
    assert!(board_snapshot_nodes(&before).iter().any(|node| node.get("id").and_then(Value::as_str) == Some(node_id.as_str())), "edge source node must exist");
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &node_id).expect("select node");
    let result = dispatch(&mut app, "deleteSelection", None, None);
    let after = fixture_of(&app);
    let node_gone = !board_snapshot_nodes(&after).iter().any(|node| node.get("id").and_then(Value::as_str) == Some(node_id.as_str()));
    let edge_gone = !board_snapshot_edges(&after).iter().any(|entry| entry.get("id") == edge.get("id"));
    let selection = render_body(&mut app, overview::BODY_KEY).contains(&node_id);
    close_app(&mut app);
    assert!(result.is_ok(), "deleteSelection must not fault: {:?}", result.err());
    assert!(node_gone && edge_gone, "the node and the edge hanging off it must be gone (node gone={node_gone}, edge gone={edge_gone})");
    assert!(!selection, "the deleted node must leave the painted selection");
}

//#region 🔖️HandleSuggestions
/// 🖌️ A handle no edge names on either end — the only kind the brush slot can grow a node onto.
fn first_free_handle_id(snapshot: &Value) -> Option<String> {
    let used: std::collections::HashSet<&str> = board_snapshot_edges(snapshot)
        .iter()
        .flat_map(|edge| [edge.get("source").and_then(Value::as_str), edge.get("target").and_then(Value::as_str)])
        .flatten()
        .collect();
    board_snapshot_nodes(snapshot)
        .iter()
        .filter_map(|node| node.get("handles").and_then(Value::as_array))
        .flatten()
        .filter_map(|handle| handle.get("id").and_then(Value::as_str))
        .find(|id| !used.contains(id))
        .map(str::to_string)
}

/// 🐁️ LAW: the framework-owned `"pointer"` hover reaches the board scene for EVERY granularity and in
/// every pane — the gap the 09-17 audit measured as `hovered_id: None` hardcoded at the scene builder.
/// Read off Nakagin: concrete-forest is a one-node SEED with no edge, so it cannot name an edge to hover.
#[test]
fn hover_id_reaches_the_board_scene_for_every_granularity_and_pane() {
    let snapshot = crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE.document_json().to_string();
    let snapshot: Value = semio_framework_pack_json::from_json_str(&snapshot, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("nakagin json");
    let node_id = board_snapshot_nodes(&snapshot)[0].get("id").and_then(Value::as_str).expect("node id").to_string();
    let handle_id = board_snapshot_nodes(&snapshot).iter().filter_map(|node| node.get("handles").and_then(Value::as_array)).flatten().filter_map(|handle| handle.get("id").and_then(Value::as_str)).next().expect("handle id").to_string();
    let edge_id = board_snapshot_edges(&snapshot)[0].get("id").and_then(Value::as_str).expect("edge id").to_string();
    for (granularity, hovered) in [(PUZZLE2D_GRANULARITY_NODE, &node_id), (PUZZLE2D_GRANULARITY_HANDLE, &handle_id), (PUZZLE2D_GRANULARITY_EDGE, &edge_id)] {
        let interaction = Puzzle2dInteractionSnapshot { granularity: granularity.into(), hovered: vec![hovered.clone()], ..Default::default() };
        assert_eq!(interaction.hovered_id().as_deref(), Some(hovered.as_str()), "{granularity} hover must resolve an id");
        let envelope = Puzzle2dScene { board_snapshot: snapshot.clone(), runtime: Default::default(), active_utility: "select".into(), interaction };
        for pane in PUZZLE2D_PANES {
            let scene = edit::canvas::puzzle2d_board_scene("{}", &envelope, pane);
            assert_eq!(scene.hovered_id.as_deref(), Some(hovered.as_str()), "{granularity} hover must reach the {pane} board scene");
            assert_eq!(scene.domain_id.as_deref(), Some(PUZZLE2D_INTERACTION_DOMAIN), "every pane must name the interaction domain its hover publishes on");
        }
    }
    let idle = Puzzle2dScene { board_snapshot: snapshot, runtime: Default::default(), active_utility: "select".into(), interaction: Puzzle2dInteractionSnapshot::default() };
    assert!(edit::canvas::puzzle2d_board_scene("{}", &idle, crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID).hovered_id.is_none(), "no hover must paint no hover");
}

/// 💡️ LAW: the popup rides the board scene with the SAME candidate page the armed brush cycles, and the
/// previewed index is the shared slot index — one mechanism, not two.
#[test]
fn suggestion_popup_publishes_the_shared_candidate_page_and_the_previewed_index() {
    let snapshot: Value = semio_framework_pack_json::from_json_str(&crate::examples::puzzle2d::concrete_forest::SOURCE.document_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("concrete forest json");
    let handle_id = first_free_handle_id(&snapshot).expect("concrete forest offers a free handle");
    let runtime = crate::editor::puzzle2d::config::Puzzle2dPlayRuntime {
        suggestion_menu: Some(crate::editor::puzzle2d::config::Puzzle2dSuggestionMenu { x: 12.0, y: 34.0, window_id: crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID.into(), handle_id: handle_id.clone() }),
        brush_candidate_source_handle_id: handle_id.clone(),
        brush_candidate_index: 1,
        brush_candidates: vec![semio_framework_value::ToValue::to_value(&json!({ "nodeKind": "alpha", "targetHandleIndex": 0 })), semio_framework_value::ToValue::to_value(&json!({ "nodeKind": "beta", "targetHandleIndex": 2 }))],
        ..Default::default()
    };
    let envelope = Puzzle2dScene { board_snapshot: snapshot, runtime, active_utility: "select".into(), interaction: Puzzle2dInteractionSnapshot::default() };
    let scene = edit::canvas::puzzle2d_board_scene("{}", &envelope, crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
    let menu: Value = semio_framework_pack_json::from_json_str(&scene.suggestion_menu_json.expect("an open popup must reach the client"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("menu json");
    assert_eq!(menu["open"], json!(true));
    assert_eq!((menu["x"].as_f64(), menu["y"].as_f64()), (Some(12.0), Some(34.0)));
    assert_eq!(menu["handleId"].as_str(), Some(handle_id.as_str()));
    assert_eq!(menu["hoveredIndex"].as_u64(), Some(1), "the previewed row is the shared slot index");
    assert_eq!(menu["pending"], json!(false), "a slot that resolved its source handle is not pending");
    let candidates = menu["candidates"].as_array().expect("candidate rows");
    assert_eq!(candidates.len(), 2);
    assert_eq!(candidates[1]["nodeLabel"].as_str(), Some("beta"));
    assert_eq!(candidates[1]["handleLabel"].as_str(), Some("handle 2"));
}

/// 💡️ LAW: a closed popup publishes nothing, so no pane renders a stale menu.
#[test]
fn a_closed_suggestion_popup_publishes_no_menu() {
    let snapshot: Value = semio_framework_pack_json::from_json_str(&crate::examples::puzzle2d::concrete_forest::SOURCE.document_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("concrete forest json");
    let envelope = Puzzle2dScene { board_snapshot: snapshot, runtime: Default::default(), active_utility: "select".into(), interaction: Puzzle2dInteractionSnapshot::default() };
    assert!(edit::canvas::puzzle2d_board_scene("{}", &envelope, crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID).suggestion_menu_json.is_none());
}

/// 🏷️ LAW: every board pane publishes the plugin's placement naming, so the os wgpu host and the React session mint the
/// same `puzzle2d.brush.<serial>` ids as the plugin's own boards.
#[test]
fn board_scene_publishes_the_plugin_placement_naming() {
    let snapshot: Value = semio_framework_pack_json::from_json_str(&crate::examples::puzzle2d::concrete_forest::SOURCE.document_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("concrete forest json");
    let envelope = Puzzle2dScene { board_snapshot: snapshot, runtime: Default::default(), active_utility: "select".into(), interaction: Puzzle2dInteractionSnapshot::default() };
    assert_eq!(edit::canvas::puzzle2d_board_scene("{}", &envelope, crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID).placement_naming.as_deref(), Some(r#"{"brushNodePrefix":"puzzle2d.brush.","brushEdgePrefix":"puzzle2d.brush.edge."}"#));
}

/// 💡️ LAW: the context menu offers the popup on ONE selected handle and on nothing else — the entry
/// point the brush slot never had (it was reachable only with the brush armed).
#[semio_framework_async_macros::async_test]
async fn context_menu_offers_suggest_nodes_on_one_selected_handle_only() {
    use semio_framework_plugin::ContextMenuRequest;
    use semio_framework_plugin::ContextMenuSelectionGroup;
    use semio_framework_plugin::ContextMenuSurfaceTarget;
    use semio_framework_plugin::UiMenuRef;
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let snapshot = fixture_of(&app);
    let handle_id = first_free_handle_id(&snapshot).expect("concrete forest offers a free handle");
    let node_id = first_node_id(&app);
    let menu_for = |app: &mut Puzzle2dApp, ids: Vec<String>| {
        let request = ContextMenuRequest {
            menu: UiMenuRef { id: "puzzle2d".into(), args: None },
            surface: Some(ContextMenuSurfaceTarget { surface_id: "puzzle2d".into(), kind: "board".into(), hits: Vec::new(), selection: vec![ContextMenuSelectionGroup { domain: PUZZLE2D_GRANULARITY_NODE.into(), ids }], text: None }),
            window_instance_id: None,
            point: None,
        };
        block_on(app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)))
    };
    let on_handle = menu_for(&mut app, vec![handle_id.clone()]);
    let on_node = menu_for(&mut app, vec![node_id]);
    close_app(&mut app);
    let suggest = on_handle.iter().find(|item| item.id == "suggestNodes").expect("a selected handle offers the suggestions popup");
    assert_eq!(suggest.action.as_deref(), Some("openHandleSuggestions"));
    assert!(!on_node.iter().any(|item| item.id == "suggestNodes"), "a selected node has no handle to grow onto");
}

/// 💡️ LAW: open → hover → accept places ONE compatible node on concrete-forest (whose kind rows are
/// inferred, it names no manifest and carries no catalog) and re-selects it; the popup closes.
#[semio_framework_async_macros::async_test]
async fn open_hover_accept_places_one_node_on_concrete_forest_and_reselects_it() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let handle_id = first_free_handle_id(&before).expect("concrete forest offers a free handle");
    let before_nodes = board_snapshot_nodes(&before).len();
    let opened = dispatch(&mut app, "openHandleSuggestions", Some(&json!({ "handleId": handle_id.as_str(), "x": 10.0, "y": 20.0 })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("open the popup");
    assert_eq!(committed_edits(&opened), 0, "opening the picker must not touch the document");
    dispatch(&mut app, "hoverSuggestion", Some(&json!({ "index": 0, "handleId": handle_id.as_str() })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("preview a candidate");
    let accepted = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "handleId": handle_id.as_str() })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("accept");
    let after = fixture_of(&app);
    let placed: Vec<String> = board_snapshot_nodes(&after).iter().filter_map(|node| node.get("id").and_then(Value::as_str)).filter(|id| !board_snapshot_nodes(&before).iter().any(|node| node.get("id").and_then(Value::as_str) == Some(*id))).map(str::to_string).collect();
    let fastened = board_snapshot_edges(&after).iter().any(|edge| [edge.get("source"), edge.get("target")].iter().flatten().any(|end| end.as_str() == Some(handle_id.as_str())));
    let board = render_body(&mut app, overview::BODY_KEY);
    close_app(&mut app);
    assert_eq!(board_snapshot_nodes(&after).len(), before_nodes + 1, "accept places exactly one node");
    assert_eq!(placed.len(), 1, "exactly one node id is new");
    assert!(committed_edits(&accepted) > 0, "the placement must commit as document operations");
    assert!(fastened, "the placed node must be fastened to the handle the popup opened on");
    assert!(board.contains(&placed[0]), "the placed node must reach the painted board: {}", &board[..board.len().min(400)]);
}

/// 💡️ LAW: a handle that is already fastened has nothing to grow onto, so Nakagin refuses politely —
/// the popup opens, lists nothing, and accepting places nothing instead of faulting.
#[semio_framework_async_macros::async_test]
async fn nakagin_refuses_the_suggestions_popup_politely() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let before = fixture_of(&app);
    let handle_id = board_snapshot_edges(&before)[0].get("source").and_then(Value::as_str).expect("edge source").to_string();
    dispatch(&mut app, "openHandleSuggestions", Some(&json!({ "handleId": handle_id.as_str(), "x": 0.0, "y": 0.0 })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("open the popup");
    let accepted = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "handleId": handle_id.as_str() })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("accept must not fault");
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(committed_edits(&accepted), 0, "there is nothing to place, so nothing commits");
    assert_eq!(board_snapshot_nodes(&after).len(), board_snapshot_nodes(&before).len(), "a refused placement leaves the document alone");
}

/// 💡️ LAW: escape (`closeHandleSuggestions`) discards the picker and its provisional preview without
/// placing anything.
#[semio_framework_async_macros::async_test]
async fn closing_the_suggestions_popup_discards_the_preview() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let handle_id = first_free_handle_id(&before).expect("concrete forest offers a free handle");
    dispatch(&mut app, "openHandleSuggestions", Some(&json!({ "handleId": handle_id.as_str(), "x": 0.0, "y": 0.0 })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("open the popup");
    let closed = dispatch(&mut app, "closeHandleSuggestions", None, Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("close");
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(committed_edits(&closed), 0, "closing the picker never commits");
    assert_eq!(board_snapshot_nodes(&after).len(), board_snapshot_nodes(&before).len(), "the provisional preview was never a document node");
}

/// 🔁️ LAW: `shift+tab`'s verb (`cycleBrushCandidateBack`) walks the slot the other way — both
/// directions reach the same shared slot and neither commits.
#[semio_framework_async_macros::async_test]
async fn cycling_candidates_forward_and_back_never_commits() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let handle_id = first_free_handle_id(&before).expect("concrete forest offers a free handle");
    dispatch(&mut app, "openHandleSuggestions", Some(&json!({ "handleId": handle_id.as_str(), "x": 0.0, "y": 0.0 })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("open the popup");
    for action in ["cycleBrushCandidate", "cycleBrushCandidateBack"] {
        let result = dispatch(&mut app, action, None, Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).unwrap_or_else(|error| panic!("{action} must not fault: {error:?}"));
        assert_eq!(committed_edits(&result), 0, "{action} is a preview step, never a commit");
    }
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(board_snapshot_nodes(&after).len(), board_snapshot_nodes(&before).len());
}
//#endregion 🔖️HandleSuggestions
