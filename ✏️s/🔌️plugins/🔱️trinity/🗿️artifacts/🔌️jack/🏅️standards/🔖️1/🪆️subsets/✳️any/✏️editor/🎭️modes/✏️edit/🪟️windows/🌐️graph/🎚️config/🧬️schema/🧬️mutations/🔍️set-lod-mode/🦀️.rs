//! 🧬️ Sets lod mode on the addressed Jack graph window.

use super::{JackGraphWindowConfig, JackGraphWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-lod-mode")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLodMode {
    pub value: String,
}

impl protocol::MutationKind<JackGraphWindowConfig, JackGraphWindowConfigMutation> for SetLodMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "window-lod-mode", kind: "set-lod-mode", record: "SetLodMode" };

    fn diff(&self, base: &JackGraphWindowConfig) -> protocol::MutationOutcome<JackGraphWindowConfig> {
        let mut next = base.clone();
        next.lod_mode.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &JackGraphWindowConfig) -> Vec<JackGraphWindowConfigMutation> {
        vec![Self { value: base.lod_mode.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Window Lod Mode".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["lod_mode".into()]
    }
}
