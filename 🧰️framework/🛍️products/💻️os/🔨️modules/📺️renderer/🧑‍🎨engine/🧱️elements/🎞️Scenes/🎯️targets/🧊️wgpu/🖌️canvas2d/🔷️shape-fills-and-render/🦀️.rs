//! 🔷️ Shape fills and render: the shape fill and outline pushes, the gradient fills, the packet item render, the framing and `render_canvas_2d`. See [`super`].

use super::*;

//#region Canvas2d
fn push_shape_fill(draw: &mut semio_framework_ui_draw_contract::DrawList, rect: Rect, color: Rgba, is_circle: bool) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        draw.push_triangle_fan(&canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    } else {
        draw.push_rounded([rect.x, rect.y, rect.w, rect.h], color, 4.0);
    }
}

fn push_circle_outline(draw: &mut semio_framework_ui_draw_contract::DrawList, cx: f32, cy: f32, radius: f32, color: Rgba, width: f32) {
    let points = canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS);
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        draw.push_line(a[0], a[1], b[0], b[1], color, width);
    }
}

fn push_shape_outline(draw: &mut semio_framework_ui_draw_contract::DrawList, rect: Rect, color: Rgba, width: f32, is_circle: bool, dash: Option<&[f64]>) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        push_circle_outline(draw, cx, cy, radius, color, width);
        return;
    }
    if dash.is_some_and(|d| !d.is_empty()) {
        draw_dashed_line(draw, rect.x, rect.y, rect.x + rect.w, rect.y, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y, rect.x + rect.w, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y + rect.h, rect.x, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x, rect.y + rect.h, rect.x, rect.y, color, width);
    } else {
        draw_ink_rect_outline(draw, rect.x, rect.y, rect.w, rect.h, color, width);
    }
}

pub fn push_linear_gradient_fill(draw: &mut semio_framework_ui_draw_contract::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (sx1, sy1) = viewport.world_to_screen((origin_x + fill.x1) as f32, (origin_y + fill.y1) as f32, inner);
    let (sx2, sy2) = viewport.world_to_screen((origin_x + fill.x2) as f32, (origin_y + fill.y2) as f32, inner);
    let dx = sx2 - sx1;
    let dy = sy2 - sy1;
    let len = (dx * dx + dy * dy).sqrt();
    draw.push_scissor(clip);
    if len < 0.5 {
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, 1.0, opacity));
        draw.push_rounded([clip.x, clip.y, clip.w, clip.h], color, 0.0);
        draw.pop_scissor();
        return;
    }
    let ux = dx / len;
    let uy = dy / len;
    let px = -uy;
    let py = ux;
    let overhang = (clip.w + clip.h).max(32.0);
    for i in 0..CANVAS_GRADIENT_BANDS {
        let t0 = i as f32 / CANVAS_GRADIENT_BANDS as f32;
        let t1 = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, (t0 + t1) * 0.5, opacity));
        let a0 = if i == 0 { -overhang } else { t0 * len };
        let a1 = if i == CANVAS_GRADIENT_BANDS - 1 { len + overhang } else { t1 * len };
        let bx = sx1 + ux * a0;
        let by = sy1 + uy * a0;
        let ex = sx1 + ux * a1;
        let ey = sy1 + uy * a1;
        let points = [[bx + px * overhang, by + py * overhang], [ex + px * overhang, ey + py * overhang], [ex - px * overhang, ey - py * overhang], [bx - px * overhang, by - py * overhang]];
        draw.push_triangle_fan(&points, color);
    }
    draw.pop_scissor();
}

pub fn push_radial_gradient_fill(draw: &mut semio_framework_ui_draw_contract::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (scx, scy) = viewport.world_to_screen((origin_x + fill.cx) as f32, (origin_y + fill.cy) as f32, inner);
    let sr = (fill.r as f32 * viewport.zoom).max(0.5);
    draw.push_scissor(clip);
    let outer_radius = sr.max(clip.w.max(clip.h));
    for i in (0..CANVAS_GRADIENT_BANDS).rev() {
        let t = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let radius = if i == CANVAS_GRADIENT_BANDS - 1 { outer_radius } else { sr * t };
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, t, opacity));
        draw.push_triangle_fan(&canvas_circle_points(scx, scy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    }
    draw.pop_scissor();
}

#[allow(clippy::too_many_arguments, reason = "one arg per resolved paint/geometry input; grouping into a struct is a T2 restructure, out of scope")]
pub fn render_canvas_shape_fill(draw: &mut semio_framework_ui_draw_contract::DrawList, viewport: &Viewport, inner: Rect, shape_rect: Rect, layer: &CanvasLayer, opacity: f32, fallback_fill: Rgba, fallback_stroke: Rgba, backdrop: Rgba, is_circle: bool) {
    let blend = layer.blend_mode.as_deref();
    match &layer.fill {
        Some(fill) if fill.kind.as_deref() == Some("linearGradient") && !fill.stops.is_empty() => {
            push_linear_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.kind.as_deref() == Some("radialGradient") && !fill.stops.is_empty() => {
            push_radial_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.color.is_some() => {
            let solid = canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(fill.color.as_deref().unwrap_or(&[]), opacity));
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
        _ => {
            let solid = canvas_apply_blend_mode(blend, backdrop, fallback_fill);
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
    }
    if let Some(stroke) = &layer.stroke {
        let color = stroke.color.as_deref().map(|c| canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(c, opacity))).unwrap_or_else(|| fallback_stroke.with_alpha(fallback_stroke.a * opacity));
        let width = (stroke.width.unwrap_or(1.0) as f32).max(1.0);
        push_shape_outline(draw, shape_rect, color, width, is_circle, stroke.dash.as_deref());
    }
}

fn render_canvas2d_packet_item(item: &Canvas2dPacketItem<'_>, viewport: &Viewport, inner: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let color = if item.id.starts_with("residual-field-") {
        theme.diagram_field_residual
    } else if item.id.starts_with("reaction-field-") || item.id.starts_with("load-") {
        theme.diagram_field_reaction
    } else if item.id.starts_with("displacement-field-") || item.id.starts_with("mode-field-") {
        theme.diagram_field_displacement
    } else {
        theme.diagram_accent
    };
    if item.kind == "line" {
        let (x0, y0) = viewport.world_to_screen(item.x0 as f32, item.y0 as f32, inner);
        let (x1, y1) = viewport.world_to_screen(item.x1 as f32, item.y1 as f32, inner);
        ctx.draw.push_line(x0, y0, x1, y1, color, (2.0 * viewport.zoom).max(1.0));
    } else if item.kind == "circle" {
        let (x, y) = viewport.world_to_screen(item.x as f32, item.y as f32, inner);
        let width = (item.width as f32 * viewport.zoom).max(4.0);
        let height = (item.height as f32 * viewport.zoom).max(4.0);
        ctx.draw.push_solid([x, y, width, height], color);
    } else if item.kind == "text" {
        let Some(text) = item.text.as_ref() else { return };
        let (x, y) = viewport.world_to_screen(item.x as f32, item.y as f32, inner);
        draw_text(ctx, text.content, x, y + text.size as f32, (text.size as f32).max(8.0), theme.text);
    }
}

/// 📷️ Resolves one framing revision after layout and publishes its resulting local camera.
pub fn apply_canvas_framing(scene: &UiComponentSceneNode, bounds: Rect) -> bool {
    let Some(request) = scene.canvas_2d.as_ref().and_then(|canvas| canvas.framing.as_ref()) else { return false };
    if scene_state(&scene.host_id).canvas_framing_revision == Some(request.revision) {
        return false;
    }
    let Some(camera) = request.fit(bounds.w as f64, bounds.h as f64) else { return false };
    let viewport = Viewport { x: camera.x as f32, y: camera.y as f32, zoom: camera.zoom as f32 };
    if ![viewport.x, viewport.y, viewport.zoom].iter().all(|value| value.is_finite()) {
        return false;
    }
    mutate_scene_state(&scene.host_id, |state| {
        state.viewport = viewport;
        state.canvas_framing_revision = Some(request.revision);
        state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
    });
    schedule_scene_camera_dispatch(&scene.host_id, &scene.surface_id);
    true
}

/// 🗒️ `role === "meta"` (activeUtility bookkeeping) and `visible === false` records are
/// non-visual — skip rendering entirely, matches `layers.filter(role !== "meta")` in
/// `canvas-2d-host.tsx`'s `JsonLayersCanvasSession.renderFrame`.
/// 🖼️ Generic bounds-rect (or `kind === "circle"`) draw record — resolves solid/gradient
/// fill, blend-mode approximation, and stroke, matching `drawSceneNode`'s bounds-layer path.
/// 🖊️ Overlay annotation: a two-pass selection highlight (soft outer glow + crisp amber ring)
/// drawn on top of the shape, matches `drawBoundsLayer`'s `isSelected` glow+ring pair in
/// `canvas-2d-host.tsx` (glow at +4px/width 5, ring at +0px/width 2.5, both amber).
/// 🫙️ `layers.length === 0 → ctx.fillText("Empty canvas", -36, 0)` in
/// `🧱️elements/📐️Canvas2dHost/🟦️.tsx`'s `renderFrame` — React counts the same `role !== "meta"`
/// records this loop iterates, so the emptiness verdict matches without a second parse.
pub fn render_canvas_2d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(canvas) = &scene.canvas_2d else {
        return render_placeholder("canvas-2d", bounds, ctx);
    };
    let inner = bounds;
    let mut order = CanvasPaintOrder::open(ctx.draw, inner);
    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let mut viewport = Viewport { x: canvas.camera_x as f32, y: canvas.camera_y as f32, zoom: canvas.zoom as f32 };
    apply_canvas_framing(scene, inner);
    let local = scene_state(&scene.host_id);
    if local.viewport.zoom > 0.0 && scene.component_kind == SurfaceKind::Canvas2d {
        viewport = local.viewport;
    }
    if let Some(snapshot) = canvas.snapshot {
        draw_canvas_infinite_grid(ctx.draw, &viewport, inner, theme);
        order.vector();
        for page_index in 0..snapshot.page_count {
            let _ = semio_framework_ui_declaration::ui::canvas2d_snapshot_with_page(snapshot, page_index, |page| {
                for item in serde_json::Deserializer::from_slice(page.bytes()).into_iter::<Canvas2dPacketItem<'_>>().flatten() {
                    if item.kind == "line" {
                        render_canvas2d_packet_item(&item, &viewport, inner, ctx);
                        order.vector();
                    } else {
                        order.quad(ctx.draw);
                        render_canvas2d_packet_item(&item, &viewport, inner, ctx);
                    }
                }
            });
        }
        semio_framework_os_renderer_wgpu_canvas2d_host::paint(scene, inner, (f64::from(viewport.x), f64::from(viewport.y), f64::from(viewport.zoom)), ctx.theme, ctx.draw);
        order.close(ctx.draw);
        ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.host_id.clone()), kind: HitKind::Generic, drag_axis: Some(DragAxis::Both), drag_data: None });
        return;
    }
    let layers: Vec<CanvasLayer> = serde_json::from_str(&canvas.layers_json).unwrap_or_default();
    if layers.iter().any(|layer| layer.kind == "polyline") {
        draw_checkerboard(ctx.draw, &viewport, inner, ctx.theme, 1024.0);
    }
    draw_canvas_infinite_grid(ctx.draw, &viewport, inner, theme);
    order.vector();
    for (index, layer) in layers.iter().enumerate() {
        if !canvas_layer_should_render(layer) {
            continue;
        }
        if canvas_layer_is_scene_node(layer) {
            render_canvas_scene_node(scene, layer, &viewport, inner, &mut order, ctx);
            continue;
        }
        let opacity = layer.opacity.unwrap_or(1.0).clamp(0.0, 1.0);
        let blend = layer.blend_mode.as_deref();
        if layer.kind == "image" && layer.data_url.is_some() {
            if let Some(data_url) = layer.data_url.as_deref().filter(|src| src.starts_with("data:")) {
                if let Some(key) = queue_canvas_image_upload(&scene.host_id, &layer.id, data_url) {
                    let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
                    let w = layer.width as f32 * viewport.zoom;
                    let h = layer.height as f32 * viewport.zoom;
                    ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], opacity);
                }
            }
            continue;
        }
        if let Some(points) = layer.points.as_ref().filter(|points| layer.kind == "polyline" && !points.is_empty()) {
            let stroke = theme.diagram_stroke.with_alpha(theme.diagram_stroke.a * opacity);
            let seam_stroke = theme.diagram_seam.with_alpha(theme.diagram_seam.a * opacity);
            let width = (1.5 * viewport.zoom).max(1.0);
            for (edge_index, chunk) in points.chunks(2).enumerate() {
                if chunk.len() < 2 {
                    continue;
                }
                let (x0, y0) = viewport.world_to_screen(chunk[0][0] as f32, chunk[0][1] as f32, inner);
                let (x1, y1) = viewport.world_to_screen(chunk[1][0] as f32, chunk[1][1] as f32, inner);
                let is_seam = layer.seams.as_ref().and_then(|seams| seams.get(edge_index)).copied().unwrap_or(0) != 0;
                if is_seam {
                    draw_dashed_line(ctx.draw, x0, y0, x1, y1, seam_stroke, width);
                } else {
                    ctx.draw.push_line(x0, y0, x1, y1, stroke, width);
                }
            }
            order.vector();
            continue;
        }
        let hue = (index * 47 % 360) as f32;
        if layer.kind == "line" || layer.x0.is_some() {
            let x0 = layer.x0.unwrap_or(layer.x) as f32;
            let y0 = layer.y0.unwrap_or(layer.y) as f32;
            let x1 = layer.x1.unwrap_or(layer.x + layer.width) as f32;
            let y1 = layer.y1.unwrap_or(layer.y + layer.height) as f32;
            let (sx0, sy0) = viewport.world_to_screen(x0, y0, inner);
            let (sx1, sy1) = viewport.world_to_screen(x1, y1, inner);
            let base_stroke = layer
                .stroke
                .as_ref()
                .and_then(|stroke| stroke.color.as_deref())
                .map(|c| canvas_color_channels(c, opacity))
                .unwrap_or_else(|| Rgba::new(theme.diagram_accent.r + hue / 720.0, theme.diagram_accent.g, theme.diagram_accent.b, theme.diagram_accent.a * opacity));
            let stroke = canvas_apply_blend_mode(blend, theme.canvas_clear, base_stroke);
            ctx.draw.push_line(sx0, sy0, sx1, sy1, stroke, (2.0 * viewport.zoom).max(1.0));
            order.vector();
            continue;
        }
        let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
        let w = (layer.width as f32 * viewport.zoom).max(8.0);
        let h = (layer.height as f32 * viewport.zoom).max(8.0);
        let shape_rect = Rect::new(sx, sy, w, h);
        let is_circle = layer.kind == "circle";
        let fallback_fill = Rgba::new(theme.diagram_accent_fill.r + hue / 720.0, theme.diagram_accent_fill.g, theme.diagram_accent_fill.b, theme.diagram_accent_fill.a * opacity);
        order.quad(ctx.draw);
        render_canvas_shape_fill(ctx.draw, &viewport, inner, shape_rect, layer, opacity, fallback_fill, theme.diagram_shape_outline, theme.canvas_clear, is_circle);
        if layer.selected.unwrap_or(false) {
            if is_circle {
                let cx = sx + w * 0.5;
                let cy = sy + h * 0.5;
                let r = w.min(h) * 0.5;
                push_circle_outline(ctx.draw, cx, cy, r + 4.0, CANVAS2D_SELECTION_GLOW, 5.0);
                push_circle_outline(ctx.draw, cx, cy, r, CANVAS2D_SELECTION_RING, 2.5);
            } else {
                draw_ink_rect_outline(ctx.draw, sx - 4.0, sy - 4.0, w + 8.0, h + 8.0, CANVAS2D_SELECTION_GLOW, 5.0);
                draw_ink_rect_outline(ctx.draw, sx, sy, w, h, CANVAS2D_SELECTION_RING, 2.5);
            }
        }
        order.vector();
        let label = if layer.name.is_empty() { layer.id.as_str() } else { layer.name.as_str() };
        if !label.is_empty() {
            order.quad(ctx.draw);
            draw_text(ctx, label, sx + 4.0, sy + 14.0, theme.font_size_small, theme.text);
        }
    }
    if !layers.iter().any(|layer| layer.role.as_deref() != Some("meta")) {
        order.quad(ctx.draw);
        draw_text(ctx, CANVAS_2D_EMPTY_LABEL, inner.x + inner.w * 0.5 - 36.0, inner.y + inner.h * 0.5, theme.font_size_small, theme.text_muted);
    }
    semio_framework_os_renderer_wgpu_canvas2d_host::paint(scene, inner, (f64::from(viewport.x), f64::from(viewport.y), f64::from(viewport.zoom)), ctx.theme, ctx.draw);
    order.close(ctx.draw);
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.host_id.clone()), kind: HitKind::Generic, drag_axis: Some(DragAxis::Both), drag_data: None });
}
//#endregion Canvas2d
