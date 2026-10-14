//! 🏭️ Process plugin — subtractive/additive processing simulation bundled as a hot-swappable WASM
//! component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! 🚫️ `#![allow(async_fn_in_trait)]`: `MachineCatalog` (below) carries `#[dyn_enum]` async methods, which
//! trips rustc's "use of `fn` in public traits is discouraged .. auto trait bounds cannot be
//! specified" lint. Per ruling **R7**: silenced here, never by adding `+ Send` to a signature (R3 — Send
//! comes structurally from the concrete `MachineCatalogs` enum, never a bound) and never by making the
//! method sync.
#![allow(async_fn_in_trait)]

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
pub mod plugin;

//#endregion 🔖️Plugin
