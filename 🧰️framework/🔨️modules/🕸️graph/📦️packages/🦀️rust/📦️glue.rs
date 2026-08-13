//! 🕸️ The semio graph framework module: storage and view vocabulary, index-based algorithms, drawing layouts, and the compile-time manifest registry.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

extern crate semio_framework_os_kernel as dsl;

#[path = "../../⚙️engine/🦀️component.rs"]
mod engine;
pub use engine::*;

#[path = "../../🧮️algorithms/🦀️component.rs"]
pub mod algorithms;

#[path = "../../🖊️drawing/🦀️component.rs"]
pub mod drawing;

#[path = "../../🛂️manifest/🦀️component.rs"]
pub mod manifest;
