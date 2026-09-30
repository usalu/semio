//! 🔬️ `reorder-primitives` implementation case `🔬️t062`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔺️primitive/🔀️reorder/🔬️t062/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderPrimitivesMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-primitives");
    super::super::component::fixture_corpus_tests::assert_case("🔺️primitive/🔀️reorder/🔬️t062");
}
