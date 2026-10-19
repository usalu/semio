//! 🔌️ Media I/O and commands of the remodeling editor: the workflow photo stream id, the typed photo and mesh ports and the aggregated command enum with its import routing. See [`super`].

use super::*;

//#region 🔖️Constants
/// 🔌️ Well-known stream id every `photos:in` import lands on — a stable identity so successive workflow
/// imports keep appending frames to the SAME stream (a pure `import_media` call has no runtime scratch
/// to remember which stream the last call used, unlike a UI drag-drop batch's `index == 0`/`> 0`
/// convention — see `🎮️commands/🖼️import-frame-payload` for that one).
pub(super) const REMODELING_WORKFLOW_PHOTOS_STREAM_ID: &str = "workflow-photos";
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
        artifact_schema: "remodeling.scene".into(),
        artifact_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
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

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `RemodelingPlayApp::Command` — the SOLE dispatch surface for remodeling's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies:
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum RemodelingCommand for RemodelingSnapshot, RemodelingMutation, NoConfig, NoConfigMutation {
        "importFramePayload" as "import-frame-payload" => import_frame_payload::ImportFramePayload,
        "importVideoFramePayload" as "import-video-frame-payload" => import_video_frame_payload::ImportVideoFramePayload,
        "importVideoDone" as "import-video-done" => import_video_done::ImportVideoDone,
        "importVideoBytesPayload" as "import-video-bytes-payload" => import_video_bytes_payload::ImportVideoBytesPayload,
        "addStream" as "add-stream" => add_stream::AddStream,
        "removeStream" as "remove-stream" => remove_stream::RemoveStream,
        "setStreamSync" as "stream-sync" => set_stream_sync::SetStreamSync,
        "editCalibration" as "edit-calibration" => edit_calibration::EditCalibration,
        "calibrateCameras" as "calibrate-cameras" => calibrate_cameras::CalibrateCameras,
        "addGcp" as "add-gcp" => add_gcp::AddGcp,
        "removeGcp" as "remove-gcp" => remove_gcp::RemoveGcp,
        "placeGcpObservation" as "place-gcp-observation" => place_gcp_observation::PlaceGcpObservation,
        "setIngestParams" as "ingest-params" => set_ingest_params::SetIngestParams,
        "setFeatureParams" as "feature-params" => set_feature_params::SetFeatureParams,
        "setMatchParams" as "match-params" => set_match_params::SetMatchParams,
        "setSfmParams" as "sfm-params" => set_sfm_params::SetSfmParams,
        "setDenseParams" as "dense-params" => set_dense_params::SetDenseParams,
        "setMeshParams" as "mesh-params" => set_mesh_params::SetMeshParams,
        "setMotionParams" as "motion-params" => set_motion_params::SetMotionParams,
        "setGeoParams" as "geo-params" => set_geo_params::SetGeoParams,
        "resetPlaceholderMesh" as "reset-placeholder-mesh" => reset_placeholder_mesh::ResetPlaceholderMesh,
        "clearSparse" as "clear-sparse" => clear_sparse::ClearSparse,
        "clearDense" as "clear-dense" => clear_dense::ClearDense,
        "clearMeshResult" as "clear-mesh-result" => clear_mesh_result::ClearMeshResult,
        "clearTracks" as "clear-tracks" => clear_tracks::ClearTracks,
        "clearGeoProducts" as "clear-geo-products" => clear_geo_products::ClearGeoProducts,
        "clearResult" as "clear-result" => clear_result::ClearResult,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setLayerVisibility" as "layer-visibility" => set_layer_visibility::SetLayerVisibility,
        "setFrameCursor" as "frame-cursor" => set_frame_cursor::SetFrameCursor,
        "setReportTable" as "report-table" => set_report_table::SetReportTable,
        "importFrames" as "import-frames" => import_frames::ImportFrames,
        "importVideo" as "import-video" => import_video::ImportVideo,
        "exportQcReport" as "export-qc-report" => export_qc_report::ExportQcReport,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "importAbort" as "import-abort" => import_abort::ImportAbort,
    }
}

impl RemodelingCommand {
    /// 📥️ An import verb through the dispatching window's tool state (`🫧️transient`): its emission and the window's next
    /// partition; `None` for every other verb.
    pub(super) fn import_in_window(&self, doc: &ArtifactView<'_, RemodelingSnapshot>, cfg: &ConfigView<'_, NoConfig>, window: &RemodelingWindowTransient) -> Option<Result<(Emit<RemodelingMutation, NoConfigMutation>, RemodelingWindowTransient), Fault>> {
        match self {
            Self::ImportFramePayload(payload) => Some(import_frame_payload::handle_in_window(payload, doc, cfg, window)),
            Self::ImportVideoFramePayload(payload) => Some(import_video_frame_payload::handle_in_window(payload, doc, window)),
            Self::ImportVideoDone(payload) => Some(import_video_done::handle_in_window(payload, doc, window)),
            Self::ImportAbort(payload) => Some(import_abort::handle_in_window(payload, doc, window)),
            _ => None,
        }
    }
}
//#endregion 🔖️Commands
