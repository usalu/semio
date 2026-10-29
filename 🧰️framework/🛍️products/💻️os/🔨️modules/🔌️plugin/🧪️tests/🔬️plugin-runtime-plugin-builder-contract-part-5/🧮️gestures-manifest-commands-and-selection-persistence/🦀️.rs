/// 🧮️ Gestures, manifest commands and selection persistence: streamed gestures, manifest command ownership, addressed windows, command op history and persisted interaction selection.

    /// 🧷️ A streamed gesture is one undo step on the typed route too, while each plain label edit is its own edit: one undo
    /// only reverts the last one.
    #[semio_framework_async_macros::async_test]
    async fn a_streamed_gesture_is_one_undo_step_while_each_commit_is_its_own() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        for (value, commit) in [("a", false), ("ab", false), ("abc", true)] {
            app.dispatch_typed(TestCommand::StreamLabel { value: value.into(), commit }, &meta(), &mut identity,cx).await.expect("streamLabel");
        }
        assert_eq!(app.stores.test_snapshot().await.label, "abc");
        reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        assert_eq!(app.stores.test_snapshot().await.label, "");

        for value in ["x", "xy"] {
            app.dispatch_typed(TestCommand::SetLabel { value: value.into() }, &meta(), &mut identity,cx).await.expect("setLabel");
        }
        assert_eq!(app.stores.test_snapshot().await.label, "xy");
        reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        assert_eq!(app.stores.test_snapshot().await.label, "x");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn operation_command_emits_kernel_op_with_true_inverse() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::IncrementViaCommand, &meta(), &mut identity,cx).await.expect("incrementViaCommand");
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 1 })).unwrap());
        assert_eq!(result.mutations[0].inverse.inverse_diff.payload, protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 0 })).unwrap()]));
        assert_eq!(app.stores.test_snapshot().await.count, 1);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_command_dispatch_validates_structural_app_ownership() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let mut valid = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into() }, command_id: "incrementViaCommand".into() }, arguments: Default::default() };
        app.handle_command(&mut valid, None, Some("edit"), &meta(), &mut identity).await.expect("app-owned command");
        settle_contract_app(&mut app,&mut identity,cx).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let mut unknown = CommandInvocation { address: CommandAddress { command_id: "nope".into(), ..valid.address }, arguments: Default::default() };
        let error = app.handle_command(&mut unknown, None, Some("edit"), &meta(), &mut identity).await.expect_err("undeclared app command");
        assert!(error.message.contains("not owned by app"), "unexpected error: {}", error.message);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_mode_command_requires_the_active_structural_owner() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
        let mut invocation = CommandInvocation {
            address: CommandAddress { owner: CommandOwnerAddress::Mode { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into(), mode_id: "edit".into() }, command_id: "mode.increment".into() },
            arguments: Default::default(),
        };
        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.handle_command(&mut invocation, None, Some("edit"), &meta(), &mut identity).await.expect("active mode-owned command");
        settle_contract_app(&mut app,&mut identity,cx).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let error = app.handle_command(&mut invocation, None, None, &meta(), &mut identity).await.expect_err("inactive mode command");
        assert!(error.message.contains("not owned by active mode edit"), "unexpected error: {}", error.message);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn addressed_window_action_injects_the_exact_window_instance_into_the_typed_handler() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{ActionAddress, ActionInvocation};
        let invocation = ActionInvocation {
            address: ActionAddress { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into(), mode_id: "edit".into(), window_kind_id: "main".into(), window_instance_id: "main-instance-2".into(), action_id: "targetWindow".into() },
            arguments: std::collections::BTreeMap::from([("windowId".into(),DslValue::String("main-instance-2".into()))]),
        };
        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let mut args=Some(DslValue::Object(invocation.arguments.into_iter().collect()));let mut fault=None;artifact_app_laws::dispatch_addressed_fixture_on_original(&mut app.0,&invocation.address,&mut args,&mut fault,Some("edit"),&meta(),&mut identity,cx).await.expect("addressed window action");
        settle_contract_app(&mut app,&mut identity,cx).await;
        assert_eq!(app.stores.test_snapshot().await.label, "main-instance-2");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn command_op_records_history_exactly_like_an_operation_action() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::IncrementViaCommand, &meta(), &mut identity,cx).await.expect("inc");
        app.dispatch_typed(TestCommand::IncrementViaCommand, &meta(), &mut identity,cx).await.expect("inc");
        assert_eq!(app.stores.test_snapshot().await.count, 2);
        reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🕹️ `interaction_topology` requires a non-empty `label` for "item-1" to exist.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_replace_persists_through_the_interaction_store() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity, cx).await;
        let selection = app.interaction_state().await.selection.get("items").cloned().expect("items domain selected");
        assert_eq!(selection.ids, vec!["item-1".to_string()]);
        assert_eq!(selection.granularity, "item");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🕰️ The framework-injected "undo" action only ever dispatches against `self.store` (the
    /// DOCUMENT store) — with only the label-seed edit on it, one undo reverts THAT, not the pick.
    #[semio_framework_async_macros::async_test]
    async fn a_pick_is_never_undoable_the_default_undo_only_ever_walks_the_document_store() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity, cx).await;
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]));

        reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        assert_eq!(app.stores.test_snapshot().await.label, "", "undo must revert the document edit (seeding the label)");
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "the pick itself must survive an unrelated document undo — lane discipline");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
