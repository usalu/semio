//! 💰 full thirty-table Rewriting owner assembly.
#[path="📤️projection/🦀️.rs"]
mod projection;
#[path="🗂️indexes/🦀️.rs"]
mod indexes;
#[path="🌲️properties/🦀️.rs"]
mod properties;
#[path="📥️reconstruction/🦀️.rs"]
mod reconstruction;
pub(super) use projection::project;
pub(super) use reconstruction::reconstruct;
