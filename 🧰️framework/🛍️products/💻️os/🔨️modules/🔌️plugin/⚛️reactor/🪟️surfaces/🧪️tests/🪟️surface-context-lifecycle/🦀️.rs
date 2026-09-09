//! 🧪️ Surface context survives rerender and stays isolated from sibling surfaces and app instances.

use super::*;

#[semio_framework_async_macros::async_test]
async fn surface_context_retains_host_preferences_and_window_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
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

/// 🧩️ A reserved section surface keeps the FULL host view it was mounted with — no window/panel
/// narrowing, and no dependence on which sibling surface mounted last — and no window closing ever
/// prunes it, because a section is keyed by no window at all.
#[test]
fn reserved_section_surfaces_keep_the_unnarrowed_view_and_outlive_their_windows() {
    let mut contexts = SurfaceContexts::default();
    let view = ViewModel { window_instances: vec![semio_framework::ViewWindowInstance { id: "left".into(), window_kind_id: "graph".into() }], window_id: Some("left".into()), active_tool_id: Some("fill".into()), ..Default::default() };
    contexts.insert("7:left".into(), "graph".into(), view.for_window_instance("left").unwrap()).unwrap();
    for section in UiRefreshSection::ALL {
        contexts.insert(format!("7:{}", section.body_key()), section.body_key().into(), view.clone()).unwrap();
    }
    contexts.insert("7:panel".into(), "properties".into(), view.for_panel()).unwrap();
    for section in UiRefreshSection::ALL {
        let context = contexts.get(&format!("7:{}", section.body_key())).unwrap();
        assert_eq!(context.body_key, section.body_key());
        assert_eq!(context.view_state.window_id.as_deref(), Some("left"));
        assert_eq!(context.view_state.active_tool_id.as_deref(), Some("fill"));
        assert_eq!(context.view_state.window_instances.len(), 1);
    }
    let mut closed = view.clone();
    closed.window_instances.clear();
    contexts.update_view(&closed);
    assert!(contexts.get("7:left").is_none());
    for section in UiRefreshSection::ALL {
        assert!(contexts.get(&format!("7:{}", section.body_key())).is_some());
        contexts.remove(&format!("7:{}", section.body_key()));
    }
    assert!(contexts.section_view.is_none());
    contexts.remove("7:panel");
    assert_eq!(contexts.len(), 0);
    eprintln!("[DEBUG] reserved section surfaces retained the unnarrowed host view across sibling mounts and window closure");
}
