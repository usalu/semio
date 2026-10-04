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

#[path = "../../🔨️modules/🪢️swing/🦀️.rs"]
pub mod swing;

#[path = "../../🔨️modules/🧗️climbing/🦀️.rs"]
pub mod climbing;

#[path = "../../🔨️modules/🧠️behavior/🦀️.rs"]
pub mod behavior;

#[path = "../../🔨️modules/💗️feeling/🦀️.rs"]
pub mod feeling;

#[path = "../../🔨️modules/👆️gesture/🦀️.rs"]
pub mod gesture;

#[path = "../../🔨️modules/🚧️clearance/🦀️.rs"]
pub mod clearance;

#[path = "../../🔨️modules/✨️effects/🦀️.rs"]
pub mod effects;

#[path = "../../🔨️modules/🪄️mischief/🦀️.rs"]
pub mod mischief;

#[path = "../../🔨️modules/📝️draft/🦀️.rs"]
mod draft;

#[path = "../../🔨️modules/📏️spacing/🦀️.rs"]
mod spacing;

#[path = "../../🔨️modules/🗓️schedule/🦀️.rs"]
mod schedule;

#[path = "../../🔨️modules/👀️attention/🦀️.rs"]
mod attention;

#[path = "../../🔨️modules/🚶️locomotion/🦀️.rs"]
mod locomotion;

#[path = "../../🔨️modules/💞️sociability/🦀️.rs"]
mod sociability;

#[path = "../../🔨️modules/🎯️choice/🦀️.rs"]
mod choice;

#[path = "../../🔨️modules/👥️population/🦀️.rs"]
mod population;

#[path = "../../🔨️modules/🕰️clock/🦀️.rs"]
mod clock;

#[path = "../../🔨️modules/🎥️projection/🦀️.rs"]
mod projection;

#[path = "../../🔨️modules/🎪️stage/🦀️.rs"]
pub mod stage;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

/// 🧪️ The JSON codec the Protocol v2 subject hosts decode the shared vectors with — enabled by the `sut` feature only.
#[cfg(feature = "sut")]
pub use serde_json;
