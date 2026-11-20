//! 🔀️ Exact funded set operations whose complete result is one certified solid.

use super::*;
use semio_framework_value::{ValueError, ValueRefusalKind};

use semio_framework_3d_brep_boolean::certified_boundaries::{coaxial_cylinders, empty_result, unsupported};
pub(super) use semio_framework_3d_brep_boolean::certified_boundaries::{BoreBoundary, OrthogonalBoundary, MAX_ORTHOGONAL_ENTITIES};
pub use semio_framework_3d_brep_representation::contract::BorePreviewSource;

#[path = "🐚️box-shell/🦀️.rs"]
mod box_shell;
pub use box_shell::BoxShellDraft;

#[path = "🧭️operand-projection/🦀️.rs"]
mod operand_projection;
pub use operand_projection::BooleanPreparation;

#[derive(Clone, Copy)]
struct Bounds { lo: [f64; 3], hi: [f64; 3] }

impl Bounds {
    fn box_primitive(origin: Pnt3, width: f64, depth: f64, height: f64) -> Self {
        Self { lo: evec(origin), hi: [origin.x + width, origin.y + depth, origin.z + height] }
    }
    fn primitive(self) -> SolidPrimitive {
        SolidPrimitive::Box { origin: pnt(self.lo), width: self.hi[0] - self.lo[0], depth: self.hi[1] - self.lo[1], height: self.hi[2] - self.lo[2] }
    }
    fn intersects(self) -> bool { (0..3).all(|i| self.hi[i] > self.lo[i]) }
    fn contains(self, other: Self) -> bool { (0..3).all(|i| self.lo[i] <= other.lo[i] && self.hi[i] >= other.hi[i]) }
    fn intersection(self, other: Self) -> Self {
        Self { lo: std::array::from_fn(|i| self.lo[i].max(other.lo[i])), hi: std::array::from_fn(|i| self.hi[i].min(other.hi[i])) }
    }
    fn union(self, other: Self) -> Self {
        Self { lo: std::array::from_fn(|i| self.lo[i].min(other.lo[i])), hi: std::array::from_fn(|i| self.hi[i].max(other.hi[i])) }
    }
    fn same_span(self, other: Self, i: usize) -> bool { self.lo[i] == other.lo[i] && self.hi[i] == other.hi[i] }
}

fn exact_box_boolean(a: Bounds, b: Bounds, operation: BooleanOp) -> Result<Bounds, ValueError> {
    let overlap = a.intersection(b);
    match operation {
        BooleanOp::Intersect => if overlap.intersects() { Ok(overlap) } else { Err(empty_result()) },
        BooleanOp::Unite => {
            if a.contains(b) { return Ok(a); }
            if b.contains(a) { return Ok(b); }
            if (0..3).any(|axis| overlap.hi[axis] >= overlap.lo[axis] && (0..3).filter(|i| *i != axis).all(|i| a.same_span(b, i))) {
                return Ok(a.union(b));
            }
            Err(unsupported())
        }
        BooleanOp::Cut => {
            if !overlap.intersects() { return Ok(a); }
            if b.contains(a) { return Err(empty_result()); }
            for axis in 0..3 {
                if !(0..3).filter(|i| *i != axis).all(|i| a.same_span(overlap, i)) { continue; }
                let mut remaining = a;
                if overlap.lo[axis] == a.lo[axis] {
                    remaining.lo[axis] = overlap.hi[axis];
                    return Ok(remaining);
                }
                if overlap.hi[axis] == a.hi[axis] {
                    remaining.hi[axis] = overlap.lo[axis];
                    return Ok(remaining);
                }
            }
            Err(unsupported())
        }
    }
}

fn nested_primitives(outer: SolidPrimitive, inner: SolidPrimitive, outer_is_a: bool, operation: BooleanOp)
    -> Result<SolidPrimitive, ValueError> {
    match operation {
        BooleanOp::Unite => Ok(outer),
        BooleanOp::Intersect => Ok(inner),
        BooleanOp::Cut if !outer_is_a => Err(empty_result()),
        BooleanOp::Cut => Err(unsupported()),
    }
}

fn primitive_result(a: SolidPrimitive, b: SolidPrimitive, operation: BooleanOp) -> Result<SolidPrimitive, ValueError> {
    match (a, b) {
        (
            SolidPrimitive::Box { origin: a_origin, width: a_width, depth: a_depth, height: a_height },
            SolidPrimitive::Box { origin: b_origin, width: b_width, depth: b_depth, height: b_height },
        ) => exact_box_boolean(
            Bounds::box_primitive(a_origin, a_width, a_depth, a_height),
            Bounds::box_primitive(b_origin, b_width, b_depth, b_height), operation,
        ).map(Bounds::primitive),
        (SolidPrimitive::Sphere { frame: a_frame, radius: a_radius }, SolidPrimitive::Sphere { frame: b_frame, radius: b_radius })
            if a_frame.origin == b_frame.origin =>
        {
            if a_radius <= b_radius { nested_primitives(b, a, false, operation) }
            else { nested_primitives(a, b, true, operation) }
        }
        (SolidPrimitive::Cylinder { .. }, SolidPrimitive::Cylinder { .. }) => coaxial_cylinders(a, b, operation),
        _ => Err(unsupported()),
    }
}

/// 🎟️ Retains the first operand's original solid identity and fixed analytic parameters.
pub struct BooleanOperand {
    source: SolidId,
    owner: usize,
    revision: u64,
    primitive: SolidPrimitive,
}

impl BrepRegistry {
    /// 🫴️ Captures precisely one live certified operand.
    pub fn capture_boolean_operand(&self, handle: &str) -> Result<BooleanOperand, ValueError> {
        let Some(Entity::Solid(source, Some(SolidProvenance::Primitive(provenance)))) = self.live.get(handle) else { return Err(unsupported()); };
        Ok(BooleanOperand {
            source: *source, owner: std::ptr::from_ref(self) as usize,
            revision: self.shape_mutation_revision, primitive: provenance.primitive,
        })
    }

    /// 🔲️ Plans an original result without copying source topology or publishing any live row.
    pub fn plan_certified_boolean(&mut self, a: &str, b: &str, first: &BooleanOperand, operation: BooleanOp)
        -> Result<SolidPublication, ValueError> {
        if a == b { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Boolean operands must be distinct solids")); }
        if first.owner != std::ptr::from_ref(&*self) as usize || first.revision != self.shape_mutation_revision
            || !matches!(self.live.get(a), Some(Entity::Solid(source, Some(_))) if *source == first.source) {
            return Err(ValueError::invariant("original Boolean operand changed during planning"));
        }
        let b = self.live_solid_primitive(b).ok_or_else(unsupported)?;
        let result = primitive_result(first.primitive, b, operation);
        match (result, first.primitive, b) {
            (Ok(shape), _, _) => self.plan_solid_primitive(shape),
            (
                Err(error),
                SolidPrimitive::Box { origin: a_origin, width: a_width, depth: a_depth, height: a_height },
                SolidPrimitive::Box { origin: b_origin, width: b_width, depth: b_depth, height: b_height },
            ) if error.kind == ValueRefusalKind::UnsupportedOwner => {
                let a = Bounds::box_primitive(a_origin, a_width, a_depth, a_height);
                let b = Bounds::box_primitive(b_origin, b_width, b_depth, b_height);
                self.plan_orthogonal_solid(OrthogonalBoundary::new([a.lo, a.hi], [b.lo, b.hi], operation)?)
            }
            (Err(error), stock @ SolidPrimitive::Box { .. }, tool @ SolidPrimitive::Cylinder { .. })
                if error.kind == ValueRefusalKind::UnsupportedOwner && matches!(operation, BooleanOp::Cut) =>
            {
                self.plan_bore_solid(BoreBoundary::new(stock, tool)?)
            }
            (Err(error), _, _) => Err(error),
        }
    }
}
