//! 🪆️ Jack retains its actual rich Semio child; query projections carry checked six-family values.
use super::{Node,Edge,Port,PortDirection,PropertyBag,PropertyValue,JackSnapshot};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2,binary64_lexeme,read_binary64_lexeme};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId,GraphNodeId,SemioGraphEdge,SemioGraphNode,SemioGraphPort,SemioGraphPortKind,SemioGraphSnapshot,STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue,SemioValueEntry};
use std::sync::Arc;

pub type JackContentChild=store::ArtifactChild<SemioGraphSnapshot>;

pub struct JackContentOwner { snapshot:Option<SemioGraphSnapshot> }
impl JackContentOwner {
 pub fn new(snapshot:SemioGraphSnapshot)->Self{Self{snapshot:Some(snapshot)}}
 pub fn snapshot(&self)->&SemioGraphSnapshot{self.snapshot.as_ref().expect("live Jack Semio child")}
}
/// 🪆️ The shared content owner retires its graph through the graph's own deferred fields.
impl RetireOwned for JackContentOwner {
 fn retirement(self)->Box<dyn RetirementCursor>{semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot)])}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot)])}
 fn controlled_retirement_supported()->bool{true}
}

fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

/// 🧮️ The copy-byte body every self-funded Jack retirement turn quotes against.
pub const JACK_SELF_FUNDED_BODY_BYTES: usize = 65_536;

/// 🏇️ The mounted-owner policy every Trinity component declares for its preparation, maintenance and close lanes.
pub fn trinity_mounted_owner_policy() -> semio_framework_plugin::MountedOwnerPolicyV1 {
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
    semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant }
}

/// 🎟️ Funds exactly one turn from the owner's own published quote, for callers no scheduler grants (standalone stores, fixtures and local scratch owners).
pub fn jack_self_funded_grant(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(JACK_SELF_FUNDED_BODY_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// ♻️ Drives an admitted retirement cursor through demand-quoted retained turns until its terminal-empty witness.
pub fn drive_retirement_to_terminal(mut owner: Box<dyn semio_framework_value::ErasedSnapshotRetirement>) -> Result<(), ValueError> {
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    const IDLE_TURNS: usize = 4;
    let mut idle = 0;
    while !owner.terminal_is_empty() {
        let demand = owner.next_demand(JACK_SELF_FUNDED_BODY_BYTES)?;
        let progress = owner.close_step(jack_self_funded_grant(demand))?.progress();
        idle = if progress == RetainedCloneProgress::default() { idle + 1 } else { 0 };
        if idle > IDLE_TURNS {
            return Err(invalid("Jack retirement stalled under its own demand quote"));
        }
    }
    Ok(())
}

/// ♻️ Admits an owned value's retirement frame under its own quote, then drives it to the terminal-empty witness.
pub fn retire_owned_to_terminal<T: RetireOwned>(value: T) -> Result<(), ValueError> {
    let birth = jack_self_funded_grant(semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), ..Default::default() });
    let (owner, _) = semio_framework_value::retirement::admit_owned_retirement(value, birth).map_err(|(error, _)| error)?;
    drive_retirement_to_terminal(owner)
}
fn property_to_semio(value:&PropertyValue)->SemioValue{match value{
 PropertyValue::Null=>SemioValue::Null,
 PropertyValue::Bool(value)=>SemioValue::Bool{value:*value},
 PropertyValue::Number(value)=>SemioValue::Float{lexeme:binary64_lexeme(*value)},
 PropertyValue::String(value)=>SemioValue::Str{value:value.clone()},
 PropertyValue::Array(values)=>SemioValue::List{items:values.iter().map(property_to_semio).collect()},
 PropertyValue::Object(values)=>SemioValue::Map{entries:properties_to_semio(values)},
}}
fn properties_to_semio(values:&PropertyBag)->Vec<SemioValueEntry>{values.iter().map(|(key,value)|SemioValueEntry{key:key.clone(),value:property_to_semio(value)}).collect()}
fn property_from_semio(value:&SemioValue)->Result<PropertyValue,ValueError>{Ok(match value{
 SemioValue::Null=>PropertyValue::Null,
 SemioValue::Bool{value}=>PropertyValue::Bool(*value),
 SemioValue::Float{lexeme}=>PropertyValue::Number(read_binary64_lexeme(lexeme)?),
 SemioValue::Str{value}=>PropertyValue::String(value.clone()),
 SemioValue::List{items}=>PropertyValue::Array(items.iter().map(property_from_semio).collect::<Result<_,_>>()?),
 SemioValue::Map{entries}=>PropertyValue::Object(properties_from_semio(entries)?),
 SemioValue::Int{..}|SemioValue::Bytes{..}|SemioValue::Ref{..}=>return Err(invalid("Jack query projection cannot represent this intrinsic Semio value family")),
})}
fn properties_from_semio(values:&[SemioValueEntry])->Result<PropertyBag,ValueError>{let mut result=PropertyBag::new();for entry in values{if result.contains_key(&entry.key){return Err(invalid("Jack query projection requires unique property keys"))}result.insert(entry.key.clone(),property_from_semio(&entry.value)?);}Ok(result)}
fn endpoint(node:&str,port:Option<&str>)->Result<String,ValueError>{if node.contains('@')||port.is_some_and(|port|port.contains('@')){return Err(invalid("Jack query endpoint cannot represent a literal separator in an endpoint component"))}Ok(match port{Some(port)=>format!("{node}@{port}"),None=>node.to_string()})}

pub fn jack_content_snapshot_from_working(nodes:&[Node],edges:&[Edge])->SemioGraphSnapshot{
 SemioGraphSnapshot{schema:STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(),nodes:nodes.iter().map(|node|SemioGraphNode{id:GraphNodeId::new(node.id.clone()),kind:node.kind.clone(),label:node.name.clone(),position:SemioPoint2{x:node.x,y:node.y},width:node.width,height:node.height,ports:node.ports.iter().map(|port|SemioGraphPort{name:port.id.clone(),kind:match port.direction{PortDirection::In=>SemioGraphPortKind::In,PortDirection::Out=>SemioGraphPortKind::Out},category:port.kind.clone(),properties:properties_to_semio(&port.properties)}).collect(),properties:properties_to_semio(&node.properties)}).collect(),edges:edges.iter().map(|edge|{let(source,source_port)=edge.source.split_once('@').map_or((edge.source.as_str(),None),|(node,port)|(node,Some(port.to_string())));let(target,target_port)=edge.target.split_once('@').map_or((edge.target.as_str(),None),|(node,port)|(node,Some(port.to_string())));SemioGraphEdge{id:GraphEdgeId::new(edge.id.clone()),source:GraphNodeId::new(source),target:GraphNodeId::new(target),kind:edge.kind.clone(),label:String::new(),source_port,target_port,properties:properties_to_semio(&edge.properties)}}).collect()}
}

pub fn working_from_jack_content_snapshot(content:&SemioGraphSnapshot)->Result<(Vec<Node>,Vec<Edge>),ValueError>{
 let nodes=content.nodes.iter().map(jack_node_from_content).collect::<Result<_,ValueError>>()?;
 let edges=content.edges.iter().map(jack_edge_from_content).collect::<Result<_,ValueError>>()?;Ok((nodes,edges))
}

/// 🔎️ Projects one declared query node from the sole retained Semio child.
pub fn jack_node_from_content(node:&SemioGraphNode)->Result<Node,ValueError>{let ports=node.ports.iter().map(|port|Ok(Port{id:port.name.clone(),kind:port.category.clone(),direction:match port.kind{SemioGraphPortKind::In=>PortDirection::In,SemioGraphPortKind::Out=>PortDirection::Out,SemioGraphPortKind::InOut=>return Err(invalid("Jack query projection requires a single port direction"))},properties:properties_from_semio(&port.properties)?})).collect::<Result<_,ValueError>>()?;Ok(Node{id:node.id.value.clone(),kind:node.kind.clone(),name:node.label.clone(),x:node.position.x,y:node.position.y,width:node.width,height:node.height,ports,properties:properties_from_semio(&node.properties)?})}

/// 🔗️ Projects one representable query edge without copying the retained child graph.
pub fn jack_edge_from_content(edge:&SemioGraphEdge)->Result<Edge,ValueError>{if !edge.label.is_empty(){return Err(invalid("Jack query projection cannot represent an intrinsic edge label"))}Ok(Edge{id:edge.id.value.clone(),kind:edge.kind.clone(),source:endpoint(&edge.source.value,edge.source_port.as_deref())?,target:endpoint(&edge.target.value,edge.target_port.as_deref())?,properties:properties_from_semio(&edge.properties)?})}

pub fn jack_content_handle(snapshot:&SemioGraphSnapshot)->JackContentChild{use store::ArtifactPack;let bytes=SemioGraphSnapshot::encode_pack(snapshot);let child_id=store::content_id("jack-content",&bytes);let target=semio_framework_artifact_reference::ArtifactRef{artifact_id:child_id.clone(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"graph".into()}};store::ArtifactChild::new(child_id,target)}
pub fn jack_content_child_handle(nodes:&[Node],edges:&[Edge])->JackContentChild{jack_content_handle(&jack_content_snapshot_from_working(nodes,edges))}
pub fn jack_content_child_with_snapshot(snapshot:SemioGraphSnapshot)->JackContentChild{jack_content_handle(&snapshot).with_local_owner(Arc::new(JackContentOwner::new(snapshot)))}
pub fn jack_content_child_with_owner(nodes:Vec<Node>,edges:Vec<Edge>)->JackContentChild{jack_content_child_with_snapshot(jack_content_snapshot_from_working(&nodes,&edges))}
pub fn materialize_jack_snapshot(handle:&mut JackContentChild,snapshot:SemioGraphSnapshot){handle.set_local_owner(Arc::new(JackContentOwner::new(snapshot)))}
pub fn materialize_jack_content(handle:&mut JackContentChild,nodes:Vec<Node>,edges:Vec<Edge>){materialize_jack_snapshot(handle,jack_content_snapshot_from_working(&nodes,&edges))}
pub fn jack_content_for_handle(handle:&JackContentChild)->Result<Arc<JackContentOwner>,ValueError>{let dialect=&handle.target.dialect;if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||dialect.subset!="graph"{return Err(invalid("Jack content requires the exact Semio v1 graph child dialect"))}handle.require_local_owner::<JackContentOwner>().map_err(|error|invalid(error.to_string()))}

#[derive(Clone,Debug)]
pub struct JackWorkingScene {pub nodes:Vec<Node>,pub edges:Vec<Edge>}
pub fn jack_working_scene_for_handle(handle:&JackContentChild)->Result<JackWorkingScene,ValueError>{let content=jack_content_for_handle(handle)?;let(nodes,edges)=working_from_jack_content_snapshot(content.snapshot())?;Ok(JackWorkingScene{nodes,edges})}
pub fn jack_working_scene(snapshot:&JackSnapshot)->Result<JackWorkingScene,ValueError>{jack_working_scene_for_handle(&snapshot.content)}
pub fn genesis_jack_child_pack(snapshot:&JackSnapshot,slot:&str,child_id:&str)->Result<Option<Vec<u8>>,ValueError>{use store::ArtifactPack;if slot!="content"||child_id!=snapshot.content.child_id{return Ok(None)}let owner=jack_content_for_handle(&snapshot.content)?;Ok(Some(SemioGraphSnapshot::encode_pack(owner.snapshot())))}

/// 📎️ The content child that ships with the plugin under `child_id`: the empty graph every fresh document starts from.
fn bundled_jack_content(child_id: &str) -> Option<SemioGraphSnapshot> {
    static EMPTY_CHILD_ID: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| jack_content_handle(&jack_content_snapshot_from_working(&[], &[])).child_id);
    (child_id == EMPTY_CHILD_ID.as_str()).then(|| jack_content_snapshot_from_working(&[], &[]))
}

/// 🧲️ Gives a freshly decoded document the content child its handle names when that child ships with the plugin; every
/// other handle stays an address the host's composed boundary materializes (design §20.15).
pub fn attach_bundled_content(snapshot: &mut JackSnapshot) {
    if snapshot.content.local_owner::<JackContentOwner>().is_none() {
        if let Some(content) = bundled_jack_content(&snapshot.content.child_id) {
            materialize_jack_snapshot(&mut snapshot.content, content);
        }
    }
}

//#region 🔖️ChildLane
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::{
    add_edge_property::AddEdgeProperty, add_node_property::AddNodeProperty, change_node_label::ChangeNodeLabel, create_edge::CreateEdge, create_node::CreateNode, delete_edge::DeleteEdge, delete_node::DeleteNode, move_node::MoveNode,
    remove_edge_property::RemoveEdgeProperty, remove_node_property::RemoveNodeProperty, set_edge_property::SetEdgeProperty, set_node_property::SetNodeProperty, SemioGraphMutation,
};

/// 🚫️ A named refusal of the composed content child; `jack_fault_notices` localizes every code.
fn content_fault(code: &'static str, detail: impl Into<String>) -> semio_framework_plugin::Fault {
    semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), detail.into())
}

/// 📢️ The localized notices of jack's own refusal codes (design §20.12).
pub fn jack_fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, semio_framework_ui_locale::LocalizedLabel); 5]> = std::sync::LazyLock::new(|| {
        [
            ("trinity.jack.content-missing", semio_framework_ui_locale::LocalizedLabel::native("The component graph of this Jack document is not loaded.", "Der Bauteilgraph dieses Jack-Dokuments ist nicht geladen.")),
            ("trinity.jack.content-dialect", semio_framework_ui_locale::LocalizedLabel::native("The component graph of this Jack document is not a Semio graph.", "Der Bauteilgraph dieses Jack-Dokuments ist kein Semio-Graph.")),
            ("trinity.jack.content-unreadable", semio_framework_ui_locale::LocalizedLabel::native("The component graph holds a value Jack cannot read.", "Der Bauteilgraph enthält einen Wert, den Jack nicht lesen kann.")),
            ("trinity.jack.layout-run.start", semio_framework_ui_locale::LocalizedLabel::native("The reorganize run could not start.", "Der Neuanordnungslauf konnte nicht starten.")),
            ("trinity.jack.retained-capacity", semio_framework_ui_locale::LocalizedLabel::native("This edit is too large to apply in one step.", "Diese Änderung ist zu groß, um sie in einem Schritt anzuwenden.")),
        ]
    });
    NOTICES.as_slice()
}

/// 🧸️ The document's exact published `content` child, read through its captured member-store view: the child store is the
/// scene's single source of truth (design §20.15), never the parent's genesis owner.
pub fn jack_content_from_children<'a>(snapshot: &JackSnapshot, children: &'a semio_framework_plugin::app::ChildContentView) -> Result<store::SnapshotReadRef<'a, SemioGraphSnapshot>, semio_framework_plugin::Fault> {
    let child_id = &snapshot.content.child_id;
    let dialect = children.dialect("content", child_id).ok_or_else(|| content_fault("trinity.jack.content-missing", format!("the content child {child_id} is not open")))?;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "graph" {
        return Err(content_fault("trinity.jack.content-dialect", format!("the content child {child_id} is {}@{}/{}", dialect.artifact_kind, dialect.standard, dialect.subset)));
    }
    children.typed_read::<SemioGraphSnapshot>("content", child_id)
}

/// 🧸️ The working scene of the document's published `content` child.
pub fn jack_scene_from_children(snapshot: &JackSnapshot, children: &semio_framework_plugin::app::ChildContentView) -> Result<JackWorkingScene, semio_framework_plugin::Fault> {
    let content = jack_content_from_children(snapshot, children)?;
    let (nodes, edges) = working_from_jack_content_snapshot(&content).map_err(|error| content_fault("trinity.jack.content-unreadable", error.into_message()))?;
    Ok(JackWorkingScene { nodes, edges })
}

/// 🧬️ Publishes child `leaves` as ONE edit of the exact composed `content` child; no leaf is the empty emit.
pub fn jack_child_emit<C, D>(snapshot: &JackSnapshot, leaves: Vec<SemioGraphMutation>) -> semio_framework_plugin::Emit<crate::TrinityGraphMutation, C, D> {
    if leaves.is_empty() {
        return semio_framework_plugin::Emit::default();
    }
    semio_framework_plugin::Emit { child_preparations: std::collections::VecDeque::from([semio_framework_plugin::app::ChildEmitPreparation::of::<SemioGraphSnapshot, _>("content", &snapshot.content.child_id, leaves)]), ..Default::default() }
}

/// 🔌️ Splits a `node@port` endpoint into the graph's node and port fields.
fn split_endpoint(endpoint: &str) -> (GraphNodeId, Option<String>) {
    endpoint.split_once('@').map_or((GraphNodeId::new(endpoint), None), |(node, port)| (GraphNodeId::new(node), Some(port.to_string())))
}

/// 🏷️ The sorted insertion index of `key` in a property list kept in jack's key order.
fn property_index(entries: &[SemioValueEntry], key: &str) -> usize {
    entries.iter().position(|entry| entry.key.as_str() > key).unwrap_or(entries.len())
}

/// 🧮️ The content child's graph leaves that apply query `effects` to `base`, each against the state the earlier ones left:
/// a node delete first cuts every incident edge as its own row (every row stays point-invertible), a property set on an
/// existing key is `set-*-property`, a new key is `add-*-property` at its sorted index.
pub fn graph_leaves(base: &SemioGraphSnapshot, effects: &[crate::GraphEffect]) -> Vec<SemioGraphMutation> {
    use crate::GraphEffect;
    let mut work = base.clone();
    let mut leaves = Vec::with_capacity(effects.len());
    let push = |work: &mut SemioGraphSnapshot, leaf: SemioGraphMutation, leaves: &mut Vec<SemioGraphMutation>| {
        let outcome = <SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::diff(&leaf, work);
        if let Ok(next) = protocol::apply_diff(outcome.diff(), work) {
            *work = next;
        }
        leaves.push(leaf);
    };
    for effect in effects {
        match effect {
            GraphEffect::CreateNode(node) => {
                let content = jack_content_snapshot_from_working(std::slice::from_ref(node), &[]).nodes.remove(0);
                let SemioGraphNode { id, kind, label, position, width, height, ports, properties } = content;
                push(&mut work, SemioGraphMutation::CreateNode(CreateNode { id, kind, label, position, width, height, ports, properties, at: None }), &mut leaves);
            }
            GraphEffect::DeleteNode(id) => {
                let incident: Vec<GraphEdgeId> = work.edges.iter().filter(|edge| edge.source.value == *id || edge.target.value == *id).map(|edge| edge.id.clone()).collect();
                for edge in incident {
                    push(&mut work, SemioGraphMutation::DeleteEdge(DeleteEdge { id: edge }), &mut leaves);
                }
                push(&mut work, SemioGraphMutation::DeleteNode(DeleteNode { id: GraphNodeId::new(id.clone()) }), &mut leaves);
            }
            GraphEffect::CreateEdge(edge) => {
                let (source, source_port) = split_endpoint(&edge.source);
                let (target, target_port) = split_endpoint(&edge.target);
                push(&mut work, SemioGraphMutation::CreateEdge(CreateEdge { id: GraphEdgeId::new(edge.id.clone()), source, target, kind: edge.kind.clone(), label: String::new(), source_port, target_port, properties: properties_to_semio(&edge.properties), at: None }), &mut leaves);
            }
            GraphEffect::DeleteEdge(id) => push(&mut work, SemioGraphMutation::DeleteEdge(DeleteEdge { id: GraphEdgeId::new(id.clone()) }), &mut leaves),
            GraphEffect::RenameNode { id, name } => push(&mut work, SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: GraphNodeId::new(id.clone()), new_label: name.clone() }), &mut leaves),
            GraphEffect::MoveNode { id, x, y } => push(&mut work, SemioGraphMutation::MoveNode(MoveNode { id: GraphNodeId::new(id.clone()), new_position: SemioPoint2 { x: *x, y: *y } }), &mut leaves),
            GraphEffect::SetProperty { entity: crate::EntityRef::Node(id), key, value } => {
                let existing = work.nodes.iter().find(|node| node.id.value == *id).map(|node| (node.properties.iter().any(|entry| entry.key == *key), property_index(&node.properties, key)));
                let leaf = match existing {
                    Some((true, _)) => SemioGraphMutation::SetNodeProperty(SetNodeProperty { node_id: GraphNodeId::new(id.clone()), key: key.clone(), value: property_to_semio(value) }),
                    other => SemioGraphMutation::AddNodeProperty(AddNodeProperty { node_id: GraphNodeId::new(id.clone()), index: other.map_or(0, |(_, index)| index), property: SemioValueEntry { key: key.clone(), value: property_to_semio(value) } }),
                };
                push(&mut work, leaf, &mut leaves);
            }
            GraphEffect::SetProperty { entity: crate::EntityRef::Edge(id), key, value } => {
                let existing = work.edges.iter().find(|edge| edge.id.value == *id).map(|edge| (edge.properties.iter().any(|entry| entry.key == *key), property_index(&edge.properties, key)));
                let leaf = match existing {
                    Some((true, _)) => SemioGraphMutation::SetEdgeProperty(SetEdgeProperty { edge_id: GraphEdgeId::new(id.clone()), key: key.clone(), value: property_to_semio(value) }),
                    other => SemioGraphMutation::AddEdgeProperty(AddEdgeProperty { edge_id: GraphEdgeId::new(id.clone()), index: other.map_or(0, |(_, index)| index), property: SemioValueEntry { key: key.clone(), value: property_to_semio(value) } }),
                };
                push(&mut work, leaf, &mut leaves);
            }
            GraphEffect::RemoveProperty { entity: crate::EntityRef::Node(id), key } => push(&mut work, SemioGraphMutation::RemoveNodeProperty(RemoveNodeProperty { node_id: GraphNodeId::new(id.clone()), key: key.clone() }), &mut leaves),
            GraphEffect::RemoveProperty { entity: crate::EntityRef::Edge(id), key } => push(&mut work, SemioGraphMutation::RemoveEdgeProperty(RemoveEdgeProperty { edge_id: GraphEdgeId::new(id.clone()), key: key.clone() }), &mut leaves),
        }
    }
    retire_owned_to_terminal(work).expect("valid Jack working child retirement");
    leaves
}
//#endregion 🔖️ChildLane
