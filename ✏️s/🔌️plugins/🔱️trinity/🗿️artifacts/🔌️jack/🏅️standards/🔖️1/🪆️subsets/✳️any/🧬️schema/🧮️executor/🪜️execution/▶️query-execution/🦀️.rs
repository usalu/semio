//! ▶️ Jack query execution: the `QueryExecution` driver that runs the clauses of a prepared query under the step budget.

use super::*;
use semio_framework_value::{record_retirement_work, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll};

/// 🔎️ Owns query evaluation state; each step advances one bounded cursor unit.
pub struct QueryExecution{
    pub(super) graph: Graph,
    pub(super) query: Option<Query>,
    clause: usize,
    item: usize,
    row: usize,
    matching: Option<PatternExecution>,
    filtering: Option<std::vec::IntoIter<Binding>>,
    merge_base: Option<Binding>,
    merge_matches: Option<std::vec::IntoIter<Binding>>,
    pub(super) bindings: Vec<Binding>,
    return_clause: Option<usize>,
    returning: Option<ReturnExecution>,
    pending: VecDeque<GraphEffect>,
    deleting: Option<DeleteExecution>,
    pub(super) operations: Vec<GraphEffect>,
    pub(super) maximum_result_owned_bytes: usize,
    creation_ids: CreationIds,
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
        let creation_ids = CreationIds { node: graph.nodes.len(), edge: graph.edges.len(), ..CreationIds::default() };
        Self {
            graph,
            query: Some(query),
            clause: 0,
            item: 0,
            row: 0,
            matching: None,
            filtering: None,
            merge_base: None,
            merge_matches: None,
            bindings: vec![Binding::default()],
            return_clause: None,
            returning: None,
            pending: VecDeque::new(),
            deleting: None,
            operations: Vec::new(),
            maximum_result_owned_bytes,
            creation_ids,
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
        self.row = 0;
        self.matching = None;
        self.filtering = None;
    }
    fn schedule(&mut self, operations: impl ExactSizeIterator<Item = GraphEffect>) -> Result<(), ValueError> {
        if self.pending.len().saturating_add(operations.len()).saturating_add(self.operations.len()) > QUERY_ENTITY_MAXIMUM {
            return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query mutations exceed their admission"));
        }
        self.pending.extend(operations);
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
                    .map(|(id, edge)| (id.clone(), endpoint_node(&edge.source) == deleting.id || endpoint_node(&edge.target) == deleting.id)),
                None => self.graph.edges.first_key_value().map(|(id, edge)| (id.clone(), endpoint_node(&edge.source) == deleting.id || endpoint_node(&edge.target) == deleting.id)),
            };
            if let Some((edge, incident)) = next {
                deleting.edge = Some(edge.clone());
                if incident {
                    if let Some(removed) = self.graph.edges.remove(&edge) {
                        self.creation_ids.deleted_edges.insert(edge);
                        retire_displaced(GraphEffect::CreateEdge(removed))?;
                    }
                }
                return Ok(true);
            }
            if let Some(removed) = self.graph.nodes.remove(&deleting.id) {
                self.creation_ids.deleted_nodes.insert(deleting.id.clone());
                retire_displaced(GraphEffect::CreateNode(removed))?;
            }
            if self.graph.root_node_id.as_deref() == Some(deleting.id.as_str()) {
                self.graph.root_node_id = None;
            }
            self.operations.push(self.deleting.take().expect("delete cursor remains owned").operation);
            return Ok(true);
        }
        let Some(operation) = self.pending.front() else {
            return Ok(false);
        };
        validate_graph_effect(operation, &self.graph).map_err(|error| invalid(error.to_string()))?;
        let operation = self.pending.pop_front().expect("validated pending effect");
        match &operation {
            GraphEffect::CreateNode(node) => self.graph.add_node(node.clone()),
            GraphEffect::DeleteNode(id) => {
                self.deleting = Some(DeleteExecution { id: id.clone(), edge: None, operation });
                return Ok(true);
            }
            GraphEffect::CreateEdge(edge) => self.graph.add_edge(edge.clone()),
            GraphEffect::DeleteEdge(id) => {
                if let Some(removed) = self.graph.edges.remove(id) {
                    self.creation_ids.deleted_edges.insert(id.clone());
                    retire_displaced(GraphEffect::CreateEdge(removed))?;
                }
            }
            GraphEffect::RenameNode { id, name } => {
                let node = self.graph.node_mut(id).expect("validated node");
                let previous = std::mem::replace(&mut node.name, name.clone());
                retire_displaced(GraphEffect::DeleteNode(previous))?;
            }
            GraphEffect::MoveNode { id, x, y } => {
                let node = self.graph.node_mut(id).expect("validated node");
                node.x = *x;
                node.y = *y;
            }
            GraphEffect::SetProperty { entity, key, .. } | GraphEffect::RemoveProperty { entity, key } => {
                let bag = match entity {
                    EntityRef::Node(id) => self.graph.nodes.get_mut(id).map(|node| &mut node.properties),
                    EntityRef::Edge(id) => self.graph.edges.get_mut(id).map(|edge| &mut edge.properties),
                };
                let bag = bag.expect("validated entity");
                let previous = match &operation {
                    GraphEffect::SetProperty { value, .. } => bag.insert(key.clone(), value.clone()),
                    _ => bag.remove(key),
                };
                if let Some(value) = previous {
                    retire_displaced(GraphEffect::SetProperty { entity: EntityRef::Node(String::new()), key: String::new(), value })?;
                }
            }
        }
        self.operations.push(operation);
        Ok(true)
    }
    /// 📊️ Counts the graph effects already staged or waiting to be staged.
    pub fn effect_count(&self) -> usize {
        self.operations.len() + self.pending.len() + usize::from(self.deleting.is_some())
    }

    pub fn step(&mut self) -> Result<Option<(QueryResult, Vec<GraphEffect>)>, ValueError> {
        if self.finished || self.closing {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query execution is terminal"));
        }
        let result = self.advance();
        self.finished |= result.is_err();
        result
    }

    fn advance(&mut self) -> Result<Option<(QueryResult, Vec<GraphEffect>)>, ValueError> {
        if self.mutation_step()? {
            return Ok(None);
        }
        let query = self.query.take().expect("query AST remains owned");
        let result = self.query_step(&query);
        self.query = Some(query);
        result
    }

    fn merge_step(&mut self, pattern: &Pattern) -> Result<(), ValueError> {
        if let Some(matches) = self.merge_matches.as_mut() {
            if let Some(binding) = matches.next() {
                return self.push_merge_row(binding);
            }
            self.merge_matches = None;
        }
        if let Some(matching) = self.matching.as_mut() {
            let Some(matches) = matching.step(&self.graph, std::slice::from_ref(pattern))? else {
                return Ok(());
            };
            self.matching = None;
            let mut base = self.merge_base.take().expect("active merge retains its input binding");
            if matches.is_empty() {
                let operations = create_operations(&self.graph, pattern, &mut base, &mut self.creation_ids)?;
                self.schedule(operations.into_iter())?;
                return self.push_merge_row(base);
            }
            self.merge_matches = Some(matches.into_iter());
            return Ok(());
        }
        let input = self.filtering.get_or_insert_with(|| std::mem::take(&mut self.bindings).into_iter());
        if let Some(base) = input.next() {
            self.matching = Some(PatternExecution::new(vec![base.clone()]));
            self.merge_base = Some(base);
        } else {
            self.advance_clause();
        }
        Ok(())
    }

    fn push_merge_row(&mut self, binding: Binding) -> Result<(), ValueError> {
        if self.bindings.len() >= QUERY_ENTITY_MAXIMUM {
            return Err(ValueError::new(ValueRefusalKind::WorkLimit, "query merge rows exceed their admission"));
        }
        self.bindings.push(binding);
        Ok(())
    }

    fn mutation_item<'a, T>(&mut self, items: &'a [T]) -> Option<(&'a T, &Binding)> {
        if self.row >= self.bindings.len() {
            self.advance_clause();
            return None;
        }
        let Some(item) = items.get(self.item) else {
            self.row += 1;
            self.item = 0;
            return None;
        };
        self.item += 1;
        Some((item, &self.bindings[self.row]))
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
                let matching = self.matching.get_or_insert_with(|| PatternExecution::new(std::mem::take(&mut self.bindings)));
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
            Clause::Create(pattern) => {
                if let Some(binding) = self.bindings.get_mut(self.row) {
                    let operations = create_operations(&self.graph, pattern, binding, &mut self.creation_ids)?;
                    self.schedule(operations.into_iter())?;
                    self.row += 1;
                } else {
                    self.advance_clause();
                }
            }
            Clause::Delete(vars) => {
                let entity = self.mutation_item(vars).map(|(var, binding)| bound_entity(binding, var)).transpose()?;
                let operation = match entity {
                    Some(EntityRef::Node(id)) if self.graph.nodes.contains_key(&id) => Some(GraphEffect::DeleteNode(id)),
                    Some(EntityRef::Edge(id)) if self.graph.edges.contains_key(&id) => Some(GraphEffect::DeleteEdge(id)),
                    _ => None,
                };
                if let Some(operation) = operation {
                    self.schedule(std::iter::once(operation))?;
                }
            }
            Clause::Set(items) => {
                if let Some((item, entity)) = self.mutation_item(items).map(|(item, binding)| {
                    bound_entity(binding, &item.var).map(|entity| (item, entity))
                }).transpose()? {
                    let operation = emit_set_operation_from_graph(&self.graph, entity, &item.prop, item.value.clone())?;
                    self.schedule(std::iter::once(operation))?;
                }
            }
            Clause::Merge(pattern) => self.merge_step(pattern)?,
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
            && self.merge_base.is_none()
            && self.merge_matches.is_none()
            && self.returning.is_none()
            && self.pending.is_empty()
            && self.deleting.is_none()
            && self.operations.is_empty()
            && self.creation_ids.deleted_edges.is_empty()
            && self.creation_ids.deleted_nodes.is_empty()
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
        if let Some(id) = self.creation_ids.deleted_edges.pop_first().or_else(|| self.creation_ids.deleted_nodes.pop_first()) {
            return effect(GraphEffect::DeleteNode(id));
        }
        if let Some(binding) = self.merge_base.take() {
            self.bindings.push(binding);
            return Some(None);
        }
        if let Some(matches) = self.merge_matches.as_mut() {
            if let Some(binding) = matches.next() {
                self.bindings.push(binding);
            } else {
                self.merge_matches = None;
            }
            return Some(None);
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

    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(empty)), work: 0 });
        }
        let grant = effect.grant();
        if !self.closing || grant.maximum_items == 0 || effect.maximum_work_units() == 0 {
            return RetainedWorkPoll::Pending { work: 0 };
        }
        let demand = match self.close_demands(grant.maximum_copy_bytes).and_then(|demand| {
            admit_demand(grant, demand, "query execution close exceeds admitted depth").map(|admitted| admitted.then_some(demand))
        }) {
            Ok(Some(demand)) => demand,
            Ok(None) => return RetainedWorkPoll::Pending { work: 0 },
            Err(error) => return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Err(error.into()), work: 0 }),
        };
        if self.retirement.is_some() {
            return match store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant.maximum_copy_bytes, effect) {
                RetainedWorkPoll::Complete(turn) => RetainedWorkPoll::Complete(RetainedWorkOutcome {
                    result: turn.result.map(|step| RetainedCloneStep::Progress(step.progress())),
                    work: turn.work,
                }),
                pending => pending,
            };
        }
        if self.closing_owner.is_some() {
            let turn = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.closing_owner, &mut self.retirement, effect);
            return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: turn.result.map_err(Into::into), work: turn.work });
        }
        let work = match record_retirement_work(effect, 1) {
            Ok(work) => work,
            Err((error, work)) => return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Err(error.into()), work }),
        };
        let step = match self.extract_closing_owner() {
            Some(staged) => {
                self.closing_owner = staged;
                RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })
            }
            None => {
                self.terminal = true;
                RetainedCloneStep::Complete(empty)
            }
        };
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Ok(step), work })
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.residual_is_empty() && self.closing_owner.is_none() && self.retirement.is_none()
    }
}
