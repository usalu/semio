//! 🧊️ Explicit reference mutation behavior retains its concrete cold capabilities.
use super::ChangeGlazingMaterialVisibleReflectanceBack;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot,Op> for ChangeGlazingMaterialVisibleReflectanceBack
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeGlazingMaterialVisibleReflectanceBack>,
{
 const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"change",entity:"glazing-material",kind:"change-glazing-material-visible-reflectance-back",record:"ChangedGlazingMaterialVisibleReflectanceBack"};


 fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native(&format!("Visible back reflectance of glazing material {}",self.id.0),&format!("Sichtbarer Reflexionsgrad hinten von Verglasungsmaterial {}",self.id.0))}
 fn target(&self)->Vec<String>{vec![self.id.0.to_string()]}
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot,Op> for ChangeGlazingMaterialVisibleReflectanceBack
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeGlazingMaterialVisibleReflectanceBack>,
{
 fn diff(&self,base:&EnergyModelSnapshot)->protocol::MutationOutcome<EnergyModelDiff>{super::diff::diff(self,base)}
 fn inverse(&self,base:&EnergyModelSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{super::inverse::inverse(self,base)}
}
