//! 🧊️ Shell material metrics, statistics, the principal frame and the `GltfGeometryContext` aggregate.

use std::{collections::BTreeMap, collections::BTreeSet};
use crate::standards::v2_0::subsets::any::schema::modules::mesh_topology::Topology;
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::*;

//#region 🧮️Kernel
fn shell_material_metrics(points: &[V3], faces: &[[usize; 3]], edge_faces: &BTreeMap<(usize, usize), Vec<(usize, bool)>>, tolerance: f64, budget: usize) -> Option<(f64, f64, f64, V3)> {
    let mut neighbors = vec![BTreeSet::new(); faces.len()];
    for incidences in edge_faces.values().filter(|incidences| incidences.len() == 2) {
        neighbors[incidences[0].0].insert(incidences[1].0);
        neighbors[incidences[1].0].insert(incidences[0].0);
    }
    let mut seen = vec![false; faces.len()];
    let mut components = Vec::<Vec<usize>>::new();
    for seed in 0..faces.len() {
        if seen[seed] {
            continue;
        }
        let mut stack = vec![seed];
        seen[seed] = true;
        let mut component = Vec::new();
        while let Some(face) = stack.pop() {
            component.push(face);
            for neighbor in &neighbors[face] {
                if !seen[*neighbor] {
                    seen[*neighbor] = true;
                    stack.push(*neighbor);
                }
            }
        }
        components.push(component);
    }
    let mut shells = Vec::<(f64, V3, V3, Vec<[usize; 3]>)>::new();
    for component in components {
        let shell_faces = component.iter().map(|index| faces[*index]).collect::<Vec<_>>();
        let mut signed_volume = 0.0;
        let mut weighted_centroid = [0.0; 3];
        for face in &shell_faces {
            let (a, b, c) = (points[face[0]], points[face[1]], points[face[2]]);
            let tetrahedron = dot(a, cross(b, c)) / 6.0;
            signed_volume += tetrahedron;
            weighted_centroid = add(weighted_centroid, mul(add(add(a, b), c), tetrahedron / 4.0));
        }
        if signed_volume.abs() <= tolerance.powi(3) {
            return None;
        }
        let centroid = mul(weighted_centroid, 1.0 / signed_volume);
        let mut representative = None;
        for face in shell_faces.iter().take(8) {
            let (a, b, c) = (points[face[0]], points[face[1]], points[face[2]]);
            let face_centroid = mul(add(add(a, b), c), 1.0 / 3.0);
            let normal = normalize(cross(sub(b, a), sub(c, a)));
            for candidate in [add(face_centroid, mul(normal, tolerance * 8.0)), sub(face_centroid, mul(normal, tolerance * 8.0))] {
                if point_in_closed_mesh(candidate, points, &shell_faces, tolerance) == Some(true) {
                    representative = Some(candidate);
                    break;
                }
            }
            if representative.is_some() {
                break;
            }
        }
        shells.push((signed_volume.abs(), centroid, representative?, shell_faces));
    }
    if shells.len().saturating_mul(faces.len()).saturating_mul(3) > budget.max(1) {
        return None;
    }
    let mut intersection_budget = budget.max(1);
    for i in 0..shells.len() {
        for j in i + 1..shells.len() {
            for first in &shells[i].3 {
                for second in &shells[j].3 {
                    if intersection_budget == 0 {
                        return None;
                    }
                    intersection_budget -= 1;
                    let first = [points[first[0]], points[first[1]], points[first[2]]];
                    let second = [points[second[0]], points[second[1]], points[second[2]]];
                    if triangle_distance(first, second) <= tolerance {
                        return None;
                    }
                }
            }
        }
    }
    let mut enclosed = 0.0;
    let mut material = 0.0;
    let mut material_centroid = [0.0; 3];
    for i in 0..shells.len() {
        let depth = (0..shells.len()).filter(|other| *other != i).filter(|other| point_in_closed_mesh(shells[i].2, points, &shells[*other].3, tolerance) == Some(true)).count();
        let sign = if depth % 2 == 0 { 1.0 } else { -1.0 };
        if depth == 0 {
            enclosed += shells[i].0;
        }
        material += sign * shells[i].0;
        material_centroid = add(material_centroid, mul(shells[i].1, sign * shells[i].0));
    }
    if material <= tolerance.powi(3) {
        return None;
    }
    Some((material, enclosed, (enclosed - material).max(0.0), mul(material_centroid, 1.0 / material)))
}

pub fn statistics(values: &[f64], edges: &[f64]) -> GltfStatistics {
    if values.is_empty() {
        return GltfStatistics::default();
    }
    let mut sorted: Vec<f64> = values.iter().copied().filter(|x| x.is_finite()).collect();
    if sorted.is_empty() {
        return GltfStatistics::default();
    }
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    let mean = sorted.iter().sum::<f64>() / n as f64;
    let variance = sorted.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / n as f64;
    let q = |p: f64| sorted[((n - 1) as f64 * p).round() as usize];
    let mut counts = vec![0u64; edges.len().saturating_sub(1)];
    for v in &sorted {
        if let Some(i) = edges.windows(2).position(|e| *v >= e[0] && *v < e[1]) {
            counts[i] += 1;
        } else if *v == *edges.last().unwrap_or(v) && !counts.is_empty() {
            *counts.last_mut().unwrap() += 1;
        }
    }
    GltfStatistics {
        minimum: sorted.first().copied(),
        maximum: sorted.last().copied(),
        mean: Some(mean),
        variance: Some(variance),
        standard_deviation: Some(variance.sqrt()),
        median: Some(q(0.5)),
        quantiles: vec![q(0.0), q(0.25), q(0.5), q(0.75), q(1.0)],
        histogram: Some(GltfHistogram { edges: edges.to_vec(), counts, weights: Vec::new() }),
    }
}

pub(crate) fn principal_frame(points: &[V3], centroid: V3) -> GltfPrincipalFrame {
    let mut a = [[0.0; 3]; 3];
    for p in points {
        let d = sub(*p, centroid);
        for i in 0..3 {
            for j in 0..3 {
                a[i][j] += d[i] * d[j] / points.len().max(1) as f64;
            }
        }
    }
    let mut v = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    for _ in 0..32 {
        let mut p = 0;
        let mut q = 1;
        for i in 0..3 {
            for j in i + 1..3 {
                if a[i][j].abs() > a[p][q].abs() {
                    p = i;
                    q = j;
                }
            }
        }
        if a[p][q].abs() < 1e-15 {
            break;
        }
        let phi = 0.5 * (2.0 * a[p][q]).atan2(a[q][q] - a[p][p]);
        let (c, s) = (phi.cos(), phi.sin());
        for row in &mut a {
            let (ap, aq) = (row[p], row[q]);
            row[p] = c * ap - s * aq;
            row[q] = s * ap + c * aq;
        }
        for k in 0..3 {
            let (ap, aq) = (a[p][k], a[q][k]);
            a[p][k] = c * ap - s * aq;
            a[q][k] = s * ap + c * aq;
            let (vp, vq) = (v[k][p], v[k][q]);
            v[k][p] = c * vp - s * vq;
            v[k][q] = s * vp + c * vq;
        }
    }
    let mut e = [(a[0][0], [v[0][0], v[1][0], v[2][0]]), (a[1][1], [v[0][1], v[1][1], v[2][1]]), (a[2][2], [v[0][2], v[1][2], v[2][2]])];
    e.sort_by(|x, y| y.0.total_cmp(&x.0));
    for x in &mut e {
        let axis = &mut x.1;
        let k = (0..3).max_by(|i, j| axis[*i].abs().total_cmp(&axis[*j].abs())).unwrap();
        if axis[k] < 0.0 {
            *axis = mul(*axis, -1.0);
        }
    }
    GltfPrincipalFrame { centroid: GltfVec3::new(centroid), axes: e.map(|x| GltfVec3::new(normalize(x.1))), eigenvalues: e.map(|x| x.0.max(0.0)) }
}

/// 🧰️ Canonical shared geometry consumed by independent inference stages.
pub struct GltfGeometryContext<'a> {
    pub policy: &'a GltfAnalysisPolicy,
    pub topology: Topology,
    pub points: Vec<V3>,
    pub faces: Vec<[usize; 3]>,
    pub edge_faces: BTreeMap<(usize, usize), Vec<(usize, bool)>>,
    pub sample_count: usize,
    pub bounds: GltfBounds3,
    pub dimensions: V3,
    pub diagonal: f64,
    pub surface_area: f64,
    pub solid: Option<(f64, f64, f64, V3)>,
    pub volume: f64,
    pub centroid: V3,
    pub principal_frame: GltfPrincipalFrame,
    pub principal_axes: Vec<GltfDirectionScore>,
    pub oriented_bounds: GltfBounds3,
    pub oriented_extent: V3,
    pub unavailable_volume: GltfAvailability,
}

impl<'a> GltfGeometryContext<'a> {
    pub fn new(points: &[V3], triangles: &[[usize; 3]], policy: &'a GltfAnalysisPolicy) -> Option<Self> {
        let (topology, points, faces, edge_faces) = topology(points, triangles);
        let sample_count = points.len();
        let (lo, hi, dimensions) = bounds(&points)?;
        let bounds = GltfBounds3 { min: GltfVec3::new(lo), max: GltfVec3::new(hi), dimensions: GltfVec3::new(dimensions) };
        let diagonal = norm(dimensions);
        let surface_area = faces.iter().map(|face| triangle_area(points[face[0]], points[face[1]], points[face[2]])).sum::<f64>();
        let mut surface_centroid = [0.0; 3];
        for face in &faces {
            let (a, b, c) = (points[face[0]], points[face[1]], points[face[2]]);
            let area = triangle_area(a, b, c);
            surface_centroid = add(surface_centroid, mul(add(add(a, b), c), area / 3.0));
        }
        if surface_area > 0.0 {
            surface_centroid = mul(surface_centroid, 1.0 / surface_area);
        } else {
            surface_centroid = mul(points.iter().fold([0.0; 3], |sum, point| add(sum, *point)), 1.0 / sample_count.max(1) as f64);
        }
        let tolerance = (diagonal * policy.relative_tolerance).max(policy.absolute_length_tolerance);
        let solid = if topology.watertight && topology.manifold && topology.oriented { shell_material_metrics(&points, &faces, &edge_faces, tolerance, policy.sampling_budget as usize) } else { None };
        let volume = solid.map_or(0.0, |metrics| metrics.0);
        let centroid = solid.map_or(surface_centroid, |metrics| metrics.3);
        let principal_frame = principal_frame(&points, centroid);
        let principal_axes = principal_frame.axes.iter().enumerate().map(|(index, axis)| GltfDirectionScore { direction: *axis, score: principal_frame.eigenvalues[index], order: Some((index + 1) as u32) }).collect::<Vec<_>>();
        let mut oriented_extent = [0.0; 3];
        let mut oriented_min = [0.0; 3];
        let mut oriented_max = [0.0; 3];
        for (index, axis) in principal_frame.axes.iter().enumerate() {
            let axis = axis.array();
            let mut minimum = f64::INFINITY;
            let mut maximum = f64::NEG_INFINITY;
            for point in &points {
                let projection = dot(sub(*point, centroid), axis);
                minimum = minimum.min(projection);
                maximum = maximum.max(projection);
            }
            oriented_min[index] = minimum;
            oriented_max[index] = maximum;
            oriented_extent[index] = maximum - minimum;
        }
        let oriented_bounds = GltfBounds3 { min: GltfVec3::new(oriented_min), max: GltfVec3::new(oriented_max), dimensions: GltfVec3::new(oriented_extent) };
        let unavailable_volume = if !topology.manifold {
            GltfAvailability::NonManifold
        } else if !topology.watertight {
            GltfAvailability::OpenSurface
        } else {
            GltfAvailability::InvalidInput
        };
        Some(Self { policy, topology, points, faces, edge_faces, sample_count, bounds, dimensions, diagonal, surface_area, solid, volume, centroid, principal_frame, principal_axes, oriented_bounds, oriented_extent, unavailable_volume })
    }
}
//#endregion 🧮️Kernel
