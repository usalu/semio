    /// 🛟️ Rollback, contributions and recovery: the invalid manifest rollback, the contribution tracking and the recovery UI of a quarantined plugin.

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
                initial_utility_id: None,
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
            io: semio_framework::AppIo::default(),
            tutorials: Vec::new(),
            fault_notices: Vec::new(),
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
                hosted_artifact_kinds: Vec::new(),
                dependencies: vec![],
                contributions: vec![],
            },
            artifact_uri: "program://draw".into(),
        });
        let instance_id = host.create_instance("draw-play", "{}".into(), ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("instance");
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
                hosted_artifact_kinds: Vec::new(),
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
                hosted_artifact_kinds: Vec::new(),
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
                hosted_artifact_kinds: Vec::new(),
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
