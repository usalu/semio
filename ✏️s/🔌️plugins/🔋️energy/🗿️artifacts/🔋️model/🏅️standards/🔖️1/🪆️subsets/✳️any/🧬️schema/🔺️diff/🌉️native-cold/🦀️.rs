//! 🌉️ The cold reference diff applies to the native persistent snapshot by projecting to plain rows and back.
use super::*;
use crate::snapshot::EnergyModelSnapshot as Native;

impl DiffAlgebra<Native> for ReferenceEnergyModelDiff {
    fn is_empty(&self) -> bool {
        <Self as DiffAlgebra<EnergyModelSnapshot>>::is_empty(self)
    }
}

impl MutationDiff<Native> for ReferenceEnergyModelDiff {}

impl ColdMutationDiff<Native> for ReferenceEnergyModelDiff {
    fn absorb(&mut self, other: Self) {
        <Self as ColdMutationDiff<EnergyModelSnapshot>>::absorb(self, other)
    }

    fn apply(&self, base: &mut Native, capability: ApplyCapability) -> MutationApplyResult<()> {
        let refuse = |error: semio_framework_value::ValueError| protocol::MutationApplyError { code: "mutation.native-bridge".into(), message: error.to_string(), target: Vec::new() };
        let mut reference = base.to_reference().map_err(refuse)?;
        <Self as ColdMutationDiff<EnergyModelSnapshot>>::apply(self, &mut reference, capability)?;
        let next = Native::from_reference(&reference).map_err(refuse)?;
        let displaced = std::mem::replace(base, next);
        displaced.retire().map_err(refuse)?;
        Ok(())
    }
}
