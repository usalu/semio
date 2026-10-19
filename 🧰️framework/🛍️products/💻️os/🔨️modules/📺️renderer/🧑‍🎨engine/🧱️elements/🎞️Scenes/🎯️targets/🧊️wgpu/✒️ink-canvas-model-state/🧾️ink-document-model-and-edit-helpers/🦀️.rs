//! 🧾️ Ink document model and edit helpers: the host id counter, the camera and document JSON, the item accessors, the scale, resize and snap helpers, the item creation, the stroke erasing and the table cell edit helpers. See [`super`].

use super::*;

//#region InkCanvas
//#region InkCanvasModel
static INK_HOST_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub(super) fn create_ink_host_id(prefix: &str) -> String {
    let next = INK_HOST_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    format!("{prefix}-host-{next}")
}

#[derive(Clone, Copy, Debug, Default)]
pub struct InkCameraF {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl From<InkCameraJson> for InkCameraF {
    fn from(camera: InkCameraJson) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InkCameraJson {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "ink_default_zoom")]
    pub(crate) zoom: f64,
}

fn ink_default_zoom() -> f64 {
    1.0
}

impl Default for InkCameraJson {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct InkDocumentJson {
    pub schema: String,
    pub id: String,
    pub camera: InkCameraJson,
    pub blocks: Vec<Value>,
    pub active_utility: Option<String>,
    pub snap_enabled: Option<bool>,
    pub snap_grid_spacing: Option<f64>,
    pub eraser_radius: Option<f64>,
    pub(crate) grid_visible: Option<bool>,
    pub(crate) grid_spacing: Option<f64>,
    pub(crate) grid_subdivisions: Option<f64>,
    pub(crate) grid_opacity: Option<f64>,
    pub(crate) assets: HashMap<String, Value>,
}

impl Default for InkDocumentJson {
    fn default() -> Self {
        Self {
            schema: "ink.document".into(),
            id: "empty".into(),
            camera: InkCameraJson::default(),
            blocks: Vec::new(),
            active_utility: Some("selectDirect".into()),
            snap_enabled: None,
            snap_grid_spacing: None,
            eraser_radius: None,
            grid_visible: None,
            grid_spacing: None,
            grid_subdivisions: None,
            grid_opacity: None,
            assets: HashMap::new(),
        }
    }
}

pub(crate) fn ink_item_str<'a>(block: &'a Value, key: &str) -> &'a str {
    block.get(key).and_then(Value::as_str).unwrap_or("")
}

pub fn ink_item_id(block: &Value) -> &str {
    ink_item_str(block, "id")
}

pub fn ink_item_kind(block: &Value) -> &str {
    ink_item_str(block, "kind")
}

pub fn ink_item_locked(block: &Value) -> bool {
    block.get("locked").and_then(Value::as_bool).unwrap_or(false)
}

pub fn ink_item_num(block: &Value, key: &str) -> f64 {
    block.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

pub(crate) fn ink_item_bounds(block: &Value) -> InkBoundsF {
    let x = ink_item_num(block, "x");
    let y = ink_item_num(block, "y");
    let w = ink_item_num(block, "width");
    let h = ink_item_num(block, "height");
    if ink_item_kind(block) == "stroke" {
        if let Some(points) = block.get("points").and_then(Value::as_array) {
            if !points.is_empty() {
                let mut min_x = f64::INFINITY;
                let mut min_y = f64::INFINITY;
                let mut max_x = f64::NEG_INFINITY;
                let mut max_y = f64::NEG_INFINITY;
                for point in points {
                    let px = point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    min_x = min_x.min(px);
                    min_y = min_y.min(py);
                    max_x = max_x.max(px);
                    max_y = max_y.max(py);
                }
                return InkBoundsF { x: x + min_x, y: y + min_y, w: (max_x - min_x).max(1.0), h: (max_y - min_y).max(1.0) };
            }
        }
    }
    InkBoundsF { x, y, w, h }
}

fn ink_scale_value(v: f64, from_min: f64, from_size: f64, to_min: f64, to_size: f64) -> f64 {
    if from_size <= 0.0 {
        return to_min;
    }
    to_min + ((v - from_min) / from_size) * to_size
}

pub fn scale_ink_item(block: &Value, from: InkBoundsF, to: InkBoundsF) -> Value {
    let bounds = ink_item_bounds(block);
    let next_x = ink_scale_value(bounds.x, from.x, from.w, to.x, to.w);
    let next_y = ink_scale_value(bounds.y, from.y, from.h, to.y, to.h);
    let next_w = (ink_scale_value(bounds.x + bounds.w, from.x, from.w, to.x, to.w) - next_x).max(8.0);
    let next_h = (ink_scale_value(bounds.y + bounds.h, from.y, from.h, to.y, to.h) - next_y).max(8.0);
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(next_x));
        obj.insert("y".into(), json!(next_y));
        obj.insert("width".into(), json!(next_w));
        obj.insert("height".into(), json!(next_h));
        if ink_item_kind(block) == "stroke" {
            let scale_x = if from.w > 0.0 { to.w / from.w } else { 1.0 };
            let scale_y = if from.h > 0.0 { to.h / from.h } else { 1.0 };
            if let Some(points) = block.get("points").and_then(Value::as_array) {
                let scaled: Vec<Value> = points
                    .iter()
                    .map(|p| {
                        let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0) * scale_x;
                        let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0) * scale_y;
                        json!([px, py])
                    })
                    .collect();
                obj.insert("points".into(), Value::Array(scaled));
            }
        }
    }
    cloned
}

pub fn ink_resize_bounds(from: InkBoundsF, handle: &str, dx: f64, dy: f64, min_size: f64) -> InkBoundsF {
    let mut x = from.x;
    let mut y = from.y;
    let mut w = from.w;
    let mut h = from.h;
    if handle.contains('e') {
        w = (w + dx).max(min_size);
    }
    if handle.contains('w') {
        let next_w = (w - dx).max(min_size);
        x += w - next_w;
        w = next_w;
    }
    if handle.contains('s') {
        h = (h + dy).max(min_size);
    }
    if handle.contains('n') {
        let next_h = (h - dy).max(min_size);
        y += h - next_h;
        h = next_h;
    }
    InkBoundsF { x, y, w, h }
}

pub(crate) fn ink_snap_coordinate(v: f64, spacing: f64) -> f64 {
    if spacing <= 0.0 {
        v
    } else {
        (v / spacing).round() * spacing
    }
}

fn ink_snap_point(x: f64, y: f64, spacing: f64) -> (f64, f64) {
    (ink_snap_coordinate(x, spacing), ink_snap_coordinate(y, spacing))
}

pub fn ink_maybe_snap_fields(snap_enabled: Option<bool>, snap_grid_spacing: Option<f64>, x: f64, y: f64) -> (f64, f64) {
    if snap_enabled.unwrap_or(false) {
        ink_snap_point(x, y, snap_grid_spacing.unwrap_or(8.0))
    } else {
        (x, y)
    }
}

pub(super) fn ink_item_with_position(block: &Value, x: f64, y: f64) -> Value {
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(x));
        obj.insert("y".into(), json!(y));
    }
    cloned
}

pub fn create_ink_item(kind: &str, x: f64, y: f64) -> Value {
    let id = create_ink_host_id(kind);
    match kind {
        "image" => json!({
            "id": id, "name": "Image", "kind": "image", "x": x, "y": y, "width": 240.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false, "imageKey": "placeholder",
        }),
        "table" => json!({
            "id": id, "name": "Table", "kind": "table", "x": x, "y": y, "width": 320.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "columns": ["A", "B", "C"],
            "rows": [
                [{"content": ""}, {"content": ""}, {"content": ""}],
                [{"content": ""}, {"content": ""}, {"content": ""}],
            ],
        }),
        "math" => json!({
            "id": id, "name": "Math", "kind": "math", "x": x, "y": y, "width": 200.0, "height": 80.0,
            "rotation": 0.0, "visible": true, "locked": false, "tex": "E = mc^2", "displayMode": true,
        }),
        "stroke" => json!({
            "id": id, "name": "Ink", "kind": "stroke", "x": x, "y": y, "width": 1.0, "height": 1.0,
            "rotation": 0.0, "visible": true, "locked": false, "points": [], "strokeWidth": 3.0, "color": [0.0, 0.0, 0.0, 1.0],
        }),
        "group" => json!({
            "id": id, "name": "Group", "kind": "group", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false, "children": [],
        }),
        _ => json!({
            "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "paragraphs": [{"runs": [{"text": ""}]}], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
        }),
    }
}

fn point_segment_distance(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    if dx == 0.0 && dy == 0.0 {
        return ((px - x1).powi(2) + (py - y1).powi(2)).sqrt();
    }
    let t = (((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    ((px - (x1 + t * dx)).powi(2) + (py - (y1 + t * dy)).powi(2)).sqrt()
}

pub(crate) fn ink_points(block: &Value) -> Vec<(f64, f64)> {
    let bx = ink_item_num(block, "x");
    let by = ink_item_num(block, "y");
    block
        .get("points")
        .and_then(Value::as_array)
        .map(|points| {
            points
                .iter()
                .map(|p| {
                    let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    (bx + px, by + py)
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn ink_hits_point(block: &Value, x: f64, y: f64, threshold: f64) -> bool {
    let points = ink_points(block);
    let stroke_width = ink_item_num(block, "strokeWidth");
    if points.len() < 2 {
        return points.first().map(|p| ((x - p.0).powi(2) + (y - p.1).powi(2)).sqrt() <= threshold).unwrap_or(false);
    }
    points.windows(2).any(|w| point_segment_distance(x, y, w[0].0, w[0].1, w[1].0, w[1].1) <= threshold + stroke_width / 2.0)
}

pub fn erase_ink_stroke_points_in_item(block: &Value, x: f64, y: f64, radius: f64) -> Vec<Value> {
    let bx = ink_item_num(block, "x");
    let by = ink_item_num(block, "y");
    let points = block.get("points").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut kept_indices = Vec::new();
    for (index, point) in points.iter().enumerate() {
        let px = bx + point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
        let py = by + point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
        if ((px - x).powi(2) + (py - y).powi(2)).sqrt() > radius {
            kept_indices.push(index);
        }
    }
    if kept_indices.len() == points.len() {
        return vec![block.clone()];
    }
    if kept_indices.is_empty() {
        return Vec::new();
    }
    let mut runs: Vec<Vec<Value>> = Vec::new();
    let mut current: Vec<Value> = vec![points[kept_indices[0]].clone()];
    for window in kept_indices.windows(2) {
        if window[1] - window[0] > 1 {
            if current.len() >= 2 {
                runs.push(current);
            }
            current = vec![points[window[1]].clone()];
        } else {
            current.push(points[window[1]].clone());
        }
    }
    if current.len() >= 2 {
        runs.push(current);
    }
    let name = ink_item_str(block, "name").to_string();
    runs.into_iter()
        .enumerate()
        .map(|(index, pts)| {
            let mut cloned = block.clone();
            if let Some(obj) = cloned.as_object_mut() {
                if index > 0 {
                    obj.insert("id".into(), json!(create_ink_host_id("stroke")));
                    obj.insert("name".into(), json!(format!("{name} fragment")));
                }
                obj.insert("points".into(), Value::Array(pts));
            }
            cloned
        })
        .collect()
}

pub fn ink_screen_to_world(camera: InkCameraF, inner: Rect, sx: f32, sy: f32) -> (f64, f64) {
    let lx = (sx - inner.x) as f64;
    let ly = (sy - inner.y) as f64;
    ((lx - camera.x) / camera.zoom, (ly - camera.y) / camera.zoom)
}

fn ink_table_cell_at_point(block: &Value, local_x: f64, local_y: f64) -> Option<(usize, usize)> {
    let columns = block.get("columns")?.as_array()?;
    let rows = block.get("rows")?.as_array()?;
    if columns.is_empty() || rows.is_empty() {
        return None;
    }
    let bounds = ink_item_bounds(block);
    let row_height = bounds.h / (rows.len() + 1) as f64;
    let col_width = bounds.w / columns.len() as f64;
    let row = (local_y / row_height).floor() as isize - 1;
    let col = (local_x / col_width).floor() as isize;
    (row >= 0 && row < rows.len() as isize && col >= 0 && col < columns.len() as isize).then_some((row as usize, col as usize))
}

pub(crate) fn ink_edit_for_point(block: &Value, world_x: f64, world_y: f64) -> Option<(String, InkEditState)> {
    let id = ink_item_id(block);
    if id.is_empty() || ink_item_locked(block) {
        return None;
    }
    match ink_item_kind(block) {
        "text" => Some((format!("text:{id}"), InkEditState { block: block.clone(), kind: InkEditKind::Text, draft: ink_text_plain(block), replace_on_input: true, screen_rect: Rect::new(0.0, 0.0, 0.0, 0.0) })),
        "table" => {
            let bounds = ink_item_bounds(block);
            let (row, col) = ink_table_cell_at_point(block, world_x - bounds.x, world_y - bounds.y)?;
            let draft = block.get("rows")?.get(row)?.get(col)?.get("content").and_then(Value::as_str).unwrap_or_default().to_owned();
            Some((format!("table:{id}:{row}:{col}"), InkEditState { block: block.clone(), kind: InkEditKind::Table { row, col }, draft, replace_on_input: true, screen_rect: Rect::new(0.0, 0.0, 0.0, 0.0) }))
        }
        _ => None,
    }
}

pub(crate) fn ink_edit_control_id(surface_id: &str, edit: &InkEditState) -> String {
    let block_id = ink_item_id(&edit.block);
    match edit.kind {
        InkEditKind::Text => format!("{surface_id}.ink.text.{block_id}.input"),
        InkEditKind::Table { row, col } => format!("{surface_id}.ink.table.{block_id}.row.{row}.column.{col}.input"),
    }
}

pub(super) fn activate_ink_edit(surface_id: &str, edit: InkEditState) {
    mutate_scene_state(surface_id, |state| {
        state.pointer_was_down = false;
        state.drag = None;
        state.ink_edit = Some(edit);
    });
}

pub(crate) fn ink_edit_screen_rect(edit: &InkEditState, camera: InkCameraF, inner: Rect) -> Rect {
    let bounds = ink_item_bounds(&edit.block);
    let (x, y) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    match edit.kind {
        InkEditKind::Text => Rect::new(x, y, (bounds.w * camera.zoom) as f32, (bounds.h * camera.zoom) as f32),
        InkEditKind::Table { row, col } => {
            let columns = edit.block.get("columns").and_then(Value::as_array).map(Vec::len).unwrap_or(1).max(1);
            let rows = edit.block.get("rows").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
            let col_width = bounds.w / columns as f64;
            let row_height = bounds.h / (rows + 1) as f64;
            Rect::new(x + (col as f64 * col_width * camera.zoom) as f32, y + ((row + 1) as f64 * row_height * camera.zoom) as f32, (col_width * camera.zoom) as f32, (row_height * camera.zoom) as f32)
        }
    }
}

pub(super) fn ink_double_click_edit(surface_id: &str, block: &Value, camera: InkCameraF, inner: Rect, x: f32, y: f32) -> Option<InkEditState> {
    let (world_x, world_y) = ink_screen_to_world(camera, inner, x, y);
    let (target, mut edit) = ink_edit_for_point(block, world_x, world_y)?;
    let now = now_ms();
    let prior = scene_state(surface_id);
    let repeat = prior.last_click_target.as_deref() == Some(target.as_str()) && now - prior.last_click_ms < 400.0;
    mutate_scene_state(surface_id, |state| {
        state.last_click_target = Some(target);
        state.last_click_ms = now;
    });
    if !repeat {
        return None;
    }
    edit.screen_rect = ink_edit_screen_rect(&edit, camera, inner);
    Some(edit)
}
//#endregion InkCanvasModel
//#endregion InkCanvas
