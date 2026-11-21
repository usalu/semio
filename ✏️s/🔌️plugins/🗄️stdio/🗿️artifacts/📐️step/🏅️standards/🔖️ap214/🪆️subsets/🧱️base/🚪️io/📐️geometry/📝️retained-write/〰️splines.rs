//! 〰️ Exact single-span unit-weight spline STEP records.

use super::{Piece, unsupported};
use semio_framework_3d_brep_curve::{Curve3, bspline::KnotVector};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::ValueError;

fn span(knots: &KnotVector) -> Option<[f64; 2]> {
    let &[a, b, c, d] = knots.knots.as_slice() else { return None; };
    (knots.degree == 1 && a == b && c == d && a.is_finite() && c.is_finite() && a < c).then_some([a, c])
}

pub(super) fn curve(piece: &mut Piece, spline: &Curve3, id: u64, points: [u64; 2], range: (f64, f64), ends: [Pnt3; 2]) -> Result<(), ValueError> {
    let invalid = || unsupported("STEP spline edges require a complete clamped degree-one unit-weight span with matching vertices");
    let Curve3::Nurbs { knots, controls, weights } = spline else { return Err(invalid()); };
    let [a, b] = span(knots).ok_or_else(invalid)?;
    let &[first, last] = controls.as_slice() else { return Err(invalid()); };
    if weights.as_slice() != [1.0; 2] || !(((a, b) == range && [first, last] == ends) || ((b, a) == range && [last, first] == ends)) {
        return Err(invalid());
    }
    let [p, q] = points;
    piece.point(p, "CARTESIAN_POINT", first.to_array())?;
    piece.point(q, "CARTESIAN_POINT", last.to_array())?;
    piece.put(format_args!("#{id} = B_SPLINE_CURVE_WITH_KNOTS('',1,(#{p},#{q}),.UNSPECIFIED.,.F.,.F.,(2,2),({a:.16E},{b:.16E}),.UNSPECIFIED.);\n"))
}

pub(super) fn surface(piece: &mut Piece, spline: &Surface, id: u64, points: [u64; 4]) -> Result<(), ValueError> {
    let invalid = || unsupported("STEP spline faces require a clamped unit-weight bilinear patch");
    let Surface::Nurbs { u_knots, v_knots, controls, weights } = spline else { return Err(invalid()); };
    let [u0, u1] = span(u_knots).ok_or_else(invalid)?;
    let [v0, v1] = span(v_knots).ok_or_else(invalid)?;
    let [first, last] = controls.as_slice() else { return Err(invalid()); };
    let ([p00, p01], [p10, p11]) = (first.as_slice(), last.as_slice()) else { return Err(invalid()); };
    if weights.len() != 2 || weights.iter().any(|row| row.as_slice() != [1.0; 2]) { return Err(invalid()); }
    for (reference, point) in points.into_iter().zip([p00, p01, p10, p11]) {
        piece.point(reference, "CARTESIAN_POINT", point.to_array())?;
    }
    let [p, q, r, s] = points;
    piece.put(format_args!("#{id} = B_SPLINE_SURFACE_WITH_KNOTS('',1,1,((#{p},#{q}),(#{r},#{s})),.UNSPECIFIED.,.F.,.F.,.F.,(2,2),(2,2),({u0:.16E},{u1:.16E}),({v0:.16E},{v1:.16E}),.UNSPECIFIED.);\n"))
}
