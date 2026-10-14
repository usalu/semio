//! 💰️ Energy model mutation — `ChangeOutdoorAirSystemEconomizerEnabled`: Sets whether the economizer may open beyond the minimum outdoor air flow when free cooling is available.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💰️ `change-outdoor-air-system-economizer-enabled` payload. Sets whether the economizer may open beyond the minimum outdoor air flow when free cooling is available.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-outdoor-air-system-economizer-enabled")]
pub struct ChangeOutdoorAirSystemEconomizerEnabled {
    pub id: crate::model::EntityId,
    pub new_economizer_enabled: bool,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_outdoor_air_system_economizer_enabled<Op: From<ChangeOutdoorAirSystemEconomizerEnabled>>(id: crate::model::EntityId, new_economizer_enabled: bool) -> Op {
    ChangeOutdoorAirSystemEconomizerEnabled { id, new_economizer_enabled }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeOutdoorAirSystemEconomizerEnabled
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeOutdoorAirSystemEconomizerEnabled>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "outdoor-air-system", kind: "change-outdoor-air-system-economizer-enabled", record: "ChangedOutdoorAirSystemEconomizerEnabled" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change outdoor air system {} economizer setting to {:?}", self.id.0, self.new_economizer_enabled), &format!("Economizer-Einstellung von Außenluftsystem {} auf {:?} ändern", self.id.0, self.new_economizer_enabled))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
