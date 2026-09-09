mod app_builder_tests {
    use super::*;
    use ui_wgpu::wgpu::LocalizedLabel;
    use ui_wgpu::wgpu::create_default_layout;

    /// 🪪️ Contract §1 fixture — a canonical id built via `surface_app_id` from a fixture `Dialect`,
    /// not a hand-written pre-migration string. One shared helper rather than one dialect per test.
    async fn canonical_test_app_id(slug: &str) -> String {
        surface_app_id(&ArtifactDialect { artifact_kind: format!("s.test.app-builder.{slug}"), standard: "1".into(), subset: "*".into() }, AppRole::Editor)
    }

    /// 🪦️ REMOVED (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, registrar):
    /// `build_definition_rejects_{,dialog_}select_arg_with_no_options` asserted that a Select
    /// argument declaring zero options is rejected. After the peer `ActionArgDef` redesign
    /// (`control` field -> `control()` derived from `ArgSchema`), `ActionArgControl::Select` is
    /// only ever produced when `options` is non-empty, so "a Select with no options" is
    /// structurally unrepresentable and the rejection can never fire. The tests were asserting
    /// an impossible outcome, not lost coverage — illegal state made unconstructible beats
    /// validating it. The defensive asserts in `validate_arg_defs` are kept as a tripwire in
    /// case `control()` ever widens again.
    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_layout_with_unknown_window_kind() {
        let base = App::builder("bad-app", LocalizedLabel::data("Bad")).await;
        let __chain = base
            .document(["semio", "bad"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .mode_tools("edit", vec![])
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "bad.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .default_layout(create_default_layout(&["missing".into()], "row", None, None))
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_is_sync_and_accepts_valid_manifest() {
        let definition = App::builder(canonical_test_app_id("good-app").await, LocalizedLabel::data("Good"))
            .await
            .document(["semio", "good"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .mode_tools("edit", vec![])
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "good.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .panel_tab("framework.panel.artifact", LocalizedLabel::data("Document"), PanelGroup::Workbench, "good.document")
            .await
            .default_layout(create_default_layout(&["main".into()], "row", None, None))
            .await
            .build_definition();
        assert_eq!(definition.window_kinds.len(), 1);
        assert_eq!(definition.window_kinds.iter().next().map(|kind| kind.icon_id.as_str()), Some("app-window"));
        assert_eq!(definition.modes.first().icon_id.as_str(), "pencil");
        // 🕰️ 1 declared + the auto-injected framework History tab.
        assert_eq!(definition.panel_tabs.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn catalog_chrome_icons_resolve_to_vendored_icon_names() {
        assert_eq!(IconName::from("menu").as_str(), "list");
        assert_eq!(IconName::from("square-pen").as_str(), "pencil");
        assert_eq!(IconName::from("trees").as_str(), "list-tree");
        let definition = App::builder(canonical_test_app_id("icon-app").await, LocalizedLabel::data("Icon"))
            .await
            .document(["semio", "icon"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .mode_tools("edit", vec![])
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "icon.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .default_layout(create_default_layout(&["main".into()], "row", None, None))
            .await
            .build_definition();
        assert_eq!(definition.modes.first().icon_id.as_str(), "pencil");
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_terminology_document_for_undeclared_terminology() {
        let base = App::builder("bad-terminology-app", LocalizedLabel::data("Bad")).await;
        let __chain = base
            .document(["semio", "bad"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .mode_tools("edit", vec![])
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "bad.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .default_layout(create_default_layout(&["main".into()], "row", None, None))
            .await
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Bad"])
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_accepts_declared_terminology_document() {
        let definition = App::builder(canonical_test_app_id("good-terminology-app").await, LocalizedLabel::data("Good"))
            .await
            .document(["semio", "good"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .mode_tools("edit", vec![])
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "good.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .default_layout(create_default_layout(&["main".into()], "row", None, None))
            .await
            .terminology("reuse")
            .await
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Aggregator"])
            .await
            .build_definition();
        assert_eq!(definition.terminology_breadcrumbs.get("reuse").map(Vec::as_slice), Some(["Entwerfen mit Bestand".to_string(), "Aggregator".to_string()].as_slice()));
    }

    async fn minimal_app(slug: &str) -> AppBuilder {
        App::builder(canonical_test_app_id(slug).await, LocalizedLabel::data("App"))
            .await
            .document(["semio", slug])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), format!("{slug}.main"), SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
    }

    #[semio_framework_async_macros::async_test]
    async fn release_catalog_rejects_unclassified_and_retains_explicit_non_ui_dispositions() {
        use semio_framework::InteractiveJobClassification::{BatchOnlyPendingRewrite, Deleted, ForbiddenFromUi, Migrated, Unclassified};

        let mut unclassified = ActionDefinition::bounded_catalog("blockedAction", LocalizedLabel::data("Blocked"), ActionKind::Mutation);
        unclassified.semantics.execution.interactive_job = Unclassified;
        let error = minimal_app("unclassified-action").await.action_with(unclassified).await.try_build_definition().expect_err("unclassified declarations must block release");
        assert_eq!(error.code, "app-definition.interactive-job-classification");

        let migrated = minimal_app("migrated-inventory")
            .await
            .action_with(ActionDefinition::bounded_catalog("migratedAction", LocalizedLabel::data("Migrated"), ActionKind::Mutation))
            .await
            .interactive_jobs(Migrated)
            .await
            .try_build_definition()
            .expect("an explicit app-wide disposition classifies its existing inventory");
        assert!(migrated.window_kinds.iter().flat_map(|window| &window.actions).all(|action| action.semantics.execution.interactive_job == Migrated));

        for (index, classification) in [BatchOnlyPendingRewrite, ForbiddenFromUi, Deleted].into_iter().enumerate() {
            let mut action = ActionDefinition::bounded_catalog("blockedAction", LocalizedLabel::data("Blocked"), ActionKind::Mutation);
            action.semantics.execution.interactive_job = classification;
            let definition = minimal_app(&format!("classified-action-{index}")).await.action_with(action).await.try_build_definition().expect("an explicit non-UI disposition is valid inventory data");
            let retained = definition.window_kinds.iter().flat_map(|window| &window.actions).find(|entry| entry.id == "blockedAction").expect("classified action retained");
            assert_eq!(retained.semantics.execution.interactive_job, classification);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_auto_injects_history_actions_and_keybindings() {
        let definition = minimal_app("history-app").await.build_definition();
        let history_ids: HashSet<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).map(|c| c.id.as_str()).collect();
        assert!(history_ids.contains("undo"));
        assert!(history_ids.contains("redo"));
        assert!(history_ids.contains("commitCheckpoint"));
        assert!(history_ids.contains("createAlternative"));
        assert!(history_ids.contains("switchAlternative"));
        assert!(history_ids.contains("checkoutCheckpoint"));
        let undo_binding = definition.keybindings.iter().find(|binding| binding.keys == "mod+z").expect("undo keybinding auto-injected");
        assert_eq!(undo_binding.action.action, "undo");
        assert_eq!(undo_binding.action.controller_id, canonical_test_app_id("history-app").await);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_does_not_duplicate_manually_declared_history_keybinding() {
        let definition = minimal_app("manual-undo-app").await.keybinding("mod+z", "undo").await.build_definition();
        assert_eq!(definition.keybindings.iter().filter(|b| b.keys == "mod+z").count(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_auto_injects_clipboard_actions_and_keybindings() {
        let definition = minimal_app("clipboard-app").await.build_definition();
        let clipboard_ids: HashSet<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).map(|c| c.id.as_str()).collect();
        assert!(clipboard_ids.contains("copy"));
        assert!(clipboard_ids.contains("cut"));
        assert!(clipboard_ids.contains("paste"));
        let copy_action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|a| a.id == "copy").expect("copy declared");
        assert_eq!(copy_action.kind, ActionKind::Clipboard);
        let paste_action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|a| a.id == "paste").expect("paste declared");
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
        assert!(history_panel_tab_ids.iter().any(|id| *id == ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID));
        let action_ids: HashSet<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).map(|a| a.id.as_str()).collect();
        assert!(action_ids.contains(REVERT_TO_COMMAND_ACTION_ID));
        assert!(action_ids.contains(SET_HISTORY_COMMAND_FILTER_ACTION_ID));
        let revert = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|a| a.id == REVERT_TO_COMMAND_ACTION_ID).expect("revertToCommand declared");
        assert_eq!(revert.kind, ActionKind::History);
        assert!(!revert.in_palette);
        let filter = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|a| a.id == SET_HISTORY_COMMAND_FILTER_ACTION_ID).expect("setHistoryCommandFilter declared");
        assert_eq!(filter.kind, ActionKind::View);
        assert!(!filter.in_palette);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_does_not_duplicate_a_manually_declared_history_panel_tab() {
        let definition = minimal_app("manual-history-app").await.panel_tab(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID, LocalizedLabel::data("Custom History"), PanelGroup::Settings, "custom.history").await.build_definition();
        let mut manual_history_tab_ids: Vec<&str> = Vec::new();
        for t in definition.panel_tabs.iter() {
            manual_history_tab_ids.push(t.id());
        }
        assert_eq!(manual_history_tab_ids.iter().filter(|id| **id == ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID).count(), 1);
        let manual_history_tab_idx = manual_history_tab_ids.iter().position(|id| *id == ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID).expect("history tab present");
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
            .build_definition();
        let by_id = |id: &str| definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|c| c.id == id).expect("declared");
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
        let set_active_utility = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility injected");
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
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID));
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
        let set_active_tool = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID).expect("setActiveTool injected");
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
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn action_args_attaches_declared_arguments() {
        let definition = minimal_app("args-app").await.mutation("resize", LocalizedLabel::data("Resize")).await.action_args("resize", vec![ActionArgDef::slider("scale", LocalizedLabel::data("Scale"), 0.0, 4.0).required()]).await.build_definition();
        let resize = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "resize").expect("declared");
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
        assert_eq!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == INTERACTION_HOVER_ACTION_ID).map(|action| action.kind), Some(ActionKind::Interaction));
        assert_eq!(definition.keybindings.iter().find(|binding| binding.keys == "escape").map(|binding| binding.action.action.as_str()), Some("clearSelection"));
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_introduction_injects_start_introduction_action() {
        use semio_framework::{ActionKind, IntroductionDefinition, IntroductionStepDefinition, START_INTRODUCTION_ACTION_ID};
        use ui_wgpu::wgpu::LocalizedLabel;
        let definition = minimal_app("intro-app")
            .await
            .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("welcome", LocalizedLabel::data("Welcome"), LocalizedLabel::data("Hi there"))] })
            .await
            .build_definition();
        let start_introduction = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == START_INTRODUCTION_ACTION_ID).expect("startIntroduction injected");
        assert_eq!(start_introduction.kind, ActionKind::View);
        assert!(!start_introduction.in_palette, "the shell-owned Introduce App command owns palette discovery");
    }

    #[semio_framework_async_macros::async_test]
    async fn no_introduction_means_no_start_introduction_action() {
        use semio_framework::START_INTRODUCTION_ACTION_ID;
        let definition = minimal_app("no-intro-app").await.build_definition();
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == START_INTRODUCTION_ACTION_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_with_no_steps() {
        use semio_framework::IntroductionDefinition;
        let __base = minimal_app("empty-intro-app").await;
        let __chain = __base.introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![] }).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_duplicate_introduction_step_ids() {
        use semio_framework::{IntroductionDefinition, IntroductionStepDefinition};
        use ui_wgpu::wgpu::LocalizedLabel;
        let __base = minimal_app("dupe-step-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")), IntroductionStepDefinition::new("step", LocalizedLabel::data("B"), LocalizedLabel::data("b"))],
            })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_introducing_undeclared_window_kind() {
        use semio_framework::{IntroductionDefinition, IntroductionStepDefinition, window_element_id};
        use ui_wgpu::wgpu::LocalizedLabel;
        let __base = minimal_app("bad-window-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce(window_element_id("missing"))] })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_introducing_undeclared_panel_tab() {
        use semio_framework::{IntroductionDefinition, IntroductionStepDefinition, panel_tab_element_id, panel_tab_first_draggable_element_id};
        use ui_wgpu::wgpu::LocalizedLabel;
        let __base = minimal_app("bad-panel-tab-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce(panel_tab_element_id("missing"))] })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
        let __base = minimal_app("bad-panel-tab-first-draggable-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce(panel_tab_first_draggable_element_id("missing"))],
            })
            .await;
        let result_first_draggable = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result_first_draggable.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_targeting_malformed_element_id() {
        use semio_framework::{IntroductionDefinition, IntroductionStepDefinition};
        use ui_wgpu::wgpu::LocalizedLabel;
        let __base = minimal_app("bad-element-app").await;
        let __chain =
            __base.introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce("not-camel-case")] }).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
        let __base = minimal_app("bad-element-show-app").await;
        let __chain =
            __base.introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).show(vec!["not-camel-case".into()])] }).await;
        let result_show = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result_show.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_accepts_introduction_step_introducing_escape_hatch_element_id() {
        use semio_framework::{IntroductionDefinition, IntroductionStepDefinition};
        use ui_wgpu::wgpu::LocalizedLabel;
        let definition = minimal_app("good-escape-hatch-app")
            .await
            .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce("ui.custom.thing")] })
            .await
            .build_definition();
        let introduction = definition.introduction.expect("introduction present");
        assert_eq!(introduction.steps.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_interacting_on_undeclared_utility() {
        use semio_framework::{IntroductionDefinition, IntroductionInteraction, IntroductionStepDefinition};
        use ui_wgpu::wgpu::LocalizedLabel;
        let __base = minimal_app("bad-interaction-utility-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).interact(vec![IntroductionInteraction::utility("missing", "Activate").await])],
            })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_interacting_on_undeclared_window_kind() {
        use semio_framework::{IntroductionDefinition, IntroductionInteraction, IntroductionStepDefinition};
        use ui_wgpu::wgpu::LocalizedLabel;
        let __base = minimal_app("bad-interaction-window-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).interact(vec![IntroductionInteraction::orbit("missing", "Orbit").await])],
            })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_accepts_introduction_with_declared_window_utility_and_action_targets() {
        use semio_framework::{IntroductionDefinition, IntroductionInteraction, IntroductionStepDefinition, window_element_id};
        use ui_wgpu::wgpu::LocalizedLabel;
        let definition = minimal_app("good-intro-app")
            .await
            .mutation("addLayer", LocalizedLabel::data("Add Layer"))
            .await
            .utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush)
            .await
            .window_kind_utilities("main", vec!["brush".into()])
            .await
            .window_kind_action_refs("main", vec!["addLayer".into()])
            .await
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![
                    IntroductionStepDefinition::new("welcome", LocalizedLabel::data("Welcome"), LocalizedLabel::data("Hi")),
                    IntroductionStepDefinition::new("main-window", LocalizedLabel::data("Main Window"), LocalizedLabel::data("…")).introduce(window_element_id("main")),
                    IntroductionStepDefinition::new("brush-utility", LocalizedLabel::data("Brush"), LocalizedLabel::data("…")).introduce("brush").interact(vec![IntroductionInteraction::utility("brush", "Activate Brush").await]),
                    IntroductionStepDefinition::new("add-layer", LocalizedLabel::data("Add Layer"), LocalizedLabel::data("…")).interact(vec![IntroductionInteraction::action("addLayer", "Add a Layer").await]),
                    IntroductionStepDefinition::new("navigate-main", LocalizedLabel::data("Navigate"), LocalizedLabel::data("…")).interact(vec![IntroductionInteraction::pan("main", "Pan").await, IntroductionInteraction::zoom("main", "Zoom").await]),
                ],
            })
            .await
            .build_definition();
        let introduction = definition.introduction.expect("introduction present");
        assert_eq!(introduction.steps.len(), 5);
    }

    async fn minimal_tutorial(id: &str) -> TutorialDefinition {
        use semio_framework::{TutorialBase, TutorialDefinition, TutorialTracks, TutorialUiSnapshot};
        TutorialDefinition {
            id: id.into(),
            title: LocalizedLabel::data("Tutorial"),
            description: None,
            duration_ms: 10_000,
            chapters: vec![],
            base: TutorialBase { document_dsl: None, example_id: None, ui: TutorialUiSnapshot::default(), cameras: vec![] },
            tracks: TutorialTracks::default(),
            recorded_at: None,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_tutorial_injects_start_tutorial_action() {
        use semio_framework::{ActionKind, START_TUTORIAL_ACTION_ID};
        let definition = minimal_app("tutorial-app").await.tutorial(minimal_tutorial("welcome-tour").await).await.build_definition();
        let start_tutorial = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == START_TUTORIAL_ACTION_ID).expect("startTutorial injected");
        assert_eq!(start_tutorial.kind, ActionKind::View);
        assert!(!start_tutorial.in_palette, "the shell-owned Play Tutorial command owns palette discovery");
    }

    #[semio_framework_async_macros::async_test]
    async fn no_tutorial_means_no_start_tutorial_action_but_record_is_always_injected() {
        use semio_framework::{RECORD_TUTORIAL_ACTION_ID, START_TUTORIAL_ACTION_ID};
        let definition = minimal_app("no-tutorial-app").await.build_definition();
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == START_TUTORIAL_ACTION_ID));
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == RECORD_TUTORIAL_ACTION_ID), "recordTutorial is injected unconditionally — recording needs no app declaration");
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_tutorial_failing_structural_validation() {
        let mut tutorial = minimal_tutorial("out-of-range-tour").await;
        tutorial.duration_ms = 100;
        tutorial.chapters.push(semio_framework::TutorialChapter { id: "late".into(), at: 999_999, title: LocalizedLabel::data("Late"), body: None });
        let base = minimal_app("bad-structural-tutorial-app").await;
        let __chain = base.tutorial(tutorial).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_duplicate_tutorial_ids() {
        let tour_a = minimal_tutorial("tour").await;
        let tour_b = minimal_tutorial("tour").await;
        let __base = minimal_app("dupe-tutorial-app").await;
        let __chain = __base.tutorial(tour_a).await.tutorial(tour_b).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_tutorial_event_referencing_undeclared_action() {
        use semio_framework::{TutorialEvent, TutorialEventKind};
        let mut tutorial = minimal_tutorial("bad-event-tour").await;
        tutorial.tracks.events = vec![TutorialEvent { at: 10, kind: TutorialEventKind::Action { action: "missingAction".into(), args: None } }];
        let base = minimal_app("bad-tutorial-event-app").await;
        let __chain = base.tutorial(tutorial).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_tutorial_ui_change_referencing_undeclared_utility() {
        use semio_framework::{TutorialUiChange, TutorialUiKeyframe, TutorialUiSample};
        let mut tutorial = minimal_tutorial("bad-ui-change-tour").await;
        tutorial.tracks.ui = vec![TutorialUiKeyframe { at: 10, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveUtility { window_id: "main".into(), utility_id: Some("missing".into()) }] } }];
        let base = minimal_app("bad-tutorial-ui-app").await;
        let __chain = base.tutorial(tutorial).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_tutorial_gesture_targeting_malformed_element_id() {
        use semio_framework::{IntroductionGesture, IntroductionPoint, TutorialGestureCue};
        let mut tutorial = minimal_tutorial("bad-gesture-tour").await;
        tutorial.tracks.gestures = vec![TutorialGestureCue { at: 10, duration_ms: 200, gesture: IntroductionGesture::LeftClick { at: IntroductionPoint::Element { id: "not-camel-case".into(), offset: None } }, cursor: None }];
        let base = minimal_app("bad-tutorial-gesture-app").await;
        let __chain = base.tutorial(tutorial).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_accepts_tutorial_with_declared_action_utility_and_gesture_targets() {
        use semio_framework::{IntroductionGesture, IntroductionPoint, TutorialEvent, TutorialEventKind, TutorialGestureCue, TutorialUiChange, TutorialUiKeyframe, TutorialUiSample, window_element_id};
        let mut tutorial = minimal_tutorial("good-tour").await;
        tutorial.tracks.events = vec![TutorialEvent { at: 10, kind: TutorialEventKind::Action { action: "addLayer".into(), args: None } }];
        tutorial.tracks.ui = vec![TutorialUiKeyframe { at: 20, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveUtility { window_id: "main".into(), utility_id: Some("brush".into()) }] } }];
        tutorial.tracks.gestures = vec![TutorialGestureCue { at: 30, duration_ms: 200, gesture: IntroductionGesture::LeftClick { at: IntroductionPoint::Element { id: window_element_id("main"), offset: None } }, cursor: None }];
        let definition =
            minimal_app("good-tutorial-app").await.mutation("addLayer", LocalizedLabel::data("Add Layer")).await.utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush).await.tutorial(tutorial).await.build_definition();
        assert_eq!(definition.tutorials.len(), 1);
        assert_eq!(definition.tutorials[0].id, "good-tour");
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_dialog_appends_to_definition() {
        use semio_framework::{ActionRef, DialogDefinition};
        let definition = minimal_app("dialog-app").await.mutation("addLayer", LocalizedLabel::data("Add Layer")).await.dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer"))).await.build_definition();
        assert_eq!(definition.dialogs.len(), 1);
        assert_eq!(definition.dialogs[0].id, "addLayer");
        assert_eq!(definition.dialogs[0].submit_label, LocalizedLabel::data("OK"));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_duplicate_dialog_ids() {
        use semio_framework::{ActionRef, DialogDefinition};
        let __base = minimal_app("dupe-dialog-app").await;
        let __chain = __base
            .mutation("addLayer", LocalizedLabel::data("Add Layer"))
            .await
            .dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer")))
            .await
            .dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer Again"), ActionRef::new("addLayer")))
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_dialog_submit_action_referencing_undeclared_action() {
        use semio_framework::{ActionRef, DialogDefinition};
        let __base = minimal_app("bad-dialog-submit-app").await;
        let __chain = __base.dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("missing"))).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_dialog_cancel_action_referencing_undeclared_action() {
        use semio_framework::{ActionRef, DialogDefinition};
        let __base = minimal_app("bad-dialog-cancel-app").await;
        let __chain = __base.mutation("addLayer", LocalizedLabel::data("Add Layer")).await.dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer")).on_cancel(ActionRef::new("missing"))).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn dialog_submit_action_may_reference_an_injected_history_action() {
        use semio_framework::{ActionRef, DialogDefinition};
        let definition = minimal_app("dialog-injected-action-app").await.dialog(DialogDefinition::new("confirmUndo", LocalizedLabel::data("Undo?"), ActionRef::new("undo"))).await.build_definition();
        assert_eq!(definition.dialogs[0].submit_action, ActionRef::new("undo"));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_dialog_duplicate_arg_ids() {
        use semio_framework::{ActionArgDef, ActionRef, DialogDefinition};
        let __base = minimal_app("dupe-dialog-arg-app").await;
        let __chain = __base
            .mutation("addLayer", LocalizedLabel::data("Add Layer"))
            .await
            .dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer")).args(vec![ActionArgDef::text("name", LocalizedLabel::data("Name")), ActionArgDef::text("name", LocalizedLabel::data("Name Again"))]))
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_accepts_app_and_mode_scope_commands() {
        use semio_framework::CommandDefinition;
        let definition = minimal_app("command-app")
            .await
            .command(CommandDefinition::new("app.export", LocalizedLabel::data("Export"), "document", "download", ActionKind::Shell))
            .await
            .mode_command("edit", CommandDefinition::new("mode.focus", LocalizedLabel::data("Focus"), "view", "focus", ActionKind::View))
            .await
            .build_definition();
        assert_eq!(definition.commands.iter().map(|command| command.id.as_str()).collect::<Vec<_>>(), vec!["app.export"]);
        assert_eq!(definition.modes[0].commands.iter().map(|command| command.id.as_str()).collect::<Vec<_>>(), vec!["mode.focus"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_duplicate_command_ids() {
        let __base = minimal_app("dupe-command-app").await;
        let __chain = __base
            .command(CommandDefinition::new("app.export", LocalizedLabel::data("Export"), "document", "download", ActionKind::Shell))
            .await
            .command(CommandDefinition::new("app.export", LocalizedLabel::data("Export Again"), "document", "download", ActionKind::Shell))
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_duplicate_mode_command_ids() {
        use semio_framework::CommandDefinition;
        let __base = minimal_app("dupe-mode-command-app").await;
        let __chain = __base
            .mode_command("edit", CommandDefinition::new("mode.focus", LocalizedLabel::data("Focus"), "view", "focus", ActionKind::View))
            .await
            .mode_command("edit", CommandDefinition::new("mode.focus", LocalizedLabel::data("Focus Again"), "view", "focus", ActionKind::View))
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_derives_command_owner_from_structural_containment() {
        use semio_framework::CommandDefinition;
        let definition = minimal_app("structural-command-app")
            .await
            .app_command("focus", LocalizedLabel::data("App Focus"), "app", ActionKind::View)
            .await
            .mode_command("edit", CommandDefinition::bounded_catalog("focus", LocalizedLabel::data("Mode Focus"), "mode", ActionKind::View))
            .await
            .build_definition();
        assert_eq!(definition.commands[0].category, "app");
        assert_eq!(definition.modes[0].commands[0].category, "mode");
    }

    /// 📇️ Top-level `.action_with(...)` declarations — the shape every procedural/flow app authors its
    /// whole action vocabulary in — MUST reach `AppActionRegistry::actions`, and MUST carry their
    /// `InteractiveJobClassification` with them. `migrated_tool_ids` is the join key
    /// `validate_tool_job_rows` uses, so an empty `actions` index makes every bounded first-step tool
    /// proof fail closed with `interactive-job.catalog-authority` and `migrated={}` — the app then cannot
    /// be constructed at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.3).
    #[semio_framework_async_macros::async_test]
    async fn app_action_registry_indexes_top_level_app_actions_and_their_migrated_disposition() {
        use semio_framework::InteractiveJobClassification::Migrated;
        let definition = minimal_app("registry-top-level-actions")
            .await
            .action_with(ActionDefinition::bounded_catalog("addWidget", LocalizedLabel::data("Add Widget"), ActionKind::Mutation))
            .await
            .view_action("setShowMode", LocalizedLabel::data("Set Show Mode"))
            .await
            .action_interactive_job("addWidget", Migrated)
            .await
            .action_interactive_job("setShowMode", Migrated)
            .await
            .try_build_definition()
            .expect("a classified top-level inventory is releasable");
        let registry = AppActionRegistry::from_definition(&definition);
        assert!(registry.actions.contains_key("addWidget"), "top-level mutation missing from the app action index: {:?}", registry.actions.keys().collect::<Vec<_>>());
        assert!(registry.actions.contains_key("setShowMode"), "top-level view action missing from the app action index");
        assert_eq!(registry.actions["addWidget"].semantics.execution.interactive_job, Migrated);
        let migrated = registry.migrated_tool_ids();
        assert!(migrated.contains("addWidget") && migrated.contains("setShowMode"), "migrated tool ids must join the top-level declarations: {migrated:?}");
        println!("[STATS] app action registry indexed {} actions, {} migrated", registry.actions.len(), migrated.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_empty_mode_command_id() {
        use semio_framework::CommandDefinition;
        let __base = minimal_app("empty-mode-command-app").await;
        let __chain = __base.mode_command("edit", CommandDefinition::bounded_catalog("", LocalizedLabel::data("Focus"), "view", ActionKind::View)).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }
}
