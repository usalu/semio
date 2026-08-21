//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️component.rs`. No wasm-bindgen wrapper:
//! [`component::InteractiveJob`] is a plain, `Send`-bound trait with no js-sys/web-sys surface, so
//! there is nothing wasm-specific to wire at the glue layer — the owner file compiles unmodified on
//! `wasm32-unknown-unknown`/`wasm32-wasip2` as well as native, same shape as `⏱️trace`/`⏳️async`'s
//! own glue files.

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
