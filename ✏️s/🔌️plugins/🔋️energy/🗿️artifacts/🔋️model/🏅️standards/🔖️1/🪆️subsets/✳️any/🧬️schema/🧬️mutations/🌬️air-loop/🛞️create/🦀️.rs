//! 🛞️ Energy model mutation — `CreateAirLoop`: Creates one air loop with its supply and return node ids, its design supply air flow and the zones its terminals serve. The terminal list is kept ascending and free of duplicates so a later membership removal's undo is exact.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛞️ `create-air-loop` payload. Creates one air loop with its supply and return node ids, its design supply air flow and the zones its terminals serve. The terminal list is kept ascending and free of duplicates so a later membership removal's undo is exact.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-air-loop")]
pub struct CreateAirLoop {
    pub id: crate::model::EntityId,
    pub name: String,
    pub supply_node_id: u32,
    pub return_node_id: u32,
    pub design_supply_air_flow_m3_s: f64,
    pub terminal_zone_ids: Vec<crate::model::EntityId>,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_air_loop<Op: From<CreateAirLoop>>(id: crate::model::EntityId, name: String, supply_node_id: u32, return_node_id: u32, design_supply_air_flow_m3_s: f64, terminal_zone_ids: Vec<crate::model::EntityId>, index: Option<u32>) -> Op {
    CreateAirLoop { id, name, supply_node_id, return_node_id, design_supply_air_flow_m3_s, terminal_zone_ids, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateAirLoop
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateAirLoop> + From<crate::DeleteAirLoop>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "air-loop", kind: "create-air-loop", record: "CreatedAirLoop" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create air loop {}", self.id.0), &format!("Luftkreislauf {} erstellen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
