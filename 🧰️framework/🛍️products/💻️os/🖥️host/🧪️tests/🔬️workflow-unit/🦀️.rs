mod tests {
    use super::*;
    use semio_framework_os_host_registry::register_artifact_descriptor;
    use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText;

    /// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1: the real
    /// end-to-end proof for the bug this whole ticket exists to remove -- "export as .xyz" must
    /// write raw file content, never a `SemioEnvelope`-wrapped pack container (`store::BINARY_MAGIC`
    /// = `[0x89, 'S','E','M', 0x0D,0x0A,0x1A,0x0A]`, `🧬️semio/🦀️.rs`). Registers a
    /// throwaway `IoEntry` straight from a synthetic artifact dialect to `CARRIER_BINARY` (its
    /// `run` returns whatever raw bytes its input carries -- no pack framing of any kind, exactly
    /// the shape a real format serializer produces), registers a matching `OsArtifactDescriptor`
    /// so `crate::registry::os_artifact_dialect` derives the SAME dialect the entry was registered
    /// under, then calls the real `registry_export_media` (task 1's actual production entry point,
    /// not a private helper) end to end: `os_artifact_dialect` -> `io_route` -> `io_run` ->
    /// `OsMediaExportResult`. Asserts the decoded bytes are byte-identical to the raw content and
    /// do NOT start with the pack magic header.
    #[test]
    fn export_via_io_mechanism_writes_raw_bytes_not_a_pack_container() {
        use semio_framework_os_kernel::io::io_mechanism::{IoEntry, IoEntryDirection, io_register};
        use semio_framework::io_schema::{CARRIER_BINARY, IoFidelity, IoOutcome, IoPayload as NewIoPayload};

        const TEST_KIND: &str = "3d.__w1b_export_bug_proof";
        const TEST_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.__w1b_export_bug_proof", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") };
        /// 🧷️ The literal bytes of `store::BINARY_MAGIC` / `os_semio::BINARY_MAGIC`
        /// (`🧬️semio/🦀️.rs`), inlined so this assertion never depends on that constant's
        /// own export path -- a genuinely independent check of the OLD pack format's header.
        const PACK_MAGIC: [u8; 8] = [0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];

        fn run(payload: &NewIoPayload, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> semio_framework::io_schema::IoResult<NewIoPayload> {
            let NewIoPayload::Text(json) = payload else {
                return Err(semio_framework::io_schema::IoError { cause: semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a text native payload"), diagnostics: Vec::new() });
            };
            let value: Value = serde_json::from_str(json).map_err(|error| semio_framework::io_schema::IoError { cause: semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()), diagnostics: Vec::new() })?;
            let raw = value["value"].as_str().unwrap_or_default().to_string();
            Ok(IoOutcome::clean(NewIoPayload::Binary(raw.into_bytes())))
        }

        static ENTRIES: [IoEntry; 1] = [IoEntry { owned_serializer: None, from: TEST_DIALECT, into: CARRIER_BINARY, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run }];
        io_register(&ENTRIES).ok();

        register_artifact_descriptor(&semio_framework::ArtifactKindSpec {
            id: TEST_KIND.to_string(),
            label: semio_framework_ui_locale::LocalizedLabel::native("W1b Export Bug Proof", "W1b-Exportfehlernachweis"),
            source_format: TEST_KIND.to_string(),
            component_kind: "__w1b_export_bug_proof".to_string(),
            dimension: "data".to_string(),
            media_capability: OsMediaCapability::MeshOnly,
            media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value },
            schema: TEST_KIND.to_string(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            export_stdio_kinds: Vec::new(),
            import_stdio_kinds: Vec::new(),
        });
        assert_eq!(os_artifact_dialect(TEST_KIND).to_coordinate(), "s.__w1b_export_bug_proof@1/*", "catalog-derived dialect must exactly match the dialect the test IoEntry was registered under");

        resolve_kernel_future(semio_framework_os_kernel::io::register_format_descriptors([semio_framework_os_kernel::io::FormatDescriptor {
            kind_id: "stdio.__w1b_export_bug_proof_fmt".to_string(),
            short_id: "w1bproof".to_string(),
            aliases: Vec::new(),
            mimes: vec!["application/octet-stream".to_string()],
            extensions: vec![".w1bproof".to_string()],
            name: "W1b Proof Format".to_string(),
            full_name: "W1b Export Bug Proof Format".to_string(),
            neutral: false,
            dir_name: "w1bproof".to_string(),
            is_binary: true,
        }]))
        .ok();

        let source_document = serde_json::json!({ "value": "RAW-FILE-CONTENT-not-a-pack" });
        let outcome =
            registry_export_media(TEST_KIND, "stdio.__w1b_export_bug_proof_fmt", &source_document, &semio_framework_async::CancelToken::root_now()).expect("io-mechanism export path must find the registered route, not fall through to the composer-registry/handler-map paths").expect("export must succeed");

        let bytes = base64_codec::base64_standard_decode(&outcome.data).expect("OsMediaExportResult base64-encodes binary payloads");
        assert_eq!(bytes, b"RAW-FILE-CONTENT-not-a-pack".to_vec(), "exported bytes must be exactly the raw content the io-mechanism route produced, byte for byte");
        assert!(!bytes.starts_with(&PACK_MAGIC), "exported bytes must NOT carry the SemioEnvelope pack magic header -- this is the exact `registry_export_media` bug (design.md, `📌️important.md`) this ticket exists to remove");
    }

    #[test]
    fn validates_empty_workflow() {
        assert!(validate_workflow(&resolve_kernel_future(empty_workflow())).ok);
    }

    // 🚪️ `solid_exporter_and_importer_registrars_round_trip_a_box_through_step` DELETED (ticket
    // 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1): it exercised
    // exactly the `register_solid_exporter`/`register_solid_importer`/`solid_exporter_for`/
    // `export_registered_solid`/`import_registered_solid` mechanism deleted above with it -- see
    // `//#region SolidMediaExport`'s removal note for why that mechanism was dead weight, not a
    // migration gap. The equivalent real coverage (a box round-tripped through STEP via the
    // genuine stdio `semio/brep` bridge) lives in cad's own
    // `export_solids_as_step_round_trips_through_real_semio_brep_bridge`
    // (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`),
    // which this test's own STEP path always deferred to in production anyway.

    /// 🧷️ Hand-built node for tests that don't need a real app registration — `os_workflow_to_flow_host_snapshot_json`/
    /// `build_os_workflow_operator_infos`/VFS listing all read straight off the node now (no more
    /// separate `OsAppInstance` join), so a plain struct literal is enough.
    fn media_node(id: &str, x: f64, y: f64) -> WorkflowNode {
        let port = |direction: semio_framework::MediaPortDirection| WorkflowMediaPort {
            id: format!("{id}:{}", if direction == semio_framework::MediaPortDirection::In { "in" } else { "out" }),
            spec: semio_framework::MediaPortSpec {
                id: if direction == semio_framework::MediaPortDirection::In { "in".into() } else { "out".into() },
                label: "Port".into(),
                direction,
                media_type: semio_framework::MediaType { class: semio_framework::MediaClass::TwoD, form: semio_framework::MediaForm::Vector },
                kind_id: Some("2d.drawing".into()),
                required: false,
                multiplicity: semio_framework::PortMultiplicity::One,
            },
        };
        WorkflowNode {
            id: id.into(),
            plugin_id: "draw".into(),
            app_id: "draw".into(),
            label: id.into(),
            yields: "2d.drawing".into(),
            artifact_ref: format!("artifacts/{id}"),
            config_ref: format!("config/{id}"),
            x,
            y,
            width: 160.0,
            height: 72.0,
            inputs: vec![port(semio_framework::MediaPortDirection::In)],
            outputs: vec![port(semio_framework::MediaPortDirection::Out)],
        }
    }

    #[test]
    fn flow_fixture_projects_neuron_preview() {
        let mut graph = resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        let fixture = os_workflow_to_flow_host_snapshot_json(&graph, &OsWorkflowCamera::default());
        assert_eq!(fixture["schema"], "flow.host_snapshot");
        assert_eq!(fixture["widgets"][0]["preview"], true);
        assert_eq!(fixture["widgets"][0]["params"]["nodeId"], "node-1");
        assert_eq!(fixture["widgets"][0]["params"]["pluginId"], "draw");
        assert_eq!(fixture["widgets"][0]["params"]["appId"], "draw");
        let operators = build_os_workflow_operator_infos(&graph, &[]);
        assert_eq!(operators.len(), 1);
        assert_eq!(operators[0].id, "os.media.node.node-1");
        assert_eq!(operators[0].module, OS_MEDIA_FLOW_MODULE_ID);
        assert_eq!(operators[0].name, "node-1");
    }

    // 🚧️ `vfs_inputs_folder_lists_a_dwg_import_row_for_2d_kinds` exercised the deleted
    // `🔖️WorkflowVfs` region (`list_os_workflow_vfs_children`/`os_workflow_vfs_inputs_folder_id`)
    // — a full collection-browser UI replaces it in a later wave, see the os-core dissolve ticket.

    #[test]
    fn flow_fixture_round_trips_camera_and_diffs_back_to_operations() {
        let mut graph = resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 40.0, 80.0));
        graph.nodes.push(media_node("node-2", 300.0, 80.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let camera = OsWorkflowCamera { x: 12.0, y: -8.0, zoom: 1.5 };
        let fixture = os_workflow_to_flow_host_snapshot_json(&graph, &camera);
        assert_eq!(fixture["camera"]["x"], 12.0);
        assert_eq!(fixture["camera"]["zoom"], 1.5);
        let unchanged = apply_flow_host_snapshot_to_os_workflow(&graph, &fixture.to_string());
        assert!(unchanged.is_empty());
        let mut moved = fixture.clone();
        moved["layout"]["node-1"] = json!({ "x": 220.0, "y": 156.0 });
        let operations = apply_flow_host_snapshot_to_os_workflow(&graph, &moved.to_string());
        assert_eq!(operations, vec![WorkflowMutation::MoveNode(MoveNode { node_id: "node-1".into(), x: 140.0, y: 120.0 })]);
    }

    #[test]
    fn flow_fixture_diff_connects_disconnects_and_removes() {
        let mut graph = resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        graph.nodes.push(media_node("node-2", 200.0, 0.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let mut fixture = os_workflow_to_flow_host_snapshot_json(&graph, &OsWorkflowCamera::default());
        fixture["synapses"] = json!([
            { "id": "", "from": "node-2", "fromPort": "node-2:out", "to": "node-1", "toPort": "node-1:in" }
        ]);
        let operations = apply_flow_host_snapshot_to_os_workflow(&graph, &fixture.to_string());
        assert!(matches!(
            &operations[0],
            WorkflowMutation::ConnectPorts(ConnectPorts { edge }) if edge.source_node_id == "node-2" && edge.target_port_id == "node-1:in" && !edge.id.is_empty()
        ));
        assert!(operations.contains(&WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: "edge-1".into() })));
        let mut removal = os_workflow_to_flow_host_snapshot_json(&graph, &OsWorkflowCamera::default());
        removal["widgets"] = json!([{ "id": "node-1" }]);
        removal["synapses"] = json!([]);
        let removal_operations = apply_flow_host_snapshot_to_os_workflow(&graph, &removal.to_string());
        assert!(removal_operations.contains(&WorkflowMutation::RemoveNode(RemoveNode { node_id: "node-2".into() })));
        assert!(!removal_operations.iter().any(|operation| matches!(operation, WorkflowMutation::DisconnectEdge(DisconnectEdge { .. }))));
    }

    //#region 🔖️WorkflowPlanner
    fn dirty_set(node_ids: &[&str]) -> HashSet<String> {
        node_ids.iter().map(|id| id.to_string()).collect()
    }

    #[test]
    fn plans_a_single_delivery_across_one_dirty_edge() {
        let mut graph = resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        graph.nodes.push(media_node("node-2", 200.0, 0.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let deliveries = resolve_kernel_future(plan_workflow(&graph, &dirty_set(&["node-1"])));
        assert_eq!(deliveries, vec![WorkflowDelivery { edge_id: "edge-1".into(), producer_node_id: "node-1".into(), producer_port_id: "node-1:out".into(), consumer_node_id: "node-2".into(), consumer_port_id: "node-2:in".into() }]);
    }

    #[test]
    fn plans_a_chain_in_topological_order_when_only_the_root_is_dirty() {
        let mut graph = resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        graph.nodes.push(media_node("node-2", 200.0, 0.0));
        graph.nodes.push(media_node("node-3", 400.0, 0.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-ab".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        graph.edges.push(WorkflowEdge {
            id: "edge-bc".into(),
            source_node_id: "node-2".into(),
            source_port_id: "node-2:out".into(),
            target_node_id: "node-3".into(),
            target_port_id: "node-3:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let deliveries = resolve_kernel_future(plan_workflow(&graph, &dirty_set(&["node-1"])));
        assert_eq!(deliveries.iter().map(|delivery| delivery.edge_id.as_str()).collect::<Vec<_>>(), vec!["edge-ab", "edge-bc"], "A→B must be planned before B→C");
    }

    #[test]
    fn plans_a_diamond_with_one_delivery_per_incoming_edge() {
        let mut graph = resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-a", 0.0, 0.0));
        graph.nodes.push(media_node("node-b", 200.0, -80.0));
        graph.nodes.push(media_node("node-c", 200.0, 80.0));
        graph.nodes.push(media_node("node-d", 400.0, 0.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-ab".into(),
            source_node_id: "node-a".into(),
            source_port_id: "node-a:out".into(),
            target_node_id: "node-b".into(),
            target_port_id: "node-b:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        graph.edges.push(WorkflowEdge {
            id: "edge-ac".into(),
            source_node_id: "node-a".into(),
            source_port_id: "node-a:out".into(),
            target_node_id: "node-c".into(),
            target_port_id: "node-c:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        graph.edges.push(WorkflowEdge {
            id: "edge-bd".into(),
            source_node_id: "node-b".into(),
            source_port_id: "node-b:out".into(),
            target_node_id: "node-d".into(),
            target_port_id: "node-d:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        graph.edges.push(WorkflowEdge {
            id: "edge-cd".into(),
            source_node_id: "node-c".into(),
            source_port_id: "node-c:out".into(),
            target_node_id: "node-d".into(),
            target_port_id: "node-d:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let deliveries = resolve_kernel_future(plan_workflow(&graph, &dirty_set(&["node-a"])));
        let edge_ids: Vec<&str> = deliveries.iter().map(|delivery| delivery.edge_id.as_str()).collect();
        assert_eq!(edge_ids.len(), 4);
        let index_of = |id: &str| edge_ids.iter().position(|candidate| *candidate == id).unwrap();
        assert!(index_of("edge-bd") > index_of("edge-ab"), "B→D must be planned after A→B");
        assert!(index_of("edge-cd") > index_of("edge-ac"), "C→D must be planned after A→C");
    }

    #[test]
    fn plans_nothing_when_no_instance_is_dirty() {
        let mut graph = resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        graph.nodes.push(media_node("node-2", 200.0, 0.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        assert!(resolve_kernel_future(plan_workflow(&graph, &dirty_set(&[]))).is_empty());
    }

    #[test]
    fn plans_nothing_for_a_dirty_node_with_no_outgoing_edges() {
        let mut graph = resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        assert!(resolve_kernel_future(plan_workflow(&graph, &dirty_set(&["node-1"]))).is_empty());
    }

    #[derive(semio_framework_value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct PlannerCase {
        name: String,
        graph: Workflow,
        dirty_node_ids: Vec<String>,
        expected_deliveries: Vec<WorkflowDelivery>,
    }

    #[test]
    fn language_neutral_planner_witnesses_match_expected_deliveries() {
        let examples = [
            include_str!("../../../🧪️tests/🧫️fixtures/➡️single-edge/🔣️.json"),
            include_str!("../../../🧪️tests/🧫️fixtures/🔗️chain/🔣️.json"),
            include_str!("../../../🧪️tests/🧫️fixtures/💎️diamond/🔣️.json"),
            include_str!("../../../🧪️tests/🧫️fixtures/💤️no-dirty/🔣️.json"),
            include_str!("../../../🧪️tests/🧫️fixtures/🛑️dead-end/🔣️.json"),
        ];
        for example in examples {
            let oracle: serde_json::Value = serde_json::from_str(example).unwrap();
            let witness: PlannerCase = semio_framework_pack_json::from_json_str(example, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            assert_eq!(oracle["name"].as_str(), Some(witness.name.as_str()));
            let dirty = witness.dirty_node_ids.into_iter().collect();
            let deliveries = resolve_kernel_future(plan_workflow(&witness.graph, &dirty));
            assert_eq!(deliveries, witness.expected_deliveries, "{}", witness.name);
            let mut document = resolve_kernel_future(empty_workflow_snapshot());
            document.graph = witness.graph;
            store::test_support::assert_dsl_pack_equivalence(&document);
        }
    }
    //#endregion 🔖️WorkflowPlanner
}
