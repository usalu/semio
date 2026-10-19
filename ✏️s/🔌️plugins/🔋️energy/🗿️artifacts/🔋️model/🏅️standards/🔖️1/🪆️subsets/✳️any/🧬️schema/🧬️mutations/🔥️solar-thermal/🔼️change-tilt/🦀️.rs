//! 🔼️ Energy model mutation — `ChangeSolarThermalSystemTilt`: Sets tilt on one solar thermal system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔼️ `change-solar-thermal-system-tilt` payload. Sets tilt on one solar thermal system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-solar-thermal-system-tilt")]
pub struct ChangeSolarThermalSystemTilt {
    pub id: crate::model::EntityId,
    pub new_tilt_deg: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_solar_thermal_system_tilt<Op: From<ChangeSolarThermalSystemTilt>>(id: crate::model::EntityId, new_tilt_deg: f64) -> Op {
    ChangeSolarThermalSystemTilt { id, new_tilt_deg }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSolarThermalSystemTilt
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSolarThermalSystemTilt>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "solar-thermal-system", kind: "change-solar-thermal-system-tilt", record: "ChangedSolarThermalSystemTilt" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Solar Thermal System Tilt of solar thermal system {}", self.id.0), &format!("Neigung von Solarthermieanlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSolarThermalSystemTilt
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSolarThermalSystemTilt>,
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
//#endregion 🔖️Mutation
