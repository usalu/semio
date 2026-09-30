//! 🛠️ Laws for the select tool's ONE promise: one logical selection operation is one `ToolTransaction` — one
//! edit, one history row stamped with its `TransactionRef`, one parametric leaf carrying the literal targets and
//! parameters — and a cancelled gesture leaves zero trace. The board laws drive the REAL engine: press, move and
//! release on a host painted from the app's own fixture, the release batch filtered exactly as both coalescers
//! filter it (`🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing`), then dispatched as `applyBoardEvents`. The
//! streamed laws drive a gesture across several dispatches of one window: its ticks live in the window's open
//! transaction (previewed, never history) until one commit, and every host abort drops it with zero trace.

use super::*;
use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
use crate::editor::puzzle2d::engine::board_host::unit_tests::context::close_board_host;
use crate::editor::puzzle2d::unit_tests::context::*;
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::InvocationResult;

/// 🧱️ Three circle nodes on one line — `left`, `mid` and a locked `pin` — whose `v0` handles are compatible.
fn board_app() -> Puzzle2dApp {
    let mut app = app_with_registry();
    let board = json!({
        "schema": "puzzle.2d.fixture",
        "meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] },
        "nodes": [
            { "id": "left", "shape": "circle", "x": -200.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "left:v0", "handleKind": "a", "angle": 0.0 }] },
            { "id": "mid", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "mid:v0", "handleKind": "a", "angle": std::f64::consts::PI }] },
            { "id": "pin", "shape": "circle", "x": 200.0, "y": 0.0, "radius": 24.0, "locked": true, "handles": [] }
        ],
        "edges": []
    });
    dispatch(&mut app, "importFixture", Some(&json!({ "json": board })), None).expect("seed the board");
    app
}

/// 🎲️ A board host painted from the app's current document, camera at the origin, zoom 1, rotate ring off.
fn painted_host(app: &Puzzle2dApp) -> BoardHost {
    let mut host = puzzle_board_host();
    host.set_size(800, 600, 1.0);
    assert!(host.parse_fixture_json(&fixture_of(app).to_string()), "the engine paints the app's document");
    host.set_camera_silent(0.0, 0.0, 1.0);
    host.set_transform_flags(true, false);
    let _ = drain_board_events_json(&mut host);
    host
}

/// 📬️ The rows a host dispatches after draining: every transient row dropped (live-preview frames, chrome, hover).
fn dispatched_rows(host: &mut BoardHost) -> Vec<Value> {
    let rows: Vec<Value> = serde_json::from_str(&drain_board_events_json(host)).expect("drained rows parse");
    rows.into_iter().filter(|row| !matches!(row["name"].as_str(), Some("nodeMove" | "transformPreview" | "preselect" | "hover" | "brushPreview" | "linkCompatibleNodes" | "linkTargetRing"))).collect()
}

fn press(host: &mut BoardHost, x: f64, y: f64) {
    let screen = host.world_to_screen(Point::new(x, y));
    host.pointer_down_screen(screen.x, screen.y, 0, false, false);
}

fn move_to(host: &mut BoardHost, x: f64, y: f64) {
    let screen = host.world_to_screen(Point::new(x, y));
    host.pointer_move_screen(screen.x, screen.y, false, false, false);
}

fn release(host: &mut BoardHost, x: f64, y: f64) {
    let screen = host.world_to_screen(Point::new(x, y));
    host.pointer_up_screen(screen.x, screen.y, false, false, false);
}

fn flush(app: &mut Puzzle2dApp, rows: &[Value]) -> InvocationResult {
    dispatch(app, "applyBoardEvents", Some(&json!({ "eventsJson": serde_json::to_string(rows).expect("rows serialize") })), Some(overview::WINDOW_KIND_ID)).expect("applyBoardEvents")
}

fn node_at(app: &Puzzle2dApp, id: &str) -> (f64, f64) {
    let fixture = fixture_of(app);
    let node = fixture_nodes(&fixture).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id)).cloned().expect("node");
    (node.get("x").and_then(Value::as_f64).expect("x"), node.get("y").and_then(Value::as_f64).expect("y"))
}

/// 🧾️ The applied history rows one dispatch upserted that carry document ops.
fn edit_rows(result: &InvocationResult) -> Vec<HistoryEntry> {
    result.history_patch.as_ref().map(|patch| patch.upserts.iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()).unwrap_or_default()
}

fn english(entry: &HistoryEntry) -> String {
    entry.label.resolve(protocol::Terminology::Native, protocol::Locale::En).to_string()
}

fn german(entry: &HistoryEntry) -> String {
    entry.label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_string()
}

//#region 🎬️BoardGestures
/// 🖱️ One drag of an unselected node, end to end: the press stages the selection, the release is ONE batch of
/// `select` + `gesture`, and the guest commits ONE edit — one history row whose op is the `drag-selection` leaf over
/// the dragged id and the offset, stamped with the select tool's transaction and labelled from the leaf.
#[test]
fn one_board_drag_is_one_edit_one_row_and_one_transaction() {
    let mut app = board_app();
    let mut host = painted_host(&app);
    press(&mut host, -200.0, 0.0);
    move_to(&mut host, -150.0, 20.0);
    move_to(&mut host, -120.0, 40.0);
    assert!(dispatched_rows(&mut host).is_empty(), "nothing dispatchable leaves mid-gesture");
    release(&mut host, -120.0, 40.0);
    let rows = dispatched_rows(&mut host);
    assert_eq!(rows.iter().map(|row| row["name"].as_str().unwrap_or_default()).collect::<Vec<_>>(), vec!["select", "gesture"], "one batch: {rows:?}");
    let result = flush(&mut app, &rows);
    assert_eq!(committed_edits(&result), 1, "one drag is one edit");
    assert_eq!(node_at(&app, "left"), (-120.0, 40.0), "the leaf moved the node by the recorded offset");
    let rows = edit_rows(&result);
    assert_eq!(rows.len(), 1, "one history row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.puzzle.puzzle2d@1/*#editor#select", "the select tool authored it: {transaction:?}");
    assert!(rows[0].op_lines.iter().any(|line| line.starts_with("drag-selection") && line.contains("left") && line.contains("80") && line.contains("40")), "the op is the parametric leaf: {:?}", rows[0].op_lines);
    assert_eq!(english(&rows[0]), "Drag 1 item by (80, 40)", "the row is labelled from the leaf");
    let inspector = render_body(&mut app, inspection::PUZZLE2D_PLAY_BODY_PROPERTIES);
    assert!(inspector.contains("left"), "the gesture's selection landed in the same dispatch");
    close_board_host(host);
    close_app(&mut app);
}

/// 🗣️ The history row (and so the time-travel editor) of a group drag is labelled from its `drag-selection` leaf in
/// every language — "Drag 2 items by (80, 40)" / "2 Elemente um (80; 40) ziehen" — never the raw op text.
#[test]
fn a_group_drag_row_is_labelled_from_its_leaf_in_english_and_german() {
    let mut app = board_app();
    let mut host = painted_host(&app);
    host.set_selection_ids_silent(&["left".to_string(), "mid".to_string()]);
    press(&mut host, -200.0, 0.0);
    move_to(&mut host, -150.0, 20.0);
    move_to(&mut host, -120.0, 40.0);
    release(&mut host, -120.0, 40.0);
    let result = flush(&mut app, &dispatched_rows(&mut host));
    let rows = edit_rows(&result);
    assert_eq!(rows.len(), 1, "one group drag, one row: {rows:?}");
    assert!(rows[0].transaction.is_some(), "the row is the select tool's transaction");
    assert_eq!(english(&rows[0]), "Drag 2 items by (80, 40)");
    assert_eq!(german(&rows[0]), "2 Elemente um (80; 40) ziehen");
    assert!(rows[0].op_lines.iter().all(|line| !english(&rows[0]).contains(line.as_str())), "the label is the leaf's, not its op text: {:?}", rows[0].op_lines);
    assert_eq!((node_at(&app, "left"), node_at(&app, "mid")), ((-120.0, 40.0), (80.0, 40.0)));
    close_board_host(host);
    close_app(&mut app);
}

/// 🧲️ A drop that lands a handle next to a compatible one yields the connection INSIDE the same transaction —
/// still one edit, the connection's id minted from its pair.
#[test]
fn a_drop_and_its_connection_are_one_transaction() {
    let mut app = board_app();
    let mut host = painted_host(&app);
    press(&mut host, -200.0, 0.0);
    move_to(&mut host, -52.0, 0.0);
    release(&mut host, -52.0, 0.0);
    let result = flush(&mut app, &dispatched_rows(&mut host));
    assert_eq!(committed_edits(&result), 1, "the drop and its connection are one edit");
    let edges = fixture_edges(&fixture_of(&app)).to_vec();
    assert_eq!(edges.len(), 1, "the drop connected the facing handles: {edges:?}");
    let rows = edit_rows(&result);
    assert_eq!(rows.len(), 1);
    assert!(rows[0].op_lines.iter().any(|line| line.starts_with("connect-handles")), "the connection rides the drag's row: {:?}", rows[0].op_lines);
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert!(fixture_edges(&fixture_of(&app)).is_empty() && node_at(&app, "left") == (-200.0, 0.0), "one undo takes the drop and its edge back together");
    close_board_host(host);
    close_app(&mut app);
}

/// ↩️ A cancelled drag leaves zero trace: nothing dispatchable leaves the engine, so the guest sees no edit, no
/// redo and no row — and the document is byte-identical.
#[test]
fn a_cancelled_drag_leaves_zero_trace() {
    let mut app = board_app();
    let before = fixture_of(&app);
    let mut host = painted_host(&app);
    press(&mut host, -200.0, 0.0);
    move_to(&mut host, -100.0, 50.0);
    assert!(host.pointer_cancel_screen(), "the cancel claims the drag");
    let rows = dispatched_rows(&mut host);
    assert!(rows.is_empty(), "a cancelled gesture publishes nothing dispatchable: {rows:?}");
    let result = flush(&mut app, &rows);
    assert_eq!(committed_edits(&result), 0, "no edit");
    assert!(edit_rows(&result).is_empty(), "no row");
    assert!(!result.history_patch.as_ref().is_some_and(|patch| patch.can_redo), "no redo");
    assert_eq!(fixture_of(&app), before, "the document is untouched");
    close_board_host(host);
    close_app(&mut app);
}

/// 🔁️ Two drags are two transactions: two edits, two rows, two distinct refs.
#[test]
fn two_drags_are_two_transactions() {
    let mut app = board_app();
    let mut transactions = Vec::new();
    for (from, to) in [(-200.0, -180.0), (-180.0, -160.0)] {
        let mut host = painted_host(&app);
        press(&mut host, from, 0.0);
        move_to(&mut host, to, 0.0);
        release(&mut host, to, 0.0);
        let result = flush(&mut app, &dispatched_rows(&mut host));
        assert_eq!(committed_edits(&result), 1);
        transactions.extend(edit_rows(&result).into_iter().filter_map(|row| row.transaction));
        close_board_host(host);
    }
    assert_eq!(transactions.len(), 2, "each drag has its own row");
    assert_ne!(transactions[0].id, transactions[1].id, "and its own transaction");
    assert_eq!(node_at(&app, "left"), (-160.0, 0.0));
    close_app(&mut app);
}

/// 🔄️ A ring rotation paints a frame per pointer move and still commits ONE transaction whose op is the
/// `rotate-selection` leaf — the streamed-gesture law: previews stay in the engine, the commit happens once.
#[test]
fn a_streamed_ring_rotation_is_one_transaction() {
    let mut app = board_app();
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, "mid").expect("select mid");
    let mut host = painted_host(&app);
    host.set_transform_flags(true, true);
    host.set_selection_ids_silent(&["left".to_string(), "mid".to_string()]);
    let gumball: Value = serde_json::from_str(&host.transform_gumball_json()).expect("gumball vitals parse");
    assert_eq!(gumball["ringVisible"], true, "the ring is armed: {gumball}");
    let (pivot, radius) = (Point::new(gumball["pivot"]["x"].as_f64().expect("pivot x"), gumball["pivot"]["y"].as_f64().expect("pivot y")), gumball["radius"].as_f64().expect("radius"));
    let ring = |degrees: f64| Point::new(pivot.x + radius * degrees.to_radians().cos(), pivot.y + radius * degrees.to_radians().sin());
    let grab = ring(0.0);
    press(&mut host, grab.x, grab.y);
    for step in 1..=30 {
        let point = ring(f64::from(step) * 3.0);
        move_to(&mut host, point.x, point.y);
    }
    let last = ring(90.0);
    release(&mut host, last.x, last.y);
    let rows = dispatched_rows(&mut host);
    assert_eq!(rows.iter().map(|row| row["name"].as_str().unwrap_or_default()).collect::<Vec<_>>(), vec!["gesture"], "thirty frames, one record: {rows:?}");
    let result = flush(&mut app, &rows);
    assert_eq!(committed_edits(&result), 1, "the whole ring gesture is one edit");
    let rows = edit_rows(&result);
    assert!(rows.len() == 1 && rows[0].transaction.is_some() && rows[0].op_lines.iter().any(|line| line.starts_with("rotate-selection")), "one rotate row: {rows:?}");
    close_board_host(host);
    close_app(&mut app);
}
//#endregion 🎬️BoardGestures

//#region 🚀️TransformVerbs
/// ⌨️ Three nudges are three transactions, each its own row with its own ref, each a `drag-selection` leaf.
#[test]
fn three_nudges_are_three_transactions() {
    let mut app = board_app();
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, "left").expect("select left");
    let mut transactions = Vec::new();
    for _ in 0..3 {
        let result = dispatch(&mut app, "translateSelection", Some(&json!({ "dx": 1.0, "dy": 0.0, "step": 10.0 })), None).expect("nudge");
        assert_eq!(committed_edits(&result), 1, "one nudge, one edit");
        let rows = edit_rows(&result);
        assert!(rows.len() == 1 && rows[0].op_lines.iter().any(|line| line.starts_with("drag-selection")), "a nudge yields the drag leaf: {rows:?}");
        transactions.push(rows[0].transaction.clone().expect("a nudge is a tool transaction"));
    }
    assert_eq!(transactions.iter().map(|transaction| transaction.id.clone()).collect::<BTreeSet<_>>().len(), 3, "three distinct transactions");
    assert!(transactions.iter().all(|transaction| transaction.tool.ends_with("#translateSelection")));
    assert_eq!(node_at(&app, "left"), (-170.0, 0.0));
    close_app(&mut app);
}

/// 🔄️📏️ The rotate and scale verbs each commit ONE transaction whose op is their own parametric leaf.
#[test]
fn the_rotate_and_scale_verbs_each_commit_one_parametric_transaction() {
    let mut app = board_app();
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, "left").expect("select left");
    dispatch(&mut app, "interactionSelect", Some(&json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": serde_json::to_string(&vec![InteractionTarget { granularity: "node".into(), id: "left".into() }, InteractionTarget { granularity: "node".into(), id: "mid".into() }]).expect("targets"), "merge": "replace", "method": "pick" })), None).expect("select pair");
    for (verb, args, leaf) in [("rotateSelection", json!({ "angle": 90.0 }), "rotate-selection"), ("scaleSelection", json!({ "factor": 2.0 }), "scale-selection")] {
        let result = dispatch(&mut app, verb, Some(&args), None).expect(verb);
        assert_eq!(committed_edits(&result), 1, "{verb} is one edit");
        let rows = edit_rows(&result);
        assert!(rows.len() == 1 && rows[0].transaction.as_ref().is_some_and(|transaction| transaction.tool.ends_with(verb)) && rows[0].op_lines.iter().any(|line| line.starts_with(leaf)), "{verb} yields {leaf}: {rows:?}");
    }
    close_app(&mut app);
}

/// 🤝️🩹️ The HUD `move` and an inspector position `delta` yield the same `drag-selection` leaf as one transaction.
#[test]
fn the_hud_move_and_an_inspector_delta_yield_the_drag_leaf() {
    let mut app = board_app();
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, "left").expect("select left");
    let hud = dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "move 5 -5" })), Some(overview::WINDOW_KIND_ID)).expect("HUD move");
    let inspector = dispatch(&mut app, "patchInspectorNodes", Some(&json!({ "ids": ["left"], "field": "y", "delta": 7.0 })), None).expect("inspector delta");
    for (what, result) in [("HUD move", &hud), ("inspector delta", &inspector)] {
        let rows = edit_rows(result);
        assert!(rows.len() == 1 && rows[0].transaction.is_some() && rows[0].op_lines.iter().any(|line| line.starts_with("drag-selection")), "{what} yields the drag leaf: {rows:?}");
    }
    assert_eq!(node_at(&app, "left"), (-195.0, 2.0));
    close_app(&mut app);
}

/// 🔒️ A mixed selection commits: the unlocked target moves, the locked one stays, and no refusal is raised — the
/// leaf reports the locked rest instead.
#[test]
fn a_mixed_lock_selection_commits_and_moves_only_the_unlocked_target() {
    let mut app = board_app();
    dispatch(&mut app, "interactionSelect", Some(&json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": serde_json::to_string(&vec![InteractionTarget { granularity: "node".into(), id: "mid".into() }, InteractionTarget { granularity: "node".into(), id: "pin".into() }]).expect("targets"), "merge": "replace", "method": "pick" })), None).expect("select mid and pin");
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "dx": 10.0, "dy": 0.0 })), None).expect("translate");
    assert_eq!(committed_edits(&result), 1, "the movable target commits");
    assert!(!result.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "no refusal for a partially locked selection");
    assert_eq!((node_at(&app, "mid"), node_at(&app, "pin")), ((10.0, 0.0), (200.0, 0.0)));
    close_app(&mut app);
}
//#endregion 🚀️TransformVerbs

//#region 🌊️StreamedGestures
fn stream(app: &mut Puzzle2dApp, dx: f64) -> InvocationResult {
    dispatch(app, "translateSelection", Some(&json!({ "dx": dx, "dy": 0.0, "phase": "stream" })), Some(overview::WINDOW_KIND_ID)).expect("stream tick")
}

fn phase(app: &mut Puzzle2dApp, args: Value) -> InvocationResult {
    dispatch(app, "translateSelection", Some(&args), Some(overview::WINDOW_KIND_ID)).expect("gesture phase")
}

/// 👁️ Where the overview window PAINTS a node — its board fixture lane, which previews the window's open gesture.
fn painted_x(app: &mut Puzzle2dApp, id: &str) -> f64 {
    let body: Value = serde_json::from_str(&render_body(app, overview::BODY_KEY)).expect("board body");
    let fixture: Value = serde_json::from_str(body["board2d"]["fixtureJson"].as_str().expect("painted fixture lane")).expect("fixture parses");
    fixture_nodes(&fixture).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id)).and_then(|node| node.get("x")).and_then(Value::as_f64).expect("painted node")
}

/// 🧯️ Zero trace: no edit, no row, no redo, the document untouched and the window painting the document again.
#[track_caller]
fn assert_zero_trace(app: &mut Puzzle2dApp, result: &InvocationResult, before: &Value, what: &str) {
    assert_eq!(committed_edits(result), 0, "{what}: no edit");
    assert!(edit_rows(result).is_empty(), "{what}: no row");
    assert!(!result.history_patch.as_ref().is_some_and(|patch| patch.can_redo), "{what}: no redo");
    assert_eq!(&fixture_of(app), before, "{what}: the document is untouched");
    assert_eq!(painted_x(app, "left"), -200.0, "{what}: the window paints the document again");
}

/// 🌊️ A gesture spanning several dispatches: every `stream` tick lands in the window's ONE open transaction — the
/// window previews it, the document never moves, no history row appears — and the one `commit` publishes it as
/// ONE edit, one row, one transaction whose leaf carries the net offset; one undo takes the whole gesture back.
#[test]
fn a_gesture_streamed_over_several_dispatches_is_one_transaction() {
    let mut app = board_app();
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, "left").expect("select left");
    let before = fixture_of(&app);
    for tick in 1..=3 {
        let result = stream(&mut app, 10.0);
        assert_eq!(committed_edits(&result), 0, "tick {tick} is no edit");
        assert!(edit_rows(&result).is_empty(), "tick {tick} is no history row");
        assert_eq!(fixture_of(&app), before, "tick {tick} never moves the document");
        assert_eq!(painted_x(&mut app, "left"), -200.0 + 10.0 * f64::from(tick), "tick {tick} is previewed in the window");
    }
    let result = phase(&mut app, json!({ "phase": "commit" }));
    assert_eq!(committed_edits(&result), 1, "the commit is ONE edit");
    let rows = edit_rows(&result);
    assert_eq!(rows.len(), 1, "one history row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by the gesture's transaction");
    assert!(transaction.tool.ends_with("#translateSelection"), "{transaction:?}");
    assert!(rows[0].op_lines.iter().any(|line| line.starts_with("drag-selection") && line.contains("30")), "the leaf carries the net offset: {:?}", rows[0].op_lines);
    assert_eq!(english(&rows[0]), "Drag 1 item by (30, 0)");
    assert_eq!(node_at(&app, "left"), (-170.0, 0.0));
    assert_eq!(painted_x(&mut app, "left"), -170.0, "the window paints the committed document");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(node_at(&app, "left"), (-200.0, 0.0), "one undo takes the whole gesture back");
    close_app(&mut app);
}

/// 🧯️ A host abort mid-gesture (`blur`, `captureLost`, `frozen`) leaves zero trace, and a later commit finds
/// nothing to publish.
#[test]
fn a_host_abort_mid_gesture_leaves_zero_trace() {
    let mut app = board_app();
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, "left").expect("select left");
    let before = fixture_of(&app);
    for reason in ["blur", "captureLost", "frozen"] {
        stream(&mut app, 10.0);
        stream(&mut app, 15.0);
        assert_eq!(painted_x(&mut app, "left"), -175.0, "{reason}: the gesture is open");
        let result = phase(&mut app, json!({ "phase": "abort", "reason": reason }));
        assert_zero_trace(&mut app, &result, &before, reason);
        let late = phase(&mut app, json!({ "phase": "commit" }));
        assert_zero_trace(&mut app, &late, &before, &format!("a commit after {reason}"));
    }
    close_app(&mut app);
}

/// 🔀️ A one-shot transform of the same window interrupts an open gesture (`captureLost`): the gesture vanishes
/// and only the one-shot is history.
#[test]
fn a_one_shot_transform_interrupts_an_open_gesture() {
    let mut app = board_app();
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, "left").expect("select left");
    stream(&mut app, 50.0);
    assert_eq!(painted_x(&mut app, "left"), -150.0, "the gesture is open");
    let result = phase(&mut app, json!({ "dx": 1.0, "dy": 0.0 }));
    assert_eq!(committed_edits(&result), 1, "the one-shot is its own edit");
    assert_eq!(node_at(&app, "left"), (-199.0, 0.0), "the interrupted gesture contributed nothing");
    let late = phase(&mut app, json!({ "phase": "commit" }));
    assert_eq!(committed_edits(&late), 0, "the interrupted gesture is gone");
    close_app(&mut app);
}

/// 🪛️ A window that leaves the select utility retires its open gesture (`retired`) with zero trace.
#[test]
fn leaving_the_select_utility_retires_an_open_gesture() {
    let mut app = board_app();
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, "left").expect("select left");
    let before = fixture_of(&app);
    stream(&mut app, 40.0);
    assert_eq!(painted_x(&mut app, "left"), -160.0, "the gesture is open");
    let brush = semio_framework_plugin::ViewModel { active_utility_id: Some(brush_utility::UTILITY_ID.into()), ..window_view(overview::WINDOW_KIND_ID, overview::WINDOW_KIND_ID) };
    dispatch_in_view(&mut app, "focusSelection", None, Some(overview::WINDOW_KIND_ID), brush).expect("a verb under the brush");
    let late = phase(&mut app, json!({ "phase": "commit" }));
    assert_zero_trace(&mut app, &late, &before, "a retired gesture");
    close_app(&mut app);
}

/// 📐️ A document edit landing under an open gesture (another window, a peer) aborts it (`baseMoved`): the commit
/// that finds it publishes nothing, and only the other edit is history.
#[test]
fn a_document_moved_under_an_open_gesture_aborts_it() {
    let mut app = board_app();
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, "left").expect("select left");
    stream(&mut app, 40.0);
    assert_eq!(painted_x(&mut app, "left"), -160.0, "the gesture is open");
    let other = dispatch(&mut app, "patchInspectorNodes", Some(&json!({ "ids": ["mid"], "field": "y", "value": 5.0 })), Some(detail::WINDOW_KIND_ID)).expect("an edit from another window");
    assert_eq!(committed_edits(&other), 1, "the other window's edit lands");
    let late = phase(&mut app, json!({ "phase": "commit" }));
    assert_eq!(committed_edits(&late), 0, "the gesture opened on the old base publishes nothing");
    assert_eq!((node_at(&app, "left"), node_at(&app, "mid")), ((-200.0, 0.0), (0.0, 5.0)));
    assert_eq!(painted_x(&mut app, "left"), -200.0, "the window paints the moved document, not the stale gesture");
    close_app(&mut app);
}
//#endregion 🌊️StreamedGestures

//#region 🔁️DispatchThreading
/// 🧱️ One node `left` at the origin, the document every threaded dispatch reads.
fn threading_snapshot() -> Puzzle2dPlaySnapshot {
    Puzzle2dPlaySnapshot::new(json!({
        "schema": "puzzle.2d.fixture",
        "nodes": [{ "id": "left", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [] }],
        "edges": []
    }))
}

/// 🌊️ The window transient a streamed `translateSelection` of `left` by `dx` persisted on `revision`.
fn streaming_transient(snapshot: &Puzzle2dPlaySnapshot, dx: f64, revision: &str) -> Puzzle2dWindowTransient {
    let mut tool = select_utility::Puzzle2dSelectTool::start("translateSelection", "seed-1", revision).expect("the tool starts");
    let request = SelectToolRequest { base: std::sync::Arc::new(snapshot.typed().clone()), proximity_radius: 0.0, records: vec![Puzzle2dSelectionRecord::drag(vec!["left".to_string()], dx, 0.0)] };
    assert_eq!(tool.send(select_utility::select_tool::Event::Stream(request)), Ok(ToolStep::Open));
    Puzzle2dWindowTransient { select_tool: tool.persist(), ..Default::default() }
}

/// 📨️ One dispatch of `action` from the overview window over `left` selected, on `revision` under `utility`.
fn threaded_emit(snapshot: &Puzzle2dPlaySnapshot, transient: &Puzzle2dWindowTransient, utility: &str, revision: &str, action: &str, args: Value) -> (Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, EphemeralEmit<EditorApp<Puzzle2dPlayApp>>) {
    let command = Puzzle2dCommand::from_action(action, Some(args), Some(overview::WINDOW_KIND_ID.to_string()));
    let selection = protocol::DomainSelection { granularity: PUZZLE2D_GRANULARITY_NODE.into(), ids: vec!["left".to_string()], anchor_id: None };
    let view = window_view(overview::WINDOW_KIND_ID, overview::WINDOW_KIND_ID);
    puzzle2d_dispatch_emit(&command, snapshot, &Puzzle2dConfig::default(), &Puzzle2dWindowConfig::default(), transient, overview::WINDOW_KIND_ID, Some(&view), utility, &selection, "seed-2", revision, None).expect("the dispatch emits")
}

/// 🌊️ A stream tick persists the open gesture in the window transient and publishes no edit; the commit dispatch
/// that resumes it publishes the net leaf under the ref minted at the first tick, and clears the transient.
#[test]
fn a_threaded_stream_publishes_nothing_until_its_commit_publishes_one_transaction() {
    let snapshot = threading_snapshot();
    let (tick, tick_transient) = threaded_emit(&snapshot, &Puzzle2dWindowTransient::default(), select_utility::UTILITY_ID, "rev-1", "translateSelection", json!({ "dx": 10.0, "dy": 0.0, "phase": "stream" }));
    assert!(tick.artifact_mutations.is_empty() && tick.transaction.is_none(), "a tick is no edit: {:?}", tick.artifact_mutations);
    assert_eq!(tick_transient.window_transient.len(), 1, "the tick persists the gesture in the window transient");
    let open = streaming_transient(&snapshot, 30.0, "rev-1");
    let opened = open.select_tool.clone().expect("open gesture");
    let (commit, commit_transient) = threaded_emit(&snapshot, &open, select_utility::UTILITY_ID, "rev-1", "translateSelection", json!({ "phase": "commit" }));
    assert_eq!(commit.artifact_mutations, vec![crate::standards::v1::subsets::any::schema::mutations::drag_selection(vec!["left".to_string()], 30.0, 0.0)], "the commit publishes the net leaf");
    assert_eq!(commit.transaction, Some(opened.transaction), "under the ref minted when the gesture opened");
    assert_eq!(commit.coalesce_key, None);
    assert_eq!(commit_transient.window_transient.len(), 1, "the commit clears the persisted gesture");
}

/// 🧯️ Every host abort of a persisted gesture — `blur`, `captureLost`, `frozen` sent as an abort phase, a document
/// that moved under it (`baseMoved`), a utility that is no longer select (`retired`) — publishes no edit and clears
/// the window transient; a verb that may not publish the transient leaves the gesture for the next one that may.
#[test]
fn every_host_abort_of_a_threaded_gesture_publishes_no_edit_and_clears_it() {
    let snapshot = threading_snapshot();
    let open = streaming_transient(&snapshot, 30.0, "rev-1");
    for reason in ["blur", "captureLost", "frozen"] {
        let (emit, ephemeral) = threaded_emit(&snapshot, &open, select_utility::UTILITY_ID, "rev-1", "translateSelection", json!({ "phase": "abort", "reason": reason }));
        assert!(emit.artifact_mutations.is_empty() && emit.transaction.is_none(), "{reason}: no edit");
        assert_eq!(ephemeral.window_transient.len(), 1, "{reason}: the gesture is cleared");
    }
    let (moved, moved_transient) = threaded_emit(&snapshot, &open, select_utility::UTILITY_ID, "rev-2", "translateSelection", json!({ "phase": "commit" }));
    assert!(moved.artifact_mutations.is_empty() && moved.transaction.is_none(), "a gesture on a moved base commits nothing");
    assert_eq!(moved_transient.window_transient.len(), 1, "baseMoved clears the gesture");
    let (retired, retired_transient) = threaded_emit(&snapshot, &open, brush_utility::UTILITY_ID, "rev-1", "applyBoardEvents", json!({ "eventsJson": "[]" }));
    assert!(retired.artifact_mutations.is_empty() && retired.ui_scope != semio_framework::kernel::UiDirtyScope::None, "a retired gesture repaints its window without an edit");
    assert_eq!(retired_transient.window_transient.len(), 1, "leaving the select utility retires the gesture");
    let (_, kept) = threaded_emit(&snapshot, &open, brush_utility::UTILITY_ID, "rev-1", "focusSelection", json!({}));
    assert!(kept.window_transient.is_empty(), "a verb without the transient lane leaves the gesture for the next verb that has it");
    let (unknown, unknown_transient) = threaded_emit(&snapshot, &open, select_utility::UTILITY_ID, "rev-1", "translateSelection", json!({ "phase": "hover" }));
    assert!(unknown.artifact_mutations.is_empty() && unknown_transient.window_transient.is_empty(), "an unknown phase is refused without touching the gesture");
}

/// 🔀️ A one-shot transform interrupting a persisted gesture aborts it (`captureLost`) and commits only itself,
/// under a fresh ref.
#[test]
fn a_one_shot_interrupting_a_threaded_gesture_commits_only_itself() {
    let snapshot = threading_snapshot();
    let open = streaming_transient(&snapshot, 30.0, "rev-1");
    let opened = open.select_tool.clone().expect("open gesture");
    let (emit, ephemeral) = threaded_emit(&snapshot, &open, select_utility::UTILITY_ID, "rev-1", "translateSelection", json!({ "dx": 1.0, "dy": 0.0 }));
    assert_eq!(emit.artifact_mutations, vec![crate::standards::v1::subsets::any::schema::mutations::drag_selection(vec!["left".to_string()], 1.0, 0.0)]);
    let transaction = emit.transaction.expect("the one-shot is its own transaction");
    assert!(transaction.id != opened.transaction.id && transaction.tool.ends_with("#translateSelection"), "{transaction:?}");
    assert_eq!(ephemeral.window_transient.len(), 1, "the interrupted gesture is cleared");
}
/// 🧹️ A selection that repeats an id drags each target once: the leaf's target set is unique, so the leaf never
/// trips its own invariant.
#[test]
fn a_selection_with_repeated_ids_drags_each_target_once() {
    let snapshot = threading_snapshot();
    let command = Puzzle2dCommand::from_action("translateSelection", Some(json!({ "dx": 5.0, "dy": 0.0 })), Some(overview::WINDOW_KIND_ID.to_string()));
    let selection = protocol::DomainSelection { granularity: PUZZLE2D_GRANULARITY_NODE.into(), ids: vec!["left".to_string(), "left".to_string(), "left".to_string()], anchor_id: None };
    let view = window_view(overview::WINDOW_KIND_ID, overview::WINDOW_KIND_ID);
    let (emit, _) = puzzle2d_dispatch_emit(&command, &snapshot, &Puzzle2dConfig::default(), &Puzzle2dWindowConfig::default(), &Puzzle2dWindowTransient::default(), overview::WINDOW_KIND_ID, Some(&view), select_utility::UTILITY_ID, &selection, "seed-3", "rev-1", None).expect("emit");
    assert_eq!(emit.artifact_mutations, vec![crate::standards::v1::subsets::any::schema::mutations::drag_selection(vec!["left".to_string()], 5.0, 0.0)]);
    assert!(emit.transaction.is_some(), "one transaction");
}
//#endregion 🔁️DispatchThreading

//#region 📤️Emit
/// 📤️ The emit itself: the select tool's transaction rides the ONE emit with its parametric leaf, never a
/// coalesce key, under a ref minted from the admission's seed on the host clock.
#[test]
fn the_board_emit_carries_the_transaction_and_the_parametric_leaf() {
    let snapshot = Puzzle2dPlaySnapshot::new(json!({
        "schema": "puzzle.2d.fixture",
        "nodes": [{ "id": "left", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [] }],
        "edges": []
    }));
    let events = json!([
        { "name": "select", "payload": { "ids": ["left"], "exitHighlightIds": [], "gestureId": "gesture-1" } },
        { "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "drag", "targets": ["left"], "dx": 3.0, "dy": 4.0, "proximity": [] } }
    ])
    .to_string();
    let command = Puzzle2dCommand::from_action("applyBoardEvents", Some(json!({ "eventsJson": events })), None);
    let emit_for = |seed: &str| puzzle2d_dispatch_emit(&command, &snapshot, &Puzzle2dConfig::default(), &Puzzle2dWindowConfig::default(), &Puzzle2dWindowTransient::default(), overview::WINDOW_KIND_ID, None, select_utility::UTILITY_ID, &protocol::DomainSelection::default(), seed, "rev-1", None).expect("emit").0;
    let emit = emit_for("seed-7");
    let transaction = emit.transaction.clone().expect("the commit carries its transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.puzzle.puzzle2d@1/*#editor#select", "{transaction:?}");
    assert_ne!(emit_for("seed-8").transaction.map(|other| other.id), Some(transaction.id), "two admissions never share a transaction id");
    assert_eq!(emit.coalesce_key, None, "a committed transaction never rides a coalesced amend");
    assert_eq!(emit.artifact_mutations, vec![crate::standards::v1::subsets::any::schema::mutations::drag_selection(vec!["left".to_string()], 3.0, 4.0)]);
    assert_eq!(emit.interaction_writes.len(), 1, "the gesture's selection is an interaction write of the same dispatch");
    let unbound = emit_for("");
    assert_eq!(unbound.transaction, None, "a view without command authority publishes the leaf plainly");
    assert_eq!(unbound.artifact_mutations.len(), 1);
}
//#endregion 📤️Emit
