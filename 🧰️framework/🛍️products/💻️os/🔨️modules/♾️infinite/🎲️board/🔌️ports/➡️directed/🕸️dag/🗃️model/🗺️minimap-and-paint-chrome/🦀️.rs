//! 🗺️ Minimap and paint chrome of the DAG model: the evaluation status kind, the minimap layout, the pointer clock, colour alpha, node paint chrome, minimap conversions, node bounds and the graph edit retirement. See [`super`].

use super::*;

pub struct MinimapWidgetLayout {
    pub panel: (f64, f64, f64, f64),
    pub world_min_x: f64,
    pub world_min_y: f64,
    pub scale: f64,
    pub map_origin_x: f64,
    pub map_origin_y: f64,
    pub viewport: (f64, f64, f64, f64),
}

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; pointer-event timestamps are a
// browser canvas concept the wasip2 guest has no pointer events to timestamp in the first place,
// so it shares the existing native no-op (`0.0`) rather than reading a WASI clock for no consumer.
pub fn pointer_event_now_ms() -> f64 {
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        js_sys::Date::now()
    }
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    {
        0.0
    }
}

pub fn canvas_color_with_alpha(color: canvas::Color, alpha: u8) -> canvas::Color {
    use canvas::Color;
    let rgba = color.to_rgba8();
    Color::from_rgba8(rgba.r, rgba.g, rgba.b, alpha)
}

#[derive(Clone, Copy)]
pub struct DagNodePaintChrome {
    pub is_dimmed: bool,
    pub is_selected: bool,
    pub is_highlighted: bool,
    pub is_hovered: bool,
    pub eval_status: DagNodeEvalStatusKind,
    pub body_fill_alpha: u8,
    pub ghost_tint: bool,
}

impl DagNodePaintChrome {
    pub fn ghost_preview() -> Self {
        Self { is_dimmed: false, is_selected: false, is_highlighted: false, is_hovered: false, eval_status: DagNodeEvalStatusKind::Ok, body_fill_alpha: 255, ghost_tint: true }
    }

    pub fn tint_highlighted(self) -> bool {
        self.is_highlighted || self.ghost_tint
    }

    pub fn has_interaction_chrome(self) -> bool {
        self.is_dimmed || self.is_selected || self.is_highlighted || self.is_hovered
    }
}

/// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::screen_to_world` — see `minimap_camera_fully_shows_content` above.
pub fn minimap_widget_screen_to_world(layout: &MinimapWidgetLayout, sx: f64, sy: f64) -> (f64, f64) {
    ui_wgpu::wgpu::minimap::screen_to_world(layout.map_origin_x, layout.map_origin_y, layout.world_min_x, layout.world_min_y, layout.scale, sx, sy)
}

/// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::point_in_rect` — see `minimap_camera_fully_shows_content` above.
pub fn minimap_widget_point_in_rect(rect: (f64, f64, f64, f64), sx: f64, sy: f64) -> bool {
    ui_wgpu::wgpu::minimap::point_in_rect(rect, sx, sy)
}

// #region 🔖️SelectionAlign
pub fn dag_node_world_bounds(node: &DagNodeSpec) -> WorldBox {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    WorldBox { min_x: node.x - hw, min_y: node.y - hh, max_x: node.x + hw, max_y: node.y + hh }
}

semio_framework_value::artifact_retire_leaf!(DagPortSide);

semio_framework_value::artifact_retire_leaf!(DagJournalRefusal);

semio_framework_value::artifact_retire_leaf!(DagNodeEvalStatusKind);

semio_framework_value::artifact_retire_leaf!(DagInteractionProjection);

semio_framework_value::artifact_retire_struct!(NoteEditState {node_id,caret,anchor});

impl RetireOwned for DagGraphEdit {
 fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Connect{source_node_id,source_port_id,target_node_id,target_port_id}=>(source_node_id,source_port_id,target_node_id,target_port_id).retirement(),Self::Disconnect{synapse_id}=>synapse_id.retirement(),Self::Move{gesture_id,node_ids,dx,dy}=>(gesture_id,node_ids,dx,dy).retirement(),Self::SetSlider{node_id,value}=>(node_id,value).retirement(),Self::InsertPort{node_id,side,index}=>(node_id,side,index).retirement()}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Connect{source_node_id,source_port_id,target_node_id,target_port_id}=>sequence_birth_bytes(&[deferred_birth_bytes_for(source_node_id),deferred_birth_bytes_for(source_port_id),deferred_birth_bytes_for(target_node_id),deferred_birth_bytes_for(target_port_id)]),Self::Disconnect{synapse_id}=>synapse_id.retirement_birth_bytes(),Self::Move{gesture_id,node_ids,dx,dy}=>sequence_birth_bytes(&[deferred_birth_bytes_for(gesture_id),deferred_birth_bytes_for(node_ids),deferred_birth_bytes_for(dx),deferred_birth_bytes_for(dy)]),Self::SetSlider{node_id,value}=>sequence_birth_bytes(&[deferred_birth_bytes_for(node_id),deferred_birth_bytes_for(value)]),Self::InsertPort{node_id,side,index}=>sequence_birth_bytes(&[deferred_birth_bytes_for(node_id),deferred_birth_bytes_for(side),deferred_birth_bytes_for(index)])}}
 fn controlled_retirement_supported()->bool{true}
}
