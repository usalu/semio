//! 📐️ Solid mass properties: signed volume, surface area, center of mass, bounding box and the full mass properties of a solid with their moment accumulation, the analytic sphere and box shortcuts and the inertia rotation. See [`super`].

use super::*;

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
    solid_mass_with_area_centroid(body, solid, tol).map(|(mass, _)| mass)
}

/// 📏 [`solid_mass_properties`] plus the centroid of the solid's boundary surface (the area-weighted mean of
/// every point on its faces), from the same single quadrature pass.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solid_mass_with_area_centroid(body: &Body, solid: SolidId, tol: f64) -> Result<(MassProperties, Pnt3), KernelError> {
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity("solid".into()));
    }
    if !(tol.is_finite() && tol > 0.0) {
        return Err(KernelError::InvalidInput("tolerance must be positive and finite".into()));
    }
    if let Some(mp) = try_analytic_sphere_mass(body, solid) {
        return Ok((mp, mp.centroid));
    }
    if let Some(mp) = try_analytic_box_properties(body, solid) {
        return Ok((mp, mp.centroid));
    }
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let (totals, err) = faces_moments(body, &faces, tol)?;
    Ok((mass_from_moments(&totals, err)?, area_centroid_from_moments(&totals).unwrap_or(Pnt3::new(0.0, 0.0, 0.0))))
}

/// 📏 Summed trimmed surface-integral moments of `faces` and their propagated error estimate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn faces_moments(body: &Body, faces: &[FaceId], tol: f64) -> Result<([f64; MOMENT_COMPONENTS], f64), KernelError> {
    let mut totals = [0.0; MOMENT_COMPONENTS];
    let mut err = 0.0;
    for &face in faces {
        let (m, e) = face_moments_general(body, face, tol)?;
        for i in 0..MOMENT_COMPONENTS {
            totals[i] += m[i];
        }
        err += e;
    }
    Ok((totals, err))
}

/// 📏 Total boundary area held in `totals`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn area_from_moments(totals: &[f64; MOMENT_COMPONENTS]) -> f64 {
    totals[IDX_AREA]
}

/// 📏 Area-weighted centroid of the surface held in `totals`, `None` for a zero area.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn area_centroid_from_moments(totals: &[f64; MOMENT_COMPONENTS]) -> Option<Pnt3> {
    let area = totals[IDX_AREA];
    (area.abs() > 1e-300).then(|| Pnt3::new(totals[IDX_AMX] / area, totals[IDX_AMY] / area, totals[IDX_AMZ] / area))
}

/// 📏 Volume, centroid and centroid inertia held in `totals`; a net-inward orientation is folded into a positive volume.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mass_from_moments(totals: &[f64; MOMENT_COMPONENTS], err: f64) -> Result<MassProperties, KernelError> {
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
    Ok(MassProperties { volume, area: totals[IDX_AREA], centroid: Pnt3::new(cx, cy, cz), inertia: [[ixx, -ixy, -ixz], [-ixy, iyy, -iyz], [-ixz, -iyz, izz]], error_estimate })
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
        let poly = loop_uv_polygon_with(body, outer, surface, tol, true)?;
        let (m, e) = loop_moments(surface, flipped, &poly, tol);
        for i in 0..MOMENT_COMPONENTS {
            total[i] += m[i];
        }
        err += e;
    }
    for &inner in &face_ent.inners {
        let poly = loop_uv_polygon_with(body, inner, surface, tol, true)?;
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
