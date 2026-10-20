//! 🚫️ `reorder-node-children` implementation case `🚫️refuses`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌿️node-child/🔀️reorder/🚫️refuses/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_node_children::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderNodeChildrenMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "reorder-node-children");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🌿️node-child/🔀️reorder/🚫️refuses");
}
