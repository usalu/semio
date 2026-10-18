//! 🎥️ Camera, style kinds and overlay of the board focus: external link preview, the camera setters, kind hover ids, style kind resolution, overlay entity ids and edge stroke paint. See [`super`].

use super::*;

/// 🔗️ Clears host-driven link preview without touching local link drags.
pub fn clear_external_link_preview(focus: &mut BoardFocusState, outbox: &mut BoardOutboxState) {
    if matches!(focus.interaction, Interaction::ExternalLinkPreview { .. }) {
        focus.interaction = Interaction::None;
        outbox.clear_link_gesture_events();
    }
}

pub fn board_set_camera(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, x: f64, y: f64, zoom: f64) {
    set_camera_internal(scene, outbox, x, y, zoom, true);
}

/// 🔇️ Updates viewport camera without enqueueing a `camera` drain row (wheel / imperative sync).
pub fn set_camera_silent(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, x: f64, y: f64, zoom: f64) {
    set_camera_internal(scene, outbox, x, y, zoom, false);
}

fn set_camera_internal(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, x: f64, y: f64, zoom: f64, emit_event: bool) {
    let zoom = semio_framework_canvas::camera::clamp_zoom(zoom);
    if (scene.camera.x - x).abs() < 1e-9 && (scene.camera.y - y).abs() < 1e-9 && (scene.camera.zoom - zoom).abs() < 1e-9 {
        return;
    }
    let reservation = if emit_event {
        let Some(reservation) = outbox.reserve_owned_event(BoardOwnedEvent::camera(x, y, zoom)) else {
            return;
        };
        Some(reservation)
    } else {
        None
    };
    scene.camera.x = x;
    scene.camera.y = y;
    scene.camera.zoom = zoom;
    scene.interaction_revision = scene.interaction_revision.wrapping_add(1);
    if let Some(reservation) = reservation {
        outbox.publish_event_reservation(reservation);
    }
}

fn ids_matching_kind_hover(scene: &BoardSceneState, focus: &BoardFocusState) -> Vec<String> {
    let Some((domain, kind_id)) = focus.hovered_kind.as_ref() else {
        return Vec::new();
    };
    let mut ids = Vec::new();
    match domain.as_str() {
        "node" => {
            for node in scene.nodes.values() {
                if &node.node_kind == kind_id && !focus.selection.contains(&node.id) {
                    ids.push(node.id.clone());
                }
            }
        }
        "handle" => {
            for handle in scene.handles.values() {
                if &handle.handle_kind == kind_id && !focus.selection.contains(&handle.id) {
                    ids.push(handle.id.clone());
                }
            }
        }
        "edge" => {
            for edge in scene.edges.values() {
                if &edge.edge_kind == kind_id && !focus.selection.contains(&edge.id) {
                    ids.push(edge.id.clone());
                }
            }
        }
        "wire" => {
            for wire in scene.wires.values() {
                if &wire.wire_kind == kind_id && !focus.selection.contains(&wire.id) {
                    ids.push(wire.id.clone());
                }
            }
        }
        _ => {}
    }
    ids
}

pub fn resolve_node_style_kind(focus: &BoardFocusState, style_state: &BoardStyleState, n: &NodeData, pass: StyleChromePass) -> BoardElementStyleKind {
    if let Some(kind) = explicit_style_kind(n.style.as_deref()) {
        return locked_style_dim(kind, n.locked);
    }
    let kind = match pass {
        StyleChromePass::CachedBase => {
            if style_state.preserve_original_element_style {
                BoardElementStyleKind::Original
            } else {
                BoardElementStyleKind::Neutral
            }
        }
        StyleChromePass::InteractionOverlay => {
            if let Some(kind) = focus.hovered_style_kind(n.id.as_str(), "node", n.node_kind.as_str()) {
                return locked_style_dim(kind, n.locked);
            }
            if let Some(kind) = focus.highlighted_style_kind(n.id.as_str()) {
                return locked_style_dim(kind, n.locked);
            }
            focus.resolve_interaction_style_kind(n.id.as_str())
        }
    };
    locked_style_dim(kind, n.locked)
}

pub(crate) fn resolve_handle_style_kind(focus: &BoardFocusState, style_state: &BoardStyleState, h: &HandleData, pass: StyleChromePass) -> BoardElementStyleKind {
    if let Some(kind) = explicit_style_kind(h.style.as_deref()) {
        return locked_style_dim(kind, h.locked);
    }
    let kind = match pass {
        StyleChromePass::CachedBase => {
            if style_state.preserve_original_element_style {
                BoardElementStyleKind::Original
            } else {
                BoardElementStyleKind::Neutral
            }
        }
        StyleChromePass::InteractionOverlay => {
            if let Some(kind) = focus.hovered_style_kind(h.id.as_str(), "handle", h.handle_kind.as_str()) {
                return locked_style_dim(kind, h.locked);
            }
            if let Some(kind) = focus.highlighted_style_kind(h.id.as_str()) {
                return locked_style_dim(kind, h.locked);
            }
            focus.resolve_interaction_style_kind(h.id.as_str())
        }
    };
    locked_style_dim(kind, h.locked)
}

/// 💠️ Entity ids whose selection/preselect/hover chrome tints fills and strokes without rebuilding {@link BoardHost.world_content_cache}.
pub fn interaction_overlay_entity_ids(scene: &BoardSceneState, focus: &BoardFocusState) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    if focus.is_preselect_active() {
        ids.extend(focus.preselect.iter().cloned());
        ids.extend(focus.selection.iter().cloned());
        ids.extend(focus.preselect_removed.iter().cloned());
    } else {
        ids.extend(focus.selection.iter().cloned());
        ids.extend(focus.selection_exit_highlight.iter().cloned());
    }
    if let Some(ref hover_id) = focus.hovered_id {
        if !focus.is_preselect_active() && !focus.selection.contains(hover_id) {
            ids.insert(hover_id.clone());
        }
    }
    if focus.hovered_id.is_none() && !focus.is_preselect_active() {
        for id in ids_matching_kind_hover(scene, focus) {
            ids.insert(id);
        }
    }
    for id in focus.highlighted_ids.iter() {
        if !focus.selection.contains(id) && focus.hovered_id.as_deref() != Some(id.as_str()) {
            ids.insert(id.clone());
        }
    }
    ids
}

pub(crate) fn resolve_edge_stroke_paint(focus: &BoardFocusState, style: &BoardStyleState, e: &EdgeData, chrome_pass: StyleChromePass, lod: BoardDrawLod, lod_scale_width: f64) -> (Color, Stroke, f64) {
    let style_kind = focus.resolve_edge_style_kind(e, chrome_pass);
    let chrome = edge_stroke_for_style(&style.canvas_theme, style_kind);
    let kind_def = style.edge_kinds.get(e.edge_kind.as_str());
    let base_color = kind_def.and_then(|d| d.color).unwrap_or(style.canvas_theme.edge_stroke);
    let stroke_color = match style_kind {
        BoardElementStyleKind::Neutral | BoardElementStyleKind::Original => base_color,
        _ if lod == BoardDrawLod::Minimap => chrome,
        _ => lerp_color(base_color, chrome, 0.55),
    };
    let catalog_w = kind_def.map_or(2.0, |d| d.stroke_width);
    let width_mult = match style_kind {
        BoardElementStyleKind::Selected => ui_styling::strokes::EDGE_SELECTED_MULT,
        BoardElementStyleKind::Hovered => ui_styling::strokes::EDGE_HOVERED_MULT,
        _ => 1.0,
    };
    let width = lod_scale_width * (catalog_w / 2.0) * width_mult;
    let pattern = kind_def.map_or(EdgeStrokePattern::Solid, |d| d.pattern);
    (stroke_color, edge_stroke_for_kind_pattern(pattern, width), width)
}
