//! 🚗️ LAW: the WGPU driver editor owns React's seven-axis draft/save/delete lifecycle.

use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🚗️driver-editor/🔣️.json")).expect("driver fixture")
}

fn find_id<'a>(value: &'a Value, id: &str) -> Option<&'a Value> {
    if value.get("id").and_then(Value::as_str) == Some(id) {
        return Some(value);
    }
    match value {
        Value::Array(items) => items.iter().find_map(|item| find_id(item, id)),
        Value::Object(fields) => fields.values().find_map(|item| find_id(item, id)),
        _ => None,
    }
}

fn find_select<'a>(node: &'a UiNode, id: &str) -> Option<&'a UiSelectNode> {
    fn tree_item<'a>(item: &'a UiTreeItemNode, id: &str) -> Option<&'a UiSelectNode> {
        if let Some(UiControlNode::Select(select)) = item.control.as_ref() {
            if select.id == id {
                return Some(select);
            }
        }
        item.items.as_deref()?.iter().find_map(|nested| tree_item(nested, id))
    }
    match node {
        UiNode::Select(select) if select.id == id => Some(select),
        UiNode::Field(field) => find_select(&field.child, id),
        UiNode::Stack(stack) => stack.children.iter().find_map(|child| find_select(child, id)),
        UiNode::Section(section) => section.children.iter().find_map(|child| find_select(child, id)),
        UiNode::Tree(tree) => tree.sections.iter().flat_map(|section| &section.items).find_map(|item| tree_item(item, id)),
        _ => None,
    }
}

fn dispatch(shell: &mut ShellState, action: &str, args: Value) {
    semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor {
        controller_id: "framework".into(),
        action: action.into(),
        args: semio_framework::optional_json_to_dsl(Some(args)),
    }))
    .expect("driver editor action");
}

#[test]
fn settings_general_publishes_all_react_driver_controls_and_localized_dirty_state() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let fixture = fixture();
    let ui = shell.build_settings_general_ui();
    let value = serde_json::to_value(&ui).expect("settings ui");
    for id in fixture["controls"].as_array().unwrap().iter().filter_map(Value::as_str) {
        if id == "framework.settings.driver.delete" {
            assert!(find_id(&value, id).is_none(), "delete is custom-only");
        } else {
            assert!(find_id(&value, id).is_some(), "missing {id}");
        }
    }
    for axis in fixture["axes"].as_array().unwrap() {
        let key = axis["key"].as_str().unwrap();
        let select = find_select(&ui, &format!("framework.settings.driver.{key}")).expect("axis select");
        let items: Vec<&str> = select.items.iter().map(|item| item.value.as_str()).collect();
        assert_eq!(items, axis["options"].as_array().unwrap().iter().filter_map(Value::as_str).collect::<Vec<_>>());
    }
    assert_eq!(find_id(&value, "framework.settings.driver.editor").unwrap()["defaultOpen"], false);
    dispatch(&mut shell, "setDriverField", serde_json::json!({ "key": "labels", "value": "icons" }));
    shell.locale_id = "de".into();
    let dirty = serde_json::to_value(shell.build_settings_general_ui()).unwrap();
    let section = find_id(&dirty, "framework.settings.driver.editor").expect("driver editor section");
    assert!(section.to_string().contains("Geändert"));
}

#[test]
fn draft_is_live_selection_clears_it_and_save_delete_use_owned_preferences() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let fixture = fixture();
    dispatch(&mut shell, "setDriverField", fixture["transitions"]["edit"].clone());
    assert!(shell.driver_draft.is_some());
    assert_eq!(shell.driver_document().config["labels"], "icons");
    assert_eq!(shell.chrome_build.driver.labels, ui_wgpu::wgpu::UiDriverLabels::Icons, "draft changes live chrome");
    assert!(shell.chrome_build.preferences.custom_drivers.is_empty(), "draft never persists");

    dispatch(&mut shell, "setDriver", serde_json::json!({ "value": fixture["transitions"]["select"]["selected"] }));
    assert!(shell.driver_draft.is_none(), "selection clears draft");
    assert_eq!(shell.driver_id, "compact");
    assert_eq!(shell.chrome_build.driver.tooltips, ui_wgpu::wgpu::UiDriverTooltips::None);
    assert_eq!(shell.chrome_build.driver.drag, ui_wgpu::wgpu::chrome::UiDriverDrag::Surface);

    dispatch(&mut shell, "setDriverField", serde_json::json!({ "key": "labelTier", "value": "beginner" }));
    dispatch(&mut shell, "setDriverSaveLabel", serde_json::json!({ "value": fixture["transitions"]["save"]["label"] }));
    dispatch(&mut shell, "saveDriver", serde_json::json!({}));
    let saved_id = fixture["transitions"]["save"]["id"].as_str().unwrap();
    let saved = shell.chrome_build.preferences.custom_drivers.get(saved_id).expect("saved driver");
    assert_eq!(saved.driver_id, saved_id);
    assert_eq!(saved.label, "Focus Flow");
    assert_eq!(saved.config["labelTier"], "beginner");
    assert_eq!(shell.driver_id, saved_id);
    assert!(shell.driver_draft.is_none());
    assert!(shell.driver_save_label.is_empty());
    let custom_ui = serde_json::to_value(shell.build_settings_general_ui()).unwrap();
    assert!(find_id(&custom_ui, "framework.settings.driver.delete").is_some());

    dispatch(&mut shell, "deleteDriver", serde_json::json!({ "value": saved_id }));
    assert!(!shell.chrome_build.preferences.custom_drivers.contains_key(saved_id));
    assert_eq!(shell.driver_id, "default");
    assert!(shell.driver_draft.is_none());
    assert_eq!(shell.chrome_build.driver, ui_wgpu::wgpu::UiDriverChrome::DEFAULT);
}
