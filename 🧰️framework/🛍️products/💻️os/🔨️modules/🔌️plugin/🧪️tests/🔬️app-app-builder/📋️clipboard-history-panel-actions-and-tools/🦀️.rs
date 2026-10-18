    /// 📋️ Clipboard, history panel, actions and tools: the clipboard and history panel injection, the declared action kinds, duplicates, utilities, tools, the tool run reserved actions, action arguments and window kind references.

    #[semio_framework_async_macros::async_test]
    async fn build_definition_auto_injects_clipboard_actions_and_keybindings() {
        let definition = minimal_app("clipboard-app").await.build_definition();
        let clipboard_ids: HashSet<&str> = declared_actions(&definition).map(|c| c.id.as_str()).collect();
        assert!(clipboard_ids.contains("copy"));
        assert!(clipboard_ids.contains("cut"));
        assert!(clipboard_ids.contains("paste"));
        let copy_action = declared_actions(&definition).find(|a| a.id == "copy").expect("copy declared");
        assert_eq!(copy_action.kind, ActionKind::Clipboard);
        let paste_action = declared_actions(&definition).find(|a| a.id == "paste").expect("paste declared");
        assert!(paste_action.args.iter().any(|arg| arg.id == "anchor"));
        let copy_binding = definition.keybindings.iter().find(|binding| binding.keys == "mod+c").expect("copy keybinding auto-injected");
        assert_eq!(copy_binding.action.action, "copy");
        assert_eq!(copy_binding.action.controller_id, canonical_test_app_id("clipboard-app").await);
        let paste_binding = definition.keybindings.iter().find(|binding| binding.keys == "mod+v").expect("paste keybinding auto-injected");
        assert_eq!(paste_binding.action.action, "paste");
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_auto_injects_the_history_panel_tab_and_filter_action() {
        let definition = minimal_app("history-panel-app").await.build_definition();
        let mut history_panel_tab_ids: Vec<&str> = Vec::new();
        for tab in definition.panel_tabs.iter() {
            history_panel_tab_ids.push(tab.id());
        }
        assert!(history_panel_tab_ids.iter().any(|id| *id == semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_HISTORY_ID));
        let action_ids: HashSet<&str> = declared_actions(&definition).map(|a| a.id.as_str()).collect();
        assert!(action_ids.contains(REVERT_TO_COMMAND_ACTION_ID));
        assert!(action_ids.contains(SET_HISTORY_COMMAND_FILTER_ACTION_ID));
        let revert = declared_actions(&definition).find(|a| a.id == REVERT_TO_COMMAND_ACTION_ID).expect("revertToCommand declared");
        assert_eq!(revert.kind, ActionKind::History);
        assert!(!revert.in_palette);
        let filter = declared_actions(&definition).find(|a| a.id == SET_HISTORY_COMMAND_FILTER_ACTION_ID).expect("setHistoryCommandFilter declared");
        assert_eq!(filter.kind, ActionKind::View);
        assert!(!filter.in_palette);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_does_not_duplicate_a_manually_declared_history_panel_tab() {
        let definition = minimal_app("manual-history-app").await.panel_tab(semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_HISTORY_ID, LocalizedLabel::data("Custom History"), PanelGroup::Settings, "custom.history").await.build_definition();
        let mut manual_history_tab_ids: Vec<&str> = Vec::new();
        for t in definition.panel_tabs.iter() {
            manual_history_tab_ids.push(t.id());
        }
        assert_eq!(manual_history_tab_ids.iter().filter(|id| **id == semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_HISTORY_ID).count(), 1);
        let manual_history_tab_idx = manual_history_tab_ids.iter().position(|id| *id == semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_HISTORY_ID).expect("history tab present");
        let tab = &definition.panel_tabs[manual_history_tab_idx];
        assert_eq!(tab.body_key.as_deref(), Some("custom.history"));
    }

    #[semio_framework_async_macros::async_test]
    async fn operation_view_and_shell_actions_are_declared_with_their_kind() {
        let definition = minimal_app("typed-actions-app")
            .await
            .mutation("addLayer", LocalizedLabel::data("Add Layer"))
            .await
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::data("Set Camera"), ActionKind::View, "camera"))
            .await
            .shell_action("exportPng", LocalizedLabel::data("Export PNG"))
            .await
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
            .await
            .build_definition();
        let by_id = |id: &str| declared_actions(&definition).find(|c| c.id == id).expect("declared");
        assert_eq!(by_id("addLayer").kind, ActionKind::Mutation);
        assert_eq!(by_id("setCamera").kind, ActionKind::View);
        assert_eq!(by_id("exportPng").kind, ActionKind::Shell);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_duplicate_action_ids() {
        let __base = minimal_app("dupe-action-app").await;
        let __chain = __base.mutation("addLayer", LocalizedLabel::data("Add Layer")).await.mutation("addLayer", LocalizedLabel::data("Add Layer Again")).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_keybinding_for_undeclared_action_once_opted_in() {
        let __base = minimal_app("undeclared-keybinding-app").await;
        let __chain = __base.mutation("addLayer", LocalizedLabel::data("Add Layer")).await.keybinding("mod+l", "removeLayer").await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_utilities_injects_set_active_utility_action_and_keybinding() {
        use semio_framework::{ActionKind, SET_ACTIVE_UTILITY_ACTION_ID, UtilityDefinition};
        let definition = minimal_app("utility-app")
            .await
            .utility(UtilityDefinition { keys: Some("b".into()), ..UtilityDefinition::new("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush) })
            .await
            .utility_simple("eraser", LocalizedLabel::data("Eraser"), IconName::Eraser)
            .await
            .build_definition();
        let set_active_utility = declared_actions(&definition).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility injected");
        assert_eq!(set_active_utility.kind, ActionKind::View);
        assert!(!set_active_utility.in_palette);
        let binding = definition.keybindings.iter().find(|binding| binding.keys == "b").expect("utility keybinding auto-injected");
        assert_eq!(binding.action.action, SET_ACTIVE_UTILITY_ACTION_ID);
        assert_eq!(binding.action.args, Some(DslValue::Object(vec![("utilityId".into(), DslValue::String("brush".into()))])));
    }

    #[semio_framework_async_macros::async_test]
    async fn no_utilities_means_no_set_active_utility_action() {
        use semio_framework::SET_ACTIVE_UTILITY_ACTION_ID;
        let definition = minimal_app("no-utility-app").await.build_definition();
        assert!(!declared_actions(&definition).any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_accepts_and_resolves_mode_tools() {
        use semio_framework::ToolRef;
        let definition = minimal_app("tool-app").await.tool_simple("fill", LocalizedLabel::data("Fill"), IconName::PaintBucket).await.mode_tools("edit", vec![ToolRef::new("fill").await]).await.build_definition();
        assert_eq!(definition.tools.len(), 1);
        assert_eq!(definition.modes[0].tools, vec![ToolRef::new("fill").await]);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_mode_tool_ref_to_undeclared_tool() {
        use semio_framework::ToolRef;
        let __base = minimal_app("undeclared-mode-tool-app").await;
        let __chain = __base.mode_tools("edit", vec![ToolRef::new("missing").await]).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_tool_referenced_by_no_mode() {
        let __base = minimal_app("orphan-tool-app").await;
        let __chain = __base.tool_simple("fill", LocalizedLabel::data("Fill"), IconName::PaintBucket).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err(), "a declared tool must be referenced by mode_tools on at least one mode");
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_tools_injects_set_active_tool_action_and_keybinding() {
        use semio_framework::{ActionKind, SET_ACTIVE_TOOL_ACTION_ID, ToolDefinition, ToolRef};
        let definition = minimal_app("tool-keybinding-app")
            .await
            .tool(ToolDefinition { keys: Some("f".into()), ..ToolDefinition::new("fill", LocalizedLabel::data("Fill"), IconName::PaintBucket).await })
            .await
            .mode_tools("edit", vec![ToolRef::new("fill").await])
            .await
            .build_definition();
        let set_active_tool = declared_actions(&definition).find(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID).expect("setActiveTool injected");
        assert_eq!(set_active_tool.kind, ActionKind::View);
        assert!(!set_active_tool.in_palette);
        let binding = definition.keybindings.iter().find(|binding| binding.keys == "f").expect("tool keybinding auto-injected");
        assert_eq!(binding.action.action, SET_ACTIVE_TOOL_ACTION_ID);
        assert_eq!(binding.action.args, Some(DslValue::Object(vec![("toolId".into(), DslValue::String("fill".into()))])));
    }

    #[semio_framework_async_macros::async_test]
    async fn no_tools_means_no_set_active_tool_action() {
        use semio_framework::SET_ACTIVE_TOOL_ACTION_ID;
        let definition = minimal_app("no-tool-app").await.build_definition();
        assert!(!declared_actions(&definition).any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_a_tool_run_injects_the_reserved_actions_and_their_chords_without_stealing_escape() {
        use semio_framework::{ToolDefinition, ToolRef};
        use semio_framework_tool_run::{JobKindId, ToolRunDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunTraceKind, TOOL_RUN_ACTION_IDS};
        let run = ToolRunDefinition { receiving: semio_framework_tool_run::ToolRunReceivingLimits::REPORTS,
            mutating: true,
            rebase: ToolRunRebasePolicy::Revalidate,
            reconfigure: ToolRunReconfigurePolicy::Resume,
            unit: LocalizedLabel::native("candidates", "Kandidaten"),
            stages: vec![ToolRunStageDefinition { id: "test".into(), label: LocalizedLabel::native("Testing", "Prüfen") }],
            counters: vec![],
            reasons: vec![],
            trace: ToolRunTraceKind::Instance3d,
            run_job: JobKindId::new("fill.run"),
            revalidate_job: None,
            settings: ToolRunSettingsReads::default(),
            windows: Vec::new(),
            member: None,
        };
        let plain = minimal_app("tool-without-run-app").await.tool(ToolDefinition::new("fill", LocalizedLabel::data("Fill"), IconName::PaintBucket).await).await.mode_tools("edit", vec![ToolRef::new("fill").await]).await.build_definition();
        assert!(!declared_actions(&plain).any(|action| TOOL_RUN_ACTION_IDS.contains(&action.id.as_str())));
        let interactive = minimal_app("tool-run-app").await.tool(ToolDefinition { run: Some(run), ..ToolDefinition::new("fill", LocalizedLabel::data("Fill"), IconName::PaintBucket).await }).await.mode_tools("edit", vec![ToolRef::new("fill").await]).await.build_definition();
        let window_actions: Vec<&str> = interactive.window_kinds.first().actions.iter().map(|action| action.id.as_str()).collect();
        for id in TOOL_RUN_ACTION_IDS {
            assert!(window_actions.contains(&id), "{id} injected");
        }
        let bound = |keys: &str| interactive.keybindings.iter().find(|binding| binding.keys == keys).map(|binding| binding.action.action.clone());
        assert_eq!(bound("mod+enter").as_deref(), Some("toolRunStart"));
        assert_eq!(bound("mod+alt+enter").as_deref(), Some("toolRunPause"));
        assert_eq!(bound("mod+alt+arrowright").as_deref(), Some("toolRunStep"));
        assert_eq!(bound("mod+.").as_deref(), Some("toolRunAbort"));
        assert_eq!(bound("mod+shift+enter").as_deref(), Some("toolRunFinalize"));
        assert!(interactive.keybindings.iter().all(|binding| binding.action.action != "toolRunDismiss" && binding.action.action != "toolRunResume"));
        assert_ne!(bound("escape").as_deref(), Some("toolRunDismiss"), "escape dismisses only with the run panel focused");
        let tool_run_tab = |definition: &AppDefinition| definition.panel_tabs.iter().find(|tab| matches!(&tab.kind, PanelTabKind::App(id) if id == semio_framework_ui_layout::layout::FRAMEWORK_PANEL_TAB_TOOL_RUN_ID)).map(|tab| (tab.group, tab.body_key.clone()));
        assert_eq!(tool_run_tab(&plain), None, "an app without a tool run gets no ToolRun panel tab");
        assert_eq!(tool_run_tab(&interactive), Some((PanelGroup::Details, Some(semio_framework_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY.to_string()))), "an app declaring a tool run mounts the framework ToolRun panel as a details tab");
    }

    #[semio_framework_async_macros::async_test]
    async fn action_args_attaches_declared_arguments() {
        let definition = minimal_app("args-app").await.mutation("resize", LocalizedLabel::data("Resize")).await.action_args("resize", vec![ActionArgDef::slider("scale", LocalizedLabel::data("Scale"), 0.0, 4.0).required()]).await.interactive_jobs(semio_framework::InteractiveJobClassification::Migrated).await.build_definition();
        let resize = declared_actions(&definition).find(|action| action.id == "resize").expect("declared");
        assert_eq!(resize.args.len(), 1);
        assert_eq!(resize.args[0].id, "scale");
        assert!(resize.args[0].required);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_window_kind_utility_referencing_undeclared_utility() {
        let __base = minimal_app("bad-utility-ref-app").await;
        let __chain = __base.utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush).await.window_kind_utilities("main", vec!["missing".into()]).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_window_kind_action_referencing_undeclared_action() {
        let __base = minimal_app("bad-action-ref-app").await;
        let __chain = __base.mutation("addLayer", LocalizedLabel::data("Add Layer")).await.window_kind_action_refs("main", vec!["removeLayer".into()]).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_carries_window_interactions_and_injects_framework_actions() {
        use semio_framework::{GranularityDefinition, HierarchyProvider, HoverSpec, INTERACTION_HOVER_ACTION_ID, InteractionDefinition, InteractionRef, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
        let definition = minimal_app("interaction-app")
            .await
            .interaction(InteractionDefinition {
                id: "world".into(),
                label: LocalizedLabel::data("World"),
                granularities: vec![GranularityDefinition { id: "object".into(), label: LocalizedLabel::data("Object"), icon_id: "box".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
            })
            .await
            .window_kind_interactions("main", vec![InteractionRef::new("world")])
            .await
            .build_definition();
        assert_eq!(definition.interactions.len(), 1);
        assert_eq!(definition.window_kinds.first().interactions, vec![InteractionRef::new("world")]);
        assert_eq!(declared_actions(&definition).find(|action| action.id == INTERACTION_HOVER_ACTION_ID).map(|action| action.kind), Some(ActionKind::Interaction));
        assert_eq!(definition.keybindings.iter().find(|binding| binding.keys == "escape").map(|binding| binding.action.action.as_str()), Some("clearSelection"));
    }
