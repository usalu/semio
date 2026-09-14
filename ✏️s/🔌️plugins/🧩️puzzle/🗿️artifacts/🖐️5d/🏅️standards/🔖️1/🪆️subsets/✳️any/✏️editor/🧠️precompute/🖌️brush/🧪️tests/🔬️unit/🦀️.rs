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
