
use super::*;
use semio_framework_plugin::testkit::new_app_with_registry as framework_new_app_with_registry;
use semio_framework_plugin::{EditorApp, PluginApp, VcsArtifactApp, ViewModel};

pub type DagApp = VcsArtifactApp<EditorApp<DagPlayApp>>;

/// 🧪️ An app instance using its declared tool catalog and concrete factories.
pub async fn new_app() -> DagApp {
    new_app_with_registry().await
}

/// ✏️ Adapts `create_dag_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `new_app_with_registry`'s framework testkit signature (contract §2.5 gap 3,
/// not yet updated for the `AppDefinition`-returning convention) still expects.
pub fn dag_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_dag_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn new_app_with_registry() -> DagApp {
    framework_new_app_with_registry::<EditorApp<DagPlayApp>>(dag_app_manifest_for_testkit).await
}

pub async fn render(app: &mut DagApp, body_key: &str) -> String {
    serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render").root).expect("render json")
}
