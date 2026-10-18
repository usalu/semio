//! 🧱️ Host and state impls of the DAG host: further host methods and the paint, pointer and notes state methods. See [`super`].

use super::*;

// #region 🔖️DagHost
impl DagHost {
    pub fn load_host_snapshot_json(json: &str) -> Result<Self, DagError> {
        let host_snapshot: DagHostSnapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        if host_snapshot.schema != "dag.hostDocument" {
            return Err(DagError::SchemaMismatch);
        }
        Ok(Self::from_host_snapshot(host_snapshot))
    }

    /// 🖱️ Whether a screen-space interaction only `pointer_*_screen` implements is already in flight
    /// — drawing/reconnecting a wire, dragging the minimap viewport, panning, or dragging an inline
    /// widget. While one is, the bounded `derive_pointer_plan` path (whose gesture vocabulary is
    /// `Idle | Pan | Drag | Select`) must not claim the follow-up move/up and cancel it.
    pub fn screen_pointer_gesture_active(&self) -> bool {
        !matches!(self.graph.engine.interaction, InteractionMode::Idle) || self.minimap.minimap_widget_drag.is_some() || self.viewport.pan_anchor.is_some() || self.pointer.widget_drag.is_some() || self.pointer.pending_port_insert.is_some()
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        self.pointer_down_screen(x, y, 0, extend, false, false, false);
    }

    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo (see infinite/board/rs, framework/editor/rs, layout/rs, etc.)")]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        if button == 0 && !shift && !ctrl_or_meta && !alt && !pan {
            if let Some((layout, on_viewport)) = minimap_widget_pointer_hit(&self.viewport, &self.graph, &self.minimap, sx, sy) {
                let (wx, wy) = minimap_widget_screen_to_world(&layout, sx, sy);
                let zoom = self.graph.host_snapshot.camera.zoom;
                if on_viewport {
                    let cam = &self.graph.host_snapshot.camera;
                    self.minimap.minimap_widget_drag = Some((cam.x - wx, cam.y - wy));
                } else {
                    self.graph.set_camera(wx, wy, zoom);
                    self.minimap.minimap_widget_drag = Some((0.0, 0.0));
                }
                return;
            }
        }
        if pan {
            self.viewport.pan_anchor = Some((sx, sy, self.graph.host_snapshot.camera.x, self.graph.host_snapshot.camera.y));
            return;
        }
        sync_connection_hit_picking_for_lod(&mut self.viewport, &mut self.graph);
        self.viewport.last_screen_x = sx;
        self.viewport.last_screen_y = sy;
        let world = screen_to_world_point(&self.viewport, &self.graph, sx, sy);
        if button == 0 && !shift && !ctrl_or_meta && !alt {
            if let Some(node_id) = self.graph.snapshot_draggable_node_hit(world.x, world.y) {
                if !world_hits_handle(&self.viewport, &self.graph, world.x, world.y) {
                    if let Some(widget_id) = self.graph.widget_id_for_node_id(node_id) {
                        if let Some(node) = self.graph.host_snapshot.nodes.iter().find(|entry| entry.id == widget_id) {
                            if let DagNodeKind::AppInstance { instance_id, .. } = &node.kind {
                                let now = pointer_event_now_ms();
                                let dist = ((world.x - self.pointer.last_pointer_down_world.0).powi(2) + (world.y - self.pointer.last_pointer_down_world.1).powi(2)).sqrt();
                                let zoom = self.graph.host_snapshot.camera.zoom.max(1e-9);
                                if self.pointer.last_pointer_down_node_id.as_deref() == Some(widget_id.as_str()) && now - self.pointer.last_pointer_down_at_ms < 350.0 && dist < 8.0 / zoom {
                                    self.pointer.pending_open_instance_id = Some(instance_id.clone());
                                    return;
                                }
                                self.pointer.last_pointer_down_at_ms = now;
                                self.pointer.last_pointer_down_world = (world.x, world.y);
                                self.pointer.last_pointer_down_node_id = Some(widget_id);
                            }
                        }
                    }
                }
            }
        }
        if let Some(hit) = self.graph.port_insert_hit(world.x, world.y, self.graph.host_snapshot.camera.zoom) {
            self.pointer.pending_port_insert = Some(hit);
            return;
        }
        let (hit_x, hit_y) = connection_hit_world(&self.viewport, &self.graph, world.x, world.y);
        if world_hits_handle(&self.viewport, &self.graph, hit_x, hit_y) {
            self.graph.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
            process_engine_events(&mut self.viewport, &mut self.graph);
            self.graph.sync_camera_from_engine();
            return;
        }
        if try_widget_pointer_down(&mut self.viewport, &mut self.graph, &mut self.pointer, world.x, world.y) {
            return;
        }
        if self.graph.try_node_rectangle_pointer_down(world.x, world.y, button, shift, ctrl_or_meta, alt) {
            process_engine_events(&mut self.viewport, &mut self.graph);
            self.graph.sync_camera_from_engine();
            return;
        }
        let merge_from_modifiers = ctrl_or_meta || shift;
        if button == 0 && !merge_from_modifiers && lod_uses_bounded_drag(&self.viewport, &self.graph) {
            let pad = DAG_BOUNDED_DRAG_HIT_PAD_PX / self.graph.host_snapshot.camera.zoom.max(1e-9);
            if self.graph.engine.try_begin_selection_union_drag_at(world, pad) {
                process_engine_events(&mut self.viewport, &mut self.graph);
                self.graph.sync_camera_from_engine();
                return;
            }
        }
        self.graph.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
        process_engine_events(&mut self.viewport, &mut self.graph);
        self.graph.sync_camera_from_engine();
    }
}

impl DagPaintState {
    /// ✅️ Clears evaluating chrome from all nodes.
    pub fn clear_computing(&mut self) {
        self.computing_active = None;
        self.computing_stale.clear();
        self.node_eval_status.reset();
        self.unresolved_input_ports.clear();
    }

    pub(crate) fn tick_computing_animation(&self) {
        if self.computing_active.is_some() {
            let next = (self.computing_active_anim_phase.get() + 0.02) % 1.0;
            self.computing_active_anim_phase.set(next);
        }
        if !self.computing_stale.is_empty() {
            let next = (self.computing_stale_anim_phase.get() + 0.008) % 1.0;
            self.computing_stale_anim_phase.set(next);
        }
    }
}

impl DagPointerState {
    /// ➕️ Returns and clears a pending variadic insert request from the last pointer down.
    pub fn take_pending_port_insert(&mut self) -> Option<(DagPortSide, String, usize)> {
        self.pending_port_insert.take()
    }

    /// 👻️ Sets or clears the placement preview node painted with the normal LOD path.
    pub fn set_ghost_node(&mut self, node: Option<DagNodeSpec>) {
        self.ghost_node = node;
    }

    pub fn ghost_node(&self) -> Option<&DagNodeSpec> {
        self.ghost_node.as_ref()
    }

    /// 💥️ Takes a pending cluster explode request from the last widget hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.pending_cluster_explode.take()
    }

    /// 📤️ Takes a pending export control click from the last widget hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.pending_export_click.take()
    }

    pub fn widget_drag_active(&self) -> bool {
        self.widget_drag.is_some()
    }
}

impl DagNotesState {
    /// ✏️ Ends inline note editing.
    pub fn note_commit_edit(&mut self) {
        self.editing_note = None;
    }

    /// ✏️ Toggles native caret visibility for the active note editor.
    pub fn set_note_caret_visible(&mut self, visible: bool) {
        self.caret_visible = visible;
    }

    /// ✏️ Returns the widget id currently being edited inline, if any.
    pub fn editing_note_id(&self) -> Option<&str> {
        self.editing_note.as_ref().map(|edit| edit.node_id.as_str())
    }
}
