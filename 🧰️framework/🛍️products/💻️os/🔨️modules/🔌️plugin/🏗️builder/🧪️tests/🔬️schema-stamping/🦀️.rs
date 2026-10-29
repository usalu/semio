use semio_framework_plugin_decl_registry::ArtifactInferenceServiceMetadata;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_builder::PluginBuilder;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer, Editor, Viewer};
use semio_framework_plugin_decl_emission::surface_contract::ViewEmit;
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, DraftView, InteractionView, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::ViewModel;
use semio_framework::{AppRole, Fault, IconName};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_2d_compute::EngineHandles;
use semio_framework_ui_locale::LocalizedLabel;

const EDITOR_STAMP_DIALECT: Dialect = Dialect { artifact_kind: "builder-test.schema-stamp-editor", standard: StandardId("1"), subset: SubsetId::ANY };
const VIEWER_STAMP_DIALECT: Dialect = Dialect { artifact_kind: "builder-test.schema-stamp-viewer", standard: StandardId("1"), subset: SubsetId::ANY };

#[derive(Default)]
struct SchemaStampEditorFixture;

impl ArtifactEditor for SchemaStampEditorFixture {
    const DIALECT: Dialect = EDITOR_STAMP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "builder-test.schema-stamp-editor.document";
    type Snapshot = NoConfig;
    type Mutation = NoConfigMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = NoConfigMutation;

    fn reference_birth()->Option<fn()->Result<NoConfig,semio_framework_plugin::Fault>>{Some(||{
        Ok(NoConfig::default())
    })}

    fn examples() -> Vec<ExampleSource> {
        vec![ExampleSource::new("stamp-column", LocalizedLabel::native("Stamp Column", "Stempelstütze"), "{\"kind\":\"stamp\"}", "file")]
    }

    fn handle(
        _command: &NoConfigMutation,
        _doc: &ArtifactView<'_, NoConfig>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<NoConfigMutation>, Fault> {
        Ok(Emit::default())
    }

    fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data("schema-stamp-editor"))
    }
}

#[derive(Default)]
struct SchemaStampViewerFixture;

impl ArtifactViewer for SchemaStampViewerFixture {
    const DIALECT: Dialect = VIEWER_STAMP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "builder-test.schema-stamp-viewer.document";
    type Snapshot = NoConfig;
    type Mutation = NoConfigMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = NoConfigMutation;

    fn reference_birth()->Option<fn()->Result<NoConfig,semio_framework_plugin::Fault>>{Some(||{
        Ok(NoConfig::default())
    })}

    fn handle(
        _command: &NoConfigMutation,
        _doc: &ArtifactView<'_, NoConfig>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<NoConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data("schema-stamp-viewer"))
    }
}

use semio_framework::{AppDefinition, *};
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, ToolCommandCatalog};
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_plugin_decl_surface::{EditorApp, ViewerApp};
use semio_framework_plugin_app_builder::ExampleSource;
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_os_kernel::io::SubsetValidator;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_value::ValueError;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_window_config::{WindowConfigOwner, WindowConfigReplaceMutation};
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_plugin_plugin_program::{WireArtifactInferenceMetadata, wire_list_artifact_inference_services};
use semio_framework_plugin_extension::artifact_inference::artifact_inference_service;
use semio_framework_plugin_window_transient::built_text_to_component_tree;
use semio_framework_os_kernel as store;
use semio_framework_ui_contract::*;
use semio_framework::kernel::*;
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed app set for the private schema stamping fixtures.
    enum SchemaStampApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<SchemaStampEditorFixture>>),
        Viewer(VcsArtifactApp<ViewerApp<SchemaStampViewerFixture>>),
    }
}

fn minimal_surface_def(dialect: Dialect, role: AppRole) -> AppDefinition {
    let label = LocalizedLabel::data("Surface");
    match role {
        AppRole::Editor => Editor::builder(dialect).document(["semio", "schema-stamp-editor"]).mode("edit", label.clone(), "pencil").window_kind("main", label, "main", SurfaceKind::Canvas2d, IconName::AppWindow).build_definition(),
        AppRole::Viewer => Viewer::builder(dialect).document(["semio", "schema-stamp-viewer"]).mode("edit", label.clone(), "pencil").window_kind("main", label, "main", SurfaceKind::Canvas2d, IconName::AppWindow).build_definition(),
    }
}

#[semio_framework_async_macros::async_test]
async fn editor_stamps_artifact_schema_from_the_type_when_left_empty() {
    let def = minimal_surface_def(EDITOR_STAMP_DIALECT, AppRole::Editor);
    assert!(def.io.artifact_schema.is_empty(), "fixture precondition: builder leaves io.artifact_schema empty");
    let plugin = PluginBuilder::<_, SchemaStampApps>::new("builder-test-schema-stamp-editor")
        .label("Builder Test Schema Stamp Editor")
        .version("0.1.0")
        .package_id("semio:builder-test-schema-stamp-editor")
        .editor::<SchemaStampEditorFixture>(def)
        .try_build()
        .expect("a minimal editor surface must assemble");
    let app = plugin.manifest.apps.iter().find(|app| app.role == AppRole::Editor).expect("the registered editor app definition");
    assert_eq!(app.io.artifact_schema, SchemaStampEditorFixture::DOCUMENT_SCHEMA);
    let offered = examples_for_app(&plugin.manifest.examples, app);
    assert_eq!(offered.iter().map(|example| example.id.as_str()).collect::<Vec<_>>(), vec!["stamp-column"]);
}

#[semio_framework_async_macros::async_test]
async fn editor_does_not_overwrite_an_explicitly_set_artifact_schema() {
    let mut def = minimal_surface_def(EDITOR_STAMP_DIALECT, AppRole::Editor);
    def.io.artifact_schema = "already-set.document".into();
    let plugin = PluginBuilder::<_, SchemaStampApps>::new("builder-test-schema-stamp-editor-explicit")
        .label("Builder Test Schema Stamp Editor Explicit")
        .version("0.1.0")
        .package_id("semio:builder-test-schema-stamp-editor-explicit")
        .editor::<SchemaStampEditorFixture>(def)
        .try_build()
        .expect("a minimal editor surface must assemble");
    let app = plugin.manifest.apps.iter().find(|app| app.role == AppRole::Editor).expect("the registered editor app definition");
    assert_eq!(app.io.artifact_schema, "already-set.document", "an explicitly set schema must survive untouched");
}

#[semio_framework_async_macros::async_test]
async fn viewer_stamps_artifact_schema_from_the_type_when_left_empty() {
    let def = minimal_surface_def(VIEWER_STAMP_DIALECT, AppRole::Viewer);
    assert!(def.io.artifact_schema.is_empty(), "fixture precondition: builder leaves io.artifact_schema empty");
    let plugin = PluginBuilder::<_, SchemaStampApps>::new("builder-test-schema-stamp-viewer")
        .label("Builder Test Schema Stamp Viewer")
        .version("0.1.0")
        .package_id("semio:builder-test-schema-stamp-viewer")
        .viewer::<SchemaStampViewerFixture>(def)
        .try_build()
        .expect("a minimal viewer surface must assemble");
    let app = plugin.manifest.apps.iter().find(|app| app.role == AppRole::Viewer).expect("the registered viewer app definition");
    assert_eq!(app.io.artifact_schema, SchemaStampViewerFixture::DOCUMENT_SCHEMA);
}

#[test]
fn routed_inference_is_frozen_into_the_plugin_roster_without_a_sync_service() {
    let metadata = ArtifactInferenceServiceMetadata {
        owner: "builder-test-routed-inference",
        artifact_kind: "s.builder-test.route",
        artifact_schema: "s.builder-test.route",
        artifact_schema_version: 1,
        inference_schema: "s.builder-test.route.solve",
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        payload: None,
    };
    let plugin = PluginBuilder::<_, NoPluginApp>::new(metadata.owner)
        .label("Builder Test Routed Inference")
        .version("0.1.0")
        .package_id("semio:builder-test-routed-inference")
        .routed_inference(metadata)
        .try_build()
        .expect("metadata-only routed inference must assemble");
    let bytes = plugin.wire_list_artifact_inference_services();
    let roster: Vec<WireArtifactInferenceMetadata> = semio_framework_pack_json::from_json_str(std::str::from_utf8(&bytes).expect("roster UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("frozen roster decodes");
    assert_eq!(roster, vec![metadata.into()]);
    assert!(artifact_inference_service(metadata.artifact_kind, metadata.inference_schema).expect("global service lookup").is_none(), "route must not manufacture a synchronous service facade");
}

impl store::os_store::serialization::ArtifactSqliteSnapshotHost for NoConfig {
    
    
    
}
impl pack::snapshot::ArtifactSqliteSnapshot for NoConfig {
const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
fn to_sqlite_database(&self, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteSnapshotPhase, SqliteValue};
        control.check_rows(1)?;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("empty_artifact_state")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1)] });
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
        Ok(database)
    }
fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
        let rows = &database.table("empty_artifact_state")?.rows;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 || rows[0].values.len() != 1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "empty artifact requires one identity row"));
        }
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(Self {})
    }
}
