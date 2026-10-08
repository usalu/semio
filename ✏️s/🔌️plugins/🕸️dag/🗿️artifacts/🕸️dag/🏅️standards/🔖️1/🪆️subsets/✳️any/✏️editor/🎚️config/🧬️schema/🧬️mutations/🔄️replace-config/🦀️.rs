//! 🔄️ Replace Config in the DAG config facet.

use super::{DagConfig, DagConfigDiff, DagConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: DagConfig,
}

impl protocol::MutationKind<DagConfig, DagConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &DagConfig) -> protocol::MutationOutcome<DagConfigDiff> {
        protocol::MutationOutcome::new(DagConfigDiff {
            camera_x: (base.camera_x != self.config.camera_x).then_some(self.config.camera_x),
            camera_y: (base.camera_y != self.config.camera_y).then_some(self.config.camera_y),
            camera_zoom: (base.camera_zoom != self.config.camera_zoom).then_some(self.config.camera_zoom),
        })
    }
    fn inverse(&self, base: &DagConfig) -> Result<Vec<DagConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![DagConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[test]
    fn inverse_diffs_sum_to_the_negative_diff() {
        let base = DagConfig { camera_x: 1.0, camera_y: 2.0, camera_zoom: 1.5 };
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&DagConfigMutation::ReplaceConfig(ReplaceConfig { config: DagConfig { camera_x: 4.0, camera_y: 2.0, camera_zoom: 0.5 } }), &base);
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&DagConfigMutation::ReplaceConfig(ReplaceConfig { config: DagConfig { camera_x: 1.0, camera_y: 2.0, camera_zoom: 1.5 } }), &base);
    }
}
