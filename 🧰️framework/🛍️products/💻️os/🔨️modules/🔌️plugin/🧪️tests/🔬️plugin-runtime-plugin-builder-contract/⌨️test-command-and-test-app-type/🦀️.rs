/// ⌨️ Test command and test app type: the typed `TestCommand` with its text and binary operation codecs, the spawn count task and the `TestApp` struct with its retained and tool parameters.

    /// 🧪️ B1: `TestApp`'s typed command enum — the sole dispatch surface for its own behavior.
    #[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_dsl_record_derive::DslEnum)]
    #[derive(semio_framework_value::RetireOwned)]
    pub enum TestCommand {
        #[dsl(key = "increment")]
        Increment,
        #[dsl(key = "set-label")]
        SetLabel { value: String },
        #[dsl(key = "stream-label")]
        StreamLabel { value: String, commit: bool },
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
        /// 🪟️ The addressed-window action's own verb. Every dispatch admits under the action id it was
        /// invoked with and `dispatch_typed_command_inner` then requires `command_id` to decode back to
        /// that same id, so a window-addressed action needs its own command, not an alias of another.
        #[dsl(key = "target-window")]
        TargetWindow { window_id: String },
        /// 🎚️ The mode-owned command's own verb, for the same identity reason as `TargetWindow`.
        #[dsl(key = "mode-increment")]
        ModeIncrement,
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
        /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the exact emit shape of fem2d's
        /// `canvasPointerUp` on the typed-operation ladder: NO durable store lane, one guest-emitted
        /// `interactionSelect` (`Effect::ReplayShellCommand`) and a partial window refresh.
        #[dsl(key = "pick-item")]
        PickItem { id: String },
        /// 🧮️ ONE edit whose publication folds `rows` bounded mutations — the shape a real example
        /// switch has (puzzle5d's `capsule-dream` is ≈ 5 745 mutations in one undoable edit). It
        /// exists so a law can drive a healthy publication ladder past
        /// `TYPED_OPERATION_STALL_FAULT_CEILING` units.
        #[dsl(key = "bulk-edit")]
        BulkEdit { rows: i32 },
    }

    impl ::protocol::OpText for TestCommand {
        fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{keyword} ");
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                    let record = semio_framework_dsl_record::parse(body, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                    return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
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

    thread_local! {
        /// 📨️ Every host event `TestApp::host_event` received on this test thread, in order.
        pub static TEST_HOST_EVENTS: std::cell::RefCell<Vec<HostEvent>> = const { std::cell::RefCell::new(Vec::new()) };
    }

    #[doc(hidden)]
#[derive(Debug,semio_framework_value::RetireOwned)]
    pub struct SpawnCountTask;
    impl semio_framework_plugin_decl_emission::AsyncTaskSource<TestMutation,TestConfigMutation,NoDraftMutation> for SpawnCountTask{
        type Future=std::pin::Pin<Box<dyn std::future::Future<Output=Result<TaskResolution<TestMutation,TestConfigMutation,NoDraftMutation>,Fault>>>>;
        fn run(self,ctx:TaskCtx)->Self::Future{Box::pin(async move{
            let bytes=ctx.host.storage_read("counter").await?;
            let value=i32::from_le_bytes(bytes.try_into().map_err(|_|Fault::from("original counter task requires exactly four bytes"))?);
            let command=TestCommand::ApplyCountFromTask{value};
            let encoded=<TestCommand as ::protocol::OpBinary>::encode_op(&command).map_err(|error|error.into_fault())?;
            Ok(TaskResolution::Command(encoded))
        })}
    }
    /// 🧪️ Records the actual typed commands received by the app under test.
    #[doc(hidden)]
#[derive(Default)]
    pub struct TestApp<const RETAINED: bool = false, const TOOLS: u8 = TEST_APP_TOOLS_FULL> {
        pub received_actions: std::cell::RefCell<Vec<String>>,
    }
