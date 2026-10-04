#[path = "../../🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use mutations::{SetSurfaceCount, SurfaceMutation};

#[path = "../🎞️media-owner-context/🦀️.rs"]
mod media_owner_context;

// 🧪️ Proves the viewer helpers against a minimal editor/viewer pair sharing one dialect.
use crate::app::artifact_app_laws::new_registered_app;
use crate::app::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates, close_registered_fixture_app, meta, new_app, new_viewer};
use crate::app::{
    built_text_to_component_tree, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry,
    ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ArtifactViewer, ChildContentView, ConfigView, DraftView, EditorApp, Emit, HistoryView, Media, MediaClass, MediaForm, MediaPayload, MediaType, NoConfig,
    NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, PluginApp, PluginCloseStep, UiAssemblyResult, ViewEmit, ViewModel, NATURAL_FILE_PORT, REVERT_TO_COMMAND_ACTION_ID,
};
use protocol::MutationDiff;
use semio_framework::{action_bus, ActionKind, Dialect, Fault, FaultOrigin, IconName, StandardId, SubsetId, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolOperationSpec};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

const SURFACE_TESTKIT_DIALECT: Dialect = Dialect { artifact_kind: "testkit.surface", standard: StandardId("1"), subset: SubsetId::ANY };

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_os_kernel::DslArtifact)]
#[artifact(extension = "testkit-surface")]
pub(crate) struct SurfaceSnapshot {
    count: i32,
}

impl store::ArtifactSqliteSnapshot for SurfaceSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteSnapshotPhase, SqliteValue};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("surface_state")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::from(self.count))] });
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
        Ok(database)
    }
    fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
        let rows = &database.table("surface_state")?.rows;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "surface_state requires one state row"));
        }
        let snapshot = Self { count: i32::try_from(rows[0].integer(1)?).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))? };
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(snapshot)
    }
}

impl semio_framework_schema_composition::ArtifactCompositionFields for SurfaceSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl store::ArtifactDsl for SurfaceSnapshot {
    const EXTENSION: &'static str = "testkit-surface";
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

impl store::ArtifactPack for SurfaceSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct SurfaceDiff {
    count: Option<i32>,
}

impl MutationDiff<SurfaceSnapshot> for SurfaceDiff {
    fn apply(&self, snapshot: &SurfaceSnapshot) -> protocol::MutationApplyResult<SurfaceSnapshot> {
        Ok(SurfaceSnapshot { count: self.count.unwrap_or(snapshot.count) })
    }
    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_dsl_record_derive::DslEnum)]
enum SurfaceEditorCommand {
    #[dsl(key = "increment")]
    Increment,
}

impl ::protocol::OpText for SurfaceEditorCommand {
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

impl ::protocol::OpBinary for SurfaceEditorCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &[SURFACE_TOOL_ID];

    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        ::dsl::variants_binary::decode_op(bytes)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum SurfaceViewerCommand {
    #[default]
    Noop,
}

impl ::protocol::OpBinary for SurfaceViewerCommand {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        Ok(SurfaceViewerCommand::Noop)
    }
}

//#region 🧪️SurfaceRegisteredFactory
/// 🪟️ The canonical surface id `EditorApp<SurfaceEditorFixture>` derives at runtime
/// (`surface_app_id(SURFACE_TESTKIT_DIALECT, Editor)`), never `EditorApp::APP_ID`'s `"surface"`
/// placeholder. It is the controller every tool key, manifest and proof row is stated against, and
/// `surface_editor_controller_is_the_derived_id` joins this literal to the derived value.
const SURFACE_CONTROLLER_ID: &str = "testkit.surface@1/*#editor";
const SURFACE_TOOL_ID: &str = "increment";
const SURFACE_PAYLOAD_SCHEMA: &str = "semio.testkit-surface.command.v1";
const SURFACE_TOOL_CONTRACT: ToolExecutionContract = ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1);

/// 🛠️ The fixture's real owned reducer body. It is a genuine [`semio_framework_job::InteractiveJob`]:
/// it owns its typed command, its retained wire pages and its completion, yields once per admitted
/// page, answers cancellation before anything else, and hands its one exact completion to the
/// publication ladder — the same shape a product editor's job has. A generic bounded proof is not a
/// substitute: it resolves to `interactive-job.missing-owned-reducer` at dispatch, so an editor
/// surface with no owned factory can only ever prove what the runtime FORBIDS.
struct SurfaceFixtureJob {
    command: Option<Box<SurfaceEditorCommand>>,
    completion: Option<ArtifactToolCompletion<EditorApp<SurfaceEditorFixture>>>,
    count: i32,
    raw: Option<action_bus::RetainedToolWireInput>,
    page: usize,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for SurfaceFixtureJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if cx.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        if self.raw.as_ref().is_some_and(|raw| self.page < raw.page_count()) {
            self.page += 1;
            return semio_framework_job::StepOutcome::Yield;
        }
        let Some(SurfaceEditorCommand::Increment) = self.command.as_deref() else {
            return semio_framework_job::StepOutcome::Cancelled;
        };
        self.completion
            .as_ref()
            .expect("surface fixture completion")
            .complete(Ok(Emit { artifact_mutations: vec![SetSurfaceCount { value: self.count + 1 }.into()], ..Default::default() }), crate::app::EphemeralEmit::default())
            .expect("one exact surface completion");
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
            if raw.terminal_is_empty() {
                self.raw = None;
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            return raw.close_step(1, maximum_bytes);
        }
        if self.command.take().is_some() || self.completion.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.raw.is_none() && self.command.is_none() && self.completion.is_none()
    }
}

struct SurfaceFixtureFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ToolJobFactory for SurfaceFixtureFactory {
    type Payload = SurfaceFixtureJob;
    type Job = SurfaceFixtureJob;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        SURFACE_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        SURFACE_TOOL_CONTRACT
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
            return Err((semio_framework::ToolJobFactoryError::new("surface fixture resume starts a fresh command owner"), input, checkpoint));
        }
        payload.raw = Some(input);
        Ok(payload)
    }
}

impl ArtifactOwnedToolJobFactory for SurfaceFixtureFactory {
    type Owner = EditorApp<SurfaceEditorFixture>;
    const TOOL_IDS: &'static [&'static str] = &[SURFACE_TOOL_ID];
    const DOCUMENT_SCHEMA: &'static str = <SurfaceEditorFixture as ArtifactEditor>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: SURFACE_TOOL_ID, lanes: &[ArtifactToolPublicationLane::Artifact] }];
    fn latest_wins_target(_command: &SurfaceEditorCommand) -> Option<&str> {
        None
    }
    fn build_latest_wins_command_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<SurfaceEditorCommand>>> {
        None
    }
}

async fn surface_manifest() -> crate::app::App {
    crate::app::App::from_builder(
        crate::app::App::builder(SURFACE_CONTROLLER_ID, LocalizedLabel::data("Surface Fixture"))
            .await
            .document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "surface.main", semio_framework_ui_contract::SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .app_command(SURFACE_TOOL_ID, LocalizedLabel::data("Increment"), "fixture", ActionKind::Mutation)
            .await
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
            .await,
    )
    .await
}
//#endregion 🧪️SurfaceRegisteredFactory

#[derive(Default)]
struct SurfaceEditorFixture;

struct SurfaceNaturalDecodeCursor {
    bytes: Option<Vec<u8>>,
    retirement: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    offset: usize,
    decoded_items: usize,
    word: [u8; 4],
    sum: i64,
    complete: bool,
    closing: bool,
}

impl SurfaceNaturalDecodeCursor {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes: Some(bytes), retirement: None, offset: 0, decoded_items: 0, word: [0; 4], sum: 0, complete: false, closing: false }
    }

    fn progress(&self) -> crate::app::NaturalFileDecodeProgress {
        crate::app::NaturalFileDecodeProgress { consumed_input_bytes: self.offset, decoded_items: self.decoded_items, owned_output_bytes: 0, completed_work_units: self.offset as u64 }
    }
}

impl crate::app::NaturalFileDecodeCursor<SurfaceSnapshot> for SurfaceNaturalDecodeCursor {
    fn advance(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<crate::app::NaturalFileDecodeStep<SurfaceSnapshot>, crate::app::MediaError> {
        if self.closing || self.complete {
            return Err(crate::app::MediaError::Payload(crate::app::NATURAL_FILE_PORT.into(), "surface decoder is no longer advanceable".into()));
        }
        let bytes = self.bytes.as_ref().expect("surface decoder retains its bytes until close");
        while self.offset < bytes.len() && !context.should_yield() && !context.is_cancelled() {
            self.word[self.offset % 4] = bytes[self.offset];
            self.offset += 1;
            context.consume_fuel(1);
            if self.offset % 4 == 0 {
                self.sum = self.sum.checked_add(i64::from(i32::from_be_bytes(self.word))).ok_or_else(|| crate::app::MediaError::Payload(crate::app::NATURAL_FILE_PORT.into(), "surface natural count overflow".into()))?;
                self.decoded_items += 1;
            }
        }
        if self.offset < bytes.len() {
            return Ok(crate::app::NaturalFileDecodeStep::Progress(self.progress()));
        }
        let count = i32::try_from(self.sum).map_err(|_| crate::app::MediaError::Payload(crate::app::NATURAL_FILE_PORT.into(), "surface natural count is outside signed 32-bit range".into()))?;
        self.complete = true;
        Ok(crate::app::NaturalFileDecodeStep::Complete { snapshot: SurfaceSnapshot { count }, progress: self.progress() })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(maximum_items, maximum_bytes).map_err(|error| Fault::from(error.message))?;
            if retirement.terminal_is_empty() {
                self.retirement = None;
            }
            return Ok(match step {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
                store::SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "surface natural input retirement is blocked" },
                store::SnapshotRetirementStep::Complete => PluginCloseStep::Pending { released_items: 1, released_bytes: 0 },
            });
        }
        if let Some(bytes) = self.bytes.take() {
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(bytes));
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.bytes.is_none() && self.retirement.is_none()
    }
}

impl ArtifactEditor for SurfaceEditorFixture {
    const DIALECT: Dialect = SURFACE_TESTKIT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-surface/v1";
    type Snapshot = SurfaceSnapshot;
    type Mutation = SurfaceMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::app::NoTransient;
    type TransientMutation = crate::app::NoTransientMutation;
    type Command = SurfaceEditorCommand;

    fn natural_file_codec() -> Option<crate::app::NaturalFileCodec> {
        Some(crate::app::NaturalFileCodec { format_kind: "semio.testkit-surface-natural/v1", extension: ".surface", media_type: "application/vnd.semio.testkit-surface", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, crate::app::MediaError> {
        Ok(snapshot.count.to_be_bytes().to_vec())
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, crate::app::MediaError> {
        let bytes: [u8; 4] = bytes.try_into().map_err(|_| crate::app::MediaError::Payload(crate::app::NATURAL_FILE_PORT.into(), "surface natural file requires one signed 32-bit count".into()))?;
        Ok(SurfaceSnapshot { count: i32::from_be_bytes(bytes) })
    }

    fn build_natural_file_decode_owner(bytes: Vec<u8>, _contract: ToolExecutionContract) -> Result<crate::app::NaturalFileDecodeOwner<Self::Snapshot>, crate::app::MediaError> {
        if !bytes.is_empty() && bytes.len() % 4 == 0 {
            Ok(crate::app::NaturalFileDecodeOwner::Controlled(Box::new(SurfaceNaturalDecodeCursor::new(bytes))))
        } else {
            Ok(crate::app::NaturalFileDecodeOwner::Synchronous(bytes))
        }
    }

    fn whole_document_operation(snapshot: Self::Snapshot) -> Option<Self::Mutation> {
        Some(SetSurfaceCount { value: snapshot.count }.into())
    }

    fn build_instance_operation_owner() -> Box<dyn crate::app::ArtifactInstanceOperationOwner> {
        media_owner_context::owner("editor-instance")
    }

    fn export_media_with_request_context(
        owner: &crate::app::ArtifactInstanceOperationOwnerHandle,
        port: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _transient: &crate::app::TransientView<'_, Self::Transient>,
    ) -> Result<Media, crate::app::MediaError> {
        if port == "retained:out" {
            media_owner_context::export(owner, port, doc)
        } else {
            Self::export_media(port, doc)
        }
    }

    crate::bounded_first_step_tool_proofs! {
        owner: EditorApp<SurfaceEditorFixture>, owner_file: "plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs", controller: "testkit.surface@1/*#editor", artifact_schema: "semio.testkit-surface/v1",
        factory: "SurfaceFixtureFactory", factory_type: SurfaceFixtureFactory,
        contract: SURFACE_TOOL_CONTRACT, tools: ["increment"]
    }

    fn command_id(command: &Self::Command) -> &'static str {
        match command {
            SurfaceEditorCommand::Increment => SURFACE_TOOL_ID,
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(SurfaceFixtureFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), SURFACE_TOOL_ID)] })
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        let job = SurfaceFixtureJob { command: Some(request.command), completion: Some(request.completion), count: request.snapshot.count, raw: None, page: 0, closing: false };
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(crate::app::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("testkit-surface-artifact-retained", 4_096))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::app::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(crate::app::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }
    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(crate::app::bounded_document_store_owners::<Self::Draft, Self::DraftMutation>())
    }
    fn build_document_store_disposer() -> crate::app::ArtifactDisposal<store::ArtifactStore<Self::Snapshot, Self::Mutation>> {
        Some(crate::app::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }
    fn build_config_store_disposer() -> crate::app::ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(crate::app::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }
    fn build_draft_store_disposer() -> crate::app::ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        Some(crate::app::bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>())
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app::mutation_fixture::no_state::presence_local_root_retirement_factory())
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app::mutation_fixture::no_state::presence_peer_retirement_factory())
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(crate::app::mutation_fixture::no_state::transient_local_root_retirement_factory())
    }
    fn build_presence_store_disposer() -> crate::app::ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        Some(crate::app::mutation_fixture::no_state::presence_store_disposer())
    }
    fn build_transient_store_disposer() -> crate::app::ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(crate::app::mutation_fixture::no_state::transient_store_disposer())
    }
    fn initial_snapshot() -> SurfaceSnapshot {
        SurfaceSnapshot::default()
    }

    fn handle(
        command: &SurfaceEditorCommand,
        doc: &ArtifactView<'_, SurfaceSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &crate::app::InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SurfaceMutation>, Fault> {
        match command {
            SurfaceEditorCommand::Increment => Ok(Emit { artifact_mutations: vec![SetSurfaceCount { value: doc.snapshot.count + 1 }.into()], ..Default::default() }),
        }
    }

    fn render(_body_key: &str, doc: &ArtifactView<'_, SurfaceSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}", doc.snapshot.count)))
    }
}

#[derive(Default)]
struct SurfaceViewerFixture;

impl ArtifactViewer for SurfaceViewerFixture {
    const DIALECT: Dialect = SURFACE_TESTKIT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-surface/v1";
    type Snapshot = SurfaceSnapshot;
    type Mutation = SurfaceMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::app::NoTransient;
    type TransientMutation = crate::app::NoTransientMutation;
    type Command = SurfaceViewerCommand;

    fn build_instance_operation_owner() -> Box<dyn crate::app::ArtifactInstanceOperationOwner> {
        media_owner_context::owner("viewer-instance")
    }

    fn export_media_with_request_context(
        owner: &crate::app::ArtifactInstanceOperationOwnerHandle,
        port: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _transient: &crate::app::TransientView<'_, Self::Transient>,
    ) -> Result<Media, crate::app::MediaError> {
        if port == "retained:out" {
            media_owner_context::export(owner, port, doc)
        } else {
            Self::export_media(port, doc)
        }
    }

    fn mounted_job_maintenance_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if instance_id == u32::MAX {
            return Ok(PluginCloseStep::Pending { released_items: maximum_items.min(1), released_bytes: maximum_bytes.min(7) });
        }
        Ok(PluginCloseStep::Complete)
    }

    fn mounted_job_close_step(instance_id: u32, _maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if instance_id == u32::MAX {
            return Ok(PluginCloseStep::Blocked { reason: "surface viewer lifecycle witness" });
        }
        Ok(PluginCloseStep::Complete)
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        instance_id != u32::MAX
    }

    fn config_schema() -> &'static str {
        "semio.testkit-surface.viewer-config/v1"
    }

    fn command_id(_command: &Self::Command) -> &'static str {
        "surface-viewer-noop"
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::app::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(crate::app::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }
    fn build_document_store_disposer() -> crate::app::ArtifactDisposal<store::ArtifactStore<Self::Snapshot, Self::Mutation>> {
        Some(crate::app::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }
    fn build_config_store_disposer() -> crate::app::ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(crate::app::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app::mutation_fixture::no_state::presence_local_root_retirement_factory())
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(crate::app::mutation_fixture::no_state::presence_peer_retirement_factory())
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(crate::app::mutation_fixture::no_state::transient_local_root_retirement_factory())
    }
    fn build_presence_store_disposer() -> crate::app::ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        Some(crate::app::mutation_fixture::no_state::presence_store_disposer())
    }
    fn build_transient_store_disposer() -> crate::app::ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(crate::app::mutation_fixture::no_state::transient_store_disposer())
    }
    fn initial_snapshot() -> SurfaceSnapshot {
        SurfaceSnapshot::default()
    }

    fn handle(
        _command: &SurfaceViewerCommand,
        _doc: &ArtifactView<'_, SurfaceSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &crate::app::InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<NoConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(_body_key: &str, doc: &ArtifactView<'_, SurfaceSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}", doc.snapshot.count)))
    }
}

#[semio_framework_async_macros::async_test]
async fn viewer_never_mutates_the_document_or_draft_store() {
    assert_viewer_never_mutates::<SurfaceViewerFixture>().await;
}

#[semio_framework_async_macros::async_test]
async fn bounded_viewer_fixture_preserves_declared_lifecycle_hooks() {
    type Fixture = crate::app::artifact_app_laws::BoundedViewerFixture<SurfaceViewerFixture>;
    assert!(matches!(<Fixture as ArtifactViewer>::mounted_job_maintenance_step(u32::MAX, 3, 11).expect("maintenance hook"), PluginCloseStep::Pending { released_items: 1, released_bytes: 7 }));
    assert!(matches!(<Fixture as ArtifactViewer>::mounted_job_close_step(u32::MAX, 3, 11).expect("close hook"), PluginCloseStep::Blocked { reason: "surface viewer lifecycle witness" }));
    assert!(!<Fixture as ArtifactViewer>::mounted_jobs_terminal_is_empty(u32::MAX));
    assert_eq!(<Fixture as ArtifactViewer>::config_schema(), "semio.testkit-surface.viewer-config/v1");
    assert_eq!(<Fixture as ArtifactViewer>::command_id(&SurfaceViewerCommand::Noop), "surface-viewer-noop");
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    assert_editor_and_viewer_share_dialect::<SurfaceEditorFixture, SurfaceViewerFixture>().await;
}

#[semio_framework_async_macros::async_test]
async fn new_viewer_constructs_a_registry_less_wrapper() {
    let mut app = new_viewer::<SurfaceViewerFixture>().await;
    assert_eq!(app.snapshot().unwrap().count, 0);
    close_registered_fixture_app(&mut app);
}

/// ✅️ The editor surface mutates through the SAME route a product editor uses: a declared
/// `Migrated` `app_command`, an exact app-owned `SurfaceFixtureFactory` keyed on the derived
/// controller id, and a real `InteractiveJob` that hands back one exact completion. Before the
/// factory existed this law dispatched the generic `"typed-command"` verb into a registry-less
/// wrapper and died on `interactive-job.unknown-key` — it proved what the runtime forbids, not that
/// an editor mutates.
///
/// 🔁️ A migrated verb's dispatch only ADMITS: the document advances when the worker's emit walks
/// the bounded publication ladder, which is the product's own route and what the host drives every
/// turn. Asserting the count before settling would assert that a migrated editor does NOT mutate.
#[semio_framework_async_macros::async_test]
async fn editor_fixture_still_mutates_normally() {
    let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest()).await;
    app.dispatch_typed(SurfaceEditorCommand::Increment, &meta("local")).await.expect("increment");
    let receipt = crate::app::artifact_app_laws::settle_registered_typed_operation(&mut app, meta("local").instance_id).await.expect("the admitted operation settles");
    assert!(receipt.lanes.contains(&crate::app::TypedOperationResultLane::Artifact), "the increment settles on the Artifact publication lane it declares, got {:?}", receipt.lanes);
    assert_eq!(app.snapshot().unwrap().count, 1);
    close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn registered_editor_consumes_intrinsic_natural_bytes_through_the_real_media_route() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).expect("natural-file lifecycle fixture");
    let row = &fixture["registeredEditor"];
    let format_kind = row["formatKind"].as_str().expect("registered editor format kind");
    let data = row["octets"].as_array().expect("registered editor octets").iter().map(|octet| octet.as_u64().expect("octet") as u8).collect::<Vec<_>>();
    let expected = i32::try_from(row["expectedCount"].as_i64().expect("expected surface count")).expect("i32 surface count");
    let artifact = crate::app::MediaArtifact {
        descriptor: crate::app::MediaArtifactDescriptor {
            edge_id: None,
            port_id: Some(crate::app::NATURAL_FILE_PORT.into()),
            kind_id: Some(format_kind.into()),
            media_type: None,
            wire: crate::app::MediaWireFormat::Binary { format_kind: format_kind.into() },
            blob_hash: None,
        },
        data,
    };
    let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest()).await;
    app.consume_media(crate::app::NATURAL_FILE_PORT, artifact).await.expect("registered editor consumes natural bytes");
    assert_eq!(app.snapshot().expect("imported surface snapshot").count, expected);
    let history_rows = app.history_snapshot().await.expect("natural-file import history").upserts.into_iter().filter(|entry| entry.edit_id.is_some()).count();
    let expected_history_rows = usize::try_from(fixture["lifecycle"]["expected"]["openedHistoryEntries"].as_u64().expect("opened history entries")).expect("usize history count");
    assert_eq!(history_rows, expected_history_rows, "natural-file Open publishes one event into the fresh owner history");
    close_registered_fixture_app(&mut app);
}

fn surface_natural_file_reserved_job(
    data: Vec<u8>,
    maximum_work_units_per_step: u64,
) -> (
    ArtifactReservedToolJob,
    ArtifactToolCompletion<EditorApp<SurfaceEditorFixture>>,
    std::sync::Arc<SurfaceSnapshot>,
    std::sync::Arc<HistoryView>,
) {
    let snapshot = std::sync::Arc::new(SurfaceSnapshot::default());
    let history = std::sync::Arc::new(HistoryView::empty());
    let completion = ArtifactToolCompletion::<EditorApp<SurfaceEditorFixture>>::new();
    let request = ArtifactReservedToolJobRequest {
        controller_id: SURFACE_CONTROLLER_ID.into(),
        tool_id: "framework.reserved.import-media".into(),
        payload_schema_id: "semio.testkit-surface-natural/v1".into(),
        contract: ToolExecutionContract::resumable(8_192, 8_192, maximum_work_units_per_step, 8_192, 7_500, 1, 1),
        app_instance_id: 7,
        ui_axes: None,
        parent_document_id: "surface-natural-file-fixture".into(),
        canonical_base_revision: [0; 32],
        snapshot: snapshot.clone(),
        config: std::sync::Arc::new(NoConfig::default()),
        history: history.clone(),
        children: ChildContentView::EMPTY,
        raw_wire: Vec::new(),
        input: ArtifactReservedToolInput::Media {
            port: NATURAL_FILE_PORT.into(),
            media: Media {
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                payload: MediaPayload::Intrinsic {
                    schema: "semio.testkit-surface-natural/v1".into(),
                    value: semio_framework_value::DslValue::Bytes(data),
                },
            },
        },
        completion: completion.clone(),
    };
    let job = <SurfaceEditorFixture as ArtifactEditor>::build_reserved_tool_job(request).expect("natural-file reserved builder").expect("mounted natural-file codec owns its reserved route");
    (job, completion, snapshot, history)
}

fn surface_natural_file_step(
    job: &mut ArtifactReservedToolJob,
    operation: semio_framework_job::OperationId,
    cancel: semio_framework_job::CancelToken,
    work_units: u64,
    sequence: &mut u64,
) -> semio_framework_job::StepOutcome {
    let mut context = semio_framework_job::StepContext::new(
        operation,
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(work_units, u64::MAX),
        cancel,
        semio_framework_job::default_now_us,
        sequence,
    );
    semio_framework_job::InteractiveJob::step(job, &mut context)
}

fn close_surface_natural_file_payload(payload: &mut semio_framework_job::RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

fn close_surface_natural_file_job(job: &mut ArtifactReservedToolJob) {
    semio_framework_job::InteractiveJob::begin_close(job);
    assert_eq!(
        semio_framework_job::InteractiveJob::close_step(job, 0, 0),
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }
    );
    for _ in 0..16_384 {
        if semio_framework_job::InteractiveJob::terminal_is_empty(job) {
            break;
        }
        match semio_framework_job::InteractiveJob::close_step(job, 1, 64) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items > 0 || released_bytes > 0, "positive natural-file close grants must make observable retirement progress");
                assert!(released_items <= 1);
                assert!(released_bytes <= 64);
            }
            semio_framework_job::InteractiveJobCloseStep::Complete => {}
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("natural-file job close retained an unmounted owner"),
        }
    }
    assert!(semio_framework_job::InteractiveJob::terminal_is_empty(job), "natural-file job must reach its exact terminal-empty witness");
}

/// 🧵️ A mounted natural-file importer accounts owned input bytes across turns, observes
/// cancellation before decode, and refuses a monolithic decoder whose input exceeds its exact
/// per-turn grant. Both exits retire every retained owner with bounded positive progress.
#[test]
fn natural_file_reserved_job_accounts_cancels_refuses_and_retires_boundedly() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).expect("natural-file lifecycle fixture");
    let row = &fixture["registeredEditor"];
    let data = row["octets"].as_array().expect("registered editor octets").iter().map(|octet| octet.as_u64().expect("octet") as u8).collect::<Vec<_>>();
    let turn = row["accountBytesPerTurn"].as_u64().expect("account turn bytes");
    let cancel_after = row["cancelAfterAccountedBytes"].as_u64().expect("cancelled accounted bytes");
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let (mut cancelled, cancelled_completion, cancelled_snapshot, cancelled_history) = surface_natural_file_reserved_job(data, 4_096);
    let mut sequence = 0;
    match surface_natural_file_step(&mut cancelled, operation, cancel.clone(), turn, &mut sequence) {
        semio_framework_job::StepOutcome::CheckpointReady(mut checkpoint) => {
            assert_eq!(checkpoint.applied_progress, cancel_after);
            close_surface_natural_file_payload(&mut checkpoint.state);
        }
        other => panic!("first bounded natural-file turn must checkpoint accounted bytes, got {other:?}"),
    }
    cancel.cancel_now();
    assert!(matches!(
        surface_natural_file_step(&mut cancelled, operation, cancel, turn, &mut sequence),
        semio_framework_job::StepOutcome::Cancelled
    ));
    drop((cancelled_completion, cancelled_snapshot, cancelled_history));
    close_surface_natural_file_job(&mut cancelled);

    let oversized = usize::try_from(row["oversizedOctetCount"].as_u64().expect("oversized octet count")).expect("usize octet count");
    let maximum = u64::try_from(oversized - 1).expect("bounded maximum work");
    let (mut refused, refused_completion, refused_snapshot, refused_history) = surface_natural_file_reserved_job(vec![0; oversized], maximum);
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    for expected in [maximum, u64::try_from(oversized).expect("oversized progress")] {
        match surface_natural_file_step(&mut refused, operation, cancel.clone(), maximum, &mut sequence) {
            semio_framework_job::StepOutcome::CheckpointReady(mut checkpoint) => {
                assert_eq!(checkpoint.applied_progress, expected);
                close_surface_natural_file_payload(&mut checkpoint.state);
            }
            other => panic!("natural-file accounting must checkpoint monotonically, got {other:?}"),
        }
    }
    match surface_natural_file_step(&mut refused, operation, cancel, maximum, &mut sequence) {
        semio_framework_job::StepOutcome::Fault(mut fault) => {
            let detail = String::from_utf8(fault.detail.single_page().expect("single refusal page").to_vec()).expect("UTF-8 refusal");
            assert_eq!(detail, "natural-file codec requires a controlled decoder for this input size");
            close_surface_natural_file_payload(&mut fault.detail);
        }
        other => panic!("an oversized monolithic natural-file decoder must refuse deterministically, got {other:?}"),
    }
    close_surface_natural_file_job(&mut refused);
    assert!(refused_completion.take_emit().expect("refused completion cell").is_none());
    drop((refused_snapshot, refused_history));
}

/// 🧭️ A format-owned decoder consumes a source larger than one scheduler grant over
/// monotonic checkpoints, publishes the independently computed value once, then retires its sole
/// source owner under the ordinary reserved-job close contract.
#[test]
fn natural_file_controlled_decoder_crosses_turns_and_retires_its_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).expect("natural-file lifecycle fixture");
    let row = &fixture["controlledEditor"];
    let word_count = usize::try_from(row["wordCount"].as_u64().expect("controlled word count")).expect("usize word count");
    let last_word = i32::try_from(row["lastWord"].as_i64().expect("controlled last word")).expect("i32 last word");
    let work = row["maximumWorkUnitsPerStep"].as_u64().expect("controlled work grant");
    let expected_minimum_turns = usize::try_from(row["expectedMinimumTurns"].as_u64().expect("minimum turns")).expect("usize turns");
    let expected_decoded_items = usize::try_from(row["expectedDecodedItems"].as_u64().expect("decoded items")).expect("usize decoded items");
    assert_eq!(word_count, expected_decoded_items);
    let mut data = Vec::with_capacity(word_count * 4);
    for word in std::iter::repeat_n(0_i32, word_count - 1).chain(std::iter::once(last_word)) {
        data.extend_from_slice(&word.to_be_bytes());
    }
    let oracle = data.chunks_exact(4).map(|word| i32::from_be_bytes(word.try_into().expect("four-byte word"))).try_fold(0_i32, i32::checked_add).expect("independent checked sum");
    assert_eq!(oracle, last_word);

    let (mut job, completion, snapshot, history) = surface_natural_file_reserved_job(data, work);
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    let mut checkpoints = 0;
    let mut last_progress = 0;
    loop {
        match surface_natural_file_step(&mut job, operation, cancel.clone(), work, &mut sequence) {
            semio_framework_job::StepOutcome::CheckpointReady(mut checkpoint) => {
                assert!(checkpoint.applied_progress >= last_progress, "controlled decoder checkpoints are monotonic");
                last_progress = checkpoint.applied_progress;
                checkpoints += 1;
                close_surface_natural_file_payload(&mut checkpoint.state);
            }
            semio_framework_job::StepOutcome::Complete(mut candidate) => {
                close_surface_natural_file_payload(&mut candidate.state);
                close_surface_natural_file_payload(&mut candidate.output);
                break;
            }
            semio_framework_job::StepOutcome::Yield => {}
            semio_framework_job::StepOutcome::Cancelled => panic!("controlled natural-file decode cancelled without a request"),
            semio_framework_job::StepOutcome::Fault(mut fault) => {
                let detail = fault.detail.single_page().map(|bytes| String::from_utf8_lossy(bytes).into_owned()).unwrap_or_default();
                close_surface_natural_file_payload(&mut fault.detail);
                panic!("controlled natural-file decode faulted: {detail}");
            }
            other => panic!("unexpected controlled natural-file outcome: {other:?}"),
        }
    }
    assert!(checkpoints >= expected_minimum_turns, "fixture must cross many scheduler turns");
    let (emit, _) = completion.take_emit().expect("controlled completion cell").expect("controlled completion value");
    assert_eq!(emit.expect("controlled import emit").artifact_mutations, vec![SurfaceMutation::from(SetSurfaceCount { value: oracle })]);
    drop((snapshot, history));
    close_surface_natural_file_job(&mut job);
}

/// 🪟️ The controller literal every proof row, tool key and manifest id in this fixture is stated
/// against IS the id `EditorApp` derives at runtime. A drift here would make the proof catalog
/// authoritative for a controller nothing dispatches to, which `validate_tool_job_rows` reports as
/// `interactive-job.catalog-controller` from inside a dispatch instead of here.
#[semio_framework_async_macros::async_test]
async fn surface_editor_controller_is_the_derived_id() {
    assert_eq!(SURFACE_CONTROLLER_ID, semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Editor));
    let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest()).await;
    assert_eq!(app.app_id().await, SURFACE_CONTROLLER_ID);
    close_registered_fixture_app(&mut app);
}

/// 🔒️ The registry-less wrapper still fails CLOSED on the same editor: an owned factory never
/// substitutes for a manifest declaration, so a surface whose verb is undeclared is refused by name
/// before any factory is reached, and the document is untouched.
#[semio_framework_async_macros::async_test]
async fn editor_fixture_without_a_manifest_declaration_fails_closed() {
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>().await;
    let error = app.dispatch_typed(SurfaceEditorCommand::Increment, &meta("local")).await.expect_err("registry-less editor must fail closed");
    assert_eq!(error.code.0, "interactive-job.unknown-key");
    assert!(error.message.contains(SURFACE_TOOL_ID), "the refusal names the editor's own verb, not the generic placeholder: {}", error.message);
    assert_eq!(app.snapshot().unwrap().count, 0, "a fail-closed dispatch never reaches the reducer");
    close_registered_fixture_app(&mut app);
}

/// 🐛️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-G — WITH
/// TEETH: `ShellHost::encodeWindowActionInvocation` addresses every real click with the
/// surface's canonical id (`surface_app_id`, e.g. `s.space.home@1/*#editor`), never
/// `EditorApp::APP_ID`'s runtime-const placeholder (`"surface"`). Before the fix,
/// `handle_action_invocation`'s ownership check compared `address.app_id` against that
/// literal placeholder, so even the textbook-correct canonical id was rejected — every
/// button click across every `EditorApp`/`ViewerApp` surface in the product. This fixture's
/// registry is empty (`new_app`, contract-enforcement-less), so once ownership passes the
/// very next check (`registry.has_mode`) must fail instead — proving the rejection was
/// specifically the ownership line, not a coincidence of an otherwise-valid dispatch.
#[semio_framework_async_macros::async_test]
async fn handle_action_invocation_accepts_the_real_canonical_surface_app_id() {
    use semio_framework::manifest::{ActionAddress, ActionInvocation};
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>().await;
    let real_id = semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Editor);
    let invocation = ActionInvocation {
        address: ActionAddress { plugin_id: "test".into(), app_id: real_id.clone(), mode_id: "edit".into(), window_kind_id: "main".into(), window_instance_id: "main-instance".into(), action_id: "increment".into() },
        arguments: Default::default(),
    };
    let error = app.handle_action_invocation(&invocation, Some("edit"), &meta("local")).await.expect_err("registry-less fixture declares no modes");
    assert!(!error.message.contains("does not match"), "the real canonical app id must satisfy the ownership check, got: {}", error.message);
    assert!(error.message.contains("unknown action mode owner"), "expected ownership to pass and the mode lookup to fail instead, got: {}", error.message);
    assert_eq!(app.app_id().await, real_id, "PluginApp::app_id must report the real canonical id, not the APP_ID placeholder");
    close_registered_fixture_app(&mut app);
}

/// 🪪️ Verifies `EditorApp` initializes its document, config, draft, and interaction
/// envelopes with the real canonical surface app id, not the `APP_ID` placeholder.
#[semio_framework_async_macros::async_test]
async fn editor_app_envelopes_carry_the_real_canonical_surface_app_id() {
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>().await;
    let real_id = semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Editor);
    assert_eq!(app.store.envelope().id, real_id);
    assert_eq!(app.config_store.envelope().id, format!("{real_id}-config"));
    assert_eq!(app.draft_store.envelope().id, format!("{real_id}-draft"));
    assert_eq!(app.interaction_store.envelope().id, format!("{real_id}-interaction"));
    close_registered_fixture_app(&mut app);
}

/// 🪪️ Verifies `ViewerApp` initializes its document, config, draft, and interaction
/// envelopes with the real canonical surface app id, not the `APP_ID` placeholder.
#[semio_framework_async_macros::async_test]
async fn viewer_app_envelopes_carry_the_real_canonical_surface_app_id() {
    let mut app = new_viewer::<SurfaceViewerFixture>().await;
    let real_id = semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Viewer);
    assert_eq!(app.store.envelope().id, real_id);
    assert_eq!(app.config_store.envelope().id, format!("{real_id}-config"));
    assert_eq!(app.draft_store.envelope().id, format!("{real_id}-draft"));
    assert_eq!(app.interaction_store.envelope().id, format!("{real_id}-interaction"));
    close_registered_fixture_app(&mut app);
}

/// 📌️ A hub Check In validates the pair it folded through the guest codec's `print-mirror`, and a zero-op `apply-ops`
/// batch passes a pair through: both read a POPULATED pair, whose unadopted envelope must be retired entry by entry —
/// dropping its owners aborted the guest on the history ledger's terminal-empty witness, so every writer and note Check In
/// was refused `codec-refused` (ticket 26/09/23, session 14).
#[semio_framework_async_macros::async_test]
async fn the_codec_table_mirrors_and_passes_through_a_populated_pair_without_aborting() {
    let table = crate::app::artifact_codec_table::<EditorApp<SurfaceEditorFixture>>();
    let genesis = (table.genesis)("artifact-5c0dec0de5c0dec0de5c0dec0de5c0de").await.expect("genesis pair of a server-minted artifact id");
    let op = protocol::OpBinary::encode_op(&SurfaceMutation::from(SetSurfaceCount { value: 7 })).expect("encode set-surface-count");
    let populated = (table.apply_ops)(&genesis.pack, &genesis.spr, &store::os_spr::encode_ops_vec(&[op])).await.expect("one op lands one edit");
    let history = store::os_spr::decode_history(&populated.spr, &store::os_spr::DecodeOptions::default()).await.expect("populated history");
    assert_eq!(history.edits.len(), 1, "the pair carries one edit, so its history ledger is populated");
    let mirror = (table.print_mirror)(&populated.pack, &populated.spr).await.expect("a populated pair mirrors");
    assert!(mirror.ops.contains("set-surface-count"), "the mirror prints the pair's edit: {}", mirror.ops);
    let passed = (table.apply_ops)(&populated.pack, &populated.spr, &store::os_spr::encode_ops_vec(&[])).await.expect("an empty batch passes a populated pair through");
    assert!(!passed.pack.is_empty() && !passed.spr.is_empty(), "an empty batch over a populated pair returns that pair");
}

/// 👁️🔒 Contract §2.3 clause 1/2 — WITH TEETH: dispatches the eight frozen mutating verbs
/// through the full `VcsArtifactApp<ViewerApp<V>>` runtime path (`handle_action` for the
/// seven string actions, `import_media` for the eighth) and asserts every one comes back
/// `Fault { origin: FaultOrigin::Framework, code: FaultCode::new("viewer.read-only"), .. }`.
#[semio_framework_async_macros::async_test]
async fn viewer_rejects_every_contract_mutating_verb() {
    let mut app = new_viewer::<SurfaceViewerFixture>().await;
    for verb in ["undo", "redo", "commitCheckpoint", "createAlternative", REVERT_TO_COMMAND_ACTION_ID, "cut", "paste"] {
        let error = app.handle_action(verb, None, &meta("local")).await.err().unwrap_or_else(|| panic!("'{verb}' must be rejected on a viewer instance"));
        assert_eq!(error.origin, FaultOrigin::Framework, "'{verb}' rejection must carry FaultOrigin::Framework");
        assert_eq!(error.code.0, "viewer.read-only", "'{verb}' rejection must carry the frozen viewer.read-only code");
    }
    let media = Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "semio.testkit-surface/v1".into(), json: "{}".into() } };
    let error = app.import_media("any-port", media, &meta("local")).await.err().expect("'import' must be rejected on a viewer instance");
    assert_eq!(error.origin, FaultOrigin::Framework, "'import' rejection must carry FaultOrigin::Framework");
    assert_eq!(error.code.0, "viewer.read-only", "'import' rejection must carry the frozen viewer.read-only code");
    close_registered_fixture_app(&mut app);
}
