//! ⚡️ Remodeling artifact — OpText/OpBinary codecs + grammar for serializing `RemodelingMutation`.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

use crate::schema::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for RemodelingMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for RemodelingMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

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

/// 🧩️ Decodes one committed `📸️snapshot/⬅️before/🔣️.json` document together with the
/// `🦠️mutation/🔣️.json` payload beside it — the same bytes the leaf's own fixture test
/// reads — into real typed values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(RemodelingSnapshot, RemodelingMutation), String> {
    let snapshot: RemodelingSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed remodeling snapshot JSON does not decode: {error}"))?;
    let mutation: RemodelingMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed remodeling mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod mutations_wire_codec {
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

/// 📤️ The bridge's answer shape: the resulting document beside the codes it raised, so a caller
/// that cannot name `protocol::MutationOutcome` can still tell an application from a refusal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn bridge_render(snapshot: &RemodelingSnapshot, messages: Vec<String>) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(snapshot))), ("messages".to_string(), semio_framework_pack_json::array(messages.into_iter().map(semio_framework_pack_json::Value::String)))]))
}
}
pub use mutations_wire_codec::*;
