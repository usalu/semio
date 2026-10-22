//! 🧬️ Remodeling artifact — document mutation dispatch enum. Every variant is a single-field tuple
//! wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/` triad
//! leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<RemodelingSnapshot>` and
//! `impl protocol::SemanticMutation<RemodelingSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use protocol::Mutation as _;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
/// 🧮️ Semantic remodeling document mutation vocabulary: id-keyed create/delete/change/add/remove per
/// collection (streams, assets, camera calibrations, rig extrinsics, GCPs), `update` for the 8
/// inseparable `ReconstructionParams` sub-facets and the calibration/rig full-record replace, and
/// `replace` for the engine-owned result sub-payloads, `append`/`truncate` for durable content leaves and
/// the atomic `commit-reconstruction` result.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RemodelingSnapshot, diff = RemodelingDiff, schema = "remodeling.scene", reference = RemodelingSnapshot, reference_diff = RemodelingDiff)]
pub enum RemodelingMutation {
    CreateStream(CreateStream),
    DeleteStream(DeleteStream),
    ChangeStreamSync(ChangeStreamSync),
    AddStreamFrame(AddStreamFrame),
    RemoveStreamFrame(RemoveStreamFrame),
    ReplaceStreamSource(ReplaceStreamSource),
    CreateAsset(CreateAsset),
    DeleteAsset(DeleteAsset),
    CreateCameraCalibration(CreateCameraCalibration),
    UpdateCameraCalibration(UpdateCameraCalibration),
    DeleteCameraCalibration(DeleteCameraCalibration),
    CreateRigExtrinsic(CreateRigExtrinsic),
    DeleteRigExtrinsic(DeleteRigExtrinsic),
    UpdateRigExtrinsic(UpdateRigExtrinsic),
    CreateGcp(CreateGcp),
    DeleteGcp(DeleteGcp),
    AddGcpObservation(AddGcpObservation),
    RemoveGcpObservation(RemoveGcpObservation),
    UpdateIngestParams(UpdateIngestParams),
    UpdateFeatureParams(UpdateFeatureParams),
    UpdateMatchParams(UpdateMatchParams),
    UpdateSfmParams(UpdateSfmParams),
    UpdateDenseParams(UpdateDenseParams),
    UpdateMeshParams(UpdateMeshParams),
    UpdateMotionParams(UpdateMotionParams),
    UpdateGeoParams(UpdateGeoParams),
    ReplaceSparse(ReplaceSparse),
    ReplaceDense(ReplaceDense),
    /// 📦️ Boxed: `RemodelingMesh` (a full `MeshData` plus an optional watertight snapshot) is far larger
    /// than any sibling variant, and `clippy::large_enum_variant` flags the resulting size disparity
    /// across `RemodelingMutation`/`RemodelingDiff` — boxing keeps every other variant cheap to move.
    ReplaceMeshResult(ReplaceMeshResult),
    ReplaceTrajectory(ReplaceTrajectory),
    ReplaceTracks(ReplaceTracks),
    ReplaceGeoProducts(ReplaceGeoProducts),
    ReplaceQc(ReplaceQc),
    AppendContent(AppendContent),
    RemoveContent(RemoveContent),
    CommitReconstruction(CommitReconstruction),
}
//#endregion 🔖️Mutations

//#region 🔖️LeafPayloads
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::append_content::AppendContent;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::add_gcp_observation::AddGcpObservation;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::add_stream_frame::AddStreamFrame;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::change_stream_sync::ChangeStreamSync;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::CommitReconstruction;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::create_camera_calibration::CreateCameraCalibration;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::create_gcp::CreateGcp;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::create_rig_extrinsic::CreateRigExtrinsic;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::create_stream::CreateStream;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::delete_asset::DeleteAsset;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::delete_camera_calibration::DeleteCameraCalibration;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::delete_gcp::DeleteGcp;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::delete_rig_extrinsic::DeleteRigExtrinsic;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::delete_stream::DeleteStream;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::remove_gcp_observation::RemoveGcpObservation;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::remove_stream_frame::RemoveStreamFrame;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::replace_dense::ReplaceDense;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::replace_geo_products::ReplaceGeoProducts;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::replace_mesh_result::ReplaceMeshResult;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::replace_qc::ReplaceQc;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::replace_sparse::ReplaceSparse;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::replace_stream_source::ReplaceStreamSource;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::replace_tracks::ReplaceTracks;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::replace_trajectory::ReplaceTrajectory;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::update_camera_calibration::UpdateCameraCalibration;
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_dense_params::UpdateDenseParams;
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_feature_params::UpdateFeatureParams;
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_geo_params::UpdateGeoParams;
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_ingest_params::UpdateIngestParams;
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_match_params::UpdateMatchParams;
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_mesh_params::UpdateMeshParams;
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_motion_params::UpdateMotionParams;
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::update_rig_extrinsic::UpdateRigExtrinsic;
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::remove_content::RemoveContent;
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_sfm_params::UpdateSfmParams;
//#endregion 🔖️LeafPayloads


//#region 🔖️ApplyInverse
#[path = "🚪️io/🦀️.rs"]
mod apply_boundary;
pub use apply_boundary::{apply_remodeling_mutation, bridge_step};

/// ↩️ Computes the inverse mutations from pre-state — kept as a free-function wrapper (matching
/// `🎬️sequence`'s `inverse_sequence_mutation`).
pub fn inverse_remodeling_mutation(base: &RemodelingSnapshot, mutation: &RemodelingMutation) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(base)?

    })
}
//#endregion 🔖️ApplyInverse

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge


//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `RemodelingMutation` variant, in declaration order — the vocabulary
/// the `remodeling-1-any` catalog (`../../🔣️oracle.json`) declares and the
/// `📸️mutate-remodeling-1` exhaustive case measures itself against. The order groups the three families:
/// id-keyed create/delete/change over the five referential pools, then the eight `update-*-params`
/// whole-record replacements, then the engine-owned `replace-*` results, the durable content leaves, and
/// finally the atomic `commit-reconstruction` result. `kinds_match_the_enum_and_the_catalog` below is what keeps this
/// list honest against the enum, since the framework never parses Rust.
pub const KINDS: &[&str] = &[
    "create-stream",
    "delete-stream",
    "change-stream-sync",
    "add-stream-frame",
    "remove-stream-frame",
    "replace-stream-source",
    "create-asset",
    "delete-asset",
    "create-camera-calibration",
    "update-camera-calibration",
    "delete-camera-calibration",
    "create-rig-extrinsic",
    "delete-rig-extrinsic",
    "update-rig-extrinsic",
    "create-gcp",
    "delete-gcp",
    "add-gcp-observation",
    "remove-gcp-observation",
    "update-ingest-params",
    "update-feature-params",
    "update-match-params",
    "update-sfm-params",
    "update-dense-params",
    "update-mesh-params",
    "update-motion-params",
    "update-geo-params",
    "replace-sparse",
    "replace-dense",
    "replace-mesh-result",
    "replace-trajectory",
    "replace-tracks",
    "replace-geo-products",
    "replace-qc",
    "append-content",
    "remove-content",
    "commit-reconstruction",
];
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;
//#endregion 🧪️KindsCatalog

//#region 🔖️Leaves
/// 🧾️ The mutation facet's five format leaves.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🔖️Leaves
