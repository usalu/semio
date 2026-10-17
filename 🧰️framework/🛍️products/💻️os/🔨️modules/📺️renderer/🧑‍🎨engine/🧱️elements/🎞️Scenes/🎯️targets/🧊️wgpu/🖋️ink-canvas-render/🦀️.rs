//! 🖋️ Ink canvas rendering.

use super::*;

//#region InkCanvas
//#region InkCanvasState
/** 📝️ Pointer-down entry point for ink-canvas: mirrors handlePointerDown in ink-canvas-host.tsx. */

/** 📝️ Pointer-up entry point for ink-canvas: commits the active gesture and finalizes marquee selection. */

/** 📝️ Pointer-move hover entry point for ink-canvas: mirrors the `!dragState` hover branch of handlePointerMove. */

/** 📝️ Wheel entry point for ink-canvas: zoom-at-cursor, mirrors handleWheel in ink-canvas-host.tsx. */

//#endregion InkCanvasState

//#region InkCanvasRender
pub fn ink_effective_bounds(block: &Value, overrides: &BTreeMap<String, Value>) -> InkBoundsF {
    match overrides.get(ink_item_id(block)) {
        Some(over) => ink_item_bounds(over),
        None => ink_item_bounds(block),
    }
}

pub fn flatten_ink_items(blocks: &[Value]) -> Vec<&Value> {
    let mut out = Vec::new();
    fn visit<'a>(blocks: &'a [Value], out: &mut Vec<&'a Value>) {
        for block in blocks {
            out.push(block);
            if ink_item_kind(block) == "group" {
                if let Some(children) = block.get("children").and_then(Value::as_array) {
                    visit(children, out);
                }
            }
        }
    }
    visit(blocks, &mut out);
    out
}

#[cfg(any(test, feature = "testkit"))]
pub fn ink_items_at_point<'a>(blocks: &'a [Value], overrides: &BTreeMap<String, Value>, x: f64, y: f64) -> Vec<&'a Value> {
    let mut flat = flatten_ink_items(blocks);
    flat.reverse();
    flat.into_iter().filter(|block| ink_effective_bounds(block, overrides).contains_point(x, y)).collect()
}

pub fn ink_selection_bounds(blocks: &[Value], overrides: &BTreeMap<String, Value>, ids: &[String]) -> Option<InkBoundsF> {
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let selected: Vec<InkBoundsF> = flatten_ink_items(blocks).into_iter().filter(|block| id_set.contains(ink_item_id(block))).map(|block| ink_effective_bounds(block, overrides)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for bounds in &selected {
        min_x = min_x.min(bounds.x);
        min_y = min_y.min(bounds.y);
        max_x = max_x.max(bounds.x + bounds.w);
        max_y = max_y.max(bounds.y + bounds.h);
    }
    Some(InkBoundsF { x: min_x, y: min_y, w: (max_x - min_x).max(1.0), h: (max_y - min_y).max(1.0) })
}

pub(crate) fn ink_text_plain(block: &Value) -> String {
    block
        .get("paragraphs")
        .and_then(Value::as_array)
        .map(|paragraphs| {
            paragraphs.iter().map(|paragraph| paragraph.get("runs").and_then(Value::as_array).map(|runs| runs.iter().filter_map(|run| run.get("text").and_then(Value::as_str)).collect::<String>()).unwrap_or_default()).collect::<Vec<_>>().join("\n")
        })
        .unwrap_or_default()
}

pub fn ink_world_to_screen(camera: InkCameraF, inner: Rect, wx: f64, wy: f64) -> (f32, f32) {
    (inner.x + (wx * camera.zoom + camera.x) as f32, inner.y + (wy * camera.zoom + camera.y) as f32)
}

pub fn ink_resize_handle_screen_pos(handle: &str, sx: f32, sy: f32, w: f32, h: f32, size: f32) -> (f32, f32) {
    let half = size * 0.5;
    let x = if handle.contains('w') {
        sx - half
    } else if handle.contains('e') {
        sx + w - half
    } else {
        sx + w * 0.5 - half
    };
    let y = if handle.contains('n') {
        sy - half
    } else if handle.contains('s') {
        sy + h - half
    } else {
        sy + h * 0.5 - half
    };
    (x, y)
}

fn draw_ink_table(ctx: &mut FrameworkWidgetContext<'_>, block: &Value, sx: f32, sy: f32, w: f32, h: f32, theme: &Theme) {
    let columns: Vec<String> = block.get("columns").and_then(Value::as_array).map(|c| c.iter().filter_map(|v| v.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let rows: Vec<Vec<String>> = block
        .get("rows")
        .and_then(Value::as_array)
        .map(|rows| rows.iter().map(|row| row.as_array().map(|cells| cells.iter().map(|cell| cell.get("content").and_then(Value::as_str).unwrap_or("").to_string()).collect()).unwrap_or_default()).collect())
        .unwrap_or_default();
    let col_count = columns.len().max(1);
    let row_count = rows.len() + 1;
    let col_w = w / col_count as f32;
    let row_h = h / row_count as f32;
    let font = theme.font_size_small.min(row_h * 0.6).max(6.0);
    for (index, label) in columns.iter().enumerate() {
        draw_text(ctx, label, sx + index as f32 * col_w + 3.0, sy + row_h * 0.7, font, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let ry = sy + (row_index + 1) as f32 * row_h;
        for (col_index, cell) in row.iter().enumerate() {
            draw_text(ctx, cell, sx + col_index as f32 * col_w + 3.0, ry + row_h * 0.7, font, theme.text);
        }
    }
    for index in 0..=col_count {
        let x = sx + index as f32 * col_w;
        ctx.draw.push_line(x, sy, x, sy + h, theme.separator, 0.5);
    }
    for index in 0..=row_count {
        let y = sy + index as f32 * row_h;
        ctx.draw.push_line(sx, y, sx + w, y, theme.separator, 0.5);
    }
}

fn draw_ink_image(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, doc: &InkDocumentJson, sx: f32, sy: f32, w: f32, h: f32) {
    let theme = ctx.theme;
    let image_key = ink_item_str(block, "imageKey");
    if let Some(asset) = doc.assets.get(image_key) {
        let mime = asset.get("mime").and_then(Value::as_str).unwrap_or("image/png");
        let data = asset.get("data").and_then(Value::as_str).unwrap_or("");
        let data_url = if data.starts_with("data:") { data.to_string() } else { format!("data:{mime};base64,{data}") };
        if let Some(key) = queue_canvas_image_upload(&scene.host_id, ink_item_id(block), &data_url) {
            ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], 1.0);
            return;
        }
    }
    draw_text(ctx, image_key, sx + 6.0, sy + h * 0.5, theme.font_size_small, theme.text_muted);
}

/// 🎨️ `bg-background/90` in `ink-canvas-host.tsx` — `theme.panel` (the app-chrome surface token)
/// previously stood in for the canvas-item card token, which is `background`, not `panel`.
fn draw_ink_item(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, camera: InkCameraF, inner: Rect, doc: &InkDocumentJson, selected: bool, hovered: bool) {
    let theme = ctx.theme;
    let kind = ink_item_kind(block);
    let bounds = ink_item_bounds(block);
    let (sx, sy) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;

    if kind == "stroke" {
        let points = ink_points(block);
        if points.len() >= 2 {
            let color = block
                .get("color")
                .and_then(Value::as_array)
                .map(|c| {
                    let get = |i: usize| c.get(i).and_then(Value::as_f64).unwrap_or(0.0) as f32;
                    Rgba::new(get(0), get(1), get(2), get(3))
                })
                .unwrap_or(theme.text);
            let stroke_width = (ink_item_num(block, "strokeWidth") as f32 * camera.zoom as f32).max(1.0);
            let screen_points: Vec<(f32, f32)> = points.iter().map(|p| ink_world_to_screen(camera, inner, p.0, p.1)).collect();
            for pair in screen_points.windows(2) {
                ctx.draw.push_line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, color, stroke_width);
            }
        }
        return;
    }

    let bg = theme.background;
    ctx.draw.push_rounded([sx, sy, w.max(4.0), h.max(4.0)], bg.with_alpha(0.9), theme.border_radius.min(6.0));

    match kind {
        "text" => {
            let text = ink_text_plain(block);
            let font_size = (ink_item_num(block, "fontSize").max(8.0) as f32 * camera.zoom as f32).max(6.0);
            draw_text_wrapped(ctx, &text, sx + 6.0, sy + 4.0, (w - 12.0).max(1.0), font_size, theme.text);
        }
        "math" => {
            let tex = ink_item_str(block, "tex");
            draw_text(ctx, tex, sx + 8.0, sy + h * 0.5 + 4.0, theme.font_size_body.max(8.0), theme.text);
        }
        "table" => draw_ink_table(ctx, block, sx, sy, w.max(4.0), h.max(4.0), theme),
        "image" => draw_ink_image(ctx, scene, block, doc, sx, sy, w.max(4.0), h.max(4.0)),
        "group" => {
            let children_len = block.get("children").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
            draw_text(ctx, &format!("Group · {children_len} children"), sx + 6.0, sy + 16.0, theme.font_size_small, theme.text_muted);
        }
        _ => {}
    }

    let border = if selected {
        theme.accent
    } else if hovered {
        theme.accent.with_alpha(theme.accent.a * 0.6)
    } else {
        theme.panel_border
    };
    let border_w = if selected { 2.0 } else { 1.0 };
    draw_ink_rect_outline(ctx.draw, sx, sy, w.max(4.0), h.max(4.0), border, border_w);
}

fn render_ink_edit(scene: &UiComponentSceneNode, camera: InkCameraF, inner: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let Some(edit) = scene_state(&scene.host_id).ink_edit else { return };
    let rect = ink_edit_screen_rect(&edit, camera, inner);
    let control_id = ink_edit_control_id(&scene.host_id, &edit);
    let block_id = ink_item_id(&edit.block).to_owned();
    let kind = edit.kind;
    mutate_scene_state(&scene.host_id, |state| {
        if let Some(current) = state.ink_edit.as_mut().filter(|current| ink_item_id(&current.block) == block_id && current.kind == kind) {
            current.screen_rect = rect;
        }
    });
    ctx.draw.push_rounded([rect.x, rect.y, rect.w.max(4.0), rect.h.max(4.0)], ctx.theme.background.with_alpha(0.97), ctx.theme.border_radius * 0.5);
    draw_ink_rect_outline(ctx.draw, rect.x, rect.y, rect.w.max(4.0), rect.h.max(4.0), ctx.theme.accent, 2.0);
    draw_text_wrapped(ctx, &edit.draft, rect.x + 6.0, rect.y + 4.0, (rect.w - 12.0).max(1.0), ctx.theme.font_size_small, ctx.theme.text);
    let caret_x = (rect.x + 6.0 + edit.draft.lines().last().unwrap_or_default().chars().count() as f32 * ctx.theme.font_size_small * 0.55).min(rect.x + rect.w - 3.0);
    ctx.draw.push_line(caret_x, rect.y + 4.0, caret_x, rect.y + rect.h - 4.0, ctx.theme.accent, 1.0);
    ctx.input.register_hit(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Input, drag_axis: None, drag_data: None });
}

fn ink_item_visible(block: &Value) -> bool {
    block.get("visible").and_then(Value::as_bool).unwrap_or(true)
}

fn positive_mod_f32(v: f32, m: f32) -> f32 {
    if m <= 0.0 {
        0.0
    } else {
        ((v % m) + m) % m
    }
}

fn draw_ink_grid(draw: &mut ui_wgpu::wgpu::DrawList, camera: InkCameraF, inner: Rect, theme: &Theme, spacing: f64, subdivisions: u32, opacity: f64) {
    let major_px = (spacing * camera.zoom) as f32;
    if major_px < 2.0 {
        return;
    }
    let minor_px = major_px / subdivisions.max(1) as f32;
    let offset_x = positive_mod_f32(camera.x as f32, major_px);
    let offset_y = positive_mod_f32(camera.y as f32, major_px);
    let color = theme.separator.with_alpha((theme.separator.a * opacity as f32).max(0.05));
    let minor_color = color.with_alpha(color.a * 0.55);

    let mut wx = inner.x + positive_mod_f32(offset_x, major_px) - major_px;
    while wx < inner.x + inner.w {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let mx = wx + s as f32 * minor_px;
                if mx >= inner.x && mx <= inner.x + inner.w {
                    draw.push_line(mx, inner.y, mx, inner.y + inner.h, minor_color, 0.5);
                }
            }
        }
        if wx >= inner.x && wx <= inner.x + inner.w {
            draw.push_line(wx, inner.y, wx, inner.y + inner.h, color, 1.0);
        }
        wx += major_px;
    }
    let mut wy = inner.y + positive_mod_f32(offset_y, major_px) - major_px;
    while wy < inner.y + inner.h {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let my = wy + s as f32 * minor_px;
                if my >= inner.y && my <= inner.y + inner.h {
                    draw.push_line(inner.x, my, inner.x + inner.w, my, minor_color, 0.5);
                }
            }
        }
        if wy >= inner.y && wy <= inner.y + inner.h {
            draw.push_line(inner.x, wy, inner.x + inner.w, wy, color, 1.0);
        }
        wy += major_px;
    }
}

fn draw_ink_selection_chrome(draw: &mut ui_wgpu::wgpu::DrawList, theme: &Theme, camera: InkCameraF, inner: Rect, bounds: InkBoundsF, show_handles: bool) {
    let (sx, sy) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;
    draw_ink_rect_outline(draw, sx, sy, w, h, theme.accent, 1.5);
    if !show_handles {
        return;
    }
    let handle_size = 8.0;
    for handle in INK_RESIZE_HANDLES {
        let (hx, hy) = ink_resize_handle_screen_pos(handle, sx, sy, w, h, handle_size);
        draw.push_rounded([hx, hy, handle_size, handle_size], theme.background, 1.0);
        draw_ink_rect_outline(draw, hx, hy, handle_size, handle_size, theme.accent, 1.0);
    }
}

pub fn render_ink_canvas(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(ink) = &scene.ink_canvas else {
        return render_placeholder("ink-canvas", bounds, ctx);
    };
    let doc: InkDocumentJson = serde_json::from_str(&ink.document_json).unwrap_or_default();
    let selected_ids: Vec<String> = serde_json::from_str(&ink.selection_json).unwrap_or_default();
    let selected_set: HashSet<&str> = selected_ids.iter().map(String::as_str).collect();
    let hovered_id = ink.hovered_id.clone();
    let is_navigator = ink.view_mode == "navigator";
    let inner = bounds;

    let state = scene_state(&scene.host_id);
    let camera = state.ink_camera.map(|(x, y, zoom)| InkCameraF { x, y, zoom }).unwrap_or_else(|| InkCameraF::from(doc.camera.clone()));

    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    ctx.draw.push_scissor(inner);

    if doc.grid_visible.unwrap_or(true) && !is_navigator {
        draw_ink_grid(ctx.draw, camera, inner, theme, doc.grid_spacing.unwrap_or(32.0), doc.grid_subdivisions.unwrap_or(4.0).max(1.0) as u32, doc.grid_opacity.unwrap_or(0.35));
    }

    let overrides = state.ink_overrides.clone();
    let hidden_text_id = state.ink_edit.as_ref().and_then(|edit| matches!(edit.kind, InkEditKind::Text).then(|| ink_item_id(&edit.block).to_owned()));
    let blocks = flatten_ink_items(&doc.blocks);
    for block in blocks.iter().copied() {
        let effective = overrides.get(ink_item_id(block)).unwrap_or(block);
        if !ink_item_visible(effective) || hidden_text_id.as_deref() == Some(ink_item_id(block)) {
            continue;
        }
        let id = ink_item_id(block);
        let selected = selected_set.contains(id);
        let hovered = hovered_id.as_deref() == Some(id);
        draw_ink_item(ctx, scene, effective, camera, inner, &doc, selected, hovered);
    }

    let selection_bounds = ink_selection_bounds(&doc.blocks, &overrides, &selected_ids);
    let utility = doc.active_utility.clone().unwrap_or_else(|| "selectDirect".into());
    let show_handles = !is_navigator && (utility == "selectDirect" || utility == "selectMarquee") && selection_bounds.is_some() && !selected_ids.is_empty();
    if let Some(sel) = selection_bounds {
        draw_ink_selection_chrome(ctx.draw, theme, camera, inner, sel, show_handles);
    }

    if state.ink_marquee_points.len() >= 2 {
        let points: Vec<[f32; 2]> = state.ink_marquee_points.iter().map(|p| [p.0, p.1]).collect();
        ui_wgpu::wgpu::paint_selection_marquee(ctx.draw, theme, false, false, &points, false);
    }

    ctx.draw.pop_scissor();

    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.host_id.clone()), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    render_ink_edit(scene, camera, inner, ctx);
}

//#endregion InkCanvasRender

//#region InkCanvasTests
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs"]
mod ink_canvas_tests;
//#endregion InkCanvasTests
//#endregion InkCanvas
