use super::*;
use protocol::{Mutation, MutationDiff, MutationLeaf};

#[test]
fn direct_leaf_descriptor_and_inverse_law() {
    assert!(AddValue::DESCRIPTOR.validate().is_ok());
    assert_eq!(AddValue::DESCRIPTOR.semantic_kind, "add-value");
    let provenance = AddValue::PROVENANCE;
    assert_eq!(provenance.owner, AddValue::DESCRIPTOR.owner);
    let scope = protocol::MutationLeafSourceScope {
        workspace_token: provenance.workspace_token,
        mutation_root: provenance.mutation_root,
        owner_layout: protocol::MutationOwnerLayout::Flat,
        taxonomy_path: provenance.taxonomy_path,
        mutation_payload_facet: "🦠️mutation",
        source_filename: "🦀️.rs",
        descriptor_filename: "🔣️.json",
    };
    assert!(protocol::validate_mutation_leaf_source(&AddValue::DESCRIPTOR, &provenance, &scope).is_ok());
    let base = WireTestSnapshot { value: 0 };
    let mutation = WireTestMutation::AddValue(AddValue { delta: i32::MIN });
    let current = mutation.diff(&base).diff().apply(&base).expect("minimum applies");
    let inverse = mutation.inverse(&base);
    assert_eq!(inverse, vec![WireTestMutation::AddValue(AddValue { delta: 1 }), WireTestMutation::AddValue(AddValue { delta: i32::MAX })]);
    let restored = inverse.iter().rev().try_fold(current, |snapshot, next| next.diff(&snapshot).diff().apply(&snapshot)).expect("stored reverse inverse");
    assert_eq!(restored, base);
}
