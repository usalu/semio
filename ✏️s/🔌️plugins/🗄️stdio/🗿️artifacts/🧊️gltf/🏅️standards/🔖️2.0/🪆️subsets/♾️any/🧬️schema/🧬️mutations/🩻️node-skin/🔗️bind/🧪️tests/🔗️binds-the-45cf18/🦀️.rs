//! 🔗️ `bind-node-skin` implementation case `🔗️binds`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🩻️node-skin/🔗️bind/🔗️binds/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindNodeSkinMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-node-skin");
    super::super::component::fixture_corpus_tests::assert_case("🩻️node-skin/🔗️bind/🔗️binds");
}
