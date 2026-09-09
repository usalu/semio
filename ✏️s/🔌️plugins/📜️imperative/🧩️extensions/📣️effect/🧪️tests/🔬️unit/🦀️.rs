use super::*;

#[semio_framework_async_macros::async_test]
async fn bundle_contributes_core_module_for_imperative_play() {
    let entry = imperative_module_contribution();
    assert_eq!(entry.plugin_id, EXTENSION_ID);
    let topic_contribution = entry.topic_contribution.expect("imperative module topic contribution");
    assert_eq!(topic_contribution.topic, "imperative.module");
    let payload = topic_contribution.payload;
    assert_eq!(payload["appId"].as_str(), Some(imperative_extension_sdk::IMPERATIVE_PLAY_APP_ID));
    assert_eq!(payload["moduleId"].as_str(), Some("core"));
    assert!(payload["manifestJson"].as_str().unwrap_or_default().contains("imperative.extension"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_json_includes_input_channels() {
    let registry = module_registry();
    let raw = catalogue_json(&registry);
    let parsed = pack::json::parse(&raw).expect("catalogue json");
    let items = parsed.get("sections").and_then(JsonValue::as_array).and_then(|sections| sections.first()).and_then(|section| section.get("items")).and_then(JsonValue::as_array).expect("catalogue items");
    let message = items.iter().find(|item| item.get("kind").and_then(JsonValue::as_str) == Some("log.print")).and_then(|item| item.get("inputs")).and_then(JsonValue::as_array).and_then(|inputs| inputs.first()).expect("log.print inputs");
    assert_eq!(message.get("name").and_then(JsonValue::as_str), Some("message"));
    assert_eq!(message.get("code").and_then(JsonValue::as_str), Some("S"));
}

#[semio_framework_async_macros::async_test]
async fn state_increment_updates_counter() {
    let registry = module_registry();
    let input = Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("by", Value::Atom(Atom::Decimal(2.0))).insert("counter", Value::Atom(Atom::Decimal(5.0)));
    let output = registry.dispatch("state.increment", &input).expect("dispatch");
    let value = output.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    assert_eq!(value, Some(7.0));
}
