//! 🎥️ Sets the retained orbit pose of ONE addressed `energy.model.3d` window.

use super::{EnergyModelCameraPose, EnergyModelWindowConfig, EnergyModelWindowConfigDiff, EnergyModelWindowConfigMutation};

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: EnergyModelCameraPose,
}

impl protocol::MutationKind<EnergyModelWindowConfig, EnergyModelWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &EnergyModelWindowConfig) -> protocol::MutationOutcome<EnergyModelWindowConfigDiff> {
        if base.camera == self.camera {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The 3d window already holds this camera.");
        }
        protocol::MutationOutcome::new(EnergyModelWindowConfigDiff { camera: Some(self.camera) })
    }
    fn inverse(&self, base: &EnergyModelWindowConfig) -> Result<Vec<EnergyModelWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        (base.camera != self.camera).then(|| EnergyModelWindowConfigMutation::SetCamera(SetCamera { camera: base.camera })).into_iter().collect()
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set camera", "Kamera setzen")
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
        let base = EnergyModelWindowConfig::default();
        let moved = EnergyModelCameraPose { position: [1.0, 2.0, 3.0], target: [0.5, 0.5, 0.0], zoom: 2.0 };
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelWindowConfigMutation::SetCamera(SetCamera { camera: moved }), &base).await;
    }
}
