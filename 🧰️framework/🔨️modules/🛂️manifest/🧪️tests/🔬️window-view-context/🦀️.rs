//! 🪟️ Host view projections isolate concrete windows, including windows of the same kind.

use crate::ViewModel;

#[semio_framework_async_macros::async_test]
async fn window_view_context_uses_the_addressed_instance() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔬️window-view-context/🔣️.json")).unwrap();
    let view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
    for key in ["locale", "terminology"] {
        let mut missing = fixture["view"].clone();
        missing.as_object_mut().unwrap().remove(key);
        assert!(serde_json::from_value::<ViewModel>(missing).is_err());
    }
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["windowId"].as_str().unwrap();
        let projected = view.for_window_instance(id);
        if case["absent"] == true {
            assert!(projected.is_none());
            continue;
        }
        let projected = projected.unwrap();
        assert_eq!(projected.window_id.as_deref(), Some(id));
        assert_eq!(projected.active_window_kind_id.as_deref(), Some("graph"));
        assert_eq!(projected.active_utility_id.as_deref(), case["activeUtilityId"].as_str());
        let oracle = &fixture["view"]["activeUtilityByWindowId"][id];
        assert_eq!(serde_json::to_value(&projected.active_utility_id).unwrap(), *oracle);
        assert_eq!(projected.locale, view.locale);
        assert_eq!(projected.terminology, view.terminology);
        assert_eq!(projected.active_mode_id, view.active_mode_id);
    }
    assert_eq!(view.window_id.as_deref(), Some("left"));
    assert_eq!(view.active_utility_id.as_deref(), Some("pan"));
    let panel = view.for_panel();
    assert!(panel.window_id.is_none());
    assert!(panel.active_window_kind_id.is_none());
    assert!(panel.active_utility_id.is_none());
    assert_eq!(panel.locale, view.locale);
    assert_eq!(panel.terminology, view.terminology);
    assert_eq!(panel.active_mode_id, view.active_mode_id);
    assert_eq!(panel.active_utility_by_window_id, view.active_utility_by_window_id);
    assert_eq!(panel.focused_window_id, view.focused_window_id, "fails-before: an app-level panel that authors per-window settings loses the pane the user is looking at and retunes the roster's first entry");
    assert_eq!(view.for_window_instance("left").unwrap().focused_window_id, view.focused_window_id, "a windowed projection carries the focused pane alongside its own render target");
    let window_only = view.for_window_instance("left").unwrap();
    assert!(window_only.active_tool_id.is_none(), "fails-before: windowed dispatch without host overlay drops the armed tool");
    for case in fixture["hostArmed"].as_array().unwrap() {
        let host_tool = case["hostActiveToolId"].as_str();
        let window_id = case["windowId"].as_str();
        let projected = view.for_host_armed_action(host_tool, window_id).unwrap();
        assert_eq!(projected.active_tool_id.as_deref(), case["activeToolId"].as_str());
        assert_eq!(projected.active_utility_id.as_deref(), case["activeUtilityId"].as_str());
    }
    for row in fixture["panelFocus"].as_array().unwrap() {
        let focused = row["focusedWindowId"].as_str().unwrap();
        let raised = ViewModel { focused_window_id: Some(focused.to_string()), ..view.clone() };
        let panel = raised.for_panel();
        assert_eq!(panel.focused_window_id.as_deref(), row["keptFocusedWindowId"].as_str(), "panel focus row {}", row["id"]);
        assert_eq!(panel.window_instances, view.window_instances, "panel focus row {} must not touch the roster", row["id"]);
    }
}

/// 🧰️ Window utility lookup resolves roster identities and never borrows a sibling's overlay.
#[test]
fn window_utility_uses_exact_instance_ownership() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔬️window-view-context/🔣️.json")).unwrap();
    for case in fixture["utilityCases"].as_array().unwrap() {
        let mut input = fixture["view"].clone();
        for (source, target) in [("currentWindowId", "windowId"), ("focusedWindowId", "focusedWindowId"), ("utilities", "activeUtilityByWindowId"), ("windows", "windowInstances")] {
            if let Some(value) = case.get(source) { input[target] = value.clone(); }
        }
        let view: ViewModel = serde_json::from_value(input.clone()).unwrap();
        let owned: ViewModel = semio_framework_pack_json::from_json_str(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let actual = owned.window_utility(case["windowId"].as_str());
        assert_eq!(actual, case["activeUtilityId"].as_str(), "{case}");
        assert_eq!(actual, view.window_utility(case["windowId"].as_str()));
    }
    println!("[DEBUG] shared window utility routing preserves exact instance ownership");
}
