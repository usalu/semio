//! 🪟️ Puzzle3d editor window laws — window options, fill, distribution, UI scope, utilities, world selection, gumball, history references, kit in-port, convergence and close.

#![allow(clippy::result_large_err)]

include!("../🧰️prelude/🦀️.rs");

#[path = "../🧰️context/🦀️.rs"]
pub mod context;
use context::*;
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
use semio_s_artifact_puzzle_3d_editor_config::Puzzle3dCamera;

include!("🪟️window-option-laws/🦀️.rs");

include!("🪣️fill-and-distribution-laws/🦀️.rs");

include!("🎯️scope-and-utility-laws/🦀️.rs");

include!("🖱️world-selection-laws/🦀️.rs");

include!("🎛️gumball-laws/🦀️.rs");

include!("🧾️history-kit-close-laws/🦀️.rs");
