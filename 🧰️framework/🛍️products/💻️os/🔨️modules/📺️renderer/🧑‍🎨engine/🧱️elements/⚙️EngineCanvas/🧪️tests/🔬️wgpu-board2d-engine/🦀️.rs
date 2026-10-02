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

/// ⚖️ Law: every corpus case coalesces here exactly as React coalesces it — same dispatched rows in the same
/// order, same flush verdict — so one drag reaches the guest as ONE batch on both hosts.
#[test]
fn the_wgpu_coalescer_replays_the_shared_corpus() {
    let corpus: Value = serde_json::from_str(BOARD_EVENT_COALESCING_CORPUS).expect("corpus parses");
    let cases = corpus["cases"].as_array().expect("cases");
    assert!(cases.len() >= 10, "the corpus covers every row family");
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let coalesced = coalesce_owned_board_events(&typed_queue(case["rows"].as_array().expect("rows"))).expect("corpus batch coalesces");
        assert_eq!(serde_json::from_str::<Value>(&coalesced.events_json).expect("dispatched rows parse"), case["expect"]["events"], "{name}");
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
    let fixture = |x: f64| json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [node("left", -x), node("mid", x)], "edges": [] }).to_string();
    let mut board = ui_wgpu::wgpu::Board2dScene::base(fixture(40.0), json!({ "x": 0.0, "y": 0.0, "zoom": 1.0 }).to_string(), true);
    board.highlighted_ids_json = json!(["mid", "left"]).to_string();
    let (mut host, mut cache) = (infinite_canvas::BoardHost::default(), BoardSyncCache::default());
    assert!(sync_board_engine(&mut host, &mut cache, &board, 800, 600), "the first scene syncs");
    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["left","mid"]"#);
    assert!(!sync_board_engine(&mut host, &mut cache, &board, 800, 600), "an unchanged scene re-applies nothing");
    board.fixture_json = fixture(80.0);
    assert!(sync_board_engine(&mut host, &mut cache, &board, 800, 600), "a new preview syncs");
    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["left","mid"]"#, "a new preview keeps what the draft references");
    board.highlighted_ids_json = "[]".into();
    assert!(sync_board_engine(&mut host, &mut cache, &board, 800, 600), "a closed draft syncs");
    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), "[]", "a closed draft highlights nothing");
}
