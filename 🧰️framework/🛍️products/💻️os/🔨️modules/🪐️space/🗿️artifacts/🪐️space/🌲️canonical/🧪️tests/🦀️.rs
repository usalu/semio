//! 🌲️ The canonical projection of SpaceMutation equals the first-party `ToValue` wire for every variant.
use super::*;
use crate::*;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as CanonicalTree};
use semio_framework_value::ToValue;

fn text(value: Text<'_>) -> String { match value { Text::Contiguous(text) => text.to_owned(), Text::Native(_) => panic!("mutation keys are contiguous") } }
fn walk(node: &dyn CanonicalTree) -> serde_json::Value {
    match node.canonical_tree_node().unwrap() {
        Node::Null => serde_json::Value::Null,
        Node::Bool(value) => value.into(),
        Node::I64(value) => value.into(),
        Node::U64(value) => value.into(),
        Node::F64(value) => serde_json::json!(value),
        Node::String(value) => value.into(),
        Node::Array(length) => serde_json::Value::Array((0..length).map(|ordinal| walk(node.canonical_tree_child(ordinal).unwrap())).collect()),
        Node::Object(length) => serde_json::Value::Object((0..length).map(|ordinal| (text(node.canonical_tree_key(ordinal).unwrap()), walk(node.canonical_tree_child(ordinal).unwrap()))).collect()),
        _ => panic!("mutation trees project no wide scalar"),
    }
}

fn samples() -> Vec<SpaceMutation> {
    let user = || SpaceUser { id: "u".into(), name: "User".into(), avatar: Some("a".into()), role: SpaceRole::Author };
    let collection = || CollectionRef { id: "c".into(), name: "Col".into(), document_id: "d".into() };
    vec![
        SpaceMutation::SetName { name: "n".into() },
        SpaceMutation::SetKind { kind: SpaceKind::Studio },
        SpaceMutation::SetVisibility { visibility: SpaceVisibility::Private },
        SpaceMutation::UpsertUser { user: user(), index: Some(2) },
        SpaceMutation::RemoveUser { user_id: "u".into() },
        SpaceMutation::AddCollection { collection: collection(), index: None },
        SpaceMutation::RemoveCollection { collection_id: "c".into() },
        SpaceMutation::RenameCollection { collection_id: "c".into(), name: "x".into() },
        SpaceMutation::InstallProgram { plugin_id: "p".into(), index: Some(0) },
        SpaceMutation::UninstallProgram { plugin_id: "p".into() },
        SpaceMutation::InstallExtension { extension_id: "e".into(), version: "1".into(), source_uri: "s".into(), package_hash: "h".into(), enabled: true, index: None },
        SpaceMutation::UninstallExtension { extension_id: "e".into() },
        SpaceMutation::SetExtensionEnabled { extension_id: "e".into(), enabled: false },
    ]
}

#[test]
fn space_mutation_canonical_tree_is_byte_identical_to_the_value_wire_for_every_variant() {
    for mutation in samples() {
        let expected: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&mutation.to_value())).unwrap();
        assert_eq!(walk(&mutation), expected, "{mutation:?}");
    }
}
