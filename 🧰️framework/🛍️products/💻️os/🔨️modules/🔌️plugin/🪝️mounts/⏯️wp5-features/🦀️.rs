// ⏯️ HUB WP5: mount hub of WP5 features: tool-run, time-travel, time-travel-panel (their root mounts live in the job facet module, so this hub is empty until a feature crate needs a root mount).
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

