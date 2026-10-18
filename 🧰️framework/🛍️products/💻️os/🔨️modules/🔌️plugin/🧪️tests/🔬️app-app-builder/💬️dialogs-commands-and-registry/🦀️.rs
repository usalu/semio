    /// 💬️ Dialogs, commands and registry: the dialog declarations, the app and mode scope commands, the command owner derivation, the app action registry, the stack default layout and the empty mode command id.

    #[semio_framework_async_macros::async_test]
    async fn declaring_dialog_appends_to_definition() {
        use semio_framework::{ActionRef, DialogDefinition};
        let definition = minimal_app("dialog-app").await.mutation("addLayer", LocalizedLabel::data("Add Layer")).await.dialog(DialogDefinition::new("addLayer", LocalizedLabel::data("Add Layer"), ActionRef::new("addLayer"))).await.interactive_jobs(semio_framework::InteractiveJobClassification::Migrated).await.build_definition();
        assert_eq!(definition.dialogs.iter().map(|dialog| dialog.id.as_str()).collect::<Vec<_>>(), ["addLayer", semio_framework::history_edit::HISTORY_EDIT_FINALIZE_DIALOG_ID], "the app's own dialog, then the framework-injected finalize prompt");
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
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
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
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
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

    /// 🪟️ `stack` is the tabbed LEAF container, never an axis direction. `create_default_layout`
    /// used to answer `"stack"` with `Axis { kind: "stack", children: [Stack[window]] }`, and every
    /// consumer that walks a stack node's children as window leaves — the React seed resolver
    /// `convertFrameworkLayoutNodeToModeLayout` among them — then read the inner stack AS a window
    /// and minted a pane with no id, which is how `🎞️animate` booted with zero panes
    /// (ticket 26/09/18, slice F1).
    #[semio_framework_async_macros::async_test]
    async fn a_stack_default_layout_is_one_stack_root_of_window_leaves() {
        use semio_framework_ui_layout::layout::{WindowLayoutRoot, create_stack_layout};
        let layout = create_default_layout(&["tile-editor".into()], "stack", None, Some(&["Tile editor".into()]));
        let WindowLayoutRoot::Stack(root) = &layout.root else {
            panic!("a stack request must mint a stack ROOT, not an axis wearing the stack discriminator");
        };
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].window_kind_id, "tile-editor");
        assert_eq!(root.children[0].title.as_deref(), Some("Tile editor"));
        assert_eq!(&layout, &create_stack_layout(&["tile-editor".into()], Some(&["Tile editor".into()])));
        let row = create_default_layout(&["a".into(), "b".into()], "row", Some(&[60.0, 40.0]), None);
        assert!(matches!(row.root, WindowLayoutRoot::Axis(_)), "a row direction is still an axis root");
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_empty_mode_command_id() {
        use semio_framework::CommandDefinition;
        let __base = minimal_app("empty-mode-command-app").await;
        let __chain = __base.mode_command("edit", CommandDefinition::bounded_catalog("", LocalizedLabel::data("Focus"), "view", ActionKind::View)).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }
