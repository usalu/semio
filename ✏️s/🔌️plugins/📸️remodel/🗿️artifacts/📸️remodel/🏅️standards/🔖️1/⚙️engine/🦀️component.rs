//! ⚙️ Remodel artifact — headless compute. Pure translation between `RemodelSnapshot`'s document types
//! and the ten sibling photogrammetry topic files (`images`/`video`/`camera`/`feature`/`sfm`/`dense`/
//! `mesh`/`motion`/`geo`/`reconstruction`), plus the mesh/raster export encoders and this artifact's
//! host registration.
//!
//! 🧭️ Placement rule (TEMPLATE §4): document-side compute lives here; anything taking an app-only
//! view-state type (`RemodelConfig`) stays at app level, because artifacts must never depend on apps.

use crate::artifacts::remodel::engine::{camera as remodel_camera, geo as remodel_geo, images as remodel_image, mesh as remodel_mesh, reconstruction as remodel_engine, sfm as remodel_sfm, video as remodel_video};
use crate::artifacts::remodel::{
    CameraPosePreview, DenseResolution, ImageAsset, QcReportSnapshot, ReconstructionParams, ReconstructionStage, RemodelSnapshot, RobustLossKind, VideoCodec as DocumentVideoCodec, WatertightReportSnapshot, REMODEL_DOCUMENT_SCHEMA,
};
use base64::Engine as _;
use semio_framework_plugin::{MeshData, MeshExporter, MediaFormat};
use serde_json::Value;

//#region 🔖️Register
/// 🗂️ Host registration for this artifact — the pack<->dsl document codec (keyed by the real
/// `document_schema()` string so `framework/sync`'s `FolderEndpoint::Pack` can print/parse remodel
/// documents without reaching for concrete `Projection`/`Mutation` types), every mesh exporter, the
/// DWG mesh handler and the PNG raster export. Moved verbatim out of the old bundle crate's
/// `register_remodel_exports`; `semio_plugin!`'s `setup:` now points here.
pub fn register() {
    crate::artifacts::remodel::composer::register();

    register_artifact_schema();
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::remodel::RemodelPlayApp>(REMODEL_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "remodel.document",
        extension: Some("remodel"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::remodel::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::remodel::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::remodel::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::remodel::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("remodel.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "remodel.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::remodel::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::remodel::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::remodel::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::remodel::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("remodel.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "remodel.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::remodel::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::remodel::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("remodel.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "remodel.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::remodel::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::remodel::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("remodel.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "remodel.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::remodel::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::remodel::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("remodel.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Ids

/// 🔢️ Replaces every former `RemodelPlayRuntime` id counter (`stream_counter`/`job_counter`/
/// `gcp_counter`/`import_counter`) — mirrors `shooting_engine::next_shooting_id`'s precedent (a plain
/// global monotonic counter, not VCS-tracked config state: uniqueness is all id generation needs, and
/// the generated id itself becomes real, undoable document content the moment an operation stores it).
pub fn next_remodel_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let next = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{prefix}-{next}")
}
//#endregion 🔖️Ids

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_remodel_app` already declares via `.artifact_kind(...)`, plus the two Wave-2 port-recipe ports:
/// `photos:in` (incoming source images for reconstruction) and `mesh:out` (the current reconstructed mesh).
pub fn remodel_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: "remodel.scene".into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        ports: vec![remodel_photos_in_port(), remodel_mesh_out_port()],
        export_formats: vec![MediaFormat::Glb, MediaFormat::Obj, MediaFormat::Stl, MediaFormat::Ply, MediaFormat::Las, MediaFormat::Png],
        import_formats: vec![MediaFormat::Glb, MediaFormat::Obj],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.remodel".into(), name: "3D Remodel".into(), dimension: "3d".into(), component_kind: "remodel".into() },
    }
}

/// 🔌️ `photos:in` — incoming photos to insert as source images for reconstruction; pinned to the
/// `2d.image` kind (declared by `shooting`'s manifest — identical-shape duplicates are harmless, so this
/// app does not redeclare it).
pub fn remodel_photos_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "photos:in".into(),
        label: "Photos".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🔌️ `mesh:out` — the current reconstructed mesh; pinned to the `3d.mesh` kind (declared by `lowpoly`'s
/// manifest — reused rather than redeclared, per the port recipe).
pub fn remodel_mesh_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "mesh:out".into(),
        label: "Mesh".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        kind_id: Some("3d.mesh".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}
//#endregion 🔖️Io

//#region 🔖️Payloads
/// 📦️ Decodes a `requestFileOpen(readAs: "dataUrl")`/`RequestMediaFrames` payload into `(mime, bytes)`.
pub fn payload_from_data_url(data_url: &str) -> Option<(String, Vec<u8>)> {
    let (header, encoded) = data_url.split_once(',')?;
    let mime = header.strip_prefix("data:")?.split(';').next().unwrap_or("application/octet-stream").to_string();
    let bytes = base64::engine::general_purpose::STANDARD.decode(encoded).ok()?;
    Some((mime, bytes))
}

/// 🖼️ Decodes a still-image payload by mime — three consumers (`🎮️commands/📥️ingest`,
/// `🎮️commands/🚀️reconstruction`, the app's `import_media`), and it takes no app-only type, so it
/// belongs here rather than at app level (TEMPLATE §4's DocumentHelpers rule).
pub fn decode_still_image(mime: &str, bytes: &[u8]) -> Result<remodel_image::ImageRgba8, remodel_image::ImageError> {
    if mime.contains("jpeg") || mime.contains("jpg") {
        remodel_image::decode_jpeg(bytes)
    } else {
        remodel_image::decode_png(bytes)
    }
}
//#endregion 🔖️Payloads

//#region 🔖️EngineMapping
/// ⚙️ Builds `remodel_engine::EngineParams` from the document's 8 param sub-structs. Fields with no
/// engine-side counterpart (`SfmParams::ransac_iterations` — the LO-RANSAC solver doesn't expose an
/// iteration cap; `GeoParams::origin_*` — no georeferencing-origin knob exists on the engine side;
/// `EngineParams::assumed_focal_ratio` — no document field feeds it, calibration is not yet a wired
/// stage) are documented simplifications, not oversights.
pub fn build_engine_params(params: &ReconstructionParams) -> remodel_engine::EngineParams {
    let mut engine_params = remodel_engine::EngineParams::default();
    engine_params.ingest.stride = params.ingest.frame_sample_stride.max(1);
    engine_params.ingest.max_frames = params.ingest.max_frames;
    engine_params.ingest.min_sharpness = params.ingest.min_sharpness;
    engine_params.target_feature_count = params.feature.target_count as usize;
    engine_params.match_ratio = params.matching.ratio_test;
    engine_params.match_mutual = params.matching.cross_check;
    engine_params.sequential_window = params.matching.sequential_window.max(1) as usize;
    engine_params.sfm.ransac_threshold_px = f64::from(params.sfm.ransac_threshold_px);
    engine_params.sfm.min_track_length = params.sfm.min_track_length as usize;
    engine_params.sfm.ba_max_iterations = params.sfm.ba_max_iterations as usize;
    engine_params.sfm.robust_loss = match params.sfm.robust_loss {
        RobustLossKind::L2 => remodel_sfm::RobustLoss::Trivial,
        RobustLossKind::Huber => remodel_sfm::RobustLoss::Huber(f64::from(params.sfm.huber_delta_px)),
        RobustLossKind::Cauchy => remodel_sfm::RobustLoss::Cauchy(f64::from(params.sfm.huber_delta_px)),
    };
    engine_params.dense.window_radius = params.dense.window_radius_px as i32;
    engine_params.dense.iterations = match params.dense.resolution {
        DenseResolution::Low => 2,
        DenseResolution::Medium => 4,
        DenseResolution::High => 8,
    };
    engine_params.dense_source_views = params.dense.min_view_consistency.max(1) as usize;
    engine_params.tsdf_voxel_size = f64::from(params.mesh.tsdf_voxel_size_mm) / 1000.0;
    engine_params.tsdf_truncation = f64::from(params.mesh.tsdf_truncation_mm) / 1000.0;
    engine_params.mesh.target_triangles = if params.mesh.decimate_target_triangles == 0 { usize::MAX } else { params.mesh.decimate_target_triangles as usize };
    engine_params.mesh.taubin_iterations = params.mesh.smoothing_iterations as usize;
    engine_params.mesh.atlas_size = params.mesh.texture_size;
    engine_params.mesh.guarantee_watertight = params.mesh.guarantee_watertight;
    engine_params.mesh.hole_fill_max_boundary_verts = params.mesh.hole_fill_max_boundary_verts as usize;
    engine_params.mesh.self_intersection_check = params.mesh.self_intersection_check;
    engine_params.texture_enabled = params.mesh.texture_enabled;
    engine_params.motion_enabled = params.motion.enabled;
    engine_params.geo_enabled = params.geo.enabled;
    engine_params.geo_cell_size = f64::from(params.geo.dsm_cell_m);
    engine_params
}

pub fn map_engine_stage(stage: remodel_engine::EngineStage) -> ReconstructionStage {
    match stage {
        remodel_engine::EngineStage::Idle => ReconstructionStage::Idle,
        remodel_engine::EngineStage::ExtractingFeatures => ReconstructionStage::ExtractingFeatures,
        remodel_engine::EngineStage::MatchingFeatures => ReconstructionStage::MatchingFeatures,
        remodel_engine::EngineStage::EstimatingPoses => ReconstructionStage::EstimatingPoses,
        remodel_engine::EngineStage::BundleAdjusting => ReconstructionStage::BundleAdjusting,
        remodel_engine::EngineStage::DenseStereo => ReconstructionStage::DenseStereo,
        remodel_engine::EngineStage::FusingVolume => ReconstructionStage::FusingVolume,
        remodel_engine::EngineStage::ExtractingSurface => ReconstructionStage::ExtractingSurface,
        remodel_engine::EngineStage::CleaningMesh => ReconstructionStage::CleaningMesh,
        remodel_engine::EngineStage::Texturing => ReconstructionStage::Texturing,
        remodel_engine::EngineStage::Done => ReconstructionStage::Done,
        remodel_engine::EngineStage::Failed => ReconstructionStage::Failed,
    }
}

pub fn stage_display(stage: ReconstructionStage) -> &'static str {
    match stage {
        ReconstructionStage::Idle => "Idle",
        ReconstructionStage::Ingesting => "Ingesting",
        ReconstructionStage::Calibrating => "Calibrating",
        ReconstructionStage::ExtractingFeatures => "Extracting Features",
        ReconstructionStage::MatchingFeatures => "Matching Features",
        ReconstructionStage::EstimatingPoses => "Estimating Poses",
        ReconstructionStage::BundleAdjusting => "Bundle Adjusting",
        ReconstructionStage::Georeferencing => "Georeferencing",
        ReconstructionStage::DenseStereo => "Dense Stereo",
        ReconstructionStage::FusingVolume => "Fusing Volume",
        ReconstructionStage::ExtractingSurface => "Extracting Surface",
        ReconstructionStage::CleaningMesh => "Cleaning Mesh",
        ReconstructionStage::Texturing => "Texturing",
        ReconstructionStage::TrackingMotion => "Tracking Motion",
        ReconstructionStage::DerivingGeoProducts => "Deriving Geo Products",
        ReconstructionStage::ReportingQc => "Reporting QC",
        ReconstructionStage::Done => "Done",
        ReconstructionStage::Failed => "Failed",
    }
}

/// 📍️ World-space camera center: `inverse().t` of the world→camera `Se3` (i.e. `-R⁻¹·t`).
fn camera_world_position(pose: &remodel_camera::CameraPose) -> [f32; 3] {
    let center = pose.0.inverse().t;
    [center[0] as f32, center[1] as f32, center[2] as f32]
}

/// 🔭️ A `CameraPosePreview` snapshot of one registration-order camera pose. `camera_id` isn't
/// recoverable from `ReconstructionEngine::sparse_preview` (it only exposes poses by registration
/// order, not by originating stream/frame/calibration id) — a documented simplification.
pub fn camera_pose_preview(index: u32, pose: &remodel_camera::CameraPose) -> CameraPosePreview {
    let translation = camera_world_position(pose);
    let quat = pose.0.inverse().r.to_quat();
    CameraPosePreview { camera_id: format!("cam-{index}"), rotation_wxyz: [quat.w as f32, quat.x as f32, quat.y as f32, quat.z as f32], translation }
}

pub fn watertight_snapshot(report: &remodel_mesh::WatertightReport) -> WatertightReportSnapshot {
    WatertightReportSnapshot {
        vertex_count: report.vertex_count as u32,
        triangle_count: report.triangle_count as u32,
        boundary_edge_count: report.boundary_edge_count as u32,
        boundary_loop_count: report.boundary_loop_count as u32,
        non_manifold_edge_count: report.non_manifold_edge_count as u32,
        non_manifold_vertex_count: report.non_manifold_vertex_count as u32,
        connected_components: report.connected_components as u32,
        consistently_oriented: report.consistently_oriented,
        euler_characteristic: report.euler_characteristic,
        genus: report.genus,
        signed_volume: report.signed_volume,
        self_intersection_pairs: report.self_intersection_pairs.map(|value| value as u32),
        closed_fallback_used: report.closed_fallback_used,
        is_closed: report.is_closed,
        is_two_manifold: report.is_two_manifold,
        is_watertight: report.is_watertight,
    }
}

/// ✅️ Maps `remodel_geo::QualityReport` (plus counts only the plugin can compute — accepted vs.
/// registered frames, whether any GCPs exist) onto the document's `QcReportSnapshot`.
/// `dense_coverage_ratio` stays `0.0`: the engine doesn't compute a density/overlap raster today —
/// a documented gap.
pub fn build_qc_snapshot(quality: &remodel_geo::QualityReport, registered_count: usize, accepted_count: usize, gcp_count: usize) -> QcReportSnapshot {
    let mut warnings = Vec::new();
    if let Some(watertight) = &quality.watertight {
        if !watertight.is_watertight {
            warnings.push("Mesh is not watertight.".to_string());
        }
    }
    if gcp_count > 0 && quality.gcp_checkpoint_rmse.is_none() {
        warnings.push("Ground control points are set but checkpoint RMSE is not yet computed.".to_string());
    }
    QcReportSnapshot {
        reprojection_rms_px: quality.reprojection_rms_px,
        gcp_checkpoint_rmse: quality.gcp_checkpoint_rmse,
        watertight: quality.watertight.as_ref().map(watertight_snapshot),
        mean_track_length: quality.track_stats.mean_track_length as f32,
        registered_frame_ratio: if accepted_count == 0 { 0.0 } else { registered_count as f32 / accepted_count as f32 },
        dense_coverage_ratio: 0.0,
        warnings,
    }
}

/// 🗺️ Normalizes a `remodel_geo::Raster`'s valid cells to a 16-bit grayscale PNG `ImageAsset` (invalid
/// cells encode as `0`) — `remodel_image::encode_png_gray16` is the shared 16-bit writer every other
/// raster-shaped asset in this codebase uses.
pub fn raster_to_png_asset(raster: &remodel_geo::Raster) -> ImageAsset {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for (index, &value) in raster.values.iter().enumerate() {
        if raster.valid.get(index).copied().unwrap_or(false) {
            min = min.min(value);
            max = max.max(value);
        }
    }
    if !min.is_finite() || !max.is_finite() {
        min = 0.0;
        max = 0.0;
    }
    let span = (max - min).max(1e-6);
    let data: Vec<u16> = raster.values.iter().enumerate().map(|(index, &value)| if raster.valid.get(index).copied().unwrap_or(false) { (((value - min) / span) * 65535.0).round().clamp(0.0, 65535.0) as u16 } else { 0 }).collect();
    let bytes = remodel_image::encode_png_gray16(&data, raster.width, raster.height).unwrap_or_default();
    ImageAsset { mime: "image/png".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), width: raster.width, height: raster.height }
}

pub fn video_codec_to_artifact(codec: remodel_video::VideoCodec) -> DocumentVideoCodec {
    match codec {
        remodel_video::VideoCodec::Avc => DocumentVideoCodec::Avc,
        remodel_video::VideoCodec::Hevc => DocumentVideoCodec::Hevc,
        remodel_video::VideoCodec::Vp9 => DocumentVideoCodec::Vp9,
        remodel_video::VideoCodec::Av1 => DocumentVideoCodec::Av1,
        remodel_video::VideoCodec::Mjpeg => DocumentVideoCodec::Mjpeg,
        remodel_video::VideoCodec::Unknown(_) => DocumentVideoCodec::Unknown,
    }
}

pub fn video_codec_from_label(label: &str) -> DocumentVideoCodec {
    match label.to_ascii_lowercase().as_str() {
        "avc" | "h264" | "h.264" => DocumentVideoCodec::Avc,
        "hevc" | "h265" | "h.265" => DocumentVideoCodec::Hevc,
        "vp9" => DocumentVideoCodec::Vp9,
        "av1" => DocumentVideoCodec::Av1,
        "mjpeg" | "mjpg" => DocumentVideoCodec::Mjpeg,
        _ => DocumentVideoCodec::Unknown,
    }
}

/// 🏷️ `(codec, width, height, duration_ms, container)` from either container family's probe.
pub fn describe_video_probe(probe: &remodel_video::VideoProbe) -> (remodel_video::VideoCodec, u32, u32, f64, &'static str) {
    match probe {
        remodel_video::VideoProbe::Mp4(info) => (info.codec, info.width, info.height, info.duration_ms, "mp4"),
        remodel_video::VideoProbe::Avi(info) => {
            let duration_ms = if info.fps > 0.0 { f64::from(info.frame_count) / info.fps * 1000.0 } else { 0.0 };
            (info.codec, info.width, info.height, duration_ms, "avi")
        }
    }
}
//#endregion 🔖️EngineMapping

//#region 🔖️Exporters
/// 🌐️ Hand-rolled ASCII Stanford PLY mesh exporter (vertex positions + optional per-vertex color, plus
/// the triangle face list). No PLY/LAS writer exists yet in `remodel_mesh`/`remodel_dense`/`remodel_geo`
/// (checked) — export-only, simple, well-specified formats are acceptable to hand-roll here per the
/// house convention that a library-side writer is only mandatory once one already exists.
pub struct PlyExporter;
impl MeshExporter for PlyExporter {
    fn format(&self) -> MediaFormat {
        MediaFormat::Ply
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_ply(mesh).into_bytes())
    }
}

fn mesh_to_ply(mesh: &MeshData) -> String {
    let vertex_count = mesh.vertex_count();
    let has_color = mesh.colors.len() == mesh.positions.len();
    let mut out = String::new();
    out.push_str("ply\nformat ascii 1.0\ncomment semio-remodel\n");
    out.push_str(&format!("element vertex {vertex_count}\n"));
    out.push_str("property float x\nproperty float y\nproperty float z\n");
    if has_color {
        out.push_str("property uchar red\nproperty uchar green\nproperty uchar blue\n");
    }
    out.push_str(&format!("element face {}\n", mesh.triangle_count()));
    out.push_str("property list uchar int vertex_indices\nend_header\n");
    for vertex in 0..vertex_count {
        let base = vertex * 3;
        out.push_str(&format!("{} {} {}", mesh.positions[base], mesh.positions[base + 1], mesh.positions[base + 2]));
        if has_color {
            let r = (mesh.colors[base].clamp(0.0, 1.0) * 255.0).round() as u8;
            let g = (mesh.colors[base + 1].clamp(0.0, 1.0) * 255.0).round() as u8;
            let b = (mesh.colors[base + 2].clamp(0.0, 1.0) * 255.0).round() as u8;
            out.push_str(&format!(" {r} {g} {b}"));
        }
        out.push('\n');
    }
    for triangle in mesh.indices.as_chunks::<3>().0 {
        out.push_str(&format!("3 {} {} {}\n", triangle[0], triangle[1], triangle[2]));
    }
    out
}

/// 🛰️ Hand-rolled binary LAS 1.2 exporter (public header block + point data record format 0): encodes
/// each mesh vertex as one unclassified LAS point at millimeter scale — a documented simplification
/// standing in for a real point-cloud-native LAS export (this codec is registered mesh-generically, so
/// it always receives a `MeshData`, never a raw point cloud).
pub struct LasExporter;
impl MeshExporter for LasExporter {
    fn format(&self) -> MediaFormat {
        MediaFormat::Las
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_las(mesh))
    }
}

fn pad_ascii(bytes: &[u8], len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    let n = bytes.len().min(len);
    out[..n].copy_from_slice(&bytes[..n]);
    out
}

fn mesh_to_las(mesh: &MeshData) -> Vec<u8> {
    let vertex_count = mesh.vertex_count();
    let scale = 0.001_f64;
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for vertex in 0..vertex_count {
        for axis in 0..3 {
            let value = f64::from(mesh.positions[vertex * 3 + axis]);
            min[axis] = min[axis].min(value);
            max[axis] = max[axis].max(value);
        }
    }
    if vertex_count == 0 {
        min = [0.0; 3];
        max = [0.0; 3];
    }
    let mut out = Vec::with_capacity(227 + vertex_count * 20);
    out.extend_from_slice(b"LASF");
    out.extend_from_slice(&0u16.to_le_bytes()); // file source id
    out.extend_from_slice(&0u16.to_le_bytes()); // global encoding
    out.extend_from_slice(&[0u8; 16]); // project id guid
    out.push(1); // version major
    out.push(2); // version minor
    out.extend_from_slice(&pad_ascii(b"semio", 32));
    out.extend_from_slice(&pad_ascii(b"semio-remodel", 32));
    out.extend_from_slice(&1u16.to_le_bytes()); // file creation day of year
    out.extend_from_slice(&2026u16.to_le_bytes()); // file creation year
    out.extend_from_slice(&227u16.to_le_bytes()); // header size
    out.extend_from_slice(&227u32.to_le_bytes()); // offset to point data (no VLRs)
    out.extend_from_slice(&0u32.to_le_bytes()); // number of variable length records
    out.push(0); // point data record format
    out.extend_from_slice(&20u16.to_le_bytes()); // point data record length
    out.extend_from_slice(&(vertex_count as u32).to_le_bytes()); // legacy number of point records
    for _ in 0..5 {
        out.extend_from_slice(&0u32.to_le_bytes()); // legacy number of points by return
    }
    for _ in 0..3 {
        out.extend_from_slice(&scale.to_le_bytes()); // x/y/z scale factor
    }
    for _ in 0..3 {
        out.extend_from_slice(&0.0f64.to_le_bytes()); // x/y/z offset
    }
    out.extend_from_slice(&max[0].to_le_bytes());
    out.extend_from_slice(&min[0].to_le_bytes());
    out.extend_from_slice(&max[1].to_le_bytes());
    out.extend_from_slice(&min[1].to_le_bytes());
    out.extend_from_slice(&max[2].to_le_bytes());
    out.extend_from_slice(&min[2].to_le_bytes());
    debug_assert_eq!(out.len(), 227, "LAS 1.2 public header block must be exactly 227 bytes");
    for vertex in 0..vertex_count {
        let x = (f64::from(mesh.positions[vertex * 3]) / scale).round() as i32;
        let y = (f64::from(mesh.positions[vertex * 3 + 1]) / scale).round() as i32;
        let z = (f64::from(mesh.positions[vertex * 3 + 2]) / scale).round() as i32;
        out.extend_from_slice(&x.to_le_bytes());
        out.extend_from_slice(&y.to_le_bytes());
        out.extend_from_slice(&z.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // intensity
        out.push(0b0000_1001); // return number 1 of 1
        out.push(0); // classification: unclassified
        out.push(0); // scan angle rank
        out.push(0); // user data
        out.extend_from_slice(&0u16.to_le_bytes()); // point source id
    }
    out
}

pub fn remodel_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let scene: RemodelSnapshot = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    Ok(scene.results.mesh.mesh)
}

/// 🖼️ Exports whichever raster/texture asset is available (DSM, else ortho, else the mesh's baked
/// texture) verbatim — every such asset is already a base64 PNG, so this is a lookup, not a re-encode.
pub fn remodel_png_export(doc: &Value) -> Result<semio_framework_os::OsMediaExportResult, String> {
    let scene: RemodelSnapshot = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    let asset_id = scene
        .results
        .geo
        .as_ref()
        .and_then(|geo| geo.dsm_asset_id.clone().or_else(|| geo.ortho_asset_id.clone()).or_else(|| geo.dtm_asset_id.clone()))
        .or_else(|| scene.results.mesh.texture_asset_id.clone())
        .ok_or_else(|| "no raster or texture asset is available to export as PNG".to_string())?;
    let asset = scene.assets.get(&asset_id).ok_or_else(|| "the referenced raster/texture asset is missing".to_string())?;
    Ok(semio_framework_os::OsMediaExportResult { data: asset.data.clone(), mime_type: "image/png".into(), file_name: "remodel-export.png".into(), encoding: Some("base64".into()) })
}
//#endregion 🔖️Exporters

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodel::default_remodel_scene;
    use semio_framework_plugin::mesh_from_kind;

    #[test]
    fn next_remodel_id_is_monotonic_and_prefixed() {
        let a = next_remodel_id("stream");
        let b = next_remodel_id("stream");
        assert!(a.starts_with("stream-"));
        assert!(b.starts_with("stream-"));
        assert_ne!(a, b);
    }

    //#region 🔖️IoTests
    #[test]
    fn remodel_io_declares_photos_in_and_mesh_out() {
        let io = remodel_io();
        assert_eq!(io.document_schema, "remodel.scene");
        assert_eq!(io.artifact.id, "3d.remodel");
        let photos_in = io.ports.iter().find(|port| port.id == "photos:in").expect("photos:in declared");
        assert_eq!(photos_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert_eq!(photos_in.kind_id.as_deref(), Some("2d.image"));
        assert!(!photos_in.required);
        assert_eq!(photos_in.multiplicity, semio_framework::PortMultiplicity::Many);
        let mesh_out = io.ports.iter().find(|port| port.id == "mesh:out").expect("mesh:out declared");
        assert_eq!(mesh_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(mesh_out.kind_id.as_deref(), Some("3d.mesh"));
        assert!(!mesh_out.required);
        assert_eq!(mesh_out.multiplicity, semio_framework::PortMultiplicity::Many);
    }
    //#endregion 🔖️IoTests

    #[test]
    fn ply_exporter_writes_a_well_formed_ascii_header() {
        let mesh = mesh_from_kind("box");
        let bytes = PlyExporter.export(&mesh).expect("ply export");
        let text = String::from_utf8(bytes).expect("ply is ascii");
        assert!(text.starts_with("ply\nformat ascii 1.0\n"));
        assert!(text.contains(&format!("element vertex {}\n", mesh.vertex_count())));
        assert!(text.contains(&format!("element face {}\n", mesh.triangle_count())));
        assert!(text.contains("end_header\n"));
    }

    #[test]
    fn las_exporter_writes_a_227_byte_header_plus_20_bytes_per_point() {
        let mesh = mesh_from_kind("box");
        let bytes = LasExporter.export(&mesh).expect("las export");
        assert_eq!(&bytes[0..4], b"LASF");
        assert_eq!(bytes.len(), 227 + mesh.vertex_count() * 20);
        let header_size = u16::from_le_bytes([bytes[94], bytes[95]]);
        assert_eq!(header_size, 227);
        let point_count = u32::from_le_bytes([bytes[107], bytes[108], bytes[109], bytes[110]]);
        assert_eq!(point_count as usize, mesh.vertex_count());
    }

    #[test]
    fn png_export_round_trips_a_stored_texture_asset() {
        let mut scene = default_remodel_scene();
        let png_bytes = remodel_image::encode_png(&remodel_image::ImageRgba8::new(4, 4)).expect("encode tiny png");
        let data = base64::engine::general_purpose::STANDARD.encode(png_bytes);
        scene.assets.insert("tex-1".into(), ImageAsset { mime: "image/png".into(), data: data.clone(), width: 4, height: 4 });
        scene.results.mesh.texture_asset_id = Some("tex-1".into());
        let doc = serde_json::to_value(&scene).expect("serialize scene");
        let result = remodel_png_export(&doc).expect("png export");
        assert_eq!(result.data, data);
        assert_eq!(result.mime_type, "image/png");
        assert_eq!(result.encoding.as_deref(), Some("base64"));
    }

    #[test]
    fn raster_to_png_asset_normalizes_valid_cells_only() {
        let mut raster = remodel_geo::Raster::new(2, 2, 1.0, [0.0, 0.0]);
        raster.set(0, 0, 0.0);
        raster.set(1, 1, 10.0);
        let asset = raster_to_png_asset(&raster);
        assert_eq!(asset.mime, "image/png");
        assert_eq!(asset.width, 2);
        assert_eq!(asset.height, 2);
        assert!(!asset.data.is_empty());
    }
}
//#endregion 🧪️Tests


//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent remodel artifact engine — owns the full artifact; snapshot is the persisted subset.
pub struct RemodelEngine {
    artifact: crate::artifacts::remodel::schema::RemodelArtifact,
    snapshot: crate::artifacts::remodel::RemodelSnapshot,
}

impl RemodelEngine {
    pub fn new(snapshot: crate::artifacts::remodel::RemodelSnapshot) -> Self {
        Self {
            artifact: crate::artifacts::remodel::schema::RemodelArtifact::from_snapshot(snapshot.clone()),
            snapshot,
        }
    }

    pub fn into_snapshot(self) -> crate::artifacts::remodel::RemodelSnapshot {
        self.snapshot
    }
}

impl protocol::ArtifactEngine for RemodelEngine {
    type Artifact = crate::artifacts::remodel::schema::RemodelArtifact;
    type Snapshot = crate::artifacts::remodel::RemodelSnapshot;
    type Mutation = crate::artifacts::remodel::mutations::RemodelMutation;
    type Diff = crate::artifacts::remodel::diff::RemodelDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }
}

/// 🧬️ Registers the remodel artifact schema descriptor once.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::remodel::schema::remodel_artifact_schema_descriptor());
}

//#endregion 🔖️ArtifactEngine
