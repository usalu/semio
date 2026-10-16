//! 🧪️ Aggregated unit tests of the `extensions` leaves, one module per leaf.

mod control {
use super::super::control::*;

#[semio_framework_async_macros::async_test]
async fn catalogue_includes_control_kinds() {
    let raw = catalogue_json();
    assert!(raw.contains("control.if"));
    assert!(raw.contains("control.while"));
    assert!(raw.contains("control.repeat"));
}
}

mod logic {
use neural_engine::{Atom, Dictionary, Value};
use super::super::logic::*;

#[semio_framework_async_macros::async_test]
async fn logic_compare_gt() {
    let registry = neural_engine::ColdOwner::new(module_registry());
    let input = Dictionary::new()
        .insert("left", Value::Atom(Atom::String("a".into())))
        .insert("right", Value::Atom(Atom::String("b".into())))
        .insert("operator", Value::Atom(Atom::String("gt".into())))
        .insert("into", Value::Atom(Atom::String("flag".into())))
        .insert("a", Value::Atom(Atom::Decimal(5.0)))
        .insert("b", Value::Atom(Atom::Decimal(2.0)));
    let output = registry.dispatch_cold("logic.compare", input).expect("dispatch");
    let value = output.get("flag").and_then(|v| v.as_atom()).and_then(|a| a.as_bool());
    assert_eq!(value, Some(true));
}
}

mod math {
use neural_engine::{Atom, Dictionary, Value};
use super::super::math::*;

#[semio_framework_async_macros::async_test]
async fn math_add_writes_into_scope() {
    let registry = neural_engine::ColdOwner::new(module_registry());
    let input = Dictionary::new().insert("a", Value::Atom(Atom::Decimal(2.0))).insert("b", Value::Atom(Atom::Decimal(3.0))).insert("into", Value::Atom(Atom::String("sum".into())));
    let output = registry.dispatch_cold("math.add", input).expect("dispatch");
    let value = output.get("sum").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    assert_eq!(value, Some(5.0));
}
}

mod text {
use neural_engine::{Atom, Dictionary, Value};
use super::super::text::*;

#[semio_framework_async_macros::async_test]
async fn text_concat_writes_into_scope() {
    let registry = neural_engine::ColdOwner::new(module_registry());
    let input = Dictionary::new().insert("left", Value::Atom(Atom::String("hello ".into()))).insert("right", Value::Atom(Atom::String("world".into()))).insert("into", Value::Atom(Atom::String("greeting".into())));
    let output = registry.dispatch_cold("text.concat", input).expect("dispatch");
    let value = output.get("greeting").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
    assert_eq!(value, Some("hello world"));
}

#[semio_framework_async_macros::async_test]
async fn text_uppercase_writes_into_scope() {
    let registry = neural_engine::ColdOwner::new(module_registry());
    let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abc".into()))).insert("into", Value::Atom(Atom::String("upper".into())));
    let output = registry.dispatch_cold("text.uppercase", input).expect("dispatch");
    let value = output.get("upper").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
    assert_eq!(value, Some("ABC"));
}

#[semio_framework_async_macros::async_test]
async fn text_length_writes_into_scope() {
    let registry = neural_engine::ColdOwner::new(module_registry());
    let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abcd".into()))).insert("into", Value::Atom(Atom::String("len".into())));
    let output = registry.dispatch_cold("text.length", input).expect("dispatch");
    let value = output.get("len").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    assert_eq!(value, Some(4.0));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_json_lists_text_operators() {
    let registry = neural_engine::ColdOwner::new(module_registry());
    let raw = catalogue_json(&registry);
    assert!(raw.contains("text.uppercase"));
    assert!(raw.contains("\"id\":\"text\""));
}
}
