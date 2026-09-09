use super::*;

#[semio_framework_async_macros::async_test]
async fn text_concat_writes_into_scope() {
    let registry = module_registry();
    let input = Dictionary::new().insert("left", Value::Atom(Atom::String("hello ".into()))).insert("right", Value::Atom(Atom::String("world".into()))).insert("into", Value::Atom(Atom::String("greeting".into())));
    let output = registry.dispatch("text.concat", &input).expect("dispatch");
    let value = output.get("greeting").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
    assert_eq!(value, Some("hello world"));
}

#[semio_framework_async_macros::async_test]
async fn text_uppercase_writes_into_scope() {
    let registry = module_registry();
    let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abc".into()))).insert("into", Value::Atom(Atom::String("upper".into())));
    let output = registry.dispatch("text.uppercase", &input).expect("dispatch");
    let value = output.get("upper").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
    assert_eq!(value, Some("ABC"));
}

#[semio_framework_async_macros::async_test]
async fn text_length_writes_into_scope() {
    let registry = module_registry();
    let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abcd".into()))).insert("into", Value::Atom(Atom::String("len".into())));
    let output = registry.dispatch("text.length", &input).expect("dispatch");
    let value = output.get("len").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    assert_eq!(value, Some(4.0));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_json_lists_text_operators() {
    let registry = module_registry();
    let raw = catalogue_json(&registry);
    assert!(raw.contains("text.uppercase"));
    assert!(raw.contains("\"id\":\"text\""));
}
