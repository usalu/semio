//! 👁️ Sourcing curation app — the preview window: a 3D preview of the currently-selected object.

use crate::schema::{instance_json, kind_mesh_json};
use crate::CurationSnapshot;
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
    let stock = crate::stock_of(document);
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
