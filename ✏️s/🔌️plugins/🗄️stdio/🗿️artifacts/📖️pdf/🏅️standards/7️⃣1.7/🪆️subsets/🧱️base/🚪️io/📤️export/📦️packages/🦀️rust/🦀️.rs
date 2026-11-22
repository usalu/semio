//! 📤️ The byte writer and the mutation bridge of `stdio.pdf` 1.7 `base`, mounted below the page editor so editor code can encode and apply mutations without reaching the test layers.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
