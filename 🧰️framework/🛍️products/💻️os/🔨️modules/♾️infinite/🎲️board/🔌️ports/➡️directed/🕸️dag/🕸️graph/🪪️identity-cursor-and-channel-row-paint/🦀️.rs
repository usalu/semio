//! 🪪️ Identity cursor and channel row paint of the DAG graph: the identity cursor and the edge path, channel row highlight and divider and sided widget column paint of the graph state. See [`super`].

use super::*;

#[derive(Default)]
pub struct IdentityCursor{pub candidate:usize,pub byte:usize}

impl IdentityCursor{
 pub fn step(&mut self,graph: &DagGraphState,target:Identity<'_>,domain:usize,node:Option<u64>)->IdentityStep{
  let count=match domain{0=>graph.host_snapshot.nodes.len(),1=>graph.host_snapshot.edges.len(),2=>graph.handle_key_map.len(),_=>node.and_then(|id|graph.host_snapshot.nodes.get((id-1)as usize)).map_or(0,|node|node.inputs().len())};
  if self.candidate==count{return IdentityStep::Missing}
  let id=match domain{0=>self.candidate as u64+1,1=>graph.edge_engine_ids.get(self.candidate).and_then(|id|*id).unwrap_or(0),2=>self.candidate as u64+10,_=>self.candidate as u64};
  let candidate=match domain{0=>graph.host_snapshot.nodes.get(self.candidate).filter(|_|graph.node_id_map.get(&id)==Some(&self.candidate)).map(|node|node.id.as_str()),1=>graph.host_snapshot.edges.get(self.candidate).filter(|_|id!=0).map(|edge|edge.id.as_str()),2=>graph.handle_key_map.get(&id).filter(|_|graph.engine.handles.contains_key(&id)).map(String::as_str),_=>node.and_then(|id|graph.host_snapshot.nodes.get((id-1)as usize)).and_then(|node|node.inputs().get(self.candidate)).map(|port|port.id.as_str())};
  let Some(candidate)=candidate else{self.candidate+=1;self.byte=0;return IdentityStep::Progress};
  if candidate.len()!=target.len()||candidate.as_bytes().get(self.byte).copied()!=target.byte(self.byte){self.candidate+=1;self.byte=0;return IdentityStep::Progress}
  if self.byte==target.len(){return IdentityStep::Resolved(id)}self.byte+=1;IdentityStep::Progress
 }
}

semio_framework_value::artifact_retire_struct!(IdentityCursor{candidate,byte});

impl DagGraphState {
    pub fn edge_sharp_path(&self, eid: EdgeId) -> Option<canvas::BezPath> {
        let edge = self.engine.edges.get(&eid)?;
        let source_handle = self.engine.handles.get(&edge.source)?;
        let target_handle = self.engine.handles.get(&edge.target)?;
        let source_node = self.engine.nodes.get(&source_handle.node_id)?;
        let target_node = self.engine.nodes.get(&target_handle.node_id)?;
        let source_position = handle_position(source_node, source_handle);
        let target_position = handle_position(target_node, target_handle);
        let source_out = handle_outward_at_node_rim(source_position, source_node.center, source_node.shape, source_node.radius, source_node.width, source_node.height)?;
        let target_out = handle_outward_at_node_rim(target_position, target_node.center, target_node.shape, target_node.radius, target_node.width, target_node.height)?;
        let source_shape = self.handle_port_shape.get(&edge.source).copied().unwrap_or_default();
        let target_shape = self.handle_port_shape.get(&edge.target).copied().unwrap_or_default();
        let source_wire = handle_cap_peak(source_position, source_out, source_handle.radius, source_shape);
        let target_wire = handle_cap_peak(target_position, target_out, target_handle.radius, target_shape);
        Some(compute_edge_sharp_sz_path(source_wire, target_wire, source_out, target_out))
    }

    pub fn paint_computation_channel_row_highlights(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, node: &DagNodeSpec, theme: &CanvasPalette, is_dimmed: bool) {
        use canvas::FillRule;
        use canvas::Rect;
        let mut paint_bounds = |(x0, y0, x1, y1): (f64, f64, f64, f64), selected: bool, highlighted: bool, hovered: bool| {
            if !selected && !highlighted && !hovered {
                return;
            }
            let fill = dag_handle_body_fill(theme, is_dimmed, selected, highlighted, hovered);
            scene.fill(FillRule::NonZero, *aff, fill, None, &Rect::new(x0, y0, x1, y1));
        };
        for (port_idx, port) in node.inputs().iter().enumerate() {
            let Some(bounds) = input_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, true) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
        for (port_idx, port) in node.outputs().iter().enumerate() {
            let Some(bounds) = output_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, false) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
    }

    fn computation_channel_row_divider_stroke(&self, node: &DagNodeSpec, port_id: &str, body_stroke: canvas::Color, label_fill: canvas::Color, default_stroke: canvas::Color) -> canvas::Color {
        let Some(hid) = self.handle_id_for_port(&node.id, port_id) else {
            return default_stroke;
        };
        let (selected, _, hovered) = self.handle_interaction_chrome(hid);
        if selected || hovered {
            dag_node_internal_chrome_stroke(body_stroke, label_fill, true)
        } else {
            default_stroke
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    pub fn paint_computation_channel_row_dividers(&self, scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, chrome_stroke: f64, stroke: canvas::Color, body_stroke: canvas::Color, label_fill: canvas::Color, channel_row_pick: bool) {
        use canvas::{Line, Point, Stroke};
        let grid_rows = computation_channel_row_count(node);
        if grid_rows <= 1 {
            return;
        }
        let (input_rows, output_rows) = computation_io_side_row_counts(node);
        let stroke_style = Stroke::new(chrome_stroke);
        let row_stroke = |port_id: &str| {
            if channel_row_pick {
                self.computation_channel_row_divider_stroke(node, port_id, body_stroke, label_fill, stroke)
            } else {
                stroke
            }
        };
        if computation_input_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Input);
            let inputs = node.inputs();
            for row in computation_io_side_row_divider_indices(input_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = inputs.get(row.saturating_sub(1)).map_or("", |port| port.id.as_str());
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
        if computation_output_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Output);
            let outputs = node.outputs();
            for row in computation_io_side_row_divider_indices(output_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = outputs.get(row.saturating_sub(1)).map_or("", |port| port.id.as_str());
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
    }

    /// 📐️ Center divider and channel-row chrome for a one-sided or two-sided widget.
    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    pub fn paint_sided_widget_columns(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, lod: DagDrawLod, node: &DagNodeSpec, theme: &CanvasPalette, chrome: &DagNodePaintChrome, chrome_stroke: f64, internal_stroke: canvas::Color, body_stroke: canvas::Color, label_fill: canvas::Color) {
        if !lod.shows_computation_layout() {
            return;
        }
        let channel_row_pick = lod.uses_channel_row_pick();
        if channel_row_pick {
            self.paint_computation_channel_row_highlights(scene, aff, node, theme, chrome.is_dimmed);
        }
        paint_computation_column_divider(scene, *aff, node, chrome_stroke, internal_stroke);
        self.paint_computation_channel_row_dividers(scene, *aff, node, chrome_stroke, internal_stroke, body_stroke, label_fill, channel_row_pick);
    }
}
