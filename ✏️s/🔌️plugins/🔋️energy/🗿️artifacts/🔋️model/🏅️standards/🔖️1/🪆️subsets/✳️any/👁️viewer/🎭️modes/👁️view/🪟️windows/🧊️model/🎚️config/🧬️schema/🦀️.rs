//! 🧬️ Energy model 3d window-config schema — the orbit pose one concrete `energy.model.3d` window
//! retains. Source of record: `🔣️.json`.

/// 🎥️ The persisted orbit pose, in the same z-up model metres the scene itself uses. Flat `[f64; 3]`
/// fields rather than a nested record so the whole config is one `lines` DSL document, and the
/// SAME three keys the react `World3dHost` sends under `camera` (`position`/`target`/`zoom`), so the
/// wire pose needs no renaming on the way in or out.
#[derive(Clone, Copy, Debug, PartialEq, dsl::DslRecord, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct EnergyModelViewerCameraPose {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub zoom: f64,
}

impl Default for EnergyModelViewerCameraPose {
    /// 🎥️ The framework's own default world camera, three-quarter from the south-west and above. A
    /// fresh window never actually shows it: `render` only publishes a STORED pose, and an unstored
    /// window gets the model-derived camera plus `fit_json`'s one-shot framing instead.
    fn default() -> Self {
        Self { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], zoom: 1.0 }
    }
}

impl EnergyModelViewerCameraPose {
    /// ✅️ A pose is admissible when every coordinate is finite and the zoom is positive — the same
    /// question `Viewport3dOrbit::validate` asks, restated here because this record is the one that
    /// gets persisted.
    pub fn is_valid(&self) -> bool {
        self.position.iter().chain(self.target.iter()).all(|value| value.is_finite()) && self.zoom.is_finite() && self.zoom > 0.0
    }

    /// 🎬️ The exact `World3dScene::camera_json` string this pose stands for. `up` is always world Z:
    /// energy geometry is z-up by convention (`crate::scene`'s module docstring).
    pub fn scene_camera_json(&self) -> String {
        semio_framework_plugin::world3d_camera_json(self.position, self.target, 45.0)
    }
}

/// 🎚️ Persisted local state for ONE concrete `energy.model.3d` window: its camera, and nothing else.
/// Per WINDOW, not per app — two open 3d windows orbit independently, which is exactly why this is a
/// `WindowConfigOwner` state and not a field on `EnergyModelConfig`.
#[derive(Clone, Copy, Debug, Default, PartialEq, dsl::DslArtifact, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(id = "energy.model3dviewerwindowconfig", layout = "lines")]
pub struct EnergyModelViewerWindowConfig {
    #[dsl(block)]
    pub camera: EnergyModelViewerCameraPose,
}
