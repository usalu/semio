//! 🎨️ Change the per-surface result field the 3d model window colours by.

use super::{EnergyModelConfig, EnergyModelConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// 🎨️ `change-result-field`: replaces `resultField` alone and leaves the three run settings exactly as
/// the base had them; its inverse restores the base's field. Unlike `change-simulation-settings` this
/// touches no pointer the simulation run reads, so publishing it never restarts a live run.
#[derive(Clone, Debug, PartialEq, Eq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
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

    /// 🎚️ The config this change installs over `base`.
    pub fn config(&self, base: &EnergyModelConfig) -> EnergyModelConfig {
        EnergyModelConfig { result_field: self.field.clone(), ..base.clone() }
    }
}

impl protocol::MutationKind<EnergyModelConfig, EnergyModelConfigMutation> for ChangeResultField {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "result-field", kind: "change-result-field", record: "ChangedResultField" };
    fn diff(&self, base: &EnergyModelConfig) -> protocol::MutationOutcome<EnergyModelConfig> {
        protocol::MutationOutcome::new(self.config(base))
    }
    fn inverse(&self, base: &EnergyModelConfig) -> Vec<EnergyModelConfigMutation> {
        vec![EnergyModelConfigMutation::ChangeResultField(Self::of(base))]
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Colour surfaces by {}", self.field), &format!("Oberflächen nach {} einfärben", self.field))
    }
    fn target(&self) -> Vec<String> {
        vec!["result-field".into()]
    }
}
