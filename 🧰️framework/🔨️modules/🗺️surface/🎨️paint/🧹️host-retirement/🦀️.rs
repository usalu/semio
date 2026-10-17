//! 🧹️ Host retirement of surface paint: the raster host retirement custody with its drop guard. See [`super`].

use super::*;

// #region 🔖️Retirement
/** 🧹️ The bounded owner a released [`RasterHost`] becomes — the raster twin of
 * `MapHostRetirement`/`EditorHostRetirement`. A composite document's layer tree, decoded-image cache
 * and per-layer pixel scratch are each drained ONE entry per [`Self::close_step`], so a surface
 * holding hundreds of megabytes of paint buffers never frees them inside one frame turn. */
pub struct RasterHostRetirement {
    pub(super) layers: Vec<LayerNode>,
    pub(super) stack: Vec<RasterStackLayer>,
    pub(super) composite: compositing::RetainedComposite,
    pub(super) paint: HashMap<String, Arc<semio_framework_pixels::RasterImage>>,
    pub(super) active_utility: String,
    pub(super) hovered_id: Option<String>,
    pub(super) selected_ids: Vec<String>,
    pub(super) paint_gesture: Option<gesture::PaintGesture>,
    pub(super) paint_edit: Option<PaintEditCommand>,
    released: bool,
}

impl RasterHostRetirement {
    pub fn new(host: RasterHost) -> Self {
        let RasterHost {
            camera: _,
            viewport: _,
            document: RasterDocument { layers,stack },
            composite,
            buffers: RasterLayerBuffers { paint },
            active_utility,
            brush_size: _,
            brush_opacity: _,
            hovered_id,
            selected_ids,
            panning: _,
            brush_color: _,
            brush_hardness: _,
            paint_target: _,
            mask_value: _,
            paint_gesture,
            paint_edit,
            pan_last: _,
            show_selection_chrome: _,
            theme_clear: _,
            checkerboard_light_cell: _,
            checkerboard_dark_cell: _,
        } = host;
        Self { layers,stack,composite,paint,active_utility,hovered_id,selected_ids,paint_gesture,paint_edit,released:false }
    }

    fn close_layer_step(&mut self) -> bool {
        let Some(layer) = self.layers.pop() else { return true };
        if let LayerNode::Group { children, .. } = layer {
            self.layers.extend(children);
        }
        false
    }

    fn close_map_step<T>(map: &mut HashMap<String,T>) -> bool {
        let Some(key) = map.keys().next().cloned() else { return true };
        map.remove(&key);
        false
    }

    pub fn close_step(&mut self) -> bool {
        if self.released {
            return true;
        }
        if !self.composite.close_step(){return false;}
        if let Some(layer)=self.stack.pop(){if let RasterStackContent::Group(children)=layer.content{self.stack.extend(children);}return false;}
        if !self.close_layer_step() || !Self::close_map_step(&mut self.paint) || self.selected_ids.pop().is_some() {
            return false;
        }
        if self.paint_gesture.as_mut().is_some_and(|gesture| !gesture.close_step()) || self.paint_edit.as_mut().is_some_and(|command| !command.close_step()) { return false; }
        self.paint_gesture = None;
        self.paint_edit = None;
        if self.active_utility.pop().is_some() || self.hovered_id.as_mut().is_some_and(|id| id.pop().is_some()) { return false; }
        self.hovered_id = None;
        self.released = true;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.released && self.layers.is_empty() && self.stack.is_empty() && self.paint.is_empty() && self.selected_ids.is_empty() && self.active_utility.is_empty() && self.hovered_id.is_none() && self.paint_gesture.is_none() && self.paint_edit.is_none()
    }
}

impl Drop for RasterHostRetirement {
    fn drop(&mut self) {
        debug_assert!(self.terminal_is_empty(), "RasterHostRetirement must reach terminal-empty before release");
    }
}
// #endregion 🔖️Retirement
