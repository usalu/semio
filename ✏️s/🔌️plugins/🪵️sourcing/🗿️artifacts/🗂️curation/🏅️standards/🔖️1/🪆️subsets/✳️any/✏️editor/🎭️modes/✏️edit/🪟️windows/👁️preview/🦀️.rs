//! 👁️ Sourcing curation app — the preview window: a 3D preview of the currently-selected object.

use crate::artifacts::curation::schema::{instance_json, kind_mesh_json};
use crate::artifacts::curation::CurationSnapshot;
use crate::editor::sourcing::terminology::SourcingLabels;
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::{world3d_default_camera, world3d_selection_json, BuiltNode, Label, LocalizedLabel, MeshView, MeshWindowKit, PluginAssemblyError, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const SOURCING_CURATION_WINDOW_PREVIEW: &str = "sourcing-preview";
pub const SOURCING_CURATION_BODY_PREVIEW: &str = "sourcing.preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATION_WINDOW_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: SOURCING_CURATION_BODY_PREVIEW.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "preview".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        interactions: Vec::new(),
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
/// 👁️ `selected_ids` is the "rows" interaction domain's current selection — `ArtifactApp::render`
/// carries no `InteractionView` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM's
/// w3b-summary.md: the breaking pass only threaded it into `handle`/`copy_fragment`/`cut_operations`),
/// so the app-level call site always passes an empty slice and this window degrades to its "no
/// selection" placeholder until a future wave threads interaction into render. Flagged as a discovered
/// framework gap, not worked around here — kept as a parameter (rather than deleted outright) so that
/// future wave has a slot to fill in.
pub fn render(document: &CurationSnapshot, selected_ids: &[String], labels: &SourcingLabels) -> UiAssemblyResult<BuiltNode> {
    let stock = crate::artifacts::curation::stock_of(document);
    let Some(kind) = selected_ids.first().and_then(|id| stock.iter().find(|kind| &kind.id == id)) else {
        return semio_framework_plugin::built_text_node(Label::data(labels.no_selection.as_str())).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sourcing preview placeholder admission failed"));
    };
    MeshWindowKit::render(&MeshView {
        camera_json: world3d_default_camera(),
        meshes_json: dsl::json::to_json_string(&dsl::DslValue::Array(vec![kind_mesh_json(kind)])),
        instances_json: dsl::json::to_json_string(&dsl::DslValue::Array(vec![instance_json(kind, [0.0, 0.0, 0.0], 1.0, false)])),
        selection_json: world3d_selection_json("rectangle", &[], None),
    })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sourcing::config::SourcingCurationConfig;
    use crate::editor::sourcing::testkit::{new_app, render as render_body};

    /// 🧬️ Direct unit coverage for `render`'s own id-lookup logic — the app-level call site always
    /// passes an empty slice (see the `selected_ids` doc comment above) until a future wave threads
    /// interaction into `render`.
    /// 🎬️ Asserts through the packed scene, not the node JSON: `MeshWindowKit::render` hands the
    /// `World3dScene` to `semio_framework_ui_scene::encode`, which packs it into the surface's opaque
    /// `UiFixedBytes` — so a serialized `BuiltNode` no longer carries any scene string. Decoding the
    /// surface back is the current idiom (see `🎪️demonstrator`'s own `🪟️main` window test).
    #[semio_framework_async_macros::async_test]
    async fn preview_renders_selected_mesh_id() {
        let document = crate::artifacts::curation::schema::default_document();
        let object_id = crate::artifacts::curation::stock_of(&document)[0].id.clone();
        let node = render(&document, &[object_id.clone()], crate::editor::sourcing::terminology::sourcing_curation_labels(&SourcingCurationConfig::default())).expect("bounded preview");
        let semio_framework_plugin::Component::Surface(props) = node.component else { panic!("preview must build a World3d surface") };
        let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(&props).expect("decode world3d scene");
        assert!(scene.meshes_json.contains(&object_id), "the selected kind's mesh must be in the scene");
        assert!(scene.instances_json.contains(&object_id), "the selected kind must be instanced once");
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_shows_placeholder_without_selection() {
        let document = crate::artifacts::curation::schema::default_document();
        let node = render(&document, &[], crate::editor::sourcing::terminology::sourcing_curation_labels(&SourcingCurationConfig::default())).expect("bounded placeholder");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("No selection"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_world3d_surface_and_body_key() {
        let def = definition();
        assert_eq!(def.body_key, SOURCING_CURATION_BODY_PREVIEW);
        assert!(matches!(def.surface_kind, SurfaceKind::World3d));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_via_the_app() {
        let mut app = new_app().await;
        // `render` carries no `InteractionView` yet, so the app-level render always shows the placeholder.
        assert!(render_body(&mut app, SOURCING_CURATION_BODY_PREVIEW).await.contains("No selection"));
    }
}
//#endregion 🧪️Tests
