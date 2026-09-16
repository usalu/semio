//! 🧊️ Energy model editor — `model` window: the World3d viewport that renders the whole building as
//! real geometry. Every opaque `crate::model::Surface`, every `Fenestration` and every
//! `ShadingSurface` is one mesh + one pickable instance, coloured by family (or by a results overlay
//! when one is supplied) and tinted by the shared interaction domain's selection/hover.
//!
//! 🕹️ Picking is entirely framework-owned: the scene carries
//! `domain_id = ENERGY_MODEL_INTERACTION_DOMAIN`, so the react `World3dHost` raycasts against its own
//! Three.js geometry and dispatches the framework-RESERVED `interactionSelect`/`interactionHover`
//! itself — this window declares no `canvas-pointer-down` command and owns no selection state. The
//! instance ids are the RAW `EntityId`s (`crate::editor::model::interaction::energy_target_id`), the
//! same vocabulary the artifact tree panel and the inspector address, which is the whole mechanism
//! behind "tree pick ⇄ 3d pick".
//!
//! 🎬️ The geometry itself is built by `crate::scene`, shared verbatim with the read-only viewer's
//! twin of this window (a viewer may not import through `✏️editor`).

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use crate::editor::model::interaction::{EnergyModelInteractionSnapshot, ENERGY_GRANULARITY_SURFACE, ENERGY_MODEL_INTERACTION_DOMAIN};
use crate::scene::{energy_model_scene, EnergySceneStyle};
use semio_framework_plugin::plugin_app_close_prelude::{column, text, Buildable, HasBase, HasChildren, HasStackLayout, Label as SemanticLabel, SurfaceKind as SemanticSurfaceKind};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, BuiltNode, InteractiveJobClassification, LocalizedLabel, PluginAssemblyError, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use std::collections::HashMap;

//#region 🔖️Constants
/// 🪟️ The manifest's window-kind id. Fixed: the tree panel, the inspector and the results overlay
/// all address this window by it.
pub const WINDOW_KIND_ID: &str = "energy.model.3d";
/// 📄️ This window's sole render body key.
pub const BODY_KEY: &str = "energy.model.3d";
/// 🎥️ The camera verb the react `World3dHost` dispatches on its own after every orbit/pan/zoom
/// (`worldCameraSetCameraDispatchArgs`, `🌐️World3dHost/🟦️.tsx:767`, debounced). It is NOT
/// framework-reserved: a window kind that does not declare it makes the host's dispatch land as
/// `dropped action "setCamera" ... no window kind declares it` on every gesture.
pub const SET_CAMERA_ACTION_ID: &str = "setCamera";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🎥️ The one verb this window owns. `ActionKind::View`: a camera pose is never document data, so it
/// publishes no model mutation and never enters undo/redo.
///
/// 📐️ It declares its `camera` argument so the host's `{windowId, camera:{position,target,zoom,up?}}`
/// survives `effective_action_args` — with a non-empty arg list that function keeps ONLY declared ids,
/// so an undeclared `camera` would be filtered away before the command bridge ever sees it.
pub fn set_camera_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(SET_CAMERA_ACTION_ID, LocalizedLabel::native("Set camera", "Kamera setzen"), ActionKind::View)
        .with_args(vec![ActionArgDef::text("camera", LocalizedLabel::native("Camera pose", "Kamerapose")).required()]);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action
}

/// 🧱️ Stitched into the editor manifest by `crate::editor::model::create_energy_model_editor`, which
/// also binds the interaction domain through `.window_kind_interactions(...)` — the domain itself is
/// declared exactly once, next to the other manifest calls.
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
        // 🕹️ Left empty on purpose: the manifest binds this window's domain with
        // `.window_kind_interactions(...)`, the same way process3d/cad do, so the binding is stated
        // once where the domain is declared.
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
/// 🔑️ Node ids of the two children a captioned render stacks. They MUST differ: an unkeyed
/// `built_text_node` and an unkeyed stack both default to `#0`, and two `#0` siblings publish as
/// `DuplicateSiblingKey parent=#0 key=#0`, which scopes the whole `energy.model.3d` surface to a
/// render fault — the window then goes blank for as long as a caption is supplied (observed in the
/// browser the moment a run started colouring, `🗑️generated/energy-results-w1/console.txt`).
pub const CAPTION_NODE_ID: &str = "energy.model.3d.caption";
pub const CAPTION_SCENE_NODE_ID: &str = "energy.model.3d.scene";

fn caption_error(reason: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("energy.model.3d.caption", reason)
}

/// 🏷️ Places a data caption (the results legend) above the world scene — fem3d's
/// `📊️results/🦀️.rs::with_caption` shape, plus the explicit sibling ids that shape is missing.
fn with_caption(scene: BuiltNode, caption: &str) -> UiAssemblyResult<BuiltNode> {
    let caption = SemanticLabel::try_from(caption.to_string()).map_err(|_| caption_error("the scene caption exceeds its UI label bound"))?;
    let label = text(caption)
        .try_id(CAPTION_NODE_ID)
        .map_err(|_| caption_error("the caption node id was refused"))?
        .try_build()
        .map_err(|_| caption_error("the caption node was refused"))?;
    let scene_stack = column()
        .grow(true)
        .try_id(CAPTION_SCENE_NODE_ID)
        .map_err(|_| caption_error("the captioned scene stack id was refused"))?
        .try_child(scene)
        .map_err(|_| caption_error("the captioned scene child was refused"))?
        .try_build()
        .map_err(|_| caption_error("the captioned scene stack was refused"))?;
    column()
        .grow(true)
        .try_children([label, scene_stack])
        .map_err(|_| caption_error("the caption children were refused"))?
        .try_build()
        .map_err(|_| caption_error("the caption node was refused"))
}

/// ✏️ Real `Model -> World3d surface`. `overlay` maps a raw entity id to an rgb triple that REPLACES
/// that entity's family swatch (the results colouring lane); `caption` is the legend line drawn above
/// the viewport. Both `None` is the plain model view the edit-mode layout renders.
pub fn render(model: &crate::model::Model, interaction: &EnergyModelInteractionSnapshot, overlay: Option<&HashMap<u32, [f64; 3]>>, caption: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    render_with_camera(model, interaction, overlay, caption, None)
}

/// 🎥️ The same render, plus the addressed window's RETAINED orbit pose when it has one
/// (`config::current`). `None` keeps the model-derived camera and lets `fit_json` frame the model
/// once; `Some` republishes exactly the pose the host last sent, which
/// `shouldReattachWorldViewportCamera` recognizes as its own echo and therefore does NOT reattach —
/// so a re-render never yanks a camera the user just moved.
pub fn render_with_camera(
    model: &crate::model::Model,
    interaction: &EnergyModelInteractionSnapshot,
    overlay: Option<&HashMap<u32, [f64; 3]>>,
    caption: Option<&str>,
    camera: Option<&config::EnergyModelWindowConfig>,
) -> UiAssemblyResult<BuiltNode> {
    let style = EnergySceneStyle { selected_ids: &interaction.selected_ids, hovered_ids: &interaction.hovered_ids, overlay };
    let mut scene = energy_model_scene(model, &style, Some((ENERGY_MODEL_INTERACTION_DOMAIN, ENERGY_GRANULARITY_SURFACE)));
    if let Some(window) = camera {
        scene.camera_json = window.camera.scene_camera_json();
    }
    let node = semio_framework_plugin::scene_surface(BODY_KEY, SemanticSurfaceKind::World3d, &scene)?;
    match caption {
        Some(caption) => with_caption(node, caption),
        None => Ok(node),
    }
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
