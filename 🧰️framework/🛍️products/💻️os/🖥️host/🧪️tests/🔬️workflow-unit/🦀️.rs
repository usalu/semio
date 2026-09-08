mod tests {
    use super::*;

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
        use semio_framework::io::io_mechanism::{IoEntry, io_register};
        use semio_framework::io_schema::{CARRIER_BINARY, IoFidelity, IoOutcome, IoPayload as NewIoPayload};

        const TEST_KIND: &str = "3d.__w1b_export_bug_proof";
        const TEST_DIALECT: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: "s.__w1b_export_bug_proof", standard: semio_framework::StandardId("1"), subset: semio_framework::SubsetId("*") };
        /// 🧷️ The literal bytes of `store::BINARY_MAGIC` / `os_semio::BINARY_MAGIC`
        /// (`🧬️semio/🦀️.rs`), inlined so this assertion never depends on that constant's
        /// own export path -- a genuinely independent check of the OLD pack format's header.
        const PACK_MAGIC: [u8; 8] = [0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];

        fn run(payload: &NewIoPayload) -> semio_framework::io_schema::IoResult<NewIoPayload> {
            let NewIoPayload::Text(json) = payload else {
                return Err(semio_framework::io_schema::IoError { message: "expected a text native payload".to_string(), diagnostics: Vec::new() });
            };
            let value: Value = serde_json::from_str(json).map_err(|error| semio_framework::io_schema::IoError { message: error.to_string(), diagnostics: Vec::new() })?;
            let raw = value["value"].as_str().unwrap_or_default().to_string();
            Ok(IoOutcome::clean(NewIoPayload::Binary(raw.into_bytes())))
        }

        // 🧷️ Built directly as a one-element array literal (never a separately-named `static`
        // copied into an array) -- `IoEntry` derives no `Copy`/`Clone`, so moving a value OUT of
        // a separate `static` to build `[ENTRY]` would not compile; a single constant-expression
        // array literal has no such move.
        static ENTRIES: [IoEntry; 1] = [IoEntry { from: TEST_DIALECT, into: CARRIER_BINARY, fidelity: IoFidelity::Exact, sniff: None, run }];
        // 📌️ Idempotent re-registration (nextest runs this file's tests in one process) -- a
        // second run of this same test binary registering the identical static entry must not error.
        io_register(&ENTRIES).ok();

        crate::registry::register_artifact_descriptor(&semio_framework::ArtifactKindSpec {
            id: TEST_KIND.to_string(),
            name: "W1b Export Bug Proof".to_string(),
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
        assert_eq!(crate::registry::os_artifact_dialect(TEST_KIND).to_coordinate(), "s.__w1b_export_bug_proof@1/*", "catalog-derived dialect must exactly match the dialect the test IoEntry was registered under");

        crate::host::resolve_kernel_future(semio_framework::register_format_descriptors([semio_framework::FormatDescriptor {
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
            registry_export_media(TEST_KIND, "stdio.__w1b_export_bug_proof_fmt", &source_document).expect("io-mechanism export path must find the registered route, not fall through to the legacy/handler-map paths").expect("export must succeed");

        let bytes = base64_codec::base64_standard_decode(&outcome.data).expect("OsMediaExportResult base64-encodes binary payloads");
        assert_eq!(bytes, b"RAW-FILE-CONTENT-not-a-pack".to_vec(), "exported bytes must be exactly the raw content the io-mechanism route produced, byte for byte");
        assert!(!bytes.starts_with(&PACK_MAGIC), "exported bytes must NOT carry the SemioEnvelope pack magic header -- this is the exact `registry_export_media` bug (design.md, `📌️important.md`) this ticket exists to remove");
    }

    #[test]
    fn validates_empty_workflow() {
        assert!(validate_workflow(&crate::host::resolve_kernel_future(empty_workflow())).ok);
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn svg_path_extraction_preserves_transformed_geometry() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect x="1" y="1" width="4" height="4"/></svg>"#;
        let paths = crate::media_export_raster::svg_to_polylines(svg).expect("SVG paths");
        assert_eq!(paths.len(), 1);
        assert!(paths[0].closed);
        assert_eq!(paths[0].vertices[0], [1.0, 9.0]);
    }

    #[test]
    fn mesh_exporter_registrar_round_trips_a_box_through_glb() {
        crate::media_export_raster::register_mesh_exporter("3d.__mesh_exporter_test", "box", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::GlbExporter));
        let result = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&os_media_handler_key("3d.__mesh_exporter_test", "glb")).expect("glb handler registered")(&serde_json::json!({})).expect("export glb");
        let bytes = base64_codec::base64_standard_decode(result.data).expect("decode base64");
        let mesh = semio_framework::mesh_from_glb(&bytes).expect("glb decodes back to a mesh");
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn mesh_importer_registrar_round_trips_a_box_through_obj() {
        crate::media_export_raster::register_mesh_importer("3d.__mesh_importer_test", |mesh| Ok(serde_json::json!({ "vertexCount": mesh.vertex_count() })), Box::new(semio_framework_plugin::ObjImporter));
        let obj_bytes = semio_framework::mesh_to_obj(&semio_framework_plugin::mesh_from_kind("box"), "box").into_bytes();
        let handlers = import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let handler = handlers.get(&os_media_handler_key("3d.__mesh_importer_test", "obj")).expect("obj handler registered");
        let document = handler(&obj_bytes).expect("import obj");
        assert!(document["vertexCount"].as_u64().expect("vertex count") > 0);
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

    /// 🧷️ Hand-built node for tests that don't need a real app registration — `os_workflow_to_flow_fixture`/
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
        let mut graph = crate::host::resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        let fixture = os_workflow_to_flow_fixture(&graph, &OsWorkflowCamera::default());
        assert_eq!(fixture["schema"], "flow.fixture");
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
        let mut graph = crate::host::resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 40.0, 80.0));
        graph.nodes.push(media_node("node-2", 300.0, 80.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let camera = OsWorkflowCamera { x: 12.0, y: -8.0, zoom: 1.5 };
        let fixture = os_workflow_to_flow_fixture(&graph, &camera);
        assert_eq!(fixture["camera"]["x"], 12.0);
        assert_eq!(fixture["camera"]["zoom"], 1.5);
        let unchanged = apply_flow_fixture_to_os_workflow(&graph, &fixture.to_string());
        assert!(unchanged.is_empty());
        let mut moved = fixture.clone();
        moved["layout"]["node-1"] = json!({ "x": 220.0, "y": 156.0 });
        let operations = apply_flow_fixture_to_os_workflow(&graph, &moved.to_string());
        assert_eq!(operations, vec![WorkflowMutation::MoveNode(MoveNode { node_id: "node-1".into(), x: 140.0, y: 120.0 })]);
    }

    #[test]
    fn flow_fixture_diff_connects_disconnects_and_removes() {
        let mut graph = crate::host::resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        graph.nodes.push(media_node("node-2", 200.0, 0.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let mut fixture = os_workflow_to_flow_fixture(&graph, &OsWorkflowCamera::default());
        fixture["synapses"] = json!([
            { "id": "", "from": "node-2", "fromPort": "node-2:out", "to": "node-1", "toPort": "node-1:in" }
        ]);
        let operations = apply_flow_fixture_to_os_workflow(&graph, &fixture.to_string());
        assert!(matches!(
            &operations[0],
            WorkflowMutation::ConnectPorts(ConnectPorts { edge }) if edge.source_node_id == "node-2" && edge.target_port_id == "node-1:in" && !edge.id.is_empty()
        ));
        assert!(operations.contains(&WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: "edge-1".into() })));
        let mut removal = os_workflow_to_flow_fixture(&graph, &OsWorkflowCamera::default());
        removal["widgets"] = json!([{ "id": "node-1" }]);
        removal["synapses"] = json!([]);
        let removal_operations = apply_flow_fixture_to_os_workflow(&graph, &removal.to_string());
        assert!(removal_operations.contains(&WorkflowMutation::RemoveNode(RemoveNode { node_id: "node-2".into() })));
        assert!(!removal_operations.iter().any(|operation| matches!(operation, WorkflowMutation::DisconnectEdge(DisconnectEdge { .. }))));
    }

    //#region 🔖️WorkflowPlanner
    fn dirty_set(node_ids: &[&str]) -> HashSet<String> {
        node_ids.iter().map(|id| id.to_string()).collect()
    }

    #[test]
    fn plans_a_single_delivery_across_one_dirty_edge() {
        let mut graph = crate::host::resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        graph.nodes.push(media_node("node-2", 200.0, 0.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let deliveries = crate::host::resolve_kernel_future(plan_workflow(&graph, &dirty_set(&["node-1"])));
        assert_eq!(deliveries, vec![WorkflowDelivery { edge_id: "edge-1".into(), producer_node_id: "node-1".into(), producer_port_id: "node-1:out".into(), consumer_node_id: "node-2".into(), consumer_port_id: "node-2:in".into() }]);
    }

    #[test]
    fn plans_a_chain_in_topological_order_when_only_the_root_is_dirty() {
        let mut graph = crate::host::resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        graph.nodes.push(media_node("node-2", 200.0, 0.0));
        graph.nodes.push(media_node("node-3", 400.0, 0.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-ab".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        graph.edges.push(WorkflowEdge {
            id: "edge-bc".into(),
            source_node_id: "node-2".into(),
            source_port_id: "node-2:out".into(),
            target_node_id: "node-3".into(),
            target_port_id: "node-3:in".into(),
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let deliveries = crate::host::resolve_kernel_future(plan_workflow(&graph, &dirty_set(&["node-1"])));
        assert_eq!(deliveries.iter().map(|delivery| delivery.edge_id.as_str()).collect::<Vec<_>>(), vec!["edge-ab", "edge-bc"], "A→B must be planned before B→C");
    }

    #[test]
    fn plans_a_diamond_with_one_delivery_per_incoming_edge() {
        // 🔀️ One delivery per edge, not per node: D has two producers (B and C), so D is the
        // target of two separate deliveries rather than a single merged one.
        let mut graph = crate::host::resolve_kernel_future(empty_workflow());
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
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        graph.edges.push(WorkflowEdge {
            id: "edge-ac".into(),
            source_node_id: "node-a".into(),
            source_port_id: "node-a:out".into(),
            target_node_id: "node-c".into(),
            target_port_id: "node-c:in".into(),
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        graph.edges.push(WorkflowEdge {
            id: "edge-bd".into(),
            source_node_id: "node-b".into(),
            source_port_id: "node-b:out".into(),
            target_node_id: "node-d".into(),
            target_port_id: "node-d:in".into(),
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        graph.edges.push(WorkflowEdge {
            id: "edge-cd".into(),
            source_node_id: "node-c".into(),
            source_port_id: "node-c:out".into(),
            target_node_id: "node-d".into(),
            target_port_id: "node-d:in".into(),
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        let deliveries = crate::host::resolve_kernel_future(plan_workflow(&graph, &dirty_set(&["node-a"])));
        let edge_ids: Vec<&str> = deliveries.iter().map(|delivery| delivery.edge_id.as_str()).collect();
        assert_eq!(edge_ids.len(), 4);
        let index_of = |id: &str| edge_ids.iter().position(|candidate| *candidate == id).unwrap();
        assert!(index_of("edge-bd") > index_of("edge-ab"), "B→D must be planned after A→B");
        assert!(index_of("edge-cd") > index_of("edge-ac"), "C→D must be planned after A→C");
    }

    #[test]
    fn plans_nothing_when_no_instance_is_dirty() {
        let mut graph = crate::host::resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        graph.nodes.push(media_node("node-2", 200.0, 0.0));
        graph.edges.push(WorkflowEdge {
            id: "edge-1".into(),
            source_node_id: "node-1".into(),
            source_port_id: "node-1:out".into(),
            target_node_id: "node-2".into(),
            target_port_id: "node-2:in".into(),
            contract: crate::host::resolve_kernel_future(placeholder_media_contract("2d.drawing")),
        });
        assert!(crate::host::resolve_kernel_future(plan_workflow(&graph, &dirty_set(&[]))).is_empty());
    }

    #[test]
    fn plans_nothing_for_a_dirty_node_with_no_outgoing_edges() {
        let mut graph = crate::host::resolve_kernel_future(empty_workflow());
        graph.nodes.push(media_node("node-1", 0.0, 0.0));
        assert!(crate::host::resolve_kernel_future(plan_workflow(&graph, &dirty_set(&["node-1"]))).is_empty());
    }

    /// 🔬️ Shared fixtures replay (`framework/product/os/core/fixtures/*.dsl`) — the same files
    /// drive `planWorkflow`'s vitest harness in `js/index.ts` (decoded there via the sibling
    /// `.spk` through a wasm export), keeping the two implementations in lockstep. See
    /// `framework/product/os/core/fixtures/README.md`.
    fn workflow_fixture_dsl_paths() -> Vec<std::path::PathBuf> {
        let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧫️fixtures");
        let entries = std::fs::read_dir(&fixtures_dir).unwrap_or_else(|error| panic!("read fixtures dir {fixtures_dir:?}: {error}"));
        let mut paths: Vec<std::path::PathBuf> = entries.map(|entry| entry.expect("dir entry").path()).filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("dsl")).collect();
        paths.sort();
        paths
    }

    #[test]
    fn workflow_fixtures_match_expected_deliveries() {
        let paths = workflow_fixture_dsl_paths();
        for path in &paths {
            let contents = std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read fixture {path:?}: {error}"));
            let fixture = <WorkflowFixture as store::ArtifactDsl>::parse_dsl(&contents).unwrap_or_else(|error| panic!("parse fixture {path:?}: {error}"));
            let dirty: HashSet<String> = fixture.dirty_node_ids.iter().cloned().collect();
            let deliveries = crate::host::resolve_kernel_future(plan_workflow(&fixture.graph, &dirty));
            assert_eq!(deliveries, fixture.expected_deliveries, "fixture {} mismatch", fixture.name);
        }
        assert!(paths.len() >= 5, "expected workflow fixtures in fixtures dir, found {}", paths.len());
    }

    /// 🧬️ Every fixture ships as a `.dsl`/`.spk` pair: both must decode to the identical
    /// `WorkflowFixture`, the `.dsl` text must already be its own canonical `print_dsl`
    /// fixpoint, and the `.spk` bytes must match a fresh canonical `encode_pack()` of the
    /// parsed document byte-for-byte (canonical pack encoding is deterministic, independent of
    /// field-map iteration order — see `store`'s pack facade docs).
    #[test]
    fn workflow_fixture_dsl_and_spk_pairs_are_canonical_and_equivalent() {
        let paths = workflow_fixture_dsl_paths();
        for dsl_path in &paths {
            let file_name = dsl_path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
            let spk_name = if file_name.starts_with("🗣️") { file_name.replacen("🗣️", "📦️", 1).replace(".dsl", ".spk") } else { file_name.replace(".dsl", ".spk") };
            let spk_path = dsl_path.with_file_name(spk_name);
            let dsl_text = std::fs::read_to_string(dsl_path).unwrap_or_else(|error| panic!("read {dsl_path:?}: {error}"));
            let spk_bytes = std::fs::read(&spk_path).unwrap_or_else(|error| panic!("read {spk_path:?}: {error}"));
            let via_dsl = <WorkflowFixture as store::ArtifactDsl>::parse_dsl(&dsl_text).unwrap_or_else(|error| panic!("parse {dsl_path:?}: {error}"));
            let via_pack = <WorkflowFixture as store::ArtifactPack>::decode_pack(&spk_bytes).unwrap_or_else(|error| panic!("decode {spk_path:?}: {error}"));
            assert_eq!(via_dsl, via_pack, "{dsl_path:?} and {spk_path:?} decode to different documents");
            assert_eq!(store::ArtifactDsl::print_dsl(&via_dsl), dsl_text, "{dsl_path:?} is not its own canonical print_dsl fixpoint");
            assert_eq!(store::ArtifactPack::encode_pack(&via_dsl), spk_bytes, "{spk_path:?} does not match a fresh canonical encode_pack()");
            store::test_support::assert_dsl_pack_equivalence(&via_dsl);
        }
    }
    //#endregion 🔖️WorkflowPlanner
}
