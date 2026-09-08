
//! 🧪️ Shared harness for every `editor::raster` node's tests — mirrors TEMPLATE.md §7.
use super::*;
use semio_framework_plugin::{App, InvocationResult, VcsArtifactApp, ViewModel, testkit as framework_testkit};

pub type RasterApp = VcsArtifactApp<EditorApp<RasterPlayApp>>;

use semio_framework_plugin::PluginApp;

/// 🚧️ SDK GAP (`📓️w2-cad-report.md` "SDK gaps found" #3): `testkit::new_app_with_registry`'s
/// signature is still `fn(manifest: fn() -> App)`, unchanged for this ticket; `create_raster_app`
/// now returns `AppDefinition`. This tiny local wrapper adapts one to the other.
fn raster_app_manifest_for_testkit() -> App {
    App { definition: create_raster_app(), examples: Vec::new() }
}

pub async fn app() -> RasterApp {
    framework_testkit::new_app::<EditorApp<RasterPlayApp>>().await
}

pub async fn app_with_registry() -> RasterApp {
    framework_testkit::new_app_with_registry::<EditorApp<RasterPlayApp>>(raster_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut RasterApp, command: RasterCommand) -> InvocationResult {
    app.dispatch_typed(command, &framework_testkit::meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut RasterApp, body_key: &str) -> String {
    let tree = app.render(body_key, None, &ViewModel::default()).await.expect("render");
    framework_testkit::project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
}

pub async fn main_window_measures(app: &mut RasterApp) -> Vec<WindowMeasure> {
    app.window_measures().await.remove(composite::RASTER_PLAY_WINDOW_COMPOSITE).unwrap_or_default()
}

pub async fn semio_app() -> RasterApp {
    let mut app = framework_testkit::new_app::<EditorApp<RasterPlayApp>>().await;
    let document = crate::schema::semio_example_document();
    let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", document, None);
    let files = store::print_document_pack(&envelope).await.expect("print document pack");
    app.load_document_pack(&files).await.expect("load semio");
    app
}
