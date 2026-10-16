// 🌱️ HUB WP1: mount hub of WP1 foundations and windows: abi, actor-host, identity, interaction, operation, window-config, window-transient, window-kits, window-kits-rich.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 🏛️OwnershipFacet
#[cfg(test)]
#[path = "../../🌱️initialization/♻️retirement/🧪️tests/🦀️.rs"]
mod bounded_initializer_laws;
pub use semio_framework_plugin_abi::sqlite_wire;
#[cfg(test)]
#[path = "../../🧪️tests/🔬️app-window-kits/🦀️.rs"]
mod app_window_kits_tests;
#[cfg(test)]
#[path = "../../🧪️tests/🔬️app-panel-kit/🦀️.rs"]
mod app_panel_kit_tests;
//#endregion 🏛️OwnershipFacet
//#region 🪟️WindowFacet
pub(crate) use semio_framework_plugin_window_kits_rich::document_kit;
pub use semio_framework_plugin_window_transient::engagement;
pub(crate) use semio_framework_plugin_window_kits::image_kit;
pub(crate) use semio_framework_plugin_interaction::interaction_topology;
pub(crate) extern crate semio_framework_plugin_interaction as local_interaction;
pub(crate) use semio_framework_plugin_window_kits::media_kit;
pub(crate) use semio_framework_plugin_window_kits::mesh_kit;
pub(crate) use semio_framework_plugin_window_kits_rich::table_kit;
pub(crate) use semio_framework_plugin_window_kits::text_kit;
pub(crate) use semio_framework_plugin_window_kits_rich::tree_kit;
pub(crate) extern crate semio_framework_plugin_window_config as window_config;
pub(crate) extern crate semio_framework_plugin_window_kits as window_kits;
pub(crate) use semio_framework_plugin_window_transient::window_mutation;
pub(crate) extern crate semio_framework_plugin_window_transient as window_surface;
pub use semio_framework_plugin_window_kits_rich::world3d_host;
//#endregion 🪟️WindowFacet
//#region 📜️DeclarationFacet
pub(crate) extern crate semio_framework_plugin_identity as artifact_identity;
pub(crate) extern crate semio_framework_plugin_abi as artifact_io;
pub(crate) use semio_framework_plugin_decl_app::completion_fault;

pub(crate) use semio_framework_plugin_decl_app::completion_refusal_retirement;

pub(crate) use semio_framework_plugin_decl_app::completion_retirement;

// 🧬️ A2 (design-abi.md §4): `host_port` is deleted — see `important.md`'s "Replace, never wrap"
// list (`host_port` · `component::host_*` · `set_host_backbone_channel` are all named explicitly).
// Replaced by `🌐host/🦀️.rs`'s async `Host`/`log`/`now_ms`/`trace_span` — see that
// module and the report's "peer-coexistence"/deferred-work sections for the backbone channel gap.
pub extern crate semio_framework_plugin_actor_host as host;
pub(crate) extern crate semio_framework_plugin_operation as tool_cancellation;
//#endregion 📜️DeclarationFacet
//#region 📬️PublicationFacet
pub use semio_framework_plugin_actor_host::document_backbone_binding;
//#endregion 📬️PublicationFacet
//#region 👷️JobsFacet
pub(crate) use semio_framework_plugin_operation::completion_effects;
pub use completion_effects::CompletionEffect;
pub(crate) use semio_framework_plugin_operation::completion_vacant;
pub use completion_vacant::{VacantVec,VacantDeque};
pub use semio_framework_plugin_abi::operation_authority;
//#endregion 👷️JobsFacet
//#region 🧪️Wp1TestHosts
#[cfg(test)]
#[path = "🧪️test-hosts/📬️window-mutation/🦀️.rs"]
mod window_mutation_tests;
#[cfg(test)]
#[path = "🧪️test-hosts/🎚️window-config/🦀️.rs"]
mod window_config_tests;
#[cfg(test)]
#[path = "🧪️test-hosts/🫧️window-transient/🦀️.rs"]
mod window_transient_tests;
//#endregion 🧪️Wp1TestHosts
