//! 🧬️ Remodeling artifact — document mutation dispatch enum. Every variant is a single-field tuple
//! wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/` triad
//! leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<RemodelingSnapshot>` and
//! `impl protocol::SemanticMutation<RemodelingSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.
use crate::diff::RemodelingDiff;
use crate::RemodelingSnapshot;
use protocol::Mutation as _;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
/// 🧮️ Semantic remodeling document mutation vocabulary: id-keyed create/delete/change/add/remove per
/// collection (streams, assets, camera calibrations, rig extrinsics, GCPs), `update` for the 8
/// inseparable `ReconstructionParams` sub-facets and the calibration/rig full-record replace, and
/// `replace` for the engine-owned job/results large structured sub-payloads.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RemodelingSnapshot, diff = RemodelingDiff, schema = "remodeling.scene")]
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
    ReplaceJob(ReplaceJob),
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
    CommitReconstruction(CommitReconstruction),
}
//#endregion 🔖️Mutations

//#region 🔖️Reexports
pub use super::add_gcp_observation::{add_gcp_observation, AddGcpObservation};
pub use super::add_stream_frame::{add_stream_frame, AddStreamFrame};
pub use super::change_stream_sync::{change_stream_sync, ChangeStreamSync};
pub use super::commit_reconstruction::{commit_reconstruction, CommitReconstruction, ReconstructionAssetCommit};
pub use super::create_asset::{create_asset, CreateAsset};
pub use super::create_camera_calibration::{create_camera_calibration, CreateCameraCalibration};
pub use super::create_gcp::{create_gcp, CreateGcp};
pub use super::create_rig_extrinsic::{create_rig_extrinsic, CreateRigExtrinsic};
pub use super::create_stream::{create_stream, CreateStream};
pub use super::delete_asset::{delete_asset, DeleteAsset};
pub use super::delete_camera_calibration::{delete_camera_calibration, DeleteCameraCalibration};
pub use super::delete_gcp::{delete_gcp, DeleteGcp};
pub use super::delete_rig_extrinsic::{delete_rig_extrinsic, DeleteRigExtrinsic};
pub use super::delete_stream::{delete_stream, DeleteStream};
pub use super::remove_gcp_observation::{remove_gcp_observation, RemoveGcpObservation};
pub use super::remove_stream_frame::{remove_stream_frame, RemoveStreamFrame};
pub use super::replace_dense::{replace_dense, ReplaceDense};
pub use super::replace_geo_products::{replace_geo_products, ReplaceGeoProducts};
pub use super::replace_job::{replace_job, ReplaceJob};
pub use super::replace_mesh_result::{replace_mesh_result, ReplaceMeshResult};
pub use super::replace_qc::{replace_qc, ReplaceQc};
pub use super::replace_sparse::{replace_sparse, ReplaceSparse};
pub use super::replace_stream_source::{replace_stream_source, ReplaceStreamSource};
pub use super::replace_tracks::{replace_tracks, ReplaceTracks};
pub use super::replace_trajectory::{replace_trajectory, ReplaceTrajectory};
pub use super::update_camera_calibration::{update_camera_calibration, UpdateCameraCalibration};
pub use super::update_dense_params::{update_dense_params, UpdateDenseParams};
pub use super::update_feature_params::{update_feature_params, UpdateFeatureParams};
pub use super::update_geo_params::{update_geo_params, UpdateGeoParams};
pub use super::update_ingest_params::{update_ingest_params, UpdateIngestParams};
pub use super::update_match_params::{update_match_params, UpdateMatchParams};
pub use super::update_mesh_params::{update_mesh_params, UpdateMeshParams};
pub use super::update_motion_params::{update_motion_params, UpdateMotionParams};
pub use super::update_rig_extrinsic::{update_rig_extrinsic, UpdateRigExtrinsic};
pub use super::update_sfm_params::{update_sfm_params, UpdateSfmParams};
//#endregion 🔖️Reexports

//#region 🔖️CanonicalOrder
/// 🔢️ Insertion point that keeps a keyed collection in ascending key order. Every collection this
/// vocabulary addresses by key — `streams` and `gcps` by `id`, `calibration.cameras` by `id`,
/// `calibration.rig` by `camera_id`, a stream's `frames` by `(index, asset_id)`, a GCP's
/// `observations` by `(stream_id, frame_index)` — is a canonically ordered document invariant, so a
/// `create-*`/`add-*` puts its member exactly where a later `delete-*`/`remove-*` took it from and
/// `inverse(m, before)` applied to `after` restores `before` member positions included.
pub fn ordered_index<T, K: Ord>(items: &[T], key: &K, key_of: impl Fn(&T) -> K) -> usize {
    items.partition_point(|item| key_of(item) < *key)
}
//#endregion 🔖️CanonicalOrder

//#region 🔖️ApplyInverse
/// ▶️ Applies `mutation` via its diff — kept as a free-function wrapper (matching
/// `🎬️sequence`'s `apply_sequence_mutation`) since external callers (the editor surface) still call it
/// by this name.
pub fn apply_remodeling_mutation(snapshot: &RemodelingSnapshot, mutation: &RemodelingMutation) -> protocol::MutationApplyResult<RemodelingSnapshot> {
    protocol::MutationDiff::apply(&mutation.diff(snapshot).into_parts().0, snapshot)
}

/// ↩️ Computes the inverse mutations from pre-state — kept as a free-function wrapper (matching
/// `🎬️sequence`'s `inverse_sequence_mutation`).
pub fn inverse_remodeling_mutation(base: &RemodelingSnapshot, mutation: &RemodelingMutation) -> Vec<RemodelingMutation> {
    mutation.inverse(base)
}
//#endregion 🔖️ApplyInverse

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge
/// 🧩️ Decodes one committed `📸️snapshot/⬅️before/🔣️.json` document together with the
/// `🦠️mutation/🔣️.json` payload beside it — the same bytes the leaf's own fixture test
/// reads — into real typed values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(RemodelingSnapshot, RemodelingMutation), String> {
    let snapshot: RemodelingSnapshot = pack::from_json_str(snapshot_json).map_err(|error| format!("the committed remodeling snapshot JSON does not decode: {error}"))?;
    let mutation: RemodelingMutation = pack::from_json_str(mutation_json).map_err(|error| format!("the committed remodeling mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}

/// ▶️ One diff-and-apply step, keeping the diagnostic codes the outcome raised — a rejected or
/// no-op kind is a RESULT this bridge reports, never an error it swallows.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_step(snapshot: &RemodelingSnapshot, mutation: &RemodelingMutation) -> Result<(RemodelingSnapshot, Vec<String>), String> {
    use protocol::{Mutation, MutationDiff};
    let outcome = <RemodelingMutation as Mutation<RemodelingSnapshot>>::diff(mutation, snapshot);
    let messages: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => Ok((next, messages)),
        Err(error) => Err(format!("{error:?}")),
    }
}

/// 📤️ The bridge's answer shape: the resulting document beside the codes it raised, so a caller
/// that cannot name `protocol::MutationOutcome` can still tell an application from a refusal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_render(snapshot: &RemodelingSnapshot, messages: Vec<String>) -> String {
    pack::json_to_string(&pack::json_object([("snapshot".to_string(), pack::json_from_dsl_value(&dsl::ToValue::to_value(snapshot))), ("messages".to_string(), pack::json_array(messages.into_iter().map(pack::JsonValue::String)))]))
}

/// 🌉️ Applies one committed mutation payload to one committed before-document and answers
/// `{"snapshot": …, "messages": [ … ]}`.
///
/// The bridge exists because the generated Rust test host links only `semio-repo-test-host` and,
/// behind its `sut` feature, this crate — `serde_json`, `protocol` and `store` are private
/// extern-crate aliases (`🦀️.rs`) and cannot be named from a case adapter. Same shape and same
/// reason as `🗄️stdio`'s `decode_semio_mesh_mutation_json`/`apply_semio_mesh_mutation` pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_remodeling_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    let (snapshot, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (applied, messages) = bridge_step(&snapshot, &mutation)?;
    Ok(bridge_render(&applied, messages))
}

/// ↩️ Applies one committed mutation payload and then EVERY step of its own computed inverse,
/// answering in the same shape — the metamorphic half of the evidence the `remodeling-mutation-semantics` no-oracle
/// decision rests on. The inverse is computed against the PRE-mutation document, which is the only
/// state that carries what a delete removed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn undo_remodeling_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    use protocol::Mutation;
    let (base, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (mut current, mut messages) = bridge_step(&base, &mutation)?;
    for undo in <RemodelingMutation as Mutation<RemodelingSnapshot>>::inverse(&mutation, &base) {
        let (next, raised) = bridge_step(&current, &undo)?;
        current = next;
        messages.extend(raised);
    }
    Ok(bridge_render(&current, messages))
}

/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_remodeling_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <RemodelingSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed remodeling example does not parse: {error:?}"))?;
    let printed = <RemodelingSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <RemodelingSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted remodeling document does not parse: {error:?}"))?;
    Ok(pack::json_to_string(&pack::json_object([
        ("printed".to_string(), pack::JsonValue::String(printed)),
        ("snapshot".to_string(), pack::json_from_dsl_value(&dsl::ToValue::to_value(&parsed))),
        ("reparsed".to_string(), pack::json_from_dsl_value(&dsl::ToValue::to_value(&reparsed))),
    ])))
}
//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `RemodelingMutation` variant, in declaration order — the vocabulary
/// the `remodeling-1-any` catalog (`../../🔣️oracle.json`) declares and the
/// `📸️mutate-remodeling-1` exhaustive case measures itself against. The order groups the three families:
/// id-keyed create/delete/change over the five referential pools, then the eight `update-*-params`
/// whole-record replacements, then the engine-owned `replace-*` results, and finally the atomic
/// `commit-reconstruction` terminal. `kinds_match_the_enum_and_the_catalog` below is what keeps this
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
    "replace-job",
    "replace-sparse",
    "replace-dense",
    "replace-mesh-result",
    "replace-trajectory",
    "replace-tracks",
    "replace-geo-products",
    "replace-qc",
    "commit-reconstruction",
];
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;
//#endregion 🧪️KindsCatalog
