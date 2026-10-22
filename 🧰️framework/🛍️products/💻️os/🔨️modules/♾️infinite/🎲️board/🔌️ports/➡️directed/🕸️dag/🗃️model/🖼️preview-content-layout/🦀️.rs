//! 🖼️ Preview content layout of the DAG model: the preview constants, tree rows and layouts, widget pointer kinds, text and media measuring and the preview widget sizes. See [`super`].

use super::*;

// #region 🔖️PreviewContent
pub const DAG_PREVIEW_PAD: f64 = 4.0;

pub const DAG_PREVIEW_ROW_HEIGHT: f64 = 14.0;

pub const DAG_PREVIEW_TREE_INDENT: f64 = 12.0;

pub const DAG_PREVIEW_TOGGLE_WIDTH: f64 = 10.0;

const DAG_PREVIEW_MAX_IMAGE: f64 = 200.0;

const DAG_PREVIEW_MIN_SIZE: f64 = 20.0;

#[derive(Clone, Debug)]
pub struct PreviewTreeRow {
    pub path: String,
    pub depth: usize,
    pub label: String,
    pub summary: String,
    pub has_children: bool,
    pub expanded: bool,
}

#[derive(Clone, Debug)]
pub struct PreviewTreeRowLayout {
    pub path: String,
    pub row_rect: (f64, f64, f64, f64),
}

pub enum WidgetPointerKind {
    SliderDrag,
    SelectClick,
    PreviewToggle(String),
    ClusterExplode,
    ExportClick,
}

fn preview_scalar_text_width(text: &str) -> f64 {
    let px = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_SCALE_MULT;
    port_label_text_width(text, px).max(text.len() as f64 * px * 0.55)
}

fn preview_media_natural_size(src: &str) -> (f64, f64) {
    use canvas::icon_codec::{resolve_icon_kind, ResolvedIcon};
    match resolve_icon_kind(src, |_| None) {
        ResolvedIcon::RasterRgba8 { w, h, .. } => (f64::from(w), f64::from(h)),
        ResolvedIcon::SvgPlain(s) | ResolvedIcon::SvgThemed(s) => {
            if let Ok((_, _, bw, bh)) = canvas::svg_icon::svg_icon_content_bounds_from_str(&s) {
                if bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite() {
                    return (bw, bh);
                }
            }
            (64.0, 48.0)
        }
        ResolvedIcon::None => (64.0, 48.0),
    }
}

fn clamp_preview_image_size(w: f64, h: f64) -> (f64, f64) {
    let max = DAG_PREVIEW_MAX_IMAGE;
    if w <= max && h <= max {
        return (w.max(1.0), h.max(1.0));
    }
    let scale = (max / w).min(max / h);
    ((w * scale).max(1.0), (h * scale).max(1.0))
}

fn preview_tree_collapsed_summary(value: &DslValue) -> String {
    match value {
        DslValue::Object(entries) => format!("{{{} keys}}", entries.len()),
        DslValue::Array(arr) => format!("[{} items]", arr.len()),
        DslValue::String(s) => format!("\"{s}\""),
        DslValue::Bytes(bytes) => format!("<{} bytes>", bytes.len()),
        DslValue::Number(n) => match n {
            semio_framework_value::Number::UInt(v) => v.to_string(),
            semio_framework_value::Number::Int(v) => v.to_string(),
            semio_framework_value::Number::Float(v) => v.to_string(),
        },
        DslValue::Bool(b) => b.to_string(),
        DslValue::Null => "null".into(),
    }
}

fn preview_tree_scalar_display(value: &DslValue) -> String {
    match value {
        DslValue::String(s) => format!("\"{s}\""),
        other => preview_tree_collapsed_summary(other),
    }
}

pub fn truncate_label_to_fit_width(text: &str, max_width: f64, px: f64) -> String {
    use canvas::text::label_extent;
    let trimmed = text.trim();
    if trimmed.is_empty() || max_width <= 0.0 || px < 4.0 {
        return text.to_string();
    }
    let (full_w, _) = label_extent(trimmed, px);
    if full_w <= max_width {
        return text.to_string();
    }
    let ellipsis = "…";
    let mut best = ellipsis.to_string();
    for (byte_idx, ch) in trimmed.char_indices() {
        let end = byte_idx + ch.len_utf8();
        let candidate = format!("{}{ellipsis}", &trimmed[..end]);
        let (w, _) = label_extent(&candidate, px);
        if w <= max_width {
            best = candidate;
        } else {
            break;
        }
    }
    best
}

pub fn note_text_origin_x(node: &DagNodeSpec) -> f64 {
    let (x0, _, _, _) = preview_content_bounds(node);
    x0 + DAG_PREVIEW_PAD
}

pub fn hit_byte_in_note_line(line: &str, world_x: f64, line_origin_x: f64, font_px: f64) -> usize {
    use canvas::text::label_byte_world_x;
    if line.is_empty() {
        return 0;
    }
    let mut boundaries = vec![0usize];
    for (index, _) in line.char_indices() {
        if index > 0 {
            boundaries.push(index);
        }
    }
    if boundaries.last().copied() != Some(line.len()) {
        boundaries.push(line.len());
    }
    for pair in boundaries.windows(2) {
        let start = pair[0];
        let end = pair[1];
        let x0 = label_byte_world_x(line, start, line_origin_x, font_px);
        let x1 = label_byte_world_x(line, end, line_origin_x, font_px);
        if world_x < (x0 + x1) * 0.5 {
            return start;
        }
    }
    line.len()
}

pub fn preview_tree_rows(json: &DslValue, expanded: &DagExpandedPaths, path: &str, depth: usize) -> Vec<PreviewTreeRow> {
    let mut rows = Vec::new();
    match json {
        DslValue::Object(entries) => {
            for (key, val) in entries {
                let row_path = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
                let has_children = matches!(val, DslValue::Object(_) | DslValue::Array(_));
                let is_expanded = expanded.contains(&row_path);
                let summary = if has_children { preview_tree_collapsed_summary(val) } else { preview_tree_scalar_display(val) };
                rows.push(PreviewTreeRow { path: row_path.clone(), depth, label: key.clone(), summary, has_children, expanded: is_expanded });
                if has_children && is_expanded {
                    rows.extend(preview_tree_rows(val, expanded, &row_path, depth + 1));
                }
            }
        }
        DslValue::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let key = format!("[{i}]");
                let row_path = if path.is_empty() { key.clone() } else { format!("{path}{key}") };
                let has_children = matches!(val, DslValue::Object(_) | DslValue::Array(_));
                let is_expanded = expanded.contains(&row_path);
                let summary = if has_children { preview_tree_collapsed_summary(val) } else { preview_tree_scalar_display(val) };
                rows.push(PreviewTreeRow { path: row_path.clone(), depth, label: key, summary, has_children, expanded: is_expanded });
                if has_children && is_expanded {
                    rows.extend(preview_tree_rows(val, expanded, &row_path, depth + 1));
                }
            }
        }
        _ => {}
    }
    rows
}

/// 📐️ Measures preview content size in world units.
pub fn measure_preview_content(content: &DagPreviewContent, expanded: &DagExpandedPaths) -> (f64, f64) {
    match content {
        DagPreviewContent::Empty => (DAG_PREVIEW_MIN_SIZE, DAG_PREVIEW_MIN_SIZE),
        DagPreviewContent::Scalar { text } => {
            let tw = preview_scalar_text_width(text);
            (tw.max(DAG_PREVIEW_MIN_SIZE), DAG_PREVIEW_ROW_HEIGHT.max(DAG_PREVIEW_MIN_SIZE))
        }
        DagPreviewContent::Image { src } => clamp_preview_image_size(preview_media_natural_size(src).0, preview_media_natural_size(src).1),
        DagPreviewContent::Tree { json } => {
            let rows = preview_tree_rows(json, expanded, "", 0);
            if rows.is_empty() {
                return (DAG_PREVIEW_MIN_SIZE, DAG_PREVIEW_MIN_SIZE);
            }
            let max_w = rows
                .iter()
                .map(|row| {
                    let indent = row.depth as f64 * DAG_PREVIEW_TREE_INDENT;
                    let toggle = if row.has_children { DAG_PREVIEW_TOGGLE_WIDTH } else { 0.0 };
                    let line = if row.has_children && row.expanded { row.label.clone() } else { format!("{}: {}", row.label, row.summary) };
                    indent + toggle + port_label_text_width(&line, DAG_LABEL_COMPACT_SCREEN_PX)
                })
                .fold(DAG_PREVIEW_MIN_SIZE, f64::max);
            (max_w, rows.len() as f64 * DAG_PREVIEW_ROW_HEIGHT)
        }
    }
}

pub(super) fn preview_content_node_size(content: &DagPreviewContent, expanded: &DagExpandedPaths) -> (f64, f64) {
    let (_, ch) = measure_preview_content(content, expanded);
    (DAG_COMPONENT_WIDTH, ch + DAG_PREVIEW_PAD * 2.0)
}

pub(super) fn preview_image_node_size(src: &str) -> (f64, f64) {
    let inner_w = (DAG_COMPONENT_WIDTH - DAG_PREVIEW_PAD * 2.0).max(1.0);
    let (nw, nh) = preview_media_natural_size(src);
    let (cw, ch) = clamp_preview_image_size(nw, nh);
    let aspect = if cw > 0.0 { ch / cw } else { 1.0 };
    let inner_h = (inner_w * aspect).max(DAG_PREVIEW_MIN_SIZE);
    (DAG_COMPONENT_WIDTH, inner_h + DAG_PREVIEW_PAD * 2.0)
}

/// 📐️ Image input node size from media source.
pub fn image_widget_size(src: &str) -> (f64, f64) {
    preview_image_node_size(src)
}

/// 📐️ Preview node size from typed content and fold state.
pub fn preview_widget_size(content: &DagPreviewContent, expanded: &DagExpandedPaths) -> (f64, f64) {
    preview_content_node_size(content, expanded)
}

pub fn preview_tree_row_layouts(node: &DagNodeSpec, json: &DslValue, expanded: &DagExpandedPaths) -> Vec<PreviewTreeRowLayout> {
    let (x0, y0, x1, _y1) = preview_content_bounds(node);
    preview_tree_rows(json, expanded, "", 0)
        .into_iter()
        .enumerate()
        .map(|(index, row)| {
            let row_y0 = y0 + index as f64 * DAG_PREVIEW_ROW_HEIGHT;
            let row_y1 = row_y0 + DAG_PREVIEW_ROW_HEIGHT;
            let row_rect = if row.has_children { (x0, row_y0, x0 + (x1 - x0).max(1.0), row_y1) } else { (0.0, 0.0, 0.0, 0.0) };
            PreviewTreeRowLayout { path: row.path, row_rect }
        })
        .collect()
}

// #endregion 🔖️PreviewContent

pub(super) fn uses_computation_layout(kind: &DagNodeKind) -> bool {
    matches!(
        kind,
        DagNodeKind::Computation { .. }
            | DagNodeKind::Cluster { .. }
            | DagNodeKind::Slider { .. }
            | DagNodeKind::Note { .. }
            | DagNodeKind::Image { .. }
            | DagNodeKind::Preview { .. }
            | DagNodeKind::Action { .. }
            | DagNodeKind::Export { .. }
    )
}
