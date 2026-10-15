//! ✒️ Ink canvas model and interaction state.

use super::*;

//#region InkCanvas
// 📝️ Direct DrawList painting for ink-canvas, ported from ink-canvas-host.tsx (framework/renderer/react).

//#region InkCanvasModel
static INK_HOST_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn create_ink_host_id(prefix: &str) -> String {
    let next = INK_HOST_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    format!("{prefix}-host-{next}")
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct InkCameraF {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) zoom: f64,
}

impl From<InkCameraJson> for InkCameraF {
    fn from(camera: InkCameraJson) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InkCameraJson {
    #[serde(default)]
    pub(crate) x: f64,
    #[serde(default)]
    pub(crate) y: f64,
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
pub(crate) struct InkDocumentJson {
    pub(crate) schema: String,
    pub(crate) id: String,
    pub(crate) camera: InkCameraJson,
    pub(crate) blocks: Vec<Value>,
    pub(crate) active_utility: Option<String>,
    pub(crate) snap_enabled: Option<bool>,
    pub(crate) snap_grid_spacing: Option<f64>,
    pub(crate) eraser_radius: Option<f64>,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct InkBoundsF {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) w: f64,
    pub(crate) h: f64,
}

impl InkBoundsF {
    pub(crate) fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    pub(crate) fn intersects(&self, other: &InkBoundsF) -> bool {
        self.x < other.x + other.w && self.x + self.w > other.x && self.y < other.y + other.h && self.y + self.h > other.y
    }
}

pub(crate) fn ink_item_str<'a>(block: &'a Value, key: &str) -> &'a str {
    block.get(key).and_then(Value::as_str).unwrap_or("")
}

pub(crate) fn ink_item_id(block: &Value) -> &str {
    ink_item_str(block, "id")
}

pub(crate) fn ink_item_kind(block: &Value) -> &str {
    ink_item_str(block, "kind")
}

pub(crate) fn ink_item_locked(block: &Value) -> bool {
    block.get("locked").and_then(Value::as_bool).unwrap_or(false)
}

pub(crate) fn ink_item_num(block: &Value, key: &str) -> f64 {
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

pub(crate) fn scale_ink_item(block: &Value, from: InkBoundsF, to: InkBoundsF) -> Value {
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

pub(crate) fn ink_resize_bounds(from: InkBoundsF, handle: &str, dx: f64, dy: f64, min_size: f64) -> InkBoundsF {
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

pub(crate) fn ink_maybe_snap_fields(snap_enabled: Option<bool>, snap_grid_spacing: Option<f64>, x: f64, y: f64) -> (f64, f64) {
    if snap_enabled.unwrap_or(false) {
        ink_snap_point(x, y, snap_grid_spacing.unwrap_or(8.0))
    } else {
        (x, y)
    }
}

fn ink_item_with_position(block: &Value, x: f64, y: f64) -> Value {
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(x));
        obj.insert("y".into(), json!(y));
    }
    cloned
}

pub(crate) fn create_ink_item(kind: &str, x: f64, y: f64) -> Value {
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

pub(crate) fn ink_hits_point(block: &Value, x: f64, y: f64, threshold: f64) -> bool {
    let points = ink_points(block);
    let stroke_width = ink_item_num(block, "strokeWidth");
    if points.len() < 2 {
        return points.first().map(|p| ((x - p.0).powi(2) + (y - p.1).powi(2)).sqrt() <= threshold).unwrap_or(false);
    }
    points.windows(2).any(|w| point_segment_distance(x, y, w[0].0, w[0].1, w[1].0, w[1].1) <= threshold + stroke_width / 2.0)
}

pub(crate) fn erase_ink_stroke_points_in_item(block: &Value, x: f64, y: f64, radius: f64) -> Vec<Value> {
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

pub(crate) fn ink_screen_to_world(camera: InkCameraF, inner: Rect, sx: f32, sy: f32) -> (f64, f64) {
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

fn activate_ink_edit(surface_id: &str, edit: InkEditState) {
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

fn ink_double_click_edit(surface_id: &str, block: &Value, camera: InkCameraF, inner: Rect, x: f32, y: f32) -> Option<InkEditState> {
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

//#region InkCanvasState
pub(crate) const INK_INTERACTION_DOCUMENT_BYTE_CAPACITY: usize = 16 * 1024;
pub(crate) const INK_INTERACTION_ITEM_CAPACITY: usize = 256;
const INK_SELECTION_ITEM_CAPACITY: usize = ui_wgpu::wgpu::action::ACTION_NODE_CAPACITY - 2;
pub(crate) const INK_EVENT_JSON_BYTE_CAPACITY: usize = ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY;

pub(crate) struct InkInteractionDocument {
    pub(crate) source: String,
    pub(crate) spans: Box<[Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY]>,
    pub(crate) span_len: usize,
    pub(crate) schema: String,
    pub(crate) id: String,
    pub(crate) camera: InkCameraJson,
    pub(crate) active_utility: Option<String>,
    pub(crate) snap_enabled: Option<bool>,
    pub(crate) snap_grid_spacing: Option<f64>,
    pub(crate) eraser_radius: Option<f64>,
}

impl InkInteractionDocument {
    pub(crate) fn block(&self, index: usize) -> Result<Option<Value>, ui_wgpu::wgpu::BoundedActionFault> {
        let Some((start, end)) = self.spans.get(index).and_then(|span| *span) else {
            return Ok(None);
        };
        serde_json::from_slice(&self.source.as_bytes()[usize::from(start)..usize::from(end)]).map(Some).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }
}

fn ink_skip_ws(bytes: &[u8], mut index: usize) -> usize {
    while bytes.get(index).is_some_and(u8::is_ascii_whitespace) {
        index += 1;
    }
    index
}

fn ink_scan_string(bytes: &[u8], start: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    if bytes.get(start) != Some(&b'"') {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    let mut index = start + 1;
    while let Some(byte) = bytes.get(index) {
        match byte {
            b'"' => return Ok(index + 1),
            b'\\' => index = index.checked_add(2).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?,
            _ => index += 1,
        }
    }
    Err(ui_wgpu::wgpu::BoundedActionFault::Structure)
}

fn ink_skip_value(bytes: &[u8], start: usize, depth: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    if depth > ui_wgpu::wgpu::action::ACTION_DEPTH_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::DepthCredits);
    }
    let mut index = ink_skip_ws(bytes, start);
    match bytes.get(index) {
        Some(b'"') => ink_scan_string(bytes, index),
        Some(b'[') => {
            index += 1;
            loop {
                index = ink_skip_ws(bytes, index);
                if bytes.get(index) == Some(&b']') {
                    return Ok(index + 1);
                }
                index = ink_skip_value(bytes, index, depth + 1)?;
                index = ink_skip_ws(bytes, index);
                match bytes.get(index) {
                    Some(b',') => index += 1,
                    Some(b']') => return Ok(index + 1),
                    _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
                }
            }
        }
        Some(b'{') => {
            index += 1;
            loop {
                index = ink_skip_ws(bytes, index);
                if bytes.get(index) == Some(&b'}') {
                    return Ok(index + 1);
                }
                index = ink_scan_string(bytes, index)?;
                index = ink_skip_ws(bytes, index);
                if bytes.get(index) != Some(&b':') {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
                }
                index = ink_skip_value(bytes, index + 1, depth + 1)?;
                index = ink_skip_ws(bytes, index);
                match bytes.get(index) {
                    Some(b',') => index += 1,
                    Some(b'}') => return Ok(index + 1),
                    _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
                }
            }
        }
        Some(_) => {
            while bytes.get(index).is_some_and(|byte| !byte.is_ascii_whitespace() && !matches!(*byte, b',' | b']' | b'}')) {
                index += 1;
            }
            Ok(index)
        }
        None => Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
    }
}

fn ink_collect_block_array(bytes: &[u8], start: usize, spans: &mut [Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY], span_len: &mut usize, depth: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    if depth > ui_wgpu::wgpu::action::ACTION_DEPTH_CAPACITY || bytes.get(start) != Some(&b'[') {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    let mut index = start + 1;
    loop {
        index = ink_skip_ws(bytes, index);
        if bytes.get(index) == Some(&b']') {
            return Ok(index + 1);
        }
        if *span_len == spans.len() {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let slot = *span_len;
        *span_len += 1;
        let end = if bytes.get(index) == Some(&b'{') { ink_collect_block_object(bytes, index, spans, span_len, depth + 1)? } else { ink_skip_value(bytes, index, depth + 1)? };
        spans[slot] = Some((u16::try_from(index).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?, u16::try_from(end).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?));
        index = ink_skip_ws(bytes, end);
        match bytes.get(index) {
            Some(b',') => index += 1,
            Some(b']') => return Ok(index + 1),
            _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
        }
    }
}

fn ink_collect_block_object(bytes: &[u8], start: usize, spans: &mut [Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY], span_len: &mut usize, depth: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    let mut index = start + 1;
    let mut is_group = false;
    let mut children = None;
    let end = loop {
        index = ink_skip_ws(bytes, index);
        if bytes.get(index) == Some(&b'}') {
            break index + 1;
        }
        let key_start = index + 1;
        let key_end_quote = ink_scan_string(bytes, index)? - 1;
        let key = bytes.get(key_start..key_end_quote).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        index = ink_skip_ws(bytes, key_end_quote + 1);
        if bytes.get(index) != Some(&b':') {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        index = ink_skip_ws(bytes, index + 1);
        let value_start = index;
        index = ink_skip_value(bytes, value_start, depth + 1)?;
        if key == b"kind" {
            is_group = serde_json::from_slice::<String>(&bytes[value_start..index]).is_ok_and(|kind| kind == "group");
        } else if key == b"children" && bytes.get(value_start) == Some(&b'[') {
            children = Some((value_start, index));
        }
        index = ink_skip_ws(bytes, index);
        match bytes.get(index) {
            Some(b',') => index += 1,
            Some(b'}') => break index + 1,
            _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
        }
    };
    if is_group {
        if let Some((children_start, children_end)) = children {
            let scanned_end = ink_collect_block_array(bytes, children_start, spans, span_len, depth + 1)?;
            if scanned_end != children_end {
                return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
            }
        }
    }
    Ok(end)
}

pub(crate) fn collect_ink_document_block_spans(bytes: &[u8], spans: &mut [Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY], span_len: &mut usize) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let mut index = ink_skip_ws(bytes, 0);
    if bytes.get(index) != Some(&b'{') {
        return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
    }
    index += 1;
    loop {
        index = ink_skip_ws(bytes, index);
        if bytes.get(index) == Some(&b'}') {
            return Ok(());
        }
        let key_start = index + 1;
        let key_end_quote = ink_scan_string(bytes, index)? - 1;
        let blocks = bytes.get(key_start..key_end_quote) == Some(b"blocks");
        index = ink_skip_ws(bytes, key_end_quote + 1);
        if bytes.get(index) != Some(&b':') {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        index = ink_skip_ws(bytes, index + 1);
        index = if blocks { ink_collect_block_array(bytes, index, spans, span_len, 0)? } else { ink_skip_value(bytes, index, 0)? };
        index = ink_skip_ws(bytes, index);
        match bytes.get(index) {
            Some(b',') => index += 1,
            Some(b'}') => return Ok(()),
            _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
        }
    }
}

pub(crate) struct InkEventJsonPages {
    pub(crate) bytes: Box<[u8; INK_EVENT_JSON_BYTE_CAPACITY]>,
    pub(crate) len: usize,
    pub(crate) items: usize,
    sealed: bool,
}

pub(crate) struct InkRawPages {
    pub(crate) bytes: Box<[u8; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]>,
    pub(crate) spans: Box<[Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY]>,
    pub(crate) byte_len: usize,
    pub(crate) head: usize,
    pub(crate) len: usize,
}

struct InkOwnedRaw {
    pub(crate) bytes: Box<[u8; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]>,
    pub(crate) len: usize,
}

impl InkOwnedRaw {
    pub(crate) fn new(raw: &str) -> Result<Self, ui_wgpu::wgpu::BoundedActionFault> {
        if raw.len() > INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        let mut bytes = Box::new([0; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]);
        bytes[..raw.len()].copy_from_slice(raw.as_bytes());
        Ok(Self { bytes, len: raw.len() })
    }

    pub(crate) fn as_str(&self) -> Result<&str, ui_wgpu::wgpu::BoundedActionFault> {
        std::str::from_utf8(&self.bytes[..self.len]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }
}

impl Default for InkRawPages {
    fn default() -> Self {
        Self { bytes: Box::new([0; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]), spans: Box::new(std::array::from_fn(|_| None)), byte_len: 0, head: 0, len: 0 }
    }
}

impl InkRawPages {
    pub(crate) fn push(&mut self, raw: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.len == self.spans.len() {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let end = self.byte_len.checked_add(raw.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        if end > self.bytes.len() {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        self.bytes[self.byte_len..end].copy_from_slice(raw.as_bytes());
        let index = (self.head + self.len) % self.spans.len();
        self.spans[index] = Some((u16::try_from(self.byte_len).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?, u16::try_from(end).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?));
        self.byte_len = end;
        self.len += 1;
        Ok(())
    }

    pub(crate) fn front(&self) -> Result<Option<&str>, ui_wgpu::wgpu::BoundedActionFault> {
        let Some((start, end)) = self.spans[self.head] else {
            return Ok(None);
        };
        std::str::from_utf8(&self.bytes[usize::from(start)..usize::from(end)]).map(Some).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }

    pub(crate) fn pop_front(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.spans[self.head] = None;
        self.head = (self.head + 1) % self.spans.len();
        self.len -= 1;
        if self.len == 0 {
            self.head = 0;
            self.byte_len = 0;
        }
        true
    }
}

impl Default for InkEventJsonPages {
    fn default() -> Self {
        let mut bytes = Box::new([0; INK_EVENT_JSON_BYTE_CAPACITY]);
        bytes[0] = b'[';
        Self { bytes, len: 1, items: 0, sealed: false }
    }
}

impl std::io::Write for InkEventJsonPages {
    fn write(&mut self, source: &[u8]) -> std::io::Result<usize> {
        let end = self.len.checked_add(source.len()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::WriteZero, "ink event byte credits exhausted"))?;
        if end >= INK_EVENT_JSON_BYTE_CAPACITY {
            return Err(std::io::Error::new(std::io::ErrorKind::WriteZero, "ink event byte credits exhausted"));
        }
        self.bytes[self.len..end].copy_from_slice(source);
        self.len = end;
        Ok(source.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl InkEventJsonPages {
    pub(crate) fn push(&mut self, value: &Value) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.sealed || self.items == INK_INTERACTION_ITEM_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let checkpoint = self.len;
        let result = (|| {
            if self.items != 0 {
                self.write_all(b",").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            }
            serde_json::to_writer(&mut *self, value).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)
        })();
        if let Err(fault) = result {
            self.len = checkpoint;
            return Err(fault);
        }
        self.items += 1;
        Ok(())
    }

    fn push_add_block_raw(&mut self, block: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.sealed || self.items == INK_INTERACTION_ITEM_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let checkpoint = self.len;
        let result = (|| {
            if self.items != 0 {
                self.write_all(b",").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            }
            self.write_all(b"{\"operation\":\"addBlock\",\"block\":").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            self.write_all(block.as_bytes()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            self.write_all(b"}").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)
        })();
        if let Err(fault) = result {
            self.len = checkpoint;
            return Err(fault);
        }
        self.items += 1;
        Ok(())
    }

    pub(crate) fn seal(&mut self) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.sealed {
            return Ok(());
        }
        self.write_all(b"]").map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        self.sealed = true;
        Ok(())
    }

    pub(crate) fn as_str(&self) -> Result<&str, ui_wgpu::wgpu::BoundedActionFault> {
        if !self.sealed {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        std::str::from_utf8(&self.bytes[..self.len]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }
}

/// 📨️ One `inkApplyEvents` action: a `stream` tick into the plugin's open ink transaction, the `commit` that publishes it
/// as ONE edit, or (no phase) a one-shot — with the drag record of a block move instead of per-block events.
fn write_ink_events_action(
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    scene: &UiComponentSceneNode,
    events: &InkEventJsonPages,
    phase: Option<&str>,
    gesture: Option<&str>,
    select_id: Option<&str>,
    mutate: impl FnOnce(),
) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let events = events.as_str()?;
    let mut parts: Vec<&str> = vec![&scene.controller_id, "inkApplyEvents", "surfaceId", &scene.surface_id, "eventsJson", events];
    if let Some(phase) = phase {
        parts.extend(["phase", phase]);
    }
    if let Some(gesture) = gesture {
        parts.extend(["gestureJson", gesture]);
    }
    let base = ui_wgpu::wgpu::checked_action_string_bytes(&parts)?;
    let bytes = match select_id {
        Some(id) => base.checked_add(ui_wgpu::wgpu::checked_action_string_bytes(&["selectIds", id])?).filter(|bytes| *bytes <= ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?,
        None => base,
    };
    let mut reservation = input.reserve_action(&scene.controller_id, "inkApplyEvents", bytes)?;
    let streams = phase == Some("stream");
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.string(Some("eventsJson"), events)?;
    if let Some(phase) = phase {
        builder.string(Some("phase"), phase)?;
    }
    if let Some(gesture) = gesture {
        builder.string(Some("gestureJson"), gesture)?;
    }
    if let Some(id) = select_id {
        builder.begin_array(Some("selectIds"))?;
        builder.string(None, id)?;
        builder.end_container()?;
    }
    builder.end_container()?;
    reservation.publish_with(|| {
        mutate();
        if streams {
            mutate_scene_state(&scene.host_id, |state| state.ink_gesture_open = true);
        }
    })
}

/// 🧯️ The plugin half of an ink pointer cancel: a gesture that streamed into the plugin's open ink transaction is
/// aborted with ONE `inkApplyEvents` (`phase: "abort"`, `reason: "captureLost"`), so the plugin drops it with zero trace;
/// a press that never streamed publishes nothing. Answers whether an abort was published; runs before
/// [`ink_pointer_cancel_into`] clears the local drag.
pub(crate) fn ink_gesture_abort_into(controller_id: &str, surface_id: &str, host_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if !SCENE_STATE.with(|cell| cell.borrow().get(host_id).is_some_and(|state| state.ink_gesture_open)) {
        return Ok(false);
    }
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "inkApplyEvents", "surfaceId", surface_id, "eventsJson", "[]", "phase", "abort", "reason", "captureLost"])?;
    let mut reservation = input.reserve_action(controller_id, "inkApplyEvents", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), surface_id)?;
    builder.string(Some("eventsJson"), "[]")?;
    builder.string(Some("phase"), "abort")?;
    builder.string(Some("reason"), "captureLost")?;
    builder.end_container()?;
    reservation.publish_with(|| mutate_scene_state(host_id, |state| state.ink_gesture_open = false))?;
    Ok(true)
}

fn ink_interaction_domain(scene: &UiComponentSceneNode) -> Option<(&str, &str)> {
    let ink = scene.ink_canvas.as_ref()?;
    let domain = ink.interaction_domain.as_ref()?;
    (!domain.id.is_empty() && !domain.granularity_id.is_empty()).then_some((domain.id.as_str(), domain.granularity_id.as_str()))
}

fn ink_interaction_id(block: &Value) -> Option<&str> {
    block.get("interactionId").and_then(Value::as_str).filter(|id| !id.is_empty())
}

fn write_ink_interaction_selection(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, ids: &[String], merge: &str, method: &str, mutate: impl FnOnce()) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let Some((domain_id, granularity)) = ink_interaction_domain(scene) else {
        mutate();
        return Ok(());
    };
    if ids.len() > INK_SELECTION_ITEM_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    }
    let targets = serde_json::to_string(&ids.iter().map(|id| json!({ "granularity": granularity, "id": id })).collect::<Vec<_>>()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "interactionSelect", "surfaceId", &scene.surface_id, "domainId", domain_id, "targets", &targets, "merge", merge, "method", method])?;
    let mut reservation = input.reserve_action(&scene.controller_id, "interactionSelect", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.string(Some("domainId"), domain_id)?;
    builder.string(Some("targets"), &targets)?;
    builder.string(Some("merge"), merge)?;
    builder.string(Some("method"), method)?;
    builder.end_container()?;
    reservation.publish_with(mutate)
}

fn write_ink_interaction_hover(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, id: Option<&str>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let Some((domain_id, granularity)) = ink_interaction_domain(scene) else { return Ok(()) };
    let targets = serde_json::to_string(&id.map(|id| vec![json!({ "granularity": granularity, "id": id })]).unwrap_or_default()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "interactionHover", "surfaceId", &scene.surface_id, "domainId", domain_id, "channel", "pointer", "targets", &targets])?;
    let mut reservation = input.reserve_action(&scene.controller_id, "interactionHover", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.string(Some("domainId"), domain_id)?;
    builder.string(Some("channel"), "pointer")?;
    builder.string(Some("targets"), &targets)?;
    builder.end_container()?;
    reservation.publish()
}

pub(crate) fn clear_ink_pointer_state(surface_id: &str) {
    mutate_scene_state(surface_id, |state| {
        state.drag = None;
        state.pointer_was_down = false;
        state.ink_gesture_open = false;
        state.ink_marquee_points.clear();
    });
}

fn ink_edit_updated_block(edit: &InkEditState) -> Result<Value, ui_wgpu::wgpu::BoundedActionFault> {
    let mut block = edit.block.clone();
    match edit.kind {
        InkEditKind::Text => {
            let Some(object) = block.as_object_mut() else { return Err(ui_wgpu::wgpu::BoundedActionFault::Structure) };
            object.insert("paragraphs".into(), json!([{ "runs": [{ "text": edit.draft }] }]));
        }
        InkEditKind::Table { row, col } => {
            let Some(rows) = block.get_mut("rows").and_then(Value::as_array_mut) else { return Err(ui_wgpu::wgpu::BoundedActionFault::Structure) };
            let Some(cells) = rows.get_mut(row).and_then(Value::as_array_mut) else { return Err(ui_wgpu::wgpu::BoundedActionFault::Structure) };
            let Some(cell) = cells.get_mut(col).and_then(Value::as_object_mut) else { return Err(ui_wgpu::wgpu::BoundedActionFault::Structure) };
            cell.insert("content".into(), Value::String(edit.draft.clone()));
        }
    }
    Ok(block)
}

fn ink_edit_next_cell(edit: &InkEditState, updated: &Value) -> Option<InkEditState> {
    let InkEditKind::Table { row, col } = edit.kind else { return None };
    let columns = updated.get("columns")?.as_array()?.len();
    let rows = updated.get("rows")?.as_array()?;
    if columns == 0 {
        return None;
    }
    let (next_row, next_col) = if col + 1 < columns { (row, col + 1) } else { (row + 1, 0) };
    let draft = rows.get(next_row)?.get(next_col)?.get("content").and_then(Value::as_str).unwrap_or_default().to_owned();
    let mut screen_rect = edit.screen_rect;
    if next_row == row {
        screen_rect.x += screen_rect.w;
    } else {
        screen_rect.x -= col as f32 * screen_rect.w;
        screen_rect.y += screen_rect.h;
    }
    Some(InkEditState { block: updated.clone(), kind: InkEditKind::Table { row: next_row, col: next_col }, draft, replace_on_input: true, screen_rect })
}

fn ink_publish_edit(scene: &UiComponentSceneNode, edit: InkEditState, advance: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let updated = ink_edit_updated_block(&edit)?;
    let block_id = ink_item_id(&updated).to_owned();
    let next = advance.then(|| ink_edit_next_cell(&edit, &updated)).flatten();
    let next_focus = next.as_ref().map(|next| (ink_edit_control_id(&scene.host_id, next), next.draft.clone()));
    let state_block = updated.clone();
    let mut events = InkEventJsonPages::default();
    events.push(&json!({ "operation": "updateBlock", "blockId": block_id.clone(), "block": updated }))?;
    events.seal()?;
    write_ink_events_action(input, scene, &events, None, None, None, || {
        mutate_scene_state(&scene.host_id, |state| {
            state.ink_overrides.insert(block_id, state_block);
            state.ink_edit = next;
        });
    })?;
    match next_focus {
        Some((control_id, draft)) => input.focus_input_owned(control_id, draft),
        None => input.blur_input(),
    }
    Ok(())
}

pub(crate) fn ink_edit_key_into(scene: &UiComponentSceneNode, key: &KeyAction, modifiers: &ui_wgpu::wgpu::PointerModifiers, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(mut edit) = scene_state(&scene.host_id).ink_edit else { return Ok(false) };
    if modifiers.ctrl_or_meta() {
        return Ok(false);
    }
    match key {
        KeyAction::Escape => {
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = None);
            input.blur_input();
        }
        KeyAction::Enter if matches!(edit.kind, InkEditKind::Table { .. }) => ink_publish_edit(scene, edit, true, input)?,
        KeyAction::Tab if matches!(edit.kind, InkEditKind::Table { .. }) => ink_publish_edit(scene, edit, true, input)?,
        KeyAction::Tab => ink_publish_edit(scene, edit, false, input)?,
        KeyAction::Enter => {
            if edit.replace_on_input {
                edit.draft.clear();
                edit.replace_on_input = false;
            }
            edit.draft.push('\n');
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
        }
        KeyAction::Backspace | KeyAction::Delete => {
            if edit.replace_on_input {
                edit.draft.clear();
                edit.replace_on_input = false;
            } else {
                edit.draft.pop();
            }
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
        }
        KeyAction::Char(text) => {
            if edit.replace_on_input {
                edit.draft.clear();
                edit.replace_on_input = false;
            }
            edit.draft.push_str(text);
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
        }
        KeyAction::Space(true) => {
            if edit.replace_on_input {
                edit.draft.clear();
                edit.replace_on_input = false;
            }
            edit.draft.push(' ');
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
        }
        _ => {}
    }
    Ok(true)
}

pub(crate) fn ink_edit_blur_into(scene: &UiComponentSceneNode, x: f32, y: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(edit) = scene_state(&scene.host_id).ink_edit else { return Ok(false) };
    if edit.screen_rect.contains(x, y) {
        return Ok(false);
    }
    ink_publish_edit(scene, edit, false, input)?;
    Ok(true)
}

pub(crate) fn cancel_ink_edit(surface_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    mutate_scene_state(surface_id, |state| state.ink_edit = None);
    if input.focused_id.as_deref().is_some_and(|id| id.starts_with(&format!("{surface_id}.ink."))) {
        input.blur_input();
    }
}

pub(crate) fn ink_edit_active(surface_id: &str) -> bool {
    scene_state(surface_id).ink_edit.is_some()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InkClipboardContentKind {
    Text,
    ImageDataUrl,
}

fn find_ink_item<'a>(blocks: &'a [Value], id: &str) -> Option<&'a Value> {
    for block in blocks {
        if ink_item_id(block) == id {
            return Some(block);
        }
        if ink_item_kind(block) == "group" {
            if let Some(found) = block.get("children").and_then(Value::as_array).and_then(|children| find_ink_item(children, id)) {
                return Some(found);
            }
        }
    }
    None
}

pub(crate) fn ink_clipboard_copy_text(scene: &UiComponentSceneNode) -> Result<Option<String>, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(ink) = scene.ink_canvas.as_ref() else { return Ok(None) };
    if ink.document_json.len() >= INK_INTERACTION_DOCUMENT_BYTE_CAPACITY || ink.selection_json.len() > INK_EVENT_JSON_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
    }
    let document: InkDocumentJson = serde_json::from_str(&ink.document_json).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let selection: Vec<String> = serde_json::from_str(&ink.selection_json).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    if selection.len() > INK_SELECTION_ITEM_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    }
    let blocks: Vec<Value> = selection.iter().filter_map(|id| find_ink_item(&document.blocks, id).cloned()).collect();
    if blocks.is_empty() {
        return Ok(None);
    }
    let text = serde_json::to_string(&json!({ "schema": "ink.clipboard", "blocks": blocks })).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    if text.len() >= INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
    }
    Ok(Some(text))
}

pub(crate) fn ink_clipboard_target(scene: &UiComponentSceneNode, inner: Rect) -> Result<(f64, f64), ui_wgpu::wgpu::BoundedActionFault> {
    let document = checked_ink_document(scene)?.ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let camera = ink_camera_checked(scene)?;
    let (x, y) = ink_screen_to_world(camera, inner, inner.x + inner.w / 2.0, inner.y + inner.h / 2.0);
    Ok(ink_maybe_snap_fields(document.snap_enabled, document.snap_grid_spacing, x, y))
}

fn reidentify_ink_clipboard_item(block: &Value) -> Result<Value, ui_wgpu::wgpu::BoundedActionFault> {
    let object = block.as_object().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let kind = object.get("kind").and_then(Value::as_str).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    if kind.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
    }
    let mut clone = block.clone();
    let clone_object = clone.as_object_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    clone_object.insert("id".into(), Value::String(create_ink_host_id(kind)));
    if kind == "group" {
        let children = object.get("children").and_then(Value::as_array).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        if children.len() > INK_INTERACTION_ITEM_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let children = children.iter().map(reidentify_ink_clipboard_item).collect::<Result<Vec<_>, _>>()?;
        clone_object.insert("children".into(), Value::Array(children));
    }
    Ok(clone)
}

fn write_ink_clipboard_events_action(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, events: &InkEventJsonPages, selected_ids: &[String]) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if selected_ids.len() > INK_SELECTION_ITEM_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    }
    let events_json = events.as_str()?;
    let mut bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "inkApplyEvents", "surfaceId", &scene.surface_id, "eventsJson", events_json, "selectIds"])?;
    for id in selected_ids {
        bytes = bytes.checked_add(ui_wgpu::wgpu::checked_action_string_bytes(&[id])?).filter(|value| *value <= ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
    }
    let mut reservation = input.reserve_action(&scene.controller_id, "inkApplyEvents", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.string(Some("eventsJson"), events_json)?;
    builder.begin_array(Some("selectIds"))?;
    for id in selected_ids {
        builder.string(None, id)?;
    }
    builder.end_container()?;
    builder.end_container()?;
    reservation.publish()
}

fn ink_clipboard_image_events(content: &str, target: (f64, f64)) -> Result<(InkEventJsonPages, Vec<String>), ui_wgpu::wgpu::BoundedActionFault> {
    let (mime, data) = if content.trim_start().starts_with("<svg") {
        ("image/svg+xml", content.trim())
    } else {
        let Some(header) = content.strip_prefix("data:").and_then(|data| data.split_once(',').map(|parts| parts.0)) else {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        };
        let mime = header.split(';').next().unwrap_or_default();
        if !mime.starts_with("image/") {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        (mime, content)
    };
    if data.len() > INK_EVENT_JSON_BYTE_CAPACITY / 2 {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
    }
    let key = format!("asset-{}", create_ink_host_id("image"));
    let mut block = create_ink_item("image", target.0 - 120.0, target.1 - 80.0);
    let id = ink_item_id(&block).to_owned();
    block.as_object_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?.insert("imageKey".into(), Value::String(key.clone()));
    let mut events = InkEventJsonPages::default();
    events.push(&json!({ "operation": "putAsset", "key": key, "asset": { "mime": mime, "data": data } }))?;
    events.push(&json!({ "operation": "addBlock", "block": block }))?;
    events.seal()?;
    Ok((events, vec![id]))
}

pub(crate) fn ink_clipboard_paste_into(scene: &UiComponentSceneNode, target: (f64, f64), kind: InkClipboardContentKind, content: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if content.is_empty() || content.len() >= INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
        return if content.is_empty() { Ok(false) } else { Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits) };
    }
    let (mut events, selected_ids) = if kind == InkClipboardContentKind::ImageDataUrl || content.trim_start().starts_with("<svg") {
        ink_clipboard_image_events(content, target)?
    } else if let Ok(payload) = serde_json::from_str::<Value>(content) {
        if payload.get("schema").and_then(Value::as_str) == Some("ink.clipboard") {
            let blocks = payload.get("blocks").and_then(Value::as_array).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            if blocks.len() > INK_INTERACTION_ITEM_CAPACITY {
                return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
            }
            let mut events = InkEventJsonPages::default();
            let mut selected_ids = Vec::with_capacity(blocks.len());
            for block in blocks {
                let mut block = reidentify_ink_clipboard_item(block)?;
                let x = ink_item_num(&block, "x") + target.0;
                let y = ink_item_num(&block, "y") + target.1;
                block = ink_item_with_position(&block, x, y);
                let mut nodes = 0;
                validate_ink_value(&block, 0, &mut nodes)?;
                selected_ids.push(ink_item_id(&block).to_owned());
                events.push(&json!({ "operation": "addBlock", "block": block }))?;
            }
            if selected_ids.is_empty() {
                return Ok(false);
            }
            (events, selected_ids)
        } else {
            let trimmed = content.trim();
            if trimmed.is_empty() {
                return Ok(false);
            }
            let mut block = create_ink_item("text", target.0, target.1);
            let id = ink_item_id(&block).to_owned();
            let paragraphs = trimmed.lines().map(|line| json!({ "runs": [{ "text": line }] })).collect();
            block.as_object_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?.insert("paragraphs".into(), Value::Array(paragraphs));
            let mut events = InkEventJsonPages::default();
            events.push(&json!({ "operation": "addBlock", "block": block }))?;
            (events, vec![id])
        }
    } else {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Ok(false);
        }
        let mut block = create_ink_item("text", target.0, target.1);
        let id = ink_item_id(&block).to_owned();
        let paragraphs = trimmed.lines().map(|line| json!({ "runs": [{ "text": line }] })).collect();
        block.as_object_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?.insert("paragraphs".into(), Value::Array(paragraphs));
        let mut events = InkEventJsonPages::default();
        events.push(&json!({ "operation": "addBlock", "block": block }))?;
        (events, vec![id])
    };
    events.seal()?;
    write_ink_clipboard_events_action(input, scene, &events, &selected_ids)?;
    Ok(true)
}

pub(crate) fn ink_edit_paste_into(scene: &UiComponentSceneNode, text: &str) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if text.len() > INK_EVENT_JSON_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
    }
    let mut state = scene_state(&scene.host_id);
    let Some(mut edit) = state.ink_edit.take() else { return Ok(false) };
    if edit.replace_on_input {
        edit.draft.clear();
        edit.replace_on_input = false;
    }
    edit.draft.push_str(text);
    mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
    Ok(true)
}

pub(crate) fn ink_pointer_cancel_into(surface_id: &str) {
    mutate_scene_state(surface_id, |state| {
        state.drag = None;
        state.pointer_was_down = false;
        state.ink_gesture_open = false;
        state.ink_marquee_points.clear();
        state.ink_overrides.clear();
    });
}

#[cfg(test)]
pub(crate) fn ink_pointer_state_is_clear(surface_id: &str) -> bool {
    let state = scene_state(surface_id);
    !state.pointer_was_down && state.drag.is_none() && !state.ink_gesture_open && state.ink_marquee_points.is_empty() && state.ink_overrides.is_empty()
}

fn checked_ink_document(scene: &UiComponentSceneNode) -> Result<Option<InkInteractionDocument>, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(ink) = scene.ink_canvas.as_ref() else {
        return Ok(None);
    };
    if ink.document_json.len() > INK_INTERACTION_DOCUMENT_BYTE_CAPACITY || ink.selection_json.len() > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
    }
    let document = serde_json::from_str::<InkDocumentJson>(&ink.document_json).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let mut nodes = 0usize;
    for block in &document.blocks {
        validate_ink_value(block, 0, &mut nodes)?;
    }
    for (key, asset) in &document.assets {
        if key.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        validate_ink_value(asset, 0, &mut nodes)?;
    }
    if document.blocks.len() > INK_INTERACTION_ITEM_CAPACITY || document.assets.len() > INK_INTERACTION_ITEM_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    }
    let source = ink.document_json.clone();
    let mut spans = Box::new(std::array::from_fn(|_| None));
    let mut span_len = 0usize;
    collect_ink_document_block_spans(source.as_bytes(), &mut spans, &mut span_len)?;
    Ok(Some(InkInteractionDocument {
        source,
        spans,
        span_len,
        schema: document.schema,
        id: document.id,
        camera: document.camera,
        active_utility: document.active_utility,
        snap_enabled: document.snap_enabled,
        snap_grid_spacing: document.snap_grid_spacing,
        eraser_radius: document.eraser_radius,
    }))
}

pub(crate) fn validate_ink_value(value: &Value, depth: usize, nodes: &mut usize) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if depth > ui_wgpu::wgpu::action::ACTION_DEPTH_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::DepthCredits);
    }
    *nodes = nodes.checked_add(1).ok_or(ui_wgpu::wgpu::BoundedActionFault::NodeCredits)?;
    if *nodes > INK_INTERACTION_ITEM_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::NodeCredits);
    }
    match value {
        Value::Array(values) => {
            for value in values {
                validate_ink_value(value, depth + 1, nodes)?;
            }
        }
        Value::Object(entries) => {
            for (key, value) in entries {
                if key.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
                }
                validate_ink_value(value, depth + 1, nodes)?;
            }
        }
        Value::String(value) if value.len() > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY => return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits),
        _ => {}
    }
    Ok(())
}

fn ink_camera_checked(scene: &UiComponentSceneNode) -> Result<InkCameraF, ui_wgpu::wgpu::BoundedActionFault> {
    if let Some(camera) = SCENE_STATE.with(|cell| cell.borrow().get(&scene.host_id).and_then(|state| state.ink_camera)) {
        return Ok(InkCameraF { x: camera.0, y: camera.1, zoom: camera.2 });
    }
    Ok(checked_ink_document(scene)?.map(|document| InkCameraF::from(document.camera)).unwrap_or_default())
}

fn write_ink_camera_action(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, camera: InkCameraF, mutate: impl FnOnce()) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "setCamera", "surfaceId", &scene.surface_id, "camera", "x", "y", "zoom"])?;
    let mut reservation = input.reserve_action(&scene.controller_id, "setCamera", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.begin_object(Some("camera"))?;
    builder.number(Some("x"), camera.x)?;
    builder.number(Some("y"), camera.y)?;
    builder.number(Some("zoom"), camera.zoom)?;
    builder.end_container()?;
    builder.end_container()?;
    reservation.publish_with(mutate)
}

pub(crate) fn ink_wheel_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(ink) = scene.ink_canvas.as_ref() else {
        return Ok(false);
    };
    if ink.view_mode == "navigator" || !inner.contains(x, y) {
        return Ok(false);
    }
    let camera = ink_camera_checked(scene)?;
    let zoom_factor: f64 = if delta < 0.0 { 1.08 } else { 0.92 };
    let next_zoom = (camera.zoom * zoom_factor).clamp(0.1, 8.0);
    let (wx, wy) = ink_screen_to_world(camera, inner, x, y);
    let next = InkCameraF { x: (x - inner.x) as f64 - wx * next_zoom, y: (y - inner.y) as f64 - wy * next_zoom, zoom: next_zoom };
    write_ink_camera_action(input, scene, next, || {
        mutate_scene_state(&scene.host_id, |state| {
            state.ink_camera = Some((next.x, next.y, next.zoom));
        });
    })?;
    Ok(true)
}

#[derive(Clone, Copy)]
pub(crate) enum InkInteractionEvent {
    PointerDown { x: f32, y: f32, button: i16, shift: bool },
    PointerUp { x: f32, y: f32 },
    PointerMove { x: f32, y: f32 },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InkInteractionStage {
    Scan,
    Publish,
    Complete,
}

pub(crate) enum InkInteractionStep {
    Pending,
    Complete,
}

#[derive(Default)]
pub(crate) struct InkBlockCursor {
    pub(crate) index: usize,
}

impl InkBlockCursor {
    pub(crate) fn next(&mut self, document: &InkInteractionDocument) -> Result<Option<Value>, ui_wgpu::wgpu::BoundedActionFault> {
        let block = document.block(self.index)?;
        if block.is_some() {
            self.index += 1;
        }
        Ok(block)
    }
}

pub(crate) struct InkInteractionJob {
    pub(crate) generation: u64,
    surface_generation: u64,
    pub(crate) pointer_id: Option<ui_render::PointerId>,
    pub(crate) event: InkInteractionEvent,
    pub(crate) stage: InkInteractionStage,
    document: Option<InkInteractionDocument>,
    pub(crate) selected_ids: Vec<String>,
    result_ids: Vec<String>,
    result_bytes: usize,
    block_cursor: InkBlockCursor,
    hit_id: Option<String>,
    hit_interaction_id: Option<String>,
    hit_origin: Option<(f64, f64)>,
    hit_block: Option<Value>,
    opened_edit: bool,
    pub(crate) events: InkEventJsonPages,
    pub(crate) utility: String,
    pub(crate) camera: InkCameraF,
    pub(crate) drag: Option<SceneDragMode>,
    stroke_update: Option<(String, InkOwnedRaw)>,
    pending_fragments: InkRawPages,
    pending_remove_id: Option<String>,
}

fn checked_ink_drag(surface_id: &str) -> Result<Option<SceneDragMode>, ui_wgpu::wgpu::BoundedActionFault> {
    SCENE_STATE.with(|cell| {
        let states = cell.borrow();
        let Some(mode) = states.get(surface_id).and_then(|state| state.drag.as_ref()).map(|drag| &drag.mode) else {
            return Ok(None);
        };
        let mut bytes = 0usize;
        match mode {
            SceneDragMode::InkMove { origins, .. } => {
                if origins.len() > INK_INTERACTION_ITEM_CAPACITY {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                for id in origins.keys() {
                    bytes = bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
                }
            }
            SceneDragMode::InkResize { handle, selected_ids, .. } => {
                if selected_ids.len() > INK_INTERACTION_ITEM_CAPACITY {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                bytes = handle.len();
                for id in selected_ids {
                    bytes = bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
                }
            }
            SceneDragMode::InkStroke { block_id } => bytes = block_id.len(),
            SceneDragMode::InkEraser { mode } => bytes = mode.len(),
            _ => {}
        }
        if bytes > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        Ok(Some(mode.clone()))
    })
}

impl InkInteractionJob {
    pub(crate) fn new(generation: u64, surface_generation: u64, pointer_id: Option<ui_render::PointerId>, scene: &UiComponentSceneNode, event: InkInteractionEvent) -> Result<Option<Self>, ui_wgpu::wgpu::BoundedActionFault> {
        if generation == 0 || surface_generation == 0 {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        let Some(ink) = scene.ink_canvas.as_ref() else {
            return Ok(None);
        };
        if ink.view_mode == "navigator" || !ink.interactive {
            return Ok(None);
        }
        let Some(document) = checked_ink_document(scene)? else {
            return Ok(None);
        };
        let selected_ids: Vec<String> = serde_json::from_str(&ink.selection_json).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        if selected_ids.len() > INK_INTERACTION_ITEM_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let mut selected_bytes = 0usize;
        for id in &selected_ids {
            if id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
            }
            selected_bytes = selected_bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        }
        if selected_bytes > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        let camera = SCENE_STATE.with(|cell| cell.borrow().get(&scene.host_id).and_then(|state| state.ink_camera)).map(|(x, y, zoom)| InkCameraF { x, y, zoom }).unwrap_or_else(|| InkCameraF::from(document.camera.clone()));
        let drag = checked_ink_drag(&scene.host_id)?;
        let utility = document.active_utility.clone().unwrap_or_else(|| "selectDirect".to_owned());
        if utility.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        Ok(Some(Self {
            generation,
            surface_generation,
            pointer_id,
            event,
            stage: InkInteractionStage::Scan,
            document: Some(document),
            selected_ids,
            result_ids: Vec::with_capacity(INK_INTERACTION_ITEM_CAPACITY),
            result_bytes: 0,
            block_cursor: InkBlockCursor::default(),
            hit_id: None,
            hit_interaction_id: None,
            hit_origin: None,
            hit_block: None,
            opened_edit: false,
            events: InkEventJsonPages::default(),
            utility,
            camera,
            drag,
            stroke_update: None,
            pending_fragments: InkRawPages::default(),
            pending_remove_id: None,
        }))
    }

    pub(crate) fn step(&mut self, generation: u64, scene: &UiComponentSceneNode, inner: Rect, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<InkInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if generation != self.generation {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        if self.stage == InkInteractionStage::Complete {
            return Ok(InkInteractionStep::Complete);
        }
        if self.stage == InkInteractionStage::Scan {
            if self.scan_one(scene, inner)? {
                self.stage = InkInteractionStage::Publish;
            }
            return Ok(InkInteractionStep::Pending);
        }
        self.publish(scene, inner, input)?;
        self.stage = InkInteractionStage::Complete;
        Ok(InkInteractionStep::Complete)
    }

    pub(crate) fn opened_edit(&self) -> bool {
        self.opened_edit
    }

    pub(crate) fn matches_pointer_owner(&self, surface_generation: u64, pointer_id: ui_render::PointerId) -> bool {
        self.surface_generation == surface_generation && self.pointer_id == Some(pointer_id)
    }

    fn scan_one(&mut self, scene: &UiComponentSceneNode, inner: Rect) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        match self.event {
            InkInteractionEvent::PointerDown { x, y, button, .. } if button == 0 && (self.utility == "eraserStroke" || self.utility == "eraserPoint") => {
                if self.push_pending_eraser_event()? {
                    return Ok(false);
                }
                let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                let Some(block) = self.block_cursor.next(document)? else {
                    self.events.seal()?;
                    return Ok(true);
                };
                let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
                let prepared = Self::prepare_eraser_block(&block, world_x, world_y, document.eraser_radius.unwrap_or(12.0), self.utility == "eraserPoint")?;
                if let Some((remove_id, fragments)) = prepared {
                    self.pending_remove_id = Some(remove_id);
                    self.pending_fragments = fragments;
                }
                Ok(false)
            }
            InkInteractionEvent::PointerDown { x, y, button: 0, .. } if self.utility == "selectDirect" => self.scan_hit(inner, x, y),
            InkInteractionEvent::PointerMove { x, y } => {
                if self.drag.is_none() {
                    self.scan_hit(inner, x, y)
                } else if matches!(self.drag.as_ref(), Some(SceneDragMode::InkMove { .. } | SceneDragMode::InkResize { .. } | SceneDragMode::InkStroke { .. } | SceneDragMode::InkEraser { .. })) {
                    self.scan_drag_event(scene, inner, x, y, false)
                } else {
                    Ok(true)
                }
            }
            InkInteractionEvent::PointerUp { x, y } => {
                if matches!(self.drag.as_ref(), Some(SceneDragMode::InkMove { .. } | SceneDragMode::InkResize { .. } | SceneDragMode::InkStroke { .. })) {
                    self.scan_drag_event(scene, inner, x, y, true)
                } else if let Some((start_x, start_y)) = self.drag.as_ref().and_then(|drag| match drag {
                    SceneDragMode::InkMarqueeDrag { start_x, start_y } => Some((*start_x, *start_y)),
                    _ => None,
                }) {
                    self.scan_marquee(inner, x, y, start_x, start_y)
                } else if matches!(self.drag.as_ref(), Some(SceneDragMode::InkEraser { .. })) {
                    self.events.seal()?;
                    Ok(true)
                } else {
                    Ok(true)
                }
            }
            _ => Ok(true),
        }
    }

    fn push_pending_eraser_event(&mut self) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        if let Some(remove_id) = self.pending_remove_id.as_deref() {
            self.events.push(&json!({ "operation": "removeBlock", "blockId": remove_id }))?;
            self.pending_remove_id = None;
            return Ok(true);
        }
        if let Some(fragment) = self.pending_fragments.front()? {
            self.events.push_add_block_raw(fragment)?;
            self.pending_fragments.pop_front();
            return Ok(true);
        }
        Ok(false)
    }

    fn prepare_eraser_block(block: &Value, x: f64, y: f64, radius: f64, points_only: bool) -> Result<Option<(String, InkRawPages)>, ui_wgpu::wgpu::BoundedActionFault> {
        if ink_item_kind(block) != "stroke" || !ink_hits_point(block, x, y, radius) {
            return Ok(None);
        }
        let id = ink_item_id(block);
        if id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        if !points_only {
            return Ok(Some((id.to_owned(), InkRawPages::default())));
        }
        let fragments = erase_ink_stroke_points_in_item(block, x, y, radius);
        if fragments.len() == 1 && fragments[0] == *block {
            return Ok(None);
        }
        if fragments.len() > INK_INTERACTION_ITEM_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        for fragment in &fragments {
            let mut nodes = 0usize;
            validate_ink_value(fragment, 0, &mut nodes)?;
        }
        let mut encoded = InkRawPages::default();
        for fragment in fragments {
            let raw = serde_json::to_string(&fragment).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            if raw.len() > INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
                return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
            }
            encoded.push(&raw)?;
        }
        Ok(Some((id.to_owned(), encoded)))
    }

    /// 🤏️ A block move's drag record — the moved ids and the world offset from the press — which the plugin yields as ONE
    /// relative `drag-blocks` leaf.
    fn ink_move_gesture(&self, x: f32, y: f32) -> Option<String> {
        let Some(SceneDragMode::InkMove { origins, start_x, start_y }) = self.drag.as_ref() else { return None };
        let zoom = self.camera.zoom.max(0.0001);
        Some(json!({ "kind": "drag", "ids": origins.keys().collect::<Vec<_>>(), "dx": (x - *start_x) as f64 / zoom, "dy": (y - *start_y) as f64 / zoom }).to_string())
    }

    fn scan_drag_event(&mut self, scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, commit: bool) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        if matches!(self.drag.as_ref(), Some(SceneDragMode::InkMove { .. })) {
            self.events.seal()?;
            return Ok(true);
        }
        if matches!(self.drag.as_ref(), Some(SceneDragMode::InkEraser { .. })) {
            if self.push_pending_eraser_event()? {
                return Ok(false);
            }
            let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let Some(block) = self.block_cursor.next(document)? else {
                self.events.seal()?;
                return Ok(true);
            };
            let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
            let points_only = matches!(self.drag.as_ref(), Some(SceneDragMode::InkEraser { mode }) if mode == "eraserPoint");
            let prepared = Self::prepare_eraser_block(&block, world_x, world_y, document.eraser_radius.unwrap_or(12.0), points_only)?;
            if let Some((remove_id, fragments)) = prepared {
                self.pending_remove_id = Some(remove_id);
                self.pending_fragments = fragments;
            }
            return Ok(false);
        }
        let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let Some(block) = self.block_cursor.next(document)? else {
            self.events.seal()?;
            return Ok(true);
        };
        let id = ink_item_id(&block);
        let event = match self.drag.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)? {
            SceneDragMode::InkResize { handle, from, start_x, start_y, selected_ids } if selected_ids.iter().any(|selected| selected == id) => {
                let dx = (x - *start_x) as f64 / self.camera.zoom.max(0.0001);
                let dy = (y - *start_y) as f64 / self.camera.zoom.max(0.0001);
                let to = ink_resize_bounds(*from, handle, dx, dy, 8.0);
                let updated = scale_ink_item(&block, *from, to);
                Some(json!({ "operation": "updateBlock", "blockId": id, "block": updated }))
            }
            SceneDragMode::InkStroke { block_id } if block_id == id => {
                let mut updated = SCENE_STATE
                    .with(|cell| -> Result<Option<Value>, ui_wgpu::wgpu::BoundedActionFault> {
                        let states = cell.borrow();
                        let Some(value) = states.get(&scene.host_id).and_then(|state| state.ink_overrides.get(id)) else {
                            return Ok(None);
                        };
                        let mut nodes = 0usize;
                        validate_ink_value(value, 0, &mut nodes)?;
                        Ok(Some(value.clone()))
                    })?
                    .unwrap_or_else(|| block.clone());
                if !commit {
                    let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
                    let block_x = ink_item_num(&updated, "x");
                    let block_y = ink_item_num(&updated, "y");
                    if let Some(points) = updated.get_mut("points").and_then(Value::as_array_mut) {
                        if points.len() == INK_INTERACTION_ITEM_CAPACITY {
                            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                        }
                        points.push(json!([world_x - block_x, world_y - block_y]));
                    }
                }
                let update_json = serde_json::to_string(&updated).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                self.stroke_update = Some((id.to_owned(), InkOwnedRaw::new(&update_json)?));
                Some(json!({ "operation": "updateBlock", "blockId": id, "block": updated }))
            }
            _ => None,
        };
        if let Some(event) = event {
            self.events.push(&event)?;
        }
        Ok(false)
    }

    fn scan_marquee(&mut self, inner: Rect, x: f32, y: f32, start_x: f32, start_y: f32) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let Some(block) = self.block_cursor.next(document)? else {
            return Ok(true);
        };
        let x0 = start_x.min(x);
        let y0 = start_y.min(y);
        let (world_x0, world_y0) = ink_screen_to_world(self.camera, inner, x0, y0);
        let (world_x1, world_y1) = ink_screen_to_world(self.camera, inner, start_x.max(x), start_y.max(y));
        let rectangle = InkBoundsF { x: world_x0.min(world_x1), y: world_y0.min(world_y1), w: (world_x1 - world_x0).abs(), h: (world_y1 - world_y0).abs() };
        if ink_item_bounds(&block).intersects(&rectangle) {
            let Some(id) = ink_interaction_id(&block) else { return Ok(false) };
            let bytes = self.result_bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            if self.result_ids.len() == INK_SELECTION_ITEM_CAPACITY || bytes > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
                return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
            }
            self.result_ids.push(id.to_owned());
            self.result_bytes = bytes;
        }
        Ok(false)
    }

    fn scan_hit(&mut self, inner: Rect, x: f32, y: f32) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let Some(block) = self.block_cursor.next(document)? else {
            return Ok(true);
        };
        let hit = {
            let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
            if !ink_item_locked(&block) && ink_item_bounds(&block).contains_point(world_x, world_y) {
                let id = ink_item_id(&block);
                if id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
                }
                Some((id.to_owned(), (ink_item_num(&block, "x"), ink_item_num(&block, "y"))))
            } else {
                None
            }
        };
        if let Some((id, origin)) = hit {
            self.hit_interaction_id = ink_interaction_id(&block).map(str::to_owned);
            self.hit_id = Some(id);
            self.hit_origin = Some(origin);
            self.hit_block = Some(block);
        }
        Ok(false)
    }

    pub(crate) fn publish(&mut self, scene: &UiComponentSceneNode, inner: Rect, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        match self.event {
            InkInteractionEvent::PointerDown { x, y, button, shift } => self.publish_down(scene, inner, x, y, button, shift, input),
            InkInteractionEvent::PointerUp { x, y } => {
                if matches!(self.drag.as_ref(), Some(SceneDragMode::InkMarqueeDrag { .. })) {
                    return write_ink_interaction_selection(input, scene, &self.result_ids, "replace", "rectangle", || clear_ink_pointer_state(&scene.host_id));
                }
                if matches!(self.drag.as_ref(), Some(SceneDragMode::InkMove { .. } | SceneDragMode::InkResize { .. } | SceneDragMode::InkStroke { .. } | SceneDragMode::InkEraser { .. })) {
                    let gesture = self.ink_move_gesture(x, y);
                    return write_ink_events_action(input, scene, &self.events, Some("commit"), gesture.as_deref(), None, || clear_ink_pointer_state(&scene.host_id));
                }
                clear_ink_pointer_state(&scene.host_id);
                Ok(())
            }
            InkInteractionEvent::PointerMove { x, y } => self.publish_move(scene, inner, x, y, input),
        }
    }

    fn publish_down(&mut self, scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, shift: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        if self.utility == "pan" || button == 1 {
            let camera = self.camera;
            mutate_scene_state(&scene.host_id, |state| {
                state.pointer_was_down = true;
                state.drag = Some(SceneDrag { mode: SceneDragMode::InkPan { start_x: x, start_y: y, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom } });
            });
            return Ok(());
        }
        if button != 0 {
            return Ok(());
        }
        if self.utility == "eraserStroke" || self.utility == "eraserPoint" {
            let utility = self.utility.clone();
            if self.events.items == 0 {
                mutate_scene_state(&scene.host_id, |state| {
                    state.pointer_was_down = true;
                    state.drag = Some(SceneDrag { mode: SceneDragMode::InkEraser { mode: utility } });
                });
                return Ok(());
            }
            return write_ink_events_action(input, scene, &self.events, Some("stream"), None, None, || {
                mutate_scene_state(&scene.host_id, |state| {
                    state.pointer_was_down = true;
                    state.drag = Some(SceneDrag { mode: SceneDragMode::InkEraser { mode: utility } });
                });
            });
        }
        if self.utility == "selectMarquee" {
            mutate_scene_state(&scene.host_id, |state| {
                state.pointer_was_down = true;
                state.drag = Some(SceneDrag { mode: SceneDragMode::InkMarqueeDrag { start_x: x, start_y: y } });
                state.ink_marquee_points.clear();
                state.ink_marquee_points.push((x, y));
            });
            return Ok(());
        }
        if self.utility == "selectDirect" {
            if let Some(edit) = self.hit_block.as_ref().and_then(|block| ink_double_click_edit(&scene.host_id, block, self.camera, inner, x, y)) {
                let control_id = ink_edit_control_id(&scene.host_id, &edit);
                let draft = edit.draft.clone();
                if let (Some(_), Some(interaction_id)) = (ink_interaction_domain(scene), self.hit_interaction_id.as_ref()) {
                    write_ink_interaction_selection(input, scene, std::slice::from_ref(interaction_id), "replace", "pick", || activate_ink_edit(&scene.host_id, edit))?;
                } else {
                    activate_ink_edit(&scene.host_id, edit);
                }
                input.focus_input_owned(control_id, draft);
                self.opened_edit = true;
                return Ok(());
            }
            let hit_id = self.hit_id.as_deref();
            let hit_origin = self.hit_origin;
            let targets = match (ink_interaction_domain(scene).is_some(), hit_id, self.hit_interaction_id.as_ref()) {
                (true, None, _) => Some(Vec::new()),
                (true, Some(_), Some(target)) => Some(vec![target.clone()]),
                _ => None,
            };
            let begin_drag = || {
                mutate_scene_state(&scene.host_id, |state| {
                    state.pointer_was_down = true;
                    if let (Some(id), Some(origin)) = (hit_id, hit_origin) {
                        let mut origins = BTreeMap::new();
                        origins.insert(id.to_owned(), origin);
                        state.drag = Some(SceneDrag { mode: SceneDragMode::InkMove { origins, start_x: x, start_y: y } });
                    }
                });
            };
            return if let Some(targets) = targets {
                write_ink_interaction_selection(input, scene, &targets, if shift { "additive" } else { "replace" }, "pick", begin_drag)
            } else {
                begin_drag();
                Ok(())
            };
        }
        let document = self.document.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        if matches!(self.utility.as_str(), "pencil" | "text" | "image" | "table" | "math") {
            let (world_x, world_y) = ink_screen_to_world(self.camera, inner, x, y);
            let (world_x, world_y) = ink_maybe_snap_fields(document.snap_enabled, document.snap_grid_spacing, world_x, world_y);
            let kind = if self.utility == "pencil" { "stroke" } else { self.utility.as_str() };
            let block = create_ink_item(kind, world_x, world_y);
            let block_id = ink_item_id(&block).to_owned();
            let state_id = block_id.clone();
            let mut events = InkEventJsonPages::default();
            events.push(&json!({ "operation": "addBlock", "block": block.clone() }))?;
            events.seal()?;
            let phase = (self.utility == "pencil").then_some("stream");
            return write_ink_events_action(input, scene, &events, phase, None, Some(&block_id), || {
                if kind == "stroke" {
                    mutate_scene_state(&scene.host_id, |state| {
                        state.pointer_was_down = true;
                        state.ink_overrides.insert(state_id.clone(), block);
                        state.drag = Some(SceneDrag { mode: SceneDragMode::InkStroke { block_id: state_id } });
                    });
                }
            });
        }
        Ok(())
    }

    fn publish_move(&mut self, scene: &UiComponentSceneNode, _inner: Rect, x: f32, y: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        match self.drag.as_ref() {
            Some(SceneDragMode::InkPan { start_x, start_y, camera_x, camera_y, zoom }) => {
                let next = InkCameraF { x: *camera_x + (x - *start_x) as f64, y: *camera_y + (y - *start_y) as f64, zoom: *zoom };
                write_ink_camera_action(input, scene, next, || {
                    mutate_scene_state(&scene.host_id, |state| {
                        state.ink_camera = Some((next.x, next.y, next.zoom));
                    });
                })
            }
            Some(SceneDragMode::InkMarqueeDrag { start_x, start_y }) => {
                mutate_scene_state(&scene.host_id, |state| {
                    state.ink_marquee_points.clear();
                    state.ink_marquee_points.push((*start_x, *start_y));
                    state.ink_marquee_points.push((x, y));
                });
                Ok(())
            }
            Some(SceneDragMode::InkMove { .. }) => {
                let gesture = self.ink_move_gesture(x, y);
                write_ink_events_action(input, scene, &self.events, Some("stream"), gesture.as_deref(), None, || {})
            }
            Some(SceneDragMode::InkResize { .. } | SceneDragMode::InkEraser { .. }) => {
                if self.events.items == 0 {
                    Ok(())
                } else {
                    write_ink_events_action(input, scene, &self.events, Some("stream"), None, None, || {})
                }
            }
            Some(SceneDragMode::InkStroke { .. }) => {
                if let Some((id, _)) = self.stroke_update.as_ref() {
                    let saturated = SCENE_STATE.with(|cell| cell.borrow().get(&scene.host_id).is_some_and(|state| !state.ink_overrides.contains_key(id) && state.ink_overrides.len() == INK_INTERACTION_ITEM_CAPACITY));
                    if saturated {
                        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                    }
                }
                let update = self.stroke_update.as_ref().map(|(id, raw)| Ok((id.clone(), raw.as_str()?.to_owned()))).transpose()?;
                write_ink_events_action(input, scene, &self.events, Some("stream"), None, None, || {
                    if let Some((id, block_json)) = update {
                        let block = serde_json::from_str(&block_json).expect("validated retained ink block");
                        mutate_scene_state(&scene.host_id, |state| {
                            state.ink_overrides.insert(id, block);
                        });
                    }
                })
            }
            Some(_) => Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
            None => {
                let hovered = scene.ink_canvas.as_ref().and_then(|ink| ink.hovered_id.as_deref());
                if hovered == self.hit_id.as_deref() {
                    Ok(())
                } else if self.hit_id.is_some() && self.hit_interaction_id.is_none() {
                    Ok(())
                } else {
                    write_ink_interaction_hover(input, scene, self.hit_interaction_id.as_deref())
                }
            }
        }
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if self.selected_ids.pop().is_some() {
            return false;
        }
        if self.result_ids.pop().is_some() {
            return false;
        }
        if self.pending_remove_id.take().is_some() {
            return false;
        }
        if self.pending_fragments.pop_front() {
            return false;
        }
        if self.stroke_update.take().is_some() {
            return false;
        }
        if self.hit_block.take().is_some() {
            return false;
        }
        if let Some(drag) = self.drag.as_mut() {
            match drag {
                SceneDragMode::InkMove { origins, .. } => {
                    if let Some(id) = origins.keys().next().cloned() {
                        origins.remove(&id);
                        return false;
                    }
                }
                SceneDragMode::InkResize { handle, selected_ids, .. } => {
                    if selected_ids.pop().is_some() {
                        return false;
                    }
                    if !handle.is_empty() {
                        handle.clear();
                        return false;
                    }
                }
                SceneDragMode::InkStroke { block_id } => {
                    if !block_id.is_empty() {
                        block_id.clear();
                        return false;
                    }
                }
                SceneDragMode::InkEraser { mode } => {
                    if !mode.is_empty() {
                        mode.clear();
                        return false;
                    }
                }
                _ => {}
            }
            self.drag = None;
            return false;
        }
        let Some(document) = self.document.as_mut() else {
            return true;
        };
        if document.span_len > 0 {
            document.span_len -= 1;
            document.spans[document.span_len] = None;
            return false;
        }
        if !document.source.is_empty() {
            document.source.clear();
            return false;
        }
        if !document.schema.is_empty() {
            document.schema.clear();
            return false;
        }
        if !document.id.is_empty() {
            document.id.clear();
            return false;
        }
        if document.active_utility.take().is_some() {
            return false;
        }
        self.document = None;
        self.hit_id = None;
        self.opened_edit = false;
        true
    }
}
//#endregion InkCanvasState
//#endregion InkCanvas

pub(crate) const INK_RESIZE_HANDLES: [&str; 8] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
