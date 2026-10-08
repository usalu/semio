//! 🧬️ Replace Config in the imperative.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: ImperativeConfig,
}

impl protocol::MutationKind<ImperativeConfig, ImperativeConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &ImperativeConfig) -> protocol::MutationOutcome<ImperativeConfigDiff> {
        if *base == self.config {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The requested configuration value is already current.");
        }
        protocol::MutationOutcome::new(ImperativeConfigDiff {
            run_output_json: (base.run_output_json != self.config.run_output_json).then(|| self.config.run_output_json.clone()),
            contributions_json: (base.contributions_json != self.config.contributions_json).then(|| self.config.contributions_json.clone()),
        })
    }
    fn inverse(&self, base: &ImperativeConfig) -> Result<Vec<ImperativeConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ImperativeConfigMutation::ReplaceConfig(Self { config: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[test]
    fn inverse_diffs_sum_to_the_negative_diff() {
        let base = ImperativeConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&ImperativeConfigMutation::ReplaceConfig(ReplaceConfig { config: ImperativeConfig { run_output_json: "{}".into(), contributions_json: "[2]".into() } }), &base);
    }
}
