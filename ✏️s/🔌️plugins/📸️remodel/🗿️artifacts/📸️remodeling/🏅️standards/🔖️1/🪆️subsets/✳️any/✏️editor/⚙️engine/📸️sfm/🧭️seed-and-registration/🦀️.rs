//! 🧭️ Seed and registration steps of the incremental structure from motion: the initial pair, the two-view fallback registration, the track triangulation and the bounded seed-pair and registration advances. See [`super`].

use super::*;

// #region 🔖️Incremental
impl IncrementalSfm {
    /// 🌱️ Seeds the reconstruction from an initial pair: estimates the essential matrix + relative pose via
    /// [`estimate_init_pair_essential`] (frame_a at identity, frame_b at the recovered relative pose), then
    /// triangulates every track shared between the two frames.
    #[cfg(test)]
    pub fn init_pair(&mut self, frame_a: usize, frame_b: usize, matches: &[Match]) -> Result<(), SfmError> {
        if matches.len() < 8 {
            return Err(SfmError::InsufficientMatches);
        }
        let corr: Vec<([f64; 2], [f64; 2])> = matches.iter().map(|m| (self.obs_px(frame_a, m.a), self.obs_px(frame_b, m.b))).collect();
        let two_view = estimate_init_pair_essential(&corr, &self.intrinsics, frame_b as u64).ok_or(SfmError::DegenerateGeometry)?;
        let TwoViewModel::Fundamental(e) = two_view.model else {
            return Err(SfmError::DegenerateGeometry);
        };
        let inlier_rays: Vec<([f64; 2], [f64; 2])> = two_view
            .inliers
            .iter()
            .map(|&idx| {
                let ra = self.intrinsics.unproject_ray(corr[idx].0);
                let rb = self.intrinsics.unproject_ray(corr[idx].1);
                ([ra[0], ra[1]], [rb[0], rb[1]])
            })
            .collect();
        let relative_pose = decompose_essential(&e, &inlier_rays).ok_or(SfmError::DegenerateGeometry)?;

        self.cameras.clear();
        self.points.clear();
        self.cameras.push((frame_a, CameraPose(Se3::identity())));
        self.cameras.push((frame_b, CameraPose(relative_pose)));

        self.triangulate_new(frame_a);
        self.triangulate_new(frame_b);
        Ok(())
    }

    /// 🎯️ Registers `frame` via PnP against every already-triangulated track observed there. Requires at
    /// least [`P3pSolver::SAMPLE_SIZE`] 2D-3D correspondences. When PnP cannot run yet, falls back to a
    /// two-view essential-matrix pose against the best-connected registered reference only when enough
    /// shared triangulated points exist to recover a consistent metric scale and the refined pose
    /// reprojects those points within `3 * ransac_threshold_px`.
    #[cfg(test)]
    pub fn register_next(&mut self, frame: usize) -> Result<(), SfmError> {
        if self.is_registered(frame) {
            return Ok(());
        }
        let mut world_pts = Vec::new();
        let mut obs = Vec::new();
        for (track_id, track) in self.tracks.tracks.iter().enumerate() {
            let Some(&point) = self.points.get(&track_id) else { continue };
            let Some(px) = self.track_obs_in(track, frame) else { continue };
            world_pts.push(point);
            obs.push(px);
        }
        if world_pts.len() >= P3pSolver::SAMPLE_SIZE {
            let cfg = RansacConfig { threshold: self.cfg.ransac_threshold_px, confidence: 0.999, max_iters: 2000, seed: frame as u64, scoring: RansacScoring::Msac };
            if let Some((pose, _inliers)) = pnp_ransac(&self.intrinsics, &world_pts, &obs, &cfg) {
                self.cameras.push((frame, pose));
                return Ok(());
            }
        }
        self.register_next_two_view(frame)
    }

    /// 📐️ Two-view fallback for [`register_next`](Self::register_next): essential-matrix relative pose vs the
    /// registered frame sharing the most track observations. Metric scale prefers the median ratio of known
    /// triangulated-point distances to unit-baseline triangulations; when JPEG/matching leaves no shared
    /// 3D point, falls back to the most recent registered inter-camera baseline length.
    #[cfg(test)]
    fn register_next_two_view(&mut self, frame: usize) -> Result<(), SfmError> {
        let registered: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        let mut best: Option<(usize, Vec<([f64; 2], [f64; 2])>)> = None;
        for &ref_f in &registered {
            let mut corr = Vec::new();
            for &(a, b, ref matches) in &self.pairwise_matches {
                let (src, dst, flip) = if a == ref_f && b == frame {
                    (ref_f, frame, false)
                } else if a == frame && b == ref_f {
                    (frame, ref_f, true)
                } else {
                    continue;
                };
                let _ = (src, dst);
                for m in matches {
                    let (px_r, px_f) = if !flip { (self.obs_px(ref_f, m.a), self.obs_px(frame, m.b)) } else { (self.obs_px(ref_f, m.b), self.obs_px(frame, m.a)) };
                    corr.push((px_r, px_f));
                }
            }
            if corr.is_empty() {
                for track in &self.tracks.tracks {
                    let Some(px_r) = self.track_obs_in(track, ref_f) else { continue };
                    let Some(px_f) = self.track_obs_in(track, frame) else { continue };
                    corr.push((px_r, px_f));
                }
            }
            if corr.len() >= 8 && best.as_ref().map_or(true, |(_, c)| corr.len() > c.len()) {
                best = Some((ref_f, corr));
            }
        }
        let (ref_f, mut corr) = best.ok_or(SfmError::PnpFailed)?;
        const MAX_TWO_VIEW_CORRS: usize = 64;
        if corr.len() > MAX_TWO_VIEW_CORRS {
            let step = corr.len().div_ceil(MAX_TWO_VIEW_CORRS);
            corr = corr.into_iter().enumerate().filter_map(|(i, c)| (i % step == 0).then_some(c)).collect();
        }
        let two_view = estimate_init_pair_essential(&corr, &self.intrinsics, frame as u64).ok_or(SfmError::DegenerateGeometry)?;
        let TwoViewModel::Fundamental(e) = two_view.model else {
            return Err(SfmError::DegenerateGeometry);
        };
        let inlier_rays: Vec<([f64; 2], [f64; 2])> = two_view
            .inliers
            .iter()
            .map(|&idx| {
                let ra = self.intrinsics.unproject_ray(corr[idx].0);
                let rb = self.intrinsics.unproject_ray(corr[idx].1);
                ([ra[0], ra[1]], [rb[0], rb[1]])
            })
            .collect();
        let relative = decompose_essential(&e, &inlier_rays).ok_or(SfmError::DegenerateGeometry)?;
        let ref_pose = self.pose_of(ref_f).ok_or(SfmError::PnpFailed)?;
        let c_ref = camera_center(&ref_pose);
        let r_arr = mat3d_to_array(&relative.r.0);

        let mut scales = Vec::new();
        let mut shared: Vec<([f64; 3], [f64; 2])> = Vec::new();
        for (track_id, track) in self.tracks.tracks.iter().enumerate() {
            let Some(&point) = self.points.get(&track_id) else { continue };
            let Some(px_r) = self.track_obs_in(track, ref_f) else { continue };
            let Some(px_f) = self.track_obs_in(track, frame) else { continue };
            let ray_r = self.intrinsics.unproject_ray(px_r);
            let ray_f = self.intrinsics.unproject_ray(px_f);
            let Some(p_unit) = triangulate_normalized_pair(&r_arr, relative.t, [ray_r[0], ray_r[1]], [ray_f[0], ray_f[1]]) else {
                continue;
            };
            let dist_metric = norm3(sub3(point, c_ref));
            let dist_unit = norm3(p_unit);
            if dist_unit < 1e-9 || dist_metric < 1e-9 {
                continue;
            }
            scales.push(dist_metric / dist_unit);
            shared.push((point, px_f));
        }
        let s = if scales.is_empty() {
            if self.cameras.len() < 2 {
                return Err(SfmError::PnpFailed);
            }
            let baseline = norm3(sub3(camera_center(&self.cameras[0].1), camera_center(&self.cameras[1].1)));
            if baseline < 1e-9 {
                return Err(SfmError::PnpFailed);
            }
            baseline
        } else {
            scales.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median = scales[scales.len() / 2];
            if !median.is_finite() || median <= 1e-9 {
                return Err(SfmError::PnpFailed);
            }
            if scales.len() >= 2 {
                let spread = scales[scales.len() - 1] / scales[0];
                if !spread.is_finite() || spread > 1.5 {
                    return Err(SfmError::PnpFailed);
                }
            }
            median
        };
        let scaled = Se3 { r: relative.r, t: scale3(relative.t, s) };
        let pose = CameraPose(scaled.semio_compose_rs(&ref_pose.0));
        if !shared.is_empty() {
            let max_reproj = self.cfg.ransac_threshold_px * 3.0;
            let mut ok = 0usize;
            for &(point, px) in &shared {
                let Some(pred) = reproject(&self.intrinsics, &pose, point) else { continue };
                let err = ((pred[0] - px[0]).powi(2) + (pred[1] - px[1]).powi(2)).sqrt();
                if err <= max_reproj {
                    ok += 1;
                }
            }
            if ok == 0 {
                return Err(SfmError::PnpFailed);
            }
        }
        self.cameras.push((frame, pose));
        Ok(())
    }

    /// 🧵️ Triangulates every not-yet-triangulated track that (a) has an observation in `frame` and (b) is
    /// observed by at least 2 registered cameras overall and meets [`SfmConfig::min_track_length`], via
    /// [`triangulate_and_validate`].
    pub(super) fn triangulate_track(&mut self, track_id: usize, required_frame: Option<usize>) {
        if self.points.contains_key(&track_id) {
            return;
        }
        let Some(track) = self.tracks.tracks.get(track_id) else { return };
        if track.len() < self.cfg.min_track_length || required_frame.is_some_and(|frame| !track.iter().any(|&(observed, _)| observed == frame)) {
            return;
        }
        let mut poses = Vec::with_capacity(track.len().min(MAX_INTERACTIVE_TRACK_OBSERVATIONS));
        let mut observations = Vec::with_capacity(track.len().min(MAX_INTERACTIVE_TRACK_OBSERVATIONS));
        for &(frame, keypoint) in track.iter().take(MAX_INTERACTIVE_TRACK_OBSERVATIONS) {
            let Some(pose) = self.pose_of(frame) else { continue };
            poses.push((pose, self.intrinsics));
            observations.push(self.obs_px(frame, keypoint));
        }
        if poses.len() < 2 {
            return;
        }
        if let Some(point) = triangulate_and_validate(&poses, &observations, self.cfg.min_triangulation_angle_rad, self.cfg.ransac_threshold_px * 3.0) {
            self.points.insert(track_id, point);
            if let Some(events) = self.point_events.as_mut() {
                events.push((track_id, point, true));
            }
        }
    }

    /// 🌱️ Advances seed correspondence collection, fixed-size essential solve, or one triangulation.
    pub fn advance_seed_pair(&mut self, preparation: &mut SeedPairPreparation, work_budget: usize) -> Result<bool, SfmError> {
        match preparation.phase {
            SeedPairPhase::Collect => {
                let end = preparation.cursor.saturating_add(work_budget.max(1)).min(preparation.matches.len());
                for matched in &preparation.matches[preparation.cursor..end] {
                    preparation.correspondences.push((self.obs_px(preparation.frame_a, matched.a), self.obs_px(preparation.frame_b, matched.b)));
                }
                preparation.cursor = end;
                if end == preparation.matches.len() {
                    preparation.phase = SeedPairPhase::Solve;
                }
            }
            SeedPairPhase::Solve => {
                if preparation.correspondences.len() < 8 {
                    return Err(SfmError::InsufficientMatches);
                }
                let threshold = self.cfg.ransac_threshold_px / self.intrinsics.fx.max(self.intrinsics.fy).max(1.0);
                let seed = (preparation.frame_a as u64) << 32 ^ preparation.frame_b as u64 ^ (preparation.attempts as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
                let candidate = estimate_essential_five_point_bounded(&preparation.correspondences, &self.intrinsics, &self.intrinsics, threshold, seed, SEED_HYPOTHESES_PER_CALL, 0);
                let linear = || {
                    let linear = estimate_essential_with_threshold(&preparation.correspondences, &self.intrinsics, &self.intrinsics, threshold, MAX_INTERACTIVE_SEED_HYPOTHESES).ok_or(SfmError::DegenerateGeometry)?;
                    let TwoViewModel::Fundamental(essential) = linear.model else { return Err(SfmError::DegenerateGeometry) };
                    let rays: Vec<([f64; 2], [f64; 2])> = linear
                        .inliers
                        .iter()
                        .take(MAX_INTERACTIVE_SEED_CORRESPONDENCES)
                        .map(|&index| {
                            let left = self.intrinsics.unproject_ray(preparation.correspondences[index].0);
                            let right = self.intrinsics.unproject_ray(preparation.correspondences[index].1);
                            ([left[0], left[1]], [right[0], right[1]])
                        })
                        .collect();
                    decompose_essential(&essential, &rays).ok_or(SfmError::DegenerateGeometry)?;
                    Ok::<TwoViewResult, SfmError>(linear)
                };
                if preparation.attempts == 0 {
                    let spread = |side: usize| {
                        let coordinate = |pair: &([f64; 2], [f64; 2])| if side == 0 { pair.0 } else { pair.1 };
                        let first = coordinate(&preparation.correspondences[0]);
                        preparation.correspondences.iter().map(|pair| { let p = coordinate(pair); (p[0] - first[0]).abs().max((p[1] - first[1]).abs()) }).fold(0.0f64, f64::max)
                    };
                    if spread(0) < 1e-9 || spread(1) < 1e-9 {
                        return Err(SfmError::DegenerateGeometry);
                    }
                }
                if let Some(candidate) = candidate {
                    if candidate.score < preparation.best_raw_score {
                        preparation.best_raw_score = candidate.score;
                        let polished = polish_essential(candidate, &preparation.correspondences, &self.intrinsics, threshold);
                        if preparation.best.as_ref().is_none_or(|best| polished.inliers.len() > best.inliers.len()) {
                            preparation.best = Some(polished);
                        }
                    }
                }
                preparation.attempts += 1;
                if preparation.attempts < SEED_SOLVE_CALLS {
                    return Ok(false);
                }
                let estimate = match preparation.best.take().filter(|estimate| estimate.inliers.len() >= 8) {
                    Some(estimate) => estimate,
                    None => linear()?,
                };
                let TwoViewModel::Fundamental(essential) = estimate.model else { return Err(SfmError::DegenerateGeometry) };
                let rays: Vec<([f64; 2], [f64; 2])> = estimate
                    .inliers
                    .iter()
                    .take(MAX_INTERACTIVE_SEED_CORRESPONDENCES)
                    .map(|&index| {
                        let left = self.intrinsics.unproject_ray(preparation.correspondences[index].0);
                        let right = self.intrinsics.unproject_ray(preparation.correspondences[index].1);
                        ([left[0], left[1]], [right[0], right[1]])
                    })
                    .collect();
                let relative = decompose_essential(&essential, &rays).ok_or(SfmError::DegenerateGeometry)?;
                self.cameras.clear();
                self.points.clear();
                self.cameras.push((preparation.frame_a, CameraPose(Se3::identity())));
                self.cameras.push((preparation.frame_b, CameraPose(relative)));
                preparation.cursor = 0;
                preparation.phase = SeedPairPhase::Triangulate;
            }
            SeedPairPhase::Triangulate => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS) {
                        preparation.phase = SeedPairPhase::Done;
                        break;
                    }
                    self.triangulate_track(preparation.cursor, None);
                    preparation.cursor += 1;
                }
            }
            SeedPairPhase::Done => {}
        }
        Ok(preparation.phase == SeedPairPhase::Done)
    }

    /// 🎯️ Advances one frame through correspondence collection, one-iteration PnP attempts, and
    /// per-track triangulation. No solve call receives more than 64 correspondences or one RANSAC draw.
    pub fn advance_registration(&mut self, preparation: &mut RegistrationPreparation, work_budget: usize) -> Result<bool, SfmError> {
        match preparation.phase {
            RegistrationPhase::Collect => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS) {
                        preparation.phase = RegistrationPhase::Solve;
                        break;
                    }
                    let track_id = preparation.cursor;
                    preparation.cursor += 1;
                    let Some(&point) = self.points.get(&track_id) else { continue };
                    let Some(pixel) = self.track_obs_in(&self.tracks.tracks[track_id], preparation.frame) else { continue };
                    if preparation.world_points.len() < MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES {
                        preparation.world_points.push(point);
                        preparation.observations.push(pixel);
                    }
                }
            }
            RegistrationPhase::Solve => {
                if preparation.world_points.len() < P3pSolver::SAMPLE_SIZE {
                    preparation.phase = RegistrationPhase::TwoViewSelect;
                } else {
                    let config = RansacConfig { threshold: self.cfg.ransac_threshold_px * INTERACTIVE_REGISTRATION_THRESHOLD_FACTOR, confidence: 0.5, max_iters: 1, seed: preparation.frame as u64 ^ preparation.attempts.wrapping_mul(0x9E37_79B9), scoring: RansacScoring::Msac };
                    preparation.attempts += 1;
                    if let Some((pose, inliers)) = pnp_ransac_bounded(&self.intrinsics, &preparation.world_points, &preparation.observations, &config, INTERACTIVE_POSE_REFINE_ITERATIONS) {
                        if preparation.best.as_ref().is_none_or(|(_, best)| inliers.len() > *best) {
                            preparation.best = Some((pose, inliers.len()));
                        }
                    }
                    let correspondences = preparation.world_points.len() as f64;
                    let share = preparation.best.as_ref().map_or(0.0, |(_, inliers)| *inliers as f64 / correspondences);
                    let settled = preparation.attempts >= MAX_INTERACTIVE_REGISTRATION_ATTEMPTS || (preparation.attempts >= MIN_INTERACTIVE_REGISTRATION_ATTEMPTS && share >= EARLY_INTERACTIVE_REGISTRATION_INLIER_SHARE);
                    if settled {
                        match preparation.best.take() {
                            Some((pose, inliers)) if inliers >= 2 * P3pSolver::SAMPLE_SIZE && share >= MIN_INTERACTIVE_REGISTRATION_INLIER_SHARE => {
                                self.cameras.push((preparation.frame, pose));
                                preparation.cursor = 0;
                                preparation.phase = RegistrationPhase::Triangulate;
                            }
                            _ => preparation.phase = RegistrationPhase::TwoViewSelect,
                        }
                    }
                }
            }
            RegistrationPhase::TwoViewSelect => {
                let registered: std::collections::BTreeSet<usize> = self.cameras.iter().map(|&(frame, _)| frame).collect();
                let mut best: Option<(usize, usize)> = None;
                for &(a, b, ref matches) in &self.pairwise_matches {
                    if matches.len() < 8 {
                        continue;
                    }
                    let reference = if a == preparation.frame && registered.contains(&b) {
                        b
                    } else if b == preparation.frame && registered.contains(&a) {
                        a
                    } else {
                        continue;
                    };
                    if best.is_none_or(|(_, count)| matches.len() > count) {
                        best = Some((reference, matches.len()));
                    }
                }
                match best {
                    Some((reference, _)) => {
                        preparation.two_view = Some(Box::new(TwoViewFallback {
                            reference,
                            correspondences: Vec::with_capacity(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES),
                            attempts: 0,
                            best: None,
                            relative: None,
                            cursor: 0,
                            scales: Vec::new(),
                            shared: Vec::new(),
                        }));
                        preparation.phase = RegistrationPhase::TwoViewCollect;
                    }
                    None => preparation.phase = RegistrationPhase::Failed,
                }
            }
            RegistrationPhase::TwoViewCollect => {
                let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                let mut collected: Vec<([f64; 2], [f64; 2])> = Vec::new();
                for &(a, b, ref matches) in &self.pairwise_matches {
                    let flip = if a == fallback.reference && b == preparation.frame {
                        false
                    } else if a == preparation.frame && b == fallback.reference {
                        true
                    } else {
                        continue;
                    };
                    let stride = matches.len().div_ceil(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES).max(1);
                    for matched in matches.iter().step_by(stride) {
                        let pair = if flip { (self.obs_px(fallback.reference, matched.b), self.obs_px(preparation.frame, matched.a)) } else { (self.obs_px(fallback.reference, matched.a), self.obs_px(preparation.frame, matched.b)) };
                        if collected.len() < MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES {
                            collected.push(pair);
                        }
                    }
                }
                fallback.correspondences = collected;
                preparation.phase = if fallback.correspondences.len() >= 8 { RegistrationPhase::TwoViewSolve } else { RegistrationPhase::Failed };
            }
            RegistrationPhase::TwoViewSolve => {
                const FIVE_POINT_THRESHOLD: f64 = 0.005;
                let intrinsics = self.intrinsics;
                let frame = preparation.frame;
                let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                let seed = (frame as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ fallback.attempts as u64;
                let candidates = [
                    estimate_essential_five_point_bounded(&fallback.correspondences, &intrinsics, &intrinsics, FIVE_POINT_THRESHOLD, seed, INTERACTIVE_TWO_VIEW_HYPOTHESES_PER_CALL, 0),
                    estimate_essential_with_limit(&fallback.correspondences, &intrinsics, &intrinsics, INTERACTIVE_TWO_VIEW_HYPOTHESES_PER_CALL),
                ];
                for candidate in candidates.into_iter().flatten() {
                    if fallback.best.as_ref().is_none_or(|best| candidate.score < best.score) {
                        fallback.best = Some(candidate);
                    }
                }
                fallback.attempts += 1;
                if fallback.attempts < MAX_INTERACTIVE_TWO_VIEW_ATTEMPTS {
                    return Ok(false);
                }
                if let Some(best) = fallback.best.take() {
                    fallback.best = Some(polish_essential(best, &fallback.correspondences, &intrinsics, FIVE_POINT_THRESHOLD));
                }
                let relative = fallback.best.as_ref().and_then(|estimate| {
                    let TwoViewModel::Fundamental(essential) = estimate.model else { return None };
                    let rays: Vec<([f64; 2], [f64; 2])> = estimate
                        .inliers
                        .iter()
                        .take(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES)
                        .map(|&index| {
                            let left = intrinsics.unproject_ray(fallback.correspondences[index].0);
                            let right = intrinsics.unproject_ray(fallback.correspondences[index].1);
                            ([left[0], left[1]], [right[0], right[1]])
                        })
                        .collect();
                    decompose_essential(&essential, &rays)
                });
                match relative {
                    Some(relative) => {
                        fallback.relative = Some(relative);
                        fallback.cursor = 0;
                        preparation.phase = RegistrationPhase::TwoViewScale;
                    }
                    None => preparation.phase = RegistrationPhase::Failed,
                }
            }
            RegistrationPhase::TwoViewScale => {
                let frame = preparation.frame;
                let limit = self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS);
                for _ in 0..work_budget.max(1) {
                    let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                    if fallback.cursor >= limit {
                        preparation.phase = RegistrationPhase::TwoViewCommit;
                        break;
                    }
                    let track_id = fallback.cursor;
                    fallback.cursor += 1;
                    let reference = fallback.reference;
                    let relative = fallback.relative.expect("solved relative pose");
                    let Some(&point) = self.points.get(&track_id) else { continue };
                    let track = &self.tracks.tracks[track_id];
                    let (Some(pixel_reference), Some(pixel_frame)) = (self.track_obs_in(track, reference), self.track_obs_in(track, frame)) else { continue };
                    let ray_reference = self.intrinsics.unproject_ray(pixel_reference);
                    let ray_frame = self.intrinsics.unproject_ray(pixel_frame);
                    let rotation = mat3d_to_array(&relative.r.0);
                    let Some(unit_point) = triangulate_normalized_pair(&rotation, relative.t, [ray_reference[0], ray_reference[1]], [ray_frame[0], ray_frame[1]]) else { continue };
                    let Some(reference_pose) = self.pose_of(reference) else { continue };
                    let metric_distance = norm3(sub3(point, camera_center(&reference_pose)));
                    let unit_distance = norm3(unit_point);
                    if unit_distance < 1e-9 || metric_distance < 1e-9 {
                        continue;
                    }
                    let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                    if fallback.scales.len() < MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES {
                        fallback.scales.push(metric_distance / unit_distance);
                        fallback.shared.push((point, pixel_frame));
                    }
                }
            }
            RegistrationPhase::TwoViewCommit => {
                let frame = preparation.frame;
                let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                let relative = fallback.relative.expect("solved relative pose");
                let Some(reference_pose) = self.pose_of(fallback.reference) else {
                    preparation.phase = RegistrationPhase::Failed;
                    return Ok(false);
                };
                let scale = if fallback.scales.is_empty() {
                    if self.cameras.len() < 2 {
                        preparation.phase = RegistrationPhase::Failed;
                        return Ok(false);
                    }
                    let baseline = norm3(sub3(camera_center(&self.cameras[0].1), camera_center(&self.cameras[1].1)));
                    if baseline < 1e-9 {
                        preparation.phase = RegistrationPhase::Failed;
                        return Ok(false);
                    }
                    baseline
                } else {
                    fallback.scales.sort_by(f64::total_cmp);
                    let median = fallback.scales[fallback.scales.len() / 2];
                    let spread = fallback.scales[fallback.scales.len() - 1] / fallback.scales[0];
                    if !median.is_finite() || median <= 1e-9 || !spread.is_finite() || (fallback.scales.len() >= 2 && spread > 1.5) {
                        preparation.phase = RegistrationPhase::Failed;
                        return Ok(false);
                    }
                    median
                };
                let pose = CameraPose(Se3 { r: relative.r, t: scale3(relative.t, scale) }.semio_compose_rs(&reference_pose.0));
                let maximum_reprojection = self.cfg.ransac_threshold_px * 3.0;
                let consistent = fallback.shared.iter().any(|&(point, pixel)| reproject(&self.intrinsics, &pose, point).is_some_and(|predicted| ((predicted[0] - pixel[0]).powi(2) + (predicted[1] - pixel[1]).powi(2)).sqrt() <= maximum_reprojection));
                if !fallback.shared.is_empty() && !consistent {
                    preparation.phase = RegistrationPhase::Failed;
                    return Ok(false);
                }
                self.cameras.push((frame, pose));
                preparation.two_view = None;
                preparation.cursor = 0;
                preparation.phase = RegistrationPhase::Triangulate;
            }
            RegistrationPhase::Triangulate => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS) {
                        preparation.phase = RegistrationPhase::Done;
                        break;
                    }
                    self.triangulate_track(preparation.cursor, Some(preparation.frame));
                    preparation.cursor += 1;
                }
            }
            RegistrationPhase::Done => return Ok(true),
            RegistrationPhase::Failed => return Err(SfmError::PnpFailed),
        }
        Ok(preparation.phase == RegistrationPhase::Done)
    }
}
// #endregion 🔖️Incremental
