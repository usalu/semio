//! ⛏️ Energy model mutation — `ChangeGroundDeep`: Sets the deep ground temperature, read by the ground heat transfer solve.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛏️ `change-ground-deep` payload. Sets the deep ground temperature, read by the ground heat transfer solve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ground-deep")]
pub struct ChangeGroundDeep {
    pub new_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ground_deep(new_temperature_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGroundDeep(ChangeGroundDeep { new_temperature_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGroundDeep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ground", kind: "change-ground-deep", record: "ChangedGroundDeep" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change deep ground temperature to {} °C", self.new_temperature_c), &format!("Erdreichtemperatur in der Tiefe auf {} °C ändern", self.new_temperature_c))
    }
}
//#endregion 🔖️Mutation
