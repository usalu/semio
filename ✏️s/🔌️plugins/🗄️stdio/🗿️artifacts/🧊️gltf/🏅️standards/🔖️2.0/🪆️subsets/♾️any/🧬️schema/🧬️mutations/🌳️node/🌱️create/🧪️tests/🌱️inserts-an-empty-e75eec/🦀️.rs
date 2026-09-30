//! 🌱️ `create-node` implementation case `🌱️inserts-an-empty-e75eec`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/🌱️create/🌱️inserts-an-empty-e75eec/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateNodeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-node");
    super::super::component::fixture_corpus_tests::assert_case("🌳️node/🌱️create/🌱️inserts-an-empty-e75eec");
}
