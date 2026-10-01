//! 🔬️ `reorder-morph-target-attributes` implementation case `🔬️t046`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎚️morph/🔀️reorder/🔬️t046/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderMorphTargetAttributesMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-morph-target-attributes");
    super::super::component::fixture_corpus_tests::assert_case("🎚️morph-attribute/🔀️reorder/🔬️t046");
}
