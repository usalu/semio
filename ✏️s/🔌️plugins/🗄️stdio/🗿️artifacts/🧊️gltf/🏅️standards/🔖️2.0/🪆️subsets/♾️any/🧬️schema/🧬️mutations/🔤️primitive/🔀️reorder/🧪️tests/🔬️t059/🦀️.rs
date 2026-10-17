//! 🔬️ `reorder-primitive-attributes` implementation case `🔬️t059`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🔤️primitive/🔀️reorder/🔬️t059/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_primitive_attributes::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderPrimitiveAttributesMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "reorder-primitive-attributes");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🔤️primitive/🔀️reorder/🔬️t059");
}
