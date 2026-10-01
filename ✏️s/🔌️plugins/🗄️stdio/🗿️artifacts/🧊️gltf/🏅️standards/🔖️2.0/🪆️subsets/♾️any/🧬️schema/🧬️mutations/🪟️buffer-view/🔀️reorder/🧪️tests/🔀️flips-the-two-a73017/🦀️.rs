//! 🔀️ `reorder-buffer-views` implementation case `🔀️flips`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪟️buffer-view/🔀️reorder/🔀️flips/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderBufferViewsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-buffer-views");
    super::super::component::fixture_corpus_tests::assert_case("🪟️buffer-view/🔀️reorder/🔀️flips");
}
