//! 🔎️ A semantic intent changes only the named glazing optical field.
use crate::{diff::EnergyModelDiff,mutations::EnergyModelMutation,EnergyModelSnapshot};
use semio_framework_value_derive::{FromValue as FromValueDerive,ToValue as ToValueDerive};
/// 🪟️ Visible front reflectance is addressed by the original glazing row identity.
#[derive(Clone,Debug,PartialEq,ToValueDerive,FromValueDerive,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase")]
#[dsl(keyword="change-glazing-material-visible-reflectance-front")]
pub struct ChangeGlazingMaterialVisibleReflectanceFront{pub id:crate::model::EntityId,pub new_visible_reflectance_front:f64}
/// 🎯️ Constructs the explicit optical field intent.
pub fn change_glazing_material_visible_reflectance_front(id:crate::model::EntityId,new_visible_reflectance_front:f64)->EnergyModelMutation{EnergyModelMutation::ChangeGlazingMaterialVisibleReflectanceFront(ChangeGlazingMaterialVisibleReflectanceFront{id,new_visible_reflectance_front})}
impl protocol::MutationKind<EnergyModelSnapshot,EnergyModelMutation> for ChangeGlazingMaterialVisibleReflectanceFront{
 const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"change",entity:"glazing-material",kind:"change-glazing-material-visible-reflectance-front",record:"ChangedGlazingMaterialVisibleReflectanceFront"};


 fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native(&format!("Visible front reflectance of glazing material {}",self.id.0),&format!("Sichtbarer Reflexionsgrad vorne von Verglasungsmaterial {}",self.id.0))}
 fn target(&self)->Vec<String>{vec![self.id.0.to_string()]}
}

impl protocol::ColdMutationKind<EnergyModelSnapshot,EnergyModelMutation> for ChangeGlazingMaterialVisibleReflectanceFront{
 fn diff(&self,base:&EnergyModelSnapshot)->protocol::MutationOutcome<EnergyModelDiff>{super::diff::diff(self,base)}
 fn inverse(&self,base:&EnergyModelSnapshot)->Result<Vec<EnergyModelMutation>,semio_framework_value::ValueError>{super::inverse::inverse(self,base)}
}
