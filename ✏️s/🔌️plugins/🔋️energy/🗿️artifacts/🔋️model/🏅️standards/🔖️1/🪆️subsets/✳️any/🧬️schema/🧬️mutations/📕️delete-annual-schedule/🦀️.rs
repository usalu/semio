//! 📕️ Energy model mutation — `DeleteAnnualSchedule`: Removes one rule-based year. Refused while any consumer still resolves its id. Its inverse is a cascade: the create that re-defines the year, then one insert per date rule in order, then one add per holiday — because the rules and the holidays are collections of their own and the create does not carry them.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📕️ `delete-annual-schedule` payload. Removes one rule-based year. Refused while any consumer still resolves its id. Its inverse is a cascade: the create that re-defines the year, then one insert per date rule in order, then one add per holiday — because the rules and the holidays are collections of their own and the create does not carry them.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-annual-schedule")]
pub struct DeleteAnnualSchedule {
    pub id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_annual_schedule(id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteAnnualSchedule(DeleteAnnualSchedule { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteAnnualSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "annual-schedule", kind: "delete-annual-schedule", record: "DeletedAnnualSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete annual schedule {}", self.id.0), &format!("Jahreszeitplan {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
