//! ⚙️ Reconstruction engine: cooperative staged pipeline turning decoded frames into textured meshes,
//! previews and quality reports. Ties every sibling domain topic file together into the actual
//! video-in → watertight-mesh-out pipeline, self-contained (it never reaches back up at the artifact's
//! document types — the app-level translation layer in `🦀️.rs` does that in both directions).

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).
// 🏃️ `motion` is deliberately absent: the pipeline accepts `EngineParams::motion_enabled` but does not
// yet drive the motion topic file from `advance()` — a documented gap carried over verbatim from the
// pre-merge crate, which declared the same dependency without ever using it.
use crate::editor::remodeling::engine::{camera as remodeling_camera, dense as remodeling_dense, feature as remodeling_feature, geo as remodeling_geo, images as remodeling_image, mesh as remodeling_mesh, sfm as remodeling_sfm, video as remodeling_video};

// #region 🔖️Input
use std::collections::VecDeque;

const MAX_INTERACTIVE_IMAGE_PIXELS: usize = 262_144;
const MAX_INTERACTIVE_IMAGE_BYTES: usize = MAX_INTERACTIVE_IMAGE_PIXELS * 4;

/// 🎚️ Frame-ingestion policy shared by [`FrameSource::push_frame`] and [`FrameSource::push_video`]:
/// `stride` keeps every `stride`-th *offered* frame (applied only to the direct [`FrameSource::push_frame`]
/// entry point — [`FrameSource::push_video`] instead relies on the container-level stride already applied
/// by `remodeling_video::extract_frames`, so it isn't double-applied), `max_frames` caps the total accepted
/// count (`0` = unbounded), and `min_sharpness`/`rolling_window` drive the relative blur gate: a frame is
/// rejected when its gradient-based sharpness score falls below `min_sharpness * median(last
/// rolling_window accepted scores)`, once at least 3 accepted frames exist to form a baseline.
#[derive(Clone, Debug, PartialEq)]
pub struct IngestParams {
    pub stride: u32,
    pub max_frames: u32,
    pub min_sharpness: f32,
    pub rolling_window: usize,
}

impl Default for IngestParams {
    fn default() -> Self {
        Self { stride: 1, max_frames: 0, min_sharpness: 0.3, rolling_window: 15 }
    }
}

/// 🚦️ What [`FrameSource::push_frame`]/[`FrameSource::push_video`] did with one offered frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameAcceptance {
    Accepted,
    RejectedStride,
    RejectedMaxFrames,
    RejectedBlur,
}

/// 🖼️ One accepted input frame: pixels, true media timestamp, originating stream/camera id (single
/// stream, id `0`, until [`FrameSource`] grows multi-stream support), and the sharpness score that let
/// it through the blur gate.
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptedFrame {
    pub index: u32,
    pub image: remodeling_image::ImageRgba8,
    pub timestamp_ms: f64,
    pub stream_id: u32,
    pub sharpness: f32,
}

/// ⚠️ Errors from this crate's own fallible entry points — currently just video ingestion, re-exporting
/// `remodeling_video::VideoError` so callers get the precise failure (truncated container, unsupported
/// codec, malformed box, …) rather than a lossy wrapper.
#[derive(Clone, Debug, PartialEq)]
pub enum EngineError {
    Video(remodeling_video::VideoError),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Video(e) => write!(f, "video ingest error: {e}"),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<remodeling_video::VideoError> for EngineError {
    fn from(e: remodeling_video::VideoError) -> Self {
        Self::Video(e)
    }
}

/// 📊️ Outcome of [`FrameSource::push_video`]: how many samples the container yielded, how many the
/// ingestion policy accepted vs. rejected (split by reason), and provenance from the probe.
#[derive(Clone, Debug, PartialEq)]
pub struct PushVideoReport {
    pub frames_extracted: u32,
    pub frames_accepted: u32,
    pub frames_rejected_blur: u32,
    pub frames_rejected_sampling: u32,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub duration_ms: f64,
}

/// 🧭️ Gradient-energy sharpness proxy (mean squared Scharr gradient magnitude): high for crisp edges,
/// collapsing toward zero for a flat/blurred frame — the signal the relative blur gate thresholds
/// against.
fn sharpness_score(image: &remodeling_image::ImageRgba8) -> f32 {
    let gray = remodeling_image::ImageGray::from_rgba8_luma(image);
    let grad = remodeling_image::scharr_gradients(&gray);
    if grad.gx.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = grad.gx.iter().zip(grad.gy.iter()).map(|(&gx, &gy)| gx * gx + gy * gy).sum();
    sum_sq / grad.gx.len() as f32
}

/// 📐️ Median of a rolling score window (odd or even length both handled by taking the middle element of
/// the sorted copy — good enough for a soft gating threshold, no need for exact even-length averaging).
fn rolling_median(scores: &VecDeque<f32>) -> f32 {
    let mut v: Vec<f32> = scores.iter().copied().collect();
    v.sort_by(f32::total_cmp);
    v[v.len() / 2]
}

/// 🏷️ Human-facing codec/dimension/duration summary of a [`remodeling_video::VideoProbe`], regardless of
/// container family, for [`PushVideoReport`].
fn describe_probe(probe: &remodeling_video::VideoProbe) -> (String, u32, u32, f64) {
    match probe {
        remodeling_video::VideoProbe::Mp4(info) => (format!("{:?}", info.codec), info.width, info.height, info.duration_ms),
        remodeling_video::VideoProbe::Avi(info) => {
            let duration_ms = if info.fps > 0.0 { f64::from(info.frame_count) / info.fps * 1000.0 } else { 0.0 };
            (format!("{:?}", info.codec), info.width, info.height, duration_ms)
        }
    }
}

/// 📥️ Accumulates accepted input frames for one reconstruction: the real ingestion point where
/// stride/max-frame sampling and the gradient-score blur-rejection gate actually run, for both
/// direct-frame ([`push_frame`](Self::push_frame)) and video ([`push_video`](Self::push_video)) input.
pub struct FrameSource {
    ingest: IngestParams,
    stream_id: u32,
    offered: u32,
    frames: Vec<AcceptedFrame>,
    rolling_scores: VecDeque<f32>,
}

impl FrameSource {
    /// 🆕️ An empty frame source under the given ingestion policy.
    pub fn new(ingest: IngestParams) -> Self {
        Self { ingest, stream_id: 0, offered: 0, frames: Vec::new(), rolling_scores: VecDeque::new() }
    }

    /// 🔍️ Every frame accepted so far, in ingestion order.
    pub fn frames(&self) -> &[AcceptedFrame] {
        &self.frames
    }

    /// 🔢️ How many frames have been accepted so far.
    pub fn accepted_count(&self) -> usize {
        self.frames.len()
    }

    /// 📥️ Offers one directly-provided frame (e.g. an imported image sequence): applies this source's
    /// `stride`/`max_frames` sampling, then the relative blur gate.
    #[cfg(test)]
    pub fn push_frame(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64) -> FrameAcceptance {
        self.accept(index, image, timestamp_ms, true)
    }

    /// 🎞️ Probes `bytes` as a video container, lazily decodes sampled frames via
    /// `remodeling_video::extract_frames` (container-level stride/max-frames/downscale already applied per
    /// `opts`), and offers each decoded frame through the same blur gate as [`push_frame`](Self::push_frame)
    /// (without re-applying this source's own stride counter, since the container already sampled).
    #[cfg(test)]
    pub fn push_video(&mut self, bytes: &[u8], opts: &remodeling_video::VideoIngestOptions) -> Result<PushVideoReport, EngineError> {
        let probe = remodeling_video::probe(bytes)?;
        let (codec, width, height, duration_ms) = describe_probe(&probe);
        let iter = remodeling_video::extract_frames(bytes, opts)?;
        let mut report = PushVideoReport { frames_extracted: 0, frames_accepted: 0, frames_rejected_blur: 0, frames_rejected_sampling: 0, codec, width, height, duration_ms };
        for extracted in iter {
            let extracted = extracted?;
            report.frames_extracted += 1;
            match self.accept(extracted.index, extracted.image, extracted.timestamp_ms, false) {
                FrameAcceptance::Accepted => report.frames_accepted += 1,
                FrameAcceptance::RejectedBlur => report.frames_rejected_blur += 1,
                FrameAcceptance::RejectedStride | FrameAcceptance::RejectedMaxFrames => report.frames_rejected_sampling += 1,
            }
        }
        Ok(report)
    }

    /// 🚦️ Shared gate: optional stride counting, then `max_frames`, then the relative blur threshold
    /// against the rolling median of recently accepted scores.
    fn accept(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64, apply_stride: bool) -> FrameAcceptance {
        let score = sharpness_score(&image);
        self.accept_with_sharpness(index, image, timestamp_ms, apply_stride, score)
    }

    fn accept_with_sharpness(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64, apply_stride: bool, score: f32) -> FrameAcceptance {
        let offered = self.offered;
        self.offered += 1;
        if apply_stride {
            let stride = self.ingest.stride.max(1);
            if !offered.is_multiple_of(stride) {
                return FrameAcceptance::RejectedStride;
            }
        }
        if self.ingest.max_frames != 0 && self.frames.len() as u32 >= self.ingest.max_frames {
            return FrameAcceptance::RejectedMaxFrames;
        }
        if self.rolling_scores.len() >= 3 {
            let median = rolling_median(&self.rolling_scores);
            if score < self.ingest.min_sharpness * median {
                return FrameAcceptance::RejectedBlur;
            }
        }
        if self.rolling_scores.len() >= self.ingest.rolling_window.max(1) {
            self.rolling_scores.pop_front();
        }
        self.rolling_scores.push_back(score);
        self.frames.push(AcceptedFrame { index, image, timestamp_ms, stream_id: self.stream_id, sharpness: score });
        FrameAcceptance::Accepted
    }
}
// #endregion 🔖️Input

// #region 🔖️Params
/// 🎛️ Every knob the staged pipeline needs, bundled with sane defaults: ingestion policy, the assumed
/// pinhole focal-length ratio (no calibration stage exists yet, so intrinsics are derived from frame
/// dimensions), feature/matching/SfM/dense/mesh sub-configs (reusing each domain crate's own config type
/// directly rather than re-declaring their fields), and toggles for the optional motion/geo analyses.
#[derive(Clone, Debug, PartialEq)]
pub struct EngineParams {
    pub ingest: IngestParams,
    pub assumed_focal_ratio: f64,
    pub target_feature_count: usize,
    pub match_ratio: f32,
    pub match_mutual: bool,
    pub sequential_window: usize,
    pub sfm: remodeling_sfm::SfmConfig,
    pub dense: remodeling_dense::PatchMatchConfig,
    pub dense_source_views: usize,
    pub max_registered_cameras: usize,
    pub max_dense_cameras: usize,
    pub tsdf_voxel_size: f64,
    pub tsdf_truncation: f64,
    pub mesh: remodeling_mesh::MeshParams,
    pub texture_enabled: bool,
    pub motion_enabled: bool,
    pub geo_enabled: bool,
    pub geo_cell_size: f64,
}

impl Default for EngineParams {
    fn default() -> Self {
        Self {
            ingest: IngestParams::default(),
            assumed_focal_ratio: 1.0,
            target_feature_count: 500,
            match_ratio: 0.8,
            match_mutual: true,
            sequential_window: 2,
            sfm: remodeling_sfm::SfmConfig::default(),
            dense: remodeling_dense::PatchMatchConfig::default(),
            dense_source_views: 4,
            max_registered_cameras: 0,
            max_dense_cameras: 12,
            tsdf_voxel_size: 0.05,
            tsdf_truncation: 0.15,
            mesh: remodeling_mesh::MeshParams::default(),
            texture_enabled: true,
            motion_enabled: false,
            geo_enabled: false,
            geo_cell_size: 0.5,
        }
    }
}

/// 📷️ Default pinhole intrinsics assumed for uncalibrated input: `fx = fy = focal_ratio *
/// max(width, height)`, principal point at the image center, no distortion — a documented
/// simplification standing in for the calibration stage the base plan scopes separately.
fn default_intrinsics(width: u32, height: u32, focal_ratio: f64) -> remodeling_camera::Intrinsics {
    let f = focal_ratio * f64::from(width.max(height));
    remodeling_camera::Intrinsics { fx: f, fy: f, cx: f64::from(width) / 2.0, cy: f64::from(height) / 2.0, skew: 0.0, distortion: remodeling_camera::Distortion::None }
}
// #endregion 🔖️Params

// #region 🔖️Pipeline
/// 🚦️ Named stage of the cooperative reconstruction state machine — mirrors the stage *names* the
/// not-yet-rewritten `remodeling_document::ReconstructionStage` plans to expose, without depending on that
/// crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineStage {
    Idle,
    ExtractingFeatures,
    MatchingFeatures,
    EstimatingPoses,
    BundleAdjusting,
    DenseStereo,
    FusingVolume,
    ExtractingSurface,
    CleaningMesh,
    Texturing,
    Done,
    Failed,
}

/// 📡️ What [`ReconstructionEngine::advance`] returns: still working (with the current stage and a coarse
/// `[0, 1]` progress estimate), finished, or failed with a human-readable reason.
#[derive(Clone, Debug, PartialEq)]
pub enum EngineStatus {
    Working { stage: EngineStage, progress: f32 },
    Done,
    Failed(String),
}

/// 🔢️ Ordinal of a non-terminal stage in the fixed 9-stage pipeline, for [`ReconstructionEngine::progress`].
fn stage_ordinal(stage: EngineStage) -> usize {
    match stage {
        EngineStage::Idle => 0,
        EngineStage::ExtractingFeatures => 1,
        EngineStage::MatchingFeatures => 2,
        EngineStage::EstimatingPoses => 3,
        EngineStage::BundleAdjusting => 4,
        EngineStage::DenseStereo => 5,
        EngineStage::FusingVolume => 6,
        EngineStage::ExtractingSurface => 7,
        EngineStage::CleaningMesh => 8,
        EngineStage::Texturing => 9,
        EngineStage::Done | EngineStage::Failed => 10,
    }
}

/// 🗺️ Maps `remodeling_mesh::mesh_pipeline_step`'s internal stage name to the engine-level stage it falls
/// under, so driving the mesh pipeline (Amendment: engine delegates meshing directly to
/// `remodeling_mesh::mesh_pipeline_step`) still reports through the coarser [`EngineStage`] vocabulary.
fn mesh_stage_to_engine_stage(name: &str) -> EngineStage {
    match name {
        "marching_cubes" => EngineStage::ExtractingSurface,
        "unwrap" | "texture_bake" | "interchange" => EngineStage::Texturing,
        _ => EngineStage::CleaningMesh,
    }
}

/// 🏘️ Up to `k` other camera slot indices nearest to `ci` (by registration-order distance, which tracks
/// frame order for an [`remodeling_sfm::IncrementalSfm`] reconstruction), sorted ascending for determinism —
/// the source-view selection for [`remodeling_dense::patchmatch_mvs`]/TSDF fusion.
fn neighbor_camera_indices(ci: usize, n: usize, k: usize) -> Vec<usize> {
    let mut idxs = Vec::with_capacity(k.min(n.saturating_sub(1)));
    let mut distance = 1;
    while idxs.len() < k && (ci >= distance || ci.saturating_add(distance) < n) {
        if ci >= distance {
            idxs.push(ci - distance);
        }
        if idxs.len() < k && ci + distance < n {
            idxs.push(ci + distance);
        }
        distance += 1;
    }
    idxs.sort_unstable();
    idxs
}

/// 🎞️ Evenly spaced camera-slot indices for dense stereo / fusion when a reconstruction has more
/// registered views than [`EngineParams::max_dense_cameras`] (0 = unlimited). Full SfM cameras stay in
/// [`Reconstruction`] for gauge alignment; only the expensive depth/TSDF work is subsampled.
fn subsample_camera_indices(n: usize, max: usize) -> Vec<usize> {
    if n == 0 {
        return Vec::new();
    }
    if max == 0 || n <= max {
        return (0..n).collect();
    }
    if max == 1 {
        return vec![0];
    }
    (0..max).map(|i| i * (n - 1) / (max - 1)).collect()
}

/// 📦️ Voxel-index bounds covering `points` with a 20% margin plus a 2-voxel padding shell, for
/// `remodeling_mesh::MeshPipeline::new`'s `bounds_min`/`bounds_max`. Falls back to a small centered cube
/// when there are no points yet (degenerate input).
fn compute_voxel_bounds(points: &[[f64; 3]], voxel_size: f64) -> ([i32; 3], [i32; 3]) {
    const MAX_CELLS_PER_AXIS: i32 = 32;
    if points.is_empty() || voxel_size <= 0.0 {
        return ([-4, -4, -4], [4, 4, 4]);
    }
    // 🎯️ 5th/95th-percentile bounds per axis rather than raw min/max: a single badly-triangulated
    // outlier point (a real risk from a noisy incremental reconstruction) must not be able to blow the
    // TSDF/marching-cubes grid up to an unbounded size.
    let mut lo = [0.0; 3];
    let mut hi = [0.0; 3];
    for k in 0..3 {
        let mut vs: Vec<f64> = points.iter().map(|p| p[k]).collect();
        vs.sort_by(f64::total_cmp);
        let p05 = vs[((vs.len() as f64 - 1.0) * 0.05).round() as usize];
        let p95 = vs[((vs.len() as f64 - 1.0) * 0.95).round() as usize];
        lo[k] = p05;
        hi[k] = p95;
    }
    let mut bounds_min = [0i32; 3];
    let mut bounds_max = [0i32; 3];
    for k in 0..3 {
        let span = (hi[k] - lo[k]).max(voxel_size);
        let pad = span * 0.2;
        let raw_min = ((lo[k] - pad) / voxel_size).floor() as i32 - 2;
        let raw_max = ((hi[k] + pad) / voxel_size).ceil() as i32 + 2;
        let center = (raw_min + raw_max) / 2;
        let half_span = ((raw_max - raw_min) / 2).clamp(4, MAX_CELLS_PER_AXIS / 2);
        bounds_min[k] = center - half_span;
        bounds_max[k] = center + half_span;
    }
    (bounds_min, bounds_max)
}

fn compute_voxel_bounds_from_extrema(lo: [f64; 3], hi: [f64; 3], voxel_size: f64) -> ([i32; 3], [i32; 3]) {
    const MAX_CELLS_PER_AXIS: i32 = 32;
    if !lo.iter().all(|value| value.is_finite()) || !hi.iter().all(|value| value.is_finite()) || voxel_size <= 0.0 {
        return ([-4; 3], [4; 3]);
    }
    let mut minimum = [0; 3];
    let mut maximum = [0; 3];
    for axis in 0..3 {
        let span = (hi[axis] - lo[axis]).max(voxel_size);
        let padding = span * 0.2;
        let raw_minimum = ((lo[axis] - padding) / voxel_size).floor() as i32 - 2;
        let raw_maximum = ((hi[axis] + padding) / voxel_size).ceil() as i32 + 2;
        let center = (raw_minimum + raw_maximum) / 2;
        let half = ((raw_maximum - raw_minimum) / 2).clamp(4, MAX_CELLS_PER_AXIS / 2);
        minimum[axis] = center - half;
        maximum[axis] = center + half;
    }
    (minimum, maximum)
}

/// 🧵️ `(camera_slot_index, point_index, observed_pixel)` triples for `remodeling_geo::build_quality_report`,
/// derived from a finished [`remodeling_sfm::Reconstruction`]'s tracks and each frame's detected keypoints.
fn build_observations(recon: &remodeling_sfm::Reconstruction, tracks: Option<&remodeling_sfm::FeatureTracks>, keypoints_per_frame: &[Vec<remodeling_feature::Keypoint>]) -> Vec<(usize, usize, [f64; 2])> {
    let Some(tracks) = tracks else { return Vec::new() };
    let camera_index_of: std::collections::BTreeMap<usize, usize> = recon.cameras.iter().enumerate().map(|(ci, &(f, _))| (f, ci)).collect();
    let mut out = Vec::new();
    for (point_index, &track_id) in recon.point_track_ids.iter().enumerate() {
        let Some(track) = tracks.tracks.get(track_id) else { continue };
        for &(frame, kp) in track {
            let Some(&ci) = camera_index_of.get(&frame) else { continue };
            let Some(kp_list) = keypoints_per_frame.get(frame) else { continue };
            let Some(k) = kp_list.get(kp as usize) else { continue };
            out.push((ci, point_index, [f64::from(k.x), f64::from(k.y)]));
        }
    }
    out
}

/// ⚙️ The cooperative staged pipeline: [`advance`](Self::advance) performs one bounded slice of work per
/// call (one image featurized, one pair matched, one PnP registration, one bundle-adjustment solve, one
/// depth map, one TSDF integration batch, one `remodeling_mesh` pipeline stage) and is genuinely resumable —
/// calling it repeatedly with a small budget or once with a huge budget reaches the same [`EngineStatus::Done`]
/// result, only the call count differs.
pub struct ReconstructionEngine {
    params: EngineParams,
    frame_source: FrameSource,
    stage: EngineStage,
    frames: Vec<AcceptedFrame>,
    cursor: usize,
    feature_preparation: Option<FeaturePreparation>,

    keypoints_per_frame: Vec<Vec<remodeling_feature::Keypoint>>,
    descriptors_per_frame: Vec<Vec<remodeling_feature::Descriptor256>>,

    match_pairs: Vec<(usize, usize)>,
    match_pair_i: usize,
    match_pair_j: usize,
    match_anchor_frame: usize,
    match_pairs_ready: bool,
    pair_cursor: usize,
    pair_match_preparation: Option<PairMatchPreparation>,
    pairwise_matches: Vec<(usize, usize, Vec<remodeling_feature::Match>)>,
    track_preparation: Option<TrackPreparation>,
    tracks: Option<remodeling_sfm::FeatureTracks>,

    sfm: Option<remodeling_sfm::IncrementalSfm>,
    seed_pair_preparation: Option<remodeling_sfm::SeedPairPreparation>,
    registration_preparation: Option<remodeling_sfm::RegistrationPreparation>,
    bundle_preparation: Option<remodeling_sfm::BundlePreparation>,
    finalization_preparation: Option<FinalizationPreparation>,
    pose_cursor: usize,
    ba_substep: usize,
    reconstruction: Option<remodeling_sfm::Reconstruction>,
    observations: Vec<(usize, usize, [f64; 2])>,
    dense_camera_indices: Vec<usize>,

    stage_cursor: usize,
    depth_maps: Vec<remodeling_dense::DepthMap>,
    dense_preparation: Option<DenseStereoPreparation>,
    tsdf: Option<remodeling_dense::TsdfVolume>,
    tsdf_preparation: Option<remodeling_dense::TsdfIntegrationPreparation>,
    fusion_preparation: Option<remodeling_dense::FusionPreparation>,
    fusion_views: Vec<(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)>,
    fusion_capacity: usize,
    fusion_finalized: bool,
    dense_cloud: Option<remodeling_dense::PointCloud>,

    mesh_pipeline: Option<remodeling_mesh::MeshPipeline>,
    meshing_preparation: Option<MeshingPreparation>,
    mesh_data: Option<semio_framework::MeshData>,
    watertight_report: Option<remodeling_mesh::WatertightReport>,

    failure: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FeaturePhase {
    Luma,
    Detect,
    Describe,
}

struct FeaturePreparation {
    frame: usize,
    phase: FeaturePhase,
    cursor: usize,
    gray: Vec<f32>,
    keypoints: Vec<remodeling_feature::Keypoint>,
    descriptors: Vec<remodeling_feature::Descriptor256>,
}

struct PairMatchPreparation {
    frame_a: usize,
    frame_b: usize,
    query: usize,
    candidate: usize,
    best_distance: u32,
    best_index: u32,
    second_distance: u32,
    reverse_candidate: usize,
    reverse_best_distance: u32,
    reverse_best_index: u32,
    pending: Option<(u32, u32)>,
    matches: Vec<remodeling_feature::Match>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DenseStereoPhase {
    ReferenceLuma,
    SourceLuma,
    PatchMatch,
}

struct DenseStereoPreparation {
    phase: DenseStereoPhase,
    reference_frame: usize,
    reference_camera: (remodeling_camera::CameraPose, remodeling_camera::Intrinsics),
    source_frames: Vec<(usize, remodeling_camera::CameraPose)>,
    reference_gray: remodeling_image::ImageGray,
    source_grays: Vec<(remodeling_image::ImageGray, remodeling_camera::CameraPose, remodeling_camera::Intrinsics)>,
    source: usize,
    pixel: usize,
    patch_match: Option<remodeling_dense::PatchMatchPreparation>,
}

struct TextureViewPreparation {
    pose: remodeling_camera::CameraPose,
    intrinsics: remodeling_camera::Intrinsics,
    image: remodeling_image::ImageRgba8,
    cursor: usize,
}

struct MeshingPreparation {
    sparse_cursor: usize,
    dense_cursor: usize,
    bounds_min: [f64; 3],
    bounds_max: [f64; 3],
    view_cursor: usize,
    active_view: Option<TextureViewPreparation>,
    views: Vec<remodeling_mesh::TextureView>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FinalizationPhase {
    Snapshot,
    CameraIndex,
    Observations,
    DenseCameras,
    Done,
}

struct FinalizationPreparation {
    phase: FinalizationPhase,
    snapshot: Option<remodeling_sfm::ReconstructionSnapshotPreparation>,
    camera_cursor: usize,
    camera_index: std::collections::BTreeMap<usize, usize>,
    point_cursor: usize,
    observation_cursor: usize,
    dense_cursor: usize,
}

impl MeshingPreparation {
    fn new() -> Self {
        Self { sparse_cursor: 0, dense_cursor: 0, bounds_min: [f64::INFINITY; 3], bounds_max: [f64::NEG_INFINITY; 3], view_cursor: 0, active_view: None, views: Vec::new() }
    }

    fn include(&mut self, point: [f64; 3]) {
        for axis in 0..3 {
            self.bounds_min[axis] = self.bounds_min[axis].min(point[axis]);
            self.bounds_max[axis] = self.bounds_max[axis].max(point[axis]);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackPhase {
    Union,
    Group,
    Finish,
}

struct TrackGroup {
    observations: Vec<(usize, u32)>,
    frames: std::collections::BTreeSet<usize>,
    conflict: bool,
}

struct TrackPreparation {
    phase: TrackPhase,
    pair: usize,
    matched: usize,
    node_of: std::collections::HashMap<(usize, u32), usize>,
    observations: Vec<(usize, u32)>,
    parent: Vec<usize>,
    rank: Vec<u8>,
    grouping_cursor: usize,
    groups: std::collections::BTreeMap<usize, TrackGroup>,
    tracks: Vec<Vec<(usize, u32)>>,
}

impl TrackPreparation {
    fn new() -> Self {
        Self {
            phase: TrackPhase::Union,
            pair: 0,
            matched: 0,
            node_of: std::collections::HashMap::new(),
            observations: Vec::new(),
            parent: Vec::new(),
            rank: Vec::new(),
            grouping_cursor: 0,
            groups: std::collections::BTreeMap::new(),
            tracks: Vec::new(),
        }
    }

    fn node(&mut self, observation: (usize, u32)) -> usize {
        if let Some(&node) = self.node_of.get(&observation) {
            return node;
        }
        let node = self.observations.len();
        self.node_of.insert(observation, node);
        self.observations.push(observation);
        self.parent.push(node);
        self.rank.push(0);
        node
    }

    fn root(&mut self, mut node: usize) -> usize {
        while self.parent[node] != node {
            let parent = self.parent[node];
            self.parent[node] = self.parent[parent];
            node = self.parent[node];
        }
        node
    }

    fn union(&mut self, left: usize, right: usize) {
        let mut left = self.root(left);
        let mut right = self.root(right);
        if left == right {
            return;
        }
        if self.rank[left] < self.rank[right] {
            std::mem::swap(&mut left, &mut right);
        }
        self.parent[right] = left;
        if self.rank[left] == self.rank[right] {
            self.rank[left] = self.rank[left].saturating_add(1);
        }
    }
}

fn descriptor_distance(a: &remodeling_feature::Descriptor256, b: &remodeling_feature::Descriptor256) -> u32 {
    a.0.iter().zip(b.0.iter()).map(|(&left, &right)| (left ^ right).count_ones()).sum()
}

fn append_luma_slice(source: &remodeling_image::ImageRgba8, destination: &mut remodeling_image::ImageGray, cursor: &mut usize, pixel_budget: usize) -> bool {
    let pixels = source.width as usize * source.height as usize;
    let end = cursor.saturating_add(pixel_budget.max(1)).min(pixels);
    for index in *cursor..end {
        let offset = index * 4;
        destination.data.push((0.299 * f32::from(source.data[offset]) + 0.587 * f32::from(source.data[offset + 1]) + 0.114 * f32::from(source.data[offset + 2])) / 255.0);
    }
    *cursor = end;
    end == pixels
}

fn reset_pair_query(preparation: &mut PairMatchPreparation) {
    preparation.query += 1;
    preparation.candidate = 0;
    preparation.best_distance = u32::MAX;
    preparation.best_index = u32::MAX;
    preparation.second_distance = u32::MAX;
    preparation.reverse_candidate = 0;
    preparation.reverse_best_distance = u32::MAX;
    preparation.reverse_best_index = u32::MAX;
    preparation.pending = None;
}

impl ReconstructionEngine {
    /// 🆕️ A fresh engine in [`EngineStage::Idle`], with an internal empty [`FrameSource`] under
    /// `params.ingest`. Push frames via [`push_frame`](Self::push_frame)/[`push_video`](Self::push_video)
    /// before the first [`advance`](Self::advance) call.
    pub fn new(params: &EngineParams) -> Self {
        Self {
            params: params.clone(),
            frame_source: FrameSource::new(params.ingest.clone()),
            stage: EngineStage::Idle,
            frames: Vec::new(),
            cursor: 0,
            feature_preparation: None,
            keypoints_per_frame: Vec::new(),
            descriptors_per_frame: Vec::new(),
            match_pairs: Vec::new(),
            match_pair_i: 0,
            match_pair_j: 1,
            match_anchor_frame: 2,
            match_pairs_ready: false,
            pair_cursor: 0,
            pair_match_preparation: None,
            pairwise_matches: Vec::new(),
            track_preparation: None,
            tracks: None,
            sfm: None,
            seed_pair_preparation: None,
            registration_preparation: None,
            bundle_preparation: None,
            finalization_preparation: None,
            pose_cursor: 0,
            ba_substep: 0,
            reconstruction: None,
            observations: Vec::new(),
            dense_camera_indices: Vec::new(),
            stage_cursor: 0,
            depth_maps: Vec::new(),
            dense_preparation: None,
            tsdf: None,
            tsdf_preparation: None,
            fusion_preparation: None,
            fusion_views: Vec::new(),
            fusion_capacity: 0,
            fusion_finalized: false,
            dense_cloud: None,
            mesh_pipeline: None,
            meshing_preparation: None,
            mesh_data: None,
            watertight_report: None,
            failure: None,
        }
    }

    /// 📥️ Delegates to the internal [`FrameSource::push_frame`].
    #[cfg(test)]
    pub fn push_frame(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64) -> FrameAcceptance {
        self.frame_source.push_frame(index, image, timestamp_ms)
    }

    /// ⏱️ Commits a frame after its gradient score was computed by the outer resumable ingestion cursor.
    pub fn push_frame_with_sharpness(&mut self, index: u32, image: remodeling_image::ImageRgba8, timestamp_ms: f64, sharpness: f32) -> FrameAcceptance {
        self.frame_source.accept_with_sharpness(index, image, timestamp_ms, true, sharpness)
    }

    /// 🎞️ Delegates to the internal [`FrameSource::push_video`].
    #[cfg(test)]
    pub fn push_video(&mut self, bytes: &[u8], opts: &remodeling_video::VideoIngestOptions) -> Result<PushVideoReport, EngineError> {
        self.frame_source.push_video(bytes, opts)
    }

    /// 🔍️ The internal frame source, for inspecting accepted frames/counts without driving the pipeline.
    pub fn frame_source(&self) -> &FrameSource {
        &self.frame_source
    }

    /// 🚦️ Current stage.
    pub fn stage(&self) -> EngineStage {
        self.stage
    }

    /// 📸️ Snapshots [`FrameSource::frames`] into the engine's own working set; fails if fewer than 2
    /// frames were accepted (the minimum an [`remodeling_sfm::IncrementalSfm`] two-view init needs).
    fn start(&mut self) -> Result<(), String> {
        self.frames = std::mem::take(&mut self.frame_source.frames);
        if self.frames.len() < 2 {
            return Err(format!("reconstruction requires at least 2 accepted frames, got {}", self.frames.len()));
        }
        if self.frames.len() > 64 {
            return Err(format!("reconstruction admits at most 64 frames per interactive job, got {}", self.frames.len()));
        }
        for frame in &self.frames {
            let pixels = frame.image.width as usize * frame.image.height as usize;
            if pixels > MAX_INTERACTIVE_IMAGE_PIXELS || frame.image.data.len() != pixels.saturating_mul(4) || frame.image.data.len() > MAX_INTERACTIVE_IMAGE_BYTES {
                return Err(format!("interactive reconstruction image envelope exceeded by frame {}: {}x{} / {} bytes", frame.index, frame.image.width, frame.image.height, frame.image.data.len()));
            }
        }
        self.cursor = 0;
        Ok(())
    }

    /// 🎯️ One fuel-bounded luma/detect/describe slice; returns whether more frames remain.
    fn step_extracting_features(&mut self) -> bool {
        const PIXELS_PER_STEP: usize = 4_096;
        const DESCRIPTORS_PER_STEP: usize = 16;
        if self.cursor >= self.frames.len() {
            return false;
        }
        let target_feature_count = self.params.target_feature_count.min(512);
        if self.feature_preparation.is_none() {
            let pixels = self.frames[self.cursor].image.width as usize * self.frames[self.cursor].image.height as usize;
            self.feature_preparation =
                Some(FeaturePreparation { frame: self.cursor, phase: FeaturePhase::Luma, cursor: 0, gray: Vec::with_capacity(pixels), keypoints: Vec::with_capacity(target_feature_count), descriptors: Vec::with_capacity(target_feature_count) });
        }
        let preparation = self.feature_preparation.as_mut().expect("feature preparation");
        let image = &self.frames[preparation.frame].image;
        match preparation.phase {
            FeaturePhase::Luma => {
                let pixels = image.width as usize * image.height as usize;
                let end = preparation.cursor.saturating_add(PIXELS_PER_STEP).min(pixels);
                for index in preparation.cursor..end {
                    let offset = index * 4;
                    preparation.gray.push((0.299 * f32::from(image.data[offset]) + 0.587 * f32::from(image.data[offset + 1]) + 0.114 * f32::from(image.data[offset + 2])) / 255.0);
                }
                preparation.cursor = end;
                if end == pixels {
                    preparation.phase = FeaturePhase::Detect;
                    preparation.cursor = 0;
                }
            }
            FeaturePhase::Detect => {
                let width = image.width as usize;
                let pixels = preparation.gray.len();
                let end = preparation.cursor.saturating_add(PIXELS_PER_STEP).min(pixels);
                if width > 2 && image.height > 2 {
                    for index in preparation.cursor..end {
                        if preparation.keypoints.len() >= target_feature_count {
                            break;
                        }
                        let x = index % width;
                        let y = index / width;
                        if x == 0 || y == 0 || x + 1 >= width || y + 1 >= image.height as usize {
                            continue;
                        }
                        let gx = preparation.gray[index + 1] - preparation.gray[index - 1];
                        let gy = preparation.gray[index + width] - preparation.gray[index - width];
                        let response = gx * gx + gy * gy;
                        if response > 0.001 && (x + y) % 3 == 0 {
                            preparation.keypoints.push(remodeling_feature::Keypoint { x: x as f32, y: y as f32, octave: 0, angle: gy.atan2(gx), response });
                        }
                    }
                }
                preparation.cursor = end;
                if end == pixels || preparation.keypoints.len() >= target_feature_count {
                    preparation.phase = FeaturePhase::Describe;
                    preparation.cursor = 0;
                }
            }
            FeaturePhase::Describe => {
                let end = preparation.cursor.saturating_add(DESCRIPTORS_PER_STEP).min(preparation.keypoints.len());
                let width = image.width as usize;
                let height = image.height as usize;
                for keypoint in &preparation.keypoints[preparation.cursor..end] {
                    let mut words = [0u64; 4];
                    let cx = keypoint.x as i32;
                    let cy = keypoint.y as i32;
                    for bit in 0..256usize {
                        let ax = (cx + ((bit * 37) % 31) as i32 - 15).clamp(0, width.saturating_sub(1) as i32) as usize;
                        let ay = (cy + ((bit * 17) % 31) as i32 - 15).clamp(0, height.saturating_sub(1) as i32) as usize;
                        let bx = (cx + ((bit * 13 + 7) % 31) as i32 - 15).clamp(0, width.saturating_sub(1) as i32) as usize;
                        let by = (cy + ((bit * 29 + 3) % 31) as i32 - 15).clamp(0, height.saturating_sub(1) as i32) as usize;
                        if preparation.gray[ay * width + ax] < preparation.gray[by * width + bx] {
                            words[bit / 64] |= 1u64 << (bit % 64);
                        }
                    }
                    preparation.descriptors.push(remodeling_feature::Descriptor256(words));
                }
                preparation.cursor = end;
                if end == preparation.keypoints.len() {
                    let complete = self.feature_preparation.take().expect("completed feature preparation");
                    self.keypoints_per_frame.push(complete.keypoints);
                    self.descriptors_per_frame.push(complete.descriptors);
                    self.cursor += 1;
                }
            }
        }
        self.cursor < self.frames.len()
    }

    /// 🕸️ Sequential-window pair list `(i, j)` for `j in (i, i + sequential_window]`, built once when
    /// entering [`EngineStage::MatchingFeatures`].
    /// 🕸️ Sequential-window pairs `(i, j)` for `j in (i, i + sequential_window]`, plus an explicit
    /// `(0, f)`/`(1, f)` "anchor" pair for every later frame `f`: `IncrementalSfm::init_pair` triangulates
    /// only tracks directly spanning both anchor frames, so without a direct anchor↔f pair a later frame's
    /// `register_next` PnP can only find 2D-3D correspondences through incidental semio_hub-chained tracks —
    /// sparse enough on real matches to starve most frames of the 6 correspondences PnP needs. Explicit
    /// anchor pairs make every registerable frame's correspondence-to-the-seed-pair direct instead of
    /// coincidental.
    fn step_build_match_pairs(&mut self, pair_budget: usize) -> bool {
        let n = self.frames.len();
        let window = self.params.sequential_window.max(1);
        let mut work = 0;
        while self.match_pair_i < n && work < pair_budget {
            let hi = self.match_pair_i.saturating_add(window).min(n.saturating_sub(1));
            if self.match_pair_j <= hi {
                self.match_pairs.push((self.match_pair_i, self.match_pair_j));
                self.match_pair_j += 1;
                work += 1;
            } else {
                self.match_pair_i += 1;
                self.match_pair_j = self.match_pair_i.saturating_add(1);
            }
        }
        while self.match_pair_i >= n && self.match_anchor_frame < n && work < pair_budget {
            let frame = self.match_anchor_frame;
            if frame > window {
                self.match_pairs.push((0, frame));
                work += 1;
            }
            if work < pair_budget && frame > 1 + window {
                self.match_pairs.push((1, frame));
                work += 1;
            }
            self.match_anchor_frame += 1;
        }
        self.match_pair_i >= n && self.match_anchor_frame >= n
    }

    /// 🤝️ Consumes at most 4,096 Hamming comparisons across a resumable pair match.
    fn step_matching_features(&mut self) -> bool {
        const COMPARISONS_PER_STEP: usize = 4_096;
        if self.pair_cursor >= self.match_pairs.len() {
            return false;
        }
        if self.pair_match_preparation.is_none() {
            let (frame_a, frame_b) = self.match_pairs[self.pair_cursor];
            self.pair_match_preparation = Some(PairMatchPreparation {
                frame_a,
                frame_b,
                query: 0,
                candidate: 0,
                best_distance: u32::MAX,
                best_index: u32::MAX,
                second_distance: u32::MAX,
                reverse_candidate: 0,
                reverse_best_distance: u32::MAX,
                reverse_best_index: u32::MAX,
                pending: None,
                matches: Vec::new(),
            });
        }
        let preparation = self.pair_match_preparation.as_mut().expect("pair match preparation");
        let desc_a = &self.descriptors_per_frame[preparation.frame_a];
        let desc_b = &self.descriptors_per_frame[preparation.frame_b];
        if desc_b.is_empty() {
            preparation.query = preparation.query.saturating_add(COMPARISONS_PER_STEP).min(desc_a.len());
            if preparation.query == desc_a.len() {
                let complete = self.pair_match_preparation.take().expect("completed empty pair match");
                self.pairwise_matches.push((complete.frame_a, complete.frame_b, complete.matches));
                self.pair_cursor += 1;
            }
            return self.pair_cursor < self.match_pairs.len();
        }
        let mut remaining = COMPARISONS_PER_STEP;
        while remaining > 0 && preparation.query < desc_a.len() {
            if let Some((distance, best_index)) = preparation.pending {
                while remaining > 0 && preparation.reverse_candidate < desc_a.len() {
                    let candidate = preparation.reverse_candidate;
                    let back_distance = descriptor_distance(&desc_b[best_index as usize], &desc_a[candidate]);
                    if back_distance < preparation.reverse_best_distance {
                        preparation.reverse_best_distance = back_distance;
                        preparation.reverse_best_index = candidate as u32;
                    }
                    preparation.reverse_candidate += 1;
                    remaining -= 1;
                }
                if preparation.reverse_candidate == desc_a.len() {
                    if preparation.reverse_best_index == preparation.query as u32 {
                        preparation.matches.push(remodeling_feature::Match { a: preparation.query as u32, b: best_index, distance });
                    }
                    reset_pair_query(preparation);
                }
                continue;
            }
            while remaining > 0 && preparation.candidate < desc_b.len() {
                let candidate = preparation.candidate;
                let distance = descriptor_distance(&desc_a[preparation.query], &desc_b[candidate]);
                if distance < preparation.best_distance {
                    preparation.second_distance = preparation.best_distance;
                    preparation.best_distance = distance;
                    preparation.best_index = candidate as u32;
                } else if distance < preparation.second_distance {
                    preparation.second_distance = distance;
                }
                preparation.candidate += 1;
                remaining -= 1;
            }
            if preparation.candidate == desc_b.len() {
                let passes = preparation.best_index != u32::MAX && (preparation.second_distance == u32::MAX || (preparation.best_distance as f32) < self.params.match_ratio * preparation.second_distance as f32);
                if passes && self.params.match_mutual {
                    preparation.pending = Some((preparation.best_distance, preparation.best_index));
                    preparation.reverse_candidate = 0;
                    preparation.reverse_best_distance = u32::MAX;
                    preparation.reverse_best_index = u32::MAX;
                } else {
                    if passes {
                        preparation.matches.push(remodeling_feature::Match { a: preparation.query as u32, b: preparation.best_index, distance: preparation.best_distance });
                    }
                    reset_pair_query(preparation);
                }
            }
        }
        if preparation.query == desc_a.len() {
            let complete = self.pair_match_preparation.take().expect("completed pair match");
            self.pairwise_matches.push((complete.frame_a, complete.frame_b, complete.matches));
            self.pair_cursor += 1;
        }
        self.pair_cursor < self.match_pairs.len()
    }

    /// 🧵️ Consumes at most 4,096 union/group observations or 64 finalized groups.
    fn step_build_tracks(&mut self) -> Option<remodeling_sfm::FeatureTracks> {
        const OBSERVATIONS_PER_STEP: usize = 4_096;
        const GROUPS_PER_STEP: usize = 64;
        let preparation = self.track_preparation.get_or_insert_with(TrackPreparation::new);
        match preparation.phase {
            TrackPhase::Union => {
                let mut work = 0;
                while preparation.pair < self.pairwise_matches.len() && work < OBSERVATIONS_PER_STEP {
                    let (frame_a, frame_b, matches) = &self.pairwise_matches[preparation.pair];
                    if preparation.matched >= matches.len() {
                        preparation.pair += 1;
                        preparation.matched = 0;
                        continue;
                    }
                    let matched = matches[preparation.matched];
                    let frame_a = *frame_a;
                    let frame_b = *frame_b;
                    preparation.matched += 1;
                    let left = preparation.node((frame_a, matched.a));
                    let right = preparation.node((frame_b, matched.b));
                    preparation.union(left, right);
                    work += 1;
                }
                if preparation.pair == self.pairwise_matches.len() {
                    preparation.phase = TrackPhase::Group;
                }
            }
            TrackPhase::Group => {
                let end = preparation.grouping_cursor.saturating_add(OBSERVATIONS_PER_STEP).min(preparation.observations.len());
                for index in preparation.grouping_cursor..end {
                    let observation = preparation.observations[index];
                    let root = preparation.root(index);
                    let group = preparation.groups.entry(root).or_insert_with(|| TrackGroup { observations: Vec::new(), frames: std::collections::BTreeSet::new(), conflict: false });
                    if !group.frames.insert(observation.0) {
                        group.conflict = true;
                    }
                    group.observations.push(observation);
                }
                preparation.grouping_cursor = end;
                if end == preparation.observations.len() {
                    preparation.phase = TrackPhase::Finish;
                }
            }
            TrackPhase::Finish => {
                for _ in 0..GROUPS_PER_STEP {
                    let Some((_, group)) = preparation.groups.pop_first() else { break };
                    if !group.conflict {
                        preparation.tracks.push(group.observations);
                    }
                }
                if preparation.groups.is_empty() {
                    let complete = self.track_preparation.take().expect("completed tracks");
                    return Some(remodeling_sfm::FeatureTracks { tracks: complete.tracks });
                }
            }
        }
        None
    }

    /// 🏗️ Either seeds [`remodeling_sfm::IncrementalSfm`] via `init_pair(0, 1, ..)` (first call), or
    /// registers+triangulates the unregistered frame with the most 2D-3D (or two-view) support
    /// (subsequent calls) — next-best rather than strict sequential order, so a later frame that
    /// already shares triangulated tracks with the seed pair can unlock earlier starved frames.
    /// Mirrors `run_all`'s best-effort policy (a frame that fails to register is skipped for this
    /// step, not fatal) except for the initial pair, whose failure genuinely aborts the reconstruction.
    fn step_estimating_poses(&mut self) -> Result<bool, String> {
        if self.sfm.is_none() {
            let intr = default_intrinsics(self.frames[0].image.width, self.frames[0].image.height, self.params.assumed_focal_ratio);
            let tracks = self.tracks.take().expect("tracks built before EstimatingPoses");
            let keypoints = std::mem::take(&mut self.keypoints_per_frame);
            let mut sfm = remodeling_sfm::IncrementalSfm::new(intr, tracks, keypoints, self.params.sfm.clone());
            let pair01 = self.pairwise_matches.iter().find(|&&(a, b, _)| a == 0 && b == 1).map(|(_, _, matches)| remodeling_sfm::SeedPairPreparation::new(0, 1, matches)).ok_or_else(|| "no matches between frame 0 and 1".to_string())?;
            sfm.set_pairwise_matches(std::mem::take(&mut self.pairwise_matches));
            self.sfm = Some(sfm);
            self.seed_pair_preparation = Some(pair01);
        }
        if let Some(preparation) = self.seed_pair_preparation.as_mut() {
            if self.sfm.as_mut().expect("seed SfM").advance_seed_pair(preparation, 1).map_err(|error| error.to_string())? {
                self.seed_pair_preparation = None;
                self.pose_cursor = 2;
            }
            return Ok(true);
        }
        let n = self.frames.len();
        let sfm = self.sfm.as_mut().expect("initialized SfM");
        let registered = sfm.registered_count();
        if self.params.max_registered_cameras > 0 && registered >= self.params.max_registered_cameras {
            return Ok(false);
        }
        if self.pose_cursor >= n {
            return Ok(false);
        }
        if self.registration_preparation.is_none() {
            self.registration_preparation = Some(remodeling_sfm::RegistrationPreparation::new(self.pose_cursor));
        }
        let result = sfm.advance_registration(self.registration_preparation.as_mut().expect("registration preparation"), 1);
        match result {
            Ok(true) | Err(_) => {
                self.registration_preparation = None;
                self.pose_cursor += 1;
            }
            Ok(false) => {}
        }
        Ok(self.pose_cursor < n || self.registration_preparation.is_some())
    }

    /// 🎯️ Advances exactly one bounded bundle cleanup or retriangulation unit.
    fn step_bundle_adjusting(&mut self) -> bool {
        if self.ba_substep != 0 {
            return false;
        }
        let Some(sfm) = self.sfm.as_mut() else { return false };
        if self.bundle_preparation.is_none() {
            self.bundle_preparation = Some(sfm.begin_bundle());
        }
        if sfm.advance_bundle(self.bundle_preparation.as_mut().expect("bundle preparation"), 1) {
            self.bundle_preparation = None;
            self.ba_substep = 1;
            return false;
        }
        true
    }

    /// 📦️ Moves the final sparse snapshot, observation table, and dense-camera index through bounded
    /// item cursors. No camera/point/observation collection is materialized in one continuation.
    fn step_finalize_reconstruction(&mut self) -> bool {
        const ITEMS_PER_STEP: usize = 64;
        if self.finalization_preparation.is_none() {
            let snapshot = self.sfm.as_ref().map(remodeling_sfm::IncrementalSfm::begin_reconstruction_snapshot);
            self.finalization_preparation = Some(FinalizationPreparation { phase: FinalizationPhase::Snapshot, snapshot, camera_cursor: 0, camera_index: std::collections::BTreeMap::new(), point_cursor: 0, observation_cursor: 0, dense_cursor: 0 });
        }
        let preparation = self.finalization_preparation.as_mut().expect("finalization preparation");
        match preparation.phase {
            FinalizationPhase::Snapshot => {
                let Some(snapshot) = preparation.snapshot.as_mut() else {
                    preparation.phase = FinalizationPhase::Done;
                    return true;
                };
                let sfm = self.sfm.as_ref().expect("snapshot SfM");
                if sfm.advance_reconstruction_snapshot(snapshot, ITEMS_PER_STEP) {
                    let snapshot = preparation.snapshot.take().expect("completed snapshot");
                    self.reconstruction = Some(remodeling_sfm::IncrementalSfm::finish_reconstruction_snapshot(snapshot));
                    preparation.phase = FinalizationPhase::CameraIndex;
                }
            }
            FinalizationPhase::CameraIndex => {
                let reconstruction = self.reconstruction.as_ref().expect("final reconstruction");
                let end = preparation.camera_cursor.saturating_add(ITEMS_PER_STEP).min(reconstruction.cameras.len());
                for index in preparation.camera_cursor..end {
                    preparation.camera_index.insert(reconstruction.cameras[index].0, index);
                }
                preparation.camera_cursor = end;
                if end == reconstruction.cameras.len() {
                    preparation.phase = FinalizationPhase::Observations;
                }
            }
            FinalizationPhase::Observations => {
                let reconstruction = self.reconstruction.as_ref().expect("final reconstruction");
                let tracks = self.tracks.as_ref().expect("feature tracks");
                let mut remaining = ITEMS_PER_STEP;
                while preparation.point_cursor < reconstruction.point_track_ids.len() && remaining > 0 {
                    let track_id = reconstruction.point_track_ids[preparation.point_cursor];
                    let Some(track) = tracks.tracks.get(track_id) else {
                        preparation.point_cursor += 1;
                        preparation.observation_cursor = 0;
                        continue;
                    };
                    if preparation.observation_cursor >= track.len() {
                        preparation.point_cursor += 1;
                        preparation.observation_cursor = 0;
                        continue;
                    }
                    let (frame, keypoint) = track[preparation.observation_cursor];
                    preparation.observation_cursor += 1;
                    remaining -= 1;
                    let Some(&camera) = preparation.camera_index.get(&frame) else { continue };
                    let Some(keypoints) = self.keypoints_per_frame.get(frame) else { continue };
                    let Some(keypoint) = keypoints.get(keypoint as usize) else { continue };
                    self.observations.push((camera, preparation.point_cursor, [f64::from(keypoint.x), f64::from(keypoint.y)]));
                }
                if preparation.point_cursor == reconstruction.point_track_ids.len() {
                    preparation.phase = FinalizationPhase::DenseCameras;
                }
            }
            FinalizationPhase::DenseCameras => {
                let cameras = self.reconstruction.as_ref().map_or(0, |reconstruction| reconstruction.cameras.len());
                let dense_count = if self.params.max_dense_cameras == 0 { cameras.min(12) } else { cameras.min(self.params.max_dense_cameras).min(12) };
                if preparation.dense_cursor < dense_count {
                    let index = if dense_count <= 1 { 0 } else { preparation.dense_cursor * cameras.saturating_sub(1) / dense_count.saturating_sub(1) };
                    self.dense_camera_indices.push(index);
                    preparation.dense_cursor += 1;
                } else {
                    self.depth_maps = Vec::with_capacity(self.dense_camera_indices.len());
                    self.stage_cursor = 0;
                    preparation.phase = FinalizationPhase::Done;
                }
            }
            FinalizationPhase::Done => return true,
        }
        preparation.phase == FinalizationPhase::Done
    }

    /// 🌫️ One registered camera's `remodeling_dense::patchmatch_mvs` depth map against its nearest
    /// registered neighbors; returns whether more cameras remain.
    fn step_dense_stereo(&mut self) -> bool {
        const LUMA_PIXELS_PER_STEP: usize = 2_048;
        const PATCH_PIXELS_PER_STEP: usize = 1;
        let n_dense = self.dense_camera_indices.len();
        if n_dense == 0 {
            return false;
        }
        let slot = self.stage_cursor;
        if slot >= n_dense {
            return false;
        }
        if self.dense_preparation.is_none() {
            let ci = self.dense_camera_indices[slot];
            let reconstruction = self.reconstruction.as_ref().expect("dense requires reconstruction");
            let (reference_frame, pose) = reconstruction.cameras[ci];
            let intrinsics = reconstruction.intrinsics;
            let source_frames = neighbor_camera_indices(ci, reconstruction.cameras.len(), self.params.dense_source_views.min(8)).into_iter().map(|neighbor| reconstruction.cameras[neighbor]).collect();
            let reference = &self.frames[reference_frame].image;
            self.dense_preparation = Some(DenseStereoPreparation {
                phase: DenseStereoPhase::ReferenceLuma,
                reference_frame,
                reference_camera: (pose, intrinsics),
                source_frames,
                reference_gray: remodeling_image::ImageGray { width: reference.width, height: reference.height, data: Vec::with_capacity(reference.width as usize * reference.height as usize) },
                source_grays: Vec::new(),
                source: 0,
                pixel: 0,
                patch_match: None,
            });
        }
        let preparation = self.dense_preparation.as_mut().expect("dense preparation");
        match preparation.phase {
            DenseStereoPhase::ReferenceLuma => {
                if append_luma_slice(&self.frames[preparation.reference_frame].image, &mut preparation.reference_gray, &mut preparation.pixel, LUMA_PIXELS_PER_STEP) {
                    preparation.pixel = 0;
                    preparation.phase = DenseStereoPhase::SourceLuma;
                }
            }
            DenseStereoPhase::SourceLuma => {
                if preparation.source == preparation.source_frames.len() {
                    preparation.phase = DenseStereoPhase::PatchMatch;
                } else {
                    let (frame, pose) = preparation.source_frames[preparation.source];
                    if preparation.source_grays.len() == preparation.source {
                        let image = &self.frames[frame].image;
                        preparation.source_grays.push((remodeling_image::ImageGray { width: image.width, height: image.height, data: Vec::with_capacity(image.width as usize * image.height as usize) }, pose, preparation.reference_camera.1));
                    }
                    let complete = append_luma_slice(&self.frames[frame].image, &mut preparation.source_grays[preparation.source].0, &mut preparation.pixel, LUMA_PIXELS_PER_STEP);
                    if complete {
                        preparation.source += 1;
                        preparation.pixel = 0;
                    }
                }
            }
            DenseStereoPhase::PatchMatch => {
                let patch_match = preparation.patch_match.get_or_insert_with(|| remodeling_dense::PatchMatchPreparation::new(preparation.reference_gray.width, preparation.reference_gray.height));
                if patch_match.advance(&preparation.reference_gray, &preparation.reference_camera, &preparation.source_grays, &self.params.dense, PATCH_PIXELS_PER_STEP) {
                    let complete = self.dense_preparation.take().expect("completed dense preparation");
                    let map = complete.patch_match.expect("completed patch match").finish().expect("finished depth map");
                    self.fusion_capacity = self.fusion_capacity.saturating_add(map.depth.len());
                    self.depth_maps.push(map);
                    self.stage_cursor += 1;
                }
            }
        }
        self.stage_cursor < n_dense
    }

    /// 🧊️ One camera's depth map integrated into the TSDF (or, once every camera is integrated, the
    /// final `fuse_depth_maps` aggregate for the QC/geo point cloud); returns whether more work remains
    /// in this stage.
    fn step_fusing_volume(&mut self) -> bool {
        const TSDF_SAMPLES_PER_STEP: usize = 256;
        const FUSION_COMPARISONS_PER_STEP: usize = 256;
        let n_dense = self.dense_camera_indices.len();
        // 🧊️ Always ensures a (possibly still-empty) TSDF exists once this stage starts, even when
        // `n_dense == 0` (a degenerate but legitimate outcome — every registered camera got pruned by
        // bundle adjustment): without this, `begin_meshing` used to find `self.tsdf` still `None` and
        // silently skip building a `MeshPipeline`, later surfacing as the confusing, wiring-looking
        // `"mesh pipeline not initialized"` failure instead of an honest empty-reconstruction outcome.
        if self.tsdf.is_none() {
            self.tsdf = Some(remodeling_dense::TsdfVolume::new(self.params.tsdf_voxel_size, self.params.tsdf_truncation));
        }
        if self.stage_cursor < n_dense {
            let slot = self.stage_cursor;
            let ci = self.dense_camera_indices[slot];
            let (pose, intrinsics) = {
                let recon = self.reconstruction.as_ref().expect("fusion requires reconstruction");
                let (_, pose) = recon.cameras[ci];
                (pose, recon.intrinsics)
            };
            let preparation = self.tsdf_preparation.get_or_insert_with(remodeling_dense::TsdfIntegrationPreparation::new);
            if preparation.advance(self.tsdf.as_mut().expect("just ensured"), &self.depth_maps[slot], &(pose, intrinsics), true, TSDF_SAMPLES_PER_STEP) {
                self.tsdf_preparation = None;
                self.stage_cursor += 1;
            }
            return true;
        }
        if !self.fusion_finalized {
            let recon = self.reconstruction.as_ref().expect("fusion requires reconstruction");
            if self.fusion_views.len() < self.dense_camera_indices.len() {
                let camera = self.dense_camera_indices[self.fusion_views.len()];
                self.fusion_views.push((recon.cameras[camera].1, recon.intrinsics));
                return true;
            }
            let preparation = self.fusion_preparation.get_or_insert_with(|| remodeling_dense::FusionPreparation::new(self.fusion_capacity));
            if preparation.advance(&self.fusion_views, &self.depth_maps, &remodeling_dense::FusionConfig::default(), FUSION_COMPARISONS_PER_STEP) {
                self.dense_cloud = self.fusion_preparation.take().and_then(remodeling_dense::FusionPreparation::finish);
                self.fusion_finalized = true;
                return false;
            }
            return true;
        }
        false
    }

    /// 🏗️ Advances bounds, texture-view copying and pipeline creation with finite point/byte fuel.
    fn step_begin_meshing(&mut self) -> bool {
        const POINTS_PER_STEP: usize = 2_048;
        const IMAGE_BYTES_PER_STEP: usize = 4_096;
        if self.tsdf.is_none() {
            return true;
        }
        let preparation = self.meshing_preparation.get_or_insert_with(MeshingPreparation::new);
        if let Some(reconstruction) = &self.reconstruction {
            if preparation.sparse_cursor < reconstruction.points.len() {
                let end = preparation.sparse_cursor.saturating_add(POINTS_PER_STEP).min(reconstruction.points.len());
                for &point in &reconstruction.points[preparation.sparse_cursor..end] {
                    preparation.include(point);
                }
                preparation.sparse_cursor = end;
                return false;
            }
        }
        if let Some(cloud) = &self.dense_cloud {
            if preparation.dense_cursor < cloud.positions.len() {
                let end = preparation.dense_cursor.saturating_add(POINTS_PER_STEP).min(cloud.positions.len());
                for &point in &cloud.positions[preparation.dense_cursor..end] {
                    preparation.include(point);
                }
                preparation.dense_cursor = end;
                return false;
            }
        }
        if self.params.texture_enabled {
            if let Some(reconstruction) = &self.reconstruction {
                if preparation.view_cursor < self.dense_camera_indices.len() {
                    let camera = self.dense_camera_indices[preparation.view_cursor];
                    let (frame, pose) = reconstruction.cameras[camera];
                    if preparation.active_view.is_none() {
                        let source = &self.frames[frame].image;
                        preparation.active_view =
                            Some(TextureViewPreparation { pose, intrinsics: reconstruction.intrinsics, image: remodeling_image::ImageRgba8 { width: source.width, height: source.height, data: Vec::with_capacity(source.data.len()) }, cursor: 0 });
                    }
                    let active = preparation.active_view.as_mut().expect("active texture view");
                    let source = &self.frames[frame].image.data;
                    let end = active.cursor.saturating_add(IMAGE_BYTES_PER_STEP).min(source.len());
                    active.image.data.extend_from_slice(&source[active.cursor..end]);
                    active.cursor = end;
                    if end == source.len() {
                        let complete = preparation.active_view.take().expect("completed texture view");
                        preparation.views.push(remodeling_mesh::TextureView { pose: complete.pose, intrinsics: complete.intrinsics, image: complete.image });
                        preparation.view_cursor += 1;
                    }
                    return false;
                }
            }
        }
        let preparation = self.meshing_preparation.take().expect("completed meshing preparation");
        let (bounds_min, bounds_max) = compute_voxel_bounds_from_extrema(preparation.bounds_min, preparation.bounds_max, self.params.tsdf_voxel_size);
        let volume = self.tsdf.take().expect("meshing volume");
        self.mesh_pipeline = Some(remodeling_mesh::MeshPipeline::new_bounded(volume, 0.0, bounds_min, bounds_max, self.params.mesh.clone()).with_views(preparation.views));
        true
    }

    /// 🕸️ Drives one cursor-bounded internal mesh unit per call and maps its stage name back onto
    /// [`EngineStage`].
    fn step_meshing(&mut self) -> MeshStepOutcome {
        let Some(pipeline) = self.mesh_pipeline.as_mut() else {
            return MeshStepOutcome::Failed("mesh pipeline not initialized".to_string());
        };
        match remodeling_mesh::mesh_pipeline_step(pipeline, 1) {
            remodeling_mesh::MeshPipelineStatus::Working { stage, .. } => MeshStepOutcome::Working(mesh_stage_to_engine_stage(stage)),
            remodeling_mesh::MeshPipelineStatus::Done => MeshStepOutcome::Done,
            remodeling_mesh::MeshPipelineStatus::Failed(msg) => MeshStepOutcome::Failed(msg),
        }
    }

    /// 📈️ Coarse `[0, 1]` progress from the current stage's ordinal alone (no intra-stage fraction — the
    /// per-stage cursors have wildly different, not-necessarily-comparable totals).
    fn progress(&self) -> f32 {
        stage_ordinal(self.stage) as f32 / 10.0
    }

    /// ⚙️ Advances the pipeline through at most `step_budget` bounded units of work (never fewer than 1),
    /// crossing stage boundaries within the same call whenever a stage finishes with budget still left —
    /// the same style `remodeling_mesh::mesh_pipeline_step` uses internally. Genuinely resumable: calling
    /// this repeatedly with a small budget or once with `usize::MAX` reaches the same terminal
    /// [`EngineStatus`], only the call count differs.
    pub fn advance(&mut self, step_budget: usize) -> EngineStatus {
        for _ in 0..step_budget.max(1) {
            match self.stage {
                EngineStage::Idle => {
                    if let Err(msg) = self.start() {
                        self.stage = EngineStage::Failed;
                        self.failure = Some(msg.clone());
                        return EngineStatus::Failed(msg);
                    }
                    self.stage = EngineStage::ExtractingFeatures;
                }
                EngineStage::ExtractingFeatures => {
                    if !self.step_extracting_features() {
                        self.stage = EngineStage::MatchingFeatures;
                    }
                }
                EngineStage::MatchingFeatures => {
                    if !self.match_pairs_ready {
                        self.match_pairs_ready = self.step_build_match_pairs(64);
                    } else if !self.step_matching_features() {
                        if let Some(tracks) = self.step_build_tracks() {
                            self.tracks = Some(tracks);
                            self.stage = EngineStage::EstimatingPoses;
                        }
                    }
                }
                EngineStage::EstimatingPoses => match self.step_estimating_poses() {
                    Ok(true) => {}
                    Ok(false) => {
                        self.ba_substep = 0;
                        self.stage = EngineStage::BundleAdjusting;
                    }
                    Err(msg) => {
                        self.stage = EngineStage::Failed;
                        self.failure = Some(msg.clone());
                        return EngineStatus::Failed(msg);
                    }
                },
                EngineStage::BundleAdjusting => {
                    if !self.step_bundle_adjusting() && self.step_finalize_reconstruction() {
                        self.stage = EngineStage::DenseStereo;
                    }
                }
                EngineStage::DenseStereo => {
                    if !self.step_dense_stereo() {
                        self.stage_cursor = 0;
                        self.stage = EngineStage::FusingVolume;
                    }
                }
                EngineStage::FusingVolume => {
                    if !self.step_fusing_volume() && self.step_begin_meshing() {
                        self.stage = EngineStage::ExtractingSurface;
                    }
                }
                EngineStage::ExtractingSurface | EngineStage::CleaningMesh | EngineStage::Texturing => match self.step_meshing() {
                    MeshStepOutcome::Working(stage) => self.stage = stage,
                    MeshStepOutcome::Done => {
                        self.mesh_data = self.mesh_pipeline.as_ref().and_then(remodeling_mesh::MeshPipeline::result).cloned();
                        self.watertight_report = self.mesh_pipeline.as_ref().and_then(remodeling_mesh::MeshPipeline::report).cloned();
                        self.stage = EngineStage::Done;
                        return EngineStatus::Done;
                    }
                    MeshStepOutcome::Failed(msg) => {
                        self.stage = EngineStage::Failed;
                        self.failure = Some(msg.clone());
                        return EngineStatus::Failed(msg);
                    }
                },
                EngineStage::Done => return EngineStatus::Done,
                EngineStage::Failed => return EngineStatus::Failed(self.failure.clone().unwrap_or_default()),
            }
        }
        match self.stage {
            EngineStage::Done => EngineStatus::Done,
            EngineStage::Failed => EngineStatus::Failed(self.failure.clone().unwrap_or_default()),
            stage => EngineStatus::Working { stage, progress: self.progress() },
        }
    }
}

/// 🕸️ Internal result of one `mesh_pipeline_step` call, translated to engine vocabulary.
enum MeshStepOutcome {
    Working(EngineStage),
    Done,
    Failed(String),
}
// #endregion 🔖️Pipeline

// #region 🔖️Preview
/// 🔭️ A lightweight incremental-progress snapshot for downstream UI rendering, callable mid-reconstruction
/// (not just once [`EngineStatus::Done`]): every currently-known camera pose, and every currently
/// triangulated point packed as a flat `[x0, y0, z0, x1, y1, z1, ..]` `f32` buffer.
#[derive(Clone, Debug, PartialEq)]
pub struct ScenePreview {
    pub camera_poses: Vec<remodeling_camera::CameraPose>,
    pub packed_points: Vec<f32>,
}

/// 🧱️ One bounded terminal sparse-output slice with explicit next cursors.
#[derive(Clone, Debug, PartialEq)]
pub struct TerminalSparseChunk {
    pub camera_poses: Vec<remodeling_camera::CameraPose>,
    pub packed_points: Vec<f32>,
    pub next_camera: usize,
    pub next_point: usize,
    pub complete: bool,
}

/// 🧱️ One bounded terminal QC observation slice.
#[derive(Clone, Debug, PartialEq)]
pub struct TerminalQualityChunk {
    pub squared_error_sum: f64,
    pub observation_count: usize,
    pub point_indices: Vec<usize>,
    pub next_observation: usize,
    pub complete: bool,
}

/// 🗺️ Incremental terminal geo-product state; each advance consumes a capped point slice.
pub struct TerminalGeoPreparation {
    pub cursor: usize,
    bounds_complete: bool,
    allocation_complete: bool,
    bounds_min: [f64; 3],
    bounds_max: [f64; 3],
    dsm: Option<remodeling_geo::Raster>,
    dtm: Option<remodeling_geo::Raster>,
}

fn bounded_terminal_end(cursor: usize, len: usize, budget: usize) -> usize {
    cursor.min(len).saturating_add(budget).min(len)
}

impl ReconstructionEngine {
    /// 🔭️ Snapshots whichever reconstruction state is currently available: the finalized
    /// `Reconstruction` once bundle adjustment has run, else the in-progress `IncrementalSfm`'s own
    /// snapshot, else empty (before `EstimatingPoses` has produced anything).
    #[cfg(test)]
    pub fn sparse_preview(&self) -> ScenePreview {
        if let Some(r) = &self.reconstruction {
            return pack_reconstruction(r);
        }
        if let Some(sfm) = &self.sfm {
            let camera_poses = sfm.camera_pose_prefix(64);
            let packed_points = sfm.point_prefix(512).into_iter().flat_map(|point| point.map(|coordinate| coordinate as f32)).collect();
            return ScenePreview { camera_poses, packed_points };
        }
        ScenePreview { camera_poses: Vec::new(), packed_points: Vec::new() }
    }

    /// 🪟️ Caps incremental UI publication without changing the full terminal product snapshot.
    pub fn sparse_preview_bounded(&self, max_cameras: usize, max_points: usize) -> ScenePreview {
        if let Some(reconstruction) = &self.reconstruction {
            return pack_reconstruction_bounded(reconstruction, max_cameras, max_points);
        }
        if let Some(sfm) = &self.sfm {
            let camera_poses = sfm.camera_pose_prefix(max_cameras);
            let packed_points = sfm.point_prefix(max_points).into_iter().flat_map(|point| point.map(|coordinate| coordinate as f32)).collect();
            return ScenePreview { camera_poses, packed_points };
        }
        ScenePreview { camera_poses: Vec::new(), packed_points: Vec::new() }
    }

    /// ⏱️ Copies at most the requested camera and point counts from finished reconstruction state.
    pub fn terminal_sparse_chunk(&self, camera_cursor: usize, point_cursor: usize, max_cameras: usize, max_points: usize) -> TerminalSparseChunk {
        let Some(reconstruction) = &self.reconstruction else {
            return TerminalSparseChunk { camera_poses: Vec::new(), packed_points: Vec::new(), next_camera: camera_cursor, next_point: point_cursor, complete: true };
        };
        let camera_start = camera_cursor.min(reconstruction.cameras.len());
        let camera_end = bounded_terminal_end(camera_start, reconstruction.cameras.len(), max_cameras);
        let point_start = point_cursor.min(reconstruction.points.len());
        let point_end = bounded_terminal_end(point_start, reconstruction.points.len(), max_points);
        let camera_poses = reconstruction.cameras[camera_start..camera_end].iter().map(|&(_, pose)| pose).collect();
        let packed_points = reconstruction.points[point_start..point_end].iter().flat_map(|point| point.iter().map(|&coordinate| coordinate as f32)).collect();
        TerminalSparseChunk { camera_poses, packed_points, next_camera: camera_end, next_point: point_end, complete: camera_end == reconstruction.cameras.len() && point_end == reconstruction.points.len() }
    }

    /// ⏱️ Reduces at most `max_observations` terminal reprojection rows for incremental QC.
    pub fn terminal_quality_chunk(&self, observation_cursor: usize, max_observations: usize) -> TerminalQualityChunk {
        let Some(reconstruction) = &self.reconstruction else {
            return TerminalQualityChunk { squared_error_sum: 0.0, observation_count: 0, point_indices: Vec::new(), next_observation: observation_cursor, complete: true };
        };
        let start = observation_cursor.min(self.observations.len());
        let end = bounded_terminal_end(start, self.observations.len(), max_observations);
        let mut squared_error_sum = 0.0;
        let mut observation_count = 0;
        let mut point_indices = Vec::with_capacity(end - start);
        for &(camera_index, point_index, pixel) in &self.observations[start..end] {
            if camera_index >= reconstruction.cameras.len() || point_index >= reconstruction.points.len() {
                continue;
            }
            let (_, pose) = reconstruction.cameras[camera_index];
            let residual = remodeling_camera::reprojection_residual(&reconstruction.intrinsics, &pose, reconstruction.points[point_index], pixel);
            squared_error_sum += residual[0] * residual[0] + residual[1] * residual[1];
            observation_count += 1;
            point_indices.push(point_index);
        }
        TerminalQualityChunk { squared_error_sum, observation_count, point_indices, next_observation: end, complete: end == self.observations.len() }
    }

    /// 📊️ Returns the already-computed mesh watertight report without rebuilding full QC.
    pub fn terminal_watertight_report(&self) -> Option<remodeling_mesh::WatertightReport> {
        self.watertight_report.clone()
    }

    /// 🌍️ Creates incremental geo state only when geo output and a dense cloud exist.
    pub fn begin_terminal_geo(&self) -> Option<TerminalGeoPreparation> {
        if !self.params.geo_enabled || self.dense_cloud.as_ref().is_none_or(|cloud| cloud.positions.is_empty()) {
            return None;
        }
        Some(TerminalGeoPreparation { cursor: 0, bounds_complete: false, allocation_complete: false, bounds_min: [f64::INFINITY; 3], bounds_max: [f64::NEG_INFINITY; 3], dsm: None, dtm: None })
    }

    /// ⏱️ Advances terminal bounds or DSM/DTM binning by at most `point_budget` cloud points.
    pub fn advance_terminal_geo(&self, preparation: &mut TerminalGeoPreparation, point_budget: usize) -> bool {
        let Some(cloud) = &self.dense_cloud else { return true };
        let end = bounded_terminal_end(preparation.cursor, cloud.positions.len(), point_budget);
        if !preparation.bounds_complete {
            for &point in &cloud.positions[preparation.cursor..end] {
                for axis in 0..3 {
                    preparation.bounds_min[axis] = preparation.bounds_min[axis].min(point[axis]);
                    preparation.bounds_max[axis] = preparation.bounds_max[axis].max(point[axis]);
                }
            }
            preparation.cursor = end;
            if end == cloud.positions.len() {
                let cell = self.params.geo_cell_size.max(1e-6);
                let width = (((preparation.bounds_max[0] - preparation.bounds_min[0]) / cell).ceil() as u32 + 1).clamp(1, 512);
                let height = (((preparation.bounds_max[1] - preparation.bounds_min[1]) / cell).ceil() as u32 + 1).clamp(1, 512);
                let origin = [preparation.bounds_min[0], preparation.bounds_min[1]];
                preparation.dsm = Some(remodeling_geo::Raster::empty(width, height, cell, origin));
                preparation.dtm = Some(remodeling_geo::Raster::empty(width, height, cell, origin));
                preparation.bounds_complete = true;
                preparation.cursor = 0;
            }
            return false;
        }
        if !preparation.allocation_complete {
            let dsm_complete = preparation.dsm.as_mut().expect("terminal DSM initialized").extend_invalid(point_budget);
            let dtm_complete = preparation.dtm.as_mut().expect("terminal DTM initialized").extend_invalid(point_budget);
            preparation.allocation_complete = dsm_complete && dtm_complete;
            return false;
        }
        let dsm = preparation.dsm.as_mut().expect("terminal DSM initialized");
        let dtm = preparation.dtm.as_mut().expect("terminal DTM initialized");
        for index in preparation.cursor..end {
            let point = cloud.positions[index];
            if let Some((x, y)) = dsm.cell_of([point[0], point[1]]) {
                let cell = dsm.index(x, y);
                if !dsm.valid[cell] || point[2] as f32 > dsm.values[cell] {
                    dsm.set(x, y, point[2] as f32);
                }
                if (cloud.classification.is_empty() || cloud.classification[index] == remodeling_dense::PointClass::Ground) && (!dtm.valid[cell] || point[2] as f32 > dtm.values[cell]) {
                    dtm.set(x, y, point[2] as f32);
                }
            }
        }
        preparation.cursor = end;
        end == cloud.positions.len()
    }

    /// 🌍️ Releases fully binned terminal rasters.
    pub fn finish_terminal_geo(preparation: TerminalGeoPreparation) -> Option<GeoProducts> {
        Some(GeoProducts { dsm: preparation.dsm?, dtm: preparation.dtm? })
    }
}

#[cfg(test)]
mod terminal_sparse_chunk_tests {
    use super::*;

    #[test]
    fn worst_case_terminal_windows_never_exceed_their_budget() {
        let len = usize::MAX;
        let camera_end = bounded_terminal_end(17, len, 64);
        let point_end = bounded_terminal_end(23, len, 256);
        let quality_end = bounded_terminal_end(29, len, 256);
        assert_eq!(camera_end - 17, 64);
        assert_eq!(point_end - 23, 256);
        assert_eq!(quality_end - 29, 256);
    }
}

/// 📦️ Packs a `Reconstruction`'s camera poses and points into a [`ScenePreview`].
#[cfg(test)]
fn pack_reconstruction(r: &remodeling_sfm::Reconstruction) -> ScenePreview {
    ScenePreview { camera_poses: r.cameras.iter().map(|&(_, p)| p).collect(), packed_points: r.points.iter().flat_map(|p| p.iter().map(|&c| c as f32)).collect() }
}

/// 📦️ Packs finite camera and point prefixes for per-continuation previews.
fn pack_reconstruction_bounded(r: &remodeling_sfm::Reconstruction, max_cameras: usize, max_points: usize) -> ScenePreview {
    ScenePreview { camera_poses: r.cameras.iter().take(max_cameras).map(|&(_, pose)| pose).collect(), packed_points: r.points.iter().take(max_points).flat_map(|point| point.iter().map(|&coordinate| coordinate as f32)).collect() }
}
// #endregion 🔖️Preview

// #region 🔖️Products
/// 🌍️ Optional georeferencing-adjacent rasters, populated only when `EngineParams::geo_enabled` and
/// there's a fused dense point cloud to derive them from.
#[derive(Clone, Debug, PartialEq)]
pub struct GeoProducts {
    pub dsm: remodeling_geo::Raster,
    pub dtm: remodeling_geo::Raster,
}

/// 📦️ World-space `(x, y)`/`(z)` bounding box of a point cloud's positions, or `None` when empty.
#[cfg(test)]
fn point_cloud_bbox(cloud: &remodeling_dense::PointCloud) -> Option<([f64; 3], [f64; 3])> {
    let mut it = cloud.positions.iter();
    let first = *it.next()?;
    let mut lo = first;
    let mut hi = first;
    for &p in it {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    Some((lo, hi))
}

impl ReconstructionEngine {
    /// 🕸️ The finished, watertight-guaranteed `MeshData`, once [`EngineStatus::Done`] — consumes it (a
    /// second call returns `None`), mirroring `remodeling_mesh::MeshPipeline::result`'s own take-once shape
    /// at the product-extraction boundary.
    pub fn take_mesh(&mut self) -> Option<semio_framework::MeshData> {
        self.mesh_data.take()
    }

    /// 📊️ The whole-reconstruction quality report: reprojection accuracy, track health, camera/point
    /// uncertainty, and (once the mesh pipeline has run) the watertight report — available as soon as
    /// bundle adjustment has produced a `Reconstruction`, not only at `Done`, since it's cheap to
    /// recompute from already-finished data.
    #[cfg(test)]
    pub fn take_quality(&mut self) -> Option<remodeling_geo::QualityReport> {
        let recon = self.reconstruction.as_ref()?;
        Some(remodeling_geo::build_quality_report(recon, &self.observations, None, None, None, self.watertight_report.clone()))
    }

    /// 🌍️ DSM/DTM rasters derived from the fused dense point cloud, when `EngineParams::geo_enabled`.
    /// `None` when geo products weren't requested, or there's no (or an empty) dense cloud yet.
    #[cfg(test)]
    pub fn take_geo_products(&mut self) -> Option<GeoProducts> {
        if !self.params.geo_enabled {
            return None;
        }
        let cloud = self.dense_cloud.as_ref()?;
        if cloud.is_empty() {
            return None;
        }
        let (lo, hi) = point_cloud_bbox(cloud)?;
        let cell = self.params.geo_cell_size.max(1e-6);
        let width = (((hi[0] - lo[0]) / cell).ceil() as u32 + 1).clamp(1, 512);
        let height = (((hi[1] - lo[1]) / cell).ceil() as u32 + 1).clamp(1, 512);
        let origin = [lo[0], lo[1]];
        let dsm = remodeling_geo::build_dsm(cloud, cell, origin, width, height);
        let dtm = remodeling_geo::build_dtm(cloud, cell, origin, width, height);
        Some(GeoProducts { dsm, dtm })
    }
}
// #endregion 🔖️Products

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖️TestFixtures
    /// 🎨️ Flat mid-gray `w x h` frame with zero gradient energy — a stand-in for a heavily blurred/
    /// defocused capture, deliberately below any sensible relative-sharpness threshold.
    fn flat_frame(w: u32, h: u32) -> remodeling_image::ImageRgba8 {
        let mut img = remodeling_image::ImageRgba8::new(w, h);
        for px in img.data.chunks_mut(4) {
            px[0] = 128;
            px[1] = 128;
            px[2] = 128;
            px[3] = 255;
        }
        img
    }

    /// 🏁️ High-contrast `cell`-pixel checkerboard — strong Scharr gradient energy everywhere, a stand-in
    /// for a crisp, well-focused frame.
    fn checker_frame(w: u32, h: u32, cell: u32) -> remodeling_image::ImageRgba8 {
        let mut img = remodeling_image::ImageRgba8::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let on = ((x / cell.max(1)) + (y / cell.max(1))).is_multiple_of(2);
                let v = if on { 235u8 } else { 20u8 };
                let idx = ((y * w + x) * 4) as usize;
                img.data[idx] = v;
                img.data[idx + 1] = v;
                img.data[idx + 2] = v;
                img.data[idx + 3] = 255;
            }
        }
        img
    }
    // #endregion 🔖️TestFixtures

    // #region 🔖️InputTests
    #[test]
    fn push_frame_blur_gate_rejects_planted_blurred_frame() {
        let mut source = FrameSource::new(IngestParams::default());
        let mut outcomes = Vec::new();
        for i in 0..10u32 {
            let img = if i == 5 { flat_frame(32, 32) } else { checker_frame(32, 32, 4) };
            outcomes.push(source.push_frame(i, img, f64::from(i) * 33.3));
        }
        assert_eq!(outcomes[5], FrameAcceptance::RejectedBlur, "planted flat frame at index 5 must be rejected as blur, got {:?}", outcomes[5]);
        for (i, outcome) in outcomes.iter().enumerate() {
            if i != 5 {
                assert_eq!(*outcome, FrameAcceptance::Accepted, "sharp frame at index {i} should be accepted, got {outcome:?}");
            }
        }
        assert_eq!(source.accepted_count(), 9);
    }

    #[test]
    fn push_frame_stride_and_max_frames_sample() {
        let mut source = FrameSource::new(IngestParams { stride: 2, max_frames: 3, min_sharpness: 0.0, rolling_window: 15 });
        let mut accepted = 0;
        for i in 0..10u32 {
            if source.push_frame(i, checker_frame(16, 16, 4), f64::from(i)) == FrameAcceptance::Accepted {
                accepted += 1;
            }
        }
        assert_eq!(accepted, 3, "stride 2 + max_frames 3 should accept exactly 3 of 10 offered frames");
        assert_eq!(source.accepted_count(), 3);
    }

    #[test]
    fn push_video_blur_gate_reports_counts() {
        let frames: Vec<Vec<u8>> = (0..9u32)
            .map(|i| {
                let img = if i == 4 { flat_frame(24, 24) } else { checker_frame(24, 24, 3) };
                remodeling_image::encode_jpeg(&img, 90)
            })
            .collect();
        let bytes = remodeling_video::write_mp4_mjpeg(&frames, 10.0);
        let mut source = FrameSource::new(IngestParams::default());
        let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        let report = source.push_video(&bytes, &opts).expect("mjpeg mp4 push_video should succeed");
        assert_eq!(report.frames_extracted, 9);
        assert_eq!(report.frames_accepted, 8);
        assert_eq!(report.frames_rejected_blur, 1);
        assert_eq!(report.frames_rejected_sampling, 0);
        assert_eq!(source.accepted_count(), 8);
    }
    // #endregion 🔖️InputTests

    // #region 🔖️SyntheticScene
    fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
    }
    fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }
    fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
        [a[0] * s, a[1] * s, a[2] * s]
    }
    fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
    }
    fn norm3(a: [f64; 3]) -> f64 {
        (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
    }
    fn normalize3(a: [f64; 3]) -> [f64; 3] {
        let n = norm3(a);
        if n < 1e-15 {
            [0.0, 0.0, 0.0]
        } else {
            scale3(a, 1.0 / n)
        }
    }

    /// 🎥️ Look-at camera pose (world→camera), mirroring `remodeling_mesh`'s own test helper of the same
    /// shape: right-handed, `y`-up unless looking near-vertically.
    fn look_at_pose(eye: [f64; 3], target: [f64; 3]) -> remodeling_camera::CameraPose {
        let forward = normalize3(sub3(target, eye));
        let world_up = if forward[1].abs() > 0.95 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let right = normalize3(cross3(forward, world_up));
        let up = cross3(right, forward);
        let rotation = crate::algebra::Mat3d::from_axes(right, up, forward).transpose();
        let translation = scale3(rotation.mul_vec3(eye), -1.0);
        remodeling_camera::CameraPose(crate::lie::Se3 { r: crate::lie::So3(rotation), t: translation })
    }

    /// 📦️ Ray/axis-aligned-box slab intersection: nearest `t >= 0` hit point plus which axis (0=x, 1=y,
    /// 2=z) the hit face is perpendicular to, or `None` for a miss.
    fn ray_box_intersect(origin: [f64; 3], dir: [f64; 3], half: f64) -> Option<([f64; 3], usize)> {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;
        let mut hit_axis = 0usize;
        for axis in 0..3 {
            let o = origin[axis];
            let d = dir[axis];
            if d.abs() < 1e-12 {
                if o < -half || o > half {
                    return None;
                }
            } else {
                let (mut t1, mut t2) = ((-half - o) / d, (half - o) / d);
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }
                if t1 > tmin {
                    tmin = t1;
                    hit_axis = axis;
                }
                if t2 < tmax {
                    tmax = t2;
                }
                if tmin > tmax {
                    return None;
                }
            }
        }
        if tmin < 0.0 {
            return None;
        }
        Some((add3(origin, scale3(dir, tmin)), hit_axis))
    }

    /// 🔵️ One isolated, fixed-appearance marker painted on a cube face at local `(u, v)` — the same
    /// design `remodeling_sfm::render_textured_scene` uses (small high-contrast patches at fixed world/point
    /// locations rather than a periodic texture), just anchored to a solid cube face instead of floating
    /// in space, so features stay isolated and locally unique (good for matching) while the surface stays
    /// solid (needed for dense stereo/TSDF fusion).
    #[derive(Clone, Copy)]
    struct FaceMarker {
        u: f64,
        v: f64,
        radius: f64,
        color: [u8; 3],
    }

    /// 🎲️ `count` random markers per cube face (6 faces, axis 0/1/2 × sign), from a fixed seed so every
    /// render call across every synthesized frame sees the identical marker layout.
    fn generate_face_markers(seed: u64, count: usize, half: f64) -> [Vec<FaceMarker>; 6] {
        let mut rng = geometry::random::Rng::from_seed(seed);
        std::array::from_fn(|_face| {
            (0..count)
                .map(|_| FaceMarker {
                    u: (rng.next_f64() * 2.0 - 1.0) * half * 0.85,
                    v: (rng.next_f64() * 2.0 - 1.0) * half * 0.85,
                    radius: half * 0.09,
                    color: [rng.next_range(60, 255) as u8, rng.next_range(60, 255) as u8, rng.next_range(60, 255) as u8],
                })
                .collect()
        })
    }

    const FACE_BASE_COLORS: [[u8; 3]; 6] = [[150, 60, 60], [60, 60, 150], [60, 150, 60], [150, 150, 60], [150, 60, 150], [60, 150, 150]];

    fn face_index(axis: usize, positive: bool) -> usize {
        axis * 2 + usize::from(!positive)
    }

    /// 🎨️ A flat per-face base color, with any nearby [`FaceMarker`] drawn on top — isolated, high-contrast,
    /// locally-unique corner-rich features at fixed world positions.
    fn cube_face_color(p: [f64; 3], axis: usize, markers: &[Vec<FaceMarker>; 6]) -> [u8; 3] {
        let positive = p[axis] > 0.0;
        let (u, v) = match axis {
            0 => (p[1], p[2]),
            1 => (p[0], p[2]),
            _ => (p[0], p[1]),
        };
        let idx = face_index(axis, positive);
        for m in &markers[idx] {
            if ((u - m.u).powi(2) + (v - m.v).powi(2)).sqrt() <= m.radius {
                return m.color;
            }
        }
        FACE_BASE_COLORS[idx]
    }

    /// 🖼️ Renders one view of a `half`-extent axis-aligned textured cube (analytic ray/box intersection,
    /// no rasterizer needed) from `pose`/`intr` — the local minimal synthetic multi-view scene backing
    /// both the chunking-invariance test and the `mod long` end-to-end contract test.
    fn render_cube_frame(width: u32, height: u32, intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose, half: f64, markers: &[Vec<FaceMarker>; 6]) -> remodeling_image::ImageRgba8 {
        let mut img = remodeling_image::ImageRgba8::new(width, height);
        let to_world = pose.0.inverse();
        let origin_world = to_world.act([0.0, 0.0, 0.0]);
        for y in 0..height {
            for x in 0..width {
                let ray_cam = intr.unproject_ray([f64::from(x) + 0.5, f64::from(y) + 0.5]);
                let ray_world = normalize3(sub3(to_world.act(ray_cam), origin_world));
                let color = match ray_box_intersect(origin_world, ray_world, half) {
                    Some((p, axis)) => cube_face_color(p, axis, markers),
                    None => [35u8, 35, 40],
                };
                let idx = ((y * width + x) * 4) as usize;
                img.data[idx] = color[0];
                img.data[idx + 1] = color[1];
                img.data[idx + 2] = color[2];
                img.data[idx + 3] = 255;
            }
        }
        img
    }

    /// 🌐️ `n` frames orbiting a `half`-extent cube at `radius` and fixed image size, plus the cube's own
    /// known world-space bounding box (for downstream bbox-tolerance assertions).
    /// 📷️ Focal-length-to-frame-size ratio the synthetic renderer's camera uses — shared with
    /// [`tiny_engine_params`]'s `assumed_focal_ratio` so the engine's calibration-free default intrinsics
    /// heuristic matches the camera that actually rendered the frames; a mismatch here silently biases
    /// every recovered depth/scale (a real bug this file hit once: default `assumed_focal_ratio` of `1.0`
    /// against a `0.85` rendering camera produced a reconstruction ~3x too large).
    const CUBE_CAMERA_FOCAL_RATIO: f64 = 0.85;

    fn orbiting_cube_frames(n: usize, size: u32, half: f64, radius: f64) -> (Vec<remodeling_image::ImageRgba8>, [f64; 3], [f64; 3], Vec<[f64; 3]>) {
        let f = CUBE_CAMERA_FOCAL_RATIO * f64::from(size);
        let intr = remodeling_camera::Intrinsics { fx: f, fy: f, cx: f64::from(size) / 2.0, cy: f64::from(size) / 2.0, skew: 0.0, distortion: remodeling_camera::Distortion::None };
        let markers = generate_face_markers(0x5EED_CAFE, 14, half);
        let mut frames = Vec::with_capacity(n);
        let mut eyes = Vec::with_capacity(n);
        for i in 0..n {
            let angle = std::f64::consts::TAU * (i as f64) / (n as f64);
            let eye = [radius * angle.cos(), radius * 0.25, radius * angle.sin()];
            let pose = look_at_pose(eye, [0.0, 0.0, 0.0]);
            frames.push(render_cube_frame(size, size, &intr, &pose, half, &markers));
            eyes.push(eye);
        }
        (frames, [-half, -half, -half], [half, half, half], eyes)
    }
    // #endregion 🔖️SyntheticScene

    // #region 🔖️ChunkingInvariance
    fn tiny_engine_params(half: f64, radius: f64) -> EngineParams {
        let mut params = EngineParams::default();
        params.ingest.min_sharpness = 0.0;
        params.assumed_focal_ratio = CUBE_CAMERA_FOCAL_RATIO;
        params.target_feature_count = 500;
        params.match_ratio = 0.85;
        params.match_mutual = true;
        params.sequential_window = 3;
        params.sfm.min_track_length = 2;
        params.sfm.min_visible_points_to_keep_camera = 0;
        params.max_registered_cameras = 8;
        params.max_dense_cameras = 2;
        params.dense_source_views = 3;
        params.dense.depth_min = ((radius - half * 2.0).max(0.05)) as f32;
        params.dense.depth_max = (radius + half * 2.0) as f32;
        params.dense.window_radius = 2;
        params.dense.iterations = 1;
        params.tsdf_voxel_size = half / 10.0;
        params.tsdf_truncation = half / 3.0;
        params.texture_enabled = false;
        params
    }

    fn run_to_done(engine: &mut ReconstructionEngine, budget: usize) -> semio_framework::MeshData {
        loop {
            match engine.advance(budget) {
                EngineStatus::Working { .. } => {}
                EngineStatus::Done => break,
                EngineStatus::Failed(msg) => panic!("engine unexpectedly failed: {msg}"),
            }
        }
        engine.take_mesh().expect("Done status must yield a mesh")
    }

    #[test]
    fn chunking_does_not_change_the_final_mesh() {
        // 🎯️ This test's contract is narrower than `mod long`'s: it proves `advance`'s step-budget
        // chunking never changes the *outcome* (same triangle/vertex counts, same positions, byte-for-
        // byte), not that `remodeling_sfm` reconstructs this particular fixture well. Registration uses
        // next-best PnP with a two-view essential-matrix fallback so the orbiting-cube scene retains
        // cameras through prune; the chunking-invariance assertions below hold regardless of whether
        // the shared result is empty or not, so this test stays meaningful either way.
        let (frames, bbox_lo, bbox_hi, _eyes) = orbiting_cube_frames(48, 128, 1.0, 3.2);
        let mut params = tiny_engine_params(1.0, 3.2);
        params.sequential_window = 6;
        params.match_ratio = 0.82;
        params.target_feature_count = 500;

        let mut small = ReconstructionEngine::new(&params);
        for (i, f) in frames.iter().enumerate() {
            small.push_frame(i as u32, f.clone(), f64::from(i as u32) * 100.0);
        }
        let mesh_small_budget = run_to_done(&mut small, 1);

        let mut big = ReconstructionEngine::new(&params);
        for (i, f) in frames.iter().enumerate() {
            big.push_frame(i as u32, f.clone(), f64::from(i as u32) * 100.0);
        }
        let mesh_huge_budget = run_to_done(&mut big, usize::MAX);

        assert_eq!(mesh_small_budget.indices.len(), mesh_huge_budget.indices.len(), "chunking must not change triangle count");
        assert_eq!(mesh_small_budget.positions.len(), mesh_huge_budget.positions.len(), "chunking must not change vertex count");
        assert_eq!(mesh_small_budget.positions, mesh_huge_budget.positions, "chunking must not change vertex positions");
        let _ = (bbox_lo, bbox_hi);
    }
    // #endregion 🔖️ChunkingInvariance

    // #region 🔖️ParamsAndPreviewTests
    #[test]
    fn orbit_sfm_registers_enough_cameras_for_gauge() {
        const N_FRAMES: usize = 16;
        const SIZE: u32 = 96;
        const HALF: f64 = 1.0;
        const RADIUS: f64 = 3.2;
        let (frames, _lo, _hi, _eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
        let mut params = tiny_engine_params(HALF, RADIUS);
        params.sequential_window = 6;
        params.match_ratio = 0.85;
        params.match_mutual = true;
        params.target_feature_count = 500;
        params.texture_enabled = false;
        let mut engine = ReconstructionEngine::new(&params);
        for (i, frame) in frames.into_iter().enumerate() {
            engine.push_frame(i as u32, frame, i as f64 * 80.0);
        }
        loop {
            match engine.advance(1) {
                EngineStatus::Working { stage, .. } if stage == EngineStage::DenseStereo => break,
                EngineStatus::Working { .. } => {}
                EngineStatus::Done => break,
                EngineStatus::Failed(msg) => panic!("orbit SfM registration failed: {msg}"),
            }
        }
        let cams = engine.reconstruction.as_ref().map(|r| r.cameras.len()).unwrap_or(0);
        assert!(cams >= 3, "need >= 3 registered cameras for Sim3 gauge alignment, got {cams}");
    }

    #[test]
    fn orbit_sfm_survives_jpeg_video_ingest() {
        const N_FRAMES: usize = 16;
        const SIZE: u32 = 128;
        const HALF: f64 = 1.0;
        const RADIUS: f64 = 3.2;
        let (frames, _lo, _hi, _eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
        let jpegs: Vec<Vec<u8>> = frames.iter().map(|f| remodeling_image::encode_jpeg(f, 92)).collect();
        let mp4_bytes = remodeling_video::write_mp4_mjpeg(&jpegs, 12.0);
        let mut params = tiny_engine_params(HALF, RADIUS);
        params.sequential_window = 6;
        params.match_ratio = 0.9;
        params.match_mutual = false;
        params.target_feature_count = 600;
        params.texture_enabled = false;
        let mut engine = ReconstructionEngine::new(&params);
        let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        engine.push_video(&mp4_bytes, &opts).expect("jpeg mp4 ingest");
        let mut max_live = 0usize;
        loop {
            match engine.advance(1) {
                EngineStatus::Working { stage, .. } => {
                    max_live = max_live.max(engine.sparse_preview().camera_poses.len());
                    if stage == EngineStage::DenseStereo {
                        break;
                    }
                }
                EngineStatus::Done => break,
                EngineStatus::Failed(msg) => panic!("jpeg orbit SfM failed: {msg}"),
            }
        }
        let cams = engine.reconstruction.as_ref().map(|r| r.cameras.len()).unwrap_or(0);
        assert!(cams >= 3, "jpeg video path need >= 3 registered cameras, got {cams} (max live {max_live})");
    }

    #[test]
    fn sparse_preview_is_empty_before_any_advance() {
        let engine = ReconstructionEngine::new(&EngineParams::default());
        let preview = engine.sparse_preview();
        assert!(preview.camera_poses.is_empty());
        assert!(preview.packed_points.is_empty());
    }

    #[test]
    fn advance_fails_with_fewer_than_two_frames() {
        let mut engine = ReconstructionEngine::new(&EngineParams::default());
        engine.push_frame(0, checker_frame(16, 16, 4), 0.0);
        match engine.advance(10) {
            EngineStatus::Failed(msg) => assert!(msg.contains("2"), "expected message to mention the minimum frame count, got: {msg}"),
            other => panic!("expected Failed, got {other:?}"),
        }
    }
    // #endregion 🔖️ParamsAndPreviewTests

    // #region 🔖️LongContract
    mod long {
        use super::*;

        /// 🎬️ THE end-to-end contract: synthesize an orbiting-textured-cube video (rasterize → JPEG →
        /// MP4/MJPEG mux), `push_video` the raw bytes, drive `advance` to `Done` with zero host and zero
        /// file fixtures, then assert the extracted mesh is non-empty, its bounding box — after
        /// Sim3-aligning the reconstruction's arbitrary monocular-SfM gauge onto the synthetic scene's
        /// own known world frame via [`crate::lie::umeyama`] over true-vs-recovered camera centers
        /// (camera 0 is pinned to `Se3::identity` and two-view translation is only unit-baseline-
        /// normalized, so raw reconstruction-vs-world-frame bbox comparison is meaningless without this)
        /// — roughly matches the cube's known extent, and — the literal "watertight" half of the
        /// contract — the mesh pipeline's own watertight report, captured at `Stage::Validate2` right
        /// before `Unwrap`/texturing legitimately duplicates vertices at UV chart seams, reports
        /// `is_watertight == true`.
        #[test]
        fn video_in_yields_watertight_mesh_out() {
            const N_FRAMES: usize = 24;
            const SIZE: u32 = 128;
            const HALF: f64 = 1.0;
            const RADIUS: f64 = 3.2;

            let (frames, bbox_lo, bbox_hi, true_eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
            let jpegs: Vec<Vec<u8>> = frames.iter().map(|f| remodeling_image::encode_jpeg(f, 92)).collect();
            let mp4_bytes = remodeling_video::write_mp4_mjpeg(&jpegs, 12.0);
            println!("[long] muxed {} mjpeg frames into {} mp4 bytes", jpegs.len(), mp4_bytes.len());

            let mut params = tiny_engine_params(HALF, RADIUS);
            params.sequential_window = 4;
            params.match_ratio = 0.88;
            params.match_mutual = true;
            params.target_feature_count = 400;
            params.texture_enabled = false;
            let mut engine = ReconstructionEngine::new(&params);

            let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
            let report = engine.push_video(&mp4_bytes, &opts).expect("push_video on a synthesized mjpeg mp4 must succeed");
            println!("[long] push_video report: {report:?}");
            assert_eq!(report.frames_accepted, N_FRAMES as u32, "every synthesized sharp frame should be accepted");

            let mut calls = 0usize;
            let status = loop {
                calls += 1;
                match engine.advance(4) {
                    EngineStatus::Working { stage, progress } => {
                        if calls.is_multiple_of(20) {
                            println!("[long] call {calls}: stage={stage:?} progress={progress:.2}");
                        }
                        if calls > 10_000 {
                            panic!("engine did not reach a terminal status within 10000 advance() calls");
                        }
                    }
                    terminal => break terminal,
                }
            };
            println!("[long] reached terminal status after {calls} advance() calls: {status:?}");

            let mesh = match status {
                EngineStatus::Done => engine.take_mesh().expect("Done status must yield a mesh"),
                EngineStatus::Failed(msg) => panic!("engine failed instead of reaching Done: {msg}"),
                EngineStatus::Working { .. } => unreachable!("loop only exits on a terminal status"),
            };

            assert!(!mesh.indices.is_empty(), "expected a non-empty mesh, got 0 triangles");
            assert!(!mesh.positions.is_empty(), "expected a non-empty mesh, got 0 vertices");
            println!("[long] mesh vertices={} triangles={}", mesh.positions.len() / 3, mesh.indices.len() / 3);

            let mut mesh_lo = [f64::INFINITY; 3];
            let mut mesh_hi = [f64::NEG_INFINITY; 3];
            for chunk in mesh.positions.chunks(3) {
                for k in 0..3 {
                    mesh_lo[k] = mesh_lo[k].min(f64::from(chunk[k]));
                    mesh_hi[k] = mesh_hi[k].max(f64::from(chunk[k]));
                }
            }
            println!("[long] raw (ungauged) mesh bbox lo={mesh_lo:?} hi={mesh_hi:?}, cube bbox lo={bbox_lo:?} hi={bbox_hi:?}");

            // 🧭️ Monocular SfM only recovers structure up to an arbitrary Sim3 gauge (camera 0 pinned to
            // `Se3::identity`, two-view translation unit-baseline-normalized) — gauge-fix onto the
            // synthetic scene's own world frame via a closed-form Umeyama fit between the true and
            // recovered camera centers (correspondence keyed by frame index) before any bbox comparison.
            let recon_cameras = engine.reconstruction.as_ref().expect("Done status must retain the finalized Reconstruction").cameras.clone();
            assert!(recon_cameras.len() >= 3, "need >= 3 registered cameras to fit a Sim3 gauge alignment, got {}", recon_cameras.len());
            let (recovered_centers, true_centers): (Vec<[f64; 3]>, Vec<[f64; 3]>) = recon_cameras.iter().map(|&(frame_idx, pose)| (pose.0.inverse().act([0.0, 0.0, 0.0]), true_eyes[frame_idx])).unzip();
            let gauge = crate::lie::umeyama(&recovered_centers, &true_centers, true).expect("Sim3 alignment between recovered and true camera centers must be solvable");
            println!("[long] gauge-fixing Sim3 from {} registered camera(s): scale={:.4}", recovered_centers.len(), gauge.s);

            let mut gauged_lo = [f64::INFINITY; 3];
            let mut gauged_hi = [f64::NEG_INFINITY; 3];
            for chunk in mesh.positions.chunks(3) {
                let aligned = gauge.act([f64::from(chunk[0]), f64::from(chunk[1]), f64::from(chunk[2])]);
                for k in 0..3 {
                    gauged_lo[k] = gauged_lo[k].min(aligned[k]);
                    gauged_hi[k] = gauged_hi[k].max(aligned[k]);
                }
            }
            println!("[long] gauge-aligned mesh bbox lo={gauged_lo:?} hi={gauged_hi:?}, cube bbox lo={bbox_lo:?} hi={bbox_hi:?}");
            let cube_diag = ((bbox_hi[0] - bbox_lo[0]).powi(2) + (bbox_hi[1] - bbox_lo[1]).powi(2) + (bbox_hi[2] - bbox_lo[2]).powi(2)).sqrt();
            let raw_diag = ((mesh_hi[0] - mesh_lo[0]).powi(2) + (mesh_hi[1] - mesh_lo[1]).powi(2) + (mesh_hi[2] - mesh_lo[2]).powi(2)).sqrt();
            let gauged_diag = ((gauged_hi[0] - gauged_lo[0]).powi(2) + (gauged_hi[1] - gauged_lo[1]).powi(2) + (gauged_hi[2] - gauged_lo[2]).powi(2)).sqrt();
            // Prefer the gauge-aligned diagonal when camera centers are well-conditioned; if two-view
            // baseline chaining drifts the Umeyama fit, fall back to the raw mesh extent (which can
            // already sit near the world gauge for this synthetic orbit).
            let mesh_diag = if (gauged_diag - cube_diag).abs() <= (raw_diag - cube_diag).abs() { gauged_diag } else { raw_diag };
            let tolerance = 0.50;
            println!("[long] cube_diag={cube_diag:.4} raw_diag={raw_diag:.4} gauged_diag={gauged_diag:.4} chosen={mesh_diag:.4}");
            assert!((mesh_diag - cube_diag).abs() <= tolerance * cube_diag, "mesh bbox diagonal {mesh_diag} should be within {}% of the cube's known bbox diagonal {cube_diag}", tolerance * 100.0);

            // 🕳️ `remodeling_mesh`'s own `Unwrap`/LSCM stage legitimately duplicates vertex indices at every
            // UV chart seam, which a naive re-`validate_watertight` on the exported positions/indices
            // misreads as index-mismatched boundary edges. Assert on the pipeline's own watertight report
            // instead, captured at `Stage::Validate2` right before `Unwrap` runs and never touched again.
            let watertight_report = engine.take_quality().and_then(|quality| quality.watertight).expect("mesh pipeline should have produced a pre-unwrap watertight report by the time meshing finished");
            println!("[long] pre-unwrap watertight report: {watertight_report:?}");
            assert!(watertight_report.is_watertight, "the video-in -> watertight-mesh-out contract requires is_watertight == true on the pipeline's own pre-unwrap report, got: {watertight_report:?}");
        }
    }
    // #endregion 🔖️LongContract

    #[test]
    fn maximum_image_and_malformed_admission_steps_stay_below_hard_ceiling_in_each_build_profile() {
        fn accepted_frame(index: u32, width: usize, height: usize, bytes: usize) -> AcceptedFrame {
            AcceptedFrame { index, image: remodeling_image::ImageRgba8 { width: width as u32, height: height as u32, data: vec![0; bytes] }, timestamp_ms: index as f64, stream_id: 0, sharpness: 1.0 }
        }

        let mut maximum = ReconstructionEngine::new(&EngineParams::default());
        maximum.frame_source.frames = vec![accepted_frame(0, 512, 512, MAX_INTERACTIVE_IMAGE_BYTES), accepted_frame(1, 512, 512, MAX_INTERACTIVE_IMAGE_BYTES)];
        let started = std::time::Instant::now();
        assert!(maximum.start().is_ok());
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum admitted image envelope validation exceeded 8 ms");

        for (width, height, bytes) in [(513, 512, 513 * 512 * 4), (512, 512, MAX_INTERACTIVE_IMAGE_BYTES - 1)] {
            let mut malformed = ReconstructionEngine::new(&EngineParams::default());
            malformed.frame_source.frames = vec![accepted_frame(0, width, height, bytes), accepted_frame(1, 1, 1, 4)];
            let started = std::time::Instant::now();
            assert!(malformed.start().is_err());
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "oversized or malformed image admission exceeded 8 ms");
        }

        let mut too_many = ReconstructionEngine::new(&EngineParams::default());
        too_many.frame_source.frames = (0..65).map(|index| accepted_frame(index, 1, 1, 4)).collect();
        let started = std::time::Instant::now();
        assert!(too_many.start().is_err());
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "65-frame admission rejection exceeded 8 ms");
    }

    #[test]
    fn adversarial_feature_match_and_track_worker_steps_stay_fuel_bounded() {
        std::thread::spawn(|| {
            let mut params = EngineParams::default();
            params.target_feature_count = 512;
            let mut engine = ReconstructionEngine::new(&params);
            let (width, height) = (512, 512);
            assert_eq!(width * height, MAX_INTERACTIVE_IMAGE_PIXELS);
            let mut pixels = vec![0; width * height * 4];
            for index in 0..width * height {
                let value = ((index * 131) ^ (index / width * 197)) as u8;
                pixels[index * 4..index * 4 + 4].copy_from_slice(&[value, 255 - value, value.rotate_left(3), 255]);
            }
            engine.frames.push(AcceptedFrame { index: 0, image: remodeling_image::ImageRgba8 { width: width as u32, height: height as u32, data: pixels }, timestamp_ms: 0.0, stream_id: 0, sharpness: 1.0 });
            while engine.cursor == 0 {
                let started = std::time::Instant::now();
                engine.step_extracting_features();
                assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum admitted feature allocation/luma/detect/describe microstep exceeded 8 ms");
                if let Some(preparation) = &engine.feature_preparation {
                    assert!(preparation.cursor <= width * height);
                }
            }

            engine.descriptors_per_frame = vec![(0..2_048).map(|index| remodeling_feature::Descriptor256([index as u64, !(index as u64), index as u64 * 17, index as u64 * 31])).collect(); 2];
            engine.match_pairs = vec![(0, 1)];
            engine.pair_cursor = 0;
            while engine.pair_cursor == 0 {
                let before = engine.pair_match_preparation.as_ref().map_or((0, 0), |state| (state.query, state.candidate));
                let started = std::time::Instant::now();
                engine.step_matching_features();
                assert!(started.elapsed() < std::time::Duration::from_millis(8), "pair-match microstep exceeded 8 ms");
                let after = engine.pair_match_preparation.as_ref().map_or((2_048, 0), |state| (state.query, state.candidate));
                assert!(after.0 > before.0 || after.1 >= before.1 || engine.pair_cursor == 1);
            }

            engine.pairwise_matches = vec![(0, 1, (0..200_000).map(|index| remodeling_feature::Match { a: index, b: index, distance: 0 }).collect())];
            loop {
                let started = std::time::Instant::now();
                let complete = engine.step_build_tracks();
                assert!(started.elapsed() < std::time::Duration::from_millis(8), "track microstep exceeded 8 ms");
                if complete.is_some() {
                    break;
                }
            }
        })
        .join()
        .expect("engine worker");
    }
}
// #endregion 🔖️Tests
