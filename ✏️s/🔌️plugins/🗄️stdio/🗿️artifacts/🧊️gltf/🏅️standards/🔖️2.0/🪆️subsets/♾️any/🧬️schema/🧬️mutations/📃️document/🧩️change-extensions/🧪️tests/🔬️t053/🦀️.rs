//! 🔬️ `change-document-extension-data` implementation case `🔬️t053`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📃️document/🧩️change-extensions/🔬️t053/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_document_extension_data::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeDocumentExtensionDataMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-document-extension-data");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("📃️document/🧩️change/🔬️t053");
}
