//! 🧪️ The Semio graph owner of a Jack content child is built by its factory, replaced by materialization and read back without a copy.
use super::*;
use jack::standards::v1::subsets::any::schema::content::{jack_content_for_handle, retire_owned_to_terminal, JackContentFactory, JackContentOwner};
use jack::standards::v1::subsets::any::schema::content_read::JackContentProjection;
use jack::standards::v1::subsets::any::schema::graph::{EntityRef, GraphEffect};

fn node(id: &str) -> Node {
    Node { id: id.into(), kind: "k".into(), name: id.into(), x: 1.0, y: 2.0, width: 3.0, height: 4.0, ports: Vec::new(), properties: PropertyBag::new() }
}

fn edge(id: &str, source: &str, target: &str) -> Edge {
    Edge { id: id.into(), kind: "e".into(), source: source.into(), target: target.into(), properties: PropertyBag::new() }
}

#[test]
fn the_factory_child_reads_back_the_working_scene() {
    let mut child = <SemioJackContent as JackContentFactory>::child_from_working(vec![node("a"), node("b")], vec![edge("e", "a", "b")]);
    let owner = jack_content_for_handle(&child).expect("factory child admits its owner");
    assert_eq!(owner.snapshot().nodes.len(), 2);
    assert_eq!(owner.snapshot().edges.len(), 1);
    assert_eq!(owner.read().projection_counts(), (2, 1));
    drop(owner);
    let source = child.take_local_owner::<JackContentOwner>().expect("typed content").expect("retained content");
    let source = Arc::try_unwrap(source).ok().expect("last local content owner");
    retire_owned_to_terminal(source).expect("typed content owner retires");
    retire_owned_to_terminal(child).expect("factory child retires");
}

#[test]
fn materialization_replaces_the_retained_graph_under_the_same_address() {
    let mut child = <SemioJackContent as JackContentFactory>::child_from_working(vec![node("a")], Vec::new());
    let address = child.child_id.clone();
    materialize_jack_snapshot(&mut child, jack_content_snapshot_from_working(&[node("a"), node("b"), node("c")], &[edge("e", "a", "c")]));
    assert_eq!(child.child_id, address);
    let owner = jack_content_for_handle(&child).expect("materialized child admits its owner");
    let ids: Vec<&str> = owner.snapshot().nodes.iter().map(|node| node.id.value.as_str()).collect();
    assert_eq!(ids, ["a", "b", "c"]);
    drop(owner);
    retire_owned_to_terminal(child).expect("materialized child retires");
}

#[test]
fn the_working_scene_projects_back_losslessly() {
    let mut a = node("a");
    a.ports.push(Port { id: "out@detail".into(), kind: "k".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
    let nodes = vec![a, node("b")];
    let edges = vec![edge("e", "a@out@detail", "b")];
    let snapshot = jack_content_snapshot_from_working(&nodes, &edges);
    let (projected_nodes, projected_edges) = working_from_jack_content_snapshot(&snapshot).expect("projection");
    assert_eq!(projected_nodes, nodes);
    assert_eq!(projected_edges, edges);
    assert_eq!(project_edge(&snapshot, 0, 4096).expect("bounded edge projection").as_ref(), edges.first());
}

#[test]
fn the_owner_refuses_a_batch_with_a_duplicate_created_node() {
    let child = jack_content_child_with_owner(vec![node("a")], Vec::new());
    let owner = jack_content_for_handle(&child).expect("retained owner");
    let before = owner.read().genesis_pack().expect("original graph pack");
    let effects = [GraphEffect::CreateNode(node("b")), GraphEffect::CreateNode(node("b"))];

    let Err(error) = owner.read().prepare_effects(&effects) else {
        panic!("a duplicate created node must refuse the whole batch");
    };
    assert!(error.message.contains("mutation.duplicate-id"));
    assert_eq!(owner.read().genesis_pack().expect("unchanged graph pack"), before);

    drop(owner);
    retire_owned_to_terminal(child).expect("factory child retires");
}

#[test]
fn dependent_effects_invert_to_the_exact_original_child() {
    use protocol::{ColdMutation, OpBinary};
    use pack::snapshot::ArtifactPack;

    let original = jack_content_snapshot_from_working(&[node("a"), node("b")], &[edge("original", "a", "b")]);
    let before = SemioGraphSnapshot::encode_pack(&original);
    let effects = [
        GraphEffect::SetProperty { entity: EntityRef::Node("b".into()), key: "score".into(), value: PropertyValue::Number(1.0) },
        GraphEffect::SetProperty { entity: EntityRef::Node("b".into()), key: "score".into(), value: PropertyValue::Number(2.0) },
        GraphEffect::SetProperty { entity: EntityRef::Node("b".into()), key: "score".into(), value: PropertyValue::Number(2.0) },
        GraphEffect::RemoveProperty { entity: EntityRef::Edge("original".into()), key: "absent".into() },
        GraphEffect::DeleteNode("a".into()),
    ];
    let leaves = graph_leaves(&original, &effects).expect("admitted owner effects");
    assert!(matches!(leaves.as_slice(), [
        SemioGraphMutation::AddNodeProperty(_),
        SemioGraphMutation::SetNodeProperty(_),
        SemioGraphMutation::DeleteEdge(_),
        SemioGraphMutation::DeleteNode(_),
    ]));

    let mut work = original;
    let mut inverses = Vec::new();
    for leaf in leaves {
        let wire = leaf.encode_op().expect("child operation wire");
        let leaf = SemioGraphMutation::decode_op(&wire).expect("decoded child operation");
        inverses.push(leaf.inverse(&work).expect("point inverse"));
        protocol::apply_diff_cold_into(leaf.diff(&work).diff(), &mut work).expect("operation applies");
    }
    assert_eq!(work.nodes.len(), 1);
    assert_eq!(work.nodes[0].id.value, "b");
    assert_eq!(work.nodes[0].properties[0].value, SemioValue::Float { lexeme: binary64_lexeme(2.0) });
    assert!(work.edges.is_empty());

    for inverse in inverses.into_iter().rev().flatten() {
        protocol::apply_diff_cold_into(inverse.diff(&work).diff(), &mut work).expect("inverse applies");
    }
    assert_eq!(SemioGraphSnapshot::encode_pack(&work), before);
    retire_owned_to_terminal(work).expect("replayed graph retires");
}
