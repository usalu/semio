//! 🐬️ Energy model mutation — `ChangeFenestrationFinDepth`: Sets how far the two vertical projections beside the window jambs reach out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 630/930 are exactly this field.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🐬️ `change-fenestration-fin-depth` payload. Sets how far the two vertical projections beside the window jambs reach out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 630/930 are exactly this field.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-fin-depth")]
pub struct ChangeFenestrationFinDepth {
    pub id: crate::model::EntityId,
    pub new_fin_depth_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_fin_depth<Op: From<ChangeFenestrationFinDepth>>(id: crate::model::EntityId, new_fin_depth_m: f64) -> Op {
    ChangeFenestrationFinDepth { id, new_fin_depth_m }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationFinDepth
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeFenestrationFinDepth>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-fin-depth", record: "ChangedFenestrationFinDepth" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change fenestration {} fin depth to {} m", self.id.0, self.new_fin_depth_m), &format!("Seitenblendentiefe von Fenster {} auf {} m ändern", self.id.0, self.new_fin_depth_m))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationFinDepth
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeFenestrationFinDepth>,
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
