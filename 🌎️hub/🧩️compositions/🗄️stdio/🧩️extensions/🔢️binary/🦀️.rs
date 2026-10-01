//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
    pub enum StdioBinaryApps: PluginApp {
        BinaryEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_binary::editor::binary::BinaryEditor>>),
        BinaryViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_binary::viewer::binary::BinaryViewer>>),
        DeflateEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_deflate::editor::deflate::DeflateEditor>>),
        DeflateViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_deflate::viewer::deflate::DeflateViewer>>),
        ZipAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_zip::editor::zip::base::ZipAnyEditor>>),
        ZipAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_zip::viewer::zip::base::ZipAnyViewer>>),
        ZipIso21320Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_zip::editor::zip::iso21320::ZipIso21320Editor>>),
        ZipIso21320Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_zip::viewer::zip::iso21320::ZipIso21320Viewer>>),
        EpwEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_epw::editor::epw::EpwEditor>>),
        EpwViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_epw::viewer::epw::EpwViewer>>),
    }
}

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioBinaryApps>, PluginAssemblyError> {
    let mut builder = Plugin::<StdioBinaryApps>::builder("stdio-binary").label("Stdio Binary").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-binary").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_binary::declaration(semio_s_artifact_stdio_binary::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_deflate::declaration(semio_s_artifact_stdio_deflate::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_zip::declaration(semio_s_artifact_stdio_zip::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_epw::declaration(semio_s_artifact_stdio_epw::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.editor::<semio_s_artifact_stdio_binary::editor::binary::BinaryEditor>(semio_s_artifact_stdio_binary::editor::binary::create_binary_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_binary::viewer::binary::BinaryViewer>(semio_s_artifact_stdio_binary::viewer::binary::create_binary_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_deflate::editor::deflate::DeflateEditor>(semio_s_artifact_stdio_deflate::editor::deflate::create_deflate_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_deflate::viewer::deflate::DeflateViewer>(semio_s_artifact_stdio_deflate::viewer::deflate::create_deflate_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_zip::editor::zip::base::ZipAnyEditor>(semio_s_artifact_stdio_zip::editor::zip::base::create_zip_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_zip::viewer::zip::base::ZipAnyViewer>(semio_s_artifact_stdio_zip::viewer::zip::base::create_zip_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_zip::editor::zip::iso21320::ZipIso21320Editor>(semio_s_artifact_stdio_zip::editor::zip::iso21320::create_zip_iso21320_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_zip::viewer::zip::iso21320::ZipIso21320Viewer>(semio_s_artifact_stdio_zip::viewer::zip::iso21320::create_zip_iso21320_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_epw::editor::epw::EpwEditor>(semio_s_artifact_stdio_epw::editor::epw::create_epw_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_epw::viewer::epw::EpwViewer>(semio_s_artifact_stdio_epw::viewer::epw::create_epw_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_binary::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_deflate::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_zip::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_epw::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioBinaryApps);
