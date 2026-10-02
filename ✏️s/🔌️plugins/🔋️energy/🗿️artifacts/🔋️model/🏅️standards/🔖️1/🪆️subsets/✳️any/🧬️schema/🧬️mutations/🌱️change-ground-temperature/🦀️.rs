//! 🌱️ Energy model mutation — `ChangeGroundTemperatureShallow`: Sets one month's shallow ground temperature, read by the ground heat transfer solve.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌱️ `change-ground-temperature-shallow` payload. Sets one month's shallow ground temperature, read by the ground heat transfer solve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ground-temperature-shallow")]
pub struct ChangeGroundTemperatureShallow {
    pub month: u8,
    pub new_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ground_temperature_shallow(month: u8, new_temperature_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGroundTemperatureShallow(ChangeGroundTemperatureShallow { month, new_temperature_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGroundTemperatureShallow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ground-temperature", kind: "change-ground-temperature-shallow", record: "ChangedGroundTemperatureShallow" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change shallow ground temperature of month {} to {} °C", self.month, self.new_temperature_c), &format!("Oberflächennahe Erdreichtemperatur im Monat {} auf {} °C ändern", self.month, self.new_temperature_c))
    }
}
//#endregion 🔖️Mutation
