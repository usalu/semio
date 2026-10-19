//! 🌡️ A semantic intent changes only the named glazing optical field.
use crate::{diff::EnergyModelDiff,EnergyModelSnapshot};
use semio_framework_value_derive::{FromValue as FromValueDerive,ToValue as ToValueDerive};
/// 🪟️ Infrared transmittance is addressed by the original glazing row identity.
#[derive(Clone,Debug,PartialEq,ToValueDerive,FromValueDerive,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase")]
#[dsl(keyword="change-glazing-material-infrared-transmittance")]
pub struct ChangeGlazingMaterialInfraredTransmittance{pub id:crate::model::EntityId,pub new_infrared_transmittance:f64}
/// 🎯️ Constructs the explicit optical field intent.
pub fn change_glazing_material_infrared_transmittance<Op: From<ChangeGlazingMaterialInfraredTransmittance>>(id:crate::model::EntityId,new_infrared_transmittance:f64)->Op{ChangeGlazingMaterialInfraredTransmittance{id,new_infrared_transmittance}.into()}
impl<Op> protocol::MutationKind<EnergyModelSnapshot,Op> for ChangeGlazingMaterialInfraredTransmittance
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeGlazingMaterialInfraredTransmittance>,
{
 const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"change",entity:"glazing-material",kind:"change-glazing-material-infrared-transmittance",record:"ChangedGlazingMaterialInfraredTransmittance"};


 fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native(&format!("Infrared transmittance of glazing material {}",self.id.0),&format!("Infraroter Transmissionsgrad von Verglasungsmaterial {}",self.id.0))}
 fn target(&self)->Vec<String>{vec![self.id.0.to_string()]}
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot,Op> for ChangeGlazingMaterialInfraredTransmittance
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeGlazingMaterialInfraredTransmittance>,
{
 fn diff(&self,base:&EnergyModelSnapshot)->protocol::MutationOutcome<EnergyModelDiff>{super::diff::diff(self,base)}
 fn inverse(&self,base:&EnergyModelSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{super::inverse::inverse(self,base)}
}
