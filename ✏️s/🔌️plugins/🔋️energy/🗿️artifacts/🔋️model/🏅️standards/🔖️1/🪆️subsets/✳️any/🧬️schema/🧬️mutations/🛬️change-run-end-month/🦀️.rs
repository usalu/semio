//! 🛬️ Energy model mutation — `ChangeRunEndMonth`: Sets the month the simulation run period ends in; the period must stay a calendar interval of its year.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛬️ `change-run-end-month` payload. Sets the month the simulation run period ends in; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-end-month")]
pub struct ChangeRunEndMonth {
    pub new_end_month: u8,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_run_end_month(new_end_month: u8) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRunEndMonth(ChangeRunEndMonth { new_end_month })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRunEndMonth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "run", kind: "change-run-end-month", record: "ChangedRunEndMonth" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change run period end month to {}", self.new_end_month), &format!("Endmonat des Simulationszeitraums auf {} ändern", self.new_end_month))
    }
}
//#endregion 🔖️Mutation
