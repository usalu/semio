//! 🌡️ Energy model mutation — `ChangeGroundBuilding`: Sets one month's ground temperature under the building, read by the ground heat transfer solve.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌡️ `change-ground-building` payload. Sets one month's ground temperature under the building, read by the ground heat transfer solve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ground-building")]
pub struct ChangeGroundBuilding {
    pub month: u8,
    pub new_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ground_building(month: u8, new_temperature_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGroundBuilding(ChangeGroundBuilding { month, new_temperature_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGroundBuilding {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ground", kind: "change-ground-building", record: "ChangedGroundBuilding" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change building-surface ground temperature of month {} to {} °C", self.month, self.new_temperature_c), &format!("Erdreichtemperatur unter dem Gebäude im Monat {} auf {} °C ändern", self.month, self.new_temperature_c))
    }
}
//#endregion 🔖️Mutation
