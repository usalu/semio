//! 🧪️ Surface context survives rerender and stays isolated from sibling surfaces and app instances.

use super::*;

#[semio_framework_async_macros::async_test]
async fn surface_context_retains_host_preferences_and_window_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let view: semio_framework::ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
    let mut contexts = SurfaceContexts::default();
    let mut oracle = serde_json::Map::new();
    for surface in fixture["surfaces"].as_array().unwrap() {
        let id = surface["id"].as_str().unwrap();
        let projected = surface["windowId"].as_str().map(|window| view.for_window_instance(window).unwrap()).unwrap_or_else(|| view.clone());
        contexts.insert(id.into(), surface["bodyKey"].as_str().unwrap().into(), projected).unwrap();
        oracle.insert(id.into(), surface.clone());
    }
    for (id, expected) in &oracle {
        let actual = contexts.get(id).unwrap();
        assert_eq!(actual.body_key, expected["bodyKey"].as_str().unwrap());
        assert_eq!(actual.view_state.window_id.as_deref(), expected["windowId"].as_str());
        assert_eq!(actual.view_state.active_utility_id.as_deref(), expected["activeUtilityId"].as_str());
        assert_eq!(actual.view_state.locale, view.locale);
        assert_eq!(actual.view_state.terminology, view.terminology);
    }
    let mut updated = fixture["view"].clone();
    for (key, value) in fixture["update"].as_object().unwrap() {
        updated[key] = value.clone();
    }
    let updated: semio_framework::ViewModel = serde_json::from_value(updated).unwrap();
    contexts.update_view(&updated);
    for (id, expected) in &oracle {
        let actual = contexts.get(id).unwrap();
        assert_eq!(actual.view_state.locale, updated.locale);
        assert_eq!(actual.view_state.terminology, updated.terminology);
        assert_eq!(actual.view_state.window_id.as_deref(), expected["windowId"].as_str());
        let utility = expected["windowId"].as_str().and_then(|window| fixture["update"]["activeUtilityByWindowId"][window].as_str());
        assert_eq!(actual.view_state.active_utility_id.as_deref(), utility);
    }
    let hidden = fixture["hidden"].as_str().unwrap();
    contexts.remove(hidden);
    oracle.remove(hidden);
    assert!(contexts.get(hidden).is_none());
    assert!(contexts.get(fixture["survivor"].as_str().unwrap()).is_some());
    assert!(SurfaceContexts::default().get(fixture["survivor"].as_str().unwrap()).is_none());
    assert_eq!(contexts.len(), oracle.len());
    let mut closed = updated.clone();
    closed.window_instances.retain(|window| !fixture["closedWindowIds"].as_array().unwrap().iter().any(|id| id.as_str() == Some(window.id.as_str())));
    contexts.update_view(&closed);
    assert_eq!(contexts.len(), 1);
    assert!(contexts.get(fixture["panelSurvivor"].as_str().unwrap()).is_some());
    contexts.remove(fixture["panelSurvivor"].as_str().unwrap());
    assert_eq!(contexts.len(), 0);
    assert!(contexts.view_state.is_none());
    eprintln!("[DEBUG] surface context retains host locale, independent window utilities and panel identity; hidden/closed surfaces release bindings");
}

#[test]
fn surface_context_capacity_reuses_closed_windows_without_partial_updates() {
    let mut contexts = SurfaceContexts::default();
    let mut view = ViewModel { window_instances: vec![semio_framework::ViewWindowInstance { id: "closed".into(), window_kind_id: "graph".into() }], window_id: Some("closed".into()), ..Default::default() };
    for index in 0..semio_framework_ui_contract::UI_RESIDENT_SLOTS {
        contexts.insert(index.to_string(), "graph".into(), view.clone()).unwrap();
    }
    assert!(contexts.insert("overflow".into(), "graph".into(), view.clone()).is_err());
    assert_eq!(contexts.len(), semio_framework_ui_contract::UI_RESIDENT_SLOTS);
    view.window_instances.clear();
    view.window_id = None;
    contexts.insert("panel".into(), "properties".into(), view).unwrap();
    assert_eq!(contexts.len(), 1);
    assert!(contexts.get("panel").unwrap().view_state.window_id.is_none());
}
