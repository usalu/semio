//! 🎵️ `stdio-media` — the mp4/mp3/wav/avi audio and video apps as their own wasm component, over the artifact kinds and codecs the
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
    /// 🗃️ Closed runtime app fleet of the mp4/mp3/wav/avi editors and viewers — one editor and one viewer per subset.
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
//#endregion 🗃️Apps

/// 🔌️ Builds the `stdio-media` bundle: every mp4/mp3/wav/avi subset's editor and viewer (with the owner-mutation roster where
/// the subset's mutation enum derives one), one activation per artifact kind it opens read live from that kind's own
/// `artifact_kind().id`, and the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs.
pub fn plugin() -> Result<Plugin<StdioMediaApps>, PluginAssemblyError> {
    Plugin::<StdioMediaApps>::builder("stdio-media")
        .label("Stdio Media")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:stdio-media")
        .depends_on("stdio", semio_framework::tree_pin!())
        .editor::<semio_s_artifact_stdio_mp4::editor::mp4::Mp4Editor>(semio_s_artifact_stdio_mp4::editor::mp4::create_mp4_editor())
        .viewer::<semio_s_artifact_stdio_mp4::viewer::mp4::Mp4Viewer>(semio_s_artifact_stdio_mp4::viewer::mp4::create_mp4_viewer())
        .editor::<semio_s_artifact_stdio_mp3::editor::mp3::Mp3Editor>(semio_s_artifact_stdio_mp3::editor::mp3::create_mp3_editor())
        .viewer::<semio_s_artifact_stdio_mp3::viewer::mp3::Mp3Viewer>(semio_s_artifact_stdio_mp3::viewer::mp3::create_mp3_viewer())
        .editor::<semio_s_artifact_stdio_wav::editor::wav::WavEditor>(semio_s_artifact_stdio_wav::editor::wav::create_wav_editor())
        .viewer::<semio_s_artifact_stdio_wav::viewer::wav::WavViewer>(semio_s_artifact_stdio_wav::viewer::wav::create_wav_viewer())
        .editor::<semio_s_artifact_stdio_avi::editor::avi::AviEditor>(semio_s_artifact_stdio_avi::editor::avi::create_avi_editor())
        .viewer::<semio_s_artifact_stdio_avi::viewer::avi::AviViewer>(semio_s_artifact_stdio_avi::viewer::avi::create_avi_viewer())
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_mp4::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_mp3::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_wav::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_avi::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist mp4/mp3/wav/avi editor edits back to the open stdio document".into(),
            optional: false,
        })
        .try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioMediaApps);
