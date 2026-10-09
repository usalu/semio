#[path = "../../🧪️testing/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs"]
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
use {semio_framework::action_bus,semio_framework::ActionKind,semio_framework_artifact_reference::Dialect,semio_framework::Fault,semio_framework::FaultOrigin,semio_framework::IconName,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework::ToolExecutionContract,semio_framework::ToolFactoryKey,semio_framework::ToolJobFactory,semio_framework::ToolOperationSpec};
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

#[test]
fn child_emission_private_input_metadata_and_genesis_retain_exact_request_and_original_batch() {
    use crate::app::{ChildEmit, ChildEmitGenesis, OwnedChildEmit, PrivateChildPublicationInput};
    use semio_framework_artifact_reference::{ArtifactRef, ArtifactDialect};
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    static DECLARATIONS: &[store::MemberOpenDeclaration] = &[
        store::MemberOpenDeclaration { kind: "test", standard: "1", subset: "*", schema: "demo/v1" },
        store::MemberOpenDeclaration { kind: "kind-δ", standard: "1", subset: "*", schema: "demo/v1" },
    ];
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🏪️store/🧩️composition/🚪️open/🌱️genesis/🧫️fixtures/🔣️.json")).unwrap();
    let reference = |value: &serde_json::Value| ArtifactRef { artifact_id: value["artifact_id"].as_str().unwrap().into(), dialect: ArtifactDialect { artifact_kind: value["dialect"]["artifact_kind"].as_str().unwrap().into(), standard: value["dialect"]["standard"].as_str().unwrap().into(), subset: value["dialect"]["subset"].as_str().unwrap().into() } };
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 262144, maximum_release_bytes: 262144, maximum_depth: 64 };
    for row in corpus["cases"].as_array().unwrap() {
        let expected = reference(&row["expected"]);
        let parent = reference(&row["owner"]["parent"]);
        let slot = row["owner"]["slot"].as_str().unwrap();
        let child = "local-private-α";
        let pack_segment = row["initialPackHex"].as_str().unwrap().as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect::<Vec<_>>();
        for stop in [Some(0usize), Some(1), Some(3), Some(17), Some(65), Some(257), None] {
            let pack = pack_segment.repeat(row["packRepeats"].as_u64().unwrap() as usize);
            let pack_pointer = pack.as_ptr();
            let original = vec![17i32, 3, 9];
            let mutation_pointer = original.as_ptr();
            let (batch, _) = store::MemberStoreOwnedBatch::try_new(original, grant).unwrap();
            let metadata = ChildEmit { genesis: Some(ChildEmitGenesis { reference: reference(&row["expected"]), initial_pack: pack }), owner: String::new(), slot: slot.into(), child_id: child.into(), ops: Vec::new(), op_schema: semio_framework::kernel::SchemaId("private.owned".into()), labels: Vec::new() };
            let (mut input, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| PrivateChildPublicationInput::new(OwnedChildEmit::new(metadata, batch)));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            for turn in 0..100000 {
                if stop.is_some_and(|stop| turn >= stop) { break; }
                let capacity = input.next_capacity_byte_demand(&parent.artifact_id, &parent.dialect, None, "actor:private-genesis", None, None).unwrap();
                assert!(capacity <= grant.maximum_capacity_bytes);
                let paused = RetainedCloneGrant { maximum_items: 0, ..grant };
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.advance(&parent.artifact_id, &parent.dialect, None, "actor:private-genesis", None, None, DECLARATIONS, semio_framework_job::OperationId(7311), semio_framework_job::Generation(3), 1_000_000, 1, paused).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                if capacity != 0 {
                    let denied = RetainedCloneGrant { maximum_capacity_bytes: capacity - 1, ..grant };
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.advance(&parent.artifact_id, &parent.dialect, None, "actor:private-genesis", None, None, DECLARATIONS, semio_framework_job::OperationId(7311), semio_framework_job::Generation(3), 1_000_000, 1, denied).unwrap());
                    assert_eq!(step.progress(), Default::default());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                }
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.advance(&parent.artifact_id, &parent.dialect, None, "actor:private-genesis", None, None, DECLARATIONS, semio_framework_job::OperationId(7311), semio_framework_job::Generation(3), 1_000_000, 1, grant).unwrap());
                assert!(step.progress().fits(grant));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                assert_eq!(input.source().unwrap().genesis.as_ref().unwrap().initial_pack.as_ptr(), pack_pointer);
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            if stop.is_none() {
                assert!(input.ready());
                assert!(input.take_ready(RetainedCloneGrant { maximum_items: 0, ..grant }).is_none());
                let (mut parts, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.take_ready(grant).unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert!(input.terminal_is_empty());
                assert_eq!(parts.source.mutations::<i32>().unwrap().as_ptr(), mutation_pointer);
                let identities = parts.metadata.parts().unwrap();
                assert_eq!(identities.key.child_id, child);
                assert_eq!(identities.prepared_identity.reference, expected);
                assert_eq!(identities.registry_owner.child_id, child);
                let request = parts.request.as_ref().unwrap();
                assert_eq!(request.admitted_expected().unwrap(), &expected);
                assert_eq!(request.owner().unwrap().parent, parent);
                assert_eq!(request.owner().unwrap().child_id, child);
                assert_eq!(request.actor().0, "actor:private-genesis");
                for _ in 0..100000 {
                    let request = parts.request.as_mut().unwrap();
                    if request.terminal_is_empty() { break; }
                    let demand = request.next_close_byte_demand().max(1);
                    assert!(demand <= grant.maximum_release_bytes);
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(1, demand).unwrap());
                    let bytes = match step { store::SnapshotRetirementStep::Pending { released_bytes, .. } => released_bytes, _ => 0 };
                    assert_eq!(heap.requested_bytes, 0);
                    assert_eq!(heap.released_bytes, bytes);
                }
                assert!(parts.request.as_ref().unwrap().terminal_is_empty());
                parts.request.take();
                for _ in 0..100000 {
                    if parts.metadata.terminal_is_empty() { break; }
                    let demand = parts.metadata.next_close_byte_demand();
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| parts.metadata.close_granted(RetainedCloneGrant { maximum_release_bytes: demand, ..grant }).unwrap());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                }
                assert!(parts.metadata.terminal_is_empty());
                for _ in 0..100000 {
                    if parts.source.terminal_is_empty() { break; }
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| parts.source.close_granted(grant).unwrap());
                    assert!(step.progress().fits(grant));
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                }
                assert!(parts.source.terminal_is_empty());
            } else {
                for _ in 0..100000 {
                    if input.terminal_is_empty() { break; }
                    let demand = input.next_close_byte_demand().unwrap();
                    assert!(demand <= grant.maximum_release_bytes);
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| input.close_granted(grant).unwrap());
                    assert!(step.progress().fits(grant));
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                }
                assert!(input.terminal_is_empty());
            }
            println!("[DEBUG] private child input row case={} stop={stop:?} retained original pack/batch pointers, exact declared local/target/request owner and all whole physical releases", row["id"]);
        }
    }
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

impl protocol::DiffAlgebra<SurfaceSnapshot> for SurfaceDiff {
    fn inverse(&self, base: &SurfaceSnapshot) -> Self {
        Self { count: self.count.map(|_| base.count) }
    }
    fn is_empty(&self) -> bool {
        self.count.is_none()
    }
}

impl MutationDiff<SurfaceSnapshot> for SurfaceDiff {
    fn apply(&self, snapshot: &SurfaceSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SurfaceSnapshot> {
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

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<crate::app::PluginLifecycleStep, Fault> {
        if self.retirement.is_some() {
            let step=semio_framework_value::close_factory_ticket(&mut self.retirement,grant).map_err(|error|Fault::from(error.into_message()))?;
            return Ok(crate::app::PluginLifecycleStep::Progress(step.progress()));
        }
        if let Some(original)=self.bytes.take(){
            return match semio_framework_value::retirement::admit_owned_retirement(original,grant){
                Ok((owner,progress))=>{self.retirement=Some(owner);Ok(crate::app::PluginLifecycleStep::Progress(progress))},
                Err((error,original))=>{self.bytes=Some(original);Err(Fault::from(error.into_message()))},
            };
        }
        Ok(crate::app::PluginLifecycleStep::Complete(Default::default()))
    }

    fn retirement_demands(&self,body:usize)->Result<crate::app::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(owner)=self.retirement.as_ref(){return semio_framework_value::factory_ticket_demands(owner,body);}
        Ok(crate::app::RetirementDemand{capacity_bytes:if self.bytes.is_some(){semio_framework_value::retirement::owned_retirement_birth_bytes::<Vec<u8>>()}else{0},depth:usize::from(self.bytes.is_some()),..Default::default()})
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.bytes.is_none() && self.retirement.is_none()
    }
}

impl ArtifactEditor for SurfaceEditorFixture {
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { Some(store::MemberStoreOwnedBatch::scaffold_byte_demand::<Self::Mutation>()) }
    fn admit_owned_mutation_batch(values: &mut Option<Vec<Self::Mutation>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { store::MemberStoreOwnedBatch::admit::<Self::Mutation>(values, grant) }
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
        Some(crate::app::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>().with_one_item_preparation(Self::build_artifact_store_one_item_preparation_factory().expect("surface exact typed preparation authority")))
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
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { Some(store::MemberStoreOwnedBatch::scaffold_byte_demand::<Self::Mutation>()) }
    fn admit_owned_mutation_batch(values: &mut Option<Vec<Self::Mutation>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { store::MemberStoreOwnedBatch::admit::<Self::Mutation>(values, grant) }
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
    let mut app = new_viewer::<SurfaceViewerFixture>(protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
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
    let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
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
    let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
    app.consume_media(crate::app::NATURAL_FILE_PORT, artifact).await.expect("registered editor consumes natural bytes");
    let receipt = crate::app::artifact_app_laws::settle_registered_typed_operation(&mut app, meta("local").instance_id).await.expect("natural-file import settles");
    let loads = receipt.effects.into_iter().filter_map(|effect| match effect { semio_framework::kernel::Effect::LoadDocument { pack, spr } => Some(store::ArtifactPackFiles { pack, spr, ops: String::new() }), _ => None }).collect::<Vec<_>>();
    assert_eq!(loads.len(), 1, "natural-file Open publishes exactly one document load, no mutation");
    crate::app::artifact_app_laws::load_document(&mut app, &loads[0]).await.expect("the document load lands");
    assert_eq!(app.snapshot().expect("imported surface snapshot").count, expected);
    let history_rows = app.history_snapshot().await.expect("natural-file import history").upserts.into_iter().filter(|entry| entry.edit_id.is_some()).count();
    let expected_history_rows = usize::try_from(fixture["lifecycle"]["expected"]["openedHistoryEntries"].as_u64().expect("opened history entries")).expect("usize history count");
    assert_eq!(history_rows, expected_history_rows, "natural-file Open is a document load: it writes no history row into the fresh owner history");
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
        authoring_seed: "surface-natural-file-fixture-admission".into(),
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

const SURFACE_JOB_POLICY:semio_framework_job::RetainedCloneGrant=semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
const SURFACE_ONE_UNIT_POLICY:semio_framework_job::RetainedCloneGrant=semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:262144,maximum_release_bytes:1048576,maximum_depth:4096};
fn surface_natural_file_step(job:&mut ArtifactReservedToolJob,operation:semio_framework_job::OperationId,cancel:semio_framework_job::CancelToken,work_units:u64,sequence:&mut u64,actual:&mut semio_framework_job::RetainedCloneProgress)->semio_framework_job::StepOutcome{
    let mut context=semio_framework_job::StepContext::new(operation,semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(work_units,u64::MAX,SURFACE_JOB_POLICY),cancel,semio_framework_job::default_now_us,sequence,actual);
    let outcome=semio_framework_job::InteractiveJob::step(job,&mut context);assert!(context.retained_progress().fits(SURFACE_JOB_POLICY));outcome
}
fn close_surface_natural_file_payload(payload:&mut semio_framework_job::RetainedJobPayload){
    for _ in 0..16384{if payload.terminal_is_empty(){return}let step=payload.close_step(SURFACE_JOB_POLICY).expect("original Surface payload full-grant close");assert!(step.progress().fits(SURFACE_JOB_POLICY));}panic!("original Surface payload did not retire under its fixed caller policy")
}
fn close_surface_natural_file_job(job:&mut ArtifactReservedToolJob){
    use semio_framework_job::{InteractiveJob as Job,InteractiveJobCloseStep as Step,RetainedCloneProgress};
    Job::begin_close(job);let zero=semio_framework_job::RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};assert_eq!(Job::close_step(job,zero).progress(),RetainedCloneProgress::default());
    for _ in 0..16384{if Job::terminal_is_empty(job){break}let demand=Job::next_close_release_byte_demand(job).expect("original Surface release demand");assert_eq!(Job::next_close_release_byte_demand(job).unwrap(),demand);if demand>64{let denied=semio_framework_job::RetainedCloneGrant{maximum_release_bytes:64,..SURFACE_JOB_POLICY};let step=Job::close_step(job,denied);assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(Job::next_close_release_byte_demand(job).unwrap(),demand);assert!(!Job::terminal_is_empty(job));}
        let step=Job::close_step(job,SURFACE_JOB_POLICY);assert!(step.progress().fits(SURFACE_JOB_POLICY));match step{Step::Pending{progress}=>assert!(progress!=RetainedCloneProgress::default(),"original Surface fixed policy made no progress"),Step::Complete{..}=>{},other=>panic!("original Surface fixed policy close retained its frontier: {other:?}")}
    }assert!(Job::terminal_is_empty(job),"original Surface exact terminal witness")
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
    let mut actual=semio_framework_job::RetainedCloneProgress::default();
    match surface_natural_file_step(&mut cancelled, operation, cancel.clone(), turn, &mut sequence,&mut actual) {
        semio_framework_job::StepOutcome::CheckpointReady(mut checkpoint) => {
            assert_eq!(checkpoint.applied_progress, cancel_after);
            close_surface_natural_file_payload(&mut checkpoint.state);
        }
        other => panic!("first bounded natural-file turn must checkpoint accounted bytes, got {other:?}"),
    }
    cancel.cancel_now();
    assert!(actual.fits(SURFACE_JOB_POLICY));actual=Default::default();
    assert!(matches!(
        surface_natural_file_step(&mut cancelled, operation, cancel, turn, &mut sequence,&mut actual),
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
    let mut actual=semio_framework_job::RetainedCloneProgress::default();
    for expected in [maximum, u64::try_from(oversized).expect("oversized progress")] {
        assert!(actual.fits(SURFACE_JOB_POLICY));actual=Default::default();
        match surface_natural_file_step(&mut refused, operation, cancel.clone(), maximum, &mut sequence,&mut actual) {
            semio_framework_job::StepOutcome::CheckpointReady(mut checkpoint) => {
                assert_eq!(checkpoint.applied_progress, expected);
                close_surface_natural_file_payload(&mut checkpoint.state);
            }
            other => panic!("natural-file accounting must checkpoint monotonically, got {other:?}"),
        }
    }
    assert!(actual.fits(SURFACE_JOB_POLICY));actual=Default::default();
    match surface_natural_file_step(&mut refused, operation, cancel, maximum, &mut sequence,&mut actual) {
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
    let mut actual=semio_framework_job::RetainedCloneProgress::default();
    let mut checkpoints = 0;
    let mut last_progress = 0;
    loop {
        assert!(actual.fits(SURFACE_JOB_POLICY));actual=Default::default();
        match surface_natural_file_step(&mut job, operation, cancel.clone(), work, &mut sequence,&mut actual) {
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
    let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
    assert_eq!(app.app_id().await, SURFACE_CONTROLLER_ID);
    close_registered_fixture_app(&mut app);
}

/// 🔒️ The registry-less wrapper still fails CLOSED on the same editor: an owned factory never
/// substitutes for a manifest declaration, so a surface whose verb is undeclared is refused by name
/// before any factory is reached, and the document is untouched.
#[semio_framework_async_macros::async_test]
async fn editor_fixture_without_a_manifest_declaration_fails_closed() {
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
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
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
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
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
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
    let mut app = new_viewer::<SurfaceViewerFixture>(protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
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
    let mut app = new_viewer::<SurfaceViewerFixture>(protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
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

#[test]
fn retained_command_work_receives_exact_one_unit_and_fallback_charges_once() {
    use crate::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
    use semio_framework_job::InteractiveJob;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧵️retained-command/🧫️fixtures/⛽️work-unit-authority.json")).unwrap();
    struct Probe { observed: Option<std::sync::Arc<std::sync::atomic::AtomicU64>>, consumes: u64, closing: bool }
    impl ArtifactCommandWork<EditorApp<SurfaceEditorFixture>> for Probe {
        fn tool_id(&self) -> &'static str { SURFACE_TOOL_ID }
        fn extent(&self, _: &SurfaceEditorCommand, _: &SurfaceSnapshot, _: &protocol::InteractionState, _: Option<&crate::app::ArtifactOwnedToolJobContext<EditorApp<SurfaceEditorFixture>>>) -> Option<usize> { Some(1) }
        fn step(&mut self, _: &ArtifactCommandInputs<'_, EditorApp<SurfaceEditorFixture>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<SurfaceEditorFixture>>, Fault> {
            self.observed.as_ref().unwrap().store(cx.fuel_remaining(), std::sync::atomic::Ordering::SeqCst);
            cx.consume_fuel(self.consumes);
            Ok(ArtifactCommandWorkStep::Complete(Emit::default()))
        }
        fn begin_close(&mut self) { self.closing = true; }
        fn close_step(&mut self,grant:semio_framework_job::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{
            use semio_framework_job::{InteractiveJobCloseStep as Step,RetainedCloneProgress};
            let Some(original)=self.observed.as_ref()else{return Step::Complete{progress:RetainedCloneProgress::default()}};let copy=std::mem::size_of::<Option<std::sync::Arc<std::sync::atomic::AtomicU64>>>();let physical=semio_framework_value::shared_retirement_allocation_bytes::<std::sync::atomic::AtomicU64>();
            if !self.closing||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<physical||grant.maximum_depth==0{return Step::Pending{progress:RetainedCloneProgress::default()}}if std::sync::Arc::weak_count(original)!=0{return Step::Refused{kind:semio_framework_value::ValueRefusalKind::UnsupportedOwner,progress:Default::default()}}
            let unique=std::sync::Arc::into_inner(self.observed.take().unwrap()).is_some();Step::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:if unique{physical}else{0},..Default::default()}}
        }
        fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if self.observed.is_some(){std::mem::size_of::<Option<std::sync::Arc<std::sync::atomic::AtomicU64>>>()}else{0})}
        fn next_close_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
        fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if self.observed.is_some(){semio_framework_value::shared_retirement_allocation_bytes::<std::sync::atomic::AtomicU64>()}else{0})}
        fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(usize::from(self.observed.is_some()))}
        fn terminal_frame_release_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
        fn terminal_is_empty(&self) -> bool { self.closing && self.observed.is_none() }
    }
    for row in fixture["cases"].as_array().unwrap() {
        let observed = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(u64::MAX));
        let work = Probe { observed: Some(observed.clone()), consumes: row["domainConsumes"].as_u64().unwrap(), closing: false };
        let completion = ArtifactToolCompletion::new();
        let consumer = completion.clone();
        let payload = ArtifactRetainedCommandPayload::new(ArtifactRetainedCommandInputs {
            command: SurfaceEditorCommand::Increment,
            snapshot: std::sync::Arc::new(SurfaceSnapshot::default()), config: std::sync::Arc::new(NoConfig::default()), history: std::sync::Arc::new(HistoryView::empty()),
            interaction_state: std::sync::Arc::new(protocol::InteractionState::default()), interaction_hover: std::sync::Arc::new(Default::default()), context: None,
            operation: crate::app::AppOperationContext { app_instance_id: 1, parent_document_id: "unit-authority".into(), operation_id: 1, generation: 1, canonical_base_revision: [0;32], retained:SURFACE_ONE_UNIT_POLICY, authoring_seed: "unit-authority".into() }, completion,
        }, |_| SURFACE_TOOL_ID, 32, 1, Box::new(work));
        let mut job = ArtifactRetainedCommandJob::new(payload);
        let mut sequence = 0;
        for _ in 0..32 {
            let mut actual=semio_framework_job::RetainedCloneProgress::default();
            let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fixture["grantUnits"].as_u64().unwrap(),100_000,SURFACE_ONE_UNIT_POLICY), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence,&mut actual);
            let mut outcome = job.step(&mut cx);
            assert!(!matches!(outcome, semio_framework_job::StepOutcome::Fault(_)));
            assert_eq!(cx.fuel_remaining(), row["remaining"].as_u64().unwrap());
            for _ in 0..32 {
                if outcome.terminal_is_empty() { break; }
                let step=outcome.close_step(SURFACE_ONE_UNIT_POLICY).expect("original one-unit outcome full-grant close");assert!(step.progress().fits(SURFACE_ONE_UNIT_POLICY));if matches!(step,semio_framework_job::RetainedCloneStep::Complete(_)){assert!(outcome.terminal_is_empty());}
            }
            assert!(outcome.terminal_is_empty());
            if observed.load(std::sync::atomic::Ordering::SeqCst) != u64::MAX { break; }
        }
        job.begin_close();
        for _ in 0..100_000 {
            let step=job.close_step(SURFACE_ONE_UNIT_POLICY);assert!(step.progress().fits(SURFACE_ONE_UNIT_POLICY));match step{semio_framework_job::InteractiveJobCloseStep::Complete{..}=>break,semio_framework_job::InteractiveJobCloseStep::Pending{..}=>{},other=>panic!("one-unit original fixed policy retained its frontier: {other:?}")}
        }
        assert!(job.terminal_is_empty());
        assert!(consumer.take_emit().unwrap().is_none());
        assert_eq!(observed.load(std::sync::atomic::Ordering::SeqCst), row["domainObserves"].as_u64().unwrap());
        println!("[DEBUG] Retained command one-unit work authority case={} observed=1 remaining=0", row["id"]);
    }
}

#[test]
fn surface_owned_mutation_admission_forwards_exact_birth_and_preserves_every_denied_original() {
    use crate::app::{ArtifactApp, ViewerApp};
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    fn verify<A: ArtifactApp<Mutation = SurfaceMutation>>(count: usize) {
        let birth = A::owned_mutation_batch_birth_bytes().expect("selected surface declares its exact borrowed admission");
        let mut values = Some((0..count).map(|index| SetSurfaceCount { value: index as i32 }.into()).collect::<Vec<SurfaceMutation>>());
        let pointer = values.as_ref().unwrap().as_ptr();
        let capacity = values.as_ref().unwrap().capacity();
        for (items, bytes) in [(0, birth), (1, birth - 1)] {
            let (result, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| A::admit_owned_mutation_batch(&mut values, RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: 0, maximum_capacity_bytes: bytes, maximum_release_bytes: 0, maximum_depth: 8 }).unwrap());
            assert!(result.is_none());
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            assert_eq!(values.as_ref().unwrap().as_ptr(), pointer);
            assert_eq!((values.as_ref().unwrap().len(), values.as_ref().unwrap().capacity()), (count, capacity));
        }
        let ((mut owner, admitted), events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| A::admit_owned_mutation_batch(&mut values, RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: birth, maximum_release_bytes: 0, maximum_depth: 8 }).unwrap().expect("exact admitted surface authority"));
        assert!(values.is_none());
        assert_eq!(admitted.retained_capacity_bytes, birth);
        assert_eq!((events.requested_bytes, events.released_bytes), (birth, 0));
        assert_eq!(owner.mutations::<SurfaceMutation>().unwrap().as_ptr(), pointer);
        let mut released = 0;
        for _ in 0..count * 8 + 32 {
            let (capacity_bytes, release_bytes) = owner.next_demands().unwrap();
            let copy_bytes = owner.next_copy_byte_demand();
            assert!(copy_bytes <= 64);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy_bytes, maximum_capacity_bytes: capacity_bytes, maximum_release_bytes: release_bytes, maximum_depth: 8 };
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_granted(grant).unwrap());
            let progress = match step { RetainedCloneStep::Progress(progress) => progress, RetainedCloneStep::Complete(progress) => progress };
            assert!(progress.fits(grant));
            assert_eq!(events.requested_bytes, progress.retained_capacity_bytes);
            assert_eq!(events.released_bytes, progress.released_bytes);
            released += progress.released_bytes;
            if owner.terminal_is_empty() { break; }
        }
        assert!(owner.terminal_is_empty());
        println!("[DEBUG] surface borrowed original count={count} birth={birth} retained-pointer=true actual-paid-release={released}");
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🏪️store/🧩️composition/📬️publication/🤝️group/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let count = row["historyEntries"].as_u64().unwrap() as usize;
        verify::<EditorApp<SurfaceEditorFixture>>(count);
        verify::<ViewerApp<SurfaceViewerFixture>>(count);
    }
}

struct SurfaceMutationRetirement { value: Option<SurfaceMutation> }
impl semio_framework_value::retirement::RetirementCursor for SurfaceMutationRetirement {
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep { if grant.maximum_items==0{return semio_framework_value::retirement::RetirementStep::BudgetExhausted;}self.value.take(); semio_framework_value::retirement::RetirementStep::Complete }
    fn terminal_is_empty(&self) -> bool { self.value.is_none() }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn next_birth_bytes(&self, _maximum_bytes: usize) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<Self>()) }
}
impl semio_framework_value::retirement::RetireOwned for SurfaceMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { Box::new(SurfaceMutationRetirement { value: Some(self) }) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<SurfaceMutationRetirement>()) }
    fn controlled_retirement_supported() -> bool { true }
}

#[semio_framework_async_macros::async_test]
async fn child_emission_private_typed_lanes_stage_three_original_batches_and_retire_all_cancelled_sources() {
    use crate::app::{ArtifactApp, PrivateOwnedPublicationLane};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use store::SpaceMember;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧩️composition/📨️emission/🌱️genesis/🧫️fixtures/🔣️.json")).unwrap();
    let mut stops: Vec<Option<usize>> = corpus["cancelBeforeCommit"].as_array().unwrap().iter().map(|stop| Some(stop.as_u64().unwrap() as usize)).collect();
    stops.push(None);
    for stop in stops {
        let mut apps = Vec::new();
        let mut lanes = Vec::new();
        let mut owner = semio_framework_os_kernel::os_vcs::ArtifactGroupVisibilityOwner::new();
        let visibility = owner.view();
        for _ in 0..3 {
            let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
            let (generation, revision) = app.store.one_item_publication_identity();
            let mut original = Some(vec![SurfaceMutation::from(SetSurfaceCount { value: 1 })]);
            let pointer = original.as_ref().unwrap().as_ptr();
            let birth = <EditorApp<SurfaceEditorFixture> as ArtifactApp>::owned_mutation_batch_birth_bytes().unwrap();
            let (mutations, _) = <EditorApp<SurfaceEditorFixture> as ArtifactApp>::admit_owned_mutation_batch(&mut original, RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: birth, maximum_release_bytes: 0, maximum_depth: 64 }).unwrap().unwrap();
            let request = store::MemberStoreOwnedBatchRequest { operation: semio_framework_job::OperationId(7101), expected_generation: generation, expected_revision: revision, actor: crate::app::LOCAL_ACTOR_ID.into(), group_id: Some("private-neutral-three-lane".into()), transaction: None, mutations };
            let mut lane = PrivateOwnedPublicationLane::new(request);
            let admitted = lane.next_capacity_byte_demand(&app.store).unwrap();
            for (items, bytes) in [(0, admitted), (1, admitted - 1)] {
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| lane.advance(&mut app.store, &visibility, store::ArtifactStoreOneItemGrant { maximum_items: items, maximum_bytes: bytes }).unwrap());
                assert_eq!(step, store::ArtifactStoreOneItemPreparationStep::Blocked);
                assert_eq!(events.requested_bytes, 0);
                assert_eq!(events.released_bytes, 0);
                assert_eq!(lane.request().unwrap().mutations.mutations::<SurfaceMutation>().unwrap().as_ptr(), pointer);
            }
            apps.push(app);
            lanes.push(lane);
        }
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 262_144 };
        let mut turns = 0;
        let mut index = 0;
        while index < lanes.len() && stop.is_none_or(|stop| turns < stop) {
            lanes[index].advance(&mut apps[index].store, &visibility, grant).unwrap();
            if lanes[index].prepared_for_staging() {
                let identity = lanes[index].prepared_edit_id().unwrap();
                assert!(!identity.is_empty());
                let pointer = identity.as_ptr();
                let prepared = lanes[index].prepared_publication().unwrap();
                assert_eq!(prepared.prepared_operation_count(false), Some(1));
                assert!(prepared.prepared_operation(false, 0).unwrap().is::<SurfaceMutation>());
                assert!(prepared.prepared_operation(false, 1).is_none());
                let operation_pointer = prepared.prepared_operation(false, 0).unwrap().downcast_ref::<SurfaceMutation>().unwrap() as *const SurfaceMutation;
                let (same_operation, operation_heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| prepared.prepared_operation(false, 0).unwrap().downcast_ref::<SurfaceMutation>().unwrap() as *const SurfaceMutation);
                assert_eq!(same_operation, operation_pointer);
                assert_eq!((operation_heap.requested_bytes, operation_heap.released_bytes), (0, 0));
                let (same_pointer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| lanes[index].prepared_edit_id().unwrap().as_ptr());
                assert_eq!(same_pointer, pointer);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let paused = store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: grant.maximum_bytes };
                assert_eq!(lanes[index].advance(&mut apps[index].store, &visibility, paused).unwrap(), store::ArtifactStoreOneItemPreparationStep::Blocked);
                assert_eq!(lanes[index].prepared_edit_id().unwrap().as_ptr(), pointer);
            }
            assert_eq!(apps.iter().map(|app| app.store.snapshot_ref().count).collect::<Vec<_>>(), vec![0; 3]);
            turns += 1;
            assert!(turns < 4096);
            if lanes[index].staged() { index += 1; }
        }
        if stop.is_none() {
            assert!(lanes.iter().all(PrivateOwnedPublicationLane::staged));
            assert!(lanes.len() <= corpus["source"]["maximumDeclaredChildren"].as_u64().unwrap() as usize + 1);
            let (_, decision_heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                assert!(owner.commit());
                for index in 0..3 {
                    assert!(matches!(lanes[index].adopt(&mut apps[index].store, &visibility, grant).unwrap(), store::ArtifactStoreOneItemPreparationStep::Prepared(_)));
                    assert_eq!(apps[index].store.generation_now(), 1);
                    assert_eq!(apps[index].store.snapshot_ref().count, 1);
                }
            });
            assert_eq!((decision_heap.requested_bytes, decision_heap.released_bytes), (0, 0));
        } else { assert!(owner.abort()); }
        for index in 0..3 {
            for _ in 0..4096 {
                let demand = lanes[index].next_close_byte_demand().unwrap().max(1);
                assert!(demand <= grant.maximum_bytes);
                let step = lanes[index].close_step(&mut apps[index].store, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: demand }).unwrap();
                if step == PluginCloseStep::Complete { break; }
            }
            assert!(lanes[index].terminal_is_empty());
            assert_eq!(apps[index].store.snapshot_ref().count, i32::from(stop.is_none()));
        }
        drop(lanes);
        for mut app in apps { close_registered_fixture_app(&mut app); }
        println!("[DEBUG] private typed publication three original lanes stop={stop:?} retained every denied pointer, staged under one common decision and closed all source/metadata/publication owners");
    }
}

#[semio_framework_async_macros::async_test]
async fn child_emission_private_group_row_frames_preserve_original_sources_on_every_input_cancel() {
    use crate::app::{ChildEmit, ChildEmitGenesis, OwnedChildEmit, PrivateChildGroupSource, PrivateChildPublicationGroup};
    use semio_framework_artifact_reference::{ArtifactRef, ArtifactDialect};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 262_144, maximum_release_bytes: 262_144, maximum_depth: 64 };
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧩️composition/📨️emission/🌱️genesis/🧫️fixtures/🔣️.json")).unwrap();
    for parent_touched in corpus["receipts"]["parentTouched"].as_array().unwrap().iter().map(|value| value.as_bool().unwrap()) {
    for stop in corpus["ownerInput"]["cancelStops"].as_array().unwrap().iter().map(|stop| stop.as_u64().unwrap() as usize) {
        let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
        let parent_values: Vec<i32> = if parent_touched { serde_json::from_value(corpus["ownerInput"]["parentMutations"].clone()).unwrap() } else { Vec::new() };
        let child_values: Vec<i32> = serde_json::from_value(corpus["ownerInput"]["childMutations"].clone()).unwrap();
        let (parent, _) = store::MemberStoreOwnedBatch::try_new(parent_values, grant).unwrap();
        let (child, _) = store::MemberStoreOwnedBatch::try_new(child_values, grant).unwrap();
        let target = &corpus["source"]["reference"];
        let metadata = ChildEmit { genesis: Some(ChildEmitGenesis { reference: ArtifactRef { artifact_id: target["artifactId"].as_str().unwrap().into(), dialect: ArtifactDialect { artifact_kind: target["dialect"]["artifactKind"].as_str().unwrap().into(), standard: target["dialect"]["standard"].as_str().unwrap().into(), subset: target["dialect"]["subset"].as_str().unwrap().into() } }, initial_pack: serde_json::to_vec(&corpus["ownerInput"]["initialPack"]).unwrap() }), owner: String::new(), slot: corpus["source"]["slot"].as_str().unwrap().into(), child_id: corpus["source"]["childId"].as_str().unwrap().into(), ops: Vec::new(), op_schema: semio_framework::kernel::SchemaId("private.owned".into()), labels: Vec::new() };
        let sources = PrivateChildGroupSource { parent, parent_touched, children: vec![OwnedChildEmit::new(metadata, child)], transaction: Some(protocol::TransactionRef { id: "private-neutral-three-lane".into(), tool: "neutral-owner".into() }), group_id: corpus["receipts"]["groupId"].as_str().unwrap().into() };
        let (mut group, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| match PrivateChildPublicationGroup::<store::NoMembers>::try_new(sources) { Ok(group) => group, Err(_) => panic!("one neutral original child is within the fixed bound") });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(group.child_count(), 1);
        assert_eq!(group.parent_touched(), parent_touched);
        assert!(group.prepared_operation_schema_parts(0, 0).is_none());
        assert!(group.prepared_operation_schema_parts(1, 0).is_none());
        assert!(PrivateChildPublicationGroup::<store::NoMembers>::frame_birth_bytes() <= 262_144);
        assert!(PrivateChildPublicationGroup::<store::NoMembers>::row_birth_bytes() <= 262_144);
        let dialect = ArtifactDialect { artifact_kind: "neutral.parent".into(), standard: "1".into(), subset: "*".into() };
        for _ in 0..stop {
            let capacity = group.next_input_capacity_byte_demand(&app.children, "neutral-parent-α", &dialect, "actor:private-genesis").unwrap();
            assert!(capacity <= grant.maximum_capacity_bytes);
            for denied in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), capacity.checked_sub(1).map(|maximum_capacity_bytes| RetainedCloneGrant { maximum_capacity_bytes, ..grant })].into_iter().flatten() {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| group.advance_inputs(&app.children, "neutral-parent-α", &dialect, "actor:private-genesis", semio_framework_job::OperationId(7391), semio_framework_job::Generation(3), 1_000_000, 1, denied).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| group.advance_inputs(&app.children, "neutral-parent-α", &dialect, "actor:private-genesis", semio_framework_job::OperationId(7391), semio_framework_job::Generation(3), 1_000_000, 1, grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        }
        let mut graph = app.composition.graph_mut().await;
        for _ in 0..100_000 {
            if group.terminal_is_empty() { break; }
            let bytes = group.next_close_byte_demand().unwrap().max(1);
            assert!(bytes <= 262_144);
            let exact = RetainedCloneGrant { maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, ..grant };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| group.close_step(&mut app.store, &mut app.children, &mut graph, &app.child_content_root, exact).unwrap());
            assert!(step.progress().fits(exact));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        }
        assert!(group.terminal_is_empty());
        drop(graph); drop(group);
        close_registered_fixture_app(&mut app);
        println!("[DEBUG] private group input cancellation parent_touched={parent_touched} stop={stop} original parent/child sources, row frames and queued backing matched exact same-turn allocator receipts");
    }
    }
}
