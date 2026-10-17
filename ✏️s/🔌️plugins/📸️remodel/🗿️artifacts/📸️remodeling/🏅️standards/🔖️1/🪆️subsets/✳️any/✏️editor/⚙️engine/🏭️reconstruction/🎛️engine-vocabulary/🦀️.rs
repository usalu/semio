//! 🎛️ Vocabulary of the reconstruction engine: the engine parameters, the stage, status and observation enums and the reconstruction frame helpers. See [`super`].

use super::*;

// #region 🔖️Params
/// 🎛️ Every knob the staged pipeline needs, bundled with sane defaults: ingestion policy, the assumed
/// pinhole focal-length ratio (no calibration stage exists yet, so intrinsics are derived from frame
/// dimensions), feature/matching/SfM/dense/mesh sub-configs (reusing each domain crate's own config type
/// directly rather than re-declaring their fields), and toggles for the optional motion/geo analyses.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
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
    pub sfm: semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::SfmConfig,
    pub dense: remodeling_dense::PatchMatchConfig,
    pub dense_source_views: usize,
    pub max_registered_cameras: usize,
    pub max_dense_cameras: usize,
    pub tsdf_voxel_size: f64,
    pub tsdf_truncation: f64,
    pub mesh: semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::MeshParams,
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
            sfm: semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::SfmConfig::default(),
            dense: remodeling_dense::PatchMatchConfig::default(),
            dense_source_views: 4,
            max_registered_cameras: 0,
            max_dense_cameras: 12,
            tsdf_voxel_size: 0.05,
            tsdf_truncation: 0.15,
            mesh: semio_s_artifact_remodel_remodeling_mesh::editor::remodeling::engine::mesh::MeshParams::default(),
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
pub(super) fn default_intrinsics(width: u32, height: u32, focal_ratio: f64, distortion: remodeling_camera::Distortion) -> remodeling_camera::Intrinsics {
    let f = focal_ratio * f64::from(width.max(height));
    remodeling_camera::Intrinsics { fx: f, fy: f, cx: f64::from(width) / 2.0, cy: f64::from(height) / 2.0, skew: 0.0, distortion }
}
// #endregion 🔖️Params

// #region 🔖️Pipeline
/// 🚦️ Named stage of the cooperative reconstruction state machine — mirrors the stage *names* the
/// not-yet-rewritten `remodeling_document::ReconstructionStage` plans to expose, without depending on that
/// crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
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
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
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
pub(super) fn stage_ordinal(stage: EngineStage) -> usize {
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

/// 🗺️ Maps `crate::editor::remodeling::engine::mesh::mesh_pipeline_step`'s internal stage name to the engine-level stage it falls
/// under, so driving the mesh pipeline (Amendment: engine delegates meshing directly to
/// `crate::editor::remodeling::engine::mesh::mesh_pipeline_step`) still reports through the coarser [`EngineStage`] vocabulary.
pub(super) fn mesh_stage_to_engine_stage(name: &str) -> EngineStage {
    match name {
        "marching_cubes" => EngineStage::ExtractingSurface,
        "unwrap" | "texture_bake" | "interchange" => EngineStage::Texturing,
        _ => EngineStage::CleaningMesh,
    }
}

/// 🏘️ Up to `k` other camera slot indices nearest to `ci` (by registration-order distance, which tracks
/// frame order for an [`crate::editor::remodeling::engine::sfm::IncrementalSfm`] reconstruction), sorted ascending for determinism —
/// the source-view selection for [`remodeling_dense::patchmatch_mvs`]/TSDF fusion.
pub(super) fn neighbor_camera_indices(ci: usize, n: usize, k: usize) -> Vec<usize> {
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
pub(super) fn sparse_depth_range(points: &[[f64; 3]], pose: &remodeling_camera::CameraPose, configured: &remodeling_dense::PatchMatchConfig) -> (f32, f32) {
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
pub(super) fn normalize_reconstruction_frame(reconstruction: &mut semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::Reconstruction) {
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

pub(super) fn compute_voxel_bounds_from_extrema(lo: [f64; 3], hi: [f64; 3], voxel_size: f64) -> ([i32; 3], [i32; 3]) {
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
// #endregion 🔖️Pipeline
