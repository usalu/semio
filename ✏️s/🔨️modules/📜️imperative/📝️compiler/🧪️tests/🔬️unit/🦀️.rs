
use super::*;
use neural_engine::Dictionary;
use std::collections::BTreeMap;

#[semio_framework_async_macros::async_test]
async fn compile_to_text_emits_one_line_per_step() {
    let path =
        Path { steps: vec![Step { id: "s1".into(), kind: "state.increment".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("by", Value::Atom(Atom::Decimal(5.0))), bodies: BTreeMap::new() }] };
    assert_eq!(compile_to_text(&path), "state.increment(by=5, key=\"counter\");");
}

#[semio_framework_async_macros::async_test]
async fn compile_to_text_emits_nested_control_blocks() {
    let mut bodies = BTreeMap::new();
    bodies.insert("then".into(), Path { steps: vec![Step { id: "t1".into(), kind: "log.print".into(), params: Dictionary::new().insert("message", Value::Atom(Atom::String("yes".into()))), bodies: BTreeMap::new() }] });
    let path = Path { steps: vec![Step { id: "s1".into(), kind: "control.if".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("flag".into()))), bodies }] };
    let text = compile_to_text(&path);
    assert!(text.contains("if (flag)"));
    assert!(text.contains("log.print"));
}
