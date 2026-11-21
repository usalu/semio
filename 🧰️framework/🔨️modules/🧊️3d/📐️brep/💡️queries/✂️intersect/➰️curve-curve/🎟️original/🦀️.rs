//! 🎯️ Bounded intersections of original line and conic supports with their topological trims.
use super::{intersect_line_line_unique, pack_hit, Curve3, CurveCurveHit, Frame3, LineIntersectionRefusal, Pnt3, Vec3};
use super::super::curve_surface::solve_trig_parameters;
use LineIntersectionRefusal::{DegenerateDirection, InvalidRange, InvalidTolerance, NonFinite};
use std::f64::consts::TAU;

#[path = "〰️conic-pair/🦀️.rs"]
mod conic_pair;
use conic_pair::{conic_contacts, ConicSupport};

type Trim = Option<(f64, f64)>;
type Contacts = [Option<CurveCurveHit>; 4];

/// ✂️ Finds supported fixed-capacity contacts. A trim limits only the underlying operand that owns it.
pub fn intersect_original_curve_curve(
    a: (&Curve3, Trim), b: (&Curve3, Trim), tol: f64,
) -> Result<Option<Contacts>, LineIntersectionRefusal> {
    if !(tol.is_finite() && tol > 0.0) { return Err(InvalidTolerance); }
    for (lo, hi) in [a.1, b.1].into_iter().flatten() {
        if !(lo.is_finite() && hi.is_finite() && lo <= hi) { return Err(InvalidRange); }
    }
    let ((origin, dir), line_trim, conic, conic_trim, swap) = match (a, b) {
        ((Curve3::Line { origin: o1, dir: d1 }, lo), (Curve3::Line { origin: o2, dir: d2 }, hi)) => {
            let hit = intersect_line_line_unique(*o1, *d1, *o2, *d2, tol)?
                .filter(|hit| belongs_to_line(hit.t_a, lo, d1.norm(), tol) && belongs_to_line(hit.t_b, hi, d2.norm(), tol));
            return Ok(Some([hit, None, None, None]));
        }
        ((Curve3::Line { origin, dir }, line_trim), (conic, conic_trim)) => ((origin, dir), line_trim, conic, conic_trim, false),
        ((conic, conic_trim), (Curve3::Line { origin, dir }, line_trim)) => ((origin, dir), line_trim, conic, conic_trim, true),
        _ => return conic_contacts(a, b, tol),
    };
    let Some(conic) = ConicSupport::from_curve(conic, conic_trim) else { return Ok(None); };
    line_conic((*origin, *dir, line_trim), conic, tol, swap).map(Some)
}

fn belongs_to_line(t: f64, trim: Trim, speed: f64, tol: f64) -> bool {
    trim.is_none_or(|(lo, hi)| (t - t.clamp(lo, hi)).abs() * speed <= tol)
}

fn periodic_angle(angle: f64) -> f64 {
    let normalized = angle.rem_euclid(TAU);
    if normalized == TAU { 0.0 } else { normalized }
}

pub(crate) fn conic_parameter(curve: &Curve3, t: f64, trim: Trim, tol: f64) -> Option<f64> {
    let Some((lo, hi)) = trim else { return Some(periodic_angle(t)); };
    let t = lo + periodic_angle(t - lo);
    if t <= hi { return Some(t); }
    let point = curve.eval(t);
    [lo, hi].into_iter().find(|&end| point.distance(curve.eval(end)) <= tol)
}

fn line_conic(
    (origin, direction, line_trim): (Pnt3, Vec3, Trim),
    conic: ConicSupport<'_>,
    tol: f64,
    swap: bool,
) -> Result<Contacts, LineIntersectionRefusal> {
    conic.validate(tol)?;
    let ConicSupport { curve: conic, frame, radii: [rx, ry], trim: conic_trim } = conic;
    let speed = direction.x.hypot(direction.y).hypot(direction.z);
    if !speed.is_finite() { return Err(NonFinite); }
    if speed <= tol { return Err(DegenerateDirection); }
    let unit = direction * (1.0 / speed);
    let o = frame.to_local(origin);
    let d = frame.to_local_vector(unit);
    if ![o.x, o.y, o.z, d.x, d.y, d.z].into_iter().all(f64::is_finite) { return Err(NonFinite); }
    let planar = d.z.abs() <= 16.0 * f64::EPSILON;
    let parameters = if planar {
        if o.z.abs() > tol { return Ok([None; 4]); }
        let n = d.x.hypot(d.y);
        let (nx, ny) = (-d.y / n, d.x / n);
        solve_trig_parameters(rx * nx, ry * ny, -nx.mul_add(o.x, ny * o.y), tol)?
    } else {
        let at_plane = -o.z / d.z;
        let x = d.x.mul_add(at_plane, o.x) / rx;
        let y = d.y.mul_add(at_plane, o.y) / ry;
        if ![at_plane, x, y].into_iter().all(f64::is_finite) { return Err(NonFinite); }
        [Some(y.atan2(x)), None]
    };
    let tangency = planar && parameters[1].is_none();
    let mut found: [Option<(f64, CurveCurveHit)>; 2] = [None, None];
    for angle in parameters.into_iter().flatten() {
        let Some(angle) = conic_parameter(conic, angle, conic_trim, tol) else { continue; };
        let point = conic.eval(angle);
        let displacement = point - origin;
        let parameter = displacement.dot(unit) / speed;
        let target = line_trim.map_or(parameter, |(lo, hi)| parameter.clamp(lo, hi));
        let residual = (displacement - direction * target).norm();
        if ![parameter, residual].into_iter().all(f64::is_finite) { return Err(NonFinite); }
        if residual > tol { continue; }
        if found[0].is_some_and(|(_, prior)| prior.point.distance(point) <= tol) { continue; }
        let hit = (target, pack_hit(point, target, angle, swap, tangency));
        if found[0].is_none() { found[0] = Some(hit); }
        else { found[1] = Some(hit); }
    }
    if let [Some(a), Some(b)] = found {
        if b.0 < a.0 { found.swap(0, 1); }
    }
    let [first, second] = found.map(|entry| entry.map(|(_, hit)| hit));
    Ok([first, second, None, None])
}
