#[path = "../../🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use mutations::{SetTransactionCount, SetTransactionCountAndNotify, SetTransactionCountWithoutPreflight, TxnMutation};
crate::app_testing::mutation_fixture::wire::fixture_operation_text!(TxnMutation);

#[cfg(test)]
#[path = "../🧬️mutation-fixtures-transaction-unit-command-close/🦀️.rs"]
mod command_close_tests;

// 🧪️ Proves the `🧫️fixtures` transaction helpers and the underlying transaction machinery
// against a minimal `ArtifactApp` fixture whose notify mutation carries a real foreign step.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::app_testing::artifact_app_laws::{assert_proposes_transaction, assert_transaction_commits_as_one_edit, assert_transaction_rollback_leaves_state_untouched, meta, new_registered_app, settle_registered_typed_operation};
use crate::window_surface::{built_text_to_component_tree, UiAssemblyResult};
use crate::app_contract::ArtifactApp;
use crate::plugin_app::PluginApp;
use crate::tool_contract::ArtifactOwnedToolJobFactory;
use crate::tool_contract::ArtifactOwnedToolJobRequest;
use crate::tool_contract::ArtifactToolCompletion;
use crate::tool_contract::ArtifactToolFactoryRegistry;
use crate::tool_contract::ArtifactToolPublicationContract;
use crate::tool_contract::ArtifactToolPublicationLane;
use crate::app_views::{ArtifactView, ConfigView, DraftView, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation};
use crate::app_emission::Emit;
use crate::artifact_instance::VcsArtifactApp;
use crate::ViewModel;
use protocol::MutationDiff;
use semio_framework::{action_bus, ActionKind, Fault, IconName, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolOperationSpec};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use store::{os_store::backbone::Backbone, os_store::backbone::BackboneMessage, os_store::backbone::MemoryBackbone};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[artifact(extension = "testkit-txn")]
pub(crate) struct TxnSnapshot {
    pub(crate) count: i32,
}

impl store::os_store::serialization::ArtifactSqliteSnapshotHost for TxnSnapshot {
    
    
    
}
impl pack::snapshot::ArtifactSqliteSnapshot for TxnSnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
fn to_sqlite_database(&self, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteSnapshotPhase, SqliteValue};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("transaction_state")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::from(self.count))] });
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
        Ok(database)
    }
fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
        let rows = &database.table("transaction_state")?.rows;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "transaction_state requires one state row"));
        }
        let snapshot = Self { count: i32::try_from(rows[0].integer(1)?).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))? };
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(snapshot)
    }
}

impl semio_framework_schema_composition::ArtifactCompositionFields for TxnSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl pack::snapshot::ArtifactDsl for TxnSnapshot {
    const EXTENSION: &'static str = "testkit-txn";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(text).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl pack::snapshot::ArtifactPack for TxnSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    

    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) })
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for TxnSnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct TxnDiff {
    pub(crate) count: Option<i32>,
}

impl protocol::DiffAlgebra<TxnSnapshot> for TxnDiff {
    fn inverse(&self, base: &TxnSnapshot) -> Self {
        Self { count: self.count.map(|_| base.count) }
    }
    fn is_empty(&self) -> bool {
        self.count.is_none()
    }
}

impl MutationDiff<TxnSnapshot> for TxnDiff {
    fn apply(&self, snapshot: &TxnSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<TxnSnapshot> {
        Ok(TxnSnapshot { count: self.count.unwrap_or(snapshot.count) })
    }
    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
enum TxnCommand {
    #[dsl(key = "increment")]
    Increment,
    #[dsl(key = "streamed-increment")]
    StreamedIncrement,
    #[dsl(key = "increment-and-notify")]
    IncrementAndNotify,
}

impl ::protocol::OpText for TxnCommand {
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
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl ::protocol::OpBinary for TxnCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &["increment", "streamed-increment", "increment-and-notify"];

    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        ::dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🧪️TransactionRegisteredFactory
/// 🌊️ The one tool transaction every `streamed-increment` tick grows (design §15).
fn txn_stream_transaction() -> protocol::TransactionRef {
    protocol::TransactionRef { id: "tx-00000000000005ac".into(), tool: "s.testkit.txn#streamed-increment".into() }
}

const TXN_PAYLOAD_SCHEMA: &str = "semio.testkit-txn.command.v1";
const TXN_TOOL_IDS: [&str; 3] = ["increment", "streamed-increment", "increment-and-notify"];

struct TxnFixtureJob {
    /// 📄️ The admitted retained wire pages this job owns beside its command and completion.
    /// `admit_exact_wire` hands every typed dispatch its own paged raw input, and a factory that
    /// declines to take it is refused with `interactive-job.dispatch` ("tool factory does not own
    /// retained wire pages alongside its typed payload") before the reducer runs.
    owners: crate::app_testing::mutation_fixture::job_close::FixtureJobOwners<TxnApp>,
    count: i32,
    page: usize,
}

impl semio_framework_job::InteractiveJob for TxnFixtureJob {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        if cx.should_yield() {
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
        }
        if self.owners.completion_is_delivered(){return semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)}
        if self.owners.raw.as_ref().is_some_and(|raw| self.page < raw.page_count()) {
            self.page += 1;
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
        }
        let Some(command) = self.owners.command.as_deref() else {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        };
        let kind=match command{TxnCommand::Increment=>0u8,TxnCommand::StreamedIncrement=>1,TxnCommand::IncrementAndNotify=>2};
        if self.owners.pending_is_empty(){let value=self.count+1;let extra=if kind==1{"tx-00000000000005ac".len()+"s.testkit.txn#streamed-increment".len()}else{0};if !self.owners.prepare_emission(cx,std::mem::size_of::<i32>()+extra,extra,||Ok(match kind{0=>Emit{artifact_mutations:vec![SetTransactionCountWithoutPreflight{value}.into()],..Default::default()},1=>Emit::stream_transaction(txn_stream_transaction(),vec![SetTransactionCountWithoutPreflight{value}.into()]),_=>Emit{artifact_mutations:vec![SetTransactionCountAndNotify{value}.into()],..Default::default()}}))?{return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
        if !self.owners.complete(cx)?{return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
        semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        crate::app_testing::artifact_app_laws::fixture_job_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{self.owners.close_step(cx)}

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.demand(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.demand(body)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.demand(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.demand(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

struct TxnFixtureFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ToolJobFactory for TxnFixtureFactory {
    type Payload = TxnFixtureJob;
    type Job = TxnFixtureJob;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        TXN_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1)
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(payload)
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        mut payload: Self::Payload,
        input: action_bus::RetainedToolWireInput,
        checkpoint: Option<action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("transaction fixture resume starts a fresh command owner"), input, checkpoint));
        }
        payload.owners.raw = Some(input);
        Ok(payload)
    }
}

impl ArtifactOwnedToolJobFactory for TxnFixtureFactory {
    type Owner = TxnApp;
    const TOOL_IDS: &'static [&'static str] = &TXN_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TxnApp::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "increment", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "streamed-increment", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "increment-and-notify", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
    fn latest_wins_target(_command: &TxnCommand) -> Option<&str> {
        None
    }
    fn build_latest_wins_command_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<TxnCommand>>> {
        None
    }
}

async fn transaction_manifest() -> crate::app_builder::App {
    let mut builder = crate::app_builder::App::builder(TxnApp::APP_ID, LocalizedLabel::data("Transaction Fixture"))
        .await
        .document(["state"])
        .mode("edit", LocalizedLabel::data("Edit"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::data("Main"), "transaction.main", semio_framework_ui_contract::SurfaceKind::Canvas2d, IconName::AppWindow)
        .await;
    for tool_id in TXN_TOOL_IDS {
        builder = builder.app_command(tool_id, LocalizedLabel::data(tool_id), "fixture", ActionKind::Mutation).await;
    }
    crate::app_builder::App::from_builder(builder.interactive_jobs(semio_framework::InteractiveJobClassification::Migrated).await).await
}
//#endregion 🧪️TransactionRegisteredFactory

#[derive(Default)]
struct TxnApp;

impl ArtifactApp for TxnApp {
    const DIALECT: crate::Dialect = crate::Dialect { artifact_kind: "s.test.transaction", standard: crate::StandardId("1"), subset: crate::SubsetId::ANY };
    const APP_ID: &'static str = "s.test.transaction@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-txn/v1";
    type Snapshot = TxnSnapshot;
    type Mutation = TxnMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::app_views::NoTransient;
    type TransientMutation = crate::app_views::NoTransientMutation;
    type Command = TxnCommand;

    crate::bounded_first_step_tool_proofs! {
        owner: TxnApp, owner_file: "plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs", controller: "s.test.transaction@1/*#editor", artifact_schema: "semio.testkit-txn/v1",
        factory: "TxnFixtureFactory", factory_type: TxnFixtureFactory,
        contract: ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1), tools: ["increment", "streamed-increment", "increment-and-notify"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        registry.register(TxnFixtureFactory { keys: TXN_TOOL_IDS.into_iter().map(|tool_id| ToolFactoryKey::new(registry.controller_id(), tool_id)).collect() })
    }

    async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
        let job = TxnFixtureJob { owners: crate::app_testing::mutation_fixture::job_close::FixtureJobOwners::new(request.command, request.completion).with_context(request.context), count: request.snapshot.count, page: 0 };
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
    }

    /// 🪪️ The tool id each variant dispatches under. Without it the `ArtifactApp` default answers
    /// `"typed-command"` for every variant, which matches no registered factory key and makes every
    /// `dispatch_typed` on this fixture `interactive-job.missing-factory`.
    async fn command_id(command: &TxnCommand) -> &'static str {
        match command {
            TxnCommand::Increment => TXN_TOOL_IDS[0],
            TxnCommand::StreamedIncrement => TXN_TOOL_IDS[1],
            TxnCommand::IncrementAndNotify => TXN_TOOL_IDS[2],
        }
    }

    async fn initial_snapshot() -> TxnSnapshot {
        TxnSnapshot::default()
    }

    async fn handle(
        command: &TxnCommand,
        doc: &ArtifactView<'_, TxnSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &crate::app_views::InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<TxnMutation>, Fault> {
        match command {
            TxnCommand::Increment => Ok(Emit { artifact_mutations: vec![SetTransactionCountWithoutPreflight { value: doc.snapshot.count + 1 }.into()], ..Default::default() }),
            TxnCommand::StreamedIncrement => Ok(Emit::stream_transaction(txn_stream_transaction(), vec![SetTransactionCountWithoutPreflight { value: doc.snapshot.count + 1 }.into()])),
            TxnCommand::IncrementAndNotify => Ok(Emit { artifact_mutations: vec![SetTransactionCountAndNotify { value: doc.snapshot.count + 1 }.into()], ..Default::default() }),
        }
    }

    async fn render(_body_key: &str, doc: &ArtifactView<'_, TxnSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}", doc.snapshot.count)))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(crate::app_testing::mutation_fixture::wire::preparation_factory::<Self::Snapshot, Self::Mutation>("testkit-txn-artifact-retained"))
    }

    fn document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { crate::app_testing::mutation_fixture::wire::document_store_owners_source_demands::<Self::Snapshot, Self::Mutation>() }
    fn build_document_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(crate::app_testing::mutation_fixture::wire::admit_document_store_owners::<Self::Snapshot, Self::Mutation>("testkit-txn-artifact-retained", grant))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }
    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }
    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>())
    }
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(crate::app_testing::mutation_fixture::no_state::presence_store_disposer())
    }
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(crate::app_testing::mutation_fixture::no_state::transient_store_disposer())
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app_testing::mutation_fixture::no_state::presence_peer_retirement_factory())
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app_testing::mutation_fixture::no_state::presence_local_root_retirement_factory())
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(crate::app_testing::mutation_fixture::no_state::transient_local_root_retirement_factory())
    }
}

fn close_transaction_store_roots(app: &mut VcsArtifactApp<TxnApp>, cx:&mut semio_framework_job::StepContext<'_>) {
    crate::app_testing::artifact_app_laws::close_registered_fixture_app(app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");
}

#[semio_framework_async_macros::async_test]
async fn dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let draft = assert_proposes_transaction(&mut app, TxnCommand::IncrementAndNotify, crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity(),cx).await;
    assert_eq!(draft.local_ops.len(), 1, "the local op must still be encoded for the proposal");
    assert_eq!(draft.foreign.len(), 1, "the foreign step must be reported");
    assert_eq!(draft.foreign[0].target.artifact_id, "peer-doc");
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🔁️ One dispatch as the host drives it: a mounted app answers with an ADMISSION receipt and hands
/// the reducer to a worker, so the store only advances once the operation settles. Every law below
/// reads `snapshot()`/the history right after its dispatch, which is only true after this.
async fn dispatch_settled(app: &mut VcsArtifactApp<TxnApp>, command: TxnCommand, actor: &str, cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework::InvocationResult, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let action_meta = meta(actor);
    let admitted = app.dispatch_typed(command, &action_meta, &mut crate::app_testing::artifact_app_laws::fixture_identity()).await?;
    settle_registered_typed_operation(app, action_meta.instance_id, crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity(),cx).await?;
    Ok(admitted)
}

#[semio_framework_async_macros::async_test]
async fn plain_command_still_applies_normally() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    dispatch_settled(&mut app, TxnCommand::Increment, "local",cx).await.expect("increment");
    assert_eq!(app.snapshot().unwrap().count, 1);
    assert!(app.take_pending_transaction_proposal().await.is_none(), "a plain command must not stash a proposal");
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn command_cache_inputs_share_immutable_arcs() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    app.render_ctx().refresh_cache().await.expect("refresh cache");
    let (_, cached_snapshot, cached_config, cached_history) = app.history.cache.as_ref().expect("cache");
    let (snapshot, config, history) = app.history.command_cache_inputs();
    assert!(std::sync::Arc::ptr_eq(cached_snapshot, &snapshot));
    assert!(std::sync::Arc::ptr_eq(cached_config, &config));
    assert!(std::sync::Arc::ptr_eq(cached_history, &history));
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn a_streamed_tick_extends_cached_history_in_place() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    dispatch_settled(&mut app, TxnCommand::StreamedIncrement, "local",cx).await.expect("first increment");
    let history_ptr = std::sync::Arc::as_ptr(&app.history.cache.as_ref().expect("first history cache").3);
    dispatch_settled(&mut app, TxnCommand::StreamedIncrement, "local",cx).await.expect("second increment");
    app.render_ctx().refresh_cache().await.expect("extend history cache");
    let history = &app.history.cache.as_ref().expect("extended history cache").3;
    assert_eq!(std::sync::Arc::as_ptr(history), history_ptr, "a streamed tick must update the uniquely-owned history allocation in place");
    assert_eq!(app.stores.store.envelope().vcs.edits.len(), 1, "streamed increments stay one open edit");
    assert_eq!(history.commands.len(), 1, "streamed increments stay one command row");
    assert_eq!(history.commands[0].op_lines.len(), 2, "only the new operation tail is appended to cached history");
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn commit_produces_exactly_one_edit_with_group_id_and_origin() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let origin = protocol::MutationOrigin::Transaction { initiator: protocol::ForeignTarget { artifact_id: "initiator-doc".into(), artifact_kind: "s.testkit.txn".into(), dialect: None } };
    let edit_id = assert_transaction_commits_as_one_edit(&mut app, "txn-1", vec![SetTransactionCount { value: 7 }.into()], origin, &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    assert_eq!(app.snapshot().unwrap().count, 7);
    assert!(!edit_id.is_empty());
    assert!(app.transaction_commit("txn-1", &meta("local"), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.is_err(), "committing an already-committed txn_id must fail, not double-apply");
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn rollback_leaves_state_untouched() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    dispatch_settled(&mut app, TxnCommand::Increment, "local",cx).await.expect("increment");
    assert_transaction_rollback_leaves_state_untouched(&mut app, "txn-2", vec![SetTransactionCount { value: 99 }.into()]).await;
    assert_eq!(app.snapshot().unwrap().count, 1, "rollback must leave the earlier state exactly as it was");
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🔀️ Contract §5.8 and §5.10 are COMPLEMENTARY, and this test is where that shows: §5.10
/// already blocks every local mutating command while a transaction is pending, so a local
/// edit can never be what moves the generation out from under a prepared member. The only
/// remaining way for the base generation to go stale is an edit that does not come from a
/// command at all — a remote envelope ingested from the backbone mid-transaction — which is
/// precisely the race §5.8's check exists to catch. Driving this through `dispatch_typed`
/// instead only proves §5.10 a second time (it rejects with `transaction.instance-busy`).
///
/// ↩️ A rejected commit RESTORES the pending transaction rather than discarding it (contract
/// §5.8), so the only honest way out of this state is the explicit rollback the refusal invites.
///
/// 🔌️ The sender still holds the near end of the pair; a store whose backbone is attached
/// retains that channel owner, and its close is `Blocked` until the attachment is released.
#[semio_framework_async_macros::async_test]
async fn generation_mismatch_is_rejected_with_the_frozen_code() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut sender = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId("remote".into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let (near, mut far) = MemoryBackbone::pair("mem://txn", "mem://txn").await;
    sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach");
    dispatch_settled(&mut sender, TxnCommand::Increment, "remote",cx).await.expect("the peer edits its own copy");
    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
        }
    }
    assert!(!envelopes.is_empty(), "the peer's edit must reach the channel");
    let operations = protocol::encode_envelopes(&envelopes);

    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let outcome = app.transaction_prepare("txn-3", "", &[], &[::protocol::OpBinary::encode_op(&TxnMutation::from(SetTransactionCount { value: 5 })).expect("encode")], &[], Some(protocol::MutationOrigin::Owner)).await;
    assert!(outcome.rejection.is_none());
    app.ingest_operations(&operations, &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.expect("a remote edit lands while the transaction is pending");
    let error = app.transaction_commit("txn-3", &meta("local"), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.expect_err("commit must reject a stale generation");
    assert_eq!(error.code.0, "transaction.generation-mismatch");
    app.transaction_rollback("txn-3").await.expect("a rejected commit leaves the transaction pending and explicitly rollback-able");
    sender.detach_backbone().await.expect("sender releases its backbone before close");
    drop(far);
    close_transaction_store_roots(&mut sender,cx);
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn second_prepare_while_pending_is_rejected_instance_busy() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let first = app.transaction_prepare("txn-4a", "", &[], &[::protocol::OpBinary::encode_op(&TxnMutation::from(SetTransactionCount { value: 1 })).expect("encode")], &[], Some(protocol::MutationOrigin::Owner)).await;
    assert!(first.rejection.is_none());
    let second = app.transaction_prepare("txn-4b", "", &[], &[::protocol::OpBinary::encode_op(&TxnMutation::from(SetTransactionCount { value: 2 })).expect("encode")], &[], Some(protocol::MutationOrigin::Owner)).await;
    let rejection = second.rejection.expect("second prepare while pending must be rejected");
    assert_eq!(rejection.code.0, "transaction.instance-busy");
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🔖️ Read-only surfaces stay unaffected — `render`/`snapshot` never go through
/// `dispatch_emit` at all, matching contract §5.10's carve-out for
/// RefreshUi/ReadDocument/ContextMenu/ephemeral lanes.
#[semio_framework_async_macros::async_test]
async fn a_mutating_command_while_pending_is_rejected_but_reads_still_work() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let prepared = app.transaction_prepare("txn-5", "", &[], &[::protocol::OpBinary::encode_op(&TxnMutation::from(SetTransactionCount { value: 1 })).expect("encode")], &[], Some(protocol::MutationOrigin::Owner)).await;
    assert!(prepared.rejection.is_none());
    let blocked = dispatch_settled(&mut app, TxnCommand::Increment, "local",cx).await;
    assert!(blocked.is_err(), "a command emitting artifact mutations must be rejected while a transaction is pending");
    assert_eq!(blocked.unwrap_err().code.0, "transaction.instance-busy");
    assert_eq!(app.snapshot().unwrap().count, 0, "the pending transaction must not have applied anything yet");
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn undo_and_redo_by_group() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    assert_transaction_commits_as_one_edit(&mut app, "txn-6", vec![SetTransactionCount { value: 42 }.into()], protocol::MutationOrigin::Owner, &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    assert_eq!(app.snapshot().unwrap().count, 42);
    app.transaction_undo("txn-6", &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.expect("undo the group");
    assert_eq!(app.snapshot().unwrap().count, 0, "undo must revert the transaction's edit");
    app.transaction_redo("txn-6", &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.expect("redo the group");
    assert_eq!(app.snapshot().unwrap().count, 42, "redo must reapply the transaction's edit");
    close_transaction_store_roots(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}
