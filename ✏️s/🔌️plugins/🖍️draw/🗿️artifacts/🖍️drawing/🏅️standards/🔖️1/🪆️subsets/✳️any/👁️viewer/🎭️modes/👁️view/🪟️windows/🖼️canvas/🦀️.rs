//! 🖼️ Drawing viewer — the Canvas window: a read-only render of the 2D drawing document, built from the
//! same artifact-level pure snapshot→scene helpers the editor's own Canvas window
//! (the sibling editor module's `🎭️modes/✏️edit/🪟️windows/🖼️canvas`) uses — this file itself imports
//! nothing from that sibling surface (`policyViewerPurityBreaches` forbids it outright). No
//! selection, no gesture overlay, no engagement: a viewer has no utilities that edit and emits no
//! mutations by construction (`ViewEmit`).

use crate::artifacts::drawing::schema::{flatten_drawing_document_to_scene_nodes, resolve_drawing_artboard};
use crate::artifacts::drawing::{DrawingArtboard, DrawingCamera, DrawingSnapshot, PathSegment};
use semio_framework_plugin::{scene_surface, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use dsl::DslValue;

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
        actions: Vec::new(),
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
/// 👁️ Pure `DrawingSnapshot -> UiNode` read: a hardcoded default camera (a viewer has no persisted
/// per-session camera — `Config = NoConfig`), no selection/gesture overlay, real artboard frame +
/// document content read straight off the document.
pub fn render(document: &DrawingSnapshot) -> UiAssemblyResult<BuiltNode> {
    let camera = DrawingCamera::default();
    let artboard_records = artboard_scene_records(document);
    let scene_nodes = flatten_drawing_document_to_scene_nodes(document);
    let mut records: Vec<DslValue> = Vec::with_capacity(scene_nodes.len() + artboard_records.len());
    records.extend(artboard_records);
    for node in &scene_nodes {
        records.push(dsl::ToValue::to_value(node));
    }
    scene_surface(
        SURFACE_ID,
        semio_framework_ui_contract::SurfaceKind::Canvas2d,
        &Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json: dsl::json::to_json_string(&records), snapshot: None },
    )
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_canvas2d_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.surface_kind, SurfaceKind::Canvas2d);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::drawing::schema::default_drawing_document("empty", None);
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
