//! ⏹️ Energy model mutation — `ChangeRunEndDay`: Sets the day of month the simulation run period ends on; the period must stay a calendar interval of its year.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏹️ `change-run-end-day` payload. Sets the day of month the simulation run period ends on; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-end-day")]
pub struct ChangeRunEndDay {
    pub new_end_day: u8,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_run_end_day<Op: From<ChangeRunEndDay>>(new_end_day: u8) -> Op {
    ChangeRunEndDay { new_end_day }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeRunEndDay
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeRunEndDay>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "run", kind: "change-run-end-day", record: "ChangedRunEndDay" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change run period end day to {}", self.new_end_day), &format!("Endtag des Simulationszeitraums auf {} ändern", self.new_end_day))
    }
}
//#endregion 🔖️Mutation
