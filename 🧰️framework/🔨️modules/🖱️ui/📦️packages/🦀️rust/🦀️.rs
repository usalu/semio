//! 🖱️ Target-neutral UI facts shared by every UI target: whether a session may offer its authored
//! introduction. The terminal target lives in the `semio-framework-ui-tui-*` crates and the wgpu
//! target in the `semio-framework-ui-wgpu-*` crates.

#[path = "../../🎓️introduction/🦀️.rs"]
pub mod introduction;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️targets-wgpu-case-wiring-law/🦀️.rs"]
mod case_wiring_law_tests;
