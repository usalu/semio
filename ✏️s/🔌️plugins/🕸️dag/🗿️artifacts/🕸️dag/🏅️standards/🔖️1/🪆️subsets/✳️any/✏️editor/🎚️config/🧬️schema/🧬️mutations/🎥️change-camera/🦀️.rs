//! 🎥️ Change Camera in the DAG config facet.

use super::{DagConfig, DagConfigDiff, DagConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "change-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl protocol::MutationKind<DagConfig, DagConfigMutation> for ChangeCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "camera", kind: "change-camera", record: "ChangeCamera" };
    fn diff(&self, base: &DagConfig) -> protocol::MutationOutcome<DagConfigDiff> {
        protocol::MutationOutcome::new(DagConfigDiff {
            camera_x: (base.camera_x != self.x).then_some(self.x),
            camera_y: (base.camera_y != self.y).then_some(self.y),
            camera_zoom: (base.camera_zoom != self.zoom).then_some(self.zoom),
        })
    }
    fn inverse(&self, base: &DagConfig) -> Result<Vec<DagConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![DagConfigMutation::ChangeCamera(ChangeCamera { x: base.camera_x, y: base.camera_y, zoom: base.camera_zoom })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Camera", "Kamera ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = DagConfig { camera_x: 1.0, camera_y: 2.0, camera_zoom: 1.5 };
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&DagConfigMutation::ChangeCamera(ChangeCamera { x: -3.0, y: 2.0, zoom: 2.0 }), &base).await;
    }
}
