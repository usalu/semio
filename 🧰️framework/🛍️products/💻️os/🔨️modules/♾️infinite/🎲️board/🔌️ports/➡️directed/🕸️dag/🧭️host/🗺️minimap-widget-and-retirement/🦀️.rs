//! 🗺️ Minimap widget and retirement of the DAG host: the minimap state methods, widget layout, hit, cursor, JSON and paint, the state retirements and the host retirement. See [`super`].

use super::*;

impl DagMinimapState {
    // #region 🔖️MinimapWidget
    /// 🗺️ Toggles the bottom-right flow minimap navigator.
    pub fn set_minimap_widget_visible(&mut self, visible: bool) {
        self.minimap_widget_visible = visible;
    }
}

/// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::layout` — see `minimap_camera_fully_shows_content` above.
pub fn minimap_widget_layout(graph: &DagGraphState, minimap_state: &DagMinimapState, viewport_w: u32, viewport_h: u32) -> Option<MinimapWidgetLayout> {
    if !minimap_state.minimap_widget_visible {
        return None;
    }
    let content = graph.minimap_widget_content_bounds()?;
    if graph.minimap_camera_fully_shows_content(&content, viewport_w, viewport_h) {
        return None;
    }
    let w = ui_styling::metrics::dag::MINIMAP_WIDGET_WIDTH;
    let h = ui_styling::metrics::dag::MINIMAP_WIDGET_HEIGHT;
    let margin = ui_styling::metrics::dag::MINIMAP_WIDGET_MARGIN;
    let ratio = ui_styling::metrics::dag::MINIMAP_WIDGET_MAX_CONTENT_RATIO;
    let cam = &graph.host_snapshot.camera;
    let layout =
        ui_wgpu::wgpu::minimap::layout(&ui_wgpu::wgpu::minimap::MinimapContentBounds { min_x: content.min_x, min_y: content.min_y, max_x: content.max_x, max_y: content.max_y }, viewport_w, viewport_h, cam.x, cam.y, cam.zoom, w, h, margin, ratio);
    Some(MinimapWidgetLayout { panel: layout.panel, world_min_x: layout.world_min_x, world_min_y: layout.world_min_y, scale: layout.scale, map_origin_x: layout.map_origin_x, map_origin_y: layout.map_origin_y, viewport: layout.viewport })
}

pub(crate) fn minimap_widget_pointer_hit(viewport_state: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, sx: f64, sy: f64) -> Option<(MinimapWidgetLayout, bool)> {
    let layout = minimap_widget_layout(graph, minimap, viewport_state.width, viewport_state.height)?;
    if !minimap_widget_point_in_rect(layout.panel, sx, sy) {
        return None;
    }
    let on_viewport = minimap_widget_point_in_rect(layout.viewport, sx, sy);
    Some((layout, on_viewport))
}

fn minimap_widget_cursor_hint(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState) -> Option<&'static str> {
    if minimap.minimap_widget_drag.is_some() {
        return Some("grabbing");
    }
    if let Some((layout, on_viewport)) = minimap_widget_pointer_hit(viewport, graph, minimap, viewport.last_screen_x, viewport.last_screen_y) {
        let _ = layout;
        return Some(if on_viewport { "grab" } else { "pointer" });
    }
    None
}

pub(crate) fn minimap_widget_json(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState) -> Option<Value> {
    let layout = minimap_widget_layout(graph, minimap, viewport.width, viewport.height)?;
    let (x0, y0, x1, y1) = layout.panel;
    Some(semio_framework_pack_json::object([
        ("x".to_string(), Value::from(x0)),
        ("y".to_string(), Value::from(y0)),
        ("width".to_string(), Value::from(x1 - x0)),
        ("height".to_string(), Value::from(y1 - y0)),
        ("cursor".to_string(), Value::from(minimap_widget_cursor_hint(viewport, graph, minimap))),
    ]))
}

pub(crate) fn paint_minimap_widget(viewport_state: &DagViewportState, graph: &DagGraphState, paint: &DagPaintState, minimap: &DagMinimapState, scene: &mut canvas::Scene, viewport_w: u32, viewport_h: u32) {
    let Some(layout) = minimap_widget_layout(graph, minimap, viewport_w, viewport_h) else {
        return;
    };
    use canvas::{Affine, FillRule, Rect, Stroke};
    use ui_styling::strokes;
    let theme = &viewport_state.canvas_theme;
    let aff = Affine::IDENTITY;
    let (px0, py0, px1, py1) = layout.panel;
    let panel = Rect::new(px0, py0, px1, py1);
    scene.fill(FillRule::NonZero, aff, theme.minimap_widget_panel_fill, None, &panel);
    scene.stroke(&Stroke::new(strokes::DAG_MINIMAP_WIDGET_PANEL), aff, theme.minimap_widget_panel_stroke, None, &panel);
    let node_min = ui_styling::metrics::dag::MINIMAP_WIDGET_NODE_MIN_SIZE;
    let lod = DagDrawLod::Minimap;
    for (idx, snapshot_node) in graph.host_snapshot.nodes.iter().enumerate() {
        let node = graph.node_spec_for_paint(idx, snapshot_node);
        let node = node.as_ref();
        let engine_nid = graph.engine_node_id_for_index(idx);
        let is_dimmed = engine_nid.is_some_and(|nid| paint.dimmed.contains(&nid));
        let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| graph.node_interaction_chrome(nid));
        let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) else {
            continue;
        };
        let hw = (node.width * layout.scale * 0.5).max(node_min * 0.5);
        let hh = (node.height * layout.scale * 0.5).max(node_min * 0.5);
        let cx = layout.map_origin_x + (node.x - layout.world_min_x) * layout.scale;
        let cy = layout.map_origin_y + (node.y - layout.world_min_y) * layout.scale;
        let rect = Rect::new(cx - hw, cy - hh, cx + hw, cy + hh);
        scene.fill(FillRule::NonZero, aff, fill, None, &rect);
    }
    let (vx0, vy0, vx1, vy1) = layout.viewport;
    let view_rect = Rect::new(vx0, vy0, vx1, vy1);
    let active = minimap.minimap_widget_hovered || minimap.minimap_widget_drag.is_some();
    scene.fill(FillRule::NonZero, aff, theme.minimap_widget_viewport_fill, None, &view_rect);
    let stroke_c = if active { theme.minimap_widget_viewport_stroke_hovered } else { theme.minimap_widget_viewport_stroke };
    scene.stroke(&Stroke::new(strokes::DAG_MINIMAP_WIDGET_VIEWPORT), aff, stroke_c, None, &view_rect);
}

// #endregion 🔖️MinimapWidget

impl RetireOwned for DagPointerState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{widget_drag,pending_port_insert,ghost_node,pending_cluster_explode,pending_export_click,pending_open_instance_id,last_pointer_down_at_ms,last_pointer_down_world,last_pointer_down_node_id}=self;sequence(vec![deferred(widget_drag),deferred(pending_port_insert),deferred(ghost_node),deferred(pending_cluster_explode),deferred(pending_export_click),deferred(pending_open_instance_id),deferred(last_pointer_down_at_ms),deferred(last_pointer_down_world),deferred(last_pointer_down_node_id)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.widget_drag),deferred_birth_bytes_for(&self.pending_port_insert),deferred_birth_bytes_for(&self.ghost_node),deferred_birth_bytes_for(&self.pending_cluster_explode),deferred_birth_bytes_for(&self.pending_export_click),deferred_birth_bytes_for(&self.pending_open_instance_id),deferred_birth_bytes_for(&self.last_pointer_down_at_ms),deferred_birth_bytes_for(&self.last_pointer_down_world),deferred_birth_bytes_for(&self.last_pointer_down_node_id)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagPaintState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{dimmed,icon_paint_cache,computing_active,computing_stale,computing_active_anim_phase,computing_stale_anim_phase,node_eval_status,unresolved_input_ports}=self;sequence(vec![deferred(dimmed),deferred(icon_paint_cache),deferred(computing_active),deferred(computing_stale),deferred(computing_active_anim_phase.into_inner()),deferred(computing_stale_anim_phase.into_inner()),deferred(node_eval_status),deferred(unresolved_input_ports)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.dimmed),deferred_birth_bytes_for(&self.icon_paint_cache),deferred_birth_bytes_for(&self.computing_active),deferred_birth_bytes_for(&self.computing_stale),deferred_birth_bytes_for(&self.computing_active_anim_phase.get()),deferred_birth_bytes_for(&self.computing_stale_anim_phase.get()),deferred_birth_bytes_for(&self.node_eval_status),deferred_birth_bytes_for(&self.unresolved_input_ports)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagMinimapState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{minimap_widget_visible,minimap_widget_hovered,minimap_widget_drag}=self;sequence(vec![deferred(minimap_widget_visible),deferred(minimap_widget_hovered),deferred(minimap_widget_drag)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.minimap_widget_visible),deferred_birth_bytes_for(&self.minimap_widget_hovered),deferred_birth_bytes_for(&self.minimap_widget_drag)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagNotesState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{editing_note,caret_visible}=self;sequence(vec![deferred(editing_note),deferred(caret_visible)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.editing_note),deferred_birth_bytes_for(&self.caret_visible)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagHost {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{viewport,graph,pointer,paint,minimap,notes}=self;sequence(vec![deferred(viewport),deferred(graph),deferred(pointer),deferred(paint),deferred(minimap),deferred(notes)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.viewport),deferred_birth_bytes_for(&self.graph),deferred_birth_bytes_for(&self.pointer),deferred_birth_bytes_for(&self.paint),deferred_birth_bytes_for(&self.minimap),deferred_birth_bytes_for(&self.notes)])}
 fn controlled_retirement_supported()->bool{true}
}

/// 🌳️ Retains the whole original DAG, including engine, scene, pending interaction and journal sources.
#[must_use="the original DAG must reach physical terminal emptiness"]
pub struct DagHostRetirement {pub owner:ControlledRetirement<DagHost>}

impl DagHostRetirement {
 pub fn new(source:DagHost)->Self{Self{owner:ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!("original DAG fields declare typed retirement"))}}
 pub fn original(&self)->Option<&DagHost>{self.owner.original()}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(copy)}
 pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.owner.step(grant)}
}
