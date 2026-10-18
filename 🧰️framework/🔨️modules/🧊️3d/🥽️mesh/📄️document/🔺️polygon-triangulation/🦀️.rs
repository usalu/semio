//! 🔺️ Polygon triangulation: the vector helpers, the plane basis, the indexed polygon triangulation with holes and bridges and the face loop merge. See [`super`].

use super::*;

//#region Polygon
/// 🧭️ Unit Newell normal evaluated relative to the first corner in double precision.
pub fn newell_normal(positions: &[Vec3]) -> Vec3 {
    let n = positions.len();
    let Some(origin) = positions.first() else { return Vec3::ZERO; };
    let mut nx = 0.0f64;
    let mut ny = 0.0f64;
    let mut nz = 0.0f64;
    for i in 0..n {
        let a = positions[i];
        let b = positions[(i + 1) % n];
        let (ax, ay, az) = (a.x() as f64 - origin.x() as f64, a.y() as f64 - origin.y() as f64, a.z() as f64 - origin.z() as f64);
        let (bx, by, bz) = (b.x() as f64 - origin.x() as f64, b.y() as f64 - origin.y() as f64, b.z() as f64 - origin.z() as f64);
        nx += (ay - by) * (az + bz);
        ny += (az - bz) * (ax + bx);
        nz += (ax - bx) * (ay + by);
    }
    let length = nx.hypot(ny).hypot(nz);
    if length == 0.0 { return Vec3::ZERO; }
    Vec3::new((nx / length) as f32, (ny / length) as f32, (nz / length) as f32)
}

pub(super) type Vec3f64 = (f64, f64, f64);

pub fn sub3(a: Vec3f64, b: Vec3f64) -> Vec3f64 {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

pub fn dot3(a: Vec3f64, b: Vec3f64) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn cross3(a: Vec3f64, b: Vec3f64) -> Vec3f64 {
    (a.1 * b.2 - a.2 * b.1, a.2 * b.0 - a.0 * b.2, a.0 * b.1 - a.1 * b.0)
}

pub fn length3(a: Vec3f64) -> f64 {
    dot3(a, a).sqrt()
}

pub(super) fn normalize3(a: Vec3f64) -> Vec3f64 {
    let l = length3(a);
    if l < 1e-12 {
        return (0.0, 0.0, 0.0);
    }
    (a.0 / l, a.1 / l, a.2 / l)
}

/// Least-parallel-world-axis projection basis for a plane with the given unit normal.
pub(super) fn plane_basis(normal: Vec3f64) -> (Vec3f64, Vec3f64) {
    let (nx, ny, nz) = normal;
    let reference: Vec3f64 = if nx.abs() < ny.abs() && nx.abs() < nz.abs() {
        (1.0, 0.0, 0.0)
    } else if ny.abs() < nz.abs() {
        (0.0, 1.0, 0.0)
    } else {
        (0.0, 0.0, 1.0)
    };
    let axis_u = normalize3(cross3(normal, reference));
    let axis_v = cross3(normal, axis_u);
    (axis_u, axis_v)
}

pub(super) fn cross2(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

pub(super) fn point_in_triangle(p: (f64, f64), a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
    let d1 = cross2(a, b, p);
    let d2 = cross2(b, c, p);
    let d3 = cross2(c, a, p);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

/// Triangulates a planar polygon that may contain holes. Holes are bridged into the outer loop with a
/// keyhole (doubled bridge edge) then ear-clipped, so the result covers only the solid region.
pub(super) fn triangulate_indexed_polygon_with_holes(positions: &[[f32; 3]], outer: &[u32], holes: &[Vec<u32>]) -> Vec<[u32; 3]> {
    if holes.is_empty() {
        let pts: Vec<Vec3> = outer.iter().map(|&i| Vec3(positions[i as usize])).collect();
        return triangulate_polygon(&pts).into_iter().map(|[a, b, c]| [outer[a], outer[b], outer[c]]).collect();
    }
    let mut combined: Vec<u32> = outer.to_vec();
    let mut remaining: Vec<Vec<u32>> = holes.to_vec();
    while let Some((hole_index, outer_pos, hole_pos)) = find_closest_bridge(&combined, &remaining, positions) {
        let hole = remaining.remove(hole_index);
        let mut spliced = Vec::with_capacity(combined.len() + hole.len() + 2);
        spliced.extend_from_slice(&combined[..=outer_pos]);
        spliced.push(hole[hole_pos]);
        for k in 1..hole.len() {
            spliced.push(hole[(hole_pos + k) % hole.len()]);
        }
        spliced.push(hole[hole_pos]);
        spliced.push(combined[outer_pos]);
        spliced.extend_from_slice(&combined[outer_pos + 1..]);
        combined = spliced;
    }
    let pts: Vec<Vec3> = combined.iter().map(|&i| Vec3(positions[i as usize])).collect();
    triangulate_polygon(&pts).into_iter().map(|[a, b, c]| [combined[a], combined[b], combined[c]]).collect()
}

fn find_closest_bridge(outer: &[u32], holes: &[Vec<u32>], positions: &[[f32; 3]]) -> Option<(usize, usize, usize)> {
    let mut best: Option<(f32, usize, usize, usize)> = None;
    for (hi, hole) in holes.iter().enumerate() {
        for (oi, &ov) in outer.iter().enumerate() {
            let operation = Vec3(positions[ov as usize]);
            for (hpi, &hv) in hole.iter().enumerate() {
                let hp = Vec3(positions[hv as usize]);
                let d = operation.sub(hp).length();
                if best.is_none_or(|(bd, _, _, _)| d < bd) {
                    best = Some((d, hi, oi, hpi));
                }
            }
        }
    }
    best.map(|(_, hi, oi, hpi)| (hi, oi, hpi))
}

/// Deterministic ear-clipping triangulation of a simple polygon (convex or concave), given ordered 3D corner
/// positions. Falls back to a fan (previous behavior) whenever the polygon is degenerate (zero-area / collinear)
/// or clipping stalls, so it never returns fewer than `n - 2` triangles or panics.
pub fn triangulate_polygon(positions: &[Vec3]) -> Vec<[usize; 3]> {
    let n = positions.len();
    if n < 3 {
        return Vec::new();
    }
    if n == 3 {
        return vec![[0, 1, 2]];
    }
    let fan = |from: usize| -> Vec<[usize; 3]> {
        let _ = from;
        (1..n - 1).map(|i| [0, i, i + 1]).collect()
    };
    let mut points_f64: Vec<Vec3f64> = Vec::with_capacity(positions.len());
    for p in positions {
        points_f64.push((p.x() as f64, p.y() as f64, p.z() as f64));
    }
    let normal = newell_normal(positions);
    if normal.length() < 1e-8 {
        return fan(0);
    }
    let normal_f64 = normalize3((normal.x() as f64, normal.y() as f64, normal.z() as f64));
    let (axis_u, axis_v) = plane_basis(normal_f64);
    let origin = points_f64[0];
    let mut projected: Vec<(f64, f64)> = Vec::with_capacity(points_f64.len());
    for &p in &points_f64 {
        let local = sub3(p, origin);
        projected.push((dot3(local, axis_u), dot3(local, axis_v)));
    }
    let scale = projected.iter().fold(0.0f64, |scale, &(x, y)| scale.max(x.abs()).max(y.abs()));
    if scale == 0.0 { return fan(0); }
    for point in &mut projected { point.0 /= scale; point.1 /= scale; }
    let mut signed_area2 = 0.0f64;
    for i in 0..n {
        let a = projected[i];
        let b = projected[(i + 1) % n];
        signed_area2 += a.0 * b.1 - b.0 * a.1;
    }
    if signed_area2.abs() < 1e-14 {
        return fan(0);
    }
    let ccw = signed_area2 > 0.0;

    let mut indices: Vec<usize> = (0..n).collect();
    let mut triangles = Vec::with_capacity(n - 2);
    let mut guard = 0usize;
    let guard_limit = n * n + 16;
    while indices.len() > 3 {
        guard += 1;
        if guard > guard_limit {
            for i in 1..indices.len() - 1 {
                triangles.push([indices[0], indices[i], indices[i + 1]]);
            }
            return triangles;
        }
        let m = indices.len();
        let mut ear_found = false;
        for i in 0..m {
            let prev = indices[(i + m - 1) % m];
            let curr = indices[i];
            let next = indices[(i + 1) % m];
            let a = projected[prev];
            let b = projected[curr];
            let c = projected[next];
            let cross = cross2(a, b, c);
            let is_convex = if ccw { cross > 1e-14 } else { cross < -1e-14 };
            if !is_convex {
                continue;
            }
            let mut contains_other = false;
            for &k in &indices {
                if k == prev || k == curr || k == next {
                    continue;
                }
                if point_in_triangle(projected[k], a, b, c) {
                    contains_other = true;
                    break;
                }
            }
            if contains_other {
                continue;
            }
            triangles.push([prev, curr, next]);
            indices.remove(i);
            ear_found = true;
            break;
        }
        if !ear_found {
            for i in 1..indices.len() - 1 {
                triangles.push([indices[0], indices[i], indices[i + 1]]);
            }
            return triangles;
        }
    }
    triangles.push([indices[0], indices[1], indices[2]]);
    triangles
}

fn find_edge_position(vertices:&[u32],from:u32,to:u32)->Option<usize> {(0..vertices.len()).find(|&index|vertices[index]==from && vertices[(index+1)%vertices.len()]==to)}

fn merge_face_loops(a:&[u32],b:&[u32])->Option<Vec<u32>> {
    if a.len()<3 || b.len()<3 {return None;}
    let ai=(0..a.len()).find(|&index|find_edge_position(b,a[(index+1)%a.len()],a[index]).is_some())?;let bi=find_edge_position(b,a[(ai+1)%a.len()],a[ai])?;
    if (0..a.len()).any(|index|index!=ai && find_edge_position(b,a[(index+1)%a.len()],a[index]).is_some()) {return None;}
    let mut merged=vec![a[ai]];merged.extend((2..b.len()).map(|index|b[(bi+index)%b.len()]));merged.extend((1..a.len()).map(|index|a[(ai+index)%a.len()]));let mut seen=HistoryFoldSet::new();if merged.iter().any(|&id|!seen.insert(id)) {None}else {Some(merged)}
}
//#endregion Polygon
