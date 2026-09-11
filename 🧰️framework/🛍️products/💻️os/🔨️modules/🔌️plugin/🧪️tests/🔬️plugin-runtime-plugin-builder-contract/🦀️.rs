mod plugin_builder_contract_tests {
    //! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
    //! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
    //! operations that emit no document operations, history interception, and remote-operation ingest
    //! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
    //! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
    //! `handle_command` — everything app-specific dispatches via `dispatch_typed`.
    use dsl::DslValue;
    use semio_framework_value_derive::{FromValue, ToValue};
    use ui_wgpu::wgpu::LocalizedLabel;

    /// 🌉️ Test-only convenience: builds a `serde_json::json!` literal, then bridges it to the
    /// `DslValue` `handle_action`/`dispatch_action`/`command_from_action` speak at the trait
    /// boundary — the fixtures below stay readable as JSON literals without keeping a
    /// `serde_json::Value` alive past this one conversion.
    fn dv(value: Value) -> DslValue {
        DslValue::from(&value)
    }

    use super::ContextMenuWireRequest;
    use crate::app::{
        ActionMeta, App, AppActionRegistry, ArtifactApp, ArtifactView, AsyncTask, ChildEmit, CommandView, ConfigView, DraftView, Emit, EphemeralSnapshot, HistoryCommandFilter, HistoryView, InteractionHoverState, InteractionView, Menu, NoDraft,
        NoDraftMutation, NoPresence, NoPresenceMutation, PeerPresence, PluginApp, TaskCtx, TaskResolution, VcsArtifactApp, ui_history_panel,
    };
    use crate::app::{ArtifactDeserializer, ArtifactSerializer, Dialect, ErasedComposeSource, IoPayload, StandardId, SubsetId, deserializer_entry_of, resolve_ready, serializer_entry_of};
    use crate::publication_fixture::{ChangePublicationPresence, ChangePublicationTransient, PublicationPresence, PublicationPresenceMutation, PublicationTransient, PublicationTransientMutation};
    use crate::store::FaultFrom;
    use crate::{IconName, MediaClass, MediaType, ViewModel, selection_count_phrase};
    use protocol::Mutation;
    use semio_framework::Fault;
    use semio_framework::kernel::ArtifactHandle;
    use semio_framework::kernel::{AppEvent, ClipboardError, ClipboardFragment, Effect, PasteAnchor, PastePlacement, UiDirtyScope};
    use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, CommandDefinition, MediaForm, NOTE_SHELL_COMMAND_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
    use semio_framework_job::InteractiveJob as _;
    mod local_interaction_dispatch {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs"));
    }
    /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): this module names every other type
    /// explicitly (no `use super::*;`), so the `🕹️IntentDispatchTests` fixture needs its own
    /// import too, rather than relying on `mod app`'s outer glob.
    use semio_framework_ui_contract::{ActionId, BuiltNode, SurfaceId, Trigger, UiIntent, UiNodeId, UiRevision, UI_BUILT_CHILDREN_MAX};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::collections::BTreeMap;
    use store::os_io::ArtifactRef;
    use store::{ArtifactPack, EngineHandles};
    use store::{Backbone, BackboneMessage, MemoryBackbone};
    use store::{MemberFactory, SpaceMember};
    use ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY;
    use ui_wgpu::wgpu::{ContextMenuItemSpec, ContextMenuRequest, UiMenuRef};

    /// 🪪️ `TestApp`'s one dialect coordinate (contract §1) — every canonical id this module needs
    /// (`TestApp::APP_ID`, the `App::builder` ids below, every `CommandAddress`/`ActionAddress`/
    /// `ClipboardFragment.source_app` literal) derives from this ONE fixture instead of the
    /// pre-migration `"synthetic-play"` string, so `test_app_surface_id` and `TestApp::APP_ID` can
    /// never independently drift (guarded by `test_app_id_matches_its_own_dialect` below).
    const TEST_APP_DIALECT: Dialect = Dialect { artifact_kind: "s.test.synthetic", standard: StandardId("1"), subset: SubsetId::ANY };

    std::thread_local! {
        static RENDER_CONTEXT_PROBE: std::cell::RefCell<Option<(String, ViewModel)>> = const { std::cell::RefCell::new(None) };
    }

    #[test]
    fn app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧵️retained-command/🧫️fixtures/🧬️request-context.json")).expect("request context fixture");
        let hex = |text: &str| text.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex pair"), 16).expect("hex byte")).collect::<Vec<_>>();
        let revision: [u8; 32] = hex(fixture["canonicalBaseRevisionHex"].as_str().expect("revision hex")).try_into().expect("revision width");
        let expected = u64::from_str_radix(fixture["expectedIdentityDigestHex"].as_str().expect("digest hex"), 16).expect("digest");
        let app_instance_id = fixture["appInstanceId"].as_u64().expect("app instance") as u32;
        let draft_generation = fixture["draftGeneration"].as_u64().expect("draft generation");
        let transient_generation = fixture["transientGeneration"].as_u64().expect("transient generation");
        let children_digest = u64::from_str_radix(fixture["childrenDigestHex"].as_str().expect("children digest hex"), 16).expect("children digest");
        let identity = test_artifact_owned_tool_job_context_identity_digest;
        assert_eq!(identity(app_instance_id, revision, draft_generation, transient_generation, children_digest), expected);
        assert_eq!(fixture["mismatchCases"].as_array().expect("mismatch cases").len(), 5);
        assert_ne!(identity(app_instance_id + 1, revision, draft_generation, transient_generation, children_digest), expected);
        let mut changed_revision = revision;
        changed_revision[31] ^= 1;
        assert_ne!(identity(app_instance_id, changed_revision, draft_generation, transient_generation, children_digest), expected);
        assert_ne!(identity(app_instance_id, revision, draft_generation + 1, transient_generation, children_digest), expected);
        assert_ne!(identity(app_instance_id, revision, draft_generation, transient_generation + 1, children_digest), expected);
        assert_ne!(identity(app_instance_id, revision, draft_generation, transient_generation, children_digest ^ 1), expected);
        assert_eq!(fixture["restartBoundary"]["workerThreadLoss"], "exact-resume-with-retained-context");
        assert_eq!(fixture["restartBoundary"]["processRestartWithoutExactTransientSnapshot"], "fail-closed");
    }

    /// 🪪️ Live `surface_app_id` call over `TEST_APP_DIALECT` — the id every `App::builder(...)` call
    /// in this module passes, per contract §1 (`AppBuilder::build_definition` rejects a hand-written id).
    async fn test_app_surface_id() -> String {
        surface_app_id(&TEST_APP_DIALECT.into(), AppRole::Editor)
    }

    //#region 🧬️TestDocumentMutationFixture
    use crate::test_app_mutation_fixture::TestSnapshot;
    use crate::test_app_mutation_fixture::document::{MAXIMUM_CHILD_CLONES, MAXIMUM_CHILD_ENCODINGS, MAXIMUM_CHILD_PROBE_BYTES};

    /// 🧪️ Trivial dummy `ArtifactSerializer`/`ArtifactDeserializer` pair, round-tripping
    /// `TestSnapshot` to itself, for `serializer_entry_of`/`deserializer_entry_of` smoke tests.
    struct DummySerializer;
    impl ArtifactSerializer for DummySerializer {
        type From = TestSnapshot;
        type Into = TestSnapshot;
        const FROM: Dialect = Dialect { artifact_kind: "s.test.dummy", standard: StandardId("1"), subset: SubsetId("*") };
        const INTO: Dialect = Dialect { artifact_kind: "s.test.dummy.out", standard: StandardId("1"), subset: SubsetId("*") };
        async fn serialize(from: &TestSnapshot) -> Result<TestSnapshot, store::PackError> {
            Ok(from.clone())
        }
    }

    struct DummyDeserializer;
    impl ArtifactDeserializer for DummyDeserializer {
        type From = TestSnapshot;
        type Into = TestSnapshot;
        const FROM: Dialect = Dialect { artifact_kind: "s.test.dummy.out", standard: StandardId("1"), subset: SubsetId("*") };
        const INTO: Dialect = Dialect { artifact_kind: "s.test.dummy", standard: StandardId("1"), subset: SubsetId("*") };
        async fn deserialize(from: &TestSnapshot) -> Result<TestSnapshot, store::PackError> {
            Ok(from.clone())
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn serializer_entry_of_and_deserializer_entry_of_erase_correctly() {
        let ser = serializer_entry_of::<DummySerializer>();
        assert_eq!(ser.writes, DummySerializer::INTO);
        assert_eq!(ser.reads.to_vec(), vec![DummySerializer::FROM]);
        let de = deserializer_entry_of::<DummyDeserializer>();
        assert_eq!(de.writes, DummyDeserializer::INTO);
        assert_eq!(de.reads.to_vec(), vec![DummyDeserializer::FROM]);

        let seed = TestSnapshot { count: 7, label: "x".into() };
        let bytes = ArtifactPack::encode_pack(&seed);
        let composed = resolve_ready((ser.compose)(&[ErasedComposeSource { dialect: DummySerializer::FROM, payload: IoPayload::Binary(bytes) }])).expect("serializer_entry_of erased compose should succeed with exactly 1 source");
        assert_eq!(composed.dialect, DummySerializer::INTO);
        match composed.payload {
            IoPayload::Binary(out) => assert_eq!(<TestSnapshot as ArtifactPack>::decode_pack(&out).unwrap(), seed),
            IoPayload::Text(_) => panic!("expected Binary payload"),
        }

        let zero_sources_err = match resolve_ready((de.compose)(&[])) {
            Err(err) => err,
            Ok(_) => panic!("deserializer_entry_of erased compose should reject 0 sources"),
        };
        assert!(zero_sources_err.message.contains("needs exactly 1 source"), "{}", zero_sources_err.message);
        let two_sources = [ErasedComposeSource { dialect: DummyDeserializer::FROM, payload: IoPayload::Binary(Vec::new()) }, ErasedComposeSource { dialect: DummyDeserializer::FROM, payload: IoPayload::Binary(Vec::new()) }];
        let two_sources_err = match resolve_ready((de.compose)(&two_sources)) {
            Err(err) => err,
            Ok(_) => panic!("deserializer_entry_of erased compose should reject 2 sources"),
        };
        assert!(two_sources_err.message.contains("needs exactly 1 source"), "{}", two_sources_err.message);
    }

    //#region 🧬️TestDocumentMutationLeaves
    use crate::test_app_mutation_fixture::{SetCount, SetLabel, TestMutation};
    //#endregion 🧬️TestDocumentMutationLeaves

    struct TestCountOneItemPreparationFactory;

    struct TestCountOneItemPreparation {
        request: Option<store::ArtifactStoreOneItemPreparationRequest<TestSnapshot, TestMutation>>,
        prepared: Option<store::ArtifactStoreOneItemPrepared<TestSnapshot, TestMutation>>,
        authority_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        turn: u8,
        closing: bool,
    }

    impl store::ArtifactStoreOneItemPreparationFactory<TestSnapshot, TestMutation> for TestCountOneItemPreparationFactory {
        fn preflight(&self, mutation: &TestMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
            if !matches!(mutation, TestMutation::SetCount(SetCount { .. })) || description.is_some() || lane != store::HistoryLane::Document {
                return Err("test count accepts exactly one scalar mutation".into());
            }
            Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 1_024 })
        }

        fn begin(
            &self,
            request: store::ArtifactStoreOneItemPreparationRequest<TestSnapshot, TestMutation>,
        ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<TestSnapshot, TestMutation>>, store::ArtifactStoreOneItemPreparationRequest<TestSnapshot, TestMutation>> {
            if !request.base.get().label.is_empty() || request.authority.actor().len() > 32 || request.authority.group_id().is_some() {
                return Err(request);
            }
            Ok(Box::new(TestCountOneItemPreparation { request: Some(request), prepared: None, authority_retirement: None, turn: 0, closing: false }))
        }
    }

    impl store::ArtifactStoreOneItemPreparation<TestSnapshot, TestMutation> for TestCountOneItemPreparation {
        fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
            if !grant.permits_one() || grant.maximum_bytes < 1_024 || self.closing {
                return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
            }
            if self.turn == 0 {
                self.turn = 1;
                return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint()));
            }
            if self.prepared.is_none() {
                let request = self.request.as_ref().ok_or_else(|| "test count lost its exact request".to_string())?;
                let TestMutation::SetCount(SetCount { value }) = &request.mutation else {
                    return Err("test count requires scalar mutation".into());
                };
                let authority = &request.authority;
                let edit = store::Edit {
                    id: format!("fixture-count-{}", authority.next_sequence_number()),
                    actor: Some(authority.actor().into()),
                    forwards: vec![TestMutation::SetCount(SetCount { value: *value })],
                    inverse: vec![TestMutation::SetCount(SetCount { value: request.base.get().count })],
                    mutation_meta: vec![protocol::MutationMeta {
                        mutation_id: None,
                        dependencies: Vec::new(),
                        base_version: 0,
                        author_id: Some(ActorId(authority.actor().into())),
                        timestamp: authority.next_clock(),
                        undo_policy: UndoPolicy::ExactBaseOnly,
                        payload_hash: None,
                        semantic_kind: None,
                        label: None,
                        group_id: None,
                        origin: Default::default(),
                    }],
                    description: None,
                    coalesce_key: None,
                    sequence_number: authority.next_sequence_number(),
                    started_at: String::new(),
                    finished_at: None,
                };
                self.prepared = Some(authority.prepare_one_item(edit, std::sync::Arc::new(TestSnapshot { count: *value, label: String::new() }))?);
                self.turn = 2;
            }
            Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint()))
        }

        fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
            store::ArtifactStoreOneItemCheckpoint {
                cursor: u32::from(self.turn),
                completed_items: u32::from(self.turn),
                completed_bytes: u64::from(self.turn),
                digest: self.prepared.as_ref().map_or([0; 32], store::ArtifactStoreOneItemPrepared::edit_digest),
            }
        }

        fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<TestSnapshot, TestMutation>> {
            self.prepared.as_ref()
        }
        fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<TestSnapshot, TestMutation>> {
            self.prepared.take()
        }
        fn cancel(&mut self) {
            self.closing = true;
        }
        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
            if !self.closing || !grant.permits_one() || grant.maximum_bytes < 1_024 {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            if self.prepared.take().is_some() {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 1_024 });
            }
            if let Some(request) = self.request.take() {
                assert!(request.base.return_to_registry());
                self.authority_retirement = Some(request.authority.retire());
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(owner) = self.authority_retirement.as_mut() {
                let step = owner.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
                if step == store::SnapshotRetirementStep::Complete {
                    assert!(owner.terminal_is_empty());
                    self.authority_retirement = None;
                }
                return Ok(step);
            }
            Ok(store::SnapshotRetirementStep::Complete)
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.prepared.is_none() && self.request.is_none() && self.authority_retirement.is_none()
        }
    }
    //#endregion 🧬️TestDocumentMutationFixture

    use crate::test_app_mutation_fixture::{ChangeTestConfigSelection, TestConfig, TestConfigMutation};

    /// 🧪️ B1: `TestApp`'s typed command enum — the sole dispatch surface for its own behavior.
    #[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, dsl::DslOps)]
    enum TestCommand {
        #[dsl(key = "increment")]
        Increment,
        #[dsl(key = "set-label")]
        SetLabel { value: String },
        #[dsl(key = "amend-label")]
        AmendLabel { value: String },
        #[dsl(key = "commit-label")]
        CommitLabel { value: String },
        #[dsl(key = "bad-view")]
        BadView,
        #[dsl(key = "select")]
        Select { id: Option<String> },
        #[dsl(key = "navigate")]
        Navigate,
        #[dsl(key = "noop-operation")]
        NoopMutation,
        #[dsl(key = "view-no-scope")]
        ViewNoScope,
        #[dsl(key = "view-partial-scope")]
        ViewPartialScope,
        #[dsl(key = "increment-via-command")]
        IncrementViaCommand,
        #[dsl(key = "watchdog-overrun")]
        WatchdogOverrun,
        #[dsl(key = "set-label-via-command")]
        SetLabelViaCommand { value: String },
        #[dsl(key = "set-active-utility")]
        SetActiveUtility { utility_id: String },
        /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): exercises `Emit.child_emits` — emits a
        /// parent-document op alongside a `ChildEmit` targeting whichever `(slot, child_id)` the
        /// test registered via `VcsArtifactApp::register_child` beforehand.
        #[dsl(key = "composite-edit")]
        CompositeEdit { slot: String, child_id: String, child_value: i32 },
        #[dsl(key = "probe-child")]
        ProbeChild { slot: String, child_id: String },
        /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: spawns an `AsyncTask` that awaits a real
        /// `host::storage_read` round trip (parked on `RequestRegistry`, resolved by an
        /// injected completion in the test — see `⚛️reactor`'s `test_support`), then resolves
        /// with `TaskResolution::Command(ApplyCountFromTask)` — Elm's Msg-from-Cmd, exercised
        /// end to end without a wasm32-wasip2 build.
        #[dsl(key = "spawn-count-task")]
        SpawnCountTask,
        /// 🧵️ The follow-up command a `SpawnCountTask` resume redispatches — also directly
        /// dispatchable on its own, so a test can assert the SAME command applied via a normal
        /// dispatch and via a task resume produce byte-identical mutations.
        #[dsl(key = "apply-count-from-task")]
        ApplyCountFromTask { value: i32 },
    }

    impl ::protocol::OpText for TestCommand {
        fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
            let variants = <Self as ::dsl::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{keyword} ");
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                    let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;
                    return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
            let variants = <Self as ::dsl::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
            if body.is_empty() { keyword } else { format!("{keyword} {body}") }
        }
    }

    impl ::protocol::OpBinary for TestCommand {
        const TOOL_JOB_IDS: &'static [&'static str] = &["compositeEdit", "applyCountFromTask"];

        fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
            ::dsl::variants_binary::encode_op(self)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
            ::dsl::variants_binary::decode_op(bytes)
        }
    }

    /// 🧪️ App under test. `received_actions` records every command id THIS app's own `handle` was
    /// actually called with — used to prove framework-owned interceptions (e.g. `noteShellCommand`)
    /// never reach it.
    #[derive(Default)]
    struct TestApp<const RETAINED: bool = false> {
        received_actions: std::cell::RefCell<Vec<String>>,
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/⏳️completion/🦀️.rs"));

    //#region 🧵️RetainedCommandReplayFixture
    const TEST_RETAINED_COMMAND_CONTROLLER: &str = "s.test.synthetic@1/*#editor";
    const TEST_RETAINED_COMMAND_TOOL: &str = "setLabel";
    const TEST_RETAINED_COMMAND_SCHEMA: &str = "semio.test.retained-command.v1";
    const TEST_RETAINED_COMMAND_RAW_BYTES: usize = 4_096;
    static TEST_RETAINED_COMMAND_STEP_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    struct TestRetainedCommandWork {
        cursor: u8,
    }

    struct TestRetainedChildCommandWork {
        emit: Option<Emit<TestMutation, TestConfigMutation, NoDraftMutation>>,
    }

    impl crate::retained_command::ArtifactCommandWork<TestApp> for TestRetainedChildCommandWork {
        fn tool_id(&self) -> &'static str {
            TEST_RETAINED_COMMAND_TOOL
        }

        fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &InteractionState, _context: Option<&ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
            Some(1)
        }

        fn step(&mut self, input: &crate::retained_command::ArtifactCommandInputs<'_, TestApp>) -> Result<crate::retained_command::ArtifactCommandWorkStep<TestApp>, Fault> {
            let crate::retained_command::ArtifactCommandInputs { command: _command, snapshot: _snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
            self.emit.take().map(crate::retained_command::ArtifactCommandWorkStep::Complete).ok_or_else(|| Fault::from("test-retained-child-work-repeated"))
        }
    }

    impl crate::retained_command::ArtifactCommandWork<TestApp> for TestRetainedCommandWork {
        fn tool_id(&self) -> &'static str {
            TEST_RETAINED_COMMAND_TOOL
        }

        fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &InteractionState, _context: Option<&ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
            Some(3)
        }

        fn step(&mut self, input: &crate::retained_command::ArtifactCommandInputs<'_, TestApp>) -> Result<crate::retained_command::ArtifactCommandWorkStep<TestApp>, Fault> {
            let crate::retained_command::ArtifactCommandInputs { command: _command, snapshot: _snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
            TEST_RETAINED_COMMAND_STEP_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if self.cursor < 2 {
                self.cursor += 1;
                return Ok(crate::retained_command::ArtifactCommandWorkStep::Progress { stage: "test-retained-command-work", preview: br#"{"en":"Applying","de":"Anwenden"}"# });
            }
            Ok(crate::retained_command::ArtifactCommandWorkStep::Complete(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value: "resumed".into() })])))
        }

        fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
            let Some(first) = target.first_mut() else { return Err(Fault::from("test-retained-command-checkpoint-capacity")) };
            *first = self.cursor;
            Ok(1)
        }

        fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
            let [cursor] = checkpoint else { return Err(Fault::from("test-retained-command-checkpoint-invalid")) };
            if *cursor > 2 {
                return Err(Fault::from("test-retained-command-checkpoint-cursor"));
            }
            self.cursor = *cursor;
            Ok(())
        }
    }

    struct TestRetainedCommandFactory {
        keys: Vec<ToolFactoryKey>,
    }

    impl TestRetainedCommandFactory {
        fn new() -> Self {
            Self { keys: vec![ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL)] }
        }
    }

    struct OtherTestRetainedCommandFactory(TestRetainedCommandFactory);

    impl ToolJobFactory for OtherTestRetainedCommandFactory {
        type Payload = crate::retained_command::ArtifactRetainedCommandPayload<TestApp>;
        type Job = crate::retained_command::ArtifactRetainedCommandJob<TestApp>;
        fn keys(&self) -> &[ToolFactoryKey] {
            &self.0.keys
        }
        fn payload_schema_id(&self) -> &str {
            TEST_RETAINED_COMMAND_SCHEMA
        }
        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }
        fn execution_contract(&self) -> ToolExecutionContract {
            ToolJobFactory::execution_contract(&self.0)
        }
        fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            ToolJobFactory::create_job(&mut self.0, operation, payload)
        }
    }

    impl ArtifactOwnedToolJobFactory for OtherTestRetainedCommandFactory {
        type Owner = TestApp;
        const TOOL_IDS: &'static [&'static str] = &[TEST_RETAINED_COMMAND_TOOL];
        const DOCUMENT_SCHEMA: &'static str = TestApp::<false>::DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = TestRetainedCommandFactory::PUBLICATION_CONTRACTS;
    }

    impl ArtifactOwnedToolJobFactory for TestRetainedCommandFactory {
        type Owner = TestApp;
        const TOOL_IDS: &'static [&'static str] = &[TEST_RETAINED_COMMAND_TOOL];
        const DOCUMENT_SCHEMA: &'static str = TestApp::<false>::DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: TEST_RETAINED_COMMAND_TOOL, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
    }

    impl ToolJobFactory for TestRetainedCommandFactory {
        type Payload = crate::retained_command::ArtifactRetainedCommandPayload<TestApp>;
        type Job = crate::retained_command::ArtifactRetainedCommandJob<TestApp>;

        fn keys(&self) -> &[ToolFactoryKey] {
            &self.keys
        }

        fn payload_schema_id(&self) -> &str {
            TEST_RETAINED_COMMAND_SCHEMA
        }

        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }

        fn execution_contract(&self) -> ToolExecutionContract {
            ToolExecutionContract::resumable(TEST_RETAINED_COMMAND_RAW_BYTES, 4, 1, TEST_RETAINED_COMMAND_RAW_BYTES, 7_500, 1, 1)
        }

        fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            Ok(crate::retained_command::ArtifactRetainedCommandJob::new(payload))
        }

        fn create_job_from_wire_pages_with_payload(
            &mut self,
            _operation: semio_framework_job::Operation,
            payload: Self::Payload,
            input: action_bus::RetainedToolWireInput,
            checkpoint: Option<action_bus::RetainedToolWireInput>,
        ) -> Result<Self::Job, (ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
            if input.declared_bytes() > TEST_RETAINED_COMMAND_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > crate::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
                return Err((ToolJobFactoryError::new("test retained command extent"), input, checkpoint));
            }
            Ok(match checkpoint {
                Some(checkpoint) => crate::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
                None => crate::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
            })
        }
    }

    fn test_retained_command_id(command: &TestCommand) -> &'static str {
        match command {
            TestCommand::SetLabel { .. } => TEST_RETAINED_COMMAND_TOOL,
            _ => "unsupported",
        }
    }

    async fn test_retained_command_payload(completion: ArtifactToolCompletion<TestApp>) -> crate::retained_command::ArtifactRetainedCommandPayload<TestApp> {
        crate::retained_command::ArtifactRetainedCommandPayload::try_new(
            crate::retained_command::ArtifactRetainedCommandInputs {
                command: TestCommand::SetLabel { value: "wire".into() },
                snapshot: std::sync::Arc::new(TestSnapshot::default()),
                config: std::sync::Arc::new(TestConfig::default()),
                history: std::sync::Arc::new(HistoryView::empty()),
                interaction_state: std::sync::Arc::new(InteractionState::default()),
                interaction_hover: std::sync::Arc::new(InteractionHoverState::new()),
                context: None,
                operation: AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 41, generation: 3, canonical_base_revision: [5; 32] },
                completion,
            },
            test_retained_command_id,
            TEST_RETAINED_COMMAND_RAW_BYTES,
            3,
            Box::new(TestRetainedCommandWork { cursor: 0 }),
        )
        .expect("test retained command payload")
    }

    async fn test_retained_child_command_payload(completion: ArtifactToolCompletion<TestApp>, emit: Emit<TestMutation, TestConfigMutation, NoDraftMutation>) -> crate::retained_command::ArtifactRetainedCommandPayload<TestApp> {
        crate::retained_command::ArtifactRetainedCommandPayload::try_new(
            crate::retained_command::ArtifactRetainedCommandInputs {
                command: TestCommand::SetLabel { value: "wire".into() },
                snapshot: std::sync::Arc::new(TestSnapshot::default()),
                config: std::sync::Arc::new(TestConfig::default()),
                history: std::sync::Arc::new(HistoryView::empty()),
                interaction_state: std::sync::Arc::new(InteractionState::default()),
                interaction_hover: std::sync::Arc::new(InteractionHoverState::new()),
                context: None,
                operation: AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 42, generation: 3, canonical_base_revision: [5; 32] },
                completion,
            },
            test_retained_command_id,
            TEST_RETAINED_COMMAND_RAW_BYTES,
            1,
            Box::new(TestRetainedChildCommandWork { emit: Some(emit) }),
        )
        .expect("test retained child command payload")
    }

    fn test_retained_wire_input(bus: &ActionBus, bytes: &[u8]) -> (ToolWireAdmission, action_bus::RetainedToolWireInput) {
        let (admission, mut input) = bus.begin_exact_wire(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, bytes.len()).expect("test retained command admission");
        for page in bytes.chunks(action_bus::TOOL_WIRE_PAGE_BYTES) {
            input.admit_page(action_bus::ToolWirePage::try_copy_from(page).expect("test retained wire page")).map_err(|(fault, _)| fault).expect("test retained page admission");
        }
        input.seal().expect("test retained wire seal");
        (admission, input)
    }

    fn test_close_retained_payload(payload: &mut semio_framework_job::RetainedJobPayload) {
        while !payload.terminal_is_empty() {
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
    }
    //#endregion 🧵️RetainedCommandReplayFixture

    //#region 🧪️TestClipboardReservedJob
    struct TestClipboardReservedJob<const RETAINED: bool> {
        tool_id: String,
        snapshot: std::sync::Arc<TestSnapshot>,
        raw_wire: Vec<u8>,
        input: Option<ArtifactReservedToolInput>,
        completion: Option<ArtifactToolCompletion<TestApp<RETAINED>>>,
        closing: bool,
    }

    impl<const RETAINED: bool> TestClipboardReservedJob<RETAINED> {
        fn new(request: ArtifactReservedToolJobRequest<TestApp<RETAINED>>) -> Self {
            Self { tool_id: request.tool_id, snapshot: request.snapshot, raw_wire: request.raw_wire, input: Some(request.input), completion: Some(request.completion), closing: false }
        }

        fn emit(&mut self) -> Emit<TestMutation, TestConfigMutation, NoDraftMutation> {
            let ArtifactReservedToolInput::Action { args, .. } = self.input.take().expect("test clipboard input") else { panic!("clipboard route received media input") };
            match self.tool_id.as_str() {
                "copy" | "cut" if self.snapshot.label.is_empty() => Emit::default(),
                "copy" | "cut" => {
                    let fragment = ClipboardFragment {
                        schema: TestApp::<RETAINED>::DOCUMENT_SCHEMA.to_string(),
                        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                        dsl_text: self.snapshot.label.clone(),
                        pack_bytes: None,
                        source_app: TestApp::<RETAINED>::APP_ID.to_string(),
                        label: self.snapshot.label.clone(),
                    };
                    Emit { artifact_mutations: (self.tool_id == "cut").then(|| TestMutation::SetLabel(SetLabel { value: String::new() })).into_iter().collect(), effects: vec![Effect::ClipboardWrite { fragment }], ..Default::default() }
                }
                "paste" => {
                    let Some(args) = args else { return Emit::default() };
                    let Some(fragment) = args.get("fragment").and_then(|value| serde_json::from_value::<ClipboardFragment>(value.clone().into()).ok()) else { return Emit::default() };
                    if fragment.media_type != (MediaType { class: MediaClass::Data, form: MediaForm::Value }) {
                        return Emit::default();
                    }
                    let placement = serde_json::from_value::<PastePlacement>(args.into()).unwrap_or_default();
                    let value = match placement.anchor {
                        PasteAnchor::Original => fragment.dsl_text,
                        anchor => format!("{}-{anchor:?}", fragment.dsl_text),
                    };
                    Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value })])
                }
                _ => Emit::default(),
            }
        }
    }

    impl<const RETAINED: bool> semio_framework_job::InteractiveJob for TestClipboardReservedJob<RETAINED> {
        fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
            if cx.is_cancelled() {
                return semio_framework_job::StepOutcome::Cancelled;
            }
            let emit = self.emit();
            self.completion.as_ref().expect("test clipboard completion").complete(Ok(emit), EphemeralEmit::default()).expect("single test clipboard completion");
            semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            })
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
            self.closing = true;
            if !self.raw_wire.is_empty() {
                if maximum_items == 0 || maximum_bytes == 0 {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                let released_bytes = self.raw_wire.len().min(maximum_bytes);
                self.raw_wire.truncate(self.raw_wire.len() - released_bytes);
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            if self.input.take().is_some() || self.completion.take().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            semio_framework_job::InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.raw_wire.is_empty() && self.input.is_none() && self.completion.is_none()
        }
    }

    impl<const RETAINED: bool> ArtifactReservedJob for TestClipboardReservedJob<RETAINED> {
        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            Ok(match semio_framework_job::InteractiveJob::close_step(self, maximum_items, maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::InteractiveJobCloseStep::Blocked => PluginCloseStep::Blocked { reason: "test clipboard route close is blocked" },
                semio_framework_job::InteractiveJobCloseStep::Complete => PluginCloseStep::Complete,
            })
        }

        fn terminal_is_empty(&self) -> bool {
            semio_framework_job::InteractiveJob::terminal_is_empty(self)
        }
    }
    //#endregion 🧪️TestClipboardReservedJob

    struct PublicationPresenceRetirement {
        snapshot: Option<std::sync::Arc<PublicationPresence>>,
    }

    impl store::ErasedSnapshotRetirement for PublicationPresenceRetirement {
        fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
            if maximum_items == 0 {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if self.snapshot.take().is_some() {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            Ok(store::SnapshotRetirementStep::Complete)
        }

        fn terminal_is_empty(&self) -> bool {
            self.snapshot.is_none()
        }
    }

    struct PublicationPresenceRetirementFactory;

    impl store::SnapshotRetirementFactory<PublicationPresence> for PublicationPresenceRetirementFactory {
        fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
            Box::new(PublicationPresenceRetirement { snapshot: Some(snapshot) })
        }
    }

    //#region 🧹️PublicationLaneFixtureOwners
    struct TestPublicationPresenceStoreDisposer(Option<store::PresenceStoreRetirement<PublicationPresence>>);

    impl ArtifactOwnedDisposer<store::PresenceStore<PublicationPresence, PublicationPresenceMutation>> for TestPublicationPresenceStoreDisposer {
        fn close_step(&mut self, owner: &mut store::PresenceStore<PublicationPresence, PublicationPresenceMutation>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            if maximum_items == 0 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if let Some(active) = self.0.as_mut() {
                return active.close_step(1, maximum_bytes).map_err(Fault::from).map(|step| match step {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
                    store::SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "presence fixture retains captured readers" },
                    store::SnapshotRetirementStep::Complete => PluginCloseStep::Complete,
                });
            }
            self.0 = Some(owner.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).map_err(|(reason, _)| Fault::from(reason))?);
            Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
        }

        fn terminal_is_empty(&self, owner: &store::PresenceStore<PublicationPresence, PublicationPresenceMutation>) -> bool {
            owner.retirement_started() && self.0.as_ref().is_some_and(store::PresenceStoreRetirement::terminal_is_empty) && owner.peers_root().is_empty()
        }
    }

    struct TestPublicationTransientStoreDisposer;

    impl ArtifactOwnedDisposer<store::TransientStore<PublicationTransient, PublicationTransientMutation>> for TestPublicationTransientStoreDisposer {
        fn close_step(&mut self, _owner: &mut store::TransientStore<PublicationTransient, PublicationTransientMutation>, maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            if maximum_items == 0 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            Ok(PluginCloseStep::Complete)
        }

        fn terminal_is_empty(&self, _owner: &store::TransientStore<PublicationTransient, PublicationTransientMutation>) -> bool {
            true
        }
    }
    //#endregion 🧹️PublicationLaneFixtureOwners

    impl<const RETAINED: bool> ArtifactApp for TestApp<RETAINED> {
        const DIALECT: Dialect = TEST_APP_DIALECT;
        fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
            test_restart_proofs::<RETAINED>()
        }

        fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
            test_restart_register::<RETAINED>(registry)
        }

        async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
            test_restart_build::<RETAINED>(request).await
        }

        fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
            if RETAINED { Some(std::sync::Arc::new(TestCountOneItemPreparationFactory)) } else { None }
        }

        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(std::sync::Arc::new(PublicationPresenceRetirementFactory))
        }
        // 🪪️ Must equal `test_app_surface_id()` — a hand-typed `&'static str` because the runtime
        // `ArtifactApp::APP_ID` const (contract §2.1, "kept") cannot call a heap-allocating fn at
        // compile time; `test_app_id_matches_its_own_dialect` below is the drift guard.
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
            Some(std::sync::Arc::new(PublicationPresenceRetirementFactory))
        }

        fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
            Some(bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
        }

        fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
            Some(bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
        }

        fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
            Some(bounded_document_store_owners::<Self::Draft, Self::DraftMutation>())
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
            Some(Box::new(TestPublicationPresenceStoreDisposer(None)))
        }

        fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            if RETAINED { Some(Box::new(TestRestartTransientDisposer::new())) } else { Some(Box::new(TestPublicationTransientStoreDisposer)) }
        }

        fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
            Ok(matches!(request.tool_id.as_str(), "copy" | "cut" | "paste").then(|| ArtifactReservedToolJob::new(TestClipboardReservedJob::new(request))))
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
            match command {
                TestCommand::Increment => "increment",
                TestCommand::SetLabel { .. } => "setLabel",
                TestCommand::AmendLabel { .. } => "amendLabel",
                TestCommand::CommitLabel { .. } => "commitLabel",
                TestCommand::BadView => "badView",
                TestCommand::Select { .. } => "select",
                TestCommand::Navigate => "navigate",
                TestCommand::NoopMutation => "noopMutation",
                TestCommand::ViewNoScope => "viewNoScope",
                TestCommand::ViewPartialScope => "viewPartialScope",
                TestCommand::IncrementViaCommand => "incrementViaCommand",
                TestCommand::WatchdogOverrun => "watchdogOverrun",
                TestCommand::SetLabelViaCommand { .. } => "setLabelViaCommand",
                TestCommand::SetActiveUtility { .. } => "setActiveUtility",
                TestCommand::CompositeEdit { .. } => "compositeEdit",
                TestCommand::ProbeChild { .. } => "probeChild",
                TestCommand::SpawnCountTask => "spawnCountTask",
                TestCommand::ApplyCountFromTask { .. } => "applyCountFromTask",
            }
        }

        async fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
            match action {
                "incrementViaCommand" | "mode.increment" => Ok(TestCommand::IncrementViaCommand),
                "setLabelViaCommand" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("value")).and_then(DslValue::as_str).unwrap_or_default().to_string() }),
                "targetWindow" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("windowId")).and_then(DslValue::as_str).unwrap_or_default().to_string() }),
                "probeChild" => Ok(TestCommand::ProbeChild {
                    slot: args.and_then(|value| value.get("slot")).and_then(DslValue::as_str).unwrap_or_default().to_string(),
                    child_id: args.and_then(|value| value.get("childId")).and_then(DslValue::as_str).unwrap_or_default().to_string(),
                }),
                // 🎯️ M1 (ticket 26/08/17 `design-unified.md`): reachable ONLY through the
                // `command_from_intent` bridge (this string is never dispatched by any other
                // existing test) — the `🕹️IntentDispatchTests` fixture proves kind discipline
                // survives the new intent path exactly like it already does for `dispatch_typed`.
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
            _engines: &EngineHandles,
        ) -> Result<Emit<TestMutation, TestConfigMutation>, Fault> {
            let _ = Self::command_id(command);
            match command {
                TestCommand::Increment | TestCommand::IncrementViaCommand => Ok(Emit { artifact_mutations: vec![TestMutation::SetCount(SetCount { value: doc.snapshot.count + 1 })], description: Some("increment".into()), ..Default::default() }),
                TestCommand::WatchdogOverrun => {
                    let started = std::time::Instant::now();
                    while started.elapsed() < std::time::Duration::from_millis(10) {
                        std::hint::spin_loop();
                    }
                    Ok(Emit::default())
                }
                TestCommand::SetLabel { value } => Ok(Emit { artifact_mutations: vec![TestMutation::SetLabel(SetLabel { value: value.clone() })], coalesce_key: Some("label".into()), ..Default::default() }),
                TestCommand::SetLabelViaCommand { value } => Ok(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value: value.clone() })])),
                TestCommand::AmendLabel { value } => Ok(Emit::amend(vec![TestMutation::SetLabel(SetLabel { value: value.clone() })], "label")),
                TestCommand::CommitLabel { value } => Ok(Emit::commit(vec![TestMutation::SetLabel(SetLabel { value: value.clone() })], "commit label")),
                TestCommand::BadView => Ok(Emit::mutations(vec![TestMutation::SetCount(SetCount { value: 99 })])),
                TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: json!({ "utilityId": utility_id.clone() }).into() })),
                TestCommand::Select { id } => Ok(Emit::config(vec![ChangeTestConfigSelection { selected: id.clone() }.into()])),
                TestCommand::Navigate => Ok(Emit::effect(Effect::Navigate { uri: "semio://home".into() })),
                TestCommand::NoopMutation => Ok(Emit::default()),
                TestCommand::ViewNoScope => Ok(Emit { ui_scope: UiDirtyScope::None, ..Default::default() }),
                TestCommand::ViewPartialScope => {
                    Ok(Emit { ui_scope: UiDirtyScope::Partial { window_bodies: vec!["some.window".into()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false }, ..Default::default() })
                }
                TestCommand::CompositeEdit { slot, child_id, child_value } => Ok(Emit {
                    artifact_mutations: vec![TestMutation::SetLabel(SetLabel { value: "composite".into() })],
                    child_emits: vec![ChildEmit::of::<TestSnapshot, _>(slot.clone(), child_id.clone(), &[TestMutation::SetCount(SetCount { value: *child_value })])],
                    ..Default::default()
                }),
                TestCommand::ProbeChild { slot, child_id } => {
                    let _snapshot = doc.children.typed_read::<TestSnapshot>(slot, child_id)?;
                    Ok(Emit::effect(Effect::DispatchAction { req: RequestId(91_001), action: "probeChildContinuation".into(), args: None, delay_ms: 0 }))
                }
                // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: no mutations of its own — the
                // task's eventual `TaskResolution::Command` follow-up is what mutates the
                // document, on a LATER dispatch (see `ApplyCountFromTask` below).
                TestCommand::SpawnCountTask => Ok(Emit::task(
                    AsyncTask::new("spawn-count-task", |ctx: TaskCtx| async move {
                        let bytes = ctx.host.storage_read("counter").await?;
                        let value = i32::from_le_bytes(bytes.try_into().unwrap_or([0; 4]));
                        let command = TestCommand::ApplyCountFromTask { value };
                        let encoded = <TestCommand as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
                        Ok(TaskResolution::Command(encoded))
                    })
                    .await,
                )),
                TestCommand::ApplyCountFromTask { value } => Ok(Emit::mutations(vec![TestMutation::SetCount(SetCount { value: *value })])),
            }
        }

        async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            RENDER_CONTEXT_PROBE.with(|probe| probe.replace(Some((body_key.into(), view_state.clone()))));
            if matches!(body_key, "graph" | "properties") {
                let item = TreeNode::try_new("item-1", Component::TreeItem(TreeItemProps {
                    label: Label(UiText::try_from_str("Item 1").expect("bounded fixture")), description: None, icon: None, default_open: None,
                    draggable: None, drag_data: None, dimmed: None, row_actions: UiFixedList::default(),
                })).expect("bounded fixture");
                let root = TreeNode::try_new("root", Component::Tree(TreeProps { interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }))
                    .expect("bounded fixture").try_with_children([item]).unwrap_or_else(|_| panic!("bounded fixture"));
                return Ok(ComponentTree { root });
            }
            built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("count={}", doc.snapshot.count)))
        }

        /// 🪟️ One group per live window instance — the reserved `measures` section surface's own payload.
        async fn window_measures(_doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, view_state: &ViewModel) -> std::collections::HashMap<String, Vec<ui_wgpu::wgpu::WindowMeasure>> {
            view_state.window_instances.iter().map(|window| (window.id.clone(), vec![ui_wgpu::wgpu::WindowMeasure::measure_group(format!("{}-measure", window.id), format!("{} measures", window.window_kind_id), Vec::new())])).collect()
        }

        /// 🛠️ One group for the active tool — the reserved `tools` section surface's own payload.
        async fn tool_measures(_doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, view_state: &ViewModel) -> std::collections::HashMap<String, Vec<ui_wgpu::wgpu::WindowMeasure>> {
            view_state.active_tool_id.iter().map(|tool| (tool.clone(), vec![ui_wgpu::wgpu::WindowMeasure::measure_group(format!("{tool}-measure"), format!("{tool} tool"), Vec::new())])).collect()
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
        async fn context_menu(_request: &ContextMenuRequest, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
            // 🩹️ Rewritten from `Menu::when(cond, |m| m.command(..))` — `when`'s closure param is
            // `impl FnOnce(Self) -> Self` (sync, a public documented pattern other plugins use), but
            // `Menu::command`/`action` are genuinely async (they await `AppActionRegistry::get*`). An
            // async closure can't satisfy a sync `FnOnce`, so this fixture inlines the two guarded
            // branches as explicit `if`s instead of touching `when`'s public signature. See R10 residue
            // class 1 (`` inside a sync closure).
            let mut menu = Menu::of(registry).action("setLabelRequired");
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
        async fn interaction_topology(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> InteractionTopology {
            let mut domains = BTreeMap::new();
            let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![TopologyNode { id: "item-1".into(), granularity: "item".into(), parent: None }] };
            domains.insert("items".to_string(), DomainTopology { ordered });
            InteractionTopology { domains }
        }
    }

    //#region 🗝️RegisteredKeyedDispatchFixture
    #[derive(Default)]
    struct KeyedTestApp;

    struct KeyedTestCommandDisposer;

    impl ArtifactOwnedDisposer<TestCommand> for KeyedTestCommandDisposer {
        fn close_step(&mut self, command: &mut TestCommand, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            if maximum_items == 0 || maximum_bytes < 4 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let TestCommand::CompositeEdit { slot, child_id, .. } = command else {
                return Err(Fault::from("keyed fixture owns only its composite command"));
            };
            if let Some(character) = slot.pop().or_else(|| child_id.pop()) {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: character.len_utf8() });
            }
            Ok(PluginCloseStep::Complete)
        }

        fn terminal_is_empty(&self, command: &TestCommand) -> bool {
            matches!(command, TestCommand::CompositeEdit { slot, child_id, .. } if slot.is_empty() && child_id.is_empty())
        }
    }

    struct KeyedTestJob {
        command: Option<Box<TestCommand>>,
        completion: Option<ArtifactToolCompletion<KeyedTestApp>>,
        raw: Option<action_bus::RetainedToolWireInput>,
        page: usize,
        base_count: i32,
        closing: bool,
    }

    impl semio_framework_job::InteractiveJob for KeyedTestJob {
        fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
            if cx.is_cancelled() {
                return semio_framework_job::StepOutcome::Cancelled;
            }
            if cx.should_yield() {
                return semio_framework_job::StepOutcome::Yield;
            }
            let Some(now_us) = cx.now_us() else {
                return semio_framework_job::StepOutcome::Yield;
            };
            assert!(cx.deadline_us().saturating_sub(now_us) <= 500, "registered factory deadline must preserve its exact 500us contract");
            if self.raw.as_ref().is_some_and(|raw| self.page < raw.page_count()) {
                self.page += 1;
                return semio_framework_job::StepOutcome::Yield;
            }
            let TestCommand::CompositeEdit { slot, child_id, child_value } = self.command.as_deref().unwrap() else {
                panic!("exact keyed fixture command");
            };
            let child_emits = (!slot.is_empty()).then(|| ChildEmit::of::<TestSnapshot, _>(slot.clone(), child_id.clone(), &[TestMutation::SetCount(SetCount { value: *child_value })])).into_iter().collect();
            let emit = Emit { artifact_mutations: vec![TestMutation::SetCount(SetCount { value: self.base_count + child_value })], child_emits, description: Some("retained composite edit".into()), ..Default::default() };
            self.completion.as_ref().unwrap().complete(Ok(emit), EphemeralEmit::default()).expect("one exact keyed completion");
            semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            })
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
            if !self.closing || maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            }
            if let Some(raw) = self.raw.as_mut() {
                let step = raw.close_step(1, maximum_bytes);
                if raw.terminal_is_empty() {
                    self.raw = None;
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                return step;
            }
            if let Some(command) = self.command.as_mut() {
                match KeyedTestCommandDisposer.close_step(command, 1, maximum_bytes).unwrap() {
                    PluginCloseStep::Pending { released_items, released_bytes } => return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    PluginCloseStep::AwaitingInput { .. } => return semio_framework_job::InteractiveJobCloseStep::Blocked,
                    PluginCloseStep::Blocked { .. } => return semio_framework_job::InteractiveJobCloseStep::Blocked,
                    PluginCloseStep::Complete => {
                        assert!(KeyedTestCommandDisposer.terminal_is_empty(command));
                        self.command = None;
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                }
            }
            if self.completion.take().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            semio_framework_job::InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.raw.is_none() && self.command.is_none() && self.completion.is_none()
        }
    }

    struct KeyedTestFactory {
        keys: Vec<ToolFactoryKey>,
    }

    impl ToolJobFactory for KeyedTestFactory {
        type Payload = KeyedTestJob;
        type Job = KeyedTestJob;
        fn keys(&self) -> &[ToolFactoryKey] {
            &self.keys
        }
        fn payload_schema_id(&self) -> &str {
            "semio.test.keyed-command.v1"
        }
        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }
        fn execution_contract(&self) -> ToolExecutionContract {
            ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1)
        }
        fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            Ok(payload)
        }
        fn create_job_from_wire_pages_with_payload(
            &mut self,
            _operation: semio_framework_job::Operation,
            mut payload: Self::Payload,
            input: action_bus::RetainedToolWireInput,
            checkpoint: Option<action_bus::RetainedToolWireInput>,
        ) -> Result<Self::Job, (ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
            assert!(checkpoint.is_none());
            payload.raw = Some(input);
            Ok(payload)
        }
    }

    impl ArtifactOwnedToolJobFactory for KeyedTestFactory {
        type Owner = KeyedTestApp;
        const TOOL_IDS: &'static [&'static str] = &["compositeEdit"];
        const DOCUMENT_SCHEMA: &'static str = KeyedTestApp::DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Child] }];
        fn latest_wins_target(command: &TestCommand) -> Option<&str> {
            match command {
                TestCommand::CompositeEdit { child_id, .. } => Some(child_id),
                _ => None,
            }
        }
        fn build_latest_wins_command_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<TestCommand>>> {
            Some(Box::new(KeyedTestCommandDisposer))
        }
    }

    //#region 🧹️KeyedNoStateFixtureOwners
    //#endregion 🧹️KeyedNoStateFixtureOwners

    impl ArtifactApp for KeyedTestApp {
        const DIALECT: Dialect = Dialect { artifact_kind: "s.test.keyed", standard: StandardId("1"), subset: SubsetId::ANY };
        const APP_ID: &'static str = "s.test.keyed@1/*#editor";
        const DOCUMENT_SCHEMA: &'static str = "semio.test/v1";
        type Snapshot = TestSnapshot;
        type Mutation = TestMutation;
        type Config = TestConfig;
        type ConfigMutation = TestConfigMutation;
        type Draft = NoDraft;
        type DraftMutation = NoDraftMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = NoTransient;
        type TransientMutation = NoTransientMutation;
        type Command = TestCommand;
        crate::bounded_first_step_tool_proofs! {
            owner: KeyedTestApp, owner_file: "plugin/🦀️.rs", controller: "s.test.keyed@1/*#editor", document_schema: "semio.test/v1",
            factory: "KeyedTestFactory", factory_type: KeyedTestFactory,
            contract: ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
        }
        fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
            registry.register(KeyedTestFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
        }
        async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
            assert!(request.snapshot.label.is_empty());
            let job = KeyedTestJob { command: Some(request.command), completion: Some(request.completion), raw: None, page: 0, base_count: request.snapshot.count, closing: false };
            Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
        }
        fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
            Some(std::sync::Arc::new(TestCountOneItemPreparationFactory))
        }
        fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
            TestApp::<false>::build_document_store_owners()
        }
        fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
            TestApp::<false>::build_config_store_owners()
        }
        fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
            TestApp::<false>::build_draft_store_owners()
        }
        fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            TestApp::<false>::build_document_store_disposer()
        }
        fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            TestApp::<false>::build_config_store_disposer()
        }
        fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
            TestApp::<false>::build_draft_store_disposer()
        }
        fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(mutation_fixture::no_state::presence_store_disposer())
        }
        fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(mutation_fixture::no_state::transient_store_disposer())
        }
        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(mutation_fixture::no_state::presence_peer_retirement_factory())
        }
        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(mutation_fixture::no_state::presence_local_root_retirement_factory())
        }
        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(mutation_fixture::no_state::transient_local_root_retirement_factory())
        }
        async fn initial_snapshot() -> TestSnapshot {
            TestSnapshot::default()
        }
        async fn command_id(command: &TestCommand) -> &'static str {
            TestApp::<false>::command_id(command).await
        }
        async fn handle(
            _command: &TestCommand,
            _doc: &ArtifactView<'_, TestSnapshot>,
            _cfg: &ConfigView<'_, TestConfig>,
            _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>,
            _draft: &DraftView<'_, NoDraft>,
            _engines: &EngineHandles,
        ) -> Result<Emit<TestMutation, TestConfigMutation>, Fault> {
            Err(Fault::from("keyed fixture requires its actual retained factory"))
        }
        async fn render(body: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            TestApp::<false>::render(body, doc, cfg, _view_state).await
        }
    }

    #[test]
    fn keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners() {
        fn close_root<T: Send + Sync + 'static>(factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<T>>, root: T) {
            let mut retirement = store::SnapshotRetirementFactory::retire(factory.as_ref(), std::sync::Arc::new(root));
            assert!(matches!(store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 0, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })));
            for _ in 0..4 {
                match store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("bounded root retirement") {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES),
                    store::SnapshotRetirementStep::Complete => {
                        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()));
                        return;
                    }
                    store::SnapshotRetirementStep::Blocked => panic!("no-state root retirement must not block"),
                }
            }
            panic!("no-state root retirement exceeded its bounded close turns");
        }

        close_root(<KeyedTestApp as ArtifactApp>::build_presence_local_root_retirement_factory().expect("presence local factory"), NoPresence::default());
        close_root(<KeyedTestApp as ArtifactApp>::build_presence_peer_retirement_factory().expect("presence peer factory"), NoPresence::default());
        close_root(<KeyedTestApp as ArtifactApp>::build_transient_local_root_retirement_factory().expect("transient local factory"), NoTransient::default());

        let mut presence = store::PresenceStore::<NoPresence, NoPresenceMutation>::new(NoPresence::default());
        presence.install_local_retirement_factory(<KeyedTestApp as ArtifactApp>::build_presence_local_root_retirement_factory().expect("presence local factory")).expect("presence local factory installs once");
        presence.install_peer_retirement_factory(<KeyedTestApp as ArtifactApp>::build_presence_peer_retirement_factory().expect("presence peer factory")).expect("presence peer factory installs once");
        let mut presence_disposer = <KeyedTestApp as ArtifactApp>::build_presence_store_disposer().expect("presence disposer");
        assert_eq!(presence_disposer.close_step(&mut presence, 0, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("zero grant"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        for _ in 0..8 {
            if presence_disposer.close_step(&mut presence, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("presence close") == PluginCloseStep::Complete {
                break;
            }
        }
        assert!(presence_disposer.terminal_is_empty(&presence));
        assert!(presence.retirement_started() && presence.peers_root().is_empty());

        let mut transient = store::TransientStore::<NoTransient, NoTransientMutation>::new(NoTransient::default());
        let mut transient_disposer = <KeyedTestApp as ArtifactApp>::build_transient_store_disposer().expect("transient disposer");
        let original_transient_root = transient.current_root();
        let original_transient_weak = std::sync::Arc::downgrade(&original_transient_root);
        let original_transient_generation = transient.generation_now();
        assert_eq!(transient_disposer.close_step(&mut transient, 0, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("zero grant"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert!(!transient_disposer.terminal_is_empty(&transient));
        let short_transient_grant = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES.checked_sub(1).expect("nonzero transient retirement page");
        assert_eq!(transient_disposer.close_step(&mut transient, 1, short_transient_grant).expect("short transient grant"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(transient.generation_now(), original_transient_generation);
        assert!(std::sync::Arc::ptr_eq(&original_transient_root, &transient.current_root()));
        assert_eq!(
            transient_disposer.close_step(&mut transient, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("transient owned-store retirement"),
            PluginCloseStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES }
        );
        assert_eq!(transient.generation_now(), 0);
        assert!(!std::sync::Arc::ptr_eq(&original_transient_root, &transient.current_root()));
        drop(original_transient_root);
        assert!(original_transient_weak.upgrade().is_none());
        assert_eq!(transient_disposer.close_step(&mut transient, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("transient terminal completion"), PluginCloseStep::Complete);
        assert!(transient_disposer.terminal_is_empty(&transient));
        assert_eq!(transient_disposer.close_step(&mut transient, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("transient repeated complete"), PluginCloseStep::Complete);
        transient = store::TransientStore::new(NoTransient::default());
        let drifted_transient_root = transient.current_root();
        assert!(!transient_disposer.terminal_is_empty(&transient));
        assert!(transient_disposer.close_step(&mut transient, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).is_err(), "transient terminal witness rejects owner drift");
        assert!(std::sync::Arc::ptr_eq(&drifted_transient_root, &transient.current_root()));
    }

    async fn keyed_test_registry() -> AppActionRegistry {
        let manifest = App::from_builder(
            App::builder(KeyedTestApp::APP_ID, LocalizedLabel::data("Keyed Fixture"))
                .await
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await
                .app_command("compositeEdit", LocalizedLabel::data("Set Parameter"), "fixture", ActionKind::Mutation)
                .await
                .interactive_jobs(InteractiveJobClassification::Migrated)
                .await,
        )
        .await;
        AppActionRegistry::from_definition(&manifest.definition)
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document() {
        test_retained_keyed_dispatch::<KeyedTestApp>(
            keyed_test_registry().await,
            |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value },
            |value| TestMutation::SetCount(SetCount { value }),
            |snapshot| snapshot.count,
            None,
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id() {
        test_typed_operation_slot_preadmission::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn microsecond_registered_factory_dispatch_preserves_exact_half_ms_fake_clock() {
        fn clock() -> Option<u64> {
            Some(1_000)
        }
        test_retained_keyed_dispatch::<KeyedTestApp>(
            keyed_test_registry().await,
            |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value },
            |value| TestMutation::SetCount(SetCount { value }),
            |snapshot| snapshot.count,
            Some(clock),
        )
        .await;
        eprintln!("[DEBUG] registered 500us factory completed real dispatch/rebase/publication/ACK/close with exact fake microsecond clock");
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_operation_continues_after_command_admission_until_publication_and_retirement() {
        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🧫️fixtures/🔣️.json")).unwrap();
        let id = fixture["wire"]["receiver"].as_u64().unwrap() as u32;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_test_registry().await).await;
        app.bind_instance_id(id).await;
        let value = fixture["command"]["value"].as_i64().unwrap() as i32;
        let command = TestCommand::CompositeEdit { slot: String::new(), child_id: fixture["command"]["target"].as_str().unwrap().into(), child_value: value };
        app.dispatch_typed(command, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }).await.unwrap();
        let runtime = super::PluginRuntime::new();
        let cell = std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let mut terminal = false;
        let mut receipts = 0;
        for turn in 0..fixture["command"]["maximumTurns"].as_u64().unwrap() {
            super::plugin_step_live_cleanup(&runtime).unwrap();
            let (output, scan) = super::plugin_continue_typed_operations(&runtime).await.unwrap();
            let more = scan.runnable || scan.contended;
            if let Some((receiver, output)) = output {
                assert_eq!(receiver, id);
                if let Some(page) = output.typed_operation_result {
                    assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                    terminal |= page.lane == TypedOperationResultLane::Terminal;
                    receipts += 1;
                    super::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
                }
            }
            if !more {
                assert!(terminal, "the actor must remain runnable until its terminal result");
                eprintln!("[DEBUG] admitted command published and retired after {turn} turns with {receipts} exact receipts");
                break;
            }
            std::thread::yield_now();
        }
        let active = cell.instance.lock().unwrap();
        assert!(terminal);
        assert!(!active.app.has_pending_typed_operations());
        assert_eq!(active.app.snapshot().unwrap().count, value);
        drop(active);
        let retired = std::sync::Arc::downgrade(&cell);
        drop(cell);
        super::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            if let Err(error) = super::plugin_step_close_cleanup(&runtime) {
                let entries = runtime.close_quarantine.borrow();
                let state = &entries.get(id).unwrap().state;
                let pump = state.pump.lock().unwrap();
                let detail = state.last_fault.lock().unwrap();
                panic!(
                    "{error:?}: elapsed={} stalled={} terminal={} complete={} blocked={} faulted={} pending={:?} origin={} detail={}",
                    state.last_callback_elapsed_us.load(std::sync::atomic::Ordering::SeqCst),
                    state.stalled_steps.load(std::sync::atomic::Ordering::SeqCst),
                    pump.terminal,
                    pump.complete,
                    pump.blocked,
                    pump.faulted,
                    pump.pending_status,
                    state.last_fault_origin.load(std::sync::atomic::Ordering::SeqCst),
                    String::from_utf8_lossy(&detail[..])
                );
            }
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(runtime.close_quarantine.borrow().get(id).is_none());
        assert!(retired.upgrade().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires() {
        let id = 41;
        let mut app = VcsArtifactApp::<KeyedTestApp, TestMembers>::with_registry(KeyedTestApp, keyed_test_registry().await).await;
        app.bind_instance_id(id).await;
        let mut child = new_test_child("child-1").await.expect("construct child");
        let TestMembers::Child(child_store) = &mut child;
        child_store.install_member_store_owners_exact(<TestSnapshot as store::MemberStoreOwner<TestMutation>>::member_store_owners());
        app.register_child("slot", "child-1", test_child_dialect().await, child).await.expect("register child");
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 9 }, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }).await.expect("admit retained child gesture");

        let runtime = super::PluginRuntime::new();
        let cell = std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let acknowledgement_fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🥇️tool-latest-wins.json")).expect("language-neutral result ACK fixture");
        let mut lanes = Vec::new();
        for turn in 0..100_000 {
            if let Err(error) = super::plugin_step_live_cleanup(&runtime) {
                let pump = cell.maintenance_pump.lock().expect("failed maintenance pump");
                panic!(
                    "[DEBUG] {error:?}: turn={turn} status={} generation={} stalled={} entries={} session={} outcome={} rejected={} terminal={} pending={:?} closing={} faulted={}",
                    cell.maintenance_status.load(std::sync::atomic::Ordering::SeqCst),
                    cell.maintenance_generation.load(std::sync::atomic::Ordering::SeqCst),
                    cell.maintenance_stalled_steps.load(std::sync::atomic::Ordering::SeqCst),
                    cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed),
                    pump.session.is_some(),
                    pump.outcome.is_some(),
                    pump.rejected.is_some(),
                    pump.terminal,
                    pump.pending_status,
                    pump.closing,
                    pump.faulted
                );
            }
            let (output, scan) = super::plugin_continue_typed_operations(&runtime).await.expect("drive one production publication turn");
            let mut more = scan.runnable || scan.contended;
            if let Some((receiver, output)) = output {
                assert_eq!(receiver, id);
                if let Some(page) = output.typed_operation_result {
                    assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                    lanes.push(page.lane);
                    if page.lane == TypedOperationResultLane::Child {
                        assert_eq!(page.token.attempt, acknowledgement_fixture["resultAck"]["attempt"].as_u64().unwrap() as u8);
                        let delayed_token = page.token;
                        {
                            let active = cell.instance.lock().expect("delayed ACK retained app");
                            assert!(active.app.has_pending_typed_operations());
                            assert_eq!(active.app.typed_operation_result_state_for_test(delayed_token.operation), Some((delayed_token, TypedOperationResultLane::Child, true, false)));
                        }
                        let maintenance_entries_before = cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed);
                        let pre_ack_polls = acknowledgement_fixture["resultAck"]["preAckPolls"].as_u64().unwrap();
                        for poll in 1..=pre_ack_polls {
                            let expected_entries = maintenance_entries_before + poll;
                            for _ in 0..100_000 {
                                super::plugin_step_live_cleanup(&runtime).expect("delayed renderer ACK remains valid maintenance input wait");
                                plugin_job_yield_once().await;
                                if cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed) >= expected_entries {
                                    break;
                                }
                            }
                            assert!(cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed) >= expected_entries, "production maintenance callback must execute before counting one delayed ACK poll");
                            let (output, _) = super::plugin_continue_typed_operations(&runtime).await.expect("delayed renderer ACK does not fault continuation");
                            assert!(output.as_ref().and_then(|(_, output)| output.typed_operation_result.as_ref()).is_none(), "presented result page must not be republished before an explicit retry deadline");
                        }
                        assert!(cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed) >= maintenance_entries_before + pre_ack_polls);
                        let input_waits_before_alignment = cell.maintenance_probe_input_waits.load(std::sync::atomic::Ordering::SeqCst);
                        for _ in 0..100_000 {
                            super::plugin_step_live_cleanup(&runtime).expect("reach one processed exact ACK-wait maintenance stage");
                            plugin_job_yield_once().await;
                            if cell.maintenance_probe_input_waits.load(std::sync::atomic::Ordering::SeqCst) > input_waits_before_alignment
                                && super::RuntimeMaintenanceStatus::from_repr(cell.maintenance_status.load(std::sync::atomic::Ordering::SeqCst)) == super::RuntimeMaintenanceStatus::Ready
                                && cell.maintenance_stalled_steps.load(std::sync::atomic::Ordering::SeqCst) == 0
                            {
                                break;
                            }
                        }
                        assert!(cell.maintenance_probe_input_waits.load(std::sync::atomic::Ordering::SeqCst) > input_waits_before_alignment, "maintenance must process the presented result as an external input wait");
                        {
                            let active = cell.instance.lock().expect("stable delayed ACK retained app");
                            assert_eq!(active.app.typed_operation_result_state_for_test(delayed_token.operation), Some((delayed_token, TypedOperationResultLane::Child, true, false)));
                        }
                        assert_eq!(cell.maintenance_stalled_steps.load(std::sync::atomic::Ordering::SeqCst), 0);
                        super::plugin_acknowledge_typed_operation_result(&runtime, delayed_token).await.expect("delayed exact retained result ACK");
                        more = true;
                    } else {
                        super::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.expect("ack exact retained result");
                        more = true;
                    }
                }
            }
            if !more {
                assert!(lanes.contains(&TypedOperationResultLane::Terminal), "runtime retired before its terminal page");
                eprintln!("[DEBUG] retained Child runtime published every host lane and retired after {turn} turns");
                break;
            }
            plugin_job_yield_once().await;
        }
        assert!(lanes.contains(&TypedOperationResultLane::Child));
        assert!(lanes.contains(&TypedOperationResultLane::Terminal));
        {
            let mut active = cell.instance.lock().expect("live runtime app");
            assert!(!active.app.has_pending_typed_operations());
            assert_eq!(active.app.snapshot().expect("parent snapshot").count, 9);
            let TestMembers::Child(child) = &mut active.app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("live child").member;
            assert_eq!(child.snapshot().expect("child snapshot").count, 9);

            active.app.dispatch_action("undo", None, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }).await.expect("undo retained group");
            assert_eq!(active.app.snapshot().expect("undone parent snapshot").count, 0);
            let TestMembers::Child(child) = &mut active.app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("undone child").member;
            assert_eq!(child.snapshot().expect("undone child snapshot").count, 0);

            active.app.dispatch_action("redo", None, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }).await.expect("redo retained group");
            assert_eq!(active.app.snapshot().expect("redone parent snapshot").count, 9);
            let TestMembers::Child(child) = &mut active.app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("redone child").member;
            assert_eq!(child.snapshot().expect("redone child snapshot").count, 9);
        }
        let retired = std::sync::Arc::downgrade(&cell);
        drop(cell);
        super::plugin_destroy_app(&runtime, id).await.expect("begin runtime destroy");
        for _ in 0..100_000 {
            super::plugin_step_close_cleanup(&runtime).expect("drive exact runtime close");
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(runtime.close_quarantine.borrow().get(id).is_none());
        assert!(retired.upgrade().is_none());
        eprintln!("[DEBUG] retained Child publication emitted Child+Terminal ACK pages, moved one parent-child undo group, and retired every owner");
    }
    //#endregion 🗝️RegisteredKeyedDispatchFixture

    use crate::plugin_app_close_prelude::*;
    semio_framework_dispatch_macros::dyn_enum_close! {
        enum TestRuntimeApps: PluginApp {
            Test(VcsArtifactApp<TestApp>),
        }
    }

    /// 🧬️ A hostile structural copy of Draw's public controller/schema/tool constants. Its
    /// concrete Rust type is deliberately not the audited Draw owner type.
    #[derive(Default)]
    struct CopyDrawApp;

    impl ArtifactApp for CopyDrawApp {
        const DIALECT: Dialect = Dialect { artifact_kind: "s.draw.draw", standard: StandardId("1"), subset: SubsetId::ANY };
        const APP_ID: &'static str = "s.draw.draw@1/*#editor";
        const DOCUMENT_SCHEMA: &'static str = "draw.document";
        type Snapshot = TestSnapshot;
        type Mutation = TestMutation;
        type Config = TestConfig;
        type ConfigMutation = TestConfigMutation;
        type Draft = NoDraft;
        type DraftMutation = NoDraftMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = NoTransient;
        type TransientMutation = NoTransientMutation;
        type Command = TestCommand;

        async fn initial_snapshot() -> TestSnapshot {
            TestSnapshot::default()
        }

        async fn command_id(_command: &TestCommand) -> &'static str {
            "canvasPointerDown"
        }

        async fn handle(
            command: &TestCommand,
            doc: &ArtifactView<'_, TestSnapshot>,
            cfg: &ConfigView<'_, TestConfig>,
            interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>,
            draft: &DraftView<'_, NoDraft>,
            engines: &EngineHandles,
        ) -> Result<Emit<TestMutation, TestConfigMutation>, Fault> {
            TestApp::<false>::handle(command, doc, cfg, interaction, _view_state, draft, engines).await
        }

        async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            TestApp::<false>::render(body_key, doc, cfg, _view_state).await
        }
    }

    // 🚫️async: E1 pure constructor, called pervasively as `&meta()` — see R9.
    fn meta() -> ActionMeta {
        ActionMeta { actor: "local".into(), instance_id: 1, view_state: None }
    }

    async fn settle_reserved(app: &mut VcsArtifactApp<TestApp>, admitted: semio_framework::InvocationResult) -> semio_framework::InvocationResult {
        crate::app::settle_framework_reserved_admission(app, admitted).await.expect("settle reserved")
    }

    async fn reserved_action(app: &mut VcsArtifactApp<TestApp>, action: &str, args: Option<&DslValue>) -> semio_framework::InvocationResult {
        let admitted = app.handle_action(action, args, &meta()).await.expect(action);
        settle_reserved(app, admitted).await
    }

    fn close_reserved_app(app: &mut VcsArtifactApp<TestApp>) {
        for _ in 0..100_000 {
            match app.close_step(1, 4096).expect("reserved fixture closes") {
                crate::app::PluginCloseStep::Complete => return,
                crate::app::PluginCloseStep::Pending { .. } => {}
                other => panic!("reserved fixture close stalled: {other:?}"),
            }
        }
        panic!("reserved fixture close did not complete");
    }

    async fn synthetic_play_app() -> App {
        App::from_builder(
            App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
                .await
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await,
        )
        .await
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
    async fn contract_registry() -> AppActionRegistry {
        let app = App::from_builder(
            App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
                    .await.document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await.window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await.mutation("setLabelRequired", LocalizedLabel::data("Set Label"))
                .await.action_args("setLabelRequired", vec![ActionArgDef::text("value", LocalizedLabel::data("Value")).required()])
                // 🧪️ `Mutation`-kind by declaration, but `TestApp` emits zero operations for it — the
                // "declared Mutation action that happened to produce nothing" fixture.
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

    async fn contract_app_under_test() -> VcsArtifactApp<TestApp> {
        VcsArtifactApp::with_registry(TestApp::<false>::default(), contract_registry().await).await
    }

    #[semio_framework_async_macros::async_test]
    async fn activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations() {
        let platform = Platform::new(None).await;
        let registry = contract_registry().await;
        let declared = registry.test_migrated_tool_ids();
        let controller_id = registry.test_controller_id().to_string();
        let mut app = VcsArtifactApp::<TestApp>::with_registry_on_bus(TestApp::<false>::default(), registry.clone(), platform.action_bus.clone()).await;
        let registered: std::collections::BTreeSet<String> = app
            .test_registered_tool_keys()
            .into_iter()
            .map(|key| {
                assert_eq!(key.0, controller_id);
                key.1
            })
            .collect();
        assert_eq!(registered, declared);
        let platform_visible = platform.action_bus.keys().into_iter().filter(|key| key.controller_id == controller_id).map(|key| key.tool_id).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(platform_visible, declared);
        let before = platform.action_bus.dispatch_count();
        let error = app.dispatch_typed(TestCommand::IncrementViaCommand, &meta()).await.expect_err("an unproved typed command must remain fail closed");
        assert_eq!(error.code.0, "interactive-job.missing-factory");
        assert_eq!(platform.action_bus.dispatch_count(), before);
    }

    //#region 🧪️SharedFrameworkActionRouteTests
    #[semio_framework_async_macros::async_test]
    async fn shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity() {
        let platform = Platform::new(None).await;
        let registry = contract_registry().await;
        let controller_id = registry.test_controller_id().to_string();
        let app = VcsArtifactApp::<TestApp>::with_registry_on_bus(TestApp::<false>::default(), registry, platform.action_bus.clone()).await;
        let expected: [(&str, &str, std::any::TypeId, &'static str, usize); 12] = [
            ("copy", "framework.reserved.copy.v1", std::any::TypeId::of::<FrameworkCopyJobFactory<TestApp>>(), std::any::type_name::<FrameworkCopyJobFactory<TestApp>>(), 1_048_576),
            ("cut", "framework.reserved.cut.v1", std::any::TypeId::of::<FrameworkCutJobFactory<TestApp>>(), std::any::type_name::<FrameworkCutJobFactory<TestApp>>(), 1_048_576),
            ("paste", "framework.reserved.paste.v1", std::any::TypeId::of::<FrameworkPasteJobFactory<TestApp>>(), std::any::type_name::<FrameworkPasteJobFactory<TestApp>>(), 1_048_576),
            ("noteShellCommand", "framework.reserved.noteShellCommand.v1", std::any::TypeId::of::<FrameworkNoteShellCommandJobFactory<TestApp>>(), std::any::type_name::<FrameworkNoteShellCommandJobFactory<TestApp>>(), 65_536),
            (
                "setHistoryCommandFilter",
                "framework.reserved.setHistoryCommandFilter.v1",
                std::any::TypeId::of::<FrameworkSetHistoryCommandFilterJobFactory<TestApp>>(),
                std::any::type_name::<FrameworkSetHistoryCommandFilterJobFactory<TestApp>>(),
                4_096,
            ),
            ("recordTutorial", "framework.reserved.recordTutorial.v1", std::any::TypeId::of::<FrameworkRecordTutorialJobFactory<TestApp>>(), std::any::type_name::<FrameworkRecordTutorialJobFactory<TestApp>>(), 4_096),
            ("clearSelection", "framework.reserved.clearSelection.v1", std::any::TypeId::of::<FrameworkClearSelectionJobFactory<TestApp>>(), std::any::type_name::<FrameworkClearSelectionJobFactory<TestApp>>(), 4_096),
            ("interactionHover", "framework.reserved.interactionHover.v1", std::any::TypeId::of::<FrameworkInteractionHoverJobFactory<TestApp>>(), std::any::type_name::<FrameworkInteractionHoverJobFactory<TestApp>>(), 65_536),
            ("interactionSelect", "framework.reserved.interactionSelect.v1", std::any::TypeId::of::<FrameworkInteractionSelectJobFactory<TestApp>>(), std::any::type_name::<FrameworkInteractionSelectJobFactory<TestApp>>(), 65_536),
            ("selectAll", "framework.reserved.selectAll.v1", std::any::TypeId::of::<FrameworkSelectAllJobFactory<TestApp>>(), std::any::type_name::<FrameworkSelectAllJobFactory<TestApp>>(), 4_096),
            (
                "setInteractionGranularity",
                "framework.reserved.setInteractionGranularity.v1",
                std::any::TypeId::of::<FrameworkSetInteractionGranularityJobFactory<TestApp>>(),
                std::any::type_name::<FrameworkSetInteractionGranularityJobFactory<TestApp>>(),
                16_384,
            ),
            ("setSelectionMode", "framework.reserved.setSelectionMode.v1", std::any::TypeId::of::<FrameworkSetSelectionModeJobFactory<TestApp>>(), std::any::type_name::<FrameworkSetSelectionModeJobFactory<TestApp>>(), 16_384),
        ];
        let joined = platform.action_bus.keys().into_iter().filter(|key| key.controller_id == controller_id).map(|key| key.tool_id).collect::<std::collections::BTreeSet<_>>();
        let registrations = app.test_framework_tool_registrations();
        for (tool_id, schema_id, factory_type_id, factory_type_name, maximum) in expected {
            let (key, registered_schema_id, registered_factory_type_id, registered_factory_type_name, contract) = registrations.get(tool_id).expect("shared framework registration");
            assert_eq!(key, &ToolFactoryKey::new(&controller_id, tool_id));
            assert_eq!(registered_schema_id, schema_id);
            assert_eq!(*registered_factory_type_id, factory_type_id);
            assert_eq!(*registered_factory_type_name, factory_type_name);
            assert_eq!(contract.max_raw_wire_bytes, maximum);
            assert!(joined.contains(tool_id));
            let exact = vec![0_u8; maximum];
            let admission = platform.action_bus.admit_exact_wire(&controller_id, tool_id, schema_id, &exact).expect("exact maximum admission");
            assert_eq!(admission.factory_type_id, factory_type_id);
            assert_eq!(admission.factory_type_name, factory_type_name);
            assert!(platform.action_bus.admit_exact_wire(&controller_id, tool_id, schema_id, &vec![0_u8; maximum + 1]).is_err());
        }
    }

    #[test]
    fn shared_framework_job_is_cancellable_resumable_and_boundedly_closeable() {
        fn context<'a>(cancel: semio_framework_job::CancelToken, preview_sequence: &'a mut u64) -> semio_framework_job::StepContext<'a> {
            semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(7), semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel, || Some(0), preview_sequence)
        }

        fn close_payload(mut payload: semio_framework_job::RetainedJobPayload) {
            while !payload.terminal_is_empty() {
                let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
        }

        let cancel_before = semio_framework_job::CancelToken::root_now();
        cancel_before.cancel_now();
        let mut cancelled_before = FrameworkSetSelectionModeJob::new(vec![1; 8_193], 1);
        let mut preview_sequence = 0;
        assert!(matches!(semio_framework_job::InteractiveJob::step(&mut cancelled_before, &mut context(cancel_before, &mut preview_sequence)), semio_framework_job::StepOutcome::Cancelled));

        let cancel_after = semio_framework_job::CancelToken::root_now();
        let mut cancelled_after = FrameworkSetSelectionModeJob::new(vec![1; 8_193], 1);
        let mut preview_sequence = 0;
        match semio_framework_job::InteractiveJob::step(&mut cancelled_after, &mut context(cancel_after.clone(), &mut preview_sequence)) {
            semio_framework_job::StepOutcome::PreviewReady(payload) => close_payload(payload),
            _ => panic!("first retained step must publish preview"),
        }
        cancel_after.cancel_now();
        assert!(matches!(semio_framework_job::InteractiveJob::step(&mut cancelled_after, &mut context(cancel_after, &mut preview_sequence)), semio_framework_job::StepOutcome::Cancelled));

        let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 29);
        let mut retained = ArtifactReservedToolJob::new(FrameworkSetSelectionModeJob::new(vec![2; 8_193], 1));
        retained.bind_operation(operation).expect("first exact operation authority");
        assert!(retained.clone().bind_operation(operation).is_err(), "a second factory transfer cannot replace the live operation generation");
        semio_framework_job::InteractiveJob::begin_close(&mut retained);
        assert_eq!(semio_framework_job::InteractiveJob::close_step(&mut retained, 0, 0), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(semio_framework_job::InteractiveJob::close_step(&mut retained, 1, 4_096), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 4_096 });
        assert_eq!(semio_framework_job::InteractiveJob::close_step(&mut retained, 1, 4_096), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 4_096 });
        assert_eq!(semio_framework_job::InteractiveJob::close_step(&mut retained, 1, 4_096), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 1 });
        assert_eq!(semio_framework_job::InteractiveJob::close_step(&mut retained, 1, 4_096), semio_framework_job::InteractiveJobCloseStep::Complete);
        assert!(semio_framework_job::InteractiveJob::terminal_is_empty(&retained));
        assert_eq!(semio_framework_job::InteractiveJob::close_step(&mut retained, 1, 4_096), semio_framework_job::InteractiveJobCloseStep::Complete);
    }
    //#endregion 🧪️SharedFrameworkActionRouteTests

    #[semio_framework_async_macros::async_test]
    async fn retained_factory_proof_requires_the_exact_registered_runtime_authority() {
        test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(contract_registry().await, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TestRetainedCommandFactory::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_cancellation_guards_real_store_publication_and_preserves_committed_ack() {
        test_retained_cancellation_publication_boundaries::<TestApp>().await;
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_reserved_slots_and_ready_publisher_are_fair() {
        test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
    }

    #[test]
    fn retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement() {
        crate::retained_command::test_raw_allocation_close::<TestApp>();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close() {
        test_retained_document_cancellation::<TestApp>(std::sync::Arc::new(TestCountOneItemPreparationFactory), || TestMutation::SetCount(SetCount { value: 42 }), |snapshot| snapshot.count).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn copied_app_type_cannot_inherit_same_controller_schema_and_id_proof() {
        let mut owner = contract_registry().await;
        let mut declaration = owner.actions.get("noopMutation").expect("fixture action").clone();
        declaration.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        owner.actions.insert("canvasPointerDown".into(), declaration);
        owner.controller_id = "s.draw.draw@1/*#editor".into();
        assert!(test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));

        let audited_draw_row =
            ArtifactBoundedFirstStepProof::new::<TestApp>("owner.rs", "s.draw.draw@1/*#editor", BOUNDED_FIRST_STEP_FACTORY, "canvasPointerDown", "draw.document", ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500));
        let audited_draw_proof = QualifiedBoundedFirstStepProof { owner: audited_draw_row.owner, row: audited_draw_row };
        assert!(TypedCommandFullOperationJobFactory::<CopyDrawApp>::from_proof(audited_draw_proof).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn tool_cancellation_isolated_by_live_instance_with_same_controller_and_document() {
        let handle = ToolCancellationHandle::default();
        let first_key = ToolOperationKey {
            app_instance_id: 41,
            document: ArtifactDocumentAuthority(41),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(7),
            generation: semio_framework_job::Generation(3),
        };
        let second_key = ToolOperationKey { app_instance_id: 42, document: ArtifactDocumentAuthority(42), operation_id: semio_framework_job::allocate_operation_id(), ..first_key.clone() };
        let first_lease = handle.test_begin(first_key).expect("first numeric authority");
        let second_lease = handle.test_begin(second_key).expect("second numeric authority");
        let first_token = first_lease.cancel_token();
        let second_token = second_lease.cancel_token();
        assert_eq!(handle.active_operation_count(), 2);
        assert!(handle.cancel_document(ArtifactDocumentAuthority(41)).expect("document cancellation"));
        assert!(first_token.is_cancelled().await);
        assert!(!second_token.is_cancelled().await);
        assert_eq!(handle.active_operation_count(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn cancellation_supersession_and_saturated_app_close_are_parent_scope_constant_time() {
        let handle = ToolCancellationHandle::default();
        let key = ToolOperationKey {
            app_instance_id: 7,
            document: ArtifactDocumentAuthority(7),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(1),
            generation: semio_framework_job::Generation(1),
        };
        let first = handle.test_begin(key.clone()).expect("first generation");
        let first_token = first.cancel_token();
        let second = handle.test_begin(ToolOperationKey { operation_id: semio_framework_job::allocate_operation_id(), ..key }).expect("replacement generation");
        let second_token = second.cancel_token();
        assert!(first_token.is_cancelled().await);
        assert!(!second_token.is_cancelled().await);
        assert_eq!(handle.active_operation_count(), 1);
        drop(first);
        assert_eq!(handle.active_operation_count(), 1, "a stale lease must not release the replacement generation");
        second.finish();

        let mut saturated = Vec::new();
        for index in 0..1_024u32 {
            saturated.push(
                handle
                    .test_begin(ToolOperationKey {
                        app_instance_id: index.saturating_add(100),
                        document: ArtifactDocumentAuthority(index.saturating_add(100)),
                        operation_id: semio_framework_job::allocate_operation_id(),
                        base_revision: semio_framework_job::RevisionId(1),
                        generation: semio_framework_job::Generation(1),
                    })
                    .expect("fixed numeric cancellation slot"),
            );
        }
        let first_saturated = saturated.first().expect("saturated lease").cancel_token();
        let last_saturated = saturated.last().expect("saturated lease").cancel_token();
        let generation = handle.scope_generation();
        handle.cancel_scope_generation();
        assert_eq!(handle.scope_generation(), generation.saturating_add(1));
        assert!(second_token.is_cancelled().await);
        assert!(first_saturated.is_cancelled().await);
        assert!(last_saturated.is_cancelled().await);
    }

    #[test]
    fn cancellation_numeric_authority_rejects_collision_capacity_and_contention_without_blocking() {
        let handle = ToolCancellationHandle::default();
        let key = |document: u32| ToolOperationKey {
            app_instance_id: document,
            document: ArtifactDocumentAuthority(document),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(1),
            generation: semio_framework_job::Generation(1),
        };
        let authority = handle.test_begin(key(100_000)).expect("arbitrary fixed-width authority");
        let collision = handle.test_begin(key(100_000 + TOOL_CANCELLATION_SLOTS as u32));
        assert_eq!(collision.err().expect("same direct slot must fail closed").code.0, "interactive-job.cancellation-collision");
        authority.finish();
        let recycled = handle.test_begin(key(100_000 + TOOL_CANCELLATION_SLOTS as u32)).expect("released direct slot");
        recycled.finish();

        let state = handle.state.try_lock().expect("test owns cancellation authority");
        let contended = handle.test_begin(key(77));
        assert_eq!(contended.err().expect("contention must not wait").code.0, "interactive-job.cancellation-busy");
        drop(state);
    }

    #[test]
    fn poisoned_cancellation_authority_fails_closed_without_recovery_or_waiting() {
        let handle = ToolCancellationHandle::default();
        let state = handle.state.clone();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _guard = state.lock().expect("test poison guard");
            panic!("poison fixed cancellation authority");
        }));
        let result = handle.test_begin(ToolOperationKey {
            app_instance_id: 9,
            document: ArtifactDocumentAuthority(9),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(1),
            generation: semio_framework_job::Generation(1),
        });
        assert_eq!(result.err().expect("poison must fail closed").code.0, "interactive-job.cancellation-busy");
    }

    #[semio_framework_async_macros::async_test]
    async fn segmented_download_remains_addressable_until_terminal_none_is_observed() {
        let mut app = contract_app_under_test().await;
        let chunks = ArtifactOutputChunks::new(4);
        assert_eq!(chunks.push(vec![1, 2, 3, 4]), Ok(4));
        assert_eq!(chunks.seal(), Ok(4));
        assert!(app.segmented_downloads.insert(91, ArtifactDownloadOutput::new("exact.bin", "application/octet-stream", None, chunks).expect("bounded segmented output")).is_ok());
        assert_eq!(app.take_segmented_download_chunk(91).await.expect("last chunk"), Some(vec![1, 2, 3, 4]));
        assert!(app.segmented_downloads.contains(91));
        assert_eq!(app.take_segmented_download_chunk(91).await.expect("terminal none"), None);
        assert!(!app.segmented_downloads.contains(91));
        assert_eq!(app.take_segmented_download_chunk(91).await.expect_err("terminal none removes authority").code.0, "interactive-job.unknown-segmented-download");
    }

    #[semio_framework_async_macros::async_test]
    async fn app_maintenance_reclaims_late_envelope_field_returns_before_close_terminal() {
        struct ReturnedDecoder {
            terminal: bool,
            drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }

        impl store::ArtifactEnvelopeFieldDecoder<TestSnapshot, TestMutation> for ReturnedDecoder {
            fn accept_field_token(
                &mut self,
                _field_id: u16,
                _token: store::OwnedSchemaToken,
                _terminal: bool,
                _source: &store::OwnedSchemaRecordCursor,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending)
            }

            fn finish_record(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending)
            }

            fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
                if maximum_items == 0 {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                self.terminal = true;
                Ok(store::SnapshotRetirementStep::Complete)
            }

            fn terminal_is_empty(&self) -> bool {
                self.terminal
            }
        }

        impl Drop for ReturnedDecoder {
            fn drop(&mut self) {
                self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }

        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let lease = app.envelope_field_decoders.try_admit(Box::new(ReturnedDecoder { terminal: false, drops: drops.clone() })).unwrap_or_else(|_| panic!("app decoder return registry admits one exact owner"));
        let ticket = lease.ticket();
        drop(lease);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);

        for _ in 0..4 {
            app.maintenance_stage = 10;
            let _ = PluginApp::maintenance_step(&mut app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("production maintenance pumps one returned decoder owner");
        }
        assert!(app.envelope_field_decoders.ticket_reclaimed(ticket));
        assert!(app.envelope_field_decoder_retirements.is_empty());
        assert!(app.envelope_field_decoders.terminal_is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(app.drive_envelope_field_decoder_returns(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, true).expect("close hierarchy observes exact terminal state"), PluginCloseStep::Complete);
    }

    #[semio_framework_async_macros::async_test]
    async fn app_maintenance_and_close_retain_completed_envelope_results_until_terminal_empty() {
        struct CompletedRecordSentinel {
            remaining: usize,
            terminal: bool,
            drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }

        impl store::ArtifactEnvelopeCompletedRecord<TestSnapshot, TestMutation> for CompletedRecordSentinel {
            fn try_publish_to(&mut self, _target: &mut dyn store::ArtifactEnvelopeCompletedRecordTarget<TestSnapshot, TestMutation>) -> bool {
                false
            }

            fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
                if maximum_items == 0 {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if self.remaining != 0 {
                    self.remaining -= 1;
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                }
                self.terminal = true;
                Ok(store::SnapshotRetirementStep::Complete)
            }

            fn terminal_is_empty(&self) -> bool {
                self.terminal
            }
        }

        impl Drop for CompletedRecordSentinel {
            fn drop(&mut self) {
                assert!(self.terminal);
                self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }

        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let owner: Box<dyn store::ArtifactEnvelopeCompletedRecord<TestSnapshot, TestMutation>> = Box::new(CompletedRecordSentinel { remaining: 2, terminal: false, drops: drops.clone() });
        let ticket = match app.envelope_completed_records.try_admit(owner) {
            Ok(ticket) => ticket,
            Err((_fault, mut owner)) => {
                while !owner.terminal_is_empty() {
                    let _ = owner.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("rejected completed owner bounded close");
                }
                panic!("app completed-record registry must admit one exact owner")
            }
        };
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        app.maintenance_stage = 11;
        assert_eq!(PluginApp::maintenance_step(&mut app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("ordinary maintenance preserves unconsumed completed output"), PluginCloseStep::Complete);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        app.envelope_completed_records.try_request_close(ticket).expect("cancelled consumer hands exact completed output to maintenance");
        for _ in 0..6 {
            app.maintenance_stage = 11;
            let _ = PluginApp::maintenance_step(&mut app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("production maintenance pumps one completed-record owner");
        }
        match app.envelope_completed_records.try_detach(ticket) {
            Err(fault) => assert_eq!(fault, store::ArtifactEnvelopeCompletedRecordFault::Stale),
            Ok(mut owner) => {
                while !owner.terminal_is_empty() {
                    let _ = owner.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("unexpected completed owner bounded close");
                }
                panic!("maintenance did not reclaim the exact completed record")
            }
        }
        assert!(app.envelope_completed_record_retirements.is_empty());
        assert!(app.envelope_completed_records.terminal_is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(app.drive_envelope_completed_record_returns(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, true).expect("close observes exact completed terminal state"), PluginCloseStep::Complete);
    }

    #[semio_framework_async_macros::async_test]
    async fn app_close_step_drains_at_most_one_segment_and_one_chunk_budget() {
        let mut app = contract_app_under_test().await;
        app.close_started = true;
        app.close_cancellation_cursor = TOOL_CANCELLATION_SLOTS;
        app.close_media_cursor = ARTIFACT_LIVE_OUTPUT_SLOTS;
        app.close_media_cleanup_cursor = ARTIFACT_LIVE_OUTPUT_SLOTS;
        let chunks = ArtifactOutputChunks::new(ARTIFACT_OUTPUT_CHUNK_BYTES * 2);
        assert_eq!(chunks.push(vec![1; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES));
        assert_eq!(chunks.push(vec![2; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES * 2));
        assert_eq!(chunks.seal(), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES * 2));
        assert!(app.segmented_downloads.insert(37, ArtifactDownloadOutput::new("close.bin", "application/octet-stream", None, chunks.clone()).expect("sealed close output")).is_ok());
        assert_eq!(app.close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES).expect("first close slice"), PluginCloseStep::Pending { released_items: 1, released_bytes: ARTIFACT_OUTPUT_CHUNK_BYTES });
        assert_eq!(chunks.chunks_remaining(), 1);
        assert_eq!(app.close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES).expect("second close slice"), PluginCloseStep::Pending { released_items: 1, released_bytes: ARTIFACT_OUTPUT_CHUNK_BYTES });
        assert_eq!(chunks.chunks_remaining(), 0);
        assert_eq!(app.close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES).expect("terminal close slice"), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        assert!(!app.segmented_downloads.contains(37));
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_close_final_destructor_is_constant_after_every_owned_field_is_drained() {
        let snapshot = std::sync::Arc::new(TestSnapshot::default());
        let config = std::sync::Arc::new(TestConfig::default());
        let history = std::sync::Arc::new(HistoryView::empty());
        let mut retirement = ArtifactCacheRetirement::<TestApp> { snapshot: Some(snapshot.clone()), config: Some(config.clone()), history: Some(history.clone()) };
        assert!(matches!(retirement.close_step(1), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
        assert_eq!(std::sync::Arc::strong_count(&history), 1);
        assert_eq!(std::sync::Arc::strong_count(&config), 2);
        assert_eq!(std::sync::Arc::strong_count(&snapshot), 2);
        assert!(matches!(retirement.close_step(1), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
        assert!(matches!(retirement.close_step(1), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
        assert_eq!(retirement.close_step(1), PluginCloseStep::Complete);
        assert!(retirement.terminal_is_empty());
        drop(retirement);
        assert_eq!(std::sync::Arc::strong_count(&snapshot), 1, "the empty retirement shell's destructor releases no nested owner");
    }

    #[test]
    fn retained_field_maximum_and_maximum_plus_one_are_language_neutral() {
        let mut exact = AppActionRegistry::test_with_controller_id("x".repeat(ARTIFACT_OUTPUT_CHUNK_BYTES));
        assert!(matches!(exact.close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES), PluginCloseStep::Pending { released_items: 1, released_bytes: ARTIFACT_OUTPUT_CHUNK_BYTES }));
        assert_eq!(exact.close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES), PluginCloseStep::Complete);
        assert!(exact.terminal_is_empty());

        let owner = "y".repeat(ARTIFACT_OUTPUT_CHUNK_BYTES + 1);
        let owner_pointer = owner.as_ptr();
        let mut plus_one = AppActionRegistry::test_with_controller_id(owner);
        assert!(matches!(plus_one.close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }));
        assert_eq!(plus_one.controller_id.as_ptr(), owner_pointer, "rejection hands back the exact retained backing owner");
        assert!(matches!(plus_one.close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES + 1), PluginCloseStep::Pending { released_items: 1, released_bytes } if released_bytes == ARTIFACT_OUTPUT_CHUNK_BYTES + 1));
        assert_eq!(plus_one.close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES), PluginCloseStep::Complete, "repeated close is idempotent after interruption and resume");
    }

    #[semio_framework_async_macros::async_test]
    async fn shared_retained_command_checkpoint_resumes_exact_cursor_and_cancels_with_bounded_close() {
        let bus = ActionBus::new();
        bus.register(TestRetainedCommandFactory::new()).expect("test retained command factory");
        let command = TestCommand::SetLabel { value: "wire".into() };
        let wire = <TestCommand as protocol::OpBinary>::encode_op(&command).expect("test retained command wire");
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(41), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 5);
        let original_completion = ArtifactToolCompletion::<TestApp>::new();
        let original_consumer = original_completion.clone();
        let original_payload = test_retained_command_payload(original_completion).await;
        let original_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, original_payload, operation);
        let (original_admission, original_input) = test_retained_wire_input(&bus, &wire);
        let mut original = match bus.dispatch_wire_retained_with_spec(&original_admission, original_input, None, original_spec) {
            Ok(dispatch) => dispatch,
            Err(_) => panic!("test retained command initial dispatch"),
        };
        let mut checkpoint_bytes = None;
        let mut sequence = 0;
        for _ in 0..64 {
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            match original.job.step(&mut context) {
                semio_framework_job::StepOutcome::PreviewReady(mut payload) => test_close_retained_payload(&mut payload),
                semio_framework_job::StepOutcome::CheckpointReady(mut checkpoint) => {
                    let bytes = checkpoint.state.single_page().expect("single ARC1 checkpoint page");
                    if bytes.get(4) == Some(&3) && bytes.get(5) == Some(&1) && bytes.get(48) == Some(&1) {
                        checkpoint_bytes = Some(bytes.to_vec());
                    }
                    test_close_retained_payload(&mut checkpoint.state);
                    if checkpoint_bytes.is_some() {
                        break;
                    }
                }
                semio_framework_job::StepOutcome::Yield => {}
                semio_framework_job::StepOutcome::Cancelled => panic!("initial retained command cancelled"),
                semio_framework_job::StepOutcome::Complete(_) => panic!("initial retained command completed before its work checkpoint"),
                semio_framework_job::StepOutcome::Fault(mut fault) => {
                    test_close_retained_payload(&mut fault.detail);
                    panic!("initial retained command faulted");
                }
            }
        }
        let checkpoint_bytes = checkpoint_bytes.expect("work checkpoint after one custom cursor step");
        original.job.begin_close();
        for _ in 0..64 {
            if original.job.terminal_is_empty() {
                break;
            }
            let _ = original.job.close_step(1, usize::MAX);
        }
        assert!(original.job.terminal_is_empty(), "interrupted original retires every retained owner incrementally");
        assert!(original_consumer.take_emit().expect("original completion cell").is_none(), "interrupted original never publishes a result");

        TEST_RETAINED_COMMAND_STEP_CALLS.store(0, std::sync::atomic::Ordering::SeqCst);
        let resumed_completion = ArtifactToolCompletion::<TestApp>::new();
        let resumed_consumer = resumed_completion.clone();
        let resumed_payload = test_retained_command_payload(resumed_completion).await;
        let resumed_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, resumed_payload, operation);
        let (resumed_admission, resumed_input) = test_retained_wire_input(&bus, &wire);
        let (_, resumed_checkpoint) = test_retained_wire_input(&bus, &checkpoint_bytes);
        let mut resumed = match bus.dispatch_wire_retained_with_spec(&resumed_admission, resumed_input, Some(resumed_checkpoint), resumed_spec) {
            Ok(dispatch) => dispatch,
            Err(_) => panic!("test retained command resume dispatch"),
        };
        let mut completed = false;
        for _ in 0..96 {
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            match resumed.job.step(&mut context) {
                semio_framework_job::StepOutcome::PreviewReady(mut payload) => test_close_retained_payload(&mut payload),
                semio_framework_job::StepOutcome::CheckpointReady(mut checkpoint) => test_close_retained_payload(&mut checkpoint.state),
                semio_framework_job::StepOutcome::Yield => {}
                semio_framework_job::StepOutcome::Complete(mut candidate) => {
                    test_close_retained_payload(&mut candidate.state);
                    test_close_retained_payload(&mut candidate.output);
                    completed = true;
                    break;
                }
                semio_framework_job::StepOutcome::Cancelled => panic!("resumed retained command cancelled"),
                semio_framework_job::StepOutcome::Fault(mut fault) => {
                    test_close_retained_payload(&mut fault.detail);
                    panic!("resumed retained command faulted");
                }
            }
        }
        assert!(completed, "resumed retained command reaches its commit boundary");
        assert_eq!(TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 2, "resume continues after cursor one instead of replaying completed custom work");
        let (result, _) = resumed_consumer.take_emit().expect("resumed completion cell").expect("resumed completion value");
        assert_eq!(result.expect("resumed command emit").artifact_mutations, vec![TestMutation::SetLabel(SetLabel { value: "resumed".into() })]);
        resumed.job.begin_close();
        for _ in 0..64 {
            if resumed.job.terminal_is_empty() {
                break;
            }
            let _ = resumed.job.close_step(1, usize::MAX);
        }
        assert!(resumed.job.terminal_is_empty(), "completed resume retires every retained owner incrementally");

        let cancelled_completion = ArtifactToolCompletion::<TestApp>::new();
        let cancelled_consumer = cancelled_completion.clone();
        let cancelled_payload = test_retained_command_payload(cancelled_completion).await;
        let cancelled_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, cancelled_payload, operation);
        let (cancelled_admission, cancelled_input) = test_retained_wire_input(&bus, &wire);
        let mut cancelled = match bus.dispatch_wire_retained_with_spec(&cancelled_admission, cancelled_input, None, cancelled_spec) {
            Ok(dispatch) => dispatch,
            Err(_) => panic!("test retained command cancellation dispatch"),
        };
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, || Some(0), &mut sequence);
        assert!(matches!(cancelled.job.step(&mut context), semio_framework_job::StepOutcome::Cancelled));
        cancelled.job.begin_close();
        for _ in 0..64 {
            if cancelled.job.terminal_is_empty() {
                break;
            }
            let _ = cancelled.job.close_step(1, usize::MAX);
        }
        assert!(cancelled.job.terminal_is_empty(), "cancelled retained command retires every owner incrementally");
        assert!(cancelled_consumer.take_emit().expect("cancelled completion cell").is_none(), "cancelled retained command never publishes a result");
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_command_child_emit_prepublication_close_and_rejected_handoff_are_bounded() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧵️retained-command/🧫️fixtures/🧩️child-prepublication-close.json")).expect("language-neutral child close fixture");
        let expected_ids = fixture["children"].as_array().expect("child close fixture rows").iter().map(|child| child["childId"].as_str().unwrap().to_string()).collect::<Vec<_>>();
        let expected_order = fixture["expectedRetirementOrder"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>();
        let build_emit = || {
            let children = fixture["children"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(index, child)| {
                    let emitted = ChildEmit::of::<TestSnapshot, _>(child["slot"].as_str().unwrap(), child["childId"].as_str().unwrap(), &[TestMutation::SetCount(SetCount { value: index as i32 + 1 })]);
                    assert!(emitted.ops.iter().all(|op| !op.is_empty()), "real composite mutation owns nonempty encoded bytes");
                    emitted
                })
                .collect();
            Emit { artifact_mutations: vec![TestMutation::SetCount(SetCount { value: 7 })], child_emits: children, ..Default::default() }
        };
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(42), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 5);
        let drive_to_publish = |job: &mut crate::retained_command::ArtifactRetainedCommandJob<TestApp>, sequence: &mut u64| {
            for expected_stage in ["preflight", "work"] {
                let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), sequence);
                match job.step(&mut context) {
                    semio_framework_job::StepOutcome::PreviewReady(mut payload) => test_close_retained_payload(&mut payload),
                    _ => panic!("retained child job failed at {expected_stage}"),
                }
            }
        };
        let close_and_observe = |job: &mut crate::retained_command::ArtifactRetainedCommandJob<TestApp>| {
            let initial = job.test_pending_emit_shape().expect("pre-publication emit owner");
            assert_eq!(initial, (2, 1, expected_ids.clone()));
            job.begin_close();
            assert_eq!(job.close_step(0, fixture["maximumBytes"].as_u64().unwrap() as usize), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!(job.test_pending_emit_shape(), Some(initial.clone()), "zero item grant preserves the exact emit owner");
            assert_eq!(job.close_step(1, 0), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }, "empty reserved raw allocation retires before produced output");
            assert_eq!(job.test_pending_emit_shape(), Some(initial.clone()), "raw allocation retirement preserves the exact emit owner");
            assert_eq!(job.close_step(1, 0), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!(job.test_pending_emit_shape(), Some(initial), "zero byte grant preserves the exact child payload");
            let mut retired = Vec::new();
            let mut prior_children = expected_ids.len();
            for _ in 0..10_000 {
                let step = job.close_step(fixture["maximumItems"].as_u64().unwrap() as usize, fixture["maximumBytes"].as_u64().unwrap() as usize);
                if let semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } = step {
                    assert!(released_items <= 1 && released_bytes <= 4);
                }
                if let Some((children, parents, _)) = job.test_pending_emit_shape() {
                    assert_eq!(parents, 1, "parent lane remains owned until every child is terminal");
                    if children < prior_children {
                        retired.push(fixture["children"][prior_children - 1]["id"].as_str().unwrap());
                        prior_children = children;
                    }
                } else {
                    break;
                }
            }
            assert_eq!(retired, expected_order);
            for _ in 0..128 {
                if job.terminal_is_empty() {
                    break;
                }
                let _ = job.close_step(1, 4_096);
            }
            assert!(job.terminal_is_empty());
        };

        let direct_completion = ArtifactToolCompletion::<TestApp>::new();
        let _direct_consumer = direct_completion.clone();
        let mut direct = crate::retained_command::ArtifactRetainedCommandJob::new(test_retained_child_command_payload(direct_completion, build_emit()).await);
        let mut sequence = 0;
        drive_to_publish(&mut direct, &mut sequence);
        close_and_observe(&mut direct);

        let duplicate_completion = ArtifactToolCompletion::<TestApp>::new();
        let duplicate_consumer = duplicate_completion.clone();
        duplicate_completion.complete(Ok(Emit::default()), EphemeralEmit::default()).expect("prefill completion once");
        let mut duplicate = crate::retained_command::ArtifactRetainedCommandJob::new(test_retained_child_command_payload(duplicate_completion, build_emit()).await);
        drive_to_publish(&mut duplicate, &mut sequence);
        let mut duplicate_context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match duplicate.step(&mut duplicate_context) {
            semio_framework_job::StepOutcome::Fault(mut fault) => test_close_retained_payload(&mut fault.detail),
            _ => panic!("duplicate completion must reject the handoff"),
        }
        assert!(duplicate.test_pending_emit_shape().is_some(), "duplicate rejection returns the exact emit owner");
        close_and_observe(&mut duplicate);
        let (existing, _) = duplicate_consumer.take_emit().expect("prefilled completion remains readable").expect("prefilled completion remains assigned");
        assert!(existing.expect("prefilled completion remains successful").child_emits.is_empty());

        let busy_completion = ArtifactToolCompletion::<TestApp>::new();
        let busy_consumer = busy_completion.clone();
        let mut busy = crate::retained_command::ArtifactRetainedCommandJob::new(test_retained_child_command_payload(busy_completion.clone(), build_emit()).await);
        drive_to_publish(&mut busy, &mut sequence);
        busy_completion.with_busy_test_lock(|| {
            let mut busy_context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            match busy.step(&mut busy_context) {
                semio_framework_job::StepOutcome::Fault(mut fault) => test_close_retained_payload(&mut fault.detail),
                _ => panic!("busy completion must reject the handoff"),
            }
        });
        assert!(busy.test_pending_emit_shape().is_some(), "busy rejection returns the exact emit owner");
        close_and_observe(&mut busy);
        assert!(busy_consumer.take_emit().expect("busy completion remains readable").is_none(), "busy completion remains untouched");
    }

    #[semio_framework_async_macros::async_test]
    async fn unproved_command_fails_before_an_overrun_reducer_can_start() {
        let mut app = contract_app_under_test().await;
        let error = app.dispatch_typed(TestCommand::WatchdogOverrun, &meta()).await.expect_err("an unproved operation must fail before an over-budget reducer starts");
        assert_eq!(error.code.0, "interactive-job.missing-factory");
    }

    /// 🧪️ A registry declaring one `HierarchyProvider::Topology` interaction domain ("items", see
    /// `TestApp::interaction_topology`) — the `🕹️InteractionDispatch` fixture. Auto-injects the six
    /// framework interaction actions via `interaction_action_definitions` (verified by
    /// `build_definition_carries_window_interactions_and_injects_framework_actions` above).
    async fn interaction_registry() -> AppActionRegistry {
        let app = App::from_builder(
            App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
                .await
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await
                .interaction(InteractionDefinition {
                    id: "items".into(),
                    label: LocalizedLabel::data("Items"),
                    granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
                    hierarchy: HierarchyProvider::Topology,
                    hover: HoverSpec::default(),
                    selection: SelectionSpec {
                        modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                        methods: vec![SelectionMethod::Pick],
                        merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                        transitive: false,
                        broadcast: true,
                    },
                })
                .await
                .window_kind_interactions("main", vec![InteractionRef::new("items")])
                .await,
        )
        .await;
        AppActionRegistry::from_definition(&app.definition)
    }

    async fn interaction_app_under_test() -> VcsArtifactApp<TestApp> {
        VcsArtifactApp::with_registry(TestApp::<false>::default(), interaction_registry().await).await
    }

    /// 🧪️ Builds the JSON `args` an `interactionSelect`/`interactionHover` dispatch carries — the
    /// `targets` arg is itself a JSON-encoded string (an `ActionArgDef::text`, matching
    /// `interaction_action_definitions`'s declaration), not a nested JSON array.
    fn interaction_target_args(extra: Value, id: &str) -> DslValue {
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "item".into(), id: id.into() }]).expect("targets serialize");
        let mut object = DslValue::from(&extra);
        if let DslValue::Object(entries) = &mut object {
            entries.retain(|(key, _)| key != "targets");
            entries.push(("targets".to_string(), DslValue::String(targets)));
        }
        object
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs"));

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../⚛️reactor/🔄️turn/🧪️tests/📏️future-size/🦀️.rs"));

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../⚛️reactor/🔄️turn/🧪️tests/📄️command-page-authority/🦀️.rs"));

    async fn __semio_plugin_bundle() -> Result<Plugin<TestRuntimeApps>, PluginAssemblyError> {
        Plugin::<TestRuntimeApps>::builder("test").label("Synthetic").version("0.0.1").package_id("semio:test").document_app::<TestApp>(synthetic_play_app().await).document_app_mutation_roster::<TestApp>().try_build()
    }

    #[semio_framework_async_macros::async_test]
    async fn plugin_builder_builds_bundle_from_fluent_spec() {
        let bundle = __semio_plugin_bundle().await.expect("synthetic plugin assembly");
        assert_eq!(bundle.manifest.plugin_id, "test");
        assert_eq!(bundle.manifest.label.as_str(), "Synthetic");
        assert_eq!(bundle.manifest.version, "0.0.1");
        assert!(bundle.manifest.apps.iter().any(|app| app.id == TestApp::<false>::APP_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn test_app_id_matches_its_own_dialect() {
        assert_eq!(TestApp::<false>::APP_ID, test_app_surface_id().await, "TestApp::<false>::APP_ID must not drift from TEST_APP_DIALECT (contract §1)");
    }

    #[semio_framework_async_macros::async_test]
    async fn plugin_builder_wires_app_factory_for_create_app() {
        let bundle = __semio_plugin_bundle().await.expect("synthetic plugin assembly");
        let app = bundle.create_app(TestApp::<false>::APP_ID).expect("registered app");
        assert_eq!(app.app_id().await, TestApp::<false>::APP_ID);
        assert!(bundle.create_app("unknown-app").is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let before_snapshot = app.test_snapshot().await;
        let before_edits = app.test_store().await.applied_edit_ids().len();

        let set = crate::plugin_runtime::set_merge_policy_frames(&mut app, 7, protocol::MergePolicy::Vigilant.as_u8()).await.expect("known policy accepts");
        assert_eq!(set, vec![protocol::AppFrame::Done { in_reply_to: 7 }]);
        assert_eq!(app.dispatch_report().await.policy, protocol::MergePolicy::Vigilant);
        assert_eq!(app.test_snapshot().await, before_snapshot, "local policy changes no document state");
        assert_eq!(app.test_store().await.applied_edit_ids().len(), before_edits, "local policy changes no shared history");

        let invalid_policy = crate::plugin_runtime::set_merge_policy_frames(&mut app, 8, 3).await.expect_err("unknown policy ordinal rejects");
        assert_eq!(invalid_policy.code.0, "merge.invalid-policy");
        assert_eq!(app.dispatch_report().await.policy, protocol::MergePolicy::Vigilant, "invalid policy never changes the active policy");

        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("seed one durable edit");
        let (mut envelope, applied_edit_ids) = {
            let store = app.test_store().await;
            let files = store::print_document_pack(store.envelope()).await.expect("print retained test envelope");
            (store::parse_document_pack::<TestSnapshot, TestMutation>(&files.pack, &files.spr).await.expect("parse retained test envelope").envelope, store.applied_edit_ids().to_vec())
        };
        let kind = protocol::ConflictKind::Degraded { edit_ids: applied_edit_ids.clone() };
        let timestamp = HybridLogicalTimestamp::new(1, u64::MAX);
        let mutation_ids: Vec<MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.clone())).collect();
        let conflict_id = protocol::ConflictId::new(&kind, &ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).await;
        envelope.conflicts.push(protocol::Conflict {
            id: conflict_id.clone(),
            kind,
            status: protocol::ConflictStatus::Open,
            messages: vec![protocol::MutationMessage::warn("mutation.partial", "test conflict").at_op(0)],
            actors: vec![ActorId("local".into())],
            timestamp,
        });
        app.test_store_mut().await.reset(envelope, applied_edit_ids, Vec::new()).await.expect("seed valid open degraded conflict");

        let read = crate::plugin_runtime::read_conflicts_frames(&app, 9).await;
        let [protocol::AppFrame::Conflicts { in_reply_to: Some(9), conflicts }] = read.as_slice() else { panic!("read must yield exactly one correlated conflict projection") };
        let open: Vec<protocol::Conflict> = crate::plugin_runtime::decode_wire_serialized(conflicts).await.expect("conflict payload decodes canonically");
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].id, conflict_id);
        assert_eq!(open[0].status, protocol::ConflictStatus::Open);

        let invalid_resolution = crate::plugin_runtime::resolve_conflict_frames(&mut app, 10, conflict_id.0.clone(), 2).await.expect_err("unknown resolution ordinal rejects");
        assert_eq!(invalid_resolution.code.0, "merge.invalid-resolution");
        assert_eq!(app.open_conflicts().await.len(), 1, "invalid resolution never changes conflict state");

        let resolved = crate::plugin_runtime::resolve_conflict_frames(&mut app, 11, conflict_id.0.clone(), 0).await.expect("accept resolves the open degraded conflict");
        let [protocol::AppFrame::MergeReport { in_reply_to: Some(11), report }, protocol::AppFrame::Conflicts { in_reply_to: Some(11), conflicts }] = resolved.as_slice() else {
            panic!("resolve must emit correlated merge report then open-conflict projection")
        };
        let report: protocol::MergeReport = crate::plugin_runtime::decode_wire_serialized(report).await.expect("merge report payload decodes canonically");
        assert!(report.accepted);
        assert_eq!(report.conflict.as_ref(), Some(&conflict_id));
        let open: Vec<protocol::Conflict> = crate::plugin_runtime::decode_wire_serialized(conflicts).await.expect("post-resolution conflict payload decodes canonically");
        assert!(open.is_empty(), "accepted conflict leaves the open projection");
        assert!(app.open_conflicts().await.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn operation_action_emits_kernel_op_with_true_inverse() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 1 })).unwrap());
        assert_eq!(result.mutations[0].inverse.inverse_diff.payload, protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 0 })).unwrap()]));
        assert_eq!(result.inverse_group.mutations.len(), 1);
        assert_eq!(app.test_snapshot().await.count, 1);
    }

    //#region 🔖️EphemeralLaneTests
    #[semio_framework_async_macros::async_test]
    async fn a_command_reaches_both_ephemeral_lanes_without_touching_history() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        assert_eq!(app.presence_store.generation().await, 0);
        assert_eq!(app.transient_store.generation().await, 0);

        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");

        assert_eq!(app.presence_store.generation().await, 1, "presence lane never received the command's emission");
        assert_eq!(app.transient_store.generation().await, 1, "transient lane never received the command's emission");
        assert_eq!(
            app.ephemeral_snapshot().await,
            EphemeralSnapshot { presence: PublicationPresence { revision: 1 }.encode_pack(), presence_generation: 1, transient_generation: 1, interaction: Vec::new() },
            "object-safe channel snapshot must carry the typed presence pack, both generations, and (declaring no interaction domain) empty interaction bytes"
        );

        // 🧾️ Neither ephemeral lane may appear in history: they have no edits, no undo, and no
        // command-log rows of their own — the document's single edit is the only thing recorded.
        assert_eq!(app.test_store().await.envelope().vcs.edits.len(), 1, "an ephemeral lane leaked into the document's edit log");

        // ↩️ Undo rolls back the DOCUMENT; the ephemeral lanes are not restored, because they
        // were never part of the undoable gesture in the first place.
        app.dispatch_action("undo", None, &meta()).await.expect("undo");
        assert_eq!(app.test_snapshot().await.count, 0);
        assert_eq!(app.presence_store.generation().await, 1, "undo must not rewind presence");
        assert_eq!(app.transient_store.generation().await, 1, "undo must not rewind transient");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_command_that_emits_nothing_ephemeral_leaves_both_lanes_untouched() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "x".into() }, &meta()).await.expect("set label");
        assert_eq!(app.presence_store.generation().await, 0);
        assert_eq!(app.transient_store.generation().await, 0);
    }

    /// 👥️ Contract-freeze §C7.6, "zero app-side code": an app that merely DECLARES an interaction
    /// domain with `broadcast: true` (nothing app-specific) must see its live selection show up in
    /// `ephemeral_snapshot().interaction` — assembled purely from `AppDefinition.interactions` plus
    /// the framework-owned `interaction_store`/`interaction_hover` state.
    #[semio_framework_async_macros::async_test]
    async fn ephemeral_snapshot_carries_encoded_interaction_from_declared_broadcast_specs() {
        let mut app = interaction_app_under_test().await;
        // 🧪️ `interaction_topology`'s fixture only knows "item-1" once `doc.snapshot.label` is
        // non-empty (see that fn's own doc comment) — without this seed, `validate_state` prunes
        // the pick as an unknown id and the domain never shows up in `interaction_state()`.
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta()).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))).await;

        let snapshot = app.ephemeral_snapshot().await;
        assert!(!snapshot.interaction.is_empty(), "a broadcasting domain with a live selection must not encode to empty bytes");
        let mut pos = 0usize;
        let decoded = protocol::decode_presence_interaction(&snapshot.interaction, &mut pos).await.expect("interaction bytes decode");
        assert_eq!(decoded.app_id, app.app_id().await);
        assert_eq!(decoded.domains.len(), 1);
        assert_eq!(decoded.domains[0].domain, "items");
        assert_eq!(decoded.domains[0].selected, vec!["item-1".to_string()]);
    }

    /// 🧵️ Every world/graph pick is one framework-reserved `interactionSelect` dispatch, and it must fit
    /// a bounded thread stack. `dispatch_framework_reserved_action` used to spell all nineteen reserved
    /// routes as separate `run_framework_reserved_job` awaits inside ONE generator; an unoptimized build
    /// gives each such await its own non-overlapping slot, so that single resume frame reserved 1.80 MiB
    /// and every pick aborted the process with `fatal runtime error: stack overflow` on the 2 MiB stack a
    /// plain OS thread gets (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️selection-overflow-2026-09-09.md`). 2 MiB is exactly that default thread; the whole
    /// construct-select-close lifecycle measures ~1.75 MiB against it today, and measured ≥3.3 MiB
    /// before the split.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn one_framework_reserved_route_fits_a_bounded_thread_stack() {
        std::thread::Builder::new()
            .stack_size(2 * 1024 * 1024)
            .spawn(|| {
                semio_framework_async::block_on(async {
                    let mut app = interaction_app_under_test().await;
                    reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))).await;
                    testkit::close_registered_fixture_app(&mut app);
                });
            })
            .expect("bounded-stack worker")
            .join()
            .expect("one interactionSelect must fit a default 2 MiB thread stack");
    }
    //#endregion 🔖️EphemeralLaneTests

    //#region 🔖️AdoptPresenceTests
    fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> PresencePeer {
        PresencePeer {
            actor: actor.to_string(),
            connected_at_ms: 1,
            label: None,
            presence_pack: with_pack.then(|| PublicationPresence::default().encode_pack()),
            user_id: None,
            role: None,
            drag_ghost_json: None,
            interaction: None,
            color,
            surface: None,
            views: Vec::new(),
            ui: None,
        }
    }

    async fn publish_presence_roster(app: &mut VcsArtifactApp<TestApp>, seq: u64, own_color: Option<u8>, peers: &[PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
        let roster = peers.iter().map(|peer| resolve_ready(encode_presence_peer(peer))).collect::<Vec<_>>();
        let admission = app.reserve_presence_ingress(seq).expect("reserve roster before decode");
        let first = roster.iter().next().map(|bytes| bytes.to_vec()).unwrap_or_default();
        let cursor = protocol::PresenceCommandCursor::admit_page(seq, own_color, roster.len() as u32, FixedCommandPage::try_copy_from(&first).expect("test peer page is fixed-authority")).map_err(|(error, _)| error).expect("admit first roster page");
        let generation = admission.generation();
        app.admit_presence_ingress(admission, cursor, now_ms);
        let mut next_page = 1usize;
        for _ in 0..512 {
            app.maintenance_stage = 7;
            let _ = PluginApp::maintenance_step(app, 1, 4096).expect("bounded roster step");
            if next_page < roster.len() {
                let page = FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster page")).expect("test peer page is fixed-authority");
                match app.push_presence_ingress(generation, next_page as u32, page) {
                    Ok(()) => next_page += 1,
                    Err((_fault, _page)) => {}
                }
            }
            if let Some(outcome) = app.take_presence_outcome() {
                return outcome;
            }
        }
        panic!("retained roster did not reach an observable terminal outcome");
    }

    /// 👥️ Contract-freeze §C7.6: `adopt_presence` (1) adopts an app-typed `presence_pack` into
    /// `presence_store` ONLY when one is present, (2) unconditionally upserts `color`/`surface`/
    /// `interaction` into `peer_presence` for every peer in the roster, and (3) treats the roster
    /// as the single source of truth — a peer absent from a later call is dropped from BOTH maps.
    #[semio_framework_async_macros::async_test]
    async fn retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let alice = sample_presence_peer("user:alice#s1", Some(3), true);
        let bob = sample_presence_peer("user:bob#s1", Some(5), false);

        assert!(publish_presence_roster(&mut app, 1, Some(9), &[alice.clone(), bob], 1000).await.fault.is_none());
        assert_eq!(app.own_color, Some(9));
        let typed_root = app.presence_store.peers_root();
        assert_eq!(typed_root.peers().count(), 1, "only the peer carrying a presence_pack adopts into presence_store");
        assert_eq!(typed_root.peers().next().unwrap().0, "user:alice#s1");
        assert_eq!(app.peer_presence.len(), 2, "both peers upsert into peer_presence regardless of presence_pack");
        assert_eq!(app.peer_presence.get("user:alice#s1").unwrap().color, Some(3));
        assert_eq!(app.peer_presence.get("user:bob#s1").unwrap().color, Some(5));

        // 👋 bob leaves the roster — a second call carrying only alice must drop bob from BOTH maps.
        assert!(publish_presence_roster(&mut app, 2, Some(9), &[alice], 2000).await.fault.is_none());
        assert_eq!(app.presence_store.peers_root().peers().count(), 1);
        assert_eq!(app.peer_presence.len(), 1);
        assert!(app.peer_presence.contains_key("user:alice#s1"));
        assert!(!app.peer_presence.contains_key("user:bob#s1"), "an actor absent from the roster must be dropped, not left stale");
    }

    #[semio_framework_async_macros::async_test]
    async fn peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let alice = sample_presence_peer("user:alice#s1", Some(3), true);
        let bob = sample_presence_peer("user:bob#s1", Some(5), false);
        assert!(publish_presence_roster(&mut app, 1, Some(9), &[alice.clone(), bob], 1000).await.fault.is_none());
        let captured = std::sync::Arc::clone(&app.peer_presence);
        let typed_captured = app.presence_store.peers_root();
        assert!(std::sync::Arc::ptr_eq(&captured, &app.peer_presence), "operation capture clones one peer root Arc");

        assert!(publish_presence_roster(&mut app, 2, Some(9), &[alice], 2000).await.fault.is_none());
        assert_eq!(captured.len(), 2, "captured roster remains revision-stable");
        assert_eq!(app.peer_presence.len(), 1, "live roster advances independently");
        app.maintenance_stage = 5;
        assert!(matches!(PluginApp::maintenance_step(&mut app, 1, 4096).expect("shared old root blocks"), PluginCloseStep::Blocked { .. }));
        app.maintenance_stage = 6;
        let _ = PluginApp::maintenance_step(&mut app, 1, 4096).expect("app-typed retirement selects its displaced entry");
        app.maintenance_stage = 6;
        assert!(matches!(PluginApp::maintenance_step(&mut app, 1, 4096).expect("captured app-typed peer blocks"), PluginCloseStep::Blocked { .. }));

        drop(captured);
        drop(typed_captured);
        for _ in 0..16 {
            if app.peer_presence_retirements.is_empty() {
                break;
            }
            app.maintenance_stage = 5;
            let _ = PluginApp::maintenance_step(&mut app, 1, 4096).expect("bounded peer root retirement progresses");
        }
        assert!(app.peer_presence_retirements.is_empty(), "old peer root reaches terminal-empty without a whole-roster drop");
        for _ in 0..16 {
            if app.presence_peer_retirements.is_empty() {
                break;
            }
            app.maintenance_stage = 6;
            let _ = PluginApp::maintenance_step(&mut app, 1, 4096).expect("bounded app-typed peer retirement progresses");
        }
        assert!(app.presence_peer_retirements.is_empty(), "displaced app-typed peer reaches its domain-owned terminal witness");
    }

    #[semio_framework_async_macros::async_test]
    async fn peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority() {
        let mut saturated = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        for seq in 0..ARTIFACT_LIVE_OUTPUT_SLOTS as u64 {
            let admission = saturated.reserve_presence_ingress(seq).expect("fixed roster slot admits before decode");
            let cursor = protocol::PresenceCommandCursor::admit_page(seq, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("empty roster cursor");
            saturated.admit_presence_ingress(admission, cursor, 0);
        }
        let saturation = match saturated.reserve_presence_ingress(ARTIFACT_LIVE_OUTPUT_SLOTS as u64) {
            Ok(_) => panic!("maximum plus one must fail before payload decode"),
            Err(fault) => fault,
        };
        assert_eq!(saturation.code.0, "interactive-job.peer-roster-saturated");
        for _ in 0..ARTIFACT_LIVE_OUTPUT_SLOTS * 16 {
            saturated.maintenance_stage = 7;
            let _ = PluginApp::maintenance_step(&mut saturated, 1, PEER_ROSTER_WIRE_BYTES).expect("one bounded saturation cleanup turn");
            if saturated.peer_roster_publications.is_empty() {
                break;
            }
        }
        assert!(saturated.peer_roster_publications.is_empty(), "every admitted publication reaches its exact outcome slot");
        for _ in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            assert!(saturated.take_presence_outcome().is_some(), "every admitted roster has one ordered outcome");
        }

        let mut cancelled = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let admission = cancelled.reserve_presence_ingress(1).expect("cancel roster admission");
        let cancel = admission.cancel.clone();
        let page = FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
        let cursor = protocol::PresenceCommandCursor::admit_page(1, None, 1, page).map_err(|(error, _)| error).expect("cancel cursor");
        let generation = admission.generation();
        cancelled.admit_presence_ingress(admission, cursor, 0);
        cancel.cancel_now();
        cancelled.maintenance_stage = 7;
        assert_eq!(PluginApp::maintenance_step(&mut cancelled, 1, 16).expect("sub-page close grant"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }, "interrupted close must retain the exact page when the byte grant is one short");
        assert!(cancelled.peer_roster_publications.get(generation).is_some(), "cancelled publication stays mounted until its retained page closes");
        for _ in 0..64 {
            cancelled.maintenance_stage = 7;
            let _ = PluginApp::maintenance_step(&mut cancelled, 1, 17).expect("bounded cancelled roster close");
            if cancelled.peer_roster_publications.is_empty() {
                break;
            }
        }
        let outcome = cancelled.take_presence_outcome().expect("cancelled roster outcome");
        assert_eq!(outcome.fault.expect("cancel is observable").code.0, "interactive-job.peer-roster-cancelled");
        assert!(cancelled.peer_presence.is_empty(), "cancelled roster never publishes metadata");
        assert_eq!(cancelled.presence_store.peers_root().len(), 0, "cancelled roster never publishes app-typed presence");

        let mut stale = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let admission = stale.reserve_presence_ingress(9).expect("stale roster admission");
        let cursor = protocol::PresenceCommandCursor::admit_page(9, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("stale empty roster cursor");
        stale.admit_presence_ingress(admission, cursor, 0);
        stale.peer_roster_processed_generation = 1;
        for _ in 0..64 {
            stale.maintenance_stage = 7;
            let _ = PluginApp::maintenance_step(&mut stale, 1, PEER_ROSTER_WIRE_BYTES).expect("bounded stale roster close");
            if stale.peer_roster_publications.is_empty() {
                break;
            }
        }
        let outcome = stale.take_presence_outcome().expect("stale roster outcome");
        assert_eq!(outcome.fault.expect("stale generation is observable").code.0, "interactive-job.peer-roster-publication-authority");
        assert!(stale.peer_presence.is_empty(), "stale roster never changes the live peer root");
        assert_eq!(stale.presence_store.peers_root().len(), 0, "stale roster never changes the typed peer root");
    }
    //#endregion 🔖️AdoptPresenceTests

    //#region 🔖️InteractionViewPeersTests
    #[semio_framework_async_macros::async_test]
    async fn interaction_view_peers_selecting_returns_actor_and_color() {
        let mut peers = PeerPresenceRoot::empty();
        let mark_interaction = |selected: &[&str], hovered: &[&str]| {
            Some(PresenceInteraction {
                app_id: "draw".to_string(),
                domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.iter().map(|id| id.to_string()).collect(), hovered: hovered.iter().map(|id| id.to_string()).collect() }],
            })
        };
        peers.insert("user:zed#s1".to_string(), PeerPresence { color: Some(7), surface: None, interaction: mark_interaction(&["item-1"], &[]) }).expect("zed peer");
        peers.insert("user:alice#s1".to_string(), PeerPresence { color: Some(2), surface: None, interaction: mark_interaction(&["item-1"], &[]) }).expect("alice peer");
        peers.insert("user:bob#s1".to_string(), PeerPresence { color: Some(4), surface: None, interaction: mark_interaction(&[], &["item-1"]) }).expect("bob peer");

        let state = InteractionState::default();
        let hover = InteractionHoverState::new();
        let view = InteractionView { state: &state, hover: &hover, peers: &peers };

        let selecting = view.peers_selecting("items", "item-1");
        assert_eq!(selecting.len(), 2, "only alice and zed selected item-1");
        assert_eq!(selecting[0].actor, "user:alice#s1", "sorted by actor");
        assert_eq!(selecting[0].color, Some(2));
        assert_eq!(selecting[1].actor, "user:zed#s1");
        assert_eq!(selecting[1].color, Some(7));
        assert!(view.peers_selecting("items", "item-2").is_empty(), "no peer selected item-2");

        let hovering = view.peers_hovering("items", "item-1");
        assert_eq!(hovering.len(), 1);
        assert_eq!(hovering[0].actor, "user:bob#s1");
        assert_eq!(hovering[0].color, Some(4));
    }
    //#endregion 🔖️InteractionViewPeersTests

    //#region 🔖️CompositionTests
    store::space_members! {
        pub enum TestMembers, TestMembersOpen {
            Child("s.test.child", "native", "*", "semio.test/v1") => (TestSnapshot, TestMutation),
        }
    }

    /// 🧪️ A live child `ArtifactStore<TestSnapshot, TestMutation>`, wrapped as `TestMembers` —
    /// the shape `VcsArtifactApp::register_child`/`open_child` expect. Built directly (no
    /// runtime `ChildStoreFactory`/`MemberFactory` registration needed — `TestMembers::open`
    /// dispatches by kind at compile time) — the SAME `TestSnapshot`/`TestMutation` pair
    /// `TestApp` itself uses, so a "child" here is just a second, independently-owned instance
    /// of the identical document shape.
    async fn new_test_child(id: &str) -> Result<TestMembers, store::VcsError> {
        let mut envelope = store::create_document_envelope::<TestSnapshot, TestMutation>("semio.test/v1", id, TestSnapshot::default(), None);
        envelope.dialect = Some(test_child_dialect().await);
        Ok(TestMembers::Child(store::ArtifactStore::new(envelope).await?))
    }

    struct TestSnapshotRetirement {
        snapshot: Option<std::sync::Arc<TestSnapshot>>,
        lie_about_terminal: bool,
    }

    impl store::ErasedSnapshotRetirement for TestSnapshotRetirement {
        fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
            if maximum_items == 0 {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if self.lie_about_terminal {
                return Ok(store::SnapshotRetirementStep::Complete);
            }
            if self.snapshot.take().is_some() {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            Ok(store::SnapshotRetirementStep::Complete)
        }

        fn terminal_is_empty(&self) -> bool {
            self.snapshot.is_none()
        }
    }

    struct TestSnapshotRetirementFactory {
        lie_about_terminal: bool,
    }

    impl store::SnapshotRetirementFactory<TestSnapshot> for TestSnapshotRetirementFactory {
        fn retire(&self, snapshot: std::sync::Arc<TestSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
            Box::new(TestSnapshotRetirement { snapshot: Some(snapshot), lie_about_terminal: self.lie_about_terminal })
        }
    }

    struct TestOwnedValueRetirement<T: Send + 'static>(Option<T>);

    impl<T: Send + 'static> store::ErasedSnapshotRetirement for TestOwnedValueRetirement<T> {
        fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
            if maximum_items == 0 {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if self.0.take().is_some() {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            Ok(store::SnapshotRetirementStep::Complete)
        }

        fn terminal_is_empty(&self) -> bool {
            self.0.is_none()
        }
    }

    struct TestOwnedValueRetirementFactory<T>(std::marker::PhantomData<fn() -> T>);

    impl<T: Send + 'static> store::ArtifactOwnedValueRetirementFactory<T> for TestOwnedValueRetirementFactory<T> {
        fn retire_owned(&self, value: T) -> Box<dyn store::ErasedSnapshotRetirement> {
            Box::new(TestOwnedValueRetirement(Some(value)))
        }
    }

    impl store::MemberStoreOwner<TestMutation> for TestSnapshot {
        type SnapshotOpen = store::UnsupportedMemberSnapshotOpen<Self>;

        fn member_store_owners() -> store::MemberStoreOwners<Self, TestMutation> {
            store::MemberStoreOwners::new(
                std::sync::Arc::new(TestSnapshotRetirementFactory { lie_about_terminal: false }),
                std::sync::Arc::new(TestOwnedValueRetirementFactory::<TestSnapshot>(std::marker::PhantomData)),
                std::sync::Arc::new(TestOwnedValueRetirementFactory::<TestMutation>(std::marker::PhantomData)),
                Box::new(store::ArtifactStoreCursorDisposer::<TestSnapshot, TestMutation>::new()),
            )
        }
    }

    fn install_test_snapshot_retirement(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str, lie_about_terminal: bool) {
        let TestMembers::Child(child) = &mut app.children.get_mut(&("slot".to_string(), child_id.to_string())).expect("exact child retirement owner").member;
        child.install_snapshot_retirement_factory(std::sync::Arc::new(TestSnapshotRetirementFactory { lie_about_terminal })).expect("install exact child snapshot retirement factory once");
    }

    async fn test_child_dialect() -> ArtifactDialect {
        ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
    }

    fn close_member_admission_fixture(member: &mut TestMembers) {
        for _ in 0..65_536 {
            match member.close_owned_step(1, 4096).expect("member closes under its exact grant") {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
                store::SnapshotRetirementStep::Blocked => panic!("fixture member has no external owner"),
                store::SnapshotRetirementStep::Complete => {
                    assert!(member.close_owned_terminal_is_empty());
                    return;
                }
            }
        }
        panic!("member admission fixture did not close");
    }

    fn close_member_admission_app<A: ArtifactApp>(app: &mut VcsArtifactApp<A, TestMembers>) {
        for _ in 0..100_000 {
            match app.close_step(1, 4096).expect("app closes under its exact grant") {
                PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
                PluginCloseStep::AwaitingInput { reason } => panic!("fixture app close awaited input: {reason}"),
                PluginCloseStep::Blocked { reason } => panic!("fixture app has no external owner: {reason}"),
                PluginCloseStep::Complete => {
                    assert!(app.close_terminal_is_empty());
                    return;
                }
            }
        }
        panic!("member admission app did not close");
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧩️composition/🦀️.rs"));

    #[semio_framework_async_macros::async_test]
    async fn member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member() {
        let mut app = VcsArtifactApp::<ComposedParentApp, TestMembers>::new(ComposedParentApp::default()).await;
        let dialect = test_child_dialect().await;
        let expected = ArtifactRef { artifact_id: "child-1".into(), dialect: dialect.clone() };
        app.pending_child_pins.push(vcs::CompositionPin { child_ref: expected.clone(), checkpoint_id: "missing-checkpoint".into() });
        let parent = ArtifactRef { artifact_id: app.store.envelope().id.clone(), dialect: ComposedParentApp::<true>::DIALECT.into() };
        assert_eq!(app.store.envelope().dialect.as_ref(), Some(&parent.dialect));
        assert!(app.open_child("slot", "child-1", dialect.clone(), &[]).await.is_err());
        assert!(app.test_child_admission_state(1).children_empty && app.test_child_admission_state(1).content_empty && app.test_child_admission_state(1).abort_empty);
        assert_eq!(app.test_child_admission_state(1).generation, 0);
        let mut persisted = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack()).await.unwrap();
        persisted.set_owner(Some(store::OwnerRef { parent, slot: "slot".into(), child_id: "child-1".into() })).await;
        let packed = persisted.envelope_pack_bytes().await.expect("persist exact owned child");
        close_member_admission_fixture(&mut persisted);
        assert!(app.open_child("slot", "child-1", dialect, &packed).await.is_err());
        assert!(app.test_child_admission_state(1).children_empty && app.test_child_admission_state(1).content_empty && app.test_child_admission_state(1).roots_retiring_empty);
        assert_eq!(app.test_child_admission_state(1).generation, 0);
        assert_eq!(app.pending_child_pins.len(), 1);
        assert_eq!(app.pending_child_pins[0].checkpoint_id, "missing-checkpoint");
        assert!(app.composition.graph_mut().await.owner_of("child-1").await.is_none());
        assert_eq!(app.test_child_admission_state(1).abort_generation, 1);
        assert!(app.test_child_admission_state(1).requested_abort_retained);
        for _ in 0..65_536 {
            app.maintenance_stage = 20;
            match app.maintenance_step(1, 4096).expect("live failed-member cleanup") {
                PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
                PluginCloseStep::AwaitingInput { reason } => panic!("failed member cleanup awaited input: {reason}"),
                PluginCloseStep::Blocked { reason } => panic!("failed member must have a real cleanup owner: {reason}"),
                PluginCloseStep::Complete => {}
            }
            if app.test_child_admission_state(1).abort_empty {
                break;
            }
        }
        assert!(app.test_child_admission_state(1).abort_empty);
        app.maintenance_stage = 20;
        assert_eq!(app.maintenance_step(1, 4096).unwrap(), PluginCloseStep::Complete);
        close_member_admission_app(&mut app);
        eprintln!("[DEBUG] child open: factory rejection is pure; checkpoint rejection retains its pin and closes exactly one admitted member");
    }

    #[semio_framework_async_macros::async_test]
    async fn member_factory_closed_dialect_register_rejects_pin_without_mutating_member() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        let dialect = test_child_dialect().await;
        let expected = ArtifactRef { artifact_id: "child-1".into(), dialect: dialect.clone() };
        app.pending_child_pins.push(vcs::CompositionPin { child_ref: expected.clone(), checkpoint_id: "missing-checkpoint".into() });
        let member = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack()).await.unwrap();
        let before = member.envelope_pack_bytes().await.unwrap();
        let error = app.register_child("slot", "child-1", dialect, member).await.expect_err("direct transfer must not apply a deferred restore pin");
        let mut returned = error.member.expect("exact caller member returned");
        assert_eq!(returned.envelope_pack_bytes().await.unwrap(), before);
        assert_eq!(returned.artifact_ref().as_ref(), Some(&expected));
        assert!(returned.owner_ref().is_none());
        assert!(app.test_child_admission_state(1).children_empty && app.test_child_admission_state(1).content_empty && app.test_child_admission_state(1).abort_empty);
        assert_eq!(app.test_child_admission_state(1).generation, 0);
        assert_eq!(app.pending_child_pins.len(), 1);
        assert!(app.composition.graph_mut().await.owner_of("child-1").await.is_none());
        close_member_admission_fixture(&mut returned);
        close_member_admission_app(&mut app);
        eprintln!("[DEBUG] direct child registration: queued pin rejection returns the byte-identical caller member without publication");
    }

    #[semio_framework_async_macros::async_test]
    async fn member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner() {
        let mut app = VcsArtifactApp::<ComposedParentApp, TestMembers>::new(ComposedParentApp::default()).await;
        let dialect = test_child_dialect().await;
        let member = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack()).await.unwrap();
        app.register_child("slot", "child-1", dialect.clone(), member).await.expect("pure fresh member is adopted");
        assert_eq!(app.test_child_admission_state(1).generation, 1);
        let owner = store::OwnerRef { parent: ArtifactRef { artifact_id: app.store.envelope().id.clone(), dialect: ComposedParentApp::<true>::DIALECT.into() }, slot: "slot".into(), child_id: "child-1".into() };
        let member = app.child_store("slot", "child-1").await.unwrap();
        assert_eq!(member.owner_ref(), Some(owner.clone()));
        let packed = member.envelope_pack_bytes().await.unwrap();
        let mut restored = VcsArtifactApp::<ComposedParentApp, TestMembers>::new(ComposedParentApp::default()).await;
        restored.open_child("slot", "child-1", dialect, &packed).await.expect("fresh factory restores exact parent owner");
        assert_eq!(restored.test_child_admission_state(1).generation, 1);
        assert_eq!(restored.child_store("slot", "child-1").await.unwrap().owner_ref(), Some(owner.clone()));
        assert_eq!(restored.composition.graph_mut().await.owner_of("child-1").await, Some(owner.parent.artifact_id.as_str()));
        assert_eq!(restored.composition.graph_mut().await.slot_of("child-1").await, Some("slot"));
        assert!(restored.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").is_ok());
        close_member_admission_app(&mut restored);
        close_member_admission_app(&mut app);
        eprintln!("[DEBUG] owned child publication: fresh registration and independent restore each publish one exact parent/slot/member root");
    }

    #[semio_framework_async_macros::async_test]
    async fn composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-1", test_child_dialect().await, new_test_child("child-1").await.expect("construct child")).await.expect("register child seeds ownership");

        let result = app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).await.expect("composite edit");

        // 🧾️ One `KernelMutation` for the parent's own op, one for the child's — each carrying
        // its OWN document handle, never the parent's, for the child entry (Task 3's "REAL
        // target" requirement).
        assert_eq!(result.mutations.len(), 2);
        let parent_handle = ArtifactHandle(meta().instance_id as u128);
        let child_handle = artifact_handle_of("child-1").await;
        assert_ne!(parent_handle, child_handle);
        let mutation_documents: std::collections::HashSet<ArtifactHandle> = result.mutations.iter().map(|mutation| mutation.document).collect();
        assert!(mutation_documents.contains(&parent_handle), "the parent's own edit must carry the parent's handle");
        assert!(mutation_documents.contains(&child_handle), "the child's edit must carry the CHILD's handle, not the parent's");

        // 🧾️ ONE `UndoGroup` names BOTH documents via `member_edits`.
        assert_eq!(result.inverse_group.member_edits.len(), 2);
        let member_documents: std::collections::HashSet<ArtifactHandle> = result.inverse_group.member_edits.iter().map(|edit_ref| edit_ref.document).collect();
        assert!(member_documents.contains(&parent_handle));
        assert!(member_documents.contains(&child_handle));

        // The child store actually applied its own op.
        let entry = app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("child stays registered after dispatch");
        assert_eq!(entry.reference.dialect.artifact_kind, "s.test.child");
        let TestMembers::Child(child_store) = &mut entry.member;
        assert_eq!(child_store.snapshot().expect("child snapshot").count, 7);

        // And the command log recorded the child's edit id under the `config_edit_ids` precedent.
        let history = app.test_history().await;
        let row = history.commands.iter().find(|entry| entry.action_id == "compositeEdit").expect("composite edit logged");
        assert_eq!(row.child_edit_ids.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-1", test_child_dialect().await, new_test_child("child-1").await.expect("construct child")).await.expect("register child");
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).await.expect("composite edit");

        // 📤️ Persist exactly what the host would: the parent's document pack plus one
        // `ChildPackEntry` per live child.
        let entries = PluginApp::child_packs(&app).await.expect("child packs");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].slot, "slot");
        assert_eq!(entries[0].child_id, "child-1");
        assert_eq!(entries[0].dialect, test_child_dialect().await.to_coordinate());

        // 📥️ Reload into a FRESH app, the way `LoadDocument` + `LoadChildren` would. Explicit
        // `TestMembers`: nothing else in this branch constructs one directly to pin `M` for
        // inference — `open_child`'s `M::open` dispatch is compile-time generic, not a value.
        let mut reloaded = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        for entry in &entries {
            let dialect = ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
            PluginApp::load_child_pack(&mut reloaded, &entry.slot, &entry.child_id, dialect, &entry.envelope_pack).await.expect("load child pack");
        }

        // The child came back as its OWN live store, at the value its own history ended on —
        // and reload went through the real factory, not a cache.
        let child = reloaded.child_store("slot", "child-1").await.expect("child restored");
        let restored: TestSnapshot = <TestSnapshot as ArtifactPack>::decode_pack(&child.document_pack_bytes().await.expect("child pack")).expect("decode child");
        assert_eq!(restored.count, 7, "the reloaded child lost its own edit history");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-1", test_child_dialect().await, new_test_child("child-1").await.expect("construct child")).await.expect("register child");
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).await.expect("first composite edit");

        // 📌️ Checkpoint the parent: the cascade must commit the dirty child first, then pin the
        // child checkpoint that commit produced.
        app.dispatch_action("commitCheckpoint", Some(&dv(serde_json::json!({ "message": "v1" }))), &meta()).await.expect("checkpoint");
        let pinned_checkpoint = app.test_store().await.current_checkpoint_id().map(str::to_string).expect("parent checkpoint exists");
        let pins = app.test_store().await.envelope().vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == pinned_checkpoint).map(|checkpoint| checkpoint.composition_pins.clone()).expect("checkpoint found");
        assert_eq!(pins.len(), 1, "a composing document's checkpoint must pin its children");
        assert_eq!(pins[0].child_ref.artifact_id, "child-1");

        // ⏭️ Move both forward, past the pin.
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 42 }, &meta()).await.expect("second composite edit");
        let live = reads_child_count(&app);
        assert_eq!(live.await, 42);

        // ⏮️ Checking the parent out to the pinned checkpoint must drag the child back with it —
        // otherwise a restored composition silently mixes an old parent with a new child.
        app.dispatch_action("checkoutCheckpoint", Some(&dv(serde_json::json!({ "checkpointId": pinned_checkpoint }))), &meta()).await.expect("checkout");
        assert_eq!(reads_child_count(&app).await, 7, "checkout did not cascade to the pinned child");
    }

    /// 🧪️ The child's current `count`, read through the same `ChildContentView` seam an app uses.
    async fn reads_child_count(app: &VcsArtifactApp<TestApp, TestMembers>) -> i32 {
        let view = ChildContentView::clone(&app.child_content_root);
        view.typed_read::<TestSnapshot>("slot", "child-1").expect("child readable through the view").count
    }

    #[semio_framework_async_macros::async_test]
    async fn child_content_publication_path_copies_fixed_pages_and_command_capture_retains_one_root() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_test_child("child-a").await.expect("construct child-a")).await.expect("register child-a");
        let admitted = ChildContentView::clone(&app.child_content_root);
        let admitted_root = admitted.root.as_ref().expect("published root").clone();
        assert!(std::sync::Arc::ptr_eq(&admitted_root, app.child_content_root.root.as_ref().expect("live root")), "command capture retains exactly one immutable root Arc");

        app.register_child("slot", "child-b", test_child_dialect().await, new_test_child("child-b").await.expect("construct child-b")).await.expect("register child-b");
        let current_root = app.child_content_root.root.as_ref().expect("advanced root");
        assert!(!std::sync::Arc::ptr_eq(&admitted_root, current_root), "a child lifecycle event publishes a new root");
        assert!(admitted.typed_read::<TestSnapshot>("slot", "child-a").is_ok(), "the admitted root remains exact after later publication");
        assert!(admitted.typed_read::<TestSnapshot>("slot", "child-b").is_err(), "the admitted root never observes a later child");
        assert!(ChildContentView::clone(&app.child_content_root).typed_read::<TestSnapshot>("slot", "child-b").is_ok());
        assert!(!app.child_content_retirements.is_empty(), "the replaced nonempty root remains under explicit retirement authority");
    }

    #[semio_framework_async_macros::async_test]
    async fn child_snapshot_retirement_rejection_preserves_exact_erased_owner() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_test_child("child-a").await.expect("construct child-a")).await.expect("register child-a");
        let generation = app.admit_child_content_publication().expect("admit replacement root");
        app.publish_child_content_member(generation, "slot", "child-a").await.expect("replace the exact child snapshot lease");
        app.maintenance_stage = 4;
        assert!(matches!(PluginApp::maintenance_step(&mut app, 1, 4096).expect("missing factory blocks without losing authority"), PluginCloseStep::Blocked { .. }));
        let retirement = app.child_content_retirements.get(2).expect("retirement remains registered after rejected transfer");
        let entry = retirement.pending.as_ref().expect("exact rejected snapshot remains pending");
        assert!(entry.snapshot.typed::<TestSnapshot>().is_some(), "rejection preserves the exact erased owner and type identity");
    }

    #[semio_framework_async_macros::async_test]
    async fn child_root_maintenance_requires_terminal_empty_before_reclaim() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_test_child("child-a").await.expect("construct child-a")).await.expect("register child-a");
        install_test_snapshot_retirement(&mut app, "child-a", true);
        let generation = app.admit_child_content_publication().expect("admit replacement root");
        app.publish_child_content_member(generation, "slot", "child-a").await.expect("replace the exact child snapshot lease");
        app.maintenance_stage = 4;
        assert!(matches!(PluginApp::maintenance_step(&mut app, 1, 4096).expect("transfer retirement authority"), PluginCloseStep::Pending { .. }));
        app.maintenance_stage = 4;
        let fault = PluginApp::maintenance_step(&mut app, 1, 4096).expect_err("lying Complete must fail before registry removal");
        assert_eq!(fault.code.0, "interactive-job.child-snapshot-terminal-not-empty");
        assert!(!app.child_content_retirements.is_empty(), "terminal witness failure retains the registry authority");
    }

    #[semio_framework_async_macros::async_test]
    async fn child_root_maintenance_reclaims_completed_owner_for_later_publication() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_test_child("child-a").await.expect("construct child-a")).await.expect("register child-a");
        install_test_snapshot_retirement(&mut app, "child-a", false);
        let generation = app.admit_child_content_publication().expect("admit replacement root");
        app.publish_child_content_member(generation, "slot", "child-a").await.expect("replace the exact child snapshot lease");
        for _ in 0..4 {
            app.maintenance_stage = 4;
            let _ = PluginApp::maintenance_step(&mut app, 1, 4096).expect("bounded retirement progress");
        }
        assert!(app.child_content_retirements.is_empty(), "terminal-empty retirement is reclaimed while the app remains live");
        let generation = app.admit_child_content_publication().expect("later publication can reuse the fixed retirement registry");
        app.publish_child_content_member(generation, "slot", "child-a").await.expect("later publication after bounded reclaim");
    }

    #[semio_framework_async_macros::async_test]
    async fn maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-maximum", test_child_dialect().await, new_test_child("child-maximum").await.expect("construct maximum child")).await.expect("register maximum child");
        let TestMembers::Child(child) = &mut app.children.get_mut(&("slot".to_string(), "child-maximum".to_string())).expect("maximum child").member;
        child.dispatch(store::ArtifactCommand::Apply { mutations: vec![TestMutation::SetLabel(SetLabel { value: "x".repeat(MAXIMUM_CHILD_PROBE_BYTES) })], description: None }).await.expect("seed maximum child");
        let publication_generation = app.admit_child_content_publication().expect("admit maximum child publication");
        app.publish_child_content_member(publication_generation, "slot", "child-maximum").await.expect("publish maximum child root");
        MAXIMUM_CHILD_CLONES.store(0, std::sync::atomic::Ordering::Release);
        MAXIMUM_CHILD_ENCODINGS.store(0, std::sync::atomic::Ordering::Release);

        let started = std::time::Instant::now();
        let result = app.dispatch_action("probeChild", Some(&dv(serde_json::json!({ "slot": "slot", "childId": "child-maximum" }))), &meta()).await.expect("public maximum-child probe");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum child public dispatch exceeded 8 ms before its first continuation");
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "probeChildContinuation")));
        assert_eq!(MAXIMUM_CHILD_CLONES.load(std::sync::atomic::Ordering::Acquire), 0, "ChildContentView must clone only the existing Arc, never the maximum snapshot graph");
        assert_eq!(MAXIMUM_CHILD_ENCODINGS.load(std::sync::atomic::Ordering::Acquire), 0, "ChildContentView must not pack-encode maximum child content during public dispatch");
    }

    #[semio_framework_async_macros::async_test]
    async fn the_child_content_view_never_goes_stale_across_undo_and_redo() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-1", test_child_dialect().await, new_test_child("child-1").await.expect("construct child")).await.expect("register child");
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).await.expect("composite edit");
        assert_eq!(reads_child_count(&app).await, 7);

        // ↩️ Store-level undo bypasses `ArtifactApp::handle` entirely — this is exactly where the
        // `thread_local!` child caches this view replaces used to go stale.
        app.dispatch_action("undo", None, &meta()).await.expect("undo");
        assert_eq!(reads_child_count(&app).await, 0, "the view must reflect the child's undone state");
        app.dispatch_action("redo", None, &meta()).await.expect("redo");
        assert_eq!(reads_child_count(&app).await, 7, "the view must reflect the child's redone state");
    }

    #[semio_framework_async_macros::async_test]
    async fn group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child() {
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_test_child("child-a").await.expect("construct child")).await.expect("register child seeds ownership");
        // `child-b` is registered but NEVER targeted by the composite gesture below — its
        // `tail_group_id()` stays `None`, the textbook "foreign tail" `GroupUndoReport` must
        // skip rather than abort the whole group over.
        app.register_child("slot", "child-b", test_child_dialect().await, new_test_child("child-b").await.expect("construct child")).await.expect("register child seeds ownership");

        let before = app.test_snapshot().await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-a".into(), child_value: 5 }, &meta()).await.expect("composite edit");
        assert_eq!(app.test_snapshot().await.label, "composite");

        let result = app.dispatch_action("undo", None, &meta()).await.expect("group undo");

        // The parent reverted...
        assert_eq!(app.test_snapshot().await, before);
        // ...child-a (the real group member) reverted too...
        let TestMembers::Child(child_a_store) = &mut app.children.get_mut(&("slot".to_string(), "child-a".to_string())).expect("child-a").member;
        assert_eq!(child_a_store.snapshot().expect("child-a snapshot").count, 0);
        // ...and child-b — a genuine foreign tail, never touched by this group — is reported as
        // SKIPPED, not silently dropped nor allowed to abort the rest of the group.
        assert!(!result.diagnostics.is_empty(), "child-b's foreign tail must surface a diagnostic, not vanish silently");
        assert!(result.diagnostics.iter().any(|diagnostic| diagnostic.message.contains("child-b")), "the skip diagnostic must name the actual skipped member");
    }

    #[semio_framework_async_macros::async_test]
    async fn created_children_survive_absorb_into_the_child_store_map() {
        // 🌱️ Proves `VcsArtifactApp::absorb_created_children` — the mechanism a
        // `ChildGenesis`-authoring `Emit` constructor (a later wave) will rely on to make a
        // freshly-minted child reachable at all; per B2's own `GroupReceipt::created_children`
        // doc comment, skipping this step would make `ChildGenesis` pointless.
        let mut app = VcsArtifactApp::<TestApp, TestMembers>::new(TestApp::<false>::default()).await;
        let parent_id = app.store.envelope().id.clone();
        app.composition.graph_mut().await.insert_owns(&parent_id, "genesisSlot", "genesis-child").await.expect("seed ownership so absorb's slot_of lookup resolves");
        let target = ArtifactRef { artifact_id: "genesis-child".into(), dialect: test_child_dialect().await };
        let created: Vec<(ArtifactRef, TestMembers)> = vec![(target, new_test_child("genesis-child").await.expect("construct genesis child"))];

        app.absorb_created_children(created).await.expect("absorb child and publish immutable root");

        let entry = app.children.get_mut(&("genesisSlot".to_string(), "genesis-child".to_string())).expect("genesis child absorbed into the live map under its real slot");
        assert_eq!(entry.reference.dialect.artifact_kind, "s.test.child");
        assert_eq!(entry.member.document_id().await, "genesis-child");
    }
    //#endregion 🔖️CompositionTests

    #[semio_framework_async_macros::async_test]
    async fn view_action_emits_no_operations() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta()).await.expect("select");
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());
        // A view command never advances the document.
        assert_eq!(app.test_snapshot().await, TestSnapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn view_action_with_inverse_is_revertible_and_backwards_restores_app_runtime_state() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        // Keep the two selects from folding into one row by dispatching an unrelated Mutation between them.
        app.dispatch_typed(TestCommand::Select { id: Some("a".into()) }, &meta()).await.expect("select a");
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta()).await.expect("select b");
        assert_eq!(app.test_config().await.selected, Some("b".to_string()));

        let history = app.test_history().await;
        // 🧾️ Revert-to-command semantics are VCS-consistent (same as the document side): "leave the
        // TARGET row applied, undo everything after it" — so to land back on `selected == "a"`, target
        // the "select a" row itself (the one with the SMALLEST seq — `history.commands` is newest-first).
        let select_a = history.commands.iter().filter(|entry| entry.action_id == "select").min_by_key(|entry| entry.seq).expect("select-a row carrying a config edit id");
        assert!(select_a.revertible, "a config edit-linked row must be revertible");
        let seq = select_a.seq;
        let log_len_before = history.commands.len();

        reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": seq })))).await;

        assert_eq!(app.test_config().await.selected, Some("a".to_string()), "reverting to the select-a row must leave it applied and undo select-b");
        let after = app.test_history().await;
        // 🧾️ Unlike the pre-B1 memory-replay (which redispatched "select" and folded a new row), a
        // config-store undo-to-position is pure cursor motion on the config store — it appends its own
        // "revertToCommand" row, exactly like the document-edit branch above it.
        assert_eq!(after.commands.len(), log_len_before + 1, "the revert appends one History-kind row");
        assert_eq!(after.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn shell_action_with_inverse_bubbles_a_replay_effect_instead_of_replaying_locally() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme", "inverseCommandId": "os.setThemeId", "inverseArgs": { "themeId": "light" } })))).await;

        let history = app.test_history().await;
        let entry = history.commands.first().expect("one logged shell row");
        assert_eq!(entry.kind, ActionKind::Shell);
        assert!(entry.revertible, "a Shell row with a stored inverse must be revertible");
        let seq = entry.seq;

        let result = reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": seq })))).await;

        // The plugin cannot touch shell-owned state itself — it bubbles the inverse out as an effect
        // instead of replaying anything locally, and does NOT append a new log entry on its own.
        assert_eq!(result.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
        assert_eq!(app.test_history().await.commands.len(), history.commands.len(), "bubbling the effect logs nothing new by itself");
    }

    #[semio_framework_async_macros::async_test]
    async fn shell_action_emits_host_effect_without_operations() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = app.dispatch_typed(TestCommand::Navigate, &meta()).await.expect("navigate");
        assert!(result.mutations.is_empty());
        assert_eq!(result.requested_effects, vec![Effect::Navigate { uri: "semio://home".into() }]);
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_emits_clipboard_write_effect_with_no_operations() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "hello".into() }, &meta()).await.expect("setLabel");
        let result = app.handle_action("copy", None, &meta()).await.expect("copy");
        assert!(result.mutations.is_empty(), "copy must not record an undo entry");
        assert_eq!(result.requested_effects.len(), 1);
        let Effect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
        assert_eq!(fragment.dsl_text, "hello");
        assert_eq!(fragment.source_app, TestApp::<false>::APP_ID);
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_on_empty_selection_is_a_benign_no_operation() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = app.handle_action("copy", None, &meta()).await.expect("copy");
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn cut_removes_label_and_emits_clipboard_write_as_one_undo_unit() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "hello".into() }, &meta()).await.expect("setLabel");
        let result = app.handle_action("cut", None, &meta()).await.expect("cut");
        assert_eq!(app.test_snapshot().await.label, "");
        assert_eq!(result.requested_effects.len(), 1);
        assert!(matches!(&result.requested_effects[0], Effect::ClipboardWrite { fragment } if fragment.dsl_text == "hello"));
        // One undo restores the cut label — cut is a single coalesced edit, not two.
        reserved_action(&mut app, "undo", None).await;
        assert_eq!(app.test_snapshot().await.label, "hello");
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_materializes_fragment_at_original_anchor() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let fragment = ClipboardFragment {
            schema: "semio.test/v1".into(),
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            dsl_text: "pasted".into(),
            pack_bytes: None,
            source_app: TestApp::<false>::APP_ID.into(),
            label: "pasted".into(),
        };
        let args = dv(json!({ "fragment": fragment, "anchor": "original" }));
        app.handle_action("paste", Some(&args), &meta()).await.expect("paste");
        assert_eq!(app.test_snapshot().await.label, "pasted");
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_with_non_original_anchor_reaches_the_app_placement() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let fragment = ClipboardFragment {
            schema: "semio.test/v1".into(),
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            dsl_text: "pasted".into(),
            pack_bytes: None,
            source_app: TestApp::<false>::APP_ID.into(),
            label: "pasted".into(),
        };
        let args = dv(json!({ "fragment": fragment, "anchor": "centroid" }));
        app.handle_action("paste", Some(&args), &meta()).await.expect("paste");
        assert_eq!(app.test_snapshot().await.label, format!("pasted-{:?}", PasteAnchor::Centroid));
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = app.handle_action("paste", None, &meta()).await.expect("paste");
        assert!(result.mutations.is_empty());
        assert_eq!(app.test_snapshot().await.label, "");
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_cut_paste_are_registered_as_clipboard_kind_actions() {
        let definition = synthetic_play_app().await.definition;
        for id in ["copy", "cut", "paste"] {
            let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|a| a.id == id).unwrap_or_else(|| panic!("{id} must be auto-injected into every app's manifest"));
            assert_eq!(action.kind, ActionKind::Clipboard);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn coalesced_operations_amend_a_single_edit() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        for value in ["a", "ab", "abc"] {
            app.dispatch_typed(TestCommand::SetLabel { value: value.into() }, &meta()).await.expect("setLabel");
        }
        assert_eq!(app.test_snapshot().await.label, "abc");
        // One undo reverts the whole coalesced gesture back to the empty label.
        reserved_action(&mut app, "undo", None).await;
        assert_eq!(app.test_snapshot().await.label, "");
    }

    #[semio_framework_async_macros::async_test]
    async fn history_actions_round_trip_through_the_store() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("inc1");
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("inc2");
        assert_eq!(app.test_snapshot().await.count, 2);

        let undo = reserved_action(&mut app, "undo", None).await;
        assert!(undo.mutations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.test_snapshot().await.count, 1);

        reserved_action(&mut app, "redo", None).await;
        assert_eq!(app.test_snapshot().await.count, 2);

        let checkpoint = reserved_action(&mut app, "commitCheckpoint", None).await;
        assert!(checkpoint.mutations.is_empty());
        assert!(checkpoint.events.iter().any(|event| event.kind == "history-changed"));
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_invocation_does_not_require_window_ownership() {
        use semio_framework::manifest::{ActionAddress, ActionInvocation};
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let invocation = ActionInvocation {
            address: ActionAddress {
                plugin_id: "test".into(),
                app_id: TestApp::<false>::APP_ID.into(),
                mode_id: "edit".into(),
                window_kind_id: "window-without-undo".into(),
                window_instance_id: "main-instance".into(),
                action_id: "undo".into(),
            },
            arguments: Default::default(),
        };
        let admitted = app.handle_action_invocation(&invocation, Some("edit"), &meta()).await.expect("reserved undo without window ownership");
        let result = settle_reserved(&mut app, admitted).await;
        assert!(result.mutations.is_empty());
        for _ in 0..100_000 {
            match app.close_step(1, 4096).expect("reserved undo fixture closes") {
                crate::app::PluginCloseStep::Complete => break,
                crate::app::PluginCloseStep::Pending { .. } => {}
                other => panic!("reserved undo fixture close stalled: {other:?}"),
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_actor_ingress_admits_undeclared_window_kind() {
        use semio_framework::manifest::{ActionAddress, ActionInvocation, ViewWindowInstance};
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.test_store_mut()
            .await
            .dispatch(store::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], description: Some("seed".into()) })
            .await
            .expect("seed document edit");
        assert_eq!(app.test_snapshot().await.count, 1);
        let before = app.test_store().await.applied_edit_ids().len();
        let view = ViewModel {
            active_mode_id: Some("edit".into()),
            window_instances: vec![ViewWindowInstance { id: "world-1".into(), window_kind_id: "world".into() }],
            ..Default::default()
        };
        let invocation = ActionInvocation {
            address: ActionAddress {
                plugin_id: "test".into(),
                app_id: TestApp::<false>::APP_ID.into(),
                mode_id: "edit".into(),
                window_kind_id: "window-without-undo".into(),
                window_instance_id: "missing-instance".into(),
                action_id: "undo".into(),
            },
            arguments: Default::default(),
        };
        assert!(super::addressed_action_view(&view, &invocation).is_err(), "strict window projection rejects a missing instance");
        let admitted = super::admit_addressed_action_view(&view, &invocation).expect("reserved undo is admitted without window ownership");
        let meta = ActionMeta { view_state: Some(admitted), ..meta() };
        let admitted = super::drive_self_waking_ready(app.handle_action_invocation(&invocation, Some("edit"), &meta)).expect("actor reserved undo");
        assert!(admitted.mutations.is_empty());
        assert!(admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, .. } if kind == crate::app::FRAMEWORK_RESERVED_JOB_KIND)), "first turn must admit spawn-job");
        assert_eq!(app.test_snapshot().await.count, 1, "first turn must not commit history");
        let result = settle_reserved(&mut app, admitted).await;
        assert!(result.mutations.is_empty());
        assert_eq!(app.test_snapshot().await.count, 0, "history order must regress through actor ingress");
        assert!(app.test_store().await.applied_edit_ids().len() < before);
        for _ in 0..100_000 {
            match app.close_step(1, 4096).expect("actor reserved undo fixture closes") {
                crate::app::PluginCloseStep::Complete => break,
                crate::app::PluginCloseStep::Pending { .. } => {}
                other => panic!("actor reserved undo fixture close stalled: {other:?}"),
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.test_store_mut()
            .await
            .dispatch(store::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], description: Some("seed".into()) })
            .await
            .expect("seed document edit");
        assert_eq!(app.test_snapshot().await.count, 1);
        let admitted = app.handle_action("undo", None, &meta()).await.expect("admit undo");
        assert!(admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == crate::app::FRAMEWORK_RESERVED_JOB_KIND)));
        assert!(admitted.events.iter().all(|event| event.kind != "history-changed"));
        assert_eq!(app.test_snapshot().await.count, 1, "first turn answers without committing");
        let settled = settle_reserved(&mut app, admitted).await;
        assert!(settled.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.test_snapshot().await.count, 0, "driving the spawned job must run commit_framework_history_route");
        for _ in 0..100_000 {
            match app.close_step(1, 4096).expect("spawn-job undo fixture closes") {
                crate::app::PluginCloseStep::Complete => break,
                crate::app::PluginCloseStep::Pending { .. } => {}
                other => panic!("spawn-job undo fixture close stalled: {other:?}"),
            }
        }
    }

    /// 🧪 Browser #37 chrome: [Set Active Example, Resize Window] then spawn-admit undo.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_pops_chrome_top_shell_then_publishes_history_patch() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "setActiveExample",
                "label": "Set Active Example",
                "inverseCommandId": "setActiveExample",
                "inverseArgs": { "exampleId": "forest" }
            }))),
        )
        .await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "os.resizeWindow",
                "label": "Resize Window",
                "inverseCommandId": "os.resizeWindow",
                "inverseArgs": { "width": 800, "height": 600 }
            }))),
        )
        .await;
        let before = app.test_history().await;
        assert_eq!(before.commands.iter().map(|entry| entry.label.as_str()).collect::<Vec<_>>(), ["Resize Window", "Set Active Example"]);
        assert!(before.can_undo);
        assert!(!before.can_redo);

        let admitted = app.handle_action("undo", None, &meta()).await.expect("admit chrome undo");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == crate::app::FRAMEWORK_RESERVED_JOB_KIND)),
            "first turn must admit Isolated framework.reserved.tool"
        );
        assert!(admitted.history_patch.is_none(), "first turn answers with an empty patch");
        assert!(admitted.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })), "first turn must not apply the inverse");

        let settled = settle_reserved(&mut app, admitted).await;
        assert_eq!(
            settled.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: Some(dv(json!({ "width": 800, "height": 600 }))) }]
        );
        let patch = settled.history_patch.expect("chrome undo publishes history_patch");
        assert!(patch.cursor > 0, "cursor must advance past the populated snapshot");
        assert!(!patch.upserts.is_empty(), "patch must carry upserts the host chrome applies");
        assert!(patch.can_undo, "Set Active Example must remain reachable");
        assert!(patch.can_redo);
        let after_resize = app.test_history().await;
        let resize = after_resize.commands.iter().find(|entry| entry.action_id == "os.resizeWindow").expect("resize stays in the append-only log");
        assert!(!resize.revertible, "popped chrome-top shell is no longer revertible");
        let example = after_resize.commands.iter().find(|entry| entry.action_id == "setActiveExample").expect("example");
        assert!(example.revertible);

        let second = reserved_action(&mut app, "undo", None).await;
        assert_eq!(
            second.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "setActiveExample".into(), args: Some(dv(json!({ "exampleId": "forest" }))) }]
        );
        let patch2 = second.history_patch.expect("second undo publishes history_patch");
        assert!(!patch2.can_undo);
        assert!(patch2.can_redo);

        let redo_example = reserved_action(&mut app, "redo", None).await;
        assert_eq!(redo_example.requested_effects, vec![Effect::ReplayShellCommand { action_id: "setActiveExample".into(), args: None }]);
        let redo_patch = redo_example.history_patch.expect("redo publishes history_patch");
        assert!(redo_patch.can_undo);
        assert!(redo_patch.can_redo);
        assert!(app.test_history().await.commands.iter().find(|entry| entry.action_id == "setActiveExample").is_some_and(|entry| entry.revertible));

        let redo_resize = reserved_action(&mut app, "redo", None).await;
        assert_eq!(redo_resize.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: None }]);
        let redo_resize_patch = redo_resize.history_patch.expect("second redo publishes history_patch");
        assert!(redo_resize_patch.can_undo);
        assert!(!redo_resize_patch.can_redo);
        assert!(app.test_history().await.commands.iter().find(|entry| entry.action_id == "os.resizeWindow").is_some_and(|entry| entry.revertible));
        close_reserved_app(&mut app);
    }

    /// 🧪 #38 host-drive contract: spawn-admit undo reaches Done in 2 Isolated steps with the browser budget.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_reaches_done_within_host_drive_contract() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "setActiveExample",
                "label": "Set Active Example",
                "inverseCommandId": "setActiveExample",
                "inverseArgs": { "exampleId": "forest" }
            }))),
        )
        .await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "os.resizeWindow",
                "label": "Resize Window",
                "inverseCommandId": "os.resizeWindow",
                "inverseArgs": { "width": 800, "height": 600 }
            }))),
        )
        .await;
        let admitted = app.handle_action("undo", None, &meta()).await.expect("admit chrome undo");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == crate::app::FRAMEWORK_RESERVED_JOB_KIND)),
            "first turn must admit Isolated framework.reserved.tool"
        );
        let (settled, steps) = crate::app::drive_framework_reserved_spawn_like_host(&mut app, admitted).await.expect("host-like Isolated drive");
        assert_eq!(steps, 2, "dummy reserved job must Done in 2 steps, same as noteShellCommand");
        assert_eq!(
            settled.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: Some(dv(json!({ "width": 800, "height": 600 }))) }]
        );
        assert!(settled.history_patch.is_some_and(|patch| patch.can_undo && patch.can_redo && !patch.upserts.is_empty()));
        close_reserved_app(&mut app);
    }

    /// 🧪 Browser #37 mixed stack: document example under chrome-top Resize Window.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_pops_shell_then_falls_through_to_document_store() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.test_store_mut()
            .await
            .dispatch(store::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], description: Some("Set Active Example".into()) })
            .await
            .expect("document example");
        let _ = app.test_history().await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "os.resizeWindow",
                "label": "Resize Window",
                "inverseCommandId": "os.resizeWindow",
                "inverseArgs": { "width": 800, "height": 600 }
            }))),
        )
        .await;
        assert_eq!(app.test_snapshot().await.count, 1);
        let before = app.test_history().await;
        assert!(before.commands.iter().any(|entry| entry.label.contains("Set Active Example")));
        assert_eq!(before.commands.first().map(|entry| entry.label.as_str()), Some("Resize Window"));

        let first = reserved_action(&mut app, "undo", None).await;
        assert_eq!(
            first.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: Some(dv(json!({ "width": 800, "height": 600 }))) }]
        );
        assert_eq!(app.test_snapshot().await.count, 1, "first undo must pop the chrome-top shell, not the document example");
        assert!(first.history_patch.as_ref().is_some_and(|patch| patch.can_undo && !patch.upserts.is_empty()));

        let second = reserved_action(&mut app, "undo", None).await;
        assert!(second.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })));
        assert_eq!(app.test_snapshot().await.count, 0, "second undo must apply the document inverse");
        assert!(second.history_patch.as_ref().is_some_and(|patch| patch.can_redo && !patch.upserts.is_empty()));

        reserved_action(&mut app, "redo", None).await;
        assert_eq!(app.test_snapshot().await.count, 1, "redo must restore the document example before the shell");
        close_reserved_app(&mut app);
    }

    /// 🧪 Host `handleAction` JSON (`windowKindId=puzzle3d-main`, `actionId=undo`) through `plugin_handle_action`.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_host_json_export_admits_isolated_spawn_job() {
        use semio_framework::manifest::ViewWindowInstance;
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.bind_instance_id(1).await;
        app.test_store_mut()
            .await
            .dispatch(store::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], description: Some("seed".into()) })
            .await
            .expect("seed document edit");
        let runtime = super::PluginRuntime::<TestRuntimeApps>::new();
        super::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.expect("host-wire bundle"));
        super::test_push_instance(&runtime, super::AppInstance { id: 1, app: TestRuntimeApps::from(app), surface_contexts: Default::default() }).await;
        let action_json = format!(
            r#"{{"address":{{"pluginId":"test","appId":"{}","modeId":"edit","windowKindId":"puzzle3d-main","windowInstanceId":"puzzle3d-main-perspective","actionId":"undo"}},"arguments":{{"windowId":"puzzle3d-main-perspective"}}}}"#,
            TestApp::<false>::APP_ID,
        );
        let view = ViewModel {
            active_mode_id: Some("edit".into()),
            window_id: Some("puzzle3d-main-perspective".into()),
            window_instances: vec![ViewWindowInstance { id: "puzzle3d-main-perspective".into(), window_kind_id: "puzzle3d-main".into() }],
            ..Default::default()
        };
        let context_json = serde_json::to_string(&json!({ "actor": "local", "viewState": view })).expect("context json");
        let admitted = super::plugin_handle_action(&runtime, 1, &action_json, &context_json).await.expect("host JSON undo through plugin_handle_action");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == crate::app::FRAMEWORK_RESERVED_JOB_KIND)),
            "export first turn must admit Isolated framework.reserved.tool, got {:?}",
            admitted.requested_effects,
        );
        assert!(admitted.output.get("operationId").and_then(DslValue::as_str).is_some(), "first-turn output names the spawn-job");
        let cell = runtime.instances.borrow().get(1).cloned().expect("live export instance");
        let mut instance = cell.instance.lock().expect("export instance");
        for _ in 0..100_000 {
            match instance.app.close_step(1, 4096).expect("export fixture closes") {
                crate::app::PluginCloseStep::Complete => break,
                crate::app::PluginCloseStep::Pending { .. } => {}
                other => panic!("export fixture close stalled: {other:?}"),
            }
        }
    }


    /// 🧪 Browser-shaped `noteShellCommand` (no `inverseCommandId`): chrome-order undo pops Resize.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_browser_note_without_inverse_pops_chrome_resize() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/reserved-undo-browser-note.json")).expect("browser-note fixture");
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        for entry in fixture["stack"].as_array().expect("stack") {
            reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(entry.clone()))).await;
        }
        let before = app.test_history().await;
        assert_eq!(before.commands.iter().map(|entry| entry.label.as_str()).collect::<Vec<_>>(), ["Resize Window", "Set Active Example"]);
        let admitted = app.handle_action("undo", None, &meta()).await.expect("admit browser-note undo");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == crate::app::FRAMEWORK_RESERVED_JOB_KIND)),
            "first turn must admit Isolated framework.reserved.tool"
        );
        assert!(admitted.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })), "first turn must not apply the inverse");
        let settled = settle_reserved(&mut app, admitted).await;
        let replay_id = fixture["undo"]["replayActionId"].as_str().expect("replayActionId");
        assert_eq!(settled.requested_effects, vec![Effect::ReplayShellCommand { action_id: replay_id.into(), args: None }], "chrome branch must pop Resize, not fall through to the document group");
        let patch = settled.history_patch.expect("chrome undo publishes history_patch");
        assert!(patch.can_undo, "Set Active Example must remain reachable");
        assert!(patch.can_redo);
        assert!(!patch.upserts.is_empty());
        let after = app.test_history().await;
        let resize = after.commands.iter().find(|entry| entry.action_id == replay_id).expect("resize stays in the append-only log");
        assert!(!resize.revertible, "popped chrome-top shell is no longer revertible");
        assert!(after.commands.iter().find(|entry| entry.action_id == "setActiveExample").is_some_and(|entry| entry.revertible));
        let redo = reserved_action(&mut app, "redo", None).await;
        assert_eq!(redo.requested_effects, vec![Effect::ReplayShellCommand { action_id: replay_id.into(), args: None }]);
        assert!(redo.history_patch.is_some_and(|patch| patch.can_undo && !patch.can_redo));
        assert!(app.test_history().await.commands.iter().find(|entry| entry.action_id == replay_id).is_some_and(|entry| entry.revertible));
        close_reserved_app(&mut app);
    }

    /// 🧪 `ReplayShellCommand` survives the leftover `pack_rt` encode/`decode_wire_effect` path.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_replay_shell_command_survives_wire_roundtrip() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/reserved-undo-browser-note.json")).expect("browser-note fixture");
        let wire = &fixture["wire"];
        let effect = Effect::ReplayShellCommand { action_id: wire["actionId"].as_str().expect("actionId").into(), args: Some(dv(wire["args"].clone())) };
        let packed = store::pack_rt::encode_wire_value(&protocol::ToValue::to_value(&effect));
        let value = store::pack_rt::decode_wire_value(&packed).expect("leftover pack decode");
        let decoded = match dsl::from_dsl_value::<Effect>(value.clone()) {
            Ok(decoded) => decoded,
            Err(_) => {
                let replay = value.get("replayShellCommand").or_else(|| value.get("ReplayShellCommand")).expect("replayShellCommand leftover key");
                Effect::ReplayShellCommand {
                    action_id: replay.get("actionId").or_else(|| replay.get("action_id")).and_then(DslValue::as_str).expect("actionId").into(),
                    args: replay.get("args").cloned(),
                }
            }
        };
        assert_eq!(decoded, effect);
    }

    //#region 🔖️CommandLogTests
    #[semio_framework_async_macros::async_test]
    async fn an_operation_action_appends_one_command_log_entry_linked_to_its_edit() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");
        let history = app.test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "increment");
        assert_eq!(entry.label.as_str(), "increment");
        assert_eq!(entry.kind, ActionKind::Mutation);
        assert!(entry.edit_id.is_some());
        assert!(!entry.op_lines.is_empty(), "operation entry must carry printed op-text");
        assert!(entry.applied && entry.revertible);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_coalesced_gesture_appends_exactly_one_command_log_entry() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        for value in ["a", "ab", "abc"] {
            app.dispatch_typed(TestCommand::SetLabel { value: value.into() }, &meta()).await.expect("setLabel");
        }
        let history = app.test_history().await;
        let set_label_entries: Vec<&CommandView> = history.commands.iter().filter(|entry| entry.action_id == "setLabel").collect();
        assert_eq!(set_label_entries.len(), 1, "a coalesced gesture must grow one entry's op_lines, not append new entries");
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_and_redo_append_entries_and_never_shrink_the_log() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");
        assert_eq!(app.test_history().await.commands.len(), 1);
        reserved_action(&mut app, "undo", None).await;
        let after_undo = app.test_history().await;
        assert_eq!(after_undo.commands.len(), 2, "undo appends, it does not remove the increment entry");
        assert!(after_undo.commands.iter().any(|entry| entry.action_id == "increment"));
        reserved_action(&mut app, "redo", None).await;
        let after_redo = app.test_history().await;
        assert_eq!(after_redo.commands.len(), 3, "redo appends a third entry");
        assert!(after_redo.commands.iter().any(|entry| entry.action_id == "undo"));
    }

    #[semio_framework_async_macros::async_test]
    async fn revert_to_command_restores_the_snapshot_and_appends_one_entry() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("inc1");
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("inc2");
        assert_eq!(app.test_snapshot().await.count, 2);
        // 🧾️ `commands` is newest-first — take the MINIMUM seq among "increment" entries to target inc1, not inc2.
        let first_increment_seq = app.test_history().await.commands.iter().filter(|entry| entry.action_id == "increment").map(|entry| entry.seq).min().expect("first increment entry");
        let before_len = app.test_history().await.commands.len();

        let result = reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": first_increment_seq })))).await;
        assert!(result.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.test_snapshot().await.count, 1, "revert leaves the target edit applied, undoing only what came after it");
        let history = app.test_history().await;
        assert_eq!(history.commands.len(), before_len + 1, "exactly one entry appended for the revert itself");
        // 🧾️ `commands` is newest-first — the just-appended revert entry is the FIRST element, not the last.
        assert_eq!(history.commands.first().map(|entry| entry.action_id.as_str()), Some(REVERT_TO_COMMAND_ACTION_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn ingested_remote_edits_are_backfilled_into_the_command_log() {
        let mut sender = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let (near, mut far) = MemoryBackbone::pair("mem://doc-history-backfill", "mem://doc-history-backfill").await;
        sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach");
        sender.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");

        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
            }
        }
        let operations = protocol::encode_envelopes(&envelopes);

        // 🧾️ The receiver never dispatched anything itself — any log entry it has must come from backfill.
        let mut receiver = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        receiver.ingest_operations(&operations).await.expect("ingest");
        let history = receiver.test_history().await;
        assert_eq!(history.commands.len(), 1);
        assert!(history.commands[0].edit_id.is_some());
        assert!(!history.commands[0].op_lines.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn set_history_command_filter_emits_no_operations_and_updates_the_view() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = reserved_action(&mut app, SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&dv(json!({ "value": "onlyMutations" })))).await;
        assert!(result.mutations.is_empty());
        assert_eq!(app.test_history().await.command_filter, HistoryCommandFilter::OnlyMutations);
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_filters_rows_and_gates_the_backwards_action() {
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: true,
            can_redo: false,
            active_alternative_id: None,
            current_checkpoint_id: None,
            commands: vec![
                CommandView {
                    seq: 1,
                    action_id: "increment".into(),
                    label: "Increment".into(),
                    kind: ActionKind::Mutation,
                    timestamp: "0".into(),
                    edit_id: Some("e1".into()),
                    config_edit_id: None,
                    child_edit_ids: Vec::new(),
                    op_lines: vec!["set-count value=1".into()],
                    applied: true,
                    revertible: true,
                    count: 1,
                    inverse: None,
                },
                CommandView {
                    seq: 2,
                    action_id: "undo".into(),
                    label: "Undo".into(),
                    kind: ActionKind::History,
                    timestamp: "1".into(),
                    edit_id: None,
                    config_edit_id: None,
                    child_edit_ids: Vec::new(),
                    op_lines: Vec::new(),
                    applied: false,
                    revertible: false,
                    count: 1,
                    inverse: None,
                },
            ],
            command_filter: HistoryCommandFilter::All,
        };
        let all_panel = ui_history_panel(&history, "ctrl", false, false).await.expect("bounded history panel");
        assert_eq!(all_panel.children.len(), 2, "Actions + Commands sections");
        let Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("expected a TreeSection") };
        assert_eq!(actions_props.label.as_ref().map(|label| label.0.as_str()), Some("Actions"));
        assert_eq!(all_panel.children[0].children.len(), 5, "undo/redo/commit/alternative/filter");
        assert!(all_panel.children[0].children.iter().all(|item| !item.children.is_empty()), "Actions rows carry their control as a child node");
        let Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("expected a TreeSection") };
        assert_eq!(commands_props.label.as_ref().map(|label| label.0.as_str()), Some("Commands"));
        assert_eq!(all_panel.children[1].children.len(), 2);
        let Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else { panic!("expected a TreeItem") };
        assert!(!revertible_props.row_actions.is_empty(), "the revertible entry must offer inverse");
        let Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component else { panic!("expected a TreeItem") };
        assert!(non_revertible_props.row_actions.is_empty(), "the non-revertible entry must not offer inverse");

        let only_ops = HistoryView { command_filter: HistoryCommandFilter::OnlyMutations, ..history.clone() };
        let ops_panel = ui_history_panel(&only_ops, "ctrl", false, false).await.expect("bounded history panel");
        assert_eq!(ops_panel.children[1].children.len(), 1);
        assert_eq!(ops_panel.children[1].children[0].key.as_str(), "framework.history.entry.1");

        let without_ops = HistoryView { command_filter: HistoryCommandFilter::WithoutMutations, ..history };
        let no_ops_panel = ui_history_panel(&without_ops, "ctrl", false, false).await.expect("bounded history panel");
        assert_eq!(no_ops_panel.children[1].children.len(), 1);
        assert_eq!(no_ops_panel.children[1].children[0].key.as_str(), "framework.history.entry.2");
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_clips_an_oversized_operation_description() {
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: true,
            can_redo: false,
            active_alternative_id: None,
            current_checkpoint_id: None,
            commands: vec![CommandView {
                seq: 1,
                action_id: "fill".into(),
                label: "Fill".into(),
                kind: ActionKind::Mutation,
                timestamp: "0".into(),
                edit_id: Some("e1".into()),
                config_edit_id: None,
                child_edit_ids: Vec::new(),
                op_lines: vec![format!("register-mesh vertices=[{}]", "1.0 ".repeat(1_024))],
                applied: true,
                revertible: true,
                count: 1,
                inverse: None,
            }],
            command_filter: HistoryCommandFilter::All,
        };
        let panel = ui_history_panel(&history, "ctrl", false, false).await.expect("an oversized operation line must not fail admission");
        let Component::TreeItem(props) = &panel.children[1].children[0].component else { panic!("expected a TreeItem") };
        let description = props.description.as_ref().expect("clipped description").as_str();
        assert!(description.starts_with("register-mesh vertices=[1.0 "));
        assert!(description.ends_with(UI_TEXT_CLIP_MARK));
    }

    /// 🏷️ A command label is authored copy that can carry the whole argument form of an edit; a folded
    /// row appends ` xN` on top. Both must clip on a char boundary instead of failing the admission of
    /// every panel in the refresh (seen 2026-09-10: `duplicateSelection` on the puzzle 3d Perspective
    /// window failed `refreshUi` with `history-panel.command-label`).
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_clips_an_oversized_command_label_and_its_folded_count() {
        let long_label = format!("Duplicate {}", "seed-left-001 ".repeat(64));
        let entry = |seq: u64, count: u32| CommandView {
            seq,
            action_id: "duplicateSelection".into(),
            label: long_label.clone(),
            kind: ActionKind::Mutation,
            timestamp: "0".into(),
            edit_id: Some("e1".into()),
            config_edit_id: None,
            child_edit_ids: Vec::new(),
            op_lines: Vec::new(),
            applied: true,
            revertible: true,
            count,
            inverse: None,
        };
        let history = HistoryView { columns: Vec::new(), can_undo: true, can_redo: false, active_alternative_id: None, current_checkpoint_id: None, commands: vec![entry(1, 1), entry(2, 3)], command_filter: HistoryCommandFilter::All };
        let panel = ui_history_panel(&history, "ctrl", false, false).await.expect("an oversized command label must not fail admission");
        for (index, expected_tail) in [(0, UI_TEXT_CLIP_MARK), (1, UI_TEXT_CLIP_MARK)] {
            let Component::TreeItem(props) = &panel.children[1].children[index].component else { panic!("expected a TreeItem") };
            let label = props.label.0.as_str();
            assert!(label.starts_with("Duplicate seed-left-001 "), "{label}");
            assert!(label.ends_with(expected_tail), "{label}");
            assert!(label.len() <= UI_TEXT_MAX_BYTES, "{label}");
        }
    }

    /// 🕰️ Wave W-AB: Commands admission is a `UI_BUILT_CHILDREN_MAX`-ary tree over the live filtered
    /// command-row count. A session log past one page must assemble — hops3 aborted every later
    /// publish at `history-panel.commands`. Bound is derived from the fixture's row count, not a
    /// bumped children ceiling. needs #40.
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_pages_command_rows_from_the_live_count() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/history-panel-command-pages/🔣️.json")).unwrap();
        let page_arity = fixture["pageArity"].as_u64().unwrap() as usize;
        assert_eq!(page_arity, UI_BUILT_CHILDREN_MAX, "fixture page arity must be the live BuiltChildren page, not a bumped stand-in");
        let overflow = fixture["overflowPastArity"].as_u64().unwrap() as usize;
        let n = page_arity + overflow;
        assert_eq!(n, fixture["commandRowCount"].as_u64().unwrap() as usize);
        let expected_pages = fixture["expectedCommandSectionChildren"].as_u64().unwrap() as usize;
        assert_eq!(expected_pages, n.div_ceil(page_arity));
        let prefix = fixture["entryKeyPrefix"].as_str().unwrap();
        let entry = |seq: u64| CommandView {
            seq,
            action_id: "hover".into(),
            label: format!("Hover {seq}"),
            kind: ActionKind::Interaction,
            timestamp: "0".into(),
            edit_id: None,
            config_edit_id: None,
            child_edit_ids: Vec::new(),
            op_lines: Vec::new(),
            applied: false,
            revertible: false,
            count: 1,
            inverse: None,
        };
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: false,
            can_redo: false,
            active_alternative_id: None,
            current_checkpoint_id: None,
            commands: (1..=n as u64).map(entry).collect(),
            command_filter: HistoryCommandFilter::All,
        };
        let panel = ui_history_panel(&history, "ctrl", false, false).await.expect("command rows past one page must not fail admission at history-panel.commands");
        assert_eq!(panel.children[1].children.len(), expected_pages, "Commands section children are pages derived from the live row count");
        fn collect_entry_keys(node: &BuiltNode, prefix: &str, keys: &mut Vec<String>) {
            if node.key.as_str().starts_with(prefix) {
                keys.push(node.key.as_str().to_string());
            }
            for child in node.children.iter() {
                collect_entry_keys(child, prefix, keys);
            }
        }
        let mut keys = Vec::new();
        collect_entry_keys(&panel, prefix, &mut keys);
        assert_eq!(keys.len(), n, "every live command row must stay reachable under the paged tree: {keys:?}");
        assert_eq!(panel.children[1].children[0].children.len(), page_arity);
        assert_eq!(panel.children[1].children[1].children.len(), overflow);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_op_less_view_action_is_logged_with_edit_id_none_and_count_one() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta()).await.expect("select");
        let history = app.test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "select");
        assert_eq!(entry.kind, ActionKind::View);
        assert!(entry.edit_id.is_none());
        assert!(entry.config_edit_id.is_some(), "select is a config-op emission");
        assert_eq!(entry.count, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn consecutive_identical_view_dispatches_are_distinct_history_entries() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        for id in ["node-1", "node-2", "node-3"] {
            app.dispatch_typed(TestCommand::Select { id: Some(id.into()) }, &meta()).await.expect("select");
        }
        let history = app.test_history().await;
        assert_eq!(history.commands.len(), 3);
        assert!(history.commands.iter().all(|entry| entry.count == 1));
    }

    #[semio_framework_async_macros::async_test]
    async fn view_dispatches_remain_distinct_across_interleaved_entries() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::Select { id: Some("a".into()) }, &meta()).await.expect("select a");
        app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta()).await.expect("select b");
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("c".into()) }, &meta()).await.expect("select c");
        let history = app.test_history().await;
        assert_eq!(history.commands.len(), 4);
        let select_counts: Vec<u32> = history.commands.iter().filter(|entry| entry.action_id == "select").map(|entry| entry.count).collect();
        assert_eq!(select_counts, vec![1, 1, 1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn note_shell_command_is_intercepted_before_the_app_and_records_each_repeat() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let args = dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme", "detail": "dark" }));
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&args)).await;
        assert!(app.test_app().await.received_actions.borrow().is_empty(), "interception must happen before the app ever sees noteShellCommand");
        let history = app.test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "os.setThemeId");
        assert_eq!(entry.kind, ActionKind::Shell);
        assert!(entry.label.contains("dark"));

        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&args)).await;
        assert!(app.test_app().await.received_actions.borrow().is_empty());
        let history = app.test_history().await;
        assert_eq!(history.commands.len(), 2);
        assert!(history.commands.iter().all(|entry| entry.count == 1));
    }

    #[semio_framework_async_macros::async_test]
    async fn history_delivery_does_not_widen_a_none_ui_scope() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = app.dispatch_typed(TestCommand::ViewNoScope, &meta()).await.expect("viewNoScope");
        assert_eq!(result.ui_scope, UiDirtyScope::None);
    }

    #[semio_framework_async_macros::async_test]
    async fn history_delivery_preserves_partial_ui_scope() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = app.dispatch_typed(TestCommand::ViewPartialScope, &meta()).await.expect("viewPartialScope");
        let UiDirtyScope::Partial { window_bodies, panel_bodies, .. } = result.ui_scope else { panic!("expected a Partial scope") };
        assert_eq!(window_bodies, vec!["some.window".to_string()]);
        assert!(panel_bodies.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn scope_upgrade_full_stays_full() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = app.dispatch_typed(TestCommand::Select { id: Some("x".into()) }, &meta()).await.expect("select");
        assert_eq!(result.ui_scope, UiDirtyScope::Full);
    }

    #[semio_framework_async_macros::async_test]
    async fn benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let before_len = app.test_history().await.commands.len();
        let result = reserved_action(&mut app, "undo", None).await;
        assert_eq!(result.ui_scope, UiDirtyScope::None, "nothing was logged, so the scope must not be upgraded either");
        assert_eq!(app.test_history().await.commands.len(), before_len);
        close_reserved_app(&mut app);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_history_command_filter_is_never_logged() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let before_len = app.test_history().await.commands.len();
        reserved_action(&mut app, SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&dv(json!({ "value": "onlyMutations" })))).await;
        assert_eq!(app.test_history().await.commands.len(), before_len, "the filter's own chrome must not fill the list it filters");
    }

    #[semio_framework_async_macros::async_test]
    async fn rendering_the_history_body_reflects_a_log_only_change_with_no_store_generation_bump() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::default()).await.expect("render before");
        app.dispatch_typed(TestCommand::Select { id: Some("x".into()) }, &meta()).await.expect("select");
        let rendered = app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::default()).await.expect("render after");
        assert_eq!(rendered.root.children.len(), 2, "Actions + Commands");
        assert_eq!(rendered.root.children[1].children.len(), 1, "a log-only cache key change (no store generation bump) must still refresh the rendered panel");
    }

    #[semio_framework_async_macros::async_test]
    async fn an_operation_kind_action_with_zero_operations_still_logs_one_entry() {
        let mut app = contract_app_under_test().await;
        app.dispatch_typed(TestCommand::NoopMutation, &meta()).await.expect("noopMutation");
        let history = app.test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "noopMutation");
        assert_eq!(entry.kind, ActionKind::Mutation);
        assert!(entry.edit_id.is_none(), "no operations means no VCS edit, even though the action is Mutation-kind");
        assert_eq!(entry.count, 1);
    }
    //#endregion 🔖️CommandLogTests

    #[semio_framework_async_macros::async_test]
    async fn undo_on_empty_history_is_a_benign_no_operation() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let result = reserved_action(&mut app, "undo", None).await;
        assert!(result.mutations.is_empty());
        assert!(result.events.is_empty());
        close_reserved_app(&mut app);
    }

    #[semio_framework_async_macros::async_test]
    async fn document_round_trips_through_serialization() {
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("inc");
        app.dispatch_typed(TestCommand::SetLabel { value: "hi".into() }, &meta()).await.expect("label");
        let files = app.document_pack().await.expect("document pack");

        let mut restored = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        restored.load_document_pack(&files).await.expect("load document pack");
        assert_eq!(restored.test_snapshot().await, TestSnapshot { count: 1, label: "hi".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent() {
        let mut sender = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let (near, mut far) = MemoryBackbone::pair("mem://doc", "mem://doc").await;
        sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach");
        sender.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment");

        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
        let operations = protocol::encode_envelopes(&envelopes);

        let mut receiver = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        receiver.ingest_operations(&operations).await.expect("ingest once");
        receiver.ingest_operations(&operations).await.expect("ingest twice");
        assert_eq!(receiver.test_snapshot().await.count, 1, "feeding the same operation twice must not double-apply");
    }

    #[semio_framework_async_macros::async_test]
    async fn attach_detach_reattach_resumes_backbone_convergence() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        assert!(app.backbone_ref().is_none(), "default is unattached");

        let (near, mut far) = MemoryBackbone::pair("mem://reattach", "mem://reattach").await;
        app.attach_backbone(store::Backbones::Memory(near)).await.expect("attach");
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment while attached");
        assert!(!far.receive().await.expect("receive after attach").is_empty(), "attached edits reach the peer");

        app.detach_backbone().await.expect("detach");
        assert!(app.backbone_ref().is_none());
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment while detached");
        assert_eq!(app.test_snapshot().await.count, 2, "detached edits still land on the in-memory graph");
        assert!(far.receive().await.expect("receive while detached").is_empty(), "detached edits never reach the peer");

        let (near_again, mut far_again) = MemoryBackbone::pair("mem://reattach-2", "mem://reattach-2").await;
        app.attach_backbone(store::Backbones::Memory(near_again)).await.expect("re-attach");
        assert!(app.backbone_ref().is_some());
        app.dispatch_typed(TestCommand::Increment, &meta()).await.expect("increment after re-attach");
        assert_eq!(app.test_snapshot().await.count, 3);
        assert!(!far_again.receive().await.expect("receive after re-attach").is_empty(), "re-attaching resumes outbound convergence on the new backbone");
    }

    #[semio_framework_async_macros::async_test]
    async fn selection_count_phrase_formats_mixed_selection() {
        assert_eq!(selection_count_phrase(false, &[(8, "node", "nodes"), (13, "edge", "edges")]), "8 nodes and 13 edges");
        assert_eq!(selection_count_phrase(false, &[(1, "node", "nodes")]), "1 node");
        assert_eq!(selection_count_phrase(true, &[(8, "Knoten", "Knoten"), (13, "Kante", "Kanten")]), "8 Knoten und 13 Kanten");
    }

    /// 🖱️ `PluginApp::context_menu` end-to-end through `VcsArtifactApp`: with an empty label the
    /// "selection guard" (`Menu::when`) drops the gated command; once a label is set (a stand-in for
    /// "something is selected"), both rows resolve label/icon from the declared registry entries.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_resolves_labels_from_the_registry_and_respects_guards() {
        let mut app = contract_app_under_test().await;
        let request = ContextMenuRequest { menu: UiMenuRef { id: "window".into(), args: None }, surface: None, window_instance_id: None, point: None };

        use semio_framework::default_action_icon_id;

        let set_label_icon = default_action_icon_id(ActionKind::Mutation).as_str().to_string();
        let increment_icon = default_action_icon_id(ActionKind::Mutation).as_str().to_string();

        let empty_label = app.context_menu(&request, &ViewModel::default()).await;
        assert_eq!(empty_label.len(), 1, "the gated command must be absent with no label set: {empty_label:?}");
        assert_eq!(empty_label[0], ContextMenuItemSpec { id: "setLabelRequired".into(), label: Some("Set Label".into()), icon: Some(set_label_icon), action: Some("setLabelRequired".into()), ..Default::default() });

        app.dispatch_typed(TestCommand::SetLabel { value: "hi".into() }, &meta()).await.expect("set label");
        let with_label = app.context_menu(&request, &ViewModel::default()).await;
        assert_eq!(with_label.len(), 2, "the guard must open once a label is set: {with_label:?}");
        assert_eq!(with_label[1], ContextMenuItemSpec { id: "incrementViaCommand".into(), label: Some("Increment".into()), icon: Some(increment_icon), action: Some("incrementViaCommand".into()), ..Default::default() });
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
        let items = Menu::of(&registry).action("setLabelRequired").group("export", |m| m.command("incrementViaCommand")).build();
        assert_eq!(items.len(), 2);
        assert_eq!(items[1].id, "menu.group.export");
        assert_eq!(items[1].label, None, "group rows travel with no label — the host resolves it via `ribbon_parent_label`");
        let children: Vec<&str> = items[1].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
        assert_eq!(children, vec!["incrementViaCommand"]);
    }

    /// 🧪️ A registry declaring `setLabelRequired` plus ten `flatLeaf1..10` actions (four carrying a
    /// `RIBBON_PARENT_CATEGORIES` category via `with_category`) — feeds `TestApp::context_menu`'s
    /// `"flat-menu-test"` branch below.
    async fn flat_menu_registry() -> AppActionRegistry {
        let app = App::from_builder(
            App::builder(test_app_surface_id().await, LocalizedLabel::data("FlatMenuTest"))
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
                .await,
        )
        .await;
        AppActionRegistry::from_definition(&app.definition)
    }

    /// 🖱️ End to end through `VcsArtifactApp::context_menu`: a synthetic emitter's 11 flat leaves come
    /// back as 5 primaries + 3 taxonomy-sorted `menu.group.<category>` rows — proving the funnel applies
    /// `organize_context_menu` to every emitter, not just ones that call `Menu::group` themselves.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_funnel_organizes_a_synthetic_apps_flat_overflow_menu() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::with_registry(TestApp::<false>::default(), flat_menu_registry().await).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "flat-menu-test".into() }, &meta()).await.expect("set label");
        let request = ContextMenuRequest { menu: UiMenuRef { id: "window".into(), args: None }, surface: None, window_instance_id: None, point: None };

        let organized = app.context_menu(&request, &ViewModel::default()).await;
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
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_wire_requires_and_forwards_canonical_view_state() {
        assert!(serde_json::from_str::<ContextMenuWireRequest>(r#"{"menu":{"id":"window"}}"#).is_err());
        let wire: ContextMenuWireRequest = serde_json::from_str(r#"{"menu":{"id":"window"},"viewState":{"locale":"de","terminology":"reuse"}}"#).expect("canonical viewState parses");
        let (request, view_state) = wire.into_parts().unwrap();
        assert_eq!(request.menu.id, "window");
        assert_eq!(view_state.locale, crate::Locale::De);
        assert_eq!(view_state.terminology, crate::Terminology::Reuse);
        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
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
        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        let runtime = super::PluginRuntime::new();
        let app = contract_app_under_test().await;
        let cell = std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() }));
        runtime.instances.borrow_mut().insert_admitted(7, cell);
        for surface in fixture["surfaces"].as_array().unwrap() {
            let view = surface["windowId"].as_str().map(|window| host_view.for_window_instance(window).unwrap()).unwrap_or_else(|| host_view.clone());
            super::plugin_mount_surface(&runtime, 7, surface["id"].as_str().unwrap().into(), surface["bodyKey"].as_str().unwrap().into(), &super::encode_wire_serialized(&view)).await.unwrap();
        }
        for surface in fixture["surfaces"].as_array().unwrap() {
            super::plugin_render_surface(&runtime, 7, surface["id"].as_str().unwrap()).await.unwrap();
            let (body, actual) = RENDER_CONTEXT_PROBE.with(|probe| probe.take().unwrap());
            assert_eq!(body, surface["bodyKey"].as_str().unwrap());
            assert_eq!(actual.window_id.as_deref(), surface["windowId"].as_str());
            assert_eq!(actual.active_utility_id.as_deref(), surface["activeUtilityId"].as_str());
            assert_eq!(actual.locale, host_view.locale);
            assert_eq!(actual.terminology, host_view.terminology);
        }
        let hidden = fixture["hidden"].as_str().unwrap();
        super::plugin_hide_surface(&runtime, 7, hidden).await.unwrap();
        assert!(super::plugin_render_surface(&runtime, 7, hidden).await.is_err());
        assert!(super::plugin_render_surface(&runtime, 7, fixture["survivor"].as_str().unwrap()).await.is_ok());
        eprintln!("[DEBUG] real app render receives host preferences and each concrete surface context; hidden surface rendering is rejected");
    }
    //#endregion 🗂️GroupedContextMenu

    #[semio_framework_async_macros::async_test]
    async fn surface_context_refresh_projects_panels_from_focused_window_state() {
        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        let focused = host_view.for_window_instance("right").unwrap();
        let runtime = super::PluginRuntime::new();
        let app = contract_app_under_test().await;
        runtime.instances.borrow_mut().insert_admitted(7, std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() })));
        let response = super::plugin_refresh_ui(&runtime, 7, &serde_json::to_string(&json!({"viewState":focused,"panels":[{"key":"properties","bodyKey":"properties"}]})).unwrap()).await.unwrap();
        let response: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(response["panels"][0]["key"], "properties");
        let (body, actual) = RENDER_CONTEXT_PROBE.with(|probe| probe.take().unwrap());
        assert_eq!(body, "properties");
        assert_eq!(serde_json::to_value(actual).unwrap(), serde_json::to_value(host_view.for_panel()).unwrap());
        eprintln!("[DEBUG] panel refresh clears focused window and utility while preserving host preferences");
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_section_surfaces_render_accessor_maps_instead_of_app_bodies() {
        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let mut host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        host_view.active_tool_id = Some("fill".into());
        let runtime = super::PluginRuntime::new();
        let app = contract_app_under_test().await;
        runtime.instances.borrow_mut().insert_admitted(7, std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() })));
        let mut payloads = BTreeMap::new();
        for section in UiRefreshSection::ALL {
            let surface = format!("7:{}", section.body_key());
            super::plugin_mount_surface(&runtime, 7, surface.clone(), section.body_key().into(), &super::encode_wire_serialized(&host_view)).await.unwrap();
            let (tree, presence) = super::plugin_render_surface(&runtime, 7, &surface).await.unwrap();
            assert!(presence.is_empty());
            assert!(RENDER_CONTEXT_PROBE.with(|probe| probe.take()).is_none(), "a reserved section body key must never reach ArtifactApp::render");
            let projected: Value = serde_json::from_str(&testkit::project_and_retire_fixture_tree(tree).unwrap()).unwrap();
            assert_eq!(projected["key"], section.body_key());
            assert_eq!(projected["component"]["type"], "container");
            let mut payload = String::new();
            let mut frontier = vec![projected.clone()];
            while let Some(node) = frontier.pop() {
                if node["component"]["type"] == "text" {
                    payload.push_str(node["component"]["value"].as_str().unwrap());
                }
                for child in node["children"].as_array().unwrap().iter().rev() {
                    frontier.push(child.clone());
                }
            }
            payloads.insert(section.key(), serde_json::from_str::<Value>(&payload).unwrap());
        }
        assert_eq!(payloads["engagements"], serde_json::json!({}));
        assert_eq!(payloads["measures"]["left"][0]["id"], "left-measure");
        assert_eq!(payloads["measures"]["right"][0]["id"], "right-measure");
        assert_eq!(payloads["tools"]["fill"][0]["id"], "fill-measure");
        eprintln!("[DEBUG] reserved section surfaces rendered {} accessor maps through the retained chunk carrier", payloads.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_section_carrier_pages_a_payload_past_one_node_of_children() {
        let payload = serde_json::to_string(&(0..1_200).map(|index| (format!("window-{index:04}"), format!("measure-{index:04}"))).collect::<BTreeMap<_, _>>()).unwrap();
        assert!(payload.len() > UI_TEXT_MAX_BYTES * UI_BUILT_CHILDREN_MAX);
        let tree = section_component_tree(UiRefreshSection::Measures, &payload).unwrap();
        let projected: Value = serde_json::from_str(&testkit::project_and_retire_fixture_tree(tree).unwrap()).unwrap();
        let mut chunks = Vec::new();
        let mut depth = 0usize;
        let mut frontier = vec![(projected.clone(), 0usize)];
        while let Some((node, level)) = frontier.pop() {
            depth = depth.max(level);
            if node["component"]["type"] == "text" {
                let value = node["component"]["value"].as_str().unwrap();
                assert!(value.len() <= UI_TEXT_MAX_BYTES);
                chunks.push(value.to_string());
                if let Some(attributes) = node["component"]["dataAttributes"].as_object() {
                    let mut keys: Vec<&String> = attributes.keys().collect();
                    keys.sort();
                    for key in keys {
                        let attr = attributes[key].as_str().unwrap();
                        assert!(attr.len() <= UI_TEXT_MAX_BYTES);
                        chunks.push(attr.to_string());
                    }
                }
            }
            assert!(node["children"].as_array().unwrap().len() <= UI_BUILT_CHILDREN_MAX);
            for child in node["children"].as_array().unwrap().iter().rev() {
                frontier.push((child.clone(), level + 1));
            }
        }
        let pack = UI_TEXT_MAX_BYTES * (1 + UI_FIXED_LIST_ITEMS);
        assert_eq!(testkit::fixture_carrier_text(&projected), payload);
        assert_eq!(chunks.concat(), payload);
        let leaves = count_projected_text_leaves(&projected);
        assert!(leaves <= payload.len().div_ceil(pack).max(1) + 2, "packed leaves must stay near ceil(bytes/pack), got {leaves} for {} bytes", payload.len());
        assert!(count_projected_nodes(&projected) <= semio_framework_ui_contract::UI_DOCUMENT_NODES, "a reserved measures carrier must fit the document node table");
        eprintln!("[DEBUG] section carrier packed {} bytes into {} text leaves at depth {depth} ({} nodes)", payload.len(), chunks.len(), count_projected_nodes(&projected));
    }

    /// 🧩️ Pins the pack this producer writes against the language-neutral fixture the TypeScript
    /// reader (`sectionValueFromBuiltNode` in `📺️renderer/…/🔌️PluginRuntime/🟦️.tsx`) reassembles from —
    /// a reader that read only `value` handed `JSON.parse` an exact 512-byte prefix, see ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END `📓️json-512-truncation-2026-09-10.md`.
    #[semio_framework_async_macros::async_test]
    async fn packed_section_carrier_matches_the_neutral_fixture_slices() {
        let fixture: Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🔬️paged-text-carrier/🔣️.json")).unwrap();
        assert_eq!(fixture["textMaxBytes"].as_u64().unwrap() as usize, UI_TEXT_MAX_BYTES);
        assert_eq!(fixture["packSlices"].as_u64().unwrap() as usize, 1 + UI_FIXED_LIST_ITEMS);
        let payload = fixture["payload"].as_str().unwrap();
        assert_eq!(payload.len(), fixture["payloadBytes"].as_u64().unwrap() as usize);
        let slices: Vec<&str> = fixture["slices"].as_array().unwrap().iter().map(|slice| slice.as_str().unwrap()).collect();
        assert_eq!(slices.concat(), payload);
        assert!(slices.len() > 1 && slices.len() <= 1 + UI_FIXED_LIST_ITEMS, "the fixture must be one packed leaf past a single slice");
        let section = UiRefreshSection::from_body_key(fixture["rootKey"].as_str().unwrap()).unwrap();
        let projected: Value = serde_json::from_str(&testkit::project_and_retire_fixture_tree(section_component_tree(section, payload).unwrap()).unwrap()).unwrap();
        assert_eq!(projected["key"].as_str().unwrap(), fixture["rootKey"].as_str().unwrap());
        assert_eq!(count_projected_text_leaves(&projected), 1, "a payload inside one pack must ride on exactly one leaf");
        let leaf = &projected["children"][0];
        assert_eq!(leaf["component"]["value"].as_str().unwrap(), slices[0]);
        let attributes = leaf["component"]["dataAttributes"].as_object().unwrap();
        assert_eq!(attributes.len(), slices.len() - 1);
        for (offset, slice) in slices.iter().enumerate().skip(1) {
            assert_eq!(attributes[&format!("{offset:02}")].as_str().unwrap(), *slice);
        }
        assert_eq!(testkit::fixture_carrier_text(&projected), payload);
        eprintln!("[DEBUG] neutral fixture pinned: {} bytes packed into 1 value slice and {} dataAttributes slices", payload.len(), slices.len() - 1);
    }

    //#region 🚚️World3dSceneLaneCarriers

    /// 🧱 Presented-node census of a projected builder tree (surface + every descendant).
    fn count_projected_nodes(node: &Value) -> usize {
        1 + node["children"].as_array().map(|children| children.iter().map(count_projected_nodes).sum()).unwrap_or(0)
    }

    fn count_projected_text_leaves(node: &Value) -> usize {
        let here = usize::from(node["component"]["type"] == "text");
        here + node["children"].as_array().map(|children| children.iter().map(count_projected_text_leaves).sum()).unwrap_or(0)
    }

    /// 🚚️ Projects one built surface node and returns `(projection, lane subtree by carrier key)`.
    fn project_scene_surface(node: crate::app::BuiltNode) -> (Value, BTreeMap<String, Value>) {
        let projection: Value = serde_json::from_str(&testkit::project_and_retire_fixture_tree(crate::app::built_to_component_tree(node)).unwrap()).unwrap();
        let lanes = projection["children"]
            .as_array()
            .unwrap()
            .iter()
            .map(|child| (child["key"].as_str().unwrap().to_string(), child.clone()))
            .collect();
        (projection, lanes)
    }

    /// 🚚️ Asserts one lane carrier obeys the contract's own bounds and reproduces `payload` exactly.
    fn assert_lane_carrier(carrier: &Value, payload: &str) -> (usize, usize) {
        let mut leaves = 0usize;
        let mut depth = 0usize;
        let mut frontier = vec![(carrier, 0usize)];
        while let Some((node, level)) = frontier.pop() {
            assert!(node["children"].as_array().unwrap().len() <= UI_BUILT_CHILDREN_MAX);
            if node["component"]["type"] == "text" {
                leaves += 1;
                depth = depth.max(level);
                assert!(node["component"]["value"].as_str().unwrap().len() <= UI_TEXT_MAX_BYTES);
            }
            for child in node["children"].as_array().unwrap().iter().rev() {
                frontier.push((child, level + 1));
            }
        }
        assert_eq!(testkit::fixture_carrier_text(carrier), payload);
        (leaves, depth)
    }

    /// 🌍️ A world-3d scene whose payload is far past the 32 KiB `UiFixedBytes` doc ceiling.
    fn oversized_world_scene(camera: &str, selection: &str) -> semio_framework_ui_scene::World3dScene {
        let instances = serde_json::to_string(
            &(0..600)
                .map(|index| json!({"id": format!("capsule-{index:04}"), "meshId": "box", "position": [index as f64, 0.0, 0.0], "rotation": [0.0, 0.0, 0.0, 1.0], "scale": [1.0, 1.0, 1.0]}))
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let mut scene = semio_framework_ui_scene::World3dScene::base(camera.into(), r#"[{"id":"box","kind":"box"}]"#.into(), instances, selection.into());
        scene.vortices_json = Some(serde_json::to_string(&(0..40).map(|index| json!({"id": format!("vortex-{index}"), "position": [0.0, index as f64, 0.0]})).collect::<Vec<_>>()).unwrap());
        scene.lod_json = Some(r#"{"maxInstances":8000}"#.into());
        scene.chunking_json = Some(r#"{"chunkSize":64,"radius":8000}"#.into());
        scene.domain_id = Some("puzzle3d".into());
        scene
    }

    #[semio_framework_async_macros::async_test]
    async fn world3d_scene_surface_pages_every_lane_beside_a_spine_that_fits_the_fixed_doc() {
        let scene = oversized_world_scene("{}", r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#);
        let assembled_bytes = <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::encode_pack(&scene).unwrap().len();
        assert!(assembled_bytes > semio_framework_ui_contract::UI_FIXED_BYTES, "the unsplit scene must be past the fixed doc ceiling to prove anything");

        let node = crate::app::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &scene).unwrap();
        let (projection, lanes) = project_scene_surface(node);

        let doc_bytes = projection["component"]["doc"]["bytes"].as_array().unwrap().len();
        assert!(doc_bytes <= semio_framework_ui_contract::UI_FIXED_BYTES);

        let (spine, expected) = semio_framework_ui_scene::SceneDoc::split_lanes(&scene);
        assert_eq!(lanes.len(), expected.len());
        let mut total = 0usize;
        for lane in &expected {
            let carrier = lanes.get(lane.key).unwrap_or_else(|| panic!("lane {} publishes a carrier", lane.key));
            assert!(semio_framework_ui_scene::World3dSceneLane::from_body_key(lane.key).is_some());
            assert_lane_carrier(carrier, &lane.payload);
            total += lane.payload.len();
        }
        for reference in &spine.lanes {
            let lane = semio_framework_ui_scene::World3dSceneLane::from_name(&reference.lane).unwrap();
            assert_eq!(reference.bytes as usize, expected.iter().find(|entry| entry.key == lane.body_key()).unwrap().payload.len());
            assert_eq!(reference.hash, semio_framework_ui_scene::world3d_scene_lane_hash(&expected.iter().find(|entry| entry.key == lane.body_key()).unwrap().payload));
        }

        let mut reassembled: semio_framework_ui_scene::World3dScene = testkit::decode_fixture_scene_with_lanes(&serde_json::to_string(&projection).unwrap()).unwrap();
        reassembled.lanes = Vec::new();
        assert_eq!(reassembled, scene);
        eprintln!("[DEBUG] world-3d scene of {assembled_bytes} packed bytes published a {doc_bytes}-byte spine plus {} lane carriers holding {total} payload bytes", lanes.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn world3d_scene_surface_republishes_only_the_lanes_that_changed() {
        let selection = r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#;
        let (_, first) = project_scene_surface(crate::app::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &oversized_world_scene("{}", selection)).unwrap());

        let (moved_projection, moved) = project_scene_surface(crate::app::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &oversized_world_scene(r#"{"position":[9,9,9]}"#, selection)).unwrap());
        assert_eq!(moved, first, "a camera move must leave every lane carrier byte-identical");

        let picked = r#"{"method":"rectangle","mode":"replace","ids":["capsule-0007"],"hoveredId":null}"#;
        let (picked_projection, picked_lanes) = project_scene_surface(crate::app::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &oversized_world_scene("{}", picked)).unwrap());
        let changed: Vec<&String> = picked_lanes.keys().filter(|key| picked_lanes.get(*key) != first.get(*key)).collect();
        assert_eq!(changed, vec![semio_framework_ui_scene::World3dSceneLane::Selection.body_key()]);
        assert_ne!(picked_projection["component"], moved_projection["component"], "a changed lane must still move the spine so its consumer re-reads");
        eprintln!("[DEBUG] a camera move republished 0 of {} lanes; a selection edit republished exactly {}", first.len(), changed.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn world3d_scene_surface_pages_one_oversized_lane_instead_of_faulting() {
        let instances = serde_json::to_string(&(0..3_000).map(|index| json!({"id": format!("capsule-{index:05}"), "meshId": "box"})).collect::<Vec<_>>()).unwrap();
        assert!(instances.len() > UI_TEXT_MAX_BYTES * UI_BUILT_CHILDREN_MAX, "one lane must be past a single carrier level to prove paging");
        let scene = semio_framework_ui_scene::World3dScene::base("{}".into(), "[]".into(), instances.clone(), "{}".into());
        let (_, lanes) = project_scene_surface(crate::app::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &scene).unwrap());
        let (leaves, depth) = assert_lane_carrier(lanes.get(semio_framework_ui_scene::World3dSceneLane::Instances.body_key()).unwrap(), &instances);
        let pack = UI_TEXT_MAX_BYTES * (1 + UI_FIXED_LIST_ITEMS);
        assert!(leaves <= instances.len().div_ceil(pack).max(1) + 2, "packed instances leaves must stay near ceil(bytes/pack), got {leaves} for {} bytes", instances.len());
        eprintln!("[DEBUG] one {}-byte instances lane packed into {leaves} text leaves at depth {depth}", instances.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn a_scene_that_declares_no_lanes_still_publishes_one_childless_surface() {
        let scene = semio_framework_ui_scene::TableScene::base("[]", "[]");
        let (projection, lanes) = project_scene_surface(crate::app::scene_surface("results", semio_framework_ui_contract::SurfaceKind::Table, &scene).unwrap());
        assert!(lanes.is_empty());
        assert_eq!(projection["component"]["type"], "surface");
        assert_eq!(testkit::decode_fixture_scene_with_lanes::<semio_framework_ui_scene::TableScene>(&serde_json::to_string(&projection).unwrap()).unwrap(), scene);
    }

    #[semio_framework_async_macros::async_test]
    async fn nakagin_scale_world3d_surface_fits_document_node_cap() {
        let scene = oversized_world_scene("{}", r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#);
        let node = crate::app::scene_surface("viewport", semio_framework_ui_contract::SurfaceKind::World3d, &scene).unwrap();
        let (projection, _) = project_scene_surface(node);
        let nodes = count_projected_nodes(&projection);
        assert!(nodes <= semio_framework_ui_contract::UI_DOCUMENT_NODES, "Nakagin-scale world-3d surface presented {nodes} nodes over UI_DOCUMENT_NODES");
        eprintln!("[DEBUG] Nakagin-scale world-3d surface presented {nodes} nodes (cap {})", semio_framework_ui_contract::UI_DOCUMENT_NODES);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_100kib_measures_section_fits_document_node_cap() {
        let payload = serde_json::to_string(&(0..2_400).map(|index| (format!("window-{index:04}"), format!("measure-{index:04}"))).collect::<BTreeMap<_, _>>()).unwrap();
        assert!(payload.len() > UI_TEXT_MAX_BYTES * 128, "unpacked 512-byte leaves of this payload would overflow UI_DOCUMENT_NODES");
        let tree = section_component_tree(UiRefreshSection::Measures, &payload).unwrap();
        let projected: Value = serde_json::from_str(&testkit::project_and_retire_fixture_tree(tree).unwrap()).unwrap();
        let nodes = count_projected_nodes(&projected);
        assert!(nodes <= semio_framework_ui_contract::UI_DOCUMENT_NODES, "a 100 KiB measures section presented {nodes} nodes over UI_DOCUMENT_NODES");
        eprintln!("[DEBUG] 100 KiB measures section presented {nodes} nodes for {} bytes", payload.len());
    }

    //#endregion 🚚️World3dSceneLaneCarriers

    #[semio_framework_async_macros::async_test]
    async fn surface_context_presence_targets_each_concrete_surface() {
        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        let mut app = interaction_app_under_test().await;
        let mut peers = PeerPresenceRoot::empty();
        peers.insert(fixture["presence"]["actor"].as_str().unwrap().into(), PeerPresence {
            color: Some(3), surface: None,
            interaction: Some(PresenceInteraction {
                app_id: "s.test.synthetic@1/*#editor".into(),
                domains: vec![PresenceDomain { domain: fixture["presence"]["domainId"].as_str().unwrap().into(), granularity: "item".into(), selected: vec![fixture["presence"]["nodeKey"].as_str().unwrap().into()], hovered: Vec::new() }],
            }),
        }).unwrap();
        *app.peer_presence = std::sync::Arc::new(peers);
        let runtime = super::PluginRuntime::new();
        runtime.instances.borrow_mut().insert_admitted(7, std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() })));
        for surface in fixture["surfaces"].as_array().unwrap() {
            let id = surface["id"].as_str().unwrap();
            let view = surface["windowId"].as_str().map(|window| host_view.for_window_instance(window).unwrap()).unwrap_or_else(|| host_view.for_panel());
            super::plugin_mount_surface(&runtime, 7, id.into(), surface["bodyKey"].as_str().unwrap().into(), &super::encode_wire_serialized(&view)).await.unwrap();
            let (_, presence) = super::plugin_render_surface(&runtime, 7, id).await.unwrap();
            assert_eq!(presence.len(), 1);
            let mut expected = fixture["presence"]["expected"].clone();
            expected["surface"] = surface["id"].clone();
            assert_eq!(serde_json::to_value(&presence[0]).unwrap(), expected);
        }
        eprintln!("[DEBUG] surface context presence follows each concrete window and panel without sibling leakage");
    }

    #[semio_framework_async_macros::async_test]
    async fn view_action_emitting_ops_is_rejected() {
        let mut app = contract_app_under_test().await;
        let error = app.dispatch_typed(TestCommand::BadView, &meta()).await.expect_err("a View command emitting operations must be rejected");
        assert!(error.message.contains("must not emit operations"), "unexpected error: {}", error.message);
        assert_eq!(app.test_snapshot().await, TestSnapshot::default());
    }

    //#region 🧪️IntentDispatchTests
    /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`) acceptance: an Activate intent on a test app
    /// produces the mutation's document change AND a command-log entry within the SAME
    /// `handle_intent_frame` call — the reactor's own `poll` never needs a second turn to see it,
    /// since `plugin_dispatch_intents` (which calls this) runs before the SAME turn's
    /// `dirty_render` pass.
    #[semio_framework_async_macros::async_test]
    async fn activate_intent_dispatches_through_the_typed_command_path_same_turn() {
        let mut app = contract_app_under_test().await;
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
        let result = app.handle_intent_frame(&intent, &meta()).await.expect("an Activate intent on a Mutation-kind action must dispatch");
        assert_eq!(app.test_snapshot().await.count, 1, "the mutation must have applied");
        assert_eq!(result.mutations.len(), 1, "the returned invocation carries the one mutation the intent caused");
        assert!(result.history_patch.is_some(), "a command-log entry must have been recorded for the intent");
    }

    /// 🎯️ M1 acceptance: a `View`-kind action arriving as an intent and returning artifact ops
    /// still hard-faults — kind discipline survives the new intent path unchanged, because
    /// `handle_intent_frame` routes through the SAME `dispatch_typed_command_inner` every other
    /// command path uses.
    #[semio_framework_async_macros::async_test]
    async fn view_kind_intent_returning_operations_hard_faults() {
        let mut app = contract_app_under_test().await;
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
        let error = app.handle_intent_frame(&intent, &meta()).await.expect_err("a View-kind intent emitting operations must be rejected");
        assert!(error.message.contains("must not emit operations"), "unexpected error: {}", error.message);
        assert_eq!(app.test_snapshot().await, TestSnapshot::default(), "kind discipline must block the mutation");
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
    //#endregion 🧪️IntentDispatchTests

    #[semio_framework_async_macros::async_test]
    async fn set_active_utility_carries_its_value_directly_and_emits_no_operations() {
        let mut app = contract_app_under_test().await;
        let result = app.dispatch_typed(TestCommand::SetActiveUtility { utility_id: "brush".into() }, &meta()).await.expect("setActiveUtility is a valid View command");
        assert!(result.mutations.is_empty(), "utility switching must not create history");
        let event = result.events.iter().find(|event| event.kind == "active-utility").expect("echoed active utility");
        assert_eq!(event.payload, dsl::DslValue::from(json!({ "utilityId": "brush" })));
    }

    #[semio_framework_async_macros::async_test]
    async fn action_emit_amend_coalesces_while_commit_does_not() {
        let mut app = contract_app_under_test().await;
        for value in ["a", "ab", "abc"] {
            app.dispatch_typed(TestCommand::AmendLabel { value: value.into() }, &meta()).await.expect("amendLabel");
        }
        assert_eq!(app.test_snapshot().await.label, "abc");
        // One undo reverts the whole coalesced amend gesture.
        reserved_action(&mut app, "undo", None).await;
        assert_eq!(app.test_snapshot().await.label, "");

        for value in ["x", "xy"] {
            app.dispatch_typed(TestCommand::CommitLabel { value: value.into() }, &meta()).await.expect("commitLabel");
        }
        assert_eq!(app.test_snapshot().await.label, "xy");
        // Each commit is its own edit: one undo only reverts the last commit.
        reserved_action(&mut app, "undo", None).await;
        assert_eq!(app.test_snapshot().await.label, "x");
    }

    #[semio_framework_async_macros::async_test]
    async fn amend_dispatch_reports_only_this_dispatch_new_operations() {
        // 🪢️ Regression guard for `result_from_last_edit`'s `tail_offset` slicing: even though the
        // coalesced edit accumulates every amend's operations (3 after this loop), each dispatch's
        // `InvocationResult` must report only the operation IT just added — never re-serializing the whole
        // growing edit into every `KernelMutation`/`UndoGroup` on every single dispatch.
        let mut app = contract_app_under_test().await;
        app.dispatch_typed(TestCommand::AmendLabel { value: "a".into() }, &meta()).await.expect("amendLabel a");
        app.dispatch_typed(TestCommand::AmendLabel { value: "ab".into() }, &meta()).await.expect("amendLabel ab");
        let result = app.dispatch_typed(TestCommand::AmendLabel { value: "abc".into() }, &meta()).await.expect("amendLabel abc");
        assert_eq!(result.mutations.len(), 1, "must report only this dispatch's new operation, not the whole coalesced edit");
        assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetLabel(SetLabel { value: "abc".into() })).unwrap());
        assert_eq!(
            result.mutations[0].inverse.inverse_diff.payload,
            protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetLabel(SetLabel { value: "ab".into() })).unwrap()]),
            "the new operation's own inverse undoes back to the pre-dispatch label, not the whole gesture"
        );
        assert_eq!(result.inverse_group.mutations.len(), 1);
        assert_eq!(result.inverse_group.inverse_mutations.len(), 1);
        assert_eq!(app.test_snapshot().await.label, "abc");
        // The narrowed per-dispatch reporting must not affect coalescing/undo semantics.
        reserved_action(&mut app, "undo", None).await;
        assert_eq!(app.test_snapshot().await.label, "");
    }

    #[semio_framework_async_macros::async_test]
    async fn operation_command_emits_kernel_op_with_true_inverse() {
        let mut app = contract_app_under_test().await;
        let result = app.dispatch_typed(TestCommand::IncrementViaCommand, &meta()).await.expect("incrementViaCommand");
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 1 })).unwrap());
        assert_eq!(result.mutations[0].inverse.inverse_diff.payload, protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 0 })).unwrap()]));
        assert_eq!(app.test_snapshot().await.count, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_command_dispatch_validates_structural_app_ownership() {
        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
        let mut app = contract_app_under_test().await;
        let valid = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into() }, command_id: "incrementViaCommand".into() }, arguments: Default::default() };
        app.handle_command(&valid, Some("edit"), &meta()).await.expect("app-owned command");
        assert_eq!(app.test_snapshot().await.count, 1);
        let unknown = CommandInvocation { address: CommandAddress { command_id: "nope".into(), ..valid.address }, arguments: Default::default() };
        let error = app.handle_command(&unknown, Some("edit"), &meta()).await.expect_err("undeclared app command");
        assert!(error.message.contains("not owned by app"), "unexpected error: {}", error.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_dispatch_backstop_rejects_every_non_migrated_action_and_command() {
        use semio_framework::InteractiveJobClassification::{BatchOnlyPendingRewrite, Deleted, ForbiddenFromUi, Unclassified};
        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};

        for classification in [Unclassified, BatchOnlyPendingRewrite, ForbiddenFromUi, Deleted] {
            let mut action_registry = contract_registry().await;
            action_registry.test_set_action_classification("badView", classification);
            let mut action_app = VcsArtifactApp::<TestApp>::with_registry(TestApp::<false>::default(), action_registry).await;
            assert!(!action_app.test_registered_tool_keys().iter().any(|key| key.1 == "badView"));
            let action_error = action_app.dispatch_typed(TestCommand::BadView, &meta()).await.expect_err("non-migrated action must be rejected before its handler runs");
            assert_eq!(action_error.code.0, "interactive-job.not-ui-safe");
            assert_eq!(action_app.test_snapshot().await.count, 0);

            let mut command_registry = contract_registry().await;
            command_registry.test_set_app_command_classification("incrementViaCommand", classification);
            let mut command_app = VcsArtifactApp::<TestApp>::with_registry(TestApp::<false>::default(), command_registry).await;
            assert!(!command_app.test_registered_tool_keys().iter().any(|key| key.1 == "incrementViaCommand"));
            let invocation =
                CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into() }, command_id: "incrementViaCommand".into() }, arguments: Default::default() };
            let command_error = command_app.handle_command(&invocation, Some("edit"), &meta()).await.expect_err("non-migrated command must be rejected before its handler runs");
            assert_eq!(command_error.code.0, "interactive-job.not-ui-safe");
            assert_eq!(command_app.test_snapshot().await.count, 0);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_mode_command_requires_the_active_structural_owner() {
        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
        let invocation = CommandInvocation {
            address: CommandAddress { owner: CommandOwnerAddress::Mode { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into(), mode_id: "edit".into() }, command_id: "mode.increment".into() },
            arguments: Default::default(),
        };
        let mut app = contract_app_under_test().await;
        app.handle_command(&invocation, Some("edit"), &meta()).await.expect("active mode-owned command");
        assert_eq!(app.test_snapshot().await.count, 1);
        let error = app.handle_command(&invocation, None, &meta()).await.expect_err("inactive mode command");
        assert!(error.message.contains("not owned by active mode edit"), "unexpected error: {}", error.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn addressed_window_action_injects_the_exact_window_instance_into_the_typed_handler() {
        use semio_framework::manifest::{ActionAddress, ActionInvocation};
        let invocation = ActionInvocation {
            address: ActionAddress { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into(), mode_id: "edit".into(), window_kind_id: "main".into(), window_instance_id: "main-instance-2".into(), action_id: "targetWindow".into() },
            arguments: Default::default(),
        };
        let mut app = contract_app_under_test().await;
        app.handle_action_invocation(&invocation, Some("edit"), &meta()).await.expect("addressed window action");
        assert_eq!(app.test_snapshot().await.label, "main-instance-2");
    }

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
                    output: dsl::DslValue::Null,
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
        let invocation = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::Plugin { plugin_id: "fixture".into() }, command_id: "refresh".into() }, arguments: Default::default() };
        plugin.handle_plugin_command(&invocation, &ActionMeta { actor: "a".into(), instance_id: 1, view_state: None }).expect("first app instance");
        plugin.handle_plugin_command(&invocation, &ActionMeta { actor: "b".into(), instance_id: 2, view_state: None }).expect("second app instance");
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn command_op_records_history_exactly_like_an_operation_action() {
        let mut app = contract_app_under_test().await;
        app.dispatch_typed(TestCommand::IncrementViaCommand, &meta()).await.expect("inc");
        app.dispatch_typed(TestCommand::IncrementViaCommand, &meta()).await.expect("inc");
        assert_eq!(app.test_snapshot().await.count, 2);
        reserved_action(&mut app, "undo", None).await;
        assert_eq!(app.test_snapshot().await.count, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_less_construction_rejects_before_the_reducer() {
        let mut app: VcsArtifactApp<TestApp> = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;
        let error = app.dispatch_typed(TestCommand::BadView, &meta()).await.expect_err("an empty registry must fail closed");
        assert_eq!(error.code.0, "interactive-job.unknown-key");
        assert_eq!(app.test_snapshot().await, TestSnapshot::default());
    }

    //#region 🔖️InteractionDispatchTests
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_replace_persists_through_the_interaction_store() {
        let mut app = interaction_app_under_test().await;
        // 🕹️ `interaction_topology` requires a non-empty `label` for "item-1" to exist.
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta()).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))).await;
        let selection = app.interaction_state().await.selection.get("items").cloned().expect("items domain selected");
        assert_eq!(selection.ids, vec!["item-1".to_string()]);
        assert_eq!(selection.granularity, "item");
    }

    #[semio_framework_async_macros::async_test]
    async fn interaction_hover_is_ephemeral_and_never_touches_the_persisted_interaction_store() {
        let mut app = interaction_app_under_test().await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta()).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))).await;
        let edits_after_select = app.interaction_store.envelope().vcs.edits.len();

        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1"))).await;

        assert_eq!(app.interaction_store.envelope().vcs.edits.len(), edits_after_select, "hover must never mint a persisted interaction_store edit");
        assert_eq!(app.interaction_state().await.hover.get("items").map(|hover| hover.ids.clone()), Some(vec!["item-1".to_string()]));

        // 🐁️ Empty targets clears the channel (see `next_hover`'s "empty batch clears" law).
        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&dv(json!({ "domainId": "items", "channel": "pointer", "targets": "[]" })))).await;
        assert!(app.interaction_state().await.hover.get("items").is_none(), "an emptied hover channel is removed, not left as an empty entry");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_pick_is_never_undoable_the_default_undo_only_ever_walks_the_document_store() {
        let mut app = interaction_app_under_test().await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta()).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))).await;
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]));

        // 🕰️ The framework-injected "undo" action only ever dispatches against `self.store` (the
        // DOCUMENT store) — with only the label-seed edit on it, one undo reverts THAT, not the pick.
        reserved_action(&mut app, "undo", None).await;
        assert_eq!(app.test_snapshot().await.label, "", "undo must revert the document edit (seeding the label)");
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "the pick itself must survive an unrelated document undo — lane discipline");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_selection_mode_and_set_interaction_granularity_persist_immediately() {
        let mut app = interaction_app_under_test().await;
        reserved_action(&mut app, SET_SELECTION_MODE_ACTION_ID, Some(&dv(json!({ "domainId": "items", "mode": "single" })))).await;
        assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(SelectionMode::Single));

        reserved_action(&mut app, SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&dv(json!({ "domainId": "items", "granularityId": "item" })))).await;
        assert_eq!(app.interaction_state().await.active_granularity.get("items").map(String::as_str), Some("item"));

        // 🛂️ An undeclared granularity is rejected, not silently accepted.
        let error = app.handle_action(SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&dv(json!({ "domainId": "items", "granularityId": "bogus" }))), &meta()).await.expect_err("undeclared granularity must be rejected");
        assert!(error.message.contains("bogus"), "unexpected error: {}", error.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn clear_selection_and_select_all_apply_across_every_declared_domain() {
        let mut app = interaction_app_under_test().await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta()).await.expect("seed label");

        reserved_action(&mut app, SELECT_ALL_ACTION_ID, None).await;
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "selectAll must select every id `interaction_topology` reports for the declared granularity");

        reserved_action(&mut app, CLEAR_SELECTION_ACTION_ID, None).await;
        assert!(app.interaction_state().await.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "clearSelection must empty every declared domain's selection");
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_state_prunes_a_stale_selection_id_after_the_document_deletes_it() {
        let mut app = interaction_app_under_test().await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta()).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))).await;
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]));

        // 🧹️ `TestApp::interaction_topology` reports NO ids once `label` is empty again — simulates
        // "item-1 was deleted from the document" — task 4: revalidated after EVERY artifact dispatch.
        app.dispatch_typed(TestCommand::SetLabel { value: "".into() }, &meta()).await.expect("delete item-1 (empty label)");

        assert!(app.interaction_state().await.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "the deleted id must be pruned from selection automatically");
    }

    #[semio_framework_async_macros::async_test]
    async fn interaction_verbs_are_recorded_under_the_interaction_action_kind() {
        let mut app = interaction_app_under_test().await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta()).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))).await;
        let history = app.test_history().await;
        let row = history.commands.first().expect("one logged row");
        assert_eq!(row.action_id, INTERACTION_SELECT_ACTION_ID);
        assert_eq!(row.kind, ActionKind::Interaction);
        assert!(!row.revertible, "an Interaction-kind row carries no edit/config_edit/inverse — never revertible");
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
            let builder = resolve_ready(__base.interaction(InteractionDefinition {
                id: "items".into(),
                label: LocalizedLabel::data("Items"),
                granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
            }));
            resolve_ready(App::from_builder(builder))
        });
        assert!(outcome.is_err(), "build_definition must reject transitive hover paired with HierarchyProvider::Flat");
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_tree_stamping_caches_interaction_topology_from_a_domain_bound_tree() {
        let mut app = interaction_app_under_test().await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta()).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))).await;

        // 👥️ Contract-freeze §C7.6 peer setup — M2 (ticket 26/08/17 `design-unified.md`) makes
        // this the real presence-derivation fixture the prior packet's own gap note anticipated.
        app.own_color = Some(9);
        let mut peer_presence = PeerPresenceRoot::empty();
        peer_presence
            .insert(
                "user:alice#s1".to_string(),
                PeerPresence {
                    color: Some(3),
                    surface: None,
                    interaction: Some(PresenceInteraction {
                        app_id: "s.test.synthetic@1/*#editor".to_string(),
                        domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item-1".to_string()], hovered: Vec::new() }],
                    }),
                },
            )
            .expect("peer fixture fits fixed root");
        let previous = std::mem::replace(&mut *app.peer_presence, std::sync::Arc::new(peer_presence));
        assert!(previous.is_empty());
        drop(previous);

        // 👥️ M2: `stamp_and_cache_interaction_ui` no longer writes selection/hover back onto the
        // tree itself (`TreeNode`/`Component::TreeItem` still carry no presence field — see that
        // method's own doc comment for why "the framework wins" onto the tree is gone for good);
        // it derives `ui_contract::PresenceUpdate`s into `self.pending_presence` instead, asserted
        // below.
        let item = TreeNode::try_new(
            "item-1",
            Component::TreeItem(TreeItemProps {
                label: Label(UiText::try_from_str("Item 1").expect("bounded fixture")),
                description: None,
                icon: None,
                default_open: None,
                draggable: None,
                drag_data: None,
                dimmed: None,
                row_actions: UiFixedList::default(),
            }),
        )
        .expect("bounded fixture");
        let section = TreeNode::try_new("sec", Component::TreeSection(TreeSectionProps { label: None, default_open: None })).expect("bounded fixture").try_with_children([item]).unwrap_or_else(|_| panic!("bounded fixture"));
        let root = TreeNode::try_new("root", Component::Tree(TreeProps { interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }))
            .expect("bounded fixture")
            .try_with_children([section])
            .unwrap_or_else(|_| panic!("bounded fixture"));
        let tree = ComponentTree { root };
        let state = app.interaction_state().await;
        app.stamp_and_cache_interaction_ui(&tree, &state, "window").await.expect("bounded fixture");
        let topology = app.interaction_ui_topology.get("items").expect("topology cached for the items domain");
        assert_eq!(topology.ordered.len(), 1);
        assert_eq!(topology.ordered[0].id, "item-1");

        // 👥️ M2 acceptance: selecting item-1 (own) with alice ALSO selecting it derives exactly
        // one `PresenceUpdate` for that node — own.selected true, one peer mark, own color
        // threaded through, with the app-local body key before runtime surface binding.
        assert_eq!(app.pending_presence.len(), 1, "expected exactly one dirty presence key, got {:?}", app.pending_presence);
        let update = &app.pending_presence[0];
        assert_eq!(update.surface, semio_framework_ui_contract::SurfaceId::try_from("window").expect("bounded fixture"));
        assert_eq!(update.node_key, "item-1");
        assert!(update.own.selected, "own selection must be reported");
        assert!(!update.own.hovered);
        assert_eq!(update.own.color, Some(9));
        assert_eq!(update.peers.len(), 1);
        assert_eq!(update.peers[0].actor, "user:alice#s1");
        assert_eq!(update.peers[0].color, Some(3));
        assert!(update.peers[0].selected);
        assert!(!update.peers[0].hovered);
    }

    /// 🧪 W-G3 §8.21 — `interactionSelect` leftover `Invocation.output` carries InteractionView selected ids + lock.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_job_completion_publishes_interaction_view_on_leftover() {
        let mut app = interaction_app_under_test().await;
        let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1"))).await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let ids = view.get("selectedIds").and_then(DslValue::as_array).expect("selectedIds");
        assert!(ids.iter().any(|id| id.as_str() == Some("item-1")), "leftover selected ids {ids:?}");
        let locked = view.get("locked").expect("lock state included on leftover InteractionView");
        assert_eq!(locked.get("item-1").and_then(DslValue::as_bool), Some(false));
        let gumball = view.get("gumball").expect("gumball leftover");
        assert_eq!(gumball.get("active").and_then(DslValue::as_bool), Some(true));
        assert_eq!(gumball.get("anchorId").and_then(DslValue::as_str), Some("item-1"));
        close_reserved_app(&mut app);
    }

    /// 🧪 W-G3 §8.21 — `interactionHover` leftover publishes the hover target on the same leftover output.
    #[semio_framework_async_macros::async_test]
    async fn interaction_hover_job_completion_publishes_hover_target_on_leftover() {
        let mut app = interaction_app_under_test().await;
        let settled = reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1"))).await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let hover = view.get("hoverTarget").expect("hover target");
        assert_eq!(hover.get("id").and_then(DslValue::as_str), Some("item-1"));
        assert_eq!(hover.get("domain").and_then(DslValue::as_str), Some("items"));
        close_reserved_app(&mut app);
    }
    //#endregion 🔖️InteractionDispatchTests

    //#region 🔖️AsyncTaskTests
    // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `⚛️reactor::poll`'s real
    // event routing (`Event::Completed`, `run_until_idle`, `drain_task_resumes`) is
    // wasm32-wasip2-only (`wit_bridge`) and cannot run under a native `cargo test`. These tests
    // exercise the SAME underlying primitives `poll` itself drives — `⚛️reactor::spawn_task`,
    // `⚛️reactor::test_support::{run_until_idle, resolve_request, pop_task_resume, ...}`, and
    // `crate::plugin_runtime::plugin_resume_task` — directly, standing in for the wasm-only
    // glue: `test_support::resolve_request` IS what `Event::Completed` routes to
    // (`RequestRegistry::resolve`), and `pop_task_resume`/`plugin_resume_task` together ARE
    // `drain_task_resumes` minus the frame-routing tail (`route_app_frame`, itself untouched by
    // this packet). An honest, named gap: the wasm-only tail is not exercised natively.

    /// 🌱️ A command spawns a task; the task awaits a REAL `host::storage_read` round trip
    /// (genuinely parked on `RequestRegistry`, not a synchronously-ready future); an injected
    /// completion (`test_support::resolve_request`, standing in for `Event::Completed`)
    /// resolves it; the follow-up `TaskResolution::Command` mutates the store, stamped with
    /// the task's CLONED originating `ActionMeta`.
    #[semio_framework_async_macros::async_test]
    async fn a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta() {
        let instance = 501;
        let spawn_meta = ActionMeta { actor: "alice".into(), instance_id: instance, view_state: None };
        let mut app = VcsArtifactApp::<TestApp>::new(TestApp::<false>::default()).await;

        let result = app.dispatch_typed(TestCommand::SpawnCountTask, &spawn_meta).await.expect("dispatching SpawnCountTask must succeed");
        assert!(result.mutations.is_empty(), "SpawnCountTask itself must emit no document mutation — only the LATER resume does");
        // 🪪️ `spawn_task` is keyed off `meta.instance_id` (`dispatch_emit`'s own `meta`, i.e.
        // `spawn_meta` above, which shares `instance`'s value by construction).
        assert_eq!(crate::reactor::test_support::task_count_for_instance(instance).await, 1, "dispatch_typed(SpawnCountTask) must have spawned exactly one task");

        // ▶️ First poll: the task runs up to its `.await` on `host.storage_read(..)` and parks —
        // genuinely pending, not synchronously resolved.
        let pending = crate::reactor::test_support::run_until_idle(8).await;
        assert!(pending, "the task must be parked on a real RequestRegistry await, not finished synchronously");
        assert_eq!(crate::reactor::test_support::pending_request_count().await, 1, "storage_read must have allocated exactly one RequestRegistry slot");
        assert!(crate::reactor::test_support::pop_task_resume().await.is_none(), "nothing can have resolved yet — the task is still parked");

        // ✅️ Inject the "completion" (`⚛️reactor::poll`'s `Event::Completed` arm calls the exact
        // same `RequestRegistry::resolve`) — request id 1 is `storage_read`'s, the first (and
        // only) request this test has allocated.
        crate::reactor::test_support::resolve_request(1, Ok(42i32.to_le_bytes().to_vec())).await;
        let pending = crate::reactor::test_support::run_until_idle(8).await;
        assert!(!pending, "the task must run to completion once its await resolves");
        assert_eq!(crate::reactor::test_support::task_count_for_instance(spawn_meta.instance_id).await, 0, "TASK_RECORDS must be cleaned up the moment the task's future completes");

        let (resumed_instance, resumed_meta, resumed_input) = crate::reactor::test_support::pop_task_resume().await.expect("the completed task must have queued exactly one resume");
        assert_eq!(resumed_instance, spawn_meta.instance_id);
        assert_eq!(resumed_meta.actor, "alice", "the follow-up must carry the task's ORIGINATING actor, not whatever is 'current' at resume time");
        assert_eq!(resumed_meta.instance_id, spawn_meta.instance_id);
        let crate::plugin_runtime::TaskResumeInput::Command(command_bytes) = resumed_input.expect("the task resolved Ok, not with a Fault") else {
            panic!("SpawnCountTask's task resolves TaskResolution::Command, not ::Emit");
        };
        let decoded_command = <TestCommand as ::protocol::OpBinary>::decode_op(&command_bytes).expect("must decode back to a TestCommand");
        assert_eq!(decoded_command, TestCommand::ApplyCountFromTask { value: 42 }, "the resolved command must carry the value the injected completion delivered");

        // 🔀️ Re-enter through the SAME typed-command path a live command would — this IS
        // `plugin_runtime::plugin_resume_task`'s `TaskResumeInput::Command` arm, exercised
        // directly rather than through `crate::reactor::drain_task_resumes` (wasm-only).
        let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
        crate::plugin_runtime::test_push_instance(&runtime, AppInstance { id: resumed_instance, app: TestRuntimeApps::from(app), surface_contexts: Default::default() }).await;
        let output = crate::plugin_runtime::plugin_resume_task(&runtime, resumed_instance, &resumed_meta, crate::plugin_runtime::TaskResumeInput::Command(command_bytes)).await;
        assert_eq!(output.frames.len(), 1, "a successful resume must frame exactly one AppFrame::Emit");
        let frame = protocol::decode_app_frame(&output.frames[0]).await.expect("must decode back to an AppFrame");
        let protocol::AppFrame::Emit { document_ops, .. } = frame else { panic!("a resumed Command follow-up must frame as AppFrame::Emit, matching dispatch_emit's own last_emit_wire idiom — got {frame:?}") };
        let ops = protocol::decode_ops_vec(&document_ops).expect("document_ops must decode as an ops-vec");
        assert_eq!(ops.len(), 1, "ApplyCountFromTask emits exactly one document mutation");
        let applied_mutation = <TestMutation as ::protocol::OpBinary>::decode_op(&ops[0]).expect("must decode back to a TestMutation");
        assert_eq!(applied_mutation, TestMutation::SetCount(SetCount { value: 42 }), "the follow-up dispatch must have applied the SAME mutation ApplyCountFromTask{{value:42}} produces directly");
    }

    /// 🚫️ The (quota+1)th task on one instance is refused with a typed `Fault` — never a
    /// silent drop — while earlier tasks and OTHER instances are unaffected.
    #[semio_framework_async_macros::async_test]
    async fn spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it() {
        let instance = 502;
        let meta = ActionMeta { actor: "local".into(), instance_id: instance, view_state: None };
        crate::reactor::test_support::set_instance_quota(instance, 2).await;

        for label in ["first", "second"] {
            let task = AsyncTask::<TestMutation, TestConfigMutation, NoDraftMutation>::new(label, |_ctx| async move { Ok(TaskResolution::Done) });
            crate::reactor::spawn_task(instance, &meta, task.await).await.unwrap_or_else(|error| panic!("task '{label}' must be admitted under quota 2: {error:?}"));
        }
        assert_eq!(crate::reactor::test_support::task_count_for_instance(instance).await, 2);

        let third = AsyncTask::<TestMutation, TestConfigMutation, NoDraftMutation>::new("third", |_ctx| async move { Ok(TaskResolution::Done) });
        let error = crate::reactor::spawn_task(instance, &meta, third.await).await.expect_err("the 3rd task must be refused — quota is 2, not a silent drop");
        assert_eq!(error.code.0, "plugin.task.quota-exceeded");
        assert_eq!(crate::reactor::test_support::task_count_for_instance(instance).await, 2, "a refused spawn must not have added a 3rd record");

        // 🔓️ A different instance has its OWN quota accounting, unaffected by 502's exhaustion.
        let other_instance = 503;
        let other_meta = ActionMeta { actor: "local".into(), instance_id: other_instance, view_state: None };
        let task = AsyncTask::<TestMutation, TestConfigMutation, NoDraftMutation>::new("elsewhere", |_ctx| async move { Ok(TaskResolution::Done) });
        crate::reactor::spawn_task(other_instance, &other_meta, task.await).await.expect("a different instance must not be affected by 502's quota exhaustion");
    }

    /// 🔑️ Spawning a second task under the SAME `(instance, key)` cancels the first — its
    /// future is dropped (never runs to completion, never queues a resume) — and the dedupe
    /// index tracks only the NEW task afterward.
    #[semio_framework_async_macros::async_test]
    async fn key_dedupe_cancels_the_previously_live_task_under_the_same_key() {
        let instance = 504;
        let meta = ActionMeta { actor: "local".into(), instance_id: instance, view_state: None };
        let first_ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let first_ran_inner = first_ran.clone();
        let first = AsyncTask::<TestMutation, TestConfigMutation, NoDraftMutation>::new("first", move |_ctx| async move {
            first_ran_inner.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(TaskResolution::Done)
        })
        .await
        .keyed("search");
        crate::reactor::spawn_task(instance, &meta, first.await).await.expect("first must be admitted");
        assert!(crate::reactor::test_support::task_key_is_live(instance, "search").await);
        assert_eq!(crate::reactor::test_support::task_count_for_instance(instance).await, 1);

        let second_ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let second_ran_inner = second_ran.clone();
        let second = AsyncTask::<TestMutation, TestConfigMutation, NoDraftMutation>::new("second", move |_ctx| async move {
            second_ran_inner.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(TaskResolution::Done)
        })
        .await
        .keyed("search");
        crate::reactor::spawn_task(instance, &meta, second.await).await.expect("second must be admitted, cancelling the first");
        assert_eq!(crate::reactor::test_support::task_count_for_instance(instance).await, 1, "the SAME key must never have two live tasks at once");

        crate::reactor::test_support::run_until_idle(8).await;
        assert!(!first_ran.load(std::sync::atomic::Ordering::SeqCst), "the cancelled first task must never have run its body");
        assert!(second_ran.load(std::sync::atomic::Ordering::SeqCst), "the surviving second task must have run");
        assert_eq!(crate::reactor::test_support::task_count_for_instance(instance).await, 0, "the second task completed with TaskResolution::Done — no follow-up, cleaned up");
    }

    /// 🚫️ `Event::InstanceClose` cancellation: every task an instance owns is dropped from the
    /// executor (never runs to completion) and its pending `RequestRegistry` slot is gone too
    /// — no leaked slot — while a DIFFERENT instance's live task/request is untouched.
    #[semio_framework_async_macros::async_test]
    async fn instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot() {
        let dying = 505;
        let survivor = 506;
        let dying_meta = ActionMeta { actor: "local".into(), instance_id: dying, view_state: None };
        let survivor_meta = ActionMeta { actor: "local".into(), instance_id: survivor, view_state: None };

        let dying_ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let dying_ran_inner = dying_ran.clone();
        let dying_task = AsyncTask::<TestMutation, TestConfigMutation, NoDraftMutation>::new("dying", move |ctx: TaskCtx| async move {
            let _ = ctx.host.storage_read("never-resolved").await; // parks forever in this test
            dying_ran_inner.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(TaskResolution::Done)
        });
        crate::reactor::spawn_task(dying, &dying_meta, dying_task.await).await.expect("dying instance's task must spawn");

        let survivor_task = AsyncTask::<TestMutation, TestConfigMutation, NoDraftMutation>::new("survivor", move |ctx: TaskCtx| async move {
            let _ = ctx.host.storage_read("also-never-resolved").await;
            Ok(TaskResolution::Done)
        });
        crate::reactor::spawn_task(survivor, &survivor_meta, survivor_task.await).await.expect("survivor instance's task must spawn");

        let pending = crate::reactor::test_support::run_until_idle(8).await;
        assert!(pending, "both tasks must have parked on their own storage_read");
        assert_eq!(crate::reactor::test_support::pending_request_count().await, 2, "one RequestRegistry slot per parked task");

        crate::reactor::cancel_instance_tasks(dying);
        // 🚫️ `cancel_instance_tasks` alone drops the future (and its parked `RequestFuture`
        // with it); `RequestRegistry::cancel_instance` is the defense-in-depth sweep `poll`'s
        // `Event::InstanceClose` arm runs right after — exercised here to prove BOTH steps
        // together leave no slot behind, matching that call site exactly.
        let removed = crate::reactor::test_support::cancel_instance_registry_requests(dying);
        assert_eq!(removed.await, 0, "cancel_instance_tasks already dropped the task's own RequestFuture — nothing left for the registry sweep to remove");

        assert_eq!(crate::reactor::test_support::task_count_for_instance(dying).await, 0, "the dying instance's task record must be gone");
        assert_eq!(crate::reactor::test_support::pending_request_count().await, 1, "only the SURVIVOR's request may remain pending");
        assert_eq!(crate::reactor::test_support::task_count_for_instance(survivor).await, 1, "the survivor's task must be untouched");

        let still_pending = crate::reactor::test_support::run_until_idle(8).await;
        assert!(still_pending, "the survivor's task is still legitimately parked");
        assert!(!dying_ran.load(std::sync::atomic::Ordering::SeqCst), "the cancelled task must never observe its await resolving, because it never runs again");
    }

    /// 📸️ A task's `restart` command survives into the checkpoint pack (not the task itself —
    /// design-abi.md §4) and `restore_now` queues it as an ordinary `Command` resume, ready for
    /// `plugin_resume_task` on the very next turn — exercised end to end against a fresh
    /// `TestApp` instance, exactly like a live task's own resume.
    #[semio_framework_async_macros::async_test]
    async fn checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume() {
        let completion: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧫️fixtures/⏳️completion/🔣️.json"))).unwrap();
        let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
        let instance = 507;
        let meta = ActionMeta { actor: "local".into(), instance_id: instance, view_state: None };
        let restart_command = <TestCommand as ::protocol::OpBinary>::encode_op(&TestCommand::ApplyCountFromTask { value: 7 }).expect("must encode");
        let observed_completion = std::sync::Arc::new(std::sync::Mutex::new(None));

        let task = AsyncTask::<TestMutation, TestConfigMutation, NoDraftMutation>::new("checkpointed", {
            let observed_completion = observed_completion.clone();
            move |ctx: TaskCtx| async move {
                let result = ctx.host.storage_read("never-resolved-either").await;
                *observed_completion.lock().unwrap() = Some(result);
                Ok(TaskResolution::Done)
            }
        })
        .await
        .restartable(restart_command.clone());
        crate::reactor::spawn_task(instance, &meta, task.await).await.expect("must spawn");
        assert!(crate::reactor::test_support::run_until_idle(8).await);
        let observed_parked_requests = crate::reactor::test_support::pending_request_count().await;

        let packed = crate::reactor::checkpoint_now(&runtime).await.expect("checkpoint must succeed while the task is in flight");

        // 🚫️ The in-flight task (and its parked request) belong to the OLD actor incarnation —
        // never resumed as though the host round-trip were still live (design-abi.md §4).
        // `restore_now` below is what re-arms it, as a fresh Command resume, not a revival.
        crate::reactor::cancel_instance_tasks(instance);
        crate::reactor::test_support::cancel_instance_registry_requests(instance).await;
        crate::reactor::cancel_instance_tasks(instance);
        assert_eq!(crate::reactor::test_support::task_count_for_instance(instance).await, 0);
        assert_eq!(observed_parked_requests as u64, completion["checkpoint"]["parkedRequests"].as_u64().unwrap(), "checkpoint must observe an actually polled and parked task");
        assert_eq!(crate::reactor::test_support::pending_request_count().await as u64, completion["checkpoint"]["requestsAfterCancel"].as_u64().unwrap());
        let fault = observed_completion.lock().unwrap().take().expect("the original retained task completed").expect_err("retired request must not become a successful host response");
        assert_eq!(fault.code.0, completion["checkpoint"]["completionFaultCode"].as_str().unwrap());
        assert_eq!(fault.message, completion["checkpoint"]["completionFaultMessage"].as_str().unwrap());

        crate::reactor::restore_now(&runtime, &packed).await.expect("restore must succeed");
        let (resumed_instance, resumed_meta, resumed_input) = crate::reactor::test_support::pop_task_resume().await.expect("restore must have queued exactly one resume for the restartable task");
        assert_eq!(resumed_instance, instance);
        assert_eq!(resumed_meta.instance_id, instance);
        let crate::plugin_runtime::TaskResumeInput::Command(bytes) = resumed_input.expect("a restart resume is always Command, never Fault") else { panic!("expected Command") };
        assert_eq!(bytes, restart_command, "restore_now must requeue the EXACT restart bytes the task declared via .restartable(..)");

        let command = <TestCommand as ::protocol::OpBinary>::decode_op(&bytes).expect("the actual restored command must decode");
        assert_eq!(command, TestCommand::ApplyCountFromTask { value: completion["checkpoint"]["restartValue"].as_i64().unwrap() as i32 });
        test_restart_publish_and_close(command, &resumed_meta, &completion).await;
    }

    #[test]
    fn checkpoint_restart_mode_requires_its_exact_concrete_factory_owner() {
        let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧫️fixtures/⏳️completion/🔣️.json"))).unwrap();
        assert_eq!(<TestCommand as ::protocol::OpBinary>::TOOL_JOB_IDS, fixture["restartAuthority"]["generatedToolIds"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>());
        assert_eq!(TestApp::<false>::bounded_first_step_tool_proofs().len() as u64, fixture["restartAuthority"]["defaultProofs"].as_u64().unwrap());
        assert_eq!(TestApp::<true>::bounded_first_step_tool_proofs().len() as u64, fixture["restartAuthority"]["retainedProofs"].as_u64().unwrap());
        assert_ne!(ToolOwnerWitness::of::<TestApp<false>>(), ToolOwnerWitness::of::<TestApp<true>>());
        assert_eq!(std::any::TypeId::of::<<TestRestartFactory<true> as ArtifactOwnedToolJobFactory>::Owner>(), std::any::TypeId::of::<TestApp<true>>());
        assert_eq!(<TestRestartFactory<true> as ArtifactOwnedToolJobFactory>::TOOL_IDS, &[fixture["restartAuthority"]["tool"].as_str().unwrap()]);
    }
    //#endregion 🔖️AsyncTaskTests
}
