//! 🧩️ Multi-view stereo of the dense reconstruction: PatchMatch, the plane sweep and the depth filters. See [`super`].

use super::*;

// #region 🔖️PatchMatch
/// 🎲️ SplitMix64 seeded pseudo-random generator. This crate has no dependency on a shared RNG
/// crate, so a tiny, deterministic, allocation-free generator is hand-rolled here purely to drive
/// PatchMatch's random plane initialization and per-iteration perturbation.
/// <https://prng.di.unimi.it/splitmix64.c>
struct SplitMix64(u64);

impl SplitMix64 {
    pub(super) fn new(seed: u64) -> Self {
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
    pub(super) normal: [f64; 3],
}

/// 📐️ Reference-view context threaded through the plane-induced warp helpers, bundled into one
/// struct so those helpers stay under clippy's argument-count ceiling.
struct RefContext<'a> {
    pub(super) img: &'a remodeling_image::ImageGray,
    pub(super) intr: &'a remodeling_camera::Intrinsics,
    pub(super) to_world: &'a semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3,
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
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct PatchMatchConfig {
    pub window_radius: i32,
    pub iterations: u32,
    pub depth_min: f32,
    pub depth_max: f32,
    pub seed: u64,
    /// 🔝️ Number of lowest-cost (highest-ZNCC) source views aggregated per pixel, per
    /// [`patch_zncc_cost`]; clamped to however many views produced a valid warp at that pixel.
    pub best_k: usize,
    /// 🚧️ Confidence a published pixel must exceed, or `None` to publish every pixel a source view
    /// warped into. Confidence is the aggregated ZNCC rescaled to `[0, 1]`, so `0.5` is ZNCC `0`:
    /// exactly the score of a textureless patch (zero variance), which carries no depth evidence
    /// yet otherwise publishes whatever hypothesis it was initialised with — a flat background
    /// then fuses into the surface as a shell around the object.
    pub confidence_floor: Option<f32>,
}

impl Default for PatchMatchConfig {
    fn default() -> Self {
        Self { window_radius: 3, iterations: 4, depth_min: 0.1, depth_max: 100.0, seed: 0x5EED_1234_ABCD_EF01, best_k: 3, confidence_floor: None }
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
            if costs[i] > -1.0 && publishes(costs[i], cfg.confidence_floor) {
                out.depth[i] = depths[i];
                out.normal[i] = normals[i];
                out.confidence[i] = ((costs[i] + 1.0) * 0.5).clamp(0.0, 1.0);
            }
        }
    }
    out
}

/// 🚧️ Whether a pixel whose aggregated ZNCC is `cost` clears [`PatchMatchConfig::confidence_floor`].
fn publishes(cost: f32, confidence_floor: Option<f32>) -> bool {
    confidence_floor.is_none_or(|floor| ((cost + 1.0) * 0.5).clamp(0.0, 1.0) > floor)
}

/// 🧭️ Fuel-bounded phases for the production PatchMatch path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum PatchMatchPhase {
    Allocate,
    Initialize,
    Propagate,
    Publish,
    Done,
}

/// 🌫️ Owned PatchMatch checkpoint. Each advance touches at most `pixel_budget` hypotheses; image
/// buffers are filled incrementally and the completed map is moved out without a terminal copy.
/// ⏱️ Pixels one `pixel_budget` unit covers in the phases that evaluate no matching cost
/// (buffer growth, publication): a plain per-pixel copy is thousands of times cheaper than one
/// multi-view ZNCC evaluation, so charging both one unit per pixel made the cheap phases dominate a
/// run's step count.
pub const PATCH_MATCH_BULK_PIXELS_PER_UNIT: usize = 4_096;

#[derive(semio_framework_value::RetireOwned)]
pub struct PatchMatchPreparation {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) phase: PatchMatchPhase,
    cursor: usize,
    iteration: u32,
    parity: u32,
    pub(super) depths: Vec<f32>,
    pub(super) normals: Vec<[f32; 3]>,
    pub(super) costs: Vec<f32>,
    pub(super) output: DepthMap,
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

    /// 🧮️ Retained heap bytes of this checkpoint: the three hypothesis buffers plus the output map.
    pub fn retained_bytes(&self) -> usize {
        self.depths.capacity() * size_of::<f32>()
            + self.normals.capacity() * size_of::<[f32; 3]>()
            + self.costs.capacity() * size_of::<f32>()
            + self.output.depth.capacity() * size_of::<f32>()
            + self.output.normal.capacity() * size_of::<[f32; 3]>()
            + self.output.confidence.capacity() * size_of::<f32>()
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
                let end = self.cursor.saturating_add(budget.saturating_mul(PATCH_MATCH_BULK_PIXELS_PER_UNIT)).min(pixels);
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
                let end = self.cursor.saturating_add(budget.saturating_mul(PATCH_MATCH_BULK_PIXELS_PER_UNIT)).min(pixels);
                for index in self.cursor..end {
                    if self.costs[index] > -1.0 && publishes(self.costs[index], cfg.confidence_floor) {
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
