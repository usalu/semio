//! 🌲️ The canonical projection of CollectionMutation equals the first-party `ToValue` wire for every variant.
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

fn samples() -> Vec<CollectionMutation> {
    let folder = || CollectionFolder { id: "f".into(), parent_id: Some("p".into()), name: "F".into() };
    let entry = |body: ArtifactBody| CollectionEntry { id: "e".into(), folder_id: None, name: "E".into(), kind_id: "k".into(), body: Box::new(body) };
    let document = || ArtifactBody::Document { schema: "s".into(), document_id: "d".into() };
    vec![
        CollectionMutation::RenameCollection { new_name: "n".into() },
        CollectionMutation::CreateFolder { folder: folder(), index: 1 },
        CollectionMutation::DeleteFolder { folder_id: "f".into() },
        CollectionMutation::MoveToCollection { folder_id: "f".into(), new_parent: None },
        CollectionMutation::RenameFolder { folder_id: "f".into(), new_name: "n".into() },
        CollectionMutation::CreateEntry { entry: entry(document()), index: 0 },
        CollectionMutation::DeleteEntry { entry_id: "e".into() },
        CollectionMutation::MoveToFolder { entry_id: "e".into(), new_folder: Some("f".into()) },
        CollectionMutation::RenameEntry { entry_id: "e".into(), new_name: "n".into() },
        CollectionMutation::ReplaceEntryBody { entry_id: "e".into(), new_body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "h".into(), size: 3, media_type: "m".into() } }) },
    ]
}

#[test]
fn collection_mutation_canonical_tree_is_byte_identical_to_the_value_wire_for_every_variant() {
    for mutation in samples() {
        let expected: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&mutation.to_value())).unwrap();
        assert_eq!(walk(&mutation), expected, "{mutation:?}");
    }
}
