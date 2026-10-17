//! 🧊️ Depth fusion and TSDF volumes of the dense reconstruction: the cross-view fusion, the volume and its resumable integration. See [`super`].

use super::*;

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
    let to_worlds: Vec<semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3> = views.iter().map(|(pose, _)| pose.0.inverse()).collect();
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
#[derive(semio_framework_value::RetireOwned)]
pub struct FusionPreparation {
    view: usize,
    pixel: usize,
    comparison: usize,
    pub(super) point_world: Option<[f64; 3]>,
    agreeing_sum: [f64; 3],
    pub(super) agreeing_count: usize,
    source_normal: [f32; 3],
    source_confidence: f32,
    pub(super) output: PointCloud,
    /// ✅️ Per view, one bit per pixel: set when that pixel's point reached `min_consistent_views`
    /// agreeing views. Every pixel is judged whatever the output cap, so a TSDF integration can keep
    /// exactly the multi-view-consistent depths (see [`TsdfIntegrationPreparation::advance_masked`]).
    consistent: Vec<Vec<u64>>,
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
            consistent: Vec::new(),
            complete: false,
        }
    }

    pub(super) fn finish_pixel(&mut self, minimum: usize) {
        if self.agreeing_count >= minimum {
            self.mark_consistent();
        }
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

    fn mark_consistent(&mut self) {
        if let Some(bits) = self.consistent.get_mut(self.view) {
            if let Some(word) = bits.get_mut(self.pixel / 64) {
                *word |= 1 << (self.pixel % 64);
            }
        }
    }

    pub fn advance(&mut self, views: &[(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)], depth_maps: &[DepthMap], cfg: &FusionConfig, comparison_budget: usize) -> bool {
        let mut remaining = comparison_budget.max(1);
        if self.consistent.is_empty() {
            self.consistent = depth_maps.iter().map(|map| vec![0; map.depth.len().div_ceil(64)]).collect();
        }
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

    /// ✅️ [`Self::finish`] plus the per-view consistency bitsets (one bit per pixel, row-major).
    pub fn finish_with_consistency(self) -> Option<(PointCloud, Vec<Vec<u64>>)> {
        self.complete.then_some((self.output, self.consistent))
    }
}
// #endregion 🔖️Fusion

// #region 🔖️Tsdf
const TSDF_BLOCK_DIM: i32 = 8;

const TSDF_BLOCK_VOXELS: usize = (TSDF_BLOCK_DIM * TSDF_BLOCK_DIM * TSDF_BLOCK_DIM) as usize;

const TSDF_MAX_WEIGHT: f32 = 100.0;

/// 🧊️ One `8x8x8` chunk of a [`TsdfVolume`]'s hashed sparse grid: per-voxel signed distance and
/// accumulated integration weight, row-major within the block (`(lz * 8 + ly) * 8 + lx`).
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
struct TsdfBlock {
    pub(super) sdf: Vec<f32>,
    weight: Vec<f32>,
}

impl TsdfBlock {
    pub(super) fn new() -> Self {
        Self { sdf: vec![0.0; TSDF_BLOCK_VOXELS], weight: vec![0.0; TSDF_BLOCK_VOXELS] }
    }

    fn local_index(local: [i32; 3]) -> usize {
        ((local[2] * TSDF_BLOCK_DIM + local[1]) * TSDF_BLOCK_DIM + local[0]) as usize
    }
}

/// 🧊️ Truncated signed distance field over a hashed grid of `8x8x8` voxel blocks (Curless-Levoy
/// weighted integration), keyed by block coordinate so only blocks actually touched by an
/// integrated depth map ever get allocated.
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub struct TsdfVolume {
    pub voxel_size: f64,
    pub truncation: f64,
    blocks: BTreeMap<[i32; 3], TsdfBlock>,
    /// 🧱️ Optional inclusive global-voxel box outside which [`Self::integrate`] allocates nothing.
    /// `None` (the default) integrates wherever a ray lands, which is only safe when the caller
    /// already knows the depth range is scene-scaled: one 4 KiB block is allocated for every touched
    /// `8³` neighbourhood, so a depth hypothesis range spanning orders of magnitude more than the
    /// scene (PatchMatch initialises in `[depth_min, depth_max]`) scatters hundreds of thousands of
    /// blocks over empty space. A caller that knows which voxels will ever be *read* — a surface
    /// extraction is always run over an explicit `[bounds_min, bounds_max]` lattice — sets the box and
    /// keeps every sample the extraction can see while allocating nothing beyond it.
    bounds: Option<([i32; 3], [i32; 3])>,
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
        Self { voxel_size, truncation, blocks: BTreeMap::new(), bounds: None }
    }

    /// 🧱️ The same volume restricted to an inclusive global-voxel box: integration outside it is
    /// dropped instead of allocating a block. Pass a box that *contains every voxel a later surface
    /// extraction samples* (a surface-net extraction over `[min, max]` reads one voxel past each face,
    /// so pad by at least one) and the retained grid shrinks to the meshed neighbourhood without any
    /// sampled value changing. See [`Self::bounds`].
    pub fn with_voxel_bounds(mut self, minimum: [i32; 3], maximum: [i32; 3]) -> Self {
        self.bounds = Some((minimum, maximum));
        self
    }

    /// 🧱️ The integration box, if this volume is bounded.
    pub fn voxel_bounds(&self) -> Option<([i32; 3], [i32; 3])> {
        self.bounds
    }

    fn voxel_coord(&self, p: [f64; 3]) -> [i32; 3] {
        p.map(|c| (c / self.voxel_size).floor() as i32)
    }

    /// 🧱️ Whether this global voxel coordinate is inside the integration box (always, when unbounded).
    fn admits_voxel(&self, voxel: [i32; 3]) -> bool {
        self.bounds.is_none_or(|(minimum, maximum)| (0..3).all(|axis| voxel[axis] >= minimum[axis] && voxel[axis] <= maximum[axis]))
    }

    fn integrate_point(&mut self, p: [f64; 3], sdf: f32, weight: f32) {
        let voxel = self.voxel_coord(p);
        if !self.admits_voxel(voxel) {
            return;
        }
        let (block_coord, local) = tsdf_block_and_local(voxel);
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

    /// 🔢️ How many `8x8x8` blocks are currently allocated — the volume's whole retained footprint is
    /// this count times [`TSDF_BLOCK_RETAINED_BYTES`], so a caller (or a memory probe) can see the
    /// sparse grid's real cost without walking it.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// 🧮️ Retained heap bytes of the hashed grid: one dense `sdf`/`weight` pair per allocated block.
    pub fn retained_bytes(&self) -> usize {
        self.blocks.len() * TSDF_BLOCK_RETAINED_BYTES
    }
}

/// 🧮️ Heap bytes one allocated [`TsdfBlock`] costs: two dense `f32` buffers of `8³` voxels each,
/// plus the ordered map's own node share. Any voxel a depth ray touches allocates the whole block,
/// which is why an unbounded integration extent is a memory hazard rather than a mere slowdown.
pub const TSDF_BLOCK_RETAINED_BYTES: usize = TSDF_BLOCK_VOXELS * 2 * size_of::<f32>() + 64;

/// 🧊️ Cursor for one depth-map integration. A continuation performs at most the requested number of
/// ray samples; sparse blocks use an ordered map so insertion never triggers a whole-table rehash.
#[derive(semio_framework_value::RetireOwned)]
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
        self.advance_masked(volume, depth, None, camera, weight_by_grazing_angle, sample_budget)
    }

    /// 🧊️ [`Self::advance`] integrating only the pixels whose bit is set in `mask` (one bit per
    /// pixel, row-major, as [`FusionPreparation::finish_with_consistency`] answers). A single view's
    /// PatchMatch outliers carry no less weight than its good depths, and each paints a truncation
    /// band of wrong-signed voxels wherever it lands — inside the object or in free space — which no
    /// later view's samples overwrite; the multi-view consistency mask keeps them out.
    pub fn advance_masked(&mut self, volume: &mut TsdfVolume, depth: &DepthMap, mask: Option<&[u64]>, camera: &(remodeling_camera::CameraPose, remodeling_camera::Intrinsics), weight_by_grazing_angle: bool, sample_budget: usize) -> bool {
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
            let admitted = mask.is_none_or(|bits| bits.get(self.pixel / 64).is_some_and(|word| word >> (self.pixel % 64) & 1 == 1));
            let Some(measured) = depth.get(x, y).filter(|_| admitted) else {
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
