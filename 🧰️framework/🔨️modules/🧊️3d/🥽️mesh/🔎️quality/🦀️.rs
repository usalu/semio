//! 🔎️ Pure mesh analysis: topology health, element quality and mass properties of a polygon soup,
//! callable on a [`HalfedgeMesh`] and on bare `positions` + `faces` by a consumer that owns no kernel.
//!
//! Everything is read in `f64` from the mesh's `f32` coordinates. Inertia and centroid come from the
//! divergence theorem over the fan triangulation of each face (signed tetrahedra from the origin),
//! so they are exact for a closed, consistently wound polyhedron.
//!
//! 🔗️ [Polyhedral mass properties](https://www.geometrictools.com/Documentation/PolyhedralMassProperties.pdf)

use super::HalfedgeMesh;
use crate::inertia::{principal_inertia, to_centroid, InertiaTensor, PrincipalInertia};
use std::collections::{BTreeMap, HashMap};

/// 📊 Minimum, maximum and arithmetic mean of one measured quantity.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct ScalarStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}

/// 🔲 Axis-aligned bounds of the used vertices.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct MeshBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// 🧊️ Unit-density mass properties of a closed, consistently wound mesh.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct MeshMassProperties {
    pub volume: f64,
    pub centroid: [f64; 3],
    pub inertia: [[f64; 3]; 3],
    pub principal: PrincipalInertia,
}

/// 🪞 Topology, element quality and mass report of one mesh.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct MeshQualityReport {
    pub vertex_count: usize,
    pub isolated_vertices: usize,
    pub edge_count: usize,
    pub face_count: usize,
    pub triangle_count: usize,
    pub invalid_faces: usize,
    pub boundary_edges: usize,
    pub boundary_loops: Vec<usize>,
    pub non_manifold_edges: usize,
    pub non_manifold_vertices: usize,
    pub degenerate_faces: usize,
    pub duplicate_vertices: usize,
    pub edge_length: Option<ScalarStats>,
    pub aspect_ratio: Option<ScalarStats>,
    pub connected_components: usize,
    pub closed: bool,
    pub orientable: bool,
    pub consistent_winding: bool,
    pub euler_characteristic: i64,
    pub genus: Option<i64>,
    pub area: f64,
    pub area_centroid: Option<[f64; 3]>,
    pub signed_volume: f64,
    pub bounding_box: Option<MeshBounds>,
    pub mass: Option<MeshMassProperties>,
}

impl HalfedgeMesh {
    /// 🔬 The mesh's full quality report, see [`analyze_polygon_soup`].
    pub fn quality_report(&self) -> MeshQualityReport {
        let (positions, faces) = self.polygon_soup();
        analyze_polygon_soup(&positions, &faces)
    }
}

type P3 = [f64; 3];

fn sub(a: P3, b: P3) -> P3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn add(a: P3, b: P3) -> P3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn cross(a: P3, b: P3) -> P3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn dot(a: P3, b: P3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn norm(a: P3) -> f64 {
    dot(a, a).sqrt()
}

struct UnionFind(Vec<usize>);

impl UnionFind {
    fn new(len: usize) -> Self {
        Self((0..len).collect())
    }
    fn find(&mut self, mut node: usize) -> usize {
        while self.0[node] != node {
            self.0[node] = self.0[self.0[node]];
            node = self.0[node];
        }
        node
    }
    fn union(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));
        if a != b {
            self.0[a] = b;
        }
    }
}

#[derive(Default)]
struct EdgeUse {
    forward: u32,
    backward: u32,
}

impl EdgeUse {
    fn total(&self) -> u32 {
        self.forward + self.backward
    }
}

fn stats(values: &[f64]) -> Option<ScalarStats> {
    if values.is_empty() {
        return None;
    }
    let (mut min, mut max, mut sum) = (f64::INFINITY, f64::NEG_INFINITY, 0.0);
    for &value in values {
        min = min.min(value);
        max = max.max(value);
        sum += value;
    }
    Some(ScalarStats { min, max, mean: sum / values.len() as f64 })
}

/// 🔎️ Analyses a polygon soup: `faces` lists vertex indices per face, wound counter-clockwise seen from outside.
///
/// - `aspect_ratio` is `longest edge * perimeter / (4 sqrt(3) * area)` of each non-degenerate triangle (1 for an
///   equilateral one); a polygon reports its worst fan triangle.
/// - `degenerate_faces` are faces with fewer than three distinct vertices or an area below `1e-10` of their longest
///   edge squared.
/// - `duplicate_vertices` counts vertices that sit within `1e-6` of the bounding diagonal of an earlier vertex.
/// - `mass` is present only for a closed, manifold, consistently wound mesh with non-zero volume; a negative
///   `signed_volume` (inward winding) is folded into a positive `volume`.
pub fn analyze_polygon_soup(positions: &[[f32; 3]], faces: &[Vec<u32>]) -> MeshQualityReport {
    let points: Vec<P3> = positions.iter().map(|p| [p[0] as f64, p[1] as f64, p[2] as f64]).collect();
    let mut valid: Vec<&Vec<u32>> = Vec::new();
    let mut invalid_faces = 0usize;
    for face in faces {
        if face.len() >= 3 && face.iter().all(|&index| (index as usize) < points.len()) {
            valid.push(face);
        } else {
            invalid_faces += 1;
        }
    }

    let mut used = vec![false; points.len()];
    let mut edges: BTreeMap<(u32, u32), EdgeUse> = BTreeMap::new();
    let mut degenerate_faces = 0usize;
    let mut triangle_count = 0usize;
    let mut aspect = Vec::new();
    let (mut area, mut signed_volume) = (0.0, 0.0);
    let mut area_moment = [0.0; 3];
    let mut volume_moment = [0.0; 3];
    let mut second_moment = [[0.0; 3]; 3];
    for face in &valid {
        for &index in face.iter() {
            used[index as usize] = true;
        }
        triangle_count += face.len() - 2;
        let origin = points[face[0] as usize];
        let mut vector_area = [0.0; 3];
        let mut longest = 0.0_f64;
        let mut distinct = true;
        let mut worst: Option<f64> = None;
        for corner in 0..face.len() {
            let (a, b) = (face[corner], face[(corner + 1) % face.len()]);
            if a == b {
                distinct = false;
                continue;
            }
            let entry = edges.entry((a.min(b), a.max(b))).or_default();
            if a < b {
                entry.forward += 1;
            } else {
                entry.backward += 1;
            }
            longest = longest.max(norm(sub(points[b as usize], points[a as usize])));
        }
        for corner in 1..face.len() - 1 {
            let (b, c) = (points[face[corner] as usize], points[face[corner + 1] as usize]);
            let triangle_normal = cross(sub(b, origin), sub(c, origin));
            vector_area = add(vector_area, triangle_normal);
            let tetra = dot(origin, cross(b, c)) / 6.0;
            signed_volume += tetra;
            let sum = add(add(origin, b), c);
            for axis in 0..3 {
                volume_moment[axis] += tetra * sum[axis] / 4.0;
                for other in 0..3 {
                    second_moment[axis][other] += tetra / 20.0 * (origin[axis] * origin[other] + b[axis] * b[other] + c[axis] * c[other] + sum[axis] * sum[other]);
                }
            }
            let tri_area = norm(triangle_normal) / 2.0;
            let perimeter = norm(sub(b, origin)) + norm(sub(c, b)) + norm(sub(origin, c));
            let tri_longest = norm(sub(b, origin)).max(norm(sub(c, b))).max(norm(sub(origin, c)));
            if tri_area > 0.0 {
                let ratio = tri_longest * perimeter / (4.0 * 3.0_f64.sqrt() * tri_area);
                worst = Some(worst.map_or(ratio, |w: f64| w.max(ratio)));
            }
        }
        let face_area = norm(vector_area) / 2.0;
        let degenerate = !distinct || face_area <= 1e-10 * longest * longest;
        if degenerate {
            degenerate_faces += 1;
        } else if let Some(ratio) = worst {
            aspect.push(ratio);
        }
        area += face_area;
        let centre = face.iter().fold([0.0; 3], |sum, &index| add(sum, points[index as usize]));
        for axis in 0..3 {
            area_moment[axis] += face_area * centre[axis] / face.len() as f64;
        }
    }

    let mut lengths = Vec::with_capacity(edges.len());
    for (&(a, b), _) in &edges {
        lengths.push(norm(sub(points[b as usize], points[a as usize])));
    }
    let boundary_edges = edges.values().filter(|e| e.total() == 1).count();
    let non_manifold_edges = edges.values().filter(|e| e.total() > 2).count();
    let consistent_winding = edges.values().all(|e| e.total() != 2 || (e.forward == 1 && e.backward == 1));

    let boundary_loops = {
        let mut union = UnionFind::new(points.len());
        for (&(a, b), edge) in &edges {
            if edge.total() == 1 {
                union.union(a as usize, b as usize);
            }
        }
        let mut per_loop: HashMap<usize, usize> = HashMap::new();
        for (&(a, _), edge) in &edges {
            if edge.total() == 1 {
                *per_loop.entry(union.find(a as usize)).or_default() += 1;
            }
        }
        let mut lengths: Vec<usize> = per_loop.into_values().collect();
        lengths.sort_unstable_by(|a, b| b.cmp(a));
        lengths
    };

    let orientable = {
        let mut adjacency: Vec<Vec<(usize, bool)>> = vec![Vec::new(); valid.len()];
        let mut by_edge: HashMap<(u32, u32), Vec<(usize, bool)>> = HashMap::new();
        for (face_index, face) in valid.iter().enumerate() {
            for corner in 0..face.len() {
                let (a, b) = (face[corner], face[(corner + 1) % face.len()]);
                if a != b {
                    by_edge.entry((a.min(b), a.max(b))).or_default().push((face_index, a < b));
                }
            }
        }
        for uses in by_edge.values().filter(|uses| uses.len() == 2) {
            let differs = uses[0].1 == uses[1].1;
            adjacency[uses[0].0].push((uses[1].0, differs));
            adjacency[uses[1].0].push((uses[0].0, differs));
        }
        let mut parity: Vec<Option<bool>> = vec![None; valid.len()];
        let mut ok = true;
        'faces: for start in 0..valid.len() {
            if parity[start].is_some() {
                continue;
            }
            parity[start] = Some(false);
            let mut stack = vec![start];
            while let Some(face) = stack.pop() {
                let here = parity[face].unwrap_or(false);
                for &(next, differs) in &adjacency[face] {
                    let want = here != differs;
                    match parity[next] {
                        None => {
                            parity[next] = Some(want);
                            stack.push(next);
                        }
                        Some(seen) if seen != want => {
                            ok = false;
                            break 'faces;
                        }
                        _ => {}
                    }
                }
            }
        }
        ok
    };

    let mut vertex_union = UnionFind::new(points.len());
    for face in &valid {
        for &index in face.iter().skip(1) {
            vertex_union.union(face[0] as usize, index as usize);
        }
    }
    let connected_components = {
        let mut roots = std::collections::HashSet::new();
        for (index, &is_used) in used.iter().enumerate() {
            if is_used {
                roots.insert(vertex_union.find(index));
            }
        }
        roots.len()
    };

    let non_manifold_vertices = {
        let mut links: HashMap<u32, Vec<(u32, usize)>> = HashMap::new();
        for (face_index, face) in valid.iter().enumerate() {
            for corner in 0..face.len() {
                let (previous, here, next) = (face[(corner + face.len() - 1) % face.len()], face[corner], face[(corner + 1) % face.len()]);
                let entry = links.entry(here).or_default();
                entry.push((previous, face_index));
                entry.push((next, face_index));
            }
        }
        links
            .into_iter()
            .filter(|(_, link)| {
                let mut ids: Vec<usize> = link.iter().map(|&(_, face)| face).collect();
                ids.sort_unstable();
                ids.dedup();
                let local: HashMap<usize, usize> = ids.iter().enumerate().map(|(slot, &face)| (face, slot)).collect();
                let mut union = UnionFind::new(ids.len());
                let mut by_neighbour: HashMap<u32, usize> = HashMap::new();
                for &(neighbour, face) in link {
                    let slot = local[&face];
                    match by_neighbour.get(&neighbour) {
                        Some(&other) => union.union(slot, other),
                        None => {
                            by_neighbour.insert(neighbour, slot);
                        }
                    }
                }
                (0..ids.len()).map(|slot| union.find(slot)).collect::<std::collections::HashSet<_>>().len() > 1
            })
            .count()
    };

    let used_points: Vec<P3> = points.iter().zip(&used).filter(|(_, &is_used)| is_used).map(|(p, _)| *p).collect();
    let bounding_box = if used_points.is_empty() {
        None
    } else {
        let mut bounds = MeshBounds { min: [f64::INFINITY; 3], max: [f64::NEG_INFINITY; 3] };
        for p in &used_points {
            for axis in 0..3 {
                bounds.min[axis] = bounds.min[axis].min(p[axis]);
                bounds.max[axis] = bounds.max[axis].max(p[axis]);
            }
        }
        Some(bounds)
    };
    let diagonal = bounding_box.map_or(0.0, |b| norm(sub(b.max, b.min)));

    let duplicate_vertices = {
        let cell = (1e-6 * diagonal).max(1e-12);
        let key = |p: P3| ((p[0] / cell).floor() as i64, (p[1] / cell).floor() as i64, (p[2] / cell).floor() as i64);
        let mut grid: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
        let mut duplicates = 0usize;
        for (index, &p) in points.iter().enumerate() {
            let (kx, ky, kz) = key(p);
            let mut found = false;
            'search: for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if let Some(bucket) = grid.get(&(kx + dx, ky + dy, kz + dz)) {
                            if bucket.iter().any(|&other| norm(sub(points[other], p)) <= cell) {
                                found = true;
                                break 'search;
                            }
                        }
                    }
                }
            }
            if found {
                duplicates += 1;
            }
            grid.entry((kx, ky, kz)).or_default().push(index);
        }
        duplicates
    };

    let vertex_count = points.len();
    let used_count = used.iter().filter(|&&is_used| is_used).count();
    let edge_count = edges.len();
    let euler_characteristic = used_count as i64 - edge_count as i64 + valid.len() as i64;
    let closed = !valid.is_empty() && boundary_edges == 0 && non_manifold_edges == 0;
    let genus = (closed && orientable && non_manifold_vertices == 0)
        .then(|| 2 * connected_components as i64 - euler_characteristic)
        .filter(|twice| twice % 2 == 0 && *twice >= 0)
        .map(|twice| twice / 2);
    let area_centroid = (area > 0.0).then(|| [area_moment[0] / area, area_moment[1] / area, area_moment[2] / area]);
    let mass = (closed && consistent_winding && signed_volume != 0.0).then(|| {
        let sign = signed_volume.signum();
        let volume = signed_volume.abs();
        let centroid = [volume_moment[0] / signed_volume, volume_moment[1] / signed_volume, volume_moment[2] / signed_volume];
        let trace = second_moment[0][0] + second_moment[1][1] + second_moment[2][2];
        let mut about_origin: InertiaTensor = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                about_origin[i][j] = sign * (if i == j { trace } else { 0.0 } - second_moment[i][j]);
            }
        }
        let inertia = to_centroid(about_origin, volume, centroid);
        MeshMassProperties { volume, centroid, inertia, principal: principal_inertia(inertia) }
    });

    MeshQualityReport {
        vertex_count,
        isolated_vertices: vertex_count - used_count,
        edge_count,
        face_count: valid.len(),
        triangle_count,
        invalid_faces,
        boundary_edges,
        boundary_loops,
        non_manifold_edges,
        non_manifold_vertices,
        degenerate_faces,
        duplicate_vertices,
        edge_length: stats(&lengths),
        aspect_ratio: stats(&aspect),
        connected_components,
        closed,
        orientable,
        consistent_winding,
        euler_characteristic,
        genus,
        area,
        area_centroid,
        signed_volume,
        bounding_box,
        mass,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
