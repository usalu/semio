//! 👁️ Sourcing curation app — the preview window: a 3D preview of the currently-selected object.

use crate::schema::{box_parts, glb_mesh_json, kind_instances_json, kind_mesh_json, preview_kind_bounds, unit_box_mesh_json};
use crate::GeometryRecipe;
use crate::ObjectKind;
use crate::CurationSnapshot;
use crate::editor::sourcing::terminology::SourcingLabels;
use semio_framework_plugin::{scene_surface, world3d_default_camera, world3d_scene, world3d_selection_json, BuiltNode, Label, LocalizedLabel, PluginAssemblyError, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions, WorldSunConfig, World3dScene};

//#region 🔖️Constants
pub const SOURCING_CURATION_WINDOW_PREVIEW: &str = "sourcing-preview";
pub const SOURCING_CURATION_BODY_PREVIEW: &str = "sourcing.preview";
pub const SOURCING_CURATION_SURFACE_PREVIEW: &str = "sourcing.preview";
const SOURCING_PREVIEW_FIT_PADDING: f64 = 1.25;
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

//#region 🔖️Framing
fn preview_fit_revision(kind_id: &str) -> u32 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in kind_id.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    (hash >> 32) as u32
}

fn preview_fit_json(kind: &ObjectKind) -> String {
    let revision = preview_fit_revision(&kind.id);
    let bounds = preview_kind_bounds(kind);
    let f64_array = |values: [f64; 3]| dsl::DslValue::Array(values.into_iter().map(dsl::DslValue::float).collect());
    let mut entries = vec![
        ("enabled".to_string(), dsl::DslValue::Bool(true)),
        ("revision".to_string(), dsl::DslValue::uint(u64::from(revision))),
        ("padding".to_string(), dsl::DslValue::float(SOURCING_PREVIEW_FIT_PADDING)),
    ];
    if let Some((minimum, maximum)) = bounds {
        entries.push(("boundsMin".to_string(), f64_array(minimum)));
        entries.push(("boundsMax".to_string(), f64_array(maximum)));
    }
    dsl::json::to_json_string(&dsl::DslValue::Object(entries))
}
//#endregion 🔖️Framing

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
    let meshes_json = dsl::json::to_json_string(&dsl::DslValue::Array(vec![if box_parts(&kind.geometry).is_some() {
        unit_box_mesh_json()
    } else if let GeometryRecipe::Glb { url, .. } = &*kind.geometry {
        glb_mesh_json(url)
    } else {
        kind_mesh_json(kind)
    }]));
    let instances_json = dsl::json::to_json_string(&dsl::DslValue::Array(kind_instances_json(kind, [0.0, 0.0, 0.0], 1.0, false)));
    let sun = WorldSunConfig::default();
    let scene = World3dScene { fit_json: Some(preview_fit_json(kind)), ..world3d_scene(world3d_default_camera(), meshes_json, instances_json, world3d_selection_json("rectangle", &[], None), &sun) };
    scene_surface(SOURCING_CURATION_SURFACE_PREVIEW, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
