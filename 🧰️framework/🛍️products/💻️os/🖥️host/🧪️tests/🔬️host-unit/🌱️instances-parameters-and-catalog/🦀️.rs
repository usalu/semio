    /// 🌱️ Instances, parameters and catalog: spawning and removing app instances, studio parameters, space catalog entries, cycles and the concurrent delete and wire reconcile.

    #[test]
    fn spawns_and_removes_app_instances() {
        test_identity!(identity);
        seed_draw_plugin();
        let mut space_store = test_space_store();
        let mut store = test_workflow_store();
        store.add_workflow_node("draw", "draw", None, 40.0, 40.0, &mut space_store, &mut identity).expect("spawn");
        assert_eq!(store.snapshot().expect("projection").graph.nodes.len(), 1);
        assert!(space_store.snapshot().expect("projection").programs.contains(&"draw".to_string()), "spawning a node must install its plugin into the owning space");
        store.dispatch_text("undo", &mut identity).expect("undo");
        assert_eq!(store.snapshot().expect("projection").graph.nodes.len(), 0);
    }

    #[test]
    fn adds_and_patches_studio_parameters() {
        test_identity!(identity);
        let mut store = test_workflow_store();
        let parameter_id = store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Zoom", &mut identity).expect("add");
        store.patch_parameter(&parameter_id, &DslValue::object([("value".to_string(), DslValue::float(12.0)), ("max".to_string(), DslValue::float(10.0))]), &mut identity).expect("patch");
        match &store.snapshot().expect("projection").parameters[0] {
            workflow::WorkflowParameter::Numeric { value, .. } => assert_eq!(*value, 10.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn creates_and_lists_space_catalog_entries() {
        let port = Arc::new(OsBackbonePorts::store(store::os_store::backbone::BackbonePorts::Memory(resolve_kernel_future(MemoryBackbonePort::new()))));
        let owner = space::SpaceUser { id: "user-1".into(), name: "Ada".into(), avatar: None, role: space::SpaceRole::Author };
        let entry = create_os_space("Catalog Space", space::SpaceKind::Studio, space::SpaceVisibility::Private, owner, &port).expect("create");
        assert_eq!(entry.collection_count, 1, "create_os_space must seed exactly one default collection");
        assert_eq!(entry.kind, space::SpaceKind::Studio);
        assert_eq!(entry.visibility, space::SpaceVisibility::Private);
        let listed = list_os_space_catalog_entries(&port).expect("list");
        assert!(listed.iter().any(|row| row.id == entry.id));
        delete_os_space(&entry.id, &port).expect("delete");
        assert!(!list_os_space_catalog_entries(&port).expect("list").iter().any(|row| row.id == entry.id));
    }

    #[test]
    fn space_pack_export_keeps_the_document_name_and_import_never_overwrites() {
        let fresh = || Arc::new(OsBackbonePorts::store(store::os_store::backbone::BackbonePorts::Memory(resolve_kernel_future(MemoryBackbonePort::new()))));
        let port = fresh();
        let owner = space::SpaceUser { id: "user-1".into(), name: "Ada".into(), avatar: None, role: space::SpaceRole::Author };
        let entry = create_os_space("Grundriss: EG/OG?", space::SpaceKind::Studio, space::SpaceVisibility::Private, owner, &port).expect("create");
        let document = load_os_space_document(&entry.id, &port).expect("load");
        let export = export_os_space_pack(&document).expect("pack export");
        assert_eq!(export.name, document.name);
        assert_eq!(export.file_stem(&entry.id), "Grundriss_ EG_OG_");
        let unnamed = BackboneExport { name: String::new(), files: () };
        assert_eq!(unnamed.file_stem("space-1"), "space-1");
        assert!(import_os_space_from_pack(&export.files.pack, &export.files.spr, &port).is_err(), "an existing id is never overwritten");
        let other = fresh();
        let admitted = import_os_space_from_pack(&export.files.pack, &export.files.spr, &other).expect("first import into an empty catalog");
        assert_eq!(admitted.id, entry.id);
        assert!(import_os_space_from_pack(&export.files.pack, &export.files.spr, &other).is_err(), "the second import of the same id is refused");
    }

    #[test]
    fn file_space_port_persists_exactly_the_chosen_file_with_no_companions() {
        let directory = std::env::temp_dir().join(format!("semio-file-space-{}", std::process::id()));
        std::fs::create_dir_all(&directory).expect("scratch directory");
        let file = directory.join("studio.space");
        let ports = open_file_space_backbone(file.to_str().expect("utf-8 scratch path")).expect("file space port");
        let source = Arc::new(OsBackbonePorts::store(store::os_store::backbone::BackbonePorts::Memory(resolve_kernel_future(MemoryBackbonePort::new()))));
        let owner = space::SpaceUser { id: "user-1".into(), name: "Ada".into(), avatar: None, role: space::SpaceRole::Author };
        let entry = create_os_space("Studio", space::SpaceKind::Studio, space::SpaceVisibility::Private, owner, &source).expect("create");
        let document = load_os_space_document(&entry.id, &source).expect("load");
        let payload = encode_backbone_payload(&document).expect("payload");
        let uri = format!("file://{}", file.display());
        assert!(ports.read(&uri).is_err(), "a file that does not exist yet is missing, not empty");
        ports.write(&uri, &payload).expect("write");
        assert_eq!(std::fs::read(&file).expect("the chosen file itself"), payload);
        assert_eq!(ports.read(&uri).expect("read back"), payload);
        assert!(ports.write(&uri, &[]).is_err(), "a file space is never emptied");
        let names: Vec<_> = std::fs::read_dir(&directory).expect("list").map(|entry| entry.expect("entry").file_name().to_string_lossy().into_owned()).collect();
        assert_eq!(names, vec!["studio.space".to_string()], "no companion or temporary files remain");
        std::fs::remove_dir_all(&directory).ok();
    }

    #[test]
    fn saved_at_stamps_sort_as_instants_and_never_as_text() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🕰️saved-at-stamps/🔣️.json")).unwrap();
        for row in fixture["accepted"].as_array().unwrap() {
            let expected: i128 = row["nanoseconds"].as_str().unwrap().parse().unwrap();
            assert_eq!(saved_at_nanoseconds(row["stamp"].as_str().unwrap()).unwrap(), expected, "{row}");
        }
        for stamp in fixture["refused"].as_array().unwrap() {
            assert!(saved_at_nanoseconds(stamp.as_str().unwrap()).is_err(), "{stamp}");
        }
        assert!(saved_at_nanoseconds("2026-11-04T06:41:07.5Z").unwrap() < saved_at_nanoseconds("2026-11-04T06:41:07Z").unwrap() + 1_000_000_000);
    }

    #[test]
    fn catalog_state_does_not_outlive_its_port() {
        let owner = || space::SpaceUser { id: "user-1".into(), name: "Ada".into(), avatar: None, role: space::SpaceRole::Author };
        let first = Arc::new(OsBackbonePorts::store(store::os_store::backbone::BackbonePorts::Memory(resolve_kernel_future(MemoryBackbonePort::new()))));
        let entry = create_os_space("First", space::SpaceKind::Studio, space::SpaceVisibility::Private, owner(), &first).expect("create");
        assert!(list_os_space_catalog_entries(&first).unwrap().iter().any(|row| row.id == entry.id));
        drop(first);
        for _ in 0..64 {
            let next = Arc::new(OsBackbonePorts::store(store::os_store::backbone::BackbonePorts::Memory(resolve_kernel_future(MemoryBackbonePort::new()))));
            assert!(list_os_space_catalog_entries(&next).unwrap().is_empty(), "a reused allocation never inherits another port's spaces");
        }
    }

    #[test]
    fn validates_workflow_cycles() {
        assert!(validate_workflow(&resolve_kernel_future(empty_workflow())).ok);
    }

    #[test]
    fn concurrent_delete_and_wire_reconciles_without_a_dangling_edge() {
        test_identity!(identity);
        seed_draw_plugin();
        seed_sink_plugin();
        let mut space_store_a = test_space_store();
        let mut store_a = test_workflow_store();
        let node_a_id = store_a.add_workflow_node("draw", "draw", None, 0.0, 0.0, &mut space_store_a, &mut identity).expect("spawn a");
        let node_b_id = store_a.add_workflow_node("sink", "sink", None, 200.0, 0.0, &mut space_store_a, &mut identity).expect("spawn b");
        let mut store_b = OsWorkflowStore::new(store_a.document(), protocol::ActorId("workflow-peer-b".into())).expect("valid replicated workflow store fixture");

        let (backbone_a, backbone_b) = resolve_kernel_future(MemoryBackbone::pair("mem://reconcile-race", "mem://reconcile-race"));
        store_a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).expect("attach a");
        store_b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).expect("attach b");

        let document = store_a.snapshot().expect("projection");
        let node_a = document.graph.nodes.iter().find(|node| node.id == node_a_id).expect("node a");
        let node_b = document.graph.nodes.iter().find(|node| node.id == node_b_id).expect("node b");
        let source_node_id = node_a.id.clone();
        let source_port_id = node_a.outputs.first().expect("node a output port").id.clone();
        let target_node_id = node_b.id.clone();
        let target_port_id = node_b.inputs.first().expect("node b input port").id.clone();

        store_a.dispatch_apply(vec![workflow::WorkflowMutation::RemoveNode(workflow::RemoveNode { node_id: node_b_id.clone() })], &mut identity).expect("remove node b");
        store_b
            .dispatch_apply(vec![workflow::WorkflowMutation::ConnectPorts(workflow::ConnectPorts {
                edge: WorkflowEdge { id: "edge-race".into(), source_node_id: source_node_id.clone(), source_port_id, target_node_id: target_node_id.clone(), target_port_id, contract: resolve_kernel_future(placeholder_media_contract("draw")) },
            })], &mut identity)
            .expect("wire edge to node b");
        store_a.tick().expect("pump a");
        store_b.tick().expect("pump b");

        let history_a = store_a.document();
        let history_b = store_b.document();
        let (converged_a, conflicts_a) = store_a.snapshot_with_conflicts().expect("snapshot with conflicts a");
        let (converged_b, conflicts_b) = store_b.snapshot_with_conflicts().expect("snapshot with conflicts b");
        assert_eq!(converged_a, converged_b, "both peers must converge on the same reconciled document");
        assert!(converged_a.graph.nodes.iter().all(|node| node.id != node_b_id), "node b must stay removed");
        assert!(converged_a.graph.edges.iter().all(|edge| edge.target_node_id != target_node_id), "the edge wired to the deleted node must be dropped, not dangling");
        assert!(
            conflicts_a.iter().any(|conflict| conflict.code == semio_framework_diagnostic::FaultCode::new("workflow/edge-orphaned") && conflict.level == semio_framework_diagnostic::Severity::Warning && conflict.target == vec!["edge-race".to_string()]),
            "dropping the dangling edge must surface a Warning-level conflict targeting the dropped edge"
        );
        assert_eq!(conflicts_a, conflicts_b, "both peers must report the same reconciliation conflicts");
        assert_eq!(store_a.document(), history_a, "reconciling a read projection must not add a history row (load repair only)");
        assert_eq!(store_b.document(), history_b, "reconciling a read projection must not add a history row (load repair only)");
    }

    // 🫀️ The old `presence_upserts_prunes_and_excludes_self` test exercised the deleted `presence:`
    // backbone-URI hack (`write_os_presence`/`read_os_presence_peers`). Presence now flows through
    // the semio_hub's `PresencePeer`/`HubServerFrame::Presence` frames and `framework/sync`'s
    // `ArtifactEvent::Presence` — see `framework/product/os/semio_hub/rs/bin.rs` and
    // `framework/sync/rs/lib.rs` for that layer's own coverage.
