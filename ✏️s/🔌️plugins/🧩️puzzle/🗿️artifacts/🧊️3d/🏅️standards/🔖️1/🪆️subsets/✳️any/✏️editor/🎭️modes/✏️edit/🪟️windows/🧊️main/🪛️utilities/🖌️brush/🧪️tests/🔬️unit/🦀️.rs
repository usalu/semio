use super::*;
use semio_framework_tool_run::{ToolRunId, ToolRunIdentity};

fn run_view(tool_id: &str, run: u64, generation: u32, state: ToolRunState) -> ToolRunView {
    ToolRunView::new(tool_id, ToolRunIdentity { generation, ..ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run }, [0; 32]) }, state)
}

fn actions(effects: &[Effect]) -> Vec<(String, serde_json::Value)> {
    effects
        .iter()
        .map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } => (action.clone(), serde_json::from_str(&dsl::json::to_json_string(&dsl::json::from_dsl_value(args.as_ref().expect("a tool run action carries its args")))).expect("args json")),
            other => panic!("the reconciler only dispatches tool run actions: {other:?}"),
        })
        .collect()
}

/// 🚦️ LAW: the reconciler starts a brush run for a target, never repeats an unanswered request, wakes a live run,
/// aborts a live run once the target is gone, waits out the abort, restarts a faulted run only on a gesture, and
/// leaves another tool's run to the framework's own busy refusal.
#[test]
fn the_brush_suggestions_reconciler_starts_wakes_and_aborts_exactly_once_per_answer() {
    let mut link = BrushSuggestionsLink::default();
    assert!(run_effects(&mut link, None).is_empty(), "no target, no run, nothing to ask");
    link.hover(Some("host:v0".into()));
    assert_eq!(actions(&run_effects(&mut link, None)), vec![(TOOL_RUN_START_ACTION_ID.to_string(), serde_json::json!({ "toolId": UTILITY_ID }))]);
    assert!(run_effects(&mut link, None).is_empty(), "an unanswered start is never repeated by a refresh");
    let starting = run_view(UTILITY_ID, 1, 0, ToolRunState::Starting);
    assert!(run_effects(&mut link, Some(&starting)).is_empty(), "a live run for the target is woken, not restarted");
    link.hover(Some("host:v1".into()));
    assert!(run_effects(&mut link, Some(&run_view(UTILITY_ID, 1, 2, ToolRunState::Running))).is_empty(), "retargeting a live run is a wake");
    link.hover(None);
    let running = run_view(UTILITY_ID, 1, 2, ToolRunState::Running);
    assert_eq!(actions(&run_effects(&mut link, Some(&running))), vec![(TOOL_RUN_ABORT_ACTION_ID.to_string(), serde_json::json!({ "runId": "1", "generation": 2 }))]);
    assert!(run_effects(&mut link, Some(&run_view(UTILITY_ID, 1, 2, ToolRunState::Aborting))).is_empty(), "the abort is outstanding while the run tears down");
    assert!(run_effects(&mut link, Some(&run_view(UTILITY_ID, 1, 2, ToolRunState::Aborted))).is_empty(), "an aborted run with no target owes nothing");
    link.open_menu("host:v0");
    let aborted = run_view(UTILITY_ID, 1, 2, ToolRunState::Aborted);
    assert_eq!(actions(&run_effects(&mut link, Some(&aborted))).len(), 1, "a popup on a terminal run starts a fresh one");
    let faulted = run_view(UTILITY_ID, 2, 0, ToolRunState::Faulted);
    assert!(run_effects(&mut link, Some(&faulted)).is_empty(), "the start was answered by the faulted run, which a refresh never restarts");
    assert!(run_effects(&mut link, Some(&faulted)).is_empty());
    link.close_menu();
    link.open_menu("host:v0");
    assert_eq!(actions(&run_effects(&mut link, Some(&faulted))).len(), 1, "a gesture restarts a faulted run");
    let fill = run_view("fill", 3, 0, ToolRunState::Running);
    let mut busy = BrushSuggestionsLink::default();
    busy.hover(Some("host:v0".into()));
    assert_eq!(actions(&run_effects(&mut busy, Some(&fill))).len(), 1, "another tool's run is the framework's busy refusal to give");
    assert!(run_effects(&mut busy, Some(&fill)).is_empty(), "and the refused start is not repeated until a gesture");
}
