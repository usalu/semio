//! 🖼️ General Canvas defining package.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;
#[path = "../../🦀️.rs"]
mod component;
pub use component::*;
