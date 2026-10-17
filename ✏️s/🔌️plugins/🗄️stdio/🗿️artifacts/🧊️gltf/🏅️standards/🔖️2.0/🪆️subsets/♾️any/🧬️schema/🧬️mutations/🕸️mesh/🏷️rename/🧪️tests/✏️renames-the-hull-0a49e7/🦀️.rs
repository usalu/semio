//! ✏️ `change-mesh-name` implementation case `✏️renames`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🕸️mesh/🏷️rename/✏️renames/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_mesh_name::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMeshNameMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-mesh-name");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🕸️mesh/🏷️rename/✏️renames");
}
