//! ☁️ Point-cloud analysis of the dense reconstruction: normals, downsampling, outlier removal, ground and plane classification and the cloud change distances. See [`super`].

use super::*;

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
