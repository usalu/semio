//! 🍵️ Energy model mutation — `ChangeShwSystemHeaterCapacity`: Sets heater capacity (W) on one service hot water system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍵️ `change-shw-system-heater-capacity` payload. Sets heater capacity (W) on one service hot water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shw-system-heater-capacity")]
pub struct ChangeShwSystemHeaterCapacity {
    pub id: crate::model::EntityId,
    pub new_heater_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_shw_system_heater_capacity<Op: From<ChangeShwSystemHeaterCapacity>>(id: crate::model::EntityId, new_heater_capacity_w: f64) -> Op {
    ChangeShwSystemHeaterCapacity { id, new_heater_capacity_w }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeShwSystemHeaterCapacity
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeShwSystemHeaterCapacity>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "service-hot-water-system", kind: "change-shw-system-heater-capacity", record: "ChangedShwSystemHeaterCapacity" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Shw System Heater Capacity of service hot water system {}", self.id.0), &format!("Heizleistung von Trinkwarmwasseranlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeShwSystemHeaterCapacity
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeShwSystemHeaterCapacity>,
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
