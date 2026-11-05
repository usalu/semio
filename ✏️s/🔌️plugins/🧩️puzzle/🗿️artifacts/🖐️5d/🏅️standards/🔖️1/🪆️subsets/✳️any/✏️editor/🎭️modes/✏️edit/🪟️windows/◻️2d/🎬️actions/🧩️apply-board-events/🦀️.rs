//! 🧩️ Board-event dispatch action.

use semio_plugin::ActionRef;

/// 🧩️ Returns the window-local board-event action reference.
pub fn reference() -> ActionRef {
    "applyBoardEvents".into()
}
