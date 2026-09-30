//! 🔀️ `reorder-materials` implementation case `🔀️flips-the-glass-98d5db`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💎️material/🔀️reorder/🔀️flips-the-glass-98d5db/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderMaterialsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-materials");
    super::super::component::fixture_corpus_tests::assert_case("💎️material/🔀️reorder/🔀️flips-the-glass-98d5db");
}
