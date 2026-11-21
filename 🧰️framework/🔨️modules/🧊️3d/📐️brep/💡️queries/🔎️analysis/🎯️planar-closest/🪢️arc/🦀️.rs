//! 🪢️ Original conic and rational quadratic trim crossings in a planar face.
use super::{finite, invalid, length, unsupported, valid_uv, Curve3, Foot, Pnt2, Pnt3, ValueError};
use super::super::nearest_analytic;
use semio_framework_3d_brep_curve::{Curve2, bounded_polynomial::{evaluate, roots, Polynomial, Wide}};
use semio_framework_3d_brep_vector::{matrix::Frame3, predicates::{orient2d, Orient}};
use std::f64::consts::{FRAC_PI_2, TAU};

#[path = "📐️area/🦀️.rs"]
mod area;

enum Arc {
    Analytic([f64; 2]),
    Rational([Polynomial; 3]),
}

impl Arc {
    fn authored(curve: &Curve3, trim: &Curve2, face: &Frame3, range: (f64, f64)) -> Result<Self, ValueError> {
        let zero = Pnt2::new(0.0, 0.0);
        let (lo, hi) = range;
        if !lo.is_finite() || !hi.is_finite() || lo >= hi { return Err(invalid()); }
        let arc = match (curve, trim) {
            (Curve3::Circle { frame, radius }, Curve2::Circle { center, radius: r })
                if frame == face && *center == zero && radius == r => Self::Analytic([*radius; 2]),
            (Curve3::Ellipse { frame, major_radius, minor_radius },
                Curve2::Ellipse { center, x_axis, major_radius: a, minor_radius: b })
                if frame == face && *center == zero && x_axis.x == 1.0 && x_axis.y == 0.0
                    && major_radius == a && minor_radius == b => Self::Analytic([*a, *b]),
            (Curve3::Nurbs { knots, controls, weights }, Curve2::Nurbs { knots: uv_knots, controls: uv, weights: uv_weights })
                if knots == uv_knots && weights == uv_weights && knots.degree == 2
                    && knots.knots.as_slice() == [0.0, 0.0, 0.0, 1.0, 1.0, 1.0]
                    && controls.len() == 3 && uv.len() == 3 && weights.len() == 3
                    && lo >= 0.0 && hi <= 1.0 => {
                for (&point, uv) in controls.iter().zip(uv) {
                    if !valid_uv(*uv) || point != face.origin + face.x * uv.x + face.y * uv.y { return Err(unsupported()); }
                }
                if orient2d(uv[0], uv[1], uv[2]) == Orient::Zero { return Err(invalid()); }
                if weights.iter().any(|weight| !weight.is_finite() || *weight <= 0.0) { return Err(invalid()); }
                let scale = weights.iter().copied().fold(0.0, f64::max);
                let normalized: [Wide; 3] = std::array::from_fn(|i| Wide::scalar(weights[i]).divided(scale));
                if normalized.iter().any(|weight| weight.value() <= 0.0) { return Err(invalid()); }
                let numerator_x = std::array::from_fn(|i| normalized[i].scaled(uv[i].x));
                let numerator_y = std::array::from_fn(|i| normalized[i].scaled(uv[i].y));
                Self::Rational([power(numerator_x), power(numerator_y), power(normalized)])
            }
            _ => return Err(unsupported()),
        };
        if let Self::Analytic(radii) = &arc {
            let unit_error = 128.0 * f64::EPSILON;
            if ![(face.x.norm_sq() - 1.0).abs(), (face.y.norm_sq() - 1.0).abs(), face.x.dot(face.y).abs()]
                .into_iter().all(|error| error <= unit_error)
            { return Err(unsupported()); }
            if radii.iter().any(|radius| !radius.is_finite() || *radius <= 0.0)
                || hi - lo > TAU || hi.abs().max(lo.abs()) * f64::EPSILON > 1e-10
            { return Err(invalid()); }
        }
        Ok(arc)
    }

    fn evaluate(&self, t: f64) -> Result<Pnt2, ValueError> {
        let point = match self {
            Self::Analytic([a, b]) => Pnt2::new(a * t.cos(), b * t.sin()),
            Self::Rational([x, y, weight]) => {
                let s = t.mul_add(2.0, -1.0);
                let weight = evaluate(weight, 2, s);
                if weight.value() <= 0.0 { return Err(invalid()); }
                Pnt2::new(evaluate(x, 2, s).divided_by(weight).value(), evaluate(y, 2, s).divided_by(weight).value())
            }
        };
        if !valid_uv(point) { return Err(invalid()); }
        Ok(point)
    }

    fn cuts(&self, (lo, hi): (f64, f64)) -> Result<([f64; 4], usize), ValueError> {
        let mut values = [lo, hi, hi, hi];
        let mut count = 1;
        let mut insert = |candidate| {
            if lo < candidate && candidate < hi {
                values[count] = candidate;
                count += 1;
            }
        };
        match self {
            Self::Analytic(_) => {
                let (sine, cosine) = lo.sin_cos();
                let origin = sine.atan2(cosine);
                for phase in [FRAC_PI_2, -FRAC_PI_2] { insert(lo + (phase - origin).rem_euclid(TAU)); }
            }
            Self::Rational([_, y, weight]) => {
                let mut derivative = Polynomial::default();
                for i in 0..=2 {
                    for j in 0..=2 {
                        if i != j {
                            derivative[i + j - 1] = derivative[i + j - 1].plus(y[i].times(weight[j]).scaled(i as f64 - j as f64));
                        }
                    }
                }
                for root in roots(&derivative).map_err(|_| invalid())?.iter() { insert((root.parameter + 1.0) * 0.5); }
            }
        }
        values[..count].sort_by(f64::total_cmp);
        Ok((values, count + 1))
    }
}

fn validated(curve: &Curve3, trim: &Curve2, range: (f64, f64), vertices: [Pnt3; 2], face: &Frame3) -> Result<Arc, ValueError> {
    let arc = Arc::authored(curve, trim, face, range)?;
    for (vertex, parameter) in vertices.into_iter().zip([range.0, range.1]) {
        let uv = arc.evaluate(parameter)?;
        let original = face.origin + face.x * uv.x + face.y * uv.y;
        let scale = original.to_array().into_iter().chain(vertex.to_array()).map(f64::abs).fold(0.0, f64::max);
        if !finite(original) || length(original - vertex) > 128.0 * f64::EPSILON * scale { return Err(invalid()); }
    }
    Ok(arc)
}

/// 📐️ Integrates the signed correction between an original arc and its topology chord.
pub(crate) fn area_correction(curve: &Curve3, trim: &Curve2, range: (f64, f64), vertices: [Pnt3; 2], face: &Frame3) -> Result<f64, ValueError> {
    let twice = match validated(curve, trim, range, vertices, face)? {
        Arc::Analytic(radii) => area::elliptic(radii, range.1 - range.0),
        Arc::Rational(_) => {
            let Curve2::Nurbs { controls, weights, .. } = trim else { return Err(unsupported()); };
            if range != (0.0, 1.0) || weights[0] != weights[2] { return Err(unsupported()); }
            let w = weights[1] / weights[0];
            if w <= 0.0 || !w.is_finite() { return Err(invalid()); }
            let a = controls[1] - controls[0];
            let b = controls[2] - controls[0];
            a.x.mul_add(b.y, -a.y * b.x) * area::rational(w)
        }
    };
    let world = twice * length(face.x.cross(face.y));
    if world.is_finite() { Ok(world) } else { Err(invalid()) }
}

/// 🎯️ Computes analytic crossings and a nearest point on the same authored curve and trim.
pub(super) fn project(curve: &Curve3, trim: &Curve2, range: (f64, f64), vertices: [Pnt3; 2], foot: &Foot)
    -> Result<(bool, bool, Pnt3), ValueError>
{
    let arc = validated(curve, trim, range, vertices, &foot.plane.frame)?;
    let ends = vertices.map(|point| foot.plane.coordinates(point));
    let target_uv = foot.uv;
    let (cuts, count) = arc.cuts(range)?;
    let mut a = ends[0];
    let mut inside = false;
    let mut touches = false;
    for i in 1..count {
        let b = if i + 1 == count { ends[1] } else { arc.evaluate(cuts[i])? };
        if (a.y > target_uv.y) != (b.y > target_uv.y) {
            let x = if a.y == target_uv.y { a.x }
                else if b.y == target_uv.y { b.x }
                else { crossing(&arc, (cuts[i - 1], cuts[i]), a, b, target_uv.y)? };
            inside ^= x > target_uv.x;
            touches |= x == target_uv.x;
        }
        a = b;
    }
    let (_, candidate, _) = nearest_analytic::curve_nearest((curve, Some(range)), foot.point)?;
    Ok((inside, touches, candidate))
}

fn crossing(arc: &Arc, (mut lo, mut hi): (f64, f64), a: Pnt2, b: Pnt2, height: f64) -> Result<f64, ValueError> {
    match arc {
        Arc::Analytic([width, radius]) => {
            let gap = radius - height.abs();
            if gap < -128.0 * f64::EPSILON * radius { return Err(invalid()); }
            let magnitude = width * ((gap.max(0.0) / radius) * (1.0 + (height / radius).abs())).sqrt();
            Ok(magnitude.copysign((lo + (hi - lo) * 0.5).cos()))
        }
        Arc::Rational([_, y, weight]) => {
            let equation = |t: f64| {
                let parameter = t.mul_add(2.0, -1.0);
                evaluate(y, 2, parameter).plus(evaluate(weight, 2, parameter).scaled(-height)).value()
            };
            let above = a.y > height;
            let mut at_lo = a.y - height;
            let mut at_hi = b.y - height;
            for _ in 0..96 {
                let middle = lo + (hi - lo) * 0.5;
                if middle == lo || middle == hi { break; }
                let residual = equation(middle);
                if !residual.is_finite() { return Err(invalid()); }
                if residual == 0.0 { return Ok(arc.evaluate(middle)?.x); }
                if (residual > 0.0) == above { lo = middle; at_lo = residual; }
                else { hi = middle; at_hi = residual; }
            }
            Ok(arc.evaluate(if at_lo.abs() < at_hi.abs() { lo } else { hi })?.x)
        }
    }
}

fn power([a, b, c]: [Wide; 3]) -> Polynomial {
    let mut result = Polynomial::default();
    let outer = a.plus(c).scaled(0.25);
    result[0] = outer.plus(b.scaled(0.5));
    result[1] = c.plus(a.scaled(-1.0)).scaled(0.5);
    result[2] = outer.plus(b.scaled(-0.5));
    result
}
