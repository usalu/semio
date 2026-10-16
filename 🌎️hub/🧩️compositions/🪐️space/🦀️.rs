//! 🔌️ Space composition of independently packaged Home and Space Index artifacts.

#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};
use semio_framework_plugin::PluginBuilder;
use semio_framework_os::OS_SPACE_SCHEMA;
pub use semio_s_space_core::*;

#[cfg(test)]
#[path = "🧪️testing/🦀️.rs"]
mod test_documents;

//#region 🔌️Registration
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the home, space-index, and studio surfaces.
    pub enum SpaceApps: PluginApp {
        HomeEditor(VcsArtifactApp<EditorApp<semio_s_artifact_space_home::editor::home::HomeApp>>),
        HomeViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_space_home::viewer::home::HomeViewer>>),
        SpaceIndexEditor(VcsArtifactApp<EditorApp<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor>>),
        SpaceIndexViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_space_space::viewer::space_index::SpaceIndexViewer>>),
        Studio(VcsArtifactApp<semio_hub_space_app::SpaceApp>),
    }
}

/// 🔌️ Builds the S Studio plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) builds all artifact, app-schema, and codec
/// contributions as immutable data before the aggregate registration commit. `.activation(…)`/
/// `.execution(…)`/`.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
/// M6-remaining, `📓️design-abi.md` §3/§6) are this crate's migration proof: one `OnArtifactKind`
/// event per owned kind (`home`/`space`, read live from each's own `artifact_kind().id`), `Isolated`
/// execution (grepped for `.handler(…)`, a `🧩️extensions/` dir, and self-tick loops — none found,
/// despite this crate having the heaviest `Effect` usage in the repo), and one `documents.write` ask
/// covering both editors' persisted mutations. No quota declared — no measured need found.
// 🚫️async: `plugin_exports!`'s `__semio_install_plugin_bundle` calls this from a bare `fn()`
// pointer slot (`PLUGIN_BUNDLE_INSTALLER: OnceLock<fn()>`), which can never itself be `async fn` —
// so this fn stays sync and bridges its still-async `home`/`engine` sub-calls with `resolve_ready`
// (in scope via `plugin_app_close_prelude::*` above), same poll-once guarantee as every other E5
// executor-bridge call site in this crate; the `space` artifact/editor/viewer calls below are
// already sync (no bridge needed).
pub fn plugin() -> Result<Plugin<SpaceApps>, PluginAssemblyError> {
    PluginBuilder::<_, SpaceApps>::new("space")
        .label("S Studio")
        .version("0.1.0")
        .package_id("semio:space")
        .local_backbone_storage()
        .artifact(::semio_framework_async::poll::resolve_ready(semio_s_artifact_space_home::declaration()).map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_space_home::editor::home::HomeApp>(::semio_framework_async::poll::resolve_ready(semio_s_artifact_space_home::editor::home::create_home_app()))
        .editor_mutation_roster::<semio_s_artifact_space_home::editor::home::HomeApp>()
        .viewer::<semio_s_artifact_space_home::viewer::home::HomeViewer>(::semio_framework_async::poll::resolve_ready(semio_s_artifact_space_home::viewer::home::create_home_viewer()))
        .viewer_mutation_roster::<semio_s_artifact_space_home::viewer::home::HomeViewer>()
        .artifact(semio_s_artifact_space_space::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor>(semio_s_artifact_space_space::editor::space_index::create_space_index_editor())
        .editor_mutation_roster::<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor>()
        .viewer::<semio_s_artifact_space_space::viewer::space_index::SpaceIndexViewer>(semio_s_artifact_space_space::viewer::space_index::create_space_index_viewer())
        .viewer_mutation_roster::<semio_s_artifact_space_space::viewer::space_index::SpaceIndexViewer>()
        .contributes_topic(semio_s_artifact_space_space::space_index_topic_contribution())
        .document_app::<semio_hub_space_app::SpaceApp>(::semio_framework_async::poll::resolve_ready(semio_hub_space_app::create_space_app()))
        .foreign_document_codec::<semio_hub_space_app::SpaceApp>(OS_SPACE_SCHEMA)
        .activation(ActivationEvent::OnArtifactKind { kind: ::semio_framework_async::poll::resolve_ready(semio_s_artifact_space_home::artifact_kind()).id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_space_space::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist home/space-index edits to the open document".into(), optional: false })
        .canvas_runtime_assets()
        .try_build()
}
//#endregion 🔌️Registration

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

//#region 🧪️SpaceIndexProjectionTests
#[cfg(test)]
#[path = "🧪️tests/🔬️space-index-projection/🦀️.rs"]
mod space_index_projection_tests;
//#endregion 🧪️SpaceIndexProjectionTests

//#region 🧪️RetainedStoreFootprintTests
#[cfg(test)]
#[path = "🧪️tests/🔬️retained-store-footprint/🦀️.rs"]
mod retained_store_footprint_tests;
//#endregion 🧪️RetainedStoreFootprintTests

//#region 🧪️InteractiveJobCatalogTests
#[cfg(test)]
#[path = "🧪️tests/🔬️interactive-job-catalog/🦀️.rs"]
mod interactive_job_catalog_tests;
//#endregion 🧪️InteractiveJobCatalogTests

