/// 📡️ Host events, intents and streamed tick: utility switch host events, forwarded host events, activate intents, view-kind hard faults and the streamed tick report.

    /// 📨️ LAW: a utility switch a dispatch's view shows is exactly one `UtilityChanged` for its window, delivered before the
    /// dispatch; the first view of a window, an unchanged utility and a window the view names no utility for deliver none.
    #[semio_framework_async_macros::async_test]
    async fn a_utility_switch_in_a_dispatch_view_delivers_one_host_event_to_its_window() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        take_test_host_events();
        let under = |view: ViewModel| ActionMeta { view_state: Some(view), ..meta() };
        app.dispatch_typed(TestCommand::Increment, &under(host_event_view("main-1", Some("select"))), &mut identity,cx).await.expect("the first view");
        app.dispatch_typed(TestCommand::Increment, &under(host_event_view("main-1", Some("select"))), &mut identity,cx).await.expect("the same utility");
        assert_eq!(take_test_host_events(), Vec::new(), "neither the first view of a window nor an unchanged utility is a switch");
        app.dispatch_typed(TestCommand::Increment, &under(host_event_view("main-1", Some("brush"))), &mut identity,cx).await.expect("the switch");
        assert_eq!(take_test_host_events(), vec![HostEvent::UtilityChanged { window_id: "main-1".into(), from: Some("select".into()), to: Some("brush".into()) }]);
        app.dispatch_typed(TestCommand::Increment, &under(host_event_view("main-2", None)), &mut identity,cx).await.expect("another window");
        assert_eq!(take_test_host_events(), Vec::new(), "main-2 is seen first, and this view names no utility for main-1");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📨️ LAW: a forwarded `hostEvent{windowId, kind}` reaches the app as its typed event for that window, the app's typed
    /// answer dispatches without a history row of its own, and an unknown kind is refused.
    #[semio_framework_async_macros::async_test]
    async fn a_forwarded_host_event_reaches_the_app_and_its_answer_is_never_a_history_row() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        take_test_host_events();
        let under = ActionMeta { view_state: Some(host_event_view("main-1", None)), ..meta() };
        for (kind, event) in [
            ("blur", HostEvent::WindowBlurred { window_id: "main-2".into() }),
            ("captureLost", HostEvent::PointerCaptureLost { window_id: "main-2".into() }),
            ("retiring", HostEvent::Retiring { window_id: "main-2".into() }),
        ] {
            app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(dv(json!({ "windowId": "main-2", "kind": kind }))), &under, &mut identity).await.unwrap_or_else(|fault| panic!("{kind}: {fault:?}"));
            assert_eq!(take_test_host_events(), vec![event], "{kind}");
        }
        let history = app.render_ctx().test_history().await;
        assert!(history.commands.is_empty(), "the answer to a blur is never a history row: {:?}", history.commands.iter().map(|entry| &entry.action_id).collect::<Vec<_>>());
        let refused = app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(dv(json!({ "windowId": "main-1", "kind": "wobble" }))), &under, &mut identity).await;
        assert_eq!(refused.err().map(|fault| fault.code), Some(FaultCode::new("hostEvent.invalid")));

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`) acceptance: an Activate intent on a test app
    /// produces the mutation's document change AND a command-log entry within the SAME
    /// `handle_intent_frame` call — the reactor's own `poll` never needs a second turn to see it,
    /// since `plugin_dispatch_intent` (which calls this) runs before the SAME turn's
    /// `dirty_render` pass.
    #[semio_framework_async_macros::async_test]
    async fn activate_intent_dispatches_through_the_typed_command_path_same_turn() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let intent = UiIntent {
            surface: SurfaceId::try_from("s").expect("bounded fixture"),
            revision: UiRevision(0),
            node: UiNodeId::default(),
            node_key: UiText::try_from_str("counter").expect("bounded fixture"),
            trigger: Trigger::Activate,
            action: ActionId::try_v1("app", "incrementViaCommand").expect("bounded fixture"),
            args: None,
            input: None,
            seq: 1,
        };
        let result = handle_intent_settled(&mut app, &intent, &meta(), &mut identity,cx).await.expect("an Activate intent on a Mutation-kind action must dispatch");
        assert_eq!(app.stores.test_snapshot().await.count, 1, "the mutation must have applied");
        assert_eq!(result.mutations.len(), 1, "the returned invocation carries the one mutation the intent caused");
        let history = app.render_ctx().test_history().await;
        assert!(history.commands.iter().any(|entry| entry.action_id == "incrementViaCommand"), "a command-log entry must have been recorded for the intent");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎯️ M1 acceptance: a `View`-kind action arriving as an intent and returning artifact ops
    /// still hard-faults — kind discipline survives the new intent path unchanged, because
    /// `handle_intent_frame` routes through the SAME `dispatch_typed_command_inner` every other
    /// command path uses.
    #[semio_framework_async_macros::async_test]
    async fn view_kind_intent_returning_operations_hard_faults() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let intent = UiIntent {
            surface: SurfaceId::try_from("s").expect("bounded fixture"),
            revision: UiRevision(0),
            node: UiNodeId::default(),
            node_key: UiText::try_from_str("bad").expect("bounded fixture"),
            trigger: Trigger::Activate,
            action: ActionId::try_v1("app", "badView").expect("bounded fixture"),
            args: None,
            input: None,
            seq: 2,
        };
        let error = handle_intent_settled(&mut app, &intent, &meta(), &mut identity,cx).await.expect_err("a View-kind intent emitting operations must be rejected");
        assert!(matches!(&error.primary,semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(fault) if fault.message.contains("must not emit operations")), "unexpected complete original error: {error:?}");
        assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default(), "kind discipline must block the mutation");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🪢️ Regression guard for `result_from_last_edit`'s `tail_offset` slicing: even though the
    /// transaction's open edit accumulates every tick's operations (3 after this loop), each dispatch's
    /// `InvocationResult` must report only the operation IT just added — never re-serializing the whole
    /// growing edit into every `KernelMutation`/`UndoGroup` on every single dispatch.
    ///
    /// The narrowed per-dispatch reporting must not affect the one-edit/undo semantics of the transaction.
    #[semio_framework_async_macros::async_test]
    async fn a_streamed_tick_reports_only_its_own_new_operation() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::StreamLabel { value: "a".into(), commit: false }, &meta(), &mut identity,cx).await.expect("streamLabel a");
        app.dispatch_typed(TestCommand::StreamLabel { value: "ab".into(), commit: false }, &meta(), &mut identity,cx).await.expect("streamLabel ab");
        let result = app.dispatch_typed(TestCommand::StreamLabel { value: "abc".into(), commit: false }, &meta(), &mut identity,cx).await.expect("streamLabel abc");
        assert_eq!(result.mutations.len(), 1, "must report only this dispatch's new operation, not the whole open edit");
        assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetLabel(SetLabel { value: "abc".into() })).unwrap());
        assert_eq!(
            result.mutations[0].inverse.inverse_diff.payload,
            protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetLabel(SetLabel { value: "ab".into() })).unwrap()]),
            "the new operation's own inverse undoes back to the pre-dispatch label, not the whole gesture"
        );
        assert_eq!(result.inverse_group.mutations.len(), 1);
        assert_eq!(result.inverse_group.inverse_mutations.len(), 1);
        assert_eq!(app.stores.test_snapshot().await.label, "abc");
        app.dispatch_typed(TestCommand::StreamLabel { value: "abcd".into(), commit: true }, &meta(), &mut identity,cx).await.expect("streamLabel commit");
        reserved_action(&mut app, "undo", None, &mut identity, cx).await;
        assert_eq!(app.stores.test_snapshot().await.label, "");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
