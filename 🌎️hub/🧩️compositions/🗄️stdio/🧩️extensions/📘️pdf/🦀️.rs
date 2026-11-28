//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

#![recursion_limit = "512"]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
    pub enum StdioPdfApps: PluginApp {
        StdioPdf14Apps(semio_hub_stdio_pdf_1_4::StdioPdf14Apps),
        StdioPdf17EditorApps(semio_hub_stdio_pdf_1_7_editors::StdioPdf17EditorApps),
        StdioPdf17ViewerApps(semio_hub_stdio_pdf_1_7_viewers::StdioPdf17ViewerApps),
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor>>) -> Self {
        Self::StdioPdf14Apps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14a::Pdf14AViewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14a::Pdf14AViewer>>) -> Self {
        Self::StdioPdf14Apps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor>>) -> Self {
        Self::StdioPdf14Apps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14::Pdf14Viewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14::Pdf14Viewer>>) -> Self {
        Self::StdioPdf14Apps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor>>) -> Self {
        Self::StdioPdf14Apps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14x::Pdf14XViewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14x::Pdf14XViewer>>) -> Self {
        Self::StdioPdf14Apps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor>>) -> Self {
        Self::StdioPdf17EditorApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor>>) -> Self {
        Self::StdioPdf17EditorApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor>>) -> Self {
        Self::StdioPdf17EditorApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor>>) -> Self {
        Self::StdioPdf17EditorApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor>>) -> Self {
        Self::StdioPdf17EditorApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor>>) -> Self {
        Self::StdioPdf17EditorApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor>>) -> Self {
        Self::StdioPdf17EditorApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17a::Pdf17AViewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17a::Pdf17AViewer>>) -> Self {
        Self::StdioPdf17ViewerApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17::Pdf17Viewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17::Pdf17Viewer>>) -> Self {
        Self::StdioPdf17ViewerApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17e::Pdf17EViewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17e::Pdf17EViewer>>) -> Self {
        Self::StdioPdf17ViewerApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17h::Pdf17HViewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17h::Pdf17HViewer>>) -> Self {
        Self::StdioPdf17ViewerApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17ua::Pdf17UaViewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17ua::Pdf17UaViewer>>) -> Self {
        Self::StdioPdf17ViewerApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17vt::Pdf17VtViewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17vt::Pdf17VtViewer>>) -> Self {
        Self::StdioPdf17ViewerApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17x::Pdf17XViewer>>> for StdioPdfApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17x::Pdf17XViewer>>) -> Self {
        Self::StdioPdf17ViewerApps(app.into())
    }
}

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioPdfApps>, PluginAssemblyError> {
    let mut builder = semio_plugin::PluginBuilder::<semio_plugin::NeedsLabel, StdioPdfApps>::new("stdio-pdf").label(semio_plugin::LocalizedLabel::native("Stdio PDF", "Stdio PDF")).version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-pdf").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_pdf::declaration(semio_s_artifact_stdio_pdf::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.component("semio-hub-stdio-pdf-1-4");
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor>(semio_s_artifact_stdio_pdf::editor::pdf14a::create_pdf14_a_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14a::Pdf14AViewer>(semio_s_artifact_stdio_pdf::viewer::pdf14a::create_pdf14_a_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor>(semio_s_artifact_stdio_pdf::editor::pdf14::create_pdf14_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14::Pdf14Viewer>(semio_s_artifact_stdio_pdf::viewer::pdf14::create_pdf14_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor>(semio_s_artifact_stdio_pdf::editor::pdf14x::create_pdf14_x_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14x::Pdf14XViewer>(semio_s_artifact_stdio_pdf::viewer::pdf14x::create_pdf14_x_viewer());
    builder = builder.component("semio-hub-stdio-pdf-1-7-editors");
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor>(semio_s_artifact_stdio_pdf::editor::pdf17a::create_pdf17_a_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor>(semio_s_artifact_stdio_pdf::editor::pdf17::create_pdf17_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor>(semio_s_artifact_stdio_pdf::editor::pdf17e::create_pdf17_e_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor>(semio_s_artifact_stdio_pdf::editor::pdf17h::create_pdf17_h_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor>(semio_s_artifact_stdio_pdf::editor::pdf17ua::create_pdf17_ua_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor>(semio_s_artifact_stdio_pdf::editor::pdf17vt::create_pdf17_vt_editor());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor>(semio_s_artifact_stdio_pdf::editor::pdf17x::create_pdf17_x_editor());
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
