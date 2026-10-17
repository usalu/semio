//! 🔩️ Bundle adjustment and snapshot steps of the incremental structure from motion: the PnP pose refinement, the local and global bundle adjustment, the outlier pruning and the bounded reconstruction snapshot. See [`super`].

use super::*;

// #region 🔖️Incremental
impl IncrementalSfm {
    /// 🎯️ Re-estimates every registered camera via PnP against the current triangulated cloud (frames with
    /// fewer than [`P3pSolver::SAMPLE_SIZE`] correspondences are left unchanged). Cleans up poses that were
    /// seeded by the two-view baseline-prior fallback before bundle adjustment / dense stereo.
    #[cfg(test)]
    pub fn refine_registered_poses_pnp(&mut self) {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        for frame in frames {
            let mut world_pts = Vec::new();
            let mut obs = Vec::new();
            for (track_id, track) in self.tracks.tracks.iter().enumerate() {
                let Some(&point) = self.points.get(&track_id) else { continue };
                let Some(px) = self.track_obs_in(track, frame) else { continue };
                world_pts.push(point);
                obs.push(px);
            }
            if world_pts.len() < P3pSolver::SAMPLE_SIZE {
                continue;
            }
            let cfg = RansacConfig { threshold: self.cfg.ransac_threshold_px, confidence: 0.999, max_iters: 2000, seed: frame as u64 ^ 0x9E37_79B9, scoring: RansacScoring::Msac };
            if let Some((pose, _)) = pnp_ransac(&self.intrinsics, &world_pts, &obs, &cfg) {
                if let Some(slot) = self.cameras.iter_mut().find(|(f, _)| *f == frame) {
                    slot.1 = pose;
                }
            }
        }
    }

    #[cfg(test)]
    pub fn triangulate_new(&mut self, frame: usize) {
        for track_id in 0..self.tracks.tracks.len() {
            self.triangulate_track(track_id, Some(frame));
        }
    }

    fn build_bundle_problem(&self, camera_frames: &[usize], fixed_frames: &[usize]) -> (SfmBundleProblem, Vec<VecD>, Vec<VecD>, Vec<usize>) {
        let a_index_of: std::collections::HashMap<usize, usize> = camera_frames.iter().enumerate().map(|(i, &f)| (f, i)).collect();
        let mut point_track_ids: Vec<usize> = Vec::new();
        let mut b_index_of: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut terms = Vec::new();
        let mut observations = std::collections::HashMap::new();
        for &track_id in self.points.keys() {
            for &(f, kp) in &self.tracks.tracks[track_id] {
                let Some(&ai) = a_index_of.get(&f) else { continue };
                let bi = *b_index_of.entry(track_id).or_insert_with(|| {
                    point_track_ids.push(track_id);
                    point_track_ids.len() - 1
                });
                terms.push(ResidualTerm { a_index: Some(ai), b_index: Some(bi), dim: 2 });
                observations.insert((ai, bi), self.obs_px(f, kp));
            }
        }
        let a0: Vec<VecD> = camera_frames
            .iter()
            .map(|&f| {
                let pose = self.pose_of(f).unwrap_or(CameraPose(Se3::identity()));
                VecD::from_vec(pose.0.log().to_vec())
            })
            .collect();
        let b0: Vec<VecD> = point_track_ids.iter().map(|&tid| VecD::from_vec(self.points[&tid].to_vec())).collect();
        let fixed_cameras = camera_frames.iter().map(|frame| fixed_frames.contains(frame)).collect();
        let problem = SfmBundleProblem { intrinsics: self.intrinsics, num_cameras: camera_frames.len(), num_points: point_track_ids.len(), terms, observations, fixed_cameras };
        (problem, a0, b0, point_track_ids)
    }

    /// 🎯️ Opens a bounded bundle adjustment over `camera_frames` (the `fixed_frames` among them held
    /// still) and the points they see: the problem is built once, the Levenberg–Marquardt iterations
    /// then run `term_budget` residual terms per [`Self::advance_bundle_adjustment`] call up to
    /// `max_iterations`, and the refined poses and points are written back when it completes.
    /// `None` when there is nothing to adjust.
    pub fn begin_bundle_adjustment(&self, camera_frames: &[usize], fixed_frames: &[usize], loss: RobustLoss, max_iterations: usize) -> Option<BundleAdjustment> {
        if camera_frames.is_empty() {
            return None;
        }
        let (problem, a0, b0, point_track_ids) = self.build_bundle_problem(camera_frames, fixed_frames);
        if problem.terms.is_empty() {
            return None;
        }
        let cfg = LmConfig { max_iters: max_iterations.max(1), loss, ..LmConfig::default() };
        let preparation = SchurLmPreparation::new(&problem, a0, b0, &cfg);
        Some(BundleAdjustment { frames: camera_frames.to_vec(), point_track_ids, problem, cfg, preparation })
    }

    /// 🎯️ One bounded unit of an open adjustment; `true` once it is done and written back.
    pub fn advance_bundle_adjustment(&mut self, adjustment: &mut BundleAdjustment, term_budget: usize) -> bool {
        if !adjustment.preparation.advance(&adjustment.problem, &adjustment.cfg, term_budget) {
            return false;
        }
        for (frame, a) in adjustment.frames.iter().zip(adjustment.preparation.a_params.iter()) {
            let xi: [f64; 6] = std::array::from_fn(|k| a.get(k));
            if let Some(entry) = self.cameras.iter_mut().find(|entry| entry.0 == *frame) {
                entry.1 = CameraPose(Se3::exp(xi));
            }
        }
        for (tid, b) in adjustment.point_track_ids.iter().zip(adjustment.preparation.b_params.iter()) {
            self.points.insert(*tid, [b.get(0), b.get(1), b.get(2)]);
        }
        true
    }

    /// 🎯️ The configured robust loss, the one a local adjustment uses.
    pub fn robust_loss(&self) -> RobustLoss {
        self.cfg.robust_loss
    }

    /// 🎯️ The frames a local bundle adjustment after the latest registration refines: the last
    /// `window` registered cameras in registration order.
    pub fn local_bundle_frames(&self, window: usize) -> Vec<usize> {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        let start = frames.len().saturating_sub(window);
        frames[start..].to_vec()
    }

    /// 🌐️ Every registered frame in registration order, for a global bundle adjustment.
    pub fn registered_frames(&self) -> Vec<usize> {
        self.cameras.iter().map(|&(f, _)| f).collect()
    }

    #[cfg(test)]
    fn run_bundle_adjustment(&mut self, camera_frames: &[usize]) {
        if camera_frames.is_empty() {
            return;
        }
        let (problem, a0, b0, point_track_ids) = self.build_bundle_problem(camera_frames, &[]);
        if problem.terms.is_empty() {
            return;
        }
        let cfg = LmConfig { max_iters: self.cfg.ba_max_iterations, loss: self.cfg.robust_loss, ..LmConfig::default() };
        let result: SchurResult = schur_lm(&problem, a0, b0, &cfg);
        for (frame, a) in camera_frames.iter().zip(result.a_params.iter()) {
            let xi: [f64; 6] = std::array::from_fn(|k| a.get(k));
            if let Some(entry) = self.cameras.iter_mut().find(|entry| entry.0 == *frame) {
                entry.1 = CameraPose(Se3::exp(xi));
            }
        }
        for (tid, b) in point_track_ids.iter().zip(result.b_params.iter()) {
            self.points.insert(*tid, [b.get(0), b.get(1), b.get(2)]);
        }
    }

    /// 🎯️ Bundle-adjusts the most-recently-registered `window` cameras (by frame insertion order) and every point they see.
    #[cfg(test)]
    pub fn local_ba(&mut self, window: usize) {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        let start = frames.len().saturating_sub(window);
        let window_frames = frames[start..].to_vec();
        self.run_bundle_adjustment(&window_frames);
    }

    /// 🌐️ Bundle-adjusts every registered camera and every triangulated point.
    #[cfg(test)]
    pub fn global_ba(&mut self) {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        self.run_bundle_adjustment(&frames);
    }

    /// 🧹️ Drops points whose worst-view reprojection error exceeds `3 * ransac_threshold_px` or whose
    /// best pairwise triangulation angle is below [`SfmConfig::min_triangulation_angle_rad`], then drops
    /// any camera left seeing fewer than [`SfmConfig::min_visible_points_to_keep_camera`] surviving points.
    #[cfg(test)]
    pub fn prune_outliers(&mut self) {
        let reproj_threshold = self.cfg.ransac_threshold_px * 3.0;
        let mut to_remove = Vec::new();
        for (&tid, &point) in &self.points {
            let track = &self.tracks.tracks[tid];
            let views: Vec<(CameraPose, [f64; 2])> = track.iter().filter_map(|&(f, kp)| self.pose_of(f).map(|pose| (pose, self.obs_px(f, kp)))).collect();
            if views.len() < 2 {
                to_remove.push(tid);
                continue;
            }
            let mut max_err = 0.0_f64;
            for (pose, px) in &views {
                match reproject(&self.intrinsics, pose, point) {
                    Some(pred) => max_err = max_err.max(((pred[0] - px[0]).powi(2) + (pred[1] - px[1]).powi(2)).sqrt()),
                    None => max_err = f64::MAX,
                }
            }
            let mut max_angle = 0.0_f64;
            for i in 0..views.len() {
                for j in (i + 1)..views.len() {
                    max_angle = max_angle.max(triangulation_angle(&views[i].0, &views[j].0, point));
                }
            }
            if max_err > reproj_threshold || max_angle < self.cfg.min_triangulation_angle_rad {
                to_remove.push(tid);
            }
        }
        for tid in to_remove {
            self.points.remove(&tid);
        }

        let min_visible = self.cfg.min_visible_points_to_keep_camera;
        let mut visible_count: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for &tid in self.points.keys() {
            for &(f, _) in &self.tracks.tracks[tid] {
                if self.is_registered(f) {
                    *visible_count.entry(f).or_insert(0) += 1;
                }
            }
        }
        self.cameras.retain(|&(f, _)| visible_count.get(&f).copied().unwrap_or(0) >= min_visible);
    }

    /// 🔁️ Re-runs [`triangulate_new`](Self::triangulate_new) over every registered frame, picking up
    /// tracks that a bundle-adjustment pass's refined poses may now connect.
    #[cfg(test)]
    pub fn retriangulate(&mut self) {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        for frame in frames {
            self.triangulate_new(frame);
        }
    }

    pub fn begin_bundle(&self) -> BundlePreparation {
        self.begin_bundle_with_tolerance(3.0)
    }

    /// 🧹️ [`Self::begin_bundle`] pruning at `tolerance` times the RANSAC threshold instead of three:
    /// the cleanup that precedes a global adjustment keeps the loop-closure observations — the ones
    /// a drifted chain reprojects tens of pixels off, and the only ones that can pull the loop shut —
    /// by pruning only gross outliers; the cleanup after it prunes at the strict tolerance.
    pub fn begin_bundle_with_tolerance(&self, tolerance: f64) -> BundlePreparation {
        let mut point_track_ids: Vec<usize> = self.points.keys().copied().take(MAX_INTERACTIVE_TRACKS).collect();
        point_track_ids.sort_unstable();
        BundlePreparation { point_track_ids, cursor: 0, phase: BundlePhase::Prune, tolerance }
    }

    /// 🎯️ Incremental robust bundle cleanup: one triangulated point's complete bounded observation
    /// residual set or one track retriangulation per fuel unit. The admitted engine envelope caps both
    /// tracks and cameras, so each inner residual set is finite and independently timed by tests.
    pub fn advance_bundle(&mut self, preparation: &mut BundlePreparation, work_budget: usize) -> bool {
        match preparation.phase {
            BundlePhase::Prune => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= preparation.point_track_ids.len() {
                        preparation.cursor = 0;
                        preparation.phase = BundlePhase::Retriangulate;
                        break;
                    }
                    let track_id = preparation.point_track_ids[preparation.cursor];
                    preparation.cursor += 1;
                    let Some(&point) = self.points.get(&track_id) else { continue };
                    let Some(track) = self.tracks.tracks.get(track_id) else { continue };
                    let mut maximum = 0.0f64;
                    let mut visible = 0usize;
                    for &(frame, keypoint) in track.iter().take(MAX_INTERACTIVE_TRACK_OBSERVATIONS) {
                        let Some(pose) = self.pose_of(frame) else { continue };
                        visible += 1;
                        let observed = self.obs_px(frame, keypoint);
                        let Some(predicted) = reproject(&self.intrinsics, &pose, point) else {
                            maximum = f64::INFINITY;
                            break;
                        };
                        maximum = maximum.max(((predicted[0] - observed[0]).powi(2) + (predicted[1] - observed[1]).powi(2)).sqrt());
                    }
                    if visible < 2 || maximum > self.cfg.ransac_threshold_px * preparation.tolerance {
                        self.points.remove(&track_id);
                        if let Some(events) = self.point_events.as_mut() {
                            events.push((track_id, point, false));
                        }
                    }
                }
            }
            BundlePhase::Retriangulate => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS) {
                        preparation.phase = BundlePhase::Done;
                        break;
                    }
                    self.triangulate_track(preparation.cursor, None);
                    preparation.cursor += 1;
                }
            }
            BundlePhase::Done => {}
        }
        preparation.phase == BundlePhase::Done
    }

    pub fn begin_reconstruction_snapshot(&self) -> ReconstructionSnapshotPreparation {
        let mut point_track_ids: Vec<usize> = self.points.keys().copied().take(MAX_INTERACTIVE_TRACKS).collect();
        point_track_ids.sort_unstable();
        ReconstructionSnapshotPreparation {
            camera_cursor: 0,
            point_cursor: 0,
            reconstruction: Reconstruction { cameras: Vec::with_capacity(self.cameras.len()), points: Vec::with_capacity(point_track_ids.len()), point_track_ids: Vec::with_capacity(point_track_ids.len()), intrinsics: self.intrinsics },
            point_track_ids,
        }
    }

    pub fn advance_reconstruction_snapshot(&self, preparation: &mut ReconstructionSnapshotPreparation, item_budget: usize) -> bool {
        let mut remaining = item_budget.max(1);
        while preparation.camera_cursor < self.cameras.len() && remaining > 0 {
            preparation.reconstruction.cameras.push(self.cameras[preparation.camera_cursor]);
            preparation.camera_cursor += 1;
            remaining -= 1;
        }
        while preparation.point_cursor < preparation.point_track_ids.len() && remaining > 0 {
            let track_id = preparation.point_track_ids[preparation.point_cursor];
            if let Some(&point) = self.points.get(&track_id) {
                preparation.reconstruction.points.push(point);
                preparation.reconstruction.point_track_ids.push(track_id);
            }
            preparation.point_cursor += 1;
            remaining -= 1;
        }
        preparation.camera_cursor == self.cameras.len() && preparation.point_cursor == preparation.point_track_ids.len()
    }

    pub fn finish_reconstruction_snapshot(mut preparation: ReconstructionSnapshotPreparation) -> Reconstruction {
        preparation.reconstruction.cameras.sort_by_key(|&(frame, _)| frame);
        preparation.reconstruction
    }

    /// 📦️ A snapshot of the current registered cameras and triangulated points.
    #[cfg(test)]
    pub fn reconstruction(&self) -> Reconstruction {
        let mut cameras = self.cameras.clone();
        cameras.sort_by_key(|&(f, _)| f);
        let mut point_track_ids: Vec<usize> = self.points.keys().copied().collect();
        point_track_ids.sort_unstable();
        let points = point_track_ids.iter().map(|tid| self.points[tid]).collect();
        Reconstruction { cameras, points, point_track_ids, intrinsics: self.intrinsics }
    }

    /// 🚀️ Convenience end-to-end driver: [`init_pair`](Self::init_pair) on the first two frames of
    /// `frame_order`, then for each subsequent frame `register_next` -> `triangulate_new` -> `local_ba`
    /// (window 5) -> `prune_outliers` -> `retriangulate`, finishing with one [`global_ba`](Self::global_ba).
    /// A frame that fails to register (e.g. too few 2D-3D correspondences yet) is skipped rather than
    /// aborting the whole run — a documented best-effort policy, since one poorly-connected frame
    /// shouldn't sink an otherwise-good reconstruction.
    #[cfg(test)]
    pub fn run_all(&mut self, frame_order: &[usize], pairwise_matches: &[(usize, usize, Vec<Match>)]) -> Result<Reconstruction, SfmError> {
        if frame_order.len() < 2 {
            return Err(SfmError::TooFewFrames);
        }
        let (f0, f1) = (frame_order[0], frame_order[1]);
        let matches01 = pairwise_matches.iter().find(|&&(a, b, _)| a == f0 && b == f1).map(|(_, _, m)| m).ok_or(SfmError::InsufficientMatches)?;
        self.set_pairwise_matches(pairwise_matches.to_vec());
        self.init_pair(f0, f1, matches01)?;

        for _ in 0..frame_order.len().saturating_sub(2) {
            let mut candidates: Vec<(usize, usize)> = frame_order.iter().copied().filter(|&frame| !self.is_registered(frame)).map(|frame| (self.pnp_correspondence_count(frame), frame)).collect();
            candidates.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            let mut registered_any = false;
            for (_corrs, frame) in candidates {
                if self.register_next(frame).is_ok() {
                    self.triangulate_new(frame);
                    self.local_ba(5);
                    self.prune_outliers();
                    self.retriangulate();
                    registered_any = true;
                    break;
                }
            }
            if !registered_any {
                break;
            }
        }
        self.global_ba();
        Ok(self.reconstruction())
    }
}
// #endregion 🔖️Incremental
