//! 🔀️ `reorder-morph-targets` implementation case `🔀️flips-the-two-14d32f`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🧬️morph-target/🔀️reorder/🔀️flips-the-two-14d32f/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderMorphTargetsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-morph-targets");
    super::super::component::fixture_corpus_tests::assert_case("🧬️morph-target/🔀️reorder/🔀️flips-the-two-14d32f");
}
