//! 🎥️ Sets the retained orbit pose of ONE addressed `energy.model.3d` window.

use super::{EnergyModelViewerCameraPose, EnergyModelViewerWindowConfig, EnergyModelViewerWindowConfigDiff, EnergyModelViewerWindowConfigMutation};

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone, semio_framework_value::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: EnergyModelViewerCameraPose,
}

impl protocol::MutationKind<EnergyModelViewerWindowConfig, EnergyModelViewerWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set camera", "Kamera setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}

impl protocol::ColdMutationKind<EnergyModelViewerWindowConfig, EnergyModelViewerWindowConfigMutation> for SetCamera {
    fn diff(&self, base: &EnergyModelViewerWindowConfig) -> protocol::MutationOutcome<EnergyModelViewerWindowConfigDiff> {
        if base.camera == self.camera {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The 3d window already holds this camera.");
        }
        protocol::MutationOutcome::new(EnergyModelViewerWindowConfigDiff { camera: Some(self.camera) })
    }
    fn inverse(&self, base: &EnergyModelViewerWindowConfig) -> Result<Vec<EnergyModelViewerWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        (base.camera != self.camera).then(|| EnergyModelViewerWindowConfigMutation::SetCamera(SetCamera { camera: base.camera })).into_iter().collect()
    
    })())
}
}

#[cfg(test)]
#[path = "🧪️tests/⚖️law/🦀️.rs"]
mod law_tests;
