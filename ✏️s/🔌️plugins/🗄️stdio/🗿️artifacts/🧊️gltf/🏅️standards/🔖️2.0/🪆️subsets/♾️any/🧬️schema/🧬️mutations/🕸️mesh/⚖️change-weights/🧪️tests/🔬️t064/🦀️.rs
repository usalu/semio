//! 🔬️ `change-mesh-morph-weights` implementation case `🔬️t064`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/⚖️change-weights/🔬️t064/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMeshMorphWeightsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-mesh-morph-weights");
    super::super::component::fixture_corpus_tests::assert_case("🕸️mesh/⚖️change-weights/🔬️t064");
}
