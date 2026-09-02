use super::*;
use protocol::{Mutation, MutationDiff, MutationLeaf, OpBinary, OpText};

#[test]
fn local_interaction_mutation_leaf_descriptor_and_exact_codecs_are_owned() {
    let source: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture/🔣️.json")).unwrap();
    let descriptor: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let state: protocol::InteractionState = serde_json::from_value(source.clone()).unwrap();
    let mutation = InteractionConfigMutation::set_state(state.clone());
    assert_eq!(InteractionConfigMutation::DESCRIPTORS.len(), 1);
    assert_eq!(serde_json::to_value(mutation.descriptor()).unwrap(), descriptor);
    assert_eq!(mutation.descriptor(), &SetInteractionState::DESCRIPTOR);
    assert!(SetInteractionState::PROVENANCE.source_path.ends_with("/🔁️set-state/🦀️.rs"));
    assert_eq!(SetInteractionState::PROVENANCE.owner, mutation.descriptor().owner);
    let text = mutation.print_op();
    assert_eq!(serde_json::from_str::<serde_json::Value>(text.strip_prefix("set-interaction-state ").unwrap()).unwrap(), source);
    assert_eq!(InteractionConfigMutation::parse_op(&text).unwrap(), mutation);
    let binary = mutation.encode_op().unwrap();
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&binary).unwrap(), source);
    assert_eq!(InteractionConfigMutation::decode_op(&binary).unwrap(), mutation);
    assert_eq!(mutation.apply(&protocol::InteractionState::default()).unwrap(), state);
    let inverse = mutation.inverse(&protocol::InteractionState::default());
    assert_eq!(inverse[0].apply(&state).unwrap(), protocol::InteractionState::default());
}
