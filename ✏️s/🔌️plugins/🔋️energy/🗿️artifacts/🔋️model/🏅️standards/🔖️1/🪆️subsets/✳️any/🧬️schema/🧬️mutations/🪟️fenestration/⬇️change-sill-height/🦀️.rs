//! ⬇️ Energy model mutation — `ChangeFenestrationSillHeight`: Sets the height of the glazing's sill above the host surface's base in metres.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⬇️ `change-fenestration-sill-height` payload. Sets the height of the glazing's sill above the host surface's base in metres.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-sill-height")]
pub struct ChangeFenestrationSillHeight {
    pub id: crate::model::EntityId,
    pub new_sill_height_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_sill_height<Op: From<ChangeFenestrationSillHeight>>(id: crate::model::EntityId, new_sill_height_m: f64) -> Op {
    ChangeFenestrationSillHeight { id, new_sill_height_m }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationSillHeight
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeFenestrationSillHeight>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-sill-height", record: "ChangedFenestrationSillHeight" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change fenestration {} sill height to {} m", self.id.0, self.new_sill_height_m), &format!("Brüstungshöhe von Fenster {} auf {} m ändern", self.id.0, self.new_sill_height_m))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationSillHeight
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeFenestrationSillHeight>,
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
