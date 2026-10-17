//! 🎯️ Tracking and trajectories of the videogrammetry: the vector helpers, the 2D and multi-object trackers and the 3D trajectory triangulation. See [`super`].

use super::*;

// #region 🔖️Vec3Helpers
pub(super) fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub(super) fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub(super) fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

pub(super) fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = sub3(a, b);
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

pub(super) fn mat3_mul(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| (0..3).map(|k| a[r][k] * b[k][c]).sum()))
}
// #endregion 🔖️Vec3Helpers

// #region 🔖️Track2d
/// 🧵️ One 2D feature's optical-flow trajectory across frames: track identity plus `(frame_idx, x, y)`
/// samples in the order they were observed. A track that fails forward-backward consistency is dropped
/// from further extension but keeps its recorded history.
#[derive(Clone, Debug, PartialEq)]
pub struct Track2d {
    pub id: u32,
    pub samples: Vec<(u32, f32, f32)>,
}

const TRACK2D_FB_MAX_ERROR: f32 = 1.0;

/// 🎥️ Stateful multi-frame 2D point tracker: advances every live [`Track2d`] one frame at a time via
/// pyramidal KLT ([`klt_track`] + [`forward_backward_prune`]), drops any track whose point becomes
/// invalid, and refills spatial coverage by re-detecting fresh corners ([`shi_tomasi_grid`]) in grid
/// cells that have too few live tracks. Dropped tracks stay in [`Tracker2d::tracks`]'s history (so
/// downstream trajectory/kinematics code keeps their full sample history) but are never extended again.
#[derive(Clone, Debug, Default)]
pub struct Tracker2d {
    pub(super) tracks: Vec<Track2d>,
    live: Vec<usize>,
    next_id: u32,
}

impl Tracker2d {
    /// 🆕️ An empty tracker with no live or historical tracks.
    pub fn new() -> Self {
        Self { tracks: Vec::new(), live: Vec::new(), next_id: 0 }
    }

    /// 👣️ Advances every live track from `pyr_prev` into `pyr_curr`, drops points that fail forward-backward
    /// consistency or leave the image, then re-detects new corners (via [`shi_tomasi_grid`] over a
    /// `redetect_grid`-pixel cell grid on `pyr_curr`'s finest level) in any cell holding fewer than
    /// `redetect_per_cell` live tracks, spawning a fresh [`Track2d`] per new corner. Returns the full
    /// (live + historical) track list, same as [`Tracker2d::tracks`].
    #[allow(clippy::too_many_arguments, reason = "one argument per physically distinct KLT/redetect tuning knob; a config struct would just move the same 7 fields one level down for this single call site")]
    pub fn step(&mut self, pyr_prev: &Pyramid, pyr_curr: &Pyramid, frame_idx: u32, window_radius: i32, max_iters: usize, redetect_grid: u32, redetect_per_cell: usize) -> &[Track2d] {
        if pyr_prev.levels.is_empty() || pyr_curr.levels.is_empty() {
            return &self.tracks;
        }
        let points: Vec<(f32, f32)> = self
            .live
            .iter()
            .map(|&i| {
                let &(_, x, y) = self.tracks[i].samples.last().expect("live index always has >=1 sample");
                (x, y)
            })
            .collect();
        let mut tracked = klt_track(pyr_prev, pyr_curr, &points, window_radius, max_iters);
        forward_backward_prune(pyr_prev, pyr_curr, &points, &mut tracked, window_radius, max_iters, TRACK2D_FB_MAX_ERROR);

        let mut still_live = Vec::with_capacity(self.live.len());
        for (&idx, tp) in self.live.iter().zip(tracked.iter()) {
            if tp.valid {
                self.tracks[idx].samples.push((frame_idx, tp.x, tp.y));
                still_live.push(idx);
            }
        }
        self.live = still_live;

        let finest = &pyr_curr.levels[0];
        let cell = redetect_grid.max(1);
        let cells_x = finest.width.div_ceil(cell).max(1);
        let cells_y = finest.height.div_ceil(cell).max(1);
        let mut counts = vec![0usize; (cells_x * cells_y) as usize];
        for &idx in &self.live {
            let &(_, x, y) = self.tracks[idx].samples.last().expect("live index always has >=1 sample");
            counts[bucket_index(x, y, cell, cells_x, cells_y)] += 1;
        }
        for (x, y, _score) in shi_tomasi_grid(finest, cell, redetect_per_cell) {
            let bucket = bucket_index(x as f32, y as f32, cell, cells_x, cells_y);
            if counts[bucket] >= redetect_per_cell {
                continue;
            }
            counts[bucket] += 1;
            let id = self.next_id;
            self.next_id += 1;
            self.tracks.push(Track2d { id, samples: vec![(frame_idx, x as f32, y as f32)] });
            self.live.push(self.tracks.len() - 1);
        }
        &self.tracks
    }

    /// 📜️ Every track this tracker has ever produced (live or dropped), in creation order.
    pub fn tracks(&self) -> &[Track2d] {
        &self.tracks
    }
}

fn bucket_index(x: f32, y: f32, cell: u32, cells_x: u32, cells_y: u32) -> usize {
    let bx = ((x.max(0.0) as u32) / cell).min(cells_x.saturating_sub(1));
    let by = ((y.max(0.0) as u32) / cell).min(cells_y.saturating_sub(1));
    (by * cells_x + bx) as usize
}
// #endregion 🔖️Track2d

// #region 🔖️Mot
/// 📍️ One raw 2D detection to associate against live tracks, with an optional external id hint (unused by
/// the gating/association logic itself, carried through purely as caller metadata).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Detection {
    pub x: f32,
    pub y: f32,
    pub id_hint: Option<u32>,
}

/// 🚦️ One multi-object track's constant-velocity state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackState {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub age: u32,
    pub missed: u32,
}

const MOT_ALPHA: f32 = 0.6;

pub(super) const MOT_MAX_MISSED: u32 = 5;

/// 🕵️ Multi-object α–β tracker over 2D detections: predicts every track forward at constant velocity,
/// associates detections by gated nearest-cost greedy matching, and spawns/prunes tracks for
/// unmatched detections/tracks. `crate::graph_matching` (this crate's originally-planned
/// max-weight-matching dependency) is an unimplemented one-line stub with no public API — confirmed by
/// direct inspection and reflected in this crate's `Cargo.toml`, which no longer depends on it — so
/// [`MultiObjectTracker::update`] solves the gated assignment with a real, working, greedy
/// ascending-cost matcher instead: a documented, simpler substitute for true max-weight bipartite
/// matching, adequate for well-separated tracks/detections.
#[derive(Clone, Debug, Default)]
pub struct MultiObjectTracker {
    pub(super) tracks: Vec<TrackState>,
    next_id: u32,
}

impl MultiObjectTracker {
    /// 🎬️ An empty tracker with no live tracks.
    pub fn new() -> Self {
        Self { tracks: Vec::new(), next_id: 0 }
    }

    /// 🔮️ Constant-velocity prediction step: advances every live track's `(x, y)` by `(vx, vy) * dt`.
    pub fn predict(&mut self, dt: f32) {
        for t in &mut self.tracks {
            t.x += t.vx * dt;
            t.y += t.vy * dt;
        }
    }

    /// 🔄️ Associates `detections` against live tracks: builds a Euclidean-distance cost matrix, gates out
    /// pairs farther apart than `gate_radius`, then greedily accepts the globally-cheapest remaining pair
    /// repeatedly (a documented simplified substitute for max-weight bipartite matching — see this type's
    /// docs). Matched tracks blend the measured `(dx, dy)` displacement into `(vx, vy)` with a fixed gain
    /// ([`MOT_ALPHA`]); unmatched tracks accumulate a miss and are pruned once `missed` exceeds
    /// [`MOT_MAX_MISSED`]; unmatched detections spawn fresh tracks. Returns `(track_id, detection_index)`
    /// pairs for every detection associated with a track this call — both re-matched existing tracks and
    /// freshly spawned ones, so callers always learn the id backing every input detection.
    pub fn update(&mut self, detections: &[Detection], gate_radius: f32) -> Vec<(u32, usize)> {
        let mut candidates: Vec<(f32, usize, usize)> = Vec::new();
        for (ti, t) in self.tracks.iter().enumerate() {
            for (di, d) in detections.iter().enumerate() {
                let dist = (t.x - d.x).hypot(t.y - d.y);
                if dist <= gate_radius {
                    candidates.push((dist, ti, di));
                }
            }
        }
        candidates.sort_by(|a, b| a.0.total_cmp(&b.0));

        let mut track_used = vec![false; self.tracks.len()];
        let mut det_used = vec![false; detections.len()];
        let mut assignments = Vec::new();
        for (_, ti, di) in candidates {
            if track_used[ti] || det_used[di] {
                continue;
            }
            track_used[ti] = true;
            det_used[di] = true;
            assignments.push((ti, di));
        }

        let mut result = Vec::with_capacity(assignments.len());
        for (ti, di) in assignments {
            let det = detections[di];
            let t = &mut self.tracks[ti];
            let (mvx, mvy) = (det.x - t.x, det.y - t.y);
            t.vx = t.vx * (1.0 - MOT_ALPHA) + mvx * MOT_ALPHA;
            t.vy = t.vy * (1.0 - MOT_ALPHA) + mvy * MOT_ALPHA;
            t.x = det.x;
            t.y = det.y;
            t.age += 1;
            t.missed = 0;
            result.push((t.id, di));
        }
        for (ti, t) in self.tracks.iter_mut().enumerate() {
            if !track_used[ti] {
                t.missed += 1;
            }
        }
        self.tracks.retain(|t| t.missed <= MOT_MAX_MISSED);
        for (di, det) in detections.iter().enumerate() {
            if !det_used[di] {
                let id = self.next_id;
                self.next_id += 1;
                self.tracks.push(TrackState { id, x: det.x, y: det.y, vx: 0.0, vy: 0.0, age: 0, missed: 0 });
                result.push((id, di));
            }
        }
        result
    }
}
// #endregion 🔖️Mot

// #region 🔖️Trajectory3d
/// 🛤️ A recovered 3D position time-series: `(timestamp, [x, y, z])` samples, one per successfully
/// triangulated frame of a [`Track2d`].
#[derive(Clone, Debug, PartialEq)]
pub struct Trajectory3d {
    pub samples: Vec<(f64, [f64; 3])>,
}

const TRAJECTORY_WINDOW_RADIUS: usize = 3;

/// 📐️ Triangulates every track into a moving 3D trajectory from a **single moving camera** whose per-frame
/// poses are already known (e.g. from prior SfM) — `cams_per_frame` is expected to hold at most one
/// `(CameraPose, Intrinsics)` per frame index, not a synced multi-camera rig. A lone camera cannot
/// triangulate an instantaneous position from a single observation, so each output sample instead reuses a
/// short sliding window of up to `2 * TRAJECTORY_WINDOW_RADIUS + 1` neighbouring frames' poses and this
/// track's pixel observation at each of them as a multi-view [`crate::editor::remodeling::engine::sfm::geometry::triangulate_dlt`] problem,
/// under the documented quasi-static approximation that the tracked point moves little relative to the
/// camera's baseline change across that short window. This is a deliberate simplification of the more
/// general synced-rig formulation (multiple simultaneous observations at one frame index), which the flat
/// [`Track2d`] type cannot represent without a per-observation camera id.
pub fn triangulate_tracks(cams_per_frame: &[(u32, CameraPose, Intrinsics)], tracks: &[Track2d], frame_timestamps: &std::collections::HashMap<u32, f64>) -> Vec<Trajectory3d> {
    let mut pose_by_frame: std::collections::HashMap<u32, (CameraPose, Intrinsics)> = std::collections::HashMap::new();
    for &(frame, pose, intr) in cams_per_frame {
        pose_by_frame.entry(frame).or_insert((pose, intr));
    }
    tracks
        .iter()
        .map(|track| {
            let mut samples = Vec::new();
            for (i, &(frame, _, _)) in track.samples.iter().enumerate() {
                let Some(&ts) = frame_timestamps.get(&frame) else { continue };
                let lo = i.saturating_sub(TRAJECTORY_WINDOW_RADIUS);
                let hi = (i + TRAJECTORY_WINDOW_RADIUS).min(track.samples.len() - 1);
                let mut poses = Vec::new();
                let mut obs = Vec::new();
                for &(wframe, wx, wy) in &track.samples[lo..=hi] {
                    if let Some(&(pose, intr)) = pose_by_frame.get(&wframe) {
                        poses.push((pose, intr));
                        obs.push([f64::from(wx), f64::from(wy)]);
                    }
                }
                if poses.len() < 2 {
                    continue;
                }
                if let Some(p) = semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::triangulate_dlt(&poses, &obs) {
                    samples.push((ts, p));
                }
            }
            Trajectory3d { samples }
        })
        .collect()
}
// #endregion 🔖️Trajectory3d
