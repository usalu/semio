//! 🔌️ Construction and geometry port of the evaluation session: the operator registry and geometry authority builders and the session constructor. See [`super`].

use super::*;

impl FlowEvalSession {
    /// 📔️ Shares the explicitly supplied registry with this session's evaluation hosts.
    pub fn with_operator_registry(mut self, registry: neural::SharedRegistry) -> Self {
        assert!(!self.closing, "closed session refuses an operator registry");
        assert!(self.state.operator_registry.is_none(), "operator registry is supplied once");
        self.state.operator_registry = Some(registry);
        self.state.flow_extension_generation = 0;
        self
    }
    /// 🔌️ Supplies one geometry authority that this owner retains and explicitly closes.
    pub fn with_geometry_port(mut self, port: Box<dyn semio_framework_os_flow_geometry::GeometryPort>) -> Self {
        assert!(!self.closing, "closed session refuses a geometry authority");
        assert!(self.state.geometry_port.is_none(), "geometry authority is supplied once");
        self.state.geometry_port = Some(port);
        self
    }
    /// 🌐️ Reads the explicitly supplied geometry authority for this evaluation session.
    pub fn geometry_port(&self) -> Option<&dyn semio_framework_os_flow_geometry::GeometryPort> { self.state.geometry_port.as_deref() }

    pub fn new() -> Self {
        Self {
            state: std::mem::ManuallyDrop::new(FlowEvalSessionState {
                retiring_geometry_port: None,
                geometry_port: None,
                operator_registry: None,
                neural_cache: Some(Arc::new(NeuralCache::new())),
                previous_snapshot: None,
                previous_channels: None,
                eval_json: Some(Arc::new(String::new())),
                painted_eval_json: String::new(),
                converged_eval_json: String::new(),
                status_json: "{}".into(),
                tick_scheduled: false,
                window_tick_latches: HistoryFoldIndex::new(),
                live_geometry_handles: HistoryFoldIndex::new(),
                preview_mesh_by_handle: HistoryFoldIndex::new(),
                pending_tessellate_by_hash: HistoryFoldIndex::new(),
                tessellate_handle_by_hash: HistoryFoldIndex::new(),
                tessellate_progress_by_hash: HistoryFoldIndex::new(),
                eval_progress_by_hash: HistoryFoldIndex::new(),
                preview_diagnostics_by_handle: HistoryFoldIndex::new(),
                retirement: SessionRetirement::default(),
                flow_extension_generation: flow_extension_registry_generation(),
                extension_evaluate_fault: None,
                preview_cancelled: false,
                published_preview_mesh_digest: 0,
                baseline_geometry_pending:false,
                invocation_origin_growth:None,
                invocation_origins:Vec::new(),
                pending_tick_display:None,
                tick_display_progress:Default::default(),
                tick_evaluation_ready:0,
                pending_host:None,
                pending_snapshot_copy:None,
                pending_tick_snapshot:None,
                pending_tick_snapshot_retirement:None,
                pending_tick_snapshot_origin:None,
                retiring_host:None,
                pending_host_cancelled:false,
                preview_cancellation_cursor:None,
                preview_cancellation_phase:0,
                preview_cancellation_progress:Default::default(),
                preview_retention:None,
                preview_retention_progress:Default::default(),
                closing: false,
            }),
        }
    }
}
