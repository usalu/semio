//! 🔬️ `move-node-child` implementation case `🔬️t045`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌿️node-child/🚚️move/🔬️t045/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveNodeChildMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-node-child");
    super::super::component::fixture_corpus_tests::assert_case("🌿️node-child/🚚️move/🔬️t045");
}
