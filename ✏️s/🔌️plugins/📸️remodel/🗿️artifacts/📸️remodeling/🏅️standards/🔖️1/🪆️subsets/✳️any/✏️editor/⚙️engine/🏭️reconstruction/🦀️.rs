//! ⚙️ Reconstruction engine: cooperative staged pipeline turning decoded frames into textured meshes,
//! previews and quality reports. Ties every sibling domain topic file together into the actual
//! video-in → watertight-mesh-out pipeline, self-contained (it never reaches back up at the artifact's
//! document types — the app-level translation layer in `🦀️.rs` does that in both directions).

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).
// 🏃️ `motion` is deliberately absent: the pipeline accepts `EngineParams::motion_enabled` but does not
// yet drive the motion topic file from `advance()` — a documented gap carried over verbatim from the
// pre-merge crate, which declared the same dependency without ever using it.
use crate::editor::remodeling::engine::{
    camera as remodeling_camera, dense as remodeling_dense, feature as remodeling_feature, geo as remodeling_geo, images as remodeling_image, mesh as remodeling_mesh, sfm as remodeling_sfm, video as remodeling_video,
};

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
#[cfg(test)]
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
#[cfg(test)]
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
    #[cfg(test)]
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
    /// 🌀️ Lens distortion of the (single) calibrated camera, applied by every unprojection. A
    /// calibrated capture that is reconstructed as if undistorted carries a systematic pixel error
    /// at the image borders that the bundle adjustment cannot explain and that accumulates into
    /// drift around a closed orbit.
    pub distortion: remodeling_camera::Distortion,
    /// 🧭️ Which bounded detector scores keypoints. The interactive engine runs an AKAZE request as
    /// ORB: the nonlinear scale space's diffusion passes cannot be sliced into the worker's
    /// per-step budget, and ORB's oriented FAST + Harris + rBRIEF is the same binary-descriptor
    /// family [`step_matching_features`](ReconstructionEngine) already matches — a documented
    /// simplification.
    pub detector: remodeling_feature::BoundedDetector,
    /// 🧭️ Upright (unsteered) descriptors instead of intensity-centroid steering; see
    /// [`remodeling_feature::BoundedDetectionPreparation`]'s `upright`. Off by default: on the orbit
    /// fixture steering pairs 83 of 91 accepted matches correctly at a 10° step against 56 of 67
    /// upright.
    pub upright_descriptors: bool,
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
            distortion: remodeling_camera::Distortion::None,
            detector: remodeling_feature::BoundedDetector::Orb,
            upright_descriptors: false,
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
fn default_intrinsics(width: u32, height: u32, focal_ratio: f64, distortion: remodeling_camera::Distortion) -> remodeling_camera::Intrinsics {
    let f = focal_ratio * f64::from(width.max(height));
    remodeling_camera::Intrinsics { fx: f, fy: f, cx: f64::from(width) / 2.0, cy: f64::from(height) / 2.0, skew: 0.0, distortion }
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

/// 🔭️ One decision the pipeline took while observed ([`ReconstructionEngine::observe`]): every feature set,
/// match candidate verdict, pair, camera registration, triangulated or pruned point, depth map, fused cloud
/// and extracted mesh. A run job turns each into a visible trace record or step.
#[derive(Clone, Debug, PartialEq)]
pub enum EngineObservation {
    FeaturesExtracted { frame: usize, keypoints: usize },
    MatchAccepted { frame_a: usize, frame_b: usize, query: u32, candidate: u32, distance: u32 },
    MatchRatioRejected { frame_a: usize, frame_b: usize, query: u32, best: u32, second: u32 },
    MatchCrossCheckRejected { frame_a: usize, frame_b: usize, query: u32, candidate: u32 },
    PairMatched { frame_a: usize, frame_b: usize, matches: usize },
    /// 🧭️ Matches of a pair the geometric verification dropped as inconsistent with its best
    /// essential matrix.
    PairGeometryRejected { frame_a: usize, frame_b: usize, dropped: usize },
    TracksBuilt { tracks: usize },
    CameraRegistered { frame: usize, pose: remodeling_camera::CameraPose },
    CameraRejected { frame: usize },
    /// 🌱️ No seed pair could be solved: the capture has no registrable two-view geometry.
    SeedPairFailed { detail: String },
    PointTriangulated { track: usize, point: [f64; 3] },
    PointPruned { track: usize, point: [f64; 3] },
    DepthMapEstimated { view: usize, samples: usize },
    DenseCloudFused { points: usize },
    MeshExtracted { vertices: usize, triangles: usize },
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

/// 📏️ The depth interval one reference view's PatchMatch hypothesises over, read off the sparse
/// reconstruction: the depths of the triangulated points in front of that camera, trimmed to their
/// 5th–95th percentiles and widened by a fixed factor, then clamped into the configured
/// `[depth_min, depth_max]`. The configured interval alone is a hazard, not a prior — an
/// uncalibrated run's `0.1–100` covers three orders of magnitude while the scene sits at a scale the
/// two-view seed fixed arbitrarily (unit baseline), so uniformly seeded hypotheses almost never land
/// near the surface and the few iterations a bounded run affords cannot recover them; every depth
/// map then scatters over the whole frustum, the fused cloud spans the frustum and the meshing
/// lattice, clamped to 32 cells, covers the scene at a resolution where nothing survives cleaning.
/// The sparse cloud is the one scene-scaled measurement the pipeline owns, so it narrows the search
/// to where structure was actually observed (fewer than two points in front of the camera leave the
/// configured interval untouched).
fn sparse_depth_range(points: &[[f64; 3]], pose: &remodeling_camera::CameraPose, configured: &remodeling_dense::PatchMatchConfig) -> (f32, f32) {
    const NEAR_FACTOR: f64 = 0.7;
    const FAR_FACTOR: f64 = 1.4;
    let mut depths: Vec<f64> = points.iter().map(|&point| pose.0.act(point)[2]).filter(|depth| depth.is_finite() && *depth > 0.0).collect();
    if depths.len() < 2 {
        return (configured.depth_min, configured.depth_max);
    }
    depths.sort_by(f64::total_cmp);
    let trim = depths.len() / 20;
    let near = depths[trim] * NEAR_FACTOR;
    let far = depths[depths.len() - 1 - trim] * FAR_FACTOR;
    let lower = f64::from(configured.depth_min).min(f64::from(configured.depth_max));
    let upper = f64::from(configured.depth_max).max(f64::from(configured.depth_min));
    let near = near.clamp(lower, upper) as f32;
    let far = far.clamp(lower, upper) as f32;
    if far > near { (near, far) } else { (configured.depth_min, configured.depth_max) }
}

/// 🧭️ Moves the sparse reconstruction into the frame every later stage and the published result
/// assume: the point centroid at the origin and the points' RMS radius at one unit. A monocular
/// reconstruction's gauge is whatever the seed pair's unit baseline made it — on the orbit fixture
/// the scene came out ~9.5 units wide, twelve units from the origin — so without this the
/// document's millimetre voxel size (sized for a metre-scale scene) met a lattice of thirty-two
/// cells that covered a third of the object, and the result mesh sat far outside the view the
/// placeholder occupied. Cameras follow the same similarity: `t' = s (R c + t)`, so depths scale
/// with the points and the dense stage's sparse-derived depth ranges stay consistent.
fn normalize_reconstruction_frame(reconstruction: &mut remodeling_sfm::Reconstruction) {
    const MIN_POINTS: usize = 4;
    if reconstruction.points.len() < MIN_POINTS {
        return;
    }
    let count = reconstruction.points.len() as f64;
    let centroid = reconstruction.points.iter().fold([0.0f64; 3], |acc, p| [acc[0] + p[0], acc[1] + p[1], acc[2] + p[2]]).map(|sum| sum / count);
    let rms = (reconstruction.points.iter().map(|p| (p[0] - centroid[0]).powi(2) + (p[1] - centroid[1]).powi(2) + (p[2] - centroid[2]).powi(2)).sum::<f64>() / count).sqrt();
    if !rms.is_finite() || rms < 1e-9 {
        return;
    }
    let scale = 1.0 / rms;
    for point in &mut reconstruction.points {
        *point = [(point[0] - centroid[0]) * scale, (point[1] - centroid[1]) * scale, (point[2] - centroid[2]) * scale];
    }
    for (_, pose) in &mut reconstruction.cameras {
        let rotated = pose.0.r.act(centroid);
        pose.0.t = [(rotated[0] + pose.0.t[0]) * scale, (rotated[1] + pose.0.t[1]) * scale, (rotated[2] + pose.0.t[2]) * scale];
    }
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
    /// 🎯️ A bundle adjustment in flight, [`BUNDLE_TERMS_PER_STEP`] terms per step: the local one
    /// after each registration and the global passes of the bundle stage.
    bundle_iterations: Option<remodeling_sfm::BundleAdjustment>,
    finalization_preparation: Option<FinalizationPreparation>,
    pose_cursor: usize,
    /// 🧭️ Frames to register after the seed pair, in registration order (outwards from the seed).
    registration_order: Vec<usize>,
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
    recorded: Option<Vec<EngineObservation>>,
}

/// 🗻️ Pyramid levels the bounded ORB detection spreads its keypoints over.
const FEATURE_PYRAMID_LEVELS: usize = 3;
/// 🔁️ Upper bound on loop-closure candidate pairs a frame is matched against beyond its window.
const LOOP_CLOSURE_PARTNERS_PER_FRAME: usize = 24;

/// 🎯️ One phase of the bundle stage.
#[derive(Clone, Copy, Debug)]
enum BundleStagePhase {
    /// 🧹️ Prune points whose worst reprojection exceeds `tolerance` × the RANSAC threshold (or
    /// that fewer than two registered cameras see), then retriangulate.
    Cleanup { tolerance: f64 },
    /// 🌐️ Global adjustment; `Some(factor)` runs a Huber kernel at `factor` × the RANSAC threshold,
    /// `None` the document's configured loss.
    Adjust { huber_factor: Option<f64> },
}

/// 🎯️ The bundle stage's phases in order (see `step_bundle_adjusting`).
const BUNDLE_STAGE_PLAN: [BundleStagePhase; 5] = [
    BundleStagePhase::Cleanup { tolerance: f64::INFINITY },
    BundleStagePhase::Adjust { huber_factor: Some(16.0) },
    BundleStagePhase::Cleanup { tolerance: 12.0 },
    BundleStagePhase::Adjust { huber_factor: None },
    BundleStagePhase::Cleanup { tolerance: 3.0 },
];

/// 🎯️ Cameras the local bundle adjustment after each registration refines (the newest ones, in
/// registration order) and the iterations it spends. Without it the poses drift by about a degree
/// per registered view on the orbit fixture — 20° by the twenty-second camera and a scale that
/// doubled — because every registration inherits the errors of the points it was solved against.
const LOCAL_BUNDLE_WINDOW: usize = 5;
const LOCAL_BUNDLE_ITERATIONS: usize = 5;

/// ⏱️ Residual terms one bundle-adjustment unit accumulates (each is ~20 reprojections).
const BUNDLE_TERMS_PER_STEP: usize = 4;

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
    /// 🧭️ The resumable detector, alive through [`FeaturePhase::Detect`].
    detection: Option<remodeling_feature::BoundedDetectionPreparation>,
    /// 🗻️ The detector's pyramid, kept through [`FeaturePhase::Describe`] for the descriptors.
    pyramid: Option<remodeling_image::Pyramid>,
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
    /// 🧭️ Geometric verification of the matched pair, once the descriptor matching is complete:
    /// hypothesis batches spent so far and the best essential-matrix estimate they produced.
    verification: Option<PairVerification>,
}

/// 🧭️ Running state of one pair's geometric verification.
struct PairVerification {
    correspondences: Vec<([f64; 2], [f64; 2])>,
    attempts: usize,
    /// The best polished model so far, by explained correspondences.
    best: Option<remodeling_sfm::TwoViewResult>,
}

/// 🧭️ Hypotheses one verification call draws (each a five-point solve — a 10×10 elimination and a
/// degree-ten root finding, ~2 ms in a debug build — scored over the pair, then polished for
/// ~0.7 ms). One per unit: two draws measured 3.8 ms at best, and under the host's scheduling
/// jitter that unit overran the 8 ms law in runs the watchdog would quarantine.
const PAIR_VERIFICATION_HYPOTHESES_PER_STEP: usize = 1;
/// 🧭️ Calls one pair's verification spends drawing hypotheses before it commits the best model.
const PAIR_VERIFICATION_STEPS: usize = 64;
/// 🧭️ Sampson tolerance of the verification, in pixels; divided by the focal length into the
/// normalized coordinates the solver scores in, so a 96-pixel frame is judged as leniently as a
/// 320-pixel one. Tight on purpose: at 2 px a fifth of a 10° pair's kept matches were still wrong
/// and the registration chain drifted 20° around the orbit; at 1 px the kept matches are 98–100 %
/// true and a legitimate pair still keeps dozens.
const PAIR_VERIFICATION_THRESHOLD_PX: f64 = 1.0;
/// 🧭️ Fewest inliers a pair keeps, and the share of its matches they must make up; below either
/// the pair's matches are all dropped as noise. A five-point model fits five of any eight random
/// matches and picks up a few more by chance, so eight of eight is what a 60° pair of unrelated
/// views answers; a pair that shares a view answers dozens at well over half.
const PAIR_VERIFICATION_MIN_INLIERS: usize = 12;
const PAIR_VERIFICATION_MIN_INLIER_SHARE: f64 = 0.5;

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
    /// 🌫️ The run's PatchMatch configuration with this view's own depth range (see
    /// [`sparse_depth_range`]).
    dense: remodeling_dense::PatchMatchConfig,
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
        for (axis, coordinate) in point.into_iter().enumerate() {
            self.bounds_min[axis] = self.bounds_min[axis].min(coordinate);
            self.bounds_max[axis] = self.bounds_max[axis].max(coordinate);
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
    /// 🎞️ Per root: bitmask of the frames its set already observes (the engine admits at most 64
    /// frames). A union whose two sets share a frame is refused — that match would put two
    /// keypoints of one frame in one track, which is a wrong match by definition — so a wrong
    /// match can no longer fuse two good tracks into a conflicting group that is then thrown away.
    frames: Vec<u64>,
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
            frames: Vec::new(),
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
        self.frames.push(1u64 << (observation.0 as u32).min(63));
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
        if left == right || self.frames[left] & self.frames[right] != 0 {
            return;
        }
        if self.rank[left] < self.rank[right] {
            std::mem::swap(&mut left, &mut right);
        }
        self.parent[right] = left;
        self.frames[left] |= self.frames[right];
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
            match_anchor_frame: 0,
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
            bundle_iterations: None,
            finalization_preparation: None,
            pose_cursor: 0,
            registration_order: Vec::new(),
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
            recorded: None,
        }
    }

    /// 🔭️ Starts recording [`EngineObservation`]s; an unobserved engine records nothing.
    pub fn observe(&mut self) {
        self.recorded.get_or_insert_with(Vec::new);
    }

    /// 📤️ Moves every recorded observation into `into`, oldest first.
    pub fn drain_observations(&mut self, into: &mut Vec<EngineObservation>) {
        if let (Some(sfm), Some(observations)) = (self.sfm.as_mut(), self.recorded.as_mut()) {
            sfm.drain_point_events(|track, point, kept| observations.push(if kept { EngineObservation::PointTriangulated { track, point } } else { EngineObservation::PointPruned { track, point } }));
        }
        if let Some(observations) = self.recorded.as_mut() {
            into.append(observations);
        }
    }

    fn record(&mut self, observation: EngineObservation) {
        if let Some(observations) = self.recorded.as_mut() {
            observations.push(observation);
        }
    }

    /// 📏️ `(completed, total)` units of the current stage; `total` is absent where the stage has no
    /// countable extent.
    pub fn stage_progress(&self) -> (u64, Option<u64>) {
        let count = |value: usize| value as u64;
        match self.stage {
            EngineStage::ExtractingFeatures => (count(self.cursor), Some(count(self.frames.len()))),
            EngineStage::MatchingFeatures if self.match_pairs_ready => (count(self.pair_cursor), Some(count(self.match_pairs.len()))),
            EngineStage::EstimatingPoses => (count(self.pose_cursor.min(self.registration_order.len())), Some(count(self.registration_order.len()))),
            EngineStage::DenseStereo | EngineStage::FusingVolume => (count(self.stage_cursor.min(self.dense_camera_indices.len())), Some(count(self.dense_camera_indices.len()))),
            _ => (0, None),
        }
    }

    /// 🔬️ The raw matching inputs an independent match oracle recomputes verdicts from: descriptor words
    /// per frame, the scheduled pairs, the ratio and whether the cross check runs.
    #[cfg(test)]
    pub fn match_oracle_inputs(&self) -> (Vec<Vec<[u64; 4]>>, Vec<(usize, usize)>, f32, bool) {
        (self.descriptors_per_frame.iter().map(|frame| frame.iter().map(|descriptor| descriptor.0).collect()).collect(), self.match_pairs.clone(), self.params.match_ratio, self.params.match_mutual)
    }

    /// ☁️ The fused dense cloud positions once [`EngineStage::FusingVolume`] finished, else empty.
    pub fn dense_positions(&self) -> &[[f64; 3]] {
        self.dense_cloud.as_ref().map_or(&[], |cloud| cloud.positions.as_slice())
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
    /// 🧭️ One frame's features in fuel-bounded slices: luma conversion by pixel runs, the resumable
    /// ORB/Harris detection ([`remodeling_feature::BoundedDetectionPreparation`]) by row bands, then
    /// rBRIEF description a few keypoints at a time; returns whether frames remain.
    fn step_extracting_features(&mut self) -> bool {
        const PIXELS_PER_STEP: usize = 4_096;
        const DETECTION_PIXELS_PER_STEP: usize = 512;
        const DESCRIPTORS_PER_STEP: usize = 4;
        if self.cursor >= self.frames.len() {
            return false;
        }
        let target_feature_count = self.params.target_feature_count.min(512);
        if self.feature_preparation.is_none() {
            let pixels = self.frames[self.cursor].image.width as usize * self.frames[self.cursor].image.height as usize;
            self.feature_preparation = Some(FeaturePreparation {
                frame: self.cursor,
                phase: FeaturePhase::Luma,
                cursor: 0,
                gray: Vec::with_capacity(pixels),
                detection: None,
                pyramid: None,
                keypoints: Vec::with_capacity(target_feature_count),
                descriptors: Vec::with_capacity(target_feature_count),
            });
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
                    let base = remodeling_image::ImageGray { width: image.width, height: image.height, data: std::mem::take(&mut preparation.gray) };
                    preparation.detection = Some(remodeling_feature::BoundedDetectionPreparation::new(base, FEATURE_PYRAMID_LEVELS, target_feature_count, self.params.detector, self.params.upright_descriptors));
                    preparation.phase = FeaturePhase::Detect;
                    preparation.cursor = 0;
                }
            }
            FeaturePhase::Detect => {
                // ⏱️ One band is about DETECTION_PIXELS_PER_STEP pixels of the level: each of the
                // streamed passes costs a few dozen operations per pixel, FAST + suppression a
                // few hundred per corner.
                let rows = (DETECTION_PIXELS_PER_STEP / (image.width as usize).max(1)).max(1);
                let detection = preparation.detection.as_mut().expect("bounded detection");
                if detection.advance(rows) {
                    let (pyramid, keypoints) = preparation.detection.take().expect("completed detection").finish();
                    preparation.pyramid = Some(pyramid);
                    preparation.keypoints = keypoints;
                    preparation.phase = FeaturePhase::Describe;
                    preparation.cursor = 0;
                }
            }
            FeaturePhase::Describe => {
                let end = preparation.cursor.saturating_add(DESCRIPTORS_PER_STEP).min(preparation.keypoints.len());
                let pyramid = preparation.pyramid.as_ref().expect("detector pyramid");
                preparation.descriptors.extend(remodeling_feature::describe_orb(pyramid, &preparation.keypoints[preparation.cursor..end]));
                preparation.cursor = end;
                if end == preparation.keypoints.len() {
                    let complete = self.feature_preparation.take().expect("completed feature preparation");
                    self.record(EngineObservation::FeaturesExtracted { frame: complete.frame, keypoints: complete.keypoints.len() });
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
        // 🔁️ Loop-closure candidates beyond the window: every frame against every `stride`-th later
        // frame, at most ~24 per frame. A candidate that does not share a view (40°–180° apart on
        // the orbit fixture) is dropped whole by the pair's geometric verification, so the cost of a
        // miss is one bounded match + solve; a hit — the orbit's last views against its first — is
        // the constraint that lets the bundle adjustment close a loop instead of accumulating a
        // sequential chain's drift (6° over the fixture's 36 views without it). The old `(0, f)` /
        // `(1, f)` anchors are gone: with real descriptors those pairs were noise, and every wrong
        // match unioned two good tracks into one no triangulation could validate.
        let stride = (n.saturating_sub(window)).div_ceil(LOOP_CLOSURE_PARTNERS_PER_FRAME).max(2);
        while self.match_pair_i >= n && self.match_anchor_frame < n && work < pair_budget {
            let frame = self.match_anchor_frame;
            let mut partner = frame + window + 1;
            while partner < n {
                if (partner - frame) % stride == 0 {
                    self.match_pairs.push((frame, partner));
                    work += 1;
                }
                partner += 1;
            }
            self.match_anchor_frame += 1;
        }
        self.match_pair_i >= n && self.match_anchor_frame >= n
    }

    /// 🤝️ Consumes at most `COMPARISONS_PER_STEP` Hamming comparisons across a resumable pair match.
    fn step_matching_features(&mut self) -> bool {
        // ⏱️ A debug-profile unit of 2 048 comparisons measured 8.2–9.5 ms against the 8 ms ceiling
        // (`every_bounded_unit_stays_under_the_interactive_ceiling_on_every_example`); half that
        // leaves the headroom the law wants.
        const COMPARISONS_PER_STEP: usize = 1_024;
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
                verification: None,
            });
        }
        let preparation = self.pair_match_preparation.as_mut().expect("pair match preparation");
        if preparation.query == self.descriptors_per_frame[preparation.frame_a].len() {
            return self.step_verifying_pair();
        }
        let desc_a = &self.descriptors_per_frame[preparation.frame_a];
        let desc_b = &self.descriptors_per_frame[preparation.frame_b];
        if desc_b.is_empty() {
            preparation.query = preparation.query.saturating_add(COMPARISONS_PER_STEP).min(desc_a.len());
            if preparation.query == desc_a.len() {
                let complete = self.pair_match_preparation.take().expect("completed empty pair match");
                self.record(EngineObservation::PairMatched { frame_a: complete.frame_a, frame_b: complete.frame_b, matches: 0 });
                self.pairwise_matches.push((complete.frame_a, complete.frame_b, complete.matches));
                self.pair_cursor += 1;
            }
            return self.pair_cursor < self.match_pairs.len();
        }
        let mut remaining = COMPARISONS_PER_STEP;
        let observations = &mut self.recorded;
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
                    let (frame_a, frame_b, query) = (preparation.frame_a, preparation.frame_b, preparation.query as u32);
                    if preparation.reverse_best_index == query {
                        preparation.matches.push(remodeling_feature::Match { a: query, b: best_index, distance });
                        if let Some(recorded) = observations.as_mut() {
                            recorded.push(EngineObservation::MatchAccepted { frame_a, frame_b, query, candidate: best_index, distance });
                        }
                    } else if let Some(recorded) = observations.as_mut() {
                        recorded.push(EngineObservation::MatchCrossCheckRejected { frame_a, frame_b, query, candidate: best_index });
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
                let (frame_a, frame_b, query) = (preparation.frame_a, preparation.frame_b, preparation.query as u32);
                if passes && self.params.match_mutual {
                    preparation.pending = Some((preparation.best_distance, preparation.best_index));
                    preparation.reverse_candidate = 0;
                    preparation.reverse_best_distance = u32::MAX;
                    preparation.reverse_best_index = u32::MAX;
                } else {
                    if passes {
                        preparation.matches.push(remodeling_feature::Match { a: query, b: preparation.best_index, distance: preparation.best_distance });
                        if let Some(recorded) = observations.as_mut() {
                            recorded.push(EngineObservation::MatchAccepted { frame_a, frame_b, query, candidate: preparation.best_index, distance: preparation.best_distance });
                        }
                    } else if preparation.best_index != u32::MAX {
                        if let Some(recorded) = observations.as_mut() {
                            recorded.push(EngineObservation::MatchRatioRejected { frame_a, frame_b, query, best: preparation.best_distance, second: preparation.second_distance });
                        }
                    }
                    reset_pair_query(preparation);
                }
            }
        }
        if preparation.query == desc_a.len() {
            return self.step_verifying_pair();
        }
        self.pair_cursor < self.match_pairs.len()
    }

    /// 🧭️ Geometric verification of the pair whose descriptor matching just completed: a bounded
    /// five-point RANSAC over the matched keypoints, one hypothesis batch per call, and only the
    /// best model's inliers survive into the pair table. Descriptor matching alone leaves a fifth
    /// to a quarter of a 10° pair's matches wrong (and nearly all of a 40° pair's); every wrong
    /// match that reaches the track builder either fuses two good tracks or plants a wrong
    /// observation in one, and a registration drawing on such tracks found 3 consistent
    /// correspondences out of 20. A pair without keypoints (a descriptor-only test harness) or with
    /// fewer than [`PAIR_VERIFICATION_MIN_INLIERS`] matches skips the solve and keeps its matches.
    fn step_verifying_pair(&mut self) -> bool {
        let preparation = self.pair_match_preparation.as_mut().expect("pair match preparation");
        let verifiable = preparation.matches.len() >= PAIR_VERIFICATION_MIN_INLIERS && self.keypoints_per_frame.len() > preparation.frame_a.max(preparation.frame_b) && !self.frames.is_empty();
        if verifiable {
            let intrinsics = default_intrinsics(self.frames[0].image.width, self.frames[0].image.height, self.params.assumed_focal_ratio, self.params.distortion);
            let (frame_a, frame_b) = (preparation.frame_a, preparation.frame_b);
            let verification = preparation.verification.get_or_insert_with(|| PairVerification {
                correspondences: preparation
                    .matches
                    .iter()
                    .map(|matched| {
                        let a = self.keypoints_per_frame[frame_a][matched.a as usize];
                        let b = self.keypoints_per_frame[frame_b][matched.b as usize];
                        ([f64::from(a.x), f64::from(a.y)], [f64::from(b.x), f64::from(b.y)])
                    })
                    .collect(),
                attempts: 0,
                best: None,
            });
            let seed = ((frame_a as u64) << 32 | frame_b as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ verification.attempts as u64;
            let threshold = PAIR_VERIFICATION_THRESHOLD_PX / intrinsics.fx.max(intrinsics.fy).max(1.0);
            if verification.attempts < PAIR_VERIFICATION_STEPS {
                if let Some(candidate) = remodeling_sfm::estimate_essential_five_point_bounded(&verification.correspondences, &intrinsics, &intrinsics, threshold, seed, PAIR_VERIFICATION_HYPOTHESES_PER_STEP, 0) {
                    // A raw five-point fit through five integer-pixel keypoints explains only a
                    // handful of correspondences at a pixel's tolerance, and its raw MSAC score
                    // does not rank the true model above a wrong one reliably enough to polish only
                    // improvements: on a marginal pair (two thirds true matches) the true draws
                    // lost to earlier wrong ones by raw score and the pair was dropped whole. So
                    // every unit polishes its batch's best (the unit's one costlier part) and the
                    // comparison is between refined models, by explained correspondences; the
                    // survivor set at the end is the refined one.
                    let polished = remodeling_sfm::polish_essential(candidate, &verification.correspondences, &intrinsics, threshold);
                    if verification.best.as_ref().is_none_or(|best| polished.inliers.len() > best.inliers.len()) {
                        verification.best = Some(polished);
                    }
                }
                verification.attempts += 1;
                return true;
            }
            let survivors: Vec<usize> = verification.best.as_ref().map(|best| best.inliers.clone()).unwrap_or_default();
            let mut kept = std::collections::BTreeSet::new();
            kept.extend(survivors);
            let before = preparation.matches.len();
            if kept.len() >= PAIR_VERIFICATION_MIN_INLIERS && kept.len() as f64 >= PAIR_VERIFICATION_MIN_INLIER_SHARE * before as f64 {
                let mut index = 0usize;
                preparation.matches.retain(|_| {
                    let keep = kept.contains(&index);
                    index += 1;
                    keep
                });
            } else {
                preparation.matches.clear();
            }
            let dropped = before - preparation.matches.len();
            if dropped > 0 {
                self.record(EngineObservation::PairGeometryRejected { frame_a, frame_b, dropped });
            }
        }
        let complete = self.pair_match_preparation.take().expect("completed pair match");
        self.record(EngineObservation::PairMatched { frame_a: complete.frame_a, frame_b: complete.frame_b, matches: complete.matches.len() });
        self.pairwise_matches.push((complete.frame_a, complete.frame_b, complete.matches));
        self.pair_cursor += 1;
        self.pair_cursor < self.match_pairs.len()
    }

    /// 🧵️ Consumes at most 4,096 union/group observations or 64 finalized groups.
    fn step_build_tracks(&mut self) -> Option<remodeling_sfm::FeatureTracks> {
        // ⏱️ A union is two hashed node lookups; a thousand per step keeps a debug-profile step
        // inside the worker law with room for the host's scheduling jitter.
        const OBSERVATIONS_PER_STEP: usize = 1_024;
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

    /// 🏗️ Seeds [`remodeling_sfm::IncrementalSfm`] on the adjacent pair with the most verified
    /// matches (first calls), then registers + triangulates one frame per bounded unit, walking
    /// outwards from the seed — forwards to the last frame, then backwards to the first — so every
    /// frame is registered against already-registered neighbours; each registration is followed by
    /// a local bundle adjustment. A frame that fails to register is skipped (best effort, like
    /// `run_all`). A seed pair that cannot be solved is not a fault either: the capture has no
    /// registrable geometry, the stage ends with zero cameras and the run completes with an empty
    /// reconstruction the trace explains, instead of a pipeline failure.
    fn step_estimating_poses(&mut self) -> Result<bool, String> {
        if self.sfm.is_none() {
            let intr = default_intrinsics(self.frames[0].image.width, self.frames[0].image.height, self.params.assumed_focal_ratio, self.params.distortion);
            let tracks = self.tracks.clone().expect("tracks built before EstimatingPoses");
            let keypoints = self.keypoints_per_frame.clone();
            let mut sfm = remodeling_sfm::IncrementalSfm::new(intr, tracks, keypoints, self.params.sfm.clone());
            if self.recorded.is_some() {
                sfm.observe_points();
            }
            let seed = self.pairwise_matches.iter().filter(|&&(a, b, _)| b == a + 1).max_by_key(|&&(a, _, ref matches)| (matches.len(), std::cmp::Reverse(a))).map(|(a, b, matches)| (*a, *b, remodeling_sfm::SeedPairPreparation::new(*a, *b, matches)));
            sfm.set_pairwise_matches(std::mem::take(&mut self.pairwise_matches));
            self.sfm = Some(sfm);
            match seed {
                Some((a, b, preparation)) => {
                    self.seed_pair_preparation = Some(preparation);
                    let n = self.frames.len();
                    self.registration_order = ((b + 1)..n).chain((0..a).rev()).collect();
                }
                None => {
                    self.record(EngineObservation::SeedPairFailed { detail: "no adjacent pair of frames shares verified matches".to_string() });
                    self.pose_cursor = usize::MAX;
                }
            }
        }
        if let Some(preparation) = self.seed_pair_preparation.as_mut() {
            match self.sfm.as_mut().expect("seed SfM").advance_seed_pair(preparation, 1) {
                Ok(false) => {}
                Ok(true) => {
                    self.seed_pair_preparation = None;
                    self.pose_cursor = 0;
                    let seeded: Vec<usize> = (0..self.frames.len()).filter(|&frame| !self.registration_order.contains(&frame)).collect();
                    for frame in seeded {
                        if let Some(pose) = self.sfm.as_ref().and_then(|sfm| sfm.camera_pose(frame)) {
                            self.record(EngineObservation::CameraRegistered { frame, pose });
                        }
                    }
                }
                Err(error) => {
                    self.seed_pair_preparation = None;
                    self.pose_cursor = usize::MAX;
                    self.record(EngineObservation::SeedPairFailed { detail: error.to_string() });
                    return Ok(false);
                }
            }
            return Ok(true);
        }
        let n = self.registration_order.len();
        let sfm = self.sfm.as_mut().expect("initialized SfM");
        // 🎯️ A local adjustment in flight finishes before anything else is decided — including the
        // camera cap below, or the last admitted camera would leave the stage half-refined with the
        // adjustment state still armed for the global pass to pick up.
        if let Some(bundle) = self.bundle_iterations.as_mut() {
            if sfm.advance_bundle_adjustment(bundle, BUNDLE_TERMS_PER_STEP) {
                self.bundle_iterations = None;
                let frame = self.registration_order[self.pose_cursor];
                if let Some(pose) = sfm.camera_pose(frame) {
                    self.record(EngineObservation::CameraRegistered { frame, pose });
                }
                self.registration_preparation = None;
                self.pose_cursor += 1;
            }
            return Ok(self.pose_cursor < n || self.registration_preparation.is_some());
        }
        let registered = sfm.registered_count();
        if self.params.max_registered_cameras > 0 && registered >= self.params.max_registered_cameras {
            return Ok(false);
        }
        if self.pose_cursor >= n {
            return Ok(false);
        }
        if self.registration_preparation.is_none() {
            self.registration_preparation = Some(remodeling_sfm::RegistrationPreparation::new(self.registration_order[self.pose_cursor]));
        }
        let result = sfm.advance_registration(self.registration_preparation.as_mut().expect("registration preparation"), 1);
        let frame = self.registration_order[self.pose_cursor];
        let registered = sfm.camera_pose(frame);
        match result {
            Ok(true) => {
                let local = registered.and_then(|_| {
                    // 🎯️ Refine the newest cameras and their points before the next registration
                    // draws on them; the registration is announced once the refinement settles.
                    let frames = sfm.local_bundle_frames(LOCAL_BUNDLE_WINDOW);
                    sfm.begin_bundle_adjustment(&frames, &frames[..frames.len().min(2)], sfm.robust_loss(), LOCAL_BUNDLE_ITERATIONS)
                });
                if local.is_some() {
                    self.bundle_iterations = local;
                } else if registered.is_some() {
                    self.record(EngineObservation::CameraRegistered { frame, pose: registered.expect("registered pose") });
                    self.registration_preparation = None;
                    self.pose_cursor += 1;
                } else {
                    self.registration_preparation = None;
                    self.pose_cursor += 1;
                }
            }
            Err(_) => {
                self.record(EngineObservation::CameraRejected { frame });
                self.registration_preparation = None;
                self.pose_cursor += 1;
            }
            Ok(false) => {}
        }
        Ok(self.pose_cursor < n || self.registration_preparation.is_some())
    }

    /// 🎯️ One bounded unit of the bundle stage, which walks [`BUNDLE_STAGE_PLAN`]: a cleanup pass
    /// (prune + retriangulate) or a global adjustment one iteration per step over every camera and
    /// point, up to the document's iteration budget per adjustment.
    ///
    /// 🔁️ Why two adjustments: the first runs with a wide Huber kernel on the unpruned points so the
    /// loop-closure observations — which a drifted chain reprojects tens of pixels off, and which are
    /// the only evidence that can pull the loop shut — act with their full residual (a pixel-scale
    /// kernel caps them to a constant the thousands of chain observations outweigh, and the loop
    /// stays open at ~6°); the cleanup between the passes prunes the gross outliers that pass
    /// tolerated, and the second adjustment converges on the configured kernel.
    fn step_bundle_adjusting(&mut self) -> bool {
        let Some(plan) = BUNDLE_STAGE_PLAN.get(self.ba_substep) else { return false };
        let Some(sfm) = self.sfm.as_mut() else { return false };
        match *plan {
            BundleStagePhase::Cleanup { tolerance } => {
                let preparation = self.bundle_preparation.get_or_insert_with(|| sfm.begin_bundle_with_tolerance(tolerance));
                if sfm.advance_bundle(preparation, 1) {
                    self.bundle_preparation = None;
                    self.ba_substep += 1;
                }
            }
            BundleStagePhase::Adjust { huber_factor } => {
                if self.bundle_iterations.is_none() {
                    let loss = match huber_factor {
                        Some(factor) => remodeling_sfm::RobustLoss::Huber(self.params.sfm.ransac_threshold_px * factor),
                        None => sfm.robust_loss(),
                    };
                    // 📌️ Only the first camera is held: holding the seed pair's second camera too
                    // would pin the reconstruction to the seed's five-point baseline (a few degrees
                    // off), and every other camera would bend to fit it. Scale is left to the damping.
                    let frames = sfm.registered_frames();
                    self.bundle_iterations = sfm.begin_bundle_adjustment(&frames, &frames[..frames.len().min(1)], loss, self.params.sfm.ba_max_iterations);
                    if self.bundle_iterations.is_none() {
                        self.ba_substep += 1;
                        return self.ba_substep < BUNDLE_STAGE_PLAN.len();
                    }
                }
                let bundle = self.bundle_iterations.as_mut().expect("an open adjustment");
                if sfm.advance_bundle_adjustment(bundle, BUNDLE_TERMS_PER_STEP) {
                    self.bundle_iterations = None;
                    self.ba_substep += 1;
                }
            }
        }
        self.ba_substep < BUNDLE_STAGE_PLAN.len()
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
                    let mut reconstruction = remodeling_sfm::IncrementalSfm::finish_reconstruction_snapshot(snapshot);
                    normalize_reconstruction_frame(&mut reconstruction);
                    self.reconstruction = Some(reconstruction);
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
        // ⏱️ One PatchMatch pixel evaluates `(2r + 1)²` reference samples against every source view
        // per hypothesis; the admitted worst case (radius 4, eight sources = 648 samples) is held to
        // one pixel per step, and a lighter configuration takes proportionally more pixels.
        const PATCH_SAMPLES_PER_STEP: usize = 648;
        let patch_side = 2 * self.params.dense.window_radius.clamp(1, 4) as usize + 1;
        let patch_sources = self.params.dense_source_views.clamp(1, 8);
        let patch_pixels_per_step = (PATCH_SAMPLES_PER_STEP / (patch_side * patch_side * patch_sources)).max(1);
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
            let (depth_min, depth_max) = sparse_depth_range(&reconstruction.points, &pose, &self.params.dense);
            let dense = remodeling_dense::PatchMatchConfig { depth_min, depth_max, ..self.params.dense };
            self.dense_preparation = Some(DenseStereoPreparation {
                phase: DenseStereoPhase::ReferenceLuma,
                reference_frame,
                reference_camera: (pose, intrinsics),
                source_frames,
                reference_gray: remodeling_image::ImageGray { width: reference.width, height: reference.height, data: Vec::with_capacity(reference.width as usize * reference.height as usize) },
                source_grays: Vec::new(),
                source: 0,
                pixel: 0,
                dense,
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
                if patch_match.advance(&preparation.reference_gray, &preparation.reference_camera, &preparation.source_grays, &preparation.dense, patch_pixels_per_step) {
                    let complete = self.dense_preparation.take().expect("completed dense preparation");
                    let map = complete.patch_match.expect("completed patch match").finish().expect("finished depth map");
                    self.record(EngineObservation::DepthMapEstimated { view: slot, samples: map.depth.iter().filter(|depth| depth.is_finite() && **depth > 0.0).count() });
                    self.fusion_capacity = self.fusion_capacity.saturating_add(map.depth.len());
                    self.depth_maps.push(map);
                    self.stage_cursor += 1;
                }
            }
        }
        self.stage_cursor < n_dense
    }

    /// 🧮️ One fuel-bounded slice of the sparse+dense extrema walk that fixes the meshing lattice (and
    /// with it the TSDF integration box); `true` once both clouds are walked. The cursors live in
    /// [`MeshingPreparation`], so the fusion stage and [`Self::step_begin_meshing`] share one walk
    /// rather than each performing their own.
    fn step_meshing_bounds(&mut self) -> bool {
        const POINTS_PER_STEP: usize = 2_048;
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
        true
    }

    /// 🧊️ The fusion stage, in the one order that keeps the volume small: first the cross-view
    /// `fuse_depth_maps` aggregate for the QC/geo cloud (it reads every depth map and never touches the
    /// volume), then the extrema walk that fixes the meshing lattice, and only then the per-camera TSDF
    /// integration — restricted to that lattice, releasing each depth map as it is consumed. Returns
    /// whether more work remains in this stage.
    ///
    /// 🧠️ Why the integration is bounded rather than global: PatchMatch hypothesises depths anywhere in
    /// `[dense.depth_min, dense.depth_max]` (a 0.1–100 m range for uncalibrated input), so an
    /// unconstrained integration scatters one 4 KiB `8³` block per touched neighbourhood across a
    /// hundred-metre frustum — measured at ~300 MiB and still climbing on a ten-view 320x240 run, which
    /// is what used to abort the guest's allocator mid-stage. `remodeling_mesh`'s surface extraction only
    /// ever samples the `compute_voxel_bounds_from_extrema` lattice (itself clamped to 32 cells per axis)
    /// plus one voxel past each face, so every voxel it can read is still integrated bit-identically
    /// while nothing is allocated beyond the margin.
    fn step_fusing_volume(&mut self) -> bool {
        const TSDF_SAMPLES_PER_STEP: usize = 256;
        const FUSION_COMPARISONS_PER_STEP: usize = 256;
        /// 🧱️ Voxels of slack around the meshing lattice: the surface net reads one voxel past each
        /// face through its cube corners, two keeps that provably inside the integrated region.
        const TSDF_INTEGRATION_MARGIN_VOXELS: i32 = 2;
        let n_dense = self.dense_camera_indices.len();
        if !self.fusion_finalized {
            let recon = self.reconstruction.as_ref().expect("fusion requires reconstruction");
            if self.fusion_views.len() < n_dense {
                let camera = self.dense_camera_indices[self.fusion_views.len()];
                self.fusion_views.push((recon.cameras[camera].1, recon.intrinsics));
                return true;
            }
            let preparation = self.fusion_preparation.get_or_insert_with(|| remodeling_dense::FusionPreparation::new(self.fusion_capacity));
            if preparation.advance(&self.fusion_views, &self.depth_maps, &remodeling_dense::FusionConfig::default(), FUSION_COMPARISONS_PER_STEP) {
                self.dense_cloud = self.fusion_preparation.take().and_then(remodeling_dense::FusionPreparation::finish);
                self.fusion_finalized = true;
                let points = self.dense_positions().len();
                self.record(EngineObservation::DenseCloudFused { points });
            }
            return true;
        }
        if !self.step_meshing_bounds() {
            return true;
        }
        // 🧊️ Always ensures a (possibly still-empty) TSDF exists once this stage starts, even when
        // `n_dense == 0` (a degenerate but legitimate outcome — every registered camera got pruned by
        // bundle adjustment): without this, `begin_meshing` used to find `self.tsdf` still `None` and
        // silently skip building a `MeshPipeline`, later surfacing as the confusing, wiring-looking
        // `"mesh pipeline not initialized"` failure instead of an honest empty-reconstruction outcome.
        if self.tsdf.is_none() {
            let preparation = self.meshing_preparation.as_ref().expect("walked meshing preparation");
            let (lattice_min, lattice_max) = compute_voxel_bounds_from_extrema(preparation.bounds_min, preparation.bounds_max, self.params.tsdf_voxel_size);
            self.tsdf = Some(
                remodeling_dense::TsdfVolume::new(self.params.tsdf_voxel_size, self.params.tsdf_truncation)
                    .with_voxel_bounds(lattice_min.map(|coordinate| coordinate.saturating_sub(TSDF_INTEGRATION_MARGIN_VOXELS)), lattice_max.map(|coordinate| coordinate.saturating_add(TSDF_INTEGRATION_MARGIN_VOXELS))),
            );
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
                // 🧹️ This map's three per-pixel buffers (~1.5 MiB for a 320x240 view, ~5 MiB at the
                // admitted 512x512 envelope) are dead the moment it is integrated: fusion already read
                // every map above and nothing downstream reads them again. The slot itself stays so the
                // stage's cursors and counts keep their meaning.
                self.depth_maps[slot] = remodeling_dense::DepthMap::new(0, 0);
                self.stage_cursor += 1;
            }
            return true;
        }
        false
    }

    /// 🏗️ Advances the shared extrema walk, texture-view copying and pipeline creation with finite
    /// point/byte fuel.
    fn step_begin_meshing(&mut self) -> bool {
        const IMAGE_BYTES_PER_STEP: usize = 4_096;
        if self.tsdf.is_none() {
            return true;
        }
        if !self.step_meshing_bounds() {
            return false;
        }
        let preparation = self.meshing_preparation.as_mut().expect("walked meshing preparation");
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
        // 🧹️ Input pixels are dead here: the texture views above hold their own copies and the mesh
        // stages read only the volume and those views. Ten 320x240 frames are 3 MiB, the admitted
        // envelope (64 frames of 512x512) is 64 MiB — all of it retained through meshing otherwise.
        // Width/height go with the buffer so `data.len() == width * height * 4` still holds.
        for frame in &mut self.frames {
            frame.image = remodeling_image::ImageRgba8 { width: 0, height: 0, data: Vec::new() };
        }
        // 🧹️ The keypoints the terminal observation table needs were already copied into the
        // finalized `observations`, so they are dead by the mesh handoff. The descriptors stay:
        // at 32 bytes each they are under a megabyte for the admitted envelope, and the match
        // oracle (`match_oracle_inputs`) reads them after the run.
        self.keypoints_per_frame = Vec::new();
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
                            self.record(EngineObservation::TracksBuilt { tracks: tracks.tracks.len() });
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
                        let (vertices, triangles) = self.mesh_data.as_ref().map_or((0, 0), |mesh| (mesh.positions.len() / 3, mesh.indices.len() / 3));
                        self.record(EngineObservation::MeshExtracted { vertices, triangles });
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

    /// 🔬️ Which bounded unit the engine is on, for the worker-law diagnostics: the stage plus the
    /// cursors of whichever preparation is active.
    pub fn unit_label(&self) -> String {
        match self.stage {
            EngineStage::EstimatingPoses => format!("poses cursor={} bundle={} registration={} seed={}", self.pose_cursor, self.bundle_iterations.is_some(), self.registration_preparation.is_some(), self.seed_pair_preparation.is_some()),
            EngineStage::BundleAdjusting => format!("bundle substep={} adjustment={} cleanup={} finalization={}", self.ba_substep, self.bundle_iterations.is_some(), self.bundle_preparation.is_some(), self.finalization_preparation.as_ref().map_or("-".to_string(), |preparation| format!("{:?}", preparation.phase))),
            stage => format!("{stage:?} cursor={}", self.stage_cursor),
        }
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
                for (axis, coordinate) in point.iter().copied().enumerate() {
                    preparation.bounds_min[axis] = preparation.bounds_min[axis].min(coordinate);
                    preparation.bounds_max[axis] = preparation.bounds_max[axis].max(coordinate);
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
#[path = "🧪️tests/🔬️terminal-sparse-chunk/🦀️.rs"]
mod terminal_sparse_chunk_tests;

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️retention/🦀️.rs"]
mod retention;
// #endregion 🔖️Tests
