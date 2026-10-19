//! 🏘️ Energy model mutation — `CreateZone`: Adds one thermal zone the caller has already chosen an id for — no mutation mints an id, so an undo can name the same zone again.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏘️ `create-zone` payload. Adds one thermal zone the caller has already chosen an id for — no mutation mints an id, so an undo can name the same zone again.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-zone")]
pub struct CreateZone {
    pub id: crate::model::EntityId,
    pub name: String,
    pub volume_m3: f64,
    pub multiplier: u32,
    pub conditioned: bool,
    pub part_of_total_floor_area: bool,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_zone<Op: From<CreateZone>>(id: crate::model::EntityId, name: String, volume_m3: f64, multiplier: u32, conditioned: bool, part_of_total_floor_area: bool, index: Option<u32>) -> Op {
    CreateZone { id, name, volume_m3, multiplier, conditioned, part_of_total_floor_area, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateZone> + From<crate::DeleteZone>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create zone \"{}\"", self.name), &format!("Zone \"{}\" erstellen", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateZone
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateZone> + From<crate::DeleteZone>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "zone", kind: "create-zone", record: "CreatedZone" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
