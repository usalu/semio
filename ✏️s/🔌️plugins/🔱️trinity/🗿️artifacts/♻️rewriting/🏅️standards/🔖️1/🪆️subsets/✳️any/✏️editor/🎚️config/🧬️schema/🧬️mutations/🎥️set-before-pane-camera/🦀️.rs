//! 🎥️ Set Before Pane Camera in the Trinity rewriting configuration channel.

use super::{RewritingConfig, RewritingConfigMutation, ReplaceConfig, Camera};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-before-pane-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBeforePaneCamera {
    #[dsl(block)]
    pub camera: Camera,
}

impl protocol::MutationKind<RewritingConfig, RewritingConfigMutation> for SetBeforePaneCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "before-pane-camera", kind: "set-before-pane-camera", record: "SetBeforePaneCamera" };
    fn diff(&self, base: &RewritingConfig) -> protocol::MutationOutcome<RewritingConfig> {
        let mut next = base.clone();
        next.before_pane_camera = self.camera.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &RewritingConfig) -> Vec<RewritingConfigMutation> { vec![RewritingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Before Pane Camera".into() }
    fn target(&self) -> Vec<String> { vec!["before_pane_camera".into()] }
}
