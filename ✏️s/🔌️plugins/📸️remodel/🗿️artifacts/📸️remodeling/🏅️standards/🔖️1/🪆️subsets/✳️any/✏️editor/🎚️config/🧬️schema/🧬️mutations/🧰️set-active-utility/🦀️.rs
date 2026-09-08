//! 🧰️ Set Active Utility in the remodeling config channel.

use super::{RemodelingConfig, RemodelingConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-active-utility")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetActiveUtility {
    pub utility_id: String,
}

impl protocol::MutationKind<RemodelingConfig, RemodelingConfigMutation> for SetActiveUtility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "active-utility", kind: "set-active-utility", record: "SetActiveUtility" };
    fn diff(&self, base: &RemodelingConfig) -> protocol::MutationOutcome<RemodelingConfig> {
        let mut next = base.clone();
        next.active_utility_id = self.utility_id.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &RemodelingConfig) -> Vec<RemodelingConfigMutation> { vec![RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Active Utility".into() }
    fn target(&self) -> Vec<String> { vec!["active-utility".into()] }
}
