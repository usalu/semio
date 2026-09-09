
use super::*;

fn sample_node(id: &str) -> DagNodeSpec {
    DagNodeSpec { id: id.into(), name: id.into(), ..Default::default() }
}

fn round_trip(document: &DagSnapshot, operation: &DagMutation) -> DagSnapshot {
    let forward = operation.diff(document).diff().apply(document).expect("valid DAG diff");
    let mut restored = forward.clone();
    for back in operation.inverse(document).into_iter().rev() {
        restored = back.diff(&restored).diff().apply(&restored).expect("valid inverse DAG diff");
    }
    assert_eq!(&restored, document, "inverse() must exactly restore the pre-operation document");
    forward
}

#[semio_framework_async_macros::async_test]
async fn dag_document_vcs_replays_node_operations() {
    let mut store = create_dag_store("dag", empty_dag_document()).await.expect("store");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 })], description: None }).await.expect("apply");
    assert_eq!(store.snapshot().expect("projection").nodes.len(), 1);
    close_dag_test_store(store);
}

#[test]
fn node_create_move_resize_delete_round_trip() {
    let document = empty_dag_document();
    let added = round_trip(&document, &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
    assert_eq!(added.nodes.len(), 1);
    let moved = round_trip(&added, &DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 42.0, y: 7.0 }));
    assert_eq!(moved.nodes[0].x, 42.0);
    assert_eq!(moved.nodes[0].y, 7.0);
    let resized = round_trip(&moved, &DagMutation::ResizeNode(ResizeNode { id: "n1".into(), width: 200.0, height: 90.0 }));
    assert_eq!(resized.nodes[0].width, 200.0);
    assert_eq!(resized.nodes[0].height, 90.0);
    let removed = round_trip(&resized, &DagMutation::DeleteNode(DeleteNode { id: "n1".into() }));
    assert!(removed.nodes.is_empty());
}

#[test]
fn node_scalar_field_changes_round_trip() {
    let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
    let renamed_label = round_trip(&document, &DagMutation::ChangeNodeName(ChangeNodeName { id: "n1".into(), new_name: "Renamed label".into() }));
    assert_eq!(renamed_label.nodes[0].name, "Renamed label");
    let iconed = round_trip(&renamed_label, &DagMutation::ChangeNodeIcon(ChangeNodeIcon { id: "n1".into(), new_icon: "emoji:🧪️".into() }));
    assert_eq!(iconed.nodes[0].icon, "emoji:🧪️");
    let abbreviated = round_trip(&iconed, &DagMutation::ChangeNodeAbbreviation(ChangeNodeAbbreviation { id: "n1".into(), new_abbreviation: "N1".into() }));
    assert_eq!(abbreviated.nodes[0].abbreviation, "N1");
    let with_operator = round_trip(&abbreviated, &DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id: "n1".into(), new_operator_kind: Some("math.add".into()) }));
    assert_eq!(with_operator.nodes[0].operator_kind.as_deref(), Some("math.add"));
    let without_operator = round_trip(&with_operator, &DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id: "n1".into(), new_operator_kind: None }));
    assert_eq!(without_operator.nodes[0].operator_kind, None);
}

#[test]
fn replace_node_kind_and_properties_round_trip() {
    let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
    let new_kind = DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec::simple("out", "value") };
    let replaced_kind = round_trip(&document, &DagMutation::ReplaceNodeKind(ReplaceNodeKind { id: "n1".into(), new_kind: new_kind.clone() }));
    assert_eq!(replaced_kind.nodes[0].kind, new_kind);
    let new_properties = PropertyBag::from([("weight".to_string(), PropertyValue::Number(3.0))]);
    let replaced_properties = round_trip(&replaced_kind, &DagMutation::ReplaceNodeProperties(ReplaceNodeProperties { id: "n1".into(), new_properties: new_properties.clone() }));
    assert_eq!(replaced_properties.nodes[0].properties, new_properties);
}

#[test]
fn rename_node_cascades_edge_endpoints() {
    let mut document = empty_dag_document();
    document.nodes = vec![sample_node("a"), sample_node("b")];
    document.edges = vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }];
    let renamed = round_trip(&document, &DagMutation::RenameNode(RenameNode { id: "a".into(), new_id: "aa".into() }));
    assert!(renamed.nodes.iter().any(|node| node.id == "aa"));
    assert_eq!(renamed.edges[0].source, "aa@out");
    assert_eq!(renamed.edges[0].target, "b@in");
}

#[test]
fn delete_node_severs_and_reconnects_edges() {
    let mut document = empty_dag_document();
    document.nodes = vec![sample_node("a"), sample_node("b")];
    document.edges = vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::from([("weight".to_string(), PropertyValue::Number(2.0))]) }];
    let deleted = round_trip(&document, &DagMutation::DeleteNode(DeleteNode { id: "a".into() }));
    assert!(deleted.nodes.iter().all(|node| node.id != "a"));
    assert!(deleted.edges.is_empty(), "the severed edge must be removed by the same delete-node diff, not left dangling");
}

#[test]
fn reorder_nodes_round_trips() {
    let mut document = empty_dag_document();
    document.nodes = vec![sample_node("a"), sample_node("b"), sample_node("c")];
    let reordered = round_trip(&document, &DagMutation::ReorderNodes(ReorderNodes { order: vec!["c".into(), "a".into(), "b".into()] }));
    let ids: Vec<&str> = reordered.nodes.iter().map(|node| node.id.as_str()).collect();
    assert_eq!(ids, vec!["c", "a", "b"]);
    document.nodes = reordered.nodes;
}

#[test]
fn connect_disconnect_nodes_round_trip() {
    let mut document = empty_dag_document();
    document.nodes = vec![sample_node("a"), sample_node("b")];
    let connected = round_trip(&document, &DagMutation::ConnectNodes(ConnectNodes { index: 0, id: "e1".into(), source: "a@out".into(), target: "b@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::default() }));
    assert_eq!(connected.edges.len(), 1);
    let disconnected = round_trip(&connected, &DagMutation::DisconnectNodes(DisconnectNodes { id: "e1".into() }));
    assert!(disconnected.edges.is_empty());
}

//#region 🔖️MutationLaws
#[test]
fn diff_and_inverse_are_deterministic() {
    let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
    let mutation = DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 12.0, y: 34.0 });
    assert_eq!(Mutation::diff(&mutation, &document), Mutation::diff(&mutation, &document), "diff(payload, base) must be a pure function of its inputs");
    assert_eq!(mutation.inverse(&document), mutation.inverse(&document), "inverse(payload, base) must be a pure function of its inputs");
}

#[test]
fn move_node_diff_is_consistent_with_direct_field_mutation() {
    let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
    let mutation = DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 5.0, y: 6.0 });
    let via_diff = Mutation::diff(&mutation, &document).diff().apply(&document).expect("valid DAG diff");
    let mut via_direct = document;
    via_direct.nodes[0].x = 5.0;
    via_direct.nodes[0].y = 6.0;
    assert_eq!(via_diff, via_direct, "diff().apply() must match the mutation's own documented field-level effect");
}

#[test]
fn move_node_diff_absorb_law_holds() {
    let document = round_trip(&empty_dag_document(), &DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
    let (mut d1, _) = Mutation::diff(&DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 10.0, y: 10.0 }), &document).into_parts();
    let mid = d1.apply(&document).expect("valid first DAG diff");
    let (d2, _) = Mutation::diff(&DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 20.0, y: 30.0 }), &mid).into_parts();
    d1.absorb(d2);
    let absorbed = d1.apply(&document).expect("valid absorbed DAG diff");
    assert_eq!(absorbed.nodes[0].x, 20.0, "absorb must converge to the LATER move, not the earlier one");
    assert_eq!(absorbed.nodes[0].y, 30.0);
}

#[test]
fn missing_target_inverse_and_diff_are_no_ops() {
    let document = empty_dag_document();
    assert_eq!(Mutation::diff(&DagMutation::MoveNode(MoveNode { id: "ghost".into(), x: 1.0, y: 1.0 }), &document).diff(), &DagDiff::default());
    assert!(DagMutation::MoveNode(MoveNode { id: "ghost".into(), x: 1.0, y: 1.0 }).inverse(&document).is_empty());
    assert!(DagMutation::DeleteNode(DeleteNode { id: "ghost".into() }).inverse(&document).is_empty());
    assert!(DagMutation::DisconnectNodes(DisconnectNodes { id: "ghost".into() }).inverse(&document).is_empty());
}
//#endregion 🔖️MutationLaws

//#region 🔖️DslTests
/// 🧩️ One node per `DagNodeKind` tag (safe field values only — no raw JSON literals in
/// `default`/`value`/`Tree.json`, see {@link json_to_property}'s docstring), so the DSL round trip
/// exercises every kind-specific payload shape the wire-literal property bag needs to carry.
fn kitchen_sink_snapshot() -> DagSnapshot {
    let port = |id: &str, label: &str| IoPortSpec::simple(id, label);
    let nodes = vec![
        DagNodeSpec {
            id: "comp".into(),
            name: "Comp".into(),
            abbreviation: "Cmp".into(),
            icon: "emoji:🧮️".into(),
            x: -120.0,
            y: -40.0,
            width: 104.0,
            height: 14.0,
            kind: DagNodeKind::Computation { inputs: vec![port("in", "In")], outputs: vec![port("out", "Out")], variadic_inputs: true, variadic_outputs: false },
            ..Default::default()
        },
        DagNodeSpec { id: "slider".into(), name: "Amount".into(), x: -400.0, y: -40.0, width: 70.0, height: 14.0, kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 5.0, output: port("out", "value") }, ..Default::default() },
        DagNodeSpec {
            id: "mode".into(),
            name: "Mode".into(),
            x: -400.0,
            y: 80.0,
            width: 56.0,
            height: 28.0,
            kind: DagNodeKind::Select { options: vec!["Add".into(), "Multiply".into()], selected: 1, output: port("out", "mode") },
            ..Default::default()
        },
        DagNodeSpec {
            id: "screen".into(),
            name: "Preview".into(),
            x: 400.0,
            y: 0.0,
            width: 200.0,
            height: 140.0,
            kind: DagNodeKind::Screen { media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,%3Csvg viewBox='0 0 1 1'%3E%3C/svg%3E".into() }), input: port("in", "result") },
            ..Default::default()
        },
        DagNodeSpec { id: "note".into(), name: "Note".into(), x: 0.0, y: 200.0, kind: DagNodeKind::Note { text: "line one\nline two — with a ' quote and a % sign".into(), output: port("out", "text") }, ..Default::default() },
        DagNodeSpec { id: "image".into(), name: "Image".into(), x: 0.0, y: 260.0, kind: DagNodeKind::Image { src: "data:image/png;base64,AAA=".into(), output: port("out", "img") }, ..Default::default() },
        DagNodeSpec {
            id: "preview".into(),
            name: "Preview2".into(),
            x: 0.0,
            y: 320.0,
            kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: "42".into() }, expanded: BTreeSet::from(["a.b".to_string()]), input: port("in", "value") },
            ..Default::default()
        },
        DagNodeSpec { id: "action".into(), name: "Action".into(), x: 0.0, y: 380.0, kind: DagNodeKind::Action { label: "Run".into(), input: port("in", "trigger") }, ..Default::default() },
        DagNodeSpec { id: "export".into(), name: "Export".into(), x: 0.0, y: 440.0, kind: DagNodeKind::Export { label: "Save".into(), format: "png".into(), input: port("in", "value") }, ..Default::default() },
        DagNodeSpec { id: "cluster".into(), name: "Cluster".into(), x: 0.0, y: 500.0, kind: DagNodeKind::Cluster { inputs: vec![port("in", "In")], outputs: vec![port("out", "Out")] }, ..Default::default() },
        DagNodeSpec {
            id: "app".into(),
            name: "App".into(),
            x: 0.0,
            y: 560.0,
            kind: DagNodeKind::AppInstance { instance_id: "inst-1".into(), plugin_id: "prog-1".into(), app_id: "note".into(), icon: "emoji:📦️".into(), inputs: vec![], outputs: vec![port("out", "Out")] },
            ..Default::default()
        },
    ];
    let edges = vec![
        DagFixtureEdge { id: "e1".into(), source: "slider@out".into(), target: "comp@in".into(), ..Default::default() },
        DagFixtureEdge { id: "e2".into(), source: "comp@out".into(), target: "screen@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::from([("weight".to_string(), PropertyValue::Number(2.0))]) },
    ];
    DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), nodes, edges }
}

#[test]
fn dag_document_dsl_round_trips_the_demo_fixture() {
    crate::os_store::test_support::assert_dsl_round_trip(&default_dag_document());
    crate::os_store::test_support::assert_dsl_pack_equivalence(&default_dag_document());
}

#[test]
fn bundled_demo_fixture_is_canonical() {
    let actual = include_str!("../../../../../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
    let expected = <DagSnapshot as crate::os_store::ArtifactDsl>::print_dsl(&default_dag_document());
    assert_eq!(actual, expected, "bundled demo fixture must stay in canonical owned DSL form");
}

#[test]
fn dag_document_dsl_round_trips_every_node_kind() {
    crate::os_store::test_support::assert_dsl_round_trip(&kitchen_sink_snapshot());
    crate::os_store::test_support::assert_dsl_pack_equivalence(&kitchen_sink_snapshot());
}

#[test]
fn dag_document_dsl_round_trips_the_empty_document() {
    crate::os_store::test_support::assert_dsl_round_trip(&empty_dag_document());
    crate::os_store::test_support::assert_dsl_pack_equivalence(&empty_dag_document());
}

#[test]
fn op_text_round_trips_create_node() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::CreateNode(CreateNode { node: sample_node("n1"), index: 0 }));
}

#[test]
fn op_text_round_trips_delete_node() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::DeleteNode(DeleteNode { id: "n1".into() }));
}

#[test]
fn op_text_round_trips_rename_node() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::RenameNode(RenameNode { id: "n1".into(), new_id: "n1-renamed".into() }));
}

#[test]
fn op_text_round_trips_change_node_name() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeName(ChangeNodeName { id: "n1".into(), new_name: "Renamed".into() }));
}

#[test]
fn op_text_round_trips_move_node() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::MoveNode(MoveNode { id: "n1".into(), x: 42.0, y: 7.0 }));
}

#[test]
fn op_text_round_trips_resize_node() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ResizeNode(ResizeNode { id: "n1".into(), width: 200.0, height: 90.0 }));
}

#[test]
fn op_text_round_trips_change_node_icon() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeIcon(ChangeNodeIcon { id: "n1".into(), new_icon: "emoji:🧪️".into() }));
}

#[test]
fn op_text_round_trips_change_node_abbreviation() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeAbbreviation(ChangeNodeAbbreviation { id: "n1".into(), new_abbreviation: "N1".into() }));
}

#[test]
fn op_text_round_trips_change_node_operator_kind_some_and_none() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id: "n1".into(), new_operator_kind: Some("math.add".into()) }));
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id: "n1".into(), new_operator_kind: None }));
}

#[test]
fn op_text_round_trips_replace_node_kind() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ReplaceNodeKind(ReplaceNodeKind { id: "n1".into(), new_kind: DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec::simple("out", "value") } }));
}

#[test]
fn op_text_round_trips_replace_node_properties() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ReplaceNodeProperties(ReplaceNodeProperties { id: "n1".into(), new_properties: PropertyBag::from([("weight".to_string(), PropertyValue::Number(2.0))]) }));
}

#[test]
fn op_text_round_trips_reorder_nodes() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ReorderNodes(ReorderNodes { order: vec!["a".into(), "b".into(), "c".into()] }));
}

#[test]
fn op_text_round_trips_connect_nodes() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::ConnectNodes(ConnectNodes {
        index: 0,
        id: "e1".into(),
        source: "a@out".into(),
        target: "b@in".into(),
        route_style: EdgeRouteStyle::SharpSz,
        properties: PropertyBag::from([("weight".to_string(), PropertyValue::Number(2.0))]),
    }));
}

#[test]
fn op_text_round_trips_disconnect_nodes() {
    crate::os_store::test_support::assert_op_line_round_trip(&DagMutation::DisconnectNodes(DisconnectNodes { id: "e1".into() }));
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_a_store_with_an_applied_operation() {
    let mut store = create_dag_store("dag", kitchen_sink_snapshot()).await.expect("store");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DagMutation::CreateNode(CreateNode { node: sample_node("extra"), index: 0 })], description: None }).await.expect("apply");
    crate::os_store::test_support::assert_document_text_round_trip(&store).await;
    crate::os_store::test_support::assert_document_pack_round_trip(&store).await;
    close_dag_test_store(store);
}

/// 🎫️ Command envelopes preserve the aggregate's direct leaf-owned operation codecs.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    let mut store = create_dag_store("dag", kitchen_sink_snapshot()).await.expect("store");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DagMutation::CreateNode(CreateNode { node: sample_node("extra"), index: 0 })], description: None }).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<DagMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    crate::os_store::test_support::assert_command_envelope_round_trip::<DagSnapshot, DagMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
    close_dag_test_store(store);
}
//#endregion 🔖️DslTests
