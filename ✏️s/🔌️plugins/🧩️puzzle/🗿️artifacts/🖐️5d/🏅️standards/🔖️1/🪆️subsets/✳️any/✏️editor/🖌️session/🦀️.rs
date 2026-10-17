//! 🖌️ Session of the puzzle 5d editor: the reserved clipboard, paste and import jobs with their fixed-page state machines. See [`super`].

pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::session::support;
pub use support::*;

#[path = "📋️clipboard/🦀️.rs"]
pub mod clipboard;
pub use clipboard::*;

#[path = "📌️paste/🦀️.rs"]
pub mod paste;
pub use paste::*;

#[path = "📥️snapshot-import/🦀️.rs"]
pub mod snapshot_import;
pub use snapshot_import::*;

//#endregion 🧵️ReservedJobs
