//! 🌫️ Dense reconstruction: PatchMatch multi-view stereo, depth fusion, TSDF volumes and point-cloud analysis.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).
use crate::editor::remodeling::engine::{camera as remodeling_camera, images as remodeling_image};

use std::collections::{BTreeMap, HashMap};

use crate::algebra::{jacobi_eigen_symmetric, MatD};
use crate::spatial::KdTree;

const MAX_INTERACTIVE_FUSED_POINTS: usize = 512;

// #region 🔖️PointCloud
/// 🏷️ Per-point semantic classification label produced by ground/planar segmentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointClass {
    Unclassified,
    Ground,
    Vegetation,
    Building,
    Noise,
}

/// ☁️ 3D point cloud as struct-of-arrays: `normals`/`colors`/`confidence`/`classification` are each
/// either empty (attribute unset) or exactly `positions.len()` entries, index-aligned with
/// `positions`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PointCloud {
    pub positions: Vec<[f64; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[u8; 3]>,
    pub confidence: Vec<f32>,
    pub classification: Vec<PointClass>,
}

/// 🔁️ Widens a stored `f32` normal to `f64` for arithmetic.
fn to_f64_3(n: [f32; 3]) -> [f64; 3] {
    [f64::from(n[0]), f64::from(n[1]), f64::from(n[2])]
}

impl PointCloud {
    /// ☁️ Empty point cloud with no optional attributes.
    pub fn new() -> Self {
        Self::default()
    }

    /// ☁️ Point cloud built from bare positions, with every optional attribute unset.
    pub fn from_positions(positions: Vec<[f64; 3]>) -> Self {
        Self { positions, ..Self::default() }
    }

    /// 🔢️ Number of points.
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// 🔢️ Whether the cloud holds no points.
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
}
// #endregion 🔖️PointCloud

// #region 🔖️DepthMap
/// 🌊️ Per-pixel depth, normal and confidence buffers for one reference view, row-major with pixel
/// `(x, y)` at `data[y * width + x]`.
///
/// Invalid-sentinel convention: a depth `<= 0.0` (including the zero-fill from [`DepthMap::new`])
/// or a non-finite value marks a pixel as having no valid estimate. [`DepthMap::get`] returns
/// `None` for such pixels as well as for out-of-bounds coordinates; `normal`/`confidence` at
/// invalid pixels carry no meaning and callers should gate on `get` first.
#[derive(Clone, Debug, PartialEq)]
pub struct DepthMap {
    pub width: u32,
    pub height: u32,
    pub depth: Vec<f32>,
    pub normal: Vec<[f32; 3]>,
    pub confidence: Vec<f32>,
}

impl DepthMap {
    /// 🌊️ Zero-filled depth map (every pixel invalid) of the given size.
    pub fn new(width: u32, height: u32) -> Self {
        let n = (width as usize) * (height as usize);
        Self { width, height, depth: vec![0.0; n], normal: vec![[0.0; 3]; n], confidence: vec![0.0; n] }
    }

    /// 🔍️ Valid depth at `(x, y)`, or `None` if out of bounds or the pixel carries the invalid
    /// sentinel (`<= 0` or non-finite).
    pub fn get(&self, x: u32, y: u32) -> Option<f32> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let d = self.depth[(y * self.width + x) as usize];
        (d > 0.0 && d.is_finite()).then_some(d)
    }
}

fn depthmap_index(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize
}

/// 🚫️ Resets one pixel to the invalid sentinel across all three buffers.
fn depthmap_invalidate(map: &mut DepthMap, idx: usize) {
    map.depth[idx] = 0.0;
    map.normal[idx] = [0.0; 3];
    map.confidence[idx] = 0.0;
}
// #endregion 🔖️DepthMap

// #region 🔖️PatchMatch
/// 🎲️ SplitMix64 seeded pseudo-random generator. This crate has no dependency on a shared RNG
/// crate, so a tiny, deterministic, allocation-free generator is hand-rolled here purely to drive
/// PatchMatch's random plane initialization and per-iteration perturbation.
/// <https://prng.di.unimi.it/splitmix64.c>
struct SplitMix64(u64);

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// 🎲️ Uniform value in `[0, 1)`.
    fn next_unit(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    /// 🎲️ Uniform value in `[lo, hi)`.
    fn next_range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.next_unit()
    }
}

/// 🌍️ A local plane hypothesis expressed in the reference camera's frame: a point the plane passes
/// through plus its (not necessarily unit) normal.
struct Plane {
    point: [f64; 3],
    normal: [f64; 3],
}

/// 📐️ Reference-view context threaded through the plane-induced warp helpers, bundled into one
/// struct so those helpers stay under clippy's argument-count ceiling.
struct RefContext<'a> {
    img: &'a remodeling_image::ImageGray,
    intr: &'a remodeling_camera::Intrinsics,
    to_world: &'a remodeling_camera::Se3,
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// 🌍️ The per-pixel plane induced by a depth/normal hypothesis at `center`: the plane passes
/// through the reference camera-frame point at that depth along the pixel's ray.
fn plane_from_depth_normal(intr: &remodeling_camera::Intrinsics, center: (u32, u32), depth: f32, normal: [f32; 3]) -> Plane {
    let center_ray = intr.unproject_ray([f64::from(center.0), f64::from(center.1)]);
    let d = f64::from(depth);
    let point = [center_ray[0] * d, center_ray[1] * d, center_ray[2] * d];
    let normal = [f64::from(normal[0]), f64::from(normal[1]), f64::from(normal[2])];
    Plane { point, normal }
}

/// 🔀️ Warps one reference-pixel offset into a source view via ray-plane intersection against
/// `plane`, in the reference camera frame. This is mathematically the same induced-homography warp
/// used by Gipuma-style PatchMatch MVS (`K_src (R_rel - t_rel nᵀ/d) K_ref⁻¹`), computed per-sample
/// via an explicit ray/plane intersection instead of a precomputed 3x3 matrix — simpler to semio_compose_rs
/// from `remodeling_camera`'s existing `project`/`unproject_ray`/`act` primitives, at the cost of
/// redoing the intersection for every neighbor pixel instead of amortizing it into one matrix.
fn warp_point_to_src(ref_ctx: &RefContext<'_>, src_pose: &remodeling_camera::CameraPose, src_intr: &remodeling_camera::Intrinsics, plane: &Plane, px: f64, py: f64) -> Option<[f64; 2]> {
    let ray = ref_ctx.intr.unproject_ray([px, py]);
    let denom = dot3(plane.normal, ray);
    if denom.abs() < 1e-9 {
        return None;
    }
    let t = dot3(plane.normal, plane.point) / denom;
    if t <= 1e-6 {
        return None;
    }
    let point_cam_ref = [ray[0] * t, ray[1] * t, ray[2] * t];
    let point_world = ref_ctx.to_world.act(point_cam_ref);
    let point_cam_src = src_pose.0.act(point_world);
    src_intr.project(point_cam_src)
}

/// 🧩️ Warps the reference patch centered at `center` into one source view under `plane`, sampling
/// bilinearly; `None` as soon as any offset fails to warp (behind the camera, parallel to the ray,
/// or projects behind the source camera).
fn warp_patch_to_src(ref_ctx: &RefContext<'_>, src_view: &(remodeling_image::ImageGray, remodeling_camera::CameraPose, remodeling_camera::Intrinsics), center: (u32, u32), radius: i32, plane: &Plane) -> Option<remodeling_image::Patch> {
    let (src_img, src_pose, src_intr) = src_view;
    let side = (2 * radius + 1) as usize;
    let mut data = Vec::with_capacity(side * side);
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let px = f64::from(center.0) + f64::from(dx);
            let py = f64::from(center.1) + f64::from(dy);
            let src_px = warp_point_to_src(ref_ctx, src_pose, src_intr, plane, px, py)?;
            data.push(src_img.sample(src_px[0] as f32, src_px[1] as f32));
        }
    }
    Some(remodeling_image::Patch { radius: radius as u32, data })
}

/// 🎯️ Average ZNCC over the `best_k` lowest-cost (highest-ZNCC) source views' warps under `plane`;
/// source views for which the warp fails are skipped before ranking. `best_k` is clamped to the
/// number of views that produced a valid warp, so `usize::MAX` means "use every valid view" (the
/// [`PlaneSweep`](self) fast path's all-views aggregation). Gipuma-style multi-view PatchMatch keeps
/// only the strongest-agreeing subset per pixel rather than averaging in occluded/low-texture source
/// views that would otherwise drag the score down. `-1.0` (the worst possible ZNCC) when no source
/// view produced a valid warp.
fn patch_zncc_cost(ref_ctx: &RefContext<'_>, src_views: &[(remodeling_image::ImageGray, remodeling_camera::CameraPose, remodeling_camera::Intrinsics)], center: (u32, u32), radius: i32, plane: &Plane, best_k: usize) -> f32 {
    let ref_patch = remodeling_image::extract_patch(ref_ctx.img, center.0 as f32, center.1 as f32, radius as u32, 0.0);
    let mut scores: Vec<f32> = src_views.iter().filter_map(|src_view| warp_patch_to_src(ref_ctx, src_view, center, radius, plane).map(|src_patch| remodeling_image::zncc(&ref_patch, &src_patch))).collect();
    if scores.is_empty() {
        return -1.0;
    }
    scores.sort_by(|a, b| b.total_cmp(a));
    let take = best_k.max(1).min(scores.len());
    scores[..take].iter().sum::<f32>() / take as f32
}

/// 🎯️ [`patch_zncc_cost`] for a per-pixel depth/normal hypothesis, via [`plane_from_depth_normal`].
fn multi_view_cost(ref_ctx: &RefContext<'_>, src_views: &[(remodeling_image::ImageGray, remodeling_camera::CameraPose, remodeling_camera::Intrinsics)], center: (u32, u32), radius: i32, depth: f32, normal: [f32; 3], best_k: usize) -> f32 {
    let plane = plane_from_depth_normal(ref_ctx.intr, center, depth, normal);
    patch_zncc_cost(ref_ctx, src_views, center, radius, &plane, best_k)
}

/// 🧭️ Random unit normal in the hemisphere facing the reference camera (`z < 0`, since camera-frame
/// points in front of the camera have `z > 0`).
fn random_hemisphere_normal(rng: &mut SplitMix64) -> [f32; 3] {
    loop {
        let x = rng.next_range(-1.0, 1.0);
        let y = rng.next_range(-1.0, 1.0);
        let z = rng.next_range(-1.0, -0.2);
        let len = (x * x + y * y + z * z).sqrt();
        if len > 1e-6 {
            return [x / len, y / len, z / len];
        }
    }
}

/// 🧭️ Small random perturbation of a unit normal, renormalized and re-clamped to face the camera.
fn perturb_normal(rng: &mut SplitMix64, n: [f32; 3], magnitude: f32) -> [f32; 3] {
    let perturbed = [n[0] + rng.next_range(-magnitude, magnitude), n[1] + rng.next_range(-magnitude, magnitude), (n[2] + rng.next_range(-magnitude, magnitude)).min(-0.05)];
    let len = (perturbed[0] * perturbed[0] + perturbed[1] * perturbed[1] + perturbed[2] * perturbed[2]).sqrt();
    if len < 1e-6 {
        return n;
    }
    [perturbed[0] / len, perturbed[1] / len, perturbed[2] / len]
}

/// 🧭️ 4-connected in-bounds neighbors of `(x, y)`.
fn neighbor_offsets(x: u32, y: u32, width: u32, height: u32) -> Vec<(u32, u32)> {
    let mut out = Vec::with_capacity(4);
    if x > 0 {
        out.push((x - 1, y));
    }
    if x + 1 < width {
        out.push((x + 1, y));
    }
    if y > 0 {
        out.push((x, y - 1));
    }
    if y + 1 < height {
        out.push((x, y + 1));
    }
    out
}

/// ⚙️ Tuning knobs for [`patchmatch_mvs`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PatchMatchConfig {
    pub window_radius: i32,
    pub iterations: u32,
    pub depth_min: f32,
    pub depth_max: f32,
    pub seed: u64,
    /// 🔝️ Number of lowest-cost (highest-ZNCC) source views aggregated per pixel, per
    /// [`patch_zncc_cost`]; clamped to however many views produced a valid warp at that pixel.
    pub best_k: usize,
}

impl Default for PatchMatchConfig {
    fn default() -> Self {
        Self { window_radius: 3, iterations: 4, depth_min: 0.1, depth_max: 100.0, seed: 0x5EED_1234_ABCD_EF01, best_k: 3 }
    }
}

/// 🌫️ Sequential Gipuma-style PatchMatch multi-view stereo: every reference pixel carries a plane
/// hypothesis (depth + normal), initialized randomly within `[depth_min, depth_max]` via a seeded
/// [`SplitMix64`], then refined over `cfg.iterations` red/black checkerboard passes. Each pass
/// propagates a pixel's 4-connected neighbor hypotheses when they score a higher `cfg.best_k`-view
/// aggregated ZNCC (via [`plane_from_depth_normal`]'s induced warp into the strongest-agreeing
/// `cfg.best_k` source views, per [`patch_zncc_cost`]), then tries one randomly perturbed hypothesis
/// with a search radius that shrinks geometrically with the iteration index. Confidence is the ZNCC
/// cost rescaled from `[-1, 1]` to `[0, 1]`; pixels for which no source view ever produced a valid
/// warp are left at the invalid sentinel.
pub fn patchmatch_mvs(
    ref_img: &remodeling_image::ImageGray,
    ref_cam: &(remodeling_camera::CameraPose, remodeling_camera::Intrinsics),
    src_views: &[(remodeling_image::ImageGray, remodeling_camera::CameraPose, remodeling_camera::Intrinsics)],
    cfg: &PatchMatchConfig,
) -> DepthMap {
    let (width, height) = (ref_img.width, ref_img.height);
    let mut out = DepthMap::new(width, height);
    if width == 0 || height == 0 || src_views.is_empty() {
        return out;
    }
    let ref_to_world = ref_cam.0 .0.inverse();
    let ref_ctx = RefContext { img: ref_img, intr: &ref_cam.1, to_world: &ref_to_world };
    let radius = cfg.window_radius.max(1);
    let n = (width as usize) * (height as usize);
    let mut depths = vec![0.0f32; n];
    let mut normals = vec![[0.0f32, 0.0, -1.0]; n];
    let mut costs = vec![-1.0f32; n];

    for y in 0..height {
        for x in 0..width {
            let i = depthmap_index(width, x, y);
            let mut rng = SplitMix64::new(cfg.seed ^ (u64::from(i as u32)).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let d = rng.next_range(cfg.depth_min, cfg.depth_max);
            let n_hat = random_hemisphere_normal(&mut rng);
            let c = multi_view_cost(&ref_ctx, src_views, (x, y), radius, d, n_hat, cfg.best_k);
            depths[i] = d;
            normals[i] = n_hat;
            costs[i] = c;
        }
    }

    for iter in 0..cfg.iterations {
        for parity in 0..2u32 {
            for y in 0..height {
                for x in 0..width {
                    if (x + y) % 2 != parity {
                        continue;
                    }
                    let i = depthmap_index(width, x, y);
                    let mut best_depth = depths[i];
                    let mut best_normal = normals[i];
                    let mut best_cost = costs[i];
                    for (nx, ny) in neighbor_offsets(x, y, width, height) {
                        let ni = depthmap_index(width, nx, ny);
                        let cost = multi_view_cost(&ref_ctx, src_views, (x, y), radius, depths[ni], normals[ni], cfg.best_k);
                        if cost > best_cost {
                            best_cost = cost;
                            best_depth = depths[ni];
                            best_normal = normals[ni];
                        }
                    }
                    let mut rng = SplitMix64::new(cfg.seed ^ (u64::from(iter) + 1).wrapping_mul(0xD1B5_4A32_9E37_79B9) ^ u64::from(i as u32));
                    let shrink = 0.5f32.powi(iter as i32);
                    let depth_span = (cfg.depth_max - cfg.depth_min).max(1e-6) * 0.5 * shrink;
                    let cand_depth = (best_depth + rng.next_range(-depth_span, depth_span)).clamp(cfg.depth_min, cfg.depth_max);
                    let cand_normal = perturb_normal(&mut rng, best_normal, 0.3 * shrink);
                    let cand_cost = multi_view_cost(&ref_ctx, src_views, (x, y), radius, cand_depth, cand_normal, cfg.best_k);
                    if cand_cost > best_cost {
                        best_cost = cand_cost;
                        best_depth = cand_depth;
                        best_normal = cand_normal;
                    }
                    depths[i] = best_depth;
                    normals[i] = best_normal;
                    costs[i] = best_cost;
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let i = depthmap_index(width, x, y);
            if costs[i] > -1.0 {
                out.depth[i] = depths[i];
                out.normal[i] = normals[i];
                out.confidence[i] = ((costs[i] + 1.0) * 0.5).clamp(0.0, 1.0);
            }
        }
    }
    out
}

/// 🧭️ Fuel-bounded phases for the production PatchMatch path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchMatchPhase {
    Allocate,
    Initialize,
    Propagate,
    Publish,
    Done,
}

/// 🌫️ Owned PatchMatch checkpoint. Each advance touches at most `pixel_budget` hypotheses; image
/// buffers are filled incrementally and the completed map is moved out without a terminal copy.
pub struct PatchMatchPreparation {
    width: u32,
    height: u32,
    phase: PatchMatchPhase,
    cursor: usize,
    iteration: u32,
    parity: u32,
    depths: Vec<f32>,
    normals: Vec<[f32; 3]>,
    costs: Vec<f32>,
    output: DepthMap,
}

impl PatchMatchPreparation {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = width as usize * height as usize;
        Self {
            width,
            height,
            phase: PatchMatchPhase::Allocate,
            cursor: 0,
            iteration: 0,
            parity: 0,
            depths: Vec::with_capacity(pixels),
            normals: Vec::with_capacity(pixels),
            costs: Vec::with_capacity(pixels),
            output: DepthMap { width, height, depth: Vec::with_capacity(pixels), normal: Vec::with_capacity(pixels), confidence: Vec::with_capacity(pixels) },
        }
    }

    pub fn phase(&self) -> PatchMatchPhase {
        self.phase
    }

    pub fn advance(
        &mut self,
        ref_img: &remodeling_image::ImageGray,
        ref_cam: &(remodeling_camera::CameraPose, remodeling_camera::Intrinsics),
        src_views: &[(remodeling_image::ImageGray, remodeling_camera::CameraPose, remodeling_camera::Intrinsics)],
        cfg: &PatchMatchConfig,
        pixel_budget: usize,
    ) -> bool {
        let budget = pixel_budget.max(1);
        let pixels = self.width as usize * self.height as usize;
        if pixels == 0 || src_views.is_empty() {
            self.phase = PatchMatchPhase::Done;
            return true;
        }
        match self.phase {
            PatchMatchPhase::Allocate => {
                let end = self.cursor.saturating_add(budget).min(pixels);
                self.depths.resize(end, 0.0);
                self.normals.resize(end, [0.0, 0.0, -1.0]);
                self.costs.resize(end, -1.0);
                self.output.depth.resize(end, 0.0);
                self.output.normal.resize(end, [0.0; 3]);
                self.output.confidence.resize(end, 0.0);
                self.cursor = end;
                if end == pixels {
                    self.phase = PatchMatchPhase::Initialize;
                    self.cursor = 0;
                }
            }
            PatchMatchPhase::Initialize => {
                let ref_to_world = ref_cam.0 .0.inverse();
                let ref_ctx = RefContext { img: ref_img, intr: &ref_cam.1, to_world: &ref_to_world };
                let radius = cfg.window_radius.clamp(1, 4);
                let end = self.cursor.saturating_add(budget).min(pixels);
                for index in self.cursor..end {
                    let x = index as u32 % self.width;
                    let y = index as u32 / self.width;
                    let mut rng = SplitMix64::new(cfg.seed ^ (index as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
                    let depth = rng.next_range(cfg.depth_min, cfg.depth_max);
                    let normal = random_hemisphere_normal(&mut rng);
                    self.depths[index] = depth;
                    self.normals[index] = normal;
                    self.costs[index] = multi_view_cost(&ref_ctx, src_views, (x, y), radius, depth, normal, cfg.best_k.min(4));
                }
                self.cursor = end;
                if end == pixels {
                    self.phase = if cfg.iterations == 0 { PatchMatchPhase::Publish } else { PatchMatchPhase::Propagate };
                    self.cursor = 0;
                }
            }
            PatchMatchPhase::Propagate => {
                let ref_to_world = ref_cam.0 .0.inverse();
                let ref_ctx = RefContext { img: ref_img, intr: &ref_cam.1, to_world: &ref_to_world };
                let radius = cfg.window_radius.clamp(1, 4);
                let mut consumed = 0;
                while self.cursor < pixels && consumed < budget {
                    let index = self.cursor;
                    self.cursor += 1;
                    let x = index as u32 % self.width;
                    let y = index as u32 / self.width;
                    if (x + y) % 2 != self.parity {
                        continue;
                    }
                    let mut best_depth = self.depths[index];
                    let mut best_normal = self.normals[index];
                    let mut best_cost = self.costs[index];
                    for (nx, ny) in neighbor_offsets(x, y, self.width, self.height) {
                        let neighbor = depthmap_index(self.width, nx, ny);
                        let cost = multi_view_cost(&ref_ctx, src_views, (x, y), radius, self.depths[neighbor], self.normals[neighbor], cfg.best_k.min(4));
                        if cost > best_cost {
                            best_depth = self.depths[neighbor];
                            best_normal = self.normals[neighbor];
                            best_cost = cost;
                        }
                    }
                    let mut rng = SplitMix64::new(cfg.seed ^ (u64::from(self.iteration) + 1).wrapping_mul(0xD1B5_4A32_9E37_79B9) ^ index as u64);
                    let shrink = 0.5f32.powi(self.iteration as i32);
                    let span = (cfg.depth_max - cfg.depth_min).max(1e-6) * 0.5 * shrink;
                    let candidate_depth = (best_depth + rng.next_range(-span, span)).clamp(cfg.depth_min, cfg.depth_max);
                    let candidate_normal = perturb_normal(&mut rng, best_normal, 0.3 * shrink);
                    let candidate_cost = multi_view_cost(&ref_ctx, src_views, (x, y), radius, candidate_depth, candidate_normal, cfg.best_k.min(4));
                    if candidate_cost > best_cost {
                        best_depth = candidate_depth;
                        best_normal = candidate_normal;
                        best_cost = candidate_cost;
                    }
                    self.depths[index] = best_depth;
                    self.normals[index] = best_normal;
                    self.costs[index] = best_cost;
                    consumed += 1;
                }
                if self.cursor == pixels {
                    self.cursor = 0;
                    if self.parity == 0 {
                        self.parity = 1;
                    } else {
                        self.parity = 0;
                        self.iteration += 1;
                        if self.iteration >= cfg.iterations {
                            self.phase = PatchMatchPhase::Publish;
                        }
                    }
                }
            }
            PatchMatchPhase::Publish => {
                let end = self.cursor.saturating_add(budget).min(pixels);
                for index in self.cursor..end {
                    if self.costs[index] > -1.0 {
                        self.output.depth[index] = self.depths[index];
                        self.output.normal[index] = self.normals[index];
                        self.output.confidence[index] = ((self.costs[index] + 1.0) * 0.5).clamp(0.0, 1.0);
                    }
                }
                self.cursor = end;
                if end == pixels {
                    self.phase = PatchMatchPhase::Done;
                }
            }
            PatchMatchPhase::Done => {}
        }
        self.phase == PatchMatchPhase::Done
    }

    pub fn finish(self) -> Option<DepthMap> {
        (self.phase == PatchMatchPhase::Done).then_some(self.output)
    }
}
// #endregion 🔖️PatchMatch

// #region 🔖️PlaneSweep
/// 🌫️ Fronto-parallel plane sweep: the faster, lower-fidelity alternative to [`patchmatch_mvs`].
/// Sweeps `num_planes` evenly-spaced inverse-depth hypotheses (a plane at constant reference-camera
/// depth `D`, i.e. `Plane { point: [0, 0, D], normal: [0, 0, 1] }`), accumulates each pixel's
/// average ZNCC across every source view at that depth, and keeps the best-scoring depth per pixel.
/// Recomputes the reference patch for every plane hypothesis rather than caching it once per pixel,
/// which keeps the implementation a straightforward nested loop at the cost of redundant work.
pub fn plane_sweep_depth(
    ref_img: &remodeling_image::ImageGray,
    ref_cam: &(remodeling_camera::CameraPose, remodeling_camera::Intrinsics),
    src_views: &[(remodeling_image::ImageGray, remodeling_camera::CameraPose, remodeling_camera::Intrinsics)],
    depth_min: f32,
    depth_max: f32,
    num_planes: u32,
) -> DepthMap {
    let (width, height) = (ref_img.width, ref_img.height);
    let mut out = DepthMap::new(width, height);
    if width == 0 || height == 0 || src_views.is_empty() || num_planes == 0 || depth_min <= 0.0 || depth_max <= depth_min {
        return out;
    }
    let ref_to_world = ref_cam.0 .0.inverse();
    let ref_ctx = RefContext { img: ref_img, intr: &ref_cam.1, to_world: &ref_to_world };
    let radius = 2i32;
    let inv_min = 1.0 / depth_max;
    let inv_max = 1.0 / depth_min;
    let n = (width as usize) * (height as usize);
    let mut best_cost = vec![-1.0f32; n];

    for plane_idx in 0..num_planes {
        let t = if num_planes == 1 { 0.0 } else { plane_idx as f32 / (num_planes - 1) as f32 };
        let inv_depth = inv_min + (inv_max - inv_min) * t;
        let depth = 1.0 / inv_depth.max(1e-9);
        let plane = Plane { point: [0.0, 0.0, f64::from(depth)], normal: [0.0, 0.0, 1.0] };
        for y in 0..height {
            for x in 0..width {
                let cost = patch_zncc_cost(&ref_ctx, src_views, (x, y), radius, &plane, usize::MAX);
                let i = depthmap_index(width, x, y);
                if cost > best_cost[i] {
                    best_cost[i] = cost;
                    out.depth[i] = depth;
                    out.normal[i] = [0.0, 0.0, -1.0];
                    out.confidence[i] = ((cost + 1.0) * 0.5).clamp(0.0, 1.0);
                }
            }
        }
    }
    out
}
// #endregion 🔖️PlaneSweep

// #region 🔖️DepthFilter
/// 🔁️ Invalidates reference pixels whose depth does not round-trip consistently into `depth_other`:
/// unprojects each valid `depth_ref` pixel to world space, reprojects into the other view, and
/// invalidates whenever the two camera-frame depths differ by more than `max_diff` (nearest-pixel
/// lookup into `depth_other`, no subpixel interpolation).
pub fn left_right_check(depth_ref: &DepthMap, depth_other: &DepthMap, ref_cam: &(remodeling_camera::CameraPose, remodeling_camera::Intrinsics), other_cam: &(remodeling_camera::CameraPose, remodeling_camera::Intrinsics), max_diff: f32) -> DepthMap {
    let mut out = depth_ref.clone();
    let ref_to_world = ref_cam.0 .0.inverse();
    for y in 0..depth_ref.height {
        for x in 0..depth_ref.width {
            let idx = depthmap_index(depth_ref.width, x, y);
            let Some(d) = depth_ref.get(x, y) else { continue };
            let ray = ref_cam.1.unproject_ray([f64::from(x), f64::from(y)]);
            let point_cam_ref = [ray[0] * f64::from(d), ray[1] * f64::from(d), ray[2] * f64::from(d)];
            let point_world = ref_to_world.act(point_cam_ref);
            let point_cam_other = other_cam.0 .0.act(point_world);
            let predicted_depth = point_cam_other[2];
            let Some(px) = other_cam.1.project(point_cam_other) else {
                depthmap_invalidate(&mut out, idx);
                continue;
            };
            let (ox, oy) = (px[0].round(), px[1].round());
            if ox < 0.0 || oy < 0.0 || ox as u32 >= depth_other.width || oy as u32 >= depth_other.height {
                depthmap_invalidate(&mut out, idx);
                continue;
            }
            match depth_other.get(ox as u32, oy as u32) {
                Some(other_d) if (f64::from(other_d) - predicted_depth).abs() <= f64::from(max_diff) => {}
                _ => depthmap_invalidate(&mut out, idx),
            }
        }
    }
    out
}

/// 🔁️ Connected-component filter over 4-connected valid pixels with similar depth (within
/// `depth_tolerance` of each 4-neighbor step, i.e. a flood fill rather than a global tolerance):
/// components smaller than `min_component_size` are invalidated. Iterative flood fill via an
/// explicit stack, so it stays safe under wasm32's default (small) stack.
pub fn speckle_filter(depth: &DepthMap, min_component_size: u32, depth_tolerance: f32) -> DepthMap {
    let mut out = depth.clone();
    let (w, h) = (depth.width, depth.height);
    let n = (w as usize) * (h as usize);
    let mut visited = vec![false; n];
    let mut stack = Vec::new();
    for start in 0..n {
        if visited[start] {
            continue;
        }
        let (sx, sy) = (start as u32 % w, start as u32 / w);
        visited[start] = true;
        if depth.get(sx, sy).is_none() {
            continue;
        }
        let mut component = vec![start];
        stack.clear();
        stack.push(start);
        while let Some(cur) = stack.pop() {
            let (cx, cy) = (cur as u32 % w, cur as u32 / w);
            let Some(cd) = depth.get(cx, cy) else { continue };
            for (nx, ny) in neighbor_offsets(cx, cy, w, h) {
                let ni = depthmap_index(w, nx, ny);
                if visited[ni] {
                    continue;
                }
                if let Some(nd) = depth.get(nx, ny) {
                    if (nd - cd).abs() <= depth_tolerance {
                        visited[ni] = true;
                        component.push(ni);
                        stack.push(ni);
                    }
                }
            }
        }
        if component.len() < min_component_size as usize {
            for idx in component {
                depthmap_invalidate(&mut out, idx);
            }
        }
    }
    out
}

/// 🩹️ Fills invalid pixels from the median depth of valid pixels in a `(2 window + 1)²` window;
/// pixels with no valid neighbor stay invalid. Normal/confidence at filled pixels are left at the
/// invalid-sentinel zero fill, since a median depth alone gives no basis for either.
pub fn median_fill(depth: &DepthMap, window: u32) -> DepthMap {
    let mut out = depth.clone();
    let (w, h) = (depth.width, depth.height);
    let r = i64::from(window);
    for y in 0..h {
        for x in 0..w {
            if depth.get(x, y).is_some() {
                continue;
            }
            let mut vals = Vec::new();
            for dy in -r..=r {
                for dx in -r..=r {
                    let nx = i64::from(x) + dx;
                    let ny = i64::from(y) + dy;
                    if nx < 0 || ny < 0 || nx as u32 >= w || ny as u32 >= h {
                        continue;
                    }
                    if let Some(v) = depth.get(nx as u32, ny as u32) {
                        vals.push(v);
                    }
                }
            }
            if vals.is_empty() {
                continue;
            }
            vals.sort_by(f32::total_cmp);
            out.depth[depthmap_index(w, x, y)] = vals[vals.len() / 2];
        }
    }
    out
}

/// 📶️ Best/second-best matching-cost margin as a confidence signal, replacing `depth`'s own
/// confidence buffer in place (depth/normal untouched): at every valid pixel, re-scores the
/// accepted hypothesis (`best_cost`, via [`multi_view_cost`] over every source view) against the
/// stronger of two depth-perturbed competitors at `depth ± depth_step` (`second_best_cost`); a wide
/// margin between them means the accepted depth clearly beats its nearest rival, while a narrow
/// margin flags an ambiguous match (low texture, repetitive pattern) independent of the accepted
/// hypothesis's own ZNCC score. Margin is clamped to `[0, 2]` (the maximum possible ZNCC spread) and
/// rescaled to `[0, 1]`.
pub fn margin_confidence(
    depth: &DepthMap,
    ref_img: &remodeling_image::ImageGray,
    ref_cam: &(remodeling_camera::CameraPose, remodeling_camera::Intrinsics),
    src_views: &[(remodeling_image::ImageGray, remodeling_camera::CameraPose, remodeling_camera::Intrinsics)],
    radius: i32,
    depth_step: f32,
) -> DepthMap {
    let mut out = depth.clone();
    if src_views.is_empty() || depth_step <= 0.0 {
        return out;
    }
    let ref_to_world = ref_cam.0 .0.inverse();
    let ref_ctx = RefContext { img: ref_img, intr: &ref_cam.1, to_world: &ref_to_world };
    let all_views = src_views.len();
    for y in 0..depth.height {
        for x in 0..depth.width {
            let idx = depthmap_index(depth.width, x, y);
            let Some(d) = depth.get(x, y) else { continue };
            let normal = depth.normal[idx];
            let best_cost = multi_view_cost(&ref_ctx, src_views, (x, y), radius, d, normal, all_views);
            let cost_minus = multi_view_cost(&ref_ctx, src_views, (x, y), radius, (d - depth_step).max(1e-6), normal, all_views);
            let cost_plus = multi_view_cost(&ref_ctx, src_views, (x, y), radius, d + depth_step, normal, all_views);
            let second_best = cost_minus.max(cost_plus);
            let margin = (best_cost - second_best).clamp(0.0, 2.0);
            out.confidence[idx] = margin * 0.5;
        }
    }
    out
}
// #endregion 🔖️DepthFilter

// #region 🔖️Fusion
/// ⚙️ Tuning knobs for [`fuse_depth_maps`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FusionConfig {
    pub max_relative_depth_diff: f32,
    pub max_normal_angle_deg: f32,
    pub min_consistent_views: usize,
}

impl Default for FusionConfig {
    fn default() -> Self {
        Self { max_relative_depth_diff: 0.01, max_normal_angle_deg: 30.0, min_consistent_views: 2 }
    }
}

/// 🧭️ Whether two (not necessarily unit or consistently oriented) normals agree within
/// `cos_thresh`; either normal being zero-length (no data) is treated as "no disagreement".
fn normal_angle_ok(n0: [f32; 3], n1: [f32; 3], cos_thresh: f32) -> bool {
    let len0 = (n0[0] * n0[0] + n0[1] * n0[1] + n0[2] * n0[2]).sqrt();
    let len1 = (n1[0] * n1[0] + n1[1] * n1[1] + n1[2] * n1[2]).sqrt();
    if len0 < 1e-6 || len1 < 1e-6 {
        return true;
    }
    let dot = (n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2]) / (len0 * len1);
    dot.abs() >= cos_thresh
}

/// 🧩️ Fuses per-view depth maps into one [`PointCloud`]: for every valid pixel of every view,
/// unprojects to world space, then checks agreement against every *other* view by reprojecting the
/// point into that view's depth map (relative depth difference within
/// `cfg.max_relative_depth_diff`, and — when both sides have normal data — a normal-angle check
/// against `cfg.max_normal_angle_deg`). Points reaching `cfg.min_consistent_views` agreeing views
/// (counting the originating view) are kept, positioned at the average of every agreeing view's own
/// unprojection of that surface point.
pub fn fuse_depth_maps(views: &[(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)], depth_maps: &[DepthMap], cfg: &FusionConfig) -> PointCloud {
    let to_worlds: Vec<remodeling_camera::Se3> = views.iter().map(|(pose, _)| pose.0.inverse()).collect();
    let cos_thresh = cfg.max_normal_angle_deg.to_radians().cos();
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut confidence = Vec::new();

    for (vi, (pose, intr)) in views.iter().enumerate() {
        let Some(dm) = depth_maps.get(vi) else { continue };
        let _ = pose;
        for y in 0..dm.height {
            for x in 0..dm.width {
                let Some(d) = dm.get(x, y) else { continue };
                let idx = depthmap_index(dm.width, x, y);
                let n0 = dm.normal[idx];
                let ray = intr.unproject_ray([f64::from(x), f64::from(y)]);
                let point_cam = [ray[0] * f64::from(d), ray[1] * f64::from(d), ray[2] * f64::from(d)];
                let point_world = to_worlds[vi].act(point_cam);

                let mut agree_positions = vec![point_world];
                for (vj, (pose2, intr2)) in views.iter().enumerate() {
                    if vj == vi {
                        continue;
                    }
                    let Some(dm2) = depth_maps.get(vj) else { continue };
                    let point_cam2 = pose2.0.act(point_world);
                    let Some(px2) = intr2.project(point_cam2) else { continue };
                    let (ox, oy) = (px2[0].round(), px2[1].round());
                    if ox < 0.0 || oy < 0.0 || ox as u32 >= dm2.width || oy as u32 >= dm2.height {
                        continue;
                    }
                    let Some(d2) = dm2.get(ox as u32, oy as u32) else { continue };
                    let rel_diff = (point_cam2[2] - f64::from(d2)).abs() / f64::from(d2).max(1e-6);
                    if rel_diff > f64::from(cfg.max_relative_depth_diff) {
                        continue;
                    }
                    let idx2 = depthmap_index(dm2.width, ox as u32, oy as u32);
                    if !normal_angle_ok(n0, dm2.normal[idx2], cos_thresh) {
                        continue;
                    }
                    let other_ray = intr2.unproject_ray(px2);
                    let other_point_cam = [other_ray[0] * f64::from(d2), other_ray[1] * f64::from(d2), other_ray[2] * f64::from(d2)];
                    agree_positions.push(to_worlds[vj].act(other_point_cam));
                }

                if agree_positions.len() >= cfg.min_consistent_views {
                    let mut avg = [0.0; 3];
                    for p in &agree_positions {
                        for (a, v) in avg.iter_mut().zip(p.iter()) {
                            *a += v;
                        }
                    }
                    let inv = 1.0 / agree_positions.len() as f64;
                    for a in avg.iter_mut() {
                        *a *= inv;
                    }
                    positions.push(avg);
                    normals.push(n0);
                    confidence.push(dm.confidence[idx]);
                }
            }
        }
    }
    PointCloud { positions, normals, colors: Vec::new(), confidence, classification: Vec::new() }
}

/// 🧩️ Per-pixel fusion checkpoint. A source pixel and each cross-view consistency comparison are
/// separate fuel units, so neither camera count nor raster size can create an opaque fusion turn.
pub struct FusionPreparation {
    view: usize,
    pixel: usize,
    comparison: usize,
    point_world: Option<[f64; 3]>,
    agreeing_sum: [f64; 3],
    agreeing_count: usize,
    source_normal: [f32; 3],
    source_confidence: f32,
    output: PointCloud,
    complete: bool,
}

impl FusionPreparation {
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.min(MAX_INTERACTIVE_FUSED_POINTS);
        Self {
            view: 0,
            pixel: 0,
            comparison: 0,
            point_world: None,
            agreeing_sum: [0.0; 3],
            agreeing_count: 0,
            source_normal: [0.0; 3],
            source_confidence: 0.0,
            output: PointCloud { positions: Vec::with_capacity(capacity), normals: Vec::with_capacity(capacity), confidence: Vec::with_capacity(capacity), ..PointCloud::default() },
            complete: false,
        }
    }

    fn finish_pixel(&mut self, minimum: usize) {
        if self.agreeing_count >= minimum && self.output.positions.len() < MAX_INTERACTIVE_FUSED_POINTS {
            let inverse = 1.0 / self.agreeing_count as f64;
            self.output.positions.push(self.agreeing_sum.map(|coordinate| coordinate * inverse));
            self.output.normals.push(self.source_normal);
            self.output.confidence.push(self.source_confidence);
        }
        self.pixel += 1;
        self.comparison = 0;
        self.point_world = None;
    }

    pub fn advance(&mut self, views: &[(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)], depth_maps: &[DepthMap], cfg: &FusionConfig, comparison_budget: usize) -> bool {
        let mut remaining = comparison_budget.max(1);
        let cosine_threshold = cfg.max_normal_angle_deg.to_radians().cos();
        while remaining > 0 && self.view < views.len().min(depth_maps.len()) {
            let source = &depth_maps[self.view];
            if self.pixel >= source.depth.len() {
                self.view += 1;
                self.pixel = 0;
                self.comparison = 0;
                self.point_world = None;
                continue;
            }
            let x = self.pixel as u32 % source.width;
            let y = self.pixel as u32 / source.width;
            if self.point_world.is_none() {
                remaining -= 1;
                let Some(depth) = source.get(x, y) else {
                    self.pixel += 1;
                    continue;
                };
                let ray = views[self.view].1.unproject_ray([f64::from(x), f64::from(y)]);
                let camera_point = ray.map(|coordinate| coordinate * f64::from(depth));
                let world = views[self.view].0 .0.inverse().act(camera_point);
                self.point_world = Some(world);
                self.agreeing_sum = world;
                self.agreeing_count = 1;
                self.source_normal = source.normal[self.pixel];
                self.source_confidence = source.confidence[self.pixel];
                self.comparison = 0;
                continue;
            }
            if self.comparison >= views.len().min(depth_maps.len()) {
                self.finish_pixel(cfg.min_consistent_views);
                continue;
            }
            let compared = self.comparison;
            self.comparison += 1;
            remaining -= 1;
            if compared == self.view {
                continue;
            }
            let world = self.point_world.expect("active fusion point");
            let (pose, intrinsics) = views[compared];
            let camera_point = pose.0.act(world);
            let Some(pixel) = intrinsics.project(camera_point) else { continue };
            let ox = pixel[0].round();
            let oy = pixel[1].round();
            let compared_map = &depth_maps[compared];
            if ox < 0.0 || oy < 0.0 || ox as u32 >= compared_map.width || oy as u32 >= compared_map.height {
                continue;
            }
            let Some(depth) = compared_map.get(ox as u32, oy as u32) else { continue };
            let relative_difference = (camera_point[2] - f64::from(depth)).abs() / f64::from(depth).max(1e-6);
            if relative_difference > f64::from(cfg.max_relative_depth_diff) {
                continue;
            }
            let compared_index = depthmap_index(compared_map.width, ox as u32, oy as u32);
            if !normal_angle_ok(self.source_normal, compared_map.normal[compared_index], cosine_threshold) {
                continue;
            }
            let ray = intrinsics.unproject_ray(pixel);
            let camera_surface = ray.map(|coordinate| coordinate * f64::from(depth));
            let agreeing_world = pose.0.inverse().act(camera_surface);
            for (sum, coordinate) in self.agreeing_sum.iter_mut().zip(agreeing_world) {
                *sum += coordinate;
            }
            self.agreeing_count += 1;
        }
        if self.view >= views.len().min(depth_maps.len()) {
            self.complete = true;
        }
        self.complete
    }

    pub fn finish(self) -> Option<PointCloud> {
        self.complete.then_some(self.output)
    }
}
// #endregion 🔖️Fusion

// #region 🔖️Tsdf
const TSDF_BLOCK_DIM: i32 = 8;
const TSDF_BLOCK_VOXELS: usize = (TSDF_BLOCK_DIM * TSDF_BLOCK_DIM * TSDF_BLOCK_DIM) as usize;
const TSDF_MAX_WEIGHT: f32 = 100.0;

/// 🧊️ One `8x8x8` chunk of a [`TsdfVolume`]'s hashed sparse grid: per-voxel signed distance and
/// accumulated integration weight, row-major within the block (`(lz * 8 + ly) * 8 + lx`).
#[derive(Clone, Debug)]
struct TsdfBlock {
    sdf: Vec<f32>,
    weight: Vec<f32>,
}

impl TsdfBlock {
    fn new() -> Self {
        Self { sdf: vec![0.0; TSDF_BLOCK_VOXELS], weight: vec![0.0; TSDF_BLOCK_VOXELS] }
    }

    fn local_index(local: [i32; 3]) -> usize {
        ((local[2] * TSDF_BLOCK_DIM + local[1]) * TSDF_BLOCK_DIM + local[0]) as usize
    }
}

/// 🧊️ Truncated signed distance field over a hashed grid of `8x8x8` voxel blocks (Curless-Levoy
/// weighted integration), keyed by block coordinate so only blocks actually touched by an
/// integrated depth map ever get allocated.
#[derive(Clone, Debug)]
pub struct TsdfVolume {
    pub voxel_size: f64,
    pub truncation: f64,
    blocks: BTreeMap<[i32; 3], TsdfBlock>,
}

/// 🧊️ Splits a global voxel coordinate into its block coordinate and within-block local coordinate.
fn tsdf_block_and_local(global: [i32; 3]) -> ([i32; 3], [i32; 3]) {
    let block = global.map(|c| c.div_euclid(TSDF_BLOCK_DIM));
    let local = global.map(|c| c.rem_euclid(TSDF_BLOCK_DIM));
    (block, local)
}

impl TsdfVolume {
    /// 🧊️ Empty volume with the given voxel edge length and truncation distance (both in world
    /// units).
    pub fn new(voxel_size: f64, truncation: f64) -> Self {
        Self { voxel_size, truncation, blocks: BTreeMap::new() }
    }

    fn voxel_coord(&self, p: [f64; 3]) -> [i32; 3] {
        p.map(|c| (c / self.voxel_size).floor() as i32)
    }

    fn integrate_point(&mut self, p: [f64; 3], sdf: f32, weight: f32) {
        let (block_coord, local) = tsdf_block_and_local(self.voxel_coord(p));
        let block = self.blocks.entry(block_coord).or_insert_with(TsdfBlock::new);
        let li = TsdfBlock::local_index(local);
        let old_w = block.weight[li];
        let new_w = (old_w + weight).min(TSDF_MAX_WEIGHT);
        block.sdf[li] = (block.sdf[li] * old_w + sdf * weight) / (old_w + weight);
        block.weight[li] = new_w;
    }

    /// 🧊️ Integrates one depth map: for each valid pixel, walks the camera ray in world space over
    /// `[measured_depth - truncation, measured_depth + truncation]` in half-voxel steps, and at
    /// each sample writes `signed_distance = (measured_depth - t) / |ray|` (the depth-axis gap
    /// rescaled onto the ray's own length, an approximation of the true point-to-surface distance
    /// standard to depth-map TSDF fusion) into that sample's voxel via a running weighted average.
    /// This naturally bounds the touched region to a thin shell around each pixel's ray rather than
    /// sweeping the whole volume. When `weight_by_grazing_angle`, each pixel's per-sample weight is
    /// scaled by `|cos(angle between depth.normal and the viewing ray)|` (floored at `0.05` so a
    /// grazing observation still contributes a little rather than being dropped outright) — both the
    /// PatchMatch-sourced normal and the camera ray are already expressed in the same reference-
    /// camera frame (see [`patchmatch_mvs`]), so no world transform is needed for the angle itself.
    pub fn integrate(&mut self, depth: &DepthMap, cam: &(remodeling_camera::CameraPose, remodeling_camera::Intrinsics), weight_by_grazing_angle: bool) {
        if self.voxel_size <= 0.0 || self.truncation <= 0.0 {
            return;
        }
        let to_world = cam.0 .0.inverse();
        let step = self.voxel_size * 0.5;
        let n_steps = ((2.0 * self.truncation) / step).ceil() as i64;
        for y in 0..depth.height {
            for x in 0..depth.width {
                let Some(d) = depth.get(x, y) else { continue };
                let idx = depthmap_index(depth.width, x, y);
                let ray = cam.1.unproject_ray([f64::from(x), f64::from(y)]);
                let ray_norm = (ray[0] * ray[0] + ray[1] * ray[1] + ray[2] * ray[2]).sqrt();
                if ray_norm < 1e-12 {
                    continue;
                }
                let angle_weight = if weight_by_grazing_angle {
                    let n = to_f64_3(depth.normal[idx]);
                    let n_len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                    if n_len > 1e-6 {
                        let view_dir = [-ray[0] / ray_norm, -ray[1] / ray_norm, -ray[2] / ray_norm];
                        let cos_angle = (n[0] * view_dir[0] + n[1] * view_dir[1] + n[2] * view_dir[2]) / n_len;
                        cos_angle.abs().max(0.05)
                    } else {
                        1.0
                    }
                } else {
                    1.0
                };
                for step_idx in 0..=n_steps {
                    let t = f64::from(d) - self.truncation + step_idx as f64 * step;
                    if t <= 0.0 {
                        continue;
                    }
                    let sdf = (f64::from(d) - t) / ray_norm;
                    if sdf.abs() > self.truncation {
                        continue;
                    }
                    let point_cam = [ray[0] * t, ray[1] * t, ray[2] * t];
                    let point_world = to_world.act(point_cam);
                    self.integrate_point(point_world, sdf as f32, angle_weight as f32);
                }
            }
        }
    }

    /// 🔍️ Signed distance and accumulated weight at the global integer voxel coordinate
    /// `(ix, iy, iz)` — the cross-block query surface `remodeling_mesh`'s crack-free marching cubes
    /// depends on (Amendment 2 of `remodeling-must-offer-a-vivid-gem`): resolves to the owning `8x8x8`
    /// block via [`tsdf_block_and_local`], so any two neighboring blocks that both border this
    /// coordinate report bit-identical results regardless of integration order or which block was
    /// touched first — it is a pure hash lookup keyed by block coordinate, with no block-local state
    /// that integration order could leave inconsistent. `None` distinguishes a voxel that was never
    /// observed/allocated (unknown) from a genuine zero signed distance.
    pub fn sample(&self, ix: i32, iy: i32, iz: i32) -> Option<(f64, f64)> {
        let (block_coord, local) = tsdf_block_and_local([ix, iy, iz]);
        let block = self.blocks.get(&block_coord)?;
        let li = TsdfBlock::local_index(local);
        (block.weight[li] > 0.0).then(|| (f64::from(block.sdf[li]), f64::from(block.weight[li])))
    }

    /// 🔍️ [`Self::sample`] at the voxel containing world-space point `p`, keeping just the signed
    /// distance.
    pub fn sample_tsdf(&self, p: [f64; 3]) -> Option<f64> {
        let v = self.voxel_coord(p);
        self.sample(v[0], v[1], v[2]).map(|(sdf, _)| sdf)
    }

    /// 🔍️ [`Self::sample`] at the voxel containing world-space point `p`, keeping just the weight.
    pub fn sample_weight(&self, p: [f64; 3]) -> Option<f64> {
        let v = self.voxel_coord(p);
        self.sample(v[0], v[1], v[2]).map(|(_, w)| w)
    }
}

/// 🧊️ Cursor for one depth-map integration. A continuation performs at most the requested number of
/// ray samples; sparse blocks use an ordered map so insertion never triggers a whole-table rehash.
pub struct TsdfIntegrationPreparation {
    pixel: usize,
    ray_step: i64,
    complete: bool,
}

impl Default for TsdfIntegrationPreparation {
    fn default() -> Self {
        Self::new()
    }
}

impl TsdfIntegrationPreparation {
    pub fn new() -> Self {
        Self { pixel: 0, ray_step: 0, complete: false }
    }

    pub fn advance(&mut self, volume: &mut TsdfVolume, depth: &DepthMap, camera: &(remodeling_camera::CameraPose, remodeling_camera::Intrinsics), weight_by_grazing_angle: bool, sample_budget: usize) -> bool {
        if volume.voxel_size <= 0.0 || volume.truncation <= 0.0 {
            self.complete = true;
            return true;
        }
        let mut remaining = sample_budget.max(1);
        let to_world = camera.0 .0.inverse();
        let step = volume.voxel_size * 0.5;
        let steps_per_ray = ((2.0 * volume.truncation) / step).ceil() as i64;
        while remaining > 0 && self.pixel < depth.depth.len() {
            let x = self.pixel as u32 % depth.width;
            let y = self.pixel as u32 / depth.width;
            let Some(measured) = depth.get(x, y) else {
                self.pixel += 1;
                self.ray_step = 0;
                remaining -= 1;
                continue;
            };
            if self.ray_step > steps_per_ray {
                self.pixel += 1;
                self.ray_step = 0;
                continue;
            }
            let ray = camera.1.unproject_ray([f64::from(x), f64::from(y)]);
            let ray_norm = (ray[0] * ray[0] + ray[1] * ray[1] + ray[2] * ray[2]).sqrt();
            let index = self.pixel;
            let ray_step = self.ray_step;
            self.ray_step += 1;
            remaining -= 1;
            if ray_norm < 1e-12 {
                continue;
            }
            let distance = f64::from(measured) - volume.truncation + ray_step as f64 * step;
            if distance <= 0.0 {
                continue;
            }
            let sdf = (f64::from(measured) - distance) / ray_norm;
            if sdf.abs() > volume.truncation {
                continue;
            }
            let weight = if weight_by_grazing_angle {
                let normal = to_f64_3(depth.normal[index]);
                let normal_length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
                if normal_length > 1e-6 {
                    let view = [-ray[0] / ray_norm, -ray[1] / ray_norm, -ray[2] / ray_norm];
                    ((normal[0] * view[0] + normal[1] * view[1] + normal[2] * view[2]) / normal_length).abs().max(0.05)
                } else {
                    1.0
                }
            } else {
                1.0
            };
            let camera_point = ray.map(|coordinate| coordinate * distance);
            volume.integrate_point(to_world.act(camera_point), sdf as f32, weight as f32);
        }
        if self.pixel >= depth.depth.len() {
            self.complete = true;
        }
        self.complete
    }
}
// #endregion 🔖️Tsdf

// #region 🔖️CloudOperations
/// ✂️ Builds a new [[`PointCloud`]] keeping only the given (order-preserved) indices, across every
/// present optional attribute.
fn pick_indexed<T: Copy>(v: &[T], indices: &[usize]) -> Vec<T> {
    if v.is_empty() {
        Vec::new()
    } else {
        indices.iter().map(|&i| v[i]).collect()
    }
}

fn keep_indices(cloud: &PointCloud, indices: &[usize]) -> PointCloud {
    PointCloud {
        positions: pick_indexed(&cloud.positions, indices),
        normals: pick_indexed(&cloud.normals, indices),
        colors: pick_indexed(&cloud.colors, indices),
        confidence: pick_indexed(&cloud.confidence, indices),
        classification: pick_indexed(&cloud.classification, indices),
    }
}

/// 🧮️ Local structure-tensor PCA at point `i`: gathers its `k` nearest neighbors (including itself)
/// from `tree`, forms their covariance matrix, and eigendecomposes it via
/// [`jacobi_eigen_symmetric`] (eigenvalues ascending, matching eigenvectors as columns). Shared by
/// [`estimate_normals`] (normal = eigenvector of the smallest eigenvalue) and
/// [`classify_building_vegetation`] (planarity/roughness from the eigenvalues themselves). `None`
/// when fewer than 3 neighbors are found or the eigendecomposition fails to converge.
fn local_pca(positions: &[[f64; 3]], tree: &KdTree<3>, i: usize, k: usize) -> Option<(Vec<f64>, MatD)> {
    let p = positions[i];
    let neighbors = tree.k_nearest(&p, k.max(3));
    if neighbors.len() < 3 {
        return None;
    }
    let mut mean = [0.0; 3];
    for &(id, _) in &neighbors {
        for (m, v) in mean.iter_mut().zip(positions[id as usize].iter()) {
            *m += v;
        }
    }
    let inv = 1.0 / neighbors.len() as f64;
    for m in mean.iter_mut() {
        *m *= inv;
    }
    let mut cov = MatD::zeros(3, 3);
    for &(id, _) in &neighbors {
        let q = positions[id as usize];
        let d = [q[0] - mean[0], q[1] - mean[1], q[2] - mean[2]];
        for (r, dr) in d.iter().enumerate() {
            for (c, dc) in d.iter().enumerate() {
                cov.add_at(r, c, dr * dc);
            }
        }
    }
    jacobi_eigen_symmetric(&cov, 100).ok()
}

/// 🧭️ Per-point normal estimation via local PCA ([`local_pca`]): the eigenvector of the smallest
/// eigenvalue is the local surface normal, oriented to face `viewpoint`. Points with fewer than 3
/// neighbors (degenerate covariance) get the zero vector.
pub fn estimate_normals(cloud: &mut PointCloud, k: usize, viewpoint: [f64; 3]) {
    let n = cloud.positions.len();
    if n == 0 {
        return;
    }
    let tree = KdTree::<3>::build(&cloud.positions);
    let mut normals = vec![[0.0f32; 3]; n];
    for (i, &p) in cloud.positions.iter().enumerate() {
        let Some((_, vecs)) = local_pca(&cloud.positions, &tree, i, k) else { continue };
        let mut normal = [vecs.get(0, 0), vecs.get(1, 0), vecs.get(2, 0)];
        let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if len > 1e-12 {
            for a in normal.iter_mut() {
                *a /= len;
            }
        }
        let to_view = [viewpoint[0] - p[0], viewpoint[1] - p[1], viewpoint[2] - p[2]];
        let dot = normal[0] * to_view[0] + normal[1] * to_view[1] + normal[2] * to_view[2];
        if dot < 0.0 {
            for a in normal.iter_mut() {
                *a = -*a;
            }
        }
        normals[i] = [normal[0] as f32, normal[1] as f32, normal[2] as f32];
    }
    cloud.normals = normals;
}

/// 🧊️ Voxel-grid downsample: buckets points into cells of edge length `cell`, averaging
/// position/normal (renormalized)/color/confidence per occupied cell. Output is sorted by cell key
/// for determinism, and never exceeds the input point count.
pub fn voxel_downsample(cloud: &PointCloud, cell: f64) -> PointCloud {
    if cell <= 0.0 || cloud.is_empty() {
        return PointCloud::new();
    }
    let (has_normals, has_colors, has_confidence) = (!cloud.normals.is_empty(), !cloud.colors.is_empty(), !cloud.confidence.is_empty());

    #[derive(Default)]
    struct Accum {
        count: usize,
        pos_sum: [f64; 3],
        normal_sum: [f64; 3],
        color_sum: [f64; 3],
        confidence_sum: f32,
    }

    let mut buckets: HashMap<(i64, i64, i64), Accum> = HashMap::new();
    for (i, &p) in cloud.positions.iter().enumerate() {
        let key = ((p[0] / cell).floor() as i64, (p[1] / cell).floor() as i64, (p[2] / cell).floor() as i64);
        let entry = buckets.entry(key).or_default();
        entry.count += 1;
        for (s, v) in entry.pos_sum.iter_mut().zip(p.iter()) {
            *s += v;
        }
        if has_normals {
            for (s, v) in entry.normal_sum.iter_mut().zip(to_f64_3(cloud.normals[i]).iter()) {
                *s += v;
            }
        }
        if has_colors {
            for (s, v) in entry.color_sum.iter_mut().zip(cloud.colors[i].iter()) {
                *s += f64::from(*v);
            }
        }
        if has_confidence {
            entry.confidence_sum += cloud.confidence[i];
        }
    }

    let mut keys: Vec<(i64, i64, i64)> = buckets.keys().copied().collect();
    keys.sort_unstable();
    let mut positions = Vec::with_capacity(keys.len());
    let mut normals = Vec::with_capacity(if has_normals { keys.len() } else { 0 });
    let mut colors = Vec::with_capacity(if has_colors { keys.len() } else { 0 });
    let mut confidence = Vec::with_capacity(if has_confidence { keys.len() } else { 0 });
    for key in keys {
        let acc = &buckets[&key];
        let inv = 1.0 / acc.count as f64;
        positions.push([acc.pos_sum[0] * inv, acc.pos_sum[1] * inv, acc.pos_sum[2] * inv]);
        if has_normals {
            let mut nv = [acc.normal_sum[0] * inv, acc.normal_sum[1] * inv, acc.normal_sum[2] * inv];
            let len = (nv[0] * nv[0] + nv[1] * nv[1] + nv[2] * nv[2]).sqrt();
            if len > 1e-12 {
                for a in nv.iter_mut() {
                    *a /= len;
                }
            }
            normals.push([nv[0] as f32, nv[1] as f32, nv[2] as f32]);
        }
        if has_colors {
            let sum = acc.color_sum;
            colors.push([(sum[0] * inv).round().clamp(0.0, 255.0) as u8, (sum[1] * inv).round().clamp(0.0, 255.0) as u8, (sum[2] * inv).round().clamp(0.0, 255.0) as u8]);
        }
        if has_confidence {
            confidence.push(acc.confidence_sum / acc.count as f32);
        }
    }
    PointCloud { positions, normals, colors, confidence, classification: Vec::new() }
}

/// 🚮️ Removes points whose mean distance to their `k` nearest neighbors exceeds `global_mean +
/// std_ratio * global_std` over the whole cloud.
pub fn statistical_outlier_removal(cloud: &PointCloud, k: usize, std_ratio: f64) -> PointCloud {
    let n = cloud.positions.len();
    if n == 0 || k == 0 {
        return cloud.clone();
    }
    let tree = KdTree::<3>::build(&cloud.positions);
    let mean_dists: Vec<f64> = cloud
        .positions
        .iter()
        .map(|p| {
            let neighbors = tree.k_nearest(p, k + 1);
            let others: Vec<f64> = neighbors.iter().filter(|&&(_, d2)| d2 > 1e-15).map(|&(_, d2)| d2.sqrt()).collect();
            if others.is_empty() {
                0.0
            } else {
                others.iter().sum::<f64>() / others.len() as f64
            }
        })
        .collect();
    let global_mean = mean_dists.iter().sum::<f64>() / n as f64;
    let variance = mean_dists.iter().map(|d| (d - global_mean).powi(2)).sum::<f64>() / n as f64;
    let threshold = global_mean + std_ratio * variance.sqrt();
    let keep: Vec<usize> = (0..n).filter(|&i| mean_dists[i] <= threshold).collect();
    keep_indices(cloud, &keep)
}

/// 🚮️ Removes points with fewer than `min_neighbors` other points within `radius`.
pub fn radius_outlier_removal(cloud: &PointCloud, radius: f64, min_neighbors: usize) -> PointCloud {
    let n = cloud.positions.len();
    if n == 0 {
        return cloud.clone();
    }
    let tree = KdTree::<3>::build(&cloud.positions);
    let keep: Vec<usize> = (0..n).filter(|&i| tree.radius(&cloud.positions[i], radius).len().saturating_sub(1) >= min_neighbors).collect();
    keep_indices(cloud, &keep)
}
// #endregion 🔖️CloudOperations

// #region 🔖️Classify
/// 🏔️ Morphological min (`erosion = true`) or max (`erosion = false`) filter over a sparse
/// `(cell_x, cell_y) -> z` grid, with a `(2 window + 1)²` neighborhood.
fn morphological_pass(grid: &HashMap<(i64, i64), f64>, window: i64, erosion: bool) -> HashMap<(i64, i64), f64> {
    let mut out = HashMap::new();
    for &(kx, ky) in grid.keys() {
        let mut best: Option<f64> = None;
        for dx in -window..=window {
            for dy in -window..=window {
                if let Some(&z) = grid.get(&(kx + dx, ky + dy)) {
                    best = Some(match best {
                        None => z,
                        Some(b) if erosion => b.min(z),
                        Some(b) => b.max(z),
                    });
                }
            }
        }
        if let Some(b) = best {
            out.insert((kx, ky), b);
        }
    }
    out
}

/// 🏔️ Simplified progressive morphological filter (PMF) ground classification: builds a min-`z`
/// grid at `cell` resolution, then for growing window sizes `1..=max_iterations` applies a
/// morphological opening (erosion then dilation) to approximate the evolving ground surface and
/// drops a point from ground consideration once its height above the opened surface exceeds
/// `max_slope * window * cell`. This is a scope-reduced variant of Zhang et al.'s PMF: it works
/// directly on a per-cell min-`z` proxy rather than tracking the full point-to-surface geometry,
/// and it does not implement the paper's `dh0`/slope-decay elevation-difference schedule. Points
/// surviving every iteration are [`PointClass::Ground`]; every other point is left
/// [`PointClass::Unclassified`] — this function does not attempt the Building/Vegetation split
/// itself (see [`classify_building_vegetation`] to further split the remainder, or
/// [`classify_points`] for both stages composed).
pub fn classify_ground_pmf(cloud: &PointCloud, cell: f64, max_slope: f64, max_iterations: u32) -> Vec<PointClass> {
    let n = cloud.positions.len();
    let mut labels = vec![PointClass::Unclassified; n];
    if n == 0 || cell <= 0.0 {
        return labels;
    }
    let (mut min_x, mut min_y) = (cloud.positions[0][0], cloud.positions[0][1]);
    for p in &cloud.positions {
        min_x = min_x.min(p[0]);
        min_y = min_y.min(p[1]);
    }
    let cell_of = |p: &[f64; 3]| -> (i64, i64) { (((p[0] - min_x) / cell).floor() as i64, ((p[1] - min_y) / cell).floor() as i64) };

    let mut grid: HashMap<(i64, i64), f64> = HashMap::new();
    for p in &cloud.positions {
        grid.entry(cell_of(p)).and_modify(|z| *z = z.min(p[2])).or_insert(p[2]);
    }

    let mut surface = grid;
    let mut ground_mask = vec![true; n];
    for iter in 1..=max_iterations {
        let window = i64::from(iter);
        let eroded = morphological_pass(&surface, window, true);
        let opened = morphological_pass(&eroded, window, false);
        let threshold = max_slope * (window as f64) * cell;
        for (i, p) in cloud.positions.iter().enumerate() {
            if !ground_mask[i] {
                continue;
            }
            if let Some(&surf_z) = opened.get(&cell_of(p)) {
                if p[2] - surf_z > threshold {
                    ground_mask[i] = false;
                }
            }
        }
        surface = opened;
    }
    for (label, &is_ground) in labels.iter_mut().zip(ground_mask.iter()) {
        if is_ground {
            *label = PointClass::Ground;
        }
    }
    labels
}

/// 🏢️🌳️ λ-ratio planarity/roughness split of already-non-ground points into [`PointClass::Building`]
/// (planar, low roughness) vs [`PointClass::Vegetation`] (high roughness), via the same local-PCA
/// eigenvalues as [`estimate_normals`] ([`local_pca`]) — dimensionality features standard in LiDAR
/// point classification <https://doi.org/10.5194/isprsannals-II-3-181-2014>: with ascending
/// eigenvalues `e0 <= e1 <= e2`, `planarity = (e1 - e0) / e2` (near `1` for a flat local patch, near
/// `0` for scattered/volumetric returns like foliage). Mutates only entries currently
/// [`PointClass::Unclassified`] in `labels` (leaves [`PointClass::Ground`] and any other label
/// untouched); points with fewer than 3 neighbors or a degenerate (near-zero) largest eigenvalue are
/// left as they were.
pub fn classify_building_vegetation(cloud: &PointCloud, labels: &mut [PointClass], k: usize, planarity_threshold: f64) {
    let n = cloud.positions.len();
    if n == 0 || labels.len() != n {
        return;
    }
    let tree = KdTree::<3>::build(&cloud.positions);
    for (i, label) in labels.iter_mut().enumerate().take(n) {
        if !matches!(label, PointClass::Unclassified) {
            continue;
        }
        let Some((vals, _)) = local_pca(&cloud.positions, &tree, i, k) else { continue };
        let (e0, e1, e2) = (vals[0], vals[1], vals[2]);
        if e2 < 1e-12 {
            continue;
        }
        let planarity = (e1 - e0) / e2;
        *label = if planarity >= planarity_threshold { PointClass::Building } else { PointClass::Vegetation };
    }
}

/// 🏔️🏢️🌳️ Full point classification pipeline: [`classify_ground_pmf`] first, then
/// [`classify_building_vegetation`] splits the remaining [`PointClass::Unclassified`] points into
/// [`PointClass::Building`] vs [`PointClass::Vegetation`] via local planarity/roughness.
pub fn classify_points(cloud: &PointCloud, cell: f64, max_slope: f64, max_iterations: u32, k: usize, planarity_threshold: f64) -> Vec<PointClass> {
    let mut labels = classify_ground_pmf(cloud, cell, max_slope, max_iterations);
    classify_building_vegetation(cloud, &mut labels, k, planarity_threshold);
    labels
}

/// 🪧️ One region-grown planar segment: its member point indices (sorted ascending), area-weighted
/// mean normal, and centroid.
#[derive(Clone, Debug, PartialEq)]
pub struct PlaneSegment {
    pub point_indices: Vec<usize>,
    pub normal: [f64; 3],
    pub centroid: [f64; 3],
}

fn normal_angle_ok_f64(n0: [f64; 3], n1: [f64; 3], cos_thresh: f64) -> bool {
    let len0 = (n0[0] * n0[0] + n0[1] * n0[1] + n0[2] * n0[2]).sqrt();
    let len1 = (n1[0] * n1[0] + n1[1] * n1[1] + n1[2] * n1[2]).sqrt();
    if len0 < 1e-12 || len1 < 1e-12 {
        return false;
    }
    let dot = (n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2]) / (len0 * len1);
    dot.abs() >= cos_thresh
}

/// 🪧️ Region-grows connected (via each point's 8 nearest [`KdTree<3>`] neighbors) planar segments:
/// starting from any unvisited point, absorbs neighbors whose normal agrees with the current
/// point's within `angle_tol_deg`. Requires [`PointCloud::normals`] to already be populated (e.g.
/// via [`estimate_normals`]); returns an empty `Vec` otherwise. Segments below `min_segment_size`
/// are dropped; the rest are returned largest-first. A tractable simplification of full
/// building/vegetation separation: it groups by local normal agreement alone, with no planarity
/// (residual-to-plane) check, so a smoothly curved surface can still form one "segment".
pub fn region_grow_planes(cloud: &PointCloud, angle_tol_deg: f64, min_segment_size: usize) -> Vec<PlaneSegment> {
    if cloud.normals.is_empty() {
        return Vec::new();
    }
    let normals = &cloud.normals;
    let n = cloud.positions.len();
    if n == 0 {
        return Vec::new();
    }
    let tree = KdTree::<3>::build(&cloud.positions);
    let cos_thresh = angle_tol_deg.to_radians().cos();
    let mut visited = vec![false; n];
    let mut segments = Vec::new();

    for seed in 0..n {
        if visited[seed] {
            continue;
        }
        visited[seed] = true;
        let mut stack = vec![seed];
        let mut members = vec![seed];
        while let Some(cur) = stack.pop() {
            for (nid, _) in tree.k_nearest(&cloud.positions[cur], 9) {
                let ni = nid as usize;
                if visited[ni] {
                    continue;
                }
                if normal_angle_ok_f64(to_f64_3(normals[cur]), to_f64_3(normals[ni]), cos_thresh) {
                    visited[ni] = true;
                    stack.push(ni);
                    members.push(ni);
                }
            }
        }
        if members.len() >= min_segment_size {
            let mut centroid = [0.0; 3];
            let mut normal = [0.0; 3];
            for &m in &members {
                for (c, v) in centroid.iter_mut().zip(cloud.positions[m].iter()) {
                    *c += v;
                }
                for (nn, v) in normal.iter_mut().zip(to_f64_3(normals[m]).iter()) {
                    *nn += v;
                }
            }
            let inv = 1.0 / members.len() as f64;
            for c in centroid.iter_mut() {
                *c *= inv;
            }
            let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
            if len > 1e-12 {
                for nn in normal.iter_mut() {
                    *nn /= len;
                }
            }
            members.sort_unstable();
            segments.push(PlaneSegment { point_indices: members, normal, centroid });
        }
    }
    segments.sort_by(|a, b| b.point_indices.len().cmp(&a.point_indices.len()).then_with(|| a.point_indices[0].cmp(&b.point_indices[0])));
    segments
}
// #endregion 🔖️Classify

// #region 🔖️Change
/// 📏️ Per-point nearest-neighbor Euclidean distance from every point of `a` to the closest point of
/// `b`, via a [`KdTree<3>`] built over `b`; `f64::INFINITY` for every point of `a` when `b` is
/// empty.
pub fn cloud_to_cloud_distance(a: &PointCloud, b: &PointCloud) -> Vec<f64> {
    if b.is_empty() {
        return vec![f64::INFINITY; a.len()];
    }
    let tree = KdTree::<3>::build(&b.positions);
    a.positions.iter().map(|p| tree.nearest(p).map_or(f64::INFINITY, |(_, d2)| d2.sqrt())).collect()
}

/// 📏️ M3C2-style signed distance along each `a`-point's normal: gathers neighbors of both `a` and
/// `b` within a cylinder of half-length `normal_scale / 2` and radius `cyl_radius` around the axis
/// through the point along its normal (found via a [`KdTree<3>`] radius query bounding the
/// cylinder, then filtered by the exact axial/radial split), projects each onto the normal, and
/// returns `mean(b_axial) - mean(a_axial)`. `None` when the point has no normal or either side has
/// fewer than 3 in-cylinder neighbors. Simplified relative to the full M3C2 algorithm: it neither
/// re-estimates a local normal from the cylinder's own points nor propagates a registration-error /
/// roughness-based precision (`LODetection`) alongside the distance — this returns the raw signed
/// mean-projection difference only.
pub fn m3c2_distance(a: &PointCloud, b: &PointCloud, normal_scale: f64, cyl_radius: f64) -> Vec<Option<f64>> {
    if a.normals.is_empty() {
        return vec![None; a.len()];
    }
    let normals = &a.normals;
    if a.is_empty() {
        return Vec::new();
    }
    const MIN_NEIGHBORS: usize = 3;
    let half_len = normal_scale * 0.5;
    let search_radius = (half_len * half_len + cyl_radius * cyl_radius).sqrt();
    let tree_a = KdTree::<3>::build(&a.positions);
    let tree_b = (!b.is_empty()).then(|| KdTree::<3>::build(&b.positions));

    let collect_axial = |positions: &[[f64; 3]], hits: &[(u32, f64)], p: [f64; 3], unit_n: [f64; 3]| -> Vec<f64> {
        hits.iter()
            .filter_map(|&(id, _)| {
                let q = positions[id as usize];
                let d = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
                let t = d[0] * unit_n[0] + d[1] * unit_n[1] + d[2] * unit_n[2];
                if t.abs() > half_len {
                    return None;
                }
                let r2 = d[0] * d[0] + d[1] * d[1] + d[2] * d[2] - t * t;
                if r2 > cyl_radius * cyl_radius {
                    None
                } else {
                    Some(t)
                }
            })
            .collect()
    };

    a.positions
        .iter()
        .enumerate()
        .map(|(i, &p)| {
            let n = to_f64_3(normals[i]);
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if len < 1e-12 {
                return None;
            }
            let unit_n = [n[0] / len, n[1] / len, n[2] / len];
            let axial_a = collect_axial(&a.positions, &tree_a.radius(&p, search_radius), p, unit_n);
            if axial_a.len() < MIN_NEIGHBORS {
                return None;
            }
            let tree_b = tree_b.as_ref()?;
            let axial_b = collect_axial(&b.positions, &tree_b.radius(&p, search_radius), p, unit_n);
            if axial_b.len() < MIN_NEIGHBORS {
                return None;
            }
            let mean_a = axial_a.iter().sum::<f64>() / axial_a.len() as f64;
            let mean_b = axial_b.iter().sum::<f64>() / axial_b.len() as f64;
            Some(mean_b - mean_a)
        })
        .collect()
}
// #endregion 🔖️Change

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
