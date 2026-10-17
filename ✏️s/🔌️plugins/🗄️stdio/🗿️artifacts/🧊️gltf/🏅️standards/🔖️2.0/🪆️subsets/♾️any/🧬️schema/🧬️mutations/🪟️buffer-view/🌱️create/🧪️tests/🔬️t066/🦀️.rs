//! 🔬️ `create-buffer-view` implementation case `🔬️t066`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪟️buffer-view/🌱️create/🔬️t066/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::create_buffer_view::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateBufferViewMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "create-buffer-view");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🪟️buffer-view/🌱️create/🔬️t066");
}
