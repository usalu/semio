//! 🖼️ Drawing viewer — the Canvas window: a read-only render of the 2D drawing document, built from the
//! same artifact-level pure snapshot→scene helpers the editor's own Canvas window
//! (the sibling editor module's `🎭️modes/✏️edit/🪟️windows/🖼️canvas`) uses — this file itself imports
//! nothing from that sibling surface (`policyViewerPurityBreaches` forbids it outright). No
//! selection, no gesture overlay, no engagement: a viewer has no utilities that edit and emits no
//! mutations by construction (`ViewEmit`).

use crate::schema::{flatten_drawing_document_to_scene_nodes, resolve_drawing_artboard};
use crate::{DrawingArtboard, DrawingSnapshot, PathSegment};
use dsl::DslValue;
use semio_framework_plugin::scene_surface;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::Canvas2dScene;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;

#[path = "🎚️config/🦀️.rs"]
pub mod config;
pub const SET_CAMERA_ACTION_ID: &str = "setCamera";

/// 📷️ Local camera navigation is available in a read-only canvas.
fn set_camera_action() -> semio_framework_plugin::ActionDefinition {
    let mut action = semio_framework_plugin::ActionDefinition::bounded_catalog(SET_CAMERA_ACTION_ID,LocalizedLabel::native("Set Camera","Kamera festlegen"),semio_framework_plugin::ActionKind::View)
        .with_args(vec![semio_framework_plugin::ActionArgDef::text("camera",LocalizedLabel::native("Camera","Kamera")).required()]);
    action.semantics.execution.interactive_job = semio_framework_plugin::InteractiveJobClassification::Migrated;
    action.in_palette = false;
    action
}

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "drawing-view-canvas";
pub const BODY_KEY: &str = "drawing.view.canvas";
pub const SURFACE_ID: &str = "drawing.view.composite";
const DRAWING_ARTBOARD_FILL: [f64; 4] = [0.969, 0.953, 0.890, 1.0];
const DRAWING_ARTBOARD_STROKE: [f64; 4] = [0.198, 0.223, 0.205, 0.55];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::drawing::create_drawing_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Canvas", "Leinwand"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "pen-tool".into(),
        options: WindowOptions::default(),
        actions: vec![set_camera_action()],
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Read-only scene with measured initial fitting to the artifact world bounds.
pub fn render(document: &DrawingSnapshot) -> UiAssemblyResult<BuiltNode> {
    render_with_camera(document,&config::DrawingViewerCanvasWindowConfig::default())
}

/// 🧭️ Restored local navigation suppresses the initial artwork fit.
pub fn render_with_camera(document: &DrawingSnapshot,config: &config::DrawingViewerCanvasWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let camera = config.viewport;
    let artboard_records = artboard_scene_records(document);
    let scene_nodes = flatten_drawing_document_to_scene_nodes(document);
    let mut records: Vec<DslValue> = Vec::with_capacity(scene_nodes.len() + artboard_records.len());
    records.extend(artboard_records);
    for node in &scene_nodes {
        records.push(dsl::ToValue::to_value(node));
    }
    scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Canvas2d, &Canvas2dScene { framing: (!config.framed).then(|| semio_framework_plugin::Canvas2dFraming { revision: 0,bounds: crate::schema::geometry::framing::drawing_scene_bounds(document.artboard.as_ref(),&scene_nodes),padding: 48.0 }), camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json: dsl::json::to_json_string(&records), snapshot: None, tool_run_trace: None, lanes: Vec::new() })
}

/// 👁️ Read-only twin of the editor's `edit::artboard_scene_records` frame-only half (no dimension
/// label — cosmetic, dropped for the viewer's minimal first pass) — duplicated on purpose rather than
/// imported through the sibling editor module, which `policyViewerPurityBreaches` forbids outright.
fn artboard_scene_records(document: &DrawingSnapshot) -> Vec<DslValue> {
    let artboard = resolve_drawing_artboard(document).unwrap_or(DrawingArtboard { width: 1024.0, height: 1024.0 });
    let width = artboard.width.max(1.0);
    let height = artboard.height.max(1.0);
    let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [width, 0.0] }, PathSegment::Line { to: [width, height] }, PathSegment::Line { to: [0.0, height] }, PathSegment::Close];
    vec![DslValue::object([
        ("id".to_string(), DslValue::String("artboard:frame".to_string())),
        ("role".to_string(), DslValue::String("overlay".to_string())),
        ("transform".to_string(), dsl::ToValue::to_value(&vec![1.0_f64, 0.0, 0.0, 1.0, 0.0, 0.0])),
        ("segments".to_string(), dsl::ToValue::to_value(&segments)),
        ("fill".to_string(), DslValue::object([("kind".to_string(), DslValue::String("solid".to_string())), ("color".to_string(), dsl::ToValue::to_value(&DRAWING_ARTBOARD_FILL.to_vec()))])),
        (
            "stroke".to_string(),
            DslValue::object([
                ("color".to_string(), dsl::ToValue::to_value(&DRAWING_ARTBOARD_STROKE.to_vec())),
                ("width".to_string(), DslValue::float(1.0)),
                ("cap".to_string(), DslValue::String("round".to_string())),
                ("join".to_string(), DslValue::String("round".to_string())),
            ]),
        ),
        ("opacity".to_string(), DslValue::float(1.0)),
        ("blendMode".to_string(), DslValue::String("normal".to_string())),
        ("visible".to_string(), DslValue::Bool(true)),
        ("fillRule".to_string(), DslValue::String("evenodd".to_string())),
    ])]
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
