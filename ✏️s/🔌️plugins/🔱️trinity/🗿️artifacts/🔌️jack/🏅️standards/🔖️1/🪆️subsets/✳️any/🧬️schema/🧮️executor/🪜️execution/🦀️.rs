//! 🪜️ Query execution retains one explicit preparation, evaluation, result, or retirement cursor.

use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::host::{JackEffectRetirementFactory, JackSnapshotCloneAuthority, JackSnapshotCloneStep, JackSnapshotRetirementFactory};
use crate::{port_key, JackSnapshot, Port, PortDirection, PropertyBag};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use std::collections::{BTreeSet, VecDeque};
use std::ops::Bound::{Excluded, Unbounded};

const QUERY_ENTITY_MAXIMUM: usize = 16_384;
pub const QUERY_RESULT_MAXIMUM_OWNED_BYTES: usize = 1_048_576;
const QUERY_ENTITY_COLLECTION_MAXIMUM: usize = 128;
const QUERY_ENTITY_NESTING_MAXIMUM: usize = 16;

fn add_owned_bytes(bytes: &mut usize, additional: usize, maximum: usize) -> Result<(), ValueError> {
    *bytes = bytes.checked_add(additional).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit,"query entity byte count overflow"))?;
    if *bytes > maximum {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"query entity exceeds its byte grant"));
    }
    Ok(())
}

fn property_owned_bytes(value: &PropertyValue, depth: usize, items: &mut usize, bytes: &mut usize, maximum: usize) -> Result<(), ValueError> {
    if depth > QUERY_ENTITY_NESTING_MAXIMUM {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query entity exceeds its nesting admission"));
    }
    *items = items.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit,"query entity item count overflow"))?;
    if *items > QUERY_ENTITY_COLLECTION_MAXIMUM {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query entity exceeds its collection admission"));
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

fn bag_owned_bytes(values: &PropertyBag, items: &mut usize, bytes: &mut usize, maximum: usize) -> Result<(), ValueError> {
    for (key, value) in values {
        add_owned_bytes(bytes, key.len(), maximum)?;
        property_owned_bytes(value, 0, items, bytes, maximum)?;
    }
    Ok(())
}

fn node_owned_bytes(node: &Node, maximum: usize) -> Result<usize, ValueError> {
    let mut bytes = size_of::<Node>();
    let mut items = node.ports.len();
    if items > QUERY_ENTITY_COLLECTION_MAXIMUM {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query node exceeds its port admission"));
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

fn edge_owned_bytes(edge: &Edge, maximum: usize) -> Result<usize, ValueError> {
    let mut bytes = size_of::<Edge>();
    let mut items = 0;
    for value in [&edge.id, &edge.id, &edge.kind, &edge.source, &edge.target] {
        add_owned_bytes(&mut bytes, value.len(), maximum)?;
    }
    bag_owned_bytes(&edge.properties, &mut items, &mut bytes, maximum)?;
    Ok(bytes)
}

fn content_property_bytes(value:&semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue,depth:usize,items:&mut usize,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    if depth>QUERY_ENTITY_NESTING_MAXIMUM{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query entity exceeds its nesting admission"))}
    *items=items.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"query entity item count overflow"))?;
    if *items>QUERY_ENTITY_COLLECTION_MAXIMUM{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query entity exceeds its collection admission"))}
    add_owned_bytes(bytes,size_of::<PropertyValue>(),maximum)?;
    match value{SemioValue::Null|SemioValue::Bool{..}|SemioValue::Float{..}=>Ok(()),SemioValue::Str{value}=>add_owned_bytes(bytes,value.len(),maximum),SemioValue::List{items:values}=>{for value in values{content_property_bytes(value,depth+1,items,bytes,maximum)?;}Ok(())},SemioValue::Map{entries}=>{for entry in entries{add_owned_bytes(bytes,entry.key.len(),maximum)?;content_property_bytes(&entry.value,depth+1,items,bytes,maximum)?;}Ok(())},_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Jack query projection cannot represent this intrinsic Semio value family"))}
}
fn content_bag_bytes(values:&[semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry],items:&mut usize,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{for entry in values{add_owned_bytes(bytes,entry.key.len(),maximum)?;content_property_bytes(&entry.value,0,items,bytes,maximum)?;}Ok(())}
fn content_node_bytes(node:&semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphNode,maximum:usize)->Result<(),ValueError>{
    let mut bytes=size_of::<Node>();let mut items=0;
    if node.ports.len()>QUERY_ENTITY_COLLECTION_MAXIMUM{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query node exceeds its port admission"))}
    for value in[&node.id.value,&node.kind,&node.label]{add_owned_bytes(&mut bytes,value.len(),maximum)?;}
    content_bag_bytes(&node.properties,&mut items,&mut bytes,maximum)?;
    for port in &node.ports{add_owned_bytes(&mut bytes,size_of::<Port>(),maximum)?;add_owned_bytes(&mut bytes,port.name.len(),maximum)?;add_owned_bytes(&mut bytes,port.category.len(),maximum)?;content_bag_bytes(&port.properties,&mut items,&mut bytes,maximum)?;}Ok(())
}
fn content_edge_bytes(edge:&semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphEdge,maximum:usize)->Result<(),ValueError>{
    let mut bytes=size_of::<Edge>();let mut items=0;
    for value in[&edge.id.value,&edge.kind,&edge.source.value,&edge.target.value]{add_owned_bytes(&mut bytes,value.len(),maximum)?;}
    for port in[edge.source_port.as_deref(),edge.target_port.as_deref()].into_iter().flatten(){add_owned_bytes(&mut bytes,port.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"query entity byte count overflow"))?,maximum)?;}
    content_bag_bytes(&edge.properties,&mut items,&mut bytes,maximum)
}

fn reserve_result<T>(values: &mut Vec<T>, owned: &mut usize, maximum: usize) -> Result<(), ValueError> {
    if values.len() < values.capacity() { return Ok(()); }
    let inline = size_of::<T>();
    let previous = values.capacity();
    let growth = previous.max(1).min(maximum.saturating_sub(*owned).checked_div(inline).unwrap_or(usize::MAX)).max(1);
    let admitted = growth.checked_mul(inline).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query result capacity overflow"))?;
    add_result_owned(owned, admitted, maximum)?;
    values.try_reserve_exact(growth).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "query result allocation refused"))?;
    let extra = values.capacity().checked_sub(previous).and_then(|count| count.checked_sub(growth)).and_then(|count| count.checked_mul(inline)).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query result capacity overflow"))?;
    add_result_owned(owned, extra, maximum)
}

fn add_result_owned(owned: &mut usize, additional: usize, maximum: usize) -> Result<(), ValueError> {
    *owned = owned.checked_add(additional).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query result byte count overflow"))?;
    if *owned > maximum { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "query result exceeds its retained ownership grant")); }
    Ok(())
}

fn graph_result_metadata_owned_bytes(graph: &Graph, maximum: usize) -> Result<usize, ValueError> {
    let mut bytes = size_of::<JackSnapshot>() + size_of::<crate::JackContentOwner>() + 2 * size_of::<usize>();
    add_result_owned(&mut bytes, JackSnapshot::SCHEMA.len() + graph.name.capacity() + graph.manifest_id.as_ref().map_or(0, String::capacity) + graph.root_node_id.as_ref().map_or(0, String::capacity), maximum)?;
    let mut properties = |values: &Vec<crate::PropertyDef>| -> Result<(), ValueError> {
        add_result_owned(&mut bytes, values.capacity().checked_mul(size_of::<crate::PropertyDef>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query metadata capacity overflow"))?, maximum)?;
        for value in values {
            add_result_owned(&mut bytes, value.name.capacity() + value.expr.as_ref().map_or(0, String::capacity), maximum)?;
            let mut value_type = &value.value_type;
            let mut depth = 0;
            loop {
                match value_type {
                    semio_framework_value::ValueType::List(inner) => {
                        depth += 1;
                        if depth > QUERY_ENTITY_NESTING_MAXIMUM { return Err(ValueError::new(ValueRefusalKind::WorkLimit, "query result metadata exceeds its nesting admission")); }
                        add_result_owned(&mut bytes, size_of::<semio_framework_value::ValueType>(), maximum)?;
                        value_type = inner;
                    }
                    semio_framework_value::ValueType::Schema(text) => { add_result_owned(&mut bytes, text.capacity(), maximum)?; break; }
                    _ => break,
                }
            }
        }
        Ok(())
    };
    for kind in &graph.manifest.node_kinds { properties(&kind.properties)?; }
    for kind in &graph.manifest.edge_kinds { properties(&kind.properties)?; }
    for kind in &graph.manifest.port_kinds { properties(&kind.properties)?; }
    for (capacity, inline) in [(graph.manifest.node_kinds.capacity(), size_of::<crate::NodeKindDef>()), (graph.manifest.edge_kinds.capacity(), size_of::<crate::EdgeKindDef>()), (graph.manifest.port_kinds.capacity(), size_of::<crate::PortKindDef>())] {
        add_result_owned(&mut bytes, capacity.checked_mul(inline).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query metadata capacity overflow"))?, maximum)?;
    }
    for kind in &graph.manifest.node_kinds {
        add_result_owned(&mut bytes, kind.name.capacity(), maximum)?;
        add_result_owned(&mut bytes, kind.port_kinds.capacity().checked_mul(size_of::<String>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query metadata capacity overflow"))?, maximum)?;
        for port in &kind.port_kinds { add_result_owned(&mut bytes, port.capacity(), maximum)?; }
    }
    for kind in &graph.manifest.edge_kinds { add_result_owned(&mut bytes, kind.name.capacity(), maximum)?; }
    for kind in &graph.manifest.port_kinds { add_result_owned(&mut bytes, kind.name.capacity(), maximum)?; }
    Ok(bytes)
}

/// 🧱 One preparation turn clones one snapshot metadata field or one working-scene entity.
pub enum QueryPreparationStep {
    Pending,
    Complete(Box<QueryExecution>),
}

/// 🧱 Incremental snapshot-to-query-workspace preparation that never clones the complete fixture.
pub struct QueryExecutionPreparation {
    query: Option<Query>,
    metadata: JackSnapshotCloneAuthority,
    metadata_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    graph: Option<Graph>,
    node: usize,
    edge: usize,
    maximum_result_owned_bytes: usize,
    closing: bool,
    terminal: bool,
}

impl QueryExecutionPreparation {
    pub fn new(query: Query) -> Self { Self::with_result_grant(query, QUERY_RESULT_MAXIMUM_OWNED_BYTES) }

    pub fn with_result_grant(query: Query, maximum_result_owned_bytes: usize) -> Self {
        Self { query: Some(query), metadata: JackSnapshotCloneAuthority::metadata_only(), metadata_retirement: None, graph: None, node: 0, edge: 0, maximum_result_owned_bytes, closing: false, terminal: false }
    }

    pub fn step(&mut self, snapshot: &JackSnapshot, scene: &SemioGraphSnapshot, maximum_bytes: usize) -> Result<QueryPreparationStep, ValueError> {
        if self.closing || self.terminal {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query preparation is terminal"));
        }
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
                query: std::mem::take(&mut metadata.query),
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
                store::SnapshotRetirementStep::Complete => return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query metadata retirement reported a false terminal")),
                _ => return Ok(QueryPreparationStep::Pending),
            }
        }
        if scene.nodes.len() > QUERY_ENTITY_MAXIMUM || scene.edges.len() > QUERY_ENTITY_MAXIMUM {
            return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query snapshot exceeds its entity admission"));
        }
        if let Some(node) = scene.nodes.get(self.node) {
            content_node_bytes(node, maximum_bytes)?;
            let graph = self.graph.as_mut().expect("query preparation graph is initialized");
            if graph.nodes.contains_key(&node.id.value) {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("duplicate node id {}", node.id.value)));
            }
            graph.nodes.insert(node.id.value.clone(), crate::jack_node_from_content(node)?);
            self.node += 1;
            return Ok(QueryPreparationStep::Pending);
        }
        if let Some(edge) = scene.edges.get(self.edge) {
            content_edge_bytes(edge, maximum_bytes)?;
            let graph = self.graph.as_mut().expect("query preparation graph is initialized");
            if graph.edges.contains_key(&edge.id.value) {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("duplicate edge id {}", edge.id.value)));
            }
            graph.edges.insert(edge.id.value.clone(), crate::jack_edge_from_content(edge)?);
            self.edge += 1;
            return Ok(QueryPreparationStep::Pending);
        }
        let graph = self.graph.take().expect("query preparation graph remains owned");
        let query = self.query.take().expect("query preparation AST remains owned");
        self.terminal = true;
        Ok(QueryPreparationStep::Complete(Box::new(QueryExecution::with_result_grant(graph, query, self.maximum_result_owned_bytes))))
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, ValueError> {
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(retirement) = self.metadata_retirement.as_mut() {
            match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => self.metadata_retirement = None,
                store::SnapshotRetirementStep::Complete => return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query preparation metadata retirement reported a false terminal")),
                step => return Ok(step),
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.metadata.terminal_is_empty() {
            return match self.metadata.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if self.metadata.terminal_is_empty() => Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }),
                store::SnapshotRetirementStep::Complete => Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query preparation metadata clone reported a false terminal")),
                step => Ok(step),
            };
        }
        if let Some(graph) = self.graph.as_mut() {
            if let Some((_, node)) = graph.nodes.pop_first() {
                self.metadata_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, GraphEffect::CreateNode(node)));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some((_, edge)) = graph.edges.pop_first() {
                self.metadata_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, GraphEffect::CreateEdge(edge)));
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

    fn step(&mut self, graph: &Graph, patterns: &[Pattern]) -> Result<Option<Vec<Binding>>, ValueError> {
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
            if edge_pattern.kind.as_ref().is_some_and(|kind| *kind != edge.kind) || crate::port_node_id(&edge.source) != Some(node_id.as_str()) {
                return Ok(None);
            }
            let Some(target) = crate::port_node_id(&edge.target) else { return Ok(None) };
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

struct DeleteExecution {
    operation: GraphEffect,
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
    result_owned_bytes: usize,
    maximum_result_owned_bytes: usize,
}

impl ReturnExecution {
    fn new(maximum_result_owned_bytes: usize) -> Self {
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
    fn step(&mut self, graph: &mut Graph, bindings: &[Binding], items: &[ReturnItem]) -> Result<Option<QueryResult>, ValueError> {
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
                    let content = crate::jack_content_child_with_owner(std::mem::take(&mut self.nodes), std::mem::take(&mut self.edges));
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
    pending: VecDeque<GraphEffect>,
    deleting: Option<DeleteExecution>,
    operations: Vec<GraphEffect>,
    maximum_result_owned_bytes: usize,
    pending_clause_advance: bool,
    finished: bool,
    closing: bool,
    retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    metadata_retired: bool,
    terminal: bool,
}

impl QueryExecution {
    pub fn new(graph: Graph, query: Query) -> Self {
        Self::with_result_grant(graph, query, QUERY_RESULT_MAXIMUM_OWNED_BYTES)
    }
    pub fn with_result_grant(graph: Graph, query: Query, maximum_result_owned_bytes: usize) -> Self {
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
        if let Some(retirement) = self.retirement.as_mut() {
            match retirement.close_step(1, 4_096)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => self.retirement = None,
                store::SnapshotRetirementStep::Complete => return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query mutation retirement reported a false terminal")),
                store::SnapshotRetirementStep::Pending { .. } => return Ok(true),
                store::SnapshotRetirementStep::Blocked => return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query mutation retirement unexpectedly blocked")),
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
                        self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, GraphEffect::CreateEdge(removed)));
                    }
                }
                return Ok(true);
            }
            if let Some(removed) = self.graph.nodes.remove(&deleting.id) {
                self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, GraphEffect::CreateNode(removed)));
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
                    self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, GraphEffect::CreateEdge(removed)));
                }
                Ok(())
            }
            GraphEffect::RenameNode { id, name } => match self.graph.nodes.get_mut(id) {
                Some(node) => {
                    let previous = std::mem::replace(&mut node.name, name.clone());
                    self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, GraphEffect::DeleteNode(previous)));
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
                            self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, GraphEffect::SetProperty { entity: EntityRef::Node(String::new()), key: String::new(), value: previous }));
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
                            self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, GraphEffect::SetProperty { entity: EntityRef::Node(String::new()), key: String::new(), value: previous }));
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
    fn retire_effect(&mut self, effect: GraphEffect) {
        self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, effect));
    }
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, ValueError> {
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => self.retirement = None,
                store::SnapshotRetirementStep::Complete => return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"query owner retirement reported a false terminal")),
                step => return Ok(step),
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(deleting) = self.deleting.as_mut() {
            if let Some(value) = deleting.edge.take() {
                self.retire_effect(GraphEffect::DeleteNode(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !deleting.id.is_empty() {
                let value = std::mem::take(&mut deleting.id);
                self.retire_effect(GraphEffect::DeleteNode(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        if let Some(deleting) = self.deleting.take() {
            self.retire_effect(deleting.operation);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(mutation) = self.pending.pop_front().or_else(|| self.operations.pop()) {
            self.retire_effect(mutation);
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
                self.retire_effect(GraphEffect::DeleteNode(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.matching = None;
        }
        if let Some(returning) = self.returning.as_mut() {
            if let Some(node) = returning.nodes.pop() {
                self.retire_effect(GraphEffect::CreateNode(node));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(edge) = returning.edges.pop() {
                self.retire_effect(GraphEffect::CreateEdge(edge));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(value) = returning.rows.last_mut().and_then(Vec::pop) {
                self.retire_effect(GraphEffect::SetProperty { entity: EntityRef::Node(String::new()), key: String::new(), value });
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if returning.rows.pop().is_some() {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(value) = returning.columns.pop().or_else(|| returning.node_ids.pop_first()).or_else(|| returning.edge_ids.pop_first()) {
                self.retire_effect(GraphEffect::DeleteNode(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.returning = None;
        }
        if let Some(binding) = self.bindings.last_mut() {
            if let Some((key, value)) = binding.nodes.pop_first().or_else(|| binding.edges.pop_first()) {
                self.retire_effect(GraphEffect::RemoveProperty { entity: EntityRef::Node(value), key });
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.bindings.pop();
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some((_, node)) = self.graph.nodes.pop_first() {
            self.retire_effect(GraphEffect::CreateNode(node));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some((_, edge)) = self.graph.edges.pop_first() {
            self.retire_effect(GraphEffect::CreateEdge(edge));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.metadata_retired {
            let metadata = JackSnapshot {
                name: std::mem::take(&mut self.graph.name),
                manifest_id: self.graph.manifest_id.take(),
                manifest: std::mem::take(&mut self.graph.manifest),
                camera: self.graph.camera.clone(),
                root_node_id: self.graph.root_node_id.take(),
                ..Default::default()
            };
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

pub(super) fn emit_set_operation_from_graph(graph: &Graph, node_id: &str, prop: &str, value: PropertyValue) -> Result<GraphEffect, ValueError> {
    let node = graph.nodes.get(node_id).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id} not found")))?;
    match prop {
        "name" => match value {
            PropertyValue::String(name) => Ok(GraphEffect::RenameNode { id: node_id.to_string(), name }),
            _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id}.name expects string value"))),
        },
        "x" => Ok(GraphEffect::MoveNode { id: node_id.to_string(), x: value.as_f64().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id}.x expects number value")))?, y: node.y }),
        "y" => Ok(GraphEffect::MoveNode { id: node_id.to_string(), x: node.x, y: value.as_f64().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id}.y expects number value")))? }),
        _ => Ok(GraphEffect::SetProperty { entity: EntityRef::Node(node_id.to_string()), key: prop.to_string(), value }),
    }
}

pub(super) fn emit_create_operations_from_graph(graph: &Graph, pattern: &Pattern) -> Result<Vec<GraphEffect>, ValueError> {
    let left = pattern.nodes.first().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"empty create pattern"))?;
    let left_id = format!("{}-{}", left.var, graph.nodes.len());
    let mut operations = Vec::with_capacity(if pattern.edge.is_some() { 3 } else { 1 });
    let mut left_ports = Vec::new();
    if pattern.edge.is_some() {
        left_ports.push(Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
    }
    operations.push(GraphEffect::CreateNode(Node { id: left_id.clone(), kind: left.kind.clone(), name: left.var.clone(), x: graph.nodes.len() as f64 * 120.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: left_ports }));
    if let Some(edge_pattern) = &pattern.edge {
        let right_id = format!("{}-{}", edge_pattern.right.var, graph.nodes.len() + 1);
        operations.push(GraphEffect::CreateNode(Node {
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
        operations.push(GraphEffect::CreateEdge(Edge {
            id: format!("e-{}", graph.edges.len()),
            kind: edge_pattern.kind.clone().unwrap_or_else(|| "Connection".into()),
            source: port_key(&left_id, "out"),
            target: port_key(&right_id, "in"),
            properties: PropertyBag::new(),
        }));
    }
    Ok(operations)
}
