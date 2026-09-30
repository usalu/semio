//! 🔬️ `reorder-primitive-attributes` implementation case `🔬️t059`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔤️primitive-attribute/🔀️reorder/🔬️t059/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderPrimitiveAttributesMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-primitive-attributes");
    super::super::component::fixture_corpus_tests::assert_case("🔤️primitive-attribute/🔀️reorder/🔬️t059");
}
