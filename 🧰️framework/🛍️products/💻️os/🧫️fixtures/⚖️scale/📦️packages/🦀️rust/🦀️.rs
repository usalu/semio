//! 🧫️ Scale-fixture crate entry — WIRING ONLY (see other `🦀️.rs` files in this repo for the
//! convention). `#[path = "."]` on the grouping module keeps the leaf's own `#[path]` relative to
//! the owner root (`🧫️fixtures/⚖️scale/`), not spliced under an inline `scale/` subdirectory.

#[path = "."]
pub mod scale {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
}

pub use scale::*;
