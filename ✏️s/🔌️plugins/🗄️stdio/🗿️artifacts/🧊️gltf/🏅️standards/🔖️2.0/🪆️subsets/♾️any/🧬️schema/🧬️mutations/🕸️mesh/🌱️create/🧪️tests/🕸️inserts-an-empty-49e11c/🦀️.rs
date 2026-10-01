//! 🕸️ `create-mesh` implementation case `🕸️inserts`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/🌱️create/🕸️inserts/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateMeshMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-mesh");
    super::super::component::fixture_corpus_tests::assert_case("🕸️mesh/🌱️create/🕸️inserts");
}
