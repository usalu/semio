//! 🏠️ `stdio-bim` — the IFC and BCF building-information apps as their own wasm component, over the artifact kinds and codecs the
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
    /// 🗃️ Closed runtime app fleet of the ifc/bcf editors and viewers — one editor and one viewer per subset.
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
//#endregion 🗃️Apps

/// 🔌️ Builds the `stdio-bim` bundle: every ifc/bcf subset's editor and viewer (with the owner-mutation roster where
/// the subset's mutation enum derives one), one activation per artifact kind it opens read live from that kind's own
/// `artifact_kind().id`, and the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs.
pub fn plugin() -> Result<Plugin<StdioBimApps>, PluginAssemblyError> {
    Plugin::<StdioBimApps>::builder("stdio-bim")
        .label("Stdio BIM")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:stdio-bim")
        .depends_on("stdio", semio_framework::tree_pin!())
        .editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_any::Ifc2x3AnyEditor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_any::create_ifc2x3_any_editor())
        .viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_any::Ifc2x3AnyViewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_any::create_ifc2x3_any_viewer())
        .editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::Ifc2x3CobieEditor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::create_ifc2x3_cobie_editor())
        .viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cobie::Ifc2x3CobieViewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_cobie::create_ifc2x3_cobie_viewer())
        .editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::Ifc2x3Cv20Editor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::create_ifc2x3_cv20_editor())
        .viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cv20::Ifc2x3Cv20Viewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_cv20::create_ifc2x3_cv20_viewer())
        .editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::Ifc2x3SavEditor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::create_ifc2x3_sav_editor())
        .viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_sav::Ifc2x3SavViewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_sav::create_ifc2x3_sav_viewer())
        .editor::<semio_s_artifact_stdio_ifc::editor::ifc4_any::Ifc4AnyEditor>(semio_s_artifact_stdio_ifc::editor::ifc4_any::create_ifc4_any_editor())
        .viewer::<semio_s_artifact_stdio_ifc::viewer::ifc4_any::Ifc4AnyViewer>(semio_s_artifact_stdio_ifc::viewer::ifc4_any::create_ifc4_any_viewer())
        .editor::<semio_s_artifact_stdio_bcf::editor::bcf::BcfAnyEditor>(semio_s_artifact_stdio_bcf::editor::bcf::create_bcf_any_editor())
        .viewer::<semio_s_artifact_stdio_bcf::viewer::bcf::BcfAnyViewer>(semio_s_artifact_stdio_bcf::viewer::bcf::create_bcf_any_viewer())
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_ifc::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_bcf::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist ifc/bcf editor edits back to the open stdio document".into(),
            optional: false,
        })
        .try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioBimApps);
