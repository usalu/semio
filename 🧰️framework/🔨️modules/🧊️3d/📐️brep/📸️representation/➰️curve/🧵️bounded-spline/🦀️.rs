//! 🧵️ Allocation-free rational spline span equations shared by retained geometry queries.
use super::{Curve3, Pnt3};
use super::bounded_polynomial::{Polynomial, Wide};
use super::bspline::KnotVector;

const MAX_CONTROLS: usize = 32;
const MAX_DEGREE: usize = 3;

#[path = "📏️length/🦀️.rs"]
mod length;
pub use length::{ArcLengthCursor, ArcLengthRefusal};

#[path = "🎯️nearest/🦀️.rs"]
mod nearest;
pub use nearest::closest_spline_parameter;

/// 🎚️ Refuses spline data outside the bounded original polynomial domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SplineRefusal {
    Unsupported,
    InvalidSpline,
    InvalidRange,
    Unresolved,
}

type ControlValues = [Wide; MAX_CONTROLS];

/// 🗺️ A verified nonperiodic rational spline and its retained parameter interval.
pub struct SplineSpans<'a> {
    knots: &'a KnotVector,
    controls: &'a [Pnt3],
    weights: ControlValues,
    range: (f64, f64),
}

impl<'a> SplineSpans<'a> {
    /// 🎟️ Admits continuous splines of degree one through three with at most 32 controls.
    pub fn new(curve: &'a Curve3, trim: Option<(f64, f64)>) -> Result<Self, SplineRefusal> {
        use SplineRefusal::{InvalidRange, InvalidSpline, Unsupported, Unresolved};
        let Curve3::Nurbs { knots, controls, weights } = curve else { return Err(Unsupported); };
        let (degree, count) = (knots.degree, controls.len());
        if !(1..=MAX_DEGREE).contains(&degree) || count > MAX_CONTROLS { return Err(Unsupported); }
        let u = &knots.knots;
        if count <= degree || u.len() != count + degree + 1 || weights.len() != count
            || u.iter().any(|knot| !knot.is_finite()) || u.windows(2).any(|pair| pair[0] > pair[1])
            || controls.iter().any(|point| !point.to_array().into_iter().all(f64::is_finite))
            || weights.iter().any(|weight| !weight.is_finite() || *weight <= 0.0)
        { return Err(InvalidSpline); }
        if knots.is_periodic() { return Err(Unsupported); }
        let domain = (u[degree], u[count]);
        if domain.0 >= u[degree + 1] || u[count - 1] >= domain.1
            || u.windows(degree + 1).any(|run| run[0] > domain.0 && run[0] < domain.1 && run[0] == run[degree])
        { return Err(InvalidSpline); }
        let range = trim.unwrap_or(domain);
        if !range.0.is_finite() || !range.1.is_finite() || range.0 < domain.0 || range.1 > domain.1 || range.0 > range.1 {
            return Err(InvalidRange);
        }
        let scale = weights.iter().copied().fold(0.0, f64::max);
        let mut normalized = ControlValues::default();
        for (value, &weight) in normalized.iter_mut().zip(weights) {
            *value = Wide::scalar(weight).divided(scale);
            if value.value() <= 0.0 || !value.value().is_finite() { return Err(Unresolved); }
        }
        Ok(Self { knots, controls, weights: normalized, range })
    }

    /// 📐️ Forms a compensated weighted scalar projection of every control position.
    pub fn project(&self, project: impl Fn(Pnt3) -> Wide) -> Result<ControlValues, SplineRefusal> {
        let mut values = ControlValues::default();
        for ((value, &control), weight) in values.iter_mut().zip(self.controls).zip(self.weights) {
            let projection = project(control);
            *value = projection.times(weight);
            if !value.value().is_finite() || value.value() == 0.0 && projection.value() != 0.0 {
                return Err(SplineRefusal::Unresolved);
            }
        }
        Ok(values)
    }

    /// 📍️ Evaluates an original spline point using the admitted homogeneous weight scale.
    pub fn point_at(&self, parameter: f64) -> Result<Pnt3, SplineRefusal> {
        if !(self.range.0..=self.range.1).contains(&parameter) { return Err(SplineRefusal::InvalidRange); }
        let index = self.knots.find_span(parameter);
        let span = SplineSpan {
            knots: &self.knots.knots, index, degree: self.knots.degree, a: parameter, b: parameter,
            middle: Wide::scalar(parameter), half_width: Wide::default(),
        };
        let weight = span.equation(&self.weights)?[0];
        if weight.value() <= 0.0 || !weight.value().is_finite() { return Err(SplineRefusal::Unresolved); }
        let mut coordinates = [0.0; 3];
        for (axis, coordinate) in coordinates.iter_mut().enumerate() {
            let active = &self.controls[index - span.degree..=index];
            let scale = active.iter().map(|point| point.to_array()[axis].abs()).fold(0.0, f64::max);
            if scale == 0.0 { continue; }
            let mut values = ControlValues::default();
            for (offset, point) in active.iter().enumerate() {
                let control = index - span.degree + offset;
                values[control] = Wide::scalar(point.to_array()[axis]).divided(scale).times(self.weights[control]);
            }
            *coordinate = span.equation(&values)?[0].divided_by(weight).scaled(scale).value();
            if !coordinate.is_finite() { return Err(SplineRefusal::Unresolved); }
        }
        Ok(Pnt3::from_array(coordinates))
    }

    /// 🧩️ Iterates nonempty retained spans or the single certified span of a point trim.
    pub fn spans(&self) -> impl Iterator<Item = SplineSpan<'_>> + '_ {
        let (lo, hi) = self.range;
        let degree = self.knots.degree;
        let u = &self.knots.knots;
        let point_span = (lo == hi).then(|| self.knots.find_span(lo));
        (degree..self.controls.len()).filter_map(move |index| {
            let (a, b) = (u[index].max(lo), u[index + 1].min(hi));
            if a >= b && point_span != Some(index) { return None; }
            let middle = Wide::sum(a * 0.5, b * 0.5);
            let half_width = Wide::sum(b * 0.5, -a * 0.5);
            Some(SplineSpan { knots: u, index, degree, a, b, middle, half_width })
        })
    }
}

/// 🧮️ Affine parameter chart for one retained knot span.
pub struct SplineSpan<'a> {
    knots: &'a [f64],
    index: usize,
    degree: usize,
    a: f64,
    b: f64,
    middle: Wide,
    half_width: Wide,
}

impl SplineSpan<'_> {
    /// 📌️ Identifies a source interval containing one authored parameter.
    pub fn is_point(&self) -> bool { self.a == self.b }

    /// 🔗️ Maps a chart coordinate into the retained knot interval.
    pub fn parameter(&self, local: f64) -> Result<f64, SplineRefusal> {
        match local {
            -1.0 => Ok(self.a),
            1.0 => Ok(self.b),
            local if self.is_point() && (-1.0..=1.0).contains(&local) => Ok(self.a),
            local => {
                let t = self.middle.plus(self.half_width.scaled(local)).value();
                if self.a < t && t < self.b { Ok(t) } else { Err(SplineRefusal::Unresolved) }
            }
        }
    }

    /// 📊️ Forms the exact local polynomial via de Boor's affine recurrence.
    pub fn equation(&self, values: &ControlValues) -> Result<Polynomial, SplineRefusal> {
        if !self.is_point() && self.half_width.value() == 0.0 { return Err(SplineRefusal::Unresolved); }
        let (span, degree) = (self.index, self.degree);
        let first = span - degree;
        let mut local = [Polynomial::default(); MAX_DEGREE + 1];
        for (term, &value) in local.iter_mut().zip(&values[first..=span]) { term[0] = value; }
        for level in 1..=degree {
            for j in (level..=degree).rev() {
                let left = self.knots[first + j];
                let right = self.knots[span + 1 + j - level];
                let denominator = Wide::sum(right, -left);
                let center = self.middle.plus(Wide::scalar(-left)).divided_by(denominator);
                let slope = self.half_width.divided_by(denominator);
                for power in (0..=level).rev() {
                    let lower = local[j - 1][power];
                    let change = local[j][power].plus(lower.scaled(-1.0));
                    let lower_order = power.checked_sub(1).map_or(Wide::default(), |previous| {
                        local[j][previous].plus(local[j - 1][previous].scaled(-1.0))
                    });
                    local[j][power] = lower.plus(change.times(center)).plus(lower_order.times(slope));
                }
            }
        }
        Ok(local[degree])
    }

    /// 🧱️ Bounds the arithmetic scale of the current source controls.
    pub fn control_magnitude(&self, values: &ControlValues) -> f64 {
        values[self.index - self.degree..=self.index].iter().map(|value| value.value().abs()).fold(0.0, f64::max)
    }
}
