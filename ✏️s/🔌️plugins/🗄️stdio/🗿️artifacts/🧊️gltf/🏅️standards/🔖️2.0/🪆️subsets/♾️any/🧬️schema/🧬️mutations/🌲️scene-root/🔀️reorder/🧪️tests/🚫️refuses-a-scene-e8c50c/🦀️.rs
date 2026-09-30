//! 🚫️ `reorder-scene-root-nodes` implementation case `🚫️refuses-a-scene-e8c50c`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌲️scene-root/🔀️reorder/🚫️refuses-a-scene-e8c50c/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderSceneRootNodesMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-scene-root-nodes");
    super::super::component::fixture_corpus_tests::assert_case("🌲️scene-root/🔀️reorder/🚫️refuses-a-scene-e8c50c");
}
