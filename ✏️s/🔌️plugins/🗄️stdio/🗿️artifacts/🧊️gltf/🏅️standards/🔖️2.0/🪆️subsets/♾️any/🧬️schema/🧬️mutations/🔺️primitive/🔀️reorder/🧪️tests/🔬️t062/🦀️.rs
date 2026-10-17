//! 🔬️ `reorder-primitives` implementation case `🔬️t062`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔺️primitive/🔀️reorder/🔬️t062/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_primitives::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderPrimitivesMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "reorder-primitives");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🔺️primitive/🔀️reorder/🔬️t062");
}
