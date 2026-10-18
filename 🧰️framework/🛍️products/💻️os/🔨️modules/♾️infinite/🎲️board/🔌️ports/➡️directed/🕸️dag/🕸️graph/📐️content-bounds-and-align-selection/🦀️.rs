//! 📐️ Content bounds and align selection of the DAG graph: content world bounds, the camera state, minimap bounds, snapshot node sync and the selection alignment. See [`super`].

use super::*;

impl DagGraphState {
    // #region 🔖️DagHost
    // #region 🔖️ContentFraming
    /// 🖼️ World bounds of every node this board holds — what a fit frames and what the minimap maps.
    /// `None` for an empty graph, which has nothing to frame.
    pub fn content_world_bounds(&self) -> Option<canvas::camera::ContentBounds> {
        let first = self.host_snapshot.nodes.first()?;
        let mut union = dag_node_world_bounds(first);
        for node in self.host_snapshot.nodes.iter().skip(1) {
            let b = dag_node_world_bounds(node);
            union.min_x = union.min_x.min(b.min_x);
            union.min_y = union.min_y.min(b.min_y);
            union.max_x = union.max_x.max(b.max_x);
            union.max_y = union.max_y.max(b.max_y);
        }
        Some(canvas::camera::ContentBounds { min_x: union.min_x, min_y: union.min_y, max_x: union.max_x, max_y: union.max_y })
    }

    pub(super) fn camera_state(&self) -> canvas::camera::Camera {
        canvas::camera::Camera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom }
    }

    pub fn minimap_widget_content_bounds(&self) -> Option<WorldBox> {
        let pad = ui_styling::metrics::dag::MINIMAP_WIDGET_CONTENT_PAD;
        let content = self.content_world_bounds()?;
        Some(WorldBox { min_x: content.min_x - pad, min_y: content.min_y - pad, max_x: content.max_x + pad, max_y: content.max_y + pad })
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::content_fully_visible` — pure layout math relocated there
    /// (see `.🧬semio/🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`).
    pub fn minimap_camera_fully_shows_content(&self, content: &WorldBox, viewport_w: u32, viewport_h: u32) -> bool {
        let cam = &self.host_snapshot.camera;
        ui_wgpu::wgpu::minimap::content_fully_visible(&ui_wgpu::wgpu::minimap::MinimapContentBounds { min_x: content.min_x, min_y: content.min_y, max_x: content.max_x, max_y: content.max_y }, viewport_w, viewport_h, cam.x, cam.y, cam.zoom, 12.0)
    }

    pub fn selected_snapshot_nodes(&self) -> Vec<(usize, DagNodeSpec)> {
        let ids = self.selected_node_ids();
        ids.into_iter().filter_map(|id| self.host_snapshot.nodes.iter().enumerate().find(|(_, node)| node.id == id).map(|(idx, node)| (idx, node.clone()))).collect()
    }

    pub(crate) fn sync_snapshot_node_center_to_engine(&mut self, idx: usize) {
        let node = &self.host_snapshot.nodes[idx];
        let Some(nid) = self.node_id_for_widget_id(&node.id) else {
            return;
        };
        if let Some(engine_node) = self.engine.nodes.get_mut(&nid) {
            engine_node.center = canvas::Point::new(node.x, node.y);
        }
    }

    /// 📐️ Aligns or distributes the current multi-node selection.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), DagError> {
        use canvas::Point;
        let mut selected = self.selected_snapshot_nodes();
        if selected.is_empty() {
            return Ok(());
        }
        match mode {
            "alignLeft" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_left = selected.iter().map(|(_, node)| dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.x = min_left + node.width * 0.5;
                }
            }
            "alignRight" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_right = selected.iter().map(|(_, node)| dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.x = max_right - node.width * 0.5;
                }
            }
            "alignTop" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_top = selected.iter().map(|(_, node)| dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.y = min_top + node.height * 0.5;
                }
            }
            "alignBottom" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_bottom = selected.iter().map(|(_, node)| dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.y = max_bottom - node.height * 0.5;
                }
            }
            "alignHorizontal" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_x = (union.min_x + union.max_x) * 0.5;
                for (_, node) in &mut selected {
                    node.x = center_x;
                }
            }
            "alignVertical" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_y = (union.min_y + union.max_y) * 0.5;
                for (_, node) in &mut selected {
                    node.y = center_y;
                }
            }
            "distributeHorizontal" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap_or(std::cmp::Ordering::Equal));
                let left = selected.iter().map(|(_, node)| dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                let right = selected.iter().map(|(_, node)| dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                let total_width: f64 = selected.iter().map(|(_, node)| node.width).sum();
                let raw_gap = (right - left - total_width) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
                let mut cursor = left;
                for (_, node) in &mut selected {
                    node.x = cursor + node.width * 0.5;
                    cursor += node.width + gap;
                }
            }
            "distributeVertical" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap_or(std::cmp::Ordering::Equal));
                let top = selected.iter().map(|(_, node)| dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                let bottom = selected.iter().map(|(_, node)| dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                let total_height: f64 = selected.iter().map(|(_, node)| node.height).sum();
                let raw_gap = (bottom - top - total_height) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
                let mut cursor = top;
                for (_, node) in &mut selected {
                    node.y = cursor + node.height * 0.5;
                    cursor += node.height + gap;
                }
            }
            other => return Err(DagError::UnknownAlignMode(other.to_string())),
        }
        for (idx, node) in selected {
            self.host_snapshot.nodes[idx].x = node.x;
            self.host_snapshot.nodes[idx].y = node.y;
            self.sync_snapshot_node_center_to_engine(idx);
        }
        Ok(())
    }
    // #endregion 🔖️SelectionAlign
    // #endregion 🔖️DagHost
}
