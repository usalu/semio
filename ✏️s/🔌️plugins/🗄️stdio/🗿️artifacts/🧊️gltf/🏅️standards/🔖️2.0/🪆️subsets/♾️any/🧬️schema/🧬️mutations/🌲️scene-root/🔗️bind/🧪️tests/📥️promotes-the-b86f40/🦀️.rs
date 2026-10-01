//! 📥️ `bind-scene-root-node` implementation case `📥️promotes`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌲️scene-root/🔗️bind/📥️promotes/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindSceneRootNodeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-scene-root-node");
    super::super::component::fixture_corpus_tests::assert_case("🌲️scene-root/🔗️bind/📥️promotes");
}
