//! ✂️ Culling, markers and snap of the board scene: the world bounds culls, handle markers, node icon paint, the wheel zoom flag, link snap tolerance and commit proximity, node drag proximity and the planned link edge id. See [`super`].

use super::*;

pub fn handle_world_bounds_cull(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<WorldBox> {
    let pos = handle_world_pos(scene, style, h)?;
    let pad = scene.drawable_cull_pad_world() + effective_handle_radius(scene, style, h).max(1.0);
    Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
}

pub fn indirect_handle_world_bounds_cull(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<WorldBox> {
    let pos = indirect_handle_world_pos(scene, style, h)?;
    let pad = scene.drawable_cull_pad_world() + indirect_handle_marker_radius_world(scene, style, h).max(1.0);
    Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
}

pub fn edge_world_bounds_for_cull(scene: &BoardSceneState, style: &BoardStyleState, e: &EdgeData) -> Option<WorldBox> {
    let c = board_edge_curve(scene, style, e)?;
    let axis = cubic_bezier_axis_bounds(c);
    let half_w_world = scene.camera.zoom.max(0.75) / scene.camera.zoom.max(1e-9);
    Some(inflate_world_box(axis, half_w_world + scene.drawable_cull_pad_world()))
}

pub fn append_handle_marker(
    scene_state: &BoardSceneState, style: &BoardStyleState,
    scene: &mut Scene,
    h: &HandleData,
    center: Point,
    radius_world: f64,
    draw_icon: bool,
    style_kind: BoardElementStyleKind,
    paint_override: Option<(Color, Color, f64)>,
    world_space: bool,
    layer: NodeHandlePaintLayer,
    exterior_cap: bool,
) {
    let c = scene_state.draw_space_point(center, world_space);
    let r = scene_state.draw_space_len(radius_world, world_space);
    let (fill, stroke_c, stroke_px) =
        if let Some((f, s, sw)) = paint_override { (f, s, sw) } else { (style.resolve_handle_fill_color(h, &style.canvas_theme, style_kind), resolve_handle_stroke_color(h, &style.canvas_theme, style_kind), 2.0_f64) };
    let paint_fill = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill);
    let paint_stroke = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
    let paint_icons = draw_icon && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
    let outward = if exterior_cap { scene_state.nodes.get(h.node_id.as_str()).and_then(|n| handle_outward_at_node_rim(center, Point::new(n.x, n.y), n.shape, n.radius, n.width, n.height)) } else { None };
    if paint_fill {
        if let Some(out) = outward {
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &handle_exterior_cap_fill_path(c, out, r));
        } else {
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &Circle::new(c, r));
        }
    }
    if paint_stroke {
        if let Some(out) = outward {
            scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &handle_exterior_cap_stroke_path(c, out, r));
        } else {
            scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &Circle::new(c, r));
        }
    }
    if paint_icons {
        if let Some(k) = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let preserve_original_style = style.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
            let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&style.canvas_theme);
            if let Some(paint) = style.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
                let (bx, by, bw, bh) = paint.bounds();
                let fit_inset = 0.62;
                let s = scene_state.draw_space_len(radius_world, world_space) * fit_inset;
                let cx = bx + bw * 0.5;
                let cy = by + bh * 0.5;
                let avail = 2.0 * s;
                let scale = (avail / bw).min(avail / bh);
                let aff = Affine::IDENTITY.translate((c.x - scale * cx, c.y - scale * cy)) * Affine::IDENTITY.scale(scale);
                let r_clip = scene_state.draw_space_len(radius_world, world_space) * 0.82;
                let disc = Circle::new(c, r_clip);
                scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                match paint.body() {
                    CachedIconBody::Vector(icon_scene) => {
                        scene.append(icon_scene, Some(aff));
                    }
                    CachedIconBody::Raster(img) => {
                        scene.draw_image(img, aff);
                    }
                }
                scene.pop_layer();
            }
        }
    }
}

pub fn paint_node_icon(scene_state: &BoardSceneState, style: &BoardStyleState, scene: &mut Scene, n: &NodeData, world_space: bool, style_kind: BoardElementStyleKind, circle_clip: Option<&Circle>, rect_clip: Option<Rect>) {
    if let Some(k) = n.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let preserve_original_style = style.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
        let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&style.canvas_theme);
        if let Some(paint) = style.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
            let (bx, by, bw, bh) = paint.bounds();
            let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
            let fit_inset = ui_styling::metrics::icon::FIT_INSET;
            let (sx_half, sy_half) = match n.shape {
                NodeShape::Circle => {
                    let s = scene_state.draw_space_len(style.scaled_node_radius(n), world_space) * fit_inset;
                    (s, s)
                }
                NodeShape::Rectangle => (scene_state.draw_space_len(style.scaled_node_width(n), world_space) * fit_inset * 0.5, scene_state.draw_space_len(style.scaled_node_height(n), world_space) * fit_inset * 0.5),
            };
            let center = scene_state.draw_space_point(Point::new(n.x, n.y), world_space);
            let cx = bx + bw * 0.5;
            let cy = by + bh * 0.5;
            let avail_w = 2.0 * sx_half;
            let avail_h = 2.0 * sy_half;
            let scale = (avail_w / bw).min(avail_h / bh);
            let aff = Affine::IDENTITY.translate((center.x - scale * cx, center.y - scale * cy)) * Affine::IDENTITY.scale(scale);
            match n.shape {
                NodeShape::Circle => {
                    let r_clip = scene_state.draw_space_len(style.scaled_node_radius(n), world_space) * clip_inset;
                    let disc = circle_clip.copied().unwrap_or_else(|| Circle::new(center, r_clip));
                    scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                    match paint.body() {
                        CachedIconBody::Vector(icon_scene) => {
                            scene.append(icon_scene, Some(aff));
                        }
                        CachedIconBody::Raster(img) => {
                            scene.draw_image(img, aff);
                        }
                    }
                    scene.pop_layer();
                }
                NodeShape::Rectangle => {
                    let hw = scene_state.draw_space_len(style.scaled_node_width(n), world_space) * clip_inset * 0.5;
                    let hh = scene_state.draw_space_len(style.scaled_node_height(n), world_space) * clip_inset * 0.5;
                    let clip_r = rect_clip.unwrap_or_else(|| Rect::from_points(Point::new(center.x - hw, center.y - hh), Point::new(center.x + hw, center.y + hh)));
                    scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
                    match paint.body() {
                        CachedIconBody::Vector(icon_scene) => {
                            scene.append(icon_scene, Some(aff));
                        }
                        CachedIconBody::Raster(img) => {
                            scene.draw_image(img, aff);
                        }
                    }
                    scene.pop_layer();
                }
            }
        }
    }
}

pub fn board_set_wheel_zoom_active(scene: &mut BoardSceneState, style: &mut BoardStyleState, active: bool) {
    if active && !style.wheel_zoom_active {
        style.wheel_zoom_render_lod = Some(current_draw_lod(scene, style));
    }
    if !active {
        style.wheel_zoom_render_lod = None;
    }
    style.wheel_zoom_active = active;
}

fn link_snap_drag_tolerance_screen(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> f64 {
    let z = scene.camera.zoom.max(1e-9);
    HANDLE_HIT_TOLERANCE_PX + LINK_HANDLE_SNAP_EXTRA_PX + effective_handle_radius(scene, style, h) * z
}

pub fn link_snap_commit_proximity_ok(scene: &BoardSceneState, style: &BoardStyleState, target_handle_id: &str, world: Point) -> bool {
    let Some(h) = scene.handles.get(target_handle_id) else {
        return false;
    };
    if !scene.handle_selectable(target_handle_id) {
        return false;
    }
    let Some(pw) = handle_world_pos(scene, style, h) else {
        return false;
    };
    let z = scene.camera.zoom.max(1e-9);
    let d_screen = distance_between(scene.world_to_screen(world), scene.world_to_screen(pw));
    let tol_commit = HANDLE_HIT_TOLERANCE_PX + LINK_COMMIT_SNAP_TIGHT_PX + effective_handle_radius(scene, style, h) * z;
    d_screen <= tol_commit
}

fn lod_allows_node_proximity_connect(scene: &BoardSceneState, style: &BoardStyleState) -> bool {
    matches!(current_draw_lod(scene, style), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro)
}

/// 🧲️ While dragging a node with no incident edges, overlapping bounds pick the nearest compatible free handle pair.
pub fn node_drag_proximity_handle_pair(scene: &BoardSceneState, style: &BoardStyleState, moving_node_id: &str) -> Option<(String, String)> {
    if !lod_allows_node_proximity_connect(scene, style) {
        return None;
    }
    if !scene.node_selectable(moving_node_id) {
        return None;
    }
    if scene.node_has_any_incident_edge(moving_node_id) {
        return None;
    }
    let moving = scene.nodes.get(moving_node_id)?;
    let moving_bounds = style.node_world_bounds(moving, 0.0);
    let mut best: Option<(f64, String, String)> = None;
    for (target_id, target) in &scene.nodes {
        if target_id == moving_node_id || !scene.node_selectable(target_id.as_str()) {
            continue;
        }
        let target_bounds = style.node_world_bounds(target, 0.0);
        if !world_boxes_overlap(moving_bounds, target_bounds) {
            continue;
        }
        let moving_handles: Vec<_> = scene.handles.iter().filter(|(id, h)| h.node_id == moving_node_id && scene.handle_selectable(id.as_str()) && !scene.handle_has_incident_edge(id.as_str())).collect();
        let target_handles: Vec<_> = scene.handles.iter().filter(|(id, h)| h.node_id == target_id.as_str() && scene.handle_selectable(id.as_str()) && !scene.handle_has_incident_edge(id.as_str())).collect();
        for (src_id, src_h) in &moving_handles {
            let Some(src_pos) = handle_world_pos(scene, style, src_h) else {
                continue;
            };
            for (tgt_id, tgt_h) in &target_handles {
                let Some(tgt_pos) = handle_world_pos(scene, style, tgt_h) else {
                    continue;
                };
                let d = distance_between(src_pos, tgt_pos);
                let pair = if handles_link_compatible_for_drag(scene, style, src_h, tgt_h) {
                    Some(((*src_id).clone(), (*tgt_id).clone()))
                } else if handles_link_compatible_for_drag(scene, style, tgt_h, src_h) {
                    Some(((*tgt_id).clone(), (*src_id).clone()))
                } else {
                    None
                };
                if let Some((s, t)) = pair {
                    if best.as_ref().is_none_or(|(bd, _, _)| d < *bd) {
                        best = Some((d, s.to_string(), t.to_string()));
                    }
                }
            }
        }
    }
    best.map(|(_, s, t)| (s, t))
}

pub fn edge_selectable(scene: &BoardSceneState, style: &BoardStyleState, edge: &EdgeData) -> bool {
    if !edge.visible || edge.locked {
        return false;
    }
    if !style.has_ports() {
        return scene.node_selectable(edge.source.as_str()) && scene.node_selectable(edge.target.as_str());
    }
    scene.handle_selectable(edge.source.as_str()) && scene.handle_selectable(edge.target.as_str())
}

pub fn edge_effectively_visible(scene: &BoardSceneState, style: &BoardStyleState, edge: &EdgeData) -> bool {
    if !style.has_ports() {
        return edge.visible && scene.node_effectively_visible(edge.source.as_str()) && scene.node_effectively_visible(edge.target.as_str());
    }
    edge.visible && scene.handle_effectively_visible(edge.source.as_str()) && scene.handle_effectively_visible(edge.target.as_str())
}

/// 📍️ Drag-phase link snap tests **screen px** to the handle anchor so detail/micro zoom keeps a stable hit halo; pointer-up re-checks with `link_snap_commit_proximity_ok` before `proximityConnect`.
pub fn nearest_link_snap_handle_world(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, world: Point) -> Option<String> {
    if matches!(current_draw_lod(scene, style), BoardDrawLod::Minimap) {
        return None;
    }
    let source_handle = scene.handles.get(source_handle_id)?;
    if !scene.handle_selectable(source_handle_id) {
        return None;
    }
    let source_node_id = source_handle.node_id.as_str();
    let p_scr = scene.world_to_screen(world);
    let mut best: Option<(f64, String)> = None;
    for (id, h) in &scene.handles {
        if id == source_handle_id || !scene.handle_selectable(id.as_str()) {
            continue;
        }
        if scene.handle_has_incident_edge(id.as_str()) {
            continue;
        }
        if h.node_id == source_node_id {
            continue;
        }
        if !handles_link_compatible_for_drag(scene, style, source_handle, h) {
            continue;
        }
        let pw = handle_world_pos(scene, style, h)?;
        let h_scr = scene.world_to_screen(pw);
        let d_screen = distance_between(p_scr, h_scr);
        let tol_screen = link_snap_drag_tolerance_screen(scene, style, h);
        if d_screen <= tol_screen && best.as_ref().is_none_or(|(bd, _)| d_screen < *bd) {
            best = Some((d_screen, id.clone()));
        }
    }
    best.map(|(_, id)| id)
}

pub fn planned_link_edge_id(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, target_handle_id: &str) -> Option<String> {
    if source_handle_id == target_handle_id || !scene.handle_selectable(source_handle_id) || !scene.handle_selectable(target_handle_id) {
        return None;
    }
    let source = scene.handles.get(source_handle_id)?;
    let target = scene.handles.get(target_handle_id)?;
    if source.node_id == target.node_id || !handles_link_compatible_for_drag(scene, style, source, target) || scene.handle_has_incident_edge(source_handle_id) || scene.handle_has_incident_edge(target_handle_id) {
        return None;
    }
    if scene.edges.values().any(|edge| edge.source == source_handle_id && edge.target == target_handle_id) {
        return None;
    }
    let mut serial = scene.edges.len().saturating_add(1);
    loop {
        let candidate = format!("edge-link-{serial}");
        if !scene.edges.contains_key(&candidate) {
            return Some(candidate);
        }
        serial = serial.saturating_add(1);
    }
}
