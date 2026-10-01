//! 🔗️ `unbind-node-camera` implementation case `🔗️clears`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📷️node-camera/✂️unbind/🔗️clears/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindNodeCameraMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-node-camera");
    super::super::component::fixture_corpus_tests::assert_case("📷️node-camera/✂️unbind/🔗️clears");
}
