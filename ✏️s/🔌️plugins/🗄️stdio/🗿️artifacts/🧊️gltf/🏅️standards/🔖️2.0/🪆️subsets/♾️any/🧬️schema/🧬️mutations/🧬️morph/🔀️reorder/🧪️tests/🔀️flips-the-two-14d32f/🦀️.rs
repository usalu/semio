//! 🔀️ `reorder-morph-targets` implementation case `🔀️flips`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🧬️morph/🔀️reorder/🔀️flips/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderMorphTargetsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-morph-targets");
    super::super::component::fixture_corpus_tests::assert_case("🧬️morph/🔀️reorder/🔀️flips");
}
