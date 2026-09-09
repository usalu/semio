use super::*;
use crate::app::{
    ArtifactEditor, ArtifactView, ArtifactViewer, ConfigView, DraftView, Editor, Emit, InteractionView, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer,
};
use crate::ViewModel;
use semio_framework::{AppRole, Dialect, Fault, IconName, StandardId, SubsetId};
use store::EngineHandles;
use ui_wgpu::wgpu::LocalizedLabel;

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

    fn initial_snapshot() -> NoConfig {
        NoConfig::default()
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
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data("schema-stamp-editor"))
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

    fn initial_snapshot() -> NoConfig {
        NoConfig::default()
    }

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
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data("schema-stamp-viewer"))
    }
}

use crate::plugin_app_close_prelude::*;
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
async fn editor_stamps_document_schema_from_the_type_when_left_empty() {
    let def = minimal_surface_def(EDITOR_STAMP_DIALECT, AppRole::Editor);
    assert!(def.io.document_schema.is_empty(), "fixture precondition: builder leaves io.document_schema empty");
    let plugin = Plugin::<SchemaStampApps>::builder("builder-test-schema-stamp-editor").label("Builder Test Schema Stamp Editor").version("0.1.0").editor::<SchemaStampEditorFixture>(def).try_build().expect("a minimal editor surface must assemble");
    let app = plugin.manifest.apps.iter().find(|app| app.role == AppRole::Editor).expect("the registered editor app definition");
    assert_eq!(app.io.document_schema, SchemaStampEditorFixture::DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn editor_does_not_overwrite_an_explicitly_set_document_schema() {
    let mut def = minimal_surface_def(EDITOR_STAMP_DIALECT, AppRole::Editor);
    def.io.document_schema = "already-set.document".into();
    let plugin = Plugin::<SchemaStampApps>::builder("builder-test-schema-stamp-editor-explicit")
        .label("Builder Test Schema Stamp Editor Explicit")
        .version("0.1.0")
        .editor::<SchemaStampEditorFixture>(def)
        .try_build()
        .expect("a minimal editor surface must assemble");
    let app = plugin.manifest.apps.iter().find(|app| app.role == AppRole::Editor).expect("the registered editor app definition");
    assert_eq!(app.io.document_schema, "already-set.document", "an explicitly set schema must survive untouched");
}

#[semio_framework_async_macros::async_test]
async fn viewer_stamps_document_schema_from_the_type_when_left_empty() {
    let def = minimal_surface_def(VIEWER_STAMP_DIALECT, AppRole::Viewer);
    assert!(def.io.document_schema.is_empty(), "fixture precondition: builder leaves io.document_schema empty");
    let plugin = Plugin::<SchemaStampApps>::builder("builder-test-schema-stamp-viewer").label("Builder Test Schema Stamp Viewer").version("0.1.0").viewer::<SchemaStampViewerFixture>(def).try_build().expect("a minimal viewer surface must assemble");
    let app = plugin.manifest.apps.iter().find(|app| app.role == AppRole::Viewer).expect("the registered viewer app definition");
    assert_eq!(app.io.document_schema, SchemaStampViewerFixture::DOCUMENT_SCHEMA);
}

#[test]
fn routed_inference_is_frozen_into_the_plugin_roster_without_a_sync_service() {
    let metadata = ArtifactInferenceServiceMetadata {
        owner: "builder-test-routed-inference",
        artifact_kind: "s.builder-test.route",
        artifact_schema: "s.builder-test.route",
        artifact_schema_version: 1,
        document_schema: "s.builder-test.route",
        document_schema_version: 1,
        inference_schema: "s.builder-test.route.solve",
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
    };
    let plugin = Plugin::<NoPluginApp>::builder(metadata.owner)
        .label("Builder Test Routed Inference")
        .version("0.1.0")
        .package_id("semio:builder-test-routed-inference")
        .routed_inference(metadata)
        .try_build()
        .expect("metadata-only routed inference must assemble");
    let bytes = plugin.wire_list_artifact_inference_services();
    let roster: Vec<WireArtifactInferenceMetadata> = protocol::json::from_json_str(std::str::from_utf8(&bytes).expect("roster UTF-8")).expect("frozen roster decodes");
    assert_eq!(roster, vec![metadata.into()]);
    assert!(artifact_inference_service(metadata.artifact_kind, metadata.inference_schema).expect("global service lookup").is_none(), "route must not manufacture a synchronous service facade");
}
