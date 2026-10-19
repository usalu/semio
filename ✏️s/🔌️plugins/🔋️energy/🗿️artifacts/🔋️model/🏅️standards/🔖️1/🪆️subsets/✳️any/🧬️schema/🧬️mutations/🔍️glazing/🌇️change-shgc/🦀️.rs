//! 🌇️ Energy model mutation — `ChangeFenestrationShgc`: Sets the solar heat gain coefficient — the fraction of incident solar the glazing passes to the zone, directly and by re-radiation.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌇️ `change-fenestration-shgc` payload. Sets the solar heat gain coefficient — the fraction of incident solar the glazing passes to the zone, directly and by re-radiation.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-shgc")]
pub struct ChangeFenestrationShgc {
    pub id: crate::model::EntityId,
    pub new_shgc: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_shgc<Op: From<ChangeFenestrationShgc>>(id: crate::model::EntityId, new_shgc: f64) -> Op {
    ChangeFenestrationShgc { id, new_shgc }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationShgc
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeFenestrationShgc>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-shgc", record: "ChangedFenestrationShgc" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change fenestration {} SHGC to {}", self.id.0, self.new_shgc), &format!("g-Wert von Fenster {} auf {} ändern", self.id.0, self.new_shgc))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationShgc
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeFenestrationShgc>,
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
