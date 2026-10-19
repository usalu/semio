//! 🏟️ Energy model mutation — `CreateThermalEnclosure`: Adds one thermal enclosure — the named set of zones that share one continuous envelope boundary, which is what an envelope-area report normalizes over.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏟️ `create-thermal-enclosure` payload. Adds one thermal enclosure — the named set of zones that share one continuous envelope boundary, which is what an envelope-area report normalizes over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-thermal-enclosure")]
pub struct CreateThermalEnclosure {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub zone_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_thermal_enclosure<Op: From<CreateThermalEnclosure>>(index: u32, id: crate::model::EntityId, name: String, zone_ids: Vec<crate::model::EntityId>) -> Op {
    CreateThermalEnclosure { index, id, name, zone_ids }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateThermalEnclosure
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateThermalEnclosure> + From<crate::DeleteThermalEnclosure>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Thermal Enclosure {} at index {}", self.id.0, self.index), &format!("Zonenverbund {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateThermalEnclosure
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateThermalEnclosure> + From<crate::DeleteThermalEnclosure>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "thermal-enclosure", kind: "create-thermal-enclosure", record: "CreatedThermalEnclosure" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
