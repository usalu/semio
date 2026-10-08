//! 🧬️ Sets lod mode on the addressed Jack graph window.

use super::{JackGraphWindowConfig, JackGraphWindowConfigDiff, JackGraphWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-lod-mode")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLodMode {
    pub value: String,
}

impl protocol::MutationKind<JackGraphWindowConfig, JackGraphWindowConfigMutation> for SetLodMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "window-lod-mode", kind: "set-lod-mode", record: "SetLodMode" };

    fn diff(&self, base: &JackGraphWindowConfig) -> protocol::MutationOutcome<JackGraphWindowConfigDiff> {
        if self.value == base.lod_mode {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window lod mode is unchanged.");
        }
        protocol::MutationOutcome::new(JackGraphWindowConfigDiff { lod_mode: Some(self.value.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &JackGraphWindowConfig) -> Result<Vec<JackGraphWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { value: base.lod_mode.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Window Lod Mode", "Detailstufenmodus des Fensters setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["lod_mode".into()]
    }
}
