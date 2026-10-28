//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::AddElectricalLoadCenterPv;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for AddElectricalLoadCenterPv
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<AddElectricalLoadCenterPv> + From<crate::RemoveElectricalLoadCenterPv>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "electrical-load-center", kind: "add-electrical-load-center-pv", record: "AddedElectricalLoadCenterPv" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add pv system {} to electrical load center {}", self.pv_id.0, self.id.0), &format!("PV-Anlage {} zu Stromverteiler {} hinzufügen", self.pv_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for AddElectricalLoadCenterPv
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<AddElectricalLoadCenterPv> + From<crate::RemoveElectricalLoadCenterPv>,
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
