mod tests {
    use super::*;
    use crate::workflow::{MediaContract, WorkflowEdge, WorkflowPosition, empty_workflow, placeholder_media_contract, validate_workflow};
    use semio_framework::{AppRole, ArtifactDialect, MediaClass, MediaForm, MediaType, MediaWireFormat, ModeDefinition, PluginManifest, WindowKindDefinition};
    use std::sync::Arc;
    use store::{MemoryBackbone, MemoryBackbonePort};
    use ui_wgpu::wgpu::{LocalizedLabel, SurfaceKind};

    #[test]
    fn loads_plugin_apps_into_registry() {
        let mut host = PluginHost::new();
        let manifest = PluginManifest {
            plugin_id: "draw".into(),
            label: "Draw".into(),
            version: "0.1.0".into(),
            apps: vec![AppDefinition {
                id: "draw-play".into(),
                role: AppRole::Editor,
                dialect: ArtifactDialect { artifact_kind: "s.test.draw".into(), standard: "1".into(), subset: "*".into() },
                label: LocalizedLabel::data("Draw"),
                breadcrumb: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: LocalizedLabel::data("Canvas"),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    interactions: Vec::new(),
                    params_schema: None,
                    artifact_snapshot_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                }),
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
                io: semio_framework::AppIo::default(),
                tutorials: Vec::new(),
            }],
            capabilities: vec![],
            topic_contributions: vec![],
            examples: vec![],
            commands: vec![],
            artifact_kinds: vec![],
            dependencies: vec![],
            contributions: vec![],
        };
        host.load_plugin(LoadedProgram { plugin_id: "draw".into(), manifest, artifact_uri: "program://draw".into() });
        assert_eq!(host.apps().len(), 1);
    }

    #[test]
    fn hot_swap_bumps_instance_generation_and_tracks_app_changes() {
        let mut host = PluginHost::new();
        let draw_app = AppDefinition {
            id: "draw-play".into(),
            role: AppRole::Editor,
            dialect: ArtifactDialect { artifact_kind: "s.test.draw".into(), standard: "1".into(), subset: "*".into() },
            label: LocalizedLabel::data("Draw"),
            breadcrumb: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
            default_mode_id: "edit".into(),
            window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                id: "composite".into(),
                label: LocalizedLabel::data("Canvas"),
                body_key: "composite".into(),
                surface_kind: SurfaceKind::Canvas2d,
                icon_id: "pen-tool".into(),
                options: ui_wgpu::wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                interactions: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: vec![],
            }),
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
            io: semio_framework::AppIo::default(),
            tutorials: Vec::new(),
        };
        let note_app = AppDefinition {
            id: "note-play".into(),
            role: AppRole::Editor,
            dialect: ArtifactDialect { artifact_kind: "s.test.note".into(), standard: "1".into(), subset: "*".into() },
            label: LocalizedLabel::data("Note"),
            breadcrumb: vec!["semio".into(), "note".into()],
            icon_id: None,
            controller_id: "note-play".into(),
            modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
            default_mode_id: "edit".into(),
            window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                id: "composite".into(),
                label: LocalizedLabel::data("Canvas"),
                body_key: "composite".into(),
                surface_kind: SurfaceKind::Canvas2d,
                icon_id: "pen-tool".into(),
                options: ui_wgpu::wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                interactions: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: vec![],
            }),
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
            io: semio_framework::AppIo::default(),
            tutorials: Vec::new(),
        };
        host.load_plugin(LoadedProgram {
            plugin_id: "draw".into(),
            manifest: PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "0.1.0".into(),
                apps: vec![draw_app.clone()],
                capabilities: vec![],
                topic_contributions: vec![],
                examples: vec![],
                commands: vec![],
                artifact_kinds: vec![],
                dependencies: vec![],
                contributions: vec![],
            },
            artifact_uri: "program://draw".into(),
        });
        let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
        let generation_before = host.instance(instance_id).expect("instance").generation;
        let event = host.hot_swap_plugin(LoadedProgram {
            plugin_id: "draw".into(),
            manifest: PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "0.2.0".into(),
                apps: vec![draw_app, note_app],
                capabilities: vec![],
                topic_contributions: vec![],
                examples: vec![],
                commands: vec![],
                artifact_kinds: vec![],
                dependencies: vec![],
                contributions: vec![],
            },
            artifact_uri: "program://draw".into(),
        });
        assert_eq!(event.added_apps, vec!["note-play".to_string()]);
        assert!(event.removed_apps.is_empty());
        assert_eq!(event.plugin_id, "draw");
        assert_eq!(event.version, "0.2.0");
        assert!(host.instance(instance_id).expect("instance").generation > generation_before, "hot swap must bump instance generation");
        assert_eq!(host.apps().len(), 2);
    }

    #[test]
    fn hot_swap_rollback_on_invalid_manifest_keeps_old_plugin() {
        let mut host = PluginHost::new();
        let draw_app = AppDefinition {
            id: "draw-play".into(),
            role: AppRole::Editor,
            dialect: ArtifactDialect { artifact_kind: "s.test.draw".into(), standard: "1".into(), subset: "*".into() },
            label: LocalizedLabel::data("Draw"),
            breadcrumb: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
            default_mode_id: "edit".into(),
            window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                id: "composite".into(),
                label: LocalizedLabel::data("Canvas"),
                body_key: "composite".into(),
                surface_kind: SurfaceKind::Canvas2d,
                icon_id: "pen-tool".into(),
                options: ui_wgpu::wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                interactions: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: vec![],
            }),
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
            io: semio_framework::AppIo::default(),
            tutorials: Vec::new(),
        };
        host.load_plugin(LoadedProgram {
            plugin_id: "draw".into(),
            manifest: PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "0.1.0".into(),
                apps: vec![draw_app],
                capabilities: vec![],
                topic_contributions: vec![],
                examples: vec![],
                commands: vec![],
                artifact_kinds: vec![],
                dependencies: vec![],
                contributions: vec![],
            },
            artifact_uri: "program://draw".into(),
        });
        let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
        let generation_before = host.instance(instance_id).expect("instance").generation;
        let event = host.hot_swap_plugin(LoadedProgram {
            plugin_id: "draw".into(),
            manifest: PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "".into(),
                apps: vec![],
                capabilities: vec![],
                topic_contributions: vec![],
                examples: vec![],
                commands: vec![],
                artifact_kinds: vec![],
                dependencies: vec![],
                contributions: vec![],
            },
            artifact_uri: "program://draw".into(),
        });
        assert_eq!(event.plugin_id, "draw");
        assert_eq!(event.version, "0.1.0");
        assert!(event.added_apps.is_empty());
        assert_eq!(host.apps().len(), 1);
        assert_eq!(host.instance(instance_id).expect("instance").generation, generation_before);
        assert_eq!(host.programs.get("draw").expect("plugin").manifest.version, "0.1.0");
    }

    #[test]
    fn contributions_track_plugin_load_and_hot_swap() {
        let mut host = PluginHost::new();
        let topic_contribution = TopicContribution::new(
            "playbook.blockKind",
            DslValue::object([
                ("appId".to_string(), DslValue::String("playbook-module-procedural".to_string())),
                ("blockKind".to_string(), DslValue::String("buildingComponent".to_string())),
                ("label".to_string(), DslValue::String("Building Component".to_string())),
                ("iconId".to_string(), DslValue::String("building".to_string())),
                ("defaultValueJson".to_string(), DslValue::String("{}".to_string())),
                ("paramsBodyKey".to_string(), DslValue::String("params".to_string())),
                ("previewBodyKey".to_string(), DslValue::String("preview".to_string())),
            ]),
        );
        host.load_plugin(LoadedProgram {
            plugin_id: "playbook-module-procedural".into(),
            manifest: PluginManifest {
                plugin_id: "playbook-module-procedural".into(),
                label: "Playbook Module Procedural".into(),
                version: "0.1.0".into(),
                apps: vec![],
                capabilities: vec![],
                topic_contributions: vec![topic_contribution.clone()],
                examples: vec![],
                commands: vec![],
                artifact_kinds: vec![],
                dependencies: vec![],
                contributions: vec![],
            },
            artifact_uri: "program://playbook-module-procedural".into(),
        });
        assert_eq!(host.contributions().len(), 1);
        assert_eq!(host.contributions()[0].plugin_id, "playbook-module-procedural");
        host.hot_swap_plugin(LoadedProgram {
            plugin_id: "playbook-module-procedural".into(),
            manifest: PluginManifest {
                plugin_id: "playbook-module-procedural".into(),
                label: "Playbook Module Procedural".into(),
                version: "0.2.0".into(),
                apps: vec![],
                capabilities: vec![],
                topic_contributions: vec![],
                examples: vec![],
                commands: vec![],
                artifact_kinds: vec![],
                dependencies: vec![],
                contributions: vec![],
            },
            artifact_uri: "program://playbook-module-procedural".into(),
        });
        assert!(host.contributions().is_empty());
    }

    #[test]
    fn recovery_ui_renders_actions_for_quarantined_plugin() {
        let mut host = PluginHost::new();
        host.quarantined.insert("draw".into());
        let ui = host.recovery_ui("draw");
        match ui {
            UiNode::Stack(stack) => assert_eq!(stack.children.len(), 5, "title + message + restart/disable/showDiagnostics buttons"),
            other => panic!("expected recovery stack, got {other:?}"),
        }
    }

    /// 🧷️ Minimal `AppDefinition` for registry tests — every field but `io`/`document` is filler;
    /// `register_app_io` only reads `.id`/`.label`/`.io` (see `workflow::workflow_node_for_app`).
    fn test_app_definition(id: &str, label: &str, document_schema: &str, ports: Vec<semio_framework::MediaPortSpec>) -> AppDefinition {
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
                id: id.into(),
                label: LocalizedLabel::data(label),
                body_key: id.into(),
                surface_kind: SurfaceKind::Canvas2d,
                icon_id: "app-window".into(),
                options: ui_wgpu::wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                interactions: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
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
                resolve_kernel_future(semio_framework::AppIo::from_document(
                    document_schema,
                    MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    semio_framework::ArtifactPresentation { id: id.into(), name: label.into(), dimension: "2d".into(), component_kind: id.into() },
                ))
                .with_ports(ports),
            ),
            tutorials: Vec::new(),
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
        let envelope = create_document_envelope(space::S_SPACE_SCHEMA, "space", space::empty_space_snapshot("Space", space::SpaceKind::Studio, space::SpaceVisibility::Private), None);
        resolve_kernel_future(ArtifactStore::new(envelope)).expect("valid artifact store fixture")
    }

    fn test_workflow_store() -> OsWorkflowStore {
        OsWorkflowStore::new(create_backbone_document(workflow::S_WORKFLOW_SCHEMA, "workflow", "Workflow", resolve_kernel_future(workflow::empty_workflow_snapshot()))).expect("valid workflow store fixture")
    }

    #[test]
    fn backbone_and_workflow_store_round_trips_preserve_outcomes_and_conflicts() {
        let mut store = test_workflow_store();
        store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Durable").expect("create one edit");
        let mut document = store.document();
        let edit_id = document.cursor.applied_edit_ids.last().expect("one applied edit").clone();
        let edit = document.vcs.edits.iter().find(|edit| edit.id == edit_id).expect("applied edit is persisted");
        let messages = vec![protocol::MutationMessage::warn("mutation.clamped", "durable host outcome").at(["parameters", "0"]).at_op(0)];
        document.edit_messages = vec![protocol::EditMessages { edit_id: edit_id.clone(), messages: messages.clone() }];
        let kind = protocol::ConflictKind::Degraded { edit_ids: vec![edit_id] };
        let timestamp = edit.mutation_meta.first().expect("operation metadata").timestamp;
        let mutation_ids = edit.mutation_meta.iter().map(|meta| meta.mutation_id.clone().expect("stable mutation identity")).collect::<Vec<_>>();
        let actors = vec![protocol::ActorId(edit.actor.clone().expect("stable edit actor"))];
        document.conflicts =
            vec![protocol::Conflict { id: resolve_kernel_future(protocol::ConflictId::new(&kind, &protocol::ArtifactId(document.id.clone()), &mutation_ids, &timestamp)), kind, status: protocol::ConflictStatus::Open, messages, actors, timestamp }];

        let payload = encode_backbone_payload(&document).expect("backbone payload encodes");
        let decoded: OsWorkflowArtifactDocument = decode_backbone_payload(&payload, workflow::S_WORKFLOW_SCHEMA).expect("backbone payload decodes");
        assert_eq!(decoded.cursor, document.cursor);
        assert_eq!(decoded.edit_messages, document.edit_messages);
        assert_eq!(decoded.conflicts, document.conflicts);

        let rebuilt = OsWorkflowStore::new(decoded).expect("workflow store rebuilds");
        let rebuilt_document = rebuilt.document();
        assert_eq!(rebuilt_document.edit_messages, document.edit_messages);
        assert_eq!(rebuilt_document.conflicts, document.conflicts);

        let text = export_backbone_dsl(&document).expect("backbone text encodes");
        let parsed = resolve_kernel_future(store::parse_document_text::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(&text.dsl, &text.ops)).expect("backbone text decodes");
        assert!(parsed.envelope.edit_messages.iter().eq(document.edit_messages.iter()));
        assert_eq!(parsed.envelope.conflicts, document.conflicts);
        assert_eq!(parsed.envelope.cursor.as_ref().expect("text carries explicit cursor"), &document.cursor);

        let mut invalid = document.clone();
        invalid.conflicts[0].id = protocol::ConflictId("conflict-invalid".into());
        assert!(encode_backbone_payload(&invalid).is_err(), "host binary persistence must reject a non-content-addressed conflict id");
        let malformed_text = text.ops.replacen(&document.conflicts[0].id.0, "conflict-invalid", 1);
        assert!(resolve_kernel_future(store::parse_document_text::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(&text.dsl, &malformed_text)).is_err(), "host text persistence must reject a non-content-addressed conflict id");
    }

    #[test]
    fn backbone_binary_text_and_workflow_store_preserve_the_complete_cursor() {
        let mut store = test_workflow_store();
        store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Committed").expect("first edit");
        resolve_kernel_future(store.inner.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("cursor checkpoint".into()), authors: Vec::new() })).expect("checkpoint");
        store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Undone").expect("second edit");
        store.dispatch_text("undo").expect("undo second edit");
        let document = store.document();
        assert!(!document.cursor.redo_edit_ids.is_empty(), "precondition: redo lane is populated");
        assert!(document.cursor.checkpoint_id.is_some(), "precondition: checkpoint is populated");

        let payload = encode_backbone_payload(&document).expect("binary encode");
        let decoded: OsWorkflowArtifactDocument = decode_backbone_payload(&payload, workflow::S_WORKFLOW_SCHEMA).expect("binary decode");
        assert_eq!(decoded.cursor, document.cursor);
        assert_eq!(OsWorkflowStore::new(decoded).expect("workflow rebuild").document().cursor, document.cursor);

        let text = export_backbone_dsl(&document).expect("text encode");
        let parsed = resolve_kernel_future(store::parse_document_text::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(&text.dsl, &text.ops)).expect("text decode");
        assert_eq!(parsed.envelope.cursor.as_ref().expect("text cursor"), &document.cursor);
    }

    #[test]
    fn spawns_and_removes_app_instances() {
        seed_draw_plugin();
        let mut space_store = test_space_store();
        let mut store = test_workflow_store();
        store.add_workflow_node("draw", "draw", None, 40.0, 40.0, &mut space_store).expect("spawn");
        assert_eq!(store.snapshot().expect("projection").graph.nodes.len(), 1);
        assert!(space_store.snapshot().expect("projection").programs.contains(&"draw".to_string()), "spawning a node must install its plugin into the owning space");
        store.dispatch_text("undo").expect("undo");
        assert_eq!(store.snapshot().expect("projection").graph.nodes.len(), 0);
    }

    #[test]
    fn adds_and_patches_studio_parameters() {
        let mut store = test_workflow_store();
        let parameter_id = store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Zoom").expect("add");
        store.patch_parameter(&parameter_id, &DslValue::object([("value".to_string(), DslValue::float(12.0)), ("max".to_string(), DslValue::float(10.0))])).expect("patch");
        match &store.snapshot().expect("projection").parameters[0] {
            workflow::WorkflowParameter::Numeric { value, .. } => assert_eq!(*value, 10.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn creates_and_lists_space_catalog_entries() {
        let port = Arc::new(OsBackbonePorts::Store(store::BackbonePorts::Memory(resolve_kernel_future(MemoryBackbonePort::new()))));
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
    fn validates_workflow_cycles() {
        assert!(validate_workflow(&resolve_kernel_future(empty_workflow())).ok);
    }

    #[test]
    fn concurrent_delete_and_wire_reconciles_without_a_dangling_edge() {
        seed_draw_plugin();
        seed_sink_plugin();
        let mut space_store_a = test_space_store();
        let mut store_a = test_workflow_store();
        let node_a_id = store_a.add_workflow_node("draw", "draw", None, 0.0, 0.0, &mut space_store_a).expect("spawn a");
        let node_b_id = store_a.add_workflow_node("sink", "sink", None, 200.0, 0.0, &mut space_store_a).expect("spawn b");
        let mut store_b = OsWorkflowStore::new(store_a.document()).expect("valid replicated workflow store fixture");

        let (backbone_a, backbone_b) = resolve_kernel_future(MemoryBackbone::pair("mem://reconcile-race", "mem://reconcile-race"));
        store_a.attach_backbone(store::Backbones::Memory(backbone_a)).expect("attach a");
        store_b.attach_backbone(store::Backbones::Memory(backbone_b)).expect("attach b");

        let document = store_a.snapshot().expect("projection");
        let node_a = document.graph.nodes.iter().find(|node| node.id == node_a_id).expect("node a");
        let node_b = document.graph.nodes.iter().find(|node| node.id == node_b_id).expect("node b");
        let source_node_id = node_a.id.clone();
        let source_port_id = node_a.outputs.first().expect("node a output port").id.clone();
        let target_node_id = node_b.id.clone();
        let target_port_id = node_b.inputs.first().expect("node b input port").id.clone();

        // 🏃️ Actor A deletes node B; actor B (unaware of the delete) concurrently wires a new edge
        // to a port on node B — the classic delete/wire race `reconcile` must clean up post-merge.
        store_a.dispatch_apply(vec![workflow::WorkflowMutation::RemoveNode(workflow::RemoveNode { node_id: node_b_id.clone() })]).expect("remove node b");
        store_b
            .dispatch_apply(vec![workflow::WorkflowMutation::ConnectPorts(workflow::ConnectPorts {
                edge: WorkflowEdge { id: "edge-race".into(), source_node_id: source_node_id.clone(), source_port_id, target_node_id: target_node_id.clone(), target_port_id, contract: resolve_kernel_future(placeholder_media_contract("draw")) },
            })])
            .expect("wire edge to node b");
        store_a.tick().expect("pump a");
        store_b.tick().expect("pump b");

        let (converged_a, conflicts_a) = store_a.snapshot_with_conflicts().expect("snapshot with conflicts a");
        let (converged_b, conflicts_b) = store_b.snapshot_with_conflicts().expect("snapshot with conflicts b");
        assert_eq!(converged_a, converged_b, "both peers must converge on the same reconciled document");
        assert!(converged_a.graph.nodes.iter().all(|node| node.id != node_b_id), "node b must stay removed");
        assert!(converged_a.graph.edges.iter().all(|edge| edge.target_node_id != target_node_id), "the edge wired to the deleted node must be dropped, not dangling");
        assert!(
            conflicts_a.iter().any(|conflict| conflict.code == dsl::FaultCode::new("workflow/edge-orphaned") && conflict.level == dsl::Severity::Warning && conflict.target == vec!["edge-race".to_string()]),
            "dropping the dangling edge must surface a Warning-level conflict targeting the dropped edge"
        );
        assert_eq!(conflicts_a, conflicts_b, "both peers must report the same reconciliation conflicts");
    }

    // 🫀️ The old `presence_upserts_prunes_and_excludes_self` test exercised the deleted `presence:`
    // backbone-URI hack (`write_os_presence`/`read_os_presence_peers`). Presence now flows through
    // the semio_hub's `PresencePeer`/`HubServerFrame::Presence` frames and `framework/sync`'s
    // `ArtifactEvent::Presence` — see `framework/product/os/semio_hub/rs/bin.rs` and
    // `framework/sync/rs/lib.rs` for that layer's own coverage.

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
            yields: "puzzle.2d.fixture".into(),
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
                    kind_id: Some("puzzle.2d.fixture".into()),
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
                    kind_id: Some("puzzle.2d.fixture".into()),
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
            contract: MediaContract { kind_id: "puzzle.2d.fixture".into(), media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, wire: MediaWireFormat::Document { schema: "puzzle.2d.fixture".into() }, conversion: None },
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
                yields: "puzzle.2d.fixture".into(),
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
                    kind_id: "puzzle.2d.fixture".into(),
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
        let envelope = create_document_envelope(workflow::S_WORKFLOW_SCHEMA, "workflow-text-test", resolve_kernel_future(workflow::empty_workflow_snapshot()), None);
        let mut store = resolve_kernel_future(ArtifactStore::new(envelope)).expect("valid artifact store fixture");
        resolve_kernel_future(store.dispatch(ArtifactCommand::Apply { mutations: vec![workflow::WorkflowMutation::UpdateNodePorts(workflow::UpdateNodePorts {})], description: None })).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
    // #endregion 🔖️DslAndOpText

    //#region 🔖️ExtensionInstall
    /// 🚫️async: E5 executor bridge — test-only (`📌️important.md` R4 clause 5: a `#[test] fn`
    /// body is a sanctioned executor entry point). Separate from `🎠️activation/🦀️.rs`'s one
    /// PRODUCTION bridge in this same crate — R2's "at most one per crate" governs production
    /// code, and R4 clause 5 explicitly does not count a test bridge against that census.
    /// `store::extension::pack`/`verify`/`content_hash` and this module's own
    /// `install_extension_package` are I/O-free (pure zip/hash work over in-memory bytes), so
    /// they resolve on the first poll by construction.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
        let mut cx = Context::from_waker(&waker);
        let mut future = std::pin::pin!(future);
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("block_on: extension install/pack futures are documented I/O-free"),
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn sample_extension_sxt(extension_id: &str, extends: &str, capabilities: Vec<String>) -> Vec<u8> {
        let manifest = store::extension::ExtensionPackageManifest {
            extension_id: extension_id.into(),
            directory_name: "🧩️sample-extension".into(),
            label: extension_id.into(),
            version: "0.1.0".into(),
            extends: extends.into(),
            capabilities,
            topic_contributions: semio_framework_os_kernel::json::Value::Array(Vec::new()),
            dependencies: vec![store::extension::PackagePluginDependency { plugin_id: extends.into(), version: "^1.0.0".into() }],
            contributions: semio_framework_os_kernel::json::Value::Array(Vec::new()),
            package_format: store::extension::EXTENSION_PACKAGE_FORMAT,
        };
        let component = b"\0asm\x01\x00\x00\x00fake-component".to_vec();
        block_on(store::extension::pack(&manifest, &component, &[])).expect("pack a valid sample .sxt")
    }

    /// 🧫️ Installs 30 real `.sxt` packages (real zip bytes, real blake3 hash — not a mock)
    /// across 3 distinct parent plugins and proves `extensions_extending_plugin` returns
    /// exactly the matching subset for each, with none for a plugin that installed nothing —
    /// the actual install→query pipeline, not just the pure filter (that claim, at the scale
    /// fixture's full 2,500-descriptor shape, is proven separately by
    /// `🧰️framework/🔨️modules/🎠️kernel`'s own `extensions_extending` test and this packet's
    /// standalone verification script — see the report).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn install_extension_package_registers_descriptors_queryable_by_extends() {
        let mut host = PluginHost::new();
        for i in 0..30 {
            let extends = format!("plugin-{}", i % 3);
            let bytes = sample_extension_sxt(&format!("ext-{i}"), &extends, vec!["storage.read".into()]);
            let installed = block_on(host.install_extension_package(&bytes)).expect("valid sample .sxt installs");
            assert_eq!(installed.manifest.extension_id, format!("ext-{i}"));
            assert!(!installed.content_hash.is_empty());
        }

        for plugin_index in 0..3 {
            let plugin_id = format!("plugin-{plugin_index}");
            let matched = host.extensions_extending_plugin(&plugin_id);
            assert_eq!(matched.len(), 10, "10 of 30 synthetic extensions extend {plugin_id}");
            assert!(matched.iter().all(|installed| installed.manifest.extends == plugin_id));
        }
        assert!(host.extensions_extending_plugin("plugin-nonexistent").is_empty());
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn install_extension_package_rejects_extends_mismatch() {
        // 🩹️ `sample_extension_sxt` always writes a matching dependency; to exercise the
        // rejection path this test packs a manifest whose `extends` disagrees with its own
        // first dependency directly, rather than corrupting bytes (which `verify`'s own
        // envelope/zip checks would reject for an unrelated reason).
        let mismatched_manifest = store::extension::ExtensionPackageManifest {
            extension_id: "ext-mismatch".into(),
            directory_name: "⚖️ext-mismatch".into(),
            label: "Ext Mismatch".into(),
            version: "0.1.0".into(),
            extends: "flow".into(),
            capabilities: vec![],
            topic_contributions: semio_framework_os_kernel::json::Value::Array(Vec::new()),
            dependencies: vec![store::extension::PackagePluginDependency { plugin_id: "cad".into(), version: "^1.0.0".into() }],
            contributions: semio_framework_os_kernel::json::Value::Array(Vec::new()),
            package_format: store::extension::EXTENSION_PACKAGE_FORMAT,
        };
        let manifest_bytes_source = block_on(store::extension::pack(&mismatched_manifest, b"\0asm\x01\x00\x00\x00x", &[])).expect("pack a structurally-valid but contract-violating .sxt");

        let mut host = PluginHost::new();
        let error = block_on(host.install_extension_package(&manifest_bytes_source)).expect_err("extends != dependencies[0].plugin_id must be rejected at install time");
        assert!(matches!(error, ExtensionInstallError::ExtendsMismatch { extension_id, extends, actual } if extension_id == "ext-mismatch" && extends == "flow" && actual == "cad"));
        assert!(host.extensions_extending_plugin("flow").is_empty(), "a rejected install must not register a descriptor");
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn extension_capabilities_scoped_to_parent_drops_what_the_parent_lacks() {
        let mut host = PluginHost::new();
        let bytes = sample_extension_sxt("ext-caps", "cad", vec!["storage.read".into(), "storage.write".into()]);
        let installed = block_on(host.install_extension_package(&bytes)).expect("valid sample .sxt installs");

        let parent_effective = vec!["storage.read".to_string(), "http:example.com".to_string()];
        let scoped = host.extension_capabilities_scoped_to_parent(&installed, &parent_effective);
        assert_eq!(scoped, vec!["storage.read"], "storage.write is not in the parent's effective set");
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn uninstall_extension_package_removes_it_from_the_query() {
        let mut host = PluginHost::new();
        let bytes = sample_extension_sxt("ext-removable", "cad", vec![]);
        block_on(host.install_extension_package(&bytes)).expect("valid sample .sxt installs");
        assert_eq!(host.extensions_extending_plugin("cad").len(), 1);

        let removed = host.uninstall_extension_package("ext-removable").expect("previously installed");
        assert_eq!(removed.manifest.extension_id, "ext-removable");
        assert!(host.extensions_extending_plugin("cad").is_empty());
        assert!(host.uninstall_extension_package("ext-removable").is_none(), "uninstall is not idempotent-returning on a second call");
    }
    //#endregion 🔖️ExtensionInstall
}
