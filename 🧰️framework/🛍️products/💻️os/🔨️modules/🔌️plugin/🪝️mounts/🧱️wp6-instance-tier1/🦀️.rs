// 🧱️ HUB WP6: mount hub of WP6 instance tier 1: instance-stores, instance-history, instance-children, instance-jobs, instance-docs, instance-extension.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 🏛️OwnershipFacet
pub use semio_framework_plugin_instance_stores::action_ingress as action_ingress;
//#endregion 🏛️OwnershipFacet
