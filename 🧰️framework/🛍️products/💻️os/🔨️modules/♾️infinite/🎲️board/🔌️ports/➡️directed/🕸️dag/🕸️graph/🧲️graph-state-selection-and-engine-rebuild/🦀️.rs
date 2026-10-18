//! 🧲️ Graph state selection and engine rebuild of the DAG graph: widget positions, selection, hover, port insert hits, layout rebuilds and the engine edge sync. See [`super`].

use super::*;

impl DagGraphState {
    /// 📍️ Sets a snapshot widget position in both the snapshot and engine snapshots.
    pub fn set_widget_position(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), DagError> {
        let idx = self.host_snapshot.nodes.iter().position(|node| node.id == widget_id).ok_or_else(|| DagError::UnknownWidget(widget_id.to_string()))?;
        self.host_snapshot.nodes[idx].x = x;
        self.host_snapshot.nodes[idx].y = y;
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return Ok(());
        };
        if let Some(node) = self.engine.nodes.get_mut(&nid) {
            node.center = canvas::Point::new(x, y);
        }
        Ok(())
    }

    /// 🗑️ Deletes the current selection from the snapshot.
    pub fn delete_selected(&mut self) {
        let widget_ids = self.selected_node_ids();
        self.engine.delete_selection();
        self.host_snapshot.nodes.retain(|node| !widget_ids.contains(&node.id));
        self.sync_edges_from_engine();
        self.rebuild_engine_with_layout(false);
    }

    /// ⌨️ Selects every snapshot node id.
    pub fn select_all_node_ids(&self) -> Vec<String> {
        self.host_snapshot.nodes.iter().map(|node| node.id.clone()).collect()
    }

    pub fn select_all(&mut self) {
        self.engine.select_all();
    }

    /// 🖱️ Sets hover to a snapshot widget id, or clears hover.
    pub fn set_hover(&mut self, widget_id: Option<&str>) {
        let next = widget_id.and_then(|id| self.node_id_for_widget_id(id));
        if self.engine.hover != next {
            self.engine.hover = next;
        }
    }

    /// 🎯️ Hit-tests variadic `+` controls; returns side, node id, and insert index.
    pub fn port_insert_hit(&self, world_x: f64, world_y: f64, zoom: f64) -> Option<(DagPortSide, String, usize)> {
        if zoom < DAG_VARIADIC_PLUS_ZOOM_THRESHOLD {
            return None;
        }
        for node in self.host_snapshot.nodes.iter().rev() {
            if node.variadic_inputs() {
                let inputs = node.inputs();
                let hw = node.width * 0.5;
                let row = inputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, hit_x1, y1) = {
                    let (x0, y0, _x1, y1) = channel_row_bounds(node, row);
                    (x0, y0, node.x - hw * 0.5, y1)
                };
                if point_in_rect(world_x, world_y, x0, y0, hit_x1, y1) {
                    return Some((DagPortSide::Input, node.id.clone(), inputs.len()));
                }
            }
            if node.variadic_outputs() {
                let outputs = node.outputs();
                let row = outputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, x1, y1) = channel_row_bounds(node, row);
                let hit_x0 = computation_output_column_x_bounds(node).map_or(x0, |(left, _)| left);
                if point_in_rect(world_x, world_y, hit_x0, y0, x1, y1) {
                    return Some((DagPortSide::Output, node.id.clone(), outputs.len()));
                }
            }
        }
        None
    }

    /// 🔗️ World-space proximity radius for channel auto-connect; `0` disables snapping.
    pub fn set_proximity_distance(&mut self, world: f64) {
        self.engine.proximity_distance_world = world.max(0.0);
    }

    pub fn host_snapshot_json(&self) -> Result<String, DagError> {
        Ok(semio_framework_pack_json::to_json_string(&self.host_snapshot))
    }

    /// 🌳️ Recomputes node positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts: &DagLayoutOptions,control:&mut LayoutControl<'_>) -> Result<(), DagError> {
        apply_dag_layout(&mut self.host_snapshot,opts,control)?;
        self.rebuild_engine_with_layout(false);
        Ok(())
    }

    pub fn rebuild_engine_with_layout(&mut self, apply_layout: bool) {
        self.engine = DagBoardEngine::new();
        self.engine.enforce_acyclic = true;
        self.node_id_map.clear();
        self.handle_key_map.clear();
        self.handle_port_shape.clear();
        self.handle_port_visible.clear();
        self.edge_id_map.clear();
        self.edge_engine_ids.clear();
        self.edge_engine_ids.resize(self.host_snapshot.edges.len(), None);
        self.edge_route_style.clear();
        for node in &mut self.host_snapshot.nodes {
            fit_node_size(node);
        }
        let (cx, cy, zoom) = (self.host_snapshot.camera.x, self.host_snapshot.camera.y, self.host_snapshot.camera.zoom);
        self.engine.set_camera(cx, cy, zoom);
        if apply_layout {
            let mut progress=|_|true;let mut control=LayoutControl::new(100_000_000,&mut progress);
            let _=apply_dag_layout(&mut self.host_snapshot,&DagLayoutOptions::default(),&mut control);
        }
        let mut next_handle: u64 = 10;
        let mut handle_map: HashMap<String, u64> = HashMap::new();
        for (idx, node) in self.host_snapshot.nodes.iter().enumerate() {
            let nid = idx as u64 + 1;
            self.node_id_map.insert(nid, idx);
            self.engine.create_rect_node(nid, node.x, node.y, node.width, node.height, true);
            let inputs = node.inputs();
            let outputs = node.outputs();
            for (port_idx, port) in inputs.iter().enumerate() {
                let in_a = io_node_rect_port_angle_for_node(node, port_idx, true);
                let hid = next_handle;
                next_handle += 1;
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(dag_port_handle_key(&node.id, &port.id, true), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
                self.engine.create_handle(hid, nid, in_a);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
                self.engine.set_handle_role(hid, HandleRole::Target);
                self.engine.set_handle_value_types(hid, port_value_types(port));
            }
            for (port_idx, port) in outputs.iter().enumerate() {
                let out_a = io_node_rect_port_angle_for_node(node, port_idx, false);
                let hid = next_handle;
                next_handle += 1;
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(dag_port_handle_key(&node.id, &port.id, false), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
                self.engine.create_handle(hid, nid, out_a);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
                self.engine.set_handle_role(hid, HandleRole::Source);
                self.engine.set_handle_value_types(hid, port_value_types(port));
            }
        }
        let existing: Vec<(String, String)> = self
            .host_snapshot
            .edges
            .iter()
            .filter_map(|e| {
                let src = e.source.split('@').next()?.to_string();
                let tgt = e.target.split('@').next()?.to_string();
                Some((src, tgt))
            })
            .collect();
        let mut eid: u64 = 100;
        for (edge_index, edge) in self.host_snapshot.edges.iter().enumerate() {
            if would_create_cycle(&existing, edge.source.split('@').next().unwrap_or(""), edge.target.split('@').next().unwrap_or("")) {
                continue;
            }
            let (source_node, source_port) = dag_port_endpoint_parts(&edge.source);
            let (target_node, target_port) = dag_port_endpoint_parts(&edge.target);
            let src = handle_map.get(&dag_port_handle_key(&source_node, &source_port, false)).copied();
            let tgt = handle_map.get(&dag_port_handle_key(&target_node, &target_port, true)).copied();
            if let (Some(s), Some(t)) = (src, tgt) {
                let id = parse_snapshot_edge_numeric_id(&edge.id).unwrap_or(eid);
                eid = eid.max(id).saturating_add(1);
                self.engine.create_edge(id, s, t);
                self.edge_id_map.insert(id, edge.id.clone());
                self.edge_engine_ids[edge_index] = Some(id);
                self.edge_route_style.insert(id, edge.route_style);
            }
        }
        self.engine.set_next_edge_id(eid);
    }

    pub fn node_spec_for_paint<'a>(&'a self, idx: usize, node: &'a DagNodeSpec) -> std::borrow::Cow<'a, DagNodeSpec> {
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return std::borrow::Cow::Borrowed(node);
        };
        let Some(engine_node) = self.engine.nodes.get(&nid) else {
            return std::borrow::Cow::Borrowed(node);
        };
        if (engine_node.center.x - node.x).abs() < 1e-9 && (engine_node.center.y - node.y).abs() < 1e-9 {
            return std::borrow::Cow::Borrowed(node);
        }
        let mut synced = node.clone();
        synced.x = engine_node.center.x;
        synced.y = engine_node.center.y;
        std::borrow::Cow::Owned(synced)
    }

    pub fn sync_edges_from_engine(&mut self) {
        let mut edges = Vec::with_capacity(self.engine.edges.len());
        let mut edge_engine_ids = Vec::with_capacity(self.engine.edges.len());
        for (eid, edge) in &self.engine.edges {
            let Some(source) = self.handle_key_map.get(&edge.source).cloned() else {
                continue;
            };
            let Some(target) = self.handle_key_map.get(&edge.target).cloned() else {
                continue;
            };
            let id = self.edge_id_map.get(eid).cloned().unwrap_or_else(|| format!("e{eid}"));
            self.edge_id_map.insert(*eid, id.clone());
            edges.push(DagHostSnapshotEdge { id, source, target, ..Default::default() });
            edge_engine_ids.push(Some(*eid));
        }
        self.host_snapshot.edges = edges;
        self.edge_engine_ids = edge_engine_ids;
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.host_snapshot.camera = DagCamera { x, y, zoom };
        self.engine.set_camera(x, y, zoom);
    }
}
