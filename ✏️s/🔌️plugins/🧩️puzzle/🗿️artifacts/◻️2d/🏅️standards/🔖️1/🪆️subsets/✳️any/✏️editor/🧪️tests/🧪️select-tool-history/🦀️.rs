//! ⏪️ Laws for the select tool's leaves under time travel (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, design §8 and
//! §16), driven by the language-agnostic corpus `🧫️fixtures/🧫️select-tool-history/🔣️.json` that the Python oracle beside
//! this file re-folds with shapely. Every scenario authors its log through the real tool entry points — a board drag
//! through the engine, a board pick, a nudge, the rotate and scale verbs, ingested leaves — edits it in a history session
//! through the reserved `historyEdit*` verbs and finalizes it as an overwrite. Each step's expectation is checked on the
//! app: the panel's inputs (grid-snapped steppers, a dial, a log slider, a reference list with "Use selection"), the board
//! preview (the state before the edited leaf with its draft, downstream not applied) and its highlight of what the draft
//! references, the session band and the per-mutation outcomes. The overwrite equals a fresh fold of the edited log, both
//! through the app and through a standalone store (`state_before`, `begin_report_replay`).

use super::select_tool_transaction_tests::{dispatched_rows, english, flush, german, move_to, painted_host_of, press, release};
use super::*;
use crate::editor::puzzle2d::engine::board_host::unit_tests::context::close_board_host;
use crate::editor::puzzle2d::unit_tests::context::*;
use crate::standards::v1::subsets::any::schema::mutations::binary::{close_puzzle2d_store, puzzle2d_store};
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;
use protocol::{OpBinary, OpText};
use semio_framework::kernel::{HistoryEntry, HistoryTimeTravel, HistoryTimeTravelStage};
use semio_framework::{ActionArgControl, ArgSchema, NumberScale, SnapSource};
use semio_framework_plugin::{ActionMeta, PluginApp, ViewModel, FRAMEWORK_HISTORY_BODY_KEY};

const SELECT_TOOL_HISTORY_CORPUS: &str = include_str!("../../🧫️fixtures/🧫️select-tool-history/🔣️.json");

/// 🧫️ The corpus, its schema tag checked.
pub(super) fn corpus() -> Value {
    let corpus: Value = serde_json::from_str(SELECT_TOOL_HISTORY_CORPUS).expect("the select-tool history corpus parses");
    assert_eq!(corpus["schema"], "s.puzzle2d.select-tool-history.v1");
    corpus
}

/// 🧱️ A registered app holding `board` as its one seed edit.
pub(super) fn seeded_app(board: &Value) -> Puzzle2dApp {
    let mut app = app_with_registry();
    dispatch(&mut app, "importFixture", Some(&json!({ "json": board })), None).expect("seed the board");
    app
}

fn leaf(value: &Value) -> Puzzle2dMutation {
    dsl::json::from_json_str(&value.to_string()).unwrap_or_else(|error| panic!("corpus leaf {value} decodes: {error:?}"))
}

fn ids(value: &Value) -> Vec<String> {
    value.as_array().expect("ids").iter().map(|id| id.as_str().expect("id").to_string()).collect()
}

//#region 🧰️Harness
/// 🪟️ The overview window, focused — the view every history verb and the history panel are read under, so a `Config`
/// snap source reads that window's grid.
fn focused_view() -> ViewModel {
    ViewModel { focused_window_id: Some(overview::WINDOW_KIND_ID.into()), ..window_view(overview::WINDOW_KIND_ID, overview::WINDOW_KIND_ID) }
}

/// ⏪️ One reserved `historyEdit*` verb from the focused overview; a refusal fails the law with its reason.
fn history_edit(app: &mut Puzzle2dApp, verb: &str, args: Value) {
    let meta = ActionMeta { view_state: Some(focused_view()), ..meta("local") };
    let result = block_on(app.handle_action(verb, Some(&dsl::DslValue::from(&args)), &meta)).unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
}

/// 🚦️ The live session as the history wire carries it (`None`: no session).
fn session(app: &mut Puzzle2dApp) -> Option<HistoryTimeTravel> {
    block_on(app.history_snapshot()).expect("history").time_travel
}

/// ⏯️ Drives the session's per-turn work until its stage satisfies `done`.
fn pump(app: &mut Puzzle2dApp, what: &str, done: impl Fn(Option<HistoryTimeTravelStage>) -> bool) {
    for _ in 0..100_000 {
        if done(session(app).map(|status| status.stage)) {
            return;
        }
        block_on(app.advance_typed_operation_publication()).unwrap_or_else(|fault| panic!("{what}: {fault:?}"));
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("{what} never settled: {:?}", session(app));
}

/// 🧾️ The document edit rows, oldest first; a history transition's own row is none of them.
fn edits(app: &mut Puzzle2dApp) -> Vec<HistoryEntry> {
    let mut rows: Vec<HistoryEntry> = block_on(app.history_snapshot()).expect("history").upserts.into_iter().filter(|entry| entry.edit_id.is_some() && entry.transition_id.is_none() && !entry.mutations.is_empty()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// 🖼️ The overview board scene as the window paints it.
fn board_scene(app: &mut Puzzle2dApp) -> Value {
    let body: Value = serde_json::from_str(&render_body(app, overview::BODY_KEY)).expect("board body");
    body["board2d"].clone()
}

/// 🎨️ The document the overview paints — the time-travel preview while a session is open.
fn painted(app: &mut Puzzle2dApp) -> Value {
    serde_json::from_str(board_scene(app)["fixtureJson"].as_str().expect("painted fixture lane")).expect("painted fixture parses")
}

fn node_position(fixture: &Value, id: &str) -> Option<(f64, f64)> {
    fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id)).map(|node| (node["x"].as_f64().expect("x"), node["y"].as_f64().expect("y")))
}

/// 🌲️ The node keyed `key` in a projected tree.
fn find_node<'a>(node: &'a Value, key: &str) -> Option<&'a Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| find_node(child, key))
}

/// 🖲️ The first node under `node` bound to `action`, with that binding — the control a host click dispatches.
fn find_binding<'a>(node: &'a Value, action: &str) -> Option<(&'a Value, &'a Value)> {
    if let Some(binding) = node["bindings"].as_array().into_iter().flatten().find(|binding| binding["action"]["name"].as_str() == Some(action)) {
        return Some((node, binding));
    }
    node["children"].as_array()?.iter().find_map(|child| find_binding(child, action))
}

/// 🔤️ Every text a projected node carries, depth first.
fn texts(node: &Value) -> Vec<&str> {
    match node {
        Value::String(text) => vec![text.as_str()],
        Value::Array(items) => items.iter().flat_map(texts).collect(),
        Value::Object(entries) => entries.values().flat_map(texts).collect(),
        _ => Vec::new(),
    }
}

/// ⚖️ `actual` holds `expected`: every expected key, numbers within 1e-12, id lists in any order.
fn same(actual: &Value, expected: &Value) -> bool {
    match (actual, expected) {
        (Value::Number(a), Value::Number(b)) => (a.as_f64().unwrap_or(f64::NAN) - b.as_f64().unwrap_or(f64::NAN)).abs() <= 1e-12,
        (Value::Array(a), Value::Array(b)) => {
            let sorted = |items: &[Value]| {
                let mut items = items.to_vec();
                items.sort_by_key(Value::to_string);
                items
            };
            let (a, b) = if a.iter().all(Value::is_string) { (sorted(a), sorted(b)) } else { (a.clone(), b.clone()) };
            a.len() == b.len() && a.iter().zip(&b).all(|(a, b)| same(a, b))
        }
        (Value::Object(a), Value::Object(b)) => b.iter().all(|(key, value)| a.get(key).is_some_and(|held| same(held, value))),
        _ => actual == expected,
    }
}

/// 🔣️ A history row's first op as its wire payload.
fn row_leaf(row: &HistoryEntry) -> Value {
    let line = row.op_lines.first().expect("the row prints its op");
    let mutation = <Puzzle2dMutation as OpText>::parse_op(line).unwrap_or_else(|error| panic!("row op {line} parses: {error:?}"));
    serde_json::from_str(&dsl::json::to_json_string(&dsl::ToValue::to_value(&mutation))).expect("leaf payload")
}

/// 🧾️ The row's first mutation as the history wire carries it.
fn row_mutation(row: &HistoryEntry) -> Value {
    serde_json::to_value(row.mutations.first().expect("the row's mutation")).expect("mutation row serializes")
}
//#endregion 🧰️Harness

//#region 🎬️Steps
/// 🖱️ Selects `ids` as nodes of the board's interaction domain (`interactionSelect`), as a pick or a marquee does.
fn select(app: &mut Puzzle2dApp, ids: Vec<String>) {
    let targets: Vec<InteractionTarget> = ids.into_iter().map(|id| InteractionTarget { granularity: PUZZLE2D_GRANULARITY_NODE.into(), id }).collect();
    dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": serde_json::to_string(&targets).expect("targets"), "merge": "replace", "method": "pick" })), None).expect("select");
}

/// ▶️ Runs one corpus step on the app; a drag grabs a selection the app holds, painted into the engine as hosts paint it.
fn run(app: &mut Puzzle2dApp, seed: usize, step: &Value, what: &str) {
    let (action, spec) = step.as_object().expect("step").iter().find(|(key, _)| key.as_str() != "expect").expect("one action per step");
    let point = |value: &Value| (value[0].as_f64().expect("x"), value[1].as_f64().expect("y"));
    match action.as_str() {
        "drag" => {
            select(app, ids(&spec["selection"]));
            let mut host = painted_host_of(&painted(app));
            host.set_selection_ids_silent(&ids(&spec["selection"]));
            let (x, y) = point(&spec["from"]);
            press(&mut host, x, y);
            for via in spec["path"].as_array().expect("path") {
                let (x, y) = point(via);
                move_to(&mut host, x, y);
            }
            let (x, y) = point(&spec["to"]);
            move_to(&mut host, x, y);
            release(&mut host, x, y);
            let result = flush(app, &dispatched_rows(&mut host));
            assert_eq!(committed_edits(&result), 1, "{what}: one drag is one edit");
            close_board_host(host);
        }
        "pick" => {
            let fixture = painted(app);
            let id = spec.as_str().expect("picked id");
            let (x, y) = node_position(&fixture, id).unwrap_or_else(|| panic!("{what}: {id} is painted"));
            let mut host = painted_host_of(&fixture);
            press(&mut host, x, y);
            release(&mut host, x, y);
            let result = flush(app, &dispatched_rows(&mut host));
            assert_eq!(committed_edits(&result), 0, "{what}: a pick edits nothing");
            assert!(!result.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "{what}: a pick is never frozen");
            close_board_host(host);
        }
        "select" => select(app, ids(spec)),
        "translate" | "rotate" | "scale" => {
            let verb = match action.as_str() {
                "translate" => "translateSelection",
                "rotate" => "rotateSelection",
                _ => "scaleSelection",
            };
            let result = dispatch(app, verb, Some(spec), None).unwrap_or_else(|fault| panic!("{what}: {verb}: {fault:?}"));
            assert_eq!(committed_edits(&result), 1, "{what}: {verb} is one edit");
        }
        "ingest" => block_on(app.ingest_operations_text(&<Puzzle2dMutation as OpText>::print_op(&leaf(spec)))).unwrap_or_else(|fault| panic!("{what}: ingest: {fault:?}")),
        "gridFactor" => {
            dispatch(app, "setGridFactor", Some(&json!({ "value": spec })), Some(overview::WINDOW_KIND_ID)).expect("grid factor");
        }
        "begin" => {
            let mutation = match spec.as_str() {
                Some("nextProblem") => app.time_travel_ledger().panel().and_then(|panel| panel.next_problem.clone()).unwrap_or_else(|| panic!("{what}: a next problem")),
                _ => edits(app)[seed + spec["row"].as_u64().expect("row") as usize].mutations[0].mutation_id.clone(),
            };
            history_edit(app, "historyEditBegin", json!({ "mutationId": mutation }));
        }
        "input" => history_edit(app, "historyEditInput", json!({ "path": spec["path"], "value": spec["value"] })),
        "useSelection" => {
            let pointer = spec.as_str().expect("the reference input's path");
            let history: Value = serde_json::from_str(&render_body_with_view(app, FRAMEWORK_HISTORY_BODY_KEY, &focused_view())).expect("history body");
            let key = format!("framework.history.editor.input{}.row", pointer.replace('/', "."));
            let control = find_node(&history, &key).unwrap_or_else(|| panic!("{what}: the history body renders {key}"));
            let (button, binding) = find_binding(control, "historyEditUseSelection").unwrap_or_else(|| panic!("{what}: {key} offers Use selection: {control}"));
            assert_ne!(button["disabled"], Value::Bool(true), "{what}: Use selection is enabled for a reference into the board's domain: {button}");
            assert_eq!(binding["args"]["path"].as_str(), Some(pointer), "{what}: Use selection drafts {pointer}: {binding}");
            history_edit(app, "historyEditUseSelection", binding["args"].clone());
        }
        "withdraw" => history_edit(app, "historyEditWithdraw", json!({})),
        "accept" => {
            history_edit(app, "historyEditAccept", json!({}));
            pump(app, what, |stage| stage != Some(HistoryTimeTravelStage::Replaying));
        }
        "finalize" => {
            history_edit(app, "historyEditFinalize", json!({}));
            history_edit(app, "historyEditCommit", json!({ "choice": spec }));
            pump(app, what, |stage| stage.is_none());
        }
        other => panic!("{what}: unknown corpus step {other}"),
    }
}

/// 🔍️ Checks one step's expectation on the app.
fn check(app: &mut Puzzle2dApp, seed: usize, expect: &Value, what: &str) {
    let rows: Vec<HistoryEntry> = edits(app).split_off(seed);
    if let Some(count) = expect["rows"].as_u64() {
        assert_eq!(rows.len() as u64, count, "{what}: rows {:?}", rows.iter().map(|row| &row.op_lines).collect::<Vec<_>>());
    }
    if let Some(row) = expect.get("row") {
        let entry = &rows[row["index"].as_u64().expect("row index") as usize];
        assert!(same(&row_leaf(entry), &row["leaf"]), "{what}: the row's leaf {} is {}", row_leaf(entry), row["leaf"]);
        let transaction = entry.transaction.as_ref().unwrap_or_else(|| panic!("{what}: the row carries its tool transaction"));
        if let Some(tool) = row["tool"].as_str() {
            assert_eq!(transaction.tool, tool, "{what}: the authoring tool");
        }
        if let Some(label) = row.get("label") {
            assert_eq!((english(entry), german(entry)), (label["en"].as_str().expect("en").to_string(), label["de"].as_str().expect("de").to_string()), "{what}: the row is labelled from its leaf");
        }
    }
    let status = session(app).map(|status| serde_json::to_value(status).expect("status serializes"));
    if let Some(stage) = expect.get("stage") {
        assert_eq!(status.as_ref().map_or(&Value::Null, |status| &status["stage"]), stage, "{what}: stage of {status:?}");
    }
    for key in ["review", "blocking", "worst"] {
        if let Some(wanted) = expect.get(key) {
            assert_eq!(status.as_ref().map(|status| &status[key]), Some(wanted), "{what}: {key} of {status:?}");
        }
    }
    for outcome in expect["outcomes"].as_array().into_iter().flatten() {
        let mutation = row_mutation(&rows[outcome["row"].as_u64().expect("outcome row") as usize]);
        assert_eq!(mutation["worst"], outcome["worst"], "{what}: worst of {mutation}");
        if let Some(introduced) = outcome.get("introduced") {
            assert_eq!(&mutation["introduced"], introduced, "{what}: whether the edit made the outcome new: {mutation}");
        }
        assert!(mutation["messages"].as_array().into_iter().flatten().any(|message| message["code"] == outcome["code"] && same(&message["target"], &outcome["target"])), "{what}: {} at {} in {mutation}", outcome["code"], outcome["target"]);
    }
    if let Some(problem) = expect.get("nextProblem") {
        let next = app.time_travel_ledger().panel().and_then(|panel| panel.next_problem.clone());
        assert_eq!(next.as_deref(), Some(rows[problem["row"].as_u64().expect("problem row") as usize].mutations[0].mutation_id.as_str()), "{what}: the next problem");
    }
    if let Some(inputs) = expect["inputs"].as_array() {
        let panel = app.time_travel_ledger().panel().and_then(|panel| panel.editor).unwrap_or_else(|| panic!("{what}: the draft editor"));
        let history: Value = serde_json::from_str(&render_body_with_view(app, FRAMEWORK_HISTORY_BODY_KEY, &focused_view())).expect("history body");
        for input in inputs {
            let path = input["path"].as_str().expect("input path");
            let row = panel.rows.iter().find(|row| row.pointer == path).unwrap_or_else(|| panic!("{what}: the editor shows {path} among {:?}", panel.rows.iter().map(|row| &row.pointer).collect::<Vec<_>>()));
            let control = row.input.control();
            let kind = match &control {
                ActionArgControl::Reference { .. } => "reference",
                ActionArgControl::Stepper { .. } => "stepper",
                ActionArgControl::Dial { .. } => "dial",
                ActionArgControl::Slider { .. } => "slider",
                _ => "other",
            };
            assert_eq!(kind, input["control"], "{what}: {path} renders as {control:?}");
            if let ArgSchema::Reference { domain, granularity, many, .. } = &row.input.schema {
                assert_eq!((domain.as_deref(), granularity.as_deref(), *many), (input["domain"].as_str(), input["granularity"].as_str(), input["many"].as_bool().unwrap_or(*many)), "{what}: {path} picks from the board selection");
            }
            if let Some(key) = input["snapSource"].as_str() {
                assert!(matches!(&row.input.schema, ArgSchema::Number { snap_source: Some(SnapSource::Config { key: source }), .. } if source == key), "{what}: {path} snaps to the {key} config: {:?}", row.input.schema);
            }
            if let Some(scale) = input["scale"].as_str() {
                assert!(matches!(&row.input.schema, ArgSchema::Number { scale: Some(NumberScale::Log), .. }) == (scale == "log"), "{what}: {path} maps on a {scale} scale: {:?}", row.input.schema);
            }
            if let Some(snaps) = input.get("snaps") {
                let ArgSchema::Number { snaps: declared, .. } = &row.input.schema else { panic!("{what}: {path} is a number") };
                assert!(same(&json!(declared), snaps), "{what}: {path} snaps {declared:?}");
            }
            if let Some(step) = input.get("step") {
                let key = format!("framework.history.editor.input{}", path.replace('/', "."));
                let control = find_node(&history, &key).unwrap_or_else(|| panic!("{what}: the history body renders {key}"));
                assert!(same(&control["component"]["step"], step), "{what}: {key} steps by the window's grid: {}", control["component"]);
            }
            if let Some(component) = input.get("component") {
                let key = format!("framework.history.editor.input{}", path.replace('/', "."));
                let control = find_node(&history, &key).unwrap_or_else(|| panic!("{what}: the history body renders {key}"));
                assert!(same(&control["component"], component), "{what}: {key} renders every facet of its descriptor: {} holds {component}", control["component"]);
            }
        }
    }
    if let Some(preview) = expect["preview"].as_object() {
        let fixture = painted(app);
        for (id, point) in preview {
            let painted = node_position(&fixture, id).unwrap_or_else(|| panic!("{what}: the preview paints {id}"));
            assert!(same(&json!([painted.0, painted.1]), point), "{what}: the preview paints {id} at {painted:?}, the corpus says {point}");
        }
    }
    for id in expect["absent"].as_array().into_iter().flatten() {
        assert!(node_position(&painted(app), id.as_str().expect("absent id")).is_none(), "{what}: the preview never paints {id}");
    }
    if let Some(highlighted) = expect.get("highlighted") {
        let scene = board_scene(app);
        let painted: Value = serde_json::from_str(scene["highlightedIdsJson"].as_str().unwrap_or("[]")).expect("highlighted ids parse");
        assert!(same(&painted, highlighted), "{what}: the board highlights {painted}, the draft references {highlighted}");
    }
    if let Some(chips) = expect["chips"].as_object() {
        let history: Value = serde_json::from_str(&render_body_with_view(app, FRAMEWORK_HISTORY_BODY_KEY, &focused_view())).expect("history body");
        for (pointer, labels) in chips {
            let key = format!("framework.history.editor.input{}", pointer.replace('/', "."));
            let control = find_node(&history, &key).unwrap_or_else(|| panic!("{what}: the history body renders {key}"));
            let shown = texts(control);
            for label in labels.as_array().expect("chip labels") {
                assert!(shown.contains(&label.as_str().expect("chip label")), "{what}: a {key} chip reads {label}: {shown:?}");
            }
        }
    }
    if let Some(draft) = expect["draft"].as_object() {
        let value = Value::from(&app.time_travel_ledger().editor().expect("the draft editor").value);
        for (pointer, wanted) in draft {
            assert!(value.pointer(pointer).is_some_and(|held| same(held, wanted)), "{what}: the draft holds {wanted} at {pointer}: {value}");
        }
    }
}
//#endregion 🎬️Steps

//#region ⏪️Laws
/// ⚖️ LAW: every corpus scenario runs through the app — one gesture, one row from its leaf with its transaction; time travel
/// on a leaf exposes its inputs from the leaf schema with the window's grid snaps and the board's selection; the preview is
/// the state before the edited leaf with its draft, downstream not applied, and the board highlights what the draft
/// references; an upstream edit that makes a downstream leaf fail blocks finalizing until the failing leaf is edited, and
/// a warning never does but stays on its row; the overwrite reaches the corpus head, exactly the document a fresh app
/// folding the edited log reaches.
#[test]
fn every_corpus_scenario_edits_its_leaves_in_history_and_overwrites_to_a_fresh_fold() {
    let corpus = corpus();
    for scenario in corpus["scenarios"].as_array().expect("scenarios") {
        let id = scenario["id"].as_str().expect("scenario id");
        let mut app = seeded_app(&corpus["board"]);
        let seed = edits(&mut app).len();
        for (index, step) in scenario["steps"].as_array().expect("steps").iter().enumerate() {
            let what = format!("{id} step {index}");
            run(&mut app, seed, step, &what);
            if let Some(expect) = step.get("expect") {
                check(&mut app, seed, expect, &what);
            }
        }
        let head = fixture_of(&app);
        for (node, expected) in scenario["head"].as_object().expect("head") {
            let record = fixture_nodes(&head).iter().find(|record| record["id"] == node.as_str()).unwrap_or_else(|| panic!("{id}: {node} is in the head"));
            assert_eq!((record["x"].as_f64(), record["y"].as_f64(), record["locked"].as_bool().unwrap_or(false)), (expected["x"].as_f64(), expected["y"].as_f64(), expected["locked"].as_bool().unwrap_or(false)), "{id}: {node} in the overwritten head");
        }
        let mut fresh = seeded_app(&corpus["board"]);
        for leaf_value in edited_log(scenario) {
            block_on(fresh.ingest_operations_text(&<Puzzle2dMutation as OpText>::print_op(&leaf(&leaf_value)))).expect("the edited log folds");
        }
        assert_eq!(fixture_nodes(&head), fixture_nodes(&fixture_of(&fresh)), "{id}: the overwrite equals a fresh fold of the edited log");
        close_app(&mut app);
        close_app(&mut fresh);
    }
}

/// 🪧️ LAW (design §16.4): a reference chip names its board entity as the outliner does — a node by its text, a target
/// region by its label, an edge by its endpoint nodes, a handle by its node and kind — for the declared kinds only, as
/// data in every locale; an entity only its id would name, or one the document lacks, reads `None` and shows its id.
#[test]
fn reference_chips_name_board_entities_like_the_outliner() {
    let fixture = json!({
        "nodes": [
            { "id": "a", "x": 0.0, "y": 0.0, "text": "Alpha", "handles": [{ "id": "a:h", "handleKind": "door" }] },
            { "id": "b", "x": 0.0, "y": 0.0, "handles": [{ "id": "b:h", "handleKind": "door" }] }
        ],
        "edges": [{ "id": "e", "source": "a:h", "target": "b:h" }],
        "targetRegions": [{ "id": "r", "label": "Courtyard" }, { "id": "s" }]
    });
    let label = |kinds: &[&str], id: &str| {
        let kinds: Vec<String> = kinds.iter().map(|kind| kind.to_string()).collect();
        puzzle2d_entity_label(&fixture, &kinds, id).map(|label| (label.resolve(protocol::Terminology::Native, protocol::Locale::En).to_string(), label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_string()))
    };
    let both = |text: &str| Some((text.to_string(), text.to_string()));
    assert_eq!(label(&["node", "targetRegion"], "a"), both("Alpha"));
    assert_eq!(label(&[], "a"), both("Alpha"), "no declared kind admits every kind");
    assert_eq!(label(&["node"], "b"), None, "a node only its id names shows the id");
    assert_eq!(label(&["node", "targetRegion"], "r"), both("Courtyard"));
    assert_eq!(label(&["node", "targetRegion"], "s"), None);
    assert_eq!(label(&["node"], "r"), None, "an undeclared kind is never looked up");
    assert_eq!(label(&["edge"], "e"), both("Alpha → b"));
    assert_eq!(label(&["handle"], "a:h"), both("Alpha \u{b7} door"));
    assert_eq!(label(&["node"], "ghost"), None, "an entity the document lacks shows its id");
}

/// ✏️ The scenario's edited log: every logged leaf with its draft, a withdrawn one dropped.
fn edited_log(scenario: &Value) -> Vec<Value> {
    let drafts = scenario["drafts"].as_array().expect("drafts");
    scenario["log"].as_array().expect("log").iter().enumerate().filter_map(|(index, logged)| match drafts.iter().find(|draft| draft["index"].as_u64() == Some(index as u64)) {
        Some(draft) => draft.get("leaf").cloned(),
        None => Some(logged.clone()),
    }).collect()
}

/// ⚖️ LAW (design §3, the siblings' store law): through a standalone store, every corpus log edited with its drafts
/// previews as the state right before the first edited leaf with that draft applied, replays its downstream in Report
/// mode to exactly the fresh fold of the edited log with the corpus report and never a blocking one, and overwrites to
/// that fold.
#[semio_framework_async_macros::async_test]
async fn every_corpus_edit_previews_and_replays_through_the_store() {
    let corpus = corpus();
    let board: Puzzle2dSnapshot = dsl::json::from_json_str(&corpus["board"].to_string()).expect("the corpus board decodes");
    for scenario in corpus["scenarios"].as_array().expect("scenarios") {
        let id = scenario["id"].as_str().expect("scenario id");
        let mut store = puzzle2d_store(store::create_document_envelope::<Puzzle2dSnapshot, Puzzle2dMutation>(crate::PUZZLE_2D_SCHEMA, id, board.clone(), None)).await.expect("the store opens");
        for logged in scenario["log"].as_array().expect("log") {
            store.dispatch(store::ArtifactCommand::Apply { mutations: vec![leaf(logged)], description: None, transaction: None }).await.expect("a logged leaf applies");
        }
        let mutation_ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
        let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = scenario["drafts"]
            .as_array()
            .expect("drafts")
            .iter()
            .map(|draft| {
                let replacement = match draft.get("leaf") {
                    Some(replaced) => protocol::InputReplacement::Input { schema: crate::PUZZLE_2D_SCHEMA.into(), payload: leaf(replaced).encode_op().expect("the draft encodes") },
                    None => protocol::InputReplacement::Withdrawn,
                };
                (mutation_ids[draft["index"].as_u64().expect("draft index") as usize].clone(), replacement)
            })
            .collect();
        let first = scenario["drafts"].as_array().expect("drafts").iter().filter_map(|draft| draft["index"].as_u64()).min().expect("a draft") as usize;
        let edited: Vec<Puzzle2dMutation> = edited_log(scenario).iter().map(leaf).collect();
        let fold = |count: usize| {
            let mut state = board.clone();
            for mutation in scenario["log"].as_array().expect("log").iter().take(count).map(leaf) {
                apply_puzzle2d_mutation(&mut state, &mutation).expect("the log folds");
            }
            state
        };
        let preview = store.state_before(&mutation_ids[first], &drafts).expect("the preview base folds").as_ref().clone();
        assert_eq!(preview, fold(first), "{id}: the preview base is the state right before the first edited leaf");
        let mut replay = store.begin_report_replay(&drafts, Some(&mutation_ids[first])).expect("the replay begins at the first edited leaf");
        let mut finished = false;
        for _ in 0..1_024 {
            if matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)) {
                finished = true;
                break;
            }
        }
        assert!(finished, "{id}: the replay finishes");
        let result = replay.finish().expect("a finished replay yields its result");
        let report = store.replay_report(&result).expect("report");
        assert!(!report.blocks_finalize(), "{id}: the drafted log replays clean of errors: {report:?}");
        for expected in scenario["report"].as_array().into_iter().flatten() {
            let outcome = report.outcomes.iter().find(|outcome| outcome.mutation_id == mutation_ids[expected["index"].as_u64().expect("report index") as usize]).unwrap_or_else(|| panic!("{id}: the report names {expected}"));
            assert_eq!(outcome.worst.map(|worst| format!("{worst:?}").to_lowercase()).as_deref(), expected["worst"].as_str(), "{id}: {outcome:?}");
            assert!(outcome.messages.iter().any(|message| message.code.0 == expected["code"].as_str().expect("code") && same(&json!(message.target), &expected["target"])), "{id}: {outcome:?}");
        }
        let mut fresh = board.clone();
        for mutation in &edited {
            apply_puzzle2d_mutation(&mut fresh, mutation).expect("the edited log folds");
        }
        if let Some(head) = scenario["head"].as_object() {
            for (node, expected) in head {
                let record = fresh.nodes.iter().find(|record| record.id == *node).unwrap_or_else(|| panic!("{id}: {node} folds"));
                assert_eq!((record.x, record.y), (expected["x"].as_f64().expect("x"), expected["y"].as_f64().expect("y")), "{id}: {node} of the fresh fold is the corpus head");
            }
        }
        assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "{id}: the replay equals the fresh fold of the edited log");
        store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
        assert_eq!(store.snapshot_ref(), &fresh, "{id}: the overwritten history folds to the edited state");
        close_puzzle2d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
    }
}
//#endregion ⏪️Laws
