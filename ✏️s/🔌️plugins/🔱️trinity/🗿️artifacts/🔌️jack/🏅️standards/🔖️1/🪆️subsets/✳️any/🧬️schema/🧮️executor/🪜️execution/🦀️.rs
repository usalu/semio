//! 🪜️ Query execution retains one explicit preparation, evaluation, result, or retirement cursor.

use super::*;
use crate::standards::v1::subsets::any::schema::wire_runtime::{JackMutationRetirementFactory, JackSnapshotCloneAuthority, JackSnapshotCloneStep, JackSnapshotRetirementFactory};
use std::collections::{BTreeSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::ops::Bound::{Excluded, Unbounded};

const QUERY_ENTITY_MAXIMUM: usize = 16_384;
const QUERY_OUTPUT_MAXIMUM_BYTES: usize = 1_048_576;
const QUERY_ENTITY_COLLECTION_MAXIMUM: usize = 128;
const QUERY_ENTITY_NESTING_MAXIMUM: usize = 16;

fn add_owned_bytes(bytes: &mut usize, additional: usize, maximum: usize) -> Result<(), String> {
    *bytes = bytes.checked_add(additional).ok_or_else(|| "query entity byte count overflow".to_string())?;
    if *bytes > maximum {
        return Err("query entity exceeds its byte grant".into());
    }
    Ok(())
}

fn property_owned_bytes(value: &PropertyValue, depth: usize, items: &mut usize, bytes: &mut usize, maximum: usize) -> Result<(), String> {
    if depth > QUERY_ENTITY_NESTING_MAXIMUM {
        return Err("query entity exceeds its nesting admission".into());
    }
    *items = items.checked_add(1).ok_or_else(|| "query entity item count overflow".to_string())?;
    if *items > QUERY_ENTITY_COLLECTION_MAXIMUM {
        return Err("query entity exceeds its collection admission".into());
    }
    add_owned_bytes(bytes, size_of::<PropertyValue>(), maximum)?;
    match value {
        PropertyValue::String(value) => add_owned_bytes(bytes, value.len(), maximum),
        PropertyValue::Array(values) => {
            for value in values {
                property_owned_bytes(value, depth + 1, items, bytes, maximum)?;
            }
            Ok(())
        }
        PropertyValue::Object(values) => {
            for (key, value) in values {
                add_owned_bytes(bytes, key.len(), maximum)?;
                property_owned_bytes(value, depth + 1, items, bytes, maximum)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn bag_owned_bytes(values: &PropertyBag, items: &mut usize, bytes: &mut usize, maximum: usize) -> Result<(), String> {
    for (key, value) in values {
        add_owned_bytes(bytes, key.len(), maximum)?;
        property_owned_bytes(value, 0, items, bytes, maximum)?;
    }
    Ok(())
}

fn node_owned_bytes(node: &Node, maximum: usize) -> Result<usize, String> {
    let mut bytes = size_of::<Node>();
    let mut items = node.ports.len();
    if items > QUERY_ENTITY_COLLECTION_MAXIMUM {
        return Err("query node exceeds its port admission".into());
    }
    for value in [&node.id, &node.id, &node.kind, &node.name] {
        add_owned_bytes(&mut bytes, value.len(), maximum)?;
    }
    bag_owned_bytes(&node.properties, &mut items, &mut bytes, maximum)?;
    for port in &node.ports {
        add_owned_bytes(&mut bytes, size_of::<Port>(), maximum)?;
        add_owned_bytes(&mut bytes, port.id.len(), maximum)?;
        add_owned_bytes(&mut bytes, port.kind.len(), maximum)?;
        bag_owned_bytes(&port.properties, &mut items, &mut bytes, maximum)?;
    }
    Ok(bytes)
}

fn edge_owned_bytes(edge: &Edge, maximum: usize) -> Result<usize, String> {
    let mut bytes = size_of::<Edge>();
    let mut items = 0;
    for value in [&edge.id, &edge.id, &edge.kind, &edge.source, &edge.target] {
        add_owned_bytes(&mut bytes, value.len(), maximum)?;
    }
    bag_owned_bytes(&edge.properties, &mut items, &mut bytes, maximum)?;
    Ok(bytes)
}

fn json_string_bytes(value: &str) -> Result<usize, String> {
    let mut bytes = 2usize;
    for byte in value.bytes() {
        let escaped = match byte {
            b'"' | b'\\' | b'\x08' | b'\x0c' | b'\n' | b'\r' | b'\t' => 2,
            0..=0x1f => 6,
            _ => 1,
        };
        bytes = bytes.checked_add(escaped).ok_or_else(|| "query JSON string bound overflow".to_string())?;
    }
    Ok(bytes)
}

fn property_json_upper_bound(value: &PropertyValue, depth: usize, items: &mut usize) -> Result<usize, String> {
    if depth > QUERY_ENTITY_NESTING_MAXIMUM {
        return Err("query result exceeds its nesting admission".into());
    }
    *items = items.checked_add(1).ok_or_else(|| "query result item count overflow".to_string())?;
    if *items > QUERY_ENTITY_COLLECTION_MAXIMUM {
        return Err("query result exceeds its collection admission".into());
    }
    match value {
        PropertyValue::Null => Ok(4),
        PropertyValue::Bool(_) => Ok(5),
        PropertyValue::Number(_) => Ok(32),
        PropertyValue::String(value) => json_string_bytes(value),
        PropertyValue::Array(values) => {
            let mut bytes = 2usize;
            for value in values {
                bytes = bytes.checked_add(property_json_upper_bound(value, depth + 1, items)?).and_then(|bytes| bytes.checked_add(1)).ok_or_else(|| "query result byte count overflow".to_string())?;
            }
            Ok(bytes)
        }
        PropertyValue::Object(values) => {
            let mut bytes = 2usize;
            for (key, value) in values {
                let value_bytes = property_json_upper_bound(value, depth + 1, items)?;
                bytes = bytes.checked_add(json_string_bytes(key)?).and_then(|bytes| bytes.checked_add(value_bytes)).and_then(|bytes| bytes.checked_add(2)).ok_or_else(|| "query result byte count overflow".to_string())?;
            }
            Ok(bytes)
        }
    }
}

fn hash_property(value: &PropertyValue, hasher: &mut impl Hasher) {
    std::mem::discriminant(value).hash(hasher);
    match value {
        PropertyValue::Bool(value) => value.hash(hasher),
        PropertyValue::Number(value) => value.to_bits().hash(hasher),
        PropertyValue::String(value) => value.hash(hasher),
        PropertyValue::Array(values) => {
            for value in values {
                hash_property(value, hasher);
            }
        }
        PropertyValue::Object(values) => {
            for (key, value) in values {
                key.hash(hasher);
                hash_property(value, hasher);
            }
        }
        PropertyValue::Null => {}
    }
}

fn hash_node(node: &Node, hasher: &mut impl Hasher) {
    node.id.hash(hasher);
    node.kind.hash(hasher);
    node.name.hash(hasher);
    node.x.to_bits().hash(hasher);
    node.y.to_bits().hash(hasher);
    node.width.to_bits().hash(hasher);
    node.height.to_bits().hash(hasher);
    for (key, value) in &node.properties {
        key.hash(hasher);
        hash_property(value, hasher);
    }
    for port in &node.ports {
        port.id.hash(hasher);
        port.kind.hash(hasher);
        std::mem::discriminant(&port.direction).hash(hasher);
        for (key, value) in &port.properties {
            key.hash(hasher);
            hash_property(value, hasher);
        }
    }
}

fn hash_edge(edge: &Edge, hasher: &mut impl Hasher) {
    edge.id.hash(hasher);
    edge.kind.hash(hasher);
    edge.source.hash(hasher);
    edge.target.hash(hasher);
    for (key, value) in &edge.properties {
        key.hash(hasher);
        hash_property(value, hasher);
    }
}

/// 🧱 One preparation turn clones one snapshot metadata field or one working-scene entity.
pub enum QueryPreparationStep {
    Pending,
    Complete(QueryExecution),
}

/// 🧱 Incremental snapshot-to-query-workspace preparation that never clones the complete fixture.
pub struct QueryExecutionPreparation {
    query: Option<Query>,
    metadata: JackSnapshotCloneAuthority,
    metadata_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    graph: Option<Graph>,
    node: usize,
    edge: usize,
    metadata_output_upper_bound: usize,
    closing: bool,
    terminal: bool,
}

impl QueryExecutionPreparation {
    pub fn new(query: Query) -> Self {
        Self { query: Some(query), metadata: JackSnapshotCloneAuthority::metadata_only(), metadata_retirement: None, graph: None, node: 0, edge: 0, metadata_output_upper_bound: 512, closing: false, terminal: false }
    }

    pub fn step(&mut self, snapshot: &JackSnapshot, maximum_bytes: usize) -> Result<QueryPreparationStep, String> {
        if self.closing || self.terminal {
            return Err("query preparation is terminal".into());
        }
        if self.graph.is_none() {
            match self.metadata.advance(snapshot, maximum_bytes)? {
                JackSnapshotCloneStep::Pending { copied_bytes } => {
                    self.metadata_output_upper_bound = self.metadata_output_upper_bound.saturating_add(copied_bytes.saturating_mul(6)).saturating_add(128);
                    return Ok(QueryPreparationStep::Pending);
                }
                JackSnapshotCloneStep::Complete => {}
            }
            let mut metadata = self.metadata.take_value().ok_or_else(|| "query metadata clone did not transfer its owner".to_string())?;
            if metadata.schema != JackSnapshot::SCHEMA {
                let error = format!("query snapshot schema '{}' is invalid", metadata.schema);
                self.metadata_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, metadata));
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
            };
            self.metadata_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, metadata));
            self.graph = Some(graph);
            return Ok(QueryPreparationStep::Pending);
        }
        if let Some(retirement) = self.metadata_retirement.as_mut() {
            match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    self.metadata_retirement = None;
                    return Ok(QueryPreparationStep::Pending);
                }
                store::SnapshotRetirementStep::Complete => return Err("query metadata retirement reported a false terminal".into()),
                _ => return Ok(QueryPreparationStep::Pending),
            }
        }
        let scene = snapshot.content.local_owner::<crate::JackWorkingScene>().ok_or_else(|| "query snapshot content is not materialized".to_string())?;
        if scene.nodes.len() > QUERY_ENTITY_MAXIMUM || scene.edges.len() > QUERY_ENTITY_MAXIMUM {
            return Err("query snapshot exceeds its entity admission".into());
        }
        if let Some(node) = scene.nodes.get(self.node) {
            node_owned_bytes(node, maximum_bytes)?;
            let graph = self.graph.as_mut().expect("query preparation graph is initialized");
            if graph.nodes.contains_key(&node.id) {
                return Err(format!("duplicate node id {}", node.id));
            }
            graph.nodes.insert(node.id.clone(), node.clone());
            self.node += 1;
            return Ok(QueryPreparationStep::Pending);
        }
        if let Some(edge) = scene.edges.get(self.edge) {
            edge_owned_bytes(edge, maximum_bytes)?;
            let graph = self.graph.as_mut().expect("query preparation graph is initialized");
            if graph.edges.contains_key(&edge.id) {
                return Err(format!("duplicate edge id {}", edge.id));
            }
            graph.edges.insert(edge.id.clone(), edge.clone());
            self.edge += 1;
            return Ok(QueryPreparationStep::Pending);
        }
        let graph = self.graph.take().expect("query preparation graph remains owned");
        let query = self.query.take().expect("query preparation AST remains owned");
        self.terminal = true;
        Ok(QueryPreparationStep::Complete(QueryExecution::with_metadata_output_upper_bound(graph, query, self.metadata_output_upper_bound)))
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(retirement) = self.metadata_retirement.as_mut() {
            match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => self.metadata_retirement = None,
                store::SnapshotRetirementStep::Complete => return Err("query preparation metadata retirement reported a false terminal".into()),
                step => return Ok(step),
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.metadata.terminal_is_empty() {
            return match self.metadata.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if self.metadata.terminal_is_empty() => Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }),
                store::SnapshotRetirementStep::Complete => Err("query preparation metadata clone reported a false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(graph) = self.graph.as_mut() {
            if let Some((_, node)) = graph.nodes.pop_first() {
                self.metadata_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, create_node(node)));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some((_, edge)) = graph.edges.pop_first() {
                self.metadata_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, create_edge(edge)));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.graph = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.query.is_some() {
            drop(self.query.take());
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: maximum_bytes });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.query.is_none() && self.graph.is_none() && self.metadata_retirement.is_none() && self.metadata.terminal_is_empty()
    }
}

struct PatternExecution {
    pattern: usize,
    bindings: Vec<Binding>,
    binding: usize,
    next: Vec<Binding>,
    node: Option<String>,
    edge: Option<String>,
    node_active: bool,
}

impl PatternExecution {
    fn new() -> Self {
        Self { pattern: 0, bindings: vec![Binding::default()], binding: 0, next: Vec::new(), node: None, edge: None, node_active: false }
    }

    fn step(&mut self, graph: &Graph, patterns: &[Pattern]) -> Result<Option<Vec<Binding>>, String> {
        let Some(pattern) = patterns.get(self.pattern) else { return Ok(Some(std::mem::take(&mut self.bindings))) };
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
            if edge_pattern.kind.as_ref().is_some_and(|kind| *kind != edge.kind) || crate::port_node_id(&edge.source) != Some(node_id.as_str()) {
                return Ok(None);
            }
            let Some(target) = crate::port_node_id(&edge.target) else { return Ok(None) };
            if !graph.nodes.get(target).is_some_and(|node| node.kind == edge_pattern.right.kind) || binding_conflicts(base, &edge_pattern.right.var, target) {
                return Ok(None);
            }
            if self.next.len() >= QUERY_ENTITY_MAXIMUM {
                return Err("query match rows exceed their admission".into());
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
                return Err("query match rows exceed their admission".into());
            }
            let mut binding = base.clone();
            binding.nodes.insert(left.var.clone(), node_id.clone());
            self.next.push(binding);
            self.node_active = false;
        }
        Ok(None)
    }
}

struct DeleteExecution {
    operation: TrinityGraphMutation,
    id: String,
    edge: Option<String>,
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

struct ReturnExecution {
    phase: ReturnPhase,
    columns: Vec<String>,
    rows: Vec<Vec<PropertyValue>>,
    binding: usize,
    item: usize,
    graph_result: bool,
    node_ids: BTreeSet<String>,
    edge_ids: BTreeSet<String>,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    root_selected: bool,
    output_bytes: usize,
    metadata_output_upper_bound: usize,
    hasher: std::collections::hash_map::DefaultHasher,
}

impl ReturnExecution {
    fn new(metadata_output_upper_bound: usize) -> Self {
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
            output_bytes: 256,
            metadata_output_upper_bound,
            hasher: Default::default(),
        }
    }
    fn add_output(&mut self, bytes: usize) -> Result<(), String> {
        self.output_bytes = self.output_bytes.checked_add(bytes).ok_or_else(|| "query result byte count overflow".to_string())?;
        if self.output_bytes > QUERY_OUTPUT_MAXIMUM_BYTES {
            return Err("query result exceeds its output admission".into());
        }
        Ok(())
    }
    fn advance_pair(&mut self, items: &[ReturnItem]) {
        self.item += 1;
        if self.item >= items.len() {
            self.item = 0;
            self.binding += 1;
        }
    }
    fn step(&mut self, graph: &mut Graph, bindings: &[Binding], items: &[ReturnItem]) -> Result<Option<QueryResult>, String> {
        match self.phase {
            ReturnPhase::Columns => {
                if let Some(item) = items.get(self.item) {
                    let column = match item {
                        ReturnItem::Var(value) => value.clone(),
                        ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
                    };
                    self.add_output(json_string_bytes(&column)?.saturating_add(1))?;
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
                        self.add_output(self.metadata_output_upper_bound)?;
                        self.phase = ReturnPhase::Select;
                    } else {
                        self.add_output(2)?;
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
                        self.add_output(bytes.saturating_mul(6).saturating_add(512))?;
                        hash_node(node, &mut self.hasher);
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
                        self.add_output(bytes.saturating_mul(6).saturating_add(384))?;
                        hash_edge(edge, &mut self.hasher);
                        self.edges.push(edge.clone());
                    }
                } else {
                    let mut content = crate::jack_content_child_handle(&[], &[]);
                    content.child_id = format!("jack-query-result-{:016x}", self.hasher.finish());
                    content.set_local_owner(std::sync::Arc::new(crate::JackWorkingScene { nodes: std::mem::take(&mut self.nodes), edges: std::mem::take(&mut self.edges) }));
                    let mut name = std::mem::take(&mut graph.name);
                    name.push_str(" subgraph");
                    let fixture = JackSnapshot {
                        schema: JackSnapshot::SCHEMA.into(),
                        name,
                        manifest_id: graph.manifest_id.take(),
                        manifest: std::mem::take(&mut graph.manifest),
                        camera: graph.camera.clone(),
                        content,
                        root_node_id: if self.root_selected { graph.root_node_id.take() } else { None },
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
                        self.add_output(3)?;
                        self.rows.push(Vec::new());
                    }
                    {
                        let value = match &items[self.item] {
                            ReturnItem::Var(var) => bindings[self.binding].nodes.get(var).and_then(|id| graph.node(id)).map_or(PropertyValue::Null, |node| PropertyValue::String(node.name.clone())),
                            ReturnItem::Property { var, prop } => binding_value(graph, &bindings[self.binding], var, prop).unwrap_or(PropertyValue::Null),
                        };
                        let mut output_items = 0;
                        self.add_output(property_json_upper_bound(&value, 0, &mut output_items)?.saturating_add(1))?;
                        self.rows[self.binding].push(value);
                        self.advance_pair(items);
                    }
                }
            }
            ReturnPhase::Complete => return Err("query return execution already completed".into()),
        }
        Ok(None)
    }
}

/// 🔎️ Owns query evaluation state; each step advances one bounded cursor unit.
pub struct QueryExecution {
    graph: Graph,
    query: Option<Query>,
    clause: usize,
    item: usize,
    matching: Option<PatternExecution>,
    filtering: Option<std::vec::IntoIter<Binding>>,
    bindings: Vec<Binding>,
    return_clause: Option<usize>,
    returning: Option<ReturnExecution>,
    pending: VecDeque<TrinityGraphMutation>,
    deleting: Option<DeleteExecution>,
    operations: Vec<TrinityGraphMutation>,
    metadata_output_upper_bound: usize,
    pending_clause_advance: bool,
    finished: bool,
    closing: bool,
    retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    metadata_retired: bool,
    terminal: bool,
}

impl QueryExecution {
    pub fn new(graph: Graph, query: Query) -> Self {
        Self::with_metadata_output_upper_bound(graph, query, 512)
    }
    fn with_metadata_output_upper_bound(graph: Graph, query: Query, metadata_output_upper_bound: usize) -> Self {
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
            metadata_output_upper_bound,
            pending_clause_advance: false,
            finished: false,
            closing: false,
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
    fn schedule(&mut self, operations: Vec<TrinityGraphMutation>) -> Result<(), String> {
        if self.pending.len().saturating_add(operations.len()).saturating_add(self.operations.len()) > QUERY_ENTITY_MAXIMUM {
            return Err("query mutations exceed their admission".into());
        }
        self.pending.extend(operations);
        self.pending_clause_advance = true;
        Ok(())
    }
    fn mutation_step(&mut self) -> Result<bool, String> {
        if let Some(retirement) = self.retirement.as_mut() {
            match retirement.close_step(1, 4_096)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => self.retirement = None,
                store::SnapshotRetirementStep::Complete => return Err("query mutation retirement reported a false terminal".into()),
                store::SnapshotRetirementStep::Pending { .. } => return Ok(true),
                store::SnapshotRetirementStep::Blocked => return Err("query mutation retirement unexpectedly blocked".into()),
            }
            return Ok(true);
        }
        if let Some(deleting) = self.deleting.as_mut() {
            let next = match deleting.edge.as_ref() {
                Some(previous) => self
                    .graph
                    .edges
                    .range::<str, _>((Excluded(previous.as_str()), Unbounded))
                    .next()
                    .map(|(id, edge)| (id.clone(), crate::port_node_id(&edge.source) == Some(deleting.id.as_str()) || crate::port_node_id(&edge.target) == Some(deleting.id.as_str()))),
                None => self.graph.edges.first_key_value().map(|(id, edge)| (id.clone(), crate::port_node_id(&edge.source) == Some(deleting.id.as_str()) || crate::port_node_id(&edge.target) == Some(deleting.id.as_str()))),
            };
            if let Some((edge, incident)) = next {
                deleting.edge = Some(edge.clone());
                if incident {
                    if let Some(removed) = self.graph.edges.remove(&edge) {
                        self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, create_edge(removed)));
                    }
                }
                return Ok(true);
            }
            if let Some(removed) = self.graph.nodes.remove(&deleting.id) {
                self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, create_node(removed)));
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
            TrinityGraphMutation::CreateNode(value) => {
                if self.graph.nodes.contains_key(&value.node.id) {
                    Err(format!("node {} already exists", value.node.id))
                } else {
                    self.graph.nodes.insert(value.node.id.clone(), value.node.clone());
                    Ok(())
                }
            }
            TrinityGraphMutation::DeleteNode(value) => {
                self.deleting = Some(DeleteExecution { id: value.id.clone(), edge: None, operation });
                return Ok(true);
            }
            TrinityGraphMutation::CreateEdge(value) => {
                if self.graph.edges.contains_key(&value.edge.id) {
                    Err(format!("edge {} already exists", value.edge.id))
                } else {
                    self.graph.edges.insert(value.edge.id.clone(), value.edge.clone());
                    Ok(())
                }
            }
            TrinityGraphMutation::DeleteEdge(value) => {
                if let Some(removed) = self.graph.edges.remove(&value.id) {
                    self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, create_edge(removed)));
                }
                Ok(())
            }
            TrinityGraphMutation::RenameNode(value) => match self.graph.nodes.get_mut(&value.id) {
                Some(node) => {
                    let previous = std::mem::replace(&mut node.name, value.new_name.clone());
                    self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, delete_node(previous)));
                    Ok(())
                }
                None => Err(format!("node {} not found", value.id)),
            },
            TrinityGraphMutation::MoveNode(value) => match self.graph.nodes.get_mut(&value.id) {
                Some(node) => {
                    node.x = value.x;
                    node.y = value.y;
                    Ok(())
                }
                None => Err(format!("node {} not found", value.id)),
            },
            TrinityGraphMutation::ChangeDataProperty(value) => match &value.entity {
                EntityRef::Node(id) => match self.graph.nodes.get_mut(id) {
                    Some(node) => {
                        let previous = node.properties.insert(value.key.clone(), value.new_value.clone());
                        if let Some(previous) = previous {
                            self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, change_data_property(EntityRef::Node(String::new()), String::new(), previous)));
                        }
                        Ok(())
                    }
                    None => Err(format!("node {id} not found")),
                },
                EntityRef::Edge(id) => match self.graph.edges.get_mut(id) {
                    Some(edge) => {
                        let previous = edge.properties.insert(value.key.clone(), value.new_value.clone());
                        if let Some(previous) = previous {
                            self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, change_data_property(EntityRef::Node(String::new()), String::new(), previous)));
                        }
                        Ok(())
                    }
                    None => Err(format!("edge {id} not found")),
                },
            },
            TrinityGraphMutation::RemoveDataProperty(value) => match &value.entity {
                EntityRef::Node(id) => match self.graph.nodes.get_mut(id) {
                    Some(node) => {
                        let previous = node.properties.remove(&value.key);
                        if let Some(previous) = previous {
                            self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, change_data_property(EntityRef::Node(String::new()), String::new(), previous)));
                        }
                        Ok(())
                    }
                    None => Err(format!("node {id} not found")),
                },
                EntityRef::Edge(id) => match self.graph.edges.get_mut(id) {
                    Some(edge) => {
                        let previous = edge.properties.remove(&value.key);
                        if let Some(previous) = previous {
                            self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, change_data_property(EntityRef::Node(String::new()), String::new(), previous)));
                        }
                        Ok(())
                    }
                    None => Err(format!("edge {id} not found")),
                },
            },
        };
        if let Err(error) = applied {
            self.pending.push_front(operation);
            return Err(error);
        }
        self.operations.push(operation);
        Ok(true)
    }
    pub fn step(&mut self) -> Result<Option<(QueryResult, Vec<TrinityGraphMutation>)>, String> {
        if self.finished || self.closing {
            return Err("query execution is terminal".into());
        }
        if self.mutation_step()? {
            return Ok(None);
        }
        let query = self.query.take().expect("query AST remains owned");
        let result = self.query_step(&query);
        self.query = Some(query);
        result
    }
    fn query_step(&mut self, query: &Query) -> Result<Option<(QueryResult, Vec<TrinityGraphMutation>)>, String> {
        let Some(clause) = query.clauses.get(self.clause) else {
            if self.return_clause.is_none() {
                self.finished = true;
                return Ok(Some((QueryResult::table(Vec::new(), Vec::new()), std::mem::take(&mut self.operations))));
            }
            let metadata_output_upper_bound = self.metadata_output_upper_bound;
            let returning = self.returning.get_or_insert_with(|| ReturnExecution::new(metadata_output_upper_bound));
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
                        self.pending.push_back(delete_node(id));
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
    fn retire_mutation(&mut self, mutation: TrinityGraphMutation) {
        self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, mutation));
    }
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => self.retirement = None,
                store::SnapshotRetirementStep::Complete => return Err("query owner retirement reported a false terminal".into()),
                step => return Ok(step),
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(deleting) = self.deleting.as_mut() {
            if let Some(value) = deleting.edge.take() {
                self.retire_mutation(delete_node(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !deleting.id.is_empty() {
                let value = std::mem::take(&mut deleting.id);
                self.retire_mutation(delete_node(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        if let Some(deleting) = self.deleting.take() {
            self.retire_mutation(deleting.operation);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(mutation) = self.pending.pop_front().or_else(|| self.operations.pop()) {
            self.retire_mutation(mutation);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(filtering) = self.filtering.as_mut() {
            if let Some(binding) = filtering.next() {
                self.bindings.push(binding);
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.filtering = None;
        }
        if let Some(matching) = self.matching.as_mut() {
            if let Some(binding) = matching.bindings.pop().or_else(|| matching.next.pop()) {
                self.bindings.push(binding);
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(value) = matching.node.take().or_else(|| matching.edge.take()) {
                self.retire_mutation(delete_node(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.matching = None;
        }
        if let Some(returning) = self.returning.as_mut() {
            if let Some(node) = returning.nodes.pop() {
                self.retire_mutation(create_node(node));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(edge) = returning.edges.pop() {
                self.retire_mutation(create_edge(edge));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(value) = returning.rows.last_mut().and_then(Vec::pop) {
                self.retire_mutation(change_data_property(EntityRef::Node(String::new()), String::new(), value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if returning.rows.pop().is_some() {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(value) = returning.columns.pop().or_else(|| returning.node_ids.pop_first()).or_else(|| returning.edge_ids.pop_first()) {
                self.retire_mutation(delete_node(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.returning = None;
        }
        if let Some(binding) = self.bindings.last_mut() {
            if let Some((key, value)) = binding.nodes.pop_first().or_else(|| binding.edges.pop_first()) {
                self.retire_mutation(crate::standards::v1::subsets::any::schema::mutations::remove_data_property(EntityRef::Node(value), key));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.bindings.pop();
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some((_, node)) = self.graph.nodes.pop_first() {
            self.retire_mutation(create_node(node));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some((_, edge)) = self.graph.edges.pop_first() {
            self.retire_mutation(create_edge(edge));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.metadata_retired {
            let mut metadata = JackSnapshot::default();
            metadata.name = std::mem::take(&mut self.graph.name);
            metadata.manifest_id = self.graph.manifest_id.take();
            metadata.manifest = std::mem::take(&mut self.graph.manifest);
            metadata.camera = self.graph.camera.clone();
            metadata.root_node_id = self.graph.root_node_id.take();
            self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, metadata));
            self.metadata_retired = true;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.query.is_some() {
            drop(self.query.take());
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: maximum_bytes });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.terminal
            && self.query.is_none()
            && self.graph.nodes.is_empty()
            && self.graph.edges.is_empty()
            && self.bindings.is_empty()
            && self.matching.is_none()
            && self.filtering.is_none()
            && self.returning.is_none()
            && self.pending.is_empty()
            && self.deleting.is_none()
            && self.operations.is_empty()
            && self.retirement.is_none()
    }
}

fn emit_set_operation_from_graph(graph: &Graph, node_id: &str, prop: &str, value: PropertyValue) -> Result<TrinityGraphMutation, String> {
    let node = graph.nodes.get(node_id).ok_or_else(|| format!("node {node_id} not found"))?;
    match prop {
        "name" => match value {
            PropertyValue::String(name) => Ok(rename_node(node_id.to_string(), name)),
            _ => Err(format!("node {node_id}.name expects string value")),
        },
        "x" => Ok(move_node(node_id.to_string(), value.as_f64().ok_or_else(|| format!("node {node_id}.x expects number value"))?, node.y)),
        "y" => Ok(move_node(node_id.to_string(), node.x, value.as_f64().ok_or_else(|| format!("node {node_id}.y expects number value"))?)),
        _ => Ok(change_data_property(EntityRef::Node(node_id.to_string()), prop.to_string(), value)),
    }
}

fn emit_create_operations_from_graph(graph: &Graph, pattern: &Pattern) -> Result<Vec<TrinityGraphMutation>, String> {
    let left = pattern.nodes.first().ok_or_else(|| "empty create pattern".to_string())?;
    let left_id = format!("{}-{}", left.var, graph.nodes.len());
    let mut operations = Vec::with_capacity(if pattern.edge.is_some() { 3 } else { 1 });
    let mut left_ports = Vec::new();
    if pattern.edge.is_some() {
        left_ports.push(Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
    }
    operations.push(create_node(Node { id: left_id.clone(), kind: left.kind.clone(), name: left.var.clone(), x: graph.nodes.len() as f64 * 120.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: left_ports }));
    if let Some(edge_pattern) = &pattern.edge {
        let right_id = format!("{}-{}", edge_pattern.right.var, graph.nodes.len() + 1);
        operations.push(create_node(Node {
            id: right_id.clone(),
            kind: edge_pattern.right.kind.clone(),
            name: edge_pattern.right.var.clone(),
            x: (graph.nodes.len() + 1) as f64 * 120.0,
            y: 80.0,
            width: 80.0,
            height: 40.0,
            properties: PropertyBag::new(),
            ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
        }));
        operations.push(create_edge(Edge {
            id: format!("e-{}", graph.edges.len()),
            kind: edge_pattern.kind.clone().unwrap_or_else(|| "Connection".into()),
            source: port_key(&left_id, "out"),
            target: port_key(&right_id, "in"),
            properties: PropertyBag::new(),
        }));
    }
    Ok(operations)
}
