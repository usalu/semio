//! ⚡️ Energy model mutation — `ChangeEquipmentGainWattsPerArea`: Sets equipment power density (W/m²) on one equipment gain, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚡️ `change-equipment-gain-watts-per-area` payload. Sets equipment power density (W/m²) on one equipment gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-watts-per-area")]
pub struct ChangeEquipmentGainWattsPerArea {
    pub id: crate::model::EntityId,
    pub new_watts_per_area: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_equipment_gain_watts_per_area<Op: From<ChangeEquipmentGainWattsPerArea>>(id: crate::model::EntityId, new_watts_per_area: f64) -> Op {
    ChangeEquipmentGainWattsPerArea { id, new_watts_per_area }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeEquipmentGainWattsPerArea
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeEquipmentGainWattsPerArea>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Equipment Gain Watts Per Area of equipment gain {}", self.id.0), &format!("Leistung pro Fläche von Gerätewärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeEquipmentGainWattsPerArea
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeEquipmentGainWattsPerArea>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "equipment-gain", kind: "change-equipment-gain-watts-per-area", record: "ChangedEquipmentGainWattsPerArea" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
