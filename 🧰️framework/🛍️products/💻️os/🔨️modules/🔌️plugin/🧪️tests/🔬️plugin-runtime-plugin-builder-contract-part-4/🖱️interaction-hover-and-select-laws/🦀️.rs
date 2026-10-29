/// 🖱️ Interaction hover and select laws: ephemeral hover, select job completion, empty replace and the guest emitted select folds.

    /// 🐁️ Empty targets clears the channel (see `next_hover`'s "empty batch clears" law).
    #[semio_framework_async_macros::async_test]
    async fn interaction_hover_is_ephemeral_and_never_touches_the_persisted_interaction_store() {
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
        let edits_after_select = app.stores.interaction_store.envelope().vcs.edits.len();

        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity, cx).await;

        assert_eq!(app.stores.interaction_store.envelope().vcs.edits.len(), edits_after_select, "hover must never mint a persisted interaction_store edit");
        assert_eq!(app.interaction_state().await.hover.get("items").map(|hover| hover.ids.clone()), Some(vec!["item-1".to_string()]));

        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&dv(json!({ "domainId": "items", "channel": "pointer", "targets": "[]" }))), &mut identity, cx).await;
        assert!(app.interaction_state().await.hover.get("items").is_none(), "an emptied hover channel is removed, not left as an empty entry");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 W-G3 §8.21 — `interactionSelect` leftover `Invocation.output` carries InteractionView selected ids + lock.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_job_completion_publishes_interaction_view_on_leftover() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity, cx).await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.iter().any(|id| id.as_str() == Some("item-1")), "leftover selected ids {ids:?}");
        let locked = view.get("locked").expect("lock state included on leftover InteractionView");
        assert_eq!(locked.get("item-1").and_then(semio_framework_value::DslValue::as_bool), Some(false));
        let gumball = view.get("gumball").expect("gumball leftover");
        assert_eq!(gumball.get("active").and_then(semio_framework_value::DslValue::as_bool), Some(false), "leftover selection must not arm the world gumball — the guest selectionJson lane owns utility-aware gumballActive");
        assert_eq!(gumball.get("anchorId").and_then(semio_framework_value::DslValue::as_str), Some("item-1"));
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 Empty-target `interactionSelect` with `replace` clears the domain (background deselect) even when an object is hovered.
    #[semio_framework_async_macros::async_test]
    async fn emitted_empty_replace_write_clears_selection_while_hover_remains() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity, cx).await;
        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity, cx).await;
        let clear = Emit { interaction_writes: vec![InteractionWrite::replace("items", "object", std::iter::empty::<String>())], ..Default::default() };
        app.test_dispatch_emit("canvasPointerDown", clear, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("explicit selection clear lands");
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert!(snapshot.selection.get("items").is_none_or(|selection| selection.ids.is_empty()));
        assert!(snapshot.selection.get("items").is_none_or(|selection| selection.anchor_id.is_none()));
        assert_eq!(app.interaction_state().await.hover.get("items").map(|hover| hover.ids.as_slice()), Some(["item-1".to_string()].as_slice()));
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_target_interaction_select_clears_selection_while_hover_remains() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity, cx).await;
        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity, cx).await;
        let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_empty_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }))), &mut identity, cx).await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.is_empty(), "empty-target replace interactionSelect must clear selectedIds, got {ids:?}");
        let hover = view.get("hoverTarget").expect("hoverTarget");
        assert_eq!(hover.get("id").and_then(semio_framework_value::DslValue::as_str), Some("item-1"));
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — a guest-emitted `interactionSelect` is applied
    /// in the SAME turn as the app command that carried it: the effect never reaches the host, the
    /// selection snapshot already moved when the carrier's result leaves, the carrier's `ui_scope` is
    /// widened by the verb's app-declared interaction refresh scope, and the leftover `InteractionView`
    /// rides the carrier's own `output` (the lane a host-dispatched verb publishes on). Every other
    /// effect still reaches the host untouched, in order. Fails-before: the effect was handed to the
    /// host and the pick landed one guest round trip later, behind any pointer input issued meanwhile.
    #[semio_framework_async_macros::async_test]
    async fn guest_emitted_interaction_select_is_folded_into_the_carrying_turn() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let emit = Emit {
            effects: vec![Effect::Navigate { uri: "semio://home".into() }, inline_interaction_select(interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))],
            ui_scope: UiDirtyScope::None,
            ..Default::default()
        };
        let result = app.test_dispatch_emit("canvasPointerUp", emit, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the carrying command lands");
        assert_eq!(result.requested_effects, vec![Effect::Navigate { uri: "semio://home".into() }], "the interaction verb never reaches the host; every other effect does, in order");
        assert!(result.diagnostics.is_empty(), "a well-formed verb applies without diagnostics: {:?}", result.diagnostics);
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert_eq!(snapshot.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "the selection snapshot moved inside the carrying turn");
        let declared = <TestApp<false> as ArtifactApp>::interaction_scope(InteractionVerb::Select, &["items"]).expect("the fixture declares its select scope");
        assert_eq!(result.ui_scope, declared, "a `None` carrier scope widens to exactly the verb's declared interaction refresh scope");
        let view = result.output.get("interactionView").expect("the leftover InteractionView rides the carrier's output");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.iter().any(|id| id.as_str() == Some("item-1")), "leftover selected ids {ids:?}");
        let history = app.render_ctx().test_history().await;
        assert!(!history.commands.iter().any(|entry| entry.action_id == INTERACTION_SELECT_ACTION_ID || entry.kind == ActionKind::Interaction), "the folded verb is never a history row");
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ A second carrier whose folded `interactionSelect` names no target with `replace` clears the
    /// selection (the background-click deselect), and its scope is the UNION of the carrier's own
    /// partial scope and the verb's declared partial scope — widest wins, nothing the app asked to
    /// refresh is dropped.
    #[semio_framework_async_macros::async_test]
    async fn guest_emitted_empty_replace_select_clears_and_unions_the_carrier_scope() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let pick = Emit { effects: vec![inline_interaction_select(interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))], ui_scope: UiDirtyScope::None, ..Default::default() };
        app.test_dispatch_emit("canvasPointerUp", pick, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("pick lands");
        assert_eq!(app.interaction.test_interaction_selection_snapshot(&app.stores).selection.get("items").map(|selection| selection.ids.len()), Some(1));

        let carrier_scope = UiDirtyScope::Partial { window_bodies: vec!["some.window".into()], panel_bodies: Vec::new(), utilities: true, tools: false, engagements: false, measures: false, labels: false };
        let clear = Emit { effects: vec![inline_interaction_select(interaction_empty_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" })))], ui_scope: carrier_scope, ..Default::default() };
        let result = app.test_dispatch_emit("canvasPointerDown", clear, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("clear lands");
        assert!(result.requested_effects.is_empty(), "nothing is left for the host to replay: {:?}", result.requested_effects);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert!(snapshot.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "an empty-target replace clears the domain inline: {:?}", snapshot.selection);
        let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, .. } = &result.ui_scope else { panic!("two partial scopes union into a partial scope, got {:?}", result.ui_scope) };
        assert!(window_bodies.iter().any(|body| body == "some.window"), "the carrier's own window body survives the merge: {window_bodies:?}");
        assert!(window_bodies.iter().any(|body| body == TEST_APP_WINDOW_BODY_KEY), "the verb's declared window body joins it: {window_bodies:?}");
        assert!(panel_bodies.iter().any(|body| body == FRAMEWORK_HISTORY_BODY_KEY), "the verb's declared panel body joins it: {panel_bodies:?}");
        assert!(*utilities, "the carrier's own section flag survives the merge");
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ The second emission shape for the same intent — `Effect::DispatchAction { action ∈
    /// INTERACTION_ACTION_IDS }`, what `📏️layout`'s `layout_select_effect` and `💡️reasoning`'s
    /// `wires_select_effect` build — folds identically to the `ReplayShellCommand` shape: the
    /// effect never reaches the host, the selection snapshot moved inside the carrying turn, the
    /// carrier's scope widens to the verb's declared interaction refresh scope, the leftover
    /// `InteractionView` rides the carrier's `output`, and the surrounding effects keep their order.
    /// The placeholder `req` an app literal carries is awaited by nothing, so dropping it is silent.
    #[semio_framework_async_macros::async_test]
    async fn guest_emitted_dispatch_action_interaction_select_is_folded_into_the_carrying_turn() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let verb = Effect::DispatchAction { req: RequestId(115), action: INTERACTION_SELECT_ACTION_ID.into(), args: Some(interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), delay_ms: 0 };
        let emit = Emit { effects: vec![Effect::Navigate { uri: "semio://home".into() }, verb], ui_scope: UiDirtyScope::None, ..Default::default() };
        let result = app.test_dispatch_emit("canvasPointerDown", emit, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the carrying command lands");
        assert_eq!(result.requested_effects, vec![Effect::Navigate { uri: "semio://home".into() }], "the DispatchAction interaction verb never reaches the host; every other effect does, in order");
        assert!(result.diagnostics.is_empty(), "a well-formed verb applies without diagnostics: {:?}", result.diagnostics);
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert_eq!(snapshot.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "the selection snapshot moved inside the carrying turn");
        let declared = <TestApp<false> as ArtifactApp>::interaction_scope(InteractionVerb::Select, &["items"]).expect("the fixture declares its select scope");
        assert_eq!(result.ui_scope, declared, "a `None` carrier scope widens to exactly the verb's declared interaction refresh scope");
        let view = result.output.get("interactionView").expect("the leftover InteractionView rides the carrier's output");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.iter().any(|id| id.as_str() == Some("item-1")), "leftover selected ids {ids:?}");
        let history = app.render_ctx().test_history().await;
        assert!(!history.commands.iter().any(|entry| entry.action_id == INTERACTION_SELECT_ACTION_ID || entry.kind == ActionKind::Interaction), "the folded verb is never a history row");
        close_reserved_app(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
