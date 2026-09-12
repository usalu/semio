// #region minimap
/// 🗺️ Computed screen-space layout for one minimap frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimapLayout {
    pub panel: (f64, f64, f64, f64),
    pub world_min_x: f64,
    pub world_min_y: f64,
    pub scale: f64,
    pub map_origin_x: f64,
    pub map_origin_y: f64,
    pub viewport: (f64, f64, f64, f64),
}

/// 🗺️ Axis-aligned content bounds in world space (already padded by the caller).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimapContentBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

/// 🗺️ True when a `viewport_w`x`viewport_h` viewport at `camera_zoom` centered on
/// `(camera_x, camera_y)` already shows the whole of `content` (within `tolerance_px` screen
/// pixels) — the minimap should hide itself in this case.
pub fn content_fully_visible(content: &MinimapContentBounds, viewport_w: u32, viewport_h: u32, camera_x: f64, camera_y: f64, camera_zoom: f64, tolerance_px: f64) -> bool {
    let zoom = camera_zoom.max(1e-9);
    let half_w = viewport_w as f64 / (2.0 * zoom);
    let half_h = viewport_h as f64 / (2.0 * zoom);
    let tol = tolerance_px / zoom;
    camera_x - half_w <= content.min_x + tol && camera_x + half_w >= content.max_x - tol && camera_y - half_h <= content.min_y + tol && camera_y + half_h >= content.max_y - tol
}

/// 🗺️ Bottom-right inset panel of `panel_w`x`panel_h` with `margin` from the viewport edge, its
/// content scaled to fit `content_fit_ratio` (clamped `0.5..=0.98`) of the panel, plus the camera's
/// current view rect mapped into minimap-local coordinates.
#[allow(clippy::too_many_arguments, reason = "mirrors the dag board's own MinimapWidgetLayout inputs 1:1 — a struct would just move the same arity into a constructor")]
pub fn layout(content: &MinimapContentBounds, viewport_w: u32, viewport_h: u32, camera_x: f64, camera_y: f64, camera_zoom: f64, panel_w: f64, panel_h: f64, margin: f64, content_fit_ratio: f64) -> MinimapLayout {
    let ratio = content_fit_ratio.clamp(0.5, 0.98);
    let panel_x0 = viewport_w as f64 - margin - panel_w;
    let panel_y0 = viewport_h as f64 - margin - panel_h;
    let panel_x1 = panel_x0 + panel_w;
    let panel_y1 = panel_y0 + panel_h;
    let inset_x = panel_w * (1.0 - ratio) * 0.5;
    let inset_y = panel_h * (1.0 - ratio) * 0.5;
    let inner = (panel_x0 + inset_x, panel_y0 + inset_y, panel_x1 - inset_x, panel_y1 - inset_y);
    let inner_w = inner.2 - inner.0;
    let inner_h = inner.3 - inner.1;
    let cw = (content.max_x - content.min_x).max(1e-6);
    let ch = (content.max_y - content.min_y).max(1e-6);
    let scale = (inner_w / cw).min(inner_h / ch);
    let graph_w = cw * scale;
    let graph_h = ch * scale;
    let offset_x = inner.0 + (inner_w - graph_w) * 0.5;
    let offset_y = inner.1 + (inner_h - graph_h) * 0.5;
    let zoom = camera_zoom.max(1e-9);
    let half_w = viewport_w as f64 / (2.0 * zoom);
    let half_h = viewport_h as f64 / (2.0 * zoom);
    let view_min_x = camera_x - half_w;
    let view_min_y = camera_y - half_h;
    let view_max_x = camera_x + half_w;
    let view_max_y = camera_y + half_h;
    let to_mini = |wx: f64, wy: f64| (offset_x + (wx - content.min_x) * scale, offset_y + (wy - content.min_y) * scale);
    let (vx0, vy0) = to_mini(view_min_x, view_min_y);
    let (vx1, vy1) = to_mini(view_max_x, view_max_y);
    let viewport = (vx0.min(vx1), vy0.min(vy1), vx0.max(vx1), vy1.max(vy1));
    MinimapLayout { panel: (panel_x0, panel_y0, panel_x1, panel_y1), world_min_x: content.min_x, world_min_y: content.min_y, scale, map_origin_x: offset_x, map_origin_y: offset_y, viewport }
}

/// 🗺️ Inverse of `layout`'s world -> minimap mapping — a minimap-local screen point `(sx, sy)` back
/// to world coordinates, given the `world_min_x`/`world_min_y`/`scale`/`map_origin_x`/`map_origin_y`
/// a prior `layout()` call returned.
pub fn screen_to_world(map_origin_x: f64, map_origin_y: f64, world_min_x: f64, world_min_y: f64, scale: f64, sx: f64, sy: f64) -> (f64, f64) {
    (world_min_x + (sx - map_origin_x) / scale, world_min_y + (sy - map_origin_y) / scale)
}

/// 🗺️ Point-in-axis-aligned-rect test — shared by panel/viewport hit-testing.
pub fn point_in_rect((x0, y0, x1, y1): (f64, f64, f64, f64), sx: f64, sy: f64) -> bool {
    sx >= x0 && sx <= x1 && sy >= y0 && sy <= y1
}
// #endregion minimap
