//! 🧬️ Sets camera on the addressed Rewriting window.

use super::{RewritingWindowConfig, RewritingWindowConfigDiff, RewritingWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: Option<semio_s_artifact_trinity_jack::Camera>,
}

impl protocol::MutationKind<RewritingWindowConfig, RewritingWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "window-camera", kind: "set-camera", record: "SetCamera" };

    fn diff(&self, base: &RewritingWindowConfig) -> protocol::MutationOutcome<RewritingWindowConfigDiff> {
        if self.camera == base.camera {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window camera is unchanged.");
        }
        protocol::MutationOutcome::new(RewritingWindowConfigDiff { camera: Some(self.camera.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &RewritingWindowConfig) -> Result<Vec<RewritingWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { camera: base.camera.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Window Camera", "Fensterkamera setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
