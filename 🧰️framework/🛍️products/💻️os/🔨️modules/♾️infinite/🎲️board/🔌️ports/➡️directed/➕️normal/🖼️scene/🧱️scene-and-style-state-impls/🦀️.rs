//! 🧱️ Scene and style state impls of the board scene: the scene state and style state methods. See [`super`].

use super::*;

impl BoardSceneState {
    pub fn drop_preview_from_json(&self, node: &serde_json::Value) -> Option<DropPreviewSnapshot> {
        let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
        let (x, y) = match (node.get("screenX").and_then(|v| v.as_f64()).filter(|v| v.is_finite()), node.get("screenY").and_then(|v| v.as_f64()).filter(|v| v.is_finite())) {
            (Some(sx), Some(sy)) => {
                let world = self.screen_to_world(Point::new(sx, sy));
                (world.x, world.y)
            }
            _ => {
                let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                (x, y)
            }
        };
        let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
            Some("rectangle") => NodeShape::Rectangle,
            _ => NodeShape::Circle,
        };
        let (radius, width, height) = match shape {
            NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
            NodeShape::Rectangle => {
                let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                (0.0, w, h)
            }
        };
        let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
        Some(DropPreviewSnapshot { node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, icon_kind })
    }

    pub fn set_pointer_selection_flag(&mut self, id: &str, selected: bool) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.selected = selected;
        }
        if let Some(handle) = self.handles.get_mut(id) {
            handle.selected = selected;
        }
        if let Some(edge) = self.edges.get_mut(id) {
            edge.selected = selected;
        }
        if let Some(wire) = self.wires.get_mut(id) {
            wire.selected = selected;
        }
    }

    pub fn world_to_screen(&self, p: Point) -> Point {
        semio_framework_canvas::camera::world_to_screen(&self.camera, &self.viewport(), p)
    }

    pub fn screen_to_world(&self, p: Point) -> Point {
        semio_framework_canvas::camera::screen_to_world(&self.camera, &self.viewport(), p)
    }

    pub fn eligible_indirect_handle_count_on_node(&self, node_id: &str) -> usize {
        self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_effectively_visible(id.as_str()) && self.handle_eligible_indirect_connect_ring(id.as_str())).count()
    }

    /// 🧭️ Returns the handle id when `node_id` has exactly one visible free indirect-eligible handle.
    pub fn sole_eligible_indirect_handle_on_node(&self, node_id: &str) -> Option<String> {
        let mut found: Option<String> = None;
        for (id, h) in &self.handles {
            if h.node_id != node_id || !self.handle_effectively_visible(id.as_str()) || !self.handle_eligible_indirect_connect_ring(id.as_str()) {
                continue;
            }
            if found.is_some() {
                return None;
            }
            found = Some(id.clone());
        }
        found
    }

    pub fn node_center_world(&self, node_id: &str) -> Option<Point> {
        let n = self.nodes.get(node_id)?;
        Some(Point::new(n.x, n.y))
    }
}

impl BoardStyleState {
    /// 🧩️ Selects world-space clip tiling for Vello scene construction (`none` | `world-clip`).
    pub fn set_world_raster_tiling(&mut self, mode: &str) {
        let next = if mode == "world-clip" { "world-clip".into() } else { "none".into() };
        if self.world_raster_tiling == next {
            return;
        }
        self.world_raster_tiling = next;
    }

    pub fn set_original_element_style(&mut self, enabled: bool) {
        if self.preserve_original_element_style == enabled {
            return;
        }
        self.preserve_original_element_style = enabled;
        self.icon_paint_cache.clear();
    }

    pub fn set_canvas_palette(&mut self, overlay: &BoardPaletteOverlay) {
        self.canvas_theme.apply_overlay(overlay);
        self.icon_paint_cache.clear();
    }

    fn node_kind_scale(&self, node_kind: &str) -> f64 {
        self.node_kinds.get(node_kind).map_or(1.0, |k| k.scale)
    }

    pub(super) fn handle_kind_scale(&self, handle_kind: &str) -> f64 {
        self.handle_kinds.get(handle_kind).map_or(1.0, |k| k.scale)
    }

    pub(super) fn effective_node_scale(&self, n: &NodeData) -> f64 {
        (n.scale * self.node_kind_scale(n.node_kind.as_str())).max(1e-9)
    }

    pub fn scaled_node_radius(&self, n: &NodeData) -> f64 {
        n.radius * self.effective_node_scale(n)
    }

    pub fn scaled_node_width(&self, n: &NodeData) -> f64 {
        n.width * self.effective_node_scale(n)
    }

    pub fn scaled_node_height(&self, n: &NodeData) -> f64 {
        n.height * self.effective_node_scale(n)
    }

    /// 📐️ Node half-extent for indirect ring layout: circle radius or half the shorter rectangle side.
    pub(super) fn indirect_node_half_extent(&self, n: &NodeData) -> f64 {
        match n.shape {
            NodeShape::Circle => self.scaled_node_radius(n),
            NodeShape::Rectangle => self.scaled_node_width(n).min(self.scaled_node_height(n)) * 0.5,
        }
    }

    /// 📐️ Radial world offset from node rim to indirect-handle center (`INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× half-extent) so ring–node proportions stay fixed when zooming.
    pub(super) fn indirect_handle_ring_offset_world(&self, n: &NodeData) -> f64 {
        (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_RING_GAP_NODE_SCALE).max(1e-9)
    }

    pub fn point_in_node_world(&self, n: &NodeData, point: Point) -> bool {
        match n.shape {
            NodeShape::Rectangle => {
                let hw = self.scaled_node_width(n) / 2.0;
                let hh = self.scaled_node_height(n) / 2.0;
                (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
            }
            NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
        }
    }
}
