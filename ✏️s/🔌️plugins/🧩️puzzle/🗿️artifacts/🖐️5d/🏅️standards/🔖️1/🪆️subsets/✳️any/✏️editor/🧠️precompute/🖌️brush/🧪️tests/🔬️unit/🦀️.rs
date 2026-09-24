use crate::editor::puzzle5d::modes::edit::windows::board2d::utilities::brush::UTILITY_ID;
use crate::editor::puzzle5d::modes::edit::windows::world3d;
use crate::editor::puzzle5d::precompute::fill::tests::{committed, host_turn, tool_run_action, FILL_RUN_TURNS};
use crate::editor::puzzle5d::precompute::PUZZLE5D_PLANNER_TRACE_TWIN_BIT;
use crate::editor::puzzle5d::puzzle5d_grip_full_id;
use crate::editor::puzzle5d::unit_tests::context::{app_with_registry, close_app, dispatch, dispatch_armed, projection_of, window_view, Puzzle5dApp};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactApp, PluginApp};
use semio_framework_tool_run::{ToolRunTraceDelta, ToolRunTraceStore, ToolRunTraceSubject, ToolRunVerdict};
use std::collections::HashMap;

fn armed_view() -> semio_framework_plugin::ViewModel {
    let mut view = window_view(world3d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID);
    view.active_utility_by_window_id.insert(world3d::WINDOW_KIND_ID.to_string(), UTILITY_ID.to_string());
    view
}

/// 🎣️ Points the armed brush at `grip` (or at nothing) and pumps the refresh effects and host turns until `until`.
fn target(app: &mut Puzzle5dApp, grip: Option<&str>, what: &str, until: impl Fn(Option<&protocol::PresenceToolRun>) -> bool) {
    let args = grip.map_or_else(|| dsl::json!({}), |grip| dsl::json!({ "fullId": grip }));
    dispatch_armed(app, "targetBrushSuggestions", Some(&args), world3d::WINDOW_KIND_ID, UTILITY_ID).expect("targetBrushSuggestions");
    for _ in 0..FILL_RUN_TURNS {
        for effect in semio_framework::io::resolve_ready(app.pending_effects(Some(&armed_view()))) {
            if let Effect::DispatchAction { action, args, .. } = effect {
                tool_run_action(app, &action, args.as_ref().map_or(serde_json::Value::Null, serde_json::Value::from));
            }
        }
        if until(app.tool_run_presence().as_ref()) {
            return;
        }
        host_turn(app);
    }
    panic!("{what} never happened; presence {:?}", app.tool_run_presence());
}

fn settled(presence: Option<&protocol::PresenceToolRun>) -> bool {
    presence.is_some_and(|presence| presence.state == protocol::PresenceToolRunState::Running && presence.total.is_some_and(|total| total > 0 && presence.completed == total))
}

fn trace(app: &Puzzle5dApp) -> (u64, HashMap<u64, (ToolRunVerdict, ToolRunTraceSubject)>) {
    let encoded = app.tool_run_trace_delta(None).expect("a live trace");
    let delta = ToolRunTraceDelta::decode(&semio_framework_io_base64::base64_url_decode(encoded).expect("base64url")).expect("trace delta");
    let mut store = ToolRunTraceStore::new(delta.identity);
    for page in &delta.pages {
        store.apply_page(page).expect("trace page");
    }
    (delta.identity.id.run, store.records().map(|(key, record)| (key, (record.verdict, record.subject))).collect())
}

/// ⏯️ LAW (lane W2-B): the armed 5d brush pointing at a grip runs the puzzle 3d brush suggestions search as a
/// READ-ONLY tool run. Every candidate it decides is a world trace record with a board twin carrying the same
/// verdict, the committed document never moves, pointing at another grip retargets the same run, and leaving every
/// grip aborts it without an undo entry.
#[test]
fn the_armed_brush_traces_every_candidate_in_both_windows_and_leaving_aborts_it() {
    let mut app = app_with_registry();
    let before = committed(&app);
    let projection = projection_of(&app);
    let grips: Vec<String> = projection["parts"].as_array().expect("parts").iter().flat_map(|part| part["grips"].as_array().into_iter().flatten().map(move |grip| puzzle5d_grip_full_id(part["id"].as_str().expect("part id"), grip["id"].as_str().expect("grip id")))).collect();
    let (first, last) = (grips.first().cloned().expect("a grip"), grips.last().cloned().expect("a grip"));
    assert!(app.tool_run_presence().is_none(), "arming the brush alone starts nothing");
    target(&mut app, Some(&first), "the search settles", settled);
    let presence = app.tool_run_presence().expect("a brush run");
    assert_eq!(presence.tool_id, UTILITY_ID);
    let (run, records) = trace(&app);
    let world: Vec<(&u64, &(ToolRunVerdict, ToolRunTraceSubject))> = records.iter().filter(|(_, (_, subject))| matches!(subject, ToolRunTraceSubject::Instance3d { .. })).collect();
    assert_eq!(world.len() as u64, presence.total.expect("a counted search"), "every candidate of the target is a world record");
    for (key, (verdict, _)) in &world {
        let twin = records.get(&(**key | PUZZLE5D_PLANNER_TRACE_TWIN_BIT)).unwrap_or_else(|| panic!("candidate {key} has no board twin"));
        assert!(matches!(twin, (twin_verdict, ToolRunTraceSubject::Placement2d { .. }) if twin_verdict == verdict), "the board twin of {key} carries its verdict");
        assert!(matches!(verdict, ToolRunVerdict::Success | ToolRunVerdict::Warning | ToolRunVerdict::Danger), "a settled search leaves no candidate under test");
    }
    assert_eq!(committed(&app), before, "a read-only run never touches the document");
    target(&mut app, Some(&last), "the retargeted search settles", settled);
    assert_eq!(trace(&app).0, run, "a new target retargets the same run");
    target(&mut app, None, "leaving aborts the run", |presence| presence.is_some_and(|presence| presence.state == protocol::PresenceToolRunState::Aborted));
    assert_eq!(committed(&app), before, "the aborted search left the document byte-identical");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(committed(&app), before, "and left no undo entry behind");
    close_app(&mut app);
}

fn select_view() -> semio_framework_plugin::ViewModel {
    window_view(world3d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID)
}

/// 🎣️ The world pane's published `suggestionMenu` record, read off a real render of the world body.
fn suggestion_menu(app: &mut Puzzle5dApp) -> serde_json::Value {
    let rendered = crate::editor::puzzle5d::unit_tests::context::render_body_with_view(app, world3d::BODY_KEY, &select_view());
    let scene: serde_json::Value = serde_json::from_str(&rendered).expect("world scene json");
    let interaction: serde_json::Value = serde_json::from_str(scene["scene"]["interactionJson"].as_str().expect("the world scene carries interactionJson")).expect("interactionJson parses");
    interaction["suggestionMenu"].clone()
}

/// 🎣️ LAW (grip context menu): opening the suggestion SUBMENU on a grip — no brush armed — starts the read-only
/// search right away, the menu lists the free candidates it found (each with its trace key), accepting one places
/// exactly one part fastened to that grip, and closing the menu aborts the search.
#[test]
fn the_grip_suggestion_submenu_searches_without_the_brush_and_accepts_a_candidate() {
    let mut app = app_with_registry();
    let projection = projection_of(&app);
    let parts = projection["parts"].as_array().map_or(0, Vec::len);
    let grip = projection["parts"].as_array().and_then(|parts| parts.first()).and_then(|part| Some(puzzle5d_grip_full_id(part["id"].as_str()?, part["grips"].as_array()?.first()?["id"].as_str()?))).expect("a grip");
    let opened = dispatch_armed(&mut app, "openVortexSuggestions", Some(&dsl::json!({ "fullId": grip.as_str(), "x": 10.0, "y": 20.0, "submenu": true })), world3d::WINDOW_KIND_ID, "select").expect("openVortexSuggestions");
    let starts: Vec<(String, serde_json::Value)> = opened
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } => Some((action.clone(), args.as_ref().map_or(serde_json::Value::Null, serde_json::Value::from))),
            _ => None,
        })
        .collect();
    assert!(
        starts.iter().any(|(action, args)| action == semio_framework_tool_run::TOOL_RUN_START_ACTION_ID && args[semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID] == serde_json::json!(UTILITY_ID)),
        "opening the menu starts the search itself, without waiting for a refresh poll: {starts:?}"
    );
    for (action, args) in starts {
        tool_run_action(&mut app, &action, args);
    }
    let mut listed = serde_json::Value::Null;
    for _ in 0..FILL_RUN_TURNS {
        for effect in semio_framework::io::resolve_ready(app.pending_effects(Some(&select_view()))) {
            if let Effect::DispatchAction { action, args, .. } = effect {
                tool_run_action(&mut app, &action, args.as_ref().map_or(serde_json::Value::Null, serde_json::Value::from));
            }
        }
        listed = suggestion_menu(&mut app);
        if listed["candidates"].as_array().is_some_and(|rows| !rows.is_empty()) || settled(app.tool_run_presence().as_ref()) {
            break;
        }
        host_turn(&mut app);
    }
    assert_eq!(listed["submenu"], serde_json::json!(true), "the menu is presented as the context menu's submenu: {listed}");
    assert_eq!(listed["vortexFullId"], serde_json::json!(grip), "the menu lists candidates for its own grip: {listed}");
    let rows = listed["candidates"].as_array().cloned().unwrap_or_default();
    assert!(!rows.is_empty(), "the search found free candidates without the brush armed: {listed}; presence {:?}", app.tool_run_presence());
    assert!(rows.iter().all(|row| row["key"].is_u64()), "every row names its trace record: {listed}");
    dispatch_armed(&mut app, "hoverSuggestion", Some(&dsl::json!({ "index": 0 })), world3d::WINDOW_KIND_ID, "select").expect("hoverSuggestion");
    dispatch_armed(&mut app, "acceptSuggestion", Some(&dsl::json!({ "index": 0, "fullId": grip.as_str() })), world3d::WINDOW_KIND_ID, "select").expect("acceptSuggestion");
    let placed = projection_of(&app);
    assert_eq!(placed["parts"].as_array().map_or(0, Vec::len), parts + 1, "accepting places exactly one part");
    assert!(placed["fasteners"].as_array().is_some_and(|rows| rows.iter().any(|row| row["source"].as_str() == Some(grip.as_str()) || row["target"].as_str() == Some(grip.as_str()))), "the placed part is fastened to the menu's grip");
    assert_eq!(suggestion_menu(&mut app), serde_json::Value::Null, "accepting closes the menu");
    close_app(&mut app);
}
