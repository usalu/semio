//! 🚚️ `move-accessor` implementation case `🚚️swaps-the-257557`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📐️accessor/🚚️move/🚚️swaps-the-257557/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveAccessorMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-accessor");
    super::super::component::fixture_corpus_tests::assert_case("📐️accessor/🚚️move/🚚️swaps-the-257557");
}
