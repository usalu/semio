//! 👁️ Sourcing curate app — the preview window: a 3D preview of the currently-selected object.

use crate::apps::curate::config::SourcingCurateConfig;
use crate::apps::curate::terminology::SourcingLabels;
use crate::apps::curate::SOURCING_CONTROLLER_ID;
use crate::artifacts::curate::schema::{instance_json, kind_mesh_json};
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{build_world_3d_scene, ui_text, world3d_default_camera, world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions, WorldSunConfig};
use serde_json::json;

//#region 🔖️Constants
pub const SOURCING_CURATE_WINDOW_PREVIEW: &str = "sourcing-preview";
pub const SOURCING_CURATE_BODY_PREVIEW: &str = "sourcing.preview";
const SOURCING_CURATE_SURFACE_PREVIEW: &str = "sourcing.preview.world";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATE_WINDOW_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: SOURCING_CURATE_BODY_PREVIEW.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "preview".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &CurateSnapshot, cfg: &SourcingCurateConfig, labels: &SourcingLabels) -> UiNode {
    let stock = crate::artifacts::curate::stock_of(document);
    let Some(kind) = cfg.selected_object_id.as_ref().and_then(|id| stock.iter().find(|kind| &kind.id == id)) else {
        return ui_text(labels.no_selection);
    };
    let meshes_json = json!([kind_mesh_json(kind)]).to_string();
    let instances_json = json!([instance_json(kind, [0.0, 0.0, 0.0], 1.0, false)]).to_string();
    let mut scene = world3d_scene(world3d_default_camera(), meshes_json, instances_json, world3d_selection_json("rectangle", &[], None), &WorldSunConfig::default());
    scene.fit_json = Some(json!({ "enabled": true, "padding": 0.2 }).to_string());
    build_world_3d_scene(SOURCING_CURATE_SURFACE_PREVIEW, SOURCING_CONTROLLER_ID, scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::curate::testkit::{new_app, render as render_body};

    #[test]
    fn preview_renders_selected_mesh_id() {
        let document = crate::artifacts::curate::schema::default_document();
        let object_id = crate::artifacts::curate::stock_of(&document)[0].id.clone();
        let cfg = SourcingCurateConfig { selected_object_id: Some(object_id.clone()), ..Default::default() };
        let node = render(&document, &cfg, crate::apps::curate::terminology::sourcing_curate_labels(&SourcingCurateConfig::default()));
        let json = serde_json::to_value(&node).unwrap();
        let meshes_json = json.pointer("/world3d/meshesJson").and_then(|value| value.as_str()).unwrap();
        let meshes: Vec<serde_json::Value> = serde_json::from_str(meshes_json).unwrap();
        assert_eq!(meshes.len(), 1);
        assert_eq!(meshes[0]["id"].as_str(), Some(object_id.as_str()));
    }

    #[test]
    fn preview_shows_placeholder_without_selection() {
        let document = crate::artifacts::curate::schema::default_document();
        let cfg = SourcingCurateConfig::default();
        let node = render(&document, &cfg, crate::apps::curate::terminology::sourcing_curate_labels(&SourcingCurateConfig::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("No selection"));
    }

    #[test]
    fn definition_declares_the_world3d_surface_and_body_key() {
        let def = definition();
        assert_eq!(def.body_key, SOURCING_CURATE_BODY_PREVIEW);
        assert!(matches!(def.surface_kind, SurfaceKind::World3d));
    }

    #[test]
    fn renders_via_the_app() {
        let mut app = new_app();
        // Default config has no selection, so the app-level render shows the placeholder.
        assert!(render_body(&mut app, SOURCING_CURATE_BODY_PREVIEW).contains("No selection"));
    }
}
//#endregion 🧪️Tests
