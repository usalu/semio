//! 🧬️ Sets lod mode on the addressed Rewriting window.

use super::{RewritingWindowConfig, RewritingWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-lod-mode")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLodMode {
    pub value: String,
}

impl protocol::MutationKind<RewritingWindowConfig, RewritingWindowConfigMutation> for SetLodMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "window-lod-mode", kind: "set-lod-mode", record: "SetLodMode" };

    fn diff(&self, base: &RewritingWindowConfig) -> protocol::MutationOutcome<RewritingWindowConfig> {
        let mut next = base.clone();
        next.lod_mode.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &RewritingWindowConfig) -> Vec<RewritingWindowConfigMutation> {
        vec![Self { value: base.lod_mode.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Window Lod Mode".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["lod_mode".into()]
    }
}
