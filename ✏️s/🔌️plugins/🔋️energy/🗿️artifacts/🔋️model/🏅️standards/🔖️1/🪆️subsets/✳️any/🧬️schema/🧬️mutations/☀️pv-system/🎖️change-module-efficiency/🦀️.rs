//! 🎖️ Energy model mutation — `ChangePvSystemModuleEfficiency`: Sets module efficiency on one pv system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎖️ `change-pv-system-module-efficiency` payload. Sets module efficiency on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-module-efficiency")]
pub struct ChangePvSystemModuleEfficiency {
    pub id: crate::model::EntityId,
    pub new_module_efficiency: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_pv_system_module_efficiency<Op: From<ChangePvSystemModuleEfficiency>>(id: crate::model::EntityId, new_module_efficiency: f64) -> Op {
    ChangePvSystemModuleEfficiency { id, new_module_efficiency }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePvSystemModuleEfficiency
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePvSystemModuleEfficiency>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Pv System Module Efficiency of pv system {}", self.id.0), &format!("Modulwirkungsgrad von PV-Anlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePvSystemModuleEfficiency
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePvSystemModuleEfficiency>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pv-system", kind: "change-pv-system-module-efficiency", record: "ChangedPvSystemModuleEfficiency" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
