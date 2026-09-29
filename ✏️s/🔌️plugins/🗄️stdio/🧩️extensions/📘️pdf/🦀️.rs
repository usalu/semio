//! 📘️ `stdio-pdf` — the PDF 1.4 and 1.7 document apps with their ISO profiles as their own wasm component, over the artifact kinds and codecs the
//! `stdio` package owns.
//!
//! Every registered app monomorphises the whole app runtime and is live code inside its component, so one stdio
//! component cannot assemble all 176 stdio apps; each family ships its bounded fleet as its own package and depends on
//! `stdio` for the kinds it opens (`🗄️stdio/🧪️tests/🚢️shipped-fleet`: every stdio app is shipped by exactly one
//! package, every stdio kind is opened by exactly one package).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet of the pdf editors and viewers — one editor and one viewer per subset.
    pub enum StdioPdfApps: PluginApp {
        Pdf14AEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor>>),
        Pdf14AViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14a::Pdf14AViewer>>),
        Pdf14Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor>>),
        Pdf14Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14::Pdf14Viewer>>),
        Pdf14XEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor>>),
        Pdf14XViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14x::Pdf14XViewer>>),
        Pdf17AEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor>>),
        Pdf17AViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17a::Pdf17AViewer>>),
        Pdf17Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor>>),
        Pdf17Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17::Pdf17Viewer>>),
        Pdf17EEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor>>),
        Pdf17EViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17e::Pdf17EViewer>>),
        Pdf17HEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor>>),
        Pdf17HViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17h::Pdf17HViewer>>),
        Pdf17UaEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor>>),
        Pdf17UaViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17ua::Pdf17UaViewer>>),
        Pdf17VtEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor>>),
        Pdf17VtViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17vt::Pdf17VtViewer>>),
        Pdf17XEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor>>),
        Pdf17XViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17x::Pdf17XViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the `stdio-pdf` bundle: every pdf subset's editor and viewer (with the owner-mutation roster where
/// the subset's mutation enum derives one), one activation per artifact kind it opens read live from that kind's own
/// `artifact_kind().id`, the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs, and the hosted runtime of
/// each of them (`host_artifact`: schemas, inferences, document codecs, composers, formats, subset validators) in this component.
pub fn plugin() -> Result<Plugin<StdioPdfApps>, PluginAssemblyError> {
    Plugin::<StdioPdfApps>::builder("stdio-pdf")
        .label("Stdio PDF")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:stdio-pdf")
        .depends_on("stdio", semio_framework::tree_pin!())
        .host_artifact(semio_s_artifact_stdio_pdf::declaration(semio_s_artifact_stdio_pdf::definition()?).map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor>(semio_s_artifact_stdio_pdf::editor::pdf14a::create_pdf14_a_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14a::Pdf14AViewer>(semio_s_artifact_stdio_pdf::viewer::pdf14a::create_pdf14_a_viewer())
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor>(semio_s_artifact_stdio_pdf::editor::pdf14::create_pdf14_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14::Pdf14Viewer>(semio_s_artifact_stdio_pdf::viewer::pdf14::create_pdf14_viewer())
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor>(semio_s_artifact_stdio_pdf::editor::pdf14x::create_pdf14_x_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14x::Pdf14XViewer>(semio_s_artifact_stdio_pdf::viewer::pdf14x::create_pdf14_x_viewer())
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor>(semio_s_artifact_stdio_pdf::editor::pdf17a::create_pdf17_a_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17a::Pdf17AViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17a::create_pdf17_a_viewer())
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor>(semio_s_artifact_stdio_pdf::editor::pdf17::create_pdf17_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17::Pdf17Viewer>(semio_s_artifact_stdio_pdf::viewer::pdf17::create_pdf17_viewer())
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor>(semio_s_artifact_stdio_pdf::editor::pdf17e::create_pdf17_e_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17e::Pdf17EViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17e::create_pdf17_e_viewer())
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor>(semio_s_artifact_stdio_pdf::editor::pdf17h::create_pdf17_h_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17h::Pdf17HViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17h::create_pdf17_h_viewer())
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor>(semio_s_artifact_stdio_pdf::editor::pdf17ua::create_pdf17_ua_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17ua::Pdf17UaViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17ua::create_pdf17_ua_viewer())
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor>(semio_s_artifact_stdio_pdf::editor::pdf17vt::create_pdf17_vt_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17vt::Pdf17VtViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17vt::create_pdf17_vt_viewer())
        .editor::<semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor>(semio_s_artifact_stdio_pdf::editor::pdf17x::create_pdf17_x_editor())
        .viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17x::Pdf17XViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17x::create_pdf17_x_viewer())
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_pdf::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist pdf editor edits back to the open stdio document".into(),
            optional: false,
        })
        .try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioPdfApps);
