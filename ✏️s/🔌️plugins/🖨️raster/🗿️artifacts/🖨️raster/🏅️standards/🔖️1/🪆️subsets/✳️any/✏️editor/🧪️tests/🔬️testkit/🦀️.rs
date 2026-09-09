//! 🧪️ Shared harness for every `editor::raster` node's tests — mirrors TEMPLATE.md §7.
use super::*;
use semio_framework_plugin::{testkit as framework_testkit, InvocationResult, VcsArtifactApp, ViewModel};

pub type RasterApp = VcsArtifactApp<EditorApp<RasterPlayApp>>;

use semio_framework_plugin::PluginApp;

pub async fn app() -> RasterApp {
    framework_testkit::new_app::<EditorApp<RasterPlayApp>>().await
}

pub async fn dispatch(app: &mut RasterApp, command: RasterCommand) -> InvocationResult {
    app.dispatch_typed(command, &framework_testkit::meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut RasterApp, body_key: &str) -> String {
    render_with_view(app, body_key, &ViewModel::default()).await
}

pub async fn render_with_view(app: &mut RasterApp, body_key: &str, view_state: &ViewModel) -> String {
    let tree = app.render(body_key, None, view_state).await.expect("render");
    framework_testkit::project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
}

pub async fn main_window_measures(app: &mut RasterApp) -> Vec<WindowMeasure> {
    app.window_measures(&ViewModel::default()).await.remove(composite::RASTER_PLAY_WINDOW_COMPOSITE).unwrap_or_default()
}

pub async fn semio_app() -> RasterApp {
    let mut app = framework_testkit::new_app::<EditorApp<RasterPlayApp>>().await;
    let document = crate::standards::v1::subsets::any::schema::semio_example_document();
    let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", document, None);
    let files = store::print_document_pack(&envelope).await.expect("print document pack");
    app.load_document_pack(&files).await.expect("load semio");
    app
}
