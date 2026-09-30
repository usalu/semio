//! 🔀️ `reorder-meshs` implementation case `🔀️flips-the-hull-ebfffa`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/🔀️reorder/🔀️flips-the-hull-ebfffa/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderMeshsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-meshs");
    super::super::component::fixture_corpus_tests::assert_case("🕸️mesh/🔀️reorder/🔀️flips-the-hull-ebfffa");
}
