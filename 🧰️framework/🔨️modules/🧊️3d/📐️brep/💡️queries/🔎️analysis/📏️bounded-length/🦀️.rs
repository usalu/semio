//! 📏️ Retained arc length of bounded curves and distinct wire edges.
use super::analytic_curve_length;
use semio_framework_3d_brep_curve::{bounded_spline::{ArcLengthCursor, ArcLengthRefusal, SplineRefusal}, Curve3};
use semio_framework_3d_brep_representation::topology::{Body, Edge, Wire};
use semio_framework_3d_brep_representation::arena::EdgeId;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

/// 🧷️ Borrowed measurement source from an original handle.
pub enum LengthSource<'a> {
    Wire(&'a Wire, u64),
    Curve(&'a Curve3, Option<(f64, f64)>),
}

/// 🪜️ Traverses distinct wire edges and resumes bounded rational B-spline integrations.
pub struct LengthAccumulator {
    total: f64,
    index: usize,
    prior: usize,
    ascending: bool,
    arc: ArcLengthCursor,
}

impl Default for LengthAccumulator {
    fn default() -> Self {
        Self { total: 0.0, index: 0, prior: 0, ascending: true, arc: ArcLengthCursor::default() }
    }
}

impl LengthAccumulator {
    /// 🪙️ Quotes the next bounded projection from the original topology rows.
    pub fn demand(&self, body: &Body, source: LengthSource<'_>) -> Result<RetirementDemand, ValueError> {
        let mut demand = RetirementDemand { copy_bytes: 2 * size_of::<f64>(), depth: 1, ..Default::default() };
        if let LengthSource::Wire(wire, _) = source {
            if let Some(&(id, _)) = wire.members.get(self.index) {
                let edge = edge(body, id)?;
                demand.depth = body.edges.next_remove_depth_demand(id)?.max(body.curves3.next_remove_depth_demand(edge.curve)?);
                if self.prior < self.index && !self.ascending { demand.copy_bytes = size_of::<EdgeId>(); }
            } else { demand.copy_bytes = 0; }
        }
        Ok(demand)
    }

    /// ⏱️ Consumes one analytic edge, duplicate check, or certified spline subdivision.
    pub fn step(&mut self, body: &Body, source: LengthSource<'_>) -> Result<Option<f64>, ValueError> {
        let (wire, label) = match source {
            LengthSource::Curve(curve, trim) => return self.integrate(curve, trim),
            LengthSource::Wire(wire, label) => (wire, label),
        };
        let Some(&(id, _)) = wire.members.get(self.index) else { return Ok(Some(self.total)); };
        let edge = edge(body, id)?;
        self.ascending &= label.checked_sub((wire.members.len() - self.index) as u64) == Some(edge.label.0);
        if !self.ascending && self.prior < self.index {
            if wire.members[self.prior].0 == id { self.advance(); }
            else { self.prior += 1; }
            return Ok(None);
        }
        let curve = body.curves3.get(edge.curve).ok_or_else(|| ValueError::invariant("live length curve is missing"))?;
        let Some(length) = self.integrate(curve, Some(edge.range))? else { return Ok(None); };
        self.arc = ArcLengthCursor::default();
        self.total += length;
        if !self.total.is_finite() { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "wire length must be finite")); }
        self.advance();
        Ok(None)
    }

    fn integrate(&mut self, curve: &Curve3, trim: Option<(f64, f64)>) -> Result<Option<f64>, ValueError> {
        match curve {
            Curve3::Nurbs { .. } => self.arc.step_spline(curve, trim).map_err(arc_error),
            Curve3::Ellipse { major_radius, minor_radius, .. } => self.arc
                .step_ellipse([*major_radius, *minor_radius], trim.unwrap_or_else(|| curve.domain())).map_err(arc_error),
            _ => analytic_curve_length(curve, trim.unwrap_or_else(|| curve.domain())).map(Some),
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        self.prior = 0;
    }
}

fn edge(body: &Body, id: EdgeId) -> Result<&Edge, ValueError> {
    body.edges.get(id).ok_or_else(|| ValueError::invariant("live length edge is missing"))
}

fn arc_error(error: ArcLengthRefusal) -> ValueError {
    use ArcLengthRefusal::{InvalidEllipse, Resolution, Source};
    let (kind, message) = match error {
        Source(SplineRefusal::Unsupported) => (ValueRefusalKind::UnsupportedOwner, "length requires a nonperiodic rational spline of degree one through three with up to 32 controls"),
        Source(SplineRefusal::InvalidSpline) => (ValueRefusalKind::InvalidValue, "length requires continuous finite spline controls, knots and positive weights"),
        Source(SplineRefusal::InvalidRange) => (ValueRefusalKind::InvalidValue, "length requires a finite ordered trim within the spline domain"),
        InvalidEllipse => (ValueRefusalKind::InvalidValue, "elliptical length requires positive finite radii and finite trim"),
        Source(SplineRefusal::Unresolved) | Resolution => (ValueRefusalKind::InvalidValue, "original curve length exceeded its certified subdivision budget or finite precision"),
    };
    ValueError::literal(kind, message)
}
