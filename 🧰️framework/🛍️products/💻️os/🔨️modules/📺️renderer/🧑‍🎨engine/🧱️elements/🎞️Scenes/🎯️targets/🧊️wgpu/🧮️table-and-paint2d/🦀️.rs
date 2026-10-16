//! 🧮️ Paint2d navigator helpers and the table scene.

use super::*;

//#region Paint2d
// 🖌️ Production paint-2d rendering and input live on `RasterHost`
// (`🔨️modules/🗺️surface/🎨️paint/🦀️.rs`, attached by `semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::sync_paint2d_scene`) — the SAME
// host React's wasm `RasterSession` wraps. What remains in this region is the fixture-side document
// projection the standalone tests read.
#[cfg(test)]
#[derive(Deserialize, Clone, Copy)]
struct Paint2dCameraFields {
    #[serde(default)]
    pub(crate) x: f64,
    #[serde(default)]
    pub(crate) y: f64,
    #[serde(default = "paint2d_default_one")]
    pub(crate) zoom: f64,
}

#[cfg(test)]
impl Default for Paint2dCameraFields {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[cfg(test)]
fn paint2d_default_one() -> f64 {
    1.0
}

#[cfg(test)]
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Paint2dDocSyncJson {
    #[serde(default)]
    pub camera: Paint2dCameraFields,
}

#[cfg(test)]
pub struct Paint2dFlatLayer {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub width: u32,
    pub height: u32,
}

//#region Paint2dNavigator
#[cfg(test)]
pub const PAINT2D_NAVIGATOR_PADDING: f32 = 24.0;

/** 🧭️ Fits a camera to the document's pixel-layer bounds so a `viewMode === "navigator"` surface
 * shows the whole composition — a port of `RasterHost::navigator_fit_camera_json`
 * (`🔨️modules/🗺️surface/🎨️paint/🦀️.rs`'s `RasterHost::navigator_fit_camera_json`, which production
 * now calls directly through the attached host). Falls back to a neutral centered camera when the
 * document has no pixel content. */

/** 🧭️ Maps the main (composite) viewport's visible world rect into the navigator's own fitted
 * screen space, producing the "you are here" overlay rectangle — a port of
 * `RasterHost::navigator_viewport_overlay_json`. `content_camera_json` is the main surface's
 * `Paint2dScene.cameraJson` (echoed into the navigator's own scene payload by the owning program) and
 * `content_viewport_json` its reported `compositeViewportJson` (`{width,height}` in CSS px, set via
 * the React reference's `setCompositeViewport` action / this renderer's `ResizeObserver` equivalent
 * — see report for the pointer/resize wiring gap notes). Returns `None` when the main viewport size
 * hasn't been reported yet. */

//#endregion Paint2dNavigator

//#endregion Paint2d

//#region Table
#[derive(Deserialize)]
pub struct TableColumn {
    pub id: String,
    pub label: String,
    #[serde(default)]
    sortable: bool,
}

/// 🔀️ Mirrors `sourcing::TableSort`'s wire format (`{columnId, direction}`) — the active sort
/// state for a [`TableScene`] whose columns opt into `sortable`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableSortJson {
    pub(crate) column_id: String,
    direction: String,
}

/// 🧾️ Mirrors `ui_wgpu::wgpu::TableCell` — a typed table cell value parsed out of a row's raw JSON.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum TableCellPayload {
    Text { value: String },
    Number { value: f64 },
    EditableText { value: String, action: ActionDescriptor },
    Stepper { value: f64, min: f64, max: f64, step: f64, action: ActionDescriptor },
    Buttons { buttons: Vec<TableCellButtonPayload> },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableCellButtonPayload {
    pub(crate) icon_id: String,
    #[serde(default)]
    pub(crate) label: Option<String>,
    pub(crate) action: ActionDescriptor,
    /// 🔘️ `"row"` (the default) draws the button in the row itself; `"menu"` moves it into the row's
    /// context menu instead — the same split `renderTableCell`/`tableRowMenuPlacementItems` make.
    #[serde(default)]
    placement: Option<String>,
}

/// 🔗️ Merges `patch` into `base`'s existing args (rather than replacing them), so a stepper/button
/// cell keeps its row-identifying args (e.g. `objectId`) alongside the delta/click patch — the port
/// of `dispatchCellAction`'s `{ ...descriptor.args, ...patch }` spread in `📊️Table/🟦️.tsx`.
pub fn merge_action_args(base: &ActionDescriptor, patch: Value) -> ActionDescriptor {
    let mut args = match &base.args {
        Some(dsl) => match Value::from(dsl) {
            Value::Object(map) => map,
            _ => serde_json::Map::new(),
        },
        None => serde_json::Map::new(),
    };
    if let Value::Object(patch_map) = patch {
        args.extend(patch_map);
    }
    ActionDescriptor { controller_id: base.controller_id.clone(), action: base.action.clone(), args: Some(semio_framework::DslValue::from(Value::Object(args))) }
}

/// 🪪️ A row's identity, exactly as `TableHost`'s `rowIds` map derives it: `id`, then `pluginId`,
/// then the row's own ordinal. The ordinal fallback matters — a payload whose rows carry neither key
/// used to collapse every row onto the empty id, so selection highlighted all of them at once.
pub fn table_row_id(row: &Value, index: usize) -> String {
    row.get("id").or_else(|| row.get("pluginId")).and_then(Value::as_str).map(str::to_string).unwrap_or_else(|| index.to_string())
}

/// 🖱️ What a row click dispatches. A table that declares an interaction domain picks into it
/// (`interactionSelect`, with `targets` a JSON STRING of `[{granularity, id}]` — the wire shape the
/// framework's interaction bus reads); a plugin-private table keeps its own `selectRow` verb.
/// Ported from `TableHost`'s `onRowClick`.
fn table_row_action(scene: &UiComponentSceneNode, table: &ui_wgpu::wgpu::TableScene, row: &Value, row_id: &str, modifiers: SceneModifiers) -> ActionDescriptor {
    match (table.domain_id.as_deref(), table.domain_granularity_id.as_deref()) {
        (Some(domain_id), Some(granularity)) => {
            scene_action(scene, "interactionSelect", semio_framework::dsl_value!({ "domainId": domain_id, "targets": json!([{ "granularity": granularity, "id": row_id }]).to_string(), "merge": modifiers.interaction_merge(), "method": "pick" }))
        }
        _ => scene_action(scene, "selectRow", semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "row": semio_framework::DslValue::from(row) })),
    }
}

/// 🔘️ The buttons a `buttons` cell draws IN the row — `placement: "menu"` entries belong to the
/// row's context menu instead, exactly as `renderTableCell` filters them.
fn table_row_buttons(buttons: &[TableCellButtonPayload]) -> Vec<&TableCellButtonPayload> {
    buttons.iter().filter(|button| button.placement.as_deref().unwrap_or("row") == "row").collect()
}

/// 📐️ One logical segment shared by Table paint, pointer routing and accessibility.
fn table_segment_rect(rect: Rect, index: usize, count: usize) -> Option<Rect> {
    if count == 0 || index >= count || rect.w <= 0.0 || rect.h <= 0.0 || !rect.x.is_finite() || !rect.y.is_finite() || !rect.w.is_finite() || !rect.h.is_finite() {
        return None;
    }
    let width = rect.w / count as f32;
    (width.is_finite() && width > 0.0).then(|| Rect::new(rect.x + index as f32 * width, rect.y, width, rect.h))
}

/// 🎯️ The segment whose half-open horizontal interval owns one pointer coordinate.
fn table_segment_index_at_x(rect: Rect, x: f32, count: usize) -> Option<usize> {
    (0..count).find(|index| table_segment_rect(rect, *index, count).is_some_and(|segment| x >= segment.x && x < segment.x + segment.w))
}

/// ✂️ The visible intersection accessibility may announce inside the Table body.
fn table_visible_rect(rect: Rect, clip: Rect) -> Option<Rect> {
    let x = rect.x.max(clip.x);
    let y = rect.y.max(clip.y);
    let right = (rect.x + rect.w).min(clip.x + clip.w);
    let bottom = (rect.y + rect.h).min(clip.y + clip.h);
    let width = right - x;
    let height = bottom - y;
    (width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0).then(|| Rect::new(x, y, width, height))
}

/// 🧾️ Renders a table cell's interactive controls (stepper/buttons) directly, or returns the plain
/// text to draw for text/number/legacy-string cells.
fn render_table_cell(cell: &Value, id: &str, rect: Rect, ctx: &mut FrameworkWidgetContext<'_>) -> Option<String> {
    let Ok(payload) = serde_json::from_value::<TableCellPayload>(cell.clone()) else {
        return Some(match cell {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        });
    };
    match payload {
        TableCellPayload::Text { value } => Some(value),
        TableCellPayload::Number { value } => Some(value.to_string()),
        TableCellPayload::EditableText { value, action } => {
            render_widget(&WidgetNode::Input { id: id.to_string(), input_kind: "text".into(), value, placeholder: None, commit: Some("table-cell".into()), min: None, max: None, step: None, accept: None, on_change: Some(action) }, rect, ctx);
            None
        }
        TableCellPayload::Stepper { value, min, max, step, action } => {
            let Some(minus) = table_segment_rect(rect, 0, 3) else { return None };
            let Some(center) = table_segment_rect(rect, 1, 3) else { return None };
            let Some(plus) = table_segment_rect(rect, 2, 3) else { return None };
            render_widget(&WidgetNode::Button { id: None, icon_id: None, label: "−".into(), event: (value > min).then(|| merge_action_args(&action, json!({ "delta": -step }))) }, minus, ctx);
            render_widget(&WidgetNode::Text { value: format!("{value:.0}"), emphasize: false }, center, ctx);
            render_widget(&WidgetNode::Button { id: None, icon_id: None, label: "+".into(), event: (value < max).then(|| merge_action_args(&action, json!({ "delta": step }))) }, plus, ctx);
            None
        }
        TableCellPayload::Buttons { buttons } => {
            let row_buttons = table_row_buttons(&buttons);
            if row_buttons.is_empty() {
                return None;
            }
            for (index, button) in row_buttons.iter().enumerate() {
                let Some(button_rect) = table_segment_rect(rect, index, row_buttons.len()) else { continue };
                render_widget(&WidgetNode::Button { id: None, icon_id: IconName::from_str(&button.icon_id), label: button.label.clone().unwrap_or_default(), event: Some(button.action.clone()) }, button_rect, ctx);
            }
            None
        }
    }
}

/// 📐️ A table's fixed metrics — the ONE derivation the paint lays rows out with and the pointer
/// path re-derives hits from, so a header band, a row band and their hit bands cannot drift.
struct TableMetrics {
    pub(crate) header_h: f32,
    pub(crate) row_h: f32,
    pub(crate) pad: f32,
    col_w: f32,
    pub(crate) handle_size: f32,
    pub(crate) body: Rect,
}

pub fn table_metrics(inner: Rect, columns: usize, theme: &Theme) -> TableMetrics {
    let header_h = theme.control_height * 1.33;
    TableMetrics {
        header_h,
        row_h: theme.control_height,
        pad: theme.padding_standard,
        col_w: if columns == 0 { inner.w } else { inner.w / columns as f32 },
        handle_size: theme.control_height_small.min(theme.control_height),
        body: Rect::new(inner.x, inner.y + header_h, inner.w, inner.h - header_h),
    }
}

fn table_transfer_handle_rect(row: Rect, theme: &Theme) -> Rect {
    let size = theme.control_height_small.min(row.h).min(row.w.max(0.0));
    Rect::new(row.x + theme.padding_standard, row.y + (row.h - size) * 0.5, size.min((row.w - theme.padding_standard).max(0.0)), size)
}

fn table_cell_rect(inner: Rect, row_y: f32, column_index: usize, metrics: &TableMetrics, driver_drag: UiDriverDrag, draggable: bool) -> Rect {
    let mut x = inner.x + column_index as f32 * metrics.col_w + metrics.pad;
    let mut width = (metrics.col_w - metrics.pad * 2.0).max(1.0);
    if column_index == 0 && draggable && driver_drag == UiDriverDrag::Handle {
        let reservation = metrics.handle_size + metrics.pad;
        x += reservation;
        width = (width - reservation).max(1.0);
    }
    Rect::new(x, row_y, width, metrics.row_h)
}

pub fn table_transfer_start(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, theme: &Theme, driver_drag: UiDriverDrag) -> Option<SceneListTransferStart> {
    let table = scene.table.as_ref()?;
    let mime = table.row_drag_mime.clone()?;
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let metrics = table_metrics(inner, columns.len(), theme);
    if !metrics.body.contains(x, y) {
        return None;
    }
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let scroll = scroll_offset(&scene.host_id, "body");
    let index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    let row = rows.get(index)?;
    let payload = row.get("_drag")?.to_string();
    let row_id = table_row_id(row, index);
    let row_rect = Rect::new(metrics.body.x, metrics.body.y + index as f32 * metrics.row_h - scroll, metrics.body.w, metrics.row_h);
    match driver_drag {
        UiDriverDrag::Handle if !table_transfer_handle_rect(row_rect, theme).contains(x, y) => return None,
        UiDriverDrag::Surface => {
            let column_index = usize::try_from(((x - inner.x) / metrics.col_w.max(1.0)).floor() as i64).ok();
            if let Some((column_index, column)) = column_index.and_then(|index| columns.get(index).map(|column| (index, column))) {
                if let Some(cell) = row.get(&column.id) {
                    let cell_rect = table_cell_rect(inner, row_rect.y, column_index, &metrics, driver_drag, true);
                    if table_cell_hit(cell, cell_rect, x).is_some() {
                        return None;
                    }
                }
            }
        }
        UiDriverDrag::Handle => {}
    }
    Some(SceneListTransferStart { source: SceneListTransferSource::TableRow { row_id, mime, payload }, rect: row_rect })
}

/// 🔀️ The direction a press on `column`'s header asks for next — ascending unless this column is
/// already the ascending sort, matching `Table`'s own `onSort` toggle.
fn table_next_sort_direction(sort: Option<&TableSortJson>, column_id: &str) -> &'static str {
    match sort.filter(|sort| sort.column_id == column_id) {
        Some(sort) if sort.direction == "asc" => "desc",
        _ => "asc",
    }
}

/// 🎯️ The cell control a press at `x` lands on inside `cell_rect`, or `None` when the cell is plain
/// text and the press belongs to the row underneath.
fn table_cell_hit(cell: &Value, cell_rect: Rect, x: f32) -> Option<Option<ActionDescriptor>> {
    let payload = serde_json::from_value::<TableCellPayload>(cell.clone()).ok()?;
    match payload {
        TableCellPayload::Stepper { value, min, max, step, action } => {
            Some(match table_segment_index_at_x(cell_rect, x, 3) {
                Some(0) if value > min => Some(merge_action_args(&action, json!({ "delta": -step }))),
                Some(2) if value < max => Some(merge_action_args(&action, json!({ "delta": step }))),
                _ => None,
            })
        }
        TableCellPayload::Buttons { buttons } => {
            let row_buttons = table_row_buttons(&buttons);
            if row_buttons.is_empty() {
                return None;
            }
            Some(table_segment_index_at_x(cell_rect, x, row_buttons.len()).and_then(|index| row_buttons.get(index)).map(|button| button.action.clone()))
        }
        TableCellPayload::EditableText { .. } => Some(None),
        TableCellPayload::Text { .. } | TableCellPayload::Number { .. } => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableEditableTextCommitOutcome {
    Unchanged,
    Committed,
    Publishing,
    AwaitingEcho,
    Conflict,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableStepperKeyOutcome {
    Unhandled,
    Consumed,
}

/** ✏️ Resolves one painted editable text cell to its stable row/column address and persisted base. */
pub fn table_editable_text_focus_target(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, driver_drag: UiDriverDrag) -> Option<TableEditableTextFocusTarget> {
    if !inner.contains(x, y) {
        return None;
    }
    let theme = scene_input_theme();
    let table = scene.table.as_ref()?;
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).ok()?;
    let metrics = table_metrics(inner, columns.len(), &theme);
    if !metrics.body.contains(x, y) {
        return None;
    }
    let column_index = usize::try_from(((x - inner.x) / metrics.col_w.max(1.0)).floor() as i64).ok()?;
    let column = columns.get(column_index)?;
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).ok()?;
    let scroll = scroll_offset(&scene.host_id, "body");
    let row_index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    let row = rows.get(row_index)?;
    let row_id = table_row_id(row, row_index);
    let draggable = table.row_drag_mime.is_some() && row.get("_drag").is_some();
    let row_y = metrics.body.y + row_index as f32 * metrics.row_h - scroll;
    if !table_cell_rect(inner, row_y, column_index, &metrics, driver_drag, draggable).contains(x, y) {
        return None;
    }
    let TableCellPayload::EditableText { value, .. } = serde_json::from_value::<TableCellPayload>(row.get(&column.id)?.clone()).ok()? else { return None };
    Some(TableEditableTextFocusTarget { row_id, column_id: column.id.clone(), value })
}

/** 📦️ Commits an editable cell draft only while its persisted base still matches the focused base. */
pub(crate) fn table_editable_text_commit(
    scene: &UiComponentSceneNode,
    row_id: &str,
    column_id: &str,
    base: &str,
    draft: &str,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Option<Result<TableEditableTextCommitOutcome, ui_wgpu::wgpu::BoundedActionFault>> {
    let table = scene.table.as_ref()?;
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).ok()?;
    if !columns.iter().any(|column| column.id == column_id) {
        return None;
    }
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).ok()?;
    let (_, row) = rows.iter().enumerate().find(|(index, row)| table_row_id(row, *index) == row_id)?;
    let TableCellPayload::EditableText { value, action } = serde_json::from_value::<TableCellPayload>(row.get(column_id)?.clone()).ok()? else { return None };
    if draft == value {
        return Some(Ok(TableEditableTextCommitOutcome::Unchanged));
    }
    if value != base {
        return Some(Ok(TableEditableTextCommitOutcome::Conflict));
    }
    Some(input.begin_retained_text_action(action, "value".into()).map(|_| TableEditableTextCommitOutcome::Publishing))
}

pub fn table_editable_text_commit_owned(
    scene: &UiComponentSceneNode,
    row_id: &str,
    column_id: &str,
    base: &str,
    draft: String,
    awaiting_echo: Option<&str>,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Option<Result<TableEditableTextCommitOutcome, ui_wgpu::wgpu::BoundedActionFault>> {
    let table = scene.table.as_ref()?;
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).ok()?;
    if !columns.iter().any(|column| column.id == column_id) {
        return None;
    }
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).ok()?;
    let (_, row) = rows.iter().enumerate().find(|(index, row)| table_row_id(row, *index) == row_id)?;
    let TableCellPayload::EditableText { value, action } = serde_json::from_value::<TableCellPayload>(row.get(column_id)?.clone()).ok()? else { return None };
    if let Some(awaiting_echo) = awaiting_echo {
        if value == awaiting_echo {
            if draft == value {
                return Some(Ok(TableEditableTextCommitOutcome::Unchanged));
            }
            return Some(input.begin_retained_string_action_owned(action, "value".into(), draft).map(|_| TableEditableTextCommitOutcome::Publishing));
        } else if value == base && draft == awaiting_echo {
            return Some(Ok(TableEditableTextCommitOutcome::AwaitingEcho));
        } else {
            return Some(Ok(TableEditableTextCommitOutcome::Conflict));
        }
    }
    if draft == value {
        return Some(Ok(TableEditableTextCommitOutcome::Unchanged));
    }
    if value != base {
        return Some(Ok(TableEditableTextCommitOutcome::Conflict));
    }
    Some(input.begin_retained_string_action_owned(action, "value".into(), draft).map(|_| TableEditableTextCommitOutcome::Publishing))
}

/** 🎯️ Resolves the read-only centre of one painted Table stepper into its stable row/column
 * address. The same metrics and scroll authority as `table_hit` are used, so keyboard focus can
 * only be armed for the cell the accepted frame actually presented. */
pub fn table_stepper_focus_target(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, driver_drag: UiDriverDrag) -> Option<TableStepperFocusTarget> {
    if !inner.contains(x, y) {
        return None;
    }
    let theme = scene_input_theme();
    let table = scene.table.as_ref()?;
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).ok()?;
    let metrics = table_metrics(inner, columns.len(), &theme);
    if !metrics.body.contains(x, y) {
        return None;
    }
    let column_index = usize::try_from(((x - inner.x) / metrics.col_w.max(1.0)).floor() as i64).ok()?;
    let column = columns.get(column_index)?;
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).ok()?;
    let scroll = scroll_offset(&scene.host_id, "body");
    let row_index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    let row = rows.get(row_index)?;
    let row_id = table_row_id(row, row_index);
    let draggable = table.row_drag_mime.is_some() && row.get("_drag").is_some();
    let row_y = metrics.body.y + row_index as f32 * metrics.row_h - scroll;
    let cell_rect = table_cell_rect(inner, row_y, column_index, &metrics, driver_drag, draggable);
    if !cell_rect.contains(x, y) || !matches!(serde_json::from_value::<TableCellPayload>(row.get(&column.id)?.clone()).ok()?, TableCellPayload::Stepper { .. }) {
        return None;
    }
    table_segment_rect(cell_rect, 1, 3)?.contains(x, y).then(|| TableStepperFocusTarget { row_id, column_id: column.id.clone() })
}

/** ♿️ Projects every visible stepper readout from the exact accepted Table geometry. These
 * virtual spinbuttons fill the semantic gap left by a `Component::Surface` document leaf, whose
 * authored accessibility record can only describe the surface as a whole. */
pub fn table_stepper_accessibility_cells(scene: &UiComponentSceneNode, inner: Rect, driver_drag: UiDriverDrag) -> Vec<TableStepperAccessibilityCell> {
    let Some(table) = scene.table.as_ref() else { return Vec::new() };
    let Ok(columns) = serde_json::from_str::<Vec<TableColumn>>(&table.columns_json) else { return Vec::new() };
    let Ok(rows) = serde_json::from_str::<Vec<Value>>(&table.rows_json) else { return Vec::new() };
    let theme = scene_input_theme();
    let metrics = table_metrics(inner, columns.len(), &theme);
    let scroll = scroll_offset(&scene.host_id, "body");
    let mut cells = Vec::new();
    for (row_index, row) in rows.iter().enumerate() {
        let row_y = metrics.body.y + row_index as f32 * metrics.row_h - scroll;
        if row_y + metrics.row_h <= metrics.body.y || row_y >= metrics.body.y + metrics.body.h {
            continue;
        }
        let row_id = table_row_id(row, row_index);
        let draggable = table.row_drag_mime.is_some() && row.get("_drag").is_some();
        for (column_index, column) in columns.iter().enumerate() {
            let Some(cell) = row.get(&column.id) else { continue };
            let Ok(TableCellPayload::Stepper { value, min, max, .. }) = serde_json::from_value::<TableCellPayload>(cell.clone()) else { continue };
            let cell_rect = table_cell_rect(inner, row_y, column_index, &metrics, driver_drag, draggable);
            let Some(rect) = table_segment_rect(cell_rect, 1, 3).and_then(|rect| table_visible_rect(rect, metrics.body)) else { continue };
            cells.push(TableStepperAccessibilityCell {
                key: format!("{}.row.{}.{}.stepper", scene.host_id, row_id, column.id),
                label: column.label.clone(),
                target: TableStepperFocusTarget { row_id: row_id.clone(), column_id: column.id.clone() },
                rect,
                min,
                max,
                value,
            });
        }
    }
    cells
}

/** ♿️ Projects every visible editable cell as a multiline textbox over its painted cell. */
pub fn table_editable_text_accessibility_cells(scene: &UiComponentSceneNode, inner: Rect, driver_drag: UiDriverDrag) -> Vec<TableEditableTextAccessibilityCell> {
    let Some(table) = scene.table.as_ref() else { return Vec::new() };
    let Ok(columns) = serde_json::from_str::<Vec<TableColumn>>(&table.columns_json) else { return Vec::new() };
    let Ok(rows) = serde_json::from_str::<Vec<Value>>(&table.rows_json) else { return Vec::new() };
    let theme = scene_input_theme();
    let metrics = table_metrics(inner, columns.len(), &theme);
    let scroll = scroll_offset(&scene.host_id, "body");
    let mut cells = Vec::new();
    for (row_index, row) in rows.iter().enumerate() {
        let row_y = metrics.body.y + row_index as f32 * metrics.row_h - scroll;
        if row_y + metrics.row_h <= metrics.body.y || row_y >= metrics.body.y + metrics.body.h {
            continue;
        }
        let row_id = table_row_id(row, row_index);
        let draggable = table.row_drag_mime.is_some() && row.get("_drag").is_some();
        for (column_index, column) in columns.iter().enumerate() {
            let Some(cell) = row.get(&column.id) else { continue };
            let Ok(TableCellPayload::EditableText { value, .. }) = serde_json::from_value::<TableCellPayload>(cell.clone()) else { continue };
            cells.push(TableEditableTextAccessibilityCell {
                key: format!("{}.row.{}.{}.editable", scene.host_id, row_id, column.id),
                label: column.label.clone(),
                target: TableEditableTextFocusTarget { row_id: row_id.clone(), column_id: column.id.clone(), value },
                rect: table_cell_rect(inner, row_y, column_index, &metrics, driver_drag, draggable),
            });
        }
    }
    cells
}

/** 🔘️ Projects every visible row-placement Table button from the same segmented cell geometry as paint and pointer hit testing. */
pub fn table_button_accessibility_cells(scene: &UiComponentSceneNode, inner: Rect, driver_drag: UiDriverDrag) -> Vec<TableButtonAccessibilityCell> {
    let Some(table) = scene.table.as_ref() else { return Vec::new() };
    let Ok(columns) = serde_json::from_str::<Vec<TableColumn>>(&table.columns_json) else { return Vec::new() };
    let Ok(rows) = serde_json::from_str::<Vec<Value>>(&table.rows_json) else { return Vec::new() };
    let theme = scene_input_theme();
    let metrics = table_metrics(inner, columns.len(), &theme);
    let scroll = scroll_offset(&scene.host_id, "body");
    let mut cells = Vec::new();
    for (row_index, row) in rows.iter().enumerate() {
        let row_y = metrics.body.y + row_index as f32 * metrics.row_h - scroll;
        if row_y + metrics.row_h <= metrics.body.y || row_y >= metrics.body.y + metrics.body.h {
            continue;
        }
        let row_id = table_row_id(row, row_index);
        let draggable = table.row_drag_mime.is_some() && row.get("_drag").is_some();
        for (column_index, column) in columns.iter().enumerate() {
            let Some(cell) = row.get(&column.id) else { continue };
            let Ok(TableCellPayload::Buttons { buttons }) = serde_json::from_value::<TableCellPayload>(cell.clone()) else { continue };
            let row_buttons = table_row_buttons(&buttons);
            if row_buttons.is_empty() {
                continue;
            }
            let cell_rect = table_cell_rect(inner, row_y, column_index, &metrics, driver_drag, draggable);
            let button_count = row_buttons.len();
            for (button_index, button) in row_buttons.into_iter().enumerate() {
                let Some(rect) = table_segment_rect(cell_rect, button_index, button_count).and_then(|rect| table_visible_rect(rect, metrics.body)) else { continue };
                cells.push(TableButtonAccessibilityCell {
                    key: format!("{}.row.{}.{}.{}", scene.host_id, row_id, column.id, button_index),
                    label: button.label.clone().unwrap_or_default(),
                    row_id: row_id.clone(),
                    column_id: column.id.clone(),
                    button_index,
                    rect,
                    action: button.action.clone(),
                });
            }
        }
    }
    cells
}

pub fn table_button_accessibility_activate(scene: &UiComponentSceneNode, key: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Option<Result<(), ui_wgpu::wgpu::BoundedActionFault>> {
    let control = accepted_table_button_accessibility_cells(&scene.host_id).into_iter().find(|control| control.key == key)?;
    let table = scene.table.as_ref()?;
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).ok()?;
    if !columns.iter().any(|column| column.id == control.column_id) {
        return None;
    }
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).ok()?;
    let (_, row) = rows.iter().enumerate().find(|(index, row)| table_row_id(row, *index) == control.row_id)?;
    let TableCellPayload::Buttons { buttons } = serde_json::from_value::<TableCellPayload>(row.get(&control.column_id)?.clone()).ok()? else { return None };
    let button = table_row_buttons(&buttons).get(control.button_index).copied()?;
    let expected_key = format!("{}.row.{}.{}.{}", scene.host_id, control.row_id, control.column_id, control.button_index);
    (control.key == expected_key && button.label.as_deref().unwrap_or_default() == control.label && button.action == control.action).then(|| write_scene_action(input, &control.action))
}

/** ⌨️ Re-resolves a focused Table stepper against the current accepted scene before every key,
 * clamps the React key delta to the live bounds, and publishes the cell's current descriptor. A
 * missing row/column or a cell that is no longer a stepper returns `None`, retiring stale focus. */
pub fn table_stepper_apply_key(
    scene: &UiComponentSceneNode,
    row_id: &str,
    column_id: &str,
    key: &ui_wgpu::wgpu::KeyAction,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Option<Result<TableStepperKeyOutcome, ui_wgpu::wgpu::BoundedActionFault>> {
    let table = scene.table.as_ref()?;
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).ok()?;
    if !columns.iter().any(|column| column.id == column_id) {
        return None;
    }
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).ok()?;
    let (_, row) = rows.iter().enumerate().find(|(index, row)| table_row_id(row, *index) == row_id)?;
    let TableCellPayload::Stepper { value, min, max, step, action } = serde_json::from_value::<TableCellPayload>(row.get(column_id)?.clone()).ok()? else {
        return None;
    };
    let requested = match key {
        ui_wgpu::wgpu::KeyAction::ArrowUp | ui_wgpu::wgpu::KeyAction::ArrowRight => step,
        ui_wgpu::wgpu::KeyAction::ArrowDown | ui_wgpu::wgpu::KeyAction::ArrowLeft => -step,
        ui_wgpu::wgpu::KeyAction::PageUp => step * 10.0,
        ui_wgpu::wgpu::KeyAction::PageDown => -step * 10.0,
        ui_wgpu::wgpu::KeyAction::Home => min - value,
        ui_wgpu::wgpu::KeyAction::End => max - value,
        _ => return Some(Ok(TableStepperKeyOutcome::Unhandled)),
    };
    if requested == 0.0 {
        return Some(Ok(TableStepperKeyOutcome::Unhandled));
    }
    let delta = (value + requested).clamp(min, max) - value;
    if delta != 0.0 {
        if let Err(fault) = write_scene_action(input, &merge_action_args(&action, json!({ "delta": delta }))) {
            return Some(Err(fault));
        }
    }
    Some(Ok(TableStepperKeyOutcome::Consumed))
}

/// 🎯️ Resolves a pointer point inside a `SurfaceKind::Table`: a sortable header band, a stepper
/// segment, a row action button, or the row itself.
pub fn table_hit(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, theme: &Theme, modifiers: SceneModifiers, driver_drag: UiDriverDrag) -> Option<SceneListHit> {
    let table = scene.table.as_ref()?;
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let metrics = table_metrics(inner, columns.len(), theme);
    let column_index = usize::try_from(((x - inner.x) / metrics.col_w.max(1.0)).floor() as i64).ok();
    let column = column_index.and_then(|index| columns.get(index).map(|column| (index, column)));
    if y < metrics.body.y {
        let (_, column) = column?;
        if !column.sortable {
            return None;
        }
        let sort: Option<TableSortJson> = table.sort_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
        let direction = table_next_sort_direction(sort.as_ref(), &column.id);
        let action = scene_action(scene, "sortTable", semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "columnId": column.id, "direction": direction }));
        return Some(SceneListHit::row(format!("{}.header.{}", scene.host_id, column.id), Some(action)));
    }
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let scroll = scroll_offset(&scene.host_id, "body");
    let index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    let row = rows.get(index)?;
    let row_id = table_row_id(row, index);
    let control_id = format!("{}.row.{}", scene.host_id, row_id);
    let draggable = table.row_drag_mime.is_some() && row.get("_drag").is_some();
    let row_y = metrics.body.y + index as f32 * metrics.row_h - scroll;
    if draggable && driver_drag == UiDriverDrag::Handle && table_transfer_handle_rect(Rect::new(metrics.body.x, row_y, metrics.body.w, metrics.row_h), theme).contains(x, y) {
        return Some(SceneListHit::row(format!("{}.row.{}.drag", scene.host_id, row_id), None));
    }
    if let Some((column_index, column)) = column {
        if let Some(cell) = row.get(&column.id) {
            let cell_rect = table_cell_rect(inner, row_y, column_index, &metrics, driver_drag, draggable);
            if let Some(action) = table_cell_hit(cell, cell_rect, x) {
                return Some(SceneListHit::row(control_id, action));
            }
        }
    }
    Some(SceneListHit::row(control_id, Some(table_row_action(scene, table, row, &row_id, modifiers))))
}

/// 📊️ Renders `SurfaceKind::Table`: a sortable header band over a scrolling, selectable row body
/// whose cells may themselves be steppers or action buttons — the port of `📊️Table/🟦️.tsx`.
///
/// 🖱️ The PAINT-time fallback action carries the unmodified merge: a paint cannot know which
/// keys a press three frames later will hold. The live press resolves through
/// `passive_scene_pointer_button` → `table_hit`, which re-derives this row from the same
/// geometry WITH the press's own `SceneModifiers` — that is the path that answers shift/ctrl.
pub fn render_table(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, driver_drag: UiDriverDrag) {
    let theme = ctx.theme;
    let Some(table) = &scene.table else {
        stage_table_button_accessibility_cells(&scene.host_id, Vec::new());
        return render_placeholder("table", bounds, ctx);
    };
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let selected_ids: Vec<String> =
        table.selection_json.as_deref().and_then(|json| serde_json::from_str::<Value>(json).ok()).and_then(|value| value.get("selectedIds").cloned()).and_then(|value| serde_json::from_value(value).ok()).unwrap_or_default();
    let sort: Option<TableSortJson> = table.sort_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    let inner = bounds;
    let metrics = table_metrics(inner, columns.len(), theme);
    let (header_h, row_h, pad, col_w) = (metrics.header_h, metrics.row_h, metrics.pad, metrics.col_w);
    ctx.draw.push_solid([inner.x, inner.y, inner.w, header_h], theme.panel);
    for (index, column) in columns.iter().enumerate() {
        let x = inner.x + index as f32 * col_w;
        let sorted_here = sort.as_ref().filter(|s| s.column_id == column.id);
        let label = match sorted_here {
            Some(s) if s.direction == "desc" => format!("{} \u{25BC}", column.label),
            Some(_) => format!("{} \u{25B2}", column.label),
            None => column.label.clone(),
        };
        draw_text(ctx, &label, x + pad, inner.y + header_h * 0.65, theme.font_size_small, if sorted_here.is_some() { theme.text } else { theme.text_muted });
        if column.sortable {
            let direction = table_next_sort_direction(sort.as_ref(), &column.id);
            ctx.input.register_hit(HitTarget {
                rect: Rect::new(x, inner.y, col_w, header_h),
                event: Some(scene_action(scene, "sortTable", semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "columnId": column.id, "direction": direction }))),
                control_id: Some(format!("{}.header.{}", scene.host_id, column.id)),
                kind: HitKind::Generic,
                drag_axis: None,
                drag_data: None,
            });
        }
    }
    ctx.draw.push_line(inner.x, inner.y + header_h, inner.x + inner.w, inner.y + header_h, theme.separator, 1.0);
    let body = metrics.body;
    let scroll = scroll_offset(&scene.host_id, "body");
    ctx.input.register_hit(HitTarget { rect: body, event: None, control_id: Some(scroll_key(&scene.host_id, "body")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(body);
    reserve_list_rows(ctx, list_visible_row_capacity(body, row_h));
    let hovered_row = scene_hovered_control_id(&scene.host_id).or_else(|| ctx.input.hovered_id.clone());
    if rows.is_empty() {
        let message = "No rows";
        draw_text(ctx, message, body.x + body.w * 0.5 - 40.0, body.y + body.h * 0.5, theme.font_size_small, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        let row_id = table_row_id(row, row_index);
        let control_id = format!("{}.row.{}", scene.host_id, row_id);
        let row_rect = Rect::new(body.x, y, body.w, row_h);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let selected = selected_ids.iter().any(|id| id == &row_id);
        let draggable = table.row_drag_mime.is_some() && row.get("_drag").is_some();
        if selected {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.selected);
        } else if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);
        for (col_index, column) in columns.iter().enumerate() {
            let cell_rect = table_cell_rect(inner, y, col_index, &metrics, driver_drag, draggable);
            let text = match row.get(&column.id) {
                Some(value) => render_table_cell(value, &format!("{}.row.{}.{}.editable", scene.host_id, row_id, column.id), cell_rect, ctx),
                None => Some("—".into()),
            };
            if let Some(text) = text {
                draw_text(ctx, &text, cell_rect.x, y + row_h * 0.65, theme.font_size_small, if selected || hovered { theme.active_foreground } else { theme.text });
            }
        }
        if draggable && driver_drag == UiDriverDrag::Handle {
            let handle = table_transfer_handle_rect(row_rect, theme);
            if let Some(icons) = ctx.icons {
                let icon_size = theme.control_height_small.min(handle.w).min(handle.h);
                push_icon(ctx.draw, icons, "move", handle.x + (handle.w - icon_size) * 0.5, handle.y + (handle.h - icon_size) * 0.5, icon_size, foreground_on_fill(theme, theme.text_muted, selected, hovered && !selected));
            }
        }
        let drag_data = table.row_drag_mime.as_ref().and_then(|mime| row.get("_drag").map(|payload| HashMap::from([(mime.clone(), payload.to_string())])));
        ctx.input.register_hit(HitTarget { rect: row_rect, event: Some(table_row_action(scene, table, row, &row_id, SceneModifiers::default())), control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data });
    }
    stage_table_stepper_accessibility_cells(&scene.host_id, table_stepper_accessibility_cells(scene, bounds, driver_drag));
    stage_table_editable_text_accessibility_cells(&scene.host_id, table_editable_text_accessibility_cells(scene, bounds, driver_drag));
    stage_table_button_accessibility_cells(&scene.host_id, table_button_accessibility_cells(scene, bounds, driver_drag));
    ctx.draw.pop_scissor();
}

//#endregion Table

//#region TableTests
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-table/🦀️.rs"]
mod table_tests;
//#endregion TableTests
