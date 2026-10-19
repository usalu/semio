    /// 🧰️ Host fixtures: the minimal app definition, the seeded draw and sink plugins and the space and workflow store fixtures.

    /// 🧷️ Minimal `AppDefinition` for registry tests — every field but `io`/`document` is filler;
    /// `register_app_io` only reads `.id`/`.label`/`.io` (see `workflow::workflow_node_for_app`).
    fn test_app_definition(id: &str, label: &str, artifact_schema: &str, ports: Vec<semio_framework::MediaPortSpec>) -> AppDefinition {
        AppDefinition {
            id: id.into(),
            role: AppRole::Editor,
            dialect: ArtifactDialect { artifact_kind: format!("s.test.{id}"), standard: "1".into(), subset: "*".into() },
            label: LocalizedLabel::data(label),
            breadcrumb: vec!["semio".into(), id.into()],
            icon_id: None,
            controller_id: format!("{id}-play"),
            modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
            default_mode_id: "edit".into(),
            window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                initial_utility_id: None,
                id: id.into(),
                label: LocalizedLabel::data(label),
                body_key: id.into(),
                surface_kind: SurfaceKind::Canvas2d,
                icon_id: "app-window".into(),
                options: semio_framework_ui_declaration::layout::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                interactions: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
            actions: vec![],
            panel_tabs: vec![],
            keybindings: vec![],
            utilities: Vec::new(),
            tools: Vec::new(),
            commands: Vec::new(),
            interactions: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_breadcrumbs: HashMap::new(),
            introduction: None,
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: resolve_kernel_future(semio_framework::ConfigSpec::empty()),
            command_grammar: resolve_kernel_future(semio_framework::CommandGrammar::empty()),
            io: resolve_kernel_future(
                resolve_kernel_future(semio_framework::AppIo::from_artifact(
                    artifact_schema,
                    MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    semio_framework::ArtifactPresentation { id: id.into(), name: label.into(), dimension: "2d".into(), component_kind: id.into() },
                ))
                .with_ports(ports),
            ),
            tutorials: Vec::new(),
            fault_notices: Vec::new(),
        }
    }

    fn seed_draw_plugin() {
        crate::registry::register_app_io("draw", &test_app_definition("draw", "Draw", "draw.document", Vec::new()));
    }

    /// 🧲️ `draw` declares zero extra output ports, so tests that need to wire an edge *into* a
    /// spawned node register this minimal sink alongside it, wired via the implicit `document:*`
    /// ports every app carries (see `AppIo::all_ports`).
    fn seed_sink_plugin() {
        crate::registry::register_app_io("sink", &test_app_definition("sink", "Sink", "sink.document", Vec::new()));
    }

    fn test_space_store() -> OsSpaceStore {
        let envelope = create_document_envelope(space::S_SPACE_SCHEMA, "space", space::empty_space_snapshot("Space", space::SpaceKind::Studio, space::SpaceVisibility::Private), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        resolve_kernel_future(ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(store::os_spr::LOCAL_ACTOR_ID.into()))).expect("valid artifact store fixture")
    }

    fn test_workflow_store() -> OsWorkflowStore {
        OsWorkflowStore::new(create_backbone_document(workflow::S_WORKFLOW_SCHEMA, "workflow", "Workflow", resolve_kernel_future(workflow::empty_workflow_snapshot())), protocol::ActorId(store::os_spr::LOCAL_ACTOR_ID.into())).expect("valid workflow store fixture")
    }

    /// 🧹️ Retires a fixture-owned workflow store under page grants before its shell is released.
    fn close_workflow_fixture(store: &mut OsWorkflowStore) {
        drop(store.inner.detach_backbone().expect("fixture transport detaches"));
        for _ in 0..65_536 {
            if store.inner.close_owned_terminal_is_empty() { return; }
            let demand = store.inner.close_owned_demands(4096).expect("fixture owner quotes its next turn");
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            store.inner.close_owned_step(grant).expect("fixture owner closes under its quoted grant");
        }
        panic!("workflow fixture did not reach terminal emptiness");
    }
