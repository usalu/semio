//! 🎨️ Node style, colors and level of detail of the DAG model: the handle radius, stroke and label constants, the stroke and label sizes, the body, label, chrome, handle and edge colours, the draw level of detail and its scale JSON. See [`super`].

use super::*;

/// 🔵️ Port dot world radius; screen size grows with camera zoom like node geometry.
pub const DAG_HANDLE_WORLD_RADIUS: f64 = ui_styling::radii::DAG_HANDLE_WORLD;

/// 🏷️ Share of a caption's own slot the text may occupy, leaving a hairline of breathing room. The
/// same number both presentations used to derive privately; it is published on the row now
/// (`maxScreenW`) so the budget is decided once, by the host that knows the layout.
pub(super) const DAG_LABEL_SCREEN_INSET: f64 = 0.88;

const DAG_NODE_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE;

const DAG_NODE_STROKE_SELECTED_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE_SELECTED;

const DAG_NODE_STROKE_HOVERED_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE_HOVERED;

pub const DAG_CHROME_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_CHROME;

pub const DAG_BOUNDED_DRAG_HIT_PAD_PX: f64 = ui_styling::metrics::board::BOUNDED_DRAG_HIT_PAD_PX;

pub fn dag_world_stroke(screen_px: f64, zoom: f64) -> f64 {
    (screen_px / zoom.max(0.05)).max(1e-3)
}

pub fn dag_label_layout_px() -> f64 {
    DAG_LABEL_SCREEN_PX
}

pub fn dag_label_paint_px(zoom: f64, lod_index: usize) -> f64 {
    canvas::lod::lod_band_label_screen_px(DAG_LABEL_SCREEN_PX, zoom, dag_lod_band_floor_zoom(lod_index))
}

pub fn dag_label_compact_paint_px(zoom: f64, lod_index: usize) -> f64 {
    canvas::lod::lod_band_label_screen_px(DAG_LABEL_COMPACT_SCREEN_PX, zoom, dag_lod_band_floor_zoom(lod_index))
}

fn dag_node_body_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.node_fill_disabled, ui_styling::opacities::DISABLED_FILL_ALPHA)
    } else if selected {
        theme.node_fill_selected
    } else if highlighted {
        theme.node_fill_selection_exit
    } else if hovered {
        theme.node_fill_hovered
    } else {
        theme.node_fill
    }
}

pub fn dag_node_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.node_stroke, ui_styling::opacities::DIM_STROKE_ALPHA)
    } else if selected {
        theme.node_stroke_selected
    } else if highlighted {
        theme.node_stroke_selection_exit
    } else if hovered {
        theme.node_stroke_hovered
    } else {
        theme.node_stroke
    }
}

pub fn dag_node_label_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.label_fill, ui_styling::opacities::DIM_LABEL_ALPHA)
    } else if selected {
        theme.label_fill_hovered
    } else if highlighted {
        theme.node_stroke_selection_exit
    } else if hovered {
        theme.label_fill_hovered
    } else {
        theme.label_fill
    }
}

/// 🧱️ Internal column/row chrome inside a node body; selection/hover matches label emphasis.
pub fn dag_node_internal_chrome_stroke(body_stroke: canvas::Color, label_fill: canvas::Color, emphasized: bool) -> canvas::Color {
    if emphasized {
        label_fill
    } else {
        body_stroke
    }
}

pub fn dag_handle_body_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.handle_fill_disabled, ui_styling::opacities::DISABLED_FILL_ALPHA)
    } else if selected {
        theme.handle_fill_selected
    } else if highlighted {
        theme.handle_fill_selection_exit
    } else if hovered {
        theme.handle_fill_hovered
    } else {
        theme.handle_fill
    }
}

pub fn dag_handle_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.handle_stroke_disabled, ui_styling::opacities::DISABLED_STROKE_ALPHA)
    } else if selected {
        theme.handle_stroke_selected
    } else if highlighted {
        theme.handle_stroke_selection_exit
    } else if hovered {
        theme.handle_stroke_hovered
    } else {
        theme.handle_stroke
    }
}

pub fn dag_edge_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.edge_stroke_disabled, ui_styling::opacities::DISABLED_STROKE_ALPHA)
    } else if selected {
        theme.edge_stroke_selected
    } else if highlighted {
        theme.edge_stroke_selection_exit
    } else if hovered {
        theme.edge_stroke_hovered
    } else {
        theme.edge_stroke
    }
}

pub fn dag_node_stroke_screen_px(dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> f64 {
    if dimmed {
        1.0
    } else if selected {
        DAG_NODE_STROKE_SELECTED_SCREEN_PX
    } else if highlighted || hovered {
        DAG_NODE_STROKE_HOVERED_SCREEN_PX
    } else {
        DAG_NODE_STROKE_SCREEN_PX
    }
}

/// 🎨️ Node body fill when painted; `None` means stroke/text only (board overview+).
pub fn dag_node_paint_fill(lod: DagDrawLod, theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> Option<canvas::Color> {
    if lod == DagDrawLod::Minimap {
        return Some(dag_node_body_stroke(theme, dimmed, selected, highlighted, hovered));
    }
    let chrome = dimmed || selected || highlighted || hovered;
    if chrome {
        Some(dag_node_body_fill(theme, dimmed, selected, highlighted, hovered))
    } else {
        None
    }
}

/// 📶️ Resolves the DAG draw LOD for a camera zoom factor.
pub fn dag_draw_lod(zoom: f64) -> DagDrawLod {
    DagDrawLod::from_scale_index(dag_lod_index(zoom))
}

fn lod_max_zoom_json(max_zoom: f64) -> Value {
    if max_zoom.is_finite() {
        Value::from(max_zoom)
    } else {
        Value::from(f64::MAX)
    }
}

/// 📶️ JSON LOD table for React window chrome (`id`, `name`, `description`, `maxZoom`).
pub fn dag_lod_scale_json() -> String {
    let rows: Vec<Value> = DAG_LODS
        .iter()
        .map(|lod| {
            let max_zoom = if lod.max_zoom.is_finite() { lod.max_zoom + DAG_LOD_ZOOM_SHIFT } else { lod.max_zoom };
            semio_framework_pack_json::object([("id".to_string(), Value::from(lod.id)), ("name".to_string(), Value::from(lod.name)), ("description".to_string(), Value::from(lod.description)), ("maxZoom".to_string(), lod_max_zoom_json(max_zoom))])
        })
        .collect();
    semio_framework_pack_json::to_string(&Value::Array(rows))
}
