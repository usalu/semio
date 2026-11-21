//! 📏️ Bounded arc length from rational Bézier enclosures, one subdivision per step.
use super::{ControlValues, Curve3, SplineRefusal, SplineSpan, SplineSpans};
use super::super::curve_ops::ellipse_perimeter;
use std::f64::consts::{FRAC_PI_2, TAU};
use super::super::bounded_polynomial::{evaluate, Polynomial, Wide, ROUNDOFF};

const ABSOLUTE_ERROR: f64 = 1e-8;
const RELATIVE_ERROR: f64 = 2e-7;
const MAX_DEPTH: u32 = 24;
const MAX_STEPS: usize = 16_384;

/// 🎚️ Refusal from a retained rational spline length cursor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArcLengthRefusal {
    Source(SplineRefusal),
    InvalidEllipse,
    Resolution,
}

impl From<SplineRefusal> for ArcLengthRefusal {
    fn from(error: SplineRefusal) -> Self { Self::Source(error) }
}

/// 🧮️ Resumes the original arc-length enclosure with no owned geometry or heap scratch.
#[derive(Default)]
pub struct ArcLengthCursor {
    span: usize,
    depth: u32,
    index: u32,
    steps: usize,
    length: Wide,
}

impl ArcLengthCursor {
    /// ⏱️ Processes one chart interval and yields a total only after closing all enclosures.
    pub fn step_spline(&mut self, curve: &Curve3, trim: Option<(f64, f64)>) -> Result<Option<f64>, ArcLengthRefusal> {
        let spline = SplineSpans::new(curve, trim)?;
        let Some(span) = spline.spans().nth(self.span) else { return Ok(Some(self.length.value())); };
        if span.is_point() { return Ok(Some(self.length.value())); }
        let width = spline.range.1 - spline.range.0;
        self.step_piece(span.a, span.b, width, |a, b| {
            let piece = SplineSpan {
                a, b, middle: Wide::sum(a * 0.5, b * 0.5), half_width: Wide::sum(b * 0.5, -a * 0.5), ..span
            };
            length_bounds(&spline, &piece)
        })
    }

    /// 🥚️ Measures complete turns and resumes the remaining exact rational ellipse arc.
    pub fn step_ellipse(&mut self, radii: [f64; 2], range: (f64, f64)) -> Result<Option<f64>, ArcLengthRefusal> {
        use ArcLengthRefusal::{InvalidEllipse, Resolution};
        let [a, b] = radii;
        let lo = range.0.min(range.1);
        let width = (range.1 - range.0).abs();
        if ![a, b, range.0, range.1, width].into_iter().all(f64::is_finite) || a <= 0.0 || b <= 0.0 {
            return Err(InvalidEllipse);
        }
        let extra = width.rem_euclid(TAU);
        let sections = (extra / FRAC_PI_2).ceil() as usize;
        if self.span >= sections {
            let turns = ((width - extra) / TAU).round();
            let full = if turns == 0.0 { 0.0 } else { turns * ellipse_perimeter(a, b) };
            let length = self.length.plus(Wide::scalar(full)).value();
            return if length.is_finite() { Ok(Some(length)) } else { Err(Resolution) };
        }
        let size = sections as f64;
        let start = extra * (self.span as f64 / size);
        let end = extra * ((self.span + 1) as f64 / size);
        let (sine, cosine) = lo.sin_cos();
        let origin = sine.atan2(cosine);
        self.step_piece(start, end, extra, |from, to| ellipse_bounds(radii, origin, from, to))
    }

    fn step_piece(&mut self, start: f64, end: f64, width: f64,
        bounds: impl FnOnce(f64, f64) -> Result<(f64, f64), ArcLengthRefusal>,
    ) -> Result<Option<f64>, ArcLengthRefusal> {
        use ArcLengthRefusal::Resolution;
        if self.steps >= MAX_STEPS || !width.is_finite() { return Err(Resolution); }
        self.steps += 1;
        let count = f64::from(1_u32 << self.depth);
        let a = interpolate(start, end, f64::from(self.index) / count);
        let b = interpolate(start, end, f64::from(self.index + 1) / count);
        if !(a < b) { return Err(Resolution); }
        let (lower, upper) = bounds(a, b)?;
        let gap = upper - lower;
        let tolerance = ABSOLUTE_ERROR * ((b - a) / width) + RELATIVE_ERROR * lower;
        if gap * 0.5 <= tolerance {
            self.length = self.length.plus(Wide::scalar(lower + gap * 0.5));
            if !self.length.value().is_finite() { return Err(Resolution); }
            self.advance();
        } else {
            if self.depth == MAX_DEPTH { return Err(Resolution); }
            self.depth += 1;
            self.index *= 2;
        }
        Ok(None)
    }

    fn advance(&mut self) {
        let completed = self.index.trailing_ones();
        if completed == self.depth {
            self.span += 1;
            self.depth = 0;
            self.index = 0;
        } else {
            self.depth -= completed;
            self.index = (self.index >> completed) + 1;
        }
    }
}

fn ellipse_bounds([a, b]: [f64; 2], origin: f64, from: f64, to: f64) -> Result<(f64, f64), ArcLengthRefusal> {
    let scale = a.max(b);
    let (a, b) = (a / scale, b / scale);
    let (sine, cosine) = (origin + (from + to) * 0.5).sin_cos();
    let (sin_half, cos_half) = ((to - from) * 0.5).sin_cos();
    let tangent = [-a * sine * sin_half, b * cosine * sin_half];
    let ratio = sin_half * (sin_half / cos_half);
    let radial = [a * cosine * ratio, b * sine * ratio];
    let lower = 2.0 * tangent[0].hypot(tangent[1]);
    let upper = (tangent[0] + radial[0]).hypot(tangent[1] + radial[1])
        + (tangent[0] - radial[0]).hypot(tangent[1] - radial[1]);
    let roundoff = 64.0 * f64::EPSILON * upper;
    let bounds = ((lower - roundoff).max(0.0) * scale, (upper.max(lower) + roundoff) * scale);
    if bounds.0.is_finite() && bounds.1.is_finite() { Ok(bounds) } else { Err(ArcLengthRefusal::Resolution) }
}

fn interpolate(a: f64, b: f64, fraction: f64) -> f64 {
    if fraction == 0.0 { a }
    else if fraction == 1.0 { b }
    else { Wide::scalar(a).scaled(1.0 - fraction).plus(Wide::scalar(b).scaled(fraction)).value() }
}

fn local_bernstein(polynomial: &Polynomial, degree: usize) -> [Wide; 4] {
    let mut result = [Wide::default(); 4];
    result[0] = evaluate(polynomial, degree, -1.0);
    result[degree] = evaluate(polynomial, degree, 1.0);
    if degree > 1 {
        let mut derivative = Polynomial::default();
        for i in 1..=degree { derivative[i - 1] = polynomial[i].scaled(i as f64); }
        result[1] = result[0].plus(evaluate(&derivative, degree - 1, -1.0).scaled(2.0).divided(degree as f64));
        if degree == 3 {
            result[2] = result[3].plus(evaluate(&derivative, degree - 1, 1.0).scaled(-2.0).divided(degree as f64));
        }
    }
    result
}

fn length_bounds(spline: &SplineSpans<'_>, span: &SplineSpan<'_>) -> Result<(f64, f64), ArcLengthRefusal> {
    use ArcLengthRefusal::Resolution;
    let degree = span.degree;
    let start = span.index - degree;
    let controls = &spline.controls[start..=span.index];
    let origin = controls[0].to_array();
    let scale = controls.iter().flat_map(|point| point.to_array().into_iter().zip(origin))
        .map(|(coordinate, origin)| (coordinate - origin).abs()).fold(0.0, f64::max);
    if !scale.is_finite() { return Err(Resolution); }
    if scale == 0.0 { return Ok((0.0, 0.0)); }

    let weight_scale = span.control_magnitude(&spline.weights);
    let weight = local_bernstein(&span.equation(&spline.weights)?.map(|value| value.divided(weight_scale)), degree);
    let minimum_weight = weight[..=degree].iter().map(|value| value.value()).fold(f64::INFINITY, f64::min);
    if minimum_weight <= 0.0 || !minimum_weight.is_finite() { return Err(Resolution); }
    let mut points = [[Wide::default(); 3]; 4];
    for axis in 0..3 {
        let mut values = ControlValues::default();
        for (offset, point) in controls.iter().enumerate() {
            let index = start + offset;
            values[index] = Wide::sum(point.to_array()[axis], -origin[axis]).divided(scale).times(spline.weights[index]);
        }
        let coords = local_bernstein(&span.equation(&values)?.map(|value| value.divided(weight_scale)), degree);
        for i in 0..=degree { points[i][axis] = coords[i].divided_by(weight[i]); }
    }
    let distance = |a: &[Wide; 3], b: &[Wide; 3]| {
        let [x, y, z] = std::array::from_fn(|axis| a[axis].plus(b[axis].scaled(-1.0)).value());
        x.hypot(y).hypot(z)
    };
    let lower = distance(&points[0], &points[degree]);
    let upper = points[..=degree].windows(2).map(|edge| distance(&edge[0], &edge[1])).sum::<f64>().max(lower);
    let roundoff = 64.0 * f64::EPSILON * upper + ROUNDOFF / minimum_weight;
    let bounds = ((lower - roundoff).max(0.0) * scale, (upper + roundoff) * scale);
    if bounds.0.is_finite() && bounds.1.is_finite() { Ok(bounds) } else { Err(Resolution) }
}
