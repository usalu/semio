//! 📏️ Divergence-theorem mass properties, axis-aligned bounds, and solid distance/classify
//! queries on `SemioBrepSnapshot`'s arena `Body`. `oracle` (below) is a closed-form ground
//! truth used only by tests — deliberately independent of this module's own algorithms.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/{📏️measure,🔮️oracle}/🦀️.rs` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL2.

// 📏 Divergence-theorem mass properties, axis-aligned bounds, and solid distance queries on [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body`].

#[cfg(test)]
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{CoedgeId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{EdgeId, FaceId, ShellId, SolidId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::predicates::{orient2d, Orient};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec3};

// #region 🔖️Types

/// 📦 Axis-aligned bounds in model space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisAlignedBox {
    pub min: Pnt3,
    pub max: Pnt3,
}

/// 📏 Volume, area, centroid and inertia tensor of a solid, integrated over its TRIMMED support
/// (ear-clipped UV triangulation, adaptive Gauss-Legendre per triangle — see `solid_mass_properties`),
/// with an error estimate propagated from that adaptive refinement rather than an unverified guess.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MassProperties {
    pub volume: f64,
    pub area: f64,
    pub centroid: Pnt3,
    pub inertia: [[f64; 3]; 3],
    pub error_estimate: f64,
}

// #endregion 🔖️Types

// #region 🔖️Solid

/// 📐 Signed volume of `solid` via divergence theorem surface quadrature (`V = (1/3) ∫ P·n dA`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solid_volume(body: &Body, solid: SolidId, chord_tol: f64) -> Result<f64, KernelError> {
    if let Some(v) = try_analytic_sphere_volume(body, solid) {
        return Ok(v);
    }
    Ok(solid_signed_volume(body, solid, chord_tol)?.abs())
}

/// 📐 Raw signed volume (no `.abs()`, no analytic fast path) — negative when the solid's outer
/// shell face normals are net inward-oriented, used by validation's shell-orientation check.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solid_signed_volume(body: &Body, solid: SolidId, chord_tol: f64) -> Result<f64, KernelError> {
    shell_signed_volume_over(body, &body.solid_faces(solid), chord_tol)
}

/// 📐 Raw signed volume contribution of one shell's own faces (e.g. one void/inner shell in
/// isolation) — validation compares this against the solid's outer shell to confirm a void shell
/// is correctly inverted relative to the exterior.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn shell_signed_volume(body: &Body, shell: ShellId, chord_tol: f64) -> Result<f64, KernelError> {
    shell_signed_volume_over(body, &body.shell_faces(shell), chord_tol)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shell_signed_volume_over(body: &Body, faces: &[FaceId], chord_tol: f64) -> Result<f64, KernelError> {
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("shell has no faces".into()));
    }
    let mut total = 0.0;
    for &face in faces {
        total += face_volume_contribution(body, face, chord_tol)?;
    }
    Ok(total)
}

/// 📐 Total outer surface area of `solid`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solid_surface_area(body: &Body, solid: SolidId, chord_tol: f64) -> Result<f64, KernelError> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut total = 0.0;
    for face in faces {
        total += face_area(body, face, chord_tol)?;
    }
    Ok(total)
}

/// 📐 Center of mass of `solid` at uniform density (tetrahedral decomposition weighted by signed volume).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solid_center_of_mass(body: &Body, solid: SolidId, chord_tol: f64) -> Result<Pnt3, KernelError> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut vol = 0.0;
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut mz = 0.0;
    for face in faces {
        let (sv, cx, cy, cz) = face_volume_moments(body, face, chord_tol)?;
        vol += sv;
        mx += cx;
        my += cy;
        mz += cz;
    }
    if vol.abs() < 1e-15 {
        return Err(KernelError::InvalidInput("solid has zero volume".into()));
    }
    let denom = 4.0 * vol;
    Ok(Pnt3::new(mx / denom, my / denom, mz / denom))
}

/// 📦 Conservative axis-aligned bounding box of `solid` (vertices plus analytic surface expansion).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solid_bounding_box(body: &Body, solid: SolidId) -> Result<AxisAlignedBox, KernelError> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut min = Pnt3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = Pnt3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    let mut any = false;
    for face in faces {
        for p in face_sample_points(body, face)? {
            any = true;
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }
        if let Some(surface) = body.faces.get(face).and_then(|f| body.surfaces.get(f.surface)) {
            expand_bbox_for_surface(&mut min, &mut max, surface);
        }
    }
    if !any {
        return Err(KernelError::InvalidInput("solid has no geometry samples".into()));
    }
    Ok(AxisAlignedBox { min, max })
}

/// 📏 Volume, area, centroid and inertia tensor of `solid`, integrated over its TRIMMED support
/// (ear-clipped UV triangulation per face, adaptive Gauss-Legendre per triangle — see `loop_moments`),
/// with analytic fast paths for a solid built from exactly one sphere or one 6-planar-face box.
/// `error_estimate` is the adaptive refinement's own relative volume-error bound, not a guess.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solid_mass_properties(body: &Body, solid: SolidId, tol: f64) -> Result<MassProperties, KernelError> {
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity("solid".into()));
    }
    if !(tol.is_finite() && tol > 0.0) {
        return Err(KernelError::InvalidInput("tolerance must be positive and finite".into()));
    }
    if let Some(mp) = try_analytic_sphere_mass(body, solid) {
        return Ok(mp);
    }
    if let Some(mp) = try_analytic_box_properties(body, solid) {
        return Ok(mp);
    }
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut totals = [0.0; MOMENT_COMPONENTS];
    let mut err = 0.0;
    for face in faces {
        let (m, e) = face_moments_general(body, face, tol)?;
        for i in 0..MOMENT_COMPONENTS {
            totals[i] += m[i];
        }
        err += e;
    }
    let vol_raw = totals[IDX_VOL];
    if vol_raw.abs() < 1e-15 {
        return Err(KernelError::InvalidInput("solid has zero volume".into()));
    }
    let sign = vol_raw.signum();
    let volume = vol_raw.abs();
    let cx = totals[IDX_MX] / vol_raw;
    let cy = totals[IDX_MY] / vol_raw;
    let cz = totals[IDX_MZ] / vol_raw;
    let jxx2 = totals[IDX_JXX2] * sign;
    let jyy2 = totals[IDX_JYY2] * sign;
    let jzz2 = totals[IDX_JZZ2] * sign;
    let jxy = totals[IDX_JXY] * sign;
    let jxz = totals[IDX_JXZ] * sign;
    let jyz = totals[IDX_JYZ] * sign;
    let ixx = jyy2 + jzz2 - volume * (cy * cy + cz * cz);
    let iyy = jxx2 + jzz2 - volume * (cx * cx + cz * cz);
    let izz = jxx2 + jyy2 - volume * (cx * cx + cy * cy);
    let ixy = jxy - volume * cx * cy;
    let ixz = jxz - volume * cx * cz;
    let iyz = jyz - volume * cy * cz;
    let error_estimate = if volume > 1e-12 { err / volume } else { err };
    Ok(MassProperties {
        volume,
        area: totals[IDX_AREA],
        centroid: Pnt3::new(cx, cy, cz),
        inertia: [[ixx, -ixy, -ixz], [-ixy, iyy, -iyz], [-ixz, -iyz, izz]],
        error_estimate,
    })
}

/// 📏 A face's trimmed surface-integral moments (outer loop minus each inner/hole loop), used
/// uniformly for planar AND curved faces by `solid_mass_properties` — the 6-point rule is already
/// exact on a flat facet (constant `cross` vector, integrand degree ≤ 3), so no special-casing is
/// needed the way the legacy `loop_area`/`loop_volume_moments` pair still does for their own
/// (unrelated, ×6/×24-scaled) callers.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_moments_general(body: &Body, face: FaceId, tol: f64) -> Result<([f64; MOMENT_COMPONENTS], f64), KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = face_surface(body, face)?;
    let flipped = face_ent.flipped;
    let mut total = [0.0; MOMENT_COMPONENTS];
    let mut err = 0.0;
    if let Some(outer) = face_ent.outer {
        let poly = loop_uv_polygon(body, outer, surface, tol)?;
        let (m, e) = loop_moments(surface, flipped, &poly, tol);
        for i in 0..MOMENT_COMPONENTS {
            total[i] += m[i];
        }
        err += e;
    }
    for &inner in &face_ent.inners {
        let poly = loop_uv_polygon(body, inner, surface, tol)?;
        let (m, e) = loop_moments(surface, flipped, &poly, tol);
        for i in 0..MOMENT_COMPONENTS {
            total[i] -= m[i];
        }
        err += e;
    }
    Ok((total, err))
}

/// 📏 Exact closed-form mass properties when `solid` is a single sphere (any number of Sphere faces
/// sharing one center/radius, e.g. a seamed sphere with pole faces).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn try_analytic_sphere_mass(body: &Body, solid: SolidId) -> Option<MassProperties> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return None;
    }
    let mut origin: Option<Pnt3> = None;
    let mut radius: Option<f64> = None;
    for fid in &faces {
        let face = body.faces.get(*fid)?;
        let surf = body.surfaces.get(face.surface)?;
        let Surface::Sphere { frame, radius: r } = surf else {
            return None;
        };
        match (origin, radius) {
            (None, None) => {
                origin = Some(frame.origin);
                radius = Some(*r);
            }
            (Some(o), Some(r0)) if o.distance(frame.origin) < 1e-9 * r0.max(1.0) && (r0 - r).abs() < 1e-9 * r0.max(1.0) => {}
            _ => return None,
        }
    }
    let r = radius?;
    let o = origin?;
    let volume = 4.0 / 3.0 * std::f64::consts::PI * r * r * r;
    let area = 4.0 * std::f64::consts::PI * r * r;
    let i = 2.0 / 5.0 * volume * r * r;
    Some(MassProperties { volume, area, centroid: o, inertia: [[i, 0.0, 0.0], [0.0, i, 0.0], [0.0, 0.0, i]], error_estimate: 0.0 })
}

/// 📏 Exact closed-form mass properties when `solid` is exactly one rectangular box: 6 planar
/// faces, 8 distinct vertices, and one corner whose 3 incident edges are mutually orthogonal.
/// Orientation-agnostic (built from raw vertex adjacency, not face normals), so it works
/// regardless of any face's `flipped` flag.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn try_analytic_box_properties(body: &Body, solid: SolidId) -> Option<MassProperties> {
    let faces = body.solid_faces(solid);
    if faces.len() != 6 {
        return None;
    }
    for &f in &faces {
        let face = body.faces.get(f)?;
        let surf = body.surfaces.get(face.surface)?;
        if !matches!(surf, Surface::Plane { .. }) {
            return None;
        }
    }
    let mut positions: Vec<Pnt3> = Vec::new();
    for &f in &faces {
        for coedge in body.face_coedges(f) {
            let (v0, _) = body.coedge_endpoints(coedge)?;
            let p = body.vertices.get(v0)?.position;
            if !positions.iter().any(|q: &Pnt3| q.distance(p) < 1e-9) {
                positions.push(p);
            }
        }
    }
    if positions.len() != 8 {
        return None;
    }
    let origin = positions[0];
    let mut neighbors: Vec<Pnt3> = Vec::new();
    for edge_id in solid_unique_edges(body, solid) {
        let e = body.edges.get(edge_id)?;
        let v0p = body.vertices.get(e.v0)?.position;
        let v1p = body.vertices.get(e.v1)?.position;
        if v0p.distance(origin) < 1e-9 && !neighbors.iter().any(|q: &Pnt3| q.distance(v1p) < 1e-9) {
            neighbors.push(v1p);
        } else if v1p.distance(origin) < 1e-9 && !neighbors.iter().any(|q: &Pnt3| q.distance(v0p) < 1e-9) {
            neighbors.push(v0p);
        }
    }
    if neighbors.len() != 3 {
        return None;
    }
    let (lx, ly, lz) = ((neighbors[0] - origin).norm(), (neighbors[1] - origin).norm(), (neighbors[2] - origin).norm());
    if lx < 1e-12 || ly < 1e-12 || lz < 1e-12 {
        return None;
    }
    let ux = (neighbors[0] - origin).normalized()?;
    let uy = (neighbors[1] - origin).normalized()?;
    let uz = (neighbors[2] - origin).normalized()?;
    let ortho_tol = 1e-6;
    if ux.dot(uy).abs() > ortho_tol || uy.dot(uz).abs() > ortho_tol || ux.dot(uz).abs() > ortho_tol {
        return None;
    }
    let volume = lx * ly * lz;
    let area = 2.0 * (lx * ly + ly * lz + lx * lz);
    let centroid = origin + (ux * (lx * 0.5) + uy * (ly * 0.5) + uz * (lz * 0.5));
    let ixx_local = (ly * ly + lz * lz) * volume / 12.0;
    let iyy_local = (lx * lx + lz * lz) * volume / 12.0;
    let izz_local = (lx * lx + ly * ly) * volume / 12.0;
    let r = [[ux.x, uy.x, uz.x], [ux.y, uy.y, uz.y], [ux.z, uy.z, uz.z]];
    let i_local = [[ixx_local, 0.0, 0.0], [0.0, iyy_local, 0.0], [0.0, 0.0, izz_local]];
    let inertia = rotate_inertia(&r, &i_local);
    Some(MassProperties { volume, area, centroid, inertia, error_estimate: 0.0 })
}

/// 📏 `R * I_local * Rᵀ` — rotates a diagonal local-axis inertia tensor into world axes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rotate_inertia(r: &[[f64; 3]; 3], i_local: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut ri = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut s = 0.0;
            for k in 0..3 {
                s += r[i][k] * i_local[k][j];
            }
            ri[i][j] = s;
        }
    }
    let mut result = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut s = 0.0;
            for k in 0..3 {
                s += ri[i][k] * r[j][k];
            }
            result[i][j] = s;
        }
    }
    result
}

// #endregion 🔖️Solid

// #region 🔖️FaceEdge

/// 📐 Area of one face (`outer` minus `inner` loops).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn face_area(body: &Body, face: FaceId, chord_tol: f64) -> Result<f64, KernelError> {
    let Some(face_ent) = body.faces.get(face) else {
        return Err(KernelError::MissingEntity("face".into()));
    };
    let mut area = 0.0;
    if let Some(outer) = face_ent.outer {
        area += loop_area(body, face, outer, chord_tol)?;
    }
    for &inner in &face_ent.inners {
        area -= loop_area(body, face, inner, chord_tol)?;
    }
    Ok(area.abs())
}

/// 📐 Arc length of an edge over its trimmed parameter range.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn edge_length(body: &Body, edge: EdgeId) -> Result<f64, KernelError> {
    let edge_ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let curve = body.curves3.get(edge_ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
    Ok(curve_ops::arc_length(curve, edge_ent.range.0, edge_ent.range.1, 1e-9))
}

// #endregion 🔖️FaceEdge

// #region 🔖️Distance

/// 📏 Minimum distance between two closed solids — `0.0` only when a real point-in-solid
/// classification (not a coarse sample-point-happens-to-be-close heuristic) finds one solid
/// genuinely containing a point of the other; otherwise the TRUE minimum via vertex/face, face
/// sample point/face, and edge/edge candidates (§6.11: "overlapping-solid distance can sample face
/// points and miss edge-edge/interior extrema").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn distance_solid_solid(body: &Body, a: SolidId, b: SolidId) -> Result<f64, KernelError> {
    let bb_a = solid_bounding_box(body, a)?;
    let bb_b = solid_bounding_box(body, b)?;
    let separated = axis_aligned_box_distance(&bb_a, &bb_b);
    if separated > 1e-9 {
        return Ok(separated);
    }
    if solids_overlap(body, a, b)? {
        return Ok(0.0);
    }
    let mut best = f64::INFINITY;
    for face in body.solid_faces(a) {
        for p in face_sample_points(body, face)? {
            let (_, d) = closest_point_on_solid(body, b, p)?;
            best = best.min(d);
        }
    }
    for face in body.solid_faces(b) {
        for p in face_sample_points(body, face)? {
            let (_, d) = closest_point_on_solid(body, a, p)?;
            best = best.min(d);
        }
    }
    if let Some(d) = edge_edge_closest_distance(body, a, b) {
        best = best.min(d);
    }
    if !best.is_finite() {
        return Err(KernelError::Operation("distance_solid_solid: no finite candidate distance found".into()));
    }
    Ok(best)
}

/// 📏 `true` when a sample point on either solid's boundary is genuinely `Inside` the other, per
/// the one authoritative classifier — not a bounding-box or sample-distance proxy for overlap.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solids_overlap(body: &Body, a: SolidId, b: SolidId) -> Result<bool, KernelError> {
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::PointClassification;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::classification::point_in_solid;
    for face in body.solid_faces(a) {
        for p in face_sample_points(body, face)? {
            if matches!(point_in_solid(body, b, p, 1e-9)?, PointClassification::Inside) {
                return Ok(true);
            }
        }
    }
    for face in body.solid_faces(b) {
        for p in face_sample_points(body, face)? {
            if matches!(point_in_solid(body, a, p, 1e-9)?, PointClassification::Inside) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_unique_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
    let mut seen = std::collections::HashSet::new();
    for face in body.solid_faces(solid) {
        for coedge in body.face_coedges(face) {
            if let Some(c) = body.coedges.get(coedge) {
                seen.insert(c.edge);
            }
        }
    }
    seen.into_iter().collect()
}

/// 📏 Nearest approach between every edge of `a` and every edge of `b`, sampling one curve against
/// the other's `curve_ops::closest_parameter` in both directions — a practical (sampling-refined,
/// not a certified global continuous optimum) edge/edge extremum candidate the old face-sample-only
/// distance missed entirely.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_edge_closest_distance(body: &Body, a: SolidId, b: SolidId) -> Option<f64> {
    let edges_a = solid_unique_edges(body, a);
    let edges_b = solid_unique_edges(body, b);
    let mut best: Option<f64> = None;
    for &ea in &edges_a {
        for &eb in &edges_b {
            if let Some(d) = edge_edge_min_distance(body, ea, eb) {
                best = Some(best.map_or(d, |b: f64| b.min(d)));
            }
        }
    }
    best
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_edge_min_distance(body: &Body, ea: EdgeId, eb: EdgeId) -> Option<f64> {
    let edge_a = body.edges.get(ea)?;
    let edge_b = body.edges.get(eb)?;
    let curve_a = body.curves3.get(edge_a.curve)?;
    let curve_b = body.curves3.get(edge_b.curve)?;
    const SAMPLES: usize = 12;
    let mut best = f64::INFINITY;
    for i in 0..=SAMPLES {
        let s = i as f64 / SAMPLES as f64;
        let t = edge_a.range.0 + (edge_a.range.1 - edge_a.range.0) * s;
        let closest = curve_ops::closest_parameter(curve_b, edge_b.range, curve_a.eval(t), 1e-9);
        best = best.min(closest.distance);
    }
    for i in 0..=SAMPLES {
        let s = i as f64 / SAMPLES as f64;
        let t = edge_b.range.0 + (edge_b.range.1 - edge_b.range.0) * s;
        let closest = curve_ops::closest_parameter(curve_a, edge_a.range, curve_b.eval(t), 1e-9);
        best = best.min(closest.distance);
    }
    Some(best)
}

/// 📏 Closest point on `solid` to `point` and the Euclidean distance.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn closest_point_on_solid(body: &Body, solid: SolidId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut best_p = point;
    let mut best_d = f64::INFINITY;
    for face in faces {
        let (p, d) = closest_point_on_face(body, face, point)?;
        if d < best_d {
            best_d = d;
            best_p = p;
        }
    }
    Ok((best_p, best_d))
}

// #endregion 🔖️Distance

// #region 🔖️Classify
//
// 🏷️ The ray-parity classifier that used to live here was deleted (ticket
// 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave W1-F): `inferences::classification::point_in_solid`
// is now the ONE authoritative point-in-solid classifier (BVH-culled, p-curve-aware trims, grazing
// retry) — see that module's docstring. `distance_solid_solid` below now calls the real classifier
// for its overlap check instead of keeping a second, coarser implementation in sync with it.
// #endregion 🔖️Classify

// #region 🔖️Quadrature

/// 📏 One UV triangle's surface-integral contributions, all in NATURAL physical units (no legacy
/// ×6/×24 tetra-sum scaling): `[area, vol, mx, my, mz, ∫x²dV, ∫y²dV, ∫z²dV, ∫xy dV, ∫xz dV, ∫yz dV]`.
/// Every entry after `area` is a divergence-theorem surface integral of the form `∮ f(p)·n dA`,
/// and since `n dA = (du × dv) du dv` exactly (no need to normalize the cross product), each is
/// just `f(p) · cross` at the sample point — the SAME per-sample machinery serves volume, first
/// moments (centroid) and second moments (inertia) at once.
const MOMENT_COMPONENTS: usize = 11;
const IDX_AREA: usize = 0;
const IDX_VOL: usize = 1;
const IDX_MX: usize = 2;
const IDX_MY: usize = 3;
const IDX_MZ: usize = 4;
const IDX_JXX2: usize = 5;
const IDX_JYY2: usize = 6;
const IDX_JZZ2: usize = 7;
const IDX_JXY: usize = 8;
const IDX_JXZ: usize = 9;
const IDX_JYZ: usize = 10;

/// 📏 6-point symmetric (degree-4) triangle quadrature rule in barycentric coordinates — exact
/// for the cubic integrands second moments need on a flat facet, and a stable adaptive-refinement
/// base case for curved ones.
const TRI_BARY: [[f64; 3]; 6] = [
    [0.108_103_018_168_070, 0.445_948_490_915_965, 0.445_948_490_915_965],
    [0.445_948_490_915_965, 0.108_103_018_168_070, 0.445_948_490_915_965],
    [0.445_948_490_915_965, 0.445_948_490_915_965, 0.108_103_018_168_070],
    [0.816_847_572_980_459, 0.091_576_213_509_771, 0.091_576_213_509_771],
    [0.091_576_213_509_771, 0.816_847_572_980_459, 0.091_576_213_509_771],
    [0.091_576_213_509_771, 0.091_576_213_509_771, 0.816_847_572_980_459],
];
const TRI_WEIGHT: [f64; 6] = [0.223_381_589_678_011, 0.223_381_589_678_011, 0.223_381_589_678_011, 0.109_951_743_655_322, 0.109_951_743_655_322, 0.109_951_743_655_322];

/// 📏 Single (non-adaptive) 6-point quadrature pass over one UV triangle.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn quad_triangle_once(surface: &Surface, flipped: bool, tri: [Pnt2; 3]) -> [f64; MOMENT_COMPONENTS] {
    let signed_area2 = (tri[1].x - tri[0].x) * (tri[2].y - tri[0].y) - (tri[1].y - tri[0].y) * (tri[2].x - tri[0].x);
    let tri_area = 0.5 * signed_area2.abs();
    let mut acc = [0.0; MOMENT_COMPONENTS];
    if tri_area < 1e-15 {
        return acc;
    }
    for k in 0..6 {
        let [l1, l2, l3] = TRI_BARY[k];
        let u = l1 * tri[0].x + l2 * tri[1].x + l3 * tri[2].x;
        let v = l1 * tri[0].y + l2 * tri[1].y + l3 * tri[2].y;
        let d = surface.derivatives(u, v);
        let p = d.point;
        let mut cross = d.du.cross(d.dv);
        if flipped {
            cross = -cross;
        }
        let w = TRI_WEIGHT[k] * tri_area;
        acc[IDX_AREA] += w * cross.norm();
        acc[IDX_VOL] += w * (p.x * cross.x + p.y * cross.y + p.z * cross.z) / 3.0;
        acc[IDX_MX] += w * 0.5 * p.x * p.x * cross.x;
        acc[IDX_MY] += w * 0.5 * p.y * p.y * cross.y;
        acc[IDX_MZ] += w * 0.5 * p.z * p.z * cross.z;
        acc[IDX_JXX2] += w * (p.x * p.x * p.x / 3.0) * cross.x;
        acc[IDX_JYY2] += w * (p.y * p.y * p.y / 3.0) * cross.y;
        acc[IDX_JZZ2] += w * (p.z * p.z * p.z / 3.0) * cross.z;
        acc[IDX_JXY] += w * 0.5 * p.x * p.x * p.y * cross.x;
        acc[IDX_JXZ] += w * 0.5 * p.x * p.x * p.z * cross.x;
        acc[IDX_JYZ] += w * 0.5 * p.y * p.y * p.z * cross.y;
    }
    acc
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_triangle_4(tri: [Pnt2; 3]) -> [[Pnt2; 3]; 4] {
    let m01 = tri[0].lerp(tri[1], 0.5);
    let m12 = tri[1].lerp(tri[2], 0.5);
    let m20 = tri[2].lerp(tri[0], 0.5);
    [[tri[0], m01, m20], [m01, tri[1], m12], [m20, m12, tri[2]], [m01, m12, m20]]
}

/// 📏 Adaptively refines one UV triangle by quartering until the volume component's relative
/// change between one refinement level and the next falls below `tol`, or `depth` is exhausted.
/// Returns the accumulated moments plus a Richardson-style absolute error estimate for the volume
/// component (the coarse/fine gap at whichever level accepted the result).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn integrate_triangle_adaptive(surface: &Surface, flipped: bool, tri: [Pnt2; 3], tol: f64, depth: u32) -> ([f64; MOMENT_COMPONENTS], f64) {
    let coarse = quad_triangle_once(surface, flipped, tri);
    let subs = split_triangle_4(tri);
    let mut fine = [0.0; MOMENT_COMPONENTS];
    for s in &subs {
        let c = quad_triangle_once(surface, flipped, *s);
        for i in 0..MOMENT_COMPONENTS {
            fine[i] += c[i];
        }
    }
    let local_err = (fine[IDX_VOL] - coarse[IDX_VOL]).abs();
    if depth == 0 {
        return (fine, local_err);
    }
    let rel = if fine[IDX_VOL].abs() > 1e-12 { local_err / fine[IDX_VOL].abs() } else { local_err };
    if rel < tol {
        (fine, local_err)
    } else {
        let mut total = [0.0; MOMENT_COMPONENTS];
        let mut err_sum = 0.0;
        for s in subs {
            let (r, e) = integrate_triangle_adaptive(surface, flipped, s, tol, depth - 1);
            for i in 0..MOMENT_COMPONENTS {
                total[i] += r[i];
            }
            err_sum += e;
        }
        (total, err_sum)
    }
}

const ADAPTIVE_MAX_DEPTH: u32 = 6;

/// 📏 One loop's surface-integral moments over its ear-clipped UV triangulation — the caller adds
/// this for the outer loop and subtracts it for each inner (hole) loop to get the face's trimmed
/// total, exactly mirroring the existing planar `signed_tetra_sum` +outer/-inner pattern.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_moments(surface: &Surface, flipped: bool, poly: &[Pnt2], tol: f64) -> ([f64; MOMENT_COMPONENTS], f64) {
    let mut total = [0.0; MOMENT_COMPONENTS];
    let mut err = 0.0;
    for tri in ear_clip(poly) {
        let (m, e) = integrate_triangle_adaptive(surface, flipped, tri, tol, ADAPTIVE_MAX_DEPTH);
        for i in 0..MOMENT_COMPONENTS {
            total[i] += m[i];
        }
        err += e;
    }
    (total, err)
}

// #endregion 🔖️Quadrature

// #region 🔖️Triangulation

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn polygon_signed_area_2d(poly: &[Pnt2]) -> f64 {
    let mut a = 0.0;
    let n = poly.len();
    for i in 0..n {
        let p = poly[i];
        let q = poly[(i + 1) % n];
        a += p.x * q.y - q.x * p.y;
    }
    0.5 * a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_or_on_triangle(p: Pnt2, a: Pnt2, b: Pnt2, c: Pnt2) -> bool {
    let d1 = orient2d(a, b, p);
    let d2 = orient2d(b, c, p);
    let d3 = orient2d(c, a, p);
    let has_neg = matches!(d1, Orient::Negative) || matches!(d2, Orient::Negative) || matches!(d3, Orient::Negative);
    let has_pos = matches!(d1, Orient::Positive) || matches!(d2, Orient::Positive) || matches!(d3, Orient::Positive);
    !(has_neg && has_pos)
}

/// 📐 Ear-clipping triangulation of a simple (non-self-intersecting) polygon, robust to either
/// winding direction (normalized to CCW internally). Holes are handled by the caller triangulating
/// each loop separately and subtracting — no hole-bridging needed. A degenerate input (collinear
/// runs that starve the ear search) stops early rather than looping forever, covering whatever
/// prefix was already clipped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ear_clip(poly: &[Pnt2]) -> Vec<[Pnt2; 3]> {
    if poly.len() < 3 {
        return Vec::new();
    }
    let mut pts = poly.to_vec();
    if polygon_signed_area_2d(&pts) < 0.0 {
        pts.reverse();
    }
    let mut idx: Vec<usize> = (0..pts.len()).collect();
    let mut tris = Vec::new();
    let guard_limit = pts.len() * pts.len() + 8;
    let mut guard = 0usize;
    while idx.len() > 3 && guard < guard_limit {
        guard += 1;
        let n = idx.len();
        let mut ear_at: Option<usize> = None;
        for i in 0..n {
            let prev = idx[(i + n - 1) % n];
            let cur = idx[i];
            let next = idx[(i + 1) % n];
            let (a, b, c) = (pts[prev], pts[cur], pts[next]);
            if orient2d(a, b, c) != Orient::Positive {
                continue;
            }
            let blocked = idx.iter().any(|&k| k != prev && k != cur && k != next && point_in_or_on_triangle(pts[k], a, b, c));
            if !blocked {
                ear_at = Some(i);
                break;
            }
        }
        let Some(i) = ear_at else { break };
        let n = idx.len();
        let prev = idx[(i + n - 1) % n];
        let cur = idx[i];
        let next = idx[(i + 1) % n];
        tris.push([pts[prev], pts[cur], pts[next]]);
        idx.remove(i);
    }
    if idx.len() == 3 {
        tris.push([pts[idx[0]], pts[idx[1]], pts[idx[2]]]);
    }
    tris
}

// #endregion 🔖️Triangulation

// #region 🔖️Loops

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn axis_aligned_box_distance(a: &AxisAlignedBox, b: &AxisAlignedBox) -> f64 {
    let dx = gap_1d(a.min.x, a.max.x, b.min.x, b.max.x);
    let dy = gap_1d(a.min.y, a.max.y, b.min.y, b.max.y);
    let dz = gap_1d(a.min.z, a.max.z, b.min.z, b.max.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gap_1d(a0: f64, a1: f64, b0: f64, b1: f64) -> f64 {
    if a1 < b0 {
        b0 - a1
    } else if b1 < a0 {
        a0 - b1
    } else {
        0.0
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_area(body: &Body, face: FaceId, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, chord_tol: f64) -> Result<f64, KernelError> {
    let surface = face_surface(body, face)?;
    let flipped = body.faces.get(face).map(|f| f.flipped).unwrap_or(false);
    match surface {
        Surface::Plane { frame } if loop_has_only_straight_edges(body, loop_id) => {
            let pts = loop_positions(body, loop_id)?;
            Ok(newell_area(&pts, outward_plane_normal(frame, flipped)))
        }
        _ => {
            let boundary = loop_uv_polygon(body, loop_id, surface, chord_tol)?;
            let (m, _err) = loop_moments(surface, flipped, &boundary, chord_tol.max(1e-5));
            Ok(m[IDX_AREA])
        }
    }
}

/// 📏 True only when every coedge of `loop_id` walks a straight [`Curve3::Line`] — the fast
/// one-vertex-per-coedge `loop_positions` path (below) is exact ONLY for such loops. A planar
/// face's boundary can still carry a curved (`Circle`/`Ellipse`/`Nurbs`) edge — a cylinder's cap,
/// a plate with a round hole — and `loop_positions` silently degenerates that loop to a 1-4 point
/// "polygon" (e.g. a cylinder cap's single full-circle coedge collapses to ONE point, zeroing the
/// cap's whole area/volume contribution): the same under-sampling class of bug as
/// [`loop_uv_polygon`]'s, just for the OTHER (planar fast-path) branch of `loop_area`/
/// `loop_volume_moments`. Route those loops through the curvature-adaptive general path instead
/// (`face_moments_general`'s own doc already notes the general quadrature is exact on a flat facet
/// too, so this loses nothing but the fast path's cheapness).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_has_only_straight_edges(body: &Body, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId) -> bool {
    body.loop_coedges(loop_id).into_iter().all(|coedge| {
        body.coedges
            .get(coedge)
            .and_then(|co| body.edges.get(co.edge))
            .and_then(|edge| body.curves3.get(edge.curve))
            .is_some_and(|curve| matches!(curve, Curve3::Line { .. }))
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_volume_contribution(body: &Body, face: FaceId, chord_tol: f64) -> Result<f64, KernelError> {
    let Some(face_ent) = body.faces.get(face) else {
        return Err(KernelError::MissingEntity("face".into()));
    };
    let mut vol = 0.0;
    if let Some(outer) = face_ent.outer {
        vol += loop_volume_contribution(body, face, outer, chord_tol)?;
    }
    for &inner in &face_ent.inners {
        vol -= loop_volume_contribution(body, face, inner, chord_tol)?;
    }
    Ok(vol)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_volume_moments(body: &Body, face: FaceId, chord_tol: f64) -> Result<(f64, f64, f64, f64), KernelError> {
    let Some(face_ent) = body.faces.get(face) else {
        return Err(KernelError::MissingEntity("face".into()));
    };
    let mut sv = 0.0;
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut mz = 0.0;
    if let Some(outer) = face_ent.outer {
        let (a, b, c, d) = loop_volume_moments(body, face, outer, chord_tol)?;
        sv += a;
        mx += b;
        my += c;
        mz += d;
    }
    for &inner in &face_ent.inners {
        let (a, b, c, d) = loop_volume_moments(body, face, inner, chord_tol)?;
        sv -= a;
        mx -= b;
        my -= c;
        mz -= d;
    }
    Ok((sv, mx, my, mz))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_volume_contribution(body: &Body, face: FaceId, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, chord_tol: f64) -> Result<f64, KernelError> {
    let (sv, _, _, _) = loop_volume_moments(body, face, loop_id, chord_tol)?;
    Ok(sv / 6.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_volume_moments(body: &Body, face: FaceId, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, chord_tol: f64) -> Result<(f64, f64, f64, f64), KernelError> {
    let surface = face_surface(body, face)?;
    let flipped = body.faces.get(face).map(|f| f.flipped).unwrap_or(false);
    match surface {
        Surface::Plane { .. } if loop_has_only_straight_edges(body, loop_id) => {
            let pts = loop_positions(body, loop_id)?;
            let (sv, mx, my, mz) = signed_tetra_sum(&pts);
            // 🐛 FIX: this fast path derives its sign purely from the loop's own vertex winding,
            // which never encodes `face.flipped` — the general (non-planar) branch below DOES
            // negate for `flipped` (`quad_triangle_once` flips `cross(du,dv)`), so a flipped
            // planar face silently got the wrong-signed volume/moment contribution here, breaking
            // `solid_signed_volume`'s orientation sign for any solid with a flipped planar face.
            if flipped {
                Ok((-sv, -mx, -my, -mz))
            } else {
                Ok((sv, mx, my, mz))
            }
        }
        _ => {
            let boundary = loop_uv_polygon(body, loop_id, surface, chord_tol)?;
            let (m, _err) = loop_moments(surface, flipped, &boundary, chord_tol.max(1e-5));
            Ok((6.0 * m[IDX_VOL], 24.0 * m[IDX_MX], 24.0 * m[IDX_MY], 24.0 * m[IDX_MZ]))
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn signed_tetra_sum(pts: &[Pnt3]) -> (f64, f64, f64, f64) {
    if pts.len() < 3 {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let p0 = pts[0];
    let mut sv = 0.0;
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut mz = 0.0;
    for i in 1..pts.len() - 1 {
        let a = p0.to_vec();
        let b = pts[i] - p0;
        let c = pts[i + 1] - p0;
        let tet = a.dot(b.cross(c));
        sv += tet;
        mx += tet * (p0.x + pts[i].x + pts[i + 1].x);
        my += tet * (p0.y + pts[i].y + pts[i + 1].y);
        mz += tet * (p0.z + pts[i].z + pts[i + 1].z);
    }
    (sv, mx, my, mz)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_positions(body: &Body, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId) -> Result<Vec<Pnt3>, KernelError> {
    let mut pts = Vec::new();
    for coedge in body.loop_coedges(loop_id) {
        let (v0, _) = body.coedge_endpoints(coedge).ok_or_else(|| KernelError::InvalidInput("open coedge".into()))?;
        let v = body.vertices.get(v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?;
        pts.push(v.position);
    }
    Ok(pts)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_surface<'a>(body: &'a Body, face: FaceId) -> Result<&'a Surface, KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    body.surfaces.get(face_ent.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn outward_plane_normal(frame: &crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3, flipped: bool) -> Vec3 {
    let mut n = frame.z;
    if flipped {
        n = -n;
    }
    n
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn newell_area(pts: &[Pnt3], normal: Vec3) -> f64 {
    if pts.len() < 3 {
        return 0.0;
    }
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        cx += pts[i].y * pts[j].z - pts[i].z * pts[j].y;
        cy += pts[i].z * pts[j].x - pts[i].x * pts[j].z;
        cz += pts[i].x * pts[j].y - pts[i].y * pts[j].x;
    }
    let area_vec = Vec3::new(cx, cy, cz);
    0.5 * area_vec.dot(normal).abs()
}

/// 📏 UV boundary polygon for a (possibly curved-edge) loop — samples EACH coedge at a
/// curvature-adaptive point count (not just its start vertex: W1-E's flagged gap, a straight-edge
/// polygon is exact with one sample per coedge, but any curved-edge loop with few coedges — every
/// lateral/cap/spherical/toroidal face `🧱️primitives/🦀️.rs` builds — degenerated to a 1-4 point
/// "polygon" and made `point_in_uv_polygon`/the ear-clip quadrature under/over-integrate).
/// 🐛 FIX: an earlier pass bumped this from 1 sample/coedge to a FIXED `EDGE_SAMPLES = 8`, which
/// fixed the degenerate-polygon shape but is still tolerance-blind — a `chord_tol = 1e-3` caller
/// (`solid_volume`) got an 8-gon inscribed circle whose area already undershoots by ~10% (n=8's
/// sagitta is a full 7.6% of a r=1.5 circle), regardless of how tight the caller's tolerance is.
/// The interior quadrature (`integrate_triangle_adaptive`) genuinely refines to `chord_tol`, but
/// only ever refines WITHIN the ear-clipped boundary's fixed straight-chord approximation, so a
/// coarse boundary chord caps accuracy no matter how deep the interior recursion goes. Now derives
/// `n` per-coedge from `chord_tol` via the same closed-form chordal-deviation formula tessellation
/// uses for edge sampling (`segments_for_chord_deviation`, kept as a separate small copy here per
/// doctrine — different call shape, `+1` for point count vs segment count, no angular-tol term
/// since this feeds a quadrature boundary, not a rendered mesh's normal quality).
/// Prefers the coedge's own p-curve (`Coedge::pcurve`/`prange`, mirroring classification.rs's
/// `coedge_uv_sample` pattern — kept close per doctrine rather than cross-imported) over
/// reprojecting the 3D curve, and reads it in the coedge's OWN order regardless of `forward`,
/// reversing only when `forward == false` (W1-E's binding convention, fixed alongside this pass —
/// see [`coedge_uv_sample`]'s own doc); the 3D-curve fallback already reverses via `t`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_uv_polygon(body: &Body, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, surface: &Surface, chord_tol: f64) -> Result<Vec<Pnt2>, KernelError> {
    let mut poly: Vec<Pnt2> = Vec::new();
    let mut prev_u: Option<f64> = None;
    let mut prev_was_pole = false;
    let coedges = body.loop_coedges(loop_id);
    for (ci, coedge_id) in coedges.iter().enumerate() {
        let co = body.coedges.get(*coedge_id).ok_or_else(|| KernelError::MissingEntity("coedge".into()))?;
        let n = coedge_sample_count(body, co, chord_tol).max(2);
        for i in 0..n {
            let s = i as f64 / (n - 1) as f64;
            let mut uv = coedge_uv_sample(body, co, surface, s)?;
            let is_pole = surface.normal(uv.x, uv.y).is_none();
            // `s` must reach 1.0 (via `i/(n-1)`, not `i/n`) so each coedge's samples actually span
            // its own full [0,1]. The last-sample-skip below (mirroring classification.rs's own
            // `loop_uv_polygon_sampled` pattern, kept close per doctrine) avoids duplicating each
            // shared vertex twice — EXCEPT at a pole: a cone's apex (or a sphere's own poles) is
            // where a lune face's seam genuinely re-enters at the OTHER `u` branch (`0` in, `2π`
            // out), so skipping it here would merge the two branches into one point and let the
            // boundary "close" via a spurious diagonal across the full angular range instead of
            // tracing the real (degenerate-width-at-the-pole, but still `0..2π`-wide) rectangle —
            // same bug class as `tessellation.rs`'s `collect_loop_uv`, see the pole-branch doc below.
            if i == n - 1 && ci + 1 != coedges.len() && !is_pole {
                continue;
            }
            if surface.is_u_periodic() {
                // A pole has a meaningless `u`; unwrapping the sample AFTER it against the pole's
                // own (arbitrary) `u` would drag the departing seam back onto the arriving branch —
                // don't chain continuity through one (mirrors `tessellation.rs`'s identical fix).
                if !prev_was_pole {
                    if let Some(pu) = prev_u {
                        uv.x = unwrap_u(pu, uv.x);
                    }
                }
                prev_u = Some(uv.x);
            }
            prev_was_pole = is_pole;
            poly.push(uv);
        }
    }
    Ok(poly)
}

/// 📏 Curvature-adaptive point count for one coedge's boundary contribution: exact 2 for a
/// straight `Line`, chordal-deviation-derived for `Circle`/`Ellipse` (their own radius, or the
/// major radius as a conservative upper bound for an ellipse's tighter minor-axis curvature), a
/// generously fine fixed count for `Nurbs` (no cheap closed-form curvature here; this file's
/// quadrature is not performance-critical enough to warrant the recursive bisection tessellation's
/// `sample_curve_adaptive` uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_sample_count(body: &Body, co: &crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Coedge, chord_tol: f64) -> usize {
    let Some(edge) = body.edges.get(co.edge) else { return 8 };
    let Some(curve) = body.curves3.get(edge.curve) else { return 8 };
    let (t0, t1) = edge.range;
    let arc_range = (t1 - t0).abs();
    let radius = match curve {
        Curve3::Line { .. } => return 2,
        Curve3::Circle { radius, .. } => *radius,
        Curve3::Ellipse { major_radius, .. } => *major_radius,
        Curve3::Nurbs { .. } => return 32,
    };
    segments_for_chord_deviation(radius, arc_range, chord_tol) + 1
}

/// 📏 Exact segment count for a circular arc of `radius` spanning `arc_range` radians so the chord
/// deviates from the arc by at most `deflection`: `n = ceil(arc_range / (2·acos(1 − deflection/radius)))`
/// — same closed form as tessellation's `segments_for_chord_deviation`, minus its angular-tol term
/// (see [`loop_uv_polygon`]'s doc for why that term doesn't apply here).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn segments_for_chord_deviation(radius: f64, arc_range: f64, deflection: f64) -> usize {
    if radius <= 0.0 || arc_range <= 0.0 {
        return 1;
    }
    let d = deflection.max(1e-12).min(radius * 1.999);
    let ratio = (1.0 - d / radius).clamp(-1.0, 1.0);
    let theta = 2.0 * ratio.acos();
    if theta <= 1e-9 {
        return ((arc_range / 1e-9).ceil() as usize).clamp(1, 200_000);
    }
    ((arc_range / theta).ceil() as usize).max(1)
}

/// 📏 One coedge, sampled at `s ∈ [0,1]` in UV — pcurve when present (edge's own order, see
/// [`loop_uv_polygon`]'s doc), else the 3D edge curve reprojected via `surface_uv`, reversed for a
/// backward coedge so the walk stays physically continuous.
/// 🐛 FIX (ticket `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` FX-5, same class as W2-B's fix to
/// `classification.rs`'s own `coedge_uv_sample`): the p-curve branch used to ignore `co.forward`
/// entirely (`p0 + (p1 - p0) * s` regardless of direction) — `prange` is stored in the underlying
/// edge's OWN curve order, never reparametrized per coedge, so a `forward == false` coedge must
/// walk it from `p1` down to `p0`, exactly like the no-pcurve fallback below already does for the
/// 3D curve's `range`. Left unfixed this silently traced every reversed pcurve-bearing coedge (half
/// of any non-planar loop) BACKWARDS relative to the rest of the ring, producing a self-crossing
/// UV boundary polygon whose shoelace/ear-clip quadrature integrates the wrong region — this is
/// what made the cylinder's general-path volume come out at ~24% of the closed-form value.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_uv_sample(body: &Body, co: &crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Coedge, surface: &Surface, s: f64) -> Result<Pnt2, KernelError> {
    if let Some(pcurve_id) = co.pcurve {
        let pcurve = body.curves2.get(pcurve_id).ok_or_else(|| KernelError::MissingEntity("pcurve".into()))?;
        let (p0, p1) = co.prange;
        let p = if co.forward { p0 + (p1 - p0) * s } else { p1 - (p1 - p0) * s };
        return Ok(pcurve.eval(p));
    }
    let edge = body.edges.get(co.edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
    let (t0, t1) = edge.range;
    let t = if co.forward { t0 + (t1 - t0) * s } else { t1 - (t1 - t0) * s };
    Ok(surface_uv(surface, curve.eval(t)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_u(prev: f64, u: f64) -> f64 {
    let mut w = u;
    let pi = std::f64::consts::PI;
    while w - prev > pi {
        w -= std::f64::consts::TAU;
    }
    while w - prev < -pi {
        w += std::f64::consts::TAU;
    }
    w
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn surface_uv(surface: &Surface, p: Pnt3) -> Pnt2 {
    match surface {
        Surface::Plane { frame } => {
            let l = frame.to_local(p);
            Pnt2::new(l.x, l.y)
        }
        Surface::Cylinder { frame, radius: _ } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            Pnt2::new(u, l.z)
        }
        Surface::Cone { frame, half_angle } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let v = l.z / half_angle.tan().max(1e-15);
            Pnt2::new(u, v)
        }
        Surface::Sphere { frame, radius } => {
            let l = (p - frame.origin).normalized().unwrap_or(Vec3::Z);
            let v = l.z.clamp(-1.0, 1.0).asin();
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let _ = radius;
            Pnt2::new(u, v)
        }
        Surface::Torus { frame, major_radius, minor_radius } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let radial = (l.x * l.x + l.y * l.y).sqrt();
            let v = ((radial - *major_radius) / minor_radius.max(1e-15)).clamp(-1.0, 1.0).acos();
            Pnt2::new(u, v)
        }
        Surface::Nurbs { .. } => {
            let domain = surface.domain();
            let closest = surface_ops::closest_uv(surface, domain, p, 1e-9);
            Pnt2::new(closest.u, closest.v)
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_uv_polygon(u: f64, v: f64, poly: &[Pnt2]) -> bool {
    if poly.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let yi = poly[i].y;
        let yj = poly[j].y;
        if (yi > v) != (yj > v) {
            let xi = poly[i].x;
            let xj = poly[j].x;
            let x_int = (xj - xi) * (v - yi) / (yj - yi) + xi;
            if u < x_int {
                inside = !inside;
            }
        }
        j = i;
    }
    inside
}

// #endregion 🔖️Loops

// #region 🔖️Samples

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn face_sample_points(body: &Body, face: FaceId) -> Result<Vec<Pnt3>, KernelError> {
    let mut pts = Vec::new();
    for loop_id in body.face_loops(face) {
        for coedge in body.loop_coedges(loop_id) {
            let edge_ent = body.coedges.get(coedge).and_then(|c| body.edges.get(c.edge));
            if let Some(edge) = edge_ent {
                if let Some(curve) = body.curves3.get(edge.curve) {
                    let mid = 0.5 * (edge.range.0 + edge.range.1);
                    pts.push(curve.eval(mid));
                }
                for vid in [edge.v0, edge.v1] {
                    if let Some(v) = body.vertices.get(vid) {
                        pts.push(v.position);
                    }
                }
            }
        }
    }
    if pts.is_empty() {
        return Err(KernelError::InvalidInput("face has no samples".into()));
    }
    Ok(pts)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_bbox_for_surface(min: &mut Pnt3, max: &mut Pnt3, surface: &Surface) {
    match surface {
        Surface::Sphere { frame, radius } => {
            let c = frame.origin;
            let r = *radius;
            min.x = min.x.min(c.x - r);
            min.y = min.y.min(c.y - r);
            min.z = min.z.min(c.z - r);
            max.x = max.x.max(c.x + r);
            max.y = max.y.max(c.y + r);
            max.z = max.z.max(c.z + r);
        }
        Surface::Cylinder { frame, radius } => {
            let c = frame.origin;
            let r = *radius;
            min.x = min.x.min(c.x - r);
            min.y = min.y.min(c.y - r);
            max.x = max.x.max(c.x + r);
            max.y = max.y.max(c.y + r);
        }
        Surface::Torus { frame, major_radius, minor_radius } => {
            let c = frame.origin;
            let ext = major_radius + minor_radius;
            min.x = min.x.min(c.x - ext);
            min.y = min.y.min(c.y - ext);
            min.z = min.z.min(c.z - minor_radius);
            max.x = max.x.max(c.x + ext);
            max.y = max.y.max(c.y + ext);
            max.z = max.z.max(c.z + minor_radius);
        }
        _ => {}
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn try_analytic_sphere_volume(body: &Body, solid: SolidId) -> Option<f64> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return None;
    }
    let mut origin: Option<Pnt3> = None;
    let mut radius: Option<f64> = None;
    for fid in faces {
        let face = body.faces.get(fid)?;
        let surf = body.surfaces.get(face.surface)?;
        let Surface::Sphere { frame, radius: r } = surf else {
            return None;
        };
        match (origin, radius) {
            (None, None) => {
                origin = Some(frame.origin);
                radius = Some(*r);
            }
            (Some(o), Some(r0)) if o.distance(frame.origin) < 1e-9 * r0.max(1.0) && (r0 - r).abs() < 1e-9 * r0.max(1.0) => {}
            _ => return None,
        }
    }
    let r = radius?;
    Some(4.0 / 3.0 * std::f64::consts::PI * r * r * r)
}

/// 📏 Closest point on one face's TRIMMED surface to `point`, with the trim test applied — public
/// so `bounding-volume`'s `FaceBvh::closest_face`/`SolidBvh::closest_face` can reuse the exact
/// per-face distance behind their BVH branch-and-bound (one implementation, not a second copy).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn closest_point_on_face(body: &Body, face: FaceId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let surface = face_surface(body, face)?;
    match surface {
        Surface::Plane { .. } => closest_point_on_planar_face(body, face, point),
        _ => {
            let domain = surface.domain();
            let closest = surface_ops::closest_uv(surface, domain, point, 1e-9);
            Ok((closest.point, closest.distance))
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn closest_point_on_planar_face(body: &Body, face: FaceId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let surface = face_surface(body, face)?;
    let domain = surface.domain();
    let closest = surface_ops::closest_uv(surface, domain, point, 1e-9);
    let (d, p) = (closest.distance, closest.point);
    if point_in_face_plane(body, face, p)? {
        return Ok((p, d));
    }
    let mut best_p = p;
    let mut best_d = f64::INFINITY;
    for loop_id in body.face_loops(face) {
        for coedge in body.loop_coedges(loop_id) {
            let co = body.coedges.get(coedge).ok_or_else(|| KernelError::MissingEntity("coedge".into()))?;
            let edge = body.edges.get(co.edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
            let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
            let closest = curve_ops::closest_parameter(curve, edge.range, point, 1e-9);
            if closest.distance < best_d {
                best_d = closest.distance;
                best_p = closest.point;
            }
        }
    }
    Ok((best_p, best_d))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_face_plane(body: &Body, face: FaceId, point: Pnt3) -> Result<bool, KernelError> {
    let surface = face_surface(body, face)?;
    let Surface::Plane { frame } = surface else {
        return Ok(true);
    };
    let uv = surface_uv(surface, point);
    let Some(outer) = body.faces.get(face).and_then(|f| f.outer) else {
        return Ok(false);
    };
    // No caller-supplied chord tolerance reaches this classification helper; a small fixed
    // boundary-sampling tolerance is a reasonable default for a point-in-face boolean test (unlike
    // the mass-integral callers above, which thread the caller's own `tol`/`chord_tol` through).
    const CLASSIFY_CHORD_TOL: f64 = 1e-4;
    let boundary = loop_uv_polygon(body, outer, surface, CLASSIFY_CHORD_TOL)?;
    if !point_in_uv_polygon(uv.x, uv.y, &boundary) {
        return Ok(false);
    }
    for inner in &body.faces.get(face).unwrap().inners {
        let hole = loop_uv_polygon(body, *inner, surface, CLASSIFY_CHORD_TOL)?;
        if point_in_uv_polygon(uv.x, uv.y, &hole) {
            return Ok(false);
        }
    }
    let _ = frame;
    Ok(true)
}

// #endregion 🔖️Samples

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::ArenaId;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::{Body, Coedge, Edge, Face, Loop, Shell, Solid, Vertex};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
    use std::f64::consts::PI;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn null_coedge() -> CoedgeId {
        ArenaId::from_raw(0, 0)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn insert_vertex(body: &mut Body, position: Pnt3) -> VertexId {
        let label = body.new_label();
        body.vertices.insert(Vertex { position, tol: Tol::DEFAULT, label })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn insert_edge(body: &mut Body, curve: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId) -> EdgeId {
        let label = body.new_label();
        body.edges.insert(Edge { curve, range, v0, v1, tol: Tol::DEFAULT, label })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn make_quad_loop(body: &mut Body, face: FaceId, corners: [Pnt3; 4]) -> crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId {
        let verts: Vec<_> = corners.iter().map(|&p| insert_vertex(body, p)).collect();
        let curves: Vec<_> = (0..4)
            .map(|i| {
                let a = corners[i];
                let b = corners[(i + 1) % 4];
                body.curves3.insert(Curve3::Line { origin: a, dir: b - a })
            })
            .collect();
        let edges: Vec<_> = (0..4).map(|i| insert_edge(body, curves[i], (0.0, 1.0), verts[i], verts[(i + 1) % 4])).collect();
        let loop_id = body.loops.insert(Loop { first: null_coedge(), face });
        let coedges: Vec<_> = edges.iter().map(|&e| body.coedges.insert(Coedge { edge: e, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() })).collect();
        for i in 0..4 {
            let c = body.coedges.get_mut(coedges[i]).unwrap();
            c.next = coedges[(i + 1) % 4];
            c.prev = coedges[(i + 3) % 4];
        }
        body.loops.get_mut(loop_id).unwrap().first = coedges[0];
        loop_id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn add_planar_face(body: &mut Body, frame: Frame3, corners: [Pnt3; 4], flipped: bool) -> FaceId {
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let label = body.new_label();
        let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped, tol: Tol::DEFAULT, label });
        let loop_id = make_quad_loop(body, face, corners);
        body.faces.get_mut(face).unwrap().outer = Some(loop_id);
        face
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn make_box_solid(body: &mut Body, origin: Pnt3, w: f64, d: f64, h: f64) -> SolidId {
        let o = origin;
        let z0 = Frame3::from_normal(o, -Vec3::Z).unwrap();
        let z1 = Frame3::from_normal(o + Vec3::new(0.0, 0.0, h), Vec3::Z).unwrap();
        let y0 = Frame3::from_normal(o, -Vec3::Y).unwrap();
        let y1 = Frame3::from_normal(o + Vec3::new(0.0, d, 0.0), Vec3::Y).unwrap();
        let x0 = Frame3::from_normal(o, -Vec3::X).unwrap();
        let x1 = Frame3::from_normal(o + Vec3::new(w, 0.0, 0.0), Vec3::X).unwrap();
        let f_bottom = add_planar_face(body, z0, [o, o + Vec3::new(w, 0.0, 0.0), o + Vec3::new(w, d, 0.0), o + Vec3::new(0.0, d, 0.0)], false);
        let f_top = add_planar_face(body, z1, [o + Vec3::new(0.0, 0.0, h), o + Vec3::new(w, 0.0, h), o + Vec3::new(w, d, h), o + Vec3::new(0.0, d, h)], false);
        let f_front = add_planar_face(body, y0, [o, o + Vec3::new(w, 0.0, 0.0), o + Vec3::new(w, 0.0, h), o + Vec3::new(0.0, 0.0, h)], false);
        let f_back = add_planar_face(body, y1, [o + Vec3::new(0.0, d, 0.0), o + Vec3::new(0.0, d, h), o + Vec3::new(w, d, h), o + Vec3::new(w, d, 0.0)], false);
        let f_left = add_planar_face(body, x0, [o, o + Vec3::new(0.0, 0.0, h), o + Vec3::new(0.0, d, h), o + Vec3::new(0.0, d, 0.0)], false);
        let f_right = add_planar_face(body, x1, [o + Vec3::new(w, 0.0, 0.0), o + Vec3::new(w, d, 0.0), o + Vec3::new(w, d, h), o + Vec3::new(w, 0.0, h)], false);
        let label = body.new_label();
        let shell = body.shells.insert(Shell { faces: vec![f_bottom, f_top, f_front, f_back, f_left, f_right], label });
        let solid_label = body.new_label();
        body.solids.insert(Solid { outer: shell, inners: vec![], label: solid_label })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn make_uv_sphere(body: &mut Body, radius: f64, n_long: usize, n_lat: usize) -> SolidId {
        let frame = Frame3::WORLD;
        let surface = body.surfaces.insert(Surface::Sphere { frame, radius });
        let mut faces = Vec::new();
        for i in 0..n_lat {
            let v0 = -PI / 2.0 + PI * (i as f64) / n_lat as f64;
            let v1 = -PI / 2.0 + PI * ((i + 1) as f64) / n_lat as f64;
            for j in 0..n_long {
                let u0 = TAU * (j as f64) / n_long as f64;
                let u1 = TAU * ((j + 1) as f64) / n_long as f64;
                let corners = [sphere_corner(&frame, radius, u0, v0), sphere_corner(&frame, radius, u1, v0), sphere_corner(&frame, radius, u1, v1), sphere_corner(&frame, radius, u0, v1)];
                let label = body.new_label();
                let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: false, tol: Tol::DEFAULT, label });
                let loop_id = make_quad_loop(body, face, corners);
                body.faces.get_mut(face).unwrap().outer = Some(loop_id);
                faces.push(face);
            }
        }
        let label = body.new_label();
        let shell = body.shells.insert(Shell { faces, label });
        let solid_label = body.new_label();
        body.solids.insert(Solid { outer: shell, inners: vec![], label: solid_label })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sphere_corner(frame: &Frame3, radius: f64, u: f64, v: f64) -> Pnt3 {
        Surface::Sphere { frame: *frame, radius }.eval(u, v)
    }

    const TAU: f64 = 2.0 * PI;

    #[semio_framework_async_macros::async_test]
    async fn unit_box_volume_and_area() {
        let mut body = Body::new();
        let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        let vol = solid_volume(&body, solid, 0.1).unwrap();
        let area = solid_surface_area(&body, solid, 0.1).unwrap();
        assert!((vol - 1.0).abs() < 1e-9, "volume {vol}");
        assert!((area - 6.0).abs() < 1e-9, "area {area}");
    }

    #[semio_framework_async_macros::async_test]
    async fn box_mass_properties_and_bbox() {
        let mut body = Body::new();
        let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 2.0, 3.0, 4.0);
        let vol = solid_volume(&body, solid, 0.1).unwrap();
        assert!((vol - 24.0).abs() < 1e-8);
        let com = solid_center_of_mass(&body, solid, 0.1).unwrap();
        assert!((com.x - 1.0).abs() < 1e-8);
        assert!((com.y - 1.5).abs() < 1e-8);
        assert!((com.z - 2.0).abs() < 1e-8);
        let bb = solid_bounding_box(&body, solid).unwrap();
        assert!((bb.min.x - 0.0).abs() < 1e-9);
        assert!((bb.max.x - 2.0).abs() < 1e-9);
        assert!((bb.max.z - 4.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn edge_length_on_unit_box() {
        let mut body = Body::new();
        let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        let face = body.solid_faces(solid)[0];
        let coedge = body.loop_coedges(body.faces.get(face).unwrap().outer.unwrap())[0];
        let edge = body.coedges.get(coedge).unwrap().edge;
        let len = edge_length(&body, edge).unwrap();
        assert!((len - 1.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn sphere_volume_coarse_tessellation() {
        let mut body = Body::new();
        let r = 2.0;
        let solid = make_uv_sphere(&mut body, r, 12, 8);
        let vol = solid_volume(&body, solid, 0.15).unwrap();
        let expected = 4.0 / 3.0 * PI * r * r * r;
        assert!((vol - expected).abs() < 0.02 * expected, "vol {vol} expected {expected}");
    }

    #[semio_framework_async_macros::async_test]
    async fn distance_and_closest_point_between_boxes() {
        let mut body = Body::new();
        let a = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        let b = make_box_solid(&mut body, Pnt3::new(3.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        let d = distance_solid_solid(&body, a, b).unwrap();
        assert!((d - 2.0).abs() < 0.25, "distance {d}");
        let (cp, dist) = closest_point_on_solid(&body, b, Pnt3::new(0.5, 0.5, 0.5)).unwrap();
        assert!(dist > 1.5 && dist < 3.5, "dist {dist}");
        assert!(cp.x > 2.5);
    }

    #[semio_framework_async_macros::async_test]
    async fn box_classifies_via_the_one_authoritative_classifier() {
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::PointClassification;
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::classification::point_in_solid;
        let mut body = Body::new();
        let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        assert_eq!(point_in_solid(&body, solid, Pnt3::new(0.5, 0.5, 0.5), 1e-9).unwrap(), PointClassification::Inside);
        assert_eq!(point_in_solid(&body, solid, Pnt3::new(2.0, 2.0, 2.0), 1e-9).unwrap(), PointClassification::Outside);
    }

    #[semio_framework_async_macros::async_test]
    async fn face_area_unit_square() {
        let mut body = Body::new();
        let frame = Frame3::WORLD;
        let face = add_planar_face(&mut body, frame, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)], false);
        let area = face_area(&body, face, 0.1).unwrap();
        assert!((area - 1.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn solid_mass_properties_box_uses_the_analytic_fast_path_and_matches_closed_form() {
        let mut body = Body::new();
        let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 2.0, 3.0, 4.0);
        let mp = solid_mass_properties(&body, solid, 1e-4).unwrap();
        assert!((mp.volume - 24.0).abs() < 1e-9, "volume {}", mp.volume);
        assert!((mp.area - 52.0).abs() < 1e-9, "area {}", mp.area);
        assert!((mp.centroid.x - 1.0).abs() < 1e-9 && (mp.centroid.y - 1.5).abs() < 1e-9 && (mp.centroid.z - 2.0).abs() < 1e-9);
        let expected_ixx = mp.volume * (3.0 * 3.0 + 4.0 * 4.0) / 12.0;
        assert!((mp.inertia[0][0] - expected_ixx).abs() < 1e-6, "ixx {} expected {expected_ixx}", mp.inertia[0][0]);
        assert_eq!(mp.error_estimate, 0.0, "the box analytic fast path is exact");
    }

    #[semio_framework_async_macros::async_test]
    async fn solid_mass_properties_sphere_matches_closed_form() {
        let mut body = Body::new();
        let r = 2.0;
        let solid = make_uv_sphere(&mut body, r, 12, 8);
        let mp = solid_mass_properties(&body, solid, 1e-4).unwrap();
        let expected_vol = 4.0 / 3.0 * PI * r * r * r;
        let expected_area = 4.0 * PI * r * r;
        assert!((mp.volume - expected_vol).abs() < 0.02 * expected_vol, "vol {} expected {expected_vol}", mp.volume);
        assert!((mp.area - expected_area).abs() < 0.02 * expected_area, "area {} expected {expected_area}", mp.area);
        assert!(mp.centroid.to_vec().norm() < 1e-6, "sphere centroid should be at the origin, got {:?}", mp.centroid);
    }

    #[semio_framework_async_macros::async_test]
    async fn solid_mass_properties_cylinder_general_path_matches_closed_form_within_error_estimate() {
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_cylinder;
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let radius = 1.0;
        let height = 3.0;
        let solid = make_cylinder(&mut body, radius, height, &mut rec).unwrap();
        let mp = solid_mass_properties(&body, solid, 1e-4).unwrap();
        let expected_vol = PI * radius * radius * height;
        assert!((mp.volume - expected_vol).abs() < 0.05 * expected_vol, "vol {} expected {expected_vol}", mp.volume);
        assert!(mp.error_estimate.is_finite() && mp.error_estimate >= 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn distance_solid_solid_overlap_returns_zero_via_real_classifier() {
        let mut body = Body::new();
        let a = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0);
        let b = make_box_solid(&mut body, Pnt3::new(1.0, 1.0, 1.0), 2.0, 2.0, 2.0);
        let d = distance_solid_solid(&body, a, b).unwrap();
        assert_eq!(d, 0.0, "overlapping boxes must report zero distance");
    }
}

// #endregion 🔖️Tests

// #region 🔖️Oracle
pub mod oracle {
    //! 🔮️ Ground truth used only by tests, kept deliberately independent from the kernel's own
    //! algorithms (WFC-crate convention: a brute-force oracle catches bugs a self-referential test
    //! never could). This module grows alongside the kernel — [`Sdf`] lands in Phase 0 with the
    //! primitives it can already describe; mass-property, watertightness and shape-generator oracles
    //! land in the phases that need them.

    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Trsf;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Pnt3;

    // #region 🔖️Sdf

    /// 🔮️ A closed-form signed distance field: negative inside, zero on the boundary, positive
    /// outside. Used to probe classification and Boolean results independently of the kernel's own
    /// ray-casting/arrangement code.
    #[derive(Clone, Debug, PartialEq)]
    pub enum Sdf {
        /// 🔮️ Axis-aligned box of the given half-extents, centered at the origin before `placement`.
        Box {
            half_extents: Pnt3,
            placement: Trsf,
        },
        /// 🔮️ Sphere of the given radius, centered at the origin before `placement`.
        Sphere {
            radius: f64,
            placement: Trsf,
        },
        /// 🔮️ Cylinder of the given radius and half-height, axis along local `z`, centered at the
        /// origin before `placement`.
        Cylinder {
            radius: f64,
            half_height: f64,
            placement: Trsf,
        },
        /// 🔮️ Capped cone along local `z`, radius `radius` at `z = -half_height` tapering to apex at
        /// `z = +half_height`, centered at the origin before `placement`.
        Cone {
            radius: f64,
            half_height: f64,
            placement: Trsf,
        },
        /// 🔮️ Torus in the local `xy` plane, major circle radius `major_radius`, tube radius
        /// `minor_radius`, axis along local `z`, centered at the origin before `placement`.
        Torus {
            major_radius: f64,
            minor_radius: f64,
            placement: Trsf,
        },
        /// 🔮️ Boolean combination of two fields.
        Union(Box<Sdf>, Box<Sdf>),
        Intersect(Box<Sdf>, Box<Sdf>),
        Difference(Box<Sdf>, Box<Sdf>),
    }

    impl Sdf {
        /// 🔮️ Evaluates the field at a world-space point.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn eval(&self, p: Pnt3) -> f64 {
            match self {
                Sdf::Box { half_extents, placement } => {
                    let local = placement.inverse().apply_point(p);
                    let dx = local.x.abs() - half_extents.x;
                    let dy = local.y.abs() - half_extents.y;
                    let dz = local.z.abs() - half_extents.z;
                    let outside = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2) + dz.max(0.0).powi(2)).sqrt();
                    let inside = dx.max(dy).max(dz).min(0.0);
                    outside + inside
                }
                Sdf::Sphere { radius, placement } => {
                    let local = placement.inverse().apply_point(p);
                    local.to_vec().norm() - radius
                }
                Sdf::Cylinder { radius, half_height, placement } => {
                    let local = placement.inverse().apply_point(p);
                    let radial = (local.x * local.x + local.y * local.y).sqrt() - radius;
                    let axial = local.z.abs() - half_height;
                    let outside = (radial.max(0.0).powi(2) + axial.max(0.0).powi(2)).sqrt();
                    let inside = radial.max(axial).min(0.0);
                    outside + inside
                }
                Sdf::Cone { radius, half_height, placement } => {
                    let local = placement.inverse().apply_point(p);
                    capped_cone_z(&local, *half_height, *radius, 0.0)
                }
                Sdf::Torus { major_radius, minor_radius, placement } => {
                    let local = placement.inverse().apply_point(p);
                    let qx = (local.x * local.x + local.y * local.y).sqrt() - major_radius;
                    let qz = local.z;
                    (qx * qx + qz * qz).sqrt() - minor_radius
                }
                Sdf::Union(a, b) => a.eval(p).min(b.eval(p)),
                Sdf::Intersect(a, b) => a.eval(p).max(b.eval(p)),
                Sdf::Difference(a, b) => a.eval(p).max(-b.eval(p)),
            }
        }
        /// 🔮️ `true` when `p` is inside (or on, within `tol`) the field's boundary.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn contains(&self, p: Pnt3, tol: f64) -> bool {
            self.eval(p) <= tol
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn union(self, other: Sdf) -> Sdf {
            Sdf::Union(Box::new(self), Box::new(other))
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn intersect(self, other: Sdf) -> Sdf {
            Sdf::Intersect(Box::new(self), Box::new(other))
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn difference(self, other: Sdf) -> Sdf {
            Sdf::Difference(Box::new(self), Box::new(other))
        }
    }

    /// 🔮️ Capped cone SDF along `z` with base radius `r1` at `z = -h` and `r2` at `z = +h`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn capped_cone_z(p: &Pnt3, h: f64, r1: f64, r2: f64) -> f64 {
        let qx = (p.x * p.x + p.y * p.y).sqrt();
        let k1_x = r2;
        let k1_z = h;
        let k2_x = r2 - r1;
        let k2_z = 2.0 * h;
        let cap_r = if p.z < 0.0 { r1 } else { r2 };
        let ca_x = qx - qx.min(cap_r);
        let ca_z = p.z.abs() - h;
        let dot_k1_q = k1_x * (k1_x - qx) + k1_z * (k1_z - p.z);
        let dot_k2_k2 = k2_x * k2_x + k2_z * k2_z;
        let t = (dot_k1_q / dot_k2_k2).clamp(0.0, 1.0);
        let cb_x = qx - k1_x + k2_x * t;
        let cb_z = p.z - k1_z + k2_z * t;
        let sign = if cb_x < 0.0 && ca_z < 0.0 { -1.0 } else { 1.0 };
        let ca_len = ca_x * ca_x + ca_z * ca_z;
        let cb_len = cb_x * cb_x + cb_z * cb_z;
        sign * ca_len.min(cb_len).sqrt()
    }

    // #endregion 🔖️Sdf

    // #region 🔖️ClosedFormMass

    /// 🔮️ Closed-form volume and surface area for analytic primitives (test oracle vs the sibling `super` mass-properties module).
    pub struct ClosedFormMass;

    impl ClosedFormMass {
        /// 🔮️ Volume of an axis-aligned box with the given half-extents.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn box_volume(half_extents: Pnt3) -> f64 {
            8.0 * half_extents.x * half_extents.y * half_extents.z
        }
        /// 🔮️ Total surface area of an axis-aligned box with the given half-extents.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn box_surface_area(half_extents: Pnt3) -> f64 {
            8.0 * (half_extents.x * half_extents.y + half_extents.y * half_extents.z + half_extents.x * half_extents.z)
        }
        /// 🔮️ Volume of a sphere with the given radius.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn sphere_volume(radius: f64) -> f64 {
            (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3)
        }
        /// 🔮️ Surface area of a sphere with the given radius.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn sphere_surface_area(radius: f64) -> f64 {
            4.0 * std::f64::consts::PI * radius.powi(2)
        }
        /// 🔮️ Volume of a right circular cylinder (including caps) with radius and full height `2 * half_height`.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn cylinder_volume(radius: f64, half_height: f64) -> f64 {
            std::f64::consts::PI * radius.powi(2) * (2.0 * half_height)
        }
        /// 🔮️ Total surface area of a capped right circular cylinder.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn cylinder_surface_area(radius: f64, half_height: f64) -> f64 {
            2.0 * std::f64::consts::PI * radius * (radius + 2.0 * half_height)
        }
    }

    // #endregion 🔖️ClosedFormMass

    // #region 🔖️Watertightness

    /// 🔮️ Watertightness classification returned by the oracle checker.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WatertightnessVerdict {
        /// 🔮️ Every edge is shared by exactly two faces with consistent orientation.
        Watertight,
        /// 🔮️ At least one boundary edge remains (open shell or non-manifold rim).
        HasBoundaryEdges { count: usize },
        /// 🔮️ Topology not inspected yet (stub until sew/heal lanes wire real counts).
        NotChecked,
    }

    /// 🔮️ Summary of a watertightness probe for differential tests.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WatertightnessReport {
        pub verdict: WatertightnessVerdict,
    }

    /// 🔮️ Stub API: derives a verdict from a pre-counted boundary-edge tally supplied by future topo tests.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn watertightness_from_boundary_edge_count(boundary_edges: usize) -> WatertightnessReport {
        let verdict = if boundary_edges == 0 { WatertightnessVerdict::Watertight } else { WatertightnessVerdict::HasBoundaryEdges { count: boundary_edges } };
        WatertightnessReport { verdict }
    }

    /// 🔮️ Count edges whose coedge valence is not exactly two (boundary or non-manifold).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn count_boundary_edges(body: &Body) -> usize {
        let mut count = 0usize;
        for (edge_id, _) in body.edges.iter() {
            let valence = body.edge_coedges(edge_id).len();
            if valence != 2 {
                count += 1;
            }
        }
        count
    }

    /// 🔮️ Real watertightness probe from body topology (boundary/non-manifold edge valence).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn watertightness_of_body(body: &Body) -> WatertightnessReport {
        watertightness_from_boundary_edge_count(count_boundary_edges(body))
    }

    /// 🔮️ Compatibility alias retained for older call sites; prefer [`watertightness_of_body`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn watertightness_stub_unchecked() -> WatertightnessReport {
        WatertightnessReport { verdict: WatertightnessVerdict::NotChecked }
    }

    // #endregion 🔖️Watertightness

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn box_sdf_is_negative_inside_and_positive_outside() {
            let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement: Trsf::IDENTITY };
            assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) < 0.0);
            assert!(b.eval(Pnt3::new(5.0, 0.0, 0.0)) > 0.0);
            assert!((b.eval(Pnt3::new(1.0, 0.0, 0.0))).abs() < 1e-9);
        }

        #[semio_framework_async_macros::async_test]
        async fn sphere_sdf_matches_analytic_distance() {
            let s = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
            assert!((s.eval(Pnt3::new(5.0, 0.0, 0.0)) - 3.0).abs() < 1e-9);
            assert!((s.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-2.0)).abs() < 1e-9);
        }

        #[semio_framework_async_macros::async_test]
        async fn cylinder_sdf_is_correct_on_axis_and_cap() {
            let c = Sdf::Cylinder { radius: 1.0, half_height: 2.0, placement: Trsf::IDENTITY };
            assert!((c.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-1.0)).abs() < 1e-9);
            assert!((c.eval(Pnt3::new(0.0, 0.0, 5.0)) - 3.0).abs() < 1e-9);
        }

        #[semio_framework_async_macros::async_test]
        async fn torus_sdf_is_negative_on_major_circle_and_positive_outside_tube() {
            let t = Sdf::Torus { major_radius: 2.0, minor_radius: 0.5, placement: Trsf::IDENTITY };
            assert!(t.eval(Pnt3::new(2.0, 0.0, 0.0)) < 0.0);
            assert!((t.eval(Pnt3::new(2.5, 0.0, 0.0))).abs() < 1e-8);
            assert!(t.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
        }

        #[semio_framework_async_macros::async_test]
        async fn cone_sdf_is_negative_inside_taper_and_positive_outside() {
            let c = Sdf::Cone { radius: 1.0, half_height: 1.0, placement: Trsf::IDENTITY };
            assert!(c.eval(Pnt3::new(0.0, 0.0, -0.5)) < 0.0);
            assert!((c.eval(Pnt3::new(1.0, 0.0, -1.0))).abs() < 1e-8);
            assert!(c.eval(Pnt3::new(2.0, 0.0, 0.0)) > 0.0);
        }

        #[semio_framework_async_macros::async_test]
        async fn union_is_the_min_and_matches_containment_of_either_operand() {
            let a = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::new(-1.0, 0.0, 0.0)) };
            let b = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::new(1.0, 0.0, 0.0)) };
            let u = a.union(b);
            assert!(u.contains(Pnt3::new(-1.0, 0.0, 0.0), 1e-9));
            assert!(u.contains(Pnt3::new(1.0, 0.0, 0.0), 1e-9));
            assert!(!u.contains(Pnt3::new(5.0, 0.0, 0.0), 1e-9));
        }

        #[semio_framework_async_macros::async_test]
        async fn difference_removes_the_second_operand() {
            let big = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
            let small = Sdf::Sphere { radius: 1.0, placement: Trsf::IDENTITY };
            let d = big.difference(small);
            assert!(!d.contains(Pnt3::new(0.0, 0.0, 0.0), 1e-9));
            assert!(d.contains(Pnt3::new(1.5, 0.0, 0.0), 1e-9));
        }

        #[semio_framework_async_macros::async_test]
        async fn placed_box_sdf_respects_transform() {
            let placement = Trsf::translation(crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::new(10.0, 0.0, 0.0));
            let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement };
            assert!(b.eval(Pnt3::new(10.0, 0.0, 0.0)) < 0.0);
            assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
        }

        #[semio_framework_async_macros::async_test]
        async fn closed_form_mass_matches_textbook_box_sphere_cylinder() {
            let half = Pnt3::new(1.0, 2.0, 3.0);
            assert!((ClosedFormMass::box_volume(half) - 48.0).abs() < 1e-12);
            assert!((ClosedFormMass::box_surface_area(half) - 88.0).abs() < 1e-12);
            assert!((ClosedFormMass::sphere_volume(3.0) - 36.0 * std::f64::consts::PI).abs() < 1e-9);
            assert!((ClosedFormMass::sphere_surface_area(3.0) - 36.0 * std::f64::consts::PI).abs() < 1e-9);
            assert!((ClosedFormMass::cylinder_volume(2.0, 3.0) - 24.0 * std::f64::consts::PI).abs() < 1e-9);
            assert!((ClosedFormMass::cylinder_surface_area(2.0, 3.0) - 32.0 * std::f64::consts::PI).abs() < 1e-9);
        }

        #[semio_framework_async_macros::async_test]
        async fn watertightness_stub_classifies_boundary_edge_count() {
            let tight = watertightness_from_boundary_edge_count(0);
            assert_eq!(tight.verdict, WatertightnessVerdict::Watertight);
            let open = watertightness_from_boundary_edge_count(3);
            assert_eq!(open.verdict, WatertightnessVerdict::HasBoundaryEdges { count: 3 });
            assert_eq!(watertightness_stub_unchecked().verdict, WatertightnessVerdict::NotChecked);
        }
    }

    #[cfg(test)]
    #[test]
    fn watertightness_of_box_is_watertight() {
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box;
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let _ = solid;
        let report = watertightness_of_body(&body);
        assert_eq!(report.verdict, WatertightnessVerdict::Watertight);
    }

    // #endregion 🔖️Tests
}
// #endregion 🔖️Oracle
