//! 🔗️ `bind-node-camera` implementation case `🔗️binds`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📷️node-camera/🔗️bind/🔗️binds/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindNodeCameraMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-node-camera");
    super::super::component::fixture_corpus_tests::assert_case("📷️node-camera/🔗️bind/🔗️binds");
}
