//! 🧱️ One runtime shard of `stdio-pdf`: its declared apps close in their own crate, so this crate alone instantiates their app-generic runtime.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The owner-authored runtime apps of this shard.
    pub enum StdioPdf17EditorApps: PluginApp {
        Pdf17AEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor>>),
        Pdf17Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor>>),
        Pdf17EEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor>>),
        Pdf17HEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor>>),
        Pdf17UaEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor>>),
        Pdf17VtEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor>>),
        Pdf17XEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor>>),
    }
}

/// 🧩️ Builds the `semio-hub-stdio-pdf-1-7-editors` component of `stdio-pdf`: the plugin's identity with this shard's apps only.
pub fn plugin() -> Result<Plugin<StdioPdf17EditorApps>, PluginAssemblyError> {
    let mut builder = semio_plugin::PluginBuilder::<semio_plugin::NeedsLabel, StdioPdf17EditorApps>::new("stdio-pdf").label("Stdio PDF").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-pdf").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_pdf::declaration(semio_s_artifact_stdio_pdf::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.component("semio-hub-stdio-pdf-1-7-editors");
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor>(semio_s_artifact_stdio_pdf::editor::pdf17a::create_pdf17_a_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor>(semio_s_artifact_stdio_pdf::editor::pdf17::create_pdf17_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor>(semio_s_artifact_stdio_pdf::editor::pdf17e::create_pdf17_e_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor>(semio_s_artifact_stdio_pdf::editor::pdf17h::create_pdf17_h_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor>(semio_s_artifact_stdio_pdf::editor::pdf17ua::create_pdf17_ua_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor>(semio_s_artifact_stdio_pdf::editor::pdf17vt::create_pdf17_vt_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor>(semio_s_artifact_stdio_pdf::editor::pdf17x::create_pdf17_x_editor());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_pdf::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}
