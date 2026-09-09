//! 🧬️ Sets camera on the addressed Rewriting window.

use super::{RewritingWindowConfig, RewritingWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: Option<semio_s_artifact_trinity_jack::Camera>,
}

impl protocol::MutationKind<RewritingWindowConfig, RewritingWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "window-camera", kind: "set-camera", record: "SetCamera" };

    fn diff(&self, base: &RewritingWindowConfig) -> protocol::MutationOutcome<RewritingWindowConfig> {
        let mut next = base.clone();
        next.camera.clone_from(&self.camera);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &RewritingWindowConfig) -> Vec<RewritingWindowConfigMutation> {
        vec![Self { camera: base.camera.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Window Camera".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
