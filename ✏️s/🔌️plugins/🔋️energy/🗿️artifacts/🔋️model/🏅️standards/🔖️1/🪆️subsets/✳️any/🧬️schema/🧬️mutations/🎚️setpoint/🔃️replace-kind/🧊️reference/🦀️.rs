//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::ReplaceSetpointManagerKind;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ReplaceSetpointManagerKind
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ReplaceSetpointManagerKind>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "setpoint-manager", kind: "replace-setpoint-manager-kind", record: "ReplacedSetpointManagerKind" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace setpoint manager {} control law with {:?}", self.id.0, self.new_kind), &format!("Regelgesetz von Sollwertführung {} durch {:?} ersetzen", self.id.0, self.new_kind))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ReplaceSetpointManagerKind
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ReplaceSetpointManagerKind>,
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
