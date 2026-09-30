//! 🔀️ `reorder-accessors` implementation case `🔀️flips-the-two-e56161`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📐️accessor/🔀️reorder/🔀️flips-the-two-e56161/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderAccessorsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-accessors");
    super::super::component::fixture_corpus_tests::assert_case("📐️accessor/🔀️reorder/🔀️flips-the-two-e56161");
}
