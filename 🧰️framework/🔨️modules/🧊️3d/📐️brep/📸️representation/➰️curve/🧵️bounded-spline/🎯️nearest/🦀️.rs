//! 🎯️ Global nearest parameters of bounded original rational spline spans.
use super::{Curve3, Pnt3, SplineRefusal, SplineSpans, MAX_DEGREE};
use super::super::bounded_polynomial::{evaluate, roots, Polynomial, Wide, ROUNDOFF};
use SplineRefusal::Unresolved;

/// 🧲️ Compares every retained endpoint and stationary point of the original distance function.
pub fn closest_spline_parameter(curve: &Curve3, trim: Option<(f64, f64)>, target: Pnt3)
    -> Result<(f64, Pnt3, f64), SplineRefusal>
{
    let spline = SplineSpans::new(curve, trim)?;
    let target_coordinates = target.to_array();
    if !target_coordinates.into_iter().all(f64::is_finite) { return Err(Unresolved); }
    let normalized_scale = spline.controls.iter().flat_map(|point| point.to_array().into_iter().zip(target_coordinates))
        .map(|(point, target)| (point - target).abs()).fold(0.0, f64::max);
    if !normalized_scale.is_finite() { return Err(Unresolved); }
    let scale = if normalized_scale == 0.0 { 1.0 } else { normalized_scale };
    let mut projected = [super::ControlValues::default(); 3];
    for (axis, coordinates) in projected.iter_mut().enumerate() {
        *coordinates = spline.project(|point| Wide::sum(point.to_array()[axis], -target_coordinates[axis]).divided(scale))?;
    }
    let mut nearest: Option<(f64, Wide)> = None;
    for span in spline.spans() {
        let degree = span.degree;
        let scale = span.control_magnitude(&spline.weights);
        let weight = span.equation(&spline.weights)?.map(|coefficient| coefficient.divided(scale));
        let mut displacements = [Polynomial::default(); 3];
        for (result, coordinates) in displacements.iter_mut().zip(&projected) {
            *result = span.equation(coordinates)?.map(|coefficient| coefficient.divided(scale));
        }
        let stationary = roots(&stationary_equation(&displacements, &weight, degree)).map_err(|_| Unresolved)?;
        let candidates = std::iter::once(-1.0).chain(stationary.iter()
            .map(|root| root.parameter).filter(|parameter| -1.0 < *parameter && *parameter < 1.0))
            .chain(std::iter::once(1.0));
        for local in candidates {
            let parameter = span.parameter(local)?;
            let weight_at = evaluate(&weight, degree, local);
            if weight_at.value() <= 0.0 || !weight_at.value().is_finite() { return Err(Unresolved); }
            let distance = displacements.iter().fold(Wide::default(), |sum, polynomial| {
                let component = evaluate(polynomial, degree, local).divided_by(weight_at);
                sum.plus(component.times(component))
            });
            if !distance.value().is_finite() { return Err(Unresolved); }
            if nearest.is_none_or(|(_, prior)| {
                distance.plus(prior.scaled(-1.0)).value() < -ROUNDOFF * distance.value().max(prior.value())
            }) {
                nearest = Some((parameter, distance));
            }
        }
    }
    let (parameter, _) = nearest.ok_or(Unresolved)?;
    let point = spline.point_at(parameter)?;
    let delta = point - target;
    let distance = delta.x.hypot(delta.y).hypot(delta.z);
    if !distance.is_finite() { return Err(Unresolved); }
    Ok((parameter, point, distance))
}

fn stationary_equation(displacements: &[Polynomial; 3], weight: &Polynomial, degree: usize) -> Polynomial {
    let mut stationary = Polynomial::default();
    for displacement in displacements {
        let mut quotient_derivative = [Wide::default(); 2 * MAX_DEGREE - 1];
        for i in 0..=degree {
            for j in 0..=degree {
                if i == j { continue; }
                let term = displacement[i].times(weight[j]).scaled(i as f64 - j as f64);
                quotient_derivative[i + j - 1] = quotient_derivative[i + j - 1].plus(term);
            }
        }
        for i in 0..=degree {
            for (j, term) in quotient_derivative.iter().take(2 * degree - 1).enumerate() {
                stationary[i + j] = stationary[i + j].plus(displacement[i].times(*term));
            }
        }
    }
    stationary
}
