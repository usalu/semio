//! 🧬️ Remodel artifact — document mutation dispatch enum. Every variant is a single-field tuple
//! wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/` triad
//! leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<RemodelSnapshot>` and
//! `impl protocol::SemanticMutation<RemodelSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;
use protocol::Mutation as _;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
/// 🧮️ Semantic remodel document mutation vocabulary: id-keyed create/delete/change/add/remove per
/// collection (streams, assets, camera calibrations, rig extrinsics, GCPs), `update` for the 8
/// inseparable `ReconstructionParams` sub-facets and the calibration/rig full-record replace, and
/// `replace` for the engine-owned job/results large structured sub-payloads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RemodelSnapshot, diff = RemodelDiff, schema = "remodel.scene")]
pub enum RemodelMutation {
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
    /// 📦️ Boxed: `RemodelMesh` (a full `MeshData` plus an optional watertight snapshot) is far larger
    /// than any sibling variant, and `clippy::large_enum_variant` flags the resulting size disparity
    /// across `RemodelMutation`/`RemodelDiff` — boxing keeps every other variant cheap to move.
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

//#region 🔖️ApplyInverse
/// ▶️ Applies `mutation` via its diff — kept as a free-function wrapper (matching
/// `🎬️sequence`'s `apply_sequence_mutation`) since external callers (the editor surface) still call it
/// by this name.
pub fn apply_remodel_mutation(snapshot: &RemodelSnapshot, mutation: &RemodelMutation) -> protocol::MutationApplyResult<RemodelSnapshot> {
    protocol::MutationDiff::apply(&mutation.diff(snapshot).into_parts().0, snapshot)
}

/// ↩️ Computes the inverse mutations from pre-state — kept as a free-function wrapper (matching
/// `🎬️sequence`'s `inverse_sequence_mutation`).
pub fn inverse_remodel_mutation(base: &RemodelSnapshot, mutation: &RemodelMutation) -> Vec<RemodelMutation> {
    mutation.inverse(base)
}
//#endregion 🔖️ApplyInverse

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodel::{
        default_remodel_scene, CameraCalibration, CameraPosePreview, CameraTrajectory, DenseCloud, FrameRef, GcpObservation, GroundControlPoint, ImageAsset, MediaKind, MediaStream, MeshSource, PackedF32, PackedU8, QcReportSnapshot,
        ReconstructionJob, ReconstructionStage, RemodelMesh, RigExtrinsic, SparseCloud, TrackClass, VideoCodec, VideoSource, WatertightReportSnapshot,
    };
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};

    //#region 🔖️Fixture
    /// 🏗️ Shared fixture — a scene that exercises every optional/collection field at least once
    /// (verbatim duplicate of the `rs`/`📝️text` crates' own private test-only builder — see that
    /// crate's `populated_scene_fixture` doc comment).
    async fn populated_scene_fixture() -> RemodelSnapshot {
        let mut scene = default_remodel_scene();
        scene.streams.push(MediaStream {
            id: "stream-1".into(),
            name: "front".into(),
            kind: MediaKind::Video,
            camera_id: Some("cam-1".into()),
            sync_offset_ms: 12.5,
            fps_hint: 30.0,
            frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: "asset-1".into() }],
            source: Some(VideoSource { name: "front.mp4".into(), container: "mp4".into(), codec: VideoCodec::Avc, duration_ms: 6633.3, frame_count: 199, width: 1920, height: 1080 }),
        });
        let asset_one = ImageAsset { mime: "image/jpeg".into(), data: "abcd".into(), width: 4, height: 4 };
        scene.assets.insert("asset-1".into(), crate::artifacts::remodel::store_remodel_asset("asset-1", &asset_one));
        scene.calibration.cameras.push(CameraCalibration {
            id: "cam-1".into(),
            label: "Front".into(),
            model: "brownConrady".into(),
            fx: 1000.0,
            fy: 1000.0,
            cx: 512.0,
            cy: 384.0,
            skew: 0.0,
            distortion: [0.01, -0.02, 0.0, 0.0, 0.0],
            rms_reprojection_px: Some(0.4),
            locked: false,
        });
        scene.calibration.rig.push(RigExtrinsic::default());
        scene.gcps.push(GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [1.0, 2.0, 3.0], observations: vec![GcpObservation { stream_id: "stream-1".into(), frame_index: 0, pixel: [10.0, 20.0] }] });
        scene.params.ingest.min_sharpness = 0.4;
        scene.params.mesh.texture_size = 4096;
        scene.job.stage = ReconstructionStage::BundleAdjusting;
        scene.job.progress_0_1 = 0.42;
        scene.job.started_at_ms = Some(1000.0);
        scene.job.error = Some("retry needed".into());
        scene.job.camera_poses_preview.push(CameraPosePreview { camera_id: "cam-1".into(), ..CameraPosePreview::default() });
        scene.job.sparse_point_cloud_preview = PackedF32::from_f32_slice(&[0.1, 0.2, 0.3]);
        scene.results.sparse = Some(SparseCloud { points: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]), colors: Some(PackedU8::from_u8_slice(&[255, 0, 0, 0, 255, 0])) });
        scene.results.dense =
            Some(DenseCloud { positions: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0]), colors: Some(PackedU8::from_u8_slice(&[0, 0, 255])), confidence: Some(PackedF32::from_f32_slice(&[0.9])), classification: Some(PackedU8::from_u8_slice(&[2])) });
        scene.results.mesh = RemodelMesh {
            mesh: crate::artifacts::remodel::mint_and_stash_mesh(semio_framework::mesh_from_kind("box")),
            source: MeshSource::Reconstructed,
            texture_asset_id: Some("tex-1".into()),
            watertight: Some(WatertightReportSnapshot {
                vertex_count: 512,
                triangle_count: 1020,
                boundary_edge_count: 0,
                boundary_loop_count: 0,
                non_manifold_edge_count: 0,
                non_manifold_vertex_count: 0,
                connected_components: 1,
                consistently_oriented: true,
                euler_characteristic: 2,
                genus: Some(0),
                signed_volume: 12.5,
                self_intersection_pairs: Some(0),
                closed_fallback_used: false,
                is_closed: true,
                is_two_manifold: true,
                is_watertight: true,
            }),
        };
        scene.results.trajectory = Some(CameraTrajectory {
            poses: vec![
                CameraPosePreview { camera_id: "cam-1".into(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [0.0, 0.0, 0.0] },
                CameraPosePreview { camera_id: "cam-1".into(), rotation_wxyz: [0.999, 0.001, 0.0, 0.0], translation: [0.1, 0.0, 0.0] },
            ],
        });
        scene.results.tracks.push(crate::artifacts::remodel::MotionTrackSummary { id: "track-1".into(), length: 42, class: TrackClass::Moving, mean_speed_m_s: 1.2 });
        scene.results.geo = Some(crate::artifacts::remodel::GeoProducts { dsm_asset_id: Some("asset-dsm".into()), dtm_asset_id: Some("asset-dtm".into()), ortho_asset_id: Some("asset-ortho".into()) });
        scene.results.qc = Some(QcReportSnapshot {
            reprojection_rms_px: 0.5,
            gcp_checkpoint_rmse: Some(0.02),
            watertight: scene.results.mesh.watertight.clone(),
            mean_track_length: 6.0,
            registered_frame_ratio: 1.0,
            dense_coverage_ratio: 0.95,
            warnings: vec!["low overlap on frame 12".into()],
        });
        scene
    }
    //#endregion 🔖️Fixture

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn create_delete_stream_inverse_law() {
        let base = populated_scene_fixture();
        let stream = MediaStream { id: "stream-99".into(), name: "extra".into(), ..MediaStream::default() };
        assert_mutation_inverse_law(&base, &create_stream(stream));
        assert_mutation_inverse_law(&base, &delete_stream("stream-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_stream_sync_inverse_law() {
        let base = populated_scene_fixture();
        assert_mutation_inverse_law(&base, &change_stream_sync("stream-1".into(), 99.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_remove_stream_frame_inverse_law() {
        let base = populated_scene_fixture();
        assert_mutation_inverse_law(&base, &add_stream_frame("stream-1".into(), FrameRef { index: 1, timestamp_ms: 33.0, asset_id: "asset-2".into() }, MediaKind::Video));
        // 🎯️ `remove-stream-frame`'s inverse only round-trips exactly for the LAST frame (see its
        // payload's doc comment) — target index 0, the only frame `populated_scene_fixture` seeds.
        assert_mutation_inverse_law(&base, &remove_stream_frame("stream-1".into(), 0));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_stream_source_inverse_law() {
        let base = populated_scene_fixture();
        assert_mutation_inverse_law(&base, &replace_stream_source("stream-1".into(), None));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_delete_asset_inverse_law() {
        let base = populated_scene_fixture();
        let asset = ImageAsset { mime: "image/png".into(), data: "zzzz".into(), width: 2, height: 2 };
        assert_mutation_inverse_law(&base, &create_asset("asset-1".into(), asset.clone()));
        assert_mutation_inverse_law(&base, &create_asset("asset-2".into(), asset));
        assert_mutation_inverse_law(&base, &delete_asset("asset-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn camera_calibration_inverse_law() {
        let base = populated_scene_fixture();
        let camera = CameraCalibration { id: "cam-99".into(), model: "pinhole".into(), ..CameraCalibration::default() };
        assert_mutation_inverse_law(&base, &create_camera_calibration(camera));
        let updated = CameraCalibration { id: "cam-1".into(), fx: 2000.0, ..base.calibration.cameras[0].clone() };
        assert_mutation_inverse_law(&base, &update_camera_calibration(updated));
        assert_mutation_inverse_law(&base, &delete_camera_calibration("cam-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn rig_extrinsic_inverse_law() {
        let base = populated_scene_fixture();
        let extrinsic = RigExtrinsic { camera_id: "cam-99".into(), ..RigExtrinsic::default() };
        assert_mutation_inverse_law(&base, &create_rig_extrinsic(extrinsic));
        let updated = RigExtrinsic { translation_m: [1.0, 0.0, 0.0], ..base.calibration.rig[0].clone() };
        assert_mutation_inverse_law(&base, &update_rig_extrinsic(updated));
        assert_mutation_inverse_law(&base, &delete_rig_extrinsic(base.calibration.rig[0].camera_id.clone()));
    }

    #[semio_framework_async_macros::async_test]
    async fn gcp_inverse_law() {
        let base = populated_scene_fixture();
        let gcp = GroundControlPoint { id: "gcp-99".into(), name: "New".into(), ..GroundControlPoint::default() };
        assert_mutation_inverse_law(&base, &create_gcp(gcp));
        assert_mutation_inverse_law(&base, &delete_gcp("gcp-1".into()));
        assert_mutation_inverse_law(&base, &add_gcp_observation("gcp-1".into(), GcpObservation { stream_id: "stream-1".into(), frame_index: 1, pixel: [1.0, 2.0] }));
        // 🎯️ Same last-index constraint as `remove-stream-frame` — target the only seeded observation.
        assert_mutation_inverse_law(&base, &remove_gcp_observation("gcp-1".into(), 0));
    }

    #[semio_framework_async_macros::async_test]
    async fn update_params_inverse_law() {
        let base = populated_scene_fixture();
        assert_mutation_inverse_law(&base, &update_ingest_params(crate::artifacts::remodel::IngestParams { min_sharpness: 0.9, ..base.params.ingest.clone() }));
        assert_mutation_inverse_law(&base, &update_feature_params(crate::artifacts::remodel::FeatureParams { target_count: 1, ..base.params.feature.clone() }));
        assert_mutation_inverse_law(&base, &update_match_params(crate::artifacts::remodel::MatchParams { ratio_test: 0.1, ..base.params.matching.clone() }));
        assert_mutation_inverse_law(&base, &update_sfm_params(crate::artifacts::remodel::SfmParams { ransac_iterations: 1, ..base.params.sfm.clone() }));
        assert_mutation_inverse_law(&base, &update_dense_params(crate::artifacts::remodel::DenseParams { max_points: 1, ..base.params.dense.clone() }));
        assert_mutation_inverse_law(&base, &update_mesh_params(crate::artifacts::remodel::MeshParams { texture_size: 1, ..base.params.mesh.clone() }));
        assert_mutation_inverse_law(&base, &update_motion_params(crate::artifacts::remodel::MotionParams { enabled: true, ..base.params.motion.clone() }));
        assert_mutation_inverse_law(&base, &update_geo_params(crate::artifacts::remodel::GeoParams { enabled: true, ..base.params.geo.clone() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_job_and_results_inverse_law() {
        let base = populated_scene_fixture();
        assert_mutation_inverse_law(&base, &replace_job(ReconstructionJob { stage: ReconstructionStage::Failed, ..base.job.clone() }));
        assert_mutation_inverse_law(&base, &replace_sparse(None));
        assert_mutation_inverse_law(&base, &replace_dense(None));
        assert_mutation_inverse_law(&base, &replace_mesh_result(Box::new(RemodelMesh::default())));
        assert_mutation_inverse_law(&base, &replace_trajectory(None));
        assert_mutation_inverse_law(&base, &replace_tracks(Vec::new()));
        assert_mutation_inverse_law(&base, &replace_geo_products(None));
        assert_mutation_inverse_law(&base, &replace_qc(None));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_step_style_diff_absorb_law() {
        let base = populated_scene_fixture();
        let d1 = change_stream_sync("stream-1".into(), 10.0).diff(&base);
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = change_stream_sync("stream-1".into(), 20.0).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_remodel_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        for kind in RemodelMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(RemodelMutation::kinds().len(), 34);
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️Convergence
    /// 🔀️ The CRDT convergence contract: two collaborators concurrently importing different assets
    /// (`create-asset` on disjoint keys) must converge to an identical scene regardless of application
    /// order — the reason `create-asset` clones+inserts into `base.assets` rather than any whole-map
    /// replace that could drop a concurrent key.
    #[semio_framework_async_macros::async_test]
    async fn concurrent_create_asset_ops_converge_regardless_of_order() {
        let base = populated_scene_fixture();
        let asset_a = ImageAsset { mime: "image/jpeg".into(), data: "frame-one".into(), width: 8, height: 8 };
        let asset_b = ImageAsset { mime: "image/jpeg".into(), data: "frame-two".into(), width: 8, height: 8 };
        let op_a = create_asset("frame-a".into(), asset_a.clone());
        let op_b = create_asset("frame-b".into(), asset_b.clone());

        let a = apply_remodel_mutation(&base, &op_a).expect("valid mutation diff");
        let b = apply_remodel_mutation(&base, &op_b).expect("valid mutation diff");
        let a_then_b = apply_remodel_mutation(&a, &op_b).expect("valid mutation diff");
        let b_then_a = apply_remodel_mutation(&b, &op_a).expect("valid mutation diff");

        assert_eq!(a_then_b, b_then_a, "concurrent create-asset on disjoint keys must converge regardless of order");
        // 🎯️ Both assets are `image/jpeg` (unsupported by the real png bridge today, see
        // `semio_image_snapshot_from_image_asset`'s doc comment), so `store_remodel_asset` falls back
        // to the deterministic raw-bytes handle (`image_asset_child_handle`) — asserting on the HANDLE
        // (content-addressed, so identical for identical `(mime,data)` regardless of who mints it) is
        // the honest convergence check here, not a round-trip through the working-scene cache.
        assert_eq!(a_then_b.assets.get("frame-a"), Some(&crate::artifacts::remodel::image_asset_child_handle("frame-a", &asset_a)));
        assert_eq!(a_then_b.assets.get("frame-b"), Some(&crate::artifacts::remodel::image_asset_child_handle("frame-b", &asset_b)));
    }

    /// 🔀️ Same convergence contract across two disjoint operation families (feature params tuning vs.
    /// adding a GCP) — proves field-granular application converges across the whole vocabulary.
    #[semio_framework_async_macros::async_test]
    async fn concurrent_edits_across_different_op_families_converge() {
        let base = populated_scene_fixture();
        let op_feature = update_feature_params(crate::artifacts::remodel::FeatureParams { target_count: 9000, ..base.params.feature.clone() });
        let gcp = GroundControlPoint { id: "gcp-99".into(), name: "New".into(), ..GroundControlPoint::default() };
        let op_gcp = create_gcp(gcp.clone());

        let feature = apply_remodel_mutation(&base, &op_feature).expect("valid mutation diff");
        let gcp = apply_remodel_mutation(&base, &op_gcp).expect("valid mutation diff");
        let feature_then_gcp = apply_remodel_mutation(&feature, &op_gcp).expect("valid mutation diff");
        let gcp_then_feature = apply_remodel_mutation(&gcp, &op_feature).expect("valid mutation diff");

        assert_eq!(feature_then_gcp, gcp_then_feature);
        assert_eq!(feature_then_gcp.params.feature.target_count, 9000);
        assert!(feature_then_gcp.gcps.iter().any(|entry| entry.id == "gcp-99"));
    }
    //#endregion 🔖️Convergence

    //#region 🔖️OpText
    /// ⚡️ One `assert_op_line_round_trip` per `RemodelMutation` variant, per the mechanism contract.
    #[semio_framework_async_macros::async_test]
    async fn every_mutation_variant_roundtrips_through_op_text() {
        let scene = populated_scene_fixture();

        store::os_store::test_support::assert_op_line_round_trip(&create_stream(scene.streams[0].clone()));
        store::os_store::test_support::assert_op_line_round_trip(&delete_stream("stream-1".into()));
        store::os_store::test_support::assert_op_line_round_trip(&change_stream_sync("stream-1".into(), 42.0));
        store::os_store::test_support::assert_op_line_round_trip(&add_stream_frame("stream-1".into(), FrameRef { index: 1, timestamp_ms: 33.0, asset_id: "asset-2".into() }, MediaKind::Video));
        store::os_store::test_support::assert_op_line_round_trip(&remove_stream_frame("stream-1".into(), 0));
        store::os_store::test_support::assert_op_line_round_trip(&replace_stream_source("stream-1".into(), scene.streams[0].source.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&replace_stream_source("stream-1".into(), None));
        store::os_store::test_support::assert_op_line_round_trip(&create_asset("asset-1".into(), ImageAsset { mime: "image/jpeg".into(), data: "abcd".into(), width: 4, height: 4 }));
        store::os_store::test_support::assert_op_line_round_trip(&delete_asset("asset-2".into()));
        store::os_store::test_support::assert_op_line_round_trip(&create_camera_calibration(scene.calibration.cameras[0].clone()));
        store::os_store::test_support::assert_op_line_round_trip(&update_camera_calibration(scene.calibration.cameras[0].clone()));
        store::os_store::test_support::assert_op_line_round_trip(&delete_camera_calibration("cam-1".into()));
        store::os_store::test_support::assert_op_line_round_trip(&create_rig_extrinsic(scene.calibration.rig[0].clone()));
        store::os_store::test_support::assert_op_line_round_trip(&update_rig_extrinsic(scene.calibration.rig[0].clone()));
        store::os_store::test_support::assert_op_line_round_trip(&delete_rig_extrinsic(scene.calibration.rig[0].camera_id.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&create_gcp(scene.gcps[0].clone()));
        store::os_store::test_support::assert_op_line_round_trip(&delete_gcp("gcp-1".into()));
        store::os_store::test_support::assert_op_line_round_trip(&add_gcp_observation("gcp-1".into(), scene.gcps[0].observations[0].clone()));
        store::os_store::test_support::assert_op_line_round_trip(&remove_gcp_observation("gcp-1".into(), 0));
        store::os_store::test_support::assert_op_line_round_trip(&update_ingest_params(scene.params.ingest.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&update_feature_params(scene.params.feature.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&update_match_params(scene.params.matching.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&update_sfm_params(scene.params.sfm.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&update_dense_params(scene.params.dense.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&update_mesh_params(scene.params.mesh.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&update_motion_params(scene.params.motion.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&update_geo_params(scene.params.geo.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&replace_job(scene.job.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&replace_sparse(scene.results.sparse.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&replace_sparse(None));
        store::os_store::test_support::assert_op_line_round_trip(&replace_dense(scene.results.dense.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&replace_dense(None));
        store::os_store::test_support::assert_op_line_round_trip(&replace_mesh_result(Box::new(scene.results.mesh.clone())));
        store::os_store::test_support::assert_op_line_round_trip(&replace_trajectory(scene.results.trajectory.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&replace_trajectory(None));
        store::os_store::test_support::assert_op_line_round_trip(&replace_tracks(scene.results.tracks.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&replace_geo_products(scene.results.geo.clone()));
        store::os_store::test_support::assert_op_line_round_trip(&replace_geo_products(None));
        store::os_store::test_support::assert_op_line_round_trip(&replace_qc(scene.results.qc));
        store::os_store::test_support::assert_op_line_round_trip(&replace_qc(None));
    }
    //#endregion 🔖️OpText
}
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge
/// 🧩️ Decodes one committed `📸️snapshot/⬅️before/🔣️.json` document together with the
/// `🦠️mutation/🔣️.json` payload beside it — the same bytes the leaf's own fixture test
/// reads — into real typed values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(RemodelSnapshot, RemodelMutation), String> {
    let snapshot: RemodelSnapshot = serde_json::from_str(snapshot_json).map_err(|error| format!("the committed remodel snapshot JSON does not decode: {error}"))?;
    let mutation: RemodelMutation = serde_json::from_str(mutation_json).map_err(|error| format!("the committed remodel mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}

/// ▶️ One diff-and-apply step, keeping the diagnostic codes the outcome raised — a rejected or
/// no-op kind is a RESULT this bridge reports, never an error it swallows.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_step(snapshot: &RemodelSnapshot, mutation: &RemodelMutation) -> Result<(RemodelSnapshot, Vec<String>), String> {
    use protocol::{Mutation, MutationDiff};
    let outcome = <RemodelMutation as Mutation<RemodelSnapshot>>::diff(mutation, snapshot);
    let messages: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => Ok((next, messages)),
        Err(error) => Err(format!("{error:?}")),
    }
}

/// 📤️ The bridge's answer shape: the resulting document beside the codes it raised, so a caller
/// that cannot name `protocol::MutationOutcome` can still tell an application from a refusal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_render(snapshot: &RemodelSnapshot, messages: Vec<String>) -> Result<String, String> {
    serde_json::to_string(&serde_json::json!({ "snapshot": snapshot, "messages": messages })).map_err(|error| error.to_string())
}

/// 🌉️ Applies one committed mutation payload to one committed before-document and answers
/// `{"snapshot": …, "messages": [ … ]}`.
///
/// The bridge exists because the generated Rust test host links only `semio-repo-test-host` and,
/// behind its `sut` feature, this crate — `serde_json`, `protocol` and `store` are private
/// extern-crate aliases (`🦀️.rs`) and cannot be named from a case adapter. Same shape and same
/// reason as `🗄️stdio`'s `decode_semio_mesh_mutation_json`/`apply_semio_mesh_mutation` pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_remodel_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    let (snapshot, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (applied, messages) = bridge_step(&snapshot, &mutation)?;
    bridge_render(&applied, messages)
}

/// ↩️ Applies one committed mutation payload and then EVERY step of its own computed inverse,
/// answering in the same shape — the metamorphic half of the evidence the `remodel-mutation-semantics` no-oracle
/// decision rests on. The inverse is computed against the PRE-mutation document, which is the only
/// state that carries what a delete removed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn undo_remodel_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    use protocol::Mutation;
    let (base, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (mut current, mut messages) = bridge_step(&base, &mutation)?;
    for undo in <RemodelMutation as Mutation<RemodelSnapshot>>::inverse(&mutation, &base) {
        let (next, raised) = bridge_step(&current, &undo)?;
        current = next;
        messages.extend(raised);
    }
    bridge_render(&current, messages)
}

/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_remodel_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <RemodelSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed remodel example does not parse: {error:?}"))?;
    let printed = <RemodelSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <RemodelSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted remodel document does not parse: {error:?}"))?;
    serde_json::to_string(&serde_json::json!({ "printed": printed, "snapshot": parsed, "reparsed": reparsed })).map_err(|error| error.to_string())
}
//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `RemodelMutation` variant, in declaration order — the vocabulary
/// the `remodel-1-any` catalog (`../../🔣️oracle.json`) declares and the
/// `mutate-remodel-1` exhaustive case measures itself against. The order groups the three families:
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
mod kinds_catalog_tests {
    use super::*;
    use protocol::SemanticMutation;

    /// 🏷️ `KINDS` must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed
    /// catalog — the framework reads the manifest and never parses Rust, so this test is the only
    /// thing that keeps the two in step. A plain `#[test]`: it suspends on nothing.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = RemodelMutation::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared RemodelMutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed remodel-1-any catalog");
        }
    }
}
//#endregion 🧪️KindsCatalog
