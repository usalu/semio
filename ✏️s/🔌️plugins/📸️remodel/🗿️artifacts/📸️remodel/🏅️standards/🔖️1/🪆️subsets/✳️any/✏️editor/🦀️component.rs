//! 📸️ Remodel editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch. `RemodelPlayApp` is a unit struct; every former `RemodelPlayRuntime` field
//! (camera/selection/layers/frame cursor/report table) lives in `crate::editor::remodel::config`, written
//! via `RemodelConfigMutation`s (real `backwards`, no ad hoc runtime mutation); every action dispatches
//! through the single typed `RemodelCommand` channel via `ArtifactEditor::handle`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window scenes in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, the photogrammetry stack in this app's own `⚙️engine` topic files (relocated from the
//! artifact tree, 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, #2553). This file is a
//! routing table: `handle` → `RemodelCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::editor::remodel::presence::{RemodelPresence, RemodelPresenceMutation};
use crate::editor::remodel::modes::{analyze, capture, model};
use crate::editor::remodel::panels::{calibration as calibration_panel, document, media, parameters, quality, results, tracks};
use crate::editor::remodel::engine::images as remodel_image;
use crate::editor::remodel::terminology::remodel_labels;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{default_remodel_scene, FrameRef, ImageAsset, MediaKind, MediaStream, RemodelSnapshot, REMODEL_DOCUMENT_SCHEMA};
use base64::Engine as _;
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppDefinition, AppIo, ConfigView, ArtifactEditor, ArtifactView, Dialect, Editor, Emit, Fault, FaultCode, FaultOrigin, GlbExporter, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm,
    MediaPayload, MediaPortDirection, MediaPortSpec, MediaType, MergeMode, MeshExporter, SelectionMethod, SelectionMode, SelectionSpec, UiNode, UtilityCategory, UtilityDefinition, WindowMeasure,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;
use serde_json::Value;
use std::collections::HashMap;
use store::ArtifactPack;

//#region 🔖️Constants
pub const REMODEL_PLAY_APP_ID: &str = "remodel-play";
/// 🔌️ Well-known stream id every `photos:in` import lands on — a stable identity so successive workflow
/// imports keep appending frames to the SAME stream (a pure `import_media` call has no runtime scratch
/// to remember which stream the last call used, unlike a UI drag-drop batch's `index == 0`/`> 0`
/// convention — see `🎮️commands/📥️import-frame-payload` for that one).
const REMODEL_WORKFLOW_PHOTOS_STREAM_ID: &str = "workflow-photos";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎚️options/*`) builds its `on_change`/item actions with.
pub async fn remodel_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(REMODEL_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_remodel_app` already declares via `.artifact_kind(...)`, plus the two Wave-2 port-recipe ports:
/// `photos:in` (incoming source images for reconstruction) and `mesh:out` (the current reconstructed mesh).
/// 🗄️ `export_formats`/`import_formats` are left empty, matching this same plugin's sibling
/// `artifact_kind()` (`🗿️artifacts/📸️remodel/🦀️component.rs`), which already carries the real dialect
/// ids on `ArtifactKindSpec::export_stdio_kinds`/`import_stdio_kinds` (`s.stdio.glb`/`obj`/`stl`/`ply`/
/// `las`/`png`) instead of the deprecated enum this file used to import from `semio_framework_plugin` — see
/// `26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT` W6. `AppIo` itself has no
/// such additive string-id peer fields yet (unlike `ArtifactKindSpec`), so unlike that sibling, this
/// list cannot be repopulated with real dialect ids without a framework-level `AppIo` change — out of
/// this plugin's write scope; flagged for the ticket's framework closer.
///
/// 🧭️ Relocated from the artifact's `⚙️engine/🦀️component.rs` (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
/// STATE-MACHINES, #2553): it returns `AppIo` — app-owned by construction — so it never belonged on
/// the artifact side.
pub async fn remodel_io() -> AppIo {
    AppIo {
        document_schema: "remodel.scene".into(),
        document_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        ports: vec![remodel_photos_in_port(), remodel_mesh_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.remodel".into(), name: "3D Remodel".into(), dimension: "3d".into(), component_kind: "remodel".into() },
    }
}

/// 🔌️ `photos:in` — incoming photos to insert as source images for reconstruction; pinned to the
/// `2d.image` kind (declared by `shooting`'s manifest — identical-shape duplicates are harmless, so this
/// app does not redeclare it).
pub async fn remodel_photos_in_port() -> MediaPortSpec {
    MediaPortSpec {
        id: "photos:in".into(),
        label: "Photos".into(),
        direction: MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🔌️ `mesh:out` — the current reconstructed mesh; pinned to the `3d.mesh` kind (declared by `lowpoly`'s
/// manifest — reused rather than redeclared, per the port recipe).
pub async fn remodel_mesh_out_port() -> MediaPortSpec {
    MediaPortSpec {
        id: "mesh:out".into(),
        label: "Mesh".into(),
        direction: MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        kind_id: Some("3d.mesh".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}
//#endregion 🔖️Io

//#region 🔖️Payloads
/// 📦️ Decodes a `requestFileOpen(readAs: "dataUrl")`/`RequestMediaFrames` payload into `(mime, bytes)`.
/// Relocated from the artifact's `⚙️engine/🦀️component.rs` (#2553): its only three consumers are all
/// app-side (`🎮️commands/📥️import-frame-payload`, `🎮️commands/🚀️run-reconstruction`, this app's own `import_media`).
pub async fn payload_from_data_url(data_url: &str) -> Option<(String, Vec<u8>)> {
    let (header, encoded) = data_url.split_once(',')?;
    let mime = header.strip_prefix("data:")?.split(';').next().unwrap_or("application/octet-stream").to_string();
    let bytes = base64::engine::general_purpose::STANDARD.decode(encoded).ok()?;
    Some((mime, bytes))
}

/// 🖼️ Decodes a still-image payload by mime — three consumers (`🎮️commands/📥️import-frame-payload`,
/// `🎮️commands/🚀️run-reconstruction`, this app's own `import_media`), and it takes no artifact-schema
/// type, so it stays app-side (relocated from `⚙️engine/🦀️component.rs`, #2553).
pub async fn decode_still_image(mime: &str, bytes: &[u8]) -> Result<remodel_image::ImageRgba8, remodel_image::ImageError> {
    if mime.contains("jpeg") || mime.contains("jpg") {
        remodel_image::decode_jpeg(bytes)
    } else {
        remodel_image::decode_png(bytes)
    }
}
//#endregion 🔖️Payloads

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `RemodelPlayApp::Command` — the SOLE dispatch surface for remodel's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies:
    /// `"setSelection" as "selection"` and `"setLocale" as "locale"` are the rows that prove it.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum RemodelCommand for RemodelSnapshot, RemodelMutation, RemodelConfig, RemodelConfigMutation {
        // 🚀️ Staged reconstruction — fully synchronous; there is no advance/cancel tick.
        "runReconstruction" as "run-reconstruction" => run_reconstruction::RunReconstruction,
        "retryStage" as "retry-stage" => retry_stage::RetryStage,
        "runStage" as "run-stage" => run_stage::RunStage,
        // 📥️ Ingestion.
        "importFramePayload" as "import-frame-payload" => import_frame_payload::ImportFramePayload,
        "importVideoFramePayload" as "import-video-frame-payload" => import_video_frame_payload::ImportVideoFramePayload,
        "importVideoDone" as "import-video-done" => import_video_done::ImportVideoDone,
        "importVideoBytesPayload" as "import-video-bytes-payload" => import_video_bytes_payload::ImportVideoBytesPayload,
        "addStream" as "add-stream" => add_stream::AddStream,
        "removeStream" as "remove-stream" => remove_stream::RemoveStream,
        "setStreamSync" as "stream-sync" => set_stream_sync::SetStreamSync,
        // 🎯️ Calibration / GCPs.
        "editCalibration" as "edit-calibration" => edit_calibration::EditCalibration,
        "calibrateCameras" as "calibrate-cameras" => calibrate_cameras::CalibrateCameras,
        "addGcp" as "add-gcp" => add_gcp::AddGcp,
        "removeGcp" as "remove-gcp" => remove_gcp::RemoveGcp,
        "placeGcpObservation" as "place-gcp-observation" => place_gcp_observation::PlaceGcpObservation,
        // ⚙️ 8 param-group setters, one per `ReconstructionParams` sub-struct.
        "setIngestParams" as "ingest-params" => set_ingest_params::SetIngestParams,
        "setFeatureParams" as "feature-params" => set_feature_params::SetFeatureParams,
        "setMatchParams" as "match-params" => set_match_params::SetMatchParams,
        "setSfmParams" as "sfm-params" => set_sfm_params::SetSfmParams,
        "setDenseParams" as "dense-params" => set_dense_params::SetDenseParams,
        "setMeshParams" as "mesh-params" => set_mesh_params::SetMeshParams,
        "setMotionParams" as "motion-params" => set_motion_params::SetMotionParams,
        "setGeoParams" as "geo-params" => set_geo_params::SetGeoParams,
        // 🧹️ Clear/reset.
        "resetPlaceholderMesh" as "reset-placeholder-mesh" => reset_placeholder_mesh::ResetPlaceholderMesh,
        "clearSparse" as "clear-sparse" => clear_sparse::ClearSparse,
        "clearDense" as "clear-dense" => clear_dense::ClearDense,
        "clearMeshResult" as "clear-mesh-result" => clear_mesh_result::ClearMeshResult,
        "clearTracks" as "clear-tracks" => clear_tracks::ClearTracks,
        "clearGeoProducts" as "clear-geo-products" => clear_geo_products::ClearGeoProducts,
        "clearResult" as "clear-result" => clear_result::ClearResult,
        // 👁️ Config-only — emit `config_mutations`, never document operations.
        "setCamera" as "camera" => set_camera::SetCamera,
        "setLayerVisibility" as "layer-visibility" => set_layer_visibility::SetLayerVisibility,
        "setFrameCursor" as "frame-cursor" => set_frame_cursor::SetFrameCursor,
        "setReportTable" as "report-table" => set_report_table::SetReportTable,
        // 🧰️ `setActiveUtility` is the framework-injected id (`SET_ACTIVE_UTILITY_ACTION_ID`).
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setLocale" as "locale" => set_locale::SetLocale,
        // 🐚️ Shell effects — no operations either way.
        "importFrames" as "import-frames" => import_frames::ImportFrames,
        "importVideo" as "import-video" => import_video::ImportVideo,
        "exportQcReport" as "export-qc-report" => export_qc_report::ExportQcReport,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::remodel::commands::{retry_stage, run_reconstruction, run_stage};
use crate::editor::remodel::commands::{add_gcp, calibrate_cameras, edit_calibration, place_gcp_observation, remove_gcp};
use crate::editor::remodel::commands::{add_stream, import_frame_payload, import_video_bytes_payload, import_video_done, import_video_frame_payload, remove_stream, set_stream_sync};
use crate::editor::remodel::commands::{set_dense_params, set_feature_params, set_geo_params, set_ingest_params, set_match_params, set_mesh_params, set_motion_params, set_sfm_params};
use crate::editor::remodel::commands::{clear_dense, clear_geo_products, clear_mesh_result, clear_result, clear_sparse, clear_tracks, reset_placeholder_mesh};
use crate::editor::remodel::commands::{export_qc_report, import_frames, import_video};
use crate::editor::remodel::commands::{set_active_utility, set_camera, set_frame_cursor, set_layer_visibility, set_locale, set_report_table};
//#endregion 🔖️Commands

//#region 🔖️ActionBridge
/// 🌉️ Host-args → typed-payload readers plus the action-id match — see `ArtifactEditor::command_from_action`
/// for why this exists (it did not before this migration). Kept in its own module so the routing table
/// above stays a routing table.
mod args_bridge {
    use super::*;

    async fn field<'a>(args: Option<&'a Value>, key: &str) -> Option<&'a Value> {
        args?.get(key)
    }

    /// 🔤️ A string arg — also accepts a number, so a select whose option ids are numeric (`textureSize`)
    /// reads the same whether the host sends `"2048"` or `2048`.
    async fn text(args: Option<&Value>, key: &str) -> Option<String> {
        match field(args, key)? {
            Value::String(value) => Some(value.clone()),
            Value::Number(value) => Some(value.to_string()),
            _ => None,
        }
    }

    /// 🔢️ A numeric arg — also accepts a numeric string, which is how select-sourced numbers arrive.
    async fn number(args: Option<&Value>, key: &str) -> Option<f64> {
        match field(args, key)? {
            Value::Number(value) => value.as_f64(),
            Value::String(value) => value.parse().ok(),
            _ => None,
        }
    }

    async fn flag(args: Option<&Value>, key: &str) -> Option<bool> {
        match field(args, key)? {
            Value::Bool(value) => Some(*value),
            Value::String(value) => value.parse().ok(),
            _ => None,
        }
    }

    async fn vec3(args: Option<&Value>, key: &str) -> Option<[f64; 3]> {
        let items = field(args, key)?.as_array()?;
        Some([items.first()?.as_f64()?, items.get(1)?.as_f64()?, items.get(2)?.as_f64()?])
    }

    async fn unknown(action: &str) -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("remodel has no command for action '{action}'"))
    }

    #[allow(clippy::too_many_lines)]
    pub async fn command_from_action(action: &str, args: Option<&Value>) -> Result<RemodelCommand, Fault> {
        let text_or = |key: &str, fallback: &str| text(args, key).unwrap_or_else(|| fallback.to_string());
        let f64_or = |key: &str, fallback: f64| number(args, key).unwrap_or(fallback);
        let f32_or = |key: &str, fallback: f32| number(args, key).map_or(fallback, |value| value as f32);
        let u32_or = |key: &str, fallback: u32| number(args, key).map_or(fallback, |value| value as u32);
        let bool_or = |key: &str, fallback: bool| flag(args, key).unwrap_or(fallback);
        Ok(match action {
            "runReconstruction" => RemodelCommand::RunReconstruction(run_reconstruction::RunReconstruction {}),
            "retryStage" => RemodelCommand::RetryStage(retry_stage::RetryStage { stage: text_or("stage", "") }),
            "runStage" => RemodelCommand::RunStage(run_stage::RunStage { stage: text_or("stage", "extracting-features") }),
            "importFramePayload" => RemodelCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload: text_or("payload", ""), name: text_or("name", ""), index: u32_or("index", 0) }),
            "importVideoFramePayload" => RemodelCommand::ImportVideoFramePayload(import_video_frame_payload::ImportVideoFramePayload {
                payload: text_or("payload", ""),
                name: text_or("name", ""),
                index: u32_or("index", 0),
                frame_index: u32_or("frameIndex", 0),
                timestamp_ms: f64_or("timestampMs", 0.0),
            }),
            "importVideoDone" => RemodelCommand::ImportVideoDone(import_video_done::ImportVideoDone {
                name: text_or("name", ""),
                duration_ms: f64_or("durationMs", 0.0),
                frame_count: u32_or("frameCount", 0),
                width: u32_or("width", 0),
                height: u32_or("height", 0),
                codec: text_or("codec", ""),
            }),
            "importVideoBytesPayload" => RemodelCommand::ImportVideoBytesPayload(import_video_bytes_payload::ImportVideoBytesPayload { payload: text_or("payload", ""), name: text_or("name", "") }),
            "addStream" => RemodelCommand::AddStream(add_stream::AddStream { name: text_or("name", "Stream"), kind: text_or("kind", "image-sequence"), camera_id: text_or("cameraId", "cam-0") }),
            "removeStream" => RemodelCommand::RemoveStream(remove_stream::RemoveStream { stream_id: text_or("streamId", "") }),
            "setStreamSync" => RemodelCommand::SetStreamSync(set_stream_sync::SetStreamSync { stream_id: text_or("streamId", ""), sync_offset_ms: f64_or("syncOffsetMs", 0.0) }),
            "editCalibration" => RemodelCommand::EditCalibration(edit_calibration::EditCalibration {
                camera_id: text_or("cameraId", ""),
                label: text_or("label", ""),
                model: text_or("model", "pinhole"),
                fx: f64_or("fx", 1000.0),
                fy: f64_or("fy", 1000.0),
                cx: f64_or("cx", 0.0),
                cy: f64_or("cy", 0.0),
                skew: f64_or("skew", 0.0),
                k1: f32_or("k1", 0.0),
                k2: f32_or("k2", 0.0),
                k3: f32_or("k3", 0.0),
                p1: f32_or("p1", 0.0),
                p2: f32_or("p2", 0.0),
                locked: bool_or("locked", false),
            }),
            "calibrateCameras" => RemodelCommand::CalibrateCameras(calibrate_cameras::CalibrateCameras {}),
            "addGcp" => RemodelCommand::AddGcp(add_gcp::AddGcp { name: text_or("name", "GCP"), world_x: f64_or("worldX", 0.0), world_y: f64_or("worldY", 0.0), world_z: f64_or("worldZ", 0.0) }),
            "removeGcp" => RemodelCommand::RemoveGcp(remove_gcp::RemoveGcp { gcp_id: text_or("gcpId", "") }),
            "placeGcpObservation" => RemodelCommand::PlaceGcpObservation(place_gcp_observation::PlaceGcpObservation {
                gcp_id: text_or("gcpId", ""),
                stream_id: text_or("streamId", ""),
                frame_index: u32_or("frameIndex", 0),
                pixel_x: f32_or("pixelX", 0.0),
                pixel_y: f32_or("pixelY", 0.0),
            }),
            "setIngestParams" => RemodelCommand::SetIngestParams(set_ingest_params::SetIngestParams {
                frame_sample_stride: u32_or("frameSampleStride", 5),
                max_frames: u32_or("maxFrames", 200),
                downscale_long_edge_px: u32_or("downscaleLongEdgePx", 1600),
                min_sharpness: f32_or("minSharpness", 0.3),
            }),
            "setFeatureParams" => {
                RemodelCommand::SetFeatureParams(set_feature_params::SetFeatureParams { detector: text_or("detector", "orb"), target_count: u32_or("targetCount", 4000), octaves: u32_or("octaves", 4), edge_threshold: f32_or("edgeThreshold", 10.0) })
            }
            "setMatchParams" => RemodelCommand::SetMatchParams(set_match_params::SetMatchParams {
                matcher: text_or("matcher", "brute-force"),
                ratio_test: f32_or("ratioTest", 0.8),
                cross_check: bool_or("crossCheck", true),
                sequential_window: u32_or("sequentialWindow", 8),
                max_pairs_per_frame: u32_or("maxPairsPerFrame", 16),
                loop_closure: bool_or("loopClosure", true),
            }),
            "setSfmParams" => RemodelCommand::SetSfmParams(set_sfm_params::SetSfmParams {
                ransac_iterations: u32_or("ransacIterations", 1000),
                ransac_threshold_px: f32_or("ransacThresholdPx", 2.0),
                min_track_length: u32_or("minTrackLength", 3),
                ba_max_iterations: u32_or("baMaxIterations", 50),
                robust_loss: text_or("robustLoss", "huber"),
                huber_delta_px: f32_or("huberDeltaPx", 1.5),
            }),
            "setDenseParams" => RemodelCommand::SetDenseParams(set_dense_params::SetDenseParams {
                resolution: text_or("resolution", "medium"),
                window_radius_px: u32_or("windowRadiusPx", 3),
                min_view_consistency: u32_or("minViewConsistency", 3),
                confidence_threshold: f32_or("confidenceThreshold", 0.5),
                max_points: u32_or("maxPoints", 500_000),
            }),
            "setMeshParams" => RemodelCommand::SetMeshParams(set_mesh_params::SetMeshParams {
                tsdf_voxel_size_mm: f32_or("tsdfVoxelSizeMm", 5.0),
                tsdf_truncation_mm: f32_or("tsdfTruncationMm", 20.0),
                decimate_target_triangles: u32_or("decimateTargetTriangles", 200_000),
                smoothing_iterations: u32_or("smoothingIterations", 2),
                texture_enabled: bool_or("textureEnabled", true),
                texture_size: u32_or("textureSize", 2048),
                guarantee_watertight: bool_or("guaranteeWatertight", true),
                hole_fill_max_boundary_verts: u32_or("holeFillMaxBoundaryVerts", 512),
                self_intersection_check: bool_or("selfIntersectionCheck", false),
            }),
            "setMotionParams" => RemodelCommand::SetMotionParams(set_motion_params::SetMotionParams {
                enabled: bool_or("enabled", false),
                max_tracks: u32_or("maxTracks", 64),
                track_window_px: u32_or("trackWindowPx", 21),
                min_track_quality: f32_or("minTrackQuality", 0.3),
                min_track_length_frames: u32_or("minTrackLengthFrames", 5),
            }),
            "setGeoParams" => RemodelCommand::SetGeoParams(set_geo_params::SetGeoParams {
                enabled: bool_or("enabled", false),
                origin_lon: number(args, "originLon"),
                origin_lat: number(args, "originLat"),
                origin_alt: number(args, "originAlt"),
                gsd_m: f32_or("gsdM", 0.05),
                dsm_cell_m: f32_or("dsmCellM", 0.1),
                dtm_filter_radius_m: f32_or("dtmFilterRadiusM", 2.0),
                ortho_max_px: u32_or("orthoMaxPx", 4096),
            }),
            "resetPlaceholderMesh" => RemodelCommand::ResetPlaceholderMesh(reset_placeholder_mesh::ResetPlaceholderMesh {}),
            "clearSparse" => RemodelCommand::ClearSparse(clear_sparse::ClearSparse {}),
            "clearDense" => RemodelCommand::ClearDense(clear_dense::ClearDense {}),
            "clearMeshResult" => RemodelCommand::ClearMeshResult(clear_mesh_result::ClearMeshResult {}),
            "clearTracks" => RemodelCommand::ClearTracks(clear_tracks::ClearTracks {}),
            "clearGeoProducts" => RemodelCommand::ClearGeoProducts(clear_geo_products::ClearGeoProducts {}),
            "clearResult" => RemodelCommand::ClearResult(clear_result::ClearResult {}),
            // 🎥️ The world-3d surface reports its orbit camera as flat `{position,target,fov}`; a
            // `{camera:{…}}`-shaped payload (what `RemodelWorldCamera` itself serializes to) is accepted too.
            "setCamera" => {
                let nested = field(args, "camera");
                let source = if nested.is_some() { nested } else { args };
                let default = crate::editor::remodel::config::RemodelWorldCamera::default();
                RemodelCommand::SetCamera(set_camera::SetCamera {
                    camera: crate::editor::remodel::config::RemodelWorldCamera {
                        position: vec3(source, "position").unwrap_or(default.position),
                        target: vec3(source, "target").unwrap_or(default.target),
                        fov: number(source, "fov").unwrap_or(default.fov),
                    },
                })
            }
            "setLayerVisibility" => RemodelCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: text_or("layer", ""), visible: bool_or("visible", true) }),
            "setFrameCursor" => RemodelCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: text(args, "streamId"), frame_index: u32_or("frameIndex", 0) }),
            "setReportTable" => RemodelCommand::SetReportTable(set_report_table::SetReportTable { table: text_or("table", "frames") }),
            "setActiveUtility" => RemodelCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: text_or("utilityId", "select") }),
            "setLocale" => RemodelCommand::SetLocale(set_locale::SetLocale { value: text_or("value", "en-US") }),
            "importFrames" => RemodelCommand::ImportFrames(import_frames::ImportFrames {}),
            "importVideo" => RemodelCommand::ImportVideo(import_video::ImportVideo {}),
            "exportQcReport" => RemodelCommand::ExportQcReport(export_qc_report::ExportQcReport {}),
            _ => return Err(unknown(action)),
        })
    }
}
//#endregion 🔖️ActionBridge

//#region 🔖️RemodelPlayApp
/// 🧪️ Unit struct — every former `RemodelPlayRuntime` field now lives in
/// `crate::editor::remodel::config::RemodelConfig` (see `ArtifactEditor::Config`).
#[derive(Default)]
pub struct RemodelPlayApp;

impl ArtifactEditor for RemodelPlayApp {
    type Snapshot = RemodelSnapshot;
    type Mutation = RemodelMutation;
    type Config = RemodelConfig;
    type ConfigMutation = RemodelConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = RemodelPresence;
    type PresenceMutation = RemodelPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = RemodelCommand;

    const DIALECT: Dialect = crate::artifacts::remodel::REMODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = REMODEL_DOCUMENT_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::remodel::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> RemodelSnapshot {
        default_remodel_scene()
    }

    async fn io() -> Option<AppIo> {
        Some(remodel_io())
    }

    /// 🎞️ `mesh:out` (the current reconstructed mesh, GLB-encoded) plus the inherited `document:out`
    /// default (the pack of `doc.snapshot`, replicated inline — overriding `export_media` shadows the
    /// trait's provided body for every port, not just the new one).
    async fn export_media(port: &str, doc: &ArtifactView<'_, RemodelSnapshot>) -> Result<Media, MediaError> {
        match port {
            "mesh:out" => {
                // 🧩️ `results.mesh.mesh` is now a composed `s.stdio.semio/v1/mesh` CHILD handle
                // (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) — the real `MeshData` reads
                // through the working-scene cache, honestly `Err` on a cold cache (documented
                // staleness gap, matches every prior exemplar in this ticket) rather than exporting a
                // fabricated empty mesh.
                let mesh = crate::artifacts::remodel::remodel_mesh_workspace(&doc.snapshot.results.mesh.mesh)
                    .ok_or_else(|| MediaError::Payload(port.to_string(), "mesh:out: composed mesh content not resolvable (cold working-scene cache)".into()))?;
                let bytes = MeshExporter::export(&GlbExporter, &mesh).map_err(|error| MediaError::Payload(port.to_string(), error))?;
                Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "3d.mesh".into(), json: base64::engine::general_purpose::STANDARD.encode(bytes) } })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `photos:in` — inserts an incoming photo as one new frame on the well-known
    /// `REMODEL_WORKFLOW_PHOTOS_STREAM_ID` image-sequence stream (creating it on the first import).
    /// `document:in` stays `MediaError::NotImplemented`, unchanged from the inherited default: remodel
    /// has no whole-document-replace `Mutation` variant to satisfy `whole_document_mutation`
    /// (`RemodelMutation` is deliberately field-granular — see that enum's doc comment).
    async fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RemodelSnapshot>) -> Result<Emit<RemodelMutation, RemodelConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "photos:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "photos:in only accepts a Structured base64-image payload".into()));
                };
                let bytes = base64::engine::general_purpose::STANDARD.decode(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let (width, height) = decode_still_image("image/png", &bytes).map_or((0, 0), |image| (image.width, image.height));
                let scene = doc.snapshot;
                let stream_id = REMODEL_WORKFLOW_PHOTOS_STREAM_ID;
                let frame_index = scene.streams.iter().find(|stream| stream.id == stream_id).map_or(0, |stream| stream.frames.len() as u32);
                let asset_key = format!("{stream_id}-frame-{frame_index}");
                let asset = ImageAsset { mime: "image/png".into(), data: json.clone(), width, height };
                let mut mutations = vec![crate::artifacts::remodel::mutations::create_asset(asset_key.clone(), asset)];
                match scene.streams.iter().any(|stream| stream.id == stream_id) {
                    true => mutations.push(crate::artifacts::remodel::mutations::add_stream_frame(
                        stream_id.to_string(),
                        FrameRef { index: frame_index, timestamp_ms: f64::from(frame_index) * 1000.0 / 30.0, asset_id: asset_key.clone() },
                        MediaKind::ImageSequence,
                    )),
                    false => mutations.push(crate::artifacts::remodel::mutations::create_stream(MediaStream {
                        id: stream_id.to_string(),
                        name: "Workflow Photos".into(),
                        kind: MediaKind::ImageSequence,
                        camera_id: None,
                        sync_offset_ms: 0.0,
                        fps_hint: 30.0,
                        frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: asset_key.clone() }],
                        source: None,
                    })),
                }
                Ok(Emit::mutations(mutations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &RemodelCommand) -> &'static str {
        command.command_id()
    }

    /// 🌉️ Host action id + JSON args → typed command. **Forward-fix, not a preserved behaviour**: the
    /// pre-migration `remodel_ui` never implemented this at all, so every one of remodel's own actions
    /// (the media drop zone, the layer toggles, every command-palette entry) fell through to the
    /// framework's reserved-action error and could only be reached by a direct `dispatch_typed` —
    /// i.e. the whole manifest surface was dead from the host's side. Arg keys are the camelCase ids
    /// declared in `🔖️Manifest`; each is read leniently (missing → the manifest's own default) because
    /// `effective_action_args` stages defaults before dispatch and select-typed args arrive as strings.
    async fn command_from_action(action: &str, args: Option<&Value>) -> Result<RemodelCommand, Fault> {
        args_bridge::command_from_action(action, args)
    }

    async fn handle(command: &RemodelCommand, doc: &ArtifactView<'_, RemodelSnapshot>, cfg: &ConfigView<'_, RemodelConfig>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<RemodelMutation, RemodelConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, RemodelSnapshot>, cfg: &ConfigView<'_, RemodelConfig>) -> UiNode {
        let scene = doc.snapshot;
        let config = cfg.snapshot;
        let labels = remodel_labels(config);
        match body_key {
            model::windows::model::REMODEL_PLAY_BODY_MAIN => model::windows::model::render(scene, config),
            capture::windows::frames::REMODEL_PLAY_BODY_FRAMES => capture::windows::frames::render(scene, config),
            analyze::windows::report::REMODEL_PLAY_BODY_REPORT => analyze::windows::report::render(scene, config),
            media::REMODEL_PLAY_BODY_MEDIA => media::render(scene, labels),
            document::REMODEL_PLAY_BODY_PIPELINE => document::render(scene, config.active_utility_id.as_str(), labels),
            results::REMODEL_PLAY_BODY_RESULTS => results::render(scene, labels),
            parameters::REMODEL_PLAY_BODY_PARAMETERS => parameters::render(scene, labels),
            calibration_panel::REMODEL_PLAY_BODY_CALIBRATION => calibration_panel::render(scene, labels),
            tracks::REMODEL_PLAY_BODY_TRACKS => tracks::render(scene, labels),
            quality::REMODEL_PLAY_BODY_QC => quality::render(scene, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 👁️ Dynamic per-render window measures — the Model window's layer toggles must reflect the LIVE
    /// config, so they are supplied here rather than frozen into the manifest.
    async fn window_measures(_doc: &ArtifactView<'_, RemodelSnapshot>, cfg: &ConfigView<'_, RemodelConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(model::windows::model::REMODEL_PLAY_WINDOW_MAIN.to_string(), model::windows::model::window_measures(cfg.snapshot, remodel_labels(cfg.snapshot)))])
    }
}
//#endregion 🔖️RemodelPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/utility declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub async fn create_remodel_app() -> AppDefinition {
    Editor::builder(crate::artifacts::remodel::REMODEL_DIALECT)
            .document(["semio", "remodel"])
            .artifact_kind(crate::artifacts::remodel::artifact_kind())
            // 🔌️ `photos:in`/`mesh:out` — `2d.image`/`3d.mesh` are declared by `shooting`/`lowpoly`
            // respectively (reused here, not redeclared).
            .media_input(remodel_photos_in_port())
            .media_output(remodel_mesh_out_port())
            .icon_id("remodel-app")
            .mode_def(capture::definition())
            .mode_def(model::definition())
            .mode_def(analyze::definition())
            .default_mode_id(model::REMODEL_PLAY_MODE_MODEL)
            .window_kind_def(model::windows::model::definition())
            .window_kind_def(capture::windows::frames::definition())
            .window_kind_def(analyze::windows::report::definition())
            // 🕹️ The "assets" interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)
            // replaces the deleted `RemodelSelection` config field — both windows that carry the "select"
            // utility (`remodel-main`'s World3d viewport, `remodel-frames`' Canvas2d frame view) declare it.
            .interaction(InteractionDefinition {
                id: "assets".into(),
                label: LocalizedLabel::native("Assets", "Assets"),
                granularities: vec![GranularityDefinition { id: "asset".into(), label: LocalizedLabel::native("Asset", "Asset"), icon_id: "image".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(model::windows::model::REMODEL_PLAY_WINDOW_MAIN, vec![InteractionRef::new("assets")])
            .window_kind_interactions(capture::windows::frames::REMODEL_PLAY_WINDOW_FRAMES, vec![InteractionRef::new("assets")])
            .default_layout(model::layout())
            .named_layout(capture::layout())
            .named_layout(analyze::layout())
            .mode_layout(capture::REMODEL_PLAY_MODE_CAPTURE, capture::REMODEL_PLAY_LAYOUT_CAPTURE)
            .mode_layout(analyze::REMODEL_PLAY_MODE_ANALYZE, analyze::REMODEL_PLAY_LAYOUT_ANALYZE)
            .panel_tab_def(document::definition())
            .panel_tab_def(media::definition())
            .panel_tab_def(results::definition())
            .panel_tab_def(parameters::definition())
            .panel_tab_def(calibration_panel::definition())
            .panel_tab_def(tracks::definition())
            .panel_tab_def(quality::definition())
            // 🚀️ Staged reconstruction — fully synchronous (see the module doc comment); there is no
            // `advanceReconstruction`/`cancelReconstruction` action.
            .mutation("runReconstruction", LocalizedLabel::native("Run Reconstruction", "Rekonstruktion starten"))
            .mutation("retryStage", LocalizedLabel::native("Retry", "Wiederholen"))
            .mutation("runStage", LocalizedLabel::native("Run Stage", "Stufe ausführen"))
            .action_args("runStage", vec![ActionArgDef::select(
                "stage",
                LocalizedLabel::native("Stage", "Stufe"),
                vec![
                    ActionArgOption::new("extracting-features", LocalizedLabel::native("Extracting Features", "Merkmale extrahieren")),
                    ActionArgOption::new("matching-features", LocalizedLabel::native("Matching Features", "Merkmale zuordnen")),
                    ActionArgOption::new("estimating-poses", LocalizedLabel::native("Estimating Poses", "Posen schätzen")),
                    ActionArgOption::new("bundle-adjusting", LocalizedLabel::native("Bundle Adjusting", "Bündelausgleich")),
                    ActionArgOption::new("dense-stereo", LocalizedLabel::native("Dense Stereo", "Dense-Stereo")),
                    ActionArgOption::new("fusing-volume", LocalizedLabel::native("Fusing Volume", "Volumen fusionieren")),
                    ActionArgOption::new("extracting-surface", LocalizedLabel::native("Extracting Surface", "Oberfläche extrahieren")),
                    ActionArgOption::new("texturing", LocalizedLabel::native("Texturing", "Texturierung")),
                ],
            )
            .default_value("extracting-features")])
            // 📥️ Ingestion.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new_catalog("importFrames", LocalizedLabel::native("Import Frames", "Frames importieren"), ActionKind::Shell) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importFramePayload", LocalizedLabel::native("Import Frame Payload", "Bild-Payload importieren"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new_catalog("importVideo", LocalizedLabel::native("Import Video", "Video importieren"), ActionKind::Shell) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importVideoFramePayload", LocalizedLabel::native("Import Video Frame Payload", "Video-Frame-Payload importieren"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importVideoDone", LocalizedLabel::native("Import Video Done", "Video-Import abgeschlossen"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importVideoBytesPayload", LocalizedLabel::native("Import Video Bytes Payload", "Video-Byte-Payload importieren"), ActionKind::Mutation) })
            .mutation("addStream", LocalizedLabel::native("Add Stream", "Stream hinzufügen"))
            .action_args("addStream", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value("Stream"),
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![ActionArgOption::new("image-sequence", LocalizedLabel::native("Image Sequence", "Bildsequenz")), ActionArgOption::new("video", LocalizedLabel::native("Video", "Video"))]).default_value("image-sequence"),
                ActionArgDef::text("cameraId", LocalizedLabel::native("Camera Id", "Kamera-Id")).default_value("cam-0"),
            ])
            .mutation("removeStream", LocalizedLabel::native("Remove Stream", "Stream entfernen"))
            .action_args("removeStream", vec![ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required()])
            .mutation("setStreamSync", LocalizedLabel::native("Set Stream Sync", "Stream-Synchronisation festlegen"))
            .action_args("setStreamSync", vec![ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required(), ActionArgDef::number("syncOffsetMs", LocalizedLabel::native("Sync Offset (ms)", "Sync-Versatz (ms)")).default_value(0)])
            // 🎯️ Calibration / GCPs.
            .mutation("editCalibration", LocalizedLabel::native("Edit Calibration", "Kalibrierung bearbeiten"))
            .action_args("editCalibration", vec![
                ActionArgDef::text("cameraId", LocalizedLabel::native("Camera Id", "Kamera-Id")).required(),
                ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")),
                ActionArgDef::select("model", LocalizedLabel::native("Model", "Modell"), vec![ActionArgOption::new("pinhole", LocalizedLabel::native("Pinhole", "Lochkamera")), ActionArgOption::new("brownConrady", LocalizedLabel::native("Brown-Conrady", "Brown-Conrady")), ActionArgOption::new("fisheye", LocalizedLabel::native("Fisheye", "Fischauge"))]).default_value("pinhole"),
                ActionArgDef::number("fx", LocalizedLabel::native("fx", "fx")).default_value(1000),
                ActionArgDef::number("fy", LocalizedLabel::native("fy", "fy")).default_value(1000),
                ActionArgDef::number("cx", LocalizedLabel::native("cx", "cx")).default_value(0),
                ActionArgDef::number("cy", LocalizedLabel::native("cy", "cy")).default_value(0),
                ActionArgDef::number("skew", LocalizedLabel::native("Skew", "Scherung")).default_value(0),
                ActionArgDef::number("k1", LocalizedLabel::native("k1", "k1")).default_value(0),
                ActionArgDef::number("k2", LocalizedLabel::native("k2", "k2")).default_value(0),
                ActionArgDef::number("k3", LocalizedLabel::native("k3", "k3")).default_value(0),
                ActionArgDef::number("p1", LocalizedLabel::native("p1", "p1")).default_value(0),
                ActionArgDef::number("p2", LocalizedLabel::native("p2", "p2")).default_value(0),
                ActionArgDef::toggle("locked", LocalizedLabel::native("Locked", "Gesperrt")).default_value(false),
            ])
            .mutation("calibrateCameras", LocalizedLabel::native("Calibrate Cameras", "Kameras kalibrieren"))
            .mutation("addGcp", LocalizedLabel::native("Add Ground Control Point", "Passpunkt hinzufügen"))
            .action_args("addGcp", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value("GCP"),
                ActionArgDef::number("worldX", LocalizedLabel::native("World X", "Welt X")).default_value(0),
                ActionArgDef::number("worldY", LocalizedLabel::native("World Y", "Welt Y")).default_value(0),
                ActionArgDef::number("worldZ", LocalizedLabel::native("World Z", "Welt Z")).default_value(0),
            ])
            .mutation("removeGcp", LocalizedLabel::native("Remove Ground Control Point", "Passpunkt entfernen"))
            .action_args("removeGcp", vec![ActionArgDef::text("gcpId", LocalizedLabel::native("GCP Id", "Passpunkt-Id")).required()])
            .mutation("placeGcpObservation", LocalizedLabel::native("Place GCP Observation", "Passpunkt-Beobachtung setzen"))
            .action_args("placeGcpObservation", vec![
                ActionArgDef::text("gcpId", LocalizedLabel::native("GCP Id", "Passpunkt-Id")).required(),
                ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required(),
                ActionArgDef::number("frameIndex", LocalizedLabel::native("Frame Index", "Frame-Index")).required(),
                ActionArgDef::number("pixelX", LocalizedLabel::native("Pixel X", "Pixel X")).required(),
                ActionArgDef::number("pixelY", LocalizedLabel::native("Pixel Y", "Pixel Y")).required(),
            ])
            // ⚙️ 8 param-group setters, one per `ReconstructionParams` sub-struct.
            .mutation("setIngestParams", LocalizedLabel::native("Set Ingest Params", "Ingest-Parameter festlegen"))
            .action_args("setIngestParams", vec![
                ActionArgDef::number("frameSampleStride", LocalizedLabel::native("Frame Sample Stride", "Frame-Abtastschrittweite")).default_value(5),
                ActionArgDef::number("maxFrames", LocalizedLabel::native("Max Frames", "Max. Frames")).default_value(200),
                ActionArgDef::number("downscaleLongEdgePx", LocalizedLabel::native("Downscale Long Edge (px)", "Verkleinerung lange Kante (px)")).default_value(1600),
                ActionArgDef::slider("minSharpness", LocalizedLabel::native("Min Sharpness", "Min. Schärfe"), 0.0, 1.0).default_value(0.3),
            ])
            .mutation("setFeatureParams", LocalizedLabel::native("Set Feature Params", "Feature-Parameter festlegen"))
            .action_args("setFeatureParams", vec![
                ActionArgDef::select("detector", LocalizedLabel::native("Detector", "Detektor"), vec![ActionArgOption::new("orb", LocalizedLabel::native("ORB", "ORB")), ActionArgOption::new("akaze", LocalizedLabel::native("AKAZE", "AKAZE")), ActionArgOption::new("harris", LocalizedLabel::native("Harris", "Harris"))]).default_value("orb"),
                ActionArgDef::number("targetCount", LocalizedLabel::native("Target Count", "Ziel-Anzahl")).default_value(4000),
                ActionArgDef::number("octaves", LocalizedLabel::native("Octaves", "Oktaven")).default_value(4),
                ActionArgDef::slider("edgeThreshold", LocalizedLabel::native("Edge Threshold", "Kanten-Schwelle"), 1.0, 50.0).default_value(10.0),
            ])
            .mutation("setMatchParams", LocalizedLabel::native("Set Match Params", "Match-Parameter festlegen"))
            .action_args("setMatchParams", vec![
                ActionArgDef::select("matcher", LocalizedLabel::native("Matcher", "Matcher"), vec![ActionArgOption::new("brute-force", LocalizedLabel::native("Brute Force", "Brute Force")), ActionArgOption::new("kd-tree", LocalizedLabel::native("KD-Tree", "KD-Baum"))]).default_value("brute-force"),
                ActionArgDef::slider("ratioTest", LocalizedLabel::native("Ratio Test", "Verhältnistest"), 0.1, 1.0).default_value(0.8),
                ActionArgDef::toggle("crossCheck", LocalizedLabel::native("Cross Check", "Kreuzprüfung")).default_value(true),
                ActionArgDef::number("sequentialWindow", LocalizedLabel::native("Sequential Window", "Sequenzielles Fenster")).default_value(8),
                ActionArgDef::number("maxPairsPerFrame", LocalizedLabel::native("Max Pairs Per Frame", "Max. Paare pro Frame")).default_value(16),
                ActionArgDef::toggle("loopClosure", LocalizedLabel::native("Loop Closure", "Schleifenschluss")).default_value(true),
            ])
            .mutation("setSfmParams", LocalizedLabel::native("Set SfM Params", "SfM-Parameter festlegen"))
            .action_args("setSfmParams", vec![
                ActionArgDef::number("ransacIterations", LocalizedLabel::native("RANSAC Iterations", "RANSAC-Iterationen")).default_value(1000),
                ActionArgDef::slider("ransacThresholdPx", LocalizedLabel::native("RANSAC Threshold (px)", "RANSAC-Schwelle (px)"), 0.1, 10.0).default_value(2.0),
                ActionArgDef::number("minTrackLength", LocalizedLabel::native("Min Track Length", "Min. Spurlänge")).default_value(3),
                ActionArgDef::number("baMaxIterations", LocalizedLabel::native("BA Max Iterations", "BA Max. Iterationen")).default_value(50),
                ActionArgDef::select("robustLoss", LocalizedLabel::native("Robust Loss", "Robuster Verlust"), vec![ActionArgOption::new("l2", LocalizedLabel::native("L2", "L2")), ActionArgOption::new("huber", LocalizedLabel::native("Huber", "Huber")), ActionArgOption::new("cauchy", LocalizedLabel::native("Cauchy", "Cauchy"))]).default_value("huber"),
                ActionArgDef::slider("huberDeltaPx", LocalizedLabel::native("Huber Delta (px)", "Huber-Delta (px)"), 0.1, 10.0).default_value(1.5),
            ])
            .mutation("setDenseParams", LocalizedLabel::native("Set Dense Params", "Dense-Parameter festlegen"))
            .action_args("setDenseParams", vec![
                ActionArgDef::select("resolution", LocalizedLabel::native("Resolution", "Auflösung"), vec![ActionArgOption::new("low", LocalizedLabel::native("Low", "Niedrig")), ActionArgOption::new("medium", LocalizedLabel::native("Medium", "Mittel")), ActionArgOption::new("high", LocalizedLabel::native("High", "Hoch"))]).default_value("medium"),
                ActionArgDef::number("windowRadiusPx", LocalizedLabel::native("Window Radius (px)", "Fensterradius (px)")).default_value(3),
                ActionArgDef::number("minViewConsistency", LocalizedLabel::native("Min View Consistency", "Min. Ansichtskonsistenz")).default_value(3),
                ActionArgDef::slider("confidenceThreshold", LocalizedLabel::native("Confidence Threshold", "Konfidenzschwelle"), 0.0, 1.0).default_value(0.5),
                ActionArgDef::number("maxPoints", LocalizedLabel::native("Max Points", "Max. Punkte")).default_value(500_000),
            ])
            .mutation("setMeshParams", LocalizedLabel::native("Set Mesh Params", "Mesh-Parameter festlegen"))
            .action_args("setMeshParams", vec![
                ActionArgDef::slider("tsdfVoxelSizeMm", LocalizedLabel::native("TSDF Voxel Size (mm)", "TSDF-Voxelgröße (mm)"), 1.0, 20.0).default_value(5.0),
                ActionArgDef::slider("tsdfTruncationMm", LocalizedLabel::native("TSDF Truncation (mm)", "TSDF-Trunkierung (mm)"), 2.0, 60.0).default_value(20.0),
                ActionArgDef::number("decimateTargetTriangles", LocalizedLabel::native("Decimate Target Triangles", "Ziel-Dreiecke (Dezimierung)")).default_value(200_000),
                ActionArgDef::number("smoothingIterations", LocalizedLabel::native("Smoothing Iterations", "Glättungs-Iterationen")).default_value(2),
                ActionArgDef::toggle("textureEnabled", LocalizedLabel::native("Texture Enabled", "Textur aktiviert")).default_value(true),
                ActionArgDef::select("textureSize", LocalizedLabel::native("Texture Size", "Texturgröße"), vec![ActionArgOption::new("1024", LocalizedLabel::native("1024", "1024")), ActionArgOption::new("2048", LocalizedLabel::native("2048", "2048")), ActionArgOption::new("4096", LocalizedLabel::native("4096", "4096"))]).default_value("2048"),
                ActionArgDef::toggle("guaranteeWatertight", LocalizedLabel::native("Guarantee Watertight", "Wasserdichtheit garantieren")).default_value(true),
                ActionArgDef::number("holeFillMaxBoundaryVerts", LocalizedLabel::native("Hole Fill Max Boundary Verts", "Max. Randpunkte für Lochfüllung")).default_value(512),
                ActionArgDef::toggle("selfIntersectionCheck", LocalizedLabel::native("Self-Intersection Check", "Selbstüberschneidungsprüfung")).default_value(false),
            ])
            .mutation("setMotionParams", LocalizedLabel::native("Set Motion Params", "Bewegungs-Parameter festlegen"))
            .action_args("setMotionParams", vec![
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).default_value(false),
                ActionArgDef::number("maxTracks", LocalizedLabel::native("Max Tracks", "Max. Spuren")).default_value(64),
                ActionArgDef::number("trackWindowPx", LocalizedLabel::native("Track Window (px)", "Spurfenster (px)")).default_value(21),
                ActionArgDef::slider("minTrackQuality", LocalizedLabel::native("Min Track Quality", "Min. Spurqualität"), 0.0, 1.0).default_value(0.3),
                ActionArgDef::number("minTrackLengthFrames", LocalizedLabel::native("Min Track Length (frames)", "Min. Spurlänge (Frames)")).default_value(5),
            ])
            .mutation("setGeoParams", LocalizedLabel::native("Set Geo Params", "Geo-Parameter festlegen"))
            .action_args("setGeoParams", vec![
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).default_value(false),
                ActionArgDef::number("originLon", LocalizedLabel::native("Origin Longitude", "Ursprung Längengrad")).default_value(0),
                ActionArgDef::number("originLat", LocalizedLabel::native("Origin Latitude", "Ursprung Breitengrad")).default_value(0),
                ActionArgDef::number("originAlt", LocalizedLabel::native("Origin Altitude", "Ursprung Höhe")).default_value(0),
                ActionArgDef::slider("gsdM", LocalizedLabel::native("Ground Sample Distance (m)", "Bodenauflösung (m)"), 0.01, 1.0).default_value(0.05),
                ActionArgDef::slider("dsmCellM", LocalizedLabel::native("DSM Cell Size (m)", "DOM-Zellgröße (m)"), 0.01, 5.0).default_value(0.1),
                ActionArgDef::slider("dtmFilterRadiusM", LocalizedLabel::native("DTM Filter Radius (m)", "DGM-Filterradius (m)"), 0.1, 10.0).default_value(2.0),
                ActionArgDef::number("orthoMaxPx", LocalizedLabel::native("Ortho Max (px)", "Ortho Max. (px)")).default_value(4096),
            ])
            // 🧹️ Clear/reset.
            .mutation("resetPlaceholderMesh", LocalizedLabel::native("Reset Placeholder Mesh", "Platzhalter-Mesh zurücksetzen"))
            .mutation("clearSparse", LocalizedLabel::native("Clear Sparse Cloud", "Dünne Punktwolke löschen"))
            .mutation("clearDense", LocalizedLabel::native("Clear Dense Cloud", "Dichte Punktwolke löschen"))
            .mutation("clearMeshResult", LocalizedLabel::native("Clear Mesh", "Mesh löschen"))
            .mutation("clearTracks", LocalizedLabel::native("Clear Tracks", "Spuren löschen"))
            .mutation("clearGeoProducts", LocalizedLabel::native("Clear Geo Products", "Geo-Produkte löschen"))
            .mutation("clearResult", LocalizedLabel::native("Clear Result", "Ergebnis löschen"))
            // 👁️ View-only runtime actions.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setLayerVisibility", LocalizedLabel::native("Set Layer Visibility", "Ebenensichtbarkeit festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setFrameCursor", LocalizedLabel::native("Set Frame Cursor", "Frame-Cursor festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setReportTable", LocalizedLabel::native("Set Report Table", "Berichtstabelle festlegen"), ActionKind::View) })
            // 📤️ Export.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new_catalog("exportQcReport", LocalizedLabel::native("Export QC Report", "QC-Bericht exportieren"), ActionKind::Shell) })
            // 🧰️ Utility groups — an exclusive per-window set (active utility is host-owned); which
            // window exposes which is declared by that window's own `definition()`.
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer-2") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("sculpt", LocalizedLabel::native("Sculpt", "Formen"), "paintbrush") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("measure", LocalizedLabel::native("Measure", "Messen"), "scaling") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("gcpPlace", LocalizedLabel::native("Place GCP", "Passpunkt setzen"), "crosshair") })
            // 🎯️ Typed channel surface — `io()` is this same information's single source of truth,
            // reused here rather than duplicated.
            .io(remodel_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old default-example
            // registration (`default_remodel_scene().print_dsl()` fed to `.example("default", …)`)
            // and the no-op `.workflow("remodel", …)` call are dropped here (not silently: reported
            // in this packet's migration notes). The subset's own `📚️examples/🎬️demo` facet
            // (`crate::artifacts::remodel::examples::…`, real content, pre-existing) is the modern,
            // role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// ✏️ `RemodelPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<RemodelPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<RemodelPlayApp>` builds it.
    pub type RemodelApp = VcsArtifactApp<EditorApp<RemodelPlayApp>>;

    /// ✏️ Adapts `create_remodel_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::assert_declared_actions_bridge_to_commands`/
    /// `testkit::new_app_with_registry` still expect — framework testkit gap, not modifiable here
    /// (`🧰️framework/**` is outside this packet's lease).
    pub async fn remodel_app_manifest_for_testkit() -> App {
        App { definition: create_remodel_app(), examples: Vec::new() }
    }

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn app() -> RemodelApp {
        new_app::<EditorApp<RemodelPlayApp>>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn app_with_registry() -> RemodelApp {
        new_app_with_registry::<EditorApp<RemodelPlayApp>>(remodel_app_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut RemodelApp, command: RemodelCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut RemodelApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodel::testkit::{app, app_with_registry, remodel_app_manifest_for_testkit, render};
    use protocol::{OpBinary, OpText};
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::{EditorApp, HistoryView};

    //#region 🔖️CommandSurface
    /// ⚡️ One representative value per `RemodelCommand` row, in declaration (= binary ordinal) order —
    /// the permanent wire guard's fixture, carried over verbatim from the pre-migration
    /// `remodel_protocol` baseline (see this ticket's `🧪️wire-baseline-before.txt`).
    async fn every_command() -> Vec<RemodelCommand> {
        vec![
            RemodelCommand::RunReconstruction(run_reconstruction::RunReconstruction {}),
            RemodelCommand::RetryStage(retry_stage::RetryStage { stage: "extracting-features".into() }),
            RemodelCommand::RunStage(run_stage::RunStage { stage: "dense-stereo".into() }),
            RemodelCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload: "data:image/png;base64,abc".into(), name: "frame.png".into(), index: 0 }),
            RemodelCommand::ImportVideoFramePayload(import_video_frame_payload::ImportVideoFramePayload { payload: "data:image/jpeg;base64,abc".into(), name: "clip.mp4".into(), index: 1, frame_index: 1, timestamp_ms: 33.3 }),
            RemodelCommand::ImportVideoDone(import_video_done::ImportVideoDone { name: "clip.mp4".into(), duration_ms: 400.0, frame_count: 4, width: 24, height: 24, codec: "mjpeg".into() }),
            RemodelCommand::ImportVideoBytesPayload(import_video_bytes_payload::ImportVideoBytesPayload { payload: "data:video/mp4;base64,abc".into(), name: "clip.mp4".into() }),
            RemodelCommand::AddStream(add_stream::AddStream { name: "Stream".into(), kind: "video".into(), camera_id: "cam-0".into() }),
            RemodelCommand::RemoveStream(remove_stream::RemoveStream { stream_id: "stream-1".into() }),
            RemodelCommand::SetStreamSync(set_stream_sync::SetStreamSync { stream_id: "stream-1".into(), sync_offset_ms: 12.5 }),
            RemodelCommand::EditCalibration(edit_calibration::EditCalibration {
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
            }),
            RemodelCommand::CalibrateCameras(calibrate_cameras::CalibrateCameras {}),
            RemodelCommand::AddGcp(add_gcp::AddGcp { name: "GCP".into(), world_x: 0.0, world_y: 0.0, world_z: 0.0 }),
            RemodelCommand::RemoveGcp(remove_gcp::RemoveGcp { gcp_id: "gcp-1".into() }),
            RemodelCommand::PlaceGcpObservation(place_gcp_observation::PlaceGcpObservation { gcp_id: "gcp-1".into(), stream_id: "stream-1".into(), frame_index: 0, pixel_x: 10.0, pixel_y: 20.0 }),
            RemodelCommand::SetIngestParams(set_ingest_params::SetIngestParams { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 }),
            RemodelCommand::SetFeatureParams(set_feature_params::SetFeatureParams { detector: "orb".into(), target_count: 4000, octaves: 4, edge_threshold: 10.0 }),
            RemodelCommand::SetMatchParams(set_match_params::SetMatchParams { matcher: "brute-force".into(), ratio_test: 0.8, cross_check: true, sequential_window: 8, max_pairs_per_frame: 16, loop_closure: true }),
            RemodelCommand::SetSfmParams(set_sfm_params::SetSfmParams { ransac_iterations: 1000, ransac_threshold_px: 2.0, min_track_length: 3, ba_max_iterations: 50, robust_loss: "huber".into(), huber_delta_px: 1.5 }),
            RemodelCommand::SetDenseParams(set_dense_params::SetDenseParams { resolution: "medium".into(), window_radius_px: 3, min_view_consistency: 3, confidence_threshold: 0.5, max_points: 500_000 }),
            RemodelCommand::SetMeshParams(set_mesh_params::SetMeshParams {
                tsdf_voxel_size_mm: 5.0,
                tsdf_truncation_mm: 20.0,
                decimate_target_triangles: 200_000,
                smoothing_iterations: 2,
                texture_enabled: true,
                texture_size: 2048,
                guarantee_watertight: true,
                hole_fill_max_boundary_verts: 512,
                self_intersection_check: false,
            }),
            RemodelCommand::SetMotionParams(set_motion_params::SetMotionParams { enabled: false, max_tracks: 64, track_window_px: 21, min_track_quality: 0.3, min_track_length_frames: 5 }),
            RemodelCommand::SetGeoParams(set_geo_params::SetGeoParams { enabled: false, origin_lon: None, origin_lat: Some(1.0), origin_alt: None, gsd_m: 0.05, dsm_cell_m: 0.1, dtm_filter_radius_m: 2.0, ortho_max_px: 4096 }),
            RemodelCommand::ResetPlaceholderMesh(reset_placeholder_mesh::ResetPlaceholderMesh {}),
            RemodelCommand::ClearSparse(clear_sparse::ClearSparse {}),
            RemodelCommand::ClearDense(clear_dense::ClearDense {}),
            RemodelCommand::ClearMeshResult(clear_mesh_result::ClearMeshResult {}),
            RemodelCommand::ClearTracks(clear_tracks::ClearTracks {}),
            RemodelCommand::ClearGeoProducts(clear_geo_products::ClearGeoProducts {}),
            RemodelCommand::ClearResult(clear_result::ClearResult {}),
            RemodelCommand::SetCamera(set_camera::SetCamera { camera: crate::editor::remodel::config::RemodelWorldCamera::default() }),
            RemodelCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: "dense".into(), visible: false }),
            RemodelCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 }),
            RemodelCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: None, frame_index: 0 }),
            RemodelCommand::SetReportTable(set_report_table::SetReportTable { table: "gcps".into() }),
            RemodelCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "measure".into() }),
            RemodelCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            RemodelCommand::ImportFrames(import_frames::ImportFrames {}),
            RemodelCommand::ImportVideo(import_video::ImportVideo {}),
            RemodelCommand::ExportQcReport(export_qc_report::ExportQcReport {}),
        ]
    }

    /// ⚖️ Every row round trips through BOTH projections, and its printed line starts with the row's own
    /// wire keyword — the guard that catches a missing `#[dsl(keyword = ..)]` on a payload struct, which
    /// no round-trip law alone would notice.
    #[test]
    async fn every_command_variant_roundtrips_and_prints_its_wire_keyword() {
        let keywords: Vec<&str> = vec![
            "run-reconstruction",
            "retry-stage",
            "run-stage",
            "import-frame-payload",
            "import-video-frame-payload",
            "import-video-done",
            "import-video-bytes-payload",
            "add-stream",
            "remove-stream",
            "stream-sync",
            "edit-calibration",
            "calibrate-cameras",
            "add-gcp",
            "remove-gcp",
            "place-gcp-observation",
            "ingest-params",
            "feature-params",
            "match-params",
            "sfm-params",
            "dense-params",
            "mesh-params",
            "motion-params",
            "geo-params",
            "reset-placeholder-mesh",
            "clear-sparse",
            "clear-dense",
            "clear-mesh-result",
            "clear-tracks",
            "clear-geo-products",
            "clear-result",
            "camera",
            "layer-visibility",
            "frame-cursor",
            "frame-cursor",
            "report-table",
            "active-utility",
            "locale",
            "import-frames",
            "import-video",
            "export-qc-report",
        ];
        let commands = every_command();
        assert_eq!(commands.len(), keywords.len(), "the keyword list must cover every row");
        for (command, keyword) in commands.iter().zip(keywords) {
            store::os_store::test_support::assert_op_text_binary_equivalence(command);
            assert!(command.print_op().starts_with(keyword), "row must print its wire keyword {keyword}, got {:?}", command.print_op());
        }
    }

    /// 📌️ Pinned hex for the rows whose `Option` fields make `None`/`Some` distinct wire cases, plus the
    /// two fieldless-variant shapes. `SetFrameCursor`'s ordinal shifted 33→32 (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM deleted the `setSelection` row ahead of it) —
    /// a legitimate wire break on this greenfield repo, not a bug. A reordered row or a changed field
    /// order breaks these immediately.
    #[test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &RemodelCommand| command.encode_op().expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&RemodelCommand::RunReconstruction(run_reconstruction::RunReconstruction {})), "01000000", "fieldless row 0");
        assert_eq!(hex(&RemodelCommand::ClearResult(clear_result::ClearResult {})), "011d0000", "fieldless row 29");
        assert_eq!(hex(&RemodelCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: None, frame_index: 0 })), "01200001010400", "Option field absent");
        assert_eq!(hex(&RemodelCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 })), "0120010873747265616d2d3102000600010402", "Option field present");
        assert_eq!(
            hex(&RemodelCommand::SetGeoParams(set_geo_params::SetGeoParams { enabled: false, origin_lon: None, origin_lat: Some(1.0), origin_alt: None, gsd_m: 0.05, dsm_cell_m: 0.1, dtm_filter_radius_m: 2.0, ortho_max_px: 4096 })),
            "0116000600010205000000000000f03f0405000000a09999a93f0505000000a09999b93f0605000000000000004007048020",
            "three interleaved Option fields, only the middle one present"
        );
    }

    /// 🏷️ Every manifest action id and every wire keyword is distinct — the cross-cutting invariant
    /// `app_commands!` exists to keep true (the fixture lists `setFrameCursor` twice on purpose, so the
    /// row count, not the fixture length, is what must dedupe cleanly).
    #[test]
    async fn command_ids_and_wire_keywords_are_unique_per_row() {
        let mut ids: Vec<&str> = every_command().iter().map(RemodelCommand::command_id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 39, "39 distinct manifest action ids");

        let mut keywords: Vec<String> = every_command().iter().map(|command| command.print_op().split_whitespace().next().unwrap_or_default().to_string()).collect();
        keywords.sort();
        keywords.dedup();
        assert_eq!(keywords.len(), 39, "39 distinct wire keywords");
    }
    /// 🌉️ The action bridge covers every action the manifest declares (framework-injected ones aside)
    /// and rejects anything else — the gap this migration closed (see `command_from_action`'s doc).
    #[test]
    async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        testkit::assert_declared_actions_bridge_to_commands::<EditorApp<RemodelPlayApp>>(remodel_app_manifest_for_testkit);
        assert!(RemodelPlayApp::command_from_action("nonsense", None).is_err());
    }

    /// 🌉️ Select-typed args arrive as strings; numeric-option selects (`textureSize`) must still land in
    /// a `u32` field, and a `setCamera` payload is accepted both flat and `{camera:{…}}`-nested.
    #[test]
    async fn the_action_bridge_coerces_select_strings_and_both_camera_arg_shapes() {
        let mesh = RemodelPlayApp::command_from_action("setMeshParams", Some(&serde_json::json!({ "textureSize": "4096" }))).expect("bridge");
        let RemodelCommand::SetMeshParams(payload) = mesh else { panic!("expected SetMeshParams") };
        assert_eq!(payload.texture_size, 4096);

        let flat = RemodelPlayApp::command_from_action("setCamera", Some(&serde_json::json!({ "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "fov": 60.0 }))).expect("bridge");
        let nested = RemodelPlayApp::command_from_action("setCamera", Some(&serde_json::json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "fov": 60.0 } }))).expect("bridge");
        assert_eq!(flat, nested);
    }

    //#endregion 🔖️CommandSurface

    /// 🖼️ Render smoke test: every window/panel body key this app declares must render without panicking.
    #[test]
    async fn render_does_not_panic_for_known_body_keys() {
        let mut app = app();
        for body_key in [
            model::windows::model::REMODEL_PLAY_BODY_MAIN,
            capture::windows::frames::REMODEL_PLAY_BODY_FRAMES,
            analyze::windows::report::REMODEL_PLAY_BODY_REPORT,
            media::REMODEL_PLAY_BODY_MEDIA,
            document::REMODEL_PLAY_BODY_PIPELINE,
            results::REMODEL_PLAY_BODY_RESULTS,
            parameters::REMODEL_PLAY_BODY_PARAMETERS,
            calibration_panel::REMODEL_PLAY_BODY_CALIBRATION,
            tracks::REMODEL_PLAY_BODY_TRACKS,
            quality::REMODEL_PLAY_BODY_QC,
        ] {
            let _ = render(&mut app, body_key);
        }
    }

    #[test]
    async fn render_unknown_body_key_reports_it_by_name() {
        let mut app = app();
        assert!(render(&mut app, "remodel.play.nope").contains("Unknown body: remodel.play.nope"));
    }

    //#region 🔖️ManifestSanity
    #[test]
    async fn the_manifest_declares_three_modes_three_windows_and_this_apps_panel_tabs() {
        let definition = create_remodel_app().definition;
        assert_eq!(definition.modes.len(), 3);
        assert_eq!(definition.window_kinds.len(), 3);
        for panel_id in [media::REMODEL_PANEL_MEDIA_ID, results::REMODEL_PANEL_RESULTS_ID, parameters::REMODEL_PANEL_PARAMETERS_ID, calibration_panel::REMODEL_PANEL_CALIBRATION_ID, tracks::REMODEL_PANEL_TRACKS_ID, quality::REMODEL_PANEL_QC_ID] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.id() == panel_id), "panel tab {panel_id} must be present");
        }
    }

    #[test]
    async fn remodel_io_declares_photos_in_and_mesh_out_on_the_manifest() {
        let app = create_remodel_app();
        assert!(app.definition.media_inputs.iter().any(|port| port.id == "photos:in"));
        assert!(app.definition.media_outputs.iter().any(|port| port.id == "mesh:out"));
    }

    /// 🧰️ The registry-backed app enforces View/Shell kind discipline — a view row must not slip
    /// through as an operation.
    #[test]
    async fn view_rows_dispatch_cleanly_against_the_real_registry() {
        let mut app = app_with_registry();
        let result = testkit::meta("local");
        app.dispatch_typed(RemodelCommand::SetReportTable(set_report_table::SetReportTable { table: "tracks".into() }), &result).expect("view dispatch");
    }
    //#endregion 🔖️ManifestSanity

    /// 🧪️ The definitional proof: two independent instances start from the same document, apply DISJOINT
    /// field edits (A tunes feature params, B adds a ground control point), and exchanging operations
    /// over a `MemoryBackbone` converges both sides to contain BOTH edits — impossible under a
    /// whole-document `setDocument` snapshot, where one side's write would clobber the other's.
    #[test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<EditorApp<RemodelPlayApp>, _>(
            "mem://remodel-convergence",
            RemodelCommand::SetFeatureParams(set_feature_params::SetFeatureParams { detector: "akaze".into(), target_count: 1000, octaves: 4, edge_threshold: 10.0 }),
            RemodelCommand::AddGcp(add_gcp::AddGcp { name: "corner".into(), world_x: 1.0, world_y: 2.0, world_z: 3.0 }),
            |app| {
                let projection = app.snapshot().expect("materialize projection");
                (projection.params.feature.detector, projection.gcps.first().map(|gcp| gcp.name.clone()))
            },
        );
    }

    //#region 🔖️MediaPortTests
    /// 🔌️ `photos:in` inserts an incoming photo as one new frame on the well-known workflow-photos
    /// stream, creating it on the first import and appending on subsequent ones.
    #[test]
    async fn import_media_photos_in_creates_and_appends_to_the_workflow_stream() {
        let app = app();
        let projection = app.snapshot().expect("projection");
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let inner = RemodelPlayApp;
        let media = Media {
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
            payload: MediaPayload::Structured {
                schema: "2d.image".into(),
                json: base64::engine::general_purpose::STANDARD.encode(crate::editor::remodel::engine::images::encode_png(&crate::editor::remodel::engine::images::ImageRgba8::new(4, 4)).expect("encode png")),
            },
        };
        let emit = RemodelPlayApp::import_media("photos:in", &media, &doc).expect("photos:in import");
        assert_eq!(emit.artifact_mutations.len(), 2, "one create-asset + one create-stream");
        let next = emit.artifact_mutations.iter().fold(projection.clone(), |scene, operation| {
            crate::artifacts::remodel::op::apply_remodel_mutation(&scene, operation)
                .expect("valid mutation diff")
        });
        assert_eq!(next.streams.len(), 1);
        assert_eq!(next.streams[0].id, REMODEL_WORKFLOW_PHOTOS_STREAM_ID);
        assert_eq!(next.streams[0].frames.len(), 1);

        let history2 = HistoryView::empty();
        let doc2 = ArtifactView::new(&next, &history2);
        let emit2 = RemodelPlayApp::import_media("photos:in", &media, &doc2).expect("second photos:in import");
        let next2 = emit2.artifact_mutations.iter().fold(next.clone(), |scene, operation| {
            crate::artifacts::remodel::op::apply_remodel_mutation(&scene, operation)
                .expect("valid mutation diff")
        });
        assert_eq!(next2.streams.len(), 1, "still one workflow-photos stream");
        assert_eq!(next2.streams[0].frames.len(), 2, "second import appends a second frame");
    }

    /// 🔌️ `mesh:out` exports the current reconstructed mesh as a GLB-encoded `3d.mesh` `Media`.
    #[test]
    async fn export_media_mesh_out_exports_a_structured_3d_mesh() {
        let app = app();
        let projection = app.snapshot().expect("projection");
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let media = semio_framework_plugin::resolve_ready(RemodelPlayApp::export_media("mesh:out", &doc)).expect("mesh:out export");
        assert_eq!(media.media_type.class, MediaClass::ThreeD);
        assert_eq!(media.media_type.form, MediaForm::Mesh);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "3d.mesh");
                assert!(!json.is_empty());
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }
    //#endregion 🔖️MediaPortTests

    //#region 🔖️IoTests
    /// 🧪️ Relocated from the artifact's `⚙️engine/🦀️component.rs` (#2553): `remodel_io()` returns
    /// `AppIo` and lives app-side now, so its own declaration test travels with it.
    #[test]
    async fn remodel_io_declares_photos_in_and_mesh_out() {
        let io = remodel_io();
        assert_eq!(io.document_schema, "remodel.scene");
        assert_eq!(io.artifact.id, "3d.remodel");
        let photos_in = io.ports.iter().find(|port| port.id == "photos:in").expect("photos:in declared");
        assert_eq!(photos_in.direction, MediaPortDirection::In);
        assert_eq!(photos_in.kind_id.as_deref(), Some("2d.image"));
        assert!(!photos_in.required);
        assert_eq!(photos_in.multiplicity, semio_framework::PortMultiplicity::Many);
        let mesh_out = io.ports.iter().find(|port| port.id == "mesh:out").expect("mesh:out declared");
        assert_eq!(mesh_out.direction, MediaPortDirection::Out);
        assert_eq!(mesh_out.kind_id.as_deref(), Some("3d.mesh"));
        assert!(!mesh_out.required);
        assert_eq!(mesh_out.multiplicity, semio_framework::PortMultiplicity::Many);
    }
    //#endregion 🔖️IoTests
}
//#endregion 🧪️Tests
