//! 🪜️ Query execution retains its clause and candidate cursors between cancellable work steps.

use super::*;
use std::ops::Bound::{Excluded, Unbounded};

struct PatternExecution {
    patterns: Vec<Pattern>,
    pattern: usize,
    bindings: Vec<Binding>,
    binding: usize,
    next: Vec<Binding>,
    node: Option<String>,
    edge: Option<String>,
    node_active: bool,
}

impl PatternExecution {
    fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns, pattern: 0, bindings: vec![Binding::default()], binding: 0, next: Vec::new(), node: None, edge: None, node_active: false }
    }

    fn step(&mut self, graph: &Graph) -> Result<Option<Vec<Binding>>, String> {
        let Some(pattern) = self.patterns.get(self.pattern) else { return Ok(Some(std::mem::take(&mut self.bindings))) };
        let left = pattern.nodes.first().ok_or_else(|| "empty pattern".to_string())?;
        let Some(base) = self.bindings.get(self.binding) else {
            self.bindings = std::mem::take(&mut self.next);
            self.binding = 0;
            self.pattern += 1;
            return Ok(None);
        };
        if !self.node_active {
            let node = match self.node.as_ref() {
                Some(previous) => graph.nodes.range::<str, _>((Excluded(previous.as_str()), Unbounded)).next(),
                None => graph.nodes.first_key_value(),
            };
            let Some((id, node)) = node else {
                self.binding += 1;
                self.node = None;
                return Ok(None);
            };
            self.node = Some(id.clone());
            if node.kind != left.kind || binding_conflicts(base, &left.var, id) { return Ok(None); }
            self.node_active = true;
        }
        let node_id = self.node.as_ref().expect("active node has an id");
        if let Some(edge_pattern) = &pattern.edge {
            let edge = match self.edge.as_ref() {
                Some(previous) => graph.edges.range::<str, _>((Excluded(previous.as_str()), Unbounded)).next(),
                None => graph.edges.first_key_value(),
            };
            let Some((edge_id, edge)) = edge else {
                self.node_active = false;
                self.edge = None;
                return Ok(None);
            };
            self.edge = Some(edge_id.clone());
            if edge_pattern.kind.as_ref().is_some_and(|kind| *kind != edge.kind) || crate::port_node_id(&edge.source) != Some(node_id.as_str()) { return Ok(None); }
            let Some(target) = crate::port_node_id(&edge.target) else { return Ok(None) };
            if !graph.nodes.get(target).is_some_and(|node| node.kind == edge_pattern.right.kind) || binding_conflicts(base, &edge_pattern.right.var, target) { return Ok(None); }
            let mut binding = base.clone();
            binding.nodes.insert(left.var.clone(), node_id.clone());
            binding.nodes.insert(edge_pattern.right.var.clone(), target.to_string());
            if let Some(var) = &edge_pattern.var { binding.edges.insert(var.clone(), edge_id.clone()); }
            self.next.push(binding);
        } else {
            let mut binding = base.clone();
            binding.nodes.insert(left.var.clone(), node_id.clone());
            self.next.push(binding);
            self.node_active = false;
        }
        Ok(None)
    }
}

/// 🔎️ Owns query evaluation state; each step examines one match candidate, filter row, or mutation item.
pub struct QueryExecution {
    graph: Graph,
    fixture: JackSnapshot,
    query: std::sync::Arc<Query>,
    clause: usize,
    item: usize,
    matching: Option<PatternExecution>,
    filtering: Option<std::vec::IntoIter<Binding>>,
    bindings: Vec<Binding>,
    return_items: Option<Vec<ReturnItem>>,
    operations: Vec<TrinityGraphMutation>,
    finished: bool,
}

impl QueryExecution {
    pub fn new(graph: Graph, query: Query) -> Self {
        let fixture = graph.to_fixture();
        Self { graph, fixture, query: std::sync::Arc::new(query), clause: 0, item: 0, matching: None, filtering: None, bindings: vec![Binding::default()], return_items: None, operations: Vec::new(), finished: false }
    }

    pub fn step(&mut self) -> Result<Option<(QueryResult, Vec<TrinityGraphMutation>)>, String> {
        if self.finished { return Err("query execution already completed".into()); }
        let query = std::sync::Arc::clone(&self.query);
        let Some(clause) = query.clauses.get(self.clause) else {
            self.finished = true;
            let result = self.return_items.as_ref().map(|items| build_return(&self.graph, &self.bindings, items)).unwrap_or_else(|| QueryResult::table(Vec::new(), Vec::new()));
            return Ok(Some((result, std::mem::take(&mut self.operations))));
        };
        match clause {
            Clause::Match(patterns) => {
                let matching = self.matching.get_or_insert_with(|| PatternExecution::new(patterns.clone()));
                if let Some(bindings) = matching.step(&self.graph)? {
                    self.bindings = bindings;
                    self.advance_clause();
                }
            }
            Clause::Where(expr) => {
                if self.filtering.is_none() { self.filtering = Some(std::mem::take(&mut self.bindings).into_iter()); }
                match self.filtering.as_mut().expect("filter cursor is initialized").next() {
                    Some(binding) => if eval_expr(&self.graph, &binding, &expr) { self.bindings.push(binding); },
                    None => self.advance_clause(),
                }
            }
            Clause::Return(items) => {
                self.return_items = Some(items.clone());
                self.advance_clause();
            }
            Clause::Create(pattern) => {
                self.apply(emit_create_operations(&self.fixture, &pattern)?)?;
                self.advance_clause();
            }
            Clause::Delete(vars) => {
                if let Some(var) = vars.get(self.item) {
                    if let Some(id) = self.bindings.first().and_then(|binding| binding.nodes.get(var).cloned()) { self.apply(vec![delete_node(id)])?; }
                    self.item += 1;
                } else { self.advance_clause(); }
            }
            Clause::Set(items) => {
                if let Some(item) = items.get(self.item) {
                    if let Some(id) = self.bindings.first().and_then(|binding| binding.nodes.get(&item.var)) {
                        self.apply(vec![emit_set_operation(&self.fixture, id, &item.prop, item.value.clone())?])?;
                    }
                    self.item += 1;
                } else { self.advance_clause(); }
            }
            Clause::Merge(pattern) => {
                let matching = self.matching.get_or_insert_with(|| PatternExecution::new(vec![pattern.clone()]));
                if let Some(bindings) = matching.step(&self.graph)? {
                    if bindings.is_empty() { self.apply(emit_create_operations(&self.fixture, &pattern)?)?; }
                    self.advance_clause();
                }
            }
        }
        Ok(None)
    }

    fn apply(&mut self, operations: Vec<TrinityGraphMutation>) -> Result<(), String> {
        self.fixture = apply_trinity_graph_mutations(self.fixture.clone(), &operations).map_err(|error| error.to_string())?;
        self.graph = Graph::from_fixture(self.fixture.clone()).map_err(|error| error.to_string())?;
        self.operations.extend(operations);
        Ok(())
    }

    fn advance_clause(&mut self) {
        self.clause += 1;
        self.item = 0;
        self.matching = None;
        self.filtering = None;
    }
}
