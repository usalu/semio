//! 🔬️ `change-asset-descriptive-metadata` implementation case `🔬️t068`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪪️asset/📝️change-description/🔬️t068/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_descriptive_metadata::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeAssetDescriptiveMetadataMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-asset-descriptive-metadata");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🪪️asset/📝️change/🔬️t068");
}
