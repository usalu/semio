//! 🧮️ Energy model mutation — `ChangeSpaceFloorArea`: Sets one space's floor area in square metres — the denominator every per-area gain in that space is expanded against.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧮️ `change-space-floor-area` payload. Sets one space's floor area in square metres — the denominator every per-area gain in that space is expanded against.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-space-floor-area")]
pub struct ChangeSpaceFloorArea {
    pub id: crate::model::EntityId,
    pub new_floor_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_space_floor_area<Op: From<ChangeSpaceFloorArea>>(id: crate::model::EntityId, new_floor_area_m2: f64) -> Op {
    ChangeSpaceFloorArea { id, new_floor_area_m2 }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSpaceFloorArea
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSpaceFloorArea>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "space", kind: "change-space-floor-area", record: "ChangedSpaceFloorArea" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change space {} floor area to {} m²", self.id.0, self.new_floor_area_m2), &format!("Geschossfläche von Raum {} auf {} m² ändern", self.id.0, self.new_floor_area_m2))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSpaceFloorArea
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSpaceFloorArea>,
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
