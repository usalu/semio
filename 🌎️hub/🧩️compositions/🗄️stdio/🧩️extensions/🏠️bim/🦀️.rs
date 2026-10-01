//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
    pub enum StdioBimApps: PluginApp {
        Ifc2x3AnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc2x3_any::Ifc2x3AnyEditor>>),
        Ifc2x3AnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc2x3_any::Ifc2x3AnyViewer>>),
        Ifc2x3CobieEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::Ifc2x3CobieEditor>>),
        Ifc2x3CobieViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cobie::Ifc2x3CobieViewer>>),
        Ifc2x3Cv20Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::Ifc2x3Cv20Editor>>),
        Ifc2x3Cv20Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cv20::Ifc2x3Cv20Viewer>>),
        Ifc2x3SavEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::Ifc2x3SavEditor>>),
        Ifc2x3SavViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc2x3_sav::Ifc2x3SavViewer>>),
        Ifc4AnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc4_any::Ifc4AnyEditor>>),
        Ifc4AnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc4_any::Ifc4AnyViewer>>),
        BcfAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_bcf::editor::bcf::BcfAnyEditor>>),
        BcfAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_bcf::viewer::bcf::BcfAnyViewer>>),
    }
}

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioBimApps>, PluginAssemblyError> {
    let mut builder = Plugin::<StdioBimApps>::builder("stdio-bim").label("Stdio BIM").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-bim").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_ifc::declaration(semio_s_artifact_stdio_ifc::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_bcf::declaration(semio_s_artifact_stdio_bcf::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_any::Ifc2x3AnyEditor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_any::create_ifc2x3_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_any::Ifc2x3AnyViewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_any::create_ifc2x3_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::Ifc2x3CobieEditor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::create_ifc2x3_cobie_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cobie::Ifc2x3CobieViewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_cobie::create_ifc2x3_cobie_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::Ifc2x3Cv20Editor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::create_ifc2x3_cv20_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cv20::Ifc2x3Cv20Viewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_cv20::create_ifc2x3_cv20_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::Ifc2x3SavEditor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::create_ifc2x3_sav_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_sav::Ifc2x3SavViewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_sav::create_ifc2x3_sav_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc4_any::Ifc4AnyEditor>(semio_s_artifact_stdio_ifc::editor::ifc4_any::create_ifc4_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc4_any::Ifc4AnyViewer>(semio_s_artifact_stdio_ifc::viewer::ifc4_any::create_ifc4_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_bcf::editor::bcf::BcfAnyEditor>(semio_s_artifact_stdio_bcf::editor::bcf::create_bcf_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_bcf::viewer::bcf::BcfAnyViewer>(semio_s_artifact_stdio_bcf::viewer::bcf::create_bcf_any_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_ifc::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_bcf::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioBimApps);
