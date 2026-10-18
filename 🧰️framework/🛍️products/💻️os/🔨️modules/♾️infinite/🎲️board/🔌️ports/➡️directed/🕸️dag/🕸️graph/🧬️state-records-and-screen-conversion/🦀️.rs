//! 🧬️ State records and screen conversion of the DAG graph: the viewport state, the graph state and the DAG screen and world conversions. See [`super`].

use super::*;

/// 🧩️ DagHost viewport component: size, device ratio, LOD and grid policy, theme and pan anchor of the canvas.
pub struct DagViewportState {
    pub canvas_theme: CanvasPalette,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
    pub last_screen_x: f64,
    pub last_screen_y: f64,
    pub wheel_zoom_active: bool,
    pub wheel_zoom_render_lod: Option<DagDrawLod>,
    pub automatic_lod: bool,
    pub forced_draw_lod: Option<DagDrawLod>,
    pub grid_visible: bool,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
    pub pan_anchor: Option<(f64, f64, f64, f64)>,
}

/// 🧩️ DagHost graph component: the host snapshot, the board engine and the id maps between them, with pending wire edits.
pub struct DagGraphState {
    pub host_snapshot: DagHostSnapshot,
    pub engine: DagBoardEngine,
    pub node_id_map: HistoryFoldIndex<NodeId, usize>,
    pub handle_key_map: HistoryFoldIndex<HandleId, String>,
    pub handle_port_shape: HistoryFoldIndex<HandleId, PortShape>,
    pub handle_port_visible: HistoryFoldIndex<HandleId, bool>,
    pub edge_id_map: HistoryFoldIndex<EdgeId, String>,
    pub edge_engine_ids: Vec<Option<EdgeId>>,
    pub edge_route_style: HistoryFoldIndex<EdgeId, EdgeRouteStyle>,
    /// 🔗️ Wire edits this host performed on its OWN geometry since the last drain, for a renderer to
    /// forward to the guest that owns the graph — see [`DagGraphEdit`].
    pub pending_graph_edits: Vec<DagGraphEdit>,
    /// 🚫️ Why the last gesture was refused whole instead of journalled — its moves outgrew one dispatch — until a renderer
    /// drains it ([`Self::take_journal_refusal`]).
    pub journal_refusal: Option<DagJournalRefusal>,
}

/// 🧭️ Screen coordinates to world coordinates for the current viewport.
pub fn dag_screen_to_world(viewport: &DagViewportState, graph: &DagGraphState, sx: f64, sy: f64) -> (f64, f64) {
    let point = screen_to_world_point(viewport, graph, sx, sy);
    (point.x, point.y)
}

/// 👻️ World coordinates to surface-local pixels through the DAG's live camera.
pub fn dag_world_to_screen(viewport: &DagViewportState, graph: &DagGraphState, wx: f64, wy: f64) -> (f64, f64) {
    world_to_screen_point(viewport, graph, wx, wy)
}
