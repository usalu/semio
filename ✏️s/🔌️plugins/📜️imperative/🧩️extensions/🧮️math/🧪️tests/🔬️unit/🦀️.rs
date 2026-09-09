use super::*;

#[semio_framework_async_macros::async_test]
async fn math_add_writes_into_scope() {
    let registry = module_registry();
    let input = Dictionary::new().insert("a", Value::Atom(Atom::Decimal(2.0))).insert("b", Value::Atom(Atom::Decimal(3.0))).insert("into", Value::Atom(Atom::String("sum".into())));
    let output = registry.dispatch("math.add", &input).expect("dispatch");
    let value = output.get("sum").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    assert_eq!(value, Some(5.0));
}
