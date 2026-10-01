//! 🚚️ `move-material` implementation case `🚚️swaps`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💎️material/🚚️move/🚚️swaps/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveMaterialMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-material");
    super::super::component::fixture_corpus_tests::assert_case("💎️material/🚚️move/🚚️swaps");
}
