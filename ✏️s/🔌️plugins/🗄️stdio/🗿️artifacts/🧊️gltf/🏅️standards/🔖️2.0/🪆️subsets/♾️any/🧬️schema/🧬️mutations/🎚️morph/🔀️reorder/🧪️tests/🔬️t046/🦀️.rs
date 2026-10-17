//! 🔬️ `reorder-morph-target-attributes` implementation case `🔬️t046`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎚️morph/🔀️reorder/🔬️t046/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_morph_target_attributes::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderMorphTargetAttributesMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "reorder-morph-target-attributes");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🎚️morph/🔀️reorder/🔬️t046");
}
