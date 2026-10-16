// ⚛️ HUB WP4: mount hub of WP4 reactor: reactor-app, reactor-tasks, reactor-lifecycle, runtime-instances, runtime-ops, runtime-exchange, reactor-patches, reactor-surfaces, reactor-core, reactor-jobs, reactor-turn, reactor-wit-bridge, reactor-context.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 📄️DeclarationFixtureMutationMount
#[cfg(test)]
#[path = "../../🧪️tests/🤝️cooperative-pump/🦀️.rs"]
mod cooperative_pump_tests;
#[cfg(test)]
#[path = "../../🧪️tests/🏃️plugin-runtime-runtime-laws/🦀️.rs"]
mod plugin_runtime_runtime_laws;
//#endregion 📄️DeclarationFixtureMutationMount
//#region ⚛️ReactorFacet
#[path = "../../⚛️reactor/🔌️app/🦀️.rs"]
pub(crate) mod plugin_app;
#[path = "../../⚛️reactor/🦀️.rs"]
pub mod reactor;
//#endregion ⚛️ReactorFacet
