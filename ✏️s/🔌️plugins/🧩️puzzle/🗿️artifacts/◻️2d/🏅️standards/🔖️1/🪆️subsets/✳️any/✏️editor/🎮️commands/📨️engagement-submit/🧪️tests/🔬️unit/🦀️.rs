use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::unit_tests::context::*;
use crate::editor::puzzle2d::{fixture_nodes, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{PluginApp, ViewModel, WindowMeasure};
use serde_json::{json, Value};

fn node_of(app: &Puzzle2dApp, id: &str) -> Value {
    fixture_nodes(&fixture_of(app)).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id)).cloned().expect("node")
}

fn submit(app: &mut Puzzle2dApp, line: &str) -> semio_framework_plugin::InvocationResult {
    dispatch(app, "engagementSubmit", Some(&json!({ "pane": overview::WINDOW_KIND_ID, "value": line })), Some(overview::WINDOW_KIND_ID)).unwrap_or_else(|fault| panic!("engagementSubmit {line:?}: {fault:?}"))
}

fn rendered_fill_count(app: &mut Puzzle2dApp) -> f64 {
    let measures = semio_framework::io::resolve_ready(app.tool_measures(&ViewModel::default()));
    let Some([WindowMeasure::Group { children, .. }]) = measures.get(fill::TOOL_ID).map(Vec::as_slice) else { panic!("fill tool measure group") };
    let Some(WindowMeasure::Number { value, .. }) = children.first() else { panic!("fill count number") };
    *value
}

/// ⌨️ The typed line reaches the guest VERBATIM, spaces and arguments included: the React action line
/// used to PascalCase and squash it (`move 50 25` → `Move5025`), which collapsed every argument into
/// the verb and made the whole `verb <args>` grammar a silent no-op. Every advertised argument verb of
/// the placeholder (`move <dx> <dy>`, `rotate <deg>`, `scale <f>`, `fill <n>`) is exercised here with
/// the exact text a user types.
#[semio_framework_async_macros::async_test]
async fn engagement_line_carries_its_arguments_verbatim() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &id).expect("select node");
    let before = node_of(&app, &id);
    let (x, y) = (before.get("x").and_then(Value::as_f64).expect("x"), before.get("y").and_then(Value::as_f64).expect("y"));

    submit(&mut app, "move 50 25");
    let moved = node_of(&app, &id);
    assert_eq!(moved.get("x").and_then(Value::as_f64), Some(x + 50.0), "`move 50 25` must translate by dx");
    assert_eq!(moved.get("y").and_then(Value::as_f64), Some(y + 25.0), "`move 50 25` must translate by dy");

    // 🔄️ A single-node selection rotates about its own centroid, so the pose is unchanged — the proof
    // that the verb parsed is the radius-preserving handle turn, asserted through a two-node selection.
    let other = fixture_nodes(&fixture_of(&app)).iter().map(|node| node.get("id").and_then(Value::as_str).unwrap_or_default().to_string()).find(|other| other != &id).expect("a second node");
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &other).expect("select the second node");
    let pair_before = node_of(&app, &other);
    submit(&mut app, "rotate 45");
    assert_ne!(node_of(&app, &other).get("x").and_then(Value::as_f64), pair_before.get("x").and_then(Value::as_f64), "`rotate 45` must turn the selection");

    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &id).expect("reselect the first node");
    let scaled_before = node_of(&app, &id);
    let radius = scaled_before.get("radius").and_then(Value::as_f64).expect("radius");
    submit(&mut app, "scale 1.5");
    assert_eq!(node_of(&app, &id).get("radius").and_then(Value::as_f64), Some(radius * 1.5), "`scale 1.5` must carry its decimal factor");

    assert_eq!(rendered_fill_count(&mut app), 100.0);
    let filled = submit(&mut app, "fill 12");
    assert_eq!(rendered_fill_count(&mut app), 12.0, "`fill 12` must retarget the shared count");
    assert!(filled.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id == fill::TOOL_ID)), "`fill <n>` must arm the fill tool");
    close_app(&mut app);
}

/// 🔤️ The verb token is matched case-insensitively, so a shell that publishes the line in another
/// casing still parses; a line whose verb is unknown stays a no-op and never faults.
#[semio_framework_async_macros::async_test]
async fn engagement_verbs_are_case_insensitive_and_unknown_lines_are_no_ops() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &id).expect("select node");
    let x = node_of(&app, &id).get("x").and_then(Value::as_f64).expect("x");
    submit(&mut app, "MOVE 7 0");
    assert_eq!(node_of(&app, &id).get("x").and_then(Value::as_f64), Some(x + 7.0), "`MOVE 7 0` must parse like `move 7 0`");
    let before = fixture_of(&app);
    for line in ["", "move", "wiggle 3", "scale 0"] {
        submit(&mut app, line);
        assert_eq!(fixture_of(&app), before, "{line:?} must leave the document untouched");
    }
    close_app(&mut app);
}
