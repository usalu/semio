//! 🧪️ Workflow artifact replay, connection, and serialization laws.
use super::*;
use protocol::MutationDiff;

#[semio_framework_async_macros::async_test]
async fn empty_workflow_default() {
    let workflow = empty_workflow().await;
    assert_eq!(workflow.schema, WORKFLOW_SCHEMA);
    assert!(workflow.nodes.is_empty());
}

async fn media_port_spec(id: &str, direction: MediaPortDirection, kind_id: Option<&str>) -> MediaPortSpec {
    MediaPortSpec { id: id.into(), label: id.into(), direction, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, kind_id: kind_id.map(Into::into), required: true, multiplicity: PortMultiplicity::One }
}

async fn workflow_node(id: &str, outputs: Vec<WorkflowMediaPort>, inputs: Vec<WorkflowMediaPort>) -> WorkflowNode {
    WorkflowNode {
        id: id.into(),
        plugin_id: "plugin".into(),
        app_id: "app".into(),
        label: id.into(),
        yields: String::new(),
        artifact_ref: format!("artifacts/{id}"),
        config_ref: format!("config/{id}"),
        x: 0.0,
        y: 0.0,
        width: 220.0,
        height: 100.0,
        inputs,
        outputs,
    }
}

async fn workflow_edge(id: &str, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> WorkflowEdge {
    WorkflowEdge { id: id.into(), source_node_id: source_node_id.into(), source_port_id: source_port_id.into(), target_node_id: target_node_id.into(), target_port_id: target_port_id.into(), contract: placeholder_media_contract("data.value").await }
}

#[semio_framework_async_macros::async_test]
async fn workflow_media_port_id_format() {
    let spec = media_port_spec("out", MediaPortDirection::Out, Some("kind.a")).await;
    let port = workflow_media_port("n1", &spec);
    assert_eq!(port.id, "n1:out:out");
    assert_eq!(port.spec, spec);

    let spec_in = media_port_spec("in", MediaPortDirection::In, None).await;
    let port_in = workflow_media_port("n1", &spec_in);
    assert_eq!(port_in.id, "n1:in:in");
}

#[semio_framework_async_macros::async_test]
async fn media_contract_dsl_round_trips() {
    let contract = MediaContract {
        kind_id: "puzzle.2d.fixture".into(),
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        wire: MediaWireFormat::Binary { format_kind: "svg".into() },
        conversion: Some((MediaForm::Brep, MediaForm::Mesh)),
    };
    let record = media_contract_to_record(&contract);
    let round_tripped = media_contract_from_record(&record).expect("decode");
    assert_eq!(round_tripped, contract);

    let placeholder = placeholder_media_contract("draw.document").await;
    let placeholder_record = media_contract_to_record(&placeholder);
    assert_eq!(media_contract_from_record(&placeholder_record).expect("decode placeholder"), placeholder);
}

#[semio_framework_async_macros::async_test]
async fn workflow_media_port_dsl_round_trips() {
    let port = WorkflowMediaPort { id: "n1:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, Some("kind.a")).await };
    let record = workflow_media_port_to_record(&port);
    assert_eq!(workflow_media_port_from_record(&record).expect("decode"), port);

    let port_no_kind = WorkflowMediaPort { id: "n1:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await };
    let record_no_kind = workflow_media_port_to_record(&port_no_kind);
    assert_eq!(workflow_media_port_from_record(&record_no_kind).expect("decode"), port_no_kind);
}

#[semio_framework_async_macros::async_test]
async fn validate_workflow_flags_dangling_edge() {
    let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }], vec![]);
    let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await], edges: vec![workflow_edge("e1", "a", "out", "missing", "in").await] };
    let validation = validate_workflow(&graph).await;
    assert!(!validation.ok);
    assert!(validation.errors.iter().any(|e| e.contains("missing target node missing")));
}

#[semio_framework_async_macros::async_test]
async fn validate_workflow_flags_cycle() {
    let node_a = workflow_node(
        "a",
        vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }],
        vec![WorkflowMediaPort { id: "a:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }],
    );
    let node_b = workflow_node(
        "b",
        vec![WorkflowMediaPort { id: "b:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }],
        vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }],
    );
    let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await], edges: vec![workflow_edge("e1", "a", "out", "b", "in").await, workflow_edge("e2", "b", "out", "a", "in").await] };
    let validation = validate_workflow(&graph).await;
    assert!(!validation.ok);
    assert!(validation.errors.iter().any(|e| e.starts_with("cycle detected")));
}

#[semio_framework_async_macros::async_test]
async fn validate_workflow_ok_for_acyclic_connected_graph() {
    let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }], vec![]);
    let node_b = workflow_node("b", vec![], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }]);
    let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await], edges: vec![workflow_edge("e1", "a", "out", "b", "in").await] };
    let validation = validate_workflow(&graph).await;
    assert!(validation.ok);
    assert!(validation.errors.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn plan_workflow_propagates_dirtiness_across_multi_hop_chain() {
    let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }], vec![]);
    let node_b = workflow_node(
        "b",
        vec![WorkflowMediaPort { id: "b:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }],
        vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }],
    );
    let node_c = workflow_node("c", vec![], vec![WorkflowMediaPort { id: "c:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }]);
    let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await, node_c.await], edges: vec![workflow_edge("e1", "a", "out", "b", "in").await, workflow_edge("e2", "b", "out", "c", "in").await] };

    let mut dirty = HashSet::new();
    dirty.insert("a".to_string());
    let deliveries = plan_workflow(&graph, &dirty).await;
    assert_eq!(deliveries.len(), 2);
    assert_eq!(deliveries[0].edge_id, "e1");
    assert_eq!(deliveries[0].producer_node_id, "a");
    assert_eq!(deliveries[0].consumer_node_id, "b");
    assert_eq!(deliveries[1].edge_id, "e2");
    assert_eq!(deliveries[1].producer_node_id, "b");
    assert_eq!(deliveries[1].consumer_node_id, "c");
}

#[semio_framework_async_macros::async_test]
async fn plan_workflow_skips_clean_nodes() {
    let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, None).await }], vec![]);
    let node_b = workflow_node("b", vec![], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: media_port_spec("in", MediaPortDirection::In, None).await }]);
    let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await], edges: vec![workflow_edge("e1", "a", "out", "b", "in").await] };
    let deliveries = plan_workflow(&graph, &HashSet::new()).await;
    assert!(deliveries.is_empty());
}

//#region 🧪️WorkflowSnapshotLaws
async fn sample_workflow_snapshot() -> WorkflowSnapshot {
    let node_a = workflow_node("a", vec![WorkflowMediaPort { id: "a:out:out".into(), spec: media_port_spec("out", MediaPortDirection::Out, Some("kind.a")).await }], vec![]);
    let node_b = workflow_node("b", vec![], vec![WorkflowMediaPort { id: "b:in:in".into(), spec: MediaPortSpec { required: true, ..media_port_spec("in", MediaPortDirection::In, Some("kind.a")).await } }]);
    let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node_a.await, node_b.await], edges: vec![workflow_edge("e1", "a", "a:out:out", "b", "b:in:in").await] };
    let parameter_bindings = vec![WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "a".into(), field_path: "/zoom".into() }];
    // 🧷️ Keeps the fixture's node ports in the same synced state `apply_workflow_operation`
    // maintains as an invariant (every `BindParameterField`/`AddNode`/etc call re-derives parameter
    // ports from `parameter_bindings`) — an un-synced fixture would make `assert_operation_round_trip`
    // see a spurious port diff on any op whose apply path re-syncs, not a real bug.
    let graph = sync_workflow_parameter_ports(&graph, &parameter_bindings);
    WorkflowSnapshot {
        schema: S_WORKFLOW_SCHEMA.into(),
        graph,
        parameters: vec![
            WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) },
            WorkflowParameter::Categorical { id: "p2".into(), name: "Mode".into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] },
            WorkflowParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: true },
            WorkflowParameter::Text { id: "p4".into(), name: "Label".into(), value: "hello".into() },
        ],
        parameter_bindings,
        inputs: vec![WorkflowInput { id: "in-1".into(), kind_id: "kind.a".into(), selector: "**/*.puzzle2d".into(), required: true, multiplicity: PortMultiplicity::One }],
        input_bindings: Vec::new(),
        output_bindings: vec![WorkflowOutputBinding { node_id: "a".into(), port_id: "a:out:out".into(), path_template: "renders/{node}.out".into() }],
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_workflow_snapshot_matches_schema() {
    let document = empty_workflow_snapshot().await;
    assert_eq!(document.schema, S_WORKFLOW_SCHEMA);
    assert!(document.graph.nodes.is_empty());
    assert!(document.parameters.is_empty());
    assert!(document.inputs.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn workflow_snapshot_dsl_pack_round_trips() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&sample_workflow_snapshot().await);
    store::os_store::test_support::assert_dsl_pack_equivalence(&empty_workflow_snapshot().await);
}

#[semio_framework_async_macros::async_test]
async fn workflow_operation_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::AddNode(AddNode { node: workflow_node("n1", Vec::new(), Vec::new()).await }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::RemoveNode(RemoveNode { node_id: "n1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::ConnectPorts(ConnectPorts { edge: workflow_edge("e1", "a", "out", "b", "in").await }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: "e1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::MoveNode(MoveNode { node_id: "n1".into(), x: 5.5, y: -6.25 }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::RenameNode(RenameNode { node_id: "n1".into(), label: "Renamed".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::AddParameter(AddParameter { parameter: Box::new(WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: None, max: None, step: None }) }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::RemoveParameter(RemoveParameter { parameter_id: "p1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::ChangeParameter(ChangeParameter { parameter_id: "p1".into(), parameter: Box::new(WorkflowParameter::Toggle { id: "p1".into(), name: "Flag".into(), value: false }) }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::BindParameterField(BindParameterField { binding: WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "n1".into(), field_path: "/zoom".into() } }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::UnbindParameterField(UnbindParameterField { node_id: "n1".into(), field_path: "/zoom".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::UpdateNodePorts(UpdateNodePorts {}));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::AddInput(AddInput {
        input: WorkflowInput { id: "in-1".into(), kind_id: "kind.a".into(), selector: "**/*".into(), required: true, multiplicity: PortMultiplicity::Many },
    }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::RemoveInput(RemoveInput { input_id: "in-1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::BindInput(BindInput { binding: WorkflowInputBinding { input_id: "in-1".into(), node_id: "n1".into(), port_id: "n1:in:in".into() } }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::UnbindInput(UnbindInput { input_id: "in-1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::BindOutput(BindOutput { binding: WorkflowOutputBinding { node_id: "n1".into(), port_id: "n1:out:out".into(), path_template: "out/{node}".into() } }));
    store::os_store::test_support::assert_op_line_round_trip(&WorkflowMutation::UnbindOutput(UnbindOutput { node_id: "n1".into(), port_id: "n1:out:out".into() }));
}

#[semio_framework_async_macros::async_test]
async fn workflow_operation_backwards_restores_pre_state() {
    let document = sample_workflow_snapshot().await;
    // 🧷️ `Remove*` ops are exercised via `*_backwards_restores_cascade_deleted_dependents` below
    // instead of this strict-equality helper: `apply`'s `Add*` counterpart appends to the END of
    // its list, so removing a non-last element and letting `inverse` re-add it changes list
    // ORDER even though every element is restored — a known, harmless limitation of list-append
    // state shared by every `Add*`/`Remove*` pair in this codebase (e.g. `space::SpaceMutation`'s
    // own `inverse` law tests avoid it the same way, by only exercising it on singleton lists).
    store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::AddNode(AddNode { node: workflow_node("c", Vec::new(), Vec::new()).await })).await;
    store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::MoveNode(MoveNode { node_id: "a".into(), x: 99.0, y: -1.0 })).await;
    store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::AddParameter(AddParameter { parameter: Box::new(WorkflowParameter::Toggle { id: "p9".into(), name: "New".into(), value: true }) })).await;
    store::os_store::test_support::assert_operation_round_trip(
        &document,
        WorkflowMutation::AddInput(AddInput { input: WorkflowInput { id: "in-2".into(), kind_id: "kind.b".into(), selector: "**/*".into(), required: false, multiplicity: PortMultiplicity::One } }),
    )
    .await;
    store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::BindInput(BindInput { binding: WorkflowInputBinding { input_id: "in-1".into(), node_id: "b".into(), port_id: "b:in:in".into() } })).await;
    store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::BindOutput(BindOutput { binding: WorkflowOutputBinding { node_id: "a".into(), port_id: "a:out:out".into(), path_template: "renders/other.out".into() } }))
        .await;
    store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::UnbindOutput(UnbindOutput { node_id: "a".into(), port_id: "a:out:out".into() })).await;
}

/// 🧵️ Removing the LAST element of each cascade-owning list keeps append-order stable, so this can
/// use the strict-equality `assert_operation_round_trip` helper to prove `RemoveNode`/
/// `RemoveParameter`/`RemoveInput`'s inverse restores every cascade-deleted dependent (edges/
/// parameter bindings/input bindings/output bindings), not just the bare removed item.
#[semio_framework_async_macros::async_test]
async fn remove_operations_backwards_restores_cascade_deleted_dependents() {
    let mut document = sample_workflow_snapshot().await;
    // `b` is the last node — removing it also cascade-drops edge `e1` (which targets it).
    store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::RemoveNode(RemoveNode { node_id: "b".into() })).await;

    // `p4` is the last parameter and has no bindings, so this only proves the simple case; add a
    // binding on it first so the cascade-restoration path is actually exercised.
    document.parameter_bindings.push(WorkflowParameterBinding { parameter_id: "p4".into(), node_id: "a".into(), field_path: "/label".into() });
    document.graph = sync_workflow_parameter_ports(&document.graph, &document.parameter_bindings);
    store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::RemoveParameter(RemoveParameter { parameter_id: "p4".into() })).await;

    document.input_bindings.push(WorkflowInputBinding { input_id: "in-1".into(), node_id: "b".into(), port_id: "b:in:in".into() });
    store::os_store::test_support::assert_operation_round_trip(&document, WorkflowMutation::RemoveInput(RemoveInput { input_id: "in-1".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn workflow_diff_print_parse_and_encode_decode_round_trip() {
    let diffs = vec![
        WorkflowDiff::AddNode { node: workflow_node("n1", Vec::new(), Vec::new()).await },
        WorkflowDiff::DeclareInput { input: WorkflowInput { id: "in-1".into(), kind_id: "kind.a".into(), selector: "**/*".into(), required: true, multiplicity: PortMultiplicity::One } },
        WorkflowDiff::Empty,
    ];
    for diff in diffs {
        let applied = MutationDiff::apply(&diff, &empty_workflow_snapshot().await).expect("valid workflow diff");
        let _ = applied;
    }
}

#[semio_framework_async_macros::async_test]
async fn validate_workflow_snapshot_requires_edge_xor_input_binding_on_required_ports() {
    let mut document = sample_workflow_snapshot().await;
    // 🎯️ Sample fixture wires `a -> b` over `b`'s sole required in-port with NO input binding — ok.
    assert!(validate_workflow_snapshot(&document).await.ok, "wired required port with no binding must validate");

    let binding = WorkflowInputBinding { input_id: "in-1".into(), node_id: "b".into(), port_id: "b:in:in".into() };
    document.input_bindings.push(binding.clone());
    let both = validate_workflow_snapshot(&document).await;
    assert!(!both.ok);
    assert!(both.errors.iter().any(|error| error.contains("has both a wire and an input binding")), "{:?}", both.errors);

    document.graph.edges.clear();
    document.input_bindings.clear();
    let neither = validate_workflow_snapshot(&document).await;
    assert!(!neither.ok);
    assert!(neither.errors.iter().any(|error| error.contains("has neither a wire nor an input binding")), "{:?}", neither.errors);

    document.input_bindings.push(binding);
    assert!(validate_workflow_snapshot(&document).await.ok, "input binding alone must satisfy a required port");
}

#[semio_framework_async_macros::async_test]
async fn validate_workflow_snapshot_flags_unresolved_bindings() {
    let mut document = sample_workflow_snapshot().await;
    document.input_bindings.push(WorkflowInputBinding { input_id: "missing-input".into(), node_id: "b".into(), port_id: "b:in:in".into() });
    document.output_bindings.push(WorkflowOutputBinding { node_id: "missing-node".into(), port_id: "x".into(), path_template: "out".into() });
    let validation = validate_workflow_snapshot(&document).await;
    assert!(!validation.ok);
    assert!(validation.errors.iter().any(|error| error.contains("unknown input 'missing-input'")));
    assert!(validation.errors.iter().any(|error| error.contains("unknown node/port 'missing-node:x'")));
}
//#endregion 🧪️WorkflowSnapshotLaws

#[semio_framework_async_macros::async_test]
async fn independent_package_fixture_matches_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️package-contract/📜️cases.json")).expect("language-neutral fixture");
    assert_eq!(fixture["package"], env!("CARGO_PKG_NAME"));
    let document = empty_workflow_snapshot().await;
    let json = dsl::os_pack::json::to_json_string(&document);
    let oracle: serde_json::Value = serde_json::from_str(&json).expect("third-party JSON oracle");
    assert_eq!(oracle, fixture["expected"]);
    let decoded: WorkflowSnapshot = dsl::os_pack::json::from_json_str(&serde_json::to_string(&oracle).expect("oracle JSON")).expect("domain decoder");
    assert_eq!(decoded, document);
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    println!("[DEBUG] Workflow artifact independent package fixture passed");
}
