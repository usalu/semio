//! 🧬️ Sets lod mode on the addressed Rewriting window.

use super::{RewritingWindowConfig, RewritingWindowConfigDiff, RewritingWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-lod-mode")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLodMode {
    pub value: String,
}

impl protocol::MutationKind<RewritingWindowConfig, RewritingWindowConfigMutation> for SetLodMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "window-lod-mode", kind: "set-lod-mode", record: "SetLodMode" };

    fn diff(&self, base: &RewritingWindowConfig) -> protocol::MutationOutcome<RewritingWindowConfigDiff> {
        if self.value == base.lod_mode {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window lod mode is unchanged.");
        }
        protocol::MutationOutcome::new(RewritingWindowConfigDiff { lod_mode: Some(self.value.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &RewritingWindowConfig) -> Result<Vec<RewritingWindowConfigMutation>, semio_framework_value::ValueError> {
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
