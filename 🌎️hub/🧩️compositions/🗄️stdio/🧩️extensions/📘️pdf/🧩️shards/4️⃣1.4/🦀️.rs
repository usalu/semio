//! 🧱️ One runtime shard of `stdio-pdf`: its declared apps close in their own crate, so this crate alone instantiates their app-generic runtime.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The owner-authored runtime apps of this shard.
    pub enum StdioPdf14Apps: PluginApp {
        Pdf14AEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor>>),
        Pdf14AViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14a::Pdf14AViewer>>),
        Pdf14Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor>>),
        Pdf14Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14::Pdf14Viewer>>),
        Pdf14XEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor>>),
        Pdf14XViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14x::Pdf14XViewer>>),
    }
}

/// 🧩️ Builds the `semio-hub-stdio-pdf-1-4` component of `stdio-pdf`: the plugin's identity with this shard's apps only.
pub fn plugin() -> Result<Plugin<StdioPdf14Apps>, PluginAssemblyError> {
    let mut builder = semio_plugin::PluginBuilder::<semio_plugin::NeedsLabel, StdioPdf14Apps>::new("stdio-pdf").label("Stdio PDF").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-pdf").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_pdf::declaration(semio_s_artifact_stdio_pdf::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.component("semio-hub-stdio-pdf-1-4");
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor>(semio_s_artifact_stdio_pdf::editor::pdf14a::create_pdf14_a_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14a::Pdf14AViewer>(semio_s_artifact_stdio_pdf::viewer::pdf14a::create_pdf14_a_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor>(semio_s_artifact_stdio_pdf::editor::pdf14::create_pdf14_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14::Pdf14Viewer>(semio_s_artifact_stdio_pdf::viewer::pdf14::create_pdf14_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor>(semio_s_artifact_stdio_pdf::editor::pdf14x::create_pdf14_x_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14x::Pdf14XViewer>(semio_s_artifact_stdio_pdf::viewer::pdf14x::create_pdf14_x_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_pdf::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}
