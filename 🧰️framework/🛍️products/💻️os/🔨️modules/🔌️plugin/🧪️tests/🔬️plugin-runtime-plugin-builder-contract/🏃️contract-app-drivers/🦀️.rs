// 🏃️ Contract app drivers: the contract app settle and construction drivers and the composed contract app wrapper.

//#region 🗝️RegisteredKeyedDispatchFixture
    impl ContractApp {
        /// 🔁️ Shadows `VcsArtifactApp::dispatch_typed` through the inherent-method-before-`Deref`
        /// rule: a mounted app answers with an ADMISSION receipt and hands the reducer to a worker,
        /// so a law that read the store right after a bare `dispatch_typed` observed the previous
        /// revision. Every dispatch here therefore settles its own operation before returning, and
        /// the returned result carries the settled effects/events/scope rather than the admission's
        /// empty ones. `mutations` stays empty by construction — read the store or the edit log.
        ///
        /// 🧩️ A COMPOSED gesture's result is published by the ladder itself, on its own bounded
        /// host outbox, so the host reads the parent's and every child's mutation under its own
        /// handle plus the single `UndoGroup` that names them — nothing is rebuilt here.
        /// 🪢️ The settle-receipt half: a migrated dispatch answers with an admission whose `mutations`
        /// and `inverse_group` are empty by construction, because the document only advances once the
        /// worker's emit has walked the publication ladder above. Rebuilt here through the SAME
        /// `result_from_last_edit` the unmigrated route calls, with the SAME `amended_same_edit` tail
        /// rule, so a coalesced gesture still reports only the operation THIS dispatch added.
        pub async fn dispatch_typed(&mut self, command: TestCommand, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework::InvocationResult, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
            let verb = test_command_id(&command);
            let before_edit_id = self.0.stores.test_last_edit_id();
            let before_tail = self.0.stores.test_edit_tail_lengths();
            let mut admitted = artifact_app_laws::dispatch_original_fixture_command(&mut self.0, &command, meta, artifact_app_laws::fixture_mounted_policy(), identity, cx).await?;
            let receipt = artifact_app_laws::settle_registered_typed_operation(&mut self.0, meta.instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), identity,cx).await?;
            admitted.requested_effects.extend(receipt.effects);
            admitted.events.extend(receipt.events);
            if let Some(scope) = receipt.ui_scope {
                admitted.ui_scope = scope;
            }
            let after_edit_id = self.0.stores.test_last_edit_id();
            if after_edit_id.is_some() {
                let tail_offset = if after_edit_id == before_edit_id { before_tail } else { (0, 0) };
                let settled = self.0.stores.test_result_from_last_edit(verb, meta, tail_offset).await;
                admitted.mutations = settled.mutations;
                admitted.inverse_group = settled.inverse_group;
            }
            Ok(admitted)
        }

        /// ⏪️ The addressed twin of {@link ContractApp::dispatch_typed}. `dispatch_action` admits in two
        /// different two-phase shapes and a law that reads the store right after a bare one observes the
        /// pre-dispatch revision in BOTH: a framework-reserved history verb (`undo`/`redo`/the
        /// checkpoint family) comes back as an `Effect::SpawnJob { kind: FRAMEWORK_RESERVED_JOB_KIND }`
        /// the caller has to run before the store ever sees `ArtifactCommand::Undo`, and an app verb
        /// hands its reducer to a worker. This drives both, exactly as
        /// {@link artifact_app_laws::settle_history_verb} does for the raw wrapper, and rebuilds the
        /// settled mutations/inverse through the same `result_from_last_edit`.
        pub async fn dispatch_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework::InvocationResult, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
            let before_edit_id = self.0.stores.test_last_edit_id();
            let before_tail = self.0.stores.test_edit_tail_lengths();
            let mut admitted = artifact_app_laws::dispatch_fixture_action(&mut self.0, action, args.cloned(), meta, identity, cx, artifact_app_laws::fixture_mounted_policy().maintenance).await?;
            let receipt = artifact_app_laws::settle_registered_typed_operation(&mut self.0, meta.instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), identity,cx).await?;
            admitted.requested_effects.extend(receipt.effects);
            admitted.events.extend(receipt.events);
            if let Some(scope) = receipt.ui_scope {
                admitted.ui_scope = scope;
            }
            let after_edit_id = self.0.stores.test_last_edit_id();
            if after_edit_id.is_some() {
                let tail_offset = if after_edit_id == before_edit_id { before_tail } else { (0, 0) };
                let settled = self.0.stores.test_result_from_last_edit(action, meta, tail_offset).await;
                admitted.mutations = settled.mutations;
                admitted.inverse_group = settled.inverse_group;
            }
            Ok(admitted)
        }
    }

    /// ⏳️ Drives the admission a manifest-addressed dispatch answers with to its publication, the way
    /// {@link ContractApp::dispatch_typed} does for the direct route: an addressed action or command
    /// hands its reducer to a worker too, so a law that reads the store right after one observed the
    /// pre-dispatch revision.
    #[doc(hidden)]
pub async fn settle_contract_app(app: &mut ContractApp,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) {
        artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta().instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), identity,cx).await.expect("registered fixture settles its addressed dispatch");
    }

    #[doc(hidden)]
pub async fn contract_app_under_test(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> ContractApp {
        contract_app_with(migrated_contract_registry().await, mounted_policy, identity).await
    }

    /// 🧪️ [`contract_app_under_test`] over an explicit registry — the flat-menu and interaction
    /// fixtures declare their own manifests but need the same mounted shape with explicit original paid closure.
    #[doc(hidden)]
pub async fn contract_app_with(registry: AppActionRegistry, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> ContractApp {
        let mut app = Box::pin(VcsArtifactApp::with_registry(TestApp::<false>::default(), registry, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await.expect("the reference snapshot births");
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(meta().instance_id).await;
        ContractApp(app, mounted_policy)
    }

    /// 🧪️ The fixture every law that used to build a bare `VcsArtifactApp::<TestApp>::new` wrapper
    /// uses now. A registry-less wrapper can no longer dispatch anything at all (it has no manifest
    /// declaration for the verb), so "no registry" was never what those laws were testing.
    #[doc(hidden)]
pub async fn contract_app(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> ContractApp {
        contract_app_with(migrated_contract_registry().await, mounted_policy, identity).await
    }

    /// 🧪️ [`contract_app`] without the self-closing wrapper — for the laws that MOVE the app into a
    /// `PluginRuntime` cell or a `dyn_enum_close!` arm, or that drive it through `PluginApp`'s
    /// generic surface (a generic bound never applies a `Deref` coercion). Those laws own the close
    /// themselves, exactly as they did before.
    #[doc(hidden)]
pub async fn contract_app_raw(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<TestApp> {
        let mut app = Box::pin(VcsArtifactApp::with_registry(TestApp::<false>::default(), migrated_contract_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await.expect("the reference snapshot births");
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(meta().instance_id).await;
        app
    }

    /// 🧩️ [`contract_app_raw`] over the composed member roster.
    #[doc(hidden)]
pub async fn contract_composed_app_raw(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<TestApp, TestMembers> {
        let mut app = Box::pin(VcsArtifactApp::<TestApp, TestMembers>::with_registry(TestApp::<false>::default(), migrated_contract_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await.expect("the reference snapshot births");
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(meta().instance_id).await;
        app
    }

    /// 🧩️ [`ContractApp`] over the composed member roster — same three laws by construction.
    #[doc(hidden)]
pub struct ContractComposedApp(VcsArtifactApp<TestApp, TestMembers>, semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1);

    impl std::ops::Deref for ContractComposedApp {
        type Target = VcsArtifactApp<TestApp, TestMembers>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for ContractComposedApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for ContractComposedApp {
        fn drop(&mut self) {
            assert!(std::thread::panicking() || self.0.close_terminal_is_empty(), "original contract fixture requires explicit funded closure before Drop");
        }
    }

    impl ContractComposedApp {
        /// 🧩️ A COMPOSED gesture's result is published by the ladder itself, on its own bounded
        /// host outbox, so the host reads the parent's and every child's mutation under its own
        /// handle plus the single `UndoGroup` that names them — nothing is rebuilt here.
        ///
        /// 🔎️ A composed law that reads the WRONG number of documents needs to know whether the
        /// composed lane answered at all or answered short — the two have different causes and
        /// the same symptom.
        ///
        /// 🪢️ The settle-receipt half: a migrated dispatch answers with an admission whose `mutations`
        /// and `inverse_group` are empty by construction, because the document only advances once the
        /// worker's emit has walked the publication ladder above. Rebuilt here through the SAME
        /// `result_from_last_edit` the unmigrated route calls, with the SAME `amended_same_edit` tail
        /// rule, so a coalesced gesture still reports only the operation THIS dispatch added.
        /// 🧩️ `test_result_from_last_edit` rebuilds from the PARENT store's tail edit only and
        /// leaves `member_edits` empty by construction, so it must never overwrite a composed
        /// result the pipeline itself already filled — a composite gesture's child lane would
        /// vanish into the rebuild.
        pub async fn dispatch_typed(&mut self, command: TestCommand, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework::InvocationResult, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
            let verb = test_command_id(&command);
            let before_edit_id = self.0.stores.test_last_edit_id();
            let before_tail = self.0.stores.test_edit_tail_lengths();
            let mut admitted = artifact_app_laws::dispatch_original_fixture_command(&mut self.0, &command, meta, artifact_app_laws::fixture_mounted_policy(), identity, cx).await?;
            let mut receipt = artifact_app_laws::settle_registered_typed_operation(&mut self.0, meta.instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), identity,cx).await?;
            admitted.requested_effects.extend(receipt.effects);
            admitted.events.extend(receipt.events);
            if let Some(scope) = receipt.ui_scope {
                admitted.ui_scope = scope;
            }
            let composed_results = receipt.composed.len();
            if let Some(composed) = receipt.composed.pop() {
                admitted.mutations = composed.mutations;
                admitted.inverse_group = composed.inverse_group;
            }
            admitted.output = semio_framework_value::DslValue::Object(vec![("composedResults".into(), semio_framework_value::DslValue::String(composed_results.to_string()))]);
            let after_edit_id = self.0.stores.test_last_edit_id();
            if after_edit_id.is_some() {
                let tail_offset = if after_edit_id == before_edit_id { before_tail } else { (0, 0) };
                let settled = self.0.stores.test_result_from_last_edit(verb, meta, tail_offset).await;
                if admitted.mutations.is_empty() {
                    admitted.mutations = settled.mutations;
                }
                if admitted.inverse_group.member_edits.is_empty() && admitted.inverse_group.mutations.is_empty() {
                    admitted.inverse_group = settled.inverse_group;
                }
            }
            Ok(admitted)
        }
        /// ⏪️ See {@link ContractApp::dispatch_action}: a framework-reserved history verb admits an
        /// `Effect::SpawnJob` the caller must run, and an app verb hands its reducer to a worker.
        pub async fn dispatch_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework::InvocationResult, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
            let before_edit_id = self.0.stores.test_last_edit_id();
            let before_tail = self.0.stores.test_edit_tail_lengths();
            let mut admitted = artifact_app_laws::dispatch_fixture_action(&mut self.0, action, args.cloned(), meta, identity, cx, artifact_app_laws::fixture_mounted_policy().maintenance).await?;
            let receipt = artifact_app_laws::settle_registered_typed_operation(&mut self.0, meta.instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), identity,cx).await?;
            admitted.requested_effects.extend(receipt.effects);
            admitted.events.extend(receipt.events);
            if let Some(scope) = receipt.ui_scope {
                admitted.ui_scope = scope;
            }
            let after_edit_id = self.0.stores.test_last_edit_id();
            if after_edit_id.is_some() {
                let tail_offset = if after_edit_id == before_edit_id { before_tail } else { (0, 0) };
                let settled = self.0.stores.test_result_from_last_edit(action, meta, tail_offset).await;
                admitted.mutations = settled.mutations;
                admitted.inverse_group = settled.inverse_group;
            }
            Ok(admitted)
        }
    }

    #[doc(hidden)]
pub async fn contract_composed_app(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> ContractComposedApp {
        let mut app = Box::pin(VcsArtifactApp::<TestApp, TestMembers>::with_registry(TestApp::<false>::default(), migrated_contract_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await.expect("the reference snapshot births");
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(meta().instance_id).await;
        ContractComposedApp(app, mounted_policy)
    }
//#endregion 🗝️RegisteredKeyedDispatchFixture
