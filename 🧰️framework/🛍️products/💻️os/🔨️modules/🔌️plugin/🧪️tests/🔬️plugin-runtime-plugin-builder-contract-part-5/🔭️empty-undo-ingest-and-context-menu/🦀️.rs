/// 🔭️ Empty undo, ingest and context menu: undo on empty history, serialization, idempotent ingest, backbone reattach, context menu labels and the active utility.

    #[semio_framework_async_macros::async_test]
    async fn undo_on_empty_history_is_a_benign_no_operation() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert!(result.mutations.is_empty());
        assert!(result.events.is_empty());
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn document_round_trips_through_serialization() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("inc");
        app.dispatch_typed(TestCommand::SetLabel { value: "hi".into() }, &meta(), &mut identity,cx).await.expect("label");
        let files = app.document_pack().await.expect("document pack");

        let mut restored = contract_app(mounted_policy, &mut identity).await;
        artifact_app_laws::load_document(&mut restored, &files, &mut identity).await.expect("load document pack");
        assert_eq!(restored.test_snapshot().await, TestSnapshot { count: 1, label: "hi".into(), slot: Vec::new() });
    
        artifact_app_laws::close_registered_fixture_app(&mut restored.0,restored.1,cx).expect("original contract fixture paid terminal close");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut sender = contract_app(mounted_policy, &mut identity).await;
        let (near, mut far) = MemoryBackbone::pair("mem://doc", "mem://doc").await;
        sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach");
        sender.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");

        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
        let operations = protocol::encode_envelopes(&envelopes);

        let mut receiver = contract_app(mounted_policy, &mut identity).await;
        receiver.ingest_operations(&operations, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("ingest once");
        receiver.ingest_operations(&operations, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("ingest twice");
        assert_eq!(receiver.test_snapshot().await.count, 1, "feeding the same operation twice must not double-apply");
    
        artifact_app_laws::close_registered_fixture_app(&mut receiver.0,receiver.1,cx).expect("original contract fixture paid terminal close");
    
        artifact_app_laws::close_registered_fixture_app(&mut sender.0,sender.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn attach_detach_reattach_resumes_backbone_convergence() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        assert!(app.backbone_ref().is_none(), "default is unattached");

        let (near, mut far) = MemoryBackbone::pair("mem://reattach", "mem://reattach").await;
        app.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment while attached");
        assert!(!far.receive().await.expect("receive after attach").is_empty(), "attached edits reach the peer");

        app.detach_backbone().await.expect("detach");
        assert!(app.backbone_ref().is_none());
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment while detached");
        assert_eq!(app.stores.test_snapshot().await.count, 2, "detached edits still land on the in-memory graph");
        assert!(far.receive().await.expect("receive while detached").is_empty(), "detached edits never reach the peer");

        let (near_again, mut far_again) = MemoryBackbone::pair("mem://reattach-2", "mem://reattach-2").await;
        app.attach_backbone(store::os_store::backbone::Backbones::Memory(near_again)).await.expect("re-attach");
        assert!(app.backbone_ref().is_some());
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment after re-attach");
        assert_eq!(app.stores.test_snapshot().await.count, 3);
        assert!(!far_again.receive().await.expect("receive after re-attach").is_empty(), "re-attaching resumes outbound convergence on the new backbone");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🖱️ `PluginApp::context_menu` end-to-end through `VcsArtifactApp`: with an empty label the
    /// "selection guard" (`Menu::when`) drops the gated command; once a label is set (a stand-in for
    /// "something is selected"), both rows resolve label/icon from the declared registry entries.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_resolves_labels_from_the_registry_and_respects_guards() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let request = ContextMenuRequest { menu: UiMenuRef { id: "window".into(), args: None }, surface: None, window_instance_id: None, point: None };

        use semio_framework::default_action_icon_id;

        let set_label_icon = default_action_icon_id(ActionKind::Mutation).as_str().to_string();
        let increment_icon = default_action_icon_id(ActionKind::Mutation).as_str().to_string();

        let empty_label = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
        assert_eq!(empty_label.len(), 1, "the gated command must be absent with no label set: {empty_label:?}");
        assert_eq!(empty_label[0], ContextMenuItemSpec { id: "setLabelRequired".into(), label: Some("Set Label".into()), icon: Some(set_label_icon), action: Some("setLabelRequired".into()), ..Default::default() });

        app.dispatch_typed(TestCommand::SetLabel { value: "hi".into() }, &meta(), &mut identity,cx).await.expect("set label");
        let with_label = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
        assert_eq!(with_label.len(), 2, "the guard must open once a label is set: {with_label:?}");
        assert_eq!(with_label[1], ContextMenuItemSpec { id: "incrementViaCommand".into(), label: Some("Increment".into()), icon: Some(increment_icon), action: Some("incrementViaCommand".into()), ..Default::default() });
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪️ A registry declaring `setLabelRequired` plus ten `flatLeaf1..10` actions (four carrying a
    /// `RIBBON_PARENT_CATEGORIES` category via `with_category`) — feeds `TestApp::context_menu`'s
    /// `"flat-menu-test"` branch below.
    async fn flat_menu_registry() -> AppActionRegistry {
        let builder = App::builder(test_app_surface_id().await, LocalizedLabel::data("FlatMenuTest"))
                .await
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), "flat-menu-test.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await
                .mutation("setLabelRequired", LocalizedLabel::data("Set Label"))
                .await
                .action_args("setLabelRequired", vec![ActionArgDef::text("value", LocalizedLabel::data("Value")).required()])
                .await
                .mutation("flatLeaf1", LocalizedLabel::data("Flat Leaf 1"))
                .await
                .mutation("flatLeaf2", LocalizedLabel::data("Flat Leaf 2"))
                .await
                .mutation("flatLeaf3", LocalizedLabel::data("Flat Leaf 3"))
                .await
                .mutation("flatLeaf4", LocalizedLabel::data("Flat Leaf 4"))
                .await
                .action_with(ActionDefinition::bounded_catalog("flatLeaf5", LocalizedLabel::data("Flat Leaf 5"), ActionKind::Mutation).with_category("view"))
                .await
                .action_with(ActionDefinition::bounded_catalog("flatLeaf6", LocalizedLabel::data("Flat Leaf 6"), ActionKind::Mutation).with_category("view"))
                .await
                .action_with(ActionDefinition::bounded_catalog("flatLeaf7", LocalizedLabel::data("Flat Leaf 7"), ActionKind::Mutation).with_category("export"))
                .await
                .action_with(ActionDefinition::bounded_catalog("flatLeaf8", LocalizedLabel::data("Flat Leaf 8"), ActionKind::Mutation).with_category("export"))
                .await
                .mutation("flatLeaf9", LocalizedLabel::data("Flat Leaf 9"))
                .await
                .mutation("flatLeaf10", LocalizedLabel::data("Flat Leaf 10"))
                .await;
        let app = App::from_builder(declare_test_app_verbs(builder).interactive_jobs(InteractiveJobClassification::Migrated).await).await;
        AppActionRegistry::from_definition(&app.definition)
    }

    /// 🖱️ End to end through `VcsArtifactApp::context_menu`: a synthetic emitter's 11 flat leaves come
    /// back as 5 primaries + 3 taxonomy-sorted `menu.group.<category>` rows — proving the funnel applies
    /// `organize_context_menu` to every emitter, not just ones that call `Menu::group` themselves.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_funnel_organizes_a_synthetic_apps_flat_overflow_menu() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_with(flat_menu_registry().await, mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "flat-menu-test".into() }, &meta(), &mut identity,cx).await.expect("set label");
        let request = ContextMenuRequest { menu: UiMenuRef { id: "window".into(), args: None }, surface: None, window_instance_id: None, point: None };

        let organized = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
        let ids: Vec<&str> = organized.iter().map(|item| item.id.as_str()).collect();
        assert_eq!(
            ids,
            vec!["setLabelRequired", "flatLeaf1", "flatLeaf2", "flatLeaf3", "flatLeaf4", "menu.group.view", "menu.group.actions", "menu.group.export"],
            "5 primaries, then groups in RIBBON_PARENT_CATEGORIES taxonomy order (view < actions < export): {ids:?}"
        );
        let view_children: Vec<&str> = organized[5].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
        assert_eq!(view_children, vec!["flatLeaf5", "flatLeaf6"]);
        let actions_children: Vec<&str> = organized[6].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
        assert_eq!(actions_children, vec!["flatLeaf9", "flatLeaf10"], "uncategorized overflow leaves default to menu.group.actions");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_utility_carries_its_value_directly_and_emits_no_operations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::SetActiveUtility { utility_id: "brush".into() }, &meta(), &mut identity,cx).await.expect("setActiveUtility is a valid View command");
        assert!(result.mutations.is_empty(), "utility switching must not create history");
        let event = result.events.iter().find(|event| event.kind == "active-utility").expect("echoed active utility");
        assert_eq!(event.payload, semio_framework_value::DslValue::from(json!({ "utilityId": "brush" })));
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
