//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️component.rs` files.

#[path = "../../🔨️modules/🧬️contract/🦀️component.rs"]
pub mod contract;

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
