//! 🍃️ Energy model mutation — `ChangeSurfaceWindExposed`: Sets whether the outside face uses the wind-driven exterior convection correlation or the sheltered one.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍃️ `change-surface-wind-exposed` payload. Sets whether the outside face uses the wind-driven exterior convection correlation or the sheltered one.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-wind-exposed")]
pub struct ChangeSurfaceWindExposed {
    pub id: crate::model::EntityId,
    pub new_wind_exposed: bool,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_surface_wind_exposed<Op: From<ChangeSurfaceWindExposed>>(id: crate::model::EntityId, new_wind_exposed: bool) -> Op {
    ChangeSurfaceWindExposed { id, new_wind_exposed }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSurfaceWindExposed
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSurfaceWindExposed>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change surface {} wind exposure to {}", self.id.0, self.new_wind_exposed), &format!("Windexposition von Oberfläche {} auf {} ändern", self.id.0, self.new_wind_exposed))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSurfaceWindExposed
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSurfaceWindExposed>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "surface", kind: "change-surface-wind-exposed", record: "ChangedSurfaceWindExposed" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
