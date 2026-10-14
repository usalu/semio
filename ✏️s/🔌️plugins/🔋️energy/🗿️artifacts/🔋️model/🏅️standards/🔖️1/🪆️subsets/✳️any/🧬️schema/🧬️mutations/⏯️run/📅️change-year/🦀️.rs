//! 📅️ Energy model mutation — `ChangeRunYear`: Sets the calendar year that fixes the run period's weekdays and leap day; the period must stay a calendar interval of that year.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📅️ `change-run-year` payload. Sets the calendar year that fixes the run period's weekdays and leap day; the period must stay a calendar interval of that year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-year")]
pub struct ChangeRunYear {
    pub new_year: u16,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_run_year(new_year: u16) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRunYear(ChangeRunYear { new_year })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRunYear {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "run", kind: "change-run-year", record: "ChangedRunYear" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change run period year to {}", self.new_year), &format!("Jahr des Simulationszeitraums auf {} ändern", self.new_year))
    }
}
//#endregion 🔖️Mutation
