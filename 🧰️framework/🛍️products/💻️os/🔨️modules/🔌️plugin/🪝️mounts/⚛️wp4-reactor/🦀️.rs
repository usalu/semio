// ⚛️ HUB WP4: mount hub of WP4 reactor: reactor-app, reactor-tasks, reactor-lifecycle, runtime-instances, runtime-ops, runtime-exchange, reactor-patches, reactor-surfaces, reactor-core, reactor-jobs, reactor-turn, reactor-wit-bridge, reactor-context.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 📄️DeclarationFixtureMutationMount

#[cfg(test)]
#[path = "../../🧪️tests/🏃️plugin-runtime-runtime-laws/🦀️.rs"]
mod plugin_runtime_runtime_laws;
//#endregion 📄️DeclarationFixtureMutationMount
//#region ⚛️ReactorFacet
pub(crate) extern crate semio_framework_plugin_reactor_app as plugin_app;
pub mod reactor {
    pub use semio_framework_plugin_reactor_core::*;
    pub use semio_framework_plugin_reactor_context as original_actor_context;
    pub use semio_framework_plugin_reactor_jobs as jobs;
    pub use semio_framework_plugin_reactor_turn as turn;
    pub use semio_framework_plugin_reactor_turn::turn_close_step::*;
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub use semio_framework_plugin_reactor_turn::drain_task_resumes;
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub use semio_framework_plugin_reactor_wit_bridge::wit_bridge::{poll, stage_cold_pair_page, stage_command_page};
    pub use semio_framework_plugin_reactor_turn::{last_turn_more_work_sources, poll_kernel, retained_command_ingress_occupancy, retained_extension_invocation_occupancy, TurnMoreWorkSources};
}
//#endregion ⚛️ReactorFacet







