//! 🧵️ Intersections of bounded rational B-spline spans with an original plane.
use super::{Curve3, CurveSurfaceHit, Frame3, LineIntersectionRefusal, PlaneContacts};
use semio_framework_3d_brep_curve::bounded_polynomial::{coefficient_size, roots, Wide, ROUNDOFF};
use semio_framework_3d_brep_curve::bounded_spline::{SplineRefusal, SplineSpans};
use LineIntersectionRefusal::{ContactCapacity, Infinite, InvalidRange, InvalidSpline, NonFinite, UnresolvedContact};

/// 🎚️ Projects each retained span's weighted plane equation before isolating geometric contacts.
pub(super) fn contacts(curve: &Curve3, trim: Option<(f64, f64)>, plane: &Frame3, tol: f64)
    -> Result<Option<PlaneContacts>, LineIntersectionRefusal>
{
    let spline = match SplineSpans::new(curve, trim) {
        Ok(spline) => spline,
        Err(SplineRefusal::Unsupported) => return Ok(None),
        Err(error) => return Err(refusal(error)),
    };
    let origin = plane.origin.to_array();
    let normal = plane.z.to_array();
    let values = spline.project(|point| point.to_array().into_iter().zip(origin).zip(normal).fold(
        Wide::default(), |distance, ((point, origin), normal)| distance.plus(Wide::sum(point, -origin).scaled(normal)),
    )).map_err(refusal)?;
    let mut hits = [None; 4];
    let mut previous = None;
    for span in spline.spans() {
        let polynomial = span.equation(&values).map_err(refusal)?;
        let norm = coefficient_size(&polynomial);
        if !norm.is_finite() { return Err(NonFinite); }
        if span.is_point() {
            if norm <= ROUNDOFF * span.control_magnitude(&values) {
                hits[0] = Some(checked_hit(&spline, plane, span.parameter(-1.0).map_err(refusal)?, tol)?);
            }
            return Ok(Some(hits));
        }
        if norm == 0.0 { return Err(Infinite); }
        for root in roots(&polynomial).map_err(|_| UnresolvedContact)?.iter() {
            let t = span.parameter(root.parameter).map_err(refusal)?;
            if root.parameter == -1.0 && previous == Some(t) { continue; }
            if previous.is_some_and(|last| last >= t) { return Err(UnresolvedContact); }
            previous = Some(t);
            let hit = checked_hit(&spline, plane, t, tol)?;
            if hits.iter().flatten().any(|prior| prior.point == hit.point) { continue; }
            *hits.iter_mut().find(|slot| slot.is_none()).ok_or(ContactCapacity)? = Some(hit);
        }
    }
    Ok(Some(hits))
}

fn refusal(error: SplineRefusal) -> LineIntersectionRefusal {
    match error {
        SplineRefusal::InvalidRange => InvalidRange,
        SplineRefusal::InvalidSpline => InvalidSpline,
        _ => UnresolvedContact,
    }
}

fn checked_hit(spline: &SplineSpans<'_>, plane: &Frame3, t: f64, tol: f64) -> Result<CurveSurfaceHit, LineIntersectionRefusal> {
    let point = spline.point_at(t).map_err(refusal)?;
    let uv = plane.to_local(point);
    if !uv.to_array().into_iter().all(f64::is_finite) { return Err(NonFinite); }
    if uv.z.abs() > tol { return Err(UnresolvedContact); }
    Ok(CurveSurfaceHit { point, t, u: uv.x, v: uv.y })
}
