//! 🔀️ `change-node-transform` implementation case `🔀️replaces`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/📐️transform/🔀️replaces/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_node_transform::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeNodeTransformMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-node-transform");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🌳️node/📐️transform/🔀️replaces");
}
