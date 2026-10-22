//! 👆️ Host pointer impl of the DAG host: the pointer method block and the pointer plan derivation. See [`super`].

use super::*;

// #region 🔖️DagHost
impl DagHost {
    pub fn default_demo() -> Self {
        Self::from_host_snapshot(DagHostSnapshot::default())
    }

    pub fn from_host_snapshot(snapshot: DagHostSnapshot) -> Self {
        Self::from_host_snapshot_with_layout(snapshot, false)
    }

    /// 🌳️ Builds a host without running auto-layout (preserves node positions).
    pub fn from_host_snapshot_without_layout(snapshot: DagHostSnapshot) -> Self {
        Self::from_host_snapshot(snapshot)
    }

    /// ♻️ Rebuilds transient DAG state while retaining the exact owner of admitted icon paints.
    pub fn replace_host_snapshot_without_layout(&mut self, snapshot: DagHostSnapshot) {
        let mut next = Self::from_host_snapshot_without_layout(snapshot);
        std::mem::swap(&mut self.paint.icon_paint_cache, &mut next.paint.icon_paint_cache);
        *self = next;
    }

    fn from_host_snapshot_with_layout(snapshot: DagHostSnapshot, apply_layout: bool) -> Self {
        let mut host = Self {
            viewport: DagViewportState {
                canvas_theme: CanvasPalette::default(),
                width: 1,
                height: 1,
                dpr: 1.0,
                last_screen_x: 0.0,
                last_screen_y: 0.0,
                wheel_zoom_active: false,
                wheel_zoom_render_lod: None,
                automatic_lod: true,
                forced_draw_lod: None,
                grid_visible: true,
                grid_snap_enabled: false,
                grid_factor: GRID_FACTOR_DEFAULT,
                pan_anchor: None,
            },
            graph: DagGraphState {
                host_snapshot: snapshot,
                engine: DagBoardEngine::new(),
                node_id_map: HistoryFoldIndex::new(),
                handle_key_map: HistoryFoldIndex::new(),
                handle_port_shape: HistoryFoldIndex::new(),
                handle_port_visible: HistoryFoldIndex::new(),
                edge_id_map: HistoryFoldIndex::new(),
                edge_engine_ids: Vec::new(),
                edge_route_style: HistoryFoldIndex::new(),
                pending_graph_edits: Vec::new(),
                journal_refusal: None,
            },
            pointer: DagPointerState {
                widget_drag: None,
                pending_port_insert: None,
                ghost_node: None,
                pending_cluster_explode: None,
                pending_export_click: None,
                pending_open_instance_id: None,
                last_pointer_down_at_ms: 0.0,
                last_pointer_down_world: (0.0, 0.0),
                last_pointer_down_node_id: None,
            },
            paint: DagPaintState {
                dimmed: HistoryFoldSet::new(),
                icon_paint_cache: semio_framework_os_infinite_board_engine::IconPaintCache::new(),
                computing_active: None,
                computing_stale: NumericSet::new(),
                computing_active_anim_phase: Cell::new(0.0),
                computing_stale_anim_phase: Cell::new(0.0),
                node_eval_status: NumericIndex::new(),
                unresolved_input_ports: NumericSet::new(),
            },
            minimap: DagMinimapState {
                minimap_widget_visible: false,
                minimap_widget_hovered: false,
                minimap_widget_drag: None,
            },
            notes: DagNotesState {
                editing_note: None,
                caret_visible: true,
            },
        };
        host.graph.rebuild_engine_with_layout(apply_layout);
        host
    }
}

pub fn derive_pointer_plan(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, projection: DagInteractionProjection, intent: DagPointerIntent) -> Result<DagPointerPlan, DagInteractionPlanFault> {
    if projection.revision == u64::MAX || graph.host_snapshot.nodes.len() > DAG_INTERACTION_NODE_CAPACITY {
        return Err(DagInteractionPlanFault::NodeCredits);
    }
    let mut next = projection;
    next.revision = projection.revision + 1;
    let mut moves = [None; DAG_INTERACTION_NODE_CAPACITY];
    let mut move_len = 0u16;
    let mut drag = None;
    match intent.phase {
        DagPointerPhase::Down => derive_projection_down(viewport, graph, minimap, &mut next, intent)?,
        DagPointerPhase::Move => derive_projection_move(viewport, graph, minimap, &mut next, &mut moves, &mut move_len, &mut drag, intent)?,
        DagPointerPhase::Up => {
            derive_projection_move(viewport, graph, minimap, &mut next, &mut moves, &mut move_len, &mut drag, intent)?;
            next.gesture = DagProjectionGesture::Idle;
        }
        DagPointerPhase::Leave => {
            next.gesture = DagProjectionGesture::Idle;
            next.hover = None;
        }
    }
    Ok(DagPointerPlan { expected_revision: projection.revision, previous_active: !matches!(projection.gesture, DagProjectionGesture::Idle), next, moves, move_len, drag })
}
