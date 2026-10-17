//! ▶️ Jack query execution: the `QueryExecution` driver that runs the clauses of a prepared query under the step budget.

use super::*;

/// 🔎️ Owns query evaluation state; each step advances one bounded cursor unit.
pub struct QueryExecution{
    pub(super) graph: Graph,
    pub(super) query: Option<Query>,
    clause: usize,
    pub(super) item: usize,
    matching: Option<PatternExecution>,
    filtering: Option<std::vec::IntoIter<Binding>>,
    pub(super) bindings: Vec<Binding>,
    return_clause: Option<usize>,
    returning: Option<ReturnExecution>,
    pending: VecDeque<GraphEffect>,
    deleting: Option<DeleteExecution>,
    pub(super) operations: Vec<GraphEffect>,
    pub(super) maximum_result_owned_bytes: usize,
    pending_clause_advance: bool,
    finished: bool,
    pub(super) closing: bool,
    pub(super) closing_owner: Option<QueryClosingOwner>,
    pub(super) retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    metadata_retired: bool,
    pub(super) terminal: bool,
}

impl QueryExecution {
    pub fn new(graph:Graph,query:Query) -> Self {
        Self::with_result_grant(graph,query,QUERY_RESULT_MAXIMUM_OWNED_BYTES)
    }
    pub fn with_result_grant(graph:Graph,query:Query,maximum_result_owned_bytes:usize)->Self{
        Self {
            graph,
            query: Some(query),
            clause: 0,
            item: 0,
            matching: None,
            filtering: None,
            bindings: vec![Binding::default()],
            return_clause: None,
            returning: None,
            pending: VecDeque::new(),
            deleting: None,
            operations: Vec::new(),
            maximum_result_owned_bytes,
            pending_clause_advance: false,
            finished: false,
            closing: false,
            closing_owner: None,
            retirement: None,
            metadata_retired: false,
            terminal: false,
        }
    }
    fn advance_clause(&mut self) {
        self.clause += 1;
        self.item = 0;
        self.matching = None;
        self.filtering = None;
    }
    fn schedule(&mut self, operations: Vec<GraphEffect>) -> Result<(), ValueError> {
        if self.pending.len().saturating_add(operations.len()).saturating_add(self.operations.len()) > QUERY_ENTITY_MAXIMUM {
            return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query mutations exceed their admission"));
        }
        self.pending.extend(operations);
        self.pending_clause_advance = true;
        Ok(())
    }
    fn mutation_step(&mut self) -> Result<bool, ValueError> {
        if let Some(deleting) = self.deleting.as_mut() {
            let next = match deleting.edge.as_ref() {
                Some(previous) => self
                    .graph
                    .edges
                    .range::<str, _>((Excluded(previous.as_str()), Unbounded))
                    .next()
                    .map(|(id, edge)| (id.clone(), crate::standards::v1::subsets::any::schema::graph::port_node_id(&edge.source) == Some(deleting.id.as_str()) || crate::standards::v1::subsets::any::schema::graph::port_node_id(&edge.target) == Some(deleting.id.as_str()))),
                None => self.graph.edges.first_key_value().map(|(id, edge)| (id.clone(), crate::standards::v1::subsets::any::schema::graph::port_node_id(&edge.source) == Some(deleting.id.as_str()) || crate::standards::v1::subsets::any::schema::graph::port_node_id(&edge.target) == Some(deleting.id.as_str()))),
            };
            if let Some((edge, incident)) = next {
                deleting.edge = Some(edge.clone());
                if incident {
                    if let Some(removed) = self.graph.edges.remove(&edge) {
                        retire_displaced(GraphEffect::CreateEdge(removed))?;
                    }
                }
                return Ok(true);
            }
            if let Some(removed) = self.graph.nodes.remove(&deleting.id) {
                retire_displaced(GraphEffect::CreateNode(removed))?;
            }
            if self.graph.root_node_id.as_deref() == Some(deleting.id.as_str()) {
                self.graph.root_node_id = None;
            }
            self.operations.push(self.deleting.take().expect("delete cursor remains owned").operation);
            return Ok(true);
        }
        let Some(operation) = self.pending.pop_front() else {
            if self.pending_clause_advance {
                self.pending_clause_advance = false;
                self.advance_clause();
                return Ok(true);
            }
            return Ok(false);
        };
        let applied = match &operation {
            GraphEffect::CreateNode(node) => {
                if self.graph.nodes.contains_key(&node.id) {
                    Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("node {} already exists", node.id)))
                } else {
                    self.graph.nodes.insert(node.id.clone(), node.clone());
                    Ok(())
                }
            }
            GraphEffect::DeleteNode(id) => {
                self.deleting = Some(DeleteExecution { id: id.clone(), edge: None, operation });
                return Ok(true);
            }
            GraphEffect::CreateEdge(edge) => {
                if self.graph.edges.contains_key(&edge.id) {
                    Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("edge {} already exists", edge.id)))
                } else {
                    self.graph.edges.insert(edge.id.clone(), edge.clone());
                    Ok(())
                }
            }
            GraphEffect::DeleteEdge(id) => {
                if let Some(removed) = self.graph.edges.remove(id) {
                    retire_displaced(GraphEffect::CreateEdge(removed))?;
                }
                Ok(())
            }
            GraphEffect::RenameNode { id, name } => match self.graph.nodes.get_mut(id) {
                Some(node) => {
                    let previous = std::mem::replace(&mut node.name, name.clone());
                    retire_displaced(GraphEffect::DeleteNode(previous))?;
                    Ok(())
                }
                None => Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("node {id} not found"))),
            },
            GraphEffect::MoveNode { id, x, y } => match self.graph.nodes.get_mut(id) {
                Some(node) => {
                    node.x = *x;
                    node.y = *y;
                    Ok(())
                }
                None => Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("node {id} not found"))),
            },
            GraphEffect::SetProperty { entity, key, value } => {
                let bag = match entity {
                    EntityRef::Node(id) => self.graph.nodes.get_mut(id).map(|node| &mut node.properties),
                    EntityRef::Edge(id) => self.graph.edges.get_mut(id).map(|edge| &mut edge.properties),
                };
                match bag {
                    Some(bag) => {
                        if let Some(previous) = bag.insert(key.clone(), value.clone()) {
                            retire_displaced(GraphEffect::SetProperty { entity: EntityRef::Node(String::new()), key: String::new(), value: previous })?;
                        }
                        Ok(())
                    }
                    None => Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{entity:?} not found"))),
                }
            }
            GraphEffect::RemoveProperty { entity, key } => {
                let bag = match entity {
                    EntityRef::Node(id) => self.graph.nodes.get_mut(id).map(|node| &mut node.properties),
                    EntityRef::Edge(id) => self.graph.edges.get_mut(id).map(|edge| &mut edge.properties),
                };
                match bag {
                    Some(bag) => {
                        if let Some(previous) = bag.remove(key) {
                            retire_displaced(GraphEffect::SetProperty { entity: EntityRef::Node(String::new()), key: String::new(), value: previous })?;
                        }
                        Ok(())
                    }
                    None => Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("{entity:?} not found"))),
                }
            }
        };
        if let Err(error) = applied {
            self.pending.push_front(operation);
            return Err(error);
        }
        self.operations.push(operation);
        Ok(true)
    }
    pub fn step(&mut self) -> Result<Option<(QueryResult, Vec<GraphEffect>)>, ValueError> {
        if self.finished || self.closing {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query execution is terminal"));
        }
        if self.mutation_step()? {
            return Ok(None);
        }
        let query = self.query.take().expect("query AST remains owned");
        let result = self.query_step(&query);
        self.query = Some(query);
        result
    }
    fn query_step(&mut self, query: &Query) -> Result<Option<(QueryResult, Vec<GraphEffect>)>, ValueError> {
        let Some(clause) = query.clauses.get(self.clause) else {
            if self.return_clause.is_none() {
                add_result_owned(&mut 0, size_of::<QueryResult>(), self.maximum_result_owned_bytes)?;
                self.finished = true;
                return Ok(Some((QueryResult::table(Vec::new(), Vec::new()), std::mem::take(&mut self.operations))));
            }
            let maximum_result_owned_bytes = self.maximum_result_owned_bytes;
            let returning = self.returning.get_or_insert_with(|| ReturnExecution::new(maximum_result_owned_bytes));
            let items = self
                .return_clause
                .and_then(|index| query.clauses.get(index))
                .and_then(|clause| match clause {
                    Clause::Return(items) => Some(items.as_slice()),
                    _ => None,
                })
                .unwrap_or(&[]);
            if let Some(result) = returning.step(&mut self.graph, &self.bindings, items)? {
                self.finished = true;
                return Ok(Some((result, std::mem::take(&mut self.operations))));
            }
            return Ok(None);
        };
        match clause {
            Clause::Match(patterns) => {
                let matching = self.matching.get_or_insert_with(PatternExecution::new);
                if let Some(bindings) = matching.step(&self.graph, patterns)? {
                    self.bindings = bindings;
                    self.advance_clause();
                }
            }
            Clause::Where(expr) => {
                if self.filtering.is_none() {
                    self.filtering = Some(std::mem::take(&mut self.bindings).into_iter());
                }
                match self.filtering.as_mut().expect("filter cursor is initialized").next() {
                    Some(binding) => {
                        if eval_expr(&self.graph, &binding, expr) {
                            self.bindings.push(binding);
                        }
                    }
                    None => self.advance_clause(),
                }
            }
            Clause::Return(_) => {
                self.return_clause = Some(self.clause);
                self.advance_clause();
            }
            Clause::Create(pattern) => self.schedule(emit_create_operations_from_graph(&self.graph, pattern)?)?,
            Clause::Delete(vars) => {
                if let Some(var) = vars.get(self.item) {
                    if let Some(id) = self.bindings.first().and_then(|binding| binding.nodes.get(var).cloned()) {
                        self.pending.push_back(GraphEffect::DeleteNode(id));
                    }
                    self.item += 1;
                } else {
                    self.advance_clause();
                }
            }
            Clause::Set(items) => {
                if let Some(item) = items.get(self.item) {
                    if let Some(id) = self.bindings.first().and_then(|binding| binding.nodes.get(&item.var)) {
                        self.pending.push_back(emit_set_operation_from_graph(&self.graph, id, &item.prop, item.value.clone())?);
                    }
                    self.item += 1;
                } else {
                    self.advance_clause();
                }
            }
            Clause::Merge(pattern) => {
                let matching = self.matching.get_or_insert_with(PatternExecution::new);
                if let Some(bindings) = matching.step(&self.graph, std::slice::from_ref(pattern))? {
                    if bindings.is_empty() {
                        self.schedule(emit_create_operations_from_graph(&self.graph, pattern)?)?;
                    } else {
                        self.advance_clause();
                    }
                }
            }
        }
        Ok(None)
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
    }
    /// 🧹️ Whether any owner other than the staged one and its retirement cursor remains in the cursor's own fields.
    fn residual_is_empty(&self) -> bool {
        self.query.is_none()
            && self.graph.nodes.is_empty()
            && self.graph.edges.is_empty()
            && self.bindings.is_empty()
            && self.matching.is_none()
            && self.filtering.is_none()
            && self.returning.is_none()
            && self.pending.is_empty()
            && self.deleting.is_none()
            && self.operations.is_empty()
            && self.metadata_retired
    }

    /// 📏️ Quotes the next close turn: the retained child cursor, a staged typed owner, else one extraction move.
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.closing_owner.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.closing_owner);
        }
        if !self.residual_is_empty() {
            return Ok(RetirementDemand { copy_bytes: size_of::<QueryClosingOwner>().max(size_of::<Binding>()), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    /// 📤️ Moves the next owned piece into a typed closing owner; `Ok(None)` is a pure move that stages nothing.
    fn extract_closing_owner(&mut self) -> Option<Option<QueryClosingOwner>> {
        let effect = |effect: GraphEffect| Some(Some(QueryClosingOwner::Effect(effect)));
        if let Some(deleting) = self.deleting.as_mut() {
            if let Some(value) = deleting.edge.take() {
                return effect(GraphEffect::DeleteNode(value));
            }
            if !deleting.id.is_empty() {
                let value = std::mem::take(&mut deleting.id);
                return effect(GraphEffect::DeleteNode(value));
            }
        }
        if let Some(deleting) = self.deleting.take() {
            return effect(deleting.operation);
        }
        if let Some(mutation) = self.pending.pop_front().or_else(|| self.operations.pop()) {
            return effect(mutation);
        }
        if let Some(filtering) = self.filtering.as_mut() {
            if let Some(binding) = filtering.next() {
                self.bindings.push(binding);
                return Some(None);
            }
            self.filtering = None;
            return Some(None);
        }
        if let Some(matching) = self.matching.as_mut() {
            if let Some(binding) = matching.bindings.pop().or_else(|| matching.next.pop()) {
                self.bindings.push(binding);
                return Some(None);
            }
            if let Some(value) = matching.node.take().or_else(|| matching.edge.take()) {
                return effect(GraphEffect::DeleteNode(value));
            }
            self.matching = None;
            return Some(None);
        }
        if let Some(returning) = self.returning.as_mut() {
            if let Some(node) = returning.nodes.pop() {
                return effect(GraphEffect::CreateNode(node));
            }
            if let Some(edge) = returning.edges.pop() {
                return effect(GraphEffect::CreateEdge(edge));
            }
            if let Some(value) = returning.rows.last_mut().and_then(Vec::pop) {
                return effect(GraphEffect::SetProperty { entity: EntityRef::Node(String::new()), key: String::new(), value });
            }
            if returning.rows.pop().is_some() {
                return Some(None);
            }
            if let Some(value) = returning.columns.pop().or_else(|| returning.node_ids.pop_first()).or_else(|| returning.edge_ids.pop_first()) {
                return effect(GraphEffect::DeleteNode(value));
            }
            self.returning = None;
            return Some(None);
        }
        if let Some(binding) = self.bindings.last_mut() {
            if let Some((key, value)) = binding.nodes.pop_first().or_else(|| binding.edges.pop_first()) {
                return effect(GraphEffect::RemoveProperty { entity: EntityRef::Node(value), key });
            }
            self.bindings.pop();
            return Some(None);
        }
        if let Some((_, node)) = self.graph.nodes.pop_first() {
            return effect(GraphEffect::CreateNode(node));
        }
        if let Some((_, edge)) = self.graph.edges.pop_first() {
            return effect(GraphEffect::CreateEdge(edge));
        }
        if !self.metadata_retired {
            self.metadata_retired = true;
            return Some(Some(QueryClosingOwner::Metadata(graph_metadata_owner(&mut self.graph))));
        }
        self.query.take().map(|query| Some(QueryClosingOwner::Query(query)))
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
        if !admit_demand(grant, demand, "query execution close exceeds admitted depth")? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.closing_owner.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.closing_owner, &mut self.retirement, grant);
        }
        match self.extract_closing_owner() {
            Some(staged) => {
                self.closing_owner = staged;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
            }
            None => {
                self.terminal = true;
                Ok(RetainedCloneStep::Complete(empty))
            }
        }
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.residual_is_empty() && self.closing_owner.is_none() && self.retirement.is_none()
    }
}
