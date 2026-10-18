//! ✅️ Blend validation and geometry: the solid and edge set checks, the blend request validation and the line, plane, curve parameter and face plane geometry helpers. See [`super`].

use super::*;

// #region 🔖️Validate
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_solid(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity("solid".into()));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn solid_edge_set(body: &Body, solid: SolidId) -> BTreeSet<EdgeId> {
    let mut edges = BTreeSet::new();
    for face in body.solid_faces(solid) {
        for coedge in body.face_coedges(face) {
            if let Some(c) = body.coedges.get(coedge) {
                edges.insert(c.edge);
            }
        }
    }
    edges
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn min_adjacent_edge_length(body: &Body, edge: EdgeId) -> Result<f64, KernelError> {
    let ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let mut min_len = f64::INFINITY;
    for vid in [ent.v0, ent.v1] {
        for adj in body.vertex_edges(vid) {
            if adj == edge {
                continue;
            }
            let len = edge_length(body, adj)?;
            if len > 0.0 {
                min_len = min_len.min(len);
            }
        }
    }
    if !min_len.is_finite() {
        return Err(KernelError::InvalidInput("edge has no measurable adjacent edges".into()));
    }
    Ok(min_len)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn validate_blend_request(body: &Body, solid: SolidId, edges: &[EdgeId], amount: f64) -> Result<(), KernelError> {
    require_solid(body, solid)?;
    if edges.is_empty() {
        return Err(KernelError::InvalidInput("blend requires at least one edge".into()));
    }
    if !(amount.is_finite() && amount > 0.0) {
        return Err(KernelError::InvalidInput("blend radius/distance must be positive".into()));
    }
    let solid_edges = solid_edge_set(body, solid);
    for &edge in edges {
        if !solid_edges.contains(&edge) {
            return Err(KernelError::MissingEntity(format!("edge {edge:?} is not on solid")));
        }
        let min_adj = min_adjacent_edge_length(body, edge)?;
        if amount >= min_adj {
            return Err(KernelError::InvalidInput(format!("blend amount {amount} must be smaller than min adjacent edge length {min_adj}")));
        }
    }
    Ok(())
}
// #endregion 🔖️Validate

// #region 🔖️Geometry
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn unit(v: Vec3, what: &str) -> Result<Vec3, KernelError> {
    v.normalized().ok_or_else(|| KernelError::Operation(format!("blend: {what} is degenerate")))
}

/// 🎨️ Where two COPLANAR, non-parallel lines meet — `None` when they are parallel or skew, which
/// is the precondition every corner construction here rejects rather than approximates.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn line_line_point(a0: Pnt3, da: Vec3, b0: Pnt3, db: Vec3) -> Option<Pnt3> {
    let n = da.cross(db);
    let nn = n.norm_sq();
    if nn <= DIR_EPS * DIR_EPS * da.norm_sq() * db.norm_sq() {
        return None;
    }
    let w = b0 - a0;
    let p = a0 + da * (w.cross(db).dot(n) / nn);
    let q = b0 + db * (w.cross(da).dot(n) / nn);
    if p.distance(q) > 1e-7 {
        return None;
    }
    Some(p)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn line_plane_point(origin: Pnt3, dir: Vec3, plane_point: Pnt3, plane_normal: Vec3) -> Option<Pnt3> {
    let den = dir.dot(plane_normal);
    if den.abs() <= DIR_EPS * dir.norm().max(1.0) {
        return None;
    }
    Some(origin + dir * ((plane_point - origin).dot(plane_normal) / den))
}

/// 🎨️ The curve parameter at `p`, shifted by whole periods into `range` for a periodic curve and
/// VERIFIED against the curve itself — so a point merely near the curve can never become a trim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn curve_param_at(curve: &Curve3, p: Pnt3, range: (f64, f64)) -> Option<f64> {
    let raw = match curve {
        Curve3::Line { origin, dir } => {
            let dd = dir.norm_sq();
            if dd <= 0.0 {
                return None;
            }
            (p - *origin).dot(*dir) / dd
        }
        Curve3::Circle { frame, .. } | Curve3::Ellipse { frame, .. } => {
            let local = frame.to_local(p);
            let mut t = local.y.atan2(local.x);
            let (lo, hi) = (range.0.min(range.1), range.0.max(range.1));
            while t < lo - 1e-12 {
                t += TAU;
            }
            while t > hi + 1e-12 {
                t -= TAU;
            }
            t
        }
        Curve3::Nurbs { .. } => return None,
    };
    if curve.eval(raw).distance(p) > 1e-7 {
        return None;
    }
    Some(raw)
}

/// 🎨️ A face's OUTWARD normal at `p`: the surface's own `du × dv`, negated when the face is
/// `flipped`. Constant over a planar face, radial over a cylindrical one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn outward_normal_at(body: &Body, face: FaceId, p: Pnt3) -> Result<Vec3, KernelError> {
    let fd = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = body.surfaces.get(fd.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
    let mut normal = match surface {
        Surface::Plane { frame } => frame.z,
        other => {
            let uv = closest_uv(other, other.domain(), p, BLEND_EPS);
            other.normal(uv.u, uv.v).ok_or_else(|| KernelError::Operation("blend: the face normal is degenerate at this edge".into()))?
        }
    };
    if fd.flipped {
        normal = -normal;
    }
    unit(normal, "face normal")
}

/// 🎨️ A planar face's `(point, outward normal)`, or `Err` — the precondition every corner and cap
/// trim states explicitly instead of guessing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn face_plane(body: &Body, face: FaceId) -> Result<(Pnt3, Vec3), KernelError> {
    let fd = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = body.surfaces.get(fd.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
    let Surface::Plane { frame } = surface else {
        return Err(KernelError::Operation("blend: this construction needs a planar face".into()));
    };
    Ok((frame.origin, if fd.flipped { -frame.z } else { frame.z }))
}
// #endregion 🔖️Geometry
