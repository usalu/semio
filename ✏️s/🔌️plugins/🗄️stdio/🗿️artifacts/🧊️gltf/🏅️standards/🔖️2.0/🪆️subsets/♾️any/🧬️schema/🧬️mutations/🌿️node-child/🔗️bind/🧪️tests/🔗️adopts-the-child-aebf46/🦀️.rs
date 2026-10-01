//! 🔗️ `bind-node-child` implementation case `🔗️adopts`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌿️node-child/🔗️bind/🔗️adopts/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindNodeChildMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "bind-node-child");
    super::super::component::fixture_corpus_tests::assert_case("🌿️node-child/🔗️bind/🔗️adopts");
}
