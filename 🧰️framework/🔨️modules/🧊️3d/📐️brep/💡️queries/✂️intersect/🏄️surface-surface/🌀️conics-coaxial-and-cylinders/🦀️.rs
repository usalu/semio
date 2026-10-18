//! 🌀️ Conics, coaxial pairs and cylinders: the conic solver, the coaxial meridian profile intersection and the cylinder and cylinder cases with their Steinmetz ellipses. See [`super`].

use super::*;

// #region 🔖️Coaxial
/// 🏄 `A·ρ² + D·ρ + B·z² + E·z + F = 0` — every meridian profile here (cylinder/cone lines,
/// sphere/torus circles) has no `ρ·z` cross term, so this reduced form suffices.
#[derive(Clone, Copy)]
struct Conic {
    pub(super) a: f64,
    pub(super) b: f64,
    pub(super) d: f64,
    pub(super) e: f64,
    pub(super) f: f64,
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn native_conic(surface: &Surface) -> Conic {
    match surface {
        Surface::Cylinder { radius, .. } => Conic { a: 0.0, b: 0.0, d: 1.0, e: 0.0, f: -radius },
        Surface::Cone { half_angle, .. } => Conic { a: 0.0, b: 0.0, d: 1.0, e: -half_angle.tan(), f: 0.0 },
        Surface::Sphere { radius, .. } => Conic { a: 1.0, b: 1.0, d: 0.0, e: 0.0, f: -radius * radius },
        Surface::Torus { major_radius, minor_radius, .. } => Conic { a: 1.0, b: 1.0, d: -2.0 * major_radius, e: 0.0, f: major_radius * major_radius - minor_radius * minor_radius },
        Surface::Plane { .. } | Surface::Nurbs { .. } => Conic { a: 0.0, b: 0.0, d: 0.0, e: 0.0, f: 0.0 },
    }
}

/// 🏄 Substitutes `ζ_local = p·z_global + q` (the affine map from a surface's own axial
/// coordinate to the shared global one) into its native conic.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn substitute_affine(c: Conic, p: f64, q: f64) -> Conic {
    Conic { a: c.a, d: c.d, b: c.b * p * p, e: 2.0 * c.b * p * q + c.e * p, f: c.b * q * q + c.e * q + c.f }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn solve_quad(a: f64, b: f64, c: f64, tol: f64) -> Vec<f64> {
    if a.abs() <= tol * tol {
        if b.abs() <= tol * tol {
            return vec![];
        }
        return vec![-c / b];
    }
    let disc = b * b - 4.0 * a * c;
    if disc < -(tol * tol) {
        return vec![];
    }
    let sq = disc.max(0.0).sqrt();
    if sq <= tol {
        return vec![-b / (2.0 * a)];
    }
    vec![(-b + sq) / (2.0 * a), (-b - sq) / (2.0 * a)]
}

/// 🏄 Solves two meridian conics (in the shared `(ρ, z)` half-plane) simultaneously: two lines →
/// direct 2×2 solve; one line + one circle → substitute; two circles → subtract (their `A=B=1`
/// leading terms always cancel here) to get a radical line, then substitute.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn solve_conic_pair(ca: Conic, cb: Conic, tol: f64) -> Vec<(f64, f64)> {
    if ca.a.abs() <= tol && cb.a.abs() <= tol {
        let det = ca.d * cb.e - ca.e * cb.d;
        if det.abs() <= tol * tol {
            return vec![];
        }
        let rho = (-ca.f * cb.e + ca.e * cb.f) / det;
        let z = (ca.d * -cb.f - -ca.f * cb.d) / det;
        return vec![(rho, z)];
    }
    let (line, circ) = if ca.a.abs() <= tol {
        (ca, cb)
    } else if cb.a.abs() <= tol {
        (cb, ca)
    } else {
        (Conic { a: 0.0, b: 0.0, d: ca.d - cb.d, e: ca.e - cb.e, f: ca.f - cb.f }, ca)
    };
    if line.d.abs() > tol {
        let m = -line.e / line.d;
        let k = -line.f / line.d;
        let aa = circ.a * m * m + circ.b;
        let bb = 2.0 * circ.a * m * k + circ.d * m + circ.e;
        let cc = circ.a * k * k + circ.d * k + circ.f;
        return solve_quad(aa, bb, cc, tol).into_iter().map(|z| (m * z + k, z)).collect();
    }
    if line.e.abs() > tol {
        let z0 = -line.f / line.e;
        let aa = circ.a;
        let bb = circ.d;
        let cc = circ.b * z0 * z0 + circ.e * z0 + circ.f;
        return solve_quad(aa, bb, cc, tol).into_iter().map(|rho| (rho, z0)).collect();
    }
    vec![]
}

/// 🏄 Cylinder/cone/sphere/torus mutual pairs that share an axis: reduces to intersecting the two
/// meridian profiles in the `(ρ, z)` half-plane, `z` measured globally from `a`'s own origin along
/// its own axis. `None` when the surfaces aren't actually coaxial (caller falls back).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn coaxial_case(a: &Surface, b: &Surface, tol: f64) -> Option<Result<Vec<IntCurve>, IntersectError>> {
    let (pa, da) = super::super::shared::axis_of(a)?;
    let (pb, db) = super::super::shared::axis_of(b)?;
    if da.cross(db).norm() > tol {
        return None;
    }
    let off = pb - pa;
    let along = off.dot(da);
    let perp = off - da * along;
    if perp.norm() > tol {
        return None;
    }
    let sign_b = if da.dot(db) >= 0.0 { 1.0 } else { -1.0 };
    let ca = native_conic(a);
    let cb = substitute_affine(native_conic(b), sign_b, -sign_b * along);
    let mut sols = solve_conic_pair(ca, cb, tol);
    sols.retain(|&(rho, z)| {
        if rho < -tol {
            return false;
        }
        let a_ok = if matches!(a, Surface::Cone { .. }) { z >= -tol } else { true };
        let b_ok = if matches!(b, Surface::Cone { .. }) { sign_b * (z - along) >= -tol } else { true };
        a_ok && b_ok
    });
    sols.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal));
    sols.dedup_by(|x, y| (x.0 - y.0).abs() <= tol && (x.1 - y.1).abs() <= tol);
    let radial0 = da.any_orthogonal();
    let mut out = Vec::new();
    for (rho, z) in sols {
        if rho <= tol {
            continue;
        }
        let center = pa + da * z;
        let Some(frame) = Frame3::from_x_z(center, radial0, da) else { continue };
        let curve3 = Curve3::Circle { frame, radius: rho };
        out.push(finish_intcurve(curve3, a, b, tol));
    }
    Some(Ok(out))
}
// #endregion 🔖️Coaxial

// #region 🔖️Cylinders
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn cylinder_cylinder(a: &Surface, b: &Surface, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    let (Surface::Cylinder { frame: fa, radius: ra }, Surface::Cylinder { frame: fb, radius: rb }) = (a, b) else {
        return Err(IntersectError::Degenerate("cylinder_cylinder called with non-cylinder surface".into()));
    };
    let axis_a = fa.z.normalized().unwrap_or(Vec3::Z);
    let axis_b = fb.z.normalized().unwrap_or(Vec3::Z);
    let cross = axis_a.cross(axis_b);
    if cross.norm() <= tol {
        let off = fb.origin - fa.origin;
        let along = off.dot(axis_a);
        let perp = off - axis_a * along;
        if perp.norm() <= tol {
            return coaxial_case(a, b, tol).unwrap_or_else(|| Ok(vec![]));
        }
        return Ok(cylinder_cylinder_parallel(fa, *ra, fb, *rb, axis_a, perp, tol));
    }
    let (_, _, dist) = closest_points_on_lines(fa.origin, axis_a, fb.origin, axis_b);
    if dist <= tol && (ra - rb).abs() <= tol {
        let meet = fa.origin + axis_a * closest_points_on_lines(fa.origin, axis_a, fb.origin, axis_b).0;
        return steinmetz((fa, *ra, axis_a), (fb, *rb, axis_b), meet, tol);
    }
    Ok(general_marching(a, b, tol))
}

/// 🏄 Closest points between two infinite lines `(p1+t·d1)`/`(p2+s·d2)` (both `d` unit) as
/// `(t, s, distance)` — falls back to `t=s=0` when the lines are parallel (caller only uses the
/// distance in that branch).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn closest_points_on_lines(p1: Pnt3, d1: Vec3, p2: Pnt3, d2: Vec3) -> (f64, f64, f64) {
    let w0 = p1 - p2;
    let a = d1.dot(d1);
    let b = d1.dot(d2);
    let c = d2.dot(d2);
    let d = d1.dot(w0);
    let e = d2.dot(w0);
    let denom = a * c - b * b;
    if denom.abs() <= 1e-300 {
        return (0.0, 0.0, w0.cross(d1).norm() / a.sqrt().max(1e-300));
    }
    let t = (b * e - c * d) / denom;
    let s = (a * e - b * d) / denom;
    let pt = p1 + d1 * t;
    let ps = p2 + d2 * s;
    (t, s, pt.distance(ps))
}

/// 🏄 Parallel, non-coaxial cylinders: the cross-section circle/circle intersection extruded
/// along the shared axis direction — zero, one (tangent) or two rulings.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn cylinder_cylinder_parallel(fa: &Frame3, ra: f64, fb: &Frame3, rb: f64, axis: Vec3, perp: Vec3, tol: f64) -> Vec<IntCurve> {
    let d = perp.norm();
    let e1 = if d > tol { perp * (1.0 / d) } else { axis.any_orthogonal() };
    let e2 = axis.cross(e1);
    let pts2 = circle_circle_2d(0.0, 0.0, ra, d, 0.0, rb, tol);
    let cyl_a = Surface::Cylinder { frame: *fa, radius: ra };
    let cyl_b = Surface::Cylinder { frame: *fb, radius: rb };
    let mut out = Vec::new();
    for (x, y) in pts2 {
        let origin = fa.origin + e1 * x + e2 * y;
        let curve3 = Curve3::Line { origin, dir: axis };
        out.push(finish_intcurve(curve3, &cyl_a, &cyl_b, tol));
    }
    out
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn circle_circle_2d(cx1: f64, cy1: f64, r1: f64, cx2: f64, cy2: f64, r2: f64, tol: f64) -> Vec<(f64, f64)> {
    let dx = cx2 - cx1;
    let dy = cy2 - cy1;
    let d = (dx * dx + dy * dy).sqrt();
    if d <= tol || d > r1 + r2 + tol || d + tol < (r1 - r2).abs() {
        return vec![];
    }
    let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
    let h = (r1 * r1 - a * a).max(0.0).sqrt();
    let mx = cx1 + dx * (a / d);
    let my = cy1 + dy * (a / d);
    let ux = dx / d;
    let uy = dy / d;
    if h <= tol {
        return vec![(mx, my)];
    }
    vec![(mx - uy * h, my + ux * h), (mx + uy * h, my - ux * h)]
}

/// 🏄 Equal-radius intersecting-axis cylinders (Steinmetz): a point on both surfaces satisfies
/// `(p·a)² = (p·b)²` (subtracting the two `|p−proj|²=R²` equations), i.e. it lies on one of the
/// two bisector planes `p·(a−b)=0`/`p·(a+b)=0` through the axes' meeting point — each plane's
/// section of cylinder A (via [`plane_cylinder`]) is one of the two Steinmetz ellipses.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn steinmetz((fa, ra, axis_a): (&Frame3, f64, Vec3), (fb, rb, axis_b): (&Frame3, f64, Vec3), meet: Pnt3, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    let cyl_a = Surface::Cylinder { frame: *fa, radius: ra };
    let cyl_b = Surface::Cylinder { frame: *fb, radius: rb };
    let mut out = Vec::new();
    for n in [(axis_a - axis_b).normalized(), (axis_a + axis_b).normalized()] {
        let Some(n) = n else { continue };
        let Some(plane) = Frame3::from_normal(meet, n) else { continue };
        let pieces = plane_cylinder(&plane, fa, ra, tol)?;
        for piece in pieces {
            out.push(finish_intcurve(piece.curve3, &cyl_a, &cyl_b, tol));
        }
    }
    Ok(out)
}
// #endregion 🔖️Cylinders
