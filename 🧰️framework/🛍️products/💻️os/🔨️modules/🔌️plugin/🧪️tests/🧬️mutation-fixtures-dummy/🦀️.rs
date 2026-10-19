#[derive(Clone,Copy)]
pub struct DummyMemberStoreProducer;

#[path = "../../🧪️testing/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use crate::testing_laws_unit_1::mutation_fixture::dummy::mutations::DummyMutation;
pub(crate) use crate::testing_laws_unit_1::mutation_fixture::dummy::mutations::set_dummy_count::SetDummyCount;
crate::testing_laws_unit_1::mutation_fixture::wire::fixture_operation_text!(DummyMutation);

// 🧪️ Proves each `artifact_app_laws` primitive against a minimal dummy `ArtifactApp` before any real app
// adopts them.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_plugin_testing_laws::artifact_app_laws::{assert_registered_ingest_idempotent, assert_registered_ingest_idempotent_with_members, assert_two_registered_instances_converge, assert_two_registered_instances_converge_with_members, assert_undo_redo_round_trip, close_registered_fixture_app, meta, new_app, new_registered_app};
use semio_framework_plugin_window_transient::built_text_to_component_tree;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobRequest;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolFactoryRegistry;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationContract;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, DraftView, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation};
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::ViewModel;
use protocol::MutationDiff;
use protocol::ColdMutationDiff;
use semio_framework::{action_bus, ActionKind, Fault, IconName, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolOperationSpec};
use semio_framework_2d_compute::EngineHandles;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[artifact(extension = "testkit-dummy")]
pub(crate) struct DummySnapshot {
    pub count: i32,
}

impl store::os_store::serialization::ArtifactSqliteSnapshotHost for DummySnapshot {



}
impl pack::snapshot::ArtifactSqliteSnapshot for DummySnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
fn to_sqlite_database(&self, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteSnapshotPhase, SqliteValue};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("dummy_state")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::from(self.count))] });
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
        Ok(database)
    }
fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
        let rows = &database.table("dummy_state")?.rows;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "dummy_state requires one state row"));
        }
        let snapshot = Self { count: i32::try_from(rows[0].integer(1)?).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))? };
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(snapshot)
    }
}

impl semio_framework_schema_composition::ArtifactCompositionFields for DummySnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack for SDK test double (artifact coincides with snapshot only in tests).
impl pack::snapshot::ArtifactDsl for DummySnapshot {
    const EXTENSION: &'static str = "testkit-dummy";
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

impl pack::snapshot::ArtifactPack for DummySnapshot {
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
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for DummySnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct DummyDiff {
    pub count: Option<i32>,
}

impl protocol::DiffAlgebra<DummySnapshot> for DummyDiff {

    fn is_empty(&self) -> bool {
        self.count.is_none()
    }
}

impl protocol::ColdDiffAlgebra<DummySnapshot> for DummyDiff {
    fn inverse(&self, base: &DummySnapshot) -> Self {
        Self { count: self.count.map(|_| base.count) }
    }
}

impl MutationDiff<DummySnapshot> for DummyDiff {



}

impl ColdMutationDiff<DummySnapshot> for DummyDiff {
    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
    }

    fn apply(&self, snapshot: &mut DummySnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
                if let Some(value) = self.count { snapshot.count = value; }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
enum DummyCommand {
    #[dsl(key = "increment")]
    Increment,
}

impl ::protocol::OpText for DummyCommand {
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
    /// 📄️ A factory that does not own the admitted retained wire pages alongside its typed payload is
    /// refused with `interactive-job.dispatch`, so the owners hold them beside the command and completion.
    pub owners: crate::testing_laws_unit_1::mutation_fixture::job_close::FixtureJobOwners<DummyApp>,
    pub count: i32,
    pub page: usize,
}

impl semio_framework_job::InteractiveJob for DummyFixtureJob {
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
        let Some(DummyCommand::Increment) = self.owners.command() else {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        };
        if self.owners.pending_is_empty(){let value=self.count+1;if !self.owners.prepare_emission(cx,std::mem::size_of::<i32>(),0,||Ok(Emit::mutations(vec![SetDummyCount{value}.into()])))?{return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
        if !self.owners.complete(cx)?{return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
        semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        semio_framework_plugin_testing_laws::artifact_app_laws::fixture_job_outcome(descriptor)
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

struct DummyFixtureFactory {
    pub keys: Vec<ToolFactoryKey>,
}

impl ToolJobFactory for DummyFixtureFactory {
    type Payload = DummyFixtureJob;
    type Job = DummyFixtureJob;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        DUMMY_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1)
    }
    fn admission_demands(&self,_operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&semio_framework::RetainedToolWireInput>,checkpoint:Option<&semio_framework::RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{if checkpoint.is_some()||(input.is_some()&&payload.owners.raw.is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original typed job refuses a replacement wire owner or restart checkpoint"));}Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())}
fn admit_job(&mut self,context:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<semio_framework::RetainedToolWireInput>,checkpoint:&mut Option<semio_framework::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed job payload is absent"))?,input.as_ref(),checkpoint.as_ref())?;semio_framework::action_bus::admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|_,payload,input,_|{if input.is_some(){payload.as_mut().expect("retained original job").owners.raw=input.take();}let job=payload.take().expect("admitted same original job");Ok((job,semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})}

}

impl ArtifactOwnedToolJobFactory for DummyFixtureFactory {
    type Owner = DummyApp;
    const TOOL_IDS: &'static [&'static str] = &[DUMMY_TOOL_ID];
    const DOCUMENT_SCHEMA: &'static str = DummyApp::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: DUMMY_TOOL_ID, lanes: &[ArtifactToolPublicationLane::Artifact] }];
    fn latest_wins_target(_command: &DummyCommand) -> Option<&str> {
        None
    }
    fn build_latest_wins_command_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<DummyCommand>>> {
        None
    }
}

async fn dummy_manifest() -> semio_framework_plugin_app_builder::App {
    semio_framework_plugin_app_builder::App::from_builder(
        semio_framework_plugin_app_builder::App::builder(DummyApp::APP_ID, LocalizedLabel::data("Dummy Fixture"))
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
    const DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.test.dummy", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };
    const APP_ID: &'static str = "s.test.dummy@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit/v1";
    fn catalogue_example_document(example_id: &str) -> Option<Result<String, semio_framework::Fault>> {
        if example_id.is_empty() {
            return Some(Ok(<DummySnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&DummySnapshot::default())));
        }
        if example_id == "four" {
            return Some(Ok(<DummySnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&DummySnapshot { count: 4 })));
        }
        Some(Err(semio_framework::Fault::new(semio_framework::FaultOrigin::App, semio_framework::FaultCode::new("app.example.unknown"), format!("no registered example '{example_id}'"))))
    }
    type Snapshot = DummySnapshot;
    type Mutation = DummyMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = semio_framework_plugin_decl_views::NoTransient;
    type TransientMutation = semio_framework_plugin_decl_views::NoTransientMutation;
    type Command = DummyCommand;

    semio_framework_plugin_decl_app::bounded_first_step_tool_proofs! {
        owner: DummyApp, owner_file: "plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs", controller: "s.test.dummy@1/*#editor", artifact_schema: "semio.testkit/v1",
        factory: "DummyFixtureFactory", factory_type: DummyFixtureFactory,
        contract: ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1), tools: ["increment"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        registry.register(DummyFixtureFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), DUMMY_TOOL_ID)] })
    }

    fn build_tool_job(request:&mut Option<ArtifactOwnedToolJobRequest<Self>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{crate::app_testing::mutation_fixture::job_close::admit_fixture_tool_job(request,output,cx,|request|{let count=request.snapshot.count;DummyFixtureJob{owners:crate::testing_laws_unit_1::mutation_fixture::job_close::FixtureJobOwners::from_request(request),count,page:0}})}

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(crate::testing_laws_unit_1::mutation_fixture::wire::preparation_factory::<Self::Snapshot, Self::Mutation>("testkit-dummy-artifact-retained"))
    }

    fn document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { crate::testing_laws_unit_1::mutation_fixture::wire::document_store_owners_source_demands::<Self::Snapshot, Self::Mutation>() }
    fn build_document_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(crate::testing_laws_unit_1::mutation_fixture::wire::admit_document_store_owners::<Self::Snapshot, Self::Mutation>("testkit-dummy-artifact-retained", grant))
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
        Some(crate::testing_laws_unit_1::mutation_fixture::no_state::presence_store_disposer())
    }
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(crate::testing_laws_unit_1::mutation_fixture::no_state::transient_store_disposer())
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::testing_laws_unit_1::mutation_fixture::no_state::presence_peer_retirement_factory())
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::testing_laws_unit_1::mutation_fixture::no_state::presence_local_root_retirement_factory())
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(crate::testing_laws_unit_1::mutation_fixture::no_state::transient_local_root_retirement_factory())
    }

    /// 🪪️ The tool id this variant dispatches under. Without it the `ArtifactApp` default answers
    /// `"typed-command"`, which matches no registered factory key, so every `dispatch_typed` on this
    /// fixture is `interactive-job.missing-factory` before it reaches the reducer.
    async fn command_id(command: &DummyCommand) -> &'static str {
        match command {
            DummyCommand::Increment => DUMMY_TOOL_ID,
        }
    }

    async fn initial_snapshot() -> DummySnapshot {
        DummySnapshot::default()
    }

    async fn handle(
        command: &DummyCommand,
        doc: &ArtifactView<'_, DummySnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<DummyMutation>, Fault> {
        match command {
            DummyCommand::Increment => Ok(Emit { artifact_mutations: vec![SetDummyCount { value: doc.snapshot.count + 1 }.into()], ..Default::default() }),
        }
    }

    async fn render(_body_key: &str, doc: &ArtifactView<'_, DummySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}", doc.snapshot.count)))
    }
}

/// 🎟️ Supplies the neutral fixture's independent caller ceilings before any artifact work.
fn dummy_fixture_caller() -> (semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, usize) {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🧾️fixture-caller/🔣️.json")).unwrap();
    let grant = &law["native"]["grant"];
    let original = semio_framework_value::RetainedCloneGrant { maximum_items: grant[0].as_u64().unwrap() as usize, maximum_copy_bytes: grant[1].as_u64().unwrap() as usize, maximum_capacity_bytes: grant[2].as_u64().unwrap() as usize, maximum_release_bytes: grant[3].as_u64().unwrap() as usize, maximum_depth: grant[4].as_u64().unwrap() as usize };
    (semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: original, maintenance: original, close: original }.validate().unwrap(), law["native"]["maximumIdentityBytes"].as_u64().unwrap() as usize)
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_the_registered_catalogue_without_a_declared_action() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let (mounted_policy, maximum_identity_bytes) = dummy_fixture_caller();
    let mut observer = |_| true;
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let mut app = new_app::<DummyApp>(protocol::ActorId("actor".into()), mounted_policy, &mut identity).await;
    let args = semio_framework_value::DslValue::object([("exampleId".to_string(), semio_framework_value::DslValue::String("four".to_string()))]);
    let result = app.dispatch_action("setActiveExample", Some(&args), &meta("actor"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("catalogue load");
    let semio_framework::kernel::Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("load effect") else {
        panic!("expected LoadDocument");
    };
    let loaded = <DummySnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).expect("pack");
    assert_eq!(loaded, DummySnapshot { count: 4 });
    let missing = semio_framework_value::DslValue::object([("exampleId".to_string(), semio_framework_value::DslValue::String("missing".to_string()))]);
    let refused = app.dispatch_action("setActiveExample", Some(&missing), &meta("actor"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect_err("unknown example");
    assert_eq!(refused.code.0, "app.example.unknown");
    close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn meta_carries_actor_and_local_instance_id() {
    let m = meta("actor-x");
    assert_eq!(m.actor.as_str(), "actor-x");
    assert_eq!(m.instance_id, 1);
}

#[semio_framework_async_macros::async_test]
async fn new_app_constructs_a_registry_less_wrapper() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let (mounted_policy, maximum_identity_bytes) = dummy_fixture_caller();
    let mut observer = |_| true;
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let mut app = new_app::<DummyApp>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
    let error = app.dispatch_typed(DummyCommand::Increment, &meta("local"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect_err("registry-less wrapper must fail closed");
    assert_eq!(error.code.0, "interactive-job.unknown-key");
    assert!(error.message.contains("no exact manifest declaration"), "the registry-less wrapper fails on the missing declaration, not on a missing factory: {}", error.message);
    assert_eq!(app.snapshot().unwrap().count, 0, "a fail-closed dispatch never reaches the reducer");
    close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn assert_undo_redo_round_trip_passes_for_a_real_operation() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let (mounted_policy, maximum_identity_bytes) = dummy_fixture_caller();
    let mut observer = |_| true;
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let mut app = new_registered_app::<DummyApp, _>(dummy_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
    assert_undo_redo_round_trip(&mut app, DummyCommand::Increment, |app| app.snapshot().unwrap().count, 0, 1, mounted_policy, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await;
    close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn assert_two_instances_converge_on_disjoint_edits() {
    let (mounted_policy, maximum_identity_bytes) = dummy_fixture_caller();
    let mut observer = |_| true;
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    assert_two_registered_instances_converge::<DummyApp, i32, _, _>("mem://testkit-converge", dummy_manifest, DummyCommand::Increment, DummyCommand::Increment, |app| app.snapshot().unwrap().count, mounted_policy, &mut identity).await;
}

#[semio_framework_async_macros::async_test]
async fn assert_ingest_idempotent_does_not_double_apply() {
    let (mounted_policy, maximum_identity_bytes) = dummy_fixture_caller();
    let mut observer = |_| true;
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    assert_registered_ingest_idempotent::<DummyApp, i32, _, _>(dummy_manifest, DummyCommand::Increment, |app| app.snapshot().unwrap().count, mounted_policy, &mut identity).await;
}

//#region 🧬️MembersRoster
// 🧬️ A non-default member roster over the dummy document shape — a "child" here is a second,
// independently owned instance of the identical shape, exactly as the plugin-runtime fixture's
// `TestMembers` is. Its only purpose is to be a roster that is NOT `NoMembers`.
store::space_members! {
    native_producer:&::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    history_limits:semio_framework_os_kernel::os_store::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},
    maximum_history_allocation_bytes:8388608,
    pub enum DummyMembers, DummyMembersOpen {
        Child("s.test.dummy-child", "native", "*", "semio.testkit-child/v1") => (DummySnapshot, DummyMutation, DummyMemberStoreProducer, DummyMemberStoreProducer),
    }
}

impl store::os_store::document_store_owners::MemberStoreProducer<DummySnapshot,DummyMutation> for DummyMemberStoreProducer {
    /// 🚪️ This roster exists to BE a non-default roster, never to open a member: the law that uses it
    /// drives the registered ladder (construct, attach, dispatch, settle, exchange, close) and admits
    /// no child, so the un-openable operation is the honest declaration.
    type SnapshotOpen = store::UnsupportedMemberSnapshotOpen<DummySnapshot>;

    fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        store::os_store::document_store_owners::bounded_artifact_store_owners_birth_demand::<DummySnapshot, DummyMutation>()
    }

    fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<DummySnapshot, DummyMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<DummySnapshot, DummyMutation>> {
        semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_owners::<DummySnapshot, DummyMutation>(grant)
    }
}
//#endregion 🧬️MembersRoster

/// 🧬️ A members-bearing app converges and ingests idempotently through the `_with_members` twins.
///
/// The registered fixture constructors were hard-wired to `VcsArtifactApp<A>`, i.e. the DEFAULT
/// roster, and the roster is a type parameter of the app the helper CONSTRUCTS — so no plugin-side
/// change could reach it. An app composing a derived child therefore failed construction outright
/// (`derived child dialect '…' is not declared by this app's member roster`) and every registered
/// convergence/idempotency law was unreachable for it (mathematical's equation, imperative's
/// procedure; ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, knowledge). This law drives the
/// whole registered ladder — paired construction, backbone attach, typed dispatch, settle, exchange,
/// checkpoint, detach, close — over `DummyMembers` instead of `NoMembers`, so the roster really is
/// threaded through every rung and not just accepted at the signature.
#[semio_framework_async_macros::async_test]
async fn registered_laws_accept_an_explicit_member_roster() {
    let (mounted_policy, maximum_identity_bytes) = dummy_fixture_caller();
    let mut observer = |_| true;
    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    assert_two_registered_instances_converge_with_members::<DummyApp, DummyMembers, i32, _, _>("mem://testkit-converge-members", dummy_manifest, DummyCommand::Increment, DummyCommand::Increment, |app| app.snapshot().unwrap().count, mounted_policy, &mut identity).await;
    assert_registered_ingest_idempotent_with_members::<DummyApp, DummyMembers, i32, _, _>(dummy_manifest, DummyCommand::Increment, |app| app.snapshot().unwrap().count, mounted_policy, &mut identity).await;
}
