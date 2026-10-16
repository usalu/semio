//! ⚙️ Shell lifecycle, part two: settings panels and dock persistence.
//!
//! Second half of the shell lifecycle `impl ShellState`: live window ids, world3d close, engine surfaces, the app
//! catalogue, document retention, settings panel builders, anchors and dock/named-layout persistence.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellLifecycle
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs"]
mod display_conflicts_marketplace_tests;

//#endregion ShellLifecycle
