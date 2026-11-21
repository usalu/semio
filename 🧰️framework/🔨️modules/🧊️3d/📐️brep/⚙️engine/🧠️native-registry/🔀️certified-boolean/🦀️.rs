//! 🔀️ Exact funded set operations publishing certified solids and material groups.

use super::*;
use semio_framework_value::{ValueError, ValueRefusalKind};

use semio_framework_3d_brep_boolean::certified_boundaries::unsupported;
use semio_framework_3d_brep_solid_transaction::boolean_recipe::{BooleanShape, Bounds};
use semio_framework_3d_brep_solid_transaction::BoxPrismBatch;
pub(super) use semio_framework_3d_brep_boolean::certified_boundaries::OrthogonalBoundary;
pub use semio_framework_3d_brep_representation::contract::BorePreviewSource;

#[path = "🐚️shell/🦀️.rs"]
mod shell;
pub use shell::ShellDraft;

#[path = "🧭️operand-projection/🦀️.rs"]
mod operand_projection;
pub use operand_projection::BooleanPreparation;

/// 🧩️ A certified native Boolean draft with either one boundary or a group of material pieces.
pub enum CertifiedBooleanResult<Group> {
    Solid(SolidPublication),
    Group(Group),
}

/// 🎟️ Retains the first certified operand's original identity and material recipe.
pub struct BooleanOperand {
    source: SolidId,
    owner: usize,
    revision: u64,
    shape: BooleanShape,
}

impl BooleanOperand {
    /// 📏️ Quotes the fixed-size captured material recipe.
    pub const fn recipe_copy_bytes() -> usize { size_of::<BooleanShape>() }
}

impl BrepRegistry {
    /// 🫴️ Captures precisely one live certified operand.
    pub fn capture_boolean_operand(&self, handle: &str) -> Result<BooleanOperand, ValueError> {
        let Some(Entity::Solid(source, Some(provenance))) = self.live.get(handle) else { return Err(unsupported()); };
        let shape = match provenance {
            SolidProvenance::Primitive(provenance) => BooleanShape::Analytic(provenance.primitive),
            SolidProvenance::Orthogonal(cells) => BooleanShape::Orthogonal(*cells),
            SolidProvenance::PlanarBoundary(Some(shape)) => BooleanShape::OrientedBox(shape.rectangular().ok_or_else(unsupported)?),
            _ => return Err(unsupported()),
        };
        Ok(BooleanOperand { source: *source, owner: std::ptr::from_ref(self) as usize, revision: self.shape_mutation_revision, shape })
    }

    /// 🔲️ Plans one original result without copying topology or publishing a live row.
    pub fn plan_certified_boolean(&mut self, a: &str, b: &str, first: &BooleanOperand, operation: BooleanOp)
        -> Result<CertifiedBooleanResult<BoxPrismBatch>, ValueError> {
        if first.owner != std::ptr::from_ref(&*self) as usize || first.revision != self.shape_mutation_revision
            || !matches!(self.live.get(a), Some(Entity::Solid(source, Some(_))) if *source == first.source) {
            return Err(ValueError::invariant("original Boolean operand changed during planning"));
        }
        let second = self.capture_boolean_operand(b)?;
        if first.source == second.source { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Boolean operands must be distinct solids")); }
        match first.shape.combine(second.shape,operation) {
            Ok(shape)=>self.plan_solid_boundary(shape).map(CertifiedBooleanResult::Solid),
            Err(error) if error.kind==ValueRefusalKind::UnsupportedOwner => first.shape.disconnected(second.shape,operation).map(CertifiedBooleanResult::Group).ok_or(error),
            Err(error)=>Err(error),
        }
    }
}
