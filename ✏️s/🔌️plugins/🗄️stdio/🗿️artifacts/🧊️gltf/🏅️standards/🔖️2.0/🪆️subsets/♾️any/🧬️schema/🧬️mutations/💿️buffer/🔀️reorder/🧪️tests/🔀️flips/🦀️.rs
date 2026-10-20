//! 🔀️ `reorder-buffers` implementation case `🔀️flips`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💿️buffer/🔀️reorder/🔀️flips/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::reorder_buffers::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderBuffersMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "reorder-buffers");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("💿️buffer/🔀️reorder/🔀️flips");
}
