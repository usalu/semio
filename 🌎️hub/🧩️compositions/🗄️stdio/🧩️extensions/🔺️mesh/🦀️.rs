//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
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

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioMeshApps>, PluginAssemblyError> {
    let mut builder = Plugin::<StdioMeshApps>::builder("stdio-mesh").label("Stdio Mesh").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-mesh").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_gltf::declaration(semio_s_artifact_stdio_gltf::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_obj::declaration(semio_s_artifact_stdio_obj::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_stl::declaration(semio_s_artifact_stdio_stl::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_ply::declaration(semio_s_artifact_stdio_ply::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_las::declaration(semio_s_artifact_stdio_las::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.editor::<semio_s_artifact_stdio_gltf::editor::gltf::GltfAnyEditor>(semio_s_artifact_stdio_gltf::editor::gltf::create_gltf_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_gltf::viewer::gltf::GltfAnyViewer>(semio_s_artifact_stdio_gltf::viewer::gltf::create_gltf_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_obj::editor::obj::ObjAnyEditor>(semio_s_artifact_stdio_obj::editor::obj::create_obj_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_obj::viewer::obj::ObjAnyViewer>(semio_s_artifact_stdio_obj::viewer::obj::create_obj_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_stl::editor::stl::StlAnyEditor>(semio_s_artifact_stdio_stl::editor::stl::create_stl_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_stl::viewer::stl::StlAnyViewer>(semio_s_artifact_stdio_stl::viewer::stl::create_stl_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ply::editor::ply::PlyAnyEditor>(semio_s_artifact_stdio_ply::editor::ply::create_ply_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ply::viewer::ply::PlyAnyViewer>(semio_s_artifact_stdio_ply::viewer::ply::create_ply_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_las::editor::las::LasAnyEditor>(semio_s_artifact_stdio_las::editor::las::create_las_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_las::viewer::las::LasAnyViewer>(semio_s_artifact_stdio_las::viewer::las::create_las_any_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_gltf::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_obj::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_stl::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_ply::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_las::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioMeshApps);
