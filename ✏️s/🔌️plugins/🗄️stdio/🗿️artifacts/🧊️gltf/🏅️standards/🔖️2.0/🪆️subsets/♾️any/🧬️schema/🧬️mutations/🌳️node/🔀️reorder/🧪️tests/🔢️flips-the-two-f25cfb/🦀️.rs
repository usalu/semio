//! 🔢️ `reorder-nodes` implementation case `🔢️flips`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/🔀️reorder/🔢️flips/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderNodesMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-nodes");
    super::super::component::fixture_corpus_tests::assert_case("🌳️node/🔀️reorder/🔢️flips");
}
