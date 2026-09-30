//! 🔀️ `reorder-animations` implementation case `🔀️flips-the-spin-5be673`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎞️animation/🔀️reorder/🔀️flips-the-spin-5be673/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderAnimationsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-animations");
    super::super::component::fixture_corpus_tests::assert_case("🎞️animation/🔀️reorder/🔀️flips-the-spin-5be673");
}
