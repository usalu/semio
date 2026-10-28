//! 🎨️ Change the per-surface result field the 3d model window colours by.

use super::{EnergyModelConfig, EnergyModelConfigDiff, EnergyModelConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// 🎨️ `change-result-field`: replaces `resultField` alone and leaves the three run settings exactly as
/// the base had them; its inverse restores the base's field. Unlike `change-simulation-settings` this
/// touches no pointer the simulation run reads, so publishing it never restarts a live run.
#[derive(Clone, Debug, PartialEq, Eq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::NativeReceive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "change-result-field")]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeResultField {
    pub field: String,
}

impl ChangeResultField {
    fn of(config: &EnergyModelConfig) -> Self {
        Self { field: config.result_field.clone() }
    }
}

impl protocol::MutationKind<EnergyModelConfig, EnergyModelConfigMutation> for ChangeResultField {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "result-field", kind: "change-result-field", record: "ChangedResultField" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Colour surfaces by {}", self.field), &format!("Oberflächen nach {} einfärben", self.field))
    }
    fn target(&self) -> Vec<String> {
        vec!["result-field".into()]
    }
}

impl protocol::ColdMutationKind<EnergyModelConfig, EnergyModelConfigMutation> for ChangeResultField {
    fn diff(&self, base: &EnergyModelConfig) -> protocol::MutationOutcome<EnergyModelConfigDiff> {
        protocol::MutationOutcome::new(EnergyModelConfigDiff { result_field: (base.result_field != self.field).then(|| self.field.clone()), ..EnergyModelConfigDiff::default() })
    }
    fn inverse(&self, base: &EnergyModelConfig) -> Result<Vec<EnergyModelConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![EnergyModelConfigMutation::ChangeResultField(Self::of(base))]
    
    })())
}
}

#[cfg(test)]
#[path = "🧪️tests/⚖️law/🦀️.rs"]
mod law_tests;
