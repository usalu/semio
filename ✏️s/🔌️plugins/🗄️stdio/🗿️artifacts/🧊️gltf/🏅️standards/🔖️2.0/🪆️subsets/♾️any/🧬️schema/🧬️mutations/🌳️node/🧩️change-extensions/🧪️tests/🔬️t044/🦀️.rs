//! 🔬️ `change-node-extension-data` implementation case `🔬️t044`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🌳️node/🧩️change-extensions/🔬️t044/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_node_extension_data::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeNodeExtensionDataMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-node-extension-data");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🌳️node/🧩️change-extensions/🔬️t044");
}
