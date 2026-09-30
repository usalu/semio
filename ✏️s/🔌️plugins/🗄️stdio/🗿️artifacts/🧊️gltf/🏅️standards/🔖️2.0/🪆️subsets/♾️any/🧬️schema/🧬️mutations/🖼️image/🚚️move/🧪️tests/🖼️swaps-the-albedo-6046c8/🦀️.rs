//! 🖼️ `move-image` implementation case `🖼️swaps-the-albedo-6046c8`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🖼️image/🚚️move/🖼️swaps-the-albedo-6046c8/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveImageMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-image");
    super::super::component::fixture_corpus_tests::assert_case("🖼️image/🚚️move/🖼️swaps-the-albedo-6046c8");
}
