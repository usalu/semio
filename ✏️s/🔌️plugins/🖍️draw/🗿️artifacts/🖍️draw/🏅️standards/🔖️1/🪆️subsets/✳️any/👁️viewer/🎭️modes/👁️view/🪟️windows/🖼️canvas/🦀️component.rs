//! 🖼️ Draw viewer — the Canvas window: a read-only render of the 2D drawing document, built from the
//! same artifact-level pure snapshot→scene helpers the editor's own Canvas window
//! (the sibling editor module's `🎭️modes/✏️edit/🪟️windows/🖼️canvas`) uses — this file itself imports
//! nothing from that sibling surface (`policyViewerPurityBreaches` forbids it outright). No
//! selection, no gesture overlay, no engagement: a viewer has no utilities that edit and emits no
//! mutations by construction (`ViewEmit`).

use crate::artifacts::draw::schema::{flatten_draw_document_to_scene_nodes, resolve_draw_artboard};
use crate::artifacts::draw::{DrawArtboard, DrawCamera, DrawSnapshot, PathSegment};
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "draw-view-canvas";
pub const BODY_KEY: &str = "draw.view.canvas";
pub const SURFACE_ID: &str = "draw.view.composite";
/// 👁️ Read-only counterpart of the editor's `DRAW_PLAY_CONTROLLER_ID` controller id — kept distinct
/// so a viewer session's canvas controller can never be mistaken for an editor session's.
const DRAW_VIEW_CONTROLLER_ID: &str = "draw-view";
const DRAW_ARTBOARD_FILL: [f64; 4] = [0.969, 0.953, 0.890, 1.0];
const DRAW_ARTBOARD_STROKE: [f64; 4] = [0.198, 0.223, 0.205, 0.55];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::draw::create_draw_viewer`.
pub async fn definition() -> WindowKindDefinition {
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
/// 👁️ Pure `DrawSnapshot -> UiNode` read: a hardcoded default camera (a viewer has no persisted
/// per-session camera — `Config = NoConfig`), no selection/gesture overlay, real artboard frame +
/// document content read straight off the document.
pub async fn render(document: &DrawSnapshot) -> UiNode {
    let camera = DrawCamera::default();
    let artboard_records = artboard_scene_records(document);
    let scene_nodes = flatten_draw_document_to_scene_nodes(document);
    let mut records: Vec<Value> = Vec::with_capacity(scene_nodes.len() + artboard_records.len());
    records.extend(artboard_records);
    for node in &scene_nodes {
        records.push(serde_json::to_value(node).unwrap_or(Value::Null));
    }
    build_canvas_2d_scene(
        SURFACE_ID,
        DRAW_VIEW_CONTROLLER_ID,
        Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json: serde_json::to_string(&records).unwrap_or_else(|_| "[]".into()) },
    )
}

/// 👁️ Read-only twin of the editor's `edit::artboard_scene_records` frame-only half (no dimension
/// label — cosmetic, dropped for the viewer's minimal first pass) — duplicated on purpose rather than
/// imported through the sibling editor module, which `policyViewerPurityBreaches` forbids outright.
async fn artboard_scene_records(document: &DrawSnapshot) -> Vec<Value> {
    let artboard = resolve_draw_artboard(document).unwrap_or(DrawArtboard { width: 1024.0, height: 1024.0 });
    let width = artboard.width.max(1.0);
    let height = artboard.height.max(1.0);
    let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [width, 0.0] }, PathSegment::Line { to: [width, height] }, PathSegment::Line { to: [0.0, height] }, PathSegment::Close];
    vec![json!({
        "id": "artboard:frame",
        "role": "overlay",
        "transform": [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        "segments": segments,
        "fill": { "kind": "solid", "color": DRAW_ARTBOARD_FILL },
        "stroke": { "color": DRAW_ARTBOARD_STROKE, "width": 1.0, "cap": "round", "join": "round" },
        "opacity": 1.0,
        "blendMode": "normal",
        "visible": true,
        "fillRule": "evenodd",
    })]
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_a_canvas2d_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.surface_kind, SurfaceKind::Canvas2d);
    }

    #[test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::draw::schema::default_draw_document("empty", None);
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
