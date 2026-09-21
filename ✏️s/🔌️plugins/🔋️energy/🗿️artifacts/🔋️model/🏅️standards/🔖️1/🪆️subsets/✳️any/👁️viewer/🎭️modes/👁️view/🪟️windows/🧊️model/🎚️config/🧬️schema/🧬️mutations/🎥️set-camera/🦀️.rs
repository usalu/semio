//! 🎥️ Sets the retained orbit pose of ONE addressed `energy.model.3d` window.

use super::{EnergyModelViewerCameraPose, EnergyModelViewerWindowConfig, EnergyModelViewerWindowConfigMutation};

#[derive(Clone, Copy, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: EnergyModelViewerCameraPose,
}

impl protocol::MutationKind<EnergyModelViewerWindowConfig, EnergyModelViewerWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &EnergyModelViewerWindowConfig) -> protocol::MutationOutcome<EnergyModelViewerWindowConfig> {
        if base.camera == self.camera {
            return protocol::MutationOutcome::new(*base).warn("mutation.no-op", "The 3d window already holds this camera.");
        }
        protocol::MutationOutcome::new(EnergyModelViewerWindowConfig { camera: self.camera })
    }
    fn inverse(&self, base: &EnergyModelViewerWindowConfig) -> Vec<EnergyModelViewerWindowConfigMutation> {
        (base.camera != self.camera).then(|| EnergyModelViewerWindowConfigMutation::SetCamera(SetCamera { camera: base.camera })).into_iter().collect()
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Set camera", "Kamera setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
