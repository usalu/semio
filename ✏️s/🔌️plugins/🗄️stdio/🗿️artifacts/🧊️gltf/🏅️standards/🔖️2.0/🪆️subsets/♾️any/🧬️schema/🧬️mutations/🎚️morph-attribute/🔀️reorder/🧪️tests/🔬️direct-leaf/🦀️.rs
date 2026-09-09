use super::*;

#[test]
fn semantic_identity_matches_the_language_neutral_descriptor() {
    assert_eq!(<ReorderMorphTargetAttributesMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-morph-target-attributes");
}
