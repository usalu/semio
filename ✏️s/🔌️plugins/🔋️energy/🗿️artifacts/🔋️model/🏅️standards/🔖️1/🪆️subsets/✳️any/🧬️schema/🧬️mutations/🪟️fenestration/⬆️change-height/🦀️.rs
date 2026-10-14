//! ⬆️ Energy model mutation — `ChangeFenestrationHeight`: Sets the glazing's head-to-sill height in metres — the length the overhang and fin shadow geometry is projected against.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⬆️ `change-fenestration-height` payload. Sets the glazing's head-to-sill height in metres — the length the overhang and fin shadow geometry is projected against.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-height")]
pub struct ChangeFenestrationHeight {
    pub id: crate::model::EntityId,
    pub new_height_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_height<Op: From<ChangeFenestrationHeight>>(id: crate::model::EntityId, new_height_m: f64) -> Op {
    ChangeFenestrationHeight { id, new_height_m }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationHeight
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeFenestrationHeight>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-height", record: "ChangedFenestrationHeight" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change fenestration {} height to {} m", self.id.0, self.new_height_m), &format!("Höhe von Fenster {} auf {} m ändern", self.id.0, self.new_height_m))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
