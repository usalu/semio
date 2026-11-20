//! 🪡️ Original bounded sampling for oriented positive-weight spline trims.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};
use semio_framework_value::{ValueError, ValueRefusalKind};

/// 🎚️ The clipped interval and orientation of an original curve edge.
#[derive(Clone, Copy)]
pub struct SplineTrim { first: usize, end: usize, lo: f64, hi: f64, reversed: bool }

/// 📐️ One authored nonempty knot span and its required uniform subdivision.
#[derive(Clone, Copy)]
pub struct SplineSpan {
    index: usize,
    start: f64,
    end: f64,
    pub segments: usize,
    pub ends_trim: bool,
}

/// 🧭️ An original raw-span traversal that skips empty intervals without joining their limits.
#[derive(Clone, Copy)]
pub struct SplineCursor { trim: SplineTrim, ordinal: usize }

/// 📍️ The result of visiting one authored knot interval or the end of the traversal.
pub enum SplineVisit { Empty, Span(SplineSpan), End }

semio_framework_value::artifact_retire_leaf!(SplineTrim, SplineSpan, SplineCursor);

impl SplineTrim {
    /// 🔍️ Admits an oriented edge range within the original spline domain.
    pub fn new(curve: &Curve3, range: (f64, f64), forward: bool) -> Option<Self> {
        let Curve3::Nurbs { knots, .. } = curve else { return None; };
        let (first, end) = curve.original_spline_span_range()?;
        let lo = range.0.min(range.1);
        let hi = range.0.max(range.1);
        let domain = (knots.knots[first], knots.knots[end]);
        if !(range.1 - range.0).is_finite() || lo >= hi || !domain.0.is_finite()
            || !domain.1.is_finite() || lo < domain.0 || hi > domain.1 { return None; }
        Some(Self { first, end, lo, hi, reversed: (range.0 > range.1) ^ !forward })
    }

    /// 🧾️ Visits the original curve's knot spans in the selected direction.
    pub fn cursor(self) -> SplineCursor { SplineCursor { trim: self, ordinal: 0 } }
}

impl SplineCursor {
    /// 🚶️ Reads one raw interval using the original local positive-weight chord bound.
    pub fn next(&mut self, curve: &Curve3, tolerance: f64) -> Option<SplineVisit> {
        let t = self.trim;
        if self.ordinal >= t.end - t.first { return Some(SplineVisit::End); }
        let index = if t.reversed { t.end - 1 - self.ordinal } else { t.first + self.ordinal };
        self.ordinal += 1;
        let Curve3::Nurbs { knots, .. } = curve else { return None; };
        let (&a, &b) = (knots.knots.get(index)?, knots.knots.get(index + 1)?);
        if !a.is_finite() || !b.is_finite() || a > b { return None; }
        let lo = a.max(t.lo);
        let hi = b.min(t.hi);
        if lo >= hi { return Some(SplineVisit::Empty); }
        let (_, _, segments) = curve.original_spline_subdivision(index, tolerance)?;
        let (start, end, ends_trim) = if t.reversed { (hi, lo, lo == t.lo) } else { (lo, hi, hi == t.hi) };
        Some(SplineVisit::Span(SplineSpan { index, start, end, segments, ends_trim }))
    }
}

impl SplineSpan {
    /// 📏️ Evaluates the indicated chord point on its own authored span, including one-sided limits.
    pub fn point(self, curve: &Curve3, step: usize) -> Option<Pnt3> {
        let t = if step == self.segments { self.end }
            else { self.start + (self.end - self.start) * (step as f64 / self.segments as f64) };
        curve.original_spline_span_point(self.index, t)
    }

    fn controls(self, curve: &Curve3) -> Option<&[Pnt3]> {
        let Curve3::Nurbs { knots, controls, .. } = curve else { return None; };
        controls.get(self.index.checked_sub(knots.degree)?..=self.index)
    }
}

/// ⛳️ Original support plane and endpoint constraints of the selected trimmed edge.
pub struct ContourBoundary { pub origin: Pnt3, pub normal: Vec3, pub tolerance: f64, pub start: Pnt3, pub end: Pnt3 }

/// 🔢️ One contour preflight event and its exact corner contribution.
pub struct ContourCount { pub corners: usize, pub edge_end: bool }

/// 🖊️ One sampled contour corner in traversal order.
pub struct ContourCorner { pub point: Pnt3, pub edge_start: bool, pub edge_end: bool }

/// 🧵️ Reads an original spline edge as a continuous planar contour without allocating storage.
#[derive(Clone, Copy)]
pub struct ContourSpline {
    cursor: SplineCursor,
    tail: Option<Pnt3>,
    span: Option<SplineSpan>,
    segment: usize,
    started: bool,
}

semio_framework_value::artifact_retire_leaf!(ContourSpline);

impl ContourSpline {
    /// 🏁️ Starts the same original raw-span traversal used by wire preview.
    pub fn new(curve: &Curve3, range: (f64, f64), forward: bool) -> Option<Self> {
        let cursor = SplineTrim::new(curve, range, forward)?.cursor();
        Some(Self { cursor, tail: None, span: None, segment: 0, started: false })
    }

    /// 🧪️ Counts one positive-weight span and checks coplanarity and both one-sided endpoints.
    pub fn count(&mut self, curve: &Curve3, deflection: f64, boundary: ContourBoundary) -> Result<Option<ContourCount>, ValueError> {
        let Some(visit) = self.cursor.next(curve, deflection) else { return Ok(None); };
        let span = match visit {
            SplineVisit::Empty => return Ok(Some(ContourCount { corners: 0, edge_end: false })),
            SplineVisit::End => return Ok(None),
            SplineVisit::Span(span) => span,
        };
        let Some(controls) = span.controls(curve) else { return Ok(None); };
        if !controls.iter().all(|&p| ((p - boundary.origin).dot(boundary.normal)).abs() <= boundary.tolerance) {
            return Ok(None);
        }
        let (Some(a), Some(b)) = (span.point(curve, 0), span.point(curve, span.segments)) else { return Ok(None); };
        if let Some(tail) = self.tail {
            let scale = a.to_array().into_iter().chain(tail.to_array()).map(f64::abs).fold(1.0, f64::max);
            if !(a.distance(tail) <= 128.0 * f64::EPSILON * scale) {
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "planar spline trim jumps at an interior knot"));
            }
        } else if !(a.distance(boundary.start) <= boundary.tolerance) { return Ok(None); }
        if span.ends_trim && !(b.distance(boundary.end) <= boundary.tolerance) { return Ok(None); }
        self.tail = Some(b);
        Ok(Some(ContourCount { corners: span.segments, edge_end: span.ends_trim }))
    }

    /// ⏱️ Selects one span or emits one start corner for a funded chord.
    pub fn sample(&mut self, curve: &Curve3, deflection: f64) -> Option<Option<ContourCorner>> {
        let Some(span) = self.span else {
            match self.cursor.next(curve, deflection)? {
                SplineVisit::Span(span) => self.span = Some(span),
                SplineVisit::Empty => {}
                SplineVisit::End => return None,
            }
            return Some(None);
        };
        let point = span.point(curve, self.segment)?;
        let edge_start = !self.started;
        self.started = true;
        self.segment += 1;
        let end = self.segment == span.segments;
        if end { self.segment = 0; self.span = None; }
        Some(Some(ContourCorner { point, edge_start, edge_end: end && span.ends_trim }))
    }
}

#[path = "🔳️quadratic-pcurve/🦀️.rs"]
mod quadratic_pcurve;
pub use quadratic_pcurve::QuadraticPcurve;
