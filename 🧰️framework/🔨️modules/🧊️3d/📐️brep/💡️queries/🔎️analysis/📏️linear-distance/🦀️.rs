//! 📏️ Minimum separation of original linear supports and finite retained edge intervals.

use semio_framework_3d_brep_curve::{bounded_polynomial::Wide, Curve3};
use semio_framework_3d_brep_vector::{Pnt3, Vec3};
use semio_framework_value::{ValueError, ValueRefusalKind};

type Vector = [Wide; 3];
type Operand<'a> = (&'a Curve3, Option<(f64, f64)>);

struct Line {
    origin: Vector,
    direction: Vector,
    extent: Option<Wide>,
}

impl Line {
    fn from_original((curve, trim): Operand<'_>) -> Result<Self, ValueError> {
        let Curve3::Line { origin, dir } = curve else {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "distance between original curves requires linear supports"));
        };
        let origin = origin.to_array();
        let direction = dir.to_array();
        if !origin.into_iter().chain(direction).all(f64::is_finite) { return Err(invalid()); }

        let maximum = direction.into_iter().map(f64::abs).fold(0.0, f64::max);
        if maximum == 0.0 && trim.is_none() { return Err(invalid()); }
        let scale = if maximum == 0.0 { 1.0 } else { power_of_two(maximum) };
        let normalized = if maximum == 0.0 { [1.0, 0.0, 0.0] } else { direction.map(|value| value / scale) };
        if maximum > 0.0 && direction.into_iter().zip(normalized).any(|(raw, normalized)| raw != 0.0 && normalized == 0.0) {
            return Err(invalid());
        }
        let mut anchor = origin.map(Wide::scalar);
        let extent = if let Some((lo, hi)) = trim {
            if !lo.is_finite() || !hi.is_finite() || lo > hi { return Err(invalid()); }
            for (i, coordinate) in anchor.iter_mut().enumerate() {
                let anchored = Wide::scalar(direction[i]).scaled(lo).plus(*coordinate);
                *coordinate = if anchored.value().is_finite() { anchored }
                    else { Wide::scalar(direction[i].mul_add(lo, origin[i])) };
            }
            let extent = if maximum == 0.0 { Wide::scalar(0.0) } else {
                let span = Wide::sum(hi, -lo);
                if span.value().is_finite() { span.scaled(scale) }
                else { Wide::scalar(hi).scaled(scale).plus(Wide::scalar(lo).scaled(-scale)) }
            };
            Some(extent)
        } else { None };
        if !anchor.into_iter().chain(extent).all(|value| value.value().is_finite()) { return Err(invalid()); }
        Ok(Self { origin: anchor, direction: normalized.map(Wide::scalar), extent })
    }
}

/// 🧭️ Compares the supporting common perpendicular to all retained boundary projections.
pub fn original_linear_distance(a: Operand<'_>, b: Operand<'_>) -> Result<f64, ValueError> {
    let (a, b) = (Line::from_original(a)?, Line::from_original(b)?);
    let separation = std::array::from_fn(|axis| minus(b.origin[axis], a.origin[axis]));
    let mut largest = 0.0_f64;
    for value in separation.into_iter().chain(a.extent).chain(b.extent) {
        if !value.value().is_finite() { return Err(invalid()); }
        largest = largest.max(value.value().abs());
    }
    let scale = if largest == 0.0 { 1.0 } else { power_of_two(largest) };
    let r = separation.map(|value| value.divided(scale));
    let a_span = a.extent.map(|value| value.divided(scale));
    let b_span = b.extent.map(|value| value.divided(scale));
    for (value, converted) in separation.into_iter().chain(a.extent).chain(b.extent).zip(r.into_iter().chain(a_span).chain(b_span)) {
        if value.value() != 0.0 && converted.value() == 0.0 { return Err(invalid()); }
    }

    let normal = cross(a.direction, b.direction);
    let extent = normal.into_iter().map(|value| value.value().abs()).fold(0.0, f64::max);
    if extent > 0.0 {
        let normal_scale = power_of_two(extent);
        let n = normal.map(|value| value.divided(normal_scale));
        let norm_squared = dot(n, n);
        let denominator = norm_squared.scaled(normal_scale);
        if admits(a_span, dot(cross(r, b.direction), n), denominator)
            && admits(b_span, dot(cross(r, a.direction), n), denominator)
        {
            return finite_distance(scale * (dot(r, n).value().abs() / norm_squared.value().sqrt()));
        }
    } else if a_span.is_none() && b_span.is_none() {
        return finite_distance(scale * point_distance(r, a.direction, None));
    }

    let mut best = f64::INFINITY;
    let reverse = r.map(|value| value.scaled(-1.0));
    for (initial, axis, extent, other_axis, other_extent) in [
        (reverse, a.direction, a_span, b.direction, b_span),
        (r, b.direction, b_span, a.direction, a_span),
    ] {
        if let Some(extent) = extent {
            let end = std::array::from_fn(|i| initial[i].plus(axis[i].times(extent)));
            best = best.min(point_distance(initial, other_axis, other_extent)).min(point_distance(end, other_axis, other_extent));
        }
    }
    finite_distance(best * scale)
}

/// 🪟️ Finds the retained linear point nearest the supporting plane.
pub(super) fn plane_probe(operand: Operand<'_>, origin: Pnt3, normal: Vec3) -> Result<Option<(Pnt3, f64)>, ValueError> {
    let line = Line::from_original(operand)?;
    let origin = origin.to_array();
    let normal = normal.to_array().map(Wide::scalar);
    let offset = std::array::from_fn(|i| minus(line.origin[i], Wide::scalar(origin[i])));
    let height = dot(offset, normal);
    let slope = dot(line.direction, normal);
    if !height.value().is_finite() || !slope.value().is_finite() { return Err(invalid()); }
    let zero = Wide::scalar(0.0);
    let (parameter, distance) = if slope.value() == 0.0 { (zero, height.value().abs()) }
        else {
            let direction = slope.value().signum();
            let numerator = height.scaled(-direction);
            let denominator = slope.scaled(direction);
            if line.extent.is_some() && numerator.value() < 0.0 {
                (zero, height.value().abs())
            } else if let Some(end) = line.extent.filter(|&end| minus(numerator, end.times(denominator)).value() > 0.0) {
                (end, height.plus(slope.times(end)).value().abs())
            } else { (numerator.divided_by(denominator), 0.0) }
        };
    if !parameter.value().is_finite() { return Ok(None); }
    let point = std::array::from_fn(|i| line.origin[i].plus(line.direction[i].times(parameter)).value());
    if !distance.is_finite() { return Err(invalid()); }
    Ok(point.into_iter().all(f64::is_finite).then_some((Pnt3::from_array(point), distance)))
}

fn point_distance(point: Vector, direction: Vector, extent: Option<Wide>) -> f64 {
    let norm = dot(direction, direction);
    let along = dot(point, direction);
    let projection = match extent {
        Some(_) if along.value() <= 0.0 => Wide::scalar(0.0),
        Some(end) if minus(along, end.times(norm)).value() >= 0.0 => end,
        _ => along.divided_by(norm),
    };
    let [x, y, z] = std::array::from_fn(|i| minus(point[i], direction[i].times(projection)).value());
    x.hypot(y).hypot(z)
}

fn admits(extent: Option<Wide>, numerator: Wide, denominator: Wide) -> bool {
    extent.is_none_or(|extent| numerator.value() >= 0.0 && minus(numerator, extent.times(denominator)).value() <= 0.0)
}

fn dot(a: Vector, b: Vector) -> Wide {
    a.into_iter().zip(b).fold(Wide::default(), |sum, (a, b)| sum.plus(a.times(b)))
}

fn cross(a: Vector, b: Vector) -> Vector {
    std::array::from_fn(|i| {
        let j = (i + 1) % 3;
        let k = (i + 2) % 3;
        minus(a[j].times(b[k]), a[k].times(b[j]))
    })
}

fn minus(a: Wide, b: Wide) -> Wide { a.plus(b.scaled(-1.0)) }

fn power_of_two(value: f64) -> f64 {
    let bits = value.to_bits();
    let exponent = bits & 0x7ff0_0000_0000_0000;
    let leading = u64::BITS - 1 - bits.leading_zeros();
    f64::from_bits(if exponent == 0 { 1 << leading } else { exponent })
}

fn finite_distance(distance: f64) -> Result<f64, ValueError> {
    if distance.is_finite() { Ok(distance) } else { Err(invalid()) }
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "linear distance requires finite geometry, ordered trims, and nonzero unbounded directions")
}
