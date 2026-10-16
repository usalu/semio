use super::*;
fn ui_text(value:&str)->UiText{UiText::try_from_str(value).expect("bounded fixture text")}
#[test]
fn action_id_displays_scope_dot_name_at_version() {
    let id = ActionId::try_v1("cad-play", "objectMove").expect("bounded action id");
    assert_eq!(id.to_string(), "cad-play.objectMove@1");
    assert_eq!(ActionId::new(ui_text("app"), ui_text("submit"), 3).to_string(), "app.submit@3");
}

#[test]
fn action_binding_round_trips_with_and_without_args() {
    let full = ActionBinding { trigger: Trigger::Change, action: ActionId::try_v1("app", "setValue").expect("bounded action id"), args: Some(UiValue::Text(ui_text("scope"))), capability: Some(ui_text("edit")) };
    let first = serde_json::to_string(&full).expect("serialize");
    let back: ActionBinding = serde_json::from_str(&first).expect("deserialize");
    assert_eq!(full, back);

    let minimal = ActionBinding::default();
    let json = serde_json::to_value(&minimal).expect("serialize");
    assert!(json.get("args").is_none());
    assert!(json.get("capability").is_none());
}

#[test]
fn menu_ref_round_trips() {
    let menu = MenuRef { id: ui_text("context.tree-item"), args: Some(UiValue::Number(2.0)) };
    let first = serde_json::to_string(&menu).expect("serialize");
    let back: MenuRef = serde_json::from_str(&first).expect("deserialize");
    assert_eq!(menu, back);
}

#[test]
fn ui_intent_round_trips() {
    let intent = UiIntent {
        surface: crate::SurfaceId::try_from("note.play.navigator").expect("bounded surface id"),
        revision: crate::UiRevision(4),
        node: crate::UiNodeId(9),
        node_key: ui_text("row-9"),
        trigger: Trigger::Delta,
        action: ActionId::try_v1("cad-play", "objectMove").expect("bounded action id"),
        args: Some(UiValue::Number(1.0)),
        input: Some(UiValue::Number(-2.0)),
        seq: 42,
    };
    let first = serde_json::to_string(&intent).expect("serialize");
    let deserialized: UiIntent = serde_json::from_str(&first).expect("deserialize");
    let second = serde_json::to_string(&deserialized).expect("re-serialize");
    assert_eq!(first, second);
    assert_eq!(intent, deserialized);
}

#[test]
fn every_trigger_variant_round_trips() {
    for trigger in [Trigger::Activate, Trigger::Change, Trigger::Commit, Trigger::Delta, Trigger::Drop, Trigger::Submit, Trigger::Abort, Trigger::RepeatLast, Trigger::HoverPreview] {
        let json = serde_json::to_string(&trigger).expect("serialize");
        let back: Trigger = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(trigger, back);
    }
}

