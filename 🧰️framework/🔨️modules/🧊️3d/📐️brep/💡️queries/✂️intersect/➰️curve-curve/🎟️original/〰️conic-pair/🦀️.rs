//! ⭕️ Fixed-capacity conic contacts with periodic edge trims.
use super::{conic_parameter, periodic_angle, Contacts, Curve3, CurveCurveHit, Frame3, LineIntersectionRefusal, Pnt3, Trim, Vec3, TAU};
use super::super::super::curve_surface::{conic_plane_equation, solve_trig_parameters};
use LineIntersectionRefusal::{DegenerateConic, Infinite, NonFinite, UnresolvedContact};

#[path = "👥️coplanar/🦀️.rs"]
mod coplanar;

const ANGULAR_ROUNDOFF: f64 = 64.0 * f64::EPSILON;

#[derive(Clone, Copy)]
pub(super) struct ConicSupport<'a> {
    pub(super) curve: &'a Curve3,
    pub(super) frame: &'a Frame3,
    pub(super) radii: [f64; 2],
    pub(super) trim: Trim,
}

impl<'a> ConicSupport<'a> {
    pub(super) fn from_curve(curve: &'a Curve3, trim: Trim) -> Option<Self> {
        let (frame, radii) = match curve {
            Curve3::Circle { frame, radius } => (frame, [*radius; 2]),
            Curve3::Ellipse { frame, major_radius, minor_radius } => (frame, [*major_radius, *minor_radius]),
            _ => return None,
        };
        Some(Self { curve, frame, radii, trim })
    }

    pub(super) fn validate(self, tol: f64) -> Result<(), LineIntersectionRefusal> {
        self.radii.into_iter().all(|r| r.is_finite() && r > tol).then_some(()).ok_or(DegenerateConic)
    }

    fn angle(self, point: Pnt3) -> f64 {
        let local = self.frame.to_local(point);
        (local.y / self.radii[1]).atan2(local.x / self.radii[0])
    }

    fn unit_tangent(self, parameter: f64) -> Result<Vec3, LineIntersectionRefusal> {
        let direction = self.curve.d1(parameter);
        let length = direction.x.hypot(direction.y).hypot(direction.z);
        if !length.is_finite() || length == 0.0 { return Err(NonFinite); }
        Ok(Vec3::new(direction.x / length, direction.y / length, direction.z / length))
    }

    fn interval(self) -> (f64, f64) {
        let (lo, hi) = self.trim.unwrap_or((0.0, TAU));
        (periodic_angle(lo), (hi - lo).min(TAU))
    }
}

/// 🧭️ Restricts the possible contacts to both owning angular intervals.
pub(super) fn conic_contacts(a: (&Curve3, Trim), b: (&Curve3, Trim), tol: f64) -> Result<Option<Contacts>, LineIntersectionRefusal> {
    let (Some(a), Some(b)) = (ConicSupport::from_curve(a.0, a.1), ConicSupport::from_curve(b.0, b.1)) else { return Ok(None); };
    a.validate(tol)?;
    b.validate(tol)?;
    let angular_separation = a.frame.z.cross(b.frame.z).norm();
    if !angular_separation.is_finite() { return Err(NonFinite); }
    let mut hits = [None; 4];
    if angular_separation > ANGULAR_ROUNDOFF {
        spatial_contacts(a, b, tol, &mut hits)?;
    } else {
        let center = a.frame.to_local(b.frame.origin);
        let distance = center.x.hypot(center.y);
        if ![center.z, distance].into_iter().all(f64::is_finite) { return Err(NonFinite); }
        if center.z.abs() > tol { return Ok(Some(hits)); }
        if !matches!((a.curve, b.curve), (Curve3::Circle { .. }, Curve3::Circle { .. })) {
            coplanar::contacts(a, b, tol, &mut hits)?;
        } else if distance <= center.z.abs() * ANGULAR_ROUNDOFF {
            if (a.radii[0] - b.radii[0]).hypot(center.z) <= tol {
                coincident_contacts(a, b, tol, &mut hits)?;
            }
        } else {
            distinct_contacts(a, b, [center.x / distance, center.y / distance], distance, tol, &mut hits)?;
        }
    }
    hits.sort_unstable_by(|a, b| {
        let parameter = |hit: &Option<CurveCurveHit>| hit.map_or(f64::INFINITY, |hit| hit.t_a);
        parameter(a).total_cmp(&parameter(b))
    });
    Ok(Some(hits))
}

fn spatial_contacts(a: ConicSupport<'_>, b: ConicSupport<'_>, tol: f64, hits: &mut Contacts) -> Result<(), LineIntersectionRefusal> {
    let [x, y, offset] = conic_plane_equation(a.frame, b.frame);
    let in_plane = x.hypot(y);
    if ![offset, in_plane].into_iter().all(f64::is_finite) || in_plane == 0.0 { return Err(NonFinite); }
    let (x, y) = (a.radii[0] * (x / in_plane), a.radii[1] * (y / in_plane));
    let amplitude = x.hypot(y);
    let distance = offset / in_plane;
    if distance.abs() - amplitude > tol { return Ok(()); }
    let parameters = solve_trig_parameters(x, y, distance.clamp(-amplitude, amplitude), 0.0)?;
    for angle in parameters.into_iter().flatten() {
        add_contact(a, b, a.curve.eval(angle), tol, None, hits)?;
    }
    Ok(())
}

fn distinct_contacts(a: ConicSupport<'_>, b: ConicSupport<'_>, [x,y]: [f64; 2], distance: f64, tol: f64, hits: &mut Contacts) -> Result<(), LineIntersectionRefusal> {
    let scale = a.radii[0].max(b.radii[0]).max(distance);
    let d = distance / scale;
    let r1 = a.radii[0] / scale;
    let r2 = b.radii[0] / scale;
    let margin = tol / scale;
    let radius_delta = a.radii[0] - b.radii[0];
    let sum = r1 + r2;
    if d > sum + margin || radius_delta.abs() - distance > tol { return Ok(()); }
    let ratio = radius_delta / distance;
    let along = (ratio * sum + d) * 0.5;
    let height = 0.5 * ((sum + d) * (sum - d).max(0.0) * (1.0 - ratio.abs()).max(0.0) * (1.0 + ratio.abs())).sqrt() * scale;
    if ![along, height].into_iter().all(f64::is_finite) { return Err(NonFinite); }
    let offset = along * scale;
    let center = a.frame.origin + (a.frame.x * x + a.frame.y * y) * offset;
    let tangent = distance == a.radii[0] + b.radii[0] || distance == radius_delta.abs() || height <= tol * 0.5;
    if tangent {
        return add_contact(a, b, center, tol, Some(true), hits);
    }
    let across = (a.frame.x * -y + a.frame.y * x) * height;
    for point in [center - across, center + across] {
        add_contact(a, b, point, tol, Some(false), hits)?;
    }
    Ok(())
}

fn coincident_contacts(a: ConicSupport<'_>, b: ConicSupport<'_>, tol: f64, hits: &mut Contacts) -> Result<(), LineIntersectionRefusal> {
    let (a_start, a_width) = a.interval();
    let (b_start, b_width) = b.interval();
    let phase = a.angle(b.curve.eval(0.0));
    let oriented = if a.frame.z.dot(b.frame.z) < 0.0 { phase - (b_start + b_width) } else { phase + b_start };
    let b_start = periodic_angle(oriented);
    let margin = (tol / a.radii.into_iter().chain(b.radii).fold(0.0, f64::max)).min(ANGULAR_ROUNDOFF);
    for shift in [-TAU, 0.0, TAU] {
        let b_start = b_start + shift;
        let lo = a_start.max(b_start);
        let hi = (a_start + a_width).min(b_start + b_width);
        if hi - lo > margin { return Err(Infinite); }
        if lo <= hi + margin {
            add_contact(a, b, a.curve.eval((lo + hi) * 0.5), tol, Some(true), hits)?;
        }
    }
    Ok(())
}

fn add_contact(a: ConicSupport<'_>, b: ConicSupport<'_>, point: Pnt3, tol: f64, tangent: Option<bool>, hits: &mut Contacts) -> Result<(), LineIntersectionRefusal> {
    let ta = a.angle(point);
    let tb = b.angle(point);
    if !point.to_array().into_iter().chain([ta,tb]).all(f64::is_finite) { return Err(NonFinite); }
    let Some(t_a) = conic_parameter(a.curve, ta, a.trim, tol) else { return Ok(()); };
    let Some(t_b) = conic_parameter(b.curve, tb, b.trim, tol) else { return Ok(()); };
    let pa = a.curve.eval(t_a);
    let pb = b.curve.eval(t_b);
    let distance = pa.distance(pb);
    if !distance.is_finite() { return Err(NonFinite); }
    if distance > tol { return Ok(()); }
    let point = pa + (pb - pa) * 0.5;
    if hits.iter().flatten().any(|previous| {
        let distance = periodic_angle(previous.t_a - t_a);
        distance.min(TAU - distance) <= ANGULAR_ROUNDOFF && previous.point.distance(point) <= tol
    }) { return Ok(()); }
    let tangent = match tangent {
        Some(value) => value,
        None => a.unit_tangent(t_a)?.cross(b.unit_tangent(t_b)?).norm() <= ANGULAR_ROUNDOFF,
    };
    let slot = hits.iter_mut().find(|hit| hit.is_none()).ok_or(UnresolvedContact)?;
    *slot = Some(CurveCurveHit { point, t_a, t_b, tangent, multiplicity: 1 });
    Ok(())
}
