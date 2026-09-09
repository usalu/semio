use super::*;

#[test]
fn semantic_identity_matches_the_language_neutral_descriptor() {
    assert_eq!(<ChangeMeshExtensionDataMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-mesh-extension-data");
}
