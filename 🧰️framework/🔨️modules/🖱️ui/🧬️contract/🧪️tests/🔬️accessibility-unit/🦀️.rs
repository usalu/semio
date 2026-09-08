
use super::*;

fn ui_text(value: &str) -> crate::UiText {
    crate::UiText::try_from_str(value).expect("bounded fixture text")
}

#[test]
fn default_spec_serializes_to_empty_object() {
    let json = serde_json::to_value(AccessibilitySpec::default()).expect("serialize");
    assert_eq!(json, serde_json::json!({}));
}

#[test]
fn liveness_field_omitted_at_default() {
    let json = serde_json::to_value(AccessibilitySpec { live: Liveness::Assertive, ..AccessibilitySpec::default() }).expect("serialize");
    assert_eq!(json, serde_json::json!({ "live": "assertive" }));
}

#[test]
fn hidden_field_omitted_at_default() {
    let json = serde_json::to_value(AccessibilitySpec { hidden: true, ..AccessibilitySpec::default() }).expect("serialize");
    assert_eq!(json, serde_json::json!({ "hidden": true }));
}

#[test]
fn shortcut_roundtrips() {
    let spec = AccessibilitySpec { shortcut: Some(ui_text("Ctrl+S")), live: Liveness::Polite, hidden: false, ..AccessibilitySpec::default() };
    let json = serde_json::to_string(&spec).expect("serialize");
    let back: AccessibilitySpec = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(spec, back);
}
