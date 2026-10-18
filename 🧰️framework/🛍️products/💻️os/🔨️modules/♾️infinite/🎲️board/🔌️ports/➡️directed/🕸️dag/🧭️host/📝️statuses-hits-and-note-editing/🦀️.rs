//! 📝️ Statuses, hits and note editing of the DAG host: dimming, computing progress, node statuses, connection cursor, screen hits, note editing and the widget pointer press. See [`super`].

use super::*;

/// 🌫️ Marks preview-off nodes as dimmed on the canvas.
pub fn set_dimmed(graph: &mut DagGraphState, paint: &mut DagPaintState, widget_ids: &[String]) {
    paint.dimmed.clear();
    for widget_id in widget_ids {
        if let Some(nid) = graph.node_id_for_widget_id(widget_id) {
            paint.dimmed.insert(nid);
        }
    }
}

/// ⚙️ Marks one actively computing widget and downstream widgets as stale.
pub fn set_computing_progress(graph: &mut DagGraphState, paint: &mut DagPaintState, active_widget_id: Option<&str>, stale_widget_ids: &[String]) {
    paint.computing_active = active_widget_id.and_then(|widget_id| graph.node_id_for_widget_id(widget_id));
    paint.computing_stale.clear();
    for widget_id in stale_widget_ids {
        if let Some(nid) = graph.node_id_for_widget_id(widget_id) {
            if paint.computing_active != Some(nid) {
                paint.computing_stale.insert(nid);
            }
        }
    }
}

/// 🚦️ Replaces chrome from complete admitted per-widget evaluation facts.
pub fn set_node_statuses(graph: &mut DagGraphState, paint: &mut DagPaintState,statuses:&DagNodeStatuses) {
    paint.computing_active=None;
    paint.computing_stale.clear();
    paint.node_eval_status.reset();
    paint.unresolved_input_ports.clear();
    for (widget_id,status)in statuses {
        let Some(nid)=graph.node_id_for_widget_id(widget_id)else{continue};
        let kind=match status {
            DagNodeEvaluationStatus::Computing{}=>{paint.computing_active=Some(nid);DagNodeEvalStatusKind::Computing},
            DagNodeEvaluationStatus::Queued{}=>{paint.computing_stale.insert(nid);DagNodeEvalStatusKind::Queued},
            DagNodeEvaluationStatus::Error{..}=>DagNodeEvalStatusKind::Error,
            DagNodeEvaluationStatus::Blocked{ports}=>{for port in ports{if let Some(node)=graph.host_snapshot.nodes.get((nid-1)as usize){for(index,input)in node.inputs().iter().enumerate(){if input.id==*port{paint.unresolved_input_ports.insert((nid,index));}}}}DagNodeEvalStatusKind::Blocked},
            DagNodeEvaluationStatus::Ok{}=>DagNodeEvalStatusKind::Ok,
        };
        paint.node_eval_status.insert(nid,kind);
    }
}

/// 📋️ Preview-off snapshot node ids currently dimmed on the canvas.
pub fn dimmed_node_ids(graph: &DagGraphState, paint: &DagPaintState) -> Vec<String> {
    paint.dimmed.iter().filter_map(|&nid| graph.widget_id_for_node_id(nid)).collect()
}

/// 🔌️ Whether React Flow's centered connection cursor applies at this point: either the
/// live hit is a port handle or a wire draw/reconnect gesture owns the pointer beyond its source.
pub fn connection_cursor_active_at_screen(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, sx: f64, sy: f64) -> bool {
    matches!(graph.engine.interaction, InteractionMode::DrawEdge { .. }) || screen_hit(viewport, graph, minimap, sx, sy).handle
}

/// 🖱️ Whether this SCREEN point belongs to the screen pointer path at all: the minimap widget, a
/// wire handle, a port insertion target, or an inline widget. Exactly the four hits
/// `bounded_node_hit_index` answers `Unsupported` for, asked as a question instead of as a fault.
///
/// 🖐️ It answers for every PHASE, not only a press: a hover or a release over one of these hits
/// is just as undescribable to the bounded plan path, and routing only presses left a plain move
/// across a port faulting the whole dispatch (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn screen_pointer_gesture_begins_at(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, sx: f64, sy: f64) -> bool {
    screen_hit(viewport, graph, minimap, sx, sy).is_screen_path()
}

/// 🩺️ The ONE classification of a screen point both the path discriminator and a renderer
/// diagnostic read, so a gesture can never be explained by a different hit test than the one that
/// routed it. `node_id` is the draggable snapshot node under the point; the four booleans are the
/// hits `bounded_node_hit_index` answers `Unsupported` for.
pub fn screen_hit(viewport: &DagViewportState, graph: &DagGraphState, minimap_state: &DagMinimapState, sx: f64, sy: f64) -> DagScreenHit {
    let minimap = minimap_widget_pointer_hit(viewport, graph, minimap_state, sx, sy);
    let world = screen_to_world_point(viewport, graph, sx, sy);
    let handle = port_pointer_handle_hit(viewport, graph, world.x, world.y);
    DagScreenHit {
        node_id: graph.snapshot_draggable_node_hit(world.x, world.y).and_then(|node_id| graph.widget_id_for_node_id(node_id)),
        channel: handle.and_then(|hid| graph.decode_channel_ref(hid)),
        minimap: minimap.is_some(),
        minimap_viewport: minimap.is_some_and(|(_, on_viewport)| on_viewport),
        port_insert: graph.port_insert_hit(world.x, world.y, graph.host_snapshot.camera.zoom).is_some(),
        handle: handle.is_some(),
        widget: graph.widget_hit_at(world.x, world.y).is_some(),
    }
}

/// 🩺️ [`Self::screen_hit`] as JSON, for a renderer that has to report why a press went where it
/// went across a wasm boundary that carries no Rust types.
pub fn screen_hit_json(viewport: &DagViewportState, graph: &DagGraphState, minimap_state: &DagMinimapState, sx: f64, sy: f64) -> String {
    let hit = screen_hit(viewport, graph, minimap_state, sx, sy);
    semio_framework_pack_json::to_json_string(&DagScreenHitJson {
        node: hit.node_id.clone(),
        draggable: hit.is_draggable_body(),
        handle: hit.channel.as_ref().map(|channel| format!("{}@{}", channel.widget_id, channel.port)),
        direction: hit.channel.as_ref().map(|channel| channel.direction.clone()),
        widget: hit.widget,
        minimap: hit.minimap,
        minimap_viewport: hit.minimap_viewport,
        port_insert: hit.port_insert,
    })
}

/// ✏️ Begins inline note text editing at a world-space click position.
pub fn begin_note_edit(graph: &mut DagGraphState, notes: &mut DagNotesState, node_id: &str, world_x: f64, _world_y: f64) -> bool {
    let Some(node) = graph.host_snapshot.nodes.iter().find(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &node.kind else {
        return false;
    };
    let lod_index = dag_lod_index(graph.host_snapshot.camera.zoom);
    let font_px = dag_label_paint_px(graph.host_snapshot.camera.zoom, lod_index) * 1.05;
    let origin_x = note_text_origin_x(node);
    let offset = hit_byte_in_note_line(text, world_x, origin_x, font_px);
    notes.editing_note = Some(NoteEditState { node_id: node_id.to_string(), caret: offset, anchor: offset });
    notes.caret_visible = true;
    true
}

/// ✏️ Inserts text at the active note caret, replacing any selection.
pub fn note_insert_text(graph: &mut DagGraphState, notes: &mut DagNotesState, chunk: &str) -> bool {
    let Some(node_id) = notes.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
        return false;
    };
    let Some(idx) = graph.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &mut graph.host_snapshot.nodes[idx].kind else {
        return false;
    };
    let Some(edit) = notes.editing_note.as_mut() else {
        return false;
    };
    let start = edit.caret.min(edit.anchor);
    let end = edit.caret.max(edit.anchor);
    text.replace_range(start..end, chunk);
    let pos = start + chunk.len();
    edit.caret = pos;
    edit.anchor = pos;
    true
}

/// ✏️ Deletes the selection or the character before the note caret.
pub fn note_backspace(graph: &mut DagGraphState, notes: &mut DagNotesState) -> bool {
    let Some(node_id) = notes.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
        return false;
    };
    let Some(idx) = graph.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &mut graph.host_snapshot.nodes[idx].kind else {
        return false;
    };
    let Some(edit) = notes.editing_note.as_mut() else {
        return false;
    };
    let start = edit.caret.min(edit.anchor);
    let end = edit.caret.max(edit.anchor);
    if start < end {
        text.replace_range(start..end, "");
        edit.caret = start;
        edit.anchor = start;
        return true;
    }
    if start == 0 {
        return false;
    }
    let prev = text[..start].char_indices().last().map_or(0, |(i, _)| i);
    text.replace_range(prev..start, "");
    edit.caret = prev;
    edit.anchor = prev;
    true
}

/// ✏️ Deletes the selection or the character after the note caret.
pub fn note_delete_forward(graph: &mut DagGraphState, notes: &mut DagNotesState) -> bool {
    let Some(node_id) = notes.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
        return false;
    };
    let Some(idx) = graph.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &mut graph.host_snapshot.nodes[idx].kind else {
        return false;
    };
    let Some(edit) = notes.editing_note.as_mut() else {
        return false;
    };
    let start = edit.caret.min(edit.anchor);
    let end = edit.caret.max(edit.anchor);
    if start < end {
        text.replace_range(start..end, "");
        edit.caret = start;
        edit.anchor = start;
        return true;
    }
    if start >= text.len() {
        return false;
    }
    let next = text[start..].char_indices().nth(1).map_or(text.len(), |(i, _)| start + i);
    text.replace_range(start..next, "");
    edit.caret = start;
    edit.anchor = start;
    true
}

/// ✏️ Moves the note caret (`left` | `right` | `home` | `end`).
pub fn note_move_caret(graph: &mut DagGraphState, notes: &mut DagNotesState, direction: &str, extend: bool) -> bool {
    let Some(node_id) = notes.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
        return false;
    };
    let Some(idx) = graph.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
        return false;
    };
    let DagNodeKind::Note { text, .. } = &graph.host_snapshot.nodes[idx].kind else {
        return false;
    };
    let Some(edit) = notes.editing_note.as_mut() else {
        return false;
    };
    let pos = match direction {
        "left" => {
            if edit.caret == 0 {
                0
            } else {
                text[..edit.caret].char_indices().last().map_or(0, |(i, _)| i)
            }
        }
        "right" => text[edit.caret..].char_indices().nth(1).map_or(text.len(), |(i, _)| edit.caret + i),
        "home" => 0,
        "end" => text.len(),
        _ => return false,
    };
    if extend {
        edit.caret = pos;
    } else {
        edit.caret = pos;
        edit.anchor = pos;
    }
    true
}

pub(super) fn try_widget_pointer_down(viewport: &mut DagViewportState, graph: &mut DagGraphState, pointer: &mut DagPointerState, world_x: f64, world_y: f64) -> bool {
    if !draw_lod_for_frame(viewport, graph).shows_controls() {
        return false;
    }
    let Some((idx, kind)) = graph.widget_hit_at(world_x, world_y) else {
        return false;
    };
    match kind {
        WidgetPointerKind::SliderDrag => {
            pointer.widget_drag = Some(idx);
            set_slider_value_from_x(&mut graph.host_snapshot.nodes[idx], world_x);
        }
        WidgetPointerKind::SelectClick => {
            advance_select_option(&mut graph.host_snapshot.nodes[idx]);
        }
        WidgetPointerKind::PreviewToggle(path) => {
            toggle_preview_tree_path(&mut graph.host_snapshot.nodes[idx], &path);
            graph.sync_snapshot_node_size_to_engine(idx);
        }
        WidgetPointerKind::ClusterExplode => {
            pointer.pending_cluster_explode = Some(graph.host_snapshot.nodes[idx].id.clone());
        }
        WidgetPointerKind::ExportClick => {
            pointer.pending_export_click = Some(graph.host_snapshot.nodes[idx].id.clone());
        }
    }
    true
}
