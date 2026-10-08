//! 🧬️ Set Run Output in the imperative.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-run-output")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRunOutput {
    pub json: String,
}

impl protocol::MutationKind<ImperativeConfig, ImperativeConfigMutation> for SetRunOutput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "run_output_json", kind: "set-run-output", record: "SetRunOutput" };
    fn diff(&self, base: &ImperativeConfig) -> protocol::MutationOutcome<ImperativeConfigDiff> {
        if base.run_output_json == self.json {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The requested configuration value is already current.");
        }
        protocol::MutationOutcome::new(ImperativeConfigDiff { run_output_json: Some(self.json.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &ImperativeConfig) -> Result<Vec<ImperativeConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ImperativeConfigMutation::SetRunOutput(Self { json: base.run_output_json.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Run Output", "Laufausgabe setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["run_output_json".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = ImperativeConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&ImperativeConfigMutation::SetRunOutput(SetRunOutput { json: "{\"counter\":2}".into() }), &base).await;
    }
}
