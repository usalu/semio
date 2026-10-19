//! 🌗️ Energy model mutation — `DeleteDaylightZone`: Removes the daylighting control from its zone. Refused when no control carries the id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌗️ `delete-daylight-zone` payload. Removes the daylighting control from its zone. Refused when no control carries the id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-daylight-zone")]
pub struct DeleteDaylightZone {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_daylight_zone<Op: From<DeleteDaylightZone>>(id: crate::model::EntityId) -> Op {
    DeleteDaylightZone { id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteDaylightZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteDaylightZone> + From<crate::CreateDaylightZone>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "daylight-zone", kind: "delete-daylight-zone", record: "DeletedDaylightZone" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete daylight zone {}", self.id.0), &format!("Tageslichtzone {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DeleteDaylightZone
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DeleteDaylightZone> + From<crate::CreateDaylightZone>,
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
