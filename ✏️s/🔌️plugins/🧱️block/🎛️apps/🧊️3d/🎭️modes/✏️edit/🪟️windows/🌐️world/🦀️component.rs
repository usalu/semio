//! 🌐️ Block 3D play app — the world window: the object kind's 3D representation viewport (block3d's
//! only window kind).

use crate::apps::block3d::config::{block3d_window_view, Block3dConfig};
use crate::apps::block3d::modes::edit::windows::world::options::{arrangement, brush, quick_representation, representations, spacing};
use crate::apps::block3d::terminology::Block3dLabels;
use crate::apps::block3d::world::{world_camera_json, world_instances_json, world_interaction_json, world_meshes_json, world_selection_json, world_vortices_json, visible_representations};
use crate::apps::block3d::{BLOCK3D_PLAY_APP_ID, BLOCK3D_PLAY_SURFACE_ID};
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{build_world_3d_scene, world3d_scene_extended, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const BLOCK3D_WINDOW_WORLD: &str = "block3d-world";
pub const BLOCK3D_BODY_WORLD: &str = "block3d.play.world";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::block3d::create_block3d_app`. `options.measures`
/// stays empty here on purpose: block3d's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: BLOCK3D_WINDOW_WORLD.into(),
        label: LocalizedLabel::native("Object Kind", "Objektart"),
        body_key: BLOCK3D_BODY_WORLD.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "box".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from its `🎚️options/*` components.
pub fn window_measures(definition: &Block3dSnapshot, config: &Block3dConfig, window_id: &str, labels: &Block3dLabels) -> Vec<WindowMeasure> {
    vec![
        representations::measure(definition, config, window_id, labels),
        quick_representation::measure(definition, config, window_id, labels),
        arrangement::measure(config, window_id, labels),
        spacing::measure(config, window_id, labels),
        brush::measure(definition, config, labels),
    ]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(definition: &Block3dSnapshot, config: &Block3dConfig, window_id: &str) -> UiNode {
    let view = block3d_window_view(config, window_id);
    let visible = visible_representations(definition, &view);
    let scene = world3d_scene_extended(
        world_camera_json(definition, config),
        world_meshes_json(definition, &visible),
        world_instances_json(definition, &visible, &view),
        world_selection_json(config),
        Some(world_vortices_json(definition, config, &visible, &view)),
        None,
        None,
        None,
        None,
        Some(world_interaction_json(config, window_id)),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    build_world_3d_scene(BLOCK3D_PLAY_SURFACE_ID, BLOCK3D_PLAY_APP_ID, scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_the_world_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, BLOCK3D_BODY_WORLD);
        assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }
}
//#endregion 🧪️Tests
