//! ⏫️ `move-animation` implementation case `⏫️promotes`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎞️animation/🚚️move/⏫️promotes/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveAnimationMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-animation");
    super::super::component::fixture_corpus_tests::assert_case("🎞️animation/🚚️move/⏫️promotes");
}
