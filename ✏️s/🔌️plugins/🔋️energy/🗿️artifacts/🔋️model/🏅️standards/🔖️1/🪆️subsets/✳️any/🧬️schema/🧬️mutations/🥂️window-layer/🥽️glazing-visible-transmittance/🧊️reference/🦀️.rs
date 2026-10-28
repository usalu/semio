//! 🧊️ Whole-value reference semantics are separate from the authored retained payload and native producer.
use super::ChangeGlazingMaterialVisibleTransmittance;
use crate::{snapshot::reference::EnergyModelSnapshot,diff::ReferenceEnergyModelDiff as EnergyModelDiff};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeGlazingMaterialVisibleTransmittance
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeGlazingMaterialVisibleTransmittance>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "glazing-material", kind: "change-glazing-material-visible-transmittance", record: "ChangedGlazingMaterialVisibleTransmittance" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Glazing Material Visible Transmittance of glazing material {}", self.id.0), &format!("Lichttransmissionsgrad von Verglasungsmaterial {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeGlazingMaterialVisibleTransmittance
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeGlazingMaterialVisibleTransmittance>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
