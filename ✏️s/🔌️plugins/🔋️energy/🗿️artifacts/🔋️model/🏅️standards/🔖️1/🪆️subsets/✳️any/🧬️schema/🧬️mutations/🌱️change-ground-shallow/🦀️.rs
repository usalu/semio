//! 🌱️ Energy model mutation — `ChangeGroundShallow`: Sets one month's shallow ground temperature, read by the ground heat transfer solve.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌱️ `change-ground-shallow` payload. Sets one month's shallow ground temperature, read by the ground heat transfer solve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ground-shallow")]
pub struct ChangeGroundShallow {
    pub month: u8,
    pub new_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ground_shallow(month: u8, new_temperature_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGroundShallow(ChangeGroundShallow { month, new_temperature_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGroundShallow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ground", kind: "change-ground-shallow", record: "ChangedGroundShallow" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change shallow ground temperature of month {} to {} °C", self.month, self.new_temperature_c), &format!("Oberflächennahe Erdreichtemperatur im Monat {} auf {} °C ändern", self.month, self.new_temperature_c))
    }
}
//#endregion 🔖️Mutation
