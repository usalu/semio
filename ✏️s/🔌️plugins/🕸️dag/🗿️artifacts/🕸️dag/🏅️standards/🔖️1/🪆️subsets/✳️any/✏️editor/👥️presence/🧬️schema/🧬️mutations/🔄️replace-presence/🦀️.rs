//! 🔄️ Replace Presence in the DAG presence facet.

use super::{DagPresence, DagPresenceDiff, DagPresenceMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: DagPresence,
}

impl protocol::MutationKind<DagPresence, DagPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, base: &DagPresence) -> protocol::MutationOutcome<DagPresenceDiff> {
        protocol::MutationOutcome::new(DagPresenceDiff {
            camera_x: (base.camera_x != self.presence.camera_x).then_some(self.presence.camera_x),
            camera_y: (base.camera_y != self.presence.camera_y).then_some(self.presence.camera_y),
            camera_zoom: (base.camera_zoom != self.presence.camera_zoom).then_some(self.presence.camera_zoom),
        })
    }
    fn inverse(&self, base: &DagPresence) -> Result<Vec<DagPresenceMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![DagPresenceMutation::ReplacePresence(ReplacePresence { presence: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Presence", "Präsenz ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[test]
    fn inverse_diffs_sum_to_the_negative_diff() {
        let base = DagPresence { camera_x: 1.0, camera_y: 2.0, camera_zoom: 1.5 };
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&DagPresenceMutation::ReplacePresence(ReplacePresence { presence: DagPresence { camera_x: 9.0, camera_y: 2.0, camera_zoom: 0.25 } }), &base);
    }
}
