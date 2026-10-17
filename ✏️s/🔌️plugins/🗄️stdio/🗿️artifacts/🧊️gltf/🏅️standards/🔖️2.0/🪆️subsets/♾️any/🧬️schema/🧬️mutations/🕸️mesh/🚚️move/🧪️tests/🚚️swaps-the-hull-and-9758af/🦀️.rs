//! 🚚️ `move-mesh` implementation case `🚚️swaps`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/🚚️move/🚚️swaps/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::move_mesh::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<MoveMeshMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "move-mesh");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🕸️mesh/🚚️move/🚚️swaps");
}
