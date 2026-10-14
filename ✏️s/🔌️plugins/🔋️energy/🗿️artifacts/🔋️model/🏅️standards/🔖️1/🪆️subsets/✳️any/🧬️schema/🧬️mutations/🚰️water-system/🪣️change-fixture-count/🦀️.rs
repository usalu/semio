//! 🪣️ Energy model mutation — `ChangeWaterSystemFixtureCount`: Sets how many fixtures one water system carries.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪣️ `change-water-system-fixture-count` payload. Sets how many fixtures one water system carries.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-water-system-fixture-count")]
pub struct ChangeWaterSystemFixtureCount {
    pub id: crate::model::EntityId,
    pub new_fixture_count: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_water_system_fixture_count<Op: From<ChangeWaterSystemFixtureCount>>(id: crate::model::EntityId, new_fixture_count: u32) -> Op {
    ChangeWaterSystemFixtureCount { id, new_fixture_count }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeWaterSystemFixtureCount
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeWaterSystemFixtureCount>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "water-system", kind: "change-water-system-fixture-count", record: "ChangedWaterSystemFixtureCount" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Water System Fixture Count of water system {}", self.id.0), &format!("Anzahl Entnahmestellen von Wasseranlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
