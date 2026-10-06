//! ⏪️ Runtime acceptance laws of non-destructive history editing, with puzzle 2d as the witness of the generic runtime
//! (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §4, §7, §16). They are driven by the language-agnostic corpus
//! `🧫️fixtures/🧫️history-edit-runtime/🔣️.json`; the Python oracle beside this file re-folds every named head of it.
//!
//! What the laws prove, all through the reserved `historyEdit*` verbs:
//! - Several drafts accepted from a review finalize as ONE overwrite.
//! - A replay is cancelled and then run again.
//! - A Warning that an edit introduces stays on its row after finalize, after a text reload and after a pack reload.
//! - Fatal and Error outcomes block finalizing while Next problem walks them. They are resolved by withdrawing and by
//!   editing targets through "Use selection", then finalized as an overwrite and as a new alternative, each undone and redone.
//! - A drag whose drop recorded a proximity connection is edited far away: the connection still lands, carries a NEW
//!   `mutation.precondition-drifted` Warning that names both handles, the review paints it, and withdrawing the connection
//!   leaves a clean review (design §22.13).
//! - Every head equals a fresh app folding the edited log, and that fold reports exactly the outcomes the head states.
//!
//! Two laws need many operations and two replicas, so they run outside the corpus:
//! - Replay progress rides the UI frames.
//! - A long remote history change replays over driver turns, pauses on cancel and resumes.

use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use protocol::OpText;
use semio_framework::kernel::{HistoryEntry, HistoryTimeTravel, HistoryTimeTravelStage};
use semio_framework_plugin::{ActionMeta, PluginApp, ViewModel, FRAMEWORK_HISTORY_BODY_KEY};
use store::{Backbone, BackboneMessage, MemoryBackbone};

const HISTORY_EDIT_RUNTIME_CORPUS: &str = include_str!("../../🧫️fixtures/🧫️history-edit-runtime/🔣️.json");

/// 🧫️ The corpus, its schema tag checked.
fn corpus() -> Value {
    let corpus: Value = serde_json::from_str(HISTORY_EDIT_RUNTIME_CORPUS).expect("the history-edit runtime corpus parses");
    assert_eq!(corpus["schema"], "s.puzzle2d.history-edit-runtime.v1");
    corpus
}

fn leaf(value: &Value) -> Puzzle2dMutation {
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("corpus leaf {value} decodes: {error:?}"))
}

fn ids(value: &Value) -> Vec<String> {
    value.as_array().expect("ids").iter().map(|id| id.as_str().expect("id").to_string()).collect()
}

//#region 🧰️Harness
/// 🧱️ A registered app holding `board` as its one seed edit.
fn seeded_app(board: &Value) -> Puzzle2dApp {
    let mut app = app_with_registry();
    dispatch(&mut app, "importSnapshot", Some(&json!({ "payload": board })), None).expect("seed the board");
    app
}

/// 🪟️ The overview window, focused — the view every history verb and the history body are read under.
fn focused_view() -> ViewModel {
    ViewModel { focused_window_id: Some(overview::WINDOW_KIND_ID.into()), ..window_view(overview::WINDOW_KIND_ID, overview::WINDOW_KIND_ID) }
}

/// ⏪️ One reserved verb from the focused overview; a refusal fails the law with its reason.
fn history_edit(app: &mut Puzzle2dApp, verb: &str, args: Value, what: &str) {
    let meta = ActionMeta { view_state: Some(focused_view()), ..meta("local") };
    let result = block_on(app.handle_action(verb, Some(&semio_framework_value::DslValue::from(&args)), &meta)).unwrap_or_else(|fault| panic!("{what}: {verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{what}: {verb} was refused: {:?}", result.output);
}

/// 🚦️ The live session as the history wire carries it (`None`: no session).
fn session(app: &mut Puzzle2dApp) -> Option<HistoryTimeTravel> {
    block_on(app.history_snapshot()).expect("history").time_travel
}

/// ⏯️ Driver turns until `done` holds.
fn pump(app: &mut Puzzle2dApp, what: &str, done: impl Fn(&mut Puzzle2dApp) -> bool) {
    for _ in 0..100_000 {
        if done(app) {
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

/// 🌿️ The history-edit rows (one per `Supersede` transition), newest first, in English and German.
fn history_edit_labels(app: &mut Puzzle2dApp) -> Vec<(String, String)> {
    let mut rows: Vec<HistoryEntry> = block_on(app.history_snapshot()).expect("history").upserts.into_iter().filter(|entry| entry.transition_id.is_some()).collect();
    rows.sort_by_key(|entry| std::cmp::Reverse(entry.seq));
    rows.iter().map(|row| (row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string(), row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).to_string())).collect()
}

/// 🌲️ The node keyed `key` in a projected tree.
fn find_node<'a>(node: &'a Value, key: &str) -> Option<&'a Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| find_node(child, key))
}

/// 📜️ The history body as the focused overview renders it in `locale`.
fn history_body(app: &mut Puzzle2dApp, locale: semio_framework_ui_locale::Locale) -> Value {
    serde_json::from_str(&render_body_with_view(app, FRAMEWORK_HISTORY_BODY_KEY, &ViewModel { locale, ..focused_view() })).expect("history body")
}

/// 🖱️ Selects `ids` as nodes of the board's interaction domain (`interactionSelect`), as a pick or a marquee does.
fn select(app: &mut Puzzle2dApp, ids: Vec<String>) {
    let targets: Vec<InteractionTarget> = ids.into_iter().map(|id| InteractionTarget { granularity: PUZZLE2D_GRANULARITY_NODE.into(), id }).collect();
    dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": serde_json::to_string(&targets).expect("targets"), "merge": "replace", "method": "pick" })), None).expect("select");
}

/// ↩️ A framework undo or redo, driven until a resumable history-edit authoring it started has landed.
fn history_lane(app: &mut Puzzle2dApp, verb: &str, what: &str) {
    dispatch(app, verb, None, None).unwrap_or_else(|fault| panic!("{what}: {verb}: {fault:?}"));
    pump(app, what, |app| !app.time_travel_ledger().authoring_pending());
}

/// 🔁️ `app` replaced by a fresh app that loaded its document as text or as a pack.
fn reload(app: &mut Puzzle2dApp, form: &str, what: &str) {
    let mut reloaded = app_with_registry();
    match form {
        "text" => block_on(semio_framework_plugin::app::artifact_app_laws::load_document_text(&mut reloaded, &block_on(app.document_text()).expect("document text"))).unwrap_or_else(|fault| panic!("{what}: text reload: {fault:?}")),
        _ => block_on(semio_framework_plugin::app::artifact_app_laws::load_document(&mut reloaded, &block_on(app.document_pack()).expect("document pack"))).unwrap_or_else(|fault| panic!("{what}: pack reload: {fault:?}")),
    }
    let mut previous = std::mem::replace(app, reloaded);
    close_app(&mut previous);
}

/// 🎲️ The board a scenario runs on: its own, else the corpus board.
fn board<'a>(corpus: &'a Value, scenario: &'a Value) -> &'a Value {
    scenario.get("board").unwrap_or(&corpus["board"])
}

/// ✏️ The scenario's log with `head`'s edits applied, every leaf with its index in the log: a replaced leaf takes its
/// slot, a withdrawn one is dropped.
fn edited_log(scenario: &Value, head: &Value) -> Vec<(usize, Value)> {
    let edits = head["edits"].as_array().expect("edits");
    scenario["log"].as_array().expect("log").iter().enumerate().filter_map(|(index, logged)| match edits.iter().find(|edit| edit["index"].as_u64() == Some(index as u64)) {
        Some(edit) => edit.get("leaf").cloned().map(|leaf| (index, leaf)),
        None => Some((index, logged.clone())),
    }).collect()
}

/// 📍️ `document` places the named corpus head: its nodes where the corpus puts them, its absent ids absent and, where
/// the head states them, exactly its edges.
fn check_placement(document: &Value, head: &Value, name: &str, what: &str) {
    let nodes = board_snapshot_nodes(document);
    for (id, expected) in head["nodes"].as_object().expect("head nodes") {
        let node = nodes.iter().find(|node| node["id"] == id.as_str()).unwrap_or_else(|| panic!("{what}: {id} is in the {name} head"));
        assert_eq!((node["x"].as_f64(), node["y"].as_f64(), node["locked"].as_bool().unwrap_or(false)), (expected["x"].as_f64(), expected["y"].as_f64(), expected["locked"].as_bool().unwrap_or(false)), "{what}: {id} in the {name} head");
    }
    for id in head["absent"].as_array().into_iter().flatten() {
        assert!(nodes.iter().all(|node| node["id"] != *id), "{what}: {id} is absent from the {name} head");
    }
    if let Some(edges) = head.get("edges") {
        let held: std::collections::BTreeSet<&str> = board_snapshot_edges(document).iter().filter_map(|edge| edge["id"].as_str()).collect();
        assert_eq!(held, edges.as_array().expect("head edges").iter().filter_map(Value::as_str).collect(), "{what}: the edges of the {name} head");
    }
}

/// 🌱️ The document a fresh app holds after folding the named head's edited log over the scenario's board. Where the head
/// states `outcomes`, that fold reported exactly those messages: the leaf's index in the log, the code and the target.
fn folded_head(corpus: &Value, scenario: &Value, name: &str, what: &str) -> Value {
    let head = &scenario["heads"][name];
    let mut fresh = seeded_app(board(corpus, scenario));
    let seed = edits(&mut fresh).len();
    let log = edited_log(scenario, head);
    for (_, logged) in &log {
        block_on(fresh.ingest_operations_text(&leaf(logged).print_op())).unwrap_or_else(|fault| panic!("{what}: the edited log folds: {fault:?}"));
    }
    if let Some(outcomes) = head.get("outcomes") {
        let rows = edits(&mut fresh).split_off(seed);
        assert_eq!(rows.len(), log.len(), "{what}: every leaf of the {name} head's edited log is one edit");
        let reported: Vec<Value> = rows.iter().zip(&log).flat_map(|(row, (index, _))| row.mutations.iter().flat_map(|mutation| &mutation.messages).map(|message| json!({ "index": index, "code": message.code, "target": message.target })).collect::<Vec<_>>()).collect();
        assert_eq!(&Value::Array(reported), outcomes, "{what}: the outcomes the {name} head's edited log folds to");
    }
    let document = fixture_of(&fresh);
    close_app(&mut fresh);
    document
}

/// 🎯️ The app's head is the named corpus head: placed as the corpus states, and every node and edge record exactly what a
/// fresh app folding the head's edited log holds.
fn check_head(app: &mut Puzzle2dApp, corpus: &Value, scenario: &Value, name: &str, what: &str) {
    let document = fixture_of(app);
    check_placement(&document, &scenario["heads"][name], name, what);
    let folded = folded_head(corpus, scenario, name, what);
    assert_eq!(board_snapshot_nodes(&document), board_snapshot_nodes(&folded), "{what}: the {name} head equals a fresh fold of its edited log");
    assert_eq!(board_snapshot_edges(&document), board_snapshot_edges(&folded), "{what}: the edges of the {name} head equal a fresh fold of its edited log");
}

/// 🖼️ The review paints the named corpus head: placed as the corpus states, every node where a fresh app folding the
/// head's edited log holds it.
fn check_preview(app: &mut Puzzle2dApp, corpus: &Value, scenario: &Value, name: &str, what: &str) {
    let painted = painted_fixture(app);
    check_placement(&painted, &scenario["heads"][name], name, what);
    let folded = folded_head(corpus, scenario, name, what);
    let places = |document: &Value| board_snapshot_nodes(document).iter().map(|node| (node["id"].clone(), node["x"].as_f64(), node["y"].as_f64())).collect::<Vec<_>>();
    assert_eq!(places(&painted), places(&folded), "{what}: the painted {name} preview places every node where a fresh fold of its edited log does");
}
//#endregion 🧰️Harness

//#region 🎬️Steps
/// ▶️ Runs one corpus step through the real verbs.
fn run(app: &mut Puzzle2dApp, seed: usize, step: &Value, what: &str) {
    let (action, spec) = step.as_object().expect("step").iter().find(|(key, _)| key.as_str() != "expect").expect("one action per step");
    match action.as_str() {
        "translate" => {
            select(app, ids(&spec["select"]));
            let result = dispatch(app, "translateSelection", Some(&json!({ "dx": spec["dx"], "dy": spec["dy"], "step": spec["step"] })), None).unwrap_or_else(|fault| panic!("{what}: translate: {fault:?}"));
            assert_eq!(committed_edits(&result), 1, "{what}: a nudge is one edit");
        }
        "ingest" => block_on(app.ingest_operations_text(&leaf(spec).print_op())).unwrap_or_else(|fault| panic!("{what}: ingest: {fault:?}")),
        "select" => select(app, ids(spec)),
        "begin" => {
            let mutation = match spec.as_str() {
                Some("nextProblem") => app.time_travel_ledger().panel().and_then(|panel| panel.next_problem.clone()).unwrap_or_else(|| panic!("{what}: a next problem")),
                _ => edits(app)[seed + spec["row"].as_u64().expect("row") as usize].mutations[spec["mutation"].as_u64().unwrap_or(0) as usize].mutation_id.clone(),
            };
            history_edit(app, "historyEditBegin", json!({ "mutationId": mutation }), what);
        }
        "input" => history_edit(app, "historyEditInput", json!({ "path": spec["path"], "value": spec["value"] }), what),
        "useSelection" => history_edit(app, "historyEditUseSelection", json!({ "path": spec }), what),
        "withdraw" => history_edit(app, "historyEditWithdraw", json!({}), what),
        "accept" => {
            history_edit(app, "historyEditAccept", json!({}), what);
            pump(app, what, |app| session(app).map(|status| status.stage) != Some(HistoryTimeTravelStage::Replaying));
        }
        "acceptPending" => history_edit(app, "historyEditAccept", json!({}), what),
        "cancelReplay" => history_edit(app, "historyEditCancelReplay", json!({}), what),
        "rerun" => {
            history_edit(app, "historyEditRerun", json!({}), what);
            pump(app, what, |app| session(app).map(|status| status.stage) != Some(HistoryTimeTravelStage::Replaying));
        }
        "exit" => {
            history_edit(app, "historyEditExit", json!({}), what);
            pump(app, what, |app| !app.time_travel_ledger().has_pending_work());
        }
        "finalize" => {
            history_edit(app, "historyEditFinalize", json!({}), what);
            let commit = match spec.as_str() {
                Some(choice) => json!({ "choice": choice }),
                None => json!({ "name": spec["alternative"] }),
            };
            history_edit(app, "historyEditCommit", commit, what);
            pump(app, what, |app| session(app).is_none() && !app.time_travel_ledger().has_pending_work());
        }
        "undo" | "redo" => history_lane(app, action, what),
        "reload" => reload(app, spec.as_str().expect("reload form"), what),
        other => panic!("{what}: unknown corpus step {other}"),
    }
}

/// 🔍️ Checks one step's expectation on the app.
fn check(app: &mut Puzzle2dApp, corpus: &Value, scenario: &Value, seed: usize, expect: &Value, what: &str) {
    let rows: Vec<HistoryEntry> = edits(app).split_off(seed);
    if let Some(count) = expect["rows"].as_u64() {
        assert_eq!(rows.len() as u64, count, "{what}: rows {:?}", rows.iter().map(|row| &row.op_lines).collect::<Vec<_>>());
    }
    let status = session(app).map(|status| serde_json::to_value(status).expect("status serializes"));
    if let Some(stage) = expect.get("stage") {
        assert_eq!(status.as_ref().map_or(&Value::Null, |status| &status["stage"]), stage, "{what}: stage of {status:?}");
    }
    for key in ["review", "blocking", "worst", "acceptedCount", "fault", "rerunnable"] {
        if let Some(wanted) = expect.get(key) {
            assert_eq!(status.as_ref().map(|status| &status[key]), Some(wanted), "{what}: {key} of {status:?}");
        }
    }
    for outcome in expect["outcomes"].as_array().into_iter().flatten() {
        let row = &rows[outcome["row"].as_u64().expect("outcome row") as usize];
        let mutation = serde_json::to_value(row.mutations.get(outcome["mutation"].as_u64().unwrap_or(0) as usize).expect("the row's mutation")).expect("mutation row serializes");
        assert_eq!(mutation["worst"], outcome["worst"], "{what}: worst of {mutation}");
        if let Some(introduced) = outcome.get("introduced") {
            assert_eq!(&mutation["introduced"], introduced, "{what}: whether the edit made the outcome new: {mutation}");
        }
        assert!(
            mutation["messages"].as_array().into_iter().flatten().any(|message| message["code"] == outcome["code"] && outcome.get("target").is_none_or(|target| message["target"] == *target)),
            "{what}: {} at {:?} in {mutation}",
            outcome["code"],
            outcome.get("target")
        );
    }
    if let Some(problem) = expect.get("nextProblem") {
        let next = app.time_travel_ledger().panel().and_then(|panel| panel.next_problem.clone());
        assert_eq!(next.as_deref(), Some(rows[problem["row"].as_u64().expect("problem row") as usize].mutations[0].mutation_id.as_str()), "{what}: the next problem");
    }
    if let Some(draft) = expect["draft"].as_object() {
        let value = Value::from(&app.time_travel_ledger().editor().expect("the draft editor").value);
        for (pointer, wanted) in draft {
            assert_eq!(value.pointer(pointer), Some(wanted), "{what}: the draft holds {wanted} at {pointer}: {value}");
        }
    }
    if let Some(name) = expect["head"].as_str() {
        check_head(app, corpus, scenario, name, what);
    }
    if let Some(name) = expect["preview"].as_str() {
        check_preview(app, corpus, scenario, name, what);
    }
    if let Some(labels) = expect["historyEdits"].as_array() {
        let wanted: Vec<(String, String)> = labels.iter().map(|label| (label["en"].as_str().expect("en").to_string(), label["de"].as_str().expect("de").to_string())).collect();
        assert_eq!(history_edit_labels(app), wanted, "{what}: the history-edit rows, newest first");
    }
    if let Some(name) = expect["alternative"].as_str() {
        let active = block_on(app.history_snapshot()).expect("history").active_alternative_id.unwrap_or_else(|| panic!("{what}: an alternative is active"));
        let body = history_body(app, semio_framework_ui_locale::Locale::En);
        let row = find_node(&body, &format!("framework.history.alternative.{active}")).unwrap_or_else(|| panic!("{what}: the active alternative's row: {body}"));
        assert!(row.to_string().contains(name) && row.to_string().contains("Current"), "{what}: {name} is the current alternative: {row}");
    }
    if let Some(words) = expect.get("words") {
        let key = format!("framework.history.entry.{}", rows[words["row"].as_u64().expect("words row") as usize].seq);
        for (locale, word) in [(semio_framework_ui_locale::Locale::En, &words["en"]), (semio_framework_ui_locale::Locale::De, &words["de"])] {
            let body = history_body(app, locale);
            let row = find_node(&body, &key).unwrap_or_else(|| panic!("{what}: the history body shows {key}: {body}"));
            assert!(row.to_string().contains(word.as_str().expect("word")), "{what} {locale:?}: {key} names its outcome in words: {row}");
        }
    }
}
//#endregion 🎬️Steps

//#region ⏪️Laws
/// ⚖️ LAW: every corpus scenario runs through the reserved verbs on the board app, and every step reaches its expectation:
/// - the session stage, review, blocking state, worst severity, accepted drafts, fault and rerun state;
/// - the per-mutation outcomes, including whether the edit introduced them;
/// - the Next problem target;
/// - the history-edit rows in English and German;
/// - the active alternative;
/// - the outcome named in words on its row;
/// - the named head, which equals a fresh fold of its edited log, and the head a review paints.
#[test]
fn every_corpus_scenario_reaches_its_session_outcomes_rows_and_heads() {
    let corpus = corpus();
    for scenario in corpus["scenarios"].as_array().expect("scenarios") {
        let id = scenario["id"].as_str().expect("scenario id");
        let mut app = seeded_app(board(&corpus, scenario));
        let seed = edits(&mut app).len();
        for (index, step) in scenario["steps"].as_array().expect("steps").iter().enumerate() {
            let what = format!("{id} step {index}");
            run(&mut app, seed, step, &what);
            if let Some(expect) = step.get("expect") {
                check(&mut app, &corpus, scenario, seed, expect, &what);
            }
        }
        close_app(&mut app);
    }
}

/// 🧱️ Op text of `count` one-unit drags of `targets`, one per line — a long downstream history in one ingest.
fn long_drags(targets: &[&str], count: usize) -> String {
    let drag = leaf(&json!({ "mutation": "dragSelection", "targets": targets, "dx": 1.0, "dy": 0.0 })).print_op();
    vec![drag; count].join("\n")
}

/// ⚖️ LAW (design §7, R13): an accepted draft replays its downstream over driver turns. Each UI frame the replay ships
/// carries the session in that frame: a replaying status with bounded progress (`done ≤ total`, `total > 0`), and the
/// completion with the ready review. The reviewed head moves the long downstream by the edited offset.
#[test]
fn replay_progress_rides_the_ui_frames_over_a_long_downstream() {
    let corpus = corpus();
    let mut app = seeded_app(&corpus["board"]);
    let seed = edits(&mut app).len();
    run(&mut app, seed, &json!({ "translate": { "select": ["left"], "dx": 1.0, "dy": 0.0, "step": 10.0 } }), "seed drag");
    for _ in 0..3 {
        block_on(app.ingest_operations_text(&long_drags(&["left"], 300))).expect("a long downstream edit");
    }
    run(&mut app, seed, &json!({ "begin": { "row": 0 } }), "begin");
    run(&mut app, seed, &json!({ "input": { "path": "/dx", "value": 30.0 } }), "input");
    run(&mut app, seed, &json!({ "acceptPending": null }), "accept");
    while app.take_typed_operation_ui_progress().is_some() {}
    let mut stages = Vec::new();
    for _ in 0..100_000 {
        block_on(app.advance_typed_operation_publication()).expect("driver turn");
        while let Some(progress) = app.take_typed_operation_ui_progress() {
            let Some(status) = progress.history_patch.and_then(|patch| patch.time_travel) else { continue };
            if status.stage == HistoryTimeTravelStage::Replaying {
                assert!(status.done.zip(status.total).is_some_and(|(done, total)| done <= total && total > 0), "progress is bounded: {status:?}");
            }
            stages.push(status.stage);
        }
        if session(&mut app).is_some_and(|status| status.stage == HistoryTimeTravelStage::Reviewing) && !app.time_travel_ledger().has_pending_work() {
            break;
        }
    }
    assert!(stages.contains(&HistoryTimeTravelStage::Replaying) && stages.last() == Some(&HistoryTimeTravelStage::Reviewing), "the replay ships its progress, then its completion: {stages:?}");
    let left = board_snapshot_nodes(&painted_fixture(&mut app)).iter().find(|node| node["id"] == "left").map(|node| node["x"].as_f64().expect("x")).expect("left is painted");
    assert_eq!(left, -200.0 + 30.0 + 900.0, "the reviewed head replays the 900 downstream drags over the edited one");
    run(&mut app, seed, &json!({ "exit": null }), "exit");
    close_app(&mut app);
}

/// 🎨️ The document the overview paints: the time-travel preview while a session is open.
fn painted_fixture(app: &mut Puzzle2dApp) -> Value {
    let body: Value = serde_json::from_str(&render_body(app, overview::BODY_KEY)).expect("board body");
    serde_json::from_str(body["board2d"]["snapshotJson"].as_str().expect("painted snapshot lane")).expect("painted snapshot parses")
}

/// 🔀️ Every event batch `from` published since the last relay, delivered into `to` through its probe `into` and ingested.
fn relay(from: &mut MemoryBackbone, into: &mut MemoryBackbone, to: &mut Puzzle2dApp) {
    for message in block_on(from.receive()).expect("replica outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        block_on(into.send(message)).expect("relay an event batch");
    }
    block_on(to.tick_backbone()).expect("ingest the relayed events");
}

/// ⚖️ LAW (design §16.6, G9): a remote history change whose replay exceeds one turn's budget
/// (`TIME_TRAVEL_REPLAY_OPERATIONS`) waits while the replica shows the history before it. Meanwhile:
/// - Its progress rides the history wire (`reprojection`, kind `remote`).
/// - Cancel replay pauses it, and driver turns leave it alone.
/// - Replay again resumes it, and the adoption equals the author's head.
#[test]
fn a_long_remote_history_change_replays_over_turns_pauses_and_resumes_on_the_board() {
    let corpus = corpus();
    let mut local = seeded_app(&corpus["board"]);
    let mut remote = app_with_registry();
    let (local_backbone, mut local_probe) = block_on(MemoryBackbone::pair("mem://puzzle2d-remote-replay-local", "mem://puzzle2d-remote-replay-local"));
    let (remote_backbone, mut remote_probe) = block_on(MemoryBackbone::pair("mem://puzzle2d-remote-replay-remote", "mem://puzzle2d-remote-replay-remote"));
    block_on(local.attach_backbone(store::Backbones::Memory(local_backbone))).expect("attach the local replica");
    block_on(remote.attach_backbone(store::Backbones::Memory(remote_backbone))).expect("attach the remote replica");
    let seed = edits(&mut local).len();
    run(&mut local, seed, &json!({ "translate": { "select": ["left"], "dx": 1.0, "dy": 0.0, "step": 10.0 } }), "seed drag");
    relay(&mut local_probe, &mut remote_probe, &mut remote);
    for _ in 0..3 {
        block_on(local.ingest_operations_text(&long_drags(&["mid"], 120))).expect("a long downstream edit");
        relay(&mut local_probe, &mut remote_probe, &mut remote);
    }
    assert_eq!(board_snapshot_nodes(&fixture_of(&remote)), board_snapshot_nodes(&fixture_of(&local)), "the remote replica holds the history");
    let before = fixture_of(&remote);
    for step in [json!({ "begin": { "row": 0 } }), json!({ "input": { "path": "/dx", "value": 30.0 } }), json!({ "accept": null }), json!({ "finalize": "overwrite" })] {
        run(&mut local, seed, &step, "the local history edit");
    }
    relay(&mut local_probe, &mut remote_probe, &mut remote);
    let waiting = block_on(remote.history_snapshot()).expect("history").reprojection.expect("the remote history change waits for its replay");
    assert!(waiting.total > 0 && waiting.done < waiting.total && !waiting.paused && waiting.kind == semio_framework::kernel::HistoryReprojectionKind::Remote && waiting.fault.is_none(), "{waiting:?}");
    assert_eq!(fixture_of(&remote), before, "the replica shows the history before the change");
    let meta = ActionMeta { view_state: Some(focused_view()), ..semio_framework_plugin::artifact_app_laws::meta("remote") };
    let cancelled = block_on(remote.handle_action("historyEditCancelReplay", Some(&semio_framework_value::DslValue::from(&json!({}))), &meta)).expect("cancel");
    assert!(cancelled.output.get("rejected").is_none(), "{:?}", cancelled.output);
    assert!(remote.time_travel_ledger().reprojection_paused(), "cancel pauses the remote replay");
    for _ in 0..4 {
        block_on(remote.advance_typed_operation_publication()).expect("driver turn");
    }
    assert_eq!(fixture_of(&remote), before, "paused, the change is not adopted");
    assert_eq!(block_on(remote.history_snapshot()).expect("history").reprojection.map(|replay| replay.paused), Some(true), "the wire says paused");
    let resumed = block_on(remote.handle_action("historyEditRerun", Some(&semio_framework_value::DslValue::from(&json!({}))), &meta)).expect("rerun");
    assert!(resumed.output.get("rejected").is_none(), "{:?}", resumed.output);
    pump(&mut remote, "the remote change is adopted", |app| block_on(app.history_snapshot()).expect("history").reprojection.is_none());
    assert_eq!(board_snapshot_nodes(&fixture_of(&remote)), board_snapshot_nodes(&fixture_of(&local)), "the adoption equals the author's head");
    block_on(local.detach_backbone()).expect("local releases its backbone");
    block_on(remote.detach_backbone()).expect("remote releases its backbone");
    close_app(&mut local);
    close_app(&mut remote);
}
/// 🧮️ The board's head and history as a history step may change them: the painted nodes, the active alternative and the
/// history rows.
fn board_trace(app: &mut Puzzle2dApp) -> (Vec<Value>, Option<String>, usize) {
    let patch = block_on(app.history_snapshot()).expect("history");
    (board_snapshot_nodes(&fixture_of(app)).to_vec(), patch.active_alternative_id, patch.upserts.len())
}

/// ⚖️ LAW (gap N17): switching to an alternative whose 360 drags are not applied is a local history step, which the
/// runtime replays over driver turns.
/// - The switch answers at once. The board, the active alternative and the history rows stay as they were.
/// - The wire carries `reprojection.kind = step`.
/// - Each turn replays at most `TIME_TRAVEL_REPLAY_OPERATIONS`.
/// - Meanwhile undo answers `history.replaying` and history editing is busy.
/// - The adoption shows the alternative's head and records the switch row.
/// - Cancel replay drops the step with zero trace.
#[test]
fn switching_to_a_long_alternative_replays_over_turns_and_cancel_leaves_zero_trace() {
    let corpus = corpus();
    for cancel in [true, false] {
        let mut app = seeded_app(&corpus["board"]);
        let seed = edits(&mut app).len();
        run(&mut app, seed, &json!({ "translate": { "select": ["left"], "dx": 1.0, "dy": 0.0, "step": 10.0 } }), "trunk drag");
        let trunk = block_on(app.history_snapshot()).expect("history").active_alternative_id.expect("the trunk line has an id");
        dispatch(&mut app, "createAlternative", Some(&json!({ "name": "Long" })), None).expect("a new alternative");
        let long = block_on(app.history_snapshot()).expect("history").active_alternative_id.expect("the new alternative is current");
        assert_ne!(long, trunk);
        for _ in 0..3 {
            block_on(app.ingest_operations_text(&long_drags(&["mid"], 120))).expect("a long edit on the alternative");
        }
        let long_nodes = board_snapshot_nodes(&fixture_of(&app)).to_vec();
        dispatch(&mut app, "switchAlternative", Some(&json!({ "alternativeId": trunk })), None).expect("back to the trunk");
        pump(&mut app, "the trunk is shown", |app| !app.time_travel_ledger().has_pending_work() && block_on(app.history_snapshot()).expect("history").reprojection.is_none());
        let before = board_trace(&mut app);
        assert_eq!(before.1.as_deref(), Some(trunk.as_str()));
        dispatch(&mut app, "switchAlternative", Some(&json!({ "alternativeId": long })), None).expect("the switch answers at once");
        let waiting = block_on(app.history_snapshot()).expect("history").reprojection.expect("the switch waits for its replay");
        assert!(waiting.kind == semio_framework::kernel::HistoryReprojectionKind::Step && waiting.total >= 360 && waiting.fault.is_none(), "{waiting:?}");
        assert_eq!(board_trace(&mut app), before, "nothing of the switch shows before its adoption");
        assert_eq!(dispatch(&mut app, "undo", None, None).err().map(|fault| fault.code.0), Some("history.replaying".to_string()), "a further history step waits");
        let first = edits(&mut app)[seed].mutations[0].mutation_id.clone();
        let meta = ActionMeta { view_state: Some(focused_view()), ..meta("local") };
        let busy = block_on(app.handle_action("historyEditBegin", Some(&semio_framework_value::DslValue::from(&json!({ "mutationId": first }))), &meta)).expect("begin answers");
        assert_eq!(busy.output.get("rejected").and_then(semio_framework_value::DslValue::as_str), Some("timeTravel.busy"), "history editing waits for the step");
        if cancel {
            history_edit(&mut app, "historyEditCancelReplay", json!({}), "cancel the switch");
            for _ in 0..4 {
                block_on(app.advance_typed_operation_publication()).expect("driver turn");
            }
            assert_eq!(board_trace(&mut app), before, "a cancelled switch leaves zero trace");
        } else {
            let mut turns = 0;
            while block_on(app.history_snapshot()).expect("history").reprojection.is_some_and(|replay| replay.kind == semio_framework::kernel::HistoryReprojectionKind::Step) {
                let done = block_on(app.history_snapshot()).expect("history").reprojection.map_or(0, |replay| replay.done);
                block_on(app.advance_typed_operation_publication()).expect("driver turn");
                while app.take_typed_operation_ui_progress().is_some() {}
                if let Some(after) = block_on(app.history_snapshot()).expect("history").reprojection.map(|replay| replay.done) {
                    assert!(after.saturating_sub(done) as usize <= semio_framework_plugin::app::time_travel::TIME_TRAVEL_REPLAY_OPERATIONS, "one turn replays at most one budget: {done} → {after}");
                }
                turns += 1;
                assert!(turns < 64, "the switch is adopted");
            }
            assert!(turns >= 2, "360 drags span several turns ({turns})");
            let (nodes, active, rows) = board_trace(&mut app);
            assert_eq!((nodes, active.as_deref(), rows), (long_nodes, Some(long.as_str()), before.2 + 1), "the adoption shows the alternative and records the switch row");
        }
        close_app(&mut app);
    }
}
//#endregion ⏪️Laws
