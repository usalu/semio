//! 🏷️ Canonical projections preserve the wire shape of internally tagged payloads.
use super::*;

#[derive(semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = crate)]
struct TaggedPayloadRecord {
    text: String,
}

#[derive(semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = crate)]
#[value(tag = "kind", rename_all = "camelCase")]
enum TaggedPayload {
    Number(f64),
    Items(Vec<u32>),
    Record(TaggedPayloadRecord),
    Empty,
}

fn projected_json(tree: &dyn ArtifactCanonicalJsonTree) -> serde_json::Value {
    match tree.canonical_tree_node().unwrap() {
        Node::Object(size) => {
            let fields = (0..size).map(|ordinal| {
                let ArtifactCanonicalJsonText::Contiguous(key) = tree.canonical_tree_key(ordinal).unwrap() else {
                    panic!("tagged fixture has a non-contiguous key")
                };
                (key.to_owned(), projected_json(tree.canonical_tree_child(ordinal).unwrap()))
            });
            serde_json::Value::Object(fields.collect())
        }
        Node::Array(size) => serde_json::Value::Array(
            (0..size).map(|ordinal| projected_json(tree.canonical_tree_child(ordinal).unwrap())).collect(),
        ),
        Node::String(text) | Node::Text(ArtifactCanonicalJsonText::Contiguous(text)) => text.into(),
        scalar => {
            let bytes = ScalarBytes::from_node(scalar).unwrap();
            serde_json::from_slice(&bytes.bytes[..bytes.length]).unwrap()
        }
    }
}

#[test]
fn canonical_internal_tags_wrap_non_object_payloads_under_value() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected = fixture["internallyTaggedPayloads"].as_array().unwrap();
    let values = [
        TaggedPayload::Number(42.5),
        TaggedPayload::Items(vec![2, 3]),
        TaggedPayload::Record(TaggedPayloadRecord { text: "kept".into() }),
        TaggedPayload::Empty,
    ];
    assert_eq!(values.len(), expected.len());
    for (value, expected) in values.into_iter().zip(expected) {
        assert_eq!(&projected_json(&value), expected);
    }
}
