//! 🚫️ `delete-buffer` implementation case `🚫️removes`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💿️buffer/🗑️delete/🚫️removes/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteBufferMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "delete-buffer");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("💿️buffer/🗑️delete/🚫️removes");
}
