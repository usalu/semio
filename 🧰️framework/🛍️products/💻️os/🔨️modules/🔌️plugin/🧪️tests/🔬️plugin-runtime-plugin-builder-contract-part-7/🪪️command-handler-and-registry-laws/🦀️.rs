/// 🪪️ Command handler and registry laws: the non-v1 action version, the dispatch backstop, the program-owned command handler, registry-less construction, flat interaction refusal and the leftover typed operation merge.

    #[semio_framework_async_macros::async_test]
    async fn plugin_command_handler_is_program_owned_across_app_instances() {
        use semio_framework::kernel::{InvocationId, InvocationResult, UndoGroup};
        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_calls = calls.clone();
        let plugin: Plugin = Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
            CommandDefinition::bounded_catalog("refresh", LocalizedLabel::data("Refresh"), "plugin", ActionKind::Shell),
            Box::new(move |_, meta| {
                handler_calls.fetch_add(meta.instance_id as usize, Ordering::SeqCst);
                Ok(InvocationResult {
                    output: semio_framework_value::DslValue::Null,
                    mutations: Vec::new(),
                    inverse_group: UndoGroup { invocation_id: InvocationId(String::new()), mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() },
                    diagnostics: Vec::new(),
                    requested_effects: Vec::new(),
                    events: Vec::new(),
                    ui_scope: UiDirtyScope::None,
                    history_patch: None,
                })
            }),
        );
        let mut invocation = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::Plugin { plugin_id: "fixture".into() }, command_id: "refresh".into() }, arguments: Default::default() };
        plugin.handle_plugin_command(&invocation, &ActionMeta { actor: "a".into(), instance_id: 1, view_state: None }).expect("first app instance");
        plugin.handle_plugin_command(&invocation, &ActionMeta { actor: "b".into(), instance_id: 2, view_state: None }).expect("second app instance");
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_less_construction_rejects_before_the_reducer() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_FACTORIES>>::new(TestApp::<false, TEST_APP_TOOLS_FACTORIES>::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await.expect("the reference snapshot births");
        let error = semio_framework_plugin_testing_laws::artifact_app_laws::dispatch_original_fixture_command(&mut app, &TestCommand::BadView, &meta(), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut identity, cx).await.expect_err("an empty registry must fail closed");
        assert_eq!(error.primary_fault().expect("a diagnostic fault").code.0, "interactive-job.unknown-key");
        assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default());
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_transitive_flat_interaction() {
        let __base = App::builder("bad-transitive-flat", LocalizedLabel::data("Bad"))
            .await
            .document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "bad.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await;
        let outcome = std::panic::catch_unwind(move || {
            let builder = ::semio_framework_async::poll::resolve_ready(__base.interaction(InteractionDefinition {
                id: "items".into(),
                label: LocalizedLabel::data("Items"),
                granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
            }));
            ::semio_framework_async::poll::resolve_ready(App::from_builder(builder))
        });
        assert!(outcome.is_err(), "build_definition must reject transitive hover paired with HierarchyProvider::Flat");
    }

    /// 🌀️ The widest, always-correct answer is what is left when there is nothing to derive from: no
    /// window kind declares the touched domain, so the framework may not guess which body paints it.
    #[semio_framework_async_macros::async_test]
    async fn a_domain_no_window_declares_keeps_the_framework_full_scope() {
        let definition = interaction_app_definition().await;
        let windows = semio_framework::interaction_window_bodies_by_domain(&definition);
        let panels = semio_framework::panel_leaf_body_keys(&definition);
        for verb in InteractionVerb::ALL {
            assert!(semio_framework::interaction_declared_refresh_scope(verb, &["nowhere"], &windows, &panels).is_none(), "{verb:?} narrowed a domain no window declares");
            assert!(semio_framework::interaction_declared_refresh_scope(verb, &[], &windows, &panels).is_none(), "{verb:?} narrowed a verb that touched no domain");
        }
        assert_eq!(windows.get("items").map(Vec::as_slice), Some([TEST_APP_WINDOW_BODY_KEY.to_string()].as_slice()), "the derivation reads the window kind's own interaction refs");
    }

    /// 🎮️ Wave B34 — the verb type the declaration is keyed by is exactly the six ids
    /// `INTERACTION_ACTION_IDS` intercepts, in both directions, so a seventh verb cannot be dispatched
    /// without an author deciding what it dirties.
    #[semio_framework_async_macros::async_test]
    async fn every_intercepted_interaction_id_round_trips_through_its_verb() {
        for verb in InteractionVerb::ALL {
            assert_eq!(InteractionVerb::of_action(verb.action_id()), Some(verb), "{verb:?}");
        }
        assert_eq!(InteractionVerb::ALL.len(), semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS.len());
        for action in semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS {
            assert!(InteractionVerb::of_action(action).is_some(), "{action} is intercepted but names no verb");
        }
        assert_eq!(InteractionVerb::of_action("undo"), None);
    }

    /// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the leftover a typed operation's folded verb
    /// owes the host rides the `Invocation` reply of the command that STARTED that operation
    /// (`merge_typed_operation_leftover_into_reply`, run by `plugin_exchange` after its typed pump):
    /// the reply's `{ operationId, generation }` witness keeps both keys and gains `interactionView`,
    /// the envelope `interactionViewFromLeftoverOutput` peels from `response.output`. A reply that
    /// answers another operation, an unsolicited (`in_reply_to: 0`) frame and a non-`Invocation`
    /// frame are left byte-identical. Fails-before: the typed ladder dropped `fold.leftover`, so the
    /// host's leftover overlay / Inspection tab never moved for a folded pick.
    #[test]
    fn typed_operation_leftover_merges_into_exactly_the_carrying_reply() {
        let started = |operation: &str| store::pack_rt::encode_wire_value(&semio_framework_value::DslValue::Object(vec![("operationId".into(), semio_framework_value::DslValue::String(operation.into())), ("generation".into(), semio_framework_value::DslValue::String("7".into()))]));
        let invocation = |in_reply_to: u64, output: Vec<u8>| protocol::AppFrame::Invocation { in_reply_to, output, diagnostics: Vec::new(), ui_scope: Vec::new(), history_patch: Vec::new(), messages: Vec::new(), mutations: Vec::new(), inverse_group: Vec::new() };
        let view = semio_framework_value::DslValue::Object(vec![("interactionView".into(), semio_framework_value::DslValue::Object(vec![("selectedIds".into(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String("item-1".into())]))]))]);
        let leftover = semio_framework_plugin_job::TypedOperationLeftover { operation: 42, view };
        let mut frames = vec![protocol::AppFrame::Done { in_reply_to: 1 }, invocation(2, started("41")), invocation(0, started("42")), invocation(3, started("42"))];
        let untouched: Vec<Vec<u8>> = frames.iter().take(3).map(|frame| ::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(frame))).collect();
        assert!(semio_framework_plugin_runtime_ops::ops_typed_operations::merge_typed_operation_leftover_into_reply(&mut frames, &leftover), "the reply that started operation 42 receives the leftover");
        for (index, before) in untouched.iter().enumerate() {
            assert_eq!(&::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&frames[index])), before, "frame {index} answers no fold and stays byte-identical");
        }
        let protocol::AppFrame::Invocation { in_reply_to: 3, output, .. } = &frames[3] else { panic!("the carrying reply keeps its sequence") };
        let merged = store::pack_rt::decode_wire_value(output).expect("merged output decodes");
        assert_eq!(merged.get("operationId").and_then(semio_framework_value::DslValue::as_str), Some("42"), "the operation witness the host awaits survives: {merged:?}");
        assert_eq!(merged.get("generation").and_then(semio_framework_value::DslValue::as_str), Some("7"));
        let ids = merged.get("interactionView").and_then(|view| view.get("selectedIds")).and_then(semio_framework_value::DslValue::as_array).expect("interactionView joins the witness");
        assert_eq!(ids.iter().filter_map(semio_framework_value::DslValue::as_str).collect::<Vec<_>>(), vec!["item-1"]);
        let mut none = vec![invocation(4, started("40"))];
        assert!(!semio_framework_plugin_runtime_ops::ops_typed_operations::merge_typed_operation_leftover_into_reply(&mut none, &leftover), "no reply of this exchange started the folding operation");
    }
