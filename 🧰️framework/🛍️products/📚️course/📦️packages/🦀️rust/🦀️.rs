//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.

#[path = "../../🔨️modules/🎲️randomness/🦀️.rs"]
pub mod randomness;

#[path = "../../🔨️modules/⛰️challenge/🦀️.rs"]
pub mod challenge;

#[path = "../../🔨️modules/🃏️sheet/🦀️.rs"]
pub mod sheet;

#[path = "../../🔨️modules/✅️validation/🦀️.rs"]
pub mod validation;

#[path = "../../🔨️modules/📏️scoring/🦀️.rs"]
pub mod scoring;

#[path = "../../🔨️modules/🏅️badges/🦀️.rs"]
pub mod badges;

#[path = "../../🔨️modules/🧾️lifecycle/🦀️.rs"]
pub mod lifecycle;

#[path = "../../🔨️modules/👁️views/🦀️.rs"]
pub mod views;

#[path = "../../🔨️modules/👥️presence/🦀️.rs"]
pub mod presence;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

/// 🧪️ The JSON codec the Protocol v2 subject hosts decode the shared vectors with.
pub use serde_json;
