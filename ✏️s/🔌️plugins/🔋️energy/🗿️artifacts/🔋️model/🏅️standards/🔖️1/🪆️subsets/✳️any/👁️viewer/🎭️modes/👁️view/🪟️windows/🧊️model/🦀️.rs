//! 🧊️ Energy model viewer — `model` window: the read-only twin of the editor's World3d viewport.
//! Same geometry, byte for byte, because both windows call the one shared builder `crate::scene`
//! (independently authored surface, no import through `✏️editor` —
//! `policyViewerPurityBreaches`).
//!
//! 🕹️ No interaction domain: this surface's manifest declares none, so the scene leaves `domain_id`
//! unset and the react host keeps the window on the OS's own world board. A viewer picks nothing and
//! tints nothing — there is no selection state for it to read.

use crate::scene::{energy_model_scene, EnergySceneStyle};
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as SemanticSurfaceKind;
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
/// 🪟️ The viewer manifest's window-kind id — the same spelling the editor's twin uses, since the two
/// manifests are separate namespaces and the window is the same thing.
pub const WINDOW_KIND_ID: &str = "energy.model.3d";
/// 📄️ This window's sole render body key.
pub const BODY_KEY: &str = "energy.model.3d";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::model::create_energy_model_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Model", "Modell"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "box".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: Some(crate::ENERGY_MODEL_DOCUMENT_SCHEMA.into()),
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Real `Model -> World3d surface`, with neither picking nor a results overlay — the read-only
/// projection of the same geometry the editor's twin renders.
pub fn render(model: &crate::model::Model) -> UiAssemblyResult<BuiltNode> {
    let scene = energy_model_scene(model, &EnergySceneStyle::default(), None);
    semio_framework_plugin::scene_surface(BODY_KEY, SemanticSurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
