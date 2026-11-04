//! 📚️ IDML 8.0 story-text surface: the `#editor` and `#viewer` apps of `s.stdio.idml@8.0/*`.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "✏️editor/🦀️.rs"]
pub mod editor;
#[path = "👁️viewer/🦀️.rs"]
pub mod viewer;

pub const IDML_DIALECT_KIND: &str = "s.stdio.idml";
pub const IDML_DIALECT_STANDARD: &str = "8.0";

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
