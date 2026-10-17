//! 🧱️ Jack query preparation: the bounded preparation steps and the `QueryExecutionPreparation` state machine.

use super::*;

/// 🧱 One preparation turn clones one snapshot metadata field or one working-scene entity.
pub enum QueryPreparationStep {
    Pending,
    Complete(Box<QueryExecution>),
}

/// 🧱 Incremental snapshot-to-query-workspace preparation that never clones the complete fixture.
pub struct QueryExecutionPreparation {
    pub(super) query: Option<Query>,
    metadata: JackSnapshotCloneAuthority,
    pub(super) closing_owner: Option<QueryClosingOwner>,
    pub(super) retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(super) graph: Option<Graph>,
    pub(super) node: usize,
    pub(super) edge: usize,
    pub(super) maximum_result_owned_bytes: usize,
    pub(super) closing: bool,
    pub(super) terminal: bool,
}

impl QueryExecutionPreparation {
    pub fn new(query:Query)->Self{Self::with_result_grant(query,QUERY_RESULT_MAXIMUM_OWNED_BYTES)}

    pub fn with_result_grant(query:Query,maximum_result_owned_bytes:usize) -> Self {
        Self { query: Some(query), metadata: JackSnapshotCloneAuthority::metadata_only(), closing_owner: None, retirement: None, graph: None, node: 0, edge: 0, maximum_result_owned_bytes,closing:false,terminal:false }
    }

    pub fn step(&mut self, snapshot: &JackSnapshot, scene: &dyn crate::standards::v1::subsets::any::schema::content_read::JackContentProjection, maximum_bytes: usize) -> Result<QueryPreparationStep, ValueError> {
        if self.closing || self.terminal {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query preparation is terminal"));
        }
        semio_s_artifact_trinity_jack_graph_query::retained_jack_graph(crate::standards::v1::subsets::any::schema::content::jack_content_identity(&snapshot.content),scene)?;
        if self.graph.is_none() {
            match self.metadata.advance(snapshot, maximum_bytes)? {
                JackSnapshotCloneStep::Pending { .. } => {
                    return Ok(QueryPreparationStep::Pending);
                }
                JackSnapshotCloneStep::Complete => {}
            }
            let mut metadata = self.metadata.take_value().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated,"query metadata clone did not transfer its owner"))?;
            if metadata.schema != JackSnapshot::SCHEMA {
                let error = ValueError::new(ValueRefusalKind::InvalidValue,format!("query snapshot schema '{}' is invalid", metadata.schema));
                self.closing_owner = Some(QueryClosingOwner::Metadata(metadata));
                return Err(error);
            }
            let graph = Graph {
                name: std::mem::take(&mut metadata.name),
                manifest_id: metadata.manifest_id.take(),
                manifest: std::mem::take(&mut metadata.manifest),
                camera: metadata.camera.clone(),
                nodes: BTreeMap::new(),
                edges: BTreeMap::new(),
                root_node_id: metadata.root_node_id.take(),
                query: std::mem::take(&mut metadata.query),
            };
            crate::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(metadata)?;
            self.graph = Some(graph);
            return Ok(QueryPreparationStep::Pending);
        }
        let (nodes,edges)=scene.projection_counts();
        if nodes>QUERY_ENTITY_MAXIMUM||edges>QUERY_ENTITY_MAXIMUM{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query snapshot exceeds its entity admission"))}
        if self.node<nodes {
            let id=scene.query_node_id(self.node).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"query node index is not retained"))?;
            let graph=self.graph.as_mut().expect("query preparation graph is initialized");
            if graph.nodes.contains_key(id){return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("duplicate node id {id}")))}
            let node=scene.query_node(self.node,maximum_bytes)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"query node is not retained"))?;
            graph.nodes.insert(node.id.clone(),node);
            self.node+=1;
            return Ok(QueryPreparationStep::Pending);
        }
        if self.edge<edges {
            let id=scene.query_edge_id(self.edge).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"query edge index is not retained"))?;
            let graph=self.graph.as_mut().expect("query preparation graph is initialized");
            if graph.edges.contains_key(id){return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("duplicate edge id {id}")))}
            let edge=scene.query_edge(self.edge,maximum_bytes)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"query edge is not retained"))?;
            graph.edges.insert(edge.id.clone(),edge);
            self.edge+=1;
            return Ok(QueryPreparationStep::Pending);
        }
        let graph = self.graph.take().expect("query preparation graph remains owned");
        let query = self.query.take().expect("query preparation AST remains owned");
        self.terminal = true;
        Ok(QueryPreparationStep::Complete(Box::new(QueryExecution::with_result_grant(graph,query,self.maximum_result_owned_bytes))))
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    /// 📏️ Quotes the next close turn: the retained child cursor, a staged typed owner, the partial metadata clone, else one extraction move.
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.closing_owner.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.closing_owner);
        }
        if !self.metadata.terminal_is_empty() {
            return self.metadata.close_demands(body);
        }
        if self.graph.is_some() || self.query.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<QueryClosingOwner>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if !self.closing || grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if !admit_demand(grant, demand, "query preparation close exceeds admitted depth")? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.closing_owner.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.closing_owner, &mut self.retirement, grant);
        }
        if !self.metadata.terminal_is_empty() {
            return self.metadata.close_step(grant);
        }
        let extracted = if let Some(graph) = self.graph.as_mut() {
            if let Some((_, node)) = graph.nodes.pop_first() {
                Some(QueryClosingOwner::Effect(GraphEffect::CreateNode(node)))
            } else if let Some((_, edge)) = graph.edges.pop_first() {
                Some(QueryClosingOwner::Effect(GraphEffect::CreateEdge(edge)))
            } else {
                let metadata = graph_metadata_owner(graph);
                self.graph = None;
                Some(QueryClosingOwner::Metadata(metadata))
            }
        } else {
            self.query.take().map(QueryClosingOwner::Query)
        };
        match extracted {
            Some(owner) => {
                self.closing_owner = Some(owner);
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
            }
            None => {
                self.terminal = true;
                Ok(RetainedCloneStep::Complete(empty))
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.query.is_none() && self.graph.is_none() && self.closing_owner.is_none() && self.retirement.is_none() && self.metadata.terminal_is_empty()
    }
}
