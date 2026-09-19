//! 🧪️ `synthetic-orbit` — fixture integrity, its deterministic regeneration routine, and the
//! plugin's first end-to-end reconstruction run scored against ground truth.
//!
//! 🔬️ Oracles: the committed PNGs are decoded by the `png` crate (this crate's only test oracle
//! dependency) as well as by the plugin's own codec, and their pixels are re-derived from scratch by
//! [`render_reference_frame`] — an independent third implementation of the same scene lives in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🐍️synthetic-orbit-fixture.py`
//! (stdlib Python), which produced the committed bytes. PNG containers differ between encoders;
//! decoded rasters do not, and that equality is what these tests assert.

use super::{CAMERA_ID, FRAMES, FRAME_MIME, GROUND_TRUTH_JSON, ID, PRIMARY_TEXT, STREAM_ID};
use crate::editor::remodeling::commands::import_frame_payload::ImportFramePayload;
use crate::editor::remodeling::engine::images as remodeling_image;
use crate::editor::remodeling::unit_tests::context::{app_with_registry, close, dispatch, durable, host_turn, pump_run, settle, run_action, run_arguments, start_reconstruction, RemodelingApp};
use crate::editor::remodeling::RemodelingCommand;
use crate::lie::{umeyama, Quatd, Sim3, So3};
use crate::{CameraCalibration, FrameRef, MediaKind, MediaStream, RemodelingSnapshot};

//#region 🔖️FixtureConstants
/// 🎲 splitmix64 seed shared with the Python reference generator.
const SEED: u64 = 0x5EED_0B17_5CE9_E000;
/// 🛰️ Views around the orbit, 10° apart. The step is what the matcher can bridge: the bounded ORB
/// features pair correctly across 5–10° of this orbit (83 of 91 accepted matches within 2 px at 10°),
/// degrade past 15° and are noise at 36° — a walked or flown capture samples far denser than that,
/// and ten views 36° apart never were a capture the pipeline could register.
const FRAME_COUNT: usize = 36;
const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
/// 📷️ Phone-like focal ratio of the rendering camera: `fx = fy = 0.85 · width`.
const FOCAL_RATIO: f64 = 0.85;
const FX: f64 = FOCAL_RATIO * WIDTH as f64;
const CX: f64 = WIDTH as f64 / 2.0;
const CY: f64 = HEIGHT as f64 / 2.0;
const K1: f64 = -0.02;
const K2: f64 = 0.005;
const CUBE_HALF: f64 = 1.0;
const ORBIT_RADIUS: f64 = 2.6;
const ORBIT_ELEVATION: f64 = 2.2;
const MARKERS_PER_FACE: usize = 22;
/// 🧱️ Face texture: two octaves of seeded value noise in the face's `(u, v)` plane, cells of these
/// side lengths (world units) with these luminance swings. A flat face carries no local structure
/// a gradient descriptor can tell apart — every marker edge looks like every other — so the faces
/// are "painted" with a blocky pattern whose cell junctions are the corners that real surfaces
/// (plaster, timber, brick) offer a detector; the coarse octave spans ~12 px, the fine ~5 px in
/// the nearest view.
const TEXTURE_CELLS: [(f64, f64); 1] = [(0.16, 0.45)];
const FPS_HINT: f64 = 2.0;
const BACKGROUND: [u8; 3] = [35, 35, 40];
const FACE_BASE_COLORS: [[u8; 3]; 6] = [[150, 60, 60], [60, 60, 150], [60, 150, 60], [150, 150, 60], [150, 60, 150], [60, 150, 150]];
const MARKER_COLORS: [[u8; 3]; 6] = [[250, 250, 250], [15, 15, 15], [245, 190, 40], [40, 200, 245], [245, 60, 130], [120, 245, 90]];

/// ☁️ At least this share of the 140 ground-truth world points must be recovered into the committed
/// sparse cloud. Deliberately modest: the pipeline triangulates AKAZE keypoints, not the fixture's
/// marker centres, so this scores coverage of the structure rather than a point-to-point identity.
const MIN_SPARSE_POINTS_PER_TRUTH_POINT: f64 = 0.25;

/// 🎯️ Ground-truth pose tolerances after Sim(3) alignment.
const MAX_ROTATION_ERROR_DEG: f64 = 2.0;
const MAX_TRANSLATION_RMSE_RATIO: f64 = 0.03;

/// 🕳️ Tolerance multiplier applied to the two constants above at APP level. It is `1.0` because
/// calibration is now a wired stage: `build_engine_params` (`✏️editor/⚙️engine/🦀️.rs`) reads the
/// document's `calibration.cameras` through `assumed_focal_ratio`, so an app-level run reconstructs
/// this fixture at the true `0.85 · width` focal instead of the engine's `fx = fy = max(w, h)` guess,
/// and the app-level bounds are the strict engine-level ones.
const UNCALIBRATED_GAUGE_SLACK: f64 = 1.0;
/// 📐️ Mesh accuracy bounds, in cube half-sizes: a fused surface of the textured cube, not a shell
/// around it. The interactive surface is capped at 512 elements, so the cube comes out as a net of
/// ~0.4-half-size cells with rounded edges and a hole-filled underside no camera of the orbit sees:
/// measured 0.11 median / 0.36 p90 (2026-09-19). A shell fused out of the background measured
/// 1.09 / 2.09, the thin-band surface before the observed-only extraction 0.12 / 0.33 at 29 %
/// completeness.
const MAX_MESH_MEDIAN_SURFACE_ERROR: f64 = 0.15;
const MAX_MESH_P90_SURFACE_ERROR: f64 = 0.5;
/// 📐️ Mesh completeness: the share of truth surface points a vertex lies within this distance of.
/// A sixth of the truth points lie on the unobserved bottom face (measured 84 %).
const MESH_COMPLETENESS_RADIUS: f64 = 0.4;
const MIN_MESH_COMPLETENESS: f64 = 0.75;
//#endregion 🔖️FixtureConstants

//#region 🎲️Rng
/// 🎲 splitmix64 — the reference generator's RNG, reproduced here so the renderer is a real second
/// implementation rather than a replay of stored parameters.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_f64(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64) * (1.0 / 9_007_199_254_740_992.0)
    }

    fn unit(&mut self) -> f64 {
        self.next_f64() * 2.0 - 1.0
    }
}
//#endregion 🎲️Rng

//#region 📐️Vec3
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn norm3(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

fn normalize3(a: [f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-300 {
        a
    } else {
        [a[0] / n, a[1] / n, a[2] / n]
    }
}

fn mat_act(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2], m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2], m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2]]
}
//#endregion 📐️Vec3

//#region 🎥️ReferenceRenderer
/// 🎥️ Camera-to-world rotation of the view that looks from `eye` at the origin — the column form
/// `(right, true_up, forward)` used by `📸️sfm/🦀️.rs`'s own `look_at_pose`.
fn camera_to_world_rotation(eye: [f64; 3]) -> [[f64; 3]; 3] {
    let forward = normalize3(sub3([0.0, 0.0, 0.0], eye));
    let right = normalize3(cross3([0.0, 1.0, 0.0], forward));
    let true_up = cross3(forward, right);
    [[right[0], true_up[0], forward[0]], [right[1], true_up[1], forward[1]], [right[2], true_up[2], forward[2]]]
}

/// 🛰️ The ten orbit camera centres, jittered from one seed.
fn camera_eyes() -> Vec<[f64; 3]> {
    let mut rng = SplitMix64(SEED ^ 0xA5A5_A5A5_A5A5_A5A5);
    (0..FRAME_COUNT)
        .map(|i| {
            let angle = std::f64::consts::TAU * (i as f64 + 0.5) / FRAME_COUNT as f64 + rng.unit() * 0.06;
            let elevation = ORBIT_ELEVATION + rng.unit() * 0.10;
            let radius = ORBIT_RADIUS + rng.unit() * 0.06;
            [radius * angle.cos(), elevation, radius * angle.sin()]
        })
        .collect()
}

/// 🎯️ One face marker: centre in the face's own `(u, v)` plane, radius, colour.
struct FaceMarker {
    u: f64,
    v: f64,
    radius: f64,
    color: [u8; 3],
}

fn face_index(axis: usize, positive: bool) -> usize {
    axis * 2 + usize::from(!positive)
}

fn generate_markers() -> Vec<Vec<FaceMarker>> {
    let mut rng = SplitMix64(SEED);
    (0..6)
        .map(|_| {
            (0..MARKERS_PER_FACE)
                .map(|_| {
                    let u = rng.unit() * CUBE_HALF * 0.78;
                    let v = rng.unit() * CUBE_HALF * 0.78;
                    let radius = CUBE_HALF * (0.055 + 0.04 * rng.next_f64());
                    let color = MARKER_COLORS[(rng.next_u64() % MARKER_COLORS.len() as u64) as usize];
                    FaceMarker { u, v, radius, color }
                })
                .collect()
        })
        .collect()
}

fn marker_world_position(axis: usize, positive: bool, u: f64, v: f64) -> [f64; 3] {
    let signed = if positive { CUBE_HALF } else { -CUBE_HALF };
    match axis {
        0 => [signed, u, v],
        1 => [u, signed, v],
        _ => [u, v, signed],
    }
}

/// 🔮️ The fixture's ground-truth world points: every face-marker centre plus the eight cube corners.
fn reference_points(markers: &[Vec<FaceMarker>]) -> Vec<[f64; 3]> {
    let mut points = Vec::new();
    for axis in 0..3 {
        for positive in [true, false] {
            for marker in &markers[face_index(axis, positive)] {
                points.push(marker_world_position(axis, positive, marker.u, marker.v));
            }
        }
    }
    for sx in [-CUBE_HALF, CUBE_HALF] {
        for sy in [-CUBE_HALF, CUBE_HALF] {
            for sz in [-CUBE_HALF, CUBE_HALF] {
                points.push([sx, sy, sz]);
            }
        }
    }
    points
}

fn distort(p: [f64; 2]) -> [f64; 2] {
    let r2 = p[0] * p[0] + p[1] * p[1];
    let radial = 1.0 + K1 * r2 + K2 * r2 * r2;
    [p[0] * radial, p[1] * radial]
}

/// 🔬️ Mirror of `📷️camera/🦀️.rs`'s `Intrinsics::newton_undistort` (8 steps, `eps = 1e-6`).
fn newton_undistort(distorted: [f64; 2]) -> [f64; 2] {
    let mut p = distorted;
    let eps = 1e-6;
    for _ in 0..8 {
        let fp = distort(p);
        let residual = [fp[0] - distorted[0], fp[1] - distorted[1]];
        if residual[0].abs() < 1e-14 && residual[1].abs() < 1e-14 {
            break;
        }
        let fx0 = distort([p[0] + eps, p[1]]);
        let fy0 = distort([p[0], p[1] + eps]);
        let j = [[(fx0[0] - fp[0]) / eps, (fy0[0] - fp[0]) / eps], [(fx0[1] - fp[1]) / eps, (fy0[1] - fp[1]) / eps]];
        let det = j[0][0] * j[1][1] - j[0][1] * j[1][0];
        if det.abs() < 1e-300 {
            break;
        }
        let dx = (j[1][1] * residual[0] - j[0][1] * residual[1]) / det;
        let dy = (j[0][0] * residual[1] - j[1][0] * residual[0]) / det;
        p = [p[0] - dx, p[1] - dy];
    }
    p
}

fn unproject_ray(px: f64, py: f64) -> [f64; 3] {
    let yd = (py - CY) / FX;
    let xd = (px - CX) / FX;
    let p = newton_undistort([xd, yd]);
    [p[0], p[1], 1.0]
}

/// 📦️ Slab ray/box intersection against the origin-centred cube; returns the hit point and the axis
/// whose slab produced the entry.
fn ray_box_intersect(origin: [f64; 3], direction: [f64; 3]) -> Option<([f64; 3], usize)> {
    let mut t_min = -1.0e30;
    let mut t_max = 1.0e30;
    let mut hit_axis = 0usize;
    for axis in 0..3 {
        let d = direction[axis];
        let o = origin[axis];
        if d.abs() < 1e-12 {
            if o.abs() > CUBE_HALF {
                return None;
            }
            continue;
        }
        let t1 = (-CUBE_HALF - o) / d;
        let t2 = (CUBE_HALF - o) / d;
        let (lo, hi) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
        if lo > t_min {
            t_min = lo;
            hit_axis = axis;
        }
        if hi < t_max {
            t_max = hi;
        }
    }
    if t_max < t_min.max(0.0) {
        return None;
    }
    let t = if t_min > 0.0 { t_min } else { t_max };
    if t <= 0.0 {
        return None;
    }
    Some(([origin[0] + direction[0] * t, origin[1] + direction[1] * t, origin[2] + direction[2] * t], hit_axis))
}

/// 🧱️ Multiplicative shade of the face texture at `(u, v)`: one seeded hash per (face, octave,
/// cell), so every view samples the identical pattern.
fn texture_shade(face: usize, u: f64, v: f64) -> f64 {
    let mut shade = 1.0;
    for (octave, &(cell, swing)) in TEXTURE_CELLS.iter().enumerate() {
        let iu = (u / cell).floor() as i64;
        let iv = (v / cell).floor() as i64;
        let mut rng = SplitMix64(SEED ^ ((face as u64) << 56) ^ ((octave as u64) << 48) ^ ((iu as u64).wrapping_mul(0x9E37_79B9)) ^ ((iv as u64).wrapping_mul(0x85EB_CA6B) << 20));
        shade *= 1.0 + swing * rng.unit();
    }
    shade
}

fn face_color(p: [f64; 3], axis: usize, markers: &[Vec<FaceMarker>]) -> [u8; 3] {
    let positive = p[axis] > 0.0;
    let (u, v) = match axis {
        0 => (p[1], p[2]),
        1 => (p[0], p[2]),
        _ => (p[0], p[1]),
    };
    let idx = face_index(axis, positive);
    for marker in &markers[idx] {
        if ((u - marker.u).powi(2) + (v - marker.v).powi(2)).sqrt() <= marker.radius {
            return marker.color;
        }
    }
    let shade = texture_shade(idx, u, v);
    FACE_BASE_COLORS[idx].map(|channel| (f64::from(channel) * shade).round().clamp(0.0, 255.0) as u8)
}

/// 🖼️ Re-derives one committed view from scratch, pixel for pixel.
fn render_reference_frame(index: usize) -> remodeling_image::ImageRgba8 {
    render_from(camera_eyes()[index])
}

/// 🖼️ The scene rendered from an arbitrary eye looking at the cube's centre.
fn render_from(eye: [f64; 3]) -> remodeling_image::ImageRgba8 {
    let markers = generate_markers();
    let r_cw = camera_to_world_rotation(eye);
    let mut image = remodeling_image::ImageRgba8::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = normalize3(mat_act(&r_cw, unproject_ray(f64::from(x) + 0.5, f64::from(y) + 0.5)));
            let color = ray_box_intersect(eye, ray).map_or(BACKGROUND, |(p, axis)| face_color(p, axis, &markers));
            let idx = ((y * WIDTH + x) * 4) as usize;
            image.data[idx] = color[0];
            image.data[idx + 1] = color[1];
            image.data[idx + 2] = color[2];
            image.data[idx + 3] = 255;
        }
    }
    image
}
//#endregion 🎥️ReferenceRenderer

//#region 🔮️GroundTruth
struct GroundTruth {
    fx: f64,
    fy: f64,
    cx: f64,
    cy: f64,
    distortion: [f64; 5],
    centers: Vec<[f64; 3]>,
    rotations_camera_to_world: Vec<Quatd>,
    points: Vec<[f64; 3]>,
}

fn ground_truth() -> GroundTruth {
    let value: serde_json::Value = serde_json::from_str(GROUND_TRUTH_JSON).expect("ground truth json");
    let number = |node: &serde_json::Value| node.as_f64().expect("ground truth number");
    let triple = |node: &serde_json::Value| {
        let array = node.as_array().expect("ground truth triple");
        [number(&array[0]), number(&array[1]), number(&array[2])]
    };
    let intrinsics = &value["intrinsics"];
    let distortion_node = intrinsics["distortion"].as_array().expect("distortion array");
    let mut distortion = [0.0; 5];
    for (slot, node) in distortion.iter_mut().zip(distortion_node) {
        *slot = number(node);
    }
    let extrinsics = value["extrinsics"].as_array().expect("extrinsics array");
    GroundTruth {
        fx: number(&intrinsics["fx"]),
        fy: number(&intrinsics["fy"]),
        cx: number(&intrinsics["cx"]),
        cy: number(&intrinsics["cy"]),
        distortion,
        centers: extrinsics.iter().map(|node| triple(&node["cameraCenterM"])).collect(),
        rotations_camera_to_world: extrinsics
            .iter()
            .map(|node| {
                let q = node["rotationWxyzCameraToWorld"].as_array().expect("camera-to-world quaternion");
                Quatd { w: number(&q[0]), x: number(&q[1]), y: number(&q[2]), z: number(&q[3]) }
            })
            .collect(),
        points: value["pointsWorldM"].as_array().expect("points array").iter().map(triple).collect(),
    }
}

/// 📏️ Scene scale used to normalise translation error: the widest separation between two truth cameras.
fn scene_scale(centers: &[[f64; 3]]) -> f64 {
    let mut scale: f64 = 0.0;
    for (i, a) in centers.iter().enumerate() {
        for b in &centers[i + 1..] {
            scale = scale.max(norm3(sub3(*a, *b)));
        }
    }
    scale
}
//#endregion 🔮️GroundTruth

//#region 📌️Alignment
/// 📌️ Greedy injective nearest-truth assignment for already-aligned recovered centres.
fn nearest_assignment(recovered: &[[f64; 3]], truth: &[[f64; 3]]) -> Vec<usize> {
    let mut taken = vec![false; truth.len()];
    recovered
        .iter()
        .map(|point| {
            let mut best = (f64::INFINITY, 0usize);
            for (index, candidate) in truth.iter().enumerate() {
                let distance = norm3(sub3(*point, *candidate));
                if !taken[index] && distance < best.0 {
                    best = (distance, index);
                }
            }
            taken[best.1] = true;
            best.1
        })
        .collect()
}

fn residual(sim: &Sim3, recovered: &[[f64; 3]], truth: &[[f64; 3]], assignment: &[usize]) -> f64 {
    recovered.iter().zip(assignment).map(|(point, index)| norm3(sub3(sim.act(*point), truth[*index])).powi(2)).sum::<f64>()
}

/// 📌️ Sim(3) registration of the recovered camera centres onto the truth orbit WITHOUT a known
/// correspondence: `terminal_sparse_chunk` exposes poses in registration order only (see
/// `camera_pose_preview`'s own docstring), so the correspondence is searched. Every ordered triple of
/// truth cameras seeds a three-point Umeyama fit; each seed is then refined by two nearest-assignment
/// rounds, and the lowest-residual result wins.
fn align_to_truth(recovered: &[[f64; 3]], truth: &[[f64; 3]]) -> Option<(Sim3, Vec<usize>)> {
    if recovered.len() < 3 || truth.len() < 3 || recovered.len() > truth.len() {
        return None;
    }
    let seed = [recovered[0], recovered[1], recovered[2]];
    let mut best: Option<(f64, Sim3, Vec<usize>)> = None;
    for a in 0..truth.len() {
        for b in 0..truth.len() {
            for c in 0..truth.len() {
                if a == b || b == c || a == c {
                    continue;
                }
                let Some(mut sim) = umeyama(&seed, &[truth[a], truth[b], truth[c]], true) else { continue };
                let mut assignment = Vec::new();
                for _ in 0..2 {
                    let moved: Vec<[f64; 3]> = recovered.iter().map(|point| sim.act(*point)).collect();
                    assignment = nearest_assignment(&moved, truth);
                    let targets: Vec<[f64; 3]> = assignment.iter().map(|index| truth[*index]).collect();
                    let Some(refined) = umeyama(recovered, &targets, true) else { break };
                    sim = refined;
                }
                if assignment.is_empty() {
                    continue;
                }
                let error = residual(&sim, recovered, truth, &assignment);
                if best.as_ref().is_none_or(|(previous, _, _)| error < *previous) {
                    best = Some((error, sim, assignment));
                }
            }
        }
    }
    best.map(|(_, sim, assignment)| (sim, assignment))
}

/// 🎯️ `(max rotation error in degrees, translation RMSE as a fraction of scene scale, per-pose
/// ledger)` — the ledger lists, per recovered pose in registration order, the truth camera it was
/// assigned to, its rotation error in degrees and its centre error as a fraction of scene scale,
/// so a failing bound names the camera that broke it.
fn pose_errors(poses: &[(Quatd, [f64; 3])], truth: &GroundTruth) -> (f64, f64, Vec<(usize, f64, f64)>) {
    let recovered: Vec<[f64; 3]> = poses.iter().map(|(_, center)| *center).collect();
    let (sim, assignment) = align_to_truth(&recovered, &truth.centers).expect("recovered cameras must admit a Sim(3) registration onto the truth orbit");
    let scale = scene_scale(&truth.centers);
    let mut max_rotation = 0.0f64;
    let mut squared = 0.0f64;
    let mut ledger = Vec::with_capacity(poses.len());
    for ((quat, center), index) in poses.iter().zip(&assignment) {
        let aligned = sim.r.semio_compose_rs(&So3::from_quat(*quat));
        let expected = So3::from_quat(truth.rotations_camera_to_world[*index]);
        let rotation = norm3(expected.inverse().semio_compose_rs(&aligned).log()).to_degrees();
        let centre = norm3(sub3(sim.act(*center), truth.centers[*index]));
        max_rotation = max_rotation.max(rotation);
        squared += centre.powi(2);
        ledger.push((*index, rotation, centre / scale));
    }
    (max_rotation, (squared / poses.len() as f64).sqrt() / scale, ledger)
}

/// 📐️ Aligned mesh vertices scored against the truth cube: `(median, p90)` of every vertex's
/// distance to the cube's surface, in cube half-sizes, and the share of truth surface points that
/// have a vertex within [`MESH_COMPLETENESS_RADIUS`] — accuracy and completeness, so neither a
/// shell fused out of the background nor a patch of one face passes.
fn mesh_surface_errors(vertices: &[[f64; 3]], truth_points: &[[f64; 3]]) -> (f64, f64, f64) {
    let surface = |p: &[f64; 3]| {
        let outside = norm3([(p[0].abs() - CUBE_HALF).max(0.0), (p[1].abs() - CUBE_HALF).max(0.0), (p[2].abs() - CUBE_HALF).max(0.0)]);
        let inside = (CUBE_HALF - p[0].abs()).min(CUBE_HALF - p[1].abs()).min(CUBE_HALF - p[2].abs()).max(0.0);
        (outside + inside) / CUBE_HALF
    };
    let mut distances: Vec<f64> = vertices.iter().map(surface).collect();
    distances.sort_by(f64::total_cmp);
    let quantile = |q: f64| distances.get(((distances.len() as f64 - 1.0) * q).round() as usize).copied().unwrap_or(f64::INFINITY);
    let covered = truth_points.iter().filter(|point| vertices.iter().any(|vertex| norm3(sub3(*vertex, **point)) <= MESH_COMPLETENESS_RADIUS)).count();
    (quantile(0.5), quantile(0.9), covered as f64 / truth_points.len().max(1) as f64)
}
//#endregion 📌️Alignment

//#region 🚚️Driver
fn frame_payload(bytes: &[u8]) -> String {
    format!("data:{FRAME_MIME};base64,{}", base64_codec::base64_standard_encode(bytes))
}

/// 📥️ Feeds every committed frame through the real still-image import command — the same handler a
/// file-picker drop reaches — so the document under test is built by the product, not by the test.
/// The rendering camera's calibration and the fixture's engine tuning (the same values
/// [`fixture_document`] commits) arrive the same way, through the calibration and parameter-group
/// commands: the boot document is the uncalibrated `demo` (ORB, 5 mm voxels, textured), under which
/// this scene has no chance, and a user who reconstructs a calibrated capture edits exactly these.
async fn imported_app() -> RemodelingApp {
    use crate::editor::remodeling::commands::{edit_calibration::EditCalibration, set_dense_params::SetDenseParams, set_feature_params::SetFeatureParams, set_ingest_params::SetIngestParams, set_match_params::SetMatchParams, set_mesh_params::SetMeshParams, set_sfm_params::SetSfmParams};
    let mut app = app_with_registry().await;
    let fixture = fixture_document();
    let camera = &fixture.calibration.cameras[0];
    let calibration = EditCalibration {
        camera_id: camera.id.clone(),
        label: camera.label.clone(),
        model: camera.model.clone(),
        fx: camera.fx,
        fy: camera.fy,
        cx: camera.cx,
        cy: camera.cy,
        skew: camera.skew,
        k1: camera.distortion[0],
        k2: camera.distortion[1],
        k3: camera.distortion[2],
        p1: camera.distortion[3],
        p2: camera.distortion[4],
        locked: camera.locked,
    };
    dispatch(&mut app, RemodelingCommand::EditCalibration(calibration)).await;
    settle(&mut app, "the camera calibration").await;
    for (index, (_, bytes)) in FRAMES.iter().enumerate() {
        let payload = ImportFramePayload { payload: frame_payload(*bytes), name: format!("🎞️frame-{index:02}.png"), index: index as u32 };
        dispatch(&mut app, RemodelingCommand::ImportFramePayload(payload)).await;
        settle(&mut app, "the frame import").await;
    }
    let params = &fixture.params;
    let tuning = [
        RemodelingCommand::SetIngestParams(SetIngestParams { frame_sample_stride: params.ingest.frame_sample_stride, max_frames: params.ingest.max_frames, downscale_long_edge_px: params.ingest.downscale_long_edge_px, min_sharpness: params.ingest.min_sharpness }),
        RemodelingCommand::SetFeatureParams(SetFeatureParams { detector: "akaze".into(), target_count: params.feature.target_count, octaves: params.feature.octaves, edge_threshold: params.feature.edge_threshold }),
        RemodelingCommand::SetMatchParams(SetMatchParams { matcher: "brute-force".into(), ratio_test: params.matching.ratio_test, cross_check: params.matching.cross_check, sequential_window: params.matching.sequential_window, max_pairs_per_frame: params.matching.max_pairs_per_frame, loop_closure: params.matching.loop_closure }),
        RemodelingCommand::SetSfmParams(SetSfmParams { ransac_iterations: params.sfm.ransac_iterations, ransac_threshold_px: params.sfm.ransac_threshold_px, min_track_length: params.sfm.min_track_length, ba_max_iterations: params.sfm.ba_max_iterations, robust_loss: "huber".into(), huber_delta_px: params.sfm.huber_delta_px }),
        RemodelingCommand::SetDenseParams(SetDenseParams { resolution: "low".into(), window_radius_px: params.dense.window_radius_px, min_view_consistency: params.dense.min_view_consistency, confidence_threshold: params.dense.confidence_threshold, max_points: params.dense.max_points }),
        RemodelingCommand::SetMeshParams(SetMeshParams {
            tsdf_voxel_size_mm: params.mesh.tsdf_voxel_size_mm,
            tsdf_truncation_mm: params.mesh.tsdf_truncation_mm,
            decimate_target_triangles: params.mesh.decimate_target_triangles,
            smoothing_iterations: params.mesh.smoothing_iterations,
            texture_enabled: params.mesh.texture_enabled,
            texture_size: params.mesh.texture_size,
            guarantee_watertight: params.mesh.guarantee_watertight,
            hole_fill_max_boundary_verts: params.mesh.hole_fill_max_boundary_verts,
            self_intersection_check: params.mesh.self_intersection_check,
        }),
    ];
    for command in tuning {
        dispatch(&mut app, command).await;
        settle(&mut app, "the reconstruction parameters").await;
    }
    let tuned = app.snapshot().expect("tuned snapshot");
    assert_eq!(tuned.params, fixture.params, "the parameter commands must land the fixture's tuning");
    assert_eq!(tuned.calibration.cameras.len(), 1, "the calibration command must land the rendering camera");
    app
}

/// 🏁️ One reconstruction tool run through the framework actions: start, pump to `complete`, finalize, and
/// pump to `finalized`. Answers the finalized document and every stage the run reported, in order.
async fn finalize_reconstruction(app: &mut RemodelingApp) -> (RemodelingSnapshot, Vec<u16>) {
    start_reconstruction(app).await;
    let mut stages = Vec::new();
    let complete = loop {
        let run = pump_run(app, "reconstruction progresses", |_| true).await;
        if stages.last() != Some(&run.stage) {
            stages.push(run.stage);
        }
        match run.state.wire_name() {
            "complete" => break run,
            "faulted" | "aborted" => panic!("the reconstruction run ended {:?}", run),
            _ => host_turn(app).await,
        }
    };
    assert_eq!(complete.state.wire_name(), "complete");
    let arguments = run_arguments(app).await;
    let output = run_action(app, semio_framework_plugin::TOOL_RUN_FINALIZE_ACTION_ID, arguments).await;
    assert_eq!(output.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("beginFinalize"));
    pump_run(app, "finalize settles", |run| run.state.wire_name() == "finalized").await;
    settle(app, "the finalize publication").await;
    (app.snapshot().expect("finalized snapshot"), stages)
}
//#endregion 🚚️Driver

//#region 🧪️FixtureIntegrity
#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    assert!(PRIMARY_TEXT.len() > 8);
    assert_eq!(ID, "synthetic-orbit");
    assert_eq!(FRAMES.len(), FRAME_COUNT);
}

#[semio_framework_async_macros::async_test]
async fn dsl_declares_the_ground_truth_camera_and_the_full_frame_table() {
    let truth = ground_truth();
    let scene = <RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("synthetic-orbit document parses");
    let camera = scene.calibration.cameras.iter().find(|camera| camera.id == CAMERA_ID).expect("calibrated camera");
    assert_eq!(camera.model, "brownConrady");
    assert!((camera.fx - truth.fx).abs() < 1e-9 && (camera.fy - truth.fy).abs() < 1e-9, "document focal must equal the rendering camera's");
    assert!((camera.cx - truth.cx).abs() < 1e-9 && (camera.cy - truth.cy).abs() < 1e-9);
    for (slot, expected) in camera.distortion.iter().zip(truth.distortion) {
        assert!((f64::from(*slot) - expected).abs() < 1e-6, "document distortion must equal the rendering camera's");
    }
    let stream = scene.streams.iter().find(|stream| stream.id == STREAM_ID).expect("declared stream");
    assert_eq!(stream.kind, MediaKind::ImageSequence);
    assert_eq!(stream.camera_id.as_deref(), Some(CAMERA_ID));
    assert_eq!(stream.frames.len(), FRAMES.len(), "the DSL frame table must list every committed frame");
    for (frame, (asset_id, _)) in stream.frames.iter().zip(FRAMES) {
        assert_eq!(frame.asset_id, asset_id);
        assert!((frame.timestamp_ms - f64::from(frame.index) * 1000.0 / FPS_HINT).abs() < 1e-6);
    }
    assert_eq!(truth.centers.len(), FRAMES.len());
    assert_eq!(truth.rotations_camera_to_world.len(), FRAMES.len());
    assert!(truth.points.len() >= 100, "ground truth must carry a real point set, got {}", truth.points.len());
}

#[semio_framework_async_macros::async_test]
async fn committed_frames_match_the_png_oracle_and_the_reference_renderer() {
    for (index, (_, bytes)) in FRAMES.iter().enumerate() {
        let ours = remodeling_image::decode_png(*bytes).expect("plugin PNG decode");
        assert_eq!((ours.width, ours.height), (WIDTH, HEIGHT));

        let mut reader = png::Decoder::new(*bytes).read_info().expect("oracle PNG header");
        let mut oracle = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut oracle).expect("oracle PNG decode");
        assert_eq!((info.width, info.height), (WIDTH, HEIGHT));
        assert_eq!(info.color_type, png::ColorType::Rgba);
        oracle.truncate(info.buffer_size());
        assert_eq!(ours.data, oracle, "frame {index}: plugin decoder must agree with the png oracle");

        assert_eq!(ours.data, render_reference_frame(index).data, "frame {index}: committed pixels must equal the reference renderer's");
    }
}
//#endregion 🧪️FixtureIntegrity

//#region 🧪️EndToEnd
#[semio_framework_async_macros::async_test]
async fn reconstructs_the_synthetic_orbit_against_ground_truth() {
    let truth = ground_truth();
    let mut app = imported_app().await;
    let imported = app.snapshot().expect("imported snapshot");
    assert_eq!(imported.streams.iter().map(|stream| stream.frames.len()).sum::<usize>(), FRAMES.len(), "every committed frame must reach the document");

    let (scene, stages) = finalize_reconstruction(&mut app).await;
    assert!(stages.windows(2).all(|pair| pair[1] >= pair[0]), "reported stages must never move backwards: {stages:?}");
    assert!(stages.len() >= 3, "a populated document runs through the pipeline stages, got {stages:?}");

    let sparse = scene.results.sparse.as_ref().expect("commit-reconstruction must carry a sparse cloud");
    let recovered_points = sparse.points.to_f32_vec_from(&scene.durable_artifacts).len() / 3;
    let required = (truth.points.len() as f64 * MIN_SPARSE_POINTS_PER_TRUTH_POINT).ceil() as usize;
    assert!(recovered_points >= required, "sparse cloud has {recovered_points} points, expected at least {required} for {} truth points", truth.points.len());

    let trajectory = scene.results.trajectory.as_ref().expect("commit-reconstruction must carry a trajectory");
    assert!(trajectory.poses.len() >= 3, "need at least three registered cameras to score a gauge, got {}", trajectory.poses.len());
    let poses: Vec<(Quatd, [f64; 3])> = trajectory
        .poses
        .iter()
        .map(|pose| {
            let q = pose.rotation_wxyz;
            (Quatd { w: f64::from(q[0]), x: f64::from(q[1]), y: f64::from(q[2]), z: f64::from(q[3]) }, [f64::from(pose.translation[0]), f64::from(pose.translation[1]), f64::from(pose.translation[2])])
        })
        .collect();
    let (rotation_deg, translation_ratio, ledger) = pose_errors(&poses, &truth);
    assert!(rotation_deg < MAX_ROTATION_ERROR_DEG * UNCALIBRATED_GAUGE_SLACK, "aligned rotation error {rotation_deg:.3}° exceeds the gauge-slackened bound; per pose (truth camera, rotation°, centre error/scale): {ledger:?}");
    assert!(translation_ratio < MAX_TRANSLATION_RMSE_RATIO * UNCALIBRATED_GAUGE_SLACK, "aligned translation RMSE {:.3}% of scene scale exceeds the gauge-slackened bound; per pose (truth camera, rotation°, centre error/scale): {ledger:?}", translation_ratio * 100.0);

    let mesh = &scene.results.mesh;
    assert_ne!(mesh.source, crate::MeshSource::Placeholder, "a completed run must replace the seeded placeholder mesh");
    let recovered: Vec<[f64; 3]> = poses.iter().map(|(_, center)| *center).collect();
    let (sim, _) = align_to_truth(&recovered, &truth.centers).expect("recovered cameras must admit a Sim(3) registration onto the truth orbit");
    let data = crate::resolve_bounded_remodeling_mesh(&scene.durable_artifacts, &mesh.mesh).expect("the finalized mesh resolves");
    let vertices: Vec<[f64; 3]> = data.positions.chunks_exact(3).map(|p| sim.act([f64::from(p[0]), f64::from(p[1]), f64::from(p[2])])).collect();
    let (median, p90, completeness) = mesh_surface_errors(&vertices, &truth.points);
    println!("[mesh-truth] {} vertices: surface distance median {median:.3} / p90 {p90:.3} (cube half-sizes), truth points within {MESH_COMPLETENESS_RADIUS} of a vertex {:.1}%", vertices.len(), completeness * 100.0);
    assert!(median < MAX_MESH_MEDIAN_SURFACE_ERROR && p90 < MAX_MESH_P90_SURFACE_ERROR, "the mesh must lie on the cube: median {median:.3}, p90 {p90:.3} half-sizes off its surface");
    assert!(completeness >= MIN_MESH_COMPLETENESS, "the mesh must cover the cube: only {:.1}% of the truth points have a vertex within {MESH_COMPLETENESS_RADIUS}", completeness * 100.0);
    close(app);
}

#[semio_framework_async_macros::async_test]
async fn aborting_the_synthetic_orbit_reconstruction_mid_run_leaves_the_document_byte_identical() {
    let mut app = imported_app().await;
    let before = durable(&mut app).await;
    start_reconstruction(&mut app).await;
    pump_run(&mut app, "the run reaches feature extraction", |run| run.stage >= 1).await;
    let arguments = run_arguments(&mut app).await;
    let output = run_action(&mut app, semio_framework_plugin::TOOL_RUN_ABORT_ACTION_ID, arguments).await;
    assert_eq!(output.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("closeJob"));
    pump_run(&mut app, "abort settles", |run| run.state.wire_name() == "aborted").await;
    let after = durable(&mut app).await;
    assert!(before.0 == after.0 && before.1 == after.1, "an aborted reconstruction leaves the document pack byte-identical");
    assert_eq!(before.2, after.2, "an aborted reconstruction leaves no history trace");
    let scene = app.snapshot().expect("aborted snapshot");
    assert!(scene.results.sparse.is_none(), "an aborted run commits no sparse cloud");
    assert_eq!(scene.results.mesh.source, crate::MeshSource::Placeholder, "an aborted run leaves the seeded placeholder mesh in place");
    close(app);
}
#[semio_framework_async_macros::async_test]
async fn a_finalized_reconstruction_is_one_undoable_edit() {
    let mut app = imported_app().await;
    let before = durable(&mut app).await;
    let before_scene = app.snapshot().expect("imported snapshot");
    start_reconstruction(&mut app).await;
    let complete = pump_run(&mut app, "the run rests", |run| matches!(run.state.wire_name(), "complete" | "faulted")).await;
    assert_eq!(complete.state.wire_name(), "complete", "the calibrated orbit completes: {complete:?}");
    let unfinalized = durable(&mut app).await;
    assert!(unfinalized.0 == before.0 && unfinalized.1 == before.1, "a complete but unfinalized run leaves the document pack byte-identical");
    let arguments = run_arguments(&mut app).await;
    let output = run_action(&mut app, semio_framework_plugin::TOOL_RUN_FINALIZE_ACTION_ID, arguments).await;
    assert_eq!(output.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("beginFinalize"));
    pump_run(&mut app, "finalize settles", |run| run.state.wire_name() == "finalized").await;
    settle(&mut app, "the finalize publication").await;
    let finalized = durable(&mut app).await;
    let finalized_scene = app.snapshot().expect("finalized snapshot");
    assert!(finalized.1 != before.1 && finalized_scene != before_scene, "finalize publishes the reconstruction");
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app, "undo", semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
    assert_eq!(app.snapshot().expect("undone snapshot"), before_scene, "one undo removes the whole finalized reconstruction");
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app, "redo", semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
    assert_eq!(app.snapshot().expect("redone snapshot"), finalized_scene, "one redo restores it");
    close(app);
}

/// 🌱️ A capture no two adjacent frames of can be solved (the boot document's stride-5 sampling
/// leaves eight views 50° apart) completes without cameras rather than faulting; finalize then has
/// nothing to publish and leaves the document byte-identical.
#[semio_framework_async_macros::async_test]
async fn an_unregistrable_capture_completes_without_cameras() {
    let mut app = app_with_registry().await;
    for (index, (_, bytes)) in FRAMES.iter().enumerate() {
        dispatch(&mut app, RemodelingCommand::ImportFramePayload(ImportFramePayload { payload: frame_payload(*bytes), name: format!("🎞️frame-{index:02}.png"), index: index as u32 })).await;
        settle(&mut app, "the frame import").await;
    }
    let before = durable(&mut app).await;
    start_reconstruction(&mut app).await;
    let complete = pump_run(&mut app, "the run rests", |run| matches!(run.state.wire_name(), "complete" | "faulted")).await;
    assert_eq!(complete.state.wire_name(), "complete", "an unregistrable sample completes: {complete:?}");
    let arguments = run_arguments(&mut app).await;
    run_action(&mut app, semio_framework_plugin::TOOL_RUN_FINALIZE_ACTION_ID, arguments).await;
    pump_run(&mut app, "finalize settles", |run| run.state.wire_name() == "finalized").await;
    settle(&mut app, "the finalize publication").await;
    let finalized = durable(&mut app).await;
    let published = app.snapshot().expect("finalized snapshot");
    assert!(
        finalized.0 == before.0 && finalized.1 == before.1,
        "nothing to publish leaves the document pack byte-identical; published {} cameras, sparse {}, mesh {:?}",
        published.results.trajectory.as_ref().map_or(0, |trajectory| trajectory.poses.len()),
        published.results.sparse.is_some(),
        published.results.mesh.source
    );
    close(app);
}
//#endregion 🧪️EndToEnd

//#region 🔭️Diagnostics
/// 🔭️ How the bounded ORB features survive one orbit step of the given size: two views of the scene
/// (at angle 0 and at `step`) are detected, described (steered and upright) and brute-force matched
/// exactly as the engine does, then scored against the true transfer through the known cube. Prints,
/// per step, the true-pair Hamming median against the best impostor and the share of accepted matches
/// that land within 2 px of the truth. Diagnostic, therefore `#[ignore]`:
/// `cargo test -p semio-s-artifact-remodel-remodeling --lib -- --ignored --nocapture diagnose_orb_matching_vs_orbit_step`.
#[test]
#[ignore = "diagnostic feature audit over synthetic orbit steps, run explicitly"]
fn diagnose_orb_matching_vs_orbit_step() {
    use crate::editor::remodeling::engine::feature as remodeling_feature;
    let eye_at = |angle: f64| [ORBIT_RADIUS * angle.cos(), ORBIT_ELEVATION, ORBIT_RADIUS * angle.sin()];
    let luma = |image: &remodeling_image::ImageRgba8| remodeling_image::ImageGray::from_rgba8_luma(image);
    let detect = |gray: remodeling_image::ImageGray, upright: bool| {
        let mut preparation = remodeling_feature::BoundedDetectionPreparation::new(gray, 3, 600, remodeling_feature::BoundedDetector::Orb, upright);
        while !preparation.advance(16) {}
        let (pyramid, keypoints) = preparation.finish();
        let descriptors = remodeling_feature::describe_orb(&pyramid, &keypoints);
        (keypoints, descriptors)
    };
    let transfer = |eye_a: [f64; 3], eye_b: [f64; 3], px: [f64; 2]| -> Option<[f64; 2]> {
        let r_a = camera_to_world_rotation(eye_a);
        let ray = normalize3(mat_act(&r_a, unproject_ray(px[0], px[1])));
        let (point, _) = ray_box_intersect(eye_a, ray)?;
        let r_b = camera_to_world_rotation(eye_b);
        let r_wc = [[r_b[0][0], r_b[1][0], r_b[2][0]], [r_b[0][1], r_b[1][1], r_b[2][1]], [r_b[0][2], r_b[1][2], r_b[2][2]]];
        let camera = mat_act(&r_wc, sub3(point, eye_b));
        if camera[2] <= 1e-9 {
            return None;
        }
        let normalized = distort([camera[0] / camera[2], camera[1] / camera[2]]);
        Some([CX + FX * normalized[0], CY + FX * normalized[1]])
    };
    for step_deg in [5.0f64, 10.0, 15.0, 18.0, 36.0] {
        let (eye_a, eye_b) = (eye_at(0.0), eye_at(step_deg.to_radians()));
        let (image_a, image_b) = (render_from(eye_a), render_from(eye_b));
        if step_deg == 5.0 {
            eprintln!("[ORB] a rendered view encodes to {} PNG bytes", remodeling_image::encode_png(&image_a).map_or(0, |bytes| bytes.len()));
        }
        for upright in [false, true] {
            let (keypoints_a, descriptors_a) = detect(luma(&image_a), upright);
            let (keypoints_b, descriptors_b) = detect(luma(&image_b), upright);
            let scale = |keypoint: &remodeling_feature::Keypoint| f64::from(1u32 << keypoint.octave);
            let pixel = |keypoint: &remodeling_feature::Keypoint| [f64::from(keypoint.x) * scale(keypoint), f64::from(keypoint.y) * scale(keypoint)];
            let matches = remodeling_feature::match_brute(&descriptors_a, &descriptors_b, 0.85, true);
            let mut correct = 0usize;
            for matched in &matches {
                let Some(expected) = transfer(eye_a, eye_b, pixel(&keypoints_a[matched.a as usize])) else { continue };
                let got = pixel(&keypoints_b[matched.b as usize]);
                if ((got[0] - expected[0]).powi(2) + (got[1] - expected[1]).powi(2)).sqrt() < 2.0 {
                    correct += 1;
                }
            }
            let mut true_pair = Vec::new();
            let mut impostor = Vec::new();
            for (index, keypoint) in keypoints_a.iter().enumerate() {
                let Some(expected) = transfer(eye_a, eye_b, pixel(keypoint)) else { continue };
                let Some(partner) = keypoints_b.iter().enumerate().filter(|(_, k)| k.octave == keypoint.octave).map(|(j, k)| (j, { let g = pixel(k); ((g[0] - expected[0]).powi(2) + (g[1] - expected[1]).powi(2)).sqrt() })).filter(|(_, d)| *d < 1.5).min_by(|x, y| x.1.total_cmp(&y.1)).map(|(j, _)| j) else { continue };
                true_pair.push(remodeling_feature::hamming(&descriptors_a[index], &descriptors_b[partner]));
                impostor.push(descriptors_b.iter().enumerate().filter(|(j, _)| *j != partner).map(|(_, d)| remodeling_feature::hamming(&descriptors_a[index], d)).min().unwrap_or(u32::MAX));
            }
            true_pair.sort_unstable();
            impostor.sort_unstable();
            let mid = |v: &[u32]| v.get(v.len() / 2).copied().unwrap_or(0);
            let quarter = |v: &[u32]| v.get(v.len() / 4).copied().unwrap_or(0);
            eprintln!("[ORB] step {step_deg:>4}° upright={upright}: {} / {} keypoints, {} true pairs (Hamming q25 {} median {}; impostor median {}), {} matches, {correct} within 2 px", keypoints_a.len(), keypoints_b.len(), true_pair.len(), quarter(&true_pair), mid(&true_pair), mid(&impostor), matches.len());
        }
    }
}
//#endregion 🔭️Diagnostics

//#region 🛠️Regeneration
/// 🎞️ Asset id of the `index`-th view, the same naming `FRAMES` commits.
fn frame_asset_id(index: usize) -> String {
    format!("synthetic-orbit-frame-{index}")
}

/// 🛠️ Rewrites `🖼️assets/` from the constants above — `bun ./📜️script.ts regenerate-example`, or
/// `cargo test -p semio-s-plugin-remodel --lib synthetic_orbit -- --ignored --exact
/// artifacts::remodeling::…::regenerates_the_synthetic_orbit_example`. Deterministic: running it twice
/// leaves the tree byte-identical.
#[test]
#[ignore = "regenerates committed fixture assets; run explicitly"]
fn regenerates_the_synthetic_orbit_example() {
    let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🖼️assets");
    std::fs::create_dir_all(&assets).expect("example asset directory");

    let markers = generate_markers();
    let points = reference_points(&markers);
    let eyes = camera_eyes();
    let mut frames = Vec::new();
    let mut extrinsics = Vec::new();
    for (index, eye) in eyes.iter().enumerate() {
        let bytes = remodeling_image::encode_png(&render_reference_frame(index)).expect("encode fixture frame");
        std::fs::write(assets.join(format!("🎞️frame-{index:02}.png")), &bytes).expect("write fixture frame");
        let r_cw = camera_to_world_rotation(*eye);
        let rotation_cw = So3(crate::algebra::Mat3d::from_axes([r_cw[0][0], r_cw[1][0], r_cw[2][0]], [r_cw[0][1], r_cw[1][1], r_cw[2][1]], [r_cw[0][2], r_cw[1][2], r_cw[2][2]]));
        let rotation_wc = rotation_cw.inverse();
        let translation = rotation_wc.act(*eye);
        let (q_wc, q_cw) = (rotation_wc.to_quat(), rotation_cw.to_quat());
        frames.push(serde_json::json!({ "index": index, "timestampMs": index as f64 * 1000.0 / FPS_HINT, "assetId": frame_asset_id(index), "file": format!("🎞️frame-{index:02}.png") }));
        extrinsics.push(serde_json::json!({
            "frameIndex": index,
            "cameraCenterM": eye,
            "rotationWxyzWorldToCamera": [q_wc.w, q_wc.x, q_wc.y, q_wc.z],
            "rotationWxyzCameraToWorld": [q_cw.w, q_cw.x, q_cw.y, q_cw.z],
            "translationWorldToCameraM": [-translation[0], -translation[1], -translation[2]],
        }));
    }

    let truth = serde_json::json!({
        "schema": "semio.remodeling.synthetic-orbit-ground-truth/1",
        "seed": SEED,
        "image": { "width": WIDTH, "height": HEIGHT },
        "intrinsics": { "fx": FX, "fy": FX, "cx": CX, "cy": CY, "skew": 0.0, "model": "brownConrady", "distortion": [K1, K2, 0.0, 0.0, 0.0] },
        "scene": { "kind": "textured-cube", "halfExtentM": CUBE_HALF, "orbitRadiusM": ORBIT_RADIUS, "orbitElevationM": ORBIT_ELEVATION },
        "streamId": STREAM_ID,
        "cameraId": CAMERA_ID,
        "fpsHint": FPS_HINT,
        "frames": frames,
        "extrinsics": extrinsics,
        "pointsWorldM": points,
    });
    std::fs::write(assets.join("🔮️ground-truth.json"), format!("{}\n", serde_json::to_string_pretty(&truth).expect("ground truth json"))).expect("write ground truth");
    std::fs::write(assets.join("🗣️.dsl.semio"), <RemodelingSnapshot as store::ArtifactDsl>::print_dsl(&fixture_document())).expect("write fixture document");
}

/// 🗣️ The committed document: the engine-tuning params this fixture reconstructs under, the rendering
/// camera's true calibration, and the stream whose frame table the loader binds pixels to.
fn fixture_document() -> RemodelingSnapshot {
    let mut scene = crate::default_remodeling_scene();
    scene.calibration.cameras = vec![CameraCalibration {
        id: CAMERA_ID.into(),
        label: "Synthetic Orbit Camera".into(),
        model: "brownConrady".into(),
        fx: FX,
        fy: FX,
        cx: CX,
        cy: CY,
        skew: 0.0,
        distortion: [K1 as f32, K2 as f32, 0.0, 0.0, 0.0],
        rms_reprojection_px: Some(0.4),
        locked: true,
    }];
    scene.params.ingest.frame_sample_stride = 1;
    scene.params.ingest.max_frames = 64;
    scene.params.ingest.downscale_long_edge_px = WIDTH;
    scene.params.ingest.min_sharpness = 0.0;
    scene.params.feature.detector = crate::FeatureDetector::Akaze;
    scene.params.feature.target_count = 600;
    scene.params.matching.ratio_test = 0.85;
    scene.params.matching.sequential_window = 3;
    scene.params.sfm.min_track_length = 2;
    scene.params.sfm.ba_max_iterations = 25;
    scene.params.dense.resolution = crate::DenseResolution::Low;
    scene.params.dense.window_radius_px = 2;
    scene.params.dense.max_points = 50_000;
    scene.params.mesh.tsdf_voxel_size_mm = 100.0;
    scene.params.mesh.tsdf_truncation_mm = 330.0;
    scene.params.mesh.decimate_target_triangles = 20_000;
    scene.params.mesh.smoothing_iterations = 1;
    scene.params.mesh.texture_enabled = false;
    scene.params.mesh.texture_size = 256;
    scene.streams = vec![MediaStream {
        id: STREAM_ID.into(),
        name: "Synthetic Orbit".into(),
        kind: MediaKind::ImageSequence,
        camera_id: Some(CAMERA_ID.into()),
        sync_offset_ms: 0.0,
        fps_hint: FPS_HINT,
        frames: (0..FRAME_COUNT).map(|index| FrameRef { index: index as u32, timestamp_ms: index as f64 * 1000.0 / FPS_HINT, asset_id: frame_asset_id(index) }).collect(),
        source: None,
    }];
    scene
}
//#endregion 🛠️Regeneration
