//! 🧱️ One runtime shard of `stdio-pdf`: its declared apps close in their own crate, so this crate alone instantiates their app-generic runtime.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The owner-authored runtime apps of this shard.
    pub enum StdioPdf17ViewerApps: PluginApp {
        Pdf17AViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17a::Pdf17AViewer>>),
        Pdf17Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17::Pdf17Viewer>>),
        Pdf17EViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17e::Pdf17EViewer>>),
        Pdf17HViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17h::Pdf17HViewer>>),
        Pdf17UaViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17ua::Pdf17UaViewer>>),
        Pdf17VtViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17vt::Pdf17VtViewer>>),
        Pdf17XViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17x::Pdf17XViewer>>),
    }
}

/// 🧩️ Builds the `semio-hub-stdio-pdf-1-7-viewers` component of `stdio-pdf`: the plugin's identity with this shard's apps only.
pub fn plugin() -> Result<Plugin<StdioPdf17ViewerApps>, PluginAssemblyError> {
    let mut builder = semio_plugin::PluginBuilder::<semio_plugin::NeedsLabel, StdioPdf17ViewerApps>::new("stdio-pdf").label("Stdio PDF").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-pdf").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_pdf::declaration(semio_s_artifact_stdio_pdf::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.component("semio-hub-stdio-pdf-1-7-viewers");
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17a::Pdf17AViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17a::create_pdf17_a_viewer());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17::Pdf17Viewer>(semio_s_artifact_stdio_pdf::viewer::pdf17::create_pdf17_viewer());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17e::Pdf17EViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17e::create_pdf17_e_viewer());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17h::Pdf17HViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17h::create_pdf17_h_viewer());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17ua::Pdf17UaViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17ua::create_pdf17_ua_viewer());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17vt::Pdf17VtViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17vt::create_pdf17_vt_viewer());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17x::Pdf17XViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17x::create_pdf17_x_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_pdf::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}
