//! 🔢️ `stdio-binary` — the raw binary, deflate, zip and EnergyPlus weather apps as their own wasm component, over the artifact kinds and codecs the
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
    /// 🗃️ Closed runtime app fleet of the binary/deflate/zip/epw editors and viewers — one editor and one viewer per subset.
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
//#endregion 🗃️Apps

/// 🔌️ Builds the `stdio-binary` bundle: every binary/deflate/zip/epw subset's editor and viewer (with the owner-mutation roster where
/// the subset's mutation enum derives one), one activation per artifact kind it opens read live from that kind's own
/// `artifact_kind().id`, and the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs.
pub fn plugin() -> Result<Plugin<StdioBinaryApps>, PluginAssemblyError> {
    Plugin::<StdioBinaryApps>::builder("stdio-binary")
        .label("Stdio Binary")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:stdio-binary")
        .depends_on("stdio", semio_framework::tree_pin!())
        .editor::<semio_s_artifact_stdio_binary::editor::binary::BinaryEditor>(semio_s_artifact_stdio_binary::editor::binary::create_binary_editor())
        .viewer::<semio_s_artifact_stdio_binary::viewer::binary::BinaryViewer>(semio_s_artifact_stdio_binary::viewer::binary::create_binary_viewer())
        .editor::<semio_s_artifact_stdio_deflate::editor::deflate::DeflateEditor>(semio_s_artifact_stdio_deflate::editor::deflate::create_deflate_editor())
        .viewer::<semio_s_artifact_stdio_deflate::viewer::deflate::DeflateViewer>(semio_s_artifact_stdio_deflate::viewer::deflate::create_deflate_viewer())
        .editor::<semio_s_artifact_stdio_zip::editor::zip::base::ZipAnyEditor>(semio_s_artifact_stdio_zip::editor::zip::base::create_zip_any_editor())
        .viewer::<semio_s_artifact_stdio_zip::viewer::zip::base::ZipAnyViewer>(semio_s_artifact_stdio_zip::viewer::zip::base::create_zip_any_viewer())
        .editor::<semio_s_artifact_stdio_zip::editor::zip::iso21320::ZipIso21320Editor>(semio_s_artifact_stdio_zip::editor::zip::iso21320::create_zip_iso21320_editor())
        .viewer::<semio_s_artifact_stdio_zip::viewer::zip::iso21320::ZipIso21320Viewer>(semio_s_artifact_stdio_zip::viewer::zip::iso21320::create_zip_iso21320_viewer())
        .editor::<semio_s_artifact_stdio_epw::editor::epw::EpwEditor>(semio_s_artifact_stdio_epw::editor::epw::create_epw_editor())
        .viewer::<semio_s_artifact_stdio_epw::viewer::epw::EpwViewer>(semio_s_artifact_stdio_epw::viewer::epw::create_epw_viewer())
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_binary::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_deflate::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_zip::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_epw::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist binary/deflate/zip/epw editor edits back to the open stdio document".into(),
            optional: false,
        })
        .try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioBinaryApps);
