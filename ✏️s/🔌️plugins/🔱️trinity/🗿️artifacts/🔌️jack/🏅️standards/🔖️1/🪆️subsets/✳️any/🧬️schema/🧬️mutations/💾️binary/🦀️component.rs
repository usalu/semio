//! 📡️ `trinity.graph` artifact — state-patch wire codec for the raw document operation
//! (constitutional: spr, renamed from the old `📡️protocol` — no `📡️protocol` segment survives).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::jack::dsl::{port_dsl_to_port, port_to_port_dsl, PortDsl};
use crate::artifacts::jack::mutations::{change_data_property, create_edge, create_node, delete_edge, delete_node, move_node, remove_data_property, rename_node};
use crate::artifacts::jack::schema::mutations::text::TrinityGraphMutation;
use crate::artifacts::jack::{Edge, EntityRef, JackSnapshot, Node, Port, PropertyBag, PropertyDef, PropertyValue};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use store::TextError;

//#region 🔖️DslMirrors
/// 🏷️ The `entity` half of `EntityRefDsl` — a plain 2-variant scalar tag (`dsl::DslScalar`, not
/// `DslEnum`): `EntityRefDsl` needs `dsl::DslField` (to bind as an ordinary record field on
/// `TrinityGraphOperationDsl`'s variants), and a `DslRecord` of `{ kind, id }` gets that directly,
/// unlike a tagged-variant `DslEnum`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar)]
enum EntityKindDsl {
    Node,
    Edge,
}

/// 🎯️ Local twin of `EntityRef` purely for the DSL engine's tuple-variant limitation — a flat
/// `{ kind, id }` twin, converted at the op-text boundary via `From`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct EntityRefDsl {
    kind: EntityKindDsl,
    id: String,
}

impl From<&EntityRef> for EntityRefDsl {
    fn from(value: &EntityRef) -> Self {
        match value {
            EntityRef::Node(id) => EntityRefDsl { kind: EntityKindDsl::Node, id: id.clone() },
            EntityRef::Edge(id) => EntityRefDsl { kind: EntityKindDsl::Edge, id: id.clone() },
        }
    }
}

impl From<EntityRefDsl> for EntityRef {
    fn from(value: EntityRefDsl) -> Self {
        match value.kind {
            EntityKindDsl::Node => EntityRef::Node(value.id),
            EntityKindDsl::Edge => EntityRef::Edge(value.id),
        }
    }
}

/// ⚡️ Local mirror of `TrinityGraphMutation` for `protocol::OpText`/`OpBinary` — `entity: EntityRef`
/// and `ports` fields transitively carry foreign/tuple-variant shapes, so the real enum (whose
/// variants each wrap a handcrafted `🦠️mutation` payload struct) can't derive `dsl::DslOps`
/// directly; this mirror's own variant names ARE the wire keywords (kept in lockstep with the real
/// enum's semantic slugs: `RenameNode` -> `rename-node`, etc.).
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum TrinityGraphOperationDsl {
    CreateNode {
        id: String,
        kind: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[dsl(table)]
        ports: Vec<PortDsl>,
    },
    DeleteNode {
        id: String,
    },
    CreateEdge {
        id: String,
        kind: String,
        source: String,
        target: String,
        properties: crate::artifacts::jack::PropertyBag,
    },
    DeleteEdge {
        id: String,
    },
    RenameNode {
        id: String,
        name: String,
    },
    MoveNode {
        id: String,
        x: f64,
        y: f64,
    },
    ChangeDataProperty {
        entity: EntityRefDsl,
        key: String,
        value: PropertyValue,
    },
    RemoveDataProperty {
        entity: EntityRefDsl,
        key: String,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for TrinityGraphOperationDsl {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl OpBinary for TrinityGraphOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn trinity_graph_operation_to_dsl(operation: &TrinityGraphMutation) -> TrinityGraphOperationDsl {
    match operation {
        TrinityGraphMutation::CreateNode(payload) => {
            let node = &payload.node;
            TrinityGraphOperationDsl::CreateNode { id: node.id.clone(), kind: node.kind.clone(), name: node.name.clone(), x: node.x, y: node.y, width: node.width, height: node.height, ports: node.ports.iter().map(port_to_port_dsl).collect() }
        }
        TrinityGraphMutation::DeleteNode(payload) => TrinityGraphOperationDsl::DeleteNode { id: payload.id.clone() },
        TrinityGraphMutation::CreateEdge(payload) => {
            let edge = &payload.edge;
            TrinityGraphOperationDsl::CreateEdge { id: edge.id.clone(), kind: edge.kind.clone(), source: edge.source.clone(), target: edge.target.clone(), properties: edge.properties.clone() }
        }
        TrinityGraphMutation::DeleteEdge(payload) => TrinityGraphOperationDsl::DeleteEdge { id: payload.id.clone() },
        TrinityGraphMutation::RenameNode(payload) => TrinityGraphOperationDsl::RenameNode { id: payload.id.clone(), name: payload.new_name.clone() },
        TrinityGraphMutation::MoveNode(payload) => TrinityGraphOperationDsl::MoveNode { id: payload.id.clone(), x: payload.x, y: payload.y },
        TrinityGraphMutation::ChangeDataProperty(payload) => TrinityGraphOperationDsl::ChangeDataProperty { entity: (&payload.entity).into(), key: payload.key.clone(), value: payload.new_value.clone() },
        TrinityGraphMutation::RemoveDataProperty(payload) => TrinityGraphOperationDsl::RemoveDataProperty { entity: (&payload.entity).into(), key: payload.key.clone() },
    }
}

fn trinity_graph_operation_from_dsl(operation: TrinityGraphOperationDsl) -> TrinityGraphMutation {
    match operation {
        TrinityGraphOperationDsl::CreateNode { id, kind, name, x, y, width, height, ports } => {
            create_node(Node { id, kind, name, x, y, width, height, properties: crate::artifacts::jack::PropertyBag::new(), ports: ports.into_iter().map(port_dsl_to_port).collect() })
        }
        TrinityGraphOperationDsl::DeleteNode { id } => delete_node(id),
        TrinityGraphOperationDsl::CreateEdge { id, kind, source, target, properties } => create_edge(Edge { id, kind, source, target, properties }),
        TrinityGraphOperationDsl::DeleteEdge { id } => delete_edge(id),
        TrinityGraphOperationDsl::RenameNode { id, name } => rename_node(id, name),
        TrinityGraphOperationDsl::MoveNode { id, x, y } => move_node(id, x, y),
        TrinityGraphOperationDsl::ChangeDataProperty { entity, key, value } => change_data_property(entity.into(), key, value),
        TrinityGraphOperationDsl::RemoveDataProperty { entity, key } => remove_data_property(entity.into(), key),
    }
}
//#endregion 🔖️DslMirrors

//#region 🔖️OpText
/// ⚡️ One-line textual notation for [`TrinityGraphMutation`] (`protocol::OpText`), delegating to the
/// derive-generated `TrinityGraphOperationDsl` mirror.
impl OpText for TrinityGraphMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        <TrinityGraphOperationDsl as OpText>::parse_op(line).map(trinity_graph_operation_from_dsl)
    }

    fn print_op(&self) -> String {
        <TrinityGraphOperationDsl as OpText>::print_op(&trinity_graph_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `TrinityGraphOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl OpBinary for TrinityGraphMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        trinity_graph_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        TrinityGraphOperationDsl::decode_op(bytes).map(trinity_graph_operation_from_dsl)
    }
}
//#endregion 🔖️OpText

/// 📦️ Encodes a Trinity graph `Mutation` to its binary command form.
pub fn encode_op(operation: &TrinityGraphMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a Trinity graph `Mutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<TrinityGraphMutation, protocol::ProtocolError> {
    TrinityGraphMutation::decode_op(bytes)
}

//#region 🔖️OwnedSprCatalog
const JACK_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

enum JackMutationFields {
    CreateNode(Option<Node>),
    DeleteNode(String),
    CreateEdge(Option<Edge>),
    DeleteEdge(String),
    RenameNode { id: String, name: String },
    MoveNode(String),
    ChangeDataProperty { entity: Option<EntityRef>, key: String, value: Option<PropertyValue> },
    RemoveDataProperty { entity: Option<EntityRef>, key: String },
}

enum JackRetirementOwner {
    Snapshot(JackSnapshot),
    Mutation(TrinityGraphMutation),
    MutationFields(JackMutationFields),
    Property(PropertyValue),
    Bag(PropertyBag),
    Node(Node),
    Edge(Edge),
    Port(Port),
    PropertyDef(PropertyDef),
    NodeKind(graph::manifest::TrinityNodeKindDef),
    EdgeKind(graph::manifest::TrinityEdgeKindDef),
    PortKind(graph::manifest::TrinityPortKindDef),
}

struct JackOwnedRetirement {
    owner: std::mem::ManuallyDrop<Option<JackRetirementOwner>>,
    active: std::mem::ManuallyDrop<Option<Box<JackOwnedRetirement>>>,
    phase: u8,
}

impl JackOwnedRetirement {
    fn new(owner: JackRetirementOwner) -> Self {
        Self { owner: std::mem::ManuallyDrop::new(Some(owner)), active: std::mem::ManuallyDrop::new(None), phase: 0 }
    }

    fn string_step(value: &mut String, maximum_items: usize, maximum_bytes: usize) -> Option<store::SnapshotRetirementStep> {
        if maximum_items == 0 || value.len() > maximum_bytes {
            return Some(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released_bytes = value.len();
        drop(std::mem::take(value));
        Some(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes })
    }

    fn phased_string_step(value: &mut String, phase: &mut u8, next: u8, maximum_items: usize, maximum_bytes: usize) -> store::SnapshotRetirementStep {
        let step = Self::string_step(value, maximum_items, maximum_bytes).expect("required Jack string owner remains established");
        if matches!(step, store::SnapshotRetirementStep::Pending { released_items: 1, .. }) {
            *phase = next;
        }
        step
    }

    fn optional_string_step(value: &mut Option<String>, maximum_items: usize, maximum_bytes: usize) -> Option<store::SnapshotRetirementStep> {
        let Some(string) = value.as_ref() else { return None };
        if maximum_items == 0 || string.len() > maximum_bytes {
            return Some(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released_bytes = string.len();
        drop(value.take());
        Some(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes })
    }

    fn spawn(active: &mut std::mem::ManuallyDrop<Option<Box<JackOwnedRetirement>>>, owner: JackRetirementOwner) -> store::SnapshotRetirementStep {
        **active = Some(Box::new(Self::new(owner)));
        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
    }

    fn entity_step(value: &mut Option<EntityRef>, maximum_items: usize, maximum_bytes: usize) -> Option<store::SnapshotRetirementStep> {
        let entity = value.as_mut()?;
        let id = match entity {
            EntityRef::Node(id) | EntityRef::Edge(id) => id,
        };
        if maximum_items == 0 || id.len() > maximum_bytes {
            return Some(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released_bytes = id.len();
        drop(std::mem::take(id));
        drop(value.take());
        Some(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes })
    }

    fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        let Some(owner) = self.owner.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match owner {
            JackRetirementOwner::Snapshot(value) => match self.phase {
                0 => Ok(Self::phased_string_step(&mut value.schema, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => Ok(Self::phased_string_step(&mut value.name, &mut self.phase, 2, maximum_items, maximum_bytes)),
                2 => {
                    if let Some(step) = Self::optional_string_step(&mut value.manifest_id, maximum_items, maximum_bytes) {
                        return Ok(step);
                    }
                    self.phase = 3;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                3 => {
                    if let Some(kind) = value.manifest.node_kinds.pop() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::NodeKind(kind)));
                    }
                    self.phase = 4;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                4 => {
                    if let Some(kind) = value.manifest.edge_kinds.pop() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::EdgeKind(kind)));
                    }
                    self.phase = 5;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                5 => {
                    if let Some(kind) = value.manifest.port_kinds.pop() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::PortKind(kind)));
                    }
                    self.phase = 6;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                6 => Ok(Self::phased_string_step(&mut value.content.child_id, &mut self.phase, 7, maximum_items, maximum_bytes)),
                7 => Ok(Self::phased_string_step(&mut value.content.target.artifact_id, &mut self.phase, 8, maximum_items, maximum_bytes)),
                8 => Ok(Self::phased_string_step(&mut value.content.target.dialect.artifact_kind, &mut self.phase, 9, maximum_items, maximum_bytes)),
                9 => Ok(Self::phased_string_step(&mut value.content.target.dialect.standard, &mut self.phase, 10, maximum_items, maximum_bytes)),
                10 => Ok(Self::phased_string_step(&mut value.content.target.dialect.subset, &mut self.phase, 11, maximum_items, maximum_bytes)),
                11 => {
                    if let Some(step) = Self::optional_string_step(&mut value.root_node_id, maximum_items, maximum_bytes) {
                        return Ok(step);
                    }
                    self.phase = 12;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            JackRetirementOwner::Mutation(_) => {
                let mutation = match self.owner.take() {
                    Some(JackRetirementOwner::Mutation(value)) => value,
                    _ => unreachable!("Jack mutation owner variant remains exact"),
                };
                let fields = match mutation {
                    TrinityGraphMutation::CreateNode(value) => JackMutationFields::CreateNode(Some(value.node)),
                    TrinityGraphMutation::DeleteNode(value) => JackMutationFields::DeleteNode(value.id),
                    TrinityGraphMutation::CreateEdge(value) => JackMutationFields::CreateEdge(Some(value.edge)),
                    TrinityGraphMutation::DeleteEdge(value) => JackMutationFields::DeleteEdge(value.id),
                    TrinityGraphMutation::RenameNode(value) => JackMutationFields::RenameNode { id: value.id, name: value.new_name },
                    TrinityGraphMutation::MoveNode(value) => JackMutationFields::MoveNode(value.id),
                    TrinityGraphMutation::ChangeDataProperty(value) => JackMutationFields::ChangeDataProperty { entity: Some(value.entity), key: value.key, value: Some(value.new_value) },
                    TrinityGraphMutation::RemoveDataProperty(value) => JackMutationFields::RemoveDataProperty { entity: Some(value.entity), key: value.key },
                };
                *self.owner = Some(JackRetirementOwner::MutationFields(fields));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            JackRetirementOwner::MutationFields(fields) => match fields {
                JackMutationFields::CreateNode(value) => {
                    if let Some(value) = value.take() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::Node(value)));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                JackMutationFields::DeleteNode(value) | JackMutationFields::DeleteEdge(value) | JackMutationFields::MoveNode(value) => {
                    if self.phase == 0 {
                        return Ok(Self::phased_string_step(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                JackMutationFields::CreateEdge(value) => {
                    if let Some(value) = value.take() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::Edge(value)));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                JackMutationFields::RenameNode { id, name } => {
                    let value = if self.phase == 0 { id } else { name };
                    if self.phase < 2 {
                        let next = self.phase + 1;
                        return Ok(Self::phased_string_step(value, &mut self.phase, next, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                JackMutationFields::ChangeDataProperty { entity, key, value } => match self.phase {
                    0 => {
                        if let Some(step) = Self::entity_step(entity, maximum_items, maximum_bytes) {
                            return Ok(step);
                        }
                        self.phase = 1;
                        Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                    1 => Ok(Self::phased_string_step(key, &mut self.phase, 2, maximum_items, maximum_bytes)),
                    2 => {
                        if let Some(value) = value.take() {
                            self.phase = 3;
                            return Ok(Self::spawn(&mut self.active, JackRetirementOwner::Property(value)));
                        }
                        self.phase = 3;
                        Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
                JackMutationFields::RemoveDataProperty { entity, key } => match self.phase {
                    0 => {
                        if let Some(step) = Self::entity_step(entity, maximum_items, maximum_bytes) {
                            return Ok(step);
                        }
                        self.phase = 1;
                        Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                    1 => Ok(Self::phased_string_step(key, &mut self.phase, 2, maximum_items, maximum_bytes)),
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
            },
            JackRetirementOwner::Property(value) => match value {
                PropertyValue::String(value) if self.phase == 0 => Ok(Self::phased_string_step(value, &mut self.phase, 1, maximum_items, maximum_bytes)),
                PropertyValue::Array(values) => {
                    if let Some(value) = values.pop() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::Property(value)));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                PropertyValue::Object(values) => {
                    if let Some((key, value)) = values.pop_first() {
                        if key.len() > maximum_bytes || maximum_items == 0 {
                            values.insert(key, value);
                            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                        }
                        let released_bytes = key.len();
                        drop(key);
                        *self.active = Some(Box::new(Self::new(JackRetirementOwner::Property(value))));
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            JackRetirementOwner::Bag(values) => {
                if let Some((key, value)) = values.pop_first() {
                    if key.len() > maximum_bytes || maximum_items == 0 {
                        values.insert(key, value);
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    let released_bytes = key.len();
                    drop(key);
                    *self.active = Some(Box::new(Self::new(JackRetirementOwner::Property(value))));
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
                }
                drop(self.owner.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            JackRetirementOwner::Node(value) => match self.phase {
                0 => Ok(Self::phased_string_step(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => Ok(Self::phased_string_step(&mut value.kind, &mut self.phase, 2, maximum_items, maximum_bytes)),
                2 => Ok(Self::phased_string_step(&mut value.name, &mut self.phase, 3, maximum_items, maximum_bytes)),
                3 => {
                    if !value.properties.is_empty() {
                        self.phase = 4;
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::Bag(std::mem::take(&mut value.properties))));
                    }
                    self.phase = 4;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                4 => {
                    if let Some(port) = value.ports.pop() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::Port(port)));
                    }
                    self.phase = 5;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            JackRetirementOwner::Edge(value) => match self.phase {
                0 => Ok(Self::phased_string_step(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => Ok(Self::phased_string_step(&mut value.kind, &mut self.phase, 2, maximum_items, maximum_bytes)),
                2 => Ok(Self::phased_string_step(&mut value.source, &mut self.phase, 3, maximum_items, maximum_bytes)),
                3 => Ok(Self::phased_string_step(&mut value.target, &mut self.phase, 4, maximum_items, maximum_bytes)),
                4 => {
                    if !value.properties.is_empty() {
                        self.phase = 5;
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::Bag(std::mem::take(&mut value.properties))));
                    }
                    self.phase = 5;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            JackRetirementOwner::Port(value) => match self.phase {
                0 => Ok(Self::phased_string_step(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => Ok(Self::phased_string_step(&mut value.kind, &mut self.phase, 2, maximum_items, maximum_bytes)),
                2 => {
                    if !value.properties.is_empty() {
                        self.phase = 3;
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::Bag(std::mem::take(&mut value.properties))));
                    }
                    self.phase = 3;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            JackRetirementOwner::PropertyDef(value) => match self.phase {
                0 => Ok(Self::phased_string_step(&mut value.name, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    if let Some(step) = Self::optional_string_step(&mut value.expr, maximum_items, maximum_bytes) {
                        return Ok(step);
                    }
                    self.phase = 2;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                2 => {
                    if maximum_items == 0 {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    let value_type = match value.retire_value_type_step(maximum_bytes) {
                        Ok(value_type) => value_type,
                        Err(()) => return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
                    };
                    if let Some(value_type) = value_type {
                        let released_bytes = value_type.len();
                        drop(value_type);
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
                    }
                    if !value.value_type_terminal_is_empty() {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                    }
                    self.phase = 3;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            JackRetirementOwner::NodeKind(value) => match self.phase {
                0 => Ok(Self::phased_string_step(&mut value.name, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    if let Some(value) = value.properties.pop() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::PropertyDef(value)));
                    }
                    self.phase = 2;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                2 => {
                    if let Some(port_kind) = value.port_kinds.pop() {
                        if port_kind.len() > maximum_bytes || maximum_items == 0 {
                            value.port_kinds.push(port_kind);
                            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                        }
                        let released_bytes = port_kind.len();
                        drop(port_kind);
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
                    }
                    self.phase = 3;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            JackRetirementOwner::EdgeKind(value) => match self.phase {
                0 => Ok(Self::phased_string_step(&mut value.name, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    if let Some(value) = value.properties.pop() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::PropertyDef(value)));
                    }
                    self.phase = 2;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            JackRetirementOwner::PortKind(value) => match self.phase {
                0 => Ok(Self::phased_string_step(&mut value.name, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    if let Some(value) = value.properties.pop() {
                        return Ok(Self::spawn(&mut self.active, JackRetirementOwner::PropertyDef(value)));
                    }
                    self.phase = 2;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
        }
    }
}

impl store::ErasedSnapshotRetirement for JackOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(maximum_items.min(1), maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("Jack nested retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        self.advance(maximum_items.min(1), maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.active.is_none()
    }
}

impl Drop for JackOwnedRetirement {
    fn drop(&mut self) {
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(self), "Jack owner reached Drop before cursor retirement reached terminal-empty");
    }
}

pub struct JackSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<JackSnapshot> for JackSnapshotRetirementFactory {
    fn retire_owned(&self, value: JackSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(JackOwnedRetirement::new(JackRetirementOwner::Snapshot(value)))
    }
}

struct JackSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<JackSnapshot>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl store::ErasedSnapshotRetirement for JackSnapshotRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("Jack root retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        let Some(owner) = self.owner.take() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, value));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            Err(owner) => {
                *self.owner = Some(owner);
                Ok(store::SnapshotRetirementStep::Blocked)
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotRootRetirement {
    fn drop(&mut self) {
        assert!(self.owner.is_none() && self.retirement.is_none(), "Jack snapshot root reached Drop before exact Arc handback");
    }
}

impl store::SnapshotRetirementFactory<JackSnapshot> for JackSnapshotRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<JackSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(JackSnapshotRootRetirement { owner: std::mem::ManuallyDrop::new(Some(snapshot)), retirement: std::mem::ManuallyDrop::new(None) })
    }
}

pub struct JackMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<TrinityGraphMutation> for JackMutationRetirementFactory {
    fn retire_owned(&self, value: TrinityGraphMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(JackOwnedRetirement::new(JackRetirementOwner::Mutation(value)))
    }
}

enum JackSnapshotDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct JackSnapshotDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: JackSnapshotDecodeState,
    value: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl JackSnapshotDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: JackSnapshotDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot> for JackSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
        if matches!(self.state, JackSnapshotDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("jack-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            self.state = JackSnapshotDecodeState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let JackSnapshotDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("jack-envelope.snapshot-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("jack-envelope.snapshot-pack-missing", token.start))?;
                let value = <JackSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| diagnostic("jack-envelope.snapshot-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Jack snapshot pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = JackSnapshotDecodeState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(diagnostic("jack-envelope.snapshot-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<JackSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, JackSnapshotDecodeState::Ready) {
            return Err(self.diagnostic("jack-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("jack-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = JackSnapshotDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let JackSnapshotDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = JackSnapshotDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, value));
                self.state = JackSnapshotDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = JackSnapshotDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let path = self.path;
        let retirement = self.retirement.as_mut().expect("Jack snapshot retirement remains retained");
        match retirement.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "jack-envelope.snapshot-retirement-fault", offset: 0, line: 0, column: 0, path })? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = JackSnapshotDecodeState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(self.diagnostic("jack-envelope.snapshot-retirement-false-terminal", 0)),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, JackSnapshotDecodeState::Published | JackSnapshotDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotDecodeAuthority {
    fn drop(&mut self) {
        assert!(store::ArtifactEnvelopeSnapshotFieldAuthority::terminal_is_empty(self), "Jack snapshot decode reached Drop before publication or bounded retirement");
    }
}

enum JackMutationDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct JackMutationDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: JackMutationDecodeState,
    value: std::mem::ManuallyDrop<Option<TrinityGraphMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl JackMutationDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: JackMutationDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }
}

impl store::ArtifactEnvelopeMutationFieldAuthority<TrinityGraphMutation> for JackMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
        if matches!(self.state, JackMutationDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("jack-envelope.mutation-pack-must-be-scalar", token.start));
            }
            self.state = JackMutationDecodeState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let JackMutationDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("jack-envelope.mutation-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("jack-envelope.mutation-pack-missing", token.start))?;
                let value = TrinityGraphMutation::decode_op(bytes).map_err(|_| diagnostic("jack-envelope.mutation-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Jack mutation pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = JackMutationDecodeState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(diagnostic("jack-envelope.mutation-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<TrinityGraphMutation>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, JackMutationDecodeState::Ready) {
            return Err(self.diagnostic("jack-envelope.mutation-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("jack-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = JackMutationDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let JackMutationDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = JackMutationDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, value));
                self.state = JackMutationDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = JackMutationDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let path = self.path;
        let retirement = self.retirement.as_mut().expect("Jack mutation retirement remains retained");
        match retirement.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "jack-envelope.mutation-retirement-fault", offset: 0, line: 0, column: 0, path })? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = JackMutationDecodeState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(self.diagnostic("jack-envelope.mutation-retirement-false-terminal", 0)),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, JackMutationDecodeState::Published | JackMutationDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(store::ArtifactEnvelopeMutationFieldAuthority::terminal_is_empty(self), "Jack mutation decode reached Drop before publication or bounded retirement");
    }
}

struct JackRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for JackRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "jack-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

pub struct JackEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<JackSnapshot, TrinityGraphMutation> for JackEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<JackSnapshot, TrinityGraphMutation>> {
        Box::new(store::ArtifactEnvelopeFreshVcsAuthority::new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(JackSnapshotRetirementFactory), std::sync::Arc::new(JackMutationRetirementFactory), self.edit_history_decoder()))
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot>> {
        Box::new(JackSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<TrinityGraphMutation>> {
        Box::new(JackMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(JackRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<TrinityGraphMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(JackMutationRetirementFactory))
    }
}

pub fn jack_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<JackSnapshot, TrinityGraphMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(JackEnvelopeOwnedFieldCatalog), std::sync::Arc::new(JackSnapshotRetirementFactory), std::sync::Arc::new(JackMutationRetirementFactory))
}
//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization
enum JackSnapshotCloneKind {
    Node { source: usize, property: usize, port: usize, value: graph::manifest::TrinityNodeKindDef },
    Edge { source: usize, property: usize, value: graph::manifest::TrinityEdgeKindDef },
    Port { source: usize, property: usize, value: graph::manifest::TrinityPortKindDef },
}

struct JackSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    active: std::mem::ManuallyDrop<Option<JackSnapshotCloneKind>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: u8,
    index: usize,
    terminal: bool,
}

impl JackSnapshotCloneAuthority {
    fn new() -> Self {
        let content = store::ArtifactChild::new(String::new(), store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
        Self {
            value: std::mem::ManuallyDrop::new(Some(JackSnapshot { schema: String::new(), name: String::new(), manifest_id: None, manifest: Default::default(), camera: Default::default(), content, root_node_id: None })),
            active: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            index: 0,
            terminal: false,
        }
    }

    fn clone_string(source: &str) -> Result<String, &'static str> {
        if source.len() > JACK_OWNED_FIELD_BYTES {
            return Err("jack-store.initializer-field-too-large");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_| "jack-store.initializer-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn clone_property(source: &PropertyDef) -> Result<PropertyDef, &'static str> {
        if source.name.len() > JACK_OWNED_FIELD_BYTES || source.expr.as_ref().is_some_and(|value| value.len() > JACK_OWNED_FIELD_BYTES) {
            return Err("jack-store.initializer-property-too-large");
        }
        Ok(source.clone())
    }

    fn begin_kind(&mut self, source: &JackSnapshot) -> Result<bool, &'static str> {
        let target = self.value.as_mut().ok_or("jack-store.initializer-clone-target")?;
        match self.phase {
            4 => {
                if self.index == 0 && target.manifest.node_kinds.capacity() == 0 {
                    target.manifest.node_kinds.try_reserve_exact(source.manifest.node_kinds.len()).map_err(|_| "jack-store.initializer-node-kind-admission")?;
                }
                let Some(kind) = source.manifest.node_kinds.get(self.index) else {
                    self.phase = 5;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_| "jack-store.initializer-node-property-admission")?;
                let mut port_kinds = Vec::new();
                port_kinds.try_reserve_exact(kind.port_kinds.len()).map_err(|_| "jack-store.initializer-node-port-admission")?;
                *self.active = Some(JackSnapshotCloneKind::Node { source: self.index, property: 0, port: 0, value: graph::manifest::TrinityNodeKindDef { name: Self::clone_string(&kind.name)?, properties, port_kinds } });
                Ok(true)
            }
            5 => {
                if self.index == 0 && target.manifest.edge_kinds.capacity() == 0 {
                    target.manifest.edge_kinds.try_reserve_exact(source.manifest.edge_kinds.len()).map_err(|_| "jack-store.initializer-edge-kind-admission")?;
                }
                let Some(kind) = source.manifest.edge_kinds.get(self.index) else {
                    self.phase = 6;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_| "jack-store.initializer-edge-property-admission")?;
                *self.active = Some(JackSnapshotCloneKind::Edge { source: self.index, property: 0, value: graph::manifest::TrinityEdgeKindDef { name: Self::clone_string(&kind.name)?, properties } });
                Ok(true)
            }
            6 => {
                if self.index == 0 && target.manifest.port_kinds.capacity() == 0 {
                    target.manifest.port_kinds.try_reserve_exact(source.manifest.port_kinds.len()).map_err(|_| "jack-store.initializer-port-kind-admission")?;
                }
                let Some(kind) = source.manifest.port_kinds.get(self.index) else {
                    self.phase = 7;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_| "jack-store.initializer-port-property-admission")?;
                *self.active = Some(JackSnapshotCloneKind::Port { source: self.index, property: 0, value: graph::manifest::TrinityPortKindDef { name: Self::clone_string(&kind.name)?, direction: kind.direction, properties } });
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn step(&mut self, source: &JackSnapshot, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if let Some(active) = self.active.as_mut() {
            let completed = match active {
                JackSnapshotCloneKind::Node { source: source_index, property, port, value } => {
                    let source = source.manifest.node_kinds.get(*source_index).ok_or("jack-store.initializer-node-kind-stale")?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition)?);
                        *property += 1;
                        false
                    } else if let Some(kind) = source.port_kinds.get(*port) {
                        value.port_kinds.push(Self::clone_string(kind)?);
                        *port += 1;
                        false
                    } else {
                        true
                    }
                }
                JackSnapshotCloneKind::Edge { source: source_index, property, value } => {
                    let source = source.manifest.edge_kinds.get(*source_index).ok_or("jack-store.initializer-edge-kind-stale")?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition)?);
                        *property += 1;
                        false
                    } else {
                        true
                    }
                }
                JackSnapshotCloneKind::Port { source: source_index, property, value } => {
                    let source = source.manifest.port_kinds.get(*source_index).ok_or("jack-store.initializer-port-kind-stale")?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition)?);
                        *property += 1;
                        false
                    } else {
                        true
                    }
                }
            };
            if completed {
                let active = self.active.take().expect("completed Jack kind clone remains exact");
                let target = self.value.as_mut().ok_or("jack-store.initializer-clone-target")?;
                match active {
                    JackSnapshotCloneKind::Node { value, .. } => target.manifest.node_kinds.push(value),
                    JackSnapshotCloneKind::Edge { value, .. } => target.manifest.edge_kinds.push(value),
                    JackSnapshotCloneKind::Port { value, .. } => target.manifest.port_kinds.push(value),
                }
                self.index += 1;
            }
            cx.consume_fuel(1);
            return Ok(false);
        }
        if self.begin_kind(source)? {
            cx.consume_fuel(1);
            return Ok(false);
        }
        let target = self.value.as_mut().ok_or("jack-store.initializer-clone-target")?;
        let observed = match self.phase {
            0 => {
                target.schema = Self::clone_string(&source.schema)?;
                source.schema.as_bytes()
            }
            1 => {
                target.name = Self::clone_string(&source.name)?;
                source.name.as_bytes()
            }
            2 => {
                target.manifest_id = source.manifest_id.as_deref().map(Self::clone_string).transpose()?;
                source.manifest_id.as_deref().unwrap_or_default().as_bytes()
            }
            3 => {
                target.camera = source.camera.clone();
                &[]
            }
            7 => {
                target.content.child_id = Self::clone_string(&source.content.child_id)?;
                source.content.child_id.as_bytes()
            }
            8 => {
                target.content.target.artifact_id = Self::clone_string(&source.content.target.artifact_id)?;
                source.content.target.artifact_id.as_bytes()
            }
            9 => {
                target.content.target.dialect.artifact_kind = Self::clone_string(&source.content.target.dialect.artifact_kind)?;
                source.content.target.dialect.artifact_kind.as_bytes()
            }
            10 => {
                target.content.target.dialect.standard = Self::clone_string(&source.content.target.dialect.standard)?;
                source.content.target.dialect.standard.as_bytes()
            }
            11 => {
                target.content.target.dialect.subset = Self::clone_string(&source.content.target.dialect.subset)?;
                source.content.target.dialect.subset.as_bytes()
            }
            12 => {
                target.root_node_id = source.root_node_id.as_deref().map(Self::clone_string).transpose()?;
                source.root_node_id.as_deref().unwrap_or_default().as_bytes()
            }
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        };
        digest.observe(observed);
        self.phase += 1;
        cx.consume_fuel(observed.len().max(1) as u64);
        Ok(false)
    }

    fn take_value(&mut self) -> Option<JackSnapshot> {
        if !self.terminal || self.active.is_some() {
            return None;
        }
        self.value.take()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(active) = self.active.take() {
                let owner = match active {
                    JackSnapshotCloneKind::Node { value, .. } => JackRetirementOwner::NodeKind(value),
                    JackSnapshotCloneKind::Edge { value, .. } => JackRetirementOwner::EdgeKind(value),
                    JackSnapshotCloneKind::Port { value, .. } => JackRetirementOwner::PortKind(value),
                };
                *self.retirement = Some(Box::new(JackOwnedRetirement::new(owner)));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Jack clone retirement remains exact");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            store::SnapshotRetirementStep::Complete => Err("Jack clone retirement reported false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.active.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Jack snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

pub fn jack_document_store_owners() -> store::MemberStoreOwners<JackSnapshot, TrinityGraphMutation> {
    store::MemberStoreOwners::new(
        std::sync::Arc::new(JackSnapshotRetirementFactory),
        std::sync::Arc::new(JackSnapshotRetirementFactory),
        std::sync::Arc::new(JackMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<JackSnapshot, TrinityGraphMutation>::new()),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JackStoreInitializationPhase {
    ValidateEnvelope,
    ValidateEditPair { left: usize, right: usize },
    CloneInitial,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FindApplied { position: usize, scan: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    HashInverse { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize, scan: usize },
    HashRedoForward { position: usize, edit: usize, mutation: usize },
    HashRedoInverse { position: usize, edit: usize, mutation: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct JackStoreInitializationAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<JackSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<JackSnapshot, TrinityGraphMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    clone: std::mem::ManuallyDrop<Option<JackSnapshotCloneAuthority>>,
    initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    phase: JackStoreInitializationPhase,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl JackStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self {
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(Some(JackSnapshotCloneAuthority::new())),
            initial_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"jack.initial"))),
            edit_digest: std::mem::ManuallyDrop::new(None),
            phase: JackStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
        }
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = JackStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self) -> Result<bool, String> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, JACK_OWNED_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= JACK_OWNED_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err("Jack store initializer retirement exceeded its exact grant".into()),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                drop(self.active.take());
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err("Jack store initializer retirement reported a false terminal".into()),
        }
    }

    fn pump_terminal_retirement(&mut self) -> Result<bool, String> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&JackSnapshotRetirementFactory, 1, JACK_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Jack initialization runtime reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if let Some(clone) = self.clone.as_mut() {
            match clone.close_step(1, JACK_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if clone.terminal_is_empty() => {
                    drop(self.clone.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Jack snapshot clone reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(jack_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, JACK_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err("Jack initialization envelope retirement reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.clone.is_none()
            && self.initial_digest.is_none()
            && self.edit_digest.is_none()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"jack-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::Cancelled) {
            self.phase = JackStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_bytes());
            self.phase = JackStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.phase {
            JackStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"jack-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::artifacts::jack::TRINITY_GRAPH_SCHEMA || envelope.id.is_empty() || envelope.id.len() > JACK_OWNED_FIELD_BYTES {
                    self.fail(b"jack-store.initializer-envelope-invalid");
                } else {
                    self.phase = JackStoreInitializationPhase::ValidateEditPair { left: 0, right: 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::ValidateEditPair { left, right } => {
                let envelope = self.envelope.as_ref().expect("validated Jack envelope remains retained");
                if left >= envelope.vcs.edits.len() {
                    self.phase = JackStoreInitializationPhase::CloneInitial;
                } else if right >= envelope.vcs.edits.len() {
                    self.phase = JackStoreInitializationPhase::ValidateEditPair { left: left + 1, right: left + 2 };
                } else if envelope.vcs.edits[left].id == envelope.vcs.edits[right].id || envelope.vcs.edits[left].id.len() > JACK_OWNED_FIELD_BYTES {
                    self.fail(b"jack-store.initializer-duplicate-or-hostile-edit");
                } else {
                    self.phase = JackStoreInitializationPhase::ValidateEditPair { left, right: right + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("Jack envelope remains retained during initial clone").vcs.initial_snapshot;
                let clone = self.clone.as_mut().expect("Jack initial clone authority remains retained");
                let complete = match clone.step(source, self.initial_digest.as_mut().expect("Jack initial digest remains retained"), cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("Jack initial snapshot was built one semantic field at a time");
                    drop(self.clone.take());
                    let initial_digest = self.initial_digest.take().expect("Jack initial digest remains retained").finish();
                    let envelope = self.envelope.as_ref().expect("Jack envelope remains retained during runtime construction");
                    *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, initial, initial_digest));
                    self.phase = JackStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = JackStoreInitializationPhase::FindApplied { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = JackStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_mutation(id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = JackStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp.clone());
                        self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = JackStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::FindApplied { position, scan } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Writer runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = JackStoreInitializationPhase::FindRedo { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"jack-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let id = edit.id.clone();
                    let sequence_number = edit.sequence_number;
                    let started_at = edit.started_at.clone();
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"jack.edit");
                    digest.observe(id.as_bytes());
                    digest.observe(&sequence_number.to_be_bytes());
                    digest.observe(started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = JackStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = JackStoreInitializationPhase::FindApplied { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack applied edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = JackStoreInitializationPhase::HashInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let encoded = match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= JACK_OWNED_FIELD_BYTES => encoded,
                    _ => {
                        self.fail(b"jack-store.initializer-forward-encoding");
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                self.edit_digest.as_mut().expect("Writer edit digest remains retained").observe(&encoded);
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Writer runtime current snapshot remains retained");
                let (diff, messages) = operation.diff(current).into_parts();
                if messages.iter().any(|message| message.level == protocol::Severity::Fatal) {
                    self.fail(b"jack-store.initializer-fatal-mutation");
                    return semio_framework_job::StepOutcome::Yield;
                }
                match diff.apply(current) {
                    Ok(next) => {
                        let previous = std::mem::replace(current, next);
                        *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, previous));
                        self.phase = JackStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    Err(error) => {
                        self.fault = Some(error.to_string().into_bytes());
                        self.phase = JackStoreInitializationPhase::RetireFault;
                    }
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack applied edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = JackStoreInitializationPhase::CommitApplied { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= JACK_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Writer edit digest remains retained").observe(&encoded);
                        self.phase = JackStoreInitializationPhase::HashInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"jack-store.initializer-inverse-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack applied edit remains retained");
                let id = entry.id.clone();
                let actor = entry.actor.clone();
                let digest = self.edit_digest.take().expect("Jack applied edit digest remains retained").finish();
                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained");
                if let Err(error) = runtime.push_applied(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = JackStoreInitializationPhase::RetireFault;
                } else {
                    runtime.set_local_actor_id(actor);
                    self.phase = JackStoreInitializationPhase::FindApplied { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::FindRedo { position, scan } => {
                let Some(id) = self.redo_id(position) else {
                    self.phase = JackStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"jack-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let id = edit.id.clone();
                    let sequence_number = edit.sequence_number;
                    let started_at = edit.started_at.clone();
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"jack.edit");
                    digest.observe(id.as_bytes());
                    digest.observe(&sequence_number.to_be_bytes());
                    digest.observe(started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = JackStoreInitializationPhase::HashRedoForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = JackStoreInitializationPhase::FindRedo { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::HashRedoForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack redo edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = JackStoreInitializationPhase::HashRedoInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= JACK_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Jack redo digest remains retained").observe(&encoded);
                        self.phase = JackStoreInitializationPhase::HashRedoForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"jack-store.initializer-redo-forward-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::HashRedoInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack redo edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = JackStoreInitializationPhase::CommitRedo { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= JACK_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Jack redo digest remains retained").observe(&encoded);
                        self.phase = JackStoreInitializationPhase::HashRedoInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"jack-store.initializer-redo-inverse-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack redo edit remains retained").id.clone();
                let digest = self.edit_digest.take().expect("Jack redo digest remains retained").finish();
                if let Err(error) = self.runtime.as_mut().expect("Writer runtime remains retained").push_redo(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = JackStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = JackStoreInitializationPhase::FindRedo { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            JackStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"jack-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("Jack envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Writer runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, jack_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = JackStoreInitializationPhase::Complete;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            }
            JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement() {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    drop(self.initial_digest.take());
                    drop(self.edit_digest.take());
                    self.terminal_handoff = true;
                    if self.phase == JackStoreInitializationPhase::RetireCancelled {
                        self.phase = JackStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = JackStoreInitializationPhase::Fault;
                        let fault = self.fault.take().unwrap_or_else(|| b"jack-store.initializer-fault".to_vec());
                        let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, &fault).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_bytes());
                    semio_framework_job::StepOutcome::Yield
                }
            },
            JackStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            JackStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            JackStoreInitializationPhase::Fault => {
                let fault = self.fault.as_deref().unwrap_or(b"jack-store.initializer-fault");
                let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, fault).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
            }
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, JackStoreInitializationPhase::Cancelled | JackStoreInitializationPhase::Fault) {
            self.phase = JackStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes < JACK_OWNED_FIELD_BYTES {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_terminal_retirement() {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                drop(self.initial_digest.take());
                drop(self.edit_digest.take());
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-close"), format!("Jack initializer close failed: {error}"))),
        }
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<JackSnapshot, TrinityGraphMutation>> {
        if self.phase != JackStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        drop(self.initial_digest.take());
        drop(self.edit_digest.take());
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for JackStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn jack_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<JackSnapshot, TrinityGraphMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(JackStoreInitializationAuthority::new(envelope, operation, generation)))
}

//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::TRINITY_GRAPH_SCHEMA;

    #[semio_framework_async_macros::async_test]
    async fn rename_op_binary_round_trips_and_agrees_with_text() {
        let operation = rename_node("node-1".into(), "Renamed".into());
        ::store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn nakagin_document_text_round_trips_store_with_applied_operation() {
        let envelope = create_document_envelope_for_test();
        let mut doc_store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![rename_node("node-1".into(), "Renamed".into())], description: None }).ok();
        ::store::os_store::test_support::assert_document_text_round_trip(&doc_store);
        ::store::os_store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    fn create_document_envelope_for_test() -> store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation> {
        create_document_envelope::<JackSnapshot, TrinityGraphMutation>(TRINITY_GRAPH_SCHEMA, "doc-text-test", crate::artifacts::jack::schema::empty_jack_document(), None)
    }
    use store::create_document_envelope;

    fn empty_jack_initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> JackStoreInitializationAuthority {
        let envelope = store::create_document_envelope(crate::artifacts::jack::TRINITY_GRAPH_SCHEMA, "jack-retained-load", crate::artifacts::jack::schema::empty_jack_document(), None);
        JackStoreInitializationAuthority::new(envelope, operation, generation)
    }

    fn drive_jack_initializer(authority: &mut JackStoreInitializationAuthority, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::StepOutcome {
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
            let outcome = semio_framework_plugin::ArtifactStoreInitializationAuthority::step(authority, &mut context);
            if outcome.is_terminal() {
                return outcome;
            }
        }
        panic!("Jack retained initializer did not reach a bounded terminal")
    }

    fn close_jack_candidate(mut candidate: store::ArtifactStore<JackSnapshot, TrinityGraphMutation>) {
        use semio_framework_plugin::ArtifactOwnedDisposer;

        let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<JackSnapshot, TrinityGraphMutation>::new();
        for _ in 0..100_000 {
            match disposer.close_step(&mut candidate, 1, JACK_OWNED_FIELD_BYTES).expect("Jack candidate close step") {
                semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= JACK_OWNED_FIELD_BYTES);
                }
                semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("fresh Jack candidate close unexpectedly blocked: {reason}"),
                semio_framework_plugin::PluginCloseStep::Complete => {
                    assert!(disposer.terminal_is_empty(&candidate));
                    drop(disposer);
                    drop(candidate);
                    return;
                }
            }
        }
        panic!("Jack candidate did not reach terminal-empty close")
    }

    #[test]
    fn jack_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally() {
        let operation = semio_framework_job::OperationId(501);
        let generation = semio_framework_job::Generation(13);
        let mut authority = empty_jack_initializer(operation, generation);
        assert!(matches!(drive_jack_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Complete(_)));
        let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact Jack candidate");
        assert_eq!(candidate.generation_now(), 14);
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
        drop(authority);
        close_jack_candidate(candidate);
    }

    #[test]
    fn jack_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty() {
        let operation = semio_framework_job::OperationId(502);
        let generation = semio_framework_job::Generation(15);
        let mut cancelled = empty_jack_initializer(operation, generation);
        semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut cancelled);
        assert!(matches!(drive_jack_initializer(&mut cancelled, operation, generation), semio_framework_job::StepOutcome::Cancelled));
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
        drop(cancelled);

        let mut stale = empty_jack_initializer(operation, generation);
        assert!(matches!(drive_jack_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1)), semio_framework_job::StepOutcome::Fault(_)));
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
        drop(stale);
    }

    #[test]
    fn jack_nested_mutation_and_child_snapshot_retire_one_exact_owner_per_grant() {
        let mut object = std::collections::BTreeMap::new();
        object.insert("nested".repeat(32), PropertyValue::Array(vec![PropertyValue::String("payload".repeat(128)), PropertyValue::String("tail".into())]));
        let mutation = TrinityGraphMutation::ChangeDataProperty(crate::artifacts::jack::mutations::ChangeDataProperty { entity: EntityRef::Node("node".repeat(64)), key: "key".repeat(64), new_value: PropertyValue::Object(object) });
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, mutation);
        for _ in 0..10_000 {
            let step = retirement.close_step(1, JACK_OWNED_FIELD_BYTES).expect("one nested Jack owner retires");
            match step {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= JACK_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return;
                }
                store::SnapshotRetirementStep::Blocked => panic!("owned Jack mutation retirement cannot block"),
            }
        }
        panic!("nested Jack mutation retirement did not reach terminal")
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_op_text_round_trips() {
        ::store::os_store::test_support::assert_op_line_round_trip(&rename_node("node-1".into(), "Renamed".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_create_node() {
        ::store::os_store::test_support::assert_op_line_round_trip(&create_node(Node {
            id: "new".into(),
            kind: "Piece".into(),
            name: "new-piece".into(),
            x: 200.0,
            y: 40.0,
            width: 80.0,
            height: 40.0,
            properties: crate::artifacts::jack::PropertyBag::new(),
            ports: vec![crate::artifacts::jack::Port { id: "p1".into(), kind: "Connector".into(), direction: crate::artifacts::jack::PortDirection::Out, properties: crate::artifacts::jack::PropertyBag::new() }],
        }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_delete_node() {
        ::store::os_store::test_support::assert_op_line_round_trip(&delete_node("root".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_create_edge() {
        let mut properties = crate::artifacts::jack::PropertyBag::new();
        properties.insert("u".into(), PropertyValue::Number(1.2));
        let mut nested = std::collections::BTreeMap::new();
        nested.insert("x".into(), PropertyValue::Number(0.0));
        properties.insert("meta".into(), PropertyValue::Object(nested));
        ::store::os_store::test_support::assert_op_line_round_trip(&create_edge(Edge {
            id: "e2".into(),
            kind: "Connection".into(),
            source: crate::artifacts::jack::port_key("root", "out-a"),
            target: crate::artifacts::jack::port_key("child", "in-a"),
            properties,
        }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_delete_edge() {
        ::store::os_store::test_support::assert_op_line_round_trip(&delete_edge("e1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_rename_node() {
        ::store::os_store::test_support::assert_op_line_round_trip(&rename_node("root".into(), "renamed \"piece\"".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_move_node() {
        ::store::os_store::test_support::assert_op_line_round_trip(&move_node("root".into(), 10.0, -20.5));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_change_data_property() {
        ::store::os_store::test_support::assert_op_line_round_trip(&change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::String("hi 'there'".into())));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_remove_data_property() {
        ::store::os_store::test_support::assert_op_line_round_trip(&remove_data_property(EntityRef::Edge("e1".into()), "u".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_op_rejects_unknown_keyword() {
        let err = TrinityGraphMutation::parse_op("bogusOp x").expect_err("unknown op");
        assert!(err.message.contains("unknown mutation line"));
    }

    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::jack::schema::mutations::text::TrinityGraphStore;
        use protocol::{ArtifactId, Edit, SchemaId};

        let mut store = TrinityGraphStore::new(create_document_envelope_for_test());
        crate::artifacts::jack::schema::mutations::text::dispatch_trinity_graph_mutations(&mut store, vec![rename_node("node-1".into(), "Renamed".into())]).unwrap_or(());
        if let Some(edit) = store.envelope().vcs.edits.last() {
            let edit: &Edit<TrinityGraphMutation> = edit;
            ::store::os_store::test_support::assert_command_envelope_round_trip::<JackSnapshot, TrinityGraphMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
        }
    }
}
//#endregion 🧪️Tests
