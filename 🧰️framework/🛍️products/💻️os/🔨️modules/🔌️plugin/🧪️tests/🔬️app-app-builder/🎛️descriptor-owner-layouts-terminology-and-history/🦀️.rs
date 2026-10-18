    /// 🎛️ Descriptor owner, layouts, terminology and history: the shared framework action descriptor owner laws, the layout window kind refusals, the terminology document laws, the release catalog and the history action injection.

    /// 🧭️ Executes neutral owner placements through the actual action disposition join and independent serde projection.
    #[semio_framework_async_macros::async_test]
    async fn shared_framework_action_descriptor_owner_laws_execute_production_join() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/⚖️shared-framework-action-routes-law.json"))).expect("plain neutral descriptor laws");
        let base = App::builder(canonical_test_app_id("shared-actions").await, LocalizedLabel::data("Shared actions")).await.document(["semio", "shared-actions"]).mode("edit", LocalizedLabel::data("Edit"), "pencil").await.window_kind("main", LocalizedLabel::data("Main"), "shared.main", SurfaceKind::Canvas2d, IconName::AppWindow).await.default_layout(create_default_layout(&["main".into()], "row", None, None)).await.build_definition();
        let laws = fixture["descriptorLaws"].as_array().expect("descriptor laws");
        for law in laws {
            let mut definition = base.clone();
            definition.actions.clear();
            definition.commands.clear();
            for window in definition.window_kinds.iter_mut() { window.actions.clear(); }
            for mode in definition.modes.iter_mut() { mode.commands.clear(); }
            let initial = if law["initial"] == "migrated" { InteractiveJobClassification::Migrated } else { InteractiveJobClassification::Unclassified };
            for id in law["actionIds"].as_array().expect("action ids") {
                let id = id.as_str().expect("exact action id");
                match law["ownerKind"].as_str().expect("owner kind") {
                    "appAction" | "windowAction" => {
                        let mut action = ActionDefinition::new_catalog(id, LocalizedLabel::data(id), ActionKind::View);
                        action.semantics.execution.interactive_job = initial;
                        if law["ownerKind"] == "appAction" { definition.actions.push(action); } else { definition.window_kinds.iter_mut().next().expect("window owner").actions.push(action); }
                    }
                    "appCommand" | "modeCommand" => {
                        let mut command = CommandDefinition::new_catalog(id, LocalizedLabel::data(id), "test", ActionKind::View);
                        command.semantics.execution.interactive_job = initial;
                        if law["ownerKind"] == "appCommand" { definition.commands.push(command); } else { definition.modes.iter_mut().next().expect("mode owner").commands.push(command); }
                    }
                    other => panic!("unknown owner kind {other}"),
                }
            }
            semio_framework_plugin_job::reserved_jobs::join_framework_shared_action_dispositions(&mut definition);
            let rows = match law["ownerKind"].as_str().expect("owner kind") {
                "appAction" => serde_json::to_value(&definition.actions),
                "windowAction" => serde_json::to_value(&definition.window_kinds.iter().next().expect("window owner").actions),
                "appCommand" => serde_json::to_value(&definition.commands),
                "modeCommand" => serde_json::to_value(&definition.modes.iter().next().expect("mode owner").commands),
                other => panic!("unknown owner kind {other}"),
            }.expect("independent descriptor projection");
            let expected = if law["expected"] == "migrated" { "migrated" } else { "unclassified" };
            for row in rows.as_array().expect("descriptor rows") { assert_eq!(row["semantics"]["execution"]["interactiveJob"], expected, "{} {}", law["id"], row["id"]); }
        }
    }

    /// 🪪️ Contract §1 fixture — a canonical id built via `surface_app_id` from a fixture `Dialect`,
    /// not a hand-written pre-migration string. One shared helper rather than one dialect per test.
    #[doc(hidden)]
pub async fn canonical_test_app_id(slug: &str) -> String {
        surface_app_id(&ArtifactDialect { artifact_kind: format!("s.test.app-builder.{slug}"), standard: "1".into(), subset: "*".into() }, AppRole::Editor)
    }

    /// 🧾️ Every action a built definition declares — the app-level roster (`AppDefinition::actions`,
    /// where `try_build_definition` puts every framework-injected verb) chained with the per-window
    /// rosters. A window kind only carries the actions its own declaration or `action_refs` named,
    /// so a lookup that reads `window_kinds` alone cannot see an injected app-level verb at all.
    fn declared_actions(definition: &AppDefinition) -> impl Iterator<Item = &ActionDefinition> {
        definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter()))
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

    /// 🕰️ 1 declared + the auto-injected framework History tab.
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
            .panel_tab("framework.panel.artifact", LocalizedLabel::data("Artifact"), PanelGroup::Workbench, "good.document")
            .await
            .default_layout(create_default_layout(&["main".into()], "row", None, None))
            .await
            .build_definition();
        assert_eq!(definition.window_kinds.len(), 1);
        assert_eq!(definition.window_kinds.iter().next().map(|kind| kind.icon_id.as_str()), Some("app-window"));
        assert_eq!(definition.modes.first().icon_id.as_str(), "pencil");
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
        let migrated_action = declared_actions(&migrated).find(|action| action.id == "migratedAction").expect("the declared action survives into the released catalog");
        assert_eq!(migrated_action.semantics.execution.interactive_job, Migrated);

        for (index, classification) in [BatchOnlyPendingRewrite, ForbiddenFromUi, Deleted].into_iter().enumerate() {
            let mut action = ActionDefinition::bounded_catalog("blockedAction", LocalizedLabel::data("Blocked"), ActionKind::Mutation);
            action.semantics.execution.interactive_job = classification;
            let definition = minimal_app(&format!("classified-action-{index}")).await.action_with(action).await.try_build_definition().expect("an explicit non-UI disposition is valid inventory data");
            let retained = declared_actions(&definition).find(|entry| entry.id == "blockedAction").expect("classified action retained");
            assert_eq!(retained.semantics.execution.interactive_job, classification);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_auto_injects_history_actions_and_keybindings() {
        let definition = minimal_app("history-app").await.build_definition();
        let history_ids: HashSet<&str> = declared_actions(&definition).map(|c| c.id.as_str()).collect();
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

    /// 👁️ One app of `slug`'s test dialect in `role`, otherwise [`minimal_app`].
    async fn surface_app(slug: &str, role: AppRole) -> AppDefinition {
        App::builder(surface_app_id(&ArtifactDialect { artifact_kind: format!("s.test.app-builder.{slug}"), standard: "1".into(), subset: "*".into() }, role), LocalizedLabel::data("App"))
            .await
            .document(["semio", slug])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), format!("{slug}.main"), SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .build_definition()
    }

    /// 👁️ Row 3.4 (C13, measured on hub 7800 p24): a viewer's manifest declares — and binds — no verb its `ViewerGuard`
    /// rejects, so no host offers a Spectator an edit control; switching an alternative or checking out a checkpoint
    /// commits a shared `Checkout` transition, so a viewer declares neither; `copy` stays, and the editor of the same
    /// dialect keeps every framework verb.
    #[semio_framework_async_macros::async_test]
    async fn build_definition_offers_a_viewer_no_verb_its_guard_rejects() {
        let viewer = surface_app("viewer-guard", AppRole::Viewer).await;
        let editor = surface_app("viewer-guard", AppRole::Editor).await;
        let viewer_ids: HashSet<&str> = declared_actions(&viewer).map(|action| action.id.as_str()).collect();
        let editor_ids: HashSet<&str> = declared_actions(&editor).map(|action| action.id.as_str()).collect();
        for verb in VIEWER_REJECTED_ACTION_IDS {
            assert!(!viewer_ids.contains(verb), "a viewer declares {verb}");
            assert!(viewer.keybindings.iter().all(|binding| binding.action.action != verb), "a viewer binds {verb}");
        }
        for verb in ["switchAlternative", "checkoutCheckpoint"] {
            assert!(!viewer_ids.contains(verb), "a viewer never moves the shared head: {verb}");
        }
        assert!(viewer_ids.contains("copy"), "a viewer still copies");
        for verb in ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint", "copy", "cut", "paste"] {
            assert!(editor_ids.contains(verb), "the editor keeps {verb}");
        }
    }

    /// ⏪️ Every editor declares the eleven history-edit verbs (no chord of their own, never in the palette) and the
    /// finalize prompt they open; a viewer declares neither, since its guard rejects every one of them.
    #[semio_framework_async_macros::async_test]
    async fn build_definition_injects_the_history_edit_verbs_and_prompt_into_an_editor_only() {
        let viewer = surface_app("history-edit", AppRole::Viewer).await;
        let editor = surface_app("history-edit", AppRole::Editor).await;
        let editor_ids: HashSet<&str> = declared_actions(&editor).map(|action| action.id.as_str()).collect();
        let viewer_ids: HashSet<&str> = declared_actions(&viewer).map(|action| action.id.as_str()).collect();
        for verb in semio_framework::history_edit::HISTORY_EDIT_ACTION_IDS {
            assert!(editor_ids.contains(verb), "the editor declares {verb}");
            assert!(!viewer_ids.contains(verb), "a viewer never declares {verb}");
            assert!(editor.keybindings.iter().all(|binding| binding.action.action != verb), "{verb} carries no framework chord");
        }
        assert!(declared_actions(&editor).filter(|action| is_time_travel_action_id(&action.id)).all(|action| !action.in_palette && action.kind == ActionKind::History));
        assert!(editor.dialogs.iter().any(|dialog| dialog.id == semio_framework::history_edit::HISTORY_EDIT_FINALIZE_DIALOG_ID), "the editor declares the finalize prompt");
        assert!(viewer.dialogs.iter().all(|dialog| dialog.id != semio_framework::history_edit::HISTORY_EDIT_FINALIZE_DIALOG_ID), "a viewer declares no finalize prompt");
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_does_not_duplicate_manually_declared_history_keybinding() {
        let definition = minimal_app("manual-undo-app").await.keybinding("mod+z", "undo").await.build_definition();
        assert_eq!(definition.keybindings.iter().filter(|b| b.keys == "mod+z").count(), 1);
    }
