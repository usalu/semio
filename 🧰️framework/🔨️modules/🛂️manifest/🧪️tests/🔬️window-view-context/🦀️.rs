//! 🪟️ Host view projections isolate concrete windows, including windows of the same kind.

use super::ViewModel;

#[semio_framework_async_macros::async_test]
async fn window_view_context_uses_the_addressed_instance() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-view-context/🔣️.json")).unwrap();
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
    let window_only = view.for_window_instance("left").unwrap();
    assert!(window_only.active_tool_id.is_none(), "fails-before: windowed dispatch without host overlay drops the armed tool");
    for case in fixture["hostArmed"].as_array().unwrap() {
        let host_tool = case["hostActiveToolId"].as_str();
        let window_id = case["windowId"].as_str();
        let projected = view.for_host_armed_action(host_tool, window_id).unwrap();
        assert_eq!(projected.active_tool_id.as_deref(), case["activeToolId"].as_str());
        assert_eq!(projected.active_utility_id.as_deref(), case["activeUtilityId"].as_str());
    }
}
