//! 🧊️ Energy model viewer — `model` window: the read-only twin of the editor's World3d viewport.
//! Same geometry, byte for byte, because both windows call the one shared builder `crate::scene`
//! (independently authored surface, no import through `✏️editor` —
//! `policyViewerPurityBreaches`).
//!
//! 🕹️ No interaction domain: this surface's manifest declares none, so the scene leaves `domain_id`
//! unset and the react host keeps the window on the OS's own world board. A viewer picks nothing and
//! tints nothing — there is no selection state for it to read.
//!
//! 🎥️ It DOES orbit, though, and the react `World3dHost` dispatches `setCamera` on its own after
//! every gesture whether the window is read-only or not. A window kind that does not declare that
//! verb drops every orbit, so this one declares it and retains the pose in its own window config —
//! the config lane, never the artifact lane, which is what keeps the surface structurally read-only.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use crate::scene::{energy_model_scene, EnergySceneStyle};
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as SemanticSurfaceKind;
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, BuiltNode, InteractiveJobClassification, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
/// 🪟️ The viewer manifest's window-kind id — the same spelling the editor's twin uses, since the two
/// manifests are separate namespaces and the window is the same thing.
pub const WINDOW_KIND_ID: &str = "energy.model.3d";
/// 📄️ This window's sole render body key.
pub const BODY_KEY: &str = "energy.model.3d";
/// 🎥️ The camera verb the react `World3dHost` dispatches after every orbit/pan/zoom
/// (`worldCameraSetCameraDispatchArgs`, debounced). It is NOT framework-reserved: an undeclared
/// `setCamera` lands as `dropped action "setCamera" … no window kind declares it` on every gesture.
pub const SET_CAMERA_ACTION_ID: &str = "setCamera";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🎥️ The one verb a read-only 3d window owns. `ActionKind::View` and a config-lane publication: a
/// camera pose is never document data, so it publishes no model mutation and enters no undo history —
/// which is exactly why a VIEWER may own it.
///
/// 📐️ It declares its `camera` argument so the host's `{windowId, camera:{position,target,zoom,up?}}`
/// survives `effective_action_args`, which keeps ONLY declared arg ids once an action declares any.
pub fn set_camera_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(SET_CAMERA_ACTION_ID, LocalizedLabel::native("Set camera", "Kamera setzen"), ActionKind::View)
        .with_args(vec![ActionArgDef::text("camera", LocalizedLabel::native("Camera pose", "Kamerapose")).required()]);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action
}

/// 🧱️ Stitched into the viewer manifest by `crate::viewer::model::create_energy_model_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Model", "Modell"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "box".into(),
        options: WindowOptions::default(),
        actions: vec![set_camera_action()],
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
    render_with_camera(model, None)
}

/// 🎥️ The same render, plus the addressed window's RETAINED orbit pose when it has one
/// (`config::current`). `None` keeps the model-derived camera and lets `fit_json` frame the model
/// once; `Some` republishes exactly the pose the host last sent, which
/// `shouldReattachWorldViewportCamera` recognizes as its own echo and therefore does NOT reattach —
/// so a re-render never yanks a camera the user just moved.
pub fn render_with_camera(model: &crate::model::Model, camera: Option<&config::EnergyModelViewerWindowConfig>) -> UiAssemblyResult<BuiltNode> {
    let mut scene = energy_model_scene(model, &EnergySceneStyle::default(), None);
    if let Some(window) = camera {
        scene.camera_json = window.camera.scene_camera_json();
    }
    semio_framework_plugin::scene_surface(BODY_KEY, SemanticSurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
