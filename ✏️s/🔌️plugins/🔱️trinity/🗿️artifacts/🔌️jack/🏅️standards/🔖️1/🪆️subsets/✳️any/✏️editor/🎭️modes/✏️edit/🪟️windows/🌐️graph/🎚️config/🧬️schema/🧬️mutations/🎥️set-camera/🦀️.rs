//! 🧬️ Sets camera on the addressed Jack graph window.

use super::{JackGraphWindowConfig, JackGraphWindowConfigDiff, JackGraphWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: Option<crate::Camera>,
}

impl protocol::MutationKind<JackGraphWindowConfig, JackGraphWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "window-camera", kind: "set-camera", record: "SetCamera" };

    fn diff(&self, base: &JackGraphWindowConfig) -> protocol::MutationOutcome<JackGraphWindowConfigDiff> {
        if self.camera == base.camera {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window camera is unchanged.");
        }
        protocol::MutationOutcome::new(JackGraphWindowConfigDiff { camera: Some(self.camera.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &JackGraphWindowConfig) -> Result<Vec<JackGraphWindowConfigMutation>, semio_framework_value::ValueError> {
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
