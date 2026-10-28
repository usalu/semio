//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::AddElectricalLoadCenterBattery;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for AddElectricalLoadCenterBattery
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<AddElectricalLoadCenterBattery> + From<crate::RemoveElectricalLoadCenterBattery>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "electrical-load-center", kind: "add-electrical-load-center-battery", record: "AddedElectricalLoadCenterBattery" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add battery {} to electrical load center {}", self.battery_id.0, self.id.0), &format!("Batterie {} zu Stromverteiler {} hinzufügen", self.battery_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for AddElectricalLoadCenterBattery
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<AddElectricalLoadCenterBattery> + From<crate::RemoveElectricalLoadCenterBattery>,
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
