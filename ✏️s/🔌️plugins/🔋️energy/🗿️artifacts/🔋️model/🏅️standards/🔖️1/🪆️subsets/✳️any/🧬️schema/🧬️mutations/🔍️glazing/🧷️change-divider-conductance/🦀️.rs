//! 🧷️ Energy model mutation — `ChangeFenestrationDividerConductance`: Sets the divider bars' whole-assembly thermal conductance in W/K, the frame term's sibling for the muntins inside the glazed area.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧷️ `change-fenestration-divider-conductance` payload. Sets the divider bars' whole-assembly thermal conductance in W/K, the frame term's sibling for the muntins inside the glazed area.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-divider-conductance")]
pub struct ChangeFenestrationDividerConductance {
    pub id: crate::model::EntityId,
    pub new_divider_conductance_w_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_divider_conductance<Op: From<ChangeFenestrationDividerConductance>>(id: crate::model::EntityId, new_divider_conductance_w_k: f64) -> Op {
    ChangeFenestrationDividerConductance { id, new_divider_conductance_w_k }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationDividerConductance
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeFenestrationDividerConductance>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-divider-conductance", record: "ChangedFenestrationDividerConductance" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change fenestration {} divider conductance to {} W/K", self.id.0, self.new_divider_conductance_w_k), &format!("Sprossen-Wärmeleitwert von Fenster {} auf {} W/K ändern", self.id.0, self.new_divider_conductance_w_k))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
