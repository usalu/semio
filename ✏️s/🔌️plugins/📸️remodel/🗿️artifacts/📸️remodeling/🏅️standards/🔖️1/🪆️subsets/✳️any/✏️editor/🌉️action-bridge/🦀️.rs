//! 🌉️ Action bridge of the remodeling editor: the typed argument decoding that turns a manifest action invocation into a remodeling command. See [`super`].

use super::*;

//#region 🔖️ActionBridge
/// 🌉️ Host-args → typed-payload readers plus the action-id match — see `ArtifactEditor::command_from_action`
/// for why this exists (it did not before this migration). Kept in its own module so the routing table
/// above stays a routing table.
pub(super) mod args_bridge {
    use super::*;

    fn field<'a>(args: Option<&'a semio_framework_value::DslValue>, key: &str) -> Option<&'a semio_framework_value::DslValue> {
        args?.get(key)
    }

    /// 🔤️ A string arg — also accepts a number, so a select whose option ids are numeric (`textureSize`)
    /// reads the same whether the host sends `"2048"` or `2048`.
    fn text(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<String> {
        let value = field(args, key)?;
        value.as_str().map(str::to_owned).or_else(|| value.as_u64().map(|number| number.to_string())).or_else(|| value.as_i64().map(|number| number.to_string())).or_else(|| value.as_f64().map(|number| number.to_string()))
    }

    /// 🔢️ A numeric arg — also accepts a numeric string, which is how select-sourced numbers arrive.
    fn number(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<f64> {
        let value = field(args, key)?;
        value.as_f64().or_else(|| value.as_str()?.parse().ok())
    }

    fn flag(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<bool> {
        let value = field(args, key)?;
        value.as_bool().or_else(|| value.as_str()?.parse().ok())
    }

    fn vec3(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<[f64; 3]> {
        let items = field(args, key)?.as_array()?;
        Some([items.first()?.as_f64()?, items.get(1)?.as_f64()?, items.get(2)?.as_f64()?])
    }

    fn unknown(action: &str) -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("remodeling has no command for action '{action}'"))
    }

    #[allow(clippy::too_many_lines)]
    pub fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<RemodelingCommand, Fault> {
        let text_or = |key: &str, fallback: &str| text(args, key).unwrap_or_else(|| fallback.to_string());
        let f64_or = |key: &str, fallback: f64| number(args, key).unwrap_or(fallback);
        let f32_or = |key: &str, fallback: f32| number(args, key).map_or(fallback, |value| value as f32);
        let u32_or = |key: &str, fallback: u32| number(args, key).map_or(fallback, |value| value as u32);
        let bool_or = |key: &str, fallback: bool| flag(args, key).unwrap_or(fallback);
        Ok(match action {
            "importFramePayload" => RemodelingCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload: text_or("payload", ""), name: text_or("name", ""), index: u32_or("index", 0), total: u32_or("total", u32_or("index", 0).saturating_add(1)) }),
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
            "addStream" => RemodelingCommand::AddStream(add_stream::AddStream { name: text_or("name", "Stream"), kind: text_or("kind", "image-sequence"), camera_id: text_or("cameraId", "") }),
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
            "setCamera" => {
                let nested = field(args, "camera");
                let source = if nested.is_some() { nested } else { args };
                let default = model::windows::model::config::RemodelingModelWindowConfig::default().camera;
                RemodelingCommand::SetCamera(set_camera::SetCamera {
                    camera: store::Viewport3dOrbit {
                        position: vec3(source, "position").unwrap_or(default.position),
                        target: vec3(source, "target").unwrap_or(default.target),
                        zoom: number(source, "zoom").unwrap_or(default.zoom),
                        up: vec3(source, "up").or(default.up),
                    },
                })
            }
            "setLayerVisibility" => RemodelingCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: text_or("layer", ""), visible: bool_or("visible", true) }),
            "setFrameCursor" => RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: text(args, "streamId"), frame_index: u32_or("frameIndex", 0) }),
            "setReportTable" => RemodelingCommand::SetReportTable(set_report_table::SetReportTable { table: text_or("table", "frames") }),
            "importFrames" => RemodelingCommand::ImportFrames(import_frames::ImportFrames {}),
            "importVideo" => RemodelingCommand::ImportVideo(import_video::ImportVideo {}),
            "exportQcReport" => RemodelingCommand::ExportQcReport(export_qc_report::ExportQcReport {}),
            "setActiveExample" => RemodelingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text_or("exampleId", semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::examples::REMODELING_EXAMPLE_BOOT_ID) }),
            "importAbort" => RemodelingCommand::ImportAbort(import_abort::ImportAbort { reason: None }),
            _ => return Err(unknown(action)),
        })
    }
}
//#endregion 🔖️ActionBridge
