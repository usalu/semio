//! 🚚️ `move-node` implementation case `🚚️swaps-the-two-scene-d73df8`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/🚚️move/🚚️swaps-the-two-scene-d73df8/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveNodeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-node");
    super::super::component::fixture_corpus_tests::assert_case("🌳️node/🚚️move/🚚️swaps-the-two-scene-d73df8");
}
