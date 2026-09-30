//! 📤️ `unbind-scene-root-node` implementation case `📤️demotes-the-root-84e1cf`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌲️scene-root/✂️unbind/📤️demotes-the-root-84e1cf/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindSceneRootNodeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-scene-root-node");
    super::super::component::fixture_corpus_tests::assert_case("🌲️scene-root/✂️unbind/📤️demotes-the-root-84e1cf");
}
