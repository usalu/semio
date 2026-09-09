//! 🧬️ Set Run Output in the imperative.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-run-output")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRunOutput {
    pub json: String,
}

impl protocol::MutationKind<ImperativeConfig, ImperativeConfigMutation> for SetRunOutput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "run_output_json", kind: "set-run-output", record: "SetRunOutput" };
    fn diff(&self, base: &ImperativeConfig) -> protocol::MutationOutcome<ImperativeConfig> {
        if base.run_output_json == self.json {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "The requested configuration value is already current.");
        }
        let mut next = base.clone();
        next.run_output_json = self.json.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &ImperativeConfig) -> Vec<ImperativeConfigMutation> {
        vec![ImperativeConfigMutation::SetRunOutput(Self { json: base.run_output_json.clone() })]
    }
    fn label(&self) -> String {
        "Set Run Output".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["run_output_json".into()]
    }
}
