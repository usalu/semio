//! 🎨️ Paint, hover and link commit of the board focus: the indirect handle ring, node, handle and edge paint, icon spans, hover ids, wheel handling, link edge commit and snapshot JSON loading. See [`super`].

use super::*;

pub fn append_indirect_handle_ring(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, tile_filter: Option<&WorldBox>, node_id: &str, chrome_pass: StyleChromePass, world_space: bool) {
    for h in scene_state.handles.values() {
        if h.node_id != node_id || !scene_state.handle_effectively_visible(h.id.as_str()) {
            continue;
        }
        if !indirect_ring_handle_eligible(scene_state, focus, style, h.id.as_str(), node_id) {
            continue;
        }
        if let Some(tb) = tile_filter {
            let Some(hb) = indirect_handle_world_bounds_cull(scene_state, style, h) else { continue };
            if !world_boxes_overlap(*tb, hb) {
                continue;
            }
        }
        let Some(wp) = indirect_handle_world_pos(scene_state, style, h) else { continue };
        let style_kind = resolve_handle_style_kind(focus, style, h, chrome_pass);
        let stroke_px = 2.0_f64;
        let paint_override = if matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { Some((style.canvas_theme.indirect_handle_fill, style.canvas_theme.indirect_handle_stroke, stroke_px)) } else { None };
        append_handle_marker(scene_state, style, scene, h, wp, indirect_handle_marker_radius_world(scene_state, style, h), false, style_kind, paint_override, world_space, NodeHandlePaintLayer::Full, false);
    }
}

fn paint_node_geometry(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, n: &NodeData, lod: BoardDrawLod, world_space: bool, layer: NodeHandlePaintLayer, chrome_pass: StyleChromePass, link_compat: bool) {
    let draw_node_icons = matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro);
    let resolved_style_kind = resolve_node_style_kind(focus, style, n, chrome_pass);
    let style_kind = if link_compat && matches!(resolved_style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { BoardElementStyleKind::Highlighted } else { resolved_style_kind };
    let draw_node_stroke = lod != BoardDrawLod::Minimap || !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral);
    let stroke_c = node_stroke_for_style(&style.canvas_theme, style_kind);
    let fill = if lod == BoardDrawLod::Minimap { stroke_c } else { style.resolve_node_fill_color(n, &style.canvas_theme, style_kind) };
    let sw = 2.0_f64;
    let paint_fill = if lod == BoardDrawLod::Minimap {
        matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill)
    } else {
        matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill) && !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral)
    };
    let paint_stroke = draw_node_stroke && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
    let paint_icons = draw_node_icons && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
    match n.shape {
        NodeShape::Circle => {
            let c = scene_state.draw_space_point(Point::new(n.x, n.y), world_space);
            let r = scene_state.draw_space_len(style.scaled_node_radius(n), world_space);
            let circle = Circle::new(c, r);
            if paint_fill {
                scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
            }
            if paint_stroke {
                scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
            }
            if paint_icons {
                paint_node_icon(scene_state, style, scene, n, world_space, style_kind, Some(&circle), None);
            }
        }
        NodeShape::Rectangle => {
            let hw = style.scaled_node_width(n) / 2.0;
            let hh = style.scaled_node_height(n) / 2.0;
            let p0 = scene_state.draw_space_point(Point::new(n.x - hw, n.y - hh), world_space);
            let p1 = scene_state.draw_space_point(Point::new(n.x + hw, n.y + hh), world_space);
            let rect = Rect::from_points(p0, p1);
            if paint_fill {
                scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &rect);
            }
            if paint_stroke {
                scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &rect);
            }
            if paint_icons {
                paint_node_icon(scene_state, style, scene, n, world_space, style_kind, None, Some(rect));
            }
        }
    }
}

pub fn append_nodes_and_handles_with_overlay_chrome(
    scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState,
    scene: &mut Scene,
    tile_filter: Option<&WorldBox>,
    lod: BoardDrawLod,
    world_space: bool,
    only_ids: Option<&BTreeSet<String>>,
    overlay_ids: &BTreeSet<String>,
    layer: NodeHandlePaintLayer,
) {
    let pad = scene_state.drawable_cull_pad_world();
    let draw_handles = style.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
    let draw_handle_icons = lod == BoardDrawLod::Micro;
    let link_source = focus.active_link_source_handle_id().map(str::to_string);
    let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|s| link_drag_compatible_target_node_ids(scene_state, style, s).into_iter().collect()).unwrap_or_default();
    for h in scene_state.handles.values() {
        if !draw_handles || !scene_state.handle_effectively_visible(h.id.as_str()) {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&h.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            let Some(hb) = handle_world_bounds_cull(scene_state, style, h) else { continue };
            if !world_boxes_overlap(*tb, hb) {
                continue;
            }
        }
        let Some(wp) = handle_world_pos(scene_state, style, h) else { continue };
        let style_kind = resolve_handle_style_kind(focus, style, h, chrome_pass_for_entity(&h.id, overlay_ids));
        append_handle_marker(scene_state, style, scene, h, wp, effective_handle_radius(scene_state, style, h), draw_handle_icons, style_kind, None, world_space, layer, true);
    }
    let paint_node = |scene: &mut Scene, n: &NodeData, chrome_pass: StyleChromePass| {
        paint_node_geometry(scene_state, focus, style, scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
    };
    for n in scene_state.nodes.values() {
        if !n.visible {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&n.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            if !world_boxes_overlap(*tb, style.node_world_bounds(n, pad)) {
                continue;
            }
        }
        if overlay_ids.contains(&n.id) {
            continue;
        }
        paint_node(scene, n, StyleChromePass::CachedBase);
    }
    for n in scene_state.nodes.values() {
        if !n.visible {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&n.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            if !world_boxes_overlap(*tb, style.node_world_bounds(n, pad)) {
                continue;
            }
        }
        if !overlay_ids.contains(&n.id) {
            continue;
        }
        paint_node(scene, n, StyleChromePass::InteractionOverlay);
    }
}

fn append_nodes_and_handles(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, chrome_pass: StyleChromePass, layer: NodeHandlePaintLayer) {
    let pad = scene_state.drawable_cull_pad_world();
    let draw_handles = style.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
    let draw_handle_icons = lod == BoardDrawLod::Micro;
    let link_source = focus.active_link_source_handle_id().map(str::to_string);
    let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|s| link_drag_compatible_target_node_ids(scene_state, style, s).into_iter().collect()).unwrap_or_default();
    for h in scene_state.handles.values() {
        if !draw_handles || !scene_state.handle_effectively_visible(h.id.as_str()) {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&h.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            let Some(hb) = handle_world_bounds_cull(scene_state, style, h) else { continue };
            if !world_boxes_overlap(*tb, hb) {
                continue;
            }
        }
        let Some(wp) = handle_world_pos(scene_state, style, h) else { continue };
        let style_kind = resolve_handle_style_kind(focus, style, h, chrome_pass);
        append_handle_marker(scene_state, style, scene, h, wp, effective_handle_radius(scene_state, style, h), draw_handle_icons, style_kind, None, world_space, layer, true);
    }
    for n in scene_state.nodes.values() {
        if !n.visible {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&n.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            if !world_boxes_overlap(*tb, style.node_world_bounds(n, pad)) {
                continue;
            }
        }
        paint_node_geometry(scene_state, focus, style, scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
    }
}

pub fn append_edges_wires_and_link(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, overlay_ids: Option<&BTreeSet<String>>) {
    let edge_sw = if world_space { scene_state.edge_world_stroke_width(lod) } else { scene_state.edge_screen_stroke_width_px(lod) };
    for e in scene_state.edges.values() {
        if !edge_effectively_visible(scene_state, style, e) {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&e.id) {
                continue;
            }
        }
        if let Some(tb) = tile_filter {
            let Some(eb) = edge_world_bounds_for_cull(scene_state, style, e) else { continue };
            if !world_boxes_overlap(*tb, eb) {
                continue;
            }
        }
        if let Some(c) = board_edge_curve(scene_state, style, e) {
            let p0 = scene_state.draw_space_point(c.p0(), world_space);
            let p1 = scene_state.draw_space_point(c.p1(), world_space);
            let p2 = scene_state.draw_space_point(c.p2(), world_space);
            let p3 = scene_state.draw_space_point(c.p3(), world_space);
            let curve = CubicBez::new(p0, p1, p2, p3);
            let chrome_pass = overlay_ids.map_or(StyleChromePass::CachedBase, |ids| chrome_pass_for_entity(&e.id, ids));
            let (stroke_color, edge_stroke, stroke_w) = resolve_edge_stroke_paint(focus, style, e, chrome_pass, lod, edge_sw);
            scene.stroke(&edge_stroke, Affine::IDENTITY, stroke_color, None, &curve);
            let (source_tip, target_tip) = style.resolve_edge_tips(e);
            append_edge_tips_on_curve(scene, &curve, stroke_color, stroke_w, source_tip, target_tip);
        }
    }
    let wire_sw = 2.25_f64;
    let wire_stroke = Stroke::new(wire_sw);
    for w in scene_state.wires.values() {
        if !scene_state.wire_effectively_visible(w) {
            continue;
        }
        if let Some(ids) = only_ids {
            if !ids.contains(&w.id) {
                continue;
            }
        }
        if let Some(c) = wire_curve(scene_state, style, w) {
            let p0 = scene_state.draw_space_point(c.p0(), world_space);
            let p1 = scene_state.draw_space_point(c.p1(), world_space);
            let p2 = scene_state.draw_space_point(c.p2(), world_space);
            let p3 = scene_state.draw_space_point(c.p3(), world_space);
            let curve = CubicBez::new(p0, p1, p2, p3);
            let chrome_pass = overlay_ids.map_or(StyleChromePass::CachedBase, |ids| chrome_pass_for_entity(&w.id, ids));
            let wc = wire_stroke_for_style(&style.canvas_theme, focus.resolve_wire_style_kind(w, chrome_pass));
            scene.stroke(&wire_stroke, Affine::IDENTITY, wc, None, &curve);
        }
    }
    let link_wire_sw = 2.85_f64;
    let link_wire_stroke = Stroke::new(link_wire_sw);
    let link_wire_color = style.canvas_theme.node_stroke;
    if let Some(c) = active_link_wire_curve(scene_state, focus, style) {
        let p0 = scene_state.draw_space_point(c.p0(), world_space);
        let p1 = scene_state.draw_space_point(c.p1(), world_space);
        let p2 = scene_state.draw_space_point(c.p2(), world_space);
        let p3 = scene_state.draw_space_point(c.p3(), world_space);
        let curve = CubicBez::new(p0, p1, p2, p3);
        scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
    }
}

/// 🎨️ Bakes each visible node's icon (and its handle icons) as a contiguous span at the node's current center.
pub fn bake_node_icon_spans(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, scene: &mut Scene, lod: BoardDrawLod) -> Vec<NodeIconSpan> {
    let draw_handles = style.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
    let draw_handle_icons = lod == BoardDrawLod::Micro;
    let link_source = focus.active_link_source_handle_id().map(str::to_string);
    let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|source| link_drag_compatible_target_node_ids(scene_state, style, source).into_iter().collect()).unwrap_or_default();
    let mut spans = Vec::new();
    for node in scene_state.nodes.values() {
        if !node.visible {
            continue;
        }
        let start = scene.command_len();
        if draw_handles {
            for handle in scene_state.handles.values() {
                if handle.node_id != node.id || !scene_state.handle_effectively_visible(handle.id.as_str()) {
                    continue;
                }
                let Some(world) = handle_world_pos(scene_state, style, handle) else { continue };
                let style_kind = resolve_handle_style_kind(focus, style, handle, StyleChromePass::CachedBase);
                append_handle_marker(scene_state, style, scene, handle, world, effective_handle_radius(scene_state, style, handle), draw_handle_icons, style_kind, None, true, NodeHandlePaintLayer::Icons, true);
            }
        }
        paint_node_geometry(scene_state, focus, style, scene, node, lod, true, NodeHandlePaintLayer::Icons, StyleChromePass::CachedBase, link_compat_nodes.contains(&node.id));
        let end = scene.command_len();
        if end > start {
            spans.push(NodeIconSpan { id: node.id.clone(), start, end, x: node.x, y: node.y });
        }
    }
    spans
}

pub fn set_hovered_id(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, id: Option<String>) {
    let event_kind = id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    if focus.hovered_id == id && focus.hovered_kind.is_none() {
        return;
    }
    let Some(reservation) = outbox.reserve_owned_event(BoardOwnedEvent::hover(id.as_deref(), event_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str())))) else {
        return;
    };
    focus.hovered_id = id;
    focus.hovered_kind = None;
    outbox.publish_event_reservation(reservation);
}

/// 🖱️ Sets transitive kind hover from a catalog row (clears direct `hovered_id`).
pub fn set_hovered_kind(focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, domain: Option<String>, kind_id: Option<String>) {
    let next_kind = domain.zip(kind_id);
    if focus.hovered_id.is_none() && focus.hovered_kind == next_kind {
        return;
    }
    let Some(reservation) = outbox.reserve_owned_event(BoardOwnedEvent::hover(None, next_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str())))) else {
        return;
    };
    focus.hovered_id = None;
    focus.hovered_kind = next_kind;
    outbox.publish_event_reservation(reservation);
}

pub fn wheel_screen(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, sx: f64, sy: f64, delta_y: f64) {
    let plan = scene.plan_wheel(sx, sy, delta_y);
    let _ = commit_wheel(scene, outbox, &plan);
}

pub fn commit_wheel(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, plan: &BoardWheelPlan) -> bool {
    if scene.interaction_revision != plan.revision || [scene.camera.x.to_bits(), scene.camera.y.to_bits(), scene.camera.zoom.to_bits()] != [plan.expected.x.to_bits(), plan.expected.y.to_bits(), plan.expected.zoom.to_bits()] {
        return false;
    }
    set_camera_silent(scene, outbox, plan.next.x, plan.next.y, plan.next.zoom);
    scene.interaction_revision = plan.revision.wrapping_add(1);
    true
}

pub fn try_commit_link_edge(scene: &mut BoardSceneState, style_state: &mut BoardStyleState, outbox: &mut BoardOutboxState, source_handle_id: &str, target_handle_id: &str, also_emit: Option<BoardEventKind>) -> bool {
    if source_handle_id == target_handle_id {
        return false;
    }
    if !scene.handle_selectable(source_handle_id) || !scene.handle_selectable(target_handle_id) {
        return false;
    }
    let Some(source_row) = scene.handles.get(source_handle_id) else {
        return false;
    };
    let Some(target_row) = scene.handles.get(target_handle_id) else {
        return false;
    };
    if source_row.node_id == target_row.node_id {
        return false;
    }
    if !handles_link_compatible_for_drag(scene, style_state, source_row, target_row) {
        return false;
    }
    if scene.handle_has_incident_edge(source_handle_id) || scene.handle_has_incident_edge(target_handle_id) {
        return false;
    }
    for e in scene.edges.values() {
        if e.source == source_handle_id && e.target == target_handle_id {
            return false;
        }
    }
    let mut n = scene.edges.len().saturating_add(1);
    let id = loop {
        let candidate = format!("edge-link-{n}");
        if !scene.edges.contains_key(&candidate) {
            break candidate;
        }
        n = n.saturating_add(1);
    };
    let edge_kind = style_state.default_edge_kind_for_created_link(source_row, target_row);
    let Some(reservation) = outbox.reserve_owned_batch(BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, &id, source_handle_id, target_handle_id), also_emit.map(|kind| BoardOwnedEvent::edge(kind, &id, source_handle_id, target_handle_id)))
    else {
        return false;
    };
    scene.edges.insert(
        id.clone(),
        EdgeData {
            id,
            source: source_handle_id.to_string(),
            target: target_handle_id.to_string(),
            selected: false,
            visible: true,
            locked: false,
            style: None,
            edge_kind,
            source_tip: None,
            target_tip: None,
            properties: semio_framework_graph_manifest::PropertyBag::new(),
        },
    );
    outbox.publish_event_batch(reservation);
    true
}

pub fn load_board_snapshot_json(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState,source:&str,control:&mut NativeDecodeControl<'_>)->bool{match decode_board_snapshot_json(source,control){Ok(snapshot)=>load_board_snapshot(scene, focus, style, outbox, snapshot),Err(_)=>false}}
