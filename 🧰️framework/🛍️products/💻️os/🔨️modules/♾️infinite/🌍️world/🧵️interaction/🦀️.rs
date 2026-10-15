//! 🧵️ World interaction transaction: registries, marquee, picking, gumball and the interaction authority.

use super::*;

//#region 🧵️WorldInteractionTransaction
#[path = "📇️registry/🦀️.rs"]
mod registry;
pub use registry::*;
#[path = "🔍️marquee/🦀️.rs"]
mod marquee;
pub(super) use marquee::*;
#[path = "🧲️picking/🦀️.rs"]
mod picking;
pub use picking::*;
#[path = "🕹️gumball/🦀️.rs"]
mod gumball;
pub use gumball::*;
#[path = "⚙️authority/🦀️.rs"]
mod authority;
pub use authority::*;
//#endregion 🧵️WorldInteractionTransaction
