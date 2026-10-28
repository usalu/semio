//! 🧊️ Whole-value reference semantics are separate from the authored retained payload and native producer.
use super::ChangeSizingObjectSizingType;
use crate::{snapshot::reference::EnergyModelSnapshot,diff::ReferenceEnergyModelDiff as EnergyModelDiff};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSizingObjectSizingType
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSizingObjectSizingType>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sizing-object", kind: "change-sizing-object-sizing-type", record: "ChangedSizingObjectSizingType" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change sizing object {} sizing type to {:?}", self.id.0, self.new_sizing_type), &format!("Auslegungsart von Auslegungsobjekt {} auf {:?} ändern", self.id.0, self.new_sizing_type))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSizingObjectSizingType
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSizingObjectSizingType>,
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
