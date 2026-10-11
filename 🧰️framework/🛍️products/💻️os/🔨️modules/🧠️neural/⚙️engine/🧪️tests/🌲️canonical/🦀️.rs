//! 🌲️ Canonical tree projections of the neural owners reproduce the language-agnostic wire fixture and the first-party `ToValue` encoding.
use crate::{ColdRetire,Dictionary,Tree};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text,ArtifactCanonicalJsonTree as CanonicalTree,JsonMemberPolicy};
use semio_framework_value::{DslValue,FromValue,ToValue};

fn text(value:Text<'_>)->String {match value {Text::Contiguous(text)=>text.to_owned(),Text::Native(_)=>panic!("neural owners project contiguous keys only")}}
fn walk(node:&dyn CanonicalTree)->serde_json::Value {
    match node.canonical_tree_node().unwrap() {
        Node::Null=>serde_json::Value::Null,
        Node::Bool(value)=>value.into(),
        Node::I64(value)=>value.into(),
        Node::U64(value)=>value.into(),
        Node::F64(value)=>serde_json::json!(value),
        Node::String(value)=>value.into(),
        Node::Array(length)=>serde_json::Value::Array((0..length).map(|ordinal|walk(node.canonical_tree_child(ordinal).unwrap())).collect()),
        Node::Object(length)=>serde_json::Value::Object((0..length).map(|ordinal|(text(node.canonical_tree_key(ordinal).unwrap()),walk(node.canonical_tree_child(ordinal).unwrap()))).collect()),
        _=>panic!("neural owners project no wide scalar or native text"),
    }
}
fn fixture()->serde_json::Value {serde_json::from_str(include_str!("../../🧫️fixtures/🌲️canonical.json")).unwrap()}
fn decode<T:FromValue>(value:&serde_json::Value)->T {
    let text=serde_json::to_string(value).unwrap();
    T::from_value(semio_framework_pack_json::from_json_str::<DslValue>(&text,JsonMemberPolicy::Reject).unwrap()).unwrap()
}

#[test]
fn neural_tree_canonical_projection_matches_fixture_and_to_value() {
    let fixture=fixture();
    let tree:Tree=decode(&fixture["tree"]);
    let projected=walk(&tree);
    let encoded:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&tree.to_value())).unwrap();
    assert_eq!(projected,fixture["tree"]);
    assert_eq!(projected,encoded);
    tree.retire_decoded();
}

#[test]
fn neural_dictionary_canonical_projection_orders_entries_like_to_value() {
    let fixture=fixture();
    let dictionary:Dictionary=decode(&fixture["tree"]["neurons"][0]["params"]);
    let projected=walk(&dictionary);
    let encoded:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&dictionary.to_value())).unwrap();
    assert_eq!(projected,encoded);
    assert_eq!(projected,fixture["tree"]["neurons"][0]["params"]);
    assert!(matches!(dictionary.canonical_tree_node().unwrap(),Node::Object(length) if length==dictionary.len()));
    dictionary.retire_cold();
}
