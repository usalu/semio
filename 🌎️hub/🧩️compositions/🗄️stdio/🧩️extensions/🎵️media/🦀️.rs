//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
    pub enum StdioMediaApps: PluginApp {
        Mp4Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_mp4::editor::mp4::Mp4Editor>>),
        Mp4Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_mp4::viewer::mp4::Mp4Viewer>>),
        Mp3Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_mp3::editor::mp3::Mp3Editor>>),
        Mp3Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_mp3::viewer::mp3::Mp3Viewer>>),
        WavEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_wav::editor::wav::WavEditor>>),
        WavViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_wav::viewer::wav::WavViewer>>),
        AviEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_avi::editor::avi::AviEditor>>),
        AviViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_avi::viewer::avi::AviViewer>>),
    }
}

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioMediaApps>, PluginAssemblyError> {
    let mut builder = Plugin::<StdioMediaApps>::builder("stdio-media").label("Stdio Media").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-media").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_mp4::declaration(semio_s_artifact_stdio_mp4::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_avi::declaration(semio_s_artifact_stdio_avi::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_mp3::declaration(semio_s_artifact_stdio_mp3::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_wav::declaration(semio_s_artifact_stdio_wav::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.editor::<semio_s_artifact_stdio_mp4::editor::mp4::Mp4Editor>(semio_s_artifact_stdio_mp4::editor::mp4::create_mp4_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_mp4::viewer::mp4::Mp4Viewer>(semio_s_artifact_stdio_mp4::viewer::mp4::create_mp4_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_mp3::editor::mp3::Mp3Editor>(semio_s_artifact_stdio_mp3::editor::mp3::create_mp3_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_mp3::viewer::mp3::Mp3Viewer>(semio_s_artifact_stdio_mp3::viewer::mp3::create_mp3_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_wav::editor::wav::WavEditor>(semio_s_artifact_stdio_wav::editor::wav::create_wav_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_wav::viewer::wav::WavViewer>(semio_s_artifact_stdio_wav::viewer::wav::create_wav_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_avi::editor::avi::AviEditor>(semio_s_artifact_stdio_avi::editor::avi::create_avi_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_avi::viewer::avi::AviViewer>(semio_s_artifact_stdio_avi::viewer::avi::create_avi_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_mp4::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_avi::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_mp3::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_wav::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioMediaApps);
