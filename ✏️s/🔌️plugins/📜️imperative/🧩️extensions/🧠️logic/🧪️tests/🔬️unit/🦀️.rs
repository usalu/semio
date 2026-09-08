
use super::*;

#[semio_framework_async_macros::async_test]
async fn logic_compare_gt() {
    let registry = module_registry();
    let input = Dictionary::new()
        .insert("left", Value::Atom(Atom::String("a".into())))
        .insert("right", Value::Atom(Atom::String("b".into())))
        .insert("operator", Value::Atom(Atom::String("gt".into())))
        .insert("into", Value::Atom(Atom::String("flag".into())))
        .insert("a", Value::Atom(Atom::Decimal(5.0)))
        .insert("b", Value::Atom(Atom::Decimal(2.0)));
    let output = registry.dispatch("logic.compare", &input).expect("dispatch");
    let value = output.get("flag").and_then(|v| v.as_atom()).and_then(|a| a.as_bool());
    assert_eq!(value, Some(true));
}
