/// 🎚️ Selection modes, verbs and hover: selection mode and granularity, clear and select all, stale selection pruning, interaction verbs, hover lanes and replay shell commands reaching the host.

    /// 🛂️ An undeclared granularity is rejected, not silently accepted — `handle_action` only ADMITS
    /// a framework-reserved verb (`FrameworkSetInteractionGranularityJob`), so the refusal rides the
    /// reserved job's commit, exactly where `reserved_action` above drives the accepted ones.
    #[semio_framework_async_macros::async_test]
    async fn set_selection_mode_and_set_interaction_granularity_persist_immediately() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        reserved_action(&mut app, SET_SELECTION_MODE_ACTION_ID, Some(&dv(json!({ "domainId": "items", "mode": "single" }))), &mut identity).await;
        assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(SelectionMode::Single));

        reserved_action(&mut app, SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&dv(json!({ "domainId": "items", "granularityId": "item" }))), &mut identity).await;
        assert_eq!(app.interaction_state().await.active_granularity.get("items").map(String::as_str), Some("item"));

        let admitted = app.handle_action(SET_INTERACTION_GRANULARITY_ACTION_ID, Some(dv(json!({ "domainId": "items", "granularityId": "bogus" }))), &meta(), &mut identity).await.expect("undeclared granularity admission");
        let error = semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut *app, admitted).await.expect_err("undeclared granularity must be rejected");
        assert!(error.message.contains("bogus"), "unexpected error: {}", error.message);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn clear_selection_and_select_all_apply_across_every_declared_domain() {
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

        reserved_action(&mut app, SELECT_ALL_ACTION_ID, None, &mut identity).await;
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "selectAll must select every id `interaction_topology` reports for the declared granularity");

        reserved_action(&mut app, CLEAR_SELECTION_ACTION_ID, None, &mut identity).await;
        assert!(app.interaction_state().await.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "clearSelection must empty every declared domain's selection");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧹️ `TestApp::interaction_topology` reports NO ids once `label` is empty again — simulates
    /// "item-1 was deleted from the document" — task 4: revalidated after EVERY artifact dispatch.
    #[semio_framework_async_macros::async_test]
    async fn validate_state_prunes_a_stale_selection_id_after_the_document_deletes_it() {
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
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]));

        app.dispatch_typed(TestCommand::SetLabel { value: "".into() }, &meta(), &mut identity,cx).await.expect("delete item-1 (empty label)");

        assert!(app.interaction_state().await.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "the deleted id must be pruned from selection automatically");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🙈️ Interaction verbs (pick, hover, clear selection) change no history, so they are never history rows:
    /// only the seeding edit stays in the log.
    #[semio_framework_async_macros::async_test]
    async fn interaction_verbs_never_become_history_rows() {
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
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity).await;
        reserved_action(&mut app, CLEAR_SELECTION_ACTION_ID, None, &mut identity).await;
        let history = app.render_ctx().test_history().await;
        let kinds: Vec<(&str, ActionKind)> = history.commands.iter().map(|entry| (entry.action_id.as_str(), entry.kind)).collect();
        assert_eq!(kinds, vec![("setLabel", ActionKind::Mutation)], "only the seeding edit is a row");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 W-G3 §8.30 — leftover `interactionSelect` of an object id on a vortex-domain-shaped pick still lands in `selectedIds`.
    #[semio_framework_async_macros::async_test]
    async fn leftover_interaction_select_object_id_on_vortex_domain_lands_in_selected_ids() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(
            &mut app,
            INTERACTION_SELECT_ACTION_ID,
            Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "seed-left-001")),
         &mut identity)
        .await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.iter().any(|id| id.as_str() == Some("seed-left-001")), "leftover selected ids {ids:?}");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 W-G3 §8.21 — `interactionHover` leftover publishes the hover target on the same leftover output.
    #[semio_framework_async_macros::async_test]
    async fn interaction_hover_job_completion_publishes_hover_target_on_leftover() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity).await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let hover = view.get("hoverTarget").expect("hover target");
        assert_eq!(hover.get("id").and_then(semio_framework_value::DslValue::as_str), Some("item-1"));
        assert_eq!(hover.get("domain").and_then(semio_framework_value::DslValue::as_str), Some("items"));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🐁️ Wave B34 — the plugin-host half of the app-declared interaction scope: an `interactionHover`
    /// dirties ONLY the hover lane of its own window. The framework used to answer `UiDirtyScope::Full`
    /// for all six `INTERACTION_ACTION_IDS`, hover included, so every pointer move repainted every
    /// window body, every panel body, the utilities/tools/engagements rails, the labels and the measures
    /// — measured at 24 interaction ingresses and 3 whole-shell completions per 80 s browser lane
    /// (ticket 26/09/02 `📓️2026-09-12-wave-B32-world-lane-after-completion.md` §3.2).
    #[semio_framework_async_macros::async_test]
    async fn interaction_hover_dirties_only_the_hovering_windows_own_lane() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity).await;
        let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = settled.ui_scope else {
            panic!("a declared hover scope is never Full or None: {:?}", settled.ui_scope);
        };
        assert_eq!(window_bodies, vec![TEST_APP_WINDOW_BODY_KEY.to_string()], "a hover dirties the hovering window's own body and no other");
        assert!(panel_bodies.is_empty(), "a hover cannot dirty a panel: {panel_bodies:?}");
        assert!(!utilities && !tools && !engagements && !measures && !labels, "a hover cannot dirty the shell chrome");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🖱️ Wave B34 — the same hop for a PICK: the declaring app's answer is carried through verbatim,
    /// panel bodies included, so an app can widen a select beyond a hover without widening the hover.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_carries_the_apps_declared_selection_lane() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        assert_eq!(
            settled.ui_scope,
            UiDirtyScope::Partial {
                window_bodies: vec![TEST_APP_WINDOW_BODY_KEY.to_string()],
                panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()],
                utilities: false,
                tools: false,
                engagements: false,
                measures: false,
                labels: false,
            },
            "a pick publishes the app's declared selection scope verbatim"
        );
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🈳️ The framework DEFAULT: a verb the app declares no scope for falls through to the scope
    /// DERIVED from the app's own surface declarations (`interaction_declared_refresh_scope` over
    /// `WindowKindDefinition.interactions`) — the window that accepts the domain, the Select chrome,
    /// and no panel, because `setInteractionGranularity` moves no selection. `TestApp` declares
    /// nothing for this verb precisely so this arm has a fixture, and `UiDirtyScope::Full` is now
    /// reached only when not even a window declares the touched domain (the sibling law below).
    #[semio_framework_async_macros::async_test]
    async fn an_undeclared_interaction_verb_falls_through_to_the_declared_surface_scope() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&dv(json!({ "domainId": "items", "granularityId": "item" }))), &mut identity).await;
        assert_eq!(
            settled.ui_scope,
            UiDirtyScope::Partial { window_bodies: vec![TEST_APP_WINDOW_BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: true, labels: false },
            "an app that declares the domain on a window narrows to that window without declaring a scope hook"
        );
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ A folded verb that cannot be applied (an undeclared domain here) must not fail the app
    /// command that carried it — the pick already happened — so it lands as one diagnostic on the
    /// carrier's result under `INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE`, the effect is still
    /// consumed (the host is never asked to retry a verb the reactor refused), and the selection is
    /// untouched. A `Full` carrier scope stays `Full`.
    #[semio_framework_async_macros::async_test]
    async fn guest_emitted_interaction_select_on_an_undeclared_domain_is_a_diagnostic_not_a_fault() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let bad = Emit { effects: vec![inline_interaction_select(interaction_target_args(json!({ "domainId": "ghosts", "merge": "replace", "method": "pick" }), "item-1"))], ui_scope: UiDirtyScope::Full, ..Default::default() };
        let result = app.test_dispatch_emit("canvasPointerUp", bad, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("a refused inline verb never fails its carrier");
        assert!(result.requested_effects.is_empty(), "the refused verb is consumed, not bounced to the host: {:?}", result.requested_effects);
        assert_eq!(result.diagnostics.len(), 1, "{:?}", result.diagnostics);
        assert_eq!(result.diagnostics[0].code.0, semio_framework_plugin_interaction::interaction_topology::INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE);
        assert!(result.diagnostics[0].message.contains("ghosts"), "the diagnostic names the refused domain: {}", result.diagnostics[0].message);
        assert_eq!(result.ui_scope, UiDirtyScope::Full);
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert!(snapshot.selection.values().all(|selection| selection.ids.is_empty()), "a refused verb moves nothing: {:?}", snapshot.selection);
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ The fold is scoped to the six interaction verbs: an undo/redo inverse replay or an `os.*`
    /// shell relay riding the same `ReplayShellCommand` variant keeps reaching the host untouched.
    #[semio_framework_async_macros::async_test]
    async fn non_interaction_replay_shell_commands_still_reach_the_host() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let relay = Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: Some(semio_framework_value::DslValue::from(&json!({ "themeId": "light" }))) };
        let result = app.test_dispatch_emit("relayTheme", Emit { effects: vec![relay.clone()], ui_scope: UiDirtyScope::None, ..Default::default() }, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("relay lands");
        assert_eq!(result.requested_effects, vec![relay]);
        assert_eq!(result.ui_scope, UiDirtyScope::None, "no verb was folded, so no scope was widened");
        assert!(result.diagnostics.is_empty());
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ The `DispatchAction` fold is scoped to the six interaction verbs exactly like the
    /// `ReplayShellCommand` one: a self-dispatch chain (`probeChildContinuation`, a delayed
    /// staged-work tick) riding the same variant keeps reaching the host untouched, `req` and
    /// `delay_ms` intact.
    #[semio_framework_async_macros::async_test]
    async fn non_interaction_dispatch_actions_still_reach_the_host() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let chain = Effect::DispatchAction { req: RequestId(91_002), action: "probeChildContinuation".into(), args: Some(semio_framework_value::DslValue::from(&json!({ "pass": 2 }))), delay_ms: 16 };
        let result = app.test_dispatch_emit("stagedPass", Emit { effects: vec![chain.clone()], ui_scope: UiDirtyScope::None, ..Default::default() }, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("chain lands");
        assert_eq!(result.requested_effects, vec![chain]);
        assert_eq!(result.ui_scope, UiDirtyScope::None, "no verb was folded, so no scope was widened");
        assert!(result.diagnostics.is_empty());
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert!(snapshot.selection.values().all(|selection| selection.ids.is_empty()), "a non-interaction DispatchAction moves no selection: {:?}", snapshot.selection);
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
