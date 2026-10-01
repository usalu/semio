//! 🌿️ `move-node-parent` implementation case `🌿️adopts`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/🌿️reparent/🌿️adopts/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveNodeParentMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-node-parent");
    super::super::component::fixture_corpus_tests::assert_case("🌳️node/🌿️reparent/🌿️adopts");
}
