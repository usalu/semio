//! 🔬️ `change-document-extra-data` implementation case `🔬️t052`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/📃️document/📝️change-extras/🔬️t052/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_document_extra_data::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeDocumentExtraDataMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-document-extra-data");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("📃️document/📝️change/🔬️t052");
}
