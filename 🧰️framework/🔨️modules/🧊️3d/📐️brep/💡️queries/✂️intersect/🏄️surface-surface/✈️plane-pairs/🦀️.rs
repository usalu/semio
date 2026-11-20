//! ✈️ Plane pairs: the exact plane intersections with planes, cylinders, cones, spheres and tori, the meridian level cases and the sphere and sphere intersection. See [`super`].

use super::*;

// #region 🔖️PlanePairs
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn plane_plane(fa: &Frame3, fb: &Frame3, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    let Some(curve3) = intersect_plane_plane_curve(fa, fb, tol)? else { return Ok(vec![]); };
    Ok(vec![finish_intcurve(curve3, &Surface::Plane { frame: *fa }, &Surface::Plane { frame: *fb }, tol)])
}

/// 📐️ Computes the unique analytic line of two supporting planes without building pcurves.
pub fn intersect_plane_plane_curve(fa: &Frame3, fb: &Frame3, tol: f64) -> Result<Option<Curve3>, LineIntersectionRefusal> {
    if !(tol.is_finite() && tol > 0.0) { return Err(LineIntersectionRefusal::InvalidTolerance); }
    let (n1, n2) = (fa.z, fb.z);
    if n1.norm() == 0.0 || n2.norm() == 0.0 { return Err(LineIntersectionRefusal::DegenerateDirection); }
    let dir = n1.cross(n2);
    let dir_n = dir.norm();
    if !dir_n.is_finite() { return Err(LineIntersectionRefusal::NonFinite); }
    if dir_n <= tol {
        let dist = n1.dot(fb.origin - fa.origin).abs();
        if !dist.is_finite() { return Err(LineIntersectionRefusal::NonFinite); }
        if dist <= tol { return Err(LineIntersectionRefusal::Infinite); }
        return Ok(None);
    }
    let d1 = n1.dot(fa.origin.to_vec());
    let d2 = n2.dot(fb.origin.to_vec());
    let point = (n2.cross(dir) * d1 + dir.cross(n1) * d2) * (1.0 / (dir_n * dir_n));
    let origin = Pnt3::new(point.x, point.y, point.z);
    let unit = dir * (1.0 / dir_n);
    if !origin.to_array().into_iter().chain(unit.to_array()).all(f64::is_finite) {
        return Err(LineIntersectionRefusal::NonFinite);
    }
    Ok(Some(Curve3::Line { origin, dir: unit }))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn plane_cylinder(plane: &Frame3, cyl: &Frame3, radius: f64, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    if radius <= tol {
        return Err(IntersectError::Degenerate("non-positive cylinder radius".into()));
    }
    let n = plane.z.normalized().unwrap_or(Vec3::Z);
    let axis = cyl.z.normalized().unwrap_or(Vec3::Z);
    let cos_theta = n.dot(axis).abs();
    let plane_surf = Surface::Plane { frame: *plane };
    let cyl_surf = Surface::Cylinder { frame: *cyl, radius };
    if cos_theta <= tol {
        return Ok(plane_cylinder_parallel(plane, cyl, radius, n, axis, tol, (&plane_surf, &cyl_surf)));
    }
    if (1.0 - cos_theta) <= tol {
        let curve3 = perpendicular_cylinder_curve(plane, cyl, radius, n, axis)
            .map_err(|error| IntersectError::Degenerate(error.message().into()))?;
        return Ok(vec![finish_intcurve(curve3, &plane_surf, &cyl_surf, tol)]);
    }
    let center = cyl.origin + axis * (n.dot(plane.origin - cyl.origin) / n.dot(axis));
    let minor = radius;
    let major = radius / cos_theta;
    let major_dir = (axis - n * axis.dot(n)).normalized().unwrap_or_else(|| n.any_orthogonal());
    let frame = Frame3::from_x_z(center, major_dir, n).ok_or_else(|| IntersectError::Degenerate("degenerate ellipse frame on cylinder".into()))?;
    let curve3 = Curve3::Ellipse { frame, major_radius: major, minor_radius: minor };
    Ok(vec![finish_intcurve(curve3, &plane_surf, &cyl_surf, tol)])
}

/// 🏄 Plane parallel to the cylinder's axis: zero, one (tangent — reported as a real one-line
/// result, not an error, per the exact-case list this DO item asks for), or two rulings.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn plane_cylinder_parallel(plane: &Frame3, cyl: &Frame3, radius: f64, n: Vec3, axis: Vec3, tol: f64, (plane_surf, cyl_surf): (&Surface, &Surface)) -> Vec<IntCurve> {
    let signed = n.dot(cyl.origin - plane.origin);
    let dist = signed.abs();
    if dist > radius + tol {
        return vec![];
    }
    let h_sq = radius * radius - dist * dist;
    if h_sq < -(tol * tol) {
        return vec![];
    }
    let h = h_sq.max(0.0).sqrt();
    let foot = cyl.origin - n * signed;
    let perp = n.cross(axis).normalized().unwrap_or_else(|| axis.any_orthogonal());
    if h <= tol {
        let curve3 = Curve3::Line { origin: foot, dir: axis };
        return vec![finish_intcurve(curve3, plane_surf, cyl_surf, tol)];
    }
    let l1 = Curve3::Line { origin: foot + perp * (-h), dir: axis };
    let l2 = Curve3::Line { origin: foot + perp * h, dir: axis };
    vec![finish_intcurve(l1, plane_surf, cyl_surf, tol), finish_intcurve(l2, plane_surf, cyl_surf, tol)]
}

/// 🏄 Plane/cone: perpendicular-to-axis (circle, via [`plane_level_case`]), tangent-to-a-ruling
/// (parabola) and cutting-both-nappes-steeper-than-a-ruling (hyperbola) fall to
/// [`general_marching`] (documented gap — see `📓️w2a-intersections.md`); the remaining bounded
/// case (a plane tilted less steeply than the cone's own rulings) is an exact ellipse.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn plane_cone(plane: &Frame3, cone: &Frame3, half_angle: f64, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    if half_angle.abs() <= tol || half_angle.abs() >= std::f64::consts::FRAC_PI_2 - tol {
        return Err(IntersectError::Degenerate("degenerate cone half-angle".into()));
    }
    let axis = cone.z.normalized().unwrap_or(Vec3::Z);
    let n = plane.z.normalized().unwrap_or(Vec3::Z);
    let cos_theta = n.dot(axis).abs();
    let cone_surf = Surface::Cone { frame: *cone, half_angle };
    if (1.0 - cos_theta) <= tol {
        return Ok(plane_level_case(plane, &cone_surf, cone.origin, axis, tol));
    }
    let angle_n_axis = cos_theta.clamp(-1.0, 1.0).acos();
    let gamma = std::f64::consts::FRAC_PI_2 - angle_n_axis;
    if gamma <= half_angle + tol {
        return Ok(general_marching(&Surface::Plane { frame: *plane }, &cone_surf, tol));
    }
    plane_cone_ellipse(plane, cone, half_angle, axis, n, tol)
}

/// 🏄 The bounded oblique cone section: substituting the plane's own principal axes (`e1` = axis
/// projected into the plane, `e2 = n × e1`) into the cone's implicit equation gives a conic with
/// no cross term (those axes *are* the ellipse's principal axes) — complete the square directly.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn plane_cone_ellipse(plane: &Frame3, cone: &Frame3, half_angle: f64, axis: Vec3, n: Vec3, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    let cone_surf = Surface::Cone { frame: *cone, half_angle };
    let plane_surf = Surface::Plane { frame: *plane };
    let fallback = || Ok(general_marching(&plane_surf, &cone_surf, tol));
    let Some(e1) = (axis - n * axis.dot(n)).normalized() else { return fallback() };
    let e2 = n.cross(e1);
    let origin_loc = cone.to_local(plane.origin);
    let e1_loc = cone.to_local_vector(e1);
    let e2_loc = cone.to_local_vector(e2);
    let k2 = half_angle.tan().powi(2);
    let quad = |ex: f64, ey: f64, ez: f64| ex * ex + ey * ey - k2 * ez * ez;
    let bilin = |e1x: f64, e1y: f64, e1z: f64, e2x: f64, e2y: f64, e2z: f64| 2.0 * (e1x * e2x + e1y * e2y - k2 * e1z * e2z);
    let a_coef = quad(e1_loc.x, e1_loc.y, e1_loc.z);
    let b_coef = quad(e2_loc.x, e2_loc.y, e2_loc.z);
    let c_coef = bilin(e1_loc.x, e1_loc.y, e1_loc.z, e2_loc.x, e2_loc.y, e2_loc.z);
    let d_coef = bilin(origin_loc.x, origin_loc.y, origin_loc.z, e1_loc.x, e1_loc.y, e1_loc.z);
    let e_coef = bilin(origin_loc.x, origin_loc.y, origin_loc.z, e2_loc.x, e2_loc.y, e2_loc.z);
    let f_coef = quad(origin_loc.x, origin_loc.y, origin_loc.z);
    if c_coef.abs() > tol.max(1e-9) * (a_coef.abs() + b_coef.abs() + 1.0) || a_coef.abs() <= tol || b_coef.abs() <= tol {
        return fallback();
    }
    let s0 = -d_coef / (2.0 * a_coef);
    let w0 = -e_coef / (2.0 * b_coef);
    let rhs = a_coef * s0 * s0 + b_coef * w0 * w0 - f_coef;
    if rhs <= tol {
        return Ok(vec![]);
    }
    let major_sq = rhs / a_coef;
    let minor_sq = rhs / b_coef;
    if major_sq <= 0.0 || minor_sq <= 0.0 {
        return fallback();
    }
    let (major_r, minor_r, x_axis_dir) = if major_sq >= minor_sq { (major_sq.sqrt(), minor_sq.sqrt(), e1) } else { (minor_sq.sqrt(), major_sq.sqrt(), e2) };
    let center = plane.origin + e1 * s0 + e2 * w0;
    if cone.to_local(center).z < -tol {
        return Ok(vec![]);
    }
    let Some(frame) = Frame3::from_x_z(center, x_axis_dir, n) else { return fallback() };
    let curve3 = Curve3::Ellipse { frame, major_radius: major_r, minor_radius: minor_r };
    Ok(vec![finish_intcurve(curve3, &plane_surf, &cone_surf, tol)])
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn plane_sphere(plane: &Frame3, sphere: &Frame3, radius: f64, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    let Some(curve3) = plane_sphere_curve(plane, sphere, radius, tol).map_err(|error| match error {
        ExactSurfaceRefusal::Tangent => IntersectError::Tangent,
        ExactSurfaceRefusal::Radius => IntersectError::Degenerate("sphere radius must be positive".into()),
        _ => IntersectError::Degenerate(error.message().into()),
    })? else { return Ok(vec![]); };
    Ok(vec![finish_intcurve(curve3, &Surface::Plane { frame: *plane }, &Surface::Sphere { frame: *sphere, radius }, tol)])
}

/// 🌐️ Failure from a bounded continuous surface intersection.
pub enum ExactSurfaceRefusal {
    Linear(LineIntersectionRefusal),
    Tangent,
    Radius,
    PlaneNormal,
    NonFinite,
    Unsupported,
}

impl ExactSurfaceRefusal {
    pub fn message(&self) -> &'static str {
        match self {
            Self::Linear(error) => error.message(),
            Self::Tangent => "tangent configuration",
            Self::Radius => "sphere or cylinder radius must be positive",
            Self::PlaneNormal => "plane normal degenerate",
            Self::NonFinite => "surface intersection requires finite arithmetic",
            Self::Unsupported => "retained surface intersection requires two planes, a plane and sphere, or a perpendicular plane and cylinder",
        }
    }
}

/// ✂️ Borrows two analytic supports for an allocation-free single continuous intersection.
pub fn intersect_original_analytic_surfaces(a: &Surface, b: &Surface, tol: f64) -> Result<Option<Curve3>, ExactSurfaceRefusal> {
    use ExactSurfaceRefusal::*;
    if !(tol.is_finite() && tol > 0.) { return Err(Linear(LineIntersectionRefusal::InvalidTolerance)); }
    match (a,b) {
        (Surface::Plane { frame: a }, Surface::Plane { frame: b }) => intersect_plane_plane_curve(a, b, tol).map_err(Linear),
        (Surface::Plane { frame: plane }, Surface::Sphere { frame: sphere, radius })
        | (Surface::Sphere { frame: sphere, radius }, Surface::Plane { frame: plane }) => plane_sphere_curve(plane, sphere, *radius, tol),
        (Surface::Plane { frame: plane }, Surface::Cylinder { frame: cylinder, radius })
        | (Surface::Cylinder { frame: cylinder, radius }, Surface::Plane { frame: plane }) => {
            if !radius.is_finite() || *radius <= tol { return Err(Radius); }
            let normal = plane.z.normalized().ok_or(PlaneNormal)?;
            let axis = cylinder.z.normalized().ok_or(NonFinite)?;
            if normal.cross(axis).norm() > 16. * f64::EPSILON { return Err(Unsupported); }
            perpendicular_cylinder_curve(plane, cylinder, *radius, normal, axis).map(Some)
        }
        _ => Err(Unsupported),
    }
}

fn perpendicular_cylinder_curve(plane: &Frame3, cylinder: &Frame3, radius: f64, normal: Vec3, axis: Vec3) -> Result<Curve3, ExactSurfaceRefusal> {
    let along = normal.dot(plane.origin - cylinder.origin) / normal.dot(axis);
    let center = cylinder.origin + axis * along;
    if !center.to_array().into_iter().all(f64::is_finite) { return Err(ExactSurfaceRefusal::NonFinite); }
    let frame = Frame3::from_normal(center, axis).ok_or(ExactSurfaceRefusal::NonFinite)?;
    Ok(Curve3::Circle { frame, radius })
}

fn plane_sphere_curve(plane: &Frame3, sphere: &Frame3, radius: f64, tol: f64) -> Result<Option<Curve3>, ExactSurfaceRefusal> {
    use ExactSurfaceRefusal::*;
    if !(radius.is_finite() && radius > tol) { return Err(Radius); }
    let n_n = plane.z.norm();
    if !n_n.is_finite() || n_n <= tol { return Err(PlaneNormal); }
    let normal = plane.z * n_n.recip();
    let distance = normal.dot(sphere.origin - plane.origin);
    if !distance.is_finite() { return Err(NonFinite); }
    if distance.abs() > radius + tol { return Ok(None); }
    let squared = radius * radius - distance * distance;
    if !squared.is_finite() { return Err(NonFinite); }
    if squared <= tol * tol { return Err(Tangent); }
    let center = sphere.origin - normal * distance;
    if !center.to_array().into_iter().all(f64::is_finite) { return Err(NonFinite); }
    let frame = Frame3::from_x_z(center, plane.x, normal).ok_or(PlaneNormal)?;
    Ok(Some(Curve3::Circle { frame, radius: squared.sqrt() }))
}

/// 🏄 Plane/torus: perpendicular-to-axis (up to two circles, via [`plane_level_case`]) and
/// axis-containing (exactly two tube circles) are exact; oblique sections (Villarceau included)
/// fall to [`general_marching`] (documented gap).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn plane_torus(plane: &Frame3, torus: &Frame3, major_radius: f64, minor_radius: f64, tol: f64) -> Vec<IntCurve> {
    let axis = torus.z.normalized().unwrap_or(Vec3::Z);
    let n = plane.z.normalized().unwrap_or(Vec3::Z);
    let cos_theta = n.dot(axis).abs();
    let torus_surf = Surface::Torus { frame: *torus, major_radius, minor_radius };
    let plane_surf = Surface::Plane { frame: *plane };
    if (1.0 - cos_theta) <= tol {
        return plane_level_case(plane, &torus_surf, torus.origin, axis, tol);
    }
    if cos_theta <= tol {
        let offset = n.dot(torus.origin - plane.origin);
        if offset.abs() > tol {
            return general_marching(&plane_surf, &torus_surf, tol);
        }
        let Some(radial) = axis.cross(n).normalized() else { return general_marching(&plane_surf, &torus_surf, tol) };
        let mut out = Vec::new();
        for sign in [1.0, -1.0] {
            let center = torus.origin + radial * (major_radius * sign);
            let Some(frame) = Frame3::from_x_z(center, axis, n) else { continue };
            let curve3 = Curve3::Circle { frame, radius: minor_radius };
            out.push(finish_intcurve(curve3, &plane_surf, &torus_surf, tol));
        }
        return out;
    }
    general_marching(&plane_surf, &torus_surf, tol)
}

/// 🏄 Solves a plane ⊥ an axisymmetric surface's axis against that surface's meridian profile —
/// shared by the perpendicular sub-case of `plane_cone`/`plane_torus` (`plane_cylinder`'s own
/// perpendicular case stays a direct formula, unchanged, to avoid regressing its tests).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn plane_level_case(plane: &Frame3, surface: &Surface, axis_pt: Pnt3, axis_dir: Vec3, tol: f64) -> Vec<IntCurve> {
    let g = axis_dir.dot(plane.origin - axis_pt);
    let radii = meridian_radii_at_level(surface, g, tol);
    let plane_surf = Surface::Plane { frame: *plane };
    let mut out = Vec::new();
    for rho in radii {
        if rho <= tol {
            continue;
        }
        let center = axis_pt + axis_dir * g;
        let radial = axis_dir.any_orthogonal();
        let Some(frame) = Frame3::from_x_z(center, radial, axis_dir) else { continue };
        let curve3 = Curve3::Circle { frame, radius: rho };
        out.push(finish_intcurve(curve3, &plane_surf, surface, tol));
    }
    out
}

/// 🏄 `surface`'s meridian radius (or radii, for a torus) at global axial level `g`, using the
/// surface's own frame directly (so `g` must already be measured from that frame's origin).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn meridian_radii_at_level(surface: &Surface, g: f64, tol: f64) -> Vec<f64> {
    match surface {
        Surface::Cylinder { radius, .. } => vec![*radius],
        Surface::Cone { half_angle, .. } => {
            if g >= -tol {
                vec![g.max(0.0) * half_angle.tan()]
            } else {
                vec![]
            }
        }
        Surface::Sphere { radius, .. } => {
            if g.abs() <= radius + tol {
                vec![(radius * radius - g * g).max(0.0).sqrt()]
            } else {
                vec![]
            }
        }
        Surface::Torus { major_radius, minor_radius, .. } => {
            if g.abs() > minor_radius + tol {
                return vec![];
            }
            let h = (minor_radius * minor_radius - g * g).max(0.0).sqrt();
            let mut out = Vec::new();
            for rho in [major_radius - h, major_radius + h] {
                if rho >= -tol && out.iter().all(|&r: &f64| (r - rho).abs() > tol) {
                    out.push(rho.max(0.0));
                }
            }
            out
        }
        Surface::Plane { .. } | Surface::Nurbs { .. } => vec![],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn sphere_sphere(fa: &Frame3, ra: f64, fb: &Frame3, rb: f64, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    if !(ra.is_finite() && rb.is_finite() && ra > tol && rb > tol) {
        return Err(IntersectError::Degenerate("sphere radii must be positive".into()));
    }
    let d_vec = fb.origin - fa.origin;
    let d = d_vec.norm();
    if d <= tol {
        return if (ra - rb).abs() <= tol { Err(IntersectError::Unresolved("coincident spheres".into())) } else { Ok(vec![]) };
    }
    if d > ra + rb + tol || d + tol < (ra - rb).abs() {
        return Ok(vec![]);
    }
    let dir = d_vec * (1.0 / d);
    let a = (ra * ra - rb * rb + d * d) / (2.0 * d);
    let h2 = ra * ra - a * a;
    if h2 <= tol * tol {
        return Err(IntersectError::Tangent);
    }
    let h = h2.sqrt();
    let center = fa.origin + dir * a;
    let x = dir.cross(Vec3::Z).normalized().or_else(|| dir.cross(Vec3::Y).normalized()).ok_or_else(|| IntersectError::Degenerate("sphere intersection frame".into()))?;
    let y = dir.cross(x);
    let curve3 = Curve3::Circle { frame: Frame3 { origin: center, x, y, z: dir }, radius: h };
    Ok(vec![finish_intcurve(curve3, &Surface::Sphere { frame: *fa, radius: ra }, &Surface::Sphere { frame: *fb, radius: rb }, tol)])
}
// #endregion 🔖️PlanePairs
