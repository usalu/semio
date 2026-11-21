//! 👥️ Bounded original coplanar ellipse contacts in two rational half-angle charts.
use super::{add_contact, coincident_contacts, ConicSupport, Contacts, LineIntersectionRefusal};
use LineIntersectionRefusal::{NonFinite, UnresolvedContact};

use semio_framework_3d_brep_curve::bounded_polynomial::{coefficient_size, roots, Polynomial, Wide, ROUNDOFF};

fn dot(point: [Wide; 3], axis: [f64; 3]) -> Wide {
    point.into_iter().zip(axis).fold(Wide::default(), |sum, (value, axis)| sum.plus(value.scaled(axis)))
}

fn normalized_coordinates(a: ConicSupport<'_>, b: ConicSupport<'_>) -> [[Wide; 3]; 2] {
    let origin_a = a.frame.origin.to_array();
    let origin_b = b.frame.origin.to_array();
    let difference = std::array::from_fn(|axis| Wide::sum(origin_a[axis], -origin_b[axis]));
    let horizontal = a.frame.x.to_array().map(Wide::scalar);
    let vertical = a.frame.y.to_array().map(Wide::scalar);
    let axes = [b.frame.x.to_array(), b.frame.y.to_array()];
    std::array::from_fn(|index| {
        let (axis, radius) = (axes[index], b.radii[index]);
        [
            dot(difference, axis).divided(radius),
            dot(horizontal, axis).scaled(a.radii[0]).divided(radius),
            dot(vertical, axis).scaled(a.radii[1]).divided(radius),
        ]
    })
}

fn implicit_polynomial(coordinates: [[Wide; 3]; 2], unit: Wide, sign: f64) -> Polynomial {
    let unit_squared = unit.times(unit);
    let mut result = Polynomial::default();
    result[0] = unit_squared.scaled(-1.0);
    result[2] = unit_squared.scaled(-2.0);
    result[4] = result[0];
    for [center, horizontal, vertical] in coordinates {
        let [p, q, r] = [center.plus(horizontal.scaled(sign)), vertical.scaled(2.0 * sign), center.plus(horizontal.scaled(-sign))];
        let squared = [p.times(p), p.times(q).scaled(2.0), q.times(q).plus(p.times(r).scaled(2.0)), q.times(r).scaled(2.0), r.times(r)];
        for (target, coefficient) in result.iter_mut().zip(squared) {
            *target = target.plus(coefficient);
        }
    }
    result
}

fn same_support(a: ConicSupport<'_>, b: ConicSupport<'_>) -> bool {
    if a.frame.origin != b.frame.origin { return false; }
    if a.radii[0] == a.radii[1] && a.radii == b.radii { return true; }
    let parallel = |axis_a, axis_b: super::Vec3| axis_a == axis_b || axis_a == -axis_b;
    (a.radii == b.radii && parallel(a.frame.x, b.frame.x) && parallel(a.frame.y, b.frame.y))
        || (a.radii == [b.radii[1], b.radii[0]] && parallel(a.frame.x, b.frame.y) && parallel(a.frame.y, b.frame.x))
}

/// 🎯️ Clips the cyclic quartic contacts to the two retained parameter intervals.
pub(super) fn contacts(a: ConicSupport<'_>, b: ConicSupport<'_>, tol: f64, hits: &mut Contacts) -> Result<(), LineIntersectionRefusal> {
    if same_support(a, b) { return coincident_contacts(a, b, tol, hits); }
    let coordinates = normalized_coordinates(a, b);
    if coordinates.iter().flatten().any(|coordinate| !coordinate.value().is_finite()) { return Err(NonFinite); }
    let scale = coordinates.iter().flatten().map(|coordinate| coordinate.value().abs()).fold(1.0, f64::max);
    let normalized = coordinates.map(|axis| axis.map(|coordinate| coordinate.divided(scale)));
    let unit = Wide::scalar(1.0).divided(scale);
    let positive = implicit_polynomial(normalized, unit, 1.0);
    let negative = implicit_polynomial(normalized, unit, -1.0);
    let unit_squared = unit.value().powi(2);
    if unit_squared == 0.0 { return Err(UnresolvedContact); }
    let same_curve = ROUNDOFF * unit_squared;
    if coefficient_size(&positive).max(coefficient_size(&negative)) <= same_curve {
        return coincident_contacts(a, b, tol, hits);
    }
    for (polynomial, phase) in [(positive, 0.0), (negative, std::f64::consts::PI)] {
        for root in roots(&polynomial).map_err(|_| UnresolvedContact)?.iter() {
            add_contact(a, b, a.curve.eval(phase + 2.0 * root.parameter.atan()), tol, Some(root.stationary), hits)?;
        }
    }
    Ok(())
}
