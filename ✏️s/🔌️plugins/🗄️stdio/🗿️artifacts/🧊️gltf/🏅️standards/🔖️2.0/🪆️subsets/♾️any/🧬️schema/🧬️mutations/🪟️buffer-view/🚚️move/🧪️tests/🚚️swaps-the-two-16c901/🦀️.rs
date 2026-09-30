//! 🚚️ `move-buffer-view` implementation case `🚚️swaps-the-two-16c901`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪟️buffer-view/🚚️move/🚚️swaps-the-two-16c901/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveBufferViewMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-buffer-view");
    super::super::component::fixture_corpus_tests::assert_case("🪟️buffer-view/🚚️move/🚚️swaps-the-two-16c901");
}
