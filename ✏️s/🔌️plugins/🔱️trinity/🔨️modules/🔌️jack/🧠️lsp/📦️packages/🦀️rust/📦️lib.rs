//! @emoji 🗑️ Jack LSP is folded into `dsl_lsp` + Jack's `LanguageSpec`. This crate remains as a
//! compatibility shim for existing launch targets until callers migrate.

pub use dsl_lsp::{handle_json_rpc, LanguageSession};
