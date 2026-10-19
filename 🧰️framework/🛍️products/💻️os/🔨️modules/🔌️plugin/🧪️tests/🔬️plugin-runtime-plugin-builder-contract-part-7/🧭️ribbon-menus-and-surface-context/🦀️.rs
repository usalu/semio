/// 🧭️ Ribbon, menus and surface context: selection phrases, ribbon taxonomy fields, grouped context menus, surface context projection and the scene lane carriers.

    #[semio_framework_async_macros::async_test]
    async fn selection_count_phrase_formats_mixed_selection() {
        assert_eq!(selection_count_phrase(semio_framework_ui_locale::Locale::En, &[(8, SelectionKind::Node), (13, SelectionKind::Edge)]).as_deref(), Some("8 nodes and 13 edges"));
        assert_eq!(selection_count_phrase(semio_framework_ui_locale::Locale::En, &[(1, SelectionKind::Node)]).as_deref(), Some("1 node"));
        assert_eq!(selection_count_phrase(semio_framework_ui_locale::Locale::De, &[(8, SelectionKind::Node), (13, SelectionKind::Edge)]).as_deref(), Some("8 Knoten und 13 Kanten"));
    }

    //#region 🗂️GroupedContextMenu
    #[semio_framework_async_macros::async_test]
    async fn action_definition_with_category_sets_the_ribbon_taxonomy_field() {
        let action = ActionDefinition::bounded_catalog("x", LocalizedLabel::data("X"), ActionKind::Mutation).with_category("view");
        assert_eq!(action.category.as_deref(), Some("view"));
    }

    #[semio_framework_async_macros::async_test]
    async fn menu_group_produces_a_group_row_keyed_by_category() {
        let registry = contract_registry().await;
        let items = Menu::of(&registry, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).action("setLabelRequired").group("export", |m| m.command("incrementViaCommand")).build();
        assert_eq!(items.len(), 2);
        assert_eq!(items[1].id, "menu.group.export");
        assert_eq!(items[1].label, None, "group rows travel with no label — the host resolves it via `ribbon_parent_label`");
        let children: Vec<&str> = items[1].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
        assert_eq!(children, vec!["incrementViaCommand"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_wire_requires_and_forwards_canonical_view_state() {
        assert!(serde_json::from_str::<ContextMenuWireRequest>(r#"{"menu":{"id":"window"}}"#).is_err());
        let wire: ContextMenuWireRequest = serde_json::from_str(r#"{"menu":{"id":"window"},"viewState":{"locale":"de","terminology":"reuse"}}"#).expect("canonical viewState parses");
        let (request, view_state) = wire.into_parts().unwrap();
        assert_eq!(request.menu.id, "window");
        assert_eq!(view_state.locale, semio_framework_ui_locale::Locale::De);
        assert_eq!(view_state.terminology, semio_framework_ui_locale::Terminology::Reuse);
        let fixture: Value = serde_json::from_str(include_str!("../../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        for surface in fixture["surfaces"].as_array().unwrap() {
            let wire: ContextMenuWireRequest = serde_json::from_value(json!({"menu":{"id":"window"},"windowInstanceId":surface["windowId"],"viewState":fixture["view"]})).unwrap();
            let (_, view) = wire.into_parts().unwrap();
            assert_eq!(view.window_id.as_deref(), surface["windowId"].as_str());
            assert_eq!(view.active_utility_id.as_deref(), surface["activeUtilityId"].as_str());
        }
        let wire: ContextMenuWireRequest = serde_json::from_value(json!({"menu":{"id":"window"},"windowInstanceId":"unknown","viewState":fixture["view"]})).unwrap();
        assert!(wire.into_parts().is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn surface_context_reaches_real_app_render_and_rejects_hidden_surfaces() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let app = contract_app_raw(mounted_policy, &mut identity).await;
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(7, cell);
        for surface in fixture["surfaces"].as_array().unwrap() {
            let view = surface["windowId"].as_str().map(|window| host_view.for_window_instance(window).unwrap()).unwrap_or_else(|| host_view.clone());
            semio_framework_plugin_runtime_ops::ops_surface_render::plugin_mount_surface(&runtime, 7, surface["id"].as_str().unwrap().into(), surface["bodyKey"].as_str().unwrap().into(), &semio_framework_plugin_plugin_program::encode_wire_serialized(&view)).await.unwrap();
        }
        for surface in fixture["surfaces"].as_array().unwrap() {
            semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 7, surface["id"].as_str().unwrap()).await.unwrap();
            let (body, actual) = RENDER_CONTEXT_PROBE.with(|probe| probe.take().unwrap());
            assert_eq!(body, surface["bodyKey"].as_str().unwrap());
            assert_eq!(actual.window_id.as_deref(), surface["windowId"].as_str());
            assert_eq!(actual.active_utility_id.as_deref(), surface["activeUtilityId"].as_str());
            assert_eq!(actual.locale, host_view.locale);
            assert_eq!(actual.terminology, host_view.terminology);
        }
        let hidden = fixture["hidden"].as_str().unwrap();
        semio_framework_plugin_runtime_ops::ops_surface_render::plugin_hide_surface(&runtime, 7, hidden).await.unwrap();
        assert!(semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 7, hidden).await.is_err());
        assert!(semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 7, fixture["survivor"].as_str().unwrap()).await.is_ok());
    }
    //#endregion 🗂️GroupedContextMenu

    #[semio_framework_async_macros::async_test]
    async fn surface_context_refresh_projects_panels_from_focused_window_state() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        let focused = host_view.for_window_instance("right").unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let app = contract_app_raw(mounted_policy, &mut identity).await;
        runtime.instances.borrow_mut().insert_admitted(7, std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() }, runtime.mounted_policy)));
        let mut sequence = 0;
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(4_096, u64::MAX, mounted_policy.preparation), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, crate::artifact_app_laws::fixture_progress());
        let response = semio_framework_plugin_runtime_ops::ops_surface_render::plugin_refresh_ui(&runtime, 7, &serde_json::to_string(&json!({"viewState":focused,"panels":[{"key":"properties","bodyKey":"properties"}]})).unwrap(), &mut cx).await.unwrap();
        let response: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(response["panels"][0]["key"], "properties");
        let (body, actual) = RENDER_CONTEXT_PROBE.with(|probe| probe.take().unwrap());
        assert_eq!(body, "properties");
        assert_eq!(serde_json::to_value(actual).unwrap(), serde_json::to_value(host_view.for_panel()).unwrap());
    }



    //#region 🚚️World3dSceneLaneCarriers












    //#endregion 🚚️World3dSceneLaneCarriers

    #[semio_framework_async_macros::async_test]
    async fn surface_context_presence_targets_each_concrete_surface() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        let mut app = interaction_app_raw(mounted_policy, &mut identity).await;
        let mut peers = PeerPresenceRoot::empty();
        peers.insert(fixture["presence"]["actor"].as_str().unwrap().into(), PeerPresence {
            color: Some(3), surface: None,
            interaction: Some(PresenceInteraction {
                app_id: "s.test.synthetic@1/*#editor".into(),
                domains: vec![PresenceDomain { domain: fixture["presence"]["domainId"].as_str().unwrap().into(), granularity: "item".into(), selected: vec![fixture["presence"]["nodeKey"].as_str().unwrap().into()], hovered: Vec::new() }],
            }),
        }).unwrap();
        *app.roster.peer_presence = std::sync::Arc::new(peers);
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        runtime.instances.borrow_mut().insert_admitted(7, std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() }, runtime.mounted_policy)));
        for surface in fixture["surfaces"].as_array().unwrap() {
            let id = surface["id"].as_str().unwrap();
            let view = surface["windowId"].as_str().map(|window| host_view.for_window_instance(window).unwrap()).unwrap_or_else(|| host_view.for_panel());
            semio_framework_plugin_runtime_ops::ops_surface_render::plugin_mount_surface(&runtime, 7, id.into(), surface["bodyKey"].as_str().unwrap().into(), &semio_framework_plugin_plugin_program::encode_wire_serialized(&view)).await.unwrap();
            let (_, presence) = semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 7, id).await.unwrap();
            assert_eq!(presence.len(), 1);
            let mut expected = fixture["presence"]["expected"].clone();
            expected["surface"] = surface["id"].clone();
            assert_eq!(serde_json::to_value(&presence[0]).unwrap(), expected);
        }
    }

    /// 🎯️ M1 decision: `ActionId.version` mismatch returns a `Fault`, never silently dispatches a
    /// stale contract — the default `command_from_intent` bridge only resolves version 1.
    #[semio_framework_async_macros::async_test]
    async fn command_from_intent_rejects_a_non_v1_action_version() {
        let intent = UiIntent {
            surface: SurfaceId::try_from("s").expect("bounded fixture"),
            revision: UiRevision(0),
            node: UiNodeId::default(),
            node_key: UiText::try_from_str("counter").expect("bounded fixture"),
            trigger: Trigger::Activate,
            action: ActionId::new(UiText::try_from_str("app").expect("bounded fixture"), UiText::try_from_str("incrementViaCommand").expect("bounded fixture"), 2),
            args: None,
            input: None,
            seq: 3,
        };
        let error = TestApp::<false>::command_from_intent(&intent).await.expect_err("a non-v1 action must be rejected, never silently dispatched");
        assert!(error.message.contains("version"), "unexpected error: {}", error.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_dispatch_backstop_rejects_every_non_migrated_action_and_command() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::InteractiveJobClassification::{BatchOnlyPendingRewrite, Deleted, ForbiddenFromUi, Unclassified};
        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};

        for classification in [Unclassified, BatchOnlyPendingRewrite, ForbiddenFromUi, Deleted] {
            let mut action_registry = contract_registry().await;
            action_registry.test_set_action_classification("badView", classification);
            let mut action_app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_NONE>>::with_registry(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), action_registry, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
            semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut action_app, mounted_policy);
            assert!(!action_app.test_registered_tool_keys().iter().any(|key| key.1 == "badView"));
            let action_error = action_app.dispatch_typed(TestCommand::BadView, &meta(), &mut identity).await.expect_err("non-migrated action must be rejected before its handler runs");
            assert_eq!(action_error.code.0, "interactive-job.not-ui-safe");
            assert_eq!(action_app.test_snapshot().await.count, 0);

            let mut command_registry = contract_registry().await;
            command_registry.test_set_app_command_classification("incrementViaCommand", classification);
            let mut command_app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_NONE>>::with_registry(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), command_registry, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
            semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut command_app, mounted_policy);
            assert!(!command_app.test_registered_tool_keys().iter().any(|key| key.1 == "incrementViaCommand"));
            let mut invocation =
                CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into() }, command_id: "incrementViaCommand".into() }, arguments: Default::default() };
            let command_error = command_app.handle_command(&mut invocation, None, Some("edit"), &meta(), &mut identity).await.expect_err("non-migrated command must be rejected before its handler runs");
            assert_eq!(command_error.code.0, "interactive-job.not-ui-safe");
            assert_eq!(command_app.test_snapshot().await.count, 0);
            artifact_app_laws::close_registered_fixture_app(&mut action_app, mounted_policy,cx).expect("original contract fixture paid terminal close");
            artifact_app_laws::close_registered_fixture_app(&mut command_app, mounted_policy,cx).expect("original contract fixture paid terminal close");
        }

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
