//! 🧪️ The interaction family's controlled native output and canonical tree are exactly its `ToValue` document, parsed by an independent serde_json.
use super::{DomainHover, DomainSelection, InteractionState, SelectionMode};
use semio_framework_value::{FromValue, NativeDecodeControl, NativeEncodeControl, ToValue};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree};
use std::collections::BTreeMap;

fn populated() -> InteractionState {
    InteractionState {
        selection: BTreeMap::from([
            ("scene".to_string(), DomainSelection { granularity: "object".into(), ids: vec!["a".into(), "b".into()], anchor_id: Some("a".into()) }),
            ("table".to_string(), DomainSelection { granularity: "row".into(), ids: Vec::new(), anchor_id: None }),
        ]),
        hover: BTreeMap::from([("scene".to_string(), DomainHover { channel: "pointer".into(), ids: vec!["b".into()] })]),
        active_mode: BTreeMap::from([("scene".to_string(), SelectionMode::Multiple), ("table".to_string(), SelectionMode::Single)]),
        active_granularity: BTreeMap::from([("scene".to_string(), "object".to_string())]),
    }
}

fn walk(tree: &dyn ArtifactCanonicalJsonTree) -> serde_json::Value {
    match tree.canonical_tree_node().expect("canonical node") {
        Node::Null => serde_json::Value::Null,
        Node::String(text) => text.into(),
        Node::Array(length) => serde_json::Value::Array((0..length).map(|ordinal| walk(tree.canonical_tree_child(ordinal).expect("canonical child"))).collect()),
        Node::Object(length) => serde_json::Value::Object((0..length).map(|ordinal| {
            let Text::Contiguous(key) = tree.canonical_tree_key(ordinal).expect("canonical key") else { panic!("interaction keys are contiguous") };
            (key.to_string(), walk(tree.canonical_tree_child(ordinal).expect("canonical child")))
        }).collect()),
        other => panic!("the interaction family has only text, array and object nodes, not {other:?}"),
    }
}

#[test]
fn the_controlled_native_output_is_the_to_value_document_for_every_shape() {
    for state in [InteractionState::default(), populated()] {
        let mut allow = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
        let mut control = NativeEncodeControl::new(1 << 20, &mut allow);
        assert_eq!(state.to_value_controlled(&mut control).expect("controlled native output"), state.to_value());
    }
}

#[test]
fn the_canonical_tree_is_the_to_value_document_for_every_shape() {
    for state in [InteractionState::default(), populated()] {
        assert_eq!(walk(&state), serde_json::to_value(state.to_value()).expect("independent serde_json document"));
    }
}

#[test]
fn the_canonical_tree_omits_exactly_an_absent_anchor_and_refuses_unknown_ordinals() {
    let absent = DomainSelection { granularity: "row".into(), ids: Vec::new(), anchor_id: None };
    assert!(matches!(absent.canonical_tree_node(), Ok(Node::Object(2))));
    assert!(absent.canonical_tree_child(2).is_err() && absent.canonical_tree_key(2).is_err());
    let anchored = DomainSelection { anchor_id: Some("a".into()), ..absent };
    assert!(matches!(anchored.canonical_tree_node(), Ok(Node::Object(3))));
}

#[test]
fn the_controlled_and_plain_decoders_round_trip_the_serde_json_document_of_every_shape() {
    for state in [InteractionState::default(), populated()] {
        let document = serde_json::to_string(&state.to_value()).expect("independent serde_json document");
        let parsed: semio_framework_value::DslValue = serde_json::from_str(&document).expect("the document parses back through an independent parser");
        assert_eq!(InteractionState::from_value(parsed.clone()).expect("plain decode"), state);
        let mut allow = |_: semio_framework_value::native_decoding::NativeDecodeProgress| true;
        let mut control = NativeDecodeControl::new(1 << 20, &mut allow);
        let decoded = InteractionState::from_value_controlled(&parsed, &mut control).expect("controlled decode");
        assert_eq!(decoded, state);
        decoded.retire_decoded();
    }
}

#[test]
fn the_serde_twin_and_the_value_twin_speak_the_same_keys_and_values() {
    for state in [InteractionState::default(), populated()] {
        assert_eq!(serde_json::to_value(&state).expect("serde twin"), serde_json::to_value(state.to_value()).expect("value twin"));
        let reread: InteractionState = serde_json::from_value(serde_json::to_value(&state).expect("serde twin")).expect("serde twin reads itself");
        assert_eq!(reread, state);
    }
}

#[test]
fn unknown_modes_and_wrong_shapes_are_refused_by_both_decoders() {
    let bad = serde_json::json!({"activeGranularity": {}, "activeMode": {"scene": "triple"}, "hover": {}, "selection": {}});
    let parsed: semio_framework_value::DslValue = serde_json::from_value(bad).expect("document parses");
    assert!(InteractionState::from_value(parsed.clone()).is_err());
    let mut allow = |_: semio_framework_value::native_decoding::NativeDecodeProgress| true;
    let mut control = NativeDecodeControl::new(1 << 20, &mut allow);
    assert!(InteractionState::from_value_controlled(&parsed, &mut control).is_err());
}
