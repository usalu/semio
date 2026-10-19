//! 🧃️ Energy model mutation — `ChangeSolarThermalSystemStorageVolume`: Sets buffer volume (m³) on one solar thermal system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧃️ `change-solar-thermal-system-storage-volume` payload. Sets buffer volume (m³) on one solar thermal system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-solar-thermal-system-storage-volume")]
pub struct ChangeSolarThermalSystemStorageVolume {
    pub id: crate::model::EntityId,
    pub new_storage_volume_m3: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_solar_thermal_system_storage_volume<Op: From<ChangeSolarThermalSystemStorageVolume>>(id: crate::model::EntityId, new_storage_volume_m3: f64) -> Op {
    ChangeSolarThermalSystemStorageVolume { id, new_storage_volume_m3 }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSolarThermalSystemStorageVolume
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSolarThermalSystemStorageVolume>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Solar Thermal System Storage Volume of solar thermal system {}", self.id.0), &format!("Speichervolumen von Solarthermieanlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSolarThermalSystemStorageVolume
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSolarThermalSystemStorageVolume>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "solar-thermal-system", kind: "change-solar-thermal-system-storage-volume", record: "ChangedSolarThermalSystemStorageVolume" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
