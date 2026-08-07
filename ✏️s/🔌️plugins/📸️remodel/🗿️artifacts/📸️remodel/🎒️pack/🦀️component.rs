//! 📦️ Remodel artifact — the binary document surface (`pack`) and its laws.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::remodel::RemodelProjection;
use store::PackError;

/// 📦️ Encodes a `RemodelProjection` to its binary pack form.
pub fn encode(scene: &RemodelProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(scene)
}

/// 📖️ Decodes a `RemodelProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<RemodelProjection, PackError> {
    <RemodelProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodel::{
        default_remodel_scene, CameraCalibration, CameraPosePreview, CameraTrajectory, DenseCloud, FrameRef, GcpObservation, GeoProducts, GroundControlPoint, ImageAsset, MediaKind, MediaStream, MeshSource, MotionTrackSummary, PackedF32, PackedU8,
        QcReportSnapshot, ReconstructionStage, RemodelMesh, RigExtrinsic, SparseCloud, TrackClass, VideoCodec, VideoSource, WatertightReportSnapshot,
    };

    /// 🏗️ Verbatim duplicate of the `rs` crate's own private test-only fixture builder — see that
    /// crate's `populated_scene_fixture` doc comment for why this is copied rather than shared.
    fn populated_scene_fixture() -> RemodelProjection {
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
        scene.assets.insert("asset-1".into(), ImageAsset { mime: "image/jpeg".into(), data: "abcd".into(), width: 4, height: 4 });
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
            mesh: semio_framework_core::mesh_from_kind("box"),
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
        scene.results.tracks.push(MotionTrackSummary { id: "track-1".into(), length: 42, class: TrackClass::Moving, mean_speed_m_s: 1.2 });
        scene.results.geo = Some(GeoProducts { dsm_asset_id: Some("asset-dsm".into()), dtm_asset_id: Some("asset-dtm".into()), ortho_asset_id: Some("asset-ortho".into()) });
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

    #[test]
    fn default_scene_roundtrips_through_dsl_pack_equivalence() {
        store::test_support::assert_dsl_pack_equivalence(&default_remodel_scene());
    }

    #[test]
    fn populated_scene_roundtrips_through_dsl_pack_equivalence() {
        let scene = populated_scene_fixture();
        store::test_support::assert_dsl_pack_equivalence(&scene);
        let bytes = encode(&scene);
        assert_eq!(decode(&bytes).expect("decode"), scene);
    }
}
//#endregion 🧪️Tests
