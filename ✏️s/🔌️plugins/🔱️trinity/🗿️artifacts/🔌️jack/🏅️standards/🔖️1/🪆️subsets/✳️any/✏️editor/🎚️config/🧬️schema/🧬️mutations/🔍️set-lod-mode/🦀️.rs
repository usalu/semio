//! 🔍️ Set Lod Mode in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-lod-mode")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLodMode {
    pub window_id: String,
    pub value: String,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetLodMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "lod-mode", kind: "set-lod-mode", record: "SetLodMode" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.lod_mode_by_window.insert(self.window_id.clone(), self.value.clone());
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Lod Mode".into() }
    fn target(&self) -> Vec<String> { vec!["lod_mode_by_window".into()] }
}
