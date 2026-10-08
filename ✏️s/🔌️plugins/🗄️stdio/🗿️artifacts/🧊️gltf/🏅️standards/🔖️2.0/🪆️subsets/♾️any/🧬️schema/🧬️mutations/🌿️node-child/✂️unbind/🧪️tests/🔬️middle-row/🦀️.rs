//! 🔬️ `unbind-node-child` implementation case `🔬️middle-row`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌿️node-child/✂️unbind/🔬️middle-row/` deletes a MIDDLE row, so the law checks that the concrete inverse restores
//! the row at its original index and that the inverse diffs sum to the negative of the forward diff.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<UnbindNodeChildMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "unbind-node-child");
    super::super::component::fixture_corpus_tests::assert_case("🌿️node-child/✂️unbind/🔬️middle-row");
}
