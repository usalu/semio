    /// 📝️ DSL and operation text laws: the sample workflow snapshot, its DSL round trips and the operation text and document text round trips.

    // #region 🔖️DslAndOpText
    /// 🧵️ A representative `WorkflowSnapshot` exercising every collection: two workflow nodes wired
    /// by one edge, one of each `WorkflowParameter` variant, and one parameter binding — so the DSL
    /// round trip actually covers the workflow encoding, not just an empty-document fixpoint.
    fn sample_workflow_snapshot() -> workflow::WorkflowSnapshot {
        let node_a = workflow::WorkflowNode {
            id: "app-1".into(),
            plugin_id: "puzzle".into(),
            app_id: "puzzle2d".into(),
            label: "Puzzle Board \"3D\"".into(),
            yields: "board.ports.directed.v1".into(),
            artifact_ref: "artifacts/app-1".into(),
            config_ref: "config/app-1".into(),
            x: 0.0,
            y: 0.0,
            width: 220.0,
            height: 92.0,
            inputs: Vec::new(),
            outputs: vec![workflow::WorkflowMediaPort {
                id: "app-1:puzzle.out:out".into(),
                spec: semio_framework::MediaPortSpec {
                    id: "puzzle.out".into(),
                    label: "Out".into(),
                    direction: semio_framework::MediaPortDirection::Out,
                    media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    kind_id: Some("board.ports.directed.v1".into()),
                    required: false,
                    multiplicity: semio_framework::PortMultiplicity::One,
                },
            }],
        };
        let node_b = workflow::WorkflowNode {
            id: "app-2".into(),
            plugin_id: "draw".into(),
            app_id: "draw".into(),
            label: "Draw Sink".into(),
            yields: "draw.document".into(),
            artifact_ref: "artifacts/app-2".into(),
            config_ref: "config/app-2".into(),
            x: 240.0,
            y: 0.0,
            width: 220.0,
            height: 92.0,
            inputs: vec![workflow::WorkflowMediaPort {
                id: "app-2:draw.in:in".into(),
                spec: semio_framework::MediaPortSpec {
                    id: "draw.in".into(),
                    label: "In".into(),
                    direction: semio_framework::MediaPortDirection::In,
                    media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    kind_id: Some("board.ports.directed.v1".into()),
                    required: false,
                    multiplicity: semio_framework::PortMultiplicity::One,
                },
            }],
            outputs: Vec::new(),
        };
        let edge = WorkflowEdge {
            id: "edge-1".into(),
            source_node_id: "app-1".into(),
            source_port_id: "app-1:puzzle.out:out".into(),
            target_node_id: "app-2".into(),
            target_port_id: "app-2:draw.in:in".into(),
            contract: MediaContract { kind_id: "board.ports.directed.v1".into(), media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, wire: MediaWireFormat::Document { schema: "board.ports.directed.v1".into() }, conversion: None },
        };
        workflow::WorkflowSnapshot {
            schema: workflow::S_WORKFLOW_SCHEMA.into(),
            graph: workflow::Workflow { schema: workflow::WORKFLOW_SCHEMA.into(), nodes: vec![node_a, node_b], edges: vec![edge] },
            parameters: vec![
                workflow::WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) },
                workflow::WorkflowParameter::Categorical { id: "p2".into(), name: "Mode".into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B, with comma".into()] },
                workflow::WorkflowParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: true },
                workflow::WorkflowParameter::Text { id: "p4".into(), name: "Label".into(), value: "hello \"world\"\nnewline".into() },
            ],
            parameter_bindings: vec![workflow::WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "app-1".into(), field_path: "/zoom".into() }],
            inputs: Vec::new(),
            input_bindings: Vec::new(),
            output_bindings: Vec::new(),
        }
    }

    #[test]
    fn dsl_round_trips_default_workflow_snapshot() {
        let snapshot = resolve_kernel_future(workflow::empty_workflow_snapshot());
        store::test_support::assert_dsl_round_trip(&snapshot);
        store::test_support::assert_dsl_pack_equivalence(&snapshot);
    }

    #[test]
    fn dsl_round_trips_workflow_snapshot_with_graph_and_parameters() {
        store::test_support::assert_dsl_round_trip(&sample_workflow_snapshot());
        store::test_support::assert_dsl_pack_equivalence(&sample_workflow_snapshot());
    }

    #[test]
    fn op_text_round_trips_add_workflow_node() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddNode(workflow::AddNode {
            node: workflow::WorkflowNode {
                id: "node-1".into(),
                plugin_id: "puzzle".into(),
                app_id: "puzzle2d".into(),
                label: "Puzzle Board".into(),
                yields: "board.ports.directed.v1".into(),
                artifact_ref: "artifacts/node-1".into(),
                config_ref: "config/node-1".into(),
                x: 10.0,
                y: -20.5,
                width: 220.0,
                height: 92.0,
                inputs: Vec::new(),
                outputs: Vec::new(),
            },
        }));
    }

    #[test]
    fn op_text_round_trips_remove_workflow_node() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::RemoveNode(workflow::RemoveNode { node_id: "app-1".into() }));
    }

    #[test]
    fn op_text_round_trips_connect_media_ports() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::ConnectPorts(workflow::ConnectPorts {
            edge: WorkflowEdge {
                id: "edge-1".into(),
                source_node_id: "node-1".into(),
                source_port_id: "app-1:out:out".into(),
                target_node_id: "node-2".into(),
                target_port_id: "app-2:in:in".into(),
                contract: MediaContract {
                    kind_id: "board.ports.directed.v1".into(),
                    media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
                    wire: MediaWireFormat::Binary { format_kind: "stl".into() },
                    conversion: Some((MediaForm::Brep, MediaForm::Mesh)),
                },
            },
        }));
    }

    #[test]
    fn op_text_round_trips_disconnect_media_edge() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::DisconnectEdge(workflow::DisconnectEdge { edge_id: "edge-1".into() }));
    }

    #[test]
    fn op_text_round_trips_move_media_node() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::MoveNode(workflow::MoveNode { node_id: "node-1".into(), x: 5.5, y: -6.25 }));
    }

    #[test]
    fn op_text_round_trips_patch_workflow_node() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::RenameNode(workflow::RenameNode { node_id: "app-1".into(), label: "Renamed \"Board\"".into() }));
    }

    #[test]
    fn op_text_round_trips_add_parameter() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddParameter(workflow::AddParameter {
            parameter: Box::new(workflow::WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) }),
        }));
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddParameter(workflow::AddParameter {
            parameter: Box::new(workflow::WorkflowParameter::Categorical { id: "p2".into(), name: "Mode".into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] }),
        }));
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddParameter(workflow::AddParameter { parameter: Box::new(workflow::WorkflowParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: false }) }));
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddParameter(workflow::AddParameter { parameter: Box::new(workflow::WorkflowParameter::Text { id: "p4".into(), name: "Label".into(), value: "hi there".into() }) }));
    }

    #[test]
    fn op_text_round_trips_remove_parameter() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::RemoveParameter(workflow::RemoveParameter { parameter_id: "p1".into() }));
    }

    #[test]
    fn op_text_round_trips_patch_parameter() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::ChangeParameter(workflow::ChangeParameter {
            parameter_id: "p1".into(),
            parameter: Box::new(workflow::WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 20.0, min: None, max: None, step: None }),
        }));
    }

    #[test]
    fn op_text_round_trips_bind_parameter_field() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::BindParameterField(workflow::BindParameterField {
            binding: workflow::WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "app-1".into(), field_path: "/zoom".into() },
        }));
    }

    #[test]
    fn op_text_round_trips_unbind_parameter_field() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::UnbindParameterField(workflow::UnbindParameterField { node_id: "app-1".into(), field_path: "/zoom".into() }));
    }

    #[test]
    fn op_text_round_trips_sync_node_ports() {
        store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::UpdateNodePorts(workflow::UpdateNodePorts {}));
    }

    #[test]
    fn document_text_round_trips_store_with_applied_operation() {
        test_identity!(identity);
        let envelope = create_document_envelope(workflow::S_WORKFLOW_SCHEMA, "workflow-text-test", resolve_kernel_future(workflow::empty_workflow_snapshot()), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut store = resolve_kernel_future(ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(store::os_spr::LOCAL_ACTOR_ID.into()))).expect("valid artifact store fixture");
        resolve_kernel_future(store.dispatch(ArtifactCommand::Apply { mutations: vec![workflow::WorkflowMutation::UpdateNodePorts(workflow::UpdateNodePorts {})], transaction: None }, &mut identity)).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
    // #endregion 🔖️DslAndOpText
