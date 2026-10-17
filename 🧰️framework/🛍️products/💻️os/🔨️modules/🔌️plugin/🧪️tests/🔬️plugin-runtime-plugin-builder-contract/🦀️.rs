// 🔬️ Support module 1 of the split test module `🔬️plugin-runtime-plugin-builder-contract`: the fixtures that more than one piece uses, as real definitions the pieces import.
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_decl_emission::{ActionMeta, AsyncTask, ChildEmit, Emit, TaskCtx, TaskResolution};
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks;
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, DraftView, InteractionHoverState, InteractionView, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation};
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework_plugin_window_kits::Menu;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_abi::{ArtifactDeserializer, ArtifactSerializer};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::change_publication_presence::ChangePublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::mutations::change_publication_transient::ChangePublicationTransient;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::PublicationTransient;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::mutations::PublicationTransientMutation;
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework::{MediaClass, MediaType};
use semio_framework_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework::kernel::{AppEvent, ClipboardError, ClipboardFragment, Effect, PasteAnchor, PastePlacement, UiDirtyScope};
use semio_framework::{ActionArgDef, ActionKind, CommandDefinition, InteractionVerb, MediaForm};
use semio_framework_job::InteractiveJob as _;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_2d::compute::EngineHandles;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_os_kernel::{MemberFactory, SpaceMember};
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework_ui_layout::layout::{ContextMenuItemSpec, ContextMenuRequest};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_slot_children::SetSlotChildren;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::change_test_config_selection::ChangeTestConfigSelection;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::TestConfig;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_app_builder::AppBuilder;
use semio_framework::{AppDefinition, *};
use semio_framework_plugin_decl_views::{AppOperationContext, NoTransient, NoTransientMutation, PresenceView};
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, DerivedArtifactSpec};
use semio_framework_plugin_decl_app::tool_contract::{ArtifactBoundedFirstStepProof, ArtifactOwnedToolJobContext, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactReservedJob, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion, ArtifactToolCompletionRejection, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactToolRawInput, ReservedJobOutcomes};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer, Editor};
use semio_framework_plugin_decl_views::tool_contract::{ArtifactInstanceOperationOwner, ArtifactInstanceOperationOwnerHandle};
use semio_framework_plugin_decl_app::{ArtifactOwnedContextHandle, NaturalFileDecodeCursor, app_emission, media_type, tool_contract};
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, HostEvent, ToolCommandCatalog, child_emit_preparation};
use semio_framework_plugin_decl_views::child_view::{ChildContentRetainers, ChildContentView};
use semio_framework_plugin_decl_emission::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep};
use semio_framework_ui_runtime::{ComponentTree, TreeNode};
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
use std::collections::HashMap;
use protocol::LOCAL_ACTOR_ID;
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit, context_menu};
use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
use semio_framework_plugin_plugin_program::{Plugin, plugin_runtime};
use semio_framework_plugin_builder::{PluginBuilder, Ready};
use semio_framework_plugin_peer::PresenceRosterOutcome;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_os_kernel::io::{SubsetValidator, resolve};
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_transient::window_transient::{TransientView, WindowTransientOwner};
use semio_framework_plugin_interaction::UiAssemblyResult;
use serde_json::Value;
use semio_framework_plugin_window_config::{WindowConfigOwner, WindowConfigReplaceMutation, WindowConfigSnapshot};
use semio_framework_ui_layout::layout::WindowMeasure;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_ownership::transient_publication::{bounded_transient_preparation_factory, bounded_transient_root_retirement_factory, transient_store_disposer};
use semio_framework_plugin_window_transient::{built_text_to_component_tree, window_transient};
use semio_framework_plugin_instance_docs::docs_maintenance_ladder::item;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;
use semio_framework_plugin_tool_run as tool_run;
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_ui_contract::*;
use semio_framework::kernel::*;


    /// 🌐️ One locale cell of a history row's label carrier. Every assertion below names the locale
    /// it reads, so a row that carries only English cannot pass by accident; the carrier's own
    /// locale axis is asserted by `a_shell_noted_row_carries_the_same_text_in_every_locale`.
    #[doc(hidden)]
pub fn test_child_emit(slot:impl Into<String>,child_id:impl Into<String>,operations:&[TestMutation])->ChildEmit{
        let mut emit=ChildEmit::open(slot,child_id,operations.len());
        for operation in operations{emit.push::<TestSnapshot,_>(operation).expect("actual fixture mutation must encode completely");}
        emit
    }

    /// 📨️ Funds the actual original child preparation quote within the fixture's declared per-axis ceiling.
    #[doc(hidden)]
pub fn prepare_original_fixture_child(emit:&mut Emit<TestMutation,TestConfigMutation>,maximum_items:usize,maximum_bytes:usize)->Result<ChildEmitPreparationStep,Fault>{
        let demand=emit.child_preparation_demands(maximum_bytes).expect("original child preparation quote");
        assert!(demand.copy_bytes<=maximum_bytes&&demand.capacity_bytes<=maximum_bytes&&demand.release_bytes<=maximum_bytes&&demand.depth<=128);
        let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items,maximum_copy_bytes:maximum_bytes,maximum_capacity_bytes:maximum_bytes,maximum_release_bytes:maximum_bytes,maximum_depth:128};
        emit.prepare_child_one(grant)
    }
    #[doc(hidden)]
pub fn label_in(label: &LocalizedLabel, locale: Locale) -> &str {
        label.resolve(Terminology::Native, locale)
    }

    /// 🌉️ Test-only convenience: builds a `serde_json::json!` literal, then bridges it to the
    /// `DslValue` `handle_action`/`dispatch_action`/`command_from_action` speak at the trait
    /// boundary — the fixtures below stay readable as JSON literals without keeping a
    /// `serde_json::Value` alive past this one conversion.
    #[doc(hidden)]
pub fn dv(value: Value) -> DslValue {
        semio_framework_value::DslValue::from(&value)
    }

    /// 🎟️ Validates a physical demand within the caller-authored fixture close ceiling.
    #[doc(hidden)]
pub fn fixture_close_grant(demand:semio_framework_value::RetirementDemand)->semio_framework_value::retained_clone::RetainedCloneGrant{
        let grant=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close;
        assert!(grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"demand exceeds the original fixture close policy");
        grant
    }

    /// 🎟️ Checks the next physical demand against the fixture's original close policy.
    #[doc(hidden)]
pub fn job_close_grant<J: semio_framework_job::InteractiveJob + ?Sized>(job: &J) -> semio_framework_value::retained_clone::RetainedCloneGrant {
        let grant=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close;
        let demand=semio_framework_value::RetirementDemand{copy_bytes:job.next_close_copy_byte_demand().expect("quoted copy demand"),capacity_bytes:job.next_close_capacity_byte_demand(grant.maximum_copy_bytes).expect("quoted capacity demand"),release_bytes:job.next_close_release_byte_demand().expect("quoted release demand"),depth:job.next_close_depth_demand().expect("quoted depth demand")};
        assert!(grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"job demand exceeds its original fixture close policy");
        grant
    }

    /// 🧹️ Drives one interactive job to its terminal-empty witness, granting exactly each quoted turn and checking every receipt fits.
    #[doc(hidden)]
pub fn drive_job_close<J:semio_framework_job::InteractiveJob+?Sized>(job:&mut J,caller:&mut semio_framework_job::OriginalJobFixtureCaller){
        for _ in 0..100_000{
            if job.terminal_is_empty(){return}
            let grant=job_close_grant(job);let step=caller.turn(grant,|cx|job.close_step(cx)).expect("original fixture close preserves its full failure");
            assert_eq!(step.progress(),caller.receipt());assert!(step.progress().fits(grant),"job close receipt {step:?} exceeds its original fixture grant {grant:?}");
        }
        panic!("job close did not reach its terminal-empty witness");
    }

    /// 🎟️ Spends exactly the quoted maintenance turn of a hosted app.
    #[doc(hidden)]
pub fn app_maintenance_turn<P: PluginApp>(app: &mut P,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let demand = app.maintenance_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)?;
        let grant = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance;
        assert!(grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"app demand exceeds its original fixture maintenance policy");
        let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant|Ok((app.maintenance_step(same,grant)?,app.maintenance_terminal_is_empty())))?;
        assert!(step.progress().is_none_or(|progress| progress.fits(grant)), "maintenance receipt {step:?} exceeds its exact grant {grant:?}");
        Ok(step)
    }

    /// ✍️ Authors one `Apply` of document mutations under the plugin's caller-owned identity authority.
    #[doc(hidden)]
pub async fn apply_test_mutations(store: &mut store::ArtifactStore<semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot, semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation>, mutations: Vec<semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation>) -> Result<store::CommandReceipt, store::VcsError> {
        semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations, transaction: None }, &mut identity)).await)
    }

    /// 🪪️ `TestApp`'s one dialect coordinate (contract §1) — every canonical id this module needs
    /// (`TestApp::APP_ID`, the `App::builder` ids below, every `CommandAddress`/`ActionAddress`/
    /// `ClipboardFragment.source_app` literal) derives from this ONE fixture instead of the
    /// pre-migration `"synthetic-play"` string, so `test_app_surface_id` and `TestApp::APP_ID` can
    /// never independently drift (guarded by `test_app_id_matches_its_own_dialect` below).
    #[doc(hidden)]
pub const TEST_APP_DIALECT: Dialect = Dialect { artifact_kind: "s.test.synthetic", standard: StandardId("1"), subset: SubsetId::ANY };

    /// 🪟️ The one window body key `interaction_registry`'s `window_kind("main", …)` declares — the lane
    /// `TestApp::interaction_scope` names, so the fixture and the law can never disagree about it.
    #[doc(hidden)]
pub const TEST_APP_WINDOW_BODY_KEY: &str = "synthetic.main";

    std::thread_local! {
        pub static RENDER_CONTEXT_PROBE: std::cell::RefCell<Option<(String, ViewModel)>> = const { std::cell::RefCell::new(None) };
    }

    /// 🪪️ Live `surface_app_id` call over `TEST_APP_DIALECT` — the id every `App::builder(...)` call
    /// in this module passes, per contract §1 (`AppBuilder::build_definition` rejects a hand-written id).
    #[doc(hidden)]
pub async fn test_app_surface_id() -> String {
        surface_app_id(&TEST_APP_DIALECT.into(), AppRole::Editor)
    }

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

    //#region 📋️ContractClipboardFixture
    /// 📋️ The exact reserved clipboard producer of the contract app: `copy` and `cut` author the clipboard write, `paste` the placed label, each through its one completion cell, and every original request owner retires under paid turns.
    #[doc(hidden)]
pub struct TestClipboardJob<const RETAINED: bool, const TOOLS: u8> {
        pub tool: String,
        pub label: String,
        pub args: Option<DslValue>,
        pub read: Option<store::os_store::snapshot_read::SnapshotRead<TestSnapshot>>,
        pub completion: Option<ArtifactToolCompletion<TestApp<RETAINED, TOOLS>>>,
        pub rejected: Option<semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionRejection<TestApp<RETAINED, TOOLS>>>,
        pub retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        pub outcomes: semio_framework_plugin_decl_app::tool_contract::ReservedJobOutcomes,
        pub completed: bool,
        pub closing: bool,
    }

    impl<const RETAINED: bool, const TOOLS: u8> TestClipboardJob<RETAINED, TOOLS> {
        pub fn new(request: semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJobRequest<TestApp<RETAINED, TOOLS>>) -> Result<Self, Fault> {
            let semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolInput::Action { args, .. } = request.input else { return Err(Fault::from("the contract clipboard requires an action input")) };
            Ok(Self { tool: request.tool_id, label: request.snapshot.label.clone(), args, read: Some(request.snapshot_read), completion: Some(request.completion), rejected: None, retirement: None, outcomes: semio_framework_plugin_decl_app::tool_contract::ReservedJobOutcomes::new(), completed: false, closing: false })
        }

        pub fn emit(&self) -> Result<Emit<TestMutation, TestConfigMutation, NoDraftMutation>, Fault> {
            let media_type = MediaType { class: MediaClass::Data, form: MediaForm::Value };
            if self.tool != "paste" {
                if self.label.is_empty() {
                    return Ok(Emit::default());
                }
                let fragment = ClipboardFragment { schema: TestApp::<false>::DOCUMENT_SCHEMA.to_string(), media_type, dsl_text: self.label.clone(), pack_bytes: None, source_app: TestApp::<false>::APP_ID.to_string(), label: self.label.clone() };
                let mut emit = Emit::effect(Effect::ClipboardWrite { fragment });
                if self.tool == "cut" {
                    emit.artifact_mutations = vec![TestMutation::SetLabel(SetLabel { value: String::new() })];
                }
                return Ok(emit);
            }
            let Some(fragment) = self.args.as_ref().and_then(|args| args.get("fragment")) else { return Ok(Emit::default()) };
            let text = fragment.get("dslText").and_then(DslValue::as_str).ok_or_else(|| Fault::from("the contract paste fragment carries no text"))?;
            let anchor = match self.args.as_ref().and_then(|args| args.get("anchor")).and_then(DslValue::as_str) {
                None | Some("original") => PasteAnchor::Original,
                Some("middle") => PasteAnchor::Middle,
                Some("centroid") => PasteAnchor::Centroid,
                Some("bottomLeft") => PasteAnchor::BottomLeft,
                Some("bottomRight") => PasteAnchor::BottomRight,
                Some("topLeft") => PasteAnchor::TopLeft,
                Some("topRight") => PasteAnchor::TopRight,
                Some(other) => return Err(Fault::from(format!("the contract paste anchor '{other}' is undeclared"))),
            };
            let value = if anchor == PasteAnchor::Original { text.to_string() } else { format!("{text}-{anchor:?}") };
            Ok(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value })]))
        }

        pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            if !self.outcomes.terminal_is_empty() { return self.outcomes.close_demands(); }
            if let Some(active) = self.retirement.as_ref() { return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body); }
            if self.rejected.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.rejected); }
            if self.read.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.read); }
            if self.completion.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion); }
            Ok(Default::default())
        }
    }

    impl<const RETAINED: bool, const TOOLS: u8> semio_framework_job::InteractiveJob for TestClipboardJob<RETAINED, TOOLS> {
        fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
            if cx.is_cancelled() { return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from); }
            if self.outcomes.is_delivered() {
                self.outcomes.retire(cx)?;
                return Ok(None);
            }
            if self.rejected.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "the contract clipboard completion remains rejected").into()); }
            if cx.should_yield() { return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from); }
            if !self.completed {
                let emit = self.emit().map_err(|fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, fault.message))?;
                let completion = self.completion.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "the contract clipboard lost its completion authority"))?;
                if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                    self.rejected = Some(rejected);
                    return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "the contract clipboard completion was rejected").into());
                }
                self.completed = true;
            }
            semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from)
        }

        fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
            self.outcomes.borrow_outcome(descriptor)
        }

        fn begin_close(&mut self) { self.closing = true; }

        fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
            if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
            let original=cx.retained_grant();let demand=self.close_demands(original.maximum_copy_bytes)?;
            let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
                if !self.outcomes.terminal_is_empty(){return self.outcomes.close_step(grant)}
                if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}
                else if self.rejected.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.rejected,&mut self.retirement,grant)}
                else if self.read.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.read,&mut self.retirement,grant)}
                else if self.completion.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.retirement,grant)}
                else{Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default()))}
            })?;
            step.admit(original,self.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
        fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
        fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
        fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.read.is_none() && self.completion.is_none() && self.rejected.is_none() && self.retirement.is_none() && self.outcomes.terminal_is_empty()
        }
    }

    impl<const RETAINED: bool, const TOOLS: u8> semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob for TestClipboardJob<RETAINED, TOOLS> {}
    //#endregion 📋️ContractClipboardFixture

    #[doc(hidden)]
pub const TEST_RETAINED_COMMAND_CONTROLLER: &str = "s.test.synthetic@1/*#editor";
    #[doc(hidden)]
pub const TEST_RETAINED_COMMAND_TOOL: &str = "setLabel";
    #[doc(hidden)]
pub const TEST_RETAINED_COMMAND_SCHEMA: &str = "semio.test.retained-command.v1";
    #[doc(hidden)]
pub const TEST_RETAINED_COMMAND_RAW_BYTES: usize = 4_096;
    #[doc(hidden)]
pub static TEST_RETAINED_COMMAND_STEP_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    #[doc(hidden)]
pub struct TestRetainedCommandWork {
        pub cursor: u8,
    }

    impl semio_framework_plugin_retained_command::ArtifactCommandWork<TestApp> for TestRetainedCommandWork {
        fn tool_id(&self) -> &'static str {
            TEST_RETAINED_COMMAND_TOOL
        }

        fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &InteractionState, _context: Option<&ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
            Some(3)
        }

        fn step(&mut self, input: &semio_framework_plugin_retained_command::ArtifactCommandInputs<'_, TestApp>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_retained_command::ArtifactCommandWorkStep<TestApp>, semio_framework_job::OriginalTurnFailure> {
            let semio_framework_plugin_retained_command::ArtifactCommandInputs { snapshot_owner: _, command: _command, snapshot: _snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
            TEST_RETAINED_COMMAND_STEP_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if self.cursor < 2 {
                self.cursor += 1;
                return Ok(semio_framework_plugin_retained_command::ArtifactCommandWorkStep::Progress { stage: "test-retained-command-work", preview: br#"{"en":"Applying","de":"Anwenden"}"# });
            }
            Ok(semio_framework_plugin_retained_command::ArtifactCommandWorkStep::Complete(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value: "resumed".into() })])))
        }

        fn checkpoint_byte(&self, index: usize) -> Option<u8> {
            (index == 0).then_some(self.cursor)
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

    #[doc(hidden)]
pub struct TestRetainedCommandFactory {
        pub keys: Vec<ToolFactoryKey>,
    }

    impl TestRetainedCommandFactory {
        pub fn new() -> Self {
            Self { keys: vec![ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL)] }
        }
    }

    impl ArtifactOwnedToolJobFactory for TestRetainedCommandFactory {
        type Owner = TestApp;
        const TOOL_IDS: &'static [&'static str] = &[TEST_RETAINED_COMMAND_TOOL];
        const DOCUMENT_SCHEMA: &'static str = TestApp::<false>::DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: TEST_RETAINED_COMMAND_TOOL, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
    }

    impl ToolJobFactory for TestRetainedCommandFactory {
        type Payload = semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload<TestApp>;
        type Job = semio_framework_plugin_retained_command::ArtifactRetainedCommandJob<TestApp>;

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
            Ok(semio_framework_plugin_retained_command::ArtifactRetainedCommandJob::new(payload))
        }

        fn create_job_from_wire_pages_with_payload(
            &mut self,
            _operation: semio_framework_job::Operation,
            payload: Self::Payload,
            input: action_bus::RetainedToolWireInput,
            checkpoint: Option<action_bus::RetainedToolWireInput>,
        ) -> Result<Self::Job, (ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
            if input.declared_bytes() > TEST_RETAINED_COMMAND_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin_retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
                return Err((ToolJobFactoryError::new("test retained command extent"), input, checkpoint));
            }
            Ok(match checkpoint {
                Some(checkpoint) => semio_framework_plugin_retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
                None => semio_framework_plugin_retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
            })
        }
    }

    #[doc(hidden)]
pub fn test_retained_command_id(command: &TestCommand) -> &'static str {
        match command {
            TestCommand::SetLabel { .. } => TEST_RETAINED_COMMAND_TOOL,
            _ => "unsupported",
        }
    }

    #[doc(hidden)]
pub const TEST_COMMAND_RETAINED_GRANT: semio_framework_value::RetainedCloneGrant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };

    #[doc(hidden)]
pub async fn test_retained_command_payload(completion: ArtifactToolCompletion<TestApp>) -> semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload<TestApp> {
        semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin_retained_command::ArtifactRetainedCommandInputs {
                command: TestCommand::SetLabel { value: "wire".into() },
                snapshot: std::sync::Arc::new(TestSnapshot::default()),
                config: std::sync::Arc::new(TestConfig::default()),
                history: std::sync::Arc::new(HistoryView::empty()),
                interaction_state: std::sync::Arc::new(InteractionState::default()),
                interaction_hover: std::sync::Arc::new(InteractionHoverState::new()),
                context: None,
                operation: AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 41, generation: 3, canonical_base_revision: [5; 32], retained: TEST_COMMAND_RETAINED_GRANT, authoring_seed: "authoring-seed-test".into() },
                completion,
            },
            test_retained_command_id,
            TEST_RETAINED_COMMAND_RAW_BYTES,
            3,
            Box::new(TestRetainedCommandWork { cursor: 0 }),
        )
    }


    /// 🌊️ The one tool transaction every `streamLabel` tick streams into and its `commit: true` tick closes (design §15).
    #[doc(hidden)]
pub const TEST_STREAM_LABEL_TRANSACTION: &str = "tx-00000000000005ab";

    /// 🪪️ `TestApp`'s verb for one command variant, as a plain `fn` so the retained
    /// command payload (`ArtifactRetainedCommandPayload::command_id`) and the async
    /// `ArtifactApp::command_id` answer the same id from ONE table.
    ///
    /// 🔀️ Rides the keyed fixture's ONE generated tool id (`TOOL_JOB_IDS` is fixture-checked):
    /// a pick and a composite edit are two commands of the same typed tool.
    #[doc(hidden)]
pub fn test_command_id(command: &TestCommand) -> &'static str {
        match command {
            TestCommand::Increment => "increment",
            TestCommand::SetLabel { .. } => "setLabel",
            TestCommand::StreamLabel { .. } => "streamLabel",
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
            TestCommand::TargetWindow { .. } => "targetWindow",
            TestCommand::ModeIncrement => "mode.increment",
            TestCommand::CompositeEdit { .. } => "compositeEdit",
            TestCommand::ProbeChild { .. } => "probeChild",
            TestCommand::SpawnCountTask => "spawnCountTask",
            TestCommand::ApplyCountFromTask { .. } => "applyCountFromTask",
            TestCommand::PickItem { .. } => "compositeEdit",
            TestCommand::BulkEdit { .. } => "compositeEdit",
        }
    }

    /// 🧮️ `TestApp`'s reducer body, shared verbatim by `ArtifactApp::handle` (host route) and
    /// `test_app_command_reduce` (the app-owned retained tool route) so a migrated dispatch
    /// and a direct one can never diverge.
    ///
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: no mutations of its own — the
    /// task's eventual `TaskResolution::Command` follow-up is what mutates the
    /// document, on a LATER dispatch (see `ApplyCountFromTask` below).
    #[doc(hidden)]
pub fn test_app_reduce(command: &TestCommand, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> Result<Emit<TestMutation, TestConfigMutation, NoDraftMutation>, Fault> {
        match command {
            TestCommand::Increment | TestCommand::IncrementViaCommand | TestCommand::ModeIncrement => Ok(Emit { artifact_mutations: vec![TestMutation::SetCount(SetCount { value: doc.snapshot.count + 1 })], ..Default::default() }),
            TestCommand::TargetWindow { window_id } => Ok(Emit { artifact_mutations: vec![TestMutation::SetLabel(SetLabel { value: window_id.clone() })], ..Default::default() }),
            TestCommand::WatchdogOverrun => {
                let started = std::time::Instant::now();
                while started.elapsed() < std::time::Duration::from_millis(10) {
                    std::hint::spin_loop();
                }
                Ok(Emit::default())
            }
            TestCommand::SetLabel { value } => Ok(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value: value.clone() })])),
            TestCommand::SetLabelViaCommand { value } => Ok(Emit::mutations(vec![TestMutation::SetLabel(SetLabel { value: value.clone() })])),
            TestCommand::StreamLabel { value, commit } => {
                let transaction = protocol::TransactionRef { id: TEST_STREAM_LABEL_TRANSACTION.into(), tool: format!("{}#streamLabel", TestApp::<false>::APP_ID) };
                let mutations = vec![TestMutation::SetLabel(SetLabel { value: value.clone() })];
                Ok(if *commit { Emit::commit_transaction(transaction, mutations) } else { Emit::stream_transaction(transaction, mutations) })
            }
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
                child_preparations:std::collections::VecDeque::from([semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparation::of::<TestSnapshot,_>(slot.clone(),child_id.clone(),vec![TestMutation::SetCount(SetCount{value:*child_value})])]),
                ..Default::default()
            }),
            TestCommand::ProbeChild { slot, child_id } => {
                let _snapshot = doc.children.typed_read::<TestSnapshot>(slot, child_id)?;
                Ok(Emit::effect(Effect::DispatchAction { req: RequestId(91_001), action: "probeChildContinuation".into(), args: None, delay_ms: 0 }))
            }
            TestCommand::SpawnCountTask => Ok(Emit::task(AsyncTask::new("spawn-count-task".into(),SpawnCountTask,semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:256}).unwrap().0)),
            TestCommand::ApplyCountFromTask { value } => Ok(Emit::mutations(vec![TestMutation::SetCount(SetCount { value: *value })])),
            TestCommand::PickItem { id } => Ok(keyed_pick_emit(id)),
            TestCommand::BulkEdit { rows } => Ok(Emit { artifact_mutations: (1..=*rows).map(|row| TestMutation::SetCount(SetCount { value: row })).collect(), ..Default::default() }),
        }
    }

    #[doc(hidden)]
pub fn test_prepared_operation_wire_source(operation: &TestMutation) -> Option<store::ArtifactPreparedOperationSource<'_>> {
        let (_, ordinal, spec) = semio_framework_dsl_record::BorrowedDslVariants::projected_borrowed_variant_identity(operation);
        let tag = u64::try_from(ordinal).ok()?;
        Some(match operation {
            TestMutation::SetCount(payload) => store::ArtifactPreparedOperationSource::Pack { tag, body: payload, spec },
            TestMutation::SetLabel(payload) => store::ArtifactPreparedOperationSource::Pack { tag, body: payload, spec },
            TestMutation::SetSlotChildren(payload) => store::ArtifactPreparedOperationSource::Pack { tag, body: payload, spec },
        })
    }

    #[doc(hidden)]
pub fn test_document_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<TestSnapshot, TestMutation>> {
        store::operation_wire_preparation_factory(store::mutation_apply_preparation_factory::<TestSnapshot, TestMutation>(), test_prepared_operation_wire_source, |operation| {
            let semantics = protocol::SemanticMutation::semantics(operation);
            Some((semantics.entity, semantics.kind))
        })
    }

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
            _engines: &EngineHandles,
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

    //#region 🗝️RegisteredKeyedDispatchFixture
    #[doc(hidden)]
#[derive(Default)]
    pub struct KeyedTestApp;

    /// 🪟️ The keyed fixture's one window body — the carrier's own partial refresh scope names it.
    #[doc(hidden)]
pub const KEYED_WINDOW_BODY_KEY: &str = "synthetic.main";

    /// 🔀️ Exactly what `canvas_gesture::pointer_up` (fem2d) returns for a pick: ONE
    /// `interaction_select_effect` (`Effect::ReplayShellCommand { interactionSelect }`), a partial
    /// window refresh, and NO store lane — the emit whose verb reached the host on the install pump.
    #[doc(hidden)]
pub fn keyed_pick_emit(id: &str) -> Emit<TestMutation, TestConfigMutation> {
        Emit {
            effects: vec![inline_interaction_select(interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), id))],
            ui_scope: keyed_pick_carrier_scope(),
            ..Default::default()
        }
    }

    #[doc(hidden)]
pub fn keyed_pick_carrier_scope() -> UiDirtyScope {
        UiDirtyScope::Partial { window_bodies: vec![KEYED_WINDOW_BODY_KEY.into()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    #[doc(hidden)]
pub struct KeyedTestCommandDisposer {
        pub pending: Option<TestCommand>,
        pub active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        pub moved: bool,
    }

    impl KeyedTestCommandDisposer {
        pub fn new() -> Self { Self { pending: None, active: None, moved: false } }
    }

    impl ArtifactOwnedDisposer<TestCommand> for KeyedTestCommandDisposer {
        fn retirement_demands(&self, _: &TestCommand, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            if let Some(active) = self.active.as_ref() { return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body); }
            if self.pending.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending); }
            Ok(if self.moved { Default::default() } else { semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<TestCommand>(), depth: 1, ..Default::default() } })
        }

        fn close_step(&mut self, command: &mut TestCommand, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, Fault> {
            use semio_framework_value::retained_clone::RetainedCloneProgress;
            if grant.maximum_items == 0 { return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default())); }
            let demand = self.retirement_demands(command, grant.maximum_copy_bytes).map_err(semio_framework_plugin_ownership::bounded_ownership::plugin_retirement_fault)?;
            if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default())); }
            if !self.moved {
                self.pending = Some(std::mem::replace(command, TestCommand::NoopMutation));
                self.moved = true;
                return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<TestCommand>(), ..Default::default() }));
            }
            let step = if self.active.is_some() { store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant) } else { store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending, &mut self.active, grant) }.map_err(semio_framework_plugin_ownership::bounded_ownership::plugin_retirement_fault)?;
            Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::retained(step, self.terminal_is_empty(command)))
        }

        fn terminal_is_empty(&self, command: &TestCommand) -> bool { self.moved && self.pending.is_none() && self.active.is_none() && matches!(command, TestCommand::NoopMutation) }
        fn terminal_frame_release_bytes(&self) -> Option<usize> { (self.moved && self.pending.is_none() && self.active.is_none()).then_some(std::mem::size_of::<Self>()) }
    }

    #[doc(hidden)]
pub struct KeyedTestJobSources {
        pub raw_wire: Option<semio_framework_plugin_decl_app::tool_contract::ArtifactToolRawInput>,
        pub read: Option<store::os_store::snapshot_read::SnapshotRead<TestSnapshot>>,
        pub snapshot: Option<std::sync::Arc<TestSnapshot>>,
        pub config: Option<std::sync::Arc<TestConfig>>,
        pub history: Option<std::sync::Arc<HistoryView>>,
        pub window_config: Option<semio_framework_plugin_window_config::WindowConfigSnapshot>,
        pub interaction: Option<std::sync::Arc<InteractionState>>,
        pub hover: Option<std::sync::Arc<InteractionHoverState>>,
        pub context: Option<ArtifactOwnedContextHandle<KeyedTestApp>>,
        pub instance_owner: Option<semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle>,
        pub output: Option<ArtifactOutputChunks>,
        pub parent_document_id: Option<String>,
        pub authoring_seed: Option<String>,
    }

    impl KeyedTestJobSources {
        /// 🧹️ The next original request input to retire, quoted as the typed retirement lane that owns it: `None` once every input is physically returned.
        pub fn demands(&self) -> Result<Option<semio_framework_value::RetirementDemand>, semio_framework_value::ValueError> {
            macro_rules! owned { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.$field).map(Some); } }; }
            macro_rules! lease { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.$field).map(Some); } }; }
            if self.raw_wire.is_some() { return Ok(Some(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })); }
            if self.window_config.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "keyed fixture has no retirement lane for a window config snapshot")); }
            owned!(read); owned!(context); owned!(instance_owner); owned!(output); owned!(parent_document_id); owned!(authoring_seed);
            lease!(snapshot); lease!(config); lease!(history); lease!(interaction); lease!(hover);
            Ok(None)
        }

        pub fn step(&mut self, active: &mut Option<Box<dyn store::ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<semio_framework_value::retained_clone::RetainedCloneStep>, semio_framework_value::ValueError> {
            macro_rules! owned { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.$field, active, grant).map(Some); } }; }
            macro_rules! lease { ($field:ident) => { if self.$field.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.$field, active, grant).map(Some); } }; }
            if self.raw_wire.take().is_some() { return Ok(Some(semio_framework_value::retained_clone::RetainedCloneStep::Progress(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))); }
            owned!(read); owned!(context); owned!(instance_owner); owned!(output); owned!(parent_document_id); owned!(authoring_seed);
            lease!(snapshot); lease!(config); lease!(history); lease!(interaction); lease!(hover);
            Ok(None)
        }
    }

    #[doc(hidden)]
pub struct KeyedTestJob {
        pub sources: Option<KeyedTestJobSources>,
        pub rejected: Option<semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionRejection<KeyedTestApp>>,
        pub admitted: bool,
        pub retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        pub command: Option<Box<TestCommand>>,
        pub completion: Option<ArtifactToolCompletion<KeyedTestApp>>,
        pub raw: Option<action_bus::RetainedToolWireInput>,
        pub page: usize,
        pub base_count: i32,
        pub closing: bool,
    }

    impl KeyedTestJob {
        pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            if let Some(raw) = self.raw.as_ref() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: raw.next_close_copy_byte_demand()?, capacity_bytes: raw.next_close_capacity_byte_demand(body)?, release_bytes: raw.next_close_release_byte_demand()?, depth: raw.next_close_depth_demand()? }); }
            if let Some(active) = self.retirement.as_ref() { return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body); }
            if self.command.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.command); }
            if self.completion.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion); }
            if let Some(sources) = self.sources.as_ref() { return Ok(sources.demands()?.unwrap_or(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })); }
            if self.rejected.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.rejected); }
            Ok(Default::default())
        }
    }

    impl semio_framework_job::InteractiveJob for KeyedTestJob {
        fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
            if self.admitted { return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from); }
            if self.rejected.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "keyed fixture original completion remains rejected").into()); }
            if cx.is_cancelled() {
                return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            if cx.should_yield() {
                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            let Some(now_us) = cx.now_us() else {
                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            };
            assert!(cx.deadline_us().saturating_sub(now_us) <= 500, "registered factory deadline must preserve its exact 500us contract");
            if self.raw.as_ref().is_some_and(|raw| self.page < raw.page_count()) {
                self.page += 1;
                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            let emit = match self.command.as_deref().unwrap() {
                TestCommand::CompositeEdit { slot, child_id, child_value } => {
                    let child_emits = (!slot.is_empty()).then(|| test_child_emit(slot.clone(), child_id.clone(), &[TestMutation::SetCount(SetCount { value: *child_value })])).into_iter().collect();
                    Emit { artifact_mutations: vec![TestMutation::SetCount(SetCount { value: self.base_count + child_value })], child_emits, ..Default::default() }
                }
                TestCommand::PickItem { id } => keyed_pick_emit(id),
                TestCommand::BulkEdit { rows } => Emit { artifact_mutations: (1..=*rows).map(|row| TestMutation::SetCount(SetCount { value: self.base_count + row })).collect(), ..Default::default() },
                _ => panic!("exact keyed fixture command"),
            };
            if let Err(original) = self.completion.as_ref().unwrap().complete(Ok(emit), EphemeralEmit::default()) {
                self.rejected = Some(original);
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "keyed fixture retains original rejected completion").into());
            }
            self.admitted = true;
            semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from)
        }

        fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        semio_framework_plugin_testing_laws::artifact_app_laws::fixture_job_outcome(descriptor)
    }

    fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
            if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}
            let original=cx.retained_grant();let demand=self.close_demands(original.maximum_copy_bytes)?;
            let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
                use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep};
                if let Some(raw)=self.raw.as_mut(){
                    let step=raw.close_step(grant)?;if raw.terminal_is_empty(){self.raw.take();}
                    return Ok(RetainedCloneStep::Progress(step.progress()))
                }
                if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}
                else if self.command.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.command,&mut self.retirement,grant)}
                else if self.completion.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.retirement,grant)}
                else if let Some(sources)=self.sources.as_mut(){
                    match sources.step(&mut self.retirement,grant){
                        Ok(Some(step))=>Ok(step),
                        Ok(None)=>{self.sources=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
                        Err(error)=>Err(error),
                    }
                }
                else if self.rejected.is_some(){store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.rejected,&mut self.retirement,grant)}
                else{Ok(RetainedCloneStep::Complete(Default::default()))}
            })?;
            step.admit(original,self.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
        fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
        fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
        fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.sources.is_none() && self.rejected.is_none() && self.retirement.is_none() && self.raw.is_none() && self.command.is_none() && self.completion.is_none()
        }
    }

    #[doc(hidden)]
pub struct KeyedTestFactory {
        pub keys: Vec<ToolFactoryKey>,
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
            Some(Box::new(KeyedTestCommandDisposer::new()))
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
        semio_framework_plugin_decl_app::bounded_first_step_tool_proofs! {
            owner: KeyedTestApp, owner_file: "plugin/🦀️.rs", controller: "s.test.keyed@1/*#editor", artifact_schema: "semio.test/v1",
            factory: "KeyedTestFactory", factory_type: KeyedTestFactory,
            contract: ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
        }
        fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
            registry.register(KeyedTestFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
        }
        async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
            assert!(request.snapshot.label.is_empty());
            let base_count = request.snapshot.count;
            let sources = KeyedTestJobSources { raw_wire: Some(request.raw_wire), read: Some(request.snapshot_read), snapshot: Some(request.snapshot), config: Some(request.config), history: Some(request.history), window_config: request.window_config, interaction: Some(request.interaction_state), hover: Some(request.interaction_hover), context: Some(request.context), instance_owner: Some(request.instance_operation_owner), output: Some(request.output_chunks), parent_document_id: Some(request.parent_document_id), authoring_seed: Some(request.authoring_seed) };
            let job = KeyedTestJob { command: Some(request.command), completion: Some(request.completion), raw: None, page: 0, base_count, closing: false, sources: Some(sources), rejected: None, admitted: false, retirement: None };
            Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
        }
        fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
            Some(store::mutation_apply_preparation_factory::<TestSnapshot, TestMutation>())
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
            Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_store_disposer())
        }
        fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_framework_plugin_decl_views::transient_retirement::no_transient_store_disposer())
        }
        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_peer_retirement_factory())
        }
        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_local_root_retirement_factory())
        }
        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_framework_plugin_decl_views::transient_retirement::no_transient_local_root_retirement_factory())
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

    #[doc(hidden)]
pub async fn keyed_test_registry() -> AppActionRegistry {
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

    /// ⛽️ The per-turn ceiling a browser turn actually carries — `DEFAULT_SHARD_BUDGET` lifted through
    /// `🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`'s `poll`, which stamps `maxFrames: 8` — so a law that
    /// drives the continuation reads the exact grant production does, not a generous test one.
    #[doc(hidden)]
pub fn reactor_native_budget(cx:&semio_framework_job::StepContext<'_>) -> semio_framework::kernel::Budget {
        semio_framework::kernel::Budget { retained: cx.retained_grant(), fuel: cx.fuel_remaining(), deadline_us: cx.deadline_us(), max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 8 }
    }
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
            let mut admitted = self.0.dispatch_typed(command, meta, identity).await?;
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
            let admitted = self.0.dispatch_action(action, args, meta, identity).await?;
            let mut admitted = semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut self.0, admitted).await?;
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
        let mut app = Box::pin(VcsArtifactApp::with_registry(TestApp::<false>::default(), registry, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await;
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
        let mut app = Box::pin(VcsArtifactApp::with_registry(TestApp::<false>::default(), migrated_contract_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(meta().instance_id).await;
        app
    }

    /// 🧩️ [`contract_app_raw`] over the composed member roster.
    #[doc(hidden)]
pub async fn contract_composed_app_raw(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<TestApp, TestMembers> {
        let mut app = Box::pin(VcsArtifactApp::<TestApp, TestMembers>::with_registry(TestApp::<false>::default(), migrated_contract_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await;
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
            let mut admitted = self.0.dispatch_typed(command, meta, identity).await?;
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
            let admitted = self.0.dispatch_action(action, args, meta, identity).await?;
            let mut admitted = semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut self.0, admitted).await?;
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
        let mut app = Box::pin(VcsArtifactApp::<TestApp, TestMembers>::with_registry(TestApp::<false>::default(), migrated_contract_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(meta().instance_id).await;
        ContractComposedApp(app, mounted_policy)
    }

    /// 🧪️ A registry declaring one `HierarchyProvider::Topology` interaction domain ("items", see
    /// `TestApp::interaction_topology`) — the `🕹️InteractionDispatch` fixture. Auto-injects the six
    /// framework interaction actions via `interaction_action_definitions` (verified by
    /// `build_definition_carries_window_interactions_and_injects_framework_actions` above).
    #[doc(hidden)]
pub async fn interaction_registry() -> AppActionRegistry {
        AppActionRegistry::from_definition(&interaction_app_definition().await)
    }

    /// 🕹️ The synthetic app the interaction laws dispatch against: ONE window kind, ONE declared
    /// domain, and the `window_kind_interactions` ref that binds them — the whole input the framework's
    /// `interaction_declared_refresh_scope` derives a verb's refresh scope from.
    #[doc(hidden)]
pub async fn interaction_app_definition() -> AppDefinition {
        let builder = App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
                .await
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), TEST_APP_WINDOW_BODY_KEY, SurfaceKind::Canvas2d, IconName::AppWindow)
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
                .await;
        let app = App::from_builder(declare_test_app_verbs(builder).interactive_jobs(InteractiveJobClassification::Migrated).await).await;
        app.definition
    }

    #[doc(hidden)]
pub async fn interaction_app_under_test(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> ContractApp {
        contract_app_with(interaction_registry().await, mounted_policy, identity).await
    }

    /// 🕹️ [`interaction_app_under_test`] without the self-closing wrapper — for the local-interaction
    /// laws that move the app into a `PluginRuntime` cell.
    #[doc(hidden)]
pub async fn interaction_app_raw(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<TestApp> {
        let mut app = Box::pin(VcsArtifactApp::with_registry(TestApp::<false>::default(), interaction_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity)).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(meta().instance_id).await;
        app
    }

    /// 🧪️ Builds the JSON `args` an `interactionSelect`/`interactionHover` dispatch carries — the
    /// `targets` arg is itself a JSON-encoded string (an `ActionArgDef::text`, matching
    /// `interaction_action_definitions`'s declaration), not a nested JSON array.
    #[doc(hidden)]
pub fn interaction_target_args(extra: Value, id: &str) -> DslValue {
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "item".into(), id: id.into() }]).expect("targets serialize");
        let mut object = semio_framework_value::DslValue::from(&extra);
        if let semio_framework_value::DslValue::Object(entries) = &mut object {
            entries.retain(|(key, _)| key != "targets");
            entries.push(("targets".to_string(), semio_framework_value::DslValue::String(targets)));
        }
        object
    }
    
    /// ⏱️ Turns one command may take to reach its terminal ingress status before the law calls it stuck.
    #[doc(hidden)]
pub const INGRESS_TURNS_PER_COMMAND: usize = 512;
    
    #[doc(hidden)]
pub fn command_page_authority_budget() -> Budget {
        Budget { retained: semio_framework_plugin_testing_laws::artifact_app_laws::fixture_retained_turn(), fuel: 64, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }
    }

    #[doc(hidden)]
pub async fn __semio_plugin_bundle() -> Result<Plugin<TestRuntimeApps>, PluginAssemblyError> {
        PluginBuilder::<_, TestRuntimeApps>::new("test").label("Synthetic").version("0.0.1").package_id("semio:test").document_app::<TestApp>(synthetic_play_app().await).document_app_mutation_roster::<TestApp>().try_build()
    }
    //#endregion 🔖️EphemeralLaneTests

    //#region 🔖️AdoptPresenceTests
    #[doc(hidden)]
pub fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> PresencePeer {
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
            tool_run: None,
            principal_kind: None,
            active_tool: None,
            history_edit: None,
            typing: Vec::new(),
        }
    }

    #[doc(hidden)]
pub async fn publish_presence_roster(app: &mut VcsArtifactApp<TestApp>, seq: u64, own_color: Option<u8>, peers: &[PresencePeer], now_ms: i64,cx:&mut semio_framework_job::StepContext<'_>) -> PresenceRosterOutcome {
        let roster = peers.iter().map(|peer| ::semio_framework_async::poll::resolve_ready(encode_presence_peer(peer))).collect::<Vec<_>>();
        let admission = app.reserve_presence_ingress(seq).expect("reserve roster before decode");
        let first = roster.iter().next().map(|bytes| bytes.to_vec()).unwrap_or_default();
        let cursor = protocol::PresenceCommandCursor::admit_page(seq, own_color, roster.len() as u32, FixedCommandPage::try_copy_from(&first).expect("test peer page is fixed-authority")).map_err(|(error, _)| error).expect("admit first roster page");
        let generation = admission.generation();
        app.admit_presence_ingress(admission, cursor, now_ms);
        let mut next_page = 1usize;
        for _ in 0..512 {
            app.maintenance.maintenance_stage = 7;
            let _ = app_maintenance_turn(app,cx).expect("bounded roster step");
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
    //#endregion 🔖️InteractionViewPeersTests

    store::space_members! {
    native_producer:&::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
        history_limits:semio_framework_os_kernel::os_store::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},
    maximum_history_allocation_bytes:8388608,
    pub enum TestMembers, TestMembersOpen {
            Child("s.test.child", "native", "*", "semio.test/v1") => (TestSnapshot, TestMutation, semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer, semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer),
        }
    }

    /// 📇️ The APP's half of a composition: declares `child_id` in the parent's one owned-child slot beside every member
    /// it already declares. The runtime admits a member only under the identity its parent's snapshot declares
    /// (`declared_child_reference`), so a law that registers a live member declares it first — exactly as a product
    /// parent's own leaf does before its member is opened.
    #[doc(hidden)]
pub async fn declare_test_child(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str) {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let mut children: Vec<String> = app.stores.test_snapshot().await.slot.iter().map(|child| child.target.to_uri()).collect();
        children.push(ArtifactRef { artifact_id: child_id.into(), dialect: test_child_dialect().await }.to_uri());
        apply_test_mutations(app.stores.test_store_mut().await, vec![TestMutation::SetSlotChildren(SetSlotChildren { children })]).await.expect("the parent declares the member it is about to own");
    }

    /// 🤝️ [`declare_test_child`], then the RUNTIME's half: a fresh [`new_test_child`] registered as that declared member.
    #[doc(hidden)]
pub async fn register_test_child(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str) {
        declare_test_child(app, child_id).await;
        app.register_child("slot", child_id, test_child_dialect().await, new_test_child(child_id).await.expect("construct child"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("register the declared child");
    }

    /// 🧪️ A live child `ArtifactStore<TestSnapshot, TestMutation>`, wrapped as `TestMembers` —
    /// the shape `VcsArtifactApp::register_child`/`open_child` expect. Built directly (no
    /// runtime `ChildStoreFactory`/`MemberFactory` registration needed — `TestMembers::open`
    /// dispatches by kind at compile time) — the SAME `TestSnapshot`/`TestMutation` pair
    /// `TestApp` itself uses, so a "child" here is just a second, independently-owned instance
    /// of the identical document shape.
    #[doc(hidden)]
pub async fn new_test_child(id: &str) -> Result<TestMembers, store::VcsError> {
        let mut envelope = store::os_store::document_materialize::create_document_envelope::<TestSnapshot, TestMutation>("semio.test/v1", id, TestSnapshot::default(), None);
        envelope.dialect = Some(test_child_dialect().await);
        let mut child = store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await?;
        store::install_funded_member_store_owners(&mut child,&semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer)?;
        Ok(TestMembers::Child(Box::new(child)))
    }

    /// 🫙️ The owner-less twin of {@link new_test_child}, for the laws whose whole subject is a
    /// member that has NOT yet adopted its bounded retirement authority. Every product path mints a
    /// member through `create_member_store`/`open_member_store`, which install the catalog, so this
    /// shape exists only to drive the refusal those laws assert.
    #[doc(hidden)]
pub async fn new_bare_test_child(id: &str) -> Result<TestMembers, store::VcsError> {
        let mut envelope = store::os_store::document_materialize::create_document_envelope::<TestSnapshot, TestMutation>("semio.test/v1", id, TestSnapshot::default(), None);
        envelope.dialect = Some(test_child_dialect().await);
        Ok(TestMembers::Child(Box::new(store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await?)))
    }

    #[doc(hidden)]
pub async fn test_child_dialect() -> ArtifactDialect {
        ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
    }

    #[doc(hidden)]
pub fn close_member_admission_fixture(member: &mut TestMembers) {
        for _ in 0..65_536 {
            let grant = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close;
            let demand=member.close_owned_demands(grant.maximum_copy_bytes).expect("actual member close demand");
            assert!(grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"member demand exceeds its original fixture close policy");
            let step = member.close_owned_step(grant).expect("actual member original close");
            assert!(step.progress().fits(grant));
            if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
                assert!(member.close_owned_terminal_is_empty());
                return;
            }
        }
        panic!("member admission fixture did not close");
    }

    #[doc(hidden)]
pub fn close_member_admission_app<A: ArtifactApp>(app: &mut VcsArtifactApp<A, TestMembers>,cx:&mut semio_framework_job::StepContext<'_>) {
        artifact_app_laws::close_registered_fixture_app(app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    }

    /// 🧪️ One reactor-turn continuation unit, polled exactly the way the guest bridges it: ONCE, with
    /// a no-op waker. A unit that suspends is the `resolve_ready: future was not ready on first poll`
    /// trap that killed the gis actor on its first auto check-in.
    #[doc(hidden)]
pub fn original_publication_turn<P:PluginApp>(app:&mut P,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,caller:&mut semio_framework_job::OriginalJobFixtureCaller,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(),semio_framework_job::OriginalTurnFailure>{
        caller.turn(grant,|original|{
            let mut task=std::task::Context::from_waker(std::task::Waker::noop());
            let mut unit=std::pin::pin!(PluginApp::advance_typed_operation_publication(app,identity,original));
            match std::future::Future::poll(unit.as_mut(),&mut task){std::task::Poll::Ready(result)=>result.map_err(Into::into),std::task::Poll::Pending=>panic!("a reserved commit unit suspended inside its original turn")}
        })
    }

    /// 🧪️ The child's current `count`, read through the same `ChildContentView` seam an app uses.
    #[doc(hidden)]
pub async fn reads_child_count(app: &VcsArtifactApp<TestApp, TestMembers>) -> i32 {
        let view = ChildContentView::clone(&app.child_composition.child_content_root);
        view.typed_read::<TestSnapshot>("slot", "child-1").expect("child readable through the view").count
    }

    /// 🔀️ The exact effect every app's `interaction_select_effect` builds — a guest asking the host
    /// to replay one framework interaction verb.
    #[doc(hidden)]
pub fn inline_interaction_select(args: DslValue) -> Effect {
        Effect::ReplayShellCommand { action_id: INTERACTION_SELECT_ACTION_ID.into(), args: Some(args) }
    }

    /// 🧵️ Test-only typed task source: the closure is the whole captured state and retires as one leaf.
    #[doc(hidden)]
pub struct TestTaskSource<F>(F);

    #[doc(hidden)]
pub struct TestTaskDrop<F>(std::mem::ManuallyDrop<Option<F>>);

    impl<F: Send + 'static> semio_framework_value::retirement::RetirementCursor for TestTaskDrop<F> {
        fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
            use semio_framework_value::retirement::RetirementStep;
            if self.0.is_none() { return RetirementStep::Complete; }
            if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
            drop(self.0.take());
            RetirementStep::Progress(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })
        }
        fn terminal_is_empty(&self) -> bool { self.0.is_none() }
        fn next_work_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
        fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
        fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
        fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(1) }
        fn allows_admitted_narrow_work(&self) -> bool { true }
        fn terminal_release_bytes(&self) -> Option<usize> { self.0.is_none().then_some(std::mem::size_of::<Self>()) }
    }

    impl<F: Send + 'static> semio_framework_value::retirement::RetireOwned for TestTaskSource<F> {
        fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { Box::new(TestTaskDrop(std::mem::ManuallyDrop::new(Some(self.0)))) }
        fn retirement_birth_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<TestTaskDrop<F>>()) }
        fn controlled_retirement_supported() -> bool { true }
    }

    #[doc(hidden)]
pub fn test_task<F, Fut>(label: &str, run: F) -> AsyncTask<TestMutation, TestConfigMutation, NoDraftMutation>
    where
        F: FnOnce(TaskCtx) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<TaskResolution<TestMutation, TestConfigMutation, NoDraftMutation>, Fault>> + 'static,
    {
        AsyncTask::new(label.to_string(), TestTaskSource(run), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation).unwrap_or_else(|(error, _, _)| panic!("test task constructor refused: {error}")).0
    }include!("../🔬️plugin-runtime-plugin-builder-contract-x/🦀️.rs");
