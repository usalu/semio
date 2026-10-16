//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};
use semio_framework_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the architect editor and viewer.
    pub enum ArchitectApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<crate::editor::architect::ArchitectPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>),
        Viewer(VcsArtifactApp<ViewerApp<crate::viewer::architect::ArchitectViewer>, semio_s_artifact_stdio_semio::SemioMembers>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.editor(…)`/`.viewer(…)` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the old single `.document_app(…)`
/// registration with the two role-carrying surfaces for `s.architect.program@1/*`. `.activation(…)`/
/// `.execution(…)`/`.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
/// M6-remaining, `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s
/// shape: the host activates one instance whenever a `program::artifact_kind().id` artifact is
/// opened, this plugin's actor runs `Isolated`, and it asks the broker for document write access. `.asset(…)` declares the
/// program's JSON Schema documents as package assets the host preloads into `Event::InstanceOpen.assets`: the descriptor no
/// longer embeds them, and the guest's validators read them through the schema registry's package-asset reader.
pub fn plugin() -> Result<Plugin<ArchitectApps>, PluginAssemblyError> {
    let mut builder = PluginBuilder::<_, ArchitectApps>::new("architect")
        .label("Architect")
        .version("0.1.0")
        .package_id("semio:architect")
        .artifact(crate::artifacts::program::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<crate::editor::architect::ArchitectPlayApp>(crate::editor::architect::create_architect_app())
        .editor_mutation_roster::<crate::editor::architect::ArchitectPlayApp>()
        .viewer::<crate::viewer::architect::ArchitectViewer>(crate::viewer::architect::create_architect_viewer())
        .viewer_mutation_roster::<crate::viewer::architect::ArchitectViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::program::standards::v1::subsets::any::schema::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist architect program edits to the open document".into(), optional: false });
    for declaration in crate::artifacts::program::standards::v1::subsets::any::schema::schema_leaf_asset_declarations() {
        builder = builder.asset(declaration);
    }
    builder.try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

//#region 🪢️TaxonomyMounts
#[path = "🎟️capabilities/🦀️.rs"]
pub mod capabilities;
#[path = "🔧️setup/🦀️.rs"]
pub mod setup;
#[path = "🪪️manifest/🦀️.rs"]
pub mod manifest;
//#endregion 🪢️TaxonomyMounts
