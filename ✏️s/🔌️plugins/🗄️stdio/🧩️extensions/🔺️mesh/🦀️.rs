//! 🔺️ `stdio-mesh` — the glTF/OBJ/STL/PLY mesh and LAS point-cloud apps as their own wasm component, over the artifact kinds and codecs the
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
    /// 🗃️ Closed runtime app fleet of the gltf/obj/stl/ply/las editors and viewers — one editor and one viewer per subset.
    pub enum StdioMeshApps: PluginApp {
        GltfAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_gltf::editor::gltf::GltfAnyEditor>>),
        GltfAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_gltf::viewer::gltf::GltfAnyViewer>>),
        ObjAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_obj::editor::obj::ObjAnyEditor>>),
        ObjAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_obj::viewer::obj::ObjAnyViewer>>),
        StlAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_stl::editor::stl::StlAnyEditor>>),
        StlAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_stl::viewer::stl::StlAnyViewer>>),
        PlyAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ply::editor::ply::PlyAnyEditor>>),
        PlyAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ply::viewer::ply::PlyAnyViewer>>),
        LasAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_las::editor::las::LasAnyEditor>>),
        LasAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_las::viewer::las::LasAnyViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the `stdio-mesh` bundle: every gltf/obj/stl/ply/las subset's editor and viewer (with the owner-mutation roster where
/// the subset's mutation enum derives one), one activation per artifact kind it opens read live from that kind's own
/// `artifact_kind().id`, the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs, and the hosted runtime of
/// each of them (`host_artifact`: schemas, inferences, document codecs, composers, formats, subset validators) in this component.
pub fn plugin() -> Result<Plugin<StdioMeshApps>, PluginAssemblyError> {
    Plugin::<StdioMeshApps>::builder("stdio-mesh")
        .label("Stdio Mesh")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:stdio-mesh")
        .depends_on("stdio", semio_framework::tree_pin!())
        .host_artifact(semio_s_artifact_stdio_gltf::declaration(semio_s_artifact_stdio_gltf::definition()?).map_err(PluginAssemblyError::definition)?)
        .host_artifact(semio_s_artifact_stdio_obj::declaration(semio_s_artifact_stdio_obj::definition()?).map_err(PluginAssemblyError::definition)?)
        .host_artifact(semio_s_artifact_stdio_stl::declaration(semio_s_artifact_stdio_stl::definition()?).map_err(PluginAssemblyError::definition)?)
        .host_artifact(semio_s_artifact_stdio_ply::declaration(semio_s_artifact_stdio_ply::definition()?).map_err(PluginAssemblyError::definition)?)
        .host_artifact(semio_s_artifact_stdio_las::declaration(semio_s_artifact_stdio_las::definition()?).map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_stdio_gltf::editor::gltf::GltfAnyEditor>(semio_s_artifact_stdio_gltf::editor::gltf::create_gltf_any_editor())
        .viewer::<semio_s_artifact_stdio_gltf::viewer::gltf::GltfAnyViewer>(semio_s_artifact_stdio_gltf::viewer::gltf::create_gltf_any_viewer())
        .editor::<semio_s_artifact_stdio_obj::editor::obj::ObjAnyEditor>(semio_s_artifact_stdio_obj::editor::obj::create_obj_any_editor())
        .viewer::<semio_s_artifact_stdio_obj::viewer::obj::ObjAnyViewer>(semio_s_artifact_stdio_obj::viewer::obj::create_obj_any_viewer())
        .editor::<semio_s_artifact_stdio_stl::editor::stl::StlAnyEditor>(semio_s_artifact_stdio_stl::editor::stl::create_stl_any_editor())
        .viewer::<semio_s_artifact_stdio_stl::viewer::stl::StlAnyViewer>(semio_s_artifact_stdio_stl::viewer::stl::create_stl_any_viewer())
        .editor::<semio_s_artifact_stdio_ply::editor::ply::PlyAnyEditor>(semio_s_artifact_stdio_ply::editor::ply::create_ply_any_editor())
        .viewer::<semio_s_artifact_stdio_ply::viewer::ply::PlyAnyViewer>(semio_s_artifact_stdio_ply::viewer::ply::create_ply_any_viewer())
        .editor::<semio_s_artifact_stdio_las::editor::las::LasAnyEditor>(semio_s_artifact_stdio_las::editor::las::create_las_any_editor())
        .viewer::<semio_s_artifact_stdio_las::viewer::las::LasAnyViewer>(semio_s_artifact_stdio_las::viewer::las::create_las_any_viewer())
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_gltf::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_obj::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_stl::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_ply::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_las::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist gltf/obj/stl/ply/las editor edits back to the open stdio document".into(),
            optional: false,
        })
        .try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioMeshApps);
