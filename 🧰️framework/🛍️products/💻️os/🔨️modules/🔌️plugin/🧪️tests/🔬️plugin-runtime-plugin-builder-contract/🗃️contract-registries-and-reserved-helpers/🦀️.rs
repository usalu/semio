// 🗃️ Contract registries and reserved helpers: the closing dispatch enum, the reserved action helpers, the drain and close fixtures, the contract registries and the contract app wrapper.

//#region 🗝️RegisteredKeyedDispatchFixture
    semio_framework_dispatch_macros::dyn_enum_close! {
        enum TestRuntimeApps: PluginApp {
            Test(VcsArtifactApp<TestApp>),
        }
    }

    // 🚫️async: E1 pure constructor, called pervasively as `&meta()` — see R9.
    #[doc(hidden)]
pub fn meta() -> ActionMeta {
        ActionMeta { actor: "local".into(), instance_id: 1, view_state: None }
    }

    #[doc(hidden)]
pub async fn settle_reserved(app: &mut VcsArtifactApp<TestApp>, admitted: semio_framework::InvocationResult) -> semio_framework::InvocationResult {
        semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(app, admitted).await.expect("settle reserved")
    }

    #[doc(hidden)]
pub async fn reserved_action(app: &mut VcsArtifactApp<TestApp>, action: &str, args: Option<&DslValue>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> semio_framework::InvocationResult {
        let admitted = app.handle_action(action, args.cloned(), &meta(), identity).await.expect(action);
        settle_reserved(app, admitted).await
    }

    /// 🧹️ Drains every maintenance stage, then closes. A law that deliberately leaves a child root, a
    /// child member or an app-typed presence peer under explicit retirement authority cannot reach the
    /// store's terminal-empty Drop witness by `close_step` alone — that authority is exactly what
    /// `maintenance_step` pumps, one bounded unit per stage rotation.
    #[doc(hidden)]
pub fn drain_and_close_fixture(app: &mut VcsArtifactApp<TestApp>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,cx:&mut semio_framework_job::StepContext<'_>) {
        for _ in 0..64 {
            for _ in 0..MAINTENANCE_STAGES {
                let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1.min(mounted_policy.maintenance.maximum_items),..mounted_policy.maintenance};
                let demand=app.maintenance_retirement_demands(grant.maximum_copy_bytes).expect("original fixture maintenance quote");
                semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant|Ok((PluginApp::maintenance_step(app,same,grant)?,app.maintenance_terminal_is_empty()))).expect("original fixture maintenance turn");
            }
        }
        artifact_app_laws::close_registered_fixture_app(app,mounted_policy,cx).expect("original fixture terminal close");
    }

    /// 🧹️ [`drain_and_close_fixture`] for a composed fixture that owns members.
    #[doc(hidden)]
pub fn drain_and_close_composed_fixture(app: &mut VcsArtifactApp<TestApp, TestMembers>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,cx:&mut semio_framework_job::StepContext<'_>) {
        for _ in 0..64 {
            for _ in 0..MAINTENANCE_STAGES {
                let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1.min(mounted_policy.maintenance.maximum_items),..mounted_policy.maintenance};
                let demand=app.maintenance_retirement_demands(grant.maximum_copy_bytes).expect("original fixture maintenance quote");
                semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant|Ok((PluginApp::maintenance_step(app,same,grant)?,app.maintenance_terminal_is_empty()))).expect("original fixture maintenance turn");
            }
        }
        artifact_app_laws::close_registered_fixture_app(app,mounted_policy,cx).expect("original fixture terminal close");
    }

    #[doc(hidden)]
pub fn close_reserved_app(app: &mut VcsArtifactApp<TestApp>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,cx:&mut semio_framework_job::StepContext<'_>) {
        artifact_app_laws::close_registered_fixture_app(app, mounted_policy,cx).expect("original fixture terminal close");
    }

    #[doc(hidden)]
pub async fn synthetic_play_app() -> App {
        let builder = App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
            .await
            .document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await;
        App::from_builder(declare_test_app_verbs(builder).interactive_jobs(InteractiveJobClassification::Migrated).await).await
    }

    /// 🧪️ A registry-backed app declaring the contract-enforcement fixtures: an operation resolved by
    /// the context-menu label lookup, a declared-but-empty operation, a mis-behaving View action, and a
    /// utility (which auto-injects the `setActiveUtility` View action). B1: `setLabelRequired`'s
    /// required/default-arg materialization tests were deleted — that mechanism was JSON-args-specific
    /// (`AppActionRegistry`/`materialize_args`) and has no meaning for a typed `Self::Command` value a
    /// Rust caller constructs directly (a "missing required field" is a compile error, not a runtime
    /// one). `setLabelRequired` stays declared here purely as a registry fixture for the context-menu
    /// label-resolution test below. These synthetic reducers have no retained factory authority;
    /// only an exact authority test may promote its own selected row to Migrated.
    ///
    /// 🧪️ `Mutation`-kind by declaration, but `TestApp` emits zero operations for it — the
    /// "declared Mutation action that happened to produce nothing" fixture.
    #[doc(hidden)]
pub async fn contract_registry() -> AppActionRegistry {
        let app = App::from_builder(
            App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
                    .await.document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await.window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await.mutation("setLabelRequired", LocalizedLabel::data("Set Label"))
                .await.action_args("setLabelRequired", vec![ActionArgDef::text("value", LocalizedLabel::data("Value")).required()])
                .await.mutation("noopMutation", LocalizedLabel::data("Noop Mutation"))
                .await.mutation("targetWindow", LocalizedLabel::data("Target Window"))
                .await.view_action("badView", LocalizedLabel::data("Bad View"))
                .await.utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush)
                .await.app_command("incrementViaCommand", LocalizedLabel::data("Increment"), "counter", ActionKind::Mutation)
                .await.app_command("watchdogOverrun", LocalizedLabel::data("Watchdog Overrun"), "counter", ActionKind::Mutation)
                .await.app_command("setLabelViaCommand", LocalizedLabel::data("Set Label"), "counter", ActionKind::Mutation)
                .await.mode_command("edit", CommandDefinition::bounded_catalog("mode.increment", LocalizedLabel::data("Mode Increment"), "counter", ActionKind::Mutation))
                .await.interactive_jobs(InteractiveJobClassification::BatchOnlyPendingRewrite).await,
        )
        .await;
        AppActionRegistry::from_definition(&app.definition)
    }

    /// 🪪️ Every verb `TestApp::command_id` can answer, declared under the kind its reducer actually
    /// emits — `View` for the config/scope-only verbs, `Shell` for the host-effect one, `Mutation`
    /// for the document ones. `admit_command_wire_with_proof` looks a verb up here before every
    /// typed dispatch, so a verb missing from this roster is `interactive-job.unknown-key` and a
    /// verb declared non-`Migrated` is `interactive-job.not-ui-safe`.
    /// 🪪️ Every verb `TestApp::command_id` can answer, declared under the kind its reducer actually
    /// emits — `View` for the config/scope-only verbs, `Shell` for the host-effect one, `Mutation`
    /// for the document ones. `admit_command_wire_with_proof` looks a verb up here before every
    /// typed dispatch, so a verb missing from this roster is `interactive-job.unknown-key` and a
    /// verb declared non-`Migrated` is `interactive-job.not-ui-safe`.
    ///
    /// 🧵️ Synchronous on purpose: `AppBuilder`'s chain moves the whole builder through every
    /// `.await`, and an unoptimized build gives each await in ONE generator its own non-overlapping
    /// slot — twelve more of them overflowed the 2 MiB bounded thread stack that
    /// `one_framework_reserved_route_fits_a_bounded_thread_stack` pins. One `resolve_ready` per
    /// statement drops each temporary at its own semicolon instead.
    #[doc(hidden)]
pub fn declare_test_app_verbs(builder: semio_framework_plugin_app_builder::AppBuilder) -> semio_framework_plugin_app_builder::AppBuilder {
        let builder = ::semio_framework_async::poll::resolve_ready(builder.mutation("increment", LocalizedLabel::data("Increment")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.mutation("setLabel", LocalizedLabel::data("Set Label")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.mutation("streamLabel", LocalizedLabel::data("Stream Label")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.mutation("compositeEdit", LocalizedLabel::data("Composite Edit")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.mutation("probeChild", LocalizedLabel::data("Probe Child")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.mutation("spawnCountTask", LocalizedLabel::data("Spawn Count Task")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.mutation("applyCountFromTask", LocalizedLabel::data("Apply Count From Task")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.view_action("select", LocalizedLabel::data("Select")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.view_action("viewNoScope", LocalizedLabel::data("View No Scope")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.view_action("viewPartialScope", LocalizedLabel::data("View Partial Scope")));
        let builder = ::semio_framework_async::poll::resolve_ready(builder.shell_action("navigate", LocalizedLabel::data("Navigate")));
        builder
    }

    /// 🧰️ The `Migrated` twin of [`contract_registry`]. `contract_registry` itself must stay
    /// `BatchOnlyPendingRewrite`: it is the only reason the three fail-closed laws
    /// (`unproved_command_fails_before_an_overrun_reducer_can_start`,
    /// `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations`,
    /// `ui_dispatch_backstop_rejects_every_non_migrated_action_and_command`) can observe a refusal at
    /// all. Every behavioural law dispatches against THIS one instead.
    #[doc(hidden)]
pub async fn migrated_contract_registry() -> AppActionRegistry {
        let builder = App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
            .await
            .document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .mutation("setLabelRequired", LocalizedLabel::data("Set Label"))
            .await
            .action_args("setLabelRequired", vec![ActionArgDef::text("value", LocalizedLabel::data("Value")).required()])
            .await
            .mutation("noopMutation", LocalizedLabel::data("Noop Mutation"))
            .await
            .mutation("targetWindow", LocalizedLabel::data("Target Window"))
            .await
            .view_action("badView", LocalizedLabel::data("Bad View"))
            .await
            .utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush)
            .await
            .app_command("incrementViaCommand", LocalizedLabel::data("Increment"), "counter", ActionKind::Mutation)
            .await
            .app_command("watchdogOverrun", LocalizedLabel::data("Watchdog Overrun"), "counter", ActionKind::Mutation)
            .await
            .app_command("setLabelViaCommand", LocalizedLabel::data("Set Label"), "counter", ActionKind::Mutation)
            .await
            .mode_command("edit", CommandDefinition::bounded_catalog("mode.increment", LocalizedLabel::data("Mode Increment"), "counter", ActionKind::Mutation))
            .await;
        let app = App::from_builder(declare_test_app_verbs(builder).interactive_jobs(InteractiveJobClassification::Migrated).await).await;
        AppActionRegistry::from_definition(&app.definition)
    }

    /// 🧪️ A mounted `TestApp` fixture whose caller explicitly pays terminal closure. Three framework laws land here by construction
    /// instead of in every one of the 70 call sites that used to build a bare wrapper: the app is
    /// registry-backed (so a typed verb has a manifest declaration at all), it binds its live
    /// instance (`dispatch_typed_command_inner` refuses `interactive-job.live-instance` otherwise),
    /// and it retires its stores before `ArtifactStore::drop` can fire its terminal-empty witness.
    #[doc(hidden)]
pub struct ContractApp(VcsArtifactApp<TestApp>, semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1);

    impl std::ops::Deref for ContractApp {
        type Target = VcsArtifactApp<TestApp>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for ContractApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for ContractApp {
        fn drop(&mut self) {
            assert!(std::thread::panicking() || self.0.close_terminal_is_empty(), "original contract fixture requires explicit funded closure before Drop");
        }
    }
//#endregion 🗝️RegisteredKeyedDispatchFixture
