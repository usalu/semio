//! ⚖️ Remodel app — binary command protocol surface + laws (constitutional: protocol).
//!
//! 🎯️ Also hosts `RemodelCommand` — the app-engine `AppCommand::Command` binary command envelope
//! (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE, full config recipe conversion).
//! One variant per action `remodel_ui::create_remodel_app` declares; see `remodel_ui`'s
//! `RemodelPlayApp::handle` for the dispatch.

use protocol::OpBinary;
use remodel_op::RemodelOperation;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `RemodelOperation` to its binary command form.
pub fn encode_op(operation: &RemodelOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RemodelOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RemodelOperation, protocol::ProtocolError> {
    RemodelOperation::decode_op(bytes)
}

//#region 🔖️RemodelCommand
/// 🎯️ B1: `RemodelPlayApp::Command` — the SOLE dispatch surface for remodel's own behavior. Field
/// shapes mirror each action's former `args` object exactly. `RunReconstruction`/`RetryStage`/
/// `RunStage` now run the WHOLE staged pipeline synchronously inside one pure `handle()` call (see
/// `remodel_ui`'s doc comment on that dispatch arm) — there is no more `AdvanceReconstruction`
/// self-rescheduling tick or `CancelReconstruction` (a synchronous compute has nothing running in the
/// background to cancel), both deleted.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum RemodelCommand {
    // 🚀️ Staged reconstruction — now fully synchronous (see module doc comment above).
    #[dsl(key = "run-reconstruction")]
    RunReconstruction,
    #[dsl(key = "retry-stage")]
    RetryStage { stage: String },
    #[dsl(key = "run-stage")]
    RunStage { stage: String },

    // 📥️ Ingestion.
    #[dsl(key = "import-frame-payload")]
    ImportFramePayload { payload: String, name: String, index: u32 },
    #[dsl(key = "import-video-frame-payload")]
    ImportVideoFramePayload { payload: String, name: String, index: u32, frame_index: u32, timestamp_ms: f64 },
    #[dsl(key = "import-video-done")]
    ImportVideoDone { name: String, duration_ms: f64, frame_count: u32, width: u32, height: u32, codec: String },
    #[dsl(key = "import-video-bytes-payload")]
    ImportVideoBytesPayload { payload: String, name: String },
    #[dsl(key = "add-stream")]
    AddStream { name: String, kind: String, camera_id: String },
    #[dsl(key = "remove-stream")]
    RemoveStream { stream_id: String },
    #[dsl(key = "stream-sync")]
    SetStreamSync { stream_id: String, sync_offset_ms: f64 },

    // 🎯️ Calibration / GCPs.
    #[dsl(key = "edit-calibration")]
    EditCalibration { camera_id: String, label: String, model: String, fx: f64, fy: f64, cx: f64, cy: f64, skew: f64, k1: f32, k2: f32, k3: f32, p1: f32, p2: f32, locked: bool },
    #[dsl(key = "calibrate-cameras")]
    CalibrateCameras,
    #[dsl(key = "add-gcp")]
    AddGcp { name: String, world_x: f64, world_y: f64, world_z: f64 },
    #[dsl(key = "remove-gcp")]
    RemoveGcp { gcp_id: String },
    #[dsl(key = "place-gcp-observation")]
    PlaceGcpObservation { gcp_id: String, stream_id: String, frame_index: u32, pixel_x: f32, pixel_y: f32 },

    // ⚙️ 8 param-group setters, one per `ReconstructionParams` sub-struct.
    #[dsl(key = "ingest-params")]
    SetIngestParams { frame_sample_stride: u32, max_frames: u32, downscale_long_edge_px: u32, min_sharpness: f32 },
    #[dsl(key = "feature-params")]
    SetFeatureParams { detector: String, target_count: u32, octaves: u32, edge_threshold: f32 },
    #[dsl(key = "match-params")]
    SetMatchParams { matcher: String, ratio_test: f32, cross_check: bool, sequential_window: u32, max_pairs_per_frame: u32, loop_closure: bool },
    #[dsl(key = "sfm-params")]
    SetSfmParams { ransac_iterations: u32, ransac_threshold_px: f32, min_track_length: u32, ba_max_iterations: u32, robust_loss: String, huber_delta_px: f32 },
    #[dsl(key = "dense-params")]
    SetDenseParams { resolution: String, window_radius_px: u32, min_view_consistency: u32, confidence_threshold: f32, max_points: u32 },
    #[dsl(key = "mesh-params")]
    SetMeshParams {
        tsdf_voxel_size_mm: f32,
        tsdf_truncation_mm: f32,
        decimate_target_triangles: u32,
        smoothing_iterations: u32,
        texture_enabled: bool,
        texture_size: u32,
        guarantee_watertight: bool,
        hole_fill_max_boundary_verts: u32,
        self_intersection_check: bool,
    },
    #[dsl(key = "motion-params")]
    SetMotionParams { enabled: bool, max_tracks: u32, track_window_px: u32, min_track_quality: f32, min_track_length_frames: u32 },
    #[dsl(key = "geo-params")]
    SetGeoParams {
        enabled: bool,
        #[serde(default)]
        origin_lon: Option<f64>,
        #[serde(default)]
        origin_lat: Option<f64>,
        #[serde(default)]
        origin_alt: Option<f64>,
        gsd_m: f32,
        dsm_cell_m: f32,
        dtm_filter_radius_m: f32,
        ortho_max_px: u32,
    },

    // 🧹️ Clear/reset.
    #[dsl(key = "reset-placeholder-mesh")]
    ResetPlaceholderMesh,
    #[dsl(key = "clear-sparse")]
    ClearSparse,
    #[dsl(key = "clear-dense")]
    ClearDense,
    #[dsl(key = "clear-mesh-result")]
    ClearMeshResult,
    #[dsl(key = "clear-tracks")]
    ClearTracks,
    #[dsl(key = "clear-geo-products")]
    ClearGeoProducts,
    #[dsl(key = "clear-result")]
    ClearResult,

    // 👁️ Config-only (was ephemeral `RemodelPlayRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "selection")]
    SetSelection { mode: String, ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: remodel_engine::RemodelWorldCamera,
    },
    #[dsl(key = "layer-visibility")]
    SetLayerVisibility { layer: String, visible: bool },
    #[dsl(key = "frame-cursor")]
    SetFrameCursor {
        #[serde(default)]
        stream_id: Option<String>,
        frame_index: u32,
    },
    #[dsl(key = "report-table")]
    SetReportTable { table: String },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },

    // 🐚️ Shell effects — no operations either way.
    #[dsl(key = "import-frames")]
    ImportFrames,
    #[dsl(key = "import-video")]
    ImportVideo,
    #[dsl(key = "export-qc-report")]
    ExportQcReport,
}
//#endregion 🔖️RemodelCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use remodel::default_remodel_scene;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let scene = default_remodel_scene();
        let operation = RemodelOperation::SetFeatureParams { params: scene.params.feature.clone() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// 📄️ Full `print_document_text`/`parse_document_text` round trip through a live `DocumentStore`
    /// with an applied edit, the ground-truth contract for replacing the JSON envelope with text files.
    #[test]
    fn store_roundtrips_through_document_text() {
        let initial = default_remodel_scene();
        let envelope = store::create_document_envelope("test/v1", "test", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        let mut feature_params = store.projection().expect("initial projection").params.feature.clone();
        feature_params.target_count = 12345;
        store.dispatch(store::DocumentCommand::Apply { operations: vec![RemodelOperation::SetFeatureParams { params: feature_params }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region 🔖️RemodelCommandTests
    /// ⚡️ One representative value per `RemodelCommand` row, in declaration (= binary ordinal) order.
    fn every_remodel_command() -> Vec<RemodelCommand> {
        vec![
            RemodelCommand::RunReconstruction,
            RemodelCommand::RetryStage { stage: "extracting-features".into() },
            RemodelCommand::RunStage { stage: "dense-stereo".into() },
            RemodelCommand::ImportFramePayload { payload: "data:image/png;base64,abc".into(), name: "frame.png".into(), index: 0 },
            RemodelCommand::ImportVideoFramePayload { payload: "data:image/jpeg;base64,abc".into(), name: "clip.mp4".into(), index: 1, frame_index: 1, timestamp_ms: 33.3 },
            RemodelCommand::ImportVideoDone { name: "clip.mp4".into(), duration_ms: 400.0, frame_count: 4, width: 24, height: 24, codec: "mjpeg".into() },
            RemodelCommand::ImportVideoBytesPayload { payload: "data:video/mp4;base64,abc".into(), name: "clip.mp4".into() },
            RemodelCommand::AddStream { name: "Stream".into(), kind: "video".into(), camera_id: "cam-0".into() },
            RemodelCommand::RemoveStream { stream_id: "stream-1".into() },
            RemodelCommand::SetStreamSync { stream_id: "stream-1".into(), sync_offset_ms: 12.5 },
            RemodelCommand::EditCalibration {
                camera_id: "cam-1".into(),
                label: "Front".into(),
                model: "pinhole".into(),
                fx: 1000.0,
                fy: 1000.0,
                cx: 0.0,
                cy: 0.0,
                skew: 0.0,
                k1: 0.0,
                k2: 0.0,
                k3: 0.0,
                p1: 0.0,
                p2: 0.0,
                locked: false,
            },
            RemodelCommand::CalibrateCameras,
            RemodelCommand::AddGcp { name: "GCP".into(), world_x: 0.0, world_y: 0.0, world_z: 0.0 },
            RemodelCommand::RemoveGcp { gcp_id: "gcp-1".into() },
            RemodelCommand::PlaceGcpObservation { gcp_id: "gcp-1".into(), stream_id: "stream-1".into(), frame_index: 0, pixel_x: 10.0, pixel_y: 20.0 },
            RemodelCommand::SetIngestParams { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 },
            RemodelCommand::SetFeatureParams { detector: "orb".into(), target_count: 4000, octaves: 4, edge_threshold: 10.0 },
            RemodelCommand::SetMatchParams { matcher: "brute-force".into(), ratio_test: 0.8, cross_check: true, sequential_window: 8, max_pairs_per_frame: 16, loop_closure: true },
            RemodelCommand::SetSfmParams { ransac_iterations: 1000, ransac_threshold_px: 2.0, min_track_length: 3, ba_max_iterations: 50, robust_loss: "huber".into(), huber_delta_px: 1.5 },
            RemodelCommand::SetDenseParams { resolution: "medium".into(), window_radius_px: 3, min_view_consistency: 3, confidence_threshold: 0.5, max_points: 500_000 },
            RemodelCommand::SetMeshParams {
                tsdf_voxel_size_mm: 5.0,
                tsdf_truncation_mm: 20.0,
                decimate_target_triangles: 200_000,
                smoothing_iterations: 2,
                texture_enabled: true,
                texture_size: 2048,
                guarantee_watertight: true,
                hole_fill_max_boundary_verts: 512,
                self_intersection_check: false,
            },
            RemodelCommand::SetMotionParams { enabled: false, max_tracks: 64, track_window_px: 21, min_track_quality: 0.3, min_track_length_frames: 5 },
            RemodelCommand::SetGeoParams { enabled: false, origin_lon: None, origin_lat: Some(1.0), origin_alt: None, gsd_m: 0.05, dsm_cell_m: 0.1, dtm_filter_radius_m: 2.0, ortho_max_px: 4096 },
            RemodelCommand::ResetPlaceholderMesh,
            RemodelCommand::ClearSparse,
            RemodelCommand::ClearDense,
            RemodelCommand::ClearMeshResult,
            RemodelCommand::ClearTracks,
            RemodelCommand::ClearGeoProducts,
            RemodelCommand::ClearResult,
            RemodelCommand::SetSelection { mode: "rectangle".into(), ids: vec!["a".into()] },
            RemodelCommand::SetCamera { camera: remodel_engine::RemodelWorldCamera::default() },
            RemodelCommand::SetLayerVisibility { layer: "dense".into(), visible: false },
            RemodelCommand::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 },
            RemodelCommand::SetFrameCursor { stream_id: None, frame_index: 0 },
            RemodelCommand::SetReportTable { table: "gcps".into() },
            RemodelCommand::SetActiveUtility { utility_id: "measure".into() },
            RemodelCommand::SetLocale { value: "de-DE".into() },
            RemodelCommand::ImportFrames,
            RemodelCommand::ImportVideo,
            RemodelCommand::ExportQcReport,
        ]
    }

    #[test]
    fn every_command_variant_roundtrips_through_op_text() {
        for command in every_remodel_command() {
            store::test_support::assert_op_line_round_trip(&command);
        }
    }
    //#endregion 🔖️RemodelCommandTests

    //#region 🔖️WireBaselineDump
    /// 🧪️ [DEBUG] Temporary pre-migration wire dump (TEMPLATE.md §0.4) — delete once the post-migration
    /// diff is clean.
    #[test]
    fn debug_dump_remodel_command_wire_baseline() {
        use protocol::{OpBinary as _, OpText as _};
        for command in every_remodel_command() {
            let printed = command.print_op();
            let bytes = command.encode_op().expect("encode");
            let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            println!("[DEBUG][WIRE] {printed} | {} | {hex}", bytes.len());
        }
    }
    //#endregion 🔖️WireBaselineDump
}
//#endregion 🧪️Tests
