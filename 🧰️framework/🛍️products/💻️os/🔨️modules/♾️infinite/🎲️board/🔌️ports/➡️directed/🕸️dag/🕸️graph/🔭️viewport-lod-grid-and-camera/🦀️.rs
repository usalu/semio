//! 🔭️ Viewport LOD, grid and camera of the DAG graph: the viewport settings, automatic and forced level of detail, grid steps and paint, channel hover and selection and the selection camera focus. See [`super`].

use super::*;

impl DagViewportState {
    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
    }

    pub fn automatic_lod(&self) -> bool {
        self.automatic_lod
    }

    pub fn forced_draw_lod_label(&self) -> Option<&'static str> {
        self.forced_draw_lod.map(|lod| lod.label())
    }

    /// 📶️ When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier.
    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        if enabled {
            self.forced_draw_lod = None;
        }
    }

    /// 🔲️ Toggles LOD-tiered world grid painting.
    pub fn set_grid_visible(&mut self, visible: bool) {
        self.grid_visible = visible;
    }

    /// 🧲️ Toggles node-drag snap to the finest visible LOD grid step.
    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.grid_snap_enabled = enabled;
    }

    /// 📐️ Sets the positive multiplier for LOD world grid steps.
    pub fn set_grid_factor(&mut self, factor: f64) -> Result<(), DagError> {
        if !factor.is_finite() || factor <= 0.0 || factor > 1e6 {
            return Err(DagError::GridFactorOutOfRange);
        }
        self.grid_factor = factor;
        Ok(())
    }

    pub fn grid_step_large_world(&self) -> f64 {
        GRID_WORLD_LARGE * self.grid_factor
    }

    pub fn grid_step_medium_world(&self) -> f64 {
        GRID_WORLD_MEDIUM * self.grid_factor
    }

    pub fn grid_step_small_world(&self) -> f64 {
        GRID_WORLD_SMALL * self.grid_factor
    }

    pub fn grid_step_micro_world(&self) -> f64 {
        GRID_WORLD_MICRO * self.grid_factor
    }

    pub fn paint_lod_grid(&self, scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, lod: DagDrawLod) {
        if !self.grid_visible || self.wheel_zoom_active || lod == DagDrawLod::Minimap {
            return;
        }
        let grid_color = self.canvas_theme.grid_minor_stroke;
        dag_stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_LARGE, self.grid_step_large_world(), 0.0);
        match lod {
            DagDrawLod::Normal | DagDrawLod::Detail | DagDrawLod::Micro => {
                dag_stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MEDIUM, self.grid_step_medium_world(), 0.0);
            }
            DagDrawLod::Minimap | DagDrawLod::Overview | DagDrawLod::Compact => {}
        }
        if matches!(lod, DagDrawLod::Detail | DagDrawLod::Micro) {
            dag_stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_SMALL, self.grid_step_small_world(), 0.0);
        }
        if lod == DagDrawLod::Micro {
            dag_stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MICRO, self.grid_step_micro_world(), 0.0);
        }
    }

    /// 📶️ Pins WASM draw LOD when {@link DagHost::set_automatic_lod} is false; pass an empty label to follow zoom bands.
    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            self.forced_draw_lod = None;
            return;
        }
        self.forced_draw_lod = DagDrawLod::from_id(trimmed);
    }
}

/// 🔌️ Sets hover to a snapshot channel handle, falling back to node hover below channel LOD.
pub fn set_hover_channel(viewport: &mut DagViewportState, graph: &mut DagGraphState, widget_id: Option<&str>, port_id: Option<&str>) {
    let Some(widget_id) = widget_id else {
        graph.set_hover(None);
        return;
    };
    if let Some(port_id) = port_id {
        if draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
            if let Some(hid) = graph.handle_id_for_port(widget_id, port_id) {
                if graph.engine.hover != Some(hid) {
                    graph.engine.hover = Some(hid);
                }
                return;
            }
        }
    }
    graph.set_hover(Some(widget_id));
}

/// 🔌️ Replaces typed channel selection, using node selection below channel LOD.
pub fn set_selected_channels(viewport: &mut DagViewportState, graph: &mut DagGraphState, channels: &[DagChannelRef]) {
    if draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
        let mut selection = Selection::default();
        for channel in channels {
            if let Some(hid) = graph.handle_id_for_port(&channel.widget_id, &channel.port) {
                selection.handle_ids.insert(hid);
            }
        }
        graph.engine.selection = selection;
        graph.engine.preselect = Selection::default();
        graph.engine.preselect_removed = Selection::default();
        return;
    }
    let widget_ids: Vec<String> = channels.iter().map(|channel| channel.widget_id.clone()).collect();
    graph.set_selection(&widget_ids);
}

/// 🔍️ Pins draw LOD while the wheel gesture is active so chrome does not flicker across bands.
pub fn dag_set_wheel_zoom_active(viewport: &mut DagViewportState, graph: &mut DagGraphState, active: bool) {
    if active && !viewport.wheel_zoom_active {
        viewport.wheel_zoom_render_lod = Some(resolved_draw_lod(viewport, graph));
    } else if !active {
        viewport.wheel_zoom_render_lod = None;
    }
    viewport.wheel_zoom_active = active;
}

fn resolved_draw_lod(viewport: &DagViewportState, graph: &DagGraphState) -> DagDrawLod {
    if !viewport.automatic_lod {
        if let Some(lod) = viewport.forced_draw_lod {
            return lod;
        }
    }
    dag_draw_lod(graph.host_snapshot.camera.zoom)
}

pub fn draw_lod_for_frame(viewport: &DagViewportState, graph: &DagGraphState) -> DagDrawLod {
    if !viewport.automatic_lod {
        if let Some(lod) = viewport.forced_draw_lod {
            return lod;
        }
    }
    if viewport.wheel_zoom_active {
        if let Some(pinned) = viewport.wheel_zoom_render_lod {
            return pinned;
        }
    }
    dag_draw_lod(graph.host_snapshot.camera.zoom)
}

/// 📶️ Active draw LOD tier label (`minimap`, `overview`, …).
pub fn dag_draw_lod_label(viewport: &DagViewportState, graph: &DagGraphState) -> &'static str {
    draw_lod_for_frame(viewport, graph).label()
}

/// 🔍️ Frames the current node selection in the viewport camera.
pub fn focus_selection_camera(viewport: &DagViewportState, graph: &DagGraphState, pad: f64) -> Option<DagCamera> {
    let selected = graph.selected_snapshot_nodes();
    if selected.is_empty() {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for (_, node) in &selected {
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        min_x = min_x.min(node.x - hw);
        min_y = min_y.min(node.y - hh);
        max_x = max_x.max(node.x + hw);
        max_y = max_y.max(node.y + hh);
    }
    if !min_x.is_finite() {
        return None;
    }
    let pad = pad.max(1.05);
    let cx = (min_x + max_x) * 0.5;
    let cy = (min_y + max_y) * 0.5;
    let span_w = (max_x - min_x).max(1.0);
    let span_h = (max_y - min_y).max(1.0);
    let vw = viewport.width.max(1) as f64;
    let vh = viewport.height.max(1) as f64;
    let zoom = (vw / (span_w * pad)).min(vh / (span_h * pad)).clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
    Some(DagCamera { x: cx, y: cy, zoom })
}
