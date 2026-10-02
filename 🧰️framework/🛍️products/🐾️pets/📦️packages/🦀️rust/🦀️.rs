//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.

#[path = "../../🧬️schema/🦀️.rs"]
pub mod schema;

#[path = "../../🔨️modules/✅️validation/🦀️.rs"]
pub mod validation;

#[path = "../../🔨️modules/📐️trigonometry/🦀️.rs"]
pub mod trigonometry;

#[path = "../../🔨️modules/🎲️randomness/🦀️.rs"]
pub mod randomness;

#[path = "../../🔨️modules/🦴️rig/🦀️.rs"]
pub mod rig;

#[path = "../../🔨️modules/🎞️animation/🦀️.rs"]
pub mod animation;

#[path = "../../🔨️modules/🏞️terrain/🦀️.rs"]
pub mod terrain;

#[path = "../../🔨️modules/🧠️behavior/🦀️.rs"]
pub mod behavior;

#[path = "../../🔨️modules/🎪️stage/🦀️.rs"]
pub mod stage;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

/// 🧪️ The JSON codec the Protocol v2 subject hosts decode the shared vectors with — enabled by the `sut` feature only.
#[cfg(feature = "sut")]
pub use serde_json;
