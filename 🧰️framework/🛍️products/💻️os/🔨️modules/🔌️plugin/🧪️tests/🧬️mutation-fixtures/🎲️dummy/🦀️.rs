#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use mutations::{DummyMutation, SetDummyCount};

// 🧪️ Proves each `testkit` primitive against a minimal dummy `ArtifactApp` before any real app
// adopts them.
use crate::app::{built_text_to_component_tree, ArtifactApp, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, DraftView, Emit, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, UiAssemblyResult};
use crate::app::testkit::{assert_registered_ingest_idempotent, assert_two_registered_instances_converge, assert_undo_redo_round_trip, close_registered_fixture_app, meta, new_app, new_registered_app};
use protocol::{Mutation, MutationDiff};
use semio_framework::{ActionKind, Fault, IconName, ToolFactoryKey, ToolJobFactory, ToolOperationSpec, ToolExecutionContract};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;
use ui_wgpu::wgpu::LocalizedLabel;

#[derive(Clone, Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue, dsl::DslArtifact)]
#[dsl(extension = "testkit-dummy")]
pub(crate) struct DummySnapshot {
    count: i32,
}

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack for SDK test double (artifact coincides with snapshot only in tests).
impl store::ArtifactDsl for DummySnapshot {
    const EXTENSION: &'static str = "testkit-dummy";
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl store::ArtifactPack for DummySnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct DummyDiff {
    count: Option<i32>,
}

impl MutationDiff<DummySnapshot> for DummyDiff {
    fn apply(&self, snapshot: &DummySnapshot) -> protocol::MutationApplyResult<DummySnapshot> {
        Ok(DummySnapshot { count: self.count.unwrap_or(snapshot.count) })
    }

    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, dsl::DslOps)]
enum DummyCommand {
    #[dsl(key = "increment")]
    Increment,
}

impl ::protocol::OpText for DummyCommand {
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
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl ::protocol::OpBinary for DummyCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &["increment"];

    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        ::dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🧪️DummyRegisteredFactory
const DUMMY_TOOL_ID: &str = "increment";
const DUMMY_PAYLOAD_SCHEMA: &str = "semio.testkit-dummy.command.v1";

struct DummyFixtureJob {
    command: Option<Box<DummyCommand>>,
    completion: Option<ArtifactToolCompletion<DummyApp>>,
    count: i32,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for DummyFixtureJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() { return semio_framework_job::StepOutcome::Cancelled; }
        if cx.should_yield() { return semio_framework_job::StepOutcome::Yield; }
        let Some(DummyCommand::Increment) = self.command.as_deref() else { return semio_framework_job::StepOutcome::Cancelled; };
        self.completion.as_ref().expect("dummy fixture completion").complete(Ok(Emit::mutations(vec![SetDummyCount { value: self.count + 1 }.into()])), crate::app::EphemeralEmit::default()).expect("one exact dummy completion");
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 { return semio_framework_job::InteractiveJobCloseStep::Blocked; }
        if self.command.take().is_some() || self.completion.take().is_some() { return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }; }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool { self.closing && self.command.is_none() && self.completion.is_none() }
}

struct DummyFixtureFactory { keys: Vec<ToolFactoryKey> }

impl ToolJobFactory for DummyFixtureFactory {
    type Payload = DummyFixtureJob;
    type Job = DummyFixtureJob;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { DUMMY_PAYLOAD_SCHEMA }
    fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
}

impl ArtifactOwnedToolJobFactory for DummyFixtureFactory {
    type Owner = DummyApp;
    const TOOL_IDS: &'static [&'static str] = &[DUMMY_TOOL_ID];
    const DOCUMENT_SCHEMA: &'static str = DummyApp::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: DUMMY_TOOL_ID, lanes: &[ArtifactToolPublicationLane::Artifact] }];
    fn latest_wins_target(_command: &DummyCommand) -> Option<&str> { None }
    fn build_latest_wins_command_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<DummyCommand>>> { None }
}

async fn dummy_manifest() -> crate::app::App {
    crate::app::App::from_builder(
        crate::app::App::builder(DummyApp::APP_ID, LocalizedLabel::data("Dummy Fixture"))
            .await
            .document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "dummy.main", semio_framework_ui_contract::SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .app_command(DUMMY_TOOL_ID, LocalizedLabel::data("Increment"), "fixture", ActionKind::Mutation)
            .await
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
            .await,
    )
    .await
}
//#endregion 🧪️DummyRegisteredFactory

#[derive(Default)]
struct DummyApp;

impl ArtifactApp for DummyApp {
    const APP_ID: &'static str = "testkit-dummy";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit/v1";
    type Snapshot = DummySnapshot;
    type Mutation = DummyMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::app::NoTransient;
    type TransientMutation = crate::app::NoTransientMutation;
    type Command = DummyCommand;

    crate::bounded_first_step_tool_proofs! {
        owner: DummyApp, owner_file: "plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🦀️.rs", controller: "testkit-dummy", document_schema: "semio.testkit/v1",
        factory: "DummyFixtureFactory", factory_type: DummyFixtureFactory,
        contract: ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1), tools: ["increment"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        registry.register(DummyFixtureFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), DUMMY_TOOL_ID)] })
    }

    async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
        let job = DummyFixtureJob { command: Some(request.command), completion: Some(request.completion), count: request.snapshot.count, closing: false };
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> { Some(crate::app::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>()) }
    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> { Some(crate::app::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>()) }
    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> { Some(crate::app::bounded_document_store_owners::<Self::Draft, Self::DraftMutation>()) }
    fn build_document_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> { Some(crate::app::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>()) }
    fn build_config_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> { Some(crate::app::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>()) }
    fn build_draft_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> { Some(crate::app::bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>()) }
    fn build_presence_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { Some(crate::app::mutation_fixture::no_state::presence_store_disposer()) }
    fn build_transient_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { Some(crate::app::mutation_fixture::no_state::transient_store_disposer()) }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_peer_retirement_factory()) }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_local_root_retirement_factory()) }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> { Some(crate::app::mutation_fixture::no_state::transient_local_root_retirement_factory()) }

    async fn initial_snapshot() -> DummySnapshot {
        DummySnapshot::default()
    }

    async fn handle(
        command: &DummyCommand,
        doc: &ArtifactView<'_, DummySnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &crate::app::InteractionView<'_>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<DummyMutation>, Fault> {
        match command {
            DummyCommand::Increment => Ok(Emit { artifact_mutations: vec![SetDummyCount { value: doc.snapshot.count + 1 }.into()], description: Some("increment".into()), ..Default::default() }),
        }
    }

    async fn render(_body_key: &str, doc: &ArtifactView<'_, DummySnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("count={}", doc.snapshot.count)))
    }
}

#[semio_framework_async_macros::async_test]
async fn meta_carries_actor_and_local_instance_id() {
    let m = meta("actor-x");
    assert_eq!(m.actor, "actor-x");
    assert_eq!(m.instance_id, 1);
}

#[semio_framework_async_macros::async_test]
async fn new_app_constructs_a_registry_less_wrapper() {
    let mut app = new_app::<DummyApp>().await;
    let error = app.dispatch_typed(DummyCommand::Increment, &meta("local")).await.expect_err("registry-less wrapper must fail closed");
    assert_eq!(error.code.0, "interactive-job.missing-factory");
    close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn assert_undo_redo_round_trip_passes_for_a_real_operation() {
    let mut app = new_registered_app::<DummyApp, _>(dummy_manifest()).await;
    assert_undo_redo_round_trip(&mut app, DummyCommand::Increment, |app| app.snapshot().unwrap().count, 0, 1).await;
    close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn assert_two_instances_converge_on_disjoint_edits() {
    assert_two_registered_instances_converge::<DummyApp, i32, _, _>("mem://testkit-converge", dummy_manifest, DummyCommand::Increment, DummyCommand::Increment, |app| app.snapshot().unwrap().count).await;
}

#[semio_framework_async_macros::async_test]
async fn assert_ingest_idempotent_does_not_double_apply() {
    assert_registered_ingest_idempotent::<DummyApp, i32, _, _>(dummy_manifest, DummyCommand::Increment, |app| app.snapshot().unwrap().count).await;
}
