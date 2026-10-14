//! 🏹️ Energy model mutation — `ChangeShwSystemSetpoint`: Sets tank setpoint (°C) on one service hot water system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏹️ `change-shw-system-setpoint` payload. Sets tank setpoint (°C) on one service hot water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shw-system-setpoint")]
pub struct ChangeShwSystemSetpoint {
    pub id: crate::model::EntityId,
    pub new_setpoint_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_shw_system_setpoint<Op: From<ChangeShwSystemSetpoint>>(id: crate::model::EntityId, new_setpoint_c: f64) -> Op {
    ChangeShwSystemSetpoint { id, new_setpoint_c }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeShwSystemSetpoint
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeShwSystemSetpoint>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "service-hot-water-system", kind: "change-shw-system-setpoint", record: "ChangedShwSystemSetpoint" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Shw System Setpoint of service hot water system {}", self.id.0), &format!("Sollwert von Trinkwarmwasseranlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
