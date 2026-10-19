//! 🛢️ Energy model mutation — `ChangeShwSystemStorageVolume`: Sets storage volume (m³) on one service hot water system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛢️ `change-shw-system-storage-volume` payload. Sets storage volume (m³) on one service hot water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shw-system-storage-volume")]
pub struct ChangeShwSystemStorageVolume {
    pub id: crate::model::EntityId,
    pub new_storage_volume_m3: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_shw_system_storage_volume<Op: From<ChangeShwSystemStorageVolume>>(id: crate::model::EntityId, new_storage_volume_m3: f64) -> Op {
    ChangeShwSystemStorageVolume { id, new_storage_volume_m3 }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeShwSystemStorageVolume
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeShwSystemStorageVolume>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "service-hot-water-system", kind: "change-shw-system-storage-volume", record: "ChangedShwSystemStorageVolume" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Shw System Storage Volume of service hot water system {}", self.id.0), &format!("Speichervolumen von Trinkwarmwasseranlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeShwSystemStorageVolume
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeShwSystemStorageVolume>,
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
