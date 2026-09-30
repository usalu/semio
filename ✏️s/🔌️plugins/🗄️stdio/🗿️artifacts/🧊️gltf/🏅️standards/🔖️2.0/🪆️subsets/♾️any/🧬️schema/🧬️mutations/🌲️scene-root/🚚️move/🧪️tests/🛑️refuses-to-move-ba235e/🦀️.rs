//! 🛑️ `move-scene-root-node` implementation case `🛑️refuses-to-move-ba235e`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌲️scene-root/🚚️move/🛑️refuses-to-move-ba235e/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveSceneRootNodeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-scene-root-node");
    super::super::component::fixture_corpus_tests::assert_case("🌲️scene-root/🚚️move/🛑️refuses-to-move-ba235e");
}
