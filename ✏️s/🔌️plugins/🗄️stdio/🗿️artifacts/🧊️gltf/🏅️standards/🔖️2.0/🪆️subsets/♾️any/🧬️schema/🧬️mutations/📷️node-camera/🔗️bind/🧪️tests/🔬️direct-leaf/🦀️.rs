use super::*;

#[test]
fn semantic_identity_matches_the_language_neutral_descriptor() {
    assert_eq!(<BindNodeCameraMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-node-camera");
}
