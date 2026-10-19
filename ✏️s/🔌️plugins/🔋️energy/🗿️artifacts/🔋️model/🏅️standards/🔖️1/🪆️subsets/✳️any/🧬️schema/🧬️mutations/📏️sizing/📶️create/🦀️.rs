//! 📶️ Energy model mutation — `CreateSizingObject`: Declares that one zone is autosized against a design day: which load the sizing run solves for and which design day it reads.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📶️ `create-sizing-object` payload. Declares that one zone is autosized against a design day: which load the sizing run solves for and which design day it reads.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-sizing-object")]
pub struct CreateSizingObject {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub sizing_type: crate::model::SizingType,
    pub design_day_type: crate::model::DesignDayType,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_sizing_object<Op: From<CreateSizingObject>>(id: crate::model::EntityId, zone_id: crate::model::EntityId, sizing_type: crate::model::SizingType, design_day_type: crate::model::DesignDayType, index: Option<u32>) -> Op {
    CreateSizingObject { id, zone_id, sizing_type, design_day_type, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateSizingObject
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateSizingObject> + From<crate::DeleteSizingObject>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "sizing-object", kind: "create-sizing-object", record: "CreatedSizingObject" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create sizing object {}", self.id.0), &format!("Auslegungsobjekt {} erstellen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateSizingObject
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateSizingObject> + From<crate::DeleteSizingObject>,
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
