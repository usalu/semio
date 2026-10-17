//! 🔬️ `change-mesh-morph-weights` implementation case `🔬️t064`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/⚖️change-weights/🔬️t064/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_morph_weights::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMeshMorphWeightsMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-mesh-morph-weights");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🕸️mesh/⚖️change-weights/🔬️t064");
}
