//! @emoji 🧪️ The conformance-corpus harness — loads every fixture under
//! `📚️examples/🧪️conformance/` and proves the contract crate's own `validate_snapshot`/
//! `apply_patch` treat it exactly as its sibling `📓️terra-conformance-corpus-report.md` documents.
//! React DOM and the GPU renderer family both consume the identical JSON files later; this file is
//! what keeps the Rust side of that promise honest.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! Entirely `#[cfg(test)]`: the corpus is authored JSON read from disk via `std::fs`, which does not
//! exist on `wasm32-unknown-unknown` and is meaningless to compile into a shipped renderer either way
//! — `cargo check --target wasm32-*` never builds `#[cfg(test)]` code, so this file costs the wasm
//! gates nothing. Not behind `typegen`: this must run under a plain `cargo test`.

//#region 🔖️Conformance
#[cfg(test)]
#[path = "../../🧪️tests/🔬️conformance-unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Conformance
