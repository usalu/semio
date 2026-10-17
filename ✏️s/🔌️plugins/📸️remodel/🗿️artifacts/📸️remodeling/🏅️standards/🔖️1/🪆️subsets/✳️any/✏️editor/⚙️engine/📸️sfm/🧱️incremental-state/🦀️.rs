//! 🧱️ State of the incremental structure from motion: the interactive limits, the configuration, the reconstruction, the bundle problem, the phase and preparation records and the accessors of the incremental reconstruction. See [`super`].

use super::*;

pub(super) const MAX_INTERACTIVE_SEED_CORRESPONDENCES: usize = 64;

pub(super) const MAX_INTERACTIVE_SEED_HYPOTHESES: usize = 32;

pub(super) const MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES: usize = 64;

pub(super) const MAX_INTERACTIVE_REGISTRATION_ATTEMPTS: u64 = 64;

/// 🎯️ Hypotheses the bounded PnP registration draws before it commits when the running best already
/// explains most correspondences; a poorer best keeps drawing up to
/// [`MAX_INTERACTIVE_REGISTRATION_ATTEMPTS`].
pub(super) const MIN_INTERACTIVE_REGISTRATION_ATTEMPTS: u64 = 8;

/// 🎯️ Inlier share of the 2D–3D correspondences a bounded PnP pose must explain to be committed.
pub(super) const MIN_INTERACTIVE_REGISTRATION_INLIER_SHARE: f64 = 0.25;

/// 🎯️ Reprojection tolerance of the bounded PnP relative to the configured RANSAC threshold: the
/// points it registers against were triangulated from neighbouring views a few degrees apart, from
/// integer-pixel keypoints, so they carry a few pixels of error the later bundle cleanup removes;
/// held to the seed threshold, a correct pose on the orbit fixture explained 29 % of its
/// correspondences and was refused.
pub(super) const INTERACTIVE_REGISTRATION_THRESHOLD_FACTOR: f64 = 2.0;

/// 🎯️ Inlier share past which the bounded PnP stops drawing early.
pub(super) const EARLY_INTERACTIVE_REGISTRATION_INLIER_SHARE: f64 = 0.6;

/// 🎯️ Bounded hypothesis batches the two-view registration fallback spends before it commits to the
/// best essential matrix it found (each batch is one [`MAX_INTERACTIVE_SEED_HYPOTHESES`] draw from
/// each of the five-point and eight-point estimators).
pub(super) const MAX_INTERACTIVE_TWO_VIEW_ATTEMPTS: usize = 32;

/// 🎯️ Hypotheses each estimator draws per two-view fallback call: four five-point and four
/// eight-point solves, unpolished, keep one call inside a worker step; the best model is polished
/// once when the fallback commits.
pub(super) const INTERACTIVE_TWO_VIEW_HYPOTHESES_PER_CALL: usize = 4;

pub(super) const MAX_INTERACTIVE_TRACK_OBSERVATIONS: usize = 8;

/// 🧵️ Tracks the bounded registration, triangulation and snapshot walks consider. Every walk is a
/// cursor advanced by the call's work budget, so this caps the reconstruction's point count, not a
/// call's cost. It must exceed the tracks a registrable capture produces before its later frames
/// start: at 512 a 36-view orbit had every track past the first ~4 views ignored, so PnP starved
/// from the fifth camera on.
pub(super) const MAX_INTERACTIVE_TRACKS: usize = 4_096;

// #region 🔖️Incremental
/// 🎛️ Tuning for [`IncrementalSfm`]: RANSAC inlier threshold (pixels), minimum track length to
/// triangulate, bundle-adjustment iteration cap, IRLS robust loss, minimum triangulation angle, and
/// the per-camera visible-point floor used by [`IncrementalSfm::prune_outliers`].
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct SfmConfig {
    pub ransac_threshold_px: f64,
    pub min_track_length: usize,
    pub ba_max_iterations: usize,
    pub robust_loss: RobustLoss,
    pub min_triangulation_angle_rad: f64,
    pub min_visible_points_to_keep_camera: usize,
}

impl Default for SfmConfig {
    fn default() -> Self {
        Self { ransac_threshold_px: 2.0, min_track_length: 3, ba_max_iterations: 50, robust_loss: RobustLoss::Huber(2.0), min_triangulation_angle_rad: 2.0_f64.to_radians(), min_visible_points_to_keep_camera: 6 }
    }
}

/// 📦️ A finished (or in-progress) reconstruction snapshot: registered cameras (by frame index), their
/// triangulated points, each point's originating track id, and the shared calibration.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Reconstruction {
    pub cameras: Vec<(usize, CameraPose)>,
    pub points: Vec<[f64; 3]>,
    pub point_track_ids: Vec<usize>,
    pub intrinsics: Intrinsics,
}

/// 🧩️ Bundle-adjustment [`BipartiteResiduals`] problem for [`IncrementalSfm`]'s `local_ba`/`global_ba`:
/// A-blocks are cameras (6-dof `se(3)` log-tangent), B-blocks are points (3-dof XYZ), both updated via
/// `schur_lm`'s plain elementwise vector addition — since `BipartiteResiduals` has no manifold-retraction
/// hook (unlike [`LeastSquaresProblem::plus`]), a camera's raw tangent parameter is *not* re-retracted
/// through `Se3::exp`/`semio_compose_rs` between iterations, only added to directly. This is a first-order
/// approximation, exact only for small per-iteration steps; it's adequate here because `local_ba`/
/// `global_ba` are always seeded from an already-reasonable pose (PnP/triangulation output), keeping
/// steps small. `observations` maps `(a_index, b_index)` to that camera-point pair's pixel observation,
/// since [`BipartiteResiduals::evaluate`] receives a term but not its index into `residual_terms()`.
#[derive(semio_framework_value::RetireOwned)]
pub struct SfmBundleProblem {
    pub intrinsics: Intrinsics,
    pub num_cameras: usize,
    pub num_points: usize,
    pub terms: Vec<ResidualTerm>,
    pub observations: std::collections::HashMap<(usize, usize), [f64; 2]>,
    /// 📌️ Cameras (by A-index) the adjustment holds still: their pose Jacobian is zero, so with
    /// damping on the diagonal the solver's step for them is zero. Holding the two oldest cameras
    /// of a window pins the reconstruction's gauge — rotation, translation AND scale — which
    /// otherwise floats freely through every local adjustment and shows up as the seed pair's
    /// pose wandering off by degrees while nothing in the images moved.
    pub fixed_cameras: Vec<bool>,
}

impl BipartiteResiduals for SfmBundleProblem {
    fn num_a_blocks(&self) -> usize {
        self.num_cameras
    }

    fn num_b_blocks(&self) -> usize {
        self.num_points
    }

    fn a_block_dim(&self) -> usize {
        6
    }

    fn b_block_dim(&self) -> usize {
        3
    }

    fn residual_terms(&self) -> &[ResidualTerm] {
        &self.terms
    }

    fn evaluate(&self, a_params: &[VecD], b_params: &[VecD], term: &ResidualTerm) -> (VecD, MatD, MatD) {
        let ai = term.a_index.expect("sfm bundle terms always touch a camera");
        let bi = term.b_index.expect("sfm bundle terms always touch a point");
        let obs = self.observations[&(ai, bi)];
        let eps = 1e-6;
        let residual_at = |xi: [f64; 6], point: [f64; 3]| -> [f64; 2] {
            let pose = CameraPose(Se3::exp(xi));
            reproject(&self.intrinsics, &pose, point).map_or([obs[0] + 1.0e3, obs[1] + 1.0e3], |p| [p[0] - obs[0], p[1] - obs[1]])
        };
        let xi: [f64; 6] = std::array::from_fn(|k| a_params[ai].get(k));
        let point: [f64; 3] = std::array::from_fn(|k| b_params[bi].get(k));
        let r = residual_at(xi, point);
        let mut ja = MatD::zeros(2, 6);
        let camera_free = !self.fixed_cameras.get(ai).copied().unwrap_or(false);
        for k in (0..6).filter(|_| camera_free) {
            let mut xp = xi;
            xp[k] += eps;
            let mut xm = xi;
            xm[k] -= eps;
            let rp = residual_at(xp, point);
            let rm = residual_at(xm, point);
            ja.set(0, k, (rp[0] - rm[0]) / (2.0 * eps));
            ja.set(1, k, (rp[1] - rm[1]) / (2.0 * eps));
        }
        let mut jb = MatD::zeros(2, 3);
        for k in 0..3 {
            let mut pp = point;
            pp[k] += eps;
            let mut pm = point;
            pm[k] -= eps;
            let rp = residual_at(xi, pp);
            let rm = residual_at(xi, pm);
            jb.set(0, k, (rp[0] - rm[0]) / (2.0 * eps));
            jb.set(1, k, (rp[1] - rm[1]) / (2.0 * eps));
        }
        (VecD::from_vec(vec![r[0], r[1]]), ja, jb)
    }
}

/// 🎥️ Two-view essential-matrix estimation for [`IncrementalSfm::init_pair`]'s seed pair: runs both the
/// Nistér five-point solver ([`estimate_essential_five_point`], primary) and the normalized 8-point
/// [`estimate_essential`] (kept as a working fallback, not deleted), keeping whichever achieves the lower
/// (better) MSAC cost. Consecutive video frames — this crate's primary target, unlike a curated
/// wide-baseline photoset — have far smaller parallax than a typical photogrammetry pair, exactly the
/// regime where the unconstrained 8-DOF linear 8-point fit degenerates (see
/// [`estimate_essential_five_point`]'s docs and the `TwoViewFivePointTests` planar-scene contract test)
/// while the constrained 5-DOF five-point manifold stays well-conditioned. Both solvers run over the same
/// correspondences with the same normalized-ray MSAC threshold, so their [`TwoViewResult::score`]s are
/// directly comparable; five-point wins ties.
#[cfg(test)]
pub(super) fn estimate_init_pair_essential(matches: &[([f64; 2], [f64; 2])], k: &Intrinsics, seed: u64) -> Option<TwoViewResult> {
    const FIVE_POINT_THRESHOLD: f64 = 0.005;
    let five_point = estimate_essential_five_point(matches, k, k, FIVE_POINT_THRESHOLD, seed);
    let eight_point = estimate_essential(matches, k, k);
    match (five_point, eight_point) {
        (Some(five), Some(eight)) => Some(if five.score <= eight.score { five } else { eight }),
        (Some(five), None) => Some(five),
        (None, Some(eight)) => Some(eight),
        (None, None) => None,
    }
}

/// 🏗️ Incremental structure-from-motion pipeline: register cameras one at a time via PnP against an
/// already-triangulated point cloud, growing the reconstruction frame by frame.
#[derive(semio_framework_value::RetireOwned)]
pub struct IncrementalSfm {
    pub(super) intrinsics: Intrinsics,
    pub(super) tracks: FeatureTracks,
    pub(super) keypoints_per_frame: Vec<Vec<Keypoint>>,
    pub(super) pairwise_matches: Vec<(usize, usize, Vec<Match>)>,
    pub(super) cfg: SfmConfig,
    pub(super) cameras: Vec<(usize, CameraPose)>,
    pub(super) points: std::collections::BTreeMap<usize, [f64; 3]>,
    pub(super) point_events: Option<Vec<(usize, [f64; 3], bool)>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum SeedPairPhase {
    Collect,
    Solve,
    Triangulate,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct SeedPairPreparation {
    pub(super) frame_a: usize,
    pub(super) frame_b: usize,
    pub(super) matches: Vec<Match>,
    pub(super) cursor: usize,
    pub(super) correspondences: Vec<([f64; 2], [f64; 2])>,
    pub(super) phase: SeedPairPhase,
    /// 🎯️ Hypothesis batches spent and the best (polished) essential-matrix estimate they
    /// produced, with the raw inlier count of the candidate it was polished from: one bounded
    /// five-point batch per call, so no call runs the whole search.
    pub(super) attempts: usize,
    pub(super) best: Option<TwoViewResult>,
    pub(super) best_raw_score: f64,
}

/// 🎯️ Five-point hypotheses one seed-solve call draws, and the calls a seed spends before it
/// commits to the best model found.
pub(super) const SEED_HYPOTHESES_PER_CALL: usize = 2;

pub(super) const SEED_SOLVE_CALLS: usize = 32;

impl SeedPairPreparation {
    pub fn new(frame_a: usize, frame_b: usize, matches: &[Match]) -> Self {
        Self {
            frame_a,
            frame_b,
            matches: matches.iter().take(MAX_INTERACTIVE_SEED_CORRESPONDENCES).copied().collect(),
            cursor: 0,
            correspondences: Vec::with_capacity(matches.len().min(MAX_INTERACTIVE_SEED_CORRESPONDENCES)),
            phase: SeedPairPhase::Collect,
            attempts: 0,
            best: None,
            best_raw_score: f64::INFINITY,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum RegistrationPhase {
    Collect,
    Solve,
    /// 📐️ PnP found no pose: pick the registered reference sharing the most direct matches.
    TwoViewSelect,
    /// 📐️ Collect that reference's correspondences (stride-subsampled to the bounded maximum).
    TwoViewCollect,
    /// 📐️ One bounded essential-matrix hypothesis batch per call, keeping the best model so far.
    TwoViewSolve,
    /// 📐️ Walk the triangulated tracks shared with the reference to recover the metric scale.
    TwoViewScale,
    /// 📐️ Commit the scaled pose once it reprojects the shared points.
    TwoViewCommit,
    Triangulate,
    Done,
    Failed,
}

/// 📐️ Bounded two-view fallback state for one frame PnP could not register: essential matrix
/// against the best-connected registered camera, scaled by the shared triangulated points (or, when
/// matching left none, by the seed pair's baseline). Every field is a cursor or a fixed-size buffer,
/// so no call materializes more than [`MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES`] items.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct TwoViewFallback {
    pub(super) reference: usize,
    pub(super) correspondences: Vec<([f64; 2], [f64; 2])>,
    pub(super) attempts: usize,
    pub(super) best: Option<TwoViewResult>,
    pub(super) relative: Option<Se3>,
    pub(super) cursor: usize,
    pub(super) scales: Vec<f64>,
    pub(super) shared: Vec<([f64; 3], [f64; 2])>,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct RegistrationPreparation {
    pub(super) frame: usize,
    pub(super) cursor: usize,
    pub(super) attempts: u64,
    pub(super) world_points: Vec<[f64; 3]>,
    pub(super) observations: Vec<[f64; 2]>,
    pub(super) phase: RegistrationPhase,
    pub(super) two_view: Option<Box<TwoViewFallback>>,
    /// 🎯️ Best PnP hypothesis so far across the one-draw-per-call attempts: pose and inlier count.
    /// One draw is one random 3-point sample, so committing the FIRST pose a call answers registers
    /// the camera on whatever three correspondences came up — and every later triangulation and
    /// registration inherits that pose's error (the orbit fixture's third camera came out 16° off,
    /// the fourth 108°). The draws are kept and the best-supported pose is committed instead.
    pub(super) best: Option<(CameraPose, usize)>,
}

impl RegistrationPreparation {
    pub fn new(frame: usize) -> Self {
        Self { frame, cursor: 0, attempts: 0, world_points: Vec::with_capacity(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES), observations: Vec::with_capacity(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES), phase: RegistrationPhase::Collect, two_view: None, best: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum BundlePhase {
    Prune,
    Retriangulate,
    Done,
}

/// 🎯️ A bundle adjustment in flight (see [`IncrementalSfm::begin_bundle_adjustment`]).
#[derive(semio_framework_value::RetireOwned)]
pub struct BundleAdjustment {
    pub(super) frames: Vec<usize>,
    pub(super) point_track_ids: Vec<usize>,
    pub(super) problem: SfmBundleProblem,
    pub(super) cfg: LmConfig,
    pub(super) preparation: SchurLmPreparation,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct BundlePreparation {
    pub(super) point_track_ids: Vec<usize>,
    pub(super) cursor: usize,
    pub(super) phase: BundlePhase,
    /// 🧹️ Multiple of the RANSAC threshold a point's worst reprojection may reach before pruning.
    pub(super) tolerance: f64,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct ReconstructionSnapshotPreparation {
    pub(super) camera_cursor: usize,
    pub(super) point_cursor: usize,
    pub(super) point_track_ids: Vec<usize>,
    pub(super) reconstruction: Reconstruction,
}

impl IncrementalSfm {
    /// 🆕️ Starts an empty incremental reconstruction over a shared calibration, precomputed feature tracks and per-frame keypoints.
    pub fn new(intrinsics: Intrinsics, tracks: FeatureTracks, keypoints_per_frame: Vec<Vec<Keypoint>>, cfg: SfmConfig) -> Self {
        Self { intrinsics, tracks, keypoints_per_frame, pairwise_matches: Vec::new(), cfg, cameras: Vec::new(), points: std::collections::BTreeMap::new(), point_events: None }
    }

    /// 🧮️ Retained heap bytes of everything this reconstruction holds on to: the tracks and keypoints it
    /// was handed, the pairwise match table, the registered cameras, the triangulated point map and any
    /// recorded point events. A memory probe needs this without seeing the private fields.
    pub fn retained_bytes(&self) -> usize {
        let nested = |outer: usize, inner: usize| outer + inner;
        nested(
            self.tracks.tracks.len() * size_of::<Vec<(usize, u32)>>(),
            self.tracks.tracks.iter().map(|track| track.capacity() * size_of::<(usize, u32)>()).sum::<usize>(),
        ) + nested(
            self.keypoints_per_frame.len() * size_of::<Vec<Keypoint>>(),
            self.keypoints_per_frame.iter().map(|frame| frame.capacity() * size_of::<Keypoint>()).sum::<usize>(),
        ) + nested(
            self.pairwise_matches.len() * size_of::<(usize, usize, Vec<Match>)>(),
            self.pairwise_matches.iter().map(|(_, _, matches)| matches.capacity() * size_of::<Match>()).sum::<usize>(),
        ) + self.cameras.capacity() * size_of::<(usize, CameraPose)>()
            + self.points.len() * (size_of::<usize>() + size_of::<[f64; 3]>() + 16)
            + self.point_events.as_ref().map_or(0, |events| events.capacity() * size_of::<(usize, [f64; 3], bool)>())
    }

    /// 🔭️ Starts recording every triangulated (`true`) and pruned (`false`) point by track id.
    pub fn observe_points(&mut self) {
        self.point_events.get_or_insert_with(Vec::new);
    }

    /// 📤️ Hands every recorded point event to `sink`, oldest first.
    pub fn drain_point_events(&mut self, mut sink: impl FnMut(usize, [f64; 3], bool)) {
        for (track, point, kept) in self.point_events.as_mut().map(std::mem::take).unwrap_or_default() {
            sink(track, point, kept);
        }
    }

    /// 📷️ The registered pose of `frame`, if any.
    pub fn camera_pose(&self, frame: usize) -> Option<CameraPose> {
        self.pose_of(frame)
    }

    /// 🕸️ Attaches the pairwise match table used for two-view registration fallbacks (track union-find alone
    /// drops conflicted chains that JPEG matching often creates, starving essential-matrix correspondence).
    pub fn set_pairwise_matches(&mut self, pairwise_matches: Vec<(usize, usize, Vec<Match>)>) {
        self.pairwise_matches = pairwise_matches;
    }

    #[cfg(test)]
    pub(super) fn is_registered(&self, frame: usize) -> bool {
        self.cameras.iter().any(|&(f, _)| f == frame)
    }

    pub(super) fn pose_of(&self, frame: usize) -> Option<CameraPose> {
        self.cameras.iter().find(|&&(f, _)| f == frame).map(|&(_, p)| p)
    }

    pub(super) fn obs_px(&self, frame: usize, kp: u32) -> [f64; 2] {
        let k = self.keypoints_per_frame[frame][kp as usize];
        [f64::from(k.x), f64::from(k.y)]
    }

    pub(super) fn track_obs_in(&self, track: &[(usize, u32)], frame: usize) -> Option<[f64; 2]> {
        track.iter().find(|&&(f, _)| f == frame).map(|&(f, kp)| self.obs_px(f, kp))
    }

    /// 🎯️ Number of already-triangulated tracks also observed in `frame` — the 2D-3D
    /// correspondence count [`register_next`](Self::register_next) would feed to PnP.
    /// 🕸️ Direct pairwise match count between `frame` and any currently registered camera.
    #[cfg(test)]
    pub fn pairwise_match_count(&self, frame: usize) -> usize {
        let registered: std::collections::HashSet<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        let mut best = 0usize;
        for &(a, b, ref matches) in &self.pairwise_matches {
            if matches.len() < 8 {
                continue;
            }
            if (a == frame && registered.contains(&b)) || (b == frame && registered.contains(&a)) {
                best = best.max(matches.len());
            }
        }
        best
    }

    #[cfg(test)]
    pub fn pnp_correspondence_count(&self, frame: usize) -> usize {
        self.tracks.tracks.iter().enumerate().filter(|(track_id, track)| self.points.contains_key(track_id) && self.track_obs_in(track, frame).is_some()).count()
    }

    /// 🌱️ Whether `frame` already has a registered camera pose in this reconstruction.
    #[cfg(test)]
    pub fn has_camera(&self, frame: usize) -> bool {
        self.is_registered(frame)
    }

    /// 🔢️ How many triangulated points `frame` observes — the 2D–3D correspondences a PnP
    /// registration of that frame can draw from.
    pub fn correspondence_count(&self, frame: usize) -> usize {
        self.points.keys().filter(|&&track_id| self.tracks.tracks.get(track_id).is_some_and(|track| track.iter().any(|&(observed, _)| observed == frame))).count()
    }

    /// 🧵️ The observations `(frame, keypoint)` of every triangulated track `frame` observes, with
    /// the track's point — what a registration of `frame` sees, laid out for an audit.
    pub fn correspondences_of(&self, frame: usize) -> Vec<(usize, [f64; 3], Vec<(usize, u32)>)> {
        self.points.iter().filter_map(|(&track_id, &point)| self.tracks.tracks.get(track_id).filter(|track| track.iter().any(|&(observed, _)| observed == frame)).map(|track| (track_id, point, track.clone()))).collect()
    }

    /// 🧵️ Every track observing `frame`, with whether it already carries a point.
    pub fn tracks_observing(&self, frame: usize) -> Vec<(Vec<(usize, u32)>, bool)> {
        self.tracks.tracks.iter().enumerate().filter(|(_, track)| track.iter().any(|&(observed, _)| observed == frame)).map(|(track_id, track)| (track.clone(), self.points.contains_key(&track_id))).collect()
    }

    /// 🔢️ Triangulated points.
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    pub fn registered_count(&self) -> usize {
        self.cameras.len()
    }

    pub fn camera_pose_prefix(&self, maximum: usize) -> Vec<CameraPose> {
        self.cameras.iter().take(maximum).map(|&(_, pose)| pose).collect()
    }

    pub fn point_prefix(&self, maximum: usize) -> Vec<[f64; 3]> {
        self.points.values().take(maximum).copied().collect()
    }
}
// #endregion 🔖️Incremental
