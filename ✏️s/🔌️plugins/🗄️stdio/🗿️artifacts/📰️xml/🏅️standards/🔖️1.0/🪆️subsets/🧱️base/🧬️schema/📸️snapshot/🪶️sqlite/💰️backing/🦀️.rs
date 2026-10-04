//! 💰️ Authored XML field projection and reconstruction share explicit admitted domain backing.
#[path = "📤️projection/🦀️.rs"]
mod projection;
#[path = "📥️reconstruction/🦀️.rs"]
mod reconstruction;
#[path = "🔍️rows/🦀️.rs"]
mod rows;
pub(super) use projection::{append, measure, project};
pub(super) use reconstruction::reconstruct;
