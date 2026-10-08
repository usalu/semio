//! 🔬️ `delete-node` implementation case `🔬️middle-row`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/🗑️delete/🔬️middle-row/` deletes a MIDDLE row, so the law checks that the concrete inverse restores
//! the row at its original index and that the inverse diffs sum to the negative of the forward diff.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteNodeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "delete-node");
    super::super::component::fixture_corpus_tests::assert_case("🌳️node/🗑️delete/🔬️middle-row");
}
