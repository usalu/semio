//! 🧢️ Energy model mutation — `ChangeFenestrationOverhangDepth`: Sets how far the horizontal projection above the window head reaches out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 610/910 are exactly this field.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧢️ `change-fenestration-overhang-depth` payload. Sets how far the horizontal projection above the window head reaches out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 610/910 are exactly this field.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-overhang-depth")]
pub struct ChangeFenestrationOverhangDepth {
    pub id: crate::model::EntityId,
    pub new_overhang_depth_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_overhang_depth<Op: From<ChangeFenestrationOverhangDepth>>(id: crate::model::EntityId, new_overhang_depth_m: f64) -> Op {
    ChangeFenestrationOverhangDepth { id, new_overhang_depth_m }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationOverhangDepth
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeFenestrationOverhangDepth>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change fenestration {} overhang depth to {} m", self.id.0, self.new_overhang_depth_m), &format!("Überstandstiefe von Fenster {} auf {} m ändern", self.id.0, self.new_overhang_depth_m))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationOverhangDepth
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeFenestrationOverhangDepth>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-overhang-depth", record: "ChangedFenestrationOverhangDepth" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
