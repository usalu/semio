//! 🏚️ Energy model mutation — `DeleteZone`: Removes one thermal zone. It RESTRICTS rather than cascades: while any space, surface, gain, HVAC object, grouping or airflow node still names the zone it is refused with `mutation.target-referenced`, because a cascade here would have to delete surfaces, which cascade again to fenestrations and adjacency pairs.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏚️ `delete-zone` payload. Removes one thermal zone. It RESTRICTS rather than cascades: while any space, surface, gain, HVAC object, grouping or airflow node still names the zone it is refused with `mutation.target-referenced`, because a cascade here would have to delete surfaces, which cascade again to fenestrations and adjacency pairs.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-zone")]
pub struct DeleteZone {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_zone<Op: From<DeleteZone>>(id: crate::model::EntityId) -> Op {
    DeleteZone { id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteZone
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteZone> + From<crate::CreateZone>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "zone", kind: "delete-zone", record: "DeletedZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete zone {}", self.id.0), &format!("Zone {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
