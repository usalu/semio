use super::*;

/// 🧫️ The corpus React's `coalesceBoard2dEvents` replays too (`🖥️Board2dHost/🧪️tests/🧪️board-event-coalescing`).
const BOARD_EVENT_COALESCING_CORPUS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json"));

/// 🧫️ The corpus React's `board2dFloat32Decimal` replays too (`🖥️Board2dHost/🧪️tests/🧪️float32-decimal`).
const BOARD_FLOAT32_DECIMAL_CORPUS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/🖥️Board2dHost/🧫️fixtures/🧫️float32-decimal/🔣️.json"));

fn typed_queue(rows: &[Value]) -> infinite_canvas::BoardEventQueue {
    let mut queue = infinite_canvas::BoardEventQueue::default();
    for row in rows {
        let name = row["name"].as_str().expect("row name");
        let kind = infinite_canvas::BoardEventKind::parse(name).unwrap_or_else(|| panic!("corpus kind {name} is one the engine publishes"));
        let payload = serde_json::to_string(&row["payload"]).expect("payload");
        let key = (kind == infinite_canvas::BoardEventKind::NodeMove).then(|| row["payload"]["id"].as_str()).flatten();
        queue.push(infinite_canvas::BoardOwnedEvent::from_payload(kind, &payload, key).expect("corpus row fits one event")).expect("corpus batch fits the queue");
    }
    queue
}

/// ✂️ Law: the board engine records a drag offset in exactly the canonical form React gives pointer coordinates —
/// the shortest decimal of the f32 value — over the one shared corpus.
#[test]
fn the_engine_offset_form_replays_the_shared_f32_decimal_corpus() {
    let corpus: Value = serde_json::from_str(BOARD_FLOAT32_DECIMAL_CORPUS).expect("corpus parses");
    for case in corpus["cases"].as_array().expect("cases") {
        let (value, expected) = (case["value"].as_f64().expect("value"), case["expect"].as_f64().expect("expect"));
        assert_eq!(infinite_canvas::board_pointer_offset(value), expected, "{}", case["name"]);
    }
}

/// ✂️ Law: the wgpu board maps a pointer exactly as React's Board2dHost maps a DOM pointer — every corpus coordinate,
/// read through f32, reaches the engine as the shortest decimal `board2dFloat32Decimal` gives it, also off a fractional
/// surface origin — so a drag records the same offset on both hosts.
#[test]
fn the_wgpu_board_pointer_replays_the_shared_f32_decimal_corpus() {
    let corpus: Value = serde_json::from_str(BOARD_FLOAT32_DECIMAL_CORPUS).expect("corpus parses");
    let origin = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let fractional = Rect { x: 0.5, y: 12.25, ..origin };
    for case in corpus["cases"].as_array().expect("cases") {
        let (value, expected) = (case["value"].as_f64().expect("value"), case["expect"].as_f64().expect("expect"));
        assert_eq!(board_local_pointer(origin, value as f32, value as f32), (expected, expected), "{}", case["name"]);
        assert_eq!(board_local_pointer(fractional, value as f32, value as f32), (expected - 0.5, expected - 12.25), "{} off a fractional origin", case["name"]);
    }
}

/// ⚖️ Law: every corpus case coalesces here exactly as React coalesces it — same board rows in the same order,
/// same camera for the view lane, same flush verdict — so one drag reaches the guest as ONE batch on both hosts and
/// a pan is never a board event on either.
#[test]
fn the_wgpu_coalescer_replays_the_shared_corpus() {
    let corpus: Value = serde_json::from_str(BOARD_EVENT_COALESCING_CORPUS).expect("corpus parses");
    let cases = corpus["cases"].as_array().expect("cases");
    assert!(cases.len() >= 10, "the corpus covers every row family");
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let coalesced = coalesce_owned_board_events(&typed_queue(case["rows"].as_array().expect("rows"))).expect("corpus batch coalesces");
        assert_eq!(serde_json::from_str::<Value>(&coalesced.events_json).expect("dispatched rows parse"), case["expect"]["events"], "{name}");
        let camera = case["expect"]["camera"].as_object().map(|camera| ["x", "y", "zoom"].map(|axis| camera[axis].as_f64().expect("camera axis")));
        assert_eq!(coalesced.camera, camera, "{name}");
        assert_eq!(coalesced.flush_now, case["expect"]["flushNow"].as_bool().expect("flushNow"), "{name}");
    }
}

/// ⚖️ Law: the transient and flush-now tables name exactly what React's two sets name
/// (`PUZZLE2D_TRANSIENT_EVENT_NAMES`/`PUZZLE2D_FLUSH_NOW_EVENT_NAMES`, `🖥️Board2dHost/🟦️.tsx`), and every kind
/// the engine publishes is exactly one of transient, terminal, `camera` or a waiting companion row.
#[test]
fn transient_and_flush_now_tables_match_the_react_sets() {
    use infinite_canvas::BoardEventKind;
    let transient: Vec<&str> = BoardEventKind::ALL.into_iter().filter(|kind| board_event_transient(*kind)).map(BoardEventKind::name).collect();
    assert_eq!(transient, ["nodeMove", "transformPreview", "preselect", "hover", "brushPreview", "linkCompatibleNodes", "linkTargetRing"]);
    let terminal: Vec<&str> = BoardEventKind::ALL.into_iter().filter(|kind| board_event_flush_now(*kind)).map(BoardEventKind::name).collect();
    assert_eq!(terminal, ["gesture", "select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete", "regionCreate", "regionResize"]);
    for kind in BoardEventKind::ALL {
        assert_eq!(BoardEventKind::parse(kind.name()), Some(kind));
        assert!(!(board_event_transient(kind) && board_event_flush_now(kind)), "{} is transient or terminal, never both", kind.name());
    }
}

/// 🎯️ Port check for `board2d_granularity_by_id` against React's `board2dGranularityById`
/// (`🖥️Board2dHost/🟦️.tsx:190`): handles nested under a node are `handle`, ids in `edges` are `edge`,
/// everything else — including an id the fixture never carries — reads as `node`.
#[test]
fn board_granularity_classification_matches_the_react_table() {
    let fixture = json!({
        "nodes": [{ "id": "n1", "handles": [{ "id": "n1:h1" }] }, { "id": "n2" }],
        "edges": [{ "id": "e1" }],
    })
    .to_string();
    let by_id = board2d_granularity_by_id(&fixture);
    assert_eq!(by_id.get("n1").map(String::as_str), Some("node"));
    assert_eq!(by_id.get("n2").map(String::as_str), Some("node"));
    assert_eq!(by_id.get("n1:h1").map(String::as_str), Some("handle"));
    assert_eq!(by_id.get("e1").map(String::as_str), Some("edge"));
    assert_eq!(by_id.get("never-published"), None, "an unknown id is absent, and the hover writer defaults it to node");
    assert!(board2d_granularity_by_id("not json").is_empty(), "a refused fixture classifies nothing");
}

/// 🔗️ Law (design §16.4): what an open time-travel draft references reaches the wgpu board engine from the scene's
/// `highlighted_ids_json` exactly as React's Board2dHost forwards it (`setHighlightedIdsJson`): the ids are highlighted,
/// an unchanged scene re-applies nothing, a new preview fixture keeps them, and `[]` clears them.
#[test]
fn a_draft_reference_highlight_reaches_the_wgpu_board_engine() {
    let node = |id: &str, x: f64| json!({ "id": id, "x": x, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [] });
    let fixture = |x: f64| json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [node("left", -x), node("mid", x)], "edges": [] }).to_string();
    let mut board = ui_wgpu::wgpu::Board2dScene::base(fixture(40.0), json!({ "x": 0.0, "y": 0.0, "zoom": 1.0 }).to_string(), true);
    board.highlighted_ids_json = json!(["mid", "left"]).to_string();
    let (mut host, mut cache) = (infinite_canvas::BoardHost::default(), BoardSyncCache::default());
    assert!(sync_board_engine(&mut host, &mut cache, &board, 800, 600), "the first scene syncs");
    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["left","mid"]"#);
    assert!(!sync_board_engine(&mut host, &mut cache, &board, 800, 600), "an unchanged scene re-applies nothing");
    board.snapshot_json = fixture(80.0);
    assert!(sync_board_engine(&mut host, &mut cache, &board, 800, 600), "a new preview syncs");
    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["left","mid"]"#, "a new preview keeps what the draft references");
    board.highlighted_ids_json = "[]".into();
    assert!(sync_board_engine(&mut host, &mut cache, &board, 800, 600), "a closed draft syncs");
    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), "[]", "a closed draft highlights nothing");
}

/// 🎛️ Law: the rotate ring runs on wgpu as it runs on React — the press opens the engine's direct lane, every move turns the
/// selection without dispatching anything (its transient previews never leave the host), and the release publishes ONE
/// `applyBoardEvents` holding the single `rotate` record; the lane is closed and nothing is left for the frame pump.
#[test]
fn the_wgpu_rotate_ring_publishes_one_rotate_record() {
    let board_id = "board-rotate-ring";
    let inner = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    ensure_engine_surface(board_id, board_id, 800, 600);
    let node = |id: &str, x: f64| json!({ "id": id, "x": x, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [] });
    let fixture = json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [node("node-a", -40.0), node("node-b", 40.0)], "edges": [] }).to_string();
    let mut host = infinite_canvas::BoardHost::default();
    host.set_size(800, 600, 1.0);
    host.set_camera_silent(0.0, 0.0, 1.0);
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);infinite_canvas::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut host,&fixture,&mut control)}, "the ring fixture parses");
    host.set_selection_ids_silent(&["node-a".to_string(), "node-b".to_string()]);
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(board_id).expect("surface").board_host = Some(ManuallyDrop::new(host)));
    while board_drain_into_buffer(board_id) {}
    board_retire_pending_events(board_id);
    let ring = |degrees: f64| {
        with_board_host(board_id, |host| {
            let (pivot, radius) = host.transform_gumball_geometry().expect("the rotate ring is armed");
            let point = host.world_to_screen(canvas::Point::new(pivot.x + radius * degrees.to_radians().cos(), pivot.y + radius * degrees.to_radians().sin()));
            (point.x as f32, point.y as f32)
        })
        .expect("board host")
    };
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let (gx, gy) = ring(0.0);
    puzzle_board_pointer_down_into(board_id, inner, gx, gy, 0, false, false, &mut input).expect("an idle board takes the press");
    assert_eq!(with_board_host(board_id, infinite_canvas::BoardHost::pointer_lane_is_direct), Some(true), "the press on the ring opens the direct lane");
    for degrees in [30.0, 60.0, 90.0] {
        let (x, y) = ring(degrees);
        assert_eq!(puzzle_board_pointer_move_into(board_id, "controller", inner, x, y, false, false, false, &mut input), Ok(false), "a ring frame at {degrees}° dispatches nothing");
        assert!(matches!(input.take_action_step(), Ok(None)), "a ring frame at {degrees}° leaves no action");
    }
    assert_ne!(with_board_host(board_id, |host| host.nodes.get("node-a").map(|node| (node.x, node.y))).flatten(), Some((-40.0, 0.0)), "the frames turned the selection");
    let (x, y) = ring(90.0);
    assert_eq!(puzzle_board_pointer_up_into(board_id, "controller", inner, x, y, false, false, false, &mut input), Ok(true), "the release dispatches");
    let action = input.take_action_step().expect("the release publishes").expect("one action").into_descriptor().expect("an action descriptor");
    assert_eq!((action.controller_id.as_str(), action.action.as_str()), ("controller", "applyBoardEvents"));
    let rows: Value = serde_json::from_str(Value::from(action.args.as_ref().expect("args"))["eventsJson"].as_str().expect("eventsJson")).expect("rows parse");
    assert_eq!(rows.as_array().map(|rows| rows.iter().map(|row| row["name"].as_str().unwrap_or_default()).collect::<Vec<_>>()), Some(vec!["gesture"]), "one record: {rows}");
    let record = &rows[0]["payload"];
    assert_eq!((record["kind"].as_str(), &record["targets"]), (Some("rotate"), &json!(["node-a", "node-b"])));
    assert!(record["angle"].as_f64().is_some_and(|angle| (angle - std::f64::consts::FRAC_PI_2).abs() < 1e-6), "a quarter turn: {record}");
    assert!(matches!(input.take_action_step(), Ok(None)), "ONE dispatch");
    assert_eq!(with_board_host(board_id, infinite_canvas::BoardHost::pointer_lane_is_direct), Some(false), "the release closes the lane");
    assert_eq!(publish_board_event_step(board_id, "controller", &mut input), Ok(false), "nothing is left for the frame pump");
    ENGINE_SURFACES.with(|cell| {
        cell.borrow_mut().remove(board_id);
    });
}

//#region 🚦️OneDrainInputLaws
/// 🖱️ A board of two draggable nodes under an identity camera, installed as `board_id`'s host with nothing pending;
/// answers the pointer position of `node-a`.
fn install_click_board(board_id: &str) -> (f32, f32) {
    ensure_engine_surface(board_id, board_id, 800, 600);
    let node = |id: &str, x: f64| json!({ "id": id, "x": x, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [] });
    let fixture = json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [node("node-a", -40.0), node("node-b", 40.0)], "edges": [] }).to_string();
    let mut host = infinite_canvas::BoardHost::default();
    host.set_size(800, 600, 1.0);
    host.set_camera_silent(0.0, 0.0, 1.0);
    assert!({let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);infinite_canvas::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut host,&fixture,&mut control)}, "the click fixture parses");
    let at = host.world_to_screen(canvas::Point::new(-40.0, 0.0));
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(board_id).expect("surface").board_host = Some(ManuallyDrop::new(host)));
    while board_drain_into_buffer(board_id) {}
    board_retire_pending_events(board_id);
    (at.x as f32, at.y as f32)
}

/// 🧾️ Every action the input holds, in publication order, as its name and — for `applyBoardEvents` — its row names, after
/// the engine's own queued events took the frame pump's coalesced flush.
fn published_board_actions(board_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<(String, Vec<String>)> {
    while publish_board_event_step(board_id, "controller", input) == Ok(true) {}
    crate::collect_fixture_actions(input)
        .iter()
        .map(|action| {
            let rows = (action.action == "applyBoardEvents")
                .then(|| serde_json::from_str::<Value>(Value::from(action.args.as_ref().expect("args"))["eventsJson"].as_str().expect("eventsJson")).expect("rows parse"))
                .and_then(|rows| rows.as_array().map(|rows| rows.iter().map(|row| row["name"].as_str().unwrap_or_default().to_string()).collect()))
                .unwrap_or_default();
            (action.action.clone(), rows)
        })
        .collect()
}

/// 🧹️ Removes a law's board surface.
fn remove_click_board(board_id: &str) {
    ENGINE_SURFACES.with(|cell| {
        cell.borrow_mut().remove(board_id);
    });
}

/// ⚖️ LAW (the engine contract the wgpu host answers to; live fault F14): every idle move is a retained commit, and a
/// direct press that reaches the board while that commit has not taken a step faults it — the commit is pinned to the
/// revision it was planned at. A host must therefore never touch a board whose pointer authority is busy.
#[test]
fn a_direct_press_under_a_pending_pointer_commit_faults_the_engine() {
    let board_id = "board-press-contract";
    let (x, y) = install_click_board(board_id);
    let step = with_board_host_mut(board_id, |host| {
        let plan = host
            .plan_pointer(infinite_canvas::BoardPointerIntent { phase: infinite_canvas::BoardPointerPhase::Move, x: f64::from(x), y: f64::from(y), shift: false, ctrl_or_meta: false, alt: false })
            .ok()
            .expect("the idle move plans");
        assert!(plan.requires_retained_commit(), "an idle move is a retained hover commit, never an idle plan");
        assert!(host.begin_pointer_commit(plan).is_ok(), "the hover commit begins");
        host.pointer_down_screen(f64::from(x), f64::from(y), 0, false, false);
        let mut step = infinite_canvas::BoardAuthorityStep::Pending;
        for _ in 0..64 {
            step = with_engine_close_context(1, |context| host.step_pointer_commit(context));
            if step != infinite_canvas::BoardAuthorityStep::Pending {
                break;
            }
        }
        step
    })
    .expect("board host");
    assert_eq!(step, infinite_canvas::BoardAuthorityStep::Fault, "the press bumped the revision under the unstepped commit");
    remove_click_board(board_id);
}

/// ⚖️ LAW (live fault F14): a click is ONE input drain on a slow frame — the move onto the node, the press and the
/// release are dispatched back to back and the frame steps the board's authority only afterwards. The press settles the
/// move's pending hover commit first — committed in the engine, dispatched nowhere (hover is transient); the release's
/// commit then reaches its terminal without a fault, the node is selected, its `select` row is published, and the
/// authority is idle with no lane left open.
#[test]
fn a_click_dispatched_in_one_input_drain_selects_the_node() {
    let board_id = "board-click-one-drain";
    let inner = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let (x, y) = install_click_board(board_id);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    puzzle_board_pointer_move_into(board_id, "controller", inner, x, y, false, false, false, &mut input).expect("the move onto the node is admitted");
    assert_eq!(with_board_host(board_id, infinite_canvas::BoardHost::pointer_authority_terminal_is_empty), Some(false), "the move's hover commit is pending and unstepped");
    puzzle_board_pointer_down_into(board_id, inner, x, y, 0, false, false, &mut input).expect("the press settles the pending commit, then presses");
    assert_eq!(with_board_host(board_id, |host| host.hovered_id.clone()).flatten().as_deref(), Some("node-a"), "the hover was committed, not lost");
    let before_release = published_board_actions(board_id, &mut input);
    assert!(before_release.iter().all(|(_, rows)| rows.iter().all(|name| name != "hover")), "the settled hover is no board row: {before_release:?}");
    puzzle_board_pointer_up_into(board_id, "controller", inner, x, y, false, false, false, &mut input).expect("the release is admitted");
    settle_board_pointer_authority_into(board_id, &mut input).expect("the release's commit reaches its terminal without a fault");
    let after_release = published_board_actions(board_id, &mut input);
    assert!(after_release.iter().any(|(_, rows)| rows.iter().any(|name| name == "select")), "the click publishes its selection: {after_release:?}");
    assert_eq!(with_board_host(board_id, |host| host.selection.iter().cloned().collect::<Vec<_>>()), Some(vec!["node-a".to_string()]), "the node is selected");
    assert_eq!(with_board_host(board_id, |host| (host.pointer_authority_terminal_is_empty(), host.pointer_lane_in_flight())), Some((true, false)), "the authority is idle and the click's lane is closed");
    remove_click_board(board_id);
}

/// ⚖️ LAW (live fault F14): a double click in one input drain — press, release, press, release with no frame between —
/// never presses under the first release's pending commit: every input is admitted, no step faults, the node stays
/// selected and the authority ends idle.
#[test]
fn a_double_click_dispatched_in_one_input_drain_keeps_the_authority() {
    let board_id = "board-double-click-one-drain";
    let inner = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let (x, y) = install_click_board(board_id);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    for click in 0..2 {
        puzzle_board_pointer_down_into(board_id, inner, x, y, 0, false, false, &mut input).unwrap_or_else(|fault| panic!("press {click} is admitted: {fault:?}"));
        puzzle_board_pointer_up_into(board_id, "controller", inner, x, y, false, false, false, &mut input).unwrap_or_else(|fault| panic!("release {click} is admitted: {fault:?}"));
    }
    settle_board_pointer_authority_into(board_id, &mut input).expect("the last release's commit reaches its terminal without a fault");
    assert_eq!(with_board_host(board_id, |host| host.selection.iter().cloned().collect::<Vec<_>>()), Some(vec!["node-a".to_string()]), "the node is selected");
    assert_eq!(with_board_host(board_id, |host| (host.pointer_authority_terminal_is_empty(), host.pointer_lane_in_flight())), Some((true, false)), "the authority is idle and no lane is left open");
    remove_click_board(board_id);
}

/// ⚖️ LAW (live fault F14, the wheel twin): a wheel that shares its input drain with the move before it zooms after the
/// move's hover commit reached its terminal — the hover is committed, the camera leaves on the view lane, no board row is
/// dispatched, nothing faults.
#[test]
fn a_wheel_after_a_move_in_one_input_drain_keeps_the_authority() {
    let board_id = "board-wheel-one-drain";
    let inner = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let (x, y) = install_click_board(board_id);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    puzzle_board_pointer_move_into(board_id, "controller", inner, x, y, false, false, false, &mut input).expect("the move onto the node is admitted");
    assert_eq!(with_board_host(board_id, infinite_canvas::BoardHost::pointer_authority_terminal_is_empty), Some(false), "the move's hover commit is pending and unstepped");
    assert_eq!(puzzle_board_wheel_into(board_id, "controller", inner, x, y, -12.0, &mut input), Ok(true), "the wheel settles the pending commit, then zooms");
    settle_board_pointer_authority_into(board_id, &mut input).expect("nothing faults");
    let published = published_board_actions(board_id, &mut input);
    assert_eq!(with_board_host(board_id, |host| host.hovered_id.clone()).flatten().as_deref(), Some("node-a"), "the pending hover was committed before the wheel");
    assert_eq!(published.iter().map(|(action, _)| action.as_str()).collect::<Vec<_>>(), ["setCamera"], "the wheel publishes its camera and a hover no board row: {published:?}");
    assert!(with_board_host(board_id, |host| (host.camera.zoom - 1.0).abs() > 1e-9).unwrap_or(false), "the wheel zoomed");
    assert_eq!(with_board_host(board_id, infinite_canvas::BoardHost::pointer_authority_terminal_is_empty), Some(true), "the authority is idle");
    remove_click_board(board_id);
}
//#endregion 🚦️OneDrainInputLaws

//#region 📬️DeliveryLaws
/// ⚖️ LAW (live fault F24; the shared coalescing corpus): a sealed pointer page dispatches exactly the board rows the corpus
/// leaves for its batch — every transient row and every camera row gone, the rest in order and byte for byte — and
/// dispatches nothing at all when no row is left.
#[test]
fn a_sealed_pointer_page_dispatches_what_the_shared_corpus_leaves() {
    let corpus: Value = serde_json::from_str(BOARD_EVENT_COALESCING_CORPUS).expect("corpus parses");
    let cases = corpus["cases"].as_array().expect("cases");
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let queue = typed_queue(case["rows"].as_array().expect("rows"));
        let mut page = String::from("[");
        let mut rows = Vec::new();
        for (index, event) in queue.iter().enumerate() {
            if index > 0 {
                page.push(',');
            }
            let mut row = String::new();
            event.write_json(&mut row);
            page.push_str(&row);
            rows.push((event.kind(), row));
        }
        page.push(']');
        let dispatched = board_page_dispatch_rows(&page).unwrap_or_else(|fault| panic!("{name}: the page is read: {fault:?}"));
        let expected = case["expect"]["events"].as_array().expect("expected events");
        assert_eq!(dispatched.is_some(), !expected.is_empty(), "{name}: a page with nothing to dispatch claims no action");
        assert_eq!(serde_json::from_str::<Value>(dispatched.as_deref().unwrap_or("[]")).expect("dispatched rows parse"), case["expect"]["events"], "{name}");
        let kept: Vec<&str> = rows.iter().filter(|(kind, _)| *kind != infinite_canvas::BoardEventKind::Camera && !board_event_transient(*kind)).map(|(_, row)| row.as_str()).collect();
        assert_eq!(dispatched.unwrap_or_else(|| "[]".to_string()), format!("[{}]", kept.join(",")), "{name}: the kept rows are the engine's own bytes");
    }
    assert_eq!(board_page_dispatch_rows(""), Ok(None), "a plan without a page dispatches nothing");
    assert_eq!(board_page_dispatch_rows("[]"), Ok(None));
    assert!(board_page_dispatch_rows("[{\"name\":\"noSuchKind\",\"payload\":{}}]").is_err(), "a row of no known kind is refused, never passed on");
    assert!(board_page_dispatch_rows("[{\"name\":\"hover\",\"payload\":{\"id\":\"]\"").is_err(), "an unterminated page is refused");
}

/// ⚖️ LAW (live fault F24): a hover and a click are ONE board batch. The move onto a node commits its hover and dispatches
/// nothing — no `applyBoardEvents`, so no History row; the press dispatches nothing; the release dispatches one
/// `applyBoardEvents` that carries the selection and no transient row.
#[test]
fn a_hover_and_a_click_publish_one_board_batch_and_no_transient_row() {
    let board_id = "board-hover-click-delivery";
    let inner = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let (x, y) = install_click_board(board_id);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(puzzle_board_pointer_move_into(board_id, "controller", inner, x, y, false, false, false, &mut input), Ok(false), "a hover claims no action");
    settle_board_pointer_authority_into(board_id, &mut input).expect("the hover commit reaches its terminal");
    assert_eq!(with_board_host(board_id, |host| host.hovered_id.clone()).flatten().as_deref(), Some("node-a"), "the hover is committed in the engine");
    assert_eq!(published_board_actions(board_id, &mut input), Vec::new(), "a hover dispatches nothing");
    puzzle_board_pointer_down_into(board_id, inner, x, y, 0, false, false, &mut input).expect("the press is admitted");
    assert_eq!(published_board_actions(board_id, &mut input), Vec::new(), "the press stages its selection and dispatches nothing");
    puzzle_board_pointer_up_into(board_id, "controller", inner, x, y, false, false, false, &mut input).expect("the release is admitted");
    settle_board_pointer_authority_into(board_id, &mut input).expect("the release's commit reaches its terminal");
    let published = published_board_actions(board_id, &mut input);
    let batches: Vec<&Vec<String>> = published.iter().filter(|(action, _)| action == "applyBoardEvents").map(|(_, rows)| rows).collect();
    assert_eq!(batches.len(), 1, "the click is one board batch: {published:?}");
    assert!(batches[0].iter().any(|name| name == "select"), "it carries the selection: {published:?}");
    assert!(batches[0].iter().all(|name| name == "select" || name == "gesture"), "and no transient row: {published:?}");
    remove_click_board(board_id);
}

/// ⚖️ LAW (live fault F24, the frame pump): what the engine queues outside a pointer page leaves through the same table —
/// the `hover` a press on another node queues is dropped, nothing is dispatched for it, and the buffer is left empty.
#[test]
fn the_frame_pump_drops_the_transient_rows_a_press_queues() {
    let board_id = "board-frame-pump-delivery";
    let inner = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let (x, y) = install_click_board(board_id);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    puzzle_board_pointer_down_into(board_id, inner, x, y, 0, false, false, &mut input).expect("the press is admitted");
    assert_eq!(with_board_host(board_id, |host| host.hovered_id.clone()).flatten().as_deref(), Some("node-a"), "the press hovers its node");
    assert_eq!(publish_board_event_step(board_id, "controller", &mut input), Ok(false), "the press's hover row is not a board batch");
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "nothing was dispatched");
    assert_eq!(with_board_host(board_id, |host| host.peek_owned_event().is_none()), Some(true), "the engine's queue was drained");
    assert_eq!(ENGINE_SURFACES.with(|cell| cell.borrow().get(board_id).map(|entry| entry.board_pending_events.is_empty())), Some(true), "and the buffer holds no transient row");
    puzzle_board_pointer_up_into(board_id, "controller", inner, x, y, false, false, false, &mut input).expect("the release is admitted");
    settle_board_pointer_authority_into(board_id, &mut input).expect("the release's commit reaches its terminal");
    remove_click_board(board_id);
}
//#endregion 📬️DeliveryLaws
