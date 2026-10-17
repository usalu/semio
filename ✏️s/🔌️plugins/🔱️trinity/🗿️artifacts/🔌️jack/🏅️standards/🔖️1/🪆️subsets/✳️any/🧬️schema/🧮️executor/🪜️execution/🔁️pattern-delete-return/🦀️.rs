//! 🔁️ Jack query clause executors: the pattern match, delete and return phases of one query execution.

use super::*;

pub(super) struct PatternExecution {
    pub(super) pattern: usize,
    pub(super) bindings: Vec<Binding>,
    pub(super) binding: usize,
    pub(super) next: Vec<Binding>,
    pub(super) node: Option<String>,
    pub(super) edge: Option<String>,
    node_active: bool,
}

impl PatternExecution {
    pub(super) fn new() -> Self {
        Self { pattern: 0, bindings: vec![Binding::default()], binding: 0, next: Vec::new(), node: None, edge: None, node_active: false }
    }

    pub(super) fn step(&mut self, graph: &Graph, patterns: &[Pattern]) -> Result<Option<Vec<Binding>>, ValueError> {
        let Some(pattern) = patterns.get(self.pattern) else { return Ok(Some(std::mem::take(&mut self.bindings))) };
        let left = pattern.nodes.first().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"empty pattern"))?;
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
            if node.kind != left.kind || binding_conflicts(base, &left.var, id) {
                return Ok(None);
            }
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
            if edge_pattern.kind.as_ref().is_some_and(|kind| *kind != edge.kind) || crate::standards::v1::subsets::any::schema::graph::port_node_id(&edge.source) != Some(node_id.as_str()) {
                return Ok(None);
            }
            let Some(target) = crate::standards::v1::subsets::any::schema::graph::port_node_id(&edge.target) else { return Ok(None) };
            if !graph.nodes.get(target).is_some_and(|node| node.kind == edge_pattern.right.kind) || binding_conflicts(base, &edge_pattern.right.var, target) {
                return Ok(None);
            }
            if self.next.len() >= QUERY_ENTITY_MAXIMUM {
                return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query match rows exceed their admission"));
            }
            let mut binding = base.clone();
            binding.nodes.insert(left.var.clone(), node_id.clone());
            binding.nodes.insert(edge_pattern.right.var.clone(), target.to_string());
            if let Some(var) = &edge_pattern.var {
                binding.edges.insert(var.clone(), edge_id.clone());
            }
            self.next.push(binding);
        } else {
            if self.next.len() >= QUERY_ENTITY_MAXIMUM {
                return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query match rows exceed their admission"));
            }
            let mut binding = base.clone();
            binding.nodes.insert(left.var.clone(), node_id.clone());
            self.next.push(binding);
            self.node_active = false;
        }
        Ok(None)
    }
}

pub(super) struct DeleteExecution {
    pub(super) operation: GraphEffect,
    pub(super) id: String,
    pub(super) edge: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReturnPhase {
    Columns,
    Inspect,
    Select,
    Nodes,
    Edges,
    Table,
    Complete,
}

pub(super) struct ReturnExecution{
    phase: ReturnPhase,
    pub(super) columns: Vec<String>,
    pub(super) rows: Vec<Vec<PropertyValue>>,
    pub(super) binding: usize,
    pub(super) item: usize,
    graph_result: bool,
    pub(super) node_ids: BTreeSet<String>,
    pub(super) edge_ids: BTreeSet<String>,
    pub(super) nodes: Vec<Node>,
    pub(super) edges: Vec<Edge>,
    root_selected: bool,
    result_owned_bytes: usize,
    pub(super) maximum_result_owned_bytes: usize,
}

impl ReturnExecution {
    pub(super) fn new(maximum_result_owned_bytes:usize) -> Self {
        Self {
            phase: ReturnPhase::Columns,
            columns: Vec::new(),
            rows: Vec::new(),
            binding: 0,
            item: 0,
            graph_result: false,
            node_ids: BTreeSet::new(),
            edge_ids: BTreeSet::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            root_selected: false,
            result_owned_bytes: size_of::<QueryResult>(),
            maximum_result_owned_bytes,
        }
    }
    fn add_owned(&mut self, bytes: usize) -> Result<(), ValueError> {
        add_result_owned(&mut self.result_owned_bytes, bytes, self.maximum_result_owned_bytes)
    }
    fn advance_pair(&mut self, items: &[ReturnItem]) {
        self.item += 1;
        if self.item >= items.len() {
            self.item = 0;
            self.binding += 1;
        }
    }
    pub(super) fn step(&mut self, graph: &mut Graph, bindings: &[Binding], items: &[ReturnItem]) -> Result<Option<QueryResult>, ValueError> {
        self.add_owned(0)?;
        match self.phase {
            ReturnPhase::Columns => {
                if let Some(item) = items.get(self.item) {
                    let column = match item {
                        ReturnItem::Var(value) => value.clone(),
                        ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
                    };
                    self.add_owned(column.capacity())?;
                    reserve_result(&mut self.columns, &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;
                    self.columns.push(column);
                    self.item += 1;
                } else {
                    self.item = 0;
                    self.phase = ReturnPhase::Inspect;
                }
            }
            ReturnPhase::Inspect => {
                if bindings.is_empty() || items.is_empty() || self.binding >= bindings.len() {
                    self.binding = 0;
                    self.item = 0;
                    if self.graph_result {
                        graph.name.try_reserve_exact(" subgraph".len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "query graph name allocation refused"))?;
                        self.add_owned(graph_result_metadata_owned_bytes(graph, self.maximum_result_owned_bytes)?)?;
                        self.phase = ReturnPhase::Select;
                    } else {
                        self.phase = ReturnPhase::Table;
                    }
                } else {
                    if let ReturnItem::Var(var) = &items[self.item] {
                        self.graph_result |= binding_has_entity(&bindings[self.binding], var);
                    }
                    self.advance_pair(items);
                }
            }
            ReturnPhase::Select => {
                if bindings.is_empty() || items.is_empty() || self.binding >= bindings.len() {
                    self.phase = ReturnPhase::Nodes;
                } else {
                    if let ReturnItem::Var(var) = &items[self.item] {
                        if let Some(id) = bindings[self.binding].nodes.get(var) {
                            self.node_ids.insert(id.clone());
                        }
                        if let Some(id) = bindings[self.binding].edges.get(var) {
                            self.edge_ids.insert(id.clone());
                        }
                    }
                    self.advance_pair(items);
                }
            }
            ReturnPhase::Nodes => {
                if let Some(id) = self.node_ids.pop_first() {
                    if let Some(node) = graph.nodes.get(&id) {
                        let bytes = node_owned_bytes(node, 4_096)?;
                        self.add_owned(bytes - size_of::<Node>())?;
                        reserve_result(&mut self.nodes, &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;
                        self.root_selected |= graph.root_node_id.as_deref() == Some(id.as_str());
                        self.nodes.push(node.clone());
                    }
                } else {
                    self.phase = ReturnPhase::Edges;
                }
            }
            ReturnPhase::Edges => {
                if let Some(id) = self.edge_ids.pop_first() {
                    if let Some(edge) = graph.edges.get(&id) {
                        let bytes = edge_owned_bytes(edge, 4_096)?;
                        self.add_owned(bytes - size_of::<Edge>())?;
                        reserve_result(&mut self.edges, &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;
                        self.edges.push(edge.clone());
                    }
                } else {
                    let mut name = std::mem::take(&mut graph.name);
                    name.push_str(" subgraph");
                    let fixture = crate::standards::v1::subsets::any::schema::ast::JackQueryGraphSnapshot {
                        schema: JackSnapshot::SCHEMA.into(),
                        name,
                        manifest_id: graph.manifest_id.take(),
                        manifest: std::mem::take(&mut graph.manifest),
                        camera: graph.camera.clone(),
                        nodes:std::mem::take(&mut self.nodes),
                        edges:std::mem::take(&mut self.edges),
                        root_node_id: if self.root_selected { graph.root_node_id.take() } else { None },
                        query: String::new(),
                    };
                    self.phase = ReturnPhase::Complete;
                    return Ok(Some(QueryResult::graph(std::mem::take(&mut self.columns), fixture)));
                }
            }
            ReturnPhase::Table => {
                if items.is_empty() || bindings.is_empty() || self.binding >= bindings.len() {
                    self.phase = ReturnPhase::Complete;
                    return Ok(Some(QueryResult::table(std::mem::take(&mut self.columns), std::mem::take(&mut self.rows))));
                } else {
                    if self.rows.len() == self.binding {
                        reserve_result(&mut self.rows, &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;
                        self.rows.push(Vec::new());
                    }
                    {
                        let value = match &items[self.item] {
                            ReturnItem::Var(var) => bindings[self.binding].nodes.get(var).and_then(|id| graph.node(id)).map_or(PropertyValue::Null, |node| PropertyValue::String(node.name.clone())),
                            ReturnItem::Property { var, prop } => binding_value(graph, &bindings[self.binding], var, prop).unwrap_or(PropertyValue::Null),
                        };
                        let mut retained_items = 0;
                        let mut retained_bytes = 0;
                        property_owned_bytes(&value, 0, &mut retained_items, &mut retained_bytes, self.maximum_result_owned_bytes)?;
                        self.add_owned(retained_bytes - size_of::<PropertyValue>())?;
                        reserve_result(&mut self.rows[self.binding], &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;
                        self.rows[self.binding].push(value);
                        self.advance_pair(items);
                    }
                }
            }
            ReturnPhase::Complete => return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query return execution already completed")),
        }
        Ok(None)
    }
}
