//! 🚫️ `reorder-node-children` implementation case `🚫️refuses`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌿️node-child/🔀️reorder/🚫️refuses/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderNodeChildrenMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-node-children");
    super::super::component::fixture_corpus_tests::assert_case("🌿️node-child/🔀️reorder/🚫️refuses");
}
