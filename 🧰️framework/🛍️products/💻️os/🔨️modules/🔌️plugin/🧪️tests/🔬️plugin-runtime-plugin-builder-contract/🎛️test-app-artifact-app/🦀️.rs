/// 🎛️ Test app artifact app: the full `ArtifactApp` implementation of the test app.

    impl<const RETAINED: bool, const TOOLS: u8> ArtifactApp for TestApp<RETAINED, TOOLS> {
        fn owned_mutation_batch_birth_bytes() -> Option<usize> { Some(store::MemberStoreOwnedBatch::scaffold_byte_demand::<TestMutation>()) }
        fn admit_owned_mutation_batch(values: &mut Option<Vec<TestMutation>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { store::MemberStoreOwnedBatch::admit(values, grant) }
        const DIALECT: Dialect = TEST_APP_DIALECT;
        fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
            test_restart_proofs::<RETAINED, TOOLS>()
        }

        fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
            test_restart_register::<RETAINED, TOOLS>(registry)
        }

        async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
            test_restart_build::<RETAINED, TOOLS>(request).await
        }

        fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
            if RETAINED {
                return Some(store::mutation_apply_preparation_factory::<TestSnapshot, TestMutation>());
            }
            (TOOLS != TEST_APP_TOOLS_NONE).then(test_document_preparation_factory)
        }

        fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
            (TOOLS != TEST_APP_TOOLS_NONE).then(|| store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
        }

        fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
            (TOOLS != TEST_APP_TOOLS_NONE).then(semio_framework_plugin_ownership::transient_publication::bounded_transient_preparation_factory::<Self::Presence, Self::PresenceMutation>)
        }

        fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
            (TOOLS != TEST_APP_TOOLS_NONE).then(semio_framework_plugin_ownership::transient_publication::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>)
        }

        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_framework_plugin_ownership::transient_publication::bounded_transient_root_retirement_factory::<Self::Transient>())
        }



        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_root_retirement_factory::<PublicationPresence>())
        }

        /// 📨️ Records every host event on this test thread; a blur answers the lane-less `noopMutation`, so a law sees
        /// that a host event's own dispatch is never a history row.
        fn host_event(event: &HostEvent) -> Option<TestCommand> {
            TEST_HOST_EVENTS.with(|events| events.borrow_mut().push(event.clone()));
            matches!(event, HostEvent::WindowBlurred { .. }).then_some(TestCommand::NoopMutation)
        }

        /// 🐢️ Wave B34: the synthetic app's DECLARED interaction scope, the fixture behind
        /// `interaction_hover_dirties_only_the_hovering_windows_own_lane`. `SetGranularity` is left
        /// deliberately undeclared so the framework's `UiDirtyScope::Full` fallback — the answer every
        /// app that declares nothing still gets — stays under law in the same fixture.
        fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
            if domains != ["items"] || verb == InteractionVerb::SetGranularity {
                return None;
            }
            let panel_bodies = if verb == InteractionVerb::Hover { Vec::new() } else { vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()] };
            Some(UiDirtyScope::Partial {
                window_bodies: vec![TEST_APP_WINDOW_BODY_KEY.to_string()],
                panel_bodies,
                utilities: false,
                tools: false,
                engagements: false,
                measures: verb == InteractionVerb::SetSelectionMode,
                labels: false,
            })
        }
        /// 🪪️ Must equal `test_app_surface_id()` — a hand-typed `&'static str` because the runtime
        /// `ArtifactApp::APP_ID` const (contract §2.1, "kept") cannot call a heap-allocating fn at
        /// compile time; `test_app_id_matches_its_own_dialect` below is the drift guard.
        const APP_ID: &'static str = "s.test.synthetic@1/*#editor";
        const DOCUMENT_SCHEMA: &'static str = "semio.test/v1";
        type Snapshot = TestSnapshot;
        type Mutation = TestMutation;
        type Config = TestConfig;
        type ConfigMutation = TestConfigMutation;
        type Draft = NoDraft;
        type DraftMutation = NoDraftMutation;
        type Presence = PublicationPresence;
        type PresenceMutation = PublicationPresenceMutation;
        type Transient = PublicationTransient;
        type TransientMutation = PublicationTransientMutation;
        type Command = TestCommand;

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_root_retirement_factory::<PublicationPresence>())
        }




        fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
        }

        fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
            Some(bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_framework_plugin_decl_views::presence_retirement::bounded_presence_store_disposer::<PublicationPresence, PublicationPresenceMutation>())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_framework_plugin_ownership::transient_publication::transient_store_disposer::<PublicationTransient, PublicationTransientMutation>(std::sync::Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<PublicationTransient>::default())))
        }

        async fn initial_snapshot() -> TestSnapshot {
            TestSnapshot::default()
        }

        async fn ephemeral(
            command: &TestCommand,
            _doc: &ArtifactView<'_, TestSnapshot>,
            _cfg: &ConfigView<'_, TestConfig>,
            presence: &PresenceView<'_, PublicationPresence>,
            transient: &TransientView<'_, PublicationTransient>,
        ) -> EphemeralEmit<Self> {
            match command {
                TestCommand::Increment => EphemeralEmit {
                    presence: vec![ChangePublicationPresence { revision: presence.local.revision.saturating_add(1) }.into()],
                    transient: vec![ChangePublicationTransient { revision: transient.snapshot.revision.saturating_add(1) }.into()],
                    window_transient: Vec::new(),
                },
                _ => EphemeralEmit::default(),
            }
        }

        async fn command_id(command: &TestCommand) -> &'static str {
            test_command_id(command)
        }

        /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): reachable ONLY through the
        /// `command_from_intent` bridge (this string is never dispatched by any other
        /// existing test) — the `🕹️IntentDispatchTests` fixture proves kind discipline
        /// survives the new intent path exactly like it already does for `dispatch_typed`.
        async fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
            let args = args.as_ref();
            match action {
                "incrementViaCommand" => Ok(TestCommand::IncrementViaCommand),
                "mode.increment" => Ok(TestCommand::ModeIncrement),
                "setLabelViaCommand" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string() }),
                "targetWindow" => Ok(TestCommand::TargetWindow { window_id: args.and_then(|value| value.get("windowId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string() }),
                "probeChild" => Ok(TestCommand::ProbeChild {
                    slot: args.and_then(|value| value.get("slot")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string(),
                    child_id: args.and_then(|value| value.get("childId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string(),
                }),
                "badView" => Ok(TestCommand::BadView),
                _ => Err(Fault::from(format!("unknown test command: {action}"))),
            }
        }

        async fn handle(
            command: &TestCommand,
            doc: &ArtifactView<'_, TestSnapshot>,
            _cfg: &ConfigView<'_, TestConfig>,
            _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>,
            _draft: &DraftView<'_, NoDraft>,
            _engines: &semio_framework_2d_compute::EngineHandles,
        ) -> Result<Emit<TestMutation, TestConfigMutation>, Fault> {
            test_app_reduce(command, doc, _cfg)
        }

        async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            RENDER_CONTEXT_PROBE.with(|probe| probe.replace(Some((body_key.into(), view_state.clone()))));
            if matches!(body_key, "graph" | "properties") {
                let item = TreeNode::try_new("item-1", Component::TreeItem(TreeItemProps {
                    label: Label(UiText::try_from_str("Item 1").expect("bounded fixture")), description: None, icon: None, default_open: None,
                    draggable: None, drag_data: None, dimmed: None, selected: None, window: None, granularity: None, inline_toolbar: None, detail: None, row_actions: UiFixedList::default(), target: None,
                })).expect("bounded fixture");
                let root = TreeNode::try_new("root", Component::Tree(TreeProps { presentation: Default::default(), interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }))
                    .expect("bounded fixture").try_with_children([item]).unwrap_or_else(|_| panic!("bounded fixture"));
                return Ok(ComponentTree::new(root));
            }
            built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}", doc.snapshot.count)))
        }

        /// 🪟️ One group per live window instance — the reserved `measures` section surface's own payload.
        async fn window_measures(_doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, view_state: &ViewModel) -> std::collections::HashMap<String, Vec<semio_framework_ui_layout::layout::WindowMeasure>> {
            view_state.window_instances.iter().map(|window| (window.id.clone(), vec![semio_framework_ui_layout::layout::WindowMeasure::measure_group(format!("{}-measure", window.id), format!("{} measures", window.window_kind_id), Vec::new())])).collect()
        }

        /// 🛠️ One group for the active tool — the reserved `tools` section surface's own payload.
        async fn tool_measures(_doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, view_state: &ViewModel) -> std::collections::HashMap<String, Vec<semio_framework_ui_layout::layout::WindowMeasure>> {
            view_state.active_tool_id.iter().map(|tool| (tool.clone(), vec![semio_framework_ui_layout::layout::WindowMeasure::measure_group(format!("{tool}-measure"), format!("{tool} tool"), Vec::new())])).collect()
        }

        fn build_reserved_tool_job(request: semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
            if !matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") {
                return Ok(None);
            }
            Ok(Some(ArtifactReservedToolJob::new(TestClipboardJob::<RETAINED, TOOLS>::new(request)?)))
        }

        async fn clipboard_media_type() -> Option<MediaType> {
            Some(MediaType { class: MediaClass::Data, form: MediaForm::Value })
        }

        async fn copy_fragment(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
            if doc.snapshot.label.is_empty() {
                return Err(ClipboardError::EmptySelection);
            }
            Ok(ClipboardFragment {
                schema: Self::DOCUMENT_SCHEMA.to_string(),
                media_type: Self::clipboard_media_type().await.expect("declared above"),
                dsl_text: doc.snapshot.label.clone(),
                pack_bytes: None,
                source_app: Self::APP_ID.to_string(),
                label: doc.snapshot.label.clone(),
            })
        }

        async fn cut_operations(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _interaction: &InteractionView<'_>) -> Vec<TestMutation> {
            if doc.snapshot.label.is_empty() { Vec::new() } else { vec![TestMutation::SetLabel(SetLabel { value: String::new() })] }
        }

        async fn paste_operations(_doc: &ArtifactView<'_, TestSnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<TestMutation>, ClipboardError> {
            if !Self::clipboard_accepts().await.contains(&fragment.media_type) {
                return Err(ClipboardError::IncompatibleMediaType(fragment.media_type));
            }
            let value = match placement.anchor {
                PasteAnchor::Original => fragment.dsl_text.clone(),
                _ => format!("{}-{:?}", fragment.dsl_text, placement.anchor),
            };
            Ok(vec![TestMutation::SetLabel(SetLabel { value })])
        }

        /// 🧪️ Menu = always "setLabelRequired"; "incrementViaCommand" gated on a non-empty label
        /// (a selection-guard stand-in) — exercises `Menu::action`/`Menu::command`/`Menu::when`. The
        /// `flatLeaf1..10` branch only fires for the magic `"flat-menu-test"` label (so
        /// `contract_registry`-backed tests, which never set that label, are untouched) — a flat >9-row
        /// menu fixture for `context_menu_funnel_organizes_a_synthetic_apps_flat_overflow_menu` below,
        /// proving `VcsArtifactApp::context_menu` runs every emitter through `organize_context_menu`.
        ///
        /// 🩹️ Rewritten from `Menu::when(cond, |m| m.command(..))` — `when`'s closure param is
        /// `impl FnOnce(Self) -> Self` (sync, a public documented pattern other plugins use), but
        /// `Menu::command`/`action` are genuinely async (they await `AppActionRegistry::get*`). An
        /// async closure can't satisfy a sync `FnOnce`, so this fixture inlines the two guarded
        /// branches as explicit `if`s instead of touching `when`'s public signature. See R10 residue
        /// class 1 (`` inside a sync closure).
        async fn context_menu(_request: &ContextMenuRequest, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
            let mut menu = Menu::of(registry, view_state).action("setLabelRequired");
            if !doc.snapshot.label.is_empty() && doc.snapshot.label != "flat-menu-test" {
                menu = menu.command("incrementViaCommand");
            }
            if doc.snapshot.label == "flat-menu-test" {
                for index in 1..=10 {
                    menu = menu.action(format!("flatLeaf{index}"));
                }
            }
            menu.build()
        }

        /// 🧪️ `🕹️InteractionDispatch` fixture: the "items" domain (declared only by `interaction_registry`
        /// below — every OTHER test's registry declares no interactions, so this is never called for
        /// them) is `HierarchyProvider::Topology`-backed by a single synthetic id, "item-1", present
        /// exactly when `doc.snapshot.label` is non-empty — lets a test simulate "delete the selected
        /// node" by setting the label back to empty and observing `validate_state` prune it.
        async fn interaction_topology(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
            let mut domains = BTreeMap::new();
            let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![TopologyNode { id: "item-1".into(), granularity: "item".into(), parent: None }] };
            domains.insert("items".to_string(), DomainTopology { ordered });
            Ok(InteractionTopology { domains })
        }
    }
