//! 🔬️ `change-asset-extra-data` implementation case `🔬️t070`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🪪️asset/🧾️change-extras/🔬️t070/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_asset_extra_data::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeAssetExtraDataMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-asset-extra-data");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("🪪️asset/🧾️change-extras/🔬️t070");
}
