//! 🛠️ `stdio-cad` — the STEP AP214, DXF and DWG CAD exchange apps as their own wasm component, over the artifact kinds and codecs the
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
    /// 🗃️ Closed runtime app fleet of the step/dxf/dwg editors and viewers — one editor and one viewer per subset.
    pub enum StdioCadApps: PluginApp {
        StepAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_any::StepAnyEditor>>),
        StepAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_any::StepAnyViewer>>),
        StepCc1Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc1::StepCc1Editor>>),
        StepCc1Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc1::StepCc1Viewer>>),
        StepCc2Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc2::StepCc2Editor>>),
        StepCc2Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc2::StepCc2Viewer>>),
        StepCc3Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc3::StepCc3Editor>>),
        StepCc3Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc3::StepCc3Viewer>>),
        StepCc4Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc4::StepCc4Editor>>),
        StepCc4Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc4::StepCc4Viewer>>),
        StepCc5Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc5::StepCc5Editor>>),
        StepCc5Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc5::StepCc5Viewer>>),
        StepCc6Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc6::StepCc6Editor>>),
        StepCc6Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc6::StepCc6Viewer>>),
        DxfAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_dxf::editor::dxf::DxfAnyEditor>>),
        DxfAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_dxf::viewer::dxf::DxfAnyViewer>>),
        DwgAc1018Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_dwg::editor::dwg_ac1018::DwgAc1018Editor>>),
        DwgAc1018Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_dwg::viewer::dwg_ac1018::DwgAc1018Viewer>>),
        DwgAc1024Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_dwg::editor::dwg_ac1024::DwgAc1024Editor>>),
        DwgAc1024Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_dwg::viewer::dwg_ac1024::DwgAc1024Viewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the `stdio-cad` bundle: every step/dxf/dwg subset's editor and viewer (with the owner-mutation roster where
/// the subset's mutation enum derives one), one activation per artifact kind it opens read live from that kind's own
/// `artifact_kind().id`, and the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs.
pub fn plugin() -> Result<Plugin<StdioCadApps>, PluginAssemblyError> {
    Plugin::<StdioCadApps>::builder("stdio-cad")
        .label("Stdio CAD")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:stdio-cad")
        .depends_on("stdio", semio_framework::tree_pin!())
        .editor::<semio_s_artifact_stdio_step::editor::step_any::StepAnyEditor>(semio_s_artifact_stdio_step::editor::step_any::create_step_any_editor())
        .viewer::<semio_s_artifact_stdio_step::viewer::step_any::StepAnyViewer>(semio_s_artifact_stdio_step::viewer::step_any::create_step_any_viewer())
        .editor::<semio_s_artifact_stdio_step::editor::step_cc1::StepCc1Editor>(semio_s_artifact_stdio_step::editor::step_cc1::create_step_cc1_editor())
        .viewer::<semio_s_artifact_stdio_step::viewer::step_cc1::StepCc1Viewer>(semio_s_artifact_stdio_step::viewer::step_cc1::create_step_cc1_viewer())
        .editor::<semio_s_artifact_stdio_step::editor::step_cc2::StepCc2Editor>(semio_s_artifact_stdio_step::editor::step_cc2::create_step_cc2_editor())
        .viewer::<semio_s_artifact_stdio_step::viewer::step_cc2::StepCc2Viewer>(semio_s_artifact_stdio_step::viewer::step_cc2::create_step_cc2_viewer())
        .editor::<semio_s_artifact_stdio_step::editor::step_cc3::StepCc3Editor>(semio_s_artifact_stdio_step::editor::step_cc3::create_step_cc3_editor())
        .viewer::<semio_s_artifact_stdio_step::viewer::step_cc3::StepCc3Viewer>(semio_s_artifact_stdio_step::viewer::step_cc3::create_step_cc3_viewer())
        .editor::<semio_s_artifact_stdio_step::editor::step_cc4::StepCc4Editor>(semio_s_artifact_stdio_step::editor::step_cc4::create_step_cc4_editor())
        .viewer::<semio_s_artifact_stdio_step::viewer::step_cc4::StepCc4Viewer>(semio_s_artifact_stdio_step::viewer::step_cc4::create_step_cc4_viewer())
        .editor::<semio_s_artifact_stdio_step::editor::step_cc5::StepCc5Editor>(semio_s_artifact_stdio_step::editor::step_cc5::create_step_cc5_editor())
        .viewer::<semio_s_artifact_stdio_step::viewer::step_cc5::StepCc5Viewer>(semio_s_artifact_stdio_step::viewer::step_cc5::create_step_cc5_viewer())
        .editor::<semio_s_artifact_stdio_step::editor::step_cc6::StepCc6Editor>(semio_s_artifact_stdio_step::editor::step_cc6::create_step_cc6_editor())
        .viewer::<semio_s_artifact_stdio_step::viewer::step_cc6::StepCc6Viewer>(semio_s_artifact_stdio_step::viewer::step_cc6::create_step_cc6_viewer())
        .editor::<semio_s_artifact_stdio_dxf::editor::dxf::DxfAnyEditor>(semio_s_artifact_stdio_dxf::editor::dxf::create_dxf_any_editor())
        .viewer::<semio_s_artifact_stdio_dxf::viewer::dxf::DxfAnyViewer>(semio_s_artifact_stdio_dxf::viewer::dxf::create_dxf_any_viewer())
        .editor::<semio_s_artifact_stdio_dwg::editor::dwg_ac1018::DwgAc1018Editor>(semio_s_artifact_stdio_dwg::editor::dwg_ac1018::create_dwg_ac1018_editor())
        .viewer::<semio_s_artifact_stdio_dwg::viewer::dwg_ac1018::DwgAc1018Viewer>(semio_s_artifact_stdio_dwg::viewer::dwg_ac1018::create_dwg_ac1018_viewer())
        .editor::<semio_s_artifact_stdio_dwg::editor::dwg_ac1024::DwgAc1024Editor>(semio_s_artifact_stdio_dwg::editor::dwg_ac1024::create_dwg_ac1024_editor())
        .viewer::<semio_s_artifact_stdio_dwg::viewer::dwg_ac1024::DwgAc1024Viewer>(semio_s_artifact_stdio_dwg::viewer::dwg_ac1024::create_dwg_ac1024_viewer())
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_step::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_dxf::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_dwg::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist step/dxf/dwg editor edits back to the open stdio document".into(),
            optional: false,
        })
        .try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioCadApps);
