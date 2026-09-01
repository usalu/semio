//! 🌉️ Trinity Jack app — editor-host aliases (was: the plugin-root `document_vcs` module +
//! `JackHost`/`JackSession` aliases in the old bundle crate's `📦️glue.rs`; the wasm-bindgen document
//! VCS bridge that used to live here was deleted — nothing ever built it for
//! `wasm32-unknown-unknown` — see
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).

pub use framework_editor::*;

pub type JackHost = EditorHost;
