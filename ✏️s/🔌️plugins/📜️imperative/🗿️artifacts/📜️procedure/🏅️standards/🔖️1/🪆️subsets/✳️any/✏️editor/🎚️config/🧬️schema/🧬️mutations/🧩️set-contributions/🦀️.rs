//! 🧬️ Set Contributions in the imperative.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-contributions")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetContributions {
    pub json: String,
}

impl protocol::MutationKind<ImperativeConfig, ImperativeConfigMutation> for SetContributions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "contributions_json", kind: "set-contributions", record: "SetContributions" };
    fn diff(&self, base: &ImperativeConfig) -> protocol::MutationOutcome<ImperativeConfig> {
        imperative_engine::sync_imperative_module_contributions(&self.json);
        if base.contributions_json == self.json {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "The requested configuration value is already current.");
        }
        let mut next = base.clone();
        next.contributions_json = self.json.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &ImperativeConfig) -> Vec<ImperativeConfigMutation> { vec![ImperativeConfigMutation::SetContributions(Self { json: base.contributions_json.clone() })] }
    fn label(&self) -> String { "Set Contributions".into() }
    fn target(&self) -> Vec<String> { vec!["contributions_json".into()] }
}
