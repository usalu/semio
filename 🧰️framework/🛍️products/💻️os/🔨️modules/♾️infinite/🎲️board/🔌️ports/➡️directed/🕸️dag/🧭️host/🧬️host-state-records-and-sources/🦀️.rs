//! 🧬️ Host state records and sources of the DAG host: the selection source, the host and state structs, the pending open instance id and the output candidates. See [`super`].

use super::*;

impl DagSelectionSource for DagHost {
    fn selection_candidate_count(&self,domain:DagSelectionDomain)->usize {
        match domain {DagSelectionDomain::Nodes=>self.graph.host_snapshot.nodes.len(),DagSelectionDomain::Edges=>self.graph.host_snapshot.edges.len()}
    }
    fn selection_candidate_id(&self,domain:DagSelectionDomain,index:usize)->Option<&str>{
        match domain {
            DagSelectionDomain::Nodes=>{let id=u64::try_from(index.checked_add(1)?).ok()?;(self.graph.node_id_map.get(&id)==Some(&index)&&self.graph.engine.selection.node_ids.contains(&id)).then(||self.graph.host_snapshot.nodes.get(index).map(|node|node.id.as_str())).flatten()},
            DagSelectionDomain::Edges=>self.graph.edge_engine_ids.get(index).and_then(|id|*id).filter(|id|self.graph.engine.selection.edge_ids.contains(id)).and_then(|_|self.graph.host_snapshot.edges.get(index).map(|edge|edge.id.as_str())),
        }
    }
}

/// 🌳️ Retained DAG host: typed nodes, edges, engine, camera.
pub struct DagHost {
    /// Size, device ratio, LOD and grid policy, theme and pan anchor of the canvas.
    pub viewport: DagViewportState,
    /// The host snapshot, the board engine and the id maps between them, with pending wire edits.
    pub graph: DagGraphState,
    /// Widget drag, ghost node, pending click intents and the last pointer down.
    pub pointer: DagPointerState,
    /// Icon paint cache, computing progress, evaluation status and dimming.
    pub paint: DagPaintState,
    /// The minimap widget state.
    pub minimap: DagMinimapState,
    /// The inline note editing state.
    pub notes: DagNotesState,
}

/// 🧩️ DagHost pointer component: widget drag, ghost node, pending click intents and the last pointer down.
pub struct DagPointerState {
    pub widget_drag: Option<usize>,
    pub pending_port_insert: Option<(DagPortSide, String, usize)>,
    pub ghost_node: Option<DagNodeSpec>,
    pub pending_cluster_explode: Option<String>,
    pub pending_export_click: Option<String>,
    pub pending_open_instance_id: Option<String>,
    pub last_pointer_down_at_ms: f64,
    pub last_pointer_down_world: (f64, f64),
    pub last_pointer_down_node_id: Option<String>,
}

/// 🧩️ DagHost paint component: icon paint cache, computing progress, evaluation status and dimming.
pub struct DagPaintState {
    pub dimmed: HistoryFoldSet<NodeId>,
    pub icon_paint_cache: graph::IconPaintCache,
    pub computing_active: Option<NodeId>,
    pub computing_stale: NumericSet<NodeId>,
    pub computing_active_anim_phase: Cell<f64>,
    pub computing_stale_anim_phase: Cell<f64>,
    pub node_eval_status: NumericIndex<NodeId, DagNodeEvalStatusKind>,
    pub unresolved_input_ports: NumericSet<(NodeId,usize)>,
}

/// 🧩️ DagHost minimap component: the minimap widget state.
pub struct DagMinimapState {
    pub minimap_widget_visible: bool,
    pub minimap_widget_hovered: bool,
    pub minimap_widget_drag: Option<(f64, f64)>,
}

/// 🧩️ DagHost notes component: the inline note editing state.
pub struct DagNotesState {
    pub editing_note: Option<NoteEditState>,
    pub caret_visible: bool,
}

/// 🪟️ Takes a pending double-click open request for an app instance node.
pub fn dag_take_pending_open_instance_id(pointer: &mut DagPointerState) -> Option<String> {
    pointer.pending_open_instance_id.take()
}

impl DagOutputCandidates for DagHost{
    fn node_candidate_count(&self)->usize{self.graph.engine.selection.node_ids.len()}
    fn node_candidate_id(&self,index:usize)->Option<&str>{self.graph.widget_id_for_node_id_ref(*self.graph.engine.selection.node_ids.key_at_rank(index)?)}
    fn edge_candidate_count(&self)->usize{self.graph.engine.selection.edge_ids.len()}
    fn edge_candidate_id(&self,index:usize)->Option<&str>{self.graph.edge_id_map.get(self.graph.engine.selection.edge_ids.key_at_rank(index)?).map(String::as_str)}
    fn channel_candidate_count(&self)->usize{self.graph.engine.selection.handle_ids.len()}
    fn channel_candidate(&self,index:usize)->Option<DagChannelView<'_>>{self.graph.channel_view(*self.graph.engine.selection.handle_ids.key_at_rank(index)?)}
    fn handle_candidate_identity(&self,index:usize)->Option<&str>{let id=self.graph.engine.selection.handle_ids.key_at_rank(index)?;self.graph.channel_view(*id)?;self.graph.handle_key_map.get(id).map(String::as_str)}
    fn hovered_channel_view(&self)->Option<DagChannelView<'_>>{self.graph.engine.hover.and_then(|id|self.graph.channel_view(id))}
    fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>{
        let(source,target)=self.graph.engine.wire_drag_type_refusal()?;
        Some(DagRefusalView{source:self.graph.handle_key_map.get(&source)?,source_types:self.graph.engine.handles.get(&source).map_or(&[],|handle|handle.value_types.as_slice()),target:self.graph.handle_key_map.get(&target)?,target_types:self.graph.engine.handles.get(&target).map_or(&[],|handle|handle.value_types.as_slice())})
    }
}
