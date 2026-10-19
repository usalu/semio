//! 🏋️ Energy model mutation — `ChangeRefrigerationSystemDesignLoad`: Sets design load (W) on one refrigeration system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏋️ `change-refrigeration-system-design-load` payload. Sets design load (W) on one refrigeration system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-refrigeration-system-design-load")]
pub struct ChangeRefrigerationSystemDesignLoad {
    pub id: crate::model::EntityId,
    pub new_design_load_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_refrigeration_system_design_load<Op: From<ChangeRefrigerationSystemDesignLoad>>(id: crate::model::EntityId, new_design_load_w: f64) -> Op {
    ChangeRefrigerationSystemDesignLoad { id, new_design_load_w }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeRefrigerationSystemDesignLoad
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeRefrigerationSystemDesignLoad>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "refrigeration-system", kind: "change-refrigeration-system-design-load", record: "ChangedRefrigerationSystemDesignLoad" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Refrigeration System Design Load of refrigeration system {}", self.id.0), &format!("Auslegungslast von Kälteanlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeRefrigerationSystemDesignLoad
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeRefrigerationSystemDesignLoad>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
