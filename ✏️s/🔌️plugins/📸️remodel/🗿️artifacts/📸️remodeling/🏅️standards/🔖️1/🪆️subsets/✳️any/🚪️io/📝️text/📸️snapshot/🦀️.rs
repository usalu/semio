//! 📜️ Remodeling artifact — the textual document grammar surface (`dsl`) and its laws.
//!
//! 🔀️ Deviation from every sibling plugin: remodeling ships no handcrafted `.remodeling` example fixture
//! file, so there is no `REMODELING_EXAMPLE_TEXT` constant here — its single "default" example is
//! generated at runtime from `default_remodeling_scene().print_dsl()` (see `create_remodeling_app`).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::RemodelingSnapshot;

/// 📖️ Parses `.remodeling` DSL text into a `RemodelingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<RemodelingSnapshot, semio_framework_diagnostic::TextError> {
    <RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RemodelingSnapshot` back to `.remodeling` DSL text.
pub fn print_dsl(scene: &RemodelingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(scene)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RemodelingSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionParams, ReconstructionResults, RemodelingAssetChild, RemodelingDurableArtifactStore, REMODELING_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for RemodelingSnapshot {
    const EXTENSION: &'static str = "remodeling";
    fn envelope_id() -> &'static str {
        "remodeling.remodeling"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::RemodelingDiff;
use crate::RemodelingSnapshot;
use protocol::Mutation as _;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::standards::v1::subsets::any::schema::mutations::append_content::{append_content, AppendContent};
use crate::standards::v1::subsets::any::schema::mutations::add_gcp_observation::{add_gcp_observation, AddGcpObservation};
use crate::standards::v1::subsets::any::schema::mutations::add_stream_frame::{add_stream_frame, AddStreamFrame};
use crate::standards::v1::subsets::any::schema::mutations::change_stream_sync::{change_stream_sync, ChangeStreamSync};
use crate::standards::v1::subsets::any::schema::mutations::commit_reconstruction::{commit_reconstruction, CommitReconstruction, ReconstructionAssetCommit};
use crate::standards::v1::subsets::any::schema::mutations::create_asset::{create_asset, CreateAsset};
use crate::standards::v1::subsets::any::schema::mutations::create_camera_calibration::{create_camera_calibration, CreateCameraCalibration};
use crate::standards::v1::subsets::any::schema::mutations::create_gcp::{create_gcp, CreateGcp};
use crate::standards::v1::subsets::any::schema::mutations::create_rig_extrinsic::{create_rig_extrinsic, CreateRigExtrinsic};
use crate::standards::v1::subsets::any::schema::mutations::create_stream::{create_stream, CreateStream};
use crate::standards::v1::subsets::any::schema::mutations::delete_asset::{delete_asset, DeleteAsset};
use crate::standards::v1::subsets::any::schema::mutations::delete_camera_calibration::{delete_camera_calibration, DeleteCameraCalibration};
use crate::standards::v1::subsets::any::schema::mutations::delete_gcp::{delete_gcp, DeleteGcp};
use crate::standards::v1::subsets::any::schema::mutations::delete_rig_extrinsic::{delete_rig_extrinsic, DeleteRigExtrinsic};
use crate::standards::v1::subsets::any::schema::mutations::delete_stream::{delete_stream, DeleteStream};
use crate::standards::v1::subsets::any::schema::mutations::remove_gcp_observation::{remove_gcp_observation, RemoveGcpObservation};
use crate::standards::v1::subsets::any::schema::mutations::remove_stream_frame::{remove_stream_frame, RemoveStreamFrame};
use crate::standards::v1::subsets::any::schema::mutations::replace_dense::{replace_dense, ReplaceDense};
use crate::standards::v1::subsets::any::schema::mutations::replace_geo_products::{replace_geo_products, ReplaceGeoProducts};
use crate::standards::v1::subsets::any::schema::mutations::replace_mesh_result::{replace_mesh_result, ReplaceMeshResult};
use crate::standards::v1::subsets::any::schema::mutations::replace_qc::{replace_qc, ReplaceQc};
use crate::standards::v1::subsets::any::schema::mutations::replace_sparse::{replace_sparse, ReplaceSparse};
use crate::standards::v1::subsets::any::schema::mutations::replace_stream_source::{replace_stream_source, ReplaceStreamSource};
use crate::standards::v1::subsets::any::schema::mutations::replace_tracks::{replace_tracks, ReplaceTracks};
use crate::standards::v1::subsets::any::schema::mutations::replace_trajectory::{replace_trajectory, ReplaceTrajectory};
use crate::standards::v1::subsets::any::schema::mutations::update_camera_calibration::{update_camera_calibration, UpdateCameraCalibration};
use crate::standards::v1::subsets::any::schema::mutations::update_dense_params::{update_dense_params, UpdateDenseParams};
use crate::standards::v1::subsets::any::schema::mutations::update_feature_params::{update_feature_params, UpdateFeatureParams};
use crate::standards::v1::subsets::any::schema::mutations::update_geo_params::{update_geo_params, UpdateGeoParams};
use crate::standards::v1::subsets::any::schema::mutations::update_ingest_params::{update_ingest_params, UpdateIngestParams};
use crate::standards::v1::subsets::any::schema::mutations::update_match_params::{update_match_params, UpdateMatchParams};
use crate::standards::v1::subsets::any::schema::mutations::update_mesh_params::{update_mesh_params, UpdateMeshParams};
use crate::standards::v1::subsets::any::schema::mutations::update_motion_params::{update_motion_params, UpdateMotionParams};
use crate::standards::v1::subsets::any::schema::mutations::update_rig_extrinsic::{update_rig_extrinsic, UpdateRigExtrinsic};
use crate::standards::v1::subsets::any::schema::mutations::remove_content::{remove_content, RemoveContent};
use crate::standards::v1::subsets::any::schema::mutations::update_sfm_params::{update_sfm_params, UpdateSfmParams};

/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_remodeling_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <RemodelingSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed remodeling example does not parse: {error:?}"))?;
    let printed = <RemodelingSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <RemodelingSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted remodeling document does not parse: {error:?}"))?;
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("printed".to_string(), semio_framework_pack_json::Value::String(printed)),
        ("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&parsed))),
        ("reparsed".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&reparsed))),
    ])))
}
}
pub use mutations_codec::*;
