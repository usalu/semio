//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::RemoveElectricalLoadCenterPv;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemoveElectricalLoadCenterPv
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemoveElectricalLoadCenterPv> + From<crate::AddElectricalLoadCenterPv>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "electrical-load-center", kind: "remove-electrical-load-center-pv", record: "RemovedElectricalLoadCenterPv" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove pv system {} from electrical load center {}", self.pv_id.0, self.id.0), &format!("PV-Anlage {} aus Stromverteiler {} entfernen", self.pv_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemoveElectricalLoadCenterPv
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemoveElectricalLoadCenterPv> + From<crate::AddElectricalLoadCenterPv>,
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
