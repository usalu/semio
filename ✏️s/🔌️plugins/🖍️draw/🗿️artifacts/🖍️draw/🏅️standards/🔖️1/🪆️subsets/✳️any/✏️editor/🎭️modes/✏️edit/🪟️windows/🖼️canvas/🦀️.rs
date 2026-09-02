//! 🖼️ Draw play app — the canvas window's render() (constitutional: was `ui`'s `Render` region).

use crate::artifacts::draw::schema::{flatten_draw_document_to_scene_nodes, resolve_draw_artboard};
use crate::artifacts::draw::{DrawArtboard, DrawSnapshot, PathSegment};
use crate::editor::draw::commands::canvas_pointer_down::{draft_preview_segments, shape_preview_segments, DrawGesturePreview, DrawGesturePreviewPhase};
use crate::editor::draw::config::DrawConfig;
use semio_framework_plugin::{scene_surface, BuiltNode, Canvas2dScene, UiAssemblyResult};
use dsl::DslValue;

pub const DRAW_PLAY_WINDOW_CANVAS: &str = "draw-composite";
pub const DRAW_PLAY_SURFACE_ID: &str = "draw.play.composite";
pub const DRAW_PLAY_BODY_COMPOSITE: &str = "draw.play.composite";

const DRAW_OVERLAY_SELECTION_STROKE: [f64; 4] = [0.98, 0.75, 0.14, 0.95];
const DRAW_OVERLAY_SELECTION_FILL: [f64; 4] = [0.98, 0.75, 0.14, 0.16];
const DRAW_OVERLAY_MARQUEE_STROKE: [f64; 4] = [0.36, 0.65, 0.98, 0.9];
const DRAW_OVERLAY_MARQUEE_FILL: [f64; 4] = [0.36, 0.65, 0.98, 0.12];
const DRAW_ARTBOARD_FILL: [f64; 4] = [0.969, 0.953, 0.890, 1.0];
const DRAW_ARTBOARD_STROKE: [f64; 4] = [0.198, 0.223, 0.205, 0.55];
const DRAW_ARTBOARD_LABEL: [f64; 4] = [0.198, 0.223, 0.205, 0.92];

fn overlay_record<T: dsl::ToValue + ?Sized>(id: &str, transform: [f64; 6], segments: &T, fill: Option<[f64; 4]>, stroke_color: [f64; 4], stroke_width: f64) -> DslValue {
    DslValue::object([
        ("id".to_string(), DslValue::String(id.to_string())),
        ("role".to_string(), DslValue::String("overlay".to_string())),
        ("transform".to_string(), dsl::ToValue::to_value(&transform.to_vec())),
        ("segments".to_string(), dsl::ToValue::to_value(segments)),
        ("fill".to_string(), fill.map_or(DslValue::Null, |color| DslValue::object([("kind".to_string(), DslValue::String("solid".to_string())), ("color".to_string(), dsl::ToValue::to_value(&color.to_vec()))]))),
        (
            "stroke".to_string(),
            DslValue::object([
                ("color".to_string(), dsl::ToValue::to_value(&stroke_color.to_vec())),
                ("width".to_string(), DslValue::float(stroke_width)),
                ("cap".to_string(), DslValue::String("round".to_string())),
                ("join".to_string(), DslValue::String("round".to_string())),
            ]),
        ),
        ("opacity".to_string(), DslValue::float(1.0)),
        ("blendMode".to_string(), DslValue::String("normal".to_string())),
        ("visible".to_string(), DslValue::Bool(true)),
        ("fillRule".to_string(), DslValue::String("evenodd".to_string())),
    ])
}

/// 📐️ Formats one artboard edge length for the dimension label (integers stay bare).
fn format_artboard_dimension(value: f64) -> String {
    if (value - value.round()).abs() < 1e-6 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.2}")
    }
}

/// 🖼️ Artboard paper + `W × H` dimension label — drawn under document content.
fn artboard_scene_records(document: &DrawSnapshot) -> Vec<DslValue> {
    let artboard = resolve_draw_artboard(document).unwrap_or(DrawArtboard { width: 1024.0, height: 1024.0 });
    let width = artboard.width.max(1.0);
    let height = artboard.height.max(1.0);
    let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [width, 0.0] }, PathSegment::Line { to: [width, height] }, PathSegment::Line { to: [0.0, height] }, PathSegment::Close];
    let label = format!("{} × {}", format_artboard_dimension(width), format_artboard_dimension(height));
    let label_size = 12.0_f64;
    let label_x = (width * 0.5) - (label.len() as f64 * label_size * 0.28);
    vec![
        overlay_record("artboard:frame", [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAW_ARTBOARD_FILL), DRAW_ARTBOARD_STROKE, 1.0),
        DslValue::object([
            ("id".to_string(), DslValue::String("artboard:dimensions".to_string())),
            ("role".to_string(), DslValue::String("overlay".to_string())),
            ("transform".to_string(), dsl::ToValue::to_value(&vec![1.0_f64, 0.0, 0.0, 1.0, label_x, height + label_size * 0.35])),
            ("segments".to_string(), DslValue::Array(Vec::new())),
            ("fill".to_string(), DslValue::object([("kind".to_string(), DslValue::String("solid".to_string())), ("color".to_string(), dsl::ToValue::to_value(&DRAW_ARTBOARD_LABEL.to_vec()))])),
            ("opacity".to_string(), DslValue::float(1.0)),
            ("blendMode".to_string(), DslValue::String("normal".to_string())),
            ("visible".to_string(), DslValue::Bool(true)),
            ("text".to_string(), DslValue::object([("content".to_string(), DslValue::String(label)), ("size".to_string(), DslValue::float(label_size))])),
        ]),
    ]
}

/// 🕹️ `config` no longer carries `selected_ids`/`hovered_id` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection/hover are framework-owned state,
/// and `ArtifactApp::render` is never given an `InteractionView`) — the selection/hover overlay
/// records this function used to bake into `layersJson` are gone; the client renders that highlight
/// itself from the framework's own interaction state now.
pub fn render(document: &DrawSnapshot, config: &DrawConfig, preview: &DrawGesturePreview, active_utility: &str) -> UiAssemblyResult<BuiltNode> {
    let scene_nodes = flatten_draw_document_to_scene_nodes(document);
    let artboard_records = artboard_scene_records(document);
    let mut records: Vec<DslValue> = Vec::with_capacity(scene_nodes.len() + artboard_records.len() + 4);
    records.push(DslValue::object([
        ("id".to_string(), DslValue::String("meta:utility".to_string())),
        ("role".to_string(), DslValue::String("meta".to_string())),
        ("utility".to_string(), DslValue::String(active_utility.to_string())),
    ]));
    records.extend(artboard_records);
    for node in &scene_nodes {
        records.push(dsl::ToValue::to_value(node));
    }
    if preview.phase == DrawGesturePreviewPhase::Marquee {
        let ctx = &preview.context;
        let x = ctx.start[0].min(ctx.cursor[0]);
        let y = ctx.start[1].min(ctx.cursor[1]);
        let width = (ctx.cursor[0] - ctx.start[0]).abs();
        let height = (ctx.cursor[1] - ctx.start[1]).abs();
        let segments = vec![PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close];
        records.push(overlay_record("overlay:marquee", [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAW_OVERLAY_MARQUEE_FILL), DRAW_OVERLAY_MARQUEE_STROKE, 1.0));
    } else if preview.phase == DrawGesturePreviewPhase::Shape {
        let ctx = &preview.context;
        let segments = shape_preview_segments(&ctx.utility, ctx.start, ctx.cursor).iter().cloned().collect::<Vec<_>>();
        records.push(overlay_record("overlay:preview", [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAW_OVERLAY_SELECTION_FILL), DRAW_OVERLAY_SELECTION_STROKE, 1.5));
    } else if preview.phase == DrawGesturePreviewPhase::Draft {
        let ctx = &preview.context;
        let segments = draft_preview_segments(&ctx.utility, &ctx.points, ctx.cursor).iter().cloned().collect::<Vec<_>>();
        records.push(overlay_record("overlay:preview", [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAW_OVERLAY_SELECTION_FILL), DRAW_OVERLAY_SELECTION_STROKE, 1.5));
    }
    scene_surface(
        DRAW_PLAY_SURFACE_ID,
        semio_framework_ui_contract::SurfaceKind::Canvas2d,
        &Canvas2dScene { camera_x: config.camera.x, camera_y: config.camera.y, zoom: config.camera.zoom, layers_json: dsl::json::to_json_string(&records), snapshot: None },
    )
}
