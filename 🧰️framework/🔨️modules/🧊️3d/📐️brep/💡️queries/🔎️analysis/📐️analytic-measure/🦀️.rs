//! 📐️ Fixed-work sampling and measurement of analytic curves and surfaces.

use semio_framework_3d_brep_curve::{Curve3, curve_ops::ellipse_perimeter};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_value::{ValueError, ValueRefusalKind};

/// 🎯️ Samples the point or first derivative of an analytic curve.
pub fn analytic_curve_sample(curve: &Curve3, parameter: f64, tangent: bool) -> Result<[f64; 3], ValueError> {
    let (point,derivative,_)=curve.jet_original(parameter,usize::from(tangent)).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue,"curve evaluation requires a finite supported spline span"))?;
    Ok(if tangent {[derivative.x,derivative.y,derivative.z]} else {[point.x,point.y,point.z]})
}

/// 📏️ Measures the finite span of the curve’s parameter domain.
pub fn analytic_curve_span(curve: &Curve3) -> Result<f64, ValueError> {
    let (min, max) = curve.domain();
    let span = max - min;
    if !span.is_finite() {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "parameter span requires a bounded curve"));
    }
    Ok(span)
}

/// 🪝️ Evaluates curvature without constructing spline work.
pub fn analytic_curve_curvature(curve: &Curve3, parameter: f64) -> Result<f64, ValueError> {
    let (_,d,dd)=curve.jet_original(parameter,2).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue,"curve curvature requires a finite supported spline span"))?;
    let speed=d.norm();
    let curvature=if speed<=f64::EPSILON {0.0} else {d.cross(dd).norm()/speed.powi(3)};
    if !curvature.is_finite() {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "curve curvature must be finite"));
    }
    Ok(curvature)
}

/// 🧮️ Measures the bounded range of a line, circle, or full ellipse.
pub fn analytic_curve_length(curve: &Curve3, range: (f64, f64)) -> Result<f64, ValueError> {
    let span = (range.1 - range.0).abs();
    if !span.is_finite() {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "length requires a bounded curve"));
    }
    let length = match curve {
        Curve3::Line { dir, .. } => span * dir.norm(),
        Curve3::Circle { radius, .. } => span * radius.abs(),
        Curve3::Ellipse { major_radius, minor_radius, .. } if span == std::f64::consts::TAU => {
            ellipse_perimeter(*major_radius, *minor_radius)
        }
        _ => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "curve length requires an original integration owner")),
    };
    if !length.is_finite() {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "curve length must be finite"));
    }
    Ok(length)
}

/// 🧭️ Samples a point or normal without copying a spline control net.
pub fn analytic_surface_sample(surface: &Surface, u: f64, v: f64, normal: bool) -> Result<[f64; 3], ValueError> {
    let axes = if normal {
        let axis = surface.normal_original(u, v).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "surface normal is undefined"))?;
        [axis.x, axis.y, axis.z]
    } else {
        let point = surface.point_original(u, v).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"surface evaluation requires a finite supported spline patch"))?;
        [point.x, point.y, point.z]
    };
    if axes.iter().any(|value| !value.is_finite()) {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "surface evaluation must have finite coordinates"));
    }
    Ok(axes)
}
