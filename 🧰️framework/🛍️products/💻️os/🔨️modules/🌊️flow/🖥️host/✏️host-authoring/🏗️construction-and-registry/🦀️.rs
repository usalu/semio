//! 🏗️ Construction and registry of the flow host: the operator registry and geometry port builders, the history store factory and the host snapshot constructors. See [`super`].

use super::*;

impl FlowHost {
    /// 📔️ Supplies immutable operator implementations owned by this host's composition.
    pub fn with_operator_registry(mut self, registry: neural::SharedRegistry) -> Self {
        assert!(self.operator_registry.is_none(), "operator registry is supplied once");
        self.set_neuron_kind_info_map(Arc::new(registry.operator_infos().map(|info| (info.id.clone(),info.clone())).collect()));
        self.operator_registry = Some(registry);
        self.baseline_registry_generation = 0;
        self
    }
    /// 🔗️ Binds the actual preborn registry authority without constructing a global catalogue.
    pub fn bind_operator_registry_step(&mut self,registry:&neural::SharedRegistry,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if self.operator_registry.as_ref().is_some_and(|source|source.owner_identity()==registry.owner_identity()){return Ok(Default::default())}
        if self.operator_registry.is_some(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original registry authority must close before replacement"))}
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original registry lease requires one admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original registry lease requires admitted depth"))}
        self.operator_registry=Some(registry.clone());self.evaluation_revision=self.evaluation_revision.wrapping_add(1);Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }
    pub fn operator_registry(&self) -> neural::SharedRegistry { self.operator_registry.clone().unwrap_or_else(flow_operator_registry) }
    pub(in super::super) fn operator_registry_generation(&self) -> u64 { if self.operator_registry.is_some() { 0 } else { flow_extension_registry_generation() } }
    /// 🔌️ Supplies one geometry authority that this owner retains and explicitly closes.
    pub fn with_geometry_port(mut self, port: Box<dyn semio_framework_os_flow_geometry::GeometryPort>) -> Self {
        assert!(self.geometry_port.is_none(), "geometry authority is supplied once");
        self.geometry_port = Some(port);
        self
    }
    pub fn geometry_port(&self) -> Result<&dyn semio_framework_os_flow_geometry::GeometryPort, String> {
        self.geometry_port.as_deref().ok_or_else(|| "flow.geometry-port-missing".into())
    }
    /// 🏭️ Selects the actual baseline Store producer supplied by this host's child composition.
    pub fn with_history_store_factory(mut self,producer:semio_framework_os_kernel::os_store::OriginalStoreGenesisFactory<FlowHostSnapshot,FlowMutation>)->Self{assert!(self.history_store_factory.is_none()&&self.history_store.is_none(),"original history producer is supplied once before Store birth");self.history_store_factory=Some(producer);self}
    pub fn from_host_snapshot(host_snapshot: FlowHostSnapshot) -> Self {
        Self::from_host_snapshot_with_cache(host_snapshot, Arc::new(NeuralCache::new()))
    }

    /// 🧠️ Builds a host sharing an existing [`NeuralCache`] — lets a long-lived caller (e.g. a
    /// stateless request/response program boundary that reconstructs `FlowHost` on every call)
    /// keep per-node memoization alive across those reconstructions instead of discarding it.
    pub fn from_host_snapshot_with_cache(host_snapshot: FlowHostSnapshot, neural_cache: Arc<NeuralCache>) -> Self {
        Self::from_host_snapshot_with_cache_and_infos(host_snapshot, neural_cache, Arc::default())
    }

    /// 🏠️ Builds a host that ALREADY indexes `kind_infos`. The ONE construction path for a caller
    /// that would set the operator catalogue immediately afterwards: the bare constructor builds its
    /// dag against an empty catalogue, and the setter's own `rebuild_dag` then throws that work away
    /// — twice per evaluation tick (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn from_host_snapshot_with_cache_and_infos(mut host_snapshot: FlowHostSnapshot, neural_cache: Arc<NeuralCache>, kind_infos: Arc<HistoryFoldIndex<String, OperatorInfo>>) -> Self {
        dedupe_host_snapshot_widgets(&mut host_snapshot);
        let mut host = Self {
            geometry_port: None,
            operator_registry: None,
            host_snapshot: host_snapshot,
            dag: DagHost::from_host_snapshot(DagHostSnapshot { schema: "dag.hostDocument".into(), camera: semio_framework_os_infinite_directed_presentation::DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            current_channels: None,
            export_payloads: HistoryFoldIndex::new(),
            last_eval_json: String::new(),
            host_catalogue_json: String::new(),
            kind_infos,
            neural_cache,
            previous_snapshot: None,
            previous_channels: None,
            pending_baseline_publication:None,
            baseline_retirement:None,
            baseline_publication_progress:Default::default(),
            pending_evaluation:None,
            evaluation_revision:0,
            evaluation_progress:Default::default(),
            baseline_registry_generation: flow_extension_registry_generation(),
            next_widget_serial: 1,
            next_synapse_serial: 100,
            viewport_w: 1,
            viewport_h: 1,
            viewport_dpr: 1.0,
            pan_anchor: None,
            ghost_node: None,
            history_store: None,
            history_store_factory:None,
            pending_history_baseline: None,
            pending_change: false,
            pending_leaves: Vec::new(),
            history_fault: None,
            #[cfg(test)]
            recorded: Vec::new(),
            journal_mark: 0,
            edited_note: None,
            gesture_active: false,
            pending_extension_evals: Vec::new(),
            pending_remaining: Vec::new(),
            interaction_revision: 0,
            interaction_projection: None,
            displaced: neural::ValueRetirement::default(),
        };
        host.rebuild_dag();
        host.refresh_interaction_projection();
        host
    }
}
