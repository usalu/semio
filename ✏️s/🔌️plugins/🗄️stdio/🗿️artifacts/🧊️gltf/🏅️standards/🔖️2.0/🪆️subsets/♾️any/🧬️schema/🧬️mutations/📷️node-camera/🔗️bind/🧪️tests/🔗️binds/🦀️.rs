//! 🔗️ `bind-node-camera` implementation case `🔗️binds`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📷️node-camera/🔗️bind/🔗️binds/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::bind_node_camera::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<BindNodeCameraMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "bind-node-camera");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("📷️node-camera/🔗️bind/🔗️binds");
}
