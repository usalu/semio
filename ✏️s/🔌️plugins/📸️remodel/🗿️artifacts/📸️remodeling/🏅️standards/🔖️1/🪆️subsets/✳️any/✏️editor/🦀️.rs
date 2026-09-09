//! 📸️ Remodeling editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch. `RemodelingPlayApp` is a unit struct; every former `RemodelingPlayRuntime` field
//! (camera/selection/layers/frame cursor/report table) lives in `crate::editor::remodeling::config`, written
//! via `RemodelingConfigMutation`s (real `backwards`, no ad hoc runtime mutation); every action dispatches
//! through the single typed `RemodelingCommand` channel via `ArtifactEditor::handle`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window scenes in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, the photogrammetry stack in this app's own `⚙️engine` topic files (relocated from the
//! artifact tree, 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, #2553). This file is a
//! routing table: `handle` → `RemodelingCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::editor::remodeling::engine::images as remodeling_image;
use crate::editor::remodeling::modes::{analyze, capture, model};
use crate::editor::remodeling::panels::{calibration as calibration_panel, document, media, parameters, quality, results, tracks};
use crate::editor::remodeling::presence::{RemodelingPresence, RemodelingPresenceMutation};
use crate::editor::remodeling::terminology::remodeling_labels;
use crate::op::RemodelingMutation;
use crate::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelingSnapshot, REMODELING_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppDefinition, AppIo, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp,
    Emit, Fault, FaultCode, FaultOrigin, GlbExporter, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractiveJobClassification, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm,
    MediaPayload, MediaPortDirection, MediaPortSpec, MediaType, MergeMode, MeshExporter, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, UtilityCategory, UtilityDefinition, WindowMeasure,
};
use std::collections::HashMap;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const REMODELING_PLAY_APP_ID: &str = "remodeling-play";
/// 🔌️ Well-known stream id every `photos:in` import lands on — a stable identity so successive workflow
/// imports keep appending frames to the SAME stream (a pure `import_media` call has no runtime scratch
/// to remember which stream the last call used, unlike a UI drag-drop batch's `index == 0`/`> 0`
/// convention — see `🎮️commands/🖼️import-frame-payload` for that one).
const REMODELING_WORKFLOW_PHOTOS_STREAM_ID: &str = "workflow-photos";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `☑️options/*`) builds its `on_change`/item actions with.
pub fn remodeling_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(REMODELING_PLAY_APP_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// ☑️ A window-measure/`ActionDescriptor` action — `WindowMeasure` is the RETAINED (`ui_wgpu`) vocabulary
/// and takes a plain `ActionDescriptor { controller_id, action, args }`, not the fallible
/// `(ActionId, Option<UiValue>)` pair `remodeling_action` builds for semantic-contract chrome.
pub fn remodeling_window_action(action: &str, args: Option<dsl::DslValue>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionDescriptor { controller_id: REMODELING_PLAY_APP_ID.into(), action: action.into(), args }
}

/// 🏷️ Admits one dynamic string as a semantic-contract `Label` — the SECOND `Label` type in scope
/// (`plugin_app_close_prelude::Label`, what `PanelTreeBuilder::section`/`tree_item` take), never the
/// retained `ui_wgpu` `Label` the SDK root's glob also re-exports and this file imports unqualified.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI label admission failed"))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_remodeling_app` already declares via `.artifact_kind(...)`, plus the two Wave-2 port-recipe ports:
/// `photos:in` (incoming source images for reconstruction) and `mesh:out` (the current reconstructed mesh).
/// 🗄️ `export_formats`/`import_formats` are left empty, matching this same plugin's sibling
/// `artifact_kind()` (`🗿️artifacts/📸️remodeling/🦀️.rs`), which already carries the real dialect
/// ids on `ArtifactKindSpec::export_stdio_kinds`/`import_stdio_kinds` (`s.stdio.glb`/`obj`/`stl`/`ply`/
/// `las`/`png`) instead of the deprecated enum this file used to import from `semio_framework_plugin` — see
/// `26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT` W6. `AppIo` itself has no
/// such additive string-id peer fields yet (unlike `ArtifactKindSpec`), so unlike that sibling, this
/// list cannot be repopulated with real dialect ids without a framework-level `AppIo` change — out of
/// this plugin's write scope; flagged for the ticket's framework closer.
///
/// 🧭️ Relocated from the artifact's `⚙️engine/🦀️.rs` (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
/// STATE-MACHINES, #2553): it returns `AppIo` — app-owned by construction — so it never belonged on
/// the artifact side.
pub fn remodeling_io() -> AppIo {
    AppIo {
        document_schema: "remodeling.scene".into(),
        document_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        ports: vec![remodeling_photos_in_port(), remodeling_mesh_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.remodeling".into(), name: "3D Remodeling".into(), dimension: "3d".into(), component_kind: "remodeling".into() },
    }
}

/// 🔌️ `photos:in` — incoming photos to insert as source images for reconstruction; pinned to the
/// `2d.image` kind (declared by `shooting`'s manifest — identical-shape duplicates are harmless, so this
/// app does not redeclare it).
pub fn remodeling_photos_in_port() -> MediaPortSpec {
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
pub fn remodeling_mesh_out_port() -> MediaPortSpec {
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
/// Relocated from the artifact's `⚙️engine/🦀️.rs` (#2553): its only three consumers are all
/// app-side (`🎮️commands/🖼️import-frame-payload`, `🎮️commands/🏗️run-reconstruction`, this app's own `import_media`).
pub fn payload_from_data_url(data_url: &str) -> Option<(String, Vec<u8>)> {
    let (header, encoded) = data_url.split_once(',')?;
    let mime = header.strip_prefix("data:")?.split(';').next().unwrap_or("application/octet-stream").to_string();
    let bytes = base64_codec::base64_standard_decode(encoded).ok()?;
    Some((mime, bytes))
}

/// 🖼️ Decodes a still-image payload by mime — three consumers (`🎮️commands/🖼️import-frame-payload`,
/// `🎮️commands/🏗️run-reconstruction`, this app's own `import_media`), and it takes no artifact-schema
/// type, so it stays app-side (relocated from `⚙️engine/🦀️.rs`, #2553).
pub fn decode_still_image(mime: &str, bytes: &[u8]) -> Result<remodeling_image::ImageRgba8, remodeling_image::ImageError> {
    if mime.contains("jpeg") || mime.contains("jpg") {
        remodeling_image::decode_jpeg(bytes)
    } else {
        remodeling_image::decode_png(bytes)
    }
}
//#endregion 🔖️Payloads

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `RemodelingPlayApp::Command` — the SOLE dispatch surface for remodeling's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies:
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum RemodelingCommand for RemodelingSnapshot, RemodelingMutation, RemodelingConfig, RemodelingConfigMutation {
        // 🚀️ Generation-tagged reconstruction; the hidden row is appended to preserve ordinals.
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
        // 🐚️ Shell effects — no operations either way.
        "importFrames" as "import-frames" => import_frames::ImportFrames,
        "importVideo" as "import-video" => import_video::ImportVideo,
        "exportQcReport" as "export-qc-report" => export_qc_report::ExportQcReport,
        "advanceReconstruction" as "advance-reconstruction" => advance_reconstruction::AdvanceReconstruction,
        "cancelReconstruction" as "cancel-reconstruction" => cancel_reconstruction::CancelReconstruction,
        // 🎬️ Example picker — appended last, preserving every existing variant ordinal.
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::remodeling::commands::{add_gcp, calibrate_cameras, edit_calibration, place_gcp_observation, remove_gcp};
use crate::editor::remodeling::commands::{add_stream, import_frame_payload, import_video_bytes_payload, import_video_done, import_video_frame_payload, remove_stream, set_stream_sync};
use crate::editor::remodeling::commands::{advance_reconstruction, cancel_reconstruction, retry_stage, run_reconstruction, run_stage};
use crate::editor::remodeling::commands::{clear_dense, clear_geo_products, clear_mesh_result, clear_result, clear_sparse, clear_tracks, reset_placeholder_mesh};
use crate::editor::remodeling::commands::{export_qc_report, import_frames, import_video};
use crate::editor::remodeling::commands::{set_active_example, set_camera, set_frame_cursor, set_layer_visibility, set_report_table};
use crate::editor::remodeling::commands::{set_dense_params, set_feature_params, set_geo_params, set_ingest_params, set_match_params, set_mesh_params, set_motion_params, set_sfm_params};
//#endregion 🔖️Commands

//#region 🔖️ActionBridge
/// 🌉️ Host-args → typed-payload readers plus the action-id match — see `ArtifactEditor::command_from_action`
/// for why this exists (it did not before this migration). Kept in its own module so the routing table
/// above stays a routing table.
mod args_bridge {
    use super::*;

    fn field<'a>(args: Option<&'a dsl::DslValue>, key: &str) -> Option<&'a dsl::DslValue> {
        args?.get(key)
    }

    /// 🔤️ A string arg — also accepts a number, so a select whose option ids are numeric (`textureSize`)
    /// reads the same whether the host sends `"2048"` or `2048`.
    fn text(args: Option<&dsl::DslValue>, key: &str) -> Option<String> {
        let value = field(args, key)?;
        value.as_str().map(str::to_owned).or_else(|| value.as_u64().map(|number| number.to_string())).or_else(|| value.as_i64().map(|number| number.to_string())).or_else(|| value.as_f64().map(|number| number.to_string()))
    }

    /// 🔢️ A numeric arg — also accepts a numeric string, which is how select-sourced numbers arrive.
    fn number(args: Option<&dsl::DslValue>, key: &str) -> Option<f64> {
        let value = field(args, key)?;
        value.as_f64().or_else(|| value.as_str()?.parse().ok())
    }

    fn flag(args: Option<&dsl::DslValue>, key: &str) -> Option<bool> {
        let value = field(args, key)?;
        value.as_bool().or_else(|| value.as_str()?.parse().ok())
    }

    fn vec3(args: Option<&dsl::DslValue>, key: &str) -> Option<[f64; 3]> {
        let items = field(args, key)?.as_array()?;
        Some([items.first()?.as_f64()?, items.get(1)?.as_f64()?, items.get(2)?.as_f64()?])
    }

    fn unknown(action: &str) -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("remodeling has no command for action '{action}'"))
    }

    #[allow(clippy::too_many_lines)]
    pub fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<RemodelingCommand, Fault> {
        let text_or = |key: &str, fallback: &str| text(args, key).unwrap_or_else(|| fallback.to_string());
        let f64_or = |key: &str, fallback: f64| number(args, key).unwrap_or(fallback);
        let f32_or = |key: &str, fallback: f32| number(args, key).map_or(fallback, |value| value as f32);
        let u32_or = |key: &str, fallback: u32| number(args, key).map_or(fallback, |value| value as u32);
        let u64_or = |key: &str, fallback: u64| number(args, key).map_or(fallback, |value| value as u64);
        let bool_or = |key: &str, fallback: bool| flag(args, key).unwrap_or(fallback);
        Ok(match action {
            "runReconstruction" => RemodelingCommand::RunReconstruction(run_reconstruction::RunReconstruction {}),
            "retryStage" => RemodelingCommand::RetryStage(retry_stage::RetryStage { stage: text_or("stage", "extracting-features") }),
            "runStage" => RemodelingCommand::RunStage(run_stage::RunStage { stage: text_or("stage", "extracting-features") }),
            "advanceReconstruction" => RemodelingCommand::AdvanceReconstruction(advance_reconstruction::AdvanceReconstruction {
                generation: u64_or("generation", 0),
                job_id: text_or("jobId", ""),
                requested_stage: text_or("requestedStage", "full"),
                phase: text_or("phase", "pipeline"),
                stream_index: u32_or("streamIndex", 0),
                frame_index: u32_or("frameIndex", 0),
                terminal_cursor: u64_or("terminalCursor", 0),
                tick: u32_or("tick", 0),
            }),
            "cancelReconstruction" => RemodelingCommand::CancelReconstruction(cancel_reconstruction::CancelReconstruction {}),
            "importFramePayload" => RemodelingCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload: text_or("payload", ""), name: text_or("name", ""), index: u32_or("index", 0) }),
            "importVideoFramePayload" => RemodelingCommand::ImportVideoFramePayload(import_video_frame_payload::ImportVideoFramePayload {
                payload: text_or("payload", ""),
                name: text_or("name", ""),
                index: u32_or("index", 0),
                frame_index: u32_or("frameIndex", 0),
                timestamp_ms: f64_or("timestampMs", 0.0),
            }),
            "importVideoDone" => RemodelingCommand::ImportVideoDone(import_video_done::ImportVideoDone {
                name: text_or("name", ""),
                duration_ms: f64_or("durationMs", 0.0),
                frame_count: u32_or("frameCount", 0),
                width: u32_or("width", 0),
                height: u32_or("height", 0),
                codec: text_or("codec", ""),
            }),
            "importVideoBytesPayload" => RemodelingCommand::ImportVideoBytesPayload(import_video_bytes_payload::ImportVideoBytesPayload { payload: text_or("payload", ""), name: text_or("name", "") }),
            "addStream" => RemodelingCommand::AddStream(add_stream::AddStream { name: text_or("name", "Stream"), kind: text_or("kind", "image-sequence"), camera_id: text_or("cameraId", "cam-0") }),
            "removeStream" => RemodelingCommand::RemoveStream(remove_stream::RemoveStream { stream_id: text_or("streamId", "") }),
            "setStreamSync" => RemodelingCommand::SetStreamSync(set_stream_sync::SetStreamSync { stream_id: text_or("streamId", ""), sync_offset_ms: f64_or("syncOffsetMs", 0.0) }),
            "editCalibration" => RemodelingCommand::EditCalibration(edit_calibration::EditCalibration {
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
            "calibrateCameras" => RemodelingCommand::CalibrateCameras(calibrate_cameras::CalibrateCameras {}),
            "addGcp" => RemodelingCommand::AddGcp(add_gcp::AddGcp { name: text_or("name", "GCP"), world_x: f64_or("worldX", 0.0), world_y: f64_or("worldY", 0.0), world_z: f64_or("worldZ", 0.0) }),
            "removeGcp" => RemodelingCommand::RemoveGcp(remove_gcp::RemoveGcp { gcp_id: text_or("gcpId", "") }),
            "placeGcpObservation" => RemodelingCommand::PlaceGcpObservation(place_gcp_observation::PlaceGcpObservation {
                gcp_id: text_or("gcpId", ""),
                stream_id: text_or("streamId", ""),
                frame_index: u32_or("frameIndex", 0),
                pixel_x: f32_or("pixelX", 0.0),
                pixel_y: f32_or("pixelY", 0.0),
            }),
            "setIngestParams" => RemodelingCommand::SetIngestParams(set_ingest_params::SetIngestParams {
                frame_sample_stride: u32_or("frameSampleStride", 5),
                max_frames: u32_or("maxFrames", 200),
                downscale_long_edge_px: u32_or("downscaleLongEdgePx", 1600),
                min_sharpness: f32_or("minSharpness", 0.3),
            }),
            "setFeatureParams" => RemodelingCommand::SetFeatureParams(set_feature_params::SetFeatureParams {
                detector: text_or("detector", "orb"),
                target_count: u32_or("targetCount", 4000),
                octaves: u32_or("octaves", 4),
                edge_threshold: f32_or("edgeThreshold", 10.0),
            }),
            "setMatchParams" => RemodelingCommand::SetMatchParams(set_match_params::SetMatchParams {
                matcher: text_or("matcher", "brute-force"),
                ratio_test: f32_or("ratioTest", 0.8),
                cross_check: bool_or("crossCheck", true),
                sequential_window: u32_or("sequentialWindow", 8),
                max_pairs_per_frame: u32_or("maxPairsPerFrame", 16),
                loop_closure: bool_or("loopClosure", true),
            }),
            "setSfmParams" => RemodelingCommand::SetSfmParams(set_sfm_params::SetSfmParams {
                ransac_iterations: u32_or("ransacIterations", 1000),
                ransac_threshold_px: f32_or("ransacThresholdPx", 2.0),
                min_track_length: u32_or("minTrackLength", 3),
                ba_max_iterations: u32_or("baMaxIterations", 50),
                robust_loss: text_or("robustLoss", "huber"),
                huber_delta_px: f32_or("huberDeltaPx", 1.5),
            }),
            "setDenseParams" => RemodelingCommand::SetDenseParams(set_dense_params::SetDenseParams {
                resolution: text_or("resolution", "medium"),
                window_radius_px: u32_or("windowRadiusPx", 3),
                min_view_consistency: u32_or("minViewConsistency", 3),
                confidence_threshold: f32_or("confidenceThreshold", 0.5),
                max_points: u32_or("maxPoints", 500_000),
            }),
            "setMeshParams" => RemodelingCommand::SetMeshParams(set_mesh_params::SetMeshParams {
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
            "setMotionParams" => RemodelingCommand::SetMotionParams(set_motion_params::SetMotionParams {
                enabled: bool_or("enabled", false),
                max_tracks: u32_or("maxTracks", 64),
                track_window_px: u32_or("trackWindowPx", 21),
                min_track_quality: f32_or("minTrackQuality", 0.3),
                min_track_length_frames: u32_or("minTrackLengthFrames", 5),
            }),
            "setGeoParams" => RemodelingCommand::SetGeoParams(set_geo_params::SetGeoParams {
                enabled: bool_or("enabled", false),
                origin_lon: number(args, "originLon"),
                origin_lat: number(args, "originLat"),
                origin_alt: number(args, "originAlt"),
                gsd_m: f32_or("gsdM", 0.05),
                dsm_cell_m: f32_or("dsmCellM", 0.1),
                dtm_filter_radius_m: f32_or("dtmFilterRadiusM", 2.0),
                ortho_max_px: u32_or("orthoMaxPx", 4096),
            }),
            "resetPlaceholderMesh" => RemodelingCommand::ResetPlaceholderMesh(reset_placeholder_mesh::ResetPlaceholderMesh {}),
            "clearSparse" => RemodelingCommand::ClearSparse(clear_sparse::ClearSparse {}),
            "clearDense" => RemodelingCommand::ClearDense(clear_dense::ClearDense {}),
            "clearMeshResult" => RemodelingCommand::ClearMeshResult(clear_mesh_result::ClearMeshResult {}),
            "clearTracks" => RemodelingCommand::ClearTracks(clear_tracks::ClearTracks {}),
            "clearGeoProducts" => RemodelingCommand::ClearGeoProducts(clear_geo_products::ClearGeoProducts {}),
            "clearResult" => RemodelingCommand::ClearResult(clear_result::ClearResult {}),
            // 🎥️ The world-3d surface reports its orbit camera as flat `{position,target,fov}`; a
            // `{camera:{…}}`-shaped payload (what `RemodelingWorldCamera` itself serializes to) is accepted too.
            "setCamera" => {
                let nested = field(args, "camera");
                let source = if nested.is_some() { nested } else { args };
                let default = crate::editor::remodeling::config::RemodelingWorldCamera::default();
                RemodelingCommand::SetCamera(set_camera::SetCamera {
                    camera: crate::editor::remodeling::config::RemodelingWorldCamera {
                        position: vec3(source, "position").unwrap_or(default.position),
                        target: vec3(source, "target").unwrap_or(default.target),
                        fov: number(source, "fov").unwrap_or(default.fov),
                    },
                })
            }
            "setLayerVisibility" => RemodelingCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: text_or("layer", ""), visible: bool_or("visible", true) }),
            "setFrameCursor" => RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: text(args, "streamId"), frame_index: u32_or("frameIndex", 0) }),
            "setReportTable" => RemodelingCommand::SetReportTable(set_report_table::SetReportTable { table: text_or("table", "frames") }),
            "importFrames" => RemodelingCommand::ImportFrames(import_frames::ImportFrames {}),
            "importVideo" => RemodelingCommand::ImportVideo(import_video::ImportVideo {}),
            "exportQcReport" => RemodelingCommand::ExportQcReport(export_qc_report::ExportQcReport {}),
            "setActiveExample" => RemodelingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text_or("exampleId", crate::editor::remodeling::examples::REMODELING_EXAMPLE_BOOT_ID) }),
            _ => return Err(unknown(action)),
        })
    }
}
//#endregion 🔖️ActionBridge

//#region 🔖️RemodelingPlayApp
/// 🧪️ Unit struct — every former `RemodelingPlayRuntime` field now lives in
/// `crate::editor::remodeling::config::RemodelingConfig` (see `ArtifactEditor::Config`).
#[derive(Default)]
pub struct RemodelingPlayApp;

//#region 🧵️RetainedCommands
/// 🧵️ Every `RemodelingCommand` row, in `app_commands!` declaration order. The framework demands set
/// EQUALITY between this list, the `Migrated` classification list in `🔖️Manifest` and the
/// `bounded_first_step_tool_proofs!` rows (`validate_tool_job_rows`: `expected = TOOL_JOB_IDS ∩ migrated`
/// must equal the proof set, else `interactive-job.catalog-incomplete`), so all three are kept in one
/// order and asserted equal by `retained_route_dispositions_are_exact_and_exhaustive`.
const REMODELING_RETAINED_TOOL_IDS: &[&str] = &[
    "runReconstruction",
    "retryStage",
    "runStage",
    "importFramePayload",
    "importVideoFramePayload",
    "importVideoDone",
    "importVideoBytesPayload",
    "addStream",
    "removeStream",
    "setStreamSync",
    "editCalibration",
    "calibrateCameras",
    "addGcp",
    "removeGcp",
    "placeGcpObservation",
    "setIngestParams",
    "setFeatureParams",
    "setMatchParams",
    "setSfmParams",
    "setDenseParams",
    "setMeshParams",
    "setMotionParams",
    "setGeoParams",
    "resetPlaceholderMesh",
    "clearSparse",
    "clearDense",
    "clearMeshResult",
    "clearTracks",
    "clearGeoProducts",
    "clearResult",
    "setCamera",
    "setLayerVisibility",
    "setFrameCursor",
    "setReportTable",
    "importFrames",
    "importVideo",
    "exportQcReport",
    "advanceReconstruction",
    "cancelReconstruction",
    "setActiveExample",
];
const REMODELING_RETAINED_PAYLOAD_SCHEMA: &str = "remodeling.scene.tool-command.v1";
const REMODELING_RETAINED_RAW_BYTES: usize = 65_536;
const REMODELING_RETAINED_WORK_ITEMS: usize = 4_096;

/// 🛣️ One publication lane per route, read off each handler's own `Emit` in `🎮️commands/*/🦀️.rs`:
/// every reconstruction, ingestion, calibration, parameter and clear verb builds
/// `Emit::mutations(..)`/`Emit::amend(..)`/`Emit { artifact_mutations, .. }` over `RemodelingMutation`
/// (`Artifact`); the six session verbs build `Emit::config(..)` over `RemodelingConfigMutation`
/// (`Config`); the three shell verbs build `Emit::effect(..)` only — `RequestFileOpen`,
/// `RequestMediaFrames`, `DownloadMediaExport` — and never touch a store (`HostOnly`). No remodeling
/// handler emits two store lanes, a draft, a presence or a transient mutation, so no route declares
/// more than one lane.
const REMODELING_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    artifact_route("runReconstruction"),
    artifact_route("retryStage"),
    artifact_route("runStage"),
    artifact_route("importFramePayload"),
    artifact_route("importVideoFramePayload"),
    artifact_route("importVideoDone"),
    artifact_route("importVideoBytesPayload"),
    artifact_route("addStream"),
    artifact_route("removeStream"),
    artifact_route("setStreamSync"),
    artifact_route("editCalibration"),
    artifact_route("calibrateCameras"),
    artifact_route("addGcp"),
    artifact_route("removeGcp"),
    artifact_route("placeGcpObservation"),
    artifact_route("setIngestParams"),
    artifact_route("setFeatureParams"),
    artifact_route("setMatchParams"),
    artifact_route("setSfmParams"),
    artifact_route("setDenseParams"),
    artifact_route("setMeshParams"),
    artifact_route("setMotionParams"),
    artifact_route("setGeoParams"),
    artifact_route("resetPlaceholderMesh"),
    artifact_route("clearSparse"),
    artifact_route("clearDense"),
    artifact_route("clearMeshResult"),
    artifact_route("clearTracks"),
    artifact_route("clearGeoProducts"),
    artifact_route("clearResult"),
    config_route("setCamera"),
    config_route("setLayerVisibility"),
    config_route("setFrameCursor"),
    config_route("setReportTable"),
    host_route("importFrames"),
    host_route("importVideo"),
    host_route("exportQcReport"),
    artifact_route("advanceReconstruction"),
    artifact_route("cancelReconstruction"),
    artifact_route("setActiveExample"),
];

/// 📄️ One document-lane publication row.
const fn artifact_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] }
}

/// 👁️ One config-lane publication row.
const fn config_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] }
}

/// 🐚️ One shell-effect-only publication row.
const fn host_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] }
}

fn remodeling_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(REMODELING_RETAINED_RAW_BYTES, REMODELING_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 📏️ One bounded first step per retained route, admitted only while the WHOLE addressable document
/// (streams and their frames, embedded assets, ground control points and their observations, calibrated
/// cameras) plus this edit fit inside `REMODELING_RETAINED_WORK_ITEMS`. `streams.len()` alone would
/// understate a document whose single stream carries a thousand frames, so the frames are counted too.
fn remodeling_retained_extent(command: &RemodelingCommand, snapshot: &RemodelingSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !REMODELING_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let frames: usize = snapshot.streams.iter().map(|stream| stream.frames.len()).sum();
    let observations: usize = snapshot.gcps.iter().map(|gcp| gcp.observations.len()).sum();
    let items = snapshot.streams.len().checked_add(frames)?.checked_add(snapshot.assets.len())?.checked_add(snapshot.gcps.len())?.checked_add(observations)?.checked_add(snapshot.calibration.cameras.len())?.checked_add(1)?;
    (items <= REMODELING_RETAINED_WORK_ITEMS).then_some(1)
}

fn remodeling_retained_reduce(
    command: &RemodelingCommand,
    snapshot: &RemodelingSnapshot,
    config: &RemodelingConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RemodelingPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation, NoDraftMutation>, Fault> {
    if !REMODELING_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("remodeling.retained.route"), "the bounded Remodeling reducer rejects undeclared routes"));
    }
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

/// 🏭️ The app-owned retained command job factory — `factory_type:` in the proof block below binds this
/// exact Rust type to `EditorApp<RemodelingPlayApp>`, which is what turns a bare (and therefore
/// `interactive-job.missing-owned-reducer`) proof into an exact-owner proof.
struct RemodelingRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl RemodelingRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: REMODELING_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for RemodelingRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<RemodelingPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<RemodelingPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        REMODELING_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        remodeling_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > REMODELING_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded Remodeling command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for RemodelingRetainedCommandJobFactory {
    type Owner = EditorApp<RemodelingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = REMODELING_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = REMODELING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = REMODELING_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
/// 📬️ The document lane's one-item retained preparation. Without it every route declaring
/// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract and
/// stays dispatch-dead, no matter how it is classified.
struct RemodelingStorePreparationFactory;

struct RemodelingStorePreparation {
    base: Option<store::SnapshotRead<RemodelingSnapshot>>,
    mutation: Option<RemodelingMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<RemodelingSnapshot, RemodelingMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<RemodelingSnapshot, RemodelingMutation> for RemodelingStorePreparationFactory {
    fn preflight(&self, _mutation: &RemodelingMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Remodeling Store preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<RemodelingSnapshot, RemodelingMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<RemodelingSnapshot, RemodelingMutation>>, store::ArtifactStoreOneItemPreparationRequest<RemodelingSnapshot, RemodelingMutation>> {
        let scene = request.base.get();
        let frames: usize = scene.streams.iter().map(|stream| stream.frames.len()).sum();
        let item_count = scene.streams.len().saturating_add(frames).saturating_add(scene.assets.len()).saturating_add(scene.gcps.len());
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || item_count > REMODELING_RETAINED_WORK_ITEMS
        {
            return Err(request);
        }
        Ok(Box::new(RemodelingStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<RemodelingSnapshot, RemodelingMutation> for RemodelingStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Remodeling preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Remodeling preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Remodeling preparation lost its Store authority".to_string())?;
        let id = format!("remodeling-retained-{}", authority.next_sequence_number());
        let edit = remodeling_retained_edit(id, authority, vec![mutation], inverse, self.description.take());
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<RemodelingSnapshot, RemodelingMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<RemodelingSnapshot, RemodelingMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Remodeling preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

/// 📬️ The config lane's twin — remodeling's six session verbs (`setCamera`/`setLayerVisibility`/
/// the runtime rejects a `Config` publication contract outright when this factory is absent.
struct RemodelingConfigStorePreparationFactory;

struct RemodelingConfigStorePreparation {
    base: Option<store::SnapshotRead<RemodelingConfig>>,
    mutation: Option<RemodelingConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<RemodelingConfig, RemodelingConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<RemodelingConfig, RemodelingConfigMutation> for RemodelingConfigStorePreparationFactory {
    fn preflight(&self, _mutation: &RemodelingConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Remodeling config preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<RemodelingConfig, RemodelingConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<RemodelingConfig, RemodelingConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<RemodelingConfig, RemodelingConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(RemodelingConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<RemodelingConfig, RemodelingConfigMutation> for RemodelingConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Remodeling config preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Remodeling config preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Remodeling config preparation lost its Store authority".to_string())?;
        let id = format!("remodeling-config-retained-{}", authority.next_sequence_number());
        let edit = remodeling_retained_edit(id, authority, vec![mutation], inverse, self.description.take());
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<RemodelingConfig, RemodelingConfigMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<RemodelingConfig, RemodelingConfigMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Remodeling config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

/// 🧾️ The one `protocol::Edit` envelope both lanes above stage — identical field-for-field, so it is
/// written once and parameterised by the mutation type rather than duplicated per lane.
fn remodeling_retained_edit<M>(id: String, authority: &store::ArtifactStoreOneItemLiveAuthority, forwards: Vec<M>, inverse: Vec<M>, description: Option<String>) -> protocol::Edit<M> {
    let mutation_id = protocol::MutationId(format!("{id}#0"));
    protocol::Edit {
        id,
        actor: Some(authority.actor().to_string()),
        forwards,
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(mutation_id),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}
//#endregion 📬️StorePreparation

impl ArtifactEditor for RemodelingPlayApp {
    type Snapshot = RemodelingSnapshot;
    type Mutation = RemodelingMutation;
    type Config = RemodelingConfig;
    type ConfigMutation = RemodelingConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = RemodelingPresence;
    type PresenceMutation = RemodelingPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = RemodelingCommand;

    const DIALECT: Dialect = crate::REMODELING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = REMODELING_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(RemodelingStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(RemodelingConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<RemodelingPlayApp>,
        owner_file: "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.remodel.remodeling@1/*#editor",
        document_schema: "remodeling.scene",
        factory: "RemodelingRetainedCommandJobFactory",
        factory_type: RemodelingRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "runReconstruction", "retryStage", "runStage",
            "importFramePayload", "importVideoFramePayload", "importVideoDone", "importVideoBytesPayload",
            "addStream", "removeStream", "setStreamSync",
            "editCalibration", "calibrateCameras", "addGcp", "removeGcp", "placeGcpObservation",
            "setIngestParams", "setFeatureParams", "setMatchParams", "setSfmParams", "setDenseParams", "setMeshParams", "setMotionParams", "setGeoParams",
            "resetPlaceholderMesh", "clearSparse", "clearDense", "clearMeshResult", "clearTracks", "clearGeoProducts", "clearResult",
            "setCamera", "setLayerVisibility", "setFrameCursor", "setReportTable", "importFrames", "importVideo", "exportQcReport",
            "advanceReconstruction", "cancelReconstruction", "setActiveExample"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(RemodelingRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !REMODELING_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("remodeling.retained.tool-mismatch"), "Remodeling command does not match its exact registered tool"));
        }
        if remodeling_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("remodeling.retained.extent"), "Remodeling bounded route exceeded its declared work extent"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, remodeling_retained_reduce, remodeling_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            RemodelingCommand::command_id,
            REMODELING_RETAINED_RAW_BYTES,
            REMODELING_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::remodeling::config::schema::app_schema_descriptor())
    }

    /// 🚀️ Boots on the registered boot example when its committed text parses, falling back to the
    /// artifact's own `default_remodeling_scene()` otherwise — the same shape `🧱️block`'s
    /// `default_block2d_snapshot()` uses, so an unparsable example degrades instead of panicking.
    fn initial_snapshot() -> RemodelingSnapshot {
        crate::editor::remodeling::examples::boot_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(remodeling_io())
    }

    /// 🎞️ `mesh:out` (the current reconstructed mesh, GLB-encoded) plus the inherited `document:out`
    /// default (the pack of `doc.snapshot`, replicated inline — overriding `export_media` shadows the
    /// trait's provided body for every port, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, RemodelingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "mesh:out" => {
                // 🧩️ `results.mesh.mesh` is now a composed `s.stdio.semio/v1/mesh` CHILD handle
                // (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) — the real `MeshData` reads
                // through the working-scene cache, honestly `Err` on a cold cache (documented
                // staleness gap, matches every prior exemplar in this ticket) rather than exporting a
                // fabricated empty mesh.
                let mesh =
                    crate::resolve_bounded_remodeling_mesh(&doc.snapshot.durable_artifacts, &doc.snapshot.results.mesh.mesh).ok_or_else(|| MediaError::Payload(port.to_string(), "mesh:out: bounded composed mesh content is unavailable".into()))?;
                let bytes = MeshExporter::export(&GlbExporter, &mesh).map_err(|error| MediaError::Payload(port.to_string(), error))?;
                Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "3d.mesh".into(), json: base64_codec::base64_standard_encode(bytes) } })
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
    /// `REMODELING_WORKFLOW_PHOTOS_STREAM_ID` image-sequence stream (creating it on the first import).
    /// `document:in` stays `MediaError::NotImplemented`, unchanged from the inherited default: remodeling
    /// has no whole-document-replace `Mutation` variant to satisfy `whole_document_mutation`
    /// (`RemodelingMutation` is deliberately field-granular — see that enum's doc comment).
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RemodelingSnapshot>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "photos:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "photos:in only accepts a Structured base64-image payload".into()));
                };
                let bytes = base64_codec::base64_standard_decode(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let (width, height) = decode_still_image("image/png", &bytes).map_or((0, 0), |image| (image.width, image.height));
                let scene = doc.snapshot;
                let stream_id = REMODELING_WORKFLOW_PHOTOS_STREAM_ID;
                let frame_index = scene.streams.iter().find(|stream| stream.id == stream_id).map_or(0, |stream| stream.frames.len() as u32);
                let asset_key = format!("{stream_id}-frame-{frame_index}");
                let asset = ImageAsset { mime: "image/png".into(), data: json.clone(), width, height };
                let mut mutations = vec![crate::mutations::create_asset(asset_key.clone(), asset)];
                match scene.streams.iter().any(|stream| stream.id == stream_id) {
                    true => mutations.push(crate::mutations::add_stream_frame(stream_id.to_string(), FrameRef { index: frame_index, timestamp_ms: f64::from(frame_index) * 1000.0 / 30.0, asset_id: asset_key }, MediaKind::ImageSequence)),
                    false => mutations.push(crate::mutations::create_stream(MediaStream {
                        id: stream_id.to_string(),
                        name: "Workflow Photos".into(),
                        kind: MediaKind::ImageSequence,
                        camera_id: None,
                        sync_offset_ms: 0.0,
                        fps_hint: 30.0,
                        frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: asset_key }],
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
    fn command_id(command: &RemodelingCommand) -> &'static str {
        command.command_id()
    }

    /// 🌉️ Host action id + JSON args → typed command. **Forward-fix, not a preserved behaviour**: the
    /// pre-migration `remodeling_ui` never implemented this at all, so every one of remodeling's own actions
    /// (the media drop zone, the layer toggles, every command-palette entry) fell through to the
    /// framework's reserved-action error and could only be reached by a direct `dispatch_typed` —
    /// i.e. the whole manifest surface was dead from the host's side. Arg keys are the camelCase ids
    /// declared in `🔖️Manifest`; each is read leniently (missing → the manifest's own default) because
    /// `effective_action_args` stages defaults before dispatch and select-typed args arrive as strings.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<RemodelingCommand, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &RemodelingCommand,
        doc: &ArtifactView<'_, RemodelingSnapshot>,
        cfg: &ConfigView<'_, RemodelingConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RemodelingSnapshot>, cfg: &ConfigView<'_, RemodelingConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let scene = doc.snapshot;
        let config = cfg.snapshot;
        let labels = remodeling_labels(view_state);
        let built = match body_key {
            model::windows::model::REMODELING_PLAY_BODY_MAIN => model::windows::model::render(scene, config),
            capture::windows::frames::REMODELING_PLAY_BODY_FRAMES => capture::windows::frames::render(scene, config),
            analyze::windows::report::REMODELING_PLAY_BODY_REPORT => analyze::windows::report::render(scene, config),
            media::REMODELING_PLAY_BODY_MEDIA => media::render(scene, labels),
            document::REMODELING_PLAY_BODY_PIPELINE => document::render(scene, view_state.active_utility_id.as_deref().unwrap_or("select"), labels),
            results::REMODELING_PLAY_BODY_RESULTS => results::render(scene, labels),
            parameters::REMODELING_PLAY_BODY_PARAMETERS => parameters::render(scene, labels),
            calibration_panel::REMODELING_PLAY_BODY_CALIBRATION => calibration_panel::render(scene, labels),
            tracks::REMODELING_PLAY_BODY_TRACKS => tracks::render(scene, labels),
            quality::REMODELING_PLAY_BODY_QC => quality::render(scene, labels),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        };
        built.map(semio_framework_plugin::built_to_component_tree)
    }

    /// 👁️ Dynamic per-render window measures — the Model window's layer toggles must reflect the LIVE
    /// config, so they are supplied here rather than frozen into the manifest.
    fn window_measures(_doc: &ArtifactView<'_, RemodelingSnapshot>, cfg: &ConfigView<'_, RemodelingConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(model::windows::model::REMODELING_PLAY_WINDOW_MAIN.to_string(), model::windows::model::window_measures(cfg.snapshot, remodeling_labels(view_state)))])
    }
}
//#endregion 🔖️RemodelingPlayApp

//#region 🔖️Manifest

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/utility declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_remodeling_app() -> AppDefinition {
    Editor::builder(crate::REMODELING_DIALECT)
            .document(["semio", "remodeling"])
            .artifact_kind(crate::artifact_kind())
            // 🔌️ `photos:in`/`mesh:out` — `2d.image`/`3d.mesh` are declared by `shooting`/`lowpoly`
            // respectively (reused here, not redeclared).
            .media_input(remodeling_photos_in_port())
            .media_output(remodeling_mesh_out_port())
            .icon_id("remodeling-app")
            .mode_def(capture::definition())
            .mode_def(model::definition())
            .mode_def(analyze::definition())
            .default_mode_id(model::REMODELING_PLAY_MODE_MODEL)
            .window_kind_def(model::windows::model::definition())
            .window_kind_def(capture::windows::frames::definition())
            .window_kind_def(analyze::windows::report::definition())
            // 🕹️ The "assets" interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)
            // replaces the deleted `RemodelingSelection` config field — both windows that carry the "select"
            // utility (`remodeling-main`'s World3d viewport, `remodeling-frames`' Canvas2d frame view) declare it.
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
            .window_kind_interactions(model::windows::model::REMODELING_PLAY_WINDOW_MAIN, vec![InteractionRef::new("assets")])
            .window_kind_interactions(capture::windows::frames::REMODELING_PLAY_WINDOW_FRAMES, vec![InteractionRef::new("assets")])
            .default_layout(model::layout())
            .named_layout(capture::layout())
            .named_layout(analyze::layout())
            .mode_layout(capture::REMODELING_PLAY_MODE_CAPTURE, capture::REMODELING_PLAY_LAYOUT_CAPTURE)
            .mode_layout(analyze::REMODELING_PLAY_MODE_ANALYZE, analyze::REMODELING_PLAY_LAYOUT_ANALYZE)
            .panel_tab_def(document::definition())
            .panel_tab_def(media::definition())
            .panel_tab_def(results::definition())
            .panel_tab_def(parameters::definition())
            .panel_tab_def(calibration_panel::definition())
            .panel_tab_def(tracks::definition())
            .panel_tab_def(quality::definition())
            // 🚀️ Public starts plus the non-palette host continuation.
            .mutation("runReconstruction", LocalizedLabel::native("Run Reconstruction", "Rekonstruktion starten"))
            .mutation("cancelReconstruction", LocalizedLabel::native("Cancel Reconstruction", "Rekonstruktion abbrechen"))
            .mutation("retryStage", LocalizedLabel::native("Retry", "Wiederholen"))
            .mutation("runStage", LocalizedLabel::native("Run Stage", "Stufe ausführen"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(run_reconstruction::ADVANCE_RECONSTRUCTION_ACTION_ID, LocalizedLabel::native("Advance Reconstruction", "Rekonstruktion fortsetzen"), ActionKind::Mutation) })
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
            .default_value(&"extracting-features")])
            .action_args("retryStage", vec![ActionArgDef::select(
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
            .default_value(&"extracting-features")])
            // 📥️ Ingestion.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new("importFrames", LocalizedLabel::native("Import Frames", "Frames importieren"), ActionKind::Shell, "hard-drive") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importFramePayload", LocalizedLabel::native("Import Frame Payload", "Bild-Payload importieren"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new("importVideo", LocalizedLabel::native("Import Video", "Video importieren"), ActionKind::Shell, "hard-drive") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importVideoFramePayload", LocalizedLabel::native("Import Video Frame Payload", "Video-Frame-Payload importieren"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importVideoDone", LocalizedLabel::native("Import Video Done", "Video-Import abgeschlossen"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importVideoBytesPayload", LocalizedLabel::native("Import Video Bytes Payload", "Video-Byte-Payload importieren"), ActionKind::Mutation) })
            .mutation("addStream", LocalizedLabel::native("Add Stream", "Stream hinzufügen"))
            .action_args("addStream", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value(&"Stream"),
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![ActionArgOption::new("image-sequence", LocalizedLabel::native("Image Sequence", "Bildsequenz")), ActionArgOption::new("video", LocalizedLabel::native("Video", "Video"))]).default_value(&"image-sequence"),
                ActionArgDef::text("cameraId", LocalizedLabel::native("Camera Id", "Kamera-Id")).default_value(&"cam-0"),
            ])
            .mutation("removeStream", LocalizedLabel::native("Remove Stream", "Stream entfernen"))
            .action_args("removeStream", vec![ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required()])
            .mutation("setStreamSync", LocalizedLabel::native("Set Stream Sync", "Stream-Synchronisation festlegen"))
            .action_args("setStreamSync", vec![ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required(), ActionArgDef::number("syncOffsetMs", LocalizedLabel::native("Sync Offset (ms)", "Sync-Versatz (ms)")).default_value(&0)])
            // 🎯️ Calibration / GCPs.
            .mutation("editCalibration", LocalizedLabel::native("Edit Calibration", "Kalibrierung bearbeiten"))
            .action_args("editCalibration", vec![
                ActionArgDef::text("cameraId", LocalizedLabel::native("Camera Id", "Kamera-Id")).required(),
                ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")),
                ActionArgDef::select("model", LocalizedLabel::native("Model", "Modell"), vec![ActionArgOption::new("pinhole", LocalizedLabel::native("Pinhole", "Lochkamera")), ActionArgOption::new("brownConrady", LocalizedLabel::native("Brown-Conrady", "Brown-Conrady")), ActionArgOption::new("fisheye", LocalizedLabel::native("Fisheye", "Fischauge"))]).default_value(&"pinhole"),
                ActionArgDef::number("fx", LocalizedLabel::native("fx", "fx")).default_value(&1000),
                ActionArgDef::number("fy", LocalizedLabel::native("fy", "fy")).default_value(&1000),
                ActionArgDef::number("cx", LocalizedLabel::native("cx", "cx")).default_value(&0),
                ActionArgDef::number("cy", LocalizedLabel::native("cy", "cy")).default_value(&0),
                ActionArgDef::number("skew", LocalizedLabel::native("Skew", "Scherung")).default_value(&0),
                ActionArgDef::number("k1", LocalizedLabel::native("k1", "k1")).default_value(&0),
                ActionArgDef::number("k2", LocalizedLabel::native("k2", "k2")).default_value(&0),
                ActionArgDef::number("k3", LocalizedLabel::native("k3", "k3")).default_value(&0),
                ActionArgDef::number("p1", LocalizedLabel::native("p1", "p1")).default_value(&0),
                ActionArgDef::number("p2", LocalizedLabel::native("p2", "p2")).default_value(&0),
                ActionArgDef::toggle("locked", LocalizedLabel::native("Locked", "Gesperrt")).default_value(&false),
            ])
            .mutation("calibrateCameras", LocalizedLabel::native("Calibrate Cameras", "Kameras kalibrieren"))
            .mutation("addGcp", LocalizedLabel::native("Add Ground Control Point", "Passpunkt hinzufügen"))
            .action_args("addGcp", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value(&"GCP"),
                ActionArgDef::number("worldX", LocalizedLabel::native("World X", "Welt X")).default_value(&0),
                ActionArgDef::number("worldY", LocalizedLabel::native("World Y", "Welt Y")).default_value(&0),
                ActionArgDef::number("worldZ", LocalizedLabel::native("World Z", "Welt Z")).default_value(&0),
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
                ActionArgDef::number("frameSampleStride", LocalizedLabel::native("Frame Sample Stride", "Frame-Abtastschrittweite")).default_value(&5),
                ActionArgDef::number("maxFrames", LocalizedLabel::native("Max Frames", "Max. Frames")).default_value(&200),
                ActionArgDef::number("downscaleLongEdgePx", LocalizedLabel::native("Downscale Long Edge (px)", "Verkleinerung lange Kante (px)")).default_value(&1600),
                ActionArgDef::slider("minSharpness", LocalizedLabel::native("Min Sharpness", "Min. Schärfe"), 0.0, 1.0).default_value(&0.3),
            ])
            .mutation("setFeatureParams", LocalizedLabel::native("Set Feature Params", "Feature-Parameter festlegen"))
            .action_args("setFeatureParams", vec![
                ActionArgDef::select("detector", LocalizedLabel::native("Detector", "Detektor"), vec![ActionArgOption::new("orb", LocalizedLabel::native("ORB", "ORB")), ActionArgOption::new("akaze", LocalizedLabel::native("AKAZE", "AKAZE")), ActionArgOption::new("harris", LocalizedLabel::native("Harris", "Harris"))]).default_value(&"orb"),
                ActionArgDef::number("targetCount", LocalizedLabel::native("Target Count", "Ziel-Anzahl")).default_value(&4000),
                ActionArgDef::number("octaves", LocalizedLabel::native("Octaves", "Oktaven")).default_value(&4),
                ActionArgDef::slider("edgeThreshold", LocalizedLabel::native("Edge Threshold", "Kanten-Schwelle"), 1.0, 50.0).default_value(&10.0),
            ])
            .mutation("setMatchParams", LocalizedLabel::native("Set Match Params", "Match-Parameter festlegen"))
            .action_args("setMatchParams", vec![
                ActionArgDef::select("matcher", LocalizedLabel::native("Matcher", "Matcher"), vec![ActionArgOption::new("brute-force", LocalizedLabel::native("Brute Force", "Brute Force")), ActionArgOption::new("kd-tree", LocalizedLabel::native("KD-Tree", "KD-Baum"))]).default_value(&"brute-force"),
                ActionArgDef::slider("ratioTest", LocalizedLabel::native("Ratio Test", "Verhältnistest"), 0.1, 1.0).default_value(&0.8),
                ActionArgDef::toggle("crossCheck", LocalizedLabel::native("Cross Check", "Kreuzprüfung")).default_value(&true),
                ActionArgDef::number("sequentialWindow", LocalizedLabel::native("Sequential Window", "Sequenzielles Fenster")).default_value(&8),
                ActionArgDef::number("maxPairsPerFrame", LocalizedLabel::native("Max Pairs Per Frame", "Max. Paare pro Frame")).default_value(&16),
                ActionArgDef::toggle("loopClosure", LocalizedLabel::native("Loop Closure", "Schleifenschluss")).default_value(&true),
            ])
            .mutation("setSfmParams", LocalizedLabel::native("Set SfM Params", "SfM-Parameter festlegen"))
            .action_args("setSfmParams", vec![
                ActionArgDef::number("ransacIterations", LocalizedLabel::native("RANSAC Iterations", "RANSAC-Iterationen")).default_value(&1000),
                ActionArgDef::slider("ransacThresholdPx", LocalizedLabel::native("RANSAC Threshold (px)", "RANSAC-Schwelle (px)"), 0.1, 10.0).default_value(&2.0),
                ActionArgDef::number("minTrackLength", LocalizedLabel::native("Min Track Length", "Min. Spurlänge")).default_value(&3),
                ActionArgDef::number("baMaxIterations", LocalizedLabel::native("BA Max Iterations", "BA Max. Iterationen")).default_value(&50),
                ActionArgDef::select("robustLoss", LocalizedLabel::native("Robust Loss", "Robuster Verlust"), vec![ActionArgOption::new("l2", LocalizedLabel::native("L2", "L2")), ActionArgOption::new("huber", LocalizedLabel::native("Huber", "Huber")), ActionArgOption::new("cauchy", LocalizedLabel::native("Cauchy", "Cauchy"))]).default_value(&"huber"),
                ActionArgDef::slider("huberDeltaPx", LocalizedLabel::native("Huber Delta (px)", "Huber-Delta (px)"), 0.1, 10.0).default_value(&1.5),
            ])
            .mutation("setDenseParams", LocalizedLabel::native("Set Dense Params", "Dense-Parameter festlegen"))
            .action_args("setDenseParams", vec![
                ActionArgDef::select("resolution", LocalizedLabel::native("Resolution", "Auflösung"), vec![ActionArgOption::new("low", LocalizedLabel::native("Low", "Niedrig")), ActionArgOption::new("medium", LocalizedLabel::native("Medium", "Mittel")), ActionArgOption::new("high", LocalizedLabel::native("High", "Hoch"))]).default_value(&"medium"),
                ActionArgDef::number("windowRadiusPx", LocalizedLabel::native("Window Radius (px)", "Fensterradius (px)")).default_value(&3),
                ActionArgDef::number("minViewConsistency", LocalizedLabel::native("Min View Consistency", "Min. Ansichtskonsistenz")).default_value(&3),
                ActionArgDef::slider("confidenceThreshold", LocalizedLabel::native("Confidence Threshold", "Konfidenzschwelle"), 0.0, 1.0).default_value(&0.5),
                ActionArgDef::number("maxPoints", LocalizedLabel::native("Max Points", "Max. Punkte")).default_value(&500_000),
            ])
            .mutation("setMeshParams", LocalizedLabel::native("Set Mesh Params", "Mesh-Parameter festlegen"))
            .action_args("setMeshParams", vec![
                ActionArgDef::slider("tsdfVoxelSizeMm", LocalizedLabel::native("TSDF Voxel Size (mm)", "TSDF-Voxelgröße (mm)"), 1.0, 20.0).default_value(&5.0),
                ActionArgDef::slider("tsdfTruncationMm", LocalizedLabel::native("TSDF Truncation (mm)", "TSDF-Trunkierung (mm)"), 2.0, 60.0).default_value(&20.0),
                ActionArgDef::number("decimateTargetTriangles", LocalizedLabel::native("Decimate Target Triangles", "Ziel-Dreiecke (Dezimierung)")).default_value(&200_000),
                ActionArgDef::number("smoothingIterations", LocalizedLabel::native("Smoothing Iterations", "Glättungs-Iterationen")).default_value(&2),
                ActionArgDef::toggle("textureEnabled", LocalizedLabel::native("Texture Enabled", "Textur aktiviert")).default_value(&true),
                ActionArgDef::select("textureSize", LocalizedLabel::native("Texture Size", "Texturgröße"), vec![ActionArgOption::new("1024", LocalizedLabel::native("1024", "1024")), ActionArgOption::new("2048", LocalizedLabel::native("2048", "2048")), ActionArgOption::new("4096", LocalizedLabel::native("4096", "4096"))]).default_value(&"2048"),
                ActionArgDef::toggle("guaranteeWatertight", LocalizedLabel::native("Guarantee Watertight", "Wasserdichtheit garantieren")).default_value(&true),
                ActionArgDef::number("holeFillMaxBoundaryVerts", LocalizedLabel::native("Hole Fill Max Boundary Verts", "Max. Randpunkte für Lochfüllung")).default_value(&512),
                ActionArgDef::toggle("selfIntersectionCheck", LocalizedLabel::native("Self-Intersection Check", "Selbstüberschneidungsprüfung")).default_value(&false),
            ])
            .mutation("setMotionParams", LocalizedLabel::native("Set Motion Params", "Bewegungs-Parameter festlegen"))
            .action_args("setMotionParams", vec![
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).default_value(&false),
                ActionArgDef::number("maxTracks", LocalizedLabel::native("Max Tracks", "Max. Spuren")).default_value(&64),
                ActionArgDef::number("trackWindowPx", LocalizedLabel::native("Track Window (px)", "Spurfenster (px)")).default_value(&21),
                ActionArgDef::slider("minTrackQuality", LocalizedLabel::native("Min Track Quality", "Min. Spurqualität"), 0.0, 1.0).default_value(&0.3),
                ActionArgDef::number("minTrackLengthFrames", LocalizedLabel::native("Min Track Length (frames)", "Min. Spurlänge (Frames)")).default_value(&5),
            ])
            .mutation("setGeoParams", LocalizedLabel::native("Set Geo Params", "Geo-Parameter festlegen"))
            .action_args("setGeoParams", vec![
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).default_value(&false),
                ActionArgDef::number("originLon", LocalizedLabel::native("Origin Longitude", "Ursprung Längengrad")).default_value(&0),
                ActionArgDef::number("originLat", LocalizedLabel::native("Origin Latitude", "Ursprung Breitengrad")).default_value(&0),
                ActionArgDef::number("originAlt", LocalizedLabel::native("Origin Altitude", "Ursprung Höhe")).default_value(&0),
                ActionArgDef::slider("gsdM", LocalizedLabel::native("Ground Sample Distance (m)", "Bodenauflösung (m)"), 0.01, 1.0).default_value(&0.05),
                ActionArgDef::slider("dsmCellM", LocalizedLabel::native("DSM Cell Size (m)", "DOM-Zellgröße (m)"), 0.01, 5.0).default_value(&0.1),
                ActionArgDef::slider("dtmFilterRadiusM", LocalizedLabel::native("DTM Filter Radius (m)", "DGM-Filterradius (m)"), 0.1, 10.0).default_value(&2.0),
                ActionArgDef::number("orthoMaxPx", LocalizedLabel::native("Ortho Max (px)", "Ortho Max. (px)")).default_value(&4096),
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
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setLayerVisibility", LocalizedLabel::native("Set Layer Visibility", "Ebenensichtbarkeit festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setFrameCursor", LocalizedLabel::native("Set Frame Cursor", "Frame-Cursor festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setReportTable", LocalizedLabel::native("Set Report Table", "Berichtstabelle festlegen"), ActionKind::View) })
            // 📤️ Export.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::bounded_catalog("exportQcReport", LocalizedLabel::native("Export QC Report", "QC-Bericht exportieren"), ActionKind::Shell) })
            // 🧰️ Utility groups — an exclusive per-window set (active utility is host-owned); which
            // window exposes which is declared by that window's own `definition()`.
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer-2") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("sculpt", LocalizedLabel::native("Sculpt", "Formen"), "paintbrush") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("measure", LocalizedLabel::native("Measure", "Messen"), "scaling") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("gcpPlace", LocalizedLabel::native("Place GCP", "Passpunkt setzen"), "crosshair") })
            // 👁️ Internal (non-palette) view actions the panels/windows dispatch — declared so they are
            // real `ActionDefinition`s the classification gate and the tool-proof catalog can both see.
            // 🎬️ Example picker — one option per `crate::editor::remodeling::examples::REMODELING_EXAMPLES`
            // entry, so appending an example there is the only edit a new example needs.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new("setActiveExample", LocalizedLabel::native("Load Example", "Beispiel laden"), ActionKind::Mutation, "panel-left") })
            .action_args("setActiveExample", vec![ActionArgDef::select(
                "exampleId",
                LocalizedLabel::native("Example", "Beispiel"),
                crate::editor::remodeling::examples::REMODELING_EXAMPLES.iter().map(|example| ActionArgOption::new(example.id, crate::editor::remodeling::examples::example_label(example))).collect(),
            )
            .default_value(&crate::editor::remodeling::examples::REMODELING_EXAMPLE_BOOT_ID)])
            // 🧵️ Phase-8 dispositions. Every declared row is `Migrated`: each is backed by the
            // exact-owner `RemodelingRetainedCommandJobFactory` proof in `🧵️RetainedCommands` and by the
            // document/config one-item store preparations in `📬️StorePreparation`, so UI dispatch (which
            // rejects anything that is not `Migrated`) and the release-blocking
            // `validate_interactive_job_classification` gate both admit it. The framework demands set
            // EQUALITY here: `validate_tool_job_rows` computes `expected = TOOL_JOB_IDS ∩ migrated` and
            // faults `interactive-job.catalog-incomplete` unless it equals the proof set.
            //
            .action_interactive_job("runReconstruction", InteractiveJobClassification::Migrated)
            .action_interactive_job("retryStage", InteractiveJobClassification::Migrated)
            .action_interactive_job("runStage", InteractiveJobClassification::Migrated)
            .action_interactive_job("advanceReconstruction", InteractiveJobClassification::Migrated)
            .action_interactive_job("cancelReconstruction", InteractiveJobClassification::Migrated)
            .action_interactive_job("importFrames", InteractiveJobClassification::Migrated)
            .action_interactive_job("importFramePayload", InteractiveJobClassification::Migrated)
            .action_interactive_job("importVideo", InteractiveJobClassification::Migrated)
            .action_interactive_job("importVideoFramePayload", InteractiveJobClassification::Migrated)
            .action_interactive_job("importVideoDone", InteractiveJobClassification::Migrated)
            .action_interactive_job("importVideoBytesPayload", InteractiveJobClassification::Migrated)
            .action_interactive_job("addStream", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeStream", InteractiveJobClassification::Migrated)
            .action_interactive_job("setStreamSync", InteractiveJobClassification::Migrated)
            .action_interactive_job("editCalibration", InteractiveJobClassification::Migrated)
            .action_interactive_job("calibrateCameras", InteractiveJobClassification::Migrated)
            .action_interactive_job("addGcp", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeGcp", InteractiveJobClassification::Migrated)
            .action_interactive_job("placeGcpObservation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setIngestParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFeatureParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setMatchParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSfmParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setDenseParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setMeshParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setMotionParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGeoParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("resetPlaceholderMesh", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearSparse", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearDense", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearMeshResult", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearTracks", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearGeoProducts", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearResult", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLayerVisibility", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFrameCursor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setReportTable", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportQcReport", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            // 🎯️ Typed channel surface — `io()` is this same information's single source of truth,
            // reused here rather than duplicated.
            .io(remodeling_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old default-example
            // registration (`default_remodeling_scene().print_dsl()` fed to `.example("default", …)`)
            // and the no-op `.workflow("remodeling", …)` call are dropped here (not silently: reported
            // in this packet's migration notes). The subset's own `📚️examples/🎬️demo` facet
            // (`crate::examples::…`, real content, pre-existing) is the modern,
            // role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
